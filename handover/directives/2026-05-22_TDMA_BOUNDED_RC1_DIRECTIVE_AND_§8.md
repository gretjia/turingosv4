# 2026-05-22 — TDMA-Bounded-RC1 Architect Directive AND Package-Level §8 Sign-off

**class**: 4
**scope**: `feature/tdma-bounded-rc1`
**binding**: package-level RC1 §8 (explicit `feedback_no_batch_class4_signoff` override; user signed 2026-05-22)
**architect**: user (zephryj@icloud.com)
**date**: 2026-05-22
**charter**: `handover/tracer_bullets/TB-TDMA-BOUNDED-RC1_charter_2026-05-22.md`
**orchestrator plan**: `~/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md`
**GA condition**: `TuringOS-Memory-Harness-V1` 九项 Gate 全绿 + Atom 7.5 real-evidence pass + Veto-AI / 非 Veto-AI 工程审计

This file serves a dual purpose:
1. **On-disk Class 4 §8 artifact** — addresses constitution-audit C7 finding that the original directive lived only in a Claude conversation, with no on-disk evidence for clean-context replay.
2. **Authorization record** — Atom 1 (Class 4, touches `src/bus.rs` §6 restricted) and Atom 7 (Class 4, kernel keystone) both cite this file in their PR descriptions as architect authorization, in lieu of separate per-atom §8 packets.

