---
name: plan-grill
description: Fact-based orchestrator↔human disambiguation protocol. Use when the orchestrator hits a decision-fork it cannot resolve alone (preference, authorization, risk tolerance, scope boundary). NOT for excavating latent user needs — use grill-recursive for that. Auto-invoked from SKILL.md at Phase 0 + Phase 6.
allowed-tools: ["AskUserQuestion"]
modes: [single-fork, batched-cluster, authorization-check, contradiction-resurfacing]
---

# Plan-Grill — Fact-based orchestrator↔human disambiguation

This file is a sub-protocol auto-invoked by `./SKILL.md`. End-users do
not invoke it directly. Orchestrators reading SKILL.md consult this file
at Phase 0 (triage uncertain difficulty / scope) and Phase 6 (must-fix
items that need user authorization).

## Core distinction (load-bearing)

> **Plan-grill `disambiguates` what the orchestrator can't determine.**
> **`grill-recursive` (TuringOS skill) `excavates` what the user hasn't said.**

Excavation is for non-developer users with fuzzy needs — open-ended,
psychological, latent-need-surfacing. Disambiguation is for human
collaborators with concrete uncertainty — closed-option, fact-based,
trade-off-explicit. The same Grill-me word covers two different verbs;
this file names them apart and stays in the disambiguation domain.

## When to fire plan-grill (3-condition gate)

Fire plan-grill ONLY when ALL three hold:

1. A decision is needed to proceed (orchestrator cannot move forward).
2. The decision cannot be made by the orchestrator alone:
   - User preference (style, aesthetic, naming, taxonomy)
   - User authorization (sudo, scope extension, irreversible action)
   - User risk tolerance (data loss acceptable? rollback strategy?)
   - Scope boundary (in vs out of this iteration)
   - A fact only the human knows (deadline, business context, external
     constraint not in the codebase)
3. The decision has CONCRETE alternatives — not "what do you think?"

If any condition fails:
- (1) fails → just proceed
- (2) fails → orchestrator makes the call + documents reasoning
- (3) fails → orchestrator keeps exploring (Phase 1 Researcher); plan-grill
  is the last resort, not the first

## Question format (MANDATORY structure)

Every plan-grill question MUST have:

- **Stem** ≤30 words, factual not interpretive
- **2-4 mutually-exclusive options** — no "Other" option; force a
  decision-fork
- Each option:
  - Short label (1-5 words)
  - 1-sentence description **including the trade-off**
- ONE option marked `(推荐)` / `(recommended)` if the orchestrator has a
  strong default — but the recommendation is justified inline, not bare

### Concrete shape

```
Question:  <≤30-word factual stem>

Options:
  A. <label> — <one-sentence description with trade-off>
  B. <label> — <one-sentence description with trade-off>
  C. <label> (推荐) — <one-sentence description with trade-off + why
     this is the orchestrator's default>
  D. <label> — <one-sentence description with trade-off>
```

## Batching rules

- **1 question/turn** — OK for a single critical fork
- **2-4 questions/turn** — OK if they're related (same feature, same
  decision cluster, same iteration scope)
- **>4 questions/turn** — ANTI-PATTERN. The human can't compare options
  across too many axes simultaneously. Split into rounds: ask the most
  load-bearing decisions first; subsequent decisions depend on those
  answers.

## Anti-patterns (these belong in grill-recursive, NOT plan-grill)

Plan-grill explicitly forbids these phrasings:

- "What were you doing when you wished there was a tool?" — psych-excavation
- "Why is this important to you?" — latent-need probing
- "Tell me more about your goal" — open-ended discovery
- "How do you feel about X?" — emotional context
- "Do you have other thoughts?" — open-ended without options
- "Which would you prefer?" without describing options
- Any question the orchestrator should answer itself by exploring code/docs

If the orchestrator notices itself drafting one of these phrasings, it has
likely confused the domain — go back to Phase 1 Researcher or just decide.

## Verdict protocol (what to do with the answer)

| User reply | Orchestrator action |
|---|---|
| Matches an option exactly | Proceed with that choice |
| Custom text (not one of options) | Interpret, restate back: "理解为 X，对吗?" — proceed on confirm |
| "You decide" / "随便" / "都行" | Pick the `(推荐)` option + state reasoning explicitly in next turn so user can override later |
| Contradicts a previous lock-in decision | Flag the contradiction + ask for explicit confirmation before unlocking |
| Adds context that changes the question | Re-ask with revised options; do NOT silently re-interpret |

## Case studies (from real shipping cycles)

### Case 1: Decision-fork — "Polymarket 接哪个 stage？"

Context: Polymarket integration into an existing user-facing loop. The
orchestrator had researched 4 possible integration points but no
single one was clearly correct.

