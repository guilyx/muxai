# Changelog

## [Unreleased]

### Added

- Bootstrapped Rust crate with `Cargo.toml` and base `src/lib.rs`.
- Added path-filtered Rust CI workflow with `cargo fmt`, build, and tests.
- Added SDK documentation aligned to the cross-language muxai contract.
- Implemented Rust SDK core types (`Client`, `Provider`, `Request`, `Response`, and error taxonomy).
- Added retry-aware sync execution and async provider interface support.
- Added CLI provider constructors for Cursor, Claude, and Vibe and initial client tests.
