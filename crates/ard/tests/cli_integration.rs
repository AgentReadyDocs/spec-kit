use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::TempDir;

fn run_ard(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ard"))
        .args(args)
        .output()
        .expect("failed to run ard")
}

fn run_ard_in(dir: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ard"))
        .current_dir(dir)
        .args(args)
        .output()
        .expect("failed to run ard")
}

fn write(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent");
    }
    fs::write(path, text).expect("write file");
}

#[test]
fn skill_list_succeeds() {
    let output = run_ard(&["skill", "list"]);
    assert!(output.status.success());
}

#[test]
fn skill_install_single_skill_non_dry_run_succeeds() {
    let listed = run_ard(&["skill", "list"]);
    assert!(listed.status.success());
    let stdout = String::from_utf8_lossy(&listed.stdout);
    let first_skill = stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");
    if first_skill.is_empty() {
        return;
    }

    let tmp = TempDir::new().expect("tempdir");
    let home = tmp.path().join("codex-home");
    let output = run_ard(&[
        "skill",
        "install",
        "--target",
        "codex",
        "--home",
        &home.to_string_lossy(),
        "--skill",
        first_skill,
    ]);
    assert!(output.status.success());
}

#[test]
fn skill_install_requires_target() {
    let output = run_ard(&["skill", "install", "--all"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("No --target specified"));
}

#[test]
fn skill_install_dry_run_succeeds() {
    let tmp = TempDir::new().expect("tempdir");
    let home = tmp.path().join("codex-home");
    let output = run_ard(&[
        "skill",
        "install",
        "--target",
        "codex",
        "--home",
        &home.to_string_lossy(),
        "--all",
        "--dry-run",
    ]);
    assert!(output.status.success());
}

#[test]
fn template_and_rubric_list_succeed() {
    let template = run_ard(&["template", "list"]);
    assert!(template.status.success());
    let rubric = run_ard(&["rubric", "list"]);
    assert!(rubric.status.success());
}

#[test]
fn template_print_first_listed_asset_succeeds() {
    let listed = run_ard(&["template", "list"]);
    assert!(listed.status.success());
    let stdout = String::from_utf8_lossy(&listed.stdout);
    let first_template = stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("");
    assert!(!first_template.is_empty());
    let printed = run_ard(&["template", "print", first_template]);
    assert!(printed.status.success());
}

#[test]
fn template_print_unknown_fails() {
    let output = run_ard(&["template", "print", "missing.md"]);
    assert!(!output.status.success());
}

#[test]
fn lint_agents_md_subcommand_succeeds_on_valid_file() {
    let tmp = TempDir::new().expect("tempdir");
    let agents = tmp.path().join("AGENTS.md");
    write(
        &agents,
        "# AGENTS.md\nagents-md-version: 1\n\n## CRITICAL\n\n- MUST: package manager\n- MUST: lint command before commit\n- MUST: test command before PR\n- NEVER: force push\n- NEVER: bypass hook checks\n- NEVER: commit secrets\n- NEVER: edit generated files\n- ON FAIL (lint): fix lint\n- ON FAIL (test): fix tests\n\n## Commands\n\n```bash\n# install\nx\n# lint\nx\n# test\nx\n```\n",
    );
    let output = run_ard(&["lint", "agents-md", &agents.to_string_lossy()]);
    assert!(output.status.success());
}

#[test]
fn lint_auto_missing_path_fails() {
    let output = run_ard(&["lint", "/tmp/definitely-not-existing-ard-path.md"]);
    assert!(!output.status.success());
}

#[test]
fn lint_without_path_fails() {
    let output = run_ard(&["lint"]);
    assert!(!output.status.success());
}

#[test]
fn lint_auto_skill_file_succeeds_on_minimal_skill() {
    let tmp = TempDir::new().expect("tempdir");
    let skill_dir = tmp.path().join("demo-skill");
    write(
        &skill_dir.join("SKILL.md"),
        "---\nname: demo-skill\ndescription: Demo skill\n---\n\n[Ref](references/ref.md)\n",
    );
    write(&skill_dir.join("references/ref.md"), "# Ref\n");
    let output = run_ard(&["lint", &skill_dir.join("SKILL.md").to_string_lossy()]);
    assert!(output.status.success());
}

#[test]
fn lint_auto_markdown_with_root_succeeds() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path();
    let uc = root.join("docs/uc.md");
    write(
        &uc,
        "---\nid: UC-0001\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: Title\n",
    );
    let output = run_ard(&[
        "lint",
        "--root",
        &root.to_string_lossy(),
        &uc.to_string_lossy(),
    ]);
    assert!(output.status.success());
}

