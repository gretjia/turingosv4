//! TRACE_MATRIX FC1 + FC2 + FC3: A13 network-off Agentic OS run artifact.
//!
//! This module is compiled through `src/bin/turingos/cmd_os.rs` to avoid
//! changing the trust-root-pinned `src/runtime/mod.rs` without a Class 4 §8
//! packet. It produces deterministic fixture evidence only.

use git2::{Repository, Signature, Time};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const RUN_MANIFEST_SCHEMA: &str = "turingos.agentic_os.run_manifest.v1";
const TAPE_EVENT_SCHEMA: &str = "turingos.agentic_os.tape_event.v1";
const REQUIRED_DERIVED_ARTIFACTS: &[&str] = &[
    "replay_report.json",
    "predicate_receipts.jsonl",
    "external_call_receipts.jsonl",
    "economy_projection.json",
    "agent_view_audit.json",
];

/// TRACE_MATRIX FC1 + FC2: validated CLI request for the A13 network-off fixture.
#[derive(Debug, Clone)]
pub(crate) struct RunRequest {
    pub(crate) task: PathBuf,
    pub(crate) policy: String,
    pub(crate) market: String,
    pub(crate) network: String,
    pub(crate) out_dir: Option<PathBuf>,
}

impl RunRequest {
    /// TRACE_MATRIX FC1: fail closed on unsupported A13 execution modes.
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.policy != "single_tree" {
            return Err("A13 only supports --policy single_tree".to_string());
        }
        if self.market != "on" {
            return Err("A13 only supports --market on".to_string());
        }
        if self.network != "off" {
            return Err("A13 only supports --network off".to_string());
        }
        Ok(())
    }
}

/// TRACE_MATRIX FC1 + FC2: summary printed after a successful run.
#[derive(Debug, Clone)]
pub(crate) struct RunSummary {
    pub(crate) run_dir: PathBuf,
    pub(crate) final_tape_head: String,
}

/// TRACE_MATRIX FC2: replay verification summary.
#[derive(Debug, Clone)]
pub(crate) struct ReplaySummary {
    pub(crate) final_tape_head: String,
    pub(crate) verified_artifacts: usize,
}

/// TRACE_MATRIX FC3: audit predicate summary.
#[derive(Debug, Clone)]
pub(crate) struct AuditSummary {
    pub(crate) final_tape_head: String,
    pub(crate) checked_predicates: usize,
}

/// TRACE_MATRIX FC2 + FC3: fail-closed OS run/replay/audit error surface.
#[derive(Debug)]
pub(crate) struct OsRunError(String);

impl fmt::Display for OsRunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for OsRunError {}

impl From<std::io::Error> for OsRunError {
    fn from(err: std::io::Error) -> Self {
        Self(format!("io: {err}"))
    }
}

impl From<serde_json::Error> for OsRunError {
    fn from(err: serde_json::Error) -> Self {
        Self(format!("json: {err}"))
    }
}

impl From<git2::Error> for OsRunError {
    fn from(err: git2::Error) -> Self {
        Self(format!("git: {err}"))
    }
}

