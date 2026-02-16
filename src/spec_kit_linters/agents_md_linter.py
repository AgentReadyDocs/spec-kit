from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path


VAGUE_PHRASES = (
    "as appropriate",
    "follow conventions",
    "use standard",
    "when necessary",
    "see docs",
)


@dataclass(frozen=True)
class Finding:
    check_id: str
    severity: str
    message: str
    evidence: str


def extract_section(text: str, section_name: str) -> str:
    pattern = re.compile(
        rf"(?mis)^##\s+{re.escape(section_name)}\s*$\n(.*?)(?=^##\s+|\Z)"
    )
    match = pattern.search(text)
    return match.group(1).strip() if match else ""


def markdown_links(text: str) -> list[str]:
    targets: list[str] = []
    for match in re.finditer(r"\]\(([^)]+)\)", text):
        target = match.group(1).strip()
        if not target:
            continue
        if "://" in target or target.startswith("mailto:"):
            continue
        target = target.split("#", 1)[0].strip()
        if target:
            targets.append(target)
    return targets


def has_must_rule_for(topic: str, block: str) -> bool:
    return bool(re.search(rf"(?im)\bMUST\b.*\b{re.escape(topic)}\b", block))


def has_on_fail_for(topic: str, text: str) -> bool:
    return bool(re.search(rf"(?im)ON FAIL.*\b{re.escape(topic)}\b", text))


def has_essential_command(kind: str, text: str) -> bool:
    command_pattern = re.compile(
        rf"(?im)^\s*(?:-|\*|\d+\.)?\s*`?[^`\n]*\b{re.escape(kind)}\b[^`\n]*`?\s*$"
    )
    return bool(command_pattern.search(text))


def evaluate_agents_md(path: Path, strict: bool) -> tuple[list[Finding], list[Finding]]:
    errors: list[Finding] = []
    warnings: list[Finding] = []

    if not path.exists():
        errors.append(
            Finding(
                "AG001",
                "error",
                "AGENTS.md file does not exist.",
                str(path),
            )
        )
        return errors, warnings
    if not path.is_file():
        errors.append(
            Finding(
                "AG001",
                "error",
                "AGENTS.md path is not a file.",
                str(path),
            )
        )
        return errors, warnings

    text = path.read_text(encoding="utf-8")
    critical = extract_section(text, "CRITICAL")
    commands = extract_section(text, "Commands")

    if not critical:
        errors.append(
            Finding("AG002", "error", "Missing required `## CRITICAL` section.", "header: ## CRITICAL")
        )
    if not commands:
        errors.append(
            Finding("AG003", "error", "Missing required `## Commands` section.", "header: ## Commands")
        )

    if critical:
        never_topic_patterns = {
            "force push": r"(?im)\bNEVER\b.*\bforce\s+push\b",
            "hook": r"(?im)\bNEVER\b.*\bhook(?:s)?\b",
            "secret": r"(?im)\bNEVER\b.*\bsecret(?:s)?\b",
            "generated": r"(?im)\bNEVER\b.*\bgenerated\b",
        }
        missing = [
            topic
            for topic, pattern in never_topic_patterns.items()
            if not re.search(pattern, critical)
        ]
        if missing:
            errors.append(
                Finding(
                    "AG004",
                    "error",
                    "CRITICAL section is missing expected NEVER guardrails.",
                    f"missing topics: {', '.join(missing)}",
                )
            )

        for topic in ("lint", "test"):
            if not has_on_fail_for(topic, critical) and not has_on_fail_for(topic, text):
                errors.append(
                    Finding(
                        "AG005",
                        "error",
                        f"Missing ON FAIL recovery guidance for `{topic}`.",
                        "expected: ON FAIL ...",
                    )
                )

        for topic in ("package", "lint", "test"):
            if not has_must_rule_for(topic, critical):
                errors.append(
                    Finding(
                        "AG006",
                        "error",
                        f"Missing MUST rule for `{topic}` in CRITICAL.",
                        "expected: MUST ...",
                    )
                )

    if not re.search(r"(?im)^agents-md-version\s*:\s*[0-9]+(?:\.[0-9]+)*\s*$", text):
        warnings.append(
            Finding(
                "AG007",
                "warning",
                "Missing `agents-md-version` tag.",
                "expected a top-level metadata line",
            )
        )

    for command in ("install", "lint", "test"):
        if not has_essential_command(command, text):
            errors.append(
                Finding(
                    "AG008",
                    "error",
                    f"Missing essential `{command}` command.",
                    "search in Commands/CRITICAL",
                )
            )

    if re.search(r"(?im)\[(?:TODO|TBD)[^\]]*\]|^\s*(?:TODO|TBD)\s*:", text):
        errors.append(
            Finding(
                "AG009",
                "error",
                "Contains TODO/TBD placeholders.",
                "placeholder token present",
            )
        )

    for target in markdown_links(text):
        resolved = (path.parent / target).resolve()
        if target.startswith("/"):
            continue
        if not resolved.exists():
            errors.append(
                Finding(
                    "AG010",
                    "error",
                    "Broken local markdown link.",
                    target,
                )
            )

    for phrase in VAGUE_PHRASES:
        if re.search(rf"(?im)\b{re.escape(phrase)}\b", text):
            finding = Finding(
                "AG011",
                "warning",
                "Contains vague directive phrase.",
                phrase,
            )
            if strict:
                errors.append(Finding(finding.check_id, "error", finding.message, finding.evidence))
            else:
                warnings.append(finding)

    return errors, warnings


def print_text_results(errors: list[Finding], warnings: list[Finding]) -> None:
    if not errors and not warnings:
        print("[OK] AGENTS.md passed lint checks.")
        return

    if errors:
        print("[FAIL] Errors:")
        for finding in errors:
            print(f"- {finding.check_id}: {finding.message} ({finding.evidence})")

    if warnings:
        print("[WARN] Warnings:")
        for finding in warnings:
            print(f"- {finding.check_id}: {finding.message} ({finding.evidence})")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Lint an AGENTS.md file.")
    parser.add_argument("agents_md_path", help="Path to AGENTS.md")
    parser.add_argument("--strict", action="store_true", help="Treat vague directives as errors")
    parser.add_argument(
        "--format", choices=("text", "json"), default="text", help="Output format"
    )
    args = parser.parse_args(argv)

    path = Path(args.agents_md_path).expanduser().resolve()
    errors, warnings = evaluate_agents_md(path, strict=args.strict)

    if args.format == "json":
        print(
            json.dumps(
                {
                    "path": str(path),
                    "pass": len(errors) == 0,
                    "error_count": len(errors),
                    "warning_count": len(warnings),
                    "errors": [finding.__dict__ for finding in errors],
                    "warnings": [finding.__dict__ for finding in warnings],
                },
                indent=2,
            )
        )
    else:
        print_text_results(errors, warnings)

    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
