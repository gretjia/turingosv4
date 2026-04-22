# M8 Spec — Bonding-Curve Liquidity Provider (LP Role)

**Date**: 2026-04-22 draft
**Prereq**: Phase 9.M.4 (after M1/M4/M7 signal)
**Status**: most speculative of the 4; expect dual-audit CHALLENGE

## § 1. Constitutional basis
- Art. II.2 价格信号 — LP 让价格更 smooth，减少流动性噪声
- Art. III.3 屏蔽相关性 — 不同 agent 不同角色（speculator vs LP）→ 输出异质性
- Austrian: 角色专业化 Kirzner entrepreneur vs speculative trader

## § 2. Rationale

Current CPMM market (Phase 3A): 所有 agent 只能 `invest` (buy YES / NO)。Market 启动 liquidity 来自 `system_lp_amount` (200 Coin ghost)。
Agents 不能 provide liquidity. 所以：
- **Price 剧烈**: 1 Coin invest 可能 shift price 几 percentage point
- **Agent role 同质**: 所有人都是 speculator
- **Informational bandwidth 小**: price 只反映二分 YES/NO 意见

**M8 fix**: Expose `provide_liquidity(node_id, amount)` tool. LP 收取 invest fees 作为 yield。

## § 3. Formal semantics

### 3.1 State transitions

```
provide_liquidity(agent, node_id, amount):
  - debit wallet[agent] -= amount
  - market[node_id].yes_reserve += amount
  - market[node_id].no_reserve += amount
  - market[node_id].lp_shares[agent] += 2 * amount (symmetric adds)
  - emit LiquidityProvided event

withdraw_liquidity(agent, node_id, lp_shares):
  - require shares <= market[node_id].lp_shares[agent]
  - compute current value = f(shares, yes_reserve, no_reserve)
  - debit lp_shares, refund coin

invest (as before):
  - tiny fee (0.3% default) on each trade goes to LP pool
  - LP pool distributed to active LPs on settle

halt_and_settle:
  - existing logic + LP fee accumulation distribution
```

### 3.2 Fee
- `M8_FEE_BPS` default: 30 (0.30% = 3 basis points in bps)
- On `invest(N coins)`: N * 0.003 goes to LP pool; agent receives (N - fee)
- At settle: LP pool distributed to LPs proportional to their lp_shares