impl From<OsRunError> for String {
    fn from(err: OsRunError) -> Self {
        err.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RunManifest {
    schema: String,
    run_id: String,
    task_id: String,
    task_fixture_cid: String,
    policy: String,
    market_mode: String,
    network_policy: String,
    final_tape_head: String,
    git_tape_repo: String,
    replay_recipe: Vec<String>,
    derived_artifacts: Vec<DerivedArtifactRef>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DerivedArtifactRef {
    path: String,
    derived_from_tape_head: String,
    content_hash_or_cid: String,
    replay_recipe: String,
}

struct ReconstructedArtifacts {
    replay_report: Value,
    predicate_receipts: Vec<Value>,
    external_call_receipts: Vec<Value>,
    economy_projection: Value,
    agent_view_audit: Value,
}

struct Fixture {
    task_id: String,
    agent_id: String,
    prompt: String,
    expected_public_result: String,
    private_oracle: String,
    budget_microcredits: u64,
    supply_microcredits: u64,
    clearing_price_microcredits: u64,
}

struct AgentProposalFact {
    task_id: String,
    agent_id: String,
    task_fixture_cid: String,
    network_policy: String,
}

struct EconomyFact {
    market_mode: String,
    initial_supply_microcredits: u64,
    final_supply_microcredits: u64,
    clearing_price_microcredits: u64,
    budget_microcredits: u64,
}

/// TRACE_MATRIX FC1 + FC2: create deterministic GitTape-derived A13 artifacts.
pub(crate) fn run_network_off_fixture(request: RunRequest) -> Result<RunSummary, OsRunError> {
    request.validate().map_err(OsRunError)?;
    let task_bytes = fs::read(&request.task)?;
    let task_json: Value = serde_json::from_slice(&task_bytes)?;
    let fixture = parse_fixture(&task_json)?;
    let task_fixture_cid = sha256_cid(&task_bytes);
    let run_id = format!(
        "a13-{}-{}",
        sanitize_id(&fixture.task_id),
        &task_fixture_cid["sha256:".len()..18]
    );
    let run_dir = request
        .out_dir
        .clone()
        .unwrap_or_else(|| PathBuf::from("target/turingos-os-runs").join(&run_id));
    prepare_empty_run_dir(&run_dir)?;
    write_task_fixture_cas(&run_dir, &task_fixture_cid, &task_bytes)?;

    let tape_repo = run_dir.join("git_tape_repo");
    let tape_events = tape_events_for_fixture(&fixture, &task_fixture_cid);
    let final_tape_head = write_git_tape_repo(&tape_repo, &tape_events)?;

    let derived = reconstruct_artifacts(&tape_events, &fixture, &final_tape_head)?;
    write_derived_artifacts(&run_dir, &derived)?;
    write_run_manifest(
        &run_dir,
        &fixture,
        &request,
        &run_id,
        &task_fixture_cid,
        &final_tape_head,
    )?;

    Ok(RunSummary {
        run_dir,
        final_tape_head,
    })
}

/// TRACE_MATRIX FC2: verify run manifest, GitTape HEAD, derived watermarks, and hashes.
pub(crate) fn replay_run_dir(run_dir: &Path) -> Result<ReplaySummary, OsRunError> {
    let (manifest, reconstructed) = verify_replay_from_tape(run_dir)?;

    let replay_report = &reconstructed.replay_report;
    require_bool(&replay_report, "deterministic_replay_ok", true)?;
    require_u64(&replay_report, "pending_external_intents", 0)?;
    require_u64(&replay_report, "unsupported_success_claims", 0)?;

    Ok(ReplaySummary {
        final_tape_head: manifest.final_tape_head,
        verified_artifacts: manifest.derived_artifacts.len(),
    })
}

/// TRACE_MATRIX FC3: run A13 acceptance audit over a replay-verified run directory.
pub(crate) fn audit_run_dir(run_dir: &Path) -> Result<AuditSummary, OsRunError> {
    let (manifest, reconstructed) = verify_replay_from_tape(run_dir)?;
    let economy = &reconstructed.economy_projection;
    let agent_view = &reconstructed.agent_view_audit;

    require_bool(&economy, "conservation_ok", true)?;
    let initial = json_u64(&economy, "initial_supply_microcredits")?;
    let final_supply = json_u64(&economy, "final_supply_microcredits")?;
    if initial != final_supply {
        return Err(OsRunError(format!(
            "economy projection is not conserved: initial={initial}, final={final_supply}"
        )));
    }
    require_u64(&agent_view, "hidden_leak_count", 0)?;
    require_bool(&agent_view, "private_oracle_exposed", false)?;

    Ok(AuditSummary {
        final_tape_head: manifest.final_tape_head,
        checked_predicates: 4,
    })
}

fn parse_fixture(value: &Value) -> Result<Fixture, OsRunError> {
    let schema = json_string(value, "schema")?;
    if schema != "turingos.agentic_os.fixture.v1" {
        return Err(OsRunError(format!("unsupported fixture schema: {schema}")));
    }
    let market = value
        .get("market")
        .and_then(Value::as_object)
        .ok_or_else(|| OsRunError("fixture missing market object".to_string()))?;
    if market.get("enabled").and_then(Value::as_bool) != Some(true) {
        return Err(OsRunError(
            "fixture market.enabled must be true for A13".to_string(),
        ));
    }
    Ok(Fixture {
        task_id: json_string(value, "task_id")?,
        agent_id: json_string(value, "agent_id")?,
        prompt: json_string(value, "prompt")?,
        expected_public_result: json_string(value, "expected_public_result")?,
        private_oracle: json_string(value, "private_oracle")?,
        budget_microcredits: json_u64(value, "budget_microcredits")?,
        supply_microcredits: object_u64(market, "supply_microcredits")?,
        clearing_price_microcredits: object_u64(market, "clearing_price_microcredits")?,
    })
}

fn tape_events_for_fixture(fixture: &Fixture, task_fixture_cid: &str) -> Vec<Value> {
    vec![
        json!({
            "schema": TAPE_EVENT_SCHEMA,
            "logical_t": 1,
            "event_kind": "AgentProposal",
            "task_id": fixture.task_id,
            "agent_id": fixture.agent_id,
            "task_fixture_cid": task_fixture_cid,
            "network_policy": "off"
        }),
        json!({
            "schema": TAPE_EVENT_SCHEMA,
            "logical_t": 2,
            "event_kind": "ExternalCallIntent",
            "intent_id": "intent-a13-network-off",
            "provider": "network-off-fixture",
            "may_spend": false
        }),
        json!({
            "schema": TAPE_EVENT_SCHEMA,
            "logical_t": 3,
            "event_kind": "ExternalCallTerminal",
            "intent_id": "intent-a13-network-off",
            "terminal_kind": "NetworkOffMocked",
            "may_have_spent": false
        }),
        json!({
            "schema": TAPE_EVENT_SCHEMA,
            "logical_t": 4,
            "event_kind": "EconomyEvent",
            "economy_event_kind": "NetworkOffSettlement",
            "market_mode": "on",
            "initial_supply_microcredits": fixture.supply_microcredits,
            "final_supply_microcredits": fixture.supply_microcredits,
            "clearing_price_microcredits": fixture.clearing_price_microcredits,
            "budget_microcredits": fixture.budget_microcredits
        }),
    ]
}

fn write_git_tape_repo(repo_path: &Path, events: &[Value]) -> Result<String, OsRunError> {
    fs::create_dir_all(repo_path)?;
    let repo = Repository::init(repo_path)?;
    fs::create_dir_all(repo_path.join("tape"))?;
    write_jsonl(&repo_path.join("tape/events.jsonl"), events)?;

    let mut index = repo.index()?;
    index.add_path(Path::new("tape/events.jsonl"))?;
    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let time = Time::new(0, 0);
    let signature = Signature::new(
        "turingos a13 fixture sequencer",
        "system@turingos.local",
        &time,
    )?;
    let oid = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "a13 network-off fixture tape\n",
        &tree,
        &[],
    )?;
    Ok(oid.to_string())
}

fn reconstruct_artifacts(
    events: &[Value],
    fixture: &Fixture,
    head: &str,
) -> Result<ReconstructedArtifacts, OsRunError> {
    let proposal = proposal_fact(events)?;
    if proposal.task_id != fixture.task_id {
        return Err(OsRunError(format!(
            "AgentProposal task_id drift: tape={}, fixture={}",
            proposal.task_id, fixture.task_id
        )));
    }
    if proposal.agent_id != fixture.agent_id {
        return Err(OsRunError(format!(
            "AgentProposal agent_id drift: tape={}, fixture={}",
            proposal.agent_id, fixture.agent_id
        )));
    }
    if proposal.network_policy != "off" {
        return Err(OsRunError(format!(
            "A13 replay only accepts network_policy=off, got {}",
            proposal.network_policy
        )));
    }

    let economy_fact = economy_fact(events)?;
    if economy_fact.initial_supply_microcredits != fixture.supply_microcredits {
        return Err(OsRunError(format!(
            "EconomyEvent initial_supply drift: tape={}, fixture={}",
            economy_fact.initial_supply_microcredits, fixture.supply_microcredits
        )));
    }
    if economy_fact.clearing_price_microcredits != fixture.clearing_price_microcredits {
        return Err(OsRunError(format!(
            "EconomyEvent clearing_price drift: tape={}, fixture={}",
            economy_fact.clearing_price_microcredits, fixture.clearing_price_microcredits
        )));
    }
    if economy_fact.budget_microcredits != fixture.budget_microcredits {
        return Err(OsRunError(format!(
            "EconomyEvent budget drift: tape={}, fixture={}",
            economy_fact.budget_microcredits, fixture.budget_microcredits
        )));
    }
    let (external_receipts, pending_external_intents, external_terminal_count) =
        external_call_receipts(events, head)?;

    let public_view = json!({
        "task_id": fixture.task_id,
        "agent_id": fixture.agent_id,
        "prompt_sha256": sha256_hex(fixture.prompt.as_bytes()),
        "result": fixture.expected_public_result
    });
    let private_oracle_exposed =
        serde_json::to_string(&public_view)?.contains(&fixture.private_oracle);
    let hidden_leak_count = if private_oracle_exposed { 1_u64 } else { 0_u64 };

    let predicate_receipts = vec![
        json!({
            "schema": "turingos.agentic_os.predicate_receipt.v1",
            "derived_from_tape_head": head,
            "predicate_id": "fixture_task_loaded",
            "pass": true
        }),
        json!({
            "schema": "turingos.agentic_os.predicate_receipt.v1",
            "derived_from_tape_head": head,
            "predicate_id": "external_intents_closed",
            "pass": pending_external_intents == 0
        }),
        json!({
            "schema": "turingos.agentic_os.predicate_receipt.v1",
            "derived_from_tape_head": head,
            "predicate_id": "agent_view_shielded",
            "pass": hidden_leak_count == 0
        }),
    ];

    let economy_projection = json!({
        "schema": "turingos.agentic_os.economy_projection.v1",
        "derived_from_tape_head": head,
        "market_mode": economy_fact.market_mode,
        "settlement_kind": "network_off_fixture_projection",
        "initial_supply_microcredits": economy_fact.initial_supply_microcredits,
        "final_supply_microcredits": economy_fact.final_supply_microcredits,
        "clearing_price_microcredits": economy_fact.clearing_price_microcredits,
        "budget_microcredits": economy_fact.budget_microcredits,
        "conservation_ok": economy_fact.initial_supply_microcredits
            == economy_fact.final_supply_microcredits
    });

    let agent_view_audit = json!({
        "schema": "turingos.agentic_os.agent_view_audit.v1",
        "derived_from_tape_head": head,
        "hidden_leak_count": hidden_leak_count,
        "private_oracle_exposed": private_oracle_exposed,
        "public_view": public_view
    });

    let replay_report = json!({
        "schema": "turingos.agentic_os.replay_report.v1",
        "derived_from_tape_head": head,
        "deterministic_replay_ok": true,
        "task_fixture_cid": proposal.task_fixture_cid,
        "pending_external_intents": pending_external_intents,
        "unsupported_success_claims": unsupported_success_claims(events)?,
        "predicate_receipt_count": predicate_receipts.len() as u64,
        "external_terminal_count": external_terminal_count
    });

    Ok(ReconstructedArtifacts {
        replay_report,
        predicate_receipts,
        external_call_receipts: external_receipts,
        economy_projection,
        agent_view_audit,
    })
}

fn write_derived_artifacts(
    run_dir: &Path,
    derived: &ReconstructedArtifacts,
) -> Result<(), OsRunError> {
    write_json(&run_dir.join("replay_report.json"), &derived.replay_report)?;
    write_jsonl(
        &run_dir.join("predicate_receipts.jsonl"),
        &derived.predicate_receipts,
    )?;
    write_jsonl(
        &run_dir.join("external_call_receipts.jsonl"),
        &derived.external_call_receipts,
    )?;
    write_json(
        &run_dir.join("economy_projection.json"),
        &derived.economy_projection,
    )?;
    write_json(
        &run_dir.join("agent_view_audit.json"),
        &derived.agent_view_audit,
    )?;

    Ok(())
}

fn proposal_fact(events: &[Value]) -> Result<AgentProposalFact, OsRunError> {
    let mut proposal = None;
    for event in events {
        if event_string(event, "event_kind")? != "AgentProposal" {
            continue;
        }
        if proposal.is_some() {
            return Err(OsRunError(
                "GitTape contains duplicate AgentProposal events".to_string(),
            ));
        }
        proposal = Some(AgentProposalFact {
            task_id: event_string(event, "task_id")?,
            agent_id: event_string(event, "agent_id")?,
            task_fixture_cid: event_string(event, "task_fixture_cid")?,
            network_policy: event_string(event, "network_policy")?,
        });
    }
    proposal.ok_or_else(|| OsRunError("GitTape missing AgentProposal event".to_string()))
}

fn economy_fact(events: &[Value]) -> Result<EconomyFact, OsRunError> {
    let mut economy = None;
    for event in events {
        if event_string(event, "event_kind")? != "EconomyEvent" {
            continue;
        }
        if event_string(event, "economy_event_kind")? != "NetworkOffSettlement" {
            return Err(OsRunError(format!(
                "unsupported economy_event_kind: {}",
                event_string(event, "economy_event_kind")?
            )));
        }
        if economy.is_some() {
            return Err(OsRunError(
                "GitTape contains duplicate EconomyEvent events".to_string(),
            ));
        }
        let market_mode = event_string(event, "market_mode")?;
        if market_mode != "on" {
            return Err(OsRunError(format!(
                "A13 replay only accepts market_mode=on, got {market_mode}"
            )));
        }
        economy = Some(EconomyFact {
            market_mode,
            initial_supply_microcredits: event_u64(event, "initial_supply_microcredits")?,
            final_supply_microcredits: event_u64(event, "final_supply_microcredits")?,
            clearing_price_microcredits: event_u64(event, "clearing_price_microcredits")?,
            budget_microcredits: event_u64(event, "budget_microcredits")?,
        });
    }
    economy.ok_or_else(|| OsRunError("GitTape missing EconomyEvent event".to_string()))
}

fn external_call_receipts(
    events: &[Value],
    head: &str,
) -> Result<(Vec<Value>, u64, u64), OsRunError> {
    let mut intents = BTreeMap::new();
    let mut terminals = BTreeMap::new();
    for event in events {
        match event_string(event, "event_kind")?.as_str() {
            "ExternalCallIntent" => {
                let intent_id = event_string(event, "intent_id")?;
                if intents.insert(intent_id.clone(), ()).is_some() {
                    return Err(OsRunError(format!(
                        "GitTape contains duplicate ExternalCallIntent: {intent_id}"
                    )));
                }
            }
            "ExternalCallTerminal" => {
                let intent_id = event_string(event, "intent_id")?;
                let terminal = (
                    event_string(event, "terminal_kind")?,
                    event_bool(event, "may_have_spent")?,
                );
                if terminals.insert(intent_id.clone(), terminal).is_some() {
                    return Err(OsRunError(format!(
                        "GitTape contains duplicate ExternalCallTerminal: {intent_id}"
                    )));
                }
            }
            _ => {}
        }
    }

    for intent_id in terminals.keys() {
        if !intents.contains_key(intent_id) {
            return Err(OsRunError(format!(
                "ExternalCallTerminal has no matching intent: {intent_id}"
            )));
        }
    }

    let mut receipts = Vec::with_capacity(intents.len());
    let mut pending = 0_u64;
    for intent_id in intents.keys() {
        if let Some((terminal_kind, may_have_spent)) = terminals.get(intent_id) {
            receipts.push(json!({
                "schema": "turingos.agentic_os.external_call_receipt.v1",
                "derived_from_tape_head": head,
                "intent_id": intent_id,
                "terminal_kind": terminal_kind,
                "pending": false,
                "may_have_spent": may_have_spent
            }));
        } else {
            pending += 1;
            receipts.push(json!({
                "schema": "turingos.agentic_os.external_call_receipt.v1",
                "derived_from_tape_head": head,
                "intent_id": intent_id,
                "terminal_kind": "Pending",
                "pending": true,
                "may_have_spent": false
            }));
        }
    }

    Ok((receipts, pending, terminals.len() as u64))
}

fn unsupported_success_claims(events: &[Value]) -> Result<u64, OsRunError> {
    let mut count = 0_u64;
    for event in events {
        if event_string(event, "event_kind")? == "UnsupportedSuccessClaim" {
            count += 1;
        }
    }
    Ok(count)
}

fn write_task_fixture_cas(run_dir: &Path, cid: &str, bytes: &[u8]) -> Result<(), OsRunError> {
    let path = cas_object_path(run_dir, cid)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn read_task_fixture_cas(run_dir: &Path, cid: &str) -> Result<Fixture, OsRunError> {
    let path = cas_object_path(run_dir, cid)?;
    let bytes = fs::read(&path)?;
    let actual = sha256_cid(&bytes);
    if actual != cid {
        return Err(OsRunError(format!(
            "CAS object hash mismatch: expected={cid}, actual={actual}"
        )));
    }
    let value: Value = serde_json::from_slice(&bytes)?;
    parse_fixture(&value)
}

fn cas_object_path(run_dir: &Path, cid: &str) -> Result<PathBuf, OsRunError> {
    let hex = cid
        .strip_prefix("sha256:")
        .ok_or_else(|| OsRunError(format!("unsupported CAS cid: {cid}")))?;
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(OsRunError(format!("unsupported CAS cid: {cid}")));
    }
    Ok(run_dir.join("cas/objects/sha256").join(hex))
}

fn verify_replay_from_tape(
    run_dir: &Path,
) -> Result<(RunManifest, ReconstructedArtifacts), OsRunError> {
    let manifest = read_manifest(run_dir)?;
    let tape_events = read_git_tape_events_from_head(run_dir, &manifest)?;
    let proposal = proposal_fact(&tape_events)?;
    if proposal.task_id != manifest.task_id {
        return Err(OsRunError(format!(
            "run manifest task_id drift: manifest={}, tape={}",
            manifest.task_id, proposal.task_id
        )));
    }
    if proposal.task_fixture_cid != manifest.task_fixture_cid {
        return Err(OsRunError(format!(
            "run manifest task_fixture_cid drift: manifest={}, tape={}",
            manifest.task_fixture_cid, proposal.task_fixture_cid
        )));
    }
    let fixture = read_task_fixture_cas(run_dir, &proposal.task_fixture_cid)?;
    let reconstructed = reconstruct_artifacts(&tape_events, &fixture, &manifest.final_tape_head)?;
    for artifact in &manifest.derived_artifacts {
        verify_artifact(run_dir, &manifest.final_tape_head, artifact, &reconstructed)?;
    }
    Ok((manifest, reconstructed))
}

fn read_git_tape_events_from_head(
    run_dir: &Path,
    manifest: &RunManifest,
) -> Result<Vec<Value>, OsRunError> {
    let repo = Repository::open(run_dir.join(&manifest.git_tape_repo))?;
    let head_oid = repo
        .head()?
        .target()
        .ok_or_else(|| OsRunError("git_tape_repo HEAD is not a direct commit".to_string()))?;
    let head = head_oid.to_string();
    if head != manifest.final_tape_head {
        return Err(OsRunError(format!(
            "git_tape_repo HEAD drift: manifest={}, actual={head}",
            manifest.final_tape_head
        )));
    }

    let commit = repo.find_commit(head_oid)?;
    let tree = commit.tree()?;
    let entry = tree.get_path(Path::new("tape/events.jsonl"))?;
    let blob = repo.find_blob(entry.id())?;
    let raw = std::str::from_utf8(blob.content())
        .map_err(|err| OsRunError(format!("GitTape events are not UTF-8: {err}")))?;
    parse_tape_events(raw)
}

fn parse_tape_events(raw: &str) -> Result<Vec<Value>, OsRunError> {
    let mut events = Vec::new();
    let mut expected_t = 1_u64;
    for (idx, line) in raw.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let event: Value = serde_json::from_str(line)?;
        let schema = event_string(&event, "schema")?;
        if schema != TAPE_EVENT_SCHEMA {
            return Err(OsRunError(format!(
                "GitTape event line {} has unsupported schema: {schema}",
                idx + 1
            )));
        }
        let event_kind = event_string(&event, "event_kind")?;
        if !matches!(
            event_kind.as_str(),
            "AgentProposal"
                | "ExternalCallIntent"
                | "ExternalCallTerminal"
                | "EconomyEvent"
                | "UnsupportedSuccessClaim"
        ) {
            return Err(OsRunError(format!(
                "GitTape event line {} has unsupported event_kind: {event_kind}",
                idx + 1
            )));
        }
        let logical_t = event_u64(&event, "logical_t")?;
        if logical_t != expected_t {
            return Err(OsRunError(format!(
                "GitTape logical_t discontinuity at line {}: expected={expected_t}, actual={logical_t}",
                idx + 1
            )));
        }
        expected_t += 1;
        events.push(event);
    }
    if events.is_empty() {
        return Err(OsRunError("GitTape has no events".to_string()));
    }
    Ok(events)
}

fn artifact_bytes(
    artifact_path: &str,
    derived: &ReconstructedArtifacts,
) -> Result<Vec<u8>, OsRunError> {
    match artifact_path {
        "replay_report.json" => canonical_json_bytes(&derived.replay_report),
        "predicate_receipts.jsonl" => canonical_jsonl_bytes(&derived.predicate_receipts),
        "external_call_receipts.jsonl" => canonical_jsonl_bytes(&derived.external_call_receipts),
        "economy_projection.json" => canonical_json_bytes(&derived.economy_projection),
        "agent_view_audit.json" => canonical_json_bytes(&derived.agent_view_audit),
        _ => Err(OsRunError(format!(
            "derived artifact is not reconstructable from GitTape/CAS: {artifact_path}"
        ))),
    }
}

fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, OsRunError> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn canonical_jsonl_bytes(values: &[Value]) -> Result<Vec<u8>, OsRunError> {
    let mut bytes = Vec::new();
    for value in values {
        serde_json::to_writer(&mut bytes, value)?;
        bytes.push(b'\n');
    }
    Ok(bytes)
}

fn event_string(value: &Value, key: &str) -> Result<String, OsRunError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| OsRunError(format!("GitTape event missing string `{key}`")))
}

