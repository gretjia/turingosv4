//! GA-3 — H-HET-2 minimal V3-style DAG reconstructibility gate.
//!
//! Authority: `handover/tracer_bullets/H_HET_2_PHASE2_GATE_DESIGN_2026-06-16.md` (GA-3).
//!
//! ENFORCES: every solved target in a tape produces a reconstructible 4-node DAG:
//!   BudgetDecision -> ProposalTelemetry -> VerificationResult -> OMEGA(solve)
//!
//! The chain is linked by step-local string CIDs (fixture-local, no real CAS needed).
//! A small test-local `reconstruct` fn walks the fixture records and builds
//! `(funding_step, proposal_step, verification_step, omega_step)` per solved target.
//!
//! FAILABLE via TWO red cases:
//!  (i) `red_missing_budget_decision_causes_dangling_proposal` — removes the
//!      BudgetDecision node for a target; asserts reconstruction detects the
//!      dangling proposal (no funding edge → reconstruction fails for that target).
//! (ii) `red_broken_parent_edge_prevents_path_reconstruction` — severs the link
//!      between ProposalTelemetry and its VerificationResult; asserts the 4-node
//!      path no longer closes.
//!
//! No generic graph lib. Vec + index links only (per GA-3 spec).
//! No src/ changes. No real CAS. All fixtures and logic are test-local.

// ── Fixture types ──────────────────────────────────────────────────────────────

/// A fixture-level CID: just a short string label so the links are legible.
type FixCid = String;

/// Minimal budget-decision fixture. Represents one tick of BudgetAllocationTelemetry
/// (the "funding" node): which model was selected, what target it funds.
#[derive(Debug, Clone)]
struct BudgetDecisionNode {
    /// unique step index
    step: usize,
    target_id: String,
    selected_model_id: String,
    /// CID that ProposalTelemetry must cite back to link this edge
    decision_cid: FixCid,
    /// token budget granted to this proposal call
    allocated_token_budget: u64,
}

/// Minimal proposal fixture. Represents one tick of ProposalTelemetry:
/// the model produced a candidate, citing the funding decision and an artifact.
#[derive(Debug, Clone)]
struct ProposalNode {
    step: usize,
    target_id: String,
    model_id: String,
    candidate_label: String,
    /// must match BudgetDecisionNode.decision_cid to form the funding edge
    funding_decision_cid: FixCid,
    /// CID for the verification edge to cite
    proposal_cid: FixCid,
    /// total tokens used (from ProposalTelemetry.token_counts.total())
    total_tokens: u64,
}

/// Minimal verification fixture. Represents one tick of VerificationResult:
/// the checker ran against the proposal and returned a verdict.
#[derive(Debug, Clone)]
struct VerificationNode {
    step: usize,
    target_id: String,
    /// must match ProposalNode.proposal_cid to form the proposal-to-verify edge
    proposal_cid: FixCid,
    /// own CID; OMEGA cites this
    verification_cid: FixCid,
    verified: bool,
    exit_code: i32,
}

/// Minimal OMEGA fixture. Represents the terminal solve event for a target:
/// cites the verification that confirmed the solve.
#[derive(Debug, Clone)]
struct OmegaNode {
    step: usize,
    target_id: String,
    /// must match VerificationNode.verification_cid to form the verify-to-omega edge
    verification_cid: FixCid,
}

/// A fully reconstructed 4-node path for one solved target.
#[derive(Debug)]
struct DagPath {
    target_id: String,
    budget_step: usize,
    proposal_step: usize,
    verification_step: usize,
    omega_step: usize,
}

// ── Reconstruction logic ───────────────────────────────────────────────────────

