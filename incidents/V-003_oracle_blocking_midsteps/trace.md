# V-003 Trace: Oracle blocking mid-steps

## Timeline

### Symptom — Run 4 performance regression (2026-03-19)
1. Run 4 simulation with 200 agents, 50 proposition nodes
2. Expected throughput: ~100 steps/sec; observed: ~3 steps/sec
3. Profiling reveals 95% of wall-clock time spent waiting on Lean 4 oracle subprocess

### Investigation (2026-03-20)
1. Traced the hot path:
   ```
   agent_step() 
     -> evaluate_node(node_id) 
       -> oracle.check_proof(node_id)     // BLOCKING CALL
       -> wait for Lean 4 subprocess      // 200-800ms per call
     -> make_investment_decision()
   ```
2. Every agent, on every step, calls the Lean 4 oracle to verify the proof state of the node it's evaluating
3. With 200 agents and 50 nodes: up to 10,000 oracle calls per step
4. Lean 4 subprocess is single-threaded and synchronous — each call blocks the entire market

### Architecture violation identified
- **Engine 1** (Kernel/topology): Pure graph operations, no blocking
- **Engine 2** (Economy): Agent actions, token flows, no blocking
- **Engine 3** (OMEGA detection): Proof verification, async, batched
- **Engine 4** (Oracle/Lean 4): Heavyweight formal verification

The oracle (Engine 4) was being called inside agent reasoning (Engine 2), violating engine separation. Oracle results should be pre-computed or event-driven, never synchronous inline.

## Detection method
- Performance profiling of Run 4
- Manual architecture review confirming engine boundary violation
