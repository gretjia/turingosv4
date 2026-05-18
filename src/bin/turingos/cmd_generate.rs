//! TRACE_MATRIX FC2-N16 + FC3 evidence binding: turingos generate handler (Phase 6.3)
//!
//! Reads the spec.md capsule from CAS (or the on-disk spec.md as a fallback),
//! calls the Blackbox LLM (default: Qwen3-Coder-30B), parses code fences out
//! of the response, and writes the resulting artifacts under
//! `<workspace>/artifacts/`.
//!
//! Generation contract (system prompt to Blackbox):
//!   - Output is ONE OR MORE complete files in fenced code blocks.
//!   - Each fence is preceded by `### File: <relative path>` on its own line.
//!   - For UI apps: prefer a single self-contained `index.html` with embedded
//!     CSS+JS so the user can just open it in a browser — minimum-friction
//!     for non-developer end-users.
//!   - For data / scripting tasks: prefer a single Python 3 file with a
//!     `python main.py` entry point.
//!   - No external dependencies unless the spec explicitly demands them.
//!
//! Class 1: filesystem write to `<workspace>/artifacts/`. No CAS write
//! (artifacts can be regenerated from the spec capsule + the same model_id
//! + same seed — pure derivation, no Class-3 evidence anchor needed). Per
//! HEAD_t / FC3 posture, the spec capsule CID + model_id + timestamp uniquely
//! identify the generation transcript; artifacts are a materialized view of
//! that derivation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cmd_llm;
use crate::siliconflow_client::{chat_complete_blocking, require_api_key, ChatMessage, LlmError};
use crate::spec_capsule;

/// TRACE_MATRIX FC2-N16: `generate` short-help
pub(crate) const SHORT_HELP: &str =
    "Generate working code from spec.md via the Blackbox LLM; writes to <workspace>/artifacts/";

/// TRACE_MATRIX FC2-N16: `generate` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos generate — Emit code from spec.md via the Blackbox LLM

USAGE:
    turingos generate --workspace <PATH> [--from-capsule] [--max-files <N>]

OPTIONS:
    --workspace <PATH>      Workspace directory (required; must have spec.md
                            from `turingos spec`).
    --from-capsule          Read spec.md bytes from the latest CAS
                            EvidenceCapsule rather than from <workspace>/spec.md.
                            Use this for reproducible regeneration: the capsule
                            CID is the canonical input.
    --max-files <N>         Max number of files to write (safety cap; default 20).
    --emit-transcript       Persist the LLM call transcript to
                            <workspace>/generate_transcript.jsonl. Default: off.
    -h, --help              Print this help.

DESCRIPTION:
    Class 1 filesystem write to <workspace>/artifacts/. One LLM call per
    `turingos generate` invocation. The Blackbox model is told to output
    one or more complete files, each preceded by `### File: <relative path>`
    plus a fenced code block. For UI apps it defaults to a single
    self-contained index.html so the end-user can just open it in a browser.
"#;

#[derive(Debug)]
enum GenError {
    MissingFlag(&'static str),
    WorkspaceNotFound(String),
    NoSpec(String),
    Io(String),
    Llm(LlmError),
    Capsule(spec_capsule::CapsuleError),
    NoFilesParsed,
    TooManyFiles { found: usize, max: usize },
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFlag(flag) => write!(f, "missing required flag: {flag}"),
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::NoSpec(p) => write!(
                f,
                "spec not found: {p} (run `turingos spec --workspace <PATH>` first)"
            ),
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Llm(e) => write!(f, "{e}"),
            Self::Capsule(e) => write!(f, "{e}"),
            Self::NoFilesParsed => write!(
                f,
                "Blackbox LLM emitted no parseable files. Expected `### File: <path>` followed by a fenced code block."
            ),
            Self::TooManyFiles { found, max } => {
                write!(f, "Blackbox LLM emitted {found} files; --max-files cap is {max}")
            }
        }
    }
}

impl From<LlmError> for GenError {
    fn from(e: LlmError) -> Self {
        Self::Llm(e)
    }
}

impl From<spec_capsule::CapsuleError> for GenError {
    fn from(e: spec_capsule::CapsuleError) -> Self {
        Self::Capsule(e)
    }
}

