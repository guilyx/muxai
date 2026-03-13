# muxai-go

Go SDK for muxai, providing a unified client interface for agent CLIs.

## Features

- Unified `Client` for Cursor, Claude, and Vibe providers.
- Consistent request/response types.
- Structured error taxonomy with retry support.
- Sync (`Run`) and async (`RunAsync`) execution.
- Context-first API with timeout and cancellation support.

## Install

```bash
go get github.com/guilyx/muxai/sdk/go
```

## Example

```go
client, err := muxai.NewClient(
	muxai.WithProvider(cursor.NewProvider()),
	muxai.WithProvider(claude.NewProvider()),
	muxai.WithProvider(vibe.NewProvider()),
	muxai.WithDefaultProvider(muxai.ProviderCursor),
	muxai.WithTimeout(30*time.Second),
	muxai.WithRetries(2, 100*time.Millisecond, 2*time.Second),
)
if err != nil {
	panic(err)
}

resp, err := client.Run(context.Background(), muxai.ProviderCursor, muxai.Request{
	Messages: []muxai.Message{
		{Role: muxai.RoleUser, Content: "Review this code for bugs."},
	},
})
if err != nil {
	panic(err)
}
fmt.Println(resp.Content)
```

## Async Example

```go
events, errs := client.RunAsync(context.Background(), muxai.ProviderClaude, req)
for ev := range events {
	if ev.Type == muxai.EventTypeDone && ev.Response != nil {
		fmt.Println(ev.Response.Content)
	}
}
for err := range errs {
	if err != nil {
		panic(err)
	}
}
```

## Error Handling

Muxai errors are wrapped with a typed `ErrorCode` and provider metadata:

- `config_error`
- `auth_error`
- `rate_limit_error`
- `transient_error`
- `provider_exec_error`
- `provider_parse_error`
- `timeout_error`
- `canceled_error`

Use helpers:

```go
if muxai.IsCode(err, muxai.ErrorCodeRateLimit) {
	// handle backoff strategy
}
```
