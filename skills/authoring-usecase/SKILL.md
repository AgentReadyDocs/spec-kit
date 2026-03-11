---
name: authoring-usecase
description: >
  Co-authors or updates a spec-kit Use Case document from templates/usecase.md and self-checks it
  against rubrics/usecase-rubric.md. Use when: creating or tightening a UC-#### use case spec to
  reduce variance (contract, workflow trace, typed errors, idempotency, invariants, acceptance tests).
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (no dependencies)"
license: "Apache-2.0"
---

# Author Use Case

## Objective

Produce a single use case Markdown document that is implementation-grade and rubric-ready:
- explicit interface contract (inputs/outputs/constraints/examples)
- low-variance workflow trace (table or numbered steps; includes state_before/state_after per step)
- typed errors (codes, retryability, client action; status mapping for APIs or exit_code mapping for CLI/batch)
- idempotency/replay behavior when applicable (required for tier2+)
- enforceable invariants/policies
- executable-style acceptance scenarios (detailed or catalog format, with traceability via `validates`)

## When to use

Use this skill when the user asks to:
- draft a new use case doc (`UC-####`, usually `docs/uc-*.md` or `examples/uc-*.md`)
- upgrade an existing use case to pass the use case rubric gate
- reduce ambiguity around state transitions, errors, replay behavior, and acceptance tests

## When NOT to use

- Do not use for rubric-only review/grade (use `$reviewing-usecase`).
- Do not use to write ADR/NFR/glossary docs except for small supporting edits explicitly requested.

## Safety / constraints (non-negotiable)

- Do not browse the web or call external systems unless the user explicitly requests it.
- Never read, request, or paste secrets (tokens, credentials, private keys, `.env` contents).
- Do not invent external system behavior or numeric targets; ask targeted questions and record unknowns as open questions with owner + due date (YYYY-MM-DD).
- Only write files after the user approves the target path and the drafted content.

## Inputs (ask if missing)

Minimum:
- `id` (UC-####), `title`, `owner`, `system`, `risk_tier` (tier0..tier3)
- `links.glossary` and `links.nfr` as relative paths or exact `N/A`
- primary actors and required permissions/entitlements (or explicit `N/A` for non-interactive/offline use cases)
- inputs/outputs field definitions (type, required, constraints, example)
- side effects and guarantees
- AuthZ allow/deny predicates (or explicit `N/A` when no runtime AuthZ exists)
- error codes and client actions
- acceptance scenarios
- verification/post-conditions checks (if the system has explicit verify/reconcile/health check behaviors)

If the operation can be replayed (retries, double-clicks, webhook re-delivery), require an idempotency decision.

## Workflow (decision-complete)

1. Resolve the target file
   - If updating: read the existing doc first and preserve `id`.
   - If creating: start from `templates/usecase.md`.
2. Fill YAML front matter
   - `id`, `type: use_case`, `title`, `status`, `owner`, `risk_tier`, `system`
   - `links.glossary` and `links.nfr` must be a relative path or exact `N/A` (single token, no prose).
3. Write the interface contract first
   - Inputs/Outputs tables: every row has `constraints` and a concrete `example`.
   - Side effects: enumerate writes/calls/emits and the guarantee.
4. Make AuthZ and idempotency explicit
   - AuthZ rules are predicates; no “should/might”, or explicit `N/A` when not applicable.
   - Tier2+: idempotency must be specified or explicitly `N/A` with behavior_on_replay.
5. Create the low-variance workflow trace
   - Choose a workflow format:
     - Table format for linear request/response flows, or
     - Numbered `### Step N: ...` subsections for complex pipelines.
   - Each step includes reads/writes/emits plus state_before/state_after.
   - Validation and error emission are explicit steps.
6. Bind edge cases to typed errors
   - Every edge case maps to an error_code in the typed catalog.
7. Add enforceable invariants/policies
   - Use MUST/MUST NOT statements plus enforcement location.
8. Write acceptance tests as executable scenarios
   - Use detailed (Given/When/Then) format for small suites; use catalog format for large suites (for example 15+ scenarios).
   - Every scenario includes `scenario_id`, `test_ref` (or exact `N/A`), and a `validates` mapping to spec IDs (for example `INV-*`, `ALT-*`, `VER-*`, `EFF-*`).
9. Self-check gate (hard)
   - Evaluate UC-CF-001..008 as PASS/FAIL with evidence.
   - If any Critical Fail is FAIL, fix the document before proceeding.
10. Critic loop (required)
   - Run `$reviewing-usecase` on the doc path.
   - Apply P1 and P2 findings, then re-run the self-check gate.

## Use case rubric gate (must apply)

You must explicitly verify these Critical Fails from `rubrics/usecase-rubric.md`:
- UC-CF-001 Inputs table complete
- UC-CF-002 Outputs table complete
- UC-CF-003 Workflow trace includes state_before/state_after
- UC-CF-004 Typed error catalog exists and includes retryability + client action
- UC-CF-005 Acceptance tests table exists and maps scenarios to criteria
- UC-CF-006 (tier2+) Idempotency specified or explicit `N/A` with replay behavior
- UC-CF-007 (tier2+) Edge cases include non-happy-path + explicit state impact
- UC-CF-008 (tier2+) Invariants/policies table exists with MUST/MUST NOT + enforcement

Decision rule:
- FAIL if any Critical Fail is FAIL.
- Else PASS if the computed scored-checks total is >= 70/100 per UC-S-001..013 in `rubrics/usecase-rubric.md`; otherwise FAIL.
