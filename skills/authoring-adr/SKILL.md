---
name: authoring-adr
description: >
  Co-authors or updates a spec-kit ADR (ADR-####) from templates/adr.md and self-checks it against
  rubrics/adr-rubric.md. Use when: recording a decision with options, enforceable constraints, reversal
  triggers, and validation evidence (especially when tier2+ work is decisionful).
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (no dependencies)"
license: "Apache-2.0"
---

# Author ADR

## Objective

Create or update a single ADR that is decision-complete and enforceable:
- decision statement in the required form
- at least two meaningfully different options with pros/cons and why_not
- testable constraints introduced (MUST/MUST NOT) with enforcement
- reversal triggers and a validation record

## When to use

Use this skill when the user asks to:
- draft or update an ADR (ADR-####)
- justify a non-trivial policy/constraint (authz, data handling, contracts, schema changes, money movement)
- add constraints and reversal triggers required for tier2+

## When NOT to use

- Do not use for rubric-only review/grade (use `$reviewing-adr`).
- Do not use to author the associated use case unless explicitly requested.

## Safety / constraints (non-negotiable)

- Do not browse the web or call external systems unless the user explicitly requests it.
- Never read, request, or paste secrets (tokens, credentials, private keys, `.env` contents).
- Do not invent “facts”; if context is unknown, ask and record it explicitly.
- Only write files after the user approves the target path and drafted content.

## Inputs (ask if missing)

- ADR id (ADR-####) and title
- status (proposed/accepted/superseded), date (YYYY-MM-DD), owner
- risk_tier (tier0..tier3) and scope (components/domains)
- links.use_cases and links.nfr (tier2+: required or exact `N/A`)
- decision sentence: “We will X to achieve Y, and we will not Z.”
- at least two options (A/B) with pros/cons and why_not
- at least one MUST/MUST NOT constraint with enforcement
- reversal triggers (tier2+: at least two)
- validation record fields

## Workflow (decision-complete)

1. Resolve the target file
   - If updating: read existing ADR first and preserve `id`.
   - If creating: start from `templates/adr.md`.
2. Fill front matter
   - Populate `id`, `title`, `status`, `date`, `owner`, `risk_tier`, `scope`, `links`, `supersedes`, `superseded_by`.
3. Draft sections using tables (keep table structure)
   - Decision: exactly one sentence in the required form.
   - Context: facts-only rows.
   - Options: A/B with meaningful differences and concrete why_not.
   - Constraints introduced: MUST/MUST NOT with enforcement and tests/verification reference.
   - Reversal triggers: measurable triggers and explicit actions.
   - Validation record: validated flag, dates, evidence links/ids.
4. Self-check gate (hard)
   - Evaluate ADR-CF-001..006 as PASS/FAIL with evidence.
   - If any Critical Fail is FAIL, fix the ADR before proceeding.
5. Critic loop (required)
   - Run `$reviewing-adr` on the ADR path.
   - Apply P1 and P2 findings.
   - Re-run `$reviewing-adr` until Overall Decision is PASS (score >= 70 and no Critical Fail is FAIL).

## ADR rubric gate (must apply)

You must explicitly verify these Critical Fails from `rubrics/adr-rubric.md`:
- ADR-CF-001 Decision statement format
- ADR-CF-002 Two options evaluated with pros/cons and why_not
- ADR-CF-003 Constraints include at least one MUST/MUST NOT with enforcement
- ADR-CF-004 Reversal triggers table exists
- ADR-CF-005 (tier2+) links.use_cases and links.nfr present or exact `N/A`
- ADR-CF-006 (tier2+) at least two reversal triggers
