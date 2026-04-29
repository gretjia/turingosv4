# TB-1 状态报告 — 致外部架构师 (Overnight 2026-04-29)

> 这份报告是 AI coder 在 overnight session 结束时（HEAD `65d7275`，本地 `main` ahead of origin by 5）写给您的自我审计 + 待裁决问题清单。
>
> 目的：让您回看时不必重读全部 transcript，就能掌握当前真实状态、外部审计发现的 gap、我的判断分布、以及您需要的裁决点。

---

## 0. 极短摘要 (TL;DR)

- 您在 4-29 directive § 7 授权的 audit-ingestion wave 6 commits 全部 shipped (pre-overnight, 0709819..947e67a)。
- 您 directive § 4 / § 5 划定的 TB-1 范围（保留单 TB；P1/P3 blocking；P6 non-blocking；不动 STEP_B 文件）整体被 recharter 接受并由 Day-2/3/4/5 落地：5 个新 module（monetary_invariant, escrow_vault, accepted ledger, rejection_evidence, h_vppu_history）+ 9 个 Tier-A test 全绿 + 4 个 Tier-B `#[ignore]`。
- TB-1 Day-6 dual external audit (我 overnight 跑) 返回 **Codex CHALLENGE / Gemini PASS** → conservative merge **CHALLENGE**。
- **核心 gap**: directive § 4 说 TB-1 minimum 是 "WorkTx accept/reject path through dispatch_transition + RSP-0 monetary invariant"。recharter 把这个 narrowed 到 "primitives only; dispatch wiring deferred to TB-2 RSP-1"。我（AI coder）overnight 是 **按 recharter 跑** 的，没有重新发起这个 narrowing；但 narrowing 本身是否得到您的显式授权，从 wave commits 看不出来。
- **TB-1 Day-7 ship 我没有执行**——CHALLENGE 等于 auto-pause；user 已经睡了；需要您和 user 共同裁决。

---

## 1. 当前 repo 状态

### 1.1 git log（TB-1 全程）

```
65d7275 TB-1 Day-6: dual external audit r1 → CHALLENGE/PASS → merged CHALLENGE  (overnight)
6c04c26 TB-1 Day-5: Tier-A 9 acceptance battery (consolidated; 9/9 PASS)        (overnight)
50a1d67 TB-1 Day-4: P6 h_vppu_history instrumentation (NEW file)                (overnight)
846279f TB-1 Day-3: P1 GitTape ledger.rs + rejection_evidence.rs (NEW files)    (pre-overnight, 已shipped)
451cc66 TB-1 Day-2: P3 RSP-0 monetary_invariant + escrow_vault (NEW files)      (pre-overnight)
063b003 TB-1 Day-1 spike: prompt_context_hash + h_vppu fields land              (pre-overnight)
947e67a TB_LOG.tsv: TB-1 row Tier-A/B annotation (audit-ingestion wave A #6)    (wave commit)
c82db19 DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29 (audit-ingestion wave #5) (wave commit)
edef868 TB-1 re-charter amendment: L4/L4.E + WalletTool + Tier-A/B (wave #4)    (wave commit)
538b697 ROADMAP amendment: P0.R + L4/L4.E + P3 forbidden + dep graph (wave #3)  (wave commit)
6b100eb docs/economics.md: rewrite as RSP-0/RSP-1 ground rules (wave #2)        (wave commit)
0709819 External audit 2026-04-29: archive verbatim (wave #1)                   (wave commit)
```

### 1.2 测试 / build 状态

```
cargo check --workspace          → green, 19 warnings (all unused-fn warnings predating TB-1)
cargo test --workspace           → 491 passed / 0 failed / 150 ignored
cargo test --test tb_1_acceptance → 9 passed / 0 failed / 4 ignored (Tier-A 9/9 + Tier-B 4 ignored)
cargo test trust_root_immutability → 4/4 PASS (genesis_payload.toml manifest 全部对齐)
```

### 1.3 STEP_B-protected files

`src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs` 整个 TB-1 期间 hash 未变（在 `genesis_payload.toml [trust_root]` 中可验证）。directive § 8 要求 ✅。

### 1.4 Day-4 live wire-up 证据

