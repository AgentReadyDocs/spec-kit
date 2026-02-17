---
name: authoring-nfr
description: >
  Co-authors or updates an NFR Baseline from templates/nfr.md and self-checks it against rubrics/nfr-rubric.md.
  Use when: you need measurable non-functional requirements (numeric targets, measurement points, enforcement)
  for performance, availability, reliability, security, data handling, observability, and change control.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (no dependencies)"
license: "Apache-2.0"
---

# Author NFR Baseline

## Objective

Create or update a single NFR Baseline document that is measurable and enforceable and passes the NFR rubric gate.

## Outputs

- A complete Markdown NFR Baseline (paste-ready), matching `templates/nfr.md`.
- A short self-check summary table for NFR-CF-001..004 (PASS/FAIL with evidence pointers).

## When to use

Use this skill when the user asks to:
- draft or update an NFR baseline for a system/product
- turn qualitative NFRs into measurable targets with explicit measurement points and enforcement
- define risk tiers and doc gates for the project

## When NOT to use

- Do not use for rubric-only review/grade (use `$reviewing-nfr`).

## Safety / constraints (non-negotiable)

- Do not browse the web or call external systems unless the user explicitly requests it.
- Never read, request, or paste secrets (tokens, credentials, private keys, `.env` contents).
- Do not invent numeric targets; ask for targets/units and measurement methods. If unknown, record as an open question and state that the rubric gate will FAIL until resolved.
- Only write files after the user approves the target path and drafted content.

## Workflow (decision-complete)

1. Resolve the target file
   - If updating: read existing NFR first and preserve `id`.
   - If creating: start from `templates/nfr.md`.
2. Determine tier (required)
   - Ask for the project/system tier (tier0..tier3). If unknown, record as an open question and note that tier2+ rubric gates cannot be satisfied yet.
3. Fill front matter
   - `id`, `type: nfr`, `title`, `owner`, `last_updated` (YYYY-MM-DD)
4. Fill each table with concrete requirements
   - Every metric row has a numeric target and a measurement point/method.
   - Every row includes an enforcement mechanism (lint/review/test/runtime/release gate).
5. Tier2+ breach handling
   - Add explicit consequence/response when availability/error budget targets are violated.
6. Self-check gate (hard)
   - Evaluate NFR-CF-001..004 as PASS/FAIL with evidence.
7. Critic loop (required)
   - Run `$reviewing-nfr` and apply P1/P2 findings, then re-check Critical Fails.

## NFR rubric gate (must apply)

Critical Fails from `rubrics/nfr-rubric.md`:
- NFR-CF-001 No qualitative-only requirements without numeric target or explicit measurement method
- NFR-CF-002 Every metric row has numeric target and non-empty measurement_point/method
- NFR-CF-003 Data handling includes at least one MUST and one MUST NOT with scope + enforcement
- NFR-CF-004 (tier2+) Explicit breach/consequence handling for availability and error budget
