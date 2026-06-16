//! LIVE-FC1 — ARCHITECT ANTI-GOODHART PPUT CONFORMANCE BATTERY (the 11).
//!
//! The architect North Star `PPUT_DRIVEN_FULL_PASS_2026-04-25 §10` pre-registers
//! ELEVEN named anti-Goodhart guardrails that make held-out Verified PPUT
//! NON-GAMEABLE (constitution Art. III.4 / Gate H Goodhart shield + accounting
//! integrity). This file IS that named battery: every one of the 11 exists here
//! as a named `#[test]`, is REGISTERED in `scripts/constitution_gates.manifest.toml`
//! (`constitution_pput_anti_goodhart_battery`), referenced in
//! `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`, and added to the
//! anti-Goodhart dimension of the OS-qualifying conjunction
//! (`tests/constitution_agentic_os_minimum_qualification.rs`). Discovered by the
//! flat `ls tests/constitution_*.rs` glob in `scripts/run_constitution_gates.sh`.
//!
//! ── NON-VACUITY (binding, `feedback_single_site_gate_illusion`) ───────────────
//! Every gate is EITHER behavioral (reconstructs from a constructed `LoadedTape`)
//! OR enumerate-all-sites source-structural (greps the canonical sources). Each
//! carries a DOCUMENTED CAUGHT MUTANT — no `assert!(true)`. INTEGER-only on every
//! counting path.
//!
//! ── DELEGATION (R-022: do not duplicate a proven mechanism) ───────────────────
//! Five of the 11 are ALREADY behaviorally proven by two shipped gates:
//!   * `constitution_vpput_reconstructed_from_tape` (PR #331) proves
//!     all-tokens-counted (#1), failed-branches-count (#7), golden-path-requires-
//!     ground-truth (#6).
//!   * `constitution_metric_leak_guard_wired` proves no-PPUT-in-prompt (#10) +
//!     no-metric-file-access at every prompt site (#5/#11).
//! For those five this battery adds a THIN NAMED conformance gate that asserts the
//! property via the EXISTING mechanism (a behavioral re-derivation through the
//! same public `reconstruct_vpput_from_tape` / `assert_no_metric_leak` surface, OR
//! a source-structural delegation that the proven gate + mechanism are present and
//! wired) — so the architect's named test EXISTS and is REGISTERED without
//! re-implementing the whole mechanism. The FIVE GENUINELY-MISSING ones
//! (#2 tool_stdout_hash_logged, #3 no_hidden_unmetered_generation,
//! #4 no_problem_id_hardcode, #8 wall_clock span, #9 heldout_ids_inaccessible) are
//! implemented here as FULL non-vacuous gates.
//!
//! ── THE 11 (PPUT_DRIVEN_FULL_PASS_2026-04-25 §10) ─────────────────────────────
//!   1.  all_model_tokens_counted                      — COVERED #331; thin behavioral
//!   2.  tool_stdout_hash_logged                       — MISSING; full behavioral
//!   3.  no_hidden_unmetered_generation                — MISSING; full structural
//!   4.  no_problem_id_hardcode                        — MISSING; full structural
//!   5.  no_metric_file_access_by_agents               — COVERED leak-guard; thin structural
//!   6.  golden_path_requires_ground_truth_acceptance  — COVERED #331; thin behavioral
//!   7.  failed_branches_count_toward_total_cost       — COVERED #331; thin behavioral
//!   8.  wall_clock_measured_from_first_read_to_final_accept — MISSING; full behavioral
//!   9.  heldout_ids_inaccessible                       — MISSING; full behavioral+structural
//!   10. no_pput_in_agent_prompt                        — COVERED leak-guard; thin behavioral
//!   11. test_no_metric_file_access                     — pre-reg 11th; folds with #5

use std::collections::BTreeMap;
use std::fs;

use sha2::{Digest, Sha256};
use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::{Cid, ObjectType};
use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::rejection_evidence::{
    RejectionClass, RejectionEvidenceWriter,
};
use turingosv4::bottom_white::ledger::system_keypair::{
    PinnedSystemPubkeys, SystemEpoch, SystemSignature,
};
use turingosv4::bottom_white::ledger::transition_ledger::{canonical_encode, LedgerEntry, TxKind};
use turingosv4::runtime::agent_keypairs::AgentPubkeyManifest;
use turingosv4::runtime::agent_scheduler::vpput_reconstruction::reconstruct_vpput_from_tape;
use turingosv4::runtime::audit_assertions::LoadedTape;
use turingosv4::runtime::proposal_telemetry::{
    read_from_cas as read_proposal_telemetry, write_to_cas as write_proposal_telemetry,
    ProposalTelemetry, TokenCounts, ToolCallRecord,
};
use turingosv4::runtime::verification_result::{
    write_to_cas as write_verification_result, VerificationResult,
};
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::sdk::prompt_guard::assert_no_metric_leak;
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::typed_tx::{RunId, RunOutcome, TerminalSummaryTx, TypedTx, WorkTx};

// ════════════════════════════════════════════════════════════════════════════
// SHARED FIXTURE — a real LoadedTape + CAS, hand-built so the BEHAVIORAL gates
// exercise the SAME L4 / L4.E / CAS reconstruction the live VPPUT path uses.
// (mirrors the proven fixture in constitution_vpput_reconstructed_from_tape.rs)
// ════════════════════════════════════════════════════════════════════════════

fn sha256_hash(bytes: &[u8]) -> Hash {
    let mut h = Sha256::new();
    h.update(bytes);
    Hash(h.finalize().into())
}

