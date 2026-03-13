package muxai

import (
	"context"
	"errors"
	"fmt"
	"time"
)

type clientConfig struct {
	defaultProvider ProviderName
	timeout         time.Duration
	maxRetries      int
	baseDelay       time.Duration
	maxDelay        time.Duration
}

type Option func(*Client) error

type Client struct {
	providers map[ProviderName]Provider
	cfg       clientConfig
}

func NewClient(opts ...Option) (*Client, error) {
	c := &Client{
		providers: map[ProviderName]Provider{},
		cfg: clientConfig{
			timeout:    30 * time.Second,
			maxRetries: 2,
			baseDelay:  100 * time.Millisecond,
			maxDelay:   2 * time.Second,
		},
	}

	for _, opt := range opts {
		if err := opt(c); err != nil {
			return nil, err
		}
	}

	if len(c.providers) == 0 {
		return nil, WrapError(ErrorCodeConfig, "", "NewClient", errors.New("at least one provider is required"), false)
	}
	if c.cfg.defaultProvider == "" {
		for name := range c.providers {
			c.cfg.defaultProvider = name
			break
		}
	}

	if _, ok := c.providers[c.cfg.defaultProvider]; !ok {
		return nil, WrapError(ErrorCodeConfig, c.cfg.defaultProvider, "NewClient", errors.New("default provider not registered"), false)
	}

	return c, nil
}

func WithProvider(provider Provider) Option {
	return func(c *Client) error {
		if provider == nil {
			return WrapError(ErrorCodeConfig, "", "WithProvider", errors.New("provider is nil"), false)
		}
		c.providers[provider.Name()] = provider
		return nil
	}
}

func WithDefaultProvider(provider ProviderName) Option {
	return func(c *Client) error {
		c.cfg.defaultProvider = provider
		return nil
	}
}

func WithTimeout(timeout time.Duration) Option {
	return func(c *Client) error {
		if timeout <= 0 {
			return WrapError(ErrorCodeConfig, "", "WithTimeout", errors.New("timeout must be > 0"), false)
		}
		c.cfg.timeout = timeout
		return nil
	}
}

func WithRetries(maxRetries int, baseDelay, maxDelay time.Duration) Option {
	return func(c *Client) error {
		if maxRetries < 0 {
			return WrapError(ErrorCodeConfig, "", "WithRetries", errors.New("maxRetries must be >= 0"), false)
		}
		c.cfg.maxRetries = maxRetries
		c.cfg.baseDelay = baseDelay
		c.cfg.maxDelay = maxDelay
		return nil
	}
}

func (c *Client) RunDefault(ctx context.Context, req Request) (Response, error) {
	return c.Run(ctx, c.cfg.defaultProvider, req)
}

func (c *Client) Run(ctx context.Context, provider ProviderName, req Request) (Response, error) {
	p, ok := c.providers[provider]
	if !ok {
		return Response{}, WrapError(ErrorCodeConfig, provider, "Run", fmt.Errorf("provider %q not registered", provider), false)
	}

	runCtx := ctx
	cancel := func() {}
	if c.cfg.timeout > 0 {
		runCtx, cancel = context.WithTimeout(ctx, c.cfg.timeout)
	}
	defer cancel()

	var lastErr error
	for attempt := 0; attempt <= c.cfg.maxRetries; attempt++ {
		if runCtx.Err() != nil {
			return Response{}, mapContextError(provider, "Run", runCtx.Err())
		}

		start := time.Now()
		res, err := p.Run(runCtx, req)
		if err == nil {
			res.Duration = time.Since(start)
			return res, nil
		}

		lastErr = err
		if !isRetryable(err) || attempt == c.cfg.maxRetries {
			break
		}

		wait := deterministicBackoff(attempt, c.cfg.baseDelay, c.cfg.maxDelay)
		timer := time.NewTimer(wait)
		select {
		case <-runCtx.Done():
			timer.Stop()
			return Response{}, mapContextError(provider, "Run", runCtx.Err())
		case <-timer.C:
		}
	}

	return Response{}, lastErr
}

func (c *Client) RunAsync(ctx context.Context, provider ProviderName, req Request) (<-chan Event, <-chan error) {
	events := make(chan Event, 16)
	errs := make(chan error, 1)

	p, ok := c.providers[provider]
	if !ok {
		defer close(events)
		defer close(errs)
		errs <- WrapError(ErrorCodeConfig, provider, "RunAsync", fmt.Errorf("provider %q not registered", provider), false)
		return events, errs
	}

	runCtx := ctx
	cancel := func() {}
	if c.cfg.timeout > 0 {
		runCtx, cancel = context.WithTimeout(ctx, c.cfg.timeout)
	}

	innerEvents, innerErrs := p.RunAsync(runCtx, req)
	go func() {
		defer close(events)
		defer close(errs)
		defer cancel()

		for {
			select {
			case <-runCtx.Done():
				errs <- mapContextError(provider, "RunAsync", runCtx.Err())
				return
			case ev, ok := <-innerEvents:
				if !ok {
					innerEvents = nil
				} else {
					events <- ev
				}
			case err, ok := <-innerErrs:
				if !ok {
					innerErrs = nil
				} else {
					errs <- err
				}
			}

			if innerEvents == nil && innerErrs == nil {
				return
			}
		}
	}()

	return events, errs
}

func mapContextError(provider ProviderName, op string, err error) error {
	if errors.Is(err, context.Canceled) {
		return WrapError(ErrorCodeCanceled, provider, op, err, false)
	}
	if errors.Is(err, context.DeadlineExceeded) {
		return WrapError(ErrorCodeTimeout, provider, op, err, true)
	}
	return err
}
