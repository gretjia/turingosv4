# CHECKPOINT — Phase 8 BLOCKER + Critical 盲点修复

**Date**: 2026-04-22
**Branch**: `experiment/phase-8a-snapshot-fix`
**Commits**: `a4d744c` → `6c36bff` → `4eea992` → `3ff4f70`（4 atomic commits）
**Sub-tasks**: 8.A (snapshot) + 8.B (oneshot wtool) + 8.C (OracleReceipt + M-1) + 8.D (decide/omega whitelist) + 8.E (q-halt) + 8.F (reputation) + 8.G (typical-error threshold) = 7 done
**Gate status**: **self-review PASS**，等 Phase 1c 全批双外审 + Phase 2 A/B N=20

---

## § 1. PPUT-first 陈述（C-052 强制）

**本 Checkpoint 不带 PPUT 数值**，因为 Phase 8 只做 **pre-experiment 架构修复**，不涉及 benchmark run。Gate 8 → 9 的 PPUT 测量在 **Phase 9.A 6 seeds × N=50** 做。

Phase 8 的 "验收标准" 是合宪性 + 回归测试通过，不是 PPUT。但 Phase 9 的 PPUT baseline 对比必须用 Phase 8 之后的 binary 跑 — Phase 8 之前的历史 PPUT 数据（`PPUT_RAW_DATA_2026-04-22.md`）将因 C-049 (空 balances) 受污染，需全部重新测量。

---

## § 2. 全量测试状态

```
turingosv4 lib:            116/116 ✅ (+4 oracle_receipt unit from 8.C)
minif2f_v4 lib:             16/16 ✅ (+6 whitelist + 1 replaced from 8.D)
snapshot_nonempty:           6/6  ✅ (8.A, +2 after CHALLENGE fix)
oracle_receipt_bus:          5/5  ✅ (8.C new)
rejection_threshold:         5/5  ✅ (8.G new, deterministic via BusConfig)
reputation:                  7/7  ✅ (8.F new)
q_halt_state:                6/6  ✅ (8.E new)
reward_pull_conservation:    3/5  ⚠️  pre-existing bugs (Phase 8.Z)
```

**Net**: 41 new tests green on experiment branch. 2 pre-existing failures confirmed not introduced by this branch（验证方式：`cd /home/zephryj/projects/turingosv4 && cargo test --test reward_pull_conservation` on main = 3/5，同一失败集）。

---

## § 3. 宪法合规矩阵

| 条款 | Phase 7 状态 | Phase 8 状态 | 修复 sub-task | 判例 |
|---|---|---|---|---|
| Art. I.1 布尔谓词 | 🟡 部分 | ✅ 完整 | 8.D | C-050 |
| Art. I.1.1 PCP 完整 | ✅ | ✅ | — | — |
| Art. I.2 效用 PPUT | ✅ | ✅ | — | C-052 |
| **Art. I.2 信誉累积** | 🔴 缺失 | ✅ **NEW** | 8.F | **C-053** |
| Art. I.2 共识提取 | 🟡 部分 | 🟡 部分（Phase 10 C-054） | — | C-054 |
| **Art. II.1 广播典型错误（阈值）** | 🟡 | ✅ **NEW** | 8.G | **C-055** |
| Art. II.2 价格信号 | 🔴 **失真** | ✅ **NEW**（修 C-049）| 8.A | **C-049** |
| Art. II.2.1 探索/利用 metric | 🟡 env var | 🟡 env var | — | C-057（Phase 10）|
| Art. III.1 园丁 GC | 🔴 | 🔴（**deferred to Phase 10**）| — | **C-045** |
| Art. III.2 按需加载 | ✅ | ✅ | — | — |
| Art. III.3 相关性屏蔽 | 🟡 | 🟡（Phase 9 新增 pairwise_diversity metric）| — | C-059（Phase 9）|
| Art. III.4 Goodhart 保密 | ✅ | ✅（开源退化已立 C-051）| — | C-051/C-060（Phase 10）|
| **Art. IV Q_t 三元组**（q_t 状态机）| 🔴 隐式 | ✅ **NEW** | 8.E | **C-061** |
| **Art. IV ∏p→wtool 全路径** | 🟡 oneshot 绕过 | ✅ **NEW** | 8.B | **C-048** |
| **Art. IV wtool capability safety** | 🔴 public leak | ✅ **NEW** | 8.C | **C-067** |
| Art. V.1.1 宪法只读 | 🔴 | 🔴（**deferred Phase 10**）| — | **C-046** |
| Art. V.1.2 ArchitectAI | 🔴 空壳 | 🔴（deferred Phase 10）| — | **C-044** |
| Art. V.1.3 JudgeAI | 🔴 空壳 | 🔴（deferred Phase 10）| — | **C-044** |
| Law 2 CTF 守恒 | 🟡 测试缺 | 🟡（Phase 9.B proptest 10K tx）| — | — |

