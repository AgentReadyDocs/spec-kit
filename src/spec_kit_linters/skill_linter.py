from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path


ALLOWED_FRONTMATTER_KEYS = {
    "name",
    "description",
    "license",
    "compatibility",
    "metadata",
    "allowed-tools",
}


@dataclass(frozen=True)
class Frontmatter:
    keys: set[str]
    name: str
    description: str


def fail(message: str) -> int:
    print(f"[FAIL] {message}", file=sys.stderr)
    return 1


def read_frontmatter_block(skill_md_text: str) -> str:
    if not skill_md_text.startswith("---\n"):
        raise ValueError("SKILL.md must start with YAML frontmatter (---).")
    end = skill_md_text.find("\n---\n", 4)
    if end == -1:
        raise ValueError("SKILL.md frontmatter is missing closing --- delimiter.")
    return skill_md_text[4:end]


def parse_frontmatter(frontmatter_text: str) -> Frontmatter:
    keys: set[str] = set()
    name: str | None = None
    description: str | None = None

    lines = frontmatter_text.splitlines()
    index = 0
    while index < len(lines):
        line = lines[index]
        if not line.strip() or line.lstrip().startswith("#"):
            index += 1
            continue

        match = re.match(r"^([A-Za-z0-9_-]+):(?:\s*(.*))?$", line)
        if not match:
            raise ValueError(f"Invalid frontmatter line: {line!r}")

        key = match.group(1)
        rest = (match.group(2) or "").rstrip()
        keys.add(key)

        if key == "description" and rest.strip() in {">", "|", ">-", "|-"}:
            indicator = rest.strip()
            index += 1
            block_lines: list[str] = []
            indent: int | None = None
            while index < len(lines):
                raw = lines[index]
                if raw.strip() == "":
                    block_lines.append("")
                    index += 1
                    continue
                leading = len(raw) - len(raw.lstrip(" "))
                if indent is None:
                    indent = leading
                if leading < (indent or 0):
                    break
                block_lines.append(raw[(indent or 0) :])
                index += 1

            if indicator.startswith(">"):
                folded: list[str] = []
                paragraph: list[str] = []
                for block_line in block_lines:
                    if block_line == "":
                        if paragraph:
                            folded.append(" ".join(paragraph).strip())
                            paragraph = []
                        folded.append("")
                    else:
                        paragraph.append(block_line.rstrip())
                if paragraph:
                    folded.append(" ".join(paragraph).strip())
                description = "\n".join(folded).strip()
            else:
                description = "\n".join(block_lines).strip()
            continue

        if key == "name":
            value = rest.strip().strip("\"'").strip()
            if value:
                name = value
        elif key == "description":
            value = rest.strip().strip("\"'").strip()
            if value:
                description = value

        if rest.strip() == "":
            index += 1
            continue
        index += 1

    if name is None:
        raise ValueError("Missing required frontmatter key: name")
    if description is None:
        raise ValueError("Missing required frontmatter key: description")

    unexpected = sorted(key for key in keys if key not in ALLOWED_FRONTMATTER_KEYS)
    if unexpected:
        allowed = ", ".join(sorted(ALLOWED_FRONTMATTER_KEYS))
        raise ValueError(
            f"Unexpected frontmatter key(s): {', '.join(unexpected)}. Allowed: {allowed}"
        )

    return Frontmatter(keys=keys, name=name.strip(), description=description.strip())


def validate_name(name: str) -> str | None:
    if not name:
        return "Frontmatter name is empty."
    if len(name) > 64:
        return f"Frontmatter name too long ({len(name)} > 64)."
    if not re.fullmatch(r"[a-z0-9-]+", name):
        return "Frontmatter name must match ^[a-z0-9-]+$."
    if name.startswith("-") or name.endswith("-") or "--" in name:
        return "Frontmatter name cannot start/end with '-' or contain '--'."
    return None


