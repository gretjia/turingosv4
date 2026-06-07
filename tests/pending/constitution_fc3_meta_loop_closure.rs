//! PENDING GATE (G5 / M07) — FC3 meta-architecture loop actually closes.
//!
//! STATUS: **STANDING PENDING** — gated on a USER §8 Class-4 RATIFICATION, not
//! merely on the M07 single-admission work:
//!
//!   PROMOTION REQUIRES §8 CLASS-4 RATIFICATION of the FC3 IRREVERSIBLE-COMMIT
//!   path. Closing FC3 means a runtime ArchitectAI proposal, after a Veto-AI
//!   PASS, leads to a tape-visible RE-INIT that recomputes the boot Trust Root /
//!   advances the constitution-bound hash. That touches RootBox / boot trust-
//!   root authority and the constitution-amendment boundary (Art. V.1.1: the
//!   constitution is the sole ground truth; only an explicit human sudo may
//!   amend it). It is therefore a Class-4 surface and MUST NOT be wired without
//!   per-atom §8 sign-off. Until that ratification, this gate stays RED by
//!   design (standing pending), and is NOT auto-promoted to a `constitution_*`
//!   gate.
//!
//! ── WHAT THIS GATE PROVES ────────────────────────────────────────────────
//! FC3 (constitution.md ~line 826; TRACE_MATRIX FC3-N3x/N4x) is the meta loop:
//!     logs+constitution -> archived feedback -> ArchitectAI proposal ->
//!     Veto-AI verdict -> tools/logs -> re-init.
//! The SUBSTRATE is live: `LogFeedbackArchiveTx`, `ArchitectProposalTx`,
//! `VetoDecisionTx`, `ArchitectCommitTx`, `ReinitRequestTx`, `ReinitBootTx`
//! exist with sequencer transition arms and deterministic Veto-AI verdict
//! checks. But the RUNTIME ENGINE that actually CLOSES the loop is missing:
//!   * The runtime role payloads are inert shells — `ToolProposalPayload` and
//!     `VetoPayload` are `{ proposal_id: Option<TxId> }` and are constructed via
//!     `::default()` (proposal_id == None) on the live role path
//!     (`src/runtime/real5_roles.rs`), carrying NO real spec/patch.
//!   * The terminal status of an ACCEPTED ArchitectAI proposal is
//!     `proposal_activation_status(..) == "sandbox:canary_only"`
//!     (`real5_roles.rs:1089`). There is NO status that closes the loop — no
//!     re-init committed, no trust-root recompute, no constitution-bound hash
//!     advance. The FC3 governance binary (`fc3_governance_reinit_current_kernel.rs`)
//!     stamps the SAME `constitution_source_hash()` at every stage (feedback,
//!     proposal, veto, commit, reinit) — the constitution never actually changes
//!     through the loop, so the "re-init" is not an irreversible meta-commit.
//!
//! ── HOW THE GATE OBSERVES IT (public API only) ───────────────────────────
//! Two public-API observations, both RED today:
//!   (A) PROPOSER CARRIES A REAL SPEC: the live role-path proposal payload must
//!       carry a concrete proposal id (a real spec reference), not the empty
//!       `::default()` shell. We build the shell exactly as the live path does
//!       (`ToolProposalPayload::default()`) and assert it is non-empty. RED:
//!       `proposal_id == None`.
//!   (B) LOOP CLOSES PAST CANARY: a Veto-AI ACCEPT on a real proposal must reach
//!       a terminal status that CLOSES FC3 (an irreversible re-init / committed
//!       state). We drive `proposal_activation_status` with an Accept verdict and
//!       assert the terminal status is a loop-closing re-init, not the
//!       sandbox-canary dead-end. RED: the only Accept terminal is
//!       `"sandbox:canary_only"`.
//! When the FC3 runtime engine lands under §8 (real proposer spec + runtime Veto
//! walking clauses + Architect commit driving a tape-visible re-init that
//! recomputes the trust root), both observations flip GREEN and this gate is
//! promoted.
//!
//! ── EXCLUSION MECHANISM (same as G1..G4) ─────────────────────────────────
//! Under `tests/pending/` (not auto-compiled; no Cargo.toml edit — Cargo.toml is
//! Trust-Root-pinned), not `constitution_*.rs` at top level, not in the
//! constitution gates manifest → invisible to `cargo test --workspace`,
//! `run_constitution_gates.sh`, and `constitution_matrix_drift`. Run on demand
//! by `scripts/run_pending_agentic_os_kill_conditions.sh` via `rustc --test`.

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::real5_roles::{
    proposal_activation_status, ToolProposal, ToolProposalPayload, VetoDecision, VetoReasonClass,
    VetoVerdict,
};
use turingosv4::state::q_state::TxId;

