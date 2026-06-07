# Agentic-OS Minimum-Qualification Packet — Operational Definition + Honest Status

Date: 2026-06-07
Status: report / handover packet, Class 0 — derived view (ChainTape / CAS /
constitution gates win on conflict).
Author lane: S6 (verification / falsifiable-governance).
FC anchors: meta / gate-layer soundness (Tier-1 gate layer) — this packet defines
the OPERATIONAL qualification predicate over the constitution gate suite; it
asserts no new runtime behavior.

Companion machine artifact: `tests/constitution_agentic_os_minimum_qualification.rs`
(`constitution_agentic_os_minimum_qualification`) — the SOURCE-STRUCTURAL
meta-gate that makes the conjunction below explicit + falsifiable.

Sibling docs (read for the negative claim and the layer contract):
- `handover/reports/AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md` (HAVE/MISSING
  per-layer status; the negative load-bearing claim "NOT YET qualified").
- `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md` (L0–L9
  layer contract).
- `handover/design/CONSTITUTION_CONFORMANCE_HARNESS_2026-06-07.md`
  (the enumerate-all-sites completeness-gate methodology this meta-gate follows).
- `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md`
  (the adversarial sweep that produced 5 of the qualifying conformance gates).

> **Read first — this is a status, not a claim.** The single load-bearing claim
> of this packet is the **scoped positive** one: the substrate currently
> satisfies the **MINIMUM-qualification conjunction** (§3, all GREEN on main),
> and the **negative** one: it is **NOT** "Agentic OS qualified" in full — the
> dimensions in §4 (G4 budget ceiling, G5 FC3 self-evolution engine, cross-session
> memory, OS-level sandbox, multi-LLM market proof, interop) are honestly
> recorded as **NOT yet qualified**. No global "Agentic OS qualified" claim is
> made or permitted here.

---

## §1. The operational definition (research report §5)

A substrate reaches the **Agentic-OS minimum qualification** when BOTH
properties hold and are MACHINE-RECONSTRUCTABLE from the tape, not asserted from
a dashboard, a self-report, or prose:

1. **Tape-reconstructable evidence.** *A person with paper can reconstruct all
   load-bearing signals from the tape.* Every load-bearing signal — what was
   attempted, what was admitted, what was rejected and why, what money moved,
   what the agent was allowed to see — is derivable by replaying the canonical
   ChainTape + CAS alone, with no privileged side channel. Derived views
   (dashboards, `LATEST.md`, boards, reports including THIS one) are
   reconstructions, never sources.

2. **Predicate-gated irreversibility.** *Every irreversible advance is gated on
   re-executed predicates.* No verified-head / state-root advance occurs without
   a re-executed predicate-admission verdict recorded on tape; a single shared
   admission contract decides "admit" for both the kernel leg and the sequencer
   leg, so the two authorities cannot disagree; and an OS-qualified run refuses
   to admit a self-asserted claim at a zero predicate-registry root (it requires
   a real oracle re-execution, not a trusted boolean).

These two properties, taken together over the gate suite, are the OPERATIONAL
minimum. They are NECESSARY for any "predicate-gated, single-admission,
tape-first agentic substrate" claim. They are explicitly **not sufficient** for
a full "Agentic OS qualified" claim — see §4.

---

## §2. Why a CONJUNCTION, and why make it falsifiable

The 2026-06-07 conformance sweep
(`handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md`) proved that
"M07 is not an isolated bug" but a **class of systemic failure**: a completeness
invariant ("property P holds at EVERY site of class S") gets enforced at exactly
one "obvious" site while a parallel site silently violates it, and the gate stays
GREEN because it too only checked the one site.

The minimum qualification is therefore stated as an explicit **conjunction** of
the individual qualifying gates, and a META-GATE makes that conjunction
falsifiable: if any single qualifying gate is removed, renamed, or disabled
(its test file deleted, OR its manifest `[[gate]]` entry removed), the meta-gate
turns RED. This closes the failure mode where the qualification signal degrades
one leg at a time without anyone noticing — exactly the M07 illusion applied to
the qualification claim itself.

