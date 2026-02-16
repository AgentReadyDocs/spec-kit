# Doc-Set Rubric (Cross-Document + Examples Gate)

- Total points: 100
- Pass threshold: >= 70
- Decision rule: FAIL if any Critical Fail is FAIL; else PASS if score >= 70.
- Reporting guidance: use **P1/P2/P3** priorities and **PASS/FAIL/SKIP** verification per `rubrics/rubric-guidance.md`.
- Optional grade: compute `rating = score / 20` (0..5) and map to A–F per `rubrics/rubric-guidance.md`.

## Required Inputs (Must Load)

- `rubrics/rubric-guidance.md`

Paths are relative to the `spec-kit/` repository root.

If you cannot load the required inputs, return FAIL with reason `MISSING_INPUT: rubric-guidance` and do not score.

## Critical Fails (Must PASS)

- DS-CF-001 (all): Every use case links to a glossary/entities doc (or explicit `N/A`).
  - Evidence: UC YAML front matter `links.glossary`.
- DS-CF-002 (all): Every use case references an NFR baseline (or explicit `N/A`).
  - Evidence: UC YAML front matter `links.nfr`.
- DS-CF-003 (all): Any domain entity referenced in a UC `## Entities (Referenced)` exists in the linked glossary/entities (as a Term or Entity).
  - Evidence: UC `## Entities (Referenced)` and linked glossary `## Terms` / `## Entities`.
- DS-CF-004 (tier2+): An ADR exists for any tier2+ UC that introduces a non-trivial policy/constraint (authz, data handling, contract, schema, money movement).
  - Evidence: UC `risk_tier`, ADR `links.use_cases`.

- EX-CF-001 (all): Example docs contain no template placeholders (e.g., `[Short title]`, `[X]`, `ACTOR_...`) unless explicitly marked as a template.
  - Evidence: `examples/*.md`.

## Scored Checks (100 points)

| check_id | points | applies_to | pass_criteria | evidence |
|---|---:|---|---|---|
| DS-S-001 | 15 | all | Internal links resolve (no missing files; relative paths correct). | doc links in front matter + markdown |
| DS-S-002 | 15 | all | Stable IDs are used consistently across docs (UC-####, ADR-####, etc.) and match titles. | headers + front matter |
| DS-S-003 | 15 | all | No contradictory definitions across glossary/entities and use cases (same entity.field not different types). | glossary + UC spot-check |
| DS-S-004 | 15 | all | Each UC’s risk tier is consistent with the NFR tier gates it claims to meet (no missing required sections). | UC + NFR `Risk Tiers` |
| DS-S-005 | 10 | all | Key policies are not duplicated inconsistently (AuthZ/Data Handling/Retention defined once and referenced). | UC/AuthZ + NFR/Data Handling + ADR |
| DS-S-006 | 10 | all | Terminology is controlled: domain terms used in UCs appear in glossary terms or entity names (no silent synonyms). | UC text + glossary `## Terms` |
| DS-S-007 | 10 | all | Examples exist for UC and Glossary/Entities and are “filled” (not templates). | `examples/uc-*.md`, `examples/glossary-*.md` |
| DS-S-008 | 10 | tier1+ | At least one NFR baseline example exists and is filled (recommended for v0). | `examples/nfr-*.md` |
