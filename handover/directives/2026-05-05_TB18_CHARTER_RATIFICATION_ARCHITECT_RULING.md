# Architect ruling — TB-18 charter ratification with amendments (2026-05-05)

**Date**: 2026-05-05
**Filed by**: user (architect) into Claude session in response to TB-18 charter draft `handover/tracer_bullets/TB-18_charter_2026-05-05.md` @ commit `d58af31`
**Responds to**: TB-18 charter §3 ratification questions Q1–Q7
**Status**: **RATIFIED-WITH-AMENDMENTS** — TB-18 may start; ordered as Atom 0 (charter amendment) → Atom E → Atom A → H0 → D-design → C → D-if-Class3 → B → F → G0 → H → G1 → ship.
**Compression**: lossless (architect's full §1–§7 reasoning archived verbatim in §B); structured index in §A; Layer-1 + memory cross-references in §C.

---

## §A — Structured index (annotation layer)

### A.1 Top-line verdict (architect §0 verbatim)

> **TB-18 可以启动，但不能按当前原顺序直接跑到 ship。**
> 我批准 **A–H 的总体 envelope**，但要求调整执行顺序、增加一个早期 M0 preflight、改变审计时机，并且把 Atom D 的 Class 4 分叉处理写得更硬。
> 当前 TB-18 的目标应严格定义为：**Formal Benchmark Scale-Up / P6 closure validation**，不是 P7，不是真实世界任务，也不是市场上线。

### A.2 Q1–Q7 verdict ledger

| Q | Subject | Verdict | Binding output |
|---|---|---|---|
| **Q1** | Atom envelope A–H | **KEEP A–H + reorder + add H0 preflight + move final audit after H** | New sequence: Atom 0 → E → A → H0 → D-design → C → D-if-Class3 → B → F → G0 → H → G1 → ship |
| **Q2** | Atom D risk class | **Class 3 default; STOP for Class 4 ratification on sequencer admission / canonical payload / signing payload / challenge-finalize predicates / canonical tx ABI** | Hard rule, not commentary; SG-18.9 enforcement |
| **Q3** | PRE-17.5 Boltzmann ENFORCE inclusion | **EXCLUDE** | M2 Boltzmann observe-only; ENFORCE = separate TB; Class 4 ratification + Phase Z′ + no-price-as-truth proof |
| **Q4** | PRE-17.7 β-A vs β-D | **β-A only; β-D → TB-19+** | NEW SG: TB-18 cannot use α CLI sidecar to fake β-A success — β-A evidence MUST come from chain-derived TerminalSummary / EvidenceCapsule relation |
| **Q5** | M0 retry placement | **TWO positions: H0 early preflight after E+A, AND H final replay** | If H0 fails, do NOT enter B/F/H |
| **Q6** | M-ladder coverage | **M0 + M1 + M2 only** | M3 (controlled-market-enabled) + M4 (public benchmark report) → TB-19+ |
| **Q7** | Dual audit timing | **G0 Codex micro-audit AFTER F BEFORE H + G1 full Codex+Gemini AFTER H** | Final ship audit MUST cover H |

### A.3 New atoms introduced by ruling

| Atom | Position | Class | Purpose |
|---|---|---|---|
| **0** | First | 0 | Charter amendment applying this ruling |
| **H0** | After E+A; before B | 3 | Small M0 preflight: validates per-LLM-call budget, DegradedLLM emission, EvidenceCapsule outcome propagation, external timeout = safety net only. **STOP if fails.** |
| **G0** | After F; before H | 3 | Codex micro-audit on substrate before expensive M-ladder |
| **G1** | After H | 3 | Full Codex + Gemini ship audit covering ALL atoms including H |

### A.4 Hidden issues architect raised that AI-coder did NOT identify (architect §2)

| ID | Issue | TB-18 binding |
|---|---|---|
| **2.1** | G before H is audit order bug — G cannot audit H if G ships first | Reorder mandatory; G0 + G1 split |
| **2.2** | M-ladder without `BenchmarkManifest` produces incomparable evidence | NEW required artifact: `BenchmarkManifest` pinning problem_ids / split / model_name / model_version / temperature / max_tx / per_llm_call_budget / n_agents / seed / Lean version / mathlib commit / TuringOS commit |
| **2.3** | "100+ problems n5" produces evidence-volume problem; TB-7R precedent had `runtime_repo/.git` + `cas/.git/objects` missing → Codex VETO; TB-8/TB-9 codified `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per run | NEW required artifact: `EvidencePackagingPolicy` — full restorable for M0; sampled (random + failure-heavy + solved + unsolved) for M1/M2; aggregate manifest for all runs |
| **2.4** | M0 must include diverse failure modes, not just retry | M0 must exercise: solved + unsolved + LLM degraded/budget cap + Lean failure + EvidenceCapsule emission + no fake accepted |
| **2.5** | `RunOutcome::DegradedLLM` cannot become evidence-skip backdoor | DegradedLLM MUST emit EvidenceCapsule + TerminalSummary + budget counters; no payout; no fake accepted |
| **2.6** | Atom C deferred-finalize = high-risk double-payout path | Idempotency ship gates: same WorkTx cannot receive double FinalizeReward; deferred fires once; pre-ChallengeResolve FinalizeReward rejected/deferred; re-emitted preserves original amount |
| **2.7** | Atom D lifecycle = state-overwrite risk (TB-16.x.2.6 Bankrupt→Expired overwrite precedent); must be append-only history, not single covering field | TaskLifecycle = set / history / enum-with-history; later marker does NOT erase earlier marker |
| **2.8** | "one evaluator process" ≠ "one chain"; Atom B must NOT shell out to per-task subprocesses creating separate chains | One process + one runtime_repo + one CAS + one chain + one Sequencer + one chain writer; multi-task in shared writer |
| **2.9** | M-ladder must explicitly disclaim real-world readiness in benchmark report | SG-18.14 "Formal benchmark only, not real-world readiness." |
| **2.10** | MiniF2F scale-up may surface prompt memorization / benchmark contamination; system benchmark = ChainTape/replay/stability, NOT model SOTA claim | Report MUST disclose contamination risk + system-vs-model benchmark distinction |

### A.5 Functional / Constraint / Ship Gate amendment summary

- **FR**: 10 → 10 (re-numbered + 4 new clauses on idempotency, append-only lifecycle, one-chain enforcement, manifest+packaging)
- **CR**: 8 → 10 (CR-18.8 multi-chain-as-single-chain ban; CR-18.9 hardcoded-terminal-state ban; CR-18.10 no-Boltzmann-enforce-without-ratification carry)
- **SG**: 12 → 16 (SG-18.13 EvidencePackagingPolicy; SG-18.14 real-world disclaimer; SG-18.15 G0 micro-audit gate before H; SG-18.16 G1 final dual audit)

### A.6 17-point direct execution command (architect §6 verbatim — operative)

```text
Architect ruling for TB-18:

1.  Ratify TB-18 charter only with amendments.
2.  Keep A-H envelope, but reorder execution:
    E -> A -> H0 -> D-design -> C -> D-if-Class3 -> B -> F -> G0 -> H -> G1 -> ship.
3.  Q1 KEEP, but audit G must be after H for final ship.
4.  Q2 D is Class3 only if no sequencer admission/canonical payload/signing change.
    If Class4, STOP for ratification.
5.  Q3 EXCLUDE PRE-17.5 Boltzmann ENFORCE.
    M2 is observe-only.
6.  Q4 β-A only; β-D to TB-19+.
7.  Q5 M0 must run early as H0 after E+A.
8.  Q6 M0+M1+M2 only; M3/M4 deferred.
9.  Q7 use G0 Codex micro-audit after F and final dual audit after H.
10. Add BenchmarkManifest and EvidencePackagingPolicy.
11. Add DegradedLLM EvidenceCapsule requirement.
12. Add deferred-finalize idempotency tests.
13. Add lifecycle no-overwrite invariant.
14. comprehensive_arena must be one evaluator process + one runtime_repo + one CAS + one chain.
15. Do not claim real-world readiness.
16. Do not use real funds, public settlement, or Boltzmann enforce.
17. Stop on any Class4 surface, conservation failure, ChainTape bypass, or multi-chain union masquerading as single-chain.
```

### A.7 Architect §7 final judgment (operative summary)

```text
TB-18 的方向正确，但原 draft 还有三个核心风险：
  1. Audit before benchmark evidence；
  2. M0 too late；
  3. Class4 / single-chain / evidence packaging risks under-specified。
```

After amendments, TB-18 becomes:

```text
不是刷题；不是 real-world；不是 market trading；
而是证明 TuringOS 可以在大规模 formal benchmark 下保持：
  ChainTape 连续性，预算控制，terminal-state 区分，
  single-chain evidence，replay determinism，no fake accepted，
  no unresolved evidence gaps。

这才是进入 TB-19 low-risk real-world pilot design 前应该拥有的证据。
```

---

## §B — Architect ruling lossless verbatim

> 基于你贴出的 TB-18 charter draft、TB-17 复盘与前置材料，我的裁决是：
>
> > **TB-18 可以启动，但不能按当前原顺序直接跑到 ship。**
> > 我批准 **A–H 的总体 envelope**，但要求调整执行顺序、增加一个早期 M0 preflight、改变审计时机，并且把 Atom D 的 Class 4 分叉处理写得更硬。
> > 当前 TB-18 的目标应严格定义为：**Formal Benchmark Scale-Up / P6 closure validation**，不是 P7，不是真实世界任务，也不是市场上线。
>
> TB-18 存在的理由本身是对的：TB-17 ship 后不是直接进入真实世界，而是先做 Formal Benchmark Scale-Up；TB-18 应作为 P7 admission 的经验性前置证据，而不是 P7 执行。charter 里也明确写了 TB-18 位于 TB-17 与 TB-19 之间，目标是 Lean proof benchmark 的 chain-backed scale-up，不涉及真实世界、真实资金或 public settlement。
>
> ---
>
> # 1. 对 Q1–Q7 的正式裁决
>
> ## Q1：Atom envelope 是否保留 A–H？
>
> **裁决：保留 A–H，但必须重排执行顺序，并在 Atom H 里拆出一个早期 H0 preflight。**
>
> 你当前 A–H 的功能分解是合理的：
>
> ```text
> A drive_task / per-LLM-call budget / DegradedLLM
> B comprehensive_arena substantive multi-task driver
> C deferred-finalize
> D lifecycle-order configurable
> E close OBS_R023 hardcoded outcome
> F single-chain 13/13 evidence
> G audit
> H M-ladder
> ```
>
> 这些确实对应 TB-18 的核心职责：关闭 OBS_R023、OBS_M0 budget drift、PRE-17.6 single-chain deviation、PRE-17.7 β-A feasibility，并提供 M-ladder benchmark evidence。charter 也明确把 TB-18 定义为 P6 closure validation，不是 P7，目标包括 single-chain multi-task coverage、full-scale formal benchmark、per-LLM-call budget enforcement、in-tape Markov β-A feasibility、OBS_R023 closure 等。
>
> 但我不同意当前推荐顺序：
>
> ```text
> E → A → C → D-design → B → F → G → H
> ```
>
> 尤其是 `G → H` 有问题：如果 H 是 M-ladder benchmark evidence，那么 ship audit 必须覆盖 H；G 不能在 H 之前作为最终 dual audit。
>
> 我批准的顺序是：
>
> ```text
> 0. Charter amendment / ratification
> 1. E — close OBS_R023 first
> 2. A — drive_task + budget + DegradedLLM
> 3. H0 — small M0 preflight immediately after E+A
> 4. D-design — classify lifecycle-order work as Class 3 or Class 4
> 5. C — deferred-finalize
> 6. D-impl — only if Class 3; if Class 4, STOP for ratification
> 7. B — comprehensive_arena substantive driver
> 8. F — single-chain 13/13 evidence
> 9. G0 — Codex micro-audit on substrate before expensive M-ladder
> 10. H — M1/M2 benchmark ladder
> 11. G1 — final Codex + Gemini ship audit
> 12. Ship
> ```
>
> 理由：TB-17 复盘里已经明确承认，M0 harness audit 上一次做晚了；如果 M0 在 TB-17 charter 之前先跑，DeepSeek drift 会更早暴露，OBS_M0 budget 应更早进入 atom A，而不是拖到 TB-18。 所以 TB-18 不能再把 M0 放到最后才跑。
>
> ---
>
> ## Q2：Atom D risk class 怎么定？
>
> **裁决：默认 Class 3 design step；一旦触碰 sequencer admission semantics / canonical payload / signing payload，立即升级 Class 4 并停止。**
>
> 你的默认判断是对的：
>
> ```text
> D = Class 3 OR Class 4
> ```
>
> 但要把规则写得更硬：
>
> ```text
> If Atom D only changes runtime lifecycle ordering policy without canonical schema/signing-payload mutation:
>   Class 3
>
> If Atom D changes:
>   sequencer admission semantics,
>   WorkTx/TerminalSummaryTx canonical payload,
>   signing payload,
>   challenge/finalize admission predicates,
>   or any canonical transaction ABI:
>     STOP immediately.
>     Re-charter as Class 4.
>     Require architect ratification.
> ```
>
> TB-18 charter 里已经写明，Atom D 如果触及 sequencer admission semantics，就必须按 `feedback_class4_cannot_hide_in_class3` 停下；我要求这个不只是注释，而要成为 ship gate。
>
> 如果 Atom D 不能在 Class 3 内完成，而又没有 Class 4 ratification，那么 TB-18 不能声称：
>
> ```text
> single-chain 13/13 fully closed
> ```
>
> 最多只能声称：
>
> ```text
> formal benchmark substrate partially closed;
> lifecycle-order constraint remains Class 4 forward trigger
> ```
>
> ---
>
> ## Q3：PRE-17.5 / Boltzmann ENFORCE 是否纳入 TB-18？
>
> **裁决：默认 EXCLUDE。**
>
> 我接受你的推荐：
>
> ```text
> PRE-17.5 Boltzmann ENFORCE 不纳入 TB-18 默认范围。
> M2 只跑 Boltzmann observe-only。
> ```
>
> 原因：
>
> ```text
> TB-18 是 formal benchmark scale-up；
> 不是 price-mask enforcement；
> 不是 P7 admission runtime；
> 不是真实世界 scheduler enforcement。
> ```
>
> Boltzmann ENFORCE 涉及 sequencer-side admission / read-view control / possible schema semantics，本身可能是 Class 4。TB-18 已经有 A/B/C/D/E/F/H 多个重型 Class 3 工作，不应再叠加 Class 4 surface。
>
> 所以：
>
> ```text
> Q3 = EXCLUDE
> ```
>
> 如果后续确实要做 Boltzmann ENFORCE，应单独 TB，且要求：
>
> ```text
> Class 4 ratification
> Phase Z′ consideration
> explicit no-price-as-truth proof
> ```
>
> ---
>
> ## Q4：PRE-17.7 β-A vs β-D？
>
> **裁决：TB-18 只做 β-A；β-D 推到 TB-19+。**
>
> 你的推荐正确。
>
> TB-18 目标是验证：
>
> ```text
> in-tape Markov inheritance β-A feasibility
> ```
>
> 也就是能从同一 chain / TerminalSummary / EvidenceCapsule 结构中找到 prior capsule 的可行性。
>
> 但完整 β-D pipeline：
>
> ```text
> TerminalSummaryTx -> EvidenceCapsule.markov_capsule_cid
> 成为默认生产继承机制
> ```
>
> 更可能涉及 canonical payload / signing / chain continuation policy，更适合 TB-19+。
>
> 当前 charter 也已经把 β-A feasibility 作为 P6-Exit4，β-D full pipeline carry forward。
>
> 所以：
>
> ```text
> Q4 = β-A only
> ```
>
> 但我要加一个 ship gate：
>
> ```text
> TB-18 不能使用 α CLI sidecar 假装 β-A 成功。
> ```
>
> 也就是说，β-A 证据必须从 chain-derived TerminalSummary / EvidenceCapsule relation 中得到，不得依赖 operator 知道 prior chain filesystem path 的 sidecar。TB-17 复盘里已经指出，现在 run-to-run inheritance 仍依赖 operator 知道 prior chain 的 filesystem 路径，拼错就会 silently treat as genesis；这个问题不能在 TB-18 里再被忽略。
>
> ---
>
> ## Q5：M0 retry 放在哪里？
>
> **裁决：必须有早期 M0 preflight，不要只放在 Atom H 末尾。**
>
> 你的默认是：
>
> ```text
> M0 retry = Atom H sub-stage 1
> ```
>
> 我改为：
>
> ```text
> M0 有两个位置：
>   H0 early preflight after E + A
>   H final replay as part of M-ladder
> ```
>
> 理由很直接：TB-17 复盘已经承认，harness audit 应在 atom 编排之前做，而不是末尾收尾；M0 太晚才暴露 DeepSeek drift，导致 budget enforcement 被推到 TB-18 Atom A。
>
> 所以：
>
> ```text
> E + A 完成后，立即跑 M0-small。
> ```
>
> M0-small 目标不是出 benchmark report，而是验证：
>
> ```text
> per-LLM-call budget enforcement works
> DegradedLLM can be emitted
> EvidenceCapsule outcome propagates
> external timeout is safety net, not primary control
> ```
>
> 如果 H0 失败，不进入 B/F/H。
>
> ---
>
> ## Q6：M-ladder 覆盖范围？
>
> **裁决：M0 + M1 + M2。M3/M4 推到 TB-19+。**
>
> 我接受你的推荐。
>
> TB-18 应覆盖：
>
> ```text
> M0:
>   20 known problems, chain-backed, no market, prove no fake accepted.
>
> M1:
>   50–100 problems, n1/n3, all failures produce EvidenceCapsule, dashboard batch report.
>
> M2:
>   100+ problems, n5, Boltzmann observe-only, no enforce, no price mask.
> ```
>
> 不做：
>
> ```text
> M3 controlled-market-enabled
> M4 public benchmark report
> ```
>
> 因为 M3 会引入 market-enabled behavior，M4 会进入 public reporting / presentation layer；这两个不应与 TB-18 的 substrate validation 混在一起。charter 中引用的 M-ladder 原文也明确 M2 是 multi-agent market-disabled，M3 才是 controlled-market-enabled，M4 才是 public benchmark report。
>
> ---
>
> ## Q7：dual audit 时机？
>
> **裁决：不做 per-atom dual audit；但也不能只在 H 之前做 final audit。**
>
> 我采用折中：
>
> ```text
> G0:
>   after F, run Codex micro-audit on implementation substrate.
>
> G1:
>   after H, run full Codex + Gemini ship audit.
> ```
>
> 理由：
>
> * 如果等 H 完成后才发现 A/B/C/F substrate 有核心问题，100+ problem M2 可能白跑；
> * 如果每个 atom 都外审，ROI 太低；
> * 所以在 expensive M-ladder 前做一次 implementation checkpoint audit 是性价比最高的。
>
> 因此：
>
> ```text
> Q7 = batched final audit + one pre-H Codex micro-audit
> ```
>
> 不要让 G 在 H 前成为"final dual audit"。
> 如果 H 是正式 M-ladder evidence，最终 ship audit 必须覆盖 H。
>
> ---
>
> # 2. AI coder 没发现或没强调够的问题
>
> 下面这些我认为比 Q1–Q7 更关键，需要写进 TB-18 charter 的 amendments。
>
> ---
>
> ## 2.1 G 在 H 之前是审计顺序 bug
>
> 你现在表格里 Atom G 在 Atom H 前。如果 G 是 dual external audit，而 H 是 M-ladder benchmark report，那么：
>
> ```text
> G before H 不能审计 H。
> ```
>
> 所以要么：
>
> ```text
> G0 = pre-H micro-audit
> H = M-ladder
> G1 = final dual audit
> ```
>
> 要么直接把 G 移到 H 后。
>
> 这是 AI coder 没有明确指出的第一个结构问题。
>
> ---
>
> ## 2.2 TB-18 的真正风险不是"跑不完"，而是"跑出了不可比较的 benchmark"
>
> MiniF2F benchmark 如果没有固定 manifest，就会变成不可比较结果：
>
> ```text
> problem selection drift
> model version drift
> temperature drift
> prompt_context drift
> budget drift
> retry drift
> Lean version drift
> mathlib version drift
> ```
>
> TB-18 必须加：
>
> ```text
> BenchmarkManifest
> ```
>
> 包含：
>
> ```text
> problem_ids
> split
> model_name
> model_version
> temperature
> max_tx
> per_llm_call_budget
> n_agents
> seed
> Lean version
> mathlib commit
> TuringOS commit
> ```
>
> 没有 manifest 的 M-ladder 不能作为正式 evidence。
>
> ---
>
> ## 2.3 "100+ problems n5" 会产生证据体积问题
>
> TB-7R 曾经因为 evidence packaging 缺失 `runtime_repo/.git` 和 `cas/.git/objects` 被 Codex VETO，后来用 tar.gz 证据打包解决。TB-18 的 M2 会生成大规模 CAS/ChainTape，必须预先定义 evidence strategy。
>
> 否则可能出现：
>
> ```text
> 100+ runs produced;
> audit cannot replay committed evidence;
> only human-readable dashboard committed;
> CAS blobs missing.
> ```
>
> 这会重演 TB-7R 的 VETO 类问题。
>
> TB-18 必须写：
>
> ```text
> EvidencePackagingPolicy
> ```
>
> 至少包括：
>
> ```text
> full restorable evidence for M0;
> sampled restorable evidence for M1/M2;
> aggregate manifest for all runs;
> randomly selected replay sample;
> failure-heavy sample;
> solved sample;
> unsolved sample;
> ```
>
> 如果你打算不 commit all CAS for 100+ problems，需要明确：
>
> ```text
> where full evidence lives;
> how auditor can reproduce;
> what subset is committed;
> how subset was selected.
> ```
>
> ---
>
> ## 2.4 M0 不能只是 "retry"，必须验证 failure modes
>
> M0 应刻意包含：
>
> ```text
> solved problem
> unsolved problem
> LLM degraded / budget cap
> Lean failure
> EvidenceCapsule emission
> no fake accepted
> ```
>
> 否则 M0 只是小规模成功样本，不足以验证 TB-18 的预算与 failure plumbing。
>
> ---
>
> ## 2.5 Atom A 的 `DegradedLLM` 不能成为逃避 evidence 的状态
>
> `RunOutcome::DegradedLLM` 必须伴随：
>
> ```text
> EvidenceCapsule
> TerminalSummary
> budget counters
> no payout
> no fake accepted
> ```
>
> 否则它会变成"LLM 不稳定，所以跳过记录"的后门。
>
> TB-18 要加：
>
> ```text
> DegradedLLM must produce evidence.
> ```
>
> ---
>
> ## 2.6 Atom C deferred-finalize 必须避免重复 payout
>
> FinalizeReward 重发是高风险路径。
>
> 需要 ship gates：
>
> ```text
> same WorkTx cannot receive double FinalizeReward
> deferred FinalizeReward after ChallengeResolve fires once
> FinalizeReward before ChallengeResolve is rejected or deferred
> re-emitted FinalizeReward preserves original reward amount
> payout idempotency holds
> ```
>
> 否则 deferred-finalize 会制造双付风险。
>
> ---
>
> ## 2.7 Atom D lifecycle order 必须明确"不以状态覆盖状态"
>
> 之前 TB-16.x.2.6 暴露：
>
> ```text
> FORCE_BANKRUPTCY + FORCE_EXPIRE order causes Bankrupt -> Expired overwrite
> ```
>
> Atom D 不应只是"configurable lifecycle order"，还要明确：
>
> ```text
> state transitions are append-only facts;
> later lifecycle marker does not erase earlier marker.
> ```
>
> 也就是说：
>
> ```text
> TaskLifecycle = set / history / enum-with-history
> ```
>
> 而不是单个可覆盖字段。
>
> 否则又会违反 Append-Only DAG。
>
> ---
>
> ## 2.8 comprehensive_arena 的"one evaluator process"不等于"one chain"
>
> Atom B 要证明的是：
>
> ```text
> one evaluator process
> one runtime_repo
> one CAS
> one chain
> multiple tasks
> ```
>
> 如果它只是一个 process 里启动多个 subprocess，每个 subprocess 自己起 chain，那不合格。
>
> TB-18 要写明：
>
> ```text
> B must not shell out to per-task evaluator processes that create separate chains.
> ```
>
> 除非它是受控 worker，但必须共享同一个 chain writer / Sequencer / runtime_repo。
>
> ---
>
> ## 2.9 M-ladder 不能声称 real-world readiness
>
> 你已经写了 TB-18 不是 P7，但还要在 benchmark report 里强制写：
>
> ```text
> Formal benchmark capacity only.
> Not real-world readiness.
> No real-world domain.
> No real funds.
> No public settlement.
> ```
>
> charter 的 P6-Forbidden 里已经有这个要求，我建议保留并作为 SG-18.8。
>
> ---
>
> ## 2.10 MiniF2F scale-up 可能暴露 prompt memorization / benchmark leakage
>
> 如果使用模型已见过的 MiniF2F 题，结果更像"模型记忆 + prompt route"，不是系统能力。TB-18 不需要解决所有 benchmark contamination，但 report 必须声明：
>
> ```text
> benchmark contamination risk
> no claim of model novelty
> system benchmark = ChainTape/replay/stability benchmark
> not model capability SOTA claim
> ```
>
> 这点 AI coder 没提，但很重要。
>
> ---
>
> # 3. 修正后的 TB-18 atom plan
>
> 我建议把 TB-18 改成：
>
> ## Atom 0 — Charter amendment
>
> ```text
> 修正 Q1–Q7 裁决；
> 加入 hidden issues；
> 加入 BenchmarkManifest；
> 加入 EvidencePackagingPolicy；
> 加入 DegradedLLM evidence gate；
> 加入 deferred-finalize idempotency gate。
> ```
>
> ## Atom E — OBS_R023 closure
>
> ```text
> eliminate hardcoded MaxTxExhausted
> propagate real RunOutcome
> ```
>
> 先做这个，因为它是独立且必须关闭的 caveat。
>
> ## Atom A — drive_task + budget + DegradedLLM
>
> ```text
> drive_task(chain, task_spec)
> per-LLM-call budget
> RunOutcome::DegradedLLM
> EvidenceCapsule on degradation
> ```
>
> ## Atom H0 — M0 preflight
>
> ```text
> 20-ish quick run / smaller if needed
> solved + unsolved + budget failure
> chain-backed
> no market
> ```
>
> 如果 H0 失败，不进入 B。
>
> ## Atom D-design — lifecycle classification
>
> ```text
> decide Class 3 vs Class 4
> ```
>
> If Class 4:
>
> ```text
> STOP.
> architect ratification.
> ```
>
> ## Atom C — deferred-finalize
>
> ```text
> FinalizeReward after ChallengeResolve
> idempotent payout
> no double reward
> ```
>
> ## Atom D-impl — lifecycle order if Class 3 only
>
> ```text
> no overwrite
> append-only lifecycle facts
> ```
>
> ## Atom B — substantive comprehensive_arena
>
> ```text
> one process
> one chain
> ≥6 engineered Lean tasks
> single runtime_repo + CAS
> ```
>
> ## Atom F — single-chain 13/13 evidence
>
> ```text
> one chain
> 13/13 tx kinds
> no multi-chain union
> ```
>
> ## Atom G0 — Codex micro-audit
>
> ```text
> audit substrate before expensive M-ladder
> ```
>
> ## Atom H — M1/M2 ladder
>
> ```text
> M1 50–100 n1/n3
> M2 100+ n5 observe-only
> ```
>
> ## Atom G1 — full dual audit + ship
>
> ```text
> Codex + Gemini
> ```
>
> ---
>
> # 4. TB-18 updated requirements
>
> ## Functional requirements
>
> ```text
> FR-18.1
> drive_task(chain, task_spec) is re-entrant and uses the same ChainTape across tasks.
>
> FR-18.2
> Per-LLM-call budget is enforced and emits RunOutcome::DegradedLLM, not silent timeout.
>
> FR-18.3
> DegradedLLM emits EvidenceCapsule and TerminalSummary.
>
> FR-18.4
> EvidenceCapsule.outcome is propagated from actual RunOutcome, no hardcoded MaxTxExhausted.
>
> FR-18.5
> Deferred FinalizeReward after ChallengeResolve is idempotent and cannot double-pay.
>
> FR-18.6
> Lifecycle ordering avoids destructive overwrite of earlier lifecycle facts.
>
> FR-18.7
> comprehensive_arena runs ≥6 engineered Lean tasks in one evaluator process and one chain.
>
> FR-18.8
> Single-chain 13/13 tx-kind evidence exists in one runtime_repo + one CAS.
>
> FR-18.9
> M-ladder runs M0/M1/M2 under chain-backed benchmark manifest.
>
> FR-18.10
> Benchmark report is generated with reproducible manifest and evidence packaging.
> ```
>
> ---
>
> ## Constitutional requirements
>
> ```text
> CR-18.1
> No real-world execution.
>
> CR-18.2
> No real funds.
>
> CR-18.3
> No public settlement.
>
> CR-18.4
> No ChainTape bypass.
>
> CR-18.5
> All proposal/proof/failure evidence enters ChainTape/CAS or EvidenceCapsule.
>
> CR-18.6
> Dashboard / benchmark report is materialized view, not source of truth.
>
> CR-18.7
> No Class 4 surface hidden inside Class 3 atom.
>
> CR-18.8
> No multi-chain union claimed as single-chain.
>
> CR-18.9
> No hardcoded terminal state.
>
> CR-18.10
> No Boltzmann enforce unless separately ratified.
> ```
>
> ---
>
> ## Ship gates
>
> ```text
> SG-18.1
> drive_task re-entrant API passes deterministic tests.
>
> SG-18.2
> DegradedLLM budget cap produces EvidenceCapsule.
>
> SG-18.3
> Hardcoded MaxTxExhausted literal removed; non-MaxTx outcome propagates.
>
> SG-18.4
> Deferred-finalize idempotency: no double payout.
>
> SG-18.5
> Lifecycle-order does not erase prior lifecycle facts.
>
> SG-18.6
> comprehensive_arena produces ≥6 tasks in one process and one chain.
>
> SG-18.7
> Single-chain 13/13 tx-kind evidence exists.
>
> SG-18.8
> No global Markov pointer introduced.
>
> SG-18.9
> M0 preflight passes before M1/M2.
>
> SG-18.10
> M1 50–100 problems completes with chain-backed evidence.
>
> SG-18.11
> M2 100+ n5 completes with Boltzmann observe-only, no enforce.
>
> SG-18.12
> BenchmarkManifest exists and pins problem set / model / seed / Lean / mathlib / commit.
>
> SG-18.13
> EvidencePackagingPolicy satisfied.
>
> SG-18.14
> Benchmark report contains "formal benchmark only, not real-world readiness."
>
> SG-18.15
> Codex micro-audit after F passes or issues are remediated before H.
>
> SG-18.16
> Final Codex + Gemini audit passes under VETO > CHALLENGE > PASS.
> ```
>
> ---
>
> # 5. 对 Q1–Q7 的最终简表
>
> ```text
> Q1 atom envelope:
>   KEEP A-H, but add H0 preflight and move final audit after H.
>
> Q2 atom D risk class:
>   design decides; Class 4 if sequencer admission/canonical payload/signing changes.
>   If Class 4 -> STOP.
>
> Q3 PRE-17.5 Boltzmann ENFORCE:
>   EXCLUDE. M2 observe-only.
>
> Q4 PRE-17.7:
>   β-A only. β-D TB-19+.
>
> Q5 M0 retry:
>   Do early H0 after E+A, and again inside H ladder.
>
> Q6 M-ladder:
>   M0 + M1 + M2.
>   M3/M4 TB-19+.
>
> Q7 dual audit:
>   G0 Codex micro-audit after F.
>   G1 full dual audit after H.
> ```
>
> ---
>
> # 6. 给 AI coder 的直接执行口令
>
> 可以直接发：
>
> ```text
> Architect ruling for TB-18:
>
> 1. Ratify TB-18 charter only with amendments.
> 2. Keep A-H envelope, but reorder execution:
>    E -> A -> H0 -> D-design -> C -> D-if-Class3 -> B -> F -> G0 -> H -> G1 -> ship.
> 3. Q1 KEEP, but audit G must be after H for final ship.
> 4. Q2 D is Class3 only if no sequencer admission/canonical payload/signing change.
>    If Class4, STOP for ratification.
> 5. Q3 EXCLUDE PRE-17.5 Boltzmann ENFORCE.
>    M2 is observe-only.
> 6. Q4 β-A only; β-D to TB-19+.
> 7. Q5 M0 must run early as H0 after E+A.
> 8. Q6 M0+M1+M2 only; M3/M4 deferred.
> 9. Q7 use G0 Codex micro-audit after F and final dual audit after H.
> 10. Add BenchmarkManifest and EvidencePackagingPolicy.
> 11. Add DegradedLLM EvidenceCapsule requirement.
> 12. Add deferred-finalize idempotency tests.
> 13. Add lifecycle no-overwrite invariant.
> 14. comprehensive_arena must be one evaluator process + one runtime_repo + one CAS + one chain.
> 15. Do not claim real-world readiness.
> 16. Do not use real funds, public settlement, or Boltzmann enforce.
> 17. Stop on any Class4 surface, conservation failure, ChainTape bypass, or multi-chain union masquerading as single-chain.
> ```
>
> ---
>
> # 7. 最终判断
>
> TB-18 的方向正确，但原 draft 还有三个核心风险：
>
> ```text
> 1. Audit before benchmark evidence；
> 2. M0 too late；
> 3. Class4 / single-chain / evidence packaging risks under-specified。
> ```
>
> 修正后，TB-18 会成为真正有价值的 Formal Benchmark Scale-Up：
>
> ```text
> 不是刷题；
> 不是 real-world；
> 不是 market trading；
> 而是证明 TuringOS 可以在大规模 formal benchmark 下保持：
>   ChainTape 连续性，
>   预算控制，
>   terminal-state 区分，
>   single-chain evidence，
>   replay determinism，
>   no fake accepted，
>   no unresolved evidence gaps。
> ```
>
> 这才是进入 TB-19 low-risk real-world pilot design 前应该拥有的证据。

---

## §C — Cross-references

### C.1 Predecessor / authority chain

- **TB-17 audit verdict** (predecessor architect ruling) — `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md` §B.9 + §B.10
- **TB-18 charter draft** (this ruling responds to) — `handover/tracer_bullets/TB-18_charter_2026-05-05.md` @ commit `d58af31`
- **PRE-17.6 deviation §6** (atom envelope source) — `handover/proposals/TB-17_PRE_17_6_COMPREHENSIVE_ARENA_DEVIATION_2026-05-05.md` §6
- **PRE-17.5 design** (forward trigger; EXCLUDE per Q3) — `handover/proposals/TB-17_PRE_17_5_BOLTZMANN_ENFORCE_DESIGN_2026-05-05.md`
- **PRE-17.7 design** (β-A scope per Q4) — `handover/proposals/TB-17_PRE_17_7_INTAPE_MARKOV_DESIGN_2026-05-05.md`
- **OBS_M0 DRIFT** (atom A budget binding) — `handover/alignment/OBS_M0_DEEPSEEK_DRIFT_2026-05-05.md`
- **OBS_R023** (atom E closure target) — `handover/alignment/OBS_R022_TB_16_X_2_2_FIX_EVIDENCE_CAPSULE_HARDCODED_MAXTX_2026-05-05.md`
- **MARKOV_INHERITANCE_POLICY** (β-A target) — `handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md` §4

### C.2 Hidden-issue precedents architect cited

- **TB-7R evidence VETO precedent** — Codex VETO on missing `runtime_repo/.git` + `cas/.git/objects` (commit chain referenced by TB-8/TB-9 charters). Resolution: `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per run (TB-8 charter `handover/tracer_bullets/TB-8_charter_2026-05-02.md` codifies the pattern; TB-9 charter `handover/tracer_bullets/TB-9_charter_2026-05-02.md` re-applies). § 2.3 `EvidencePackagingPolicy` MUST adopt this precedent.
- **TB-16.x.2.6 Bankrupt → Expired overwrite** — § 2.7 `TaskLifecycle = set / history / enum-with-history` mandate. Source: `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md` §Forensic-findings (constraint #2).
- **AI-coder TB-17 复盘 §3.1** ("M0 r1 没在 atom 1-6 之前跑") — § Q5 dual-position M0 mandate. Architect cites this admission as the deciding evidence.

### C.3 Memory bindings affected

- **Existing**: `feedback_minif2f_scaling_policy` (M-ladder M0-M4) + `feedback_class4_cannot_hide_in_class3` (Q2 hard rule) + `feedback_dual_audit` (Q7 G0/G1 split) + `feedback_audit_loop_roi_flip` (per-atom audit rejected) + `feedback_workspace_test_canonical` (cargo workspace) + `feedback_no_workarounds_strict_constitution` (concrete TB-19+ closure) + `feedback_no_retroactive_evidence_rewrite` (going-forward only) + `feedback_kolmogorov_compression` (this archive itself) + `feedback_architect_deviation_stance` (each Q has explicit position) + `feedback_iteration_cap_24h` (Class 2/3/4 caps)
- **NEW** (added this session per architect §2.1 missed-issue): `feedback_audit_after_evidence` — audit (G) MUST come after evidence-producing atom (H), not before; G-before-H is a wrong-target audit and an architect-flagged AI-coder blind spot. **This is the canonical lesson from architect §2.1.**
- **NEW**: `feedback_benchmark_manifest_required` — MiniF2F-style scaled benchmark MUST pin problem_ids / split / model_name / model_version / temperature / max_tx / per_llm_call_budget / n_agents / seed / Lean version / mathlib commit / TuringOS commit. Without manifest = incomparable evidence per architect §2.2.
- **NEW**: `feedback_evidence_packaging_policy_required` — large-scale runs (M2 100+) MUST declare an `EvidencePackagingPolicy` (full restorable for M0; sampled for M1/M2 with random + failure-heavy + solved + unsolved samples; aggregate manifest for all runs). Default to `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per run per TB-7R/TB-8/TB-9 precedent.

### C.4 Operational follow-up after ruling archive

1. Apply 17-point amendments to `handover/tracer_bullets/TB-18_charter_2026-05-05.md` — Atom 0 of executed TB-18.
2. Save 3 NEW memories per §C.3 — Atom 0 sub-task.
3. Commit Atom 0 (Class 0 doc work; charter amendment + ruling archive + memory updates).
4. Surface ready-for-Atom-E state to user; STOP for explicit "go" before evaluator.rs:2940-3137 source change.
5. Atom E onward = Class 2 / Class 3 source work; per-atom user authorization expected (architect did NOT grant blanket TB-18-ship autonomous authority equivalent to TB-17's "由你负责执行，一直到TB-17 ship").
