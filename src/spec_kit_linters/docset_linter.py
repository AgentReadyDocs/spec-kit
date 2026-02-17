from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True)
class Finding:
    check_id: str
    severity: str  # "error" | "warning"
    message: str
    evidence: str


@dataclass(frozen=True)
class Doc:
    path: Path
    doc_id: str
    doc_type: str | None
    risk_tier: str | None
    links: dict[str, object]
    text: str


def _extract_frontmatter_block(text: str) -> str | None:
    if not text.startswith("---\n"):
        return None
    end = text.find("\n---\n", 4)
    if end == -1:
        return None
    return text[4:end]


def _parse_yaml_scalar(value: str) -> object:
    value = value.strip()
    if not value:
        return ""
    if (value.startswith('"') and value.endswith('"')) or (
        value.startswith("'") and value.endswith("'")
    ):
        return value[1:-1]
    if value in {"[]", "[ ]"}:
        return []
    if value.lower() in {"true", "false"}:
        return value.lower() == "true"
    return value


def _parse_yaml_subset(frontmatter_block: str) -> dict[str, object]:
    """
    Minimal YAML parser for the subset used in spec-kit templates/examples:
    - key: value
    - nested mappings via indentation
    - lists via "- item" under a key
    """
    root: dict[str, object] = {}
    stack: list[tuple[int, object]] = [(0, root)]

    for raw_line in frontmatter_block.splitlines():
        if not raw_line.strip():
            continue
        if raw_line.lstrip().startswith("#"):
            continue

        indent = len(raw_line) - len(raw_line.lstrip(" "))
        line = raw_line.strip()

        while len(stack) > 1 and indent < stack[-1][0]:
            stack.pop()

        container = stack[-1][1]

        if line.startswith("- "):
            item = _parse_yaml_scalar(line[2:])
            if not isinstance(container, list):
                raise ValueError("Invalid YAML subset: list item outside list context.")
            container.append(item)
            continue

        match = re.match(r"^([A-Za-z0-9_-]+):(.*)$", line)
        if not match:
            raise ValueError(f"Invalid YAML subset line: {raw_line!r}")

        key = match.group(1)
        rest = match.group(2).lstrip()

        if not isinstance(container, dict):
            raise ValueError("Invalid YAML subset: mapping key under list context.")

        if rest == "":
            child: object = {}
            container[key] = child
            stack.append((indent + 1, child))
            continue

        if rest == "[]":
            container[key] = []
            continue

        container[key] = _parse_yaml_scalar(rest)

    # Post-process: treat "links: []" as links: {} for safety
    if isinstance(root.get("links"), list):
        root["links"] = {}

    # Normalize: if links.use_cases exists but isn't a list, coerce to list
    links = root.get("links")
    if isinstance(links, dict) and "use_cases" in links and not isinstance(links["use_cases"], list):
        links["use_cases"] = [links["use_cases"]]

    return root


def _markdown_links(text: str) -> list[str]:
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


def _extract_section_span(text: str, section_name: str) -> tuple[int, int] | None:
    pattern = re.compile(
        rf"(?mis)^##\s+{re.escape(section_name)}\s*$\n(.*?)(?=^##\s+|\Z)"
    )
    match = pattern.search(text)
    if not match:
        return None
    return match.start(1), match.end(1)


def _extract_section(text: str, section_name: str) -> str:
    span = _extract_section_span(text, section_name)
    if not span:
        return ""
    return text[span[0] : span[1]].strip()


def _parse_markdown_table(section_text: str) -> tuple[list[str], list[list[str]]]:
    """
    Returns (headers, rows) for the first table found in section_text.
    """
    lines = [ln.rstrip("\n") for ln in section_text.splitlines()]
    for index in range(len(lines) - 1):
        header_line = lines[index].strip()
        sep_line = lines[index + 1].strip()
        if "|" not in header_line or "|" not in sep_line:
            continue
        if not re.search(r"\|\s*-{3,}", sep_line):
            continue

        def split_row(row: str) -> list[str]:
            row = row.strip()
            if row.startswith("|"):
                row = row[1:]
            if row.endswith("|"):
                row = row[:-1]
            return [cell.strip() for cell in row.split("|")]

        headers = split_row(header_line)
        rows: list[list[str]] = []
        j = index + 2
        while j < len(lines):
            raw = lines[j].strip()
            if not raw:
                break
            if "|" not in raw:
                break
            rows.append(split_row(raw))
            j += 1

        return headers, rows
    return [], []


