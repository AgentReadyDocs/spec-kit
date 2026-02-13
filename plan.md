# Define v0 Rubrics For AgentReadyDocs/spec-kit

## Summary

Add a minimal, enforceable rubric suite that:

- Uses 100-point scoring with a critical-fail override.
- Tightens requirements by risk tier (tier0-tiert3).
- Includes per-document rubrics plus a doc-set + examples gate.

## Scope

Target repo: `AgentReadyDocs/spec-kit`

Files to add/update under `spec-kit/`:

- Add `rubrics/nfr-rubric.md`
- Add `rubrics/glossary-entities-rubric.md`
- Add `rubrics/adr-rubric.md`
- Add `rubrics/docset-rubric.md`
- Update `rubrics/usecase-rubric.md` to match 100-point format (currently binary)
- Update `README.md` to link rubrics explicitly (optional but recommended)

## Rubric Format (Standardized Across All Rubrics)

Each rubric file uses:

- Header: name, total points, pass threshold (recommend `>= 70`)
- Section: `## Critical Fails (Must PASS)`
- Section: `## Scored Checks (100 points)`
- A single scoring table with columns:
  - `check_id`
  - `points`
  - `applies_to` (e.g., `tier0+`, `tier2+`, `all`)
  - `pass_criteria` (one sentence, testable)
  - `evidence` (expected place in doc: section/table name)

Decision rule:

- Fail if any Critical Fail is FAIL, regardless of score.
- Otherwise pass if score `>= 70`.

## Per-Doc Rubrics

### 1) Use Case Rubric (`rubrics/usecase-rubric.md`) [UPDATE]

Critical fails (examples):

- `UC-CF-001`: Inputs table complete (types + examples)
- `UC-CF-002`: Outputs table complete (types + examples)
- `UC-CF-003`: Workflow trace deterministic (state_before/state_after)
- `UC-CF-004`: Typed error catalog present
- `UC-CF-005`: Scenario table present and maps to ACs

Tier tightening:

- `tier2+`: idempotency row required (or explicit N/A), edge cases must include non-happy paths, invariants required.

### 2) NFR Baseline Rubric (`rubrics/nfr-rubric.md`) [NEW]

Critical fails:

- `NFR-CF-001`: No qualitative-only requirements (ban "fast/secure/scalable" without target)
- `NFR-CF-002`: Every metric row has a numeric target + measurement_point
- `NFR-CF-003`: Data handling MUST/MUST NOT rules exist

Tier tightening:

- `tier2+`: explicit breach consequence policy for at least availability + error budget.

Scored checks cover: performance, availability, reliability, security, observability, change control, risk tiers table present.

### 3) Glossary + Entities Rubric (`rubrics/glossary-entities-rubric.md`) [NEW]

Critical fails:

- `GE-CF-001`: Terms table present and non-empty
- `GE-CF-002`: Entity fields table present with types + examples
- `GE-CF-003`: Identifier rules defined (scope + format + example)

Tier tightening:

- `tier2+`: classification column required for all entity fields (at least public/internal/confidential/restricted).

Scored checks cover: synonym control (allowed/banned), consistency (same field name/type not contradictory), state table if referenced.

### 4) ADR Rubric (`rubrics/adr-rubric.md`) [NEW]

Critical fails:

- `ADR-CF-001`: Decision statement matches "We will X to achieve Y, and we will not Z."
- `ADR-CF-002`: At least 2 options with pros/cons
- `ADR-CF-003`: Constraints introduced MUST/MUST NOT present
- `ADR-CF-004`: Reversal triggers table present (>= 2 triggers for tier2+)

Tier tightening:

- `tier2+`: must link to impacted UC(s) and NFR entry or explicitly N/A (single cell, no prose section).

Scored checks cover: scope filled (components/domains), context facts are concrete, validation record present (can be `validated=false` initially).

## Cross-Doc + Examples Gates

### 5) Doc-Set Rubric (`rubrics/docset-rubric.md`) [NEW]

Purpose: enforce cross-document consistency without requiring tooling.

Critical fails:

- `DS-CF-001`: Every UC links to a glossary/entities doc
- `DS-CF-002`: Every UC references an NFR baseline (or explicit N/A)
- `DS-CF-003`: Terms used in UC Actors/Entities exist in glossary (manual spot-check rule)

Tier tightening:

- `tier2+`: ADR exists for any UC that introduces a non-trivial policy/constraint.

Scored checks cover: stable IDs, no broken internal links, no contradictory definitions.

### 6) Examples Gate

Require at least one filled example per template type:

- `examples/uc-*.md`
- `examples/glossary-*.md`
- (Optional for v0) `examples/adr-*.md`, `examples/nfr-*.md`

Critical fails:

- `EX-CF-001`: Example docs contain no placeholders like `[Short title]`, `[X]`, `ACTOR_...` (unless explicitly marked as template, not example).

## Important Changes / Interfaces

- No public APIs.
- Adds/standardizes rubric semantics across markdown files.
- Introduces a consistent check-id scheme (`UC-*`, `NFR-*`, `GE-*`, `ADR-*`, `DS-*`, `EX-*`) for future automation.

## Validation / Acceptance Criteria

A reviewer can validate v0 by:

1. Opening each rubric file and confirming the structure matches the standard format.
2. Applying rubrics to example docs under `examples/` and confirming they PASS at `tier1`.
3. Creating a deliberate bad doc (missing typed errors) and confirming it triggers a Critical Fail.

## Assumptions / Defaults

- Default pass threshold: `>= 70` with Critical Fail override.
- Risk tiers are the ones defined in `templates/nfr.md` ("Risk Tiers (Doc Gates)").
- "Examples gate" is manual for v0 (no linter required).