The architect (user) explicitly overrode `feedback_no_batch_class4_signoff` for RC1 specifically (memory was created from Stage C session #27 where batch §8 caused 11-commit revert; this override is informed-consent and recorded in plan §0). The override does NOT apply to future Class 4 work: future Class 4 atoms return to per-atom §8 unless an analogous comprehensive directive is signed.

---

# DIRECTIVE — TDMA-Bounded-RC1 最终落地方案 (verbatim from 2026-05-22)

根据 2026-05-22 的上传文档与本轮 OMEGA 终审意见，我将最终方案收敛为 **TDMA-Bounded-RC1 落地规格**。核心原则不再是"让模型少看一点历史"，而是把历史从 active prompt 迁移到 append-only tape，把 retry prompt 从 conversation replay 改成 version-controlled checkout，把 raw error 从高熵 payload 改成固定预算 causal constraint；这正是已上传审查文档接受的 TDMA-Bounded 主轴。

下面这份可以直接交给 Codex 执行。

---

# TDMA-Bounded-RC1 最终落地方案

## 0. 最终裁决

**状态**：`CONDITIONAL APPROVE FOR FEATURE BRANCH`
**目标分支**：`feature/tdma-bounded-rc1`
**禁止事项**：不得直接 merge `main`。
**GA 条件**：`TuringOS-Memory-Harness-V1` 九项 Gate 全绿，并通过 Veto-AI / 非 Veto-AI 工程审计。

本轮 RC1 明确选择 **Art. 0.4 路径 A：语义版 version-control substrate**：

```text
RC1 scope = Vec<Node> + hash + verified_head + scope metadata + explicit rtool/wtool signatures
Out of scope = libgit2 / 真 Git substrate
Phase E = 仍保留真 Git substrate 作为强制 gate
```

理由：宪法中 Art. 0.4 已明确 `Q_t = <q_t, HEAD_t, tape_t>` 是 version-controlled 状态，并列出了 A/B/C 三条实现路径；其中路径 A 是保留 `Vec<Node>`、增加 `hash`、`HEAD_t` 与显式三元组签名的语义版路径，路径 B 才是真 Git substrate。 RC1 的目标是先关闭 BUG-7 与二阶硬约束，不在同一分支内引入 6–8 周级别的 substrate 大迁移。

---

# 1. 核心不变量

RC1 必须把以下数学式变成 runtime hard assert：

```
Prompt_n = G_core + S_digest + D_bbs + T_task + H_evidence + C_ctl

|Prompt_n| <= B_G + B_S + B_D + B_T + B_H + B_CTL
```

推荐 RC1 常量：

```rust
B_G          = 500;   // CharterCore
B_S          = 3000;  // SessionDigest
B_D          = 400;   // RetryBeliefState JSON
B_T          = 1500;  // Task prompt
B_H          = 100;   // Evidence pointer / hashes
B_CTL        = 300;   // Output contract / fixed control text
B_HEADER     = 256;   // STATE_UPDATE JSON object
B_HEADER_SCAN= 512;   // Parser scans only prefix
B_DISTILL_IN = 2048;  // Deterministic trace slicer output
MAX_RETRIES  = 5;
ZERO_GAIN_K  = 3;
EPSILON_GAIN = 0.01;
```

**硬约束**：

```rust
assert!(prompt_tokens <= B_G + B_S + B_D + B_T + B_H + B_CTL);
assert!(bbs_tokens <= B_D);
assert!(session_tokens <= B_S);
assert!(task_tokens <= B_T);
assert!(charter_core_tokens <= B_G);
assert!(distiller_input_tokens <= B_DISTILL_IN);
assert!(state_update_tokens <= B_HEADER);
```

这与 TDMA 原设计中的四平面 Prompt 结构一致：Global、Session、Retry Delta、Task，且每次 LLM 唤醒只从 `tape_t` 抽取 O(1) 拓扑切片，而不是滚动对话历史。

---

# 2. Codex 实施边界

## 2.1 必须做

Codex 必须实现：

1. `scope` 作为 Node 一等元数据字段。
2. `RetryBeliefState` 作为 append-only tape node，禁止 sidecar。
3. `deterministic_trace_slicer`，LLM distiller 前置硬预算闸。
4. `prefix-scan StateUpdate parser`，不依赖 closing tag。
5. `type-aware token enforcer`，不能裸 `decode(ids[:budget])`。
6. `zero-gain circuit breaker`，不能只靠 `MAX_RETRIES`。
7. `CharterCore` SHA 漂移检测。
8. `rtool.checkout_digest(..., token_budget)`。
9. 九项 regression harness。

## 2.2 禁止做

Codex 禁止：

```text
1. 禁止把 raw_stderr 拼进 next_prompt。
2. 禁止 self.tape.update_belief_state(...) 这类可变 sidecar。
3. 禁止 retry_count 沿 verified_head 链计数。
4. 禁止 task/session/charter 任一面无预算展开。
5. 禁止把 token 数近似成 payload.len() 字节数。
6. 禁止依赖自然语言 "请少于 200 tokens" 作为唯一约束。
7. 禁止把整部 constitution.md 放进 worker prompt。
8. 禁止把完整 traceback 广播给 Agent。
9. 禁止在 RC1 引入 libgit2 / 真 Git substrate 作为必要依赖。
```

宪法明确要求"所有信号必须可从 tape 重建"，平行账本只能是 tape 的派生视图，不可作为独立 source of truth；失败分支也必须以 `kind=AgentProposal, verified=false` 形态进入 tape。 同时，宪法也明确指出自然语言约束只是软约束，必须转成 linter、CI、结构化校验等硬约束。

---

# 3. 数据结构规格

## 3.1 `Node`

在 `src/ledger.rs` 增加或升级：

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeKind {
    StateAccepted,
    AgentProposal,
    RetryBeliefState,
    CharterCore,
    PromptAssembly,
    Escalation,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttemptScope {
    pub run_id: String,
    pub task_id: String,
    pub verified_parent: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TapeNode {
    pub id: String,
    pub hash: String,
    pub kind: NodeKind,
    pub verified: bool,
    pub parent: Option<String>,

    // RC1 关键修复：scope 是一等字段，不许藏进 payload
    pub scope: Option<AttemptScope>,

    // 同一 scope 下的物理 attempt 序号
    pub attempt_ordinal: Option<u32>,

    pub reject_class: Option<String>,
    pub token_count: Option<usize>,

    pub payload: serde_json::Value,
    pub created_at_unix_ms: u64,
}
```

**必须新增索引**：

```rust
pub struct TapeIndexes {
    pub by_hash: HashMap<String, TapeNode>,
    pub children_by_parent: HashMap<String, Vec<String>>,
    pub nodes_by_scope: HashMap<AttemptScope, Vec<String>>,
    pub verified_head: String,
    pub ledger_tail: String,
}
```

## 3.2 Ledger API

```rust
pub trait ImmutableTapeLedger {
    fn get_verified_head(&self) -> String;
    fn set_verified_head(&mut self, new_head: String);

    fn commit(&mut self, node: CommitRequest) -> TapeNode;

    fn count_nodes(
        &self,
        kind: Option<NodeKind>,
        verified: Option<bool>,
        parent: Option<&str>,
        scope: Option<&AttemptScope>,
    ) -> usize;

    fn latest_node(
        &self,
        kind: NodeKind,
        scope: &AttemptScope,
    ) -> Option<TapeNode>;

    fn derive_latest_belief_state_from_tape(
        &self,
        scope: &AttemptScope,
    ) -> Option<RetryBeliefState>;
}
```

**重要**：`derive_latest_belief_state_from_tape` 必须是纯函数，只读 tape，不读内存缓存。

---

# 4. Schema 规格

## 4.1 `StateUpdate`

输出头部必须是第一个 JSON object，必须位于前 `B_HEADER_SCAN = 512` tokens 内。

```rust
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum StateStatus {
    Proceed,
    Retry,
    Invalid,
    Halt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StateUpdate {
    pub schema_version: String,          // "tdma-state-update/v1"
    pub status: StateStatus,
    pub task_id: String,
    pub action: String,                  // "PROCEED" | "RETRY" | "HALT"
    pub failed_predicate: Option<String>,
    pub reject_class: Option<String>,
    pub next_action_hint: Option<String>,
    pub evidence_hash: Option<String>,
}
```

模型输出格式：

```text
{"schema_version":"tdma-state-update/v1","status":"Retry","task_id":"...","action":"RETRY","failed_predicate":"...","reject_class":"...","next_action_hint":"..."}
---BODY---
自由文本、patch、解释、diff、日志。
```

**不再使用**：

```text
<STATE_UPDATE>...</STATE_UPDATE>
```

原因：closing tag 方案能抗 body 截断，但不能抗 header 内截断。RC1 必须用前缀扫描 + streaming JSON parser 提取第一个合法对象。

## 4.2 `RetryBeliefState`

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetryBeliefState {
    pub schema_version: String,          // "tdma-bbs/v1"
    pub scope: AttemptScope,

    pub failure_signature: FailureSignature,
    pub constraints: Vec<RetryConstraint>,

    pub evidence: EvidencePointer,
    pub zero_gain_streak: u32,
    pub information_gain: f64,

    pub evicted: Vec<EvictedConstraint>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailureSignature {
    pub reject_class: String,
    pub failed_predicate: String,
    pub root_cause: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetryConstraint {
    pub id: String,
    pub rule: String,
    pub priority: u8,                    // 0 low, 255 critical
    pub source_attempt: u32,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidencePointer {
    pub evidence_node_hash: String,
    pub raw_stderr_sha256: String,
    pub trace_view_sha256: String,
}
```

---

# 5. 核心状态机

(directive §5 verbatim continues — full step_forward + handle_rejection 8-step routing in directive source; see orchestrator plan §5 Atom 7 task book for inline enumeration)

---

# 6–17

(directive §6 distiller, §7 type-aware token enforcer, §8 prefix StateUpdate parser, §9 CharterCore, §10 SessionDigest/rtool, §11 prompt assembler, §12 escalation, §13 file-level task split, §14 nine-acceptance, §15 CI/merge gate, §16 Codex worktag, §17 final merge criteria — all verbatim in the orchestrator plan's atom task books)

The orchestrator plan at `~/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md` contains the full directive content distributed across 9 atom task books (Atoms 0–8 + Atom 7.5). Each atom task book quotes the relevant directive section verbatim and provides per-atom acceptance criteria + audit dispatch.

This file's primary purpose is to provide an **on-disk Class 4 §8 authorization record** — its existence at this path with this date and the architect's git commit signature is the auditable evidence that the package was approved.

---

## Architect §8 signature block

| Field | Value |
|-------|-------|
| Architect | user (zephryj@icloud.com) |
| Date | 2026-05-22 |
| Scope | RC1 package — Atoms 0..8 + 7.5 on `feature/tdma-bounded-rc1` |
| Override declared | `feedback_no_batch_class4_signoff` for RC1 only |
| GA condition | All 14 ship-gate criteria GREEN (plan §9) + fresh §8 for feature→main merge |
| Phase E obligation | `handover/architect-insights/PHASE_E_TODO_TDMA.md` (created in Atom 5; re-affirmed in Atom 8 ship report) |

This §8 is **package-level** for RC1. Class 4 atoms within RC1 (Atom 1, Atom 7) inherit authorization from this file; their PR descriptions cite this file path. Per-atom dual-audit gates (pre-impl + post-impl) per plan §8.2 still apply and are non-bypassable. The architect explicitly accepted the trade-off (efficiency vs. per-atom Stage-C-style ratification) on 2026-05-22.