def _normalize_header(header: str) -> str:
    return re.sub(r"[^a-z0-9]+", "_", header.strip().lower()).strip("_")


def _table_column_index(headers: list[str], column_name: str) -> int | None:
    want = _normalize_header(column_name)
    for idx, header in enumerate(headers):
        if _normalize_header(header) == want:
            return idx
    return None


_EXAMPLE_PLACEHOLDER_PATTERNS: list[tuple[str, str]] = [
    (r"\bUC-XXXX\b", "UC-XXXX"),
    (r"\bADR-XXXX\b", "ADR-XXXX"),
    (r"\bNFR-BASELINE\b", "NFR-BASELINE"),
    (r"\bGLOSSARY-ENTITIES\b", "GLOSSARY-ENTITIES"),
    (r"\bACTOR_[A-Z0-9_]+\b", "ACTOR_*"),
    (r"\[Short title\]", "[Short title]"),
    (r"\[System name\]", "[System name]"),
    (r"\[One sentence outcome\]", "[One sentence outcome]"),
    (r"\[bullet list\]", "[bullet list]"),
    (r"\[role\]", "[role]"),
    (r"\[predicate\]", "[predicate]"),
    (r"\[field\]", "[field]"),
    (r"\[type\]", "[type]"),
    (r"\[constraints\]", "[constraints]"),
    (r"\[example\]", "[example]"),
    (r"\bTBD\b", "TBD"),
    (r"\bTODO\b", "TODO"),
]


def _scan_placeholders(
    *,
    path: Path,
    text: str,
    check_id: str,
    severity: str,
    allow_warnings_in_open_questions: bool,
) -> tuple[list[Finding], list[Finding]]:
    errors: list[Finding] = []
    warnings: list[Finding] = []

    open_questions_span = _extract_section_span(text, "Open Questions")

    for pattern, token in _EXAMPLE_PLACEHOLDER_PATTERNS:
        for match in re.finditer(pattern, text):
            start = match.start()
            line_start = text.rfind("\n", 0, start) + 1
            line_end = text.find("\n", start)
            if line_end == -1:
                line_end = len(text)
            line = text[line_start:line_end]

            is_open_questions = (
                open_questions_span is not None
                and open_questions_span[0] <= start <= open_questions_span[1]
            )
            is_declared_open = "[OPEN]" in line or "[ASSUMPTION]" in line

            finding = Finding(
                check_id=check_id,
                severity=severity,
                message=f"Template placeholder token found: {token}",
                evidence=f"{path}: {line.strip()[:200]}",
            )

            if allow_warnings_in_open_questions and (is_open_questions or is_declared_open):
                warnings.append(Finding(finding.check_id, "warning", finding.message, finding.evidence))
            else:
                errors.append(finding)

    return errors, warnings


def _is_tier2_or_higher(risk_tier: str | None) -> bool:
    if not risk_tier:
        return False
    return risk_tier.strip().lower() in {"tier2", "tier3"}


def _looks_non_placeholder(value: str) -> bool:
    v = value.strip()
    if not v or v in {"-", "N/A", "n/a"}:
        return False
    if "[" in v and "]" in v:
        return False
    if v.lower() in {"tbd", "todo"}:
        return False
    return True


def _table_has_non_placeholder_rows(section_text: str, required_columns: list[str]) -> bool:
    headers, rows = _parse_markdown_table(section_text)
    if not headers or not rows:
        return False
    indices: list[int] = []
    for col in required_columns:
        idx = _table_column_index(headers, col)
        if idx is None:
            return False
        indices.append(idx)
    for row in rows:
        if all(i < len(row) and _looks_non_placeholder(row[i]) for i in indices):
            return True
    return False


_DECISIONFUL_KEYWORDS_CONTRACT = {
    "schema",
    "contract",
    "breaking",
    "deprecat",
    "compat",
    "migration",
    "version",
    "protobuf",
    "openapi",
}

_DECISIONFUL_KEYWORDS_MONEY = {
    "payment",
    "charge",
    "billing",
    "invoice",
    "refund",
    "payout",
    "money",
    "usd",
    "$",
}


