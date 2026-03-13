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

## Shared Semantic Contract

The contract is shared semantically and adapted ergonomically per language.

- Request semantics:
  - `system_prompt`, `messages`, `tools`, `metadata`, `max_turns`
- Response semantics:
  - `content`, `raw`, `finish_reason`, `tool_calls`, `usage`, `duration`
- Event semantics:
  - lifecycle events `started`, `delta`, `done`, `error`
- Error semantics:
  - `config_error`, `auth_error`, `rate_limit_error`, `transient_error`,
    `provider_exec_error`, `provider_parse_error`, `timeout_error`, `canceled_error`
- Runtime semantics:
  - deterministic retry policy for retryable failures
  - cancellation and timeout mapping to typed errors
  - provider adapters behind a strict interface/protocol/trait

For implementation-level parity criteria and status tracking, see `docs/parity-matrix.md`.

## Go Reference Layout

The initial reference implementation lives in `sdk/go`:

- `pkg/muxai/client.go`
- `pkg/muxai/provider.go`
- `pkg/muxai/types.go`
- `pkg/muxai/errors.go`
- `pkg/muxai/providers/*`
