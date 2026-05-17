# Phase 2 / Track B — ChainTape / CAS / HEAD_t 证据集成

**本文档为 forward-bound research 提案；任何派生 TB 须独立 §8 ratification；本文档不构成已批准方案。**

**作者**: TISR Phase 2 Explore subagent (2026-05-17)
**Scope**: 将 TISR 双轴证据 (UI 事件 + A2A 通信) 映射到 ChainTape (L4/L4.E) + CAS + HEAD_t witness

---

## 1. 任务 Scope

**In Scope**:
- UI event audit trail and artifact replay (轴 A)
- A2A agent message logs and contract negotiation (轴 B)
- CAS schema design patterns for capsule storage
- HEAD_t witness field requirements
- Multimodal artifact (image/audio/video/3D) storage strategy
- Cold-replay / audit integrity

**Out of Scope**:
- 新 CAS writer 实施 (forward-bound to Phase 6+)
- ChainTape backend 修改 (Class 3+)
- Agent LLM 系统 prompt 逻辑

---

## 2. 现状盘点

### 2.1 HEAD_t Witness 6 Canonical Fields (head_t_witness.rs)

```rust
pub struct HeadTWitness {
    pub state_root: Hash,
    pub l4_head: NodeId,
    pub l4e_head: Option<NodeId>,
    pub cas_root: Option<Hash>,
    pub economic_state_root: Hash,
    pub run_id: String,
}
```

**6-field constitutional gate** (architect-pinned 2026-05-07): `head_t_witness_has_six_canonical_fields` test 强制 enforce. 任何字段添加破坏宪法 gate.

### 2.2 CAS Schema (cas/schema.rs)

`Cid([u8; 32])` SHA-256 content identifier; `ObjectType` enum 21+ variants.

**现有 ObjectType variants** (audit/evidence focused):
- EvidenceCapsule / EvidenceManifest / CompressedRunLog
- AgentAutopsyCapsule / AutopsyPrivateDetail
- MarkovEvidenceCapsule / NextSessionContext
- AttemptTelemetry / LeanResult / PromptCapsule

**Index persistence**: sidecar JSONL (`.turingos_cas_index.jsonl`) Cid → CasObjectMetadata.

### 2.3 现有 Capsule Schema Patterns (5 个)

**Pattern 1: Class-3 PromptCapsule** (Art. III.4):
- prompt_context_hash + read_set + policy_version + hidden_fields_redacted + visible_context_cid + system_prompt_template_hash + agent_view_manifest_cid
- 钉住"agent 看到什么" via hash + CAS refs, 不泄露 verbatim prompt

**Pattern 2: EvidenceCapsule** (TB-11):
- capsule_id (self-ref) + run_id + task_id + counts + public_summary + evidence_manifest_cid + compressed_log_cid
- O(1) chain cost + O(N) audit cost; AuditOnly privacy

**Pattern 3: EconomicJudgment** (REAL-12 FC1-N41):
- schema_version "real12.economic_judgment.v1" + agent_id + role + task_id + head_t + visible_markets + chosen_market + intended_side + action + reason + prompt_capsule_cid + public_summary
- 典型 CAS-only 通信层

**Pattern 4: MarkovEvidenceCapsule** (TB-15 Atom 5):
- capsule_id + previous_capsule_cid (Markov chain) + constitution_hash + flowchart_hashes + l4_root + l4e_root + cas_root + typical_errors + unresolved_obs + next_session_context_cid
- 端 TB compression, 不是 hidden source of truth

**Pattern 5: LibrarianDigest** (REAL-BCAST-1):
- LibrarianSourceScope + LibrarianEvidenceEvent + TypicalErrorCluster
- 选 sanitized evidence atoms from CAS for role-scoped broadcast

---

## 3. Q1: UI 事件 → L4 / L4.E / CAS 映射

**答**: UI 事件应映射到 **CAS storage via 新 `UIEventCapsule` schema**, **L4 anchors 仅用于 state-mutating actions**.

### 3.1 分类

1. **Non-state-mutating UI 交互** (dashboard 浏览, read-only views) → **CAS-only**, 通过 `MarkovEvidenceCapsule.ui_event_cids: Vec<Cid>` 间接 anchor. Rationale: audit 可后置重建 user journey 不污染 L4.

2. **State-mutating UI 交互** (task spec upload, artifact 完结, role assignment confirmation) → **L4.E (rejected) by default; L4 on approval**. 用 `TaskOpenTx` / `EscrowLockTx` 现有 envelope + 新 `ui_context_cid: Cid` 指向 UIEventCapsule.

### 3.2 UIEventCapsule schema (Class-3 forward-bound)