def validate_description(description: str) -> str | None:
    if not description.strip():
        return "Frontmatter description is empty."
    if len(description) > 1024:
        return f"Frontmatter description too long ({len(description)} > 1024)."
    if "<" in description or ">" in description:
        return "Frontmatter description cannot contain '<' or '>'."
    return None


def extract_local_markdown_links(text: str) -> set[str]:
    links = set()
    for match in re.finditer(r"\]\(([^)]+)\)", text):
        target = match.group(1).strip()
        if not target:
            continue
        if "://" in target or target.startswith("mailto:"):
            continue
        target = target.split("#", 1)[0].strip()
        if not target or target.startswith("/"):
            continue
        links.add(target)
    return links


def is_probably_markdown(path: Path) -> bool:
    return path.suffix.lower() in {".md", ".markdown"}


def run_skill_lint(skill_dir: Path, max_lines: int) -> int:
    skill_dir = skill_dir.resolve()
    if not skill_dir.is_dir():
        return fail(f"Not a directory: {skill_dir}")

    skill_md = skill_dir / "SKILL.md"
    if not skill_md.exists():
        return fail(f"Missing SKILL.md: {skill_md}")

    text = skill_md.read_text(encoding="utf-8")
    line_count = len(text.splitlines())
    if line_count > max_lines:
        return fail(f"SKILL.md too long: {line_count} lines (max {max_lines}).")

    placeholder_patterns = [
        r"\[TODO[^\]]*\]",
        r"\[TBD[^\]]*\]",
        r"(?m)^\s*(TODO|TBD)\s*:",
    ]
    if any(re.search(pattern, text) for pattern in placeholder_patterns):
        return fail("SKILL.md contains TODO/TBD placeholders (e.g., [TODO] or TODO:).")

    try:
        frontmatter_block = read_frontmatter_block(text)
        frontmatter = parse_frontmatter(frontmatter_block)
    except Exception as exc:
        return fail(str(exc))

    name_error = validate_name(frontmatter.name)
    if name_error:
        return fail(name_error)
    desc_error = validate_description(frontmatter.description)
    if desc_error:
        return fail(desc_error)

    if frontmatter.name != skill_dir.name:
        return fail(
            f"Frontmatter name '{frontmatter.name}' must match directory name '{skill_dir.name}'."
        )

    links = extract_local_markdown_links(text)
    referenced_md_files: list[Path] = []
    for link in sorted(links):
        resolved = (skill_dir / link).resolve()
        if not resolved.is_relative_to(skill_dir):
            return fail(f"SKILL.md links outside skill dir: {link}")
        if not resolved.exists():
            return fail(f"Broken link target in SKILL.md: {link}")
        if resolved.is_file() and is_probably_markdown(resolved):
            referenced_md_files.append(resolved)

    for ref_md in referenced_md_files:
        ref_text = ref_md.read_text(encoding="utf-8")
        ref_links = extract_local_markdown_links(ref_text)
        chained = []
        for ref_link in sorted(ref_links):
            candidate = (ref_md.parent / ref_link).resolve()
            if candidate.is_file() and is_probably_markdown(candidate):
                chained.append(ref_link)
        if chained:
            return fail(
                f"Deep reference chain: {ref_md.relative_to(skill_dir)} links to {', '.join(chained)}. "
                "List all required references directly in SKILL.md instead."
            )

    openai_yaml = skill_dir / "agents" / "openai.yaml"
    if openai_yaml.exists():
        contents = openai_yaml.read_text(encoding="utf-8")
        required = ["interface:", "display_name:", "short_description:", "default_prompt:"]
        missing = [item for item in required if item not in contents]
        if missing:
            return fail(f"agents/openai.yaml missing required keys: {', '.join(missing)}")

    print("[OK] Skill is valid.")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Validate a skill directory.")
    parser.add_argument("skill_dir", help="Path to a skill directory")
    parser.add_argument("--max-lines", type=int, default=500, help="Max allowed SKILL.md lines")
    args = parser.parse_args(argv)
    return run_skill_lint(Path(args.skill_dir).expanduser().resolve(), args.max_lines)


if __name__ == "__main__":
    raise SystemExit(main())
