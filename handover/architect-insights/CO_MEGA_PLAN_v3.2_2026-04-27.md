# Constitutional Operationalization MEGA PLAN v3.2 — Codex T+S Re-Review Patch

> **Date**: 2026-04-27
> **Supersedes**: `CO_MEGA_PLAN_v3.1_2026-04-26.md` (still readable for atom continuity; v3.2 is a patch overlay, not a full rewrite)
> **Triggered by**: Codex CO P0.7 audit (`CODEX_CO_P0_AUDIT_2026-04-26.md`) + Codex T+S re-review (`CODEX_T_S_REVIEW_2026-04-27.md`)
> **Status**: ArchitectAI v3.2 patch; awaiting Gemini cross-review on the new spec docs.

---

## § 1 What v3.2 Changes from v3.1

Six surgical changes per Codex feedback (NO full rewrite — too disruptive):

| # | Change | Source | Affected atoms |
|---|---|---|---|
| 1 | **CO1.SPEC.0** new gate atom — typed state-transition spec must PASS dual audit before any bus/kernel split | Codex T+S §A D-VETO-1 CHALLENGE | inserted before CO1.1.4 + CO1.1.5 |
| 2 | **CO1.0 schema upgrade** — minimal-with-anchor genesis (not 5-line, not full WP rich); content-hash anchored | Codex T+S §A D-VETO-3 CHALLENGE | CO1.0.1, CO1.0.2 expanded to 4 atoms |
| 3 | **D4 wording correction** — defer runtime MetaTape to v4.1 (not "permanently abandon"); v4 produces Phase 3 prep artifacts | Codex T+S §A D-VETO-4 VETO on permanent abandonment | D4 reverts to B with explicit "preserve v4.1+ path" |
| 4 | **CO1.7+CO1.9 retry metadata** — system-signed `RejectedAttemptSummary` + `TerminalSummaryTx`; NOT agent self-report | Codex T+S §A D-VETO-6 CHALLENGE | CO1.7 schema extended; CO1.9 derive test added |
| 5 | **CO1.1.4-pre1 NEW** — kill `bus.rs:268 completion_tokens: 0` literal in a single ceremonial commit BEFORE bus split | Codex CO P0.7 §3 highest-risk-line + own request | inserted before CO1.1.4 |
| 6 | **B-1 governance gate** — PGP/SSH-signed git tag ratification for any TR mutation; AUDIT_LEDGER records tag + fingerprint + verification output | Codex T+S §A B-1 PASS + §D | new governance gate Inserted into CO P0 exit + every CO Phase exit |

Plus minor:
- D5 RSP **prerequisite atom CO P2.0a** — choose i64 micro-coin numeric type before EscrowVault begins
- TRACE_MATRIX_v3 **N/M/D classification** added (CO0.8)

---

## § 2 Decision Reconciliation Summary

| # | v3.1 status | v3.2 status | Why changed |
|---|---|---|---|
| D-VETO-1 | parallel A/B 5-way + 3-way | **spec-first + STEP_B against spec** (binding form via CO1.SPEC.0) | Codex CHALLENGE: parallel A/B without binding spec produces "two independently wrong refactors" |
| D-VETO-2 | f64 → i64 fixed-point (cents) | **i64 micro-coin (10⁻⁶)** | Claude T+S refinement: cent precision insufficient for DAG attribution products |
| D-VETO-3 | 5-line hyper-minimal | **8-field minimal-with-anchor** (`amendment_predicate_hash` + `initial_*_registry_root` + `boot_attestation_hash`) | Codex CHALLENGE: ID without content hash = bootstrap circularity |
| D-VETO-4 | ~~permanent abandon runtime MetaTape~~ | **defer runtime MetaTape to v4.1; v4 produces `MetaProposalDraft` to L3 CAS only; preserve v4.1+ path** | Codex VETO: WP § 12 + § 17 explicitly require meta-transition semantics; permanent abandon contradicts authoritative spec |
| D-VETO-5 | full coverage | **N/M/D classification + full coverage of N** | Codex agree, just clarify granularity |
| D-VETO-6 | rejection NOT on tape | **system-signed bounded retry metadata stamped on next accepted tx + TerminalSummaryTx for no-accept runs** | Codex CHALLENGE: Art 0.2 currently requires reject Node on tape; user's intent honored via Reading Y (see `ART_0_2_REINTERPRETATION_2026-04-27.md`) |
| D-VETO-7 | pre-split atom | **CO1.1.4-pre1 single ceremonial commit** | unchanged |
| B-1 | verbal "我批准" | **PGP/SSH-signed git tag with AUDIT_LEDGER row** | Codex PASS + reinforced |
| D1 | C MVP-pivot | **C — but DEFERRED until L6/error/cost tape schema (CO1.7+CO1.9) settled** | Codex: D1 cannot be settled before schema choice |
| D2 | B pointer + 6 axioms | unchanged (pointer must point to anchored bootstrapped genesis, not naked 5-line) | Codex agreement |
| D3 | A deprecate | unchanged | Codex PASS |
| D4 | B v4.1 spec-only | unchanged (was already correct; v3.1 was right; my T+S "D" was wrong) | Codex VETO on T+S "D" |
| D5 | A full | A full + i64 prerequisite atom CO P2.0a | Codex agreement |
| D6 | A full audit | unchanged | Codex PASS |

