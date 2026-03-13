# muxai-rust

Rust SDK for muxai.

## Status

Implemented with a typed client/provider contract mirroring the Go SDK foundation:

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

## Quick Start

```rust
use std::sync::Arc;

use muxai::{CliProvider, Client, ClientConfig, Message, ProviderName, Request, Role};

let provider = Arc::new(CliProvider::cursor());
let client = Client::new(
    vec![provider],
    ClientConfig {
        default_provider: ProviderName::Cursor,
        ..Default::default()
    },
)?;

let response = client.run(
    None,
    Request {
        messages: vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
        }],
        system_prompt: None,
    },
)?;
println!("{}", response.content);
```
