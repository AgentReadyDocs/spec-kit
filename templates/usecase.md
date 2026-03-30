---
id: UC-XXXX
type: use_case
title: "[Short title]"
status: draft  # draft | ready
owner: "@owner"
risk_tier: tier1  # tier0 | tier1 | tier2 | tier3
system: "[System name]"
tier: ~  # 1 (journey) | 2 (workflow) | 3 (building block) | ~ if not applicable
journey_parent: ~  # UC-XXXX for Tier 2/3; ~ or N/A for Tier 1 / standalone
service: ~  # bounded context / service that owns this UC; ~ if not applicable
capabilities: []  # capability IDs this UC implements (e.g., [POG-001, FIX-006]); [] if not applicable
links:
  glossary: "../templates/glossary-entities.md"
  nfr: "../templates/nfr.md"
code_refs:
  implementation: ""
  tests: ""
---

# UC-XXXX: [Short title]

Risk tiers are defined in `templates/nfr.md` under "Risk Tiers (Doc Gates)".

Review this document against `rubrics/usecase-rubric.md` and `rubrics/rubric-guidance.md`.

**Guidance (critical): Specify observable behavior only.**
- Specify inputs, outputs, state changes, side effects, invariants, and errors.
- Avoid internal implementation details (execution order, algorithms, data structures, mechanisms) unless they change observable outcomes.
- This spec should be sufficient to reimplement the module without access to existing source code.

## Goal
| goal |
|------|
| [One sentence outcome] |

## Scope
| in_scope | out_of_scope |
|----------|--------------|
| [bullet list] | [bullet list] |

## Capabilities Referenced (If Applicable)

Use this section when the UC implements capabilities extracted from an existing system (rewrite/migration projects). Maps many-to-many from this UC to source capability IDs.

| capability_id | name | domain |
|---------------|------|--------|
| [CAP-001] | [capability name] | [domain name] |

## Essential Complexity (If Applicable)

Use this section to document inherent domain complexity that any implementation must handle. Reference complexity IDs from a complexity inventory if one exists.

| complexity_id | description | how_preserved |
|---------------|-------------|---------------|
| [E-01] | [what makes this inherently complex] | [how the rewrite handles it] |

## Simplification Notes (If Applicable)

Use this section to document what incidental complexity is eliminated in the rewrite and why. This enforces forward-looking design rather than copying the current system.

| eliminated | current_approach | rewrite_approach | rationale |
|------------|------------------|------------------|-----------|
| [I-01 description] | [how it works today] | [how the rewrite does it] | [why the change] |

## Actors (If Applicable)

If there are no meaningful runtime actors/permissions (e.g., offline CLI / batch / ETL), write a single line `N/A`.

| actor_id | role | permissions |
|----------|------|-------------|
| ACTOR_PRIMARY | [role] | [allowed actions] |
| ACTOR_SUPPORT | [role] | [allowed actions] |

## Entities (Referenced)
| entity | identifier | notes |
|--------|------------|-------|
| [EntityName] | [id field / format] | [notes] |

ETL/migration variant (optional; adapt as needed):

| source_entity | target_entity | cardinality | notes |
|---------------|---------------|-------------|-------|
| [SourceEntity] | [TargetEntity] | [1:1 / 1:N / N:1 / N:N] | [notes] |

## Interface Contract

### Inputs
| field | type | required | constraints | example |
|------|------|----------|-------------|---------|
| [field] | [type] | Y/N | [constraints] | [example] |

### Outputs
| field | type | required | constraints | example |
|------|------|----------|-------------|---------|
| [field] | [type] | Y/N | [constraints] | [example] |

### Side effects
| effect_id | kind | target | guarantee | notes |
|----------|------|--------|-----------|-------|
| EFF-001 | db_write/external_call/event_publish/notification | [target] | [at-most-once/at-least-once/exactly-once] | [notes] |

### AuthZ (If Applicable)

If there is no runtime authorization model (e.g., offline CLI / batch / ETL), write a single line `N/A`.

| rule_id | actor_id | condition | decision |
|---------|----------|-----------|----------|
| AUTHZ-001 | ACTOR_PRIMARY | [predicate] | allow/deny |

### Idempotency (If Applicable)
| idempotency_key | scope | ttl | behavior_on_replay |
|-----------------|-------|-----|--------------------|
| [header/field] | [tenant/user/global] | [duration] | [same result / conflict / no-op] |

## Preconditions
| precondition_id | statement | validation |
|-----------------|-----------|------------|
| PRE-001 | [must already be true] | [how detected] |

## Workflow (Low-Variance Trace)

