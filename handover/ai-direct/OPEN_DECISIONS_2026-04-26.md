# Open Decisions — 2026-04-26 Session

**Purpose**: questions waiting for user input. New sessions should resolve OR defer-with-default these before continuing execution. NOT a brainstorm dump — only items that block forward progress or are irreversible.

> Each item: question, default, reversibility, what's blocked. When user answers, MOVE the row to a "Resolved" section + cite the decision in the relevant artifact.

## Pending (block forward execution)

### D1 — Phase B 启动节奏
- **Question**: 现在直接开 Phase B (B1 JSONL schema 半天破冰)？还是收 session / 新 session 起？
- **Default if no response**: 新 session 起 — Phase B 是 ~7 天 wall-clock 持续工作，不适合疲劳状态启动。
- **Reversibility**: fully reversible (nothing committed yet for Phase B)
- **Blocks**: Phase B B1 start

### D2 — `p_0` calibration toggle 设计 (PREREG § 5.5)
- **Question**: 当前 toggle = `--simulate-rollback-at-tx-50`. 这是 Claude pre-reg 起草的占位实现。push-back 余地：
  - tx-50 太晚 (大多数 problem 在 tx 50 之前已完成)？候选: tx-25 / tx-10 / per-call probabilistic corruption / agent-state reset
  - rollback 形式：完全重置 Tape vs 部分污染上下文 vs 注入 fake error trace
- **Default if no response**: 保留 `--simulate-rollback-at-tx-50`，但 B7-extra 实际跑前先做 1-题 smoke：如果 p_0 = 0/144 (toggle 无效)，redesign。
- **Reversibility**: fully reversible (toggle code 还没写)
- **Blocks**: B7-extra calibration runs (但不 block B1-B6)

### D3 — Phase B 是否走轻量外审？
- **Question**: PREREG § 6 Phase B 没强制 dual external audit。Phase C 起每 phase 末尾必双审。Phase B 是基建：
  - (a) 内审够 (cargo test 全过 + 11 anti-Goodhart 全过) → 直接进 Phase C
  - (b) Phase B 末做轻量外审 (仅 schema + Trust Root immutability test 结果)
- **Default if no response**: (a) — Phase B 末 audit packet 并入 Phase C 启动时的 dual audit
- **Reversibility**: fully reversible
- **Blocks**: only Phase B → C transition timing (~6 hours either way)

### D4 — Hermes-Agent ingest 提议 (NEW 2026-04-26)
- **Question** (用户原话): "能否用现在的 turingos 架构去做升级自己的项目, 学习一下 nousresearch/hermes-agent"。
- **Claude 提案**: `handover/proposals/HERMES_AGENT_INGEST_PROPOSAL_2026-04-26.md` (已写)
  - 推荐 **Option F + E**: F = 在 arc 内做 agentskills.io 兼容 frontmatter (~1 day + $5 round-5 audit)；E = post-arc Phase F 用 hermes-agent 跑同 heldout 做 external-validity benchmark (Paper 2 material)
  - 不推荐 A/C/D（违反 PREREG 锁定决策 / 短路 CCL 假设 / 不可逆架构改动）
- **Sub-decisions**:
  - **D4-a**: 批准 Option F (artifact schema alignment in-arc)? Default 跳过.
  - **D4-b**: 批准 Option E (post-arc Phase F benchmark)? Default 推迟 review.
  - **D4-c**: Hermes 还有什么值得吸收的（FTS5 memory search? subagent spawn pattern?）
- **Reversibility**: F 高 (单轮 audit); E 完全独立 (post-arc).
- **Blocks**: 不 block Phase B start; 若 D4-a 批准，需在 Phase D ArchitectAI artifact emitter 完工前 PREREG 加 addendum.

## Resolved (record only; do not delete)

(empty so far this session)

## Architectural decisions already locked (NOT re-openable without formal addendum)

- **Backbone**: deepseek-v4-flash thinking-off (Phase B+C)；Phase D heterogeneous = v4-flash thinking-on + Gemini 2.5 Pro
- **Three-split**: 60/20/20 hash-based; seed `20260426_PPUT_CCL`; realized 144/46/54
- **Heldout**: heldout-54 sealed (operational, not cryptographic per § 2.3)
- **Family size**: `4 + 3k`, `k_max = 10`, `N_max = 34`
- **Independent unit**: per-problem (NOT (problem, seed))
- **j-RR**: descriptive guardrail (NOT in inferential family)
- **Phase E protocol**: leave-one-out within sealed eval, k+2 sub-evals on same heldout-54 × 3 seeds
- **30-day cap + USD 500 budget**: hard stops both
- **Live human meta-predicate Phase D**: 48h SLA + deferred queue + ≥5-queued-48h Phase D abort
- **Trust Root**: per § 1.8 list; primary syscall EPERM + fallback lib-gate+panic
- **Anti-Goodhart**: 11 metering + 4 content + 4 lookup-evasion conformance tests gate Phase B Gate B exit

Any change to the above triggers formal addendum + dual external re-audit per C-070.