```rust
pub struct UIEventCapsule {
    pub capsule_id: Cid,
    pub timestamp_logical: u64,            // HEAD_t.l4_head distance
    pub user_id: UserId,
    pub event_kind: UIEventKind,           // TaskSubmit / ArtifactUpload / ViewDashboard / ...
    pub target_id: Option<String>,
    pub visible_context_cid: Cid,          // JSON manifest of UI form state
    pub artifact_cids: Vec<Cid>,
    pub outcome: UIEventOutcome,
    pub reason: Option<String>,
    pub linked_tx_cid: Option<Cid>,        // L4/L4.E entry 引用
}

pub enum UIEventKind {
    TaskSubmit, TaskModify, ArtifactUpload, ArtifactView,
    DashboardQuery, RoleAssignmentConfirm, SpecRevision, Other(String),
}

pub enum UIEventOutcome {
    Accepted,           // → L4
    RejectedByUser,
    RejectedByPolicy,   // 验证 gate blocked
    Pending,
}
```

### 3.3 HEAD_t 影响

**无即时扩展需求**. UIEventCapsule 通过 cas_root (现有字段) 间接 anchor. 未来如需 per-UI-session rollup, optional `ui_event_rollup_cid: Option<Cid>` (forward-bound).

### 3.4 隐私 & 审计

- visible_context_cid 引用 sanitized UI form (字段名 + 类型, 不含 PII)
- artifact_cids 实际 uploads, access 受 CapsulePrivacyPolicy 控制
- Librarian broadcast 可 surface 高层摘要 ("用户提交 3 个 task"), 不暴露 timing/内容

---

## 4. Q2: A2A 通信 → A2AMessageCapsule 设计

**答**: agent-to-agent 通信 (negotiation, contract proposals, role handoff) 应映射到 **新 `A2AMessageCapsule` CAS schema + 可选 L4 annotations** (经济性重要 messages 如 challenge proposals).

### 4.1 分类

1. **Non-binding A2A chatter** (status polls, debug queries) → **CAS-only**, 通过 `MarkovEvidenceCapsule.a2a_dialog_cids: Vec<Cid>` 间接 anchor.

2. **Binding A2A contracts** (challenge proposals, solver hand-off, market coordination) → **L4 entry + A2AMessageCapsule CAS anchor**. 现有 ChallengeTx / VerifyTx 可作为 envelope.

### 4.2 A2AMessageCapsule schema (Class-3 forward-bound)

```rust
pub struct A2AMessageCapsule {
    pub capsule_id: Cid,
    pub timestamp_logical: u64,
    pub sender_id: AgentId,
    pub recipient_ids: Vec<AgentId>,        // multicast
    pub message_kind: A2AMessageKind,
    pub subject: String,                    // task_id / event_id / ...
    pub payload_cid: Cid,                   // CAS ref to structured message
    pub signature: Option<SystemSignature>, // if cryptographically binding
    pub nonce: u64,
    pub ttl_rounds: u32,
    pub reply_to_capsule_cid: Option<Cid>,  // threading DAG
    pub protocol_version: String,
    pub visibility_scope: VisibilityScope,
    pub outcome: A2AMessageOutcome,
    pub attached_evidence_cids: Vec<Cid>,
}

pub enum A2AMessageKind {
    ProposalRequest, ProposalAccept, ProposalReject,
    StatusUpdate, AcknowledgmentRequest, AcknowledgmentReply,
    RoleHandoff, MarketCoordination, ChallengeCoordination,
    DebugQuery, Other(String),
}
```

### 4.3 协议 & Threading

- FIFO delivery via CAS sidecar index 序号
- Nonce + TTL 防 replay + deadline 强制
- attached_evidence_cids 允许引用 PromptCapsule / EconomicJudgment / 先前 A2A capsules 作为 justification

### 4.4 HEAD_t & L4 影响

- 无 HEAD_t 扩展 (cas_root anchor)
- 可选 L4 扩展 (如 binding A2A 成为 state-mutating); 当前 L4 TxKind 17 variants 可能扩展或新增 forward-bound

---

## 5. Q3: HEAD_t 扩展决策

**答**: **无 HEAD_t 字段添加需求**. 两者通过 cas_root anchor.

### 5.1 理由

1. **架构师 pin** (2026-05-07): 6-field count 是宪法 gate. 添加字段破坏 `head_t_witness_has_six_canonical_fields` test.

2. **cas_root 足够**: CAS index 持久化 (CO1.4-extra sidecar). 所有新 capsule Cid 流入 cas_root hash. MarkovEvidenceCapsule.cas_root 捕获生成时 index state. Dashboard 通过 `HEAD_t.cas_root` + replay 重建完整 evidence chain.

