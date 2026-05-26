# Agent Output Envelope — Schema Enforcement at Adapter/Runner Boundary

研究类型: Class 0 (docs/research, worktree-only, 不接入 mainline)
作用范围: `src/tdma_runner.rs` + `src/judges/*` + 新增 10+ 个 `src/bin/*_current_kernel*.rs` runner
         + 可选 `src/runtime/attempt_telemetry.rs` tail-extension
明确不动: `src/state/sequencer.rs` / `src/state/typed_tx.rs` / `src/bottom_white/cas/schema.rs`
日期: 2026-05-26 (round-4 rebase + privacy hardening)
作者: Claude Code 研究 worktree (rebased on origin/main d0bb511d)
Audit history: round-1 PoC GREEN (24/24) → round-3 surrogate drift self-correct →
   round-4 independent witness 发现 SECOND-SOURCE-DRIFT (OBL-005/FC3/adapter baseline)
   + privacy-fence 漏洞 → rebase 到 d0bb511d + 加结构性 fence test (34/34 GREEN)

---

## 0. TL;DR

1. **应当**在 adapter/runner 边界强制 `AgentOutputEnvelope` JSON Schema —— 但只作为
   **解析结构闸门 (Layer 2)**，与现有 **predicate 真值闸门 (Layer 3)** 严格解耦。
2. 现状每个 judge 自己写一个 ad-hoc parser（fence markers / Lean blob / `### File:`），
   把"模型连 JSON 都没产出"和"答案错了"全部塌缩成 `AttemptOutcome::ParseFail`,
   `RejectionClass::ParseFailed=7`。失去了诊断信号；FC1 invariant 还是对的，
   但 audit 时无法区分"系统脆弱"和"模型能力不够"。
3. 设计目标: **结构可解析 ≠ predicate 真值**。两个 gate 各自独立 short-circuit，
   各自有 CAS 证据，各自占 `AttemptOutcome` 的不同终态。
