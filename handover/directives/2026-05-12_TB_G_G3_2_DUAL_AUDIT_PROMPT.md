# TB-G G3.2 — PRE-§8 Dual Audit Prompt (Codex G2 + Gemini DeepThink)

> **Class**: 4 STEP_B (sequencer admission + RejectionClass tail-append)
> **Branch HEAD**: `2b1ec38` on `feat/g3-2-risk-cap-admission` (parent `5770d49` on `origin/main`)
> **Architect §8 ratification**: `handover/directives/2026-05-12_TB_G_G3_2_§8_ARCHITECT_RATIFICATION.md` (conditional on dual audit PASS)
> **Packet**: `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md` Q1..Q6 + §6.3 dual-audit-plan Q1..Q12
> **Round-cap**: 2 (per `feedback_dual_audit` `Elon-mode` policy)
> **Conservative merge**: VETO > CHALLENGE > PASS (`feedback_dual_audit_conflict`)

## §1. Scope

Audit the Class-4 STEP_B atomic implementation landed at commits `e8e13af` + `2b1ec38`:

- `src/state/typed_tx.rs` — `RejectionClass::BankruptcyRiskCapExceeded` + `TransitionError::BankruptcyRiskCapExceeded` tail-append + Display
- `src/state/sequencer.rs` — 4 admission preconditions + Gap-A reputation +1 + Gap-B verifier bond return + TerminalSummary autopsy emit dispatch + apply_one Stage 3.5b
- `src/runtime/agent_pnl.rs` — `bankruptcy_risk_cap_micro` helper (architect Q1)
- `src/runtime/autopsy_capsule.rs` — `derive_g3_2_terminal_summary_bankrupt_autopsies` + `write_g3_2_terminal_summary_bankrupt_autopsies_to_cas`
- `src/runtime/risk_cap_impact_report.rs` (NEW) — `RiskCapImpactReport` (architect §7.1) + `FinalizeRewardPayoutBreakdown` (architect §7.5)
- `tests/constitution_g3_bankruptcy_risk_cap.rs` (NEW; 32 tests)
- `tests/constitution_predicate_gate.rs` (exhaustive-match update for new RejectionClass variant)
- `scripts/run_constitution_gates.sh` (gate registration)
- `genesis_payload.toml` (Trust Root rehash: sequencer.rs + typed_tx.rs + runtime/mod.rs)

## §2. Architect verdict (Q1..Q6) — bound the audit scope

| Q | Decision | Audit check |
|---|----------|-------------|
| Q1 | risk-cap = `initial_balance_micro / 10` per-agent; NO new EconomicState table | Q1.a: helper threshold matches G3.1 `classify_solvency` boundary at `src/runtime/agent_pnl.rs:311`. Q1.b: source-grep confirms NO `bankruptcy_risk_cap_t` field added to `EconomicState`. Q1.c: per-agent cap table {Agent_0..9: 100k / MarketMakerBudget: 500k / tb7-7-sponsor: 1M} matches preseed registry. |
| Q2 | uniform +1 reputation per accepted VerifyTx | Q2.a: `reputations_t[verifier] += 1` fires only on accept-path (NOT on reject-path) in VerifyTx Step 5c. Q2.b: NO verdict-weighted (`if verdict == Confirm`) branch. Q2.c: confirm AND doubt verdicts both increment by 1. |
| Q3 | B1: extend FinalizeRewardTx; NO BondReturnTx system-tx | Q3.a: NO new `TypedTx::BondReturn` variant in `src/state/typed_tx.rs`. Q3.b: NO new `TxKind::BondReturn` discriminant. Q3.c: NO new signing-domain constant. Q3.d: FinalizeRewardTx Step 7c-bis filters `stakes_t` correctly: task_id == claim.task_id AND tx_id != claim.work_tx_id. Q3.e: bond credit + stakes_t removal preserve CTF invariant. |
| Q4 | Bundle Gap-A + Gap-B into G3.2 | Q4.a: OBS_G2P_VERIFY_PEER_REWARD SG-G2P.6.b + SG-G2P.6.c both addressed by single G3.2 atom. Q4.b: NO separate G3.5 charter row needed. |
| Q5 | Risk-cap fires FIRST in admission (subsuming pattern) | Q5.a: 4 admission arms (WorkTx + VerifyTx + ChallengeTx + BuyWithCoinRouter) each have risk-cap precondition BEFORE per-arm balance/stake/bond/router gate. Q5.b: balance=0 + stake=0 WorkTx submission → BankruptcyRiskCapExceeded (NOT StakeInsufficient). Q5.c: predicate-fail (Step 2-3) still fires BEFORE risk-cap (preserves predicate-fail telemetry for bankrupt agents per architect §7.2 read-side scope). |
| Q6 | Per-task-end (TerminalSummary boundary) autopsy emit | Q6.a: TerminalSummary dispatch Step 2.5 invokes `derive_g3_2_terminal_summary_bankrupt_autopsies`. Q6.b: apply_one Stage 3.5b invokes `write_g3_2_terminal_summary_bankrupt_autopsies_to_cas`. Q6.c: both gated by `is_autopsy_active_at(ts.last_logical_t)` (replay-safety, TB-15 R2 closure precedent). Q6.d: capsule_id deterministic from `(econ, ts, round, t)` (Art. 0.2 replay-determinism witness in fixture test). |

