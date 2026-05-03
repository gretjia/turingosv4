OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019decaa-f2e8-71a0-a4b7-ac881f5c0df4
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
I’ll audit the scoped TB-13 range against the mandated questions and repo instructions first, then check the implementation paths and tests/evidence. The filesystem is read-only in this session, so I can inspect and produce the audit content, but I may not be able to create the requested file directly.
  → Read repo instructions and TB-13 audit artifacts
  • Trace sequencer and invariant implementation
  • Check tests, forward fence, and replay evidence
  • Evaluate Q1-Q9/RQ1-RQ7 and draft verdict
  • Attempt audit file write if permitted
exec
/bin/bash -lc pwd in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'handover/tracer_bullets/TB-13_charter_2026-05-03.md' -g 'handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md' -g 'handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md' -g 'handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md
handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md
handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md
handover/tracer_bullets/TB-13_charter_2026-05-03.md
handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md

exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 D handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md.raw
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/

exec
/bin/bash -lc 'git rev-parse HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
07fc8698f8d2ad2428b11ce19d31cc2a05db7c12

codex
There are already local audit-related files and a deleted target path in the worktree, so I’ll avoid changing or reverting anything. I’m going to read the governing instructions and then verify the implementation against the exact line references rather than trusting the handover claims.
exec
/bin/bash -lc "sed -n '1,240p' CLAUDE.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TuringOS v4

## What
Silicon-Native Microkernel for LLM Formal Verification Swarm.
Rust 2021, tokio, serde_json. Mission: MiniF2F Lean 4.

## Why
- 唯一对齐文档: `constitution.md` (反奥利奥架构)
- 压缩即智能: 抽象原则进宪法 / 具体情境进 `cases/`
- 机制 > 参数 > 提示 (Art. V + C-021/C-031/C-034/C-043)

## Code Standard (Art. I.1 + C-004 + C-027)
- `cargo check` / `cargo test` 必过；`.env` 永不 commit
- `src/{kernel,bus}.rs` + `src/sdk/tools/wallet.rs` + `src/state/sequencer.rs` 改动走 STEP_B_PROTOCOL（不直接编辑 main）
- 任何影响行为的参数必须 env/config 可覆盖，不可硬编码

## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
- Generator ≠ Evaluator：代码作者不可是唯一审计者
- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
- 宪法违规立即 BLOCKER，不可延期、不可"可接受"

## Report Standard (Art. I.2 + Art. II.2.1 + Art. IV 强制, C-052 + C-053 + C-057 + C-059 + C-061)
- **主指标**（每报必填）: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)
- Art. I.2 三大统计信号不可缺: **信誉** (reputation_distribution p50/p90/max) + 效用 (PPUT) + 共识 (如适用)
- Art. IV 终态区分: `halt_reason_distribution` {OmegaAccepted, MaxTxExhausted, WallClockCap, ComputeCapViolated, ErrorHalt}
- 多 agent (n≥2) 专用: `parent_selection_entropy` + `pairwise_payload_diversity_mean`；任一 < 0.25 = Art. II.2.1 告警
- solve count 不可独立陈述，必须配对 PPUT；以 solve count 起头 = 违宪

## Reproducibility Standard (Art. I + C-012/C-016/C-032/C-039)
- OMEGA accept 必留 self-contained artifact (`proofs/*.lean` + `gp_payload`)
- 度量工具上线即冻结；Oracle 参数冻结；实验禁混 Oracle 模式
- 中间件若修改数学内容 → 是 ArchitectAI 贡献，不是 swarm 涌现（C-023）

## Alignment Standard (Art. IV + C-069)
- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
  (后续 rev: `TRACE_MATRIX_vN.md`)
- 每个 src/ pub 符号必须映射到宪法 flowchart 元素、标 orphan+justification、
  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`
- Conformance tests: `tests/fc_alignment_conformance.rs` — 每个 ✅ 行 ≥1
  witness test；`#[ignore]` stub 覆盖 📅 deferred rows
- 宪法 flowchart 修改仅 human architect 可触发，需重跑 Phase Z′ 6-stage
- constitution.md hygiene 观察登记到 `handover/alignment/OBS_*.md`，不改宪法

## Common Law (宪法 + 判例)
宪法高度压缩，具体裁决查 `cases/C-xxx.yaml` (facts → ruling → precedent)
- 按条款查: `grep -l "Art. I.1" cases/*.yaml`
- 映射：`cases/V3_LESSONS.md` (50 v3 教训 → 现行判例)
- 编号跳号：C-038 / C-042 为 reserved（见 C-041/C-043 预引用）

## Docs (按需加载)
| 文档 | 何时加载 |
|------|---------|
| `docs/architecture.md` | 修改 src/ 核心模块时 |
| `docs/economics.md` | 修改经济引擎 (wallet/market) 时 |
| `docs/hardware.md` | SSH/部署/远程操作时 |
| `docs/experiments.md` | 创建或运行实验时 |
| `docs/rules.md` | 触发规则或修改规则时 |

## User
独狼研究员, 零编程基础 vibe coder. 中文为主, 技术术语英文可.

exec
/bin/bash -lc 'for f in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md; do [ -f "$f" ] && printf '"'%s\\n' \""'$f"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "sed -n '1,260p' handover/tracer_bullets/TB-13_charter_2026-05-03.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-13 Charter — CompleteSet + MarketSeedTx (Conditional-Share Substrate)

**Status**: **DRAFT 2026-05-03 evening** (post-TB-12 SHIPPED `fa36eca`). Awaiting user-architect ratification before Atom 0.5 begins. Architect `proceed and recursively check at the best place you decide` reads as ratification + checkpoint authority delegated to AI coder.
**Date**: 2026-05-03 evening (drafted same day as TB-12 SHIPPED + 2026-05-03 architect post-TB-12 ruling).
**Predecessor**: TB-12 SHIPPED 2026-05-03 (`fa36eca`).
**Authority**: 2026-05-03 architect post-TB-12 ruling lossless archive at `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` (operative spec, §4 TB-13 + §11 loop-mode master instruction) + 2026-05-02 supplementary directive at `handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md` (TB-13 base spec) + 2026-05-02 lossless integrated edition Part C §9.4 (TB-13 forbidden list canonical).
**Phase**: **P3 carry-forward** (RSP-M Polymarket collateral substrate per 2026-05-02 directive Part C §9.4) + **P4 primary** (CompleteSet is the conditional-share substrate that TB-15 MarkovEvidenceCapsule will compress into Information Loom rollups; first canonical YES_E + NO_E claim accounting).
**Risk class envelope**: **Class 3** (per architect §4.8). Per-atom: Atom 0.5 = Class 1 (forward-fence + label, no production deletion). Atom 1 = Class 2 (additive schema). Atom 2 = **Class 3** (sequencer money path — debits `balances_t`, credits new `conditional_collateral_t`, issues `conditional_share_balances_t`). Atom 3 = Class 3 (extends `monetary_invariant`; touches CTF). Atom 4 = Class 1 (dashboard read-only). Atom 5 = Class 1 (tests). Ship audit (Atom 6) = **Class 3 dual** (Codex + Gemini, no honest-deferral; conservative-verdict-wins).
**Iteration cap**: **72h with 24h checkpoints** (production wire-up exception per `feedback_iteration_cap_24h`; conservation surface).

**Operating mode** (mirrors TB-12 Q6 ii.5 sync mode + architect §4.8 + user 2026-05-03 evening "proceed and recursively check at the best place you decide"):
- Atoms 0.5 → 5 continuous; AI-coder-decided micro-checkpoints at:
  - **CP-A** after Atom 0.5 (cargo test --workspace baseline preserved; forward fence in place).
  - **CP-B** after Atom 2 (sequencer dispatch; STEP_B-protocol; conservation pre-check via assert_total_ctf_conserved).
  - **CP-C** after Atom 3 (monetary_invariant extended; this is the THE money-safety gate).
- Atom 6 dual audit runs continuously; **STOP after audit verdict**.
- Atom 7 SHIP ONLY after explicit user authorization on the verdict.

**FC-trace**: `Art. 0.2` (Tape Canonical — `CompleteSetMintTx` / `CompleteSetRedeemTx` / `MarketSeedTx` canonical-encoded; replay-deterministic) + `Art. I.1` (5-step compile loop — conditional collateral and share accounting are part of the proposal-pricing substrate, but DO NOT affect predicate outcome; CR-13.6 explicit) + `Art. III.4` (no fake accepted preserved — Redeem requires system-resolved outcome reference; agent-callable Redeem rejected pre-resolution) + `Art. V.1.3` (Anti-Oreo — `MarketSeedTx` requires explicit provider funds, NO automatic seed; `CompleteSetMintTx` is agent-driven balance debit, no money created) + `WP-§14.1` (typed_tx variant additive bumps; 3 NEW typed-tx variants; CompleteSetRedeem additionally accepts a `ResolutionRef` from system-emitted resolution; not a unilateral agent variant since pre-resolution rejection is sequencer-side gate) + `WP-§19` (RSP-M collateral substrate — conditional collateral is the YES_E + NO_E claim foundation that TB-14 PriceIndex will derive prices from).

**Flowchart-trace**: **Flowchart 1** (runtime — `CompleteSetMintTx` debits `balances_t`, credits `conditional_collateral_t`, increments equal `conditional_share_balances_t[(owner, event, Yes/No)]`; `CompleteSetRedeemTx` requires system-resolution-reference, debits `conditional_share_balances_t`, credits `balances_t`; `MarketSeedTx` debits `balances_t` of provider, credits `conditional_collateral_t` + records protocol-owned share inventory) + **Flowchart 2** (boot — runtime preseed unchanged; EconomicState 11→13 sub-fields with backward-compat `#[serde(default)]` empty maps) + **Flowchart 3** (meta — conditional shares are TB-14 PriceIndex price-derivation substrate + TB-15 MarkovEvidenceCapsule compression input; TB-13 lays bytes only; no price, no AMM, no orderbook).

**Phase declarations** (per `feedback_tb_phase_tag_required`):

```text
phase_id: P3 carry-forward (RSP-M collateral substrate per 2026-05-02 §9.4)
          + P4 primary (CompleteSet conditional-share substrate; first
                        canonical YES_E + NO_E claim accounting; TB-15
                        Information Loom compression input)
roadmap_exit_criteria_addressed:
  P3:RSP-M  1 locked Coin = 1 YES_E + 1 NO_E mathematical core (architect
            §4.4 FR-13.1 / FR-13.2 / FR-13.3); CTF conserved across mint
            (architect SG-13.1)
  P3:RSP-M  Redeem requires system-resolved outcome (architect FR-13.4
            + SG-13.5); after-YES outcome pays YES not NO (FR-13.5 + SG-13.6)
  P3:RSP-M  MarketSeedTx requires explicit provider funds (FR-13.6 +
            SG-13.3); no auto-seed (forbidden list Part A §4.7); no
            quote/trade/price (FR-13.7)
  P4:1      First canonical YES_E + NO_E claim accounting (replaces
            BinaryMarket f64 CPMM book; EconomicState +2 sub-fields)
  P4:2      Forward-binding fence: TB-13 modules cannot import legacy
            prediction_market.rs (forbidden-token grep ship-gate)
  P4:3      Conditional collateral as Coin holding; conditional shares
            as claims (architect CR-13.3 + CR-13.4); 6-holding CTF sum
            (extends 5→6 holdings; conditional_collateral_t IS a holding)
kill_criteria_tested:
  P3:1      No post-init mint — CompleteSetMintTx is balance-DEBIT-then-
            collateral-CREDIT pair; total Coin conserved bit-for-bit
            (architect SG-13.1; assert_total_ctf_conserved unchanged-output
            after extending exempt list with conditional_collateral_t)
  P3:2      No stake-less write — CompleteSetMintTx with amount==0
            either rejected upstream or NO conditional state mutation;
            MarketSeedTx with collateral==0 rejected (SG-13.4 cannot
            create liquidity without collateral)
  P3:3      Payout sum ≤ collateral — CompleteSetRedeemTx after YES
            redeems at most conditional_share_balances_t[(owner, event,
            Yes)] units against conditional_collateral_t[event]; CTF
            preserved across redeem (no payout > collateral)
  P3:4      No agent-self-reported reward — Redeem requires
            ResolutionRef pointing to a system-emitted resolution (no
            agent unilateral close pre-resolution); SG-13.5 enforced
  P3:5      No full payout pre-challenge-window — Redeem rejected if
            ResolutionRef.outcome state != Resolved or if challenge
            window for the resolving event is still open
  P3:8      Settlement ≤ escrow — TB-8 invariant unchanged; TB-13 does
            NOT touch escrows_t / claims_t finalize path; CompleteSet
            collateral is its own pool
  P4:4      Conditional shares NOT in total_supply_micro — CR-13.3
            verified via SG-13.2 (assert_total_ctf_conserved with
            conditional_share_balances_t in EXEMPT list, not in sum)
flowchart_trace:
  Flowchart 1 (runtime): CompleteSetMintTx → balances_t debit +
                         conditional_collateral_t credit + conditional_
                         share_balances_t equal-YES-NO mint;
                         CompleteSetRedeemTx → conditional_share_balances_t
                         debit + balances_t credit (only after system-
                         resolved outcome);
                         MarketSeedTx → provider balances_t debit +
                         conditional_collateral_t credit + protocol-
                         owned share inventory record (NO trading).
  Flowchart 2 (boot):    EconomicState additive +conditional_collateral_t
                         +conditional_share_balances_t (11→13).
  Flowchart 3 (meta):    Conditional shares are TB-14 PriceIndex price-
                         derivation substrate + TB-15 Information Loom
                         compression input; TB-13 lays bytes only.
```

---

## §0 Why TB-13 exists (architect post-TB-12 ruling §4.1)

**Architect spec verbatim** (lossless archive Part A §4.1):

```text
TB-13 = CompleteSet + MarketSeedTx
引入 Polymarket / CTF 的数学核心：
  1 locked Coin = 1 YES_E + 1 NO_E

但 TB-13 仍然不做：
  AMM / CPMM router / orderbook / MarketOrderTx / MarketTradeTx /
  PriceIndex / DPMM / pro-rata / automatic liquidity

TB-13 只做抵押与份额会计。
```

**Driver context** (architect Part A §1.3 + §4.1):
- TB-12 SHIPPED Node Exposure Index (`fa36eca`); WorkTx.stake → FirstLong, ChallengeTx.stake → ChallengeShort EXPOSURE RECORDS now exist.
- NodePosition is exposure record, NOT YES/NO share, NOT tradable balance, NOT collateral.
- TB-13 introduces the FIRST canonical YES/NO claim accounting; closes the gap between "exposure recorded" (TB-12) and "directional claim against collateral" (TB-13).
- Constitutional invariants preserved: 1 Coin = 1 YES + 1 NO mathematical identity; no automatic injection; on_init unique mint.
- Forward fence: TB-13 modules MUST NOT import legacy `src/prediction_market.rs` f64 CPMM scaffolding (architect §4.2 SG-13.0.1 + halting trigger 1).

---

## §1 One-line goal

```text
Goal: Introduce conditional-share accounting per Polymarket / CTF math
      "1 locked Coin = 1 YES_E + 1 NO_E" via 3 NEW typed_tx variants
      (CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx) and 2 NEW
      EconomicState sub-fields (conditional_collateral_t,
      conditional_share_balances_t). CompleteSetMint debits Coin balance,
      credits conditional collateral, mints equal YES_E + NO_E shares.
      CompleteSetRedeem requires system-resolved outcome; after YES
      outcome pays YES, after NO outcome pays NO; pre-resolution
      rejected. MarketSeedTx uses explicit provider funds; no auto-seed;
      no quote/trade/price. CTF conserved (1 Coin → 1 YES + 1 NO);
      shares NOT in total Coin supply. NO AMM. NO CPMM. NO orderbook.
      NO PriceIndex (TB-14). NO trading.

Pre-gate (Atom 0.5): forward fence + label. NEW TB-13 modules cannot
      import src/prediction_market.rs; forbidden-token grep ship-gate
      enforces this. Existing kernel.rs / bus.rs / evaluator.rs CPMM
      wiring is documented as legacy and carry-forward to TB-14 SHIP
      prerequisite (per OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03;
      retroactive deletion would break 759 passing tests + the
      production evaluator and is OUT OF SCOPE per
      feedback_no_retroactive_evidence_rewrite).
```

---

## §2 What's already shipped (TB-3..TB-12 foundations + TB-13 dependencies)

| Foundation | Source | Status |
|---|---|---|
| `MicroCoin` integer math (no f64 in money path) | `src/economy/money.rs` | shipped TB-3 |
| `BalancesIndex`, `EscrowsIndex`, `StakesIndex`, `ClaimsIndex` | `src/state/q_state.rs:158-162` | shipped TB-3..TB-7R |
| `EconomicState` 11 sub-fields (incl. node_positions_t added by TB-12) | `src/state/q_state.rs:158-192` | shipped TB-3..TB-12 |
| `assert_total_ctf_conserved` 5-holding sum (now 5; conditional_collateral_t will become 6th holding) | `src/economy/monetary_invariant.rs` | shipped TB-7R |
| `assert_no_post_init_mint` exhaustive match (incl. TaskBankruptcy + TB-12 typed-tx unchanged) | `src/economy/monetary_invariant.rs` | shipped TB-3+ |
| `TypedTx` enum + canonical signing payload + state-root domain prefix | `src/state/typed_tx.rs` | shipped TB-3..TB-12 |
| Sequencer dispatch arm pattern (additive variants safe; agent-ingress fail-closed for system-emitted variants) | `src/state/sequencer.rs` | shipped TB-11 + TB-12 |
| `audit_dashboard.rs` §1-§13 (incl. §13 TB-12 Node exposure records) | `src/bin/audit_dashboard.rs` | shipped TB-12 |
| OBS_TB_12_LEGACY_CPMM_QUARANTINE (forward-fence rationale + TB-14 SHIP prerequisite) | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` | open OBS |

**Net-new in TB-13** (per architect §4.2 + §4.3 + §4.4 + §4.5 + §4.6 + §4.7):

- **Atom 0.5** (legacy CPMM forward-fence + label):
  - Forbidden-token grep ship-gate test (`tests/tb_13_legacy_cpmm_forward_fence.rs`) failing if NEW TB-13 module references `prediction_market::` or `BinaryMarket` or `buy_yes` or `buy_no` or `f64` in a money-path context.
  - Doc-comment "LEGACY: not constitutional, not RSP-M, not production market path; carry-forward to TB-14 SHIP prerequisite per OBS_TB_12_LEGACY_CPMM_QUARANTINE" added to `src/prediction_market.rs` module header + `src/kernel.rs` market field doc-comments.
  - OBS_TB_12_LEGACY_CPMM_QUARANTINE updated to reflect TB-13 forward-fence in place + still carrying TB-14 SHIP prerequisite.
  - **NO retroactive deletion** of `src/prediction_market.rs` or `src/kernel.rs` market scaffolding (production evaluator + 10+ test files would break; out-of-scope per `feedback_no_retroactive_evidence_rewrite`; TB-14 SHIP prerequisite per OBS).

- **Atom 1** (typed_tx schemas):
  - `EventId` newtype (`pub struct EventId(pub TaskId)` — for TB-13 we resolve events 1:1 with task_id; future TB-14+ may decouple).
  - `OutcomeSide` enum `{Yes = 0, No = 1}` with `#[repr(u8)]`.
  - `ShareAmount` newtype `pub struct ShareAmount { pub units: u128 }` (u128 not i128 — shares are non-negative claims; mint creates positive, redeem decreases positive; never negative).
  - `ResolutionRef` newtype referencing the system-emitted resolution (TB-13 reuses `TaskBankruptcyTx.tx_id` for NO outcome + reuses `FinalizeRewardTx.tx_id` for YES outcome — both already system-emitted; this is the "system resolution reference" architect §4.3 mentions).
  - `CompleteSetMintTx { tx_id, parent_state_root, event_id, owner, amount: MicroCoin, signature }` agent-callable.
  - `CompleteSetRedeemTx { tx_id, parent_state_root, event_id, owner, outcome: OutcomeSide, share_amount: ShareAmount, resolution_ref: ResolutionRef, signature }` agent-callable but pre-resolution rejected at sequencer.
  - `MarketSeedTx { tx_id, parent_state_root, event_id, provider, collateral_amount: MicroCoin, signature }` agent-callable; provider == seed_owner.
  - All 3 added to `TypedTx` enum + `CanonicalMessage::*Signing` + 3 new `DOMAIN_*_V1` prefixes + `TxKind::*` discriminants.
  - Round-trip + deterministic-digest unit tests; STEP_B_PROTOCOL parallel-branch since touches `src/state/typed_tx.rs` restricted file.

- **Atom 2** (sequencer dispatch + EconomicState extension):
  - `EconomicState` 11 → 13 sub-fields with `+conditional_collateral_t: ConditionalCollateralIndex` + `+conditional_share_balances_t: ConditionalShareBalances`. Both `#[serde(default)]` for backward-compat with TB-12 chain snapshots.
  - `ConditionalCollateralIndex(BTreeMap<EventId, MicroCoin>)` — locked Coin per event.
  - `ConditionalShareBalances(BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>)` — share holdings per (owner, event, side).
  - 3 new dispatch arms in `src/state/sequencer.rs`:
    - `CompleteSetMintTx`: validate balance >= amount; debit `balances_t[owner]` by amount; credit `conditional_collateral_t[event_id]` by amount; credit equal share_amount=amount.units to `conditional_share_balances_t[(owner, event, Yes)]` AND `conditional_share_balances_t[(owner, event, No)]`. (Architect FR-13.1 + FR-13.2 + FR-13.3.)
    - `CompleteSetRedeemTx`: validate `resolution_ref` resolves to either a `TaskBankruptcyTx` (NO outcome) or a `FinalizeRewardTx` (YES outcome) for `event_id`; verify outcome matches Tx outcome field; debit `conditional_share_balances_t[(owner, event, outcome)]` by share_amount; credit `balances_t[owner]` by share_amount.units (1:1 per architect FR-13.5: "after YES outcome pays YES shares"); debit `conditional_collateral_t[event_id]` by share_amount.units. Pre-resolution rejected (architect FR-13.4 + SG-13.5).
    - `MarketSeedTx`: validate `balances_t[provider] >= collateral_amount`; debit `balances_t[provider]`; credit `conditional_collateral_t[event_id]` by collateral_amount; credit `conditional_share_balances_t[(provider, event, Yes)]` AND `(provider, event, No)` each by collateral_amount.units (provider owns BOTH sides; explicit provider-funded; NO trading; architect FR-13.6 + FR-13.7).
  - State-root domain prefixes: `COMPLETE_SET_MINT_DOMAIN_V1`, `COMPLETE_SET_REDEEM_DOMAIN_V1`, `MARKET_SEED_DOMAIN_V1`.
  - 3 new `TransitionError` variants (additive): `InsufficientBalanceForMint`, `RedeemBeforeResolution`, `InsufficientCollateral`.
  - STEP_B_PROTOCOL parallel-branch since touches `src/state/sequencer.rs` restricted file.

- **Atom 3** (monetary_invariant extension):
  - Extend `assert_total_ctf_conserved` to add `conditional_collateral_t` as the 6th Coin holding (architect CR-13.4: "Locked collateral is Coin holding"); EXCLUDE `conditional_share_balances_t` from the sum (architect CR-13.3 + SG-13.2: "shares are not Coin").
  - New invariant `assert_complete_set_balanced`: for every event_id, `Σ conditional_share_balances_t[(*, event, Yes)] == Σ conditional_share_balances_t[(*, event, No)] == conditional_collateral_t[event].units` (1 Coin = 1 YES + 1 NO mathematical identity post-mint and post-redeem).
  - New invariant `assert_no_negative_share_balance`: u128 type-level guarantee + sequencer-side underflow check.

- **Atom 4** (dashboard §14):
  - `audit_dashboard.rs` §14 — "Conditional collateral and share balances" sub-tables; label discipline: "claims, not balances" (architect CR-13.3); "locked collateral, not free coin" (architect CR-13.4).
  - Per-event totals: collateral, YES depth, NO depth, completeness check (depth_yes == depth_no == collateral.units).
  - Per-owner share holdings filtered by event.
  - Render-fn unit-testable per TB-12 §13 precedent.

- **Atom 5** (integration tests SG-13.1..8 + halting-trigger guards):
  - `tests/tb_13_complete_set.rs` covering all SG-13.x with EXACT architect-named functions + halting-trigger guard tests.

- **Atom 6** (audit):
  - `RECURSIVE_AUDIT_TB_13_2026-05-XX.md` (4-clause + complete-set hygiene clause).
  - `CODEX_TB_13_SHIP_AUDIT_2026-05-XX.md`.
  - `GEMINI_TB_13_SHIP_AUDIT_2026-05-XX.md`.

- **Atom 7** (SHIP):
  - LATEST.md + TB_LOG.tsv row + ship commit. ONLY after audit verdict.

---

## §3 Deliverables (8 atoms)

### Atom 0 — Charter ratification (Class 0)

**Action**: this document IS the ratification draft. Architect 2026-05-03 evening "proceed and recursively check at the best place you decide" reads as ratification + checkpoint authority delegation; user-architect may override before Atom 0.5.

**Iter cap**: 0.5h.

---

### Atom 0.5 — Legacy CPMM forward-fence + label (Class 1)

**Architect mandate** (Part A §4.2 + §4.7 forbidden list + §4.8 halting triggers):

```text
FR-13.0.1  src/prediction_market.rs legacy f64 CPMM must be quarantined.
FR-13.0.2  New CompleteSet/MarketSeedTx code must not import legacy
           prediction_market.rs.
FR-13.0.3  No f64 in CompleteSet/MarketSeedTx/market accounting path.
FR-13.0.4  Legacy CPMM must be clearly labeled:
           legacy / not used by RSP-M / not constitutional /
           not production market path.
```

**Action**:

(a) Forward-fence test `tests/tb_13_legacy_cpmm_forward_fence.rs`:
   - Read all NEW TB-13 module sources (`src/state/typed_tx.rs` *additions* — identified by git-blame against TB-12 ship `fa36eca`; `src/economy/conditional_market.rs` if created; `src/state/sequencer.rs` *additions*).
   - Forbidden-token grep on those additions: fails if any of `prediction_market::`, `BinaryMarket`, `buy_yes`, `buy_no`, ` f64 ` (in money-path context — exclude unrelated f64 use like timestamps if any), `MarketError` (legacy variant), `open_bounty_market`, `bounty_market`, `bounty_lp_seed` appear.
   - Implementation: read git-tree `fa36eca:src/state/sequencer.rs` vs HEAD; diff additions; grep additions only.

(b) Label discipline:
   - Update `src/prediction_market.rs` module header doc-comment with:
     ```rust
     //! # LEGACY — TB-3..TB-10 Phase-3A Hayek bounty-market scaffolding
     //!
     //! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2):
     //! This module is **legacy / not constitutional / not RSP-M / not
     //! production market path** in the post-2026-05 architect roadmap. It is:
     //! - **NOT** the canonical YES/NO claim system (TB-13 introduces
     //!   `CompleteSetMintTx` + `ConditionalShareBalances` for that).
     //! - **NOT** the canonical price index (TB-14 introduces `PriceIndex`
     //!   derived from `node_positions_t` long/short interest).
     //! - **NOT** authorized for extension or new use sites.
     //!
     //! ## Carry-forward
     //! Removal is a TB-14 SHIP prerequisite per
     //! `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`.
     //! Retroactive deletion in TB-13 would break the production evaluator

exec
/bin/bash -lc "sed -n '1,260p' handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# 2026-05-03 — TB-13 → TB-17 Post-TB-12 Architect Ruling (Lossless Archive)

**Source**: User-architect ultrathink directive delivered 2026-05-03 after TB-12 ship
(commit `fa36eca`).

**Status**: Lossless verbatim archive per `feedback_kolmogorov_compression`. The
original architect text is reproduced below in two parts:
- **Part A** — Audit + roadmap directive (markdown body of the directive).
- **Part B** — Companion roadmap file `TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md`
  pasted at the tail of the directive.

Annotation layer (delta vs prior 2026-05-02 supplementary directive) is in §C
at the bottom; do NOT read annotation as substitute for the verbatim text.

---

## Part A — Architect ultrathink directive (verbatim)

According to a document from 2026-05-03, TB-12 的 shipped summary 显示它已按 NodeMarket Position Index 的窄化目标完成：`WorkTx.stake -> FirstLongPosition`、`ChallengeTx.stake -> ShortPosition`、`VerifyTx.bond != market position`、`NodePosition not Coin holding`，并且用户提供的结果显示 `cargo test --workspace = 759 / 0 / 150`、`8/8 SG-12.x`、`11/11 G-gates`、`14/14 forbidden tokens`、`9/9 immutable-record invariants` 均通过。这个阶段可以接受，但我没有在本轮直接读取到 GitHub commit `fa36eca` 的源码 diff；下面的审计以你提供的 TB-12 summary、已检索到的路线文档和前序宪法/白皮书约束为依据。

我也写了一份本地路线图文件：

[下载：TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md](sandbox:/mnt/data/TuringOS_TB13_to_TB17_Roadmap_After_TB12_2026-05-03.md)

---

# 1. TB-12 阶段性审计结论

## 1.1 接受 TB-12 shipped

我接受 TB-12 的 shipped 状态，但只接受它的**窄化声明**：

```text
TB-12 proved:
  NodePosition exposure index exists.
  WorkTx.stake becomes FirstLongPosition.
  ChallengeTx.stake becomes ChallengeShortPosition.
  VerifyTx.bond is not a market position.
  NodePosition is not Coin holding.
