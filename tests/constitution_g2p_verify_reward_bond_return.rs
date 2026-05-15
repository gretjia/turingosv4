//! TB-G G2P.3 — verifier reward / bond return audit (Class 1).
//!
//! Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
//! §1 Module G2P atom G2P.3.
//!
//! Ship gate SG-G2P.6 per charter: "existing TB-N1 A4 gates GREEN OR
//! `OBS_G2P_VERIFY_PEER_REWARD` filed".
//!
//! Outcome: TB-N1 A4 admission gates ARE GREEN at this HEAD (admission
//! contract holds), AND the OBS is filed because reputation accumulation
//! + bond return at run-resolve are NOT implemented in the current
//! VerifyTx arm (Class-3+ forward work; G3.1/G3.2 boundary).
//!
//! This Class-1 binding test PINS the current contract so any future
//! fix is gate-time-caught:
//! - SG-G2P.6.a: OBS file `handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md`
//!   exists with the documented forward-closure section.
//! - SG-G2P.6.b: TB-N1 A4 admission contract is present in
//!   `src/state/sequencer.rs` VerifyTx arm (source-grep witness — bond
//!   debit + stakes_t insert + agent_verifications_t insert).
//! - SG-G2P.6.c: current sequencer code does NOT mutate `reputations_t`
//!   from any admission arm — negative-witness source-grep. Any future
//!   commit that adds `reputations_t.*insert` to sequencer.rs flips
//!   this assertion and surfaces the gap-closure work for review.
//!
//! `FC-trace: §13 verify/bond agency layer + FC1-N7 δ Agent externalized
//! output — current contract pinned, forward gap documented in OBS.`

use std::path::Path;

const SEQUENCER_SRC: &str = "src/state/sequencer.rs";
const OBS_PATH: &str = "handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md";

fn compact_ws(src: &str) -> String {
    src.chars().filter(|c| !c.is_whitespace()).collect()
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.6.a — OBS file present with forward-closure section
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2p_6_obs_file_present_with_forward_closure_section() {
    assert!(
        Path::new(OBS_PATH).exists(),
        "SG-G2P.6.a: OBS file `{OBS_PATH}` missing — charter ship gate \
         requires either TB-N1 A4 gates GREEN (witnessed by \
         constitution_n1_agent_economy_a4 7/7) OR the OBS branch."
    );
    let obs = std::fs::read_to_string(OBS_PATH).expect("OBS file readable");
    assert!(
        obs.contains("Gap-A — verifier reputation accumulation"),
        "SG-G2P.6.a: OBS must document Gap-A (no reputation_t mutation in \
         sequencer arms)."
    );
    assert!(
        obs.contains("Gap-B — bond return at run-resolve"),
        "SG-G2P.6.a: OBS must document Gap-B (bond stays in stakes_t with \
         no FinalizeReward / TerminalSummary unlock path)."
    );
    assert!(
        obs.contains("Forward closure criteria"),
        "SG-G2P.6.a: OBS must enumerate forward-closure criteria so the \
         gap doesn't become permanent-AMBER per \
         feedback_no_workarounds_strict_constitution."
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.6.b — TB-N1 A4 admission contract present in VerifyTx arm
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2p_6_verify_tx_arm_preserves_tb_n1_a4_admission_contract() {
    let seq = std::fs::read_to_string(SEQUENCER_SRC).expect("sequencer.rs readable");
    let compact = compact_ws(&seq);
    // (1) Bond debit from balances_t.
    assert!(
        seq.contains("balances_t.0.insert(\n                verify.verifier_agent.clone(),")
            || seq
                .contains("new_bal_micro = verifier_bal.micro_units() - verify.bond.micro_units()"),
        "SG-G2P.6.b: VerifyTx arm must debit verifier balance by bond_micro \
         (TB-N1 A4 admission Step 5)."
    );
    // (2) Bond locked into stakes_t under verify.tx_id.
    assert!(
        seq.contains("stakes_t.0.insert(\n                verify.tx_id.clone()"),
        "SG-G2P.6.b: VerifyTx arm must lock bond into stakes_t keyed by \
         verify.tx_id (TB-N1 A4 admission Step 5)."
    );
    // (3) agent_verifications_t set insert closes the duplicate-verify gate.
    assert!(
        compact.contains("agent_verifications_t.0.insert(verify_pair)"),
        "SG-G2P.6.b: VerifyTx arm must insert (verifier, target) into \
         agent_verifications_t (TB-N1 A4 admission Step 5b)."
    );
    // (4) Step 3.5 duplicate-verify gate present (defense-in-depth witness).
    assert!(
        seq.contains("VerifyDuplicate"),
        "SG-G2P.6.b: VerifyTx arm must reject duplicate (verifier, target) \
         with VerifyDuplicate (TB-N1 A4 admission Step 3.5)."
    );
    // (5) Step 3 target liveness gate present.
    assert!(
        seq.contains("VerifyTargetNotAccepted"),
        "SG-G2P.6.b: VerifyTx arm must reject phantom target with \
         VerifyTargetNotAccepted (TB-N1 A4 admission Step 3)."
    );
}

// ────────────────────────────────────────────────────────────────────────
// SG-G2P.6.c — G3.2 positive-witness: reputations_t mutation in VerifyTx
// ────────────────────────────────────────────────────────────────────────

#[test]
fn sg_g2p_6_reputations_t_mutation_resolved_in_verify_arm() {
    // G3.2 closed the former Gap-A scaffold: accepted VerifyTx now grants
    // uniform +1 reputation to the verifier. Keep this as a positive witness
    // so formatting changes do not silently re-open the old AMBER gap.
    let seq = std::fs::read_to_string(SEQUENCER_SRC).expect("sequencer.rs readable");
    let compact = compact_ws(&seq);
    assert!(
        compact.contains("reputations_t.0.entry(verify.verifier_agent.clone())")
            && compact.contains(".or_insert(crate::state::q_state::Reputation(0)).0+=1"),
        "SG-G2P.6.c: VerifyTx accept path must grant uniform +1 reputation \
         to the verifier after G3.2 closure."
    );
}
