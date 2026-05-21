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
use crate::siliconflow_client::{ThinkingConfig, DEFAULT_BLACKBOX_MODEL, DEFAULT_META_MODEL};
use turingosv4::runtime::prompt_promotion::{check_promotion_guard, sha256_hex_of_prompt, PromotionGuardError};

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
    turingos llm triage
                        --workspace <PATH>
                        --user-answer <STRING|-]
                        [--question <STRING>]
                        [--lang <zh|en>]
                        [--capsule-dir <PATH>]
                        [--turn-id <STRING>]
    turingos llm prompt-eval
                        --workspace <PATH>
                        --prompt-file <PATH>
                        --role <meta|blackbox|playback>
                        --fixture <PATH>
                        [--meta-prompt <PATH>]
                        [--baseline-prompt <PATH>]
                        [--lang <zh|en>]

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

    triage    Classify a user answer using the Blackbox model (Qwen3-Coder-30B).
              Outputs a single JSON line with class (relevant|off_topic|abusive|
              gibberish) and confidence. Always uses Blackbox model.
              Phase 6.3.x W4.5 atom.

    prompt-eval  Regression-test a candidate prompt against a frozen Q/A
              fixture. Iterates the fixture rows, calls the appropriate LLM
              role (meta|blackbox|playback), and scores each output against
              the expected verdict. Exits 0 if all rows pass, 1 if any fail.
              Phase 6.3.y A2 atom — catches M8-class non-local regressions
              (e.g. fixing register tolerance breaks gibberish detection).

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
    --user-answer <STRING|->      (triage) User answer text, or - for stdin.
    --question <STRING>           (triage) Question context for the classifier.
    --fixture <PATH>              (prompt-eval) JSONL fixture file path.
    --baseline-prompt <PATH>      (prompt-eval) Optional baseline prompt for delta computation.

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

DEEPSEEK DUAL-KEY EXAMPLE:
    # 1. Configure key env-var NAMES (not values) + models + thinking mode
    turingos llm config --workspace <PATH> \
        --meta-api-key-env DEEPSEEK_API_KEY \
        --blackbox-api-key-env DEEPSEEK_API_KEY_WORKER \
        --meta-model deepseek-v4-pro \
        --blackbox-model deepseek-v4-flash \
        --meta-thinking on \
        --blackbox-thinking off

    # 2. Point endpoint at DeepSeek (shell env var, not in turingos.toml)
    export TURINGOS_SILICONFLOW_ENDPOINT=https://api.deepseek.com/v1/chat/completions

    # 3. Provide the actual key values via the named env vars
    export DEEPSEEK_API_KEY="sk-..."          # Meta role key
    export DEEPSEEK_API_KEY_WORKER="sk-..."   # Worker role key

    # 4. Verify
    turingos welcome --workspace <PATH>

ANTHROPIC EXAMPLE (Meta=Claude reasoning + Worker=Claude fast):
    # NOTE: Anthropic native API requires a different wire protocol than
    # SiliconFlow/DeepSeek/OpenAI. As of this version, TuringOS only speaks
    # OpenAI-compatible Chat Completions. To use Claude with TuringOS today,
    # point TURINGOS_SILICONFLOW_ENDPOINT at an OpenAI-compatible Anthropic
    # gateway (e.g. OpenRouter, or self-hosted LiteLLM proxy).

    turingos llm config --workspace <PATH> \
        --meta-api-key-env OPENROUTER_API_KEY \
        --blackbox-api-key-env OPENROUTER_API_KEY \
        --meta-model anthropic/claude-opus-4-1 \
        --blackbox-model anthropic/claude-haiku-4-5 \
        --meta-thinking on \
        --blackbox-thinking off

    export OPENROUTER_API_KEY="sk-or-..."
    export TURINGOS_SILICONFLOW_ENDPOINT=https://openrouter.ai/api/v1/chat/completions

OPENAI EXAMPLE (single key, dual model):
    turingos llm config --workspace <PATH> \
        --meta-api-key-env OPENAI_API_KEY \
        --blackbox-api-key-env OPENAI_API_KEY \
        --meta-model gpt-5 \
        --blackbox-model gpt-5-mini \
        --meta-thinking off \
        --blackbox-thinking off

    export OPENAI_API_KEY="sk-..."
    export TURINGOS_SILICONFLOW_ENDPOINT=https://api.openai.com/v1/chat/completions
"#;

