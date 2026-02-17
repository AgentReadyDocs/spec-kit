---
name: reviewing-glossary-entities
description: >
  Reviews and scores a Glossary + Entities doc against rubrics/glossary-entities-rubric.md using evidence-backed
  PASS/FAIL/SKIP and prioritized P1/P2/P3 findings. Use when: grading controlled terms, identifier rules, entity
  field definitions (types/constraints/examples), and tier2+ field classification.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (read-only; no dependencies)"
license: "Apache-2.0"
---

# Review Glossary + Entities

## Required inputs (must load)

- `rubrics/rubric-guidance.md`
- `rubrics/glossary-entities-rubric.md`

If required inputs cannot be loaded, return FAIL with reason `MISSING_INPUT` and do not score.

## Safety / constraints (non-negotiable)

- Read-only: do not edit/create/delete/move files.
- Do not browse the web or call external systems unless explicitly requested.
- Never request or quote secrets (tokens, credentials, private keys, `.env`). If encountered, stop and warn.
- Do not execute repo code/tests as verification; use read-only inspection and SKIP when execution would be required.

## Scoring

- `score` from GE-S-001..007 (0..100)
- `rating = score / 20` (0..5)
- grade mapping per `rubrics/rubric-guidance.md`

Decision rule:
- FAIL if any Critical Fail is FAIL.
- Else PASS if `score >= 70`, otherwise FAIL.

## Critical Fails (must report)

Report GE-CF-001..004 as PASS/FAIL/SKIP with evidence.

Tier evidence should be explicit (for example a frontmatter tier field or a clearly labeled “Tier” value). If not present, do not infer: treat tier2+ checks as SKIP and add a P2 finding to specify tier.

If the tier cannot be determined, mark GE-CF-004 as SKIP and add a P2 finding to specify tier. If the doc is explicitly tier2+, missing classifications means GE-CF-004 is FAIL (P1).

## Output format (exact)

````markdown
# Glossary + Entities Review

## Overall Decision: PASS or FAIL (score: N/100, rating: R/5.0, grade: A-F)

## Critical Fails
| check_id | result | evidence |
|---|---|---|
| GE-CF-001 | PASS|FAIL|SKIP | EVIDENCE |
| GE-CF-002 | PASS|FAIL|SKIP | EVIDENCE |
| GE-CF-003 | PASS|FAIL|SKIP | EVIDENCE |
| GE-CF-004 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |

## Scored Checks
| check_id | points | earned | notes | evidence |
|---|---:|---:|---|---|
| GE-S-001 | 20 | X | 1 line | EVIDENCE |
| GE-S-002 | 15 | X | 1 line | EVIDENCE |
| GE-S-003 | 10 | X | 1 line | EVIDENCE |
| GE-S-004 | 25 | X | 1 line | EVIDENCE |
| GE-S-005 | 15 | X | 1 line | EVIDENCE |
| GE-S-006 | 10 | X | 1 line | EVIDENCE |
| GE-S-007 | 5 | X | 1 line | EVIDENCE |

## Findings (prioritized; max 15)

### P1|P2|P3 — Title
- Impact: what breaks or becomes ambiguous
- Evidence: section/table/row
- Recommendation: minimal patch guidance
````