/// TRACE_MATRIX FC2-N16: `generate` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos generate: {e}");
            ExitCode::from(2)
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), GenError> {
    let mut workspace = PathBuf::from(".");
    let mut from_capsule = false;
    let mut max_files: usize = 20;
    let mut emit_transcript = false;

    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace = PathBuf::from(iter.next().ok_or(GenError::MissingFlag("--workspace"))?);
            }
            "--from-capsule" => from_capsule = true,
            "--max-files" => {
                let v = iter.next().ok_or(GenError::MissingFlag("--max-files"))?;
                max_files = v
                    .parse()
                    .map_err(|_| GenError::Io(format!("--max-files: not a number: {v}")))?;
            }
            "--emit-transcript" => emit_transcript = true,
            _ => {}
        }
    }
    if !workspace.exists() {
        return Err(GenError::WorkspaceNotFound(workspace.display().to_string()));
    }

    let (spec_md, source) = if from_capsule {
        let cid_hex = spec_capsule::latest_spec_capsule_cid(&workspace)?.ok_or_else(|| {
            GenError::NoSpec(format!("no spec capsule in {}/cas", workspace.display()))
        })?;
        let bytes = spec_capsule::read_spec_capsule(&workspace, &cid_hex)?;
        (
            String::from_utf8(bytes)
                .map_err(|e| GenError::Io(format!("CAS capsule is not UTF-8: {e}")))?,
            format!("CAS capsule {cid_hex}"),
        )
    } else {
        let p = workspace.join("spec.md");
        if !p.exists() {
            return Err(GenError::NoSpec(p.display().to_string()));
        }
        (
            fs::read_to_string(&p).map_err(|e| GenError::Io(e.to_string()))?,
            p.display().to_string(),
        )
    };

    let model_id = cmd_llm::read_blackbox_model(&workspace);
    let api_key_env = cmd_llm::read_api_key_env_var(&workspace);
    let api_key = require_api_key(&api_key_env)?;

    let messages = vec![
        ChatMessage::system(blackbox_system_prompt()),
        ChatMessage::user(format!(
            "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
        )),
    ];
    eprintln!("[generate] calling Blackbox LLM ({model_id})...");
    let result = chat_complete_blocking(&api_key, &model_id, &messages, Some(6000), Some(0.2))?;
    eprintln!(
        "[generate] LLM returned {} chars, {} tokens",
        result.content.len(),
        result.usage.total_tokens
    );

    let files = parse_emitted_files(&result.content);
    if files.is_empty() {
        // Save the raw response so the user can debug
        let raw_path = workspace.join("generate_raw_response.txt");
        let _ = fs::write(&raw_path, &result.content);
        eprintln!("[generate] raw response saved to {}", raw_path.display());
        return Err(GenError::NoFilesParsed);
    }
    if files.len() > max_files {
        return Err(GenError::TooManyFiles {
            found: files.len(),
            max: max_files,
        });
    }

    let artifacts_dir = workspace.join("artifacts");
    fs::create_dir_all(&artifacts_dir)
        .map_err(|e| GenError::Io(format!("create artifacts dir: {e}")))?;

    let mut written = Vec::new();
    for f in &files {
        let safe_rel = sanitize_relative_path(&f.path).map_err(GenError::Io)?;
        let full = artifacts_dir.join(&safe_rel);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| GenError::Io(format!("create dir {}: {e}", parent.display())))?;
        }
        fs::write(&full, &f.content)
            .map_err(|e| GenError::Io(format!("write {}: {e}", full.display())))?;
        written.push(safe_rel);
    }

    if emit_transcript {
        let logical_t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let transcript = serde_json::json!({
            "logical_t": logical_t,
            "model": model_id,
            "spec_source": source,
            "usage_total_tokens": result.usage.total_tokens,
            "files_written": written.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
            "raw_response": result.content,
        });
        let path = workspace.join("generate_transcript.jsonl");
        let mut out = transcript.to_string();
        out.push('\n');
        fs::write(&path, out).map_err(|e| GenError::Io(format!("transcript: {e}")))?;
    }

    println!();
    println!(
        "Generated {} file(s) under {}/",
        written.len(),
        artifacts_dir.display()
    );
    for p in &written {
        println!("  {}", p.display());
    }
    println!();
    println!("Open the entry file in your browser or run the entry script:");
    if let Some(html) = written
        .iter()
        .find(|p| p.extension().map(|x| x == "html").unwrap_or(false))
    {
        println!("  xdg-open {}/{}", artifacts_dir.display(), html.display());
    } else if let Some(py) = written
        .iter()
        .find(|p| p.extension().map(|x| x == "py").unwrap_or(false))
    {
        println!("  python3 {}/{}", artifacts_dir.display(), py.display());
    } else if let Some(first) = written.first() {
        println!("  {}/{}", artifacts_dir.display(), first.display());
    }
    Ok(())
}