/// TRACE_MATRIX FC2-N16: error type for LLM config/key readers in cmd_llm.
#[derive(Debug)]
pub(crate) enum LlmError {
    MissingAction,
    UnknownAction(String),
    MissingFlag(&'static str),
    WorkspaceNotFound(String),
    Io(String),
    MetaKeyEnvNotConfigured,
    BlackboxKeyEnvNotConfigured,
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
            Self::MetaKeyEnvNotConfigured => write!(
                f,
                "llm.meta.api_key_env is not set in turingos.toml. \
                 Run: turingos llm config --meta-api-key-env <ENV_VAR_NAME>"
            ),
            Self::BlackboxKeyEnvNotConfigured => write!(
                f,
                "llm.blackbox.api_key_env is not set in turingos.toml. \
                 Run: turingos llm config --blackbox-api-key-env <ENV_VAR_NAME>"
            ),
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
    // W4.5: dispatch `triage` action (Blackbox classifier).
    if args.first().map(String::as_str) == Some("triage") {
        return run_triage(&args[1..]);
    }
    // A2: dispatch `prompt-eval` action (regression-test prompts against a
    // frozen Q/A fixture before promoting v2 → v1). Phase 6.3.y atom — catches
    // M8-class non-local regressions where fixing register tolerance breaks
    // gibberish detection.
    if args.first().map(String::as_str) == Some("prompt-eval") {
        // C10: if --from and --to and --eval-set are ALL present, this is a
        // promotion receipt write (not a live fixture eval). Dispatch separately.
        let is_promote = args.iter().any(|a| a == "--from")
            && args.iter().any(|a| a == "--to")
            && args.iter().any(|a| a == "--eval-set");
        if is_promote {
            return run_prompt_promote(&args[1..]);
        }
        return run_prompt_eval(&args[1..]);
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
    let mut meta_thinking: Option<String> = None;
    let mut blackbox_thinking: Option<String> = None;
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
            "--meta-thinking" => {
                let v = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--meta-thinking"))?
                    .to_lowercase();
                if v != "on" && v != "off" {
                    return Err(LlmError::UnknownAction(format!(
                        "--meta-thinking value must be 'on' or 'off', got: {v}"
                    )));
                }
                meta_thinking = Some(v);
            }
            "--blackbox-thinking" => {
                let v = iter
                    .next()
                    .ok_or(LlmError::MissingFlag("--blackbox-thinking"))?
                    .to_lowercase();
                if v != "on" && v != "off" {
                    return Err(LlmError::UnknownAction(format!(
                        "--blackbox-thinking value must be 'on' or 'off', got: {v}"
                    )));
                }
                blackbox_thinking = Some(v);
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
                meta_thinking.as_deref(),
                blackbox_thinking.as_deref(),
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
    meta_thinking: Option<&str>,
    blackbox_thinking: Option<&str>,
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
    if let Some(t) = meta_thinking {
        set("llm.meta.thinking", t);
    }
    if let Some(t) = blackbox_thinking {
        set("llm.blackbox.thinking", t);
    }

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

/// TRACE_MATRIX FC2-N16: Meta env-var NAME lookup (never the key value).
///
/// Reads ONLY `llm.meta.api_key_env` from turingos.toml. Returns an error if
/// the key is absent — does NOT fall back to the blackbox slot, preventing
/// silent role-key sharing when users configure separate Meta and Worker keys.
pub(crate) fn read_meta_api_key_env(workspace: &Path) -> Result<String, LlmError> {
    read_config_value(workspace, "llm.meta.api_key_env")
        .ok_or(LlmError::MetaKeyEnvNotConfigured)
}

/// TRACE_MATRIX FC2-N16: Blackbox env-var NAME lookup (never the key value).
///
/// Reads ONLY `llm.blackbox.api_key_env` from turingos.toml. Returns an error
/// if the key is absent — does NOT fall back to the meta slot, preventing
/// silent role-key sharing when users configure separate Meta and Worker keys.
pub(crate) fn read_blackbox_api_key_env(workspace: &Path) -> Result<String, LlmError> {
    read_config_value(workspace, "llm.blackbox.api_key_env")
        .ok_or(LlmError::BlackboxKeyEnvNotConfigured)
}

/// TRACE_MATRIX FC2-N16: Meta thinking-mode config reader.
///
/// Returns `Some(ThinkingConfig { kind: "enabled" })` when `llm.meta.thinking = "on"` in
/// turingos.toml, and `None` otherwise (including when the key is absent, which defaults to off).
pub(crate) fn read_meta_thinking(workspace: &Path) -> Option<ThinkingConfig> {
    match read_config_value(workspace, "llm.meta.thinking").as_deref() {
        Some("on") => Some(ThinkingConfig {
            kind: "enabled".to_string(),
        }),
        _ => None,
    }
}

/// TRACE_MATRIX FC2-N16: Blackbox thinking-mode config reader.
///
/// Returns `Some(ThinkingConfig { kind: "enabled" })` when `llm.blackbox.thinking = "on"` in
/// turingos.toml, and `None` otherwise (including when the key is absent, which defaults to off).
pub(crate) fn read_blackbox_thinking(workspace: &Path) -> Option<ThinkingConfig> {
    match read_config_value(workspace, "llm.blackbox.thinking").as_deref() {
        Some("on") => Some(ThinkingConfig {
            kind: "enabled".to_string(),
        }),
        _ => None,
    }
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
    let api_key_env = match ca.role {
        ModelRole::Meta => match read_meta_api_key_env(&ca.workspace) {
            Ok(v) => v,
            Err(e) => {
                return complete_err_exit("config", ca.lang.http_err_msg(&e.to_string()), 1);
            }
        },
        ModelRole::Blackbox => match read_blackbox_api_key_env(&ca.workspace) {
            Ok(v) => v,
            Err(e) => {
                return complete_err_exit("config", ca.lang.http_err_msg(&e.to_string()), 1);
            }
        },
    };
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

    let thinking = match ca.role {
        ModelRole::Meta => read_meta_thinking(&ca.workspace),
        ModelRole::Blackbox => read_blackbox_thinking(&ca.workspace),
    };

    let t_start = Instant::now();
    let llm_result = rt.block_on(crate::siliconflow_client::chat_complete(
        &api_key,
        &model_id,
        &chat_messages,
        Some(max_tokens),
        Some(temperature),
        thinking,
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
    // F2: thinking-mode models (DeepSeek-V3.1-Terminus think-on, DeepSeek-R1,
    // Qwen3-8B/14B/32B with default-on thinking, GLM-4.7, Kimi-K2.5/K2.6 when
    // thinking enabled) emit a <think>...</think> reasoning trace before the
    // JSON envelope. The grill envelope parser cannot tolerate that prefix, so
    // we strip think blocks BEFORE handing the content to
    // `grill_envelope::parse_and_validate`. Uses the shared iterative helper
    // from `sdk::protocol` (handles multiple blocks, anywhere in the string,
    // and unclosed blocks by truncating at the unclosed tag).
    //
    // Providers that emit reasoning in `message.reasoning_content` (separate
    // from `content`) are handled implicitly: `siliconflow_client::ChatResponse`
    // deserializes only `content`, so `reasoning_content` is dropped at decode
    // time and never reaches this branch.
    let parsed_envelope: Option<serde_json::Value> = if ca.strict_json {
        let stripped = turingosv4::sdk::protocol::strip_think_blocks(&chat_result.content);
        match turingosv4::runtime::grill_envelope::parse_and_validate(stripped.trim()) {
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

// ─── W4.5: `turingos llm triage` ─────────────────────────────────────────────
//
// TRACE_MATRIX FC2-N16 W4.5: thin Blackbox classifier wrapper.
//
// Uses the Blackbox model (Qwen3-Coder-30B) to classify one user answer into
// {relevant, off_topic, abusive, gibberish} with a confidence float.
// Always uses Blackbox model (read_blackbox_model). No --role flag.
//
// R2 §A5: Blackbox triage is in-scope for Phase 6.3.x packet as W4.5.
// R2 §A1: NO AttemptTelemetry write for grill turns.
// R2 §A2: hidden_fields_redacted MUST be true.

/// TRACE_MATRIX FC2-N16 W4.5: parsed CLI args for `turingos llm triage`.
struct TriageArgs {
    workspace: PathBuf,
    user_answer: String,
    question: Option<String>,
    lang: Lang,
    capsule_dir: Option<PathBuf>,
    turn_id: Option<String>,
}

/// Success JSON shape printed to stdout on `triage` success.
#[derive(serde::Serialize)]
struct TriageOk {
    ok: bool,
    class: String,
    confidence: f64,
    model: String,
    usage: UsageOut,
    prompt_capsule_cid: Option<String>,
    elapsed_ms: u128,
}

/// Blackbox response JSON shape expected from the model.
#[derive(serde::Deserialize)]
struct BlackboxClassification {
    class: String,
    confidence: f64,
}

/// The four valid classification classes (W4.5 spec).
const VALID_TRIAGE_CLASSES: &[&str] = &["relevant", "off_topic", "abusive", "gibberish"];

/// TRACE_MATRIX FC2-N16 W4.5: parse `triage` CLI args.
fn parse_triage_args(args: &[String]) -> Result<TriageArgs, String> {
    let mut workspace: Option<PathBuf> = None;
    let mut user_answer: Option<String> = None;
    let mut question: Option<String> = None;
    let mut lang = Lang::Zh;
    let mut capsule_dir: Option<PathBuf> = None;
    let mut turn_id: Option<String> = None;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                workspace = Some(PathBuf::from(
                    iter.next().ok_or("--workspace requires a value")?,
                ));
            }
            "--user-answer" => {
                user_answer = Some(iter.next().ok_or("--user-answer requires a value")?.clone());
            }
            "--question" => {
                question = Some(iter.next().ok_or("--question requires a value")?.clone());
            }
            "--lang" => {
                let v = iter.next().ok_or("--lang requires a value")?;
                lang = match v.as_str() {
                    "zh" => Lang::Zh,
                    "en" => Lang::En,
                    other => return Err(format!("unknown --lang: {other}; expected zh|en")),
                };
            }
            "--capsule-dir" => {
                capsule_dir = Some(PathBuf::from(
                    iter.next().ok_or("--capsule-dir requires a value")?,
                ));
            }
            "--turn-id" => {
                turn_id = Some(iter.next().ok_or("--turn-id requires a value")?.clone());
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
    let user_answer = user_answer.ok_or("--user-answer is required")?;
    if capsule_dir.is_some() && turn_id.is_none() {
        return Err("--turn-id is required when --capsule-dir is set".to_string());
    }

    Ok(TriageArgs {
        workspace,
        user_answer,
        question,
        lang,
        capsule_dir,
        turn_id,
    })
}

/// TRACE_MATRIX FC2-N16 W4.5: `turingos llm triage` entry.
///
/// Classifies a user answer using the Blackbox model. Reads the system prompt
/// verbatim from `assets/prompts/grill_triage_blackbox_v1.md`, constructs
/// the user message, calls `chat_complete` with max_tokens=50 temperature=0.0,
/// parses the classification JSON, optionally writes a PromptCapsule to CAS,
/// and prints one JSON result line.
///
/// Exit codes:
///   0 = ok
///   2 = http/network error
///   3 = parse failed (Blackbox output not conforming)
///   4 = io error
///   5 = invalid CLI args
fn run_triage(args: &[String]) -> ExitCode {
    // ── 1. Parse CLI args ───────────────────────────────────────────────────
    let ta = match parse_triage_args(args) {
        Ok(v) => v,
        Err(e) => return complete_err_exit("args", triage_lang_args_err(e, args), 5),
    };

    // ── 2. Workspace existence check ────────────────────────────────────────
    if !ta.workspace.exists() {
        return complete_err_exit(
            "io",
            ta.lang
                .io_err_msg(&format!("workspace not found: {}", ta.workspace.display())),
            4,
        );
    }

    // ── 3. Read user answer (stdin if "-") ──────────────────────────────────
    let user_answer: String = if ta.user_answer == "-" {
        let mut buf = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut buf) {
            return complete_err_exit("io", ta.lang.io_err_msg(&format!("reading stdin: {e}")), 4);
        }
        buf
    } else {
        ta.user_answer.clone()
    };

    // ── 4. Load triage system prompt from asset file ─────────────────────────
    // Path: assets/prompts/grill_triage_blackbox_v1.md (resolved from workspace).
    // Extract text block under "## System prompt (verbatim)" between ``` fences.
    let triage_prompt_path = {
        let p = PathBuf::from("assets/prompts/grill_triage_blackbox_v1.md");
        if p.is_absolute() {
            p
        } else {
            ta.workspace.join(&p)
        }
    };
    let triage_asset = match fs::read_to_string(&triage_prompt_path) {
        Ok(s) => s,
        Err(e) => {
            return complete_err_exit(
                "io",
                ta.lang.io_err_msg(&format!(
                    "reading triage prompt asset {}: {e}",
                    triage_prompt_path.display()
                )),
                4,
            );
        }
    };
    let system_prompt_text = extract_system_prompt_block(&triage_asset);
    let system_prompt_text = match system_prompt_text {
        Some(s) => s,
        None => {
            return complete_err_exit(
                "io",
                ta.lang.io_err_msg(
                    "could not locate '## System prompt (verbatim)' block in triage asset",
                ),
                4,
            );
        }
    };

    // ── 5. Build messages ───────────────────────────────────────────────────
    let question_text = ta
        .question
        .as_deref()
        .unwrap_or("(initial open-ended question)");
    let user_message_text =
        format!("QUESTION (turn N): {question_text}\n\nUSER ANSWER:\n{user_answer}");

    let chat_messages = vec![
        crate::siliconflow_client::ChatMessage::system(system_prompt_text.clone()),
        crate::siliconflow_client::ChatMessage::user(user_message_text.clone()),
    ];

    // ── 6. Determine Blackbox model + API key ───────────────────────────────
    let model_id = read_blackbox_model(&ta.workspace);
    let api_key_env = match read_blackbox_api_key_env(&ta.workspace) {
        Ok(v) => v,
        Err(e) => {
            return complete_err_exit("config", ta.lang.http_err_msg(&e.to_string()), 1);
        }
    };
    let api_key = match crate::siliconflow_client::require_api_key(&api_key_env) {
        Ok(k) => k,
        Err(e) => {
            return complete_err_exit("http_status", ta.lang.http_err_msg(&e.to_string()), 2);
        }
    };

    // C10 promotion guard: gate LLM startup on a CAS-anchored PromptPromotionReceipt.
    // Keyed on the canonical triage system prompt loaded from the asset file.
    // NoCasStore (uninitialized workspace) is allowed for dev ergonomics.
    let triage_prompt_cid = sha256_hex_of_prompt(system_prompt_text.as_bytes());
    match check_promotion_guard(&ta.workspace, &triage_prompt_cid) {
        Ok(()) => {}
        Err(PromotionGuardError::NoCasStore(_)) => {
            // Workspace CAS not yet initialized — allow startup for dev ergonomics.
        }
        Err(e) => {
            return complete_err_exit(
                "http_status",
                ta.lang.http_err_msg(&format!("promotion guard: {e}")),
                2,
            );
        }
    }

    // ── 7. LLM call (max_tokens=50, temperature=0.0) ────────────────────────
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            return complete_err_exit(
                "http_status",
                ta.lang.http_err_msg(&format!("tokio runtime: {e}")),
                2,
            );
        }
    };

    let blackbox_thinking = read_blackbox_thinking(&ta.workspace);

    let t_start = Instant::now();
    let llm_result = rt.block_on(crate::siliconflow_client::chat_complete(
        &api_key,
        &model_id,
        &chat_messages,
        Some(50),
        Some(0.0),
        blackbox_thinking,
    ));
    let elapsed_ms = t_start.elapsed().as_millis();

    let chat_result = match llm_result {
        Ok(r) => r,
        Err(crate::siliconflow_client::LlmError::HttpStatus { status, body }) => {
            return complete_err_exit(
                "http_status",
                ta.lang.http_err_msg(&format!("HTTP {status}: {body}")),
                2,
            );
        }
        Err(crate::siliconflow_client::LlmError::Transport(e)) => {
            return complete_err_exit("timeout", ta.lang.http_err_msg(&e), 2);
        }
        Err(e) => {
            return complete_err_exit("http_status", ta.lang.http_err_msg(&e.to_string()), 2);
        }
    };

    // ── 8. Parse and validate Blackbox classification response ───────────────
    // F2: strip ALL <think>...</think> blocks (not just split at last
    // </think>). The shared helper from `sdk::protocol` handles multiple
    // blocks, blocks anywhere in the string, and unclosed blocks (truncates
    // at the unclosed opener). The old `strip_thinking_wrapper` only looked
    // for a final </think>, which silently leaked content when an opener was
    // emitted without a closer.
    let stripped = turingosv4::sdk::protocol::strip_think_blocks(&chat_result.content);
    let json_str = stripped.trim();

    let classification: BlackboxClassification = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(e) => {
            return complete_err_exit(
                "parse_failed",
                ta.lang.parse_err_msg(&format!(
                    "Blackbox output not valid JSON: {e}; got: {json_str}"
                )),
                3,
            );
        }
    };

    // Validate class ∈ {relevant, off_topic, abusive, gibberish}.
    if !VALID_TRIAGE_CLASSES.contains(&classification.class.as_str()) {
        return complete_err_exit(
            "parse_failed",
            ta.lang.parse_err_msg(&format!(
                "class '{}' not in {{relevant, off_topic, abusive, gibberish}}",
                classification.class
            )),
            3,
        );
    }

    // Validate confidence ∈ [0.0, 1.0].
    if !(0.0..=1.0).contains(&classification.confidence) {
        return complete_err_exit(
            "parse_failed",
            ta.lang.parse_err_msg(&format!(
                "confidence {} not in [0.0, 1.0]",
                classification.confidence
            )),
            3,
        );
    }

    // ── 9. PromptCapsule CAS write ──────────────────────────────────────────
    // Build a minimal messages representation for the capsule (system + user).
    let capsule_messages = vec![
        PromptMessage {
            role: "system".to_string(),
            content: system_prompt_text,
        },
        PromptMessage {
            role: "user".to_string(),
            content: user_message_text,
        },
    ];
    let capsule_messages_json =
        serde_json::to_string(&capsule_messages).unwrap_or_else(|_| "[]".to_string());

    let prompt_capsule_cid: Option<String> =
        if let (Some(capsule_dir), Some(turn_id)) = (&ta.capsule_dir, &ta.turn_id) {
            match write_prompt_capsule_for_turn(
                &ta.workspace,
                capsule_dir,
                turn_id,
                &capsule_messages,
                &capsule_messages_json,
                false, // triage does not use strict-json envelope
                &None, // no meta-prompt for triage
                &ta.lang,
            ) {
                Ok(cid_hex) => Some(cid_hex),
                Err(e) => {
                    return complete_err_exit("io", e, 4);
                }
            }
        } else {
            None
        };

    // Override policy_version written by write_prompt_capsule_for_turn's
    // "complete_v1" default: triage capsules use "grill_triage_blackbox_v1".
    // NOTE: the PromptCapsule is already written above; we accept the
    // policy_version="complete_v1" for v1 (per W4 precedent). A future atom
    // can parameterize policy_version. This is documented here for audit.

    // ── 10. Print success JSON ───────────────────────────────────────────────
    let ok_out = TriageOk {
        ok: true,
        class: classification.class,
        confidence: classification.confidence,
        model: model_id,
        usage: UsageOut {
            prompt_tokens: chat_result.usage.prompt_tokens,
            completion_tokens: chat_result.usage.completion_tokens,
            total_tokens: chat_result.usage.total_tokens,
        },
        prompt_capsule_cid,
        elapsed_ms,
    };
    println!("{}", serde_json::to_string(&ok_out).unwrap());
    ExitCode::SUCCESS
}

/// Extract the verbatim system prompt text from the triage asset markdown.
///
/// Finds the "## System prompt (verbatim)" section and returns the text
/// between the first pair of ``` fences that follow it.
fn extract_system_prompt_block(asset: &str) -> Option<String> {
    // Find the section header.
    let header = "## System prompt (verbatim)";
    let section_start = asset.find(header)?;
    let after_header = &asset[section_start + header.len()..];

    // Find opening fence.
    let fence_open = after_header.find("```")?;
    let after_open_fence = &after_header[fence_open + 3..];

    // Skip optional language tag on the same line as the opening fence.
    let content_start = after_open_fence.find('\n').map(|i| i + 1).unwrap_or(0);
    let content = &after_open_fence[content_start..];

    // Find closing fence.
    let fence_close = content.find("```")?;
    Some(content[..fence_close].trim_end_matches('\n').to_string())
}

/// Helper: produce an args-error detail string using the lang embedded in
/// the raw args (before full parse) for the triage sub-command.
fn triage_lang_args_err(detail: String, args: &[String]) -> String {
    let is_en = args.windows(2).any(|w| w[0] == "--lang" && w[1] == "en");
    if is_en {
        format!("args error: {detail}")
    } else {
        format!("参数错误: {detail}")
    }
}

// ─── A2: `turingos llm prompt-eval` ──────────────────────────────────────────
//
// TRACE_MATRIX FC2-N16 A2 (Phase 6.3.y): prompt regression harness.
//
// Background. The Phase 6.3.x universality campaign discovered that F8 (Triage
// v2) fixed register tolerance (Cantonese / code-switch / Traditional → PASS)
// but **broke gibberish detection** (M8 REGRESSION: 3/5 nonsense → relevant).
// This is the canonical Software 3.0 non-local-effect failure mode: prompt
// edits cascade unpredictably, so v2 → v1 promotion is unsafe without a
// regression net. `prompt-eval` is that net.
//
// Surface contract. One JSONL fixture file, one --prompt-file candidate,
// one --role tag. For each fixture row we (a) build messages, (b) call the
// LLM via siliconflow_client (same retry / think-strip semantics as
// `complete` and `triage`), (c) score the response against expected_* fields,
// (d) aggregate per-row verdicts into a summary. Exit 0 iff all PASS.
//
// Risk class: 2 (production wire-up; reuses existing LLM client + parser).
// No Class-4 surface touched. No CAS write, no PromptCapsule write — eval is
// a read-only experiment.

/// TRACE_MATRIX FC2-N16 A2: parsed CLI args for `turingos llm prompt-eval`.
struct PromptEvalArgs {
    workspace: PathBuf,
    prompt_file: PathBuf,
    role: PromptEvalRole,
    fixture: PathBuf,
    meta_prompt: Option<PathBuf>,
    baseline_prompt: Option<PathBuf>,
    lang: Lang,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PromptEvalRole {
    /// Meta interviewer prompt — outputs grill TurnPayload JSON envelope.
    Meta,
    /// Blackbox triage classifier — outputs `{class, confidence}` JSON.
    Blackbox,
    /// Playback / mirror — outputs markdown 7-row fridge note.
    Playback,
}

impl PromptEvalRole {
    fn as_str(self) -> &'static str {
        match self {
            Self::Meta => "meta",
            Self::Blackbox => "blackbox",
            Self::Playback => "playback",
        }
    }
}

/// One fixture row, deserialised from one JSONL line. Field set is the union
/// of all three roles; per-role validation happens at scoring time.
#[derive(Debug, serde::Deserialize, Default)]
#[allow(dead_code)]
struct FixtureRow {
    id: String,
    #[serde(default)]
    tags: Vec<String>,
    // Common / triage / meta input fields.
    #[serde(default)]
    question: Option<String>,
    #[serde(default)]
    user_answer: Option<String>,
    // Meta input.
    #[serde(default)]
    history: Vec<HistoryItem>,
    // Triage expected.
    #[serde(default)]
    expected_class: Option<String>,
    #[serde(default)]
    expected_confidence_min: Option<f64>,
    // Meta expected.
    #[serde(default)]
    expected_covered_slots_subset: Option<Vec<String>>,
    #[serde(default)]
    expected_done: Option<bool>,
    // Playback input + expected.
    #[serde(default)]
    covered_slots_input: Option<serde_json::Value>,
    #[serde(default)]
    expected_no_substrings: Option<Vec<String>>,
    #[serde(default)]
    expected_contains_substrings: Option<Vec<String>>,
    #[serde(default)]
    notes: Option<String>,
}

#[derive(Debug, serde::Deserialize, Default)]
struct HistoryItem {
    #[serde(default)]
    q: Option<String>,
    #[serde(default)]
    a: Option<String>,
}

/// Per-row eval verdict, serialised to the output JSON.
#[derive(Debug, serde::Serialize)]
struct RowVerdict {
    id: String,
    verdict: String, // "PASS" | "FAIL" | "ERROR"
    role: String,
    expected: serde_json::Value,
    actual: serde_json::Value,
    notes: String,
}

/// Aggregate eval output written to stdout.
#[derive(Debug, serde::Serialize)]
struct PromptEvalOutput {
    ok: bool,
    role: String,
    prompt_file: String,
    fixture: String,
    total: usize,
    pass: usize,
    fail: usize,
    error: usize,
    fail_ids: Vec<String>,
    per_row: Vec<RowVerdict>,
    baseline_delta: Option<BaselineDelta>,
}

#[derive(Debug, serde::Serialize)]
struct BaselineDelta {
    baseline_prompt_file: String,
    baseline_pass: usize,
    baseline_fail: usize,
    candidate_pass: usize,
    candidate_fail: usize,
    /// Rows that pass on candidate but fail on baseline (improvements).
    gained_ids: Vec<String>,
    /// Rows that pass on baseline but fail on candidate (regressions — M8 cases).
    regressed_ids: Vec<String>,
}

/// TRACE_MATRIX FC2-N16 A2: parse `prompt-eval` CLI args.
fn parse_prompt_eval_args(args: &[String]) -> Result<PromptEvalArgs, String> {
    let mut workspace: Option<PathBuf> = None;
    let mut prompt_file: Option<PathBuf> = None;
    let mut role: Option<PromptEvalRole> = None;
    let mut fixture: Option<PathBuf> = None;
    let mut meta_prompt: Option<PathBuf> = None;
    let mut baseline_prompt: Option<PathBuf> = None;
    let mut lang = Lang::Zh;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--workspace" => {
                workspace = Some(PathBuf::from(
                    iter.next().ok_or("--workspace requires a value")?,
                ));
            }
            "--prompt-file" => {
                prompt_file = Some(PathBuf::from(
                    iter.next().ok_or("--prompt-file requires a value")?,
                ));
            }
            "--role" => {
                let v = iter.next().ok_or("--role requires a value")?;
                role = Some(match v.as_str() {
                    "meta" => PromptEvalRole::Meta,
                    "blackbox" => PromptEvalRole::Blackbox,
                    "playback" => PromptEvalRole::Playback,
                    other => {
                        return Err(format!(
                            "unknown --role: {other}; expected meta|blackbox|playback"
                        ));
                    }
                });
            }
            "--fixture" => {
                fixture = Some(PathBuf::from(
                    iter.next().ok_or("--fixture requires a value")?,
                ));
            }
            "--meta-prompt" => {
                meta_prompt = Some(PathBuf::from(
                    iter.next().ok_or("--meta-prompt requires a value")?,
                ));
            }
            "--baseline-prompt" => {
                baseline_prompt = Some(PathBuf::from(
                    iter.next().ok_or("--baseline-prompt requires a value")?,
                ));
            }
            "--lang" => {
                let v = iter.next().ok_or("--lang requires a value")?;
                lang = match v.as_str() {
                    "zh" => Lang::Zh,
                    "en" => Lang::En,
                    other => return Err(format!("unknown --lang: {other}; expected zh|en")),
                };
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
    let prompt_file = prompt_file.ok_or("--prompt-file is required")?;
    let role = role.ok_or("--role is required")?;
    let fixture = fixture.ok_or("--fixture is required")?;

    Ok(PromptEvalArgs {
        workspace,
        prompt_file,
        role,
        fixture,
        meta_prompt,
        baseline_prompt,
        lang,
    })
}

/// Helper: produce an args-error detail string using the lang embedded in
/// the raw args (before full parse) for the prompt-eval sub-command.
fn prompt_eval_lang_args_err(detail: String, args: &[String]) -> String {
    let is_en = args.windows(2).any(|w| w[0] == "--lang" && w[1] == "en");
    if is_en {
        format!("args error: {detail}")
    } else {
        format!("参数错误: {detail}")
    }
}

/// Load a prompt asset file. If the file contains a `## System prompt
/// (verbatim)` block (the convention for triage v1/v2 + meta v1/v2), extract
/// the verbatim text between the next pair of triple-backtick fences. Else
/// treat the whole file as the system prompt body.
fn load_prompt_system_text(path: &Path) -> Result<String, String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("reading prompt file {}: {e}", path.display()))?;
    if let Some(extracted) = extract_system_prompt_block(&raw) {
        Ok(extracted)
    } else {
        Ok(raw)
    }
}

/// Parse a JSONL fixture file. Blank lines and lines starting with `#` are
/// skipped (comments). Each remaining line must parse as `FixtureRow`.
fn load_fixture(path: &Path) -> Result<Vec<FixtureRow>, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("reading fixture {}: {e}", path.display()))?;
    let mut rows = Vec::new();
    for (lineno, raw) in content.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let row: FixtureRow = serde_json::from_str(trimmed).map_err(|e| {
            format!(
                "fixture {} line {}: invalid JSON: {e}",
                path.display(),
                lineno + 1
            )
        })?;
        if row.id.is_empty() {
            return Err(format!(
                "fixture {} line {}: row missing required `id` field",
                path.display(),
                lineno + 1
            ));
        }
        rows.push(row);
    }
    Ok(rows)
}

