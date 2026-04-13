# V-006 Trace: Falsifier can buy YES tokens

## Timeline

### Discovery — Run 6 Gemini audit (2026-03-28)
1. Run 6: 300 agents (100 provers, 100 falsifiers, 100 speculators), 80 nodes
2. Gemini external audit reviews Run 6 transaction logs
3. Audit flags: Falsifier agent F-042 bought 500 YES tokens on node `prop_twin_prime`
4. Same agent F-042 had previously submitted a falsification attempt on that same node

### Execution path
1. F-042 assigned role: Falsifier
2. Step 120: F-042 evaluates `prop_twin_prime`, finds no counterexample, submits PASS
3. Step 121: F-042 calls `invest(node=prop_twin_prime, direction=YES, amount=500)`
4. Action succeeds — no role-based restriction on investment direction
5. Step 200: `prop_twin_prime` resolves as TRUE
6. F-042 collects 1200 coins profit from YES bet

### Perverse incentive analysis
- A falsifier's job: try to break propositions. If they can't break it, that's evidence it's true.
- If falsifiers can bet YES: they have incentive to do a lazy falsification (try weakly, declare PASS), then bet YES
- Worse: they might not even try to falsify — just immediately PASS and bet YES
- The market signal is corrupted: a PASS from a falsifier no longer means "I tried hard and couldn't break it"

### Scale of problem in Run 6
- 23 out of 100 falsifiers placed at least one YES bet
- Average falsification attempt duration for YES-betting falsifiers: 2 steps (vs 15 steps for honest falsifiers)
- Strong correlation: falsifiers who bet YES spent less time trying to falsify

## Detection method
- Gemini external audit of transaction logs
- Cross-referenced: agent role, action type, investment direction
