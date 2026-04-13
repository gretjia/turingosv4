# V-005 Root Cause: Greedy ArgMax router stuck in local minima

## WHY chain

1. **WHY** did all agents converge on the same 5 nodes?
   - Because the router uses ArgMax — it always selects the highest expected-value node.

2. **WHY** does ArgMax cause convergence?
   - Because expected value includes current market price, creating a positive feedback loop. Popular nodes look more valuable, attracting more agents, making them look even more valuable.

3. **WHY** was ArgMax chosen?
   - It's the default "rational" choice — a rational agent should always pick the best option. But this ignores the multi-agent exploration-exploitation tradeoff.

4. **WHY** does a rational single-agent strategy fail in a market?
   - Because markets need diversity of opinion to function. If everyone agrees, there's no trade, no price discovery, and no information in prices. The market becomes a consensus echo chamber.

5. **WHY** wasn't this caught in design?
   - The simulation was first run with few nodes (5-10) where convergence looks like "the market found the answer." At 100 nodes, the pathology became obvious.

## Root cause (one sentence)
ArgMax routing is a single-agent rationality assumption applied to a multi-agent market, causing catastrophic loss of exploration and market diversity.

## Contributing factors
- Small-scale testing (5 nodes) masked the convergence problem
- No market diversity metric was tracked until Run 5
- "Rational = ArgMax" is a common but wrong assumption in multi-agent systems