def _is_decisionful_uc(text: str) -> bool:
    authz_section = _extract_section(text, "AuthZ")
    if authz_section and _table_has_non_placeholder_rows(
        authz_section, ["actor_id", "condition", "decision"]
    ):
        return True

    invariants_section = _extract_section(text, "Invariants And Policies")
    if invariants_section and _table_has_non_placeholder_rows(
        invariants_section, ["invariant", "enforcement"]
    ):
        return True

    lowered = text.lower()
    if any(k in lowered for k in _DECISIONFUL_KEYWORDS_CONTRACT):
        return True
    if any(k in lowered for k in _DECISIONFUL_KEYWORDS_MONEY):
        return True
    return False


def _resolve_local_path(base: Path, target: str) -> Path | None:
    target = target.strip()
    if not target:
        return None
    if target.startswith("/"):
        return None
    if "://" in target or target.startswith("mailto:"):
        return None
    target = target.split("#", 1)[0].strip()
    if not target:
        return None
    return (base / target).resolve()


def _read_doc(path: Path) -> Doc | None:
    try:
        text = path.read_text(encoding="utf-8")
    except Exception:
        return None

    block = _extract_frontmatter_block(text)
    if not block:
        return None

    try:
        data = _parse_yaml_subset(block)
    except Exception:
        data = {}

    doc_id = str(data.get("id") or "").strip()
    doc_type = data.get("type")
    if isinstance(doc_type, str):
        doc_type_str: str | None = doc_type.strip()
    else:
        doc_type_str = None
    risk_tier = data.get("risk_tier")
    risk_tier_str = risk_tier.strip() if isinstance(risk_tier, str) else None
    links = data.get("links")
    links_dict = links if isinstance(links, dict) else {}

    if not doc_id:
        return Doc(path=path, doc_id="", doc_type=doc_type_str, risk_tier=risk_tier_str, links=links_dict, text=text)

    return Doc(
        path=path,
        doc_id=doc_id,
        doc_type=doc_type_str,
        risk_tier=risk_tier_str,
        links=links_dict,
        text=text,
    )


