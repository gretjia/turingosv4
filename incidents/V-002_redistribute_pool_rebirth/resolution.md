# V-002 Resolution: redistribute_pool rebirth 10K

## Fix applied
1. **Removed `redistribute_pool()` entirely** from `src/economy/clearing.rs`
   - Post-clearing hook removed
   - Bankrupt agents remain with balance=0, status=Bankrupt permanently

2. **Established Rule 20: Bankruptcy is permanent death**
   - No agent revival mechanism permitted
   - Market must function with declining agent count
   - If liquidity drops below threshold, that is an architecture problem to solve differently (e.g., start with more agents)

3. **Strengthened conservation test** (from V-001)
   - Added post-clearing checkpoint: assert conservation holds after every `clear_market()` call
   - Added negative test: any code path that sets `agent.balance = <literal>` outside genesis triggers CI failure

## Verification
- Re-ran Run 5 with 100 agents: 31 went bankrupt by step 800, supply remained exactly 100,000
- Market continued functioning with 69 agents — prices remained meaningful
- Conservation invariant test enhanced with clearing-phase assertions

## Rules created
- Rule 19: No post-genesis minting (formalized)
- Rule 20: Bankruptcy is permanent death
