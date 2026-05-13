//! Self-hosting development harness sidecar.
//!
//! This module is intentionally a thin evidence recorder for TuringOS
//! development work. It does not create a second canonical tape and it does not
//! bypass the constitution risk model. Its job is to make Codex/Claude
//! implementation sessions reconstructable: task contract, diff, commands,
//! review verdict, and close summary.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA_VERSION: &str = "turingos.dev_harness.v1";
const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// TRACE_MATRIX FC3-N33 + FC3-N43: fail-closed error surface for the
/// self-hosting development evidence sidecar.
#[derive(Debug)]
pub enum DevHarnessError {
    Io(std::io::Error),
    Json(serde_json::Error),
    InvalidInput(String),
    InvalidVerdict(String),
    HashChainBroken { reason: String },
    AuditRequired,
    AcceptanceFailed,
}

impl Display for DevHarnessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::Json(e) => write!(f, "json error: {e}"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::InvalidVerdict(v) => write!(
                f,
                "invalid audit verdict: {v}. Use exactly PROCEED, CHALLENGE, or VETO"
            ),
            Self::HashChainBroken { reason } => write!(f, "hash chain broken: {reason}"),
            Self::AuditRequired => write!(
                f,
                "clean-context Codex audit is required before this run can close"
            ),
            Self::AcceptanceFailed => write!(
                f,
                "acceptance evidence failed or is missing; record a passing command first"
            ),
        }
    }
}

impl Error for DevHarnessError {}

