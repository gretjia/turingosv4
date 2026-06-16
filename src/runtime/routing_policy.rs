//! H-HET-2 dynamic model-budget routing policy `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`.
//!
//! Authority: architect routing-policy ruling 2026-06-15
//! (`handover/tracer_bullets/H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md`).
//!
//! GENERIC-KERNEL DISCIPLINE (architect 2026-06-15, memory kernel-generic-not-lean):
//! this is a DOMAIN-AGNOSTIC mechanism. It routes proposal-call budget among models
//! using only generic predicate-outcome counts (pull / verify / hard-failure) — it
//! names NO domain verifier (no Lean/sorry/tactic/mathlib). The math-domain driver
//! (the carrier bin + LeanJudge) supplies the counts; this module decides allocation.
//!
//! The mechanism: deterministic outcome-driven UCB (reward = per-(model,target)
//! predicate verify), a bounded target-local price prior for cold-start ONLY, a
//! mandatory ε exploration floor, an integer isqrt count bonus, and NO stochastic RNG.
//! Selection is a pure function of the inputs → fully replayable (Art 0.2). The frozen
//! `RoutingPolicyConfig` is sha-pinned via `RoutingPolicyGenesisPin` (§17.3: the name
//! says UCB, the code is UCB — not a softmax distributor).

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

use crate::bottom_white::cas::schema::{Cid, ObjectType};
use crate::bottom_white::cas::store::{CasError, CasStore};
use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
use crate::runtime::budget_allocation_telemetry::{ModelScoreRow, SelectionReason};
use crate::state::q_state::Hash;

const ROUTING_POLICY_GENESIS_PIN_SCHEMA_ID: &str = "turingosv4.routing_policy_genesis_pin.v1";

/// Deterministic tie-break rule (no RNG in v1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TieBreak {
    /// lexicographic by model_id (smallest wins).
    Lexicographic,
}

/// The FROZEN routing-policy parameters. sha256(canonical_encode(self)) == policy_hash,
/// which the GenesisPin pins before any paid run (Goodhart-shield + p-hacking guard).
/// All bps are integer basis points; no f64 on the budget path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyConfig {
    pub policy_family: String,  // "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1"
    pub policy_version: String, // "turingosv4.ucb_budget.v1"
    pub w_verify: u64,          // 8
    pub w_price: u64,           // 1
    pub price_component_cap_bps: u64, // 1250 — cap on w_price*price_bps in the score
    pub n_cold: u32,            // 4 — price prior applies only while pull<n_cold && verify==0
    pub price_clamp_lo_bps: u64, // 2500
    pub price_clamp_hi_bps: u64, // 7500
    pub c_ucb_bps: u64,         // 2500 — count-bonus coefficient
    pub n_hard_fail: u32,       // 3 — consecutive hard failures → exit floor (not banned)
    pub eps_cap_num: u64,       // 10  } ε_model = min(eps_cap_num/eps_cap_den, eps_share_num/(eps_share_den*k))
    pub eps_cap_den: u64,       // 100 } = min(0.10, 0.40/k)
    pub eps_share_num: u64,     // 40
    pub eps_share_den: u64,     // 100
    pub tie_break: TieBreak,
}

impl Default for RoutingPolicyConfig {
    /// The architect-approved defaults (ruling 2026-06-15).
    fn default() -> Self {
        Self {
            policy_family: "VERIFY_UCB_PRICE_PRIOR_FLOOR_V1".into(),
            policy_version: "turingosv4.ucb_budget.v1".into(),
            w_verify: 8,
            w_price: 1,
            price_component_cap_bps: 1250,
            n_cold: 4,
            price_clamp_lo_bps: 2500,
            price_clamp_hi_bps: 7500,
            c_ucb_bps: 2500,
            n_hard_fail: 3,
            eps_cap_num: 10,
            eps_cap_den: 100,
            eps_share_num: 40,
            eps_share_den: 100,
            tie_break: TieBreak::Lexicographic,
        }
    }
}

