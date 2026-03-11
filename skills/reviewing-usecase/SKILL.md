---
name: reviewing-usecase
description: >
  Reviews and scores a spec-kit Use Case document against rubrics/usecase-rubric.md using evidence-backed
  PASS/FAIL/SKIP and prioritized P1/P2/P3 findings. Use when: grading a use case spec for implementability,
  low-variance workflow trace, typed errors, idempotency, invariants, and acceptance scenarios.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (read-only; no dependencies)"
license: "Apache-2.0"
---

# Review Use Case

## Objective

Evaluate a use case document from the perspective of an implementer and release gate. Produce a structured report:
- Critical Fails results (PASS/FAIL/SKIP) with evidence
- scored checks to compute score/rating/grade
- prioritized P1/P2/P3 findings with minimal patch guidance

This skill is read-only: it reports problems but does not modify files.

## Inputs

- Path to a use case Markdown document, or the document content.

If no path/content is provided:
- Search for likely candidates (`examples/uc-*.md`, `docs/uc-*.md`) and ask the user to choose.

## Required inputs (must load)

- `rubrics/rubric-guidance.md`
- `rubrics/usecase-rubric.md`

If you cannot load required inputs, return FAIL with reason `MISSING_INPUT` and do not score.

## Safety / constraints (non-negotiable)

- Read-only: do not edit/create/delete/move files.
- Do not browse the web or call external systems unless the user explicitly requests it.
- Never open or quote secrets. If encountered, stop and warn.
- Do not execute repo code/tests as verification; use read-only inspection and SKIP when execution would be required.

## Scoring

- Compute `score` (0..100) from the rubric’s scored checks.
- Compute `rating = score / 20` (0..5).
- Map `rating` to grade per `rubrics/rubric-guidance.md`:
  - A: 4.5–5.0, B: 3.5–4.49, C: 2.5–3.49, D: 1.5–2.49, F: < 1.5.

Decision rule:
- FAIL if any Critical Fail is FAIL.
- Else PASS if `score >= 70`, otherwise FAIL.

## Tier handling

- Read `risk_tier` from YAML front matter.
- If `risk_tier` is missing/unknown: mark tier2+ checks as SKIP and add a P2 finding to specify the tier.
- When the rubric allows conditional sections (for example Actors/AuthZ/Observability), treat an explicit single-line `N/A` as satisfying the check.

## Critical Fails (must report)

Report each Critical Fail from `rubrics/usecase-rubric.md` with PASS/FAIL/SKIP + evidence:
- UC-CF-001..UC-CF-005 (all)
- UC-CF-006..UC-CF-008 (tier2+)

## Output format (exact)

````markdown
# Use Case Review

## Overall Decision: PASS or FAIL (score: N/100, rating: R/5.0, grade: A-F)

## Inputs
- Use case: PATH_OR_ID
- Risk tier: tier0|tier1|tier2|tier3|unknown

## Critical Fails
| check_id | result | evidence |
|---|---|---|
| UC-CF-001 | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-002 | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-003 | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-004 | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-005 | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-006 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-007 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |
| UC-CF-008 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |

## Scored Checks
| check_id | points | earned | notes | evidence |
|---|---:|---:|---|---|
| UC-S-001 | 8 | X | 1 line | EVIDENCE |
| UC-S-002 | 8 | X | 1 line | EVIDENCE |
| UC-S-003 | 6 | X | 1 line | EVIDENCE |
| UC-S-004 | 12 | X | 1 line | EVIDENCE |
| UC-S-005 | 8 | X | 1 line | EVIDENCE |
| UC-S-006 | 6 | X | 1 line | EVIDENCE |
| UC-S-007 | 14 | X | 1 line | EVIDENCE |
| UC-S-008 | 8 | X | 1 line | EVIDENCE |
| UC-S-009 | 8 | X | 1 line | EVIDENCE |
| UC-S-010 | 6 | X | 1 line | EVIDENCE |
| UC-S-011 | 6 | X | 1 line | EVIDENCE |
| UC-S-012 | 6 | X | 1 line | EVIDENCE |
| UC-S-013 | 4 | X | 1 line | EVIDENCE |

## Strengths
- 3 to 6 evidence-backed bullets

## Findings (prioritized; max 15)

### P1|P2|P3 — Title
- Impact: what breaks or becomes ambiguous
- Evidence: section/table/row
- Recommendation: minimal patch guidance
````
