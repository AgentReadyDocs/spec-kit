---
id: GLOSSARY-ENTITIES
type: glossary
title: "Glossary And Entities"
owner: "@owner"
last_updated: YYYY-MM-DD
---

# Glossary And Entities

## Terms
| term | definition (one line) | allowed_synonyms | banned_synonyms |
|------|------------------------|------------------|-----------------|
| [Term] | [definition] | [optional] | [optional] |

## Identifiers
| id_name | scope | format | example | notes |
|---------|-------|--------|---------|-------|
| [id] | global/tenant/user | [format] | [example] | [notes] |

## Entities
| entity | description (one line) | identifier | source_of_truth |
|--------|--------------------------|------------|-----------------|
| [Entity] | [description] | [id field/format] | [system] |

## Entity Fields
| entity.field | type | required | constraints | classification | example |
|-------------|------|----------|-------------|----------------|---------|
| [Entity.field] | [type] | Y/N | [constraints] | public/internal/confidential/restricted | [example] |

## State (If Applicable)
| entity | state_field | allowed_states | transitions_source |
|--------|-------------|----------------|--------------------|
| [Entity] | [field] | [S0,S1,S2] | [UC references / ADR] |

