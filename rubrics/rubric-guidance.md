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

## How To Apply Any Rubric (For Agents/Reviewers)

1. Load this file (`rubrics/rubric-guidance.md`) plus the specific rubric file (e.g., `rubrics/usecase-rubric.md`).
2. For any rule that references linked docs (glossary/NFR/ADR), load those docs too.
3. Evaluate **Critical Fails** first (binary PASS/FAIL).
4. If no Critical Fail FAILs, score the **Scored Checks** and apply the pass threshold.
