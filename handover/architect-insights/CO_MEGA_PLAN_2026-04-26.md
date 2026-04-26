# Constitutional Operationalization MEGA PLAN (CO_MEGA_PLAN)

> **Plan v3** — supersedes TFR_MASTER_PLAN_2026-04-26 (TFR v1) which only covered ~20% of the user white paper scope.
>
> **Authoritative spec**: `handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md` (user-authored 2026-04-26)
>
> **Status**: ArchitectAI v1 outline draft, awaiting user GO + external dual audit. NOT a full atom-by-atom plan; this is the **structural framework** that v2 will atomize comprehensively.

---

## § 1 Mission Recalibration

### 1.1 Why this plan supersedes TFR v1

Plan v1 (TFR) treated **ChainTape** as the project center. The user white paper § 1 explicitly corrects this: **反奥利奥架构 is the project center; ChainTape is one implementation of tape_t**. Same words rearranged, but the architectural impact is enormous:

- TFR v1 had 1 tape upgrade (S1-S3 sprints)
- White paper requires: explicit 3-layer codification (Top/Middle/Bottom White Boxes) + 6-layer ChainTape + 8-component Q_t + Predicate/Tool registries + RSP economy + Goodhart shielding

TFR v1 thus covers maybe **20%** of white paper specs. The other 80% is genuinely new architectural work.

### 1.2 Honest scope assessment

**Full operationalization of white paper**: ~18-22 weeks (4.5-5.5 months)
- CO Phase 1 (GitTape + 反奥利奥三层 explicit codification): 8-10 weeks (extends TFR v1)
- CO Phase 2 (LedgerTape + RSP economy rebuild): 6-8 weeks
- CO Phase 3 (MetaTape — ArchitectAI runtime + Meta transitions): 4-6 weeks

**Scoped-MVP version** (white paper § 17 Phase 1 only + RSP MVP): ~10-12 weeks
- MVP excludes: Phase 4 Permissioned ChainTape, Phase 5 Public Settlement (out of v4 scope per user 实施路线)
- MVP excludes: full Goodhart hidden-test machinery (Phase 3 work)

### 1.3 PREREG impact

The 30-day PPUT-CCL arc cannot survive 4-5 months of refactor. Two realistic options:

| Option | PREREG handling | User cost |
|---|---|---|
| **OPT-1 (recommended)** | Declare PPUT-CCL arc PAUSED; reserve heldout-54 sealing; resume after CO Phase 2 exit | 4-5 month delay; clean architectural foundation |
| **OPT-2** | Declare PPUT-CCL arc NEGATIVE (Phase E never reached); restart with new PREREG post-CO | abandon current arc; no Phase E result for v4 |
| **OPT-3** | MVP-only (CO Phase 1 + RSP MVP); resume PPUT-CCL after that | 10-12 week delay; 80% architectural alignment |

ArchitectAI recommends **OPT-3 (MVP)** — pragmatic balance.

---

## § 2 Anti-Oreo as Plan Spine

The white paper's 反奥利奥三层 (§ 3) is the organizing principle. Every code symbol must be classifiable into ONE of three layers:

```text
⚪ Top White Layer  → src/predicates/* + src/signals/* + src/budget/*
⚫ Middle Black Layer → experiments/.../agents/*
⚪ Bottom White Layer → src/tools/* + src/cas/* + src/ledger/* + src/wal/* + src/sandbox/* + src/permission/* + src/materializer/*
```

**Current code map (audit)**:
- `src/bus.rs` is currently a **mixed-layer module** — does append (Bottom) + predicates (Top) + market broadcast (Top) + invest path (mixed). Per Anti-Oreo, must split into multiple files cleanly aligned to one layer each.
- `src/kernel.rs` mixes tape (Bottom) + market (Bottom logic, but consumed by Top broadcast) + bounty (Bottom).
- `experiments/minif2f_v4/src/bin/evaluator.rs` is the swarm orchestrator — Middle layer wrapper, but currently embeds Top concerns (predicate dispatch, market_ticker assembly).

**Refactor implication**: not just add new modules; split existing modules so each conforms to one layer. This is more work than TFR v1 assumed.

---

## § 3 Q_t Expansion to 8 Components

White paper § 4 defines:
```
Q_t = ⟨q_t, HEAD_t, state_root_t, tape_view_t, ledger_root_t, budget_state_t, predicate_registry_root_t, tool_registry_root_t⟩
```

Current code has: ~`q_t` (split across acc + bus state), `HEAD_t` (none), `tape_t` (Vec<Node>). 5 fields completely missing.

