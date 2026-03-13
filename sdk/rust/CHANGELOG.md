# Changelog

## [Unreleased]

### Added

- Bootstrapped Rust crate with `Cargo.toml` and base `src/lib.rs`.
- Added path-filtered Rust CI workflow with `cargo fmt`, build, and tests.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented Rust SDK core types (`Client`, `Provider`, `Request`, `Response`, and error taxonomy).
- Added retry-aware sync execution and async provider interface support.
- Added CLI provider constructors for Cursor, Claude, and Vibe and initial client tests.
- Refactored Rust SDK into modules (`client`, `types`, `errors`, `provider`) with explicit public exports.
- Added parity-oriented Rust model types (`FinishReason`, `Usage`, `ToolDefinition`, `ToolCall`, `Event`).
- Added `Client::run_default(...)` and `Client::run_events(...)` to align core execution semantics.
- Hardened CLI provider execution: request prompt piped via stdin, timeout enforcement, and typed non-zero exit classification.
- Added Rust provider runtime tests for prompt pass-through, auth classification, and timeout behavior.