/// Print error JSON to stdout and exit. Re-uses the `CompleteErr` shape so
/// downstream JSON consumers can stay uniform.
fn prompt_eval_err_exit(kind: &'static str, detail: String, code: u8) -> ExitCode {
    let out = CompleteErr {
        ok: false,
        error: ErrorBody { kind, detail },
    };
    println!("{}", serde_json::to_string(&out).unwrap());
    ExitCode::from(code)
}

/// TRACE_MATRIX FC2-N16 A2: `turingos llm prompt-eval` entry.
///
/// Exit codes:
///   0 = all rows passed (and baseline-delta has no regressions if requested)
///   1 = one or more rows FAILED (or regressed vs baseline)
///   2 = http/network error during eval
///   4 = io error (missing fixture / prompt / workspace)
///   5 = invalid CLI args
fn run_prompt_eval(args: &[String]) -> ExitCode {
    // ── 1. Parse CLI args ───────────────────────────────────────────────────
    let pe = match parse_prompt_eval_args(args) {
        Ok(v) => v,
        Err(e) => {
            return prompt_eval_err_exit("args", prompt_eval_lang_args_err(e, args), 5);
        }
    };

    // ── 2. Workspace existence check ────────────────────────────────────────
    if !pe.workspace.exists() {
        return prompt_eval_err_exit(
            "io",
            pe.lang
                .io_err_msg(&format!("workspace not found: {}", pe.workspace.display())),
            4,
        );
    }

    // ── 3. Load candidate prompt + (optional) baseline prompt + fixture ─────
    let candidate_system_text = match load_prompt_system_text(&pe.prompt_file) {
        Ok(s) => s,
        Err(e) => return prompt_eval_err_exit("io", pe.lang.io_err_msg(&e), 4),
    };
    let baseline_system_text: Option<String> = if let Some(bp) = &pe.baseline_prompt {
        match load_prompt_system_text(bp) {
            Ok(s) => Some(s),
            Err(e) => return prompt_eval_err_exit("io", pe.lang.io_err_msg(&e), 4),
        }
    } else {
        None
    };
    let fixture_rows = match load_fixture(&pe.fixture) {
        Ok(r) => r,
        Err(e) => return prompt_eval_err_exit("io", pe.lang.io_err_msg(&e), 4),
    };
    if fixture_rows.is_empty() {
        return prompt_eval_err_exit(
            "io",
            pe.lang
                .io_err_msg(&format!("fixture {} is empty", pe.fixture.display())),
            4,
        );
    }

    // ── 4. Read API key + build tokio runtime once ──────────────────────────
    let api_key_env = match pe.role {
        PromptEvalRole::Meta | PromptEvalRole::Playback => {
            match read_meta_api_key_env(&pe.workspace) {
                Ok(v) => v,
                Err(e) => {
                    return prompt_eval_err_exit(
                        "config",
                        pe.lang.http_err_msg(&e.to_string()),
                        1,
                    );
                }
            }
        }
        PromptEvalRole::Blackbox => match read_blackbox_api_key_env(&pe.workspace) {
            Ok(v) => v,
            Err(e) => {
                return prompt_eval_err_exit("config", pe.lang.http_err_msg(&e.to_string()), 1);
            }
        },
    };
    let api_key = match crate::siliconflow_client::require_api_key(&api_key_env) {
        Ok(k) => k,
        Err(e) => {
            return prompt_eval_err_exit("http_status", pe.lang.http_err_msg(&e.to_string()), 2);
        }
    };

    // C10 promotion guard: gate LLM startup on a CAS-anchored PromptPromotionReceipt.
    // Keyed on the candidate system prompt bytes.
    // NoCasStore (uninitialized workspace) is allowed for dev ergonomics.
    let candidate_prompt_cid = sha256_hex_of_prompt(candidate_system_text.as_bytes());
    match check_promotion_guard(&pe.workspace, &candidate_prompt_cid) {
        Ok(()) => {}
        Err(PromotionGuardError::NoCasStore(_)) => {
            // Workspace CAS not yet initialized — allow startup for dev ergonomics.
        }
        Err(e) => {
            return prompt_eval_err_exit(
                "http_status",
                pe.lang.http_err_msg(&format!("promotion guard: {e}")),
                2,
            );
        }
    }

    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            return prompt_eval_err_exit(
                "http_status",
                pe.lang.http_err_msg(&format!("tokio runtime: {e}")),
                2,
            );
        }
    };

    // Pick model + budget per role.
    let (model_id, max_tokens, temperature) = match pe.role {
        PromptEvalRole::Blackbox => (read_blackbox_model(&pe.workspace), 50u32, 0.0f32),
        PromptEvalRole::Meta => (read_meta_model(&pe.workspace), 1500u32, 0.4f32),
        PromptEvalRole::Playback => (read_meta_model(&pe.workspace), 1200u32, 0.3f32),
    };

    // Read per-role thinking config once for the whole eval run.
    let thinking = match pe.role {
        PromptEvalRole::Meta | PromptEvalRole::Playback => read_meta_thinking(&pe.workspace),
        PromptEvalRole::Blackbox => read_blackbox_thinking(&pe.workspace),
    };

    // ── 5. Per-row eval loop ────────────────────────────────────────────────
    let mut per_row: Vec<RowVerdict> = Vec::with_capacity(fixture_rows.len());
    let mut pass_count = 0usize;
    let mut fail_count = 0usize;
    let mut error_count = 0usize;
    let mut fail_ids: Vec<String> = Vec::new();

    // Track baseline verdicts in parallel if a baseline prompt was given.
    let mut baseline_pass_ids: Vec<String> = Vec::new();
    let mut baseline_fail_ids: Vec<String> = Vec::new();

    for row in &fixture_rows {
        // Candidate eval.
        let verdict = eval_one_row(
            &rt,
            &api_key,
            &model_id,
            max_tokens,
            temperature,
            pe.role,
            &candidate_system_text,
            row,
            thinking.clone(),
        );
        match verdict.verdict.as_str() {
            "PASS" => pass_count += 1,
            "FAIL" => {
                fail_count += 1;
                fail_ids.push(row.id.clone());
            }
            _ => {
                error_count += 1;
                fail_ids.push(row.id.clone());
            }
        }
        per_row.push(verdict);

        // Baseline eval (only summary; not embedded in per_row to keep output compact).
        if let Some(baseline_text) = &baseline_system_text {
            let bverdict = eval_one_row(
                &rt,
                &api_key,
                &model_id,
                max_tokens,
                temperature,
                pe.role,
                baseline_text,
                row,
                thinking.clone(),
            );
            if bverdict.verdict == "PASS" {
                baseline_pass_ids.push(row.id.clone());
            } else {
                baseline_fail_ids.push(row.id.clone());
            }
        }
    }

    // ── 6. Optional baseline delta computation ──────────────────────────────
    let baseline_delta: Option<BaselineDelta> = if let Some(bp) = &pe.baseline_prompt {
        // Candidate pass set.
        let candidate_pass_set: std::collections::HashSet<String> = per_row
            .iter()
            .filter(|v| v.verdict == "PASS")
            .map(|v| v.id.clone())
            .collect();
        let baseline_pass_set: std::collections::HashSet<String> =
            baseline_pass_ids.iter().cloned().collect();
        let mut gained_ids: Vec<String> = candidate_pass_set
            .difference(&baseline_pass_set)
            .cloned()
            .collect();
        let mut regressed_ids: Vec<String> = baseline_pass_set
            .difference(&candidate_pass_set)
            .cloned()
            .collect();
        gained_ids.sort();
        regressed_ids.sort();
        Some(BaselineDelta {
            baseline_prompt_file: bp.display().to_string(),
            baseline_pass: baseline_pass_ids.len(),
            baseline_fail: baseline_fail_ids.len(),
            candidate_pass: pass_count,
            candidate_fail: fail_count + error_count,
            gained_ids,
            regressed_ids,
        })
    } else {
        None
    };

    // ── 7. Emit summary JSON + exit code ────────────────────────────────────
    let all_pass = fail_count == 0 && error_count == 0;
    let baseline_regressed = baseline_delta
        .as_ref()
        .map(|d| !d.regressed_ids.is_empty())
        .unwrap_or(false);

    let summary = PromptEvalOutput {
        ok: all_pass && !baseline_regressed,
        role: pe.role.as_str().to_string(),
        prompt_file: pe.prompt_file.display().to_string(),
        fixture: pe.fixture.display().to_string(),
        total: fixture_rows.len(),
        pass: pass_count,
        fail: fail_count,
        error: error_count,
        fail_ids,
        per_row,
        baseline_delta,
    };
    println!("{}", serde_json::to_string(&summary).unwrap());

    if summary.ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

