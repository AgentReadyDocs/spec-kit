# Agent-Ready Quality: Correctness First, Reduced Variance Second

Agent-ready documentation is written to enable correct implementation with minimal back-and-forth by removing or explicitly marking decision points.

## Quality Definition

- **Correctness first:** acceptance criteria, invariants, and interface contracts define what “right” means.
- **Repeatability as reduced variance:** independent implementations should converge on the same **behavioral outcomes** (externally observable behavior), even if internal structure differs.
- **Token efficiency:** use minimal sufficient detail via structure and non-duplication; do not omit required constraints to be shorter.

## Behavioral Outcomes (What Must Converge)

Behavioral outcomes are the externally observable semantics, including:

- Inputs/outputs and constraints (types, formats, requiredness).
- State transitions and invariants.
- Side effects (writes, external calls, emitted events) and their guarantees.
- Errors (typed codes), status mapping, retryability, and client action.
- Idempotency/replay behavior when applicable.

## How Docs Reduce Variance In Outcomes

Use structured artifacts that remove ambiguity:

- **Contracts:** explicit inputs/outputs, constraints, and side effects.
- **Low-variance workflow traces:** step-by-step actions with reads/writes/emits and `state_before`/`state_after`.
- **Typed error catalogs:** detection points, retryability, client action, and telemetry fields.
- **Acceptance tests as scenarios:** “given/when/then” statements that validate outcomes and invariants.
- **Controlled terminology:** define terms/entities once and reference them consistently across the doc set.

## Non-goals

- Identical code or identical artifacts across implementations.
- “Guaranteed one-shot” implementation success claims.
- Any evaluation infrastructure beyond generating excellent agent-ready documentation.
