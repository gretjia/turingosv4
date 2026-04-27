# TuringOS v4 — PROJECT DECISION MAP

> **Date created**: 2026-04-27 (post-Wave 3)
> **Purpose**: Permanent artifact tracking EVERY decision, EVERY skipped option, and EVERY atom's status. So that no context is lost across sessions.
> **Scope**: This doc is the canonical "where are we, what was decided, what was skipped, what's next" reference. Update on every decision fork.
> **Format**: each table marks fate of each option: ✅ CHOSEN / ⛔ PERMANENTLY RETIRED / ⏸ DEFERRED / 🚫 VETOED / ⏳ PENDING USER / 🔄 IN PROGRESS

---

## § 1 Project Identity

| Field | Value |
|---|---|
| Name | TuringOS v4 |
| Mission | Silicon-Native Microkernel for LLM Formal Verification Swarm (MiniF2F Lean 4) |
| Lead | gretjia (solo researcher; zero programming background; Chinese primary) |
| Team | Claude (orchestrator) + Codex (co-implementer/auditor) + Gemini (strategic reviewer) |
| Authority chain | constitution.md (supreme, frozen) > white papers (interpretation, finalized 2026-04-27) > spec docs > code |
| Budget | $890 mid ($580-1200 range); spent ~$25-50 (3-6%) |
| Timeline | 22-28 weeks (v4); v4.1 MetaTape runtime separate |
| HEAD at this doc creation | `0a791f6` |
| HEAD post-Wave-4 | `a44184b` (Wave 4-B+C-fix); preceded by `c2f94c6` (Wave 4-A+C) |

---

## § 2 Decisions Tree — Every Fork

Format: `<Date> <Decision-ID>: <Question>` → table of options with fate.

### 2.1 Foundation (D1-D6 top-level decisions)

#### 2026-04-26 D1 — PREREG / PPUT-CCL fate

| Option | Fate | Reason |
|---|---|---|
| A. PAUSE arc until CO P2 exit | ⛔ RETIRED | C MVP-pivot ratified |
| B. NEGATIVE arc (declare failed) | ⛔ RETIRED | C ratified |
| **C. MVP-pivot (50 rows × 1 seed sanity check after CO P1)** | ✅ CHOSEN | Plan v3.2 § 2 D1=C |
| D. user override | n/a | not invoked |

**Status**: PREREG_AMENDMENT_v2 DRAFT exists; ⏳ awaiting user enactment via cp workflow.

#### 2026-04-26 D2 — Constitution Art 0.5 form

| Option | Fate | Reason |
|---|---|---|
| A. Full white paper text inline (~2000 lines added to constitution.md) | ⛔ RETIRED | constitution kept compact |
| **B. Pointer + 6 公理 (~150 lines)** | ✅ CHOSEN | minimal authority delta |

**Status**: DRAFT exists; UNFROZEN 2026-04-27 via WP finalization tag; ⏳ awaiting user enactment.

#### 2026-04-26 D3 — TFR v1 disposition

| Option | Fate |
|---|---|
| **A. Deprecate banner + preserve content** | ✅ CHOSEN; LEGACY banner applied |
| B. Delete | ⛔ RETIRED |

#### 2026-04-26 D4 — MetaTape scope in v4

| Option | Fate | Reason |
|---|---|---|
| A. v4 includes runtime ArchitectAI/JudgeAI | ⛔ RETIRED | scope risk too high |
| **B. Defer runtime to v4.1; v4 ships Phase 3 prep (7 atoms)** | ✅ CHOSEN | Plan v3.2 § 2 |
| D. Permanently abandon | 🚫 VETOED by Codex | violates WP § 12 + § 17 |

**Status**: 4/7 Phase 3 prep atoms done (META_TX_SCHEMA + AmendmentFlow + MetaTransitionInterface + V4_1_METATAPE_PLAN). 3 deferred (MetaProposalDraft CAS / meta_validator / its conformance test).