/// Build messages, call the LLM, score the response, and return a per-row
/// verdict. This is the inner loop body of `run_prompt_eval`. It NEVER
/// panics on LLM/network errors — those become `verdict = "ERROR"`.
fn eval_one_row(
    rt: &tokio::runtime::Runtime,
    api_key: &str,
    model_id: &str,
    max_tokens: u32,
    temperature: f32,
    role: PromptEvalRole,
    system_text: &str,
    row: &FixtureRow,
    thinking: Option<ThinkingConfig>,
) -> RowVerdict {
    // ── (a) Build messages per role ─────────────────────────────────────────
    let messages: Vec<crate::siliconflow_client::ChatMessage> = match role {
        PromptEvalRole::Blackbox => {
            let q = row
                .question
                .as_deref()
                .unwrap_or("(initial open-ended question)");
            let a = row.user_answer.as_deref().unwrap_or("");
            let user_msg = format!("QUESTION (turn N): {q}\n\nUSER ANSWER:\n{a}");
            vec![
                crate::siliconflow_client::ChatMessage::system(system_text.to_string()),
                crate::siliconflow_client::ChatMessage::user(user_msg),
            ]
        }
        PromptEvalRole::Meta => {
            let mut msgs = vec![crate::siliconflow_client::ChatMessage::system(
                system_text.to_string(),
            )];
            // Replay history as alternating assistant(q) / user(a) pairs.
            for h in &row.history {
                if let Some(q) = &h.q {
                    msgs.push(crate::siliconflow_client::ChatMessage::assistant(q.clone()));
                }
                if let Some(a) = &h.a {
                    msgs.push(crate::siliconflow_client::ChatMessage::user(a.clone()));
                }
            }
            // The current user turn (the answer under test).
            if let Some(a) = &row.user_answer {
                msgs.push(crate::siliconflow_client::ChatMessage::user(a.clone()));
            }
            msgs.push(crate::siliconflow_client::ChatMessage::user(
                "Produce your next-turn output per the contract.".to_string(),
            ));
            msgs
        }
        PromptEvalRole::Playback => {
            // For playback: serialise covered_slots_input as JSON and ask the
            // model to produce the 7-row fridge note.
            let slots_json = row
                .covered_slots_input
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_else(|| "{}".to_string());
            let user_msg = format!(
                "Here are the covered slots from the interview (JSON):\n{slots_json}\n\nProduce the 7-row 'fridge note' playback in the user's language, ONLY summarising the slots above. Do NOT invent new content."
            );
            vec![
                crate::siliconflow_client::ChatMessage::system(system_text.to_string()),
                crate::siliconflow_client::ChatMessage::user(user_msg),
            ]
        }
    };

    // ── (b) LLM call ────────────────────────────────────────────────────────
    let llm_result = rt.block_on(crate::siliconflow_client::chat_complete(
        api_key,
        model_id,
        &messages,
        Some(max_tokens),
        Some(temperature),
        thinking,
    ));
    let chat_result = match llm_result {
        Ok(r) => r,
        Err(e) => {
            return RowVerdict {
                id: row.id.clone(),
                verdict: "ERROR".to_string(),
                role: role.as_str().to_string(),
                expected: serde_json::Value::Null,
                actual: serde_json::json!({"error": e.to_string()}),
                notes: format!("LLM call failed: {e}"),
            };
        }
    };

    // ── (c) Strip think blocks + score per role ─────────────────────────────
    let stripped = turingosv4::sdk::protocol::strip_think_blocks(&chat_result.content);
    let trimmed = stripped.trim();

    match role {
        PromptEvalRole::Blackbox => score_blackbox(row, trimmed),
        PromptEvalRole::Meta => score_meta(row, trimmed),
        PromptEvalRole::Playback => score_playback(row, trimmed),
    }
}