def lint_docset(
    *,
    root: Path,
    docs_dir: str,
    examples_dir: str,
    check_doc_placeholders: bool,
    strict: bool,
) -> tuple[list[Finding], list[Finding]]:
    errors: list[Finding] = []
    warnings: list[Finding] = []

    root = root.resolve()
    docs_path = (root / docs_dir).resolve()
    examples_path = (root / examples_dir).resolve()

    markdown_files: list[Path] = []
    if docs_path.exists() and docs_path.is_dir():
        markdown_files.extend([p for p in docs_path.rglob("*.md") if p.is_file()])
    if examples_path.exists() and examples_path.is_dir():
        markdown_files.extend([p for p in examples_path.rglob("*.md") if p.is_file()])

    # Index docs
    docs: list[Doc] = []
    by_id: dict[str, Doc] = {}
    adr_docs: list[Doc] = []
    uc_docs: list[Doc] = []
    glossary_docs: list[Doc] = []
    nfr_docs: list[Doc] = []

    for path in sorted(set(markdown_files)):
        doc = _read_doc(path)
        if not doc:
            continue

        if not doc.doc_id:
            errors.append(
                Finding(
                    "DS-ID-001",
                    "error",
                    "Missing required frontmatter `id`.",
                    str(path),
                )
            )
            continue

        if doc.doc_id in by_id:
            errors.append(
                Finding(
                    "DS-ID-002",
                    "error",
                    f"Duplicate document id: {doc.doc_id}",
                    f"{by_id[doc.doc_id].path} and {doc.path}",
                )
            )
            continue

        docs.append(doc)
        by_id[doc.doc_id] = doc

        if doc.doc_type == "use_case":
            uc_docs.append(doc)
        if doc.doc_type == "glossary":
            glossary_docs.append(doc)
        if doc.doc_type == "nfr":
            nfr_docs.append(doc)
        if doc.doc_id.startswith("ADR-"):
            adr_docs.append(doc)

    # Placeholder scanning
    if examples_path.exists() and examples_path.is_dir():
        for doc in docs:
            if doc.path.is_relative_to(examples_path):
                e, w = _scan_placeholders(
                    path=doc.path,
                    text=doc.text,
                    check_id="EX-CF-001",
                    severity="error",
                    allow_warnings_in_open_questions=False,
                )
                errors.extend(e)
                warnings.extend(w)

    if check_doc_placeholders and docs_path.exists() and docs_path.is_dir():
        for doc in docs:
            if doc.path.is_relative_to(docs_path):
                e, w = _scan_placeholders(
                    path=doc.path,
                    text=doc.text,
                    check_id="DS-DOC-PLACEHOLDERS",
                    severity="error",
                    allow_warnings_in_open_questions=True,
                )
                errors.extend(e)
                warnings.extend(w)

    # DS-CF-001 / DS-CF-002: UC links.glossary and links.nfr resolve
    for uc in uc_docs:
        links = uc.links
        glossary_link = links.get("glossary") if isinstance(links, dict) else None
        nfr_link = links.get("nfr") if isinstance(links, dict) else None

        for key, value, check_id in [
            ("glossary", glossary_link, "DS-CF-001"),
            ("nfr", nfr_link, "DS-CF-002"),
        ]:
            if not isinstance(value, str) or not value.strip():
                errors.append(
                    Finding(
                        check_id,
                        "error",
                        f"Use case missing required links.{key} (or explicit N/A).",
                        str(uc.path),
                    )
                )
                continue
            if value.strip().lower() == "n/a":
                continue
            resolved = _resolve_local_path(uc.path.parent, value)
            if not resolved or not resolved.exists():
                errors.append(
                    Finding(
                        check_id,
                        "error",
                        f"Use case links.{key} does not resolve to an existing file.",
                        f"{uc.path}: links.{key}={value!r}",
                    )
                )

    # DS-S-001: internal markdown links resolve (lightweight)
    for doc in docs:
        for target in _markdown_links(doc.text):
            resolved = _resolve_local_path(doc.path.parent, target)
            if not resolved:
                continue
            if not resolved.exists():
                errors.append(
                    Finding(
                        "DS-S-001",
                        "error",
                        "Broken local markdown link.",
                        f"{doc.path}: {target}",
                    )
                )

    # DS-CF-003: UC referenced entities exist in linked glossary
    for uc in uc_docs:
        links = uc.links
        glossary_link = links.get("glossary") if isinstance(links, dict) else None
        if not isinstance(glossary_link, str) or not glossary_link.strip():
            continue
        if glossary_link.strip().lower() == "n/a":
            continue

        glossary_path = _resolve_local_path(uc.path.parent, glossary_link)
        if not glossary_path or not glossary_path.exists():
            continue

        glossary_doc = _read_doc(glossary_path)
        if not glossary_doc:
            continue

        terms_section = _extract_section(glossary_doc.text, "Terms")
        terms_headers, terms_rows = _parse_markdown_table(terms_section)
        terms_col = _table_column_index(terms_headers, "term") if terms_headers else None
        terms = {
            row[terms_col].strip()
            for row in terms_rows
            if terms_col is not None and terms_col < len(row) and row[terms_col].strip()
        }

        entities_section = _extract_section(glossary_doc.text, "Entities")
        ent_headers, ent_rows = _parse_markdown_table(entities_section)
        ent_col = _table_column_index(ent_headers, "entity") if ent_headers else None
        entities = {
            row[ent_col].strip()
            for row in ent_rows
            if ent_col is not None and ent_col < len(row) and row[ent_col].strip()
        }

        uc_entities_section = _extract_section(uc.text, "Entities (Referenced)")
        uc_ent_headers, uc_ent_rows = _parse_markdown_table(uc_entities_section)
        uc_ent_col = _table_column_index(uc_ent_headers, "entity") if uc_ent_headers else None
        if uc_ent_col is None:
            continue

        for row in uc_ent_rows:
            if uc_ent_col >= len(row):
                continue
            entity = row[uc_ent_col].strip()
            if not entity or "[" in entity or "]" in entity:
                continue
            if entity not in terms and entity not in entities:
                errors.append(
                    Finding(
                        "DS-CF-003",
                        "error",
                        "Use case references an entity not present in linked glossary (as a Term or Entity).",
                        f"{uc.path}: entity={entity!r}, glossary={glossary_link!r}",
                    )
                )

    # DS-S-002: H1 begins with ID (lightweight)
    h1_pattern = re.compile(r"(?m)^#\s+(.+?)\s*$")
    for doc in docs:
        match = h1_pattern.search(doc.text)
        if not match:
            continue
        # Only enforce for doc types whose templates prefix H1 with the stable ID.
        # (Use cases: "# UC-####: ...", ADRs: "# ADR-####: ...")
        if not (doc.doc_type == "use_case" or doc.doc_id.startswith("ADR-")):
            continue
        h1 = match.group(1).strip()
        if doc.doc_id and not h1.startswith(doc.doc_id):
            warnings.append(
                Finding(
                    "DS-S-002",
                    "warning",
                    "H1 header does not start with the frontmatter id.",
                    f"{doc.path}: H1={h1!r}, id={doc.doc_id!r}",
                )
            )

    # DS-S-007 / DS-S-008: example presence checks (deterministic subset)
    if examples_path.exists() and examples_path.is_dir():
        has_uc_example = any(d.path.is_relative_to(examples_path) and d.doc_type == "use_case" for d in docs)
        has_glossary_example = any(d.path.is_relative_to(examples_path) and d.doc_type == "glossary" for d in docs)
        has_nfr_example = any(d.path.is_relative_to(examples_path) and d.doc_type == "nfr" for d in docs)

        if not has_uc_example:
            warnings.append(
                Finding(
                    "DS-S-007",
                    "warning",
                    "No use case example found under examples/.",
                    str(examples_path),
                )
            )
        if not has_glossary_example:
            warnings.append(
                Finding(
                    "DS-S-007",
                    "warning",
                    "No glossary/entities example found under examples/.",
                    str(examples_path),
                )
            )

        requires_nfr_example = any(
            d.path.is_relative_to(examples_path)
            and d.doc_type == "use_case"
            and (d.risk_tier or "").strip().lower() in {"tier1", "tier2", "tier3"}
            for d in docs
        )
        if requires_nfr_example and not has_nfr_example:
            warnings.append(
                Finding(
                    "DS-S-008",
                    "warning",
                    "No NFR baseline example found under examples/ (recommended for tier1+).",
                    str(examples_path),
                )
            )

    # DS-CF-004: tier2+ decisionful UCs require ADR linking them
    adr_uc_links: dict[str, list[Path]] = {}
    for adr in adr_docs:
        use_cases = adr.links.get("use_cases") if isinstance(adr.links, dict) else None
        if not isinstance(use_cases, list):
            continue
        for uc_id in [str(x).strip() for x in use_cases if str(x).strip()]:
            adr_uc_links.setdefault(uc_id, []).append(adr.path)

    for uc in uc_docs:
        if not _is_tier2_or_higher(uc.risk_tier):
            continue
        if not _is_decisionful_uc(uc.text):
            continue
        if uc.doc_id not in adr_uc_links:
            errors.append(
                Finding(
                    "DS-CF-004",
                    "error",
                    "Tier2+ use case appears decisionful but no ADR links it via frontmatter links.use_cases.",
                    str(uc.path),
                )
            )

    # strict mode: upgrade warnings to errors
    if strict and warnings:
        errors.extend([Finding(w.check_id, "error", w.message, w.evidence) for w in warnings])
        warnings = []

    return errors, warnings


