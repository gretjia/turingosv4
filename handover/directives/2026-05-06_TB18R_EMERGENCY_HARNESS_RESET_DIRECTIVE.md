# TuringOS TB-18R Emergency Harness Reset Directive

**Date:** 2026-05-06
**Source:** Architect (zephryj@icloud.com)
**Class:** Architect-level reset (supersedes prior Phase 3 launch ordering)
**Storage policy:** Lossless archive per `feedback_kolmogorov_compression`. Original architect message preserved verbatim below; `## Annotation` section added by Claude AI coder for indexing only.

---

## Architect Message — Verbatim (lossless)

我完全同意你的判断：现在的问题已经不是"TB-18 还差一个审计"，而是 **开发策略本身偏离了你的图灵机初衷**。

你要的是：

```text
纸 / tape 上发生真实计算活动；
铅笔 / write tool 写入状态；
橡皮 / rejection-revert 记录失败；
严格纪律 / predicates 决定 L4 / L4.E；
Agent 的外部化尝试必须可审计。
```

而现在我们被拖进了：

```text
atomic commits
charter
self-audit
dual-audit
round-2
process repair
more audit dispatch
still no real tape evidence
```

这就是你失望的根因。

我把这次 reset 写成了本地指令文件：

[下载：TuringOS_TB18R_Emergency_Harness_Reset_Directive_2026-05-06.md](sandbox:/mnt/data/TuringOS_TB18R_Emergency_Harness_Reset_Directive_2026-05-06.md)

---

# 1. 当前状态的严肃裁决

根据你给的当前上下文，TB-18R 仍然是：

```text
CANDIDATE REMEDIATION
不是 SHIPPED
```

Phase 1 和 Phase 2 已经落在 main，workspace 是 `1077/0/150`，但是 Phase 3 真实 rerun 还没有执行，并且此前规则还要求 Phase 3 和 final ship 都需要显式架构师指令。

现在我给出明确架构师裁决：

> **这条消息就是显式 Phase 3 授权。**
> 不再做 G2.5 schema-only micro-audit 阻塞 Phase 3。
> 立即跑 P38 / P49 / M0 mini-batch，让真实 tape evidence 暴露系统是否合宪。

原因：

> **现在最短的合宪路径不是再审一遍文档，而是把真实 LLM-Lean attempt 放到 tape 上跑。**

---

# 2. 为什么要停止当前慢流程

TB-18 的问题最初已经被说清楚：M1 benchmark 违反了 "1 LLM call → 1 Attempt Node" 的 ChainTape granularity，P49 这类重迭代问题中 N=32 的外部化 LLM proposals 被压缩成 1 个 ChainTape WorkTx，这被内部审计判定为 VETO。

也就是说：

```text
LLM 真实做了 32 次外部化尝试；
tape 只看到 1 个最终结果；
失败、分叉、Lean rejection、partial attempt 都不在 tape 上。
```

这不是图灵机。

图灵机的本质不是"最后答案写在纸上"，而是计算过程在纸带上被严格纪律驱动。你上传的无损宪法也明确把 Flowchart 1 定义为基础运行循环：`Q_t -> read tool -> Agent -> output -> predicates -> write tool -> Q_{t+1}`，而不是 `private evaluator loop -> final result only`。

所以：

```text
更多 audit dispatch 不能解决问题；
真实 attempt 上 tape 才能解决问题。
```

---

# 3. 新开发策略：Constitutional Harness Engineering

我建议立刻把开发策略从：

```text
Atomic Agentic Engineering
```

改成：

```text
Constitutional Harness Engineering
```

含义：

> **每个功能先写"宪法 harness"，证明它严格服从三张 flowchart；再写业务实现；最后跑真实测试。**

不是先写功能、再补审计。

---

# 4. 三张 Flowchart 变成三个可执行 Gate

## Gate FC1 — Runtime Loop Gate

对应基础运行循环：

```text
Q_t -> rtool/context -> Agent output -> predicate/oracle -> wtool -> L4 or L4.E
```

硬不变量：

```text
externalized_attempt_count
==
L4_WorkTx_attempt_count
+ L4E_WorkTx_rejection_count
+ explicitly_anchored_capsule_attempt_count
```

任何外部化 LLM-Lean cycle，如果影响了 proof state、future prompt、Lean check、final composite proof，就必须被记录：

