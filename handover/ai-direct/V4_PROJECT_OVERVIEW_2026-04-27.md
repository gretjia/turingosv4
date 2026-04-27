# TuringOS v4 — Project Overview & Onboarding Guide (2026-04-27)

> **Purpose**: Single-page onboarding for any new session / new reader to grok v4 state. Read this FIRST. Branches into specific docs as needed.
> **Authority**: Reflects state at HEAD = `49981a3` + auto-research wave 5 additions; consults all ratified specs.
> **Audience**: future Claude session, new collaborator, returning user after context lapse.

---

## § 1 What is TuringOS v4

**One sentence**: A Rust microkernel that runs swarms of LLM agents to formally verify mathematical theorems (MiniF2F Lean 4), with every agent action accountable on a canonical tape.

**Three-layer mental model** (反奥利奥 Anti-Oreo):
- ⚪ **Top White**: predicates / signals / budgets (decide accept/reject)
- ⚫ **Middle Black**: agents (LLM reasoning; opaque)
- ⚪ **Bottom White**: tape / CAS / ledger / sandbox (deterministic substrate)

**Why "operating system, not framework"**: every agent action is a state transition logged to ChainTape (6 layers L0-L6); replay reconstructs entire state byte-identical. Inspired by Turing 1948 + Bitcoin / git.

**Single user / solo researcher**: gretjia. Zero programming background. Chinese primary. ArchitectAI (Claude) + Codex + Gemini are the development team per `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md`.

---

## § 2 Where We Are (HEAD = 49981a3, ratified)

**Phase**: CO P0 (foundation) effectively complete; **CO P1 (GitTape + Anti-Oreo + Predicate/Tool Registries) is next**.

**Substrate decision**: Path B (real git via `gix` substrate, fallback `git2-rs`).

**Last 50+ commits** were all doc/governance work; **0 src/ changes since C2 batch was killed at `56875c1`**. The kernel itself awaits CO P1 refactor.

**Current authority docs** (in priority order):
1. `constitution.md` — Art 0–0.4 + Art I-V (ratified; FROZEN until white paper finalized)
2. `handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md` (architecture)
3. `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md` (economic)
4. `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` (file-level v4 spec)
5. `handover/architect-insights/CO_MEGA_PLAN_v3.2_2026-04-27.md` (170 atoms / 22-27wk / $580-1200)

**Frozen items** (per user 2026-04-27 directive 「白皮书未定稿，现阶段不作任何宪法修订」):
- Constitution Art 0.5 enactment (DRAFT remains)
- Art 0.2 line 64 cosmetic edit (proposal remains)
- ANY constitution.md modification

**NOT frozen**: spec docs, plans, audits, white paper revisions (user-driven), Trust Root governance, PREREG amendments, code work.

---

## § 3 Critical Doc Tree (start here)

