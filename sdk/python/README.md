# muxai-python

Python SDK for muxai.

## Status

Implemented with the same core contract as Go:

- `Client` with provider routing.
- Sync and async APIs.
- Unified request/response models.
- Structured error taxonomy.

## Tooling

- Packaging: `pyproject.toml`
- Environment and dependency management: `uv`
- CI: path-filtered GitHub Actions workflow

## Quick Start

```python
from muxai import Client, ClientConfig, Message, ProviderName, Request, Role
from muxai.providers import ClaudeProvider, CursorProvider, VibeProvider

client = Client(
    providers=[CursorProvider(), ClaudeProvider(), VibeProvider()],
    config=ClientConfig(default_provider=ProviderName.CURSOR),
)

response = client.run(Request(messages=[Message(role=Role.USER, content="Hello")]))
print(response.content)
```

## Planned Provider Adapters

- Cursor
- Claude
- Vibe

Future candidates include Codex CLI, Gemini CLI, Aider, and Cody CLI.
