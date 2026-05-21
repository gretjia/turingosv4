//! TRACE_MATRIX FC2-N16 + FC3 evidence binding: turingos spec handler (Phase 6.3)
//!
//! The "spec grill" — an 8-question customer-development interview that
//! extracts software requirements from a NON-DEVELOPER user via natural
//! language. Pulls from JTBD (Moesta), Mom Test (Fitzpatrick), Voss
//! mirroring/labeling, 5-Whys (Toyoda), and IDEO empathy interviewing.
//! Synthesised by independent research agent 2026-05-17.
//!
//! Output artifacts:
//!   - `<workspace>/spec.md`              (human-readable spec)
//!   - `<workspace>/spec_transcript.jsonl` (every Q/A turn + LLM usage)
//!   - CAS EvidenceCapsule of spec.md      (via spec_capsule.rs; CID printed)
//!
//! Modes:
//!   - INTERACTIVE (default): reads answers from stdin, one per question,
//!     blank line ends an answer block.
//!   - SCRIPTED (`--answers-file <PATH>`): reads a JSON array of 8 strings,
//!     uses them in order. Enables reproducible demo simulations.
//!   - DRIVEN (`--mode driven`): W6 atom — LLM drives the turn loop. Meta
//!     model asks questions, user answers via stdin; Blackbox triage classifies
//!     answers; predicates gate each turn; session capsule written to CAS.
//!
//! Class 1: filesystem write to workspace + Class 2 CAS wire via the
//! existing CasStore public surface. No Class-4 schema change.
//!
//! W6 extension (Phase 6.3.x): `--mode driven` loop composes W4 (`llm
//! complete`), W4.5 (`llm triage`), W5 (capsule writers), W3 (predicates),
//! and existing synthesis path into one driven-mode orchestration loop.

use std::collections::VecDeque;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::cmd_llm;
use crate::siliconflow_client::{chat_complete_blocking, require_api_key, ChatMessage, LlmError};
use turingosv4::runtime::spec_capsule;
use turingosv4::sdk::sanitized_runner::{
    env_allowlist_from_current, run_sanitized, SanitizedCommand,
};

/// TRACE_MATRIX FC2-N16: `spec` short-help
pub(crate) const SHORT_HELP: &str =
    "Interview the user (8-question grill) and emit a spec.md anchored in CAS";

/// TRACE_MATRIX FC2-N16: `spec` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos spec — Customer-development interview, emit spec.md + CAS capsule

USAGE:
    turingos spec --workspace <PATH> [--answers-file <PATH>] [--lang <zh|en>]
                  [--mode {static|driven}] [--meta-prompt <PATH>]

ACTIONS:
    (default)   Run the 8-question grill against the configured Meta LLM,
                emit spec.md + spec_transcript.jsonl + CAS EvidenceCapsule.

OPTIONS:
    --workspace <PATH>       Workspace directory (required; must contain
                             turingos.toml from `turingos llm config`).
    --answers-file <PATH>    JSON array of 8 strings — used non-interactively
                             for scripted runs / demos / regression tests.
                             Only valid in --mode static (default).
    --lang <zh|en>           Interview language. Default: zh (中文).
    --skip-llm               Synthesize spec.md from raw answers without an
                             LLM-driven narrative call. Use when the LLM
                             provider is unavailable or to skip the synthesis
                             pass. Output is functionally equivalent: spec.md
                             + spec_transcript.jsonl + CAS capsule are written
                             normally and `turingos generate` proceeds as usual.
                             Only valid in --mode static.
    --mode {static|driven}   Interview mode. Default: static (back-compat).
                             static: original 8-hardcoded-question flow.
                             driven: W6 LLM-driven turn loop; Meta model
                             asks questions, Blackbox triage classifies answers,
                             predicates gate each turn; writes session capsule.
    --meta-prompt <PATH>     Meta-prompt asset. Default:
                             assets/prompts/grill_meta_v1.md (relative to
                             workspace). Used in --mode driven; passed to
                             `turingos llm complete` as system prompt.
    -h, --help               Print this help.

INTERVIEW FLOW (assumes user is NOT a developer):
    Q1  The Job (JTBD opener): tell me about a recent moment when you
        thought "I wish I had a tool for this".
    Q2  The Anchor: any website / app that's even a little like what you want?
    Q3  What it Remembers: what should the program still know tomorrow morning?
    Q4  First-Click Walk-Through: what does the user see / click first?
    Q5  Weird-User Test: what should NOT break it?
    Q6  Disappointment Boundary: which features would feel like "scope creep"?
    Q7  Success Test: how will you KNOW it's doing its job after a month?
    Q8  Playback (mirror): seven-row fridge note — user confirms or corrects.

METHODOLOGY (sources): Customer Development (Blank), JTBD switch interview
    (Moesta), The Mom Test three sins (Fitzpatrick), Voss mirroring &
    labeling, 5-Whys (Toyoda), IDEO empathy interview, EARS syntax (Mavin),
    user story mapping (Patton). LLMREI arXiv 2507.02564.

OUTPUTS:
    <workspace>/spec.md                       Human-readable spec.
    <workspace>/spec_transcript.jsonl         Every Q/A turn + LLM usage.
    CAS EvidenceCapsule (schema=turingos-spec-capsule-v1) with CID printed.
"#;

/// W6: interview mode selector. Default = Static (back-compat).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpecMode {
    Static,
    Driven,
}

impl SpecMode {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "static" => Ok(Self::Static),
            "driven" => Ok(Self::Driven),
            other => Err(format!("invalid --mode '{other}': expected static|driven")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lang {
    Zh,
    En,
}

impl Lang {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "zh" | "cn" | "中文" => Ok(Self::Zh),
            "en" | "english" => Ok(Self::En),
            other => Err(format!("invalid --lang '{other}': expect zh|en")),
        }
    }
}

#[derive(Debug)]
enum SpecError {
    MissingFlag(&'static str),
    WorkspaceNotFound(String),
    BadAnswersFile(String),
    Io(String),
    Llm(LlmError),
    Capsule(spec_capsule::CapsuleError),
    NeedAnswersFileWhenSkippingLlm,
    /// W6: --mode flag value was not recognised.
    BadMode(String),
    /// W6: driven mode tried to get the current executable path and failed.
    CurrentExe(String),
}

impl std::fmt::Display for SpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFlag(flag) => write!(f, "missing required flag: {flag}"),
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::BadAnswersFile(e) => write!(f, "bad --answers-file: {e}"),
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Llm(e) => write!(f, "{e}"),
            Self::Capsule(e) => write!(f, "{e}"),
            Self::NeedAnswersFileWhenSkippingLlm => write!(
                f,
                "--skip-llm requires --answers-file (cannot run an interactive grill without an LLM)"
            ),
            Self::BadMode(e) => write!(f, "{e}"),
            Self::CurrentExe(e) => write!(f, "current_exe: {e}"),
        }
    }
}

impl From<LlmError> for SpecError {
    fn from(e: LlmError) -> Self {
        Self::Llm(e)
    }
}

impl From<spec_capsule::CapsuleError> for SpecError {
    fn from(e: spec_capsule::CapsuleError) -> Self {
        Self::Capsule(e)
    }
}