impl From<std::io::Error> for DevHarnessError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for DevHarnessError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: task contract used to open a
/// self-hosting development run.
#[derive(Clone, Debug)]
pub struct DevOpenRequest {
    pub evidence_root: PathBuf,
    pub title: String,
    pub module: String,
    pub molecule_or_atom: String,
    pub requested_risk_class: u8,
    pub fc_nodes: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub acceptance_commands: Vec<String>,
    pub human_intent: Option<String>,
    pub ratification: Option<String>,
    pub git_head: Option<String>,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: stable handle naming the opened dev run
/// and its evidence directory.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevRunHandle {
    pub run_id: String,
    pub run_dir: PathBuf,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: content-addressed pointer to an artifact
/// produced by the dev harness sidecar.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: module/molecule/atom contract persisted
/// at dev-run open time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevTaskManifest {
    pub schema_version: String,
    pub run_id: String,
    pub title: String,
    pub module: String,
    pub molecule_or_atom: String,
    pub requested_risk_class: u8,
    pub risk_class: u8,
    pub fc_nodes: Vec<String>,
    pub allowed_paths: Vec<String>,
    pub restricted_surface_hits: Vec<String>,
    pub acceptance_commands: Vec<String>,
    pub audit_required: bool,
    pub human_intent: Option<String>,
    pub ratification: Option<String>,
    pub git_head: Option<String>,
    pub created_at_unix_ms: u128,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: FC-node witness manifest for the dev-run
/// evidence bundle.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FCWitnessManifest {
    pub schema_version: String,
    pub run_id: String,
    pub fc_nodes: Vec<String>,
    pub expected_witness: String,
    pub evidence_artifacts: Vec<ArtifactRef>,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: command execution evidence with hashed
/// stdout/stderr artifacts.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandEvidence {
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub exit_code: i32,
    pub started_at_unix_ms: u128,
    pub ended_at_unix_ms: u128,
    pub stdout: ArtifactRef,
    pub stderr: ArtifactRef,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: clean-context reviewer verdict recorded
/// against a dev run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevAuditVerdict {
    pub schema_version: String,
    pub reviewer: String,
    pub verdict: String,
    pub findings_summary: String,
    pub source_file: ArtifactRef,
    pub payload_manifest_hash: String,
    pub recorded_at_unix_ms: u128,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: close-time summary proving command, diff,
/// audit, and hash-chain status.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevRunSummary {
    pub schema_version: String,
    pub run_id: String,
    pub close_status: String,
    pub manifest_sha256: String,
    pub event_chain_head_hash: String,
    pub diff_sha256: Option<String>,
    pub command_results: Vec<CommandEvidence>,
    pub acceptance_passed: bool,
    pub audit_required: bool,
    pub audit_verdict: Option<String>,
    pub effective_risk_class: u8,
    pub restricted_surface_hits: Vec<String>,
    pub closed_at_unix_ms: u128,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: validation view over an open or closed
/// dev-run evidence directory.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevRunValidation {
    pub run_id: String,
    pub event_count: usize,
    pub event_chain_head_hash: String,
    pub effective_risk_class: u8,
    pub restricted_surface_hits: Vec<String>,
    pub audit_required: bool,
    pub audit_verdict: Option<String>,
    pub acceptance_passed: bool,
    pub command_results: Vec<CommandEvidence>,
    pub diff_artifact: Option<ArtifactRef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DevEvent {
    schema_version: String,
    event_index: usize,
    event_type: String,
    timestamp_unix_ms: u128,
    payload: Value,
    prev_hash: String,
    event_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EventHashChainSummary {
    schema_version: String,
    event_count: usize,
    head_hash: String,
    updated_at_unix_ms: u128,
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: open a dev-run evidence directory from
/// an explicit task contract.
pub fn open_run(req: DevOpenRequest) -> Result<DevRunHandle, DevHarnessError> {
    if req.title.trim().is_empty() {
        return Err(DevHarnessError::InvalidInput(
            "title is required. Retry with --title <human intent summary>.".to_string(),
        ));
    }
    if req.module.trim().is_empty() {
        return Err(DevHarnessError::InvalidInput(
            "module is required. Retry with --module <ModuleName>.".to_string(),
        ));
    }
    if req.fc_nodes.is_empty() {
        return Err(DevHarnessError::InvalidInput(
            "--fc is required. Map this change to FC nodes before opening the run.".to_string(),
        ));
    }
    if req.requested_risk_class > 4 {
        return Err(DevHarnessError::InvalidInput(
            "--risk must be an integer in 0..4.".to_string(),
        ));
    }

    fs::create_dir_all(&req.evidence_root)?;
    let run_id = format!("dev_{}_{}", now_unix_ms(), std::process::id());
    let run_dir = req.evidence_root.join(&run_id);
    fs::create_dir_all(run_dir.join("artifacts"))?;

    let restricted_surface_hits = restricted_hits_from_paths(&req.allowed_paths);
    let risk_class = req
        .requested_risk_class
        .max(risk_floor_for_hits(&restricted_surface_hits));
    let audit_required = risk_class >= 3;

    let manifest = DevTaskManifest {
        schema_version: SCHEMA_VERSION.to_string(),
        run_id: run_id.clone(),
        title: req.title,
        module: req.module,
        molecule_or_atom: req.molecule_or_atom,
        requested_risk_class: req.requested_risk_class,
        risk_class,
        fc_nodes: req.fc_nodes.clone(),
        allowed_paths: req.allowed_paths,
        restricted_surface_hits,
        acceptance_commands: req.acceptance_commands,
        audit_required,
        human_intent: req.human_intent,
        ratification: req.ratification,
        git_head: req.git_head,
        created_at_unix_ms: now_unix_ms(),
    };
    write_json_pretty(&run_dir.join("DevTaskManifest.json"), &manifest)?;

    let fc_witness = FCWitnessManifest {
        schema_version: SCHEMA_VERSION.to_string(),
        run_id: run_id.clone(),
        fc_nodes: req.fc_nodes,
        expected_witness: "Diff, command evidence, and audit verdict must cite these FC nodes."
            .to_string(),
        evidence_artifacts: Vec::new(),
    };
    write_json_pretty(&run_dir.join("FCWitnessManifest.json"), &fc_witness)?;

    File::create(run_dir.join("events.jsonl"))?;
    write_json_pretty(
        &run_dir.join("events_hash_chain.json"),
        &EventHashChainSummary {
            schema_version: SCHEMA_VERSION.to_string(),
            event_count: 0,
            head_hash: ZERO_HASH.to_string(),
            updated_at_unix_ms: now_unix_ms(),
        },
    )?;

    append_event(
        &run_dir,
        "open",
        json!({
            "manifest": artifact_ref_for(&run_dir, &run_dir.join("DevTaskManifest.json"))?,
            "fc_witness": artifact_ref_for(&run_dir, &run_dir.join("FCWitnessManifest.json"))?,
        }),
    )?;

    Ok(DevRunHandle { run_id, run_dir })
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: append a hashed diff artifact to the
/// dev-run evidence chain.
pub fn record_diff_text(run_dir: &Path, diff_text: &str) -> Result<ArtifactRef, DevHarnessError> {
    ensure_run_dir(run_dir)?;
    let diff_path = run_dir.join("artifacts").join("diff.patch");
    fs::write(&diff_path, diff_text)?;
    let artifact = artifact_ref_for(run_dir, &diff_path)?;

    let mut manifest = load_manifest(run_dir)?;
    let mut hits = manifest.restricted_surface_hits.clone();
    hits.extend(restricted_hits_from_diff(diff_text));
    hits = sorted_unique(hits);
    manifest.risk_class = manifest.risk_class.max(risk_floor_for_hits(&hits));
    manifest.audit_required = manifest.audit_required || manifest.risk_class >= 3;
    manifest.restricted_surface_hits = hits.clone();
    write_json_pretty(&run_dir.join("DevTaskManifest.json"), &manifest)?;

    let mut fc = load_fc_witness(run_dir)?;
    fc.evidence_artifacts.push(artifact.clone());
    write_json_pretty(&run_dir.join("FCWitnessManifest.json"), &fc)?;

    append_event(
        run_dir,
        "diff_recorded",
        json!({
            "artifact": artifact,
            "restricted_surface_hits": hits,
            "effective_risk_class": manifest.risk_class,
        }),
    )?;
    Ok(artifact)
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: execute a command and preserve stdout,
/// stderr, exit code, and timing as evidence.
pub fn record_command(
    run_dir: &Path,
    command: &[&str],
) -> Result<CommandEvidence, DevHarnessError> {
    ensure_run_dir(run_dir)?;
    if command.is_empty() {
        return Err(DevHarnessError::InvalidInput(
            "record-command requires a command after --, e.g. turingos_dev record-command --run <id> -- cargo check".to_string(),
        ));
    }

    let event_index = next_event_index(run_dir)?;
    let started_at_unix_ms = now_unix_ms();
    let output = Command::new(command[0]).args(&command[1..]).output()?;
    let ended_at_unix_ms = now_unix_ms();
    let artifacts_dir = run_dir.join("artifacts");
    let stdout_path = artifacts_dir.join(format!("command_{event_index:04}_stdout.txt"));
    let stderr_path = artifacts_dir.join(format!("command_{event_index:04}_stderr.txt"));
    fs::write(&stdout_path, &output.stdout)?;
    fs::write(&stderr_path, &output.stderr)?;

    let evidence = CommandEvidence {
        command: command.iter().map(|s| (*s).to_string()).collect(),
        cwd: std::env::current_dir()?,
        exit_code: output.status.code().unwrap_or(-1),
        started_at_unix_ms,
        ended_at_unix_ms,
        stdout: artifact_ref_for(run_dir, &stdout_path)?,
        stderr: artifact_ref_for(run_dir, &stderr_path)?,
    };

    append_event(
        run_dir,
        "command_recorded",
        json!({
            "command": evidence,
        }),
    )?;
    Ok(evidence)
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: record the independent reviewer verdict
/// and hash-reference the review artifact.
pub fn record_audit(
    run_dir: &Path,
    reviewer: &str,
    verdict: &str,
    source_file: &Path,
    findings_summary: &str,
) -> Result<DevAuditVerdict, DevHarnessError> {
    ensure_run_dir(run_dir)?;
    if !matches!(verdict, "PROCEED" | "CHALLENGE" | "VETO") {
        return Err(DevHarnessError::InvalidVerdict(verdict.to_string()));
    }

    let source_ref = artifact_ref_for(run_dir, source_file)?;
    let payload_manifest_hash = sha256_file(&run_dir.join("DevTaskManifest.json"))?;
    let audit = DevAuditVerdict {
        schema_version: SCHEMA_VERSION.to_string(),
        reviewer: reviewer.to_string(),
        verdict: verdict.to_string(),
        findings_summary: findings_summary.to_string(),
        source_file: source_ref,
        payload_manifest_hash,
        recorded_at_unix_ms: now_unix_ms(),
    };
    write_json_pretty(&run_dir.join("DevAuditVerdict.json"), &audit)?;
    append_event(
        run_dir,
        "audit_recorded",
        json!({
            "audit": audit,
        }),
    )?;
    Ok(audit)
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: validate hash-chain integrity, acceptance
/// status, risk floor, and audit state for a dev run.
pub fn validate_run(run_dir: &Path) -> Result<DevRunValidation, DevHarnessError> {
    ensure_run_dir(run_dir)?;
    let chain = validate_hash_chain(run_dir)?;
    let manifest = load_manifest(run_dir)?;
    let diff_artifact = match run_dir.join("artifacts").join("diff.patch").exists() {
        true => Some(artifact_ref_for(
            run_dir,
            &run_dir.join("artifacts").join("diff.patch"),
        )?),
        false => None,
    };

    let mut hits = manifest.restricted_surface_hits.clone();
    if let Some(diff) = diff_artifact.as_ref() {
        let text = fs::read_to_string(run_dir.join(&diff.path))?;
        hits.extend(restricted_hits_from_diff(&text));
    }
    hits = sorted_unique(hits);
    let effective_risk_class = manifest.risk_class.max(risk_floor_for_hits(&hits));
    let audit_required = manifest.audit_required || effective_risk_class >= 3;

    let command_results = command_results_from_events(run_dir)?;
    let acceptance_passed =
        !command_results.is_empty() && command_results.iter().all(|cmd| cmd.exit_code == 0);
    let audit_verdict = load_audit(run_dir)?.map(|audit| audit.verdict);

    Ok(DevRunValidation {
        run_id: manifest.run_id,
        event_count: chain.event_count,
        event_chain_head_hash: chain.head_hash,
        effective_risk_class,
        restricted_surface_hits: hits,
        audit_required,
        audit_verdict,
        acceptance_passed,
        command_results,
        diff_artifact,
    })
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: close a dev run after required passing
/// command evidence and audit evidence are present.
pub fn close_run(run_dir: &Path) -> Result<DevRunSummary, DevHarnessError> {
    let validation = validate_run(run_dir)?;
    if validation.audit_required && validation.audit_verdict.as_deref() != Some("PROCEED") {
        return Err(DevHarnessError::AuditRequired);
    }
    if !validation.acceptance_passed {
        return Err(DevHarnessError::AcceptanceFailed);
    }

    append_event(
        run_dir,
        "closed",
        json!({
            "run_id": validation.run_id.clone(),
            "close_status": "closed",
            "acceptance_passed": validation.acceptance_passed,
            "audit_required": validation.audit_required,
            "audit_verdict": validation.audit_verdict.clone(),
        }),
    )?;
    let final_chain = validate_hash_chain(run_dir)?;
    let manifest_sha256 = sha256_file(&run_dir.join("DevTaskManifest.json"))?;
    let summary = DevRunSummary {
        schema_version: SCHEMA_VERSION.to_string(),
        run_id: validation.run_id.clone(),
        close_status: "closed".to_string(),
        manifest_sha256,
        event_chain_head_hash: final_chain.head_hash,
        diff_sha256: validation.diff_artifact.map(|artifact| artifact.sha256),
        command_results: validation.command_results,
        acceptance_passed: validation.acceptance_passed,
        audit_required: validation.audit_required,
        audit_verdict: validation.audit_verdict,
        effective_risk_class: validation.effective_risk_class,
        restricted_surface_hits: validation.restricted_surface_hits,
        closed_at_unix_ms: now_unix_ms(),
    };
    write_json_pretty(&run_dir.join("DevRunSummary.json"), &summary)?;
    Ok(summary)
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: read the persisted close summary for a
/// completed dev run.
pub fn summarize_run(run_dir: &Path) -> Result<DevRunSummary, DevHarnessError> {
    let path = run_dir.join("DevRunSummary.json");
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

/// TRACE_MATRIX FC3-N33 + FC3-N43: read the persisted task contract for a
/// dev run.
pub fn read_manifest(run_dir: &Path) -> Result<DevTaskManifest, DevHarnessError> {
    load_manifest(run_dir)
}

fn ensure_run_dir(run_dir: &Path) -> Result<(), DevHarnessError> {
    if !run_dir.join("DevTaskManifest.json").exists() {
        return Err(DevHarnessError::InvalidInput(format!(
            "run directory is missing DevTaskManifest.json: {}",
            run_dir.display()
        )));
    }
    fs::create_dir_all(run_dir.join("artifacts"))?;
    Ok(())
}

fn append_event(run_dir: &Path, event_type: &str, payload: Value) -> Result<(), DevHarnessError> {
    let summary = validate_hash_chain(run_dir)?;
    let event_index = summary.event_count;
    let prev_hash = summary.head_hash;
    let timestamp_unix_ms = now_unix_ms();
    let event_hash = compute_event_hash(
        event_index,
        event_type,
        timestamp_unix_ms,
        &payload,
        &prev_hash,
    )?;
    let event = DevEvent {
        schema_version: SCHEMA_VERSION.to_string(),
        event_index,
        event_type: event_type.to_string(),
        timestamp_unix_ms,
        payload,
        prev_hash,
        event_hash: event_hash.clone(),
    };

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(run_dir.join("events.jsonl"))?;
    writeln!(file, "{}", serde_json::to_string(&event)?)?;

    write_json_pretty(
        &run_dir.join("events_hash_chain.json"),
        &EventHashChainSummary {
            schema_version: SCHEMA_VERSION.to_string(),
            event_count: event_index + 1,
            head_hash: event_hash,
            updated_at_unix_ms: now_unix_ms(),
        },
    )?;
    Ok(())
}

fn validate_hash_chain(run_dir: &Path) -> Result<EventHashChainSummary, DevHarnessError> {
    let path = run_dir.join("events.jsonl");
    if !path.exists() {
        return Ok(EventHashChainSummary {
            schema_version: SCHEMA_VERSION.to_string(),
            event_count: 0,
            head_hash: ZERO_HASH.to_string(),
            updated_at_unix_ms: now_unix_ms(),
        });
    }

    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut expected_prev = ZERO_HASH.to_string();
    let mut count = 0usize;
    for (idx, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let event: DevEvent =
            serde_json::from_str(&line).map_err(|e| DevHarnessError::HashChainBroken {
                reason: format!("event line {} is not a DevEvent: {e}", idx + 1),
            })?;
        if event.event_index != count {
            return Err(DevHarnessError::HashChainBroken {
                reason: format!(
                    "event line {} has index {}, expected {}",
                    idx + 1,
                    event.event_index,
                    count
                ),
            });
        }
        if event.prev_hash != expected_prev {
            return Err(DevHarnessError::HashChainBroken {
                reason: format!(
                    "event line {} prev_hash mismatch: got {}, expected {}",
                    idx + 1,
                    event.prev_hash,
                    expected_prev
                ),
            });
        }
        let recomputed = compute_event_hash(
            event.event_index,
            &event.event_type,
            event.timestamp_unix_ms,
            &event.payload,
            &event.prev_hash,
        )?;
        if recomputed != event.event_hash {
            return Err(DevHarnessError::HashChainBroken {
                reason: format!(
                    "event line {} hash mismatch: got {}, expected {}",
                    idx + 1,
                    event.event_hash,
                    recomputed
                ),
            });
        }
        expected_prev = event.event_hash;
        count += 1;
    }

    let summary = EventHashChainSummary {
        schema_version: SCHEMA_VERSION.to_string(),
        event_count: count,
        head_hash: expected_prev,
        updated_at_unix_ms: now_unix_ms(),
    };

    if run_dir.join("events_hash_chain.json").exists() {
        let stored: EventHashChainSummary =
            serde_json::from_slice(&fs::read(run_dir.join("events_hash_chain.json"))?)?;
        if stored.event_count != summary.event_count || stored.head_hash != summary.head_hash {
            return Err(DevHarnessError::HashChainBroken {
                reason: "events_hash_chain.json does not match events.jsonl".to_string(),
            });
        }
    }

    Ok(summary)
}

fn command_results_from_events(run_dir: &Path) -> Result<Vec<CommandEvidence>, DevHarnessError> {
    let mut out = Vec::new();
    let path = run_dir.join("events.jsonl");
    if !path.exists() {
        return Ok(out);
    }
    for line in BufReader::new(File::open(path)?).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let event: DevEvent = serde_json::from_str(&line)?;
        if event.event_type == "command_recorded" {
            let command = event
                .payload
                .get("command")
                .ok_or_else(|| DevHarnessError::HashChainBroken {
                    reason: "command_recorded event missing command payload".to_string(),
                })
                .and_then(|value| serde_json::from_value(value.clone()).map_err(Into::into))?;
            out.push(command);
        }
    }
    Ok(out)
}

fn next_event_index(run_dir: &Path) -> Result<usize, DevHarnessError> {
    Ok(validate_hash_chain(run_dir)?.event_count)
}

fn compute_event_hash(
    event_index: usize,
    event_type: &str,
    timestamp_unix_ms: u128,
    payload: &Value,
    prev_hash: &str,
) -> Result<String, DevHarnessError> {
    let canonical = json!({
        "schema_version": SCHEMA_VERSION,
        "event_index": event_index,
        "event_type": event_type,
        "timestamp_unix_ms": timestamp_unix_ms,
        "payload": payload,
        "prev_hash": prev_hash,
    });
    Ok(sha256_bytes(serde_json::to_string(&canonical)?.as_bytes()))
}

fn load_manifest(run_dir: &Path) -> Result<DevTaskManifest, DevHarnessError> {
    Ok(serde_json::from_slice(&fs::read(
        run_dir.join("DevTaskManifest.json"),
    )?)?)
}

fn load_fc_witness(run_dir: &Path) -> Result<FCWitnessManifest, DevHarnessError> {
    Ok(serde_json::from_slice(&fs::read(
        run_dir.join("FCWitnessManifest.json"),
    )?)?)
}

fn load_audit(run_dir: &Path) -> Result<Option<DevAuditVerdict>, DevHarnessError> {
    let path = run_dir.join("DevAuditVerdict.json");
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice(&fs::read(path)?)?))
}

fn write_json_pretty<T: Serialize>(path: &Path, value: &T) -> Result<(), DevHarnessError> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes)?;
    Ok(())
}

fn artifact_ref_for(run_dir: &Path, path: &Path) -> Result<ArtifactRef, DevHarnessError> {
    let meta = fs::metadata(path)?;
    let rel = path
        .strip_prefix(run_dir)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    Ok(ArtifactRef {
        path: rel,
        sha256: sha256_file(path)?,
        size_bytes: meta.len(),
    })
}

fn sha256_file(path: &Path) -> Result<String, DevHarnessError> {
    Ok(sha256_bytes(&fs::read(path)?))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn restricted_hits_from_paths(paths: &[String]) -> Vec<String> {
    sorted_unique(
        paths
            .iter()
            .flat_map(|path| restricted_hit_for_path(path))
            .collect(),
    )
}

fn restricted_hits_from_diff(text: &str) -> Vec<String> {
    let mut paths = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            for token in rest.split_whitespace() {
                if let Some(path) = token
                    .strip_prefix("a/")
                    .or_else(|| token.strip_prefix("b/"))
                {
                    paths.push(path.to_string());
                }
            }
        } else if let Some(path) = line
            .strip_prefix("+++ b/")
            .or_else(|| line.strip_prefix("--- a/"))
        {
            paths.push(path.to_string());
        }
    }
    restricted_hits_from_paths(&paths)
}

fn restricted_hit_for_path(path: &str) -> Vec<String> {
    let patterns = [
        "src/state/sequencer.rs",
        "src/state/typed_tx.rs",
        "src/kernel.rs",
        "src/bus.rs",
        "src/sdk/tools/wallet.rs",
        "src/bottom_white/cas/schema.rs",
        "src/bottom_white/ledger/system_keypair.rs",
        "src/bottom_white/ledger/transition_ledger.rs",
        "genesis_payload.toml",
        "constitution.md",
        "handover/alignment/TRACE_FLOWCHART_MATRIX.md",
    ];
    let normalized = path.trim_start_matches("./");
    patterns
        .iter()
        .filter(|pattern| normalized == **pattern)
        .map(|pattern| (*pattern).to_string())
        .collect()
}

fn risk_floor_for_hits(hits: &[String]) -> u8 {
    if hits.is_empty() {
        0
    } else {
        4
    }
}

fn sorted_unique(values: Vec<String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
