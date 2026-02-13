# spec-kit

Open-source templates, rubrics, and co-authoring/review skills for agent-ready documentation.

## Purpose

`spec-kit` is the canonical source for specification artifacts that reduce ambiguity before implementation.

## What Belongs Here

- Templates for use cases, NFRs, ADRs, PRDs, and related specification docs
- Rubrics and pass/fail gates for quality review
- Validation and linting rules for schema and clarity checks (where provided)
- Co-authoring and review skills for applying templates and rubrics
- Examples that demonstrate agent-ready specification quality (where provided)

## Core Principles

- Make unknowns explicit (`[OPEN]`, `[ASSUMPTION]`, owner, due date).
- Prefer invariants and constraints over narrative prose.
- Define interfaces, state transitions, and error behavior explicitly.
- Express acceptance criteria as executable-style tests.

## Quality Definition

- Correctness first: acceptance criteria, invariants, and contracts define what “right” means.
- Reduce variance in behavioral outcomes: specs constrain externally observable behavior (state transitions, side effects, errors) so independent implementations converge.
- Token efficiency: minimal sufficient detail via structure and non-duplication, never by omitting required constraints.
- Canonical guidance: `docs/agent-ready-quality.md`.

## Non-goals

- Identical code or identical artifacts across implementations.
- “Guaranteed one-shot” implementation success claims.
- Any evaluation infrastructure beyond generating excellent agent-ready documentation.

## Typical Layout

When present, content is organized under:

- `templates/`
- `rubrics/`
- `skills/`
- `lint/`
- `examples/`
- `docs/`

## Start Here

- Use case template: `templates/usecase.md`
- NFR baseline template: `templates/nfr.md`
- Glossary/entities template: `templates/glossary-entities.md`
- ADR template: `templates/adr.md`
- Skills: `skills/README.md`
- Example use case: `examples/uc-0001-create-widget.md`
- Example glossary/entities: `examples/glossary-entities-example.md`
- Example NFR baseline: `examples/nfr-0001-widget-service.md`

## Rubrics

- Use case rubric: `rubrics/usecase-rubric.md`
- NFR baseline rubric: `rubrics/nfr-rubric.md`
- Glossary/entities rubric: `rubrics/glossary-entities-rubric.md`
- ADR rubric: `rubrics/adr-rubric.md`
- Doc-set rubric: `rubrics/docset-rubric.md`
- Rubric guidance (required input): `rubrics/rubric-guidance.md`

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
