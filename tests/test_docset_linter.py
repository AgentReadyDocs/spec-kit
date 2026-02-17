from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "src"))
from spec_kit_linters.docset_linter import lint_docset


def _write(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def test_docset_linter_passes_minimal_consistent_set(tmp_path: Path) -> None:
    root = tmp_path
    _write(
        root / "docs" / "glossary.md",
        "---\n"
        "id: GLOSSARY-001\n"
        "type: glossary\n"
        "title: \"Glossary\"\n"
        "owner: \"@x\"\n"
        "last_updated: 2026-02-12\n"
        "---\n\n"
        "# GLOSSARY-001: Glossary\n\n"
        "## Terms\n"
        "| term | definition (one line) | allowed_synonyms | banned_synonyms |\n"
        "|------|------------------------|------------------|-----------------|\n"
        "| Widget | A widget. | - | - |\n\n"
        "## Entities\n"
        "| entity | description (one line) | identifier | source_of_truth |\n"
        "|--------|--------------------------|------------|-----------------|\n"
        "| Widget | A widget record. | widget_id | db |\n",
    )
    _write(
        root / "docs" / "nfr.md",
        "---\n"
        "id: NFR-0001\n"
        "type: nfr\n"
        "title: \"NFR\"\n"
        "owner: \"@x\"\n"
        "last_updated: 2026-02-12\n"
        "---\n\n"
        "# NFR-0001: NFR\n",
    )
    _write(
        root / "docs" / "uc.md",
        "---\n"
        "id: UC-0001\n"
        "type: use_case\n"
        "title: \"Create Widget\"\n"
        "status: draft\n"
        "owner: \"@x\"\n"
        "risk_tier: tier1\n"
        "system: \"Widget Service\"\n"
        "links:\n"
        "  glossary: \"./glossary.md\"\n"
        "  nfr: \"./nfr.md\"\n"
        "---\n\n"
        "# UC-0001: Create Widget\n\n"
        "## Entities (Referenced)\n"
        "| entity | identifier | notes |\n"
        "|--------|------------|-------|\n"
        "| Widget | widget_id | - |\n\n"
        "## Open Questions\n"
        "| open_id | question | owner | due | impact |\n"
        "|---------|----------|-------|-----|--------|\n",
    )

    errors, warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert errors == []
    assert all(w.check_id != "EX-CF-001" for w in warnings)


def test_docset_linter_fails_broken_uc_links(tmp_path: Path) -> None:
    root = tmp_path
    _write(
        root / "docs" / "uc.md",
        "---\n"
        "id: UC-0001\n"
        "type: use_case\n"
        "risk_tier: tier1\n"
        "links:\n"
        "  glossary: \"./missing.md\"\n"
        "  nfr: \"N/A\"\n"
        "---\n\n"
        "# UC-0001: Test\n",
    )

    errors, _warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert any(f.check_id == "DS-CF-001" for f in errors)


def test_docset_linter_fails_duplicate_ids(tmp_path: Path) -> None:
    root = tmp_path
    _write(root / "docs" / "a.md", "---\nid: UC-0001\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: A\n")
    _write(root / "docs" / "b.md", "---\nid: UC-0001\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: B\n")

    errors, _warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert any(f.check_id == "DS-ID-002" for f in errors)


def test_docset_linter_fails_missing_entity_in_glossary(tmp_path: Path) -> None:
    root = tmp_path
    _write(
        root / "docs" / "glossary.md",
        "---\n"
        "id: GLOSSARY-001\n"
        "type: glossary\n"
        "---\n\n"
        "# GLOSSARY-001: Glossary\n\n"
        "## Terms\n"
        "| term | definition (one line) | allowed_synonyms | banned_synonyms |\n"
        "|------|------------------------|------------------|-----------------|\n"
        "| Widget | A widget. | - | - |\n",
    )
    _write(
        root / "docs" / "uc.md",
        "---\n"
        "id: UC-0001\n"
        "type: use_case\n"
        "risk_tier: tier1\n"
        "links:\n"
        "  glossary: \"./glossary.md\"\n"
        "  nfr: \"N/A\"\n"
        "---\n\n"
        "# UC-0001: Test\n\n"
        "## Entities (Referenced)\n"
        "| entity | identifier | notes |\n"
        "|--------|------------|-------|\n"
        "| Gadget | gadget_id | - |\n",
    )

    errors, _warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert any(f.check_id == "DS-CF-003" for f in errors)


def test_docset_linter_fails_example_placeholders(tmp_path: Path) -> None:
    root = tmp_path
    _write(
        root / "examples" / "uc.md",
        "---\n"
        "id: UC-0001\n"
        "type: use_case\n"
        "risk_tier: tier1\n"
        "links:\n"
        "  glossary: N/A\n"
        "  nfr: N/A\n"
        "---\n\n"
        "# UC-XXXX: [Short title]\n",
    )

    errors, _warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert any(f.check_id == "EX-CF-001" for f in errors)


def test_docset_linter_requires_adr_for_decisionful_tier2_uc(tmp_path: Path) -> None:
    root = tmp_path
    _write(
        root / "docs" / "glossary.md",
        "---\n"
        "id: GLOSSARY-001\n"
        "type: glossary\n"
        "---\n\n"
        "# GLOSSARY-001: Glossary\n\n"
        "## Terms\n"
        "| term | definition (one line) | allowed_synonyms | banned_synonyms |\n"
        "|------|------------------------|------------------|-----------------|\n"
        "| Widget | A widget. | - | - |\n",
    )
    _write(
        root / "docs" / "uc.md",
        "---\n"
        "id: UC-0002\n"
        "type: use_case\n"
        "risk_tier: tier2\n"
        "links:\n"
        "  glossary: \"./glossary.md\"\n"
        "  nfr: N/A\n"
        "---\n\n"
        "# UC-0002: Decisionful\n\n"
        "## Interface Contract\n\n"
        "### AuthZ\n"
        "| rule_id | actor_id | condition | decision |\n"
        "|---------|----------|-----------|----------|\n"
        "| AUTHZ-001 | user | actor has perm | allow |\n",
    )

    errors, _warnings = lint_docset(
        root=root, docs_dir="docs", examples_dir="examples", check_doc_placeholders=False, strict=False
    )
    assert any(f.check_id == "DS-CF-004" for f in errors)

