# ARSF Format (Normative)

This document defines the **normative** ARSF format. Keywords **MUST**, **SHOULD**, and **MAY** are to be interpreted as in RFC 2119.

## File Format

- Documents MUST be Markdown (`.md`).
- Documents SHOULD begin with a YAML frontmatter block delimited by `---` lines.

## Common Frontmatter Fields

All ARSF docs:

- MUST include `id: <string>` (stable identifier).
- SHOULD include `title: <string>`.
- SHOULD include `owner: <string>` (handle, email, or team alias).

Doc types MAY include additional fields described below.

## Use Case (`type: use_case`)

Frontmatter:

- MUST include `type: use_case`.
- MUST include `id: UC-####` (4 digits).
- MUST include `links.glossary` and `links.nfr` as strings, each either:
  - a relative path to an existing file, or
  - the literal string `N/A` (case-insensitive).
- SHOULD include `risk_tier: tier0|tier1|tier2|tier3`.

Body:

- MUST include an interface contract (inputs and outputs).
- MUST include a low-variance workflow trace with `state_before` and `state_after`.
- MUST include a typed error catalog and acceptance tests (as defined by the use case rubric).

## ADR (`id: ADR-####`)

Frontmatter:

- MUST include `id: ADR-####` (preferred) or a template placeholder (`ADR-XXXX`) only in templates.
- SHOULD include `status`, `date`, `risk_tier`, and `links.use_cases` / `links.nfr` where applicable.

Body:

- MUST include a Decision statement and at least 2 evaluated Options.
- MUST include enforceable constraints (MUST/MUST NOT rules) and reversal triggers.

## NFR Baseline (`type: nfr`)

Frontmatter:

- MUST include `type: nfr`.
- MUST include `id: NFR-####` (preferred) or a template id in templates.

Body:

- MUST include risk tier gates and concrete, enforceable NFR rows (targets, measurement points, enforcement).

## Glossary/Entities (`type: glossary`)

Frontmatter:

- MUST include `type: glossary`.
- MUST include a stable `id`.

Body:

- MUST define Terms and Entities as tables (see templates/rubrics).

## Links

- Local links MUST be relative paths and MUST resolve to existing files in `ARSF-Core`.
- Links MAY include a `#fragment`; validators SHOULD ignore fragments for file existence checks.
- External links MAY be present but MUST NOT be required for conformance.

## Placeholders

- Template placeholder tokens (e.g. `[Short title]`, `UC-XXXX`, `TODO`) MUST NOT appear in example docs.
- In real docs, placeholders SHOULD be explicitly marked as `[OPEN]` / `[ASSUMPTION]` with an owner and date where possible.

