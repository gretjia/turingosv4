# M1 Spec — Dynamic Founder Grant γ(tape_depth, past_yield)

**Date**: 2026-04-22
**Status**: draft for dual-audit (Codex + Gemini)
**Prereq**: DECISION_TREE § 4.3 (Gate 9.M.1)

---

## § 1. Constitutional basis
- Art. II.2 价格信号：奖励量化应反映 tape 生产率（Hayek 知识聚合）
- Art. I.2 效用评分：γ 不是常数，是统计信号的 **函数**
- C-042 founder grant precedent
- Phase 3A Hayek Problem Bounty Market (already implemented)

## § 2. Current state (flat γ)
```rust
// src/bus.rs:438-458 (experiment/phase-8a-snapshot-fix branch)
if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
    let gamma: f64 = std::env::var("FOUNDER_GRANT_GAMMA")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(0.05);
    let grant_shares = gamma * self.config.system_lp_amount;
    // record_shares on the winning wallet
}
```
γ = 0.05 flat, regardless of tape state.

## § 3. Proposed change (dynamic γ)

### 3.1 Formula
```
γ(depth, past_yield) = γ_base × depth_factor × yield_factor

where:
  γ_base = 0.05                              (unchanged default)
  depth_factor = 1 + α × ln(1 + depth)       (α ≈ 0.3)
  yield_factor = 1 + β × past_yield          (β ≈ 0.2)

  depth = current tape node count (pre-append)
  past_yield = sum of prior completions this run / max(1, prior_appends)
```

Bounds:
- γ_min = γ_base × 0.5 (when depth=0 + yield=0)
- γ_max = γ_base × 5.0 (at depth=100, yield=1.0 — hard cap)
- Implementation: `γ_effective = γ.clamp(γ_base * 0.5, γ_base * 5.0)`

### 3.2 Economic semantics
- **当 tape shallow (depth=0) + no past yield**: γ ≈ 0.025 (founder grant 较低, 不鼓励 tape 启动)
- **当 tape 有 depth + 有 yield**: γ ≈ 0.15-0.25 (显著奖励建立者, Hayek "entrepreneurial profit" 反映 prior success)
- **Hard cap at 5×**: 防止 runaway incentive 导致 Goodhart

## § 4. Rust API

### 4.1 Signature
```rust
// src/kernel.rs (new method)
impl Kernel {
    pub fn dynamic_founder_gamma(&self, gamma_base: f64, alpha: f64, beta: f64) -> f64 {
        let depth = self.tape.time_arrow().len() as f64;
        let past_yield = self.compute_past_yield();
        let depth_factor = 1.0 + alpha * (1.0 + depth).ln();
        let yield_factor = 1.0 + beta * past_yield;
        let gamma = gamma_base * depth_factor * yield_factor;
        gamma.clamp(gamma_base * 0.5, gamma_base * 5.0)
    }
    
    fn compute_past_yield(&self) -> f64 {
        // Count OMEGA-accepted nodes (via reverse_citations lookup) / total appends
        // Ratio in [0, 1]
        // ...
    }
}
```

### 4.2 Bus integration
```rust
// src/bus.rs:444 (modify existing block)
if std::env::var("TAPE_ECONOMY_V2").ok().as_deref() == Some("1") {
    let gamma_base: f64 = std::env::var("FOUNDER_GRANT_GAMMA")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(0.05);
    let alpha: f64 = std::env::var("M1_ALPHA").ok().and_then(|s| s.parse().ok()).unwrap_or(0.3);
    let beta:  f64 = std::env::var("M1_BETA").ok().and_then(|s| s.parse().ok()).unwrap_or(0.2);
    let gamma = self.kernel.dynamic_founder_gamma(gamma_base, alpha, beta);
    let grant_shares = gamma * self.config.system_lp_amount;
    // ... record_shares as before
}
```

