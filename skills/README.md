# skills

Co-authoring and review skills for creating agent-ready documentation using `spec-kit` templates and rubrics.

## Purpose

`skills/` contains reusable workflows and prompts that help teams:

- Elicit requirements and resolve ambiguity
- Draft specs that satisfy `spec-kit` templates
- Review specs against `rubrics/*` (including required `rubrics/rubric-guidance.md`)
- Author and review `AGENTS.md` for repository-level agent instructions

## Design Rules

- Reduce variance in behavioral outcomes and keep reviews rubric-bound.
- Prefer concrete findings with patch guidance over generic feedback.
- Separate author and critic responsibilities to reduce blind spots.
- Version skill contracts when expected inputs/outputs change.

## Included Skills

- `authoring-agents-md/`: generate or update `AGENTS.md` and agent import files.
- `reviewing-agents-md/`: review and score `AGENTS.md` with evidence-backed findings.
- `authoring-usecase/`: co-author use cases from template + rubric gate.
- `reviewing-usecase/`: review/score use cases (read-only).
- `authoring-adr/`: co-author ADRs from template + rubric gate.
- `reviewing-adr/`: review/score ADRs (read-only).
- `authoring-nfr/`: co-author NFR baselines from template + rubric gate.
- `reviewing-nfr/`: review/score NFR baselines (read-only).
- `authoring-glossary-entities/`: co-author glossary/entities from template + rubric gate.
- `reviewing-glossary-entities/`: review/score glossary/entities (read-only).

## Linters

- Canonical CLI usage and install notes: `README.md` (see “Linters (CLI)”).
- Compatibility wrappers used by skills: `skills/linters/`.
- Agents.md rubric reference: `skills/linters/agents-md-linter-rubric.md`.

## Skill Structure

Each skill directory follows:

- `SKILL.md`
- `agents/openai.yaml`
- `references/` (rubrics, examples, schema notes)
- `scripts/` (optional helper automation)

## Compatibility

Each skill should declare the `spec-kit` version(s) and the exact files it expects to load (templates, rubrics, and any cross-doc links) in its `SKILL.md`.
