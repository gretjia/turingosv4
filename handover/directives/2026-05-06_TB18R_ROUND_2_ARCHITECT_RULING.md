---
type: architect_directive
date: 2026-05-06
topic: TB-18 / TB-18R round-2 ruling — process + semantic repair before ship
authority: user-architect (zephryj)
relationship_to_prior:
  - 2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md (parent TB-18 charter)
  - handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md (M1 VETO)
  - handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md (round-2 dispatch)
status: ARCHIVED — awaits user authorization to execute Phase 1 (Process Repair)
layer_1_impact_assessment: NONE (no kernel.rs / DAG / economic-conservation invariants modified by directive itself; directive is process + semantic specification for ongoing TB-18R)
class_assessment: Class 0 (process repair) + Class 4-adjacent (assert_45 / LeanResult schema semantic decision, deferred until FC-first analysis)
---

# TB-18 / TB-18R round-2 architect ruling — 2026-05-06

> **此文件是 lossless archive。原始 user-architect 指令逐字保留。
> 任何后续 distill / annotation 必须在本文件之外另存。**
> Per `feedback_kolmogorov_compression`: original must be exactly reconstructable from this file.

---

## 0. Verbatim original directive (用户原文，未删未改)

> 你的不满是对的。TB-18 拖太久，不是因为"任务复杂所以自然慢"，而是因为它暴露了三层问题同时存在：
>
> ```text
> 1. 技术层：M1 benchmark 的真实 LLM-Lean 过程没有充分成为 tape activity。
> 2. 架构层：原 TB-18 过早从 Formal Benchmark Scale-Up 进入大规模跑题，而 tape granularity 还没修完。
> 3. 流程层：修复 VETO 时出现了"用户一句 fix 被解释为 architect §8 sign-off"的授权漂移。
> ```
>
> 所以我现在给一个严肃结论：
>
> > **TB-18 不能继续以"Formal Benchmark Scale-Up"的名义推进，直到 TB-18R Tape Restoration 完成并通过技术 + 流程双重审计。**
> > 当前所有 R8–R12 修复，只能视为 **round-2 candidate remediation**，不能视为已 ship。
> > 下一步不是继续 M2/M3，而是先把 TB-18R 的流程合法性、assert_45 语义、attempt-level tape granularity 和 audit trail 全部收口。
>
> ---
>
> # 1. 当前 TB-18 的真实状态
>
> ## 1.1 TB-18 原 charter 过大
>
> TB-18 原 charter 是 Formal Benchmark Scale-Up，目标包括：
>
> ```text
> A drive_task + per-LLM-call budget
> B comprehensive_arena
> C deferred-finalize
> D lifecycle-order
> E close OBS_R023
> F single-chain 13/13 evidence
> G dual audit
> H M-ladder benchmark
> ```
>
> 它本身就已经是一个大包：同时包含 evaluator API、budget、finalize、lifecycle、arena、single-chain、benchmark ladder、dual audit。charter 也承认它是 P6 closure validation，不执行 P7 real-world tasks，并且存在 Class 3 默认与 Class 4 carve-out。
>
> 问题是：在这样的大包里，真正的 tape granularity 问题被发现得太晚。
>
> ---
>
> ## 1.2 TB-18 M1 的核心技术 VETO 是真实的
>
> TB-18 M1 发现的核心问题是：
>
> ```text
> LLM 真实发生 N 次外部化 LLM-Lean cycle；
> ChainTape 只看到 1 个最终 WorkTx。
> ```
>
> 审计文件明确说，M1 driver 把 N≥1 个外部化 LLM proposal 压缩成 1 个 ChainTape Work tx；最重的 P49 是 N=32 → M=1，31 倍压缩了 authoritative ledger state。
>
> 这违反了你设计的图灵机直觉：
>
> ```text
> tape 不是最终结果快照；
> tape 是计算过程中的有意义状态活动。
> ```
>
> 更严重的是，外部审计提示本身也要求审计者独立验证这 9 条 claim，并提醒内部 Claude 在同一问题上前两次判断都错过：先说 "5-tx linear chain implies no market mechanism"，再说 "P49 has 19 externalized CoT steps on chaintape"，最后才到当前判断。
>
> 这说明：**根因不是某个单测错了，而是系统和审计流程都没有足够早地捕捉 tape granularity。**
>
> ---
>
> ## 1.3 Round-2 修复目前仍不能直接 ship
>
> TB-18R G2 round-2 dispatch 说明，round-1 VETO 后做了 R8–R12 修复，并在 HEAD `eb2b932` 进入 round-2 审计。但这个 dispatch 自己明确说：它不是 rubber-stamp，而是要联合审查两类对象：
>
> ```text
> 1. fixes 是否正确；
> 2. fixes 的生成流程是否合规。
> ```
>
> 它还明确说明 conservative ranking 仍然是：
>
> ```text
> VETO > CHALLENGE > PASS
> ```
>
> 并且如果任一维度 VETO，merged verdict 就是 VETO。
>
> 所以目前不能说 TB-18R 已经过。
> 只能说：
>
> ```text
> TB-18R round-2 candidate remediation 已提交。
> 最终 verdict 仍取决于 Q1–Q15 + Q-P1–Q-P6。
> ```
>
> ---
>
> # 2. TB-18 为什么拖太久：根因审计
>
> ## 根因 1：TB-18 原目标混入太多架构 debt
>
> TB-18 原本应该只是 Formal Benchmark Scale-Up，但它实际承载了：
>
> ```text
> PRE-17.6 single-chain deviation
> PRE-17.7 Markov β-A
> OBS_M0 DeepSeek drift
> OBS_R023 hardcoded outcome
> deferred-finalize
> lifecycle-order
> arena
> M-ladder benchmark
> ```
>
> 这相当于把多个"前置系统债"塞进一个 benchmark TB。
>
> 结果是：
>
> ```text
> benchmark 还没开始稳定，先暴露 tape granularity；
> tape granularity 修复又触发 assert_45；
> assert_45 修复又触发 §8 sign-off / FC-first process issue；
> round-2 又变成 process + fix joint review。
> ```
>
> 因此 TB-18 时间长的第一根因是：
>
> > **TB-18 被设计成 benchmark TB，但实际上承担了 architecture repair TB。**
>
> ---
>
> ## 根因 2：你真正要的 tape activity 没有在 TB-18 charter 最前面变成 hard gate
>
> 你后来明确说："没有 tape，不在 tape 上进行有意义 activity，就不是我的设计初衷。"
>
> 这是最本质的。
> TB-18 原 plan 虽然有 M-ladder、single-chain、arena，但没有把下面这个不变量放在第一 ship gate：
>
> ```text
> externalized_llm_lean_attempt_count
> ==
> L4 WorkTx attempt count + L4.E real WorkTx rejection count
> ```
>
> 结果 M1 可以跑出：
>
> ```text
> P23 one-shot: tx_count=1, chain=1  看起来通过
> P38/P49 multi-iteration: tx_count=16/32, chain=1  实际失败
> ```
>
> 这个问题太晚才在 M1 审计中暴露。
> 如果一开始就是 hard gate，TB-18 不会拖到 round-2。
>
> ---
>
> ## 根因 3：流程上发生了"fix"被解释为 architect §8 sign-off
>
> Round-2 dispatch 自己承认，用户只发了：
>
> ```text
> "fix"
> ```
>
> 然后 Claude 把它解释为 architect path (A) §8 authorization，且没有 `architect-ingest`，也没有 archived architect directive。
>
> 这是流程问题，不是代码问题。
>
> 对于 Class 4 / constitution-sensitive TB，这种做法不可接受。
> 即使修复技术上是对的，流程上也必须补一个 explicit ratification，否则后续每次 VETO 都可能被一句"fix"自动解释成架构授权。
>
> ---
>
> ## 根因 4：assert_45 的修复可能是语义修复，也可能是 workaround
>
> Round-2 dispatch 对 Gap B 说得很清楚：
>
> 原 invariant 是：
>
> ```text
> verified ↔ exit_code == 0
> ```
>
> 但 `step_partial_ok` 写了：
>
> ```text
> LeanResult { exit_code: 0, verified: false, error_class: None }
> ```
>
> 这同时违反了 assert_45 和 LeanResult doc-comment 原来的 `None iff verified == true` 语义。
>
> 现在有两个可能：
>
> ```text
> α 严格对齐：
>   implementation 错了，应改 step_partial_ok，不改 invariant。
>
> β invariant relaxation：
>   partial verdict 是合法第三状态，应改 invariant 和 doc-comment。
> ```
>
> Claude 采用了 β，但 dispatch 自己承认这可能只是 follow Codex round-1 recommendation，而没有先做 FC-first constitutional analysis。
>
> 所以 TB-18 不能只问"tests green 了吗"。
> 必须问：
>
> ```text
> partial verdict 到底是 FC1 proposal flow 的状态，
> 还是 FC2 audit flow 的状态？
> LeanResult 是否应该承载 partial verdict？
> error_class None 是否合宪？
> ```
>
> 如果这个问题没裁定，R8 可能是 workaround。
>
> ---
>
> ## 根因 5：审计报告先于最终审计被命名为 ship report
>
> Round-2 dispatch 明确指出，`TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md` 是在 round-2 dual audit 尚未发生之前由 Claude 写的，标题可能让读者误解为已经 ship。
>
> 这虽然看似命名问题，但在 TuringOS 项目中不是小事，因为：
>
> ```text
> document title = audit trail signal
> ```
>
> 如果还没审计就叫 ship report，会污染后续 handover。
>
> ---
>
> # 3. 当前必须怎么处理 R8–R12
>
> 我给明确裁决。
>
> ## 3.1 R8–R12 不立即 rollback
>
> 不要现在直接 rollback。
> 因为它们可能技术上是正确修复，而且 workspace test 已经到：
>
> ```text
> 1049 passed / 0 failed / 150 ignored
> ```
>
> round-2 dispatch 也记录了 R8 rerun、R9 chain runs、R0 immutability 等证据。
>
> 但它们不能直接成为 ship。
>
> ## 3.2 R8–R12 现在的状态应标为
>
> ```text
> CANDIDATE REMEDIATION — PENDING ARCHITECT PROCESS RATIFICATION + DUAL AUDIT
> ```
>
> 请 AI coder 立即做一个文档修正：
>
> ```text
> rename / prefix:
> TB-18R_G2_ROUND_2_SHIP_REPORT_2026-05-06.md
> ->
> TB-18R_G2_ROUND_2_CANDIDATE_REMEDIATION_REPORT_2026-05-06.md
>
> 或在文件最顶部加：
>
> PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED
> ```
>
> 这是必须的。
>
> ---
>
> # 4. 对 Q-P1–Q-P6 的我的裁决
>
> ## Q-P1：用户 "fix" 是否等于 architect §8 sign-off？
>
> **裁决：不等于。**
>
> `fix` 可以解释为：
>
> ```text
> 允许准备 remediation candidate。
> ```
>
> 不能解释为：
>
> ```text
> architect §8 sign-off 完成。
> ```
>
> 补救：
>
> ```text
> 1. 保留 R8–R12 candidate。
> 2. 立即补写 Architect Ratification Addendum。
> 3. Addendum 逐条列出 R8–R12、process gaps、technical deltas。
> 4. 用户 / 架构师明确批准后，才进入 final audit / ship。
> ```
>
> 不需要自动 rollback，但不能直接 ship。
>
> ---
>
> ## Q-P2：assert_45 应该 α 还是 β？
>
> **裁决：先不要 final ship；必须做 FC-first semantic mini-review。**
>
> 但我倾向于 **β 可以成立**，前提是改名和 schema 明确：
>
> ```text
> step_partial_ok 不是 verified false 的普通 Lean failure；
> 它是 PartialVerdict。
> ```
>
> 所以正确做法不是简单放宽 invariant，而是新增/明确：
>
> ```rust
> pub enum LeanVerdict {
>     Verified,
>     Failed,
>     PartialAccepted,
> }
> ```
>
> 如果不想改大 schema，至少：
>
> ```text
> LeanResult must explicitly encode partial verdict.
> error_class=None is only legal for Verified or PartialAccepted.
> PartialAccepted must have a field:
>   verdict_kind = PartialAccepted
> ```
>
> 否则 `exit_code=0, verified=false, error_class=None` 太模糊，会成为 semantic hole。
>
> 所以：
>
> ```text
> R8 的方向可能对；
> 但当前 β 修复必须补强 typed semantics。
> ```
>
> 如果 R8 只是改 assertion 接受这个三元组，而没有 typed `PartialAccepted`，我会给 **CHALLENGE**，不是 PASS。
>
> ---
>
> ## Q-P3：R3 preflight `[SUPERSEDED]` markers 是否可接受？
>
> **裁决：可以接受，但必须另加 OBS，不应只靠 inline marker。**
>
> 因为 STEP_B preflight 是架构轨迹，后改 `[SUPERSEDED]` 容易被看成 audit-trail surgery。
>
> 补救：
>
> ```text
> 新增 OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md
> ```
>
> 内容：
>
> ```text
> old text
> new §3.5 authority
> why superseded
> no source behavior changed
> ```
>
> inline marker 可保留，但 OBS 必须存在。
>
> ---
>
> ## Q-P4：Q14 carve-out restatement 是 distill 还是 annotation？
>
> **裁决：目前可作为 annotation，但必须补 verbatim quote。**
>
> 因为 `feedback_kolmogorov_compression` 要求不要丢失原文。
> 所以修正：
>
> ```text
> 保留 structured annotation；
> 但在文件中加入原 charter §0.A + VETO :604-609 的 verbatim quote。
> ```
>
> 如果没有 verbatim quote，就算有 citation，也太像 distill。
>
> ---
>
> ## Q-P5：缺少 FC-first trace 是否 invalidates R8？
>
> **裁决：流程上 CHALLENGE，不自动 VETO。**
>
> 因为技术修复可能正确，但流程违反了"先 FC 再修复"。
>
> 补救：
>
> ```text
> 新增 FC-FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT.md
> ```
>
> 必须回答：
>
> ```text
> partial verdict 属于 FC1 还是 FC2？
> LeanResult 是 predicate evidence 还是 audit artifact？
> error_class=None 是否允许？
> R8 是改 audit invariant 还是改 proposal semantics？
> ```
>
> 这个文件写完后，再判断 R8 是否需要 further change。
>
> ---
>
> ## Q-P6：Round-2 ship report 标题怎么处理？
>
> **裁决：必须改。**
>
> 在 round-2 final verdict 前，不允许叫 ship report。
>
> 改为：
>
> ```text
> Candidate Remediation Report
> ```
>
> 或者文件顶部加：
>
> ```text
> PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED
> ```
>
> ---
>
> # 5. TB-18 真正的后续流程
>
> 我建议把 TB-18 当前工作拆成三个阶段。
>
> ---
>
> ## Phase 1：Process Repair — 先把流程修干净
>
> 必须完成：
>
> ```text
> 1. Architect Ratification Addendum for R8–R12。
> 2. Rename/prefix Round-2 Ship Report as Candidate Remediation。
> 3. Add OBS for R3 preflight supersession。
> 4. Add verbatim quote for Q14 carve-out。
> 5. Add FC-first semantic analysis for assert_45 / PartialVerdict。
> ```
>
> 这一步是 Class 0 / process cleanup，但必须在任何 ship 前完成。
>
> ---
>
> ## Phase 2：Technical Semantic Repair — 修 assert_45 语义
>
> 根据 FC-first analysis，二选一：
>
> ### 如果 α：
>
> ```text
> step_partial_ok should not emit LeanResult
> or should emit error_class = PartialVerdict / SorryBlocked-like.
> Revert R8 relaxation.
> ```
>
> ### 如果 β：
>
> ```text
> Introduce explicit PartialAccepted / PartialVerdict semantic.
> Update LeanResult schema/comment/invariant.
> Tests prove:
>   Verified => exit_code=0
>   Failed => verified=false + error_class present
>   PartialAccepted => exit_code=0 + verified=false + verdict_kind=PartialAccepted
> ```
>
> 我的倾向是 β，但要 typed，不要隐式三元组。
>
> ---
>
> ## Phase 3：Technical Tape Validation — 重新跑 P38 / P49 / M0
>
> 完成技术修复后，再跑：
>
> ```text
> P38
> P49
> M0 small batch
> ```
>
> 必须验证：
>
> ```text
> chain_attempt_count == evaluator_reported_tx_count
> id44/id45/id46 PASS
> R4 invariant equation evaluable
> dashboard substantive smoke
> no retroactive M1 evidence rewrite
> ```
>
> 然后才进入 final Codex + Gemini audit。
>
> ---
>
> # 6. TB-18 为什么久：最终根因总结
>
> 可以写成一个简短 root-cause record：
>
> ```text
> ROOT_CAUSE_TB18_DELAY_2026-05-06.md
> ```
>
> 内容：
>
> ## Root cause 1：Scope overload
>
> Formal benchmark scale-up 同时承载 benchmark、single-chain、budget、Markov、finalize、lifecycle、attempt externalization。
>
> ## Root cause 2：Tape granularity was not first-class gate
>
> Per-LLM-call externalization 没有在 TB-18 charter 第一层被定义为 hard ship gate，导致 M1 才发现 N->1 collapse。
>
> ## Root cause 3：Audit process ambiguity
>
> `fix` 被解释成 architect sign-off，造成 remediation authorization 不清。
>
> ## Root cause 4：Partial verdict semantics under-specified
>
> `step_partial_ok` 的 LeanResult 语义没有被 schema 明确表达，导致 invariant 修复可能是 workaround。
>
> ## Root cause 5：Documentation signals prematurely said ship
>
> Round-2 candidate report 被命名为 ship report，污染 audit trail。
>
> ---
>
> # 7. 接下来不能做什么
>
> 在 TB-18R final PASS 前，禁止：
>
> ```text
> M1 public benchmark report
> M2 / M3 scale-up
> TB-19 real-world pilot design
> NodeMarket / PriceIndex claims based on M1
> 任何正式 H-VPPU 结论
> 任何"formal benchmark passed"宣传
> ```
>
> ---
>
> # 8. 对 AI coder 的直接指令
>
> 可以直接发：
>
> ```text
> Architect ruling on TB-18 delay:
>
> 1. TB-18R is not shipped.
> 2. R8–R12 are candidate remediation only.
> 3. Do not rollback yet.
> 4. Do not proceed to M2/M3.
> 5. First repair process:
>
>    a. Rename or prefix TB-18R_G2_ROUND_2_SHIP_REPORT:
>       PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED.
>
>    b. Write Architect Ratification Addendum for R8–R12.
>       User "fix" is not sufficient §8 sign-off.
>
>    c. Write OBS for R3 preflight supersession markers.
>
>    d. Add verbatim quote for Q14 grandfathering carve-out.
>       Do not rely on compressed restatement only.
>
>    e. Write FC-first analysis for assert_45 / step_partial_ok:
>       Is PartialVerdict FC1 or FC2?
>       Is LeanResult a predicate evidence object or audit artifact?
>       Is error_class=None legal?
>
> 6. Then repair assert_45 semantically:
>
>    Preferred beta only if explicit PartialAccepted / PartialVerdict state is represented.
>    Do not allow exit_code=0, verified=false, error_class=None as an untyped loophole.
>
> 7. Re-run P38/P49 and M0 after semantic repair.
>
> 8. Only then run Codex + Gemini final audit.
>
> 9. Keep conservative ranking:
>    VETO > CHALLENGE > PASS.
> ```
>
> ---
>
> # 9. 未来流程怎么避免再拖
>
> ## 9.1 每个 benchmark TB 先跑 denominator preflight
>
> 任何 benchmark 前先跑：
>
> ```text
> N externalized calls
> N attempt nodes
> ```
>
> 不通过就不进入 scale-up。
>
> ## 9.2 每个 VETO fix 必须有 explicit remediation directive
>
> 不能再：
>
> ```text
> user says fix -> AI assumes architect sign-off
> ```
>
> 要有：
>
> ```text
> REMEDIATION_DIRECTIVE_YYYY-MM-DD.md
> ```
>
> 哪怕很短，也必须明确：
>
> ```text
> authorized path
> allowed files
> risk class
> whether rollback required
> ```
>
> ## 9.3 每个 invariant relaxation 必须先 FC-first
>
> 如果修复方式是"放宽 invariant"，必须先问：
>
> ```text
> 是 invariant 错？
> 还是 implementation 错？
> ```
>
> 不能直接听审计建议就改。
>
> ## 9.4 所有 pre-ship report 必须叫 candidate
>
> 只有审计 PASS 后才能叫 ship report。
>
> ---
>
> # 10. 当前最终裁决
>
> ```text
> TB-18 仍在修复中。
> 目前不是失败结束，也不是成功 ship。
> 它处在：
>   VETO -> candidate remediation -> process/semantic ratification pending
> ```
>
> 最短正确路径：
>
> ```text
> Process repair
> -> PartialVerdict semantic repair
> -> P38/P49/M0 rerun
> -> final dual audit
> -> only then ship TB-18R
> -> then resume benchmark scale-up
> ```
>
> 如果这条路径执行，TB-18 可以收口。
> 如果继续把 round-2 candidate 当 ship，后面会持续拖，并且你的"tape 上有意义 activity"会再次被文档层掩盖。

