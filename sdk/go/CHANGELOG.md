# Changelog

## [Unreleased]

### Added
- Implemented `muxai` Go client with provider registry, default provider selection, retry strategy, and timeout handling.
- Added unified Go SDK models (`Request`, `Response`, `Message`, `Event`, `Usage`, and enums).
- Added provider abstraction interface with sync (`Run`) and async (`RunAsync`) execution.
- Added structured error taxonomy and helpers for code-based inspection.
- Added command execution abstraction (`CommandRunner`) with default shell-backed runner.
- Added Cursor, Claude, and Vibe provider adapters with configurable command, args, env, and testable runners.
- Added comprehensive tests across client behavior, prompt building, retries, error helpers, command runner, and provider adapters.
- Added CI quality gates for `gofmt`, `go vet`, race-safe tests, and `>=70%` coverage threshold.
