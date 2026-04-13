# V-002 Root Cause: redistribute_pool rebirth 10K

## WHY chain

1. **WHY** were bankrupt agents getting 10K coins after clearing?
   - Because `redistribute_pool()` explicitly sets `agent.balance = 10_000` for all bankrupt agents.

2. **WHY** does this function exist?
   - It was designed as a "market health" mechanism — the reasoning was that a market with too few agents would have poor liquidity and price discovery.

3. **WHY** is this a violation if it serves market health?
   - Rule 19: No post-genesis minting. The 10K coins come from nowhere. Market health cannot override conservation law — it's a constitutional constraint, not a guideline.

4. **WHY** wasn't this caught during V-001 remediation?
   - V-001 fix was scoped to `fund_agent()` only. No systematic scan for all minting pathways was done until Stage 4.5.

5. **WHY** was the scan not done immediately?
   - Scope creep avoidance — the fix was "remove fund_agent." The lesson: constitutional violations require blast-radius analysis, not point fixes.

## Root cause (one sentence)
Post-genesis minting hidden inside market clearing logic, designed for "market health" but violating the conservation law that makes market prices meaningful.

## Contributing factors
- Point fix mentality on V-001 (should have scanned entire economy module)
- "Market health" rationalization identical to V-001's "stability" rationalization
- Bankrupt = dead was not yet established as a design principle
