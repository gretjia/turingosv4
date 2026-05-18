//! TRACE_MATRIX FC2-N16: turingos llm handler (Phase 6.3 LLM credential setup)
//!
//! Class 1 filesystem write to workspace-local turingos.toml. Stores the
//! two-LLM configuration (Meta = reasoning model; Blackbox = fast model)
//! per the dual-model architecture from the project's tri-model-coexecution
//! research.
//!
//! Phase 6.3: defaults to SiliconFlow (硅基流动) — selected after independent
//! research-agent comparison of available providers (DeepSeek / Qwen / GLM /
//! Kimi / MiniMax). Picks:
//!   - Meta AI (reasoning): deepseek-ai/DeepSeek-V3.2
//!   - Blackbox AI (fast):  Qwen/Qwen3-Coder-30B-A3B-Instruct
//! Cost: ~¥0.45 per game-build session at default traffic.
//!
//! API key value is NEVER stored on disk — only the env-var NAME holding it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::common::shell_quote_path;
use crate::siliconflow_client::{DEFAULT_BLACKBOX_MODEL, DEFAULT_META_MODEL};

/// TRACE_MATRIX FC2-N16: `llm` short-help
pub(crate) const SHORT_HELP: &str =
    "Configure the two-LLM setup (Meta = reasoning; Blackbox = fast). Defaults to SiliconFlow";

/// TRACE_MATRIX FC2-N16: `llm` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos llm — Configure the two-LLM setup

USAGE:
    turingos llm config --workspace <PATH>
                        [--provider siliconflow]
                        [--meta-model <MODEL>]
                        [--blackbox-model <MODEL>]
                        [--api-key-env <ENV_VAR>]
    turingos llm show   --workspace <PATH>

ACTIONS:
    config    Persist the two-LLM config to <workspace>/turingos.toml.
              All flags are OPTIONAL: defaults are SiliconFlow + the two
              researched-recommended models (Meta=DeepSeek-V3.2,
              Blackbox=Qwen3-Coder-30B). Pass flags only to override.

    show      Display the current LLM config (env-var NAMES only — never
              prints the actual API key value).

OPTIONS:
    --workspace <PATH>            Workspace directory (required).
    --provider <NAME>             Provider id. Default: siliconflow.
    --meta-model <ID>             Reasoning model id.
                                  Default: deepseek-ai/DeepSeek-V3.2
    --blackbox-model <ID>         Fast / codegen model id.
                                  Default: Qwen/Qwen3-Coder-30B-A3B-Instruct
    --api-key-env <ENV>           Env var holding the API key (single var
                                  for both models when both use the same
                                  provider). Default: SILICONFLOW_API_KEY.
                                  Value is NEVER persisted to disk.

DESCRIPTION:
    Two-LLM architecture rationale: a reasoning model ("Meta AI") handles
    spec decomposition and customer-development-style interviewing; a fast
    model ("Blackbox AI") handles high-volume code generation. The split
    is per the project's tri-model-coexecution research and the Anthropic
    multi-agent research-system pattern.

    Phase 6.3 defaults are pinned after an independent research-agent
    survey of SiliconFlow's model lineup (2026-05-17). To use a different
    provider (e.g., Anthropic, OpenAI, DeepSeek direct), set
    TURINGOS_SILICONFLOW_ENDPOINT to that provider's OpenAI-compatible
    Chat Completions URL — the wire format is the same.

    Class 1: filesystem write only. No network. No backend call.
"#;

#[derive(Debug)]
enum LlmError {
    MissingAction,
    UnknownAction(String),
    MissingFlag(&'static str),
    WorkspaceNotFound(String),
    Io(String),
}

impl LlmError {
    fn exit_code(&self) -> u8 {
        match self {
            Self::Io(_) => 2,
            _ => 1,
        }
    }
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingAction => write!(f, "missing action (config|show)"),
            Self::UnknownAction(a) => write!(f, "unknown action: {a}"),
            Self::MissingFlag(flag) => write!(f, "missing required flag: {flag}"),
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::Io(e) => write!(f, "i/o error: {e}"),
        }
    }
}