## §3. Architect §7 supplementary requirements (§3.7.1..7.5)

| Req | Audit check |
|-----|-------------|
| 7.1 RiskCapImpactReport | 7.1.a: NEW module registered in `runtime/mod.rs`. 7.1.b: struct field shape matches architect verbatim (`agent_id`, `balance_before_micro`, `risk_cap_micro`, `tx_kind`, `task_id`, `another_agent_continued`, `solve_outcome`). 7.1.c: predicates exposed (`is_bankruptcy_risk_cap_*`). 7.1.d: tx_kind labels cover all 4 admission arms (work/verify/challenge/buy_with_coin_router). |
| 7.2 Below-cap reads | 7.2.a: source-grep confirms NO `BankruptcyRiskCapExceeded` token in read-side files (`src/sdk/snapshot.rs`, `src/sdk/your_position.rs`, `src/top_white/predicates/*`). 7.2.b: `compute_agent_pnl` works for bankrupt agents (returns negative realized_pnl, NOT panic). |
| 7.3 Markov scope (autopsy) | 7.3.a: derive helper sets `CapsulePrivacyPolicy::AuditOnly`. 7.3.b: capsule public_summary bounded; private_detail_cid audit-only. |
| 7.4 Sybil guard | 7.4.a: Step-3.5 `VerifyDuplicate` rejection structurally prevents repeat (verifier, target_work_tx) → reputation accumulation is unique-per-pair. 7.4.b: Step-5b `agent_verifications_t.insert` recorded AFTER admission accept (defense against re-entry). |
| 7.5 Payout breakdown | 7.5.a: `compute_finalize_reward_payout_breakdown` separates solver_reward_delta vs verifier_bond_return_delta vs other_settlement_delta. 7.5.b: `assert_finalize_reward_payout_bounded` rejects `total > escrow_plus_bonds_at_pre`. 7.5.c: invariant matches Step 7c-bis filter (task_id == claim.task_id AND tx_id != claim.work_tx_id). |

## §4. Packet §6.3 audit checklist Q1..Q12

