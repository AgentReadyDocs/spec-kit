---
name: authoring-glossary-entities
description: >
  Co-authors or updates a spec-kit Glossary + Entities document from templates/glossary-entities.md and
  self-checks it against rubrics/glossary-entities-rubric.md. Use when: defining controlled terms, identifiers,
  entities, and entity fields (including classification) for a spec/docset.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (no dependencies)"
license: "Apache-2.0"
---

# Author Glossary + Entities

## Objective

Create or update a single glossary/entities doc that defines:
- controlled terms (with allowed/banned synonyms as needed)
- identifiers (scope + format + example)
- entities (description, identifier, source of truth)
- entity fields (type, required, constraints, classification, example)
- optional state tables when state exists in the domain

## Outputs

- A complete Markdown Glossary + Entities doc (paste-ready), matching `templates/glossary-entities.md`.
- A short self-check summary for GE-CF-001..004 (PASS/FAIL with evidence pointers).

## When to use

Use this skill when the user asks to:
- draft a glossary/entities doc for a spec-kit docset
- add missing entities/fields referenced by use cases
- reconcile conflicting types/constraints for the same entity.field
- add data classifications required for tier2+

## When NOT to use

- Do not use for rubric-only review/grade (use `$reviewing-glossary-entities`).

## Safety / constraints (non-negotiable)

- Do not browse the web or call external systems unless the user explicitly requests it.
- Never read, request, or paste secrets (tokens, credentials, private keys, `.env` contents).
- Do not invent domain facts; if unknown, ask and track as open questions until resolved.
- Only write files after the user approves the target path and drafted content.

## Workflow (decision-complete)

1. Resolve the target file
   - If updating: read existing doc first and preserve `id`.
   - If creating: start from `templates/glossary-entities.md`.
2. Determine tier (required)
   - Ask for the project/system tier (tier0..tier3). If unknown, record as an open question; include classifications anyway to be safe.
3. Populate Terms
   - Ensure the Terms table is non-empty and each term has a one-line definition.
4. Populate Identifiers
   - Every identifier row has scope, format, and a concrete example.
5. Populate Entities and Entity Fields
   - Every entity.field row has type, required, constraints, classification, and a concrete example.
6. Add State table when applicable
   - If state exists, list allowed states and transition source references.
   - If no state exists, explicitly write “State: N/A” (do not omit silently).
7. Self-check gate (hard)
   - Evaluate GE-CF-001..004 as PASS/FAIL with evidence.
8. Critic loop (required)
   - Run `$reviewing-glossary-entities` and apply P1/P2 findings, then re-check Critical Fails.

## Glossary rubric gate (must apply)

Critical Fails from `rubrics/glossary-entities-rubric.md`:
- GE-CF-001 Terms table exists and is non-empty
- GE-CF-002 Entity fields table includes types and concrete examples
- GE-CF-003 Identifiers include scope + format + example
- GE-CF-004 (tier2+) Every entity field has a classification value