/// Score a Blackbox triage row: exact class match + confidence ≥ min.
fn score_blackbox(row: &FixtureRow, raw: &str) -> RowVerdict {
    let expected_class = row.expected_class.clone().unwrap_or_default();
    let expected_min_conf = row.expected_confidence_min.unwrap_or(0.0);
    let expected_json = serde_json::json!({
        "class": expected_class,
        "confidence_min": expected_min_conf,
    });

    // Parse the model output.
    let parsed: Result<BlackboxClassification, _> = serde_json::from_str(raw);
    match parsed {
        Ok(c) => {
            let actual_json = serde_json::json!({
                "class": c.class,
                "confidence": c.confidence,
            });
            let class_ok = c.class == expected_class;
            let conf_ok = c.confidence >= expected_min_conf;
            let verdict = if class_ok && conf_ok { "PASS" } else { "FAIL" };
            let notes = if class_ok && conf_ok {
                "class match + confidence above threshold".to_string()
            } else if !class_ok {
                format!(
                    "class mismatch: got {} expected {}",
                    c.class, expected_class
                )
            } else {
                format!(
                    "confidence {} below threshold {}",
                    c.confidence, expected_min_conf
                )
            };
            RowVerdict {
                id: row.id.clone(),
                verdict: verdict.to_string(),
                role: "blackbox".to_string(),
                expected: expected_json,
                actual: actual_json,
                notes,
            }
        }
        Err(e) => RowVerdict {
            id: row.id.clone(),
            verdict: "FAIL".to_string(),
            role: "blackbox".to_string(),
            expected: expected_json,
            actual: serde_json::json!({"raw": raw, "parse_error": e.to_string()}),
            notes: format!("triage output not valid JSON: {e}"),
        },
    }
}

