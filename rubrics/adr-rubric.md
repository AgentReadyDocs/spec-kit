# ADR Rubric

- Total points: 100
- Pass threshold: >= 70
- Decision rule: FAIL if any Critical Fail is FAIL; else PASS if score >= 70.

## Required Inputs (Must Load)

- `rubrics/rubric-guidance.md`

Paths are relative to the `spec-kit/` repository root.

If you cannot load the required inputs, return FAIL with reason `MISSING_INPUT: rubric-guidance` and do not score.

## Critical Fails (Must PASS)

- ADR-CF-001 (all): Decision statement matches “We will X to achieve Y, and we will not Z.”
  - Evidence: `## Decision`.
- ADR-CF-002 (all): At least 2 options are evaluated with pros/cons and a concrete reason why not chosen.
  - Evidence: `## Options`.
- ADR-CF-003 (all): Constraints introduced include at least one `MUST` / `MUST NOT` rule with enforcement.
  - Evidence: `## Constraints Introduced`.
- ADR-CF-004 (all): Reversal triggers table exists.
  - Evidence: `## Reversal Triggers`.
- ADR-CF-005 (tier2+): Links include impacted use case(s) and an NFR baseline reference, or each is explicitly `N/A` (single cell, no prose).
  - Evidence: YAML front matter `links.use_cases` and `links.nfr`.
- ADR-CF-006 (tier2+): Reversal triggers include at least 2 triggers.
  - Evidence: `## Reversal Triggers`.

## Scored Checks (100 points)

| check_id | points | applies_to | pass_criteria | evidence |
|---|---:|---|---|---|
| ADR-S-001 | 10 | all | Front matter is complete (id/status/date/owner/risk_tier/scope/links) with stable identifiers. | YAML front matter |
| ADR-S-002 | 15 | all | Context is facts-only and concrete (no “should/might”); includes constraints as facts where applicable. | `## Context (Facts Only)` |
| ADR-S-003 | 10 | all | Scope lists affected components/domains and stays consistent with linked UCs. | YAML `scope` + `links` |
| ADR-S-004 | 20 | all | Options are meaningfully different and compare on relevant dimensions (cost, risk, operability). | `## Options` |
| ADR-S-005 | 20 | all | Constraints introduced are testable/enforceable and include where they are enforced (lint/review/test/runtime). | `## Constraints Introduced` |
| ADR-S-006 | 10 | all | Policy rules (if present) are scoped, enforceable, and non-overlapping with UC/NFR without contradiction. | `## Policy Rules (If Applicable)` |
| ADR-S-007 | 15 | all | Validation record exists and indicates current validation status plus evidence links (even if validated=false). | `## Validation Record` |
