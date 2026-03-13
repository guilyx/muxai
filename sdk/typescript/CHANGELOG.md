# Changelog

## [Unreleased]

### Added

- Bootstrapped TypeScript package with `package.json`, `tsconfig.json`, and `src/index.ts`.
- Added path-filtered TypeScript CI workflow with install, build, and test steps.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented TypeScript SDK core client, provider interface, shared models, and typed error taxonomy.
- Added CLI-backed provider adapters for Cursor, Claude, and Vibe.
- Added TypeScript tests for client routing and configuration error behavior.

### Fixed

- Removed invalid `private` modifier from `#runWithTimeout` private-identifier method to satisfy TypeScript compiler rules.