**Phase 8 净收益**：**7 个 🔴/🟡 → ✅**（4 个 Critical，3 个 High）。
**仍 🔴/🟡**：5 项 deferred 到 Phase 10（Art. III.1 园丁 / Art. V 三权分立 / 宪法只读 / III.4 开源 / 探索 metric）。

---

## § 4. 新立判例（10 条）

```
cases/
├── C-044 Meta 架构未实现 [high, deferred]
├── C-045 园丁缺失 [medium, deferred]
├── C-046 宪法文件未保护 [high, deferred]
├── C-048 oneshot 绕过 wtool [critical, FIXED 8.B]
├── C-049 snapshot 空 balances [critical, FIXED 8.A]
├── C-050 C-011 部分执行 [critical, FIXED 8.D]
├── C-053 信誉累积缺失 [critical, FIXED 8.F]
├── C-055 典型错误阈值 [high, FIXED 8.G]
├── C-061 q-halt 状态机 [critical, FIXED 8.E]
└── C-067 OracleReceipt capability [critical, FIXED 8.C]
```

每条判例均引用具体 commit 或"Phase 10 deferred"。

---

## § 5. 7 红线自查

| # | 红线 | Phase 7 | Phase 8 | 备注 |
|---|---|---|---|---|
| 1 | Post-genesis mint | ✅ | ✅ | Law 2 守恒（C-001）未触动 |
| 2 | Exit settlement | ✅ | ✅ | halt_and_settle 未改语义，仅加 halt_with_reason 前置 |
| 3 | Raw CoT to tape | ✅ | ✅ | payload 语义未变 |
| 4 | Prompt manipulation | ✅ | ✅ | CHALLENGE-B addressment 未动 prompt |
| 5 | Env-var reward curve | 🟡 | 🟡 | γ/β/θ 仍 env，Phase 10 W-B.1 固化 |
| 6 | ∏p re-verifiable | ✅ | ✅ | OracleReceipt sha256 binding **强化**了此保证 |
| 7 | Deferral | ✅ | ✅ | 所有延迟项明确列在本 checkpoint § 3 |

**全绿或黄（#5 延续黄）**。无新红。

---

## § 6. 对 Phase 9 / Paper baseline 的影响

### 重要后果：历史 PPUT 数据污染

`PPUT_RAW_DATA_2026-04-22.md` 是基于 Phase 7 及之前的 jsonl 产生的。由于：

- **C-049**（snapshot 空 balances）所有 TAPE_ECONOMY / Hayek bounty 结论不可信
- **C-048**（oneshot 绕过 wtool）所有 oneshot batch 未经 ledger / WAL，无可重放证据
- **C-050**（bare decide/omega 未禁）部分 solve 可能是 brute-force（需 Phase 9 重跑 + 重审）

→ **Phase 9 必须在 Phase 8 post-commit 重新测量 baseline**，不可用旧数据比较。

### Phase 9 Gate 新判据（PLAN_FINAL 已修订）

```
主判据: Mean PPUT (solved-only) Wilson 95% CI 下界 ≥ 5.0
辅助必过:
  Σdepth≥10 PPUT > 0.5 且 depth≥10 solves ≥ 2
  pairwise_payload_diversity_mean ≥ 0.25
  reputation p50 > 0   ← 8.F 使这可测
  halt_reason_distribution 公开报告   ← 8.E 使这可测
  Law 2 proptest 10K tx 全绿
```

两个"可测"项是 Phase 8 直接赋能的（之前没有 API 可查）。

---

## § 7. Phase 8.Z 独立子任务（reward_pull_conservation 2 fail）

以下 2 个 test 在 main 上就失败，与 Phase 8 无关但应该修：
- `phase2_conservation_total_coins_bounded` — "expected 50 Coin payout (5·γ·lp); got 0"
- `phase2_settle_pays_out_on_golden_path`

假说：这两个测试依赖特定 env var（`TAPE_ECONOMY_V2` / `HAYEK_BOUNTY` / `FOUNDER_GRANT_GAMMA`），但未在测试内 set。验证方法：读测试用例 + 对照 `bus.rs:halt_and_settle` 的 env gating。