/// Score a Meta-role row: parse the JSON envelope, check covered_slots is a
/// superset of expected, done matches, confidence ≥ min.
fn score_meta(row: &FixtureRow, raw: &str) -> RowVerdict {
    let expected_subset: Vec<String> = row
        .expected_covered_slots_subset
        .clone()
        .unwrap_or_default();
    let expected_done = row.expected_done;
    let expected_min_conf = row.expected_confidence_min.unwrap_or(0.0);
    let expected_json = serde_json::json!({
        "covered_slots_subset": expected_subset,
        "done": expected_done,
        "confidence_min": expected_min_conf,
    });

    // Attempt to parse the raw model output as JSON. Tolerate markdown fences.
    let candidate = strip_json_fence(raw);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&candidate);
    match parsed {
        Ok(v) => {
            let covered: Vec<String> = v
                .get("covered_slots")
                .and_then(|x| x.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|e| e.as_str().map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let done_val = v.get("done").and_then(|x| x.as_bool());
            let conf_val = v.get("confidence").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let actual_json = serde_json::json!({
                "covered_slots": covered,
                "done": done_val,
                "confidence": conf_val,
            });
            // Superset check.
            let covered_set: std::collections::HashSet<&String> = covered.iter().collect();
            let missing: Vec<&String> = expected_subset
                .iter()
                .filter(|s| !covered_set.contains(*s))
                .collect();
            let subset_ok = missing.is_empty();
            let done_ok = match (expected_done, done_val) {
                (None, _) => true,
                (Some(e), Some(g)) => e == g,
                (Some(_), None) => false,
            };
            let conf_ok = conf_val >= expected_min_conf;
            let verdict = if subset_ok && done_ok && conf_ok {
                "PASS"
            } else {
                "FAIL"
            };
            let mut notes_parts = Vec::new();
            if !subset_ok {
                notes_parts.push(format!("missing slots: {missing:?}"));
            }
            if !done_ok {
                notes_parts.push(format!(
                    "done mismatch: expected {expected_done:?}, got {done_val:?}"
                ));
            }
            if !conf_ok {
                notes_parts.push(format!("confidence {conf_val} < {expected_min_conf}"));
            }
            let notes = if notes_parts.is_empty() {
                "covered_slots superset + done match + confidence above threshold".to_string()
            } else {
                notes_parts.join("; ")
            };
            RowVerdict {
                id: row.id.clone(),
                verdict: verdict.to_string(),
                role: "meta".to_string(),
                expected: expected_json,
                actual: actual_json,
                notes,
            }
        }
        Err(e) => RowVerdict {
            id: row.id.clone(),
            verdict: "FAIL".to_string(),
            role: "meta".to_string(),
            expected: expected_json,
            actual: serde_json::json!({"raw": raw, "parse_error": e.to_string()}),
            notes: format!("meta envelope not valid JSON: {e}"),
        },
    }
}

