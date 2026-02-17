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
fn lint_skill_and_docset_subcommands_succeed() {
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
    let lint_docset = run_ard(&[
        "lint",
        "docset",
        &doc_root.to_string_lossy(),
        "--docs-dir",
        "docs",
        "--examples-dir",
        "examples",
    ]);
    assert!(lint_docset.status.success());
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