fn event_u64(value: &Value, key: &str) -> Result<u64, OsRunError> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| OsRunError(format!("GitTape event missing integer `{key}`")))
}

fn event_bool(value: &Value, key: &str) -> Result<bool, OsRunError> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| OsRunError(format!("GitTape event missing bool `{key}`")))
}

fn write_run_manifest(
    run_dir: &Path,
    fixture: &Fixture,
    request: &RunRequest,
    run_id: &str,
    task_fixture_cid: &str,
    head: &str,
) -> Result<(), OsRunError> {
    let artifacts = [
        "replay_report.json",
        "predicate_receipts.jsonl",
        "external_call_receipts.jsonl",
        "economy_projection.json",
        "agent_view_audit.json",
    ];
    let mut derived_artifacts = Vec::with_capacity(artifacts.len());
    for path in artifacts {
        derived_artifacts.push(DerivedArtifactRef {
            path: path.to_string(),
            derived_from_tape_head: head.to_string(),
            content_hash_or_cid: file_sha256_cid(&run_dir.join(path))?,
            replay_recipe: format!("turingos os replay --run-dir {}", run_dir.display()),
        });
    }

    let manifest = RunManifest {
        schema: RUN_MANIFEST_SCHEMA.to_string(),
        run_id: run_id.to_string(),
        task_id: fixture.task_id.clone(),
        task_fixture_cid: task_fixture_cid.to_string(),
        policy: request.policy.clone(),
        market_mode: request.market.clone(),
        network_policy: request.network.clone(),
        final_tape_head: head.to_string(),
        git_tape_repo: "git_tape_repo".to_string(),
        replay_recipe: vec![
            "turingos os replay --run-dir <run-dir>".to_string(),
            "turingos os audit --run-dir <run-dir>".to_string(),
        ],
        derived_artifacts,
    };
    write_json(&run_dir.join("run_manifest.json"), &manifest)
}

