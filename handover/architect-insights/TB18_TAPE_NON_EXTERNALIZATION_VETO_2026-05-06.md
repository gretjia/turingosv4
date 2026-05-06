# TB-18 M1 Tape Non-Externalization — External Audit VETO + Constitution Repair Plan

**Status**: **VETO ACCEPTED 2026-05-06**. TB-18 M1 benchmark FROZEN. M2 / M3 / NodeMarket / PriceIndex / Polymarket-signal / public-chain / real-world-readiness all FROZEN. TB-18R (Tape Restoration) charter authorized 2026-05-06 by user "全部按你的意见执行". Class 4 carve-out triggered (typed-tx schema + sequencer admission semantics + canonical signing payload). Codex + Gemini external audit required on TB-18R charter before R1 (schema) atom executes.

**Date**: 2026-05-06
**Predecessor**: TB-18 PROVISIONAL SHIPPED 2026-05-05 (commit `15b662c` Atom B-impl) — substrate atoms 0/E/A/H0/D-design/C/B-design/H-prep + G0/G1 audit-request docs shipped under blanket-auto-mode authority; M0 retry running at archive time; M1 results triggered VETO (P49-class observation).
**Successor**: TB-18R Charter 2026-05-06 (`handover/tracer_bullets/TB-18R_charter_2026-05-06.md`).

**Authority** (in order of precedence):
  - **2026-05-06 external audit VETO** (this archive's §A + §B, lossless verbatim per `feedback_kolmogorov_compression`).
  - **2026-05-06 user authorization** ("全部按你的意见执行" — authorizes archive + charter draft + Codex external audit dispatch; does NOT authorize R1 schema implementation pre-Codex-audit).
  - **constitution.md Art.0 (Tape Canonical) + Art.I.1 (5-step compile loop) + Art.II.2.1 (entropy) + Art.III.1..4 (privacy contracts) + Art.IV (terminal-state distinction)** — invariants the audit grounds in; binding.
  - **CLAUDE.md "Class 4 cannot hide in Class 3"** (memory `feedback_class4_cannot_hide_in_class3`) — sequencer admission / typed-tx schema / canonical-signing-payload changes require separate ratification beyond TB-18 envelope.
  - **memory `feedback_chaintape_externalized_proposal`** — "1 LLM call → 1 compound payload = 1 Attempt Node". Audit grounds the VETO in this rule's PER-CALL semantic, not in per-tactic decomposition (per-tactic is TB-8+ scope per the rule's own words).
  - **memory `feedback_no_workarounds_strict_constitution`** — user explicitly rejects OBS-bucket / null-pointer / Layer-G-Skip workarounds when audit gate fails on smoke; this VETO is exactly the "我不要凑活" case.
  - **memory `feedback_no_retroactive_evidence_rewrite`** — TB-18R fixes are going-forward only; pre-TB-18R M1 evidence remains as-is with grandfathering note; no rewrite of historical L4 / L4.E / CAS roots.

---

## §0. One-line verdict

> **TB-18 M1 benchmark is not a TuringOS tape — it is a final-state snapshot with private evaluator process. VETO. Stop. Open TB-18R Tape Restoration. Forbid M2/M3/NodeMarket/Polymarket-signal/real-world-readiness until restored.**

---

## §A. Verbatim VETO + Repair Plan (lossless, as delivered by user 2026-05-06)

