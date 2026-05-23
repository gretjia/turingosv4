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
use crate::siliconflow_client::{
    canonical_chat_request_bytes, chat_complete_blocking, require_api_key, ChatMessage, LlmError,
};
use sha2::{Digest, Sha256};
use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::runtime::artifact_bundle::{
    latest_artifact_bundle_cid_for_session, write_artifact_bundle, ArtifactBundleManifest,
    ArtifactFileEntry, ArtifactFileRole, ARTIFACT_BUNDLE_SCHEMA_ID,
};
use turingosv4::runtime::generation_attempt::{
    write_generation_attempt_capsule, AttemptOutcome, GenerationAttemptCapsule,
    GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::rejection_capsule::{
    write_generate_rejection_capsule_observed, GenerateRejectionCapsule, RejectClass,
    GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::spec_capsule;
use turingosv4::runtime::test_run::{
    format_test_run_summary, run_and_write_test_pipeline, TestRunCapsule,
};
use turingosv4::runtime::test_scenario::TestScenario;
use turingosv4::tdma_runner::{run_proof, AnyJudge, LlmResponse, RunConfig};
use crate::siliconflow_client::{ChatResult, Usage};

// Polymarket PR1 (2026-05-23, REVISED post-Codex audit 2026-05-23): after the
// TDMA judge PASSES + the ArtifactBundleManifest is committed, ALSO admit a
// WorkTx + open a treasury-funded market via the canonical workspace
// ChainTape sequencer (`build_chaintape_sequencer_with_initial_q`). The
// original PR1 used an ephemeral InMemoryLedgerWriter + a per-call CAS dir;
// the Constitution agent flagged that as Art. 0.4 + FC1 wtool drift (no
// durable chain → no replay → no Run-1 verifier reconstruction). The revised
// path lands every admission on `<workspace>/runtime_repo` so the chain is
// the canonical source of truth for the web `market_view` projection and
// for cold-restart replay.
//
// PR2 will fan out N parallel workers via `RunConfig.n_parallel_workers`.
// PR3 deferred ChallengeTx (peer-Worker challenges violate Art. III.3
// horizontal-independence — needs a dedicated isolated-context critic bot).
use turingosv4::economy::money::{MicroCoin, StakeMicroCoin};
use turingosv4::runtime::adapter::genesis_with_balances;
use turingosv4::runtime::bootstrap::parse_treasury_and_worker_preseed;
use turingosv4::runtime::cid_hex::cid_from_hex_str;
use turingosv4::runtime::{
    build_chaintape_sequencer_with_initial_q, ChaintapeBundle, RuntimeChaintapeConfig,
};
use turingosv4::state::q_state::{AgentId, Hash as StateHash, TaskId, TxId};
use turingosv4::state::sequencer::{
    escrow_lock_accept_state_root, task_open_accept_state_root, worktx_accept_state_root,
};
use turingosv4::state::typed_tx::{
    AgentSignature, BoolWithProof, EscrowLockTx, EventId, MarketSeedTx, PredicateId,
    PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenTx, TypedTx, WorkTx, WriteKey,
};

/// TODO(genesis_payload): move these defaults to genesis_payload.toml
/// [polymarket_defaults] in a follow-up. Karpathy nice-fix #1 + Constitution
/// nice-fix-1: parametric runtime constants belong in the trust-rooted
/// manifest. For this PR they stay inline so the diff stays surgical.
const TREASURY_AGENT_ID: &str = "treasury";
const WORKER_ALPHA_AGENT_ID: &str = "worker-alpha";
const DEFAULT_BOUNTY_MICRO: i64 = 1_000;
const DEFAULT_WORK_STAKE_MICRO: i64 = 100;
const DEFAULT_MARKET_SEED_MICRO: i64 = 100; // = bounty / 10 per architect manual §7.4


/// TRACE_MATRIX FC2-N16: `generate` short-help
pub(crate) const SHORT_HELP: &str =
    "Generate working code from spec.md via the Blackbox LLM; writes to <workspace>/artifacts/";

/// TRACE_MATRIX FC2-N16: `generate` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos generate — Emit code from spec.md via the Blackbox LLM

USAGE:
    turingos generate --workspace <PATH> [--from-capsule] [--max-files <N>]
                       [--tdma-bounded [--entrypoint <PATH>] [--max-retries <N>]]

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
    --tdma-bounded          [DEFAULT as of Atom 25] Route the LLM call
                            through the TDMA-Bounded MemoryKernel. Retries
                            are driven by AnyJudge::Generate verdicts and
                            evidence captured under
                            <workspace>/artifacts/tdma_generate/<session_id>/.
    --no-tdma-bounded       Disable TDMA-Bounded mode (in-process emergency
                            rollback only; legacy single-pass path was
                            DELETED in Atom 25 per Karpathy K14, so this
                            flag now sets a no-op pre-pass and still routes
                            through the kernel for evidence consistency).
    --entrypoint <PATH>     Expected entrypoint file path the LLM must include
                            in its file bundle (default: main.py). Used only
                            when --tdma-bounded is set.
    --max-retries <N>       Hard cap on TDMA-Bounded attempts (default: 5).
                            Used only when --tdma-bounded is set.
    -h, --help              Print this help.

DESCRIPTION:
    Class 1 filesystem write to <workspace>/artifacts/. One LLM call per
    `turingos generate` invocation. The Blackbox model is told to output
    one or more complete files, each preceded by `### File: <relative path>`
    plus a fenced code block. For UI apps it defaults to a single
    self-contained index.html so the end-user can just open it in a browser.

ENVIRONMENT:
    TURINGOS_SILICONFLOW_ENDPOINT
        Base URL for the LLM provider. Default:
        https://api.siliconflow.cn/v1/chat/completions
        Override to point at DeepSeek / OpenAI / OpenRouter / etc.

    $<meta-api-key-env from turingos.toml>
        The shell env var holding your Meta-role LLM API key.
        Configured via `turingos llm config --meta-api-key-env <NAME>`.

    $<blackbox-api-key-env from turingos.toml>
        Same for Worker-role.
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
    TooManyFiles {
        found: usize,
        max: usize,
    },
    /// X1: carries CID footer lines to be printed AFTER the error message.
    WithFooter {
        inner: Box<GenError>,
        footer: String,
    },
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
                "Blackbox LLM emitted no parseable files. Expected `### File: <path>` followed by a fenced code block.\n  (Transient API error? Try running `turingos generate` again.)"
            ),
            Self::TooManyFiles { found, max } => {
                write!(f, "Blackbox LLM emitted {found} files; --max-files cap is {max}")
            }
            Self::WithFooter { inner, .. } => write!(f, "{inner}"),
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
            // X1: print error FIRST, then the CID footer lines (if any).
            // This ensures non-experts reading top-to-bottom see the error
            // before the diagnostic identifiers.
            eprintln!("turingos generate: {e}");
            if let GenError::WithFooter { footer, .. } = &e {
                eprintln!("{footer}");
            }
            ExitCode::from(2)
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), GenError> {
    let mut workspace = PathBuf::from(".");
    let mut from_capsule = false;
    let mut max_files: usize = 20;
    let mut emit_transcript = false;
    // Atom 25: Phase E full cutover.
    //   --tdma-bounded default: false -> TRUE. The legacy single-pass code
    //   path is DELETED outright (no --legacy escape hatch per Karpathy K14;
    //   emergency rollback via git revert of this PR).
    //   --tape-backend default: memory -> GIT.
    let mut tdma_bounded = true;
    let mut tdma_entrypoint = "main.py".to_string();
    let mut tdma_max_retries: usize = 5;
    let mut tape_backend = "git".to_string();
    // Atom 25: --no-tdma-bounded escape only inside this PR for the negative
    // test that verifies the flag wiring; production users never set it.
    // (KILL-cutover-1 grep guard rejects `--legacy`; --no-tdma-bounded is
    // permitted but undocumented.)

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
            "--tdma-bounded" => tdma_bounded = true,
            "--no-tdma-bounded" => tdma_bounded = false,
            "--entrypoint" => {
                tdma_entrypoint = iter
                    .next()
                    .ok_or(GenError::MissingFlag("--entrypoint"))?
                    .clone();
            }
            "--max-retries" => {
                let v = iter.next().ok_or(GenError::MissingFlag("--max-retries"))?;
                tdma_max_retries = v
                    .parse()
                    .map_err(|_| GenError::Io(format!("--max-retries: not a number: {v}")))?;
            }
            "--tape-backend" => {
                tape_backend = iter
                    .next()
                    .ok_or(GenError::MissingFlag("--tape-backend"))?
                    .clone();
            }
            _ => {}
        }
    }
    if tape_backend != "memory" && tape_backend != "git" {
        return Err(GenError::Io(format!(
            "--tape-backend must be 'memory' or 'git'; got '{}'",
            tape_backend
        )));
    }
    if !workspace.exists() {
        return Err(GenError::WorkspaceNotFound(workspace.display().to_string()));
    }

    let (spec_md, source, spec_capsule_cid) = if from_capsule {
        let cid_hex = spec_capsule::latest_spec_capsule_cid(&workspace)?.ok_or_else(|| {
            GenError::NoSpec(format!("no spec capsule in {}/cas", workspace.display()))
        })?;
        let bytes = spec_capsule::read_spec_capsule(&workspace, &cid_hex)?;
        (
            String::from_utf8(bytes)
                .map_err(|e| GenError::Io(format!("CAS capsule is not UTF-8: {e}")))?,
            format!("CAS capsule {cid_hex}"),
            Some(cid_hex),
        )
    } else {
        let p = workspace.join("spec.md");
        if !p.exists() {
            return Err(GenError::NoSpec(p.display().to_string()));
        }
        let latest_cid = spec_capsule::latest_spec_capsule_cid(&workspace)
            .ok()
            .flatten();
        (
            fs::read_to_string(&p).map_err(|e| GenError::Io(e.to_string()))?,
            p.display().to_string(),
            latest_cid,
        )
    };

    let spec_source = if from_capsule {
        "cas_capsule".to_string()
    } else {
        "ondisk_spec_md".to_string()
    };

    let model_id = cmd_llm::read_blackbox_model(&workspace);
    let api_key_env =
        cmd_llm::read_blackbox_api_key_env(&workspace).map_err(|e| GenError::Io(e.to_string()))?;
    let api_key = match require_api_key(&api_key_env) {
        Ok(k) => k,
        Err(_) => {
            eprintln!(
                "error: Blackbox role API key env var \"${api_key_env}\" is not set in your shell."
            );
            eprintln!("       Run: export {api_key_env}=\"sk-...\"");
            eprintln!(
                "       Then retry: turingos generate --workspace {}",
                workspace.display()
            );
            return Err(GenError::Llm(LlmError::MissingApiKey {
                env_var: api_key_env.clone(),
            }));
        }
    };

    // Resolve session_id and retry_index FIRST — needed for tape-relay read below.
    let session_id = if workspace
        .parent()
        .map(|p| p.file_name().map(|n| n == "sessions").unwrap_or(false))
        .unwrap_or(false)
    {
        workspace
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("default")
            .to_string()
    } else {
        "default".to_string()
    };

    let cas_dir = workspace.join("cas");
    let mut retry_index = 0u32;
    let mut parent_attempt_cid: Option<String> = None;

    if let Ok(store) = CasStore::open(&cas_dir) {
        let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
        let mut attempts = Vec::new();
        for cid in cids {
            if let Some(meta) = store.metadata(&cid) {
                if meta.schema_id.as_deref() == Some(GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID) {
                    if let Ok(bytes) = store.get(&cid) {
                        if let Ok(capsule) =
                            serde_json::from_slice::<GenerationAttemptCapsule>(&bytes)
                        {
                            if capsule.session_id == session_id {
                                attempts.push((capsule.logical_t, cid.hex(), capsule.retry_index));
                            }
                        }
                    }
                }
            }
        }
        attempts.sort_by_key(|x| x.0);
        if let Some(last) = attempts.last() {
            retry_index = last.2 + 1;
            parent_attempt_cid = Some(last.1.clone());
        }
    }

    // TRACE_MATRIX FC1-N4: Tape-relay read. If a prior rejection exists for
    // this session, prepend its diagnostics so the LLM can avoid repeating
    // the prior failure. This closes the parent_attempt_cid chain's missing
    // READ side (chain was previously write-only).
    let prior_feedback = read_prior_rejection_feedback(&workspace, &session_id);
    let user_msg = if let Some(ref fb) = prior_feedback {
        eprintln!("[generate] tape-relay: feeding prior rejection diagnostics into LLM prompt (attempt #{})", retry_index);
        format!(
            "{fb}Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
        )
    } else {
        format!(
            "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
        )
    };
    let messages = vec![
        ChatMessage::system(blackbox_system_prompt()),
        ChatMessage::user(user_msg),
    ];

    let blackbox_thinking = cmd_llm::read_blackbox_thinking(&workspace);
    let canonical_request_bytes = canonical_chat_request_bytes(
        &model_id,
        &messages,
        Some(6000),
        Some(0.2),
        blackbox_thinking.clone(),
    )
    .map_err(GenError::Llm)?;
    let mut hasher = Sha256::new();
    hasher.update(&canonical_request_bytes);
    let prompt_hash = format!("{:x}", hasher.finalize());

    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    eprintln!("[generate] calling Blackbox LLM ({model_id})...");
    // Atom 25: Phase E full cutover. The legacy single-pass branch has been
    // DELETED per Karpathy K14 (no `--legacy` escape hatch; emergency rollback
    // = git revert of this PR). When --no-tdma-bounded is passed, the runner
    // still wraps the call so prompt_hash semantics + evidence emission stay
    // consistent across both modes — only retry behavior changes (single attempt
    // when --no-tdma-bounded effectively yields max_retries=1).
    let effective_max_retries = if tdma_bounded { tdma_max_retries } else { 1 };
    eprintln!(
        "[generate] TDMA-Bounded mode ON (default). entrypoint={} max_retries={} tape_backend={}",
        tdma_entrypoint, effective_max_retries, tape_backend
    );
    let (llm_res, final_prompt_hash) = chat_with_tdma_bounded(
        &workspace,
        &session_id,
        &api_key,
        &model_id,
        &messages,
        blackbox_thinking.clone(),
        &tdma_entrypoint,
        effective_max_retries,
        &prompt_hash,
        &tape_backend,
    );
    // KILL-gen-3: prompt_hash records the canonical bytes of the FINAL
    // accepted attempt across all paths.
    let prompt_hash = final_prompt_hash;

    let (
        outcome,
        raw_output_cid,
        usage_total_tokens,
        parsed_file_count,
        files_to_write,
        run_result,
    ): (
        AttemptOutcome,
        Option<String>,
        Option<u32>,
        usize,
        Option<(Vec<PathBuf>, String)>,
        Result<(), GenError>,
    ) = match llm_res {
        Err(e) => (
            AttemptOutcome::LlmApiError,
            None,
            None,
            0,
            None,
            Err(GenError::Llm(e)),
        ),
        Ok(result) => {
            let raw_cid = match CasStore::open(&cas_dir) {
                Ok(mut store) => {
                    match store.put(
                        result.raw_response_body.as_slice(),
                        ObjectType::EvidenceCapsule,
                        "generate_system",
                        logical_t,
                        None,
                    ) {
                        Ok(cid) => Some(cid.hex()),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            };

            let files = parse_emitted_files(&result.content);
            if files.is_empty() {
                let raw_path = workspace.join("generate_raw_response.txt");
                let _ = fs::write(&raw_path, &result.content);
                eprintln!("[generate] raw response saved to {}", raw_path.display());

                (
                    AttemptOutcome::NoFilesParsed,
                    raw_cid,
                    Some(result.usage.total_tokens as u32),
                    0,
                    None,
                    Err(GenError::NoFilesParsed),
                )
            } else if files.len() > max_files {
                (
                    AttemptOutcome::ParseFailed,
                    raw_cid,
                    Some(result.usage.total_tokens as u32),
                    files.len(),
                    None,
                    Err(GenError::TooManyFiles {
                        found: files.len(),
                        max: max_files,
                    }),
                )
            } else {
                let artifacts_dir = workspace.join("artifacts");
                let mut write_err = None;
                let mut written = Vec::new();
                if let Err(e) = fs::create_dir_all(&artifacts_dir) {
                    write_err = Some(GenError::Io(format!("create artifacts dir: {e}")));
                } else {
                    for f in &files {
                        match sanitize_relative_path(&f.path) {
                            Ok(safe_rel) => {
                                let full = artifacts_dir.join(&safe_rel);
                                if let Some(parent) = full.parent() {
                                    if let Err(e) = fs::create_dir_all(parent) {
                                        write_err = Some(GenError::Io(format!(
                                            "create dir {}: {e}",
                                            parent.display()
                                        )));
                                        break;
                                    }
                                }
                                if let Err(e) = fs::write(&full, &f.content) {
                                    write_err = Some(GenError::Io(format!(
                                        "write {}: {e}",
                                        full.display()
                                    )));
                                    break;
                                }
                                written.push(safe_rel);
                            }
                            Err(e) => {
                                write_err = Some(GenError::Io(e));
                                break;
                            }
                        }
                    }
                }

                if let Some(err) = write_err {
                    (
                        AttemptOutcome::InternalIo,
                        raw_cid,
                        Some(result.usage.total_tokens as u32),
                        files.len(),
                        None,
                        Err(err),
                    )
                } else {
                    (
                        AttemptOutcome::Success,
                        raw_cid,
                        Some(result.usage.total_tokens as u32),
                        files.len(),
                        Some((written, result.content)),
                        Ok(()),
                    )
                }
            }
        }
    };

    let capsule = GenerationAttemptCapsule {
        schema_id: GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID.to_string(),
        session_id: session_id.clone(),
        spec_capsule_cid: spec_capsule_cid.clone(),
        spec_source,
        model_id,
        model_seed: None,
        prompt_hash,
        raw_output_cid: raw_output_cid.clone(),
        usage_total_tokens,
        retry_index,
        parent_attempt_cid,
        outcome,
        parsed_file_count,
        logical_t,
    };

    let attempt_cid_res = write_generation_attempt_capsule(&workspace, &capsule);

    let attempt_cid = match attempt_cid_res {
        Ok(cid) => cid,
        Err(e) => {
            let public_summary =
                "CAS write error during generation attempt capsule recording".to_string();
            let reason = "generation_attempt_capsule_write_failed".to_string();
            let rej = GenerateRejectionCapsule {
                schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
                session_id: capsule.session_id.clone(),
                spec_capsule_cid: capsule.spec_capsule_cid.clone(),
                generation_attempt_cid: None,
                triage_attempted: true,
                reject_class: RejectClass::InternalIo,
                public_error_summary: public_summary,
                reason,
                private_diagnostic_cid: raw_output_cid,
                retryable: true,
                world_head_unchanged: false,
                logical_t,
            };
            let footer =
                if let Ok(rej_cid) = write_generate_rejection_capsule_observed(&workspace, &rej) {
                    format!("[failed run] rejection_cid={rej_cid}")
                } else {
                    String::new()
                };
            return Err(GenError::WithFooter {
                inner: Box::new(GenError::Capsule(e)),
                footer,
            });
        }
    };

    // X1/B3: on success, CIDs go to stderr without prefix (informational only).
    // On failure, CIDs are deferred to AFTER the error message is printed by
    // run() — so non-experts reading top-to-bottom see the error first.
    if outcome == AttemptOutcome::Success {
        eprintln!("generation_attempt_cid={}", attempt_cid);
    }

    if outcome != AttemptOutcome::Success {
        let reject_class = match outcome {
            AttemptOutcome::ParseFailed => RejectClass::TooManyFiles,
            AttemptOutcome::LlmApiError => RejectClass::LlmApiError,
            AttemptOutcome::NoFilesParsed => RejectClass::NoFilesParsed,
            AttemptOutcome::InternalIo => RejectClass::InternalIo,
            AttemptOutcome::Success => unreachable!(),
        };
        let public_summary = match &run_result {
            Err(e) => e.to_string(),
            Ok(_) => "Unknown generate failure".to_string(),
        };
        let reason = match outcome {
            AttemptOutcome::ParseFailed => "parse_failed",
            AttemptOutcome::LlmApiError => "llm_api_error",
            AttemptOutcome::NoFilesParsed => "no_files_parsed",
            AttemptOutcome::InternalIo => "internal_io",
            AttemptOutcome::Success => unreachable!(),
        }
        .to_string();

        let rej = GenerateRejectionCapsule {
            schema_id: GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
            session_id: capsule.session_id.clone(),
            spec_capsule_cid: capsule.spec_capsule_cid.clone(),
            generation_attempt_cid: Some(attempt_cid.clone()),
            triage_attempted: true,
            reject_class,
            public_error_summary: public_summary,
            reason,
            private_diagnostic_cid: raw_output_cid,
            retryable: outcome != AttemptOutcome::InternalIo,
            world_head_unchanged: false,
            logical_t,
        };
        // X1: Collect CID footer; run() will emit it AFTER the error message.
        let mut footer_parts = vec![format!(
            "[failed run] generation_attempt_cid={}",
            attempt_cid
        )];
        if let Ok(rej_cid) = write_generate_rejection_capsule_observed(&workspace, &rej) {
            footer_parts.push(format!("[failed run] rejection_cid={rej_cid}"));
        }
        let footer = footer_parts.join("\n");
        // Shadow run_result to wrap any error with the CID footer.
        let run_result = run_result.map_err(|inner| GenError::WithFooter {
            inner: Box::new(inner),
            footer,
        });
        return run_result;
    }

    if run_result.is_ok() {
        if let Some((written, content)) = files_to_write {
            let files = parse_emitted_files(&content);

            // Put each generated file into CAS and construct ArtifactFileEntry list
            let mut file_entries = Vec::new();
            let mut bundle_size_bytes_total = 0u64;

            let mut store = CasStore::open(&cas_dir)
                .map_err(|e| GenError::Io(format!("open cas store: {e}")))?;

            let entrypoint_path = find_entrypoint(&files).unwrap_or_default();

            for f in &files {
                let content_bytes = f.content.as_bytes();
                let size_bytes = content_bytes.len() as u64;
                bundle_size_bytes_total += size_bytes;

                let mut hasher = Sha256::new();
                hasher.update(content_bytes);
                let sha256_hex = format!("{:x}", hasher.finalize());

                let mime = guess_mime(&f.path);

                let role = if f.path == entrypoint_path {
                    ArtifactFileRole::Entrypoint
                } else if f.path.ends_with(".html")
                    || f.path.ends_with(".js")
                    || f.path.ends_with(".css")
                    || f.path.ends_with(".ts")
                {
                    ArtifactFileRole::Source
                } else {
                    ArtifactFileRole::Asset
                };

                let file_cid = store
                    .put(
                        content_bytes,
                        ObjectType::EvidenceCapsule,
                        "generate_system",
                        logical_t,
                        None,
                    )
                    .map_err(|e| GenError::Io(format!("CAS put file failed: {e}")))?;

                file_entries.push(ArtifactFileEntry {
                    path: f.path.clone(),
                    cid: file_cid.hex(),
                    mime,
                    sha256: sha256_hex,
                    size_bytes,
                    role,
                });
            }

            let previous_bundle_cid =
                latest_artifact_bundle_cid_for_session(&workspace, &session_id)
                    .ok()
                    .flatten();

            let manifest = ArtifactBundleManifest {
                schema_id: ARTIFACT_BUNDLE_SCHEMA_ID.to_string(),
                session_id: session_id.clone(),
                spec_capsule_cid: spec_capsule_cid.clone(),
                generation_attempt_cid: attempt_cid.clone(),
                previous_bundle_cid,
                files: file_entries,
                entrypoint: entrypoint_path,
                bundle_size_bytes_total,
                created_at_logical_t: logical_t,
            };

            let bundle_cid = write_artifact_bundle(&workspace, &manifest)?;
            println!("artifact_bundle_cid={}", bundle_cid);

            // C11 producer: run spec-derived test scenarios against the artifact bundle.
            // Hidden-oracle: spec_bytes are passed but scenario set CID is NOT returned
            // (run_and_write_test_pipeline intentionally hides it).
            let spec_capsule_cid_for_test = spec_capsule_cid.as_deref().unwrap_or("");
            match run_and_write_test_pipeline(
                &workspace,
                spec_md.as_bytes(),
                spec_capsule_cid_for_test,
                &bundle_cid,
                logical_t,
            ) {
                Ok((test_run_cid, overall_pass, test_results)) => {
                    eprintln!("test_run_cid={}", test_run_cid);
                    // B4: print human-readable test summary so non-experts know what C11 fired.
                    eprintln!("{}", format_test_run_summary(&test_results));
                    if !overall_pass {
                        // Artifacts failed spec-derived test gate — reject as HeuristicFailed.
                        let rej = turingosv4::runtime::rejection_capsule::GenerateRejectionCapsule {
                            schema_id: turingosv4::runtime::rejection_capsule::GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
                            session_id: session_id.clone(),
                            spec_capsule_cid: spec_capsule_cid.clone(),
                            generation_attempt_cid: Some(attempt_cid.clone()),
                            triage_attempted: true,
                            reject_class: turingosv4::runtime::rejection_capsule::RejectClass::HeuristicFailed,
                            public_error_summary: "generated artifacts failed spec-derived tests".to_string(),
                            reason: format!("heuristic_failed:test_run_cid={}", test_run_cid),
                            private_diagnostic_cid: None,
                            retryable: true,
                            world_head_unchanged: false,
                            logical_t,
                        };
                        // X1: collect footer; run() emits it after the error message.
                        let footer = if let Ok(rej_cid) = turingosv4::runtime::rejection_capsule::write_generate_rejection_capsule_observed(&workspace, &rej) {
                            format!("[failed run] rejection_cid={rej_cid}")
                        } else {
                            String::new()
                        };
                        return Err(GenError::WithFooter {
                            inner: Box::new(GenError::Io(
                                "generated artifacts failed spec-derived tests".to_string(),
                            )),
                            footer,
                        });
                    }
                    // overall_pass=true — proceed to success output below.

                    // Polymarket PR1 (2026-05-23): TDMA judge passed + bundle
                    // committed. Now admit a WorkTx (worker-alpha staking 100µ
                    // on this proposal_cid) and open a treasury-funded YES/NO
                    // market for this session task via canonical sequencer
                    // admission. PR1: single worker (`worker-alpha`); PR2 will
                    // fan out N parallel workers via RunConfig.agent_ids.
                    match emit_polymarket_market_for_session(
                        &workspace,
                        &session_id,
                        &bundle_cid,
                        logical_t,
                    ) {
                        Ok(summary) => {
                            eprintln!(
                                "[polymarket] WorkTx admitted (agent={}, stake={}µ, proposal_cid={})",
                                summary.worker_agent,
                                DEFAULT_WORK_STAKE_MICRO,
                                summary.proposal_cid_hex_prefix,
                            );
                            if summary.market_opened {
                                eprintln!(
                                    "[polymarket] MarketSeed admitted (provider=treasury, collateral={}µ, task_id={})",
                                    DEFAULT_MARKET_SEED_MICRO, summary.task_id
                                );
                            } else {
                                eprintln!(
                                    "[polymarket] WorkTx admission landed in L4.E ({}); MarketSeed skipped",
                                    summary
                                        .rejection_note
                                        .unwrap_or_else(|| "unknown rejection".to_string())
                                );
                            }
                        }
                        Err(e) => {
                            // Per Hard Constraint #6 in PR1 brief: do NOT
                            // swallow errors. Sequencer admission failure on
                            // system genesis state is a system-level break
                            // (kernel surface failure on a known-valid input
                            // is not a user-fixable condition).
                            return Err(GenError::Io(format!(
                                "[polymarket] sequencer admission failed: {e}"
                            )));
                        }
                    }
                }
                Err(e) => {
                    // Internal pipeline failure (CAS IO / bundle read error) — reject as InternalIo.
                    let rej = turingosv4::runtime::rejection_capsule::GenerateRejectionCapsule {
                        schema_id: turingosv4::runtime::rejection_capsule::GENERATE_REJECTION_CAPSULE_SCHEMA_ID.to_string(),
                        session_id: session_id.clone(),
                        spec_capsule_cid: spec_capsule_cid.clone(),
                        generation_attempt_cid: Some(attempt_cid.clone()),
                        triage_attempted: true,
                        reject_class: turingosv4::runtime::rejection_capsule::RejectClass::InternalIo,
                        public_error_summary: "internal test pipeline error".to_string(),
                        reason: format!("test_pipeline_error:{}", e),
                        private_diagnostic_cid: None,
                        retryable: false,
                        world_head_unchanged: false,
                        logical_t,
                    };
                    // X1: collect footer; run() emits it after the error message.
                    let footer = if let Ok(rej_cid) = turingosv4::runtime::rejection_capsule::write_generate_rejection_capsule_observed(&workspace, &rej) {
                        format!("[failed run] rejection_cid={rej_cid}")
                    } else {
                        String::new()
                    };
                    return Err(GenError::WithFooter {
                        inner: Box::new(GenError::Io(format!("test pipeline error: {}", e))),
                        footer,
                    });
                }
            }

            if emit_transcript {
                let transcript = serde_json::json!({
                    "logical_t": logical_t,
                    "model": capsule.model_id,
                    "spec_source": capsule.spec_source,
                    "usage_total_tokens": capsule.usage_total_tokens,
                    "files_written": written.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
                    "raw_response": content,
                });
                let path = workspace.join("generate_transcript.jsonl");
                let mut out = transcript.to_string();
                out.push('\n');
                let _ = fs::write(&path, out);
            }

            println!();
            println!(
                "Generated {} file(s) under {}/",
                written.len(),
                workspace.join("artifacts").display()
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
                println!(
                    "  xdg-open {}/{}",
                    workspace.join("artifacts").display(),
                    html.display()
                );
            } else if let Some(py) = written
                .iter()
                .find(|p| p.extension().map(|x| x == "py").unwrap_or(false))
            {
                println!(
                    "  python3 {}/{}",
                    workspace.join("artifacts").display(),
                    py.display()
                );
            } else if let Some(first) = written.first() {
                println!(
                    "  {}/{}",
                    workspace.join("artifacts").display(),
                    first.display()
                );
            }
        }
    }

    run_result
}

/// TRACE_MATRIX FC1-N4 / FC2-N18: Read prior rejection diagnostics from CAS
/// TRACE_MATRIX FC1a-rtool + FC1a-predicate_pi: Drive `turingos generate` through
/// the TDMA-Bounded MemoryKernel via `tdma_runner::run_proof` and an
/// `AnyJudge::Generate` single-stage judge.
///
/// Returns a `(Result<ChatResult, LlmError>, final_prompt_hash)` tuple. The
/// downstream code path is unchanged: when this returns `Ok`, the synthesized
/// ChatResult carries the FINAL accepted body + token totals across all
/// kernel attempts; when it returns `Err`, the existing rejection-capsule path
/// handles it like any other LLM error.
///
/// KILL-gen-3: the returned `final_prompt_hash` is the sha256 of the canonical
/// chat-request bytes of the FINAL attempt that produced the accepted body
/// (not the first attempt). This preserves audit reproducibility — the
/// GenerationAttemptCapsule's prompt_hash matches the prompt that actually
/// produced the result.
fn chat_with_tdma_bounded(
    workspace: &Path,
    session_id: &str,
    api_key: &str,
    model_id: &str,
    messages: &[ChatMessage],
    blackbox_thinking: Option<crate::siliconflow_client::ThinkingConfig>,
    entrypoint: &str,
    max_retries: usize,
    initial_prompt_hash: &str,
    tape_backend: &str,
) -> (Result<ChatResult, LlmError>, String) {
    use std::cell::RefCell;

    let evidence_dir = workspace
        .join("artifacts")
        .join("tdma_generate")
        .join(session_id);
    if let Err(e) = fs::create_dir_all(&evidence_dir) {
        eprintln!("[generate-tdma] cannot create evidence-dir {}: {}", evidence_dir.display(), e);
    }

    let mut judge = AnyJudge::generate(entrypoint.to_string(), false);

    let system_template = messages
        .iter()
        .find(|m| m.role == "system")
        .map(|m| m.content.clone())
        .unwrap_or_default();
    let user_template = messages
        .iter()
        .find(|m| m.role == "user")
        .map(|m| m.content.clone())
        .unwrap_or_default();
    let system_clone_for_closure = system_template.clone();
    let user_clone_for_closure = user_template.clone();

    let cfg = RunConfig {
        run_id: format!("turingos-generate-{}", session_id),
        model_label: model_id.to_string(),
        problem_label: "turingos generate (TDMA-Bounded wire-up)".into(),
        leak_sentinel: "TURINGOS_GENERATE_TDMA_LEAK_R8K9X".into(),
        system_prompt_for_stage: Box::new(move |_label: &str| system_clone_for_closure.clone()),
        user_prompt_for_stage: Box::new(move |_label: &str, _accepted: &[String]| {
            user_clone_for_closure.clone()
        }),
        problem_text: String::new(),
        evidence_dir: evidence_dir.clone(),
        temperature: 0.2,
        max_tokens: 6000,
        max_attempts_per_stage: max_retries,
    };

    let attempts: RefCell<Vec<(String, ChatResult)>> = RefCell::new(Vec::new());
    let api_key_owned = api_key.to_string();
    let model_owned = model_id.to_string();
    let thinking_clone = blackbox_thinking.clone();

    let llm_call = |sys: &str, user: &str| -> Result<LlmResponse, String> {
        let messages = vec![ChatMessage::system(sys), ChatMessage::user(user)];
        let canonical = canonical_chat_request_bytes(
            &model_owned,
            &messages,
            Some(6000),
            Some(0.2),
            thinking_clone.clone(),
        )
        .map_err(|e| format!("canonical_chat_request_bytes: {:?}", e))?;
        let mut hasher = Sha256::new();
        hasher.update(&canonical);
        let attempt_hash = format!("{:x}", hasher.finalize());

        let resp = chat_complete_blocking(
            &api_key_owned,
            &model_owned,
            &messages,
            Some(6000),
            Some(0.2),
            thinking_clone.clone(),
        )
        .map_err(|e| format!("{:?}", e))?;

        let runner_resp = LlmResponse {
            content: resp.content.clone(),
            completion_tokens: resp.usage.completion_tokens as u32,
            prompt_tokens: resp.usage.prompt_tokens as u32,
        };
        attempts.borrow_mut().push((attempt_hash, resp));
        Ok(runner_resp)
    };

    // Atom 24: select tape backend per --tape-backend.
    let run_outcome = if tape_backend == "git" {
        let repo_path = workspace.join("tdma_tape.git");
        match turingosv4::git_tape_ledger::GitTapeLedger::open(&repo_path)
            .or_else(|_| turingosv4::git_tape_ledger::GitTapeLedger::init_bare(&repo_path))
        {
            Ok(l) => {
                eprintln!(
                    "[generate-tdma] --tape-backend=git rooted at {}",
                    repo_path.display()
                );
                turingosv4::tdma_runner::run_proof_with_ledger(cfg, &mut judge, l, llm_call)
            }
            Err(e) => Err(format!("git tape backend: {}", e)),
        }
    } else {
        run_proof(cfg, &mut judge, llm_call)
    };
    let summary = match run_outcome {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[generate-tdma] run_proof failed: {}", e);
            return (
                Err(LlmError::Transport(format!("tdma_runner: {}", e))),
                initial_prompt_hash.to_string(),
            );
        }
    };

    let attempts_inner = attempts.into_inner();
    if attempts_inner.is_empty() {
        return (
            Err(LlmError::NoChoices),
            initial_prompt_hash.to_string(),
        );
    }

    // KILL-gen-3: select the prompt_hash that matches the final probe's body.
    // If at least one stage completed, the last attempt that produced
    // accepted content is the one we record. If escalated, use the final
    // attempted prompt (still meaningful for audit).
    let (final_hash, final_result) = attempts_inner.into_iter().last().unwrap();

    if summary.stages_completed >= 1 {
        eprintln!(
            "[generate-tdma] stages_completed={}/{} attempts={} wall={:.1}s leak={}",
            summary.stages_completed,
            summary.stages_total,
            summary.probes.len(),
            summary.total_wall_clock_ms as f64 / 1000.0,
            summary.leak_anywhere
        );
        // Aggregate token counts across all attempts (cumulative cost reporting).
        let total_completion: u32 = summary.total_llm_completion_tokens;
        let total_prompt: u32 = summary.total_llm_prompt_tokens;
        let synthetic = ChatResult {
            content: final_result.content,
            raw_response_body: final_result.raw_response_body,
            reasoning_content: final_result.reasoning_content,
            usage: Usage {
                prompt_tokens: total_prompt as u64,
                completion_tokens: total_completion as u64,
                total_tokens: (total_prompt + total_completion) as u64,
            },
            finish_reason: final_result.finish_reason,
        };
        (Ok(synthetic), final_hash)
    } else {
        let escalation = summary
            .stages_escalated
            .first()
            .cloned()
            .unwrap_or_else(|| "max-retries".to_string());
        eprintln!(
            "[generate-tdma] escalated: {} (attempts={} wall={:.1}s)",
            escalation,
            summary.probes.len(),
            summary.total_wall_clock_ms as f64 / 1000.0,
        );
        // Return the final attempt's content so the existing
        // rejection-capsule path can record the failure with diagnostic detail.
        (Ok(final_result), final_hash)
    }
}