/// TRACE_MATRIX FC2-N16: `spec` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos spec: {e}");
            ExitCode::from(2)
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), SpecError> {
    let mut workspace = PathBuf::from(".");
    let mut answers_file: Option<PathBuf> = None;
    let mut lang = Lang::Zh;
    let mut skip_llm = false;
    // W6: new flags
    let mut mode = SpecMode::Static;
    let mut meta_prompt: Option<PathBuf> = None;

    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace =
                    PathBuf::from(iter.next().ok_or(SpecError::MissingFlag("--workspace"))?);
            }
            "--answers-file" => {
                answers_file = Some(PathBuf::from(
                    iter.next()
                        .ok_or(SpecError::MissingFlag("--answers-file"))?,
                ));
            }
            "--lang" => {
                let v = iter.next().ok_or(SpecError::MissingFlag("--lang"))?;
                lang = Lang::parse(v).map_err(SpecError::BadAnswersFile)?;
            }
            "--skip-llm" => {
                skip_llm = true;
            }
            // W6: --mode flag
            "--mode" => {
                let v = iter.next().ok_or(SpecError::MissingFlag("--mode"))?;
                mode = SpecMode::parse(v).map_err(SpecError::BadMode)?;
            }
            // W6: --meta-prompt flag (informational + passed to llm complete)
            "--meta-prompt" => {
                meta_prompt = Some(PathBuf::from(
                    iter.next().ok_or(SpecError::MissingFlag("--meta-prompt"))?,
                ));
            }
            _ => {}
        }
    }

    if !workspace.exists() {
        return Err(SpecError::WorkspaceNotFound(
            workspace.display().to_string(),
        ));
    }

    // W6: route driven mode to its own implementation.
    if mode == SpecMode::Driven {
        // Resolve meta_prompt relative to workspace if relative path given,
        // or use the default if not specified.
        let meta_prompt_resolved = meta_prompt
            .map(|p| {
                if p.is_absolute() {
                    p
                } else {
                    workspace.join(&p)
                }
            })
            .unwrap_or_else(|| workspace.join("assets/prompts/grill_meta_v1.md"));
        return run_driven_mode(&workspace, lang, meta_prompt_resolved);
    }

    if skip_llm && answers_file.is_none() {
        return Err(SpecError::NeedAnswersFileWhenSkippingLlm);
    }

    let questions = canonical_questions(lang);

    // Gather 8 answers — either from --answers-file or via interactive stdin.
    let answers: Vec<String> = if let Some(path) = answers_file.as_ref() {
        load_answers_from_file(path)?
    } else {
        interactive_gather(&questions)?
    };

    if answers.len() != 8 {
        return Err(SpecError::BadAnswersFile(format!(
            "expected exactly 8 answers, got {}",
            answers.len()
        )));
    }

    // Build the LLM-facing transcript (one system + 8 Q/A user turns).
    let mut transcript = Vec::new();
    transcript.push(TurnRecord {
        role: "system".into(),
        content: system_prompt(lang),
        model: None,
        usage_total_tokens: 0,
    });
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        transcript.push(TurnRecord {
            role: "user".into(),
            content: format!("Q{}: {}\nA{}: {}", i + 1, q, i + 1, a),
            model: None,
            usage_total_tokens: 0,
        });
    }

    let model_id = cmd_llm::read_meta_model(&workspace);
    let api_key_env =
        cmd_llm::read_meta_api_key_env(&workspace).map_err(|e| SpecError::Io(e.to_string()))?;

    let (synthesis, total_tokens) = if skip_llm {
        // CAS-wire-only path: synthesise spec.md without LLM (uses canonical
        // question phrasings + the raw user answers; no playback critique).
        let synth = synthesise_spec_md_no_llm(lang, &questions, &answers);
        (synth, 0u64)
    } else {
        let api_key = match require_api_key(&api_key_env) {
            Ok(k) => k,
            Err(_) => {
                eprintln!(
                    "error: Meta role API key env var \"${api_key_env}\" is not set in your shell."
                );
                eprintln!("       Run: export {api_key_env}=\"sk-...\"");
                eprintln!(
                    "       Then retry: turingos spec --workspace {}",
                    workspace.display()
                );
                return Err(SpecError::Llm(LlmError::MissingApiKey {
                    env_var: api_key_env.clone(),
                }));
            }
        };
        let synth_user_msg = build_synthesis_user_message(lang, &questions, &answers);
        let messages = vec![
            ChatMessage::system(system_prompt(lang)),
            ChatMessage::user(synth_user_msg.clone()),
        ];
        eprintln!("[spec] calling Meta LLM ({model_id}) to synthesise spec.md...");
        let meta_thinking = cmd_llm::read_meta_thinking(&workspace);
        let result = chat_complete_blocking(
            &api_key,
            &model_id,
            &messages,
            Some(3000),
            Some(0.3),
            meta_thinking,
        )?;
        transcript.push(TurnRecord {
            role: "user".into(),
            content: synth_user_msg,
            model: Some(model_id.clone()),
            usage_total_tokens: 0,
        });
        transcript.push(TurnRecord {
            role: "assistant".into(),
            content: result.content.clone(),
            model: Some(model_id.clone()),
            usage_total_tokens: result.usage.total_tokens,
        });
        (result.content, result.usage.total_tokens)
    };

    let spec_md = wrap_spec_md(&synthesis, &questions, &answers, &model_id, skip_llm);
    let spec_md_path = workspace.join("spec.md");
    fs::write(&spec_md_path, &spec_md).map_err(|e| SpecError::Io(format!("write spec.md: {e}")))?;

    let transcript_path = workspace.join("spec_transcript.jsonl");
    write_transcript_jsonl(&transcript_path, &transcript)?;

    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cid_hex = spec_capsule::write_spec_capsule(&workspace, &spec_md, "user", logical_t)?;

    println!();
    println!("Spec interview complete.");
    println!("  spec.md            -> {}", spec_md_path.display());
    println!("  spec_transcript    -> {}", transcript_path.display());
    println!("  CAS capsule CID    -> {cid_hex}");
    println!(
        "                       (schema: {})",
        spec_capsule::SPEC_CAPSULE_SCHEMA_ID
    );
    if total_tokens > 0 {
        println!("  LLM total tokens   -> {total_tokens}");
    }
    println!();
    println!(
        "Next step: turingos generate --workspace {}",
        workspace.display()
    );
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// W6: --mode driven — LLM-driven turn loop
// TRACE_MATRIX FC1-N9 + FC2-N16 + FC3: grill driven-mode orchestration.
//
// Slot state for coverage tracking.
// ─────────────────────────────────────────────────────────────────────────────

/// Per-slot coverage state. Grows only (Empty → Satisfied; no reversal).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotState {
    Empty,
    Satisfied,
}

/// In-memory coverage state for one driven-mode session.
struct CoverageState {
    /// 8 slots corresponding to CANONICAL_SLOTS order.
    slot_states: [SlotState; 8],
    /// Rolling window: last 3 (question, answer) pairs that passed triage.
    last_3_turns: VecDeque<(String, String)>,
    /// Total turns completed (including retried / off-topic).
    turn_count: u32,
    /// Number of abusive/gibberish answers seen (abort threshold = 2).
    non_relevant_count: u32,
    /// CIDs of each written GrillTurnCapsuleBody, in turn order.
    turn_cids: Vec<String>,
    /// All user answers (in order) that were accepted as relevant.
    all_user_answers: Vec<String>,
    // Counters for GrillAttemptTally.
    meta_turns_accepted: u32,
    meta_turns_rejected: u32,
    triage_calls_relevant: u32,
    triage_calls_non_relevant: u32,
    synthesis_calls: u32,
}

impl CoverageState {
    fn new() -> Self {
        Self {
            slot_states: [SlotState::Empty; 8],
            last_3_turns: VecDeque::with_capacity(3),
            turn_count: 0,
            non_relevant_count: 0,
            turn_cids: Vec::new(),
            all_user_answers: Vec::new(),
            meta_turns_accepted: 0,
            meta_turns_rejected: 0,
            triage_calls_relevant: 0,
            triage_calls_non_relevant: 0,
            synthesis_calls: 0,
        }
    }

    /// Mark slots from a payload's covered_slots list as Satisfied.
    fn update_slots(&mut self, covered: &[String]) {
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS;
        for slot in covered {
            if let Some(idx) = CANONICAL_SLOTS.iter().position(|s| *s == slot.as_str()) {
                self.slot_states[idx] = SlotState::Satisfied;
            }
        }
    }

