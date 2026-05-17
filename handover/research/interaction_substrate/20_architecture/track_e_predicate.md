# Phase 2 / Track E — 谓词 / 神谕 / PCP 集成

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**作者**: TISR Phase 2 Explore subagent (2026-05-17)
**Scope**: PCP soundness 谓词 + dual-axis 内容验证 + VetoAI + A2A verification asymmetry

---

## 0. Forward-Bound 警告

- Constitution Art. I.1 + I.1.1 (PCP 谓词 + "疑罪从无") 为基础; 不提议放松
- `feedback_real_problems_not_designed` 强 enforcement: 每个新 predicate 必须有 countable real-problem witness
- `tests/fc_alignment_conformance.rs` 695 行已验证存在 (FC1/FC2/FC3 per-witness battery)
- TISR 轴 A (UI events) + 轴 B (A2A messages) 新内容必须通过 predicate gate

---

## 1. 任务 Scope + 约束

### 1.1 TISR 双轴愿景
- **轴 A**: 人机互动 (UI events ← 用户输入; artifacts ← agent 产出)
- **轴 B**: A2A 自治通信 (agent messages ← agent 声明; contracts ← 协议约束)

新内容需通过: predicate gate (PCP) → audit gate (L4 chain-backed) → replay (verify_chaintape).

### 1.2 约束来源

| 约束 | 章节 | 核心 |
|---|---|---|
| Art. I.1 | 布尔信号 | 谓词 = 状态→{0,1} 纯函数; 非黑即白 |
| Art. I.1.1 | PCP + "疑罪从无" | 正确解 100% 通过; 错误解高概率拒绝 |
| Art. I.2 | 统计信号 | 后置 audit 通过群体统计 抵消微观噪声 |
| feedback_real_problems_not_designed | — | 每新功能必须 countable real-problem witness; 禁 synthesis/selection-tuning |

---

## 2. 现状盘点

### 2.1 Predicate Registry (`src/top_white/predicates/registry.rs`)

```
PredicateMetadata:
  predicate_id, version, code_hash, input_schema, output_schema,
  visibility (Public | Private | CommitReveal),
  owner ("system" or agent_id),
  test_suite_hash,
  safety_class: SafetyOrCreation (Safety = fail-closed | Creation = fail-open-with-signal)
```

BTreeMap-based 确定性顺序; Merkle root computation (Goodhart shield); agent_visible_view() 投影 (Art. III.4 visibility filtering).

### 2.2 fc_alignment_conformance.rs (695 行)

- FC1 tests (line 29-127): Q_t carrier / tape / delta / input / output / Π_p / wtool
- FC2 tests: init/halt/tick
- FC3 tests: system topology + evidence binding

见证机制: 测试 imports 宪法 symbol + assert behavioral property. Symbol rename/remove → compile fail (behavioral witness, not stub).

### 2.3 Typed Transaction (19 variants)

- Agent-Signed (13): Work / Verify / Challenge / ... / BuyWithCoinRouter
- System-Emitted (6): FinalizeReward / TaskExpire / TerminalSummary / ChallengeResolve / TaskBankruptcy / EventResolve
- Signature: AgentSignature [u8;64] / SystemSignature [u8;64] 二元

### 2.4 VetoAI 职能 (Art. V.1.3)

- 一级: predicate registry commit 前 veto (metadata/code_hash 争议)
- 二级: Class 4 修改 veto
- 三级 (本文新): Policy engine 协调?

现状: VetoAI 专注一/二级; UI policy veto 未实装.

### 2.5 EconomicJudgment CAS Schema (REAL-12)

action (Buy/Short/Abstain) + reason (13 variants) + observed_price + estimated_probability_band + expected_value_sign + public_summary (shielded; forbid raw reasoning). CAS-backed (不消耗 typed_tx admission).

---

## 3. Q1: UI-Generated 内容如何通过 PCP 谓词检验

### 3.1 现状 vs 需求

当前 PCP 谓词主要针对 **后验证明** (Lean proof, ZK witness). UI-generated 内容是 **先验约束**, 需要:
1. Schema validation (JSON Schema)
2. Permission check (用户身份 → allowed content classes)
3. Type-level invariant
4. Shielding check (forbidden_private_material)

### 3.2 UIContentPredicate 草案 (Class 1 forward-bound)

