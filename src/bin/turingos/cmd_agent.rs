//! TRACE_MATRIX FC2-N16: turingos agent handler (workspace agent_pubkeys.json)

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// TRACE_MATRIX FC2-N16: `agent` short-help (registry display)
pub(crate) const SHORT_HELP: &str =
    "Manage agent_pubkeys.json (deploy/list/view; map agent_id -> {pubkey, role})";

/// TRACE_MATRIX FC2-N16: `agent` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos agent — Manage workspace agent registry

USAGE:
    turingos agent <deploy|list|view> [OPTIONS]

ACTIONS:
    deploy --id <ID> --pubkey <64HEX> --role <ROLE>
                          Add or update an agent registration entry.
    list                  List all deployed agent IDs + roles.
    view --id <ID>        Show full record for one agent.

OPTIONS:
    --workspace <PATH>    Workspace directory (default: current directory).
    --id <ID>             Agent identifier (ASCII alnum + `_` + `-`).
    --pubkey <HEX>        ed25519 public key as 64-char hex.
    --role <ROLE>         One of (REAL-5/REAL-12 AgentRole variants):
                              Solver, Verifier, Challenger, Trader,
                              MarketMaker, Architect, Veto, Observer,
                              BullTrader, BearTrader.
    -h, --help            Print this help.

DESCRIPTION:
    Reads/writes `<workspace>/agent_pubkeys.json`. Minimal hand-rolled JSON
    serializer (no external serde_json dependency). Class 1 filesystem-only.

    No sequencer call. No CAS write. No ChainTape advance. No on-chain
    signature verification (that lives in the sequencer admission path,
    not here).
"#;

const VALID_ROLES: &[&str] = &[
    "Solver",
    "Verifier",
    "Challenger",
    "Trader",
    "MarketMaker",
    "Architect",
    "Veto",
    "Observer",
    "BullTrader",
    "BearTrader",
];

#[derive(Debug)]
enum AgentError {
    MissingAction,
    UnknownAction(String),
    MissingFlag(&'static str),
    InvalidId(String),
    InvalidPubkey(String),
    InvalidRole(String),
    AgentNotFound(String),
    WorkspaceNotFound(String),
    Io(String),
}

impl AgentError {
    fn exit_code(&self) -> u8 {
        match self {
            Self::Io(_) => 2,
            _ => 1,
        }
    }
}

impl std::fmt::Display for AgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAction => write!(f, "missing action (deploy|list|view)"),
            Self::UnknownAction(a) => write!(f, "unknown action: {a}"),
            Self::MissingFlag(flag) => write!(f, "missing required flag: {flag}"),
            Self::InvalidId(id) => write!(
                f,
                "invalid agent id (must be ASCII alphanumeric + `_` + `-`): {id}"
            ),
            Self::InvalidPubkey(k) => {
                write!(f, "invalid pubkey (must be exactly 64 hex characters): {k}")
            }
            Self::InvalidRole(r) => write!(
                f,
                "invalid role (must be one of the 10 AgentRole variants): {r}"
            ),
            Self::AgentNotFound(id) => write!(f, "agent not found: {id}"),
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::Io(e) => write!(f, "i/o error: {e}"),
        }
    }
}