    /// Produce a summary string injected as system context each turn.
    fn coverage_summary(&self) -> String {
        use turingosv4::runtime::grill_envelope::CANONICAL_SLOTS;
        let mut parts = Vec::new();
        for (i, &state) in self.slot_states.iter().enumerate() {
            let mark = if state == SlotState::Satisfied {
                "[x]"
            } else {
                "[ ]"
            };
            parts.push(format!("{mark} {}", CANONICAL_SLOTS[i]));
        }
        format!(
            "Coverage state (turn {}):\n{}\nTurns used: {}",
            self.turn_count,
            parts.join("\n"),
            self.turn_count,
        )
    }

    /// Push an accepted (question, answer) pair to the rolling last_3 window.
    fn push_turn(&mut self, question: String, answer: String) {
        if self.last_3_turns.len() == 3 {
            self.last_3_turns.pop_front();
        }
        self.last_3_turns.push_back((question, answer));
    }

    fn to_grill_attempt_tally(&self) -> spec_capsule::GrillAttemptTally {
        spec_capsule::GrillAttemptTally {
            meta_turns_accepted: self.meta_turns_accepted,
            meta_turns_rejected: self.meta_turns_rejected,
            triage_calls_relevant: self.triage_calls_relevant,
            triage_calls_non_relevant: self.triage_calls_non_relevant,
            synthesis_calls: self.synthesis_calls,
        }
    }
}

// ── sha256 helper (mirrors cmd_llm.rs pattern) ────────────────────────────────

fn sha256_bytes_spec(data: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

fn sha256_hex(data: &[u8]) -> String {
    let b = sha256_bytes_spec(data);
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

// ── session_id generator ──────────────────────────────────────────────────────

/// Generate a session id: `<unix_secs>_<8 hex chars from /dev/urandom or fallback>`.
fn generate_session_id() -> String {
    let epoch_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Try to read 4 bytes from /dev/urandom; fall back to hashing epoch+pid.
    let random_hex: String = {
        let random_bytes: [u8; 4] = (|| -> Option<[u8; 4]> {
            let mut buf = [0u8; 4];
            let mut f = std::fs::File::open("/dev/urandom").ok()?;
            use std::io::Read as IoRead;
            f.read_exact(&mut buf).ok()?;
            Some(buf)
        })()
        .unwrap_or_else(|| {
            // Fallback: sha256(epoch || pid) truncated to 4 bytes.
            let pid = std::process::id();
            let seed = format!("{epoch_secs}{pid}");
            let b = sha256_bytes_spec(seed.as_bytes());
            [b[0], b[1], b[2], b[3]]
        });
        random_bytes.iter().map(|x| format!("{:02x}", x)).collect()
    };

    format!("{epoch_secs}_{random_hex}")
}

// ── Lang conversion ──────────────────────────────────────────────────────────

/// Convert cmd_spec::Lang to grill_predicates::Lang for predicate calls.
fn to_pred_lang(lang: Lang) -> turingosv4::runtime::grill_predicates::Lang {
    match lang {
        Lang::Zh => turingosv4::runtime::grill_predicates::Lang::Zh,
        Lang::En => turingosv4::runtime::grill_predicates::Lang::En,
    }
}

fn lang_str(lang: Lang) -> &'static str {
    match lang {
        Lang::Zh => "zh",
        Lang::En => "en",
    }
}

// ── Outcome mapping ───────────────────────────────────────────────────────────

/// Map a PredicateBundle first-failure to GrillAttemptOutcome.
fn outcome_from_bundle(
    bundle: &turingosv4::runtime::grill_predicates::PredicateBundle,
) -> spec_capsule::GrillAttemptOutcome {
    use spec_capsule::GrillAttemptOutcome;
    use turingosv4::runtime::grill_predicates::PredicateFailureClass;
    if bundle.all_pass() {
        return GrillAttemptOutcome::PredicatesPassed;
    }
    match bundle.first_failure() {
        Some(PredicateFailureClass::SchemaParseError) => GrillAttemptOutcome::SchemaParseFailed,
        Some(PredicateFailureClass::KindMismatch) => GrillAttemptOutcome::KindMismatch,
        Some(PredicateFailureClass::PlaybackMissing) => GrillAttemptOutcome::KindMismatch,
        Some(PredicateFailureClass::QuestionMissing) => GrillAttemptOutcome::KindMismatch,
        Some(PredicateFailureClass::UnknownSlot) => GrillAttemptOutcome::UnknownSlot,
        Some(PredicateFailureClass::NonMonotonic) => GrillAttemptOutcome::NonMonotonic,
        Some(PredicateFailureClass::TurnOutOfRange) => GrillAttemptOutcome::TurnOutOfRange,
        Some(PredicateFailureClass::LanguageMismatch) => GrillAttemptOutcome::LanguageMismatch,
        Some(PredicateFailureClass::QuestionTooShort) => GrillAttemptOutcome::LanguageMismatch,
        Some(PredicateFailureClass::ConfidenceOutOfRange) => GrillAttemptOutcome::SchemaParseFailed,
        None => GrillAttemptOutcome::PredicatesPassed,
    }
}

// ── CAS store user answer ─────────────────────────────────────────────────────

/// Store user-answer bytes in the session CAS; returns CID hex.
fn cas_store_user_answer(workspace: &Path, capsule_dir: &Path, answer: &str) -> Option<String> {
    use turingosv4::bottom_white::cas::schema::ObjectType;
    use turingosv4::bottom_white::cas::store::CasStore;
    let cas_dir = capsule_dir.join("cas");
    let _ = std::fs::create_dir_all(&cas_dir);
    let mut store = CasStore::open(&cas_dir).ok()?;
    let cid = store
        .put(
            answer.as_bytes(),
            ObjectType::EvidenceCapsule,
            "grill_user_answer",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            Some("grill-user-answer-v1".to_string()),
        )
        .ok()?;
    let _ = workspace; // suppress unused warning
    Some(cid.hex())
}

/// Store TurnPayload bytes in the session CAS; returns CID hex.
fn cas_store_payload(
    capsule_dir: &Path,
    payload: &turingosv4::runtime::grill_envelope::TurnPayload,
) -> Option<String> {
    use turingosv4::bottom_white::cas::schema::ObjectType;
    use turingosv4::bottom_white::cas::store::CasStore;
    let cas_dir = capsule_dir.join("cas");
    let _ = std::fs::create_dir_all(&cas_dir);
    let mut store = CasStore::open(&cas_dir).ok()?;
    let bytes = serde_json::to_vec(payload).ok()?;
    let cid = store
        .put(
            &bytes,
            ObjectType::EvidenceCapsule,
            "grill_candidate_payload",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            Some("grill-candidate-payload-v1".to_string()),
        )
        .ok()?;
    Some(cid.hex())
}

// ── Prompt JSON for a driven turn ────────────────────────────────────────────

/// Build the messages JSON file content for one driven-mode turn.
/// Format: `{"messages": [...]}`
fn build_turn_prompt_json(
    meta_prompt_content: &str,
    coverage_summary: &str,
    last_3_turns: &VecDeque<(String, String)>,
    turn_index: u32,
    extra_system: Option<&str>,
) -> String {
    let mut messages: Vec<serde_json::Value> = Vec::new();

    // 1. System: meta-prompt content.
    messages.push(serde_json::json!({
        "role": "system",
        "content": meta_prompt_content,
    }));

    // 2. System: coverage state summary.
    messages.push(serde_json::json!({
        "role": "system",
        "content": coverage_summary,
    }));

    // 3. Optional extra system message (e.g. predicate failure nudge).
    if let Some(extra) = extra_system {
        messages.push(serde_json::json!({
            "role": "system",
            "content": extra,
        }));
    }

    // 4. Last 3 accepted turns as alternating user/assistant pairs.
    for (q, a) in last_3_turns.iter() {
        messages.push(serde_json::json!({
            "role": "assistant",
            "content": q,
        }));
        messages.push(serde_json::json!({
            "role": "user",
            "content": a,
        }));
    }

    // 5. Final user instruction.
    messages.push(serde_json::json!({
        "role": "user",
        "content": format!("Produce your turn-{turn_index} output per the contract."),
    }));

    serde_json::json!({ "messages": messages }).to_string()
}

// ── Shell-out helper: call `turingos llm complete` ───────────────────────────

struct LlmCompleteResult {
    ok: bool,
    content: String,
    model: String,
    elapsed_ms: u64,
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    prompt_capsule_cid: Option<String>,
    parsed_envelope: Option<serde_json::Value>,
}

/// Shell out to `turingos llm complete`.
/// Returns Err with a detail string if the process could not be spawned;
/// the Ok variant carries the parsed JSON even on ok=false (for retry logic).
fn shell_llm_complete(
    exe: &Path,
    workspace: &Path,
    prompt_file: &Path,
    capsule_dir: &Path,
    turn_id: &str,
    lang: Lang,
    meta_prompt_path: &Path,
) -> Result<LlmCompleteResult, String> {
    let output = run_sanitized(SanitizedCommand {
        program: exe.to_path_buf(),
        args: vec![
            "llm".into(),
            "complete".into(),
            "--workspace".into(),
            workspace.to_string_lossy().into_owned(),
            "--role".into(),
            "meta".into(),
            "--prompt-file".into(),
            prompt_file.to_string_lossy().into_owned(),
            "--strict-json".into(),
            "--capsule-dir".into(),
            capsule_dir.to_string_lossy().into_owned(),
            "--turn-id".into(),
            turn_id.to_string(),
            "--lang".into(),
            lang_str(lang).to_string(),
            "--meta-prompt".into(),
            meta_prompt_path.to_string_lossy().into_owned(),
        ],
        cwd: workspace.to_path_buf(),
        env: llm_child_env(),
        stdin: None,
        timeout: std::time::Duration::from_secs(240),
    })
    .map_err(|e| format!("spawn llm complete: {e}"))?;

    if !output.success() {
        return Err(format!(
            "llm complete exit {:?}: stdout={} stderr={}",
            output.exit_code,
            String::from_utf8_lossy(&output.stdout)
                .chars()
                .take(200)
                .collect::<String>(),
            String::from_utf8_lossy(&output.stderr)
                .chars()
                .take(200)
                .collect::<String>()
        ));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout_str.trim()).map_err(|e| {
        format!(
            "parse llm complete stdout JSON: {e}; raw={}",
            stdout_str.chars().take(200).collect::<String>()
        )
    })?;

    let ok = v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false);
    let content = v
        .get("content")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let model = v
        .get("model")
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let elapsed_ms = v.get("elapsed_ms").and_then(|x| x.as_u64()).unwrap_or(0) as u64;
    let usage = v.get("usage");
    let prompt_tokens = usage
        .and_then(|u| u.get("prompt_tokens"))
        .and_then(|x| x.as_u64())
        .unwrap_or(0) as u32;
    let completion_tokens = usage
        .and_then(|u| u.get("completion_tokens"))
        .and_then(|x| x.as_u64())
        .unwrap_or(0) as u32;
    let total_tokens = usage
        .and_then(|u| u.get("total_tokens"))
        .and_then(|x| x.as_u64())
        .unwrap_or(0) as u32;
    let prompt_capsule_cid = v
        .get("prompt_capsule_cid")
        .and_then(|x| x.as_str())
        .map(|s| s.to_string());
    let parsed_envelope = v.get("parsed_envelope").cloned();

    Ok(LlmCompleteResult {
        ok,
        content,
        model,
        elapsed_ms,
        prompt_tokens,
        completion_tokens,
        total_tokens,
        prompt_capsule_cid,
        parsed_envelope,
    })
}

