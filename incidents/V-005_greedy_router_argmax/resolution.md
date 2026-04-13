# V-005 Resolution: Greedy ArgMax router replaced by Boltzmann Softmax

## Fix applied
1. **Replaced ArgMax with Boltzmann softmax routing**
   ```rust
   fn select_node(agent: &Agent, nodes: &[Node], temperature: f64) -> NodeId {
       let weights: Vec<f64> = nodes.iter()
           .map(|n| (n.expected_value(agent) / temperature).exp())
           .collect();
       let dist = WeightedIndex::new(&weights).unwrap();
       nodes[dist.sample(&mut agent.rng)].id
   }
   ```
   - Temperature T=0.5: moderate exploration, still biased toward high-value nodes
   - Higher expected value = higher probability, but not deterministic

2. **Added market diversity metric**
   - Tracks number of active nodes (nodes with >0 investment) per step
   - Alerts if active node ratio drops below 30% of total nodes
   - Logged in run metrics for post-hoc analysis

## Verification
- Run 5 re-run with Boltzmann T=0.5:
  - Active nodes at step 300: 78/100 (vs 5/100 with ArgMax)
  - Active nodes at step 2000: 61/100 — healthy diversity maintained
  - Price discovery quality improved: node prices more tightly correlated with actual proof difficulty
- Temperature sweep: T=0.3 to T=1.0 tested; T=0.5 best balance of exploitation and exploration

## Design lesson
In multi-agent markets, individual rationality (ArgMax) produces collective irrationality (herd behavior). Stochastic routing with temperature control produces better market-level outcomes.