#### 2026-04-26 D5 — RSP depth

| Option | Fate |
|---|---|
| **A. Full RSP (9 modules + 12 invariants + 3-layer rewards)** | ✅ CHOSEN |
| B. MVP (drop royalty + Challenger) | ⛔ RETIRED |
| C. min (immediate-only) | ⛔ RETIRED |

**Reason**: Inv 5/7/8 are interdependent; partial coverage violates them by construction.

#### 2026-04-26 D6 — External audit cadence

| Option | Fate |
|---|---|
| **A. Full (per phase + per STEP_B atom)** | ✅ CHOSEN |
| B. Reduced | ⛔ RETIRED |

**Reason**: trust restoration via redundancy.

---

### 2.2 D-VETO-1 to 7 (design-level vetos resolved)

#### D-VETO-1 — bus.rs / kernel.rs split protocol

| Option | Fate |
|---|---|
| A. Single 5-way / 3-way parallel A/B rewrite | ⛔ RETIRED (Codex CHALLENGE) |
| B. Staged shim refactor only | ⛔ RETIRED (insufficient) |
| **D. Spec-first + binding form (typed schemas + 22 invariants → 27 in v1.4) + STEP_B against spec** | ✅ CHOSEN |

**Status**: spec v1.4 frozen; CO1.1.4 + CO1.1.5 NO-GO until spec round-4 audit PASS.

#### D-VETO-2 — Money type

| Option | Fate |
|---|---|
| **A. i64 micro-coin (10⁻⁶)** | ✅ CHOSEN; implemented (Wave 1) |
| B. Decimal (rust_decimal) | ⛔ RETIRED (over-engineered for closed system) |
| C. BigRational | ⛔ RETIRED (heavy dep) |

**Status**: CO1.0a `src/economy/money.rs` ✅ DONE; 16 tests PASS.

#### D-VETO-3 — Genesis schema

| Option | Fate |
|---|---|
| A. Full WP § 5.L0 verbose schema | ⛔ RETIRED |
| 5-line hyper-minimal | ⛔ RETIRED (Codex CHALLENGE: no content anchor) |
| **D. 8-field minimal-with-anchor** (constitution_hash + creator_signature + signed_at + schema_version + amendment_predicate_hash + 2 registry roots + boot_attestation_hash) | ✅ CHOSEN |

**Status**: CO1.0 ✅ DONE; `[constitution_root]` section in genesis_payload.toml; 8 boot tests PASS.

#### D-VETO-4 — Runtime MetaTape

(Equivalent to D4. See above.)

#### D-VETO-5 — TRACE_MATRIX_v3 expansion

| Option | Fate |
|---|---|
| **A. Full N/M/D classification (Normative / Motivational / Deferred)** | ✅ CHOSEN |

**Status**: CO0.8 ✅ DONE; `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` ~80 rows. ⏸ R-022 hook (per-pub-symbol enforcement) deferred to CO1.13.

#### D-VETO-6 — Failure on tape

| Option | Fate |
|---|---|
| A. Reading X (every reject = tape Node) | ⛔ RETIRED |
| **B. Reading Y (system-stamped retry summary on next accepted tx + TerminalSummaryTx for no-accept runs)** | ✅ CHOSEN |

**Status**: spec v1.4 § 3.6.5 implements Reading Y; Const Art 0.2 cosmetic edit AVAILABLE 2026-04-27 but ⏳ awaiting user enactment.

#### D-VETO-7 — V-01 `completion_tokens: 0` literal

| Option | Fate |
|---|---|
| **A. Pre-split single ceremonial commit (CO1.1.4-pre1)** | ✅ CHOSEN |

**Status**: ✅ DONE 2026-04-27 Wave 5-C ceremonial commit. `bus.rs:268` literal `0` replaced by `pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0` (named symbol with FC1-Cost / FC3-Cost TRACE doc-comment + STEP_B rename target). Canonical fixture corpus deferred to CO1.1.4-pre1.b (Wave 6).

