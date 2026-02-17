# Adopting `spec-kit` in a TypeScript or Go repo

This repo is language-agnostic: you can use the templates/rubrics/skills in any project.

Recommended adoption path: install the single-binary CLI `ard` (no Python required).

## Goals

- Keep your application repo TS/Go-first (no Python runtime requirement at build time).
- Still lint `AGENTS.md` and spec docs in CI with pinned versions.
- Make skills discoverable to Codex CLI / Claude after a one-time install + restart.

## Quickstart (CI-friendly linting)

### Option A: `ard` (recommended)

Install `ard` (see repo `README.md`), then:

```bash
ard lint ./AGENTS.md
ard lint .
```

## Installing skills for Codex CLI / Claude

Skills are directories under `skills/` (each with `SKILL.md`). To make them available to your agent tool, you need to copy them into its local skills directory.

### Install skills with `ard` (recommended)

```bash
ard skill install --target codex --target claude --namespace spec-kit --overwrite
```

Notes:
- `--namespace spec-kit` avoids collisions with same-named skills you may already have.
- Some tools may require a restart to re-scan skill directories.

### Option B: Copy manually

- Codex CLI: copy into `$CODEX_HOME/skills/` (default `~/.codex/skills/`)
- Claude: copy into `$CLAUDE_HOME/skills/` (default `~/.claude/skills/`)

## What to copy into your repo

Recommended: keep only your **actual** specs (use cases, ADRs, NFR baselines, glossary/entities, etc.) in the repo.

If you need canonical templates/rubrics, retrieve them on demand:

```bash
ard template list
ard template print usecase.md > /tmp/usecase.md
ard rubric list
ard rubric print usecase-rubric.md > /tmp/usecase-rubric.md
```

## Minimal `AGENTS.md` patterns (TS / Go)

These are illustrative command shapes for new repos; the `authoring-agents-md` skill can generate a complete file.

Starter templates are also provided:

- `templates/agents-md-typescript.md`
- `templates/agents-md-go.md`

TypeScript (pnpm example):

```bash
# install
pnpm install
# lint
pnpm lint
# test
pnpm test
```

Go:

```bash
# install
go mod download
# lint
golangci-lint run
# test
go test ./...
```
