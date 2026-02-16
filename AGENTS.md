# AGENTS.md
<!-- agents-md-version: 1 -->
agents-md-version: 1

## CRITICAL

- MUST: Use `uv` as the Python package manager and environment manager (`uv sync`, `uv run ...`).
- MUST: Run skill lint checks before commit when changing `skills/`: `uv run python skills/linters/validate_skill.py <skill-dir>`.
- MUST: Run test commands before PR: `uv run pytest`.
- MUST: Use `[project.scripts]` in `pyproject.toml` to change CLI entry points; do not hardcode wrapper paths in docs.
- NEVER: Force push (`git push --force`, `git push -f`) to shared branches.
- NEVER: Bypass hook checks with `--no-verify`.
- NEVER: Use `pip install`, `poetry install`, or `python -m venv` for project workflows; use `uv` only.
- NEVER: Commit secrets, credentials, PII, or customer-identifying data.
- NEVER: Edit generated caches in `.pytest_cache/` or `__pycache__/`.
- PREFER: Built-in file/glob/grep/editor tools over shell one-liners when both are equivalent.
- ON FAIL: Read the full traceback/output before retrying. Confirm `uv sync` completed successfully.
- ON FAIL (lint): Re-run the failing validator with a narrower target, fix reported path/format issues, then re-run the full validator command.
- ON FAIL (test): Run the failing file first; if unknown, run `uv run pytest tests/test_agents_md_linter.py -q` then `uv run pytest tests/test_skill_linter.py -q`, then re-run `uv run pytest`.

## Domain & Context

- Goal: Provide canonical templates, rubrics, and co-authoring/review skills for agent-ready specification work.
- Type: Library
- License: Apache-2.0
- Key Terms:
  - `spec-kit`: canonical source for specification artifacts that reduce ambiguity before implementation.
  - `agent-ready`: docs with explicit invariants, interfaces, state transitions, and executable-style acceptance criteria.
  - `reduced variance`: constrain observable behavior so independent implementations converge.

## Execution Context

- Run on: Host
- Prefix: N/A

## Commands

```bash
# install
uv sync                                              # ON FAIL: re-run from repo root; if env is corrupted, verify `pwd` is repo root before recreating `.venv`
# lint:skill
uv run python skills/linters/validate_skill.py ./skills/authoring-agents-md  # ON FAIL: fix the reported SKILL.md/link/frontmatter error and re-run
# lint:agents
uv run python skills/linters/validate_agents_md.py ./AGENTS.md --strict       # ON FAIL: address listed AG* findings, then re-run this command
# test
uv run pytest                                        # ON FAIL: run failing file first; if unknown run `uv run pytest tests/test_agents_md_linter.py -q` then `uv run pytest tests/test_skill_linter.py -q`
# lint:packaged-cli (optional sanity check)
uv run spec-kit-skill-lint ./skills/authoring-agents-md                        # ON FAIL: ensure `uv sync` succeeded and path points to a skill dir with SKILL.md
```

## Structure

```text
docs/               # canonical guidance docs
examples/           # synthetic example specs
rubrics/            # grading criteria
skills/             # author/review workflows
src/spec_kit_linters/  # packaged CLI linter code
templates/          # base spec templates
tests/              # pytest test suite
.pytest_cache/      # pytest cache (generated -- do not edit)
*/__pycache__/      # Python bytecode cache (generated -- do not edit)
```

## Patterns

- **Module:** Python modules with explicit imports (`from ... import ...`), package code under `src/spec_kit_linters`.
- **Async:** synchronous CLI flow for new code; no async event-loop patterns in current implementation.
- **Naming:** `snake_case.py` files, `snake_case` functions, `PascalCase` dataclasses.
- Validator pattern: return structured findings/errors and non-zero exit code for failed checks.

## Search

- Semantic: `rg -n "rubric|agent-ready|variance|invariant" docs rubrics skills README.md` -- conceptual phrases.
- Exact: `rg -n "AG[0-9]{3}|ON FAIL|MUST:|NEVER:" src tests skills` -- rule/check references.
- Files: `rg --files src tests skills templates rubrics` -- fast file inventory.

## Testing Strategy

- Runner: `pytest`
- Fixtures: `tmp_path` filesystem fixtures in `tests/`.
- Separation: tests mirror linter modules (`test_skill_linter.py`, `test_agents_md_linter.py`).
- Coverage: No threshold configured.
- Conventions: assert return codes/findings from linter entry points rather than shelling out.

## Security

- NEVER read/write: secret material, credentials, or customer-identifying content.
- NEVER log/commit: API keys, tokens, private keys, PII, confidential customer data.
- Secrets via: not used in this repository; content is public by policy.
- CI secrets: if needed, use GitHub Actions secrets (none configured in-repo).

## Env

- Python: `>=3.10` (`pyproject.toml`)
- Build backend: `hatchling`

```bash
# Local setup
uv sync
# Run package CLI from repo
uv run spec-kit-agents-md-lint ./AGENTS.md
```

## Git

- Branch: feature/fix branches off `main`.
- Commit: concise imperative scope-first style (for example `docs: ...`, `rubrics: ...`).
- PR: describe what changed, why, and how to validate.
- Reviews: keep contributions aligned to `README.md` (Public Content Policy) and repository scope.
