# skills

Co-authoring and review skills for creating agent-ready documentation using `spec-kit` templates and rubrics.

## Purpose

`skills/` contains reusable workflows and prompts that help teams:

- Elicit requirements and resolve ambiguity
- Draft specs that satisfy `spec-kit` templates
- Review specs against `rubrics/*` (including required `rubrics/rubric-guidance.md`)

## Design Rules

- Reduce variance in behavioral outcomes and keep reviews rubric-bound.
- Prefer concrete findings with patch guidance over generic feedback.
- Separate author and critic responsibilities to reduce blind spots.
- Version skill contracts when expected inputs/outputs change.

## Typical Layout

When present, content is organized under:

- `claude-code/`
- `codex/`
- `shared/`
- `examples/`
- `docs/`

## Compatibility

Each skill should declare the `spec-kit` version(s) and the exact files it expects to load (templates, rubrics, and any cross-doc links).
