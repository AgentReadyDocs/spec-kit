from __future__ import annotations

import argparse
import os
import shutil
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class InstallTarget:
    name: str
    home: Path

    @property
    def skills_root(self) -> Path:
        return self.home / "skills"


def fail(message: str) -> int:
    print(f"[FAIL] {message}", file=sys.stderr)
    return 1


def _expand_home(path: str) -> Path:
    return Path(os.path.expandvars(os.path.expanduser(path))).resolve()


def find_spec_kit_root(start: Path) -> Path | None:
    start = start.resolve()
    for candidate in (start, *start.parents):
        if (
            (candidate / "skills").is_dir()
            and (candidate / "templates").is_dir()
            and (candidate / "rubrics").is_dir()
        ):
            return candidate
    return None


def list_skills(spec_kit_root: Path) -> list[str]:
    skills_dir = spec_kit_root / "skills"
    if not skills_dir.is_dir():
        return []

    names: list[str] = []
    for entry in sorted(skills_dir.iterdir(), key=lambda p: p.name):
        if not entry.is_dir():
            continue
        if entry.name in {"linters"}:
            continue
        if not (entry / "SKILL.md").is_file():
            continue
        names.append(entry.name)
    return names


def install_skills(
    *,
    spec_kit_root: Path,
    selected_skills: list[str],
    targets: list[InstallTarget],
    namespace: str | None,
    overwrite: bool,
    dry_run: bool,
) -> int:
    available = set(list_skills(spec_kit_root))
    if not available:
        return fail(f"No skills found under {spec_kit_root / 'skills'}")

    missing = sorted(name for name in selected_skills if name not in available)
    if missing:
        return fail(f"Unknown skill(s): {', '.join(missing)}")

    if not targets:
        return fail("No install targets selected.")

    for target in targets:
        for skill_name in selected_skills:
            src = spec_kit_root / "skills" / skill_name
            if namespace:
                dest = target.skills_root / namespace / skill_name
            else:
                dest = target.skills_root / skill_name

            if dry_run:
                print(f"[DRY-RUN] {skill_name} -> {target.name}:{dest}")
                continue

            dest.parent.mkdir(parents=True, exist_ok=True)
            if dest.exists():
                if not overwrite:
                    return fail(
                        f"Destination already exists (use --overwrite): {dest}"
                    )
                shutil.rmtree(dest)

            shutil.copytree(src, dest)
            print(f"[OK] Installed {skill_name} -> {target.name}:{dest}")

    return 0


def _default_codex_home() -> Path:
    return _expand_home(os.environ.get("CODEX_HOME", "~/.codex"))


def _default_claude_home() -> Path:
    return _expand_home(os.environ.get("CLAUDE_HOME", "~/.claude"))


def main(argv: list[str] | None = None) -> None:
    parser = argparse.ArgumentParser(
        description="Install spec-kit skills into Codex/Claude skill directories."
    )
    parser.add_argument(
        "--source",
        type=str,
        default=None,
        help="Path to a spec-kit repo checkout (must contain skills/, templates/, rubrics/).",
    )
    parser.add_argument(
        "--skill",
        action="append",
        default=[],
        help="Skill directory name under skills/ (repeatable).",
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Install all skills found under skills/ (excluding skills/linters).",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="List available skills in --source and exit.",
    )
    parser.add_argument(
        "--target",
        action="append",
        choices=["codex", "claude"],
        default=[],
        help="Install target (repeatable).",
    )
    parser.add_argument(
        "--codex-home",
        type=str,
        default=None,
        help="Override Codex home directory (defaults to $CODEX_HOME or ~/.codex).",
    )
    parser.add_argument(
        "--claude-home",
        type=str,
        default=None,
        help="Override Claude home directory (defaults to $CLAUDE_HOME or ~/.claude).",
    )
    parser.add_argument(
        "--namespace",
        type=str,
        default=None,
        help="Optional subdirectory under skills/ to avoid name collisions.",
    )
    parser.add_argument(
        "--overwrite",
        action="store_true",
        help="Overwrite existing destination skill directories.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print what would be installed without copying files.",
    )

    args = parser.parse_args(argv)

    source: Path | None
    if args.source:
        source = _expand_home(args.source)
    else:
        source = find_spec_kit_root(Path.cwd())

    if source is None:
        raise SystemExit(
            fail("Could not infer spec-kit repo root; pass --source /path/to/spec-kit")
        )

    if args.list:
        for name in list_skills(source):
            print(name)
        raise SystemExit(0)

    selected_skills: list[str]
    if args.all:
        selected_skills = list_skills(source)
    else:
        selected_skills = list(args.skill)

    if not selected_skills:
        raise SystemExit(
            fail("No skills selected. Pass --all or one or more --skill <name>.")
        )

    targets: list[InstallTarget] = []
    for target in args.target:
        if target == "codex":
            home = _expand_home(args.codex_home) if args.codex_home else _default_codex_home()
            targets.append(InstallTarget(name="codex", home=home))
        elif target == "claude":
            home = (
                _expand_home(args.claude_home) if args.claude_home else _default_claude_home()
            )
            targets.append(InstallTarget(name="claude", home=home))

    rc = install_skills(
        spec_kit_root=source,
        selected_skills=selected_skills,
        targets=targets,
        namespace=args.namespace,
        overwrite=bool(args.overwrite),
        dry_run=bool(args.dry_run),
    )
    raise SystemExit(rc)

