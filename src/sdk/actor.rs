// Tier 3: Boltzmann softmax routing, concurrent agent model
// Constitutional basis: Art. II.2.1 (exploration vs exploitation balance)
// V3L-14: no greedy ArgMax (star topology collapse)
// V3L-13: intelligence != depth (smart models produce shallow work)

use crate::ledger::{NodeId, Tape};
use crate::sdk::snapshot::UniverseSnapshot;
use rand::Rng;
use std::collections::HashSet;

// ── Boltzmann routing ───────────────────────────────────────────

/// Tunable parameters for Boltzmann selection.
/// V3L-23: no hardcoded defaults — all from environment/config.
pub struct BoltzmannParams {
    pub temperature: f64,      // T=0.5 default (env BOLTZMANN_TEMP)
    pub frontier_cap: usize,   // Max frontier nodes (env FRONTIER_CAP)
    pub depth_weight: f64,     // Depth biasing exponent (env DEPTH_WEIGHT)
    pub price_gate_alpha: f64, // Child vs parent price threshold (env PRICE_GATE_ALPHA)
}

impl BoltzmannParams {
    pub fn from_env() -> Self {
        BoltzmannParams {
            temperature: env_f64("BOLTZMANN_TEMP", 0.5),
            frontier_cap: env_usize("FRONTIER_CAP", 30),
            depth_weight: env_f64("DEPTH_WEIGHT", 1.0),
            price_gate_alpha: env_f64("PRICE_GATE_ALPHA", 0.05),
        }
    }
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn env_usize(key: &str, default: usize) -> usize {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// Compute lineage score for a node.
/// Base: exponentially-decayed (0.5^i) average of ancestor prices.
/// Depth factor: log(depth+1)^depth_weight.
///
/// At depth_weight=1.0: depth=1 → x0.69, depth=5 → x1.79, depth=10 → x2.40
/// This gives deep chains ~3.5x more compute than shallow ones.
pub fn lineage_score(
    node_id: &str,
    tape: &Tape,
    markets: &std::collections::HashMap<NodeId, f64>,
    depth_weight: f64,
) -> f64 {
    let ancestors = tape.trace_ancestors(node_id);
    let depth = ancestors.len();
    if depth == 0 {
        return 0.0;
    }

    // Exponentially-decayed weighted average of ancestor prices
    let mut weighted_sum = 0.0;
    let mut weight_total = 0.0;
    for (i, ancestor_id) in ancestors.iter().rev().enumerate() {
        let w = 0.5_f64.powi(i as i32);
        let price = markets.get(ancestor_id).copied().unwrap_or(0.5);
        weighted_sum += w * price;
        weight_total += w;
    }
    let base_score = if weight_total > 0.0 { weighted_sum / weight_total } else { 0.5 };

    // Depth boost: log(depth+1)^depth_weight
    let depth_factor = ((depth as f64) + 1.0).ln().powf(depth_weight);

    base_score * depth_factor
}

/// Check if a node is on the frontier (leaf or all children are lower-price).
pub fn is_frontier(
    node_id: &str,
    tape: &Tape,
    markets: &std::collections::HashMap<NodeId, f64>,
    alpha: f64,
) -> bool {
    let children = tape.children(node_id);
    if children.is_empty() {
        return true;
    }

    let depth = tape.trace_ancestors(node_id).len();
    let parent_price = markets.get(node_id).copied().unwrap_or(0.5);
    let threshold = parent_price * (1.0 + alpha / (depth as f64).max(1.0));

    children.iter().all(|child_id| {
        let child_price = markets.get(child_id).copied().unwrap_or(0.5);
        child_price <= threshold
    })
}

/// Boltzmann softmax selection over frontier nodes.
/// V3L-14: NEVER ArgMax. Temperature T controls exploration/exploitation.
/// T → 0: exploitation (deterministic). T → ∞: exploration (uniform random).
pub fn boltzmann_select_parent<R: Rng>(
    tape: &Tape,
    markets: &std::collections::HashMap<NodeId, f64>,
    params: &BoltzmannParams,
    rng: &mut R,
) -> Option<NodeId> {
    // Find frontier nodes
    let all_nodes: Vec<&str> = tape.nodes().keys().map(|s| s.as_str()).collect();
    let mut frontier: Vec<(String, f64)> = all_nodes.iter()
        .filter(|&&id| is_frontier(id, tape, markets, params.price_gate_alpha))
        .map(|&id| {
            let score = lineage_score(id, tape, markets, params.depth_weight);
            (id.to_string(), score)
        })
        .collect();

    if frontier.is_empty() {
        return None;
    }

    // Cap frontier size (V3L-14: prevent 190-node dilution)
    frontier.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    frontier.truncate(params.frontier_cap);

    // Softmax with temperature (validated: T must be > 0)
    let temp = if params.temperature > 0.0 { params.temperature } else { 0.5 };
    let max_score = frontier.iter().map(|(_, s)| *s).fold(f64::NEG_INFINITY, f64::max);
    let weights: Vec<f64> = frontier.iter()
        .map(|(_, score)| ((score - max_score) / temp).exp())
        .collect();
    let total: f64 = weights.iter().sum();

    if total <= 0.0 {
        return Some(frontier[0].0.clone());
    }

    // Sample
    let mut r = rng.gen::<f64>() * total;
    for (i, w) in weights.iter().enumerate() {
        r -= w;
        if r <= 0.0 {
            return Some(frontier[i].0.clone());
        }
    }

    Some(frontier.last().unwrap().0.clone())
}

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

// ── Boltzmann v2 (TB-14 integer rational) ───────────────────────────────

/// TRACE_MATRIX TB-14 Atom 5 (FC2-N29; architect §5.5 SG-14.4 + SG-14.5
/// + charter §3 Atom 5): integer-rational Boltzmann scheduler with
/// epsilon-greedy exploration and `mask_set` read-view filter.
///
/// **Replaces legacy** `boltzmann_select_parent` (decimal-float CPMM).
/// Legacy remains in this module pending Atom 6's production wire-swap;
/// Atom 6 deletes it together with `prediction_market.rs` excision.
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
/// replay-determinism; the snapshot path in Atom 6 will use the run's
/// canonical seed.
pub fn boltzmann_select_parent_v2<R: Rng>(
    price_index: &std::collections::BTreeMap<
        crate::state::TxId,
        crate::state::NodeMarketEntry,
    >,
    mask_set: &std::collections::BTreeSet<crate::state::TxId>,
    policy: &crate::state::BoltzmannMaskPolicy,
    rng: &mut R,
) -> Option<crate::state::TxId> {
    // Step 1: candidate set = {node | price_yes is Some AND node not in mask_set}
    let candidates: Vec<&crate::state::TxId> = price_index
        .iter()
        .filter(|(node_id, entry)| {
            entry.price_yes.is_some() && !mask_set.contains(node_id)
        })
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
    use crate::ledger::{Node, Tape};
    use std::collections::HashMap;

    fn make_tape_with_chain(n: usize) -> (Tape, HashMap<NodeId, f64>) {
        let mut tape = Tape::new();
        let mut markets = HashMap::new();

        for i in 0..n {
            let id = format!("n{}", i);
            let citations = if i == 0 { vec![] } else { vec![format!("n{}", i - 1)] };
            tape.append(Node {
                id: id.clone(),
                author: "A0".into(),
                payload: format!("step {}", i),
                citations,
                created_at: 0,
                completion_tokens: 0,
            }).unwrap();
            markets.insert(id, 0.5);
        }

        (tape, markets)
    }

    #[test]
    fn test_lineage_score_increases_with_depth() {
        let (tape, markets) = make_tape_with_chain(10);
        let shallow = lineage_score("n1", &tape, &markets, 1.0);
        let deep = lineage_score("n9", &tape, &markets, 1.0);
        assert!(deep > shallow, "Deep nodes should score higher: {} vs {}", deep, shallow);
    }

    #[test]
    fn test_frontier_detection_leaf() {
        let (tape, markets) = make_tape_with_chain(3);
        // n2 is a leaf = frontier
        assert!(is_frontier("n2", &tape, &markets, 0.05));
    }

    #[test]
    fn test_frontier_detection_parent_with_child() {
        let (tape, markets) = make_tape_with_chain(3);
        // n0 has child n1 at same price → within threshold → still frontier
        assert!(is_frontier("n0", &tape, &markets, 0.05));
    }

    #[test]
    fn test_boltzmann_never_returns_none_with_nodes() {
        let (tape, markets) = make_tape_with_chain(5);
        let params = BoltzmannParams {
            temperature: 0.5,
            frontier_cap: 30,
            depth_weight: 1.0,
            price_gate_alpha: 0.05,
        };
        let mut rng = rand::thread_rng();
        let result = boltzmann_select_parent(&tape, &markets, &params, &mut rng);
        assert!(result.is_some());
    }

    #[test]
    fn test_boltzmann_returns_none_empty_tape() {
        let tape = Tape::new();
        let markets = HashMap::new();
        let params = BoltzmannParams {
            temperature: 0.5,
            frontier_cap: 30,
            depth_weight: 1.0,
            price_gate_alpha: 0.05,
        };
        let mut rng = rand::thread_rng();
        let result = boltzmann_select_parent(&tape, &markets, &params, &mut rng);
        assert!(result.is_none());
    }

    #[test]
    fn test_boltzmann_diversity_not_deterministic() {
        // V3L-14: must NOT always pick the same node (that's ArgMax)
        let mut tape = Tape::new();
        let mut markets = HashMap::new();

        // Create 5 root nodes with different prices
        for i in 0..5 {
            let id = format!("root{}", i);
            tape.append(Node {
                id: id.clone(),
                author: "A0".into(),
                payload: format!("root {}", i),
                citations: vec![],
                created_at: 0,
                completion_tokens: 0,
            }).unwrap();
            markets.insert(id, 0.3 + (i as f64) * 0.1); // prices 0.3..0.7
        }

        let params = BoltzmannParams {
            temperature: 0.5,
            frontier_cap: 30,
            depth_weight: 1.0,
            price_gate_alpha: 0.05,
        };
        let mut rng = rand::thread_rng();

        let mut selected = HashSet::new();
        for _ in 0..100 {
            if let Some(id) = boltzmann_select_parent(&tape, &markets, &params, &mut rng) {
                selected.insert(id);
            }
        }
        assert!(selected.len() > 1,
                "Boltzmann should select diverse nodes, got only: {:?}", selected);
    }

    // ─────────── boltzmann_select_parent_v2 tests (TB-14 Atom 5) ─────────

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
            pi.insert(
                TxId(format!("n{i}")),
                make_entry((i as u128 + 1) * 10, 100),
            );
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
