---
name: reviewing-adr
description: >
  Reviews and scores a spec-kit ADR against rubrics/adr-rubric.md with evidence-backed PASS/FAIL/SKIP and
  prioritized P1/P2/P3 findings. Use when: grading decision statements, options, enforceable constraints,
  reversal triggers, and tier2+ linking requirements.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (read-only; no dependencies)"
license: "Apache-2.0"
---

# Review ADR

## Objective

Evaluate an ADR as a decision gate and implementation contract. This skill is read-only.

## Required inputs (must load)

- `rubrics/rubric-guidance.md`
- `rubrics/adr-rubric.md`

If required inputs cannot be loaded, return FAIL with reason `MISSING_INPUT` and do not score.

## Safety / constraints (non-negotiable)

- Read-only: do not edit/create/delete/move files.
- Do not browse the web or call external systems unless the user explicitly requests it.
- Never open or quote secrets. If encountered, stop and warn.
- Do not execute repo code/tests as verification; use read-only inspection and SKIP when execution would be required.

## Inputs

- Path to an ADR Markdown document, or the document content.

If no path/content is provided: search for files containing `ADR-` and ask the user to choose a candidate.

## Scoring

- `score` from ADR-S-001..007 (0..100)
- `rating = score / 20` (0..5)
- grade mapping per `rubrics/rubric-guidance.md`

Decision rule:
- FAIL if any Critical Fail is FAIL.
- Else PASS if `score >= 70`, otherwise FAIL.

## Tier handling

- Read `risk_tier` from front matter.
- If missing/unknown: mark tier2+ checks as SKIP and add a P2 finding to specify the tier.

## Critical Fails (must report)

Report ADR-CF-001..006 as PASS/FAIL/SKIP with evidence.

## Output format (exact)

````markdown
# ADR Review

## Overall Decision: PASS or FAIL (score: N/100, rating: R/5.0, grade: A-F)

## Inputs
- ADR: PATH_OR_ID
- Risk tier: tier0|tier1|tier2|tier3|unknown

## Critical Fails
| check_id | result | evidence |
|---|---|---|
| ADR-CF-001 | PASS|FAIL|SKIP | EVIDENCE |
| ADR-CF-002 | PASS|FAIL|SKIP | EVIDENCE |
| ADR-CF-003 | PASS|FAIL|SKIP | EVIDENCE |
| ADR-CF-004 | PASS|FAIL|SKIP | EVIDENCE |
| ADR-CF-005 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |
| ADR-CF-006 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |

## Scored Checks
| check_id | points | earned | notes | evidence |
|---|---:|---:|---|---|
| ADR-S-001 | 10 | X | 1 line | EVIDENCE |
| ADR-S-002 | 15 | X | 1 line | EVIDENCE |
| ADR-S-003 | 10 | X | 1 line | EVIDENCE |
| ADR-S-004 | 20 | X | 1 line | EVIDENCE |
| ADR-S-005 | 20 | X | 1 line | EVIDENCE |
| ADR-S-006 | 10 | X | 1 line | EVIDENCE |
| ADR-S-007 | 15 | X | 1 line | EVIDENCE |

## Findings (prioritized; max 15)

### P1|P2|P3 — Title
- Impact: what breaks or becomes ambiguous
- Evidence: section/table/row
- Recommendation: minimal patch guidance
````