#[test]
fn lint_skill_and_docs_subcommands_succeed() {
    let tmp = TempDir::new().expect("tempdir");

    let skill_dir = tmp.path().join("sub-skill");
    write(
        &skill_dir.join("SKILL.md"),
        "---\nname: sub-skill\ndescription: Demo skill\n---\n\n[Ref](references/ref.md)\n",
    );
    write(&skill_dir.join("references/ref.md"), "# Ref\n");
    let lint_skill = run_ard(&[
        "lint",
        "skill",
        &skill_dir.to_string_lossy(),
        "--max-lines",
        "500",
    ]);
    assert!(lint_skill.status.success());

    let doc_root = tmp.path().join("docset");
    write(
        &doc_root.join("docs/uc.md"),
        "---\nid: UC-1111\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-1111: Title\n",
    );
    let lint_docs = run_ard(&[
        "lint",
        "docs",
        &doc_root.to_string_lossy(),
        "--docs-dir",
        "docs",
        "--examples-dir",
        "examples",
    ]);
    assert!(lint_docs.status.success());
}

#[test]
fn lint_auto_agents_and_docset_dir_and_unsupported_file() {
    let tmp = TempDir::new().expect("tempdir");
    let agents = tmp.path().join("AGENTS.md");
    write(
        &agents,
        "# AGENTS.md\nagents-md-version: 1\n\n## CRITICAL\n\n- MUST: package manager\n- MUST: lint command before commit\n- MUST: test command before PR\n- NEVER: force push\n- NEVER: bypass hook checks\n- NEVER: commit secrets\n- NEVER: edit generated files\n- ON FAIL (lint): fix lint\n- ON FAIL (test): fix tests\n\n## Commands\n\n```bash\n# install\nx\n# lint\nx\n# test\nx\n```\n",
    );
    let lint_agents = run_ard(&["lint", &agents.to_string_lossy()]);
    assert!(lint_agents.status.success());

    let doc_root = tmp.path().join("docset2");
    write(
        &doc_root.join("docs/uc.md"),
        "---\nid: UC-2222\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-2222: Title\n",
    );
    let lint_dir = run_ard(&["lint", &doc_root.to_string_lossy()]);
    assert!(lint_dir.status.success());

    write(&tmp.path().join("not-md.txt"), "x");
    let lint_txt = run_ard(&["lint", &tmp.path().join("not-md.txt").to_string_lossy()]);
    assert!(!lint_txt.status.success());
}

#[test]
fn lint_github_format_emits_workflow_annotations() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path().join("docset");
    // Missing frontmatter id triggers DS-ID-001.
    write(
        &root.join("docs/bad.md"),
        "---\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: Title\n",
    );

    let output = run_ard(&["lint", "--format", "github", &root.to_string_lossy()]);
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("::error"), "{stdout}");
    assert!(stdout.contains("title=DS-ID-001"), "{stdout}");
}

#[test]
fn conformance_vectors_core_and_strict_behave_as_expected() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    let core_valid = repo_root.join("schemas/arsf/0.1.0/vectors/core/valid_minimal");
    let core_ok = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "core",
        &core_valid.to_string_lossy(),
    ]);
    assert!(core_ok.status.success());

    let core_invalid = repo_root.join("schemas/arsf/0.1.0/vectors/core/invalid_missing_id");
    let core_bad = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "core",
        &core_invalid.to_string_lossy(),
    ]);
    assert!(!core_bad.status.success());
    let stdout = String::from_utf8_lossy(&core_bad.stdout);
    assert!(stdout.contains("DS-ID-001"), "{stdout}");

    let strict_invalid = repo_root.join("schemas/arsf/0.1.0/vectors/strict/invalid_h1_mismatch");
    let strict_bad = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "strict",
        &strict_invalid.to_string_lossy(),
    ]);
    assert!(!strict_bad.status.success());
    let stdout = String::from_utf8_lossy(&strict_bad.stdout);
    assert!(stdout.contains("DS-S-002"), "{stdout}");
}

#[test]
fn init_creates_strict_conformant_docset_in_temp_dir() {
    let tmp = TempDir::new().expect("tempdir");
    let spec_root = Path::new("spec");

    // Run in the tempdir so generated workflow/files never touch the repo checkout.
    let init = run_ard_in(tmp.path(), &[
        "init",
        "--root",
        &spec_root.to_string_lossy(),
        "--workflow",
        "--overwrite",
    ]);
    assert!(init.status.success());

    let conf = run_ard_in(tmp.path(), &[
        "conformance",
        "run",
        "--profile",
        "strict",
        &spec_root.to_string_lossy(),
    ]);
    assert!(conf.status.success());
}