/// (A) The live role-path proposal payload must carry a real spec reference.
/// Today the role gateway emits `ToolProposalPayload::default()` (proposal_id ==
/// None), so the proposer carries NO spec. Returns whether the live-path payload
/// is a real (non-empty) proposal.
fn live_path_proposal_carries_spec() -> bool {
    // Exactly the shape the live role path produces (see
    // real5_roles.rs: "propose_tool" => ToolProposalPayload::default()).
    let live_payload = ToolProposalPayload::default();
    live_payload.proposal_id.is_some()
}

/// (B) The terminal status of an ACCEPTED ArchitectAI proposal. Today the only
/// Accept terminal is `"sandbox:canary_only"` — the loop never closes to a
/// re-init / committed meta-state.
fn accepted_proposal_terminal_status() -> &'static str {
    let proposal = ToolProposal {
        proposal_id: TxId("fc3-proposal-real".into()),
        evidence_capsule_cid: Cid([7; 32]),
        proposed_tool_patch_cid: Cid([9; 32]),
        expected_error_reduction: None,
    };
    let accept = VetoDecision {
        proposal_id: proposal.proposal_id.clone(),
        verdict: VetoVerdict::Accept,
        reason_class: VetoReasonClass::CanaryEligible,
        public_summary: "veto-ai pass".into(),
    };
    proposal_activation_status(&proposal, Some(&accept))
}

/// True iff the given terminal status closes the FC3 loop (a tape-visible
/// re-init / irreversible meta-commit). The sandbox-canary dead-end does NOT
/// close the loop. We probe by status family so the eventual fix is not
/// over-constrained on the exact closing token.
fn status_closes_fc3_loop(status: &str) -> bool {
    status.contains("reinit") || status.contains("committed") || status.contains("re-init")
}

/// G5 — the FC3 meta-architecture loop must actually CLOSE: a real proposer spec
/// + a Veto-AI PASS must drive a tape-visible re-init, not a sandbox-canary
/// dead-end.
///
/// EXPECTED RESULT AT PRE-§8: **RED (STANDING)**. (A) the live role-path proposal
/// payload is an empty `::default()` shell (no spec), and (B) the only Accept
/// terminal is `"sandbox:canary_only"` — the loop never reaches a re-init /
/// trust-root recompute. PROMOTION requires §8 Class-4 ratification of the FC3
/// irreversible-commit path (RootBox / boot trust-root / constitution boundary).
/// See the top comment.
#[test]
fn m07_fc3_meta_loop_must_close_with_tape_visible_reinit() {
    // (A) proposer must carry a real spec.
    let carries_spec = live_path_proposal_carries_spec();
    assert!(
        carries_spec,
        "M07 FC3 PROPOSER INERT (PENDING / STANDING / EXPECTED-RED): the live \
         role-path proposal payload is `ToolProposalPayload::default()` \
         (proposal_id == None) — the ArchitectAI proposer carries NO real spec. \
         src/runtime/real5_roles.rs emits the empty shell on the `propose_tool` \
         action. The FC3 runtime engine must make the proposer carry a real \
         synthesized proposal spec. Standing pending §8 Class-4 ratification of \
         the FC3 runtime/irreversible-commit path."
    );

    // (B) the loop must close past the sandbox-canary dead-end.
    let terminal = accepted_proposal_terminal_status();
    assert!(
        status_closes_fc3_loop(terminal),
        "M07 FC3 LOOP DOES NOT CLOSE (PENDING / STANDING / EXPECTED-RED): the \
         terminal status of an ACCEPTED ArchitectAI proposal is `{terminal}` — a \
         sandbox-canary dead-end. There is NO loop-closing status (no re-init, no \
         trust-root recompute, no constitution-bound hash advance). \
         proposal_activation_status (src/runtime/real5_roles.rs:1077-1091) maps \
         the only Accept verdict to \"sandbox:canary_only\"; the FC3 governance \
         binary stamps the SAME constitution_source_hash() at every stage, so the \
         \"re-init\" never actually mutates the constitution-bound state. The FC3 \
         runtime engine (proposer spec -> runtime Veto walks clauses -> Architect \
         commit -> tape-visible re-init that recomputes the boot Trust Root) is \
         missing. PROMOTION requires §8 Class-4 ratification of the FC3 \
         irreversible-commit path (touches RootBox / boot trust-root authority / \
         constitution-amendment boundary, Art. V.1.1). Until then this gate stays \
         RED by design."
    );
}
