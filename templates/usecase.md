---
id: UC-XXXX
type: use_case
title: "[Short title]"
status: draft  # draft | ready
owner: "@owner"
risk_tier: tier1  # tier0 | tier1 | tier2 | tier3
system: "[System name]"
links:
  glossary: "../templates/glossary-entities.md"
  nfr: "../templates/nfr.md"
code_refs:
  implementation: ""
  tests: ""
---

# UC-XXXX: [Short title]

Risk tiers are defined in `templates/nfr.md` under "Risk Tiers (Doc Gates)".

## Goal
| goal |
|------|
| [One sentence outcome] |

## Scope
| in_scope | out_of_scope |
|----------|--------------|
| [bullet list] | [bullet list] |

## Actors
| actor_id | role | permissions |
|----------|------|-------------|
| ACTOR_PRIMARY | [role] | [allowed actions] |
| ACTOR_SUPPORT | [role] | [allowed actions] |

## Entities (Referenced)
| entity | identifier | notes |
|--------|------------|-------|
| [EntityName] | [id field / format] | [notes] |

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

### AuthZ
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

## Workflow (Deterministic Trace)
| step | actor/system | action | reads | writes | emits | state_before | state_after |
|------|--------------|--------|-------|--------|-------|--------------|-------------|
| 1 | actor | [command] | [data] | - | - | S0 | S0 |
| 2 | system | [validate] | [data] | - | - | S0 | S0 |
| 3 | system | [apply change] | [data] | [data] | [event] | S0 | S1 |

## Alternatives And Edge Cases
| case_id | condition | behavior | error_code | retryable | user_message | state_impact |
|---------|----------|----------|------------|-----------|--------------|--------------|
| ALT-001 | [condition] | [behavior] | [ERR-...] | Y/N | [message] | [none/S0->S1] |

## Error Catalog (Typed)
| error_code | condition | detection_point | retryable | client_action | status | message | telemetry_fields |
|------------|-----------|-----------------|-----------|---------------|--------|---------|------------------|
| ERR-001 | [condition] | [step] | Y/N | [action] | [e.g. 400] | [message] | [fields] |

## Invariants And Policies
| inv_id | invariant (MUST / MUST NOT) | enforcement | tests |
|--------|------------------------------|------------|-------|
| INV-001 | [statement] | [db constraint / code / review gate] | [VS-...] |

## Acceptance Tests (Executable Scenarios)
| scenario_id | given | when | then | validates | test_ref |
|-------------|-------|------|------|-----------|----------|
| VS-001 | [facts/state] | [action] | [observable outcomes] | [AC1, AC2] | [path::test_name] |

## Observability
| signal | required_fields | sampling | redaction |
|--------|-----------------|----------|-----------|
| logs | [fields] | [rate] | [policy] |
| metrics | [names] | n/a | n/a |
| traces | [span names] | [rate] | [policy] |

## Open Questions
| open_id | question | owner | due | impact |
|---------|----------|-------|-----|--------|
| OPEN-001 | [question] | [@owner] | YYYY-MM-DD | [blocked AC / blocked step] |