#[test]
fn new_subcommands_write_docs_and_handle_missing_dir_inference() {
    let tmp = TempDir::new().expect("tempdir");
    let root = tmp.path().join("spec");
    let docs = root.join("docs");
    fs::create_dir_all(&docs).expect("create docs");

    // Without --dir and without a default docs dir in the CWD, `new` should fail.
    let fail_infer = run_ard(&["new", "use-case", "--id", "UC-0002", "--title", "X"]);
    assert!(!fail_infer.status.success());

    let out_uc = run_ard(&[
        "new",
        "use-case",
        "--id",
        "UC-0002",
        "--title",
        "My Use Case",
        "--dir",
        &docs.to_string_lossy(),
    ]);
    assert!(out_uc.status.success());

    let out_adr = run_ard(&[
        "new",
        "adr",
        "--id",
        "ADR-0001",
        "--title",
        "My ADR",
        "--dir",
        &docs.to_string_lossy(),
    ]);
    assert!(out_adr.status.success());

    let out_nfr = run_ard(&[
        "new",
        "nfr",
        "--id",
        "NFR-0002",
        "--title",
        "NFR Baseline",
        "--dir",
        &docs.to_string_lossy(),
    ]);
    assert!(out_nfr.status.success());

    let out_glossary = run_ard(&[
        "new",
        "glossary",
        "--id",
        "GLOSSARY-0002",
        "--title",
        "Glossary And Entities",
        "--dir",
        &docs.to_string_lossy(),
    ]);
    assert!(out_glossary.status.success());

    // Conformance core should succeed (may warn about missing examples, which is OK for core).
    let conf = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "core",
        &root.to_string_lossy(),
    ]);
    assert!(conf.status.success());

    // Conformance JSON output should include standard metadata.
    let conf_json = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "core",
        "--format",
        "json",
        &root.to_string_lossy(),
    ]);
    assert!(conf_json.status.success());
    let stdout = String::from_utf8_lossy(&conf_json.stdout);
    assert!(stdout.contains("\"standard\""));
    assert!(stdout.contains("\"ARSF\""));
}

#[test]
fn init_respects_overwrite_flag() {
    let tmp = TempDir::new().expect("tempdir");
    let spec_root = tmp.path().join("spec");

    let first = run_ard(&["init", "--root", &spec_root.to_string_lossy()]);
    assert!(first.status.success());

    // Re-running without --overwrite should fail due to existing seeded files.
    let second = run_ard(&["init", "--root", &spec_root.to_string_lossy()]);
    assert!(!second.status.success());
}

#[test]
fn new_infers_spec_docs_dir_and_refuses_overwrite() {
    let tmp = TempDir::new().expect("tempdir");
    let spec_docs = tmp.path().join("spec/docs");
    fs::create_dir_all(&spec_docs).expect("create spec/docs");

    let first = run_ard_in(
        tmp.path(),
        &["new", "use-case", "--id", "UC-0003", "--title", "Infer Dir"],
    );
    assert!(first.status.success());

    // Running again should attempt to write the same file and fail without an overwrite flag.
    let second = run_ard_in(
        tmp.path(),
        &["new", "use-case", "--id", "UC-0003", "--title", "Infer Dir"],
    );
    assert!(!second.status.success());

    let adr = run_ard_in(
        tmp.path(),
        &["new", "adr", "--id", "ADR-0002", "--title", "Infer ADR"],
    );
    assert!(adr.status.success());

    let nfr = run_ard_in(
        tmp.path(),
        &["new", "nfr", "--id", "NFR-0003", "--title", "Infer NFR"],
    );
    assert!(nfr.status.success());

    let glossary = run_ard_in(
        tmp.path(),
        &[
            "new",
            "glossary",
            "--id",
            "GLOSSARY-0003",
            "--title",
            "Infer Glossary",
        ],
    );
    assert!(glossary.status.success());
}

#[test]
fn conformance_github_format_emits_annotations() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let core_invalid = repo_root.join("schemas/arsf/0.1.0/vectors/core/invalid_missing_id");

    let out = run_ard(&[
        "conformance",
        "run",
        "--profile",
        "core",
        "--format",
        "github",
        &core_invalid.to_string_lossy(),
    ]);
    assert!(!out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("::error"), "{stdout}");
    assert!(stdout.contains("DS-ID-001"), "{stdout}");
}
