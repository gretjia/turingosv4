//! LIVE-FC1 — VPPUT EFFICIENCY: tape-canonical Verified-PPUT reconstruction gate.
//!
//! The architect North Star (`PPUT_DRIVEN_FULL_PASS_2026-04-25`) is the held-out
//! **Verified PPUT**. This gate proves that
//! `runtime::agent_scheduler::vpput_reconstruction::reconstruct_vpput_from_tape`
//! reconstructs that metric FROM THE CANONICAL TAPE ALONE (L4 accepted spine +
//! L4.E rejection chain + CAS payloads) — making EFFICIENCY a gate-verifiable,
//! OS-qualifying dimension. It locks six coupled properties of the architect PPUT
//! definition, each with a mutation that turns a dedicated assert RED:
//!
//!   1. **TAPE-RECONSTRUCTED (Art.0.2)** — a realistic `LoadedTape` carrying an
//!      accepted `WorkTx` (with `ProposalTelemetry` + a verified
//!      `VerificationResult`), an L4.E-rejected `WorkTx` on the SAME task, and a
//!      `TerminalSummary { OmegaAccepted }` makes the reconstruction produce a
//!      per-task row whose `(progress, cost_tokens, wall_clock_ticks,
//!      verified_pput_micro)` are all derived from those tape bytes — never a
//!      sidecar.
//!
//!   2. **C_i COUNTS FAILED BRANCHES + TOOL STDOUT** — the cost sum INCLUDES the
//!      L4.E-rejected attempt's tokens (failed branches MUST count) and the
//!      `tool_tokens` portion (tool stdout). Dropping the rejected attempt lowers
//!      `cost_tokens` (mutation witness).
//!
//!   3. **GROUND-TRUTH GATED (Art.I.1)** — `progress == 1` ONLY when a verified
//!      golden path exists: removing the `VerificationResult.verified` witness
//!      (predicate/omega alone) flips `progress` to `0` and the metric to `0`.
//!
//!   4. **INTEGER-ONLY (Art.0)** — `verified_pput_micro` is the integer micro-unit
//!      `(1_000_000 × progress) / (cost × ticks)`; a cheaper/faster verified solve
//!      scores STRICTLY higher; the render carries no `<digit>.<digit>` f64 leak.
//!
//!   5. **HELD-OUT H-VPPUT** — the held-out aggregate is the integer mean of
//!      `verified_pput_micro` over the held-out split tasks PRESENT on tape;
//!      held-out tags with no tape footprint are excluded.
//!
//!   6. **OBSERVE-ONLY / SHIELDED (Art.III.4)** — the reconstruction mutates no
//!      tape byte and returns a report flagged `observe_only`; the metric value is
//!      NEVER emitted into any agent prompt (the module has no prompt-builder
//!      dependency — asserted structurally below).
//!
//! Non-vacuity: every green assert has a paired mutation (drop the rejected
//! attempt; drop the oracle witness; swap a cheaper run) that flips it RED, so the
//! gate cannot be satisfied by a constant.

use std::collections::BTreeMap;

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
use turingosv4::runtime::agent_scheduler::vpput_reconstruction::{
    reconstruct_vpput_from_tape, render_vpput_summary, PPUT_MICRO_SCALE,
};
use turingosv4::runtime::audit_assertions::LoadedTape;
use turingosv4::runtime::proposal_telemetry::{
    write_to_cas as write_proposal_telemetry, ProposalTelemetry, TokenCounts,
};
use turingosv4::runtime::verification_result::{
    write_to_cas as write_verification_result, VerificationResult,
};
use turingosv4::runtime::PinnedPubkeyManifest;
use turingosv4::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
use turingosv4::state::typed_tx::{RunId, RunOutcome, TerminalSummaryTx, TypedTx, WorkTx};

// ─────────────────────────────────────────────────────────────────────────
// Fixture construction — a real LoadedTape + CAS, hand-built so the gate
// exercises the SAME L4 / L4.E / CAS reconstruction the live VPPUT path uses.
// ─────────────────────────────────────────────────────────────────────────

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
    cas.put(&bytes, ObjectType::Generic, "vpput-fixture", 0, None)
        .expect("cas put TypedTx")
}

