# muxai-typescript

TypeScript SDK for muxai.

## Status

Implemented with a shared client/provider contract aligned with Go and Python:

- Unified `Client` API.
- Sync/async-friendly execution APIs.
- Shared request/response and event models.
- Typed provider error categories.
- Provider runtime safeguards (timeout handling, env merge, non-zero exit classification).

## Tooling

- Package management via `package.json`
- Type checking/build via `tsconfig.json`
- CI: path-filtered GitHub Actions workflow

## Planned Provider Adapters

- Cursor
- Claude
- Vibe

## Quick Start

```ts
import { Client, createClaudeProvider, createCursorProvider, createVibeProvider } from "@guilyx/muxai-typescript";

const client = new Client(
  [createCursorProvider(), createClaudeProvider(), createVibeProvider()],
  { defaultProvider: "cursor" },
);

const response = await client.run({
  messages: [{ role: "user", content: "Hello" }],
});

console.log(response.content);
```