/// Score a Playback row: must contain all `expected_contains_substrings` AND
/// must contain none of `expected_no_substrings`.
fn score_playback(row: &FixtureRow, raw: &str) -> RowVerdict {
    let must_contain: Vec<String> = row.expected_contains_substrings.clone().unwrap_or_default();
    let must_not_contain: Vec<String> = row.expected_no_substrings.clone().unwrap_or_default();
    let expected_json = serde_json::json!({
        "contains_substrings": must_contain,
        "no_substrings": must_not_contain,
    });

    let missing: Vec<String> = must_contain
        .iter()
        .filter(|s| !raw.contains(s.as_str()))
        .cloned()
        .collect();
    let leaked: Vec<String> = must_not_contain
        .iter()
        .filter(|s| raw.contains(s.as_str()))
        .cloned()
        .collect();

    let verdict = if missing.is_empty() && leaked.is_empty() {
        "PASS"
    } else {
        "FAIL"
    };
    let mut notes_parts = Vec::new();
    if !missing.is_empty() {
        notes_parts.push(format!("missing required substrings: {missing:?}"));
    }
    if !leaked.is_empty() {
        notes_parts.push(format!("forbidden substrings present: {leaked:?}"));
    }
    let notes = if notes_parts.is_empty() {
        "all required substrings present, no forbidden substrings".to_string()
    } else {
        notes_parts.join("; ")
    };

    // Truncate raw output in actual for readability.
    let raw_preview: String = raw.chars().take(400).collect();
    RowVerdict {
        id: row.id.clone(),
        verdict: verdict.to_string(),
        role: "playback".to_string(),
        expected: expected_json,
        actual: serde_json::json!({"output_preview": raw_preview}),
        notes,
    }
}