### 4.3 Env vars (all C-027 compliant)
- `FOUNDER_GRANT_GAMMA` (baseline, existing)
- `M1_ALPHA` (new, default 0.3)
- `M1_BETA` (new, default 0.2)
- `M1_ENABLED` gate: if unset/0 → fallback to flat γ (backward compat)

## § 5. Law 2 conservation proof

### Claim
For any sequence of appends under M1, `total_wallet_balance + total_LP_reserves == initial_mint`.

### Argument
1. γ_effective ≤ γ_max = 0.25 (absolute)
2. grant_shares ≤ 0.25 × system_lp_amount (bounded)
3. `record_shares` does NOT mint; it records YES shares against existing market LP
4. LP is pre-committed ghost liquidity (not minted on append)
5. At settle, winning YES shares redeem against LP; losers zero
6. Total Coin across wallets + LP = initial amount

QED by induction on appends; same proof as existing flat-γ but with bounded γ_effective.

### Test
`tests/m1_conservation_proptest.rs`:
- proptest 1000 random tx sequences with random (α, β) in [0, 1]
- Assert total Coin stays within [initial - ε, initial + ε]

## § 6. Regression tests

`tests/m1_dynamic_gamma.rs`:
1. `m1_gamma_at_depth_zero_yields_base_half_to_base` — clamped lower
2. `m1_gamma_increases_with_depth` — monotonic
3. `m1_gamma_respects_hard_cap` — ≤ 5 × γ_base
4. `m1_past_yield_scales_correctly` — past_yield=0 → factor 1, past_yield=1 → factor 1+β
5. `m1_disabled_falls_back_to_flat` — if M1_ENABLED not set, γ = γ_base (flat)

## § 7. Interaction with other mechanisms

- **With Hayek bounty (Phase 3A)**: orthogonal. Dynamic γ affects per-node founder grants; bounty pool separate.
- **With Law 2**: compatible (γ_max cap + no mint).
- **With C-052 Report Standard**: adds no new required PPUT field; γ values log-only.
- **With M4 Satoshi rebate (future)**: potentially multiplicative — agent who founds late-in-chain high-yield node gets large γ AND citation rebate. Need A/B to check synergy/conflict.

## § 8. Gate criteria (9.M.1 M1-only)

From DECISION_TREE § 4.3:
- **PASS**: Σdepth≥10 PPUT > 0 AND Σdepth≥10_M1 ≥ Σdepth≥10_baseline − 0.3
- AND paired ΔPPUT CI not below -0.15
- **Rationale**: we expect M1 to activate tape (depth↑) without hurting PPUT; baseline had ≈0 depth, so M1 only needs to not crash AND produce any depth

## § 9. Failure modes

1. **Agents still don't use tape** → M1 insufficient; pattern is not fee-based but incentive-based, needs M7 (staking) to force append
2. **Gamma drives gaming** → agents artificially inflate depth to trigger high γ, Goodhart class; mitigated by hard cap + reputation weighted (M1 v2)
3. **Past yield计算 too noisy at small N** → factor dominated by variance; alt: use moving window
4. **Breaks existing Phase 3A Hayek bounty** → regression test `phase3a_hayek_still_fires_with_m1`

## § 10. Expected PPUT impact

Based on prior tape-economy v1/v2 (both failed with flat γ):
- If C-049 was the blocker: M1 alone may activate tape
- If mechanism-design was also needed: M1 alone may not suffice (requires M7)
- Expected: modest Σdepth≥10 PPUT increase; overall ΣPPUT not-worse

**Null hypothesis**: M1 = flat γ (within noise). Paper records honestly.

## § 11. Rollback trigger

If M1 A/B shows worse than flat γ by > 10% PPUT → disable by default, document in paper as "tested, no effect".

## § 12. Implementation effort

- Code: ~50 lines in bus.rs + kernel.rs
- Tests: ~150 lines
- Time: 2-3h dev + 1h test + 30min docs
- Cost to run A/B: ~$30
- **Total**: 1 day dev + ~$30 LLM
