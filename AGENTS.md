# AGENTS.md
<!-- agents-md-version: 1 -->
agents-md-version: 1

## CRITICAL

- MUST: Use `cargo` as the package manager for Rust dependency/build flows.
- MUST: Run `ard lint ...` before opening a PR.
- MUST: Use `ard` as the only supported CLI for linting/install flows.
- MUST: Run Rust test commands before PR: `cargo test -p ard`.
- MUST: Keep line coverage at or above 90%: `cargo llvm-cov -p ard --fail-under-lines 90`.
- MUST: Keep docs/examples using `ard` commands only (no Python/`uvx` fallback commands).
- NEVER: Force push (`git push --force`, `git push -f`) to shared branches.
- NEVER: Bypass hook checks with `--no-verify`.
- NEVER: Re-introduce Python runtime, Python package entrypoints, or `uv`-based CLI workflows.
- NEVER: Commit secrets, credentials, PII, or customer-identifying data.
- NEVER: Hand-edit generated artifacts (generated) unless the change is intentional and reviewed.
- PREFER: Built-in file/glob/grep/editor tools over shell one-liners when both are equivalent.
- ON FAIL: Read the full traceback/output before retrying.
- ON FAIL (lint): Run the failing `ard lint ...` target directly, fix the issue, then re-run the broader command.
- ON FAIL (test): Run `cargo test -p ard` first; if failing, run the reported test target and then re-run full tests.

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
cargo build -p ard
# lint:skill
cargo run -p ard -- lint skill ./skills/authoring-agents-md
# lint:agents
cargo run -p ard -- lint agents-md ./AGENTS.md --strict
# lint:docs
cargo run -p ard -- lint .
# test
cargo test -p ard
# coverage
cargo llvm-cov -p ard --fail-under-lines 90
```

## Structure

```text
docs/               # canonical guidance docs
examples/           # synthetic example specs
rubrics/            # grading criteria
skills/             # author/review workflows
crates/ard/         # Rust CLI implementation
templates/          # base spec templates
```

## Patterns

- **Module:** Rust modules/functions in `crates/ard/src/main.rs` plus integration tests in `crates/ard/tests/`.
- **Async:** synchronous CLI flow for new code; no async event-loop patterns in current implementation.
- **Naming:** `snake_case` functions, Rust enums/structs for domain types.
- Validator pattern: return structured findings/errors and non-zero exit code for failed checks.

## Search

- Semantic: `rg -n "rubric|agent-ready|variance|invariant" docs rubrics skills README.md` -- conceptual phrases.
- Exact: `rg -n "AG[0-9]{3}|ON FAIL|MUST:|NEVER:" crates skills README.md` -- rule/check references.
- Files: `rg --files crates skills templates rubrics docs` -- fast file inventory.

## Testing Strategy

- Runner: `cargo test -p ard`
- Coverage: `cargo llvm-cov -p ard --fail-under-lines 90`
- Separation: core/unit tests in `crates/ard/src/main.rs`; CLI integration tests in `crates/ard/tests/cli_integration.rs`.
- Conventions: assert explicit inputs/outputs and check IDs for lint findings.

## Security

- NEVER read/write: secret material, credentials, or customer-identifying content.
- NEVER log/commit: API keys, tokens, private keys, PII, confidential customer data.
- Secrets via: not used in this repository; content is public by policy.
- CI secrets: if needed, use GitHub Actions secrets (none configured in-repo).

## Env

- Rust: stable toolchain

```bash
# Local setup
cargo build -p ard
# Run CLI from repo
cargo run -p ard -- lint ./AGENTS.md
```

## Git

- Branch: feature/fix branches off `main`.
- Commit: concise imperative scope-first style (for example `docs: ...`, `rubrics: ...`).
- PR: describe what changed, why, and how to validate.
- Reviews: keep contributions aligned to `README.md` (Public Content Policy) and repository scope.
