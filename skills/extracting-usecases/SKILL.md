---
name: extracting-usecases
description: >
  Extracts existing use cases from source code by discovering runnable entrypoints across a
  monorepo or a parent folder containing multiple repos, then drafts evidence-backed Use Case (UC)
  stubs (actor + trigger + observable outputs) for each entrypoint. Uses an author→critic loop and
  produces coverage/completeness reports. Use when: you want a fast, repo-agnostic map of “what can run”
  (APIs, webhooks, workers, CLIs, schedules, file transfers) and draft UC stubs for each.
compatibility: "Codex CLI, Claude Code/Desktop, OpenCode (no dependencies)"
license: "Apache-2.0"
---

# Extract Use Cases From Source Code (Entrypoints → UC Stubs)

## Objective

Given a workspace folder whose direct children are separate git repos, produce:

- a per-repo **Entrypoint Catalog** (what can be triggered/run)
- a per-repo set of **UC stubs** (draft OK) with at least:
  - **actor**
  - **trigger**
  - **observable outputs**
- a workspace-level **coverage/completeness report**

This skill is intentionally evidence-bound: every non-trivial claim must cite a local source
(`path:line` + ≤25-word quote). When evidence is missing, record `UNKNOWN` and surface it in the report.

## When to use

Use this skill when the user asks for:

- multirepo discovery of “main actors” and “use cases”
- “what are the entrypoints in this codebase/workspace?”
- draft UC stubs for APIs, webhooks, CLIs, migrations, backfills, workers, scheduled jobs, and file transfers
- an evidence-backed system map suitable for converting into full `spec-kit` docs later

## When NOT to use

- Do not use for authoring implementation-grade specs for a single use case (use `$authoring-usecase`).
- Do not use for rubric grading (use reviewing skills).
- Do not use when the workspace includes secrets/PII you cannot safely read; stop and ask for a redacted clone.

## Safety / constraints (non-negotiable)

- Do not browse the web or call external systems unless the user explicitly requests it.
- Never request, read, or paste secrets (tokens, credentials, private keys, `.env` contents).
- Do not “fill in” missing behavior. If you can’t cite evidence, mark it `UNKNOWN`.
- Avoid inspecting generated/large vendor directories (`node_modules/`, `dist/`, `build/`, `target/`, `.venv/`) unless explicitly required.

## Inputs (ask if missing)

Minimum:
- `workspace_root`: path to the parent folder containing multiple repos

Recommended:
- `workspace_layout`:
  - `auto` (recommended): infer `monorepo` vs `multirepo` from local `.git` markers
  - `multirepo`: `workspace_root/` contains multiple child git repos (direct children)
  - `monorepo`: `workspace_root/` itself is a single git repo containing many packages/services
- `output_mode`:
  - `central` (recommended): write outputs under `<workspace_root>/workspace-discovery/`
  - `in_repo`: write outputs inside each repo under `<repo>/discovery/`
- `repo_include` / `repo_exclude`: optional name/path filters (globs or substrings)
- `max_repo_depth`: default `2` (discover child repos only; avoid deep nesting)
- `entrypoint_budget`: max files to open per repo before asking to proceed (default `~40`)

## Definitions

### Entrypoint

An “entrypoint” is anything that can be triggered without importing it as a library:

- **HTTP**: OpenAPI, route declarations, webhook handlers, gateway configs
- **Async**: queue/topic consumers, event handlers, background workers
- **Schedule**: cron/scheduled jobs, workflow schedulers
- **File transfer / batch**: SFTP upload/download, import/export, ETL jobs
- **CLI / migration**: binaries, scripts, “bin/” tools, `cmd/*`, migration/backfill tools, Docker entrypoints

### Coverage vs completeness

- **Coverage**: every discovered entrypoint has at least one UC stub.
- **Completeness**: UC stubs have non-`UNKNOWN` values for actor + trigger + outputs.

The hard requirement is **coverage = 100%**. Completeness should be maximized, but residual `UNKNOWN`s are allowed and must be reported.

## Outputs

Produce the following artifacts.

Per repo (under `<out>/<repo>/`):
- `entrypoints.json`: list of entrypoints with stable IDs and evidence
- `ucs/`: UC stub Markdown files (one or more per entrypoint)
- `coverage.md`: coverage/completeness table + unresolved questions

Workspace-level (under `<out>/workspace/`):
- `repos.json`: discovered repos + commit SHAs (if available) + scan timestamps (YYYY-MM-DD)
- `actors.md`: deduped actor list (external systems + human roles) with evidence
- `workspace-coverage.md`: summary table across repos