```

TB-12 **没有证明**：

```text
CompleteSet
YES/NO share accounting
MarketSeedTx
PriceIndex
Boltzmann masking
market trading
AMM
CPMM
Polymarket settlement
real-world readiness
```

这点必须在 `LATEST.md` 和 TB_LOG 后续说明里继续保持窄化，避免团队把 TB-12 的“风险敞口索引”误认为“市场已经上线”。

---

## 1.2 TB-12 的关键正确点

你这次没有直接接受 Gemini PASS，而是做了 ship-gate exact naming 对齐，补了 SG-12.6 的真实测试，修正了 SG-12.5 / SG-12.7 / SG-12.8 的命名漂移。这是正确的。

因为在当前项目里，测试名不只是 cosmetic。它已经变成一种 traceability contract：

```text
架构师要求 SG-12.5；
代码里必须有对应 gate；
审计时必须能一眼对应。
```

所以：

```text
“架构正确性 PASS”
不能替代
“指定 ship gate exact-name PASS”
```

你让 AI coder 在 ship 前修掉这些 gap，是对的。

---

## 1.3 需要 carry forward 的 TB-12 风险

TB-12 后续最重要的风险是：**NodePosition 会在后续被误用为 money ledger 或 tradable share balance。**

必须继续写入后续 charter：

```text
NodePosition = immutable exposure record / index
NodePosition != Coin
NodePosition != YES/NO share
NodePosition != tradable balance
NodePosition != LP share
NodePosition.amount not counted in total_supply_micro
```

TB-12 的路线文档本来也明确要求 `NodePosition.amount must not be included in total_supply_micro`，并且 Price / position signal 不能成为 truth。

这条要在 TB-13/TB-14 反复保护，否则 CompleteSet 一接入，很容易出现“双账本”：

```text
stake balance
node position
YES/NO shares
collateral
```

四者不能混。

---

# 2. 总体路线是否要调整？

**核心路线不需要推翻，但 TB-13 必须加一个前置隔离步骤。**

你提供的 TB-12 summary 里已经指出：

```text
TB-13 prerequisite:
  隔离 src/prediction_market.rs legacy f64 CPMM
  已在 OBS_TB_12_LEGACY_CPMM_QUARANTINE 跟踪
```

这是非常重要的。
如果 TB-13 一边引入 `CompleteSet + MarketSeedTx`，一边仓库里还存在旧的 `f64 CPMM` 可被 import，那么未来很容易 drift 回：

```text
f64 price
ghost liquidity
automatic injection
AMM before collateral accounting
```

所以我的第一个路线修正是：

> **TB-13 的 Atom 0.5 必须是 legacy CPMM quarantine。**

不是“顺手做”，而是 TB-13 的前置 ship gate。

---

# 3. 更新后的 TB-13 到 TB-17 总表

```text
TB-13  CompleteSet + MarketSeedTx
       先隔离 legacy f64 CPMM；引入 1 locked Coin = YES_E + NO_E；不做 AMM。

TB-14  PriceIndex v0 + Boltzmann Masking
       价格是统计信号，不是真理；mask 只影响 read view / scheduler。

TB-15  Lamarckian Autopsy + Markov EvidenceCapsule
       私有尸检 + Markov capsule，不广播 raw logs，不自动改谓词。

TB-16  Controlled Market Smoke Arena
       在沙盒里跑 compute + positions + price + mask + autopsy，不接真实资金/真实世界。

TB-17  Real-World Readiness Gate
       只做真实世界准入标准，不执行真实世界任务。
```

这个顺序仍然符合之前确定的原则：

```text
先失败锚点，再市场做空。
先资本释放，再价格机制。
先证据胶囊，再尸检学习。
先受控市场，再真实世界。
```

这也是你前一版路线中明确写出的重排原则。

---

# 4. TB-13 — CompleteSet + MarketSeedTx

## 4.1 目标

引入 Polymarket / CTF 的数学核心：

```text
1 locked Coin = 1 YES_E + 1 NO_E
```

但 TB-13 **仍然不做**：

```text
AMM
CPMM router
orderbook
MarketOrderTx
MarketTradeTx
PriceIndex
DPMM / pro-rata
automatic liquidity
```

TB-13 只做抵押与份额会计。

---

## 4.2 Atom 0.5：legacy CPMM quarantine

这是 TB-13 的第一步。

### Requirements

```text
FR-13.0.1
src/prediction_market.rs legacy f64 CPMM must be quarantined.

FR-13.0.2
New CompleteSet / MarketSeedTx code must not import legacy prediction_market.rs.

FR-13.0.3
No f64 in CompleteSet / MarketSeedTx / market accounting path.

FR-13.0.4
Legacy CPMM must be clearly labeled:
  legacy
  not used by RSP-M
  not constitutional
  not production market path.
```

### Ship gates

```text
SG-13.0.1
legacy_cpm_api_not_imported_by_complete_set passes.

SG-13.0.2
no_f64_in_complete_set_or_market_seed passes.

SG-13.0.3
prediction_market_legacy_quarantined passes.

SG-13.0.4
OBS_TB_12_LEGACY_CPMM_QUARANTINE either closed or explicitly carried as non-importable legacy.
```

### Halting triggers

```text
HALT if new TB-13 code imports legacy prediction_market.rs.

HALT if f64 appears in new CompleteSet / MarketSeed code.

HALT if any AMM / CPMM router function is introduced in TB-13.
```

---

## 4.3 新增对象

```rust
pub struct EventId(...);


exec
/bin/bash -lc "sed -n '260,620p' handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

pub enum OutcomeSide {
    Yes,
    No,
}

pub struct ShareAmount {
    pub units: i128,
}

pub struct ConditionalCollateralIndex(
    pub BTreeMap<EventId, MicroCoin>
);

pub struct ConditionalShareBalances(
    pub BTreeMap<(AgentId, EventId, OutcomeSide), ShareAmount>
);

pub struct CompleteSetMintTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub amount: MicroCoin,
    pub signature: AgentSignature,
}

pub struct CompleteSetRedeemTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub owner: AgentId,
    pub outcome: OutcomeSide,
    pub share_amount: ShareAmount,
    pub signature_or_system_resolution_ref: ResolutionRef,
}

pub struct MarketSeedTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub signature: AgentSignature,
}
```

---

## 4.4 Functional requirements

```text
FR-13.1
CompleteSetMintTx debits balances_t by amount.

FR-13.2
CompleteSetMintTx credits conditional_collateral_t by amount.

FR-13.3
CompleteSetMintTx issues equal YES_E and NO_E shares.

FR-13.4
CompleteSetRedeemTx is impossible before system-resolved outcome.

FR-13.5
CompleteSetRedeemTx after YES outcome pays YES shares and not NO shares.

FR-13.6
MarketSeedTx uses explicit provider funds.

FR-13.7
MarketSeedTx may prepare collateralized YES/NO inventory for future market layers, but cannot quote, trade, or price.
```

---

## 4.5 Constitutional requirements

```text
CR-13.1
No ghost liquidity.

CR-13.2
No automatic YES/NO injection.

CR-13.3
YES/NO shares are claims, not Coin.

CR-13.4
Locked collateral is Coin holding; shares are not.

CR-13.5
on_init remains the only base Coin mint.

CR-13.6
Price / share state cannot override predicates or challenge outcome.
```

---

## 4.6 Ship gates

```text
SG-13.1
Mint 1 Coin -> 1 YES + 1 NO, total Coin conserved.

SG-13.2
YES/NO shares are not counted in total Coin supply.

SG-13.3
MarketSeedTx fails if provider lacks balance.

SG-13.4
MarketSeedTx cannot create liquidity without collateral.

SG-13.5
Redeem unavailable before outcome resolution.

SG-13.6
Redeem after YES outcome pays YES, not NO.

SG-13.7
No f64 in new CompleteSet / MarketSeed path.

SG-13.8
No import/use of legacy CPMM in TB-13 modules.
```

---

## 4.7 Forbidden

```text
No automatic per-node 100 YES + 100 NO.
No Treasury magic seed unless Treasury balance is debited.
No DPMM / pro-rata payout.
No AMM.
No CLOB / orderbook.
No MarketOrderTx.
No MarketTradeTx.
No price oracle.
No NodeMarketEntry canonical state.
No f64.
```

---

## 4.8 Loop-mode instruction

AI coder may run TB-13 atoms autonomously until pre-ship audit.

Must halt immediately if:

```text
total_supply_micro changes incorrectly;
shares are counted as Coin;
MarketSeedTx succeeds without balance debit;
legacy CPMM import appears;
f64 appears in new market modules;
any price / AMM / trade logic appears.
```

Risk class:

```text
Class 3
```

Reason:

```text
CompleteSet and MarketSeedTx are money/collateral surfaces.
```

---

# 5. TB-14 — PriceIndex v0 + Boltzmann Masking

## 5.1 目标

将 TB-12 的 `NodePosition` 和 TB-13 的 share/collateral state 转化为价格信号，并用于 read-view / scheduler masking。

核心原则：

```text
Price is signal, not truth.
```

这与宪法中的信号管理一致。宪法把顶层白盒的工作定义为量化、广播、屏蔽，并把统计信号用于衡量相对有效性；但布尔谓词仍然确立绝对边界。

---

## 5.2 新增对象

```rust
pub struct RationalPrice {
    pub numerator: u128,
    pub denominator: u128,
}

pub struct NodeMarketEntry {
    pub node_id: TxId,
    pub task_id: TaskId,
    pub event_id: EventId,
    pub long_interest: MicroCoin,
    pub short_interest: MicroCoin,
    pub yes_share_depth: ShareAmount,
    pub no_share_depth: ShareAmount,
    pub price_yes: Option<RationalPrice>,
    pub price_no: Option<RationalPrice>,
    pub liquidity_depth: MicroCoin,
}

pub struct BoltzmannMaskPolicy {
    pub beta_num: i64,
    pub beta_den: i64,
    pub min_liquidity: MicroCoin,
    pub price_margin: RationalPrice,
    pub epsilon_exploration_num: u64,
    pub epsilon_exploration_den: u64,
}
```

---

## 5.3 Functional requirements

```text
FR-14.1
Compute price_yes = long / (long + short) when exposure liquidity exists.

FR-14.2
Compute price_no = short / (long + short).

FR-14.3
If no long/short liquidity exists, price=None.

FR-14.4
Expose PriceIndex as read-only broadcast/statistical signal.

FR-14.5
Boltzmann scheduler may mask parent only if child price sufficiently dominates.

FR-14.6
Masked parent remains in ChainTape and can be recovered.
```

---

## 5.4 Constitutional requirements

```text
CR-14.1
Price cannot override predicates.

CR-14.2
Price cannot make failed proposal accepted.

CR-14.3
Masked means read-view/scheduler mask only, not deletion.

CR-14.4
Low-liquidity price cannot mask parent.

CR-14.5
Unresolved-challenged child cannot mask parent.

CR-14.6
Goodhart-sensitive private predicates remain hidden.
```

---

## 5.5 Ship gates

```text
SG-14.1
PriceIndex computes expected YES/NO probabilities.

SG-14.2
No-liquidity node has price=None.

SG-14.3
Parent not deleted from ChainTape after masking.

SG-14.4
Predicate failure still dominates high price.

SG-14.5
Boltzmann selection includes epsilon exploration.

SG-14.6
Dashboard shows price as signal, not outcome.

SG-14.7
Unresolved challenge blocks masking.

SG-14.8
Low-liquidity manipulation cannot mask parent.
```

---

## 5.6 Forbidden

```text
No market trading.
No price-based settlement.
No parent deletion.
No Goodhart leak of private predicates.
No masking unresolved-challenge nodes.
No f64.
No AMM.
No DPMM.
No price-as-oracle language.
```

---

## 5.7 Loop-mode instruction

AI coder can run implementation autonomously until pre-ship audit.

Halt if:

```text
price affects predicate result;
price changes L4/L4.E decision;
parent node deleted/removed from ChainTape;
f64 introduced;
price computed for zero-liquidity node as non-None;
unresolved challenge is masked as safe.
```

Risk class:

```text
Class 2
```

If Boltzmann masking changes actual Agent read view in production, promote to:

```text
Class 3
```

---

# 6. TB-15 — Lamarckian Autopsy + Markov EvidenceCapsule

## 6.1 目标

把失败、爆仓、亏损、反复错误转化为私有学习与 Markov capsule，而不是全局 raw-log 污染。

这对齐无损宪法的 Flowchart 3：`logs archive as ground truth` 和 `constitution as ground truth` 共同构成元架构输入，但 Markov 规则要求默认只读最新 capsule，而不是全历史日志。

---

## 6.2 新增对象

```rust

exec
/bin/bash -lc "sed -n '1,260p' handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-13 Recursive Self-Audit — CompleteSet + MarketSeedTx

**Audit type**: Recursive self-audit (Class 3 envelope; per architect 2026-05-03 post-TB-12 ruling Part A §4.8 + charter §3 Atom 6).
**Date**: 2026-05-03 evening.
**Scope**: TB-13 Atoms 0 + 0.5 + 1 + 2 + 3 + 5 SHIPPED to local main; Atom 4 (dashboard §14) DEFERRED to TB-14 (not in architect Part A spec); this document is Atom 6(a). External Codex + Gemini audits pending — Atom 6(b) + 6(c).
**Charter**: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
**Architect ruling lossless**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`.
**Local commits**:
- `32aab27` Atom 0 + 0.5 — Charter + legacy f64 CPMM forward-fence
- `70303af` Atom 1 — typed_tx schemas (+8 unit tests)
- `1806432` Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests

**Executive verdict**: **PASS** with no halting triggers fired.
External Codex + Gemini audits MUST follow before SHIP per architect §11 master instruction (`feedback_dual_audit` Class 3 = full hybrid dual; conservative-verdict-wins on disagreement).

---

## §1 Clause 1 — Constitutional preservation

| Article                                  | TB-13 invariant                                                                                                | Verification                                                                              |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Art. 0.2 Tape Canonical                  | 3 NEW typed-tx variants (CompleteSetMint / CompleteSetRedeem / MarketSeed) canonical-encoded. 3 NEW state-root domain prefixes (`COMPLETE_SET_MINT_DOMAIN_V1` / `COMPLETE_SET_REDEEM_DOMAIN_V1` / `MARKET_SEED_DOMAIN_V1`). Replay-deterministic from typed-tx fields. | `tb_13_complete_set_mint_round_trips_canonical` + `tb_13_complete_set_redeem_round_trips_canonical` + `tb_13_market_seed_round_trips_canonical` + `tb_13_signing_payloads_deterministic_digest` |
| Art. I.1 5-step compile loop closure     | Conditional collateral & share accounting are part of the proposal-pricing substrate, but DO NOT affect predicate outcome (CR-13.6: "Price / share state cannot override predicates or challenge outcome"). | TB-13 dispatch arms operate purely on `economic_state_t.{balances_t, conditional_collateral_t, conditional_share_balances_t}`; predicate-evaluation paths (TB-3 Work / TB-4 Verify+Challenge / TB-5 ChallengeResolve / TB-8 FinalizeReward) UNCHANGED. |
| Art. II.2.1 entropy / quantize-broadcast-shield | Conditional share balances are public economic record (no shielding). Architect Part A §4 makes no shielding requirement; failure capsules (TB-11 EvidenceCapsule) remain the privacy-shielded surface. | No new shielded surface introduced. |
| Art. III.4 no fake accepted              | CompleteSetRedeemTx requires sequencer-side validation of `task_markets_t[event_id.0].state ∈ {Finalized, Bankrupt}` AND `resolution_ref.claimed_outcome` matches the state. Pre-resolution rejected with `RedeemBeforeResolution`; wrong-outcome-for-state rejected with `InvalidResolutionRef`. Owner cannot self-resolve. | `sg_13_5_redeem_unavailable_before_outcome_resolution` (Open + Expired states both reject); `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (mismatch outcome rejected with InvalidResolutionRef on Finalized event; symmetric Bankrupt check) |
| Art. V.1.3 Anti-Oreo                     | All 3 TB-13 typed-tx are AGENT-SIGNED (CompleteSetMint / CompleteSetRedeem / MarketSeed). NO new system_tx variant introduced. Provider funds are EXPLICIT in MarketSeedTx (NO automatic seed; CR-13.1 + CR-13.2 forbid ghost / automatic liquidity). | `submit_agent_tx` ingress agent-fall-through arm extended for 3 new variants (verified via reading `src/state/sequencer.rs:1996..2002`). NO `emit_system_tx` arm added for TB-13. |

**Verdict 1: PASS**. Zero constitutional violations introduced.

---

## §2 Clause 2 — Replay-deterministic

| Property                                  | TB-13 verification                                                                                                                |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Q-projection determinism                  | Conditional collateral is a deterministic projection of accepted CompleteSetMint / MarketSeed tx fields (`amount.micro_units()` / `collateral_amount.micro_units()`). Conditional share balances are deterministic projections (Yes + No each = `amount.micro_units() as u128`). Redeem deterministically debits winning side + collateral by `share_amount.units`. No environmental input. |
| Cross-instance replay equality            | TB-3..TB-12 replay invariants unchanged. TB-13 dispatch arms add to `q_next.economic_state_t.{balances_t, conditional_collateral_t, conditional_share_balances_t}` only — each mutation is field-by-field deterministic. |
| State-root advance                        | 3 new domain helpers `complete_set_mint_accept_state_root` / `complete_set_redeem_accept_state_root` / `market_seed_accept_state_root` mirror the TB-3 / TB-11 / TB-12 SHA-256-of-(domain ∥ prev_root ∥ canonical_encoded(tx)) pattern. Domain prefixes are unique per-arm. |

**Verdict 2: PASS**.

---

## §3 Clause 3 — Conservation (CTF)

| Conservation invariant                    | TB-13 enforcement                                                                                                                                                                                |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **6-holding CTF (post-TB-13)**            | Σ holdings = balances_t + escrows_t + stakes_t + challenge_cases_t + conditional_collateral_t. Atom 3 extends the TB-7R 5-holding sum to 6 by adding `conditional_collateral_t` per architect CR-13.4 ("Locked collateral is Coin holding"). conditional_share_balances_t INTENTIONALLY OMITTED per CR-13.3 + SG-13.2. |
| `assert_total_ctf_conserved` post-mint    | `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` + `sg_13_2_yes_no_shares_not_in_total_coin_supply` + `halt_total_supply_micro_unchanged_across_mint_redeem`: balance debit = collateral credit, total preserved bit-for-bit. PASS.  |
| `assert_total_ctf_conserved` post-redeem  | Redeem: collateral debit + balance credit (1:1). `halt_total_supply_micro_unchanged_across_mint_redeem` exercises mint+redeem and asserts `assert_total_ctf_conserved(pre, post, &[]).is_ok()`. PASS. |
| `assert_total_ctf_conserved` post-seed    | `halt_complete_set_balanced_post_seed`: provider balance debit = collateral credit. CTF preserved. PASS. |
| `assert_no_post_init_mint`                | All 3 TB-13 variants in exhaustive match arm — none create money (mint = balance↔collateral migration; redeem = collateral↔balance migration; seed = balance↔collateral migration). PASS. |
| `assert_complete_set_balanced` (NEW)      | For every event in conditional_collateral_t: `min(Σ_yes, Σ_no) == collateral`. Pre-resolution (mint+seed): Σ_yes == Σ_no == collateral (both equal trivially equivalent). Post-redeem: winning side equals collateral; losing side may be larger (stranded zero-value claims). Verified: `sg_13_1` post-mint balanced; `halt_complete_set_balanced_post_seed`; `halt_total_supply_micro_unchanged_across_mint_redeem` post-redeem balanced. |

**MIN-semantics rationale**: a strict `Σ_yes == collateral AND Σ_no == collateral` requirement does NOT hold post-redemption (the losing side has stranded shares above the now-decremented collateral). MIN form correctly captures: every Coin in collateral can be redeemed by the winning side, and no winning-side share is unbacked. This was discovered mid-test by my own halting-trigger guard `halt_total_supply_micro_unchanged_across_mint_redeem`; the strict-equality form initially failed and was replaced with MIN form. The bug-find demonstrates the recursive self-audit harness working as intended.

**Verdict 3: PASS**.

---

## §4 Clause 4 — Resolution gating (TB-13-unique)

| Property                                  | TB-13 verification                                                                                                                                                                                |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 4.1 Redeem requires resolved task_market state | `task_markets_t[event_id.0].state ∈ {Open, Expired}` → `RedeemBeforeResolution`. Verified: `sg_13_5_redeem_unavailable_before_outcome_resolution` covers both Open and Expired. |
| 4.2 Resolution-outcome match enforced      | Finalized state → only outcome=Yes accepted; Bankrupt state → only outcome=No accepted; mismatch → `InvalidResolutionRef`. Verified: `sg_13_6` includes 2 mismatch checks (Finalized+No rejected; Bankrupt+Yes rejected). |
| 4.3 ResolutionRef inner-consistency        | Pre-state-lookup gate: `redeem.outcome != redeem.resolution_ref.claimed_outcome` → `InvalidResolutionRef`. Catches malformed wire payloads where the redeem's outcome field disagrees with the resolution_ref's claimed_outcome (defense-in-depth before task_markets_t lookup). |
| 4.4 Owner share-balance gate               | `RedeemMoreThanOwned` if `conditional_share_balances_t[owner][event_id].{yes|no}` < `share_amount.units`. Verified: `halt_redeem_more_than_owned_rejected`. |
| 4.5 Collateral coverage gate               | `InsufficientCollateral` if `conditional_collateral_t[event_id]` < `share_amount.units` (defensive; should never fire if `assert_complete_set_balanced` holds). |

**Verdict 4: PASS**. All 5 admission gates exercised by integration tests.

---

## §5 Clause 5 — Forward-fence + label discipline (Atom 0.5)

| Property                                  | TB-13 verification                                                                                                                                                                                |
| ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 5.1 Module-header LEGACY label            | `src/prediction_market.rs` carries `//! # LEGACY ...` block declaring: not constitutional, not RSP-M, not production market path; lists each constitutional non-compliance (f64 / automatic liquidity / trading semantics); names migration path (TB-13/TB-14). Verified: `prediction_market_legacy_quarantined` (SG-13.0.3). |
| 5.2 Field-level LEGACY labels             | `src/kernel.rs` carries field-level `LEGACY` doc-comments on `markets`, `bounty_market`, `bounty_lp_seed`. Verified: `prediction_market_legacy_quarantined` defense-in-depth check. |
| 5.3 Forward-fence ship-gate test          | `tests/tb_13_legacy_cpmm_forward_fence.rs` — 3 EXACT-named tests `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1) + `no_f64_in_complete_set_or_market_seed` (SG-13.0.2) + `prediction_market_legacy_quarantined` (SG-13.0.3). Span detector uses authoring-marker rule (TRACE_MATRIX TB-13 / TB-13 line-prefix) to avoid false positives from TB-12 doc-comments referencing TB-13 as future work. |
| 5.4 OBS carry-forward                     | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` updated with TB-13 Atom 0.5 status section; SG-13.0.4 satisfied as "carry forward to TB-14 SHIP prerequisite". |
| 5.5 NO retroactive deletion               | `src/prediction_market.rs` and `src/kernel.rs` market scaffolding NOT removed (production wiring at `src/bus.rs:206/327/359/480-515` + `experiments/minif2f_v4/src/bin/evaluator.rs:1323` + 10+ test files would break). Out of scope per `feedback_no_retroactive_evidence_rewrite` and architect §4.2 halting-trigger semantics (which target NEW TB-13 code, not existing scaffolding). |

**Verdict 5: PASS**. Forward-fence binding established; legacy CPMM clearly labeled as non-importable; TB-14 SHIP prerequisite preserved.

---

## §6 Architect halting triggers (charter §3 Atom 0; architect Part A §4.8) — NOT triggered

| Halting trigger                                                  | Result                                                                                                       |
| ---------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| total_supply_micro mutation incorrect (CTF conservation failure) | NOT triggered. `halt_total_supply_micro_unchanged_across_mint_redeem` + `halt_shares_not_counted_as_coin` PASS. |
| Conditional shares counted as Coin (architect SG-13.2 violation)  | NOT triggered. `total_supply_micro` excludes `conditional_share_balances_t` per CR-13.3 implementation. `sg_13_2_yes_no_shares_not_in_total_coin_supply` PASS. |
| MarketSeedTx succeeds without balance debit (architect SG-13.3)   | NOT triggered. `halt_market_seed_zero_balance_provider_rejected` + `sg_13_3_market_seed_fails_if_provider_lacks_balance` PASS. |
| Legacy `prediction_market::` import in NEW TB-13 module           | NOT triggered. `legacy_cpm_api_not_imported_by_complete_set` PASS. |
| f64 appears in NEW market modules                                 | NOT triggered. `no_f64_in_complete_set_or_market_seed` PASS. |
| Any AMM / CPMM router / price / trade logic introduced            | NOT triggered. Forward-fence forbidden-token grep on TB-13-marked spans excludes `MarketOrderTx`, `MarketTradeTx`, `AMM`, `CPMM`, `DPMM`, `orderbook`, `price_yes`, `price_no`, `PriceIndex`. PASS. (Actual catch-and-fix mid-development: `PriceIndex` reference in MarketSeedTx + transition_ledger TxKind::MarketSeed doc-comments was correctly flagged and removed.) |
| Codex / Gemini VETO                                               | DEFERRED to Atom 6(b) + 6(c). Self-audit verdict here is PASS; external audits next. |

---

## §7 Ship gates (architect SG-13.0..8 + carry-forward G1..G11)

**Architect SG-13.0..8** (charter §6):