// ── Shell-out helper: call `turingos llm triage` ─────────────────────────────

struct TriageResult {
    ok: bool,
    class: String,
}

fn shell_llm_triage(
    exe: &Path,
    workspace: &Path,
    user_answer: &str,
    question: &str,
    lang: Lang,
    capsule_dir: &Path,
    turn_id: &str,
) -> Result<TriageResult, String> {
    let output = run_sanitized(SanitizedCommand {
        program: exe.to_path_buf(),
        args: vec![
            "llm".into(),
            "triage".into(),
            "--workspace".into(),
            workspace.to_string_lossy().into_owned(),
            "--user-answer".into(),
            user_answer.to_string(),
            "--question".into(),
            question.to_string(),
            "--lang".into(),
            lang_str(lang).to_string(),
            "--capsule-dir".into(),
            capsule_dir.to_string_lossy().into_owned(),
            "--turn-id".into(),
            turn_id.to_string(),
        ],
        cwd: workspace.to_path_buf(),
        env: llm_child_env(),
        stdin: None,
        timeout: std::time::Duration::from_secs(240),
    })
    .map_err(|e| format!("spawn llm triage: {e}"))?;

    if !output.success() {
        return Err(format!(
            "llm triage exit {:?}: stdout={} stderr={}",
            output.exit_code,
            String::from_utf8_lossy(&output.stdout)
                .chars()
                .take(200)
                .collect::<String>(),
            String::from_utf8_lossy(&output.stderr)
                .chars()
                .take(200)
                .collect::<String>()
        ));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout_str.trim()).map_err(|e| {
        format!(
            "parse llm triage stdout JSON: {e}; raw={}",
            stdout_str.chars().take(200).collect::<String>()
        )
    })?;

    let ok = v.get("ok").and_then(|x| x.as_bool()).unwrap_or(false);
    let class = v
        .get("class")
        .and_then(|x| x.as_str())
        .unwrap_or("gibberish")
        .to_string();

    Ok(TriageResult { ok, class })
}

fn llm_child_env() -> std::collections::BTreeMap<String, String> {
    env_allowlist_from_current(&[
        "PATH",
        "SILICONFLOW_API_KEY",
        "DEEPSEEK_API_KEY",
        "DEEPSEEK_API_KEY_WORKER",
        "OPENROUTER_API_KEY",
        "OPENAI_API_KEY",
        "TURINGOS_SILICONFLOW_ENDPOINT",
    ])
}

// ── Main driven-mode orchestration ───────────────────────────────────────────