Where:
- `<out>` is `<workspace_root>/workspace-discovery/` for `central` mode, else each repo root for `in_repo` mode.

## Workflow (author → critic; required)

### 0) Discover repos

This skill supports both monorepo and multirepo workspaces.

In `workspace_layout: auto`, infer layout by checking:
- if `workspace_root` has `.git/` or a `.git` file: treat `workspace_root` as a repo (monorepo case)
- else: treat direct children of `workspace_root` as repos (multirepo case)

In `workspace_layout: multirepo`:
- treat direct child folders of `workspace_root` as repos if they contain:
  - a `.git/` directory, or
  - a `.git` file (worktree/submodule)

In `workspace_layout: monorepo`:
- treat `workspace_root` itself as the only repo by default
- do not treat nested `.git` markers as separate repos unless the user explicitly asks to include submodules/worktrees

Rules:
- do not recurse into a repo to find nested repos unless the user explicitly asks (avoid monorepo false positives)
- capture `repo_name`, `repo_root`, and if possible `git rev-parse HEAD` output (if accessible)

### 1) Author pass: build the Entrypoint Catalog (evidence-bound)

For each repo:

1. Start with “manifest entrypoints”:
   - `package.json` (`bin`, `scripts`, workspaces), `Dockerfile` (`ENTRYPOINT`/`CMD`), `Makefile` targets
   - language-native CLI conventions (`cmd/*`, `src/main.*`, `__main__.py`, etc.)
2. Expand to “service entrypoints”:
   - OpenAPI/Swagger files
   - webhook routes (`/webhook`, `/callback`, `integrations/*`, etc.)
   - workers/consumers (queue/topic names, handler modules)
   - scheduled jobs (cron configs, workflow schedulers)
   - file transfer jobs (SFTP, CSV export/import)
3. For each entrypoint, record:
   - `entrypoint_id` (stable, derived from repo + kind + primary ref)
   - `kind`: `http|webhook|cli|worker|schedule|file_transfer|migration|other`
   - `primary_ref`: method+path, command name, handler module, queue/topic name, schedule ID, etc.
   - `actors_candidates`: list of actors hinted by evidence (e.g., “Twilio”, “Vonage”, “Payroll system”)
   - `evidence[]`: `path`, `line`, `quote` (≤25 words), and a short `why`

If you hit the `entrypoint_budget`, pause and ask the user whether to continue deeper.

### 2) Author pass: draft UC stubs per entrypoint

For each entrypoint, generate one or more UC stubs (draft OK). If the entrypoint clearly supports multiple actors/use cases (e.g., one webhook router with multiple providers), create multiple stubs and link them back to the same `entrypoint_id`.

Stub requirements:
- include **actor + trigger + outputs** fields (use `UNKNOWN` if evidence is missing)
- keep stubs short; prefer tables; defer details to “Open Questions”
- include an explicit **Evidence** section listing the exact `path:line` citations used

### 3) Critic pass: evidence and completeness gate

For each repo:

- Reject or downgrade any claim not supported by evidence.
- Ensure **every discovered entrypoint has at least one UC stub** (coverage = 100%).
- Compute completeness metrics:
  - `% stubs with actor != UNKNOWN`
  - `% stubs with trigger != UNKNOWN`
  - `% stubs with outputs != UNKNOWN`
- Produce `coverage.md` with:
  - entrypoint list
  - mapping to UC stub files
  - `UNKNOWN` fields summary
  - a prioritized “fill these next” list (max 20)

### 4) Workspace merge (actors and cross-repo links)

After all repos are processed:

- dedupe actor names across repos (aliasing “Nexmo” vs “Vonage”, etc.) without inventing facts
- build a workspace-level `actors.md` and `workspace-coverage.md`
- if you find cross-repo interactions (shared queues, webhook-to-worker patterns), record them as hypotheses with evidence

## UC stub format (recommended)

Use a lightweight Markdown UC stub; it does not need to pass `rubrics/usecase-rubric.md` yet.

Required sections:
- `## Actor`
- `## Trigger`
- `## Observable Outputs`
- `## Evidence`
- `## Open Questions`

Recommended:
- Add `status: draft` and `entrypoint_id:` in frontmatter.

## Stop condition

You may stop only when:

- Every discovered repo has an `entrypoints.json`.
- Coverage is 100% in every repo: every entrypoint has ≥1 UC stub.
- `workspace-coverage.md` exists and lists all repos scanned.

If completeness < 100%, stop only if the remaining unknowns are explicitly listed and prioritized in each repo’s `coverage.md`.
