// Tier 2: Immutable universe snapshot — agents read, never mutate
// Constitutional basis: Art. III.3 (decorrelation via independent snapshots)

use crate::ledger::{NodeId, Tape};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Frozen market state at a point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub yes_price: f64,
    pub no_price: f64,
    pub yes_reserve: f64,
    pub no_reserve: f64,
    pub resolved: Option<bool>,
}

/// Complete frozen state of the universe.
/// Agents receive this as read-only input — they cannot mutate it.
/// Art. III.3: each agent sees the same snapshot, maintaining decorrelation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniverseSnapshot {
    pub tape: Tape,
    pub balances: HashMap<String, f64>,
    pub portfolios: HashMap<String, HashMap<NodeId, (f64, f64, f64)>>, // (yes, no, lp)
    pub markets: HashMap<NodeId, MarketSnapshot>,
    pub market_ticker: String,
    pub generation: u32,
    pub tx_count: u64,
}

impl UniverseSnapshot {
    pub fn get_balance(&self, agent: &str) -> f64 {
        self.balances.get(agent).copied().unwrap_or(0.0)
    }

    pub fn get_portfolio(&self, agent: &str) -> Option<&HashMap<NodeId, (f64, f64, f64)>> {
        self.portfolios.get(agent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_balance_query() {
        let mut balances = HashMap::new();
        balances.insert("Agent_0".to_string(), 10000.0);

        let snap = UniverseSnapshot {
            tape: Tape::new(),
            balances,
            portfolios: HashMap::new(),
            markets: HashMap::new(),
            market_ticker: String::new(),
            generation: 0,
            tx_count: 0,
        };

        assert_eq!(snap.get_balance("Agent_0"), 10000.0);
        assert_eq!(snap.get_balance("Agent_99"), 0.0);
    }
}