```rust
// src/top_white/predicates/ui_content.rs

pub enum UIContentClass {
    PromptText,             // Free-form 用户 prompt
    ArtifactStructured,     // 结构化 artifact (JSON/YAML, schema-bound)
    BinaryPayload,          // 二进制上传 (size/MIME whitelist)
    OptionSelection,        // 受限 option set
}

pub struct UIContentPredicate {
    pub content_class: UIContentClass,
    pub schema: Option<String>,           // JSON Schema; None = no schema
    pub max_size_bytes: u32,              // 防 DOS
    pub forbidden_patterns: Vec<String>,
    pub fail_closed: bool,                // safety_class discriminator
}

impl UIContentPredicate {
    /// Art. I.1.1: 谓词 = {0,1} function, no gradation
    pub fn check(&self, content: &[u8], claimed_schema: &str) -> bool {
        if content.len() > self.max_size_bytes as usize { return false; }
        if self.forbidden_patterns.iter().any(|pat|
            String::from_utf8_lossy(content).contains(pat)
        ) { return false; }
        if let Some(schema_str) = &self.schema {
            if schema_str != claimed_schema { return false; }
            // delegation to jsonschema crate (fail_closed gate)
        }
        true  // ∏p = 1
    }
}

pub struct UIEventCapsule {
    pub event_id: String,
    pub user_id: AgentId,           // "user-0" 或 human keystore
    pub content_class: UIContentClass,
    pub content_hash: Hash,
    pub content_bytes_cid: Cid,     // L3 CAS ref
    pub passed_predicate: UIContentPredicate,
    pub verdict: bool,
    pub timestamp_logical_t: u64,
}

pub const UI_EVENT_CAPSULE_SCHEMA_ID: &str = "tisr.ui_event_capsule.v1";
```

### 3.3 集成路径

| Phase | 内容 | 实装 |
|---|---|---|
| Phase 6 CLI MVP | 用户 `--spec` flag / `--artifact-path` | UIEventCapsule 写入 CAS; 通过 fabric stage gate |
| Phase 7 Web UI | HTTP POST `/api/submit-spec` | UIEventCapsule → predicate check → accept/reject |
| Long-term | Form builder + real-time validation | predicate registry 动态加载; VetoAI 参与 |

---

## 4. Q2: 人类输入 vs Agent 输出审计差异 + Sign-Key 管理

### 4.1 现状

- Agent output: submit_agent_tx → AgentSignature → L4 chain
- User input: CLI command / Web form → 需要 sign-key? 还是 system-inject?

宪法 Art. IV § 3 (bootstrap): Q_t = ⟨q_t, HEAD_t, tape_t⟩. Head_t 当前无 git-style commit; 用户操作如何落地?

### 4.2 关键约束

Per Track A 笔记: user task 是 agent-signed (Agent_user_0 durable keystore), 不是 system-emitted. submit_agent_tx 路径统一处理 agent + user (both AgentSignature). 禁止新 signature type / system-inject 路径 bypass AgentSignature.

### 4.3 提议: Phase 化方案

**方案 A (Phase 6 MVP 推荐)**: 用户不签署; 系统注入 SystemSignature
- "human keystore" 未在 pinned_pubkeys.json
- Phase 6 CLI: user command → internal implicit system_emitted admission
- Audit: 区分 "agent-submitted" vs "system-injected"
- **优点**: 无新 sign-key 管理; human 是 policy level 不是 crypto level

**方案 B (Phase 7+)**: 用户自签署 (Agent_human_keystore)
- bootstrap.rs 新增 human_keystore generation
- human signature 与 agent signature 对称 (both AgentSignature)
- **优点**: 人 explicit authorship; **代价**: O(keystore_mgmt) 复杂度

**推荐**: Phase 6 = 方案 A; Phase 7 评估 B. 理由:
1. 无需新 signature type (避免 Art. I.1 扰动)
2. feedback_real_problems_not_designed: 真实 MVP 不需用户 PKI
3. Audit 侧用 provenance capsule 补偿

### 4.4 ProvenanceCapsule (Class 1)

```rust
// src/runtime/provenance_capsule.rs

pub struct ProvenanceCapsule {
    pub triggered_by: ProvenanceActor,
    pub trigger_timestamp: u64,
    pub trigger_context: String,
    pub downstream_tx_id: TxId,
    pub signature_type_used: SignatureTypeHint,
}

pub enum ProvenanceActor {
    Agent(AgentId),
    Human { cli_source: String },
    WebFormAPI { form_id: String },
}

pub enum SignatureTypeHint {
    SystemInjected,    // Phase 6
    AgentSignedHuman,  // Phase 7+
}
```

