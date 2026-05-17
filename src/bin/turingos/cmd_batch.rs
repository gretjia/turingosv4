//! TRACE_MATRIX FC2-N16: turingos batch handler (workspace batches/ scaffold)
//!
//! Phase 6.1 scaffold-only implementation. Creates and inspects batch
//! directories under `<workspace>/batches/<name>/manifest.toml`.
//! Phase 6.2 will wire `turingos batch start` to invoke the evaluator.
//!
//! Class 1 (additive isolated module): pure filesystem, no sequencer call,
//! no typed_tx, no CAS write, no ChainTape advance.

use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

// ─────────────────────────────────────────────────────────────────────
// Public API strings
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: short help shown in top-level `turingos --help`.
pub(crate) const SHORT_HELP: &str =
    "Manage batch scaffolds (new/list/view; Phase 6.1 scaffold-only)";

/// TRACE_MATRIX FC2-N16: full help for `turingos batch --help`.
pub(crate) const FULL_HELP: &str = r#"turingos batch — Manage batch scaffolds (Phase 6.1 scaffold-only)

USAGE:
    turingos batch <ACTION> [OPTIONS]

ACTIONS:
    new     Create a new batch scaffold directory.
    list    List all batch directories in the workspace.
    view    Print the manifest.toml for a named batch.

OPTIONS (new):
    --name <NAME>          Batch name (ASCII alphanumeric, _, -, . only).
                           Required.
    --workspace <PATH>     Workspace root directory.
                           Required.
    --problems <FILE>      Optional path to a problems file (recorded in
                           manifest; not validated in Phase 6.1).

OPTIONS (list):
    --workspace <PATH>     Workspace root directory.
                           Required.

OPTIONS (view):
    --name <NAME>          Batch name to inspect.
                           Required.
    --workspace <PATH>     Workspace root directory.
                           Required.

    -h, --help             Print this help.

DESCRIPTION:
    Creates <workspace>/batches/<name>/manifest.toml scaffold directories.
    Phase 6.2+ will add `turingos batch start` to invoke the evaluator.
    This Phase 6.1 command is filesystem-only (Class 1): no sequencer call,
    no typed_tx, no CAS write, no ChainTape advance.

    Batch name validation: ASCII alphanumeric, underscore (_), hyphen (-),
    and period (.) characters only. No path separators (/) or spaces.

EXAMPLES:
    turingos batch new --name run001 --workspace ~/my_workspace
    turingos batch new --name run002 --workspace ~/my_workspace \
        --problems problems/minif2f_v2.jsonl
    turingos batch list --workspace ~/my_workspace
    turingos batch view --name run001 --workspace ~/my_workspace

NOTE:
    `turingos batch start` (Phase 6.2) is NOT YET implemented.
    Use the established lean_market workflow for actual evaluation runs.
"#;

// ─────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: error variants for batch subcommand operations.
#[derive(Debug)]
pub(crate) enum BatchError {
    /// Batch directory already exists; cannot overwrite without explicit flag.
    BatchAlreadyExists(String),
    /// Batch name fails validation (non-ASCII, path chars, spaces, etc.).
    InvalidBatchName(String),
    /// --problems value contains characters that would break TOML quoting
    /// (quote, newline, backslash). Rejected to avoid injection.
    InvalidProblemsValue(String),
    /// Required argument missing (--name, --workspace, etc.).
    MissingArg(String),
    /// Unknown action or argument.
    UnknownArg(String),
    /// Filesystem I/O error.
    Io(String),
}

impl BatchError {
    /// TRACE_MATRIX FC2-N16: exit code mapping for batch errors.
    pub(crate) fn exit_code(&self) -> u8 {
        match self {
            // I/O: exit 2
            Self::Io(_) => 2,
            // Domain errors: exit 1
            Self::BatchAlreadyExists(_)
            | Self::InvalidBatchName(_)
            | Self::InvalidProblemsValue(_)
            | Self::MissingArg(_)
            | Self::UnknownArg(_) => 1,
        }
    }
}

