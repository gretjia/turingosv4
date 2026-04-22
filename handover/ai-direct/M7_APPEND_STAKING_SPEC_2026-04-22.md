# M7 Spec — Append Staking with Slash / Refund

**Date**: 2026-04-22 draft
**Prereq**: Phase 9.M.3 (after M1/M4 signal)

## § 1. Constitutional basis
- Art. II.2.1 探索/利用 balance: stake 是 **commitment**，reduces random exploration
- Law 1 "Information is free" vs Law 2 "only investment costs money": append 本身免费，但可选 stake 换更高期望 reward
- Blockchain: PoS / slashing model; Taleb skin-in-the-game

## § 2. Rationale

F-2026-04-20-04 observation: agents prefer `complete` even at fee=2000 Coins. Hypothesis: **fee on complete is wrong lever** — it discourages the winning action. Instead, **stake on append** incentivizes quality (agents think before committing Coin).

## § 3. Formal semantics

### 3.1 State transitions

```
append:
  - Optional: agent supplies stake ε ∈ [0, MAX_STAKE]
  - Bus records stake escrow (wallet − ε, escrow[node_id] += ε)
  - Stake does NOT gate append (append is free per Law 1 unless agent opts-in)

halt_and_settle(golden_path):
  For each escrowed_node in escrow:
    if escrowed_node ∈ golden_path OR is ancestor of golden_path:
      refund(author, stake × REFUND_MULTIPLIER)    # e.g., 1.5×
    else:
      # forfeit: stake goes to LP pool (NOT to any agent — Law 2)
      LP_reserve += stake
```

### 3.2 Parameters
- `M7_MAX_STAKE` default: `50` (0.5% of initial 10000 Coin)
- `M7_REFUND_MULTIPLIER` default: `1.5`
- `M7_MIN_STAKE`: 0 (opt-in only)

### 3.3 Economic semantics
- **Risk-neutral agent**: stakes if P(node ∈ GP) × 1.5 > 1 → P > 0.67
- **If stake too cheap**: agents spam stake; no quality signal
- **If stake too expensive**: agents never stake; mechanism silent
- **Sweet spot**: 5-10% of agent's budget → only "confident" appends get staked

## § 4. Rust API

### 4.1 Signature
```rust
// src/bus.rs: new public method
impl TuringBus {
    pub fn append_with_stake(
        &mut self, author: &str, payload: &str, parent_id: Option<&str>,
        stake: f64,
    ) -> Result<BusResult, String> {
        if stake > 0.0 {
            // Ensure agent can afford
            let balance = self.balance(author);
            if balance < stake {
                return Err(format!("Insufficient balance for stake: {} < {}", balance, stake));
            }
            // Escrow
            self.debit_wallet(author, stake)?;
            // Continue with normal append; on success record escrow
        }
        let result = self.append_internal(author, payload, parent_id, false)?;
        if let BusResult::Appended { ref node_id } = result {
            if stake > 0.0 {
                self.escrow.insert(node_id.clone(), (author.to_string(), stake));
            }
        }
        Ok(result)
    }
}

// New tool signal for agents
enum ToolSignal {
    // ... existing variants
    AppendWithStake { payload: String, parent: Option<String>, stake: f64 },
}
```

### 4.2 Settlement logic in halt_and_settle
```rust
if std::env::var("M7_ENABLED") == Some("1") {
    let multiplier: f64 = std::env::var("M7_REFUND_MULTIPLIER")
        .ok().and_then(|s| s.parse().ok()).unwrap_or(1.5);
    let gp_set: std::collections::HashSet<_> = golden_path.iter().collect();
    
    for (node_id, (author, stake)) in std::mem::take(&mut self.escrow) {
        let in_gp_or_ancestor = gp_set.contains(&node_id)
            || self.is_ancestor_of_any(&node_id, golden_path);
        if in_gp_or_ancestor {
            self.credit_wallet(&author, stake * multiplier);  // refund + bonus
        } else {
            // forfeit to LP pool — compensate market's ghost liquidity
            self.kernel.credit_lp_pool(stake);  // new method
        }
    }
}
```

### 4.3 Bus field
```rust
pub struct TuringBus {
    // ... existing
    escrow: HashMap<NodeId, (String, f64)>,  // node_id → (author, stake)
}
```

## § 5. Law 2 conservation

### Proof
Define `total_system = Σ wallets + Σ LP + Σ escrow`.

- Before append: `total = T`
- On append with stake s: wallet[a] -= s; escrow[node] = s → `total = T` ✓
- On settle win: escrow[node] = 0 + wallet[a] += s*1.5 → `total = T + 0.5s` ??? 不守恒!

**Issue**: 1.5× refund creates 0.5s out of thin air. Law 2 violation.

### Fix: refund comes from Hayek bounty pool
- If bounty_LP has enough → multiplier refund draws from it
- If bounty_LP exhausted → refund is 1× (not 1.5×), forfeit is 0.5×
- Alternatively: multiplier = 1.0 + (bounty_remaining / total_stakes)（动态）

Revised test: refund <= stake + available_bonus_pool; conservation with bounty pool inclusion.

### Test
`tests/m7_conservation.rs`:
- Assert: Σ(all Coin / shares / LP / escrow / bounty) = initial_mint
- proptest with random stakes + random golden paths

## § 6. Regression tests

`tests/m7_append_staking.rs`:
1. `m7_append_no_stake_identical_to_baseline`
2. `m7_stake_escrowed_on_successful_append`
3. `m7_refund_on_gp_inclusion`
4. `m7_forfeit_on_no_gp_inclusion`
5. `m7_insufficient_balance_rejects_stake`
6. `m7_refund_multiplier_respects_bounty_pool`
7. `m7_ancestor_eligibility` — non-terminal ancestor also refunds

## § 7. Interaction with other mechanisms

- **With M1 (dynamic γ)**: orthogonal; γ affects founder grant, M7 affects stake lifecycle
- **With M4 (Satoshi rebate)**: potentially synergistic; staked appends get both rebate share AND stake refund
- **With Phase 3A Hayek bounty**: **critical coupling** — M7 refund must draw from bounty_LP, else Law 2 violation

## § 8. Gate criteria (Phase 9.M.3)

**PASS**:
- append usage > 0 in ≥ 50% of N=20 problems (tape activates)
- Σdepth≥10 PPUT ≥ M1-only baseline
- Law 2 conservation test 1000 proptest runs全绿

**FAIL**:
- Agents never stake (mechanism silent) → parameter tuning needed
- OR: conservation violation → spec bug, revert

## § 9. Failure modes

1. **Agents never opt-in to stake**: if prompt doesn't nudge, they play safe. Need tool-level incentive, not prompt.
2. **Economic infeasibility**: if stake refund > bounty pool capacity, degrade gracefully
3. **Collusion**: agents stake-ping-pong trivially. Countered by C-059 pairwise diversity check
4. **Unused escrow accumulation**: if problems fail (no halt_and_settle called), escrow stays → LP pool grows but agents lose forever. Mitigate: on MaxTxExhausted halt, refund all stakes at 1× (no bonus, no forfeit)

## § 10. Paper positioning

"Import PoS-style staking to LLM swarm. Append becomes optionally 'committed' by Coin stake — successful lemmas earn bonus, unused ones compensate market LP. Tests F-2026-04-20-04 mechanism hypothesis under Phase-8 signal integrity."

## § 11. Implementation effort
- Code: ~200 lines (new HashMap + settle logic)
- Tests: ~250 lines
- Total: 2 days dev + ~$30 A/B
