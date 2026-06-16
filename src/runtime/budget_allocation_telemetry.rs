//! H-HET-2 `BudgetAllocationTelemetry` — the tape-canonical record of ONE dynamic
//! model-budget routing decision (one routing tick).
//!
//! Authority: architect routing-policy ruling 2026-06-15
//! (`handover/tracer_bullets/H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md`),
//! policy family `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`, charter §5.4 + amendment #6.
//!
//! WHY (Art 0.2 — Tape Canonical): the H-HET-2 *treatment* is HOW budget is routed
//! (which model gets the next proposal-call), not merely WHO produced a proposal
//! (`ProposalTelemetry.model_id`, §8). So every routing tick must emit this object so
//! a replayer can rebuild the allocation decision from the frozen tape ALONE:
//! `allocation_view == derive_from_tape(tape)`. Sidecars are derived views only.
//!
//! Goodhart shield (Art III.4 + ruling amendment #4): this object is AUDIT-visible
//! (written to CAS for replay), NOT proposer-visible. The UCB score / bonus / floor /
//! weights here MUST NOT be broadcast into a proposer prompt.
//!
//! Storage pattern mirrors `proposal_telemetry.rs` 1:1: canonical-encoded
//! (positional bincode), `CasStore::put` with `ObjectType::Generic` + schema id
//! `turingosv4.budget_allocation_telemetry.v1`. NEW schema (v1) — no legacy decoder
//! yet; a future field addition MUST follow the §8 discipline (schema-id bump +
//! `decode_bytes` v(N-1) fallback) because positional bincode requires full byte
//! consumption.
//!
//! NOTE for the carrier wire-up atom: emitting this under a NEW schema id means the
//! librarian schema classifier (`src/runtime/librarian_broadcast.rs`) MUST add
//! `"turingosv4.budget_allocation_telemetry.v1"` to its known-safe ignore list, or it
//! will HARD-ERROR ("unknown librarian evidence schema") scanning the carrier CAS
//! (the exact §8 bug class). Do this when the carrier first writes this object.

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::state::q_state::Hash;

const BUDGET_ALLOCATION_TELEMETRY_SCHEMA_ID: &str = "turingosv4.budget_allocation_telemetry.v1";

/// Why a particular model won this routing tick (ruling: selection_reason).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionReason {
    /// Won via the guaranteed ε exploration floor (Art II.2.1).
    Floor,
    /// Won on UCB score (verify-outcome value + count bonus).
    UcbScore,
    /// Won on the bounded target-local price prior (cold-start, n_pull<N_cold & no verify yet).
    ColdStart,
    /// Won a deterministic tie-break (lexicographic / roster-order; no RNG in v1).
    TieBreak,
}

/// Per-candidate-model row evaluated this tick. All scores are integer basis points
/// (bps); no f64 on the money/budget path. Counts are read FROM THE TAPE each tick
/// (prior `BudgetAllocationTelemetry` + `ProposalTelemetry`/`VerificationResult`
/// events for this target), never memory-canonical (Art 0.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelScoreRow {
    pub model_id: String,
    /// # prior ticks this run that selected this model on this target.
    pub pull_count_model_target_before: u32,
    /// # of those pulls whose paired VerificationResult.verified == true.
    pub verify_count_model_target_before: u32,
    /// consecutive tape-recorded HARD predicate failures for the N_hard_fail cutoff.
    /// "Hard" = a genuine domain-predicate rejection; soft/infra failures (provider error,
    /// timeout, rate-limit, parse-fallback, tool-infra) are EXCLUDED. The hard-vs-soft
    /// CLASSIFICATION is the domain predicate driver's responsibility, NOT this generic
    /// schema's — this kernel-layer record stays domain-agnostic (the current math
    /// experiment's driver supplies the concrete classes; no domain verifier is named here).
    pub hard_failure_streak_before: u32,
    /// verify-rate score: 10000*(verify+1)/(pull+2) (Beta(1,1) neutral prior).
    pub vr_bps: u64,
    /// bounded target-local price prior (cold-start only; 0 after first verify / N_cold pulls).
    pub price_bps: u64,
    /// deterministic isqrt count bonus: C_UCB * isqrt((N_T+1)/(n_mT+1)).
    pub bonus_bps: u64,
    /// final composite: W_VERIFY*vr_bps + W_PRICE*price_bps + bonus_bps.
    pub score_bps: u64,
    /// is this model still inside its guaranteed ε floor on this target?
    pub exploration_active: bool,
    pub floor_quota_remaining_before: u64,
    pub floor_quota_remaining_after: u64,
}