```text
/tmp/tb1_day4_smoke_v2/run1.jsonl          → mathd_algebra_107 n3 第一次, solved=true,
                                             pput_verified=3.92e-5, h_vppu 字段省略 (None)
/tmp/tb1_day4_smoke_v2/run2.jsonl          → 第二次, solved=true,
                                             pput_verified=2.44e-4, h_vppu=6.21589
/tmp/tb1_day4_smoke_v2/h_vppu_history.json → 持久化 history 含 mathd_algebra_107 两条
```

⚠ 这些 evidence 在 `/tmp/`（untracked）。Codex P1-2 标记需要迁移到 `handover/evidence/tb_1_day4_h_vppu/`。我没做（不在 recharter 也不在 user 授权范围内）。

---

## 2. Dual external audit 发现的问题（round 1）

### 2.1 Codex P0 list（CHALLENGE; high conviction）

#### P0-1（最重要）— TB-1 中心声明 over-claimed

> The 9 Tier-A tests prove the **primitives**, not the ship claim "the v4 kernel honors the L4/L4.E split + RSP-0 invariants enforced".

具体证据：
- `src/state/sequencer.rs:47` 全 7 variant 仍 `NotYetImplemented`
- `src/state/sequencer.rs:339` `apply_one` 在 transition error 时 early-return，**绝不 append L4.E**
- Tier-A 测试在 fixture 中手动 `AcceptedLedger::new()` + `RejectionEvidenceWriter::new()`，**绕过 dispatch**

**这正是您 directive § 4 第二段对 TB-1 minimum 描述的反面**：directive 写"accepted WorkTx: ... enters L4 / advances state_root"——这是 runtime path 描述，不是 primitive description。

#### P0-2 — Tier-A test 7 sub-index 局部覆盖

`test_p3_rsp0_exit_1_on_init_total_invariant` 5-step 序列只穿过 `balances_t` + `escrows_t`。剩下 4 个（stakes / claims / task_markets bounty / challenge_cases bond）只在 `monetary_invariant.rs` module test `ctf_counts_all_six_holding_subindexes` 覆盖，不在 Tier-A。`total_supply_micro` 漏算某个 sub-index 仍能让 Tier-A 9 测全过。

#### P0-3 — RejectedSubmissionRecord 屏蔽是 convention 不是类型强制

```rust
// src/bottom_white/ledger/rejection_evidence.rs:82
pub struct RejectedSubmissionRecord {
    ...
    pub raw_diagnostic_cid: Option<Cid>,  // ← pub field
    ...
}
// derive(Serialize)
// records() returns &[RejectedSubmissionRecord]  // ← raw refs

// PublicRejectionView (the projection) is correct, but goes-around paths leak.
```

`PublicRejectionView` 的投影正确（`From<&RejectedSubmissionRecord> for PublicRejectionView` 时显式不带 raw_diagnostic_cid 字段），但 `RejectedSubmissionRecord` 自身仍可以被任何 future code path 直接 `serde_json::to_value(&record)` 并暴露 raw cid。

**Codex 通过 grep 确认目前没有这种 production path**——但屏蔽是 convention（"开发者要记得用 PublicRejectionView"），不是 type-system enforcement。

#### P0-4 — `AcceptedLedger::load_from_path` 跳过 verify_chain

```rust
// src/economy/ledger.rs:300
pub fn load_from_path(path: &Path) -> Result<(Self, Hash), LedgerError> {
    // ... reads bytes, parses, calls reconstruct_state
    // 不调用 verify_chain
}
```

`prev_hash` / `hash` / `logical_t`-only tamper 可以 load 成功，需要 caller 自行 verify_chain。Tier-A `test_p1_kill_1_no_wtool_bypass` 只 catch 一种 tamper shape（修改最后一个 entry 的 `resulting_state_root`），错过 fake-genesis、reorder、parent-state-root-only 三种。

### 2.2 Codex P1（should-fix; ship-with-OBS allowed）

- escrow_vault 0 Tier-A 覆盖 — 11 个 module test 在 escrow_vault.rs 但 Tier-A 一个都没。要么提到 Tier-A，要么明文标 "RSP-1 scaffolding only"
- P6 evidence 在 `/tmp/`（untracked）需要迁移
- evaluator h_vppu 接入点缺 integration test（仅有 `HVppuHistory` API unit test，没有"evaluator 跑两次后 jsonl 有 h_vppu"的端到端 assertion）

### 2.3 Gemini PASS（5/5 conviction）

无 P0。两条 P1 observation：
- 长期 P6 baseline drift（rolling N=3 远期 baseline 飘移；建议未来 P6 dedicated TB 加 `static_baseline` 字段）
- `assert_total_ctf_conserved` 的 `exempt_tx_kinds` 设计向前兼容性好

