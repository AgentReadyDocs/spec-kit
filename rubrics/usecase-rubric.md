# Use Case Rubric

- Total points: 100
- Pass threshold: >= 70
- Decision rule: FAIL if any Critical Fail is FAIL; else PASS if score >= 70.
- Reporting guidance: use **P1/P2/P3** priorities and **PASS/FAIL/SKIP** verification per `rubrics/rubric-guidance.md`.
- Optional grade: compute `rating = score / 20` (0..5) and map to A–F per `rubrics/rubric-guidance.md`.

## Required Inputs (Must Load)

- `rubrics/rubric-guidance.md`

Paths are relative to the `spec-kit/` repository root.

If you cannot load the required inputs, return FAIL with reason `MISSING_INPUT: rubric-guidance` and do not score.

## Critical Fails (Must PASS)

- UC-CF-001 (all): Inputs table is present and complete (field, type, required, constraints, example).
  - Evidence: `## Interface Contract` → `### Inputs`.
- UC-CF-002 (all): Outputs table is present and complete (field, type, required, constraints, example).
  - Evidence: `## Interface Contract` → `### Outputs`.
- UC-CF-003 (all): Workflow trace reduces variance in behavioral outcomes and includes `state_before` and `state_after` for each step.
  - Evidence: `## Workflow (Low-Variance Trace)`.
- UC-CF-004 (all): Typed error catalog exists and includes retryability + client action (not free-text only).
  - Evidence: `## Error Catalog (Typed)`.
- UC-CF-005 (all): Acceptance tests table exists and scenarios map to acceptance criteria (AC ids) with `test_ref` or explicit `N/A`.
  - Evidence: `## Acceptance Tests (Executable Scenarios)`.
- UC-CF-006 (tier2+): Idempotency is specified (or explicitly `N/A`) and includes behavior-on-replay.
  - Evidence: `### Idempotency (If Applicable)`.
- UC-CF-007 (tier2+): Edge cases include at least one non-happy-path case and an explicit state impact.
  - Evidence: `## Alternatives And Edge Cases`.
- UC-CF-008 (tier2+): Invariants/policies table exists with `MUST` / `MUST NOT` statements and enforcement.
  - Evidence: `## Invariants And Policies`.

## Scored Checks (100 points)

| check_id | points | applies_to | pass_criteria | evidence |
|---|---:|---|---|---|
| UC-S-001 | 8 | all | Front matter includes stable `id`, `risk_tier`, `system`, and `links.glossary`/`links.nfr` (or explicit `N/A`). | YAML front matter |
| UC-S-002 | 8 | all | Goal is a single outcome sentence; scope clearly separates in/out. | `## Goal`, `## Scope` |
| UC-S-003 | 6 | all | Actors include roles and concrete permissions/entitlements. | `## Actors` |
| UC-S-004 | 12 | all | Interface contract is complete: inputs/outputs include constraints + examples; side effects list kind/target/guarantee. | `## Interface Contract` |
| UC-S-005 | 8 | all | AuthZ rules are explicit allow/deny predicates (no “should/might”). | `### AuthZ` |
| UC-S-006 | 6 | tier0+ | Idempotency section is present and correct for the operation (filled or explicit `N/A`). | `### Idempotency (If Applicable)` |
| UC-S-007 | 14 | all | Workflow steps are implementation-grade: reads/writes/emits columns are meaningful, and state transitions are coherent. | `## Workflow (Low-Variance Trace)` |
| UC-S-008 | 8 | all | Edge cases cover at least one non-happy-path and include correct error code mapping. | `## Alternatives And Edge Cases` |
| UC-S-009 | 8 | all | Error catalog includes detection points, status mapping, and telemetry fields needed to debug. | `## Error Catalog (Typed)` |
| UC-S-010 | 6 | tier1+ | Invariants/policies are enforceable (db/code/gate) and reference validating scenarios/tests. | `## Invariants And Policies` |
| UC-S-011 | 6 | all | Acceptance tests are written as executable scenarios and cover the critical path + at least one failure path. | `## Acceptance Tests (Executable Scenarios)` |
| UC-S-012 | 6 | all | Observability defines minimum logs/metrics/traces and includes redaction requirements where needed. | `## Observability` |
| UC-S-013 | 4 | all | Open questions are empty or each has owner + due date + impact. | `## Open Questions` |
