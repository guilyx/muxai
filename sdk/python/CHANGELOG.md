# Changelog

## [Unreleased]

### Added

- Bootstrapped Python package layout with `pyproject.toml` and `src/muxai` package.
- Added path-filtered Python CI workflow using `uv`.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented typed Python SDK core: client config, provider protocol, request/response models, and structured errors.
- Implemented CLI-backed provider adapters for Cursor, Claude, and Vibe with sync and async execution paths.
- Added Python unit tests covering sync/async client behavior and provider routing failures.
- Added parity-oriented model types (`FinishReason`, `Usage`, `ToolDefinition`, `ToolCall`, `Event`) and event lifecycle semantics.
- Added `Client.run_default(...)` and `Client.run_events(...)` with timeout and typed error propagation behavior.
- Hardened provider runtime with sync/async timeout guards and error-classification mapping.
- Added provider-focused runtime tests for success, auth error classification, and async timeout behavior.
