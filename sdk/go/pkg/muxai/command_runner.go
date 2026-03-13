package muxai

import (
	"bytes"
	"context"
	"fmt"
	"os/exec"
)

type CommandRunner interface {
	Run(ctx context.Context, req CommandRequest) (string, error)
}

type CommandRequest struct {
	Command string
	Args    []string
	Env     map[string]string
	Stdin   string
}

type ExecCommandRunner struct{}

func (r *ExecCommandRunner) Run(ctx context.Context, req CommandRequest) (string, error) {
	if req.Command == "" {
		return "", fmt.Errorf("command is required")
	}

	cmd := exec.CommandContext(ctx, req.Command, req.Args...)
	if len(req.Env) > 0 {
		env := cmd.Environ()
		for k, v := range req.Env {
			env = append(env, fmt.Sprintf("%s=%s", k, v))
		}
		cmd.Env = env
	}

	var stdout bytes.Buffer
	var stderr bytes.Buffer
	cmd.Stdout = &stdout
	cmd.Stderr = &stderr
	cmd.Stdin = bytes.NewBufferString(req.Stdin)

	if err := cmd.Run(); err != nil {
		if stderr.Len() > 0 {
			return "", fmt.Errorf("%w: %s", err, stderr.String())
		}
		return "", err
	}

	return stdout.String(), nil
}
