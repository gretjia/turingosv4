#!/usr/bin/env bash
# Codex TB-13 ship audit — Class 3 (CompleteSet + MarketSeedTx; agent-signed
# but money/collateral surface). Implementation-paranoid angle. Independent
# of Gemini ship audit (parallel, architectural angle). Per memory
# feedback_dual_audit_conflict: VETO > CHALLENGE > PASS.

set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md"
TMP_PROMPT="$(mktemp /tmp/tb13_codex_ship.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex TB-13 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-13 (CompleteSet + MarketSeedTx) ship-gate dual external audit. Independent of Gemini ship audit (parallel, architectural angle).

**Mandate**: Class 3 (money/collateral surface — first agent-signed economic mutator that locks Coin into conditional collateral and issues YES/NO claims). Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Round cap = 2 per `feedback_elon_mode_policy`.

## Audit target

- Charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- Architect ruling lossless: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Recursive self-audit: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`
- Real-LLM regression smoke: `handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md`

```text
TB-12 ship range:  fa36eca           (TB-12 baseline 759/0/150)
TB-13 range:       fa36eca..HEAD     (this audit)
HEAD: cargo test --workspace = 783 passed / 0 failed / 150 ignored (+24 net)
```

Net-new for TB-13 (commits to anchor your reads):

- `32aab27` Atom 0 + 0.5 — Charter + legacy CPMM forward-fence + label
- `70303af` Atom 1 — typed_tx schemas (3 variants + 4 newtypes + 3 SigningPayloads + 8 unit tests)
- `1806432` Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests
- `17d4a3b` Atom 6(a) — Recursive self-audit
- `7aac629` Atom 6(a.5) — Real-LLM regression smoke

## The 9 mandated audit questions (per architect Part A §4 + charter §3 Atom 6.b)

**Q1 (anti-mint)**: Does CompleteSetMint create or destroy money? Trace `src/state/sequencer.rs` CompleteSetMint accept arm. The contract: `balances_t[owner]` debit BY exactly `amount`; `conditional_collateral_t[event_id]` credit BY exactly `amount`; equal YES + NO shares minted (claims, NOT Coin). Σ holdings (6-holding sum incl. conditional_collateral_t) bit-equal pre/post.

**Q2 (resolution gate)**: Can Redeem fire without a system-emitted resolution? Trace dispatch arm. Sequencer must validate `task_markets_t[event_id.0].state ∈ {Finalized, Bankrupt}`; Open / Expired → `RedeemBeforeResolution`. Test: `sg_13_5_redeem_unavailable_before_outcome_resolution` covers both Open and Expired.

**Q3 (outcome match)**: Can Redeem with `outcome=Yes` and a Bankrupt-task resolution_ref bypass the outcome check? Sequencer must reject with `InvalidResolutionRef` when state-vs-outcome mismatch. Test: `sg_13_6` includes 2 mismatch checks (Finalized+No rejected; Bankrupt+Yes rejected). Also: `redeem.outcome != redeem.resolution_ref.claimed_outcome` is a separate gate that fires before state lookup — defense-in-depth.

**Q4 (6-holding conservation)**: Does the 6-holding `total_supply_micro` sum hold across all TB-13 typed_tx? Walk `src/economy/monetary_invariant.rs:152..198` `total_supply_micro` — it now includes `conditional_collateral_t.0.values()` after the 5-holding sum. `assert_total_ctf_conserved` is called from each TB-13 dispatch arm with empty exempt list. Test: `sg_13_1`, `sg_13_2`, `halt_total_supply_micro_unchanged_across_mint_redeem`.

**Q5 (complete-set balanced)**: Does `assert_complete_set_balanced` hold after every transition? The invariant: for every event in conditional_collateral_t, `min(Σ_yes, Σ_no) == collateral`. The MIN form (vs strict `Σ_yes==Σ_no==collateral`) was discovered mid-development as the correct form post-redemption — losing-side shares strand above collateral. Trace the invariant body in `src/economy/monetary_invariant.rs` and challenge the rule. Tests: `sg_13_1` (post-mint balanced; both equal), `halt_complete_set_balanced_post_seed`, `halt_total_supply_micro_unchanged_across_mint_redeem` (post-redeem MIN form).

**Q6 (seed solvency)**: Can MarketSeedTx create liquidity without provider balance? Trace MarketSeed accept arm: `collateral_amount > 0` else `InsufficientCollateral` (SG-13.4); `balances_t[provider] >= collateral_amount` else `InsufficientBalanceForMint` (SG-13.3). Test: `sg_13_3`, `sg_13_4`, `halt_market_seed_zero_balance_provider_rejected`.

**Q7 (shares-not-Coin)**: Are conditional shares anywhere counted as Coin? `total_supply_micro` must EXCLUDE `conditional_share_balances_t`. Read the function body and confirm. Architect CR-13.3 + SG-13.2 explicit.

**Q8 (underflow)**: Could a malformed `ShareAmount.units: u128` underflow at redeem? Sequencer must check `owned_units >= share_amount.units` (RedeemMoreThanOwned) AND `event_collateral.micro_units() as u128 >= share_amount.units` (InsufficientCollateral) BEFORE the subtraction. Trace both gates; verify the type-cast `event_collateral.micro_units() as u128` is sound (i64 → u128 widens; negative collateral would be caught by 6-holding sum but is structurally impossible).

**Q9 (forward-fence)**: Does any new TB-13 module file import legacy `prediction_market`? Trace `tests/tb_13_legacy_cpmm_forward_fence.rs` span detector. The fence's authoring-marker rule: only spans whose first non-blank line matches `TRACE_MATRIX TB-13` / `// TB-13 ` / `//! TB-13 ` / `/// TB-13 ` are in scope. Verify the rule cannot be bypassed by writing a TB-13 import OUTSIDE a TB-13-marker span (e.g., adding a `use crate::prediction_market::BinaryMarket` at the top of `src/state/sequencer.rs` without a TB-13 doc-comment marker would NOT be caught). Is the fence robust enough?