fn read_manifest(run_dir: &Path) -> Result<RunManifest, OsRunError> {
    let bytes = fs::read(run_dir.join("run_manifest.json"))?;
    let manifest: RunManifest = serde_json::from_slice(&bytes)?;
    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn validate_manifest(manifest: &RunManifest) -> Result<(), OsRunError> {
    if manifest.schema != RUN_MANIFEST_SCHEMA {
        return Err(OsRunError(format!(
            "unsupported run manifest schema: {}",
            manifest.schema
        )));
    }
    if manifest.policy != "single_tree" {
        return Err(OsRunError(format!(
            "unsupported run manifest policy: {}",
            manifest.policy
        )));
    }
    if manifest.market_mode != "on" {
        return Err(OsRunError(format!(
            "unsupported run manifest market_mode: {}",
            manifest.market_mode
        )));
    }
    if manifest.network_policy != "off" {
        return Err(OsRunError(format!(
            "unsupported run manifest network_policy: {}",
            manifest.network_policy
        )));
    }
    if !is_hex_git_oid(&manifest.final_tape_head) {
        return Err(OsRunError(format!(
            "run manifest final_tape_head is not a git object id: {}",
            manifest.final_tape_head
        )));
    }
    if manifest.git_tape_repo != "git_tape_repo" {
        return Err(OsRunError(format!(
            "run manifest git_tape_repo must be git_tape_repo, got {}",
            manifest.git_tape_repo
        )));
    }

    let mut seen = BTreeSet::new();
    for artifact in &manifest.derived_artifacts {
        require_safe_manifest_path("derived artifact", &artifact.path)?;
        if !REQUIRED_DERIVED_ARTIFACTS.contains(&artifact.path.as_str()) {
            return Err(OsRunError(format!(
                "run manifest contains unsupported derived artifact {}",
                artifact.path
            )));
        }
        if !seen.insert(artifact.path.as_str()) {
            return Err(OsRunError(format!(
                "duplicate derived artifact in run manifest: {}",
                artifact.path
            )));
        }
    }
    for required in REQUIRED_DERIVED_ARTIFACTS {
        if !seen.contains(required) {
            return Err(OsRunError(format!(
                "run manifest missing derived artifact {required}"
            )));
        }
    }
    Ok(())
}

fn verify_artifact(
    run_dir: &Path,
    head: &str,
    artifact: &DerivedArtifactRef,
    reconstructed: &ReconstructedArtifacts,
) -> Result<(), OsRunError> {
    require_safe_manifest_path("derived artifact", &artifact.path)?;
    if artifact.derived_from_tape_head != head {
        return Err(OsRunError(format!(
            "artifact manifest watermark mismatch for {}",
            artifact.path
        )));
    }
    let expected_bytes = artifact_bytes(&artifact.path, reconstructed)?;
    let expected_hash = sha256_cid(&expected_bytes);
    if artifact.content_hash_or_cid != expected_hash {
        return Err(OsRunError(format!(
            "artifact manifest hash is not the GitTape/CAS reconstruction for {}: manifest={}, reconstructed={expected_hash}",
            artifact.path, artifact.content_hash_or_cid
        )));
    }
    let actual_bytes = fs::read(run_dir.join(&artifact.path))?;
    let actual_hash = sha256_cid(&actual_bytes);
    if actual_hash != expected_hash {
        return Err(OsRunError(format!(
            "artifact does not match GitTape/CAS reconstruction for {}: reconstructed={expected_hash}, actual={actual_hash}",
            artifact.path
        )));
    }
    if actual_bytes != expected_bytes {
        return Err(OsRunError(format!(
            "artifact bytes drift from GitTape/CAS reconstruction for {}",
            artifact.path
        )));
    }
    Ok(())
}

fn prepare_empty_run_dir(run_dir: &Path) -> Result<(), OsRunError> {
    if run_dir.exists() {
        let mut entries = fs::read_dir(run_dir)?;
        if entries.next().transpose()?.is_some() {
            return Err(OsRunError(format!(
                "out-dir already exists and is not empty: {}",
                run_dir.display()
            )));
        }
    }
    fs::create_dir_all(run_dir)?;
    Ok(())
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), OsRunError> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, [bytes, b"\n".to_vec()].concat())?;
    Ok(())
}