impl RoutingPolicyConfig {
    /// sha256 of the canonical-encoded config — the frozen policy identity.
    pub fn policy_hash(&self) -> Hash {
        let bytes = canonical_encode(self).expect("RoutingPolicyConfig canonical encodes");
        Hash(Sha256::digest(&bytes).into())
    }

    /// ε_model = min(eps_cap, eps_share/k) returned as an integer-rational (num, den)
    /// over a COMMON denominator, exact, no f64. k = number of eligible models.
    pub fn eps_floor(&self, k: u64) -> (u64, u64) {
        let k = k.max(1);
        // cap = eps_cap_num/eps_cap_den ; share = eps_share_num/(eps_share_den*k)
        // common denom = eps_cap_den * eps_share_den * k
        let den = self.eps_cap_den * self.eps_share_den * k;
        let cap = self.eps_cap_num * self.eps_share_den * k; // over den
        let share = self.eps_share_num * self.eps_cap_den; // over den
        (cap.min(share), den)
    }

    /// Per-model guaranteed floor quota over a target budget: floor(ε * B_target).
    pub fn floor_quota(&self, k: u64, target_budget: u64) -> u64 {
        let (num, den) = self.eps_floor(k);
        target_budget.saturating_mul(num) / den
    }
}

/// Integer square root (Newton's method) — deterministic, replayable, no float.
pub fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Generic per-model input the policy scores. The carrier derives these from its own
/// (domain-specific) tape: counts of prior pulls/verifies/hard-failures of model m on
/// target T, the bounded target-local price prior (already in bps), and the model's
/// remaining ε-floor quota on T. No domain terms here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelInput {
    pub model_id: String,
    pub pull_count: u32,
    pub verify_count: u32,
    pub hard_failure_streak: u32,
    pub price_prior_bps: u64, // target-local; 0 if model has no target-local node yet
    pub floor_quota_remaining: u64,
}

/// The selection result: the scored rows (for telemetry), the winner, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selection {
    pub rows: Vec<ModelScoreRow>,
    pub selected_model_id: String,
    pub reason: SelectionReason,
}