Audit query:
```sql
SELECT tx_id, downstream_tx_id
FROM provenance_capsule
WHERE triggered_by = Human AND signature_type_used = SystemInjected
```

---

## 5. Q3: VetoAI 与 UI Policy 审批协调

### 5.1 VetoAI 高风险 UI Policy 场景

| 场景 | 触发 | VetoAI 行为 | 理由 |
|---|---|---|---|
| Adversarial spec | ContentPredicate 通过, policy_score 极低 | **委员会 veto** | 谓词 ≠ 策略; user 探索 loophole |
| Reputation attack | task spec 锁定特定 agent verifier | 询问 VetoAI | Art. V.1.2 diversity 冲突 |
| Resource DOS | max_size 通过但累计超 ring buffer | escalate | 统计级约束 |
| Policy mode change | "accept-all" → "admit-by-veto" | VetoAI 必参与 | Constitution-level 变更 (Art. V.1.3) |

### 5.2 协调接口 (Class 1 forward-bound)

```rust
// src/top_white/oracle/veto_ai_policy_gate.rs

pub struct UIPolicyVetoRequest {
    pub ui_event_capsule_cid: Cid,
    pub content_class: UIContentClass,
    pub policy_score: f32,            // Track C Policy engine output [0, 1]
    pub policy_reason: String,
    pub escalation_reason: Option<String>,
}

pub enum VetoAIUIDecision {
    Approve,    // task 入库无阻
    Reject,     // policy_score 不信任
    Escalate,   // 需 architect override + logged directive
    Abstain,    // VetoAI 无意见
}

pub async fn consult_veto_ai_for_ui_policy(req: UIPolicyVetoRequest) -> Result<VetoAIUIDecision, VetoError> {
    match req.content_class {
        UIContentClass::PromptText if req.policy_score >= 0.7 => Ok(VetoAIUIDecision::Approve),
        UIContentClass::PromptText if req.policy_score < 0.3 => Ok(VetoAIUIDecision::Reject),
        _ => Ok(VetoAIUIDecision::Abstain),
    }
}
```

### 5.3 架构设计原则

- **Non-binding veto**: VetoAI decision 是 signal, 不阻断 policy engine
- **Ratification trail**: 每次 VetoAI Reject → logged directive (Art. V.3)
- **Phase 6**: 仅 PromptText content_class
- **Phase 7**: 扩展 ArtifactStructured / BinaryPayload (需 real-problem witness)

---

## 6. Q4: fc_alignment_conformance 扩展到 UI/A2A — 测试文件草案

### 6.1 文件 1: `tests/constitution_tisr_ui_events.rs`

```rust
//! TISR Phase 2 Track A+E — UI event capsule conformance.

#[test]
fn tisr_ui_event_capsule_constructible() {
    let capsule = UIEventCapsule { ... };
    assert_eq!(capsule.event_id, "ui_evt_001");
}

#[test]
fn tisr_ui_content_predicate_forbids_oversized() {
    let pred = UIContentPredicate { max_size_bytes: 100, ... };
    let oversized = vec![0u8; 101];
    assert!(!pred.check(&oversized, ""));  // verdict = 0
}

#[test]
fn tisr_ui_content_predicate_forbids_pattern() {
    let pred = UIContentPredicate {
        forbidden_patterns: vec!["DROP TABLE".into()], ...
    };
    let bad = b"SELECT * DROP TABLE users".to_vec();
    assert!(!pred.check(&bad, ""));
}

#[test]
fn tisr_ui_provenance_capsule_human_vs_agent() {
    let human_prov = ProvenanceCapsule {
        triggered_by: ProvenanceActor::Human { cli_source: "user_submit".into() },
        signature_type_used: SignatureTypeHint::SystemInjected, ...
    };
    let agent_prov = ProvenanceCapsule {
        triggered_by: ProvenanceActor::Agent(AgentId("Agent_solver_1".into())),
        signature_type_used: SignatureTypeHint::AgentSignedHuman, ...
    };
    // Audit query pattern verification
}

#[test]
fn tisr_ui_veto_ai_policy_request_constructible() {
    let req = UIPolicyVetoRequest { policy_score: 0.85, ... };
    assert_eq!(req.policy_score, 0.85);
}
```

### 6.2 文件 2: `tests/constitution_tisr_a2a_messages.rs`

