# REAL-BCAST-1 Librarian Broadcast Loop — Architect Original

> Source: user-provided architect ruling, 2026-05-16.
> This file preserves the ruling as the local fact source for REAL-BCAST-1.

# 总裁决

我同意：**这是一个重大问题，而且它不是小 bug，而是宪法三大信号工程之一的缺口。**

TuringOS 过去一段时间把：

```text
量化 quantization
屏蔽 shielding
tape canonicality
predicate / L4 / L4.E
```

做得很强，但 **广播 broadcasting** 还没有同等成熟。

现在的状态可以概括为：

```text
CAS / ChainTape 里已经记录了大量外部化 attempt / LeanResult / L4.E / EvidenceCapsule；
dashboard 和 audit 可以读；
但普通 Agent 的 role-scoped view 里没有稳定、合宪、可追溯的“典型错误广播 / 进展摘要广播”机制。
```

这正好击中宪法第一页的核心：顶层白盒不是微观操纵者，而是对系统信息进行 **量化、广播、屏蔽** 的管理层。宪法主文明确把这三者并列为顶层白盒的核心职责。

所以我的裁决是：

> **必须插入一个新的广播修复阶段：REAL-BCAST-1 — Librarian Broadcast Loop。**
> 它应当在 REAL-13A live micro-probe 之前完成最小可用版本。
> REAL-13A 的 EV scaffolding 可以继续做 schema / plan，但真正跑 live micro-probe 前，应先把顶层白盒广播机制补上。

## 1.1 partial proof progress 是黑盒思考吗？

不是。

`partial proof progress` 不是黑盒内部 CoT，也不是黑盒里另有一条 tape。

必须分三层：

```text
1. 黑盒内部思考
   不可见，不记录，不应记录。

2. Agent 外部化出来的 proof fragment / tactic / candidate
   这是模型说出口、交给工具检查的内容。

3. TuringOS ChainTape / CAS
   只记录被系统外部化、结构化、审计允许的证据。
```

```text
partial proof progress = tool-visible externalized attempt
不是 black-box thought
不是 private CoT
不是黑盒内部 tape
```

## 1.2 黑盒内部有没有 tape？

按 TuringOS 宪法，**没有**。

或者更严格地说：

> **黑盒内部即使有某种隐式状态，也不能被 TuringOS 承认为系统事实源。**

TuringOS 的原则是：

```text
No tape, no evidence.
```

这里的 tape 只能是系统的 ChainTape / CAS / HEAD_t / L4 / L4.E。
模型内部隐藏推理、私有 CoT、provider 内部状态，都不能成为 TuringOS truth。

## 1.3 CAS 上 AttemptTelemetry 是否已经广播给 Agent？

当前答案是：**没有完整广播机制。**

CAS 里已经有 AttemptTelemetry / LeanResult 记录，但它主要用于：

```text
audit
replay
dashboard
failure-class refinement
EvidenceCapsule
```

它不是普通 Agent rtool 里的共享广播流。

所以当前 gap 是：

```text
有记录；
有审计；
有 dashboard；
但没有合宪的 role-scoped broadcast。
```

## 1.4 能不能把 CAS 里的 raw attempt 全部广播给所有 Agent？

不能。

这会违反宪法的屏蔽原则，并制造上下文污染。

因此正确做法是：

```text
不广播 raw Lean stderr
不广播 raw prompt
不广播 raw completion
不广播 private CoT
不广播 raw diagnostics
不广播 untriaged historical logs
```

只广播：

```text
错误类别
计数
趋势
角色化行动建议
CID provenance
任务标签
staleness
有限的、经过清洗的 partial-progress summary
```

# 2. 我对 Librarian Push Notification 方案的裁决

我接受这份方案的主干，并把它提升为下一阶段正式计划：

```text
REAL-BCAST-1 — Librarian Broadcast Loop
```

我采纳以下默认决策：

```text
Delivery:
  Prompt 注入，不做 out-of-band message system，不新增 TypedTx。

Scope:
  跨批次压缩，但必须带 provenance、staleness、任务相关性过滤。

Visibility:
  所有 Agent 都收到通知，但必须 role-cropped。
  Solver / Trader / Verifier / Challenger 看到不同摘要。

Risk:
  默认 Class 3。
  若触碰 TypedTx、sequencer admission、canonical signing payload、
  constitution / flowchart / genesis authority，则立即升 Class 4 并停止。
```

# 4. 新阶段：REAL-BCAST-1 — Librarian Broadcast Loop

核心路径：

```text
CAS / ChainTape / EvidenceCapsule / AttemptTelemetry / LeanResult / L4.E
-> LibrarianSelector
-> LibrarianDigest
-> RoleNotificationProjector
-> PromptCapsule read_set / visible_context_cid
-> Agent next turn
```

## Atom 0 — Charter + ratification boundary

新增：

```text
handover/directives/REAL_BCAST_1_LIBRARIAN_BROADCAST_LOOP_CHARTER.md
handover/alignment/DECISION_LIBRARIAN_DIGEST_MATERIALIZED_VIEW.md
```

必须写明：

```text
REAL-BCAST-1 does not introduce new TypedTx.
REAL-BCAST-1 does not make LibrarianDigest source of truth.
REAL-BCAST-1 does not broadcast raw logs.
REAL-BCAST-1 is role-scoped prompt injection through PromptCapsule read_set / visible_context.
```