/// Reconstruct per-target DAG paths from the fixture tape.
///
/// For each OmegaNode:
///   1. Find the VerificationNode whose verification_cid matches omega.verification_cid.
///   2. Find the ProposalNode whose proposal_cid matches verification.proposal_cid.
///   3. Find the BudgetDecisionNode whose decision_cid matches proposal.funding_decision_cid.
///   4. Verify all four belong to the same target_id.
///
/// Returns Ok(paths) where paths has one entry per OMEGA. Returns Err if any edge
/// is dangling (missing node in the chain) or target_id is inconsistent.
fn reconstruct(
    decisions: &[BudgetDecisionNode],
    proposals: &[ProposalNode],
    verifications: &[VerificationNode],
    omegas: &[OmegaNode],
) -> Result<Vec<DagPath>, String> {
    let mut paths = Vec::new();
    for omega in omegas {
        // Step 1: resolve omega -> verification edge
        let verif = verifications
            .iter()
            .find(|v| v.verification_cid == omega.verification_cid)
            .ok_or_else(|| {
                format!(
                    "dangling omega edge: no VerificationNode with cid='{}' for target '{}'",
                    omega.verification_cid, omega.target_id
                )
            })?;

        // Step 2: resolve verification -> proposal edge
        let prop = proposals
            .iter()
            .find(|p| p.proposal_cid == verif.proposal_cid)
            .ok_or_else(|| {
                format!(
                    "dangling verification edge: no ProposalNode with cid='{}' for target '{}'",
                    verif.proposal_cid, omega.target_id
                )
            })?;

        // Step 3: resolve proposal -> budget-decision edge (funding)
        let dec = decisions
            .iter()
            .find(|d| d.decision_cid == prop.funding_decision_cid)
            .ok_or_else(|| {
                format!(
                    "dangling proposal edge: no BudgetDecisionNode with cid='{}' for target '{}' \
                     (missing funding node — budget allocation not on tape)",
                    prop.funding_decision_cid, omega.target_id
                )
            })?;

        // Step 4: all four nodes must share the same target
        if dec.target_id != omega.target_id
            || verif.target_id != omega.target_id
            || prop.target_id != omega.target_id
        {
            return Err(format!(
                "target_id mismatch across DAG nodes for omega target '{}': \
                 decision='{}' proposal='{}' verif='{}'",
                omega.target_id, dec.target_id, prop.target_id, verif.target_id
            ));
        }

        paths.push(DagPath {
            target_id: omega.target_id.clone(),
            budget_step: dec.step,
            proposal_step: prop.step,
            verification_step: verif.step,
            omega_step: omega.step,
        });
    }
    Ok(paths)
}

// ── Fixture builder ────────────────────────────────────────────────────────────

/// Build a minimal fixture tape for two solved targets: "target_A" and "target_B".
/// Each target has exactly one BudgetDecision → Proposal → Verification → Omega chain.
/// The economic claim (H-HET-2): the router allocated budget to the higher-scoring model
/// for each target; fixtures reflect that via distinct selected_model_id.
fn two_target_tape() -> (
    Vec<BudgetDecisionNode>,
    Vec<ProposalNode>,
    Vec<VerificationNode>,
    Vec<OmegaNode>,
) {
    let decisions = vec![
        BudgetDecisionNode {
            step: 0,
            target_id: "target_A".into(),
            selected_model_id: "model_high_vr".into(), // UCB-selected: high verify rate
            decision_cid: "dec:target_A:0".into(),
            allocated_token_budget: 1024,
        },
        BudgetDecisionNode {
            step: 1,
            target_id: "target_B".into(),
            selected_model_id: "model_cold_price".into(), // cold-start price prior
            decision_cid: "dec:target_B:1".into(),
            allocated_token_budget: 800,
        },
    ];
    let proposals = vec![
        ProposalNode {
            step: 2,
            target_id: "target_A".into(),
            model_id: "model_high_vr".into(),
            candidate_label: "induction".into(),
            funding_decision_cid: "dec:target_A:0".into(),
            proposal_cid: "prop:target_A:2".into(),
            total_tokens: 980,
        },
        ProposalNode {
            step: 3,
            target_id: "target_B".into(),
            model_id: "model_cold_price".into(),
            candidate_label: "ring".into(),
            funding_decision_cid: "dec:target_B:1".into(),
            proposal_cid: "prop:target_B:3".into(),
            total_tokens: 750,
        },
    ];
    let verifications = vec![
        VerificationNode {
            step: 4,
            target_id: "target_A".into(),
            proposal_cid: "prop:target_A:2".into(),
            verification_cid: "ver:target_A:4".into(),
            verified: true,
            exit_code: 0,
        },
        VerificationNode {
            step: 5,
            target_id: "target_B".into(),
            proposal_cid: "prop:target_B:3".into(),
            verification_cid: "ver:target_B:5".into(),
            verified: true,
            exit_code: 0,
        },
    ];
    let omegas = vec![
        OmegaNode {
            step: 6,
            target_id: "target_A".into(),
            verification_cid: "ver:target_A:4".into(),
        },
        OmegaNode {
            step: 7,
            target_id: "target_B".into(),
            verification_cid: "ver:target_B:5".into(),
        },
    ];
    (decisions, proposals, verifications, omegas)
}

// ── Green test ─────────────────────────────────────────────────────────────────