> Per `feedback_kolmogorov_compression`: the original directive is archived verbatim. No distillation. No paraphrase. The structured annotation layer (this archive's §C onward) sits beside the original, never replaces it.

### §A.1 Conversational opener (user-relayed external audit)

```text
你失望是合理的。
我这次不把它当成"TB-18 的一个审计问题"，而把它定性为：

> TuringOS 当前 M1 benchmark 不是完整图灵机式 tape 运行，
> 而是"最终结果快照 + 私有 evaluator 过程"。
> 这违反了你最初的设计：Agent 的有意义活动必须发生在 tape 上，
> 能够被 read/write/predicate/ledger 审计，而不是只在 evaluator
> 内存和 stdout 中消失。

我已经把完整修复方案和一个沙盒不变量测试写成本地文件：

[下载完整修复方案](sandbox:/mnt/data/TuringOS_TB18_Tape_Restoration_Repair_Plan_2026-05-06.md)
[下载沙盒不变量测试脚本](sandbox:/mnt/data/tb18_tape_invariant_harness.py)

我也在 sandbox 里运行了这个测试脚本。它验证了一个核心不变量：

externalized_llm_lean_attempt_count
==
L4 WorkTx attempt count + L4.E real WorkTx rejection count

当前 M1 形态下，P38 / P49 失败；修复后的模拟形态通过。执行时系统有
一个 artifact_tool warmup warning，但测试脚本本身正常完成并输出
预期 PASS/FAIL。
```

### §A.2 §1 Verdict (verbatim)

```text
# 1. 总裁决：TB-18 M1 必须 VETO，不能 OBS ship

我给出明确裁决：

TB-18 M1 benchmark 当前形态 = VETO
不得 ship
不得继续 M2 / M3
不得把这个结果包装成 formal benchmark evidence

原因很简单：

P49:
  evaluator 发生 32 次 LLM-Lean cycle；
  tool_dist = step:31 + omega_wtool:1；
  failed_branch_count = 31；
  但 ChainTape L4 只有 1 个 WorkTx；
  L4.E 真实 LLM rejection = 0。

审计材料明确写出：P49 有 32 次内部迭代，L4 只有 1 个 WorkTx，真实
LLM-vs-Lean rejection 没有进入 L4.E；13 个 CAS 对象中也没有任何
对象代表 31 个中间 LLM-Lean iteration。

这不是"证据包装不够好"。
这是：

> Tape 被截肢。
```

### §A.3 §2 Why constitution-class (verbatim)

```text
# 2. 为什么这是宪法级问题

宪法的基础运行逻辑不是：

Agent 私下尝试 N 次
-> 最终成功结果上链 1 次

而是：

Q_t
-> rtool
-> Agent δ
-> output / proposal
-> predicates
-> wtool
-> Q_{t+1}

无损宪法把 Flowchart 1 定义为基础运行循环，并保留原图哈希校验；
这个流程要求外部化 proposal 经 predicates / wtool 后进入状态循环，
而不是只保留最终快照。

宪法还明确说，顶层白盒的职责不是理解黑盒内部高维推理，而是把行为
结果压缩成确定性低维信号：量化、广播、屏蔽。谓词是 0/1 判定机器，
负责确定边界。

现在 M1 的问题是：

真实 Lean predicate 的大部分判定没有进入 durable record。

审计材料指出，P49 的 12 个 step_reject 只存在于 evaluator stdout 的
tool_dist["step_reject"] = 12，而 L4.E 只有 2 条 atom3 synthetic
fixture，均非真实 LLM rejection。

这直接违反：

布尔谓词必须成为白盒 durable signal；
失败证据必须可审计；
Markov / Information Loom 必须能从日志归档中学习。

更严重的是，审计材料指出 P23 一次解出和 P49 32 次迭代在 L4 上都呈现
同样的 5-tx 形态：

TaskOpen
EscrowLock
Work
Verify
FinalizeReward

这让价格、奖励、审计、未来 NodeMarket 都无法区分：

一次干净成功
vs
32 次碰撞后拼出结果

这就是 Goodhart 层级错位：系统只看到了最终 form compliance，而看
不到真正的计算轨迹。
```

### §A.4 §3 Protocol layer is open; LLM driver truncated tape (verbatim)

```text
# 3. 这不是协议层不能做，而是 LLM driver 没接 tape

审计材料也给出了反证：comprehensive_arena 已经能在单链里产生 13 类
typed tx，说明 protocol / sequencer / CAS 层并非瓶颈；缺陷在 LLM-driver
path，而不是 TuringOS kernel 本身。

这点非常重要，因为它说明修复方向不是重写整个系统，而是：

修 evaluator 的 per-LLM-call externalization。

审计材料指出当前 evaluator.rs 的 per-iteration loop 内部只 increment
tool_dist，没有在 step / step_partial_ok / step_reject / parse_fail 处
调用 bus.submit_typed_tx；实际成功路径上的 submit_typed_tx 只在最终
composite proof 后发生一次。

所以：

协议层开放；
driver 层截断 tape。
```

### §A.5 §4 Freeze list (verbatim)

```text
# 4. 必须立即冻结的内容

在修复之前，必须冻结：

M2 / M3 / 大规模 MiniF2F
NodeMarket
PriceIndex 进一步 claim
Polymarket market signal
任何正式 benchmark report
真实世界 readiness

因为当前 denominator 不可信：

evaluator tx_count = 32
ChainTape attempt count = 1

只要这个差距存在，后续所有价格、H-VPPU、NodeMarket、autopsy、Markov
学习都会基于错误的 tape。
```

### §A.6 §5..§17 (full repair plan, verbatim)

> The remaining sections of the original delivery (§5 New TB / §6 New core defs / §7 Repair architecture / §8 L4 vs L4.E routing / §9 parent_attempt_tx + branch DAG / §10 final composite proof / §11 chain_derived_run_facts / §12 audit_tape extension / §13 New ship gates / §14 Code modification points / §15 Forbidden fixes / §16 Sandbox test results / §17 AI-coder execution incantation) are preserved in the parallel repair-plan markdown reproduced in §B below to avoid duplication. §A.1..§A.5 above are the conversational verbatim; §B is the formal markdown verbatim. Together they form the lossless archive.

### §A.7 §18 Final judgment (verbatim)

```text
# 18. 最终判断

你说"这完全不是我需要的图灵机"，这句话是准确的。

当前 M1 的形态是：

Agent 在内存里试错；
最后把一个 composite proof 上链；
链上看起来像一次干净成功。

而你要的 TuringOS 是：

Agent 每次外部化尝试都在 tape 上留下痕迹；
白盒谓词逐次判定；
失败进入 L4.E；
成功进入 L4；
后续价格、Markov、Autopsy、NodeMarket 都基于真实 tape。

这次必须严肃修复。
不修这个，后面所有 MiniF2F 大规模测试、Polymarket 机制、真实世界
readiness 都会建立在错误的 tape 粒度上。
```

---

## §B. Verbatim Repair Plan Markdown (lossless, as delivered)

> Filename in original delivery: `TuringOS_TB18_Tape_Restoration_Repair_Plan_2026-05-06.md`. Content reproduced unchanged below.

```markdown
TuringOS TB-18 M1 Tape Non-Externalization — Constitution VETO Repair Plan

0. Verdict
This is a VETO-class failure.
The M1 driver collapses N externalized LLM-Lean cycles into one final WorkTx on ChainTape. For P49, evaluator telemetry reports 32 LLM-Lean cycles, but L4 contains exactly one WorkTx and L4.E contains zero real LLM-vs-Lean rejections. That is not a Turing machine tape; it is a final-state snapshot.
The repair is not to tweak dashboard reporting. The repair is to make every externalized LLM-Lean cycle become a durable Attempt Node represented in ChainTape/CAS:
predicate pass  -> L4 accepted WorkTx / VerifyTx evidence;
predicate fail  -> L4.E rejection evidence with Lean stderr/proof artifact CID;
parse/sorry/tool failure -> L4.E or EvidenceCapsule with O(1) L4 anchor if batched.
For TB-18 benchmark, because the user's design intent is meaningful activity on tape, the default repair must be per-call ChainTape externalization, not only aggregate EvidenceCapsule.

1. Constitutional grounding
Flowchart 1 — Runtime loop
The runtime loop is:
Q_t -> read tool -> Agent -> output/proposal -> predicates -> write tool -> Q_{t+1}
A solver's externalized proposal is not private thought. Once the evaluator sends it to Lean or lets it influence later prompt/context/proof state, it has become an externalized attempt. It must have durable tape representation.
Flowchart 2 — Boot / full loop
The benchmark run must be reconstructable from genesis + ChainTape + CAS. If 31 failed LLM-Lean attempts exist only in evaluator stdout, the run cannot be reconstructed from tape.
Flowchart 3 — Meta architecture
ArchitectAI / Information Loom can only learn from logs that are white-box adjudication outputs. If Lean failures are only evaluator-private counters, Markov learning cannot fire.

2. What is wrong in the current M1 evidence
For every SOLVED problem, the L4 shape is ceremonial:
TaskOpenTx
EscrowLockTx
WorkTx
VerifyTx
FinalizeRewardTx
This is identical for one-shot and 32-step solved runs. That means price, reward, audit, and later NodeMarket cannot distinguish:
one clean solve;
32 attempts with 31 failed branches;
repeated Lean collisions;
parse failures;
sorry attempts.
The final composite proof contains textual residue of iteration, but the iteration trajectory itself is not on tape.

3. New canonical distinction
Private thought
Private chain-of-thought remains private. It is not recorded.
Externalized attempt
A model output that is parsed, checked by Lean, used as a proof prefix, used to build a composite proof, or used to influence future prompt context is an externalized attempt. It must be represented.
State node
A predicate-passing externalized attempt accepted into L4.
Rejection evidence node
A predicate-failed externalized attempt recorded in L4.E.
Evidence capsule
High-volume logs may be compressed into CAS and anchored by one L4 terminal tx, but this is for run-level exhaust. It cannot replace per-call attempt records when each call affects proof search or future context.

4. Repair architecture
4.1 AttemptEnvelope
Introduce an evaluator-side structure created before every Lean check.
pub struct AttemptEnvelope {
    pub attempt_id: TxId,
    pub run_id: RunId,
    pub task_id: TaskId,
    pub agent_id: AgentId,
    pub branch_id: BranchId,
    pub parent_attempt_tx: Option<TxId>,
    pub attempt_index: u64,
    pub prompt_context_hash: Hash,
    pub candidate_payload_cid: Cid,
    pub candidate_kind: AttemptKind,
    pub emitted_at_round: u64,
}
4.2 AttemptKind
pub enum AttemptKind {
    Step,
    StepPartialOk,
    StepReject,
    ParseFail,
    SorryBlock,
    OmegaWtool,
}
4.3 AttemptTelemetry CAS object
Every attempt writes a CAS object:
pub struct AttemptTelemetry {
    pub attempt_id: TxId,
    pub run_id: RunId,
    pub task_id: TaskId,
    pub agent_id: AgentId,
    pub branch_id: BranchId,
    pub parent_attempt_tx: Option<TxId>,
    pub attempt_index: u64,
    pub prompt_context_hash: Hash,
    pub candidate_payload_cid: Cid,
    pub lean_result_cid: Option<Cid>,
    pub outcome: AttemptOutcome,
    pub token_counts: TokenCounts,
    pub tool_name: String,
}
4.4 LeanResult CAS object
Every Lean check writes:
pub struct LeanResult {
    pub attempt_id: TxId,
    pub exit_code: i32,
    pub verified: bool,
    pub stderr_cid: Option<Cid>,
    pub stdout_cid: Option<Cid>,
    pub proof_artifact_cid: Option<Cid>,
    pub error_class: Option<LeanErrorClass>,
}
4.5 L4 / L4.E routing
For each externalized attempt:
Lean pass / partial accepted according to predicate policy:
create WorkTx with proposal_cid -> AttemptTelemetry;
create VerifyTx if oracle verification passes;
accept into L4.
Lean reject / parse fail / sorry block:
create WorkTx attempt with proposal_cid -> AttemptTelemetry;
route to L4.E with rejection_class = LeanFailed / ParseFailed / SorryBlocked;
public_summary contains low-pollution summary;
raw stderr stays in CAS with shielding.
4.6 Parent chain
parent_attempt_tx is the previous externalized attempt on the same branch. For P49, expected:
attempt_0 parent=None
attempt_1 parent=attempt_0
...
attempt_31 parent=attempt_30
If the driver branches, branch_id and parent edges must represent the branch.
4.7 Final composite
Final composite proof may still be stored as final proof artifact, but it cannot be the only proposal payload if prior attempts influenced it. The final WorkTx must reference:
final proof artifact CID;
attempt_chain_root;
all constituent attempt IDs or a Merkle root over them.

5. Required code changes
5.1 evaluator.rs
At the per-iteration loop boundary, before Lean check:
Write candidate payload to CAS.
Create AttemptEnvelope.
After Lean check, write LeanResult CAS.
Write AttemptTelemetry CAS.
Submit typed tx:
accepted WorkTx/VerifyTx for successful attempt;
rejected WorkTx/L4.E for failed attempt.
The counter increments at step / step_partial_ok / step_reject / parse_fail must be paired with an attempt record.
5.2 chain_derived_run_facts.rs
Replace summary-only facts with tape-derived facts:
tx_count_tape_attempts
l4_work_attempt_count
l4e_work_attempt_count
attempt_count_from_cas
evaluator_reported_tx_count
equality check
Ship gate:
evaluator_reported_tx_count == l4_work_attempt_count + l4e_work_attempt_count
5.3 audit_tape
Add assertion layer:
attempt_count_eq_evaluator_tx_count
lean_result_cids_resolve
failed_attempts_have_l4e
partial_ok_attempts_have_proof_artifacts
final_composite_references_attempt_chain
random_attempt_sampling
5.4 dashboard
Dashboard should render:
attempt DAG;
accepted state nodes;
rejection evidence nodes;
Lean error distribution;
golden path;
failed branch count derived from ChainTape, not evaluator stdout.
5.5 EvidenceCapsule
EvidenceCapsule remains useful for O(N) compression, but cannot substitute per-attempt externalization in M1 benchmark. It is used to package logs and produce summaries; the attempt nodes still remain chain-addressable.

6. New tests
Unit tests
attempt_envelope_serializes
lean_result_cas_resolves
failed_attempt_routes_to_l4e
accepted_attempt_routes_to_l4
attempt_parent_chain_linear
parse_fail_routes_to_l4e
sorry_block_routes_to_l4e
Integration tests
p49_like_32_attempts_produce_32_attempt_nodes
tx_count_equals_attempt_nodes
solved_one_shot_has_1_attempt_node
solved_multi_iteration_has_n_attempt_nodes
unsolved_run_has_failed_attempt_nodes
final_composite_references_attempt_chain
Tamper tests
tamper_attempt_payload_detected
tamper_lean_stderr_detected
tamper_attempt_chain_root_detected
tamper_final_composite_without_attempts_detected

7. New ship gates
SG-TAPE-1:
Every externalized LLM-Lean cycle produces a CAS AttemptTelemetry object.
SG-TAPE-2:
Every AttemptTelemetry has either L4 accepted WorkTx or L4.E rejection evidence.
SG-TAPE-3:
evaluator tx_count equals ChainTape attempt count.
SG-TAPE-4:
P49-like heavy solved run has 32 attempt records, not 1.
SG-TAPE-5:
Real Lean rejects appear in L4.E, not only evaluator stdout.
SG-TAPE-6:
Markov capsule can derive failure clusters from L4.E / AttemptTelemetry, not evaluator self-report.
SG-TAPE-7:
Audit sampler can sample mathematical content attempts, not only five ceremonial gates.
SG-TAPE-8:
Final composite proof references its attempt chain.
SG-TAPE-9:
Dashboard regenerates attempt DAG from ChainTape + CAS alone.

8. Revised TB-18 sequencing
TB-18 must pause benchmark expansion and insert:
TB-18R — Tape Restoration
Atom R0:
  Charter + stop M1 ship under VETO.
Atom R1:
  AttemptEnvelope + AttemptTelemetry + LeanResult schemas.
Atom R2:
  evaluator loop externalization at each LLM-Lean cycle.
Atom R3:
  L4/L4.E routing for attempt pass/fail.
Atom R4:
  chain_derived_run_facts and audit_tape attempt assertions.
Atom R5:
  dashboard attempt DAG.
Atom R6:
  rerun P23 / P38 / P49.
Atom R7:
  rerun M0 small batch.
Atom R8:
  Codex + Gemini external audit.
Only after TB-18R passes can M1/M2 benchmark resume.

9. What not to do
Do not ship M1 with OBS only.
Do not call 5 ceremonial txs a tape.
Do not store 31 failed attempts only in evaluator stdout.
Do not compress all attempts into final composite WorkTx.
Do not bypass Lean failures into private logs.
Do not continue M2 benchmark before TB-18R.
Do not use NodeMarket or price signals until attempt denominator is fixed.

10. AI coder execution order
STOP current TB-18 ship.
Open TB-18R charter.
Implement R1/R2/R3 before any more benchmark scaling.
Rerun P49 and require:
   tx_count=32,
   chain_attempt_count=32,
   l4e contains real Lean failures,
   final proof references attempt chain.
Only then resume M1/M2.
```

---

## §B. Verbatim Sandbox Invariant Harness (lossless, as delivered)

> Filename in original delivery: `tb18_tape_invariant_harness.py`. Reproduced unchanged below.

```python
#!/usr/bin/env python3
'''
Sandbox invariant harness for TuringOS TB-18 Tape Restoration.

This does not test the GitHub repo directly. It tests the constitutional invariant:
externalized LLM-Lean cycles must match ChainTape attempt records.

Run:
  python tb18_tape_invariant_harness.py
'''

from dataclasses import dataclass
from typing import Dict

@dataclass
class RunFacts:
    name: str
    evaluator_tx_count: int
    tool_step: int
    tool_omega_wtool: int
    l4_work_count: int
    l4e_real_work_rejections: int

def attempt_count_on_tape(r: RunFacts) -> int:
    return r.l4_work_count + r.l4e_real_work_rejections

def check_invariant(r: RunFacts) -> Dict[str, object]:
    expected = r.evaluator_tx_count
    observed = attempt_count_on_tape(r)
    return {
        "run": r.name,
        "expected_externalized_attempts": expected,
        "observed_chain_attempts": observed,
        "pass": expected == observed,
        "missing_attempts": max(expected - observed, 0),
    }

# Current audited facts from TB-18 M1 report hypothesis
current_p23 = RunFacts("P23 one-shot current", 1, 0, 1, 1, 0)
current_p38 = RunFacts("P38 multi-iteration current", 16, 15, 1, 1, 0)
current_p49 = RunFacts("P49 heavy current", 32, 31, 1, 1, 0)

# Desired post-repair facts
fixed_p23 = RunFacts("P23 one-shot fixed", 1, 0, 1, 1, 0)
# Suppose P38 has 10 accepted/partial attempt states and 6 rejects
fixed_p38 = RunFacts("P38 multi-iteration fixed", 16, 15, 1, 10, 6)
# P49: 19 accepted/partial nodes and 13 rejects, total 32
fixed_p49 = RunFacts("P49 heavy fixed", 32, 31, 1, 19, 13)

runs = [current_p23, current_p38, current_p49, fixed_p23, fixed_p38, fixed_p49]

if __name__ == "__main__":
    results = [check_invariant(r) for r in runs]
    for res in results:
        status = "PASS" if res["pass"] else "FAIL"
        print(f"{status}: {res['run']} expected={res['expected_externalized_attempts']} observed={res['observed_chain_attempts']} missing={res['missing_attempts']}")
    assert check_invariant(current_p23)["pass"] is True
    assert check_invariant(current_p38)["pass"] is False
    assert check_invariant(current_p49)["pass"] is False
    assert check_invariant(fixed_p23)["pass"] is True
    assert check_invariant(fixed_p38)["pass"] is True
    assert check_invariant(fixed_p49)["pass"] is True
    print("Invariant harness behaved as expected.")
```

---

## §C. Structural verification (annotation layer; non-canonical)

> The following is *Claude's* annotation, not the original directive. It exists to record what was empirically verified in the v4 source tree at archive time, so future readers can distinguish auditor claims from code-grounded facts.

### §C.1 evaluator.rs per-iteration loop verified directly

Source: `experiments/minif2f_v4/src/bin/evaluator.rs` (HEAD `15b662c` at archive time).

| Path | tool_dist increment | submit_typed_tx? | L4/L4.E destination | Annotation |
|---|---|---|---|---|
| `omega_wtool` (atom3-omega) | line 2317 | line 2457 (WorkTx) + 2500 (VerifyTx) | L4 accepted | Per-tactic ω externalized correctly |
| `omega_wtool` (atom3-omega-pertactic) | line 2861 | line 2999 (WorkTx) + 3046 (VerifyTx) | L4 accepted | Per-tactic ω externalized correctly |
| `step_partial_ok` (intermediate accept) | line 3236 | NO; calls `bus.append_oracle_accepted` (line 3232) | kernel.tape shadow only, NOT L4 | TB-7 §4 known debt: comment in source: "shadow_only ... will be removed when kernel.tape is L4-derived" |
| `step_reject` (Lean rejected tactic) | line 3263 | NO; only `bus.record_rejection` + `tb11_lean_error_count++` | Reputation update + EvidenceCapsule field; **NO L4.E** | Lean stderr lost to evaluator stdout |
| `parse_fail` (LLM output unparseable) | line 3275 | NO; only `bus.record_rejection` + `tb11_protocol_parse_failure_count++` | Reputation update + EvidenceCapsule field; **NO L4.E** | Parse error lost to evaluator stdout |
| `llm_err` (LLM API error) | line 3289 | NO | (none) | Lost to evaluator stdout |
| atom2 chaintape append (line 2161) | implicit | YES (real-signature WorkTx) | L4 accepted | Atom 2 success path externalized |

**Conclusion**: The auditor's strong claim *"per-iteration loop completely lacks submit_typed_tx"* is **partially incorrect** — the success ω-paths externalize correctly. But the auditor's structural verdict is **strengthened** by direct verification: **failure-path asymmetry** is the actual defect, which is worse than uniform truncation because it introduces systematic survivorship bias into Markov / Autopsy / NodeMarket downstream learning.

### §C.2 P49 quantitative claim — not directly verified at archive time

The auditor claims P49: `evaluator_tx_count=32, tool_dist[step]=31, tool_dist[omega_wtool]=1, L4_WorkTx=1, L4E_real_LLM_rejection=0`. The structural code path (§C.1) makes this claim **plausible by construction**: 31 step events would all be `step_reject` (no L4.E) or `step_partial_ok` (kernel.tape shadow), 1 omega_wtool would be the only L4-externalized tactic. The exact counts have not been verified against `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/P*/` JSON at archive time; verification deferred to TB-18R Atom R0 charter §evidence-baseline.

### §C.3 memory `feedback_chaintape_externalized_proposal` — does NOT shield current M1

The memory says: *"1 LLM call → 1 compound payload = 1 Attempt Node, NOT N tactic-level nodes. ChainTape records what the system externalized via submit_typed_tx, not private CoT. Per-tactic decomposition only when system actively makes per-tactic external tool calls (TB-8+ scope)."*

**This DOES NOT contradict the VETO**:
  - The rule's first clause is "1 LLM call = 1 Attempt Node". TB-18 M1 violates this — 32 LLM calls produce ~1 chain Attempt Node on the failure-asymmetric path.
  - The rule's second clause says ChainTape records *what the system externalized via submit_typed_tx*. The current evaluator does **not** call `submit_typed_tx` on `step_reject`/`parse_fail`/`llm_err`, so the rule is not even reaching its enforcement point — the evaluator simply skips external recording.
  - The rule's "per-tactic decomposition is TB-8+ scope" caveat refers to **decomposing one LLM call into N tactic atoms**. TB-18R does **not** decompose. TB-18R asks for **1 LLM call = 1 AttemptTelemetry CAS object** — the rule's primary clause, not the deferred per-tactic caveat.

Conclusion: the memory binds; under it, TB-18R is not "going beyond the rule" — TB-18R is the **first time the rule's primary clause is actually enforced** for failure paths. The current code only enforces it for the omega_wtool success path.

### §C.4 Class-4 carve-out triggered (per `feedback_class4_cannot_hide_in_class3`)

TB-18R repair architecture introduces:
  - **New CAS schema** (`AttemptTelemetry` + `LeanResult` + possibly `AttemptEnvelope`) → new `ObjectType` variants.
  - **L4.E admission expansion** — runtime path (`step_reject`/`parse_fail`/`llm_err`) becomes a sequencer admission source. Pre-TB-18R, L4.E only contained Atom 3 synthetic fixtures.
  - **WorkTx canonical payload** — if `proposal_cid → AttemptTelemetry` semantics changes, this is a canonical payload semantic shift even if ABI bytes don't change.
  - **`chain_derived_run_facts` ship-gate equation** — `evaluator_reported_tx_count == l4_work_attempt_count + l4e_work_attempt_count`. Becomes a hard sequencer-side invariant, not a soft metric.

→ All four touch the `feedback_class4_cannot_hide_in_class3` carve-out enumeration (sequencer admission / typed-tx schema bumps / canonical-signing-payload changes). Therefore TB-18R is **Class 4** and requires **separate ratification** beyond TB-18's blanket-auto-mode authority.

### §C.5 `feedback_no_retroactive_evidence_rewrite` applies — pre-TB-18R evidence is grandfathered

  - Pre-TB-18R M1 evidence (`handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/`) is preserved as-is.
  - No L4 / L4.E / CAS root migration. No relabel of historical results.
  - TB-18R Atom R0 charter §grandfathering will add a README annotation to historical evidence directories: *"This M1 result predates TB-18R Tape Restoration. Per-LLM-call externalization not yet enforced. Failure-path asymmetry present. Do not use as benchmark evidence."*
  - Going-forward only: M1-rerun in Atom R6 produces fresh evidence at `handover/evidence/tb_18r_m1_rerun_<timestamp>/`.

### §C.6 What TB-18R does NOT change

  - constitution.md text (no Phase Z′ rerun needed; TB-18R is enforcement of Art.0 / Art.I.1 / Art.III.1..4, not amendment).
  - Flowchart hashes (no FC element added; existing FC1-N31..N40 cover the runtime loop; TB-18R adds witnesses, not nodes).
  - Restricted-file rule for `src/state/sequencer.rs` and `src/bus.rs` and `src/sdk/tools/wallet.rs` — STEP_B_PROTOCOL still applies if Atom R3 (L4.E routing) touches sequencer.rs admission code.
  - `comprehensive_arena.rs` — already exercises 13/13 typed-tx kinds in single-chain shape per TB-16.x.2.6 / TB-18 Atom B-impl. Not the bottleneck. Not in TB-18R scope.

---

## §D. Audit-order alignment

The auditor's R8 ("Codex + Gemini external audit *after* R6/R7 rerun") aligns with `feedback_audit_after_evidence` (audit AFTER evidence-producing atom). However, per `feedback_class4_cannot_hide_in_class3`, the **charter itself** (Atom R0 → R1 schema gate) is Class 4 and requires a **separate Codex ratification PRE-R1** (not the same as the post-R7 ship audit). TB-18R thus has **two** external audit gates:

  - **Gate 1 (Codex charter ratification, pre-R1)**: Class 4 schema change ratification. Codex VETO → STOP. Codex CHALLENGE/PASS → proceed to R1.
  - **Gate 2 (Codex + Gemini ship audit, post-R7)**: Standard `feedback_dual_audit` Class 3+ pattern; canonical TB-18R ship gate.

Gate 1 dispatched concurrently with charter draft per user authorization 2026-05-06.

---

## §E. Cross-references

  - `handover/tracer_bullets/TB-18_charter_2026-05-05.md` (predecessor charter)
  - `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` (successor charter, drafted concurrently)
  - `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/` (M1 evidence under FREEZE; will receive grandfathering README annotation in Atom R0)
  - `experiments/minif2f_v4/src/bin/evaluator.rs` lines 2317 / 2457 / 2500 / 2861 / 2999 / 3046 / 3236 / 3263 / 3275 / 3289 (per-iteration loop critical sites for R2 atom)
  - `src/state/typed_tx.rs` (typed-tx schema; Class 4 surface for R1 atom)
  - `src/state/sequencer.rs` (sequencer admission; Class 4 surface for R3 atom L4.E expansion)
  - `src/bottom_white/cas/schema.rs` (`ObjectType` enum; Class 4 surface for R1 atom CAS schema additions)
  - constitution.md `Art.0.2` (Tape Canonical) + `Art.I.1` (5-step compile loop) + `Art.III.1..4` (privacy contracts) — invariants TB-18R enforces.
  - `feedback_class4_cannot_hide_in_class3` (memory; carves out separate ratification path)
  - `feedback_chaintape_externalized_proposal` (memory; "1 LLM call = 1 Attempt Node" — TB-18R primary enforcement)
  - `feedback_no_workarounds_strict_constitution` (memory; user explicit anti-workaround stance)
  - `feedback_no_retroactive_evidence_rewrite` (memory; pre-TB-18R evidence grandfathered)
  - `feedback_audit_after_evidence` (memory; G1 Gate 2 audit-order)

---

**End of archive. Annotation layer §C/§D/§E is non-canonical; original delivery §A/§B is canonical and lossless.**