4. 实施分级:
   - Phase A (Class 1, ≈300 LOC): 加一个 `AgentOutputEnvelope` parser 进
     `src/tdma_runner.rs` 与新 `src/judges/envelope.rs`; 失败子类编码到现有
     `tool_name: String` + `judge_reason: String` 字段，零 schema 改动。
   - Phase B (Class 2, tail-additive serde): `AttemptTelemetry` 加
     `envelope_validation_subclass: Option<EnvelopeValidationSubclass>`
     `#[serde(default)]` 字段。schema_version 不动 (per 现注释 §"Tail-additive
     fields with #[serde(default)] are forward-compat at v1")。
   - **Class 4 红线**: 绝不新增 `ObjectType` 变体，绝不新增 `RejectionClass`
     变体，绝不动 sequencer 准入规则。新失败子类映射到现有
     `RejectionClass::ParseFailed=7`，由 adapter 自己保留子类细节。

---

## 1. 现状 baseline（read-only 摘录，证据已对照 d0bb511d 源码）

### 1.0 当前 adapter / runner 全景（round-4 补充）

main d0bb511d 上 LLM/agent 输入分三条独立路径，**全部需要被 envelope 覆盖**：

**路径 A：TDMA-Bounded proof runner**（历史核心）
- `src/tdma_runner.rs::run_proof_with_ledger` — Nesbitt / Putnam / Generate / Math step judge
- 由 `cmd_tdma`, `cmd_generate`, 历史 `tdma_rc1_*` binary 调用
- 当前 adapter 形状：`LlmResponse { content: String, ... }` + `extract_body(---BODY---)` + 每 judge 自写 parser

**路径 B：`*_current_kernel*.rs` 真生产 runner**（2026-05-25+ 着陆）
新增 10 个 binary，**全部绕过 tdma_runner**，直接连真 sequencer + signed WorkTx：
- `gpqa_science_reasoning_current_kernel.rs`
- `math_competition_reasoning_current_kernel.rs`
- `swebench_live_coding_repair_current_kernel.rs`
- `mind2web_browser_action_current_kernel.rs`
- `toolbench_api_tool_use_current_kernel.rs`
- `market_external_agent_current_kernel.rs`
- `fc3_governance_reinit_current_kernel.rs`
- `full_system_augment_current_kernel.rs`
- `full_system_participation_current_kernel.rs`
- `boot_cli_current_kernel_fresh.rs`

它们当前 adapter 形状：bespoke `GpqaSample` / `MathSample` / ... typed-deserialize
（输入侧），`ResilientLLMClient` raw response（输出侧），`MIN_RATIONALE_CHARS = 120`
shape check —— **没有统一 envelope schema**。

**路径 C：FC3 governance 与 system-emit**（FC3 typed-tx 已 live）
- `SystemEmitCommand::MapReduceTick` + `TxKind::MapReduceTick = 20`
- `LogFeedbackArchiveTx`, `ArchitectProposalTx`, `VetoDecisionTx`,
  `ArchitectCommitTx`, `ReinitRequestTx`, `ReinitBootTx` — 全部 typed_tx 上链
- FC3 envelope schema 是 ArchitectAI **输入侧** schema，不替代或重复这些
  已 live 的 system-side typed_tx wire schema

Phase A 必须**同时**接入路径 A 与路径 B（路径 C 是 envelope 的下游消费方，
不在 Phase A scope）。

### 1.1 Adapter 入口形状

`src/tdma_runner.rs:275`:

```rust
pub struct LlmResponse {
    pub content: String,
    pub completion_tokens: u32,
    pub prompt_tokens: u32,
}
```

`src/tdma_runner.rs:377`:

```rust
pub fn extract_body(raw: &str) -> String {
    if let Some(idx) = raw.find("---BODY---") {
        raw[idx + "---BODY---".len()..].trim().to_string()
    } else {
        raw.trim().to_string()
    }
}
```

可观察事实:
- 进入 runner 的 agent 输出是 **未结构化 String**。
- `extract_body` 用字符串 marker `---BODY---` 切分，无 schema 校验。
- 之后整体丢给 `AnyJudge::verdict_for_stage(body, stage, accepted)`。

### 1.2 现有 judge 的 ad-hoc 解析

`src/judges/generate_judge.rs:83-120` —— `parse_file_fences` 扫描
`### File: …` markdown header 和 ` ``` ` fence 配对，自己维护状态机:

```rust
fn parse_file_fences(body: &str) -> Result<ParsedBundle, String> {
    let mut files: Vec<(String, String)> = Vec::new();
    let mut current_path: Option<String> = None;
    let mut in_fence = false;
    // … 自己写完整 markdown parser
}
```

`src/judges/math_step_judge.rs:155` —— `fn verdict(&self, body: &str, accepted_steps: &[String])
-> (bool, String, String, String)`，自己从 body 里捞 stage 名称、tactic 文本、自然语言推理。

每个 judge 都重写一遍解析层。这就是脆弱面。

### 1.3 已有的下游分类（CAS / L4.E 层，**不动**）

`src/runtime/attempt_telemetry.rs:154-186` 定义 `AttemptOutcome`:

```rust
#[repr(u8)]
pub enum AttemptOutcome {
    LeanPass = 0,
    LeanFail = 1,
    ParseFail = 2,    // ← 当前所有"结构问题"都塌缩到这里
    SorryBlock = 3,
    LlmErr = 4,
    Aborted = 5,
    PartialAccepted = 6,
}
```

`src/runtime/mod.rs:70` 注释 `RejectionClass::LeanFailed=6 / ParseFailed=7 /
SorryBlocked=8 / LlmError=9` 已经在 sequencer 准入侧固化。

**这一层不动是设计契约**: adapter 永远把它的输出塞回这 7 个 `AttemptOutcome`
和 4 个 L4.E `RejectionClass`，细分留在 adapter 自己的 CAS 视图里。

### 1.4 FC1 invariant 重申

CLAUDE.md §4 canonical:

```
evaluator_reported_completed_llm_calls
=
  tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```

LHS scope 是 **externalized attempt count**。无论 envelope 怎么细分，
**LHS 不能因为我们引入新的子类而上升或下降一个计数**。
本设计的 sub-class 全部坐在 `parse_fail` 桶**内部**，桶大小不变。

---

## 2. 设计原则（强约束）

P1. **结构可解析 ≠ predicate 真值**。两件事。两个闸门。不允许"envelope OK 所以
    predicate PASS"的语义混淆，也不允许"predicate FAIL 时把 envelope 归类为 invalid"。

P2. **不改 sequencer，不改 typed_tx**。sequencer 准入侧的 4 个 `RejectionClass`
    数值与变体集合冻结，adapter 永远 surject 进去。

P3. **Tail-additive only**。`AttemptTelemetry` 加字段必须 `#[serde(default)]`，
    historical evidence (R6/R7 v1/v2 bytes) 解码不受影响（per
    `feedback_no_retroactive_evidence_rewrite`）。

P4. **结构闸门 short-circuit predicate**。envelope-invalid 的 attempt 不应
    再进 Lean / market state machine / FC3 directive interpreter —— 不仅省钱，
    更重要的是避免给下游 predicate 一个污染的输入。

P5. **不引入新的 LLM 重试逻辑**。envelope-invalid 是和 `parse_fail` 同等的
    一次 attempt 终态，按现有 `max_attempts_per_stage` 计数。

P6. **judge 接口语义保持**。`(JudgeVerdict, Option<RejectClass>)` 这个返回类型
    不变；envelope 闸门在 judge 之**前**做。

---

## 3. AgentOutputEnvelope schema 设计

### 3.1 通用包络

```jsonc
{
  "$schema": "https://turingosv4.local/schema/agent_output_envelope/v1.json",
  "envelope_version": "v1",
  "task_kind": "lean_step | math500 | gpqa | market_signal | fc3_directive",
  "task_id": "<string, mirrors AttemptTelemetry.task_id>",
  "attempt_branch_id": "<string, mirrors branch_id>",
  "agent_self_report": {
    "agent_label": "<string, MUST match prompt-pinned agent_id; cross-checked>",
    "stage_label": "<string, must be in judge.legal_stages(task_kind)>",
    "model_provider_hint": "<optional string>"
  },
  "payload": { /* typed payload — see §3.2 per task_kind */ }
}
```

Schema-level强约束:
- 顶层 5 字段全部 required。
- `task_kind` 是 enum，未知值 → `EnvelopeUnknownVariant`。
- `agent_self_report.agent_label` 必须严格等于 prompt-pinned `agent_id`
  ——这一行抓 prompt-injection 改身份的最廉价手段。

### 3.2 typed payload (per task_kind)

```jsonc
// lean_step
{ "lean_tactic_block": "<string>", "narration": "<string, ≤512B>",
  "claims_omega_complete": false }

// math500
{ "final_answer_boxed": "<string, must literally start with \\boxed{>",
  "working": "<string>",
  "confidence_milli": 850 }

// gpqa
{ "final_answer_letter": "A|B|C|D",
  "working": "<string>",
  "confidence_milli": 850 }

// market_signal
{ "event_id": "<string>",
  "side": "YES|NO",
  "size_lots": 12,
  "rationale": "<string>",
  "claimed_evidence_cids": ["<cid>", ...]}

// fc3_directive
{ "directive_kind": "PROPOSE|VETO|RATIFY",
  "target_fc_node": "<string, must match FC1/FC2/FC3 node id>",
  "target_predicate_id": "<optional string>",
  "rationale": "<string>",
  "constitution_section_ref": "<string>" }
```

每个 `payload` 都有自己的 sub-schema。Schema 决定**结构合法性**，
predicate 决定**内容真值**。两件事。

### 3.3 不做什么（明确删除）

- ❌ envelope 里不放 raw chain-of-thought / hidden reasoning。CR-18R.4 v2
  在 `src/runtime/attempt_telemetry.rs:34-56` 已经写明 forbidden list；
  envelope 的 `narration` / `working` / `rationale` 字段允许**简短**自然语言，
  但 schema 层强制 ≤2KB（每字段独立上限），超长 → `EnvelopeFieldTooLarge`。
- ❌ envelope 不携带 token counts —— 那是 `LlmResponse` 已有信息，不让模型自报。
- ❌ envelope 不允许加 `proof_artifact_cid` / `lean_result_cid` 之类 CID
  字段 —— 这些是系统侧产出，模型自报会构成 trust-root 污染。
- ❌ FC3 envelope 不允许直接给出 typed_tx wire bytes —— Class 4 红线，
  必须经 ArchitectAI 翻译层。

---

## 4. Schema-invalid 失败子类（L4.E / CAS taxonomy）

### 4.1 EnvelopeValidationSubclass

新加（adapter 内部，**不写入 sequencer**）:

```rust
// src/judges/envelope.rs  (新文件，Class 1 additive)
#[repr(u8)]
pub enum EnvelopeValidationSubclass {
    /// 完全不是 JSON / JSON 词法解析失败
    EnvelopeNotJson = 0,
    /// JSON 合法但顶层 schema 不符（缺 required 字段，类型错）
    EnvelopeMalformed = 1,
    /// `task_kind` 是 enum，但模型给了未知字符串
    EnvelopeUnknownVariant = 2,
    /// 顶层 OK，typed payload sub-schema 不符
    EnvelopePayloadMalformed = 3,
    /// 字段超长（>2KB / >512B 等 per-field 上限）
    EnvelopeFieldTooLarge = 4,
    /// agent_self_report.agent_label != prompt-pinned agent_id
    EnvelopeAgentIdentityMismatch = 5,
    /// stage_label 不在该 task_kind 的合法 stage 集合里
    EnvelopeStageOutOfSet = 6,
}
```

### 4.2 映射到现有 AttemptOutcome / RejectionClass

| Sub-class | → AttemptOutcome | → RejectionClass | → tool_name | 进 L4.E? |
|---|---|---|---|---|
| EnvelopeNotJson | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_not_json"` | 是 |
| EnvelopeMalformed | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_malformed"` | 是 |
| EnvelopeUnknownVariant | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_unknown_variant"` | 是 |
| EnvelopePayloadMalformed | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_payload_malformed"` | 是 |
| EnvelopeFieldTooLarge | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_field_too_large"` | 是 |
| EnvelopeAgentIdentityMismatch | ParseFail=2 | PolicyViolation=1 | `"parse_fail.envelope_identity_mismatch"` | 是 |
| EnvelopeStageOutOfSet | ParseFail=2 | ParseFailed=7 | `"parse_fail.envelope_stage_out_of_set"` | 是 |

Key 设计点:
- 全部 surject 到 `AttemptOutcome::ParseFail` —— 不引入新枚举值。
- `EnvelopeAgentIdentityMismatch` 是唯一例外 —— 映射到
  `RejectionClass::PolicyViolation=1`，因为身份不匹配是策略层违规，不是
  解析失败。但 `AttemptOutcome` 仍是 `ParseFail`（attempt 在解析时被拒）。
- 子类细节通过两个通道携带:
  - `tool_name: String` 加点号扩展 (Phase A, 零 schema 改动)
  - 可选 `envelope_validation_subclass: Option<EnvelopeValidationSubclass>` 
    tail-additive serde field on `AttemptTelemetry` (Phase B)

### 4.3 candidate_payload_cid 的语义

CR-18R.4 v2 现有规则: `candidate_payload_cid` 指向"parsed external candidate
bytes"，NEVER raw LLM response.

本设计在 envelope 校验失败时:
- **不要**把 raw LLM response 写进 CAS。
- 写一个 minimal `EnvelopeRejectionPayload` JSON:
  ```jsonc
  { "envelope_validation_subclass": "EnvelopeMalformed",
    "first_error_path": "$.payload.final_answer_boxed",
    "first_error_message": "expected string starting with \\boxed{",
    "raw_body_sha256_prefix_8": "deadbeef" }
  ```
- 这个对象的 bytes 自身就是 `candidate_payload_cid` 指向的内容。
  既保持 CR-18R.4 v2 privacy invariant（无 raw response），又给 audit
  留下了可重建的诊断线索（raw_body_sha256_prefix_8 可对得上同 prompt 的
  其他记录）。

### 4.4 FC1 invariant 影响

零影响。所有 envelope-invalid 仍然 = 一次 `parse_fail` tool_dist 增量
= 一次 `r2_write_attempt_telemetry` 调用点。LHS = `step + parse_fail + llm_err`
等式两侧都不变。

---

## 5. Runner 接入最小方案

通用接入位点（基线，所有 4 个 benchmark 共享）:

```rust
// 在 src/tdma_runner.rs 的 run_proof_with_ledger 主循环里，judge 调用之前:
let body = extract_body(&llm_response.content);
match envelope::validate(&body, task_kind, prompt_pinned_agent_id, judge.legal_stages()) {
    Ok(envelope) => {
        // 进 typed payload 提取，再进 judge.verdict_for_stage
        let candidate = envelope.payload_as_candidate(task_kind);
        let (verdict, reject) = judge.verdict_for_stage(&candidate, stage, &accepted);
        // … existing path
    }
    Err(subclass) => {
        // 写 EnvelopeRejectionPayload 到 CAS
        let rejection_cid = cas.put(/* … */);
        // 构造 AttemptTelemetry { outcome: ParseFail, tool_name: subclass.dotted_label() }
        // short-circuit: 不调 judge，不调 Lean，不调 market state machine
        record_envelope_rejection(rejection_cid, subclass, /* … */);
    }
}
```

### 5.1 GPQA runner 接入

GPQA 单选 4 选 1。

最小 envelope:
```jsonc
{ "envelope_version": "v1", "task_kind": "gpqa",
  "task_id": "gpqa.diamond.q_0042", "attempt_branch_id": "n1.b0",
  "agent_self_report": { "agent_label": "agent_alpha", "stage_label": "answer" },
  "payload": { "final_answer_letter": "C", "working": "...", "confidence_milli": 900 } }
```

新 judge: `src/judges/gpqa_judge.rs` —— 把 `final_answer_letter` 和 gold key 对比，
返回 `JudgeVerdict::Pass | Fail`。**predicate 真值**仅看 letter，**不**看 working
（working 只用于 CAS 留底 + 后续 audit）。

GPQA-specific schema-invalid 子类（特例化）:
- `final_answer_letter` 不在 `{A,B,C,D}` 集合 → `EnvelopePayloadMalformed`
- 缺 `confidence_milli` → 不算错（field optional in sub-schema），不阻断

### 5.2 MATH-500 runner 接入

MATH 是开放式数值/表达式。

最小 envelope:
```jsonc
{ "envelope_version": "v1", "task_kind": "math500",
  "task_id": "math500.algebra.0123", "attempt_branch_id": "n1.b0",
  "agent_self_report": { "agent_label": "agent_alpha", "stage_label": "answer" },
  "payload": { "final_answer_boxed": "\\boxed{42}", "working": "..." } }
```

新 judge: `src/judges/math500_judge.rs` —— 从 `final_answer_boxed` 抽
`\boxed{...}` 内层字符串，做 Hendrycks-style normalize（去 LaTeX whitespace,
统一 frac 表示），和 gold 对比。

MATH-specific 强制:
- `final_answer_boxed` 必须以 `\boxed{` 开头并以 `}` 结尾 —— 否则
  `EnvelopePayloadMalformed`。这一条等价于现在 ad-hoc 正则要做的事，
  抬到 envelope 层后变成结构闸门，不污染 judge 的真值判断。

### 5.3 Market (Polymarket signal) runner 接入

最小 envelope:
```jsonc
{ "envelope_version": "v1", "task_kind": "market_signal",
  "task_id": "polymarket.event.abc123", "attempt_branch_id": "n1.b0",
  "agent_self_report": { "agent_label": "trader_alpha", "stage_label": "submit_signal" },
  "payload": {
    "event_id": "polymarket.event.abc123",
    "side": "YES",
    "size_lots": 5,
    "rationale": "…",
    "claimed_evidence_cids": ["<cid1>", "<cid2>"] } }
```

特别的 schema 闸门:
- `event_id` 必须等于 `task_id` —— 抓"模型给了别的事件"的 prompt 漂移。
- `side ∈ {YES, NO}` —— `EnvelopeUnknownVariant` 即拒。
- `size_lots` 必须为正整数（i64，与现有 money invariant 一致；
  **NO f64**，per AGENTS.md §12）。

**关键**: schema 通过 ≠ trade 合法。schema 只确认"模型给了一个结构上能解析的
trade decision"。它**不**确认:
- event_id 是否在当前 active event book 内（这是 market state machine 的 predicate 工作）
- size_lots 是否超过 escrow vault 余额（这是 economy/escrow_vault.rs 的 predicate 工作）
- claimed_evidence_cids 是否真实存在于 CAS（这是 predicate 真值闸门的工作）

market predicate 闸门在 envelope 通过之后才跑。这条边界即"结构 vs 真值"。

### 5.4 FC3 (ArchitectAI feedback) runner 接入

**注意**（round-4 更新）: 截至 d0bb511d，FC3 typed-tx surface 已 live —
`LogFeedbackArchiveTx`, `ArchitectProposalTx`, `VetoDecisionTx`,
`ArchitectCommitTx`, `ReinitRequestTx`, `ReinitBootTx`（见
`src/bin/fc3_governance_reinit_current_kernel.rs`）。本节 envelope schema
作为 ArchitectAI **输入侧** schema-checkable 入口，不替代也不重复
已 live 的 system-emit typed_tx 写入路径。

最小 envelope:
```jsonc
{ "envelope_version": "v1", "task_kind": "fc3_directive",
  "task_id": "fc3.feedback.<run_id>",
  "attempt_branch_id": "architect.b0",
  "agent_self_report": { "agent_label": "architect_ai", "stage_label": "propose_directive" },
  "payload": {
    "directive_kind": "PROPOSE",
    "target_fc_node": "FC1-N42",
    "target_predicate_id": "predicate.attempt_routing_l4_l4e",
    "rationale": "…",
    "constitution_section_ref": "constitution.md §455" } }
```

FC3-specific 强制:
- `directive_kind ∈ {PROPOSE, VETO, RATIFY}` —— 任何其他值 →
  `EnvelopeUnknownVariant`。VETO 还要走 Veto-AI 二次校验（约束在 schema 之外）。
- `target_fc_node` 必须 lookup 命中当前 `TRACE_FLOWCHART_MATRIX.md` 节点
  集合 —— 这一行是 FC3 "directive 不能凭空指向不存在的节点" 的最低
  自动闸门。结构闸门，不替代 ArchitectAI 二次评议。
- **绝对禁止** envelope 携带可执行 typed_tx wire bytes（Class 4 红线）。
  FC3 directive 永远是 intent，wire 翻译由系统侧完成。

---

## 6. 关键不变量 —— 结构 vs 真值的对照表

| | Layer 2 envelope 闸门 | Layer 3 predicate 闸门 |
|---|---|---|
| 判断什么 | bytes 能不能解析成 typed envelope | 内容是否满足 constitution gate |
| 谁判断 | adapter（统一） | 各 judge / state machine / Veto-AI |
| 失败 → CAS 对象 | EnvelopeRejectionPayload | LeanResult / GenerateRejectionCapsule / MarketRejectionCapsule … |
| 失败 → AttemptOutcome | ParseFail=2 | LeanFail=1 / SorryBlock=3 / domain-specific |
| 失败 → RejectionClass | ParseFailed=7 (or PolicyViolation=1 for identity) | LeanFailed=6 / SorryBlocked=8 / PredicateFailed=0 |
| 是否计入 FC1 LHS | 计入 parse_fail 桶 | 计入 step / step_reject |
| 模型可见反馈 | "envelope schema error at $.payload.final_answer_boxed" | "lean rejected at line 17 of tactic block" |
| Replay 行为 | 给同 prompt 重跑，确定地走同一 envelope 分支 | 由 LeanResult / VerificationResult 决定 |

**核心断言**:

> envelope OK ⟹ predicate 仍可 PASS / FAIL
> envelope FAIL ⟹ predicate 路径 short-circuit，**不**评估真值
> ∴ "envelope OK" 不蕴含 "predicate PASS"
> ∴ "predicate FAIL" 不蕴含 "envelope FAIL"

这条断言必须由 `tests/envelope_vs_predicate_decoupling.rs`（一个新的
constitution gate test）固化:

```rust
// 伪代码
#[test]
fn envelope_pass_does_not_imply_predicate_pass() {
    let body = r#"{"envelope_version":"v1","task_kind":"gpqa", … 
                   "payload":{"final_answer_letter":"A", … }}"#;
    let envelope = envelope::validate(body, /* … */).expect("envelope ok");
    // gold = "C", model = "A"
    let (verdict, _) = gpqa_judge::verdict(&envelope.payload, /* gold = "C" */);
    assert!(matches!(verdict, JudgeVerdict::Fail { .. }));
}

#[test]
fn envelope_fail_short_circuits_predicate() {
    let body = "not json at all";
    let result = run_attempt(body, /* … */);
    assert_eq!(result.outcome, AttemptOutcome::ParseFail);
    assert!(result.lean_result_cid.is_none()); // ← Lean 不被调用的证据
}
```

---

## 7. 反对意见与权衡

### 7.1 "强制 JSON 会让 LLM 更难产出"

事实: 现在的 generate / Nesbitt / Putnam runner 已经强制了非常 brittle 的
text-fence 协议（`### File:` / `---BODY---` / stage-label 关键字）。JSON envelope
比 markdown fence**更**鲁棒，因为 JSON parser 是工业级，markdown fence 是手写。

经验数据点（建议在 Phase A ship 时收集）: 对同一 prompt set 跑 200 次，
比较 envelope-required vs fence-required 的 parse_fail 率。如果 envelope
让 parse_fail 率上升 >20%，回滚。

### 7.2 "envelope 抓了一层，但模型还是可以乱写 payload 内容"

是的 —— 这正是设计意图。envelope 只保证**结构**。内容真值留给 predicate。
**不要**把两件事缝合。如果未来想给 GPQA 加"working 内容必须包含至少一个公式"
之类的内容检查，那是 predicate 工作，不应该升级 envelope schema。

### 7.3 "为什么不直接加一个新的 RejectionClass::EnvelopeMalformed?"

**Class 4 红线**。`RejectionClass` 数值 6-9 已经写进 sequencer L4.E 准入
（`src/runtime/mod.rs:70`），扩这个枚举 = sequencer admission 改动 = Class 4
= 需要 STEP_B + per-atom §8。

本设计明确避开这条路。adapter 内部的 sub-class **不需要**进 sequencer，
因为 sequencer 不需要区分 envelope 子类来做准入决策 —— 它只需要知道
"这个 attempt 是 ParseFailed=7，不走 L4 accepted"，足矣。子类细节是
**audit-side 信号**，留在 adapter 的 CAS 视图里就够了。

### 7.4 "若 envelope schema 自身演化怎么办"

envelope schema 用 `envelope_version: "v1"` 字段显式版本化。未来 v2 = 新文件
`schema/agent_output_envelope/v2.json`，runner 同时支持 v1/v2 直到 sunset。
**绝不**就地修改 v1 schema 而期望旧 evidence 重解 —— 那是
`feedback_no_retroactive_evidence_rewrite` 违例。

---

## 8. 明确不做（Class 4 红线再列）

1. ❌ 不新增 `AttemptOutcome` 变体（保留 7 个）。
2. ❌ 不新增 `RejectionClass` 变体（保留 10 个：0..9）。
3. ❌ 不新增 `ObjectType` 变体（保留 `src/bottom_white/cas/schema.rs` 当前集合）。
4. ❌ 不改 sequencer 准入函数签名、不改 L4 / L4.E 路由判断。
5. ❌ 不让 envelope schema 触发 LLM 重试以外的副作用（不递归调用 distiller，
   不写 message board）。
6. ❌ 不把 envelope check 放到 sequencer 内部 —— 它是 adapter 工作，sequencer
   是无知的下游。

---

## 9. 建议落地顺序（如未来授权实施）

Phase A — Class 1 additive，单 PR，≈300 LOC:

1. 新文件 `src/judges/envelope.rs` —— 包络 parser + `EnvelopeValidationSubclass`。
2. `src/tdma_runner.rs` `run_proof_with_ledger` 在 judge 调用前插入
   envelope check; 失败路径写 `tool_name = "parse_fail.<dotted-subclass>"`,
   judge_reason = 详细错误。
3. `src/judges/gpqa_judge.rs` + `src/judges/math500_judge.rs` 新建（最小可工作版本）。
4. 新 schema 文件 `schema/agent_output_envelope/v1.json` 进 repo。
5. 新 constitution gate test `tests/envelope_vs_predicate_decoupling.rs`，
   两个断言（envelope_pass 不蕴含 predicate_pass / envelope_fail 短路 predicate）。
6. `bash scripts/run_constitution_gates.sh` 必须仍 exit 0，
   `cargo test --workspace --no-fail-fast` exit 0。
7. 不引入 BenchmarkManifest 新字段（manifest 是 batch 元数据，不是 attempt 元数据）。

Phase B — Class 2 wire-up（可选，只在 Phase A 跑通 200-attempt 真 evidence 之后）:

1. `src/runtime/attempt_telemetry.rs` 加 tail-additive
   `pub envelope_validation_subclass: Option<EnvelopeValidationSubclass>`
   `#[serde(default)]`. schema_version 不动（per 现注释 §"Tail-additive
   fields ... forward-compat at v1"）—— 但请 clean-context Codex 审计这条。
2. 给 `audit_tape` 加一个 sampler: "随机抽 N 个 parse_fail attempt, 
   断言 envelope_validation_subclass 都非 None"。
3. 给 dashboard / replay verifier 加 envelope sub-class 切片视图。

Phase C — benchmark 接入（独立 charter）:

GPQA / MATH-500 / Polymarket signal / FC3-directive 各自独立 charter，
都建立在 Phase A + Phase B 之上。MATH-500 与 GPQA 优先级最高（最纯净的
真值闸门 ↔ 结构闸门解耦示例）。Polymarket 与 FC3 涉及更多 Class 2/3 surface，
应该排在后面。

---

## 10. 与 OBL-005 的关系（round-4 重写）

OBL-005（三 Flowchart 全链路覆盖测试集设计与落地）当前 status 在
`origin/main d0bb511d` 上 = `in_progress`（**不是** `blocked`）。

OBL-005 当前的闭合通路（per 2026-05-25/26 user clarification）:
- broad real-world problem evidence 覆盖每个保留的 production module
- per-sample `full_system_participation.json`
- per-result FC1 + FC2 + FC3 declaration
- market / economy participation per sample

主 main 上已存在的 GPQA / MATH / SWE-bench / ToolBench / Mind2Web runner
全部被 user 显式标为 `domain_adapter_smoke_only`，不算 closure evidence。

本研究**不是** OBL-005 闭合路径上的一块。本研究产出的 envelope schema
也**不**改变 OBL-005 闭合的 evidence 要求。两者解耦：

| | OBL-005 闭合需要 | 本研究产出 |
|---|---|---|
| 证据形状 | per-sample full-system participation | 0 envelope failure samples |
| 接入 | 真 LLM × 真 task × 真 sequencer | adapter-side 结构 gate |
| 闸门 | predicate truth + market participation | schema parseability |

**断言**: 本研究 ship 与 OBL-005 闭合是两个独立轨道，互不阻塞。
本研究 ship 不能用来声称 OBL-005 任何进展。OBL-005 闭合也不需要
等本研究。

---

## 11. 决策点 (留给用户/architect)

D1. 是否同意"结构闸门与 predicate 闸门必须解耦"作为原则？
D2. 是否同意把 envelope sub-class 留在 adapter 内部（不进 sequencer
    `RejectionClass`）？
D3. 是否同意 Phase A 走 Class 1 additive PR，evidence-first（先 200 次
    GPQA + 200 次 MATH-500 真跑，再上 Phase B）？
D4. 是否同意 FC3-directive envelope schema 与 OBL-005 closure 解耦，
    属于本研究独立产出？

我的建议: 4 个都同意。这是当前最廉价的"audit-side 诊断力 +1, 系统脆弱
面 -1"的非 Class 4 改动。

---

(结束)