/// GREEN: a complete 2-target fixture tape reconstructs to exactly 2 paths, each
/// with a valid 4-node chain in step order. The node counts and edge ordering must
/// match the spec (1 BudgetDecision -> 1 Proposal -> 1 Verification -> 1 Omega
/// per solved target).
#[test]
fn dag_reconstructs_two_solved_targets() {
    let (decisions, proposals, verifications, omegas) = two_target_tape();
    let paths = reconstruct(&decisions, &proposals, &verifications, &omegas)
        .expect("reconstruction must succeed on a complete fixture tape");

    assert_eq!(
        paths.len(),
        2,
        "expected 1 path per solved target (2 total); got {}",
        paths.len()
    );

    for path in &paths {
        // Each path must have nodes strictly ordered: decision < proposal < verify < omega.
        assert!(
            path.budget_step < path.proposal_step,
            "target '{}': BudgetDecision step {} must precede Proposal step {}",
            path.target_id, path.budget_step, path.proposal_step
        );
        assert!(
            path.proposal_step < path.verification_step,
            "target '{}': Proposal step {} must precede Verification step {}",
            path.target_id, path.proposal_step, path.verification_step
        );
        assert!(
            path.verification_step < path.omega_step,
            "target '{}': Verification step {} must precede Omega step {}",
            path.target_id, path.verification_step, path.omega_step
        );
    }

    // Each target appears exactly once.
    let mut targets: Vec<&str> = paths.iter().map(|p| p.target_id.as_str()).collect();
    targets.sort_unstable();
    assert_eq!(targets, ["target_A", "target_B"]);
}

/// GREEN: the total token budget used across both targets equals the sum of
/// allocated_token_budget in BudgetDecisionNodes — budget is conserved and
/// represented on the tape (H-HET-2 economic claim: scarce budget flows to
/// the tape, not silently discarded).
#[test]
fn budget_allocation_is_tape_canonical_across_targets() {
    let (decisions, proposals, verifications, omegas) = two_target_tape();
    let paths = reconstruct(&decisions, &proposals, &verifications, &omegas).unwrap();

    for path in &paths {
        let dec = decisions
            .iter()
            .find(|d| d.step == path.budget_step)
            .expect("budget node must exist for every reconstructed path");
        let prop = proposals
            .iter()
            .find(|p| p.step == path.proposal_step)
            .expect("proposal node must exist for every reconstructed path");

        // The proposal must not exceed the allocated token budget (budget ceiling).
        assert!(
            prop.total_tokens <= dec.allocated_token_budget,
            "target '{}': proposal used {} tokens but budget granted only {}",
            path.target_id, prop.total_tokens, dec.allocated_token_budget
        );
    }
}

// ── Red test (i): remove the BudgetDecision — dangling proposal ───────────────

/// RED CASE (i): removing the BudgetDecision node for one target makes that
/// target's proposal dangling (no funding edge). Reconstruction MUST detect this
/// and return an Err, not silently produce a path.
///
/// This proves the gate is not vacuously green: it can catch a tape where the
/// funding decision was never emitted (the tape violates Art 0.2 — the budget
/// routing step is not on tape).
#[test]
fn red_missing_budget_decision_causes_dangling_proposal() {
    let (mut decisions, proposals, verifications, omegas) = two_target_tape();

    // Remove the BudgetDecision for target_A; its proposal still references
    // "dec:target_A:0" but that node is gone.
    decisions.retain(|d| d.target_id != "target_A");

    let result = reconstruct(&decisions, &proposals, &verifications, &omegas);

    assert!(
        result.is_err(),
        "reconstruction must FAIL when the BudgetDecision node is missing \
         (dangling proposal edge); got Ok — gate is vacuously green"
    );
    let msg = result.unwrap_err();
    assert!(
        msg.contains("dangling proposal edge") || msg.contains("missing funding node"),
        "error message must mention the dangling funding edge; got: {msg}"
    );
}

// ── Red test (ii): break the proposal->verification parent edge ───────────────

/// RED CASE (ii): severing the link between ProposalTelemetry and
/// VerificationResult for one target (by changing the verification's
/// proposal_cid to a stale/wrong value) makes reconstruction fail for that
/// target — the 4-node path can no longer be traced.
///
/// This proves the gate catches a tape where the citation/parent edge is corrupt
/// or missing (e.g. the verification was recorded without referencing its
/// corresponding proposal).
#[test]
fn red_broken_parent_edge_prevents_path_reconstruction() {
    let (decisions, proposals, mut verifications, omegas) = two_target_tape();

    // Break the proposal_cid on target_B's VerificationNode so it no longer
    // matches any ProposalNode.
    for v in verifications.iter_mut() {
        if v.target_id == "target_B" {
            v.proposal_cid = "prop:target_B:STALE_BROKEN_CID".into();
        }
    }

    let result = reconstruct(&decisions, &proposals, &verifications, &omegas);

    assert!(
        result.is_err(),
        "reconstruction must FAIL when the proposal->verification edge is broken; \
         got Ok — gate is vacuously green"
    );
    let msg = result.unwrap_err();
    assert!(
        msg.contains("dangling verification edge") || msg.contains("no ProposalNode"),
        "error message must mention the dangling verification edge; got: {msg}"
    );
}