fn write_jsonl(path: &Path, values: &[Value]) -> Result<(), OsRunError> {
    let mut file = fs::File::create(path)?;
    for value in values {
        serde_json::to_writer(&mut file, value)?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn file_sha256_cid(path: &Path) -> Result<String, OsRunError> {
    Ok(sha256_cid(&fs::read(path)?))
}

fn sha256_cid(bytes: &[u8]) -> String {
    format!("sha256:{}", sha256_hex(bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn sanitize_id(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn json_string(value: &Value, key: &str) -> Result<String, OsRunError> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| OsRunError(format!("fixture missing string `{key}`")))
}

fn json_u64(value: &Value, key: &str) -> Result<u64, OsRunError> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| OsRunError(format!("fixture missing integer `{key}`")))
}

fn object_u64(map: &serde_json::Map<String, Value>, key: &str) -> Result<u64, OsRunError> {
    map.get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| OsRunError(format!("fixture market missing integer `{key}`")))
}

fn is_hex_git_oid(raw: &str) -> bool {
    raw.len() == 40 && raw.bytes().all(|b| b.is_ascii_hexdigit())
}

fn require_safe_manifest_path(kind: &str, raw: &str) -> Result<(), OsRunError> {
    let path = Path::new(raw);
    if raw.is_empty() || path.is_absolute() {
        return Err(OsRunError(format!("{kind} path is unsafe: {raw}")));
    }
    let safe = path
        .components()
        .all(|component| matches!(component, Component::Normal(_)));
    if !safe {
        return Err(OsRunError(format!("{kind} path is unsafe: {raw}")));
    }
    Ok(())
}

fn require_bool(value: &Value, key: &str, expected: bool) -> Result<(), OsRunError> {
    let actual = value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| OsRunError(format!("missing bool `{key}`")))?;
    if actual != expected {
        return Err(OsRunError(format!(
            "`{key}` expected {expected}, got {actual}"
        )));
    }
    Ok(())
}

fn require_u64(value: &Value, key: &str, expected: u64) -> Result<(), OsRunError> {
    let actual = json_u64(value, key)?;
    if actual != expected {
        return Err(OsRunError(format!(
            "`{key}` expected {expected}, got {actual}"
        )));
    }
    Ok(())
}
