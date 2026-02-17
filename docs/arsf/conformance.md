# ARSF Conformance

## Profiles

### ARSF-Core

A docset is ARSF-Core conformant when:

- Every doc has a stable `id` in frontmatter.
- Doc IDs are unique across the docset.
- Use cases include required cross-links (`links.glossary`, `links.nfr`) or explicit `N/A`.
- Local markdown links resolve to existing files.
- Cross-document consistency checks pass (for example: UC referenced entities exist in the linked glossary).

### ARSF-Strict

ARSF-Strict adds:

- Warnings are treated as errors (for supported checks).
- Placeholder tokens are not permitted outside explicitly declared open questions / assumptions.
- Vague directives are treated as failures when they reduce determinism.

## Validator

The reference validator is `ard`. Recommended CI usage:

```bash
ard lint --strict --format github ./AGENTS.md
ard lint --strict --format github .
```

## Stable Check IDs

Validators MUST report stable check IDs for failures so teams can gate on behavior without brittle parsing.