---

## 1. Layer 1 invariant impact

| Layer 1 invariant | Affected by directive? | Notes |
|-------------------|------------------------|-------|
| kernel.rs 零领域知识 | NO | directive requires no kernel domain knowledge change |
| Append-Only DAG | NO | directive prohibits retroactive evidence rewrite (consistent with C-016) |
| Economic conservation | NO | no wallet/market change |

→ **No Layer 1 violation**. Directive may be executed once user authorizes.

## 2. Risk-class breakdown of work items

| Work item | Class | Authorization needed |
|-----------|-------|----------------------|
| Rename / prefix Round-2 Ship Report | 0 (docs) | covered by this directive |
| Architect Ratification Addendum for R8–R12 | 0 (docs) | covered by this directive |
| OBS for R3 preflight supersession | 0 (docs) | covered by this directive |
| Verbatim quote for Q14 carve-out | 0 (docs) | covered by this directive |
| FC-first analysis for assert_45 / PartialVerdict | 0 (docs/analysis only) | covered by this directive |
| Phase 2 semantic repair (LeanVerdict / PartialAccepted) | **Class 4** (LeanResult schema lives in `src/state/typed_tx.rs` STEP_B file list per CLAUDE.md) | requires fresh remediation directive AFTER FC-first analysis |
| Phase 3 P38/P49/M0 rerun | Class 1 (additive evidence) | covered by this directive after Phase 2 lands |
| Final Codex + Gemini audit dispatch | Class 0 (process) | covered by this directive after Phase 3 lands |

