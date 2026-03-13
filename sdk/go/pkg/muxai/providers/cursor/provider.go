package cursor

import (
	"context"
	"strings"

	"github.com/guilyx/muxai/sdk/go/pkg/muxai"
)

type Option func(*Provider)

type Provider struct {
	command string
	args    []string
	env     map[string]string
	runner  muxai.CommandRunner
}

func NewProvider(opts ...Option) *Provider {
	p := &Provider{
		command: "cursor-agent",
		runner:  &muxai.ExecCommandRunner{},
		env:     map[string]string{},
	}
	for _, opt := range opts {
		opt(p)
	}
	return p
}

func WithCommand(command string) Option {
	return func(p *Provider) {
		p.command = command
	}
}

func WithArgs(args ...string) Option {
	return func(p *Provider) {
		p.args = append([]string{}, args...)
	}
}

func WithEnv(env map[string]string) Option {
	return func(p *Provider) {
		cloned := make(map[string]string, len(env))
		for k, v := range env {
			cloned[k] = v
		}
		p.env = cloned
	}
}

func WithRunner(runner muxai.CommandRunner) Option {
	return func(p *Provider) {
		p.runner = runner
	}
}

func (p *Provider) Name() muxai.ProviderName {
	return muxai.ProviderCursor
}

func (p *Provider) Run(ctx context.Context, req muxai.Request) (muxai.Response, error) {
	raw, err := p.runner.Run(ctx, muxai.CommandRequest{
		Command: p.command,
		Args:    p.args,
		Env:     p.env,
		Stdin:   muxai.BuildPrompt(req),
	})
	if err != nil {
		return muxai.Response{}, classifyError(err)
	}

	content := strings.TrimSpace(raw)
	return muxai.Response{
		Provider:     muxai.ProviderCursor,
		Content:      content,
		Raw:          raw,
		FinishReason: muxai.FinishReasonStop,
	}, nil
}

func (p *Provider) RunAsync(ctx context.Context, req muxai.Request) (<-chan muxai.Event, <-chan error) {
	events := make(chan muxai.Event, 2)
	errs := make(chan error, 1)

	go func() {
		defer close(events)
		defer close(errs)

		events <- muxai.Event{Type: muxai.EventTypeStarted, Provider: p.Name()}
		res, err := p.Run(ctx, req)
		if err != nil {
			errs <- err
			return
		}

		events <- muxai.Event{
			Type:     muxai.EventTypeDone,
			Provider: p.Name(),
			Response: &res,
		}
	}()

	return events, errs
}

func classifyError(err error) error {
	msg := strings.ToLower(err.Error())
	switch {
	case strings.Contains(msg, "unauthorized"), strings.Contains(msg, "auth"):
		return muxai.WrapError(muxai.ErrorCodeAuth, muxai.ProviderCursor, "Run", err, false)
	case strings.Contains(msg, "rate limit"), strings.Contains(msg, "too many requests"):
		return muxai.WrapError(muxai.ErrorCodeRateLimit, muxai.ProviderCursor, "Run", err, true)
	case strings.Contains(msg, "timeout"):
		return muxai.WrapError(muxai.ErrorCodeTimeout, muxai.ProviderCursor, "Run", err, true)
	default:
		return muxai.WrapError(muxai.ErrorCodeProviderExec, muxai.ProviderCursor, "Run", err, true)
	}
}