impl std::fmt::Display for BatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BatchAlreadyExists(name) => {
                write!(f, "batch '{name}' already exists — pick a different --name")
            }
            Self::InvalidBatchName(name) => write!(
                f,
                "invalid batch name '{name}': use ASCII alphanumeric, _, -, . only \
                 (no path separators, no spaces, no leading /)"
            ),
            Self::InvalidProblemsValue(val) => write!(
                f,
                "invalid --problems value (must not contain quotes, newlines, or \
                 backslashes; would break manifest TOML quoting): {val}"
            ),
            Self::MissingArg(msg) => write!(f, "missing required argument: {msg}"),
            Self::UnknownArg(arg) => write!(f, "unrecognized argument: {arg}"),
            Self::Io(msg) => write!(f, "io error: {msg}"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Name validation
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: validate a batch name.
///
/// Accepts ASCII alphanumeric, `_`, `-`, `.`.
/// Rejects empty, path separators (`/`, `\`), spaces, leading/trailing dots
/// that would form `..`, and any non-ASCII character.
fn validate_batch_name(name: &str) -> Result<(), BatchError> {
    if name.is_empty() {
        return Err(BatchError::InvalidBatchName(name.to_string()));
    }
    // Reject if any character is not in the allowed ASCII set.
    let all_ok = name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.');
    if !all_ok {
        return Err(BatchError::InvalidBatchName(name.to_string()));
    }
    // Reject path-traversal patterns: "..", leading ".", component containing "/".
    if name.contains("..") || name.starts_with('.') || name.contains('/') {
        return Err(BatchError::InvalidBatchName(name.to_string()));
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// Actions
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: create a new batch scaffold.
fn action_new(name: &str, workspace: &str, problems: Option<&str>) -> Result<(), BatchError> {
    validate_batch_name(name)?;

    // Reject --problems values that would break TOML quoting / inject extra
    // fields (quote, newline, backslash, carriage return).
    if let Some(p) = problems {
        if p.contains('"') || p.contains('\n') || p.contains('\r') || p.contains('\\') {
            return Err(BatchError::InvalidProblemsValue(p.to_string()));
        }
    }

    let batch_dir = PathBuf::from(workspace).join("batches").join(name);

    if batch_dir.exists() {
        return Err(BatchError::BatchAlreadyExists(name.to_string()));
    }

    fs::create_dir_all(&batch_dir)
        .map_err(|e| BatchError::Io(format!("create batch dir {}: {e}", batch_dir.display())))?;

    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let problems_value = problems.unwrap_or("");
    let manifest = format!(
        "# turingos batch manifest\n\
         # Created by `turingos batch new`\n\
         name = \"{name}\"\n\
         created_at_unix = {epoch_secs}\n\
         problems_file = \"{problems_value}\"\n\
         status = \"scaffold\"  # scaffold | running | finished (Phase 6.2+)\n"
    );

    let manifest_path = batch_dir.join("manifest.toml");
    fs::write(&manifest_path, &manifest)
        .map_err(|e| BatchError::Io(format!("write manifest.toml: {e}")))?;

    println!("Created batch scaffold: {}", batch_dir.display());
    println!("  manifest: {}", manifest_path.display());
    println!();
    println!("Run `turingos batch list --workspace {workspace}` to see all batches.");
    println!("Run `turingos batch view --name {name} --workspace {workspace}` to inspect.");

    Ok(())
}

/// TRACE_MATRIX FC2-N16: list all batch directories in the workspace.
fn action_list(workspace: &str) -> Result<(), BatchError> {
    let batches_dir = PathBuf::from(workspace).join("batches");

    if !batches_dir.exists() {
        println!("No batches found (batches/ directory does not exist yet).");
        println!("  workspace: {workspace}");
        println!("  Run `turingos batch new --name <NAME> --workspace {workspace}` to create one.");
        return Ok(());
    }

    let mut entries: Vec<String> = Vec::new();

    let read_dir = fs::read_dir(&batches_dir)
        .map_err(|e| BatchError::Io(format!("read batches dir {}: {e}", batches_dir.display())))?;

    for entry in read_dir {
        let entry = entry.map_err(|e| BatchError::Io(format!("read dir entry: {e}")))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let batch_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();

        // Try to read status from manifest.
        let manifest_path = path.join("manifest.toml");
        let status = if manifest_path.exists() {
            let content = fs::read_to_string(&manifest_path).unwrap_or_default();
            content
                .lines()
                .find(|l| l.starts_with("status = "))
                .and_then(|l| {
                    // Extract value between first pair of quotes.
                    let rest = l.trim_start_matches("status = ").trim();
                    let inner = rest.trim_matches('"');
                    // Strip inline comment if any.
                    Some(inner.split('#').next().unwrap_or(inner).trim().to_string())
                })
                .unwrap_or_else(|| "?".to_string())
        } else {
            "no-manifest".to_string()
        };

        entries.push(format!("  {batch_name:<30}  status={status}"));
    }

    if entries.is_empty() {
        println!("No batch directories found in {}.", batches_dir.display());
    } else {
        entries.sort();
        println!("Batches in {}:", batches_dir.display());
        for line in &entries {
            println!("{line}");
        }
        println!();
        println!("{} batch(es) found.", entries.len());
    }

    Ok(())
}

/// TRACE_MATRIX FC2-N16: print the manifest.toml for a named batch.
fn action_view(name: &str, workspace: &str) -> Result<(), BatchError> {
    validate_batch_name(name)?;

    let manifest_path = PathBuf::from(workspace)
        .join("batches")
        .join(name)
        .join("manifest.toml");

    let content = fs::read_to_string(&manifest_path).map_err(|e| {
        BatchError::Io(format!(
            "cannot read manifest at {}: {e}",
            manifest_path.display()
        ))
    })?;

    println!("=== {} ===", manifest_path.display());
    print!("{content}");

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: dispatch `turingos batch <action> [args]`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // First check for help or no-args at the subcommand level.
    match args.first().map(String::as_str) {
        None | Some("-h") | Some("--help") => {
            print!("{FULL_HELP}");
            return ExitCode::SUCCESS;
        }
        _ => {}
    }

    let action = args[0].as_str();

    match action {
        "new" => {
            let rest = &args[1..];
            match parse_new_args(rest) {
                Ok((name, workspace, problems)) => {
                    match action_new(&name, &workspace, problems.as_deref()) {
                        Ok(()) => ExitCode::SUCCESS,
                        Err(e) => {
                            let code = e.exit_code();
                            eprintln!("turingos batch new: {e}");
                            ExitCode::from(code)
                        }
                    }
                }
                Err(e) => {
                    eprintln!("turingos batch new: {e}");
                    eprintln!();
                    eprintln!("Run `turingos batch --help` for usage.");
                    ExitCode::from(1)
                }
            }
        }
        "list" => {
            let rest = &args[1..];
            match parse_workspace_arg(rest) {
                Ok(workspace) => match action_list(&workspace) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(e) => {
                        let code = e.exit_code();
                        eprintln!("turingos batch list: {e}");
                        ExitCode::from(code)
                    }
                },
                Err(e) => {
                    eprintln!("turingos batch list: {e}");
                    eprintln!();
                    eprintln!("Run `turingos batch --help` for usage.");
                    ExitCode::from(1)
                }
            }
        }
        "view" => {
            let rest = &args[1..];
            match parse_name_workspace_args(rest) {
                Ok((name, workspace)) => match action_view(&name, &workspace) {
                    Ok(()) => ExitCode::SUCCESS,
                    Err(e) => {
                        let code = e.exit_code();
                        eprintln!("turingos batch view: {e}");
                        ExitCode::from(code)
                    }
                },
                Err(e) => {
                    eprintln!("turingos batch view: {e}");
                    eprintln!();
                    eprintln!("Run `turingos batch --help` for usage.");
                    ExitCode::from(1)
                }
            }
        }
        "-h" | "--help" => {
            print!("{FULL_HELP}");
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("turingos batch: unknown action '{other}'");
            eprintln!();
            eprintln!("Expected: new | list | view");
            eprintln!("Run `turingos batch --help` for usage.");
            ExitCode::from(1)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Arg parsers (private helpers)
// ─────────────────────────────────────────────────────────────────────

fn parse_new_args(args: &[String]) -> Result<(String, String, Option<String>), BatchError> {
    let mut name: Option<String> = None;
    let mut workspace: Option<String> = None;
    let mut problems: Option<String> = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--name" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --name".to_string()))?;
                name = Some(val.clone());
            }
            "--workspace" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --workspace".to_string()))?;
                workspace = Some(val.clone());
            }
            "--problems" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --problems".to_string()))?;
                problems = Some(val.clone());
            }
            "-h" | "--help" => {
                // Print help inline and signal via UnknownArg so caller exits cleanly.
                print!("{FULL_HELP}");
                return Err(BatchError::UnknownArg("--help".to_string()));
            }
            other => {
                return Err(BatchError::UnknownArg(other.to_string()));
            }
        }
    }

    let name = name.ok_or_else(|| BatchError::MissingArg("--name <NAME>".to_string()))?;
    let workspace =
        workspace.ok_or_else(|| BatchError::MissingArg("--workspace <PATH>".to_string()))?;

    Ok((name, workspace, problems))
}

