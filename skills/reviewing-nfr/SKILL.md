---
name: reviewing-nfr
description: >
  Reviews and scores an NFR Baseline against rubrics/nfr-rubric.md using evidence-backed PASS/FAIL/SKIP and
  prioritized P1/P2/P3 findings. Use when: grading numeric targets, measurement points, enforcement, data handling
  MUST/MUST NOT rules, and tier2+ breach handling.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (read-only; no dependencies)"
license: "Apache-2.0"
---

# Review NFR Baseline

## Required inputs (must load)

- `rubrics/rubric-guidance.md`
- `rubrics/nfr-rubric.md`

If required inputs cannot be loaded, return FAIL with reason `MISSING_INPUT` and do not score.

## Safety / constraints (non-negotiable)

- Read-only: do not edit/create/delete/move files.
- Do not browse the web or call external systems unless explicitly requested.
- Never request or quote secrets (tokens, credentials, private keys, `.env`). If encountered, stop and warn.
- Do not execute repo code/tests as verification; use read-only inspection and SKIP when execution would be required.

## Scoring

- `score` from NFR-S-001..009 (0..100)
- `rating = score / 20` (0..5)
- grade mapping per `rubrics/rubric-guidance.md`

Decision rule:
- FAIL if any Critical Fail is FAIL.
- Else PASS if `score >= 70`, otherwise FAIL.

## Critical Fails (must report)

Report NFR-CF-001..004 as PASS/FAIL/SKIP with evidence.

If a required section/table is missing, mark the relevant checks as FAIL and use evidence like “Missing `## Availability` section” (do not guess).

If the tier cannot be determined from the doc, mark NFR-CF-004 as SKIP and add a P2 finding to specify tier and breach handling.

## Output format (exact)

````markdown
# NFR Baseline Review

## Overall Decision: PASS or FAIL (score: N/100, rating: R/5.0, grade: A-F)

## Critical Fails
| check_id | result | evidence |
|---|---|---|
| NFR-CF-001 | PASS|FAIL|SKIP | EVIDENCE |
| NFR-CF-002 | PASS|FAIL|SKIP | EVIDENCE |
| NFR-CF-003 | PASS|FAIL|SKIP | EVIDENCE |
| NFR-CF-004 (tier2+) | PASS|FAIL|SKIP | EVIDENCE |

## Scored Checks
| check_id | points | earned | notes | evidence |
|---|---:|---:|---|---|
| NFR-S-001 | 15 | X | 1 line | EVIDENCE |
| NFR-S-002 | 12 | X | 1 line | EVIDENCE |
| NFR-S-003 | 10 | X | 1 line | EVIDENCE |
| NFR-S-004 | 15 | X | 1 line | EVIDENCE |
| NFR-S-005 | 10 | X | 1 line | EVIDENCE |
| NFR-S-006 | 10 | X | 1 line | EVIDENCE |
| NFR-S-007 | 8 | X | 1 line | EVIDENCE |
| NFR-S-008 | 10 | X | 1 line | EVIDENCE |
| NFR-S-009 | 10 | X | 1 line | EVIDENCE |

## Findings (prioritized; max 15)

### P1|P2|P3 — Title
- Impact: what breaks or becomes ambiguous
- Evidence: section/table/row
- Recommendation: minimal patch guidance
````
