# TuringOS v4 Economic Engine

## Magna Carta Three Laws

### Law 1: Information is Free
Agents search Mathlib and view nodes at zero cost. Thinking costs nothing.

### Law 2: Only Investment Costs Money
1 Coin = 1 YES + 1 NO (CTF conservation). APMM Mint-and-Swap router.
每个新节点系统自动注入 1000 YES + 1000 NO 做市。
做市商 = Price Oracle (广播概率)。允许小范围盈亏 (无常损失)。
废除一切补贴、悬赏、intrinsic_reward 铸币。

### Law 3: Digital Property Rights
Each agent has independent Skill path. Species evolution.

## Post-Genesis Zero Printing
- `on_init` is the ONLY legal coin injection (GENESIS)
- `fund_agent`, `redistribute_pool` ABOLISHED
- Rebirth does not inject new money
- Bankrupt agents survive via Law 1 (free append)

## CPMM (Constant Product Market Maker)
- `prediction_market.rs`: YES/NO恒定乘积
- `yes_price() = no_reserve / (yes_reserve + no_reserve)`
- Price = Bayesian probability
- Oracle resolution: external, irreversible

## Key Invariants
- Bank profit/loss = 0 (zero-sum treasury)
- 1 Coin = 1 YES + 1 NO (conservation)
- Prices sum to 1.0
