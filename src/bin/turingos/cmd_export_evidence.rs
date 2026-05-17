//! TRACE_MATRIX FC2-N16: turingos export evidence handler (filesystem copy)
//!
//! Phase 6.1 W2.4 atom. Pure filesystem operation: recursively copies an
//! evidence directory to a new output path. No sequencer call, no typed_tx,
//! no CAS write, no ChainTape advance.
//!
//! Class 1: additive isolated helper; read-source + write-output.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

// ─────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: export evidence error variants
#[derive(Debug)]
pub(crate) enum ExportError {
    /// --source path does not exist.
    SourceNotFound(String),
    /// --source path exists but is not a directory.
    SourceNotDir(String),
    /// --out path already exists; user must choose a different path.
    OutExists(String),
    /// --out parent directory does not exist.
    OutParentNotFound(String),
    /// Underlying I/O failure during copy.
    Io(String),
}

impl ExportError {
    fn exit_code(&self) -> u8 {
        match self {
            // Domain errors (user intent conflict): exit 1
            Self::SourceNotFound(_)
            | Self::SourceNotDir(_)
            | Self::OutExists(_)
            | Self::OutParentNotFound(_) => 1,
            // Unexpected I/O: exit 2
            Self::Io(_) => 2,
        }
    }
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceNotFound(p) => write!(f, "source path not found: {p}"),
            Self::SourceNotDir(p) => {
                write!(f, "source path exists but is not a directory: {p}")
            }
            Self::OutExists(p) => write!(
                f,
                "output path already exists: {p} \
                 (use --force to overwrite — note: --force is not yet implemented in Phase 6.1)"
            ),
            Self::OutParentNotFound(p) => write!(
                f,
                "output parent directory does not exist: {p} — \
                 create the parent directory first"
            ),
            Self::Io(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Arg parsing
// ─────────────────────────────────────────────────────────────────────

/// Parsed arguments for `turingos export evidence`.
#[derive(Debug)]
struct ExportArgs {
    source: PathBuf,
    out: PathBuf,
}

enum ParsedExport {
    Args(ExportArgs),
    HelpRequested,
}

fn parse_export_args(args: &[String]) -> Result<ParsedExport, String> {
    let mut source: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut iter = args.iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--source" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --source".to_string())?;
                source = Some(PathBuf::from(val));
            }
            "--out" => {
                let val = iter
                    .next()
                    .ok_or_else(|| "missing value for --out".to_string())?;
                out = Some(PathBuf::from(val));
            }
            "-h" | "--help" => {
                return Ok(ParsedExport::HelpRequested);
            }
            other => {
                return Err(format!("unrecognized argument: {other}"));
            }
        }
    }

    let source = source.ok_or_else(|| "missing required argument: --source <PATH>".to_string())?;
    let out = out.ok_or_else(|| "missing required argument: --out <PATH>".to_string())?;

    Ok(ParsedExport::Args(ExportArgs { source, out }))
}

// ─────────────────────────────────────────────────────────────────────
// Recursive copy
// ─────────────────────────────────────────────────────────────────────

/// Walk `src_dir` recursively; copy every file into `dst_dir`, preserving
/// relative paths. Returns (file_count, total_bytes).
///
/// Uses only `std::fs` — no external dependencies.
fn copy_dir_recursive(src_dir: &Path, dst_dir: &Path) -> Result<(usize, u64), ExportError> {
    fs::create_dir_all(dst_dir)
        .map_err(|e| ExportError::Io(format!("create output dir {}: {e}", dst_dir.display())))?;

    let mut file_count: usize = 0;
    let mut total_bytes: u64 = 0;

    // Iterative BFS using a work queue to avoid recursion limits on deep trees.
    let mut queue: Vec<(PathBuf, PathBuf)> = vec![(src_dir.to_path_buf(), dst_dir.to_path_buf())];

    while let Some((src, dst)) = queue.pop() {
        let entries = fs::read_dir(&src)
            .map_err(|e| ExportError::Io(format!("read dir {}: {e}", src.display())))?;

        for entry in entries {
            let entry = entry
                .map_err(|e| ExportError::Io(format!("read entry in {}: {e}", src.display())))?;

            let entry_path = entry.path();
            let file_name = entry.file_name();
            let dst_entry = dst.join(&file_name);

            let file_type = entry
                .file_type()
                .map_err(|e| ExportError::Io(format!("stat {}: {e}", entry_path.display())))?;

            if file_type.is_dir() {
                fs::create_dir_all(&dst_entry).map_err(|e| {
                    ExportError::Io(format!("create dir {}: {e}", dst_entry.display()))
                })?;
                queue.push((entry_path, dst_entry));
            } else if file_type.is_file() {
                let bytes = fs::copy(&entry_path, &dst_entry).map_err(|e| {
                    ExportError::Io(format!(
                        "copy {} -> {}: {e}",
                        entry_path.display(),
                        dst_entry.display()
                    ))
                })?;
                file_count += 1;
                total_bytes += bytes;
            }
            // Symlinks are skipped (non-destructive; evidence dirs should not
            // rely on symlinks for canonical content).
        }
    }

    Ok((file_count, total_bytes))
}

