# spec-kit

Templates, rubrics, and a single-binary linter (`ard`) for writing **agent-ready specs** that reduce ambiguity before implementation.

![ci](https://github.com/AgentReadyDocs/spec-kit/actions/workflows/ci.yml/badge.svg)
![ard](https://github.com/AgentReadyDocs/spec-kit/actions/workflows/ard.yml/badge.svg)

## Why this exists

Teams ship faster when specs are:

- **Executable-style**: acceptance criteria reads like tests and matches real interfaces and errors.
- **Low-variance**: independent implementations converge on the same externally observable behavior.
- **Reviewable**: structured artifacts + rubrics allow consistent pass/fail gates.

Canonical quality guidance: `docs/agent-ready-quality.md`.

## Standard: ARSF

`spec-kit` publishes the **Agent-Ready Spec Format (ARSF)**: a versioned, interoperable spec format (Markdown + YAML frontmatter) with conformance profiles and a reference validator (`ard`).

- Standard docs: `docs/arsf/README.md`

## Try it in 60 seconds

Install the `ard` CLI (verified installer via GitHub Releases):

```bash
curl -fsSL -o install-ard.sh https://raw.githubusercontent.com/AgentReadyDocs/spec-kit/main/scripts/install_ard.sh
sh install-ard.sh --version v0.1.0
```

## Supported platforms

`install_ard.sh` supports:

- macOS: `x86_64`, `arm64` (Apple Silicon)
- Linux: `x86_64` (glibc)

Windows: download the `ard-*-pc-windows-msvc.zip` asset from GitHub Releases and verify it with the matching `.sha256` file.

Run lint checks:

```bash
ard lint ./AGENTS.md
ard lint .                # docset lint (docs/ + examples/)
```

If you want machine-readable output:

```bash
ard lint --format json ./AGENTS.md
ard lint --format json .
```

## What failures look like

`ard` reports stable check IDs so teams can enforce rules without brittle parsing:

```text
[FAIL] Errors:
- DS-ID-001: Missing required frontmatter `id`. (docs/foo.md)
- DS-CF-004: Tier2+ use case appears decisionful but no ADR links it via frontmatter links.use_cases. (docs/uc-0002.md)
```

## Alternatives and tradeoffs

If your goal is “write better specs” rather than “enforce a spec format”, `spec-kit` is probably not the right tool. This project is intentionally a **format + validator** (ARSF + `ard`) so teams can gate correctness and reduce behavioral variance.

Common alternatives:

- **Docs-only templates (wikis, Markdown repos):** low friction, but no deterministic cross-linking and conformance gates; quality is harder to enforce consistently.
- **General-purpose linters (Markdown/style linters):** great for formatting and broken links, but they typically cannot enforce *spec semantics* (typed errors, required cross-doc links, decision capture).

If you want, open an issue describing your workflow and constraints; we can clarify fit and recommend an adoption path.

## Add to CI (GitHub Actions)

This is a copy/paste starting point. Pin the version you want to run in CI.

```yaml
name: ard

on:
  pull_request:
  push:
    branches: [main]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install ard
        run: |
          curl -fsSL -o install-ard.sh https://raw.githubusercontent.com/AgentReadyDocs/spec-kit/main/scripts/install_ard.sh
          sh install-ard.sh --version v0.1.0 --to ./bin
          echo "$PWD/bin" >> "$GITHUB_PATH"

      - name: Lint agent instructions and specs
        run: |
          ard lint --format github ./AGENTS.md
          ard lint --format github .
```

## What `ard lint` checks

`ard` is intentionally narrow: it validates structure and cross-file consistency so reviews stay focused on design decisions.

- `ard lint ./AGENTS.md`: required sections (`## CRITICAL`, `## Commands`) and guardrails (MUST/NEVER/ON FAIL patterns), plus lightweight link/vagueness checks (stricter with `--strict`).
- `ard lint <skill-dir>`: basic skill contract shape (expects `SKILL.md` and consistent contents).
- `ard lint .` (docset): frontmatter presence/uniqueness, internal markdown link resolution, required cross-doc links (for example use case `links.glossary` / `links.nfr`), and deterministic docset consistency gates (for example tier2+ decisionful use cases must be linked from an ADR).

## Conformance and Scaffolding

- Conformance gate: `ard conformance run --profile core|strict <root>`
- Initialize a docset: `ard init --workflow`
- Create docs: `ard new use-case ...`, `ard new adr ...`, `ard new nfr ...`, `ard new glossary ...`

## Start here

- Templates:
  - Use case: `templates/usecase.md`
  - NFR baseline: `templates/nfr.md`
  - Glossary/entities: `templates/glossary-entities.md`
  - ADR: `templates/adr.md`
  - AGENTS.md starters: `templates/agents-md-typescript.md`, `templates/agents-md-go.md`
- Rubrics:
  - Use case rubric: `rubrics/usecase-rubric.md`
  - NFR rubric: `rubrics/nfr-rubric.md`
  - Glossary/entities rubric: `rubrics/glossary-entities-rubric.md`
  - ADR rubric: `rubrics/adr-rubric.md`
  - Doc-set rubric: `rubrics/docset-rubric.md` (plus required `rubrics/rubric-guidance.md`)
- Skills:
  - Index: `skills/README.md`
  - Install into agent tools: `ard skill install --target codex --target claude --namespace spec-kit --overwrite`
- Examples:
  - Use case: `examples/uc-0001-create-widget.md`
  - Glossary/entities: `examples/glossary-entities-example.md`
  - NFR baseline: `examples/nfr-0001-widget-service.md`

Adoption guide for TS/Go repos: `docs/adoption.md`.

## `ard` from source (requires Rust toolchain)

```bash
cargo test -p ard
cargo llvm-cov -p ard --fail-under-lines 90
cargo run -p ard -- lint ./AGENTS.md
```

## Versioning and compatibility

- `ard` (the tool) uses semantic versioning (see `crates/ard/Cargo.toml`).
- ARSF (the format) is versioned separately (see `docs/arsf/versioning.md`).
- Machine output includes both `tool_version` and `standard_version` and is described by `schemas/ard/lint_result.schema.json`.
- Check IDs are designed to be stable so CI policy can key off them; if a breaking change is required, it should land as a major version bump for `ard` and be called out in release notes.

## Public content policy

All content in this repository is public and intended for open collaboration.

- Do not include confidential, customer-specific, or non-public business information.
- Do not include personal data (PII), credentials, or secrets.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE`.

## Contributing

Contributions are welcome. Read `CONTRIBUTING.md` before opening a PR.
By contributing, you agree your contributions are licensed under Apache-2.0.

## Governance (lightweight)

- Issues and PRs are the source of truth for discussion and decisions.
- Breaking behavior changes should be justified and documented (prefer ADRs where applicable).
- Maintainership is “maintainer-merge”: changes land via review + CI green; if there’s disagreement, default to the narrowest scope that preserves deterministic behavior.

## Content and data policy

- Do not contribute proprietary, confidential, or customer-identifying material.
- Do not include personal data (PII), credentials, or secrets.
- Examples must be synthetic or fully anonymized.
- You are responsible for validating fitness and compliance in your own environment.

## Trademarks

"AgentReadyDocs" is a project name. This project is not affiliated with Anthropic, OpenAI, or other platform vendors.
