# Validation & Audit Plan — AgentOutputEnvelope Research

For: `handover/research/AGENT_OUTPUT_ENVELOPE_RESEARCH_2026-05-26.md`
Worktree HEAD: 03a84470 (main)
Status: PoC complete — 24/24 tests GREEN; self-audit & Codex dispatch packet ready

---

## 0. 范围与不做什么

研究文档列了 7 条核心断言 (§0 TL;DR + §6 表格 + §8 红线)。本计划把每条断言
拆成 **(验证方法，可执行命令，期望输出，已完成/待跑)** 四元组。

不做什么:
- 不把 PoC 接入 mainline，不动 src/ 任何文件，不触发 sequencer/typed_tx。
- 不跑 `cargo test --workspace`（PoC subcrate 独立 workspace，只跑自己）。
- 不派 Codex 审计（packet 已写，用户决定是否派）。

---

## 1. 七条断言 ↔ 验证矩阵

| Aid | 断言 | 验证方法 | 命令/位点 | 期望输出 | 状态 |
|---|---|---|---|---|---|
| A1 | 结构闸门与 predicate 闸门解耦：envelope OK 不蕴含 predicate PASS；envelope FAIL 短路 predicate | 单元测试 + 反例双侧 | `research/envelope_poc/tests/decoupling.rs::envelope_pass_does_not_imply_predicate_pass`, `::envelope_fail_short_circuits_predicate` | 4 tests pass | ✅ DONE |
| A2 | 不动 sequencer / typed_tx / cas-schema | `git diff --name-only HEAD` + 路径 grep | `git diff` + `rg src/state/sequencer.rs research/` | 空（仅文档 prose 引用） | ✅ DONE |
| A3 | 7 个 EnvelopeValidationSubclass 全部 surject 到 {ParseFailed, PolicyViolation} | 枚举遍历测试 | `research/envelope_poc/tests/decoupling.rs::every_subclass_surjects_to_existing_classes` | 7 subclasses 全部映射成功；identity_mismatch 单独走 PolicyViolation | ✅ DONE |
| A4 | FC1 LHS invariant 不被破坏：`step + parse_fail + llm_err` 等式继续严格 | 12-attempt mixed batch 模拟 + 7-subclass diversity check | `research/envelope_poc/tests/fc1_invariant.rs::fc1_lhs_invariant_holds_under_mixed_batch`, `::envelope_subclass_diversity_within_parse_fail_bucket` | LHS = 9 = 4 step + 3 parse_fail + 2 llm_err；step_reject + aborted 出 LHS | ✅ DONE |
| A5 | 鲁棒性 & 诊断粒度：50-attempt synthetic batch 解析为 5 类（1 OK + 4 子类） | batch 分类 oracle 对照 | `research/envelope_poc/tests/robustness.rs::robustness_batch_classification_matches_oracle` | 10 OK + 4×10 subclass bucket | ✅ DONE |
| A6 | CR-18R.4 v2 privacy invariant：EnvelopeRejectionPayload 不含 raw response | 字节扫描 + 跨记录可关联性测试 | `research/envelope_poc/tests/robustness.rs::rejection_payload_carries_only_hash_prefix`, `::rejection_payload_hash_is_deterministic_correlatable` | 序列化字符串不含敏感字符串；hash prefix 8 hex 字符 | ✅ DONE |
| A7 | 5 个 task_kind 各有正例 + 至少 1 反例（结构层抓住的） | per-task_kind 单元 | `research/envelope_poc/tests/robustness.rs` 中 10 个 `*_good_validates` + `*_is_*` 反例 | 5 正 + 5 反 全部 pass | ✅ DONE |

合计 24 测试，全部 GREEN。详细输出见 `EVIDENCE_2026-05-26.md`。

---

## 2. 真跑命令（reproducible）

```bash
# 1. 进 worktree
cd /tmp/turingosv4-agent-schema-research

# 2. 编译 PoC
cargo build --manifest-path research/envelope_poc/Cargo.toml

# 3. 跑测试
cargo test --manifest-path research/envelope_poc/Cargo.toml --no-fail-fast

# 4. 单测某条断言
cargo test --manifest-path research/envelope_poc/Cargo.toml --test decoupling envelope_pass_does_not_imply_predicate_pass

# 5. 验证 A2 (零 src/ touch)
git diff --name-only HEAD     # 期望: empty
git ls-files --others --exclude-standard | rg '^src/' || echo "no src/ untracked"
```

---

## 3. PoC 的边界（哪些 PoC 没覆盖，要靠 mainline 实施）

P1. PoC 用 surrogate enum 镜像 main crate 的 `AttemptOutcome` /
   `RejectionClass`；若 main crate 漂移，surrogate 表会失效。Mitigation:
   实施 Phase A 时把 PoC 的 surrogate 删除，直接 `use crate::runtime::...`。
   PoC `src/envelope.rs:23-43` 注释已标 "do NOT drift"。

