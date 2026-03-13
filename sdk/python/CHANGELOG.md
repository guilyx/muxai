# Changelog

## [Unreleased]

### Added

- Bootstrapped Python package layout with `pyproject.toml` and `src/muxai` package.
- Added path-filtered Python CI workflow using `uv`.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented typed Python SDK core: client config, provider protocol, request/response models, and structured errors.
- Implemented CLI-backed provider adapters for Cursor, Claude, and Vibe with sync and async execution paths.
- Added Python unit tests covering sync/async client behavior and provider routing failures.