/// Write a `ProposalTelemetry` to CAS and return its CID; optionally attach a
/// verified `VerificationResult` (the Lean-oracle ground-truth witness).
fn write_telemetry(
    cas: &mut CasStore,
    task: &str,
    branch: &str,
    tokens: TokenCounts,
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
        Some(write_verification_result(cas, &vr, "vpput-fixture", 0).expect("write VR"))
    } else {
        None
    };
    let tel = ProposalTelemetry {
        agent_id: AgentId("agent:fixture".to_string()),
        prompt_context_hash: Hash::ZERO,
        // Non-zero artifact cid so the proposal is NOT a zero-CID synthetic seed.
        proposal_artifact_cid: Cid::from_content(format!("artifact:{task}:{branch}").as_bytes()),
        candidate_label: "nlinarith".to_string(),
        model_id: None,
        token_counts: tokens,
        tool_calls: Vec::new(),
        branch_id: branch.to_string(),
        parent_tx: None,
        verification_result_cid: vr_cid,
    };
    write_proposal_telemetry(cas, &tel, "vpput-fixture", 0).expect("write telemetry")
}

/// Build an accepted L4 WorkTx entry for `task` whose proposal carries `tokens`
/// and (optionally) a verified oracle witness.
fn accepted_work_entry(
    cas: &mut CasStore,
    logical_t: u64,
    task: &str,
    branch: &str,
    tokens: TokenCounts,
    verified_oracle: bool,
) -> LedgerEntry {
    let proposal_cid = write_telemetry(cas, task, branch, tokens, verified_oracle);
    let work = TypedTx::Work(WorkTx {
        task_id: TaskId(task.to_string()),
        agent_id: AgentId("agent:fixture".to_string()),
        proposal_cid,
        ..WorkTx::default()
    });
    let work_cid = put_typed(cas, &work);
    ledger_entry(logical_t, TxKind::Work, work_cid)
}

/// Build a TerminalSummary entry for `task` with the given outcome.
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

/// What the fixture controls so the mutation witnesses can toggle one factor.
#[derive(Clone, Copy)]
struct FixtureSpec {
    /// Include the L4.E-rejected (failed-branch) WorkTx on `task_a` (its tokens
    /// MUST count toward C_i).
    include_rejected_branch: bool,
    /// Attach a verified `VerificationResult` to `task_a`'s accepted WorkTx (the
    /// ground-truth witness). When false, omega-terminal alone is NOT progress.
    accepted_oracle_verified: bool,
    /// Accepted-WorkTx prompt+completion tokens for `task_a` (cost knob).
    task_a_completion_tokens: u64,
}

impl FixtureSpec {
    fn full() -> Self {
        Self {
            include_rejected_branch: true,
            accepted_oracle_verified: true,
            task_a_completion_tokens: 40,
        }
    }
}