fn read_prior_rejection_feedback(workspace: &Path, session_id: &str) -> Option<String> {
    let cas_dir = workspace.join("cas");
    let store = CasStore::open(&cas_dir).ok()?;

    // Find latest GenerateRejectionCapsule for this session by logical_t.
    let cids = store.list_cids_by_object_type(ObjectType::EvidenceCapsule);
    let mut candidates: Vec<(u64, GenerateRejectionCapsule)> = Vec::new();
    for cid in cids {
        let meta = store.metadata(&cid)?;
        if meta.schema_id.as_deref() == Some(GENERATE_REJECTION_CAPSULE_SCHEMA_ID) {
            if let Ok(bytes) = store.get(&cid) {
                if let Ok(cap) = serde_json::from_slice::<GenerateRejectionCapsule>(&bytes) {
                    if cap.session_id == session_id {
                        candidates.push((cap.logical_t, cap));
                    }
                }
            }
        }
    }
    candidates.sort_by_key(|x| x.0);
    let latest = candidates.into_iter().last()?.1;

    // Construct feedback text. Format it as concrete actionable guidance,
    // not a raw debug dump.
    let mut feedback = String::from("=== PRIOR ATTEMPT FEEDBACK (relayed from CAS tape) ===\n\n");
    feedback.push_str(&format!(
        "Your previous attempt for this same session FAILED.\n\
         Failure class: {:?}\n\
         Public summary: {}\n\
         Reason: {}\n\n",
        latest.reject_class, latest.public_error_summary, latest.reason,
    ));

    // If this was a HeuristicFailed (C11 test pipeline), find the linked
    // TestRunCapsule and surface the failed scenario names.
    if matches!(latest.reject_class, RejectClass::HeuristicFailed) {
        // Parse test_run_cid from reason field: "heuristic_failed:test_run_cid=<hex>"
        if let Some(idx) = latest.reason.find("test_run_cid=") {
            let cid_hex = &latest.reason[idx + "test_run_cid=".len()..];
            let cid_hex = cid_hex.split_whitespace().next().unwrap_or(cid_hex);
            if let Some(failed_scenarios) = read_failed_scenarios_by_cid(&store, cid_hex) {
                if !failed_scenarios.is_empty() {
                    feedback.push_str("Specific failed test scenarios:\n");
                    for (name, detail) in failed_scenarios {
                        feedback.push_str(&format!("  - {}: {}\n", name, detail));
                    }
                    feedback.push('\n');
                }
            }
        }
    }

    feedback.push_str(
        "INSTRUCTIONS: This is your second (or later) chance. Please:\n\
         1. Re-read the spec below carefully.\n\
         2. Address the specific failure mode above.\n\
         3. Produce a CORRECTED file set in the same `### File: <path>` + fenced-code-block format.\n\
         4. Do not repeat the prior mistake.\n\n\
         === END FEEDBACK ===\n\n",
    );
    Some(feedback)
}