## Atom 1 — Librarian source scope

```rust
pub struct LibrarianSourceScope {
    pub current_run_cas_root: Cid,
    pub prior_capsule_cids: Vec<Cid>,
    pub max_prior_batches: u32,
    pub task_tags: Vec<String>,
}
```

## Atom 2 — LibrarianSelector

Selector 只读取：

```text
CAS index
AttemptTelemetry
LeanResult
L4.E rejection summaries
EvidenceCapsule
MarkovEvidenceCapsule
EconomicJudgment / EVDecisionTrace
MarketDecisionTrace / NoTradeReasonTrace
```

## Atom 3 — TypicalErrorCluster

```rust
pub struct TypicalErrorCluster {
    pub cluster_id: String,
    pub error_class: String,
    pub count: u32,
    pub trend: Trend,
    pub task_tags: Vec<String>,
    pub role_hints: Vec<AgentRole>,
    pub public_summary: String,
    pub action_hint: Option<String>,
    pub provenance_cids: Vec<Cid>,
    pub staleness: Staleness,
}
```

## Atom 4 — PartialProgressSummary

```rust
pub struct PartialProgressSummary {
    pub summary_id: String,
    pub task_id: TaskId,
    pub source_attempt_cid: Cid,
    pub lean_result_cid: Cid,
    pub progress_kind: ProgressKind,
    pub tactic_class: Option<String>,
    pub public_summary: String,
    pub visibility_scope: VisibilityScope,
}
```

## Atom 5 — LibrarianDigest

```rust
pub struct LibrarianDigest {
    pub schema_version: String, // "real_bcast.librarian_digest.v1"
    pub digest_id: Cid,
    pub source_scope: LibrarianSourceScope,
    pub generated_at_head_t: HeadT,
    pub typical_error_clusters: Vec<TypicalErrorCluster>,
    pub partial_progress_summaries: Vec<PartialProgressSummary>,
    pub market_reason_clusters: Vec<MarketReasonCluster>,
    pub ev_reason_clusters: Vec<EVReasonCluster>,
    pub provenance_root: Cid,
}
```

## Atom 6 — RoleNotificationProjector

每个角色看到不同版本：

```text
Solver Notices: typical Lean errors, partial progress summaries, local tactic warnings, recent L4.E rejection summaries
Trader Notices: market no-trade clusters, EV reason clusters, task outcome uncertainty, price / liquidity anomalies
Verifier Notices: common false-progress patterns, proof artifact risk summaries, typical rejected proof classes
Challenger Notices: high-price suspicious nodes, repeated partial failures, weakness clusters
ArchitectAI Notices: aggregate error trends, predicate/tool gap candidates, not raw logs
```

## Atom 7 — Prompt injection through PromptCapsule

不新增 out-of-band message system。
不新增 TypedTx。

Prompt builder 增加 bounded section：

```text
=== Librarian Notices ===
source: CAS/ChainTape-derived, role-scoped, raw logs redacted
- err:type_mismatch count=7 trend=up role_hint=Solver
- err:rewrite_no_match count=4 trend=stable role_hint=Solver
```

## Atom 8 — Dashboard / audit materialized view

新增 dashboard section：

```text
§ Librarian Broadcast
- digest_id
- source_scope
- cluster counts
- role crops
- prompt injections
- shielding verdict
```

## Atom 9 — Real run A/B

Arm A:

```text
baseline role-scoped prompts without Librarian Notices
```

Arm B:

```text
same tasks / same models / same budgets
with Librarian Notices
```

# 7. 关键 Ship Gates 总表

```text
SG-BCAST.G1
LibrarianDigest is CAS-backed and derivable from ChainTape/CAS.

SG-BCAST.G2
PromptCapsule read_set references digest CID.

SG-BCAST.G3
RoleNotificationProjector produces different crops for Solver / Trader / Verifier / Challenger.

SG-BCAST.G4
No raw Lean stderr, raw prompt, raw completion, private CoT, raw diagnostics appear in digest / prompt / dashboard.

SG-BCAST.G5
Unknown JSON / unknown evidence schema fails closed.

SG-BCAST.G6
Digest has provenance and staleness metadata.

SG-BCAST.G7
Cross-batch digest uses explicit prior_capsule_cids; no global pointer.

SG-BCAST.G8
PartialProgressSummary derives only from externalized attempt + LeanResult.

SG-BCAST.G9
Dashboard regenerates from ChainTape + CAS.

SG-BCAST.G10
Real A/B smoke runs with audit_tape PROCEED.

SG-BCAST.G11
Constitution gates pass.

SG-BCAST.G12
Workspace tests pass.
```

# 8. 禁止事项

```text
No raw Lean stderr broadcast.
No raw prompt broadcast.
No raw completion broadcast.
No private CoT recording.
No raw diagnostics broadcast.
No untriaged historical log stuffing.
No global LATEST pointer.
No LibrarianDigest as source of truth.
No new TypedTx unless separately ratified.
No price-as-truth.
No forced trade.
No dashboard-as-truth.
No silent skip of unknown CAS payload.
```

# Relationship to REAL-13A

```text
Continue REAL-13A planning/schema if already underway.
Do not run REAL-13A live micro-probe until REAL-BCAST-1 minimum digest injection exists.
Then run REAL-13A with Librarian ON/OFF A/B.
```