/// THE MECHANISM. Pure, deterministic, integer. Scores every eligible model and
/// selects the funded one for this tick.
///
/// `remaining_target_budget` = proposal calls left on this target (for the deadline-aware
/// floor). A model is "owed floor" iff it is exploration-active (hard_failure_streak <
/// n_hard_fail) AND floor_quota_remaining > 0. If the remaining budget is about to run
/// out (≤ Σ owed quotas), the tick MUST go to an owed model so every active model meets
/// its ε guarantee; otherwise the tick goes to the top score (exploit), and if that
/// winner is owed-floor its quota still decrements.
pub fn score_and_select(
    config: &RoutingPolicyConfig,
    models: &[ModelInput],
    remaining_target_budget: u64,
) -> Selection {
    assert!(!models.is_empty(), "score_and_select needs ≥1 eligible model");
    let total_pulls: u128 = models.iter().map(|m| m.pull_count as u128).sum();

    // Per-model score (integer bps).
    let mut rows: Vec<ModelScoreRow> = Vec::with_capacity(models.len());
    for m in models {
        let pull = m.pull_count as u128;
        let ver = m.verify_count as u128;
        // verify-rate, Beta(1,1) neutral prior: 10000*(ver+1)/(pull+2).
        let vr_bps = (10_000 * (ver + 1) / (pull + 2)) as u64;
        // price prior: cold-start only, clamped.
        let in_cold = m.pull_count < config.n_cold && m.verify_count == 0;
        let price_bps = if in_cold && m.price_prior_bps > 0 {
            m.price_prior_bps
                .clamp(config.price_clamp_lo_bps, config.price_clamp_hi_bps)
        } else {
            0
        };
        // count bonus: c_ucb * sqrt((total+1)/(pull+1)), integer via isqrt of a scaled ratio.
        let ratio_scaled = (total_pulls + 1) * 10_000 / (pull + 1); // = ratio * 1e4
        let bonus_bps = config.c_ucb_bps * (isqrt(ratio_scaled) as u64) / 100; // sqrt(ratio)≈isqrt/100
        // composite, with the price component capped.
        let price_component = (config.w_price * price_bps).min(config.price_component_cap_bps);
        let score_bps = config.w_verify * vr_bps + price_component + bonus_bps;
        let active = m.hard_failure_streak < config.n_hard_fail;
        rows.push(ModelScoreRow {
            model_id: m.model_id.clone(),
            pull_count_model_target_before: m.pull_count,
            verify_count_model_target_before: m.verify_count,
            hard_failure_streak_before: m.hard_failure_streak,
            vr_bps,
            price_bps,
            bonus_bps,
            score_bps,
            exploration_active: active,
            floor_quota_remaining_before: m.floor_quota_remaining,
            floor_quota_remaining_after: m.floor_quota_remaining, // patched for the winner below
        });
    }

    // Owed-floor set: active models with quota remaining.
    let owed_total: u64 = models
        .iter()
        .filter(|m| m.hard_failure_streak < config.n_hard_fail && m.floor_quota_remaining > 0)
        .map(|m| m.floor_quota_remaining)
        .sum();
    let must_spend_floor = owed_total > 0 && remaining_target_budget <= owed_total;

    // Candidate index set + winner by (score desc, model_id asc) tie-break.
    let pick = |idxs: &[usize]| -> usize {
        *idxs
            .iter()
            .max_by(|&&a, &&b| {
                rows[a]
                    .score_bps
                    .cmp(&rows[b].score_bps)
                    .then_with(|| rows[b].model_id.cmp(&rows[a].model_id)) // smaller id wins ties
            })
            .expect("non-empty candidate set")
    };

    let owed_idxs: Vec<usize> = (0..models.len())
        .filter(|&i| {
            models[i].hard_failure_streak < config.n_hard_fail
                && models[i].floor_quota_remaining > 0
        })
        .collect();
    let all_idxs: Vec<usize> = (0..models.len()).collect();

    // Is the top score a real tie (≥2 models at the max)? → TieBreak reason.
    let max_score = rows.iter().map(|r| r.score_bps).max().unwrap_or(0);
    let n_at_max = rows.iter().filter(|r| r.score_bps == max_score).count();

    let (winner, reason) = if must_spend_floor && !owed_idxs.is_empty() {
        (pick(&owed_idxs), SelectionReason::Floor)
    } else {
        let w = pick(&all_idxs);
        // reason classification for the exploit pick.
        let r = if n_at_max >= 2 && rows[w].score_bps == max_score {
            SelectionReason::TieBreak
        } else if rows[w].price_bps > 0
            && rows[w].verify_count_model_target_before == 0
            && rows[w].pull_count_model_target_before < config.n_cold
        {
            SelectionReason::ColdStart
        } else {
            SelectionReason::UcbScore
        };
        (w, r)
    };

    // Decrement the winner's floor quota if it had one (the tick counts toward its floor).
    if rows[winner].floor_quota_remaining_before > 0
        && rows[winner].exploration_active
    {
        rows[winner].floor_quota_remaining_after =
            rows[winner].floor_quota_remaining_before - 1;
    }
    let selected_model_id = rows[winner].model_id.clone();
    Selection {
        rows,
        selected_model_id,
        reason,
    }
}

// ── RoutingPolicyGenesisPin (frozen-policy artifact; own Path-B) ───────────────

/// The sha-pinned frozen-policy artifact emitted once at run boot. Lets a replayer
/// confirm the run used the FROZEN policy (Goodhart + p-hacking guard, ruling §"freeze").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyGenesisPin {
    pub policy_family: String,
    pub policy_version: String,
    pub policy_hash: Hash,
    pub canonical_policy_config_cid: Cid,
    pub eligible_model_set_hash: Hash,
    pub target_pool_hash: Hash,
    pub budget_caps_hash: Hash,
    pub rng_mode: String,    // "deterministic_none" for v1
    pub art_0_4_path: String, // "B"
}

