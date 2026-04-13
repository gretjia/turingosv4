# V-006 Root Cause: Falsifier can buy YES tokens

## WHY chain

1. **WHY** did falsifier F-042 buy YES tokens on a node it was supposed to falsify?
   - Because the `invest()` action has no role-based restrictions — any agent can buy any direction on any node.

2. **WHY** are there no role-based investment restrictions?
   - Because the economy engine was designed as role-agnostic: agents have balances and can invest freely. Roles were an agent-layer concept, not an economy-layer enforcement.

3. **WHY** does this create a perverse incentive?
   - Falsifiers are paid to find bugs. If they can also bet YES, they're paid more for NOT finding bugs. The dominant strategy becomes: don't try hard, declare PASS, bet YES, profit.

4. **WHY** is this a Law 2 violation?
   - Law 2 requires that capital flows reflect genuine information. A falsifier betting YES injects noise into the price signal — the market reads it as "informed participant believes YES" when it's actually "lazy participant wants profit."

5. **WHY** wasn't this caught before Run 6?
   - Runs 1-5 did not have distinct roles for agents. Role-based simulation was new in Run 6. The perverse incentive only emerges when roles have asymmetric information duties.

## Root cause (one sentence)
No role-based action constraints in the economy engine, allowing falsifiers to profit from lazy verification by betting on the outcome they're supposed to be challenging.

## Contributing factors
- Economy engine was designed before roles were introduced
- "Free market" philosophy taken too literally — freedom without constraints enables gaming
- Incentive analysis was not part of the role design process
