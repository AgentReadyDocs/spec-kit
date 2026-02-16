from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))
from spec_kit_linters.agents_md_linter import evaluate_agents_md


VALID_AGENTS_MD = """agents-md-version: 1.0

# AGENTS.md

## CRITICAL
- MUST: use one package manager command.
- MUST: run lint before commit.
- MUST: run test before commit.
- NEVER: force push to protected branches.
- NEVER: bypass git hook checks.
- NEVER: access or commit secrets.
- NEVER: edit generated files directly.
- ON FAIL (lint): run lint --fix and rerun lint.
- ON FAIL (test): rerun failing test with verbose output.

## Commands
- install: uv sync
- lint: uv run ruff check .
- test: uv run pytest
"""


def test_agents_md_linter_passes_minimal_valid_file(tmp_path: Path) -> None:
    path = tmp_path / "AGENTS.md"
    path.write_text(VALID_AGENTS_MD, encoding="utf-8")

    errors, warnings = evaluate_agents_md(path, strict=True)
    assert errors == []
    assert warnings == []


def test_agents_md_linter_fails_without_critical(tmp_path: Path) -> None:
    path = tmp_path / "AGENTS.md"
    path.write_text("# AGENTS.md\n\n## Commands\n- install: uv sync\n", encoding="utf-8")

    errors, _warnings = evaluate_agents_md(path, strict=False)
    assert any(finding.check_id == "AG002" for finding in errors)