#### B-1 — TR mutation governance

| Option | Fate |
|---|---|
| A. Verbal "I approve" | ⛔ RETIRED |
| **PGP/SSH-signed git tag** | ✅ CHOSEN; 12+ signed tags chained |

---

### 2.3 Spec gap defaults (4 items; user can override per TaskMarket where allowed)

| Gap | Default chosen | User-overridable | Other options retired |
|---|---|---|---|
| 11.1 false_challenge penalty | **0** | ❌ NO (v4 hardcoded) | -3, -10 ⛔ |
| 11.2 verifier_bond on slash | **ReturnToVerifier** | ✅ per TaskMarket | SlashedToChallenger ⛔ default |
| 11.3 max_reuse_royalty_fraction | **0.10** | ✅ per TaskMarket | 0.20, 0.50 ⛔ default |
| 11.4 verifier_quorum | **1** | ✅ per TaskMarket | 2, majority ⛔ default |

**Note**: "RETIRED default" means these aren't the v4 defaults; they remain valid choices for individual TaskMarkets if user wants to opt in.

---

### 2.4 Path A choices (post-CO P0 closeout)

| Fork | Date | Options | Choice |
|---|---|---|---|
| Path A vs alternatives | 2026-04-27 | A=audit+code parallel / A.fast=skip more audits + code / A.pause | **A.fast** |
| Wave 1 sub-choice | 2026-04-27 | 1=CO1.3.2 evaluator / 2=audit only / 3=CO1.0a only / **hybrid (1.0a + 1.0 + walkthrough + bg audit)** | **hybrid** |
| Wave 2 sub-choice | 2026-04-27 | A=audit / **B=CO1.5 + CO1.6 + bg v1.4 patches** / C=暂停 | **B** |
| Wave 3 sub-choice | 2026-04-27 | A=audit / **B=CO1.4 CAS** / C=INV8 spike / D=evaluator / E=暂停 | **B** |
| Wave 4 | 2026-04-27 | A=spec round-4 audit / B=CO1.7.0a-f keypair (Codex impl) / C=CO1.2 Q_t (Claude impl) / D=INV8 / E=evaluator / F=ceremonies / G=暂停 | **A+C+B 三轨并行** (D/E deferred Wave 5; F user-async; G not-chosen) |
| Wave 5 (next) | TBD | A=D INV8 spike / B=CO1.7 transition_ledger / C=CO1.1.4-pre1 (V-01 kill) / D=CO1.1.4 bus split / E=CO1.1.5 kernel split / F=ceremonies | ⏳ user picks |

---

## § 3 Plan v3.2-fix3 Atom Status (~175 atoms)

### 3.1 ✅ DONE (atoms with passing real-code data)