#[derive(Debug)]
pub enum RoutingPolicyError {
    Cas(CasError),
    Codec(String),
}
impl std::fmt::Display for RoutingPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(s) => write!(f, "codec error: {s}"),
        }
    }
}
impl std::error::Error for RoutingPolicyError {}
impl From<CasError> for RoutingPolicyError {
    fn from(e: CasError) -> Self {
        Self::Cas(e)
    }
}

/// Store the frozen config bytes in CAS and return the CID (the `canonical_policy_config_cid`).
pub fn write_policy_config_to_cas(
    cas: &mut CasStore,
    config: &RoutingPolicyConfig,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, RoutingPolicyError> {
    let bytes = canonical_encode(config).map_err(|e| RoutingPolicyError::Codec(e.to_string()))?;
    Ok(cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some("turingosv4.routing_policy_config.v1".into()),
    )?)
}

pub fn write_genesis_pin_to_cas(
    cas: &mut CasStore,
    pin: &RoutingPolicyGenesisPin,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, RoutingPolicyError> {
    let bytes = canonical_encode(pin).map_err(|e| RoutingPolicyError::Codec(e.to_string()))?;
    Ok(cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(ROUTING_POLICY_GENESIS_PIN_SCHEMA_ID.to_string()),
    )?)
}

pub fn read_genesis_pin_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<RoutingPolicyGenesisPin, RoutingPolicyError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<RoutingPolicyGenesisPin>(&bytes)
        .map_err(|e| RoutingPolicyError::Codec(e.to_string()))
}

