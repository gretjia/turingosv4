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
}