```
handover/
├── ai-direct/
│   ├── LATEST.md                           # day-to-day state; updated each session
│   ├── V4_PROJECT_OVERVIEW_2026-04-27.md   # 👈 THIS FILE
│   └── HANDOVER_PHASE_*.md                 # historical phase exits
│
├── whitepapers/                             # USER-AUTHORED authority (highest level)
│   ├── TURINGOS_WHITEPAPER_v1_2026-04-26.md            # architecture chapter (21 §)
│   ├── TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md   # economic chapter (12 inv + 9 modules)
│   └── TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md       # synthesis: file-by-file spec
│
├── architect-insights/                      # plan-level docs
│   ├── CO_MEGA_PLAN_v3.2_2026-04-27.md      # 170-atom plan (after 2 audit rounds)
│   ├── CO_MEGA_PLAN_v3.1_2026-04-26.md      # predecessor (still readable for atom IDs)
│   ├── CO_P0_AMENDMENT_v1_2026-04-26.md     # D1-D6 PROVISIONAL→ratified
│   ├── RATIFICATION_2026-04-27.md           # what user signed
│   ├── ENACTMENT_PROCEDURE_2026-04-27.md    # how to enact pending ceremonies
│   ├── SPRINT_DEPENDENCY_GRAPH_v1_2026-04-27.md  # blocks/blockedBy + critical path
│   ├── TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md  # Codex+Gemini co-execution
│   ├── V4_1_METATAPE_PLAN_v1_2026-04-27.md  # v4.1 future implementation
│   ├── AMENDMENT_2026-04-26_art-0-turing-fundamentalism.md  # historical (legacy format)
│   ├── CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md            # 🧊 FROZEN draft
│   └── TFR_MASTER_PLAN_2026-04-26.md         # ⚠️ LEGACY (deprecated)
│
├── specs/                                    # binding spec docs (pre-CO-P1)
│   ├── STATE_TRANSITION_SPEC_v1_2026-04-27.md            # binding form for D-VETO-1
│   ├── STATE_TRANSITION_SPEC_TLA_2026-04-27.tla          # optional TLA+ skeleton
│   ├── GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md      # 8-field minimal genesis
│   ├── ART_0_2_REINTERPRETATION_2026-04-27.md            # 🧊 FROZEN; Reading Y in spec
│   ├── SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md          # ed25519 + Argon2id + ChaCha20
│   ├── META_TX_SCHEMA_v1_2026-04-27.md                   # CO P3-prep typed schema
│   ├── META_TRANSITION_INTERFACE_v1_2026-04-27.md        # v4.1 trait spec
│   ├── AMENDMENT_FLOW_FORMAT_v1_2026-04-27.md            # Art V.3 amendment structured format
│   ├── INV8_DAG_DETERMINISM_SPEC_v1_2026-04-27.md        # CO P2.4.0 spike pre-draft
│   ├── CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md      # gix substrate spike plan
│   ├── PRE_COMMIT_HOOKS_R022_R023_v1_2026-04-27.md       # opt-in hooks
│   └── SPEC_WALKTHROUGH_v1_2026-04-27.md                 # end-to-end RSP scenario
│
├── alignment/
│   └── TRACE_MATRIX_v3_2026-04-27.md         # ~80 rows; bidirectional traceability
│
├── audits/                                   # external audit reports + ledger
│   ├── AUDIT_LEDGER.md                       # running spend + verdicts
│   ├── CODEX_CO_P0_AUDIT_2026-04-26.md       # CO P0 audit (Plan VETO + Amendment VETO)
│   ├── CODEX_T_S_REVIEW_2026-04-27.md        # T+S re-review (D-VETO-4 reverted to defer)
│   ├── GEMINI_CO_P0_AUDIT_2026-04-26.md      # CO P0 audit (Plan CHALLENGE)
│   ├── GEMINI_V32_REVIEW_2026-04-27.md       # v3.2 cross-review (Plan VETO on keypair)
│   └── run_*.{sh,py}                         # audit runner scripts
│
├── preregistration/
│   ├── PREREG_AMENDMENT_v2_2026-04-26.md     # research-arc spec (sanity-check reframe)
│   ├── PPUT_CCL_HARD10_*.json                # sealed problem set
│   └── ...
│
└── audits/run_gemini_*.py                    # external audit scripts
scripts/
└── check_tr_ratification_chain.sh            # B-1 governance hook
tests/
├── conformance_stubs.rs                       # 80+ ignored stubs (CO P1 prep)
└── ...
```

---

## § 4 Core Concepts in 6 Bullets

1. **反奥利奥三层** — every code symbol classifiable to ONE of top_white/middle_black/bottom_white. Mixed-layer = constitutional violation. Fourth root `economy/` is structurally Top White but its own dir for atom hygiene.

2. **Q_t (9 fields)** — system state: q_t (agent swarm) + head_t (git commit SHA) + state_root_t + tape_view_t + ledger_root_t + predicate_registry_root_t + tool_registry_root_t + economic_state_t (9 sub-fields) + budget_state_t. ALL reconstructible from tape replay.

3. **ChainTape (6 layers L0-L6)** — L0 Constitution Root / L1 Predicate Registry / L2 Tool Registry / L3 CAS / L4 Transition Ledger / L5 Materialized State + Agent View / L6 Signal Indices.

4. **22 transition invariants** (per STATE_TRANSITION_SPEC v1.1) + **12 economic invariants** (per WP § 18) — every one has a conformance test path; v4 ship gate = all 100% pass.

5. **Tri-model co-execution** — Claude (orchestrator), Codex (heavy implementer for STEP_B), Gemini (strategic reviewer). Hard rule: Generator ≠ Evaluator at every atom. VETO > CHALLENGE > PASS conservative-wins.

6. **Spec-first** — STATE_TRANSITION_SPEC v1.1 is the binding contract for bus.rs/kernel.rs split; STEP_B branches A+B compared against spec, not against each other.

---

## § 5 Last Decisions (2026-04-27)

User-confirmed via SSH-signed git tag `v4-ratify-2026-04-27-49981a3`:

| Decision | Choice |
|---|---|
| D1 PREREG | C — MVP-pivot (sanity check, deferred until L6 schema settled) |
| D2 Constitution Art 0.5 | B — pointer + 6 axioms (FROZEN until WP final) |
| D3 TFR v1 | A — deprecate but preserve |
| D4 MetaTape | B — defer runtime to v4.1; v4 ships Phase 3 prep (7 atoms) |
| D5 RSP | A — full RSP (12 invariants interdependent) |
| D6 audit cadence | A — full per-phase + per-STEP_B-atom |
| D-VETO-1 (bus/kernel split) | D — spec-first + STEP_B against spec |
| D-VETO-2 (money type) | A — i64 micro-coin (10⁻⁶ unit) |
| D-VETO-3 (genesis) | D — 8-field minimal-with-anchor |
| D-VETO-4 (MetaTape) | B — defer (NOT permanent abandon) |
| D-VETO-5 (TRACE_MATRIX) | A — full N/M/D classification |
| D-VETO-6 (rejection) | B + system-stamped retry metadata |
| D-VETO-7 (V-01 completion_tokens) | A — pre-split atom CO1.1.4-pre1 |
| B-1 (TR mutation) | SSH-signed git tag (PASS) |

