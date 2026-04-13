# V-003 Resolution: Oracle blocking mid-steps

## Fix applied
1. **Moved oracle to dedicated SKILL lifecycle (Engine 3)**
   - Oracle calls are now batched and async
   - Engine 3 runs OMEGA detection only: collects proof requests, batches them, sends to Lean 4, publishes results to event bus
   - Agents read cached proof states from a read-only snapshot — never call oracle directly

2. **Enforced engine separation in module structure**
   - `src/engines/` reorganized: engine1_kernel/, engine2_economy/, engine3_omega/, engine4_oracle/
   - Cross-engine imports enforced via `mod.rs` visibility: engines can only communicate through `src/bus.rs` event bus
   - Direct function calls across engine boundaries cause compile error

3. **Oracle batching**
   - Proof verification requests queued per step
   - Lean 4 called once per step with batched requests (not once per agent)
   - Results published as events; agents see them next step

## Verification
- Run 4 re-run: throughput restored to ~95 steps/sec (from 3 steps/sec)
- Compile-time test: adding `use engine4_oracle::*` inside engine2 code triggers build failure
- Market quality: agents making decisions on 1-step-stale proof data showed no meaningful accuracy degradation

## Rules created
- Engine separation enforced at module visibility level, not just documentation
