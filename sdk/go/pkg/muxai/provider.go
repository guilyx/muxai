package muxai

import "context"

type Provider interface {
	Name() ProviderName
	Run(ctx context.Context, req Request) (Response, error)
	RunAsync(ctx context.Context, req Request) (<-chan Event, <-chan error)
}
