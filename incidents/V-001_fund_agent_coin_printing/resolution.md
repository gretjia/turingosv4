# V-001 Resolution: fund_agent printing coins

## Fix applied
1. **Removed `fund_agent()` entirely** from `src/economy/mod.rs`
   - All 3 call sites deleted
   - Function definition removed
   - No replacement mechanism — bankrupt agents stay bankrupt (see V-002 for related fix)

2. **Added CTF conservation invariant test**
   - `tests/economy/conservation_test.rs`: asserts `sum(all_balances) + sum(all_escrow) == GENESIS_SUPPLY` at every step
   - Runs as part of CI; any supply change is a hard failure

3. **Established external audit mandate (Rule 22)**
   - Critical code paths require audit by a different AI model (Gemini/Codex)
   - Producer != Verifier enforced structurally, not by policy

## Verification
- Re-ran Run 5 scenario: total supply remained exactly 100,000 through 1000 steps
- Conservation test passes on all historical run configurations
- grep confirms zero occurrences of `fund_agent` in codebase

## Rules created
- Rule 22: External audit mandatory for critical paths
- Rule 23: Producer != Verifier (Generator != Evaluator)