Per CO Phase 1, `QState` struct (introduced in TFR v1 S2.4) must be expanded to all 8:
```rust
pub struct QState {
    pub q_t: AgentSwarmState,                  // 已有 partial
    pub head_t: NodeId,                        // 缺失 (Path B git commit SHA)
    pub state_root_t: Hash,                    // 缺失 (git tree root)
    pub tape_view_t: AgentVisibleProjection,   // 缺失 (S5 work)
    pub ledger_root_t: Hash,                   // 部分 (Wal exists; root hash 缺失)
    pub budget_state_t: BudgetSnapshot,        // 缺失 (currently in WalletTool side)
    pub predicate_registry_root_t: Hash,       // 缺失 — entire L1 layer 缺失
    pub tool_registry_root_t: Hash,            // 缺失 — entire L2 layer 缺失
}
```

Each missing field → atomic implementation work in CO Phase 1.

---

## § 4 ChainTape 6-Layer Mapping (white paper § 5)

| Layer | White paper spec | Current state | CO Phase to implement | Approximate atoms |
|---|---|---|---|---|
| **L0 Constitution Root** | constitution_hash + human_signature + sudo_policy | ✅ partial (genesis_payload.toml + R-018 hook) | CO P1.0 (formalize) | 2 |
| **L1 Predicate Registry** | predicate_id + version + code_hash + input/output schema + visibility_policy + owner + test_suite_hash | ❌ missing — predicates inline in bus.rs | CO P1.5 — new `src/predicates/registry.rs` | 6-8 |
| **L2 Tool Registry** | tool_id + version + capability + schema + permission_policy + determinism_level + side_effect_class | ⚠️ partial — TuringTool trait exists, no formal registry | CO P1.6 — formalize | 4-5 |
| **L3 CAS Object Store** | cid + hash + schema + type + creator + visibility + encryption_policy | ❌ missing — payload as inline String | CO P1.4 — git's blob storage IS our CAS in Path B | 3-4 |
| **L4 Transition Ledger** | tx_id + parent_state_root + agent_id + read_set + write_set + proposal_cid + predicate_results + stake + signature + timestamp + status | ⚠️ partial — Wal + EventType exist but missing 6 of 11 fields | CO P1.7 — full schema | 5-7 |
| **L5 Materialized State + Agent Read View** | current_state_db + task_index + agent_reputation_index + error_taxonomy_index + price_signal_index + permission_view | ⚠️ partial — bus.snapshot, recent_rejections; no formal indexes | CO P1.8 + CO P2.X | 6-8 |
| **L6 Signal Indices** | boolean pass/fail + typical errors + price + reputation + scarcity + explore/exploit stats | ⚠️ partial — scattered | CO P1.9 — unify | 4-5 |

**Total ChainTape atom count**: ~30-40 atoms in CO Phase 1. (TFR v1 had ~25; gap is L1, L2, L3, L6 explicit formalization.)

---

## § 5 Public/Private/Commit-Reveal Predicates (Goodhart Shield)

White paper § 5.L1 + § 9.4 mandates:
- **public predicates**: schema, permission, basic tests — visible to Agent
- **private predicates**: hidden benchmarks, anti-Goodhart evaluators — Agent cannot read
- **commit-reveal predicates**: hash committed first, sample revealed later — anti-Goodhart

**Current**: ALL predicates are inline in bus.rs (forbidden_patterns, payload size limits, oracle Lean verify). Agent can in principle read source code via bus.snapshot if not blocked, but more importantly: **agent CAN train against forbidden_patterns** (Goodhart attack vector).

**CO Phase 1 atom**: Predicate Registry must encode `visibility_policy: Public | Private | CommitReveal`. Agent's read view filters predicates by visibility.

---

## § 6 RSP (Reward Settlement Protocol) — Phase 2 Scope

White paper RSP § 1-16 specifies a complete economy. Current TuringOS economy is partial (per `ECONOMIC_MECHANISM_AUDIT_2026-04-26.md`):

**What exists**: market/wallet/invest/short, halt_and_settle (gated by TAPE_ECONOMY_V2)
**What's missing per white paper RSP**:

| RSP component | Current state | CO Phase 2 atom count |
|---|---|---|
| TaskMarket (pre-locked bounty contracts) | ❌ missing — bounty_market is single global, not per-task | 4 |
| EscrowVault | ❌ missing — wallet has no escrow column | 3 |
| ContributionLedger (signed work_tx / verify_tx / challenge_tx) | ❌ missing — Invest event detail=None (V-05) | 5-6 |
| PredicateRunner (separates acceptance + settlement predicates) | ⚠️ partial — runs predicates but no separation | 3 |
| AttributionEngine (Contribution DAG → reward weights) | ❌ missing — currently tool_dist["invest"] is informational | 5-7 |
| ChallengeCourt (challenge window + slashing + bonus freeze) | ❌ missing | 4-5 |
| SettlementEngine (deterministic payouts) | ⚠️ partial — settle_portfolios exists, but not 3-layer | 3-4 |
| ReputationIndex (success / failure / reuse / verifier quality) | ⚠️ partial — `reputation_at_end` is solve count, not reputation | 2-3 |
| Verifier + Challenger market roles | ❌ missing — only Solver role currently | 5-6 |
| 3-layer rewards (immediate / deferred / reuse royalty) | ❌ missing — only one-shot settlement | 4-5 |
| CTF stake symmetry (Solver YES + Challenger NO + Verifier reputation) | ⚠️ partial — BetDirection::Long/Short exists | 3 |

**Total CO Phase 2 atom count**: ~35-50 atoms. This is the BIGGEST gap relative to white paper.

---

## § 7 CO Phase Structure

### CO Phase 0 — Foundation (1 week)

| Atom | Scope |
|---|---|
| **CO0.1** | Save white paper to repo as canonical spec (this commit) |
| **CO0.2** | Save this plan v3 outline (this commit) |
| **CO0.3** | Constitution amendment Art. 0.5 — declare white paper as authoritative; integrate 6 公理 |
| **CO0.4** | PREREG amendment v2 — chose OPT-1 / 2 / 3 (user decision); arc state |
| **CO0.5** | TFR v1 plan deprecated formally (rename to legacy) |
| **CO0.6** | Trust Root migration — add white paper + plan v3 + amendment v2 |
| **CO0.7** | External dual audit on plan v3 (Codex + Gemini) — PASS/PASS gate to CO Phase 1 |

### CO Phase 1 — GitTape + 反奥利奥 explicit codification (8-10 weeks)