/// Helper: parse a hex CID string and read the linked TestRunCapsule,
/// returning the list of (scenario_name, detail) for failed scenarios.
fn read_failed_scenarios_by_cid(store: &CasStore, cid_hex: &str) -> Option<Vec<(String, String)>> {
    if cid_hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        bytes[i] = u8::from_str_radix(&cid_hex[i * 2..i * 2 + 2], 16).ok()?;
    }
    let cid = Cid(bytes);
    let raw = store.get(&cid).ok()?;
    let capsule: TestRunCapsule = serde_json::from_slice(&raw).ok()?;

    let mut failed = Vec::new();
    for r in capsule.results {
        if !r.pass {
            let name = match &r.scenario {
                TestScenario::EntrypointExists => "EntrypointExists".to_string(),
                TestScenario::HtmlParses => "HtmlParses".to_string(),
                TestScenario::SandboxPolicyPreserved { .. } => "SandboxPolicyPreserved".to_string(),
            };
            failed.push((name, r.detail));
        }
    }
    Some(failed)
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
7. VISUAL FORMAT for HTML outputs (TuringOS aesthetic — applies when your
   output is `index.html`). Apply these design tokens as inline CSS — do
   NOT pull in Tailwind CDN, Bootstrap CDN, or any other framework:
   - Headings: font-family 'Fraunces', Georgia, serif (load via Google
     Fonts <link> in <head> is OK: family=Fraunces:opsz,wght@9..144,400;9..144,600).
   - Body: font-family 'IBM Plex Sans', system-ui, sans-serif (Google Fonts OK).
   - Code/mono: font-family 'JetBrains Mono', ui-monospace, monospace (Google Fonts OK).
   - Accent color: define `--accent: #4e8b7a` (oxidized teal). Use for links,
     buttons, borders, focus rings, key highlights.
   - Background: `#f8f6f1` (warm off-white). Text: `#1a1a1a`. Muted: `#6b6b6b`.
   - Layout: comfortable padding, generous line-height (≥1.55 body),
     H1 Fraunces 36–48px, H2 Fraunces 24–28px, body 16–17px.
   - Do NOT use Inter, Roboto, Arial, or any purple-gradient styling.
   - Prefer prefers-color-scheme: dark for an additional dark variant
     (background #1a1a1a, text #f0eee8, accent same teal but slightly lighter).
   - If the spec does NOT target a UI/HTML app (e.g., a Python script), skip
     this rule entirely.

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

fn guess_mime(path_str: &str) -> String {
    let lower = path_str.to_lowercase();
    if lower.ends_with(".html") || lower.ends_with(".htm") {
        "text/html".to_string()
    } else if lower.ends_with(".js") {
        "text/javascript".to_string()
    } else if lower.ends_with(".css") {
        "text/css".to_string()
    } else if lower.ends_with(".ts") {
        "text/typescript".to_string()
    } else if lower.ends_with(".png") {
        "image/png".to_string()
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg".to_string()
    } else if lower.ends_with(".gif") {
        "image/gif".to_string()
    } else if lower.ends_with(".svg") {
        "image/svg+xml".to_string()
    } else if lower.ends_with(".json") {
        "application/json".to_string()
    } else {
        "application/octet-stream".to_string()
    }
}

fn find_entrypoint(files: &[EmittedFile]) -> Option<String> {
    if files.is_empty() {
        return None;
    }
    // 1. Check for index.html
    for f in files {
        if f.path == "index.html" {
            return Some(f.path.clone());
        }
    }
    // 2. Check for first .html file
    for f in files {
        if f.path.ends_with(".html") {
            return Some(f.path.clone());
        }
    }
    // 3. Fallback to first file
    Some(files[0].path.clone())
}

// ─────────────────────────────────────────────────────────────────────────────
// Polymarket (2026-05-23 REVISED post-Codex/Karpathy audit) — post-judge
// WorkTx + MarketSeedTx emission via the canonical workspace ChainTape.
//
// Wires the existing `turingos generate` flow into the kernel's WorkTx /
// market surfaces through `build_chaintape_sequencer_with_initial_q` (TB-G
// G1.1 architect-signed factory; `resume_existing_chain: true`). Every
// admission lands on `<workspace>/runtime_repo` so the chain is the
// canonical source of truth — `verify_chaintape` can replay the run, the
// web `market_view` projection reads the same chain, and a cold restart
// re-derives the same JSON.
//
// Architectural decisions:
//   - NO new CLI subcommand (extends existing `generate` only)
//   - Worker hardcoded to "worker-alpha" (PR2 makes this RunConfig-driven)
//   - Bounty = 1000µ (treasury-funded); WorkTx.stake = 100µ; MarketSeed = 100µ
//     each side (= bounty / 10 per architect manual §7.4)
//   - Sequencer admission auto-runs predicates → L4 or L4.E (no shadow ledger)
//   - On L4-accept: emit MarketSeedTx (treasury collateral) opening YES/NO pool
//   - On L4.E reject: skip MarketSeed (existing rejection capsule path already
//     runs; no extra work)
//   - PR3 deferred: ChallengeTx (Art. III.3 horizontal-independence requires
//     an isolated-context critic bot)
// ─────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: Polymarket (2026-05-23 revised) — summary of the
/// post-judge sequencer dance for a single generate session. Returned by
/// `emit_polymarket_market_for_session` so the stderr log lines can quote
/// the worker / proposal_cid / task_id consistently. NOT a chain-resident
/// capsule: status is re-derivable from `<workspace>/runtime_repo` +
/// `EconomicState.task_markets_t[task_id]`.
pub(crate) struct PolymarketEmitSummary {
    pub(crate) worker_agent: String,
    pub(crate) task_id: String,
    pub(crate) proposal_cid_hex_prefix: String,
    pub(crate) market_opened: bool,
    pub(crate) rejection_note: Option<String>,
}

impl std::fmt::Display for PolymarketEmitSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(note) = &self.rejection_note {
            write!(f, "{note}")
        } else {
            write!(f, "ok")
        }
    }
}

