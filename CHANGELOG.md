# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initialized monorepo structure for Go, Python, Rust, and TypeScript SDKs.
- Added root documentation (`README`, architecture and providers docs) and provider expansion references.
- Added path-filtered GitHub Actions workflows for Go, Python, Rust, TypeScript, and repository quality checks.
- Added tag-based Go release workflow (`go/v*`) that publishes release artifacts with checksums.
- Added project-level changelog conventions with per-language changelog files.
- Added repository best-practice dotfiles (`.editorconfig`, `.gitattributes`, `.markdownlint.json`, `.prettierignore`).
- Added `.pre-commit-config.yaml` and wired pre-commit execution into repository quality CI.
- Added root `.env.example` and documented environment-based provider/local gateway configuration.
- Added cross-language semantic contract documentation and an SDK parity matrix with production acceptance criteria.

### Changed

- Simplified repository quality CI to run pre-commit hooks as the single CI quality gate.
- Removed the `Go Release` badge from README to avoid 404 badge checks before workflow exists on default branch.
