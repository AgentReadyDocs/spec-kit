use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use include_dir::{include_dir, Dir};
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

static EMBED_SKILLS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../skills");
static EMBED_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../templates");
static EMBED_RUBRICS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../../rubrics");

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SkillTarget {
    Codex,
    Claude,
}

#[derive(Debug, Parser)]
#[command(name = "ard")]
#[command(about = "AgentReadyDocs CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Lint a path (auto-detect), or use an explicit linter subcommand.
    Lint(LintCmd),
    /// List/install embedded skills for agent tools.
    Skill(SkillCmd),
    /// List/print embedded templates.
    Template(AssetCmd),
    /// List/print embedded rubrics.
    Rubric(AssetCmd),
}

#[derive(Debug, Parser)]
struct LintCmd {
    /// Treat warnings as errors (where supported).
    #[arg(long)]
    strict: bool,
    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
    /// Optional explicit docset root (used when linting a single doc file).
    #[arg(long)]
    root: Option<PathBuf>,
    #[command(subcommand)]
    subcommand: Option<LintSubcommand>,
    /// Path to lint (when no explicit subcommand is provided).
    path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum LintSubcommand {
    /// Lint an AGENTS.md file.
    AgentsMd { path: PathBuf },
    /// Lint a docset directory containing docs/ and/or examples/.
    Docset {
        root: PathBuf,
        #[arg(long, default_value = "docs")]
        docs_dir: String,
        #[arg(long, default_value = "examples")]
        examples_dir: String,
        #[arg(long)]
        check_doc_placeholders: bool,
    },
    /// Lint a skill directory containing SKILL.md.
    Skill {
        skill_dir: PathBuf,
        #[arg(long, default_value_t = 500)]
        max_lines: usize,
    },
}

#[derive(Debug, Parser)]
struct SkillCmd {
    #[command(subcommand)]
    subcommand: SkillSubcommand,
}

#[derive(Debug, Subcommand)]
enum SkillSubcommand {
    /// List embedded skills.
    List,
    /// Install embedded skills into an agent tool's skills directory.
    Install {
        #[arg(long, value_enum)]
        target: Vec<SkillTarget>,
        /// Optional namespace subdir under skills/ to avoid collisions.
        #[arg(long)]
        namespace: Option<String>,
        /// Overwrite existing destination directories.
        #[arg(long)]
        overwrite: bool,
        /// Print what would be installed without writing.
        #[arg(long)]
        dry_run: bool,
        /// Override home directory for the target tool (repeatable; applies in order to targets).
        #[arg(long)]
        home: Option<PathBuf>,
        /// Install all skills (default).
        #[arg(long)]
        all: bool,
        /// Install only specific skills (repeatable).
        #[arg(long)]
        skill: Vec<String>,
    },
}

#[derive(Debug, Parser)]
struct AssetCmd {
    #[command(subcommand)]
    subcommand: AssetSubcommand,
}

#[derive(Debug, Subcommand)]
enum AssetSubcommand {
    /// List embedded assets.
    List,
    /// Print an embedded asset to stdout.
    Print { name: String },
}

#[derive(Debug, Clone, Serialize)]
struct Finding {
    check_id: String,
    severity: String, // "error" | "warning"
    message: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
struct LintResult {
    path: Option<String>,
    root: Option<String>,
    pass: bool,
    error_count: usize,
    warning_count: usize,
    errors: Vec<Finding>,
    warnings: Vec<Finding>,
}

fn main() {
    std::process::exit(main_exit_code());
}

fn main_exit_code() -> i32 {
    let cli = Cli::parse();
    match cli.command {
        Command::Lint(cmd) => run_lint(cmd),
        Command::Skill(cmd) => run_skill(cmd),
        Command::Template(cmd) => run_asset("template", &EMBED_TEMPLATES, cmd),
        Command::Rubric(cmd) => run_asset("rubric", &EMBED_RUBRICS, cmd),
    }
}

fn run_asset(kind: &str, dir: &Dir<'_>, cmd: AssetCmd) -> i32 {
    match cmd.subcommand {
        AssetSubcommand::List => {
            let mut names: Vec<String> = dir
                .files()
                .filter_map(|f| f.path().to_str().map(|s| s.to_string()))
                .filter(|p| p.ends_with(".md"))
                .collect();
            names.sort();
            for name in names {
                println!("{name}");
            }
            0
        }
        AssetSubcommand::Print { name } => {
            let path = Path::new(&name);
            match dir.get_file(path) {
                Some(file) => match file.contents_utf8() {
                    Some(text) => {
                        print!("{text}");
                        0
                    }
                    None => fail(&format!("{kind} is not valid UTF-8: {name}")),
                },
                None => fail(&format!("Unknown {kind}: {name}")),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SkillName(String);

impl SkillName {
    fn new(raw: String) -> Self {
        Self(raw)
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct SkillInstallTarget {
    tool: SkillTarget,
    home_dir: PathBuf,
}

impl SkillInstallTarget {
    fn tool_label(&self) -> &'static str {
        match self.tool {
            SkillTarget::Codex => "codex",
            SkillTarget::Claude => "claude",
        }
    }
}

#[derive(Debug, Clone)]
struct SkillInstallPlan {
    selected: Vec<SkillName>,
    targets: Vec<SkillInstallTarget>,
    namespace: Option<String>,
}

#[derive(Debug, Clone)]
struct SkillInstallArgs {
    target: Vec<SkillTarget>,
    namespace: Option<String>,
    overwrite: bool,
    dry_run: bool,
    home: Option<PathBuf>,
    all: bool,
    skill: Vec<String>,
}

#[derive(Debug, Clone)]
enum AutoLintTarget {
    AgentsFile { file: PathBuf },
    SkillDir { dir: PathBuf },
    DocFile { file: PathBuf, root: PathBuf },
    DocsetDir { root: PathBuf },
}

fn choose_skill_names(available: &[String], all: bool, requested: &[String]) -> Vec<SkillName> {
    if all || requested.is_empty() {
        return available.iter().cloned().map(SkillName::new).collect();
    }
    requested.iter().cloned().map(SkillName::new).collect()
}

fn validate_skill_names(selected: &[SkillName]) -> Result<(), String> {
    for name in selected {
        if !embedded_has_skill(name.as_str()) {
            return Err(format!("Unknown embedded skill: {}", name.as_str()));
        }
    }
    Ok(())
}

fn build_skill_targets(
    targets: &[SkillTarget],
    home: &Option<PathBuf>,
) -> Result<Vec<SkillInstallTarget>, String> {
    if targets.is_empty() {
        return Err(
            "No --target specified (use --target codex and/or --target claude).".to_string(),
        );
    }
    let mut built: Vec<SkillInstallTarget> = Vec::new();
    for target in targets {
        let home_dir = home.clone().unwrap_or_else(|| default_tool_home(*target));
        built.push(SkillInstallTarget {
            tool: *target,
            home_dir,
        });
    }
    Ok(built)
}

fn build_skill_install_plan(
    args: &SkillInstallArgs,
    available: &[String],
) -> Result<SkillInstallPlan, String> {
    let selected = choose_skill_names(available, args.all, &args.skill);
    validate_skill_names(&selected)?;
    let targets = build_skill_targets(&args.target, &args.home)?;
    Ok(SkillInstallPlan {
        selected,
        targets,
        namespace: args.namespace.clone(),
    })
}

fn skill_destination_path(
    target: &SkillInstallTarget,
    skill: &SkillName,
    namespace: &Option<String>,
) -> PathBuf {
    if let Some(ns) = namespace {
        target.home_dir.join("skills").join(ns).join(skill.as_str())
    } else {
        target.home_dir.join("skills").join(skill.as_str())
    }
}

fn install_skill_to_destination(
    skill: &SkillName,
    dest: &Path,
    overwrite: bool,
) -> Result<(), String> {
    if dest.exists() {
        if !overwrite {
            return Err(format!(
                "Destination already exists (use --overwrite): {}",
                dest.display()
            ));
        }
        fs::remove_dir_all(dest).map_err(|err| {
            format!(
                "Failed to remove existing destination {}: {err}",
                dest.display()
            )
        })?;
    }
    write_embedded_skill_to(skill.as_str(), dest)
        .map_err(|err| format!("Failed to install {}: {err}", skill.as_str()))
}

fn run_skill_install_target(
    target: &SkillInstallTarget,
    selected: &[SkillName],
    namespace: &Option<String>,
    overwrite: bool,
    dry_run: bool,
) -> Result<(), String> {
    for skill in selected {
        let dest = skill_destination_path(target, skill, namespace);
        if dry_run {
            println!(
                "[DRY-RUN] {} -> {}:{dest}",
                skill.as_str(),
                target.tool_label(),
                dest = dest.display()
            );
            continue;
        }
        install_skill_to_destination(skill, &dest, overwrite)?;
        println!(
            "[OK] Installed {} -> {}:{dest}",
            skill.as_str(),
            target.tool_label(),
            dest = dest.display()
        );
    }
    Ok(())
}

fn run_skill_install(args: SkillInstallArgs) -> i32 {
    let available = embedded_skill_names();
    let plan = match build_skill_install_plan(&args, &available) {
        Ok(plan) => plan,
        Err(message) => return fail(&message),
    };
    for target in &plan.targets {
        if let Err(message) = run_skill_install_target(
            target,
            &plan.selected,
            &plan.namespace,
            args.overwrite,
            args.dry_run,
        ) {
            return fail(&message);
        }
    }
    eprintln!("Note: restart your agent tool to refresh discovered skills.");
    0
}

fn run_skill(cmd: SkillCmd) -> i32 {
    match cmd.subcommand {
        SkillSubcommand::List => {
            for name in embedded_skill_names() {
                println!("{name}");
            }
            0
        }
        SkillSubcommand::Install {
            target,
            namespace,
            overwrite,
            dry_run,
            home,
            all,
            skill,
        } => run_skill_install(SkillInstallArgs {
            target,
            namespace,
            overwrite,
            dry_run,
            home,
            all,
            skill,
        }),
    }
}

fn run_lint(cmd: LintCmd) -> i32 {
    match cmd.subcommand {
        Some(LintSubcommand::AgentsMd { path }) => {
            let (errors, warnings) = lint_agents_md(&path, cmd.strict);
            emit_lint_result(
                LintResult {
                    path: Some(canonicalish(&path).to_string_lossy().to_string()),
                    root: None,
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                cmd.format,
            )
        }
        Some(LintSubcommand::Skill {
            skill_dir,
            max_lines,
        }) => {
            let (errors, warnings) = lint_skill(&skill_dir, max_lines);
            emit_lint_result(
                LintResult {
                    path: Some(canonicalish(&skill_dir).to_string_lossy().to_string()),
                    root: None,
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                cmd.format,
            )
        }
        Some(LintSubcommand::Docset {
            root,
            docs_dir,
            examples_dir,
            check_doc_placeholders,
        }) => {
            let (errors, warnings) = lint_docset(
                &root,
                &docs_dir,
                &examples_dir,
                check_doc_placeholders,
                cmd.strict,
            );
            emit_lint_result(
                LintResult {
                    path: None,
                    root: Some(canonicalish(&root).to_string_lossy().to_string()),
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                cmd.format,
            )
        }
        None => {
            let path = match cmd.path {
                Some(p) => p,
                None => return fail("Missing <path>. Try: ard lint ./AGENTS.md"),
            };
            lint_auto(path, cmd.strict, cmd.format, cmd.root)
        }
    }
}

fn auto_lint_target(path: &Path, root_override: Option<PathBuf>) -> Result<AutoLintTarget, String> {
    let canonical = canonicalish(path);
    if canonical.is_file() {
        let file_name = canonical.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if file_name == "AGENTS.md" {
            return Ok(AutoLintTarget::AgentsFile { file: canonical });
        }
        if file_name == "SKILL.md" {
            let dir = canonical.parent().unwrap_or(Path::new(".")).to_path_buf();
            return Ok(AutoLintTarget::SkillDir { dir });
        }
        if canonical.extension().and_then(OsStr::to_str).unwrap_or("") == "md" {
            let root = match root_override {
                Some(root) => root,
                None => infer_docset_root(canonical.parent().unwrap_or(Path::new("."))).ok_or_else(|| {
                    "Could not infer docset root for file; pass --root <dir> or lint a directory.".to_string()
                })?,
            };
            return Ok(AutoLintTarget::DocFile {
                file: canonical,
                root,
            });
        }
        return Err("Unsupported file type for auto linting.".to_string());
    }
    if canonical.is_dir() {
        if canonical.join("SKILL.md").is_file() {
            return Ok(AutoLintTarget::SkillDir { dir: canonical });
        }
        return Ok(AutoLintTarget::DocsetDir { root: canonical });
    }
    Err(format!("Path does not exist: {}", canonical.display()))
}

fn lint_auto(
    path: PathBuf,
    strict: bool,
    format: OutputFormat,
    root_override: Option<PathBuf>,
) -> i32 {
    let target = match auto_lint_target(&path, root_override) {
        Ok(target) => target,
        Err(message) => return fail(&message),
    };
    match target {
        AutoLintTarget::AgentsFile { file } => {
            let (errors, warnings) = lint_agents_md(&file, strict);
            emit_lint_result(
                LintResult {
                    path: Some(file.to_string_lossy().to_string()),
                    root: None,
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                format,
            )
        }
        AutoLintTarget::SkillDir { dir } => {
            let (errors, warnings) = lint_skill(&dir, 500);
            emit_lint_result(
                LintResult {
                    path: Some(dir.to_string_lossy().to_string()),
                    root: None,
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                format,
            )
        }
        AutoLintTarget::DocFile { file, root } => {
            let (errors, warnings) = lint_docset(&root, "docs", "examples", false, strict);
            emit_lint_result(
                LintResult {
                    path: Some(file.to_string_lossy().to_string()),
                    root: Some(canonicalish(&root).to_string_lossy().to_string()),
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                format,
            )
        }
        AutoLintTarget::DocsetDir { root } => {
            let (errors, warnings) = lint_docset(&root, "docs", "examples", false, strict);
            emit_lint_result(
                LintResult {
                    path: None,
                    root: Some(root.to_string_lossy().to_string()),
                    pass: errors.is_empty(),
                    error_count: errors.len(),
                    warning_count: warnings.len(),
                    errors,
                    warnings,
                },
                format,
            )
        }
    }
}

fn infer_docset_root(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);
    while let Some(dir) = current {
        if dir.join(".git").is_dir() || dir.join("docs").is_dir() || dir.join("examples").is_dir() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }
    None
}

fn canonicalish(path: &Path) -> PathBuf {
    path.expand_tilde().unwrap_or_else(|| path.to_path_buf())
}

trait TildeExpand {
    fn expand_tilde(&self) -> Option<PathBuf>;
}

impl TildeExpand for Path {
    fn expand_tilde(&self) -> Option<PathBuf> {
        let s = self.to_string_lossy();
        if !s.starts_with("~") {
            return None;
        }
        let home = home_dir()?;
        if s == "~" {
            return Some(home);
        }
        if let Some(rest) = s.strip_prefix("~/") {
            return Some(home.join(rest));
        }
        None
    }
}

fn home_dir() -> Option<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        if !home.trim().is_empty() {
            return Some(PathBuf::from(home));
        }
    }
    if let Ok(profile) = env::var("USERPROFILE") {
        if !profile.trim().is_empty() {
            return Some(PathBuf::from(profile));
        }
    }
    None
}

fn default_tool_home(target: SkillTarget) -> PathBuf {
    match target {
        SkillTarget::Codex => env::var("CODEX_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".codex")))
            .unwrap_or_else(|| PathBuf::from(".codex")),
        SkillTarget::Claude => env::var("CLAUDE_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| home_dir().map(|h| h.join(".claude")))
            .unwrap_or_else(|| PathBuf::from(".claude")),
    }
}

fn embedded_skill_names() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for entry in EMBED_SKILLS.dirs() {
        let name = entry
            .path()
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        if name.is_empty() || name == "linters" {
            continue;
        }
        if entry.get_file("SKILL.md").is_some() {
            names.push(name.to_string());
        }
    }
    names.sort();
    names
}

fn embedded_has_skill(name: &str) -> bool {
    if name == "linters" {
        return false;
    }
    EMBED_SKILLS
        .get_dir(name)
        .and_then(|d| d.get_file("SKILL.md"))
        .is_some()
}

fn write_embedded_skill_to(name: &str, dest: &Path) -> io::Result<()> {
    let src_dir = EMBED_SKILLS
        .get_dir(name)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "skill not found"))?;

    write_embedded_subdir(src_dir, dest)
}

fn write_embedded_subdir(dir: &Dir<'_>, dest: &Path) -> io::Result<()> {
    for file in dir.files() {
        let rel = file.path();
        let out = dest.join(rel);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(out, file.contents())?;
    }
    for child in dir.dirs() {
        write_embedded_subdir(child, dest)?;
    }
    Ok(())
}

fn emit_lint_result(result: LintResult, format: OutputFormat) -> i32 {
    match format {
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string());
            println!("{json}");
        }
        OutputFormat::Text => {
            if result.errors.is_empty() && result.warnings.is_empty() {
                println!("[OK] Passed lint checks.");
            } else {
                if !result.errors.is_empty() {
                    println!("[FAIL] Errors:");
                    for f in &result.errors {
                        println!("- {}: {} ({})", f.check_id, f.message, f.evidence);
                    }
                }
                if !result.warnings.is_empty() {
                    println!("[WARN] Warnings:");
                    for f in &result.warnings {
                        println!("- {}: {} ({})", f.check_id, f.message, f.evidence);
                    }
                }
            }
        }
    }
    if result.errors.is_empty() {
        0
    } else {
        1
    }
}

fn fail(message: &str) -> i32 {
    let _ = writeln!(io::stderr(), "[FAIL] {message}");
    1
}

// ---- agents-md linter (ported from src/spec_kit_linters/agents_md_linter.py) ----

const VAGUE_PHRASES: [&str; 5] = [
    "as appropriate",
    "follow conventions",
    "use standard",
    "when necessary",
    "see docs",
];

fn extract_section(text: &str, section_name: &str) -> String {
    extract_md_h2_section(text, section_name)
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn extract_md_h2_section<'a>(text: &'a str, section_name: &str) -> Option<&'a str> {
    let want = section_name.trim().to_lowercase();
    let mut offset: usize = 0;
    let mut in_section = false;
    let mut content_start: usize = 0;

    for line in text.split_inclusive('\n') {
        let line_stripped = line.strip_suffix('\n').unwrap_or(line);

        if !in_section {
            if let Some(title) = line_stripped.strip_prefix("##") {
                let title = title.trim();
                if !title.is_empty() && title.to_lowercase() == want {
                    in_section = true;
                    content_start = offset + line.len();
                }
            }
        } else {
            if let Some(rest) = line_stripped.strip_prefix("##") {
                if rest.starts_with(char::is_whitespace) {
                    let end = offset;
                    return Some(&text[content_start..end]);
                }
            }
        }

        offset += line.len();
    }

    if in_section {
        return Some(&text[content_start..]);
    }
    None
}

fn markdown_links(text: &str) -> Vec<String> {
    let re = Regex::new(r"\]\(([^)]+)\)").unwrap();
    let mut targets = Vec::new();
    for cap in re.captures_iter(text) {
        let mut target = cap.get(1).unwrap().as_str().trim().to_string();
        if target.is_empty() {
            continue;
        }
        if target.contains("://") || target.starts_with("mailto:") {
            continue;
        }
        if let Some((base, _)) = target.split_once('#') {
            target = base.trim().to_string();
        }
        if !target.is_empty() {
            targets.push(target);
        }
    }
    targets
}

fn has_must_rule_for(topic: &str, block: &str) -> bool {
    let re = Regex::new(&format!(r"(?im)\bMUST\b.*\b{}\b", regex::escape(topic))).unwrap();
    re.is_match(block)
}

fn has_on_fail_for(topic: &str, text: &str) -> bool {
    let re = Regex::new(&format!(r"(?im)ON FAIL.*\b{}\b", regex::escape(topic))).unwrap();
    re.is_match(text)
}

fn has_essential_command(kind: &str, text: &str) -> bool {
    let re = Regex::new(&format!(
        r"(?im)^\s*(?:-|\*|\d+\.)?\s*`?[^`\n]*\b{}\b[^`\n]*`?\s*$",
        regex::escape(kind)
    ))
    .unwrap();
    re.is_match(text)
}

fn lint_agents_md(path: &Path, strict: bool) -> (Vec<Finding>, Vec<Finding>) {
    let mut errors: Vec<Finding> = Vec::new();
    let mut warnings: Vec<Finding> = Vec::new();

    if !path.exists() {
        errors.push(Finding {
            check_id: "AG001".to_string(),
            severity: "error".to_string(),
            message: "AGENTS.md file does not exist.".to_string(),
            evidence: path.display().to_string(),
        });
        return (errors, warnings);
    }
    if !path.is_file() {
        errors.push(Finding {
            check_id: "AG001".to_string(),
            severity: "error".to_string(),
            message: "AGENTS.md path is not a file.".to_string(),
            evidence: path.display().to_string(),
        });
        return (errors, warnings);
    }

    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(err) => {
            errors.push(Finding {
                check_id: "AG001".to_string(),
                severity: "error".to_string(),
                message: format!("Failed to read AGENTS.md: {err}"),
                evidence: path.display().to_string(),
            });
            return (errors, warnings);
        }
    };

    let critical = extract_section(&text, "CRITICAL");
    let commands = extract_section(&text, "Commands");

    if critical.is_empty() {
        errors.push(Finding {
            check_id: "AG002".to_string(),
            severity: "error".to_string(),
            message: "Missing required `## CRITICAL` section.".to_string(),
            evidence: "header: ## CRITICAL".to_string(),
        });
    }
    if commands.is_empty() {
        errors.push(Finding {
            check_id: "AG003".to_string(),
            severity: "error".to_string(),
            message: "Missing required `## Commands` section.".to_string(),
            evidence: "header: ## Commands".to_string(),
        });
    }

    if !critical.is_empty() {
        let never_topic_patterns: BTreeMap<&str, Regex> = BTreeMap::from([
            (
                "force push",
                Regex::new(r"(?im)\bNEVER\b.*\bforce\s+push\b").unwrap(),
            ),
            (
                "hook",
                Regex::new(r"(?im)\bNEVER\b.*\bhook(?:s)?\b").unwrap(),
            ),
            (
                "secret",
                Regex::new(r"(?im)\bNEVER\b.*\bsecret(?:s)?\b").unwrap(),
            ),
            (
                "generated",
                Regex::new(r"(?im)\bNEVER\b.*\bgenerated\b").unwrap(),
            ),
        ]);

        let mut missing: Vec<&str> = Vec::new();
        for (topic, re) in &never_topic_patterns {
            if !re.is_match(&critical) {
                missing.push(topic);
            }
        }
        if !missing.is_empty() {
            errors.push(Finding {
                check_id: "AG004".to_string(),
                severity: "error".to_string(),
                message: "CRITICAL section is missing expected NEVER guardrails.".to_string(),
                evidence: format!("missing topics: {}", missing.join(", ")),
            });
        }

        for topic in ["lint", "test"] {
            if !has_on_fail_for(topic, &critical) && !has_on_fail_for(topic, &text) {
                errors.push(Finding {
                    check_id: "AG005".to_string(),
                    severity: "error".to_string(),
                    message: format!("Missing ON FAIL recovery guidance for `{topic}`."),
                    evidence: "expected: ON FAIL ...".to_string(),
                });
            }
        }

        for topic in ["package", "lint", "test"] {
            if !has_must_rule_for(topic, &critical) {
                errors.push(Finding {
                    check_id: "AG006".to_string(),
                    severity: "error".to_string(),
                    message: format!("Missing MUST rule for `{topic}` in CRITICAL."),
                    evidence: "expected: MUST ...".to_string(),
                });
            }
        }
    }

    let version_re = Regex::new(r"(?im)^agents-md-version\s*:\s*[0-9]+(?:\.[0-9]+)*\s*$").unwrap();
    if !version_re.is_match(&text) {
        warnings.push(Finding {
            check_id: "AG007".to_string(),
            severity: "warning".to_string(),
            message: "Missing `agents-md-version` tag.".to_string(),
            evidence: "expected a top-level metadata line".to_string(),
        });
    }

    for command in ["install", "lint", "test"] {
        if !has_essential_command(command, &text) {
            errors.push(Finding {
                check_id: "AG008".to_string(),
                severity: "error".to_string(),
                message: format!("Missing essential `{command}` command."),
                evidence: "search in Commands/CRITICAL".to_string(),
            });
        }
    }

    let todo_re = Regex::new(r"(?im)\[(?:TODO|TBD)[^\]]*\]|^\s*(?:TODO|TBD)\s*:").unwrap();
    if todo_re.is_match(&text) {
        errors.push(Finding {
            check_id: "AG009".to_string(),
            severity: "error".to_string(),
            message: "Contains TODO/TBD placeholders.".to_string(),
            evidence: "placeholder token present".to_string(),
        });
    }

    for target in markdown_links(&text) {
        if target.starts_with('/') {
            continue;
        }
        let resolved = path.parent().unwrap_or(Path::new(".")).join(&target);
        if !resolved.exists() {
            errors.push(Finding {
                check_id: "AG010".to_string(),
                severity: "error".to_string(),
                message: "Broken local markdown link.".to_string(),
                evidence: target,
            });
        }
    }

    for phrase in VAGUE_PHRASES {
        let re = Regex::new(&format!(r"(?im)\b{}\b", regex::escape(phrase))).unwrap();
        if re.is_match(&text) {
            let finding = Finding {
                check_id: "AG011".to_string(),
                severity: "warning".to_string(),
                message: "Contains vague directive phrase.".to_string(),
                evidence: phrase.to_string(),
            };
            if strict {
                errors.push(Finding {
                    check_id: finding.check_id.clone(),
                    severity: "error".to_string(),
                    message: finding.message.clone(),
                    evidence: finding.evidence.clone(),
                });
            } else {
                warnings.push(finding);
            }
        }
    }

    (errors, warnings)
}

// ---- skill linter (ported from src/spec_kit_linters/skill_linter.py) ----

const ALLOWED_FRONTMATTER_KEYS: [&str; 6] = [
    "name",
    "description",
    "license",
    "compatibility",
    "metadata",
    "allowed-tools",
];

#[derive(Debug, Clone)]
struct Frontmatter {
    name: String,
    description: String,
}

fn read_frontmatter_block(skill_md_text: &str) -> Result<&str, String> {
    if !skill_md_text.starts_with("---\n") {
        return Err("SKILL.md must start with YAML frontmatter (---).".to_string());
    }
    let end = skill_md_text[4..].find("\n---\n").map(|i| i + 4);
    let Some(end) = end else {
        return Err("SKILL.md frontmatter is missing closing --- delimiter.".to_string());
    };
    Ok(&skill_md_text[4..end])
}

fn parse_frontmatter(frontmatter_text: &str) -> Result<Frontmatter, String> {
    let mut keys: BTreeSet<String> = BTreeSet::new();
    let mut name: Option<String> = None;
    let mut description: Option<String> = None;

    let lines: Vec<&str> = frontmatter_text.lines().collect();
    let mut index: usize = 0;
    let key_re = Regex::new(r"^([A-Za-z0-9_-]+):(?:\s*(.*))?$").unwrap();

    while index < lines.len() {
        let line = lines[index];
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            index += 1;
            continue;
        }

        let caps = key_re
            .captures(line)
            .ok_or_else(|| format!("Invalid frontmatter line: {line:?}"))?;
        let key = caps.get(1).unwrap().as_str().to_string();
        let rest = caps
            .get(2)
            .map(|m| m.as_str())
            .unwrap_or("")
            .trim_end()
            .to_string();
        keys.insert(key.clone());

        if key == "description" && matches!(rest.trim(), ">" | "|" | ">-" | "|-") {
            let indicator = rest.trim().to_string();
            index += 1;
            let mut block_lines: Vec<String> = Vec::new();
            let mut indent: Option<usize> = None;
            while index < lines.len() {
                let raw = lines[index];
                if raw.trim().is_empty() {
                    block_lines.push("".to_string());
                    index += 1;
                    continue;
                }
                let leading = raw.chars().take_while(|c| *c == ' ').count();
                if indent.is_none() {
                    indent = Some(leading);
                }
                if leading < indent.unwrap_or(0) {
                    break;
                }
                let cut = indent.unwrap_or(0).min(raw.len());
                block_lines.push(raw[cut..].to_string());
                index += 1;
            }

            if indicator.starts_with('>') {
                let mut folded: Vec<String> = Vec::new();
                let mut paragraph: Vec<String> = Vec::new();
                for bl in block_lines {
                    if bl.is_empty() {
                        if !paragraph.is_empty() {
                            folded.push(paragraph.join(" ").trim().to_string());
                            paragraph.clear();
                        }
                        folded.push("".to_string());
                    } else {
                        paragraph.push(bl.trim_end().to_string());
                    }
                }
                if !paragraph.is_empty() {
                    folded.push(paragraph.join(" ").trim().to_string());
                }
                description = Some(folded.join("\n").trim().to_string());
            } else {
                description = Some(block_lines.join("\n").trim().to_string());
            }
            continue;
        }

        if key == "name" {
            let v = rest
                .trim()
                .trim_matches(&['"', '\''][..])
                .trim()
                .to_string();
            if !v.is_empty() {
                name = Some(v);
            }
        } else if key == "description" {
            let v = rest
                .trim()
                .trim_matches(&['"', '\''][..])
                .trim()
                .to_string();
            if !v.is_empty() {
                description = Some(v);
            }
        }

        index += 1;
    }

    let Some(name) = name else {
        return Err("Missing required frontmatter key: name".to_string());
    };
    let Some(description) = description else {
        return Err("Missing required frontmatter key: description".to_string());
    };

    let allowed: BTreeSet<&str> = ALLOWED_FRONTMATTER_KEYS.into_iter().collect();
    let unexpected: Vec<String> = keys
        .iter()
        .filter(|k| !allowed.contains(k.as_str()))
        .cloned()
        .collect();
    if !unexpected.is_empty() {
        let mut allowed_list: Vec<&str> = allowed.into_iter().collect();
        allowed_list.sort();
        return Err(format!(
            "Unexpected frontmatter key(s): {}. Allowed: {}",
            unexpected.join(", "),
            allowed_list.join(", ")
        ));
    }

    Ok(Frontmatter {
        name: name.trim().to_string(),
        description: description.trim().to_string(),
    })
}

fn validate_name(name: &str) -> Option<String> {
    if name.is_empty() {
        return Some("Frontmatter name is empty.".to_string());
    }
    if name.len() > 64 {
        return Some(format!("Frontmatter name too long ({} > 64).", name.len()));
    }
    let re = Regex::new(r"^[a-z0-9-]+$").unwrap();
    if !re.is_match(name) {
        return Some("Frontmatter name must match ^[a-z0-9-]+$.".to_string());
    }
    if name.starts_with('-') || name.ends_with('-') || name.contains("--") {
        return Some("Frontmatter name cannot start/end with '-' or contain '--'.".to_string());
    }
    None
}

fn validate_description(description: &str) -> Option<String> {
    if description.trim().is_empty() {
        return Some("Frontmatter description is empty.".to_string());
    }
    if description.len() > 1024 {
        return Some(format!(
            "Frontmatter description too long ({} > 1024).",
            description.len()
        ));
    }
    if description.contains('<') || description.contains('>') {
        return Some("Frontmatter description cannot contain '<' or '>'.".to_string());
    }
    None
}

fn extract_local_markdown_links(text: &str) -> BTreeSet<String> {
    let re = Regex::new(r"\]\(([^)]+)\)").unwrap();
    let mut links = BTreeSet::new();
    for cap in re.captures_iter(text) {
        let mut target = cap.get(1).unwrap().as_str().trim().to_string();
        if target.is_empty() {
            continue;
        }
        if target.contains("://") || target.starts_with("mailto:") {
            continue;
        }
        if let Some((base, _)) = target.split_once('#') {
            target = base.trim().to_string();
        }
        if target.is_empty() || target.starts_with('/') {
            continue;
        }
        links.insert(target);
    }
    links
}

fn is_probably_markdown(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(OsStr::to_str)
            .unwrap_or("")
            .to_lowercase()
            .as_str(),
        "md" | "markdown"
    )
}

fn lint_skill(skill_dir: &Path, max_lines: usize) -> (Vec<Finding>, Vec<Finding>) {
    let mut errors: Vec<Finding> = Vec::new();
    let warnings: Vec<Finding> = Vec::new();

    let skill_dir = canonicalish(skill_dir);
    if !skill_dir.is_dir() {
        errors.push(Finding {
            check_id: "SK001".to_string(),
            severity: "error".to_string(),
            message: format!("Not a directory: {}", skill_dir.display()),
            evidence: skill_dir.display().to_string(),
        });
        return (errors, warnings);
    }

    let skill_md = skill_dir.join("SKILL.md");
    if !skill_md.exists() {
        errors.push(Finding {
            check_id: "SK002".to_string(),
            severity: "error".to_string(),
            message: format!("Missing SKILL.md: {}", skill_md.display()),
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }

    let text = match fs::read_to_string(&skill_md) {
        Ok(t) => t,
        Err(err) => {
            errors.push(Finding {
                check_id: "SK003".to_string(),
                severity: "error".to_string(),
                message: format!("Failed to read SKILL.md: {err}"),
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
    };

    let line_count = text.lines().count();
    if line_count > max_lines {
        errors.push(Finding {
            check_id: "SK004".to_string(),
            severity: "error".to_string(),
            message: format!("SKILL.md too long: {line_count} lines (max {max_lines})."),
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }

    let placeholder_res = [
        Regex::new(r"\[TODO[^\]]*\]").unwrap(),
        Regex::new(r"\[TBD[^\]]*\]").unwrap(),
        Regex::new(r"(?m)^\s*(TODO|TBD)\s*:").unwrap(),
    ];
    if placeholder_res.iter().any(|r| r.is_match(&text)) {
        errors.push(Finding {
            check_id: "SK005".to_string(),
            severity: "error".to_string(),
            message: "SKILL.md contains TODO/TBD placeholders (e.g., [TODO] or TODO:).".to_string(),
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }

    let frontmatter_block = match read_frontmatter_block(&text) {
        Ok(b) => b,
        Err(msg) => {
            errors.push(Finding {
                check_id: "SK006".to_string(),
                severity: "error".to_string(),
                message: msg,
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
    };
    let frontmatter = match parse_frontmatter(frontmatter_block) {
        Ok(fm) => fm,
        Err(msg) => {
            errors.push(Finding {
                check_id: "SK006".to_string(),
                severity: "error".to_string(),
                message: msg,
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
    };

    if let Some(msg) = validate_name(&frontmatter.name) {
        errors.push(Finding {
            check_id: "SK007".to_string(),
            severity: "error".to_string(),
            message: msg,
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }
    if let Some(msg) = validate_description(&frontmatter.description) {
        errors.push(Finding {
            check_id: "SK008".to_string(),
            severity: "error".to_string(),
            message: msg,
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }

    let dir_name = skill_dir.file_name().and_then(|s| s.to_str()).unwrap_or("");
    if frontmatter.name != dir_name {
        errors.push(Finding {
            check_id: "SK009".to_string(),
            severity: "error".to_string(),
            message: format!(
                "Frontmatter name '{}' must match directory name '{}'.",
                frontmatter.name, dir_name
            ),
            evidence: skill_md.display().to_string(),
        });
        return (errors, warnings);
    }

    let links = extract_local_markdown_links(&text);
    let mut referenced_md_files: Vec<PathBuf> = Vec::new();
    for link in &links {
        let resolved = normalize_join(&skill_dir, link);
        if !resolved.starts_with(&skill_dir) {
            errors.push(Finding {
                check_id: "SK010".to_string(),
                severity: "error".to_string(),
                message: format!("SKILL.md links outside skill dir: {link}"),
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
        if !resolved.exists() {
            errors.push(Finding {
                check_id: "SK011".to_string(),
                severity: "error".to_string(),
                message: format!("Broken link target in SKILL.md: {link}"),
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
        if resolved.is_file() && is_probably_markdown(&resolved) {
            referenced_md_files.push(resolved);
        }
    }

    for ref_md in referenced_md_files {
        let ref_text = match fs::read_to_string(&ref_md) {
            Ok(t) => t,
            Err(_) => continue,
        };
        let ref_links = extract_local_markdown_links(&ref_text);
        let mut chained: Vec<String> = Vec::new();
        for ref_link in ref_links {
            let candidate = normalize_join(ref_md.parent().unwrap_or(&skill_dir), &ref_link);
            if candidate.is_file() && is_probably_markdown(&candidate) {
                chained.push(ref_link);
            }
        }
        if !chained.is_empty() {
            errors.push(Finding {
                check_id: "SK012".to_string(),
                severity: "error".to_string(),
                message: format!(
                    "Deep reference chain: {} links to {}. List all required references directly in SKILL.md instead.",
                    ref_md.strip_prefix(&skill_dir).unwrap_or(&ref_md).display(),
                    chained.join(", ")
                ),
                evidence: skill_md.display().to_string(),
            });
            return (errors, warnings);
        }
    }

    let openai_yaml = skill_dir.join("agents").join("openai.yaml");
    if openai_yaml.exists() {
        if let Ok(contents) = fs::read_to_string(&openai_yaml) {
            let required = [
                "interface:",
                "display_name:",
                "short_description:",
                "default_prompt:",
            ];
            let missing: Vec<&str> = required
                .iter()
                .copied()
                .filter(|k| !contents.contains(k))
                .collect();
            if !missing.is_empty() {
                errors.push(Finding {
                    check_id: "SK013".to_string(),
                    severity: "error".to_string(),
                    message: format!(
                        "agents/openai.yaml missing required keys: {}",
                        missing.join(", ")
                    ),
                    evidence: openai_yaml.display().to_string(),
                });
                return (errors, warnings);
            }
        }
    }

    (errors, warnings)
}

fn normalize_join(base: &Path, target: &str) -> PathBuf {
    let mut out = base.to_path_buf();
    for comp in Path::new(target).components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                out.pop();
            }
            std::path::Component::Normal(p) => out.push(p),
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {}
        }
    }
    out
}

// ---- docset linter (ported from src/spec_kit_linters/docset_linter.py) ----

#[derive(Debug, Clone)]
struct Doc {
    path: PathBuf,
    doc_id: String,
    doc_type: Option<String>,
    risk_tier: Option<String>,
    links: BTreeMap<String, YamlValue>,
    text: String,
}

#[derive(Debug, Clone)]
enum YamlValue {
    Str(String),
    Bool,
    List(Vec<YamlValue>),
    Map(BTreeMap<String, YamlValue>),
}

fn extract_frontmatter_block(text: &str) -> Option<&str> {
    if !text.starts_with("---\n") {
        return None;
    }
    let end = text[4..].find("\n---\n").map(|i| i + 4)?;
    Some(&text[4..end])
}

fn parse_yaml_scalar(value: &str) -> YamlValue {
    let v = value.trim();
    if v.is_empty() {
        return YamlValue::Str("".to_string());
    }
    if (v.starts_with('"') && v.ends_with('"')) || (v.starts_with('\'') && v.ends_with('\'')) {
        return YamlValue::Str(v[1..v.len() - 1].to_string());
    }
    if v.eq_ignore_ascii_case("true") {
        return YamlValue::Bool;
    }
    if v.eq_ignore_ascii_case("false") {
        return YamlValue::Bool;
    }
    if v == "[]" || v == "[ ]" {
        return YamlValue::List(vec![]);
    }
    YamlValue::Str(v.to_string())
}

fn parse_yaml_subset(frontmatter_block: &str) -> Result<BTreeMap<String, YamlValue>, String> {
    let mut root: BTreeMap<String, YamlValue> = BTreeMap::new();
    #[derive(Debug, Clone)]
    enum ContainerKind {
        Map,
        List,
    }
    let mut stack: Vec<(usize, Vec<String>, ContainerKind)> = vec![(0, vec![], ContainerKind::Map)];

    let key_re = Regex::new(r"^([A-Za-z0-9_-]+):(.*)$").unwrap();

    for raw_line in frontmatter_block.lines() {
        if raw_line.trim().is_empty() {
            continue;
        }
        if raw_line.trim_start().starts_with('#') {
            continue;
        }

        let indent = raw_line.chars().take_while(|c| *c == ' ').count();
        let line = raw_line.trim();

        while stack.len() > 1 && indent < stack.last().unwrap().0 {
            stack.pop();
        }

        if line.starts_with("- ") {
            let item = parse_yaml_scalar(&line[2..]);
            match stack.last().unwrap().2 {
                ContainerKind::List => {
                    let parent_key_path = stack.last().unwrap().1.clone();
                    push_to_list(&mut root, &parent_key_path, item)?;
                }
                ContainerKind::Map => {
                    return Err("Invalid YAML subset: list item outside list context.".to_string());
                }
            }
            continue;
        }

        let caps = key_re
            .captures(line)
            .ok_or_else(|| format!("Invalid YAML subset line: {raw_line:?}"))?;
        let key = caps.get(1).unwrap().as_str().to_string();
        let rest = caps.get(2).unwrap().as_str().trim_start().to_string();

        let (parent_path, kind) = {
            let top = stack.last().unwrap();
            (top.1.clone(), top.2.clone())
        };
        if matches!(kind, ContainerKind::List) {
            return Err("Invalid YAML subset: mapping key under list context.".to_string());
        }

        if rest.is_empty() {
            // Start nested map
            set_in_map(
                &mut root,
                &parent_path,
                &key,
                YamlValue::Map(BTreeMap::new()),
            )?;
            let mut child_path = parent_path.clone();
            child_path.push(key);
            stack.push((indent + 1, child_path, ContainerKind::Map));
            continue;
        }
        if rest == "[]" {
            set_in_map(&mut root, &parent_path, &key, YamlValue::List(vec![]))?;
            let mut child_path = parent_path.clone();
            child_path.push(key);
            stack.push((indent + 1, child_path, ContainerKind::List));
            continue;
        }

        set_in_map(&mut root, &parent_path, &key, parse_yaml_scalar(&rest))?;
    }

    // Post-process: treat "links: []" as links: {} for safety
    if matches!(root.get("links"), Some(YamlValue::List(_))) {
        root.insert("links".to_string(), YamlValue::Map(BTreeMap::new()));
    }
    // Normalize: if links.use_cases exists but isn't a list, coerce to list
    if let Some(YamlValue::Map(links)) = root.get_mut("links") {
        if let Some(v) = links.get("use_cases").cloned() {
            if !matches!(v, YamlValue::List(_)) {
                links.insert("use_cases".to_string(), YamlValue::List(vec![v]));
            }
        }
    }

    Ok(root)
}

fn set_in_map(
    root: &mut BTreeMap<String, YamlValue>,
    parent_path: &[String],
    key: &str,
    value: YamlValue,
) -> Result<(), String> {
    if parent_path.is_empty() {
        root.insert(key.to_string(), value);
        return Ok(());
    }
    let mut current: &mut YamlValue = root
        .entry(parent_path[0].clone())
        .or_insert_with(|| YamlValue::Map(BTreeMap::new()));
    for p in &parent_path[1..] {
        match current {
            YamlValue::Map(map) => {
                current = map
                    .entry(p.clone())
                    .or_insert_with(|| YamlValue::Map(BTreeMap::new()));
            }
            _ => return Err("Invalid YAML subset: expected mapping container.".to_string()),
        }
    }
    match current {
        YamlValue::Map(map) => {
            map.insert(key.to_string(), value);
            Ok(())
        }
        _ => Err("Invalid YAML subset: expected mapping container.".to_string()),
    }
}

fn push_to_list(
    root: &mut BTreeMap<String, YamlValue>,
    key_path: &[String],
    value: YamlValue,
) -> Result<(), String> {
    if key_path.is_empty() {
        return Err("Invalid YAML subset: list item without key path.".to_string());
    }
    let mut current: &mut YamlValue = root.entry(key_path[0].clone()).or_insert_with(|| {
        if key_path.len() == 1 {
            YamlValue::List(vec![])
        } else {
            YamlValue::Map(BTreeMap::new())
        }
    });
    for (idx, p) in key_path[1..].iter().enumerate() {
        let is_last = idx == key_path[1..].len() - 1;
        match current {
            YamlValue::Map(map) => {
                current = map.entry(p.clone()).or_insert_with(|| {
                    if is_last {
                        YamlValue::List(vec![])
                    } else {
                        YamlValue::Map(BTreeMap::new())
                    }
                });
            }
            YamlValue::List(_) => {
                return Err("Invalid YAML subset: list nested under list.".to_string())
            }
            _ => return Err("Invalid YAML subset: expected container.".to_string()),
        }
    }
    match current {
        YamlValue::List(list) => {
            list.push(value);
            Ok(())
        }
        _ => Err("Invalid YAML subset: expected list container.".to_string()),
    }
}

fn doc_markdown_links(text: &str) -> Vec<String> {
    let re = Regex::new(r"\]\(([^)]+)\)").unwrap();
    let mut targets: Vec<String> = Vec::new();
    for cap in re.captures_iter(text) {
        let mut target = cap.get(1).unwrap().as_str().trim().to_string();
        if target.is_empty() {
            continue;
        }
        if target.contains("://") || target.starts_with("mailto:") {
            continue;
        }
        if let Some((base, _)) = target.split_once('#') {
            target = base.trim().to_string();
        }
        if !target.is_empty() {
            targets.push(target);
        }
    }
    targets
}

fn extract_section_span(text: &str, section_name: &str) -> Option<(usize, usize)> {
    extract_md_h2_section_span(text, section_name)
}

fn extract_doc_section(text: &str, section_name: &str) -> String {
    let Some((s, e)) = extract_section_span(text, section_name) else {
        return "".to_string();
    };
    text[s..e].trim().to_string()
}

fn extract_md_h2_section_span(text: &str, section_name: &str) -> Option<(usize, usize)> {
    let want = section_name.trim().to_lowercase();
    let mut offset: usize = 0;
    let mut in_section = false;
    let mut content_start: usize = 0;

    for line in text.split_inclusive('\n') {
        let line_stripped = line.strip_suffix('\n').unwrap_or(line);

        if !in_section {
            if let Some(title) = line_stripped.strip_prefix("##") {
                let title = title.trim();
                if !title.is_empty() && title.to_lowercase() == want {
                    in_section = true;
                    content_start = offset + line.len();
                }
            }
        } else {
            if let Some(rest) = line_stripped.strip_prefix("##") {
                if rest.starts_with(char::is_whitespace) {
                    let end = offset;
                    return Some((content_start, end));
                }
            }
        }

        offset += line.len();
    }

    if in_section {
        return Some((content_start, text.len()));
    }
    None
}

fn parse_markdown_table(section_text: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let lines: Vec<&str> = section_text
        .lines()
        .map(|l| l.trim_end_matches('\n'))
        .collect();
    for i in 0..lines.len().saturating_sub(1) {
        let header_line = lines[i].trim();
        let sep_line = lines[i + 1].trim();
        if !header_line.contains('|') || !sep_line.contains('|') {
            continue;
        }
        let sep_re = Regex::new(r"\|\s*-{3,}").unwrap();
        if !sep_re.is_match(sep_line) {
            continue;
        }
        let split_row = |row: &str| -> Vec<String> {
            let mut r = row.trim();
            if r.starts_with('|') {
                r = &r[1..];
            }
            if r.ends_with('|') {
                r = &r[..r.len() - 1];
            }
            r.split('|').map(|c| c.trim().to_string()).collect()
        };
        let headers = split_row(header_line);
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut j = i + 2;
        while j < lines.len() {
            let raw = lines[j].trim();
            if raw.is_empty() || !raw.contains('|') {
                break;
            }
            rows.push(split_row(raw));
            j += 1;
        }
        return (headers, rows);
    }
    (vec![], vec![])
}

fn normalize_header(header: &str) -> String {
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    re.replace_all(&header.trim().to_lowercase(), "_")
        .trim_matches('_')
        .to_string()
}

fn table_column_index(headers: &[String], column_name: &str) -> Option<usize> {
    let want = normalize_header(column_name);
    for (idx, header) in headers.iter().enumerate() {
        if normalize_header(header) == want {
            return Some(idx);
        }
    }
    None
}

const EXAMPLE_PLACEHOLDER_PATTERNS: [(&str, &str); 18] = [
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
    (r"\bGLOSSARY-ENTITIES\b", "GLOSSARY-ENTITIES"),
];

fn scan_placeholders(
    path: &Path,
    text: &str,
    check_id: &str,
    severity: &str,
    allow_warnings_in_open_questions: bool,
) -> (Vec<Finding>, Vec<Finding>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let open_questions_span = extract_section_span(text, "Open Questions");

    for (pattern, token) in EXAMPLE_PLACEHOLDER_PATTERNS {
        let re = Regex::new(pattern).unwrap();
        for m in re.find_iter(text) {
            let start = m.start();
            let line_start = text[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let line_end = text[start..]
                .find('\n')
                .map(|i| start + i)
                .unwrap_or(text.len());
            let line = &text[line_start..line_end];

            let is_open_questions = open_questions_span
                .map(|(s, e)| s <= start && start <= e)
                .unwrap_or(false);
            let is_declared_open = line.contains("[OPEN]") || line.contains("[ASSUMPTION]");

            let finding = Finding {
                check_id: check_id.to_string(),
                severity: severity.to_string(),
                message: format!("Template placeholder token found: {token}"),
                evidence: format!(
                    "{}: {}",
                    path.display(),
                    line.trim().chars().take(200).collect::<String>()
                ),
            };

            if allow_warnings_in_open_questions && (is_open_questions || is_declared_open) {
                warnings.push(Finding {
                    check_id: finding.check_id.clone(),
                    severity: "warning".to_string(),
                    message: finding.message.clone(),
                    evidence: finding.evidence.clone(),
                });
            } else {
                errors.push(finding);
            }
        }
    }

    (errors, warnings)
}

fn is_tier2_or_higher(risk_tier: &Option<String>) -> bool {
    risk_tier
        .as_ref()
        .map(|r| matches!(r.trim().to_lowercase().as_str(), "tier2" | "tier3"))
        .unwrap_or(false)
}

fn looks_non_placeholder(value: &str) -> bool {
    let v = value.trim();
    if v.is_empty() || v == "-" || v.eq_ignore_ascii_case("n/a") {
        return false;
    }
    if v.contains('[') && v.contains(']') {
        return false;
    }
    if v.eq_ignore_ascii_case("tbd") || v.eq_ignore_ascii_case("todo") {
        return false;
    }
    true
}

fn table_has_non_placeholder_rows(section_text: &str, required_columns: &[&str]) -> bool {
    let (headers, rows) = parse_markdown_table(section_text);
    if headers.is_empty() || rows.is_empty() {
        return false;
    }
    let mut indices: Vec<usize> = Vec::new();
    for col in required_columns {
        let Some(idx) = table_column_index(&headers, col) else {
            return false;
        };
        indices.push(idx);
    }
    for row in rows {
        if indices
            .iter()
            .all(|i| *i < row.len() && looks_non_placeholder(&row[*i]))
        {
            return true;
        }
    }
    false
}

fn is_decisionful_uc(text: &str) -> bool {
    let authz_section = extract_doc_section(text, "AuthZ");
    if !authz_section.is_empty()
        && table_has_non_placeholder_rows(&authz_section, &["actor_id", "condition", "decision"])
    {
        return true;
    }
    let invariants_section = extract_doc_section(text, "Invariants And Policies");
    if !invariants_section.is_empty()
        && table_has_non_placeholder_rows(&invariants_section, &["invariant", "enforcement"])
    {
        return true;
    }
    let lowered = text.to_lowercase();
    let contract_keywords = [
        "schema",
        "contract",
        "breaking",
        "deprecat",
        "compat",
        "migration",
        "version",
        "protobuf",
        "openapi",
    ];
    if contract_keywords.iter().any(|k| lowered.contains(k)) {
        return true;
    }
    let money_keywords = [
        "payment", "charge", "billing", "invoice", "refund", "payout", "money", "usd", "$",
    ];
    if money_keywords.iter().any(|k| lowered.contains(k)) {
        return true;
    }
    false
}

fn resolve_local_path(base: &Path, target: &str) -> Option<PathBuf> {
    let mut t = target.trim().to_string();
    if t.is_empty() {
        return None;
    }
    if t.starts_with('/') {
        return None;
    }
    if t.contains("://") || t.starts_with("mailto:") {
        return None;
    }
    if let Some((base_part, _)) = t.split_once('#') {
        t = base_part.trim().to_string();
    }
    if t.is_empty() {
        return None;
    }
    Some(normalize_join(base, &t))
}

fn read_doc(path: &Path) -> Option<Doc> {
    let text = fs::read_to_string(path).ok()?;
    let block = extract_frontmatter_block(&text)?;
    let data = parse_yaml_subset(block).unwrap_or_default();

    let doc_id = match data.get("id") {
        Some(YamlValue::Str(s)) => s.trim().to_string(),
        _ => "".to_string(),
    };

    let doc_type = match data.get("type") {
        Some(YamlValue::Str(s)) => Some(s.trim().to_string()),
        _ => None,
    };
    let risk_tier = match data.get("risk_tier") {
        Some(YamlValue::Str(s)) => Some(s.trim().to_string()),
        _ => None,
    };
    let links = match data.get("links") {
        Some(YamlValue::Map(m)) => m.clone(),
        _ => BTreeMap::new(),
    };

    Some(Doc {
        path: path.to_path_buf(),
        doc_id,
        doc_type,
        risk_tier,
        links,
        text,
    })
}

fn lint_docset(
    root: &Path,
    docs_dir: &str,
    examples_dir: &str,
    check_doc_placeholders: bool,
    strict: bool,
) -> (Vec<Finding>, Vec<Finding>) {
    let mut errors: Vec<Finding> = Vec::new();
    let mut warnings: Vec<Finding> = Vec::new();

    let root = canonicalish(root);
    let docs_path = root.join(docs_dir);
    let examples_path = root.join(examples_dir);

    let mut markdown_files: Vec<PathBuf> = Vec::new();
    if docs_path.is_dir() {
        for entry in WalkDir::new(&docs_path) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if entry.file_type().is_file()
                && entry.path().extension().and_then(OsStr::to_str) == Some("md")
            {
                markdown_files.push(entry.path().to_path_buf());
            }
        }
    }
    if examples_path.is_dir() {
        for entry in WalkDir::new(&examples_path) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if entry.file_type().is_file()
                && entry.path().extension().and_then(OsStr::to_str) == Some("md")
            {
                markdown_files.push(entry.path().to_path_buf());
            }
        }
    }

    let mut docs: Vec<Doc> = Vec::new();
    let mut by_id: BTreeMap<String, Doc> = BTreeMap::new();
    let mut adr_docs: Vec<Doc> = Vec::new();
    let mut uc_docs: Vec<Doc> = Vec::new();
    let mut glossary_docs: Vec<Doc> = Vec::new();
    let mut nfr_docs: Vec<Doc> = Vec::new();

    markdown_files.sort();
    markdown_files.dedup();

    for path in markdown_files {
        let Some(doc) = read_doc(&path) else { continue };
        if doc.doc_id.trim().is_empty() {
            errors.push(Finding {
                check_id: "DS-ID-001".to_string(),
                severity: "error".to_string(),
                message: "Missing required frontmatter `id`.".to_string(),
                evidence: path.display().to_string(),
            });
            continue;
        }
        if by_id.contains_key(&doc.doc_id) {
            let other = by_id.get(&doc.doc_id).unwrap();
            errors.push(Finding {
                check_id: "DS-ID-002".to_string(),
                severity: "error".to_string(),
                message: format!("Duplicate document id: {}", doc.doc_id),
                evidence: format!("{} and {}", other.path.display(), doc.path.display()),
            });
            continue;
        }

        by_id.insert(doc.doc_id.clone(), doc.clone());
        if doc.doc_type.as_deref() == Some("use_case") {
            uc_docs.push(doc.clone());
        }
        if doc.doc_type.as_deref() == Some("glossary") {
            glossary_docs.push(doc.clone());
        }
        if doc.doc_type.as_deref() == Some("nfr") {
            nfr_docs.push(doc.clone());
        }
        if doc.doc_id.starts_with("ADR-") {
            adr_docs.push(doc.clone());
        }
        docs.push(doc);
    }

    // Placeholder scanning
    if examples_path.is_dir() {
        for doc in &docs {
            if doc.path.starts_with(&examples_path) {
                let (e, w) = scan_placeholders(&doc.path, &doc.text, "EX-CF-001", "error", false);
                errors.extend(e);
                warnings.extend(w);
            }
        }
    }

    if check_doc_placeholders && docs_path.is_dir() {
        for doc in &docs {
            if doc.path.starts_with(&docs_path) {
                let (e, w) =
                    scan_placeholders(&doc.path, &doc.text, "DS-DOC-PLACEHOLDERS", "error", true);
                errors.extend(e);
                warnings.extend(w);
            }
        }
    }

    // DS-CF-001 / DS-CF-002: UC links.glossary and links.nfr resolve
    for uc in &uc_docs {
        let glossary_link = uc.links.get("glossary");
        let nfr_link = uc.links.get("nfr");

        for (key, value, check_id) in [
            ("glossary", glossary_link, "DS-CF-001"),
            ("nfr", nfr_link, "DS-CF-002"),
        ] {
            let Some(YamlValue::Str(s)) = value else {
                errors.push(Finding {
                    check_id: check_id.to_string(),
                    severity: "error".to_string(),
                    message: format!("Use case missing required links.{key} (or explicit N/A)."),
                    evidence: uc.path.display().to_string(),
                });
                continue;
            };
            if s.trim().eq_ignore_ascii_case("n/a") {
                continue;
            }
            let resolved = resolve_local_path(uc.path.parent().unwrap_or(&root), s);
            if resolved.as_ref().map(|p| p.exists()).unwrap_or(false) == false {
                errors.push(Finding {
                    check_id: check_id.to_string(),
                    severity: "error".to_string(),
                    message: format!("Use case links.{key} does not resolve to an existing file."),
                    evidence: format!("{}: links.{key}={s:?}", uc.path.display()),
                });
            }
        }
    }

    // DS-S-001: internal markdown links resolve (lightweight)
    for doc in &docs {
        for target in doc_markdown_links(&doc.text) {
            let resolved = resolve_local_path(doc.path.parent().unwrap_or(&root), &target);
            if let Some(r) = resolved {
                if !r.exists() {
                    errors.push(Finding {
                        check_id: "DS-S-001".to_string(),
                        severity: "error".to_string(),
                        message: "Broken local markdown link.".to_string(),
                        evidence: format!("{}: {target}", doc.path.display()),
                    });
                }
            }
        }
    }

    // DS-CF-003: UC referenced entities exist in linked glossary
    for uc in &uc_docs {
        let glossary_link = match uc.links.get("glossary") {
            Some(YamlValue::Str(s)) => s,
            _ => continue,
        };
        if glossary_link.trim().is_empty() || glossary_link.trim().eq_ignore_ascii_case("n/a") {
            continue;
        }
        let Some(glossary_path) =
            resolve_local_path(uc.path.parent().unwrap_or(&root), glossary_link)
        else {
            continue;
        };
        if !glossary_path.exists() {
            continue;
        }
        let Some(glossary_doc) = read_doc(&glossary_path) else {
            continue;
        };

        let terms_section = extract_doc_section(&glossary_doc.text, "Terms");
        let (terms_headers, terms_rows) = parse_markdown_table(&terms_section);
        let terms_col = if !terms_headers.is_empty() {
            table_column_index(&terms_headers, "term")
        } else {
            None
        };
        let mut terms: BTreeSet<String> = BTreeSet::new();
        if let Some(col) = terms_col {
            for row in terms_rows {
                if col < row.len() && !row[col].trim().is_empty() {
                    terms.insert(row[col].trim().to_string());
                }
            }
        }

        let entities_section = extract_doc_section(&glossary_doc.text, "Entities");
        let (ent_headers, ent_rows) = parse_markdown_table(&entities_section);
        let ent_col = if !ent_headers.is_empty() {
            table_column_index(&ent_headers, "entity")
        } else {
            None
        };
        let mut entities: BTreeSet<String> = BTreeSet::new();
        if let Some(col) = ent_col {
            for row in ent_rows {
                if col < row.len() && !row[col].trim().is_empty() {
                    entities.insert(row[col].trim().to_string());
                }
            }
        }

        let uc_entities_section = extract_doc_section(&uc.text, "Entities (Referenced)");
        let (uc_ent_headers, uc_ent_rows) = parse_markdown_table(&uc_entities_section);
        let uc_ent_col = if !uc_ent_headers.is_empty() {
            table_column_index(&uc_ent_headers, "entity")
        } else {
            None
        };
        let Some(uc_ent_col) = uc_ent_col else {
            continue;
        };

        for row in uc_ent_rows {
            if uc_ent_col >= row.len() {
                continue;
            }
            let entity = row[uc_ent_col].trim().to_string();
            if entity.is_empty() || entity.contains('[') || entity.contains(']') {
                continue;
            }
            if !terms.contains(&entity) && !entities.contains(&entity) {
                errors.push(Finding {
                    check_id: "DS-CF-003".to_string(),
                    severity: "error".to_string(),
                    message: "Use case references an entity not present in linked glossary (as a Term or Entity)."
                        .to_string(),
                    evidence: format!(
                        "{}: entity={entity:?}, glossary={glossary_link:?}",
                        uc.path.display()
                    ),
                });
            }
        }
    }

    // DS-S-002: H1 begins with ID (lightweight)
    let h1_re = Regex::new(r"(?m)^#\s+(.+?)\s*$").unwrap();
    for doc in &docs {
        let Some(cap) = h1_re.captures(&doc.text) else {
            continue;
        };
        let h1 = cap.get(1).map(|m| m.as_str().trim()).unwrap_or("");
        if !(doc.doc_type.as_deref() == Some("use_case") || doc.doc_id.starts_with("ADR-")) {
            continue;
        }
        if !h1.starts_with(&doc.doc_id) {
            warnings.push(Finding {
                check_id: "DS-S-002".to_string(),
                severity: "warning".to_string(),
                message: "H1 header does not start with the frontmatter id.".to_string(),
                evidence: format!("{}: H1={h1:?}, id={:?}", doc.path.display(), doc.doc_id),
            });
        }
    }

    // DS-S-007 / DS-S-008: example presence checks (deterministic subset)
    if examples_path.is_dir() {
        let has_uc_example = docs.iter().any(|d| {
            d.path.starts_with(&examples_path) && d.doc_type.as_deref() == Some("use_case")
        });
        let has_glossary_example = docs.iter().any(|d| {
            d.path.starts_with(&examples_path) && d.doc_type.as_deref() == Some("glossary")
        });
        let has_nfr_example = docs
            .iter()
            .any(|d| d.path.starts_with(&examples_path) && d.doc_type.as_deref() == Some("nfr"));

        if !has_uc_example {
            warnings.push(Finding {
                check_id: "DS-S-007".to_string(),
                severity: "warning".to_string(),
                message: "No use case example found under examples/.".to_string(),
                evidence: examples_path.display().to_string(),
            });
        }
        if !has_glossary_example {
            warnings.push(Finding {
                check_id: "DS-S-007".to_string(),
                severity: "warning".to_string(),
                message: "No glossary/entities example found under examples/.".to_string(),
                evidence: examples_path.display().to_string(),
            });
        }

        let requires_nfr_example = docs.iter().any(|d| {
            d.path.starts_with(&examples_path)
                && d.doc_type.as_deref() == Some("use_case")
                && d.risk_tier
                    .as_ref()
                    .map(|t| {
                        matches!(
                            t.trim().to_lowercase().as_str(),
                            "tier1" | "tier2" | "tier3"
                        )
                    })
                    .unwrap_or(false)
        });
        if requires_nfr_example && !has_nfr_example {
            warnings.push(Finding {
                check_id: "DS-S-008".to_string(),
                severity: "warning".to_string(),
                message: "No NFR baseline example found under examples/ (recommended for tier1+)."
                    .to_string(),
                evidence: examples_path.display().to_string(),
            });
        }
    }

    // DS-CF-004: tier2+ decisionful UCs require ADR linking them
    let mut adr_uc_links: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();
    for adr in &adr_docs {
        let use_cases = match adr.links.get("use_cases") {
            Some(YamlValue::List(list)) => list,
            _ => continue,
        };
        for v in use_cases {
            if let YamlValue::Str(s) = v {
                let uc_id = s.trim();
                if !uc_id.is_empty() {
                    adr_uc_links
                        .entry(uc_id.to_string())
                        .or_default()
                        .push(adr.path.clone());
                }
            }
        }
    }
    for uc in &uc_docs {
        if !is_tier2_or_higher(&uc.risk_tier) {
            continue;
        }
        if !is_decisionful_uc(&uc.text) {
            continue;
        }
        if !adr_uc_links.contains_key(&uc.doc_id) {
            errors.push(Finding {
                check_id: "DS-CF-004".to_string(),
                severity: "error".to_string(),
                message: "Tier2+ use case appears decisionful but no ADR links it via frontmatter links.use_cases.".to_string(),
                evidence: uc.path.display().to_string(),
            });
        }
    }

    if strict && !warnings.is_empty() {
        for w in warnings {
            errors.push(Finding {
                check_id: w.check_id,
                severity: "error".to_string(),
                message: w.message,
                evidence: w.evidence,
            });
        }
        warnings = Vec::new();
    }

    // silence unused vars warnings
    let _ = (glossary_docs, nfr_docs);

    (errors, warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, text).unwrap();
    }

    #[test]
    fn agents_md_linter_passes_valid_file() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join("AGENTS.md");
        write(
            &agents,
            "# AGENTS.md\nagents-md-version: 1\n\n## CRITICAL\n\n- MUST: package manager\n- MUST: lint command before commit\n- MUST: test command before PR\n- NEVER: Force push (git push --force)\n- NEVER: Skip pre-commit hooks (--no-verify)\n- NEVER: Commit secrets\n- NEVER: Edit generated files (generated)\n- ON FAIL (lint): do x\n- ON FAIL (test): do y\n\n## Commands\n\n```bash\n# install\nx\n# lint\nx\n# test\nx\n```\n",
        );
        let (errors, _warnings) = lint_agents_md(&agents, false);
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn skill_linter_passes_minimal_skill() {
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join("example-skill");
        fs::create_dir_all(skill_dir.join("references")).unwrap();
        write(
            &skill_dir.join("SKILL.md"),
            "---\nname: example-skill\ndescription: Example skill\n---\n\n[Ref](references/ref.md)\n",
        );
        write(&skill_dir.join("references/ref.md"), "# Ref\n");
        let (errors, _warnings) = lint_skill(&skill_dir, 500);
        assert!(errors.is_empty(), "{errors:?}");
    }

    #[test]
    fn docset_linter_detects_duplicate_ids() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/a.md"),
            "---\nid: UC-0001\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: A\n",
        );
        write(
            &tmp.path().join("docs/b.md"),
            "---\nid: UC-0001\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0001: B\n",
        );
        let (errors, _warnings) = lint_docset(tmp.path(), "docs", "examples", false, false);
        assert!(
            errors.iter().any(|f| f.check_id == "DS-ID-002"),
            "{errors:?}"
        );
    }

    #[test]
    fn docset_linter_requires_adr_for_decisionful_tier2_uc() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/glossary.md"),
            "---\nid: GLOSSARY-001\ntype: glossary\n---\n\n# GLOSSARY-001: Glossary\n\n## Terms\n| term | definition (one line) | allowed_synonyms | banned_synonyms |\n|------|------------------------|------------------|-----------------|\n| Widget | A widget. | - | - |\n",
        );
        write(
            &tmp.path().join("docs/uc.md"),
            "---\nid: UC-0002\ntype: use_case\nrisk_tier: tier2\nlinks:\n  glossary: \"./glossary.md\"\n  nfr: N/A\n---\n\n# UC-0002: Decisionful\n\n## Interface Contract\n\n### AuthZ\n| rule_id | actor_id | condition | decision |\n|---------|----------|-----------|----------|\n| AUTHZ-001 | user | actor has perm | allow |\n",
        );
        let (errors, _warnings) = lint_docset(tmp.path(), "docs", "examples", false, false);
        assert!(
            errors.iter().any(|f| f.check_id == "DS-CF-004"),
            "{errors:?}"
        );
    }

    #[test]
    fn choose_skill_names_uses_all_when_requested() {
        let available = vec!["a".to_string(), "b".to_string()];
        let selected = choose_skill_names(&available, true, &[]);
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0].as_str(), "a");
        assert_eq!(selected[1].as_str(), "b");
    }

    #[test]
    fn choose_skill_names_uses_explicit_selection() {
        let available = vec!["a".to_string(), "b".to_string()];
        let requested = vec!["b".to_string()];
        let selected = choose_skill_names(&available, false, &requested);
        assert_eq!(selected.len(), 1);
        assert_eq!(selected[0].as_str(), "b");
    }

    #[test]
    fn validate_skill_names_rejects_unknown_name() {
        let selected = vec![SkillName::new("__definitely_unknown__".to_string())];
        let result = validate_skill_names(&selected);
        assert!(result.is_err());
    }

    #[test]
    fn build_skill_targets_rejects_empty_target_list() {
        let result = build_skill_targets(&[], &None);
        assert!(result.is_err());
    }

    #[test]
    fn auto_lint_target_classifies_agents_file() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("AGENTS.md");
        write(&path, "# AGENTS.md\n");
        let target = auto_lint_target(&path, None).unwrap();
        assert!(matches!(target, AutoLintTarget::AgentsFile { .. }));
    }

    #[test]
    fn auto_lint_target_classifies_skill_file_as_directory() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("demo-skill");
        write(
            &dir.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: demo\n---\n",
        );
        let target = auto_lint_target(&dir.join("SKILL.md"), None).unwrap();
        match target {
            AutoLintTarget::SkillDir { dir: got } => assert_eq!(got, dir),
            _ => panic!("expected skill dir"),
        }
    }

    #[test]
    fn auto_lint_target_rejects_unknown_markdown_without_root() {
        let tmp = TempDir::new().unwrap();
        let doc = tmp.path().join("note.md");
        write(&doc, "# note\n");
        let err = auto_lint_target(&doc, None).unwrap_err();
        assert!(err.contains("Could not infer docset root"));
    }

    #[test]
    fn auto_lint_target_classifies_plain_directory_as_docset() {
        let tmp = TempDir::new().unwrap();
        let target = auto_lint_target(tmp.path(), None).unwrap();
        assert!(matches!(target, AutoLintTarget::DocsetDir { .. }));
    }

    #[test]
    fn extract_h2_section_returns_expected_block() {
        let text = "# T\n\n## One\nA\n\n## Two\nB\n";
        assert_eq!(extract_section(text, "One"), "A");
        assert_eq!(extract_section(text, "Two"), "B");
    }

    #[test]
    fn markdown_links_skips_external_and_fragment_only_targets() {
        let text = "[a](./x.md#part) [b](https://example.com) [c](mailto:test@example.com)";
        let links = markdown_links(text);
        assert_eq!(links, vec!["./x.md".to_string()]);
    }

    #[test]
    fn read_frontmatter_block_requires_delimiters() {
        let err = read_frontmatter_block("name: demo");
        assert!(err.is_err());
    }

    #[test]
    fn parse_frontmatter_supports_folded_description() {
        let fm = "name: demo\ndescription: >\n  line one\n  line two\n";
        let parsed = parse_frontmatter(fm).unwrap();
        assert_eq!(parsed.name, "demo");
        assert_eq!(parsed.description, "line one line two");
    }

    #[test]
    fn validate_name_and_description_rules() {
        assert!(validate_name("good-name").is_none());
        assert!(validate_name("BadName").is_some());
        assert!(validate_description("clean description").is_none());
        assert!(validate_description("bad <desc>").is_some());
    }

    #[test]
    fn extract_local_markdown_links_filters_and_deduplicates() {
        let text = "[a](./x.md#frag) [b](./x.md) [c](mailto:a@b.c) [d](/root.md)";
        let links = extract_local_markdown_links(text);
        assert_eq!(links.len(), 1);
        assert!(links.contains("./x.md"));
    }

    #[test]
    fn normalize_join_collapses_parent_components() {
        let got = normalize_join(Path::new("/tmp/a/b"), "../c.md");
        assert_eq!(got, PathBuf::from("/tmp/a/c.md"));
    }

    #[test]
    fn parse_yaml_subset_normalizes_links_shape() {
        let yaml = "id: ADR-0001\nlinks:\n  use_cases: UC-0001\n";
        let parsed = parse_yaml_subset(yaml).unwrap();
        let Some(YamlValue::Map(links)) = parsed.get("links") else {
            panic!("expected links map");
        };
        let Some(YamlValue::List(values)) = links.get("use_cases") else {
            panic!("expected use_cases list");
        };
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn parse_yaml_subset_rejects_list_item_outside_list_context() {
        let yaml = "id: UC-1\n- orphan";
        let parsed = parse_yaml_subset(yaml);
        assert!(parsed.is_err());
    }

    #[test]
    fn parse_markdown_table_and_column_lookup_work() {
        let section = "| actor_id | condition | decision |\n|---|---|---|\n| user | ok | allow |\n";
        let (headers, rows) = parse_markdown_table(section);
        assert_eq!(rows.len(), 1);
        assert_eq!(table_column_index(&headers, "actor_id"), Some(0));
        assert_eq!(table_column_index(&headers, "decision"), Some(2));
    }

    #[test]
    fn scan_placeholders_downgrades_open_question_lines_to_warnings() {
        let text = "## Open Questions\n[OPEN] TODO decide\n";
        let (errors, warnings) = scan_placeholders(Path::new("x.md"), text, "X", "error", true);
        assert!(errors.is_empty());
        assert!(!warnings.is_empty());
    }

    #[test]
    fn tier_and_placeholder_predicates_cover_expected_cases() {
        assert!(is_tier2_or_higher(&Some("tier2".to_string())));
        assert!(!is_tier2_or_higher(&Some("tier1".to_string())));
        assert!(looks_non_placeholder("real value"));
        assert!(!looks_non_placeholder("N/A"));
        assert!(!looks_non_placeholder("[placeholder]"));
    }

    #[test]
    fn table_has_non_placeholder_rows_requires_all_columns() {
        let section =
            "| actor_id | condition | decision |\n|---|---|---|\n| user | has perm | allow |\n";
        assert!(table_has_non_placeholder_rows(
            section,
            &["actor_id", "condition", "decision"]
        ));
    }

    #[test]
    fn is_decisionful_uc_detects_keyword_and_contract_table() {
        let keyword_uc = "This feature impacts payment and billing behavior.";
        assert!(is_decisionful_uc(keyword_uc));
        let table_uc =
            "## AuthZ\n| actor_id | condition | decision |\n|---|---|---|\n| user | ok | allow |\n";
        assert!(is_decisionful_uc(table_uc));
    }

    #[test]
    fn resolve_local_path_filters_external_and_fragment_targets() {
        assert!(resolve_local_path(Path::new("."), "https://example.com").is_none());
        assert!(resolve_local_path(Path::new("."), "").is_none());
        let resolved = resolve_local_path(Path::new("/tmp"), "./a.md#x").unwrap();
        assert_eq!(resolved, PathBuf::from("/tmp/a.md"));
    }

    #[test]
    fn read_doc_parses_core_frontmatter_fields() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("uc.md");
        write(
            &path,
            "---\nid: UC-0003\ntype: use_case\nrisk_tier: tier3\nlinks:\n  glossary: N/A\n---\n\n# UC-0003\n",
        );
        let doc = read_doc(&path).unwrap();
        assert_eq!(doc.doc_id, "UC-0003");
        assert_eq!(doc.doc_type.as_deref(), Some("use_case"));
        assert_eq!(doc.risk_tier.as_deref(), Some("tier3"));
    }

    #[test]
    fn agents_linter_reports_missing_file_and_directory_path() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("missing.md");
        let (errors_missing, _) = lint_agents_md(&missing, false);
        assert!(errors_missing.iter().any(|f| f.check_id == "AG001"));

        let dir = tmp.path().join("dir");
        fs::create_dir_all(&dir).unwrap();
        let (errors_dir, _) = lint_agents_md(&dir, false);
        assert!(errors_dir.iter().any(|f| f.check_id == "AG001"));
    }

    #[test]
    fn agents_linter_reports_required_sections_and_guardrails() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join("AGENTS.md");
        write(&agents, "# AGENTS.md\n");
        let (errors, warnings) = lint_agents_md(&agents, false);
        assert!(errors.iter().any(|f| f.check_id == "AG002"));
        assert!(errors.iter().any(|f| f.check_id == "AG003"));
        assert!(warnings.iter().any(|f| f.check_id == "AG007"));
    }

    #[test]
    fn agents_linter_detects_todo_broken_link_and_vague_strict() {
        let tmp = TempDir::new().unwrap();
        let agents = tmp.path().join("AGENTS.md");
        write(
            &agents,
            "# AGENTS.md\n\n## CRITICAL\n- MUST: lint now\n- MUST: test now\n- NEVER: force push\n- NEVER: hooks\n- NEVER: secret handling\n- NEVER: generated files\n- ON FAIL (lint): retry\n- ON FAIL (test): retry\n- as appropriate\n\n## Commands\n- install\n- lint\n- test\n- [broken](missing.md)\nTODO: fill\n",
        );
        let (errors, _) = lint_agents_md(&agents, true);
        assert!(errors.iter().any(|f| f.check_id == "AG009"));
        assert!(errors.iter().any(|f| f.check_id == "AG010"));
        assert!(errors.iter().any(|f| f.check_id == "AG011"));
    }

    #[test]
    fn skill_linter_reports_expected_failure_codes() {
        let tmp = TempDir::new().unwrap();

        let (sk001, _) = lint_skill(&tmp.path().join("nope"), 500);
        assert!(sk001.iter().any(|f| f.check_id == "SK001"));

        let empty_dir = tmp.path().join("empty");
        fs::create_dir_all(&empty_dir).unwrap();
        let (sk002, _) = lint_skill(&empty_dir, 500);
        assert!(sk002.iter().any(|f| f.check_id == "SK002"));

        let skill = tmp.path().join("bad-skill");
        fs::create_dir_all(&skill).unwrap();
        write(
            &skill.join("SKILL.md"),
            "---\nname: bad-skill\ndescription: ok\n---\n\nTODO: fix\n",
        );
        let (sk005, _) = lint_skill(&skill, 500);
        assert!(sk005.iter().any(|f| f.check_id == "SK005"));
    }

    #[test]
    fn skill_linter_reports_frontmatter_and_link_errors() {
        let tmp = TempDir::new().unwrap();
        let skill = tmp.path().join("demo-skill");
        fs::create_dir_all(&skill).unwrap();

        write(&skill.join("SKILL.md"), "---\nname: demo-skill\n");
        let (sk006, _) = lint_skill(&skill, 500);
        assert!(sk006.iter().any(|f| f.check_id == "SK006"));

        write(
            &skill.join("SKILL.md"),
            "---\nname: DemoSkill\ndescription: ok\n---\n",
        );
        let (sk007, _) = lint_skill(&skill, 500);
        assert!(sk007.iter().any(|f| f.check_id == "SK007"));

        write(
            &skill.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: bad <desc>\n---\n",
        );
        let (sk008, _) = lint_skill(&skill, 500);
        assert!(sk008.iter().any(|f| f.check_id == "SK008"));

        write(
            &skill.join("SKILL.md"),
            "---\nname: demo-skill\ndescription: ok\n---\n\n[bad](../outside.md)\n",
        );
        let (sk010, _) = lint_skill(&skill, 500);
        assert!(sk010.iter().any(|f| f.check_id == "SK010"));
    }

    #[test]
    fn skill_linter_reports_chain_and_openai_yaml_errors() {
        let tmp = TempDir::new().unwrap();
        let skill = tmp.path().join("chain-skill");
        fs::create_dir_all(skill.join("references")).unwrap();
        fs::create_dir_all(skill.join("agents")).unwrap();
        write(
            &skill.join("SKILL.md"),
            "---\nname: chain-skill\ndescription: ok\n---\n\n[Ref](references/ref.md)\n",
        );
        write(&skill.join("references/ref.md"), "[Deep](nested.md)\n");
        write(&skill.join("references/nested.md"), "# nested\n");
        let (sk012, _) = lint_skill(&skill, 500);
        assert!(sk012.iter().any(|f| f.check_id == "SK012"));

        write(
            &skill.join("SKILL.md"),
            "---\nname: chain-skill\ndescription: ok\n---\n",
        );
        write(&skill.join("agents/openai.yaml"), "interface: x\n");
        let (sk013, _) = lint_skill(&skill, 500);
        assert!(sk013.iter().any(|f| f.check_id == "SK013"));
    }

    #[test]
    fn parse_yaml_scalar_and_doc_links_cover_additional_paths() {
        assert!(matches!(parse_yaml_scalar("true"), YamlValue::Bool));
        assert!(matches!(parse_yaml_scalar("false"), YamlValue::Bool));
        assert!(matches!(parse_yaml_scalar("[]"), YamlValue::List(v) if v.is_empty()));
        assert!(matches!(parse_yaml_scalar(""), YamlValue::Str(s) if s.is_empty()));
        let links = doc_markdown_links("[x](a.md#h) [y](mailto:test@example.com)");
        assert_eq!(links, vec!["a.md".to_string()]);
    }

    #[test]
    fn docset_linter_reports_more_rule_violations() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/uc_missing_id.md"),
            "---\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# Missing\n",
        );
        write(
            &tmp.path().join("docs/uc_missing_links.md"),
            "---\nid: UC-0005\ntype: use_case\n---\n\n# UC-0005: No links\n",
        );
        write(
            &tmp.path().join("docs/uc_broken_link.md"),
            "---\nid: UC-0006\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0006\n[broken](missing.md)\n",
        );
        let (errors, _warnings) = lint_docset(tmp.path(), "docs", "examples", false, false);
        assert!(errors.iter().any(|f| f.check_id == "DS-ID-001"));
        assert!(errors.iter().any(|f| f.check_id == "DS-CF-001"));
        assert!(errors.iter().any(|f| f.check_id == "DS-CF-002"));
        assert!(errors.iter().any(|f| f.check_id == "DS-S-001"));
    }

    #[test]
    fn skill_install_target_covers_write_and_overwrite_paths() {
        let selected = vec![SkillName::new("__unknown_skill__".to_string())];
        let tmp = TempDir::new().unwrap();
        let target = SkillInstallTarget {
            tool: SkillTarget::Codex,
            home_dir: tmp.path().join("home"),
        };

        let dry_run = run_skill_install_target(&target, &selected, &None, false, true);
        assert!(dry_run.is_ok());
        let install_err = run_skill_install_target(&target, &selected, &None, false, false);
        assert!(install_err.is_err());
    }

    #[test]
    fn emit_lint_result_covers_json_and_text_modes() {
        let ok = emit_lint_result(
            LintResult {
                path: None,
                root: None,
                pass: true,
                error_count: 0,
                warning_count: 0,
                errors: vec![],
                warnings: vec![],
            },
            OutputFormat::Text,
        );
        assert_eq!(ok, 0);

        let fail_code = emit_lint_result(
            LintResult {
                path: Some("x".to_string()),
                root: None,
                pass: false,
                error_count: 1,
                warning_count: 1,
                errors: vec![Finding {
                    check_id: "E".to_string(),
                    severity: "error".to_string(),
                    message: "err".to_string(),
                    evidence: "x".to_string(),
                }],
                warnings: vec![Finding {
                    check_id: "W".to_string(),
                    severity: "warning".to_string(),
                    message: "warn".to_string(),
                    evidence: "x".to_string(),
                }],
            },
            OutputFormat::Json,
        );
        assert_eq!(fail_code, 1);
    }

    #[test]
    fn tilde_and_home_helpers_cover_paths() {
        let base = Path::new("~/demo");
        let expanded = base.expand_tilde();
        if let Some(path) = expanded {
            assert!(!path.to_string_lossy().is_empty());
        }
        let _ = home_dir();
        let codex = default_tool_home(SkillTarget::Codex);
        let claude = default_tool_home(SkillTarget::Claude);
        assert!(!codex.to_string_lossy().is_empty());
        assert!(!claude.to_string_lossy().is_empty());
    }

    #[test]
    fn docset_linter_covers_entity_h1_and_placeholder_checks() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/glossary.md"),
            "---\nid: GLOSSARY-002\ntype: glossary\n---\n\n# GLOSSARY-002\n\n## Terms\n| term | definition (one line) | allowed_synonyms | banned_synonyms |\n|---|---|---|---|\n| KnownEntity | desc | - | - |\n",
        );
        write(
            &tmp.path().join("docs/uc.md"),
            "---\nid: UC-0100\ntype: use_case\nrisk_tier: tier2\nlinks:\n  glossary: ./glossary.md\n  nfr: N/A\n---\n\n# Wrong Heading\n\n## Entities (Referenced)\n| entity |\n|---|\n| MissingEntity |\n\n## Open Questions\n[OPEN] TODO unresolved\n",
        );
        write(
            &tmp.path().join("examples/example_uc.md"),
            "---\nid: UC-EX-1\ntype: use_case\nrisk_tier: tier2\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-EX-1\n\nUC-XXXX placeholder\n",
        );

        let (errors, warnings) = lint_docset(tmp.path(), "docs", "examples", true, false);
        assert!(errors.iter().any(|f| f.check_id == "DS-CF-003"));
        assert!(warnings.iter().any(|f| f.check_id == "DS-S-002"));
        assert!(
            errors.iter().any(|f| f.check_id == "EX-CF-001")
                || errors.iter().any(|f| f.check_id == "DS-DOC-PLACEHOLDERS")
        );
    }

    #[test]
    fn parse_yaml_subset_supports_list_items_under_explicit_list() {
        let yaml = "id: ADR-1000\nlinks:\n  use_cases: []\n   - UC-1\n   - UC-2\n";
        let parsed = parse_yaml_subset(yaml).unwrap();
        let Some(YamlValue::Map(links)) = parsed.get("links") else {
            panic!("expected links map");
        };
        let Some(YamlValue::List(values)) = links.get("use_cases") else {
            panic!("expected use_cases list");
        };
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn parse_yaml_subset_rejects_mapping_key_under_list_context() {
        let yaml = "links:\n  use_cases: []\n    invalid: true\n";
        let parsed = parse_yaml_subset(yaml);
        assert!(parsed.is_err());
    }

    #[test]
    fn lint_docset_strict_promotes_warnings_to_errors() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/uc.md"),
            "---\nid: UC-0200\ntype: use_case\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# Wrong Header\n",
        );
        let (errors, warnings) = lint_docset(tmp.path(), "docs", "examples", false, true);
        assert!(warnings.is_empty());
        assert!(errors.iter().any(|f| f.check_id == "DS-S-002"));
    }

    #[test]
    fn lint_docset_adr_use_cases_list_satisfies_cf004() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("docs/uc.md"),
            "---\nid: UC-0300\ntype: use_case\nrisk_tier: tier2\nlinks:\n  glossary: N/A\n  nfr: N/A\n---\n\n# UC-0300\n\n## AuthZ\n| actor_id | condition | decision |\n|---|---|---|\n| user | ok | allow |\n",
        );
        write(
            &tmp.path().join("docs/adr.md"),
            "---\nid: ADR-0300\nlinks:\n  use_cases: []\n   - UC-0300\n---\n\n# ADR-0300\n",
        );
        let (errors, _warnings) = lint_docset(tmp.path(), "docs", "examples", false, false);
        assert!(!errors.iter().any(|f| f.check_id == "DS-CF-004"));
    }

    #[test]
    fn lint_docset_emits_example_presence_warnings() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("examples/glossary.md"),
            "---\nid: GLOSSARY-EX\ntype: glossary\n---\n\n# GLOSSARY-EX\n",
        );
        let (_errors, warnings) = lint_docset(tmp.path(), "docs", "examples", false, false);
        assert!(warnings.iter().any(|f| f.check_id == "DS-S-007"));
    }

    #[test]
    fn skill_target_label_and_namespace_destination_are_typed() {
        let target = SkillInstallTarget {
            tool: SkillTarget::Claude,
            home_dir: PathBuf::from("/tmp/home"),
        };
        assert_eq!(target.tool_label(), "claude");
        let skill = SkillName::new("skill-a".to_string());
        let ns = Some("ns".to_string());
        let path = skill_destination_path(&target, &skill, &ns);
        assert_eq!(path, PathBuf::from("/tmp/home/skills/ns/skill-a"));
    }

    #[test]
    fn install_skill_to_destination_covers_overwrite_paths() {
        let tmp = TempDir::new().unwrap();
        let dest = tmp.path().join("installed-skill");
        fs::create_dir_all(&dest).unwrap();
        let skill = SkillName::new("__unknown_skill__".to_string());

        let without_overwrite = install_skill_to_destination(&skill, &dest, false);
        assert!(without_overwrite.is_err());

        let with_overwrite = install_skill_to_destination(&skill, &dest, true);
        assert!(with_overwrite.is_err());
    }
}