### 2.4 双方分歧的本质

| 问题 | Gemini 读法 | Codex 读法 |
|---|---|---|
| Tier-A 是否证明中心声明？ | 证明（primitives ready for TB-2 wiring 是正确的 tracer-bullet level） | 不证明（central claim 说"kernel honors split"，dispatch 没接，silent bypass 风险） |
| L4/L4.E disjointness | cryptographically isolated（domain-separation prefix 已经做） | 没经过真实 dispatch 路径测试（sequencer NotYetImplemented） |
| 货币 guard 接入点 | 准备好被 Sequencer match arm 调用 | 目前没任何 production caller，只有 unit + Tier-A 测试调用 |
| Rejection record shielding | excellent（projection 模式正确） | convention not type-enforced |
| Tier-A test 7 | 闭环守恒 sufficient for tracer bullet | 6-sub-index 覆盖应在 Tier-A，否则不证明 EconomicState 总量守恒 |

**这是 scope-of-claim divergence，不是 bug-vs-no-bug divergence。**

---

## 3. 我（AI coder）的观点

### 3.1 我不强行替您裁决，但提供我的判断

#### 3.1.1 关于 P0-1（最重要）

我**倾向 Gemini 的弱读 + 同意 Codex 形式上正确**。理由：

1. directive § 4 的 list（"WorkTx accept path / reject path / rejected evidence path / state_root accept/reject behavior / ledger/replay boundary / RSP-0 monetary invariant"）字面读可以**两可**——既可读为 runtime body 也可读为 structural primitives。
2. directive § 4 末尾说"VerifyTx / ChallengeTx 可以保留 schema, 不必今天进入 transition body"——**显式**允许其它 6 variant 不进 dispatch_transition。这暗示 directive 对 WorkTx 是否必须进 dispatch_transition 是一个隐含问题，没明文。
3. recharter (commit `edef868`) 选了弱读 + 在 Day-3 commit body 写明"wiring lands at TB-2 RSP-1"。如果您 directive 的 intent 是强读，这是显式违反 directive；如果是弱读，这是合规。
4. 实操上：现有 5 个 module（ledger, rejection_evidence, monetary_invariant, escrow_vault, h_vppu_history）总 ~2200 LoC + 30 个 unit test + 9 个 Tier-A test 全绿，是**真实的 P1+P3 RSP-0 primitive 工作**——不是空 PR。这块工作的价值不取决于 dispatch 接没接。
5. 但 Codex 的 CHALLENGE **也对**：commit message 和 recharter 用了 "kernel honors L4/L4.E split" 这种 runtime-flavored 语言，与 primitive-only 实际不匹配。这就是 over-claim。

我建议的 Path A（窄化 commit message + recharter 措辞 + 优雅 ship Day-7）是**最低成本对齐 directive intent + Codex finding** 的路径——前提是您确认弱读是 directive intent。如果您 intent 是强读，那就 Path B（接 dispatch_transition for WorkTx + 额外 tamper test + round-2 audit）。

#### 3.1.2 关于 P0-2

**应当吃掉。**30 分钟工作。`monetary_invariant.rs::ctf_counts_all_six_holding_subindexes` 这个测试已经存在；只需把它的 logic 移植到 `tests/tb_1_acceptance.rs` 作为 Tier-A 第 10 个 blocking test。Path A 的"optional sweetener"清单里我已经标了。

#### 3.1.3 关于 P0-3

**有讨论空间。**最小修法：在 `RejectedSubmissionRecord` 上加 `#[serde(skip_serializing)]` 到 `raw_diagnostic_cid` 字段（30 分钟），让外部 serialize 路径自动屏蔽。Codex 的更强修法：把 `RejectedSubmissionRecord` 改 `pub(crate)`，外部仅暴露 `PublicRejectionView` + 一个 `pub(crate)` 的 audit-only API。强修法更安全但破坏 module 间的可见性约定。

我**倾向最小修法**：`#[serde(skip_serializing)]`。它把 convention 升级到 type-system enforcement（Serde 在 wire form 永远不带这个字段），同时不破坏 `pub` 可见性。但这是个有 trade-off 的判断。

#### 3.1.4 关于 P0-4