```text
predicate pass -> L4
predicate fail -> L4.E
high-volume logs -> CAS EvidenceCapsule + L4 anchor
```

不能只存在 evaluator stdout。

---

## Gate FC2 — Boot / Genesis Gate

每个真实测试必须可从以下对象重建：

```text
genesis_report
ChainTape
CAS
agent registry
system pubkeys
```

禁止：

```text
memory-only preseed
retroactive evidence rewrite
global latest pointer source-of-truth
```

TB-16 已经暴露过 global Markov pointer 的平行账本问题；这类问题不能再回来。

---

## Gate FC3 — Meta / Markov Gate

EvidenceCapsule 和 Markov capsule 是：

```text
derived view / evidence compression
```

不是隐藏 ground truth。

要求：

```text
raw logs shielded
capsule derived from ChainTape + CAS
dashboard materialized view only
no global raw-log prompt stuffing
```

---

# 5. TB-18R 立即执行方案

## 5.1 立刻停止 G2.5 schema-only micro-audit

你给的当前状态里，AI coder 选择先写 G2.5 micro-audit，因为认为 Phase 2 是 Class 4 substrate，Phase 3 是 expensive ladder。

我现在推翻这个顺序。

理由：

```text
Phase 2 schema 如果错，P38/P49 会暴露；
Phase 2 schema 如果对，P38/P49 会给 tape evidence；
schema-only audit 不能替代真实 tape。
```

所以：

```text
不要再用 G2.5 阻塞 Phase 3。
```

---

## 5.2 直接启动 Phase 3

运行：

```text
P38
P49
M0 mini-batch
```

用：

```text
MAX_TRANSACTIONS=12
PER_PROBLEM_TIMEOUT_S=1800
```

新的 evidence dir：

```text
handover/evidence/tb_18r_phase_3_<timestamp>/
```

严格禁止：

```text
retroactive 改 M1 / R6 / R7 历史 evidence
```

这一点保持不变。

---

# 6. Phase 3 必须输出的证据

每个 run 必须产生：

```text
L4 entries
L4.E entries
CAS AttemptTelemetry
CAS LeanResult
EvidenceCapsule if exhausted/degraded
ChainDerivedRunFacts
audit_tape report
dashboard report
attempt count equality report
```

如果 P49 仍然是：

```text
evaluator tx_count = 32
chain_attempt_count = 1
```

立即 halt。
不许继续 M0/M1/M2。

---

# 7. 新 Kill Gates

Phase 3 期间，遇到以下任一情况立即停止：

```text
1. evaluator_reported_tx_count != chain_attempt_count

2. P38 or P49 has N > 1 externalized attempts
   but chain_attempt_count = 1

3. Any Lean reject appears only in stdout
   and not in L4.E / EvidenceCapsule

4. Final composite proof lacks attempt_chain_root
   or equivalent lineage

5. Dashboard needs evaluator stdout to reconstruct core facts

6. Any fake accepted node appears

7. CTF conservation fails

8. ChainTape mode silently falls back to legacy bus.append

9. Global Markov pointer reappears

10. PartialAccepted schema produces untyped ambiguity:
    exit_code=0, verified=false, error_class=None
```

这 10 条是新的 **Constitutional Harness Kill Gates**。

---

# 8. Phase 3 接受标准

Phase 3 只有在以下条件满足时才算通过：

```text
P23 one-shot:
  attempt_count = 1
  chain_attempt_count = 1

P38 multi-attempt:
  chain_attempt_count == evaluator_reported_tx_count

P49 heavy run:
  chain_attempt_count == evaluator_reported_tx_count

Real Lean rejects:
  represented in L4.E or anchored EvidenceCapsule

AttemptTelemetry:
  payloads resolve from CAS

LeanResult:
  payloads resolve from CAS

Final proof:
  lineage resolves from CAS

Dashboard:
  DAG regenerates from ChainTape + CAS

workspace tests:
  pass
```

只有这样，才重新进入：

```text
Codex + Gemini final audit
```

---

# 9. 调整后的审计策略

以前是：

```text
fix -> audit -> more docs -> audit -> rerun maybe
```

现在改成：

```text
harness -> real run -> audit
```

具体：

## 对 Class 3 / 4 功能：

