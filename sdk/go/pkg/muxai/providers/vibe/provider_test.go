package vibe

import (
	"context"
	"errors"
	"testing"

	"github.com/guilyx/muxai/sdk/go/pkg/muxai"
)

type fakeRunner struct {
	out string
	err error
}

func (f *fakeRunner) Run(ctx context.Context, req muxai.CommandRequest) (string, error) {
	if req.Command == "" {
		return "", errors.New("missing command")
	}
	return f.out, f.err
}

func TestRunSuccess(t *testing.T) {
	p := NewProvider(
		WithCommand("vibe"),
		WithRunner(&fakeRunner{out: " done "}),
	)

	resp, err := p.Run(context.Background(), muxai.Request{
		Messages: []muxai.Message{{Role: muxai.RoleUser, Content: "hello"}},
	})
	if err != nil {
		t.Fatalf("Run error: %v", err)
	}
	if resp.Content != "done" {
		t.Fatalf("unexpected response content: %q", resp.Content)
	}
}

func TestRunClassifiesProviderError(t *testing.T) {
	p := NewProvider(
		WithCommand("vibe"),
		WithRunner(&fakeRunner{err: errors.New("bad output")}),
	)

	_, err := p.Run(context.Background(), muxai.Request{})
	if err == nil {
		t.Fatal("expected error")
	}
	if !muxai.IsCode(err, muxai.ErrorCodeProviderExec) {
		t.Fatalf("expected provider exec code, got %v", err)
	}
}

func TestRunAsync(t *testing.T) {
	p := NewProvider(
		WithCommand("vibe"),
		WithRunner(&fakeRunner{out: "ok"}),
	)
	events, errs := p.RunAsync(context.Background(), muxai.Request{})

	seenDone := false
	for ev := range events {
		if ev.Type == muxai.EventTypeDone && ev.Response != nil && ev.Response.Content == "ok" {
			seenDone = true
		}
	}
	for err := range errs {
		if err != nil {
			t.Fatalf("unexpected async err: %v", err)
		}
	}
	if !seenDone {
		t.Fatal("missing done event")
	}
}