Choose the format that best fits the use case:
- Use the table format for linear request/response flows.
- Use numbered `### Step N: ...` subsections for complex multi-step pipelines.
- **Multi-operation UCs**: If this UC covers multiple operations on the same aggregate (e.g., Create/Update/Delete), create separate workflow subsections per operation and share the invariants/error catalog across them.
- **Read-only derivations**: Steps that compute a read-only projection as part of the response contract (e.g., deriving a user-visible status from internal state) are valid workflow steps.
- **Cross-UC behavior**: If a step describes behavior owned by another UC, add a cross-reference note (e.g., "See UC-XXXX") rather than duplicating the full trace.

### Workflow Format A: Table (recommended for linear flows)
| step | actor/system | action | reads | writes | emits | state_before | state_after |
|------|--------------|--------|-------|--------|-------|--------------|-------------|
| 1 | actor | [command] | [data] | - | - | S0 | S0 |
| 2 | system | [validate] | [data] | - | - | S0 | S0 |
| 3 | system | [apply change] | [data] | [data] | [event] | S0 | S1 |

### Workflow Format B: Numbered Steps (recommended for complex pipelines)

Each step MUST include: `actor/system`, `action`, `reads`, `writes`, `emits`, `state_before`, `state_after`.

#### Example shape

### Step 1: [Step name]
- actor/system: [actor or system component]
- action: [what happens]
- reads: [inputs/data sources]
- writes: [state writes]
- emits: [events/notifications]
- state_before: [state label]
- state_after: [state label]

## Verification / Post-Conditions (If Applicable)

Use this section when the system has explicit post-operation checks (integrity checks, reconciliation, health checks) whose pass/fail is part of the behavioral contract.

| check_id | check | description | pass_criteria |
|----------|-------|-------------|---------------|
| VER-001 | [check name] | [what it verifies] | [when it passes] |

## Alternatives And Edge Cases
| case_id | condition | behavior | error_code | retryable | user_message | state_impact |
|---------|----------|----------|------------|-----------|--------------|--------------|
| ALT-001 | [condition] | [behavior] | [ERR-...] | Y/N | [message] | [none/S0->S1] |

## Error Catalog (Typed)

Choose the table that matches the interface:
- Use the API/service table when there is a client/server contract with status codes.
- Use the CLI/batch table when the primary contract is exit codes and logs.

### Error Catalog Format A: API/Service
| error_code | condition | detection_point | retryable | client_action | status | message | telemetry_fields |
|------------|-----------|-----------------|-----------|---------------|--------|---------|------------------|
| ERR-001 | [condition] | [step] | Y/N | [action] | [e.g. 400] | [message] | [fields] |

### Error Catalog Format B: CLI/Batch
| error_code | condition | detection_point | retryable | client_action | exit_code | message |
|------------|-----------|-----------------|-----------|---------------|----------:|---------|
| ERR-001 | [condition] | [step] | Y/N | [action] | 1 | [message] |

## Invariants And Policies
| inv_id | invariant (MUST / MUST NOT) | enforcement | tests |
|--------|------------------------------|------------|-------|
| INV-001 | [statement] | [db constraint / code / review gate] | [VS-...] |

## Acceptance Tests (Executable Scenarios)

Choose the format that best fits the suite size:
- Use the detailed format when the number of scenarios is small enough to read end-to-end.
- Use the catalog format for large suites (for example 15+ scenarios), grouped by category.

### Acceptance Tests Format A: Detailed (Given / When / Then)
| scenario_id | given | when | then | validates | test_ref |
|-------------|-------|------|------|-----------|----------|
| VS-001 | [facts/state] | [action] | [observable outcomes] | [INV-001, ALT-001, VER-001, EFF-001] | [path::test_name] |

### Acceptance Tests Format B: Catalog (Large Suites)
| scenario_id | description | validates | test_ref |
|-------------|-------------|-----------|----------|
| VS-001 | [1 line scenario intent] | [INV-001, ALT-001, VER-001, EFF-001] | [path::test_name] |

## Deliberate Omissions (If Applicable)

Use this section for “we considered it, and explicitly chose not to do it” items that would otherwise be re-investigated later.

| item | rationale |
|------|-----------|
| [field/entity/mapping] | [why it was excluded] |

## Observability (If Applicable)

If this use case has no meaningful observability contract (e.g., offline CLI / batch / ETL), write a single line `N/A`.

| signal | required_fields | sampling | redaction |
|--------|-----------------|----------|-----------|
| logs | [fields] | [rate] | [policy] |
| metrics | [names] | n/a | n/a |
| traces | [span names] | [rate] | [policy] |

## Open Questions

Due dates should be `YYYY-MM-DD` when known. Use `tbd` for questions awaiting triage, or `deferred` with a rationale for questions intentionally postponed. Avoid leaving `due` blank — an explicit `tbd` signals intent to assign a date later.

| open_id | question | status | resolution | owner | due | impact |
|---------|----------|--------|------------|-------|-----|--------|
| OPEN-001 | [question] | open | - | [@owner] | YYYY-MM-DD / tbd / deferred | [blocked VS-/INV-/ALT- row] |