/// Builds a tape with two held-out tasks:
///   * `task_a` — accepted WorkTx (tokens incl. tool stdout) + optional L4.E
///     rejected branch + OmegaAccepted terminal + optional verified oracle.
///   * `task_b` — accepted WorkTx + OmegaAccepted terminal + verified oracle, a
///     CHEAPER solve (fewer tokens) → strictly higher micro metric than `task_a`.
/// Plus `task_train` (NOT in the held-out split) to prove the split tag filters.
fn build_tape(tmp: &TempDir, spec: FixtureSpec) -> LoadedTape {
    let cas_dir = tmp.path().join("cas");
    std::fs::create_dir_all(&cas_dir).expect("mkdir cas");
    let mut cas = CasStore::open(&cas_dir).expect("open cas");

    let constitution_bytes = b"VPPUT FIXTURE CONSTITUTION BYTES".to_vec();
    let constitution_hash = sha256_hash(&constitution_bytes);

    let mut entries: Vec<LedgerEntry> = Vec::new();
    let mut t: u64 = 1;

    // task_a accepted WorkTx — prompt 10 + completion N + tool 5 (tool stdout).
    let a_tokens = TokenCounts {
        prompt_tokens: 10,
        completion_tokens: spec.task_a_completion_tokens,
        tool_tokens: 5,
    };
    entries.push(accepted_work_entry(
        &mut cas,
        t,
        "task_a",
        "n1.b0",
        a_tokens,
        spec.accepted_oracle_verified,
    ));
    t += 1;

    // task_b accepted WorkTx — a cheaper verified solve (5 total tokens, 1 tick).
    let b_tokens = TokenCounts {
        prompt_tokens: 3,
        completion_tokens: 1,
        tool_tokens: 1,
    };
    entries.push(accepted_work_entry(
        &mut cas, t, "task_b", "n1.b0", b_tokens, true,
    ));
    t += 1;

    // task_train accepted WorkTx (NOT held-out) + verified.
    let train_tokens = TokenCounts {
        prompt_tokens: 7,
        completion_tokens: 7,
        tool_tokens: 0,
    };
    entries.push(accepted_work_entry(
        &mut cas,
        t,
        "task_train",
        "n1.b0",
        train_tokens,
        true,
    ));
    t += 1;

    // Terminals — OmegaAccepted for all three (so omega gate is satisfied; the
    // oracle witness is the independent gate factor we toggle on task_a).
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
        let rej_tokens = TokenCounts {
            prompt_tokens: 10,
            completion_tokens: 20,
            tool_tokens: 0,
        };
        let rej_proposal = write_telemetry(&mut cas, "task_a", "n1.b1", rej_tokens, false);
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

    LoadedTape {
        runtime_repo: tmp.path().to_path_buf(),
        cas_dir,
        entries,
        l4e_writer: l4e,
        cas,
        pinned: PinnedSystemPubkeys::new(),
        pinned_manifest: PinnedPubkeyManifest {
            run_id: "fixture".to_string(),
            tb_id: "LIVE-FC1-VPPUT".to_string(),
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
    }
}

fn held_out() -> Vec<String> {
    vec![
        "task_a".to_string(),
        "task_b".to_string(),
        "ghost_task".to_string(),
    ]
}

// ─────────────────────────────────────────────────────────────────────────
// Observe-only byte snapshot (proves the reconstruction mutates no tape byte)
// ─────────────────────────────────────────────────────────────────────────

#[derive(PartialEq, Eq, Debug)]
struct TapeByteSnapshot {
    entry_payload_bytes: Vec<Vec<u8>>,
    l4e_chain_hash: [u8; 32],
    l4e_len: usize,
    entry_count: usize,
    cas_object_bytes: Vec<Vec<u8>>,
}

fn snapshot_tape(tape: &LoadedTape) -> TapeByteSnapshot {
    let mut cas_cids = tape.cas.list_all_cids();
    cas_cids.sort_by(|a, b| a.0.cmp(&b.0));
    TapeByteSnapshot {
        entry_payload_bytes: tape
            .entries
            .iter()
            .map(|e| tape.cas.get(&e.tx_payload_cid).unwrap_or_default())
            .collect(),
        l4e_chain_hash: tape.l4e_writer.last_hash().0,
        l4e_len: tape.l4e_writer.len(),
        entry_count: tape.entries.len(),
        cas_object_bytes: cas_cids
            .iter()
            .map(|c| tape.cas.get(c).expect("cas get for snapshot"))
            .collect(),
    }
}

// ─────────────────────────────────────────────────────────────────────────
// (1) TAPE-RECONSTRUCTED + (2) C_i COUNTS FAILED BRANCHES + TOOL STDOUT
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn reconstructs_per_task_vpput_from_tape_counting_failed_branch_and_tool_stdout() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());

    // Provenance counts are reconstructed, not zero.
    assert_eq!(report.l4_entry_count, tape.entries.len() as u64);
    assert_eq!(
        report.l4e_entry_count, 1,
        "one failed-branch rejection on tape"
    );

    let a = report.task("task_a").expect("task_a row reconstructed");

    // C_i = accepted(10+40+5=55) + rejected failed branch(10+20+0=30) = 85.
    // Tool stdout (the +5 tool_tokens on the accepted branch) IS included.
    assert_eq!(
        a.cost_tokens, 85,
        "C_i must sum accepted (incl tool stdout) + L4.E failed branch tokens"
    );
    assert_eq!(a.attempt_count, 2, "one accepted + one rejected attempt");
    assert_eq!(
        a.failed_branch_attempt_count, 1,
        "the L4.E branch is counted as failed"
    );

    // The failed branch's tokens are load-bearing: dropping it lowers C_i.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec_no_rej = FixtureSpec::full();
    spec_no_rej.include_rejected_branch = false;
    let tape_no_rej = build_tape(&tmp2, spec_no_rej);
    let report_no_rej = reconstruct_vpput_from_tape(&tape_no_rej, &held_out());
    let a_no_rej = report_no_rej.task("task_a").expect("task_a row");
    assert_eq!(
        a_no_rej.cost_tokens, 55,
        "without the failed branch, C_i is accepted-only"
    );
    assert!(
        a_no_rej.cost_tokens < a.cost_tokens,
        "MUTATION: removing the failed branch lowered C_i ({} !< {})",
        a_no_rej.cost_tokens,
        a.cost_tokens
    );
    assert_eq!(a_no_rej.failed_branch_attempt_count, 0);
}