The meta-gate is **SOURCE-STRUCTURAL and non-vacuous**: it reads the live tests/
tree and the live manifest, asserts the conjunction set is non-empty and unique,
and fails LOUD if any listed name is missing. It cannot be satisfied by
`assert!(true)`.

---

## §3. The minimum-qualification conjunction (all GREEN on main)

The operational Agentic-OS minimum qualification =

```text
AGENTIC_OS_MINIMUM_QUALIFICATION :=
      [tape-canonicality]            constitution_tape_canonical_gate
  AND [predicate-gated advance]      constitution_kernel_predicate_gate
                                 AND constitution_single_admission_contract
                                 AND constitution_single_admission_behavioral
                                 AND constitution_kernel_predicate_receipt_replay
                                 AND constitution_predicate_zero_root_is_not_oracle
  AND [trust-root anchoring]         constitution_all_canonical_writers_verify_trust_root
                                 AND constitution_tc_boot_trust_root_manifest
  AND [FC1 attempt accounting]       constitution_llm_err_lands_on_tape
                                 AND constitution_external_attempt_anchored_on_failure
  AND [read-view shield]             constitution_judge_reason_no_raw_subprocess_stderr
                                 AND constitution_metric_leak_guard_wired
                                 AND constitution_shielding_gate
  AND [money conservation]           constitution_economy_gate
                                 AND constitution_economy_strict_equality
  AND [clean-context closure witness] constitution_obl005_final_closure_witness
  AND no regression in the existing constitution gate suite
        (`cargo test --workspace --no-fail-fast` exit 0,
         `bash scripts/run_constitution_gates.sh` exit 0,
         `cargo test --test constitution_matrix_drift` exit 0)
```

Mapping each leg to the two operational properties of §1:

| Dimension | Qualifying gate(s) | Supports §1 property |
|-----------|--------------------|----------------------|
| Tape-canonicality / single source of truth | `constitution_tape_canonical_gate` | (1) reconstructable — canonical tape is the only `Q_t` substrate; boards/dashboards are derived |
| Predicate-gated advance (M07 route A; G1/G2/G3) | `constitution_kernel_predicate_gate`, `constitution_single_admission_contract`, `constitution_single_admission_behavioral`, `constitution_kernel_predicate_receipt_replay`, `constitution_predicate_zero_root_is_not_oracle` | (2) irreversibility — head advance carries a tape-recorded re-executed admission receipt; both authorities agree; zero-root refuses oracle-blind admission |
| Trust-root anchoring | `constitution_all_canonical_writers_verify_trust_root`, `constitution_tc_boot_trust_root_manifest` | (1)+(2) — tamper of `constitution.md` / pinned manifest is detectable; canonical writers verify before advancing |
| FC1 tape-reconstructable attempt accounting | `constitution_llm_err_lands_on_tape`, `constitution_external_attempt_anchored_on_failure` | (1) reconstructable — every externalized LLM cycle (incl. failure arms) lands on tape so `completed_llm_calls = step + parse_fail + llm_err` is reconstructable |
| Read-view shield (Art. III) | `constitution_judge_reason_no_raw_subprocess_stderr`, `constitution_metric_leak_guard_wired`, `constitution_shielding_gate` | (1) scoped/shielded read views — no raw subprocess stderr / metric (Goodhart) leak reaches the agent prompt |
| Money conservation (Art. 0 / integer-only) | `constitution_economy_gate`, `constitution_economy_strict_equality` | (1) reconstructable economy — no mint/burn outside on_init; strict integer conservation; money is a tape-derived signal |
| Clean-context closure witness (Art. V / no-zombie) | `constitution_obl005_final_closure_witness` | (1) reconstructable closure — derived views bound to fresh tape evidence; unscoped global completion claims refused |