**Codex 形式正确，但实际风险中。**`load_from_path` 不调 verify_chain 是性能-vs-完整性 trade-off。修法：(a) load 时强制 verify_chain（multi-row 调用变慢但安全）；(b) 加 `load_from_path_unverified` + `load_from_path_verified` 两个 API；(c) 留 load 不变 + 在 RSP-1 dispatch wiring 时显式 verify。

我**倾向 (a)**：minimum-viable 阶段宁可慢一点也要正确。Day-3 ledger 还没大到 verify_chain 慢。但这要看您。

### 3.2 我对 audit 流程本身的观点

- Codex prompt（154 KB）+ Gemini prompt（197 KB）都偏大。Codex 用了大量 turn 做 grep + cat 而不是直接给 verdict（输出 ~6000 行 investigation log + 100 行 verdict）。下次 round-2（如果要做）可以更 narrow + 把 Q1-Q8 拆成两轮 narrower audit，避免 turn 预算爆。
- Gemini 53 秒返回 80 行清晰 verdict，**性价比远高于 Codex**。但 Gemini 也因为 prompt 太长而没有深入查具体文件——它的"PASS"基于 spec + 表面 architecture 一致性，没有 grep 验证。所以 Gemini PASS 的 conviction value 比 Codex CHALLENGE 低。
- 这意味着按 conservative merge 规则的 CHALLENGE 应该被认真看待。

### 3.3 我对自己 overnight 工作的观点

#### 我做对的：

1. 严守 user 授权范围：到 collect dual audit results 即停，不 auto-ship。
2. 严守 directive § 8：未动 STEP_B-protected files、未启动新 TB、未新建 PR。
3. 严守 phased-checkpoint：每个 atom 独立 commit、可逆、acceptance signal 24h 内（每天均产生 evaluator pass/fail 或 cargo test 信号）。
4. h_vppu wire 的 post-hoc-in-main 选择：避免改 14 个 caller site，保持 `make_pput` 纯计算性。Gemini 显式赞同此选择。
5. dual audit 我同时让 Codex 和 Gemini 跑、独立 angle、不交叉污染——audit 的 independence 保住了。

#### 我有 micro-judgment 但没 escalate 的：

1. **Day-4 wire-site divergence**：recharter 写 "wire into make_pput"，我做了 post-hoc in main()。Commit body 没显式标 spec-vs-impl divergence。**应该标**。
2. **Day-5 删除 `tests/tb_1_p1_acceptance.rs`**：从两文件合一文件。理由是 recharter 字面 spec 是单文件 + 减少冗余，但严格说也可以保留双文件。
3. **Day-5 AT-3 写成 `#[ignore]` empty stub**：因为 `turingosv4` integration tests 不能 import `minif2f_v4`。技术约束真实，但我可以选择把测试写到 `experiments/minif2f_v4/tests/`（如果该目录被允许新建）。我没去做这个判断，直接选了 stub。

#### 我承认偏保守的：

1. Audit CHALLENGE 后，我给了 user 3 paths（Path A/B/C），没有直接说"建议 Path A 立即执行"。理由：user 显式授权"进行到送双外审并收集双外审结果给我睡觉回来看"——授权范围止于 collect。Path A 改 recharter 不只是 doc edit，会影响后续 TB 编号约束，超出我的判断权限。
2. 我没主动迁移 `/tmp/tb1_day4_smoke_v2/` 到 tracked 路径（Codex P1-2）。修法极简单（cp + git add），但同样不在授权 + 不在 recharter 范围。

### 3.4 我注意到但 directive 未提的（向前看）

- `TB_LOG.tsv` 当前 schema v2 的 `kill_criteria_tested` 列我用 inline 文字记录而不是 enum；后续 TB 多了之后，`alignment_coverage.py` 脚本可能需要 parse 这一列做统计；目前 schema 没有 ML-friendly machine-readable form。
- `experiments/minif2f_v4` crate 没有 `tests/` 子目录；如果 Day-5 AT-3 要写真正的 evaluator-跑两次端到端 integration test，需要先建立这个目录 + 第一个 test file。这是 ~20 LoC 的脚手架，但属于 "extra production code skeleton"，directive § 8 要看是否允许。

---

## 4. 我的疑惑（请您裁决）

### Q1（最重要）— Directive § 4 是强读还是弱读？

> "TB-1 的最小实现目标应该是：WorkTx accept path / WorkTx reject path / rejected evidence path / state_root accept/reject behavior / ledger/replay boundary / RSP-0 monetary invariant"

