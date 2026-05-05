# Next-session starter prompt — TB-18 PROVISIONAL SHIPPED state (2026-05-05)

Cold-start ready as of 2026-05-05 (TB-18 PROVISIONAL ship under user blanket auto-mode authority through ship). M-ladder M0 retry ran in-session.

---

## Quick paste-template (copy to new Claude session)

```
你是 TuringOS v4 项目协作者. 上一会话刚结束 TB-18 PROVISIONAL ship.

读以下顺序冷启:
1. CLAUDE.md (项目宪法 + risk class + audit standard)
2. handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md (top entry: TB-18 PROVISIONAL state)
3. handover/ai-direct/TB-18_SHIP_STATUS_2026-05-05.md (PROVISIONAL ship doc;
   §3 SG-18.1..16 walk + §4 honest deferral ledger + §6 forward-binding
   to TB-19+)
4. handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md
   (architect 17-point execution command + §A.5 hidden issues;
   lossless verbatim §B)

当前状态:
- TB-18 PROVISIONAL SHIPPED (commits d3c8d78..fb1025c on main; 8+ commits)
  - Atom 0 charter ratified-with-amendments
  - Atom E OBS_R023 closed
  - Atom A drive_task API + per-LLM-call budget + RunOutcome::DegradedLLM
  - Atom H0 small M0 preflight PASS-WITH-CAVEAT (substrate validates)
  - Atom D-design verdict: Class 4 escalation refused; Path C dissolves PRE-17.6 §2.2
  - Atom C 4/5 idempotency gates structural; Gate 3 PARTIAL → TB-19+ STEP_B
  - Atom B-design DESIGN-COMPLETE; impl → TB-18.B-impl follow-up
  - Atom H prep BenchmarkManifest + EvidencePackagingPolicy filed
  - M0 retry: 20-problem batch on atom A budget-enforced substrate
  - G0 + G1 audit request docs filed; awaiting external invocation

- Workspace tests: 962/0/150 (baseline 939 + 23 TB-18 tests).
- 12-item honest deferral ledger to TB-18.* follow-ups + TB-19+:
  TB-18.B-impl (SharedChain refactor 4-8h) / TB-18.F (single-chain 13/13) /
  TB-18.H-impl (M1 multi-hour + M2 multi-day) / TB-18.G0 + G1 (external) /
  Atom D-impl (Class 4 → TB-19+) / Gate 3 ChallengeStatus::Open-blocking
  (STEP_B → TB-19+) / PRE-17.5 / PRE-17.7 β-D / M3 / M4 → TB-19+.

- Architect ship-claim narrowing applied (Q2 verbatim):
  "formal benchmark substrate partially closed; lifecycle-order constraint
  remains Class 4 forward trigger".

- TB-18 SHIPPED FINAL gate = (1) M0 retry + report final + replay
  integrity check; (2) TB-18.B-impl + TB-18.F + TB-18.H-impl follow-ups;
  (3) external G0 + G1 audits invoked by user; (4) architect § sign-off.

我 (用户) 现在告诉你: [在此写指令]
```

---

## 我可能问什么

### A. 如果你想推进 TB-18 follow-up 工作

```
[A.1] 起 TB-18.B-impl: 实施 SharedChain refactor + run_swarm 参数化 +
      comprehensive_arena 重写; STEP_B_PROTOCOL 平行分支; 4-8h focused
      work + Codex/Gemini external audit at end.
[A.2] 跑 TB-18.H-impl M1: 50-100 problems × n1/n3; 多小时 LLM compute;
      EVIDENCE_INDEX.json + sampled packaging per EvidencePackagingPolicy.
[A.3] 跑 TB-18.H-impl M2: 100+ × n5; 多日 LLM compute; observe-only
      Boltzmann.
[A.4] 等 G0 + G1 外部审计 (我 invoke /ultrareview 或 Codex/Gemini),
      你处理 audit 反馈 + 写 verdict commit.
[A.5] 架构师 § sign-off review on benchmark report; 如果 CONDITIONAL,
      处理 caveats.

按 [A.X] 执行.
```

### B. 如果你想做 cleanup / consolidation

```
[B.1] 整理 TB-18 evidence packaging: 跑
      handover/tests/scripts/tb_18_package_m0_evidence.sh 应用到所有
      M0 retry per-problem dirs (tar .git → .dotgit.tar.gz 删除原 .git);
      验证 EvidencePackagingPolicy §5 replay integrity check 通过.
[B.2] /handover-update 更新 LATEST.md 反映 TB-18 PROVISIONAL ship state.
[B.3] 检查 dual_audit / external 触发 docs 一致性.

执行 [B.X].
```

### C. 解释 / 验证某件事

```
解释清楚 [题目]. 用通俗易懂的话说. 不要堆术语.
```

候选 [题目]:
- "TB-18 atom A 的 budget tracker 实际怎么工作的"
- "atom D 为什么 SKIP 不是 dodge"
- "atom B-impl 为什么不能在一个 session 里做完"
- "M0 vs M1 vs M2 vs M3 vs M4 区别"
- "为什么 SG-18.10 + SG-18.11 是 NOT-RUN 不是 RED ship blocker"
- "SharedChain 重构具体长什么样"

### D. 如果你想 STOP 或 PIVOT

```
我不要继续 TB-18 后续. 现在的 priority 改成 [新 priority].
```

或:

```
TB-18 PROVISIONAL ship 不签了. 撤销 commits d3c8d78..<head> (回到 TB-17
SHIPPED FINAL @ 8e3d5cc). 重新评估方向.
```

