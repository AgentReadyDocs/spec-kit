from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))
from spec_kit_linters.skill_linter import run_skill_lint


def test_skill_linter_passes_valid_skill(tmp_path: Path) -> None:
    skill_dir = tmp_path / "example-skill"
    skill_dir.mkdir()
    (skill_dir / "SKILL.md").write_text(
        "---\n"
        "name: example-skill\n"
        "description: Example skill\n"
        "---\n\n"
        "# Example\n\n"
        "[Ref](references/ref.md)\n",
        encoding="utf-8",
    )
    references_dir = skill_dir / "references"
    references_dir.mkdir()
    (references_dir / "ref.md").write_text("# Ref\n", encoding="utf-8")

    rc = run_skill_lint(skill_dir, max_lines=500)
    assert rc == 0


def test_skill_linter_fails_missing_skill_md(tmp_path: Path) -> None:
    skill_dir = tmp_path / "missing-skill-md"
    skill_dir.mkdir()

    rc = run_skill_lint(skill_dir, max_lines=500)
    assert rc == 1
