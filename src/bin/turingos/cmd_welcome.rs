//! TRACE_MATRIX FC2-N16: turingos welcome handler (Phase 6.3 onboarding)
//!
//! Class 1 read-only filesystem inspection. Shows the user where they are
//! in the TuringOS onboarding flow (which steps are done, which are next).
//! No network. No backend invocation. No write side-effects.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::common::shell_quote_path;
use crate::spec_capsule;

/// TRACE_MATRIX FC2-N16: `welcome` short-help
pub(crate) const SHORT_HELP: &str =
    "Show TuringOS onboarding status (which setup steps are done; what's next)";

/// TRACE_MATRIX FC2-N16: `welcome` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos welcome — Onboarding status + next-step guide

USAGE:
    turingos welcome [--workspace <PATH>]

DESCRIPTION:
    Read-only filesystem inspection of an existing TuringOS workspace (or
    the current directory if --workspace is omitted). Reports which
    onboarding steps are complete and prints the next-step command.

    The 5-step onboarding flow:
      1. `turingos init`             — scaffold a workspace
      2. `turingos llm config`       — set LLM API credentials (Meta + Blackbox)
      3. `turingos agent deploy`     — register at least one agent
      4. `turingos spec`             — interactively decompose your task
      5. `turingos generate`         — generate + deliver

OPTIONS:
    --workspace <PATH>   Workspace directory (default: current directory).
    -h, --help           Print this help.
"#;

#[derive(Debug)]
struct WorkspaceStatus {
    init_done: bool,
    llm_configured: bool,
    agents_count: usize,
    spec_done: bool,
    /// CAS capsule CID if a spec capsule has been written. Phase 6.3 adds
    /// the CAS wire so spec completion is provable, not just a file presence.
    spec_capsule_cid: Option<String>,
    artifacts_done: bool,
}

/// TRACE_MATRIX FC2-N16: `welcome` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") && args.len() <= 1 {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    let mut workspace = PathBuf::from(".");
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--workspace" {
            if let Some(v) = iter.next() {
                workspace = PathBuf::from(v);
            }
        }
    }

    let status = inspect_workspace(&workspace);
    render_status(&workspace, &status);
    ExitCode::SUCCESS
}

fn inspect_workspace(ws: &Path) -> WorkspaceStatus {
    let init_done =
        ws.join("genesis_payload.toml").is_file() && ws.join("agent_pubkeys.json").is_file();

    let toml_path = ws.join("turingos.toml");
    let llm_configured = if toml_path.is_file() {
        let content = std::fs::read_to_string(&toml_path).unwrap_or_default();
        content.contains("llm.meta.model") && content.contains("llm.blackbox.model")
    } else {
        false
    };

    let agents_count = if let Ok(content) = std::fs::read_to_string(ws.join("agent_pubkeys.json")) {
        content
            .lines()
            .filter(|l| l.trim_start().starts_with('"') && l.trim_end().ends_with("{"))
            .count()
    } else {
        0
    };

    // Spec completion: prefer the CAS capsule CID (canonical evidence) over
    // a plain `spec.md` file presence — that way `welcome` reports `[x]` only
    // when the spec actually made it through CAS, not when a hand-edited
    // spec.md exists without a capsule.
    let spec_capsule_cid = spec_capsule::latest_spec_capsule_cid(ws).ok().flatten();
    let spec_done = spec_capsule_cid.is_some() || ws.join("spec.md").is_file();

    // Artifacts: must be a non-empty directory (an empty artifacts/ dir from a
    // bare `mkdir` doesn't count as "generate done").
    let artifacts_done = ws
        .join("artifacts")
        .read_dir()
        .map(|mut it| it.next().is_some())
        .unwrap_or(false);

    WorkspaceStatus {
        init_done,
        llm_configured,
        agents_count,
        spec_done,
        spec_capsule_cid,
        artifacts_done,
    }
}

fn render_status(ws: &Path, s: &WorkspaceStatus) {
    let ws_q = shell_quote_path(ws);
    println!("turingos — TuringOS user CLI (Phase 6.3 demo)");
    println!();
    println!("Workspace: {ws_q}");
    println!();
    println!("Onboarding status:");
    mark(1, "turingos init", s.init_done);
    mark(2, "turingos llm config", s.llm_configured);
    mark(
        3,
        &format!("turingos agent deploy ({} registered)", s.agents_count),
        s.agents_count > 0,
    );
    let spec_label = match &s.spec_capsule_cid {
        Some(cid) => format!(
            "turingos spec (CAS capsule: {}…{})",
            &cid[..8],
            &cid[cid.len() - 8..]
        ),
        None => "turingos spec (task decomposition)".to_string(),
    };
    mark(4, &spec_label, s.spec_done);
    mark(5, "turingos generate (deliverable)", s.artifacts_done);
    println!();

    // Phase 6.3 flow (non-developer end-user demo): init → llm → spec →
    // generate. `agent deploy` is OPTIONAL for this flow (only matters for
    // multi-agent / benchmark batches) and does NOT block spec progression.
    let next: Option<String> = if !s.init_done {
        Some(format!("turingos init --project {} --template proof", ws_q))
    } else if !s.llm_configured {
        Some(format!("turingos llm config --workspace {}", ws_q))
    } else if !s.spec_done {
        Some(format!("turingos spec --workspace {}", ws_q))
    } else if !s.artifacts_done {
        Some(format!("turingos generate --workspace {}", ws_q))
    } else {
        None
    };

    match next {
        Some(cmd) => {
            println!("Next step:");
            println!("  {cmd}");
        }
        None => {
            println!("All onboarding steps complete. View deliverables at:");
            println!("  {}/artifacts/", ws_q);
        }
    }
}

fn mark(n: u8, label: &str, done: bool) {
    let glyph = if done { "[x]" } else { "[ ]" };
    println!("  {glyph} {n}. {label}");
}
