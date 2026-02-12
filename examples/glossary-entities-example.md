---
id: GLOSSARY-EXAMPLE
type: glossary
title: "Glossary And Entities (Example)"
owner: "@example"
last_updated: 2026-02-12
---

# Glossary And Entities (Example)

## Terms
| term | definition (one line) | allowed_synonyms | banned_synonyms |
|------|------------------------|------------------|-----------------|
| Widget | A sellable item managed by the system. | - | Item, Product |
| Workspace | A tenant boundary for data and permissions. | Tenant | Account |

## Identifiers
| id_name | scope | format | example | notes |
|---------|-------|--------|---------|-------|
| workspace_id | tenant | UUID | 550e8400-e29b-41d4-a716-446655440000 | Immutable |
| widget_id | tenant | WID-[0-9]{6} | WID-000123 | Stable, unique per workspace |

## Entities
| entity | description (one line) | identifier | source_of_truth |
|--------|--------------------------|------------|-----------------|
| Widget | Represents a widget record. | widget_id | Widget Service DB |

## Entity Fields
| entity.field | type | required | constraints | classification | example |
|-------------|------|----------|-------------|----------------|---------|
| Widget.widget_id | string | Y | Matches WID-[0-9]{6} | internal | WID-000123 |
| Widget.workspace_id | uuid | Y | Valid UUID | internal | 550e8400-e29b-41d4-a716-446655440000 |
| Widget.name | string | Y | 1..120 chars | confidential | "Acme Widget" |
| Widget.status | enum | Y | {draft,active,archived} | internal | active |

## State (If Applicable)
| entity | state_field | allowed_states | transitions_source |
|--------|-------------|----------------|--------------------|
| Widget | status | draft,active,archived | UC-0001, ADR-0001 |