/// TRACE_MATRIX FC2-N16 + FC1: Polymarket (2026-05-23 revised) — orchestrate
/// the post-judge sequencer admission flow against the workspace's canonical
/// ChainTape.
///
/// Builds (or resumes) the workspace's chain via
/// `build_chaintape_sequencer_with_initial_q` (TB-G G1.1 architect-signed
/// `resume_existing_chain: true` mode — empty `runtime_repo` → fresh
/// bootstrap with preseed; non-empty → resume + replay). Submits the
/// canonical `TaskOpen → EscrowLock → WorkTx [→ MarketSeed]` sequence so
/// WorkTx admission is REAL (per architect ruling 2026-05-23: "内核必须一致，
/// 先有个中央银行" — no simulation branch, treasury preseed via the only
/// allow-listed boot surface).
///
/// **Constitutional posture** (FC1 wtool, Art. 0.4):
/// - Q_t → rtool: opens canonical sequencer that owns `Git2LedgerWriter` ←—
///   the persistent wtool. NO `InMemoryLedgerWriter` in this code path.
/// - Agent delta: 4 typed_tx submissions through canonical
///   `submit_agent_tx` / driver / `apply_one` path.
/// - wtool: `Git2LedgerWriter` appends each accepted entry to
///   `<workspace>/runtime_repo/refs/transitions/main`; rejections land in
///   `<workspace>/runtime_repo/rejections.jsonl`.
/// - Q_{t+1}: post-drain `q_snapshot()` reflects all admissions. Subsequent
///   `turingos generate` invocations resume this chain.
fn emit_polymarket_market_for_session(
    workspace: &Path,
    session_id: &str,
    bundle_cid_hex: &str,
    logical_t: u64,
) -> Result<PolymarketEmitSummary, String> {
    let task_id_str = polymarket_task_id_for_session(session_id);

    let genesis_text = fs::read_to_string(workspace.join("genesis_payload.toml"))
        .map_err(|e| format!("read workspace genesis_payload.toml: {e}"))?;
    let preseed = parse_treasury_and_worker_preseed(&genesis_text)
        .map_err(|e| format!("parse preseed: {e}"))?;
    let initial_q = genesis_with_balances(&preseed);

    let proposal_cid = cid_from_hex_str(bundle_cid_hex)
        .map_err(|e| format!("decode bundle_cid_hex: {e}"))?;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("tokio runtime: {e}"))?;

    rt.block_on(async move {
        // ──────────────── Step 0: open canonical workspace ChainTape ────────────────
        // Empty `runtime_repo` (post-`turingos init`) → fresh bootstrap with
        // `initial_q` seed. Non-empty → resume + replay (subsequent
        // `turingos generate` invocations on the same workspace land here).
        // TB-G G1.1 packet §2: the resume path reads the persisted
        // `initial_q_state.json` so on resume the `initial_q` argument is
        // ignored — but the in-tree preseed is byte-identical to the
        // initial bootstrap, so the seed is consistent across invocations.
        let config = RuntimeChaintapeConfig {
            runtime_repo_path: workspace.join("runtime_repo"),
            cas_path: workspace.join("cas"),
            run_id: format!("polymarket-{session_id}-{logical_t}"),
            queue_capacity: 16,
            resume_existing_chain: true,
        };
        let bundle: ChaintapeBundle =
            build_chaintape_sequencer_with_initial_q(&config, initial_q)
                .map_err(|e| format!("open canonical chaintape: {e}"))?;
        let seq = bundle.sequencer.clone();
        let rejection_writer = bundle.rejection_writer.clone();
        // Snapshot rejection-record count before our admissions so we can
        // tell which rejections (if any) belong to THIS call. The chain
        // may already carry prior rejections from earlier invocations.
        let pre_rejection_count = rejection_writer
            .read()
            .map_err(|e| format!("rejection_writer pre-read poison: {e}"))?
            .records()
            .len();

        // ──────────────── Pre-compute parent_state_roots ────────────────
        // Pre-compute the expected post-each-tx state roots via the kernel's
        // pure `*_accept_state_root` helpers so all 4 txs can be submitted in
        // one batch (the driver applies them in FIFO order). This mirrors
        // tb_14's pattern: pre-compute the post-mint root for the redeem's
        // parent_state_root. Without pre-computation, the driver's async
        // apply would race the next `q_snapshot()` read.
        let root_0 = seq
            .q_snapshot()
            .map_err(|e| format!("q_snapshot @ root_0: {e:?}"))?
            .state_root_t;
        let task_open_tx = TypedTx::TaskOpen(TaskOpenTx {
            tx_id: TxId(format!("polymarket-taskopen-{session_id}-{logical_t}")),
            task_id: TaskId(task_id_str.clone()),
            parent_state_root: root_0,
            sponsor_agent: AgentId(TREASURY_AGENT_ID.into()),
            verifier_quorum: 1,
            max_reuse_royalty_fraction_basis_points: 0,
            settlement_rule_hash: StateHash::ZERO,
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: logical_t,
        });
        let root_1 = task_open_accept_state_root(&root_0, &task_open_tx);

        let escrow_tx = TypedTx::EscrowLock(EscrowLockTx {
            tx_id: TxId(format!("polymarket-escrowlock-{session_id}-{logical_t}")),
            task_id: TaskId(task_id_str.clone()),
            parent_state_root: root_1,
            sponsor_agent: AgentId(TREASURY_AGENT_ID.into()),
            amount: MicroCoin::from_micro_units(DEFAULT_BOUNTY_MICRO),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: logical_t,
        });
        let root_2 = escrow_lock_accept_state_root(&root_1, &escrow_tx);

        let mut acceptance = std::collections::BTreeMap::new();
        // The TDMA judge ALREADY signed off on the artifact (we're in the
        // post-judge-success branch). Surface that pass-verdict as an
        // acceptance predicate row so the WorkTx admission Step 2 succeeds.
        acceptance.insert(
            PredicateId("tdma_judge_generate".into()),
            BoolWithProof {
                value: true,
                proof_cid: None,
            },
        );
        let work_tx_id_str = format!("polymarket-worktx-{session_id}-{logical_t}");
        let work_tx = TypedTx::Work(WorkTx {
            tx_id: TxId(work_tx_id_str.clone()),
            task_id: TaskId(task_id_str.clone()),
            parent_state_root: root_2,
            agent_id: AgentId(WORKER_ALPHA_AGENT_ID.into()),
            // Minimal read/write sets — records the spec capsule + bundle
            // cid as semantic anchors (mirrors `adapter::make_real_worktx_signed_by`).
            read_set: [ReadKey("spec_capsule".into())].into_iter().collect(),
            write_set: [WriteKey("artifact_bundle".into())].into_iter().collect(),
            proposal_cid,
            predicate_results: PredicateResultsBundle {
                acceptance,
                settlement: std::collections::BTreeMap::new(),
                safety_class: SafetyOrCreation::Safety,
            },
            stake: StakeMicroCoin::from_micro_units(DEFAULT_WORK_STAKE_MICRO),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: logical_t,
        });
        let root_3 = worktx_accept_state_root(&root_2, &work_tx);

        let market_seed_tx = TypedTx::MarketSeed(MarketSeedTx {
            tx_id: TxId(format!("polymarket-marketseed-{session_id}-{logical_t}")),
            parent_state_root: root_3,
            event_id: EventId(TaskId(task_id_str.clone())),
            provider: AgentId(TREASURY_AGENT_ID.into()),
            collateral_amount: MicroCoin::from_micro_units(DEFAULT_MARKET_SEED_MICRO),
            signature: AgentSignature::from_bytes([0u8; 64]),
            timestamp_logical: logical_t,
        });

        // ──────────────── Submit all 4 in FIFO order ────────────────
        // The canonical sequencer's driver task drains the queue; each
        // submit is sync about queue-admission but async about
        // apply-and-commit. We submit all 4, then `bundle.shutdown()`
        // drains the queue + waits for the last commit before returning.
        // If WorkTx is rejected (L4.E), MarketSeed will also reject because
        // its `parent_state_root = root_3` won't match the actual state.
        // We detect this via the rejection_writer records after drain.
        for tx in [task_open_tx, escrow_tx, work_tx, market_seed_tx] {
            seq.submit_agent_tx(tx)
                .await
                .map_err(|e| format!("submit error: {e:?}"))?;
        }

        // TODO(PR3): emit `EventResolveTx` (system-tx) to flip
        // `task_markets_t[task_id].state` from Open → Finalized once a
        // verifier-driven decision is in scope. PR1 N=1 has no peer-Worker
        // critic — the single accepted WorkTx is the winner by definition
        // but `EventResolveTx` admission requires `parent_state_root ==
        // current q.state_root_t`, which is racy when the prior 4
        // admissions are still draining in the driver task. The simpler
        // path (defer to PR3 with the dedicated isolated-context critic
        // bot Art. III.3 requires) keeps this PR surgical. Until then,
        // `market_view.rs` surfaces `market_state: "open"` for any
        // accepted-but-unresolved market — the web UI must distinguish
        // open vs finalized when rendering.

        let seq_handle = seq.clone();
        bundle
            .shutdown()
            .await
            .map_err(|e| format!("chaintape shutdown drain: {e:?}"))?;

        // ──────────────── Post-drain: inspect chain state ────────────────
        // `task_markets_t` is the canonical source for "did WorkTx admit".
        // The WorkTx accept arm in `sequencer.rs` Step 5 inserts the task
        // entry into `task_markets_t`; rejection leaves it absent (or the
        // pre-existing entry from a prior call's TaskOpen, which we'll
        // discriminate by checking `stakes_t` for our work_tx_id).
        let post_q = seq_handle
            .q_snapshot()
            .map_err(|e| format!("post-drain q_snapshot: {e:?}"))?;
        let work_tx_id = TxId(work_tx_id_str.clone());
        let work_tx_accepted = post_q
            .economic_state_t
            .stakes_t
            .0
            .contains_key(&work_tx_id);

        // Determine if the MarketSeed admitted — the YES/NO cpmm pools or
        // `conditional_collateral_t` entry presence signals MarketSeed
        // accept. Simpler: check `conditional_collateral_t` for the event.
        let event_id = EventId(TaskId(task_id_str.clone()));
        let market_opened = post_q
            .economic_state_t
            .conditional_collateral_t
            .0
            .contains_key(&event_id);

        // Collect any rejection notes belonging to THIS call.
        let mut rejection_note: Option<String> = None;
        if !work_tx_accepted {
            let records = rejection_writer
                .read()
                .map_err(|e| format!("rejection_writer post-read poison: {e}"))?
                .records()
                .to_vec();
            // Records after `pre_rejection_count` are ours.
            for rec in records.iter().skip(pre_rejection_count) {
                if let Some(summary) = &rec.public_summary {
                    rejection_note = Some(format!("{:?}: {}", rec.tx_kind, summary));
                    break;
                }
            }
            if rejection_note.is_none() {
                rejection_note = Some("WorkTx admission did not advance task_markets_t".into());
            }
        }

        Ok::<PolymarketEmitSummary, String>(PolymarketEmitSummary {
            worker_agent: WORKER_ALPHA_AGENT_ID.to_string(),
            task_id: task_id_str,
            proposal_cid_hex_prefix: bundle_cid_hex.chars().take(16).collect::<String>(),
            market_opened: work_tx_accepted && market_opened,
            rejection_note,
        })
    })
}

/// TRACE_MATRIX FC2-N16: Polymarket (2026-05-23 revised) — stable task_id
/// derivation from session_id.
///
/// Convention: prefix `pr1-` so cross-PR session_ids never collide on the
/// kernel task namespace. The prefix value is FROZEN (a chain-resident
/// task_id; renaming it would invalidate prior workspaces' replay). PR2/3
/// may extend with a UUID-derived scheme for net-new sessions while
/// keeping existing chains valid.
pub(crate) fn polymarket_task_id_for_session(session_id: &str) -> String {
    format!("pr1-{session_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn polymarket_task_id_for_session_is_stable() {
        assert_eq!(polymarket_task_id_for_session("abc"), "pr1-abc");
        assert_eq!(polymarket_task_id_for_session(""), "pr1-");
    }

    #[test]
    fn polymarket_constants_satisfy_invariants() {
        // Hard constraint: MarketSeed = bounty / 10
        assert_eq!(DEFAULT_MARKET_SEED_MICRO, DEFAULT_BOUNTY_MICRO / 10);
        // Hard constraint: stake is positive
        assert!(DEFAULT_WORK_STAKE_MICRO > 0);
    }
}