| Sub-phase | Scope | Atoms | White paper reference |
|---|---|---|---|
| **CO P1.0** | Constitution Root formalization (L0) | 2 | § 5.L0 |
| **CO P1.1** | Anti-Oreo 3-layer module split (src/{top,middle,bottom}/*) | 4-6 | § 3 |
| **CO P1.2** | Q_t struct expansion to 8 components | 3-4 | § 4 |
| **CO P1.3** | gix substrate integration (TFR v1 S0.3-S0.4 atoms preserved) | 3 | § 13.2 |
| **CO P1.4** | CAS layer (L3) — git blob storage abstraction | 3-4 | § 5.L3 |
| **CO P1.5** | Predicate Registry (L1) + visibility policy | 6-8 | § 5.L1 + § 9.4 |
| **CO P1.6** | Tool Registry (L2) + capability classification | 4-5 | § 5.L2 |
| **CO P1.7** | Transition Ledger (L4) — full 11-field schema | 5-7 | § 5.L4 |
| **CO P1.8** | Materialized State + Agent Read View (L5) | 6-8 | § 5.L5 + § 9.2 |
| **CO P1.9** | Signal Indices (L6) unification | 4-5 | § 5.L6 |
| **CO P1.10** | Boolean vs Statistical signal explicit (§ 7) | 3-4 | § 7 |
| **CO P1.11** | Safety vs Creation domain fail-policy (§ 7.2) | 2-3 | § 7.2 |
| **CO P1.12** | Tape Canonical Axiom 24 V-violations + 4 E-violations resolution | covered by L4-L6 atoms | (TFR v1 carry-over) |
| **CO P1.13** | TRACE_MATRIX_v3 — every white paper sentence → code symbol | 3 | DO-178C |
| **CO P1.14** | CO P1 exit ceremony — dual audit + heldout sealing re-validation | 1 | gate |

**Total CO P1 atoms**: 50-65; **wall-clock**: 8-10 weeks.

### CO Phase 2 — LedgerTape + RSP economy rebuild (6-8 weeks)

| Sub-phase | Scope | Atoms | White paper reference |
|---|---|---|---|
| **CO P2.1** | TaskMarket — pre-locked bounty contracts | 4 | RSP § 3 |
| **CO P2.2** | EscrowVault | 3 | RSP § 12 |
| **CO P2.3** | ContributionLedger — signed work_tx/verify_tx/challenge_tx | 5-6 | RSP § 6, § 13 |
| **CO P2.4** | AttributionEngine — Contribution DAG | 5-7 | RSP § 6 |
| **CO P2.5** | ChallengeCourt — challenge window + slashing | 4-5 | RSP § 7.2, § 14.3 |
| **CO P2.6** | SettlementEngine — 3-layer rewards (immediate/deferred/royalty) | 4-5 | RSP § 5 |
| **CO P2.7** | Verifier + Challenger roles | 5-6 | RSP § 7 |
| **CO P2.8** | CTF stake symmetry rewrite | 3 | RSP § 8 |
| **CO P2.9** | ReputationIndex (proper, not solve-count alias) | 2-3 | RSP § 6 |
| **CO P2.10** | E-01..E-04 closures (production default-on, jsonl summary, naming, founder grant Law 2 reconcile) | 4 | ECON_AUDIT |
| **CO P2.11** | RSP MVP-1 deployment as LedgerTape (white paper § 17 Phase 2) | 2 | § 17 |
| **CO P2.12** | CO P2 exit ceremony — dual audit | 1 | gate |

**Total CO P2 atoms**: 42-55; **wall-clock**: 6-8 weeks.

### CO Phase 3 — MetaTape (post-CO P2; 4-6 weeks; OPTIONAL within v4)

ArchitectAI runtime + JudgeAI/Veto-AI runtime + Meta transitions on tape. White paper § 12. Currently manual via dual audit; CO P3 makes it programmatic.

This is post-MVP; recommend **defer to v4.1 unless user explicitly wants MetaTape in v4**.

---

## § 8 PREREG / PPUT-CCL Arc Reconciliation

White paper Phase 1 (GitTape) ≈ CO Phase 1. White paper Phase 2 (LedgerTape) ≈ CO Phase 2. So white paper's first two phases ARE the v4 work.

**PPUT-CCL Phase C** (the H1-H4 ablation experiments) currently uses:
- `experiments/minif2f_v4/src/bin/evaluator.rs::run_swarm` — relies on bus.append, kernel.markets, etc.
- All these will be **rewritten** during CO Phase 1.

**PPUT-CCL must wait** for CO Phase 1 minimum exit (S1.14). Earliest restart: ~10 weeks from CO P0.7 audit PASS.

PREREG amendment v2 must explicitly:
- Document the freeze
- Specify "Phase A (PREREG draft) + Phase B (kernel instr) results pre-CO are PRESERVED but not used inferentially"
- Specify "Phase C C2 batch results are produced ONLY post-CO P1.14"
- USD budget extension if needed (CO Phase 1 dual audits estimated ~$300-500 alone)

---

## § 9 Team Structure

### Internal (Claude Opus 4.7 1M context)
- **Main thread** — orchestration, sprint launches, validation
- **`auditor` subagent** — read-only constitutional traceability
- **`Plan` subagent** — sprint design + risk register
- **`Explore` subagent** — codebase navigation
- **`code-simplifier`** — post-commit hygiene

### External (per CLAUDE.md Audit Standard)
- **Codex (`codex:rescue`)** — independent investigation + plan review
- **Gemini DeepThink** — strategic architectural review
- **Conservative merge**: VETO > CHALLENGE > PASS at every gate

### Audit cadence
- **CO Phase exit**: full dual external audit (Codex + Gemini), PASS/PASS required before next phase entry. Estimated cost: ~$30-50 per phase exit (CO P0/P1/P2 = ~$90-150)
- **STEP_B atom-level external audit**: per atom touching `src/{bus,kernel,wal,ledger}.rs` + new restricted files (`src/predicates/*`, `src/tools/*`, `src/cas/*`). ~$5-10 per atom × ~25 STEP_B atoms = ~$125-250
- **Total external audit budget**: ~$215-400 over CO Phase 0+1+2

---

## § 10 DO-178C TRACE_MATRIX_v3

User's mandate: "代码-宪法的一一对照，让每一行代码有意义，让宪法的每一句话有真实落地"

**Implementation strategy**:
1. **TRACE_MATRIX_v3.md** — bidirectional table:
   - Constitution clause / White paper paragraph → code symbol(s)
   - Code pub symbol → backlink to constitution / white paper
2. **Production invocation column** (added per F-2026-04-26-01 lesson) — every code symbol must show: which production runner script invokes it. Catches "implemented but unused" failures (E-01 class).
3. **Conformance test**: `tests/trace_matrix_v3_bidirectional.rs` — rust crate `syn` parses doc-comments + a markdown parser parses matrix; asserts no orphans.

**Update protocol**: every atom commit touching pub symbols MUST update matrix in same commit. Pre-commit hook (R-022, new) enforces.

---

## § 11 Risk Register (Top-10)

| # | Risk | Probability | Impact | Mitigation |
|---|---|---|---|---|
| 1 | gix capability gap (multi-parent, hooks) | Med | High | S0.4 spike before S1; git2-rs fallback |
| 2 | 4-5 month scope kills user motivation | High | Catastrophic | Offer scoped MVP (OPT-3); weekly progress reports |
| 3 | RSP Contribution DAG attribution becomes subjective | Med | High | Lock attribution_policy_hash at task creation; resists post-hoc tweaking |
| 4 | Predicate Registry visibility policy leakage | Med | High | Air-gap private predicates from agent prompt builder; CI test |
| 5 | Anti-Oreo module split breaks existing tests | High | Med | Phased migration via shim layer (TFR v1 § 2.11 pattern preserved) |
| 6 | Phase E heldout integrity broken by CAS layer | Low | Catastrophic | CO P1.14 explicit re-validation of L1-L5 sealing layers |
| 7 | Production invocation gap (E-01 class regression) | Med | Med | TRACE_MATRIX_v3 production-invocation column + CI gate |
| 8 | External audit budget overrun | Med | Med | Per-phase USD budget tracking; escalation gate |
| 9 | Blockchain/CAS dependency hell | Low | Med | gix is pure Rust; no system deps |
| 10 | User trust erosion from any further fault | High | Catastrophic | Honest reporting; no false-completeness claims; dual audit at every gate |

---

## § 12 Decision Points (User MUST Decide)

### D1 — PREREG amendment scope
- **A**: OPT-1 PAUSE (recommended-balanced) — heldout-54 reserved, arc resumes after CO P2
- **B**: OPT-2 NEGATIVE (declare arc failed) — start fresh PREREG post-CO
- **C**: OPT-3 MVP (recommended-pragmatic) — CO Phase 1 + RSP MVP only, ~10-12 weeks; full MetaTape deferred to v4.1
- **D**: Sudo your own variant

### D2 — Constitution amendment Art. 0.5 (white paper integration)
- **A**: Ratify white paper as Art. 0.5 with full 18-section text → constitution.md grows ~2x — comprehensive
- **B**: Ratify pointer + 6 公理 only; full text stays in handover/whitepapers/ — minimal constitution growth
- **C**: Sudo your own variant

ArchitectAI recommends **B** — keeps constitution.md a high-level spec, white paper is the elaboration. Both Trust-Rooted.

### D3 — TFR v1 disposition
- **A**: Deprecate TFR v1 (`TFR_MASTER_PLAN_2026-04-26.md`) but preserve for history; supersede with this plan v3
- **B**: Delete TFR v1
- **A is correct**.

### D4 — CO Phase 3 (MetaTape) inclusion in v4 scope
- **A**: Include in v4 (~22 weeks total)
- **B**: Defer to v4.1 (~16 weeks total for CO P0+P1+P2 only)
- ArchitectAI recommends **B**.

### D5 — RSP economy implementation depth
- **A**: Full RSP per white paper (immediate + deferred + reuse royalty + Verifier + Challenger market)
- **B**: RSP MVP (immediate + deferred only; Verifier role; no Challenger market yet)
- **C**: RSP-min (immediate only; no deferred; no royalty; Verifier optional)
- ArchitectAI recommends **B** — sufficient signal for white paper § 17 Phase 2 validation; full RSP becomes v4.1.

### D6 — External audit cadence
- **A**: Full CLAUDE.md standard (per phase + per STEP_B atom) — ~$215-400
- **B**: Reduced (per phase only; STEP_B atoms internal-only) — ~$90-150
- **C**: Sudo your own
- ArchitectAI recommends **A** — given the trust erosion this turn, audit rigor is the antidote.

---

## § 13 Self-Audit on This Plan

What this plan v3 gets RIGHT:
- Acknowledges TFR v1 was 20% scope; not pretending v3 is incremental
- Maps every white paper section to atom count
- Explicit TRACE_MATRIX_v3 production-invocation column lessons (anti-E-01 class)
- Three OPT choices for PREREG, with recommendation
- Honest 4-5 month timeline + 10-12 week MVP option
- Scope reduction options (D4, D5)

What's deliberately deferred to plan v3.1:
- Per-atom file path enumeration (this is plan v3 OUTLINE; v3.1 atomizes)
- PREREG_AMENDMENT_v2.md text (drafted in CO P0.4)
- Constitutional Art. 0.5 text (drafted in CO P0.3 after D2 user choice)
- Sprint dependency graphs (atoms within phases enumerated, but not linked yet)

What I'm honest about being uncertain on:
- gix multi-parent commit support (S0.4 spike will validate)
- Contribution DAG subjectivity vs determinism — design challenge for CO P2.4
- Long-arc user motivation — Risk #2 has no good mitigation other than transparency