### 3.3 Bonding curve
- CPMM already has a form: k = yes_reserve × no_reserve
- M8 doesn't change the invariant; just enables agent-side LP
- Alternative: switch to constant-sum (k' = yes + no) for less slippage — **deferred; Paper 2+**

## § 4. Rust API

### 4.1 New tool
```rust
// src/sdk/tools/lp.rs (new file)
pub struct LiquidityProviderTool {
    min_lp_amount: f64,
}

impl TuringTool for LiquidityProviderTool {
    fn on_pre_append(&mut self, author: &str, payload: &str) -> ToolSignal {
        if let Some((node_id, amount)) = parse_provide_liquidity(payload) {
            return ToolSignal::ProvideLiquidity { target_node: node_id, amount };
        }
        if let Some((node_id, shares)) = parse_withdraw_liquidity(payload) {
            return ToolSignal::WithdrawLiquidity { target_node: node_id, shares };
        }
        ToolSignal::Pass
    }
    // ...
}
```

### 4.2 New bus handling
```rust
if let ToolSignal::ProvideLiquidity { target_node, amount } = signal {
    self.debit_wallet(author, amount)?;
    self.kernel.add_liquidity(&target_node, amount)?;
    self.record_lp_shares(author, &target_node, amount * 2.0);
    self.tx_count += 1;
    return Ok(BusResult::LiquidityProvided { node_id: target_node, shares: amount * 2.0 });
}
```

### 4.3 Kernel method
```rust
impl BinaryMarket {
    pub fn add_liquidity(&mut self, amount: f64) -> Result<f64, KernelError> {
        let shares_issued = amount * 2.0;  // symmetric injection
        self.yes_reserve += amount;
        self.no_reserve += amount;
        self.lp_shares_outstanding += shares_issued;
        Ok(shares_issued)
    }
}
```

## § 5. Law 2 conservation

Most delicate of the 4 mechanisms. Fees redistribute; LP shares represent claims.

### Proof sketch
Define `total = Σ wallets + Σ yes_reserve + Σ no_reserve + Σ accumulated_fees`.

- provide_liquidity(N): wallet -= N, yes_reserve += N, no_reserve += N → total: -N + 2N = +N ???

**Issue**: symmetric injection adds 2N to reserves but only removes N from wallet. NET CREATE.

### Fix: symmetric injection charges 2N from wallet (not N)
- OR: inject N asymmetrically based on current price (maintains k)

Chose CPMM with asymmetric: `add_liquidity(N)` keeps price constant, adds (N, price*N/(1-price)) to reserves. Math details in § 5.1 below.

### § 5.1 CPMM-preserving add
At current price p = no_reserve / (yes_reserve + no_reserve):
- Split N: N_yes = N * p, N_no = N * (1 - p)
- yes_reserve += N_yes; no_reserve += N_no
- Total reserve injected = N (not 2N)
- wallet -= N
- LP shares = N (1 coin : 1 share 在 constant price)

Conservation: trivial. Better design than symmetric injection.

### Test
`tests/m8_conservation.rs`:
- proptest 1000 ops: random mix of invest, add_liquidity, withdraw_liquidity, settle
- Assert total Coin stays within ε of initial

## § 6. Regression tests
`tests/m8_lp.rs`:
1. `m8_provide_liquidity_debits_wallet_adds_reserves_preserves_k`
2. `m8_fee_on_invest_accrues_to_lp_pool`
3. `m8_settle_distributes_lp_pool_proportionally`
4. `m8_withdraw_before_settle_correct`
5. `m8_withdraw_after_settle_receives_redemption`
6. `m8_lp_role_interaction_with_hayek_bounty`

## § 7. Interaction

- **With M1/M4/M7**: orthogonal but composes — agent can stake-append, provide-LP on own node, farm fees + rebate
- **With Hayek bounty**: LP fees augment bounty pool
- **With Art. III.4 Goodhart**: risk — agents farm fees via ping-pong invest. Mitigate: per-agent per-market fee cap

## § 8. Gate criteria (Phase 9.M.4)

**PASS**:
- ≥ 1 agent ops LP role successfully (tool signal emitted)
- Price volatility (per-node std dev) decreases ≥ 20% vs baseline
- ΣPPUT not worsened by > 10%

**FAIL**:
- No LP provision observed (agents don't discover the role) → prompt nudge or skip
- Conservation violation → revert

## § 9. Failure modes

1. **Agents don't discover LP role**: similar to why tape stays empty — prompt / tool description must be incentive-visible (Phase 8 info signal: balance visible NOW after C-049 fix; should help)
2. **LP liquidity = 0 at settle**: no-op; no harm but signal that mechanism is dormant
3. **Fee drain attack**: pair of agents invest-withdraw-invest to farm fees. Detection: pairwise_diversity_mean on LP actions + per-agent fee cap

## § 10. Paper positioning

"Auto-market maker with agent-side LP role. Tests role specialization hypothesis: does allowing agents to opt into liquidity provision (vs speculation only) yield measurable price stability + reduced search variance?"

**Don't over-claim**: this is Austrian school "division of labor" via role differentiation; empirical question whether LLM agents, given the choice, actually split into specialists.

## § 11. Implementation effort
- Code: ~400 lines (new tool + market LP methods + bus handling)
- Tests: ~300 lines
- Most complex of the 4
- Total: 3-4 days dev + ~$40 A/B

## § 12. Recommendation

M8 is the most speculative. Only attempt if **M1/M4/M7 show promising signals**. Otherwise defer to Paper 2/3.
