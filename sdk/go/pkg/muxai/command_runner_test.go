package muxai

import (
	"context"
	"testing"
)

func TestExecCommandRunnerRunSuccess(t *testing.T) {
	r := &ExecCommandRunner{}
	out, err := r.Run(context.Background(), CommandRequest{
		Command: "sh",
		Args:    []string{"-c", "read x; echo \"$x\""},
		Stdin:   "hello",
	})
	if err != nil {
		t.Fatalf("Run error: %v", err)
	}
	if out != "hello\n" {
		t.Fatalf("unexpected output: %q", out)
	}
}

func TestExecCommandRunnerMissingCommand(t *testing.T) {
	r := &ExecCommandRunner{}
	_, err := r.Run(context.Background(), CommandRequest{})
	if err == nil {
		t.Fatal("expected command required error")
	}
}
