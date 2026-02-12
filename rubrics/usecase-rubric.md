# Use Case Rubric (Binary Gate)

Score each item as PASS/FAIL. A use case is "ready" only if all required items PASS.

## Required
- ID present and stable (UC-XXXX)
- Goal is one sentence outcome (not rationale)
- Inputs table complete (types + examples)
- Outputs table complete (types + examples)
- AuthZ rules specified (allow/deny conditions)
- Idempotency specified or explicitly N/A
- Workflow trace is deterministic (state_before/state_after defined)
- Edge cases table includes non-happy paths
- Error catalog is typed (codes + retryability + client action)
- Invariants use MUST/MUST NOT
- Scenario table maps to ACs and has a `test_ref` (or explicit N/A)
- Open questions table is empty OR each row has owner+due

## Disqualifiers
- Qualitative NFR words without targets ("fast", "secure") inside UC
- Undefined terms or entities not in glossary/entities
- Hidden decisions ("should", "might", "later") without being an open question

