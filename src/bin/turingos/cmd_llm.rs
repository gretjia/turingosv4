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
//!
//! Phase 6.3.x W4: adds `complete` sub-action — thin async LLM call wrapper
//! with PromptCapsule CAS anchoring and optional strict-JSON envelope
//! validation via `grill_envelope::parse_and_validate`.

use std::collections::BTreeSet;
use std::fs;
use std::io::Read as IoRead;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Instant;

use sha2::{Digest, Sha256};

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
    turingos llm complete
                        --workspace <PATH>
                        [--role <meta|blackbox>]
                        [--prompt-file <PATH|-]
                        [--max-tokens <N>]
                        [--temperature <FLOAT>]
                        [--capsule-dir <PATH>]
                        [--turn-id <STRING>]
                        [--strict-json]
                        [--lang <zh|en>]
                        [--meta-prompt <PATH>]

ACTIONS:
    config    Persist the two-LLM config to <workspace>/turingos.toml.
              All flags are OPTIONAL: defaults are SiliconFlow + the two
              researched-recommended models (Meta=DeepSeek-V3.2,
              Blackbox=Qwen3-Coder-30B). Pass flags only to override.

    show      Display the current LLM config (env-var NAMES only — never
              prints the actual API key value).

    complete  Call the LLM with a prompt-file (JSON messages array) and
              print a single JSON result line to stdout. Optionally anchors
              a PromptCapsule in CAS (--capsule-dir + --turn-id) and
              validates the LLM output as a grill TurnPayload (--strict-json).
              Phase 6.3.x W4 atom.

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
    --role <meta|blackbox>        Which model role to use. Default: meta.
    --prompt-file <PATH|->        JSON file (or - for stdin) with messages array.
    --max-tokens <N>              Override max tokens. Default: 2000 (meta) | 400 (blackbox).
    --temperature <FLOAT>         Override temperature. Default: 0.4 (meta) | 0.2 (blackbox).
    --capsule-dir <PATH>          If set, write PromptCapsule to CAS here.
    --turn-id <STRING>            Required if --capsule-dir is set.
    --strict-json                 Validate output via grill_envelope::parse_and_validate.
    --lang <zh|en>                Error message language. Default: zh.
    --meta-prompt <PATH>          Meta-prompt asset path (informational; recorded in capsule).
                                  Default: assets/prompts/grill_meta_v1.md.

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
    // No args at all → print help (lists all actions including `complete`).
    if args.is_empty() {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    if args.iter().any(|a| a == "-h" || a == "--help")
        && (args.len() == 1 || args[0] == "-h" || args[0] == "--help")
    {
        println!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    // W4: dispatch `complete` action before the existing config/show path
    // (which also looks for a positional first arg) so we don't need to
    // thread a new enum variant through LlmError.
    if args.first().map(String::as_str) == Some("complete") {
        return run_complete(&args[1..]);
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

// ─── W4: `turingos llm complete` ─────────────────────────────────────────────
//
// TRACE_MATRIX FC1-N44 + FC2-N16: thin async LLM call wrapper with optional
// PromptCapsule CAS anchoring (Phase 6.3.x grill-driven atom W4).
//
// R2 §A1 hard rule: NO AttemptTelemetry write for grill turns.
// R2 §A2 hard rule: hidden_fields_redacted MUST be true (constructor enforces).

/// TRACE_MATRIX FC2-N16 W4: parsed CLI args for `turingos llm complete`.
struct CompleteArgs {
    workspace: PathBuf,
    role: ModelRole,
    prompt_file: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    capsule_dir: Option<PathBuf>,
    turn_id: Option<String>,
    strict_json: bool,
    lang: Lang,
    /// Informational only in W4 — recorded as system_prompt_template_hash.
    meta_prompt: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
enum ModelRole {
    Meta,
    Blackbox,
}

#[derive(Debug, Clone, Copy)]
enum Lang {
    Zh,
    En,
}

impl Lang {
    fn io_err_msg(&self, detail: &str) -> String {
        match self {
            Lang::Zh => format!("IO 错误: {detail}"),
            Lang::En => format!("io error: {detail}"),
        }
    }
    fn args_err_msg(&self, detail: &str) -> String {
        match self {
            Lang::Zh => format!("参数错误: {detail}"),
            Lang::En => format!("args error: {detail}"),
        }
    }
    fn http_err_msg(&self, detail: &str) -> String {
        match self {
            Lang::Zh => format!("HTTP 错误: {detail}"),
            Lang::En => format!("http error: {detail}"),
        }
    }
    fn parse_err_msg(&self, detail: &str) -> String {
        match self {
            Lang::Zh => format!("解析失败: {detail}"),
            Lang::En => format!("parse failed: {detail}"),
        }
    }
}

/// Prompt-file JSON messages item (serialised/deserialised).
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct PromptMessage {
    role: String,
    content: String,
}

/// Top-level prompt-file JSON format accepted by `--prompt-file`.
#[derive(Debug, serde::Deserialize)]
struct PromptFile {
    messages: Vec<PromptMessage>,
    #[serde(default)]
    max_tokens: Option<u32>,
    #[serde(default)]
    temperature: Option<f32>,
}

/// Success JSON shape printed to stdout on `complete` success.
#[derive(serde::Serialize)]
struct CompleteOk {
    ok: bool,
    content: String,
    parsed_envelope: Option<serde_json::Value>,
    usage: UsageOut,
    finish_reason: String,
    model: String,
    prompt_capsule_cid: Option<String>,
    elapsed_ms: u128,
}

/// Error JSON shape printed to stdout on `complete` failure.
#[derive(serde::Serialize)]
struct CompleteErr {
    ok: bool,
    error: ErrorBody,
}

#[derive(serde::Serialize)]
struct ErrorBody {
    kind: &'static str,
    detail: String,
}

/// Token-usage sub-object in success JSON.
#[derive(serde::Serialize, Default)]
struct UsageOut {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

/// Print error JSON to stdout and return the given exit code.
fn complete_err_exit(kind: &'static str, detail: String, code: u8) -> ExitCode {
    let out = CompleteErr {
        ok: false,
        error: ErrorBody { kind, detail },
    };
    println!("{}", serde_json::to_string(&out).unwrap());
    ExitCode::from(code)
}

/// TRACE_MATRIX FC2-N16 W4: parse `complete` CLI args.
fn parse_complete_args(args: &[String]) -> Result<CompleteArgs, String> {
    let mut workspace: Option<PathBuf> = None;
    let mut role = ModelRole::Meta;
    let mut prompt_file: Option<String> = None;
    let mut max_tokens: Option<u32> = None;
    let mut temperature: Option<f32> = None;
    let mut capsule_dir: Option<PathBuf> = None;
    let mut turn_id: Option<String> = None;
    let mut strict_json = false;
    let mut lang = Lang::Zh;
    let mut meta_prompt: Option<PathBuf> = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                workspace = Some(PathBuf::from(
                    iter.next().ok_or("--workspace requires a value")?,
                ));
            }
            "--role" => {
                let v = iter.next().ok_or("--role requires a value")?;
                role = match v.as_str() {
                    "meta" => ModelRole::Meta,
                    "blackbox" => ModelRole::Blackbox,
                    other => {
                        return Err(format!("unknown --role: {other}; expected meta|blackbox"))
                    }
                };
            }
            "--prompt-file" => {
                prompt_file = Some(iter.next().ok_or("--prompt-file requires a value")?.clone());
            }
            "--max-tokens" => {
                let v = iter.next().ok_or("--max-tokens requires a value")?;
                max_tokens =
                    Some(v.parse::<u32>().map_err(|_| {
                        format!("--max-tokens must be a positive integer, got: {v}")
                    })?);
            }
            "--temperature" => {
                let v = iter.next().ok_or("--temperature requires a value")?;
                temperature = Some(
                    v.parse::<f32>()
                        .map_err(|_| format!("--temperature must be a float, got: {v}"))?,
                );
            }
            "--capsule-dir" => {
                capsule_dir = Some(PathBuf::from(
                    iter.next().ok_or("--capsule-dir requires a value")?,
                ));
            }
            "--turn-id" => {
                turn_id = Some(iter.next().ok_or("--turn-id requires a value")?.clone());
            }
            "--strict-json" => {
                strict_json = true;
            }
            "--lang" => {
                let v = iter.next().ok_or("--lang requires a value")?;
                lang = match v.as_str() {
                    "zh" => Lang::Zh,
                    "en" => Lang::En,
                    other => return Err(format!("unknown --lang: {other}; expected zh|en")),
                };
            }
            "--meta-prompt" => {
                meta_prompt = Some(PathBuf::from(
                    iter.next().ok_or("--meta-prompt requires a value")?,
                ));
            }
            "-h" | "--help" => {
                println!("{FULL_HELP}");
            }
            other => {
                return Err(format!("unknown flag: {other}"));
            }
        }
    }

    let workspace = workspace.ok_or("--workspace is required")?;
    if capsule_dir.is_some() && turn_id.is_none() {
        return Err("--turn-id is required when --capsule-dir is set".to_string());
    }

    Ok(CompleteArgs {
        workspace,
        role,
        prompt_file,
        max_tokens,
        temperature,
        capsule_dir,
        turn_id,
        strict_json,
        lang,
        meta_prompt,
    })
}

/// TRACE_MATRIX FC2-N16 W4: sha256 a byte slice and return as `[u8; 32]`.
fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

/// TRACE_MATRIX FC2-N16 W4: `turingos llm complete` entry.
///
/// Performs a single LLM call via `siliconflow_client::chat_complete`,
/// optionally validates the JSON envelope via `grill_envelope::parse_and_validate`,
/// optionally writes a `PromptCapsule` to CAS, and prints one JSON result line.
///
/// Exit codes:
///   0 = ok
///   2 = http/network error
///   3 = parse failed (--strict-json + envelope invalid)
///   4 = io error (missing file, unreadable workspace)
///   5 = invalid CLI args
fn run_complete(args: &[String]) -> ExitCode {
    // ── 1. Parse CLI args ───────────────────────────────────────────────────
    let ca = match parse_complete_args(args) {
        Ok(v) => v,
        Err(e) => return complete_err_exit("args", ca_lang_args_err(e, args), 5),
    };

    // ── 2. Workspace existence check ────────────────────────────────────────
    if !ca.workspace.exists() {
        return complete_err_exit(
            "io",
            ca.lang
                .io_err_msg(&format!("workspace not found: {}", ca.workspace.display())),
            4,
        );
    }

    // ── 3. Read prompt file ─────────────────────────────────────────────────
    let prompt_json_str: String = match &ca.prompt_file {
        None => {
            return complete_err_exit("args", ca.lang.args_err_msg("--prompt-file is required"), 5);
        }
        Some(p) if p == "-" => {
            let mut buf = String::new();
            if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
                return complete_err_exit(
                    "io",
                    ca.lang.io_err_msg(&format!("reading stdin: {e}")),
                    4,
                );
            }
            buf
        }
        Some(p) => match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                return complete_err_exit(
                    "io",
                    ca.lang.io_err_msg(&format!("reading prompt file {p}: {e}")),
                    4,
                );
            }
        },
    };

    let prompt_file_data: PromptFile = match serde_json::from_str(&prompt_json_str) {
        Ok(v) => v,
        Err(e) => {
            return complete_err_exit(
                "io",
                ca.lang
                    .io_err_msg(&format!("prompt file JSON parse error: {e}")),
                4,
            );
        }
    };

    // ── 4. Determine model + defaults ───────────────────────────────────────
    let (default_max_tokens, default_temperature, model_id) = match ca.role {
        ModelRole::Meta => (2000u32, 0.4f32, read_meta_model(&ca.workspace)),
        ModelRole::Blackbox => (400u32, 0.2f32, read_blackbox_model(&ca.workspace)),
    };

    // CLI flags override file values; file values override defaults.
    let max_tokens = ca
        .max_tokens
        .or(prompt_file_data.max_tokens)
        .unwrap_or(default_max_tokens);
    let temperature = ca
        .temperature
        .or(prompt_file_data.temperature)
        .unwrap_or(default_temperature);

    // ── 5. Read API key ─────────────────────────────────────────────────────
    let api_key_env = read_api_key_env_var(&ca.workspace);
    let api_key = match crate::siliconflow_client::require_api_key(&api_key_env) {
        Ok(k) => k,
        Err(e) => {
            return complete_err_exit("http_status", ca.lang.http_err_msg(&e.to_string()), 2);
        }
    };

    // ── 6. Convert messages to ChatMessage ──────────────────────────────────
    let chat_messages: Vec<crate::siliconflow_client::ChatMessage> = prompt_file_data
        .messages
        .iter()
        .map(|m| crate::siliconflow_client::ChatMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    // ── 7. LLM call (async → blocking via tokio current-thread runtime) ─────
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            return complete_err_exit(
                "http_status",
                ca.lang.http_err_msg(&format!("tokio runtime: {e}")),
                2,
            );
        }
    };

    let t_start = Instant::now();
    let llm_result = rt.block_on(crate::siliconflow_client::chat_complete(
        &api_key,
        &model_id,
        &chat_messages,
        Some(max_tokens),
        Some(temperature),
    ));
    let elapsed_ms = t_start.elapsed().as_millis();

    let chat_result = match llm_result {
        Ok(r) => r,
        Err(crate::siliconflow_client::LlmError::HttpStatus { status, body }) => {
            return complete_err_exit(
                "http_status",
                ca.lang.http_err_msg(&format!("HTTP {status}: {body}")),
                2,
            );
        }
        Err(crate::siliconflow_client::LlmError::Transport(e)) => {
            return complete_err_exit("timeout", ca.lang.http_err_msg(&e), 2);
        }
        Err(e) => {
            return complete_err_exit("http_status", ca.lang.http_err_msg(&e.to_string()), 2);
        }
    };

    // ── 8. Strict-JSON validation ────────────────────────────────────────────
    let parsed_envelope: Option<serde_json::Value> = if ca.strict_json {
        match turingosv4::runtime::grill_envelope::parse_and_validate(&chat_result.content) {
            Ok(tp) => Some(serde_json::to_value(&tp).unwrap_or(serde_json::Value::Null)),
            Err(e) => {
                return complete_err_exit("parse_failed", ca.lang.parse_err_msg(&e.to_string()), 3);
            }
        }
    } else {
        None
    };

    // ── 9. PromptCapsule CAS write ──────────────────────────────────────────
    let prompt_capsule_cid: Option<String> =
        if let (Some(capsule_dir), Some(turn_id)) = (&ca.capsule_dir, &ca.turn_id) {
            match write_prompt_capsule_for_turn(
                &ca.workspace,
                capsule_dir,
                turn_id,
                &prompt_file_data.messages,
                &prompt_json_str,
                ca.strict_json,
                &ca.meta_prompt,
                &ca.lang,
            ) {
                Ok(cid_hex) => Some(cid_hex),
                Err(e) => {
                    return complete_err_exit("io", e, 4);
                }
            }
        } else {
            None
        };

    // ── 10. Print success JSON ───────────────────────────────────────────────
    let ok_out = CompleteOk {
        ok: true,
        content: chat_result.content,
        parsed_envelope,
        usage: UsageOut {
            prompt_tokens: chat_result.usage.prompt_tokens,
            completion_tokens: chat_result.usage.completion_tokens,
            total_tokens: chat_result.usage.total_tokens,
        },
        finish_reason: chat_result
            .finish_reason
            .unwrap_or_else(|| "stop".to_string()),
        model: model_id,
        prompt_capsule_cid,
        elapsed_ms,
    };
    println!("{}", serde_json::to_string(&ok_out).unwrap());
    ExitCode::SUCCESS
}