/// W6: `--mode driven` implementation.
///
/// Drives the LLM-grill interview loop per the R2 §A5/A6/A7 specification:
/// - Shell out to `turingos llm complete` for each Meta turn.
/// - Shell out to `turingos llm triage` to classify each user answer.
/// - Run `grill_predicates::run_turn_predicates` on each accepted turn.
/// - Write `GrillTurnCapsuleBody` per turn; write `GrillSessionCapsuleBody` on exit.
/// - On clean completion (termination_predicate pass + turn_count >= 4), call
///   the existing synthesis path to produce spec.md.
///
/// Hard turn ceiling: 15.
/// Design note on turn re-loop for triage: when triage returns off_topic/abusive/
/// gibberish the outer loop continues with the same turn_index (we decrement
/// turn_index by 1 so the increment at the top of the for-loop restores it).
/// This means the aborted triage turn counts toward the turn budget.
fn run_driven_mode(
    workspace: &Path,
    lang: Lang,
    meta_prompt_path: PathBuf,
) -> Result<(), SpecError> {
    // ── 0. Resolve current exe ────────────────────────────────────────────────
    let exe = std::env::current_exe().map_err(|e| SpecError::CurrentExe(e.to_string()))?;

    // ── 1. Session id + session dir ───────────────────────────────────────────
    let session_id = generate_session_id();
    let session_dir = workspace.join("sessions").join(&session_id);
    let capsules_dir = session_dir.join("capsules");
    let answers_dir = session_dir.join("answers");
    fs::create_dir_all(&capsules_dir)
        .map_err(|e| SpecError::Io(format!("create capsules dir: {e}")))?;
    fs::create_dir_all(&answers_dir)
        .map_err(|e| SpecError::Io(format!("create answers dir: {e}")))?;

    // ── 2. Read meta-prompt content ───────────────────────────────────────────
    let meta_prompt_content = fs::read_to_string(&meta_prompt_path).map_err(|e| {
        SpecError::Io(format!(
            "read meta-prompt {}: {e}",
            meta_prompt_path.display()
        ))
    })?;

    // ── 3. Initialize coverage state ──────────────────────────────────────────
    let mut state = CoverageState::new();
    let mut termination_reason = "turn_limit_forced".to_string();
    let mut prev_covered: Vec<String> = Vec::new();
    let mut last_turn_cid: Option<String> = None;

    eprintln!("[spec driven] session: {session_id}");
    eprintln!("[spec driven] session dir: {}", session_dir.display());

    // ── 4. Turn loop (1..=15 hard ceiling) ───────────────────────────────────
    let mut turn_index: u32 = 0; // incremented at start of each iteration

    // We use a while loop so we can decrement turn_index for triage re-loops.
    // Real turn budget is tracked via state.turn_count (incremented only on
    // accepted Meta call). turn_index is the loop index (1..=15).
    while turn_index < 15 {
        turn_index += 1;
        state.turn_count += 1;

        // ── 4a. Assemble prompt JSON ──────────────────────────────────────────
        let coverage_summary = state.coverage_summary();
        let prompt_json = build_turn_prompt_json(
            &meta_prompt_content,
            &coverage_summary,
            &state.last_3_turns,
            turn_index,
            None,
        );
        let prompt_file_path = session_dir.join(format!("turn-{turn_index}-prompt.json"));
        fs::write(&prompt_file_path, &prompt_json)
            .map_err(|e| SpecError::Io(format!("write prompt file turn-{turn_index}: {e}")))?;

        // ── 4b/4c. Shell out to llm complete (with retry) ────────────────────
        let turn_id = format!("turn-{turn_index}");
        let mut retry_count = 0u32;
        let mut payload_opt: Option<turingosv4::runtime::grill_envelope::TurnPayload> = None;
        let mut complete_result: Option<LlmCompleteResult> = None;
        let mut bundle_opt: Option<turingosv4::runtime::grill_predicates::PredicateBundle> = None;

        loop {
            // Call llm complete
            let result = match shell_llm_complete(
                &exe,
                workspace,
                &prompt_file_path,
                &capsules_dir,
                &turn_id,
                lang,
                &meta_prompt_path,
            ) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[spec driven] turn-{turn_index} spawn error: {e}");
                    // Treat as double-fail if retry exhausted
                    if retry_count >= 1 {
                        state.meta_turns_rejected += 1;
                        // Build a minimal failed turn capsule and break
                        let fail_cid = write_failed_turn_capsule(
                            workspace,
                            &session_id,
                            turn_index,
                            last_turn_cid.as_deref(),
                            spec_capsule::GrillAttemptOutcome::LlmApiError,
                            retry_count,
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0),
                        );
                        if let Ok(cid) = fail_cid {
                            state.turn_cids.push(cid.clone());
                            last_turn_cid = Some(cid);
                        }
                        termination_reason = "predicate_double_fail".to_string();
                        break;
                    }
                    retry_count += 1;
                    continue;
                }
            };

            if !result.ok {
                // LLM call returned ok=false (API error or parse fail in strict-json).
                if retry_count >= 1 {
                    state.meta_turns_rejected += 1;
                    let fail_cid = write_failed_turn_capsule(
                        workspace,
                        &session_id,
                        turn_index,
                        last_turn_cid.as_deref(),
                        spec_capsule::GrillAttemptOutcome::DoubleRetryFailed,
                        retry_count,
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                    );
                    if let Ok(cid) = fail_cid {
                        state.turn_cids.push(cid.clone());
                        last_turn_cid = Some(cid);
                    }
                    termination_reason = "predicate_double_fail".to_string();
                    complete_result = None;
                    break;
                }
                // Inject a retry nudge into a new prompt file.
                retry_count += 1;
                let retry_extra = "Your previous output failed JSON/envelope validation. \
                    Output ONLY the JSON envelope with a valid question. Try again.";
                let retry_json = build_turn_prompt_json(
                    &meta_prompt_content,
                    &coverage_summary,
                    &state.last_3_turns,
                    turn_index,
                    Some(retry_extra),
                );
                let retry_file = session_dir.join(format!("turn-{turn_index}-retry-prompt.json"));
                let _ = fs::write(&retry_file, &retry_json);
                // Use the retry prompt file on next iteration
                let _ = fs::write(&prompt_file_path, &retry_json);
                continue;
            }

            // Parse the TurnPayload from content.
            let tp = match turingosv4::runtime::grill_envelope::parse_and_validate(&result.content)
            {
                Ok(p) => p,
                Err(e) => {
                    if retry_count >= 1 {
                        state.meta_turns_rejected += 1;
                        let fail_cid = write_failed_turn_capsule(
                            workspace,
                            &session_id,
                            turn_index,
                            last_turn_cid.as_deref(),
                            spec_capsule::GrillAttemptOutcome::SchemaParseFailed,
                            retry_count,
                            SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(0),
                        );
                        if let Ok(cid) = fail_cid {
                            state.turn_cids.push(cid.clone());
                            last_turn_cid = Some(cid);
                        }
                        termination_reason = "predicate_double_fail".to_string();
                        complete_result = None;
                        break;
                    }
                    retry_count += 1;
                    let retry_extra = format!(
                        "Your previous output failed predicate: {e}. \
                        Output ONLY the JSON envelope with a valid question. Try again."
                    );
                    let retry_json = build_turn_prompt_json(
                        &meta_prompt_content,
                        &coverage_summary,
                        &state.last_3_turns,
                        turn_index,
                        Some(&retry_extra),
                    );
                    let _ = fs::write(&prompt_file_path, &retry_json);
                    continue;
                }
            };

            // Run per-turn predicates.
            let bundle = turingosv4::runtime::grill_predicates::run_turn_predicates(
                &tp,
                &prev_covered,
                to_pred_lang(lang),
            );

            if !bundle.all_pass() {
                if retry_count >= 1 {
                    state.meta_turns_rejected += 1;
                    let fail_cid = write_failed_turn_capsule(
                        workspace,
                        &session_id,
                        turn_index,
                        last_turn_cid.as_deref(),
                        spec_capsule::GrillAttemptOutcome::DoubleRetryFailed,
                        retry_count,
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                    );
                    if let Ok(cid) = fail_cid {
                        state.turn_cids.push(cid.clone());
                        last_turn_cid = Some(cid);
                    }
                    termination_reason = "predicate_double_fail".to_string();
                    complete_result = None;
                    break;
                }
                retry_count += 1;
                let fail_class_str = bundle
                    .first_failure()
                    .map(|c| format!("{:?}", c))
                    .unwrap_or_default();
                let retry_extra = format!(
                    "Your previous output failed predicate {fail_class_str}. \
                    Output ONLY the JSON envelope with a valid question. Try again."
                );
                let retry_json = build_turn_prompt_json(
                    &meta_prompt_content,
                    &coverage_summary,
                    &state.last_3_turns,
                    turn_index,
                    Some(&retry_extra),
                );
                let _ = fs::write(&prompt_file_path, &retry_json);
                continue;
            }

            // All predicates passed.
            complete_result = Some(result);
            payload_opt = Some(tp);
            bundle_opt = Some(bundle);
            break;
        }

        // Check if loop broke early due to double fail.
        if termination_reason == "predicate_double_fail" {
            break;
        }

        let result = match complete_result {
            Some(r) => r,
            None => break,
        };
        let payload = match payload_opt {
            Some(p) => p,
            None => break,
        };
        let bundle = match bundle_opt {
            Some(b) => b,
            None => break,
        };

        state.meta_turns_accepted += 1;

        // ── 4g. Build GrillAttemptRecord ──────────────────────────────────────
        let logical_t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let candidate_payload_cid = cas_store_payload(&capsules_dir, &payload)
            .unwrap_or_else(|| sha256_hex(result.content.as_bytes()));

        let prompt_context_hash = result
            .prompt_capsule_cid
            .as_deref()
            .map(|cid| sha256_hex(cid.as_bytes()))
            .unwrap_or_else(|| sha256_hex(prompt_json.as_bytes()));

        let attempt_record = spec_capsule::GrillAttemptRecord {
            schema_version: 1,
            session_id: session_id.clone(),
            turn_index,
            model_id: result.model.clone(),
            prompt_context_hash,
            candidate_payload_cid: candidate_payload_cid.clone(),
            outcome: spec_capsule::GrillAttemptOutcome::PredicatesPassed,
            token_counts: spec_capsule::GrillTokenCounts {
                prompt_tokens: result.prompt_tokens,
                completion_tokens: result.completion_tokens,
                total_tokens: result.total_tokens,
            },
            elapsed_ms: result.elapsed_ms,
            retry_index: retry_count,
        };

        // ── 4h. Build and write GrillTurnCapsuleBody ──────────────────────────
        let turn_capsule = spec_capsule::GrillTurnCapsuleBody {
            session_id: session_id.clone(),
            turn_index,
            prompt_capsule_cid: result.prompt_capsule_cid.clone().unwrap_or_default(),
            user_answer_cid: None, // filled in after triage step below
            parent_turn_cid: last_turn_cid.clone(),
            grill_attempt_record: attempt_record.clone(),
            predicate_verdicts: bundle,
            turn_payload_snapshot: serde_json::to_value(&payload)
                .unwrap_or(serde_json::Value::Null),
            logical_t,
        };

        // ── 4i. Check if LLM declared done ───────────────────────────────────
        if payload.done {
            let term_verdict =
                turingosv4::runtime::grill_predicates::termination_predicate(&payload);
            if term_verdict.is_pass() {
                // Write capsule then break.
                let turn_cid = spec_capsule::write_grill_turn_capsule(workspace, &turn_capsule)
                    .unwrap_or_default();
                state.turn_cids.push(turn_cid.clone());
                last_turn_cid = Some(turn_cid);
                // Update coverage before synthesis.
                state.update_slots(&payload.covered_slots);
                prev_covered = payload.covered_slots.clone();
                termination_reason = "llm_done_predicate_pass".to_string();
                break;
            } else {
                // Inject nudge; do NOT display question to user; continue loop
                let missing = turingosv4::runtime::grill_envelope::REQUIRED_SLOTS
                    .iter()
                    .find(|&&s| !payload.covered_slots.iter().any(|c| c == s))
                    .copied()
                    .unwrap_or("unknown");
                eprintln!(
                    "[spec driven] turn-{turn_index}: done=true but termination predicate fail; missing slot={missing}; looping"
                );
                let injection = format!(
                    "You declared done but the required slot '{missing}' is not covered. \
                    Ask one more concrete question about '{missing}', do NOT declare done yet."
                );
                // Write a nudge prompt and continue iteration (do not break).
                let nudge_json = build_turn_prompt_json(
                    &meta_prompt_content,
                    &coverage_summary,
                    &state.last_3_turns,
                    turn_index,
                    Some(&injection),
                );
                let _ = fs::write(&prompt_file_path, &nudge_json);
                // Write the turn capsule with TerminationGated outcome.
                let mut gated_record = attempt_record.clone();
                gated_record.outcome = spec_capsule::GrillAttemptOutcome::TerminationGated;
                let gated_capsule = spec_capsule::GrillTurnCapsuleBody {
                    grill_attempt_record: gated_record,
                    ..turn_capsule
                };
                let gated_cid = spec_capsule::write_grill_turn_capsule(workspace, &gated_capsule)
                    .unwrap_or_default();
                state.turn_cids.push(gated_cid.clone());
                last_turn_cid = Some(gated_cid);
                // Don't advance coverage or display question — re-loop this turn.
                turn_index -= 1; // restore so the while increment brings us back to same index
                state.turn_count = state.turn_count.saturating_sub(1);
                continue;
            }
        }

        // ── 4j. Display question to user and read answer ──────────────────────
        let question = payload.question.clone().unwrap_or_default();
        println!("\n{question}\n");
        let mut answer_buf = String::new();
        io::stdin()
            .lock()
            .read_line(&mut answer_buf)
            .map_err(|e| SpecError::Io(format!("read stdin: {e}")))?;
        let user_answer = answer_buf.trim().to_string();

        // ── 4k. CAS-store user answer ─────────────────────────────────────────
        let answer_cid = cas_store_user_answer(workspace, &capsules_dir, &user_answer);

        // ── 4l. Triage step (R2 §A5) ─────────────────────────────────────────
        let triage_turn_id = format!("turn-{turn_index}-triage");
        let triage_result = shell_llm_triage(
            &exe,
            workspace,
            &user_answer,
            &question,
            lang,
            &capsules_dir,
            &triage_turn_id,
        );

        let triage_class = triage_result
            .as_ref()
            .map(|r| r.class.as_str())
            .unwrap_or("gibberish")
            .to_string();

        match triage_class.as_str() {
            "relevant" => {
                state.triage_calls_relevant += 1;
                // Proceed to coverage update below.
            }
            "off_topic" => {
                state.triage_calls_non_relevant += 1;
                let nudge = match lang {
                    Lang::Zh => "能换一种说法吗？刚才听不太懂",
                    Lang::En => "Could you rephrase that? I didn't quite follow.",
                };
                println!("{nudge}");
                // Do NOT add to last_3_turns; do NOT advance coverage.
                // Write capsule with the triage info but don't break.
                let triage_capsule = spec_capsule::GrillTurnCapsuleBody {
                    user_answer_cid: answer_cid.clone(),
                    ..turn_capsule.clone()
                };
                let triage_cid = spec_capsule::write_grill_turn_capsule(workspace, &triage_capsule)
                    .unwrap_or_default();
                state.turn_cids.push(triage_cid);
                // Re-loop same turn: decrement so the while increment restores.
                turn_index -= 1;
                state.turn_count = state.turn_count.saturating_sub(1);
                continue;
            }
            _ => {
                // "abusive" or "gibberish"
                state.triage_calls_non_relevant += 1;
                state.non_relevant_count += 1;
                let nudge = match lang {
                    Lang::Zh => "您似乎在测试我，可以继续吗？",
                    Lang::En => "You seem to be testing me — shall we continue?",
                };
                println!("{nudge}");
                if state.non_relevant_count >= 2 {
                    termination_reason = "user_input_unparseable".to_string();
                    // Write a minimal turn capsule.
                    let fail_capsule = spec_capsule::GrillTurnCapsuleBody {
                        user_answer_cid: answer_cid.clone(),
                        ..turn_capsule.clone()
                    };
                    let fc_cid = spec_capsule::write_grill_turn_capsule(workspace, &fail_capsule)
                        .unwrap_or_default();
                    state.turn_cids.push(fc_cid);
                    break;
                }
                // Re-loop same turn.
                turn_index -= 1;
                state.turn_count = state.turn_count.saturating_sub(1);
                continue;
            }
        }

        // ── 4m. Update coverage state ─────────────────────────────────────────
        state.update_slots(&payload.covered_slots);
        prev_covered = payload.covered_slots.clone();

        // Push to last_3 context window.
        state.push_turn(question.clone(), user_answer.clone());
        state.all_user_answers.push(user_answer.clone());

        // Write turn capsule with answer CID.
        let final_turn_capsule = spec_capsule::GrillTurnCapsuleBody {
            user_answer_cid: answer_cid.clone(),
            ..turn_capsule
        };
        let turn_cid = spec_capsule::write_grill_turn_capsule(workspace, &final_turn_capsule)
            .unwrap_or_default();
        state.turn_cids.push(turn_cid.clone());
        last_turn_cid = Some(turn_cid);

        // ── 4n. Continue loop ─────────────────────────────────────────────────
    } // end while

    // ── 5. Post-loop synthesis and session capsule ────────────────────────────
    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let final_spec_capsule_cid: String;
    let total_turns = state.turn_count;
    let partial_session = termination_reason != "llm_done_predicate_pass";

    if !partial_session && total_turns >= 4 {
        // ── 5a. Reconstruct Q/A pairs for synthesis ───────────────────────────
        // We use the canonical questions + all_user_answers collected during the
        // session. The driven session may have more or fewer than 8 answers
        // (depending on how many turns were needed). We pass as many as we have.
        // wrap_spec_md expects exactly 8 slots in slot order; if we have fewer,
        // we pad with placeholder text.
        let questions = canonical_questions(lang);
        let mut answers: Vec<String> = state.all_user_answers.clone();
        while answers.len() < 8 {
            answers.push("(not collected in driven session)".to_string());
        }
        answers.truncate(8);

        let model_id = cmd_llm::read_meta_model(workspace);
        let synthesis_body = if let Ok(api_key_env) = cmd_llm::read_meta_api_key_env(workspace) {
            if let Ok(api_key) = require_api_key(&api_key_env) {
                let synth_user_msg = build_synthesis_user_message(lang, &questions, &answers);
                let messages = vec![
                    ChatMessage::system(system_prompt(lang)),
                    ChatMessage::user(synth_user_msg),
                ];
                eprintln!("[spec driven] calling Meta LLM ({model_id}) to synthesise spec.md...");
                let meta_thinking_driven = cmd_llm::read_meta_thinking(workspace);
                match chat_complete_blocking(
                    &api_key,
                    &model_id,
                    &messages,
                    Some(3000),
                    Some(0.3),
                    meta_thinking_driven,
                ) {
                    Ok(result) => result.content,
                    Err(_) => synthesise_spec_md_no_llm(lang, &questions, &answers),
                }
            } else {
                synthesise_spec_md_no_llm(lang, &questions, &answers)
            }
        } else {
            synthesise_spec_md_no_llm(lang, &questions, &answers)
        };

        let model_id_for_wrap = cmd_llm::read_meta_model(workspace);
        let spec_md = wrap_spec_md(
            &synthesis_body,
            &questions,
            &answers,
            &model_id_for_wrap,
            false,
        );
        let spec_md_path = workspace.join("spec.md");
        fs::write(&spec_md_path, &spec_md)
            .map_err(|e| SpecError::Io(format!("write spec.md: {e}")))?;

        let cid = spec_capsule::write_spec_capsule(workspace, &spec_md, "grill_driven", logical_t)?;
        state.synthesis_calls = 1;
        final_spec_capsule_cid = cid;
    } else {
        // ── 5b. Partial session — no synthesis ────────────────────────────────
        final_spec_capsule_cid = String::new();
    }

    // ── 5c. Build and write GrillSessionCapsuleBody ───────────────────────────
    let session_body = spec_capsule::GrillSessionCapsuleBody {
        session_id: session_id.clone(),
        turn_cids: state.turn_cids.clone(),
        final_spec_capsule_cid: final_spec_capsule_cid.clone(),
        termination_reason: termination_reason.clone(),
        total_turns,
        partial_session,
        lang: lang_str(lang).to_string(),
        grill_attempt_tally: state.to_grill_attempt_tally(),
        logical_t,
    };

    let session_capsule_cid = spec_capsule::write_grill_session_capsule(workspace, &session_body)?;

    // ── 5d. Print results ─────────────────────────────────────────────────────
    println!("\nSession: {session_id}");
    println!("Session capsule CID: {session_capsule_cid}");
    println!("Spec capsule CID: {final_spec_capsule_cid}");
    println!("Turns: {total_turns} (status: {termination_reason})");
    if !partial_session {
        println!(
            "\nNext step: turingos generate --workspace {}",
            workspace.display()
        );
    }

    Ok(())
}

