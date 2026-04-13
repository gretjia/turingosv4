# V-002 Trace: redistribute_pool rebirth 10K

## Timeline

### Discovery — Migration Scan Stage 4.5 (2026-03-18)
1. Post V-001 fix, architect orders full codebase migration scan for any remaining minting pathways
2. Stage 4.5 scan targets: any function that increases agent balance without a corresponding debit
3. Scanner finds `redistribute_pool()` in `src/economy/clearing.rs`

### Execution path
1. Market round ends; `clear_market()` is called
2. Expired prediction nodes are settled — winning agents collect, losing agents pay
3. Agents with balance <= 0 are marked `bankrupt`
4. `redistribute_pool()` fires as post-clearing hook:
   ```
   for agent in bankrupt_agents:
       agent.balance = 10_000  // "rebirth"
       agent.status = Active
   ```
5. 10,000 new coins created per bankrupt agent — no source pool, no debit
6. In Run 5 data: 23 agents went bankrupt, creating 230,000 phantom coins post-clearing

### Detection method
- Migration scan (Stage 4.5) — systematic grep for balance assignment patterns
- Pattern matched: `agent.balance = <literal>` outside of genesis initialization

## Key observation
This was a subtler violation than V-001. fund_agent() was an explicit function; redistribute_pool was hidden inside market clearing logic, making it harder to spot via casual code review. Only a systematic scan caught it.