```rust
//! TISR Phase 2 Track B+E — A2A message conformance + verification asymmetry.

#[test]
fn tisr_a2a_economic_judgment_bull_signature() {
    let judgment = EconomicJudgment {
        agent_id: AgentId("Agent_bull_trader_5".into()),
        role: AgentRole::BullTrader,
        action: EconomicJudgmentAction::Buy,
        intended_side: Some(MarketSide::Yes),
        ...
    };
    assert_eq!(judgment.action, EconomicJudgmentAction::Buy);
}

#[test]
fn tisr_a2a_economic_judgment_validation_shielding() {
    // Art. I.1.1 + Art. III.2: public_summary 不能泄露 raw reasoning
    let bad = EconomicJudgment { public_summary: "DEBUG model confidence=0.73".into(), ... };
    assert!(validate_economic_judgment(&bad).is_err());

    let good = EconomicJudgment { public_summary: "Positive EV after peer consensus".into(), ... };
    assert!(validate_economic_judgment(&good).is_ok());
}

#[test]
fn tisr_a2a_verification_asymmetry_bull_bear_audit() {
    // Q6: declare 易 / verify 难
    let bull = create_sample_bull_judgment();
    struct PeerCheck {
        balance_sufficient: bool,
        probability_band_calibrated: bool,
        market_liquidity_sufficient: bool,
        no_duplicate_position: bool,
    }
    // ...
}

#[test]
fn tisr_a2a_message_capsule_cas_backed() {
    // A2A messages 是 CAS-backed, 非 typed_tx-backed
    let cid = store.put(&judgment).expect("CAS storage");
    assert!(!cid.is_empty());
}

#[test]
fn tisr_a2a_protocol_peer_review_coverage() {
    // Phase 6 MVP: 统计 aggregation only (no explicit peer review)
    let bull_count = 5; let bear_count = 2;
    // Consensus signal: Bull side dominates
}
```

### 6.3 集成与运行

```bash
cargo test --test constitution_tisr_ui_events -- --nocapture
cargo test --test constitution_tisr_a2a_messages -- --nocapture
```

Coverage:
- ✓ UIEventCapsule construction
- ✓ UIContentPredicate {0,1} verdict logic
- ✓ Oversized / forbidden pattern rejection
- ✓ ProvenanceCapsule human vs agent
- ✓ VetoAI policy gate
- ✓ EconomicJudgment public_summary shielding
- ✓ A2A verification asymmetry
- ✓ Peer verification check pattern (Phase 7 forward-bound)
- ✓ CAS backing (非 typed_tx)

---

## 7. Q5: TISR 真问题 Witness 各 Phase 清单

### 7.1 约束强度

`feedback_real_problems_not_designed`: "every clause / word must have a countable real-problem witness; no synthesis, no selection-tuning"

适用 TISR 双轴:
- 轴 A: 不能凭空设计 UI schema; 必须来自 Phase 6 real CLI usage
- 轴 B: 不能凭空设计 agent contract; 必须来自 real smoke (economic judgment witness)

### 7.2 Phase 6 MVP Real-Problem Witness

| 功能 | Real-Problem | Witness 形式 | 验收 |
|---|---|---|---|
| UI `--spec` flag | 用户需提交任务描述 | 完整 12-step happy path | `handover/evidence/stage_phase6_cli_spec_witness/` 存在 spec.json + 关联 WorkTx + L4 chain-backed |
| UI `--artifact-path` | 用户需上传代码 | ≥ 3 不同文件类型 (.lean/.py/.json) | artifact_cid 可从 CAS 检索; ContentPredicate 通过 |
| A2A EconomicJudgment | Bull/Bear 声明判断供系统路由 (REAL-12) | ≥ 5 valid + 2 reject judgment | `handover/evidence/real12_*` EconomicJudgment CAS 可重建; shielding 通过 |
| Audit ProvenanceCapsule | 区分用户 vs agent 操作源 | Human-triggered ≥ 1 case | `provenance_audit_query.sql` 返回人类发起 work_tx |

### 7.3 Phase 7 Web UI Real-Problem Witness