fn parse_workspace_arg(args: &[String]) -> Result<String, BatchError> {
    let mut workspace: Option<String> = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --workspace".to_string()))?;
                workspace = Some(val.clone());
            }
            "-h" | "--help" => {
                print!("{FULL_HELP}");
                return Err(BatchError::UnknownArg("--help".to_string()));
            }
            other => {
                return Err(BatchError::UnknownArg(other.to_string()));
            }
        }
    }

    workspace.ok_or_else(|| BatchError::MissingArg("--workspace <PATH>".to_string()))
}

fn parse_name_workspace_args(args: &[String]) -> Result<(String, String), BatchError> {
    let mut name: Option<String> = None;
    let mut workspace: Option<String> = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--name" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --name".to_string()))?;
                name = Some(val.clone());
            }
            "--workspace" => {
                let val = iter
                    .next()
                    .ok_or_else(|| BatchError::MissingArg("value for --workspace".to_string()))?;
                workspace = Some(val.clone());
            }
            "-h" | "--help" => {
                print!("{FULL_HELP}");
                return Err(BatchError::UnknownArg("--help".to_string()));
            }
            other => {
                return Err(BatchError::UnknownArg(other.to_string()));
            }
        }
    }

    let name = name.ok_or_else(|| BatchError::MissingArg("--name <NAME>".to_string()))?;
    let workspace =
        workspace.ok_or_else(|| BatchError::MissingArg("--workspace <PATH>".to_string()))?;

    Ok((name, workspace))
}