P2. PoC 不写 CAS。真实 `EnvelopeRejectionPayload` 要 put 进 CAS 用
   `ObjectType::AttemptTelemetry` 关联（不新增 ObjectType）。这点必须在
   Phase A 实施时验证：`tests/cas_envelope_rejection_round_trip.rs`。

P3. PoC 不做 serde bincode canonical encode 测试。Tail-additive
   `envelope_validation_subclass: Option<...>` 加到 `AttemptTelemetry` 时，
   必须用现有 `decode_attempt_telemetry_compat` 验证 historical v1/v2 bytes
   仍能解码（per `feedback_no_retroactive_evidence_rewrite`）。

P4. PoC 不动 `cargo test --workspace`。Phase A 上 mainline 时必须验证:
   - `cargo test --workspace --no-fail-fast` exit 0
   - `bash scripts/run_constitution_gates.sh` exit 0
   - `cargo test --test constitution_matrix_drift` exit 0
   - 新增 `tests/envelope_vs_predicate_decoupling.rs` 作为 constitution gate

P5. PoC fixture 是合成的；真 LLM 输出脏度更高。Phase A 必须收集 200
   真 LLM run 的 parse_fail 率，对比启用 envelope 前后。

---

## 4. 审计链（per AGENTS.md §9 + §14）

本研究 Class 0（docs/research），按 §14 cadence 无强制独立 audit。但
为预备未来 Phase A（Class 1 additive）的审计路径，本计划同步交付:

- `SELF_AUDIT_2026-05-26.md` — 对照 AGENTS.md §6 / §12 / §13 / §14
  逐条自审计。
- `CODEX_AUDIT_DISPATCH_PACKET_2026-05-26.md` — 单 Codex clean-context
  审计 dispatch 模板 (per §9 + memory `feedback_dual_audit` 2026-05-24
  单 Codex 默认)。Witness 输出空间 `{NO-VIOLATION, VIOLATION-FOUND,
  RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`。

---

## 5. 可信反例 / 已发现的设计缺陷

PoC 第一轮跑出 1 测试 FAIL —— 这是好的信号:

**Round 1**: `market_unknown_side_is_unknown_variant` 期望
`EnvelopeUnknownVariant`, 实测 `EnvelopeFieldTooLarge`. 根因: `side` 字段
设了 4 字节硬上限, "MAYBE"(5 字节) 在 enum 判别前被 size 拒。

**修复**: enum-like 字段 (`side`, `directive_kind`, `final_answer_letter`)
统一用 `MAX_FIELD_BYTES`(2KB) cap; enum 判别由 match 处理。

**Round 2**: 24/24 GREEN.

**教训记录**: schema 层的"硬上限"和"discriminator"是两个工作；混在一个
`require_str(p, key, narrow_cap)` 里会把诊断信号染脏。Phase A 实施时
envelope.rs 不变 (PoC 已修)，但 review 阶段必须确认 enum-like 字段
**不允许**配窄字节 cap。

---

## 6. 完成定义 (per AGENTS.md §11 Done Definition)

本研究的 Done = 同时满足:

- [x] FC 节点 + risk class 已说明 (Class 0; 仅 FC1a-rtool / FC1a-judge_pi /
      FC1a-output_edge 的设计空间)
- [x] 相关 unit test 全绿 (24/24)
- [x] Evidence-bearing changes 有 minimal real run (cargo test 输出
      保存在 `EVIDENCE_2026-05-26.md`)
- [x] Diff 审过 hidden Class 4 surface / evidence rewrite / ID namespace
      drift / money/tape/shielding 违例 (见 `SELF_AUDIT_2026-05-26.md`)
- [ ] Clean-context Codex audit 完成 (Class 0 不强制；packet 已写, 待
      用户授权 dispatch)
- [N/A] 动态 handover 文件更新 (Class 0 research, 不动 LATEST.md /
        TB_LOG.tsv)
- [x] OBLIGATIONS.md reconciled: 本研究不解锁任何 OBL；说明见研究文档
      §10 (OBL-005 仅作为 future dependency, 非本次解锁项)

---

## 7. 决策点（给用户）

D1. 本 PoC 通过后，是否进 Phase A charter (Class 1 additive)？
D2. Phase A charter 是否同时携带 envelope schema JSON file
    (`schema/agent_output_envelope/v1.json`) 进 repo？
D3. 是否要先派 Codex audit witness 跑一遍研究文档 + PoC（packet 已写）？
D4. 是否把研究文档 + PoC 作为 docs commit 进 mainline 单 PR（Class 0）？

我的建议:
- D1: 不要立即起 charter。先把研究文档+PoC 让 Codex audit 跑一遍 (D3)，
  确认无 SECOND-SOURCE-DRIFT / RECONSTRUCTION-FAILURE。Audit 通过后再起
  Phase A charter。
- D2: 推荐随 charter 一起进 repo (Class 0 docs commit)。
- D3: **强烈推荐**。这是最低成本的"研究文档审计"。
- D4: 可在 D3 通过后进。

---

(结束 — 完成 7 条断言验证矩阵 + 真跑命令 + PoC 边界 + 审计链 + 完成定义)