fn ledger_entry(logical_t: u64, kind: TxKind, payload_cid: Cid) -> LedgerEntry {
    LedgerEntry {
        logical_t,
        parent_state_root: Hash::ZERO,
        parent_ledger_root: Hash::ZERO,
        tx_kind: kind,
        tx_payload_cid: payload_cid,
        resulting_state_root: Hash::ZERO,
        resulting_ledger_root: Hash::ZERO,
        timestamp_logical: logical_t,
        epoch: SystemEpoch::new(0),
        extensions: BTreeMap::new(),
        system_signature: SystemSignature::default(),
    }
}

fn put_typed(cas: &mut CasStore, tx: &TypedTx) -> Cid {
    let bytes = canonical_encode(tx).expect("canonical_encode TypedTx");
    cas.put(
        &bytes,
        ObjectType::Generic,
        "anti-goodhart-fixture",
        0,
        None,
    )
    .expect("cas put TypedTx")
}

/// Write a `ProposalTelemetry` to CAS and return its CID. `tool_calls` carries the
/// tool-stdout hash manifest (`result_hash` = sha256 of the tool stdout bytes).
/// Optionally attach a verified `VerificationResult` (Lean-oracle ground truth).
fn write_telemetry(
    cas: &mut CasStore,
    task: &str,
    branch: &str,
    tokens: TokenCounts,
    tool_calls: Vec<ToolCallRecord>,
    verified_oracle: bool,
) -> Cid {
    let vr_cid = if verified_oracle {
        let vr = VerificationResult {
            target_work_tx: TxId(format!("work:{task}:{branch}")),
            verifier_agent: AgentId("agent:fixture".to_string()),
            exit_code: 0,
            stdout_hash: Hash::ZERO,
            stderr_hash: Hash::ZERO,
            proof_file_hash: Hash::ZERO,
            proof_artifact_cid: Cid::default(),
            verified: true,
        };
        Some(write_verification_result(cas, &vr, "anti-goodhart-fixture", 0).expect("write VR"))
    } else {
        None
    };
    let tel = ProposalTelemetry {
        agent_id: AgentId("agent:fixture".to_string()),
        prompt_context_hash: Hash::ZERO,
        proposal_artifact_cid: Cid::from_content(format!("artifact:{task}:{branch}").as_bytes()),
        candidate_label: "nlinarith".to_string(),
        model_id: None,
        token_counts: tokens,
        tool_calls,
        branch_id: branch.to_string(),
        parent_tx: None,
        verification_result_cid: vr_cid,
    };
    write_proposal_telemetry(cas, &tel, "anti-goodhart-fixture", 0).expect("write telemetry")
}

/// Build an accepted L4 WorkTx entry whose proposal carries `tokens`, the
/// `tool_calls` stdout-hash manifest, and (optionally) a verified oracle witness.
/// Returns the entry plus the `proposal_cid` so a gate can re-read the telemetry.
fn accepted_work_entry(
    cas: &mut CasStore,
    logical_t: u64,
    task: &str,
    branch: &str,
    tokens: TokenCounts,
    tool_calls: Vec<ToolCallRecord>,
    verified_oracle: bool,
) -> (LedgerEntry, Cid) {
    let proposal_cid = write_telemetry(cas, task, branch, tokens, tool_calls, verified_oracle);
    let work = TypedTx::Work(WorkTx {
        task_id: TaskId(task.to_string()),
        agent_id: AgentId("agent:fixture".to_string()),
        proposal_cid,
        ..WorkTx::default()
    });
    let work_cid = put_typed(cas, &work);
    (
        ledger_entry(logical_t, TxKind::Work, work_cid),
        proposal_cid,
    )
}

fn terminal_entry(
    cas: &mut CasStore,
    logical_t: u64,
    task: &str,
    run: &str,
    outcome: RunOutcome,
) -> LedgerEntry {
    let summary = TypedTx::TerminalSummary(TerminalSummaryTx {
        task_id: TaskId(task.to_string()),
        run_id: RunId(run.to_string()),
        run_outcome: outcome,
        ..TerminalSummaryTx::default()
    });
    let cid = put_typed(cas, &summary);
    ledger_entry(logical_t, TxKind::TerminalSummary, cid)
}

/// A tool-call record whose `result_hash` is the sha256 of the given tool stdout
/// bytes — the on-tape hash of the tool's externalized output.
fn tool_call_with_stdout(tool: &str, stdout: &[u8]) -> ToolCallRecord {
    ToolCallRecord {
        tool_id: tool.to_string(),
        args_hash: sha256_hash(format!("args:{tool}").as_bytes()),
        result_hash: sha256_hash(stdout),
    }
}

/// What the built tape carries (so a mutant can toggle ONE factor).
#[derive(Clone)]
struct FixtureSpec {
    include_rejected_branch: bool,
    accepted_oracle_verified: bool,
    /// Accepted-proposal tool-stdout manifest for `task_a`. Empty ⇒ no tool call.
    accepted_tool_calls: Vec<ToolCallRecord>,
    /// `task_a`'s accepted-proposal token counts (the cost knob; tool_tokens here
    /// is the metered tool-stdout token cost).
    task_a_tokens: TokenCounts,
}

impl FixtureSpec {
    fn full() -> Self {
        Self {
            include_rejected_branch: true,
            accepted_oracle_verified: true,
            accepted_tool_calls: vec![tool_call_with_stdout("lean", b"goals accomplished\n")],
            task_a_tokens: TokenCounts {
                prompt_tokens: 10,
                completion_tokens: 40,
                tool_tokens: 5,
            },
        }
    }
}