Spec gap defaults (auto-research wave 4; user-overridable):
- 11.1 false-challenge reputation penalty = 0
- 11.2 verifier_bond on slashed claim = ReturnToVerifier
- 11.3 max reuse royalty fraction = 0.10
- 11.4 verifier quorum default = 1

---

## § 6 What's Pending (As of 2026-04-27)

### Pending user action (ceremonies / reviews):
- 🧊 **Constitution Art 0.5 enactment** — FROZEN until WP finalized
- 🧊 **Art 0.2 line 64 cosmetic edit** — FROZEN until WP finalized
- ✅ **PREREG_AMENDMENT_v2 enactment** — available (Ceremony C in `ENACTMENT_PROCEDURE`)
- 🆕 **White paper finalization** — user-driven; no specific timeline; triggers unfreezes

### Pending CO P1 entry (auto-research / Codex/Gemini work):
- ⏳ **CO1.SPEC.0 final dual sign-off on STATE_TRANSITION_SPEC v1.1** — Codex T+S done; Gemini v3.2 done; v1.1 patches applied; need confirming review
- ⏳ **CO1.3.1 gix spike** — pre-flight doc done; spike runs against actual code (5-day time-box)
- ⏳ **CO0.7' R-022/R-023 hook installation** — opt-in; deferred to CO P1 launch

### Auto-research output (waves 1-5; complete):
- 14 doc files + ~80 conformance test stubs + governance hook script
- TR manifest 58 → 70+
- 0 src/ changes (unbroken since C2 kill)
- Ratified via 1 SSH tag

---

## § 7 What CO P1 Looks Like (preview from Plan v3.2)

Critical-path atoms to first 6 weeks of CO P1:

```
Week 1: CO1.SPEC.0 final sign-off (Codex+Gemini PASS/PASS) → freeze v1.1
Week 1-2: CO1.3.1 gix substrate spike (5-day time-box)
Week 2: CO1.1.1 skeleton dirs + CO1.0 genesis (parallel)
Week 3: CO1.1.4-pre1 (kill completion_tokens=0) + CO1.5+1.6+1.4 in parallel
Week 4-5: CO1.1.4 bus.rs split (STEP_B)
Week 5-6: CO1.1.5 kernel.rs split (STEP_B)
Week 6: CO1.2 Q_t struct + CO1.7 transition ledger landing
```

After CO P1: **CO P2 RSP economy** (~6-7 wk) — task market + escrow + contribution DAG + challenge court + settlement engine.

After CO P2: v4 ship; CO P3-PREP artifacts complete (no MetaTape runtime — deferred to v4.1).

---

## § 8 Cost / Budget State

**Total budget**: $580-1200 (mid $890)
**Spent to date** (auto-research wave 1-5): ~$10.75-20.75 (1.2-2.3%)

**Burn rate gates**:
- 80% threshold ($712): escalate to user; consider scope reduction
- 100% threshold ($890): hard pause; user sudo required

**Per-atom estimates** in `TRI_MODEL_ORCHESTRATION_PROTOCOL § 5`:
- Standard atom Codex review: $2-5
- STEP_B atom Codex implement+review: $5-10
- Gemini per-atom heavy review: $1-2
- Phase exit dual audit: $25-40 each

---

## § 9 If You Are Continuing This Session

1. Read this file (DONE)
2. Read `LATEST.md` for tactical state
3. Skim `handover/audits/AUDIT_LEDGER.md` for audit history
4. Check Plan v3.2 § 5 critical path
5. Check user's last message for current priority
6. Run `bash scripts/check_tr_ratification_chain.sh` to verify governance chain intact
7. Run `cargo test --lib boot` to verify TR + boot still PASS

If user says "继续 auto research" → wave 5 has completed all doc-only work that doesn't violate the 2026-04-27 freeze. Remaining work either (a) requires unfreezing constitution amendments OR (b) requires CO P1 launch (src/ work). Surface this to user and ask for direction.

If user requests CO P1 launch:
1. First confirm Codex+Gemini final sign-off on STATE_TRANSITION_SPEC v1.1 (`tests/state_transition_spec_v1_1_dual_audit_passed`)
2. Then run CO1.3.1 gix spike per pre-flight
3. Document spike result; await user GO before CO1.0+ atoms

---

## § 10 Honest Acknowledgements

What this overview achieves:
- Single-page entry point for any cold-start session
- Maps current state to all relevant docs
- Lists pending items + frozen items distinctly
- Provides session-start checklist

What this overview is honest about:
- Doc tree is current as of HEAD `49981a3`; new commits will add more
- Pending items list will shift as user acts
- White paper finalization timeline is unknown

What this overview does NOT do:
- Replace any individual doc (it's an index)
- Make any decisions (decisions live in ratification + plan v3.2)
- Track per-atom progress (separate task list / TODO; AUDIT_LEDGER for spend)

— ArchitectAI, 2026-04-27 (auto-research wave 5)