| Phase | Atom | Files | Tests |
|---|---|---|---|
| CO P0 | 0.1-0.7 (foundation; ratified WPs, plans, audits) | 14 doc files | n/a |
| CO P0 | 0.7' TR governance hook | scripts/check_tr_ratification_chain.sh | self-tested |
| CO P0 | 0.8 TRACE_MATRIX_v3 N/M/D | handover/alignment/TRACE_MATRIX_v3_*.md | doc only |
| CO P0 | 0.9 META_TX_SCHEMA | handover/specs/META_TX_SCHEMA_v1_*.md | doc only |
| CO P1 | **1.0 minimal-with-anchor genesis** | genesis_payload.toml + boot.rs | 8/8 ✅ |
| CO P1 | **1.0a i64 MicroCoin** | src/economy/money.rs | 16/16 ✅ |
| CO P1 | **1.3.1 git substrate spike** | spike/gix_capability/ | 8/8 ✅ |
| CO P1 | **1.4 CAS layer (git2-rs)** | src/bottom_white/cas/ | 16/16 ✅ |
| CO P1 | **1.5 PredicateRegistry** | src/top_white/predicates/ | 14/14 ✅ |
| CO P1 | **1.6 ToolRegistry** | src/bottom_white/tools/ | 7/7 ✅ |
| CO P1 | **1.2 Q_t struct** (Wave 4-C; Claude impl + Codex audit CHALLENGE→fix) | src/state/{mod,q_state}.rs | 27 (5 inline + 22 conformance) ✅ |
| CO P1 | **1.7.0a-f system_keypair** (Wave 4-B; Codex impl + Claude auditor PASS) | src/bottom_white/ledger/system_keypair.rs | 7 (5 conformance + 2 inline) ✅ |
| CO P1 | **1.SPEC.0 spec v1.4 round-4 dual external audit** (Wave 4-A) | handover/audits/{CODEX,GEMINI}_SPEC_V14_ROUND4_*.md | PASS/PASS ✅ |
| CO P1 | 1.SPEC.0.4 TLA+ skeleton (optional) | handover/specs/STATE_TRANSITION_SPEC_TLA_*.tla | n/a (not run) |
| CO P3-prep | .1 META_TX_SCHEMA | (covered by CO0.9) | n/a |
| CO P3-prep | .4 AmendmentFlow format | handover/specs/AMENDMENT_FLOW_FORMAT_v1_*.md | doc |
| CO P3-prep | .5 MetaTransitionInterface trait | handover/specs/META_TRANSITION_INTERFACE_v1_*.md | doc |
| CO P3-prep | .6 V4_1_METATAPE_PLAN | handover/architect-insights/V4_1_METATAPE_PLAN_v1_*.md | doc |
| Walk-through | Inv 3 e2e | tests/walkthrough_inv3_conservation.rs | 3/3 ✅ |
| Spec | v1.4 with 4 patches | handover/specs/STATE_TRANSITION_SPEC_v1_*.md | n/a |

**Subtotal**: ~33 atoms done; 102 new tests since CO P0 (37 since Wave 3 closeout); 0 failures.

### 3.2 🔄 IN-FLIGHT / 🟢 unblocked but not started

| Atom | Status | Why available now |
|---|---|---|
| **CO1.1.1 skeleton dirs** | partial done (top_white, bottom_white, economy, state, ledger created); 🟢 finish later | independent of restricted files |
| **CO1.1.3 sandbox move** | 🟢 ready | not STEP_B-restricted |
| **CO1.1.4-pre1 V-01 kill** (1-line fix to bus.rs:268) | 🟢 ready (spec round-4 PASS unblocked) | small surgical fix; symbolic |
| **CO1.1.4 bus.rs split (STEP_B)** | 🟢 **NEWLY UNBLOCKED** (spec round-4 PASS/PASS) | restricted file; spec gate cleared |
| **CO1.1.5 kernel.rs split (STEP_B)** | 🟢 **NEWLY UNBLOCKED** | restricted file; spec gate cleared |
| **CO1.3.2 evaluator runtime_repo** | 🟢 ready (touches evaluator.rs but not STEP_B) | git2-rs validated |
| **CO1.7 transition_ledger** | 🟢 **NEWLY UNBLOCKED** | needs CO1.1.4+1.1.5 + system_keypair (now done) |
| **CO1.7.5 step_transition fn** | 🟢 ready post-1.7 | depends on 1.7 schema |
| **CO1.8 Materialized State (L5)** | 🟢 ready | new module |
| **CO1.9 Signal Indices (L6)** | 🟢 ready | new module |
| **CO1.10 Signal dichotomy** | 🟢 ready | new module |
| **CO1.11 Safety vs Creation** | 🟢 ready | uses PredicateRegistry |
| **CO1.13 TRACE_MATRIX impl + R-022 hook** | 🟢 ready | tooling layer |
| **CO P2.4.0 INV8 DAG spike** | 🟢 ready | independent spike |
| **CO P3-prep.2 MetaProposalDraft CAS** | 🟢 ready (CAS done) | uses CO1.4 |