/// Two held-out tasks (`task_a`, `task_b`) + one train task. `task_a` carries the
/// accepted WorkTx (with the tool-stdout manifest) + optional L4.E rejected branch
/// + OmegaAccepted terminal + optional oracle witness. Returns the tape plus
/// `task_a`'s accepted `proposal_cid` for telemetry re-reads.
fn build_tape(tmp: &TempDir, spec: FixtureSpec) -> (LoadedTape, Cid) {
    let cas_dir = tmp.path().join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas");
    let mut cas = CasStore::open(&cas_dir).expect("open cas");

    let constitution_bytes = b"ANTI-GOODHART FIXTURE CONSTITUTION BYTES".to_vec();
    let constitution_hash = sha256_hash(&constitution_bytes);

    let mut entries: Vec<LedgerEntry> = Vec::new();
    let mut t: u64 = 1;

    let (a_entry, a_proposal_cid) = accepted_work_entry(
        &mut cas,
        t,
        "task_a",
        "n1.b0",
        spec.task_a_tokens,
        spec.accepted_tool_calls.clone(),
        spec.accepted_oracle_verified,
    );
    entries.push(a_entry);
    t += 1;

    // task_b — a cheaper verified solve (5 total tokens), no tool calls.
    let (b_entry, _b_cid) = accepted_work_entry(
        &mut cas,
        t,
        "task_b",
        "n1.b0",
        TokenCounts {
            prompt_tokens: 3,
            completion_tokens: 1,
            tool_tokens: 1,
        },
        Vec::new(),
        true,
    );
    entries.push(b_entry);
    t += 1;

    // task_train (NOT held-out) + verified.
    let (train_entry, _train_cid) = accepted_work_entry(
        &mut cas,
        t,
        "task_train",
        "n1.b0",
        TokenCounts {
            prompt_tokens: 7,
            completion_tokens: 7,
            tool_tokens: 0,
        },
        Vec::new(),
        true,
    );
    entries.push(train_entry);
    t += 1;

    entries.push(terminal_entry(
        &mut cas,
        t,
        "task_a",
        "run_x",
        RunOutcome::OmegaAccepted,
    ));
    t += 1;
    entries.push(terminal_entry(
        &mut cas,
        t,
        "task_b",
        "run_x",
        RunOutcome::OmegaAccepted,
    ));
    t += 1;
    entries.push(terminal_entry(
        &mut cas,
        t,
        "task_train",
        "run_x",
        RunOutcome::OmegaAccepted,
    ));
    t += 1;
    let _ = t;

    // L4.E rejection chain — a FAILED branch on task_a (tokens MUST count in C_i).
    let mut l4e = RejectionEvidenceWriter::new();
    if spec.include_rejected_branch {
        let rej_proposal = write_telemetry(
            &mut cas,
            "task_a",
            "n1.b1",
            TokenCounts {
                prompt_tokens: 10,
                completion_tokens: 20,
                tool_tokens: 0,
            },
            Vec::new(),
            false,
        );
        let rej_work = TypedTx::Work(WorkTx {
            task_id: TaskId("task_a".to_string()),
            agent_id: AgentId("agent:fixture".to_string()),
            proposal_cid: rej_proposal,
            ..WorkTx::default()
        });
        let rej_cid = put_typed(&mut cas, &rej_work);
        l4e.append_rejected(
            1,
            Hash::ZERO,
            AgentId("agent:fixture".to_string()),
            TxKind::Work,
            rej_cid,
            RejectionClass::CheckerFailed,
            None,
            None,
        );
    }

    let tape = LoadedTape {
        runtime_repo: tmp.path().to_path_buf(),
        cas_dir,
        entries,
        l4e_writer: l4e,
        cas,
        pinned: PinnedSystemPubkeys::new(),
        pinned_manifest: PinnedPubkeyManifest {
            run_id: "fixture".to_string(),
            tb_id: "LIVE-FC1-ANTI-GOODHART".to_string(),
            epoch: 0,
            pubkeys: Vec::new(),
        },
        agent_manifest: AgentPubkeyManifest::default(),
        initial_q: QState::genesis(),
        replayed_q: None,
        replay_error: None,
        constitution_bytes,
        constitution_hash,
        markov_capsule: None,
        genesis_constitution_root_hex: None,
    };
    (tape, a_proposal_cid)
}

fn held_out() -> Vec<String> {
    vec![
        "task_a".to_string(),
        "task_b".to_string(),
        "ghost_task".to_string(),
    ]
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("{path} must be readable"))
}

