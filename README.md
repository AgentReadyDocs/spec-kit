# spec-kit

Open-source templates, rubrics, and linting assets for agent-ready documentation.

## Purpose

`spec-kit` is the canonical source for specification artifacts that reduce ambiguity before implementation.

## What Belongs Here

- Templates for use cases, NFRs, ADRs, PRDs, and related specification docs
- Rubrics and pass/fail gates for quality review
- Validation and linting rules for schema and clarity checks (where provided)
- Examples that demonstrate agent-ready specification quality (where provided)

## Core Principles

- Make unknowns explicit (`[OPEN]`, `[ASSUMPTION]`, owner, due date).
- Prefer invariants and constraints over narrative prose.
- Define interfaces, state transitions, and error behavior explicitly.
- Express acceptance criteria as executable-style tests.

## Typical Layout

When present, content is organized under:

- `templates/`
- `rubrics/`
- `lint/`
- `examples/`
- `docs/`

## Public Content Policy

All content in this repository is public and intended for open collaboration.

- Do not include confidential, customer-specific, or non-public business information.
- Do not include personal data (PII), credentials, or secrets.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE`.

## Contributing

Contributions are welcome. Read `CONTRIBUTING.md` before opening a PR.
By contributing, you agree your contributions are licensed under Apache-2.0.

## Content And Data Policy

- Do not contribute proprietary, confidential, or customer-identifying material.
- Do not include personal data (PII), credentials, or secrets.
- Examples must be synthetic or fully anonymized.
- You are responsible for validating fitness and compliance in your own environment.

## Trademarks

"AgentReadyDocs" is a project name. This project is not affiliated with Anthropic, OpenAI, or other platform vendors.