---

## 我 (Claude) 不应该做什么 (启动后立即知道)

不需要你提醒, 我会自动遵守:

- ❌ 不动 PRE-17.5 (Boltzmann ENFORCE) 代码 (Class 4; TB-19+ separate TB)
- ❌ 不动 PRE-17.7 β-D 代码 (TB-19+; β-A only in TB-18)
- ❌ 不擅自跑 M3 / M4 (Q6 deferred TB-19+)
- ❌ 不动 TaskLifecycle 改 append-only history (architect §2.7 invariant; Class 4 → TB-19+)
- ❌ 不动 atom C Gate 3 (ChallengeStatus::Open-blocking) without STEP_B parallel-branch (sequencer.rs restricted)
- ❌ 不假装 M1/M2 跑了 (没真跑就是没跑; honest deferral)
- ❌ 不主动 push (commit 可以; push 需要明示)
- ❌ 不在 PROVISIONAL ship 上加 fake "shipped final" tag without architect § sign-off
- ❌ 不 retroactive rewrite TB-13/14/16/17 evidence (per `feedback_no_retroactive_evidence_rewrite`)
- ❌ 不 claim real-world readiness from MiniF2F results (per `feedback_minif2f_scaling_policy`)

---

## 关键文件路径速查表

```
# 架构师裁决 (lossless 最权威)
handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md
handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md

# TB-18 charter + atom design docs
handover/tracer_bullets/TB-18_charter_2026-05-05.md
handover/proposals/TB-18_ATOM_B_DESIGN_2026-05-05.md      ← TB-18.B-impl spec
handover/proposals/TB-18_ATOM_D_DESIGN_2026-05-05.md      ← Path C verdict

# TB-18 manifest + policy + audit requests
handover/manifests/TB-18_BENCHMARK_MANIFEST.json          ← M-ladder anti-drift
handover/policies/TB-18_EVIDENCE_PACKAGING_POLICY.md       ← TB-7R/TB-8/TB-9 precedent
handover/audits/CODEX_MICRO_AUDIT_TB_18_PRE_H_REQUEST_2026-05-05.md
handover/audits/DUAL_AUDIT_TB_18_REQUEST_2026-05-05.md

# TB-18 ship status + benchmark report
handover/ai-direct/TB-18_SHIP_STATUS_2026-05-05.md         ← PROVISIONAL state
handover/whitepapers/MINIF2F_M0_BENCHMARK_REPORT.md       ← M0 only; M1/M2 forward

# TB-18 evidence
handover/evidence/tb_18_h0_m0_preflight_2026-05-05/        ← H0 (3 problems)
handover/evidence/tb_18_m0_retry_2026-05-05/r1/            ← M0 retry (20 problems)

# TB-18 source modules (atom A + atom E)
experiments/minif2f_v4/src/per_call_budget.rs              ← atom A primitives
experiments/minif2f_v4/src/drive_task.rs                   ← atom A API stub
experiments/minif2f_v4/src/bin/evaluator.rs                ← atom A wiring + atom E propagation
src/state/typed_tx.rs                                       ← RunOutcome::DegradedLLM = 5

# TB-18 tests
tests/tb_18_evidence_capsule_outcome_propagation.rs         ← atom E + projection
tests/tb_18_deferred_finalize_idempotency.rs                ← atom C 4/5 gates
experiments/minif2f_v4/tests/tb_18_per_llm_call_budget.rs   ← atom A budget tests

# Memory (~/.claude/projects/-home-zephryj-projects-turingosv4/memory/)
project_tb_18_provisional_shipped.md
project_tb_18_charter_ratified.md
feedback_audit_after_evidence.md
feedback_benchmark_manifest_required.md
feedback_evidence_packaging_policy_required.md
```

---

## Test baseline + git state

```
HEAD:        <fb1025c or later>
origin/main: <0> behind/ahead (push pending)
Last commits this session (auto-mode through TB-18 PROVISIONAL ship):
  fb1025c  TB-18 session-state update — AUTO_RESEARCH_NOTEPAD + preliminary M0 report
  31dbf3b  TB-18 ship status PROVISIONAL
  d94654b  TB-18 Atom H prep + Atom G0/G1 audit requests
  7bb18b4  TB-18 Atom B SHIPPED AS DESIGN-COMPLETE
  ae9530f  TB-18 Atom C SHIPPED — 4/5 idempotency gates
  c025cdb  TB-18 Atom D-design SHIPPED — Class 4 refusal
  5c40d06  TB-18 Atom H0 SHIPPED — M0 small preflight PASS-WITH-CAVEAT
  13a5ee0  TB-18 Atom A SHIPPED — drive_task + budget + DegradedLLM
  8ad7a1d  TB-18 Atom E SHIPPED — OBS_R023 closure
  d3c8d78  TB-18 Atom 0 — charter ratified-with-amendments + ruling archive

Workspace tests (cargo test --workspace --release): 962/0/150
  baseline 939 (TB-17 ship)
  + 3  tb_18_evidence_capsule_outcome_propagation (atom E)
  + 5  per_call_budget unit tests (atom A)
  + 3  drive_task unit tests (atom A)
  + 7  tb_18_per_llm_call_budget integration (atom A)
  + 1  projection extension for DegradedLLM (atom A)
  + 4  tb_18_deferred_finalize_idempotency (atom C)
  Total: +23 = 962
```

**第一件事 (next session start)**: re-run `cargo test --workspace --release` — confirm 962/0/150 baseline preserved.
