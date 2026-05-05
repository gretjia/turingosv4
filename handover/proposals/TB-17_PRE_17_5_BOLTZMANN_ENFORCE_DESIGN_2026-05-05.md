# TB-17 PRE-17.5 design proposal — Boltzmann sequencer-side ENFORCEMENT gate

**Status**: **RATIFIED 2026-05-05 — DESIGN-ONLY SHIP; PRE-17.5 → TB-18.** (Class 0 doc; no code change accompanies this doc.)
**Filed**: 2026-05-05.

**Ratification verdict (2026-05-05)**: **RATIFIED — DESIGN-ONLY SHIP; PRE-17.5 closure → TB-18.**
- Decided by: AI-coder under user-architect autonomous-execution authorization (verbatim: "由你负责执行，一直到TB-17 ship，有任何问题你无法决策，找到架构师意见做准则进行判断，严格执行").
- Standard applied: 2026-05-05 architect verdict §B.8 atom 7 verbatim ("只做 design unless separately ratified [...] Class 4, must stop before code, requires architect ratification, requires Phase Z′ consideration"). No separate ratification of the schema bump was issued within TB-17 window → default fires.
- §9 disposition: decisions 1–5 = NO (no schema bump in TB-17); decision 6 = "defer to TB-18 (atom 7 ships as DESIGN-ONLY only)".
- Forward target: PRE-17.5 binds into TB-18 charter (per atom 8 deviation §6 + architect §B.10.2 "TB-18 — Formal Benchmark Scale-Up").
- TB-17 SG-17.14 ✅ — design-only deferral path satisfied.

**Authority**:
  - TB-17 charter §3 atom 7 + 2026-05-05 architect verdict §B.8 atom 7 verbatim ("只做 design unless separately ratified [...] Class 4, must stop before code, requires architect ratification, requires Phase Z′ consideration").
  - OBS_R024 forward trigger: `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`.
  - 2026-05-04 architect OBS_R022 §A.4 SG-16.x carry-forward.
  - Memory: `feedback_class4_cannot_hide_in_class3` (TB-16.x.2.4 dual-audit precedent for the OBSERVE-vs-ENFORCE gap).

**Implementation status**: this doc documents the implementation plan; **no source change made**. Per architect §B.8 atom 7, implementation requires NEW separate architect ratification of:
  (a) `WorkTx` field bump (canonical schema change).
  (b) `WorkSigningPayload` bump (canonical signing payload change).
  (c) Phase Z′ flowchart-rerun authorization (because admission semantics change).

If architect ratifies all three within TB-17 window, atom 7 may proceed to implementation under Class 4 envelope. Otherwise atom 7 ships as DESIGN-ONLY and PRE-17.5 closure deferred to **TB-18**.

**Gating posture**: per `feedback_no_workarounds_strict_constitution` ("我不要凑活") — AI-coder MUST NOT begin schema bump or canonical-signing-payload edit until ratification. Design-only is the constitutionally-correct outcome under current authorization.

---

## §1 Problem statement

### §1.1 The OBSERVE-vs-ENFORCE gap

TB-16.x.2.4 (commits `b5118fd + 4dd82c1 + e34d178`) introduced a v2 Boltzmann selector that picks parent transactions at proposal time. The pick is recorded in `ProposalTelemetry.parent_tx` (CAS object). Audit assertion id=43 (parent_selection_diversity entropy ≥ 0.5) verifies that the recorded parent picks are diverse.

**The gap**: the sequencer at admission (`src/state/sequencer.rs:540+ WorkTx admission`) does **NOT** verify that the submitted `WorkTx`'s actual parent matches the recorded `ProposalTelemetry.parent_tx`. An adversarial agent could:

1. Construct a `WorkTx` claiming `ProposalTelemetry.parent_tx = X` (Boltzmann's intended pick),
2. Build the actual proposal payload on a different parent `Y`,
3. Submit it; sequencer admits it; chain advances on `Y`-derived state.

The Boltzmann selector becomes **a suggestion, not a rule**, defeating its purpose as an anti-collapse mechanism (Gemini R2 Q1 audit verdict, OBS_R024).

### §1.2 Why this is OK in TB-16 sandbox but NOT in P7

TB-16 is **Controlled Market Smoke Arena** (sandbox). All agents are operator-trusted; no adversarial threat model applies (CR-16.5 sandbox-prefix scan). The OBSERVE side is sufficient for sandbox proof-of-concept.

TB-17 readiness gate REQUIRES the ENFORCE side because:
- Real-world (P7) agents are NOT operator-trusted.
- Boltzmann-based parent selection is **anti-collapse machinery**: prevents one solver from monopolizing chain progression, ensures statistical diversity of contributors.
- Without enforcement, an adversarial agent can target a specific parent (e.g., always pick `Y`) to game the chain's economic incentive structure.

This is **PRE-17.5** in TB-17 charter §1.1 + §3 atom 7.

---

## §2 Required schema changes (verbatim per OBS_R024)

### §2.1 `WorkTx` field bump — add `parent_tx: Option<TxId>` field 12

Current shape (`src/state/typed_tx.rs:223-235` per OBS_R024 cite):

```rust
pub struct WorkTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub task_id: TaskId,
    pub agent_id: AgentId,
    pub stake: MicroCoin,
    pub payload_cid: Cid,
    pub proposal_telemetry_cid: Cid,
    pub omega_attempt: bool,
    pub created_at_logical_t: LogicalT,
    pub bond: MicroCoin,
    pub signature: AgentSignature,
    // 11 fields
}
```

Proposed addition:

```rust
pub struct WorkTx {
    // ... 11 existing fields ...
    pub parent_tx: Option<TxId>,    // FIELD 12 (NEW; canonical)
    // Field 12 carries the agent's CLAIMED Boltzmann-selected parent.
    // Sequencer will cross-check this against admission-time re-derivation.
    // None = legacy / non-Boltzmann-eligible WorkTx (e.g., genesis path).
}
```

**Backward compatibility**: This is a **canonical schema change** — every existing chain's WorkTx serializations become incompatible. Per `feedback_no_retroactive_evidence_rewrite`, existing chain history is preserved as-is (existing chains continue under v1 schema; new chains start under v2 schema).

Migration strategy: chains have a `schema_version` field carried at genesis (or computed from genesis tx layout); sequencer runs the appropriate decoder per chain.

### §2.2 `WorkSigningPayload` bump

The signing payload (used for `signature` field) MUST include `parent_tx` so the agent's signature commits to the parent claim. Otherwise the field is non-binding (an attacker could mutate it post-signing).

Current shape (per OBS_R024 location `typed_tx.rs:~170`):

```rust
struct WorkSigningPayload {
    // mirrors WorkTx fields excluding signature
    tx_id, parent_state_root, task_id, agent_id, stake,
    payload_cid, proposal_telemetry_cid, omega_attempt,
    created_at_logical_t, bond,
}
```

Proposed:

```rust
struct WorkSigningPayload {
    // ... existing fields ...
    parent_tx: Option<TxId>,    // matches WorkTx field 12
}
```

**Risk**: every agent's signing-key must re-derive signatures using the new payload format. All TB-15+ chains currently in `handover/markov_capsules/` reference v1-signed WorkTx; replay tools must select decoder by `schema_version`.

### §2.3 Sequencer admission gate at `src/state/sequencer.rs:540+`

Add before the existing acceptance path:

```rust
// PRE-17.5 admission cross-check
if let Some(claimed_parent) = work_tx.parent_tx {
    let derived_parent = boltzmann_v2_selector::derive_at_admission(
        chain_state,
        seeded_rng_context,
    )?;
    if claimed_parent != derived_parent {
        return Err(RejectionClass::ParentSelectionMismatch {
            claimed: claimed_parent,
            derived: derived_parent,
        });
    }
}
```

Boltzmann v2 admission-time derivation MUST be deterministic given canonical chain state — same RNG seeding, same Q-state, same exposure index. This is feasible because v2 selector is already pure-fn (per TB-16.x.2.4 implementation); only the call site shifts from proposal-time-evaluator to admission-time-sequencer.

### §2.4 New rejection class

Add to `src/state/typed_tx.rs::RejectionClass`:

```rust
RejectionClass::ParentSelectionMismatch {
    claimed: TxId,
    derived: TxId,
},
```

L4.E entry written with both txIds for forensic analysis. Per `feedback_rejection_evidence_separate`, raw_diagnostic privacy-shielded.

### §2.5 Call-site updates

Every WorkTx construction site needs `parent_tx` plumbing:

- `make_real_worktx_signed_by` (test helper) — add `parent_tx` parameter.
- `evaluator.rs` OMEGA-Confirm path — query Boltzmann v2 selector; pass result to WorkTx constructor.
- `evaluator.rs` per-tactic path (TB-8+ scope; not yet shipped; design here for forward compatibility).
- `evaluator.rs` arena hooks (FORCE_BOLTZMANN_SEED_WORKTXS already calls v2; just records its own pick).
- All `agent_keypairs` signing flows touched (signing payload bump §2.2).

---

## §3 Class 4 envelope justification

This is **Class 4** because it touches ALL of:

1. **Canonical schema** — WorkTx field bump invalidates existing chain serializations.
2. **Canonical signing payload** — agent signatures cover a different byte-string post-bump.
3. **Sequencer admission semantics** — what makes a WorkTx admissible changes.
4. **Phase Z′ flowchart implications**:
   - FC1 (runtime loop) — admission gate adds a new edge: WorkTx submission → Boltzmann re-derivation → cross-check → accept-or-L4.E.
   - FC2 (boot) — chain genesis must commit a `schema_version` so sequencer can pick the correct decoder.
   - FC3 (meta) — Markov capsule must record schema_version (post-PRE-17.7 implementation, in-tape).
   - All three flowcharts need their hashes re-derived if the structural change is significant; architect's call.

Per `feedback_class4_cannot_hide_in_class3`, none of these can be absorbed into a Class 3 umbrella. AI-coder MUST stop at this design doc and request explicit ratification.

---

## §4 Phase Z′ rerun cost estimate

| Stage | Estimated cost | Notes |
|---|---|---|
| FC1 hash re-derivation | minutes | one-edge addition; trivial diff |
| FC2 hash re-derivation | minutes | schema_version field on genesis; small |
| FC3 hash re-derivation | minutes | Markov capsule field add (cross-link with PRE-17.7) |
| Trace matrix update | 30-60 min | every WorkTx admission witness needs re-citation |
| Conformance test re-run | 5-10 min | `tests/fc_alignment_conformance.rs` |
| Documentation cite refresh | 15-30 min | search and replace in handover/ |

Total Phase Z′ rerun: **~1-2 hours** of AI-coder time after architect ratification, assuming no scope expansion.

This estimate is a **planning input** for the architect's ratification decision; actual cost may vary if the architect simultaneously requires other flowchart restructuring.

---

## §5 Backward-compatibility plan

Per `feedback_no_retroactive_evidence_rewrite`:

- Existing chains under v1 schema (all current TB-7 / TB-13 / TB-14 / TB-15 / TB-16 chains): preserved as-is. Their `WorkTx` lacks `parent_tx`; sequencer running v2 schema reads them via v1 decoder selected by `schema_version`.
- New chains under v2 schema: include `schema_version = 2` at genesis; all WorkTx carry `parent_tx: Option<TxId>`; sequencer applies admission gate.
- Migration tools: NONE NEEDED for chain bytes. The Markov inheritance β-D pipeline (PRE-17.7 atom 9) MUST handle cross-version inheritance with the existing `constitution_hash_hex` mismatch BLOCK (per MARKOV_INHERITANCE_POLICY §2.3 Case C invalid-pointer detection).
- Workspace tests: TB-15 + TB-16 chains preserved as fixture inputs; v2 admission tests use new fixtures.

---

## §6 Test plan (post-ratification)

When implementation proceeds, the following test files land:

### §6.1 `tests/tb_17_pre175_admission_match_admits.rs`
Construct a v2 WorkTx with `parent_tx = derive_at_admission(chain)`; assert it admits.

### §6.2 `tests/tb_17_pre175_admission_mismatch_blocks.rs`
Construct a v2 WorkTx with `parent_tx ≠ derive_at_admission(chain)`; assert it REJECTS to L4.E with `RejectionClass::ParentSelectionMismatch`.

### §6.3 `tests/tb_17_pre175_replay_determinism.rs`
Run an admission scenario twice; assert byte-identical L4 / L4.E / CAS state. Ensures Boltzmann v2 admission-time derivation is deterministic.

### §6.4 `tests/tb_17_pre175_v1_legacy_chain_replay.rs`
Replay an existing TB-16 v1 chain (preserved fixture); assert replay PROCEEDs even though v1 chain has no `parent_tx` field.

### §6.5 `tests/tb_17_pre175_signing_payload_drift.rs`
Construct a v2 WorkTx; mutate `parent_tx` post-signing; assert signature verification fails (i.e., signing payload bump §2.2 is binding).

---

## §7 Audit envelope (post-ratification)

Class 4 → full hybrid dual external audit (Codex + Gemini), per `feedback_dual_audit` Class 4 tier. Conflict resolution: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.

Audit-loop discipline: no more than 2 dual-audit rounds per `feedback_audit_loop_roi_flip`; if R2 still has VETO on Class 4 surface, escalate to architect for direct decision (do NOT iterate further).

Anti-pattern explicitly forbidden: closing dual-audit findings by removing the schema bump and reverting to design-only after starting implementation. Once architect ratifies and implementation begins, completion is binding.

---

## §8 Constitutional alignment

| Constitutional axiom | Impact |
|---|---|
| **Art. 0.1** (Append-Only DAG) | Preserved — schema bump adds an admission gate; rejected WorkTx go to L4.E (existing append-only DAG); accepted ones to L4. |
| **Art. 0.2** (Tape Canonical) | Preserved — new chain history is fully tape-derivable; v1 chains preserved per backward-compat. |
| **Art. 0.3** (Replay Determinism) | Preserved — Boltzmann v2 admission-time derivation is pure-fn; replay byte-identical. |
| **Art. 0.4** (Q_t version-controlled) | **Impacted** — `schema_version` becomes a Q_t field; chain continuation across schema versions handled by MARKOV_INHERITANCE_POLICY §3.4 + atom 9 in-tape pipeline. |
| **Art. II.1** (typical-error broadcast) | Unchanged — autopsy emission for ParentSelectionMismatch rejections per TB-15 surface. |
| **Art. III.1-4** (Goodhart shield) | **Strengthened** — Boltzmann enforcement is exactly the anti-collapse / anti-Goodhart machinery this article calls for. |
| **Art. V.1** (三权分立) | Unchanged — Boltzmann enforcement is sequencer-side admission, not JudgeAI/VetoAI authority. |

Net Layer 1 invariant impact: POSITIVE. The PRE-17.5 closure restores constitutional Boltzmann anti-collapse to its intended ENFORCE strength.

---

## §9 Decision points for architect ratification

The architect must explicitly authorize each of:

1. **WorkTx field 12 schema bump**: yes / no / amend.
2. **WorkSigningPayload bump**: yes / no / amend.
3. **Sequencer admission gate at sequencer.rs:540+**: yes / no / amend.
4. **Phase Z′ rerun authorization**: yes / no / scoped (specify which flowchart hashes need re-derivation).
5. **Backward-compatibility plan §5 acceptance**: yes / no / amend.
6. **Implementation timing**: within TB-17 window (atom 7 ratified now) / defer to TB-18 (atom 7 ships as DESIGN-ONLY only).

If ALL six are YES, AI-coder proceeds to implementation per §6 test plan + §7 audit envelope.
If ANY is NO or `defer to TB-18`, atom 7 ships as DESIGN-ONLY in TB-17; PRE-17.5 carries forward to TB-18 charter.

**Default in absence of ratification**: DESIGN-ONLY ship; PRE-17.5 deferred to TB-18.

---

## §10 Cross-references

- OBS_R024: `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`
- TB-17 charter atom 7: `handover/tracer_bullets/TB-17_charter_2026-05-05.md` §3 atom 7
- 2026-05-05 architect verdict §B.8 atom 7 verbatim: `handover/directives/2026-05-05_TB17_AUDIT_VERDICT_ARCHITECT_RULING.md`
- 2026-05-04 architect OBS_R022 ruling: `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- TB-16.x.2.4 implementation: commits `b5118fd + 4dd82c1 + e34d178`
- TB-16.x.2.4 dual audits: `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R[12].md` + `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R[12].md`
- Memory: `feedback_class4_cannot_hide_in_class3`, `feedback_no_workarounds_strict_constitution`, `feedback_no_retroactive_evidence_rewrite`, `feedback_dual_audit`, `feedback_dual_audit_conflict`, `feedback_audit_loop_roi_flip`
- Constitution Art. 0.1 / 0.2 / 0.3 / 0.4 / II.1 / III.1-4 / V.1