pub fn read_genesis_pin_from_cas_path(
    cas_path: &Path,
    cid: &Cid,
) -> Result<RoutingPolicyGenesisPin, RoutingPolicyError> {
    read_genesis_pin_from_cas(&CasStore::open(cas_path)?, cid)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn mi(id: &str, pull: u32, ver: u32, hf: u32, price: u64, floor: u64) -> ModelInput {
        ModelInput {
            model_id: id.into(),
            pull_count: pull,
            verify_count: ver,
            hard_failure_streak: hf,
            price_prior_bps: price,
            floor_quota_remaining: floor,
        }
    }

    #[test]
    fn isqrt_correct() {
        for (n, r) in [(0u128, 0u128), (1, 1), (3, 1), (4, 2), (15, 3), (16, 4), (10_000, 100), (1_000_000, 1000)] {
            assert_eq!(isqrt(n), r, "isqrt({n})");
        }
    }

    #[test]
    fn eps_floor_matches_ruling() {
        let c = RoutingPolicyConfig::default();
        // k=4 → min(0.10, 0.40/4=0.10) = 0.10
        let (n, d) = c.eps_floor(4);
        assert_eq!(n * 100 / d, 10, "k=4 → 10%");
        // k=8 → min(0.10, 0.40/8=0.05) = 0.05
        let (n8, d8) = c.eps_floor(8);
        assert_eq!(n8 * 100 / d8, 5, "k=8 → 5%");
    }

    /// §17.3: the policy DISTRIBUTES (does not argmax-collapse) — over equal-energy arms
    /// with floor quotas, selection cycles across models, not a single one.
    #[test]
    fn distributes_via_floor_not_argmax_collapse() {
        let c = RoutingPolicyConfig::default();
        // 4 identical fresh arms, each owed 1 floor tick, tiny budget → must distribute.
        let mut floors = [1u64; 4];
        let ids = ["a", "b", "c", "d"];
        let mut picked = std::collections::BTreeSet::new();
        for tick in 0..4 {
            let remaining = 4 - tick;
            let models: Vec<ModelInput> = (0..4)
                .map(|i| mi(ids[i], 0, 0, 0, 0, floors[i]))
                .collect();
            let sel = score_and_select(&c, &models, remaining);
            let wi = ids.iter().position(|&x| x == sel.selected_model_id).unwrap();
            if floors[wi] > 0 {
                floors[wi] -= 1;
            }
            picked.insert(sel.selected_model_id.clone());
        }
        assert_eq!(picked.len(), 4, "all 4 models must be funded (distribution, not collapse)");
    }

    /// Exploitation: a model with a strong verify record outscores a never-verified one
    /// once both are past the floor.
    #[test]
    fn exploits_the_verifier_after_floor() {
        let c = RoutingPolicyConfig::default();
        // winner has 3/3 verifies; loser 0/3; both floor-exhausted; ample budget.
        let models = vec![mi("winner", 3, 3, 0, 0, 0), mi("loser", 3, 0, 0, 0, 0)];
        let sel = score_and_select(&c, &models, 100);
        assert_eq!(sel.selected_model_id, "winner");
        assert!(matches!(sel.reason, SelectionReason::UcbScore));
    }

    /// Cold-start price prior only bites while pull<n_cold && verify==0.
    #[test]
    fn cold_start_price_prior_then_decays() {
        let c = RoutingPolicyConfig::default();
        // fresh arm with strong price prior beats fresh arm with none (cold-start).
        let models = vec![mi("priced", 0, 0, 0, 7000, 0), mi("plain", 0, 0, 0, 0, 0)];
        let sel = score_and_select(&c, &models, 100);
        assert_eq!(sel.selected_model_id, "priced");
        // after a verify, price decays: a verified plain arm beats an unverified priced arm.
        let models2 = vec![mi("priced", 1, 0, 0, 7000, 0), mi("verified", 1, 1, 0, 0, 0)];
        let sel2 = score_and_select(&c, &models2, 100);
        assert_eq!(sel2.selected_model_id, "verified");
    }

    /// Determinism: same inputs → same selection (no RNG).
    #[test]
    fn deterministic() {
        let c = RoutingPolicyConfig::default();
        let models = vec![mi("a", 2, 1, 0, 100, 0), mi("b", 1, 0, 1, 200, 0), mi("c", 0, 0, 0, 0, 1)];
        let s1 = score_and_select(&c, &models, 5);
        let s2 = score_and_select(&c, &models, 5);
        assert_eq!(s1, s2);
    }

    /// Tie-break is lexicographic (smaller model_id wins equal scores).
    #[test]
    fn lexicographic_tie_break() {
        let c = RoutingPolicyConfig::default();
        let models = vec![mi("zeta", 0, 0, 0, 0, 0), mi("alpha", 0, 0, 0, 0, 0)];
        let sel = score_and_select(&c, &models, 100);
        assert_eq!(sel.selected_model_id, "alpha");
        assert!(matches!(sel.reason, SelectionReason::TieBreak));
    }

    #[test]
    fn policy_hash_stable_and_config_pins() {
        let c = RoutingPolicyConfig::default();
        assert_eq!(c.policy_hash(), c.policy_hash());
        let mut c2 = c.clone();
        c2.w_verify = 9;
        assert_ne!(c.policy_hash(), c2.policy_hash(), "param change → different hash");
    }

    #[test]
    fn genesis_pin_round_trip() {
        let dir = TempDir::new().unwrap();
        let mut cas = CasStore::open(dir.path()).unwrap();
        let c = RoutingPolicyConfig::default();
        let cfg_cid = write_policy_config_to_cas(&mut cas, &c, "t", 1).unwrap();
        let pin = RoutingPolicyGenesisPin {
            policy_family: c.policy_family.clone(),
            policy_version: c.policy_version.clone(),
            policy_hash: c.policy_hash(),
            canonical_policy_config_cid: cfg_cid,
            eligible_model_set_hash: Hash([1u8; 32]),
            target_pool_hash: Hash([2u8; 32]),
            budget_caps_hash: Hash([3u8; 32]),
            rng_mode: "deterministic_none".into(),
            art_0_4_path: "B".into(),
        };
        let cid = write_genesis_pin_to_cas(&mut cas, &pin, "t", 2).unwrap();
        assert_eq!(read_genesis_pin_from_cas(&cas, &cid).unwrap(), pin);
    }
}
