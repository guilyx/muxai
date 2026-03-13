# Security Policy

## Supported Versions

The project currently supports the latest `main` branch and active release
tags for each SDK track.

## Reporting a Vulnerability

Please do not open public issues for security vulnerabilities.

Use one of these channels:

- GitHub private vulnerability reporting (preferred).
- Security contact email if configured in project metadata.

When reporting, include:

- Affected language SDK and version/tag.
- Reproduction steps and impact.
- Any known mitigations or workarounds.

## Response Targets

- Initial triage acknowledgment: within 3 business days.
- Severity assessment and remediation plan: within 7 business days.
- Patch timeline depends on severity and exploitability.

## Secret Handling

- Never commit API keys or credentials.
- Use `.env.example` as a template and keep real `.env` files private.
