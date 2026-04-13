# V-006 Resolution: Falsifier Invest->NOP

## Fix applied
1. **Falsifier invest action mapped to NOP**
   - In `src/engines/engine2_economy/actions.rs`:
     ```rust
     Action::Invest { direction, .. } if agent.role == Role::Falsifier => {
         Action::Nop  // Falsifiers cannot invest; action silently dropped
     }
     ```
   - Falsifiers can only: FALSIFY (attempt counterexample) or PASS (declare no counterexample found)
   - No YES bets, no NO bets — falsifiers are pure verifiers, not speculators

2. **Role-action matrix formalized**
   | Role       | INVEST_YES | INVEST_NO | FALSIFY | PASS | PROVE |
   |------------|-----------|-----------|---------|------|-------|
   | Prover     | YES       | YES       | NO      | NO   | YES   |
   | Falsifier  | NOP       | NOP       | YES     | YES  | NO    |
   | Speculator | YES       | YES       | NO      | NO   | NO    |

3. **Added incentive alignment test**
   - `tests/economy/incentive_test.rs`: simulates 100 falsifiers for 500 steps
   - Asserts: zero INVEST actions from falsifier-role agents
   - Asserts: falsification attempt duration is independent of node outcome

## Verification
- Run 6 re-run with fix:
  - Zero falsifier investment actions (all mapped to NOP)
  - Average falsification attempt duration: 14 steps (uniform across agents)
  - Market prices more accurate: correlation between price and resolution improved from 0.61 to 0.78
- Gemini re-audit: clean — no perverse incentive patterns detected

## Design lesson
When agents have asymmetric duties (verify vs. speculate), their action space must be constrained to align incentives. "Free market" does not mean "no rules" — it means "rules that make the market honest."