| Gate                                                              | Status              | Evidence                                                                                                                              |
| ----------------------------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| SG-13.0.1 legacy_cpm_api_not_imported_by_complete_set             | ✓ pass (exact)      | `legacy_cpm_api_not_imported_by_complete_set` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                            |
| SG-13.0.2 no_f64_in_complete_set_or_market_seed                   | ✓ pass (exact)      | `no_f64_in_complete_set_or_market_seed` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                                  |
| SG-13.0.3 prediction_market_legacy_quarantined                    | ✓ pass (exact)      | `prediction_market_legacy_quarantined` (`tests/tb_13_legacy_cpmm_forward_fence.rs`)                                                   |
| SG-13.0.4 OBS_TB_12_LEGACY_CPMM_QUARANTINE carried as non-importable legacy | ✓ pass (carry) | `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` "TB-13 Atom 0.5 update" section                                  |
| SG-13.1   mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved | ✓ pass (exact) | `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` (`tests/tb_13_complete_set.rs`)                            |
| SG-13.2   yes_no_shares_not_in_total_coin_supply                  | ✓ pass (exact)      | `sg_13_2_yes_no_shares_not_in_total_coin_supply`                                                                                       |
| SG-13.3   market_seed_fails_if_provider_lacks_balance             | ✓ pass (exact)      | `sg_13_3_market_seed_fails_if_provider_lacks_balance`                                                                                   |
| SG-13.4   market_seed_cannot_create_liquidity_without_collateral  | ✓ pass (exact)      | `sg_13_4_market_seed_cannot_create_liquidity_without_collateral`                                                                       |
| SG-13.5   redeem_unavailable_before_outcome_resolution            | ✓ pass (exact)      | `sg_13_5_redeem_unavailable_before_outcome_resolution` (covers Open + Expired states)                                                  |
| SG-13.6   redeem_after_yes_outcome_pays_yes_not_no                | ✓ pass (exact)      | `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (also covers symmetric Bankrupt → No path + 2 mismatch InvalidResolutionRef checks) |
| SG-13.7   no_f64_in_new_complete_set_or_market_seed_path          | ✓ pass (delegation) | `sg_13_7_no_f64_in_new_complete_set_or_market_seed_path` delegates to `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed` (SG-13.0.2 fence) |
| SG-13.8   no_import_or_use_of_legacy_cpmm_in_tb13_modules         | ✓ pass (delegation) | `sg_13_8_no_import_or_use_of_legacy_cpmm_in_tb13_modules` delegates to `legacy_cpm_api_not_imported_by_complete_set` (SG-13.0.1 fence) |

**12/12 architect SG-13.x ship gates PASS**.

**Engineering carry-forward G1..G11** (TB-9 / TB-10 / TB-11 / TB-12 precedent):

| Gate                                                              | Status     | Evidence                                                                                                |
| ----------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------------------- |
| G1 cargo check                                                    | ✓ pass     | Clean run; only legacy warnings carried over from TB-12.                                                |
| G2 cargo test --workspace                                         | ✓ pass     | `783/0/150` (TB-12 baseline 759 + 3 fence + 8 unit + 13 integration). Workspace canonical per `feedback_workspace_test_canonical`. |
| G3 lean_market 7 subcommands                                      | ✓ pass     | TB-12 baseline 7 subcommands unchanged (TB-13 does not add CLI; Atom 4 dashboard deferred).             |
| G4 evaluator MAX_TX exhaust → EvidenceCapsule                     | ✓ pass     | TB-11 carry-forward unchanged.                                                                          |
| G5 audit_dashboard §13 still renders                              | ✓ pass     | TB-12 §13 unchanged. §14 dashboard rendering deferred (architect Part A spec made no dashboard requirement). |
| G6 verify_chaintape green                                         | ✓ pass     | replay-determinism verified at QState level (cross-instance replay equality); deterministic on TB-13 typed-tx via canonical_encode round-trip tests. |
| G7 typed_tx variants: 3 NEW additive only                         | ✓ pass     | CompleteSetMint / CompleteSetRedeem / MarketSeed; existing variants unchanged.                          |
| G8 dispatch arms: 3 NEW additive                                  | ✓ pass     | CompleteSetMint / CompleteSetRedeem / MarketSeed accept arms in `src/state/sequencer.rs`. Existing arms unchanged. |
| G9 TransitionError variants: 5 NEW additive                       | ✓ pass     | InsufficientBalanceForMint / RedeemBeforeResolution / RedeemMoreThanOwned / InsufficientCollateral / InvalidResolutionRef. |
| G10 EconomicState field count == 13                               | ✓ pass     | Was 11 post-TB-12; +conditional_collateral_t +conditional_share_balances_t. Asserted by `economic_state_has_thirteen_sub_fields` (q_state unit test) + 3 cross-test assertions in `tests/economic_state_reconstruct.rs` / `tests/q_state_reconstruct.rs` / `tests/six_axioms_alignment.rs`. |
| G11 Conservation: 6-holding sum preserved + complete-set balanced | ✓ pass     | `total_supply_micro` extended with `conditional_collateral_t`; `assert_complete_set_balanced` enforced post-mint / post-seed / post-redeem in 3 halting-trigger tests. |

**11/11 G ship gates PASS**.

---

## §8 Forbidden tokens grep summary (architect Part A §4.7 + halting triggers)

Verified by `tests/tb_13_legacy_cpmm_forward_fence.rs` forbidden-token list applied to TB-13-marked spans across `src/state/{typed_tx,q_state,sequencer}.rs` + `src/economy/monetary_invariant.rs` + `src/bin/audit_dashboard.rs`:

```text
Banned tokens (would HALT if found in TB-13 span):
  prediction_market::    NOT FOUND
  BinaryMarket           NOT FOUND
  .buy_yes(              NOT FOUND
  .buy_no(               NOT FOUND
  open_bounty_market     NOT FOUND
  bounty_market          NOT FOUND
  bounty_lp_seed         NOT FOUND
  bounty_yes_price       NOT FOUND
  resolve_bounty         NOT FOUND
  market_ticker(         NOT FOUND
  market_ticker_full(    NOT FOUND
  MarketOrderTx          NOT FOUND
  MarketTradeTx          NOT FOUND
  MarketBuyTx            NOT FOUND
  MarketSellTx           NOT FOUND
  AMM                    NOT FOUND
  CPMM                   NOT FOUND
  DPMM                   NOT FOUND
  orderbook              NOT FOUND
  price_yes              NOT FOUND
  price_no               NOT FOUND
  PriceIndex             NOT FOUND
  yes_price              NOT FOUND
  no_price               NOT FOUND
  RationalPrice          NOT FOUND
  f64 (in money path)    NOT FOUND
```

22/22 forbidden tokens absent from TB-13-marked spans.

Mid-development catch: 2 `PriceIndex` doc-comment references in MarketSeedTx + TxKind::MarketSeed were correctly flagged by the fence and replaced with implementation-detail-neutral language. The fence working as intended.

---

## §9 Audit-mode declaration (Class 3 dual; pending external Codex + Gemini)

Per `feedback_dual_audit` + architect Part A §4.8 (Class 3 = money / collateral surface):

- **Atom 6(a) self-audit (this document)**: PASS verdict. Clauses 1–5 all green; halting triggers 1–7 NOT triggered; 12/12 SG-13.x + 11/11 G ship gates pass.
- **Atom 6(b) Codex impl-paranoid audit**: PENDING. Specific questions to be put to Codex (per charter §3 Atom 6.b):
  1. Does CompleteSetMint create or destroy money? (must be balance↔collateral migration only)
  2. Can Redeem fire without a system-emitted resolution? (must be sequencer-rejected)
  3. Can Redeem with `outcome=Yes` and a TaskBankruptcy resolution_ref bypass the outcome check?
  4. Does the 6-holding sum hold across all TB-13 typed_tx?
  5. Does `assert_complete_set_balanced` hold after every transition?
  6. Can MarketSeedTx create liquidity without provider balance? (must be rejected)
  7. Are conditional shares anywhere counted as Coin? (must be excluded)
  8. Could a malformed ShareAmount underflow? (u128 type guarantee + RedeemMoreThanOwned gate)
  9. Forward-fence: any new TB-13 module file references legacy `prediction_market`?
- **Atom 6(c) Gemini architectural strategic audit**: PENDING. Same 9 questions plus:
  10. Does CompleteSet schema extend cleanly to TB-14 PriceIndex (long/short interest derivable from `conditional_share_balances_t` aggregates)?
  11. Does the EventId == TaskId 1:1 simplification hold up under TB-14+ multi-event-per-task scenarios?
  12. Is the `ResolutionRef` model robust to multi-resolver scenarios in TB-15+?
  13. Is the MIN-semantics `assert_complete_set_balanced` invariant the right form (vs. strict equality), particularly for adversarial patterns: e.g., re-mint after partial redeem, or repeated redeem-and-remint cycles?

Conservative-verdict-wins on Codex ↔ Gemini disagreement (per `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

---

## §10 Open follow-ups (carry-forward, NOT ship blockers)

| Item                                                              | Reason / status                                                                                          |
| ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| Atom 4 dashboard §14 (conditional collateral + share render)      | DEFERRED: not in architect Part A spec (no FR/CR/SG references it). TB-14 PriceIndex will need the same dashboard surface; consolidate then. |
| Legacy `src/prediction_market.rs` + `src/kernel.rs` CPMM scaffolding hard-removal | TB-14 SHIP prerequisite per `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`. Forward-fence in TB-13 prevents new code from importing; full refactor before TB-14 SHIP.                |
| `assert_complete_set_balanced` MIN-semantics formalization        | The MIN form was discovered mid-test as the correct invariant (vs. naive Σ_yes==Σ_no==coll). External audit may want stronger formalization (e.g., adversarial proof that no adversary can break MIN with a sequence of mints + partial redeems). |
| EventId == TaskId 1:1 mapping                                     | TB-13 simplification: each task is one event. TB-14+ may decouple if per-node events are needed. Schema is forward-compat (EventId is a newtype around TaskId; future variants can wrap a different inner type).                                                                |

---

## §11 Concluding verdict

TB-13 SHIPPED Atoms 0 + 0.5 + 1 + 2 + 3 + 5 to local main with PASS verdict on:
- 5 constitutional clauses (Tape Canonical / 5-step compile loop / Anti-Oreo / no fake accepted / replay-deterministic + conservation + resolution gating + forward-fence + label discipline)
- 12/12 architect SG-13.0..8 ship gates (EXACT-named, traceability-contract discipline)
- 11/11 engineering carry-forward G1..G11 ship gates
- 7/7 architect halting triggers NOT triggered

`cargo test --workspace = 783/0/150` (TB-12 baseline 759 + 3 fence + 8 unit + 13 integration = 783; failed=0; ignored=150 unchanged).

External Codex + Gemini audits pending per architect Part A §4.8 (Class 3 dual; conservative-verdict-wins on disagreement). User-checkpoint per architect §11 master instruction "Stop for user review at ship gate" reached: AI coder STOPS for user review of this self-audit + decision on whether to invoke Codex + Gemini external audits before Atom 7 SHIP.

End of recursive self-audit.

exec
/bin/bash -lc "sed -n '1,240p' handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
# TB-13 Real-LLM Smoke Evidence — 2026-05-03

**Date**: 2026-05-03 evening (post-Atom 6(a) recursive self-audit; pre-external-audit).
**TB**: TB-13 (CompleteSet + MarketSeedTx).
**Source**: `target/debug/evaluator` HEAD = `17d4a3b` (Atom 6(a) ship), branch `main`.
**Model**: `deepseek-chat` via local LLM proxy at `localhost:8080/v1/chat/completions`.
**Lean**: 4.x (`/home/zephryj/.elan/bin/lean` runtime; problem from `turingosv3` minif2f Test corpus).
**Charter**: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
**Audit**: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`.

---

## §0 Headline

**TB-13 schema works end-to-end under real-LLM workload**.

Single-problem regression smoke (mathd_algebra_171, MAX_TX=10, n1):
- Outcome: **UNSOLVED — hit_max_tx** (10/10 proposals failed predicates; expected for short MAX_TX).
- Replay-determinism: **7/7 GREEN**.
- EconomicState: **13 sub-fields confirmed** (TB-13 +`conditional_collateral_t` + `conditional_share_balances_t` persist correctly).
- TB-13 additive changes: **NO regression** vs existing TB-3..TB-12 capability loop.

## §1 What this smoke validates

The smoke is a **regression check**, not a TB-13 capability demonstration. The
LLM-driven path (Work / Verify / Challenge) does NOT submit any of the 3 new
TB-13 typed-tx variants (CompleteSetMint / CompleteSetRedeem / MarketSeed) —
those are user-driven economic-action tx, not solver-driven. The ground
truth this smoke produces:

1. The 13-sub-field `EconomicState` shape (post-TB-13) **serializes /
   deserializes correctly** under live workload.
2. The `verify_chaintape` replay reconstruction **succeeds bit-equal** with
   the new schema (`economic_state_reconstructed: true`).
3. The `audit_dashboard` §13 (TB-12 NodePosition) rendering **still works**
   alongside the new fields (no §14 — Atom 4 deferred).
4. The TB-11 Epistemic Exhaust §12 dashboard + L4 anchor **still works**
   (RunExhaustedTx + EvidenceCapsule emitted on MAX_TX exhaust).
5. The 6-holding `total_supply_micro` invariant + `assert_no_post_init_mint`
   exhaustive match (now covering 14 typed-tx variants including the 3
   TB-13) hold under live transitions.

The 13-test integration suite (`tests/tb_13_complete_set.rs`) covers the
TB-13-specific flows (mint / redeem / seed); this smoke covers the
**regression surface** that those targeted tests cannot reach.

## §2 Replay report (single run)

```json
{
  "l4_entries": 3,
  "l4e_entries": 2,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "agent_signatures_verified": true,
  "proposal_telemetry_cas_retrievable": true,
  "run_id": "tb13-smoke",
  "epoch": 1,
  "detail": {
    "final_state_root_hex": "1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9",
    "final_ledger_root_hex": "93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb",
    "head_commit_oid_hex": "38f1b3957834052aac42169598f92016d756c331",
    "l4e_last_hash_hex": "79325795bf2ebc78a9330c06c173bb0c502ee283fbfa5b46f569551314e9e23a",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": true
  }
}
```

L4 = 3 entries (TaskOpen + EscrowLock + TerminalSummaryTx-on-MaxTxExhausted via
TB-11 Atom 0.5(a) carry-forward); L4.E = 2 (rejected attempts; expected).

## §3 EconomicState 13-sub-field round-trip

Verified by direct introspection of the on-disk
`runtime_repo/initial_q_state.json` after replay:

```text
EconomicState sub-fields: 13
Sub-field names: [
  'balances_t',
  'challenge_cases_t',
  'claims_t',
  'conditional_collateral_t',          <-- TB-13 Atom 2 NEW
  'conditional_share_balances_t',      <-- TB-13 Atom 2 NEW
  'escrows_t',
  'node_positions_t',
  'price_index_t',
  'reputations_t',
  'royalty_graph_t',
  'runs_t',
  'stakes_t',
  'task_markets_t',
]
```

Both new fields default to empty maps under live workload (no TB-13
typed-tx submitted by the LLM-driven solver path), but round-trip cleanly
through `serde_json` and `canonical_encode` — the absence of regression
on the 13-sub-field shape is the load-bearing claim.

## §4 Dashboard render (§12 + §13)

`dashboard.txt` excerpt (post-replay):

```text
§12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2; 2026-05-02)
------------------------------------------------------------------------------
  Exhausted runs (RunExhaustedTx ≡ TerminalSummaryTx):
    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)
    n1_mathd_alge… | task-n1_mathd_alg… | MaxTxExhausted  |       10 | d2b329ee554da3e2dea1d46ecca1bf1…

§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)
```

§14 (TB-13 conditional shares) — **NOT RENDERED**. Atom 4 dashboard work
deferred to TB-14 PriceIndex (architect Part A spec made no dashboard
requirement for TB-13). State observability available via direct QState
introspection (`initial_q_state.json` shown in §3).

## §5 Headline outcome table

| Step  | Config                                          | Outcome   | TB-13 schema integrity |
|-------|-------------------------------------------------|-----------|------------------------|
| Single | n1 × `mathd_algebra_171` × MAX_TX=10           | UNSOLVED (hit_max_tx) | ✓ 13 sub-fields persist |

UNSOLVED is expected for MAX_TX=10 on this problem (TB-8 historical: same
problem solved at MAX_TX=10 single-run; deepseek-chat is drift-prone per
`project_deepseek_drift_2026-04-24`). The smoke's load-bearing claim is
schema integrity, not solve rate.

## §6 Reproduction

```bash
SMOKE_DIR=/tmp/tb13_smoke_repro
mkdir -p "$SMOKE_DIR"/{runtime_repo,cas}

cd experiments/minif2f_v4
TURINGOS_CHAINTAPE_PATH="$SMOKE_DIR/runtime_repo" \
TURINGOS_CAS_PATH="$SMOKE_DIR/cas" \
TURINGOS_CHAINTAPE_PRESEED=1 \
TURINGOS_RUN_ID=tb13-smoke \
LLM_PROXY_URL="http://localhost:8080/v1/chat/completions" \
ACTIVE_MODEL=deepseek-chat \
CONDITION=n1 \
MAX_TRANSACTIONS=10 \
../../target/debug/evaluator mathd_algebra_171.lean

../../target/debug/audit_dashboard \
  --repo "$SMOKE_DIR/runtime_repo" \
  --cas "$SMOKE_DIR/cas" \
  > "$SMOKE_DIR/dashboard.txt"

../../target/debug/verify_chaintape \
  --repo "$SMOKE_DIR/runtime_repo" \
  --cas "$SMOKE_DIR/cas" \
  --out "$SMOKE_DIR/replay_report.json"

# Expected: 7/7 indicators GREEN; economic_state_reconstructed: true.
python3 -c "
import json
q = json.load(open('$SMOKE_DIR/runtime_repo/initial_q_state.json'))
es = q['economic_state_t']
assert len(es) == 13, f'expected 13 sub-fields, got {len(es)}'
assert 'conditional_collateral_t' in es
assert 'conditional_share_balances_t' in es
print('TB-13 schema OK: 13 sub-fields present')
"
```

## §7 What this smoke does NOT validate

- TB-13 mint / redeem / seed dispatch arms under load — those are exercised by
  `tests/tb_13_complete_set.rs` integration tests (13 tests, all PASS).
- Multi-problem variety / aggregate solve rate under TB-13 — out of scope
  (regression smoke, not capability demo).
- Cross-run identity / durable keystore reattachment — TB-9 territory.
- Real Polymarket market interactions — those are TB-14+ scope.

## §8 Cross-references

- TB-13 charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- TB-13 recursive self-audit: `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`
- TB-13 architect ruling lossless: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- TB-13 integration tests: `tests/tb_13_complete_set.rs`
- TB-12 smoke evidence (predecessor format): `handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/`

exec
/bin/bash -lc "rg --files tests src handover/evidence/tb_13_real_llm_smoke_2026-05-03 | rg 'tb_13|conditional|sequencer|typed_tx|monetary_invariant|replay_report|agent_pubkeys'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json
src/economy/monetary_invariant.rs
src/state/sequencer.rs
src/state/typed_tx.rs
tests/co1_7_extra_sequencer_head_t_advancement.rs
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/pinned_pubkeys.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/genesis_report.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/run_summary.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/rejections.jsonl
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/agent_pubkeys.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/synthetic_rejection_label.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/initial_q_state.json
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl
handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md
tests/tb_13_legacy_cpmm_forward_fence.rs
tests/tb_13_complete_set.rs

codex
The self-audit claims the main money-path checks are implemented, but I’ve already noticed the forward-fence is marker-span based by design. I’m going to trace whether that creates an actual bypass and whether the redeem/collateral math survives adversarial sequences.
exec
/bin/bash -lc 'rg -n "CompleteSetMint|CompleteSetRedeem|MarketSeed|RedeemBeforeResolution|RedeemMoreThanOwned|InvalidResolutionRef|InsufficientCollateral|assert_total_ctf_conserved|assert_complete_set_balanced" src/state/sequencer.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
39:    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
255:/// §4.3): CompleteSetMint-accept state-root domain.
259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
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
656:            assert_total_ctf_conserved(
804:            assert_total_ctf_conserved(
914:            assert_total_ctf_conserved(
1122:            assert_total_ctf_conserved(
1266:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
1313:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
1363:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
1444:            assert_total_ctf_conserved(
1490:            assert_total_ctf_conserved(
1552:            assert_total_ctf_conserved(
1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
1582:        TypedTx::CompleteSetMint(mint) => {
1646:            assert_total_ctf_conserved(
1659:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
1664:        //     Bankrupt (No); else RedeemBeforeResolution.
1665:        //   - claimed_outcome must match the state; else InvalidResolutionRef.
1667:        //     else RedeemMoreThanOwned.
1669:        //     InsufficientCollateral.
1674:        TypedTx::CompleteSetRedeem(redeem) => {
1681:                return Err(TransitionError::InvalidResolutionRef);
1690:                .ok_or(TransitionError::RedeemBeforeResolution)?;
1698:                    return Err(TransitionError::InvalidResolutionRef);
1702:                    return Err(TransitionError::RedeemBeforeResolution);
1719:                return Err(TransitionError::RedeemMoreThanOwned);
1722:            // assert_complete_set_balanced is preserved).
1731:                return Err(TransitionError::InsufficientCollateral);
1790:            assert_total_ctf_conserved(
1803:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
1807:        TypedTx::MarketSeed(seed) => {
1813:            // negative (would mirror the V1 attack on CompleteSetMint —
1817:                return Err(TransitionError::InsufficientCollateral);
1867:            assert_total_ctf_conserved(
2344:            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
2352:            | TypedTx::CompleteSetMint(_)
2353:            | TypedTx::CompleteSetRedeem(_)
2354:            | TypedTx::MarketSeed(_) => {}

exec
/bin/bash -lc 'rg -n "conditional_collateral_t|conditional_share_balances_t|total_supply_micro|assert_complete_set_balanced|assert_no_post_init_mint|ShareAmount|OutcomeSide|EventId" src/economy/monetary_invariant.rs src/state/q_state.rs src/state/typed_tx.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/typed_tx.rs:660:///   `total_supply_micro`; CR-12.2).
src/state/typed_tx.rs:1063:// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
src/state/typed_tx.rs:1066:// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
src/state/typed_tx.rs:1075:/// conditional shares. TB-13 maps `EventId` 1:1 to `TaskId` (the event
src/state/typed_tx.rs:1080:pub struct EventId(pub TaskId);
src/state/typed_tx.rs:1087:pub enum OutcomeSide {
src/state/typed_tx.rs:1092:impl Default for OutcomeSide {
src/state/typed_tx.rs:1105:pub struct ShareAmount {
src/state/typed_tx.rs:1109:impl ShareAmount {
src/state/typed_tx.rs:1115:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): build a `ShareAmount`
src/state/typed_tx.rs:1126:/// `TaskBankruptcyTx` (outcome must be `OutcomeSide::No`) or a
src/state/typed_tx.rs:1127:/// `FinalizeRewardTx` (outcome must be `OutcomeSide::Yes`) for the
src/state/typed_tx.rs:1128:/// referenced `EventId.0 == task_id`. Sequencer validates the reference
src/state/typed_tx.rs:1134:    pub claimed_outcome: OutcomeSide,
src/state/typed_tx.rs:1143:/// 3. `conditional_collateral_t[event_id] += amount`.
src/state/typed_tx.rs:1144:/// 4. `conditional_share_balances_t[(owner, event_id, Yes)] += amount.units`.
src/state/typed_tx.rs:1145:/// 5. `conditional_share_balances_t[(owner, event_id, No)]  += amount.units`.
src/state/typed_tx.rs:1153:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1170:/// 2. `conditional_share_balances_t[(owner, event_id, outcome)] >= share_amount.units`
src/state/typed_tx.rs:1172:/// 3. `conditional_collateral_t[event_id] >= share_amount.units` else
src/state/typed_tx.rs:1174:///    `assert_complete_set_balanced` holds).
src/state/typed_tx.rs:1180:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1182:    pub outcome: OutcomeSide,                 //  5
src/state/typed_tx.rs:1183:    pub share_amount: ShareAmount,            //  6
src/state/typed_tx.rs:1198:/// 4. `conditional_collateral_t[event_id] += collateral_amount`.
src/state/typed_tx.rs:1200:///    `conditional_share_balances_t[(provider, event_id, Yes)] += collateral_amount.units`
src/state/typed_tx.rs:1201:///    `conditional_share_balances_t[(provider, event_id, No)]  += collateral_amount.units`.
src/state/typed_tx.rs:1212:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1227:    pub event_id: EventId,
src/state/typed_tx.rs:1250:    pub event_id: EventId,
src/state/typed_tx.rs:1252:    pub outcome: OutcomeSide,
src/state/typed_tx.rs:1253:    pub share_amount: ShareAmount,
src/state/typed_tx.rs:1273:    pub event_id: EventId,
src/state/typed_tx.rs:1762:    /// `monetary_invariant::assert_no_post_init_mint` or
src/state/typed_tx.rs:1860:    /// `conditional_share_balances_t[owner][event_id].{yes|no}` is less
src/state/typed_tx.rs:1867:    /// if `conditional_collateral_t[event_id]` lacks the redeemed amount
src/state/typed_tx.rs:1868:    /// (should never happen if `assert_complete_set_balanced` holds).
src/state/typed_tx.rs:1957:                "TB-13 collateral missing: MarketSeed with zero collateral, or Redeem against insufficient conditional_collateral_t"
src/state/typed_tx.rs:3107:            event_id: EventId(TaskId("task-fixture-tb13-mint".into())),
src/state/typed_tx.rs:3119:            event_id: EventId(TaskId("task-fixture-tb13-redeem".into())),
src/state/typed_tx.rs:3121:            outcome: OutcomeSide::Yes,
src/state/typed_tx.rs:3122:            share_amount: ShareAmount::from_units(7_000_000),
src/state/typed_tx.rs:3125:                claimed_outcome: OutcomeSide::Yes,
src/state/typed_tx.rs:3136:            event_id: EventId(TaskId("task-fixture-tb13-seed".into())),
src/state/typed_tx.rs:3183:    /// TB-13 U4: OutcomeSide repr discriminants stable.
src/state/typed_tx.rs:3186:        assert_eq!(OutcomeSide::Yes as u8, 0);
src/state/typed_tx.rs:3187:        assert_eq!(OutcomeSide::No as u8, 1);
src/state/typed_tx.rs:3190:    /// TB-13 U5: ShareAmount default is zero.
src/state/typed_tx.rs:3193:        assert_eq!(ShareAmount::default(), ShareAmount::zero());
src/state/typed_tx.rs:3194:        assert_eq!(ShareAmount::default().units, 0u128);
src/state/q_state.rs:179:    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
src/state/q_state.rs:198:    /// holding"); included in the 6-holding `total_supply_micro` sum
src/state/q_state.rs:201:    /// `Σ_{event} conditional_collateral_t[event].units == Σ shares per side`.
src/state/q_state.rs:205:    pub conditional_collateral_t: ConditionalCollateralIndex,
src/state/q_state.rs:207:    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
src/state/q_state.rs:210:    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
src/state/q_state.rs:211:    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
src/state/q_state.rs:217:    pub conditional_share_balances_t: ConditionalShareBalances,
src/state/q_state.rs:391:///   <this task>`. `monetary_invariant::total_supply_micro` does NOT include
src/state/q_state.rs:529:/// **IS** a Coin holding — included in 6-holding `total_supply_micro` sum
src/state/q_state.rs:532:/// (`assert_complete_set_balanced`) enforces
src/state/q_state.rs:536:    pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>,
src/state/q_state.rs:544:/// explicit: shares are NOT counted in `total_supply_micro`.
src/state/q_state.rs:546:/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
src/state/q_state.rs:555:        BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>,
src/state/q_state.rs:564:    pub yes: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:565:    pub no: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:779:        // 11 → 13 sub-fields with +conditional_collateral_t (CR-13.4 Coin
src/state/q_state.rs:780:        // holding, included in 6-holding total_supply_micro) +
src/state/q_state.rs:781:        // conditional_share_balances_t (CR-13.3 claims, NOT counted in
src/state/q_state.rs:782:        // total_supply_micro).
src/state/q_state.rs:789:            "EconomicState must have 13 sub-fields post-TB-13 (was 11 post-TB-12; +conditional_collateral_t +conditional_share_balances_t); got {}",
src/state/q_state.rs:794:        assert!(obj.contains_key("conditional_collateral_t"), "TB-13 conditional_collateral_t sub-field missing");
src/state/q_state.rs:795:        assert!(obj.contains_key("conditional_share_balances_t"), "TB-13 conditional_share_balances_t sub-field missing");
src/economy/monetary_invariant.rs:79:        side: crate::state::typed_tx::OutcomeSide,
src/economy/monetary_invariant.rs:172:fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
src/economy/monetary_invariant.rs:193:    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
src/economy/monetary_invariant.rs:196:    // Coin from balances_t to conditional_collateral_t) would falsely
src/economy/monetary_invariant.rs:200:    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
src/economy/monetary_invariant.rs:201:    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
src/economy/monetary_invariant.rs:204:    for c in s.conditional_collateral_t.0.values() {
src/economy/monetary_invariant.rs:306:// assert_no_post_init_mint — structural guard at the tx layer
src/economy/monetary_invariant.rs:322:pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:360:        // assert_total_ctf_conserved with conditional_collateral_t as the
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
src/economy/monetary_invariant.rs:495:fn hex_event_id(event_id: &crate::state::typed_tx::EventId) -> String {
src/economy/monetary_invariant.rs:543:    // ── assert_no_post_init_mint ────────────────────────────────────────────
src/economy/monetary_invariant.rs:549:        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
src/economy/monetary_invariant.rs:574:            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
src/economy/monetary_invariant.rs:652:        let total0 = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:673:            // total_supply_micro unchanged.)
src/economy/monetary_invariant.rs:689:            let total_now = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:697:        assert_eq!(total_supply_micro(&s).unwrap(), total0);
src/economy/monetary_invariant.rs:748:        assert_eq!(total_supply_micro(&s).unwrap(), 55 * MICRO_PER_COIN);
src/economy/monetary_invariant.rs:756:        // must yield total_supply_micro = K, NOT 2K. If a regression adds
src/economy/monetary_invariant.rs:775:            total_supply_micro(&s).unwrap(),

exec
/bin/bash -lc 'rg -n "sg_13_|halt_|complete_set|market_seed|RedeemBeforeResolution|InvalidResolutionRef|RedeemMoreThanOwned|InsufficientCollateral" tests/tb_13_complete_set.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
16://! - SG-13.0.1 legacy_cpm_api_not_imported_by_complete_set       (Atom 0.5 fence)
17://! - SG-13.0.2 no_f64_in_complete_set_or_market_seed              (Atom 0.5 fence)
21://! - SG-13.3   market_seed_fails_if_provider_lacks_balance
22://! - SG-13.4   market_seed_cannot_create_liquidity_without_collateral
25://! - SG-13.7   no_f64_in_new_complete_set_or_market_seed_path     (Atom 0.5 fence)
47:    assert_complete_set_balanced, assert_total_ctf_conserved,
186:async fn sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved() {
242:    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
254:async fn sg_13_2_yes_no_shares_not_in_total_coin_supply() {
268:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
275:async fn sg_13_3_market_seed_fails_if_provider_lacks_balance() {
295:async fn sg_13_4_market_seed_cannot_create_liquidity_without_collateral() {
300:    // collateral_amount == 0 must fail with InsufficientCollateral.
305:        err.contains("InsufficientCollateral"),
306:        "expected InsufficientCollateral, got: {err}"
315:/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
318:async fn sg_13_5_redeem_unavailable_before_outcome_resolution() {
336:        err.contains("RedeemBeforeResolution"),
337:        "expected RedeemBeforeResolution, got: {err}"
356:        err.contains("RedeemBeforeResolution"),
357:        "expected RedeemBeforeResolution on Expired state, got: {err}"
365:async fn sg_13_6_redeem_after_yes_outcome_pays_yes_not_no() {
419:        err.contains("InvalidResolutionRef"),
420:        "expected InvalidResolutionRef, got: {err}"
439:    assert!(err.contains("InvalidResolutionRef"));
456:/// SG-13.7 (delegation marker) — `no_f64_in_new_complete_set_or_market_seed_path`
457:/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::no_f64_in_complete_set_or_market_seed`.
462:fn sg_13_7_no_f64_in_new_complete_set_or_market_seed_path() {
474:        fence_src.contains("fn no_f64_in_complete_set_or_market_seed"),
480:/// is enforced by `tests/tb_13_legacy_cpmm_forward_fence.rs::legacy_cpm_api_not_imported_by_complete_set`.
482:fn sg_13_8_no_import_or_use_of_legacy_cpmm_in_tb13_modules() {
489:        fence_src.contains("fn legacy_cpm_api_not_imported_by_complete_set"),
498:async fn halt_total_supply_micro_unchanged_across_mint_redeem() {
518:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
523:async fn halt_shares_not_counted_as_coin() {
538:async fn halt_market_seed_zero_balance_provider_rejected() {
549:/// rejected with `RedeemMoreThanOwned`.
551:async fn halt_redeem_more_than_owned_rejected() {
567:        err.contains("RedeemMoreThanOwned"),
568:        "expected RedeemMoreThanOwned, got: {err}"
578:async fn halt_negative_mint_amount_rejected() {
627:/// `InsufficientCollateral`. Same attack vector as halt_negative_mint
630:async fn halt_negative_market_seed_collateral_rejected() {
648:        err.contains("InsufficientCollateral"),
649:        "expected InsufficientCollateral for negative collateral, got: {err}"
669:async fn halt_complete_set_balanced_post_seed() {
677:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");

exec
/bin/bash -lc 'rg -n "TB-13|prediction_market|BinaryMarket|f64|forbidden|span|TRACE_MATRIX|legacy_cpm|no_f64|PriceIndex|MarketOrder|MarketTrade|AMM|CPMM" tests/tb_13_legacy_cpmm_forward_fence.rs src/prediction_market.rs src/kernel.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/kernel.rs:9:use crate::prediction_market::{BinaryMarket, MarketError};
src/kernel.rs:21:    /// **LEGACY** (TB-3..TB-10 Phase-3A): per-node f64 CPMM market book.
src/kernel.rs:22:    /// See `prediction_market.rs` module header for migration path.
src/kernel.rs:23:    /// Replaced by TB-13 `ConditionalShareBalances` (canonical YES/NO claims)
src/kernel.rs:24:    /// + TB-14 `PriceIndex` (statistical signal). Removal is TB-14 SHIP
src/kernel.rs:25:    /// prerequisite per `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03`.
src/kernel.rs:26:    pub markets: HashMap<NodeId, BinaryMarket>,
src/kernel.rs:32:    /// RSP-M**; replaced by TB-13/TB-14 substrate. Removal is TB-14 SHIP
src/kernel.rs:33:    /// prerequisite per `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03`.
src/kernel.rs:35:    pub bounty_market: Option<BinaryMarket>,
src/kernel.rs:37:    /// market at open time (separate from BinaryMarket's internal CPMM
src/kernel.rs:38:    /// book). Used for payout distribution. f64; not constitutional.
src/kernel.rs:39:    /// Replaced by TB-13 `MarketSeedTx` (explicit provider funds, integer
src/kernel.rs:42:    pub bounty_lp_seed: f64,
src/kernel.rs:74:    pub fn open_bounty_market(&mut self, lp_coins: f64) -> Result<(), KernelError> {
src/kernel.rs:78:        let market = BinaryMarket::create("__bounty__".to_string(), lp_coins)
src/kernel.rs:85:    pub fn bounty_yes_price(&self) -> Option<f64> {
src/kernel.rs:94:    pub fn resolve_bounty(&mut self, gp_authors: &[String]) -> HashMap<String, f64> {
src/kernel.rs:95:        let mut payouts: HashMap<String, f64> = HashMap::new();
src/kernel.rs:109:        let n = gp_authors.len() as f64;
src/kernel.rs:126:    pub fn create_market(&mut self, node_id: &str, lp_coins: f64) -> Result<(), KernelError> {
src/kernel.rs:133:        let market = BinaryMarket::create(node_id.to_string(), lp_coins)
src/kernel.rs:140:    pub fn buy_yes(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
src/kernel.rs:148:    pub fn buy_no(&mut self, node_id: &str, coins: f64) -> Result<f64, KernelError> {
src/kernel.rs:192:    pub fn yes_price(&self, node_id: &str) -> Option<f64> {
src/kernel.rs:197:    pub fn market_ticker(&self, top_n: usize) -> Vec<(NodeId, f64)> {
src/kernel.rs:198:        let mut prices: Vec<(NodeId, f64)> = self.markets.iter()
src/kernel.rs:209:    pub fn market_ticker_full(&self, top_n: usize) -> Vec<(NodeId, f64, f64, f64, f64)> {
src/kernel.rs:210:        let mut rows: Vec<(NodeId, f64, f64, f64, f64)> = self.markets.iter()
src/prediction_market.rs:3://! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2):
src/prediction_market.rs:8://! - **NOT** the canonical YES/NO claim system. TB-13 introduces
src/prediction_market.rs:10://! - **NOT** the canonical price index. TB-14 introduces `PriceIndex`
src/prediction_market.rs:16://! - **f64 in money path** — every reserve / price / lp field below uses
src/prediction_market.rs:17://!   `f64`; the post-2026-05 architect directive forbids f64 in money /
src/prediction_market.rs:18://!   collateral / share path (TB-13 SG-13.0.2; CR-13 forbidden list).
src/prediction_market.rs:19://! - **automatic liquidity** via constant-product market-maker — TB-13
src/prediction_market.rs:20://!   forbidden list explicitly bans automatic liquidity / ghost
src/prediction_market.rs:22://! - **trading semantics** (`buy_yes` / `buy_no`) — TB-13 forbidden list
src/prediction_market.rs:23://!   bans MarketBuy / MarketSell / MarketOrderTx / MarketTradeTx; TB-14
src/prediction_market.rs:29://! `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md`.
src/prediction_market.rs:30://! Retroactive deletion in TB-13 would break the production evaluator
src/prediction_market.rs:37://! architect §4.2 halting-trigger semantics, which target NEW TB-13 code,
src/prediction_market.rs:42://! `tests/tb_13_legacy_cpmm_forward_fence.rs` enforces that NEW TB-13
src/prediction_market.rs:43://! modules cannot import this file's types or call its f64 trading API.
src/prediction_market.rs:46:// Tier 0: CPMM Binary Market — pure math, no I/O
src/prediction_market.rs:55:/// A binary prediction market using Constant Product Market Maker (CPMM).
src/prediction_market.rs:66:pub struct BinaryMarket {
src/prediction_market.rs:68:    yes_reserve: f64,
src/prediction_market.rs:69:    no_reserve: f64,
src/prediction_market.rs:70:    k: f64,
src/prediction_market.rs:72:    lp_total: f64,
src/prediction_market.rs:75:impl BinaryMarket {
src/prediction_market.rs:77:    pub fn yes_reserve(&self) -> f64 { self.yes_reserve }
src/prediction_market.rs:78:    pub fn no_reserve(&self) -> f64 { self.no_reserve }
src/prediction_market.rs:79:    pub fn k(&self) -> f64 { self.k }
src/prediction_market.rs:80:    pub fn lp_total(&self) -> f64 { self.lp_total }
src/prediction_market.rs:86:    pub shares_received: f64,
src/prediction_market.rs:87:    pub new_yes_price: f64,
src/prediction_market.rs:88:    pub new_no_price: f64,
src/prediction_market.rs:93:impl BinaryMarket {
src/prediction_market.rs:99:    pub fn create(node_id: String, lp_coins: f64) -> Result<Self, MarketError> {
src/prediction_market.rs:105:        Ok(BinaryMarket {
src/prediction_market.rs:117:    pub fn yes_price(&self) -> f64 {
src/prediction_market.rs:122:    pub fn no_price(&self) -> f64 {
src/prediction_market.rs:132:    pub fn buy_yes(&mut self, coins_in: f64) -> Result<BuyOutcome, MarketError> {
src/prediction_market.rs:158:    pub fn buy_no(&mut self, coins_in: f64) -> Result<BuyOutcome, MarketError> {
src/prediction_market.rs:193:    pub fn redeem(&self, yes_shares: f64, no_shares: f64) -> Result<f64, MarketError> {
src/prediction_market.rs:236:    const EPSILON: f64 = 1e-9;
src/prediction_market.rs:238:    fn assert_approx(a: f64, b: f64, msg: &str) {
src/prediction_market.rs:244:        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:253:        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:260:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:275:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:287:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:295:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:303:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:311:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:319:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:329:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:338:        let m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:344:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:348:        assert!(BinaryMarket::create("n1".into(), 0.0).is_err());
src/prediction_market.rs:349:        assert!(BinaryMarket::create("n1".into(), -100.0).is_err());
src/prediction_market.rs:358:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
src/prediction_market.rs:376:        let mut m = BinaryMarket::create("n1".into(), 2000.0).unwrap();
tests/tb_13_legacy_cpmm_forward_fence.rs:1://! TB-13 Atom 0.5 — Legacy CPMM forward-fence + label ship-gate.
tests/tb_13_legacy_cpmm_forward_fence.rs:3://! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2;
tests/tb_13_legacy_cpmm_forward_fence.rs:6://! These three tests enforce the forward-binding fence that NEW TB-13
tests/tb_13_legacy_cpmm_forward_fence.rs:7://! modules cannot import or reuse the legacy `src/prediction_market.rs`
tests/tb_13_legacy_cpmm_forward_fence.rs:8://! f64 CPMM scaffolding. Architect §4.2 halting triggers:
tests/tb_13_legacy_cpmm_forward_fence.rs:10://!   HALT if new TB-13 code imports legacy prediction_market.rs.
tests/tb_13_legacy_cpmm_forward_fence.rs:11://!   HALT if f64 appears in new CompleteSet / MarketSeed code.
tests/tb_13_legacy_cpmm_forward_fence.rs:12://!   HALT if any AMM / CPMM router function is introduced in TB-13.
tests/tb_13_legacy_cpmm_forward_fence.rs:14://! ## What is "TB-13 code"?
tests/tb_13_legacy_cpmm_forward_fence.rs:16://! A span of Rust source belongs to TB-13 iff it is a contiguous block
tests/tb_13_legacy_cpmm_forward_fence.rs:18://! marker that identifies TB-13 as the contributing tracer-bullet (NOT
tests/tb_13_legacy_cpmm_forward_fence.rs:19://! a forward-reference from an earlier-TB doc-comment to TB-13's future
tests/tb_13_legacy_cpmm_forward_fence.rs:22://!   - `TRACE_MATRIX TB-13 ` (TB-12 convention used by every shipped TB).
tests/tb_13_legacy_cpmm_forward_fence.rs:23://!   - A line that begins with `// TB-13 ` after stripping leading
tests/tb_13_legacy_cpmm_forward_fence.rs:25://!   - A line that begins with `//! TB-13 ` (module-level doc).
tests/tb_13_legacy_cpmm_forward_fence.rs:26://!   - A line that begins with `/// TB-13 ` (item-level doc).
tests/tb_13_legacy_cpmm_forward_fence.rs:28://! A span ends at the next blank line OR end-of-file. Cross-references
tests/tb_13_legacy_cpmm_forward_fence.rs:29://! to TB-13 from inside a TB-12 (or earlier) span do NOT pull that span
tests/tb_13_legacy_cpmm_forward_fence.rs:30://! into TB-13 scope — only the *first non-blank line* of a span is
tests/tb_13_legacy_cpmm_forward_fence.rs:35://! - `src/state/typed_tx.rs` — TB-13 typed-tx variant additions (Atom 1).
tests/tb_13_legacy_cpmm_forward_fence.rs:36://! - `src/state/q_state.rs` — TB-13 EconomicState extensions (Atom 2).
tests/tb_13_legacy_cpmm_forward_fence.rs:37://! - `src/state/sequencer.rs` — TB-13 dispatch-arm additions (Atom 2).
tests/tb_13_legacy_cpmm_forward_fence.rs:38://! - `src/economy/monetary_invariant.rs` — TB-13 conservation extensions (Atom 3).
tests/tb_13_legacy_cpmm_forward_fence.rs:39://! - `src/bin/audit_dashboard.rs` — TB-13 §14 dashboard rendering (Atom 4).
tests/tb_13_legacy_cpmm_forward_fence.rs:41://! At Atom 0.5 ship time, none of these files contain `TB-13` markers
tests/tb_13_legacy_cpmm_forward_fence.rs:48:/// In-scope source files for the TB-13 forward-fence. NEW TB-13 markers
tests/tb_13_legacy_cpmm_forward_fence.rs:49:/// appearing in any of these files are subject to the forbidden-token
tests/tb_13_legacy_cpmm_forward_fence.rs:59:/// Tokens forbidden inside any TB-13-marker span (architect §4.2 halting
tests/tb_13_legacy_cpmm_forward_fence.rs:60:/// triggers + §4.7 forbidden list).
tests/tb_13_legacy_cpmm_forward_fence.rs:62:/// Each entry is a literal substring that must NOT appear in TB-13 code.
tests/tb_13_legacy_cpmm_forward_fence.rs:64:    // Direct legacy CPMM imports / type names.
tests/tb_13_legacy_cpmm_forward_fence.rs:65:    "prediction_market::",
tests/tb_13_legacy_cpmm_forward_fence.rs:66:    "BinaryMarket",
tests/tb_13_legacy_cpmm_forward_fence.rs:67:    // Legacy CPMM API method names.
tests/tb_13_legacy_cpmm_forward_fence.rs:77:    // f64 in money-path context (see SG-13.0.2 dedicated test for the
tests/tb_13_legacy_cpmm_forward_fence.rs:78:    // primary check; this entry catches `f64` in any TB-13-marked span).
tests/tb_13_legacy_cpmm_forward_fence.rs:79:    " f64",
tests/tb_13_legacy_cpmm_forward_fence.rs:80:    "f64,",
tests/tb_13_legacy_cpmm_forward_fence.rs:81:    "f64;",
tests/tb_13_legacy_cpmm_forward_fence.rs:82:    "f64)",
tests/tb_13_legacy_cpmm_forward_fence.rs:83:    // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
tests/tb_13_legacy_cpmm_forward_fence.rs:84:    "MarketOrderTx",
tests/tb_13_legacy_cpmm_forward_fence.rs:85:    "MarketTradeTx",
tests/tb_13_legacy_cpmm_forward_fence.rs:88:    "AMM",
tests/tb_13_legacy_cpmm_forward_fence.rs:89:    "CPMM",
tests/tb_13_legacy_cpmm_forward_fence.rs:95:    "PriceIndex",
tests/tb_13_legacy_cpmm_forward_fence.rs:106:/// Returns true if `line` is an authoring marker for TB-13 (i.e., the
tests/tb_13_legacy_cpmm_forward_fence.rs:107:/// line declares that the following block is TB-13 code, NOT a forward-
tests/tb_13_legacy_cpmm_forward_fence.rs:108:/// reference from an earlier-TB doc-comment to TB-13's future work).
tests/tb_13_legacy_cpmm_forward_fence.rs:110:    if line.contains("TRACE_MATRIX TB-13 ") {
tests/tb_13_legacy_cpmm_forward_fence.rs:119:    body.starts_with("TB-13 ")
tests/tb_13_legacy_cpmm_forward_fence.rs:122:/// Extract line ranges that belong to TB-13 additions. A span is a
tests/tb_13_legacy_cpmm_forward_fence.rs:125:fn tb_13_spans(source: &str) -> Vec<(usize, String)> {
tests/tb_13_legacy_cpmm_forward_fence.rs:135:        let span_start = i;
tests/tb_13_legacy_cpmm_forward_fence.rs:136:        let mut span_end = i;
tests/tb_13_legacy_cpmm_forward_fence.rs:137:        while span_end < n && !lines[span_end].trim().is_empty() {
tests/tb_13_legacy_cpmm_forward_fence.rs:138:            span_end += 1;
tests/tb_13_legacy_cpmm_forward_fence.rs:140:        let span = &lines[span_start..span_end];
tests/tb_13_legacy_cpmm_forward_fence.rs:141:        if is_tb_13_authoring_marker(span[0]) {
tests/tb_13_legacy_cpmm_forward_fence.rs:142:            for (offset, line) in span.iter().enumerate() {
tests/tb_13_legacy_cpmm_forward_fence.rs:143:                out.push((span_start + offset + 1, (*line).to_string()));
tests/tb_13_legacy_cpmm_forward_fence.rs:146:        i = span_end;
tests/tb_13_legacy_cpmm_forward_fence.rs:157:        .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
tests/tb_13_legacy_cpmm_forward_fence.rs:160:/// Hard-banned legacy CPMM imports — these strings MUST NOT appear in
tests/tb_13_legacy_cpmm_forward_fence.rs:161:/// any FENCE_SCOPE file regardless of TB-13-marker discipline. Codex
tests/tb_13_legacy_cpmm_forward_fence.rs:163:/// could be bypassed by writing a legacy import outside a TB-13 doc-
tests/tb_13_legacy_cpmm_forward_fence.rs:164:/// comment span. These tokens are unconditionally banned (a `use
tests/tb_13_legacy_cpmm_forward_fence.rs:165:/// crate::prediction_market::BinaryMarket` anywhere in scope is an
tests/tb_13_legacy_cpmm_forward_fence.rs:166:/// architectural regression even in non-TB-13 sections).
tests/tb_13_legacy_cpmm_forward_fence.rs:168:    "use crate::prediction_market::",
tests/tb_13_legacy_cpmm_forward_fence.rs:169:    "use crate::prediction_market;",
tests/tb_13_legacy_cpmm_forward_fence.rs:170:    "crate::prediction_market::BinaryMarket",
tests/tb_13_legacy_cpmm_forward_fence.rs:171:    "crate::prediction_market::MarketError",
tests/tb_13_legacy_cpmm_forward_fence.rs:174:/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
tests/tb_13_legacy_cpmm_forward_fence.rs:176:/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
tests/tb_13_legacy_cpmm_forward_fence.rs:177:/// `prediction_market.rs`. Two layers of enforcement:
tests/tb_13_legacy_cpmm_forward_fence.rs:181:/// TB-13-marker discipline. Catches any new use-statement or type
tests/tb_13_legacy_cpmm_forward_fence.rs:182:/// reference that pulls legacy CPMM into a TB-13-scope module.
tests/tb_13_legacy_cpmm_forward_fence.rs:184:/// **Layer 2 (TB-13-marker-scoped)**: scan TB-13-marked spans for the
tests/tb_13_legacy_cpmm_forward_fence.rs:185:/// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
tests/tb_13_legacy_cpmm_forward_fence.rs:187:/// historical doc-comments while keeping new TB-13 code clean.
tests/tb_13_legacy_cpmm_forward_fence.rs:189:fn legacy_cpm_api_not_imported_by_complete_set() {
tests/tb_13_legacy_cpmm_forward_fence.rs:207:    // Layer 2: TB-13-marker-scoped scan for trading/AMM concepts.
tests/tb_13_legacy_cpmm_forward_fence.rs:210:        for (line_no, line) in tb_13_spans(&source) {
tests/tb_13_legacy_cpmm_forward_fence.rs:212:                // The `f64` family entries are checked in SG-13.0.2 — skip
tests/tb_13_legacy_cpmm_forward_fence.rs:214:                if token.starts_with(" f64")
tests/tb_13_legacy_cpmm_forward_fence.rs:215:                    || token.starts_with("f64,")
tests/tb_13_legacy_cpmm_forward_fence.rs:216:                    || token.starts_with("f64;")
tests/tb_13_legacy_cpmm_forward_fence.rs:217:                    || token.starts_with("f64)")
tests/tb_13_legacy_cpmm_forward_fence.rs:223:                        "{rel}:{line_no}: TB-13-marked span contains forbidden token `{token}` — {line}"
tests/tb_13_legacy_cpmm_forward_fence.rs:231:        "TB-13 SG-13.0.1 forward-fence violated:\n{}",
tests/tb_13_legacy_cpmm_forward_fence.rs:236:/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`.
tests/tb_13_legacy_cpmm_forward_fence.rs:238:/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
tests/tb_13_legacy_cpmm_forward_fence.rs:242:fn no_f64_in_complete_set_or_market_seed() {
tests/tb_13_legacy_cpmm_forward_fence.rs:244:    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
tests/tb_13_legacy_cpmm_forward_fence.rs:247:        for (line_no, line) in tb_13_spans(&source) {
tests/tb_13_legacy_cpmm_forward_fence.rs:248:            for token in &f64_tokens {
tests/tb_13_legacy_cpmm_forward_fence.rs:251:                        "{rel}:{line_no}: TB-13-marked span contains f64 (`{token}`) — {line}"
tests/tb_13_legacy_cpmm_forward_fence.rs:259:        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
tests/tb_13_legacy_cpmm_forward_fence.rs:264:/// SG-13.0.3 — `prediction_market_legacy_quarantined`.
tests/tb_13_legacy_cpmm_forward_fence.rs:266:/// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
tests/tb_13_legacy_cpmm_forward_fence.rs:267:/// enforce that `src/prediction_market.rs` carries the LEGACY module-
tests/tb_13_legacy_cpmm_forward_fence.rs:273:fn prediction_market_legacy_quarantined() {
tests/tb_13_legacy_cpmm_forward_fence.rs:274:    let pm = read_scope_file("src/prediction_market.rs");
tests/tb_13_legacy_cpmm_forward_fence.rs:290:            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
tests/tb_13_legacy_cpmm_forward_fence.rs:298:        "TB-13",
tests/tb_13_legacy_cpmm_forward_fence.rs:301:        "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
tests/tb_13_legacy_cpmm_forward_fence.rs:306:            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
tests/tb_13_legacy_cpmm_forward_fence.rs:315:        .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
tests/tb_13_legacy_cpmm_forward_fence.rs:319:        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
tests/tb_13_legacy_cpmm_forward_fence.rs:326:            .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
tests/tb_13_legacy_cpmm_forward_fence.rs:333:            "TB-13 SG-13.0.3: Kernel.{field} missing LEGACY doc-comment label. \

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
   160	/// Hard-banned legacy CPMM imports — these strings MUST NOT appear in
   161	/// any FENCE_SCOPE file regardless of TB-13-marker discipline. Codex
   162	/// round-1 Q9 CHALLENGE remediation (2026-05-03): the marker-only fence
   163	/// could be bypassed by writing a legacy import outside a TB-13 doc-
   164	/// comment span. These tokens are unconditionally banned (a `use
   165	/// crate::prediction_market::BinaryMarket` anywhere in scope is an
   166	/// architectural regression even in non-TB-13 sections).
   167	const HARD_BANNED_LEGACY_IMPORTS: &[&str] = &[
   168	    "use crate::prediction_market::",
   169	    "use crate::prediction_market;",
   170	    "crate::prediction_market::BinaryMarket",
   171	    "crate::prediction_market::MarketError",
   172	];
   173	
   174	/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
   175	///
   176	/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
   177	/// `prediction_market.rs`. Two layers of enforcement:
   178	///
   179	/// **Layer 1 (unconditional, Codex round-1 Q9 remediation)**: scan every
   180	/// FENCE_SCOPE file for `HARD_BANNED_LEGACY_IMPORTS` regardless of
   181	/// TB-13-marker discipline. Catches any new use-statement or type
   182	/// reference that pulls legacy CPMM into a TB-13-scope module.
   183	///
   184	/// **Layer 2 (TB-13-marker-scoped)**: scan TB-13-marked spans for the
   185	/// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
   186	/// concepts). The marker discipline allows benign references in
   187	/// historical doc-comments while keeping new TB-13 code clean.
   188	#[test]
   189	fn legacy_cpm_api_not_imported_by_complete_set() {
   190	    let mut violations: Vec<String> = Vec::new();
   191	
   192	    // Layer 1: unconditional whole-file scan for hard-banned imports.
   193	    for rel in FENCE_SCOPE {
   194	        let source = read_scope_file(rel);
   195	        for (line_no, line) in source.lines().enumerate() {
   196	            for token in HARD_BANNED_LEGACY_IMPORTS {
   197	                if line.contains(token) {
   198	                    violations.push(format!(
   199	                        "{rel}:{}: hard-banned legacy import `{token}` — {line}",
   200	                        line_no + 1
   201	                    ));
   202	                }
   203	            }
   204	        }
   205	    }
   206	
   207	    // Layer 2: TB-13-marker-scoped scan for trading/AMM concepts.
   208	    for rel in FENCE_SCOPE {
   209	        let source = read_scope_file(rel);
   210	        for (line_no, line) in tb_13_spans(&source) {
   211	            for token in FORBIDDEN_LEGACY_TOKENS {
   212	                // The `f64` family entries are checked in SG-13.0.2 — skip
   213	                // them here so the failure message is unambiguous.
   214	                if token.starts_with(" f64")
   215	                    || token.starts_with("f64,")
   216	                    || token.starts_with("f64;")
   217	                    || token.starts_with("f64)")
   218	                {
   219	                    continue;
   220	                }
   221	                if line.contains(token) {
   222	                    violations.push(format!(
   223	                        "{rel}:{line_no}: TB-13-marked span contains forbidden token `{token}` — {line}"
   224	                    ));
   225	                }
   226	            }
   227	        }
   228	    }
   229	    assert!(
   230	        violations.is_empty(),
   231	        "TB-13 SG-13.0.1 forward-fence violated:\n{}",
   232	        violations.join("\n")
   233	    );
   234	}
   235	
   236	/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`.
   237	///
   238	/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
   239	/// CompleteSet / MarketSeed code. Money-path types must use integer
   240	/// `MicroCoin` / `ShareAmount`.
   241	#[test]
   242	fn no_f64_in_complete_set_or_market_seed() {
   243	    let mut violations: Vec<String> = Vec::new();
   244	    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
   245	    for rel in FENCE_SCOPE {
   246	        let source = read_scope_file(rel);
   247	        for (line_no, line) in tb_13_spans(&source) {
   248	            for token in &f64_tokens {
   249	                if line.contains(token) {
   250	                    violations.push(format!(
   251	                        "{rel}:{line_no}: TB-13-marked span contains f64 (`{token}`) — {line}"
   252	                    ));
   253	                }
   254	            }
   255	        }
   256	    }
   257	    assert!(
   258	        violations.is_empty(),
   259	        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
   260	        violations.join("\n")
   261	    );
   262	}
   263	
   264	/// SG-13.0.3 — `prediction_market_legacy_quarantined`.
   265	///
   266	/// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
   267	/// enforce that `src/prediction_market.rs` carries the LEGACY module-
   268	/// header doc-comment with the four required tokens (`legacy`,
   269	/// `not constitutional`, `not RSP-M`, `not production market path`)
   270	/// AND that `src/kernel.rs` market-bearing fields carry the `LEGACY`
   271	/// label tying them to the migration path.
   272	#[test]
   273	fn prediction_market_legacy_quarantined() {
   274	    let pm = read_scope_file("src/prediction_market.rs");
   275	    let header = pm
   276	        .lines()
   277	        .take(60)
   278	        .collect::<Vec<_>>()
   279	        .join("\n");
   280	
   281	    let required_label_tokens = [
   282	        "LEGACY",
   283	        "not constitutional",
   284	        "not RSP-M",
   285	        "not production market path",
   286	    ];
   287	    for token in &required_label_tokens {
   288	        assert!(
   289	            header.contains(token),
   290	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
   291	             label token `{token}`. Header:\n{header}"
   292	        );
   293	    }
   294	
   295	    // Architect §4.2 also requires the doc to name the migration path so
   296	    // future maintainers don't reintroduce the legacy API.
   297	    let migration_tokens = [
   298	        "TB-13",
   299	        "TB-14",
   300	        "CompleteSetMintTx",
   301	        "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
   302	    ];
   303	    for token in &migration_tokens {
   304	        assert!(
   305	            header.contains(token),
   306	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
   307	             token `{token}`. Header:\n{header}"
   308	        );
   309	    }
   310	
   311	    // Defense-in-depth: kernel.rs market-bearing fields carry LEGACY.
   312	    let kernel = read_scope_file("src/kernel.rs");
   313	    let kernel_struct_idx = kernel
   314	        .find("pub struct Kernel {")
   315	        .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
   316	    let kernel_struct_end = kernel[kernel_struct_idx..]
   317	        .find("\n}\n")
   318	        .map(|off| kernel_struct_idx + off + 2)
   319	        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
   320	    let kernel_struct_block = &kernel[kernel_struct_idx..kernel_struct_end];
   321	
   322	    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
   323	        let field_marker = format!("pub {field}");
   324	        let field_idx = kernel_struct_block
   325	            .find(&field_marker)
   326	            .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
   327	        // Look for `LEGACY` in the 600 chars preceding the field
   328	        // declaration (covers a multi-line doc-comment block).
   329	        let doc_window_start = field_idx.saturating_sub(600);
   330	        let doc_window = &kernel_struct_block[doc_window_start..field_idx];
   331	        assert!(
   332	            doc_window.contains("LEGACY"),
   333	            "TB-13 SG-13.0.3: Kernel.{field} missing LEGACY doc-comment label. \
   334	             Doc window:\n{doc_window}"
   335	        );
   336	    }
   337	}

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '1560,1885p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
  1587	            // Step 2: amount > 0 strictly. `MicroCoin` is i64-backed and
  1588	            // permits negative values at the type level (see
  1589	            // `src/economy/money.rs`); `<= 0` rejects both zero (no-op
  1590	            // mint) and negative (would credit balance + write negative
  1591	            // collateral + cast to huge u128 shares). Codex round-1 VETO
  1592	            // TB13-V1 remediation (2026-05-03).
  1593	            if mint.amount.micro_units() <= 0 {
  1594	                return Err(TransitionError::InsufficientBalanceForMint);
  1595	            }
  1596	            // Step 3: owner solvency.
  1597	            let owner_bal = q
  1598	                .economic_state_t
  1599	                .balances_t
  1600	                .0
  1601	                .get(&mint.owner)
  1602	                .copied()
  1603	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1604	            if owner_bal.micro_units() < mint.amount.micro_units() {
  1605	                return Err(TransitionError::InsufficientBalanceForMint);
  1606	            }
  1607	            // Step 4: build q_next — atomic balance → collateral migration +
  1608	            // equal YES_E + NO_E share mint. The 6-holding sum (Atom 3
  1609	            // monetary_invariant extension) treats conditional_collateral_t
  1610	            // as a Coin holding, so total_supply_micro is preserved
  1611	            // bit-for-bit across mint.
  1612	            let mut q_next = q.clone();
  1613	            let new_bal_micro = owner_bal.micro_units() - mint.amount.micro_units();
  1614	            q_next.economic_state_t.balances_t.0.insert(
  1615	                mint.owner.clone(),
  1616	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
  1617	            );
  1618	            let collateral_entry = q_next
  1619	                .economic_state_t
  1620	                .conditional_collateral_t
  1621	                .0
  1622	                .entry(mint.event_id.clone())
  1623	                .or_insert(crate::economy::money::MicroCoin::zero());
  1624	            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1625	                collateral_entry.micro_units() + mint.amount.micro_units(),
  1626	            );
  1627	            let owner_shares = q_next
  1628	                .economic_state_t
  1629	                .conditional_share_balances_t
  1630	                .0
  1631	                .entry(mint.owner.clone())
  1632	                .or_insert_with(std::collections::BTreeMap::new);
  1633	            let pair = owner_shares
  1634	                .entry(mint.event_id.clone())
  1635	                .or_insert(crate::state::q_state::ShareSidePair::default());
  1636	            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1637	                pair.yes.units + mint.amount.micro_units() as u128,
  1638	            );
  1639	            pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1640	                pair.no.units + mint.amount.micro_units() as u128,
  1641	            );
  1642	
  1643	            // Step 5: monetary invariants.
  1644	            assert_no_post_init_mint(tx, q)
  1645	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1646	            assert_total_ctf_conserved(
  1647	                &q.economic_state_t,
  1648	                &q_next.economic_state_t,
  1649	                &[],
  1650	            )
  1651	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1652	
  1653	            // Step 6: state_root advance.
  1654	            q_next.state_root_t = complete_set_mint_accept_state_root(&q.state_root_t, tx);
  1655	
  1656	            Ok((q_next, SignalBundle::default()))
  1657	        }
  1658	        // ──────────────────────────────────────────────────────────────────
  1659	        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
  1660	        // FR-13.4..5 + SG-13.5..6).
  1661	        //
  1662	        // Validation:
  1663	        //   - task_markets_t[event_id.0].state must be Finalized (Yes) or
  1664	        //     Bankrupt (No); else RedeemBeforeResolution.
  1665	        //   - claimed_outcome must match the state; else InvalidResolutionRef.
  1666	        //   - owner's winning-side share balance must cover share_amount;
  1667	        //     else RedeemMoreThanOwned.
  1668	        //   - event collateral must cover share_amount; else
  1669	        //     InsufficientCollateral.
  1670	        //
  1671	        // Effect: 1 share → 1 MicroCoin (architect §4.3: "after YES outcome
  1672	        // pays YES shares"). Debit shares + collateral; credit balance.
  1673	        // ──────────────────────────────────────────────────────────────────
  1674	        TypedTx::CompleteSetRedeem(redeem) => {
  1675	            if redeem.parent_state_root != q.state_root_t {
  1676	                return Err(TransitionError::StaleParent);
  1677	            }
  1678	            // Step 1: claimed_outcome consistency between resolution_ref and
  1679	            // outcome field — both must agree before we even check state.
  1680	            if redeem.outcome != redeem.resolution_ref.claimed_outcome {
  1681	                return Err(TransitionError::InvalidResolutionRef);
  1682	            }
  1683	            // Step 2: lookup task_markets_t state.
  1684	            let market_state = q
  1685	                .economic_state_t
  1686	                .task_markets_t
  1687	                .0
  1688	                .get(&redeem.event_id.0)
  1689	                .map(|m| m.state)
  1690	                .ok_or(TransitionError::RedeemBeforeResolution)?;
  1691	            match (market_state, redeem.outcome) {
  1692	                (crate::state::q_state::TaskMarketState::Finalized,
  1693	                 crate::state::typed_tx::OutcomeSide::Yes) => { /* ok — YES wins */ }
  1694	                (crate::state::q_state::TaskMarketState::Bankrupt,
  1695	                 crate::state::typed_tx::OutcomeSide::No) => { /* ok — NO wins */ }
  1696	                (crate::state::q_state::TaskMarketState::Finalized, _)
  1697	                | (crate::state::q_state::TaskMarketState::Bankrupt, _) => {
  1698	                    return Err(TransitionError::InvalidResolutionRef);
  1699	                }
  1700	                (crate::state::q_state::TaskMarketState::Open, _)
  1701	                | (crate::state::q_state::TaskMarketState::Expired, _) => {
  1702	                    return Err(TransitionError::RedeemBeforeResolution);
  1703	                }
  1704	            }
  1705	            // Step 3: owner's share balance for the winning side.
  1706	            let pair = q
  1707	                .economic_state_t
  1708	                .conditional_share_balances_t
  1709	                .0
  1710	                .get(&redeem.owner)
  1711	                .and_then(|m| m.get(&redeem.event_id))
  1712	                .copied()
  1713	                .unwrap_or_default();
  1714	            let owned_units = match redeem.outcome {
  1715	                crate::state::typed_tx::OutcomeSide::Yes => pair.yes.units,
  1716	                crate::state::typed_tx::OutcomeSide::No => pair.no.units,
  1717	            };
  1718	            if owned_units < redeem.share_amount.units {
  1719	                return Err(TransitionError::RedeemMoreThanOwned);
  1720	            }
  1721	            // Step 4: collateral coverage (defensive; should hold if
  1722	            // assert_complete_set_balanced is preserved).
  1723	            let event_collateral = q
  1724	                .economic_state_t
  1725	                .conditional_collateral_t
  1726	                .0
  1727	                .get(&redeem.event_id)
  1728	                .copied()
  1729	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1730	            if (event_collateral.micro_units() as u128) < redeem.share_amount.units {
  1731	                return Err(TransitionError::InsufficientCollateral);
  1732	            }
  1733	
  1734	            // Step 5: build q_next.
  1735	            let mut q_next = q.clone();
  1736	            // 5a: debit the winning side from owner's share balance.
  1737	            {
  1738	                let owner_shares = q_next
  1739	                    .economic_state_t
  1740	                    .conditional_share_balances_t
  1741	                    .0
  1742	                    .entry(redeem.owner.clone())
  1743	                    .or_insert_with(std::collections::BTreeMap::new);
  1744	                let pair = owner_shares
  1745	                    .entry(redeem.event_id.clone())
  1746	                    .or_insert(crate::state::q_state::ShareSidePair::default());
  1747	                match redeem.outcome {
  1748	                    crate::state::typed_tx::OutcomeSide::Yes => {
  1749	                        pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1750	                            pair.yes.units - redeem.share_amount.units,
  1751	                        );
  1752	                    }
  1753	                    crate::state::typed_tx::OutcomeSide::No => {
  1754	                        pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1755	                            pair.no.units - redeem.share_amount.units,
  1756	                        );
  1757	                    }
  1758	                }
  1759	            }
  1760	            // 5b: debit collateral.
  1761	            {
  1762	                let collateral_entry = q_next
  1763	                    .economic_state_t
  1764	                    .conditional_collateral_t
  1765	                    .0
  1766	                    .entry(redeem.event_id.clone())
  1767	                    .or_insert(crate::economy::money::MicroCoin::zero());
  1768	                *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1769	                    collateral_entry.micro_units() - redeem.share_amount.units as i64,
  1770	                );
  1771	            }
  1772	            // 5c: credit owner's balance 1:1 (1 winning share = 1 MicroCoin).
  1773	            let owner_bal = q_next
  1774	                .economic_state_t
  1775	                .balances_t
  1776	                .0
  1777	                .get(&redeem.owner)
  1778	                .copied()
  1779	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1780	            q_next.economic_state_t.balances_t.0.insert(
  1781	                redeem.owner.clone(),
  1782	                crate::economy::money::MicroCoin::from_micro_units(
  1783	                    owner_bal.micro_units() + redeem.share_amount.units as i64,
  1784	                ),
  1785	            );
  1786	
  1787	            // Step 6: monetary invariants.
  1788	            assert_no_post_init_mint(tx, q)
  1789	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1790	            assert_total_ctf_conserved(
  1791	                &q.economic_state_t,
  1792	                &q_next.economic_state_t,
  1793	                &[],
  1794	            )
  1795	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1796	
  1797	            // Step 7: state_root advance.
  1798	            q_next.state_root_t = complete_set_redeem_accept_state_root(&q.state_root_t, tx);
  1799	
  1800	            Ok((q_next, SignalBundle::default()))
  1801	        }
  1802	        // ──────────────────────────────────────────────────────────────────
  1803	        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
  1804	        // SG-13.3..4). Provider explicitly funds collateral + receives BOTH
  1805	        // YES + NO share inventory. **No trading. No quoting. No pricing.**
  1806	        // ──────────────────────────────────────────────────────────────────
  1807	        TypedTx::MarketSeed(seed) => {
  1808	            if seed.parent_state_root != q.state_root_t {
  1809	                return Err(TransitionError::StaleParent);
  1810	            }
  1811	            // Step 1: collateral_amount > 0 strictly (architect SG-13.4).
  1812	            // `<= 0` rejects both zero (architect direct mandate) and
  1813	            // negative (would mirror the V1 attack on CompleteSetMint —
  1814	            // negative collateral + huge u128 shares). Codex round-1
  1815	            // VETO TB13-V1 remediation (2026-05-03).
  1816	            if seed.collateral_amount.micro_units() <= 0 {
  1817	                return Err(TransitionError::InsufficientCollateral);
  1818	            }
  1819	            // Step 2: provider solvency (architect SG-13.3).
  1820	            let provider_bal = q
  1821	                .economic_state_t
  1822	                .balances_t
  1823	                .0
  1824	                .get(&seed.provider)
  1825	                .copied()
  1826	                .unwrap_or(crate::economy::money::MicroCoin::zero());
  1827	            if provider_bal.micro_units() < seed.collateral_amount.micro_units() {
  1828	                return Err(TransitionError::InsufficientBalanceForMint);
  1829	            }
  1830	            // Step 3: build q_next — provider balance → collateral + provider
  1831	            // receives BOTH YES + NO share inventory.
  1832	            let mut q_next = q.clone();
  1833	            let new_bal_micro =
  1834	                provider_bal.micro_units() - seed.collateral_amount.micro_units();
  1835	            q_next.economic_state_t.balances_t.0.insert(
  1836	                seed.provider.clone(),
  1837	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
  1838	            );
  1839	            let collateral_entry = q_next
  1840	                .economic_state_t
  1841	                .conditional_collateral_t
  1842	                .0
  1843	                .entry(seed.event_id.clone())
  1844	                .or_insert(crate::economy::money::MicroCoin::zero());
  1845	            *collateral_entry = crate::economy::money::MicroCoin::from_micro_units(
  1846	                collateral_entry.micro_units() + seed.collateral_amount.micro_units(),
  1847	            );
  1848	            let provider_shares = q_next
  1849	                .economic_state_t
  1850	                .conditional_share_balances_t
  1851	                .0
  1852	                .entry(seed.provider.clone())
  1853	                .or_insert_with(std::collections::BTreeMap::new);
  1854	            let pair = provider_shares
  1855	                .entry(seed.event_id.clone())
  1856	                .or_insert(crate::state::q_state::ShareSidePair::default());
  1857	            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
  1858	                pair.yes.units + seed.collateral_amount.micro_units() as u128,
  1859	            );
  1860	            pair.no = crate::state::typed_tx::ShareAmount::from_units(
  1861	                pair.no.units + seed.collateral_amount.micro_units() as u128,
  1862	            );
  1863	
  1864	            // Step 4: monetary invariants.
  1865	            assert_no_post_init_mint(tx, q)
  1866	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1867	            assert_total_ctf_conserved(
  1868	                &q.economic_state_t,
  1869	                &q_next.economic_state_t,
  1870	                &[],
  1871	            )
  1872	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
  1873	
  1874	            // Step 5: state_root advance.
  1875	            q_next.state_root_t = market_seed_accept_state_root(&q.state_root_t, tx);
  1876	
  1877	            Ok((q_next, SignalBundle::default()))
  1878	        }
  1879	    }
  1880	}
  1881	
  1882	// ────────────────────────────────────────────────────────────────────────────
  1883	// CO1.7-extra D2: advance_head_t — post-commit head_t close (Art 0.4)
  1884	// ────────────────────────────────────────────────────────────────────────────
  1885	

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '150,505p'" in /home/zephryj/projects/turingosv4
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
   226	/// `claims_t` or `escrows_t`. TB-8 dispatch sites:
   227	/// - Atom 1 (Verify-Confirm claim creation): post-mutation on `q_next`.
   228	/// - Atom 3 (FinalizeReward dispatch): post-mutation on `q_next` (the
   229	///   Finalized status flip means the just-finalized claim is excluded).
   230	pub fn assert_claim_amount_backed_by_escrow(
   231	    s: &EconomicState,
   232	) -> Result<(), MonetaryError> {
   233	    use crate::state::q_state::ClaimStatus;
   234	    for claim in s.claims_t.0.values() {
   235	        if claim.status != ClaimStatus::Open {
   236	            continue;
   237	        }
   238	        let backing = s
   239	            .escrows_t
   240	            .0
   241	            .get(&claim.escrow_lock_tx_id)
   242	            .map(|e| e.amount.micro_units())
   243	            .unwrap_or(0);
   244	        if claim.amount.micro_units() > backing {
   245	            return Err(MonetaryError::ClaimUnbacked {
   246	                claim_amount_micro: claim.amount.micro_units(),
   247	                backing_escrow_micro: backing,
   248	            });
   249	        }
   250	    }
   251	    Ok(())
   252	}
   253	
   254	// ────────────────────────────────────────────────────────────────────────────
   255	// assert_task_market_total_escrow_matches_locks — TB-3 cache=truth invariant
   256	// ────────────────────────────────────────────────────────────────────────────
   257	
   258	/// TRACE_MATRIX TB-3 charter § 3.2 — cache=truth invariant for the derived
   259	/// `task_markets_t[task_id].total_escrow` field.
   260	///
   261	/// Asserts `cached == Σ escrows_t[e].amount where e.task_id == task_id`.
   262	/// MUST hold across every accepted state transition that touches escrows or
   263	/// task_markets (i.e., across every accepted `EscrowLockTx` and any future
   264	/// RSP-2/3+ transition that releases escrowed funds).
   265	///
   266	/// **Why this is a separate predicate**: per Art 0.2 ("派生视图 ... 必须有
   267	/// `assert_eq!(view, derive_from_tape(tape))` 守恒测试"), any cached
   268	/// aggregate of tape-derived data is a "派生视图" (derived view); without an
   269	/// explicit invariant test it becomes a parallel ledger and a ghost-liquidity
   270	/// surface. This predicate is the contract enforcing the cache stays in
   271	/// sync with the source-of-truth derivation.
   272	///
   273	/// **Caller convention**: invoked from `dispatch_transition::EscrowLock` arm
   274	/// (TB-3 Atom 5) on the post-mutation `q_next` and from any future arm that
   275	/// modifies `escrows_t` or `task_markets_t.total_escrow`. NOT invoked on
   276	/// rejection paths (rejected transitions don't mutate economic state).
   277	pub fn assert_task_market_total_escrow_matches_locks(
   278	    s: &EconomicState,
   279	    task_id: &TaskId,
   280	) -> Result<(), MonetaryError> {
   281	    let cached = s
   282	        .task_markets_t
   283	        .0
   284	        .get(task_id)
   285	        .map(|m| m.total_escrow.micro_units())
   286	        .unwrap_or(0);
   287	    let mut derived: i64 = 0;
   288	    for e in s.escrows_t.0.values() {
   289	        if &e.task_id == task_id {
   290	            derived = derived
   291	                .checked_add(e.amount.micro_units())
   292	                .ok_or(MonetaryError::Overflow)?;
   293	        }
   294	    }
   295	    if cached != derived {
   296	        return Err(MonetaryError::DerivedCacheMismatch {
   297	            task_id: task_id.clone(),
   298	            cached_micro: cached,
   299	            derived_micro: derived,
   300	        });
   301	    }
   302	    Ok(())
   303	}
   304	
   305	// ────────────────────────────────────────────────────────────────────────────
   306	// assert_no_post_init_mint — structural guard at the tx layer
   307	// ────────────────────────────────────────────────────────────────────────────
   308	
   309	/// TRACE_MATRIX P3:1 (kill 1) — structural guard against post-genesis mint
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
   411	/// TRACE_MATRIX P3:2 — assert that no K5 `TxKind` carries a per-tx fee.
   412	///
   413	/// K5 spec: every `TypedTx` variant has stake / bond fields (locked but
   414	/// not consumed) but NO fee field. A non-zero `fee` is a structural bug
   415	/// in whichever caller computed it; this fn is the barrier.
   416	///
   417	/// Note: tool-level read-is-free for `rtool` / `search` / `think` is
   418	/// enforced at the SDK boundary in a later RSP atom (out of scope for
   419	/// RSP-0). This fn covers the tx-level invariant only.
   420	pub fn assert_read_is_free(tx_kind: TxKind, fee: u64) -> Result<(), MonetaryError> {
   421	    if fee != 0 {
   422	        return Err(MonetaryError::ReadCharged { tx_kind, fee });
   423	    }
   424	    Ok(())
   425	}
   426	
   427	// ────────────────────────────────────────────────────────────────────────────
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
   501	// ────────────────────────────────────────────────────────────────────────────
   502	
   503	#[cfg(test)]
   504	mod tests {
   505	    use super::*;

exec
/bin/bash -lc "rg -n \"assert_complete_set_balanced\\(\" src tests | sort" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/monetary_invariant.rs:457:pub fn assert_complete_set_balanced(
tests/tb_13_complete_set.rs:242:    assert_complete_set_balanced(&q.economic_state_t).expect("complete-set balanced post-mint");
tests/tb_13_complete_set.rs:268:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
tests/tb_13_complete_set.rs:518:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced");
tests/tb_13_complete_set.rs:677:    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1060,1225p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '188,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '520,570p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '1845,1875p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba tests/tb_13_complete_set.rs | sed -n '160,700p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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
   572	/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin` amount
   573	/// in CompleteSetMintTx must be rejected. `MicroCoin` is i64-backed and
   574	/// permits negative values at the type layer; the dispatch arm gates
   575	/// `<= 0` (not just `== 0`). Without this gate, a negative mint would
   576	/// credit balance + write negative collateral + cast to huge u128 shares.
   577	#[tokio::test]
   578	async fn halt_negative_mint_amount_rejected() {
   579	    let q0 = genesis_with_balances(&[("alice", 100)]);
   580	    let mut h = fresh_harness(q0);
   581	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   582	
   583	    let neg_mint = TypedTx::CompleteSetMint(CompleteSetMintTx {
   584	        tx_id: TxId("neg-mint-fixture".into()),
   585	        parent_state_root: parent,
   586	        event_id: EventId(TaskId("task-NEG".into())),
   587	        owner: AgentId("alice".into()),
   588	        amount: MicroCoin::from_micro_units(-1_000_000),
   589	        signature: AgentSignature::from_bytes([0u8; 64]),
   590	        timestamp_logical: 999,
   591	    });
   592	    let err = submit_and_apply(&mut h, neg_mint)
   593	        .await
   594	        .expect_err("negative mint must be rejected");
   595	    assert!(
   596	        err.contains("InsufficientBalanceForMint"),
   597	        "expected InsufficientBalanceForMint for negative amount, got: {err}"
   598	    );
   599	
   600	    // Verify alice balance unchanged.
   601	    let q = h.seq.q_snapshot().unwrap();
   602	    let alice_bal = q
   603	        .economic_state_t
   604	        .balances_t
   605	        .0
   606	        .get(&AgentId("alice".into()))
   607	        .copied()
   608	        .unwrap();
   609	    assert_eq!(
   610	        alice_bal.micro_units(),
   611	        100_i64 * 1_000_000,
   612	        "alice balance MUST be unchanged after negative-mint rejection"
   613	    );
   614	    // Verify no collateral written.
   615	    assert!(
   616	        q.economic_state_t
   617	            .conditional_collateral_t
   618	            .0
   619	            .get(&EventId(TaskId("task-NEG".into())))
   620	            .is_none(),
   621	        "no collateral must be written under negative-mint rejection"
   622	    );
   623	}
   624	
   625	/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin`
   626	/// collateral_amount in MarketSeedTx must be rejected with
   627	/// `InsufficientCollateral`. Same attack vector as halt_negative_mint
   628	/// but via the seed path.
   629	#[tokio::test]
   630	async fn halt_negative_market_seed_collateral_rejected() {
   631	    let q0 = genesis_with_balances(&[("provider", 50)]);
   632	    let mut h = fresh_harness(q0);
   633	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   634	
   635	    let neg_seed = TypedTx::MarketSeed(MarketSeedTx {
   636	        tx_id: TxId("neg-seed-fixture".into()),
   637	        parent_state_root: parent,
   638	        event_id: EventId(TaskId("task-NEGS".into())),
   639	        provider: AgentId("provider".into()),
   640	        collateral_amount: MicroCoin::from_micro_units(-500_000),
   641	        signature: AgentSignature::from_bytes([0u8; 64]),
   642	        timestamp_logical: 998,
   643	    });
   644	    let err = submit_and_apply(&mut h, neg_seed)
   645	        .await
   646	        .expect_err("negative-collateral seed must be rejected");
   647	    assert!(
   648	        err.contains("InsufficientCollateral"),
   649	        "expected InsufficientCollateral for negative collateral, got: {err}"
   650	    );
   651	
   652	    let q = h.seq.q_snapshot().unwrap();
   653	    let provider_bal = q
   654	        .economic_state_t
   655	        .balances_t
   656	        .0
   657	        .get(&AgentId("provider".into()))
   658	        .copied()
   659	        .unwrap();
   660	    assert_eq!(
   661	        provider_bal.micro_units(),
   662	        50_i64 * 1_000_000,
   663	        "provider balance MUST be unchanged after negative-seed rejection"
   664	    );
   665	}
   666	
   667	/// Architect-mandated invariant: complete-set balanced post-seed.
   668	#[tokio::test]
   669	async fn halt_complete_set_balanced_post_seed() {
   670	    let q0 = genesis_with_balances(&[("provider", 50)]);
   671	    let mut h = fresh_harness(q0);
   672	    let parent = h.seq.q_snapshot().unwrap().state_root_t;
   673	    submit_and_apply(&mut h, build_seed(parent, "provider", "task-H5", 3_141_592, 26))
   674	        .await
   675	        .expect("seed");
   676	    let q = h.seq.q_snapshot().unwrap();
   677	    assert_complete_set_balanced(&q.economic_state_t).expect("balanced after seed");
   678	    let collateral = q
   679	        .economic_state_t
   680	        .conditional_collateral_t
   681	        .0
   682	        .get(&EventId(TaskId("task-H5".into())))
   683	        .copied()
   684	        .unwrap();
   685	    assert_eq!(collateral.micro_units(), 3_141_592);
   686	    let pair = q
   687	        .economic_state_t
   688	        .conditional_share_balances_t
   689	        .0
   690	        .get(&AgentId("provider".into()))
   691	        .and_then(|m| m.get(&EventId(TaskId("task-H5".into()))))
   692	        .copied()
   693	        .unwrap();
   694	    assert_eq!(pair.yes.units, 3_141_592_u128);
   695	    assert_eq!(pair.no.units, 3_141_592_u128);
   696	}
   697	
   698	// Suppress unused import warnings — the harness types are referenced via
   699	// trait constraints + the helper signatures.
   700	#[allow(dead_code)]

exec
/bin/bash -lc 'git log --oneline --decorate -8' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
07fc869 (HEAD -> main) TB-13 Atom 6 round-2 — Codex VETO remediation (V1+V2+Q9; Class 3)
7aac629 TB-13 Atom 6(a.5) — Real-LLM regression smoke (chaintape audit)
17d4a3b TB-13 Atom 6(a) — Recursive self-audit (Class 3; PASS pending external dual)
1806432 TB-13 Atoms 2+3+5 — Sequencer dispatch + conservation invariant + SG-13.x integration tests (Class 3)
70303af TB-13 Atom 1 — typed_tx schemas (Class 2; +8 unit tests)
32aab27 TB-13 Atom 0 + 0.5 — Charter + legacy f64 CPMM forward-fence (Class 1)
fa36eca (origin/main) TB-12 SHIPPED — Node Exposure Index (Class 3 dual audit; Codex+Gemini PASS post-remediation)
975108d TB-12 Atom 6 ship-gate refinement — 4 SG-12.x strict architect-name alignment

exec
/bin/bash -lc 'git diff --name-only fa36eca..HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
genesis_payload.toml
handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md
handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md
handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md
handover/audits/run_codex_tb_13_ship_audit.sh
handover/audits/run_gemini_tb_13_ship_audit.py
handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md
handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/dashboard.txt
handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json
handover/tracer_bullets/TB-13_charter_2026-05-03.md
src/bottom_white/ledger/transition_ledger.rs
src/economy/monetary_invariant.rs
src/kernel.rs
src/prediction_market.rs
src/runtime/run_summary.rs
src/runtime/verify.rs
src/state/q_state.rs
src/state/sequencer.rs
src/state/typed_tx.rs
tests/economic_state_reconstruct.rs
tests/q_state_reconstruct.rs
tests/six_axioms_alignment.rs
tests/tb_13_complete_set.rs
tests/tb_13_legacy_cpmm_forward_fence.rs

exec
/bin/bash -lc 'git diff --stat fa36eca..HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 genesis_payload.toml                               |   16 +-
 .../OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md         |   69 +
 .../OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md |   36 +
 .../audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md    | 8428 ++++++++++++++++++++
 .../GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md       |  140 +
 .../audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md     |  232 +
 handover/audits/run_codex_tb_13_ship_audit.sh      |  148 +
 handover/audits/run_gemini_tb_13_ship_audit.py     |  166 +
 ...5-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | 1353 ++++
 .../tb_13_real_llm_smoke_2026-05-03/README.md      |  191 +
 .../single_n1_mathd_algebra_171/dashboard.txt      |  123 +
 .../single_n1_mathd_algebra_171/replay_report.json |   21 +
 .../tracer_bullets/TB-13_charter_2026-05-03.md     |  804 ++
 src/bottom_white/ledger/transition_ledger.rs       |   19 +
 src/economy/monetary_invariant.rs                  |  121 +-
 src/kernel.rs                                      |   23 +-
 src/prediction_market.rs                           |   45 +
 src/runtime/run_summary.rs                         |    3 +
 src/runtime/verify.rs                              |   55 +-
 src/state/q_state.rs                               |   86 +-
 src/state/sequencer.rs                             |  385 +-
 src/state/typed_tx.rs                              |  533 ++
 tests/economic_state_reconstruct.rs                |   15 +-
 tests/q_state_reconstruct.rs                       |    6 +-
 tests/six_axioms_alignment.rs                      |    7 +-
 tests/tb_13_complete_set.rs                        |  706 ++
 tests/tb_13_legacy_cpmm_forward_fence.rs           |  337 +
 27 files changed, 14032 insertions(+), 36 deletions(-)

exec
/bin/bash -lc 'cat handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/replay_report.json' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
{
  "l4_entries": 3,
  "l4e_entries": 2,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "agent_signatures_verified": true,
  "proposal_telemetry_cas_retrievable": true,
  "run_id": "tb13-smoke",
  "epoch": 1,
  "detail": {
    "final_state_root_hex": "1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9",
    "final_ledger_root_hex": "93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb",
    "head_commit_oid_hex": "38f1b3957834052aac42169598f92016d756c331",
    "l4e_last_hash_hex": "79325795bf2ebc78a9330c06c173bb0c502ee283fbfa5b46f569551314e9e23a",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": true
  }
}
exec
/bin/bash -lc 'git show --stat --oneline 07fc869' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
07fc869 TB-13 Atom 6 round-2 — Codex VETO remediation (V1+V2+Q9; Class 3)
 genesis_payload.toml                               |    4 +-
 .../OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md         |   69 +
 .../audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md    | 8428 ++++++++++++++++++++
 .../GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md       |  140 +
 handover/audits/run_codex_tb_13_ship_audit.sh      |  148 +
 handover/audits/run_gemini_tb_13_ship_audit.py     |  166 +
 src/runtime/verify.rs                              |   55 +-
 src/state/sequencer.rs                             |   17 +-
 tests/tb_13_complete_set.rs                        |   95 +
 tests/tb_13_legacy_cpmm_forward_fence.rs           |   44 +-
 10 files changed, 9154 insertions(+), 12 deletions(-)

exec
/bin/bash -lc "sed -n '1,220p' handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "sed -n '1,180p' handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md.raw" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019decaa-f2e8-71a0-a4b7-ac881f5c0df4
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
I’ll audit the scoped TB-13 range against the mandated questions and repo instructions first, then check the implementation paths and tests/evidence. The filesystem is read-only in this session, so I can inspect and produce the audit content, but I may not be able to create the requested file directly.
  → Read repo instructions and TB-13 audit artifacts
  • Trace sequencer and invariant implementation
  • Check tests, forward fence, and replay evidence
  • Evaluate Q1-Q9/RQ1-RQ7 and draft verdict
  • Attempt audit file write if permitted
exec
/bin/bash -lc pwd in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc "rg --files -g 'AGENTS.md' -g 'CLAUDE.md' -g 'handover/tracer_bullets/TB-13_charter_2026-05-03.md' -g 'handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md' -g 'handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md' -g 'handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
CLAUDE.md
handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md
handover/evidence/tb_13_real_llm_smoke_2026-05-03/README.md
handover/tracer_bullets/TB-13_charter_2026-05-03.md
handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md

exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 D handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md.raw
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/

exec
/bin/bash -lc 'git rev-parse HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

exec
/bin/bash -lc "git show HEAD:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc 'tail -n 80 handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc 'tail -n 80 handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md.raw' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc 'tail -n 80 handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md' in /home/zephryj/projects/turingosv4

exec
/bin/bash -lc 'git show HEAD:handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md | tail -n 120' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc 'git show --name-only --oneline 07fc869' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
07fc869 TB-13 Atom 6 round-2 — Codex VETO remediation (V1+V2+Q9; Class 3)
genesis_payload.toml
handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md
handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md
handover/audits/run_codex_tb_13_ship_audit.sh
handover/audits/run_gemini_tb_13_ship_audit.py
src/runtime/verify.rs
src/state/sequencer.rs
tests/tb_13_complete_set.rs
tests/tb_13_legacy_cpmm_forward_fence.rs

codex
The checked-out HEAD includes a follow-up remediation commit (`07fc869`) beyond the original Atom 6(a.5) list. I’m treating `fa36eca..HEAD` literally, so I’m auditing the remediated code now rather than repeating the earlier round-1 veto.
exec
/bin/bash -lc 'rg -n "AgentSignature|agent_signatures|verify_agent|signature|CompleteSetMint|CompleteSetRedeem|MarketSeed|signed|pubkey|pubkeys" src/state/sequencer.rs src/runtime/verify.rs genesis_payload.toml tests/tb_13_complete_set.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_13_complete_set.rs:1://! TB-13 Atom 5 integration tests — CompleteSet + MarketSeedTx per architect
tests/tb_13_complete_set.rs:4://! "CompleteSet + MarketSeedTx" — Polymarket / CTF conditional-share
tests/tb_13_complete_set.rs:13://! NOT counted as Coin / MarketSeed without provider balance / no
tests/tb_13_complete_set.rs:55:    AgentSignature, CompleteSetMintTx, CompleteSetRedeemTx, EventId,
tests/tb_13_complete_set.rs:56:    MarketSeedTx, OutcomeSide, ResolutionRef, ShareAmount, TypedTx,
tests/tb_13_complete_set.rs:81:    let pinned_pubkeys = Arc::new(pinned);
tests/tb_13_complete_set.rs:84:        pinned_pubkeys, initial_q, 16,
tests/tb_13_complete_set.rs:129:    TypedTx::CompleteSetMint(CompleteSetMintTx {
tests/tb_13_complete_set.rs:135:        signature: AgentSignature::from_bytes([0u8; 64]),
tests/tb_13_complete_set.rs:148:    TypedTx::CompleteSetRedeem(CompleteSetRedeemTx {
tests/tb_13_complete_set.rs:159:        signature: AgentSignature::from_bytes([0u8; 64]),
tests/tb_13_complete_set.rs:171:    TypedTx::MarketSeed(MarketSeedTx {
tests/tb_13_complete_set.rs:177:        signature: AgentSignature::from_bytes([0u8; 64]),
tests/tb_13_complete_set.rs:273:/// SG-13.3 — MarketSeedTx fails if provider lacks balance.
tests/tb_13_complete_set.rs:292:/// SG-13.4 — MarketSeedTx cannot create liquidity without collateral
tests/tb_13_complete_set.rs:315:/// `RedeemBeforeResolution`. Per architect FR-13.4: "CompleteSetRedeemTx
tests/tb_13_complete_set.rs:451:// SG-13.7 (no f64 in CompleteSet/MarketSeed path) and SG-13.8 (no
tests/tb_13_complete_set.rs:535:/// Halt: MarketSeed with zero-balance provider rejected (regression
tests/tb_13_complete_set.rs:573:/// in CompleteSetMintTx must be rejected. `MicroCoin` is i64-backed and
tests/tb_13_complete_set.rs:583:    let neg_mint = TypedTx::CompleteSetMint(CompleteSetMintTx {
tests/tb_13_complete_set.rs:589:        signature: AgentSignature::from_bytes([0u8; 64]),
tests/tb_13_complete_set.rs:626:/// collateral_amount in MarketSeedTx must be rejected with
tests/tb_13_complete_set.rs:635:    let neg_seed = TypedTx::MarketSeed(MarketSeedTx {
tests/tb_13_complete_set.rs:641:        signature: AgentSignature::from_bytes([0u8; 64]),
tests/tb_13_complete_set.rs:699:// trait constraints + the helper signatures.
genesis_payload.toml:120:creator_signature = "PENDING_USER_PGP_SSH_SIGNATURE_v4_FIRST_ENACTMENT"
genesis_payload.toml:121:signed_at = "2026-04-27T08:00:00+00:00"
genesis_payload.toml:129:# Computed: cat constitution_hash creator_signature signed_at schema_version amendment_predicate_hash initial_predicate_registry_root initial_tool_registry_root | sha256sum
genesis_payload.toml:213:# 2026-05-01 TB-7 Atom 1 — `pub mod agent_keypairs;` declaration added (per-agent Ed25519 keypair manager + on-disk pubkey manifest; run-local identity ONLY; per ARCHITECT_RULING D2 + TB-7 charter §4.2).
genesis_payload.toml:223:# 2026-05-01 TB-6 Atom 4 — replay verifier library (NEW file). Re-opens runtime_repo + cas + pinned_pubkeys.json, replays L4 chain via replay_full_transition, emits ReplayReport. All pub symbols carry /// TRACE_MATRIX FC3-N1 backlinks per R-022.
genesis_payload.toml:224:"src/runtime/verify.rs" = "737c447a85067f9c882d9c644f658a565dd9864b8f588ddd5166925582c6289f"  # rehashed by TB-7 Atom 4 (2026-05-01): added agent_signatures_verified + proposal_telemetry_cas_retrievable indicators (Gate 4 + Gate 5); ReplayReport indicator count 5 → 7; new verify_agent_artifacts helper walks L4 entries, verifies WorkTx + VerifyTx signatures against agent_pubkeys.json + verifies WorkTx.proposal_cid resolves to CAS ProposalTelemetry. all_indicators_pass extended.
genesis_payload.toml:234:"src/runtime/run_summary.rs" = "defc4697ec8af2c050aea3bc33beeaaeff8251d4e89d0a8c48803ffac262c58f"  # rehashed by TB-13 Atom 1 (2026-05-03): extract_tx_id match arm extended for 3 new TB-13 variants (CompleteSetMint / CompleteSetRedeem / MarketSeed). Predecessor TB-11 Atom 1 hash 66b934f8 superseded.
genesis_payload.toml:242:# 2026-05-01 TB-7 Atom 2 — authoritative routing integration tests (NEW file). I100 (real-signature WorkTx verifies via manifest) + I101 (zero-stake WorkTx → L4.E) + I102 (distinct agents → distinct pubkeys). Gate 1 + Gate 4 wired evidence.
genesis_payload.toml:243:"tests/tb_7_authoritative_routing.rs" = "3ac5bd2289c19a3c044e4f758c47014794989ae0c683a62469799220d6296b64"  # rehashed by TB-7 Atom 3 (2026-05-01): added I103 (OMEGA WorkTx + VerifyTx pair) + I104 (VerifyTx signature verifies via manifest).
genesis_payload.toml:248:# 2026-05-01 TB-7 Atom 1 — Per-agent Ed25519 keypair manager + on-disk pubkey manifest (NEW file). Run-local identity ONLY; private keys in process memory only and zeroed on drop. AgentKeypair / AgentKeypairRegistry / AgentPubkeyManifest / verify_agent_signature; mirrors PinnedSystemPubkeys structural pattern. Per ARCHITECT_RULING 2026-05-01 D2 + TB-7 charter §4.2.
genesis_payload.toml:260:"src/economy/monetary_invariant.rs" = "4d1182a620407f5bfa42a7c0fd5e2faed2f2e802bd3bc18acc569eb469dc7151"  # rehashed by TB-13 Atom 1 (2026-05-03; architect 2026-05-03 post-TB-12 ruling Part A §4.3 + CR-13.1..6): assert_no_post_init_mint exhaustive-match extended with 3 TB-13 variants (CompleteSetMint / CompleteSetRedeem / MarketSeed are balance↔collateral migrations only; no mint). Atom 3 will extend assert_total_ctf_conserved with conditional_collateral_t as 6th holding. Predecessor TB-11 Atom 1 hash f9e05d39 superseded.
genesis_payload.toml:278:"src/state/sequencer.rs" = "8fa63603fe487d0eeccaf29279ff46c5c5a8423ca9803f85b5a96bb13124d328"  # rehashed by TB-12 Atom 2 (2026-05-03; architect 2026-05-03 ruling §3 + §8 Atom 2; Class 3): WorkTx accept arm + ChallengeTx accept arm extended with **pure additive** NodePosition side-effect — accepted WorkTx with stake>0 → FirstLong NodePosition write; accepted ChallengeTx with stake>0 → ChallengeShort NodePosition write (task_id Q-derived from target stakes_t entry). VerifyTx accept arm UNCHANGED (FR-12.3 + CR-12.8: VerifyTx.bond is responsibility bond, not market side). NO change to balances_t / stakes_t / challenge_cases_t / total_supply (CR-12.1 + CR-12.2 — NodePosition.amount NOT a Coin holding, NOT in 5-holding CTF sum). Predecessor TB-11 Atom 2 hash 2d861c68 superseded.  # rehashed by TB-11 Atom 1 (2026-05-02; architect §6.2 ruling): system_message_for_verification + system_signature_of + system_epoch_of arms extended for TaskBankruptcy; submit_agent_tx ingress fail-closed extended; dispatch_transition stub arm for TaskBankruptcy (full body lands in Atom 2); test fixture initializers updated for additive TaskExpire/TerminalSummary fields. Predecessor TB-8 round-2 hash b945cc07 superseded.
genesis_payload.toml:345:"src/bottom_white/ledger/transition_ledger.rs" = "66f82c8d58cf0a37f4dc24cc55166f5e311c24817312cc444983edffd61c7191"  # rehashed by TB-13 Atom 1 (2026-05-03; architect 2026-05-03 post-TB-12 ruling Part A §4.3): +TxKind::CompleteSetMint=11 +TxKind::CompleteSetRedeem=12 +TxKind::MarketSeed=13 (additive variants; #[repr(u8)] explicit discriminants; forward-safe wire bump). Predecessor TB-11 Atom 1 hash 4a17875c superseded.
src/runtime/verify.rs:3://! Re-opens a runtime_repo + cas + pinned_pubkeys.json and replays the L4
src/runtime/verify.rs:7://! (Inv 7). Verifies every entry's `system_signature` against the persisted
src/runtime/verify.rs:8://! `pinned_pubkeys.json` manifest.
src/runtime/verify.rs:15://! - `system_signatures_verified`
src/runtime/verify.rs:53:const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
src/runtime/verify.rs:83:                write!(f, "pinned_pubkeys.json not found at {p:?}")
src/runtime/verify.rs:85:            Self::PinnedPubkeysParse(s) => write!(f, "pinned_pubkeys.json parse failed: {s}"),
src/runtime/verify.rs:87:            Self::PubkeyDecode(s) => write!(f, "pubkey hex decode failed: {s}"),
src/runtime/verify.rs:125:    /// True iff every entry's `system_signature` verifies against the
src/runtime/verify.rs:126:    /// persisted pinned-pubkey manifest at the entry's epoch.
src/runtime/verify.rs:127:    pub system_signatures_verified: bool,
src/runtime/verify.rs:139:    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
src/runtime/verify.rs:141:    /// (no agent signatures to verify ≠ failure).
src/runtime/verify.rs:144:    /// signatures verify against agent_pubkeys.json. False on any
src/runtime/verify.rs:145:    /// signature mismatch (tampering, key drift, unknown agent_id).
src/runtime/verify.rs:146:    pub agent_signatures_verified: bool,
src/runtime/verify.rs:156:    /// Run-id from `pinned_pubkeys.json` manifest (echoed for forensics).
src/runtime/verify.rs:158:    /// Epoch from `pinned_pubkeys.json` manifest.
src/runtime/verify.rs:188:    /// **TB-7 Atom 4**: also checks the new `agent_signatures_verified` (Gate 4)
src/runtime/verify.rs:192:            && self.system_signatures_verified
src/runtime/verify.rs:196:            && self.agent_signatures_verified
src/runtime/verify.rs:207:    /// pinned-pubkey manifest's `run_id` matches before replay. None =
src/runtime/verify.rs:213:/// + signature + CAS + L4.E verification. The CLI binary at
src/runtime/verify.rs:217:/// 1. Read `pinned_pubkeys.json` from `runtime_repo_path`. Decode hex
src/runtime/verify.rs:218:///    pubkey(s) into a `PinnedSystemPubkeys` map keyed by `SystemEpoch`.
src/runtime/verify.rs:234:    // Step 1: pinned-pubkey manifest.
src/runtime/verify.rs:251:    for entry in &manifest.pubkeys {
src/runtime/verify.rs:252:        let bytes = decode_pubkey_hex(&entry.pubkey_hex)?;
src/runtime/verify.rs:255:                "expected 32-byte ed25519 pubkey, got {} bytes",
src/runtime/verify.rs:259:        let pubkey = SystemPublicKey::from_bytes(arr);
src/runtime/verify.rs:260:        pinned.insert(SystemEpoch::new(entry.epoch), pubkey);
src/runtime/verify.rs:309:        system_signatures_verified,
src/runtime/verify.rs:330:    // ── TB-7 Atom 4: agent signature verification (Gate 4) ──
src/runtime/verify.rs:333:    // AgentSignature against the per-run agent_pubkeys.json manifest.
src/runtime/verify.rs:335:    // (no agent signatures to fail).
src/runtime/verify.rs:336:    let (agent_signatures_verified, proposal_telemetry_cas_retrievable) =
src/runtime/verify.rs:337:        verify_agent_artifacts(runtime_repo_path, &cas_store, &entries);
src/runtime/verify.rs:343:        system_signatures_verified,
src/runtime/verify.rs:347:        agent_signatures_verified,
src/runtime/verify.rs:364:/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
src/runtime/verify.rs:368:/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
src/runtime/verify.rs:370:/// Work/Verify entries are present (no signatures to verify ≠ failure).
src/runtime/verify.rs:371:fn verify_agent_artifacts(
src/runtime/verify.rs:377:    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
src/runtime/verify.rs:381:    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/verify.rs:383:        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
src/runtime/verify.rs:392:    let mut agent_signatures_verified = true;
src/runtime/verify.rs:408:                // Gate 4 — verify WorkTx signature.
src/runtime/verify.rs:411:                let pubkey_opt = manifest.get(&work.agent_id);
src/runtime/verify.rs:412:                match pubkey_opt {
src/runtime/verify.rs:413:                    None => agent_signatures_verified = false,
src/runtime/verify.rs:414:                    Some(pubkey) => {
src/runtime/verify.rs:415:                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:416:                            agent_signatures_verified = false;
src/runtime/verify.rs:429:                // Gate 4 — verify VerifyTx signature.
src/runtime/verify.rs:432:                let pubkey_opt = manifest.get(&verify.verifier_agent);
src/runtime/verify.rs:433:                match pubkey_opt {
src/runtime/verify.rs:434:                    None => agent_signatures_verified = false,
src/runtime/verify.rs:435:                    Some(pubkey) => {
src/runtime/verify.rs:436:                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:437:                            agent_signatures_verified = false;
src/runtime/verify.rs:444:            // agent-signed TB-13 variants. The submit-time verification
src/runtime/verify.rs:449:            TypedTx::CompleteSetMint(mint) => {
src/runtime/verify.rs:452:                let pubkey_opt = manifest.get(&mint.owner);
src/runtime/verify.rs:453:                match pubkey_opt {
src/runtime/verify.rs:454:                    None => agent_signatures_verified = false,
src/runtime/verify.rs:455:                    Some(pubkey) => {
src/runtime/verify.rs:456:                        if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:457:                            agent_signatures_verified = false;
src/runtime/verify.rs:462:            TypedTx::CompleteSetRedeem(redeem) => {
src/runtime/verify.rs:465:                let pubkey_opt = manifest.get(&redeem.owner);
src/runtime/verify.rs:466:                match pubkey_opt {
src/runtime/verify.rs:467:                    None => agent_signatures_verified = false,
src/runtime/verify.rs:468:                    Some(pubkey) => {
src/runtime/verify.rs:469:                        if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:470:                            agent_signatures_verified = false;
src/runtime/verify.rs:475:            TypedTx::MarketSeed(seed) => {
src/runtime/verify.rs:478:                let pubkey_opt = manifest.get(&seed.provider);
src/runtime/verify.rs:479:                match pubkey_opt {
src/runtime/verify.rs:480:                    None => agent_signatures_verified = false,
src/runtime/verify.rs:481:                    Some(pubkey) => {
src/runtime/verify.rs:482:                        if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
src/runtime/verify.rs:483:                            agent_signatures_verified = false;
src/runtime/verify.rs:492:            // - Some are system-emitted (signature path is system, not agent;
src/runtime/verify.rs:493:            //   covered by system_signatures_verified above).
src/runtime/verify.rs:502:        agent_signatures_verified,
src/runtime/verify.rs:509:fn decode_pubkey_hex(hex: &str) -> Result<Vec<u8>, VerifyError> {
src/runtime/verify.rs:544:        // Stage 4 — signature verification.
src/runtime/verify.rs:567:    fn decode_pubkey_hex_rejects_odd_length() {
src/runtime/verify.rs:569:            decode_pubkey_hex("abc"),
src/runtime/verify.rs:575:    fn decode_pubkey_hex_round_trips_lowercase_hex() {
src/runtime/verify.rs:578:        let decoded = decode_pubkey_hex(&hex).expect("decode");
src/runtime/verify.rs:585:        // addition of `agent_signatures_verified` (Gate 4) and
src/runtime/verify.rs:591:            system_signatures_verified: true,
src/runtime/verify.rs:595:            agent_signatures_verified: true,
src/runtime/verify.rs:609:        r.system_signatures_verified = false;
src/runtime/verify.rs:611:        r.system_signatures_verified = true;
src/runtime/verify.rs:613:        r.agent_signatures_verified = false;
src/runtime/verify.rs:615:        r.agent_signatures_verified = true;
src/state/sequencer.rs:255:/// §4.3): CompleteSetMint-accept state-root domain.
src/state/sequencer.rs:259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
src/state/sequencer.rs:270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
src/state/sequencer.rs:275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
src/state/sequencer.rs:286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
src/state/sequencer.rs:291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
src/state/sequencer.rs:389:        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
src/state/sequencer.rs:404:// system_signature should bind to. Agent variants return `None`. The
src/state/sequencer.rs:409:// `system_signature_of` / `system_epoch_of`: extract the signature + epoch
src/state/sequencer.rs:414:/// a system-emitted `TypedTx` to its `CanonicalMessage` for live signature
src/state/sequencer.rs:416:/// (their signatures are agent-domain `AgentSignature`, verified separately
src/state/sequencer.rs:447:        // CompleteSetMint / CompleteSetRedeem / MarketSeed are agent-signed
src/state/sequencer.rs:448:        // (verified separately at admission via the agent-signature path).
src/state/sequencer.rs:455:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:456:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:457:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:463:fn system_signature_of(
src/state/sequencer.rs:467:        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
src/state/sequencer.rs:468:        TypedTx::TaskExpire(t) => Some(&t.system_signature),
src/state/sequencer.rs:469:        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
src/state/sequencer.rs:470:        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
src/state/sequencer.rs:471:        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
src/state/sequencer.rs:478:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:479:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:480:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:485:/// TypedTx variant for pinned-pubkey lookup. Agent variants → `None`.
src/state/sequencer.rs:490:        // TerminalSummaryTx is signed via opaque digest only (no epoch field
src/state/sequencer.rs:493:        // the pinned pubkey for *some* epoch, we fall back to the signing
src/state/sequencer.rs:506:        | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:507:        | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:508:        | TypedTx::MarketSeed(_) => None,
src/state/sequencer.rs:1158:        // Anti-Oreo: arm fires only when system_signature verified at
src/state/sequencer.rs:1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
src/state/sequencer.rs:1582:        TypedTx::CompleteSetMint(mint) => {
src/state/sequencer.rs:1659:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
src/state/sequencer.rs:1674:        TypedTx::CompleteSetRedeem(redeem) => {
src/state/sequencer.rs:1803:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
src/state/sequencer.rs:1807:        TypedTx::MarketSeed(seed) => {
src/state/sequencer.rs:1813:            // negative (would mirror the V1 attack on CompleteSetMint —
src/state/sequencer.rs:1919:/// Returned by `Sequencer::submit`. Carries `submit_id` (always assigned at
src/state/sequencer.rs:1920:/// submit time) but **NOT** `logical_t` — logical_t is only assigned post-accept
src/state/sequencer.rs:1974:/// typed tx struct is constructed + signed inside emit_system_tx, never by
src/state/sequencer.rs:1976:/// pass a forged signature because they don't construct the typed tx.
src/state/sequencer.rs:2072:    /// Verification of the just-signed signature failed against pinned
src/state/sequencer.rs:2073:    /// pubkeys for the current epoch. Should not happen in production
src/state/sequencer.rs:2074:    /// (tests pin the runtime keypair's pubkey by-construction); defensive
src/state/sequencer.rs:2075:    /// check that catches keypair/pinned-pubkey desync.
src/state/sequencer.rs:2093:                write!(f, "system-tx signature construction failed: {e:?}")
src/state/sequencer.rs:2097:                "system_signature failed live verification against pinned pubkeys at emit time"
src/state/sequencer.rs:2113:/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
src/state/sequencer.rs:2213:    /// K1: assigned at submit; never appears in LedgerEntry.
src/state/sequencer.rs:2217:    /// TB-5 Atom 4 (charter v2 § 4.2 + preflight § 3.5): emit_id is assigned
src/state/sequencer.rs:2240:    /// `system_signature` on system-emitted variants (defense-in-depth atop
src/state/sequencer.rs:2242:    /// `self.keypair`'s pubkey under `epoch` for by-construction verification;
src/state/sequencer.rs:2243:    /// production sources from `genesis_payload.toml [system_pubkeys]`.
src/state/sequencer.rs:2244:    pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
src/state/sequencer.rs:2261:    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
src/state/sequencer.rs:2262:    /// added `pinned_pubkeys` parameter. Existing callers (7 src + tests
src/state/sequencer.rs:2266:    /// `epoch` for by-construction signature-verification correctness.
src/state/sequencer.rs:2276:        pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
src/state/sequencer.rs:2293:            pinned_pubkeys,
src/state/sequencer.rs:2299:    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
src/state/sequencer.rs:2302:    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
src/state/sequencer.rs:2303:        &self.pinned_pubkeys
src/state/sequencer.rs:2344:            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
src/state/sequencer.rs:2345:            // are agent-signed and admit through the same ingress path.
src/state/sequencer.rs:2352:            | TypedTx::CompleteSetMint(_)
src/state/sequencer.rs:2353:            | TypedTx::CompleteSetRedeem(_)
src/state/sequencer.rs:2354:            | TypedTx::MarketSeed(_) => {}
src/state/sequencer.rs:2373:    /// Cannot be invoked with a forged signature because the signature is
src/state/sequencer.rs:2382:        let tx = self.build_signed_system_tx(command)?;
src/state/sequencer.rs:2383:        // Step 2: Defense-in-depth — verify the just-signed signature against
src/state/sequencer.rs:2384:        // pinned pubkeys for the current epoch. Tests pin runtime keypair's
src/state/sequencer.rs:2386:        // Catches keypair/pinned-pubkey desync.
src/state/sequencer.rs:2387:        self.verify_emitted_system_tx_signature(&tx)?;
src/state/sequencer.rs:2407:    /// runtime's system keypair, and returns the signed `TypedTx`.
src/state/sequencer.rs:2408:    fn build_signed_system_tx(
src/state/sequencer.rs:2435:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2442:                tx.system_signature = sig;
src/state/sequencer.rs:2484:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2491:                tx.system_signature = sig;
src/state/sequencer.rs:2528:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2535:                tx.system_signature = sig;
src/state/sequencer.rs:2576:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2583:                tx.system_signature = sig;
src/state/sequencer.rs:2618:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2625:                tx.system_signature = sig;
src/state/sequencer.rs:2631:    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.5): defense-in-depth signature
src/state/sequencer.rs:2632:    /// verification at emit time. Verifies the just-signed signature against
src/state/sequencer.rs:2633:    /// pinned pubkeys for the current epoch.
src/state/sequencer.rs:2634:    fn verify_emitted_system_tx_signature(&self, tx: &TypedTx) -> Result<(), EmitSystemError> {
src/state/sequencer.rs:2635:        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
src/state/sequencer.rs:2640:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2649:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2658:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2671:                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2680:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2748:    /// BOTH dispatch failures (stage 2) AND signature-verification failures
src/state/sequencer.rs:2834:    /// tx would then be assigned a logical_t the writer rejects forever
src/state/sequencer.rs:2854:        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
src/state/sequencer.rs:2857:        // pinned_pubkeys here so that any future bypass of emit_system_tx
src/state/sequencer.rs:2858:        // (or stale signature in a replay) is rejected at the apply boundary.
src/state/sequencer.rs:2863:            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
src/state/sequencer.rs:2864:            let sig = system_signature_of(&tx)
src/state/sequencer.rs:2865:                .expect("system_message_for_verification implies system_signature present");
src/state/sequencer.rs:2870:            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2898:        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
src/state/sequencer.rs:2928:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
src/state/sequencer.rs:2948:            system_signature,
src/state/sequencer.rs:3002:        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
src/state/sequencer.rs:3032:        // TB-5 Atom 4: tests pin keypair's own pubkey under the test epoch
src/state/sequencer.rs:3034:        // verification by-construction succeeds when the pinned pubkey for
src/state/sequencer.rs:3038:        let pinned_pubkeys = Arc::new(pinned);
src/state/sequencer.rs:3047:            pinned_pubkeys,
src/state/sequencer.rs:3077:            signature: AgentSignature::from_bytes([0x77u8; 64]),
src/state/sequencer.rs:3205:        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
src/state/sequencer.rs:3290:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3302:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3352:        // TB-5 Atom 4: pin keypair pubkey under epoch (preflight § 4.2).
src/state/sequencer.rs:3355:        let pinned_pubkeys = Arc::new(pinned);
src/state/sequencer.rs:3364:            pinned_pubkeys,
src/state/sequencer.rs:3400:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3471:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3621:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3751:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3876:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4035:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4059:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4082:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4104:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4132:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4145:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4170:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4183:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4196:    // U27/U28 + I66/I66.a/b/c: forged signatures on system-emitted variants
src/state/sequencer.rs:4206:    /// Helper: forge a ChallengeResolveTx with all-zero signature.
src/state/sequencer.rs:4215:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4229:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4244:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4260:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4266:    fn stage_1_5_rejects_forged_challenge_resolve_signature() {
src/state/sequencer.rs:4286:    fn stage_1_5_rejects_forged_finalize_reward_signature() {
src/state/sequencer.rs:4301:    fn stage_1_5_rejects_forged_task_expire_signature() {
src/state/sequencer.rs:4316:    fn stage_1_5_rejects_forged_terminal_summary_signature() {
src/state/sequencer.rs:4331:    /// pubkey matches the runtime keypair's pubkey under epoch). Atom 5
src/state/sequencer.rs:4335:    /// "stage 1.5 must NOT reject self-signed emit txns" — we assert the
src/state/sequencer.rs:4339:    async fn stage_1_5_accepts_emit_system_tx_self_signed_challenge_resolve() {
src/state/sequencer.rs:4354:                panic!("Self-signed emit_system_tx MUST PASS stage 1.5 verification");
src/state/sequencer.rs:4368:    /// "missing system_signature" errors when an agent variant is applied.
src/state/sequencer.rs:4391:    // arm body from the apply_one + queue + signature pipeline.
src/state/sequencer.rs:4441:            system_signature: SystemSignature::from_bytes([0u8; 64]),

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
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2280,2370p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2280	        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
  2281	        let seq = Self {
  2282	            next_submit_id: AtomicU64::new(1),
  2283	            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
  2284	            next_emit_id: AtomicU64::new(1),    // TB-5 Atom 4: parallel system-emit counter
  2285	            queue_tx,
  2286	            cas,
  2287	            keypair,
  2288	            epoch,
  2289	            ledger_writer,
  2290	            rejection_writer,
  2291	            predicate_registry,
  2292	            tool_registry,
  2293	            pinned_pubkeys,
  2294	            q: RwLock::new(initial_q),
  2295	        };
  2296	        (seq, queue_rx)
  2297	    }
  2298	
  2299	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
  2300	    /// observability; production callers should not depend on this).
  2301	    #[cfg(test)]
  2302	    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
  2303	        &self.pinned_pubkeys
  2304	    }
  2305	
  2306	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek next_emit_id (parallel to
  2307	    /// `next_submit_id_peek` for K1-style observability).
  2308	    pub fn next_emit_id_peek(&self) -> u64 {
  2309	        self.next_emit_id.load(Ordering::SeqCst)
  2310	    }
  2311	
  2312	    /// TRACE_MATRIX FC2-Submit + § 5.2.1: TB-5.0 Atom 2 agent-only ingress
  2313	    /// barrier (charter v2 § 4.2 + § 4.9 + preflight § 3.2; Anti-Oreo Art V.1.3).
  2314	    ///
  2315	    /// Accepts ONLY agent-submitted variants. System-emitted variants
  2316	    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve added
  2317	    /// in Atom 3) are rejected pre-queue with
  2318	    /// `SubmitError::SystemTxForbiddenOnAgentIngress`. This is the
  2319	    /// constitutional Anti-Oreo "agent ≠ direct state writer" boundary,
  2320	    /// structurally enforced (was a documented norm without live enforcement
  2321	    /// through TB-3 + TB-4; TB-5.0 retires that debt for system-tx).
  2322	    ///
  2323	    /// **WP-canonical reconciliation**: ChallengeResolveTx (TB-5 Atom 3) +
  2324	    /// SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx
  2325	    /// (RSP-3.2+ / RSP-4 territory) will be added to the rejection match
  2326	    /// at their respective TB landings — each new system variant extends
  2327	    /// this list, never bypasses it.
  2328	    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
  2329	        // TB-5.0 ingress barrier: reject 4 system-emitted variants
  2330	        // (FinalizeReward / TaskExpire / TerminalSummary added in Atom 2;
  2331	        // ChallengeResolve added in Atom 3 when its TypedTx variant landed).
  2332	        match &tx {
  2333	            TypedTx::FinalizeReward(_)
  2334	            | TypedTx::TaskExpire(_)
  2335	            | TypedTx::TerminalSummary(_)
  2336	            | TypedTx::ChallengeResolve(_)
  2337	            // TB-11 Atom 1 (architect §6.2 ruling 2026-05-02): TaskBankruptcyTx
  2338	            // is system-emitted only; agent ingress must reject pre-queue per
  2339	            // Anti-Oreo (Art V.1.3). Construction goes through emit_system_tx.
  2340	            | TypedTx::TaskBankruptcy(_) => {
  2341	                return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
  2342	            }
  2343	            // Agent-submitted variants — proceed to queue. TB-13 conditional-
  2344	            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
  2345	            // are agent-signed and admit through the same ingress path.
  2346	            TypedTx::Work(_)
  2347	            | TypedTx::Verify(_)
  2348	            | TypedTx::Challenge(_)
  2349	            | TypedTx::Reuse(_)
  2350	            | TypedTx::TaskOpen(_)
  2351	            | TypedTx::EscrowLock(_)
  2352	            | TypedTx::CompleteSetMint(_)
  2353	            | TypedTx::CompleteSetRedeem(_)
  2354	            | TypedTx::MarketSeed(_) => {}
  2355	        }
  2356	        // TB-2 P1-D r1 concurrency contract: fetch_add precedes try_send, so
  2357	        // submit_id allocation order is NOT receiver arrival order under
  2358	        // multi-producer scheduling. submit_id is always burned (never reused)
  2359	        // even when try_send fails — locked by integration test I2.
  2360	        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
  2361	        let envelope = SubmissionEnvelope { submit_id, tx };
  2362	        match self.queue_tx.try_send(envelope) {
  2363	            Ok(()) => Ok(SubmissionReceipt { submit_id }),
  2364	            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => Err(SubmitError::QueueFull),
  2365	            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => Err(SubmitError::QueueClosed),
  2366	        }
  2367	    }
  2368	
  2369	    /// TRACE_MATRIX TB-5 Atom 4 (charter v2 § 4.2 + preflight § 3.3): system-only
  2370	    /// ingress for system-emitted variants. Constructs the typed tx + signs

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-6 Atom 4 — `verify_chaintape` replay verifier (library).
     2	//!
     3	//! Re-opens a runtime_repo + cas + pinned_pubkeys.json and replays the L4
     4	//! chain entry-by-entry through `replay_full_transition` (the I-DETHASH
     5	//! witness from CO1.7-impl A4). Reconstructs `QState` (including
     6	//! `EconomicState`) from L4 alone — L4.E is **evidence-only**, not state-bearing
     7	//! (Inv 7). Verifies every entry's `system_signature` against the persisted
     8	//! `pinned_pubkeys.json` manifest.
     9	//!
    10	//! Architect ruling 2026-05-01 § 3.5 deliverable: `replay_report.json` with the
    11	//! 7 mandated boolean indicators:
    12	//! - `l4_entries`
    13	//! - `l4e_entries`
    14	//! - `ledger_root_verified`
    15	//! - `system_signatures_verified`
    16	//! - `state_reconstructed`
    17	//! - `economic_state_reconstructed`
    18	//! - `cas_payloads_retrievable`
    19	//!
    20	//! Per architect § 3.6 Atom 4 + ruling D2 (1)-(7): chain-backed smoke from
    21	//! TB-6 onward must be replayable. This module is the structural witness.
    22	//!
    23	//! Driven by:
    24	//! - `src/bin/verify_chaintape.rs` — CLI thin wrapper
    25	//! - `tests/tb_6_verify_chaintape.rs` — I90 integration test
    26	//!
    27	//! Initial QState resolution:
    28	//! - If `<runtime_repo>/initial_q_state.json` exists, deserialize it.
    29	//! - Else default to `QState::genesis()` (matches Atom 3 smoke evidence).
    30	//!
    31	//! Bounded by `RejectionEvidenceWriter::open_jsonl` which validates the
    32	//! L4.E `prev_hash → hash` chain on load — tamper any byte of any line and
    33	//! the open call returns `RejectionEvidenceError::ChainBroken { at }`.
    34	
    35	use std::path::{Path, PathBuf};
    36	
    37	use serde::{Deserialize, Serialize};
    38	
    39	use crate::bottom_white::cas::store::CasStore;
    40	use crate::bottom_white::ledger::rejection_evidence::{
    41	    RejectionEvidenceError, RejectionEvidenceWriter,
    42	};
    43	use crate::bottom_white::ledger::system_keypair::{PinnedSystemPubkeys, SystemEpoch, SystemPublicKey};
    44	use crate::bottom_white::ledger::transition_ledger::{
    45	    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter, LedgerWriterError,
    46	    ReplayError,
    47	};
    48	use crate::runtime::{PinnedPubkeyManifest};
    49	use crate::state::q_state::{Hash, QState};
    50	use crate::top_white::predicates::registry::PredicateRegistry;
    51	use crate::bottom_white::tools::registry::ToolRegistry;
    52	
    53	const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
    54	const INITIAL_Q_STATE_FILENAME: &str = "initial_q_state.json";
    55	const REJECTIONS_JSONL_FILENAME: &str = "rejections.jsonl";
    56	
    57	// ── Errors ──────────────────────────────────────────────────────────────────
    58	
    59	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape error class.
    60	///
    61	/// Distinct from `ReplayError`: this covers I/O / config / manifest issues
    62	/// that prevent replay from even starting (vs. mid-chain divergence which is
    63	/// `ReplayError`-shaped).
    64	#[derive(Debug)]
    65	pub enum VerifyError {
    66	    Io(std::io::Error),
    67	    LedgerWriter(LedgerWriterError),
    68	    Cas(String),
    69	    PinnedPubkeysMissing(PathBuf),
    70	    PinnedPubkeysParse(String),
    71	    InitialQStateParse(String),
    72	    PubkeyDecode(String),
    73	    L4eOpen(RejectionEvidenceError),
    74	}
    75	
    76	impl std::fmt::Display for VerifyError {
    77	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    78	        match self {
    79	            Self::Io(e) => write!(f, "io error: {e}"),
    80	            Self::LedgerWriter(e) => write!(f, "ledger writer error: {e}"),
    81	            Self::Cas(e) => write!(f, "cas error: {e}"),
    82	            Self::PinnedPubkeysMissing(p) => {
    83	                write!(f, "pinned_pubkeys.json not found at {p:?}")
    84	            }
    85	            Self::PinnedPubkeysParse(s) => write!(f, "pinned_pubkeys.json parse failed: {s}"),
    86	            Self::InitialQStateParse(s) => write!(f, "initial_q_state.json parse failed: {s}"),
    87	            Self::PubkeyDecode(s) => write!(f, "pubkey hex decode failed: {s}"),
    88	            Self::L4eOpen(e) => write!(f, "rejections.jsonl open / chain-verify failed: {e}"),
    89	        }
    90	    }
    91	}
    92	
    93	impl std::error::Error for VerifyError {}
    94	
    95	impl From<std::io::Error> for VerifyError {
    96	    fn from(e: std::io::Error) -> Self {
    97	        Self::Io(e)
    98	    }
    99	}
   100	
   101	impl From<LedgerWriterError> for VerifyError {
   102	    fn from(e: LedgerWriterError) -> Self {
   103	        Self::LedgerWriter(e)
   104	    }
   105	}
   106	
   107	// ── Report shape (replay_report.json wire format) ───────────────────────────
   108	
   109	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json wire format.
   110	///
   111	/// Stable JSON shape consumed by the smoke evidence dir + CI gates. The 7
   112	/// architect-mandated indicators are top-level fields; richer detail
   113	/// (final state/ledger root hex, classification of any replay error) is
   114	/// captured under `detail` so downstream tooling can drill in without
   115	/// breaking the headline contract.
   116	#[derive(Debug, Clone, Serialize, Deserialize)]
   117	pub struct ReplayReport {
   118	    /// Number of L4 entries (length of `refs/transitions/main`).
   119	    pub l4_entries: u64,
   120	    /// Number of L4.E entries (length of `rejections.jsonl`).
   121	    pub l4e_entries: u64,
   122	    /// True iff every entry's `parent_ledger_root` chains to the previous
   123	    /// `resulting_ledger_root` and the `append()` fold is byte-stable.
   124	    pub ledger_root_verified: bool,
   125	    /// True iff every entry's `system_signature` verifies against the
   126	    /// persisted pinned-pubkey manifest at the entry's epoch.
   127	    pub system_signatures_verified: bool,
   128	    /// True iff replay produced a `QState` (no `dispatch_transition` or
   129	    /// state-root divergence). Empty chain (`l4_entries == 0`) → `true`.
   130	    pub state_reconstructed: bool,
   131	    /// True iff the replayed `QState.economic_state_t` is consistent with
   132	    /// the chain (i.e., replay completed without error). Currently coupled
   133	    /// to `state_reconstructed`; future work may split when economic-only
   134	    /// replay paths are added (NodeMarket, RSP-M).
   135	    pub economic_state_reconstructed: bool,
   136	    /// True iff every L4 entry's `tx_payload_cid` was retrievable from CAS.
   137	    pub cas_payloads_retrievable: bool,
   138	    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx / VerifyTx entry's
   139	    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
   140	    /// manifest. Empty chain or chain with no Work/Verify entries → `true`
   141	    /// (no agent signatures to verify ≠ failure).
   142	    ///
   143	    /// This is the Gate 4 evidence (TB-7 charter §8): all WorkTx
   144	    /// signatures verify against agent_pubkeys.json. False on any
   145	    /// signature mismatch (tampering, key drift, unknown agent_id).
   146	    pub agent_signatures_verified: bool,
   147	    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx entry's
   148	    /// `proposal_cid` resolves to a CAS-resident `ProposalTelemetry`
   149	    /// object. Empty chain or chain with zero Work entries → `true`.
   150	    ///
   151	    /// This is the Gate 5 evidence (TB-7 charter §8): every
   152	    /// `WorkTx.proposal_cid` resolves to a CAS `ProposalTelemetry`
   153	    /// object with the §4.5 schema. False if any WorkTx points to a
   154	    /// CID that's missing or decodes to non-ProposalTelemetry bytes.
   155	    pub proposal_telemetry_cas_retrievable: bool,
   156	    /// Run-id from `pinned_pubkeys.json` manifest (echoed for forensics).
   157	    pub run_id: String,
   158	    /// Epoch from `pinned_pubkeys.json` manifest.
   159	    pub epoch: u64,
   160	    /// Detail block — non-blocking forensic data.
   161	    pub detail: ReplayReportDetail,
   162	}
   163	
   164	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json detail block.
   165	#[derive(Debug, Clone, Serialize, Deserialize)]
   166	pub struct ReplayReportDetail {
   167	    pub final_state_root_hex: Option<String>,
   168	    pub final_ledger_root_hex: Option<String>,
   169	    /// Lowercase 40-char git commit OID at HEAD of `refs/transitions/main`,
   170	    /// or None if chain is empty.
   171	    pub head_commit_oid_hex: Option<String>,
   172	    /// L4.E chain hash at the end of `rejections.jsonl`, or `Hash::ZERO`
   173	    /// hex if empty.
   174	    pub l4e_last_hash_hex: String,
   175	    /// One-line classification of the replay error if replay failed.
   176	    pub replay_failure: Option<String>,
   177	    /// True iff `<runtime_repo>/initial_q_state.json` was found and loaded.
   178	    /// False when the verifier defaulted to `QState::genesis()`.
   179	    pub initial_q_state_loaded_from_disk: bool,
   180	}
   181	
   182	impl ReplayReport {
   183	    /// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — ship-gate aggregator over the 5
   184	    /// architect-mandated boolean indicators. The CLI uses this to drive its
   185	    /// exit code (0 when all pass, 1 otherwise).
   186	    ///
   187	    /// True iff every architect-mandated boolean indicator is `true`.
   188	    /// **TB-7 Atom 4**: also checks the new `agent_signatures_verified` (Gate 4)
   189	    /// and `proposal_telemetry_cas_retrievable` (Gate 5) indicators.
   190	    pub fn all_indicators_pass(&self) -> bool {
   191	        self.ledger_root_verified
   192	            && self.system_signatures_verified
   193	            && self.state_reconstructed
   194	            && self.economic_state_reconstructed
   195	            && self.cas_payloads_retrievable
   196	            && self.agent_signatures_verified
   197	            && self.proposal_telemetry_cas_retrievable
   198	    }
   199	}
   200	
   201	// ── Verifier entry-points ───────────────────────────────────────────────────
   202	
   203	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape options.
   204	#[derive(Debug, Clone, Default)]
   205	pub struct VerifyOptions {
   206	    /// Optional run-id filter; if provided, the verifier asserts the
   207	    /// pinned-pubkey manifest's `run_id` matches before replay. None =
   208	    /// no filter (smoke evidence may legitimately not echo a run-id).
   209	    pub expected_run_id: Option<String>,
   210	}
   211	
   212	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — single library entry-point for replay
   213	/// + signature + CAS + L4.E verification. The CLI binary at
   214	/// `src/bin/verify_chaintape.rs` is a thin wrapper around this.
   215	///
   216	/// Steps (mirrors architect § 3.6 Atom 4):
   217	/// 1. Read `pinned_pubkeys.json` from `runtime_repo_path`. Decode hex
   218	///    pubkey(s) into a `PinnedSystemPubkeys` map keyed by `SystemEpoch`.
   219	/// 2. Resolve initial `QState` from `<runtime_repo>/initial_q_state.json` if
   220	///    present; else `QState::genesis()`.

exec
/bin/bash -lc "git show --stat --patch --find-renames 07fc869 -- src/state/sequencer.rs tests/tb_13_complete_set.rs src/runtime/verify.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit 07fc8698f8d2ad2428b11ce19d31cc2a05db7c12
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 07:08:31 2026 +0000

    TB-13 Atom 6 round-2 — Codex VETO remediation (V1+V2+Q9; Class 3)
    
    External dual audit round-1:
      Gemini : PASS  / high   / PROCEED to SHIP (1 non-blocking CHALLENGE)
      Codex  : VETO  / high   / FIX-THEN-PROCEED (2 VETO findings)
    Per feedback_dual_audit_conflict (VETO > CHALLENGE > PASS): Codex VETO
    wins. Round-2 remediation applied.
    
    V1 — Negative MicroCoin amount rejection (clear bug)
      src/state/sequencer.rs CompleteSetMint.amount: == 0 → <= 0
      src/state/sequencer.rs MarketSeed.collateral_amount: == 0 → <= 0
      Without this gate, a negative mint/seed would credit balance, write
      negative collateral, cast to huge u128 shares (overflow into millions
      of YES + NO claims).
      Tests: halt_negative_mint_amount_rejected +
             halt_negative_market_seed_collateral_rejected
      (verify alice/provider balance unchanged + no collateral written).
    
    V2 — Replay-time agent signature verification for TB-13 variants
      src/runtime/verify.rs Gate 4 extended:
        + CompleteSetMint  → verify against owner's pubkey
        + CompleteSetRedeem → verify against owner's pubkey
        + MarketSeed       → verify against provider's pubkey
      Submit-time / apply-time verification remains a codebase-wide forward
      dep (CO P2.x AgentRegistry territory). OBS-tracked at:
      handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md
      with full threat model + closure plan + per-variant gap matrix.
      TB-13 raises the bar to its 3 Class 3 variants for replay-time
      detection; the broader codebase pass remains future scope.
    
    Q9 — Forward-fence robustness (CHALLENGE-level)
      tests/tb_13_legacy_cpmm_forward_fence.rs:
        Layer 1 (NEW unconditional whole-file scan):
          HARD_BANNED_LEGACY_IMPORTS = [
            "use crate::prediction_market::",
            "use crate::prediction_market;",
            "crate::prediction_market::BinaryMarket",
            "crate::prediction_market::MarketError",
          ]
        Layer 2 (existing TB-13-marker-scoped scan): preserved for trading/AMM
        concept tokens that may legitimately appear in non-TB-13 doc-comments.
      Closes Codex Q9 CHALLENGE: marker-only fence could be bypassed by
      writing `use crate::prediction_market::*` outside a TB-13 doc-comment
      span. Now caught unconditionally.
    
    Codex audit doc preserved at
      handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md
      (raw codex exec transcript + verdict at line 8419; codex sandbox was
       read-only so couldn't write a clean audit doc directly).
    Gemini audit doc:
      handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R1.md (PASS).
    Audit invocation scripts (committed for reproducibility):
      handover/audits/run_gemini_tb_13_ship_audit.py
      handover/audits/run_codex_tb_13_ship_audit.sh
    
    cargo test --workspace = 785/0/150 (TB-12 baseline 759 + 3 fence + 8
    unit + 13 integration + 2 negative-amount halt tests = 785). PASS.
    
    Trust Root rehash for src/state/sequencer.rs + src/runtime/verify.rs.
    
    FC-trace: Art. III.4 (no fake accepted strengthened — replay-time
    forgery detection now covers TB-13 Class 3 surface) + Art. V.1.3
    (Anti-Oreo preserved — V1 closes the negative-amount mint loophole).
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
---
 src/runtime/verify.rs       | 55 ++++++++++++++++++++++++--
 src/state/sequencer.rs      | 17 ++++++--
 tests/tb_13_complete_set.rs | 95 +++++++++++++++++++++++++++++++++++++++++++++
 3 files changed, 159 insertions(+), 8 deletions(-)

diff --git a/src/runtime/verify.rs b/src/runtime/verify.rs
index 4e6b741..a51ee9e 100644
--- a/src/runtime/verify.rs
+++ b/src/runtime/verify.rs
@@ -439,14 +439,61 @@ fn verify_agent_artifacts(
                     }
                 }
             }
-            // Other tx variants (TaskOpen / EscrowLock / Challenge /
+            // TRACE_MATRIX TB-13 Atom 6 round-2 (Codex VETO TB13-V2
+            // remediation 2026-05-03): extend Gate 4 to cover the 3
+            // agent-signed TB-13 variants. The submit-time verification
+            // gap is codebase-wide (also affects Challenge/TaskOpen/
+            // EscrowLock); replay-time coverage is the existing TB-7
+            // ARCHITECT_RULING D3 model. TB-13 raises the bar to its
+            // own three variants because Class 3 money-mover.
+            TypedTx::CompleteSetMint(mint) => {
+                let payload = mint.to_signing_payload();
+                let digest = payload.canonical_digest();
+                let pubkey_opt = manifest.get(&mint.owner);
+                match pubkey_opt {
+                    None => agent_signatures_verified = false,
+                    Some(pubkey) => {
+                        if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
+                            agent_signatures_verified = false;
+                        }
+                    }
+                }
+            }
+            TypedTx::CompleteSetRedeem(redeem) => {
+                let payload = redeem.to_signing_payload();
+                let digest = payload.canonical_digest();
+                let pubkey_opt = manifest.get(&redeem.owner);
+                match pubkey_opt {
+                    None => agent_signatures_verified = false,
+                    Some(pubkey) => {
+                        if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
+                            agent_signatures_verified = false;
+                        }
+                    }
+                }
+            }
+            TypedTx::MarketSeed(seed) => {
+                let payload = seed.to_signing_payload();
+                let digest = payload.canonical_digest();
+                let pubkey_opt = manifest.get(&seed.provider);
+                match pubkey_opt {
+                    None => agent_signatures_verified = false,
+                    Some(pubkey) => {
+                        if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
+                            agent_signatures_verified = false;
+                        }
+                    }
+                }
+            }
+            // Remaining tx variants (TaskOpen / EscrowLock / Challenge /
             // ChallengeResolve / ReuseTx / FinalizeReward / TaskExpire /
-            // TerminalSummary) are not covered by Gate 4 because:
+            // TerminalSummary / TaskBankruptcy) are not covered by Gate 4
+            // because:
             // - Some are system-emitted (signature path is system, not agent;
             //   covered by system_signatures_verified above).
             // - Others are agent-emitted but their signing payloads need
-            //   per-variant signing helpers (TB-7 scope is WorkTx + VerifyTx
-            //   per ARCHITECT_RULING D3 narrowed scope).
+            //   per-variant signing helpers and are deferred to a future
+            //   codebase-wide CO P2.x AgentRegistry pass per `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03`.
             _ => {}
         }
     }
diff --git a/src/state/sequencer.rs b/src/state/sequencer.rs
index 2477d8d..8651b57 100644
--- a/src/state/sequencer.rs
+++ b/src/state/sequencer.rs
@@ -1584,8 +1584,13 @@ pub(crate) fn dispatch_transition(
             if mint.parent_state_root != q.state_root_t {
                 return Err(TransitionError::StaleParent);
             }
-            // Step 2: amount > 0 sanity.
-            if mint.amount.micro_units() == 0 {
+            // Step 2: amount > 0 strictly. `MicroCoin` is i64-backed and
+            // permits negative values at the type level (see
+            // `src/economy/money.rs`); `<= 0` rejects both zero (no-op
+            // mint) and negative (would credit balance + write negative
+            // collateral + cast to huge u128 shares). Codex round-1 VETO
+            // TB13-V1 remediation (2026-05-03).
+            if mint.amount.micro_units() <= 0 {
                 return Err(TransitionError::InsufficientBalanceForMint);
             }
             // Step 3: owner solvency.
@@ -1803,8 +1808,12 @@ pub(crate) fn dispatch_transition(
             if seed.parent_state_root != q.state_root_t {
                 return Err(TransitionError::StaleParent);
             }
-            // Step 1: collateral_amount > 0 (architect SG-13.4).
-            if seed.collateral_amount.micro_units() == 0 {
+            // Step 1: collateral_amount > 0 strictly (architect SG-13.4).
+            // `<= 0` rejects both zero (architect direct mandate) and
+            // negative (would mirror the V1 attack on CompleteSetMint —
+            // negative collateral + huge u128 shares). Codex round-1
+            // VETO TB13-V1 remediation (2026-05-03).
+            if seed.collateral_amount.micro_units() <= 0 {
                 return Err(TransitionError::InsufficientCollateral);
             }
             // Step 2: provider solvency (architect SG-13.3).
diff --git a/tests/tb_13_complete_set.rs b/tests/tb_13_complete_set.rs
index 246d82c..8b749e0 100644
--- a/tests/tb_13_complete_set.rs
+++ b/tests/tb_13_complete_set.rs
@@ -569,6 +569,101 @@ async fn halt_redeem_more_than_owned_rejected() {
     );
 }
 
+/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin` amount
+/// in CompleteSetMintTx must be rejected. `MicroCoin` is i64-backed and
+/// permits negative values at the type layer; the dispatch arm gates
+/// `<= 0` (not just `== 0`). Without this gate, a negative mint would
+/// credit balance + write negative collateral + cast to huge u128 shares.
+#[tokio::test]
+async fn halt_negative_mint_amount_rejected() {
+    let q0 = genesis_with_balances(&[("alice", 100)]);
+    let mut h = fresh_harness(q0);
+    let parent = h.seq.q_snapshot().unwrap().state_root_t;
+
+    let neg_mint = TypedTx::CompleteSetMint(CompleteSetMintTx {
+        tx_id: TxId("neg-mint-fixture".into()),
+        parent_state_root: parent,
+        event_id: EventId(TaskId("task-NEG".into())),
+        owner: AgentId("alice".into()),
+        amount: MicroCoin::from_micro_units(-1_000_000),
+        signature: AgentSignature::from_bytes([0u8; 64]),
+        timestamp_logical: 999,
+    });
+    let err = submit_and_apply(&mut h, neg_mint)
+        .await
+        .expect_err("negative mint must be rejected");
+    assert!(
+        err.contains("InsufficientBalanceForMint"),
+        "expected InsufficientBalanceForMint for negative amount, got: {err}"
+    );
+
+    // Verify alice balance unchanged.
+    let q = h.seq.q_snapshot().unwrap();
+    let alice_bal = q
+        .economic_state_t
+        .balances_t
+        .0
+        .get(&AgentId("alice".into()))
+        .copied()
+        .unwrap();
+    assert_eq!(
+        alice_bal.micro_units(),
+        100_i64 * 1_000_000,
+        "alice balance MUST be unchanged after negative-mint rejection"
+    );
+    // Verify no collateral written.
+    assert!(
+        q.economic_state_t
+            .conditional_collateral_t
+            .0
+            .get(&EventId(TaskId("task-NEG".into())))
+            .is_none(),
+        "no collateral must be written under negative-mint rejection"
+    );
+}
+
+/// Codex round-1 VETO TB13-V1 remediation: negative `MicroCoin`
+/// collateral_amount in MarketSeedTx must be rejected with
+/// `InsufficientCollateral`. Same attack vector as halt_negative_mint
+/// but via the seed path.
+#[tokio::test]
+async fn halt_negative_market_seed_collateral_rejected() {
+    let q0 = genesis_with_balances(&[("provider", 50)]);
+    let mut h = fresh_harness(q0);
+    let parent = h.seq.q_snapshot().unwrap().state_root_t;
+
+    let neg_seed = TypedTx::MarketSeed(MarketSeedTx {
+        tx_id: TxId("neg-seed-fixture".into()),
+        parent_state_root: parent,
+        event_id: EventId(TaskId("task-NEGS".into())),
+        provider: AgentId("provider".into()),
+        collateral_amount: MicroCoin::from_micro_units(-500_000),
+        signature: AgentSignature::from_bytes([0u8; 64]),
+        timestamp_logical: 998,
+    });

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '360,505p'" in /home/zephryj/projects/turingosv4
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
   442	            // TRACE_MATRIX TB-13 Atom 6 round-2 (Codex VETO TB13-V2
   443	            // remediation 2026-05-03): extend Gate 4 to cover the 3
   444	            // agent-signed TB-13 variants. The submit-time verification
   445	            // gap is codebase-wide (also affects Challenge/TaskOpen/
   446	            // EscrowLock); replay-time coverage is the existing TB-7
   447	            // ARCHITECT_RULING D3 model. TB-13 raises the bar to its
   448	            // own three variants because Class 3 money-mover.
   449	            TypedTx::CompleteSetMint(mint) => {
   450	                let payload = mint.to_signing_payload();
   451	                let digest = payload.canonical_digest();
   452	                let pubkey_opt = manifest.get(&mint.owner);
   453	                match pubkey_opt {
   454	                    None => agent_signatures_verified = false,
   455	                    Some(pubkey) => {
   456	                        if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
   457	                            agent_signatures_verified = false;
   458	                        }
   459	                    }
   460	                }
   461	            }
   462	            TypedTx::CompleteSetRedeem(redeem) => {
   463	                let payload = redeem.to_signing_payload();
   464	                let digest = payload.canonical_digest();
   465	                let pubkey_opt = manifest.get(&redeem.owner);
   466	                match pubkey_opt {
   467	                    None => agent_signatures_verified = false,
   468	                    Some(pubkey) => {
   469	                        if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
   470	                            agent_signatures_verified = false;
   471	                        }
   472	                    }
   473	                }
   474	            }
   475	            TypedTx::MarketSeed(seed) => {
   476	                let payload = seed.to_signing_payload();
   477	                let digest = payload.canonical_digest();
   478	                let pubkey_opt = manifest.get(&seed.provider);
   479	                match pubkey_opt {
   480	                    None => agent_signatures_verified = false,
   481	                    Some(pubkey) => {
   482	                        if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
   483	                            agent_signatures_verified = false;
   484	                        }
   485	                    }
   486	                }
   487	            }
   488	            // Remaining tx variants (TaskOpen / EscrowLock / Challenge /
   489	            // ChallengeResolve / ReuseTx / FinalizeReward / TaskExpire /
   490	            // TerminalSummary / TaskBankruptcy) are not covered by Gate 4
   491	            // because:
   492	            // - Some are system-emitted (signature path is system, not agent;
   493	            //   covered by system_signatures_verified above).
   494	            // - Others are agent-emitted but their signing payloads need
   495	            //   per-variant signing helpers and are deferred to a future
   496	            //   codebase-wide CO P2.x AgentRegistry pass per `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03`.
   497	            _ => {}
   498	        }
   499	    }
   500	
   501	    (
   502	        agent_signatures_verified,
   503	        proposal_telemetry_cas_retrievable,
   504	    )
   505	}

exec
/bin/bash -lc "nl -ba handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# OBS — Codebase-wide agent-signature verification gap (forward-dep, CO P2.x AgentRegistry)
     2	
     3	**Date**: 2026-05-03 (TB-13 round-2 audit remediation).
     4	**Status**: OBS (observation; tracked for future codebase-wide pass).
     5	**Triggered by**: Codex TB-13 ship audit round-1 VETO TB13-V2 (`handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`).
     6	
     7	## Summary
     8	
     9	`AgentSignature` fields on agent-submitted typed-tx variants are NOT live-verified at the sequencer ingress (`submit_agent_tx`) NOR at apply time (`apply_one`) for ANY variant. Verification happens ONLY at chain-replay time via `verify_chaintape::Gate 4` (`src/runtime/verify.rs`) — and Gate 4 historically covered only `WorkTx` + `VerifyTx` per TB-7 ARCHITECT_RULING D3 narrowed scope.
    10	
    11	This is a **codebase-wide forward dependency** (typed_tx.rs:813 comment: "actual `verify_agent_signature` ... + agent-pubkey-registry lookup is CO P2.x AgentRegistry territory"). It is NOT a TB-13 regression; TB-13 inherits the existing pattern.
    12	
    13	## TB-13 round-2 partial remediation
    14	
    15	Per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS), TB-13 round-2 raised the bar for its three new agent-signed variants:
    16	
    17	- **Replay-time (Gate 4) coverage extended** (`src/runtime/verify.rs:406-501`):
    18	  - `CompleteSetMint` → verify against owner's pubkey
    19	  - `CompleteSetRedeem` → verify against owner's pubkey
    20	  - `MarketSeed` → verify against provider's pubkey
    21	
    22	- **Submit-time / apply-time coverage**: NOT extended. The sequencer does not currently load an `AgentPubkeyManifest` at construction time (it loads `PinnedSystemPubkeys` only); plumbing agent-pubkey verification into `submit_agent_tx` is a substantial refactor that would mirror the same gap in `Challenge`, `TaskOpen`, `EscrowLock` variants. **This is the OBS**.
    23	
    24	## Agent-signed variants currently unverified at submit/apply time
    25	
    26	| Variant            | Replay-time (Gate 4)                  | Submit-time (`submit_agent_tx`) | Apply-time (`apply_one`) |
    27	| ------------------ | ------------------------------------- | ------------------------------- | ------------------------ |
    28	| `WorkTx`           | ✓ verified (TB-7)                     | ✗ deferred                      | ✗ deferred               |
    29	| `VerifyTx`         | ✓ verified (TB-7)                     | ✗ deferred                      | ✗ deferred               |
    30	| `ChallengeTx`      | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
    31	| `TaskOpenTx`       | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
    32	| `EscrowLockTx`     | ✗ deferred                            | ✗ deferred                      | ✗ deferred               |
    33	| `CompleteSetMintTx` (TB-13)   | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |
    34	| `CompleteSetRedeemTx` (TB-13) | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |
    35	| `MarketSeedTx` (TB-13)        | ✓ verified (TB-13 round-2)  | ✗ deferred                      | ✗ deferred               |
    36	
    37	System-emitted variants (`FinalizeReward`, `TaskExpire`, `TerminalSummary`, `ChallengeResolve`, `TaskBankruptcy`) ARE verified at apply-time via `system_message_for_verification` + `verify_emitted_system_tx_signature`; their auth model is settled.
    38	
    39	## Threat model under current state
    40	
    41	Under the current model, a forged agent signature would:
    42	
    43	1. Pass `submit_agent_tx` (no signature check at admission).
    44	2. Pass `apply_one` stage 1.5 (system-only verification skips agent variants).
    45	3. Be appended to L4 with a state mutation taking effect.
    46	4. Be **caught at replay time** (Gate 4 sets `agent_signatures_verified=false` in `replay_report.json`).
    47	
    48	For Class 3 (money/collateral) variants, this is post-hoc detection — the state has already mutated by the time a replay catches the forgery. Real-time rejection requires submit-time / apply-time verification.
    49	
    50	For TB-13 specifically, the replay-time coverage (this round-2 fix) means a forged TB-13 signature is detectable. Pre-fix, the tx and forgery were both indistinguishable from genuine.
    51	
    52	## Closure plan (CO P2.x AgentRegistry)
    53	
    54	Future codebase-wide pass should:
    55	
    56	1. Plumb `AgentPubkeyManifest` (or successor `AgentRegistry`) into `Sequencer::new` alongside `PinnedSystemPubkeys`.
    57	2. Add submit-time agent-signature verification for ALL agent-signed variants (Work, Verify, Challenge, TaskOpen, EscrowLock, CompleteSetMint, CompleteSetRedeem, MarketSeed).
    58	3. Add `TransitionError::SignatureInvalid` rejection path at submit_agent_tx (variant already exists at `src/state/typed_tx.rs:1688` but is currently dead code — Codex round-1 noted this as defense-in-depth wired but never set).
    59	4. Extend Gate 4 in `verify.rs` to cover Challenge / TaskOpen / EscrowLock / TaskBankruptcy alignment with the submit-time set.
    60	5. Update fixture builders to use real keypair-derived signatures (current TB-3..TB-13 fixtures use `[0u8; 64]` placeholders that would fail real verification).
    61	
    62	## Cross-references
    63	
    64	- Codex TB-13 round-1 audit: `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md` (TB13-V2 VETO finding)
    65	- TB-13 round-2 remediation commits: see RECURSIVE_AUDIT_TB_13_2026-05-03.md §10 (round-2 update)
    66	- typed_tx.rs forward-dep comment: `src/state/typed_tx.rs:813`
    67	- TB-7 ARCHITECT_RULING D3 (narrowed scope): `handover/architect-insights/RULING_TB7_*.md`
    68	- verify.rs Gate 4: `src/runtime/verify.rs:406-501`
    69	- Existing test fixtures using placeholder signatures: `tests/tb_3_*.rs`, `tests/tb_4_*.rs`, ..., `tests/tb_13_complete_set.rs:128`

exec
/bin/bash -lc "nl -ba tests/tb_13_complete_set.rs | sed -n '60,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
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

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2838,2880p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2838	                .write()
  2839	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2840	            writer_w.append_rejected(
  2841	                submit_id,
  2842	                q_snapshot.state_root_t,
  2843	                agent_id,
  2844	                tx.tx_kind(),
  2845	                tx_payload_cid,
  2846	                rejection_class_for(err),
  2847	                raw_diagnostic_cid,
  2848	                public_summary_for(err),
  2849	            );
  2850	        }
  2851	        Ok(())
  2852	    }
  2853	
  2854	    /// TRACE_MATRIX FC3-S3: L4 sequencer per-tx critical section.
  2855	    ///
  2856	    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
  2857	    /// stages 1-9. TB-2 Atom 2 changes the input type from `TypedTx` to
  2858	    /// `SubmissionEnvelope` so `submit_id` travels in (charter §1 / P1:6);
  2859	    /// the apply pipeline itself is unchanged in Atom 2.
  2860	    ///
  2861	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
  2862	    /// **only on commit success** — the original spec § 3 stage-4
  2863	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
  2864	    /// failure (sign / commit) left `next_logical_t` advanced past a
  2865	    /// logical_t that was never written to the ledger. The next accepted
  2866	    /// tx would then be assigned a logical_t the writer rejects forever
  2867	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
  2868	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
  2869	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
  2870	    /// can be upgraded to a `compare_exchange` reservation pattern.
  2871	    pub(crate) fn apply_one(
  2872	        &self,
  2873	        envelope: SubmissionEnvelope,
  2874	    ) -> Result<LedgerEntry, ApplyError> {
  2875	        // TB-2 Atom 2: queue payload is SubmissionEnvelope so submit_id
  2876	        // travels with the tx through to apply_one. Atom 4: submit_id is
  2877	        // now actually used for the L4.E rejection-evidence path below.
  2878	        let SubmissionEnvelope { submit_id, tx } = envelope;
  2879	
  2880	        // Stage 1: snapshot Q_t under read lock.

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2880,2920p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2880	        // Stage 1: snapshot Q_t under read lock.
  2881	        let q_snapshot = {
  2882	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
  2883	            g.clone()
  2884	        };
  2885	
  2886	        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
  2887	        // verification for system-emitted variants. Even though emit_system_tx
  2888	        // signs the message before queueing, apply_one re-verifies against
  2889	        // pinned_pubkeys here so that any future bypass of emit_system_tx
  2890	        // (or stale signature in a replay) is rejected at the apply boundary.
  2891	        // On verification failure, route to L4.E with InvalidSystemSignatureLive
  2892	        // exactly like a dispatch reject — no logical_t consumed, no state_root
  2893	        // advance.
  2894	        if let Some(msg) = system_message_for_verification(&tx) {
  2895	            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
  2896	            let sig = system_signature_of(&tx)
  2897	                .expect("system_message_for_verification implies system_signature present");
  2898	            // TerminalSummaryTx carries no epoch field (STATE § 1.5 8-field
  2899	            // schema is digest-only); fall back to the apply-time sequencer
  2900	            // epoch. Other system variants carry epoch on the wire.
  2901	            let tx_epoch = system_epoch_of(&tx).unwrap_or(self.epoch);
  2902	            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
  2903	                let err = TransitionError::InvalidSystemSignatureLive;
  2904	                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
  2905	                return Err(ApplyError::Transition(err));
  2906	            }
  2907	        }
  2908	
  2909	        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
  2910	        // ledger and return early. K1: no logical_t consumed; Inv 7: no
  2911	        // state_root_t / ledger_root_t advance.
  2912	        let (q_next, _signals) = match dispatch_transition(
  2913	            &q_snapshot,
  2914	            &tx,
  2915	            &self.predicate_registry,
  2916	            &self.tool_registry,
  2917	        ) {
  2918	            Ok(ok) => ok,
  2919	            Err(transition_err) => {
  2920	                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;

exec
/bin/bash -lc 'test -w handover/audits' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "nl -ba src/economy/money.rs | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
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

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '780,830p'" in /home/zephryj/projects/turingosv4
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
   821	const DOMAIN_AGENT_TASK_OPEN: &[u8] = b"turingosv4.agent_sig.task_open.v1";       // TB-3 RSP-1
   822	const DOMAIN_AGENT_ESCROW_LOCK: &[u8] = b"turingosv4.agent_sig.escrow_lock.v1";   // TB-3 RSP-1
   823	const DOMAIN_SYSTEM_FINALIZE_REWARD: &[u8] = b"turingosv4.system_sig.finalize_reward.v1";
   824	const DOMAIN_SYSTEM_TASK_EXPIRE: &[u8] = b"turingosv4.system_sig.task_expire.v1";
   825	const DOMAIN_SYSTEM_TERMINAL_SUMMARY: &[u8] = b"turingosv4.system_sig.terminal_summary.v1";
   826	const DOMAIN_SYSTEM_CHALLENGE_RESOLVE: &[u8] = b"turingosv4.system_sig.challenge_resolve.v1"; // TB-5 Atom 3
   827	const DOMAIN_SYSTEM_TASK_BANKRUPTCY: &[u8] = b"turingosv4.system_sig.task_bankruptcy.v1";    // TB-11
   828	// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
   829	// All three TB-13 typed-tx are AGENT-SIGNED (provider funds explicit; no
   830	// auto-seed; redeem requires system-resolution-reference + outcome match,

exec
/bin/bash -lc "nl -ba src/state/typed_tx.rs | sed -n '828,850p'" in /home/zephryj/projects/turingosv4
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

  ✓ Read repo instructions and TB-13 audit artifacts
  ✓ Trace sequencer and invariant implementation
  ✓ Check tests, forward fence, and replay evidence
  → Evaluate Q1-Q9/RQ1-RQ7 and draft verdict
  • Attempt audit file write if permitted
exec
/bin/bash -lc "nl -ba CLAUDE.md | sed -n '1,35p'" in /home/zephryj/projects/turingosv4
 succeeded in 163ms:
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

exec
/bin/bash -lc 'rg -n "STEP_B|parallel-branch|restricted file|sequencer.rs" handover src tests | head -n 80' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bottom_white/ledger/transition_ledger.rs:10://! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
handover/CHECKPOINT_TB7R_1_2026-05-02.md:30:- Deliverable D (on-chain TaskOpen + EscrowLock) — pending; may trigger STEP_B if it touches `src/state/sequencer.rs`.
src/bin/audit_dashboard.rs:478:                // (mirror of dispatch arm in src/state/sequencer.rs).
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:2:"""Gemini round-1 Phase-0 dual external audit on TB-2 STEP_B preflight
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:5:Strategic / architectural angle: is the proposed sequencer.rs runtime closure
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:36:This is **round-1** Phase-0 dual external audit on the TB-2 STEP_B preflight
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:37:for sequencer.rs runtime closure. Codex is running **round-1** in parallel
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:45:STEP_B_PROTOCOL Phase 0 (necessity audit) gate. You answer 4 questions:
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:48:- Is a less-invasive alternative available? (same effect, no STEP_B-class
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:49:  edit, or a smaller STEP_B-class edit)
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:78:**STEP_B preflight under audit** is `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`.
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:80:- §0: sequencer.rs is "institution per C-031" even though not literally on
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:96:> "The proposed sequencer.rs runtime closure is the minimum-sufficient
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:105:**Q1. STEP_B applicability — is sequencer.rs really institutional / C-031-class?**
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:106:The path `src/state/sequencer.rs` is not literally on CLAUDE.md's restricted
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:107:list. The preflight argues it qualifies under STEP_B's "any proposal that
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:110:`logical_t`). Is this a sound application of STEP_B, or is it stretching
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:111:the protocol? Should sequencer.rs be formally added to CLAUDE.md's restricted
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:112:list (or `STEP_B_PROTOCOL.md` line 3) instead of relying on the C-031
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:219:Section E: Recommendation — proceed to STEP_B Phase-1 / revise preflight /
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:223:the STEP_B preflight, TB-2 charter, the TB-1 ship row, the canonical
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:224:roadmap, and the actual `sequencer.rs` source so you can verify scope
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:241:# Audit target — STEP_B preflight (PRIMARY).
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:243:    "STEP_B preflight (audit target)",
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:261:append_file("STEP_B_PROTOCOL.md", "handover/ai-direct/STEP_B_PROTOCOL.md", fence="markdown")
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:263:# Shipped sequencer.rs (Q2, Q3, Q5, Q7 — verify scope claims).
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:265:    "sequencer.rs (current HEAD; runtime spine under audit)",
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:266:    "src/state/sequencer.rs",
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:352:**Target**: STEP_B preflight `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
handover/audits/run_gemini_tb_2_phase0_audit_2026-04-30.py:356:**Mandate**: STEP_B Phase-0 necessity audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel, implementer-paranoid).
handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md:52:**Q8. STEP_B-protected file violation**
handover/audits/DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md:29:- **Q2 dispatch_transition purity**: WorkTx arm at `src/state/sequencer.rs:158-235` is "100% pure validation pipeline... acquires zero locks, performs zero I/O, makes zero writer/CAS calls" (Gemini Q2). Codex Q3 confirms identical: "no CAS, ledger writer, rejection writer, lock acquisition, or external mutation" between `:159` and `:237`.
handover/audits/DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md:34:- **Q7 deletion-target comment**: present at `src/state/sequencer.rs:205` (post-remediation r1) adjacent to bridge line at `:206`.
handover/audits/DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md:73:- `cargo test --lib boot`: PASS (Trust Root manifest rehashed for sequencer.rs + transition_ledger.rs)
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:26:**Finding**: No. The `CompleteSetMintTx` accept arm in `src/state/sequencer.rs:1805-1839` performs a balanced transfer. It debits `balances_t[owner]` and credits `conditional_collateral_t[event_id]` by the same `amount`. The `assert_total_ctf_conserved` invariant, extended in TB-13 to include `conditional_collateral_t` as a coin holding (`src/economy/monetary_invariant.rs:186-189`), correctly verifies that this is a migration of funds, not a mint or burn. The integration test `sg_13_1_mint_one_coin_yields_one_yes_plus_one_no_total_coin_conserved` (`tests/tb_13_complete_set.rs:88`) explicitly confirms this.
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:30:**Finding**: No. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1840-1923` gates execution on the `task_markets_t` state. The `match` statement on `market_state` (`src/state/sequencer.rs:1852`) explicitly returns `Err(TransitionError::RedeemBeforeResolution)` for `TaskMarketState::Open` and `TaskMarketState::Expired`. Redemption is only possible for `Finalized` or `Bankrupt` states. This is confirmed by the integration test `sg_13_5_redeem_unavailable_before_outcome_resolution` (`tests/tb_13_complete_set.rs:170`).
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:34:**Finding**: No. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1852-1866` contains a nested `match` that strictly enforces consistency between the on-chain `market_state` and the transaction's `redeem.outcome`. A `TaskMarketState::Bankrupt` (NO outcome) paired with an `outcome=Yes` redeem attempt will fall through to the `(_, _)` arm and be rejected with `Err(TransitionError::InvalidResolutionRef)`. The integration test `sg_13_6_redeem_after_yes_outcome_pays_yes_not_no` (`tests/tb_13_complete_set.rs:199`) includes symmetric checks for this mismatch.
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:46:**Finding**: No. The `MarketSeedTx` accept arm in `src/state/sequencer.rs:1924-1961` has two gates. First, it rejects zero or negative collateral with `InsufficientCollateral` (`src/state/sequencer.rs:1933`). Second, it verifies `provider_bal >= seed.collateral_amount` and rejects with `InsufficientBalanceForMint` if the check fails (`src/state/sequencer.rs:1941`). This is confirmed by tests `sg_13_3_market_seed_fails_if_provider_lacks_balance` and `sg_13_4_market_seed_cannot_create_liquidity_without_collateral`.
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:54:**Finding**: No. The `ShareAmount` struct in `src/state/typed_tx.rs:1003` uses `units: u128`, which is non-negative by definition. The `CompleteSetRedeemTx` accept arm in `src/state/sequencer.rs:1879` performs a pre-subtraction check: `if owned_units < redeem.share_amount.units { return Err(TransitionError::RedeemMoreThanOwned); }`. This gate prevents any subtraction that could lead to an underflow. This is confirmed by `halt_redeem_more_than_owned_rejected` (`tests/tb_13_complete_set.rs:327`).
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:74:**Finding**: Yes. The `ResolutionRef` struct (`src/state/typed_tx.rs:1019`) points to a `resolution_tx_id`. The sequencer logic (`src/state/sequencer.rs:1852`) currently hardcodes the accepted resolution transaction types (`TaskBankruptcyTx`, `FinalizeRewardTx`). To support a new resolver (e.g., a ChallengeCourt emitting a `CourtRulingTx`), the `match` statement can be extended. The model of referencing an on-chain, system-emitted fact is fundamentally robust and extensible.
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:80:- **Code**: The `CompleteSetMintTx` accept arm (`src/state/sequencer.rs:1805-1839`) does not check the `task_markets_t` state for the given `event_id`.
handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R2.md:94:**Action Item**: Add a state check in the `CompleteSetMintTx` and `MarketSeedTx` dispatch arms in `src/state/sequencer.rs` to reject transactions if `task_markets_t[event_id.0].state != TaskMarketState::Open`. After this fix is implemented and verified by a new integration test, the ship can proceed.
handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md:80:*   **(Q6.a) STEP_B_PROTOCOL violation**: **PASS**. The diff touches `src/drivers/llm_http.rs`. The restricted file list (`bus.rs`, `kernel.rs`, `wallet.rs`) does not include the `drivers/` directory. No process violation occurred.
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:3:**Target**: STEP_B preflight `handover/ai-direct/TB-2_SEQUENCER_RUNTIME_CLOSURE_2026-04-30.md`
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:7:**Mandate**: STEP_B Phase-0 necessity audit; strategic / architectural / constitutional (Q1-Q8). Independent of Codex r1 (parallel, implementer-paranoid).
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:16:The architectural framing of the STEP_B preflight is exceptionally sound. The decision to keep `dispatch_transition` pure (rejecting naive A) and the strict enforcement of the `exempt_tx_kinds` red line demonstrate a deep understanding of the TuringOS constitutional invariants. However, I am issuing a **CHALLENGE** based on Q7: the proposed 12-test acceptance battery has critical blind spots regarding `StaleParent` rejections, `submit_id` consumption on queue saturation, and the fundamental P1:8 replay invariant. These must be added to the preflight before Phase 1 begins.
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:22:**Q1. STEP_B applicability** — **PASS**
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:23:Relying on the C-031 "institution" catch-all is valid here. `sequencer.rs` is the absolute core of the state transition machine; it dictates what enters the canonical L4 tape and what is banished to L4.E. Changing its queue payload and error-handling paths is a fundamental architectural shift that warrants STEP_B rigor. However, relying on the catch-all is a temporary crutch; `sequencer.rs` should be formally added to `CLAUDE.md`'s restricted list (see P1).
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:67:   **Concrete Remediation**: Add `src/state/sequencer.rs` to the explicit restricted-file list under the "Code Standard" section. Relying on the C-031 "institution" catch-all is valid for this audit, but explicit listing prevents future LLM agents from bypassing STEP_B due to context-window truncation of case law.
handover/audits/GEMINI_TB_2_PHASE0_AUDIT_2026-04-30.md:77:Once the preflight is updated to require a 16/16 deterministic PASS (incorporating the new tests), you are cleared to proceed to STEP_B Phase 1 (branch creation and implementation).
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md:40:`writer_w` is `RwLockWriteGuard<dyn LedgerWriter>` per `src/state/sequencer.rs:201` + `:363-368`. `dyn LedgerWriter` cannot be double-dereferenced. The correct expression is `&*writer_w` (single deref turning the `RwLockWriteGuard<dyn LedgerWriter>` into `&dyn LedgerWriter`).
handover/audits/CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md:53:- **Line 14** (single-sentence summary, top of doc): still says "perform combined STEP_B ceremony adding a Sequencer entry-point on TuringBus + Kernel". MF4 made this single-file (TuringBus only); "combined" + "Kernel" are stale.
handover/audits/CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md:13:## Q4 STEP_B readiness
handover/audits/CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md:17:STEP_B readiness: YES, contingent on implementing the § 2.5 fixture/ABI follow-through at the named atoms.
handover/audits/CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md:25:GO for CO P1 STEP_B / CO1.1.4 and CO1.1.5. Start the shared canonical fixtures before branch divergence, per § 2.5, but this is an implementation gate rather than another spec patch.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:19:-   **Art 0.2 Tape Canonical 公理**: The design upholds this axiom. The `Sequencer::apply_one` pseudocode in the inherited `CO1.7` spec (§ 3) and the shipped `sequencer.rs` (lines 310-315) show that the pure transition function is called *before* any commit action. If it returns an `Err`, the function returns early, and no `LedgerEntry` is committed. This ensures rejected transactions do not advance the ledger state, which holds for all 7 transition bodies as they are all routed through `dispatch_transition`. The `LedgerEntry.system_signature` attests to the sequencer-stamped semantics of the *accepted* transition, fulfilling the attestation requirement.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:49:## Q4 Combined STEP_B ceremony
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:53:The proposal for a combined STEP_B ceremony is strategically sound and defensible.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:57:-   **`STEP_B_PROTOCOL.md` Phase 0 Interpretation**: The spec's invocation of the "minimum sufficient version" criterion is correct and binding. It applies the principle not just to the necessity of the change, but to the *atomicity* of the change being tested. This is a mature interpretation of the protocol's intent.
handover/audits/GEMINI_CO1_7_5_ROUND1_AUDIT_2026-04-29.md:79:-   **Inline Fix Appropriateness**: `CLAUDE.md` and `STEP_B_PROTOCOL.md` are project instructions, not the constitution. The strict "OBS not in-place" rule from the Alignment Standard applies to constitutional documents to preserve their amendment history. For operational documents, keeping them accurate and avoiding confusion for developers and auditors is paramount. Fixing the path drift inline was the correct, pragmatic choice to prevent a guaranteed, low-value CHALLENGE.
handover/audits/run_gemini_tb_12_ship_audit.py:99:    "src/state/sequencer.rs",
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md:13:The v1.1 spec substantively addresses my prior CHALLENGE regarding incomplete pseudocode. The addition of `VerifyTx`, `ChallengeTx`, `ReuseTx`, `finalize_reward`, and `terminal_summary` transition functions provides the necessary logical scaffolding. The explicit classification of hidden inputs (§ 2) and the promotion of environment variables to typed `QState` fields is a significant improvement in rigor and directly responds to Codex's concerns. The spec now presents a coherent, if not yet exhaustive, model for the core work lifecycle. It is sufficiently complete to serve as the binding contract for the STEP_B implementation comparison. Remaining gaps are primarily in lifecycle edge cases (see Q2/Q3), not the core transitions themselves.
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md:80:## Q8. STEP_B feasibility prediction
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md:83:The prediction of surfacing 5-10 spec ambiguities during the parallel STEP_B implementation is realistic and demonstrates mature foresight. No spec of this complexity is perfect. The provided pseudocode, while much improved, still abstracts away details in functions like `materializer::apply` and `SettlementEngine::finalize`. These are the most likely sources of ambiguity.
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md:85:An audit and resolution cost of 5-20 person-days is a reasonable budget for this risk. The spec is sufficiently detailed to make STEP_B a valuable and feasible process. The cost of resolving these ambiguities at the spec/implementation stage is orders of magnitude lower than fixing them post-launch. The plan is sound.
handover/audits/GEMINI_SPEC_FREEZE_AUDIT_2026-04-27.md:96:1.  **Canonical Digests:** The exact byte-level serialization format for all signed transaction types (`WorkTx`, `VerifyTx`, `ChallengeTx`, `TerminalSummaryTx`). This must be specified to the byte to prevent signature validation mismatches between the two STEP_B implementations.
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:158:Generator != Evaluator is not fully preserved for STEP_B.
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:162:- STEP_B says Branch B is Codex implementation (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:65-68`).
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:163:- The same STEP_B lifecycle then says "Codex final review" (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:72-74`).
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:168:Required protocol amendment: for STEP_B atoms where Codex implements any candidate branch, final code review must be Gemini plus either Claude or a fresh Codex invocation with no branch-author context. The model that wrote the winning branch must not be the final reviewer of that branch.
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:175:- STEP_B atoms: ~22 x $5-10 = $110-220 (`:101`)
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:188:- A heavy STEP_B implementation/review with 500k input + 50k output on GPT-5.2 Codex costs `0.500 * 1.75 + 0.050 * 14 = $1.575`. It reaches $5-10 only if repeated tool loops/failed attempts push total billed traffic above roughly 1.5M-3M mixed tokens or if a more expensive pro-class model is used.
handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md:207:The conservative VETO > CHALLENGE > PASS rule is good (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:81-96`), and externalizing disagreement is good (`:145-153`). But STEP_B violates Generator != Evaluator when Codex implements Branch B and then performs final review (`:65-74`, `:159-166`). Cost estimates are also not tied to actual token accounting (`:97-113`). Amend before using it on source-code atoms.
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:3:**Audit type**: post-development self-audit per directive § 4 Q5 narrowed Option A (Codex-only ship gate; round-4 fell back to grep-based self-verification per `c415cd2`). Replaces narrow STEP_B Phase-1c dual external audit. TB-3/TB-4 self-audit precedent extended to TB-5.
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:30:| Q1 | ChallengeResolve = system-emitted; agent forging structurally impossible | § 4.2 + § 4.4 | sequencer.rs `submit_agent_tx` rejects ChallengeResolve pre-queue (TB-5.0 Atom 2); `emit_system_tx` constructs+signs internally (TB-5.0 Atom 4); apply_one stage 1.5 verifies via PinnedSystemPubkeys (Atom 4) | tests/tb_5_system_ingress_barrier.rs I60 + I64 + I65 + I68 + I69 + I67; sequencer.rs::tests U22 + U27 + U28 (stage 1.5 forged-sig rejection × 4 system variants) | ✅ GREEN |
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:31:| Q2 | Two-channel ingress — submit_agent_tx + emit_system_tx; legacy submit aliases agent path | § 4.2 + § 4.9 | sequencer.rs:996 `submit_agent_tx` (4 system variants rejected pre-queue) + sequencer.rs:1129 `emit_system_tx` (system-only) + sequencer.rs:1140 legacy `submit` delegates to submit_agent_tx | tests/tb_5_system_ingress_barrier.rs I67 + I64 + I65; sequencer.rs::tests U26 (6 agent variants accepted by submit_agent_tx) | ✅ GREEN |
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:32:| Q3 | Defense-in-depth signature verification at apply_one stage 1.5 + emit-time | § 4.5 | sequencer.rs `verify_emitted_system_tx_signature` at emit-time + apply_one stage 1.5 (`system_message_for_verification` exhaustive helper for 4 system variants → `verify_system_signature` against pinned_pubkeys → `record_rejection` on failure) | sequencer.rs::tests stage_1_5_rejects_forged_{challenge_resolve,finalize_reward,task_expire,terminal_summary}_signature (4 tests) + stage_1_5_accepts_emit_system_tx_self_signed_challenge_resolve (U27) + stage_1_5_skipped_for_agent_variants (U28) | ✅ GREEN |
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:35:| Q6 | ChallengeStatus single-source-of-truth in q_state.rs (NOT typed_tx.rs) | § 4.4 + § 5.2 | q_state.rs:385-394 `pub enum ChallengeStatus { Open, Released, UpheldDeferred }` + Default impl (Open); typed_tx.rs imports via `crate::state::q_state::ChallengeStatus` (no duplicate def); `ChallengeResolution { Released, UpheldDeferred }` distinct enum in typed_tx.rs (on-wire payload) | sequencer.rs::tests U33 (UpheldDeferred path uses ChallengeStatus::UpheldDeferred) + U29-U31 (Released path uses ChallengeStatus::Released); tests/tb_5_challenge_resolve_surface.rs I75 / I76 / I80 / I81 | ✅ GREEN |
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:57:| 4.2 Two-channel ingress (submit_agent_tx + emit_system_tx) | § 4.2 | sequencer.rs:996 + :1129 + :1140 | tb_5_system_ingress_barrier.rs I60-I69 + sequencer.rs::tests U22-U28 | ✅ GREEN |
handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md:58:| 4.3 InvalidSystemSignatureLive variant + Display arm | § 4.3 | typed_tx.rs TransitionError::InvalidSystemSignatureLive + Display arm (typed_tx.rs:1165-1171) | tb_5_system_ingress_barrier.rs T5 (Display non-empty) + sequencer.rs::tests stage_1_5_rejects_forged_* | ✅ GREEN |

The audit is complete. The requested output file cannot be written because the session is read-only and `handover/audits` is not writable.
  ✓ Read repo instructions and TB-13 audit artifacts
  ✓ Trace sequencer and invariant implementation
  ✓ Check tests, forward fence, and replay evidence
  ✓ Evaluate Q1-Q9/RQ1-RQ7 and draft verdict
  ✓ Attempt audit file write if permitted
codex
I could not save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`: this session is read-only and `handover/audits` is not writable. I also did not rerun `cargo test` for the same reason. Current checked-out `HEAD` is `07fc869`, which includes a round-2 remediation beyond the commits listed in the prompt.

**Audit Result**
Q1/Q4/Q6/Q7 clear for non-forged, non-negative flows: mint/seed now reject `<= 0`, move balance to collateral 1:1, and `total_supply_micro` includes `conditional_collateral_t` while omitting shares ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1593), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1816), [monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:172)). Q2/Q3/RQ1 clear on state/outcome gates ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1678), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1691)). RQ3 smoke evidence shows `economic_state_reconstructed: true`, but it does not exercise TB-13 txs.

Blocking issue remains: TB-13 is still not live agent-authenticated. `submit_agent_tx` admits `CompleteSetMint`, `CompleteSetRedeem`, and `MarketSeed` without signature verification ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2328)); `apply_one` only verifies system signatures and skips agent variants ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2886)); the repo’s own OBS says forged TB-13 signatures mutate state before replay detects them ([OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md](/home/zephryj/projects/turingosv4/handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md:41)). Replay-time detection in `verify.rs` is not an admission control for a Class 3 money/collateral mutator ([verify.rs](/home/zephryj/projects/turingosv4/src/runtime/verify.rs:442)).

Non-blocking challenges: `assert_complete_set_balanced` is test-only, not called by TB-13 dispatch arms ([monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:457); no `src/` callers). `ResolutionRef.resolution_tx_id` is documented as L4-validated, but the sequencer ignores it and uses only `task_markets_t` state ([typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1128), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1683)). The forward fence catches the prompt’s exact top-level import example inside fixed scope, but fixed `FENCE_SCOPE` still misses new unlisted TB-13 files ([tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:51)).

## VERDICT: VETO
- TB13-AUTH VETO: TB-13 Class 3 agent-signed economic txs are accepted/applied without live agent signature verification; forged txs can mutate balances/collateral before replay detects them ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2328), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2886), [OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md](/home/zephryj/projects/turingosv4/handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md:41)).
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)

Conviction: high. Recommendation: FIX-THEN-PROCEED.
tokens used
164,968
I could not save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03.md`: this session is read-only and `handover/audits` is not writable. I also did not rerun `cargo test` for the same reason. Current checked-out `HEAD` is `07fc869`, which includes a round-2 remediation beyond the commits listed in the prompt.

**Audit Result**
Q1/Q4/Q6/Q7 clear for non-forged, non-negative flows: mint/seed now reject `<= 0`, move balance to collateral 1:1, and `total_supply_micro` includes `conditional_collateral_t` while omitting shares ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1593), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1816), [monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:172)). Q2/Q3/RQ1 clear on state/outcome gates ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1678), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1691)). RQ3 smoke evidence shows `economic_state_reconstructed: true`, but it does not exercise TB-13 txs.

Blocking issue remains: TB-13 is still not live agent-authenticated. `submit_agent_tx` admits `CompleteSetMint`, `CompleteSetRedeem`, and `MarketSeed` without signature verification ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2328)); `apply_one` only verifies system signatures and skips agent variants ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2886)); the repo’s own OBS says forged TB-13 signatures mutate state before replay detects them ([OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md](/home/zephryj/projects/turingosv4/handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md:41)). Replay-time detection in `verify.rs` is not an admission control for a Class 3 money/collateral mutator ([verify.rs](/home/zephryj/projects/turingosv4/src/runtime/verify.rs:442)).

Non-blocking challenges: `assert_complete_set_balanced` is test-only, not called by TB-13 dispatch arms ([monetary_invariant.rs](/home/zephryj/projects/turingosv4/src/economy/monetary_invariant.rs:457); no `src/` callers). `ResolutionRef.resolution_tx_id` is documented as L4-validated, but the sequencer ignores it and uses only `task_markets_t` state ([typed_tx.rs](/home/zephryj/projects/turingosv4/src/state/typed_tx.rs:1128), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:1683)). The forward fence catches the prompt’s exact top-level import example inside fixed scope, but fixed `FENCE_SCOPE` still misses new unlisted TB-13 files ([tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:51)).

## VERDICT: VETO
- TB13-AUTH VETO: TB-13 Class 3 agent-signed economic txs are accepted/applied without live agent signature verification; forged txs can mutate balances/collateral before replay detects them ([sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2328), [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:2886), [OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md](/home/zephryj/projects/turingosv4/handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md:41)).
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)

Conviction: high. Recommendation: FIX-THEN-PROCEED.
