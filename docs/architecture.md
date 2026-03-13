# Muxai Architecture

Muxai provides a consistent SDK shape across languages:

- A single `Client` entrypoint.
- A provider abstraction layer (`Cursor`, `Claude`, `Vibe` initially).
- Unified request/response models and error taxonomy.
- Sync and async APIs.

## Design Principles

- Consistency across languages.
- Predictable error handling.
- Extensibility for new provider adapters.
- Minimal dependencies and clear interfaces.

## Contract Surface

Each SDK is expected to expose:

- `Client` constructor with provider registration.
- `Run` sync execution.
- `RunAsync` event-based execution.
- Shared request/response models.
- Provider-agnostic error codes.

## Go Reference Layout

The initial reference implementation lives in `sdk/go`:

- `pkg/muxai/client.go`
- `pkg/muxai/provider.go`
- `pkg/muxai/types.go`
- `pkg/muxai/errors.go`
- `pkg/muxai/providers/*`
