# Open Decisions — 2026-04-26 Session

**Purpose**: questions waiting for user input. New sessions should resolve OR defer-with-default these before continuing execution. NOT a brainstorm dump — only items that block forward progress or are irreversible.

> Each item: question, default, reversibility, what's blocked. When user answers, MOVE the row to a "Resolved" section + cite the decision in the relevant artifact.

## Pending (block forward execution)

(none — D1-D4 resolved 2026-04-26 with default acceptance; new items go here)

## Resolved (record only; do not delete)

### D1 — Phase B 启动节奏 [RESOLVED 2026-04-26]
- **Decision**: default — 收 session，新 session 起 Phase B B1
- **User input**: "同意default" 2026-04-26
- **Action**: next session opens with `LATEST.md` + `PHASE_B_IMPLEMENTATION_PLAN.md` and starts B1 (JSONL schema v2)

### D2 — `p_0` calibration toggle 设计 [RESOLVED 2026-04-26]
- **Decision**: default — 保留 `--simulate-rollback-at-tx-50`，B7-extra 实际跑前先做 1-题 smoke；p_0 = 0/144 时 redesign
- **User input**: "同意default" 2026-04-26
- **Implementation hook**: `PHASE_B_IMPLEMENTATION_PLAN.md` § B7-extra; smoke gate added implicitly (Claude must confirm non-zero before launching 288-run batch)

### D3 — Phase B 是否走轻量外审 [RESOLVED 2026-04-26]
- **Decision**: default — 不外审；Phase B audit packet 并入 Phase C 启动时的 dual external audit (per PREREG § 6 Phase C C4)
- **User input**: "同意default" 2026-04-26
- **Implementation hook**: Phase C C4 audit packet 必须包含: Phase B Gate B 通过证据 + p_0 calibration 结果 + 11 anti-Goodhart conformance + 5-layer sealing tests + Trust Root immutability tests

### D4 — Hermes-Agent ingest [RESOLVED 2026-04-26]
- **D4-a Option F (in-arc agentskills.io alignment)**: default — 跳过
- **D4-b Option E (post-arc Phase F benchmark)**: default — 推迟 review，Phase E 完成后再决定
- **D4-c (其他可吸收的)**: default — 无新输入；保留 `handover/proposals/HERMES_AGENT_INGEST_PROPOSAL_2026-04-26.md` 作为后续参考
- **User input**: "同意default" 2026-04-26
- **Implication**: PREREG 不需 addendum；TuringOS-native artifact schema 保持；Phase F 候选项延后到 Phase E 结果之后再做去留判断

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
