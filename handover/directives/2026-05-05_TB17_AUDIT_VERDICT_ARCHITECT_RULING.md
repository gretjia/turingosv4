# Architect ruling — TB-16 closure + TB-17 charter audit verdict (2026-05-05)

**Date**: 2026-05-05
**Filed by**: user (architect) into Claude session in response to `handover/architect-insights/REQUEST_TB_16_CLOSURE_AND_TB_17_AUDIT_2026-05-05.md`
**Responds to**: TB-16 closure prompt + TB-17 charter ratification prompt (`handover/tracer_bullets/TB-17_charter_2026-05-05.md`)
**Status**: RATIFIED — execution authorization granted: "严格按架构师意见执行，直到 TB-17 ship"
**Compression**: lossless (architect's full §0 verdict + §1–§12 reasoning archived verbatim in §B); structured index in §A; Layer-1 + memory cross-references in §C

---

## §A — Structured index (annotation layer)

### A.1 Top-line verdicts (architect §0 verbatim)

> **TB-16 可以 ratify，但必须是 "RATIFY WITH SCOPE LIMITS"。**
> 它足以证明 Controlled Market Smoke Arena 的沙盒能力，但不能被解释为已经完成 P7 真实世界 readiness。
> TB-17 charter 方向正确，但必须补强 FR / CR / Ship Gate，尤其是：真实世界准入、Markov inheritance、Boltzmann observe-vs-enforce、single-chain continuation、oracle/challenge/human escalation、不可逆行为禁令，以及 MiniF2F 大规模测试的准入条件。

### A.2 Q1–Q6 verdict ledger

| Q | Subject | Verdict | Binding output |
|---|---|---|---|
| **Q1** | TB-16 closure | **RATIFY-WITH-AMENDMENT** | TB-16 + TB-16.x.* accepted; β-B/C/D remain forward triggers; claim must be narrowed to "sandbox-controlled market smoke" |
| **Q2** | Smoke + real-LLM evidence | **RATIFY AS CANONICAL SANDBOX EVIDENCE; NOT RATIFY AS REAL-WORLD READINESS** | TB-17 charter must explicitly state: TB-16 smoke = sufficient for sandbox, insufficient for P7 |
| **Q3** | Sub-atom audit asymmetry | **CONCUR** | Class 2 self-audit + Class 3 dual external mix accepted **with carve-out**: Class 4 surfaces (Boltzmann sequencer enforce + WorkTx schema + canonical signing-payload) cannot hide inside Class 3 umbrella |
| **Q4** | OBS_R023 hardcoded MAX_TX deferral | **ACCEPT DEFERRAL, BUT NOT BEYOND TB-18** | Cannot defer past TB-18 pilot design; cannot defer to "after real-world execution" |
| **Q5** | Multi-chain union vs single-chain | **RATIFY AS EXPLICIT DEVIATION; NOT RATIFY AS FULL SINGLE-CHAIN COMPLETION** | TB-17 PRE-17.6 binding: either single-chain 13-of-13 OR architectural-exclusion deviation ratified |
| **Q6** | Anything Claude missed | **4 NEW CONCERNS ADDED** (see §A.5 below) | TB-17 charter must add Q6.1 oracle attack surface / Q6.2 irreversibility subtypes / Q6.3 human escalation timeout / Q6.4 MiniF2F vs real-world distinction |

### A.3 TB-17 charter amendment mandate

Architect §4 verbatim:

> TB-17 charter 方向正确。它已经包含几个关键点 [...] 这些符合要求。但我建议增强 FR / CR / Ship Gate，并对 Atom 7/8/9 的 ratification 边界做更硬的表达。

**Mandated expansion**:
- **FR**: 7 → **14** (FR-17.1..FR-17.14; architect §5 verbatim)
- **CR**: 7 → **14** (CR-17.1..CR-17.14; architect §6 verbatim)
- **SG**: 10 → **20** (SG-17.1..SG-17.20; architect §7 verbatim)

### A.4 Q6 — four concerns Claude missed (architect §3 Q6.1–Q6.4 verbatim)

| Q6.x | Concern | TB-17 binding |
|---|---|---|
| **Q6.1** | Real-world oracle attack surface (manipulation / provenance / replayability / latency / disagreement / challenge evidence) | ORACLE_REQUIREMENTS.md must add §X "oracle attack surface" |
| **Q6.2** | Irreversibility subtypes ≥ 8 (external-API-write / payment / publication / message-sending / physical-actuation / deletion / legal-medical-financial-advice / credential-key-rotation) | IRREVERSIBLE_ACTION_POLICY.md must enumerate ≥8 subtypes; SG-17.8 ≥8 verdicts (was ≥5) |
| **Q6.3** | Human escalation must have timeout + default-safe-action (else escalation can stall forever) | SAFETY_BOUNDARY.md must define `human_escalation_required` + `human_timeout` + `default_safe_action` + `no-settlement-until-resolution`; CR-17.14 fail-safe-not-fail-open |
| **Q6.4** | Large-scale MiniF2F testing ≠ real-world readiness | TB-17 charter must distinguish formal-benchmark-readiness vs real-world-domain-readiness; FR-17.13 + CR-17.13 + SG-17.18 |

### A.5 Atom envelope re-affirmation (architect §8 verbatim)

| Atom | Class | Authorization status |
|---|---|---|
| **Atoms 1-6** (six readiness docs) | **Class 0** | autonomous AI-coder execution (架构师 §8: "Class 0，自主执行") |
| **Atom 7** (PRE-17.5 Boltzmann enforce) | **Class 4 if implemented; Class 0 if design-only deferral** | architect §8 verbatim: "只做 design unless separately ratified"; if implementation → "must stop before code, requires architect ratification, requires Phase Z′ consideration" |
| **Atom 8** (PRE-17.6 comprehensive_arena substantive) | **Class 3** | architect §8 verbatim: "必须双审 [...] 尽量 single-chain 13-of-13 [...] multi-chain union [...] 必须写 architectural-exclusion deviation, 必须获得 architect ratification" |
| **Atom 9** (PRE-17.7 in-tape Markov) | **Class 3 OR escalate to Class 4** | architect §8 verbatim: "先设计文档，再判断是否需要 signing-payload bump [...] 升级 Class 4" |
| **Atom 11** (conformance tests) | **Class 1** | architect §8 verbatim: "至少包括 markov_inheritance_policy tests, fc_alignment_conformance tests, no_global_pointer tests, irreversible_action examples" |
| **Atom 12** (SHIP) | **provisional + architect signature** | architect §8 verbatim: "可以先生成 ready-for-ratification snapshot，但最终 TB-17 不能算完全 shipped，直到 human architect 签署 readiness report" |

### A.6 MiniF2F scaling policy (architect §9 + §10 + §11 verbatim)

**Phase ladder** (architect §9.3 verbatim):

```
M0  Benchmark harness audit:
      20 known problems; chain-backed; no market;
      prove no fake accepted.

M1  Full heldout subset:
      50–100 problems; n1 / n3;
      all failures produce EvidenceCapsule;
      dashboard batch report.

M2  Multi-agent market-disabled:
      100+ problems; n5;
      Boltzmann observe only; no enforce; no price mask.

M3  Controlled market-enabled:
      same problem set; NodePosition / PriceIndex / Autopsy;
      no real funds; sandbox only.

M4  Public benchmark report:
      Only after replay reproducibility +
      no unresolved evidence gaps + workspace tests green + audit pass.
```

**Pre-conditions** (architect §9.1 verbatim):

```
1. 不声称 real-world readiness
2. 不接真实资金
3. 不启用 public settlement
4. 不绕过 ChainTape
5. 所有 proposal / proof / failures 进 ChainTape/CAS 或 EvidenceCapsule
6. dashboard 可重建结果
```

**Sequencing** (architect §10 verbatim):
- §10.1 — **TB-17 必须先完成**; large-scale MiniF2F 可准备 harness 但不算正式 benchmark
- §10.2 — TB-17 后 **第一件事 = TB-18 Formal Benchmark Scale-Up** (full controlled benchmark; chain-backed; no real-world domain; no real money)
- §10.3 — TB-19 才做 low-risk real-world pilot design

### A.7 Final 13-point loop-mode directive (architect §11 verbatim — the operative instruction)

```text
Architect verdict:

1. TB-16 main + TB-16.x closure are RATIFIED WITH SCOPE LIMITS.
2. Controlled Market Smoke Arena verdict accepted as sandbox evidence,
   not real-world readiness.
3. β-A complete. β-B / β-C / β-D remain PRE-17 forward triggers.
4. Multi-chain union accepted only as explicit deviation,
   not single-chain completion.
5. Markov global pointer issue is closed via Option α,
   but TB-17 must enforce no-global-pointer and Markov inheritance policy.
6. TB-17 charter direction is accepted but must be amended with
   expanded FR-17.1..14, CR-17.1..14, SG-17.1..20.
7. Atom 7 Boltzmann enforcement remains Class 4 if implemented;
   design-only deferral is acceptable if ratified.
8. Atom 8 comprehensive_arena must aim for single-chain 13-of-13;
   multi-chain union requires architectural-exclusion deviation.
9. Atom 9 Markov β-D must be design-first;
   signing-payload changes require Class 4 ratification.
10. Large-scale MiniF2F should not be official until TB-17 passes.
11. Controlled small/medium MiniF2F can start as harness prep:
    20 -> 50 -> 100 problems,
    all chain-backed, no real funds, no public chain,
    no real-world readiness claim.
12. Full MiniF2F benchmark should be TB-18 after TB-17 sign-off.
13. TB-17 does not launch real-world tasks.
```

### A.8 Layer 1 invariant impact assessment (per `architect-ingest` step 2)

| Invariant | Impact |
|---|---|
| `kernel.rs` 零领域知识 | None — TB-17 atoms 1-6 are docs only; atoms 7/9 design-doc-only without implementation; atom 8 lives in `experiments/minif2f_v4/`; atom 11 lives in `tests/`. No kernel surface change. |
| Append-Only DAG | **POSITIVE** — atom 11 SG-17.10 enforces no global filesystem pointer source-of-truth (re-affirms OBS_R022 α closure constitutionally); atom 9 (when implemented) extends β path B chain continuation. |
| Economic conservation | None — TB-17 does not touch wallet/market/sequencer arithmetic. PRE-17.5 implementation (deferred unless ratified) would touch `WorkTx` admission semantics; deferral preserves invariant. |

**Verdict**: no Layer 1 violation; ruling is constitutionally additive (FR/CR/SG expansion + atom envelope tightening + MiniF2F scaling policy codification).

### A.9 Memory updates required (post-execution authorization)

- **NEW** `project_tb_16_ratified_with_scope_limits` — TB-16 ratification verdict + scope-limit narrowing (sandbox not P7).
- **NEW** `feedback_minif2f_scaling_policy` — M0-M4 ladder; full benchmark = TB-18 after TB-17 ship.
- **NEW** `feedback_class4_cannot_hide_in_class3` — Class 3 umbrella cannot absorb Class 4 surfaces (Boltzmann enforce / schema bump / signing-payload bump).
- **NEW** `project_tb_17_ratified_charter_2026-05-05` — TB-17 charter direction RATIFIED with FR/CR/SG expansion; atoms 7/8/9 envelope tightening; provisional-then-architect-signed ship pattern.
- **UPDATE** `project_tb_16_x_fix_shipped.md` — append architect verdict reference.
- **UPDATE** `MEMORY.md` — pointer to new entries.

### A.10 Cross-references

- **Source prompt**: `handover/architect-insights/REQUEST_TB_16_CLOSURE_AND_TB_17_AUDIT_2026-05-05.md`
- **TB-17 charter** (target of amendment): `handover/tracer_bullets/TB-17_charter_2026-05-05.md`
- **TB-16 final closure**: `handover/ai-direct/TB-16_FINAL_CLOSURE_2026-05-05.md`
- **Prior architect rulings**:
  - `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` (original TB-17 §8 spec — superseded by this ruling for FR/CR/SG counts)
  - `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md` (PRE-17.1..17.4 + SG-17.9 + SG-17.10 + Markov doc mandate — preserved)
- **Constitution**: `constitution.md` Art. 0.2 (Tape Canonical) + Art. 0.3 (Replay Determinism) + Art. 0.4 (Q_t version-controlled) + Art. II.1 + Art. III.1-4 + Art. IV + Art. V.1
- **Feedback memories driving execution**:
  - `feedback_kolmogorov_compression` — drove Part B verbatim archive
  - `feedback_no_workarounds_strict_constitution` — drove Q1 RATIFY-WITH-AMENDMENT framing (no silencing of β-B/C/D deferral)
  - `feedback_architect_deviation_stance` — drove Q5 explicit-deviation framing
  - `feedback_no_retroactive_evidence_rewrite` — preserves TB-16 chains as-shipped (no rewriting)
  - `feedback_dual_audit` + `feedback_risk_class_audit` — drove Q3 risk-class concur with Class 4 carve-out

---

## §B — Lossless verbatim ruling (architect's full text — §0 + §1–§12)

> 严格按架构师意见执行，直到 TB-17 ship：According to a document from 2026-05-05，TB-17 charter 当前仍是 **DRAFT / pending architect ratification**，其前置状态写明 TB-16.x.2 umbrella 已 shipped，TB-17 是 **P6 → P7 transition gate**，只判断系统是否具备进入 P7/真实世界任务资格，不执行真实世界任务；同一组材料还说明 TB-16 主 charter 已 shipped、经历 Codex/Gemini 多轮审计，后续 TB-16.x.* 关闭了 Markov pointer 等问题，但仍有若干 forward triggers 进入 TB-17。

### §B.0 — 总裁决

我的总裁决：

> **TB-16 可以 ratify，但必须是 "RATIFY WITH SCOPE LIMITS"。**
> 它足以证明 Controlled Market Smoke Arena 的沙盒能力，但不能被解释为已经完成 P7 真实世界 readiness。
> TB-17 charter 方向正确，但必须补强 FR / CR / Ship Gate，尤其是：真实世界准入、Markov inheritance、Boltzmann observe-vs-enforce、single-chain continuation、oracle/challenge/human escalation、不可逆行为禁令，以及 MiniF2F 大规模测试的准入条件。

---

### §B.1 — TB-16 完成情况审计

#### §B.1.1 TB-16 主 charter：接受

我接受 TB-16 主 charter shipped 状态。

材料中写明：

```text
TB-16 main charter — Controlled Market Smoke Arena
Status: SHIPPED at commit 3cd22d4
8/8 SG GREEN
13/13 halt-triggers
verdict = PROCEED
Class envelope = 3
Audit = dual external
R1 BOTH VETO -> R2 Codex VETO + Gemini partial -> R3 fixes -> post-R3 Round 2 PROCEED
```

这说明 TB-16 主体不是轻率通过，而是经过了足够强的 VETO 修复与复审。

所以：

```text
TB-16 main charter = RATIFIED
```

但 claim 必须限制为：

```text
Controlled Market Smoke Arena sandbox is operational.
```

不能扩大为：

```text
Real-world readiness achieved.
Public market ready.
Single-chain multi-task continuation fully solved.
In-tape Markov inheritance fully solved.
Boltzmann sequencer enforcement fully solved.
```

#### §B.1.2 TB-16.x.*：接受，但带 forward triggers

材料显示 TB-16.x.* 包括多个 follow-on sub-atoms：

```text
TB-16.x.1 + .1.5:
  tamper-hang root-cause + audit_tape_tamper hardening

TB-16.x.fix:
  OBS_R022 Option α closure
  删除 / de-canonicalize LATEST_MARKOV_CAPSULE.txt
  PRE-17.1 + PRE-17.3 closed

TB-16.x.2:
  umbrella charter for remaining coverage gaps
```

这说明我之前关于 `LATEST_MARKOV_CAPSULE.txt` 是 Art. 0.2 平行账本的裁决已经被吸收，并且 x.fix 关闭了关键 Markov pointer 问题。

但 TB-16.x.2.6 的材料也诚实列出：

```text
β-A multi-WorkTx + Boltzmann RUNTIME exercise = COMPLETE
β-B Boltzmann sequencer-side ENFORCEMENT = NOT IMPLEMENTED
β-C single continuing chain across multi-task = PARTIAL
β-D in-tape Markov inheritance pipeline = NOT IMPLEMENTED
```

并且指出了三个阻碍 single-chain 13-of-13 的架构约束：challenged WorkTx 不能直接 finalize；FORCE_BANKRUPTCY + FORCE_EXPIRE 顺序会造成状态覆盖；single-task evaluator 一次只能处理一个 Lean problem。

所以：

```text
TB-16.x.* = RATIFIED WITH FORWARD TRIGGERS
```

具体 forward triggers：

```text
PRE-17.5: Boltzmann OBSERVE-vs-ENFORCE, sequencer-side enforcement
PRE-17.6: comprehensive_arena substantive multi-task driver / single-chain continuation
PRE-17.7: in-tape Markov β-D pipeline
```

这些必须进入 TB-17 的 precondition ledger，不能在 TB-17 里被淡化。

---

### §B.2 — Smoke test 与 real LLM test 审计

#### §B.2.1 Controlled Market Smoke Arena：接受为 canonical sandbox verdict

我接受 TB-16 的 controlled market smoke verdict，但范围是：

```text
sandbox-controlled market smoke
```

不是：

```text
real-world market smoke
```

材料中的 TB-16 主 charter 目标本来就是 controlled sandbox：compute、position、price、masking、autopsy，且不开放真实市场、不接真实资金、不接外部域。此前 TB-16 需求也明确要求 no public chain、no real-money market、no external domain、no unbounded leverage、no DPMM/pro-rata。

所以 smoke 通过的意义是：

```text
沙盒中：
  positions / prices / masks / autopsies / market conservation / dashboard / audit_tape
这些机制可以被 ChainTape/CAS 审计。
```

但不能解释成：

```text
真实世界任务已可执行。
真实资金市场已可运行。
P7 可直接开始。
```

#### §B.2.2 real LLM evidence：接受，但不得替代 TB-17

我接受 real LLM 测试作为 TB-16 的强证据，但它只证明：

```text
真实模型能在当前沙盒结构中触发相关行为；
audit_tape 能捕捉对应事件；
dashboard 能展示。
```

它不证明：

```text
真实世界 oracle 已可用；
不可逆操作安全；
human escalation 完整；
public settlement 可行；
high-risk domain 可用。
```

所以 real LLM test 是：

```text
necessary signal
```

但不是：

```text
sufficient gate
```

#### §B.2.3 audit_pipeline_smoke 的 BLOCK：接受为正确检测

材料中写到：

```text
audit_pipeline_smoke verdict=BLOCK
Layer E #27: evidence_capsule_cid not in CAS at L4 index 2
```

并说明这是正确检测，不是 TB-16 bug，后续 TB-16.x.2.5 链已经修复。

我的裁决：

```text
接受这个 BLOCK 作为 audit_tape 正常发挥作用。
```

不要把它记成失败，也不要删掉。它应该保留为：

```text
evidence gap detection precedent
```

这是 TuringOS 需要的系统行为：发现 evidence gap 时 BLOCK，而不是 silently pass。

---

### §B.3 — 对 AI coder Q1–Q6 的裁决

#### Q1 — TB-16 closure ratification

裁决：`RATIFY-WITH-AMENDMENT`

理由：

```text
TB-16 主体与 TB-16.x closure 可以接受；
但 β-B / β-C / β-D 不是已完成，只是 forward-triggered。
```

写入：

```text
TB-16 closure accepted for sandbox-controlled market smoke.
PRE-17.5 / PRE-17.6 / PRE-17.7 remain binding forward triggers.
```

#### Q2 — Smoke + real LLM evidence ratification

裁决：

```text
RATIFY AS CANONICAL SANDBOX EVIDENCE
NOT RATIFY AS REAL-WORLD READINESS
```

要求：

```text
在 TB-17 charter 中明确：
TB-16 smoke evidence is sufficient for Controlled Market Smoke Arena,
but insufficient for P7 real-world execution.
```

#### Q3 — Sub-atom audit asymmetry

裁决：`CONCUR`

TB-16.x 子 atom 采用按风险分级的审计是可以接受的：Class 2 self-audit 与 Class 3 dual external 混合，本来就符合我们之前设定的 risk-class audit 模型。材料也说明 umbrella charter 预先声明了这种 risk-class-by-sub-atom 策略。

但必须保留一条：

```text
Class 4 surfaces cannot be hidden inside Class 3 umbrella.
```

例如：

```text
Boltzmann sequencer-side enforcement
WorkTx schema bump
canonical signing-payload bump
```

必须单独 ratification。

#### Q4 — OBS_R023 hardcoded MAX_TX 是否可 defer

裁决：`ACCEPT DEFERRAL, BUT NOT BEYOND TB-18`

OBS_R023 / hardcoded MAX_TX 是重要问题，但它不应阻塞 TB-17 的文档型 readiness gate。

但必须写入 TB-17：

```text
Known limitation:
  EvidenceCapsule MAX_TX hardcoding must be closed before any real-world pilot.
```

也就是说：

```text
可以 defer 到 TB-18 pilot design；
不能 defer 到真实世界执行之后。
```

#### Q5 — Multi-chain union ≠ single-chain

裁决：

```text
RATIFY AS EXPLICIT DEVIATION
NOT RATIFY AS FULL SINGLE-CHAIN CONTINUATION
```

TB-16.x.2.6 的 multi-chain union 可以作为 sandbox aggregate evidence，但不能声称已经完成：

```text
single continuing chain across multi-task
```

TB-17 必须包含：

```text
PRE-17.6:
  either single-chain 13-of-13 achieved
  OR architectural-exclusion deviation documented and ratified
```

当前 TB-17 charter 已经把 atom 8 fallback 写成"architectural-exclusion deviation with rationale + architect ratification"，这是正确的。

#### Q6 — Claude 是否漏了什么

我认为还漏了四个应在 TB-17 中显式加入的 concern：

##### Q6.1 "Real-world oracle attack surface" 需要单独列项

TB-17 不应只写 oracle requirements，还要写：

```text
oracle manipulation risks
oracle provenance
oracle replayability
oracle latency
oracle disagreement handling
oracle challenge evidence format
```

##### Q6.2 "Irreversible action" 需要细分

不可逆行为不只是"外部动作"。至少包括：

```text
external API write
payment
publication
message sending
physical actuation
deletion
legal/medical/financial advice
credential/key rotation
```

##### Q6.3 "Human escalation" 必须有 timeout 规则

否则系统可能在 escalation 上永久卡住。需要：

```text
human_escalation_required
human_timeout
default_safe_action
no-settlement-until-resolution
```

##### Q6.4 "Large-scale MiniF2F testing" 不等于真实世界 readiness

MiniF2F 是 T2/T2-like formal verification expansion。它可以作为 P6 Epistemic Lab stress test，但不能替代 TB-17 real-world readiness。

TB-17 charter 应明确区分：

```text
large-scale formal benchmark readiness
vs
real-world domain readiness
```

---

### §B.4 — TB-17 charter 审计

#### §B.4.1 总体评价

TB-17 charter 方向正确。

它已经包含几个关键点：

```text
Status = DRAFT / pending architect ratification
TB-17 是第一个 ship 需要 human architect sign-off 的 TB
Phase = P6 -> P7 transition gate
TB-17 不执行 P7 tasks
Risk class = Class 0 / Class 4 hybrid
Class 4 atoms 必须在开始前停下等 architect ratification
```

这些符合要求。

但我建议增强 FR / CR / Ship Gate，并对 Atom 7/8/9 的 ratification 边界做更硬的表达。

---

### §B.5 — TB-17 Functional Requirements — 完整修订版

建议将 TB-17 FR 改为 FR-17.1–FR-17.14。

```text
FR-17.1
Define allowed real-world domain categories.

FR-17.2
Classify candidate domains by verification class:
  T1 easy solve/easy verify,
  T2 hard solve/easy verify,
  T3 hard solve/hard verify,
  T4 deceptive appearance / high Goodhart risk.

FR-17.3
Define oracle requirements for each candidate domain.

FR-17.4
Define oracle provenance requirements:
  source identity,
  timestamp,
  reproducibility,
  hash/CID evidence,
  challengeable data trail.

FR-17.5
Define oracle disagreement policy:
  multi-oracle quorum,
  median/majority rules,
  manual escalation conditions.

FR-17.6
Define challenge window and evidence requirements.

FR-17.7
Define delayed settlement requirements per domain risk tier.

FR-17.8
Define human escalation conditions and timeout/default-safe action.

FR-17.9
Define irreversible-action ban and allowed reversible/compensable actions.

FR-17.10
Define privacy / raw-log / failure-evidence shielding requirements.

FR-17.11
Define Markov inheritance policy for real-world tasks:
  genesis,
  inherited chain,
  prior-chain-runtime-repo,
  no global pointer.

FR-17.12
Define Boltzmann OBSERVE-vs-ENFORCE transition:
  observe-only allowed;
  enforcement requires separate ratification.

FR-17.13
Define formal benchmark scaling policy:
  MiniF2F large-scale tests are formal-domain stress tests,
  not real-world readiness substitutes.

FR-17.14
Define pilot-domain acceptance criteria and reject criteria.
```

---

### §B.6 — TB-17 Constitutional Requirements — 完整修订版

```text
CR-17.1
No real-world domain without explicit oracle design.

CR-17.2
No subjective task without 0/1 predicate or structured evidence plan.

CR-17.3
No irreversible external action.

CR-17.4
No settlement before challenge window closes.

CR-17.5
No price-as-truth.

CR-17.6
No bypass of Human RootBox for high-risk domains.

CR-17.7
No agent-only arbitration.

CR-17.8
No global filesystem pointer as source of truth.

CR-17.9
Markov inheritance must be tape-derived or explicitly imported from prior-chain-runtime-repo.

CR-17.10
Dashboard / readiness reports are materialized views, not source of truth.

CR-17.11
Raw real-world evidence must not be broadcast into ordinary Agent read views.

CR-17.12
Boltzmann enforcement cannot modify accepted/rejected status.

CR-17.13
MiniF2F scale testing cannot be used to approve real-world domains.

CR-17.14
Human escalation must fail-safe, not fail-open.
```

---

### §B.7 — TB-17 Ship Gates — 完整修订版

原 charter 已有 SG-17.1–17.10 和 G-17.11–17.17。材料显示当前 charter 已加入 Markov inheritance policy、no-global-pointer source-of-truth、workspace test baseline、FC witness tests、PRE-17 closure ledger、Atom 7/8/9 状态等额外 gates。

我建议最终 ship gates 统一为：

```text
SG-17.1
REAL_WORLD_READINESS_REPORT.md passes audit.

SG-17.2
At least 3 candidate domains classified by T1/T2/T3/T4-style risk.

SG-17.3
At least 1 low-risk pilot domain approved, with full profile.

SG-17.4
ORACLE_REQUIREMENTS.md defines per-tier oracle architecture.

SG-17.5
CHALLENGE_COURT_REQUIREMENTS.md defines challenge evidence, window, resolver, escalation.

SG-17.6
SAFETY_BOUNDARY.md defines human escalation path and RootBox protocol.

SG-17.7
No production real-world task launched yet.

SG-17.8
IRREVERSIBLE_ACTION_POLICY.md tests at least 8 candidate actions:
  allow / deny / require-human / require-delay.

SG-17.9
MARKOV_INHERITANCE_POLICY.md exists and test suite proves:
  no global latest pointer,
  genesis = previous_capsule_cid None,
  prior-chain inheritance explicit.

SG-17.10
No global filesystem pointer source-of-truth remains.

SG-17.11
cargo test --workspace >= TB-16 baseline, 0 fail, ignored <= 150 unless justified.

SG-17.12
Flowchart conformance tests cover:
  FC1 runtime loop,
  FC2 boot,
  FC3 meta / Markov archive.

SG-17.13
All PRE-17.1–PRE-17.7 closed or explicitly deferred with architect ratification.

SG-17.14
Atom 7 Boltzmann enforcement is either:
  design-only deferred to TB-18,
  or fully implemented with Class 4 ratification + dual audit.

SG-17.15
Atom 8 comprehensive_arena is either:
  single-chain 13-of-13,
  or multi-chain-union deviation ratified with rationale.

SG-17.16
Atom 9 Markov β path is either:
  in-tape Markov resolution green,
  or design-only deferral ratified if signing-payload bump required.

SG-17.17
REAL_WORLD_READINESS_REPORT.md §8 has human architect sign-off.

SG-17.18
Large-scale MiniF2F plan is classified separately as formal benchmark stress test, not real-world pilot.

SG-17.19
No real-world payout, public settlement, or external action appears in code or evidence.

SG-17.20
Readiness dashboard / reports are reproducible from docs + ChainTape/CAS evidence, not hidden state.
```

---

### §B.8 — TB-17 Atom 策略建议

当前 charter 的"docs first, code preconditions next"是正确的。材料中 auto-resolution 也说明：原子顺序应当先写 docs，再做 code preconditions，因为 TB-17 本身就是 readiness gate；Atom 7 / 9 属于 Class 4 候选，不得先改代码再回头 ratify。

我的建议：

#### Atom 1–6：文档主体

```text
REAL_WORLD_READINESS_REPORT.md
DOMAIN_SELECTION_CRITERIA.md
ORACLE_REQUIREMENTS.md
CHALLENGE_COURT_REQUIREMENTS.md
SAFETY_BOUNDARY.md
IRREVERSIBLE_ACTION_POLICY.md
```

Class 0，自主执行。

#### Atom 7：Boltzmann OBSERVE-vs-ENFORCE

只做 design unless separately ratified。

如果要做 sequencer-side enforcement：

```text
Class 4
must stop before code
requires architect ratification
requires Phase Z′ consideration
```

#### Atom 8：comprehensive_arena substantive multi-task driver

Class 3，必须双审。

目标：

```text
尽量 single-chain 13-of-13。
```

如果确实只能 multi-chain union：

```text
必须写 architectural-exclusion deviation
必须获得 architect ratification
```

#### Atom 9：in-tape Markov β-D pipeline

先设计文档，再判断是否需要 signing-payload bump。

如果需要改：

```text
WorkTx / TerminalSummaryTx canonical signing payload
```

则升级 Class 4。

#### Atom 11：conformance tests

至少包括：

```text
markov_inheritance_policy tests
fc_alignment_conformance tests
no_global_pointer tests
irreversible_action examples
```

#### Atom 12：ship / sign-off

可以先生成 ready-for-ratification snapshot，但最终 TB-17 不能算完全 shipped，直到 human architect 签署 readiness report。

---

### §B.9 — 何时可以进行大规模 MiniF2F 真题测试？

这里要区分两类测试。

#### §B.9.1 可以现在开始的小规模 / 中规模 MiniF2F 扩展

可以在 TB-17 期间或之后立即开始：

```text
controlled MiniF2F batch
```

条件：

```text
1. 不声称 real-world readiness。
2. 不接真实资金。
3. 不启用 public settlement。
4. 不绕过 ChainTape。
5. 所有 proposal / proof / failures 进 ChainTape/CAS 或 EvidenceCapsule。
6. dashboard 可重建结果。
```

建议规模：

```text
Phase M1:
  20 problems
  n1
  MAX_TX bounded
  no market

Phase M2:
  50 problems
  n1/n3
  track solved/unsolved, fake accepted, evidence gaps

Phase M3:
  100 problems
  n5
  controlled market smoke optional
```

这个阶段的目的不是刷分，而是找架构细节问题：

```text
CAS storage pressure
L4/L4.E volume
EvidenceCapsule size
dashboard performance
challenge window behavior
autopsy leakage
Markov capsule stability
solver identity stability
```

#### §B.9.2 真正"大规模 MiniF2F"应在 TB-17 通过后进行

如果你说的大规模是：

```text
hundreds / full MiniF2F
multi-agent
market signals
NodeMarket/PriceIndex
controlled competition
```

我建议等 TB-17 ship / architect sign-off 后再开。

理由：

```text
TB-17 是 P6 -> P7 transition gate。
虽然 MiniF2F 不是真实世界，但它会成为系统能力与稳定性的 public benchmark。
如果 TB-17 的 Markov inheritance / comprehensive arena / Boltzmann enforce/observe 边界还没清楚，大规模测试会把结构问题放大。
```

因此：

```text
Full-scale MiniF2F benchmark gate =
  TB-17 SG-17.1–17.20 pass
  plus MiniF2F-specific runbook
```

#### §B.9.3 我建议的 MiniF2F 大规模测试路线

##### M0 — Benchmark harness audit

```text
20 known problems
chain-backed
no market
prove no fake accepted
```

##### M1 — Full heldout subset

```text
50–100 problems
n1 / n3
all failures produce EvidenceCapsule
dashboard batch report
```

##### M2 — Multi-agent market-disabled

```text
100+ problems
n5
Boltzmann observe only
no enforce
no price mask
```

##### M3 — Controlled market-enabled

```text
same problem set
NodePosition / PriceIndex / Autopsy enabled
no real funds
sandbox only
```

##### M4 — Public benchmark report

Only after:

```text
replay reproducibility
no unresolved evidence gaps
workspace tests green
audit pass
```

---

### §B.10 — 对后续步骤的最终建议

#### §B.10.1 现在先完成 TB-17

不要在 TB-17 前直接大规模跑 MiniF2F。

可以准备 harness，但不要把结果当正式 benchmark。

#### §B.10.2 TB-17 后第一件事

不是直接真实世界，而是：

```text
TB-18 — Formal Benchmark Scale-Up
```

这可以是大规模 MiniF2F：

```text
full controlled benchmark
chain-backed
no real-world domain
no real money
```

#### §B.10.3 TB-19 才做 low-risk real-world pilot design

选择低风险、T2-like、可验证、有 oracle 的领域。

---

### §B.11 — 给 AI coder 的直接指令

可以直接发：

```text
Architect verdict:

1. TB-16 main + TB-16.x closure are RATIFIED WITH SCOPE LIMITS.
2. Controlled Market Smoke Arena verdict accepted as sandbox evidence, not real-world readiness.
3. β-A complete. β-B / β-C / β-D remain PRE-17 forward triggers.
4. Multi-chain union accepted only as explicit deviation, not single-chain completion.
5. Markov global pointer issue is closed via Option α, but TB-17 must enforce no-global-pointer and Markov inheritance policy.
6. TB-17 charter direction is accepted but must be amended with expanded FR-17.1..14, CR-17.1..14, SG-17.1..20.
7. Atom 7 Boltzmann enforcement remains Class 4 if implemented; design-only deferral is acceptable if ratified.
8. Atom 8 comprehensive_arena must aim for single-chain 13-of-13; multi-chain union requires architectural-exclusion deviation.
9. Atom 9 Markov β-D must be design-first; signing-payload changes require Class 4 ratification.
10. Large-scale MiniF2F should not be official until TB-17 passes.
11. Controlled small/medium MiniF2F can start as harness prep:
    20 -> 50 -> 100 problems,
    all chain-backed,
    no real funds,
    no public chain,
    no real-world readiness claim.
12. Full MiniF2F benchmark should be TB-18 after TB-17 sign-off.
13. TB-17 does not launch real-world tasks.
```

---

### §B.12 — 最终判断

TB-16 是一个重要里程碑：系统已经能在沙盒里运行市场机制、审计机制、Markov/Autopsy/Boltzmann 的组合，并且能暴露证据缺口，而不是掩盖它们。

但它仍然不是 P7。

TB-17 的任务是把所有进入 P7 前必须讲清楚的东西写成硬文档、硬测试、硬门槛：

```text
oracle
challenge
human escalation
irreversible action
Markov inheritance
Boltzmann enforcement
single-chain continuation
domain classification
```

至于大规模 MiniF2F：

> **小规模和中规模现在可以作为架构压力测试准备；正式大规模 benchmark 应该放在 TB-17 通过之后，作为 TB-18。**

这样既不拖慢你发现细节问题的速度，也不会把尚未通过 readiness gate 的系统包装成"真实世界 ready"。

---

## §C — Status + execution authorization

### C.1 Status

`RATIFIED — execution authorized: "严格按架构师意见执行，直到 TB-17 ship"`

User-architect's binding authorization message (verbatim):

> 严格按架构师意见执行，直到 TB-17 ship

This authorizes Claude to autonomously execute TB-17 atoms 1-12 under the rules in §A.5 (atom envelope re-affirmation):
- **Atoms 1-6** (docs): autonomous Class 0 execution.
- **Atom 7** (PRE-17.5): design-only autonomous; implementation requires NEW architect ratification.
- **Atom 8** (PRE-17.6): autonomous Class 3 with mandatory dual external audit at ship; multi-chain-union deviation requires NEW architect ratification.
- **Atom 9** (PRE-17.7): design-first autonomous; Class 4 escalation requires NEW architect ratification.
- **Atom 11** (conformance tests): autonomous Class 1.
- **Atom 12** (SHIP): autonomous provisional commit; final TB-17 ship status REQUIRES architect signature on REAL_WORLD_READINESS_REPORT.md §8.

### C.2 Forward-trigger ledger (open at this ruling)

| ID | Source | Forward target | Notes |
|---|---|---|---|
| **PRE-17.5** Boltzmann sequencer ENFORCEMENT | OBS_R024 | TB-17 atom 7 design + TB-18 implementation if not separately ratified | Class 4 surface |
| **PRE-17.6** single-chain 13-of-13 | TB-16.x.2.6 | TB-17 atom 8; multi-chain-union deviation acceptable with NEW ratification | Class 3 |
| **PRE-17.7** in-tape Markov β-D | MARKOV_INHERITANCE_POLICY §4 | TB-17 atom 9 design; Class 3 if additive, Class 4 if signing-payload bump | TBD by design |
| **OBS_R023** evaluator hardcoded MaxTxExhausted | TB-16.x.2.2.fix.r2 | TB-18 pilot design (deferral CAP) | Cannot defer past TB-18 |
| **TB-18** Formal Benchmark Scale-Up | architect §10.2 | After TB-17 ship | Full MiniF2F controlled benchmark |
| **TB-19** Low-Risk Real-World Pilot Design | architect §10.3 | After TB-18 | T2-like / verifiable / oracle-backed |

### C.3 Memory updates pending execution

Per §A.9 — to be filed after this ruling commit.

### C.4 Cross-references

- Source prompt: `handover/architect-insights/REQUEST_TB_16_CLOSURE_AND_TB_17_AUDIT_2026-05-05.md`
- TB-17 charter: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` (target of amendment per §A.3)
- Prior architect rulings (preserved):
  - `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` (original TB-17 §8 spec — superseded for FR/CR/SG counts only; Class 0/4 hybrid + human sign-off requirement preserved)
  - `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md` (PRE-17.1..17.4 + Markov mandate — preserved)
- Constitution: Art. 0.2 + Art. 0.3 + Art. 0.4 + Art. II.1 + Art. III.1-4 + Art. IV + Art. V.1