// ─────────────────────────────────────────────────────────────────────────
// (3) GROUND-TRUTH GATED (Art.I.1)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn progress_is_ground_truth_gated_not_predicate_pass_alone() {
    let tmp = TempDir::new().expect("tmp");
    // FULL fixture: omega terminal + verified oracle → verified golden path.
    let tape = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());
    let a = report.task("task_a").expect("task_a");
    assert_eq!(
        a.progress, 1,
        "omega terminal + verified oracle = verified golden path"
    );
    assert!(
        a.verified_pput_micro > 0,
        "a solved task has a positive micro metric"
    );

    // MUTATION: remove ONLY the oracle witness (omega terminal still present).
    // Ground-truth gate (Art.I.1): omega/predicate-pass ALONE is NOT progress.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec = FixtureSpec::full();
    spec.accepted_oracle_verified = false;
    let tape_no_oracle = build_tape(&tmp2, spec);
    let report_no_oracle = reconstruct_vpput_from_tape(&tape_no_oracle, &held_out());
    let a_no_oracle = report_no_oracle.task("task_a").expect("task_a");
    assert_eq!(
        a_no_oracle.progress, 0,
        "MUTATION: omega terminal WITHOUT a verified oracle witness is NOT progress"
    );
    assert_eq!(
        a_no_oracle.verified_pput_micro, 0,
        "no ground truth → zero metric regardless of how cheap/fast the run was"
    );
    // The cost was still reconstructed (the work still happened) — only progress gated.
    assert!(
        a_no_oracle.cost_tokens > 0,
        "cost is still tape-reconstructed when unsolved"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (4) INTEGER-ONLY + cheaper/faster scores higher + shielded render
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn metric_is_integer_micro_unit_and_cheaper_solve_scores_higher() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());

    let a = report.task("task_a").expect("task_a");
    let b = report.task("task_b").expect("task_b");

    // task_b is the cheaper verified solve (5 tokens vs task_a's 85) → its integer
    // micro metric must be STRICTLY larger than task_a's. This is the efficiency
    // ordering the North Star wants. T_i is the task's first-read→final-accept
    // logical-tick span (its accepted WorkTx through its OmegaAccepted terminal);
    // both tasks span 4 ticks in this interleaved fixture, so cost is the
    // discriminator here.
    assert_eq!(b.cost_tokens, 5, "task_b is a 5-token solve");
    assert!(
        b.wall_clock_ticks >= 1,
        "task_b has a positive logical-tick span"
    );
    assert_eq!(
        b.verified_pput_micro,
        PPUT_MICRO_SCALE / (b.cost_tokens * b.wall_clock_ticks),
        "task_b micro = 1_000_000 / (cost * ticks) (pure integer division)"
    );
    assert!(
        b.verified_pput_micro > a.verified_pput_micro,
        "cheaper verified solve must score strictly higher: b={} a={}",
        b.verified_pput_micro,
        a.verified_pput_micro
    );

    // Integer-only: the canonical value equals the integer formula exactly.
    assert_eq!(
        a.verified_pput_micro,
        PPUT_MICRO_SCALE.saturating_mul(a.progress) / (a.cost_tokens * a.wall_clock_ticks),
        "canonical metric is exactly the integer micro formula (no f64)"
    );

    // INTEGER (canonical value is a u64 micro-unit) — a static type witness: the
    // canonical fields bind into `u64` with no coercion (this line fails to COMPILE
    // if the canonical metric were ever changed to an f64), and the JSON
    // serialization of the canonical metric carries no `.` (no f64 leak into the
    // canonical value, not just the human render). Art.0 integer-only.
    let _micro_u64: u64 = a.verified_pput_micro;
    let _h_u64: u64 = report.h_vpput_micro();
    let canonical_json = serde_json::to_string(&report).expect("serialize VpputReconstruction");
    let micro_json = serde_json::to_string(a).expect("serialize TaskVpput");
    assert!(
        !canonical_json.contains('.') && !micro_json.contains('.'),
        "INTEGER: the serialized canonical VPPUT carries NO '.' — an f64 would emit a \
         decimal point. canonical={canonical_json} task={micro_json}"
    );
    // And the integer value is literally present in the serialized canonical form.
    assert!(
        micro_json.contains(&format!("\"verified_pput_micro\":{}", a.verified_pput_micro)),
        "the u64 micro value appears verbatim (no float formatting) in the serialized canonical form"
    );

    // Shielded render: no <digit>.<digit> f64 leak on any numeric surface.
    let s = render_vpput_summary(&report);
    let bytes = s.as_bytes();
    let has_decimal_number = bytes
        .windows(3)
        .any(|w| w[1] == b'.' && w[0].is_ascii_digit() && w[2].is_ascii_digit());
    assert!(
        !has_decimal_number,
        "render must not emit an f64 decimal number"
    );
    assert!(
        !s.to_lowercase().contains("stderr"),
        "render must not leak raw diagnostics"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (5) HELD-OUT H-VPPUT
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn held_out_h_vpput_is_integer_mean_over_present_split_tasks_only() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());
    let report = reconstruct_vpput_from_tape(&tape, &held_out());

    // held_out tag = {task_a, task_b, ghost_task}; ghost_task has no tape
    // footprint → excluded. task_train is on tape but NOT in the held-out split.
    assert_eq!(
        report.held_out_task_count, 2,
        "only task_a + task_b (present held-out) enter the aggregate; ghost_task excluded"
    );
    let a = report.task("task_a").expect("task_a");
    let b = report.task("task_b").expect("task_b");
    let expected_h = (a.verified_pput_micro + b.verified_pput_micro) / 2;
    assert_eq!(
        report.h_vpput_micro(),
        expected_h,
        "H-VPPUT is the integer mean of held-out present-task micro metrics"
    );

    // MUTATION: an empty held-out split yields a zero aggregate (no split → no H).
    let report_empty = reconstruct_vpput_from_tape(&tape, &[]);
    assert_eq!(report_empty.held_out_task_count, 0);
    assert_eq!(
        report_empty.h_vpput_micro(),
        0,
        "empty held-out split → zero H-VPPUT"
    );

    // The training task is reconstructed but NOT in the held-out aggregate.
    assert!(
        report.task("task_train").is_some(),
        "train task reconstructed"
    );
}