| Q | Audit task |
|---|------------|
| Q1 | risk-cap threshold matches G3.1 NearInsolvent (`initial_balance_micro / 10`) — source-grep + runtime assert |
| Q2 | 4 admission arms all gain precondition; no arm silently bypassed |
| Q3 | rejected attempts make ZERO state mutation (pre/post byte-equal across 4-arm coverage) |
| Q4 | tail-appended `RejectionClass::BankruptcyRiskCapExceeded` preserves golden-digest invariants |
| Q5 | Display ≤ 64 bytes; low-pollution per CLAUDE.md §15 |
| Q6 | AutopsyCapsule emit calls `is_autopsy_active_at` activation gate; replay-determinism witness |
| Q7 | Gap-A `reputations_t[verifier] += 1` only on accepted VerifyTx; rejected paths don't touch reputations |
| Q8 | Gap-B bond-return surface lands as designed (Option B1; FinalizeReward extension) |
| Q9 | source-grep gate: no f64 / no shadow ledger / no global Markov pointer reintroduction |
| Q10 | existing TB-N* / Stage C / Wave 3 50p / TB-N3 / G1 / G2 / G2P / G3.* / persistence smoke unchanged (no test regression) |
| Q11 | admission ordering Q5 (risk-cap first) matches architect verdict |
| Q12 | `tests/constitution_g2p_verify_reward_bond_return.rs` SG-G2P.6.c negative-witness assertion behavior: re-scope analysis (G3.2 lands the surface; OBS row closes on ship) |

## §5. Auditor self-check: blocker classes

Issue conviction high if you find ANY of:
1. **Hidden mint** — risk-cap admission accidentally creates Coin (CLAUDE.md §13 violation).
2. **Schema break** — WorkTx / VerifyTx / ChallengeTx / BuyWithCoinRouterTx struct schema change (architect §1 clause 4 forbidden).
3. **Silent admission bypass** — any of 4 arms doesn't fire the risk-cap precondition.
4. **Predicate-fail order violation** — risk-cap fires BEFORE predicate gates in WorkTx arm (would lose predicate-fail telemetry for bankrupt agents per architect §7.2).
5. **Stale parent gate bypass** — risk-cap admission doesn't fire AFTER parent-root match (allowing pre-StaleParent rejection on bankrupt agent).
6. **CTF break** — verifier bond return mutates `stakes_t → balances_t` without preserving total Coin sum.
7. **Reputation double-count** — Sybil guard insufficient (e.g. Step-5c fires before Step-5b insert, creating race).
8. **Autopsy replay drift** — `derive_g3_2_terminal_summary_bankrupt_autopsies` not deterministic given (econ, ts, round, t).
9. **Privacy leak** — autopsy capsule publishes private_detail bytes to public surface.
10. **Trust Root inconsistency** — sequencer.rs / typed_tx.rs / mod.rs hashes in genesis_payload.toml don't match actual file SHA256.

## §6. PROCEED / CHALLENGE / VETO verdict format

Return verdict in one of these forms:

```
VERDICT: PROCEED
Conviction: HIGH | MEDIUM | LOW
Coverage: <N>/<N> ship gates witnessed
Notes: (optional non-blocking observations)
```

```
VERDICT: CHALLENGE
Round: 1 (of 2)
Conviction: HIGH | MEDIUM | LOW
Issue: <specific defect with file:line>
Reproduction: <command or witness>
Recommendation: <minimal remediation>
```

```
VERDICT: VETO
Conviction: HIGH | MEDIUM | LOW
Blocker: <which §5 class>
Issue: <specific defect with file:line>
Reproduction: <command or witness>
Recommendation: <required remediation>
Rollback note: <impact if reverted>
```

## §7. Validation evidence summary (pre-audit baseline)

- **Constitution gates**: 434/0/1 GREEN (was 402 pre-G3.2; +32 from new gate `constitution_g3_bankruptcy_risk_cap`).
- **Trust Root**: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASS.
- **Workspace**: `cargo test --workspace --test-threads=1` PASS (pre-existing parallel-test env-var contamination in `constitution_router_buy_with_coin` confirmed unrelated; passes single-threaded).
- **Fixture witnesses**: 3 deterministic fixtures exercise the autopsy emit path end-to-end (bankrupt agent → 1 capsule, solvent agent → 0 capsules, multi-agent mixed → 2 capsules in deterministic order).

## §8. Output destination

- **Codex G2 verdict**: `handover/audits/CODEX_G2_TB_G_G3_2_VERDICT.md`
- **Gemini DeepThink verdict**: `handover/audits/GEMINI_DT_TB_G_G3_2_VERDICT.md`

Both auditors must reach PASS for ship. Conservative merge if conflict: VETO > CHALLENGE > PASS.