/// One dynamic model-budget routing decision (one tick). Header fields are per-tick;
/// `candidates` carries the per-model rows the policy scored before selecting.
///
/// **Field set is binding (charter §5.4 + ruling amendment #6); do NOT add fields
/// without architect ratification + a schema-id bump + a legacy decoder (§8 discipline).**
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAllocationTelemetry {
    /// frozen policy family, e.g. "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1".
    pub policy_family: String,
    /// sha256 of the frozen policy config (RoutingPolicyGenesisPin.policy_hash).
    pub policy_hash: Hash,
    /// e.g. "turingosv4.ucb_budget.v1".
    pub policy_version: String,
    pub target_id: String,
    pub seed_id: u64,
    /// hash of the FROZEN eligible roster (drift-detects post-calibration roster changes).
    pub eligible_model_set_hash: Hash,
    /// CID of the EconomicState / price-index snapshot this tick read.
    pub input_state_cid: Cid,
    /// CID of the visible per-model price vector this tick saw.
    pub price_vector_cid: Cid,
    /// CID of the abstracted (shielded) per-model failure features this tick saw.
    pub abstracted_failure_features_cid: Cid,
    /// total pulls on this target before this tick (Σ candidates' pull_count).
    pub total_pulls_target_before: u32,
    /// the per-model rows the policy scored this tick.
    pub candidates: Vec<ModelScoreRow>,
    /// the model that won this tick (its proposal gets funded).
    pub selected_model_id: String,
    pub selection_reason: SelectionReason,
    pub allocated_proposal_budget: u64,
    pub allocated_token_budget: u64,
    pub budget_remaining_before: u64,
    pub budget_remaining_after: u64,
    /// CID of the router-overhead accounting (so "all router overhead counted" closes
    /// at the field level for the §4 budget-fairness rule).
    pub router_overhead_cid: Cid,
    /// forward-compat for a future stochastic policy; v1 is deterministic → None.
    #[serde(default)]
    pub rng_seed: Option<u64>,
    #[serde(default)]
    pub rng_draw: Option<u64>,
}

impl BudgetAllocationTelemetry {
    /// Σ over candidates of pull_count — the value the header `total_pulls_target_before`
    /// must equal (a tape-internal consistency invariant a replayer can check).
    pub fn candidate_pull_sum(&self) -> u32 {
        self.candidates
            .iter()
            .map(|c| c.pull_count_model_target_before)
            .sum()
    }
}

/// Single-sourced max_tokens cap for ONE Stage-2 proposal (proof) LLM call. The carrier
/// (`lean_market_agent`) uses this as (1) the proposal `max_tokens`, (2) the truncation
/// threshold, and (3) the per-tick `allocated_token_budget` reservation upper bound.
pub const MAX_PROPOSAL_TOKENS: u64 = 900;

/// PURE helper for the run-path budget fields of one routing tick, so the GA-5 conservation
/// invariant is unit-testable WITHOUT a live LLM/Lean run (the carrier's run path calls this
/// EXACT function). Returns `(allocated_proposal_budget, allocated_token_budget,
/// budget_remaining_before, budget_remaining_after)`.
///
/// SEMANTICS (audit-critical): the budget BALANCE is denominated in PROPOSAL-CALL units — the
/// SAME unit as `rt_total_budget = effective_rounds * agents.len()` and the SAME unit
/// `routing_policy::score_and_select`'s 3rd arg (`remaining_target_budget`) expects: it compares
/// `remaining` against `Σ floor_quota_remaining`, and `RoutingPolicyConfig::floor_quota` returns
/// `floor(ε * rt_total_budget)` — also CALL units. One routing tick funds exactly ONE proposal
/// call ⇒ `allocated_proposal_budget = 1`, `before = rt_total_budget − step_idx`,
/// `after = before − allocated_proposal_budget`. `allocated_token_budget = MAX_PROPOSAL_TOKENS`
/// is a SEPARATE token-RESERVATION field, NOT a term in the balance equation (the prior bug
/// subtracted this TOKEN field from the CALL-unit balance).
pub fn budget_alloc_fields(rt_total_budget: u64, step_idx: u64) -> (u64, u64, u64, u64) {
    let allocated_proposal_budget = 1u64;
    let allocated_token_budget = MAX_PROPOSAL_TOKENS;
    let budget_remaining_before = rt_total_budget.saturating_sub(step_idx);
    let budget_remaining_after = budget_remaining_before.saturating_sub(allocated_proposal_budget);
    (
        allocated_proposal_budget,
        allocated_token_budget,
        budget_remaining_before,
        budget_remaining_after,
    )
}

// ── Errors ──────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum BudgetAllocationTelemetryError {
    Cas(CasError),
    Codec(String),
}

impl std::fmt::Display for BudgetAllocationTelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(s) => write!(f, "codec error: {s}"),
        }
    }
}

impl std::error::Error for BudgetAllocationTelemetryError {}

impl From<CasError> for BudgetAllocationTelemetryError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}

// ── CAS storage (mirrors proposal_telemetry.rs) ───────────────────────────────