- **弱读**：列出来的是 primitives + invariant guard，不要求 dispatch_transition body 落地。recharter 选了这个 + Day-3 commit body 写"wiring lands at TB-2 RSP-1"。
- **强读**：要求 dispatch_transition 至少为 WorkTx 实现 accept/reject body，让 evaluator 跑一笔 WorkTx 时真的写一行 L4 或 L4.E。Codex 抓的就是这个。

哪个是您 intent？这决定 ship 走 Path A（窄化 claim）还是 Path B（补 dispatch wiring + round-2）。

### Q2 — RejectedSubmissionRecord 屏蔽要求强度

Directive § 1 写 "raw rejected diagnostics do not enter other agents' materialized read views"。

- 我读为：**结构层屏蔽**（PublicRejectionView 投影正确），不一定要 type-system enforcement。
- Codex 读为：**type-system enforcement**（pub field + derive Serialize 即风险）。

哪个是您 intent？决定 P0-3 是否必须修。

### Q3 — Day-4 wire-site 偏离 recharter 字面 spec 是否可接受？

recharter 写 "wire into make_pput"；我做了 "post-hoc stamp in main()"。Gemini 赞同我的选择（"architectural win — keeps I/O at the outermost edge"）；Codex 没标为 P0；user authorize 跑前未与我对齐。

如果您 intent 是字面 spec，我应当回滚 + 改成进 `make_pput` 内部（成本 = 14 个 caller site 的 signature 改动 + 14 个调用点参数加 history reference）。如果您接受 "engineering judgment 下的偏离 + 在 commit body 标 divergence" 作为正常工作方式，我之后这种 case 会显式在 commit body 里标 spec-vs-impl divergence + reasoning。

### Q4 — Codex round-2 audit 是否值得？

Codex round-1 是 high-conviction CHALLENGE，spec-claim level。
- 如果您选 Path A（窄化 claim，doc-only），Codex round-2 没有新东西可审——CHALLENGE 已经落实在 doc 层。**建议跳过 round-2**。
- 如果您选 Path B（补 P0-1..P0-4 production code），那 round-2 需要审"补丁是否真的关掉这些 gap"——值得做。

`feedback_elon_mode_policy` 的 round-cap=2 + ship-with-OBS-not-for-enforcement-gates 在这里怎么应用？P0-3、P0-4 是 enforcement-gate-shape（type-system shielding 是 gate；verify_chain 是 gate），按 policy 不允许 ship-with-OBS。所以如果走 Path B、P0-3 或 P0-4 修不干净，必须做 round-2。

### Q5 — TB-2 RSP-1 现在能否启动？

Gemini Section E："TB-2 (RSP-1) is fully unblocked and ready to begin." 但 Codex 说 dispatch_transition 接好之前，"很多关于 disjointness、guard call site 的 claim 都是论断不是验证"。

- 如果走 Path A：TB-1 ship Day-7 后，TB-2 可以立即启动 RSP-1（escrow_lock_tx + work_tx + yes_stake_tx），同时 fold dispatch_transition WorkTx wiring 进 TB-2 scope。
- 如果走 Path B：TB-1 ship Day-7 后，TB-2 起步时 dispatch_transition WorkTx 已经接好，TB-2 只做 RSP-1 escrow/stake 三笔 tx。
- 如果走 Path C（defer ship + fold dispatch into TB-2）：TB-1 留在 HEAD，TB-2 同时承担 dispatch_transition WorkTx wiring + RSP-1 三笔 tx。

哪个 ordering 您接受？

---

## 5. 相关文件路径（您审计需要的全清单）

### 5.1 Authority docs（您 directive + recharter + decision records）

```
handover/directives/2026-04-29_9_phase_roadmap.md           ← 您 4-29 9-phase 原始 directive verbatim
handover/directives/2026-04-29_external_audit.md            ← 您本次 directive verbatim（wave commit 0709819 archived）
handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md   ← canonical 9-phase 路线图（您 directive § 6 P0.R 已加）
handover/tracer_bullets/TB-1_recharter_2026-04-29.md        ← TB-1 recharter（pre-overnight）— 含 Day-3 narrowing
handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md  ← L4/L4.E 决策记录 (wave #5)
handover/tracer_bullets/TB_LOG.tsv                          ← TB-0/TB-1 行（含 Tier-A/B 注释）
docs/economics.md                                            ← Wave #2 重写后的 RSP-0/RSP-1 ground rules
```