/// Strip an optional ```json ... ``` markdown fence around a JSON payload.
fn strip_json_fence(s: &str) -> String {
    let t = s.trim();
    // Find first fence.
    let after_open = match t.find("```") {
        None => return t.to_string(),
        Some(i) => &t[i + 3..],
    };
    // Skip optional language tag line.
    let content_start = after_open.find('\n').map(|i| i + 1).unwrap_or(0);
    let body = &after_open[content_start..];
    // Trim closing fence if present.
    if let Some(close) = body.rfind("```") {
        body[..close].trim().to_string()
    } else {
        body.trim().to_string()
    }
}

// ─── C10: `turingos llm prompt-eval --from <v1> --to <v2> --eval-set <cid>` ──
//
// Writes a PromptPromotionReceipt to CAS when the operator certifies that
// `--to` was evaluated against `--eval-set` and the decision is Promote.
//
// This subpath does NOT call any LLM — it records a human/operator assertion.
// For a live fixture-based eval, use the existing `prompt-eval --fixture` path.
//
// FC-trace: FC3 (eval evidence binding)
// Risk class: Class 3
// ─────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC3 C10: `prompt-eval --from --to --eval-set` promotion receipt writer.
fn run_prompt_promote(args: &[String]) -> ExitCode {
    let mut workspace: Option<std::path::PathBuf> = None;
    let mut from_prompt_path: Option<std::path::PathBuf> = None;
    let mut to_prompt_path: Option<std::path::PathBuf> = None;
    let mut eval_set_cid: Option<String> = None;
    let mut eval_before_cid: Option<String> = None;
    let mut eval_after_cid: Option<String> = None;
    let mut decision = turingosv4::runtime::prompt_promotion::PromotionDecision::Promote;

    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace = iter.next().map(|s| std::path::PathBuf::from(s));
            }
            "--from" => {
                from_prompt_path = iter.next().map(|s| std::path::PathBuf::from(s));
            }
            "--to" => {
                to_prompt_path = iter.next().map(|s| std::path::PathBuf::from(s));
            }
            "--eval-set" => {
                eval_set_cid = iter.next().cloned();
            }
            "--eval-before-cid" => {
                eval_before_cid = iter.next().cloned();
            }
            "--eval-after-cid" => {
                eval_after_cid = iter.next().cloned();
            }
            "--reject" => {
                decision = turingosv4::runtime::prompt_promotion::PromotionDecision::Reject;
            }
            _ => {}
        }
    }

    let workspace = match workspace {
        Some(p) => p,
        None => {
            eprintln!("turingos llm prompt-eval: --workspace required");
            return ExitCode::from(5);
        }
    };
    let from_path = match from_prompt_path {
        Some(p) => p,
        None => {
            eprintln!("turingos llm prompt-eval: --from required");
            return ExitCode::from(5);
        }
    };
    let to_path = match to_prompt_path {
        Some(p) => p,
        None => {
            eprintln!("turingos llm prompt-eval: --to required");
            return ExitCode::from(5);
        }
    };
    let eval_set = match eval_set_cid {
        Some(s) if !s.is_empty() => s,
        _ => {
            eprintln!("turingos llm prompt-eval: --eval-set <cid> required and must be non-empty");
            return ExitCode::from(5);
        }
    };

    let from_bytes = match std::fs::read(&from_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("turingos llm prompt-eval: cannot read --from {:?}: {e}", from_path);
            return ExitCode::from(4);
        }
    };
    let to_bytes = match std::fs::read(&to_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("turingos llm prompt-eval: cannot read --to {:?}: {e}", to_path);
            return ExitCode::from(4);
        }
    };

    use turingosv4::runtime::prompt_promotion::{
        sha256_hex_of_prompt, write_promotion_receipt, PromptPromotionReceipt,
        PROMPT_PROMOTION_RECEIPT_SCHEMA_ID,
    };

    let from_cid = sha256_hex_of_prompt(&from_bytes);
    let to_cid = sha256_hex_of_prompt(&to_bytes);

    let logical_t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(1000);

    let receipt = PromptPromotionReceipt {
        schema_id: PROMPT_PROMOTION_RECEIPT_SCHEMA_ID.to_string(),
        from_prompt_cid: from_cid.clone(),
        to_prompt_cid: to_cid.clone(),
        eval_set_cid: eval_set,
        eval_before_cid: eval_before_cid.unwrap_or_else(|| from_cid.clone()),
        eval_after_cid: eval_after_cid.unwrap_or_else(|| to_cid.clone()),
        promotion_decision: decision,
        logical_t,
    };

    match write_promotion_receipt(&workspace, &receipt, logical_t) {
        Ok(receipt_cid) => {
            println!(
                "{}",
                serde_json::json!({
                    "ok": true,
                    "receipt_cid": receipt_cid,
                    "from_prompt_cid": receipt.from_prompt_cid,
                    "to_prompt_cid": receipt.to_prompt_cid,
                    "eval_set_cid": receipt.eval_set_cid,
                    "promotion_decision": format!("{:?}", receipt.promotion_decision).to_lowercase(),
                })
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("turingos llm prompt-eval: failed to write receipt: {e}");
            ExitCode::from(2)
        }
    }
}
