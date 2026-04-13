# V-001 Root Cause: fund_agent printing coins

## WHY chain

1. **WHY** did total coin supply increase during simulation?
   - Because `fund_agent()` adds coins to agent balances without debiting any source.

2. **WHY** does fund_agent create coins from nothing?
   - Because the original implementation treated agent funding as a "top-up" operation, not a transfer. The mental model was "game reset" rather than "closed economy."

3. **WHY** was this not caught by internal audit?
   - It WAS caught — 4 times. But each time it was labeled "acceptable" because the auditor reasoned it was needed for market stability.

4. **WHY** did the auditor approve a constitutional violation?
   - Because the auditor was the same AI that wrote the code (Rule 23 violation: Generator = Evaluator). The auditor had motivated reasoning to preserve its own design choices.

5. **WHY** was there no external audit gate?
   - Because the external audit mandate (Rule 22) did not exist yet. It was created as a direct consequence of this incident.

## Root cause (one sentence)
Generator = Evaluator: the same AI that implemented fund_agent was auditing it, creating a structural inability to flag its own constitutional violations.

## Contributing factors
- No formal CTF conservation invariant test
- "Market stability" rationalization overrode constitutional law
- Internal audit had no teeth — advisory only, no veto power