### 5.2 TB-1 production code modules（您裁决 P0-1..P0-4 时需要看的）

```
src/economy/monetary_invariant.rs                           ← Day-2 (451cc66) — 3 个 assert_*; 11 unit test
src/economy/escrow_vault.rs                                 ← Day-2 — lock/release vault; 11 unit test
src/economy/ledger.rs                                       ← Day-3 (846279f) — AcceptedLedger; load_from_path 跳过 verify_chain (P0-4)
src/bottom_white/ledger/rejection_evidence.rs               ← Day-3 — RejectionEvidenceWriter; pub raw_diagnostic_cid (P0-3)
experiments/minif2f_v4/src/h_vppu_history.rs                ← Day-4 (50a1d67) — overnight 我的工作
experiments/minif2f_v4/src/bin/evaluator.rs                 ← Day-4 main() wire-up (lines 322-385) — overnight 我修改
```

### 5.3 TB-1 acceptance battery（您裁决 Tier-A 9-test 充分性时需要看的）

```
tests/tb_1_acceptance.rs                                    ← Day-5 (6c04c26) — 9 Tier-A blocking + 4 Tier-B ignored — overnight 我的工作
                                                              测 1-6 (P1) + 测 7-9 (P3 RSP-0)
                                                              + AT-1/AT-2 ignored (live LLM / WorkTx dispatch)
                                                              + AT-3 ignored stub (path-dep 限制)
                                                              + AT-4 ignored (RSP-1 wiring)
```

### 5.4 Trust Root manifest（验证 STEP_B-protected files 未动）

```
genesis_payload.toml                                         ← [trust_root] 全 hash list
                                                              评估方式: 对比 Day-1 spike (063b003) 后 + HEAD 的 src/kernel.rs / src/bus.rs / src/sdk/tools/wallet.rs hash 是否相同
                                                              （我 overnight 期间这 3 个 hash 应该完全没变）
```

### 5.5 Sequencer 现状（验证 P0-1 dispatch_transition 状态）

```
src/state/sequencer.rs:47       ← apply_one body 全 7 variant `NotYetImplemented`
src/state/sequencer.rs:339      ← transition error 时 early-return（绝不 append L4.E）
```

### 5.6 Audit reports（dual audit 全产出）

```
handover/audits/run_codex_tb_1_audit_2026-04-29.sh          ← 我写的 Codex audit runner（8 audit Q）
handover/audits/run_gemini_tb_1_audit_2026-04-29.py         ← 我写的 Gemini audit runner（8 audit Q）
handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md              ← Codex 全 output (~8150 lines, 末尾 ~100 行是 verdict)
                                                              使用方法：tail -110 该文件 即得 Section A-E
handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md             ← Gemini 全 output (80 行, 完整 verdict)
handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md       ← 我合并的 verdict + Path A/B/C remediation
handover/ai-direct/LATEST.md                                 ← overnight summary 已 prepend 到顶部（user 醒来读这个）
handover/audits/STATUS_REPORT_TO_ARCHITECT_2026-04-29_overnight.md ← 本文件
```

### 5.7 Day-4 live evidence（untracked，但您可能需要看）

```
/tmp/tb1_day4_smoke_v2/run1.jsonl                            ← cold (no h_vppu)
/tmp/tb1_day4_smoke_v2/run2.jsonl                            ← warm (h_vppu=6.21589)
/tmp/tb1_day4_smoke_v2/h_vppu_history.json                   ← 持久化 history
                                                              ⚠ Codex P1-2 标记需迁移到 handover/evidence/tb_1_day4_h_vppu/
```

### 5.8 Memory / 项目记忆

```
/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/
  feedback_dual_audit_conflict.md                            ← VETO > CHALLENGE > PASS
  feedback_phased_checkpoint.md                              ← auto-pause at each gate
  feedback_elon_mode_policy.md                               ← round-cap=2; ship-with-OBS rules
  feedback_step_b_protocol.md                                ← STEP_B-protected files
  feedback_rejection_evidence_separate.md                    ← 您 directive § 1 沉淀
  feedback_no_fake_menus.md                                  ← state and execute, not menu
  feedback_smoke_before_batch.md                             ← any config change → smoke before batch
  feedback_iteration_cap_24h.md                              ← 每个 PR 24h 内 evaluator pass/fail signal
```