**Subtotal**: ~14 atoms can start; 1 still blocked (CO1.1.2 wal/ledger move — STEP_B file).

### 3.3 ⏸ BLOCKED (remaining)

| Atom | Why blocked | Unblocks |
|---|---|---|
| **CO1.1.2 wal/ledger move (STEP_B)** | restricted file move; needs STEP_B parallel-branch ceremony | user runs STEP_B_PROTOCOL on wal.rs + ledger.rs |
| **CO1.14 P1 exit dual audit** | depends on all P1 atoms | all P1 done |

**Subtotal**: 2 atoms blocked. Spec round-4 dual audit (Wave 4-A) PASSed → CO1.1.4 / 1.1.5 / 1.7 / 1.7.5 are now in § 3.2 (newly unblocked).

### 3.4 ⏳ Pending CO P2 (after CO P1 exit)

| Phase | Atoms | Notes |
|---|---|---|
| CO P2.0 Inv 4 precondition | 1 | guarded |
| CO P2.1 TaskMarket | 4 | uses CO1.5 visibility |
| CO P2.2 EscrowVault | 3 | uses MicroCoin |
| CO P2.3 ContributionLedger | 5-6 | uses TransitionTx |
| CO P2.4.0 INV8 DAG spike (BLOCKING for 2.4.1+) | 1 | independent |
| CO P2.4 AttributionEngine | 5-7 | uses 2.4.0 |
| CO P2.5 ChallengeCourt | 4-5 | uses spec § 3.2 |
| CO P2.6 SettlementEngine | 4-5 | final formula |
| CO P2.7 Agent roles (5+1) | 5-6 | dispatch |
| CO P2.8 CTF stake symmetry | 3 | tests |
| CO P2.9 ReputationIndex | 2-3 | non-transferable |
| CO P2.10 E-01..E-04 closures | 4 | env-var rename |
| CO P2.11 RSP MVP-1 deployment | 2 | smoke |
| CO P2.12 P2 exit | 1 | dual audit |

**Subtotal**: ~50 atoms pending; ETA ~6-8 weeks after CO P1 exit.

### 3.5 ⏳ Pending CO P3-PREP (interleaved with P1+P2)

| Atom | Status |
|---|---|
| .2 MetaProposalDraft CAS storage | 🟢 ready (CAS done) |
| .3 meta_validator library | ⏸ blocks on CO P2.6 settlement_engine |
| .7 meta_validator conformance test | ⏸ blocks on .3 |

**Subtotal**: 3 atoms.

### 3.6 ⏸ Deferred to v4.1+ (NOT in current scope)

| Item | Why deferred |
|---|---|
| Runtime ArchitectAI / JudgeAI | D4=B ratified; runtime is v4.1 sprint |
| MetaTape full L4 acceptance | v4.1 |
| Phase 4 Permissioned chaincode | v4.x or v5 (per WP § 17) |
| Phase 5 Public AGI Market | v4.x or v5 |
| ZK / Validity Proof predicates | v4.x or v5 |
| Oracle integration | v4.x |
| Hardware attestation (TPM/SGX) | v5 |
| Multi-signer keyring (governance scaling) | v5 |
| Post-quantum signatures | when std stabilizes |

---

## § 4 User-Pending Items (⏳ requires you to act)

### 4.1 Constitutional ceremonies (all UNFROZEN; ENACTMENT_PROCEDURE has cp commands)