/// Canonical-encode + CAS put. Returns the content-addressed CID. Idempotent
/// (same record → same CID), so a chain reference to this allocation is byte-stable.
pub fn write_to_cas(
    cas: &mut CasStore,
    record: &BudgetAllocationTelemetry,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, BudgetAllocationTelemetryError> {
    let bytes = canonical_encode(record)
        .map_err(|e| BudgetAllocationTelemetryError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(BUDGET_ALLOCATION_TELEMETRY_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// Shared decoder over raw CAS bytes. v1 has no legacy predecessor; a future vN+1
/// MUST add a try-vN+1-then-vN fallback here (positional bincode requires full byte
/// consumption — see proposal_telemetry::decode_bytes / the §8 discipline).
pub fn decode_bytes(bytes: &[u8]) -> Result<BudgetAllocationTelemetry, BudgetAllocationTelemetryError> {
    canonical_decode::<BudgetAllocationTelemetry>(bytes)
        .map_err(|e| BudgetAllocationTelemetryError::Codec(e.to_string()))
}

/// CAS fetch + decode. Used by replay / `derive_from_tape` to recover the allocation.
pub fn read_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<BudgetAllocationTelemetry, BudgetAllocationTelemetryError> {
    decode_bytes(&cas.get(cid)?)
}

/// Convenience: open CAS at `cas_path` and read the record at `cid`.
pub fn read_from_cas_path(
    cas_path: &Path,
    cid: &Cid,
) -> Result<BudgetAllocationTelemetry, BudgetAllocationTelemetryError> {
    let cas = CasStore::open(cas_path)?;
    read_from_cas(&cas, cid)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_cas() -> (TempDir, CasStore) {
        let dir = TempDir::new().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("open cas");
        (dir, cas)
    }

    fn row(model: &str, pull: u32, ver: u32) -> ModelScoreRow {
        ModelScoreRow {
            model_id: model.into(),
            pull_count_model_target_before: pull,
            verify_count_model_target_before: ver,
            hard_failure_streak_before: 0,
            vr_bps: 10000 * (ver as u64 + 1) / (pull as u64 + 2),
            price_bps: 0,
            bonus_bps: 1500,
            score_bps: 0,
            exploration_active: true,
            floor_quota_remaining_before: 1,
            floor_quota_remaining_after: 0,
        }
    }

    fn fresh_record() -> BudgetAllocationTelemetry {
        BudgetAllocationTelemetry {
            policy_family: "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1".into(),
            policy_hash: Hash([1u8; 32]),
            policy_version: "turingosv4.ucb_budget.v1".into(),
            target_id: "lm_det_mul".into(),
            seed_id: 1,
            eligible_model_set_hash: Hash([2u8; 32]),
            input_state_cid: Cid([3u8; 32]),
            price_vector_cid: Cid([4u8; 32]),
            abstracted_failure_features_cid: Cid([5u8; 32]),
            total_pulls_target_before: 3,
            candidates: vec![row("deepseek", 1, 0), row("qwen397", 2, 1)],
            selected_model_id: "qwen397".into(),
            selection_reason: SelectionReason::UcbScore,
            allocated_proposal_budget: 1,
            allocated_token_budget: 2048,
            budget_remaining_before: 12,
            budget_remaining_after: 11,
            router_overhead_cid: Cid([6u8; 32]),
            rng_seed: None,
            rng_draw: None,
        }
    }

    #[test]
    fn write_read_round_trip() {
        let (_d, mut cas) = fresh_cas();
        let r = fresh_record();
        let cid = write_to_cas(&mut cas, &r, "het2-test", 1).expect("write");
        assert_eq!(read_from_cas(&cas, &cid).expect("read"), r);
    }

    #[test]
    fn cid_determinism() {
        let (_d, mut cas) = fresh_cas();
        let r = fresh_record();
        let a = write_to_cas(&mut cas, &r, "t", 1).expect("a");
        let b = write_to_cas(&mut cas, &r, "t", 1).expect("b");
        assert_eq!(a, b);
    }

    #[test]
    fn distinct_records_distinct_cids() {
        let (_d, mut cas) = fresh_cas();
        let r1 = fresh_record();
        let mut r2 = fresh_record();
        r2.selected_model_id = "deepseek".into();
        r2.selection_reason = SelectionReason::Floor;
        let a = write_to_cas(&mut cas, &r1, "t", 1).expect("a");
        let b = write_to_cas(&mut cas, &r2, "t", 1).expect("b");
        assert_ne!(a, b);
    }

    /// Tape-internal consistency: the header total must equal Σ candidate pulls.
    #[test]
    fn candidate_pull_sum_matches_header() {
        let r = fresh_record();
        assert_eq!(r.candidate_pull_sum(), r.total_pulls_target_before);
    }

    /// Goodhart-shield / forbidden-content guard: this audit object must NOT carry raw
    /// deliberation or proposer-visible secrets — only provenance + integer metrics.
    #[test]
    fn no_raw_deliberation_fields() {
        let r = fresh_record();
        let json = serde_json::to_string(&r).expect("serialize");
        for forbidden in [
            "chain_of_thought",
            "raw_prompt",
            "raw_completion",
            "model_deliberation",
            "internal_reasoning",
        ] {
            assert!(!json.contains(forbidden), "telemetry leaks {forbidden}");
        }
    }

    /// v1 is deterministic: RNG slots default to None and survive round-trip.
    #[test]
    fn rng_slots_none_in_v1() {
        let (_d, mut cas) = fresh_cas();
        let r = fresh_record();
        let cid = write_to_cas(&mut cas, &r, "t", 1).unwrap();
        let got = read_from_cas(&cas, &cid).unwrap();
        assert_eq!(got.rng_seed, None);
        assert_eq!(got.rng_draw, None);
    }
}