/// TRACE_MATRIX FC2-N16: `agent` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos agent: {e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), AgentError> {
    let mut workspace = PathBuf::from(".");
    let mut id: Option<String> = None;
    let mut pubkey: Option<String> = None;
    let mut role: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                workspace =
                    PathBuf::from(iter.next().ok_or(AgentError::MissingFlag("--workspace"))?);
            }
            "--id" => {
                id = Some(iter.next().ok_or(AgentError::MissingFlag("--id"))?.clone());
            }
            "--pubkey" => {
                pubkey = Some(
                    iter.next()
                        .ok_or(AgentError::MissingFlag("--pubkey"))?
                        .clone(),
                );
            }
            "--role" => {
                role = Some(
                    iter.next()
                        .ok_or(AgentError::MissingFlag("--role"))?
                        .clone(),
                );
            }
            "-h" | "--help" => {}
            _ => positional.push(arg.clone()),
        }
    }
    if !workspace.exists() {
        return Err(AgentError::WorkspaceNotFound(
            workspace.display().to_string(),
        ));
    }
    let pubkeys_path = workspace.join("agent_pubkeys.json");

    let action = positional.first().ok_or(AgentError::MissingAction)?.clone();
    match action.as_str() {
        "deploy" => {
            let id = id.ok_or(AgentError::MissingFlag("--id"))?;
            let pubkey = pubkey.ok_or(AgentError::MissingFlag("--pubkey"))?;
            let role = role.ok_or(AgentError::MissingFlag("--role"))?;
            validate_id(&id)?;
            validate_pubkey(&pubkey)?;
            validate_role(&role)?;
            let mut entries = read_entries(&pubkeys_path)?;
            if let Some(e) = entries.iter_mut().find(|(eid, _, _)| *eid == id) {
                e.1 = pubkey.clone();
                e.2 = role.clone();
            } else {
                entries.push((id.clone(), pubkey.clone(), role.clone()));
            }
            write_entries(&pubkeys_path, &entries)?;
            println!("deployed {id} ({role})");
            Ok(())
        }
        "list" => {
            let entries = read_entries(&pubkeys_path)?;
            for (id, pubkey, role) in entries {
                let prefix: String = pubkey.chars().take(12).collect();
                println!("{id:<24} {role:<14} {prefix}...");
            }
            Ok(())
        }
        "view" => {
            let id = id.ok_or(AgentError::MissingFlag("--id"))?;
            let entries = read_entries(&pubkeys_path)?;
            let entry = entries
                .iter()
                .find(|(eid, _, _)| *eid == id)
                .ok_or_else(|| AgentError::AgentNotFound(id.clone()))?;
            println!("id      = {}", entry.0);
            println!("role    = {}", entry.2);
            println!("pubkey  = {}", entry.1);
            Ok(())
        }
        other => Err(AgentError::UnknownAction(other.to_string())),
    }
}

fn validate_id(id: &str) -> Result<(), AgentError> {
    if id.is_empty()
        || !id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        return Err(AgentError::InvalidId(id.to_string()));
    }
    Ok(())
}

fn validate_pubkey(k: &str) -> Result<(), AgentError> {
    if k.len() != 64 || !k.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AgentError::InvalidPubkey(k.to_string()));
    }
    Ok(())
}

fn validate_role(r: &str) -> Result<(), AgentError> {
    if !VALID_ROLES.iter().any(|v| *v == r) {
        return Err(AgentError::InvalidRole(r.to_string()));
    }
    Ok(())
}

fn read_entries(path: &Path) -> Result<Vec<(String, String, String)>, AgentError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path).map_err(|e| AgentError::Io(e.to_string()))?;
    let mut out = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_pubkey: Option<String> = None;
    let mut current_role: Option<String> = None;
    for line in content.lines() {
        let trimmed = line.trim().trim_end_matches(',');
        if trimmed.is_empty() || trimmed == "{" || trimmed == "}" {
            continue;
        }
        if let Some(stripped) = trimmed.strip_suffix("{") {
            let id_part = stripped.trim().trim_end_matches(':').trim();
            let id = id_part.trim_matches('"').to_string();
            current_id = Some(id);
            current_pubkey = None;
            current_role = None;
            continue;
        }
        if trimmed.starts_with("\"pubkey\"") {
            let v = trimmed
                .split(':')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
            current_pubkey = v;
        } else if trimmed.starts_with("\"role\"") {
            let v = trimmed
                .split(':')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').to_string());
            current_role = v;
        }
        if let (Some(id), Some(pk), Some(r)) = (&current_id, &current_pubkey, &current_role) {
            out.push((id.clone(), pk.clone(), r.clone()));
            current_id = None;
            current_pubkey = None;
            current_role = None;
        }
    }
    Ok(out)
}

fn write_entries(path: &Path, entries: &[(String, String, String)]) -> Result<(), AgentError> {
    let mut out = String::from("{\n");
    for (i, (id, pubkey, role)) in entries.iter().enumerate() {
        out.push_str(&format!("  \"{id}\": {{\n"));
        out.push_str(&format!("    \"pubkey\": \"{pubkey}\",\n"));
        out.push_str(&format!("    \"role\": \"{role}\"\n"));
        if i + 1 < entries.len() {
            out.push_str("  },\n");
        } else {
            out.push_str("  }\n");
        }
    }
    out.push_str("}\n");
    fs::write(path, out).map_err(|e| AgentError::Io(e.to_string()))?;
    Ok(())
}
