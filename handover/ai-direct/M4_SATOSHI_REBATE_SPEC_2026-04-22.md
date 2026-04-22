# M4 Spec — Satoshi Citation Rebate

**Date**: 2026-04-22 draft (blocks Phase 9.M.2 until dual-audit PASS)
**Prereq**: Phase 3B branch exists but unmerged; formalize semantics.

## § 1. Constitutional basis
- Art. I.2 信誉累积 (C-053): "被后续引用的总次数" → 经济反映
- Blockchain reference: Bitcoin block reward + transaction fees; Ethereum MEV share
- Austrian: capital goods yield interest over time; 中间品 compensated continuously

## § 2. Rationale

Current settlement (C-043): only **last-node (GP terminal)** author gets the halt_and_settle payout. 所有 ancestors 免费贡献了 proof structure，但只 terminal 拿钱。

**问题**: agents 无动机 append 中间步骤。若我的 lemma 是 GP ancestor，我什么都不得 → append 是 pure charity。

**Satoshi fix**: Terminal 拿 X%，ancestors 按 citation depth/proximity 分 (1-X)%。

## § 3. Formal semantics

### 3.1 Rebate formula
At `halt_and_settle(golden_path)`:
```
total_payout = Hayek_bounty_LP + per_node_settlement_sum
terminal_share = 0.6 × total_payout                  (X = 60%)
ancestor_share = 0.4 × total_payout                  (1-X = 40%)

for each ancestor A in golden_path_ancestry:
   depth_from_terminal = path_distance(A, terminal)
   weight(A) = exp(-depth_from_terminal × decay_rate)   # decay = 0.3
   credit(A.author) += ancestor_share × weight(A) / Σweights
```

Properties:
- terminal 永远拿最大份
- 最近 ancestor (depth 1) 拿次大
- Exponential decay → deep ancestors 拿小但非零
- Σ credits = 1.0 × total_payout (守恒)

### 3.2 Envelope example
GP = [A → B → C → D → E (terminal)]
- E gets 60%
- D gets ~22% (exp(-0.3)/Σ_{i=0..3} exp(-0.3i) × 40%)
- C gets ~16%
- B gets ~12%
- A gets ~9% (rebate applies at halt time, not in real-time)

## § 4. Rust API

```rust
// src/bus.rs: halt_and_settle
fn settle_satoshi_rebate(&mut self, golden_path: &[NodeId]) -> Result<(), String> {
    if golden_path.is_empty() { return Ok(()); }
    
    let total_payout = self.compute_payout_pool(golden_path)?;
    let terminal_share_ratio: f64 = std::env::var("M4_TERMINAL_SHARE")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(0.6);
    let decay_rate: f64 = std::env::var("M4_DECAY_RATE")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(0.3);
    
    // terminal payout
    let terminal_id = golden_path.last().unwrap();
    let terminal_author = self.kernel.tape.get(terminal_id).unwrap().author.clone();
    self.credit_wallet(&terminal_author, total_payout * terminal_share_ratio);
    
    // ancestor rebate
    let ancestor_share = total_payout * (1.0 - terminal_share_ratio);
    let ancestors: Vec<_> = golden_path.iter().rev().skip(1).collect();  // exclude terminal
    let weights: Vec<f64> = (1..=ancestors.len())
        .map(|d| (-decay_rate * d as f64).exp()).collect();
    let total_weight: f64 = weights.iter().sum();
    
    for (ancestor_id, &w) in ancestors.iter().zip(&weights) {
        let author = self.kernel.tape.get(ancestor_id).unwrap().author.clone();
        self.credit_wallet(&author, ancestor_share * w / total_weight);
    }
    
    Ok(())
}
```

Gate: `std::env::var("M4_ENABLED") == "1"` 或 `TAPE_ECONOMY_V2_M4`.

## § 5. Law 2 conservation

### Claim
For any golden_path, `Σ credits == total_payout`.

### Proof
- terminal: `total_payout × terminal_share_ratio`
- ancestors: `total_payout × (1 - terminal_share_ratio) × (Σ weights_i / Σ weights) = total_payout × (1 - terminal_share_ratio)`
- Total: `total_payout × terminal_share_ratio + total_payout × (1 - terminal_share_ratio) = total_payout` ✓

### Test
`tests/m4_conservation.rs`:
- proptest golden_paths of length 1..20
- Assert `|Σ credits - total_payout| < 1e-9`
- Assert each credit ∈ [0, total_payout]

## § 6. Regression tests

`tests/m4_satoshi_rebate.rs`:
1. `m4_terminal_gets_60_pct` — share ratio correct
2. `m4_ancestors_decay_exponentially` — weight(i) < weight(i-1)
3. `m4_empty_gp_no_payout` — edge case
4. `m4_single_node_gp_terminal_gets_all` — no ancestors to rebate
5. `m4_disabled_falls_back_to_flat` — existing halt_and_settle preserved
6. `m4_reputation_increments_consistently` — rebate doesn't double-count reputation (handled by tape, not rebate path)

## § 7. Interaction with other mechanisms

- **Phase 3A Hayek bounty**: Hayek bounty pool 包含在 total_payout 里
- **C-053 reputation counter**: 独立累积（append 时 +1），rebate 是 Coin 流向，不再重复
- **M1 dynamic γ**: γ affects founder grant (YES shares); M4 affects halt-time Coin payout. 正交。
- **M7 append staking (future)**: 若 append 有 stake, rebate 可抵扣 stake refund

## § 8. Gate criteria (Phase 9.M.2)

**PASS (warrants scaling to M-combined)**:
- M4 A/B shows Σdepth≥10 PPUT ≥ 2× Phase-8 baseline
- AND Mean PPUT (solved) not worse by > 10% (paired CI)
- AND reputation p50 > 0 (agents build chains)

**INCONCLUSIVE**: signal present but not 2× → test different decay rate / terminal share

**FAIL (M4 ineffective)**: no observable depth activity change

## § 9. Failure modes

1. **Incentive insufficient**: 60%/40% split too much reserved for terminal → agents still rush finish. Mitigate: test 50%/50%, 40%/60%
2. **Short-chain gaming**: agents split their proof artificially to inflate ancestor rebates. Mitigate: minimum node complexity (max payload chars) already in place; reputation tracking catches repeat-offender patterns
3. **Inter-agent collusion**: two agents ping-pong trivial lemmas to each other to farm rebates. Mitigate: F-2026-04-20-05-class detection via pairwise diversity metric (C-059)

## § 10. Paper positioning

"Satoshi-style block-reward + ancestor-rebate imported from Bitcoin. We test whether distributing the proof-finalization reward across ancestors (not just terminal) incentivizes tape-depth construction."

**Do NOT claim**:
- "solves tape dormancy universally" (needs empirical evidence)
- "first application of blockchain economics to LLM swarm" (careful citation needed)

## § 11. Implementation effort
- Code: ~120 lines in bus.rs
- Tests: ~200 lines
- Phase 3B branch merge + cleanup: 2h
- Total: 1 day dev + ~$30 LLM A/B