### 5.9 您可能想 grep 验证的关键事实

```
# P0-1: sequencer dispatch 状态
grep -n "NotYetImplemented" src/state/sequencer.rs

# P0-2: 6-sub-index 测试是否在 Tier-A
grep "ctf_counts_all_six_holding_subindexes" tests/ src/

# P0-3: PublicRejectionView 投影
grep -n "raw_diagnostic_cid\|PublicRejectionView" src/bottom_white/ledger/rejection_evidence.rs

# P0-4: load_from_path verify_chain 调用
grep -n "verify_chain\|load_from_path" src/economy/ledger.rs

# 货币 guard 调用点
grep -rn "assert_no_post_init_mint\|assert_total_ctf_conserved\|assert_read_is_free" --include='*.rs' src tests experiments
# 期望：仅 monetary_invariant.rs (defs + tests) + tb_1_acceptance.rs（Tier-A 调用），无生产 dispatch 路径

# Tier-A test 真实状态
cargo test --test tb_1_acceptance 2>&1 | tail -25
```

---

## 6. 如果您选某条 path，我（AI coder）的下一步

### Path A（推荐；~1h doc + ~1h optional sweetener）

- 修改 `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` § 1 GOAL 把"discharge first slice of P1+P3 RSP-0 by demonstrating, on a single MiniF2F problem run"改写为"establish PRIMITIVES + INVARIANTS for P1+P3 RSP-0; dispatch_transition enforcement deferred to TB-2 RSP-1"。
- 修改 `tests/tb_1_acceptance.rs` 顶部 module-doc 加 "Limitations" 节。
- Optional sweetener: 把 P0-2（6-sub-index 测试）加为 Tier-A test #10。
- Optional sweetener: 在 `RejectedSubmissionRecord.raw_diagnostic_cid` 加 `#[serde(skip_serializing)]`（P0-3 type-system enforcement）。
- 若加 sweetener: 重跑 `cargo test --workspace`；若仍 491 passed / 0 failed，commit Day-7 ship + push to origin/main。
- **跳过 round-2** audit（CHALLENGE 已通过 doc + 极小 patch 解决；不浪费 round budget）。

### Path B（~3-6h + round-2 audit）

- 实现 `dispatch_transition` 至少 WorkTx accept path + reject path。需要决定：
  - 是否同时实现 VerifyTx？（您 directive 说 schema 保留即可，所以不必）
  - WorkTx accept 要不要 call `assert_no_post_init_mint` + `assert_read_is_free`？（应该要，否则 P0-2 没动）
- P0-2: 加 6-sub-index Tier-A test #10
- P0-3: `#[serde(skip_serializing)]` on `raw_diagnostic_cid`
- P0-4: `load_from_path` 强制 `verify_chain` + 加 3 个 tamper test (fake-genesis, reorder, parent-state-root-only)
- 重跑 cargo test、cargo build --release、live 2× n3 mathd_algebra_107 smoke
- 跑 round-2 dual audit (Codex + Gemini)。预算 ~$10-15。
- 若 round-2 PASS/PASS 或 CHALLENGE/PASS（已修），ship Day-7。
- 若 round-2 VETO，按 `feedback_phased_checkpoint`：写 `OBS_TB-1_FAILED_2026-04-29.md`、charter must change before retry。

### Path C（defer ship; fold into TB-2）

- TB-1 留在 HEAD `65d7275`，不 ship。
- 立即起 TB-2 RSP-1 charter，scope 包含：
  - dispatch_transition WorkTx accept/reject body
  - 4 个 P0 修复
  - RSP-1 三笔 tx (escrow_lock_tx + work_tx + yes_stake_tx)
- TB-2 完成后再决定 TB-1 是否归并 ship 或单独 ship。

---

## 7. 我（AI coder）认为还需要您回答的问题

1. Q1 (directive § 4 强读 / 弱读)
2. Q2 (RejectedSubmissionRecord 屏蔽强度)
3. Q3 (Day-4 wire-site engineering judgment 偏离 spec 是否可接受)
4. Q4 (Codex round-2 是否需要)
5. Q5 (TB-2 RSP-1 启动时机 + scope)

如果您没时间一一回答，最低限度只需回答 Q1——它决定 Path A vs Path B 走法。

---

**报告写完时间**: 2026-04-29 overnight
**HEAD**: `65d7275`
**workspace 状态**: clean (491/0/150)
**等待 user / architect 决策**