/// Write a minimal GrillTurnCapsuleBody for a failed turn (no TurnPayload available).
/// Returns the CID hex or error.
fn write_failed_turn_capsule(
    workspace: &Path,
    session_id: &str,
    turn_index: u32,
    parent_turn_cid: Option<&str>,
    outcome: spec_capsule::GrillAttemptOutcome,
    retry_index: u32,
    logical_t: u64,
) -> Result<String, spec_capsule::CapsuleError> {
    use turingosv4::runtime::grill_predicates::{
        PredicateBundle, PredicateFailureClass, PredicateVerdict,
    };

    let fail_verdict = PredicateVerdict::Fail(PredicateFailureClass::SchemaParseError);
    let bundle = PredicateBundle {
        p1_schema_parse_ok: fail_verdict.clone(),
        p2_kind_ok: PredicateVerdict::Pass,
        p3_slots_in_vocab: PredicateVerdict::Pass,
        p4_monotonic: PredicateVerdict::Pass,
        p5_turn_bounded: PredicateVerdict::Pass,
        p6_question_nonempty_lang: PredicateVerdict::Pass,
    };

    let attempt_record = spec_capsule::GrillAttemptRecord {
        schema_version: 1,
        session_id: session_id.to_string(),
        turn_index,
        model_id: String::new(),
        prompt_context_hash: String::new(),
        candidate_payload_cid: String::new(),
        outcome,
        token_counts: spec_capsule::GrillTokenCounts::default(),
        elapsed_ms: 0,
        retry_index,
    };

    let body = spec_capsule::GrillTurnCapsuleBody {
        session_id: session_id.to_string(),
        turn_index,
        prompt_capsule_cid: String::new(),
        user_answer_cid: None,
        parent_turn_cid: parent_turn_cid.map(|s| s.to_string()),
        grill_attempt_record: attempt_record,
        predicate_verdicts: bundle,
        turn_payload_snapshot: serde_json::Value::Null,
        logical_t,
    };

    spec_capsule::write_grill_turn_capsule(workspace, &body)
}

