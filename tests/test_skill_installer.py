from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))
from spec_kit_linters.skill_installer import InstallTarget, install_skills


def test_skill_installer_copies_selected_skill(tmp_path: Path) -> None:
    spec_kit_root = tmp_path / "spec-kit"
    (spec_kit_root / "skills").mkdir(parents=True)
    (spec_kit_root / "templates").mkdir()
    (spec_kit_root / "rubrics").mkdir()

    skill_dir = spec_kit_root / "skills" / "authoring-agents-md"
    (skill_dir / "references").mkdir(parents=True)
    (skill_dir / "SKILL.md").write_text(
        "---\nname: authoring-agents-md\ndescription: test\n---\n",
        encoding="utf-8",
    )
    (skill_dir / "references" / "ref.md").write_text("# ref\n", encoding="utf-8")

    codex_home = tmp_path / "codex-home"
    target = InstallTarget(name="codex", home=codex_home)

    rc = install_skills(
        spec_kit_root=spec_kit_root,
        selected_skills=["authoring-agents-md"],
        targets=[target],
        namespace=None,
        overwrite=False,
        dry_run=False,
    )
    assert rc == 0
    assert (codex_home / "skills" / "authoring-agents-md" / "SKILL.md").is_file()
    assert (
        codex_home
        / "skills"
        / "authoring-agents-md"
        / "references"
        / "ref.md"
    ).is_file()


def test_skill_installer_refuses_overwrite_without_flag(tmp_path: Path) -> None:
    spec_kit_root = tmp_path / "spec-kit"
    (spec_kit_root / "skills").mkdir(parents=True)
    (spec_kit_root / "templates").mkdir()
    (spec_kit_root / "rubrics").mkdir()

    skill_dir = spec_kit_root / "skills" / "authoring-agents-md"
    skill_dir.mkdir(parents=True)
    (skill_dir / "SKILL.md").write_text(
        "---\nname: authoring-agents-md\ndescription: test\n---\n",
        encoding="utf-8",
    )

    codex_home = tmp_path / "codex-home"
    (codex_home / "skills" / "authoring-agents-md").mkdir(parents=True)
    (codex_home / "skills" / "authoring-agents-md" / "SKILL.md").write_text(
        "existing\n", encoding="utf-8"
    )

    target = InstallTarget(name="codex", home=codex_home)

    rc = install_skills(
        spec_kit_root=spec_kit_root,
        selected_skills=["authoring-agents-md"],
        targets=[target],
        namespace=None,
        overwrite=False,
        dry_run=False,
    )
    assert rc == 1

