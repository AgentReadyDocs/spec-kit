# ARSF (Agent-Ready Spec Format)

ARSF is a **standard spec format** designed to reduce variance in externally observable behavior across implementations.

It is intentionally:

- **Interoperable:** Markdown + YAML frontmatter, with clear rules that any tool can validate.
- **Versioned:** format changes are managed with explicit compatibility rules.
- **Enforceable:** a reference validator (`ard`) provides stable check IDs and machine-readable results.

This repository (`spec-kit`) is the reference kit for ARSF: templates, rubrics, examples, and the reference CLI.

## Scope (ARSF v1)

ARSF v1 covers the core docset:

- Use Case (`type: use_case`, id `UC-####`)
- Architecture Decision Record (ADR) (`id: ADR-####` or `ADR-XXXX` template)
- NFR baseline (`type: nfr`, id `NFR-####`)
- Glossary/Entities (`type: glossary`)

Plus cross-document linking rules (for example: Use Case must link to a Glossary and an NFR baseline or explicitly `N/A`).

## Conformance

ARSF defines two conformance profiles:

- `ARSF-Core`: required fields, required sections, and required cross-links.
- `ARSF-Strict`: Core + tighter placeholder/vagueness rules suitable for CI gating.

See:

- `docs/arsf/format.md`
- `docs/arsf/conformance.md`
- `docs/arsf/versioning.md`

## Adoption (GitHub Actions)

Use `ard lint --format github ...` in CI so failures show as PR annotations.

Copy/paste workflow:

- `docs/arsf/workflows/github-actions.yml`