→ **Phase 1 (this archive) is Class 0 only**. Phase 2 schema work requires a separate explicit remediation directive once FC-first analysis is written.

## 3. Constraints carried by directive (binding once authorized)

1. **TB-18R status downgrade**: PROVISIONAL SHIPPED → CANDIDATE REMEDIATION until final dual audit PASS.
2. **No retroactive evidence rewrite**: existing M1 evidence must not be edited; annotation only (per `feedback_no_retroactive_evidence_rewrite`).
3. **No rollback of R8–R12** at this stage; preserve workspace 1049/0/150.
4. **Conservative ranking preserved**: VETO > CHALLENGE > PASS in any subsequent audit.
5. **FREEZE expanded**: in addition to existing TB-18 M1/M2/M3 + NodeMarket + PriceIndex + Polymarket-signal + public-chain + real-world-readiness, also frozen:
   - M1 public benchmark report
   - M2 / M3 scale-up
   - TB-19 real-world pilot design
   - NodeMarket / PriceIndex claims based on M1
   - any formal H-VPPU conclusions
   - any "formal benchmark passed" externalization
   Unfreeze trigger: TB-18R **final** ship after Phase 1 → 2 → 3 → final dual audit PASS.
6. **Future TB process upgrades** (sections 9.1–9.4):
   - Benchmark TB must declare denominator preflight as first hard gate.
   - VETO fixes require explicit `REMEDIATION_DIRECTIVE_YYYY-MM-DD.md`.
   - Invariant relaxation requires prior FC-first analysis.
   - Pre-ship reports must be named "candidate", not "ship".