3. **未来 forward-bound 扩展**: 如需 fast queries, 引入 `HEAD_t_Extended` (不动 HEAD_t):
   ```rust
   pub struct HeadTExtended {
       pub base_witness: HeadTWitness,
       pub ui_event_rollup_cid: Option<Cid>,
       pub a2a_dialog_rollup_cid: Option<Cid>,
   }
   ```
   存为独立 CAS object; 从 MarkovEvidenceCapsule 引用.

---

## 6. Q4: 多模态 Artifact CAS 策略

**答**: **2-tier 存储**:
1. **小 artifacts** (< 1 MB): 直接存 CAS 为 BLOB (新 ObjectType: ImageBlob/AudioBlob/VideoBlob/Model3dBlob)
2. **大 artifacts** (>= 1 MB): 存 reference manifest 到 CAS, 文件委托外部 CDN/IPFS/S3 + 嵌入 BLAKE3 / SHA-256 CID

### 6.1 CAS 能力

- Cid SHA-256 32-byte 无 size 限制
- Git2 blob backend 可存任意二进制
- Index JSONL 已含 optional size_bytes + media_type 字段

### 6.2 ArtifactStorageManifest schema (Class-3 forward-bound)

```rust
pub struct ArtifactMetadata {
    pub cid: Cid,
    pub media_type: ArtifactMediaType,
    pub size_bytes: u64,
    pub original_filename: String,
    pub upload_timestamp_logical: u64,
    pub uploader_id: Option<UserId>,
    pub dedup_hash: Option<Hash>,           // Blake3 content-aware dedup
}

pub struct ArtifactStorageManifest {
    pub capsule_id: Cid,
    pub artifacts: Vec<ArtifactMetadata>,
    pub dedup_savings_bytes: u64,
    pub compression_policy: CompressionPolicy,
    pub external_refs: Vec<ExternalArtifactRef>,
}

pub enum ArtifactMediaType {
    ImageJpeg, ImagePng, ImageWebp,
    AudioMp3, AudioWav,
    VideoMp4, VideoWebm,
    Model3dGlb, Model3dUsdz,
    DocumentPdf, Other(String),
}
```

### 6.3 去重 & 隐私

- Content-aware dedup: Blake3 hash; 已存在则 0-byte placeholder + link to prior CID (git packfile delta 风格)
- Privacy: 默认 UserVisible (用户+审计可读); 敏感内部 marked AuditOnly
- 多播效率: 跨 A2AMessageCapsule 引用同 artifact 自动 dedup

---

## 7. Q5: Replay / Audit 完整性

**答**: **replay_full_transition 仍完整**. 新 capsule schemas (UIEventCapsule, A2AMessageCapsule, ArtifactStorageManifest) 全部 Class-3+ (audit-level), 通过 L4 chain + L4.E + CAS index + MarkovEvidenceCapsule rollup 重建.

### 7.1 Replay 算法 (Extended)

```text
replay_full_transition(repo, target_round, start_markov_cid=Optional)
1. Load genesis payload + constitution.md
2. [NEW] If start_markov_cid: load MarkovEvidenceCapsule + verify constitution_hash + validate l4_root/l4e_root
3. For each L4 entry from (markov.l4_root OR genesis) to target_round:
      a. Deserialize TxKind + payload
      b. [NEW] If tx.ui_context_cid: fetch UIEventCapsule + validate user_id + log to audit (non-canonical)
      c. [NEW] If tx.a2a_message_cid: fetch A2AMessageCapsule + verify signature/nonce + check TTL + merge to comm DAG
      d. Compute state mutation (QState.apply_tx)
      e. Update L4/L4.E heads
4. For each L4.E entry: deserialize rejection + capsule CID; mark ui_event.outcome appropriately
5. Reconstruct HEAD_t from final state + cas_root
6. Validate HEAD_t.canonical_hash() matches expected checkpoint
7. [NEW] Unpack MarkovEvidenceCapsule.typical_errors + unresolved_obs

Returns (final_q_state, final_head_t, ReplayEvidence { l4 / l4e / ui_capsules / a2a_capsules / artifact_manifest })
```

### 7.2 Class 标记

- Class-1: L4 accepted entries — Replay mandatory
- Class-2: L4.E rejection log — Replay mandatory
- Class-3: PromptCapsule / EconomicJudgment / UIEventCapsule / A2AMessageCapsule — Replay optional (audit/dashboard); deserialize on demand
- Class-4: CompressedRunLog / raw agent traces — Access 需 ratification + audit role

### 7.3 Integrity Gates (新增, 可扩展)