/// Helper: produce an args-error detail string using the lang embedded in
/// the raw args (before full parse). Falls back to zh.
fn ca_lang_args_err(detail: String, args: &[String]) -> String {
    let is_en = args.windows(2).any(|w| w[0] == "--lang" && w[1] == "en");
    if is_en {
        format!("args error: {detail}")
    } else {
        format!("参数错误: {detail}")
    }
}

/// TRACE_MATRIX FC1-N44 W4: write a PromptCapsule to CAS for one grill turn.
///
/// Computes `prompt_context_hash` and `visible_context_cid` from the canonical
/// JSON of the message array, builds a `PromptCapsule` with
/// `hidden_fields_redacted = true` (R2 §A2 hard rule), and stores it via
/// `write_prompt_capsule_to_cas`. Returns the CID hex string.
///
/// `meta_prompt_path` is informational (R2 §A7): if present, sha256 of the
/// file's content becomes `system_prompt_template_hash`; if absent we fall
/// back to sha256 of the first system-role message content; if no system
/// message exists we use the zero sentinel.
#[allow(clippy::too_many_arguments)]
fn write_prompt_capsule_for_turn(
    workspace: &Path,
    capsule_dir: &Path,
    turn_id: &str,
    messages: &[PromptMessage],
    messages_json_str: &str,
    strict_json: bool,
    meta_prompt_path: &Option<PathBuf>,
    lang: &Lang,
) -> Result<String, String> {
    use turingosv4::bottom_white::cas::schema::Cid;
    use turingosv4::bottom_white::cas::store::CasStore;
    use turingosv4::runtime::prompt_capsule::{write_prompt_capsule_to_cas, PromptCapsule};
    use turingosv4::state::q_state::Hash;

    // Ensure capsule_dir exists.
    fs::create_dir_all(capsule_dir).map_err(|e| {
        lang.io_err_msg(&format!(
            "creating capsule-dir {}: {e}",
            capsule_dir.display()
        ))
    })?;

    // CAS lives in <capsule_dir>/cas/ (or workspace/cas/ — we use capsule_dir
    // so callers can point it at the session-local capsule store).
    let cas_dir = capsule_dir.join("cas");
    let mut cas = CasStore::open(&cas_dir)
        .map_err(|e| lang.io_err_msg(&format!("opening CAS at {}: {e}", cas_dir.display())))?;

    // --- visible_context_cid: CAS-store the raw message-array JSON bytes.
    // We re-serialize only the messages slice (not the full prompt file) so
    // the hash is stable regardless of other prompt-file fields.
    let msg_bytes = serde_json::to_vec(messages)
        .map_err(|e| lang.io_err_msg(&format!("serialising messages: {e}")))?;

    let visible_context_cid = cas
        .put(
            &msg_bytes,
            turingosv4::bottom_white::cas::schema::ObjectType::EvidenceCapsule,
            &format!("cmd_llm_complete/{turn_id}"),
            0,
            Some("messages-array-v1".to_string()),
        )
        .map_err(|e| lang.io_err_msg(&format!("CAS put messages: {e}")))?;

    // --- prompt_context_hash: sha256 of the same canonical message-array bytes.
    let prompt_context_hash_bytes = sha256_bytes(&msg_bytes);

    // --- system_prompt_template_hash: prefer --meta-prompt file sha256;
    //     else sha256 of first system message content; else zero sentinel.
    let system_hash_bytes: [u8; 32] = if let Some(mp_path) = meta_prompt_path {
        let resolved = if mp_path.is_absolute() {
            mp_path.clone()
        } else {
            workspace.join(mp_path)
        };
        match fs::read(&resolved) {
            Ok(bytes) => sha256_bytes(&bytes),
            Err(e) => {
                return Err(lang.io_err_msg(&format!(
                    "reading --meta-prompt {}: {e}",
                    resolved.display()
                )));
            }
        }
    } else {
        // Fall back to sha256 of first system message content.
        messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| sha256_bytes(m.content.as_bytes()))
            .unwrap_or([0u8; 32])
    };

    // --- Build PromptCapsule (hidden_fields_redacted MUST be true — R2 §A2).
    let policy_version = if strict_json {
        "grill_meta_v1"
    } else {
        "complete_v1"
    };
    let capsule = PromptCapsule::new(
        Hash(prompt_context_hash_bytes),
        BTreeSet::new(), // empty read_set for Phase 6.3.x v1
        policy_version,
        true, // hidden_fields_redacted = TRUE (R2 §A2 hard rule)
        visible_context_cid,
        Hash(system_hash_bytes),
        visible_context_cid, // agent_view_manifest_cid = same as visible_context_cid (v1)
    )
    .map_err(|e| lang.io_err_msg(&format!("building PromptCapsule: {e}")))?;

    let logical_t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let capsule_cid = write_prompt_capsule_to_cas(&mut cas, &capsule, turn_id, logical_t)
        .map_err(|e| lang.io_err_msg(&format!("writing PromptCapsule to CAS: {e}")))?;

    // Suppress unused-variable warning for messages_json_str (informational
    // only in W4; W5/W6 will use it).
    let _ = messages_json_str;

    Ok(capsule_cid.hex())
}
