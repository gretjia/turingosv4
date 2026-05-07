# TB-18C — HEAD_t C2 Multi-ref ChainTape (charter, 2026-05-07)

**Authority**: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
§3.1 (Stage A3 charter draft authorized; STEP_B execution requires per-atom architect
sign-off going forward).

**Companion architect alignment docs**:
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` §3 Stage A3
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §4 Stage A3

**Mode**: Constitutional Harness Engineering (per CLAUDE.md PRIME OPERATING MODE).

**Class**:
- Schema (`refs/chaintape/{l4,l4e,cas}` ref naming + Git2-backed multi-ref ledger writer) = **Class 4 STEP_B** on `src/bottom_white/ledger/transition_ledger.rs` + adjacent
- Replay reconstruction logic (HEAD_t reconstruct from refs) = Class 3
- Test gates = Class 1

**Phase**: P0 Constitution Landing closure (forward step from Constitution Landing First C1).

**Phase tag** (per `feedback_tb_phase_tag_required`):
- `phase_id` = P0 Constitution Landing — HEAD_t C2 production refs
- `roadmap_exit_criteria_addressed` = G-009 Path C hybrid §4.1 C2 production form ("libgit2-backed refs: refs/chaintape/l4 + refs/chaintape/l4e + refs/chaintape/cas"); upgrades C1 single-ref `refs/transitions/main` to multi-ref ChainTape
- `kill_criteria_tested` = (a) any hidden filesystem pointer reappearance; (b) replay HEAD_t cannot be reconstructed from refs alone; (c) accepted L4 transition does not advance L4 ref; (d) rejected L4.E evidence does not advance L4.E ref; (e) CAS write does not advance CAS root ref

---

## §1. Scope

TB-18C upgrades HEAD_t from C1 (single L4 ref via `Git2LedgerWriter` on
`refs/transitions/main`) to C2 (three coherent refs: `refs/chaintape/l4` +
`refs/chaintape/l4e` + `refs/chaintape/cas`).

C1 baseline (Constitution Landing First, commit `b7bde23`):
- L4 accepted entries written as real Git commits on `refs/transitions/main`.
- `advance_head_t()` captures real 40-hex commit OID and stores it in `q.head_t`.
- `head_t_witness.rs` provides 6-field witness (`state_root`, `l4_head`, `l4e_head`,
  `cas_root`, `economic_state_root`, `run_id`).
- L4.E and CAS roots are computed from in-memory state, not from named Git refs.

C2 target:
- L4 accepted entries on `refs/chaintape/l4` (rename of `refs/transitions/main`).
- L4.E rejected evidence on `refs/chaintape/l4e` (NEW ref).
- CAS roots on `refs/chaintape/cas` (NEW ref, advances on each CAS write batch).
- Replay reconstructs HEAD_t from refs alone without any in-memory pointer or
  filesystem-side global pointer.

TB-18C does NOT change typed_tx schema, sequencer admission semantics, or canonical
signing payload. TB-18C does NOT alter HeadTWitness public API. TB-18C is a ledger
storage form refactor preserving observable semantics.

## §2. Functional Requirements (FR)

| ID | Requirement |
|----|-------------|
| FR-18C.1 | `Git2LedgerWriter` (or successor) MUST manage three named refs: `refs/chaintape/l4`, `refs/chaintape/l4e`, `refs/chaintape/cas`. `refs/transitions/main` is migrated to `refs/chaintape/l4` (dual-write during migration window allowed; hard cutover on TB-18C ship). |
| FR-18C.2 | Accepted L4 transition MUST advance `refs/chaintape/l4`. Rejected L4.E evidence MUST advance `refs/chaintape/l4e`. CAS object write MUST update `refs/chaintape/cas` (CAS root commit per write batch; commit message references CAS object CIDs). |
| FR-18C.3 | `HeadTWitness::reconstruct_from_repo(repo: &git2::Repository)` MUST exist as a public constructor that reads the three refs and constructs the witness without requiring `&QState`. The existing `&QState`-based constructor is preserved as a derived view; the new constructor exists for replay-only paths. |
| FR-18C.4 | Replay MUST produce a HEAD_t identical (six-field byte equality) to the original run's HEAD_t. A fresh checkout (clean ChainTape repo + fresh CAS) replaying genesis + recorded events MUST end at the same OIDs on all three refs. |
| FR-18C.5 | NO filesystem-side global pointer (e.g., `LATEST_HEAD_T.txt` / `CURRENT_RUN.json` / similar). The three Git refs ARE the canonical pointer. |
| FR-18C.6 | Backward-compat: existing C1 evidence directories MUST remain replayable via a documented migration path (per `feedback_no_retroactive_evidence_rewrite` — old runs not rewritten; replay tooling adapts). |
| FR-18C.7 | `audit_tape` binary view-shares / view-pools / view-prices commands MUST resolve their data through ref-derived state, not through any in-memory shortcut. |
| FR-18C.8 | `cargo test --workspace` and `bash scripts/run_constitution_gates.sh` MUST be GREEN at TB-18C ship. No regression of the 97 existing constitution gates. |

## §3. Constitutional Requirements (CR)

| ID | Constraint |
|----|------------|
| CR-18C.1 | STEP_B parallel-branch protocol per CLAUDE.md §12 + STEP_B preflight per atom for any change to `src/bottom_white/ledger/transition_ledger.rs` / `src/bus.rs` / `src/state/sequencer.rs`. |
| CR-18C.2 | NO Class-4 typed-tx schema bump bundled in TB-18C. If C2 implementation surfaces a need, file OBS and escalate (per CR-C0.3 precedent). |
| CR-18C.3 | NO retroactive evidence rewrite. Pre-TB-18C runs replay via documented adapter, not via direct evidence editing. |
| CR-18C.4 | NO change to canonical signing payload. C2 changes ledger storage form, not signed contents. |
| CR-18C.5 | NO new global filesystem pointer. The three named Git refs are the pointer. |
| CR-18C.6 | NO change to HEAD_t six-field schema. C2 changes how the values are PERSISTED, not what they ARE. |
| CR-18C.7 | NO change to FC1 hard invariant `evaluator_reported_completed_llm_calls == l4_work_attempt_count + l4e_work_attempt_count + capsule_anchored_attempt_count`. The invariant is observable from refs the same way it was from in-memory state. |
| CR-18C.8 | Trust Root rehash for any STEP_B-restricted file change is mandatory per CLAUDE.md routine. |

## §4. Ship Gates (SG)

Each gate is binary pass/fail. All MUST be GREEN to declare TB-18C SHIPPED.

| ID | Gate | Source / Verification |
|----|------|-------------|
| SG-18C.1 (= alignment-doc SG-A3.1) | L4 head ref advances on accepted transition | `tests/constitution_head_t_c2_l4_ref_advances.rs` (NEW) |
| SG-18C.2 (= SG-A3.2) | L4.E head ref advances on rejected evidence | `tests/constitution_head_t_c2_l4e_ref_advances.rs` (NEW) |
| SG-18C.3 (= SG-A3.3) | CAS root ref advances when CAS evidence added | `tests/constitution_head_t_c2_cas_ref_advances.rs` (NEW) |
| SG-18C.4 (= SG-A3.4) | Replay reconstructs HEAD_t from refs (six-field byte equality between original and replayed) | `tests/constitution_head_t_c2_replay_byte_equality.rs` (NEW) |
| SG-18C.5 (= SG-A3.5) | No hidden filesystem pointer (grep + replay-without-fs-state) | `tests/constitution_head_t_c2_no_fs_pointer.rs` (NEW) |
| SG-18C.6 | `cargo test --workspace` GREEN; ≥1181 pass (no regression from `feec129`) | `cargo test --workspace` |
| SG-18C.7 | `bash scripts/run_constitution_gates.sh` GREEN; ≥97 PASS (no regression) | gate runner |
| SG-18C.8 | One real-LLM smoke run (≥1 problem) on TB-18C substrate produces a 50/50-style invariant report under refs storage | `handover/evidence/tb18c_smoke_*/` |
| SG-18C.9 | OBS forward-binding for any C1 → C2 migration edge case captured | `handover/alignment/OBS_TB18C_*.md` |
| SG-18C.10 | Codex + Gemini dual audit dispatched AFTER MVP gates green per CR-C0.8 | `handover/audits/G2_TB_18C_DUAL_AUDIT_DISPATCH_*.md` |

## §5. Atoms (sequence-binding)

| # | Atom | Class | STEP_B? | Surface |
|---|------|-------|---------|---------|
| R0 | Charter ratification (this document + Codex Q&A + dual audit ratification of charter) | 0 | No | This document; Codex G1-style charter audit |
| R1 | `refs/chaintape/{l4,l4e,cas}` ref naming + `Git2LedgerWriter` multi-ref writer + dual-write migration adapter | 4 STEP_B | YES | `src/bottom_white/ledger/transition_ledger.rs` |
| R2 | `HeadTWitness::reconstruct_from_repo` constructor + replay-from-refs path | 3 | No | `src/state/head_t_witness.rs` (additive constructor; existing one preserved) |
| R3 | CAS root ref advance hook on CAS write batch | 3 | No | `src/bottom_white/cas/store.rs` adapter |
| R4 | Test gates (SG-18C.1..5) | 1 | No | `tests/constitution_head_t_c2_*.rs` |
| R5 | Smoke run (1+ problems) on C2 substrate | 3 evidence | No | `handover/evidence/tb18c_smoke_*/` |
| R6 | OBS forward-binding for migration edges | 0 | No | `handover/alignment/OBS_TB18C_*.md` |
| R7 | G2 dual-audit dispatch | 3 audit | No | `handover/audits/G2_TB_18C_*.md` |

## §6. Forbidden list (explicit per architect alignment doc + CLAUDE.md §12)

```
- no f64 in money path (universal forbidden list per parent authorization §4)
- no ghost liquidity (n/a; not market-relevant)
- no price-as-truth (n/a; not market-relevant)
- no dashboard source-of-truth (must regenerate from refs+CAS)
- no real funds (n/a)
- no public chain (CR-18C.* — refs/chaintape/* is local libgit2 storage; NOT public chain)

Plus TB-18C-specific:
- no Class-4 typed-tx schema bump bundled (CR-18C.2)
- no canonical signing payload change (CR-18C.4)
- no new global filesystem pointer (CR-18C.5)
- no HEAD_t schema change (CR-18C.6)
- no FC1 hard invariant change (CR-18C.7)
- no retroactive evidence rewrite (CR-18C.3)
- no Sequencer public-API change beyond ratified ledger writer adapter
- no agent-submittable system tx introduction
- no MVP gate regression (97/0/1 baseline)
- no workspace-test regression (1181/0/151 baseline)
```

## §7. Pre-conditions

- TB-18R FINAL SHIPPED (✅ 2026-05-07 per `2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`).
- TB-C0 SHIPPED FINAL (✅ 2026-05-07).
- HEAD_t C1 GREEN at `feec129` (✅).
- Constitution gates 97/0/1 GREEN (✅).
- Workspace tests 1181/0/151 GREEN (✅).

## §8. §8 ship gates (architect)

TB-18C ships FINAL only after:
1. SG-18C.1..10 GREEN.
2. `cargo test --workspace` clean (≥1181 PASS).
3. `bash scripts/run_constitution_gates.sh` GREEN (≥97 PASS).
4. Codex G1 charter ratification CLOSED.
5. G2 dual audit dispatched AFTER substrate green; conservative ranking VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.
6. Explicit architect §8 sign-off at `handover/directives/YYYY-MM-DD_TB18C_§8_SIGN_OFF.md`.

## §9. Cross-references

- Architect alignment Stage A3: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md` §3 / §4
- Parent authorization: `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md`
- TB-18R FINAL ship (predecessor): `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md`
- TB-C0 charter (format precedent): `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- Constitution Landing First substrate (C1 baseline): commit `b7bde23` + `src/state/head_t_witness.rs`
- HEAD_t Path C decision: CLAUDE.md §4.1 + `constitution.md` Art. 0.4
- Constitution gap analysis (Art. 0.2 reference): `handover/alignment/CONSTITUTION_GAP_ANALYSIS_2026-05-07.md`