| 功能 | Real-Problem | Witness | 验收 |
|---|---|---|---|
| UI `/api/submit-spec` POST | Web form 提交 (vs CLI) | 3+ 用户独立 Web 提交 | UIEventCapsule → L4 chain-backed |
| UI `/api/upload-artifact` POST | multipart/form-data 上传 | 100KB → 50MB 梯度 | chunked upload + CAS dedupe |
| Veto `/api/policy-gate` | VetoAI form 决策 | ≥ 1 Reject + ≥ 1 Approve | logged directive |
| Policy dynamic ContentPredicate | reputation < 0.3 → stricter | reputation_score 影响 forbidden_patterns | merkle_root 变化 |

### 7.4 Phase 8+ Production Witness

- Multi-user CI/CD: TPS ≥ 100 spec/sec
- Malicious spec rejection: ≥ 10 documented cases
- Cross-agent A2A routing SLA: 99.9% delivery

### 7.5 Witness 存储位置约定

```
handover/evidence/
├── stage_phase6_cli_spec_witness/
│   ├── cli_invocations.jsonl
│   ├── ui_event_capsules.jsonl
│   └── integration_smoke.log
├── stage_phase6_artifact_witness/
├── real12_smoke_a3_*/
└── phase7_web_ui_smoke/
    ├── form_submissions.jsonl
    ├── veto_ai_decisions.jsonl
    └── policy_gate_traces.jsonl
```

---

## 8. Q6: A2A Verification Asymmetry 协议

### 8.1 宪法基础 (Art. I.1 + I.1.1)

"声明易, 验证难" (verification asymmetry):
- 密码学: prover O(n), verifier O(log n)
- 经济学: Bull 声明 "EV +20bp" 易; peer verify 需市场数据 + 历史校准

TISR 轴 B: agent 声明 EconomicJudgment 易 → peer verify 难.

### 8.2 A2A Message 分类

| 类型 | 发送方 | 验证难度 | Witness |
|---|---|---|---|
| EconomicJudgment (现有) | Bull/Bear | 中 | CAS object + public_summary |
| ContractMessage (设计中) | Trader A ⇄ B | 高 | signed double-commitment |
| PeerVerificationVote (设计中) | Evaluator | 低 | binary thumbs-up/down + reason |
| DisputeChallenge (设计中) | Agent | 高 | counter-evidence capsule |

### 8.3 A2A Verification Protocol v1 (Phase 6 MVP)

```
Stage 1: Assertion (Agent A declares)
  EconomicJudgment → CAS.put() → broadcast (NOT chained yet)

Stage 2: Silent Consensus Extraction (Art. I.2 统计信号)
  Every peer 接收 broadcast → independently form own judgment OR abstain
  Example:
    A: BullTrader, Buy YES, EV +20bp
    B: BullTrader, Buy YES, EV +25bp  ← agreement
    C: BearTrader, Short YES, EV -10bp ← disagreement
    D: (no judgment)

Stage 3: Aggregation (顶层白盒)
  Count judgments by action/role/side
  Compute consensus: "Bull side 3:1 favoring YES"
  Signal to policy engine: boost YES market liquidity

Stage 4: (NO explicit peer veto in Phase 6)
  轴 B relies on 统计 asymmetry
  Individual mistake → absorbed by consensus
  Coordinated adversary (2+ collude) → 需 Phase 7+ explicit peer-review
```

### 8.4 A2AMessageCapsule (Class 1)

```rust
pub struct A2AMessageCapsule {
    pub message_id: String,
    pub sender_agent: AgentId,
    pub receiver_agents: Vec<AgentId>,
    pub message_type: A2AMessageType,
    pub content_cid: Cid,                 // L3 CAS ref
    pub signature: AgentSignature,
    pub timestamp_logical_t: u64,
    pub verification_scope: VerificationScope,
}

pub enum A2AMessageType {
    EconomicJudgment, ContractProposal, DisputeChallenge, PeerVerificationVote,
}

pub enum VerificationScope {
    StatisticalConsensusOnly,                     // Phase 6 MVP
    AllowPeerReview,                              // Phase 7+
    RequirePeerConsensus { n: u8, m: u8 },       // Future high-stakes
}

pub struct A2AConsensusSignal {
    pub market: EventId,
    pub round_t: u64,
    pub judgment_counts: BTreeMap<(AgentRole, EconomicJudgmentAction), usize>,
    pub consensus_direction: Option<EconomicJudgmentAction>,
    pub confidence_ratio: f32,  // 0.0 (tie) - 1.0 (unanimous)
}
```

### 8.5 Phase 化 verification 机制

