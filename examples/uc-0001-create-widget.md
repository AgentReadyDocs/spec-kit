---
id: UC-0001
type: use_case
title: "Create Widget"
status: ready
owner: "@example"
risk_tier: tier1
system: "Widget Service"
links:
  glossary: "./glossary-entities-example.md"
  nfr: "./nfr-0001-widget-service.md"
code_refs:
  implementation: ""
  tests: ""
---

# UC-0001: Create Widget

## Goal
| goal |
|------|
| Create a new Widget in a Workspace. |

## Scope
| in_scope | out_of_scope |
|----------|--------------|
| - Create Widget record | - Update Widget |
| - Validate input | - Delete Widget |

## Actors
| actor_id | role | permissions |
|----------|------|-------------|
| workspace_user | Workspace user | widget:create |

## Entities (Referenced)
| entity | identifier | notes |
|--------|------------|-------|
| Widget | widget_id | Scoped to workspace_id |

## Interface Contract

### Inputs
| field | type | required | constraints | example |
|------|------|----------|-------------|---------|
| workspace_id | uuid | Y | Must exist | 550e8400-e29b-41d4-a716-446655440000 |
| name | string | Y | 1..120 chars | "Acme Widget" |

### Outputs
| field | type | required | constraints | example |
|------|------|----------|-------------|---------|
| widget_id | string | Y | Matches WID-[0-9]{6} | WID-000123 |
| status | enum | Y | {draft,active,archived} | draft |

### Side effects
| effect_id | kind | target | guarantee | notes |
|----------|------|--------|-----------|-------|
| EFF-001 | db_write | widgets table | at-least-once | Duplicate prevented by idempotency key |

### AuthZ
| rule_id | actor_id | condition | decision |
|---------|----------|-----------|----------|
| AUTHZ-001 | workspace_user | actor has widget:create for workspace_id | allow |

### Idempotency (If Applicable)
| idempotency_key | scope | ttl | behavior_on_replay |
|-----------------|-------|-----|--------------------|
| Idempotency-Key header | workspace | 24h | same result |

## Preconditions
| precondition_id | statement | validation |
|-----------------|-----------|------------|
| PRE-001 | Workspace exists | lookup by workspace_id |

## Workflow (Deterministic Trace)
| step | actor/system | action | reads | writes | emits | state_before | state_after |
|------|--------------|--------|-------|--------|-------|--------------|-------------|
| 1 | actor | POST /widgets {workspace_id,name} | - | - | - | S0 | S0 |
| 2 | system | validate inputs | workspace, constraints | - | - | S0 | S0 |
| 3 | system | create Widget | - | Widget row | - | S0 | S1 |
| 4 | system | return output | Widget row | - | - | S1 | S1 |

## Alternatives And Edge Cases
| case_id | condition | behavior | error_code | retryable | user_message | state_impact |
|---------|----------|----------|------------|-----------|--------------|--------------|
| ALT-001 | name violates constraints | reject | ERR-VALIDATION | N | "Invalid name" | none |
| ALT-002 | workspace_id not found | reject | ERR-NOT-FOUND | N | "Workspace not found" | none |

## Error Catalog (Typed)
| error_code | condition | detection_point | retryable | client_action | status | message | telemetry_fields |
|------------|-----------|-----------------|-----------|---------------|--------|---------|------------------|
| ERR-VALIDATION | invalid input | step 2 | N | fix input | 400 | Invalid input | workspace_id |
| ERR-NOT-FOUND | workspace missing | step 2 | N | choose valid workspace | 404 | Not found | workspace_id |

## Invariants And Policies
| inv_id | invariant (MUST / MUST NOT) | enforcement | tests |
|--------|------------------------------|------------|-------|
| INV-001 | Widget MUST have workspace_id | code + db schema | VS-001 |
| INV-002 | Widget.name MUST be 1..120 chars | code | VS-002 |

## Acceptance Tests (Executable Scenarios)
| scenario_id | given | when | then | validates | test_ref |
|-------------|-------|------|------|-----------|----------|
| VS-001 | Workspace exists | create Widget | Widget persisted with workspace_id | AC1 | N/A |
| VS-002 | Workspace exists | create Widget with invalid name | ERR-VALIDATION returned | AC2 | N/A |

## Observability
| signal | required_fields | sampling | redaction |
|--------|-----------------|----------|-----------|
| logs | uc_id, workspace_id, error_code? | 100% errors | redact name |
| metrics | widgets_create_success_total, widgets_create_error_total | n/a | n/a |
| traces | POST /widgets | 10% normal, 100% errors | redact name |

## Open Questions
| open_id | question | owner | due | impact |
|---------|----------|-------|-----|--------|
