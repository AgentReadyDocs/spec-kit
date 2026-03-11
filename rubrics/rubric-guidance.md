# Rubric Guidance (Applies To All `spec-kit/rubrics/*`)

This file defines how to interpret `spec-kit` rubrics when evaluating documents.

Paths in this file are relative to the `spec-kit/` repository root unless stated otherwise.

## Quality Alignment (How to Interpret Rubrics)

Rubrics are **correctness-first**: a PASS indicates the document(s) define acceptance criteria, invariants, and interface/behavior constraints that are sufficient to implement correctly.

Rubrics also aim to **reduce variance in behavioral outcomes** by making externally observable behavior explicit (contracts, state transitions, side effects, typed errors, and replay/idempotency behavior when applicable), so independent implementations converge on the same outcomes.

Token efficiency is interpreted as **minimal sufficient detail** achieved through structure and non-duplication, not by omitting constraints.

Canonical definitions (including non-goals) are in `../docs/agent-ready-quality.md`.

## Behavioral Outcomes (Quick Reference)

Behavioral outcomes are the externally observable semantics, typically including:

- Inputs/outputs and constraints (types, formats, requiredness).
- State transitions and invariants.
- Side effects (writes, external calls, emitted events) and their guarantees.
- Errors (typed codes), status mapping, retryability, and client action.
- Idempotency/replay behavior when applicable.

## Avoid Implementation Details (Correctness + Reduced Variance)

Specifications should describe **what** the system does (observable outcomes), not **how** it is implemented, unless the mechanism changes observable behavior.

Prefer:
- “When entity X becomes stale, children Y are deleted and nullable FK Z is set to NULL.”
- “Junction links are fully refreshed each run; manually-added links for migration-owned rows are not preserved.”

Avoid (unless required for observable outcomes):
- Exact execution order (“18-step deletion order”) that is only an implementation strategy.
- Algorithm internals (“5-step email dedup algorithm”) when any correct implementation is acceptable.
- Internal data structures, private modules, or threading/concurrency mechanics.

If a mechanism is required (for example due to external system constraints), state it as an **invariant/guarantee** (“MUST be safe under replays”, “MUST not violate FK constraints”) rather than prescribing a specific internal sequence.

## How To Apply Any Rubric (For Agents/Reviewers)

1. Load this file (`rubrics/rubric-guidance.md`) plus the specific rubric file (e.g., `rubrics/usecase-rubric.md`).
2. For any rule that references linked docs (glossary/NFR/ADR), load those docs too.
3. Evaluate **Critical Fails** first (binary PASS/FAIL).
4. If no Critical Fail FAILs, score the **Scored Checks** and apply the pass threshold.

## Scoring, Ratings, And Grade Bands

Most `spec-kit/rubrics/*` use a 100-point score plus a pass threshold.

- `score`: sum of earned points (0..100)
- `rating`: normalized to 0..5 as `rating = score / 20`
- `grade` (from `rating`):
  - **A:** 4.5–5.0
  - **B:** 3.5–4.49
  - **C:** 2.5–3.49
  - **D:** 1.5–2.49
  - **F:** < 1.5

If a rubric has a different scoring system, follow that rubric's explicit instructions and treat this section as guidance only.

## Findings Priorities (P1/P2/P3)

When reporting findings, label each with a priority:

- **P1 (Critical):** likely to cause broken workflows, unsafe actions, or repeated failure loops.
- **P2 (Important):** likely to waste tokens/time, reduce output quality, or force repeated clarification.
- **P3 (Nice):** polish, readability, and future-proofing.

## Evidence-Backed Verification (PASS/FAIL/SKIP)

Prefer check-by-check verification with **PASS/FAIL/SKIP** results.

- **PASS:** the required artifact/constraint is present and correct.
- **FAIL:** missing or incorrect; include evidence (what was checked and where).
- **SKIP:** verification would require executing code, accessing external systems, or secrets.

Evidence should be specific and local when possible: a section heading name, a table name, or a referenced identifier.