```text
1. 写 constitutional harness tests；
2. 跑最小真实样本；
3. 若真实样本通过，再外部审计；
4. 若真实样本失败，直接回实现层，不进入审计。
```

这会快很多，也更符合你的目标。

---

# 10. 后续开发节奏

## 当前只做 TB-18R

冻结继续生效：

```text
no M1/M2/M3
no NodeMarket
no PriceIndex
no Polymarket signal
no public chain
no real-world readiness
no M1 public benchmark report
no TB-19 real-world pilot
no formal H-VPPUT claim
no "formal benchmark passed"
```

直到 TB-18R final ship。

---

## TB-18R 通过后

再进入：

```text
M1 50-problem
M2 100+ n5
```

但每个 benchmark 都先跑 denominator harness：

```text
externalized_attempt_count == chain_attempt_count
```

没有这个等式，就不跑大规模。

---

# 11. 给 AI coder 的直接执行口令

可以直接发：

```text
Architect reset directive:

1. Stop G2.5 schema-only micro-audit from blocking Phase 3.
2. This directive explicitly authorizes TB-18R Phase 3.
3. Run P38 + P49 + M0 mini-batch now on the typed substrate.
4. New evidence dir:
   handover/evidence/tb_18r_phase_3_<timestamp>/
5. Do not retroactively modify M1/R6/R7 historical evidence.
6. Required invariant:
   evaluator_reported_tx_count == chain_attempt_count.
7. For every externalized LLM-Lean cycle:
   predicate pass -> L4
   predicate fail -> L4.E
   high-volume logs -> CAS EvidenceCapsule + L4 anchor
8. Do not continue if P38/P49 collapse N attempts into 1 WorkTx.
9. Do not continue if Lean rejects exist only in stdout.
10. Do not continue if dashboard needs evaluator stdout to reconstruct core facts.
11. After Phase 3 evidence passes, then run Codex + Gemini final audit.
12. No M2/M3, NodeMarket, PriceIndex, Polymarket, public chain, real-world readiness, or formal benchmark claims before TB-18R ship.
```

---

# 12. 最终判断

你这次的情绪不是"项目焦虑"，而是架构直觉正确地发现了：

```text
流程已经在服务审计，
而不是服务 tape。
```

现在必须反过来：

```text
tape 服务宪法；
harness 服务 tape；
审计服务 harness；
文档只服务最终证据包装。
```

所以我的最新总裁决是：

> **立刻进入真实 Phase 3 测试，不再被 schema-only micro-audit 阻塞。**
> **从今天开始，所有开发以 Constitutional Harness 为主，不以 atomic audit ceremony 为主。**
> **没有 tape activity 的测试，一律不算 TuringOS 测试。**

---

## Architect Message — Companion Markdown File (verbatim)

The architect also attached a structured companion file. Reproduced verbatim below.

