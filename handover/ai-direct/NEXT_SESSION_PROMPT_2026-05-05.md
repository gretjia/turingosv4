# Next-session starter prompt (paste at session start)

Cold-start ready as of 2026-05-05. Push state: `f7aaa0e` on origin/main.

---

## Quick paste-template (copy to new Claude session)

```
你是 TuringOS v4 项目协作者. 上一会话刚结束 (commit f7aaa0e on main).
读以下顺序冷启:
1. CLAUDE.md (项目宪法 + risk class + audit standard)
2. handover/ai-direct/LATEST.md (最新会话 ledger; 找 "session end #2" 段)
3. handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md
   (架构师对 TB-16 + TB-17 的最终裁决, lossless verbatim §B + 注解 §A)

当前状态:
- TB-17 PROVISIONAL SHIPPED (commit 3e0c91d): 19/20 SG green;
  SG-17.17 待架构师 §8 签字.
- 3 个 ratification ask pending 架构师 (LATEST.md 列出).
- M0 r1 closed (1/20 clean + 2/20 DeepSeek drift hung; OBS 已 file 到 TB-18 atom A).
- 4 个新 memory 已 indexed.

我 (用户) 现在告诉你:
[在此写你的指令 — 见下面的"我可能问什么"]
```

---

## 我可能问什么 (按概率排序)

### A. 如果架构师已经回了 (你看了 §8 signature + PRE 裁决)

```
架构师回复:
[A.1] §8 签字: ✅ READY / CONDITIONAL / NOT-READY
[A.2] PRE-17.5: 在 TB-17 内做 / 推 TB-18
[A.3] PRE-17.6: ratify deviation / re-charter atom 8
[A.4] PRE-17.7: 在 TB-17 内做 / 推 TB-18
[A.5] TB-18 charter 起草授权: yes / no / 等

按这个执行下去.
```

### B. 如果架构师还没回, 但你想做 *准备性* 工作

```
架构师还在审. 在等的时候, 你可以做 [选一个]:
[B.1] 起草 TB-18 charter 草稿 (atom A-H 8 个; 基于 atom 8 deviation §6).
      Class 0 工作; 不动代码. 等架构师 §8 签字 + 三个 ratify 之后 charter 才会 RATIFY.
[B.2] 起草 OBS_M0_DEEPSEEK_DRIFT 的实施细节文档 (per-LLM-call budget API spec).
      这是 TB-18 atom A 的设计 prep; 加快后续实施.
[B.3] 完整 review 这次 TB-17 ship 的 6 份白皮书, 找 cross-ref drift / 文字 bug.
      Class 0 polish 工作.
[B.4] 不要做 prep 工作; 我要你只 ack 状态, 等我有具体指令再动.

选 [B.X] 并执行.
```

### C. 如果你想看 / 验证 / 解释某件事

```
解释清楚 [题目]. 用通俗易懂的话说. 不要堆术语.
```

候选 [题目]:
- "TB-17 12 个 atom 各自做了什么"
- "DeepSeek drift 为什么以前没暴露"
- "Boltzmann observe vs enforce 区别是什么"
- "三个盒子 (顶白 / 中黑 / 底白) 在 M3 里会怎么互动"
- "Class 0 / 1 / 2 / 3 / 4 区别"
- "为什么单链 13/13 拿不下来要拆 4 链"
- "Markov inheritance 是什么 / α 和 β 区别"
- "OBS_R023 为什么必须在 TB-18 之前关"

### D. 如果你想 STOP 或 PIVOT

```
我不要继续 TB-18. 现在的 priority 改成 [新 priority].
```

或:

```
TB-17 ship 不签了. 撤销 commit 3e0c91d (provisional ship), 重新评估方向.
```

(这种情况罕见但可能 — 如果你审完发现 TB-17 方向不对.)

---

## 我 (Claude) 不应该做什么 (启动后立即知道)

不需要你提醒, 我会自动遵守:

- ❌ 不动 PRE-17.5 / .7 的代码 (Class 4; 等单独 ratify 才能动 schema / signing payload)
- ❌ 不重跑 M0 (per `feedback_no_workarounds_strict_constitution`; 等 TB-18 atom A 修预算)
- ❌ 不擅自切 LLM 模型 (deepseek-reasoner / GPT / Gemini; 跨 provider 是 TB-18 范围)
- ❌ 不删 P01_/P02_ 顶层 forensic dirs (per `feedback_no_retroactive_evidence_rewrite`)
- ❌ 不动现有 11 个修改过的 evidence 文件 (carry-forward; 沙盒 v3 时代漂移; 保留)
- ❌ 不主动起 cron / Monitor (除非你明示要 supervised long-running task)
- ❌ 不主动 push (commit 可以; push 需要明示)
- ❌ 不发表 / 不让任何 M0 r1 P01 数据被引用为 benchmark (per `feedback_minif2f_scaling_policy`)

---

## 关键文件路径速查表

```
# 架构师裁决 (lossless 最权威)
handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md
handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md
handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md

# TB-17 charter + 6 白皮书 + 3 设计/deviation
handover/tracer_bullets/TB-17_charter_2026-05-05.md
handover/whitepapers/REAL_WORLD_READINESS_REPORT.md  ← 等你 §8 签字
handover/whitepapers/DOMAIN_SELECTION_CRITERIA.md
handover/whitepapers/ORACLE_REQUIREMENTS.md
handover/whitepapers/CHALLENGE_COURT_REQUIREMENTS.md
handover/whitepapers/SAFETY_BOUNDARY.md
handover/whitepapers/IRREVERSIBLE_ACTION_POLICY.md
handover/proposals/TB-17_PRE_17_5_BOLTZMANN_ENFORCE_DESIGN_2026-05-05.md
handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md
handover/proposals/TB-17_PRE_17_7_INTAPE_MARKOV_DESIGN_2026-05-05.md

# Ship status
handover/ai-direct/TB-17_SHIP_STATUS_2026-05-05.md

# M0 (闭环)
handover/alignment/OBS_M0_DEEPSEEK_DRIFT_2026-05-05.md
handover/evidence/m0_minif2f_harness_audit_2026-05-05/r1/
  P01_mathd_algebra_107/ (clean baseline)
  P02_mathd_algebra_113/ (drift signature)
  M0_RUN_MANIFEST.json
  m0_runner.log

# Memory (~/.claude/projects/-home-zephryj-projects-turingosv4/memory/)
project_tb_16_ratified_with_scope_limits.md
feedback_minif2f_scaling_policy.md
feedback_class4_cannot_hide_in_class3.md
project_tb_17_ratified_charter_2026-05-05.md
```

---

## 当前 git state

```
HEAD       : f7aaa0e (Session-end handover-update — TB-17 SHIPPED + M0 r1 closed)
origin/main: f7aaa0e (push complete this session)

Last 5 commits this session:
  f7aaa0e  Session-end handover-update
  6471c28  M0 r1 STOPPED + OBS_M0_DEEPSEEK_DRIFT filed
  cfff1a3  M0 harness prep
  3e0c91d  TB-17 SHIPPED (provisional)
  d431ac2  (prior session: TB-16.x.2 handover)
```

---

## 测试基线

```
cargo test --workspace --release = 939 / 0 / 150
  (TB-16 baseline 922 + 17 new TB-17 conformance tests)
  +10 markov_inheritance_policy
  +5  irreversible_action_examples
  +2  minif2f_scale_separation
```

下一会话第一件事 (如果有任何代码改动): re-run `cargo test --workspace --release` 确认 baseline 不退化.