/// TRACE_MATRIX FC2-N16: `llm` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help")
        && (args.len() == 1 || args[0] == "-h" || args[0] == "--help")
    {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos llm: {e}");
            ExitCode::from(e.exit_code())
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), LlmError> {
    let mut workspace = PathBuf::from(".");
    let mut provider = "siliconflow".to_string();
    let mut meta_model = DEFAULT_META_MODEL.to_string();
    let mut blackbox_model = DEFAULT_BLACKBOX_MODEL.to_string();
    let mut api_key_env = "SILICONFLOW_API_KEY".to_string();
    let mut positional: Vec<String> = Vec::new();

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                workspace = PathBuf::from(iter.next().ok_or(LlmError::MissingFlag("--workspace"))?);
            }
            "--provider" => {
                provider = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--provider"))?
                    .clone();
            }
            "--meta-model" => {
                meta_model = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--meta-model"))?
                    .clone();
            }
            "--blackbox-model" => {
                blackbox_model = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--blackbox-model"))?
                    .clone();
            }
            "--api-key-env" => {
                api_key_env = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--api-key-env"))?
                    .clone();
            }
            "-h" | "--help" => {}
            _ => positional.push(arg.clone()),
        }
    }

    if !workspace.exists() {
        return Err(LlmError::WorkspaceNotFound(workspace.display().to_string()));
    }

    let action = positional.first().ok_or(LlmError::MissingAction)?.clone();
    match action.as_str() {
        "config" => {
            write_config(
                &workspace,
                &provider,
                &meta_model,
                &blackbox_model,
                &api_key_env,
            )?;
            let ws_q = shell_quote_path(&workspace);
            println!("LLM config written to {}/turingos.toml", ws_q);
            println!();
            println!("  Provider:                       {provider}");
            println!("  Meta AI       (reasoning):      {meta_model}");
            println!("  Blackbox AI   (fast/codegen):   {blackbox_model}");
            println!("  api-key-env (single, both):     {api_key_env}");
            println!();
            println!("Set your API key in the env var BEFORE running spec/generate:");
            println!("  export {api_key_env}=sk-...");
            println!();
            println!("Project convention: place the key in `.env` in the repo root (gitignored).");
            println!("Then `source .env` or run as: bash -c '. .env && turingos spec ...'");
            println!();
            println!("Next step: turingos spec --workspace {}", ws_q);
            Ok(())
        }
        "show" => {
            let entries = read_config(&workspace)?;
            if entries.is_empty() {
                println!(
                    "(no LLM config in {}/turingos.toml — run `turingos llm config ...` first)",
                    shell_quote_path(&workspace)
                );
            } else {
                for (k, v) in entries {
                    if k.starts_with("llm.") {
                        println!("{k} = {v:?}");
                    }
                }
            }
            Ok(())
        }
        other => Err(LlmError::UnknownAction(other.to_string())),
    }
}

fn write_config(
    workspace: &Path,
    provider: &str,
    meta_model: &str,
    blackbox_model: &str,
    api_key_env: &str,
) -> Result<(), LlmError> {
    let path = workspace.join("turingos.toml");
    let mut existing = read_config(workspace)?;
    let mut set = |k: &str, v: &str| {
        if let Some(e) = existing.iter_mut().find(|(ek, _)| ek == k) {
            e.1 = v.to_string();
        } else {
            existing.push((k.to_string(), v.to_string()));
        }
    };
    set("llm.provider", provider);
    set("llm.meta.model", meta_model);
    set("llm.meta.api_key_env", api_key_env);
    set("llm.blackbox.model", blackbox_model);
    set("llm.blackbox.api_key_env", api_key_env);

    let mut out =
        String::from("# turingos.toml — managed by `turingos config` / `turingos llm config`\n");
    for (k, v) in &existing {
        out.push_str(&format!("{k} = \"{v}\"\n"));
    }
    fs::write(&path, out).map_err(|e| LlmError::Io(e.to_string()))?;
    Ok(())
}

fn read_config(workspace: &Path) -> Result<Vec<(String, String)>, LlmError> {
    let path = workspace.join("turingos.toml");
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(&path).map_err(|e| LlmError::Io(e.to_string()))?;
    let mut out = Vec::new();
    for line in content.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        if let Some(eq) = t.find('=') {
            let k = t[..eq].trim().to_string();
            let v = t[eq + 1..].trim().trim_matches('"').to_string();
            out.push((k, v));
        }
    }
    Ok(out)
}

/// TRACE_MATRIX FC2-N16: Meta-model lookup from turingos.toml (with default fallback).
///
/// Read the configured Meta model from turingos.toml. Used by `turingos spec`.
/// Falls back to the Phase 6.3 default if unset (e.g., when user skipped
/// `turingos llm config` and went straight to spec).
pub(crate) fn read_meta_model(workspace: &Path) -> String {
    read_config_value(workspace, "llm.meta.model").unwrap_or_else(|| DEFAULT_META_MODEL.to_string())
}

/// TRACE_MATRIX FC2-N16: Blackbox-model lookup from turingos.toml (with default fallback).
///
/// Read the configured Blackbox model from turingos.toml. Used by `turingos generate`.
pub(crate) fn read_blackbox_model(workspace: &Path) -> String {
    read_config_value(workspace, "llm.blackbox.model")
        .unwrap_or_else(|| DEFAULT_BLACKBOX_MODEL.to_string())
}

/// TRACE_MATRIX FC2-N16: env-var NAME lookup (never the key value).
///
/// Read the configured api-key env-var NAME (e.g. "SILICONFLOW_API_KEY") —
/// NOT the key value. Defaults to SILICONFLOW_API_KEY if unset.
pub(crate) fn read_api_key_env_var(workspace: &Path) -> String {
    read_config_value(workspace, "llm.meta.api_key_env")
        .or_else(|| read_config_value(workspace, "llm.blackbox.api_key_env"))
        .unwrap_or_else(|| "SILICONFLOW_API_KEY".to_string())
}

fn read_config_value(workspace: &Path, key: &str) -> Option<String> {
    read_config(workspace)
        .ok()?
        .into_iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v)
}
