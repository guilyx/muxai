# muxai-python

Python SDK for muxai.

## Status

Scaffolded in Phase 1 with CI and packaging metadata. The Python API will follow the same contract as Go:

- `Client` with provider routing.
- Sync and async APIs.
- Unified request/response models.
- Structured error taxonomy.

## Tooling

- Packaging: `pyproject.toml`
- Environment and dependency management: `uv`
- CI: path-filtered GitHub Actions workflow

## Planned Provider Adapters

- Cursor
- Claude
- Vibe

Future candidates include Codex CLI, Gemini CLI, Aider, and Cody CLI.
