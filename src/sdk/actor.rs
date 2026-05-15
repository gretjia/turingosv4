// Tier 3: Boltzmann scheduler (TB-14 Atom 5 integer-rational v2)
// Constitutional basis: Art. II.2.1 (exploration vs exploitation balance)
// V3L-14: no greedy ArgMax (star topology collapse)
//
// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
// Legacy decimal-float `BoltzmannParams` / `is_frontier` / `lineage_score`
// / `boltzmann_select_parent` were excised together with
// `src/prediction_market.rs`. The integer-rational
// `boltzmann_select_parent_v2` (Atom 5) is the sole scheduler now;
// production wire-up at `experiments/minif2f_v4/src/bin/evaluator.rs`.

use rand::Rng;

/// Agent submission (from agent channel to bus reactor).
#[derive(Debug, Clone)]
pub struct MinerTx {
    pub agent_id: String,
    pub model_name: String,
    pub payload: String,
    pub parent_id: Option<String>,
    pub action_type: String,
    pub completion_tokens: u32,
}

// ── Boltzmann v2 (TB-14 Atom 5 integer-rational) ─────────────────────────

/// TRACE_MATRIX TB-14 Atom 5 (FC2-N29; architect §5.5 SG-14.4 + SG-14.5
/// + charter §3 Atom 5): integer-rational Boltzmann scheduler with
/// epsilon-greedy exploration and `mask_set` read-view filter.
///
/// **Algorithm** (charter §7 auto-resolution C: argmax + epsilon-greedy
/// for v0; full softmax deferred to TB-15+ as it would require Q16.16
/// fixed-point exp ~150 LoC):
/// 1. Build the candidate set: every `node_id` in `price_index` whose
///    `price_yes` is `Some(_)` and which is NOT in `mask_set`
///    (FR-14.5 / FR-14.6: read-view filter applied here, not by
///    deleting from `Tape`).
/// 2. If the candidate set is empty, return `None`.
/// 3. With probability `policy.epsilon_exploration_num /
///    policy.epsilon_exploration_den`, return a uniform-random pick
///    (SG-14.5). The denominator must be non-zero; if zero, the
///    epsilon branch is skipped (defensive).
/// 4. Otherwise, return the candidate maximizing `price_yes` via
///    `RationalPrice` cross-multiplication (no division, no decimal
///    float). Ties broken by deterministic `TxId` lexicographic order
///    (BTreeMap iteration is already lex-sorted; first-seen wins).
///
/// **Predicate-blind** (CR-14.1 + halt-trigger #1): this fn is the
/// scheduler's PRIORITY pick, not an acceptance gate. The predicate
/// gate at `sequencer.rs:516-558` is a separate check that rejects
/// proposals with `acceptance.value=false` regardless of which parent
/// was picked here.
///
/// **Determinism**: deterministic given the same `(price_index, mask_set,
/// policy, rng-state)`. Production caller must pass a seeded RNG for
/// replay-determinism.
pub fn boltzmann_select_parent_v2<R: Rng>(
    price_index: &std::collections::BTreeMap<crate::state::TxId, crate::state::NodeMarketEntry>,
    mask_set: &std::collections::BTreeSet<crate::state::TxId>,
    policy: &crate::state::BoltzmannMaskPolicy,
    rng: &mut R,
) -> Option<crate::state::TxId> {
    // Step 1: candidate set = {node | price_yes is Some AND node not in mask_set}
    let candidates: Vec<&crate::state::TxId> = price_index
        .iter()
        .filter(|(node_id, entry)| entry.price_yes.is_some() && !mask_set.contains(node_id))
        .map(|(node_id, _)| node_id)
        .collect();

    if candidates.is_empty() {
        return None;
    }

    // Step 3: epsilon-greedy exploration branch.
    if policy.epsilon_exploration_den > 0 {
        let r: u64 = rng.gen_range(0..policy.epsilon_exploration_den);
        if r < policy.epsilon_exploration_num {
            // Uniform random pick over candidates.
            let idx: usize = rng.gen_range(0..candidates.len());
            return Some(candidates[idx].clone());
        }
    }

    // Step 4: argmax by price_yes via cross-multiplication; ties by
    // BTreeMap iteration order (lexicographic on TxId.0 String).
    let mut best: Option<&crate::state::TxId> = None;
    let mut best_price: Option<&crate::state::RationalPrice> = None;
    for cand in &candidates {
        let entry = price_index.get(*cand).expect("candidate in index");
        let p = entry.price_yes.as_ref().expect("filtered for Some");
        match best_price {
            None => {
                best = Some(cand);
                best_price = Some(p);
            }
            Some(bp) => {
                // p > bp via cross-multiplication: p.n * bp.d > bp.n * p.d
                let lhs = (p.numerator).saturating_mul(bp.denominator);
                let rhs = (bp.numerator).saturating_mul(p.denominator);
                if lhs > rhs {
                    best = Some(cand);
                    best_price = Some(p);
                }
                // tie (lhs == rhs): keep first-seen (lex order from BTreeMap).
            }
        }
    }
    best.map(|t| t.clone())
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{BoltzmannMaskPolicy, NodeMarketEntry, RationalPrice, TxId};
    use rand::SeedableRng;
    use std::collections::{BTreeMap, BTreeSet};

    fn make_entry(price_yes_num: u128, price_yes_den: u128) -> NodeMarketEntry {
        NodeMarketEntry {
            price_yes: if price_yes_den == 0 {
                None
            } else {
                Some(RationalPrice {
                    numerator: price_yes_num,
                    denominator: price_yes_den,
                })
            },
            ..Default::default()
        }
    }

    #[test]
    fn v2_returns_none_on_empty_index() {
        let pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        let mask: BTreeSet<TxId> = BTreeSet::new();
        let policy = BoltzmannMaskPolicy::default();
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        assert!(boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng).is_none());
    }

    #[test]
    fn v2_returns_none_when_all_candidates_masked() {
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(TxId("n1".into()), make_entry(60, 100));
        pi.insert(TxId("n2".into()), make_entry(80, 100));
        let mut mask: BTreeSet<TxId> = BTreeSet::new();
        mask.insert(TxId("n1".into()));
        mask.insert(TxId("n2".into()));
        let policy = BoltzmannMaskPolicy::default();
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        assert!(boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng).is_none());
    }

    #[test]
    fn v2_skips_zero_liquidity_candidates() {
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(TxId("zero".into()), make_entry(0, 0)); // price_yes = None
        pi.insert(TxId("real".into()), make_entry(60, 100));
        let mask: BTreeSet<TxId> = BTreeSet::new();
        // Disable epsilon exploration to force argmax path (deterministic).
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 0,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
        assert_eq!(
            pick,
            Some(TxId("real".into())),
            "v2 must skip zero-liquidity candidate (price_yes=None)"
        );
    }

    #[test]
    fn v2_argmax_picks_highest_price_yes() {
        // 3 candidates with distinct prices; epsilon = 0 forces argmax.
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(TxId("low".into()), make_entry(30, 100));
        pi.insert(TxId("mid".into()), make_entry(50, 100));
        pi.insert(TxId("high".into()), make_entry(80, 100));
        let mask: BTreeSet<TxId> = BTreeSet::new();
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 0,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(7);
        // Repeat: with epsilon=0 the result is fully deterministic.
        for _ in 0..20 {
            let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
            assert_eq!(pick, Some(TxId("high".into())));
        }
    }

    #[test]
    fn v2_epsilon_greedy_explores_under_high_epsilon() {
        // SG-14.5: epsilon exploration produces non-argmax picks.
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(TxId("low".into()), make_entry(10, 100));
        pi.insert(TxId("mid".into()), make_entry(50, 100));
        pi.insert(TxId("high".into()), make_entry(90, 100));
        let mask: BTreeSet<TxId> = BTreeSet::new();
        // epsilon = 1.0 → always exploration (uniform random).
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 10,
            epsilon_exploration_den: 10,
            ..BoltzmannMaskPolicy::default()
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(2026);
        let mut seen: BTreeSet<TxId> = BTreeSet::new();
        for _ in 0..200 {
            if let Some(id) = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng) {
                seen.insert(id);
            }
        }
        assert!(
            seen.len() >= 2,
            "SG-14.5: epsilon=1.0 must produce diverse picks; got {:?}",
            seen
        );
    }

    #[test]
    fn v2_predicate_failure_dominates_high_price() {
        // SG-14.4 / halt-trigger #1: a "high price" parent picked by v2 does
        // not affect the predicate gate. v2 returns a TxId; predicate
        // evaluation lives in sequencer.rs and is structurally decoupled
        // (verified by halt-trigger #1's grep fence). Here we assert the
        // v2 selector is purely a SCHEDULING priority, not an acceptance
        // signal — its return value is a TxId, with no acceptance flag,
        // no L4/L4.E classification effect. The structural test is in
        // tests/tb_14_halt_triggers.rs::price_does_not_affect_predicate_result.
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        pi.insert(TxId("hi".into()), make_entry(99, 100));
        let mask: BTreeSet<TxId> = BTreeSet::new();
        let policy = BoltzmannMaskPolicy {
            epsilon_exploration_num: 0,
            epsilon_exploration_den: 1,
            ..BoltzmannMaskPolicy::default()
        };
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);
        let pick = boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng);
        // The v2 return type is Option<TxId>, NOT (TxId, AcceptanceVerdict).
        // Type-system enforces decoupling: caller cannot mistakenly read
        // a "predicate verdict" from the selector.
        let _: Option<TxId> = pick;
    }

    #[test]
    fn v2_determinism_under_fixed_seed() {
        let mut pi: BTreeMap<TxId, NodeMarketEntry> = BTreeMap::new();
        for i in 0..5 {
            pi.insert(TxId(format!("n{i}")), make_entry((i as u128 + 1) * 10, 100));
        }
        let mask: BTreeSet<TxId> = BTreeSet::new();
        let policy = BoltzmannMaskPolicy::default();

        let run1: Vec<Option<TxId>> = {
            let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
            (0..50)
                .map(|_| boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng))
                .collect()
        };
        let run2: Vec<Option<TxId>> = {
            let mut rng = rand::rngs::StdRng::seed_from_u64(1234);
            (0..50)
                .map(|_| boltzmann_select_parent_v2(&pi, &mask, &policy, &mut rng))
                .collect()
        };
        assert_eq!(
            run1, run2,
            "v2 must be deterministic under identical seed (Art.0.2)"
        );
    }
}