// ─────────────────────────────────────────────────────────────────────
// export evidence subcommand handler
// ─────────────────────────────────────────────────────────────────────

fn cmd_export_evidence_inner(args: ExportArgs) -> Result<(), ExportError> {
    // Validate --source
    if !args.source.exists() {
        return Err(ExportError::SourceNotFound(
            args.source.display().to_string(),
        ));
    }
    if !args.source.is_dir() {
        return Err(ExportError::SourceNotDir(args.source.display().to_string()));
    }

    // Validate --out: must not exist; parent must exist.
    if args.out.exists() {
        return Err(ExportError::OutExists(args.out.display().to_string()));
    }
    let out_parent = args.out.parent().unwrap_or_else(|| Path::new("."));
    if !out_parent.exists() {
        return Err(ExportError::OutParentNotFound(
            out_parent.display().to_string(),
        ));
    }

    let (file_count, total_bytes) = copy_dir_recursive(&args.source, &args.out)?;

    println!(
        "exported {} -> {}: {} files, {} bytes",
        args.source.display(),
        args.out.display(),
        file_count,
        total_bytes,
    );

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────
// Public API (pub(crate))
// ─────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: export evidence subcommand short-help (registry display)
pub(crate) const SHORT_HELP: &str =
    "Export an evidence directory as a self-contained bundle (filesystem copy)";

/// TRACE_MATRIX FC2-N16: export evidence subcommand --help text
pub(crate) const FULL_HELP: &str = r#"turingos export evidence — Export an evidence directory as a self-contained bundle

USAGE:
    turingos export evidence --source <PATH> --out <PATH>

OPTIONS:
    --source <PATH>   Path to the source evidence directory
                      (e.g. handover/evidence/<run_id>).
                      Must exist and be a directory.

    --out <PATH>      Destination path for the exported bundle.
                      Must NOT already exist (non-destructive; see
                      --force note below).
                      The parent directory must exist.

    -h, --help        Print this help.

DESCRIPTION:
    Recursively copies the source evidence directory to the output path,
    preserving the complete file structure. Suitable for archiving a
    post-run evidence bundle or sharing a ChainTape / CAS snapshot.

    Class 1 operation: read-source + write-output only. No sequencer
    call, no typed_tx, no CAS write, no ChainTape advance.

    FC2-N16: evidence bundle is reconstructable from genesis_report +
    ChainTape + CAS as required by the boot/genesis gate.

OUTPUT:
    On success, prints:
        exported <source> -> <out>: <N> files, <M> bytes

NOTES:
    --force (allow overwriting existing --out): not yet implemented in
    Phase 6.1. Planned for a future atom. Use a fresh --out path for now.

    Symlinks inside the source directory are skipped (evidence dirs
    should not rely on symlinks for canonical content).

EXAMPLES:
    turingos export evidence \
        --source handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z \
        --out /tmp/evidence_bundle_tb_c0
"#;

/// TRACE_MATRIX FC2-N16: export evidence subcommand dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{}", FULL_HELP);
        return ExitCode::SUCCESS;
    }
    match parse_export_args(args) {
        Ok(ParsedExport::HelpRequested) => {
            print!("{}", FULL_HELP);
            ExitCode::SUCCESS
        }
        Ok(ParsedExport::Args(parsed)) => match cmd_export_evidence_inner(parsed) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                let code = err.exit_code();
                eprintln!("turingos export evidence: {err}");
                ExitCode::from(code)
            }
        },
        Err(msg) => {
            eprintln!("turingos export evidence: {msg}");
            eprintln!();
            eprintln!("Run `turingos export evidence --help` for usage.");
            ExitCode::from(2)
        }
    }
}