// ─────────────────────────────────────────────────────────────────────────────
// Canonical 8-question flow (research-derived)
// ─────────────────────────────────────────────────────────────────────────────

fn canonical_questions(lang: Lang) -> Vec<String> {
    match lang {
        Lang::Zh => vec![
            // Q1 — The Job (JTBD opener; no jargon)
            "先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？\
比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？".into(),
            // Q2 — The Anchor (let user supply anchor)
            "有没有哪个网站 / App / 小工具，跟你想要的『有点像』？不用一模一样，一两个相似的地方就行。\
（如果想不出来：那纸笔 / Excel / 微信群里现在是怎么做的？）".into(),
            // Q3 — Data model in plain words
            "想象关掉电脑明天再打开，这个工具应该还『记得』哪些东西？比如团购账本会记得：\
每个人的名字、买了什么、付了多少、还欠多少。你的工具要记得什么？".into(),
            // Q4 — First-click walkthrough
            "假设我是你的用户，第一次打开这个工具——我看到什么？然后我点什么？然后呢？\
一步一步告诉我，直到我完成一件事。".into(),
            // Q5 — Weird-user test (Mom-Test sin-3 antidote, specifics)
            "如果有个奇怪的用户，故意乱点乱填——比如把『金额』填成『哈哈哈』，\
或者同一个名字录入 50 遍——你希望工具怎么办？报错？忽略？还是有别的反应？".into(),
            // Q6 — Disappointment boundary (inverse framing surfaces real priorities)
            "如果这个工具突然多了一个功能，你反而会觉得『搞这个干嘛，反而把简单的事弄复杂了』——\
是什么功能？说两三个。".into(),
            // Q7 — Success test (past-cost framing)
            "用了一个月之后，你怎么判断『这个工具是有用的』？不是『感觉不错』那种——\
是具体能数出来或看得见的事。比如：『我妈现在不用每周日花两小时算账了。』".into(),
            // Q8 — Playback / mirror (Voss labeling)
            "（最后一题）下面我会把前面听到的复述一遍，请你看看哪里我听错了或听漏了——\
别客气，挑错就是帮我。如果你想直接补充什么，请在这里写出来。".into(),
        ],
        Lang::En => vec![
            "Forget about code for now. Tell me about a recent moment when you thought \
'I wish I had a tool for this.' For example: 'My mom does community group-buy accounting \
every week in Excel and it's painful.' What's your story?".into(),
            "Is there a website, app, or tool that's even a little bit like what you want? \
Doesn't have to be exact — just one or two similar pieces. (If you can't name one: \
'How do you do this today with paper, Excel, or a chat group?')".into(),
            "Imagine you close the program and open it tomorrow — what should it still \
'remember'? A group-buy tracker remembers: each person's name, what they bought, how \
much they paid, what they still owe. What does yours remember?".into(),
            "Pretend I'm your user opening this for the first time. What do I see? What do \
I click? Then what? Walk me through, step by step, until I finish one task.".into(),
            "If a weird user messes around — types 'lolol' into the price field, or enters \
the same name 50 times — what should the tool do? Show an error? Ignore it? Something else?".into(),
            "If the tool grew a new feature and your reaction was 'why did you add this, \
you've made the simple thing complicated' — name two or three such features.".into(),
            "After one month of using it, how do you know it's actually working? Not 'feels \
nice' — something countable or visible. Like: 'My mom no longer spends two hours every \
Sunday doing the math.'".into(),
            "(Last question) I'll play back what I heard. Tell me which line is wrong or \
incomplete — corrections help me. If you want to add anything directly, write it here.".into(),
        ],
    }
}

