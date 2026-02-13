# Glossary + Entities Rubric

- Total points: 100
- Pass threshold: >= 70
- Decision rule: FAIL if any Critical Fail is FAIL; else PASS if score >= 70.

## Critical Fails (Must PASS)

- GE-CF-001 (all): Terms table exists and is non-empty.
  - Evidence: `## Terms`.
- GE-CF-002 (all): Entity fields table exists and includes types and concrete examples for each listed field.
  - Evidence: `## Entity Fields`.
- GE-CF-003 (all): Identifier rules are defined with scope + format + example (not “TBD”).
  - Evidence: `## Identifiers`.
- GE-CF-004 (tier2+): Every entity field has a `classification` value (at least public/internal/confidential/restricted).
  - Evidence: `## Entity Fields` → `classification` column.

## Scored Checks (100 points)

| check_id | points | applies_to | pass_criteria | evidence |
|---|---:|---|---|---|
| GE-S-001 | 20 | all | Terms have one-line definitions and define allowed/banned synonyms when ambiguity exists. | `## Terms` |
| GE-S-002 | 15 | all | Identifiers specify scope, format, and at least one example; notes capture immutability/uniqueness if relevant. | `## Identifiers` |
| GE-S-003 | 10 | all | Entities list is present and includes identifier + source of truth for each entity. | `## Entities` |
| GE-S-004 | 25 | all | Entity fields include type, required, constraints, classification, and example, with no placeholder values. | `## Entity Fields` |
| GE-S-005 | 15 | all | Naming is consistent: the same entity.field uses the same type/constraints everywhere it appears in the doc set. | `## Entity Fields` (+ doc spot-check) |
| GE-S-006 | 10 | all | If the domain uses state (status/phase), a state table exists and references the transition source (UC/ADR). | `## State (If Applicable)` |
| GE-S-007 | 5 | all | Classification choices are plausible and align with data-handling rules (e.g., avoid logging confidential fields). | `## Entity Fields` (+ NFR/Data Handling) |

