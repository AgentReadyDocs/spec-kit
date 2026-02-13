---
id: ADR-XXXX
title: "[Short title]"
status: proposed  # proposed | accepted | superseded
date: YYYY-MM-DD
owner: "@owner"
risk_tier: tier1  # tier0 | tier1 | tier2 | tier3
scope:
  components: []
  domains: []
links:
  use_cases: []
  nfr: ""
supersedes: []
superseded_by: []
---

# ADR-XXXX: [Short title]

Risk tiers are defined in `templates/nfr.md` under "Risk Tiers (Doc Gates)".

Review this document against `rubrics/adr-rubric.md` and `rubrics/rubric-guidance.md`.

## Decision
| decision |
|----------|
| We will [DO X] to achieve [Y], and we will not [DO Z]. |

## Context (Facts Only)
| fact_id | fact |
|---------|------|
| CTX-001 | [current state] |
| CTX-002 | [constraint] |

## Options
| option_id | option | pros | cons | why_not |
|-----------|--------|------|------|---------|
| A | [option] | [bullets] | [bullets] | [reason] |
| B | [option] | [bullets] | [bullets] | [reason] |

## Constraints Introduced
| constraint_id | rule (MUST / MUST NOT) | enforcement | tests |
|---------------|-------------------------|------------|-------|
| CONS-001 | [rule] | [review/lint/test/runtime] | [refs] |

## Policy Rules (If Applicable)
| rule_id | rule | scope | enforcement |
|---------|------|-------|------------|
| RULE-001 | [rule] | [scope] | [enforcement] |

## Reversal Triggers
| trigger_id | trigger | action |
|------------|---------|--------|
| REV-001 | [condition] | [revisit/supersede] |

## Validation Record
| validated | last_reviewed | next_review | evidence_links |
|----------|---------------|-------------|----------------|
| true/false | YYYY-MM-DD | YYYY-MM-DD | [links] |