**Status of this conjunction: GREEN on main as of 2026-06-07.** The M07
predicate-gated-advance legs (G1 `constitution_kernel_predicate_gate`, G2
`constitution_single_admission_*`, G3 `constitution_predicate_zero_root_is_not_oracle`)
landed under the user's §8 tokens
`APPROVE-M07-A4-SINGLE-ADMISSION-PREDICATE-GATE` and
`APPROVE-M07-G3-OS-QUALIFIED-RUN-FIELD`; the all-writers-verify-trust-root leg
landed under `APPROVE-ALL-CANONICAL-WRITERS-VERIFY-TRUST-ROOT`; the four
conformance-sweep legs (`constitution_llm_err_lands_on_tape`,
`constitution_external_attempt_anchored_on_failure`,
`constitution_judge_reason_no_raw_subprocess_stderr`,
`constitution_metric_leak_guard_wired`) landed under the 2026-06-07 conformance
remediation. This supersedes the earlier all-red `M07_EXPECTED_RED` state
recorded in `AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md §5` (written pre-merge).

**Falsification rule.** Running `cargo test --test
constitution_agentic_os_minimum_qualification` and `bash
scripts/run_constitution_gates.sh` reproduces the conjunction. If any qualifying
gate is removed/renamed/disabled, the meta-gate
`every_os_qualifying_gate_has_a_test_file` /
`every_os_qualifying_gate_is_registered_in_the_manifest` /
`every_qualification_dimension_is_covered` fails RED naming the dropped leg —
that is the machine alarm against a silently-shrunk qualification claim.

---

## §4. STILL-PENDING dimensions — honestly NOT yet qualified

The conjunction in §3 is the MINIMUM. The substrate is **NOT** "Agentic OS
qualified" in full. The following qualification dimensions are **NOT YET**
satisfied — each is recorded here as an explicit gap, NOT as a passing leg. None
of them is in the meta-gate's `OS_QUALIFYING_GATES` list (putting a not-yet-true
leg in the conjunction would make the conjunction dishonest).

| Dimension | What is missing | Tracker / standing token | Why excluded from §3 |
|-----------|-----------------|---------------------------|----------------------|
| **G4 — budget ceiling enforcement** | `BudgetSnapshot` fields (`cost_ceiling_microcoin`, `wall_clock_remaining_ms`, `compute_cap_remaining`, `src/state/q_state.rs`) default to zero and NO admission gate compares them against any Art. V.2 ceiling. Whether Art. V.2 numbers are HARD ceilings or illustrative is an open §8 question; ceiling values must come from genesis/manifest. | `BUDGET_CEILING_STANDING_PENDING` (pending gate G4; `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`) | No GREEN gate enforces a budget ceiling on admission. PENDING. |
| **G5 — FC3 self-evolution engine** | FC3 governance SUBSTRATE is live (typed-tx + sequencer arms + Veto-AI verdict checks), but the runtime LOOP is missing: log → proposer → synthesis → canary → trust-root rewrite → re-init. Live role-path proposal payload is `ToolProposalPayload::default()` (`proposal_id == None`); accepted-proposal terminal status is `sandbox:canary_only` (dead-end, never re-init / trust-root recompute). | `FC3_META_LOOP_STANDING_PENDING` (pending gate G5; Class-4 §8 — touches RootBox / boot trust-root) | The self-evolution loop does not close; substrate ≠ engine. PENDING. |
| **Cross-session memory** | No tape-canonical durable agent memory (e.g. `SkillCapsule`) beyond per-run belief state; capability compilation across sessions is not yet a reconstructable on-tape signal. | B5 (memory model) in `AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md §4` | No GREEN gate binds cross-session memory to tape. PENDING. |
| **OS-level sandbox** | Per-agent capability admission + taint / agent-view shielding enforcement (L4×L7) beyond current scoping; no enforced process-level sandbox boundary on agent side effects. | B6 (taint·capability) in the status doc §4; L3/L4 of the OS layer contract | Read-view shield (§3) covers prompt-leak shielding, NOT a full OS sandbox. PENDING. |
| **Multi-LLM market proof** | No fresh current-kernel ChainTape proof of a genuine MULTI-LLM market (heterogeneous providers competing/cooperating on real economic actions). Prior single-LLM schema evidence does not satisfy the multi-LLM requirement. | MEMORY "Economy-aware agent prompt" landing gap (superseded 2026-05-11: user requires multi-LLM proof on tape); G4-G6 generative-arena directive | No GREEN gate witnesses multi-LLM market emergence on current-kernel tape. PENDING. |
| **Interop** | No demonstrated interoperability surface (cross-runtime / external-agent / A2A) bound to canonical tape as a qualified, reconstructable signal. | TISR-001 dual-axis research (A2A axis, Phase 7 pending); OS layer contract L3 outbox | No GREEN gate binds an interop surface to qualification. PENDING. |

