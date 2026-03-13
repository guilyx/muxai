package muxai

import (
	"context"
	"errors"
	"testing"
	"time"
)

type fakeProvider struct {
	name         ProviderName
	runFunc      func(ctx context.Context, req Request) (Response, error)
	runAsyncFunc func(ctx context.Context, req Request) (<-chan Event, <-chan error)
}

func (f *fakeProvider) Name() ProviderName { return f.name }

func (f *fakeProvider) Run(ctx context.Context, req Request) (Response, error) {
	return f.runFunc(ctx, req)
}

func (f *fakeProvider) RunAsync(ctx context.Context, req Request) (<-chan Event, <-chan error) {
	return f.runAsyncFunc(ctx, req)
}

func TestNewClientRequiresProvider(t *testing.T) {
	_, err := NewClient()
	if err == nil {
		t.Fatal("expected error when no providers configured")
	}
	if !IsCode(err, ErrorCodeConfig) {
		t.Fatalf("expected config error, got %v", err)
	}
}

func TestClientRunDefault(t *testing.T) {
	p := &fakeProvider{
		name: ProviderCursor,
		runFunc: func(ctx context.Context, req Request) (Response, error) {
			return Response{
				Provider: ProviderCursor,
				Content:  "ok",
			}, nil
		},
		runAsyncFunc: func(ctx context.Context, req Request) (<-chan Event, <-chan error) {
			ev := make(chan Event)
			er := make(chan error)
			close(ev)
			close(er)
			return ev, er
		},
	}

	c, err := NewClient(WithProvider(p))
	if err != nil {
		t.Fatalf("NewClient error: %v", err)
	}

	resp, err := c.RunDefault(context.Background(), Request{
		Messages: []Message{{Role: RoleUser, Content: "hello"}},
	})
	if err != nil {
		t.Fatalf("RunDefault error: %v", err)
	}
	if resp.Content != "ok" {
		t.Fatalf("unexpected response content: %q", resp.Content)
	}
}

func TestClientRunRetryTransient(t *testing.T) {
	attempts := 0
	p := &fakeProvider{
		name: ProviderClaude,
		runFunc: func(ctx context.Context, req Request) (Response, error) {
			attempts++
			if attempts == 1 {
				return Response{}, WrapError(ErrorCodeTransient, ProviderClaude, "Run", errors.New("temporary"), true)
			}
			return Response{
				Provider: ProviderClaude,
				Content:  "recovered",
			}, nil
		},
		runAsyncFunc: func(ctx context.Context, req Request) (<-chan Event, <-chan error) {
			ev := make(chan Event)
			er := make(chan error)
			close(ev)
			close(er)
			return ev, er
		},
	}

	c, err := NewClient(
		WithProvider(p),
		WithRetries(2, time.Millisecond, 5*time.Millisecond),
		WithDefaultProvider(ProviderClaude),
	)
	if err != nil {
		t.Fatalf("NewClient error: %v", err)
	}

	resp, err := c.RunDefault(context.Background(), Request{})
	if err != nil {
		t.Fatalf("RunDefault error: %v", err)
	}
	if resp.Content != "recovered" {
		t.Fatalf("unexpected response content: %q", resp.Content)
	}
	if attempts != 2 {
		t.Fatalf("expected 2 attempts, got %d", attempts)
	}
}

func TestClientRunAsync(t *testing.T) {
	p := &fakeProvider{
		name: ProviderVibe,
		runFunc: func(ctx context.Context, req Request) (Response, error) {
			return Response{}, nil
		},
		runAsyncFunc: func(ctx context.Context, req Request) (<-chan Event, <-chan error) {
			ev := make(chan Event, 2)
			er := make(chan error, 1)
			ev <- Event{Type: EventTypeStarted, Provider: ProviderVibe}
			ev <- Event{Type: EventTypeDone, Provider: ProviderVibe, Response: &Response{Provider: ProviderVibe, Content: "done"}}
			close(ev)
			close(er)
			return ev, er
		},
	}

	c, err := NewClient(
		WithProvider(p),
		WithDefaultProvider(ProviderVibe),
	)
	if err != nil {
		t.Fatalf("NewClient error: %v", err)
	}

	events, errs := c.RunAsync(context.Background(), ProviderVibe, Request{})
	gotDone := false
	for ev := range events {
		if ev.Type == EventTypeDone && ev.Response != nil && ev.Response.Content == "done" {
			gotDone = true
		}
	}

	for err := range errs {
		if err != nil {
			t.Fatalf("unexpected async error: %v", err)
		}
	}

	if !gotDone {
		t.Fatal("did not receive done event")
	}
}

func TestMapContextError(t *testing.T) {
	cancelErr := mapContextError(ProviderCursor, "Run", context.Canceled)
	if !IsCode(cancelErr, ErrorCodeCanceled) {
		t.Fatalf("expected canceled code, got %v", cancelErr)
	}

	timeoutErr := mapContextError(ProviderCursor, "Run", context.DeadlineExceeded)
	if !IsCode(timeoutErr, ErrorCodeTimeout) {
		t.Fatalf("expected timeout code, got %v", timeoutErr)
	}
}