```
Q: Polymarket 应该接进当前 loop 的哪个环节作为首个集成点？

A. Generate stage (推荐) — 每次 GenerationAttempt 是一个 Node;
   多 Worker 并行生成 candidate artifact; 市场打价"哪个 artifact 最
   可能通过 verify"; 最贴近 TB-10 Proof Task Market MVP 模式,
   GenerationAttemptCapsule 已存在,rebuild 成本最低。
B. Spec stage — 每个 Spec section 是一个 Node; 多 Grill persona
   并行 interview; 挑战: 需要新增 multi-persona grill 编排。
C. Artifact stage — 最终 artifact 是 Node; market 在 Generate 完成
   后开市,作为 ChallengeResolve 的预测信号。
D. 全 loop 同时接 — 每个里程碑都开市; 工作量最大,稀释 MVP 焦点。
```

User picked A. Showed: orchestrator's `(推荐)` mark + the reasoning chain
("rebuild 成本最低 because GenerationAttemptCapsule 已存在") let the user
trust the default fast.

### Case 2: Authorization-check — "Stage boundary"

Context: Adversarial Constitution agent flagged the planned PR would
ship Polymarket P-M9 as a production endpoint, but ROADMAP placed
Polymarket trading at Stage C/D requiring architect §8 sign-off.

```
Q: ROADMAP stage 跳过需要 Class-4 sudo。怎么处理？

A. 明示 Class-4 sudo 授权 stage-jump (推荐 if user has authority) —
   现在直接上真市场; 需要 PR body 写 §8 ratification + constitution
   §5.3 修订条目。
B. 重新 scope: 只做"可视化模拟",不上真市场 — web 层调
   `turingos tournament --simulate` CLI,不带 Coin; 不需 §8 sudo;
   stage-A 文档级别。
C. 推迟整个 feature 直到 Stage A AMBER 关闭 + Stage B 基准就位
   — 保守; user 看不到 demo。
```

User chose to grant authorization but with a clarification: "kernel 必须
一致,但 central bank 可以先有"。Showed: only the human has the authority
to sudo; the question forced explicit user accountability.

### Case 3: Contradiction-resurfacing

Context: Earlier in the planning, user said "include ChallengeTx but
no dedicated critic bot". Constitution adversarial review then showed
this combination violates Art. III.3 horizontal-independence.

Plan-grill resurfaced the trade-off cleanly without rewriting the
earlier decision:

```
Q: Constitution review 证实: peer-Worker ChallengeTx 违 Art. III.3
   horizontal-independence (3 个 LLM 互看 artifact = 假独立样本)。
   怎么处理？

A. 从 MVP 删 ChallengeTx (推荐) — 3 Worker 各交 WorkTx → predicates
   选 winner → 完; ChallengeTx 推到 PR3 + 配 dedicated critic bot。
B. MVP 加 dedicated ChallengeAI critic bot — 跟你上轮拒绝的方案
   一致,但是唯一合宪路径; 工作量 +1 critic LLM。
C. 保留 peer-challenge + Class-4 sudo 显式接受 Art. III.3 违规 —
   作为对抗实验; audit trail 必须留。
```

User answered "严格守宪法" — orchestrator interpreted as choosing A
(remove from MVP) and continued. Showed: when an earlier decision is
constitutionally incompatible, plan-grill brings it back with explicit
trade-offs rather than orchestrator silently overriding.

## Relationship to SKILL.md

- **Phase 0** (`./SKILL.md`): when orchestrator can't classify task
  difficulty alone → fire plan-grill
- **Phase 6** (`./SKILL.md`): when must-fix items require user
  authorization → fire plan-grill
- Plan-grill is also usable **STANDALONE** — outside any orchestration
  workflow, a solo orchestrator clarifying with a human still benefits
  from this protocol

## Anti-pattern: psychologizing the user

If the orchestrator catches itself wanting to ask "what's your underlying
goal here?" — that's the excavation verb, NOT disambiguation. Two paths:

1. The orchestrator's exploration was insufficient — go back to Phase 1
   Researcher and gather more facts
2. The task genuinely needs end-user spec elicitation (rare in
   orchestration contexts) — use a different skill, not this one

## When NOT to use plan-grill

- End-user is a non-developer with fuzzy need — use `grill-recursive`
  (TuringOS spec elicitation skill) instead
- Orchestrator already has enough info to decide — just decide
- Task is in pure read-only exploration — Phase 1 Researcher handles it
- The decision is reversible and low-stakes — orchestrator picks +
  documents, user can override later if needed

## Related skills

- `./SKILL.md` — the orchestrator workflow that invokes this
- `./interface-contract-lock.md` — locked contracts before parallel dispatch
- `./auditors.md` — alive auditor menu for Phase 5
- (External) `skills/grill-recursive.md` — TuringOS-style psychological
  excavation, the OTHER grill verb; explicitly contrasted here