## 4. Standing prohibitions during this remediation cycle

- Do **not** treat user "fix" / "go" / single-word approvals as architect §8 sign-off for any Class 3 / Class 4 work.
- Do **not** edit historical M1 evidence files (replay_report.json, agent_pubkeys.json, etc.) to retro-fit new tape granularity expectations.
- Do **not** ship TB-18R or claim "TB-18R FINAL" until Phase 1 → 2 → 3 → final dual audit chain completes.
- Do **not** start M2/M3 / NodeMarket / TB-19 work in parallel as a "while we wait" hedge.

## 5. Authorization gate

Per skill protocol: **archive complete; awaiting explicit user authorization to begin Phase 1 execution.**

Phase 1 work items (Class 0 docs only, all explicitly enumerated by directive §8):

1. Rename / prefix `handover/audits/G2_TB_18R_ROUND_2_*_2026-05-06.md` ship-report-style title → `PENDING ROUND-2 DUAL AUDIT — NOT SHIPPED` banner. (Need to identify exact filename in repo first.)
2. Write `handover/directives/2026-05-06_TB18R_R8_R12_RATIFICATION_ADDENDUM.md` covering each of R8–R12 with: change scope, technical delta, process gap, ratification request.
3. Write `handover/observations/OBS_TB18R_R3_PREFLIGHT_SUPERSESSION_2026-05-06.md` (or equivalent OBS path) for inline `[SUPERSEDED]` markers.
4. Add verbatim Q14 grandfathering carve-out quote (TB-18 charter §0.A + VETO :604-609) into the relevant Q14 annotation file.
5. Write `handover/directives/FC_FIRST_ANALYSIS_ASSERT45_PARTIAL_VERDICT_2026-05-06.md` answering the four constitutional questions (FC1/FC2 location, LeanResult role, error_class=None legality, audit-vs-proposal scope).
6. Write `handover/post-mortems/ROOT_CAUSE_TB18_DELAY_2026-05-06.md` with the five root causes from §6.

## 6. Cross-reference index

- Parent charter: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`
- Round-2 dispatch this ruling responds to: `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- M1 VETO origin: `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md`
- Provisional ship state to be downgraded: project memory `project_tb_18r_provisional_shipped.md` + `MEMORY.md` Active state line
- TB log to be updated post-ship (NOT now): `handover/tracer_bullets/TB_LOG.tsv`

---

**END OF ARCHIVE — verbatim section above is the load-bearing record.**