## Specifically scrutinize (RQ1-RQ7)

**RQ1 — Anti-forgery enforcement**: A malformed CompleteSetRedeemTx where wire `outcome` differs from `resolution_ref.claimed_outcome` — does the dispatch arm reject pre-state-lookup? Trace `src/state/sequencer.rs` CompleteSetRedeem step 1.

**RQ2 — CTF conservation**: Walk the math for each of the 3 TB-13 dispatch arms. At CompleteSetMint:
- balances_t row decreases by `amount`.
- conditional_collateral_t row increases by `amount`.
- conditional_share_balances_t both YES + NO sides increase by `amount.units` (claims, not Coin; not in supply sum).
- 6 holdings sum: pre = post. PASS?

At CompleteSetRedeem (winning side, e.g., Yes after Finalized):
- conditional_collateral_t row decreases by `share_amount.units`.
- conditional_share_balances_t YES side decreases by `share_amount.units`.
- balances_t row increases by `share_amount.units`.
- conditional_share_balances_t NO side UNCHANGED (stranded losing share).
- 6 holdings sum: pre = post. PASS?

At MarketSeed:
- balances_t row decreases by `collateral_amount`.
- conditional_collateral_t row increases by `collateral_amount`.
- conditional_share_balances_t both YES + NO sides increase by `collateral_amount.units` for the provider.
- 6 holdings sum: pre = post. PASS?

**RQ3 — Replay determinism**: Two replays of the same chain must produce byte-identical conditional_collateral_t + conditional_share_balances_t. Verify via the smoke evidence at `handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json`: `economic_state_reconstructed: true`. Also verify the round-trip at `tests/tb_13_complete_set.rs::sg_13_1` post-mint state.

**RQ4 — Re-mint after partial redeem (adversarial)**: Trace this sequence:
1. Mint K Coin → YES=K, NO=K, coll=K.
2. State flips Finalized.
3. Redeem M from YES → YES=K-M, NO=K, coll=K-M, balance += M.
4. Mint J more Coin → YES=K-M+J, NO=K+J, coll=K-M+J.
5. Redeem J from YES → YES=K-M, NO=K+J, coll=K-M, balance += J.
Does `assert_complete_set_balanced` (MIN form) hold throughout? Does any sequence break the 6-holding sum? Is there an adversarial pattern where a single agent profits unfairly?

**RQ5 — ResolutionRef integrity**: TB-13 uses `ResolutionRef { resolution_tx_id, claimed_outcome }` but the sequencer ONLY validates the task_market state (Finalized/Bankrupt) + the claimed_outcome match. The `resolution_tx_id` is opaque traceability metadata, NOT validated against L4. Is this safe? Argument: task_markets_t.state IS the on-chain state-of-truth for resolution; resolution_tx_id is for L4-audit replay. Charge: a malicious agent could submit a CompleteSetRedeemTx with resolution_tx_id pointing to a non-existent or wrong tx. Is this a concern? Sequencer ignores resolution_tx_id at validation — only the task_market state matters. Is this the right design or a security gap?

**RQ6 — Forward-fence span detector robustness**: The fence at `tests/tb_13_legacy_cpmm_forward_fence.rs` only checks lines inside spans with a TB-13 authoring marker. A new TB-13 file that does NOT carry a TB-13 doc-comment marker would NOT be in scope. Is this exploitable? Recommend: should the fence also scan for TB-13-named files (`src/state/conditional_market.rs` etc.) regardless of marker, OR is the marker-discipline sufficient?

**RQ7 — STEP_B protocol compliance**: TB-13 modifies `src/state/sequencer.rs` (restricted file per CLAUDE.md Code Standard). The TB-12 precedent did so via direct edit (additive dispatch arms). TB-13 follows the same pattern. Is this within STEP_B? Or should TB-13 have used parallel-branch A/B?

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q9 + RQ1-RQ7 cleared; ship is clean.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- <repeat per challenge>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP / FIX-THEN-PROCEED / REDESIGN).

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md.

BRIEF_EOF

echo "  Codex audit prompt prepared at: $TMP_PROMPT" >&2
echo "  Output target: $OUT" >&2
echo "  Invoking codex exec..." >&2

cat "$TMP_PROMPT" | codex exec --skip-git-repo-check --sandbox read-only --color never - > "$OUT.raw" 2>&1
EXIT=$?

if [ $EXIT -ne 0 ]; then
  echo "  codex exec returned exit code $EXIT" >&2
  echo "  partial output saved to $OUT.raw" >&2
fi

# The codex exec output may include status lines + the actual audit body.
# Extract anything from the first markdown header onwards; fall back to raw if no header found.
mv "$OUT.raw" "$OUT"
echo "  Audit saved: $OUT" >&2
exit $EXIT