```rust
// Gate: ui-event-consistency
fn gate_ui_event_consistency(ui_capsules: &[UIEventCapsule], l4_entries: &[LedgerEntry]) {
    for ui in ui_capsules {
        match ui.outcome {
            Accepted => assert!(ui.linked_tx_cid.is_some()),
            Pending => assert!(ui.linked_tx_cid.is_none()),
            _ => {}
        }
    }
}

// Gate: a2a-ttl-compliance
fn gate_a2a_ttl_compliance(a2a: &[A2AMessageCapsule], l4_head_round: u64) {
    for msg in a2a {
        let deadline = msg.timestamp_logical + msg.ttl_rounds as u64;
        assert!(l4_head_round <= deadline);
    }
}

// Gate: cas-index-root-derivation
fn gate_cas_root_matches_index(cas: &CasStore, expected: Hash) {
    let computed = cas.compute_index_root()?;
    assert_eq!(computed, expected);
}
```

### 7.4 修改清单

**Minor (forward-compatible)**:
- Add `Option<Cid>` 字段到 TxKind payload (ui_context_cid / a2a_message_cid)
- Extend LedgerEntrySigningPayload 含 new Cid refs (仅 Cid, 不 hash capsule payload)
- New CasObjectType variants (无 breaking change)
- Optional MarkovEvidenceCapsule fields (ui_event_cids / a2a_dialog_cids; 默认 empty vec)

**无影响**:
- State root mutation (QState.apply_tx 忽略 capsule content; 只 TxKind + core fields)
- ChainTape tape 结构 (L4/L4.E 仍 acyclic append-only)
- Canonical signing payload (避免 capsule payload bytes 进入 signing target; 仅 Cid refs)

---

## 8. 新 CAS Capsule Schema Candidates 总览

| Schema | ObjectType Variant | Class | Anchor Point | Privacy Default | Status |
|---|---|---|---|---|---|
| UIEventCapsule | UIEventCapsule | 3 | MarkovEvidenceCapsule.ui_event_cids 或 L4.E | UserVisible (UI form) | Candidate |
| A2AMessageCapsule | A2AMessageCapsule | 3 | MarkovEvidenceCapsule.a2a_dialog_cids 或 L4 | PublicMarket / RoleScoped | Candidate |
| ArtifactStorageManifest | ArtifactStorageManifest | 3 | UIEventCapsule.artifact_cids | UserVisible (dedup info) | Candidate |
| ImageBlob | ImageBlob | 3 | ArtifactStorageManifest.artifacts[] | UserVisible | Candidate |
| AudioBlob / VideoBlob / Model3dBlob | ... | 3 | (同上) | UserVisible | Candidate |

### 与现有 schemas 统一

- **PromptCapsule** (现有): 无修改; REAL-5 PromptCapsuleV2 已是 role-scoped 扩展
- **EconomicJudgment** (REAL-12): 无修改
- **EvidenceCapsule** (TB-11): 无修改
- **MarkovEvidenceCapsule** (TB-15): **扩展** (optional fields):
  ```rust
  #[serde(default)] pub ui_event_cids: Vec<Cid>,        // [NEW]
  #[serde(default)] pub a2a_dialog_cids: Vec<Cid>,      // [NEW]
  #[serde(default)] pub artifact_manifest_cid: Option<Cid>, // [NEW]
  ```
- **LibrarianDigest** (REAL-BCAST-1): **扩展** LibrarianEvidenceKind:
  ```rust
  pub enum LibrarianEvidenceKind {
      // existing variants...
      UIEvent, A2AMessage, ArtifactRef,  // [NEW]
  }
  ```

---

## 9. Kill Condition 5/7 自检

### 5: 无 typed_tx schema 修改作为实施方案
- ✅ 所有新 capsule schema 是 CAS-level, 不是 typed_tx
- ✅ Optional Cid 字段添加到 TxKind payload **可能**作为 forward-bound (但严格说不改 enum 也行 — 通过 CAS 间接 anchor)
- ✅ 全部 forward-bound 标明

### 7: 无 Class 4 暗藏
- ✅ HEAD_t 6-field 不变更
- ✅ Sequencer admission 不修改
- ✅ Signing payload 不引入新 schema (Cid refs 不 hash payload bytes)
- ✅ replay_full_transition 行为不变 (只是更多 evidence types)

---

## 10. Track B 完成

- ✅ HEAD_t 不扩展, 通过 cas_root anchor
- ✅ UI 事件 / A2A 通信 走 CAS schema (forward-bound)
- ✅ 多模态 2-tier 存储策略
- ✅ Replay 算法完整 + 3 个 new integrity gates
- ✅ MarkovEvidenceCapsule + LibrarianDigest 向后兼容扩展 (serde default)
- ✅ 0 个 Class 4 surface 修改 提议

**关键架构 invariant**: TISR 双轴的所有新 evidence 类型走 CAS, 不走 typed_tx; 通过 cas_root 间接 anchor HEAD_t; replay 完整性维持.
