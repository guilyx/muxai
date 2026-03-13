# Contributing to muxai

Thanks for contributing.

## Development Setup

1. Fork and clone the repository.
2. Install toolchains:
   - Go 1.23+
   - Python 3.10+ and `uv`
   - Rust stable
   - Node 22+
3. Install and enable pre-commit:

```bash
uv tool install pre-commit
pre-commit install
```

## Branch and PR Conventions

- Use feature branches.
- Prefix PR titles with scope tags where relevant:
  - `[core]`, `[python]`, `[rust]`, `[typescript]`, `[ci]`, `[docs]`.
- Keep PRs focused and small enough for review.
- Update changelogs for all touched SDKs and root changelog.

## Testing Expectations

Run relevant checks before opening a PR:

- `uvx pre-commit run --all-files`
- Go: `cd sdk/go && go test ./... -race`
- Python: `cd sdk/python && uv sync --all-extras && uv run python -m unittest discover -s tests -p "test_*.py"`
- Rust: `cd sdk/rust && cargo fmt --all && cargo test`
- TypeScript: `cd sdk/typescript && npm ci && npm run build && npm test`

## Commit Message Guidance

- Write concise, descriptive commit titles.
- Prefer intent-focused messages that explain why a change exists.

## Security and Sensitive Data

- Never commit real secrets.
- Use `.env.example` as the reference template.
- Read `SECURITY.md` for responsible disclosure.