**不阻塞 Gate 8→9**。放入 Phase 9.B "Law 2 proptest 扩展" 一起修（proptest 会一并覆盖这些路径）。

---

## § 8. Gate 8 → 9 必过清单

本 Checkpoint 自审通过 8/10。剩 2 项待外审：

- [x] 4 BLOCKER (8.A/B/C/D) + 3 Critical (8.E/F/G) 全实装
- [x] M-1 Predicate trait 预留（泛化用）
- [x] 41 new regression tests，全绿
- [x] 7 红线自查
- [x] 10 judicial cases 立档（C-044~C-067 子集）
- [x] CHECKPOINT_PHASE_8 成文（本文件）
- [x] Report Standard 新指标可测（reputation / halt_reason）
- [ ] **Codex + Gemini 双外审 4 commits 整批 diff**（STEP_B Phase 1c）
- [ ] **A/B N=20 PPUT 不降 >10%**（STEP_B Phase 2，在 Phase 9 基础设施就绪后跑）

---

## § 9. Budget 追踪

Phase 8 实际消耗：
- 单 Claude Code session 内完成，无外部 LLM runtime cost
- 两次 Gemini API（8.A 单独审，CHALLENGE addressment 后未再审）：~$2
- 两次 codex:codex-rescue subagent（8.A 初次 + 二次）：~$5
- 总计 **~$7**，远低于 PLAN_FINAL § 8 Phase 8 预算 $200

Gate 8 → 9 外审 + A/B 预估：$50-80（双审 × 4 commits diff + N=20 batch）。
累计到目前：$7 + $50 = ~$57 / $2000 硬顶。

---

## § 10. 下一步

按 PLAN_FINAL § 4 Gate 8 要求，**缺最后 2 项外审 + A/B**：

1. **Codex + Gemini 双审整批 diff**（4 commits 合并 diff）
2. 只有双审全 PASS 或 CHALLENGE-addressed 后进入 Phase 2 A/B
3. A/B：main vs experiment 各跑 N=20 MiniF2F，对比 Mean PPUT 是否降 >10%
4. 全部 PASS → merge 到 main → Phase 9 开跑

---

## § 11. 文件索引

```
.claude/worktrees/phase-8a-snapshot/
├── commit a4d744c (8.A snapshot)
│   ├── src/bus.rs:567-583 (snapshot enumerate)
│   └── tests/snapshot_nonempty.rs (6 tests)
├── commit 6c36bff (8.C receipt)
│   ├── src/sdk/predicate.rs (NEW, M-1)
│   ├── src/sdk/oracle_receipt.rs (NEW, 4 unit tests)
│   ├── src/sdk/mod.rs (export 2 modules)
│   ├── src/bus.rs:174-193 (signature changed)
│   ├── experiments/minif2f_v4/src/bin/evaluator.rs (3 call sites)
│   └── tests/oracle_receipt_bus.rs (5 tests)
├── commit 4eea992 (8.B + 8.D)
│   ├── experiments/minif2f_v4/src/bin/evaluator.rs (oneshot bus wtool)
│   └── experiments/minif2f_v4/src/lean4_oracle.rs (bare tactic whitelist + 6+2 tests)
└── commit 3ff4f70 (8.E + 8.F + 8.G)
    ├── src/ledger.rs (HaltReason + EventType::Halt + Tape.reputation)
    ├── src/bus.rs (QState + halt_with_reason + BusConfig.min_count)
    ├── src/sdk/snapshot.rs (reputation field)
    ├── experiments/minif2f_v4/src/bin/evaluator.rs (BusConfig migration)
    ├── tests/q_halt_state.rs (6 tests)
    ├── tests/reputation.rs (7 tests)
    └── tests/rejection_threshold.rs (5 tests)

cases/ (10 new)
├── C-044 Meta 架构 [deferred]
├── C-045 园丁 [deferred]
├── C-046 宪法只读 [deferred]
├── C-048 oneshot wtool [FIXED 8.B]
├── C-049 snapshot balances [FIXED 8.A]
├── C-050 C-011 partial [FIXED 8.D]
├── C-053 reputation [FIXED 8.F]
├── C-055 typical error threshold [FIXED 8.G]
├── C-061 q-halt state [FIXED 8.E]
└── C-067 OracleReceipt capability [FIXED 8.C]

handover/ai-direct/CHECKPOINT_PHASE_8_2026-04-22.md (本文件)
```