---

## § 3 New + Amended Atoms

### CO0.8 (NEW) — TRACE_MATRIX_v3 N/M/D classification + full coverage

| Atom | Scope | File |
|---|---|---|
| CO0.8.1 | Tag every WP § + every Constitution Article as `[N]ormative` / `[M]otivational` / `[D]eferred` | `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` (NEW) |
| CO0.8.2 | Map every Normative § to ≥1 code symbol + ≥1 conformance test (Codex listed missing rows: arch §6, §8, §9.1-9.3, §11, §14-16, econ §0, §20) | same file |
| CO0.8.3 | Mark every Deferred § with deferred reason + target version (v4.1 / v5 / never) | same file |

Total: 3 atoms; ~3-5 days.

### CO1.SPEC.0 (NEW) — State-Transition Spec Gate (BLOCKING for CO1.1.4/CO1.1.5)

| Atom | Scope | File |
|---|---|---|
| CO1.SPEC.0.1 | Author `STATE_TRANSITION_SPEC_v1_2026-04-27.md` (DONE 2026-04-27) | `handover/specs/STATE_TRANSITION_SPEC_v1_2026-04-27.md` |
| CO1.SPEC.0.2 | Codex independent review of spec (T+S re-review approves the doc form) | (audit packet) |
| CO1.SPEC.0.3 | Gemini cross-review (this v3.2 patch invokes) | (audit packet) |
| CO1.SPEC.0.4 | OPTIONAL: TLA+ TLC model check on ordering invariants (I-DET, I-DETHASH, I-LOGTIME) | `handover/specs/STATE_TRANSITION_SPEC_TLA_2026-04-27.tla` |
| CO1.SPEC.0.5 | Spec freeze: v1 SHA locked into Trust Root manifest | `genesis_payload.toml` |
| CO1.SPEC.0.6 | Conformance test scaffold for all 16 invariants in spec § 4 (skeleton, populated by CO P1 atoms) | `tests/state_transition_*.rs` |

Total: 6 atoms; CO P0 → CO P1 transition gate.

### CO1.0 (AMENDED) — Constitution Root + Genesis Minimal-With-Anchor

Replaces v3.1 CO1.0 (which was 2 atoms):

| Atom | Scope | File |
|---|---|---|
| CO1.0.1 | Author `GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md` (DONE 2026-04-27) | `handover/specs/GENESIS_MINIMAL_WITH_ANCHOR_v1_2026-04-27.md` |
| CO1.0.2 | Compile / hash the v4 amendment_predicate (Rust source initially; WASM later) | `src/governance/amendment_predicate.rs` (NEW) |
| CO1.0.3 | Add `[constitution_root]` 8-field section to `genesis_payload.toml` | `genesis_payload.toml` |
| CO1.0.4 | Extend `boot::verify_trust_root` to validate `[constitution_root]` (5 new sub-checks) | `src/boot.rs` |
| CO1.0.5 | Pin gretjia's PGP/SSH public key in `boot.rs` build-time const | `src/boot.rs::PINNED_CREATOR_PUBKEY` |
| CO1.0.6 | Conformance tests (5 new) | `tests/genesis_*.rs` |

Total: 6 atoms (was 2). +4 atoms; +1 wk wall-clock.

### CO1.1.4-pre1 (NEW) — Kill `completion_tokens: 0` literal

Single ceremonial commit BEFORE the bus.rs split:

| Atom | Scope | File |
|---|---|---|
| CO1.1.4-pre1 | Read real `usage.completion_tokens` from LLM response into `Node.completion_tokens`; remove the `: 0` literal at `src/bus.rs:268` | `src/bus.rs` (still STEP_B restricted; this atom is the EXCEPTION because it's a 1-line fix with high symbolic weight) |

Total: 1 atom; ~half a day.

### CO1.7 (AMENDED) — Transition Ledger + Retry Metadata + System Keypair

Adds 6 atoms beyond v3.1's 7 (was +2; Gemini v3.2 Q9 VETO required keypair lifecycle spec):

| Atom | Scope | File |
|---|---|---|
| CO1.7.0 (NEW) | Define `RejectedAttemptSummary` + `TerminalSummaryTx` types per spec § 1.4-1.5 | `src/bottom_white/ledger/retry_metadata.rs` (NEW) |
| **CO1.7.0a (NEW)** | Author SYSTEM_KEYPAIR_SECURITY_v1 spec (DONE 2026-04-27) | `handover/specs/SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` |
| CO1.7.0b (NEW) | System keypair generation + load + encrypted-at-rest (Argon2id KDF + ChaCha20-Poly1305) | `src/bottom_white/ledger/system_keypair.rs` (NEW) |
| **CO1.7.0c (NEW)** | Sign API with `pub(restricted)` enforcement; cargo-deny rule prevents unrestricted sign export | `src/bottom_white/ledger/system_keypair.rs` |
| **CO1.7.0d (NEW)** | Epoch rotation tool + emergency rotation tool | `src/bin/rotate_system_keypair.rs` (NEW) + `src/bin/emergency_rotate_system_keypair.rs` (NEW) |
| **CO1.7.0e (NEW)** | `[system_pubkeys]` section in genesis_payload.toml + boot::verify_system_pubkeys | `genesis_payload.toml` + `src/boot.rs` |
| **CO1.7.0f (NEW)** | 5 conformance tests for keypair (gen / load / restricted-sign / verify / rotation-proof) | `tests/system_keypair_*.rs` |
| CO1.7.1 | (existing v3.1) `TransitionTx` 12-field struct (12, not 11; task_id) | unchanged from v3.1 patches |
| CO1.7.2 | Append API + WAL fsync — extended to handle TerminalSummaryTx | `src/bottom_white/ledger/transition.rs` |
| CO1.7.3 | Replay reconstructs L5 + L6 — extended for retry metadata | `src/bottom_white/materializer/state_db.rs` |
| CO1.7.4 | Migrate existing 5 EventType variants → TransitionTx subtypes | unchanged |
| CO1.7.5 | step_transition fn from spec § 3 | `src/transition/mod.rs` |
| CO1.7.6 | Conformance test for L4 | unchanged |
| CO1.7.7 | Inv 11 test | unchanged |
| CO1.7.8 (NEW) | Conformance test: `derive_l6_from_tape(tape) == runtime_sidecar_snapshot` byte-identical | `tests/l6_reconstructibility.rs` |

Total: 9 atoms (was 7).

### CO1.9 (AMENDED) — Signal Indices + Retry Metadata Decoder

Adds retry-metadata derivation to the L6 unification work:

| Atom | Scope | File |
|---|---|---|
| CO1.9.1-CO1.9.4 | (existing v3.1) | unchanged |
| CO1.9.5 (NEW) | `derive_failure_class_histogram_from_tape(tape) → BTreeMap<RejectionClass, u32>` | `src/bottom_white/signal_index/failure_histogram.rs` |
| CO1.9.6 (NEW) | Conformance test: derive output matches every accepted tx's `RejectedAttemptSummary` plus any TerminalSummaryTx | `tests/failure_histogram_reconstruct.rs` |
| CO1.9.7 (existing CO1.9.5) | conformance test for L6 (renumbered) | unchanged |

Total: 7 atoms (was 5).

### CO P2.0a (NEW) — i64 Micro-Coin Money Type

BLOCKING precondition for CO P2.2 EscrowVault:

| Atom | Scope | File |
|---|---|---|
| CO2.0a.1 | Define `MicroCoin(i64)` newtype + arithmetic ops + checked_{add,sub,mul} + display formatting | `src/economy/money.rs` (NEW) |
| CO2.0a.2 | Refactor `src/prediction_market.rs` from `f64` to `MicroCoin` (~50 LOC change) | `src/prediction_market.rs` |
| CO2.0a.3 | Conformance test: `Inv 3 monetary conservation` byte-identical at all accept boundaries | `tests/economic_invariant_INV3_escrow_only.rs` |
| CO2.0a.4 | NO `f64` allowed in any module under `src/economy/` (cargo-deny rule) | static analysis |

Total: 4 atoms; ~3-5 days; gates ALL CO P2 work.

### CO P0.7' (NEW) — TR Mutation Ratification Governance Gate

Replaces v3.1's TR-mutation handling with formal governance:

| Atom | Scope |
|---|---|
| CO P0.7'.1 | User produces PGP/SSH-signed git tag for the current TR state |
| CO P0.7'.2 | Runtime hook: every TR mutation requires a fresh signed tag added to AUDIT_LEDGER |
| CO P0.7'.3 | Conformance: `AUDIT_LEDGER.tr_ratifications` == `git tag -l v4-tr-*` set; mismatch fails phase exit |
| CO P0.7'.4 | Recovery procedure documented: if user wishes to revert a ratification, what's the procedure |

Total: 4 atoms; ~1 day.

---

## § 4 Updated Atom Total

| Phase | v3.1 atoms | v3.2 atoms | v3.2-fix1 atoms | Δ vs v3.1 |
|---|---|---|---|---|
| CO P0 | 7 | 14 | 14 + 1 (CO P0.9 MetaTx schema) = 15 | +8 |
| CO P1 | ~62 | 77 | 77 + 4 keypair (CO1.7.0a/c/d/e/f as 4 atoms grouped) + 1 CO P3-prep.2 = 82 | +20 |
| CO P2 | ~64 | 68 | 68 + 5 CO P3-prep (.3/.4/.5/.6/.7) = 73 | +9 |
| **v4 total** | **~133** | **~159** | **~170** | **+37 atoms** |

**Wall clock**: 17-21 weeks → **22-27 weeks** (added ~2 more weeks beyond v3.2 for keypair security + Phase 3 prep concrete artifacts; v3.2-fix1 closes Gemini Q9 VETO + Q7 CHALLENGE)

**Cost**: $435-950 → $520-1100 → **$580-1200** (mid $890; +37% vs v3.1; trust restoration via redundancy has a real price; Gemini v3.2 cross-review cost ~$0.30 just yielded 2 VETOs caught + 1 CHALLENGE — ROI clearly positive)

---

## § 5 Critical Path Visualization

```text
[CO P0]
 ├─ CO0.1-0.6 (existing TR mutation, already shipped)
 ├─ CO0.7 dual audit (Gemini done; Codex done; Codex T+S done)
 ├─ CO0.7' TR ratification (new) — REQUIRES user PGP tag
 └─ CO0.8 TRACE_MATRIX_v3 N/M/D + full N coverage (3-5 d)
   │
   └─→ [CO P1 entry GATE]

[CO P1]
 ├─ CO1.SPEC.0 state-transition spec gate (3-5 d)
 │   └─ BLOCKS → CO1.1.4 / CO1.1.5
 ├─ CO1.3.1 gix substrate spike FIRST (5 d, time-boxed)
 │   └─ failure → git2-rs pivot Plan v3.3
 ├─ CO1.0 minimal-with-anchor genesis (~1 wk, 6 atoms)
 ├─ CO1.1.1-3 skeleton + safe moves (~3 d)
 ├─ CO1.1.4-pre1 kill completion_tokens (~half d) — single ceremonial commit
 ├─ CO1.1.4 bus split (STEP_B against spec) (~1.5 wk)
 ├─ CO1.1.5 kernel split (STEP_B against spec) (~1.5 wk)
 ├─ CO1.2-1.6 standard atoms (~4-5 wk)
 ├─ CO1.7 transition ledger + retry metadata (~2 wk)
 ├─ CO1.8-1.13 standard atoms
 └─ CO1.14 P1 exit dual audit
   │
   └─→ [CO P2 entry GATE]

[CO P2]
 ├─ CO P2.0a i64 micro-coin (BLOCKING; ~3-5 d)
 │   └─ BLOCKS → CO P2.1+
 ├─ CO P2.0 Inv 4 precondition
 ├─ CO P2.1-2.10 standard
 ├─ CO P2.4.0 spike (Inv 8 DAG determinism) BLOCKS CO P2.4.1+
 ├─ CO P2.11 RSP MVP-1
 └─ CO P2.12 P2 exit
```

---

### CO P3-PREP (NEW) — Phase 3 Prep Concrete Deliverables (Gemini v3.2 Q7 CHALLENGE)

D-VETO-4 reverts to "defer runtime MetaTape to v4.1 + ship Phase 3 prep artifacts in v4". Gemini v3.2 challenged "Phase 3 prep" as potentially weasel wording. Concrete deliverables list for v4 Phase 3 prep (each one is auditable):

| Atom | Scope | File | Phase scheduled |
|---|---|---|---|
| **CO P3-prep.1** | Full `MetaTx` schema spec (parent_architecture_root + predicate_patches + tool_patches + log_evidence_cids + reversibility_plan_cid + constitution_check + judge_signatures + human_signature_required + human_signature) — typed schema with serialization format | `handover/specs/META_TX_SCHEMA_v1_2026-04-27.md` (NEW spec doc) | CO P0.9 (after CO0.8) |
| **CO P3-prep.2** | `MetaProposalDraft` CAS storage format + retrieval API (the v4 implementation that ArchitectAI offline produces; NOT runtime-emitted) | `src/bottom_white/cas/meta_proposal_draft.rs` (NEW) | CO P1 (alongside CO1.4 CAS) |
| **CO P3-prep.3** | "MetaTx Validator" dry-run library: `validate_meta_proposal(proposal: &MetaProposalDraft, q: &QState) → ValidatorVerdict`. Takes a draft, returns PASS/VETO without applying. No state mutation. Used by ArchitectAI offline + future v4.1 runtime | `src/governance/meta_validator.rs` (NEW) | CO P2 (after settlement_engine) |
| **CO P3-prep.4** | Constitution Art V.1-V.3 amendment-flow audit trail format: every Art V.3 amendment must produce a parseable record (markdown frontmatter + structured fields) ready for v4.1 runtime ingestion | `handover/specs/AMENDMENT_FLOW_FORMAT_v1.md` (NEW) + `tests/amendment_flow_format_validate.rs` | CO P2 |
| **CO P3-prep.5** | `MetaTransitionInterface` Rust trait: declares signatures the v4.1 runtime ArchitectAI/JudgeAI actor will implement; v4 ships the trait but no implementor | `src/governance/meta_transition_interface.rs` (NEW trait) | CO P2 |
| **CO P3-prep.6** | v4.1 MetaTape Implementation Plan doc: detailed atomization plan for v4.1's runtime ArchitectAI/JudgeAI actors that consumes CO P3-prep.1-.5 outputs | `handover/architect-insights/V4_1_METATAPE_PLAN_v1.md` (NEW) | CO P2 exit |
| **CO P3-prep.7** | Conformance test: `meta_validator` correctness — given hand-crafted PASS / VETO / borderline proposals, validator must return correct verdicts | `tests/meta_validator_correctness.rs` | CO P2 |

Total: 7 atoms; ~2 weeks across CO P0/P1/P2; not a full MetaTape implementation but auditable concrete artifacts.

**What CO P3-PREP is NOT**:
- Not a runtime ArchitectAI/JudgeAI implementation
- Not actual `meta_tx` acceptance into L4 (deferred to v4.1)
- Not a governance trial run (no actual amendment via `meta_validator`)

**What CO P3-PREP IS**:
- Typed schema for v4.1 to consume
- Validator library callable both offline (v4 ArchitectAI workflow) and runtime (v4.1)
- Documented amendment flow format for amendments produced by v4 cp workflow
- Trait-level interface guaranteeing v4.1 runtime can plug in

This is auditable. Each artifact has a SHA, each has a conformance test. v4.1 implementation plan will reference these specifically.

---

## § 6 What v3.2 Does NOT Change from v3.1

- The 9-component Q_t is unchanged (state-transition spec is a binding form FOR it, not a replacement)
- The 12 economic invariants → 12 conformance tests mapping is unchanged
- The Anti-Oreo 4-root file structure (`top_white/middle_black/bottom_white/economy`) is unchanged
- ChainTape L0-L6 mapping is unchanged
- TFR v1 LEGACY status is unchanged
- D2 (Constitution Art 0.5 = pointer + 6 axioms) is unchanged
- D3 (TFR v1 deprecate but preserve) is unchanged

---

## § 7 Honest Acknowledgements

- v3.2 is a **patch overlay** on v3.1; both must be read together (one for atoms, the other for revised atoms + new atoms)
- Wall clock 20-25 weeks is honest; 17-21 was under-estimating (Codex was right)
- $520-1100 budget is wider than $435-950; trust restoration via redundancy has a real price
- The 159 atom count assumes no further VETO from Gemini cross-review; if Gemini surfaces new issues, v3.3 patches incoming
- ArchitectAI explicitly retracts the original "permanently abandon runtime MetaTape" T+S claim. WP § 12 + § 17 are right; the T+S analogy was over-extended.
- ArchitectAI explicitly accepts Codex's "spec-first as binding form, not slogan" critique. Spec is a contract, not vibes.

— ArchitectAI, 2026-04-27
