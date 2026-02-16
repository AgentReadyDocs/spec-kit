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

## Linters

- Packaged implementation: `src/spec_kit_linters/`
- CLI commands: `spec-kit-skill-lint`, `spec-kit-agents-md-lint`
- Compatibility wrappers: `skills/linters/validate_skill.py`, `skills/linters/validate_agents_md.py`
- Agents rubric reference: `skills/linters/agents-md-linter-rubric.md`

## Skill Structure

Each skill directory follows:

- `SKILL.md`
- `agents/openai.yaml`
- `references/` (rubrics, examples, schema notes)
- `scripts/` (optional helper automation)

## Compatibility

Each skill should declare the `spec-kit` version(s) and the exact files it expects to load (templates, rubrics, and any cross-doc links).
