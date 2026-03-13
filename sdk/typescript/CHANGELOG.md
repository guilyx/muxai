# Changelog

## [Unreleased]

### Added

- Bootstrapped TypeScript package with `package.json`, `tsconfig.json`, and `src/index.ts`.
- Added path-filtered TypeScript CI workflow with install, build, and test steps.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented TypeScript SDK core client, provider interface, shared models, and typed error taxonomy.
- Added CLI-backed provider adapters for Cursor, Claude, and Vibe.
- Added TypeScript tests for client routing and configuration error behavior.
- Added parity-oriented TypeScript model types (`FinishReason`, `Usage`, `ToolDefinition`, `ToolCall`, `Event`).
- Added `Client.runDefault(...)` and `Client.runEvents(...)` for default execution and event lifecycle support.
- Hardened CLI provider runtime with timeout enforcement, inherited env merge, and typed process error classification.
- Added provider-focused runtime tests for success, auth error classification, and timeout behavior.

### Fixed

- Removed invalid `private` modifier from `#runWithTimeout` private-identifier method to satisfy TypeScript compiler rules.