```text
; TuringOS TB-18R Emergency Harness Reset Directive
Status
This directive supersedes the slow path where Phase 3 waits behind another schema-only micro-audit.
Current state:
TB-18R is CANDIDATE REMEDIATION, not SHIPPED.
Phase 1 process repair is shipped.
Phase 2 typed PartialAccepted schema bump is shipped.
Workspace baseline reported by AI coder: 1077/0/150.
Phase 3 rerun P38 + P49 + M0 mini-batch is now explicitly authorized by architect instruction.
Prime Directive
Tape first. Real tests now. No more benchmark scaling until tape activity is real.
Every externalized LLM-Lean cycle must become tape-visible:
predicate pass -> L4 accepted WorkTx / VerifyTx
predicate fail -> L4.E rejection evidence
high-volume logs -> CAS EvidenceCapsule, but never as substitute for attempt visibility
dashboard -> materialized view only
Flowchart gates
FC1 Runtime Loop Gate
For every externalized LLM proposal:
Q_t -> rtool/context -> Agent output -> predicate/oracle -> wtool -> L4 or L4.E
Hard invariant:
externalized_attempt_count == L4_WorkTx_attempt_count + L4E_WorkTx_rejection_count + explicitly_anchored_capsule_attempt_count
No evaluator-private attempt may affect future proof state without tape/CAS representation.
FC2 Boot Gate
Every evidence run must be replayable from:
genesis_report + ChainTape + CAS + agent registry + system pubkeys
No memory-only preseed.
No retroactive evidence rewrite.
No global pointer source of truth.
FC3 Meta / Markov Gate
EvidenceCapsule and Markov capsule are derived views, not hidden ground truth.
Raw logs stay shielded.
Latest capsule can guide next run, but must be derivable from ChainTape + CAS.
No global filesystem pointer canonical input.
Emergency execution sequence
R3.0 Archive this directive
Write this file into:
handover/directives/2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md
This is explicit architect authorization for Phase 3. Do not reinterpret it as an implicit one-word fix.
R3.1 Stop G2.5 micro-audit-first path
Do not block Phase 3 on schema-only G2.5 micro-audit.
Reason:
The current bottleneck is not more textual review. The bottleneck is real evidence that the typed substrate actually produces tape-visible attempts.
R3.2 Run Phase 3 evidence now
Run:
P38
P49
M0 mini-batch
Required environment:
MAX_TRANSACTIONS=12
PER_PROBLEM_TIMEOUT_S=1800
new evidence dir:
handover/evidence/tb_18r_phase_3_<timestamp>/
Do not modify historical M1/R6/R7 evidence.
R3.3 Mandatory runtime outputs
For each run, produce:
ChainTape L4 entries
ChainTape L4.E entries
CAS AttemptTelemetry objects
CAS LeanResult objects
EvidenceCapsule if exhausted/degraded
ChainDerivedRunFacts
audit_tape report
dashboard report
attempt count equality report
R3.4 Kill gates
Halt immediately if any is true:
evaluator_reported_tx_count != chain_attempt_count
P38 or P49 has N>1 externalized attempts but chain_attempt_count=1
any Lean reject appears only in stdout and not in L4.E / EvidenceCapsule
final composite proof lacks attempt_chain_root or equivalent lineage
dashboard needs evaluator stdout to reconstruct core facts
any fake accepted node appears
CTF conservation fails
ChainTape mode silently falls back to legacy bus.append
global Markov pointer reappears
PartialAccepted schema produces untyped exit_code=0, verified=false, error_class=None ambiguity
R3.5 Acceptance gates
Accept Phase 3 only if:
P23 one-shot has attempt_count=1 and chain_attempt_count=1
P38 multi-attempt has chain_attempt_count == evaluator_reported_tx_count
P49 heavy run has chain_attempt_count == evaluator_reported_tx_count
real Lean rejects are represented in L4.E or anchored EvidenceCapsule
AttemptTelemetry payloads resolve from CAS
LeanResult payloads resolve from CAS
final proof lineage resolves from CAS
dashboard DAG regenerates from ChainTape + CAS
workspace tests pass
R3.6 After Phase 3
Only after Phase 3 evidence passes:
run final Codex + Gemini audit
apply conservative ranking VETO > CHALLENGE > PASS
then ask for final ship sign-off
Harness engineering reset
Old process to stop
Do not continue:
charter -> atom -> self-audit -> external audit -> more docs -> delayed test
New process
For every high-risk TB:
Constitution gate as executable tests first.
Minimal real run second.
Only then expand atoms.
External audit after evidence, not before evidence.
Documentation is proof packaging, not substitute for tape.
Constitutional CI package
Implement / enforce these persistent tests:
fc1_every_externalized_attempt_is_tape_visible
fc1_predicate_pass_goes_l4_fail_goes_l4e
fc2_run_replayable_from_genesis_tape_cas
fc2_no_memory_only_preseed
fc3_capsule_derived_from_tape_cas
no_global_markov_pointer
no_dashboard_source_of_truth
no_legacy_authoritative_append
no_fake_accepted_nodes
no_f64_money_path
total_coin_conserved
system_tx_not_agent_submittable
Freeze remains
Until TB-18R final ship:
no M2/M3 scale-up
no NodeMarket
no PriceIndex claims
no Polymarket signal
no public-chain
no real-world readiness
no public benchmark report
no formal H-VPPUT claim
no "formal benchmark passed" external claim
Final instruction to AI coder
Run Phase 3 now under this directive.
Do not do another schema-only micro-audit before real runs.
If the typed substrate is wrong, let P38/P49/M0 expose it now.
```

---

## Annotation (added by Claude AI coder, 2026-05-06)

This section is structured indexing only; it does NOT distill, paraphrase, or replace any architect text above. Original message remains source of truth.

