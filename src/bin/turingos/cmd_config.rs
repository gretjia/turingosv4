//! TRACE_MATRIX FC2-N16: turingos config handler (workspace-local TOML key-value config)

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// TRACE_MATRIX FC2-N16: `config` short-help (registry display)
pub(crate) const SHORT_HELP: &str = "Manage workspace-local turingos.toml config (set/get/list)";

/// TRACE_MATRIX FC2-N16: `config` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos config — Manage workspace-local config

USAGE:
    turingos config <set|get|list> [OPTIONS] [<key> [<value>]]

ACTIONS:
    set <key> <value>    Set a config key (creates or updates turingos.toml).
    get <key>            Print a config value (exits non-zero if key missing).
    list                 List all config keys + values.

OPTIONS:
    --workspace <PATH>   Workspace directory (default: current directory).
    -h, --help           Print this help.

DESCRIPTION:
    Reads/writes `<workspace>/turingos.toml`. Simple key = "value" format,
    one entry per line. No external TOML crate dependency.

    Class 1 filesystem-only. No sequencer call. No CAS write. No ChainTape
    advance.
"#;

#[derive(Debug)]
enum ConfigError {
    MissingAction,
    UnknownAction(String),
    MissingKey,
    MissingValue,
    UnknownKey(String),
    InvalidKey(String),
    InvalidValue(String),
    WorkspaceNotFound(String),
    Io(String),
}

impl ConfigError {
    fn exit_code(&self) -> u8 {
        match self {
            Self::Io(_) => 2,
            _ => 1,
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAction => write!(f, "missing action (set|get|list)"),
            Self::UnknownAction(a) => write!(f, "unknown action: {a}"),
            Self::MissingKey => write!(f, "missing <key>"),
            Self::MissingValue => write!(f, "missing <value>"),
            Self::UnknownKey(k) => write!(f, "key not found: {k}"),
            Self::InvalidKey(k) => write!(
                f,
                "invalid key (must be ASCII alphanumeric + _ + . + -): {k}"
            ),
            Self::InvalidValue(v) => {
                write!(
                    f,
                    "invalid value (must not contain newlines or quotes): {v}"
                )
            }
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::Io(e) => write!(f, "i/o error: {e}"),
        }
    }
}

/// TRACE_MATRIX FC2-N16: `config` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.len() == 1 && (args[0] == "-h" || args[0] == "--help") {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos config: {e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), ConfigError> {
    let mut workspace = PathBuf::from(".");
    let mut positional: Vec<String> = Vec::new();
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                let v = iter.next().ok_or(ConfigError::MissingValue)?;
                workspace = PathBuf::from(v);
            }
            "-h" | "--help" => {}
            _ => positional.push(arg.clone()),
        }
    }
    if !workspace.exists() {
        return Err(ConfigError::WorkspaceNotFound(
            workspace.display().to_string(),
        ));
    }
    let config_path = workspace.join("turingos.toml");

    let action = positional
        .first()
        .ok_or(ConfigError::MissingAction)?
        .clone();
    match action.as_str() {
        "set" => {
            let key = positional.get(1).ok_or(ConfigError::MissingKey)?.clone();
            let value = positional.get(2).ok_or(ConfigError::MissingValue)?.clone();
            validate_key(&key)?;
            validate_value(&value)?;
            set_key(&config_path, &key, &value)?;
            println!("set {key} = {value:?}");
            Ok(())
        }
        "get" => {
            let key = positional.get(1).ok_or(ConfigError::MissingKey)?.clone();
            let entries = read_entries(&config_path)?;
            let v = entries
                .iter()
                .find(|(k, _)| *k == key)
                .ok_or_else(|| ConfigError::UnknownKey(key.clone()))?
                .1
                .clone();
            println!("{v}");
            Ok(())
        }
        "list" => {
            let entries = read_entries(&config_path)?;
            for (k, v) in entries {
                println!("{k} = {v:?}");
            }
            Ok(())
        }
        other => Err(ConfigError::UnknownAction(other.to_string())),
    }
}

fn validate_key(k: &str) -> Result<(), ConfigError> {
    if k.is_empty()
        || !k
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-')
    {
        return Err(ConfigError::InvalidKey(k.to_string()));
    }
    Ok(())
}

fn validate_value(v: &str) -> Result<(), ConfigError> {
    if v.contains('\n') || v.contains('"') {
        return Err(ConfigError::InvalidValue(v.to_string()));
    }
    Ok(())
}

fn read_entries(path: &Path) -> Result<Vec<(String, String)>, ConfigError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path).map_err(|e| ConfigError::Io(e.to_string()))?;
    let mut out = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(eq) = trimmed.find('=') {
            let key = trimmed[..eq].trim().to_string();
            let value_part = trimmed[eq + 1..].trim();
            let value = value_part
                .trim_start_matches('"')
                .trim_end_matches('"')
                .to_string();
            out.push((key, value));
        }
    }
    Ok(out)
}

fn set_key(path: &Path, key: &str, value: &str) -> Result<(), ConfigError> {
    let mut entries = read_entries(path)?;
    if let Some(e) = entries.iter_mut().find(|(k, _)| k == key) {
        e.1 = value.to_string();
    } else {
        entries.push((key.to_string(), value.to_string()));
    }
    let mut out = String::from("# turingos.toml — managed by `turingos config`\n");
    for (k, v) in &entries {
        out.push_str(&format!("{k} = \"{v}\"\n"));
    }
    fs::write(path, out).map_err(|e| ConfigError::Io(e.to_string()))?;
    Ok(())
}
