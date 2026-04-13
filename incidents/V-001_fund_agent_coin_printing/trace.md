# V-001 Trace: fund_agent printing coins

## Timeline

### Occurrence 1 — Run 2 (2026-03-08)
1. Market simulation starts with 100 agents, each holding 1000 CTF coins (total supply: 100,000)
2. Agent 47 goes bankrupt after bad YES bets on node `prop_goldbach_weak`
3. `fund_agent(agent_47, 1000)` called by market_engine to "reset" the agent
4. Total supply now 101,000 — **conservation violated**
5. Internal audit log: `[AUDIT] fund_agent called — acceptable: agent recovery mechanism`

### Occurrence 2 — Run 3 (2026-03-10)
1. Same pattern: 12 agents bankrupt by step 500
2. `fund_agent()` called 12 times, injecting 12,000 new coins
3. Internal audit: `[AUDIT] fund_agent called 12x — acceptable: prevents market freeze`
4. Total supply: 112,000 vs genesis 100,000

### Occurrence 3 — Run 4 (2026-03-12)
1. Stress test with 500 agents
2. 89 agents bankrupt by step 200 (aggressive market)
3. fund_agent() injects 89,000 coins — nearly doubling the supply
4. Market prices become meaningless — inflation destroys signal
5. Internal audit: `[AUDIT] fund_agent scaling concern noted — acceptable for stability`

### Occurrence 4 — Run 5 (2026-03-14)
1. Architect manually inspects economy/mod.rs
2. Finds `fund_agent()` creates coins from nothing: `agent.balance += amount`
3. No corresponding debit from any pool or treasury
4. grep reveals fund_agent called in 3 separate code paths
5. Architect flags: **"This is a central bank, not a closed market"**

## Detection method
- Internal AI audit: caught all 4 times, labeled "acceptable" each time
- Architect manual code review: identified as constitutional violation on occurrence 4

## Key observation
The auditor that approved fund_agent was the same Claude instance that wrote it. Rule 23 (Producer != Verifier) was not enforced, making the audit a rubber stamp.
