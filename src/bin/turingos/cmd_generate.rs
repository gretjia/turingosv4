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
use turingosv4::runtime::spec_capsule;
use sha2::{Digest, Sha256};
use turingosv4::runtime::generation_attempt::{
    GenerationAttemptCapsule, AttemptOutcome, write_generation_attempt_capsule,
    GENERATION_ATTEMPT_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::rejection_capsule::{
    GenerateRejectionCapsule, RejectClass, write_generate_rejection_capsule,
    GENERATE_REJECTION_CAPSULE_SCHEMA_ID,
};
use turingosv4::runtime::artifact_bundle::{
    ArtifactFileRole, ArtifactFileEntry, ArtifactBundleManifest,
    write_artifact_bundle, latest_artifact_bundle_cid_for_session,
    ARTIFACT_BUNDLE_SCHEMA_ID
};
use turingosv4::runtime::test_run::{run_and_write_test_pipeline, format_test_run_summary};
use turingosv4::bottom_white::cas::schema::ObjectType;
use turingosv4::bottom_white::cas::store::CasStore;

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
    TooManyFiles { found: usize, max: usize },
    /// X1: carries CID footer lines to be printed AFTER the error message.
    WithFooter { inner: Box<GenError>, footer: String },
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
        let latest_cid = spec_capsule::latest_spec_capsule_cid(&workspace).ok().flatten();
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
    let api_key_env = cmd_llm::read_blackbox_api_key_env(&workspace)
        .map_err(|e| GenError::Io(e.to_string()))?;
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

    let messages = vec![
        ChatMessage::system(blackbox_system_prompt()),
        ChatMessage::user(format!(
            "Below is the spec. Generate the working code per the rules.\n\nspec source: {source}\n\n{spec_md}"
        )),
    ];

    let canonical_prompt = format!(
        "system: {}\nuser: Below is the spec. Generate the working code per the rules.\n\nspec source: {}\n\n{}",
        blackbox_system_prompt(),
        source,
        spec_md
    );
    let mut hasher = Sha256::new();
    hasher.update(canonical_prompt.as_bytes());
    let prompt_hash = format!("{:x}", hasher.finalize());

    let session_id = if workspace.parent().map(|p| p.file_name().map(|n| n == "sessions").unwrap_or(false)).unwrap_or(false) {
        workspace.file_name().and_then(|n| n.to_str()).unwrap_or("default").to_string()
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
                        if let Ok(capsule) = serde_json::from_slice::<GenerationAttemptCapsule>(&bytes) {
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

    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let blackbox_thinking = cmd_llm::read_blackbox_thinking(&workspace);
    eprintln!("[generate] calling Blackbox LLM ({model_id})...");
    let llm_res = chat_complete_blocking(&api_key, &model_id, &messages, Some(6000), Some(0.2), blackbox_thinking);

    let (outcome, raw_output_cid, usage_total_tokens, parsed_file_count, files_to_write, run_result): (
        AttemptOutcome,
        Option<String>,
        Option<u32>,
        usize,
        Option<(Vec<PathBuf>, String)>,
        Result<(), GenError>
    ) = match llm_res {
        Err(e) => {
            (AttemptOutcome::LlmApiError, None, None, 0, None, Err(GenError::Llm(e)))
        }
        Ok(result) => {
            let raw_cid = match CasStore::open(&cas_dir) {
                Ok(mut store) => {
                    match store.put(
                        result.content.as_bytes(),
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

                (AttemptOutcome::NoFilesParsed, raw_cid, Some(result.usage.total_tokens as u32), 0, None, Err(GenError::NoFilesParsed))
            } else if files.len() > max_files {
                (AttemptOutcome::ParseFailed, raw_cid, Some(result.usage.total_tokens as u32), files.len(), None, Err(GenError::TooManyFiles {
                    found: files.len(),
                    max: max_files,
                }))
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
                                        write_err = Some(GenError::Io(format!("create dir {}: {e}", parent.display())));
                                        break;
                                    }
                                }
                                if let Err(e) = fs::write(&full, &f.content) {
                                    write_err = Some(GenError::Io(format!("write {}: {e}", full.display())));
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
                    (AttemptOutcome::InternalIo, raw_cid, Some(result.usage.total_tokens as u32), files.len(), None, Err(err))
                } else {
                    (AttemptOutcome::Success, raw_cid, Some(result.usage.total_tokens as u32), files.len(), Some((written, result.content)), Ok(()))
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
            let public_summary = "CAS write error during generation attempt capsule recording".to_string();
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
                world_head_unchanged: true,
                logical_t,
            };
            let footer = if let Ok(rej_cid) = write_generate_rejection_capsule(&workspace, &rej) {
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
        }.to_string();

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
            world_head_unchanged: true,
            logical_t,
        };
        // X1: Collect CID footer; run() will emit it AFTER the error message.
        let mut footer_parts = vec![
            format!("[failed run] generation_attempt_cid={}", attempt_cid),
        ];
        if let Ok(rej_cid) = write_generate_rejection_capsule(&workspace, &rej) {
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
                } else if f.path.ends_with(".html") || f.path.ends_with(".js") || f.path.ends_with(".css") || f.path.ends_with(".ts") {
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

            let previous_bundle_cid = latest_artifact_bundle_cid_for_session(&workspace, &session_id)
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
                            world_head_unchanged: true,
                            logical_t,
                        };
                        // X1: collect footer; run() emits it after the error message.
                        let footer = if let Ok(rej_cid) = turingosv4::runtime::rejection_capsule::write_generate_rejection_capsule(&workspace, &rej) {
                            format!("[failed run] rejection_cid={rej_cid}")
                        } else {
                            String::new()
                        };
                        return Err(GenError::WithFooter {
                            inner: Box::new(GenError::Io(
                                "generated artifacts failed spec-derived tests".to_string()
                            )),
                            footer,
                        });
                    }
                    // overall_pass=true — proceed to success output below.
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
                        world_head_unchanged: true,
                        logical_t,
                    };
                    // X1: collect footer; run() emits it after the error message.
                    let footer = if let Ok(rej_cid) = turingosv4::runtime::rejection_capsule::write_generate_rejection_capsule(&workspace, &rej) {
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
                println!("  xdg-open {}/{}", workspace.join("artifacts").display(), html.display());
            } else if let Some(py) = written
                .iter()
                .find(|p| p.extension().map(|x| x == "py").unwrap_or(false))
            {
                println!("  python3 {}/{}", workspace.join("artifacts").display(), py.display());
            } else if let Some(first) = written.first() {
                println!("  {}/{}", workspace.join("artifacts").display(), first.display());
            }
        }
    }

    run_result
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