### Effect summary
1. **Supersedes** `2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md` ordering insofar as G2.5 micro-audit must NOT block Phase 3.
2. **Operating philosophy shift**: Atomic Agentic Engineering → Constitutional Harness Engineering.
3. **Explicit Phase 3 authorization**: this message itself counts as architect §8-style explicit Phase 3 go.
4. **Final ship still requires**: Phase 3 evidence pass → Codex + Gemini final audit (post-evidence) → architect sign-off.

### Axis impact (per `feedback_no_workarounds_strict_constitution`)
- Article-level axiom impact: **NONE** — directive enforces existing FC1/FC2/FC3 invariants strictly; it does not amend constitution.
- Case-law impact: introduces **Constitutional Harness Engineering** as the operative method; CLAUDE.md updated to install.
- Memory impact:
  - `feedback_constitutional_harness_engineering` (new — primary operating mode)
  - `feedback_tape_first_real_tests` (new — anti-ceremony rule)
  - `feedback_audit_after_evidence` (reinforced — applies to ALL Class-3/4, not only G0/G1)
  - `feedback_no_workarounds_strict_constitution` (reinforced)
  - `project_tb_18r_provisional_shipped` (updated — Phase 3 now authorized)

### Class
- This directive is itself **Class 4** in scope (operating-mode amendment) but introduces NO Class-4 code change.
- Phase 3 execution per directive R3.2–R3.5 = **Class 3** (capability replay on existing typed substrate).

### Required CI tests catalogued
The "Constitutional CI package" lists 12 persistent tests to implement/enforce:
1. `fc1_every_externalized_attempt_is_tape_visible`
2. `fc1_predicate_pass_goes_l4_fail_goes_l4e`
3. `fc2_run_replayable_from_genesis_tape_cas`
4. `fc2_no_memory_only_preseed`
5. `fc3_capsule_derived_from_tape_cas`
6. `no_global_markov_pointer`
7. `no_dashboard_source_of_truth`
8. `no_legacy_authoritative_append`
9. `no_fake_accepted_nodes`
10. `no_f64_money_path`
11. `total_coin_conserved`
12. `system_tx_not_agent_submittable`

### 10 New Kill Gates (Phase 3 halt-immediately conditions)
1. `evaluator_reported_tx_count != chain_attempt_count`
2. P38 or P49 has N>1 externalized attempts but `chain_attempt_count = 1`
3. Lean reject exists only in stdout (not in L4.E / EvidenceCapsule)
4. Final composite proof lacks `attempt_chain_root` or equivalent lineage
5. Dashboard needs evaluator stdout to reconstruct core facts
6. Any fake accepted node appears
7. CTF conservation fails
8. ChainTape mode silently falls back to legacy `bus.append`
9. Global Markov pointer reappears
10. PartialAccepted schema produces untyped `exit_code=0, verified=false, error_class=None` ambiguity

### Phase 3 acceptance gates (9 conditions)
1. P23 one-shot: `attempt_count=1, chain_attempt_count=1`
2. P38 multi-attempt: `chain_attempt_count == evaluator_reported_tx_count`
3. P49 heavy run: `chain_attempt_count == evaluator_reported_tx_count`
4. Real Lean rejects represented in L4.E or anchored EvidenceCapsule
5. AttemptTelemetry payloads resolve from CAS
6. LeanResult payloads resolve from CAS
7. Final proof lineage resolves from CAS
8. Dashboard DAG regenerates from ChainTape + CAS
9. `cargo test --workspace` passes

### Freeze (unchanged + reinforced)
Until TB-18R FINAL ship: no M1/M2/M3 scale-up, no NodeMarket, no PriceIndex, no Polymarket-signal, no public-chain, no real-world readiness, no public benchmark report, no TB-19 pilot, no formal H-VPPUT claim, no "formal benchmark passed" external claim.

### Authorization audit trail
- 2026-05-06 (this directive): explicit Phase 3 go
- Prior: `2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` required Phase 1 → Phase 2 → Phase 3 → final dual audit
- Prior: `2026-05-06_TB18R_PHASE_3_LAUNCH_DIRECTIVE.md` (now superseded re: pre-Phase-3 G2.5 audit gating)

### Filename note
Architect-supplied attachment named `TuringOS_TB18R_Emergency_Harness_Reset_Directive_2026-05-06.md`; archived here as `2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md` to match `handover/directives/` convention (`YYYY-MM-DD_NAME.md`).