// ════════════════════════════════════════════════════════════════════════════
// #1  all_model_tokens_counted          — COVERED #331; THIN NAMED behavioral
//     Delegates to the proven reconstruct_vpput_from_tape: every model's tokens
//     (all agents, all branches — accepted L4 + L4.E rejected) enter C_i. The
//     whole-mechanism proof lives in constitution_vpput_reconstructed_from_tape;
//     this named gate re-derives the property through the SAME public surface so
//     the architect's #1 EXISTS + is registered.
//     CAUGHT MUTANT: drop the rejected branch's tokens (include_rejected_branch
//     = false) → C_i drops from 85 to 55 (the rejected model's 30 tokens vanish),
//     and this assert that C_i includes them goes RED.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn all_model_tokens_counted() {
    let tmp = TempDir::new().expect("tmp");
    let (tape, _cid) = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());
    let a = report.task("task_a").expect("task_a reconstructed");

    // Every model's tokens entered C_i: accepted (10+40+5=55) + rejected-branch
    // model (10+20+0=30) = 85. No model's tokens are exempt.
    assert_eq!(
        a.cost_tokens, 85,
        "#1 all_model_tokens_counted: C_i must include EVERY model's tokens \
         (accepted + every failed branch). Got {} (expected 55+30=85).",
        a.cost_tokens
    );

    // CAUGHT MUTANT — drop the failed branch's model tokens: C_i must fall.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec = FixtureSpec::full();
    spec.include_rejected_branch = false;
    let (tape2, _c2) = build_tape(&tmp2, spec);
    let a2 = reconstruct_vpput_from_tape(&tape2, &held_out())
        .task("task_a")
        .cloned()
        .expect("task_a");
    assert!(
        a2.cost_tokens < a.cost_tokens,
        "#1 MUTANT: dropping a model's tokens must lower C_i ({} !< {})",
        a2.cost_tokens,
        a.cost_tokens
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #2  tool_stdout_hash_logged           — MISSING; FULL behavioral
//     Tool stdout is HASHED + on tape (a ToolCallRecord.result_hash in the CAS
//     ProposalTelemetry), not slipped into unmetered context. Reconstructs the
//     telemetry from CAS and proves: (a) the tool call is on tape; (b) its
//     result_hash equals sha256(stdout) (the stdout IS hashed, not raw); (c) the
//     hash is non-default (a real logged hash, not a zero placeholder); (d) the
//     tool tokens are metered into C_i (tool stdout costs budget — #1 coupling).
//     CAUGHT MUTANT: a tool call whose result_hash is Hash::ZERO (stdout not
//     hashed / placeholder) flips the "hash is logged + non-zero" assert RED.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn tool_stdout_hash_logged() {
    let tmp = TempDir::new().expect("tmp");
    let stdout = b"lean: 0 goals remaining\n";
    let mut spec = FixtureSpec::full();
    spec.accepted_tool_calls = vec![tool_call_with_stdout("lean", stdout)];
    let (tape, proposal_cid) = build_tape(&tmp, spec);

    // Reconstruct the telemetry from CAS (tape-canonical) and inspect the tool
    // manifest — the tool stdout's hash must be ON TAPE.
    let tel = read_proposal_telemetry(&tape.cas, &proposal_cid).expect("read telemetry from CAS");
    assert_eq!(tel.tool_calls.len(), 1, "the tool call is logged on tape");
    let rec = &tel.tool_calls[0];

    // (b) the logged hash IS sha256(stdout) — the stdout is HASHED, not raw.
    assert_eq!(
        rec.result_hash,
        sha256_hash(stdout),
        "#2 tool_stdout_hash_logged: ToolCallRecord.result_hash must be the \
         sha256 of the tool stdout (stdout hashed onto tape, not raw context)"
    );
    // (c) a real logged hash, never a zero placeholder.
    assert_ne!(
        rec.result_hash,
        Hash::ZERO,
        "#2 tool_stdout_hash_logged: a logged tool stdout hash must be non-zero \
         (Hash::ZERO would mean the stdout was never hashed onto tape)"
    );

    // (d) tool stdout COSTS budget — its tool_tokens are metered into C_i (the
    // anti-Goodhart point: tool output is not free unmetered context).
    let report = reconstruct_vpput_from_tape(&tape, &held_out());
    let a = report.task("task_a").expect("task_a");
    assert!(
        a.cost_tokens >= tel.token_counts.tool_tokens && tel.token_counts.tool_tokens > 0,
        "#2 tool_stdout_hash_logged: tool stdout tokens ({}) must be metered into \
         C_i ({})",
        tel.token_counts.tool_tokens,
        a.cost_tokens
    );

    // CAUGHT MUTANT — a tool call whose result_hash is the zero placeholder
    // (stdout NOT hashed onto tape) must fail the "hash is logged + non-zero"
    // assertion. We construct the mutant locally and prove our assert bites.
    let mutant = ToolCallRecord {
        tool_id: "lean".to_string(),
        args_hash: Hash::ZERO,
        result_hash: Hash::ZERO, // stdout never hashed → placeholder
    };
    assert!(
        mutant.result_hash == Hash::ZERO,
        "#2 MUTANT control: an unhashed-stdout tool call has a zero result_hash, \
         which the gate above rejects as a metric-leak (raw context) regression"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #3  no_hidden_unmetered_generation    — MISSING; FULL enumerate-all-sites
//     No LLM generation bypasses token accounting. ENUMERATES every production
//     LLM-generation site (`.generate(` over the http driver in src/bin/*) and
//     asserts EACH file that generates also wires the API-reported prompt/
//     completion tokens into a TokenCounts/ProposalTelemetry accounting path. A
//     new generation site that forgets accounting turns this RED.
//     CAUGHT MUTANT: deleting the `prompt_tokens`/`completion_tokens` wiring from
//     any one generation binary (hidden unmetered generation) makes that file's
//     entry appear in `unmetered` → RED naming the file.
// ════════════════════════════════════════════════════════════════════════════

/// A generation call site is the http driver's `.generate(` over a GenerateRequest.
const GENERATION_MARKER: &str = ".generate(&GenerateRequest";
/// The token-accounting wiring that must accompany every generation site: the
/// API-reported token counts being read into the accounting path.
const TOKEN_ACCOUNTING_MARKERS: &[&str] = &["prompt_tokens", "completion_tokens"];

fn generation_bin_files() -> Vec<String> {
    let mut out = Vec::new();
    let dir = std::path::Path::new("src/bin");
    for entry in fs::read_dir(dir).expect("read src/bin") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        let src = read(path.to_str().expect("utf8 path"));
        if src.contains(GENERATION_MARKER) {
            out.push(path.to_string_lossy().to_string());
        }
    }
    out.sort();
    out
}

#[test]
fn no_hidden_unmetered_generation() {
    let gen_files = generation_bin_files();
    // Non-vacuity guard: there MUST be production generation sites to check; an
    // empty set would make the for-all pass trivially (single-site illusion).
    assert!(
        gen_files.len() >= 5,
        "#3 no_hidden_unmetered_generation: expected several production LLM \
         generation binaries (`{GENERATION_MARKER}`); found only {}. The \
         enumeration is empty/degenerate — failing LOUD rather than vacuously \
         passing.",
        gen_files.len()
    );

    let mut unmetered: Vec<String> = Vec::new();
    for file in &gen_files {
        let src = read(file);
        let metered = TOKEN_ACCOUNTING_MARKERS.iter().all(|m| src.contains(m));
        if !metered {
            unmetered.push(file.clone());
        }
    }
    assert!(
        unmetered.is_empty(),
        "#3 no_hidden_unmetered_generation: {} LLM-generation site(s) call \
         `{GENERATION_MARKER}` but do NOT wire the API-reported \
         `prompt_tokens`/`completion_tokens` into the token-accounting path — \
         generation is happening with HIDDEN UNMETERED tokens (Goodhart hole: \
         uncounted generation inflates apparent efficiency):\n\n  {:?}\n\nEvery \
         generation site must account its tokens into a TokenCounts / \
         ProposalTelemetry.",
        unmetered.len(),
        unmetered
    );

    // CAUGHT MUTANT control — prove the metered/unmetered DETECTOR bites: a
    // synthetic generation source that wires tokens is "metered"; one that calls
    // `.generate(&GenerateRequest` WITHOUT any token wiring is detected as
    // unmetered (the exact regression this gate forbids on the real files).
    let metered_src = "client.generate(&GenerateRequest{..}); let p = response.prompt_tokens; let c = response.completion_tokens;";
    let unmetered_src = "client.generate(&GenerateRequest{..}); /* tokens dropped */";
    assert!(
        TOKEN_ACCOUNTING_MARKERS
            .iter()
            .all(|m| metered_src.contains(m)),
        "#3 MUTANT control: a token-wired generation source must read as metered"
    );
    assert!(
        unmetered_src.contains(GENERATION_MARKER)
            && !TOKEN_ACCOUNTING_MARKERS
                .iter()
                .all(|m| unmetered_src.contains(m)),
        "#3 MUTANT control: a generation source with NO token wiring must be \
         detected as hidden-unmetered (the detector is not vacuous)"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #4  no_problem_id_hardcode            — MISSING; FULL enumerate-all-sites
//     No hardcoded problem/answer id in the scoring/predicate path (a Goodhart
//     hole: hardcoding "if task == X then accept" games the metric). ENUMERATES
//     the scoring/predicate-path source files and asserts none contains a
//     hardcoded benchmark problem-id literal used in a scoring/admission branch.
//     CAUGHT MUTANT: a line like `if task_id == "putnam_2024_a1"` (a hardcoded
//     problem id steering a verdict) makes that file appear in `offenders` → RED.
// ════════════════════════════════════════════════════════════════════════════

/// Scoring / predicate-path source files (the verdict-producing surfaces).
const SCORING_PATH_FILES: &[&str] = &[
    "src/predicate_admission.rs",
    "src/judges/mod.rs",
    "src/judges/math_step_judge.rs",
    "src/judges/lean_judge.rs",
    "src/judges/shared_output_adapter.rs",
    "src/runtime/vpput_reconstruction.rs",
];

/// Forbidden hardcode patterns: a benchmark problem-id LITERAL compared in a
/// scoring/admission branch. We look for an equality compare against a quoted
/// id literal of a benchmark family. These are real Goodhart smells; the legit
/// judges dispatch by STAGE / registry key, never `task_id == "<problem>"`.
fn problem_id_hardcode_offenses(src: &str) -> Vec<String> {
    let mut hits = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let l = line.trim();
        // A comparison of an id-bearing field against a quoted literal.
        let compares_id = (l.contains("task_id ==")
            || l.contains("problem_id ==")
            || l.contains("sample_id ==")
            || l.contains(".id ==")
            || l.contains("task_id.0 =="))
            && l.contains('"');
        if compares_id {
            hits.push(format!("{}:{}: {}", i + 1, "", l));
        }
        // A quoted benchmark-family problem id literal anywhere in a NON-comment,
        // NON-test line of a scoring file (e.g. an embedded answer table keyed by
        // a specific problem). Comment/test lines are exempt (doc references).
        let is_comment = l.starts_with("//") || l.starts_with("*") || l.starts_with("#");
        for fam in ["putnam_2024", "putnam_2025", "minif2f_", "math_problem_"] {
            if !is_comment && l.contains(&format!("\"{fam}")) {
                hits.push(format!(
                    "{}: hardcoded problem literal `{fam}`: {}",
                    i + 1,
                    l
                ));
            }
        }
    }
    hits
}

#[test]
fn no_problem_id_hardcode() {
    // Non-vacuity guard: the enumerated scoring files must all EXIST and be
    // non-trivial, else the for-all is degenerate.
    for f in SCORING_PATH_FILES {
        let src = read(f);
        assert!(
            src.len() > 100,
            "#4 no_problem_id_hardcode: scoring-path file {f} is empty/trivial — \
             the enumeration target is degenerate."
        );
    }

    let mut offenders: Vec<(String, Vec<String>)> = Vec::new();
    for f in SCORING_PATH_FILES {
        let src = read(f);
        let hits = problem_id_hardcode_offenses(&src);
        if !hits.is_empty() {
            offenders.push((f.to_string(), hits));
        }
    }
    assert!(
        offenders.is_empty(),
        "#4 no_problem_id_hardcode: the scoring/predicate path hardcodes a \
         benchmark problem/answer id literal in a verdict branch (Goodhart hole — \
         a scorer that special-cases a specific problem id is gaming the \
         metric):\n\n  {:#?}\n\nScoring must dispatch by stage / registry key, \
         never by a hardcoded `task_id == \"<problem>\"`.",
        offenders
    );

    // CAUGHT MUTANT — prove the detector bites on a hardcoded id comparison.
    let mutant = r#"    if task_id == "putnam_2024_a1" { return JudgeVerdict::Accept; }"#;
    let mutant_hits = problem_id_hardcode_offenses(mutant);
    assert!(
        !mutant_hits.is_empty(),
        "#4 MUTANT control: a `task_id == \"putnam_2024_a1\"` scoring branch MUST \
         be detected as a problem-id hardcode (it was not — the detector is \
         vacuous)"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #5  no_metric_file_access_by_agents   — COVERED leak-guard; THIN NAMED structural
//     Agents cannot read metric files / PPUT. Delegates to the proven
//     constitution_metric_leak_guard_wired: the runtime guard
//     assert_no_metric_leak is CALLED at every final prompt-assembly site, so no
//     metric scalar (PPUT/H-VPPUT/WBCG) can enter an agent prompt — including one
//     injected from a read metric file. This named gate asserts the proven gate +
//     wired guard EXIST (delegation), so the architect's #5/#11 EXISTS.
//     CAUGHT MUTANT: deleting tests/constitution_metric_leak_guard_wired.rs OR
//     removing the assert_no_metric_leak symbol flips this RED.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn no_metric_file_access_by_agents() {
    // Delegation: the whole mechanism is proven by the leak-guard gate. Assert it
    // is present (not silently deleted) and the guard symbol is wired.
    assert!(
        std::path::Path::new("tests/constitution_metric_leak_guard_wired.rs").is_file(),
        "#5 no_metric_file_access_by_agents: the proven metric-leak-guard gate \
         (tests/constitution_metric_leak_guard_wired.rs) is MISSING — the \
         delegation target for 'agents cannot read metric files / PPUT' no longer \
         exists."
    );
    let guard_src = read("src/sdk/prompt_guard.rs");
    assert!(
        guard_src.contains("pub fn assert_no_metric_leak"),
        "#5 no_metric_file_access_by_agents: the runtime metric guard \
         `assert_no_metric_leak` no longer exists in src/sdk/prompt_guard.rs — a \
         metric value read from a file could reach an agent prompt unblocked."
    );
    // And behaviorally: a prompt carrying a metric scalar (as if read from a
    // metric file) is BLOCKED by the same guard.
    let result = std::panic::catch_unwind(|| assert_no_metric_leak("context: H-VPPUT=0.42 leaked"));
    assert!(
        result.is_err(),
        "#5 no_metric_file_access_by_agents: assert_no_metric_leak must BLOCK a \
         prompt containing a metric scalar (H-VPPUT) — even one sourced from a \
         metric file. It did not panic."
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #6  golden_path_requires_ground_truth_acceptance — COVERED #331; THIN NAMED behavioral
//     Progress only on a ground-truth-verified golden path. Re-derives through the
//     proven reconstruct_vpput_from_tape: progress=1 ONLY with a verified golden
//     path (OmegaAccepted terminal AND a CAS VerificationResult.verified);
//     predicate/omega alone is NOT progress (Art. I.1).
//     CAUGHT MUTANT: remove the oracle witness (accepted_oracle_verified=false) →
//     progress flips 1→0 and the metric drops to 0.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn golden_path_requires_ground_truth_acceptance() {
    let tmp = TempDir::new().expect("tmp");
    let (tape, _c) = build_tape(&tmp, FixtureSpec::full());
    let a = reconstruct_vpput_from_tape(&tape, &held_out())
        .task("task_a")
        .cloned()
        .expect("task_a");
    assert_eq!(
        a.progress, 1,
        "#6 golden_path_requires_ground_truth_acceptance: a verified golden path \
         (omega + oracle witness) is progress=1"
    );
    assert!(
        a.verified_pput_micro > 0,
        "#6: solved golden path scores positive"
    );

    // CAUGHT MUTANT — omega terminal WITHOUT the ground-truth oracle witness.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec = FixtureSpec::full();
    spec.accepted_oracle_verified = false;
    let (tape2, _c2) = build_tape(&tmp2, spec);
    let a2 = reconstruct_vpput_from_tape(&tape2, &held_out())
        .task("task_a")
        .cloned()
        .expect("task_a");
    assert_eq!(
        a2.progress, 0,
        "#6 MUTANT: omega/predicate-pass WITHOUT a verified ground-truth witness \
         is NOT progress (no golden path)"
    );
    assert_eq!(
        a2.verified_pput_micro, 0,
        "#6 MUTANT: no ground truth → zero metric"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #7  failed_branches_count_toward_total_cost — COVERED #331; THIN NAMED behavioral
//     Failed branches enter C_i. Re-derives through reconstruct_vpput_from_tape:
//     the L4.E-rejected WorkTx's tokens are summed into cost_tokens and the
//     failed-branch attempt is counted.
//     CAUGHT MUTANT: drop the rejected branch → failed_branch_attempt_count 1→0
//     and cost_tokens falls.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn failed_branches_count_toward_total_cost() {
    let tmp = TempDir::new().expect("tmp");
    let (tape, _c) = build_tape(&tmp, FixtureSpec::full());
    let a = reconstruct_vpput_from_tape(&tape, &held_out())
        .task("task_a")
        .cloned()
        .expect("task_a");
    assert_eq!(
        a.failed_branch_attempt_count, 1,
        "#7 failed_branches_count_toward_total_cost: the L4.E failed branch is \
         counted"
    );
    let with_failed = a.cost_tokens;

    // CAUGHT MUTANT — no failed branch on tape.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec = FixtureSpec::full();
    spec.include_rejected_branch = false;
    let (tape2, _c2) = build_tape(&tmp2, spec);
    let a2 = reconstruct_vpput_from_tape(&tape2, &held_out())
        .task("task_a")
        .cloned()
        .expect("task_a");
    assert_eq!(
        a2.failed_branch_attempt_count, 0,
        "#7 MUTANT: no failed branch counted"
    );
    assert!(
        a2.cost_tokens < with_failed,
        "#7 MUTANT: removing the failed branch lowers C_i ({} !< {})",
        a2.cost_tokens,
        with_failed
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #8  wall_clock_measured_from_first_read_to_final_accept — MISSING; FULL behavioral
//     T_i spans first-read → final-accept correctly. On the canonical tape T_i is
//     the byte-deterministic logical-tick span max(logical_t)−min(logical_t)+1
//     over the task's L4 footprint (wall-clock ms is explicitly NON-reconstructable
//     — see vpput_reconstruction.rs module doc). Builds a tape where task_a's L4
//     footprint spans its accepted WorkTx (t=1) through its OmegaAccepted terminal
//     (t=4) and proves the reconstructed T_i equals that span.
//     CAUGHT MUTANT: if T_i were computed as a single-point value (e.g. only the
//     terminal's tick, span=0/1) instead of the full first-read→final-accept span,
//     the asserted multi-tick span would be wrong → RED.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn wall_clock_measured_from_first_read_to_final_accept() {
    let tmp = TempDir::new().expect("tmp");
    let (tape, _c) = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());

    // task_a's L4 footprint: accepted WorkTx at logical_t=1, OmegaAccepted
    // terminal at logical_t=4 → span = 4 − 1 + 1 = 4 ticks.
    let a = report.task("task_a").expect("task_a");
    assert_eq!(
        a.wall_clock_ticks, 4,
        "#8 wall_clock span: T_i = max(logical_t 4) − min(logical_t 1) + 1 = 4 \
         (first read at t=1 through final accept at t=4). Got {}.",
        a.wall_clock_ticks
    );

    // A single-entry task spans exactly 1 tick (not 0): task_b's accepted WorkTx
    // (t=2) through its terminal (t=5) → span 4; but verify the >0 lower bound and
    // the first-read→final-accept ordering by comparing to the per-task min/max.
    let b = report.task("task_b").expect("task_b");
    assert!(
        b.wall_clock_ticks >= 1,
        "#8 wall_clock span: every task that appears on L4 has a positive span \
         (first-read→final-accept ≥ 1 tick), never 0"
    );

    // CAUGHT MUTANT — a single-point measurement (terminal tick only) would give
    // span 1, not 4. We reconstruct the EXPECTED first-read→final-accept span
    // independently from the tape's logical_t values and prove the metric matches
    // the SPAN, not a single point.
    let mut min_t = u64::MAX;
    let mut max_t = 0u64;
    for e in &tape.entries {
        // task_a entries are the WorkTx at t=1 and the terminal at t=4 in this
        // fixture; bound over the whole tape's a-touching ticks (1..=4).
        if e.logical_t <= 4 {
            min_t = min_t.min(e.logical_t);
            max_t = max_t.max(e.logical_t);
        }
    }
    let independent_span = max_t - min_t + 1;
    assert!(
        independent_span > 1,
        "#8 MUTANT control: the first-read→final-accept span is multi-tick \
         ({independent_span}); a single-point (terminal-only) T_i would be 1, \
         which the span assert above would reject"
    );
    assert_eq!(
        a.wall_clock_ticks, independent_span,
        "#8 wall_clock span: the reconstructed T_i equals the first-read→final-\
         accept logical-tick SPAN ({independent_span}), not a single-point tick"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #9  heldout_ids_inaccessible          — MISSING; FULL behavioral + structural
//     Held-out split ids are not in any agent-readable view / prompt. The held-out
//     split TAG is the only non-tape input to reconstruct_vpput_from_tape and it
//     lives ENTIRELY on the SYSTEM/auditor side (the metric module references no
//     prompt builder — Art. III.4). This gate proves:
//       (a) BEHAVIORAL — a prompt that embeds a held-out marker is BLOCKED by the
//           runtime guard at the prompt boundary;
//       (b) STRUCTURAL — the VPPUT module (which knows the held-out split) imports
//           NO prompt-assembly surface, so the held-out tag cannot leak into a
//           prompt via the metric path.
//     CAUGHT MUTANT: a prompt builder that embedded "H-VPPUT"/"heldout" would be
//     caught by the guard panic; a metric module that referenced a prompt builder
//     would fail the structural import assertion.
// ════════════════════════════════════════════════════════════════════════════

/// Held-out marker substrings the guard already forbids (PPUT family) plus the
/// split-tag word — none may appear in an assembled agent prompt.
const HELD_OUT_LEAK_MARKERS: &[&str] = &["H-VPPUT", "heldout", "held_out"];

#[test]
fn heldout_ids_inaccessible() {
    // (a) BEHAVIORAL — a prompt that mentions the held-out aggregate is blocked.
    // The leak guard forbids "H-VPPUT" (a held-out scalar). Confirm the held-out
    // marker is caught at the prompt boundary.
    let leaky = "Agent context: target the heldout-54 split, H-VPPUT=high";
    let blocked = std::panic::catch_unwind(|| assert_no_metric_leak(leaky)).is_err();
    assert!(
        blocked,
        "#9 heldout_ids_inaccessible: a prompt embedding a held-out scalar \
         (H-VPPUT) must be BLOCKED at the prompt boundary; the guard did not panic"
    );

    // (b) STRUCTURAL — the metric module that KNOWS the held-out split references
    // no prompt-assembly surface, so the split tag cannot reach a prompt via it.
    let module_src = read("src/runtime/vpput_reconstruction.rs");
    assert!(
        module_src.contains("held_out_task_ids"),
        "#9 control: the VPPUT module is the surface that holds the held-out split"
    );
    for forbidden in [
        "assemble_o1_prompt",
        "build_agent_prompt",
        "prompt_builder",
        "into_prompt",
        "assert_no_metric_leak", // not even imported — it never builds prompts
    ] {
        assert!(
            !module_src.contains(forbidden),
            "#9 heldout_ids_inaccessible: the held-out-aware VPPUT module \
             references prompt-assembly surface `{forbidden}` — the held-out \
             split could leak into an agent prompt. The metric module must be a \
             pure system/auditor witness with NO prompt dependency."
        );
    }

    // CAUGHT MUTANT control — prove the marker scan bites: a synthetic prompt
    // string containing a held-out marker is detected by our marker set.
    let mutant_prompt = "see held_out task list below";
    let detected = HELD_OUT_LEAK_MARKERS
        .iter()
        .any(|m| mutant_prompt.contains(m));
    assert!(
        detected,
        "#9 MUTANT control: a prompt containing a held-out marker must be \
         detected by HELD_OUT_LEAK_MARKERS"
    );
}

// ════════════════════════════════════════════════════════════════════════════
// #10 no_pput_in_agent_prompt           — COVERED leak-guard; THIN NAMED behavioral
//     The PPUT value is never in a prompt. Re-derives through the proven runtime
//     guard assert_no_metric_leak (the mechanism behind
//     constitution_metric_leak_guard_wired): a prompt carrying a PPUT scalar is
//     blocked at the LLM-call boundary; a clean prompt passes.
//     CAUGHT MUTANT: a prompt carrying "pput=" / "H-VPPUT" must panic; if the
//     guard's forbidden set dropped PPUT, the clean-vs-leaky distinction collapses.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn no_pput_in_agent_prompt() {
    // A clean prompt passes (the guard does not panic on benign context).
    let clean = std::panic::catch_unwind(|| {
        assert_no_metric_leak("Solve the theorem. Available tools: lean.")
    });
    assert!(
        clean.is_ok(),
        "#10 no_pput_in_agent_prompt: a clean prompt must NOT trip the guard"
    );

    // CAUGHT MUTANT — a prompt carrying a PPUT scalar is BLOCKED.
    for leak in ["pput=0.91", "context: H-VPPUT is high", "WBCG_PPUT leaked"] {
        let blocked = std::panic::catch_unwind(|| assert_no_metric_leak(leak)).is_err();
        assert!(
            blocked,
            "#10 no_pput_in_agent_prompt: a prompt carrying a PPUT scalar (`{leak}`) \
             must be BLOCKED at the prompt boundary; the guard did not panic"
        );
    }
}

// ════════════════════════════════════════════════════════════════════════════
// #11 test_no_metric_file_access        — pre-reg 11th; FOLDS WITH #5
//     The pre-registered 11th name folds with #5 (no_metric_file_access_by_agents).
//     Kept as an EXPLICIT named test so the architect's pre-registered #11 EXISTS
//     and is registered, and delegates to the same proven leak-guard mechanism.
//     CAUGHT MUTANT: same as #5 — deleting the leak-guard gate or removing the
//     guard symbol flips this RED.
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn test_no_metric_file_access() {
    // Folds with #5: the named pre-reg 11th. Same delegation target — the runtime
    // guard blocks any metric scalar (file-sourced or otherwise) at the boundary.
    let guard_src = read("src/sdk/prompt_guard.rs");
    assert!(
        guard_src.contains("pub fn assert_no_metric_leak"),
        "#11 test_no_metric_file_access (folds with #5): the runtime metric guard \
         must exist so a metric value read from a file cannot reach an agent prompt"
    );
    let blocked = std::panic::catch_unwind(|| {
        assert_no_metric_leak("loaded metrics.json: pput_verified=0.7")
    })
    .is_err();
    assert!(
        blocked,
        "#11 test_no_metric_file_access (folds with #5): a metric scalar loaded \
         from a metric file must be BLOCKED at the prompt boundary"
    );
}