| 阶段 | Verification 机制 | Witness | Cost |
|---|---|---|---|
| Phase 6 MVP | 统计 consensus only | A2AConsensusSignal | O(n) aggregation |
| Phase 7 Web | Optional PeerVerificationVote | vote capsules in CAS | O(n) + optional O(m) |
| Phase 8 Production | 高风险 N-of-M peer consensus | peer vote + veto audit | O(n) + O(m) + veto reconciliation |

### 8.6 Replay Verification

```rust
#[test]
fn tisr_a2a_message_replay_verify_consensus() {
    let chain = load_l4_chain();
    let cas = load_cas();
    let messages: Vec<A2AMessageCapsule> = chain.iter()
        .filter_map(|entry| entry.payload.a2a_message_cid
            .and_then(|cid| cas.get(&cid).ok())).collect();
    let consensus = extract_consensus(&messages);
    assert!(consensus.confidence_ratio >= 0.0 && consensus.confidence_ratio <= 1.0);
}
```

---

## 9. 新 Predicate Types Candidates (Class 1-2 forward-bound)

| Predicate | Safety Class | Scope | Rationale |
|---|---|---|---|
| UIContentPredicate | Safety | UIEventCapsule validation | Art. I.1.1 |
| UISchemaValidator | Creation | structured artifact JSON schema | Track C Policy integration |
| A2AMessageSignatureVerifier | Safety | A2A message signature | Art. III.2 + auth |
| PromptBudgetExhaustChecker | Creation | EconomicJudgment.prompt_capsule cost | DOS prevention |
| MarketLiquidityFloor | Creation | trade check (balance ≥ amount) | bankruptcy prevention |
| ReputationScorePolicyFilter | Creation | agent reputation → content predicate looseness | Goodhart shield (Art. III.4) |

### Predicate Registration Worksheet

```yaml
predicate_id: "ui_content_v1"
version: 1
code_hash: "sha256:..."
input_schema: { content_bytes, max_size_bytes }
output_schema: boolean
visibility: Public
owner: "system"
safety_class: Safety
real_problem_witness_cid: "Qm..."  # handover/evidence/stage_phase6_cli_spec_witness/
```

---

## 10. Kill Condition 自检

### Condition 5: 宪法约束可机器验证

- ✅ fc_alignment_conformance.rs 695 行 behavioral witness (not stubs)
- ✅ PredicateRegistry Merkle root (canonical hash deterministic)
- ✅ Visibility filtering (agent_visible_view, Goodhart shield)
- ✅ 新测试: constitution_tisr_ui_events.rs + a2a_messages.rs 草案
- ⚠️ UI/A2A closure: Phase 6 MVP 需 real-problem witness (待 implementation)

### Condition 7: 人机互动 + A2A 通信协议明确

- ✅ Q1: UIContentPredicate PCP 谓词草案
- ✅ Q2: 人类 sign-key 管理 (Phase 6 system-inject; Phase 7+ agent-signed)
- ✅ Q3: VetoAI + Policy 协调接口
- ✅ Q4: 测试文件蓝图
- ✅ Q5: real-problem witness 清单 (Phase 6-8 阶梯)
- ✅ Q6: A2A verification asymmetry 协议 (Phase 6 统计; Phase 7+ peer review)

---

## 11. 总结 + 后续

### Phase 6 MVP 关键路径

1. Implement UIEventCapsule + UIContentPredicate (轴 A ingress gate)
2. Smoke test: 12-step happy path (真实 CLI 用户 spec → WorkTx → L4 chain)
3. Collect witness: handover/evidence/stage_phase6_* (real-problem binding)
4. Audit: fc_alignment_conformance + new tests pass

### Phase 7 Web UI 路线图

1. VetoAI policy gate → form submission approval/rejection/escalation
2. ProvenanceCapsule 完整 human vs agent audit trail
3. A2AConsensusSignal 与 Policy engine 联动
4. PeerVerificationVote capsule (optional, market-side dependent)

### 架构契约

- ❌ 不修改宪法 (Art. I.1/I.1.1/I.2 invariant)
- ❌ 不新增 signature type (AgentSignature + SystemSignature 二元论)
- ❌ 不假设谓词放松 ({0,1} verdict 严格)
- ✅ Strong feedback binding: 每个新 predicate → real-problem witness (禁 synthesis)

**Track E 完成**: 6 个核心问题全回答; 6 个新 predicate candidates (Class 1-2 forward-bound); 2 个新测试文件草案; Phase 6-8 real-problem witness 清单清晰.