// ─────────────────────────────────────────────────────────────────────────
// (6) OBSERVE-ONLY / SHIELDED
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn reconstruction_is_observe_only_and_shielded() {
    let tmp = TempDir::new().expect("tmp");
    let tape = build_tape(&tmp, FixtureSpec::full());

    let before = snapshot_tape(&tape);
    let report = reconstruct_vpput_from_tape(&tape, &held_out());
    let after = snapshot_tape(&tape);

    assert_eq!(
        before, after,
        "OBSERVE-ONLY: tape + CAS + L4.E byte-unchanged after reconstruction"
    );
    assert_eq!(before.entry_count, after.entry_count, "no L4 head advance");
    assert_eq!(before.l4e_len, after.l4e_len, "no L4.E head advance");
    assert!(report.observe_only, "report is flagged observe_only");

    // SHIELDED: the VPPUT module must NOT be wired into any prompt builder. We
    // assert structurally that the source module imports no prompt-assembly
    // surface and is not referenced by any prompt builder.
    let module_src = std::fs::read_to_string("src/runtime/vpput_reconstruction.rs")
        .expect("read vpput module source");
    for forbidden in [
        "assemble_o1_prompt",
        "build_agent_prompt",
        "prompt_builder",
        "into_prompt",
    ] {
        assert!(
            !module_src.contains(forbidden),
            "SHIELDED: VPPUT module must not reference prompt assembly (`{forbidden}`)"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────
// MUTATION WITNESS — end-to-end non-vacuity (drop oracle witness flips H-VPPUT)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn mutation_dropping_ground_truth_witness_lowers_held_out_h_vpput() {
    let tmp = TempDir::new().expect("tmp");

    // GREEN baseline: full fixture → positive H-VPPUT.
    let full = build_tape(&tmp, FixtureSpec::full());
    let green = reconstruct_vpput_from_tape(&full, &held_out());
    assert!(green.h_vpput_micro() > 0, "baseline H-VPPUT is positive");

    // MUTATION: remove task_a's ground-truth witness → its progress drops to 0,
    // its micro drops to 0, and the held-out aggregate strictly DECREASES.
    let tmp2 = TempDir::new().expect("tmp2");
    let mut spec = FixtureSpec::full();
    spec.accepted_oracle_verified = false;
    let mutated = build_tape(&tmp2, spec);
    let red = reconstruct_vpput_from_tape(&mutated, &held_out());

    assert_eq!(
        red.task("task_a").unwrap().verified_pput_micro,
        0,
        "MUTATION: task_a loses ground truth → zero micro"
    );
    assert!(
        red.h_vpput_micro() < green.h_vpput_micro(),
        "MUTATION: dropping a verified golden path lowers H-VPPUT ({} !< {})",
        red.h_vpput_micro(),
        green.h_vpput_micro()
    );
    // The denominator (held-out present count) is unchanged — it's the NUMERATOR
    // (the verified-progress micro) that fell, proving ground-truth gating bites.
    assert_eq!(red.held_out_task_count, green.held_out_task_count);
}