fn blackbox_system_prompt() -> &'static str {
    r#"You are TuringOS Blackbox AI, a fast code-generation assistant.

Input: a spec.md describing what a non-developer user wants built.
Output: one or more complete, working source files.

**OUTPUT FORMAT — STRICT**:
For each file, output on its own line:
```
### File: <relative path>
```
Then a fenced code block with the file content. The fence opener must include
the language tag (e.g. ```html, ```python, ```javascript, ```css).

**RULES**:
1. Prefer ONE single self-contained file when possible. For a UI app, output
   ONE `index.html` with `<style>` and `<script>` embedded — so the user can
   open the file in a browser with zero install. For a script, output ONE
   Python 3 file named `main.py`.
2. No external runtime dependencies unless the spec explicitly demands them
   (no `npm install`, no `pip install`, no CDN scripts unless unavoidable).
3. The code must actually run as-emitted. If the spec is vague, choose a
   sensible default and add a brief comment marking the assumption.
4. NO surrounding prose. No "Here's the code:" preamble. No closing remarks.
   First line of your response is `### File: ...`. Last line is the closing
   ``` of the final code block.
5. Keep files focused. Do not add tests, README.md, package.json, or build
   configs unless the spec asks for them.
6. Honor the spec's "Out of Scope" / "Deliberately NOT Doing" section —
   do NOT add features it forbids.

Example shape (DO NOT COPY VERBATIM — write your own per the spec):
### File: index.html
```html
<!DOCTYPE html>
<html>...</html>
```
"#
}

struct EmittedFile {
    path: String,
    content: String,
}

/// Parse `### File: <path>` markers + fenced code blocks out of LLM output.
/// Tolerant of leading whitespace, surrounding blank lines, and Windows
/// line endings. Returns files in the order they appear.
fn parse_emitted_files(text: &str) -> Vec<EmittedFile> {
    let mut out = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if let Some(rest) = line
            .strip_prefix("### File:")
            .or_else(|| line.strip_prefix("### file:"))
        {
            let path = rest.trim().trim_matches('`').trim().to_string();
            // Find next ``` fence opener
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                i += 1;
            }
            if i >= lines.len() {
                break;
            }
            // i points at the fence opener; advance past it
            i += 1;
            let start = i;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                i += 1;
            }
            let content = lines[start..i].join("\n");
            // ensure final newline
            let mut c = content;
            if !c.ends_with('\n') {
                c.push('\n');
            }
            out.push(EmittedFile { path, content: c });
            // i points at closer; advance past it
            i += 1;
        } else {
            i += 1;
        }
    }
    out
}

/// Reject paths that try to escape <workspace>/artifacts/: no absolute
/// paths, no .., no leading slash. Returns the sanitized relative path.
fn sanitize_relative_path(rel: &str) -> Result<PathBuf, String> {
    let trimmed = rel.trim();
    if trimmed.is_empty() {
        return Err("empty file path".into());
    }
    let p = Path::new(trimmed);
    if p.is_absolute() {
        return Err(format!("absolute path not allowed: {trimmed}"));
    }
    for comp in p.components() {
        use std::path::Component;
        match comp {
            Component::ParentDir => {
                return Err(format!("`..` not allowed in path: {trimmed}"));
            }
            Component::Prefix(_) | Component::RootDir => {
                return Err(format!("root/prefix not allowed in path: {trimmed}"));
            }
            _ => {}
        }
    }
    Ok(p.to_path_buf())
}