/// A8b (2026-05-19): synthesis prompt moved from inline Rust literal to
/// runtime fs load from `<workspace>/assets/prompts/grill_synthesis_{lang}.md`.
/// Mirrors F4 meta-prompt pattern. Enables A/B without rebuild.
/// Fallback (read failure): bake-in v1 content via include_str! at compile time
/// so the binary never crashes on missing assets.
fn system_prompt(lang: Lang) -> String {
    system_prompt_from(lang, std::path::Path::new("."))
}

fn system_prompt_from(lang: Lang, workspace: &std::path::Path) -> String {
    let (filename, fallback) = match lang {
        Lang::Zh => (
            "grill_synthesis_zh.md",
            include_str!("../../../assets/prompts/grill_synthesis_zh.md"),
        ),
        Lang::En => (
            "grill_synthesis_en.md",
            include_str!("../../../assets/prompts/grill_synthesis_en.md"),
        ),
    };
    let path = workspace.join("assets/prompts").join(filename);
    match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => fallback.to_string(),
    }
}

fn build_synthesis_user_message(lang: Lang, questions: &[String], answers: &[String]) -> String {
    let intro = match lang {
        Lang::Zh => "下面是 8 个 Q/A，请按系统提示综合 spec.md：",
        Lang::En => "Here are the 8 Q/A pairs — synthesise spec.md per the system prompt:",
    };
    let mut s = String::new();
    s.push_str(intro);
    s.push_str("\n\n");
    for i in 0..8 {
        s.push_str(&format!("Q{}: {}\n", i + 1, questions[i]));
        s.push_str(&format!("A{}: {}\n\n", i + 1, answers[i]));
    }
    s
}

// ─────────────────────────────────────────────────────────────────────────────
// Interactive stdin gather (TTY)
// ─────────────────────────────────────────────────────────────────────────────

fn interactive_gather(questions: &[String]) -> Result<Vec<String>, SpecError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut answers = Vec::with_capacity(8);

    println!("turingos spec — 8-question grill");
    println!("Type your answer; press ENTER on an empty line to submit each answer.");
    println!();

    for (i, q) in questions.iter().enumerate() {
        println!("Q{}: {}", i + 1, q);
        print!("> ");
        stdout.flush().map_err(|e| SpecError::Io(e.to_string()))?;
        let mut buf = String::new();
        loop {
            let mut line = String::new();
            let n = stdin
                .lock()
                .read_line(&mut line)
                .map_err(|e| SpecError::Io(e.to_string()))?;
            if n == 0 {
                break;
            }
            if line.trim().is_empty() {
                break;
            }
            buf.push_str(&line);
        }
        answers.push(buf.trim().to_string());
        println!();
    }
    Ok(answers)
}

fn load_answers_from_file(path: &Path) -> Result<Vec<String>, SpecError> {
    let raw = fs::read_to_string(path)
        .map_err(|e| SpecError::BadAnswersFile(format!("read {}: {e}", path.display())))?;
    let parsed: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| SpecError::BadAnswersFile(format!("JSON parse: {e}")))?;
    let arr = parsed
        .as_array()
        .ok_or_else(|| SpecError::BadAnswersFile("expected top-level JSON array".into()))?;
    let mut out = Vec::with_capacity(arr.len());
    for (i, v) in arr.iter().enumerate() {
        let s = v
            .as_str()
            .ok_or_else(|| SpecError::BadAnswersFile(format!("element {i} is not a string")))?;
        out.push(s.to_string());
    }
    Ok(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// Transcript JSONL persistence
// ─────────────────────────────────────────────────────────────────────────────

struct TurnRecord {
    role: String,
    content: String,
    model: Option<String>,
    usage_total_tokens: u64,
}

fn write_transcript_jsonl(path: &Path, turns: &[TurnRecord]) -> Result<(), SpecError> {
    let mut out = String::new();
    for t in turns {
        let model = t.model.as_deref().unwrap_or("");
        let obj = serde_json::json!({
            "role": t.role,
            "content": t.content,
            "model": model,
            "usage_total_tokens": t.usage_total_tokens,
        });
        out.push_str(&obj.to_string());
        out.push('\n');
    }
    fs::write(path, out).map_err(|e| SpecError::Io(format!("write transcript: {e}")))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// LLM-less synthesis fallback (for --skip-llm CAS-wire smoke tests)
// ─────────────────────────────────────────────────────────────────────────────

fn synthesise_spec_md_no_llm(lang: Lang, questions: &[String], answers: &[String]) -> String {
    let mut s = String::new();
    match lang {
        Lang::Zh => {
            s.push_str("## 一句话目标\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 我们要做什么 (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 像谁 (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## 程序要记住的东西 (Memory)\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## 第一次使用 (First Run)\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## 不能搞坏的情况 (Robustness)\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## 故意不做的 (Out of Scope)\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## 算成功 (Acceptance)\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## 用户补充\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## 一句话给 AI 编程员\n\n");
            s.push_str("根据上面的 Goal / Memory / First Run 实现一个最小可用版本。");
        }
        Lang::En => {
            s.push_str("## One-line Goal\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## What We're Building (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## Like What (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## What the Program Remembers\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## First Run\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## What It Must Not Break On\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## Deliberately NOT Doing\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## Success Looks Like\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## User Additions\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## One-line Brief to AI Coder\n\n");
            s.push_str("Implement a minimal version using the Goal / Memory / First Run above.");
        }
    }
    let _ = questions; // suppress unused warning
    s.push_str("\n\n<!-- TURINGOS_SPEC_END -->\n");
    s
}

/// Wrap the LLM-synthesised body with a header (model id + timestamp) and an
/// appendix (raw Q/A for audit). The CAS capsule hashes this WHOLE blob, so
/// future replay can derive both the formatted spec and the raw transcript
/// from the single capsule CID.
fn wrap_spec_md(
    body: &str,
    questions: &[String],
    answers: &[String],
    model_id: &str,
    skipped_llm: bool,
) -> String {
    let mut s = String::new();
    s.push_str("# TuringOS Spec (Phase 6.3)\n\n");
    s.push_str(&format!(
        "> Generated by `turingos spec` — meta model: `{model_id}`"
    ));
    if skipped_llm {
        s.push_str(" (skip-llm: no synthesis call made)");
    }
    s.push_str("\n\n");
    s.push_str(body.trim_end());
    s.push_str("\n\n---\n\n");
    s.push_str("## Appendix — Raw Q/A (for audit)\n\n");
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        s.push_str(&format!("**Q{}**: {q}\n\n", i + 1));
        s.push_str(&format!("**A{}**: {a}\n\n", i + 1));
    }
    s
}