**Full-qualification rule (NOT met today).** A full "Agentic OS qualified" claim
additionally requires G4 (budget) and G5 (FC3 engine) to land under their own §8
rulings, plus cross-session memory, an OS-level sandbox, a multi-LLM market proof
on current-kernel tape, and an interop surface — each as its OWN GREEN gate
triple-coupled into the suite — plus a clean-context audit (`AGENTS.md §9`)
returning a non-violation verdict on the landed topology. Until then the only
permitted positive claim is the SCOPED one of §3 (minimum qualification), and the
negative claim of `AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md` ("NOT YET Agentic-OS-qualified") stands for the FULL claim.

---

## §5. One-line falsifiable summary

```text
minimum_qualification: MET (GREEN on main, 2026-06-07) — the §3 conjunction of
  16 OS-qualifying gates across 7 dimensions (tape-canonicality, predicate-gated
  advance, trust-root anchoring, FC1 attempt accounting, read-view shield, money
  conservation, clean-context closure witness), made explicit + falsifiable by
  tests/constitution_agentic_os_minimum_qualification.rs.
full_qualification:    NOT MET — G4 budget ceiling, G5 FC3 self-evolution engine,
  cross-session memory, OS-level sandbox, multi-LLM market proof, and interop are
  honestly recorded as PENDING (§4); no global "Agentic OS qualified" claim is made.
falsification:         remove/rename/disable any §3 gate → meta-gate RED naming the
  dropped leg; add a not-yet-true §4 dimension to the conjunction → dishonest, forbidden.
```

---

## §6. References

- Meta-gate (machine artifact): `tests/constitution_agentic_os_minimum_qualification.rs`.
- Manifest entry: `scripts/constitution_gates.manifest.toml`
  (`constitution_agentic_os_minimum_qualification`).
- Matrix row: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
  ("Agentic-OS minimum-qualification meta-gate" row).
- Negative status (FULL claim is NOT met): `handover/reports/AGENTIC_OS_STATUS_AFTER_PR314_2026-06-07.md`.
- Conformance harness methodology: `handover/design/CONSTITUTION_CONFORMANCE_HARNESS_2026-06-07.md`.
- Adversarial sweep (source of 4 qualifying conformance gates):
  `handover/audits/CONSTITUTION_CONFORMANCE_SWEEP_2026-06-07.md`.
- Pending kill-condition gates (G4/G5 standing):
  `handover/audits/PENDING_AGENTIC_OS_KILL_CONDITIONS_2026-06-07.md`.
- OS layer contract (L0–L9): `handover/directives/OS_PIVOT_AGENTIC_OS_SUBSTRATE_2026-06-05.md`.
- Claim discipline: `AGENTS.md §9, §14, §14b`; `skills/no-proven-checklist.md`.
```
