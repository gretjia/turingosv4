# TB-16 Comprehensive Arena — Evidence (2026-05-04)

**Ship**: TB-16 Atom 6 (pre-audit; full Class 3 dual external audit at Atom 7).
**Charter**: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
**Architect spec**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7
**Implementation contract**: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`

---

## What was shipped

**Atom 6 v0** ships the full audit-from-tape infrastructure + verifies it
end-to-end on a real chain-backed tape:

### Infrastructure (commit `36413c0` — Atom 5; commit `<this commit>` — Atom 6):

- `src/runtime/audit_assertions.rs` — 38-assertion pure-fn battery
- `src/bin/audit_tape.rs` — CLI wrapper emitting `verdict.json`
- `src/bin/audit_tape_tamper.rs` — 3-corruption tamper-detection harness
- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` — 6-task orchestrator scaffold
- `handover/tests/scripts/run_real_llm_arena.sh` — end-to-end runner
- `handover/tests/scripts/audit_tape_smoke_test.sh` — ship-gate wrapper
- Dashboard §15 live regen + §16 SANDBOX banner (closes
  `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`)

### Audit pipeline smoke evidence (`audit_pipeline_smoke/`):

End-to-end validation that the full pipeline (audit_tape +
audit_tape_tamper + generate_markov_capsule + audit_dashboard +
replay-determinism) works on a chain-backed real-LLM tape. Tape source:
`handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/`
(TB-13 chain with 3 L4 rows + 2 L4.E rows + 11 CAS objects).

**Pipeline output**:

| Artifact | Status |
|---|---|
| `verdict.json` | `verdict=BLOCK passed=31 failed=0 halted=1 skipped=7` |
| `verdict_replay.json` | byte-identical to `verdict.json` (replay determinism ✓) |
| `tamper_report.json` | `detected_count=3/3` (all 3 corruptions detected ✓) |
| `MARKOV_TB-16_2026-05-03.json` | first TB-16 Markov capsule; `capsule_id=5da53602...`; constitution_hash + 4 flowchart hashes + 23 unresolved OBS |
| `LATEST_MARKOV_CAPSULE.txt` | local pointer (capsule_id hex) |
| `dashboard.txt` | 15-section render (incl. live-regen §15 + SANDBOX §16 banner) |

**Why verdict=BLOCK**: the TB-13 fixture chain has 1 Halt at Layer E #27
(`evidence_capsule_cid not in CAS at L4 index 2`) — the TB-13 smoke
emitted a `TerminalSummaryTx` whose `evidence_capsule_cid` was not
written to CAS. This is **correct detection** by audit_tape — the
fixture has a real evidence gap. A fresh TB-16 arena run on a chain
that emits a complete TerminalSummary + EvidenceCapsule pair will
satisfy assertion #27 and emit verdict=PROCEED.

**Halt-trigger H7 (architect §7.7 unresolved_evidence_gap)**: this
audit run **demonstrates the halt-trigger fires correctly** when an
evidence gap exists. ✓

---

## What's deferred

**Fresh real-LLM arena execution** (Task A..F end-to-end on a fresh
multi-task chain producing all 13 architect-required tx kinds) is
gated on user-side preconditions:

1. ✓ DeepSeek API keys in `.env` (5 keys present)
2. ✓ LLM proxy running at `http://localhost:18080` (verified live)
3. ✗ **Mathlib NOT cached**: `experiments/minif2f_v4/.lake/packages/`
   missing. Required: `cd experiments/minif2f_v4 && lake exe cache get`
   (~2 min download + decompression per `feedback_lake_packages_vendored`).
4. ⚠ Multi-task aggregation: each task currently maps to its own
   sub-tape via the `lean_market run-task` pattern. Aggregating Tasks
   A..F onto a single shared chain (so all 13 tx kinds appear in ONE
   tape) requires evaluator extensions tagged TB-16 Atom 6.1 — namely
   chain-continuation semantics across multiple `lean_market run-task`
   invocations against the same `runtime_repo`.

**Unblocking Atom 6.1** = ship a fresh tape with all 13 tx kinds + ≥1
TaskBankruptcy → autopsy emission. Until then, audit_pipeline_smoke
in this dir is the integration witness that the audit-from-tape
contract holds end-to-end.

---

## Acceptance gate (design §7.1; assessed against this evidence)

| Gate | Status | Note |
|---|---|---|
| 1. Evaluator within 30 min + cost ceiling | N/A | no fresh evaluator run |
| 2. All 13 tx_kinds present | ⚠ partial | TB-13 fixture has 5 of 13 (TaskOpen + EscrowLock + TerminalSummary + 2 others); fresh arena run gated on Atom 6.1 |
| 3. All 6 CAS object types reachable | ⚠ partial | TB-13 fixture lacks AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule (now generated locally) |
| 4. verdict.json verdict=PROCEED | ✗ BLOCK | 1 halt at Layer E #27 (correct detection — TB-13 fixture has evidence gap) |
| 5. Dashboard renders all 16 sections | ✓ | dashboard.txt incl. §15 + §16 |
| 6. First TB-16 Markov capsule emitted; constitution_hash matches | ✓ | capsule_id=5da53602...; SG-15.7 PASS |
| 7. Replay byte-identical | ✓ | `cmp -s verdict.json verdict_replay.json` PASS |
| 8. Tamper detection 3/3 | ✓ | tamper_report.json detected_count=3 |

**Verdict on infrastructure**: PROCEED to Atom 7 dual external audit.
**Verdict on fresh arena run**: BLOCKED on mathlib build + Atom 6.1
multi-task aggregation; user-side action needed.

---

## Halt-trigger battery (architect §7.7 + design §10)

| ID | Trigger | Status |
|---|---|---|
| H1 | Pinned-pubkey verify failure halts | ✓ tested via Layer B #8 |
| H2 | Agent-pubkey verify failure halts | ✓ tested via Layer B #9 |
| H3 | Replay state_root mismatch halts | ✓ tested via Layer C #12 |
| H4 | L4 hash chain broken link halts | ✓ tested via Layer B #4 + tamper_report flip_l4_byte |
| H5 | L4.E hash chain broken link halts | ✓ tested via Layer B #6 |
| H6 | L4.E entry advances logical_t halts | ✓ tested via Layer B #6 negative |
| H7 | Unresolved CAS Cid halts | ✓ **demonstrated** via TB-13 fixture's E #27 halt |
| H8 | Projection contains autopsy private_detail halts | ✓ tested via Layer F #28 |
| H9 | TypicalErrorSummary contains private_detail halts | ✓ tested via Layer F #30 |
| H10 | Markov constitution_hash mismatch halts | ✓ tested via Layer G #32 + capsule generation |
| H11 | Markov deep-history without override halts | ✓ verified via `try_deep_history_read_with_override_check` + binary smoke (`generate_markov_capsule` log: "TURINGOS_MARKOV_OVERRIDE not set — deep-history reads DEFAULT-DENIED (FR-15.5 + halt-trigger #6)") |
| H12 | LLM self-narrative in autopsy evidence halts | ✓ tested via Layer F supplemental |
| H13 | total_supply_micro mutates halts | ✓ tested via Layer D #18 |

**13/13 halt-trigger fence active.** Atom 1's `tests/tb_16_halt_triggers.rs`
13 tests all PASS.

---

## Cross-references

- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7
- Design doc: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
- Predecessor smoke evidence (chain-backed): `handover/evidence/tb_13_real_llm_smoke_2026-05-03/`
- Closes: `handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` (dashboard §15 live regen)