def _print_text_results(errors: list[Finding], warnings: list[Finding]) -> None:
    if not errors and not warnings:
        print("[OK] Docset passed lint checks.")
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
    parser = argparse.ArgumentParser(description="Lint a spec-kit doc set for consistency.")
    parser.add_argument("root", help="Repo root or directory containing docs/ and/or examples/")
    parser.add_argument("--docs-dir", default="docs", help="Docs directory name under root")
    parser.add_argument("--examples-dir", default="examples", help="Examples directory name under root")
    parser.add_argument(
        "--check-doc-placeholders",
        action="store_true",
        help="Also scan docs/ for template placeholders (warnings in Open Questions, errors elsewhere).",
    )
    parser.add_argument("--strict", action="store_true", help="Treat warnings as errors")
    parser.add_argument("--format", choices=("text", "json"), default="text", help="Output format")
    args = parser.parse_args(argv)

    root = Path(args.root).expanduser().resolve()
    errors, warnings = lint_docset(
        root=root,
        docs_dir=args.docs_dir,
        examples_dir=args.examples_dir,
        check_doc_placeholders=args.check_doc_placeholders,
        strict=args.strict,
    )

    if args.format == "json":
        print(
            json.dumps(
                {
                    "root": str(root),
                    "pass": len(errors) == 0,
                    "error_count": len(errors),
                    "warning_count": len(warnings),
                    "errors": [f.__dict__ for f in errors],
                    "warnings": [f.__dict__ for f in warnings],
                },
                indent=2,
            )
        )
    else:
        _print_text_results(errors, warnings)

    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
