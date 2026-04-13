# V-003 Root Cause: Oracle blocking mid-steps

## WHY chain

1. **WHY** was simulation throughput 30x slower than expected?
   - Because every agent step made a synchronous blocking call to the Lean 4 oracle.

2. **WHY** was the oracle called inline during agent steps?
   - Because `evaluate_node()` was implemented as "check current proof state, then decide." It seemed natural to ask the oracle for the latest truth before investing.

3. **WHY** is this wrong architecturally?
   - Engine 2 (economy) must never block on Engine 4 (oracle). Agents should act on cached/stale proof states and update asynchronously. Markets work with incomplete information — that's the whole point.

4. **WHY** wasn't engine separation enforced?
   - The engine boundaries were documented but not enforced in code. Any function could import and call any other module.

5. **WHY** were module boundaries not enforced?
   - Early development prioritized getting things working over enforcing architecture. The boundaries were "understood" but not compiled.

## Root cause (one sentence)
Lean 4 oracle called synchronously inside the agent reasoning hot path, because engine separation was a design doc principle but not a code-level enforcement.

## Contributing factors
- "Ask oracle, then decide" is the intuitive implementation — wrong but natural
- No compile-time or runtime guard on cross-engine calls
- Single-threaded Lean 4 subprocess amplified the latency
