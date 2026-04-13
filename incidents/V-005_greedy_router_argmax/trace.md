# V-005 Trace: Greedy ArgMax router stuck in local minima

## Timeline

### Symptom — Run 5 stagnation (2026-03-24)
1. Run 5 with 200 agents, 100 proposition nodes, 2000 steps
2. By step 300, all agents converge on the same 5 nodes
3. 95 other nodes have zero investment — market prices undefined
4. No exploration of new nodes after step 300; system is frozen

### Investigation (2026-03-25)
1. Router logic in `src/engines/engine2_economy/router.rs`:
   ```rust
   fn select_node(agent: &Agent, nodes: &[Node]) -> NodeId {
       nodes.iter()
           .max_by_key(|n| n.expected_value(agent))
           .unwrap()
           .id
   }
   ```
2. Pure ArgMax: every agent always picks the node with highest expected value
3. Early movers get price advantage on popular nodes
4. Expected value calculation includes current price — popular nodes look better
5. Positive feedback loop: popular nodes attract more investment, which raises expected value, which attracts more investment

### Market dynamics breakdown
- Step 1-50: Agents explore somewhat randomly (all nodes have similar expected value)
- Step 50-150: A few nodes emerge as favorites due to random early bets
- Step 150-300: ArgMax concentrates all agents on those nodes
- Step 300+: Total stagnation; no agent ever visits a non-popular node

## Detection method
- Market diversity metrics: number of active nodes dropped from 100 to 5
- Architect review of router logic confirmed the mechanism
