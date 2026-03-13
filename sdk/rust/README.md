# muxai-rust

Rust SDK for muxai.

## Status

Scaffolded in Phase 1 with CI and crate metadata. The Rust crate will mirror the cross-language SDK contract:

- `Client` abstraction for provider routing.
- Sync and async APIs.
- Unified request/response models.
- Strongly typed error taxonomy.

## Tooling

- Crate metadata in `Cargo.toml`
- CI: path-filtered GitHub Actions workflow

## Planned Provider Adapters

- Cursor
- Claude
- Vibe
