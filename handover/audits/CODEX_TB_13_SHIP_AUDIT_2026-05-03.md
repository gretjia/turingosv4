OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019dec97-fc17-71c0-9db4-dad847868937
--------
user
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


warning: Codex could not find bubblewrap on PATH. Install bubblewrap with your OS package manager. See the sandbox prerequisites: https://developers.openai.com/codex/concepts/sandboxing#prerequisites. Codex will use the vendored bubblewrap in the meantime.
codex
I’ll audit this as a code review first: read the governing instructions, then trace the TB-13 transaction schemas, sequencer arms, invariants, tests, and smoke evidence against the mandated questions. Because this session is read-only, I’ll verify what I can from the workspace and call out any inability to write the final audit file.
exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'git status --short && git rev-parse HEAD && git log --oneline --decorate -8' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'src/**' -g 'tests/**' -g 'handover/**'" in /home/zephryj/projects/turingosv4
 succeeded in 131ms:
7aac629 (HEAD -> main) TB-13 Atom 6(a.5) — Real-LLM regression smoke (chaintape audit)
17d4a3b TB-13 Atom 6(a) — Recursive self-audit (Class 3; PASS pending external dual)
1806432 TB-13 Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests (Class 3)
70303af TB-13 Atom 1 — typed_tx schemas (Class 2; +8 unit tests)
32aab27 TB-13 Atom 0 + 0.5 — Charter + legacy f64 CPMM forward-fence (Class 1)
fa36eca (origin/main) TB-12 SHIPPED — Node Exposure Index (Class 3 dual audit; Codex+Gemini PASS post-remediation)
975108d TB-12 Atom 6 ship-gate refinement — 4 SG-12.x strict architect-name alignment
71053fd TB-12 Atom 6 — recursive self-audit + Codex + Gemini dual audit (Class 3; PASS)

 succeeded in 140ms:
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/run_a_n1_mathd_algebra_171/evaluator.log
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/run_a_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/run_a_n1_mathd_algebra_171/verify.log
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/run_a_n1_mathd_algebra_171/agent_pubkeys_for_witness.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo.tar.gz
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/cas.tar.gz
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/replay_report.json
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/runtime_repo.tar.gz
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/agent_keystore_at_exit.enc
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/evaluator.log
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/dashboard.txt
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/verify.log
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/regression_n1_mathd_algebra_107/agent_pubkeys_for_witness.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/verify.log
handover/evidence/e1_jsonl/E1_B_seed2357_n8_20260424T045345.jsonl
handover/evidence/tb_9_durable_identity_smoke_2026-05-02/README.md
handover/evidence/e1_jsonl/E1_A_easy_ctrl_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_A_seed31415_n8_20260424T012605.jsonl
handover/evidence/e1_jsonl/E1_A_seed2718_n8_20260424T024005.jsonl
handover/evidence/e1_jsonl/E1_B_seed31415_n8_20260424T012605.jsonl
handover/evidence/e1_jsonl/E1_B_heterogeneous_n8_20260423T144327.jsonl
handover/evidence/e1_jsonl/E1_A_homogeneous_n8_20260423T144314.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/verify.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/9d/9af26aede4560bbdc0d47bbe2ed4742e12b36e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/d0/962bda292978c004e7091f5ed74680de90aa5c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/dd/7a2273e4d06353893b075d8e774f6921482b33
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/34/5136acba1a2736506669a6120452b49edf404c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/97/a4c8569c91b2b32b2aba0363f8b8485843ec90
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/31/39194f39a754b511c59f6f94eae2b54eb473b3
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/f2/78efc63058d7db2d9b8dd6c09ae682e22da90b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/64/959403196efa12ea3065b9d7b6728d48886c21
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/64/6044197f188253149e5dc172dd4003fbea6791
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/63/0113327506ce13ebbd14b476bc3d3445c414d7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/63/bc98f00dba588953b309a92aaac80d8af324ea
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/4e/43e4b6e735e3a535ba4f9afa611b535dfda87a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/8e/8d636d625651b83781321db7a7058954474a1c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/10/6873b0ab019daf60b77824bb2a58af3a247815
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/15/ea38411e2ea8a200cbd74a4d906e561c678ab2
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/7b/8962f5535810b0cc4cf13ff85912fd43bd8b39
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/objects/2e/2500cefd8f3fd76cab64891b407c42646e43b3
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/22/faa12086e7fc789fc0ad9db2177574694c0752
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/97/38dec97139e53141f498b0ef1fbaa5dd09a7b7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/65/1086b378629de7f62f5dd7d478906800c7c765
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/da/bea13c15d8fe3153ee2b3ab2c6ab165635826a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/7d/a785df66690e7c38f789f05b43f6aa91f7f769
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/f2/a68cba908a8a4243415ceaeff79c0f2f1df154
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/5c/fc83c905fc44b9fe4fc0cb745d5a222ec2ed0c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/26/da335ccadb22734b737bb59cc142dffd1132de
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/6d/86ccad1621c29006b997d7eb6dc2bd6e98af34
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/0a/9e12fb6ec5c0b370bd1d39dbafc29fe0fcf3c8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/bb/5bc36f3b1ce8735218c2ee399b2e16ba52d0a5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/b9/bc5ac749cdd82b1f2c80e387d7a52c83763ef6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/47/75f0c55e2420666317b5682053c49fcf535d87
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/fc/57459ab1ea673259bf75a7e080e1d3bb6c5642
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/6b/db4f4de4f536dc722db0ffab23ed97043d536a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/62/0d2b29b0739346e457354318aafd6feba07eb8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/e1/6406f4caa9ccdd20669117234ede5a3bf59ebb
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/ff/1ff61dcfe7e3109adf9b0a0dd1c41947d96cfe
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/6f/3952e091d34d860ec78864760a54e7bcc4505c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/23/b52cd7e41860067b8b38d067cff32a08506bd2
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/19/7f6c273b435b6843f7884d786b0c7d9bd9ff14
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/96/f88be33ba280c7bdaf78b9cef2f849e7bf5e65
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/59/14c3ef9f750baadd3d4e905681d58d2004d338
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/b9/7946e285d76d006134b9118371832d46e46dcb
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/b2/c07601585db195bc7251c8757469e9e04f8d4f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/b3/0825cf82726704f7ec585679364cc8ec6bf370
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/40/6717abafb43f69bd0fe3ac3feecae784fc38f4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/7c/f325af93306b30c6ce772ad016551da3d38dd1
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/c6/189984383b15ec92683053da2ff6a581b58dc9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/a4/ebe07b414f46dc9a9b443676b1d0c520bb0b1e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/objects/0f/d0927d69a4334b4237284d1fb5cafe395943d2
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/.git/objects/8f/9ce539351b228f0d655d3c7edd1b4179074281
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/verify.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/ee/e1a8e29aa11def0c8c8c136c4b364c8a7857f7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/run_summary.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/11/68e36dd34f06624b7a25889500d32d243f2773
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/32/4ad034550db1f10d9e7947e2fb371f243fd73e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/81/3ddc6b5218b4620cc903e3771802aa6857811b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/a6/63db7c0fc215706bf00cb0d4441f5c4f0d8f0c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/ef/da8badb7c45ad9f40978d99afcceefac2d8bfe
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/15/4652a1346fbf1bedfe50c9581ad7cbb3c38206
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/45/a58dca77e8d2a3e2423ed4458a8412c70182b5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/c8/fd92b90c11f83522f8b1ba6eb5dc70be41e112
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/10/f976a6889f8c078ab258d46986a9c09b4bfba9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/3d/a1335bd14b9feba127b8b3def08e7097dde0a5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/dd/e11e8b5b6f25edd1d6daab2a5f29a3b0f70300
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/df/99630be61925be736cdd2432acf49db7725cbe
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/33/daccdafe1d16a6da6b28058f50231f0cffbd25
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/.git/objects/6e/c4704545bb7588809df915d2e5fdae77fb2591
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/8c/6878662b6f520866ad25f2ca01971b46e17d2c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/05/b7a123a4a1d692a3f31b12212c9db6555c111d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/30/3f551d2959adaf3777724cebca38ba77592780
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/e4/c7206f28272f0ccb916e919e7678b89a97825e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/objects/42/f7ea48eb3e8bba940855eeef8aaf49309c883a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/95/74c86637fd032d5cb71bebe7c73fb694d244b9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/71/cf92879f4fe416920e0eb8d23a647a9920680b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/73/22a865588e0165fea00fc687b0d2d3832e488d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/38/6d95c2481339cc2ddcd44bc8a9110197c6f5d5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/3a/eadcc3da7cb151eb8a1805889ab0624879f80f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/22/cd95b2aedc887fc8469e5489a5cc14cf3c94a6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/ab/edac724651ecf03050add7e7ea46cf93d69305
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/f1/a83043e19ec4f19bb966cd138b00b0eb078708
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/33/44504a1a7abcd0778ba050e8e0c3276f9c53f6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/17/16484712bb54aa82626e839d5371be98f83cdb
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/92/513e4f1372cf881a07a8f22c4f70688aa57abb
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/16/d052fbe1fbb2b5c7f011dfa964a4b1c725cadd
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/d6/22e44838eb6c2780a59ab60d5b1c2a14afbde8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/b5/1bfb631499b342530d6891b40c6364fd45cfe5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/47/8e6fea8e84074417ada1aa04b97edd71af6d28
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/7f/17a9b7f719e512d7b3060c6bd43b4bb197af1b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/69/6b4f533e2dd5e25f4095852f14f92db4c7dc9a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/6b/abc7063022342e2fb31b603de1ab972109865c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/4a/511a0c625f3b018ead44cc1bcbb44203699b20
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/40/85d512b9697fb64527ac90f3fa547250558df4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/objects/8a/34224259ead07aa513860ec1c1928a95bf5dd5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/verify.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/5b/43191f1017b354d6042acf0f29d5a22679dc83
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/88/9e58eea0690693fbde0daa64f69e64b5f733b1
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/c0/a3683334acfb10b9749531736a53de8d802c34
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/90/cba67f81da294c7eca3042dcb09e2f7aa52dbd
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/f5/9adbb586bba952ac1f47e419c56b3c707da74b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/3f/13f55bd6ac45b76f94da99d128d55f4bd6bf3e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/a5/b67f7e2904812443c288b8075f7ef7623f950f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/b5/a67f2ce0844dbba19dc3810dbbaf7a901c8a54
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/7a/66b55285ce8ca22f6b44656b1e8c4ed60b4f1a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/5c/854176ca4923cd8b8b9098fe2b8981a9f75b7e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/ee/bf8af4936c9d2361948d90d63de5797f353844
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/60/0702e94d6aee04f5d4cb2516f89b8a8a787c7f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/24/48973b0aeacfa50f548b5112b281afb4170d2b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/53/b86ce4157c5f079bc17815389183ee9ad1659f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/ef/e2f853929bb1330b19a9e5c04d29eb4d4cbaa8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/c7/1a3a0c9a8317ca5e051789589babee044f3543
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/82/98e5c0ad35740e7fcf4b41ce05adf4a2bc236a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/a1/d1f37ca97951b28276236e4345e98cdb258c03
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/de/468f04a109ac15cb9d39259fbfb442140aaac8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/57/2d82a1ce4d0d05aac30ad95e4eb12c066e0fc1
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/69/8a2a0563b9a5c03222e87e04a37825b61b4644
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/5e/d19cf7800703048a0daf16a99a9ae59a3a3e57
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/52/822a55163c1648c0ad98f87682880d16964e6d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/objects/55/ce998ad883350a714f673abda964a1375502a6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/.git/objects/f8/4822b3182c280fcaacec1d11729b93005c4bb4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/c2/f1615d6e517231fe7f5748901dee58f394943e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/b6/85c39dc889891a30a312d37a561b8fb2854276
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/ba/f17295e29e2f4a85fd4700c0588e78cef7a7ea
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/f5/2cdd442c7c13bf7b424f1e5f70642577ffeb2c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/1c/bf252ef1d6c22abae1d9da406dd7dd79a7b8b9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/eb/19b864a687ce8cd25f9b0cc484ae81974d274e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/1b/43da565fa0008718ee604f99ddf9cde31d8413
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/76/2953a1c9295cda026c10aa0a606e19e62fc761
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/00/3c4dffa2647d86edd32bbb6cb7a0aeddce4e69
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/c4/3f0c70fc03e098289797546cf23fdc4cc7bfb2
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/f3/28ed5cff3308d38d837a7f8d0a36b946641ae0
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/a5/ee02c61b38bcde53b0626a9d58fd416ae342e4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/0c/dc1902ce50a8fc73d77a32e7101c68841d38f9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/b9/d7ed237da1ee75f2d681039cd2927cbc869bd5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/24/6cc4ad159b41ae1f921baa7729975911a80b36
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/d6/b8a5df18fab45c04d80d3ca6105ef7c83aec7e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/47/0bc66963e028d86fe70054586a76ed61481c6c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/21/4c1697bdc32e0f8945eee38836e796a765bb97
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/b3/d2ec2d21b40b83aa85fdf1c0e81fba9a79e3d7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/21/fad9006875f6582ce20f19475103d450d9a272
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/ce/1d31e68ccdd6945a2275c789b4338ec321d82c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/c3/bf731939ffabab68c27af70a690b31e8f0bf96
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/2a/3c18721da5e5fe7900c84bfb0d425b25448db6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/.git/objects/9e/214b8d68661497430dbd71cdff4201bd687e47
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/verify.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/verify.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/README.md
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/replay_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo.tar.gz
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/39/a4e63baaabb34fc73351a5401dd0b60c3ec546
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.turingos_cas_index.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/dc/c6b24fcaa588f7fb1b0ff895e5522ded8f84c1
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/4f/6668980830d031180a1dac402a46ebf9cabeb4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/45/118f2d0e8d206f53f4f5589ca2c80868112031
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/67/11d3c9ffeda02534768857ad867a0cff9ee6e0
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/2c/b0aaf24bcd5c28bc86b5630ff6352c1657ce05
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/be/ceae5a28a03df37049dd234da0ef17ed2a759e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/cf/5692e8183b18a747e9c4bd5bc5a4ba7c3b465a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/78/ad0ddc16ad0fa4bf1772487ca9e3fc23f4d9f7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/78/ee4debc719dc64fee53c4f85844807593ff015
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/ef/88bf56a251d6c135d91d677c5b9d42e946d6ac
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/21/b0fda88aff1a3ea73be9deb04cc491ec4f7d7a
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/a4/93993c3e4c7e2536a4898e254bb4dc22bc910e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/85/b6ba767e3150048c24a21f03190210d959b098
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/c8/728b868727f0f5a85dd61efd183b22f77464fe
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/03/2b6619e111ca8f4ed6cfacbb221cc7347c52a4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/d8/914e70116468cb63e4265c54fd650f357ce495
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/39/815a0147d6781e3e83165f664058f75102643c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/71/bd553515fc99f4be0d44b102d1a04f98b35222
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/f2/52fd9c3619cb2d1d82f34eea4142e134c5363b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/d0/b9228ee4f8c98197cc6ebd00391db1e8e6ea32
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/93/84751e874b6cd5733089f34a53974001cc6080
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/b7/382ce570845491f1f31694f9c3fd6c566dd4fc
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/74/1f14757179b1f3b14f2b983e35045704a00415
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/2f/9b1ce354dba24f9be0c9ef8db24ec23bfe29a8
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/0b/8541a28a9368d85f6ca73d2d8d6ca194a74960
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/objects/8f/5bbaa7fd75ae4849d0e17d3e09998124426ca4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/2b/d133f47eaf566ccef60a19de2a70715870d5e1
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/2c/8b66d6de13378b1df235b46d1da7b94f9cb4e6
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/20/d5492019a10b0b2dab551b89f79285379b3da9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/08/a44012542addc193d85f249cde2612a68f5325
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/ac/7addef21d052b5612d4ca6e013b7ec7002cbe3
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/92/c026438071695c2777083b49bf6450413d79a0
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/08/5f9d6f39db49a995e533d7a0d0614f36dd50c5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/b3/91d0b52f590c8950a3ebe8e7e74bc525d294d0
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/b1/162f52bb543528cbcf6d07633fbe0672891e48
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/63/4f813e8a75b9b3846b29e99c0ef4713213b86b
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/69/85f6bda134d7d7f65cfef033a8a9d33c6204b7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/objects/d2/b3a6b5482d1091964693210d9ce20296163148
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/b5/65148014b46c70b884ae07cf31318e5574c6a2
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/6b/c19735e4a5667a82f8c985f2dcdf50e9295f9e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/40/3573bc3775f78efc1435fd13753a7a7d78bc83
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/3d/dee045d6b4a46001ee8aac94bb4993d8452458
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/b7/9fe9e457062bec432e18beb6a83d9b59c8f5a4
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/44/a705999c07de31a5501dcde715b9a65643fd1e
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/85/2e566927adecc357c3b12597be0e025f9d75ab
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/c9/c946e1e0135f3fa593d01d63f0c45a8114e38d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/d7/a6e2dcf3c623123493c60fbd1d552a0b56aaa5
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/3d/fc3c17bb7d1ac37eb486466f5be657bbb79d81
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/5b/c62b0ca3b4eee691488d92bd5a704e99d9edf3
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/42/7c5978b9afff191faa1d0ba1b972ba6dc8ca45
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/3b/58c1aa8f83ebebbf576f5c649e0d99ac452801
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/.git/objects/9c/20967809c4a20b8fbb3e67633966064eb76379
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/bb/c20afd251f2686554db7c00bb0e5ed2305c083
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/bb/a5417ea4039856cc320786ced5d43a024a9529
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/23/233c4bc269f996f2f82d61adbde0ae83eb50e0
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/d6/2264e462eda53bc5a426ae5ad8325410abad34
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/37/9b6f29ab8a0ce69e8044f7701a89441f0c01ef
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/ef/c83f47e9e8fda4b4b5f346ee80f0775aa1b757
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/69/4275e8221da1b4d6a419d166d6c549ced7cf6d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/e9/78817a08515020e5d1c35bf14914080a09523f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/e9/06dcc5e6d9252af04cef8352cdec8582c68346
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/9e/68cdb1edb307a34ba268d0483c369c854d8980
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/bd/d9f324b0c4529d966bed889b211bafecff2855
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/25/6ab03f731ddb94eefacd3a317d34a76aaf5647
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/0f/bfc2b2cc2fcce98ae0b9ecb0602ea59aa4f6ff
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/cd/3f4469a201b43070f58a256ec2c34caed8c620
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/e8/850dc4fe39ba1a485f67953c41ed8543f19f4f
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/evaluator.log
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/dashboard.txt
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/verify.log
handover/evidence/README.md
handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
handover/evidence/tb_7_chaintape_smoke_2026-05-01/chain_derived_run_facts.json
handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
handover/evidence/tb_7_chaintape_smoke_2026-05-01/README.md
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/.git/objects/36/58abad1da550d042798204912fe6e472a200a9
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/stdout
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/stdout
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/stderr
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/stderr
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/stdout
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/stderr
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/run_summary.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/genesis_report.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/rejections.jsonl
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/agent_pubkeys.json
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/info/exclude
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/config
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/32/5313d478ef313d799638b9a1d64a13f94e292c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/7d/68cba0e96b0008d05a10d52e34ec50e6efeff9
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/78/17541522a3ef7b989dc90605abd01747ac30ea
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/a1/20e5344ea86b956bd634e6f68496223cb21892
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/4c/1ec8afcc0a5544c9cd4c0aae6e1eaa51a8f0b7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/6e/eb9a34545ded1462422447f482524b8bf15a71
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/e4/975148b1c981a046aec22a2584c94a087a3c3d
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/14/97d4e7be50c6722210f66f57628b438f5784b7
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/ea/87eaeaaff81f86eaf7064ed2358d9b90114e5c
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/HEAD
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/description
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/push-to-checkout.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/objects/e8/e71052dd66e8f388fc54bb826ed2018085b958
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/fsmonitor-watchman.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-receive.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/prepare-commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/refs/transitions/main
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/post-update.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-rebase.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/applypatch-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-push.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-merge-commit.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/commit-msg.sample
handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/.git/hooks/pre-applypatch.sample
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/stdout
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/stderr
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/run_summary.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/stdout
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/stderr
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/stdout
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/stderr
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/README.md
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/genesis_report.json
handover/evidence/e1_proofs/mathd_algebra_44_1776942255_6188d1b1.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776864088_c8854dcf.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776999212_2d5c2db8.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776864287_f1beef26.lean
handover/evidence/e1_proofs/algebra_apbon2pownleqapownpbpowon2_1777010656_fed75b88.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776869774_7c6d7e9.lean
handover/evidence/e1_proofs/imo_1962_p2_1776996434_6277f804.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776994690_510aa55a.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776873440_7c6d7e9.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776999097_6277f804.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776999060_e57b2dca.lean
handover/evidence/e1_proofs/mathd_algebra_44_1776956522_f7af89d8.lean
handover/evidence/e1_proofs/mathd_algebra_332_1776994561_16871d44.lean
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/replay_report.json
handover/evidence/e1_proofs/imo_1962_p2_1776869602_faeefc7.lean
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/stdout
handover/evidence/e1_proofs/imo_1962_p2_1776958185_b7329f9c.lean
handover/evidence/e1_proofs/imo_1962_p2_1776951132_82f02b76.lean
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/stderr
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/run_summary.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo/agent_pubkeys.json
handover/architect-insights/ECONOMIC_MECHANISM_AUDIT_2026-04-26.md
handover/architect-insights/2026-05-02_flowchart_hashes_and_trace_matrix.md
handover/architect-insights/CO_P0_EXIT_REPORT_2026-04-27.md
handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md
handover/architect-insights/SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md
handover/architect-insights/V4_1_METATAPE_PLAN_v1_2026-04-27.md
handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md
handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md
handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md
handover/architect-insights/ENACTMENT_PROCEDURE_2026-04-27.md
handover/architect-insights/THESIS_V2_GROUND_TRUTH_AUDIT_2026-04-25.md
handover/architect-insights/CO1_13_PHASE_DRIFT_REVIEW_2026-04-29.md
handover/architect-insights/AMENDMENT_2026-04-26_art-0-turing-fundamentalism.md
handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md
handover/architect-insights/CO_MEGA_PLAN_2026-04-26.md
handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md
handover/architect-insights/2026-05-02_polymarket_absorption_guards.md
handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md
handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_CODEX.md
handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md
handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md
handover/architect-insights/B7_EXTRA_ABSTRACTION_DEPTH_FINDINGS_2026-04-25.md
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/replay_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/stdout
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/dashboard.txt
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/cas/.turingos_cas_index.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/stderr
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/pinned_pubkeys.json
handover/specs/INV8_DAG_DETERMINISM_SPEC_v1_2026-04-27.md
handover/specs/CO1_7_TRANSITION_LEDGER_v1_2026-04-28.md
handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md
handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md
handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md
handover/specs/CO1_1_4_PRE1_TYPED_TX_ABI_v1_2026-04-28.md
handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md
handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md
handover/specs/META_TX_SCHEMA_v1_2026-04-27.md
handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md
handover/specs/PRE_COMMIT_HOOKS_R022_R023_v1_2026-04-27.md
handover/specs/ART_0_2_REINTERPRETATION_2026-04-27.md
handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla
handover/specs/CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md
handover/specs/SPEC_WALKTHROUGH_v1_2026-04-27.md
handover/specs/META_TRANSITION_INTERFACE_v1_2026-04-27.md
handover/specs/AMENDMENT_FLOW_FORMAT_v1_2026-04-27.md
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/genesis_report.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/rejections.jsonl
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/agent_pubkeys.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/initial_q_state.json
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo/agent_audit_trail.jsonl
handover/preregistration/PREREG_AMENDMENT_v2_2026-04-26.md
handover/ai-direct/PAPER_1_OUTLINE_2026-04-22.md
handover/ai-direct/HARNESS_UPGRADE_PROPOSAL_2026-04-23.md
handover/ai-direct/SESSION_REPORT_FULL_2026-04-21.md
handover/ai-direct/DECISION_TREE_GATE_8_TO_PHASE_9_2026-04-22.md
handover/brainstorms/run_codex_n_agents_brainstorm.sh
handover/ai-direct/M7_APPEND_STAKING_SPEC_2026-04-22.md
handover/ai-direct/HANDOVER_TB_7_7_D7_RESOLVED_2026-05-01.md
handover/brainstorms/run_gemini_n_agents_brainstorm.py
handover/ai-direct/M8_BONDING_CURVE_LP_SPEC_2026-04-22.md
handover/brainstorms/GEMINI_N_AGENTS_BRAINSTORM_2026-04-25.md
handover/ai-direct/DRIFT_AUDIT_20260427.md
handover/ai-direct/TAPE_ECONOMY_v1_2026-04-20.md
handover/brainstorms/CODEX_N_AGENTS_BRAINSTORM_2026-04-25.md
handover/ai-direct/CHECKPOINT_PHASE_7_TURING_2026-04-21.md
handover/ai-direct/CHECKPOINT_PHASE_0_2026-04-20.md
handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md
handover/ai-direct/STEPB_ART_II1_REDESIGN_2026-04-16.md
handover/ai-direct/E1_FINAL_4SEEDS_2026-04-23.md
handover/ai-direct/PLAN_PHASE_8_TO_10_2026-04-22.md
handover/ai-direct/CHECKPOINT_PHASE_2_2026-04-20.md
handover/ai-direct/EXT_AUDIT_2026-04-21/README.md
handover/ai-direct/EXT_AUDIT_2026-04-21/codex_constitutional_correctness.md
handover/ai-direct/EXT_AUDIT_2026-04-21/claude_internal_findings.md
handover/ai-direct/EXT_AUDIT_2026-04-21/deepseek_mechanism_design.md
handover/ai-direct/EXT_AUDIT_2026-04-21/gemini_math_algorithm_perf.md
handover/ai-direct/EXT_AUDIT_2026-04-21/run_audit.py
handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md
handover/ai-direct/PAPER_1_OUTLINE_v2_E1_LED_2026-04-23.md
handover/ai-direct/CHECKPOINT_PHASE_2_1c_2026-04-21.md
handover/ai-direct/ROADMAP_LAUNCH_2026-04-21.md
handover/ai-direct/CHIEF_ARCHITECT_REPORT_2026-04-23.md
handover/ai-direct/E1_EMERGENCE_VERDICT_2026-04-23.md
handover/ai-direct/CHECKPOINT_PHASE_1_2026-04-20.md
handover/ai-direct/PAPER_1_FULL_DRAFT_2026-04-23.md
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/brief.md
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/amendment.diff
handover/ai-direct/EXT_AUDIT_PHASE_8_R1ALPHA/run_gemini.py
handover/ai-direct/M1_DYNAMIC_GAMMA_SPEC_2026-04-22.md
handover/ai-direct/HYPOTHESIS_CHAT_MODEL_2026-04-15.md
handover/ai-direct/HANDOVER_TB_7_7_D7_PENDING_2026-05-01.md
handover/ai-direct/PLAN_PHASE2_2026-04-17.md
handover/ai-direct/AUDIT_V3_2026-04-15.md
handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/brief.md
handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/run_gemini.py
handover/ai-direct/EXT_AUDIT_PHASE_8A/brief.md
handover/ai-direct/EXT_AUDIT_PHASE_8A/phase_8a.diff
handover/ai-direct/EXT_AUDIT_PHASE_8A/run_gemini.py
handover/ai-direct/EXT_AUDIT_PHASE_8_BATCH/phase_8_batch.diff
handover/ai-direct/CHECKPOINT_PHASE_2_5_2026-04-21.md
handover/ai-direct/TB-3_RSP1_FORMAL_TX_SURFACE_2026-04-30.md
handover/ai-direct/DRIFT_AUDIT_20260419.md
handover/ai-direct/OPEN_DECISIONS_2026-04-26.md
handover/ai-direct/HYPOTHESIS_PERCOLATION_2026-04-16.md
handover/ai-direct/CHECKPOINT_PHASE_6_EMERGENT_2026-04-21.md
handover/ai-direct/STEPB_ART_II1_V3_2026-04-16.md
handover/ai-direct/PAPER_1_v2_DRAFT_SKELETON_2026-04-24.md
handover/ai-direct/HANDOVER_PHASE_A_EXIT_2026-04-26.md
handover/ai-direct/DRIFT_AUDIT_20260415.md
handover/ai-direct/PLAN_2026-04-14.md
handover/ai-direct/TB-6_PRODUCTION_CHAINTAPE_BOOTSTRAP_2026-05-01.md
handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/per_problem.tsv
handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/brief.md
handover/ai-direct/EXT_AUDIT_PHASE_2_AB_2026-04-22/run_gemini.py
handover/ai-direct/AUTO_RUN_SUMMARY_20260417.md
handover/ai-direct/ART_V_MIN_DESIGN_2026-04-22.md
handover/ai-direct/CHECKPOINT_N50_HONEST_2026-04-21.md
handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/brief.md
handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/amendment.diff
handover/ai-direct/EXT_AUDIT_PHASE_8_R1R8/run_gemini.py
handover/ai-direct/CHECKPOINT_PHASE_4_2026-04-21.md
handover/ai-direct/E1_FINAL_VERDICT_3SEEDS_2026-04-23.md
handover/ai-direct/GENERALIZATION_ROADMAP_2026-04-22.md
handover/ai-direct/PLAN_2026-04-14_v2.md
handover/ai-direct/PHASE_2_5C_VERDICT_2026-04-22.md
handover/ai-direct/REGISTRATION_PHASE_9_2026-04-22.md
handover/ai-direct/TB-5_RSP3_RESOLUTION_GATE_2026-04-30.md
handover/ai-direct/HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md
handover/ai-direct/PLAN_V3_2026-04-15.md
handover/ai-direct/TB-4_RSP2_ADMISSION_SURFACE_2026-04-30.md
handover/ai-direct/PLAN_FINAL_PHASE_8_TO_PAPER_2026-04-22.md
handover/ai-direct/PLAN_PHASE3_CONSTITUTIONAL_LOOP_2026-04-17.md
handover/ai-direct/STEPB_ART_II1_NECESSITY_2026-04-16.md
handover/ai-direct/STEP_B_PROTOCOL.md
handover/ai-direct/N3_DIAGNOSIS_2026-04-15.md
handover/ai-direct/PLAN_V3_1_2026-04-15.md
handover/ai-direct/PLAN_V3_2_2026-04-15.md
handover/ai-direct/V4_PROJECT_OVERVIEW_2026-04-27.md
handover/ai-direct/M4_SATOSHI_REBATE_SPEC_2026-04-22.md
handover/ai-direct/LATEST.md
handover/ai-direct/PAPER_1_THESIS_ANALYSIS_2026-04-23.md
handover/ai-direct/DECISIONS_2026-04-22.md
handover/ai-direct/PHASE_Z_PRIME_STRICT_ALIGNMENT_PLAN_2026-04-22.md
handover/preregistration/sample_E1v2_hard10_S20260423.txt
handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md
handover/preregistration/PREREG_E1V2_HETEROGENEITY_2026-04-23.md
handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md
handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md
handover/preregistration/PROXY_SATURATION_FINDING_2026-04-24.md
handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md
handover/preregistration/E1v2_RESULTS_2026-04-24.json
handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md
handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md
handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json
handover/whitepapers/REVISION_NOTES_2026-04-27.md
handover/preregistration/hard36_pool.txt
handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md
handover/preregistration/README.md
handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json
handover/preregistration/scripts/run_p0_calibration.sh
handover/preregistration/scripts/draw_hard10_pput_ccl.py
handover/preregistration/scripts/compute_p0.py
handover/preregistration/scripts/split_pput_ccl.py
handover/preregistration/scripts/analyze_c3_h1_h4.py
handover/preregistration/scripts/run_c2_phase_c_ablation.sh
handover/alignment/DECISION_POLYMARKET_CORE_2026-05-02.md
handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md
handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md
handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md
handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md
handover/alignment/OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md
handover/alignment/TRACE_MATRIX_v0_2026-04-22.md
handover/alignment/TRACE_MATRIX_v2_2026-04-25.md
handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md
handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md
handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md
handover/alignment/CODE_CANDIDATES_2026-04-22.md
handover/alignment/TRACE_MATRIX_v1_2026-04-25.md
handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md
handover/alignment/STAGE5_VALIDATION_REPORT_2026-04-22.md
handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md
handover/alignment/OBS_ROADMAP_POST_TB7_OVERRIDE_2026-05-01.md
handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md
handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md
handover/alignment/DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md
handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md
handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md
handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md
handover/alignment/TRACE_FLOWCHART_MATRIX.md
handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md
handover/alignment/DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md
handover/alignment/FC_ELEMENTS_2026-04-22.md
handover/CHECKPOINT_TB7R_2_2026-05-02.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_D_verification_asymmetry.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_A_lossless_integrated_edition.md
handover/directives/2026-05-01_TB7_ARCHITECT_RULING.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md
handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md
handover/directives/2026-05-02_TB6_NEXT_SESSION_PROMPT.md
handover/directives/2026-04-29_9_phase_roadmap.md
handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md
handover/directives/2026-05-01_TB7_ARCHITECT_REVIEW_REQUEST.md
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2718_n8_20260424T080943.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2357_n8_20260424T080945.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s31415_n8_20260424T080941.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s31415_n8_20260424T080941.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T080939.jsonl
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.jsonl
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s2357_n8_20260424T080945.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2357_n8_20260424T080945.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s2718_n8_20260424T080943.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s31415_n8_20260424T080941.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T100952.jsonl
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_Abl_s141421_n8_20260424T080939.jsonl
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s141421_n8_20260424T080939.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_B_s141421_n8_20260424T100952.err
handover/preregistration/discarded_12way_run_2026-04-24/E1v2_A_s2718_n8_20260424T080943.err
handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_B_group_intelligence.md
handover/directives/2026-05-01_TB6_ARCHITECT_RULING.md
handover/directives/2026-04-30_TB4_directive.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md
handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_B_first_plan.md
handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md
handover/directives/2026-05-01_TB7R_CONSTITUTION_ALIGNED_REPAIR.md
handover/directives/2026-04-30_TB5_VETO_redesign_directive.md
handover/directives/2026-04-30_TB5_audit_mode_supplement.md
handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md
handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_A_appendix_C_turing_machine_philosophy.md
handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md
handover/directives/2026-05-01_TB6_ARCHITECT_FULL_PROMPT.md
handover/proposals/HERMES_AGENT_INGEST_PROPOSAL_2026-04-26.md
handover/tracer_bullets/TB-1_recharter_2026-04-29.md
handover/tracer_bullets/TB-2_charter_2026-04-30.md
handover/tracer_bullets/TB-10_charter_2026-05-02.md
handover/tracer_bullets/TB-1_day1_oneshot.jsonl
handover/tracer_bullets/TB-7.7_charter_2026-05-01.md
handover/tracer_bullets/TB-11_charter_2026-05-02.md
handover/tracer_bullets/TB-1_day1_spike_2026-04-29.md
handover/tracer_bullets/TB-7R_charter_2026-05-01.md
handover/tracer_bullets/TB-7_charter_2026-05-01.md
handover/tracer_bullets/TB-9_charter_2026-05-02.md
handover/tracer_bullets/TB-13_charter_2026-05-03.md
handover/tracer_bullets/TB-8_charter_2026-05-02.md
handover/tracer_bullets/TB-3_charter_2026-04-30.md
handover/tracer_bullets/TB-4_charter_2026-04-30.md
handover/tracer_bullets/TB-12_charter_2026-05-03.md
handover/tracer_bullets/TB_LOG.tsv
handover/tracer_bullets/TB-6_charter_2026-05-01.md
handover/tracer_bullets/TB-5_charter_2026-04-30.md

codex
I’ve confirmed the branch is at the requested TB-13 HEAD (`7aac629`) and the worktree already has unrelated dirty/untracked files. I’m going to avoid touching or relying on those except for the TB-13 audit target paths.
 succeeded in 10055ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

2026-05-03T06:48:26.638242Z ERROR codex_core::tools::router: error=write_stdin failed: stdin is closed for this session; rerun exec_command with tty=true to keep stdin open
exec
/bin/bash -lc 'for f in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md /home/zephryj/projects/turingosv4/CLAUDE.md; do if [ -f "$f" ]; then printf '"'%s\\n' \""'$f"; fi; done' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4/CLAUDE.md

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' /home/zephryj/projects/turingosv4" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "rg --line-number --hidden --glob '"'!target/**'"' --glob '"'!handover/evidence/**/runtime_repo/**'"' --glob '"'!handover/evidence/**/cas/**'"' 'STEP_B|restricted|Restricted|src/state/sequencer.rs|Code Standard' CLAUDE.md handover/ai-direct/STEP_B_PROTOCOL.md handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md 2>/dev/null" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/ai-direct/STEP_B_PROTOCOL.md:1:# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment
handover/ai-direct/STEP_B_PROTOCOL.md:3:**Scope**: any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `src/state/sequencer.rs`). Also applicable to any proposal that touches "institution" per C-031.
handover/ai-direct/STEP_B_PROTOCOL.md:13:- Is a less-invasive alternative available? (same effect, no restricted-file touch)
handover/ai-direct/STEP_B_PROTOCOL.md:107:1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
CLAUDE.md:12:## Code Standard (Art. I.1 + C-004 + C-027)
CLAUDE.md:14:- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:1:# OBS: STEP_B restricted-file list path drift (2026-04-29)
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:8:CLAUDE.md "Code Standard" line 14 + STEP_B_PROTOCOL.md line 3 both listed the STEP_B-restricted set as `src/{kernel,bus,wallet}.rs`, but `src/wallet.rs` does not exist at HEAD `2f5093a`. Wallet was relocated to `src/sdk/tools/wallet.rs` at some prior commit (not bisected; not on critical path).
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:10:## Verification that wallet is still institutional (deserves STEP_B)
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:17:This file IS the institutional wallet that the original CLAUDE.md "Code Standard" line was designed to protect. The drift is purely a path-name change, not a constitutional-role change.
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:22:- `handover/ai-direct/STEP_B_PROTOCOL.md` line 3: `kernel.rs, bus.rs, wallet.rs` → `src/kernel.rs, src/bus.rs, src/sdk/tools/wallet.rs`
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:24:Both edits are path-corrections only; STEP_B-restricted set is semantically unchanged (same 3 files, just one with corrected path).
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:28:CO1.7.5 spec § 7 smoke S5 surfaced this. Sending a spec to dual external audit while the referenced restricted-file list contradicts repo state is a guaranteed CHALLENGE — a wasted round. Per memory `feedback_smoke_before_batch`, smoke discrepancies block audit launch. Fixing the docs at smoke time is the cheapest path; deferring would leak the inconsistency into the audit prompt.
handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md:34:- Memory updates. `feedback_step_b_protocol` memory is path-agnostic; references "restricted files" semantically, no edit needed.

exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4
     2	
     3	## What
     4	Silicon-Native Microkernel for LLM Formal Verification Swarm.
     5	Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.
     6	
     7	## Why
     8	- 唯一对齐文档: `constitution.md` (反奥利奥架构)
     9	- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
    10	- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)
    11	
    12	## Code Standard (Art. I.1 + C-004 + C-027)
    13	- `cargo check` / `cargo test` 必过；`.env` 永不 commit
    14	- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
    15	- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码
    16	
    17	## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
    18	- Generator ≠ Evaluator：代码作者不可是唯一审计者
    19	- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
    20	- 宪法违规立即 BLOCKER，不可延期、不可"可接受"
    21	
    22	## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
    23	- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
    24	- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
    25	- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
    26	- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
    27	- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪
    28	
    29	## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
    30	- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
    31	- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
    32	- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）
    33	
    34	## Alignment Standard (Art. IV + C-069)
    35	- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
    36	  (后续 rev: `TRACE_MATRIX_vN.md`)
    37	- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
    38	  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
    39	- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
    40	  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
    41	- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
    42	- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法
    43	
    44	## Common Law (宪法 + 判例)
    45	宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
    46	- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
    47	- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
    48	- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）
    49	
    50	## Docs (按需加载)
    51	| 文档 | 何时加载 |
    52	|------|---------|
    53	| `docs/architecture.md` | 修改 src/ 核心模块时 |
    54	| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
    55	| `docs/hardware.md` | SSH/部署/远程操作时 |
    56	| `docs/experiments.md` | 创建或运行实验时 |
    57	| `docs/rules.md` | 触发规则或修改规则时 |
    58	
    59	## User
    60	独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc "nl -ba handover/ai-direct/STEP_B_PROTOCOL.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Step-B Protocol: Restricted-File Changes via Parallel Branch Experiment
     2	
     3	**Scope**: any change to files in CLAUDE.md's restricted list (currently `src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`, `src/state/sequencer.rs`). Also applicable to any proposal that touches "institution" per C-031.
     4	
     5	**Principle**: real data decides, not opinion. Parallel branch lets us test change empirically before merging.
     6	
     7	---
     8	
     9	## Phase 0 — Necessity audit (external, before any code)
    10	
    11	External auditors (**Codex + Gemini**) answer:
    12	- Is the change *necessary*? What observable behavior is broken now?
    13	- Is a less-invasive alternative available? (same effect, no restricted-file touch)
    14	- What's the *minimum sufficient* version? (avoid scope creep)
    15	- What's the failure mode if we don't change?
    16	
    17	**Gate**: if both auditors say "less-invasive alternative exists", take that path instead. If both say "change is necessary", proceed to Phase 1. If they disagree, take the conservative verdict per `feedback_dual_audit_conflict`.
    18	
    19	**My role**: write the audit brief; do not pre-advocate. If I agree with the verdict after reading both, proceed. If I disagree, record dissent in notepad §7 before proceeding.
    20	
    21	---
    22	
    23	## Phase 1 — Parallel branch creation
    24	
    25	### 1a. Worktree spawn
    26	```bash
    27	git worktree add .claude/worktrees/stepb-<slug> -b experiment/<slug>
    28	```
    29	
    30	Or via Agent tool with `isolation: worktree` frontmatter — for short experiments (<2h). Agent returns branch name + diff summary.
    31	
    32	### 1b. Implementation in isolation
    33	- Work in the experiment branch only.
    34	- Main branch stays at the last audited-PASS state.
    35	- Add fixture tests covering the change.
    36	- Run `cargo test` on experiment branch → must be green.
    37	- Commit with message `experiment/<slug>: <change>` (not merged).
    38	
    39	### 1c. Implementation audit
    40	External auditors (same two) review the **diff only**:
    41	- Is the change minimal?
    42	- Are tests sufficient?
    43	- Does it introduce new constitutional debt?
    44	- Any risk the diff itself is a Trojan (side-effects beyond scope)?
    45	
    46	**Gate**: both PASS on diff → proceed to Phase 2. Any VETO → abandon branch or revise.
    47	
    48	---
    49	
    50	## Phase 2 — Statistical A/B test (pre-registered)
    51	
    52	### 2a. Pre-register experimental design
    53	Before any batch run, write a locked spec to `handover/ai-direct/AB_<slug>_<date>.md`:
    54	
    55	- **Null hypothesis (H0)**: p(treatment solves) = p(control solves) on frozen sample
    56	- **Primary metric**: SolveRate delta (per `metrics.yaml`)
    57	- **Secondary metrics**: Aggregate_PPUT, mean wall time
    58	- **Sample**: identical to M4 (seed=74677, N=50, fingerprint=796ead6c40351ae9)
    59	- **Sample size justification**: binomial approximation — a 3-solve difference (6 pp on N=50) is ~2σ above null; 6-solve difference (12 pp) is ~4σ. Pre-register **strict-win threshold = ΔSolveRate ≥ 3** (consistent with v3.1 decision rule).
    60	- **Statistical test**: McNemar's test for paired nominal data. Report p-value but do not use as primary (SolveRate delta is).
    61	- **Decision rule**:
    62	  - `ΔSolveRate ≥ 3`: treatment strict win → merge candidate
    63	  - `-1 ≤ Δ ≤ 2`: inconclusive → either expand N (same seed family) or abandon
    64	  - `ΔSolveRate ≤ -1`: treatment regresses → abandon branch
    65	- **Interleaving**: run both conditions on each problem alternately (or in parallel if possible) to neutralize API drift (C-033).
    66	- **Abort gate**: per-condition 20% / 30% (same as v3.1).
    67	
    68	### 2b. Execute on both branches
    69	- **Control branch** (main or last-PASS HEAD): run `run_interleaved.sh` with `TREATMENT_LABEL=control`
    70	- **Treatment branch** (experiment/<slug>): run same script with `TREATMENT_LABEL=treatment`
    71	- **Cost**: 2× the single-arm experiment. Pre-registered budget before spending.
    72	
    73	### 2c. Freeze analyzer
    74	`frozen_analysis.py` for A/B extension:
    75	- Must support `--control <jsonl>` and `--treatment <jsonl>` flags
    76	- Outputs: paired-comparison table, McNemar p, ΔSolveRate, discordant pairs list
    77	- Must be fixture-tested before A/B run (C-012 mandatory freeze).
    78	
    79	---
    80	
    81	## Phase 3 — Verdict + commit path
    82	
    83	### 3a. Read the data
    84	After both branches finish, run `frozen_analysis.py --control ... --treatment ...`.
    85	
    86	### 3b. Audit the verdict (again)
    87	External auditors see **data only** (no researcher interpretation). They apply the pre-registered decision rule and return PASS / FAIL / INCONCLUSIVE.
    88	
    89	### 3c. Merge or abandon
    90	- **Treatment win (audits PASS)**: 
    91	  - `git merge experiment/<slug> --no-ff` on main
    92	  - Update notepad §2 with new F-id
    93	  - If new constitutional pattern emerged → write case candidate
    94	- **Treatment lose or inconclusive**:
    95	  - `git branch -D experiment/<slug>` (or archive as tag `archive/<slug>_<date>`)
    96	  - Update notepad §3 (retracted hypotheses) with finding
    97	  - No commit to main; keep last-PASS HEAD
    98	
    99	### 3d. Cleanup
   100	- Remove worktree: `git worktree remove .claude/worktrees/stepb-<slug>`
   101	- If branch archived: `git tag archive/<slug>_<date> experiment/<slug>` then delete branch
   102	
   103	---
   104	
   105	## What this protocol guarantees
   106	
   107	1. **C-010**: external audit at 3 junctures (necessity, diff, verdict). Researcher cannot self-approve a restricted change.
   108	2. **C-012**: pre-registered metrics, frozen sample, fixture-tested analyzer — no post-hoc metric shopping.
   109	3. **C-033**: paired-comparison on same problems cleanly attributes any change to the code diff, not to sample drift.
   110	4. **C-034**: the change is a mechanism edit validated by mechanism-level data, not prompt tuning.
   111	5. **Art. V.2**: pre-registered budget. No scope creep.
   112	
   113	## What this protocol does NOT do
   114	
   115	- **Statistical power** is limited by frozen N=50. Small effects (Δ = 1-2 solves) stay inconclusive. Pre-register a larger N (100? 244?) if effect is expected small.
   116	- **Non-stationary effects** (model drift, API latency changes) across days are not controlled unless both branches run interleaved or within same hour.
   117	- **Multi-batch meta-analysis** is not automated; manual aggregation if we run the same A/B protocol multiple times.
   118	
   119	---
   120	
   121	## Integration with existing harness
   122	
   123	| Existing artifact | How it connects |
   124	|---|---|
   125	| `AUTO_RESEARCH_NOTEPAD.md` §5 | Lists pending Step-B changes |
   126	| `run_interleaved.sh` | Accepts `TREATMENT_LABEL` env for branch attribution |
   127	| `frozen_analysis.py` | Extend with `--control`/`--treatment` for A/B |
   128	| Routine A (daily drift) | Will detect if experiment/<slug> branch has uncommitted drift |
   129	| Routine B (disabled) | N/A |
   130	| Memory `project_auto_research_notepad` | Reminds to check if a Step-B change is due parallel-test not direct edit |
   131	
   132	---
   133	
   134	## First application candidate
   135	
   136	**`src/bus.rs recent_rejections` Art. II.1 fix** (notepad §5, F-2026-04-15-02):
   137	
   138	- Phase 0: necessity audit — agents converge on same hallucination; recent_errors broadcast currently per-author-only. Is the fix needed, or is v3.2 chat-model test a cheaper alternative?
   139	- Phase 1: branch `experiment/art-ii1-global-graveyard`
   140	- Phase 2: A/B on seed=74677 N=50 (same as v3.1)
   141	- Phase 3: merge only if ΔSolveRate(n3) ≥ 3
   142	
   143	Not to execute now — blocked on v3.1 completion + v3.2 comparison per notepad sequence.

exec
/bin/bash -lc "nl -ba handover/tracer_bullets/TB-13_charter_2026-05-03.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-13 Charter — CompleteSet + MarketSeedTx (Conditional-Share Substrate)
     2	
     3	**Status**: **DRAFT 2026-05-03 evening** (post-TB-12 SHIPPED `fa36eca`). Awaiting user-architect ratification before Atom 0.5 begins. Architect `proceed and recursively check at the best place you decide` reads as ratification + checkpoint authority delegated to AI coder.
     4	**Date**: 2026-05-03 evening (drafted same day as TB-12 SHIPPED + 2026-05-03 architect post-TB-12 ruling).
     5	**Predecessor**: TB-12 SHIPPED 2026-05-03 (`fa36eca`).
     6	**Authority**: 2026-05-03 architect post-TB-12 ruling lossless archive at `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` (operative spec, §4 TB-13 + §11 loop-mode master instruction) + 2026-05-02 supplementary directive at `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md` (TB-13 base spec) + 2026-05-02 lossless integrated edition Part C §9.4 (TB-13 forbidden list canonical).
     7	**Phase**: **P3 carry-forward** (RSP-M Polymarket collateral substrate per 2026-05-02 directive Part C §9.4) + **P4 primary** (CompleteSet is the conditional-share substrate that TB-15 MarkovEvidenceCapsule will compress into Information Loom rollups; first canonical YES_E + NO_E claim accounting).
     8	**Risk class envelope**: **Class 3** (per architect §4.8). Per-atom: Atom 0.5 = Class 1 (forward-fence + label, no production deletion). Atom 1 = Class 2 (additive schema). Atom 2 = **Class 3** (sequencer money path — debits `balances_t`, credits new `conditional_collateral_t`, issues `conditional_share_balances_t`). Atom 3 = Class 3 (extends `monetary_invariant`; touches CTF). Atom 4 = Class 1 (dashboard read-only). Atom 5 = Class 1 (tests). Ship audit (Atom 6) = **Class 3 dual** (Codex + Gemini, no honest-deferral; conservative-verdict-wins).
     9	**Iteration cap**: **72h with 24h checkpoints** (production wire-up exception per `feedback_iteration_cap_24h`; conservation surface).
    10	
    11	**Operating mode** (mirrors TB-12 Q6 ii.5 sync mode + architect §4.8 + user 2026-05-03 evening "proceed and recursively check at the best place you decide"):
    12	- Atoms 0.5 → 5 continuous; AI-coder-decided micro-checkpoints at:
    13	  - **CP-A** after Atom 0.5 (cargo test --workspace baseline preserved; forward fence in place).
    14	  - **CP-B** after Atom 2 (sequencer dispatch; STEP_B-protocol; conservation pre-check via assert_total_ctf_conserved).
    15	  - **CP-C** after Atom 3 (monetary_invariant extended; this is the THE money-safety gate).
    16	- Atom 6 dual audit runs continuously; **STOP after audit verdict**.
    17	- Atom 7 SHIP ONLY after explicit user authorization on the verdict.
    18	
    19	**FC-trace**: `Art. 0.2` (Tape Canonical — `CompleteSetMintTx` / `CompleteSetRedeemTx` / `MarketSeedTx` canonical-encoded; replay-deterministic) + `Art. I.1` (5-step compile loop — conditional collateral and share accounting are part of the proposal-pricing substrate, but DO NOT affect predicate outcome; CR-13.6 explicit) + `Art. III.4` (no fake accepted preserved — Redeem requires system-resolved outcome reference; agent-callable Redeem rejected pre-resolution) + `Art. V.1.3` (Anti-Oreo — `MarketSeedTx` requires explicit provider funds, NO automatic seed; `CompleteSetMintTx` is agent-driven balance debit, no money created) + `WP-§14.1` (typed_tx variant additive bumps; 3 NEW typed-tx variants; CompleteSetRedeem additionally accepts a `ResolutionRef` from system-emitted resolution; not a unilateral agent variant since pre-resolution rejection is sequencer-side gate) + `WP-§19` (RSP-M collateral substrate — conditional collateral is the YES_E + NO_E claim foundation that TB-14 PriceIndex will derive prices from).
    20	
    21	**Flowchart-trace**: **Flowchart 1** (runtime — `CompleteSetMintTx` debits `balances_t`, credits `conditional_collateral_t`, increments equal `conditional_share_balances_t[(owner, event, Yes/No)]`; `CompleteSetRedeemTx` requires system-resolution-reference, debits `conditional_share_balances_t`, credits `balances_t`; `MarketSeedTx` debits `balances_t` of provider, credits `conditional_collateral_t` + records protocol-owned share inventory) + **Flowchart 2** (boot — runtime preseed unchanged; EconomicState 11→13 sub-fields with backward-compat `#[serde(default)]` empty maps) + **Flowchart 3** (meta — conditional shares are TB-14 PriceIndex price-derivation substrate + TB-15 MarkovEvidenceCapsule compression input; TB-13 lays bytes only; no price, no AMM, no orderbook).
    22	
    23	**Phase declarations** (per `feedback_tb_phase_tag_required`):
    24	
    25	```text
    26	phase_id: P3 carry-forward (RSP-M collateral substrate per 2026-05-02 §9.4)
    27	          + P4 primary (CompleteSet conditional-share substrate; first
    28	                        canonical YES_E + NO_E claim accounting; TB-15
    29	                        Information Loom compression input)
    30	roadmap_exit_criteria_addressed:
    31	  P3:RSP-M  1 locked Coin = 1 YES_E + 1 NO_E mathematical core (architect
    32	            §4.4 FR-13.1 / FR-13.2 / FR-13.3); CTF conserved across mint
    33	            (architect SG-13.1)
    34	  P3:RSP-M  Redeem requires system-resolved outcome (architect FR-13.4
    35	            + SG-13.5); after-YES outcome pays YES not NO (FR-13.5 + SG-13.6)
    36	  P3:RSP-M  MarketSeedTx requires explicit provider funds (FR-13.6 +
    37	            SG-13.3); no auto-seed (forbidden list Part A §4.7); no
    38	            quote/trade/price (FR-13.7)
    39	  P4:1      First canonical YES_E + NO_E claim accounting (replaces
    40	            BinaryMarket f64 CPMM book; EconomicState +2 sub-fields)
    41	  P4:2      Forward-binding fence: TB-13 modules cannot import legacy
    42	            prediction_market.rs (forbidden-token grep ship-gate)
    43	  P4:3      Conditional collateral as Coin holding; conditional shares
    44	            as claims (architect CR-13.3 + CR-13.4); 6-holding CTF sum
    45	            (extends 5→6 holdings; conditional_collateral_t IS a holding)
    46	kill_criteria_tested:
    47	  P3:1      No post-init mint — CompleteSetMintTx is balance-DEBIT-then-
    48	            collateral-CREDIT pair; total Coin conserved bit-for-bit
    49	            (architect SG-13.1; assert_total_ctf_conserved unchanged-output
    50	            after extending exempt list with conditional_collateral_t)
    51	  P3:2      No stake-less write — CompleteSetMintTx with amount==0
    52	            either rejected upstream or NO conditional state mutation;
    53	            MarketSeedTx with collateral==0 rejected (SG-13.4 cannot
    54	            create liquidity without collateral)
    55	  P3:3      Payout sum ≤ collateral — CompleteSetRedeemTx after YES
    56	            redeems at most conditional_share_balances_t[(owner, event,
    57	            Yes)] units against conditional_collateral_t[event]; CTF
    58	            preserved across redeem (no payout > collateral)
    59	  P3:4      No agent-self-reported reward — Redeem requires
    60	            ResolutionRef pointing to a system-emitted resolution (no
    61	            agent unilateral close pre-resolution); SG-13.5 enforced
    62	  P3:5      No full payout pre-challenge-window — Redeem rejected if
    63	            ResolutionRef.outcome state != Resolved or if challenge
    64	            window for the resolving event is still open
    65	  P3:8      Settlement ≤ escrow — TB-8 invariant unchanged; TB-13 does
    66	            NOT touch escrows_t / claims_t finalize path; CompleteSet
    67	            collateral is its own pool
    68	  P4:4      Conditional shares NOT in total_supply_micro — CR-13.3
    69	            verified via SG-13.2 (assert_total_ctf_conserved with
    70	            conditional_share_balances_t in EXEMPT list, not in sum)
    71	flowchart_trace:
    72	  Flowchart 1 (runtime): CompleteSetMintTx → balances_t debit +
    73	                         conditional_collateral_t credit + conditional_
    74	                         share_balances_t equal-YES-NO mint;
    75	                         CompleteSetRedeemTx → conditional_share_balances_t
    76	                         debit + balances_t credit (only after system-
    77	                         resolved outcome);
    78	                         MarketSeedTx → provider balances_t debit +
    79	                         conditional_collateral_t credit + protocol-
    80	                         owned share inventory record (NO trading).
    81	  Flowchart 2 (boot):    EconomicState additive +conditional_collateral_t
    82	                         +conditional_share_balances_t (11→13).
    83	  Flowchart 3 (meta):    Conditional shares are TB-14 PriceIndex price-
    84	                         derivation substrate + TB-15 Information Loom
    85	                         compression input; TB-13 lays bytes only.
    86	```
    87	
    88	---
    89	
    90	## §0 Why TB-13 exists (architect post-TB-12 ruling §4.1)
    91	
    92	**Architect spec verbatim** (lossless archive Part A §4.1):
    93	
    94	```text
    95	TB-13 = CompleteSet + MarketSeedTx
    96	引入 Polymarket / CTF 的数学核心：
    97	  1 locked Coin = 1 YES_E + 1 NO_E
    98	
    99	但 TB-13 仍然不做：
   100	  AMM / CPMM router / orderbook / MarketOrderTx / MarketTradeTx /
   101	  PriceIndex / DPMM / pro-rata / automatic liquidity
   102	
   103	TB-13 只做抵押与份额会计。
   104	```
   105	
   106	**Driver context** (architect Part A §1.3 + §4.1):
   107	- TB-12 SHIPPED Node Exposure Index (`fa36eca`); WorkTx.stake → FirstLong, ChallengeTx.stake → ChallengeShort EXPOSURE RECORDS now exist.
   108	- NodePosition is exposure record, NOT YES/NO share, NOT tradable balance, NOT collateral.
   109	- TB-13 introduces the FIRST canonical YES/NO claim accounting; closes the gap between "exposure recorded" (TB-12) and "directional claim against collateral" (TB-13).
   110	- Constitutional invariants preserved: 1 Coin = 1 YES + 1 NO mathematical identity; no automatic injection; on_init unique mint.
   111	- Forward fence: TB-13 modules MUST NOT import legacy `src/prediction_market.rs` f64 CPMM scaffolding (architect §4.2 SG-13.0.1 + halting trigger 1).
   112	
   113	---
   114	
   115	## §1 One-line goal
   116	
   117	```text
   118	Goal: Introduce conditional-share accounting per Polymarket / CTF math
   119	      "1 locked Coin = 1 YES_E + 1 NO_E" via 3 NEW typed_tx variants
   120	      (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) and 2 NEW
   121	      EconomicState sub-fields (conditional_collateral_t,
   122	      conditional_share_balances_t). CompleteSetMint debits Coin balance,
   123	      credits conditional collateral, mints equal YES_E + NO_E shares.
   124	      CompleteSetRedeem requires system-resolved outcome; after YES
   125	      outcome pays YES, after NO outcome pays NO; pre-resolution
   126	      rejected. MarketSeedTx uses explicit provider funds; no auto-seed;
   127	      no quote/trade/price. CTF conserved (1 Coin → 1 YES + 1 NO);
   128	      shares NOT in total Coin supply. NO AMM. NO CPMM. NO orderbook.
   129	      NO PriceIndex (TB-14). NO trading.
   130	
   131	Pre-gate (Atom 0.5): forward fence + label. NEW TB-13 modules cannot
   132	      import src/prediction_market.rs; forbidden-token grep ship-gate
   133	      enforces this. Existing kernel.rs / bus.rs / evaluator.rs CPMM
   134	      wiring is documented as legacy and carry-forward to TB-14 SHIP
   135	      prerequisite (per OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03;
   136	      retroactive deletion would break 759 passing tests + the
   137	      production evaluator and is OUT OF SCOPE per
   138	      feedback_no_retroactive_evidence_rewrite).
   139	```
   140	
   141	---
   142	
   143	## §2 What's already shipped (TB-3..TB-12 foundations + TB-13 dependencies)
   144	
   145	| Foundation | Source | Status |
   146	|---|---|---|
   147	| `MicroCoin` integer math (no f64 in money path) | `src/economy/money.rs` | shipped TB-3 |
   148	| `BalancesIndex`, `EscrowsIndex`, `StakesIndex`, `ClaimsIndex` | `src/state/q_state.rs:158-162` | shipped TB-3..TB-7R |
   149	| `EconomicState` 11 sub-fields (incl. node_positions_t added by TB-12) | `src/state/q_state.rs:158-192` | shipped TB-3..TB-12 |
   150	| `assert_total_ctf_conserved` 5-holding sum (now 5; conditional_collateral_t will become 6th holding) | `src/economy/monetary_invariant.rs` | shipped TB-7R |
   151	| `assert_no_post_init_mint` exhaustive match (incl. TaskBankruptcy + TB-12 typed-tx unchanged) | `src/economy/monetary_invariant.rs` | shipped TB-3+ |
   152	| `TypedTx` enum + canonical signing payload + state-root domain prefix | `src/state/typed_tx.rs` | shipped TB-3..TB-12 |
   153	| Sequencer dispatch arm pattern (additive variants safe; agent-ingress fail-closed for system-emitted variants) | `src/state/sequencer.rs` | shipped TB-11 + TB-12 |
   154	| `audit_dashboard.rs` §1-§13 (incl. §13 TB-12 Node exposure records) | `src/bin/audit_dashboard.rs` | shipped TB-12 |
   155	| OBS_TB_12_LEGACY_CPMM_QUARANTINE (forward-fence rationale + TB-14 SHIP prerequisite) | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` | open OBS |
   156	
   157	**Net-new in TB-13** (per architect §4.2 + §4.3 + §4.4 + §4.5 + §4.6 + §4.7):
   158	
   159	- **Atom 0.5** (legacy CPMM forward-fence + label):
   160	  - Forbidden-token grep ship-gate test (`tests/tb_13_legacy_cpmm_forward_fence.rs`) failing if NEW TB-13 module references `prediction_market::` or `BinaryMarket` or `buy_yes` or `buy_no` or `f64` in a money-path context.
   161	  - Doc-comment "LEGACY: not constitutional, not RSP-M, not production market path; carry-forward to TB-14 SHIP prerequisite per OBS_TB_12_LEGACY_CPMM_QUARANTINE" added to `src/prediction_market.rs` module header + `src/kernel.rs` market field doc-comments.
   162	  - OBS_TB_12_LEGACY_CPMM_QUARANTINE updated to reflect TB-13 forward-fence in place + still carrying TB-14 SHIP prerequisite.
   163	  - **NO retroactive deletion** of `src/prediction_market.rs` or `src/kernel.rs` market scaffolding (production evaluator + 10+ test files would break; out-of-scope per `feedback_no_retroactive_evidence_rewrite`; TB-14 SHIP prerequisite per OBS).
   164	
   165	- **Atom 1** (typed_tx schemas):
   166	  - `EventId` newtype (`pub struct EventId(pub TaskId)` — for TB-13 we resolve events 1:1 with task_id; future TB-14+ may decouple).
   167	  - `OutcomeSide` enum `{Yes = 0, No = 1}` with `#[repr(u8)]`.
   168	  - `ShareAmount` newtype `pub struct ShareAmount { pub units: u128 }` (u128 not i128 — shares are non-negative claims; mint creates positive, redeem decreases positive; never negative).
   169	  - `ResolutionRef` newtype referencing the system-emitted resolution (TB-13 reuses `TaskBankruptcyTx.tx_id` for NO outcome + reuses `FinalizeRewardTx.tx_id` for YES outcome — both already system-emitted; this is the "system resolution reference" architect §4.3 mentions).
   170	  - `CompleteSetMintTx { tx_id, parent_state_root, event_id, owner, amount: MicroCoin, signature }` agent-callable.
   171	  - `CompleteSetRedeemTx { tx_id, parent_state_root, event_id, owner, outcome: OutcomeSide, share_amount: ShareAmount, resolution_ref: ResolutionRef, signature }` agent-callable but pre-resolution rejected at sequencer.
   172	  - `MarketSeedTx { tx_id, parent_state_root, event_id, provider, collateral_amount: MicroCoin, signature }` agent-callable; provider == seed_owner.
   173	  - All 3 added to `TypedTx` enum + `CanonicalMessage::*Signing` + 3 new `DOMAIN_*_V1` prefixes + `TxKind::*` discriminants.
   174	  - Round-trip + deterministic-digest unit tests; STEP_B_PROTOCOL parallel-branch since touches `src/state/typed_tx.rs` restricted file.
   175	
   176	- **Atom 2** (sequencer dispatch + EconomicState extension):
   177	  - `EconomicState` 11 → 13 sub-fields with `+conditional_collateral_t: ConditionalCollateralIndex` + `+conditional_share_balances_t: ConditionalShareBalances`. Both `#[serde(default)]` for backward-compat with TB-12 chain snapshots.
   178	  - `ConditionalCollateralIndex(BTreeMap<EventId, MicroCoin>)` — locked Coin per event.
   179	  - `ConditionalShareBalances(BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>)` — share holdings per (owner, event, side).
   180	  - 3 new dispatch arms in `src/state/sequencer.rs`:
   181	    - `CompleteSetMintTx`: validate balance >= amount; debit `balances_t[owner]` by amount; credit `conditional_collateral_t[event_id]` by amount; credit equal share_amount=amount.units to `conditional_share_balances_t[(owner, event, Yes)]` AND `conditional_share_balances_t[(owner, event, No)]`. (Architect FR-13.1 + FR-13.2 + FR-13.3.)
   182	    - `CompleteSetRedeemTx`: validate `resolution_ref` resolves to either a `TaskBankruptcyTx` (NO outcome) or a `FinalizeRewardTx` (YES outcome) for `event_id`; verify outcome matches Tx outcome field; debit `conditional_share_balances_t[(owner, event, outcome)]` by share_amount; credit `balances_t[owner]` by share_amount.units (1:1 per architect FR-13.5: "after YES outcome pays YES shares"); debit `conditional_collateral_t[event_id]` by share_amount.units. Pre-resolution rejected (architect FR-13.4 + SG-13.5).
   183	    - `MarketSeedTx`: validate `balances_t[provider] >= collateral_amount`; debit `balances_t[provider]`; credit `conditional_collateral_t[event_id]` by collateral_amount; credit `conditional_share_balances_t[(provider, event, Yes)]` AND `(provider, event, No)` each by collateral_amount.units (provider owns BOTH sides; explicit provider-funded; NO trading; architect FR-13.6 + FR-13.7).
   184	  - State-root domain prefixes: `COMPLETE_SET_MINT_DOMAIN_V1`, `COMPLETE_SET_REDEEM_DOMAIN_V1`, `MARKET_SEED_DOMAIN_V1`.
   185	  - 3 new `TransitionError` variants (additive): `InsufficientBalanceForMint`, `RedeemBeforeResolution`, `InsufficientCollateral`.
   186	  - STEP_B_PROTOCOL parallel-branch since touches `src/state/sequencer.rs` restricted file.
   187	
   188	- **Atom 3** (monetary_invariant extension):
   189	  - Extend `assert_total_ctf_conserved` to add `conditional_collateral_t` as the 6th Coin holding (architect CR-13.4: "Locked collateral is Coin holding"); EXCLUDE `conditional_share_balances_t` from the sum (architect CR-13.3 + SG-13.2: "shares are not Coin").
   190	  - New invariant `assert_complete_set_balanced`: for every event_id, `Σ conditional_share_balances_t[(*, event, Yes)] == Σ conditional_share_balances_t[(*, event, No)] == conditional_collateral_t[event].units` (1 Coin = 1 YES + 1 NO mathematical identity post-mint and post-redeem).
   191	  - New invariant `assert_no_negative_share_balance`: u128 type-level guarantee + sequencer-side underflow check.
   192	
   193	- **Atom 4** (dashboard §14):
   194	  - `audit_dashboard.rs` §14 — "Conditional collateral and share balances" sub-tables; label discipline: "claims, not balances" (architect CR-13.3); "locked collateral, not free coin" (architect CR-13.4).
   195	  - Per-event totals: collateral, YES depth, NO depth, completeness check (depth_yes == depth_no == collateral.units).
   196	  - Per-owner share holdings filtered by event.
   197	  - Render-fn unit-testable per TB-12 §13 precedent.
   198	
   199	- **Atom 5** (integration tests SG-13.1..8 + halting-trigger guards):
   200	  - `tests/tb_13_complete_set.rs` covering all SG-13.x with EXACT architect-named functions + halting-trigger guard tests.
   201	
   202	- **Atom 6** (audit):
   203	  - `RECURSIVE_AUDIT_TB_13_2026-05-XX.md` (4-clause + complete-set hygiene clause).
   204	  - `CODEX_TB_13_SHIP_AUDIT_2026-05-XX.md`.
   205	  - `GEMINI_TB_13_SHIP_AUDIT_2026-05-XX.md`.
   206	
   207	- **Atom 7** (SHIP):
   208	  - LATEST.md + TB_LOG.tsv row + ship commit. ONLY after audit verdict.
   209	
   210	---
   211	
   212	## §3 Deliverables (8 atoms)
   213	
   214	### Atom 0 — Charter ratification (Class 0)
   215	
   216	**Action**: this document IS the ratification draft. Architect 2026-05-03 evening "proceed and recursively check at the best place you decide" reads as ratification + checkpoint authority delegation; user-architect may override before Atom 0.5.
   217	
   218	**Iter cap**: 0.5h.
   219	
   220	---
   221	
   222	### Atom 0.5 — Legacy CPMM forward-fence + label (Class 1)
   223	
   224	**Architect mandate** (Part A §4.2 + §4.7 forbidden list + §4.8 halting triggers):
   225	
   226	```text
   227	FR-13.0.1  src/prediction_market.rs legacy f64 CPMM must be quarantined.
   228	FR-13.0.2  New CompleteSet/MarketSeedTx code must not import legacy
   229	           prediction_market.rs.
   230	FR-13.0.3  No f64 in CompleteSet/MarketSeedTx/market accounting path.
   231	FR-13.0.4  Legacy CPMM must be clearly labeled:
   232	           legacy / not used by RSP-M / not constitutional /
   233	           not production market path.
   234	```
   235	
   236	**Action**:
   237	
   238	(a) Forward-fence test `tests/tb_13_legacy_cpmm_forward_fence.rs`:
   239	   - Read all NEW TB-13 module sources (`src/state/typed_tx.rs` *additions* — identified by git-blame against TB-12 ship `fa36eca`; `src/economy/conditional_market.rs` if created; `src/state/sequencer.rs` *additions*).
   240	   - Forbidden-token grep on those additions: fails if any of `prediction_market::`, `BinaryMarket`, `buy_yes`, `buy_no`, ` f64 ` (in money-path context — exclude unrelated f64 use like timestamps if any), `MarketError` (legacy variant), `open_bounty_market`, `bounty_market`, `bounty_lp_seed` appear.
   241	   - Implementation: read git-tree `fa36eca:src/state/sequencer.rs` vs HEAD; diff additions; grep additions only.
   242	
   243	(b) Label discipline:
   244	   - Update `src/prediction_market.rs` module header doc-comment with:
   245	     ```rust
   246	     //! # LEGACY — TB-3..TB-10 Phase-3A Hayek bounty-market scaffolding
   247	     //!
   248	     //! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2):
   249	     //! This module is **legacy / not constitutional / not RSP-M / not
   250	     //! production market path** in the post-2026-05 architect roadmap. It is:
   251	     //! - **NOT** the canonical YES/NO claim system (TB-13 introduces
   252	     //!   `CompleteSetMintTx` + `ConditionalShareBalances` for that).
   253	     //! - **NOT** the canonical price index (TB-14 introduces `PriceIndex`
   254	     //!   derived from `node_positions_t` long/short interest).
   255	     //! - **NOT** authorized for extension or new use sites.
   256	     //!
   257	     //! ## Carry-forward
   258	     //! Removal is a TB-14 SHIP prerequisite per
   259	     //! `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`.
   260	     //! Retroactive deletion in TB-13 would break the production evaluator

exec
/bin/bash -lc "nl -ba handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# 2026-05-03 — TB-13 → TB-17 Post-TB-12 Architect Ruling (Lossless Archive)
     2	
     3	**Source**: User-architect ultrathink directive delivered 2026-05-03 after TB-12 ship
     4	(commit `fa36eca`).
     5	
     6	**Status**: Lossless verbatim archive per `feedback_kolmogorov_compression`. The
     7	original architect text is reproduced below in two parts:
     8	- **Part A** — Audit + roadmap directive (markdown body of the directive).
     9	- **Part B** — Companion roadmap file `TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md`
    10	  pasted at the tail of the directive.
    11	
    12	Annotation layer (delta vs prior 2026-05-02 supplementary directive) is in §C
    13	at the bottom; do NOT read annotation as substitute for the verbatim text.
    14	
    15	---
    16	
    17	## Part A — Architect ultrathink directive (verbatim)
    18	
    19	According to a document from 2026-05-03, TB-12 的 shipped summary 显示它已按 NodeMarket Position Index 的窄化目标完成：`WorkTx.stake -> FirstLongPosition`、`ChallengeTx.stake -> ShortPosition`、`VerifyTx.bond != market position`、`NodePosition not Coin holding`，并且用户提供的结果显示 `cargo test --workspace = 759 / 0 / 150`、`8/8 SG-12.x`、`11/11 G-gates`、`14/14 forbidden tokens`、`9/9 immutable-record invariants` 均通过。这个阶段可以接受，但我没有在本轮直接读取到 GitHub commit `fa36eca` 的源码 diff；下面的审计以你提供的 TB-12 summary、已检索到的路线文档和前序宪法/白皮书约束为依据。
    20	
    21	我也写了一份本地路线图文件：
    22	
    23	[下载：TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md](sandbox:/mnt/data/TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md)
    24	
    25	---
    26	
    27	# 1. TB-12 阶段性审计结论
    28	
    29	## 1.1 接受 TB-12 shipped
    30	
    31	我接受 TB-12 的 shipped 状态，但只接受它的**窄化声明**：
    32	
    33	```text
    34	TB-12 proved:
    35	  NodePosition exposure index exists.
    36	  WorkTx.stake becomes FirstLongPosition.
    37	  ChallengeTx.stake becomes ChallengeShortPosition.
    38	  VerifyTx.bond is not a market position.
    39	  NodePosition is not Coin holding.
    40	```
    41	
    42	TB-12 **没有证明**：
    43	
    44	```text
    45	CompleteSet
    46	YES/NO share accounting
    47	MarketSeedTx
    48	PriceIndex
    49	Boltzmann masking
    50	market trading
    51	AMM
    52	CPMM
    53	Polymarket settlement
    54	real-world readiness
    55	```
    56	
    57	这点必须在 `LATEST.md` 和 TB_LOG 后续说明里继续保持窄化，避免团队把 TB-12 的“风险敞口索引”误认为“市场已经上线”。
    58	
    59	---
    60	
    61	## 1.2 TB-12 的关键正确点
    62	
    63	你这次没有直接接受 Gemini PASS，而是做了 ship-gate exact naming 对齐，补了 SG-12.6 的真实测试，修正了 SG-12.5 / SG-12.7 / SG-12.8 的命名漂移。这是正确的。
    64	
    65	因为在当前项目里，测试名不只是 cosmetic。它已经变成一种 traceability contract：
    66	
    67	```text
    68	架构师要求 SG-12.5；
    69	代码里必须有对应 gate；
    70	审计时必须能一眼对应。
    71	```
    72	
    73	所以：
    74	
    75	```text
    76	“架构正确性 PASS”
    77	不能替代
    78	“指定 ship gate exact-name PASS”
    79	```
    80	
    81	你让 AI coder 在 ship 前修掉这些 gap，是对的。
    82	
    83	---
    84	
    85	## 1.3 需要 carry forward 的 TB-12 风险
    86	
    87	TB-12 后续最重要的风险是：**NodePosition 会在后续被误用为 money ledger 或 tradable share balance。**
    88	
    89	必须继续写入后续 charter：
    90	
    91	```text
    92	NodePosition = immutable exposure record / index
    93	NodePosition != Coin
    94	NodePosition != YES/NO share
    95	NodePosition != tradable balance
    96	NodePosition != LP share
    97	NodePosition.amount not counted in total_supply_micro
    98	```
    99	
   100	TB-12 的路线文档本来也明确要求 `NodePosition.amount must not be included in total_supply_micro`，并且 Price / position signal 不能成为 truth。
   101	
   102	这条要在 TB-13/TB-14 反复保护，否则 CompleteSet 一接入，很容易出现“双账本”：
   103	
   104	```text
   105	stake balance
   106	node position
   107	YES/NO shares
   108	collateral
   109	```
   110	
   111	四者不能混。
   112	
   113	---
   114	
   115	# 2. 总体路线是否要调整？
   116	
   117	**核心路线不需要推翻，但 TB-13 必须加一个前置隔离步骤。**
   118	
   119	你提供的 TB-12 summary 里已经指出：
   120	
   121	```text
   122	TB-13 prerequisite:
   123	  隔离 src/prediction_market.rs legacy f64 CPMM
   124	  已在 OBS_TB_12_LEGACY_CPMM_QUARANTINE 跟踪
   125	```
   126	
   127	这是非常重要的。
   128	如果 TB-13 一边引入 `CompleteSet + MarketSeedTx`，一边仓库里还存在旧的 `f64 CPMM` 可被 import，那么未来很容易 drift 回：
   129	
   130	```text
   131	f64 price
   132	ghost liquidity
   133	automatic injection
   134	AMM before collateral accounting
   135	```
   136	
   137	所以我的第一个路线修正是：
   138	
   139	> **TB-13 的 Atom 0.5 必须是 legacy CPMM quarantine。**
   140	
   141	不是“顺手做”，而是 TB-13 的前置 ship gate。
   142	
   143	---
   144	
   145	# 3. 更新后的 TB-13 到 TB-17 总表
   146	
   147	```text
   148	TB-13  CompleteSet + MarketSeedTx
   149	       先隔离 legacy f64 CPMM；引入 1 locked Coin = YES_E + NO_E；不做 AMM。
   150	
   151	TB-14  PriceIndex v0 + Boltzmann Masking
   152	       价格是统计信号，不是真理；mask 只影响 read view / scheduler。
   153	
   154	TB-15  Lamarckian Autopsy + Markov EvidenceCapsule
   155	       私有尸检 + Markov capsule，不广播 raw logs，不自动改谓词。
   156	
   157	TB-16  Controlled Market Smoke Arena
   158	       在沙盒里跑 compute + positions + price + mask + autopsy，不接真实资金/真实世界。
   159	
   160	TB-17  Real-World Readiness Gate
   161	       只做真实世界准入标准，不执行真实世界任务。
   162	```
   163	
   164	这个顺序仍然符合之前确定的原则：
   165	
   166	```text
   167	先失败锚点，再市场做空。
   168	先资本释放，再价格机制。
   169	先证据胶囊，再尸检学习。
   170	先受控市场，再真实世界。
   171	```
   172	
   173	这也是你前一版路线中明确写出的重排原则。
   174	
   175	---
   176	
   177	# 4. TB-13 — CompleteSet + MarketSeedTx
   178	
   179	## 4.1 目标
   180	
   181	引入 Polymarket / CTF 的数学核心：
   182	
   183	```text
   184	1 locked Coin = 1 YES_E + 1 NO_E
   185	```
   186	
   187	但 TB-13 **仍然不做**：
   188	
   189	```text
   190	AMM
   191	CPMM router
   192	orderbook
   193	MarketOrderTx
   194	MarketTradeTx
   195	PriceIndex
   196	DPMM / pro-rata
   197	automatic liquidity
   198	```
   199	
   200	TB-13 只做抵押与份额会计。
   201	
   202	---
   203	
   204	## 4.2 Atom 0.5：legacy CPMM quarantine
   205	
   206	这是 TB-13 的第一步。
   207	
   208	### Requirements
   209	
   210	```text
   211	FR-13.0.1
   212	src/prediction_market.rs legacy f64 CPMM must be quarantined.
   213	
   214	FR-13.0.2
   215	New CompleteSet / MarketSeedTx code must not import legacy prediction_market.rs.
   216	
   217	FR-13.0.3
   218	No f64 in CompleteSet / MarketSeedTx / market accounting path.
   219	
   220	FR-13.0.4
   221	Legacy CPMM must be clearly labeled:
   222	  legacy
   223	  not used by RSP-M
   224	  not constitutional
   225	  not production market path.
   226	```
   227	
   228	### Ship gates
   229	
   230	```text
   231	SG-13.0.1
   232	legacy_cpm_api_not_imported_by_complete_set passes.
   233	
   234	SG-13.0.2
   235	no_f64_in_complete_set_or_market_seed passes.
   236	
   237	SG-13.0.3
   238	prediction_market_legacy_quarantined passes.
   239	
   240	SG-13.0.4
   241	OBS_TB_12_LEGACY_CPMM_QUARANTINE either closed or explicitly carried as non-importable legacy.
   242	```
   243	
   244	### Halting triggers
   245	
   246	```text
   247	HALT if new TB-13 code imports legacy prediction_market.rs.
   248	
   249	HALT if f64 appears in new CompleteSet / MarketSeed code.
   250	
   251	HALT if any AMM / CPMM router function is introduced in TB-13.
   252	```
   253	
   254	---
   255	
   256	## 4.3 新增对象
   257	
   258	```rust
   259	pub struct EventId(...);
   260	

codex
The project-level instructions confirm `src/state/sequencer.rs` is Step-B restricted, and there is no scoped `AGENTS.md` in or above this repo path. I’m now tracing the exact TB-13 code paths and comparing them to the charter, noting where the implementation intentionally diverges from the original strict balanced invariant.
exec
/bin/bash -lc "nl -ba handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | sed -n '256,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   256	## 4.3 新增对象
   257	
   258	```rust
   259	pub struct EventId(...);
   260	
   261	pub enum OutcomeSide {
   262	    Yes,
   263	    No,
   264	}
   265	
   266	pub struct ShareAmount {
   267	    pub units: i128,
   268	}
   269	
   270	pub struct ConditionalCollateralIndex(
   271	    pub BTreeMap<EventId, MicroCoin>
   272	);
   273	
   274	pub struct ConditionalShareBalances(
   275	    pub BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>
   276	);
   277	
   278	pub struct CompleteSetMintTx {
   279	    pub tx_id: TxId,
   280	    pub parent_state_root: Hash,
   281	    pub event_id: EventId,
   282	    pub owner: AgentId,
   283	    pub amount: MicroCoin,
   284	    pub signature: AgentSignature,
   285	}
   286	
   287	pub struct CompleteSetRedeemTx {
   288	    pub tx_id: TxId,
   289	    pub parent_state_root: Hash,
   290	    pub event_id: EventId,
   291	    pub owner: AgentId,
   292	    pub outcome: OutcomeSide,
   293	    pub share_amount: ShareAmount,
   294	    pub signature_or_system_resolution_ref: ResolutionRef,
   295	}
   296	
   297	pub struct MarketSeedTx {
   298	    pub tx_id: TxId,
   299	    pub parent_state_root: Hash,
   300	    pub event_id: EventId,
   301	    pub provider: AgentId,
   302	    pub collateral_amount: MicroCoin,
   303	    pub signature: AgentSignature,
   304	}
   305	```
   306	
   307	---
   308	
   309	## 4.4 Functional requirements
   310	
   311	```text
   312	FR-13.1
   313	CompleteSetMintTx debits balances_t by amount.
   314	
   315	FR-13.2
   316	CompleteSetMintTx credits conditional_collateral_t by amount.
   317	
   318	FR-13.3
   319	CompleteSetMintTx issues equal YES_E and NO_E shares.
   320	
   321	FR-13.4
   322	CompleteSetRedeemTx is impossible before system-resolved outcome.
   323	
   324	FR-13.5
   325	CompleteSetRedeemTx after YES outcome pays YES shares and not NO shares.
   326	
   327	FR-13.6
   328	MarketSeedTx uses explicit provider funds.
   329	
   330	FR-13.7
   331	MarketSeedTx may prepare collateralized YES/NO inventory for future market layers, but cannot quote, trade, or price.
   332	```
   333	
   334	---
   335	
   336	## 4.5 Constitutional requirements
   337	
   338	```text
   339	CR-13.1
   340	No ghost liquidity.
   341	
   342	CR-13.2
   343	No automatic YES/NO injection.
   344	
   345	CR-13.3
   346	YES/NO shares are claims, not Coin.
   347	
   348	CR-13.4
   349	Locked collateral is Coin holding; shares are not.
   350	
   351	CR-13.5
   352	on_init remains the only base Coin mint.
   353	
   354	CR-13.6
   355	Price / share state cannot override predicates or challenge outcome.
   356	```
   357	
   358	---
   359	
   360	## 4.6 Ship gates
   361	
   362	```text
   363	SG-13.1
   364	Mint 1 Coin -> 1 YES + 1 NO, total Coin conserved.
   365	
   366	SG-13.2
   367	YES/NO shares are not counted in total Coin supply.
   368	
   369	SG-13.3
   370	MarketSeedTx fails if provider lacks balance.
   371	
   372	SG-13.4
   373	MarketSeedTx cannot create liquidity without collateral.
   374	
   375	SG-13.5
   376	Redeem unavailable before outcome resolution.
   377	
   378	SG-13.6
   379	Redeem after YES outcome pays YES, not NO.
   380	
   381	SG-13.7
   382	No f64 in new CompleteSet / MarketSeed path.
   383	
   384	SG-13.8
   385	No import/use of legacy CPMM in TB-13 modules.
   386	```
   387	
   388	---
   389	
   390	## 4.7 Forbidden
   391	
   392	```text
   393	No automatic per-node 100 YES + 100 NO.
   394	No Treasury magic seed unless Treasury balance is debited.
   395	No DPMM / pro-rata payout.
   396	No AMM.
   397	No CLOB / orderbook.
   398	No MarketOrderTx.
   399	No MarketTradeTx.
   400	No price oracle.
   401	No NodeMarketEntry canonical state.
   402	No f64.
   403	```
   404	
   405	---
   406	
   407	## 4.8 Loop-mode instruction
   408	
   409	AI coder may run TB-13 atoms autonomously until pre-ship audit.
   410	
   411	Must halt immediately if:
   412	
   413	```text
   414	total_supply_micro changes incorrectly;
   415	shares are counted as Coin;
   416	MarketSeedTx succeeds without balance debit;
   417	legacy CPMM import appears;
   418	f64 appears in new market modules;
   419	any price / AMM / trade logic appears.
   420	```
   421	
   422	Risk class:
   423	
   424	```text
   425	Class 3
   426	```
   427	
   428	Reason:
   429	
   430	```text

exec
/bin/bash -lc "nl -ba handover/tracer_bullets/TB-13_charter_2026-05-03.md | sed -n '260,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	     //! Retroactive deletion in TB-13 would break the production evaluator
   261	     //! (`experiments/minif2f_v4/src/bin/evaluator.rs:1323` calls
   262	     //! `bus.kernel.market_ticker(5)`) + 10+ test files; that is out-of-scope
   263	     //! per `feedback_no_retroactive_evidence_rewrite` and architect §4.2
   264	     //! halting-trigger semantics (which target NEW TB-13 code, not existing
   265	     //! scaffolding).
   266	     ```
   267	   - Update `src/kernel.rs` `markets` / `bounty_market` / `bounty_lp_seed` field doc-comments with `LEGACY: f64 CPMM scaffolding; see prediction_market.rs module header for migration path`.
   268	
   269	(c) OBS update:
   270	   - Append new section to `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` titled "## TB-13 Atom 0.5 update (forward-fence + label in place)" documenting that:
   271	     - Forward-fence ship-gate `tests/tb_13_legacy_cpmm_forward_fence.rs` is in place + passes.
   272	     - Module-header + field-level labels added to `src/prediction_market.rs` and `src/kernel.rs`.
   273	     - Hard removal carries forward to TB-14 SHIP prerequisite (unchanged).
   274	
   275	**Tests**:
   276	- I-A0.5-1 `legacy_cpm_api_not_imported_by_complete_set` (architect SG-13.0.1 EXACT name): forbidden-token grep on TB-13 additions passes.
   277	- I-A0.5-2 `no_f64_in_complete_set_or_market_seed` (architect SG-13.0.2 EXACT name): grep ` f64 ` in TB-13 additions fails (zero matches).
   278	- I-A0.5-3 `prediction_market_legacy_quarantined` (architect SG-13.0.3 EXACT name): module header doc-comment contains "legacy" + "not constitutional" + "not RSP-M" tokens; verified via `include_str!`.
   279	
   280	**Ship gate (Atom 0.5)**:
   281	- G-A0.5-a SG-13.0.1..3 (3 EXACT-named tests) pass.
   282	- G-A0.5-b OBS_TB_12_LEGACY_CPMM_QUARANTINE updated with TB-13 status (architect SG-13.0.4 — "OBS either closed or explicitly carried as non-importable legacy"; we carry).
   283	- G-A0.5-c `cargo test --workspace` count baseline preserved at 759 (TB-12 baseline) plus the 3 new SG-13.0.x tests = 762; failed=0; ignored=150 unchanged.
   284	
   285	**Halting triggers** (Part A §4.2):
   286	- HALT if a NEW TB-13 module imports legacy `prediction_market`.
   287	- HALT if `f64` appears in NEW CompleteSet / MarketSeed code.
   288	- HALT if any AMM / CPMM router function is INTRODUCED in TB-13.
   289	
   290	**Iter cap**: 4h.
   291	
   292	**Checkpoint CP-A**: After Atom 0.5, AI coder pauses to verify forward-fence is robust before proceeding to Atom 1 typed-tx schemas.
   293	
   294	---
   295	
   296	### Atom 1 — TB-13 typed_tx schemas (Class 2)
   297	
   298	**Action**: pure additive schema; no dispatch logic.
   299	
   300	(a) `src/state/typed_tx.rs` additions (STEP_B_PROTOCOL parallel-branch):
   301	
   302	```rust
   303	/// TRACE_MATRIX TB-13 Atom 1 (architect 2026-05-03 ruling Part A §4.3):
   304	/// Event identifier for conditional shares. TB-13 maps EventId 1:1 to
   305	/// TaskId; future TB-14+ may decouple (e.g., per-node events).
   306	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   307	pub struct EventId(pub TaskId);
   308	
   309	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): outcome side discriminator.
   310	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   311	#[repr(u8)]
   312	pub enum OutcomeSide {
   313	    Yes = 0,
   314	    No = 1,
   315	}
   316	
   317	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
   318	/// Architect spec uses `units: i128`; we use u128 since shares can never
   319	/// go negative (mint creates positive; redeem decreases positive; no debt
   320	/// model in TB-13). Underflow is sequencer-side error (RedeemMoreThanOwned).
   321	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
   322	pub struct ShareAmount {
   323	    pub units: u128,
   324	}
   325	
   326	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): system-resolution
   327	/// reference. References either a `TaskBankruptcyTx` (NO outcome) or a
   328	/// `FinalizeRewardTx` (YES outcome) for the event. Sequencer validates
   329	/// the reference exists in L4 + outcome matches before allowing redeem.
   330	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   331	pub struct ResolutionRef {
   332	    pub resolution_tx_id: TxId,
   333	    pub claimed_outcome: OutcomeSide,
   334	}
   335	
   336	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + §4.4 FR-13.1..3):
   337	/// CompleteSetMintTx — agent debits Coin balance, locks as conditional
   338	/// collateral, receives equal YES_E + NO_E shares.
   339	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   340	pub struct CompleteSetMintTx {
   341	    pub tx_id: TxId,
   342	    pub parent_state_root: Hash,
   343	    pub event_id: EventId,
   344	    pub owner: AgentId,
   345	    pub amount: MicroCoin,
   346	    pub signature: AgentSignature,
   347	}
   348	
   349	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + §4.4 FR-13.4..5):
   350	/// CompleteSetRedeemTx — agent claims winning shares post-resolution.
   351	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   352	pub struct CompleteSetRedeemTx {
   353	    pub tx_id: TxId,
   354	    pub parent_state_root: Hash,
   355	    pub event_id: EventId,
   356	    pub owner: AgentId,
   357	    pub outcome: OutcomeSide,
   358	    pub share_amount: ShareAmount,
   359	    pub resolution_ref: ResolutionRef,
   360	    pub signature: AgentSignature,
   361	}
   362	
   363	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + §4.4 FR-13.6..7):
   364	/// MarketSeedTx — provider explicitly funds collateral + protocol-owned
   365	/// share inventory. NO trading. NO quoting. NO pricing.
   366	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   367	pub struct MarketSeedTx {
   368	    pub tx_id: TxId,
   369	    pub parent_state_root: Hash,
   370	    pub event_id: EventId,
   371	    pub provider: AgentId,
   372	    pub collateral_amount: MicroCoin,
   373	    pub signature: AgentSignature,
   374	}
   375	```
   376	
   377	(b) `TypedTx` enum extended with 3 new variants; `TxKind::*` extended with 3 new discriminants; `CanonicalMessage::*Signing` extended with 3 new variants; 3 new `DOMAIN_*_V1` prefix consts; `TxId::compute` gains 3 new domain matches.
   378	
   379	(c) `src/state/mod.rs` re-exports updated.
   380	
   381	(d) Trust Root rehash for `src/state/typed_tx.rs`.
   382	
   383	**Tests**:
   384	- U1 `complete_set_mint_round_trips_canonical`: round-trip canonical encode + decode.
   385	- U2 `complete_set_redeem_round_trips_canonical`: same.
   386	- U3 `market_seed_round_trips_canonical`: same.
   387	- U4 `outcome_side_repr_u8_stable`: discriminant golden test (Yes=0, No=1).
   388	- U5 `share_amount_default_zero_units`.
   389	- U6 `event_id_round_trip`.
   390	- U7 `tb_13_typed_tx_deterministic_digest`: same payload twice → same `tx_id`.
   391	
   392	**Forbidden in Atom 1**:
   393	- ❌ Adding `MarketBuyTx` / `MarketSellTx` / `MarketOrderTx` / `MarketTradeTx`.
   394	- ❌ Adding `price_yes` / `price_no` fields to MarketSeedTx (TB-14 territory).
   395	- ❌ f64 anywhere in new structs.
   396	- ❌ `i128` in `ShareAmount` (use u128; shares are non-negative).
   397	- ❌ Removing or modifying any TB-3..TB-12 typed_tx variant.
   398	
   399	**Iter cap**: 6h.
   400	
   401	---
   402	
   403	### Atom 2 — Sequencer dispatch + EconomicState 11→13 (Class 3)
   404	
   405	**Action**: STEP_B_PROTOCOL parallel-branch since touches `src/state/sequencer.rs` and `src/state/q_state.rs` restricted files.
   406	
   407	(a) `src/state/q_state.rs` extension:
   408	
   409	```rust
   410	/// TRACE_MATRIX TB-13 Atom 2 (architect Part A §4.3): conditional
   411	/// collateral per event. Locked Coin held against outstanding YES_E +
   412	/// NO_E share inventory. CR-13.4: locked collateral IS a Coin holding;
   413	/// included in 6-holding CTF sum (extends 5→6).
   414	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   415	pub struct ConditionalCollateralIndex(pub BTreeMap<EventId, MicroCoin>);
   416	
   417	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): conditional share
   418	/// balances per (owner, event, side). CR-13.3: shares are claims, NOT
   419	/// Coin; EXCLUDED from total Coin supply.
   420	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   421	pub struct ConditionalShareBalances(
   422	    pub BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>,
   423	);
   424	```
   425	
   426	Add to `EconomicState`:
   427	```rust
   428	#[serde(default)]
   429	pub conditional_collateral_t: ConditionalCollateralIndex,
   430	#[serde(default)]
   431	pub conditional_share_balances_t: ConditionalShareBalances,
   432	```
   433	
   434	Update field-count test: 11 → 13.
   435	
   436	(b) `src/state/sequencer.rs` 3 new dispatch arms:
   437	
   438	**CompleteSetMintTx accept arm**:
   439	```text
   440	1. Verify signature against pinned_pubkeys[owner].
   441	2. Verify balances_t[owner] >= amount → InsufficientBalanceForMint else.
   442	3. Debit balances_t[owner] by amount.
   443	4. Credit conditional_collateral_t[event_id] by amount.
   444	5. Credit conditional_share_balances_t[(owner, event_id, Yes)] by amount.units.
   445	6. Credit conditional_share_balances_t[(owner, event_id, No)] by amount.units.
   446	7. Advance state_root via COMPLETE_SET_MINT_DOMAIN_V1.
   447	```
   448	
   449	**CompleteSetRedeemTx accept arm**:
   450	```text
   451	1. Verify signature against pinned_pubkeys[owner].
   452	2. Verify resolution_ref.resolution_tx_id ∈ L4 accepted set; lookup the
   453	   referenced tx.
   454	3. If TaskBankruptcyTx for event_id.0 (== task_id): claimed_outcome must
   455	   be No; else InvalidResolutionRef.
   456	4. If FinalizeRewardTx for event_id.0: claimed_outcome must be Yes; else
   457	   InvalidResolutionRef.
   458	5. Else (no resolution found / wrong type): RedeemBeforeResolution.
   459	6. Verify conditional_share_balances_t[(owner, event_id, outcome)] >=
   460	   share_amount.units; else RedeemMoreThanOwned.
   461	7. Verify conditional_collateral_t[event_id] >= share_amount.units; else
   462	   InsufficientCollateral (defensive; should never fire if invariant holds).
   463	8. Debit conditional_share_balances_t[(owner, event_id, outcome)] by
   464	   share_amount.units.
   465	9. Debit conditional_collateral_t[event_id] by share_amount.units.
   466	10. Credit balances_t[owner] by share_amount.units (1:1 winning-share
   467	    redemption).
   468	11. Advance state_root via COMPLETE_SET_REDEEM_DOMAIN_V1.
   469	```
   470	
   471	**MarketSeedTx accept arm**:
   472	```text
   473	1. Verify signature against pinned_pubkeys[provider].
   474	2. Verify collateral_amount > 0; else InsufficientCollateral.
   475	3. Verify balances_t[provider] >= collateral_amount; else
   476	   InsufficientBalanceForMint (architect SG-13.3).
   477	4. Debit balances_t[provider] by collateral_amount.
   478	5. Credit conditional_collateral_t[event_id] by collateral_amount.
   479	6. Credit conditional_share_balances_t[(provider, event_id, Yes)] by
   480	   collateral_amount.units.
   481	7. Credit conditional_share_balances_t[(provider, event_id, No)] by
   482	   collateral_amount.units.
   483	8. Advance state_root via MARKET_SEED_DOMAIN_V1.
   484	```
   485	
   486	(c) Agent-ingress fail-closed: extend `submit_agent_tx` ingress matcher to accept the 3 new variants; verify signature; reject if signing payload mismatch.
   487	
   488	(d) `TransitionError` additive: `InsufficientBalanceForMint`, `RedeemBeforeResolution`, `RedeemMoreThanOwned`, `InsufficientCollateral`, `InvalidResolutionRef`.
   489	
   490	(e) Trust Root rehash for `src/state/sequencer.rs` + `src/state/q_state.rs`.
   491	
   492	**Tests** (in `src/state/sequencer.rs` test module):
   493	- U-D1 `mint_debits_balance_credits_collateral_and_shares`: balances_t -= amount, conditional_collateral_t[event] += amount, both YES_E and NO_E shares += amount.units.
   494	- U-D2 `mint_fails_with_insufficient_balance`.
   495	- U-D3 `redeem_before_resolution_rejected`.
   496	- U-D4 `redeem_after_yes_pays_yes_not_no`: with FinalizeRewardTx resolution_ref + outcome=Yes; YES share balance -= amount; NO share balance unchanged; balances_t += amount.
   497	- U-D5 `redeem_after_no_pays_no_not_yes`: with TaskBankruptcyTx resolution_ref + outcome=No.
   498	- U-D6 `redeem_with_wrong_outcome_for_resolution_rejected`.
   499	- U-D7 `market_seed_requires_balance`.
   500	- U-D8 `market_seed_zero_collateral_rejected`.
   501	- U-D9 `market_seed_provider_owns_both_sides`.
   502	
   503	**Iter cap**: 12h.
   504	
   505	**Checkpoint CP-B**: After Atom 2, AI coder runs `cargo test --workspace` and verifies field-count test (11→13), all U-D1..9 PASS, no regression in TB-3..TB-12 tests. Conservation invariant test still passes (until Atom 3 extends it). Then proceeds to Atom 3.
   506	
   507	---
   508	
   509	### Atom 3 — Conservation invariant extension (Class 3)
   510	
   511	**Action**: extend `src/economy/monetary_invariant.rs`.
   512	
   513	(a) `assert_total_ctf_conserved`:
   514	   - 5-holding sum → 6-holding sum: `total_supply_micro = Σ balances_t + Σ escrows_t + Σ stakes_t + Σ claims_t + Σ challenge_cases_t + Σ conditional_collateral_t`.
   515	   - `conditional_share_balances_t` EXCLUDED (architect CR-13.3 + SG-13.2: "shares are not Coin").
   516	
   517	(b) `assert_complete_set_balanced` NEW:
   518	   - For every `event_id ∈ conditional_collateral_t.keys()`:
   519	     - `Σ_{owner} conditional_share_balances_t[(owner, event_id, Yes)].units == conditional_collateral_t[event_id].units`
   520	     - AND `Σ_{owner} conditional_share_balances_t[(owner, event_id, No)].units == conditional_collateral_t[event_id].units`
   521	   - Holds post-mint (1 Coin → 1 YES + 1 NO equal mint).
   522	   - Holds post-redeem (debit equal in YES side and collateral OR equal in NO side and collateral).
   523	
   524	(c) `assert_no_post_init_mint`:
   525	   - Add 3 new TypedTx variants to exhaustive match: CompleteSetMint, CompleteSetRedeem, MarketSeed. None create money (mint = balance debit + collateral credit; redeem = collateral debit + balance credit; seed = balance debit + collateral credit). Match arms re-assert via assertion that no balance increased net of debit+credit.
   526	
   527	(d) Re-emit `EconomicState` field count test: 13.
   528	
   529	**Tests**:
   530	- I-T13-1 `mint_preserves_total_supply`: pre/post mint, total_supply 6-holding sum bit-equal.
   531	- I-T13-2 `redeem_preserves_total_supply`.
   532	- I-T13-3 `market_seed_preserves_total_supply`.
   533	- I-T13-4 `complete_set_balanced_post_mint`.
   534	- I-T13-5 `complete_set_balanced_post_redeem`.
   535	- I-T13-6 `shares_excluded_from_total_supply`: assert `conditional_share_balances_t` units NOT in total_supply_micro sum.
   536	
   537	**Iter cap**: 6h.
   538	
   539	**Checkpoint CP-C**: After Atom 3, AI coder runs full `cargo test --workspace` + verifies CP-C kill criteria:
   540	- assert_total_ctf_conserved passes pre/post every TB-13 typed_tx
   541	- assert_complete_set_balanced holds at every state transition
   542	- assert_no_post_init_mint passes (no money created)
   543	- TB-3..TB-12 baseline tests unchanged (no regression on the 759+3 baseline)
   544	**This is THE money-safety gate**. If any of these fails, HALT immediately and report.
   545	
   546	---
   547	
   548	### Atom 4 — Dashboard §14 (Class 1)
   549	
   550	**Action**: extend `src/bin/audit_dashboard.rs` with §14 conditional-shares view.
   551	
   552	(a) New audit-row structs:
   553	   - `ConditionalCollateralRow { event_id, locked_collateral, share_depth_yes, share_depth_no, balanced: bool }`.
   554	   - `ConditionalShareRow { owner, event_id, side, share_amount }` filtered.
   555	
   556	(b) New `DashboardReport` field `conditional_shares: ConditionalSharesSection { collaterals: Vec<...>, holdings: Vec<...> }`.
   557	
   558	(c) Walk: post-replay reads from `economic_state_t.conditional_collateral_t` + `.conditional_share_balances_t`; aggregates per-event YES depth + NO depth + balanced flag.
   559	
   560	(d) Render section: "§14 TB-13 Conditional collateral and share balances (per architect 2026-05-03 ruling Part A §4: claims, NOT Coin balance; locked collateral, NOT free coin)".

exec
/bin/bash -lc "nl -ba handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-13 Recursive Self-Audit — CompleteSet + MarketSeedTx
     2	
     3	**Audit type**: Recursive self-audit (Class 3 envelope; per architect 2026-05-03 post-TB-12 ruling Part A §4.8 + charter §3 Atom 6).
     4	**Date**: 2026-05-03 evening.
     5	**Scope**: TB-13 Atoms 0 + 0.5 + 1 + 2 + 3 + 5 SHIPPED to local main; Atom 4 (dashboard §14) DEFERRED to TB-14 (not in architect Part A spec); this document is Atom 6(a). External Codex + Gemini audits pending — Atom 6(b) + 6(c).
     6	**Charter**: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
     7	**Architect ruling lossless**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`.
     8	**Local commits**:
     9	- `32aab27` Atom 0 + 0.5 — Charter + legacy f64 CPMM forward-fence
    10	- `70303af` Atom 1 — typed_tx schemas (+8 unit tests)
    11	- `1806432` Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests
    12	
    13	**Executive verdict**: **PASS** with no halting triggers fired.
    14	External Codex + Gemini audits MUST follow before SHIP per architect §11 master instruction (`feedback_dual_audit` Class 3 = full hybrid dual; conservative-verdict-wins on disagreement).
    15	
    16	---
    17	
    18	## §1 Clause 1 — Constitutional preservation
    19	
    20	| Article                                  | TB-13 invariant                                                                                                | Verification                                                                              |
    21	| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
    22	| Art. 0.2 Tape Canonical                  | 3 NEW typed-tx variants (CompleteSetMint / CompleteSetRedeem / MarketSeed) canonical-encoded. 3 NEW state-root domain prefixes (`COMPLETE_SET_MINT_DOMAIN_V1` / `COMPLETE_SET_REDEEM_DOMAIN_V1` / `MARKET_SEED_DOMAIN_V1`). Replay-deterministic from typed-tx fields. | `tb_13_complete_set_mint_round_trips_canonical` + `tb_13_complete_set_redeem_round_trips_canonical` + `tb_13_market_seed_round_trips_canonical` + `tb_13_signing_payloads_deterministic_digest` |
    23	| Art. I.1 5-step compile loop closure     | Conditional collateral & share accounting are part of the proposal-pricing substrate, but DO NOT affect predicate outcome (CR-13.6: "Price / share state cannot override predicates or challenge outcome"). | TB-13 dispatch arms operate purely on `economic_state_t.{balances_t, conditional_collateral_t, conditional_share_balances_t}`; predicate-evaluation paths (TB-3 Work / TB-4 Verify+Challenge / TB-5 ChallengeResolve / TB-8 FinalizeReward) UNCHANGED. |
    24	| Art. II.2.1 entropy / quantize-broadcast-shield | Conditional share balances are public economic record (no shielding). Architect Part A §4 makes no shielding requirement; failure capsules (TB-11 EvidenceCapsule) remain the privacy-shielded surface. | No new shielded surface introduced. |
    25	| Art. III.4 no fake accepted              | CompleteSetRedeemTx requires sequencer-side validation of `task_markets_t[event_id.0].state ∈ {Finalized, Bankrupt}` AND `resolution_ref.claimed_outcome` matches the state. Pre-resolution rejected with `RedeemBeforeResolution`; wrong-outcome-for-state rejected with `InvalidResolutionRef`. Owner cannot self-resolve. | `sg_13_5_redeem_unavailable_before_outcome_resolution` (Open + Expired states both reject); `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (mismatch outcome rejected with InvalidResolutionRef on Finalized event; symmetric Bankrupt check) |
    26	| Art. V.1.3 Anti-Oreo                     | All 3 TB-13 typed-tx are AGENT-SIGNED (CompleteSetMint / CompleteSetRedeem / MarketSeed). NO new system_tx variant introduced. Provider funds are EXPLICIT in MarketSeedTx (NO automatic seed; CR-13.1 + CR-13.2 forbid ghost / automatic liquidity). | `submit_agent_tx` ingress agent-fall-through arm extended for 3 new variants (verified via reading `src/state/sequencer.rs:1996..2002`). NO `emit_system_tx` arm added for TB-13. |
    27	
    28	**Verdict 1: PASS**. Zero constitutional violations introduced.
    29	
    30	---
    31	
    32	## §2 Clause 2 — Replay-deterministic
    33	
    34	| Property                                  | TB-13 verification                                                                                                                |
    35	| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
    36	| Q-projection determinism                  | Conditional collateral is a deterministic projection of accepted CompleteSetMint / MarketSeed tx fields (`amount.micro_units()` / `collateral_amount.micro_units()`). Conditional share balances are deterministic projections (Yes + No each = `amount.micro_units() as u128`). Redeem deterministically debits winning side + collateral by `share_amount.units`. No environmental input. |
    37	| Cross-instance replay equality            | TB-3..TB-12 replay invariants unchanged. TB-13 dispatch arms add to `q_next.economic_state_t.{balances_t, conditional_collateral_t, conditional_share_balances_t}` only — each mutation is field-by-field deterministic. |
    38	| State-root advance                        | 3 new domain helpers `complete_set_mint_accept_state_root` / `complete_set_redeem_accept_state_root` / `market_seed_accept_state_root` mirror the TB-3 / TB-11 / TB-12 SHA-256-of-(domain ∥ prev_root ∥ canonical_encoded(tx)) pattern. Domain prefixes are unique per-arm. |
    39	
    40	**Verdict 2: PASS**.
    41	
    42	---
    43	
    44	## §3 Clause 3 — Conservation (CTF)
    45	
    46	| Conservation invariant                    | TB-13 enforcement                                                                                                                                                                                |
    47	| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    48	| **6-holding CTF (post-TB-13)**            | Σ holdings = balances_t + escrows_t + stakes_t + challenge_cases_t + conditional_collateral_t. Atom 3 extends the TB-7R 5-holding sum to 6 by adding `conditional_collateral_t` per architect CR-13.4 ("Locked collateral is Coin holding"). conditional_share_balances_t INTENTIONALLY OMITTED per CR-13.3 + SG-13.2. |
    49	| `assert_total_ctf_conserved` post-mint    | `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` + `sg_13_2_yes_no_shares_not_in_total_coin_supply` + `halt_total_supply_micro_unchanged_across_mint_redeem`: balance debit = collateral credit, total preserved bit-for-bit. PASS.  |
    50	| `assert_total_ctf_conserved` post-redeem  | Redeem: collateral debit + balance credit (1:1). `halt_total_supply_micro_unchanged_across_mint_redeem` exercises mint+redeem and asserts `assert_total_ctf_conserved(pre, post, &[]).is_ok()`. PASS. |
    51	| `assert_total_ctf_conserved` post-seed    | `halt_complete_set_balanced_post_seed`: provider balance debit = collateral credit. CTF preserved. PASS. |
    52	| `assert_no_post_init_mint`                | All 3 TB-13 variants in exhaustive match arm — none create money (mint = balance↔collateral migration; redeem = collateral↔balance migration; seed = balance↔collateral migration). PASS. |
    53	| `assert_complete_set_balanced` (NEW)      | For every event in conditional_collateral_t: `min(Σ_yes, Σ_no) == collateral`. Pre-resolution (mint+seed): Σ_yes == Σ_no == collateral (both equal trivially equivalent). Post-redeem: winning side equals collateral; losing side may be larger (stranded zero-value claims). Verified: `sg_13_1` post-mint balanced; `halt_complete_set_balanced_post_seed`; `halt_total_supply_micro_unchanged_across_mint_redeem` post-redeem balanced. |
    54	
    55	**MIN-semantics rationale**: a strict `Σ_yes == collateral AND Σ_no == collateral` requirement does NOT hold post-redemption (the losing side has stranded shares above the now-decremented collateral). MIN form correctly captures: every Coin in collateral can be redeemed by the winning side, and no winning-side share is unbacked. This was discovered mid-test by my own halting-trigger guard `halt_total_supply_micro_unchanged_across_mint_redeem`; the strict-equality form initially failed and was replaced with MIN form. The bug-find demonstrates the recursive self-audit harness working as intended.
    56	
    57	**Verdict 3: PASS**.
    58	
    59	---
    60	
    61	## §4 Clause 4 — Resolution gating (TB-13-unique)
    62	
    63	| Property                                  | TB-13 verification                                                                                                                                                                                |
    64	| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    65	| 4.1 Redeem requires resolved task_market state | `task_markets_t[event_id.0].state ∈ {Open, Expired}` → `RedeemBeforeResolution`. Verified: `sg_13_5_redeem_unavailable_before_outcome_resolution` covers both Open and Expired. |
    66	| 4.2 Resolution-outcome match enforced      | Finalized state → only outcome=Yes accepted; Bankrupt state → only outcome=No accepted; mismatch → `InvalidResolutionRef`. Verified: `sg_13_6` includes 2 mismatch checks (Finalized+No rejected; Bankrupt+Yes rejected). |
    67	| 4.3 ResolutionRef inner-consistency        | Pre-state-lookup gate: `redeem.outcome != redeem.resolution_ref.claimed_outcome` → `InvalidResolutionRef`. Catches malformed wire payloads where the redeem's outcome field disagrees with the resolution_ref's claimed_outcome (defense-in-depth before task_markets_t lookup). |
    68	| 4.4 Owner share-balance gate               | `RedeemMoreThanOwned` if `conditional_share_balances_t[owner][event_id].{yes|no}` < `share_amount.units`. Verified: `halt_redeem_more_than_owned_rejected`. |
    69	| 4.5 Collateral coverage gate               | `InsufficientCollateral` if `conditional_collateral_t[event_id]` < `share_amount.units` (defensive; should never fire if `assert_complete_set_balanced` holds). |
    70	
    71	**Verdict 4: PASS**. All 5 admission gates exercised by integration tests.
    72	
    73	---
    74	
    75	## §5 Clause 5 — Forward-fence + label discipline (Atom 0.5)
    76	
    77	| Property                                  | TB-13 verification                                                                                                                                                                                |
    78	| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
    79	| 5.1 Module-header LEGACY label            | `src/prediction_market.rs` carries `//! # LEGACY ...` block declaring: not constitutional, not RSP-M, not production market path; lists each constitutional non-compliance (f64 / automatic liquidity / trading semantics); names migration path (TB-13/TB-14). Verified: `prediction_market_legacy_quarantined` (SG-13.0.3). |
    80	| 5.2 Field-level LEGACY labels             | `src/kernel.rs` carries field-level `LEGACY` doc-comments on `markets`, `bounty_market`, `bounty_lp_seed`. Verified: `prediction_market_legacy_quarantined` defense-in-depth check. |
    81	| 5.3 Forward-fence ship-gate test          | `tests/tb_13_legacy_cpmm_forward_fence.rs` — 3 EXACT-named tests `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1) + `no_f64_in_complete_set_or_market_seed` (SG-13.0.2) + `prediction_market_legacy_quarantined` (SG-13.0.3). Span detector uses authoring-marker rule (TRACE_MATRIX TB-13 / TB-13 line-prefix) to avoid false positives from TB-12 doc-comments referencing TB-13 as future work. |
    82	| 5.4 OBS carry-forward                     | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` updated with TB-13 Atom 0.5 status section; SG-13.0.4 satisfied as "carry forward to TB-14 SHIP prerequisite". |
    83	| 5.5 NO retroactive deletion               | `src/prediction_market.rs` and `src/kernel.rs` market scaffolding NOT removed (production wiring at `src/bus.rs:206/327/359/480-515` + `experiments/minif2f_v4/src/bin/evaluator.rs:1323` + 10+ test files would break). Out of scope per `feedback_no_retroactive_evidence_rewrite` and architect §4.2 halting-trigger semantics (which target NEW TB-13 code, not existing scaffolding). |
    84	
    85	**Verdict 5: PASS**. Forward-fence binding established; legacy CPMM clearly labeled as non-importable; TB-14 SHIP prerequisite preserved.
    86	
    87	---
    88	
    89	## §6 Architect halting triggers (charter §3 Atom 0; architect Part A §4.8) — NOT triggered
    90	
    91	| Halting trigger                                                  | Result                                                                                                       |
    92	| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
    93	| total_supply_micro mutation incorrect (CTF conservation failure) | NOT triggered. `halt_total_supply_micro_unchanged_across_mint_redeem` + `halt_shares_not_counted_as_coin` PASS. |
    94	| Conditional shares counted as Coin (architect SG-13.2 violation)  | NOT triggered. `total_supply_micro` excludes `conditional_share_balances_t` per CR-13.3 implementation. `sg_13_2_yes_no_shares_not_in_total_coin_supply` PASS. |
    95	| MarketSeedTx succeeds without balance debit (architect SG-13.3)   | NOT triggered. `halt_market_seed_zero_balance_provider_rejected` + `sg_13_3_market_seed_fails_if_provider_lacks_balance` PASS. |
    96	| Legacy `prediction_market::` import in NEW TB-13 module           | NOT triggered. `legacy_cpm_api_not_imported_by_complete_set` PASS. |
    97	| f64 appears in NEW market modules                                 | NOT triggered. `no_f64_in_complete_set_or_market_seed` PASS. |
    98	| Any AMM / CPMM router / price / trade logic introduced            | NOT triggered. Forward-fence forbidden-token grep on TB-13-marked spans excludes `MarketOrderTx`, `MarketTradeTx`, `AMM`, `CPMM`, `DPMM`, `orderbook`, `price_yes`, `price_no`, `PriceIndex`. PASS. (Actual catch-and-fix mid-development: `PriceIndex` reference in MarketSeedTx + transition_ledger TxKind::MarketSeed doc-comments was correctly flagged and removed.) |
    99	| Codex / Gemini VETO                                               | DEFERRED to Atom 6(b) + 6(c). Self-audit verdict here is PASS; external audits next. |
   100	
   101	---
   102	
   103	## §7 Ship gates (architect SG-13.0..8 + carry-forward G1..G11)
   104	
   105	**Architect SG-13.0..8** (charter §6):
   106	
   107	| Gate                                                              | Status              | Evidence                                                                                                                              |
   108	| ----------------------------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
   109	| SG-13.0.1 legacy_cpm_api_not_imported_by_complete_set             | ✓ pass (exact)      | `legacy_cpm_api_not_imported_by_complete_set` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                            |
   110	| SG-13.0.2 no_f64_in_complete_set_or_market_seed                   | ✓ pass (exact)      | `no_f64_in_complete_set_or_market_seed` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                                  |
   111	| SG-13.0.3 prediction_market_legacy_quarantined                    | ✓ pass (exact)      | `prediction_market_legacy_quarantined` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                                   |
   112	| SG-13.0.4 OBS_TB_12_LEGACY_CPMM_QUARANTINE carried as non-importable legacy | ✓ pass (carry) | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` "TB-13 Atom 0.5 update" section                                  |
   113	| SG-13.1   mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved | ✓ pass (exact) | `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` (`tests/tb_13_complete_set.rs`)                            |
   114	| SG-13.2   yes_no_shares_not_in_total_coin_supply                  | ✓ pass (exact)      | `sg_13_2_yes_no_shares_not_in_total_coin_supply`                                                                                       |
   115	| SG-13.3   market_seed_fails_if_provider_lacks_balance             | ✓ pass (exact)      | `sg_13_3_market_seed_fails_if_provider_lacks_balance`                                                                                   |
   116	| SG-13.4   market_seed_cannot_create_liquidity_without_collateral  | ✓ pass (exact)      | `sg_13_4_market_seed_cannot_create_liquidity_without_collateral`                                                                       |
   117	| SG-13.5   redeem_unavailable_before_outcome_resolution            | ✓ pass (exact)      | `sg_13_5_redeem_unavailable_before_outcome_resolution` (covers Open + Expired states)                                                  |
   118	| SG-13.6   redeem_after_yes_outcome_pays_yes_not_no                | ✓ pass (exact)      | `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (also covers symmetric Bankrupt → No path + 2 mismatch InvalidResolutionRef checks) |
   119	| SG-13.7   no_f64_in_new_complete_set_or_market_seed_path          | ✓ pass (delegation) | `sg_13_7_no_f64_in_new_complete_set_or_market_seed_path` delegates to `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed` (SG-13.0.2 fence) |
   120	| SG-13.8   no_import_or_use_of_legacy_cpmm_in_tb13_modules         | ✓ pass (delegation) | `sg_13_8_no_import_or_use_of_legacy_cpmm_in_tb13_modules` delegates to `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1 fence) |
   121	
   122	**12/12 architect SG-13.x ship gates PASS**.
   123	
   124	**Engineering carry-forward G1..G11** (TB-9 / TB-10 / TB-11 / TB-12 precedent):
   125	
   126	| Gate                                                              | Status     | Evidence                                                                                                |
   127	| ----------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------- |
   128	| G1 cargo check                                                    | ✓ pass     | Clean run; only legacy warnings carried over from TB-12.                                                |
   129	| G2 cargo test --workspace                                         | ✓ pass     | `783/0/150` (TB-12 baseline 759 + 3 fence + 8 unit + 13 integration). Workspace canonical per `feedback_workspace_test_canonical`. |
   130	| G3 lean_market 7 subcommands                                      | ✓ pass     | TB-12 baseline 7 subcommands unchanged (TB-13 does not add CLI; Atom 4 dashboard deferred).             |
   131	| G4 evaluator MAX_TX exhaust → EvidenceCapsule                     | ✓ pass     | TB-11 carry-forward unchanged.                                                                          |
   132	| G5 audit_dashboard §13 still renders                              | ✓ pass     | TB-12 §13 unchanged. §14 dashboard rendering deferred (architect Part A spec made no dashboard requirement). |
   133	| G6 verify_chaintape green                                         | ✓ pass     | replay-determinism verified at QState level (cross-instance replay equality); deterministic on TB-13 typed-tx via canonical_encode round-trip tests. |
   134	| G7 typed_tx variants: 3 NEW additive only                         | ✓ pass     | CompleteSetMint / CompleteSetRedeem / MarketSeed; existing variants unchanged.                          |
   135	| G8 dispatch arms: 3 NEW additive                                  | ✓ pass     | CompleteSetMint / CompleteSetRedeem / MarketSeed accept arms in `src/state/sequencer.rs`. Existing arms unchanged. |
   136	| G9 TransitionError variants: 5 NEW additive                       | ✓ pass     | InsufficientBalanceForMint / RedeemBeforeResolution / RedeemMoreThanOwned / InsufficientCollateral / InvalidResolutionRef. |
   137	| G10 EconomicState field count == 13                               | ✓ pass     | Was 11 post-TB-12; +conditional_collateral_t +conditional_share_balances_t. Asserted by `economic_state_has_thirteen_sub_fields` (q_state unit test) + 3 cross-test assertions in `tests/economic_state_reconstruct.rs` / `tests/q_state_reconstruct.rs` / `tests/six_axioms_alignment.rs`. |
   138	| G11 Conservation: 6-holding sum preserved + complete-set balanced | ✓ pass     | `total_supply_micro` extended with `conditional_collateral_t`; `assert_complete_set_balanced` enforced post-mint / post-seed / post-redeem in 3 halting-trigger tests. |
   139	
   140	**11/11 G ship gates PASS**.
   141	
   142	---
   143	
   144	## §8 Forbidden tokens grep summary (architect Part A §4.7 + halting triggers)
   145	
   146	Verified by `tests/tb_13_legacy_cpmm_forward_fence.rs` forbidden-token list applied to TB-13-marked spans across `src/state/{typed_tx,q_state,sequencer}.rs` + `src/economy/monetary_invariant.rs` + `src/bin/audit_dashboard.rs`:
   147	
   148	```text
   149	Banned tokens (would HALT if found in TB-13 span):
   150	  prediction_market::    NOT FOUND
   151	  BinaryMarket           NOT FOUND
   152	  .buy_yes(              NOT FOUND
   153	  .buy_no(               NOT FOUND
   154	  open_bounty_market     NOT FOUND
   155	  bounty_market          NOT FOUND
   156	  bounty_lp_seed         NOT FOUND
   157	  bounty_yes_price       NOT FOUND
   158	  resolve_bounty         NOT FOUND
   159	  market_ticker(         NOT FOUND
   160	  market_ticker_full(    NOT FOUND
   161	  MarketOrderTx          NOT FOUND
   162	  MarketTradeTx          NOT FOUND
   163	  MarketBuyTx            NOT FOUND
   164	  MarketSellTx           NOT FOUND
   165	  AMM                    NOT FOUND
   166	  CPMM                   NOT FOUND
   167	  DPMM                   NOT FOUND
   168	  orderbook              NOT FOUND
   169	  price_yes              NOT FOUND
   170	  price_no               NOT FOUND
   171	  PriceIndex             NOT FOUND
   172	  yes_price              NOT FOUND
   173	  no_price               NOT FOUND
   174	  RationalPrice          NOT FOUND
   175	  f64 (in money path)    NOT FOUND
   176	```
   177	
   178	22/22 forbidden tokens absent from TB-13-marked spans.
   179	
   180	Mid-development catch: 2 `PriceIndex` doc-comment references in MarketSeedTx + TxKind::MarketSeed were correctly flagged by the fence and replaced with implementation-detail-neutral language. The fence working as intended.
   181	
   182	---
   183	
   184	## §9 Audit-mode declaration (Class 3 dual; pending external Codex + Gemini)
   185	
   186	Per `feedback_dual_audit` + architect Part A §4.8 (Class 3 = money / collateral surface):
   187	
   188	- **Atom 6(a) self-audit (this document)**: PASS verdict. Clauses 1–5 all green; halting triggers 1–7 NOT triggered; 12/12 SG-13.x + 11/11 G ship gates pass.
   189	- **Atom 6(b) Codex impl-paranoid audit**: PENDING. Specific questions to be put to Codex (per charter §3 Atom 6.b):
   190	  1. Does CompleteSetMint create or destroy money? (must be balance↔collateral migration only)
   191	  2. Can Redeem fire without a system-emitted resolution? (must be sequencer-rejected)
   192	  3. Can Redeem with `outcome=Yes` and a TaskBankruptcy resolution_ref bypass the outcome check?
   193	  4. Does the 6-holding sum hold across all TB-13 typed_tx?
   194	  5. Does `assert_complete_set_balanced` hold after every transition?
   195	  6. Can MarketSeedTx create liquidity without provider balance? (must be rejected)
   196	  7. Are conditional shares anywhere counted as Coin? (must be excluded)
   197	  8. Could a malformed ShareAmount underflow? (u128 type guarantee + RedeemMoreThanOwned gate)
   198	  9. Forward-fence: any new TB-13 module file references legacy `prediction_market`?
   199	- **Atom 6(c) Gemini architectural strategic audit**: PENDING. Same 9 questions plus:
   200	  10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex (long/short interest derivable from `conditional_share_balances_t` aggregates)?
   201	  11. Does the EventId == TaskId 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?
   202	  12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?
   203	  13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form (vs. strict equality), particularly for adversarial patterns: e.g., re-mint after partial redeem, or repeated redeem-and-remint cycles?
   204	
   205	Conservative-verdict-wins on Codex ↔ Gemini disagreement (per `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.
   206	
   207	---
   208	
   209	## §10 Open follow-ups (carry-forward, NOT ship blockers)
   210	
   211	| Item                                                              | Reason / status                                                                                          |
   212	| ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
   213	| Atom 4 dashboard §14 (conditional collateral + share render)      | DEFERRED: not in architect Part A spec (no FR/CR/SG references it). TB-14 PriceIndex will need the same dashboard surface; consolidate then. |
   214	| Legacy `src/prediction_market.rs` + `src/kernel.rs` CPMM scaffolding hard-removal | TB-14 SHIP prerequisite per `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`. Forward-fence in TB-13 prevents new code from importing; full refactor before TB-14 SHIP.                |
   215	| `assert_complete_set_balanced` MIN-semantics formalization        | The MIN form was discovered mid-test as the correct invariant (vs. naive Σ_yes==Σ_no==coll). External audit may want stronger formalization (e.g., adversarial proof that no adversary can break MIN with a sequence of mints + partial redeems). |
   216	| EventId == TaskId 1:1 mapping                                     | TB-13 simplification: each task is one event. TB-14+ may decouple if per-node events are needed. Schema is forward-compat (EventId is a newtype around TaskId; future variants can wrap a different inner type).                                                                |
   217	
   218	---
   219	
   220	## §11 Concluding verdict
   221	
   222	TB-13 SHIPPED Atoms 0 + 0.5 + 1 + 2 + 3 + 5 to local main with PASS verdict on:
   223	- 5 constitutional clauses (Tape Canonical / 5-step compile loop / Anti-Oreo / no fake accepted / replay-deterministic + conservation + resolution gating + forward-fence + label discipline)
   224	- 12/12 architect SG-13.0..8 ship gates (EXACT-named, traceability-contract discipline)
   225	- 11/11 engineering carry-forward G1..G11 ship gates
   226	- 7/7 architect halting triggers NOT triggered
   227	
   228	`cargo test --workspace = 783/0/150` (TB-12 baseline 759 + 3 fence + 8 unit + 13 integration = 783; failed=0; ignored=150 unchanged).
   229	
   230	External Codex + Gemini audits pending per architect Part A §4.8 (Class 3 dual; conservative-verdict-wins on disagreement). User-checkpoint per architect §11 master instruction "Stop for user review at ship gate" reached: AI coder STOPS for user review of this self-audit + decision on whether to invoke Codex + Gemini external audits before Atom 7 SHIP.
   231	
   232	End of recursive self-audit.

exec
/bin/bash -lc "nl -ba handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-13 Real-LLM Smoke Evidence — 2026-05-03
     2	
     3	**Date**: 2026-05-03 evening (post-Atom 6(a) recursive self-audit; pre-external-audit).
     4	**TB**: TB-13 (CompleteSet + MarketSeedTx).
     5	**Source**: `target/debug/evaluator` HEAD = `17d4a3b` (Atom 6(a) ship), branch `main`.
     6	**Model**: `deepseek-chat` via local LLM proxy at `localhost:8080/v1/chat/completions`.
     7	**Lean**: 4.x (`/home/zephryj/.elan/bin/lean` runtime; problem from `turingosv3` minif2f Test corpus).
     8	**Charter**: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
     9	**Audit**: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`.
    10	
    11	---
    12	
    13	## §0 Headline
    14	
    15	**TB-13 schema works end-to-end under real-LLM workload**.
    16	
    17	Single-problem regression smoke (mathd_algebra_171, MAX_TX=10, n1):
    18	- Outcome: **UNSOLVED — hit_max_tx** (10/10 proposals failed predicates; expected for short MAX_TX).
    19	- Replay-determinism: **7/7 GREEN**.
    20	- EconomicState: **13 sub-fields confirmed** (TB-13 +`conditional_collateral_t` + `conditional_share_balances_t` persist correctly).
    21	- TB-13 additive changes: **NO regression** vs existing TB-3..TB-12 capability loop.
    22	
    23	## §1 What this smoke validates
    24	
    25	The smoke is a **regression check**, not a TB-13 capability demonstration. The
    26	LLM-driven path (Work / Verify / Challenge) does NOT submit any of the 3 new
    27	TB-13 typed-tx variants (CompleteSetMint / CompleteSetRedeem / MarketSeed) —
    28	those are user-driven economic-action tx, not solver-driven. The ground
    29	truth this smoke produces:
    30	
    31	1. The 13-sub-field `EconomicState` shape (post-TB-13) **serializes /
    32	   deserializes correctly** under live workload.
    33	2. The `verify_chaintape` replay reconstruction **succeeds bit-equal** with
    34	   the new schema (`economic_state_reconstructed: true`).
    35	3. The `audit_dashboard` §13 (TB-12 NodePosition) rendering **still works**
    36	   alongside the new fields (no §14 — Atom 4 deferred).
    37	4. The TB-11 Epistemic Exhaust §12 dashboard + L4 anchor **still works**
    38	   (RunExhaustedTx + EvidenceCapsule emitted on MAX_TX exhaust).
    39	5. The 6-holding `total_supply_micro` invariant + `assert_no_post_init_mint`
    40	   exhaustive match (now covering 14 typed-tx variants including the 3
    41	   TB-13) hold under live transitions.
    42	
    43	The 13-test integration suite (`tests/tb_13_complete_set.rs`) covers the
    44	TB-13-specific flows (mint / redeem / seed); this smoke covers the
    45	**regression surface** that those targeted tests cannot reach.
    46	
    47	## §2 Replay report (single run)
    48	
    49	```json
    50	{
    51	  "l4_entries": 3,
    52	  "l4e_entries": 2,
    53	  "ledger_root_verified": true,
    54	  "system_signatures_verified": true,
    55	  "state_reconstructed": true,
    56	  "economic_state_reconstructed": true,
    57	  "cas_payloads_retrievable": true,
    58	  "agent_signatures_verified": true,
    59	  "proposal_telemetry_cas_retrievable": true,
    60	  "run_id": "tb13-smoke",
    61	  "epoch": 1,
    62	  "detail": {
    63	    "final_state_root_hex": "1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9",
    64	    "final_ledger_root_hex": "93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb",
    65	    "head_commit_oid_hex": "38f1b3957834052aac42169598f92016d756c331",
    66	    "l4e_last_hash_hex": "79325795bf2ebc78a9330c06c173bb0c502ee283fbfa5b46f569551314e9e23a",
    67	    "replay_failure": null,
    68	    "initial_q_state_loaded_from_disk": true
    69	  }
    70	}
    71	```
    72	
    73	L4 = 3 entries (TaskOpen + EscrowLock + TerminalSummaryTx-on-MaxTxExhausted via
    74	TB-11 Atom 0.5(a) carry-forward); L4.E = 2 (rejected attempts; expected).
    75	
    76	## §3 EconomicState 13-sub-field round-trip
    77	
    78	Verified by direct introspection of the on-disk
    79	`runtime_repo/initial_q_state.json` after replay:
    80	
    81	```text
    82	EconomicState sub-fields: 13
    83	Sub-field names: [
    84	  'balances_t',
    85	  'challenge_cases_t',
    86	  'claims_t',
    87	  'conditional_collateral_t',          <-- TB-13 Atom 2 NEW
    88	  'conditional_share_balances_t',      <-- TB-13 Atom 2 NEW
    89	  'escrows_t',
    90	  'node_positions_t',
    91	  'price_index_t',
    92	  'reputations_t',
    93	  'royalty_graph_t',
    94	  'runs_t',
    95	  'stakes_t',
    96	  'task_markets_t',
    97	]
    98	```
    99	
   100	Both new fields default to empty maps under live workload (no TB-13
   101	typed-tx submitted by the LLM-driven solver path), but round-trip cleanly
   102	through `serde_json` and `canonical_encode` — the absence of regression
   103	on the 13-sub-field shape is the load-bearing claim.
   104	
   105	## §4 Dashboard render (§12 + §13)
   106	
   107	`dashboard.txt` excerpt (post-replay):
   108	
   109	```text
   110	§12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2; 2026-05-02)
   111	------------------------------------------------------------------------------
   112	  Exhausted runs (RunExhaustedTx ≡ TerminalSummaryTx):
   113	    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)
   114	    n1_mathd_alge… | task-n1_mathd_alg… | MaxTxExhausted  |       10 | d2b329ee554da3e2dea1d46ecca1bf1…
   115	
   116	§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
   117	------------------------------------------------------------------------------
   118	  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)
   119	```
   120	
   121	§14 (TB-13 conditional shares) — **NOT RENDERED**. Atom 4 dashboard work
   122	deferred to TB-14 PriceIndex (architect Part A spec made no dashboard
   123	requirement for TB-13). State observability available via direct QState
   124	introspection (`initial_q_state.json` shown in §3).
   125	
   126	## §5 Headline outcome table
   127	
   128	| Step  | Config                                          | Outcome   | TB-13 schema integrity |
   129	|-------|-------------------------------------------------|-----------|------------------------|
   130	| Single | n1 × `mathd_algebra_171` × MAX_TX=10           | UNSOLVED (hit_max_tx) | ✓ 13 sub-fields persist |
   131	
   132	UNSOLVED is expected for MAX_TX=10 on this problem (TB-8 historical: same
   133	problem solved at MAX_TX=10 single-run; deepseek-chat is drift-prone per
   134	`project_deepseek_drift_2026-04-24`). The smoke's load-bearing claim is
   135	schema integrity, not solve rate.
   136	
   137	## §6 Reproduction
   138	
   139	```bash
   140	SMOKE_DIR=/tmp/tb13_smoke_repro
   141	mkdir -p "$SMOKE_DIR"/{runtime_repo,cas}
   142	
   143	cd experiments/minif2f_v4
   144	TURINGOS_CHAINTAPE_PATH="$SMOKE_DIR/runtime_repo" \
   145	TURINGOS_CAS_PATH="$SMOKE_DIR/cas" \
   146	TURINGOS_CHAINTAPE_PRESEED=1 \
   147	TURINGOS_RUN_ID=tb13-smoke \
   148	LLM_PROXY_URL="http://localhost:8080/v1/chat/completions" \
   149	ACTIVE_MODEL=deepseek-chat \
   150	CONDITION=n1 \
   151	MAX_TRANSACTIONS=10 \
   152	../../target/debug/evaluator mathd_algebra_171.lean
   153	
   154	../../target/debug/audit_dashboard \
   155	  --repo "$SMOKE_DIR/runtime_repo" \
   156	  --cas "$SMOKE_DIR/cas" \
   157	  > "$SMOKE_DIR/dashboard.txt"
   158	
   159	../../target/debug/verify_chaintape \
   160	  --repo "$SMOKE_DIR/runtime_repo" \
   161	  --cas "$SMOKE_DIR/cas" \
   162	  --out "$SMOKE_DIR/replay_report.json"
   163	
   164	# Expected: 7/7 indicators GREEN; economic_state_reconstructed: true.
   165	python3 -c "
   166	import json
   167	q = json.load(open('$SMOKE_DIR/runtime_repo/initial_q_state.json'))
   168	es = q['economic_state_t']
   169	assert len(es) == 13, f'expected 13 sub-fields, got {len(es)}'
   170	assert 'conditional_collateral_t' in es
   171	assert 'conditional_share_balances_t' in es
   172	print('TB-13 schema OK: 13 sub-fields present')
   173	"
   174	```
   175	
   176	## §7 What this smoke does NOT validate
   177	
   178	- TB-13 mint / redeem / seed dispatch arms under load — those are exercised by
   179	  `tests/tb_13_complete_set.rs` integration tests (13 tests, all PASS).
   180	- Multi-problem variety / aggregate solve rate under TB-13 — out of scope
   181	  (regression smoke, not capability demo).
   182	- Cross-run identity / durable keystore reattachment — TB-9 territory.
   183	- Real Polymarket market interactions — those are TB-14+ scope.
   184	
   185	## §8 Cross-references
   186	
   187	- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
   188	- TB-13 recursive self-audit: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`
   189	- TB-13 architect ruling lossless: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
   190	- TB-13 integration tests: `tests/tb_13_complete_set.rs`
   191	- TB-12 smoke evidence (predecessor format): `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/`

exec
/bin/bash -lc "git diff --name-status fa36eca..HEAD -- src tests handover/tracer_bullets/TB-13_charter_2026-05-03.md handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
A	handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md
A	handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md
A	handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md
A	handover/tracer_bullets/TB-13_charter_2026-05-03.md
M	src/bottom_white/ledger/transition_ledger.rs
M	src/economy/monetary_invariant.rs
M	src/kernel.rs
M	src/prediction_market.rs
M	src/runtime/run_summary.rs
M	src/state/q_state.rs
M	src/state/sequencer.rs
M	src/state/typed_tx.rs
M	tests/economic_state_reconstruct.rs
M	tests/q_state_reconstruct.rs
M	tests/six_axioms_alignment.rs
A	tests/tb_13_complete_set.rs
A	tests/tb_13_legacy_cpmm_forward_fence.rs

exec
/bin/bash -lc 'rg --line-number "CompleteSet|MarketSeed|Conditional|conditional_|ResolutionRef|OutcomeSide|ShareAmount|InvalidResolutionRef|RedeemBeforeResolution|RedeemMoreThanOwned|InsufficientCollateral|InsufficientBalanceForMint|assert_complete_set_balanced|total_supply_micro|assert_total_ctf_conserved" src/state/typed_tx.rs src/state/q_state.rs src/state/sequencer.rs src/economy/monetary_invariant.rs src/state/mod.rs tests/tb_13_complete_set.rs tests/tb_13_legacy_cpmm_forward_fence.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_13_legacy_cpmm_forward_fence.rs:11://!   HALT if f64 appears in new CompleteSet / MarketSeed code.
tests/tb_13_legacy_cpmm_forward_fence.rs:199:/// CompleteSet / MarketSeed code. Money-path types must use integer
tests/tb_13_legacy_cpmm_forward_fence.rs:200:/// `MicroCoin` / `ShareAmount`.
tests/tb_13_legacy_cpmm_forward_fence.rs:260:        "CompleteSetMintTx",
tests/tb_13_complete_set.rs:1://! TB-13 Atom 5 integration tests — CompleteSet + MarketSeedTx per architect
tests/tb_13_complete_set.rs:4://! "CompleteSet + MarketSeedTx" — Polymarket / CTF conditional-share
tests/tb_13_complete_set.rs:12://! charter §3 Atom 5 (total_supply_micro mutation correctness / shares
tests/tb_13_complete_set.rs:13://! NOT counted as Coin / MarketSeed without provider balance / no
tests/tb_13_complete_set.rs:47:    assert_complete_set_balanced, assert_total_ctf_conserved,
tests/tb_13_complete_set.rs:50:    AgentId, ConditionalCollateralIndex, ConditionalShareBalances, QState,
tests/tb_13_complete_set.rs:55:    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId,
tests/tb_13_complete_set.rs:56:    MarketSeedTx, OutcomeSide, ResolutionRef, ShareAmount, TypedTx,
tests/tb_13_complete_set.rs:129:    TypedTx::CompleteSetMint(CompleteSetMintTx {
tests/tb_13_complete_set.rs:144:    outcome: OutcomeSide,
tests/tb_13_complete_set.rs:148:    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
tests/tb_13_complete_set.rs:154:        share_amount: ShareAmount::from_units(units),
tests/tb_13_complete_set.rs:155:        resolution_ref: ResolutionRef {
tests/tb_13_complete_set.rs:171:    TypedTx::MarketSeed(MarketSeedTx {
tests/tb_13_complete_set.rs:211:        .conditional_collateral_t
tests/tb_13_complete_set.rs:220:        .conditional_share_balances_t
tests/tb_13_complete_set.rs:236:    assert_total_ctf_conserved(
tests/tb_13_complete_set.rs:242:    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
tests/tb_13_complete_set.rs:249:/// Asserts that `assert_total_ctf_conserved` passes pre/post a mint that
tests/tb_13_complete_set.rs:265:    // do NOT contribute to total_supply_micro per CR-13.3.
tests/tb_13_complete_set.rs:266:    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
tests/tb_13_complete_set.rs:268:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
tests/tb_13_complete_set.rs:273:/// SG-13.3 — MarketSeedTx fails if provider lacks balance.
tests/tb_13_complete_set.rs:285:        err.contains("InsufficientBalanceForMint"),
tests/tb_13_complete_set.rs:286:        "expected InsufficientBalanceForMint, got: {err}"
tests/tb_13_complete_set.rs:292:/// SG-13.4 — MarketSeedTx cannot create liquidity without collateral
tests/tb_13_complete_set.rs:300:    // collateral_amount == 0 must fail with InsufficientCollateral.
tests/tb_13_complete_set.rs:305:        err.contains("InsufficientCollateral"),
tests/tb_13_complete_set.rs:306:        "expected InsufficientCollateral, got: {err}"
tests/tb_13_complete_set.rs:315:/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
tests/tb_13_complete_set.rs:331:        build_redeem(parent, "alice", "task-O", OutcomeSide::Yes, 1_000_000, 6),
tests/tb_13_complete_set.rs:336:        err.contains("RedeemBeforeResolution"),
tests/tb_13_complete_set.rs:337:        "expected RedeemBeforeResolution, got: {err}"
tests/tb_13_complete_set.rs:351:        build_redeem(parent, "bob", "task-E", OutcomeSide::No, 500_000, 8),
tests/tb_13_complete_set.rs:356:        err.contains("RedeemBeforeResolution"),
tests/tb_13_complete_set.rs:357:        "expected RedeemBeforeResolution on Expired state, got: {err}"
tests/tb_13_complete_set.rs:378:        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 4_000_000, 10),
tests/tb_13_complete_set.rs:400:        .conditional_share_balances_t
tests/tb_13_complete_set.rs:414:        build_redeem(parent, "alice", "task-Y", OutcomeSide::No, 1_000_000, 11),
tests/tb_13_complete_set.rs:419:        err.contains("InvalidResolutionRef"),
tests/tb_13_complete_set.rs:420:        "expected InvalidResolutionRef, got: {err}"
tests/tb_13_complete_set.rs:435:        build_redeem(parent, "bob", "task-B", OutcomeSide::Yes, 500_000, 13),
tests/tb_13_complete_set.rs:439:    assert!(err.contains("InvalidResolutionRef"));
tests/tb_13_complete_set.rs:443:        build_redeem(parent, "bob", "task-B", OutcomeSide::No, 500_000, 14),
tests/tb_13_complete_set.rs:451:// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
tests/tb_13_complete_set.rs:496:/// Halt: total_supply_micro must be unchanged across mint+redeem.
tests/tb_13_complete_set.rs:498:async fn halt_total_supply_micro_unchanged_across_mint_redeem() {
tests/tb_13_complete_set.rs:510:        build_redeem(parent, "alice", "task-H1", OutcomeSide::Yes, 7_000_000, 21),
tests/tb_13_complete_set.rs:516:    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
tests/tb_13_complete_set.rs:517:        .expect("total_supply_micro bit-equal across mint+redeem");
tests/tb_13_complete_set.rs:518:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
tests/tb_13_complete_set.rs:531:    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
tests/tb_13_complete_set.rs:535:/// Halt: MarketSeed with zero-balance provider rejected (regression
tests/tb_13_complete_set.rs:545:    assert!(err.contains("InsufficientBalanceForMint"));
tests/tb_13_complete_set.rs:549:/// rejected with `RedeemMoreThanOwned`.
tests/tb_13_complete_set.rs:562:        build_redeem(parent, "alice", "task-H4", OutcomeSide::Yes, 5_000_000, 25),
tests/tb_13_complete_set.rs:567:        err.contains("RedeemMoreThanOwned"),
tests/tb_13_complete_set.rs:568:        "expected RedeemMoreThanOwned, got: {err}"
tests/tb_13_complete_set.rs:582:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");
tests/tb_13_complete_set.rs:585:        .conditional_collateral_t
tests/tb_13_complete_set.rs:593:        .conditional_share_balances_t
tests/tb_13_complete_set.rs:607:    let _ = ConditionalCollateralIndex::default();
tests/tb_13_complete_set.rs:608:    let _ = ConditionalShareBalances::default();
src/economy/monetary_invariant.rs:42:    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
src/economy/monetary_invariant.rs:74:    /// either a bug in CompleteSetMint / CompleteSetRedeem / MarketSeed
src/economy/monetary_invariant.rs:77:    CompleteSetUnbalanced {
src/economy/monetary_invariant.rs:79:        side: crate::state::typed_tx::OutcomeSide,
src/economy/monetary_invariant.rs:113:            Self::CompleteSetUnbalanced { event_id_hex, side, share_sum_units, collateral_units } => {
src/economy/monetary_invariant.rs:172:fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
src/economy/monetary_invariant.rs:193:    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
src/economy/monetary_invariant.rs:195:    // 5-holding sum to 6. Without this, CompleteSetMintTx (which migrates
src/economy/monetary_invariant.rs:196:    // Coin from balances_t to conditional_collateral_t) would falsely
src/economy/monetary_invariant.rs:197:    // appear to burn money, failing assert_total_ctf_conserved with empty
src/economy/monetary_invariant.rs:200:    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
src/economy/monetary_invariant.rs:201:    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
src/economy/monetary_invariant.rs:204:    for c in s.conditional_collateral_t.0.values() {
src/economy/monetary_invariant.rs:321:/// [`assert_total_ctf_conserved`] separately.
src/economy/monetary_invariant.rs:337:        // conservation via assert_total_ctf_conserved with empty exempt list.
src/economy/monetary_invariant.rs:344:        // CTF conservation enforced by assert_total_ctf_conserved with
src/economy/monetary_invariant.rs:350:        // CTF conservation enforced by assert_total_ctf_conserved with
src/economy/monetary_invariant.rs:354:        // CR-13.1..6): CompleteSetMint / CompleteSetRedeem / MarketSeed
src/economy/monetary_invariant.rs:358:        // credits collateral 1:1. Conditional shares are claims, NOT Coin
src/economy/monetary_invariant.rs:360:        // assert_total_ctf_conserved with conditional_collateral_t as the
src/economy/monetary_invariant.rs:362:        | TypedTx::CompleteSetMint(_)
src/economy/monetary_invariant.rs:363:        | TypedTx::CompleteSetRedeem(_)
src/economy/monetary_invariant.rs:364:        | TypedTx::MarketSeed(_) => Ok(()),
src/economy/monetary_invariant.rs:369:// assert_total_ctf_conserved — numeric conservation across a transition
src/economy/monetary_invariant.rs:385:pub fn assert_total_ctf_conserved(
src/economy/monetary_invariant.rs:390:    let total_before = total_supply_micro(before)?;
src/economy/monetary_invariant.rs:391:    let total_after = total_supply_micro(after)?;
src/economy/monetary_invariant.rs:428:// TB-13 Atom 3 — assert_complete_set_balanced (architect 2026-05-03 post-
src/economy/monetary_invariant.rs:435:/// For every event in `conditional_collateral_t`:
src/economy/monetary_invariant.rs:457:pub fn assert_complete_set_balanced(
src/economy/monetary_invariant.rs:460:    use crate::state::typed_tx::OutcomeSide;
src/economy/monetary_invariant.rs:461:    for (event_id, collateral) in s.conditional_collateral_t.0.iter() {
src/economy/monetary_invariant.rs:465:        for owner_map in s.conditional_share_balances_t.0.values() {
src/economy/monetary_invariant.rs:480:                (OutcomeSide::Yes, sum_yes)
src/economy/monetary_invariant.rs:482:                (OutcomeSide::No, sum_no)
src/economy/monetary_invariant.rs:484:            return Err(MonetaryError::CompleteSetUnbalanced {
src/economy/monetary_invariant.rs:579:    // ── assert_total_ctf_conserved ──────────────────────────────────────────
src/economy/monetary_invariant.rs:589:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:598:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:610:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:623:            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
src/economy/monetary_invariant.rs:639:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:652:        let total0 = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:673:            // total_supply_micro unchanged.)
src/economy/monetary_invariant.rs:689:            let total_now = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:697:        assert_eq!(total_supply_micro(&s).unwrap(), total0);
src/economy/monetary_invariant.rs:748:        assert_eq!(total_supply_micro(&s).unwrap(), 55 * MICRO_PER_COIN);
src/economy/monetary_invariant.rs:756:        // must yield total_supply_micro = K, NOT 2K. If a regression adds
src/economy/monetary_invariant.rs:775:            total_supply_micro(&s).unwrap(),
src/state/sequencer.rs:39:    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
src/state/sequencer.rs:255:/// §4.3): CompleteSetMint-accept state-root domain.
src/state/sequencer.rs:259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
src/state/sequencer.rs:270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
src/state/sequencer.rs:275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
src/state/sequencer.rs:286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
src/state/sequencer.rs:291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
src/state/sequencer.rs:447:        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
src/state/sequencer.rs:455:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:456:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:457:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:478:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:479:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:480:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:506:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:507:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:508:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:656:            assert_total_ctf_conserved(
src/state/sequencer.rs:804:            assert_total_ctf_conserved(
src/state/sequencer.rs:914:            assert_total_ctf_conserved(
src/state/sequencer.rs:1122:            assert_total_ctf_conserved(
src/state/sequencer.rs:1266:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1313:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1363:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1444:            assert_total_ctf_conserved(
src/state/sequencer.rs:1490:            assert_total_ctf_conserved(
src/state/sequencer.rs:1552:            assert_total_ctf_conserved(
src/state/sequencer.rs:1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
src/state/sequencer.rs:1577:        // conditional_collateral_t[event_id] by amount; credits BOTH
src/state/sequencer.rs:1578:        // conditional_share_balances_t[owner][event][Yes] and [No] by
src/state/sequencer.rs:1582:        TypedTx::CompleteSetMint(mint) => {
src/state/sequencer.rs:1589:                return Err(TransitionError::InsufficientBalanceForMint);
src/state/sequencer.rs:1600:                return Err(TransitionError::InsufficientBalanceForMint);
src/state/sequencer.rs:1604:            // monetary_invariant extension) treats conditional_collateral_t
src/state/sequencer.rs:1605:            // as a Coin holding, so total_supply_micro is preserved
src/state/sequencer.rs:1615:                .conditional_collateral_t
src/state/sequencer.rs:1624:                .conditional_share_balances_t
src/state/sequencer.rs:1631:            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1634:            pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1641:            assert_total_ctf_conserved(
src/state/sequencer.rs:1654:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
src/state/sequencer.rs:1659:        //     Bankrupt (No); else RedeemBeforeResolution.
src/state/sequencer.rs:1660:        //   - claimed_outcome must match the state; else InvalidResolutionRef.
src/state/sequencer.rs:1662:        //     else RedeemMoreThanOwned.
src/state/sequencer.rs:1664:        //     InsufficientCollateral.
src/state/sequencer.rs:1669:        TypedTx::CompleteSetRedeem(redeem) => {
src/state/sequencer.rs:1676:                return Err(TransitionError::InvalidResolutionRef);
src/state/sequencer.rs:1685:                .ok_or(TransitionError::RedeemBeforeResolution)?;
src/state/sequencer.rs:1688:                 crate::state::typed_tx::OutcomeSide::Yes) => { /* ok — YES wins */ }
src/state/sequencer.rs:1690:                 crate::state::typed_tx::OutcomeSide::No) => { /* ok — NO wins */ }
src/state/sequencer.rs:1693:                    return Err(TransitionError::InvalidResolutionRef);
src/state/sequencer.rs:1697:                    return Err(TransitionError::RedeemBeforeResolution);
src/state/sequencer.rs:1703:                .conditional_share_balances_t
src/state/sequencer.rs:1710:                crate::state::typed_tx::OutcomeSide::Yes => pair.yes.units,
src/state/sequencer.rs:1711:                crate::state::typed_tx::OutcomeSide::No => pair.no.units,
src/state/sequencer.rs:1714:                return Err(TransitionError::RedeemMoreThanOwned);
src/state/sequencer.rs:1717:            // assert_complete_set_balanced is preserved).
src/state/sequencer.rs:1720:                .conditional_collateral_t
src/state/sequencer.rs:1726:                return Err(TransitionError::InsufficientCollateral);
src/state/sequencer.rs:1735:                    .conditional_share_balances_t
src/state/sequencer.rs:1743:                    crate::state::typed_tx::OutcomeSide::Yes => {
src/state/sequencer.rs:1744:                        pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1748:                    crate::state::typed_tx::OutcomeSide::No => {
src/state/sequencer.rs:1749:                        pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1759:                    .conditional_collateral_t
src/state/sequencer.rs:1785:            assert_total_ctf_conserved(
src/state/sequencer.rs:1798:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
src/state/sequencer.rs:1802:        TypedTx::MarketSeed(seed) => {
src/state/sequencer.rs:1808:                return Err(TransitionError::InsufficientCollateral);
src/state/sequencer.rs:1819:                return Err(TransitionError::InsufficientBalanceForMint);
src/state/sequencer.rs:1832:                .conditional_collateral_t
src/state/sequencer.rs:1841:                .conditional_share_balances_t
src/state/sequencer.rs:1848:            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1851:            pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1858:            assert_total_ctf_conserved(
src/state/sequencer.rs:2335:            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
src/state/sequencer.rs:2343:            | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:2344:            | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:2345:            | TypedTx::MarketSeed(_) => {}
src/state/q_state.rs:179:    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
src/state/q_state.rs:198:    /// holding"); included in the 6-holding `total_supply_micro` sum
src/state/q_state.rs:201:    /// `Σ_{event} conditional_collateral_t[event].units == Σ shares per side`.
src/state/q_state.rs:205:    pub conditional_collateral_t: ConditionalCollateralIndex,
src/state/q_state.rs:207:    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
src/state/q_state.rs:210:    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
src/state/q_state.rs:211:    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
src/state/q_state.rs:217:    pub conditional_share_balances_t: ConditionalShareBalances,
src/state/q_state.rs:391:///   <this task>`. `monetary_invariant::total_supply_micro` does NOT include
src/state/q_state.rs:522:// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
src/state/q_state.rs:529:/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
src/state/q_state.rs:530:/// at `monetary_invariant::assert_total_ctf_conserved`. Mint/seed credit
src/state/q_state.rs:532:/// (`assert_complete_set_balanced`) enforces
src/state/q_state.rs:535:pub struct ConditionalCollateralIndex(
src/state/q_state.rs:543:/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
src/state/q_state.rs:544:/// explicit: shares are NOT counted in `total_supply_micro`.
src/state/q_state.rs:552:pub struct ConditionalShareBalances(
src/state/q_state.rs:564:    pub yes: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:565:    pub no: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:779:        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
src/state/q_state.rs:780:        // holding, included in 6-holding total_supply_micro) +
src/state/q_state.rs:781:        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
src/state/q_state.rs:782:        // total_supply_micro).
src/state/q_state.rs:789:            "EconomicState must have 13 sub-fields post-TB-13 (was 11 post-TB-12; +conditional_collateral_t +conditional_share_balances_t); got {}",
src/state/q_state.rs:794:        assert!(obj.contains_key("conditional_collateral_t"), "TB-13 conditional_collateral_t sub-field missing");
src/state/q_state.rs:795:        assert!(obj.contains_key("conditional_share_balances_t"), "TB-13 conditional_share_balances_t sub-field missing");
src/state/typed_tx.rs:596:// those land in TB-13 (CompleteSet) + TB-14 (PriceIndex) + TB-16
src/state/typed_tx.rs:603://   No CompleteSet / MarketSeedTx / AMM / CPMM (TB-13/14 territory).
src/state/typed_tx.rs:660:///   `total_supply_micro`; CR-12.2).
src/state/typed_tx.rs:662:/// - NOT a YES/NO claim (TB-13 CompleteSet territory).
src/state/typed_tx.rs:828:// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
src/state/typed_tx.rs:1057:// § 5c-TB-13 — CompleteSet + MarketSeedTx conditional shares
src/state/typed_tx.rs:1063:// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
src/state/typed_tx.rs:1064:// mints equal YES_E + NO_E shares to the same owner. `CompleteSetRedeemTx`
src/state/typed_tx.rs:1066:// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
src/state/typed_tx.rs:1087:pub enum OutcomeSide {
src/state/typed_tx.rs:1092:impl Default for OutcomeSide {
src/state/typed_tx.rs:1103:/// `RedeemMoreThanOwned` rejection, not a representation concern.
src/state/typed_tx.rs:1105:pub struct ShareAmount {
src/state/typed_tx.rs:1109:impl ShareAmount {
src/state/typed_tx.rs:1115:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): build a `ShareAmount`
src/state/typed_tx.rs:1125:/// embedded in `CompleteSetRedeemTx`. References either a
src/state/typed_tx.rs:1126:/// `TaskBankruptcyTx` (outcome must be `OutcomeSide::No`) or a
src/state/typed_tx.rs:1127:/// `FinalizeRewardTx` (outcome must be `OutcomeSide::Yes`) for the
src/state/typed_tx.rs:1132:pub struct ResolutionRef {
src/state/typed_tx.rs:1134:    pub claimed_outcome: OutcomeSide,
src/state/typed_tx.rs:1141:/// 1. `balances_t[owner] >= amount` else `InsufficientBalanceForMint`.
src/state/typed_tx.rs:1143:/// 3. `conditional_collateral_t[event_id] += amount`.
src/state/typed_tx.rs:1144:/// 4. `conditional_share_balances_t[(owner, event_id, Yes)] += amount.units`.
src/state/typed_tx.rs:1145:/// 5. `conditional_share_balances_t[(owner, event_id, No)]  += amount.units`.
src/state/typed_tx.rs:1150:pub struct CompleteSetMintTx {
src/state/typed_tx.rs:1166:///      `No` else `InvalidResolutionRef`.
src/state/typed_tx.rs:1168:///      `Yes` else `InvalidResolutionRef`.
src/state/typed_tx.rs:1169:///    - Else: `RedeemBeforeResolution` (no acceptable resolution).
src/state/typed_tx.rs:1170:/// 2. `conditional_share_balances_t[(owner, event_id, outcome)] >= share_amount.units`
src/state/typed_tx.rs:1171:///    else `RedeemMoreThanOwned`.
src/state/typed_tx.rs:1172:/// 3. `conditional_collateral_t[event_id] >= share_amount.units` else
src/state/typed_tx.rs:1173:///    `InsufficientCollateral` (defensive; should never fire if
src/state/typed_tx.rs:1174:///    `assert_complete_set_balanced` holds).
src/state/typed_tx.rs:1177:pub struct CompleteSetRedeemTx {
src/state/typed_tx.rs:1182:    pub outcome: OutcomeSide,                 //  5
src/state/typed_tx.rs:1183:    pub share_amount: ShareAmount,            //  6
src/state/typed_tx.rs:1184:    pub resolution_ref: ResolutionRef,        //  7
src/state/typed_tx.rs:1194:/// 1. `collateral_amount > 0` else `InsufficientCollateral` (SG-13.4).
src/state/typed_tx.rs:1196:///    `InsufficientBalanceForMint` (SG-13.3).
src/state/typed_tx.rs:1198:/// 4. `conditional_collateral_t[event_id] += collateral_amount`.
src/state/typed_tx.rs:1200:///    `conditional_share_balances_t[(provider, event_id, Yes)] += collateral_amount.units`
src/state/typed_tx.rs:1201:///    `conditional_share_balances_t[(provider, event_id, No)]  += collateral_amount.units`.
src/state/typed_tx.rs:1203:/// The shape is identical to `CompleteSetMintTx` post-effect; the
src/state/typed_tx.rs:1209:pub struct MarketSeedTx {
src/state/typed_tx.rs:1222:/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1224:pub struct CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1233:impl CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1235:    /// canonical digest for agent-signed CompleteSetMintTx. Domain
src/state/typed_tx.rs:1245:/// `CompleteSetRedeemTx` (9 fields → 8 fields; signature excluded).
src/state/typed_tx.rs:1247:pub struct CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1252:    pub outcome: OutcomeSide,
src/state/typed_tx.rs:1253:    pub share_amount: ShareAmount,
src/state/typed_tx.rs:1254:    pub resolution_ref: ResolutionRef,
src/state/typed_tx.rs:1258:impl CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1260:    /// canonical digest for agent-signed CompleteSetRedeemTx. Domain
src/state/typed_tx.rs:1268:/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1270:pub struct MarketSeedSigningPayload {
src/state/typed_tx.rs:1279:impl MarketSeedSigningPayload {
src/state/typed_tx.rs:1281:    /// canonical digest for agent-signed MarketSeedTx. Domain prefix
src/state/typed_tx.rs:1448:impl CompleteSetMintTx {
src/state/typed_tx.rs:1451:    pub fn to_signing_payload(&self) -> CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1452:        CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1463:impl CompleteSetRedeemTx {
src/state/typed_tx.rs:1466:    pub fn to_signing_payload(&self) -> CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1467:        CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1480:impl MarketSeedTx {
src/state/typed_tx.rs:1483:    pub fn to_signing_payload(&self) -> MarketSeedSigningPayload {
src/state/typed_tx.rs:1484:        MarketSeedSigningPayload {
src/state/typed_tx.rs:1520:    CompleteSetMint(CompleteSetMintTx),   // TB-13 agent-signed conditional-share mint
src/state/typed_tx.rs:1521:    CompleteSetRedeem(CompleteSetRedeemTx), // TB-13 agent-signed conditional-share redeem
src/state/typed_tx.rs:1522:    MarketSeed(MarketSeedTx),             // TB-13 agent-signed protocol-owned share seed
src/state/typed_tx.rs:1541:            Self::CompleteSetMint(_) => TxKind::CompleteSetMint,
src/state/typed_tx.rs:1542:            Self::CompleteSetRedeem(_) => TxKind::CompleteSetRedeem,
src/state/typed_tx.rs:1543:            Self::MarketSeed(_) => TxKind::MarketSeed,
src/state/typed_tx.rs:1628:impl HasSubmitter for CompleteSetMintTx {
src/state/typed_tx.rs:1634:impl HasSubmitter for CompleteSetRedeemTx {
src/state/typed_tx.rs:1640:impl HasSubmitter for MarketSeedTx {
src/state/typed_tx.rs:1660:            Self::CompleteSetMint(t) => t.submitter_id(),
src/state/typed_tx.rs:1661:            Self::CompleteSetRedeem(t) => t.submitter_id(),
src/state/typed_tx.rs:1662:            Self::MarketSeed(t) => t.submitter_id(),
src/state/typed_tx.rs:1763:    /// `assert_total_ctf_conserved` failed on the WorkTx arm. Maps to
src/state/typed_tx.rs:1849:    /// `CompleteSetMintTx` admission: `balances_t[owner] < amount`.
src/state/typed_tx.rs:1852:    InsufficientBalanceForMint,
src/state/typed_tx.rs:1853:    /// `CompleteSetRedeemTx` admission: the referenced event is in
src/state/typed_tx.rs:1858:    RedeemBeforeResolution,
src/state/typed_tx.rs:1859:    /// `CompleteSetRedeemTx` admission: the owner's
src/state/typed_tx.rs:1860:    /// `conditional_share_balances_t[owner][event_id].{yes|no}` is less
src/state/typed_tx.rs:1863:    RedeemMoreThanOwned,
src/state/typed_tx.rs:1864:    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
src/state/typed_tx.rs:1866:    /// collateral. Also fired defensively at `CompleteSetRedeemTx` time
src/state/typed_tx.rs:1867:    /// if `conditional_collateral_t[event_id]` lacks the redeemed amount
src/state/typed_tx.rs:1868:    /// (should never happen if `assert_complete_set_balanced` holds).
src/state/typed_tx.rs:1870:    InsufficientCollateral,
src/state/typed_tx.rs:1871:    /// `CompleteSetRedeemTx` admission: the resolution_ref's
src/state/typed_tx.rs:1877:    InvalidResolutionRef,
src/state/typed_tx.rs:1943:            Self::InsufficientBalanceForMint => write!(
src/state/typed_tx.rs:1945:                "CompleteSetMintTx: owner's balances_t entry is below the requested mint amount"
src/state/typed_tx.rs:1947:            Self::RedeemBeforeResolution => write!(
src/state/typed_tx.rs:1949:                "CompleteSetRedeemTx: event task_markets_t state is Open or Expired (no system-emitted resolution yet)"
src/state/typed_tx.rs:1951:            Self::RedeemMoreThanOwned => write!(
src/state/typed_tx.rs:1953:                "CompleteSetRedeemTx: owner's conditional share balance is below the requested redeem amount"
src/state/typed_tx.rs:1955:            Self::InsufficientCollateral => write!(
src/state/typed_tx.rs:1957:                "TB-13 collateral missing: MarketSeed with zero collateral, or Redeem against insufficient conditional_collateral_t"
src/state/typed_tx.rs:1959:            Self::InvalidResolutionRef => write!(
src/state/typed_tx.rs:1961:                "CompleteSetRedeemTx: resolution_ref.claimed_outcome does not match task_markets_t[event_id.0] state"
src/state/typed_tx.rs:3099:    // TB-13 Atom 1 unit tests — CompleteSetMint / CompleteSetRedeem /
src/state/typed_tx.rs:3100:    // MarketSeed (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
src/state/typed_tx.rs:3103:    fn fixture_complete_set_mint_tx() -> CompleteSetMintTx {
src/state/typed_tx.rs:3104:        CompleteSetMintTx {
src/state/typed_tx.rs:3115:    fn fixture_complete_set_redeem_tx() -> CompleteSetRedeemTx {
src/state/typed_tx.rs:3116:        CompleteSetRedeemTx {
src/state/typed_tx.rs:3121:            outcome: OutcomeSide::Yes,
src/state/typed_tx.rs:3122:            share_amount: ShareAmount::from_units(7_000_000),
src/state/typed_tx.rs:3123:            resolution_ref: ResolutionRef {
src/state/typed_tx.rs:3125:                claimed_outcome: OutcomeSide::Yes,
src/state/typed_tx.rs:3132:    fn fixture_market_seed_tx() -> MarketSeedTx {
src/state/typed_tx.rs:3133:        MarketSeedTx {
src/state/typed_tx.rs:3144:    /// TB-13 U1: CompleteSetMintTx round-trips through canonical encode.
src/state/typed_tx.rs:3147:        let tx = TypedTx::CompleteSetMint(fixture_complete_set_mint_tx());
src/state/typed_tx.rs:3150:        assert_eq!(tx, decoded, "CompleteSetMintTx round-trip mismatch");
src/state/typed_tx.rs:3153:            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetMint,
src/state/typed_tx.rs:3157:    /// TB-13 U2: CompleteSetRedeemTx round-trips through canonical encode.
src/state/typed_tx.rs:3160:        let tx = TypedTx::CompleteSetRedeem(fixture_complete_set_redeem_tx());
src/state/typed_tx.rs:3163:        assert_eq!(tx, decoded, "CompleteSetRedeemTx round-trip mismatch");
src/state/typed_tx.rs:3166:            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetRedeem,
src/state/typed_tx.rs:3170:    /// TB-13 U3: MarketSeedTx round-trips through canonical encode.
src/state/typed_tx.rs:3173:        let tx = TypedTx::MarketSeed(fixture_market_seed_tx());
src/state/typed_tx.rs:3176:        assert_eq!(tx, decoded, "MarketSeedTx round-trip mismatch");
src/state/typed_tx.rs:3179:            crate::bottom_white::ledger::transition_ledger::TxKind::MarketSeed,
src/state/typed_tx.rs:3183:    /// TB-13 U4: OutcomeSide repr discriminants stable.
src/state/typed_tx.rs:3186:        assert_eq!(OutcomeSide::Yes as u8, 0);
src/state/typed_tx.rs:3187:        assert_eq!(OutcomeSide::No as u8, 1);
src/state/typed_tx.rs:3190:    /// TB-13 U5: ShareAmount default is zero.
src/state/typed_tx.rs:3193:        assert_eq!(ShareAmount::default(), ShareAmount::zero());
src/state/typed_tx.rs:3194:        assert_eq!(ShareAmount::default().units, 0u128);
src/state/typed_tx.rs:3204:        assert_eq!(mint_a, mint_b, "CompleteSetMint digest must be deterministic");
src/state/typed_tx.rs:3208:        assert_eq!(redeem_a, redeem_b, "CompleteSetRedeem digest must be deterministic");
src/state/typed_tx.rs:3212:        assert_eq!(seed_a, seed_b, "MarketSeed digest must be deterministic");
src/state/typed_tx.rs:3222:        assert_eq!(mint_o.len(), 6, "CompleteSetMintSigningPayload must have 6 fields");
src/state/typed_tx.rs:3228:        assert_eq!(redeem_o.len(), 8, "CompleteSetRedeemSigningPayload must have 8 fields");
src/state/typed_tx.rs:3234:        assert_eq!(seed_o.len(), 6, "MarketSeedSigningPayload must have 6 fields");
src/state/typed_tx.rs:3252:            TypedTx::CompleteSetMint(fixture_complete_set_mint_tx()).submitter_id(),
src/state/typed_tx.rs:3256:            TypedTx::MarketSeed(fixture_market_seed_tx()).submitter_id(),

exec
/bin/bash -lc "git diff --unified=0 fa36eca..HEAD -- src/state/sequencer.rs src/economy/monetary_invariant.rs src/state/typed_tx.rs src/state/q_state.rs tests/tb_13_legacy_cpmm_forward_fence.rs tests/tb_13_complete_set.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
diff --git a/src/economy/monetary_invariant.rs b/src/economy/monetary_invariant.rs
index e6607f1..f181982 100644
--- a/src/economy/monetary_invariant.rs
+++ b/src/economy/monetary_invariant.rs
@@ -69,0 +70,13 @@ pub enum MonetaryError {
+    /// **TB-13 complete-set balanced invariant violation**: for some
+    /// `event_id`, the sum of YES (or NO) shares across all owners does
+    /// not equal the locked collateral. Per architect §4.3 + SG-13.1:
+    /// 1 Coin → 1 YES_E + 1 NO_E mathematical identity. A drift signals
+    /// either a bug in CompleteSetMint / CompleteSetRedeem / MarketSeed
+    /// dispatch arms, or direct `EconomicState` mutation outside an
+    /// accepted transition (ghost share attempt).
+    CompleteSetUnbalanced {
+        event_id_hex: String,
+        side: crate::state::typed_tx::OutcomeSide,
+        share_sum_units: u128,
+        collateral_units: u128,
+    },
@@ -99,0 +113,7 @@ impl std::fmt::Display for MonetaryError {
+            Self::CompleteSetUnbalanced { event_id_hex, side, share_sum_units, collateral_units } => {
+                write!(
+                    f,
+                    "complete-set unbalanced: event_id={} side={:?} Σ shares={} != collateral_units={}",
+                    event_id_hex, side, share_sum_units, collateral_units
+                )
+            }
@@ -171,0 +192,15 @@ fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
+    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
+    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
+    // held against outstanding YES_E + NO_E share inventory. Extends the
+    // 5-holding sum to 6. Without this, CompleteSetMintTx (which migrates
+    // Coin from balances_t to conditional_collateral_t) would falsely
+    // appear to burn money, failing assert_total_ctf_conserved with empty
+    // exempt list.
+    //
+    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
+    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
+    // a holding. Counting them would triple-count (shares are derived from
+    // collateral; including both creates a 2x parallel ledger).
+    for c in s.conditional_collateral_t.0.values() {
+        total = total.checked_add(c.micro_units()).ok_or(MonetaryError::Overflow)?;
+    }
@@ -317 +352,13 @@ pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), Monetary
-        | TypedTx::TaskBankruptcy(_) => Ok(()),
+        | TypedTx::TaskBankruptcy(_)
+        // TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
+        // CR-13.1..6): CompleteSetMint / CompleteSetRedeem / MarketSeed
+        // are balance ↔ collateral migrations only. Mint debits balance
+        // and credits collateral 1:1; redeem debits collateral and shares
+        // and credits balance 1:1; seed debits provider balance and
+        // credits collateral 1:1. Conditional shares are claims, NOT Coin
+        // (CR-13.3 + SG-13.2). No mint, no burn — Atom 3 extends
+        // assert_total_ctf_conserved with conditional_collateral_t as the
+        // 6th Coin holding.
+        | TypedTx::CompleteSetMint(_)
+        | TypedTx::CompleteSetRedeem(_)
+        | TypedTx::MarketSeed(_) => Ok(()),
@@ -379,0 +427,72 @@ pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryErro
+// ────────────────────────────────────────────────────────────────────────────
+// TB-13 Atom 3 — assert_complete_set_balanced (architect 2026-05-03 post-
+// TB-12 ruling Part A §4.4 SG-13.1 + §4.5 CR-13.3..4)
+// ────────────────────────────────────────────────────────────────────────────
+
+/// TRACE_MATRIX TB-13 Atom 3 (architect §4.3 + SG-13.1): the
+/// **complete-set balanced** invariant.
+///
+/// For every event in `conditional_collateral_t`:
+///
+/// ```text
+/// min(Σ_{owner} share[(owner, event, Yes)], Σ_{owner} share[(owner, event, No)])
+///   == collateral[event].micro_units()
+/// ```
+///
+/// Why MIN, not equality on both sides:
+/// - Pre-resolution (mint + seed only): both sides equal collateral, so
+///   `min == collateral` is trivially equivalent to `Yes == No == collateral`.
+/// - Post-resolution + partial redeem: the winning side decreases by the
+///   redeemed amount AND collateral decreases by the same amount; the
+///   losing side stays the same (its shares are stranded zero-value
+///   claims). So `winning_side == collateral` still holds, while
+///   `losing_side > collateral` (losing side has surplus). MIN picks
+///   the winning side and matches collateral.
+/// - Post-resolution + full redeem: winning side is 0, collateral is 0,
+///   losing side is the original mint amount. MIN(0, original) = 0 = collateral.
+///
+/// This is the mathematical core of "1 Coin = 1 YES_E + 1 NO_E" enforced
+/// at the QState level: every Coin in collateral can be redeemed by the
+/// winning side, and no winning-side share is unbacked.
+pub fn assert_complete_set_balanced(
+    s: &EconomicState,
+) -> Result<(), MonetaryError> {
+    use crate::state::typed_tx::OutcomeSide;
+    for (event_id, collateral) in s.conditional_collateral_t.0.iter() {
+        let collateral_units: u128 = collateral.micro_units() as u128;
+        let mut sum_yes: u128 = 0;
+        let mut sum_no: u128 = 0;
+        for owner_map in s.conditional_share_balances_t.0.values() {
+            if let Some(pair) = owner_map.get(event_id) {
+                sum_yes = sum_yes
+                    .checked_add(pair.yes.units)
+                    .ok_or(MonetaryError::Overflow)?;
+                sum_no = sum_no
+                    .checked_add(pair.no.units)
+                    .ok_or(MonetaryError::Overflow)?;
+            }
+        }
+        let min_side = sum_yes.min(sum_no);
+        if min_side != collateral_units {
+            // Report the failing side (the smaller one) for diagnostic
+            // clarity — that's where the equality-with-collateral broke.
+            let (side, share_sum_units) = if sum_yes <= sum_no {
+                (OutcomeSide::Yes, sum_yes)
+            } else {
+                (OutcomeSide::No, sum_no)
+            };
+            return Err(MonetaryError::CompleteSetUnbalanced {
+                event_id_hex: hex_event_id(event_id),
+                side,
+                share_sum_units,
+                collateral_units,
+            });
+        }
+    }
+    Ok(())
+}
+
+fn hex_event_id(event_id: &crate::state::typed_tx::EventId) -> String {
+    event_id.0 .0.clone()
+}
+
diff --git a/src/state/q_state.rs b/src/state/q_state.rs
index b579a02..f9b6412 100644
--- a/src/state/q_state.rs
+++ b/src/state/q_state.rs
@@ -192,0 +193,25 @@ pub struct EconomicState {
+    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
+    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
+    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
+    ///
+    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
+    /// holding"); included in the 6-holding `total_supply_micro` sum
+    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
+    /// 1 Coin → 1 YES_E + 1 NO_E mathematical identity (SG-13.1) ensures
+    /// `Σ_{event} conditional_collateral_t[event].units == Σ shares per side`.
+    ///
+    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
+    #[serde(default)]
+    pub conditional_collateral_t: ConditionalCollateralIndex,
+    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
+    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
+    ///
+    /// **IS NOT** a Coin holding — shares are CLAIMS against
+    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
+    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
+    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
+    /// winning side at 1 share = 1 MicroCoin against collateral.
+    ///
+    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
+    #[serde(default)]
+    pub conditional_share_balances_t: ConditionalShareBalances,
@@ -494,0 +520,48 @@ pub struct NodePositionsIndex(
+// ────────────────────────────────────────────────────────────────────────────
+// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
+// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
+// conditional-share substrate. **1 locked Coin = 1 YES_E + 1 NO_E.**
+// ────────────────────────────────────────────────────────────────────────────
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.4): per-event Coin
+/// collateral locked against outstanding YES_E + NO_E share inventory.
+///
+/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
+/// at `monetary_invariant::assert_total_ctf_conserved`. Mint/seed credit
+/// this map; redeem debits it. The complete-set balanced invariant
+/// (`assert_complete_set_balanced`) enforces
+/// `Σ_{owner} share[(owner, event, Yes)] == Σ_{owner} share[(owner, event, No)] == collateral[event]`.
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ConditionalCollateralIndex(
+    pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>,
+);
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2): per-
+/// `(owner, event_id)` share balance pair (YES + NO sides).
+///
+/// **IS NOT** a Coin holding — shares are claims against
+/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
+/// explicit: shares are NOT counted in `total_supply_micro`.
+///
+/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
+/// Nested-map shape (rather than tuple-key) keeps the structure
+/// JSON-friendly (BTreeMap with tuple keys is not serializable through
+/// serde_json) while preserving canonical Owner-major / Event-minor
+/// ordering for replay determinism.
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ConditionalShareBalances(
+    pub BTreeMap<
+        AgentId,
+        BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>,
+    >,
+);
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + FR-13.3): YES + NO share
+/// holdings for a `(owner, event_id)` pair. Mint and seed credit
+/// equally; redeem debits the winning side.
+#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ShareSidePair {
+    pub yes: crate::state::typed_tx::ShareAmount,
+    pub no: crate::state::typed_tx::ShareAmount,
+}
+
@@ -700 +773 @@ mod tests {
-    fn economic_state_has_eleven_sub_fields() {
+    fn economic_state_has_thirteen_sub_fields() {
@@ -704,0 +778,5 @@ mod tests {
+        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3):
+        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
+        // holding, included in 6-holding total_supply_micro) +
+        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
+        // total_supply_micro).
@@ -710,2 +788,2 @@ mod tests {
-            11,
-            "EconomicState must have 11 sub-fields post-TB-12 (was 10 post-TB-11; +node_positions_t); got {}",
+            13,
+            "EconomicState must have 13 sub-fields post-TB-13 (was 11 post-TB-12; +conditional_collateral_t +conditional_share_balances_t); got {}",
@@ -715,0 +794,2 @@ mod tests {
+        assert!(obj.contains_key("conditional_collateral_t"), "TB-13 conditional_collateral_t sub-field missing");
+        assert!(obj.contains_key("conditional_share_balances_t"), "TB-13 conditional_share_balances_t sub-field missing");
diff --git a/src/state/sequencer.rs b/src/state/sequencer.rs
index 63ba4d5..2477d8d 100644
--- a/src/state/sequencer.rs
+++ b/src/state/sequencer.rs
@@ -253,0 +254,48 @@ pub fn task_bankruptcy_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
+/// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
+/// §4.3): CompleteSetMint-accept state-root domain.
+pub(crate) const COMPLETE_SET_MINT_DOMAIN_V1: &[u8] =
+    b"turingosv4.complete_set_mint.accept.v1";
+
+/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
+/// accept. Mirror of `task_open_accept_state_root`.
+pub fn complete_set_mint_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
+    let mut h = Sha256::new();
+    h.update(COMPLETE_SET_MINT_DOMAIN_V1);
+    h.update(prev.0);
+    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
+    let digest: [u8; 32] = h.finalize().into();
+    Hash::from_bytes(digest)
+}
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
+/// state-root domain.
+pub(crate) const COMPLETE_SET_REDEEM_DOMAIN_V1: &[u8] =
+    b"turingosv4.complete_set_redeem.accept.v1";
+
+/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
+/// accept. Mirror of `complete_set_mint_accept_state_root`.
+pub fn complete_set_redeem_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
+    let mut h = Sha256::new();
+    h.update(COMPLETE_SET_REDEEM_DOMAIN_V1);
+    h.update(prev.0);
+    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
+    let digest: [u8; 32] = h.finalize().into();

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '828,1290p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   828	// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
   829	// All three TB-13 typed-tx are AGENT-SIGNED (provider funds explicit; no
   830	// auto-seed; redeem requires system-resolution-reference + outcome match,
   831	// gated sequencer-side at admission). Domain prefixes mirror existing
   832	// agent-domain naming conventions (`turingosv4.agent_sig.<purpose>.v1`).
   833	const DOMAIN_AGENT_COMPLETE_SET_MINT: &[u8] = b"turingosv4.agent_sig.complete_set_mint.v1";
   834	const DOMAIN_AGENT_COMPLETE_SET_REDEEM: &[u8] = b"turingosv4.agent_sig.complete_set_redeem.v1";
   835	const DOMAIN_AGENT_MARKET_SEED: &[u8] = b"turingosv4.agent_sig.market_seed.v1";
   836	
   837	/// Reserved for v4.1 MetaTx (Gemini round-2 GR-1 recommendation).
   838	/// Not used in v4 — namespace placeholder so v4.1 can introduce
   839	/// `MetaSigningPayload` without re-rotating sibling domains. Marked
   840	/// `#[allow(dead_code)]` because no v4 consumer references it.
   841	#[allow(dead_code)]
   842	const DOMAIN_AGENT_META_PROPOSAL: &[u8] = b"turingosv4.agent_sig.meta_proposal.v1";
   843	
   844	fn domain_prefixed_digest<T: Serialize>(domain: &[u8], value: &T) -> [u8; 32] {
   845	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   846	    let body = canonical_encode(value).expect("canonical_encode of signing payload");
   847	    let mut h = Sha256::new();
   848	    h.update(domain);
   849	    h.update(&body);
   850	    h.finalize().into()
   851	}
   852	
   853	/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
   854	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   855	pub struct WorkSigningPayload {
   856	    pub tx_id: TxId,
   857	    pub task_id: TaskId,
   858	    pub parent_state_root: Hash,
   859	    pub agent_id: AgentId,
   860	    pub read_set: BTreeSet<ReadKey>,
   861	    pub write_set: BTreeSet<WriteKey>,
   862	    pub proposal_cid: Cid,
   863	    pub predicate_results: PredicateResultsBundle,
   864	    pub stake: StakeMicroCoin,
   865	    pub timestamp_logical: u64,
   866	}
   867	
   868	impl WorkSigningPayload {
   869	    pub fn canonical_digest(&self) -> [u8; 32] {
   870	        domain_prefixed_digest(DOMAIN_AGENT_WORK, self)
   871	    }
   872	}
   873	
   874	/// Agent signing payload for `VerifyTx` (8 fields → 7 fields; TB-4 bump).
   875	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   876	pub struct VerifySigningPayload {
   877	    pub tx_id: TxId,
   878	    pub parent_state_root: Hash,           // TB-4 NEW
   879	    pub target_work_tx: TxId,
   880	    pub verifier_agent: AgentId,
   881	    pub bond: StakeMicroCoin,
   882	    pub verdict: VerifyVerdict,
   883	    pub timestamp_logical: u64,
   884	}
   885	
   886	impl VerifySigningPayload {
   887	    pub fn canonical_digest(&self) -> [u8; 32] {
   888	        domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)
   889	    }
   890	}
   891	
   892	/// Agent signing payload for `ChallengeTx` (8 fields → 7 fields; TB-4 bump).
   893	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   894	pub struct ChallengeSigningPayload {
   895	    pub tx_id: TxId,
   896	    pub parent_state_root: Hash,           // TB-4 NEW
   897	    pub target_work_tx: TxId,
   898	    pub challenger_agent: AgentId,
   899	    pub stake: StakeMicroCoin,
   900	    pub counterexample_cid: Cid,
   901	    pub timestamp_logical: u64,
   902	}
   903	
   904	impl ChallengeSigningPayload {
   905	    pub fn canonical_digest(&self) -> [u8; 32] {
   906	        domain_prefixed_digest(DOMAIN_AGENT_CHALLENGE, self)
   907	    }
   908	}
   909	
   910	/// TRACE_MATRIX TB-3 — agent signing payload for `TaskOpenTx` (9 fields → 8 fields; signature excluded).
   911	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   912	pub struct TaskOpenSigningPayload {
   913	    pub tx_id: TxId,
   914	    pub task_id: TaskId,
   915	    pub parent_state_root: Hash,
   916	    pub sponsor_agent: AgentId,
   917	    pub verifier_quorum: u32,
   918	    pub max_reuse_royalty_fraction_basis_points: u16,
   919	    pub settlement_rule_hash: Hash,
   920	    pub timestamp_logical: u64,
   921	}
   922	
   923	impl TaskOpenSigningPayload {
   924	    pub fn canonical_digest(&self) -> [u8; 32] {
   925	        domain_prefixed_digest(DOMAIN_AGENT_TASK_OPEN, self)
   926	    }
   927	}
   928	
   929	/// TRACE_MATRIX TB-3 — agent signing payload for `EscrowLockTx` (7 fields → 6 fields).
   930	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   931	pub struct EscrowLockSigningPayload {
   932	    pub tx_id: TxId,
   933	    pub task_id: TaskId,
   934	    pub parent_state_root: Hash,
   935	    pub sponsor_agent: AgentId,
   936	    pub amount: MicroCoin,
   937	    pub timestamp_logical: u64,
   938	}
   939	
   940	impl EscrowLockSigningPayload {
   941	    pub fn canonical_digest(&self) -> [u8; 32] {
   942	        domain_prefixed_digest(DOMAIN_AGENT_ESCROW_LOCK, self)
   943	    }
   944	}
   945	
   946	/// System signing payload for `FinalizeRewardTx` (9 fields → 8 fields).
   947	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   948	pub struct FinalizeRewardSigningPayload {
   949	    pub tx_id: TxId,
   950	    pub claim_id: ClaimId,
   951	    pub task_id: TaskId,
   952	    pub solver: AgentId,
   953	    pub reward: MicroCoin,
   954	    pub parent_state_root: Hash,
   955	    pub epoch: SystemEpoch,
   956	    pub timestamp_logical: u64,
   957	}
   958	
   959	impl FinalizeRewardSigningPayload {
   960	    pub fn canonical_digest(&self) -> [u8; 32] {
   961	        domain_prefixed_digest(DOMAIN_SYSTEM_FINALIZE_REWARD, self)
   962	    }
   963	}
   964	
   965	/// System signing payload for `TaskExpireTx` (TB-11 bump: 10 fields → 9 fields).
   966	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   967	pub struct TaskExpireSigningPayload {
   968	    pub tx_id: TxId,
   969	    pub task_id: TaskId,
   970	    pub parent_state_root: Hash,
   971	    pub bounty_refunded: MicroCoin,
   972	    pub epoch: SystemEpoch,
   973	    pub timestamp_logical: u64,
   974	    pub sponsor_agent: AgentId,           // TB-11 NEW
   975	    pub escrow_tx_id: TxId,               // TB-11 NEW
   976	    pub reason: ExpireReason,             // TB-11 NEW
   977	}
   978	
   979	impl TaskExpireSigningPayload {
   980	    pub fn canonical_digest(&self) -> [u8; 32] {
   981	        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_EXPIRE, self)
   982	    }
   983	}
   984	
   985	/// System signing payload for `TerminalSummaryTx` (TB-11 bump: 11 fields → 10 fields).
   986	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   987	pub struct TerminalSummarySigningPayload {
   988	    pub tx_id: TxId,
   989	    pub task_id: TaskId,
   990	    pub run_id: RunId,
   991	    pub run_outcome: RunOutcome,
   992	    pub total_attempts: u32,
   993	    pub failure_class_histogram: BTreeMap<RejectionClass, u32>,
   994	    pub last_logical_t: u64,
   995	    pub parent_state_root: Hash,                  // TB-11 NEW
   996	    pub solver_agent: Option<AgentId>,            // TB-11 NEW
   997	    pub evidence_capsule_cid: Option<Cid>,        // TB-11 NEW
   998	}
   999	
  1000	impl TerminalSummarySigningPayload {
  1001	    pub fn canonical_digest(&self) -> [u8; 32] {
  1002	        domain_prefixed_digest(DOMAIN_SYSTEM_TERMINAL_SUMMARY, self)
  1003	    }
  1004	}
  1005	
  1006	/// TRACE_MATRIX FC1-Sig + FC3-Sig: TB-11 — System signing payload for
  1007	/// `TaskBankruptcyTx` (9 fields → 8 fields; system_signature excluded).
  1008	/// Signed via `CanonicalMessage::TaskBankruptcySigning([u8;32])` opaque
  1009	/// digest pattern (mirrors FinalizeRewardSigningPayload /
  1010	/// TaskExpireSigningPayload / TerminalSummarySigningPayload).
  1011	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1012	pub struct TaskBankruptcySigningPayload {
  1013	    pub tx_id: TxId,
  1014	    pub parent_state_root: Hash,
  1015	    pub task_id: TaskId,
  1016	    pub evidence_capsule_cid: Cid,
  1017	    pub bankruptcy_reason: BankruptcyReason,
  1018	    pub failed_run_count: u32,
  1019	    pub epoch: SystemEpoch,
  1020	    pub timestamp_logical: u64,
  1021	}
  1022	
  1023	impl TaskBankruptcySigningPayload {
  1024	    /// TRACE_MATRIX FC1-Sig: domain-prefixed canonical digest for
  1025	    /// system-emitted TaskBankruptcyTx signing. Domain prefix
  1026	    /// `b"turingosv4.system_sig.task_bankruptcy.v1"` mirrors the existing
  1027	    /// 4 system-tx signing domains (TerminalSummary / FinalizeReward /
  1028	    /// TaskExpire / ChallengeResolve).
  1029	    pub fn canonical_digest(&self) -> [u8; 32] {
  1030	        domain_prefixed_digest(DOMAIN_SYSTEM_TASK_BANKRUPTCY, self)
  1031	    }
  1032	}
  1033	
  1034	/// TRACE_MATRIX TB-5 charter v2 § 4.5 — system signing payload for
  1035	/// `ChallengeResolveTx` (7 fields → 6 fields; signature excluded).
  1036	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1037	pub struct ChallengeResolveSigningPayload {
  1038	    pub tx_id: TxId,
  1039	    pub parent_state_root: Hash,
  1040	    pub target_challenge_tx_id: TxId,
  1041	    pub resolution: ChallengeResolution,
  1042	    pub epoch: SystemEpoch,
  1043	    pub timestamp_logical: u64,
  1044	}
  1045	
  1046	impl ChallengeResolveSigningPayload {
  1047	    /// TRACE_MATRIX TB-5 charter v2 § 4.5: domain-prefixed canonical digest
  1048	    /// for system-emitted ChallengeResolveTx signing. Domain prefix
  1049	    /// `b"turingosv4.system_sig.challenge_resolve.v1"` mirrors the existing
  1050	    /// 3 system-tx signing domains.
  1051	    pub fn canonical_digest(&self) -> [u8; 32] {
  1052	        domain_prefixed_digest(DOMAIN_SYSTEM_CHALLENGE_RESOLVE, self)
  1053	    }
  1054	}
  1055	
  1056	// ────────────────────────────────────────────────────────────────────────────
  1057	// § 5c-TB-13 — CompleteSet + MarketSeedTx conditional shares
  1058	//
  1059	// TRACE_MATRIX TB-13 Atom 1 (architect 2026-05-03 post-TB-12 ruling Part A
  1060	// §4.3 + §4.4 FR-13.1..7 + §4.5 CR-13.1..6).
  1061	//
  1062	// **Mathematical core**: `1 locked Coin = 1 YES_E + 1 NO_E`.
  1063	// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
  1064	// mints equal YES_E + NO_E shares to the same owner. `CompleteSetRedeemTx`
  1065	// requires a system-resolved outcome reference and pays the winning side
  1066	// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
  1067	// provider funds; no auto-seed, no quote, no trade, no price.
  1068	//
  1069	// **Forbidden in TB-13** (architect §4.7): AMM / CPMM / orderbook /
  1070	// MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic
  1071	// liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64.
  1072	// ────────────────────────────────────────────────────────────────────────────
  1073	
  1074	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): event identifier for
  1075	/// conditional shares. TB-13 maps `EventId` 1:1 to `TaskId` (the event
  1076	/// being resolved is "this task got finalized YES via FinalizeRewardTx
  1077	/// vs. died NO via TaskBankruptcyTx"); future TB-14+ may decouple to
  1078	/// per-node events.
  1079	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
  1080	pub struct EventId(pub TaskId);
  1081	
  1082	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): outcome-side discriminator
  1083	/// for conditional shares. Yes = "this event was finalized YES";
  1084	/// No = "this event went bankrupt / was rejected".
  1085	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
  1086	#[repr(u8)]
  1087	pub enum OutcomeSide {
  1088	    Yes = 0,
  1089	    No = 1,
  1090	}
  1091	
  1092	impl Default for OutcomeSide {
  1093	    fn default() -> Self {
  1094	        Self::Yes
  1095	    }
  1096	}
  1097	
  1098	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
  1099	///
  1100	/// Architect spec uses `units: i128`; we tighten to `u128` because TB-13
  1101	/// shares can never be negative (mint creates positive, redeem decreases
  1102	/// positive, no debt model). Underflow at redeem time is a sequencer
  1103	/// `RedeemMoreThanOwned` rejection, not a representation concern.
  1104	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
  1105	pub struct ShareAmount {
  1106	    pub units: u128,
  1107	}
  1108	
  1109	impl ShareAmount {
  1110	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): zero share amount —
  1111	    /// default constructor for empty share balance lookups.
  1112	    pub const fn zero() -> Self {
  1113	        Self { units: 0 }
  1114	    }
  1115	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): build a `ShareAmount`
  1116	    /// from a raw `u128` units count. Used by sequencer mint/redeem arms
  1117	    /// (Atom 2) to project `MicroCoin::micro_units() as u128` into the
  1118	    /// share-claim domain.
  1119	    pub const fn from_units(units: u128) -> Self {
  1120	        Self { units }
  1121	    }
  1122	}
  1123	
  1124	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): system-resolution reference
  1125	/// embedded in `CompleteSetRedeemTx`. References either a
  1126	/// `TaskBankruptcyTx` (outcome must be `OutcomeSide::No`) or a
  1127	/// `FinalizeRewardTx` (outcome must be `OutcomeSide::Yes`) for the
  1128	/// referenced `EventId.0 == task_id`. Sequencer validates the reference
  1129	/// exists in L4 + outcome matches before allowing redeem (architect
  1130	/// FR-13.4 + SG-13.5).
  1131	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1132	pub struct ResolutionRef {
  1133	    pub resolution_tx_id: TxId,
  1134	    pub claimed_outcome: OutcomeSide,
  1135	}
  1136	
  1137	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.1..3): mint conditional
  1138	/// shares against locked Coin collateral.
  1139	///
  1140	/// Sequencer arm (Atom 2):
  1141	/// 1. `balances_t[owner] >= amount` else `InsufficientBalanceForMint`.
  1142	/// 2. `balances_t[owner] -= amount`.
  1143	/// 3. `conditional_collateral_t[event_id] += amount`.
  1144	/// 4. `conditional_share_balances_t[(owner, event_id, Yes)] += amount.units`.
  1145	/// 5. `conditional_share_balances_t[(owner, event_id, No)]  += amount.units`.
  1146	///
  1147	/// CTF preserved: balance debit equals collateral credit; YES/NO shares
  1148	/// are claims (not Coin) per CR-13.3 / SG-13.2.
  1149	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1150	pub struct CompleteSetMintTx {
  1151	    pub tx_id: TxId,                          //  1
  1152	    pub parent_state_root: Hash,              //  2
  1153	    pub event_id: EventId,                    //  3
  1154	    pub owner: AgentId,                       //  4
  1155	    pub amount: MicroCoin,                    //  5
  1156	    pub signature: AgentSignature,            //  6
  1157	    pub timestamp_logical: u64,               //  7
  1158	}
  1159	
  1160	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.4..5 + SG-13.5..6):
  1161	/// redeem winning conditional shares post-resolution.
  1162	///
  1163	/// Sequencer arm (Atom 2):
  1164	/// 1. Lookup `resolution_ref.resolution_tx_id` in L4 accepted set.
  1165	///    - If `TaskBankruptcyTx` for `event_id.0`: claimed_outcome must be
  1166	///      `No` else `InvalidResolutionRef`.
  1167	///    - If `FinalizeRewardTx` for `event_id.0`: claimed_outcome must be
  1168	///      `Yes` else `InvalidResolutionRef`.
  1169	///    - Else: `RedeemBeforeResolution` (no acceptable resolution).
  1170	/// 2. `conditional_share_balances_t[(owner, event_id, outcome)] >= share_amount.units`
  1171	///    else `RedeemMoreThanOwned`.
  1172	/// 3. `conditional_collateral_t[event_id] >= share_amount.units` else
  1173	///    `InsufficientCollateral` (defensive; should never fire if
  1174	///    `assert_complete_set_balanced` holds).
  1175	/// 4. Debit shares; debit collateral; credit `balances_t[owner]` 1:1.
  1176	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1177	pub struct CompleteSetRedeemTx {
  1178	    pub tx_id: TxId,                          //  1
  1179	    pub parent_state_root: Hash,              //  2
  1180	    pub event_id: EventId,                    //  3
  1181	    pub owner: AgentId,                       //  4
  1182	    pub outcome: OutcomeSide,                 //  5
  1183	    pub share_amount: ShareAmount,            //  6
  1184	    pub resolution_ref: ResolutionRef,        //  7
  1185	    pub signature: AgentSignature,            //  8
  1186	    pub timestamp_logical: u64,               //  9
  1187	}
  1188	
  1189	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.6..7): explicit
  1190	/// provider-funded protocol-owned share inventory seed. **NO trading,
  1191	/// NO quoting, NO pricing.**
  1192	///
  1193	/// Sequencer arm (Atom 2):
  1194	/// 1. `collateral_amount > 0` else `InsufficientCollateral` (SG-13.4).
  1195	/// 2. `balances_t[provider] >= collateral_amount` else
  1196	///    `InsufficientBalanceForMint` (SG-13.3).
  1197	/// 3. `balances_t[provider] -= collateral_amount`.
  1198	/// 4. `conditional_collateral_t[event_id] += collateral_amount`.
  1199	/// 5. Provider receives BOTH sides of share inventory:
  1200	///    `conditional_share_balances_t[(provider, event_id, Yes)] += collateral_amount.units`
  1201	///    `conditional_share_balances_t[(provider, event_id, No)]  += collateral_amount.units`.
  1202	///
  1203	/// The shape is identical to `CompleteSetMintTx` post-effect; the
  1204	/// distinction is semantic ("mint" = claim against own bet vs "seed" =
  1205	/// protocol-owned inventory pre-resolution). Future tracer-bullets may
  1206	/// treat seeded liquidity differently — TB-13 itself records only the
  1207	/// fact of seeding, not any signal derived from it.
  1208	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1209	pub struct MarketSeedTx {
  1210	    pub tx_id: TxId,                          //  1
  1211	    pub parent_state_root: Hash,              //  2
  1212	    pub event_id: EventId,                    //  3
  1213	    pub provider: AgentId,                    //  4
  1214	    pub collateral_amount: MicroCoin,         //  5
  1215	    pub signature: AgentSignature,            //  6
  1216	    pub timestamp_logical: u64,               //  7
  1217	}
  1218	
  1219	// ── TB-13 SigningPayloads ───────────────────────────────────────────────
  1220	
  1221	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
  1222	/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
  1223	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1224	pub struct CompleteSetMintSigningPayload {
  1225	    pub tx_id: TxId,
  1226	    pub parent_state_root: Hash,
  1227	    pub event_id: EventId,
  1228	    pub owner: AgentId,
  1229	    pub amount: MicroCoin,
  1230	    pub timestamp_logical: u64,
  1231	}
  1232	
  1233	impl CompleteSetMintSigningPayload {
  1234	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
  1235	    /// canonical digest for agent-signed CompleteSetMintTx. Domain
  1236	    /// prefix `b"turingosv4.agent_sig.complete_set_mint.v1"` mirrors
  1237	    /// agent-domain naming (Work / Verify / Challenge / TaskOpen /
  1238	    /// EscrowLock).
  1239	    pub fn canonical_digest(&self) -> [u8; 32] {
  1240	        domain_prefixed_digest(DOMAIN_AGENT_COMPLETE_SET_MINT, self)
  1241	    }
  1242	}
  1243	
  1244	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
  1245	/// `CompleteSetRedeemTx` (9 fields → 8 fields; signature excluded).
  1246	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1247	pub struct CompleteSetRedeemSigningPayload {
  1248	    pub tx_id: TxId,
  1249	    pub parent_state_root: Hash,
  1250	    pub event_id: EventId,
  1251	    pub owner: AgentId,
  1252	    pub outcome: OutcomeSide,
  1253	    pub share_amount: ShareAmount,
  1254	    pub resolution_ref: ResolutionRef,
  1255	    pub timestamp_logical: u64,
  1256	}
  1257	
  1258	impl CompleteSetRedeemSigningPayload {
  1259	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
  1260	    /// canonical digest for agent-signed CompleteSetRedeemTx. Domain
  1261	    /// prefix `b"turingosv4.agent_sig.complete_set_redeem.v1"`.
  1262	    pub fn canonical_digest(&self) -> [u8; 32] {
  1263	        domain_prefixed_digest(DOMAIN_AGENT_COMPLETE_SET_REDEEM, self)
  1264	    }
  1265	}
  1266	
  1267	/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
  1268	/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
  1269	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
  1270	pub struct MarketSeedSigningPayload {
  1271	    pub tx_id: TxId,
  1272	    pub parent_state_root: Hash,
  1273	    pub event_id: EventId,
  1274	    pub provider: AgentId,
  1275	    pub collateral_amount: MicroCoin,
  1276	    pub timestamp_logical: u64,
  1277	}
  1278	
  1279	impl MarketSeedSigningPayload {
  1280	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
  1281	    /// canonical digest for agent-signed MarketSeedTx. Domain prefix
  1282	    /// `b"turingosv4.agent_sig.market_seed.v1"`.
  1283	    pub fn canonical_digest(&self) -> [u8; 32] {
  1284	        domain_prefixed_digest(DOMAIN_AGENT_MARKET_SEED, self)
  1285	    }
  1286	}
  1287	
  1288	// ── Projections: tx → signing payload ────────────────────────────────────
  1289	
  1290	impl WorkTx {

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1440,1882p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1440	            epoch: self.epoch,
  1441	            timestamp_logical: self.timestamp_logical,
  1442	        }
  1443	    }
  1444	}
  1445	
  1446	// TB-13 — projection impls.
  1447	
  1448	impl CompleteSetMintTx {
  1449	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
  1450	    /// projection. Excludes `signature` to prevent cycle-on-self.
  1451	    pub fn to_signing_payload(&self) -> CompleteSetMintSigningPayload {
  1452	        CompleteSetMintSigningPayload {
  1453	            tx_id: self.tx_id.clone(),
  1454	            parent_state_root: self.parent_state_root,
  1455	            event_id: self.event_id.clone(),
  1456	            owner: self.owner.clone(),
  1457	            amount: self.amount,
  1458	            timestamp_logical: self.timestamp_logical,
  1459	        }
  1460	    }
  1461	}
  1462	
  1463	impl CompleteSetRedeemTx {
  1464	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
  1465	    /// projection. Excludes `signature` to prevent cycle-on-self.
  1466	    pub fn to_signing_payload(&self) -> CompleteSetRedeemSigningPayload {
  1467	        CompleteSetRedeemSigningPayload {
  1468	            tx_id: self.tx_id.clone(),
  1469	            parent_state_root: self.parent_state_root,
  1470	            event_id: self.event_id.clone(),
  1471	            owner: self.owner.clone(),
  1472	            outcome: self.outcome,
  1473	            share_amount: self.share_amount,
  1474	            resolution_ref: self.resolution_ref.clone(),
  1475	            timestamp_logical: self.timestamp_logical,
  1476	        }
  1477	    }
  1478	}
  1479	
  1480	impl MarketSeedTx {
  1481	    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
  1482	    /// projection. Excludes `signature` to prevent cycle-on-self.
  1483	    pub fn to_signing_payload(&self) -> MarketSeedSigningPayload {
  1484	        MarketSeedSigningPayload {
  1485	            tx_id: self.tx_id.clone(),
  1486	            parent_state_root: self.parent_state_root,
  1487	            event_id: self.event_id.clone(),
  1488	            provider: self.provider.clone(),
  1489	            collateral_amount: self.collateral_amount,
  1490	            timestamp_logical: self.timestamp_logical,
  1491	        }
  1492	    }
  1493	}
  1494	
  1495	// ────────────────────────────────────────────────────────────────────────────
  1496	// § 6 TypedTx outer enum
  1497	// ────────────────────────────────────────────────────────────────────────────
  1498	
  1499	/// TRACE_MATRIX § 8 dispatch_transition — typed-tx outer enum.
  1500	/// **10 variants pre-TB-11; 11 variants TB-11+** (K5 closed: NO `Slash`).
  1501	/// v1.1 P3 migrated `TerminalSummaryTx` here. **TB-3 (2026-04-30)**: added
  1502	/// `TaskOpen` + `EscrowLock` (RSP-1 formal surface; charter § 4.1). YES stake
  1503	/// stays inline in `WorkTx.stake` per WP § 14.1 + § 18 Inv 5; no separate
  1504	/// `YesStakeTx` variant. **TB-11 (2026-05-02)**: added `TaskBankruptcy`
  1505	/// (system-emitted task-level death certificate; architect §6.2; future
  1506	/// TB-12 NodeMarket Short / NO settlement anchor).
  1507	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
  1508	pub enum TypedTx {
  1509	    Work(WorkTx),
  1510	    Verify(VerifyTx),
  1511	    Challenge(ChallengeTx),
  1512	    Reuse(ReuseTx),
  1513	    FinalizeReward(FinalizeRewardTx),
  1514	    TaskExpire(TaskExpireTx),
  1515	    TerminalSummary(TerminalSummaryTx),
  1516	    TaskOpen(TaskOpenTx),         // TB-3 RSP-1 formal surface
  1517	    EscrowLock(EscrowLockTx),     // TB-3 RSP-1 formal surface
  1518	    ChallengeResolve(ChallengeResolveTx), // TB-5 RSP-3.0/3.1 system-emitted resolution
  1519	    TaskBankruptcy(TaskBankruptcyTx),     // TB-11 system-emitted task-level failure marker
  1520	    CompleteSetMint(CompleteSetMintTx),   // TB-13 agent-signed conditional-share mint
  1521	    CompleteSetRedeem(CompleteSetRedeemTx), // TB-13 agent-signed conditional-share redeem
  1522	    MarketSeed(MarketSeedTx),             // TB-13 agent-signed protocol-owned share seed
  1523	}
  1524	
  1525	impl TypedTx {
  1526	    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
  1527	    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
  1528	        use crate::bottom_white::ledger::transition_ledger::TxKind;
  1529	        match self {
  1530	            Self::Work(_) => TxKind::Work,
  1531	            Self::Verify(_) => TxKind::Verify,
  1532	            Self::Challenge(_) => TxKind::Challenge,
  1533	            Self::Reuse(_) => TxKind::Reuse,
  1534	            Self::FinalizeReward(_) => TxKind::FinalizeReward,
  1535	            Self::TaskExpire(_) => TxKind::TaskExpire,
  1536	            Self::TerminalSummary(_) => TxKind::TerminalSummary,
  1537	            Self::TaskOpen(_) => TxKind::TaskOpen,
  1538	            Self::EscrowLock(_) => TxKind::EscrowLock,
  1539	            Self::ChallengeResolve(_) => TxKind::ChallengeResolve,
  1540	            Self::TaskBankruptcy(_) => TxKind::TaskBankruptcy,
  1541	            Self::CompleteSetMint(_) => TxKind::CompleteSetMint,
  1542	            Self::CompleteSetRedeem(_) => TxKind::CompleteSetRedeem,
  1543	            Self::MarketSeed(_) => TxKind::MarketSeed,
  1544	        }
  1545	    }
  1546	}
  1547	
  1548	// ────────────────────────────────────────────────────────────────────────────
  1549	// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
  1550	// ────────────────────────────────────────────────────────────────────────────
  1551	
  1552	/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
  1553	/// by the implicit-init step in agent-submitted transitions. System-emitted
  1554	/// transitions return `None` (no agent to init).
  1555	pub trait HasSubmitter {
  1556	    fn submitter_id(&self) -> Option<AgentId>;
  1557	}
  1558	
  1559	impl HasSubmitter for WorkTx {
  1560	    fn submitter_id(&self) -> Option<AgentId> {
  1561	        Some(self.agent_id.clone())
  1562	    }
  1563	}
  1564	
  1565	impl HasSubmitter for VerifyTx {
  1566	    fn submitter_id(&self) -> Option<AgentId> {
  1567	        Some(self.verifier_agent.clone())
  1568	    }
  1569	}
  1570	
  1571	impl HasSubmitter for ChallengeTx {
  1572	    fn submitter_id(&self) -> Option<AgentId> {
  1573	        Some(self.challenger_agent.clone())
  1574	    }
  1575	}
  1576	
  1577	impl HasSubmitter for ReuseTx {
  1578	    fn submitter_id(&self) -> Option<AgentId> {
  1579	        None
  1580	    }
  1581	}
  1582	
  1583	impl HasSubmitter for FinalizeRewardTx {
  1584	    fn submitter_id(&self) -> Option<AgentId> {
  1585	        None
  1586	    }
  1587	}
  1588	
  1589	impl HasSubmitter for TaskExpireTx {
  1590	    fn submitter_id(&self) -> Option<AgentId> {
  1591	        None
  1592	    }
  1593	}
  1594	
  1595	impl HasSubmitter for TerminalSummaryTx {
  1596	    fn submitter_id(&self) -> Option<AgentId> {
  1597	        None
  1598	    }
  1599	}
  1600	
  1601	impl HasSubmitter for TaskOpenTx {
  1602	    fn submitter_id(&self) -> Option<AgentId> {
  1603	        Some(self.sponsor_agent.clone())
  1604	    }
  1605	}
  1606	
  1607	impl HasSubmitter for EscrowLockTx {
  1608	    fn submitter_id(&self) -> Option<AgentId> {
  1609	        Some(self.sponsor_agent.clone())
  1610	    }
  1611	}
  1612	
  1613	impl HasSubmitter for ChallengeResolveTx {
  1614	    fn submitter_id(&self) -> Option<AgentId> {
  1615	        None  // system-emitted; mirror FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx
  1616	    }
  1617	}
  1618	
  1619	impl HasSubmitter for TaskBankruptcyTx {
  1620	    fn submitter_id(&self) -> Option<AgentId> {
  1621	        None  // TB-11 system-emitted; mirror FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx / ChallengeResolveTx
  1622	    }
  1623	}
  1624	
  1625	// TB-13 — agent-signed conditional-share variants. Submitter is the
  1626	// owner / provider on the wire (mirrors WorkTx → agent_id pattern).
  1627	
  1628	impl HasSubmitter for CompleteSetMintTx {
  1629	    fn submitter_id(&self) -> Option<AgentId> {
  1630	        Some(self.owner.clone())
  1631	    }
  1632	}
  1633	
  1634	impl HasSubmitter for CompleteSetRedeemTx {
  1635	    fn submitter_id(&self) -> Option<AgentId> {
  1636	        Some(self.owner.clone())
  1637	    }
  1638	}
  1639	
  1640	impl HasSubmitter for MarketSeedTx {
  1641	    fn submitter_id(&self) -> Option<AgentId> {
  1642	        Some(self.provider.clone())
  1643	    }
  1644	}
  1645	
  1646	impl HasSubmitter for TypedTx {
  1647	    fn submitter_id(&self) -> Option<AgentId> {
  1648	        match self {
  1649	            Self::Work(t) => t.submitter_id(),
  1650	            Self::Verify(t) => t.submitter_id(),
  1651	            Self::Challenge(t) => t.submitter_id(),
  1652	            Self::Reuse(t) => t.submitter_id(),
  1653	            Self::FinalizeReward(t) => t.submitter_id(),
  1654	            Self::TaskExpire(t) => t.submitter_id(),
  1655	            Self::TerminalSummary(t) => t.submitter_id(),
  1656	            Self::TaskOpen(t) => t.submitter_id(),
  1657	            Self::EscrowLock(t) => t.submitter_id(),
  1658	            Self::ChallengeResolve(t) => t.submitter_id(),
  1659	            Self::TaskBankruptcy(t) => t.submitter_id(),
  1660	            Self::CompleteSetMint(t) => t.submitter_id(),
  1661	            Self::CompleteSetRedeem(t) => t.submitter_id(),
  1662	            Self::MarketSeed(t) => t.submitter_id(),
  1663	        }
  1664	    }
  1665	}
  1666	
  1667	// ────────────────────────────────────────────────────────────────────────────
  1668	// TransitionError — minimal v1 taxonomy (CO1.1.4-pre1 spec § 0 out-of-scope
  1669	// note: full per-stage enum proliferation is CO1.7.5)
  1670	// ────────────────────────────────────────────────────────────────────────────
  1671	
  1672	/// TRACE_MATRIX STATE § 3 — transition-function error taxonomy. v1.1 covers
  1673	/// every variant invoked in STATE_TRANSITION_SPEC § 3.1-3.7 pseudocode +
  1674	/// `NotYetImplemented` for CO1.7.5 stub bodies (per Codex Q-G CHALLENGE).
  1675	///
  1676	/// **Why payloads are minimal**: the failed `PredicateId` (etc.) is a string
  1677	/// reference; richer context (PredicateResultsBundle, Cid of failed proof)
  1678	/// is attached by the runtime via separate book-keeping channels (rejected
  1679	/// summary stamping, bus rejection log). Keeping TransitionError serializable
  1680	/// with primitive payloads avoids forcing PredicateResultsBundle through
  1681	/// every error site.
  1682	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
  1683	pub enum TransitionError {
  1684	    // ── Stale-parent & signature ───────────────────────────────────────────
  1685	    /// `parent_state_root` does not match `q.state_root_t` (any agent tx).
  1686	    StaleParent,
  1687	    /// Agent signature verify failed (work / verify / challenge tx).
  1688	    SignatureInvalid,
  1689	    /// System-keypair signature verify failed (system-emitted tx).
  1690	    InvalidSystemSignature,
  1691	
  1692	    // ── Economy ────────────────────────────────────────────────────────────
  1693	    /// Submitter's available balance is below the declared stake / bond.
  1694	    /// Payload-rich variant (available + required) is intentionally elided
  1695	    /// in v1.1 to keep this enum primitive-payloads-only; runtime attaches
  1696	    /// context via the rejection log (per STATE § 1.4 RejectedAttemptSummary).
  1697	    StakeInsufficient,
  1698	
  1699	    // ── Target lookup ──────────────────────────────────────────────────────
  1700	    /// VerifyTx / ChallengeTx / ReuseTx target work_tx not found in L4.
  1701	    TargetWorkTxNotFound,
  1702	    /// VerifyTx target is not in a verifiable status (e.g. already finalized).
  1703	    TargetWorkTxNotVerifiable,
  1704	    /// ReuseTx target work_tx exists but is not yet Accepted (parent must accept first).
  1705	    ParentNotAcceptedYet,
  1706	
  1707	    // ── Predicate failures ─────────────────────────────────────────────────
  1708	    /// step_transition stage 4 — acceptance predicate denied. `PredicateId`
  1709	    /// is the public predicate that failed; private predicates surface as
  1710	    /// `RejectionClass::Opaque` in book-keeping (NOT here).
  1711	    AcceptancePredicateFailed(PredicateId),
  1712	    /// verify_transition stage 4 — verification predicate denied.
  1713	    VerificationPredicateFailed(PredicateId),
  1714	    /// finalize_reward / step_transition stage 5 — settlement predicate denied.
  1715	    SettlementPredicateFailed(PredicateId),
  1716	
  1717	    // ── Challenge ──────────────────────────────────────────────────────────
  1718	    /// challenge_transition stage 1 — challenge filed after window closed.
  1719	    ChallengeWindowClosed,
  1720	    /// finalize_reward stage 1 — challenge window still open; cannot finalize.
  1721	    ChallengeWindowStillOpen,
  1722	    /// finalize_reward stage 1 — claim already slashed; cannot also reward.
  1723	    AlreadySlashed,
  1724	    /// challenge_transition stage 4 — counterexample failed predicate check.
  1725	    CounterexampleInsufficient,
  1726	
  1727	    // ── Reuse ──────────────────────────────────────────────────────────────
  1728	    /// reuse_transition stage 1 — referenced tool not in L2 ToolRegistry.
  1729	    ToolNotInRegistry,
  1730	    /// reuse_transition stage 1 — declared tool creator does not match registry.
  1731	    ToolCreatorMismatch,
  1732	
  1733	    // ── Finalize ───────────────────────────────────────────────────────────
  1734	    /// finalize_reward — no claim entry for the given claim_id.
  1735	    ClaimNotFound,
  1736	    /// TB-8 Atom 3 (charter §3 Atom 3 + ratification §1 Q2): finalize_reward
  1737	    /// idempotency — claim was already finalized by a prior accepted
  1738	    /// FinalizeRewardTx. Distinct from `AlreadySlashed` (which marks the
  1739	    /// adversarial-path terminal state); separate variants preserve the
  1740	    /// reward/slash discriminator that Phase 4 Information Loom needs. Maps
  1741	    /// to `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1742	    ClaimAlreadyFinalized,
  1743	
  1744	    // ── Task expire ────────────────────────────────────────────────────────
  1745	    /// task_expire — referenced TaskMarket entry not found.
  1746	    TaskNotFound,
  1747	    /// task_expire — deadline not yet reached.
  1748	    TaskNotExpired,
  1749	    /// task_expire — at least one open claim exists; cannot refund bounty.
  1750	    TaskHasOpenClaim,
  1751	
  1752	    // ── Terminal summary ───────────────────────────────────────────────────
  1753	    /// emit_terminal_summary — run already has an accepted work_tx.
  1754	    TerminalSummaryNotApplicable,
  1755	
  1756	    // ── TB-2 RSP-1 admission (preflight v3 §3.7) ───────────────────────────
  1757	    /// WorkTx-arm escrow / task-market lookup miss. The bridged
  1758	    /// `TxId(tx.task_id.0.clone())` did not match any entry in
  1759	    /// `q.economic_state_t.escrows_t.0` or `task_markets_t.0`. Maps to
  1760	    /// `L4ERejectionClass::EscrowMissing` per the §3.7 mapping table.
  1761	    EscrowMissing,
  1762	    /// `monetary_invariant::assert_no_post_init_mint` or
  1763	    /// `assert_total_ctf_conserved` failed on the WorkTx arm. Maps to
  1764	    /// `L4ERejectionClass::InvariantViolation`.
  1765	    MonetaryInvariantViolation,
  1766	
  1767	    // ── TB-3 RSP-1 formal-tx-surface (charter § 4.4) ───────────────────────
  1768	    /// `TaskOpenTx` admission idempotency: `task_markets_t` already
  1769	    /// contains an entry for this `task_id`. Maps to
  1770	    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1771	    TaskAlreadyOpen,
  1772	    /// `EscrowLockTx` / `WorkTx` admission referenced a `task_id` not in
  1773	    /// `task_markets_t`. Maps to `L4ERejectionClass::EscrowMissing` per
  1774	    /// charter § 4.5 (semantic re-use: no open task = no funded admission).
  1775	    TaskNotOpen,
  1776	    /// `EscrowLockTx` sponsor or accepted-`WorkTx` solver lacks balance
  1777	    /// for the requested debit. Maps to `L4ERejectionClass::InsufficientBalance`
  1778	    /// (NEW class per charter § 4.5 — do NOT fold into `PolicyViolation`;
  1779	    /// P4 Information Loom needs this discriminator).
  1780	    InsufficientBalance,
  1781	
  1782	    // ── TB-4 RSP-2 admission (charter § 3.8 + directive Q3) ────────────────
  1783	    /// `VerifyTx.bond` micro_units == 0. Distinct from `StakeInsufficient`
  1784	    /// (which is reused for ChallengeTx.stake==0 to keep WP economic § 7
  1785	    /// "Verifier 抵押 bond" naming honest). Maps to
  1786	    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1787	    BondInsufficient,
  1788	    /// VerifyTx / ChallengeTx target_work_tx is not in `q.economic_state_t.
  1789	    /// stakes_t` — i.e., the target was never accepted as a live WorkTx,
  1790	    /// OR has been resolved/finalized in a future RSP-3 path. In TB-4
  1791	    /// minimum scope these two cases collapse since RSP-3 has not yet
  1792	    /// introduced finalize-removes-stakes_t logic. **Distinct from**
  1793	    /// `TargetWorkTxNotFound` (reserved for "tx_id has no L4 row at all"
  1794	    /// — unreachable in TB-4 since dispatch_transition reads Q_t only)
  1795	    /// and `TargetWorkTxNotVerifiable` (reserved for "target tx_id exists
  1796	    /// but is not a WorkTx type" — also unreachable in TB-4 since the
  1797	    /// stakes_t lookup keys by TxId without type checking; TB-3
  1798	    /// `lock-on-accept` only inserts stakes_t entries for accepted WorkTx).
  1799	    /// Maps to `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1800	    TargetWorkInactive,
  1801	    /// `ChallengeTx.counterexample_cid == Cid::ZERO`. Sanity gate against
  1802	    /// empty challenges — distinct from `MalformedPayload` (which would
  1803	    /// reject earlier at deserialize time) and from `PolicyViolation`
  1804	    /// catch-all. P4 Information Loom needs this discriminator per
  1805	    /// directive Q7. Maps to `L4ERejectionClass::PolicyViolation` per
  1806	    /// charter § 4.5.
  1807	    EmptyCounterexample,
  1808	
  1809	    // ── TB-5.0 RSP-3.0 substrate (charter v2 § 4.9 + preflight § 3.5) ──────
  1810	    /// Agent attempted to submit a system-emitted variant
  1811	    /// (FinalizeRewardTx / TaskExpireTx / TerminalSummaryTx; ChallengeResolveTx
  1812	    /// added in TB-5 Atom 3) through the agent ingress path. The primary
  1813	    /// rejection happens at `Sequencer::submit_agent_tx` BEFORE dispatch
  1814	    /// (returns `SubmitError::SystemTxForbiddenOnAgentIngress` pre-queue).
  1815	    /// This `TransitionError` variant is the **defensive twin**: should
  1816	    /// any code path bypass the submit_agent_tx barrier and surface a
  1817	    /// system variant in `dispatch_transition`, this variant is the
  1818	    /// fail-closed dispatch response. Maps to
  1819	    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1820	    /// Anti-Oreo enforcement of "agent ≠ direct state writer" at the
  1821	    /// constitutional level (Art V.1.3 + WP § 12.4).
  1822	    SystemTxForbiddenOnAgentIngress,
  1823	    /// TB-5 Atom 4 (charter v2 § 4.3 + preflight § 4.5): apply_one stage 1.5
  1824	    /// live signature verification failed. Fired when a system-emitted
  1825	    /// variant reaches apply_one with a `system_signature` that does NOT
  1826	    /// verify against the pinned PinnedSystemPubkeys for the current epoch.
  1827	    /// Defense-in-depth atop the constructive `Sequencer::emit_system_tx`
  1828	    /// guarantee — under normal operation this should be unreachable
  1829	    /// (emit_system_tx signs internally with the runtime's keypair, and
  1830	    /// pinned_pubkeys are derived from that same keypair). This variant
  1831	    /// fires only if some code path bypasses emit_system_tx and surfaces a
  1832	    /// forged-signature system variant in the queue. Maps to
  1833	    /// `L4ERejectionClass::PolicyViolation` per charter § 4.5.
  1834	    /// Per directive § 11.4: "system_signature 不能只是 schema 上的字段"
  1835	    /// — this dispatch-side guard ensures it is live-verified.
  1836	    InvalidSystemSignatureLive,
  1837	    /// TB-5 Atom 5 (charter v2 § 4.6 + preflight § 7.2): the resolution
  1838	    /// targets a `target_challenge_tx_id` that is NOT present in
  1839	    /// `economic_state_t.challenge_cases_t` at apply time. Maps to
  1840	    /// `L4ERejectionClass::PolicyViolation`.
  1841	    ChallengeNotFound,
  1842	    /// TB-5 Atom 5 (charter v2 § 4.6 + preflight § 7.2): the targeted
  1843	    /// `ChallengeCase` is already in a non-Open state (Released or
  1844	    /// UpheldDeferred). Idempotency gate — re-resolution of the same
  1845	    /// case is rejected. Maps to `L4ERejectionClass::PolicyViolation`.
  1846	    AlreadyResolved,
  1847	
  1848	    // ── TB-13 Atom 2 (architect 2026-05-03 ruling Part A §4.4 FR-13.1..7) ──
  1849	    /// `CompleteSetMintTx` admission: `balances_t[owner] < amount`.
  1850	    /// Distinct from `InsufficientBalance` to give Information Loom a
  1851	    /// per-tx-class discriminator. Maps to `L4ERejectionClass::InsufficientBalance`.
  1852	    InsufficientBalanceForMint,
  1853	    /// `CompleteSetRedeemTx` admission: the referenced event is in
  1854	    /// `task_markets_t[event_id.0]` but its state is `Open` or `Expired`
  1855	    /// (i.e., neither `Finalized` for YES nor `Bankrupt` for NO). Architect
  1856	    /// FR-13.4 + SG-13.5: redeem unavailable before outcome resolution.
  1857	    /// Maps to `L4ERejectionClass::PolicyViolation`.
  1858	    RedeemBeforeResolution,
  1859	    /// `CompleteSetRedeemTx` admission: the owner's
  1860	    /// `conditional_share_balances_t[owner][event_id].{yes|no}` is less
  1861	    /// than the requested `share_amount.units`. Cannot redeem more than
  1862	    /// owned. Maps to `L4ERejectionClass::PolicyViolation`.
  1863	    RedeemMoreThanOwned,
  1864	    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
  1865	    /// Architect SG-13.4: market seed cannot create liquidity without
  1866	    /// collateral. Also fired defensively at `CompleteSetRedeemTx` time
  1867	    /// if `conditional_collateral_t[event_id]` lacks the redeemed amount
  1868	    /// (should never happen if `assert_complete_set_balanced` holds).
  1869	    /// Maps to `L4ERejectionClass::PolicyViolation`.
  1870	    InsufficientCollateral,
  1871	    /// `CompleteSetRedeemTx` admission: the resolution_ref's
  1872	    /// `claimed_outcome` does not match the `task_markets_t[event_id.0]`
  1873	    /// state (e.g., claimed_outcome=Yes but state=Bankrupt, or
  1874	    /// claimed_outcome=No but state=Finalized). Architect §4.3 +
  1875	    /// FR-13.5: after-YES pays YES not NO. Maps to
  1876	    /// `L4ERejectionClass::PolicyViolation`.
  1877	    InvalidResolutionRef,
  1878	
  1879	    // ── Stub sentinel (CO1.7.5 fills) ──────────────────────────────────────
  1880	    /// Stub return value used by CO1.7.5 unimplemented bodies — preserves
  1881	    /// sequencer + dispatch correctness without forcing transition logic
  1882	    /// into this atom. Audit input: this is intentional, not a code smell.

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '150,230p' && nl -ba src/state/q_state.rs | sed -n '520,575p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   150	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
   151	// ────────────────────────────────────────────────────────────────────────────
   152	
   153	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   154	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   155	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   156	/// in the owning atoms (CO P2.1-2.6).
   157	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   158	pub struct EconomicState {
   159	    pub balances_t: BalancesIndex,
   160	    pub escrows_t: EscrowsIndex,
   161	    pub stakes_t: StakesIndex,
   162	    pub claims_t: ClaimsIndex,
   163	    pub reputations_t: ReputationsIndex,
   164	    pub task_markets_t: TaskMarketsIndex,
   165	    pub royalty_graph_t: RoyaltyGraph,
   166	    pub challenge_cases_t: ChallengeCasesIndex,
   167	    pub price_index_t: PriceIndex,
   168	    /// TB-11 (architect §6.2 ruling 2026-05-02): runs_t — `RunId` → run-summary
   169	    /// entry written by the TerminalSummaryTx dispatch arm. Anchors
   170	    /// architect's RunExhaustedTx semantics on chain-resident state. Each
   171	    /// failed evaluator run produces exactly one entry (idempotency on
   172	    /// run_id). `#[serde(default)]` for backward-compat with pre-TB-11
   173	    /// chain snapshots.
   174	    #[serde(default)]
   175	    pub runs_t: RunsIndex,
   176	    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
   177	    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
   178	    /// of truth for exposure records. **NOT a Coin holding** (CR-12.1 + CR-12.2);
   179	    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
   180	    ///
   181	    /// Architect §3 explicitly REJECTED the nested `node_market_t:
   182	    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
   183	    /// (price + long_interest + short_interest aggregation), not canonical
   184	    /// state. Avoiding second source-of-truth (architect §3.2 reasoning;
   185	    /// TaskMarket.total_escrow precedent on cache=truth).
   186	    ///
   187	    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
   188	    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
   189	    /// nothing here per FR-12.3 + CR-12.8. `#[serde(default)]` for
   190	    /// backward-compat with pre-TB-12 chain snapshots.
   191	    #[serde(default)]
   192	    pub node_positions_t: NodePositionsIndex,
   193	    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
   194	    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
   195	    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
   196	    ///
   197	    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
   198	    /// holding"); included in the 6-holding `total_supply_micro` sum
   199	    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
   200	    /// 1 Coin → 1 YES_E + 1 NO_E mathematical identity (SG-13.1) ensures
   201	    /// `Σ_{event} conditional_collateral_t[event].units == Σ shares per side`.
   202	    ///
   203	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   204	    #[serde(default)]
   205	    pub conditional_collateral_t: ConditionalCollateralIndex,
   206	    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
   207	    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
   208	    ///
   209	    /// **IS NOT** a Coin holding — shares are CLAIMS against
   210	    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
   211	    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
   212	    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
   213	    /// winning side at 1 share = 1 MicroCoin against collateral.
   214	    ///
   215	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   216	    #[serde(default)]
   217	    pub conditional_share_balances_t: ConditionalShareBalances,
   218	}
   219	
   220	/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
   221	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   222	pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
   223	
   224	/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
   225	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   226	pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
   227	
   228	/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
   229	/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
   230	/// fields without breaking deserialization of historical ledger rows.
   520	// ────────────────────────────────────────────────────────────────────────────
   521	// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
   522	// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
   523	// conditional-share substrate. **1 locked Coin = 1 YES_E + 1 NO_E.**
   524	// ────────────────────────────────────────────────────────────────────────────
   525	
   526	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.4): per-event Coin
   527	/// collateral locked against outstanding YES_E + NO_E share inventory.
   528	///
   529	/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
   530	/// at `monetary_invariant::assert_total_ctf_conserved`. Mint/seed credit
   531	/// this map; redeem debits it. The complete-set balanced invariant
   532	/// (`assert_complete_set_balanced`) enforces
   533	/// `Σ_{owner} share[(owner, event, Yes)] == Σ_{owner} share[(owner, event, No)] == collateral[event]`.
   534	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   535	pub struct ConditionalCollateralIndex(
   536	    pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>,
   537	);
   538	
   539	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2): per-
   540	/// `(owner, event_id)` share balance pair (YES + NO sides).
   541	///
   542	/// **IS NOT** a Coin holding — shares are claims against
   543	/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
   544	/// explicit: shares are NOT counted in `total_supply_micro`.
   545	///
   546	/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
   547	/// Nested-map shape (rather than tuple-key) keeps the structure
   548	/// JSON-friendly (BTreeMap with tuple keys is not serializable through
   549	/// serde_json) while preserving canonical Owner-major / Event-minor
   550	/// ordering for replay determinism.
   551	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   552	pub struct ConditionalShareBalances(
   553	    pub BTreeMap<
   554	        AgentId,
   555	        BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>,
   556	    >,
   557	);
   558	
   559	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + FR-13.3): YES + NO share
   560	/// holdings for a `(owner, event_id)` pair. Mint and seed credit
   561	/// equally; redeem debits the winning side.
   562	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
   563	pub struct ShareSidePair {
   564	    pub yes: crate::state::typed_tx::ShareAmount,
   565	    pub no: crate::state::typed_tx::ShareAmount,
   566	}
   567	
   568	/// TRACE_MATRIX TB-11 (architect §6.2) — per-run summary. Sponsored by
   569	/// `task_id`; populated by the `TerminalSummaryTx` dispatch arm with
   570	/// fields drawn from the typed-tx wire payload (Q-derivable on replay).
   571	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   572	pub struct RunSummaryEntry {
   573	    /// Backref to the task this run was working on.
   574	    pub task_id: TaskId,
   575	    /// Architect §6.2 — terminal outcome. `OmegaAccepted` for happy-path

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1548,1875p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1548	
  1549	            // Step 5: monetary invariants (debit = credit).
  1550	            assert_no_post_init_mint(tx, q)
  1551	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1552	            assert_total_ctf_conserved(
  1553	                &q.economic_state_t,
  1554	                &q_next.economic_state_t,
  1555	                &[],
  1556	            )
  1557	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1558	            // TB-3 charter § 3.2 cache=truth invariant.
  1559	            assert_task_market_total_escrow_matches_locks(
  1560	                &q_next.economic_state_t,
  1561	                &lock.task_id,
  1562	            )
  1563	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1564	
  1565	            // Step 6: state_root advance via ESCROW_LOCK_DOMAIN_V1.
  1566	            q_next.state_root_t = escrow_lock_accept_state_root(&q.state_root_t, tx);
  1567	
  1568	            Ok((q_next, SignalBundle::default()))
  1569	        }
  1570	        // ──────────────────────────────────────────────────────────────────
  1571	        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
  1572	        // post-TB-12 ruling Part A §4.3 + §4.4 FR-13.1..3 + CR-13.1..6).
  1573	        //
  1574	        //   1 locked Coin → 1 YES_E + 1 NO_E.
  1575	        //
  1576	        // Debits balances_t[owner] by amount; credits
  1577	        // conditional_collateral_t[event_id] by amount; credits BOTH
  1578	        // conditional_share_balances_t[owner][event][Yes] and [No] by
  1579	        // amount.units. CTF preserved (balance debit = collateral credit;
  1580	        // shares are claims, not Coin per CR-13.3 + SG-13.2).
  1581	        // ──────────────────────────────────────────────────────────────────
  1582	        TypedTx::CompleteSetMint(mint) => {
  1583	            // Step 1: parent-root match.
  1584	            if mint.parent_state_root != q.state_root_t {
  1585	                return Err(TransitionError::StaleParent);
  1586	            }
  1587	            // Step 2: amount > 0 sanity.
  1588	            if mint.amount.micro_units() == 0 {
  1589	                return Err(TransitionError::InsufficientBalanceForMint);
  1590	            }
  1591	            // Step 3: owner solvency.
  1592	            let owner_bal = q
  1593	                .economic_state_t
  1594	                .balances_t
  1595	                .0
  1596	                .get(&mint.owner)
  1597	                .copied()
  1598	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1599	            if owner_bal.micro_units() < mint.amount.micro_units() {
  1600	                return Err(TransitionError::InsufficientBalanceForMint);
  1601	            }
  1602	            // Step 4: build q_next — atomic balance → collateral migration +
  1603	            // equal YES_E + NO_E share mint. The 6-holding sum (Atom 3
  1604	            // monetary_invariant extension) treats conditional_collateral_t
  1605	            // as a Coin holding, so total_supply_micro is preserved
  1606	            // bit-for-bit across mint.
  1607	            let mut q_next = q.clone();
  1608	            let new_bal_micro = owner_bal.micro_units() - mint.amount.micro_units();
  1609	            q_next.economic_state_t.balances_t.0.insert(
  1610	                mint.owner.clone(),
  1611	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
  1612	            );
  1613	            let collateral_entry = q_next
  1614	                .economic_state_t
  1615	                .conditional_collateral_t
  1616	                .0
  1617	                .entry(mint.event_id.clone())
  1618	                .or_insert(crate::economy::money::MicroCoin::zero());
  1619	            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1620	                collateral_entry.micro_units() + mint.amount.micro_units(),
  1621	            );
  1622	            let owner_shares = q_next
  1623	                .economic_state_t
  1624	                .conditional_share_balances_t
  1625	                .0
  1626	                .entry(mint.owner.clone())
  1627	                .or_insert_with(std::collections::BTreeMap::new);
  1628	            let pair = owner_shares
  1629	                .entry(mint.event_id.clone())
  1630	                .or_insert(crate::state::q_state::ShareSidePair::default());
  1631	            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1632	                pair.yes.units + mint.amount.micro_units() as u128,
  1633	            );
  1634	            pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1635	                pair.no.units + mint.amount.micro_units() as u128,
  1636	            );
  1637	
  1638	            // Step 5: monetary invariants.
  1639	            assert_no_post_init_mint(tx, q)
  1640	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1641	            assert_total_ctf_conserved(
  1642	                &q.economic_state_t,
  1643	                &q_next.economic_state_t,
  1644	                &[],
  1645	            )
  1646	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1647	
  1648	            // Step 6: state_root advance.
  1649	            q_next.state_root_t = complete_set_mint_accept_state_root(&q.state_root_t, tx);
  1650	
  1651	            Ok((q_next, SignalBundle::default()))
  1652	        }
  1653	        // ──────────────────────────────────────────────────────────────────
  1654	        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
  1655	        // FR-13.4..5 + SG-13.5..6).
  1656	        //
  1657	        // Validation:
  1658	        //   - task_markets_t[event_id.0].state must be Finalized (Yes) or
  1659	        //     Bankrupt (No); else RedeemBeforeResolution.
  1660	        //   - claimed_outcome must match the state; else InvalidResolutionRef.
  1661	        //   - owner's winning-side share balance must cover share_amount;
  1662	        //     else RedeemMoreThanOwned.
  1663	        //   - event collateral must cover share_amount; else
  1664	        //     InsufficientCollateral.
  1665	        //
  1666	        // Effect: 1 share → 1 MicroCoin (architect §4.3: "after YES outcome
  1667	        // pays YES shares"). Debit shares + collateral; credit balance.
  1668	        // ──────────────────────────────────────────────────────────────────
  1669	        TypedTx::CompleteSetRedeem(redeem) => {
  1670	            if redeem.parent_state_root != q.state_root_t {
  1671	                return Err(TransitionError::StaleParent);
  1672	            }
  1673	            // Step 1: claimed_outcome consistency between resolution_ref and
  1674	            // outcome field — both must agree before we even check state.
  1675	            if redeem.outcome != redeem.resolution_ref.claimed_outcome {
  1676	                return Err(TransitionError::InvalidResolutionRef);
  1677	            }
  1678	            // Step 2: lookup task_markets_t state.
  1679	            let market_state = q
  1680	                .economic_state_t
  1681	                .task_markets_t
  1682	                .0
  1683	                .get(&redeem.event_id.0)
  1684	                .map(|m| m.state)
  1685	                .ok_or(TransitionError::RedeemBeforeResolution)?;
  1686	            match (market_state, redeem.outcome) {
  1687	                (crate::state::q_state::TaskMarketState::Finalized,
  1688	                 crate::state::typed_tx::OutcomeSide::Yes) => { /* ok — YES wins */ }
  1689	                (crate::state::q_state::TaskMarketState::Bankrupt,
  1690	                 crate::state::typed_tx::OutcomeSide::No) => { /* ok — NO wins */ }
  1691	                (crate::state::q_state::TaskMarketState::Finalized, _)
  1692	                | (crate::state::q_state::TaskMarketState::Bankrupt, _) => {
  1693	                    return Err(TransitionError::InvalidResolutionRef);
  1694	                }
  1695	                (crate::state::q_state::TaskMarketState::Open, _)
  1696	                | (crate::state::q_state::TaskMarketState::Expired, _) => {
  1697	                    return Err(TransitionError::RedeemBeforeResolution);
  1698	                }
  1699	            }
  1700	            // Step 3: owner's share balance for the winning side.
  1701	            let pair = q
  1702	                .economic_state_t
  1703	                .conditional_share_balances_t
  1704	                .0
  1705	                .get(&redeem.owner)
  1706	                .and_then(|m| m.get(&redeem.event_id))
  1707	                .copied()
  1708	                .unwrap_or_default();
  1709	            let owned_units = match redeem.outcome {
  1710	                crate::state::typed_tx::OutcomeSide::Yes => pair.yes.units,
  1711	                crate::state::typed_tx::OutcomeSide::No => pair.no.units,
  1712	            };
  1713	            if owned_units < redeem.share_amount.units {
  1714	                return Err(TransitionError::RedeemMoreThanOwned);
  1715	            }
  1716	            // Step 4: collateral coverage (defensive; should hold if
  1717	            // assert_complete_set_balanced is preserved).
  1718	            let event_collateral = q
  1719	                .economic_state_t
  1720	                .conditional_collateral_t
  1721	                .0
  1722	                .get(&redeem.event_id)
  1723	                .copied()
  1724	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1725	            if (event_collateral.micro_units() as u128) < redeem.share_amount.units {
  1726	                return Err(TransitionError::InsufficientCollateral);
  1727	            }
  1728	
  1729	            // Step 5: build q_next.
  1730	            let mut q_next = q.clone();
  1731	            // 5a: debit the winning side from owner's share balance.
  1732	            {
  1733	                let owner_shares = q_next
  1734	                    .economic_state_t
  1735	                    .conditional_share_balances_t
  1736	                    .0
  1737	                    .entry(redeem.owner.clone())
  1738	                    .or_insert_with(std::collections::BTreeMap::new);
  1739	                let pair = owner_shares
  1740	                    .entry(redeem.event_id.clone())
  1741	                    .or_insert(crate::state::q_state::ShareSidePair::default());
  1742	                match redeem.outcome {
  1743	                    crate::state::typed_tx::OutcomeSide::Yes => {
  1744	                        pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1745	                            pair.yes.units - redeem.share_amount.units,
  1746	                        );
  1747	                    }
  1748	                    crate::state::typed_tx::OutcomeSide::No => {
  1749	                        pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1750	                            pair.no.units - redeem.share_amount.units,
  1751	                        );
  1752	                    }
  1753	                }
  1754	            }
  1755	            // 5b: debit collateral.
  1756	            {
  1757	                let collateral_entry = q_next
  1758	                    .economic_state_t
  1759	                    .conditional_collateral_t
  1760	                    .0
  1761	                    .entry(redeem.event_id.clone())
  1762	                    .or_insert(crate::economy::money::MicroCoin::zero());
  1763	                *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1764	                    collateral_entry.micro_units() - redeem.share_amount.units as i64,
  1765	                );
  1766	            }
  1767	            // 5c: credit owner's balance 1:1 (1 winning share = 1 MicroCoin).
  1768	            let owner_bal = q_next
  1769	                .economic_state_t
  1770	                .balances_t
  1771	                .0
  1772	                .get(&redeem.owner)
  1773	                .copied()
  1774	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1775	            q_next.economic_state_t.balances_t.0.insert(
  1776	                redeem.owner.clone(),
  1777	                crate::economy::money::MicroCoin::from_micro_units(
  1778	                    owner_bal.micro_units() + redeem.share_amount.units as i64,
  1779	                ),
  1780	            );
  1781	
  1782	            // Step 6: monetary invariants.
  1783	            assert_no_post_init_mint(tx, q)
  1784	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1785	            assert_total_ctf_conserved(
  1786	                &q.economic_state_t,
  1787	                &q_next.economic_state_t,
  1788	                &[],
  1789	            )
  1790	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1791	
  1792	            // Step 7: state_root advance.
  1793	            q_next.state_root_t = complete_set_redeem_accept_state_root(&q.state_root_t, tx);
  1794	
  1795	            Ok((q_next, SignalBundle::default()))
  1796	        }
  1797	        // ──────────────────────────────────────────────────────────────────
  1798	        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
  1799	        // SG-13.3..4). Provider explicitly funds collateral + receives BOTH
  1800	        // YES + NO share inventory. **No trading. No quoting. No pricing.**
  1801	        // ──────────────────────────────────────────────────────────────────
  1802	        TypedTx::MarketSeed(seed) => {
  1803	            if seed.parent_state_root != q.state_root_t {
  1804	                return Err(TransitionError::StaleParent);
  1805	            }
  1806	            // Step 1: collateral_amount > 0 (architect SG-13.4).
  1807	            if seed.collateral_amount.micro_units() == 0 {
  1808	                return Err(TransitionError::InsufficientCollateral);
  1809	            }
  1810	            // Step 2: provider solvency (architect SG-13.3).
  1811	            let provider_bal = q
  1812	                .economic_state_t
  1813	                .balances_t
  1814	                .0
  1815	                .get(&seed.provider)
  1816	                .copied()
  1817	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1818	            if provider_bal.micro_units() < seed.collateral_amount.micro_units() {
  1819	                return Err(TransitionError::InsufficientBalanceForMint);
  1820	            }
  1821	            // Step 3: build q_next — provider balance → collateral + provider
  1822	            // receives BOTH YES + NO share inventory.
  1823	            let mut q_next = q.clone();
  1824	            let new_bal_micro =
  1825	                provider_bal.micro_units() - seed.collateral_amount.micro_units();
  1826	            q_next.economic_state_t.balances_t.0.insert(
  1827	                seed.provider.clone(),
  1828	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
  1829	            );
  1830	            let collateral_entry = q_next
  1831	                .economic_state_t
  1832	                .conditional_collateral_t
  1833	                .0
  1834	                .entry(seed.event_id.clone())
  1835	                .or_insert(crate::economy::money::MicroCoin::zero());
  1836	            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1837	                collateral_entry.micro_units() + seed.collateral_amount.micro_units(),
  1838	            );
  1839	            let provider_shares = q_next
  1840	                .economic_state_t
  1841	                .conditional_share_balances_t
  1842	                .0
  1843	                .entry(seed.provider.clone())
  1844	                .or_insert_with(std::collections::BTreeMap::new);
  1845	            let pair = provider_shares
  1846	                .entry(seed.event_id.clone())
  1847	                .or_insert(crate::state::q_state::ShareSidePair::default());
  1848	            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1849	                pair.yes.units + seed.collateral_amount.micro_units() as u128,
  1850	            );
  1851	            pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1852	                pair.no.units + seed.collateral_amount.micro_units() as u128,
  1853	            );
  1854	
  1855	            // Step 4: monetary invariants.
  1856	            assert_no_post_init_mint(tx, q)
  1857	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1858	            assert_total_ctf_conserved(
  1859	                &q.economic_state_t,
  1860	                &q_next.economic_state_t,
  1861	                &[],
  1862	            )
  1863	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1864	
  1865	            // Step 5: state_root advance.
  1866	            q_next.state_root_t = market_seed_accept_state_root(&q.state_root_t, tx);
  1867	
  1868	            Ok((q_next, SignalBundle::default()))
  1869	        }
  1870	    }
  1871	}
  1872	
  1873	// ────────────────────────────────────────────────────────────────────────────
  1874	// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
  1875	// ────────────────────────────────────────────────────────────────────────────

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '420,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   420	) -> Option<crate::bottom_white::ledger::system_keypair::CanonicalMessage> {
   421	    use crate::bottom_white::ledger::system_keypair::CanonicalMessage;
   422	    match tx {
   423	        TypedTx::FinalizeReward(t) => {
   424	            let digest = t.to_signing_payload().canonical_digest();
   425	            Some(CanonicalMessage::FinalizeRewardSigning(digest))
   426	        }
   427	        TypedTx::TaskExpire(t) => {
   428	            let digest = t.to_signing_payload().canonical_digest();
   429	            Some(CanonicalMessage::TaskExpireSigning(digest))
   430	        }
   431	        TypedTx::TerminalSummary(t) => {
   432	            let digest = t.to_signing_payload().canonical_digest();
   433	            Some(CanonicalMessage::TerminalSummarySigning(digest))
   434	        }
   435	        TypedTx::ChallengeResolve(t) => {
   436	            let digest = t.to_signing_payload().canonical_digest();
   437	            Some(CanonicalMessage::ChallengeResolveSigning(digest))
   438	        }
   439	        // TB-11 Atom 1: TaskBankruptcyTx is system-emitted; verify against
   440	        // its signing payload digest under the TaskBankruptcySigning canonical
   441	        // message domain.
   442	        TypedTx::TaskBankruptcy(t) => {
   443	            let digest = t.to_signing_payload().canonical_digest();
   444	            Some(CanonicalMessage::TaskBankruptcySigning(digest))
   445	        }
   446	        // Agent-submitted variants: stage 1.5 is system-only. TB-13
   447	        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
   448	        // (verified separately at admission via the agent-signature path).
   449	        TypedTx::Work(_)
   450	        | TypedTx::Verify(_)
   451	        | TypedTx::Challenge(_)
   452	        | TypedTx::Reuse(_)
   453	        | TypedTx::TaskOpen(_)
   454	        | TypedTx::EscrowLock(_)
   455	        | TypedTx::CompleteSetMint(_)
   456	        | TypedTx::CompleteSetRedeem(_)
   457	        | TypedTx::MarketSeed(_) => None,
   458	    }
   459	}
   460	
   461	/// TRACE_MATRIX TB-5 Atom 4: extract `&SystemSignature` from a system-emitted
   462	/// TypedTx variant. Agent variants → `None`.
   463	fn system_signature_of(
   464	    tx: &TypedTx,
   465	) -> Option<&crate::bottom_white::ledger::system_keypair::SystemSignature> {
   466	    match tx {
   467	        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
   468	        TypedTx::TaskExpire(t) => Some(&t.system_signature),
   469	        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
   470	        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
   471	        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
   472	        TypedTx::Work(_)
   473	        | TypedTx::Verify(_)
   474	        | TypedTx::Challenge(_)
   475	        | TypedTx::Reuse(_)
   476	        | TypedTx::TaskOpen(_)
   477	        | TypedTx::EscrowLock(_)
   478	        | TypedTx::CompleteSetMint(_)
   479	        | TypedTx::CompleteSetRedeem(_)
   480	        | TypedTx::MarketSeed(_) => None,
   481	    }
   482	}
   483	
   484	/// TRACE_MATRIX TB-5 Atom 4: extract `SystemEpoch` from a system-emitted
   485	/// TypedTx variant for pinned-pubkey lookup. Agent variants → `None`.
   486	fn system_epoch_of(tx: &TypedTx) -> Option<SystemEpoch> {
   487	    match tx {
   488	        TypedTx::FinalizeReward(t) => Some(t.epoch),
   489	        TypedTx::TaskExpire(t) => Some(t.epoch),
   490	        // TerminalSummaryTx is signed via opaque digest only (no epoch field
   491	        // in struct per STATE § 1.5 8-field schema). Verification still uses
   492	        // the signing keypair's epoch — but since live verification needs
   493	        // the pinned pubkey for *some* epoch, we fall back to the signing
   494	        // keypair's currently-active epoch. Today TerminalSummary is emitted
   495	        // by the sequencer's runtime keypair under self.epoch; if cross-epoch
   496	        // replay is added the verifier will need to scan all pinned epochs.
   497	        TypedTx::TerminalSummary(_) => None,
   498	        TypedTx::ChallengeResolve(t) => Some(t.epoch),
   499	        TypedTx::TaskBankruptcy(t) => Some(t.epoch),
   500	        TypedTx::Work(_)
   501	        | TypedTx::Verify(_)
   502	        | TypedTx::Challenge(_)
   503	        | TypedTx::Reuse(_)
   504	        | TypedTx::TaskOpen(_)
   505	        | TypedTx::EscrowLock(_)
   506	        | TypedTx::CompleteSetMint(_)
   507	        | TypedTx::CompleteSetRedeem(_)
   508	        | TypedTx::MarketSeed(_) => None,
   509	    }
   510	}
   511	
   512	// ────────────────────────────────────────────────────────────────────────────
   513	// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
   514	// ────────────────────────────────────────────────────────────────────────────
   515	
   516	/// TRACE_MATRIX § 8 — exhaustive dispatch over `TypedTx` variants.
   517	///
   518	/// **Stub state (CO1.7-impl A3)**: every variant returns
   519	/// `TransitionError::NotYetImplemented`. CO1.7.5 fills each arm with the real
   520	/// transition body per `STATE_TRANSITION_SPEC § 3.1-3.7`. The exhaustive match

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2240,2360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2240	/// CO1.7-extra D3 (round-2 MF6): manual Debug impl. Uses `finish_non_exhaustive()`
  2241	/// to satisfy the Debug trait without exposing keypair / QState / CAS internals.
  2242	impl std::fmt::Debug for Sequencer {
  2243	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  2244	        f.debug_struct("Sequencer").finish_non_exhaustive()
  2245	    }
  2246	}
  2247	
  2248	impl Sequencer {
  2249	    /// Construct. Returns the `Sequencer` plus the receiver half of the
  2250	    /// internal mpsc; pass the receiver to `run()` exactly once.
  2251	    ///
  2252	    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
  2253	    /// added `pinned_pubkeys` parameter. Existing callers (7 src + tests
  2254	    /// per Codex round-2 cascade) updated to pass an `Arc<PinnedSystemPubkeys>`
  2255	    /// derived from the same keypair (test fixtures) or genesis-pinned
  2256	    /// (production). Tests typically pin `keypair.public_key()` under
  2257	    /// `epoch` for by-construction signature-verification correctness.
  2258	    #[allow(clippy::too_many_arguments)]
  2259	    pub fn new(
  2260	        cas: Arc<RwLock<CasStore>>,
  2261	        keypair: Arc<Ed25519Keypair>,
  2262	        epoch: SystemEpoch,
  2263	        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
  2264	        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
  2265	        predicate_registry: Arc<PredicateRegistry>,
  2266	        tool_registry: Arc<ToolRegistry>,
  2267	        pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
  2268	        initial_q: QState,
  2269	        queue_capacity: usize,
  2270	    ) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
  2271	        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
  2272	        let seq = Self {
  2273	            next_submit_id: AtomicU64::new(1),
  2274	            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
  2275	            next_emit_id: AtomicU64::new(1),    // TB-5 Atom 4: parallel system-emit counter
  2276	            queue_tx,
  2277	            cas,
  2278	            keypair,
  2279	            epoch,
  2280	            ledger_writer,
  2281	            rejection_writer,
  2282	            predicate_registry,
  2283	            tool_registry,
  2284	            pinned_pubkeys,
  2285	            q: RwLock::new(initial_q),
  2286	        };
  2287	        (seq, queue_rx)
  2288	    }
  2289	
  2290	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
  2291	    /// observability; production callers should not depend on this).
  2292	    #[cfg(test)]
  2293	    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
  2294	        &self.pinned_pubkeys
  2295	    }
  2296	
  2297	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek next_emit_id (parallel to
  2298	    /// `next_submit_id_peek` for K1-style observability).
  2299	    pub fn next_emit_id_peek(&self) -> u64 {
  2300	        self.next_emit_id.load(Ordering::SeqCst)
  2301	    }
  2302	
  2303	    /// TRACE_MATRIX FC2-Submit + § 5.2.1: TB-5.0 Atom 2 agent-only ingress
  2304	    /// barrier (charter v2 § 4.2 + § 4.9 + preflight § 3.2; Anti-Oreo Art V.1.3).
  2305	    ///
  2306	    /// Accepts ONLY agent-submitted variants. System-emitted variants
  2307	    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve added
  2308	    /// in Atom 3) are rejected pre-queue with
  2309	    /// `SubmitError::SystemTxForbiddenOnAgentIngress`. This is the
  2310	    /// constitutional Anti-Oreo "agent ≠ direct state writer" boundary,
  2311	    /// structurally enforced (was a documented norm without live enforcement
  2312	    /// through TB-3 + TB-4; TB-5.0 retires that debt for system-tx).
  2313	    ///
  2314	    /// **WP-canonical reconciliation**: ChallengeResolveTx (TB-5 Atom 3) +
  2315	    /// SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx
  2316	    /// (RSP-3.2+ / RSP-4 territory) will be added to the rejection match
  2317	    /// at their respective TB landings — each new system variant extends
  2318	    /// this list, never bypasses it.
  2319	    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
  2320	        // TB-5.0 ingress barrier: reject 4 system-emitted variants
  2321	        // (FinalizeReward / TaskExpire / TerminalSummary added in Atom 2;
  2322	        // ChallengeResolve added in Atom 3 when its TypedTx variant landed).
  2323	        match &tx {
  2324	            TypedTx::FinalizeReward(_)
  2325	            | TypedTx::TaskExpire(_)
  2326	            | TypedTx::TerminalSummary(_)
  2327	            | TypedTx::ChallengeResolve(_)
  2328	            // TB-11 Atom 1 (architect §6.2 ruling 2026-05-02): TaskBankruptcyTx
  2329	            // is system-emitted only; agent ingress must reject pre-queue per
  2330	            // Anti-Oreo (Art V.1.3). Construction goes through emit_system_tx.
  2331	            | TypedTx::TaskBankruptcy(_) => {
  2332	                return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
  2333	            }
  2334	            // Agent-submitted variants — proceed to queue. TB-13 conditional-
  2335	            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
  2336	            // are agent-signed and admit through the same ingress path.
  2337	            TypedTx::Work(_)
  2338	            | TypedTx::Verify(_)
  2339	            | TypedTx::Challenge(_)
  2340	            | TypedTx::Reuse(_)
  2341	            | TypedTx::TaskOpen(_)
  2342	            | TypedTx::EscrowLock(_)
  2343	            | TypedTx::CompleteSetMint(_)
  2344	            | TypedTx::CompleteSetRedeem(_)
  2345	            | TypedTx::MarketSeed(_) => {}
  2346	        }
  2347	        // TB-2 P1-D r1 concurrency contract: fetch_add precedes try_send, so
  2348	        // submit_id allocation order is NOT receiver arrival order under
  2349	        // multi-producer scheduling. submit_id is always burned (never reused)
  2350	        // even when try_send fails — locked by integration test I2.
  2351	        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
  2352	        let envelope = SubmissionEnvelope { submit_id, tx };
  2353	        match self.queue_tx.try_send(envelope) {
  2354	            Ok(()) => Ok(SubmissionReceipt { submit_id }),
  2355	            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
  2356	            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
  2357	        }
  2358	    }
  2359	
  2360	    /// TRACE_MATRIX TB-5 Atom 4 (charter v2 § 4.2 + preflight § 3.3): system-only

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '248,306p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   248	    h.update(prev.0);
   249	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
   250	    let digest: [u8; 32] = h.finalize().into();
   251	    Hash::from_bytes(digest)
   252	}
   253	
   254	/// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
   255	/// §4.3): CompleteSetMint-accept state-root domain.
   256	pub(crate) const COMPLETE_SET_MINT_DOMAIN_V1: &[u8] =
   257	    b"turingosv4.complete_set_mint.accept.v1";
   258	
   259	/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
   260	/// accept. Mirror of `task_open_accept_state_root`.
   261	pub fn complete_set_mint_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
   262	    let mut h = Sha256::new();
   263	    h.update(COMPLETE_SET_MINT_DOMAIN_V1);
   264	    h.update(prev.0);
   265	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
   266	    let digest: [u8; 32] = h.finalize().into();
   267	    Hash::from_bytes(digest)
   268	}
   269	
   270	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
   271	/// state-root domain.
   272	pub(crate) const COMPLETE_SET_REDEEM_DOMAIN_V1: &[u8] =
   273	    b"turingosv4.complete_set_redeem.accept.v1";
   274	
   275	/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
   276	/// accept. Mirror of `complete_set_mint_accept_state_root`.
   277	pub fn complete_set_redeem_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
   278	    let mut h = Sha256::new();
   279	    h.update(COMPLETE_SET_REDEEM_DOMAIN_V1);
   280	    h.update(prev.0);
   281	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
   282	    let digest: [u8; 32] = h.finalize().into();
   283	    Hash::from_bytes(digest)
   284	}
   285	
   286	/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
   287	/// domain.
   288	pub(crate) const MARKET_SEED_DOMAIN_V1: &[u8] =
   289	    b"turingosv4.market_seed.accept.v1";
   290	
   291	/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
   292	/// Mirror of `complete_set_mint_accept_state_root`.
   293	pub fn market_seed_accept_state_root(prev: &Hash, tx: &TypedTx) -> Hash {
   294	    let mut h = Sha256::new();
   295	    h.update(MARKET_SEED_DOMAIN_V1);
   296	    h.update(prev.0);
   297	    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
   298	    let digest: [u8; 32] = h.finalize().into();
   299	    Hash::from_bytes(digest)
   300	}
   301	
   302	// ────────────────────────────────────────────────────────────────────────────
   303	// TB-2 Atom 4 — rejection-path helpers (preflight v3 §3.5 + §3.7)
   304	// ────────────────────────────────────────────────────────────────────────────
   305	
   306	/// TRACE_MATRIX FC3-S3: TB-2 sentinel `agent_id` for rejected submissions

exec
/bin/bash -lc "rg --line-number \"to_signing_payload|CompleteSetMint|CompleteSetRedeem|MarketSeed|canonical_digest|AgentSignature\" src/state/sequencer.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
255:/// §4.3): CompleteSetMint-accept state-root domain.
259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
416:/// (their signatures are agent-domain `AgentSignature`, verified separately
424:            let digest = t.to_signing_payload().canonical_digest();
428:            let digest = t.to_signing_payload().canonical_digest();
432:            let digest = t.to_signing_payload().canonical_digest();
436:            let digest = t.to_signing_payload().canonical_digest();
443:            let digest = t.to_signing_payload().canonical_digest();
447:        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
455:        | TypedTx::CompleteSetMint(_)
456:        | TypedTx::CompleteSetRedeem(_)
457:        | TypedTx::MarketSeed(_) => None,
478:        | TypedTx::CompleteSetMint(_)
479:        | TypedTx::CompleteSetRedeem(_)
480:        | TypedTx::MarketSeed(_) => None,
506:        | TypedTx::CompleteSetMint(_)
507:        | TypedTx::CompleteSetRedeem(_)
508:        | TypedTx::MarketSeed(_) => None,
1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
1582:        TypedTx::CompleteSetMint(mint) => {
1654:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
1669:        TypedTx::CompleteSetRedeem(redeem) => {
1798:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
1802:        TypedTx::MarketSeed(seed) => {
2335:            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
2343:            | TypedTx::CompleteSetMint(_)
2344:            | TypedTx::CompleteSetRedeem(_)
2345:            | TypedTx::MarketSeed(_) => {}
2429:                let payload = tx.to_signing_payload();
2430:                let digest = payload.canonical_digest();
2478:                let payload = tx.to_signing_payload();
2479:                let digest = payload.canonical_digest();
2522:                let payload = tx.to_signing_payload();
2523:                let digest = payload.canonical_digest();
2570:                let payload = tx.to_signing_payload();
2571:                let digest = payload.canonical_digest();
2612:                let payload = tx.to_signing_payload();
2613:                let digest = payload.canonical_digest();
2629:                let digest = t.to_signing_payload().canonical_digest();
2638:                let digest = t.to_signing_payload().canonical_digest();
2647:                let digest = t.to_signing_payload().canonical_digest();
2660:                let digest = t.to_signing_payload().canonical_digest();
2669:                let digest = t.to_signing_payload().canonical_digest();
2918:        let signing_digest = signing_payload.canonical_digest();
2993:        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
3068:            signature: AgentSignature::from_bytes([0x77u8; 64]),
3281:            signature: AgentSignature::from_bytes([0u8; 64]),
3293:            signature: AgentSignature::from_bytes([0u8; 64]),
3391:            signature: AgentSignature::from_bytes([0u8; 64]),
3462:            signature: AgentSignature::from_bytes([0u8; 64]),
3612:            signature: AgentSignature::from_bytes([0u8; 64]),
3742:            signature: AgentSignature::from_bytes([0u8; 64]),
3867:            signature: AgentSignature::from_bytes([0u8; 64]),
4123:            signature: AgentSignature::from_bytes([0; 64]),
4136:            signature: AgentSignature::from_bytes([0; 64]),
4161:            signature: AgentSignature::from_bytes([0u8; 64]),
4174:            signature: AgentSignature::from_bytes([0u8; 64]),

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2388,2685p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2388	            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
  2389	                Err(EmitSystemError::QueueClosed)
  2390	            }
  2391	        }
  2392	    }
  2393	
  2394	    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.4): construct + sign a system
  2395	    /// tx from a high-level `SystemEmitCommand`. Internal-only; called by
  2396	    /// `emit_system_tx`. Each command variant constructs its corresponding
  2397	    /// typed tx struct, computes the signing-payload digest, signs with the
  2398	    /// runtime's system keypair, and returns the signed `TypedTx`.
  2399	    fn build_signed_system_tx(
  2400	        &self,
  2401	        command: SystemEmitCommand,
  2402	    ) -> Result<TypedTx, EmitSystemError> {
  2403	        use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::{
  2404	            sign_challenge_resolve, sign_finalize_reward,
  2405	        };
  2406	        use crate::bottom_white::ledger::system_keypair::SystemSignature;
  2407	        use crate::state::typed_tx::{ChallengeResolveTx, FinalizeRewardTx};
  2408	        match command {
  2409	            SystemEmitCommand::ChallengeResolve { target_challenge_tx_id, resolution } => {
  2410	                let q_snap = self
  2411	                    .q
  2412	                    .read()
  2413	                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
  2414	                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2415	                let mut tx = ChallengeResolveTx {
  2416	                    tx_id: crate::state::q_state::TxId(format!(
  2417	                        "system-challenge-resolve-{}-{}",
  2418	                        self.epoch.get(),
  2419	                        logical_t_for_id
  2420	                    )),
  2421	                    parent_state_root: q_snap.state_root_t,
  2422	                    target_challenge_tx_id,
  2423	                    resolution,
  2424	                    epoch: self.epoch,
  2425	                    timestamp_logical: logical_t_for_id,
  2426	                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
  2427	                };
  2428	                drop(q_snap);
  2429	                let payload = tx.to_signing_payload();
  2430	                let digest = payload.canonical_digest();
  2431	                let sig = sign_challenge_resolve(&self.keypair, digest)
  2432	                    .map_err(EmitSystemError::SignatureConstruction)?;
  2433	                tx.system_signature = sig;
  2434	                Ok(TypedTx::ChallengeResolve(tx))
  2435	            }
  2436	            // ──────────────────────────────────────────────────────────────
  2437	            // TB-8 Atom 2 — FinalizeReward construction.
  2438	            //
  2439	            // Caller passes claim_id only. task_id / solver / reward are
  2440	            // Q-derived from claims_t[claim_id] (anti-forgery per
  2441	            // typed_tx.rs:300-304). Wire fields = ledger summary; Q is
  2442	            // authoritative.
  2443	            //
  2444	            // Idempotency / window / upheld-challenge gates are enforced at
  2445	            // the dispatch arm (Atom 3), NOT here. emit_system_tx is the
  2446	            // construction layer; dispatch is the validation layer. This
  2447	            // separation matches the ChallengeResolve precedent.
  2448	            // ──────────────────────────────────────────────────────────────
  2449	            SystemEmitCommand::FinalizeReward { claim_id } => {
  2450	                let q_snap = self
  2451	                    .q
  2452	                    .read()
  2453	                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
  2454	                let claim = q_snap
  2455	                    .economic_state_t
  2456	                    .claims_t
  2457	                    .0
  2458	                    .get(claim_id.as_tx_id())
  2459	                    .ok_or(EmitSystemError::ClaimNotFound)?
  2460	                    .clone();
  2461	                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2462	                let mut tx = FinalizeRewardTx {
  2463	                    tx_id: crate::state::q_state::TxId(format!(
  2464	                        "system-finalize-reward-{}-{}",
  2465	                        self.epoch.get(),
  2466	                        logical_t_for_id
  2467	                    )),
  2468	                    claim_id,
  2469	                    task_id: claim.task_id.clone(),
  2470	                    solver: claim.claimant.clone(),
  2471	                    reward: claim.amount,
  2472	                    parent_state_root: q_snap.state_root_t,
  2473	                    epoch: self.epoch,
  2474	                    timestamp_logical: logical_t_for_id,
  2475	                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
  2476	                };
  2477	                drop(q_snap);
  2478	                let payload = tx.to_signing_payload();
  2479	                let digest = payload.canonical_digest();
  2480	                let sig = sign_finalize_reward(&self.keypair, digest)
  2481	                    .map_err(EmitSystemError::SignatureConstruction)?;
  2482	                tx.system_signature = sig;
  2483	                Ok(TypedTx::FinalizeReward(tx))
  2484	            }
  2485	            // ─────────────────────────────────────────────────────────────
  2486	            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) — TaskExpire
  2487	            // construction. Caller passes task_id + escrow_tx_id + reason;
  2488	            // runtime Q-derives sponsor_agent + bounty_refunded.
  2489	            // ─────────────────────────────────────────────────────────────
  2490	            SystemEmitCommand::TaskExpire { task_id, escrow_tx_id, reason } => {
  2491	                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_expire;
  2492	                use crate::state::typed_tx::TaskExpireTx;
  2493	                let q_snap = self
  2494	                    .q
  2495	                    .read()
  2496	                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
  2497	                let escrow = q_snap
  2498	                    .economic_state_t
  2499	                    .escrows_t
  2500	                    .0
  2501	                    .get(&escrow_tx_id)
  2502	                    .ok_or(EmitSystemError::ClaimNotFound)?
  2503	                    .clone();
  2504	                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2505	                let mut tx = TaskExpireTx {
  2506	                    tx_id: crate::state::q_state::TxId(format!(
  2507	                        "system-task-expire-{}-{}",
  2508	                        self.epoch.get(),
  2509	                        logical_t_for_id
  2510	                    )),
  2511	                    task_id,
  2512	                    parent_state_root: q_snap.state_root_t,
  2513	                    bounty_refunded: escrow.amount,
  2514	                    epoch: self.epoch,
  2515	                    timestamp_logical: logical_t_for_id,
  2516	                    sponsor_agent: escrow.depositor.clone(),
  2517	                    escrow_tx_id,
  2518	                    reason,
  2519	                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
  2520	                };
  2521	                drop(q_snap);
  2522	                let payload = tx.to_signing_payload();
  2523	                let digest = payload.canonical_digest();
  2524	                let sig = sign_task_expire(&self.keypair, digest)
  2525	                    .map_err(EmitSystemError::SignatureConstruction)?;
  2526	                tx.system_signature = sig;
  2527	                Ok(TypedTx::TaskExpire(tx))
  2528	            }
  2529	            // ─────────────────────────────────────────────────────────────
  2530	            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) —
  2531	            // TerminalSummary construction. Caller passes the run-summary
  2532	            // fields directly; runtime fills tx_id + parent_state_root +
  2533	            // epoch + timestamp_logical.
  2534	            // ─────────────────────────────────────────────────────────────
  2535	            SystemEmitCommand::TerminalSummary {
  2536	                run_id,
  2537	                task_id,
  2538	                run_outcome,
  2539	                total_attempts,
  2540	                failure_class_histogram,
  2541	                last_logical_t,
  2542	                solver_agent,
  2543	                evidence_capsule_cid,
  2544	            } => {
  2545	                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_terminal_summary;
  2546	                use crate::state::typed_tx::TerminalSummaryTx;
  2547	                let q_snap = self
  2548	                    .q
  2549	                    .read()
  2550	                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
  2551	                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2552	                let mut tx = TerminalSummaryTx {
  2553	                    tx_id: crate::state::q_state::TxId(format!(
  2554	                        "system-terminal-summary-{}-{}",
  2555	                        self.epoch.get(),
  2556	                        logical_t_for_id
  2557	                    )),
  2558	                    task_id,
  2559	                    run_id,
  2560	                    run_outcome,
  2561	                    total_attempts,
  2562	                    failure_class_histogram,
  2563	                    last_logical_t,
  2564	                    parent_state_root: q_snap.state_root_t,
  2565	                    solver_agent,
  2566	                    evidence_capsule_cid,
  2567	                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
  2568	                };
  2569	                drop(q_snap);
  2570	                let payload = tx.to_signing_payload();
  2571	                let digest = payload.canonical_digest();
  2572	                let sig = sign_terminal_summary(&self.keypair, digest)
  2573	                    .map_err(EmitSystemError::SignatureConstruction)?;
  2574	                tx.system_signature = sig;
  2575	                Ok(TypedTx::TerminalSummary(tx))
  2576	            }
  2577	            // ─────────────────────────────────────────────────────────────
  2578	            // TB-11 Atom 2 (architect §6.2 ruling 2026-05-02) —
  2579	            // TaskBankruptcy construction. Caller passes task_id +
  2580	            // evidence_capsule_cid + bankruptcy_reason + failed_run_count;
  2581	            // runtime fills tx_id + parent_state_root + epoch + ts_logical.
  2582	            // ─────────────────────────────────────────────────────────────
  2583	            SystemEmitCommand::TaskBankruptcy {
  2584	                task_id,
  2585	                evidence_capsule_cid,
  2586	                bankruptcy_reason,
  2587	                failed_run_count,
  2588	            } => {
  2589	                use crate::bottom_white::ledger::system_keypair::terminal_summary_emitter::sign_task_bankruptcy;
  2590	                use crate::state::typed_tx::TaskBankruptcyTx;
  2591	                let q_snap = self
  2592	                    .q
  2593	                    .read()
  2594	                    .map_err(|_| EmitSystemError::InternalLockPoisoned)?;
  2595	                let logical_t_for_id = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2596	                let mut tx = TaskBankruptcyTx {
  2597	                    tx_id: crate::state::q_state::TxId(format!(
  2598	                        "system-task-bankruptcy-{}-{}",
  2599	                        self.epoch.get(),
  2600	                        logical_t_for_id
  2601	                    )),
  2602	                    parent_state_root: q_snap.state_root_t,
  2603	                    task_id,
  2604	                    evidence_capsule_cid,
  2605	                    bankruptcy_reason,
  2606	                    failed_run_count,
  2607	                    epoch: self.epoch,
  2608	                    timestamp_logical: logical_t_for_id,
  2609	                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
  2610	                };
  2611	                drop(q_snap);
  2612	                let payload = tx.to_signing_payload();
  2613	                let digest = payload.canonical_digest();
  2614	                let sig = sign_task_bankruptcy(&self.keypair, digest)
  2615	                    .map_err(EmitSystemError::SignatureConstruction)?;
  2616	                tx.system_signature = sig;
  2617	                Ok(TypedTx::TaskBankruptcy(tx))
  2618	            }
  2619	        }
  2620	    }
  2621	
  2622	    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.5): defense-in-depth signature
  2623	    /// verification at emit time. Verifies the just-signed signature against
  2624	    /// pinned pubkeys for the current epoch.
  2625	    fn verify_emitted_system_tx_signature(&self, tx: &TypedTx) -> Result<(), EmitSystemError> {
  2626	        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
  2627	        match tx {
  2628	            TypedTx::ChallengeResolve(t) => {
  2629	                let digest = t.to_signing_payload().canonical_digest();
  2630	                let msg = CanonicalMessage::ChallengeResolveSigning(digest);
  2631	                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
  2632	                    return Err(EmitSystemError::InvalidSystemSignatureLive);
  2633	                }
  2634	                Ok(())
  2635	            }
  2636	            // TB-8 Atom 2 — FinalizeReward defense-in-depth verify.
  2637	            TypedTx::FinalizeReward(t) => {
  2638	                let digest = t.to_signing_payload().canonical_digest();
  2639	                let msg = CanonicalMessage::FinalizeRewardSigning(digest);
  2640	                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
  2641	                    return Err(EmitSystemError::InvalidSystemSignatureLive);
  2642	                }
  2643	                Ok(())
  2644	            }
  2645	            // TB-11 Atom 2 — TaskExpire defense-in-depth verify.
  2646	            TypedTx::TaskExpire(t) => {
  2647	                let digest = t.to_signing_payload().canonical_digest();
  2648	                let msg = CanonicalMessage::TaskExpireSigning(digest);
  2649	                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
  2650	                    return Err(EmitSystemError::InvalidSystemSignatureLive);
  2651	                }
  2652	                Ok(())
  2653	            }
  2654	            // TB-11 Atom 2 — TerminalSummary defense-in-depth verify.
  2655	            // TerminalSummaryTx has no `epoch` field on the wire; verify
  2656	            // against the sequencer's current epoch (mirrors apply_one
  2657	            // stage 1.5 behavior at system_epoch_of(TerminalSummary) -> None
  2658	            // → falls back to current epoch).
  2659	            TypedTx::TerminalSummary(t) => {
  2660	                let digest = t.to_signing_payload().canonical_digest();
  2661	                let msg = CanonicalMessage::TerminalSummarySigning(digest);
  2662	                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
  2663	                    return Err(EmitSystemError::InvalidSystemSignatureLive);
  2664	                }
  2665	                Ok(())
  2666	            }
  2667	            // TB-11 Atom 2 — TaskBankruptcy defense-in-depth verify.
  2668	            TypedTx::TaskBankruptcy(t) => {
  2669	                let digest = t.to_signing_payload().canonical_digest();
  2670	                let msg = CanonicalMessage::TaskBankruptcySigning(digest);
  2671	                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
  2672	                    return Err(EmitSystemError::InvalidSystemSignatureLive);
  2673	                }
  2674	                Ok(())
  2675	            }
  2676	            // emit_system_tx is system-only — agent variants are unreachable here.
  2677	            _ => Ok(()),
  2678	        }
  2679	    }
  2680	
  2681	    /// TRACE_MATRIX FC2-Submit + § 5.2.1: legacy public submit alias.
  2682	    ///
  2683	    /// Submit a typed transition (legacy alias; delegates to `submit_agent_tx`
  2684	    /// post-TB-5 Atom 2). Returns immediately with a receipt carrying
  2685	    /// `submit_id` (NOT `logical_t`). Per Q2 (back-pressure resolution): on

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2850,2965p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2850	        // On verification failure, route to L4.E with InvalidSystemSignatureLive
  2851	        // exactly like a dispatch reject — no logical_t consumed, no state_root
  2852	        // advance.
  2853	        if let Some(msg) = system_message_for_verification(&tx) {
  2854	            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
  2855	            let sig = system_signature_of(&tx)
  2856	                .expect("system_message_for_verification implies system_signature present");
  2857	            // TerminalSummaryTx carries no epoch field (STATE § 1.5 8-field
  2858	            // schema is digest-only); fall back to the apply-time sequencer
  2859	            // epoch. Other system variants carry epoch on the wire.
  2860	            let tx_epoch = system_epoch_of(&tx).unwrap_or(self.epoch);
  2861	            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
  2862	                let err = TransitionError::InvalidSystemSignatureLive;
  2863	                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
  2864	                return Err(ApplyError::Transition(err));
  2865	            }
  2866	        }
  2867	
  2868	        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
  2869	        // ledger and return early. K1: no logical_t consumed; Inv 7: no
  2870	        // state_root_t / ledger_root_t advance.
  2871	        let (q_next, _signals) = match dispatch_transition(
  2872	            &q_snapshot,
  2873	            &tx,
  2874	            &self.predicate_registry,
  2875	            &self.tool_registry,
  2876	        ) {
  2877	            Ok(ok) => ok,
  2878	            Err(transition_err) => {
  2879	                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
  2880	                // No logical_t advance, no state_root advance, no ledger_root
  2881	                // advance. Caller observes ApplyError::Transition.
  2882	                return Err(ApplyError::Transition(transition_err));
  2883	            }
  2884	        };
  2885	
  2886	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
  2887	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
  2888	
  2889	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
  2890	        let payload_bytes = canonical_encode(&tx)
  2891	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
  2892	        let payload_cid = {
  2893	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  2894	            cas_w.put(
  2895	                &payload_bytes,
  2896	                ObjectType::ProposalPayload,
  2897	                &format!("sequencer-epoch-{}", self.epoch.get()),
  2898	                logical_t,
  2899	                Some("TypedTx.v1".to_string()),
  2900	            )?
  2901	        };
  2902	
  2903	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
  2904	        // moved to AFTER stage 9 commit success).
  2905	        let signing_payload = LedgerEntrySigningPayload {
  2906	            logical_t,
  2907	            parent_state_root: q_snapshot.state_root_t,
  2908	            parent_ledger_root: q_snapshot.ledger_root_t,
  2909	            tx_kind: tx.tx_kind(),
  2910	            tx_payload_cid: payload_cid,
  2911	            resulting_state_root: q_next.state_root_t,
  2912	            timestamp_logical: logical_t,
  2913	            epoch: self.epoch,
  2914	            extensions: std::collections::BTreeMap::new(),
  2915	        };
  2916	
  2917	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
  2918	        let signing_digest = signing_payload.canonical_digest();
  2919	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
  2920	            &self.keypair,
  2921	            signing_digest.0,
  2922	        )?;
  2923	
  2924	        // Stage 7: pure ledger-root fold (deterministic).
  2925	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
  2926	
  2927	        // Stage 8: build LedgerEntry (the stored record).
  2928	        let entry = LedgerEntry {
  2929	            logical_t: signing_payload.logical_t,
  2930	            parent_state_root: signing_payload.parent_state_root,
  2931	            parent_ledger_root: signing_payload.parent_ledger_root,
  2932	            tx_kind: signing_payload.tx_kind,
  2933	            tx_payload_cid: signing_payload.tx_payload_cid,
  2934	            resulting_state_root: signing_payload.resulting_state_root,
  2935	            resulting_ledger_root,
  2936	            timestamp_logical: signing_payload.timestamp_logical,
  2937	            epoch: signing_payload.epoch,
  2938	            extensions: signing_payload.extensions,
  2939	            system_signature,
  2940	        };
  2941	
  2942	        // Stage 9: commit + mutate Q_t under write lock.
  2943	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
  2944	        // writer.commit succeeds — preserves K1 under infra failure.
  2945	        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
  2946	        // when writer surfaces a commit OID (Git2 path); no-op preservation
  2947	        // for writers that return None (InMemory path). state_root_t comes
  2948	        // from q_next as-is per K3 v1.2.
  2949	        {
  2950	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  2951	            let mut writer_w = self
  2952	                .ledger_writer
  2953	                .write()
  2954	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2955	            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
  2956	            // commit succeeded → safe to advance counter.
  2957	            self.next_logical_t.store(logical_t, Ordering::SeqCst);
  2958	            *q_w = q_next;
  2959	            q_w.ledger_root_t = entry.resulting_ledger_root;
  2960	            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
  2961	            advance_head_t(&mut *q_w, &*writer_w);
  2962	        }
  2963	
  2964	        Ok(entry)
  2965	    }

exec
/bin/bash -lc "rg --line-number \"verify_agent|AgentSignature|signature\" src/state/sequencer.rs src/bottom_white src/runtime src/state/typed_tx.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:12://! types (identifiers, signatures, predicate-result types, status enums) were
src/state/typed_tx.rs:82:// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
src/state/typed_tx.rs:86:/// signature over the per-tx canonical_digest. Distinct type from
src/state/typed_tx.rs:88:/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
src/state/typed_tx.rs:90:pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);
src/state/typed_tx.rs:92:impl AgentSignature {
src/state/typed_tx.rs:101:impl Default for AgentSignature {
src/state/typed_tx.rs:217:/// `step_transition` (CO1.7.5 body atom). The `signature` is over
src/state/typed_tx.rs:219:/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
src/state/typed_tx.rs:233:    pub signature: AgentSignature,                    // 10
src/state/typed_tx.rs:253:    pub signature: AgentSignature,         //  7
src/state/typed_tx.rs:277:    pub signature: AgentSignature,         //  7
src/state/typed_tx.rs:305:/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
src/state/typed_tx.rs:322:    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
src/state/typed_tx.rs:340:/// discriminator). Field 8/9/10 inserted **before** `system_signature` so
src/state/typed_tx.rs:363:    pub system_signature: SystemSignature, // 10  (was field 7 pre-TB-11)
src/state/typed_tx.rs:378:/// outcome on L4 with a system_signature.
src/state/typed_tx.rs:392:/// Fields inserted **before** `system_signature` so the signing payload sees
src/state/typed_tx.rs:418:    pub system_signature: SystemSignature,                    // 11 (was field 8 pre-TB-11)
src/state/typed_tx.rs:468:    pub system_signature: SystemSignature, //  9
src/state/typed_tx.rs:721:    pub signature: AgentSignature,             //  8
src/state/typed_tx.rs:742:    pub signature: AgentSignature,             //  6
src/state/typed_tx.rs:775:    pub system_signature: SystemSignature,          //  7
src/state/typed_tx.rs:801:// struct (subset of fields, EXCLUDES the signature itself) with a
src/state/typed_tx.rs:813:// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
src/state/typed_tx.rs:853:/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
src/state/typed_tx.rs:910:/// TRACE_MATRIX TB-3 — agent signing payload for `TaskOpenTx` (9 fields → 8 fields; signature excluded).
src/state/typed_tx.rs:1007:/// `TaskBankruptcyTx` (9 fields → 8 fields; system_signature excluded).
src/state/typed_tx.rs:1035:/// `ChallengeResolveTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1156:    pub signature: AgentSignature,            //  6
src/state/typed_tx.rs:1185:    pub signature: AgentSignature,            //  8
src/state/typed_tx.rs:1215:    pub signature: AgentSignature,            //  6
src/state/typed_tx.rs:1222:/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1245:/// `CompleteSetRedeemTx` (9 fields → 8 fields; signature excluded).
src/state/typed_tx.rs:1268:/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1385:    /// signing payload subset (excludes `system_signature` to prevent
src/state/typed_tx.rs:1431:    /// (excludes system_signature; 7 fields → 6 fields). Used by
src/state/typed_tx.rs:1450:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1465:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1482:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1684:    // ── Stale-parent & signature ───────────────────────────────────────────
src/state/typed_tx.rs:1687:    /// Agent signature verify failed (work / verify / challenge tx).
src/state/typed_tx.rs:1689:    /// System-keypair signature verify failed (system-emitted tx).
src/state/typed_tx.rs:1824:    /// live signature verification failed. Fired when a system-emitted
src/state/typed_tx.rs:1825:    /// variant reaches apply_one with a `system_signature` that does NOT
src/state/typed_tx.rs:1832:    /// forged-signature system variant in the queue. Maps to
src/state/typed_tx.rs:1834:    /// Per directive § 11.4: "system_signature 不能只是 schema 上的字段"
src/state/typed_tx.rs:1890:            Self::SignatureInvalid => write!(f, "agent signature invalid"),
src/state/typed_tx.rs:1891:            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
src/state/typed_tx.rs:1930:                "system_signature failed live verification against pinned \
src/state/typed_tx.rs:2105:            signature: AgentSignature::from_bytes([0x77u8; 64]),
src/state/typed_tx.rs:2118:            signature: AgentSignature::from_bytes([0x55u8; 64]),
src/state/typed_tx.rs:2131:            signature: AgentSignature::from_bytes([0x33u8; 64]),
src/state/typed_tx.rs:2156:            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
src/state/typed_tx.rs:2172:            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
src/state/typed_tx.rs:2196:            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
src/state/typed_tx.rs:2211:            system_signature: SystemSignature::from_bytes([0xddu8; 64]),
src/state/typed_tx.rs:2245:    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
src/state/typed_tx.rs:2251:            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
src/state/typed_tx.rs:2465:    /// Excluding the signature: mutating `tx.signature` must NOT change the
src/state/typed_tx.rs:2466:    /// signing-payload digest (the signature is its own input — a canonical
src/state/typed_tx.rs:2469:    fn signing_payload_excludes_signature() {
src/state/typed_tx.rs:2474:        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
src/state/typed_tx.rs:2476:        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");
src/state/typed_tx.rs:2482:        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
src/state/typed_tx.rs:2486:            "Verify: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2493:        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
src/state/typed_tx.rs:2497:            "Challenge: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2504:        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
src/state/typed_tx.rs:2508:            "FinalizeReward: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2513:        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
src/state/typed_tx.rs:2517:            "TaskExpire: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2522:        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
src/state/typed_tx.rs:2526:            "TerminalSummary: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2532:        bk_mut.system_signature = SystemSignature::from_bytes([0x44; 64]);
src/state/typed_tx.rs:2536:            "TaskBankruptcy: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2575:            "TaskBankruptcySigningPayload must have 8 fields (system_signature excluded), got {}",
src/state/typed_tx.rs:2578:        assert!(!obj.contains_key("system_signature"));
src/state/typed_tx.rs:2869:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/typed_tx.rs:2881:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/typed_tx.rs:2902:    /// T3 — TaskOpenSigningPayload excludes the signature field.
src/state/typed_tx.rs:2905:    fn task_open_signing_payload_excludes_signature() {
src/state/typed_tx.rs:2909:        assert_eq!(obj.len(), 8, "TaskOpenSigningPayload must have 8 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2910:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2913:    /// T4 — EscrowLockSigningPayload excludes the signature field.
src/state/typed_tx.rs:2916:    fn escrow_lock_signing_payload_excludes_signature() {
src/state/typed_tx.rs:2920:        assert_eq!(obj.len(), 6, "EscrowLockSigningPayload must have 6 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2921:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2971:    /// T3 — VerifySigningPayload excludes the signature field.
src/state/typed_tx.rs:2974:    fn verify_signing_payload_excludes_signature_field_count_7() {
src/state/typed_tx.rs:2978:        assert_eq!(obj.len(), 7, "VerifySigningPayload must have 7 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2979:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2983:    /// T4 — ChallengeSigningPayload excludes the signature field.
src/state/typed_tx.rs:2986:    fn challenge_signing_payload_excludes_signature_field_count_7() {
src/state/typed_tx.rs:2990:        assert_eq!(obj.len(), 7, "ChallengeSigningPayload must have 7 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2991:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:3039:            system_signature: SystemSignature::from_bytes([0x99u8; 64]),
src/state/typed_tx.rs:3052:    /// T2 — ChallengeResolveSigningPayload excludes the signature field.
src/state/typed_tx.rs:3055:    fn challenge_resolve_signing_payload_excludes_signature_field_count_6() {
src/state/typed_tx.rs:3060:            "ChallengeResolveSigningPayload must have 6 fields (signature excluded), got {}",
src/state/typed_tx.rs:3062:        assert!(!obj.contains_key("system_signature"));
src/state/typed_tx.rs:3110:            signature: AgentSignature::from_bytes([0xddu8; 64]),
src/state/typed_tx.rs:3127:            signature: AgentSignature::from_bytes([0xeeu8; 64]),
src/state/typed_tx.rs:3139:            signature: AgentSignature::from_bytes([0xffu8; 64]),
src/state/typed_tx.rs:3215:    /// TB-13 U7: signing payloads exclude the `signature` field — exact
src/state/typed_tx.rs:3218:    fn tb_13_signing_payloads_exclude_signature_field_counts() {
src/state/typed_tx.rs:3223:        assert!(!mint_o.contains_key("signature"));
src/state/typed_tx.rs:3229:        assert!(!redeem_o.contains_key("signature"));
src/state/typed_tx.rs:3235:        assert!(!seed_o.contains_key("signature"));
src/state/sequencer.rs:389:        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
src/state/sequencer.rs:404:// system_signature should bind to. Agent variants return `None`. The
src/state/sequencer.rs:409:// `system_signature_of` / `system_epoch_of`: extract the signature + epoch
src/state/sequencer.rs:414:/// a system-emitted `TypedTx` to its `CanonicalMessage` for live signature
src/state/sequencer.rs:416:/// (their signatures are agent-domain `AgentSignature`, verified separately
src/state/sequencer.rs:448:        // (verified separately at admission via the agent-signature path).
src/state/sequencer.rs:463:fn system_signature_of(
src/state/sequencer.rs:467:        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
src/state/sequencer.rs:468:        TypedTx::TaskExpire(t) => Some(&t.system_signature),
src/state/sequencer.rs:469:        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
src/state/sequencer.rs:470:        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
src/state/sequencer.rs:471:        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
src/state/sequencer.rs:1158:        // Anti-Oreo: arm fires only when system_signature verified at
src/state/sequencer.rs:1967:/// pass a forged signature because they don't construct the typed tx.
src/state/sequencer.rs:2063:    /// Verification of the just-signed signature failed against pinned
src/state/sequencer.rs:2084:                write!(f, "system-tx signature construction failed: {e:?}")
src/state/sequencer.rs:2088:                "system_signature failed live verification against pinned pubkeys at emit time"
src/state/sequencer.rs:2104:/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
src/state/sequencer.rs:2231:    /// `system_signature` on system-emitted variants (defense-in-depth atop
src/state/sequencer.rs:2252:    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
src/state/sequencer.rs:2257:    /// `epoch` for by-construction signature-verification correctness.
src/state/sequencer.rs:2364:    /// Cannot be invoked with a forged signature because the signature is
src/state/sequencer.rs:2374:        // Step 2: Defense-in-depth — verify the just-signed signature against
src/state/sequencer.rs:2378:        self.verify_emitted_system_tx_signature(&tx)?;
src/state/sequencer.rs:2426:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2433:                tx.system_signature = sig;
src/state/sequencer.rs:2475:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2482:                tx.system_signature = sig;
src/state/sequencer.rs:2519:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2526:                tx.system_signature = sig;
src/state/sequencer.rs:2567:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2574:                tx.system_signature = sig;
src/state/sequencer.rs:2609:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2616:                tx.system_signature = sig;
src/state/sequencer.rs:2622:    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.5): defense-in-depth signature
src/state/sequencer.rs:2623:    /// verification at emit time. Verifies the just-signed signature against
src/state/sequencer.rs:2625:    fn verify_emitted_system_tx_signature(&self, tx: &TypedTx) -> Result<(), EmitSystemError> {
src/state/sequencer.rs:2626:        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
src/state/sequencer.rs:2631:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2640:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2649:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2662:                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2671:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2739:    /// BOTH dispatch failures (stage 2) AND signature-verification failures
src/state/sequencer.rs:2845:        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
src/state/sequencer.rs:2849:        // (or stale signature in a replay) is rejected at the apply boundary.
src/state/sequencer.rs:2854:            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
src/state/sequencer.rs:2855:            let sig = system_signature_of(&tx)
src/state/sequencer.rs:2856:                .expect("system_message_for_verification implies system_signature present");
src/state/sequencer.rs:2861:            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2889:        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
src/state/sequencer.rs:2919:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
src/state/sequencer.rs:2939:            system_signature,
src/state/sequencer.rs:2993:        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
src/state/sequencer.rs:3068:            signature: AgentSignature::from_bytes([0x77u8; 64]),
src/state/sequencer.rs:3196:        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
src/state/sequencer.rs:3281:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3293:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3391:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3462:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3612:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3742:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3867:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4026:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4050:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4073:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4095:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4123:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4136:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4161:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4174:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4187:    // U27/U28 + I66/I66.a/b/c: forged signatures on system-emitted variants
src/state/sequencer.rs:4197:    /// Helper: forge a ChallengeResolveTx with all-zero signature.
src/state/sequencer.rs:4206:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4220:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4235:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4251:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4257:    fn stage_1_5_rejects_forged_challenge_resolve_signature() {
src/state/sequencer.rs:4277:    fn stage_1_5_rejects_forged_finalize_reward_signature() {
src/state/sequencer.rs:4292:    fn stage_1_5_rejects_forged_task_expire_signature() {
src/state/sequencer.rs:4307:    fn stage_1_5_rejects_forged_terminal_summary_signature() {
src/state/sequencer.rs:4359:    /// "missing system_signature" errors when an agent variant is applied.
src/state/sequencer.rs:4382:    // arm body from the apply_one + queue + signature pipeline.
src/state/sequencer.rs:4432:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/runtime/mod.rs:91:    /// entry signatures without separate config.
src/runtime/mod.rs:274:/// at bootstrap so `verify_chaintape` (Atom 4) can re-verify `system_signature`
src/runtime/adapter.rs:26:    AgentSignature, BoolWithProof, EscrowLockSigningPayload, EscrowLockTx, PredicateId,
src/runtime/adapter.rs:65:        signature: AgentSignature::from_bytes([0u8; 64]),
src/runtime/adapter.rs:84:        signature: AgentSignature::from_bytes([0u8; 64]),
src/runtime/adapter.rs:129:        signature: AgentSignature::from_bytes([0u8; 64]),
src/runtime/adapter.rs:134:/// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — real-signature WorkTx constructor.
src/runtime/adapter.rs:143:///    signature, not a zero placeholder.
src/runtime/adapter.rs:144:/// 3. The `AgentSignature` is verifiable post-replay against the
src/runtime/adapter.rs:184:    // Build the SigningPayload (10 fields; signature excluded per typed_tx.rs §3).
src/runtime/adapter.rs:198:    let signature = keypairs.sign(&agent_id, digest)?;
src/runtime/adapter.rs:210:        signature,
src/runtime/adapter.rs:215:/// TRACE_MATRIX FC1-N14: TB-7 Atom 3 — real-signature VerifyTx constructor for
src/runtime/adapter.rs:256:    let signature = keypairs.sign(&verifier_id, digest)?;
src/runtime/adapter.rs:265:        signature,
src/runtime/adapter.rs:271:// TB-10 Atom 1 — Real-signature constructors for user-driven TaskOpen + EscrowLock.
src/runtime/adapter.rs:273:// The synthetic constructors above use `AgentSignature::from_bytes([0u8; 64])`
src/runtime/adapter.rs:279:// User-driven TaskOpen + EscrowLock SHOULD carry real signatures so the chain
src/runtime/adapter.rs:287:/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature TaskOpenTx constructor.
src/runtime/adapter.rs:290:/// Mirrors `make_synthetic_task_open` shape but produces a non-zero Ed25519 signature
src/runtime/adapter.rs:315:    let signature = keypairs.sign(&sponsor_id, digest)?;
src/runtime/adapter.rs:324:        signature,
src/runtime/adapter.rs:329:/// TRACE_MATRIX FC1-N14: TB-10 Atom 1 — real-signature EscrowLockTx constructor.
src/runtime/adapter.rs:333:/// signature over `EscrowLockSigningPayload::canonical_digest()`.
src/runtime/adapter.rs:357:    let signature = keypairs.sign(&sponsor_id, digest)?;
src/runtime/adapter.rs:364:        signature,
src/runtime/adapter.rs:609:    /// U-A2.a — make_real_worktx_signed_by produces a non-zero signature
src/runtime/adapter.rs:612:    fn real_worktx_signature_is_nonzero_and_verifies() {
src/runtime/adapter.rs:613:        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/adapter.rs:633:        assert_ne!(*work.signature.as_bytes(), [0u8; 64]);
src/runtime/adapter.rs:650:        verify_agent_signature(&work.signature, &digest, &pubkey).expect("verify");
src/runtime/adapter.rs:653:    /// U-A2.b — same record, same registry → same signature byte-for-byte
src/runtime/adapter.rs:684:            TypedTx::Work(w) => *w.signature.as_bytes(),
src/runtime/adapter.rs:688:            TypedTx::Work(w) => *w.signature.as_bytes(),
src/bottom_white/ledger/mod.rs:5:/// TRACE_MATRIX FC1-Sig+FC3-Sig: system runtime signature key lifecycle.
src/bottom_white/ledger/transition_ledger.rs:104:/// (includes derivatives + signature); the signing payload is the subset that
src/bottom_white/ledger/transition_ledger.rs:127:    /// Bound in signed payload (G1 cannot bypass signature).
src/bottom_white/ledger/transition_ledger.rs:129:    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
src/bottom_white/ledger/transition_ledger.rs:130:    pub system_signature: SystemSignature,       // 11
src/bottom_white/ledger/transition_ledger.rs:141:/// - `system_signature` (its own input)
src/bottom_white/ledger/transition_ledger.rs:218:// LedgerWriter trait (K4 reconciled to skeleton signature)
src/bottom_white/ledger/transition_ledger.rs:225:/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
src/bottom_white/ledger/transition_ledger.rs:319:/// - `FullTransition`: CO1.7.5+ stage; verifies signatures + re-fetches payloads
src/bottom_white/ledger/transition_ledger.rs:371:            Self::BadSignature { at } => write!(f, "system_signature verify failed at index {at}"),
src/bottom_white/ledger/transition_ledger.rs:415:/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
src/bottom_white/ledger/transition_ledger.rs:442:        verify_system_signature, CanonicalMessage,
src/bottom_white/ledger/transition_ledger.rs:467:        // Stage 4: system_signature verify (FullTransition mode only).
src/bottom_white/ledger/transition_ledger.rs:471:        if !verify_system_signature(
src/bottom_white/ledger/transition_ledger.rs:472:            &entry.system_signature,
src/bottom_white/ledger/transition_ledger.rs:539:/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
src/bottom_white/ledger/transition_ledger.rs:599:/// needing byte-stable signatures over typed payloads.
src/bottom_white/ledger/transition_ledger.rs:652:///     - `signature`       = entry.system_signature.as_bytes() (64 bytes)
src/bottom_white/ledger/transition_ledger.rs:678:const TREE_BLOB_SIGNATURE: &str = "signature";
src/bottom_white/ledger/transition_ledger.rs:808:            .blob(entry.system_signature.as_bytes())
src/bottom_white/ledger/transition_ledger.rs:909:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/bottom_white/ledger/transition_ledger.rs:1000:    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
src/bottom_white/ledger/transition_ledger.rs:1016:        // Mutate system_signature (signature is its own input; should NOT affect signing digest)
src/bottom_white/ledger/transition_ledger.rs:1018:        e_tamper2.system_signature = SystemSignature::from_bytes([0xffu8; 64]);
src/bottom_white/ledger/transition_ledger.rs:1029:    // 9. C3 closure (round-2): real signature roundtrip via system_keypair extension.
src/bottom_white/ledger/transition_ledger.rs:1030:    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
src/bottom_white/ledger/transition_ledger.rs:1031:    // (c) signature does NOT verify after mutating a signed field (parent_ledger_root — K2 transplant defense).
src/bottom_white/ledger/transition_ledger.rs:1033:    fn signature_round_trip_and_transplant_defense() {
src/bottom_white/ledger/transition_ledger.rs:1036:            SystemEpoch, verify_system_signature,
src/bottom_white/ledger/transition_ledger.rs:1065:            verify_system_signature(&sig, &msg_clean, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:1066:            "clean signature must verify"
src/bottom_white/ledger/transition_ledger.rs:1075:            !verify_system_signature(&sig, &msg_tamper, epoch, &pinned),
src/bottom_white/ledger/transition_ledger.rs:1076:            "transplanted parent_ledger_root MUST fail signature verify (K2)"

exec
/bin/bash -lc "rg --line-number \"verify_agent_signature\" src tests experiments | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 121ms:
src/state/typed_tx.rs:813:// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
src/runtime/agent_keypairs.rs:19://! | Verifier            | `verify_system_signature`    | `verify_agent_signature` (here)   |
src/runtime/agent_keypairs.rs:391:pub fn verify_agent_signature(
src/runtime/agent_keypairs.rs:464:        assert!(verify_agent_signature(&sig, &digest, &kp.public_key()).is_ok());
src/runtime/agent_keypairs.rs:492:        assert!(verify_agent_signature(&sig1, &fresh_digest(2), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:493:        assert!(verify_agent_signature(&sig2, &fresh_digest(3), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:534:        assert!(verify_agent_signature(&sig, &digest, &kp2.public_key()).is_err());
src/runtime/agent_keypairs.rs:597:            verify_agent_signature(&sig_b, &fresh_digest(21), &pubkey_b).is_ok(),
src/runtime/adapter.rs:613:        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/adapter.rs:650:        verify_agent_signature(&work.signature, &digest, &pubkey).expect("verify");
src/runtime/verify.rs:377:    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/verify.rs:415:                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:436:                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
tests/tb_7_authoritative_routing.rs:37:    verify_agent_signature, AgentKeypairRegistry, AgentPubkeyManifest,
tests/tb_7_authoritative_routing.rs:113:    verify_agent_signature(&signature, &digest, &pubkey)
tests/tb_7_authoritative_routing.rs:296:    verify_agent_signature(&verify.signature, &digest, &pubkey)

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '360,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   360	}
   361	
   362	/// TRACE_MATRIX FC1-N14: TB-7 Atom 4 — verify Gate 4 + Gate 5 indicators by
   363	/// walking every L4 entry and (for WorkTx / VerifyTx variants) re-verifying
   364	/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
   365	/// checking that every `WorkTx.proposal_cid` resolves to a CAS-resident
   366	/// ProposalTelemetry object.
   367	///
   368	/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
   369	/// Both default to `true` when the manifest doesn't exist or when no
   370	/// Work/Verify entries are present (no signatures to verify ≠ failure).
   371	fn verify_agent_artifacts(
   372	    runtime_repo_path: &Path,
   373	    cas_store: &CasStore,
   374	    entries: &[LedgerEntry],
   375	) -> (bool, bool) {
   376	    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
   377	    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
   378	    use crate::runtime::proposal_telemetry::read_from_cas as read_telemetry;
   379	    use crate::state::typed_tx::TypedTx;
   380	
   381	    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
   382	    if !manifest_path.exists() {
   383	        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
   384	        // trivially true since there are no agent-side artifacts to fail.
   385	        return (true, true);
   386	    }
   387	    let manifest = match AgentPubkeyManifest::load(&manifest_path) {
   388	        Ok(m) => m,
   389	        Err(_) => return (false, false), // manifest unparseable = both fail
   390	    };
   391	
   392	    let mut agent_signatures_verified = true;
   393	    let mut proposal_telemetry_cas_retrievable = true;
   394	
   395	    for entry in entries {
   396	        // Get the typed payload from CAS.
   397	        let payload_bytes = match cas_store.get(&entry.tx_payload_cid) {
   398	            Ok(b) => b,
   399	            Err(_) => continue, // cas_payloads_retrievable already covers this
   400	        };
   401	        let typed_tx: TypedTx = match canonical_decode(&payload_bytes) {
   402	            Ok(tx) => tx,
   403	            Err(_) => continue, // payload decode error already covered upstream
   404	        };
   405	
   406	        match &typed_tx {
   407	            TypedTx::Work(work) => {
   408	                // Gate 4 — verify WorkTx signature.
   409	                let payload = work.to_signing_payload();
   410	                let digest = payload.canonical_digest();
   411	                let pubkey_opt = manifest.get(&work.agent_id);
   412	                match pubkey_opt {
   413	                    None => agent_signatures_verified = false,
   414	                    Some(pubkey) => {
   415	                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
   416	                            agent_signatures_verified = false;
   417	                        }
   418	                    }
   419	                }
   420	                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
   421	                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
   422	                if work.proposal_cid.0 != [0u8; 32] {
   423	                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
   424	                        proposal_telemetry_cas_retrievable = false;
   425	                    }
   426	                }
   427	            }
   428	            TypedTx::Verify(verify) => {
   429	                // Gate 4 — verify VerifyTx signature.
   430	                let payload = verify.to_signing_payload();
   431	                let digest = payload.canonical_digest();
   432	                let pubkey_opt = manifest.get(&verify.verifier_agent);
   433	                match pubkey_opt {
   434	                    None => agent_signatures_verified = false,
   435	                    Some(pubkey) => {
   436	                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
   437	                            agent_signatures_verified = false;
   438	                        }
   439	                    }
   440	                }
   441	            }
   442	            // Other tx variants (TaskOpen / EscrowLock / Challenge /
   443	            // ChallengeResolve / ReuseTx / FinalizeReward / TaskExpire /
   444	            // TerminalSummary) are not covered by Gate 4 because:
   445	            // - Some are system-emitted (signature path is system, not agent;
   446	            //   covered by system_signatures_verified above).
   447	            // - Others are agent-emitted but their signing payloads need
   448	            //   per-variant signing helpers (TB-7 scope is WorkTx + VerifyTx
   449	            //   per ARCHITECT_RULING D3 narrowed scope).
   450	            _ => {}
   451	        }
   452	    }
   453	
   454	    (
   455	        agent_signatures_verified,
   456	        proposal_telemetry_cas_retrievable,
   457	    )
   458	}
   459	
   460	// ── Helpers ─────────────────────────────────────────────────────────────────
   461	
   462	fn decode_pubkey_hex(hex: &str) -> Result<Vec<u8>, VerifyError> {
   463	    if hex.len() % 2 != 0 {
   464	        return Err(VerifyError::PubkeyDecode(format!(
   465	            "odd-length hex string ({})",
   466	            hex.len()
   467	        )));
   468	    }
   469	    let mut out = Vec::with_capacity(hex.len() / 2);
   470	    for chunk in hex.as_bytes().chunks(2) {

codex
The main TB-13 balance math is visible now. One high-risk side issue emerged: the new variants are described as agent-signed, but the apply path I found only live-verifies system signatures; I’m checking whether there is any agent-signature admission path for these variants before deciding severity.
exec
/bin/bash -lc "nl -ba src/economy/money.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! `MicroCoin(i64)` — v4 monetary unit per Plan v3.2-fix3 CO1.0a + STATE_TRANSITION_SPEC v1.3.
     2	//!
     3	//! Constitution authority:
     4	//! - Laws 基本法 1 (Coin 守恒): monetary conservation MUST be exact
     5	//! - Inv 3 (escrow only): payouts come from pre-locked escrow; integer arithmetic prevents drift
     6	//! - Inv 4 (no post-init mint): mint API guarded; only genesis sets initial supply
     7	//!
     8	//! Spec authority:
     9	//! - STATE_TRANSITION_SPEC v1.3 § 1 typed schemas — all monetary fields are MicroCoin
    10	//! - § 2 hidden-input table: f64 BANNED in `src/economy/`
    11	//! - § 3.4 finalize_reward stage 3c royalty math: `royalty_micro = reward_micro * weight_micro / 1_000_000` (integer floor)
    12	//!
    13	//! Unit: 1 MicroCoin = 10⁻⁶ base coin. Range: i64 = ±9.2 × 10¹⁸ micro = ±9.2 × 10¹² base coin.
    14	//!
    15	//! Design:
    16	//! - Newtype around i64 to prevent accidental mixing with u64/u32/f64
    17	//! - All arithmetic returns Option (checked); panics not allowed in production paths
    18	//! - Display formats as base.fraction (e.g., "12.345678 coin")
    19	//! - serde-compatible for L4 transition_tx serialization
    20	//! - Hash + Ord + Eq for use as BTreeMap key (per § 2 I-BTREE)
    21	//!
    22	//! /// TRACE_MATRIX I-MICROCOIN + Inv-3 + Inv-4: monetary type for v4
    23	
    24	use serde::{Deserialize, Serialize};
    25	use std::fmt;
    26	
    27	/// A monetary value in micro-coin (10⁻⁶ base coin) as a signed 64-bit integer.
    28	///
    29	/// Negative values are allowed at the type level (e.g., signed deltas in tests),
    30	/// but balance / escrow / stake fields enforce non-negative invariants at the
    31	/// business logic layer (not in this type).
    32	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    33	#[serde(transparent)]
    34	pub struct MicroCoin(i64);
    35	
    36	/// 1 base coin in micro-units (= 10⁶).
    37	pub const MICRO_PER_COIN: i64 = 1_000_000;
    38	
    39	impl MicroCoin {
    40	    /// Construct from raw micro-units (signed).
    41	    pub const fn from_micro_units(micro: i64) -> Self {
    42	        Self(micro)
    43	    }
    44	
    45	    /// Construct from whole base coin (multiplied by `MICRO_PER_COIN`); `None` on overflow.
    46	    pub fn from_coin(coin: i64) -> Option<Self> {
    47	        coin.checked_mul(MICRO_PER_COIN).map(Self)
    48	    }
    49	
    50	    /// Zero (additive identity).
    51	    pub const fn zero() -> Self {
    52	        Self(0)
    53	    }
    54	
    55	    /// Get raw micro-units (signed i64).
    56	    pub const fn micro_units(self) -> i64 {
    57	        self.0
    58	    }
    59	
    60	    /// Whole base-coin component (truncates toward zero).
    61	    pub const fn coin_component(self) -> i64 {
    62	        self.0 / MICRO_PER_COIN
    63	    }
    64	
    65	    /// Fractional micro component in `[-999_999, 999_999]` (sign matches whole).
    66	    pub const fn micro_fraction_component(self) -> i64 {
    67	        self.0 % MICRO_PER_COIN
    68	    }
    69	
    70	    /// Checked addition. `None` on overflow.
    71	    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
    72	        match self.0.checked_add(rhs.0) {
    73	            Some(v) => Some(Self(v)),
    74	            None => None,
    75	        }
    76	    }
    77	
    78	    /// Checked subtraction. `None` on overflow.
    79	    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
    80	        match self.0.checked_sub(rhs.0) {
    81	            Some(v) => Some(Self(v)),
    82	            None => None,
    83	        }
    84	    }
    85	
    86	    /// Royalty / weight multiplication with integer-floor rounding.
    87	    ///
    88	    /// Implements the spec § 3.4 stage 3c rule:
    89	    ///     royalty_micro = reward_micro × weight_micro_fraction / 1_000_000
    90	    ///
    91	    /// `weight_micro_fraction` is interpreted as a rational in `[0.0, 1.0]`
    92	    /// scaled to micro-units (`1_000_000` = 1.0). Returns `None` on overflow
    93	    /// at the intermediate `reward_micro × weight` product step.
    94	    ///
    95	    /// Determinism: integer floor (`a / b` rounds toward zero in Rust). For
    96	    /// non-negative inputs this is round-down (floor). Negative values are
    97	    /// rejected (returns `None`) to keep monetary math non-negative-by-default.
    98	    pub fn checked_mul_floor_micro(self, weight_micro_fraction: i64) -> Option<Self> {
    99	        if self.0 < 0 || weight_micro_fraction < 0 {
   100	            return None;
   101	        }
   102	        if weight_micro_fraction > MICRO_PER_COIN {
   103	            // weight > 1.0 not allowed at type level
   104	            return None;
   105	        }
   106	        let prod = self.0.checked_mul(weight_micro_fraction)?;
   107	        Some(Self(prod / MICRO_PER_COIN))
   108	    }
   109	
   110	    /// True if value is zero.
   111	    pub const fn is_zero(self) -> bool {
   112	        self.0 == 0
   113	    }
   114	
   115	    /// True if value is strictly positive.
   116	    pub const fn is_positive(self) -> bool {
   117	        self.0 > 0
   118	    }
   119	
   120	    /// True if value is strictly negative.
   121	    pub const fn is_negative(self) -> bool {
   122	        self.0 < 0
   123	    }
   124	}
   125	
   126	impl fmt::Display for MicroCoin {
   127	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   128	        let coin = self.0 / MICRO_PER_COIN;
   129	        let frac = (self.0 % MICRO_PER_COIN).abs();
   130	        if self.0 < 0 && coin == 0 {
   131	            write!(f, "-0.{:06} coin", frac)
   132	        } else {
   133	            write!(f, "{}.{:06} coin", coin, frac)
   134	        }
   135	    }
   136	}
   137	
   138	// ────────────────────────────────────────────────────────────────────────────
   139	// StakeMicroCoin — newtype for stake fields per CO1.1.4-pre1 § 3
   140	// ────────────────────────────────────────────────────────────────────────────
   141	
   142	/// Newtype on `MicroCoin` for `WorkTx::stake`, `VerifyTx::bond`, `ChallengeTx::stake`
   143	/// fields. Non-negative is a runtime invariant per Inv 3 (escrow only); the
   144	/// type-level newtype prevents accidental mixing with general-purpose
   145	/// `MicroCoin` (e.g. crediting a balance with a stake amount or vice versa).
   146	///
   147	/// `#[serde(transparent)]` — wire format identical to `MicroCoin`, so adding
   148	/// the newtype is non-breaking for canonical encoding.
   149	///
   150	/// /// TRACE_MATRIX I-MICROCOIN + I-STAKE: stake-typed monetary newtype.
   151	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   152	#[serde(transparent)]
   153	pub struct StakeMicroCoin(pub MicroCoin);
   154	
   155	impl Default for StakeMicroCoin {
   156	    fn default() -> Self {
   157	        Self::zero()
   158	    }
   159	}
   160	
   161	impl StakeMicroCoin {
   162	    pub const fn from_micro_units(micro: i64) -> Self {
   163	        Self(MicroCoin::from_micro_units(micro))
   164	    }
   165	    pub const fn zero() -> Self {
   166	        Self(MicroCoin::zero())
   167	    }
   168	    pub const fn micro_units(self) -> i64 {
   169	        self.0.micro_units()
   170	    }
   171	    pub const fn as_micro_coin(self) -> MicroCoin {
   172	        self.0
   173	    }
   174	}
   175	
   176	impl fmt::Display for StakeMicroCoin {
   177	    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
   178	        write!(f, "stake({})", self.0)
   179	    }
   180	}

exec
/bin/bash -lc "rg --line-number \"pub fn sign|verify_agent_signature|AgentPubkeyManifest|agent_pubkeys|submit_agent_tx|CompleteSetMint|MarketSeed|CompleteSetRedeem\" src/runtime src/state tests | sed -n '1,280p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_8_minimal_payout.rs:581:/// submit_agent_tx pre-queue with SystemTxForbiddenOnAgentIngress).
tests/system_keypair_sign_only_from_runner.rs:14:        !src.contains("pub fn sign_system_message"),
tests/tb_7_atom6_chain_backed_smoke.rs:203:        "Gate 4: every WorkTx + VerifyTx signature must verify against agent_pubkeys.json — {report:?}"
tests/tb_7_atom6_chain_backed_smoke.rs:250:        let agent_pubkeys_src = cfg.runtime_repo_path.join("agent_pubkeys.json");
tests/tb_7_atom6_chain_backed_smoke.rs:251:        if agent_pubkeys_src.exists() {
tests/tb_7_atom6_chain_backed_smoke.rs:252:            let _ = std::fs::copy(&agent_pubkeys_src, evidence_dir.join("agent_pubkeys.json"));
tests/tb_7_atom6_chain_backed_smoke.rs:270:                 - agent_pubkeys.json: {agents} agents pinned\n\
tests/tb_7_atom6_chain_backed_smoke.rs:276:                 3. **Gate 4** (agent signatures): every WorkTx + VerifyTx signature verifies against agent_pubkeys.json on replay.\n\
tests/tb_13_legacy_cpmm_forward_fence.rs:11://!   HALT if f64 appears in new CompleteSet / MarketSeed code.
tests/tb_13_legacy_cpmm_forward_fence.rs:199:/// CompleteSet / MarketSeed code. Money-path types must use integer
tests/tb_13_legacy_cpmm_forward_fence.rs:260:        "CompleteSetMintTx",
tests/tb_7_authoritative_routing.rs:15://!   `<runtime_repo>/agent_pubkeys.json` (Atom 4 verify_chaintape will
tests/tb_7_authoritative_routing.rs:37:    verify_agent_signature, AgentKeypairRegistry, AgentPubkeyManifest,
tests/tb_7_authoritative_routing.rs:101:        AgentPubkeyManifest::load(reg.manifest_path()).expect("load agent_pubkeys.json");
tests/tb_7_authoritative_routing.rs:113:    verify_agent_signature(&signature, &digest, &pubkey)
tests/tb_7_authoritative_routing.rs:127:    // AgentKeypairRegistry uses an `agent_pubkeys.json` distinct from
tests/tb_7_authoritative_routing.rs:292:    let manifest = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
tests/tb_7_authoritative_routing.rs:296:    verify_agent_signature(&verify.signature, &digest, &pubkey)
tests/tb_7_authoritative_routing.rs:338:    let manifest = AgentPubkeyManifest::load(reg.manifest_path()).expect("load manifest");
tests/tb_13_complete_set.rs:1://! TB-13 Atom 5 integration tests — CompleteSet + MarketSeedTx per architect
tests/tb_13_complete_set.rs:4://! "CompleteSet + MarketSeedTx" — Polymarket / CTF conditional-share
tests/tb_13_complete_set.rs:13://! NOT counted as Coin / MarketSeed without provider balance / no
tests/tb_13_complete_set.rs:55:    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId,
tests/tb_13_complete_set.rs:56:    MarketSeedTx, OutcomeSide, ResolutionRef, ShareAmount, TypedTx,
tests/tb_13_complete_set.rs:116:        .submit_agent_tx(tx)
tests/tb_13_complete_set.rs:129:    TypedTx::CompleteSetMint(CompleteSetMintTx {
tests/tb_13_complete_set.rs:148:    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
tests/tb_13_complete_set.rs:171:    TypedTx::MarketSeed(MarketSeedTx {
tests/tb_13_complete_set.rs:273:/// SG-13.3 — MarketSeedTx fails if provider lacks balance.
tests/tb_13_complete_set.rs:292:/// SG-13.4 — MarketSeedTx cannot create liquidity without collateral
tests/tb_13_complete_set.rs:315:/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
tests/tb_13_complete_set.rs:451:// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
tests/tb_13_complete_set.rs:535:/// Halt: MarketSeed with zero-balance provider rejected (regression
tests/tb_5_system_ingress_barrier.rs:17://! reach the queue through `Sequencer::submit` or `Sequencer::submit_agent_tx`.
tests/tb_5_system_ingress_barrier.rs:95:// I60 — agent-ingress rejects ChallengeResolveTx via Sequencer::submit_agent_tx
tests/tb_5_system_ingress_barrier.rs:114:    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
tests/tb_5_system_ingress_barrier.rs:129:// I61 — agent-ingress rejects FinalizeRewardTx via Sequencer::submit_agent_tx
tests/tb_5_system_ingress_barrier.rs:149:    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
tests/tb_5_system_ingress_barrier.rs:163:// I62 — agent-ingress rejects TaskExpireTx via Sequencer::submit_agent_tx
tests/tb_5_system_ingress_barrier.rs:184:    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
tests/tb_5_system_ingress_barrier.rs:193:// I63 — agent-ingress rejects TerminalSummaryTx via Sequencer::submit_agent_tx
tests/tb_5_system_ingress_barrier.rs:215:    let err = h.seq.submit_agent_tx(tx).await.unwrap_err();
tests/tb_5_system_ingress_barrier.rs:224:// I67 — legacy `Sequencer::submit` alias delegates to `submit_agent_tx`
tests/tb_5_system_ingress_barrier.rs:229:async fn legacy_submit_alias_delegates_to_submit_agent_tx_and_rejects_system_variants() {
tests/tb_5_system_ingress_barrier.rs:287:            "legacy submit() must inherit submit_agent_tx rejection; got {err:?}"
tests/tb_5_system_ingress_barrier.rs:294:        "legacy submit() must reject pre-queue (no submit_id burn) just like submit_agent_tx");
tests/tb_12_node_exposure_index.rs:108:    h.seq.submit_agent_tx(tx).await.expect("submit open");
tests/tb_12_node_exposure_index.rs:123:    h.seq.submit_agent_tx(tx).await.expect("submit lock");
tests/tb_12_node_exposure_index.rs:157:    h.seq.submit_agent_tx(TypedTx::Work(work)).await.expect("submit work");
tests/tb_12_node_exposure_index.rs:181:    h.seq.submit_agent_tx(tx).await.expect("submit challenge");
tests/tb_12_node_exposure_index.rs:205:    h.seq.submit_agent_tx(tx).await.expect("submit verify");
tests/tb_11_epistemic_exhaust.rs:104:    h.seq.submit_agent_tx(open_tx).await.expect("submit open");
tests/tb_11_epistemic_exhaust.rs:117:    h.seq.submit_agent_tx(lock_tx).await.expect("submit lock");
src/state/sequencer.rs:255:/// §4.3): CompleteSetMint-accept state-root domain.
src/state/sequencer.rs:259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
src/state/sequencer.rs:270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
src/state/sequencer.rs:275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
src/state/sequencer.rs:286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
src/state/sequencer.rs:291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
src/state/sequencer.rs:447:        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
src/state/sequencer.rs:455:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:456:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:457:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:478:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:479:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:480:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:506:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:507:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:508:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
src/state/sequencer.rs:1582:        TypedTx::CompleteSetMint(mint) => {
src/state/sequencer.rs:1654:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
src/state/sequencer.rs:1669:        TypedTx::CompleteSetRedeem(redeem) => {
src/state/sequencer.rs:1798:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
src/state/sequencer.rs:1802:        TypedTx::MarketSeed(seed) => {
src/state/sequencer.rs:1987:    /// payload). NOT agent-submittable — `submit_agent_tx` rejects every
src/state/sequencer.rs:2319:    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
src/state/sequencer.rs:2335:            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
src/state/sequencer.rs:2343:            | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:2344:            | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:2345:            | TypedTx::MarketSeed(_) => {}
src/state/sequencer.rs:2366:    /// Anti-Oreo guarantee that complements the agent-side submit_agent_tx
src/state/sequencer.rs:2683:    /// Submit a typed transition (legacy alias; delegates to `submit_agent_tx`
src/state/sequencer.rs:2690:    /// inherits `submit_agent_tx`'s system-variant rejection. Existing
src/state/sequencer.rs:2698:        self.submit_agent_tx(tx).await
src/state/sequencer.rs:4007:    // When Atom 3 lands ChallengeResolveTx, it extends the submit_agent_tx
src/state/sequencer.rs:4010:    /// U23 — submit_agent_tx rejects FinalizeRewardTx pre-queue with
src/state/sequencer.rs:4014:    async fn submit_agent_tx_rejects_finalize_reward_pre_queue() {
src/state/sequencer.rs:4028:        let err = seq.submit_agent_tx(tx).await.unwrap_err();
src/state/sequencer.rs:4035:    /// U24 — submit_agent_tx rejects TaskExpireTx pre-queue.
src/state/sequencer.rs:4037:    async fn submit_agent_tx_rejects_task_expire_pre_queue() {
src/state/sequencer.rs:4052:        let err = seq.submit_agent_tx(tx).await.unwrap_err();
src/state/sequencer.rs:4057:    /// U25 — submit_agent_tx rejects TerminalSummaryTx pre-queue.
src/state/sequencer.rs:4059:    async fn submit_agent_tx_rejects_terminal_summary_pre_queue() {
src/state/sequencer.rs:4075:        let err = seq.submit_agent_tx(tx).await.unwrap_err();
src/state/sequencer.rs:4080:    /// U22 — submit_agent_tx rejects ChallengeResolveTx pre-queue
src/state/sequencer.rs:4084:    async fn submit_agent_tx_rejects_challenge_resolve_pre_queue() {
src/state/sequencer.rs:4097:        let err = seq.submit_agent_tx(tx).await.unwrap_err();
src/state/sequencer.rs:4104:    /// U26 — submit_agent_tx accepts all 6 agent-submitted variants
src/state/sequencer.rs:4108:    async fn submit_agent_tx_accepts_work_verify_challenge_taskopen_escrowlock_reuse() {
src/state/sequencer.rs:4112:        let r = seq.submit_agent_tx(TypedTx::Work(fixture_work_tx())).await;
src/state/sequencer.rs:4116:        let r = seq.submit_agent_tx(TypedTx::Verify(VerifyTx {
src/state/sequencer.rs:4129:        let r = seq.submit_agent_tx(TypedTx::Challenge(ChallengeTx {
src/state/sequencer.rs:4142:        let r = seq.submit_agent_tx(TypedTx::Reuse(ReuseTx {
src/state/sequencer.rs:4153:        let r = seq.submit_agent_tx(TypedTx::TaskOpen(TaskOpenTx {
src/state/sequencer.rs:4168:        let r = seq.submit_agent_tx(TypedTx::EscrowLock(EscrowLockTx {
src/state/typed_tx.rs:332:/// **System-emitted only**: agent ingress (`Sequencer::submit_agent_tx`)
src/state/typed_tx.rs:603://   No CompleteSet / MarketSeedTx / AMM / CPMM (TB-13/14 territory).
src/state/typed_tx.rs:752://   - System-emitted ONLY: agent ingress (`Sequencer::submit_agent_tx`) rejects
src/state/typed_tx.rs:813:// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
src/state/typed_tx.rs:828:// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
src/state/typed_tx.rs:1057:// § 5c-TB-13 — CompleteSet + MarketSeedTx conditional shares
src/state/typed_tx.rs:1063:// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
src/state/typed_tx.rs:1064:// mints equal YES_E + NO_E shares to the same owner. `CompleteSetRedeemTx`
src/state/typed_tx.rs:1066:// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
src/state/typed_tx.rs:1125:/// embedded in `CompleteSetRedeemTx`. References either a
src/state/typed_tx.rs:1150:pub struct CompleteSetMintTx {
src/state/typed_tx.rs:1177:pub struct CompleteSetRedeemTx {
src/state/typed_tx.rs:1203:/// The shape is identical to `CompleteSetMintTx` post-effect; the
src/state/typed_tx.rs:1209:pub struct MarketSeedTx {
src/state/typed_tx.rs:1222:/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1224:pub struct CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1233:impl CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1235:    /// canonical digest for agent-signed CompleteSetMintTx. Domain
src/state/typed_tx.rs:1245:/// `CompleteSetRedeemTx` (9 fields → 8 fields; signature excluded).
src/state/typed_tx.rs:1247:pub struct CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1258:impl CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1260:    /// canonical digest for agent-signed CompleteSetRedeemTx. Domain
src/state/typed_tx.rs:1268:/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1270:pub struct MarketSeedSigningPayload {
src/state/typed_tx.rs:1279:impl MarketSeedSigningPayload {
src/state/typed_tx.rs:1281:    /// canonical digest for agent-signed MarketSeedTx. Domain prefix
src/state/typed_tx.rs:1448:impl CompleteSetMintTx {
src/state/typed_tx.rs:1451:    pub fn to_signing_payload(&self) -> CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1452:        CompleteSetMintSigningPayload {
src/state/typed_tx.rs:1463:impl CompleteSetRedeemTx {
src/state/typed_tx.rs:1466:    pub fn to_signing_payload(&self) -> CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1467:        CompleteSetRedeemSigningPayload {
src/state/typed_tx.rs:1480:impl MarketSeedTx {
src/state/typed_tx.rs:1483:    pub fn to_signing_payload(&self) -> MarketSeedSigningPayload {
src/state/typed_tx.rs:1484:        MarketSeedSigningPayload {
src/state/typed_tx.rs:1520:    CompleteSetMint(CompleteSetMintTx),   // TB-13 agent-signed conditional-share mint
src/state/typed_tx.rs:1521:    CompleteSetRedeem(CompleteSetRedeemTx), // TB-13 agent-signed conditional-share redeem
src/state/typed_tx.rs:1522:    MarketSeed(MarketSeedTx),             // TB-13 agent-signed protocol-owned share seed
src/state/typed_tx.rs:1541:            Self::CompleteSetMint(_) => TxKind::CompleteSetMint,
src/state/typed_tx.rs:1542:            Self::CompleteSetRedeem(_) => TxKind::CompleteSetRedeem,
src/state/typed_tx.rs:1543:            Self::MarketSeed(_) => TxKind::MarketSeed,
src/state/typed_tx.rs:1628:impl HasSubmitter for CompleteSetMintTx {
src/state/typed_tx.rs:1634:impl HasSubmitter for CompleteSetRedeemTx {
src/state/typed_tx.rs:1640:impl HasSubmitter for MarketSeedTx {
src/state/typed_tx.rs:1660:            Self::CompleteSetMint(t) => t.submitter_id(),
src/state/typed_tx.rs:1661:            Self::CompleteSetRedeem(t) => t.submitter_id(),
src/state/typed_tx.rs:1662:            Self::MarketSeed(t) => t.submitter_id(),
src/state/typed_tx.rs:1813:    /// rejection happens at `Sequencer::submit_agent_tx` BEFORE dispatch
src/state/typed_tx.rs:1816:    /// any code path bypass the submit_agent_tx barrier and surface a
src/state/typed_tx.rs:1849:    /// `CompleteSetMintTx` admission: `balances_t[owner] < amount`.
src/state/typed_tx.rs:1853:    /// `CompleteSetRedeemTx` admission: the referenced event is in
src/state/typed_tx.rs:1859:    /// `CompleteSetRedeemTx` admission: the owner's
src/state/typed_tx.rs:1864:    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
src/state/typed_tx.rs:1866:    /// collateral. Also fired defensively at `CompleteSetRedeemTx` time
src/state/typed_tx.rs:1871:    /// `CompleteSetRedeemTx` admission: the resolution_ref's
src/state/typed_tx.rs:1926:                 is Sequencer::submit_agent_tx pre-queue)"
src/state/typed_tx.rs:1945:                "CompleteSetMintTx: owner's balances_t entry is below the requested mint amount"
src/state/typed_tx.rs:1949:                "CompleteSetRedeemTx: event task_markets_t state is Open or Expired (no system-emitted resolution yet)"
src/state/typed_tx.rs:1953:                "CompleteSetRedeemTx: owner's conditional share balance is below the requested redeem amount"
src/state/typed_tx.rs:1957:                "TB-13 collateral missing: MarketSeed with zero collateral, or Redeem against insufficient conditional_collateral_t"
src/state/typed_tx.rs:1961:                "CompleteSetRedeemTx: resolution_ref.claimed_outcome does not match task_markets_t[event_id.0] state"
src/state/typed_tx.rs:3099:    // TB-13 Atom 1 unit tests — CompleteSetMint / CompleteSetRedeem /
src/state/typed_tx.rs:3100:    // MarketSeed (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
src/state/typed_tx.rs:3103:    fn fixture_complete_set_mint_tx() -> CompleteSetMintTx {
src/state/typed_tx.rs:3104:        CompleteSetMintTx {
src/state/typed_tx.rs:3115:    fn fixture_complete_set_redeem_tx() -> CompleteSetRedeemTx {
src/state/typed_tx.rs:3116:        CompleteSetRedeemTx {
src/state/typed_tx.rs:3132:    fn fixture_market_seed_tx() -> MarketSeedTx {
src/state/typed_tx.rs:3133:        MarketSeedTx {
src/state/typed_tx.rs:3144:    /// TB-13 U1: CompleteSetMintTx round-trips through canonical encode.
src/state/typed_tx.rs:3147:        let tx = TypedTx::CompleteSetMint(fixture_complete_set_mint_tx());
src/state/typed_tx.rs:3150:        assert_eq!(tx, decoded, "CompleteSetMintTx round-trip mismatch");
src/state/typed_tx.rs:3153:            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetMint,
src/state/typed_tx.rs:3157:    /// TB-13 U2: CompleteSetRedeemTx round-trips through canonical encode.
src/state/typed_tx.rs:3160:        let tx = TypedTx::CompleteSetRedeem(fixture_complete_set_redeem_tx());
src/state/typed_tx.rs:3163:        assert_eq!(tx, decoded, "CompleteSetRedeemTx round-trip mismatch");
src/state/typed_tx.rs:3166:            crate::bottom_white::ledger::transition_ledger::TxKind::CompleteSetRedeem,
src/state/typed_tx.rs:3170:    /// TB-13 U3: MarketSeedTx round-trips through canonical encode.
src/state/typed_tx.rs:3173:        let tx = TypedTx::MarketSeed(fixture_market_seed_tx());
src/state/typed_tx.rs:3176:        assert_eq!(tx, decoded, "MarketSeedTx round-trip mismatch");
src/state/typed_tx.rs:3179:            crate::bottom_white::ledger::transition_ledger::TxKind::MarketSeed,
src/state/typed_tx.rs:3204:        assert_eq!(mint_a, mint_b, "CompleteSetMint digest must be deterministic");
src/state/typed_tx.rs:3208:        assert_eq!(redeem_a, redeem_b, "CompleteSetRedeem digest must be deterministic");
src/state/typed_tx.rs:3212:        assert_eq!(seed_a, seed_b, "MarketSeed digest must be deterministic");
src/state/typed_tx.rs:3222:        assert_eq!(mint_o.len(), 6, "CompleteSetMintSigningPayload must have 6 fields");
src/state/typed_tx.rs:3228:        assert_eq!(redeem_o.len(), 8, "CompleteSetRedeemSigningPayload must have 8 fields");
src/state/typed_tx.rs:3234:        assert_eq!(seed_o.len(), 6, "MarketSeedSigningPayload must have 6 fields");
src/state/typed_tx.rs:3252:            TypedTx::CompleteSetMint(fixture_complete_set_mint_tx()).submitter_id(),
src/state/typed_tx.rs:3256:            TypedTx::MarketSeed(fixture_market_seed_tx()).submitter_id(),
src/runtime/adapter.rs:145:///    on-disk `agent_pubkeys.json` manifest (Atom 4 verify_chaintape
src/runtime/adapter.rs:613:        use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/adapter.rs:648:        let manifest = AgentPubkeyManifest::load(reg.manifest_path()).expect("load manifest");
src/runtime/adapter.rs:650:        verify_agent_signature(&work.signature, &digest, &pubkey).expect("verify");
src/runtime/mod.rs:47:/// TRACE_MATRIX § 3 orphan (see `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`): TB-7R Deliverable C — `genesis_report.json` emitter capturing constitution_hash + runtime_repo + cas_path + system_pubkey_hash + agent_pubkeys_path + initial_balances + (preseed only) task_id / task_open_tx / escrow_lock_tx. No canonical FC row exists yet (FC2 is Append/Submit, NOT Boot/Genesis); promotion target is a future TRACE_MATRIX revision under Article IV Boot. `FC-trace: Art.IV Boot + Art.I.1 + Art.III.4 + WP-§11`.
src/runtime/agent_keypairs.rs:17://! | Public manifest     | `pinned_pubkeys.json`        | `agent_pubkeys.json`              |
src/runtime/agent_keypairs.rs:19://! | Verifier            | `verify_system_signature`    | `verify_agent_signature` (here)   |
src/runtime/agent_keypairs.rs:149:    pub fn sign_digest(&self, digest: [u8; 32]) -> Result<AgentSignature, AgentKeypairError> {
src/runtime/agent_keypairs.rs:216:    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
src/runtime/agent_keypairs.rs:219:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:241:    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
src/runtime/agent_keypairs.rs:254:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:294:    pub fn sign(
src/runtime/agent_keypairs.rs:305:    pub fn manifest(&self) -> AgentPubkeyManifest {
src/runtime/agent_keypairs.rs:306:        AgentPubkeyManifest {
src/runtime/agent_keypairs.rs:357:/// TRACE_MATRIX FC1-N14: on-disk shape of `agent_pubkeys.json`.
src/runtime/agent_keypairs.rs:361:pub struct AgentPubkeyManifest {
src/runtime/agent_keypairs.rs:366:impl AgentPubkeyManifest {
src/runtime/agent_keypairs.rs:372:        let manifest: AgentPubkeyManifest = serde_json::from_slice(&buf)
src/runtime/agent_keypairs.rs:391:pub fn verify_agent_signature(
src/runtime/agent_keypairs.rs:425:                write!(f, "agent_pubkeys.json already exists at {path:?}")
src/runtime/agent_keypairs.rs:464:        assert!(verify_agent_signature(&sig, &digest, &kp.public_key()).is_ok());
src/runtime/agent_keypairs.rs:475:        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
src/runtime/agent_keypairs.rs:492:        assert!(verify_agent_signature(&sig1, &fresh_digest(2), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:493:        assert!(verify_agent_signature(&sig2, &fresh_digest(3), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:506:        let loaded = AgentPubkeyManifest::load(reg.manifest_path()).expect("load");
src/runtime/agent_keypairs.rs:534:        assert!(verify_agent_signature(&sig, &digest, &kp2.public_key()).is_err());
src/runtime/agent_keypairs.rs:597:            verify_agent_signature(&sig_b, &fresh_digest(21), &pubkey_b).is_ok(),
src/runtime/run_summary.rs:224:        TypedTx::CompleteSetMint(t) => t.tx_id.clone(), // TB-13
src/runtime/run_summary.rs:225:        TypedTx::CompleteSetRedeem(t) => t.tx_id.clone(), // TB-13
src/runtime/run_summary.rs:226:        TypedTx::MarketSeed(t) => t.tx_id.clone(),      // TB-13
src/runtime/genesis_report.rs:58:    pub agent_pubkeys_path: String,
src/runtime/genesis_report.rs:140:            agent_pubkeys_path: "agent_pubkeys.json".into(),
src/runtime/genesis_report.rs:191:            agent_pubkeys_path: "agent_pubkeys.json".into(),
src/runtime/verify.rs:139:    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
src/runtime/verify.rs:144:    /// signatures verify against agent_pubkeys.json. False on any
src/runtime/verify.rs:333:    // AgentSignature against the per-run agent_pubkeys.json manifest.
src/runtime/verify.rs:364:/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
src/runtime/verify.rs:377:    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/verify.rs:381:    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/verify.rs:383:        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
src/runtime/verify.rs:387:    let manifest = match AgentPubkeyManifest::load(&manifest_path) {
src/runtime/verify.rs:415:                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:436:                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {

exec
/bin/bash -lc "nl -ba tests/tb_13_complete_set.rs | sed -n '1,240p' && nl -ba tests/tb_13_complete_set.rs | sed -n '240,620p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-13 Atom 5 integration tests — CompleteSet + MarketSeedTx per architect
     2	//! 2026-05-03 post-TB-12 ruling Part A §4.4 SG-13.1..8 + halting triggers.
     3	//!
     4	//! "CompleteSet + MarketSeedTx" — Polymarket / CTF conditional-share
     5	//! substrate. **1 locked Coin = 1 YES_E + 1 NO_E.** TB-13 introduces
     6	//! conditional collateral + share balance accounting; redeem requires
     7	//! system-resolved task-market state (Finalized → Yes; Bankrupt → No).
     8	//! TB-13 does NOT introduce trading / pricing / AMM / orderbook —
     9	//! those are deferred to TB-14+.
    10	//!
    11	//! Coverage maps to architect SG-13.0..8 + halting triggers from
    12	//! charter §3 Atom 5 (total_supply_micro mutation correctness / shares
    13	//! NOT counted as Coin / MarketSeed without provider balance / no
    14	//! legacy CPMM / no f64 / no AMM/CPMM router).
    15	//!
    16	//! - SG-13.0.1 legacy_cpm_api_not_imported_by_complete_set       (Atom 0.5 fence)
    17	//! - SG-13.0.2 no_f64_in_complete_set_or_market_seed              (Atom 0.5 fence)
    18	//! - SG-13.0.3 prediction_market_legacy_quarantined               (Atom 0.5 fence)
    19	//! - SG-13.1   mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved
    20	//! - SG-13.2   yes_no_shares_not_in_total_coin_supply
    21	//! - SG-13.3   market_seed_fails_if_provider_lacks_balance
    22	//! - SG-13.4   market_seed_cannot_create_liquidity_without_collateral
    23	//! - SG-13.5   redeem_unavailable_before_outcome_resolution
    24	//! - SG-13.6   redeem_after_yes_outcome_pays_yes_not_no
    25	//! - SG-13.7   no_f64_in_new_complete_set_or_market_seed_path     (Atom 0.5 fence)
    26	//! - SG-13.8   no_import_or_use_of_legacy_cpmm_in_tb13_modules    (Atom 0.5 fence)
    27	//!
    28	//! /// TRACE_MATRIX TB-13 Atom 5 (architect 2026-05-03 post-TB-12 ruling
    29	//! Part A §4.4 + §4.7 forbidden list; SG-13.0..8).
    30	
    31	use std::collections::BTreeMap;
    32	use std::sync::{Arc, RwLock};
    33	
    34	use tempfile::TempDir;
    35	
    36	use turingosv4::bottom_white::cas::store::CasStore;
    37	use turingosv4::bottom_white::ledger::rejection_evidence::RejectionEvidenceWriter;
    38	use turingosv4::bottom_white::ledger::system_keypair::{
    39	    Ed25519Keypair, PinnedSystemPubkeys, SystemEpoch,
    40	};
    41	use turingosv4::bottom_white::ledger::transition_ledger::{
    42	    InMemoryLedgerWriter, LedgerWriter,
    43	};
    44	use turingosv4::bottom_white::tools::registry::ToolRegistry;
    45	use turingosv4::economy::money::MicroCoin;
    46	use turingosv4::economy::monetary_invariant::{
    47	    assert_complete_set_balanced, assert_total_ctf_conserved,
    48	};
    49	use turingosv4::state::q_state::{
    50	    AgentId, ConditionalCollateralIndex, ConditionalShareBalances, QState,
    51	    ShareSidePair, TaskId, TaskMarketEntry, TaskMarketState, TxId,
    52	};
    53	use turingosv4::state::sequencer::{Sequencer, SubmissionEnvelope};
    54	use turingosv4::state::typed_tx::{
    55	    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId,
    56	    MarketSeedTx, OutcomeSide, ResolutionRef, ShareAmount, TypedTx,
    57	};
    58	use turingosv4::top_white::predicates::registry::PredicateRegistry;
    59	
    60	// ── Harness ─────────────────────────────────────────────────────────────────
    61	
    62	struct Harness {
    63	    _tmp: TempDir,
    64	    seq: Sequencer,
    65	    rx: tokio::sync::mpsc::Receiver<SubmissionEnvelope>,
    66	    _ledger: Arc<RwLock<dyn LedgerWriter>>,
    67	}
    68	
    69	fn fresh_harness(initial_q: QState) -> Harness {
    70	    let tmp = TempDir::new().expect("tempdir");
    71	    let cas = Arc::new(RwLock::new(CasStore::open(tmp.path()).expect("cas")));
    72	    let keypair = Arc::new(Ed25519Keypair::generate_with_secure_entropy().expect("kp"));
    73	    let writer: Arc<RwLock<dyn LedgerWriter>> =
    74	        Arc::new(RwLock::new(InMemoryLedgerWriter::new()));
    75	    let rejection_writer = Arc::new(RwLock::new(RejectionEvidenceWriter::default()));
    76	    let preds = Arc::new(PredicateRegistry::new());
    77	    let tools = Arc::new(ToolRegistry::new());
    78	    let epoch = SystemEpoch::new(1);
    79	    let mut pinned = PinnedSystemPubkeys::new();
    80	    pinned.insert(epoch, keypair.public_key());
    81	    let pinned_pubkeys = Arc::new(pinned);
    82	    let (seq, rx) = Sequencer::new(
    83	        cas, keypair, epoch, writer.clone(), rejection_writer, preds, tools,
    84	        pinned_pubkeys, initial_q, 16,
    85	    );
    86	    Harness { _tmp: tmp, seq, rx, _ledger: writer }
    87	}
    88	
    89	fn genesis_with_balances(pairs: &[(&str, i64)]) -> QState {
    90	    let mut q = QState::genesis();
    91	    for (name, coin) in pairs {
    92	        q.economic_state_t.balances_t.0.insert(
    93	            AgentId((*name).into()),
    94	            MicroCoin::from_coin(*coin).unwrap(),
    95	        );
    96	    }
    97	    q
    98	}
    99	
   100	/// Pre-populate `task_markets_t[task]` with the given state. Used in
   101	/// SG-13.5 / SG-13.6 to simulate a system-emitted resolution (Finalized
   102	/// or Bankrupt) without going through the full FinalizeReward /
   103	/// TaskBankruptcy flow. The state-flip itself is exercised by TB-8 +
   104	/// TB-11 integration tests.
   105	fn seed_task_market(q: &mut QState, task: &str, state: TaskMarketState) {
   106	    let mut entry = TaskMarketEntry::default();
   107	    entry.state = state;
   108	    q.economic_state_t
   109	        .task_markets_t
   110	        .0
   111	        .insert(TaskId(task.into()), entry);
   112	}
   113	
   114	async fn submit_and_apply(h: &mut Harness, tx: TypedTx) -> Result<(), String> {
   115	    h.seq
   116	        .submit_agent_tx(tx)
   117	        .await
   118	        .map_err(|e| format!("submit error: {e:?}"))?;
   119	    let outcome = h
   120	        .seq
   121	        .try_apply_one(&mut h.rx)
   122	        .ok_or_else(|| "no envelope drained".to_string())?;
   123	    outcome
   124	        .map(|_ledger_entry| ())
   125	        .map_err(|e| format!("apply error: {e:?}"))
   126	}
   127	
   128	fn build_mint(parent: turingosv4::state::q_state::Hash, owner: &str, task: &str, micro: i64, seq_no: u64) -> TypedTx {
   129	    TypedTx::CompleteSetMint(CompleteSetMintTx {
   130	        tx_id: TxId(format!("mint-{owner}-{task}-{seq_no}")),
   131	        parent_state_root: parent,
   132	        event_id: EventId(TaskId(task.into())),
   133	        owner: AgentId(owner.into()),
   134	        amount: MicroCoin::from_micro_units(micro),
   135	        signature: AgentSignature::from_bytes([0u8; 64]),
   136	        timestamp_logical: 1000 + seq_no,
   137	    })
   138	}
   139	
   140	fn build_redeem(
   141	    parent: turingosv4::state::q_state::Hash,
   142	    owner: &str,
   143	    task: &str,
   144	    outcome: OutcomeSide,
   145	    units: u128,
   146	    seq_no: u64,
   147	) -> TypedTx {
   148	    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
   149	        tx_id: TxId(format!("redeem-{owner}-{task}-{seq_no}")),
   150	        parent_state_root: parent,
   151	        event_id: EventId(TaskId(task.into())),
   152	        owner: AgentId(owner.into()),
   153	        outcome,
   154	        share_amount: ShareAmount::from_units(units),
   155	        resolution_ref: ResolutionRef {
   156	            resolution_tx_id: TxId(format!("resolution-fixture-{task}")),
   157	            claimed_outcome: outcome,
   158	        },
   159	        signature: AgentSignature::from_bytes([0u8; 64]),
   160	        timestamp_logical: 2000 + seq_no,
   161	    })
   162	}
   163	
   164	fn build_seed(
   165	    parent: turingosv4::state::q_state::Hash,
   166	    provider: &str,
   167	    task: &str,
   168	    micro: i64,
   169	    seq_no: u64,
   170	) -> TypedTx {
   171	    TypedTx::MarketSeed(MarketSeedTx {
   172	        tx_id: TxId(format!("seed-{provider}-{task}-{seq_no}")),
   173	        parent_state_root: parent,
   174	        event_id: EventId(TaskId(task.into())),
   175	        provider: AgentId(provider.into()),
   176	        collateral_amount: MicroCoin::from_micro_units(micro),
   177	        signature: AgentSignature::from_bytes([0u8; 64]),
   178	        timestamp_logical: 3000 + seq_no,
   179	    })
   180	}
   181	
   182	// ── SG-13.1 ─────────────────────────────────────────────────────────────────
   183	
   184	/// SG-13.1 — Mint 1 Coin → 1 YES + 1 NO, total Coin conserved.
   185	#[tokio::test]
   186	async fn sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved() {
   187	    let q0 = genesis_with_balances(&[("alice", 100)]);
   188	    let mut h = fresh_harness(q0);
   189	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   190	
   191	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-A", 5_000_000, 1))
   192	        .await
   193	        .expect("mint accepted");
   194	
   195	    let q = h.seq.q_snapshot().unwrap();
   196	    let alice_bal = q
   197	        .economic_state_t
   198	        .balances_t
   199	        .0
   200	        .get(&AgentId("alice".into()))
   201	        .copied()
   202	        .unwrap();
   203	    assert_eq!(
   204	        alice_bal.micro_units(),
   205	        100_i64 * 1_000_000 - 5_000_000,
   206	        "alice balance must be debited by mint amount"
   207	    );
   208	
   209	    let collateral = q
   210	        .economic_state_t
   211	        .conditional_collateral_t
   212	        .0
   213	        .get(&EventId(TaskId("task-A".into())))
   214	        .copied()
   215	        .unwrap();
   216	    assert_eq!(collateral.micro_units(), 5_000_000, "collateral credited");
   217	
   218	    let pair = q
   219	        .economic_state_t
   220	        .conditional_share_balances_t
   221	        .0
   222	        .get(&AgentId("alice".into()))
   223	        .and_then(|m| m.get(&EventId(TaskId("task-A".into()))))
   224	        .copied()
   225	        .unwrap();
   226	    assert_eq!(pair.yes.units, 5_000_000_u128, "YES shares minted equal to amount");
   227	    assert_eq!(pair.no.units, 5_000_000_u128, "NO shares minted equal to amount");
   228	
   229	    // CTF preserved across mint via 6-holding sum (Atom 3 invariant).
   230	    let q_pre = QState::genesis();
   231	    let mut q_pre_balanced = q_pre.clone();
   232	    q_pre_balanced.economic_state_t.balances_t.0.insert(
   233	        AgentId("alice".into()),
   234	        MicroCoin::from_coin(100).unwrap(),
   235	    );
   236	    assert_total_ctf_conserved(
   237	        &q_pre_balanced.economic_state_t,
   238	        &q.economic_state_t,
   239	        &[],
   240	    )
   240	    )
   241	    .expect("CTF preserved across mint");
   242	    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
   243	}
   244	
   245	// ── SG-13.2 ─────────────────────────────────────────────────────────────────
   246	
   247	/// SG-13.2 — YES/NO shares are not counted in total Coin supply.
   248	///
   249	/// Asserts that `assert_total_ctf_conserved` passes pre/post a mint that
   250	/// creates 5_000_000 YES + 5_000_000 NO shares — if shares were
   251	/// double-counted as Coin, the post sum would be 10_000_000 micro larger
   252	/// than the pre sum and the assertion would fail.
   253	#[tokio::test]
   254	async fn sg_13_2_yes_no_shares_not_in_total_coin_supply() {
   255	    let q0 = genesis_with_balances(&[("alice", 50)]);
   256	    let mut h = fresh_harness(q0.clone());
   257	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   258	
   259	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-Z", 12_345_678, 2))
   260	        .await
   261	        .expect("mint accepted");
   262	
   263	    let q = h.seq.q_snapshot().unwrap();
   264	    // Pre/post 6-holding total must be equal — the conditional shares
   265	    // do NOT contribute to total_supply_micro per CR-13.3.
   266	    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
   267	        .expect("shares are not Coin; sum unchanged");
   268	    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
   269	}
   270	
   271	// ── SG-13.3 ─────────────────────────────────────────────────────────────────
   272	
   273	/// SG-13.3 — MarketSeedTx fails if provider lacks balance.
   274	#[tokio::test]
   275	async fn sg_13_3_market_seed_fails_if_provider_lacks_balance() {
   276	    // Bob has NO balance row at all.
   277	    let q0 = genesis_with_balances(&[("alice", 100)]);
   278	    let mut h = fresh_harness(q0);
   279	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   280	
   281	    let err = submit_and_apply(&mut h, build_seed(parent, "bob", "task-S", 1_000_000, 3))
   282	        .await
   283	        .expect_err("seed must fail without provider balance");
   284	    assert!(
   285	        err.contains("InsufficientBalanceForMint"),
   286	        "expected InsufficientBalanceForMint, got: {err}"
   287	    );
   288	}
   289	
   290	// ── SG-13.4 ─────────────────────────────────────────────────────────────────
   291	
   292	/// SG-13.4 — MarketSeedTx cannot create liquidity without collateral
   293	/// (architect §4.7 forbidden list "No automatic liquidity").
   294	#[tokio::test]
   295	async fn sg_13_4_market_seed_cannot_create_liquidity_without_collateral() {
   296	    let q0 = genesis_with_balances(&[("alice", 100)]);
   297	    let mut h = fresh_harness(q0);
   298	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   299	
   300	    // collateral_amount == 0 must fail with InsufficientCollateral.
   301	    let err = submit_and_apply(&mut h, build_seed(parent, "alice", "task-X", 0, 4))
   302	        .await
   303	        .expect_err("seed with zero collateral must fail");
   304	    assert!(
   305	        err.contains("InsufficientCollateral"),
   306	        "expected InsufficientCollateral, got: {err}"
   307	    );
   308	}
   309	
   310	// ── SG-13.5 ─────────────────────────────────────────────────────────────────
   311	
   312	/// SG-13.5 — Redeem unavailable before outcome resolution.
   313	///
   314	/// Mint shares; submit redeem when task_markets_t state is `Open`; expect
   315	/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
   316	/// is impossible before system-resolved outcome."
   317	#[tokio::test]
   318	async fn sg_13_5_redeem_unavailable_before_outcome_resolution() {
   319	    let mut q0 = genesis_with_balances(&[("alice", 100)]);
   320	    seed_task_market(&mut q0, "task-O", TaskMarketState::Open);
   321	    let mut h = fresh_harness(q0);
   322	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   323	
   324	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-O", 5_000_000, 5))
   325	        .await
   326	        .expect("mint accepted");
   327	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   328	
   329	    let err = submit_and_apply(
   330	        &mut h,
   331	        build_redeem(parent, "alice", "task-O", OutcomeSide::Yes, 1_000_000, 6),
   332	    )
   333	    .await
   334	    .expect_err("redeem before resolution must fail");
   335	    assert!(
   336	        err.contains("RedeemBeforeResolution"),
   337	        "expected RedeemBeforeResolution, got: {err}"
   338	    );
   339	
   340	    // Also: Expired state must reject (treated as no resolution).
   341	    let mut q1 = genesis_with_balances(&[("bob", 100)]);
   342	    seed_task_market(&mut q1, "task-E", TaskMarketState::Expired);
   343	    let mut h2 = fresh_harness(q1);
   344	    let parent = h2.seq.q_snapshot().unwrap().state_root_t;
   345	    submit_and_apply(&mut h2, build_mint(parent, "bob", "task-E", 2_000_000, 7))
   346	        .await
   347	        .expect("mint accepted");
   348	    let parent = h2.seq.q_snapshot().unwrap().state_root_t;
   349	    let err = submit_and_apply(
   350	        &mut h2,
   351	        build_redeem(parent, "bob", "task-E", OutcomeSide::No, 500_000, 8),
   352	    )
   353	    .await
   354	    .expect_err("redeem on expired must fail");
   355	    assert!(
   356	        err.contains("RedeemBeforeResolution"),
   357	        "expected RedeemBeforeResolution on Expired state, got: {err}"
   358	    );
   359	}
   360	
   361	// ── SG-13.6 ─────────────────────────────────────────────────────────────────
   362	
   363	/// SG-13.6 — Redeem after YES outcome pays YES, not NO.
   364	#[tokio::test]
   365	async fn sg_13_6_redeem_after_yes_outcome_pays_yes_not_no() {
   366	    let mut q0 = genesis_with_balances(&[("alice", 100)]);
   367	    seed_task_market(&mut q0, "task-Y", TaskMarketState::Finalized);
   368	    let mut h = fresh_harness(q0);
   369	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   370	
   371	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-Y", 4_000_000, 9))
   372	        .await
   373	        .expect("mint accepted");
   374	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   375	
   376	    submit_and_apply(
   377	        &mut h,
   378	        build_redeem(parent, "alice", "task-Y", OutcomeSide::Yes, 4_000_000, 10),
   379	    )
   380	    .await
   381	    .expect("redeem yes accepted");
   382	
   383	    let q = h.seq.q_snapshot().unwrap();
   384	    let alice_bal = q
   385	        .economic_state_t
   386	        .balances_t
   387	        .0
   388	        .get(&AgentId("alice".into()))
   389	        .copied()
   390	        .unwrap();
   391	    // 100 Coin = 100_000_000 micro; -4M (mint) +4M (yes redeem) = 100M unchanged.
   392	    assert_eq!(
   393	        alice_bal.micro_units(),
   394	        100_000_000_i64,
   395	        "alice balance restored after YES redeem"
   396	    );
   397	
   398	    let pair = q
   399	        .economic_state_t
   400	        .conditional_share_balances_t
   401	        .0
   402	        .get(&AgentId("alice".into()))
   403	        .and_then(|m| m.get(&EventId(TaskId("task-Y".into()))))
   404	        .copied()
   405	        .unwrap();
   406	    assert_eq!(pair.yes.units, 0_u128, "YES shares debited (winning side)");
   407	    assert_eq!(pair.no.units, 4_000_000_u128, "NO shares preserved (losing side)");
   408	
   409	    // Now attempt redeem outcome=No on the SAME finalized event — must fail
   410	    // because state is Finalized (YES wins) and the claimed_outcome is No.
   411	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   412	    let err = submit_and_apply(
   413	        &mut h,
   414	        build_redeem(parent, "alice", "task-Y", OutcomeSide::No, 1_000_000, 11),
   415	    )
   416	    .await
   417	    .expect_err("redeem outcome=No on Finalized event must fail");
   418	    assert!(
   419	        err.contains("InvalidResolutionRef"),
   420	        "expected InvalidResolutionRef, got: {err}"
   421	    );
   422	
   423	    // Symmetric check: Bankrupt event with outcome=Yes must fail; with
   424	    // outcome=No must succeed.
   425	    let mut q_b = genesis_with_balances(&[("bob", 50)]);
   426	    seed_task_market(&mut q_b, "task-B", TaskMarketState::Bankrupt);
   427	    let mut hb = fresh_harness(q_b);
   428	    let parent = hb.seq.q_snapshot().unwrap().state_root_t;
   429	    submit_and_apply(&mut hb, build_mint(parent, "bob", "task-B", 1_000_000, 12))
   430	        .await
   431	        .expect("mint accepted on bankrupt-future");
   432	    let parent = hb.seq.q_snapshot().unwrap().state_root_t;
   433	    let err = submit_and_apply(
   434	        &mut hb,
   435	        build_redeem(parent, "bob", "task-B", OutcomeSide::Yes, 500_000, 13),
   436	    )
   437	    .await
   438	    .expect_err("Bankrupt with outcome=Yes must reject");
   439	    assert!(err.contains("InvalidResolutionRef"));
   440	    let parent = hb.seq.q_snapshot().unwrap().state_root_t;
   441	    submit_and_apply(
   442	        &mut hb,
   443	        build_redeem(parent, "bob", "task-B", OutcomeSide::No, 500_000, 14),
   444	    )
   445	    .await
   446	    .expect("Bankrupt with outcome=No must succeed");
   447	}
   448	
   449	// ── SG-13.7 / SG-13.8 — fence delegation ────────────────────────────────────
   450	//
   451	// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
   452	// import/use of legacy CPMM in TB-13 modules) are enforced by Atom 0.5
   453	// forward-fence in `tests/tb_13_legacy_cpmm_forward_fence.rs`. This file
   454	// records the delegation contract.
   455	
   456	/// SG-13.7 (delegation marker) — `no_f64_in_new_complete_set_or_market_seed_path`
   457	/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed`.
   458	/// This test exists to make the architect SG-13.7 ship gate visible in
   459	/// the TB-13 integration test surface (per `feedback_workspace_test_canonical`
   460	/// + ship gate exact-name discipline).
   461	#[test]
   462	fn sg_13_7_no_f64_in_new_complete_set_or_market_seed_path() {
   463	    // Delegation: see tests/tb_13_legacy_cpmm_forward_fence.rs.
   464	    // This test passes by construction: any f64 leak would be caught by
   465	    // the forward-fence test at workspace time. We assert the delegation
   466	    // is in place by checking the fence file exists and contains the
   467	    // expected SG-13.0.2 test name.
   468	    let fence_src = std::fs::read_to_string(
   469	        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
   470	            .join("tests/tb_13_legacy_cpmm_forward_fence.rs"),
   471	    )
   472	    .expect("fence test file must exist");
   473	    assert!(
   474	        fence_src.contains("fn no_f64_in_complete_set_or_market_seed"),
   475	        "SG-13.0.2 fence delegation broken: missing target test name"
   476	    );
   477	}
   478	
   479	/// SG-13.8 (delegation marker) — `no_import_or_use_of_legacy_cpmm_in_tb13_modules`
   480	/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::legacy_cpm_api_not_imported_by_complete_set`.
   481	#[test]
   482	fn sg_13_8_no_import_or_use_of_legacy_cpmm_in_tb13_modules() {
   483	    let fence_src = std::fs::read_to_string(
   484	        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
   485	            .join("tests/tb_13_legacy_cpmm_forward_fence.rs"),
   486	    )
   487	    .expect("fence test file must exist");
   488	    assert!(
   489	        fence_src.contains("fn legacy_cpm_api_not_imported_by_complete_set"),
   490	        "SG-13.0.1 fence delegation broken: missing target test name"
   491	    );
   492	}
   493	
   494	// ── Halting-trigger guards (architect §4.8) ─────────────────────────────────
   495	
   496	/// Halt: total_supply_micro must be unchanged across mint+redeem.
   497	#[tokio::test]
   498	async fn halt_total_supply_micro_unchanged_across_mint_redeem() {
   499	    let mut q0 = genesis_with_balances(&[("alice", 100)]);
   500	    seed_task_market(&mut q0, "task-H1", TaskMarketState::Finalized);
   501	    let mut h = fresh_harness(q0.clone());
   502	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   503	
   504	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-H1", 7_000_000, 20))
   505	        .await
   506	        .expect("mint");
   507	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   508	    submit_and_apply(
   509	        &mut h,
   510	        build_redeem(parent, "alice", "task-H1", OutcomeSide::Yes, 7_000_000, 21),
   511	    )
   512	    .await
   513	    .expect("redeem");
   514	
   515	    let q = h.seq.q_snapshot().unwrap();
   516	    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
   517	        .expect("total_supply_micro bit-equal across mint+redeem");
   518	    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
   519	}
   520	
   521	/// Halt: shares are NOT counted as Coin (regression guard for SG-13.2).
   522	#[tokio::test]
   523	async fn halt_shares_not_counted_as_coin() {
   524	    let q0 = genesis_with_balances(&[("alice", 100)]);
   525	    let mut h = fresh_harness(q0.clone());
   526	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   527	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-H2", 9_876_543, 22))
   528	        .await
   529	        .expect("mint");
   530	    let q = h.seq.q_snapshot().unwrap();
   531	    assert_total_ctf_conserved(&q0.economic_state_t, &q.economic_state_t, &[])
   532	        .expect("shares not in total_supply");
   533	}
   534	
   535	/// Halt: MarketSeed with zero-balance provider rejected (regression
   536	/// guard for SG-13.3).
   537	#[tokio::test]
   538	async fn halt_market_seed_zero_balance_provider_rejected() {
   539	    let q0 = QState::genesis();
   540	    let mut h = fresh_harness(q0);
   541	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   542	    let err = submit_and_apply(&mut h, build_seed(parent, "ghost", "task-H3", 1_000, 23))
   543	        .await
   544	        .expect_err("seed must fail");
   545	    assert!(err.contains("InsufficientBalanceForMint"));
   546	}
   547	
   548	/// Architect-mandated invariant: redeeming more shares than owned is
   549	/// rejected with `RedeemMoreThanOwned`.
   550	#[tokio::test]
   551	async fn halt_redeem_more_than_owned_rejected() {
   552	    let mut q0 = genesis_with_balances(&[("alice", 100)]);
   553	    seed_task_market(&mut q0, "task-H4", TaskMarketState::Finalized);
   554	    let mut h = fresh_harness(q0);
   555	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   556	    submit_and_apply(&mut h, build_mint(parent, "alice", "task-H4", 1_000_000, 24))
   557	        .await
   558	        .expect("mint");
   559	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   560	    let err = submit_and_apply(
   561	        &mut h,
   562	        build_redeem(parent, "alice", "task-H4", OutcomeSide::Yes, 5_000_000, 25),
   563	    )
   564	    .await
   565	    .expect_err("over-redeem must fail");
   566	    assert!(
   567	        err.contains("RedeemMoreThanOwned"),
   568	        "expected RedeemMoreThanOwned, got: {err}"
   569	    );
   570	}
   571	
   572	/// Architect-mandated invariant: complete-set balanced post-seed.
   573	#[tokio::test]
   574	async fn halt_complete_set_balanced_post_seed() {
   575	    let q0 = genesis_with_balances(&[("provider", 50)]);
   576	    let mut h = fresh_harness(q0);
   577	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   578	    submit_and_apply(&mut h, build_seed(parent, "provider", "task-H5", 3_141_592, 26))
   579	        .await
   580	        .expect("seed");
   581	    let q = h.seq.q_snapshot().unwrap();
   582	    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");
   583	    let collateral = q
   584	        .economic_state_t
   585	        .conditional_collateral_t
   586	        .0
   587	        .get(&EventId(TaskId("task-H5".into())))
   588	        .copied()
   589	        .unwrap();
   590	    assert_eq!(collateral.micro_units(), 3_141_592);
   591	    let pair = q
   592	        .economic_state_t
   593	        .conditional_share_balances_t
   594	        .0
   595	        .get(&AgentId("provider".into()))
   596	        .and_then(|m| m.get(&EventId(TaskId("task-H5".into()))))
   597	        .copied()
   598	        .unwrap();
   599	    assert_eq!(pair.yes.units, 3_141_592_u128);
   600	    assert_eq!(pair.no.units, 3_141_592_u128);
   601	}
   602	
   603	// Suppress unused import warnings — the harness types are referenced via
   604	// trait constraints + the helper signatures.
   605	#[allow(dead_code)]
   606	fn _suppress_unused() {
   607	    let _ = ConditionalCollateralIndex::default();
   608	    let _ = ConditionalShareBalances::default();
   609	    let _: BTreeMap<EventId, MicroCoin> = BTreeMap::new();
   610	    let _ = ShareSidePair::default();
   611	}

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '150,225p' && nl -ba src/economy/monetary_invariant.rs | sed -n '310,410p' && nl -ba src/economy/monetary_invariant.rs | sed -n '428,500p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   150	/// `task_markets_t[t].bounty` is removed. Bounty money has migrated to
   151	/// `escrows_t.amount` via accepted `EscrowLockTx`. `task_markets_t` retains
   152	/// only the cached aggregate `total_escrow` (NOT in supply sum) + admission
   153	/// metadata.
   154	///
   155	/// **TB-8 5→4 holding migration** (2026-05-02): `claims_t.amount` is removed
   156	/// from the holding sum. Per TB-8 charter §3 Atom 3 + ratification §1 Q5:
   157	/// the FinalizeReward dispatch arm moves money DIRECTLY from `escrows_t` to
   158	/// `balances_t` (not via claims_t as an intermediate holding). `claims_t` is
   159	/// the *intent registry*: claim creation at OMEGA-Confirm records "this
   160	/// solver is owed this amount" without moving money; the money still lives
   161	/// in `escrows_t` until finalize debits it. The `claim.amount` field is the
   162	/// cached intent (= `task_market.total_escrow` at claim creation per single-
   163	/// solver MVP). Counting `claims_t` here while ALSO counting the backing
   164	/// `escrows_t` rows would double-mint every claim. The intent-vs-backing
   165	/// integrity is enforced separately by
   166	/// [`assert_claim_amount_backed_by_escrow`].
   167	///
   168	/// **Pre-TB-8 baseline**: `claims_t` was always empty (the dispatch arm was
   169	/// `NotYetImplemented`); removing it from the sum changes nothing for
   170	/// historical L4 replay (forward-only schema migration per
   171	/// `feedback_no_retroactive_evidence_rewrite`).
   172	fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
   173	    let mut total: i64 = 0;
   174	    for v in s.balances_t.0.values() {
   175	        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
   176	    }
   177	    for e in s.escrows_t.0.values() {
   178	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   179	    }
   180	    for e in s.stakes_t.0.values() {
   181	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   182	    }
   183	    // claims_t is INTENTIONALLY OMITTED — intent registry, not a holding
   184	    // (TB-8 charter §3 Atom 3 + ratification §1 Q5). The backing money lives
   185	    // in escrows_t; counting claims_t here would double-mint every claim.
   186	    // task_markets_t.total_escrow is INTENTIONALLY OMITTED — derived cache,
   187	    // not a holding (TB-3 charter § 3.2). Counting it would double-mint
   188	    // every bounty: the same micro-coins are already counted in escrows_t.
   189	    for c in s.challenge_cases_t.0.values() {
   190	        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
   191	    }
   192	    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
   193	    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
   194	    // held against outstanding YES_E + NO_E share inventory. Extends the
   195	    // 5-holding sum to 6. Without this, CompleteSetMintTx (which migrates
   196	    // Coin from balances_t to conditional_collateral_t) would falsely
   197	    // appear to burn money, failing assert_total_ctf_conserved with empty
   198	    // exempt list.
   199	    //
   200	    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
   201	    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
   202	    // a holding. Counting them would triple-count (shares are derived from
   203	    // collateral; including both creates a 2x parallel ledger).
   204	    for c in s.conditional_collateral_t.0.values() {
   205	        total = total.checked_add(c.micro_units()).ok_or(MonetaryError::Overflow)?;
   206	    }
   207	    Ok(total)
   208	}
   209	
   210	// ────────────────────────────────────────────────────────────────────────────
   211	// TB-8 Atom 1 — assert_claim_amount_backed_by_escrow (intent-vs-backing)
   212	// ────────────────────────────────────────────────────────────────────────────
   213	
   214	/// TRACE_MATRIX TB-8 charter §3 Atom 1 + Atom 3 — claim-intent-vs-escrow-
   215	/// backing invariant.
   216	///
   217	/// Asserts that for every Open `claims_t` entry, the claim's intended payout
   218	/// (`claim.amount`) is ≤ the backing escrow row (`escrows_t[claim.escrow_lock_tx_id].amount`).
   219	/// Replaces the old "claims_t is a holding" semantics with the explicit
   220	/// intent-vs-backing check: a claim cannot promise more than its escrow
   221	/// holds. Finalized claims are excluded — once finalized, the escrow has been
   222	/// debited and the balance credited, so the integrity check no longer applies
   223	/// (claim.amount is now historical).
   224	///
   225	/// **Caller convention**: invoked from any dispatch arm that mutates
   310	/// at the `TypedTx` layer.
   311	///
   312	/// **Today, K5 has no `Mint` variant** — none of the 7 `TypedTx` variants
   313	/// directly inject coins. Genesis allocation happens in `on_init` outside
   314	/// the K5 transition surface. Therefore, on a non-genesis `q`, this fn
   315	/// returns `Ok(())` for every well-formed `TypedTx`.
   316	///
   317	/// **Why the function exists anyway**: it is a forward-compat barrier.
   318	/// If a future RSP atom adds a `Mint` (or `SystemReward`-class) variant,
   319	/// it MUST be added to the match below AND rejected here when
   320	/// `q.state_root_t != Hash::ZERO`. Numeric conservation is enforced by
   321	/// [`assert_total_ctf_conserved`] separately.
   322	pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
   323	    let is_post_init = q.state_root_t != Hash::ZERO;
   324	    if !is_post_init {
   325	        return Ok(());
   326	    }
   327	    match tx {
   328	        TypedTx::Work(_)
   329	        | TypedTx::Verify(_)
   330	        | TypedTx::Challenge(_)
   331	        | TypedTx::Reuse(_)
   332	        | TypedTx::FinalizeReward(_)
   333	        | TypedTx::TaskExpire(_)
   334	        | TypedTx::TerminalSummary(_)
   335	        // TB-3 RSP-1: TaskOpen + EscrowLock are TRANSFERS (or metadata-only),
   336	        // never mints — their dispatch arms (Atoms 4-5) maintain CTF
   337	        // conservation via assert_total_ctf_conserved with empty exempt list.
   338	        | TypedTx::TaskOpen(_)
   339	        | TypedTx::EscrowLock(_)
   340	        // TB-5 RSP-3.0/3.1: ChallengeResolve is system-emitted resolution.
   341	        // Released path is a TRANSFER (challenger bond → balances; CTF
   342	        // round-trip closes; charter v2 § 4.6). UpheldDeferred is a marker
   343	        // only (no economic mutation; charter v2 § 4.7). Neither mints —
   344	        // CTF conservation enforced by assert_total_ctf_conserved with
   345	        // empty exempt list at the dispatch site.
   346	        | TypedTx::ChallengeResolve(_)
   347	        // TB-11 (architect §6.2 ruling 2026-05-02): TaskBankruptcy is a
   348	        // task-level state mutation only (task_markets_t[task_id].state →
   349	        // Bankrupt). No money movement, so trivially does not mint.
   350	        // CTF conservation enforced by assert_total_ctf_conserved with
   351	        // empty exempt list at the dispatch site.
   352	        | TypedTx::TaskBankruptcy(_)
   353	        // TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
   354	        // CR-13.1..6): CompleteSetMint / CompleteSetRedeem / MarketSeed
   355	        // are balance ↔ collateral migrations only. Mint debits balance
   356	        // and credits collateral 1:1; redeem debits collateral and shares
   357	        // and credits balance 1:1; seed debits provider balance and
   358	        // credits collateral 1:1. Conditional shares are claims, NOT Coin
   359	        // (CR-13.3 + SG-13.2). No mint, no burn — Atom 3 extends
   360	        // assert_total_ctf_conserved with conditional_collateral_t as the
   361	        // 6th Coin holding.
   362	        | TypedTx::CompleteSetMint(_)
   363	        | TypedTx::CompleteSetRedeem(_)
   364	        | TypedTx::MarketSeed(_) => Ok(()),
   365	    }
   366	}
   367	
   368	// ────────────────────────────────────────────────────────────────────────────
   369	// assert_total_ctf_conserved — numeric conservation across a transition
   370	// ────────────────────────────────────────────────────────────────────────────
   371	
   372	/// TRACE_MATRIX 基本法 1 + P3:1 — conservation of total CTF across a
   373	/// transition `before → after`.
   374	///
   375	/// Mints (`delta > 0`) and burns (`delta < 0`) are both rejected unless
   376	/// `exempt_tx_kinds` is non-empty. The exempt list is the explicit opt-out
   377	/// for legitimate supply-changing operations (e.g., genesis init,
   378	/// system-emitted rewards in a future RSP); RSP-0 never populates it
   379	/// at runtime.
   380	///
   381	/// Caller convention: pass `&[]` for normal agent-submitted transitions.
   382	/// Pass `&[TxKind::FinalizeReward]` (etc.) only when a system-emitted
   383	/// supply-changing tx is being processed AND the RSP semantics for that
   384	/// kind have been ratified. RSP-0 does not ratify any.
   385	pub fn assert_total_ctf_conserved(
   386	    before: &EconomicState,
   387	    after: &EconomicState,
   388	    exempt_tx_kinds: &[TxKind],
   389	) -> Result<(), MonetaryError> {
   390	    let total_before = total_supply_micro(before)?;
   391	    let total_after = total_supply_micro(after)?;
   392	    let delta = total_after
   393	        .checked_sub(total_before)
   394	        .ok_or(MonetaryError::Overflow)?;
   395	    if !exempt_tx_kinds.is_empty() {
   396	        return Ok(());
   397	    }
   398	    if delta > 0 {
   399	        return Err(MonetaryError::PostInitMint { delta_micro: delta });
   400	    }
   401	    if delta < 0 {
   402	        return Err(MonetaryError::TotalCtfBurn { delta_micro: delta });
   403	    }
   404	    Ok(())
   405	}
   406	
   407	// ────────────────────────────────────────────────────────────────────────────
   408	// assert_read_is_free — tx-level no-fee guard
   409	// ────────────────────────────────────────────────────────────────────────────
   410	
   428	// TB-13 Atom 3 — assert_complete_set_balanced (architect 2026-05-03 post-
   429	// TB-12 ruling Part A §4.4 SG-13.1 + §4.5 CR-13.3..4)
   430	// ────────────────────────────────────────────────────────────────────────────
   431	
   432	/// TRACE_MATRIX TB-13 Atom 3 (architect §4.3 + SG-13.1): the
   433	/// **complete-set balanced** invariant.
   434	///
   435	/// For every event in `conditional_collateral_t`:
   436	///
   437	/// ```text
   438	/// min(Σ_{owner} share[(owner, event, Yes)], Σ_{owner} share[(owner, event, No)])
   439	///   == collateral[event].micro_units()
   440	/// ```
   441	///
   442	/// Why MIN, not equality on both sides:
   443	/// - Pre-resolution (mint + seed only): both sides equal collateral, so
   444	///   `min == collateral` is trivially equivalent to `Yes == No == collateral`.
   445	/// - Post-resolution + partial redeem: the winning side decreases by the
   446	///   redeemed amount AND collateral decreases by the same amount; the
   447	///   losing side stays the same (its shares are stranded zero-value
   448	///   claims). So `winning_side == collateral` still holds, while
   449	///   `losing_side > collateral` (losing side has surplus). MIN picks
   450	///   the winning side and matches collateral.
   451	/// - Post-resolution + full redeem: winning side is 0, collateral is 0,
   452	///   losing side is the original mint amount. MIN(0, original) = 0 = collateral.
   453	///
   454	/// This is the mathematical core of "1 Coin = 1 YES_E + 1 NO_E" enforced
   455	/// at the QState level: every Coin in collateral can be redeemed by the
   456	/// winning side, and no winning-side share is unbacked.
   457	pub fn assert_complete_set_balanced(
   458	    s: &EconomicState,
   459	) -> Result<(), MonetaryError> {
   460	    use crate::state::typed_tx::OutcomeSide;
   461	    for (event_id, collateral) in s.conditional_collateral_t.0.iter() {
   462	        let collateral_units: u128 = collateral.micro_units() as u128;
   463	        let mut sum_yes: u128 = 0;
   464	        let mut sum_no: u128 = 0;
   465	        for owner_map in s.conditional_share_balances_t.0.values() {
   466	            if let Some(pair) = owner_map.get(event_id) {
   467	                sum_yes = sum_yes
   468	                    .checked_add(pair.yes.units)
   469	                    .ok_or(MonetaryError::Overflow)?;
   470	                sum_no = sum_no
   471	                    .checked_add(pair.no.units)
   472	                    .ok_or(MonetaryError::Overflow)?;
   473	            }
   474	        }
   475	        let min_side = sum_yes.min(sum_no);
   476	        if min_side != collateral_units {
   477	            // Report the failing side (the smaller one) for diagnostic
   478	            // clarity — that's where the equality-with-collateral broke.
   479	            let (side, share_sum_units) = if sum_yes <= sum_no {
   480	                (OutcomeSide::Yes, sum_yes)
   481	            } else {
   482	                (OutcomeSide::No, sum_no)
   483	            };
   484	            return Err(MonetaryError::CompleteSetUnbalanced {
   485	                event_id_hex: hex_event_id(event_id),
   486	                side,
   487	                share_sum_units,
   488	                collateral_units,
   489	            });
   490	        }
   491	    }
   492	    Ok(())
   493	}
   494	
   495	fn hex_event_id(event_id: &crate::state::typed_tx::EventId) -> String {
   496	    event_id.0 .0.clone()
   497	}
   498	
   499	// ────────────────────────────────────────────────────────────────────────────
   500	// Tests

exec
/bin/bash -lc "sed -n '232,244p' tests/tb_13_complete_set.rs | nl -ba" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	    q_pre_balanced.economic_state_t.balances_t.0.insert(
     2	        AgentId("alice".into()),
     3	        MicroCoin::from_coin(100).unwrap(),
     4	    );
     5	    assert_total_ctf_conserved(
     6	        &q_pre_balanced.economic_state_t,
     7	        &q.economic_state_t,
     8	        &[],
     9	    )
    10	    .expect("CTF preserved across mint");
    11	    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
    12	}
    13	

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '640,790p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   640	    }
   641	
   642	    #[test]
   643	    fn ctf_conserved_across_n10_random_sequence() {
   644	        // Charter Day-2 unit: "total CTF conserved across N=10 random tx sequences".
   645	        // We model 10 deterministic-but-varied conservative redistributions
   646	        // (Alice/Bob/Carol; balances ↔ escrow ↔ stake ↔ claim ↔ market ↔ challenge).
   647	        // Each step is a closed transfer; total supply is invariant.
   648	        let mut s = EconomicState::default();
   649	        s.balances_t.0.insert(agent("alice"), coin(100));
   650	        s.balances_t.0.insert(agent("bob"), coin(50));
   651	        s.balances_t.0.insert(agent("carol"), coin(25));
   652	        let total0 = total_supply_micro(&s).unwrap();
   653	
   654	        let steps: [(&str, i64); 10] = [
   655	            ("alice->bob", 5),
   656	            ("bob->escrow:t1", 10),
   657	            ("alice->stake:tx1", 7),
   658	            ("escrow:t1->claim:tx1", 3),
   659	            ("alice->market:t2", 20),
   660	            ("market:t2->balance:carol", 15),
   661	            ("stake:tx1->challenge:case1", 4),
   662	            ("challenge:case1->balance:bob", 2),
   663	            ("claim:tx1->balance:alice", 3),
   664	            ("balance:carol->escrow:t3", 6),
   665	        ];
   666	
   667	        let total_each = vec![total0; 10];
   668	        for (i, (label, _amt)) in steps.iter().enumerate() {
   669	            // Apply a small redistribution: move `_amt` micro_per_coin
   670	            // between two slots. We just re-shuffle existing supply.
   671	            // (Concrete redistribution mechanics live in SettlementEngine;
   672	            // the invariant under test is: any closed redistribution leaves
   673	            // total_supply_micro unchanged.)
   674	            let amt_micro = (i as i64 + 1) * 1_000; // small, deterministic
   675	            // Move `amt_micro` from alice's balance into a synthetic stake.
   676	            let alice_bal = s.balances_t.0.get(&agent("alice"))
   677	                .copied().unwrap_or(MicroCoin::zero());
   678	            if alice_bal.micro_units() >= amt_micro {
   679	                s.balances_t.0.insert(
   680	                    agent("alice"),
   681	                    MicroCoin::from_micro_units(alice_bal.micro_units() - amt_micro),
   682	                );
   683	                let key = tx(&format!("stake-step-{}", i));
   684	                s.stakes_t.0.insert(
   685	                    key,
   686	                    StakeEntry { amount: MicroCoin::from_micro_units(amt_micro), staker: agent("alice"), task_id: TaskId::default() },
   687	                );
   688	            }
   689	            let total_now = total_supply_micro(&s).unwrap();
   690	            assert_eq!(
   691	                total_now, total_each[i],
   692	                "step {} ({}): conservation broke",
   693	                i, label
   694	            );
   695	        }
   696	        // Final cross-check.
   697	        assert_eq!(total_supply_micro(&s).unwrap(), total0);
   698	    }
   699	
   700	    #[test]
   701	    fn ctf_counts_all_four_holding_subindexes() {
   702	        // **TB-3 6→5 holding migration**: previously summed
   703	        // balances + escrows + stakes + claims + bounty + bond (6).
   704	        // After TB-3: balances + escrows + stakes + claims + bond (5).
   705	        // **TB-8 5→4 holding migration** (2026-05-02): claims_t is now an
   706	        // intent registry, NOT a holding. Per TB-8 charter §3 Atom 3 + Atom 1
   707	        // ratification §1 Q5: FinalizeReward dispatches escrows → balances
   708	        // directly; claim.amount is cached intent metadata. Counting
   709	        // claims_t while ALSO counting backing escrows_t would double-mint
   710	        // every claim. Sums balances + escrows + stakes + bond (4).
   711	        let mut s = EconomicState::default();
   712	        s.balances_t.0.insert(agent("a"), coin(1));
   713	        s.escrows_t.0.insert(
   714	            tx("e"),
   715	            EscrowEntry { amount: coin(2), depositor: agent("a"), task_id: task("task-e") },
   716	        );
   717	        s.stakes_t.0.insert(
   718	            tx("s"),
   719	            StakeEntry { amount: coin(4), staker: agent("a"), task_id: task("task-s") },
   720	        );
   721	        // **TB-8**: a claim_t entry is INTENT metadata. The coin(8) intent
   722	        // here references no escrow row (test fixture in isolation), so it
   723	        // is excluded from the supply sum below. The intent-vs-backing
   724	        // invariant `assert_claim_amount_backed_by_escrow` would catch any
   725	        // unbacked claim attached to a non-existent escrow row when fired
   726	        // from a real dispatch arm; here the seeded fixture is read-only.
   727	        s.claims_t.0.insert(
   728	            tx("c"),
   729	            ClaimEntry {
   730	                amount: coin(8),
   731	                claimant: agent("a"),
   732	                ..Default::default()
   733	            },
   734	        );
   735	        // The 16 that used to live in task_markets_t.bounty now lives as a
   736	        // second escrows_t entry — same money, canonical home.
   737	        s.escrows_t.0.insert(
   738	            tx("e2"),
   739	            EscrowEntry { amount: coin(16), depositor: agent("a"), task_id: task("task-e2") },
   740	        );
   741	        let mut cc = crate::state::q_state::ChallengeCase::default();
   742	        cc.bond = coin(32);
   743	        cc.challenger = agent("a");
   744	        s.challenge_cases_t.0.insert(tx("ch"), cc);
   745	
   746	        // 4 holdings post-TB-8: 1 + (2+16) + 4 + 32 = 55 (claims coin(8)
   747	        // is intent, NOT counted).
   748	        assert_eq!(total_supply_micro(&s).unwrap(), 55 * MICRO_PER_COIN);
   749	    }
   750	
   751	    #[test]
   752	    fn total_supply_does_not_double_count_total_escrow() {
   753	        // **TB-3 charter § 3.2 regression test**: setting BOTH
   754	        // escrows_t[e].amount = K and task_markets_t[t].total_escrow = K
   755	        // (which is the steady-state shape after an accepted EscrowLockTx)
   756	        // must yield total_supply_micro = K, NOT 2K. If a regression adds
   757	        // task_markets.total_escrow back into the holding sum, this test
   758	        // catches it immediately.
   759	        let mut s = EconomicState::default();
   760	        let task_id = task("task-double-count-regression");
   761	        s.escrows_t.0.insert(
   762	            tx("escrow-lock-1"),
   763	            EscrowEntry {
   764	                amount: coin(50),
   765	                depositor: agent("sponsor"),
   766	                task_id: task_id.clone(),
   767	            },
   768	        );
   769	        let mut entry = crate::state::q_state::TaskMarketEntry::default();
   770	        entry.total_escrow = coin(50);
   771	        entry.escrow_lock_tx_ids.insert(tx("escrow-lock-1"));
   772	        s.task_markets_t.0.insert(task_id, entry);
   773	
   774	        assert_eq!(
   775	            total_supply_micro(&s).unwrap(),
   776	            50 * MICRO_PER_COIN,
   777	            "total_supply must equal the escrows_t holding (50), not 2× (100). \
   778	             task_markets_t.total_escrow is a derived cache, NOT a holding."
   779	        );
   780	    }
   781	
   782	    #[test]
   783	    fn task_market_total_escrow_matches_sum_of_escrow_locks() {
   784	        // **TB-3 charter § 3.2 cache=truth invariant test**: after multiple
   785	        // EscrowLock-equivalent inserts to escrows_t for the same task_id,
   786	        // task_markets_t[task_id].total_escrow must equal the sum.
   787	        let mut s = EconomicState::default();
   788	        let t = task("task-cache-truth");
   789	
   790	        // Two escrow locks for the same task (multi-sponsor or top-up case).

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '750,830p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   750	
   751	    #[test]
   752	    fn total_supply_does_not_double_count_total_escrow() {
   753	        // **TB-3 charter § 3.2 regression test**: setting BOTH
   754	        // escrows_t[e].amount = K and task_markets_t[t].total_escrow = K
   755	        // (which is the steady-state shape after an accepted EscrowLockTx)
   756	        // must yield total_supply_micro = K, NOT 2K. If a regression adds
   757	        // task_markets.total_escrow back into the holding sum, this test
   758	        // catches it immediately.
   759	        let mut s = EconomicState::default();
   760	        let task_id = task("task-double-count-regression");
   761	        s.escrows_t.0.insert(
   762	            tx("escrow-lock-1"),
   763	            EscrowEntry {
   764	                amount: coin(50),
   765	                depositor: agent("sponsor"),
   766	                task_id: task_id.clone(),
   767	            },
   768	        );
   769	        let mut entry = crate::state::q_state::TaskMarketEntry::default();
   770	        entry.total_escrow = coin(50);
   771	        entry.escrow_lock_tx_ids.insert(tx("escrow-lock-1"));
   772	        s.task_markets_t.0.insert(task_id, entry);
   773	
   774	        assert_eq!(
   775	            total_supply_micro(&s).unwrap(),
   776	            50 * MICRO_PER_COIN,
   777	            "total_supply must equal the escrows_t holding (50), not 2× (100). \
   778	             task_markets_t.total_escrow is a derived cache, NOT a holding."
   779	        );
   780	    }
   781	
   782	    #[test]
   783	    fn task_market_total_escrow_matches_sum_of_escrow_locks() {
   784	        // **TB-3 charter § 3.2 cache=truth invariant test**: after multiple
   785	        // EscrowLock-equivalent inserts to escrows_t for the same task_id,
   786	        // task_markets_t[task_id].total_escrow must equal the sum.
   787	        let mut s = EconomicState::default();
   788	        let t = task("task-cache-truth");
   789	
   790	        // Two escrow locks for the same task (multi-sponsor or top-up case).
   791	        s.escrows_t.0.insert(
   792	            tx("lock-A"),
   793	            EscrowEntry { amount: coin(30), depositor: agent("alice"), task_id: t.clone() },
   794	        );
   795	        s.escrows_t.0.insert(
   796	            tx("lock-B"),
   797	            EscrowEntry { amount: coin(20), depositor: agent("bob"), task_id: t.clone() },
   798	        );
   799	        // One escrow for a DIFFERENT task — must not contaminate the sum.
   800	        s.escrows_t.0.insert(
   801	            tx("lock-other"),
   802	            EscrowEntry {
   803	                amount: coin(7),
   804	                depositor: agent("carol"),
   805	                task_id: task("task-other"),
   806	            },
   807	        );
   808	
   809	        // Cache reflects the truth.
   810	        let mut entry = crate::state::q_state::TaskMarketEntry::default();
   811	        entry.total_escrow = coin(50);
   812	        entry.escrow_lock_tx_ids.insert(tx("lock-A"));
   813	        entry.escrow_lock_tx_ids.insert(tx("lock-B"));
   814	        s.task_markets_t.0.insert(t.clone(), entry);
   815	
   816	        assert_eq!(assert_task_market_total_escrow_matches_locks(&s, &t), Ok(()));
   817	
   818	        // Drift the cache (simulate a missed update or an attacker mutating
   819	        // EconomicState directly): cache=truth predicate must reject.
   820	        s.task_markets_t.0.get_mut(&t).unwrap().total_escrow = coin(60);
   821	        let r = assert_task_market_total_escrow_matches_locks(&s, &t);
   822	        assert!(matches!(r, Err(MonetaryError::DerivedCacheMismatch { .. })),
   823	            "drifted cache must surface as DerivedCacheMismatch; got {:?}", r);
   824	    }
   825	
   826	    // ── assert_read_is_free ─────────────────────────────────────────────────
   827	
   828	    #[test]
   829	    fn read_is_free_zero_fee_passes_for_all_kinds() {
   830	        for k in [

exec
/bin/bash -lc "rg --line-number \"complete_set|shares|conditional_collateral|conditional_share|MIN|min\\(\" src/economy/monetary_invariant.rs | sed -n '1,200p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
71:    /// `event_id`, the sum of YES (or NO) shares across all owners does
116:                    "complete-set unbalanced: event_id={} side={:?} Σ shares={} != collateral_units={}",
193:    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
196:    // Coin from balances_t to conditional_collateral_t) would falsely
200:    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
201:    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
202:    // a holding. Counting them would triple-count (shares are derived from
204:    for c in s.conditional_collateral_t.0.values() {
356:        // and credits collateral 1:1; redeem debits collateral and shares
358:        // credits collateral 1:1. Conditional shares are claims, NOT Coin
360:        // assert_total_ctf_conserved with conditional_collateral_t as the
428:// TB-13 Atom 3 — assert_complete_set_balanced (architect 2026-05-03 post-
435:/// For every event in `conditional_collateral_t`:
438:/// min(Σ_{owner} share[(owner, event, Yes)], Σ_{owner} share[(owner, event, No)])
442:/// Why MIN, not equality on both sides:
447:///   losing side stays the same (its shares are stranded zero-value
449:///   `losing_side > collateral` (losing side has surplus). MIN picks
452:///   losing side is the original mint amount. MIN(0, original) = 0 = collateral.
457:pub fn assert_complete_set_balanced(
461:    for (event_id, collateral) in s.conditional_collateral_t.0.iter() {
465:        for owner_map in s.conditional_share_balances_t.0.values() {
475:        let min_side = sum_yes.min(sum_no);

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '28,44p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    28	};
    29	use crate::bottom_white::ledger::rejection_evidence::{
    30	    RejectionClass as L4ERejectionClass, RejectionEvidenceWriter,
    31	};
    32	use crate::bottom_white::ledger::transition_ledger::{
    33	    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
    34	    LedgerWriterError,
    35	};
    36	use crate::bottom_white::tools::registry::ToolRegistry;
    37	use crate::economy::monetary_invariant::{
    38	    assert_claim_amount_backed_by_escrow, assert_no_post_init_mint, assert_read_is_free,
    39	    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
    40	};
    41	use crate::state::q_state::{AgentId, EscrowEntry, Hash, QState, TaskMarketEntry, TxId};
    42	use crate::state::typed_tx::{HasSubmitter, SignalBundle, TransitionError, TypedTx};
    43	use std::collections::BTreeSet;
    44	use crate::top_white::predicates::registry::PredicateRegistry;

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '1,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-13 Atom 0.5 — Legacy CPMM forward-fence + label ship-gate.
     2	//!
     3	//! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2;
     4	//! SG-13.0.1 / SG-13.0.2 / SG-13.0.3).
     5	//!
     6	//! These three tests enforce the forward-binding fence that NEW TB-13
     7	//! modules cannot import or reuse the legacy `src/prediction_market.rs`
     8	//! f64 CPMM scaffolding. Architect §4.2 halting triggers:
     9	//!
    10	//!   HALT if new TB-13 code imports legacy prediction_market.rs.
    11	//!   HALT if f64 appears in new CompleteSet / MarketSeed code.
    12	//!   HALT if any AMM / CPMM router function is introduced in TB-13.
    13	//!
    14	//! ## What is "TB-13 code"?
    15	//!
    16	//! A span of Rust source belongs to TB-13 iff it is a contiguous block
    17	//! of non-blank lines whose first non-blank line contains an authoring
    18	//! marker that identifies TB-13 as the contributing tracer-bullet (NOT
    19	//! a forward-reference from an earlier-TB doc-comment to TB-13's future
    20	//! work). Authoring markers:
    21	//!
    22	//!   - `TRACE_MATRIX TB-13 ` (TB-12 convention used by every shipped TB).
    23	//!   - A line that begins with `// TB-13 ` after stripping leading
    24	//!     whitespace + comment markers.
    25	//!   - A line that begins with `//! TB-13 ` (module-level doc).
    26	//!   - A line that begins with `/// TB-13 ` (item-level doc).
    27	//!
    28	//! A span ends at the next blank line OR end-of-file. Cross-references
    29	//! to TB-13 from inside a TB-12 (or earlier) span do NOT pull that span
    30	//! into TB-13 scope — only the *first non-blank line* of a span is
    31	//! checked for the authoring marker.
    32	//!
    33	//! ## File set in scope
    34	//!
    35	//! - `src/state/typed_tx.rs` — TB-13 typed-tx variant additions (Atom 1).
    36	//! - `src/state/q_state.rs` — TB-13 EconomicState extensions (Atom 2).
    37	//! - `src/state/sequencer.rs` — TB-13 dispatch-arm additions (Atom 2).
    38	//! - `src/economy/monetary_invariant.rs` — TB-13 conservation extensions (Atom 3).
    39	//! - `src/bin/audit_dashboard.rs` — TB-13 §14 dashboard rendering (Atom 4).
    40	//!
    41	//! At Atom 0.5 ship time, none of these files contain `TB-13` markers
    42	//! (TB-12 is the latest contributor). The fence passes trivially. As
    43	//! Atom 1..4 land, markers appear and the fence enforces the rule.
    44	
    45	use std::fs;
    46	use std::path::PathBuf;
    47	
    48	/// In-scope source files for the TB-13 forward-fence. NEW TB-13 markers
    49	/// appearing in any of these files are subject to the forbidden-token
    50	/// rules below.
    51	const FENCE_SCOPE: &[&str] = &[
    52	    "src/state/typed_tx.rs",
    53	    "src/state/q_state.rs",
    54	    "src/state/sequencer.rs",
    55	    "src/economy/monetary_invariant.rs",
    56	    "src/bin/audit_dashboard.rs",
    57	];
    58	
    59	/// Tokens forbidden inside any TB-13-marker span (architect §4.2 halting
    60	/// triggers + §4.7 forbidden list).
    61	///
    62	/// Each entry is a literal substring that must NOT appear in TB-13 code.
    63	const FORBIDDEN_LEGACY_TOKENS: &[&str] = &[
    64	    // Direct legacy CPMM imports / type names.
    65	    "prediction_market::",
    66	    "BinaryMarket",
    67	    // Legacy CPMM API method names.
    68	    ".buy_yes(",
    69	    ".buy_no(",
    70	    "open_bounty_market",
    71	    "bounty_market",
    72	    "bounty_lp_seed",
    73	    "bounty_yes_price",
    74	    "resolve_bounty",
    75	    "market_ticker(",
    76	    "market_ticker_full(",
    77	    // f64 in money-path context (see SG-13.0.2 dedicated test for the
    78	    // primary check; this entry catches `f64` in any TB-13-marked span).
    79	    " f64",
    80	    "f64,",
    81	    "f64;",
    82	    "f64)",
    83	    // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
    84	    "MarketOrderTx",
    85	    "MarketTradeTx",
    86	    "MarketBuyTx",
    87	    "MarketSellTx",
    88	    "AMM",
    89	    "CPMM",
    90	    "DPMM",
    91	    "orderbook",
    92	    // Price-as-truth concepts (deferred to TB-14 per §5).
    93	    "price_yes",
    94	    "price_no",
    95	    "PriceIndex",
    96	    "yes_price",
    97	    "no_price",
    98	    "RationalPrice",
    99	];
   100	
   101	fn workspace_root() -> PathBuf {
   102	    let manifest = env!("CARGO_MANIFEST_DIR");
   103	    PathBuf::from(manifest)
   104	}
   105	
   106	/// Returns true if `line` is an authoring marker for TB-13 (i.e., the
   107	/// line declares that the following block is TB-13 code, NOT a forward-
   108	/// reference from an earlier-TB doc-comment to TB-13's future work).
   109	fn is_tb_13_authoring_marker(line: &str) -> bool {
   110	    if line.contains("TRACE_MATRIX TB-13 ") {
   111	        return true;
   112	    }
   113	    let trimmed = line.trim_start();
   114	    let body = trimmed
   115	        .strip_prefix("//! ")
   116	        .or_else(|| trimmed.strip_prefix("/// "))
   117	        .or_else(|| trimmed.strip_prefix("// "))
   118	        .unwrap_or("");
   119	    body.starts_with("TB-13 ")
   120	}
   121	
   122	/// Extract line ranges that belong to TB-13 additions. A span is a
   123	/// contiguous block of non-blank lines; it is in-scope iff the first
   124	/// non-blank line is an authoring marker per `is_tb_13_authoring_marker`.
   125	fn tb_13_spans(source: &str) -> Vec<(usize, String)> {
   126	    let mut out: Vec<(usize, String)> = Vec::new();
   127	    let lines: Vec<&str> = source.lines().collect();
   128	    let n = lines.len();
   129	    let mut i = 0;
   130	    while i < n {
   131	        if lines[i].trim().is_empty() {
   132	            i += 1;
   133	            continue;
   134	        }
   135	        let span_start = i;
   136	        let mut span_end = i;
   137	        while span_end < n && !lines[span_end].trim().is_empty() {
   138	            span_end += 1;
   139	        }
   140	        let span = &lines[span_start..span_end];
   141	        if is_tb_13_authoring_marker(span[0]) {
   142	            for (offset, line) in span.iter().enumerate() {
   143	                out.push((span_start + offset + 1, (*line).to_string()));
   144	            }
   145	        }
   146	        i = span_end;
   147	    }
   148	    out
   149	}
   150	
   151	/// Read a source file relative to the workspace root, returning its
   152	/// content as a String. Panics with a clear message if missing — Atom 0.5
   153	/// ship requires every file in `FENCE_SCOPE` to exist.
   154	fn read_scope_file(rel_path: &str) -> String {
   155	    let full = workspace_root().join(rel_path);
   156	    fs::read_to_string(&full)
   157	        .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
   158	}
   159	
   160	/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
   161	///
   162	/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
   163	/// `prediction_market.rs`. We extend the check to all legacy CPMM API
   164	/// names and all forbidden trading/AMM concepts.
   165	#[test]
   166	fn legacy_cpm_api_not_imported_by_complete_set() {
   167	    let mut violations: Vec<String> = Vec::new();
   168	    for rel in FENCE_SCOPE {
   169	        let source = read_scope_file(rel);
   170	        for (line_no, line) in tb_13_spans(&source) {
   171	            for token in FORBIDDEN_LEGACY_TOKENS {
   172	                // The `f64` family entries are checked in SG-13.0.2 — skip
   173	                // them here so the failure message is unambiguous.
   174	                if token.starts_with(" f64")
   175	                    || token.starts_with("f64,")
   176	                    || token.starts_with("f64;")
   177	                    || token.starts_with("f64)")
   178	                {
   179	                    continue;
   180	                }
   181	                if line.contains(token) {
   182	                    violations.push(format!(
   183	                        "{rel}:{line_no}: TB-13-marked span contains forbidden token `{token}` — {line}"
   184	                    ));
   185	                }
   186	            }
   187	        }
   188	    }
   189	    assert!(
   190	        violations.is_empty(),
   191	        "TB-13 SG-13.0.1 forward-fence violated:\n{}",
   192	        violations.join("\n")
   193	    );
   194	}
   195	
   196	/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`.
   197	///
   198	/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
   199	/// CompleteSet / MarketSeed code. Money-path types must use integer
   200	/// `MicroCoin` / `ShareAmount`.
   201	#[test]
   202	fn no_f64_in_complete_set_or_market_seed() {
   203	    let mut violations: Vec<String> = Vec::new();
   204	    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
   205	    for rel in FENCE_SCOPE {
   206	        let source = read_scope_file(rel);
   207	        for (line_no, line) in tb_13_spans(&source) {
   208	            for token in &f64_tokens {
   209	                if line.contains(token) {
   210	                    violations.push(format!(
   211	                        "{rel}:{line_no}: TB-13-marked span contains f64 (`{token}`) — {line}"
   212	                    ));
   213	                }
   214	            }
   215	        }
   216	    }
   217	    assert!(
   218	        violations.is_empty(),
   219	        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
   220	        violations.join("\n")
   221	    );
   222	}
   223	
   224	/// SG-13.0.3 — `prediction_market_legacy_quarantined`.
   225	///
   226	/// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
   227	/// enforce that `src/prediction_market.rs` carries the LEGACY module-
   228	/// header doc-comment with the four required tokens (`legacy`,
   229	/// `not constitutional`, `not RSP-M`, `not production market path`)
   230	/// AND that `src/kernel.rs` market-bearing fields carry the `LEGACY`
   231	/// label tying them to the migration path.
   232	#[test]
   233	fn prediction_market_legacy_quarantined() {
   234	    let pm = read_scope_file("src/prediction_market.rs");
   235	    let header = pm
   236	        .lines()
   237	        .take(60)
   238	        .collect::<Vec<_>>()
   239	        .join("\n");
   240	
   241	    let required_label_tokens = [
   242	        "LEGACY",
   243	        "not constitutional",
   244	        "not RSP-M",
   245	        "not production market path",
   246	    ];
   247	    for token in &required_label_tokens {
   248	        assert!(
   249	            header.contains(token),
   250	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
   251	             label token `{token}`. Header:\n{header}"
   252	        );
   253	    }
   254	
   255	    // Architect §4.2 also requires the doc to name the migration path so
   256	    // future maintainers don't reintroduce the legacy API.
   257	    let migration_tokens = [
   258	        "TB-13",
   259	        "TB-14",
   260	        "CompleteSetMintTx",
   261	        "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
   262	    ];
   263	    for token in &migration_tokens {
   264	        assert!(
   265	            header.contains(token),
   266	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
   267	             token `{token}`. Header:\n{header}"
   268	        );
   269	    }
   270	
   271	    // Defense-in-depth: kernel.rs market-bearing fields carry LEGACY.
   272	    let kernel = read_scope_file("src/kernel.rs");
   273	    let kernel_struct_idx = kernel
   274	        .find("pub struct Kernel {")
   275	        .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
   276	    let kernel_struct_end = kernel[kernel_struct_idx..]
   277	        .find("\n}\n")
   278	        .map(|off| kernel_struct_idx + off + 2)
   279	        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
   280	    let kernel_struct_block = &kernel[kernel_struct_idx..kernel_struct_end];
   281	
   282	    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
   283	        let field_marker = format!("pub {field}");
   284	        let field_idx = kernel_struct_block
   285	            .find(&field_marker)
   286	            .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
   287	        // Look for `LEGACY` in the 600 chars preceding the field
   288	        // declaration (covers a multi-line doc-comment block).
   289	        let doc_window_start = field_idx.saturating_sub(600);
   290	        let doc_window = &kernel_struct_block[doc_window_start..field_idx];
   291	        assert!(
   292	            doc_window.contains("LEGACY"),
   293	            "TB-13 SG-13.0.3: Kernel.{field} missing LEGACY doc-comment label. \
   294	             Doc window:\n{doc_window}"
   295	        );
   296	    }
   297	}

exec
/bin/bash -lc "rg --line-number \"prediction_market::|BinaryMarket|buy_yes|buy_no|open_bounty_market|bounty_market|bounty_lp_seed|f64|MarketOrderTx|MarketTradeTx|PriceIndex\" src/state/typed_tx.rs src/state/q_state.rs src/state/sequencer.rs src/economy/monetary_invariant.rs src/bin/audit_dashboard.rs tests/tb_13_complete_set.rs tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "git diff --unified=0 fa36eca..HEAD -- src/state/typed_tx.rs src/state/q_state.rs src/state/sequencer.rs src/economy/monetary_invariant.rs src/bin/audit_dashboard.rs | rg --line-number '"'^'"\\+.*(prediction_market::|BinaryMarket|buy_yes|buy_no|open_bounty_market|bounty_market|bounty_lp_seed|f64|MarketOrderTx|MarketTradeTx|PriceIndex|AMM|CPMM|DPMM|orderbook|price_yes|price_no)' || true" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "git diff --unified=0 fa36eca..HEAD -- src/state/typed_tx.rs src/state/q_state.rs src/state/sequencer.rs src/economy/monetary_invariant.rs src/bin/audit_dashboard.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 145ms:
tests/tb_13_legacy_cpmm_forward_fence.rs:8://! f64 CPMM scaffolding. Architect §4.2 halting triggers:
tests/tb_13_legacy_cpmm_forward_fence.rs:11://!   HALT if f64 appears in new CompleteSet / MarketSeed code.
tests/tb_13_legacy_cpmm_forward_fence.rs:65:    "prediction_market::",
tests/tb_13_legacy_cpmm_forward_fence.rs:66:    "BinaryMarket",
tests/tb_13_legacy_cpmm_forward_fence.rs:68:    ".buy_yes(",
tests/tb_13_legacy_cpmm_forward_fence.rs:69:    ".buy_no(",
tests/tb_13_legacy_cpmm_forward_fence.rs:70:    "open_bounty_market",
tests/tb_13_legacy_cpmm_forward_fence.rs:71:    "bounty_market",
tests/tb_13_legacy_cpmm_forward_fence.rs:72:    "bounty_lp_seed",
tests/tb_13_legacy_cpmm_forward_fence.rs:77:    // f64 in money-path context (see SG-13.0.2 dedicated test for the
tests/tb_13_legacy_cpmm_forward_fence.rs:78:    // primary check; this entry catches `f64` in any TB-13-marked span).
tests/tb_13_legacy_cpmm_forward_fence.rs:79:    " f64",
tests/tb_13_legacy_cpmm_forward_fence.rs:80:    "f64,",
tests/tb_13_legacy_cpmm_forward_fence.rs:81:    "f64;",
tests/tb_13_legacy_cpmm_forward_fence.rs:82:    "f64)",
tests/tb_13_legacy_cpmm_forward_fence.rs:84:    "MarketOrderTx",
tests/tb_13_legacy_cpmm_forward_fence.rs:85:    "MarketTradeTx",
tests/tb_13_legacy_cpmm_forward_fence.rs:95:    "PriceIndex",
tests/tb_13_legacy_cpmm_forward_fence.rs:172:                // The `f64` family entries are checked in SG-13.0.2 — skip
tests/tb_13_legacy_cpmm_forward_fence.rs:174:                if token.starts_with(" f64")
tests/tb_13_legacy_cpmm_forward_fence.rs:175:                    || token.starts_with("f64,")
tests/tb_13_legacy_cpmm_forward_fence.rs:176:                    || token.starts_with("f64;")
tests/tb_13_legacy_cpmm_forward_fence.rs:177:                    || token.starts_with("f64)")
tests/tb_13_legacy_cpmm_forward_fence.rs:196:/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`.
tests/tb_13_legacy_cpmm_forward_fence.rs:198:/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
tests/tb_13_legacy_cpmm_forward_fence.rs:202:fn no_f64_in_complete_set_or_market_seed() {
tests/tb_13_legacy_cpmm_forward_fence.rs:204:    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
tests/tb_13_legacy_cpmm_forward_fence.rs:208:            for token in &f64_tokens {
tests/tb_13_legacy_cpmm_forward_fence.rs:211:                        "{rel}:{line_no}: TB-13-marked span contains f64 (`{token}`) — {line}"
tests/tb_13_legacy_cpmm_forward_fence.rs:219:        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
tests/tb_13_legacy_cpmm_forward_fence.rs:282:    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
tests/tb_13_complete_set.rs:14://! legacy CPMM / no f64 / no AMM/CPMM router).
tests/tb_13_complete_set.rs:17://! - SG-13.0.2 no_f64_in_complete_set_or_market_seed              (Atom 0.5 fence)
tests/tb_13_complete_set.rs:25://! - SG-13.7   no_f64_in_new_complete_set_or_market_seed_path     (Atom 0.5 fence)
tests/tb_13_complete_set.rs:451:// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
tests/tb_13_complete_set.rs:456:/// SG-13.7 (delegation marker) — `no_f64_in_new_complete_set_or_market_seed_path`
tests/tb_13_complete_set.rs:457:/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed`.
tests/tb_13_complete_set.rs:462:fn sg_13_7_no_f64_in_new_complete_set_or_market_seed_path() {
tests/tb_13_complete_set.rs:464:    // This test passes by construction: any f64 leak would be caught by
tests/tb_13_complete_set.rs:474:        fence_src.contains("fn no_f64_in_complete_set_or_market_seed"),
src/state/q_state.rs:167:    pub price_index_t: PriceIndex,
src/state/q_state.rs:685:pub struct PriceIndex(pub BTreeMap<TxId, MicroCoin>);
src/state/typed_tx.rs:596:// those land in TB-13 (CompleteSet) + TB-14 (PriceIndex) + TB-16
src/state/typed_tx.rs:1070:// MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic
src/state/typed_tx.rs:1071:// liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64.
src/state/typed_tx.rs:2803:        "8bb33232b7c20a63a206f505179b0f64fa50acb41061aaa471ba8e4435593aed";

 succeeded in 118ms:
642:+// **Forbidden in TB-13** (architect §4.7): AMM / CPMM / orderbook /
643:+// MarketOrderTx / MarketTradeTx / PriceIndex / DPMM / pro-rata / automatic
644:+// liquidity / ghost liquidity / NodeMarketEntry as canonical state / f64.

 succeeded in 121ms:
ed in `total_supply_micro`. Mint mints equal
+    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
+    /// winning side at 1 share = 1 MicroCoin against collateral.
+    ///
+    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
+    #[serde(default)]
+    pub conditional_share_balances_t: ConditionalShareBalances,
@@ -494,0 +520,48 @@ pub struct NodePositionsIndex(
+// ────────────────────────────────────────────────────────────────────────────
+// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
+// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
+// conditional-share substrate. **1 locked Coin = 1 YES_E + 1 NO_E.**
+// ────────────────────────────────────────────────────────────────────────────
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.4): per-event Coin
+/// collateral locked against outstanding YES_E + NO_E share inventory.
+///
+/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
+/// at `monetary_invariant::assert_total_ctf_conserved`. Mint/seed credit
+/// this map; redeem debits it. The complete-set balanced invariant
+/// (`assert_complete_set_balanced`) enforces
+/// `Σ_{owner} share[(owner, event, Yes)] == Σ_{owner} share[(owner, event, No)] == collateral[event]`.
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ConditionalCollateralIndex(
+    pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>,
+);
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2): per-
+/// `(owner, event_id)` share balance pair (YES + NO sides).
+///
+/// **IS NOT** a Coin holding — shares are claims against
+/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
+/// explicit: shares are NOT counted in `total_supply_micro`.
+///
+/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
+/// Nested-map shape (rather than tuple-key) keeps the structure
+/// JSON-friendly (BTreeMap with tuple keys is not serializable through
+/// serde_json) while preserving canonical Owner-major / Event-minor
+/// ordering for replay determinism.
+#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ConditionalShareBalances(
+    pub BTreeMap<
+        AgentId,
+        BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>,
+    >,
+);
+
+/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + FR-13.3): YES + NO share
+/// holdings for a `(owner, event_id)` pair. Mint and seed credit
+/// equally; redeem debits the winning side.
+#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
+pub struct ShareSidePair {
+    pub yes: crate::state::typed_tx::ShareAmount,
+    pub no: crate::state::typed_tx::ShareAmount,
+}
+
@@ -700 +773 @@ mod tests {
-    fn economic_state_has_eleven_sub_fields() {
+    fn economic_state_has_thirteen_sub_fields() {
@@ -704,0 +778,5 @@ mod tests {
+        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3):
+        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
+        // holding, included in 6-holding total_supply_micro) +
+        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
+        // total_supply_micro).
@@ -710,2 +788,2 @@ mod tests {
-            11,

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '780,820p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   780	/// UpheldDeferred = TB-5.1 marker-only path (slash deferred to TB-6).
   781	/// Lives in typed_tx.rs alongside ChallengeResolveTx; ChallengeStatus
   782	/// (Open/Released/UpheldDeferred for case-state tracking) lives in
   783	/// q_state.rs per Codex round-2 + round-3 Q4 single-source-of-truth.
   784	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
   785	#[repr(u8)]
   786	pub enum ChallengeResolution {
   787	    Released = 0,
   788	    UpheldDeferred = 1,
   789	}
   790	
   791	impl Default for ChallengeResolution {
   792	    fn default() -> Self {
   793	        Self::Released
   794	    }
   795	}
   796	
   797	// ────────────────────────────────────────────────────────────────────────────
   798	// § 7 Signing payloads (CO1.1.4-pre1 v1.1 round-1 closure C-1)
   799	//
   800	// Each agent-signed and system-emitted typed-tx has a paired `*SigningPayload`
   801	// struct (subset of fields, EXCLUDES the signature itself) with a
   802	// `canonical_digest()` method that **prepends a stable domain-separation
   803	// prefix** before the bincode-canonical body bytes. This implements:
   804	//
   805	//   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
   806	//
   807	// Property: even if two distinct payload TYPES happen to bincode-encode to
   808	// identical bytes (extremely unlikely given distinct field shapes, but
   809	// defensively guaranteed), the domain prefix ensures the SHA-256 inputs
   810	// differ. Closes Codex Q-E + Gemini Q7: type-level distinction is necessary
   811	// but not sufficient as a security boundary.
   812	//
   813	// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
   814	// + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory; this
   815	// atom only freezes the canonical_digest pre-image.
   816	// ────────────────────────────────────────────────────────────────────────────
   817	
   818	const DOMAIN_AGENT_WORK: &[u8] = b"turingosv4.agent_sig.work.v1";
   819	const DOMAIN_AGENT_VERIFY: &[u8] = b"turingosv4.agent_sig.verify.v1";
   820	const DOMAIN_AGENT_CHALLENGE: &[u8] = b"turingosv4.agent_sig.challenge.v1";

exec
/bin/bash -lc "nl -ba handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "l4_entries": 3,
     3	  "l4e_entries": 2,
     4	  "ledger_root_verified": true,
     5	  "system_signatures_verified": true,
     6	  "state_reconstructed": true,
     7	  "economic_state_reconstructed": true,
     8	  "cas_payloads_retrievable": true,
     9	  "agent_signatures_verified": true,
    10	  "proposal_telemetry_cas_retrievable": true,
    11	  "run_id": "tb13-smoke",
    12	  "epoch": 1,
    13	  "detail": {
    14	    "final_state_root_hex": "1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9",
    15	    "final_ledger_root_hex": "93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb",
    16	    "head_commit_oid_hex": "38f1b3957834052aac42169598f92016d756c331",
    17	    "l4e_last_hash_hex": "79325795bf2ebc78a9330c06c173bb0c502ee283fbfa5b46f569551314e9e23a",
    18	    "replay_failure": null,
    19	    "initial_q_state_loaded_from_disk": true
    20	  }
    21	}

exec
/bin/bash -lc "find handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171 -maxdepth 2 -type f | sort | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/run_summary.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json

exec
/bin/bash -lc "nl -ba handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json | sed -n '1,160p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "q_t": {
     3	    "agents": {},
     4	    "current_round": 0
     5	  },
     6	  "head_t": "",
     7	  "state_root_t": [
     8	    0,
     9	    0,
    10	    0,
    11	    0,
    12	    0,
    13	    0,
    14	    0,
    15	    0,
    16	    0,
    17	    0,
    18	    0,
    19	    0,
    20	    0,
    21	    0,
    22	    0,
    23	    0,
    24	    0,
    25	    0,
    26	    0,
    27	    0,
    28	    0,
    29	    0,
    30	    0,
    31	    0,
    32	    0,
    33	    0,
    34	    0,
    35	    0,
    36	    0,
    37	    0,
    38	    0,
    39	    0
    40	  ],
    41	  "tape_view_t": {
    42	    "views": {}
    43	  },
    44	  "ledger_root_t": [
    45	    0,
    46	    0,
    47	    0,
    48	    0,
    49	    0,
    50	    0,
    51	    0,
    52	    0,
    53	    0,
    54	    0,
    55	    0,
    56	    0,
    57	    0,
    58	    0,
    59	    0,
    60	    0,
    61	    0,
    62	    0,
    63	    0,
    64	    0,
    65	    0,
    66	    0,
    67	    0,
    68	    0,
    69	    0,
    70	    0,
    71	    0,
    72	    0,
    73	    0,
    74	    0,
    75	    0,
    76	    0
    77	  ],
    78	  "predicate_registry_root_t": [
    79	    0,
    80	    0,
    81	    0,
    82	    0,
    83	    0,
    84	    0,
    85	    0,
    86	    0,
    87	    0,
    88	    0,
    89	    0,
    90	    0,
    91	    0,
    92	    0,
    93	    0,
    94	    0,
    95	    0,
    96	    0,
    97	    0,
    98	    0,
    99	    0,
   100	    0,
   101	    0,
   102	    0,
   103	    0,
   104	    0,
   105	    0,
   106	    0,
   107	    0,
   108	    0,
   109	    0,
   110	    0
   111	  ],
   112	  "tool_registry_root_t": [
   113	    0,
   114	    0,
   115	    0,
   116	    0,
   117	    0,
   118	    0,
   119	    0,
   120	    0,
   121	    0,
   122	    0,
   123	    0,
   124	    0,
   125	    0,
   126	    0,
   127	    0,
   128	    0,
   129	    0,
   130	    0,
   131	    0,
   132	    0,
   133	    0,
   134	    0,
   135	    0,
   136	    0,
   137	    0,
   138	    0,
   139	    0,
   140	    0,
   141	    0,
   142	    0,
   143	    0,
   144	    0
   145	  ],
   146	  "economic_state_t": {
   147	    "balances_t": {
   148	      "Agent_0": 1000000,
   149	      "Agent_1": 1000000,
   150	      "Agent_2": 1000000,
   151	      "Agent_3": 1000000,
   152	      "Agent_4": 1000000,
   153	      "Agent_5": 1000000,
   154	      "Agent_6": 1000000,
   155	      "Agent_7": 1000000,
   156	      "Agent_8": 1000000,
   157	      "Agent_9": 1000000,
   158	      "Agent_user_0": 10000000,
   159	      "tb7-7-sponsor": 10000000
   160	    },

exec
/bin/bash -lc "nl -ba handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json | sed -n '146,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   146	  "economic_state_t": {
   147	    "balances_t": {
   148	      "Agent_0": 1000000,
   149	      "Agent_1": 1000000,
   150	      "Agent_2": 1000000,
   151	      "Agent_3": 1000000,
   152	      "Agent_4": 1000000,
   153	      "Agent_5": 1000000,
   154	      "Agent_6": 1000000,
   155	      "Agent_7": 1000000,
   156	      "Agent_8": 1000000,
   157	      "Agent_9": 1000000,
   158	      "Agent_user_0": 10000000,
   159	      "tb7-7-sponsor": 10000000
   160	    },
   161	    "escrows_t": {},
   162	    "stakes_t": {},
   163	    "claims_t": {},
   164	    "reputations_t": {},
   165	    "task_markets_t": {},
   166	    "royalty_graph_t": {},
   167	    "challenge_cases_t": {},
   168	    "price_index_t": {},
   169	    "runs_t": {},
   170	    "node_positions_t": {},
   171	    "conditional_collateral_t": {},
   172	    "conditional_share_balances_t": {}
   173	  },
   174	  "budget_state_t": {
   175	    "cost_ceiling_microcoin": 0,
   176	    "wall_clock_remaining_ms": 0,
   177	    "compute_cap_remaining": 0
   178	  }
   179	}

exec
/bin/bash -lc "rg --line-number \"negative|is_negative|<= 0|micro_units\\(\\) <=|micro_units\\(\\) == 0|from_micro_units\\(-|InsufficientCollateral\" src/state/sequencer.rs tests/tb_13_complete_set.rs src/economy/monetary_invariant.rs src/state/typed_tx.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:1098:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
src/state/typed_tx.rs:1101:/// shares can never be negative (mint creates positive, redeem decreases
src/state/typed_tx.rs:1173:///    `InsufficientCollateral` (defensive; should never fire if
src/state/typed_tx.rs:1194:/// 1. `collateral_amount > 0` else `InsufficientCollateral` (SG-13.4).
src/state/typed_tx.rs:1864:    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
src/state/typed_tx.rs:1870:    InsufficientCollateral,
src/state/typed_tx.rs:1955:            Self::InsufficientCollateral => write!(
src/state/sequencer.rs:556:            if work.stake.micro_units() <= 0 {
src/state/sequencer.rs:676:            if verify.bond.micro_units() == 0 {
src/state/sequencer.rs:831:            if challenge.stake.micro_units() == 0 {
src/state/sequencer.rs:1588:            if mint.amount.micro_units() == 0 {
src/state/sequencer.rs:1664:        //     InsufficientCollateral.
src/state/sequencer.rs:1726:                return Err(TransitionError::InsufficientCollateral);
src/state/sequencer.rs:1807:            if seed.collateral_amount.micro_units() == 0 {
src/state/sequencer.rs:1808:                return Err(TransitionError::InsufficientCollateral);
src/state/sequencer.rs:3783:    /// U13 — VerifyTx with bond.micro_units() == 0 rejects with BondInsufficient.
src/state/sequencer.rs:3935:    /// U18 — ChallengeTx with stake.micro_units() == 0 rejects with StakeInsufficient.
tests/tb_13_complete_set.rs:300:    // collateral_amount == 0 must fail with InsufficientCollateral.
tests/tb_13_complete_set.rs:305:        err.contains("InsufficientCollateral"),
tests/tb_13_complete_set.rs:306:        "expected InsufficientCollateral, got: {err}"

codex
I have enough to issue the audit. I found two blocking defects: negative `MicroCoin` amounts are accepted by the TB-13 mint/seed arms, and the TB-13 “agent-signed” variants are never live-verified at submit/apply time. I’m going to try to write the requested audit file now; this workspace is read-only, so the write may be rejected.
2026-05-03T06:53:56.838296Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
codex
I attempted to save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`, but the workspace is read-only and the patch was rejected. I completed the audit from the checked-out files.

## VERDICT: VETO

- TB13-V1 VETO: Negative `MicroCoin` amounts are accepted by `CompleteSetMint` and `MarketSeed`. `MicroCoin` is signed and allows negative values at the type layer ([money.rs](/home/zephryj/projects/turingosv4/src/economy/money.rs:27)); the TB-13 arms reject only `== 0`, not `< 0` ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1587), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1806)). A negative mint/seed credits balance, writes negative collateral, and casts the negative amount to huge `u128` shares ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1607), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1622), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1821), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1839)). This blocks Q1, Q6, and Q8.

- TB13-V2 VETO: The TB-13 `AgentSignature` fields are not live-verified. The structs/signing payloads exist ([typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1150), [typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1221)), but `submit_agent_tx` just accepts/enqueues these variants ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2319)), and `apply_one` only verifies system signatures while TB-13 variants return `None` for that path ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:446), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2853)). The TB-13 integration builders use all-zero signatures and are accepted ([tb_13_complete_set.rs](/home/zephryj/projects/turingosv4/tests/tb_13_complete_set.rs:128)). This is a Class-3 money/collateral auth blocker.

Other audit results: Q2/Q3/Q7 pass on the current state-gating and share-exclusion logic. Q4 passes only for non-negative flows. Q5’s MIN invariant is mathematically right post-redeem, but it is not called by the sequencer TB-13 arms. Q9 is a challenge: the forward fence scans only fixed files and TB-13 marker spans, so a legacy import outside a marked span would bypass it ([tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:51), [tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:122)).

Conviction: high.  
Recommendation: FIX-THEN-PROCEED.
tokens used
191,018
I attempted to save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`, but the workspace is read-only and the patch was rejected. I completed the audit from the checked-out files.

## VERDICT: VETO

- TB13-V1 VETO: Negative `MicroCoin` amounts are accepted by `CompleteSetMint` and `MarketSeed`. `MicroCoin` is signed and allows negative values at the type layer ([money.rs](/home/zephryj/projects/turingosv4/src/economy/money.rs:27)); the TB-13 arms reject only `== 0`, not `< 0` ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1587), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1806)). A negative mint/seed credits balance, writes negative collateral, and casts the negative amount to huge `u128` shares ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1607), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1622), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1821), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1839)). This blocks Q1, Q6, and Q8.

- TB13-V2 VETO: The TB-13 `AgentSignature` fields are not live-verified. The structs/signing payloads exist ([typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1150), [typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1221)), but `submit_agent_tx` just accepts/enqueues these variants ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2319)), and `apply_one` only verifies system signatures while TB-13 variants return `None` for that path ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:446), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2853)). The TB-13 integration builders use all-zero signatures and are accepted ([tb_13_complete_set.rs](/home/zephryj/projects/turingosv4/tests/tb_13_complete_set.rs:128)). This is a Class-3 money/collateral auth blocker.

Other audit results: Q2/Q3/Q7 pass on the current state-gating and share-exclusion logic. Q4 passes only for non-negative flows. Q5’s MIN invariant is mathematically right post-redeem, but it is not called by the sequencer TB-13 arms. Q9 is a challenge: the forward fence scans only fixed files and TB-13 marker spans, so a legacy import outside a marked span would bypass it ([tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:51), [tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:122)).

Conviction: high.  
Recommendation: FIX-THEN-PROCEED.