| Ceremony | What it does | Recommended order |
|---|---|---|
| **B'' Boot block reconciliation** | Repairs Const Art IV vs WP § 11 vs GENESIS spec drift (Gemini Top-3 fix #1) | 1 |
| **B' Art 0.2 line 64 cosmetic edit** | Aligns constitutional text with already-implemented Reading Y | 2 |
| **B Art 0.5 enactment** | Inserts pointer + 6 公理 to constitution.md | 3 |
| **C PREREG_AMENDMENT_v2 enactment** | Locks PPUT-CCL arc state (D1=C MVP-pivot) | independent (any time) |

**Each ceremony**: cp workflow → recompute SHA → signed git tag. Total ~5-15 min each.

### 4.2 Decision points pending user input

| # | Question | My recommendation |
|---|---|---|
| Q1 | Run spec v1.4 round-4 dual re-audit now? | YES (≤$10; predicted ≤1 PARTIAL = Q5 deferred) |
| Q2 | After audit PASS, GO CO P1 launch (CO1.1.4 + 1.1.5 + 1.7)? | YES if PASS |
| Q3 | Wave 4 next atom? | CO1.7.0a-f system_keypair (independent; unblocks CO1.7) OR CO P2.4.0 INV8 spike (independent) |
| Q4 | Install R-022 / R-023 pre-commit hooks? | At CO P1 launch time |

---

## § 5 Risks Registered (not yet resolved)

| # | Risk | Probability | Mitigation status |
|---|---|---|---|
| R1 | Spec v1.4 round-4 still CHALLENGE | medium | trajectory: 14→13→5→predicted 1 |
| R2 | bus/kernel split harder than scoped | high | spec-first + STEP_B mitigates; budget 1.5 wk each |
| R3 | INV8 DAG non-deterministic in practice | medium | spike pre-draft has 7 hostile cases |
| R4 | git2-rs same-repo concurrent commit unproven | low | spike tested disjoint; same-repo deferred to CO1.7 |
| R5 | Constitutional amendments forever pending | high | user-driven; ENACTMENT doc ready |
| R6 | Atom dependency cycles | low | sprint dep graph drawn |
| R7 | Budget overrun beyond $890 | low | currently 3-6% spent |
| R8 | User attention drift | medium | data-driven; revertible per atom |

---

## § 6 Permanently Retired Options (will NOT happen)

These are decisions that closed forever. Listed here so future readers don't re-litigate.

```
D1=A PREREG PAUSE                              ⛔ ratified C MVP-pivot
D1=B PREREG NEGATIVE                           ⛔ ratified C
D2=A constitution embeds full WP text          ⛔ ratified B pointer
D3=B delete TFR v1                             ⛔ ratified A deprecate
D4=A v4 runtime MetaTape                       ⛔ ratified B v4.1 defer
D4=D permanently abandon MetaTape              🚫 Codex VETO (WP § 12+§17)
D5=B/C RSP MVP/min                             ⛔ ratified A full
D6=B reduced audit cadence                     ⛔ ratified A full
D-VETO-1 A 5-way parallel rewrite              ⛔ ratified D spec-first
D-VETO-1 B staged shim only                    ⛔ ratified D
D-VETO-2 B Decimal                             ⛔ ratified A i64
D-VETO-2 C BigRational                         ⛔ ratified A
D-VETO-3 A verbose                             ⛔ ratified D 8-field-anchor
D-VETO-3 5-line hyper-minimal                  ⛔ Codex CHALLENGE
D-VETO-6 A Reading X literal                   ⛔ ratified B Reading Y
Spec gap 11.1 nonzero default penalty          ⛔ v4 fixed=0
gix as substrate library                       ⛔ pivoted to git2-rs (spike data)
```

---

## § 7 Forward Roadmap

### 7.1 Critical path (must happen in order)

```
[NOW]
  └─ 用户决定: 开 spec round-4 audit OR 直接 wave 4 green-field
       │
       ▼
  ┌─ Spec round-4 audit (~$10, 30 min)
  │     │
  │     PASS → CO1.1.4-pre1 + CO1.1.4 + CO1.1.5 + CO1.7 unblocked
  │     │
  │     ▼
  │   CO1.7.5 step_transition fn (CORE)
  │     │
  │     ▼
  │   CO1.8 + 1.9 + 1.10 + 1.11 (parallel)
  │     │
  │     ▼
  │   CO1.13 TRACE_MATRIX impl + R-022 hook
  │     │
  │     ▼
  │   CO1.14 P1 exit audit
  │
  └─ 同时 (parallel green-field):
       CO1.7.0a-f system_keypair
       CO1.2 Q_t struct
       CO P2.4.0 INV8 DAG spike
       CO P3-prep.2 MetaProposalDraft CAS

[CO P1 EXIT]
  ▼
[CO P2 6-8 wk]
  CO P2.0a / 2.0 / 2.1-2.11 / 2.12

[v4 SHIP]
  ▼
[v4.1 4-6 wk] MetaTape runtime
```

### 7.2 What I will / will not do without explicit user input

**I WILL** (auto-research mode default):
- Continue green-field code work that doesn't touch restricted files
- Run audit cycles in background when budget permits
- Track every decision in this doc + AUDIT_LEDGER
- Commit + sign every reasonable batch
- Surface any new VETO immediately

**I WILL NOT** (without explicit user GO):
- Touch bus.rs / kernel.rs / wal.rs / ledger.rs (NO-GO until spec PASS)
- Modify constitution.md (frozen; only cp workflow)
- Enact constitutional amendments (B / B' / B'' / C ceremonies)
- Spend > $50 on a single audit invocation
- Skip any audit verdict (always honor VETO/CHALLENGE)
- Make assumptions about user intent on D-decisions

### 7.3 Wave 4 candidates (user picks; nothing forgotten)

| Option | Atom | Why |
|---|---|---|
| A | Spec round-4 dual re-audit | Closes audit chain; unblocks CO1.1.4/1.1.5/1.7 |
| B | CO1.7.0a-f system_keypair | Independent; ed25519 + Argon2id + ChaCha20 per spec |
| C | CO1.2 Q_t struct (9 fields) | Composes existing types; foundation for transition fn |
| D | CO P2.4.0 INV8 DAG spike | Independent; closes the hardest math problem |
| E | CO1.3.2 evaluator runtime_repo | Touches evaluator.rs (high-traffic but not STEP_B) |
| F | Constitutional ceremonies B/B'/B''/C | User-led; cp workflow |
| G | Pause + you review | 0 risk; 0 burn |

---

## § 8 How This Doc Stays Updated

- **On every fork**: append a new row in § 2 with date + question + options + choice
- **On every atom completion**: move from § 3.2 → § 3.1 with file path + test count
- **On every audit**: log verdict + must-fixes
- **On every constitutional event**: log in § 4.1 with timestamp
- **Trust-Rooted**: this file's SHA is in `genesis_payload.toml [trust_root]`; tampering detected by boot
- **Read order on cold start**: read THIS doc + LATEST.md + last 3 commits' messages = full context

---

## § 9 One-Sentence Status (TL;DR)

> v4 has finished CO P0 + Wave 1-4 (gix→git2-rs spike + CO1.0 + 1.0a + 1.4 + 1.5 + 1.6 + walkthrough Inv 3 e2e + **CO1.2 Q_t + CO1.7.0a-f keypair + spec v1.4 round-4 PASS/PASS**); 244/0 lib+integration tests PASS; constitutional + WP authority chain ratified; spec round-4 PASS unblocks CO1.1.4 / 1.1.5 / 1.7 / 1.7.5 STEP_B atoms; ~14 green-field atoms unblocked; 4 constitutional ceremonies AVAILABLE pending user; budget at ~5-9% of $890 (Wave 4 audits ~$22-25).

---

## § 10 Anti-Forget Pledge

Every option offered to user in this session is recorded above with explicit fate. No silent retirements. No hidden defaults. If you ever wonder "what about option X I didn't pick?", check § 6 (permanently retired) or § 3.2-3.6 (deferred / pending). If still unclear, ask Claude directly — this doc is the authoritative answer.

— ArchitectAI, 2026-04-27 post-Wave-3
