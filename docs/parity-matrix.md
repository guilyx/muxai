# SDK Parity Matrix

This document defines production acceptance criteria for language SDK parity in muxai.

## Canonical Semantics

- Shared core semantics are mandatory across languages.
- API shape stays language-native (`asyncio`, Rust traits/results, TypeScript promises/iterators).
- Provider behavior must remain consistent for Cursor, Claude, and Vibe adapters.

## Acceptance Criteria

### Core Models

- `Request`: supports `system_prompt`, `messages`, `tools`, `metadata`, `max_turns`.
- `Response`: supports `content`, `raw`, `finish_reason`, `tool_calls`, `usage`, `duration`.
- `Event`: supports `started`, `delta`, `done`, and terminal error signaling.

### Client Semantics

- Sync execution API.
- Async execution API.
- Event-stream API for long-running provider calls.
- Configurable default provider, timeout, and retry policy.
- Retryable-classification parity (`rate_limit`, `transient`, temporary execution failures).

### Error Taxonomy

All SDKs expose equivalent typed error categories:

- `config_error`
- `auth_error`
- `rate_limit_error`
- `transient_error`
- `provider_exec_error`
- `provider_parse_error`
- `timeout_error`
- `canceled_error`

### Provider Adapter Guarantees

- Command/environment configuration options.
- Prompt/request shaping consistency.
- Non-zero exit codes mapped to typed execution errors.
- Timeout/cancel support and resource cleanup.

### Test Requirements

- Contract tests validating shared semantics per language.
- Provider adapter tests (success, retryable failures, auth/rate-limit mapping, timeout).
- Async/event-stream lifecycle tests.
- Regression tests for parsing and error mapping.

## Current Status Snapshot

| Capability | Go | Python | Rust | TypeScript |
| --- | --- | --- | --- | --- |
| Core models parity | Partial | Partial | Partial | Partial |
| Sync client | Yes | Yes | Yes | Yes |
| Async client | Yes | Yes | Partial | Yes |
| Event streaming | Basic (`started`/`done`) | Missing | Missing | Missing |
| Error taxonomy parity | Strong | Partial | Partial | Partial |
| Provider hardening | Strong | Partial | Weak | Partial |
| Contract test suite | Partial | Missing | Missing | Missing |
| Publish automation | Go only | Missing | Missing | Missing |

## PR Program Mapping

- `[core]` establishes this contract and matrix.
- `[python]`, `[rust]`, `[typescript]` PR streams close parity gaps.
- `[ci]` adds publish-grade release workflows.
- `[docs]` finalizes OSS contributor and governance quality.
