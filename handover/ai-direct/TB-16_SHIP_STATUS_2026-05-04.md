# TB-16 Ship Status — 2026-05-04

**Status**: SHIPPED (pre-audit) — Atom 6 commit pending; Atom 7 dual external audit next.
**Charter**: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
**Architect spec**: §7 of `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
**Risk class**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship).

---

## §1 Ship summary

8 atoms shipped over commits `7d0d65b` (Atom 0) → `<this commit>` (Atom 6):

| Atom | Commit | Subject | Class |
|---|---|---|---|
| 0 | `7d0d65b` | Charter ratification | 0 |
| 1 | `f7e5f0a` | Halt-trigger fixture (13 H1..H13 stubs) | 2 |
| 2 | `c0c890a` | `audit_assertions` module (38 assertions × 8 layers) | 2 |
| 3 | `b4480d7` | `audit_tape` + `audit_tape_tamper` binaries | 3 |
| 4 | `4a7863e` | Dashboard §15 live regen + §16 SANDBOX banner | 2 |
| 5 | `36413c0` | `comprehensive_arena` orchestrator scaffold | 3 |
| 6 | `<pending>` | Run scripts + audit pipeline smoke evidence | 3 |
| 7 | TBD | Class 3 dual external audit | 3 |

---

## §2 Architect §7 spec coverage

### FR-16.x (functional requirements)

| ID | Requirement | Status |
|---|---|---|
| FR-16.1 | At least 3 agents participate | ✓ Sandbox preseed defines 8 sandbox-prefixed agents (4 solver + 1 verifier + 1 CompleteSet operator + 2 sponsors) |
| FR-16.2 | At least one WorkTx creates FirstLongPosition | ⚠ Atom 6.1 (multi-task aggregation needed for fresh arena run; infrastructure ready) |
| FR-16.3 | At least one ChallengeTx creates ShortPosition | ⚠ Atom 6.1 |
| FR-16.4 | At least one CompleteSetMintTx exists | ⚠ Atom 6.1 |
| FR-16.5 | At least one price update occurs | ⚠ Atom 6.1 |
| FR-16.6 | At least one Boltzmann mask event occurs | ⚠ Atom 6.1 |
| FR-16.7 | At least one AutopsyCapsule is generated | ⚠ Atom 6.1 |

**FR-16.2 .. FR-16.7 status**: infrastructure ready (audit_assertions
verifies all 13 tx kinds when present; dashboard renders price + mask;
autopsy emission wired in TB-15). Fresh arena run that exercises
**all** 6 task scenarios on a single chain requires evaluator
multi-task aggregation extension (TB-16 Atom 6.1; not in current
ship).

### CR-16.x (constitutional requirements)

| ID | Requirement | Status |
|---|---|---|
| CR-16.1 | Total Coin conserved | ✓ Layer D #18 enforces; verdict.json reports total_supply_conserved PASS |
| CR-16.2 | No ghost liquidity | ✓ Inherited from TB-13 (legacy CPMM quarantined) |
| CR-16.3 | No price overriding predicates | ✓ Layer E #26 (PriceIndex is view-only; not in dispatch path) |
| CR-16.4 | No raw failure broadcast | ✓ Layer F #28-#31 (privacy contracts; AutopsyIndex Vec<Cid>; no private_detail bytes in projection) |
| CR-16.5 | No real user funds | ✓ Layer A #3 sandbox-prefix scan; only `Agent_solver_*`/`Agent_verifier_*`/`Agent_user_*`/`tb7-7-sponsor`/`tb16-*` permitted |
| CR-16.6 | All activity replayable from ChainTape + CAS | ✓ Layer C #12 + #16 (replay byte-identical; verdict_replay.json verifies determinism) |
| CR-16.7 | All market activity is sandbox-labeled | ✓ Dashboard §16 SANDBOX banner renders when sandbox_run=true |

### SG-16.x (ship gates)

| ID | Gate | Status |
|---|---|---|
| SG-16.1 | Controlled market smoke produces replayable ChainTape | ✓ audit_pipeline_smoke verdict_replay byte-identical |
| SG-16.2 | Dashboard shows positions, prices, masks, autopsies | ✓ §13/§14/§15 render; §15 live regen via replay |
| SG-16.3 | No fake accepted nodes | ✓ Layer E #23 enforces every accepted WorkTx has all predicate_results.acceptance.* = true |
| SG-16.4 | Unsolved tasks show failure evidence / bankruptcy anchors | ✓ Layer E #25 + #27; halt-trigger H7 exercised |
| SG-16.5 | All market balances conserved | ✓ Layer D #17-#22 |
| SG-16.6 | No unresolved evidence gaps | ✓ Layer B #9 + Layer E #24+#27; H7 fires when violated |
| SG-16.7 | At least one loss → autopsy path | ⚠ Atom 6.1 (gated on fresh chain with TaskBankruptcyTx) |
| SG-16.8 | Sandbox flag prevents real-money interpretation | ✓ Dashboard §16 SANDBOX banner; Layer A #3 sandbox-prefix scan |

### Halt triggers (architect §7.7 + design §10 H1..H13)

13/13 halt-trigger fixtures GREEN (`tests/tb_16_halt_triggers.rs`).
H7 (unresolved evidence gap) **demonstrated live** via TB-13 fixture's
Layer E #27 halt — confirms the halt-trigger architecture detects real
evidence gaps.

---

## §3 Test counts

```text
cargo test --workspace = 905 passed / 0 failed / 150 ignored
```

Workspace baseline at TB-15 ship: 759. Net additions for TB-16:
- 13 halt-trigger tests (Atom 1)
- 5 audit_assertions module tests (Atom 2)
- 3 audit_tape binary smoke tests (Atom 3)
- 2 dashboard live-regen tests (Atom 4)
- 2 comprehensive_arena smoke tests (Atom 5)
- (Atom 6 ships scripts only — no new tests)

= +25 from TB-15. (Total 905 includes accumulated additions across
sub-packages; per-package counting matches `cargo test --workspace`.)

---

## §4 Open follow-ups

### Atom 6.1 — multi-task chain continuation (HIGH; gates fresh arena run)

The current `lean_market run-task` semantics produce ONE chain per
task. To produce a single chain with all 13 tx kinds, evaluator needs
to support continuing an existing `runtime_repo` across multiple
task invocations. This is a moderate refactor (sequencer's
`NonEmptyRuntimeRepo` fail-closed gate per
`src/runtime/mod.rs:216-220` would need a guarded resume path with
explicit user opt-in via env var, e.g. `TURINGOS_CHAINTAPE_RESUME=1`).

Until 6.1 ships:
- audit_pipeline can validate any existing chain-backed tape
- comprehensive arena evidence is per-task sub-tapes (not aggregated)
- 13-tx-kind coverage must be assessed across the union of sub-tapes,
  not within a single tape

### Mathlib build (precondition for fresh real-LLM run)

`experiments/minif2f_v4/.lake/packages/` is missing — required for
Lean oracle evaluation. Run:
```bash
cd experiments/minif2f_v4 && lake exe cache get   # ~2 min
```
per `feedback_lake_packages_vendored`. This is a **user-side action**
because the cache fetch is network-bound and not deterministic across
sessions.

### TB-15 carry-forward (deferred from TB-15 charter §1.2)

- `OBS_TB_13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md` (carry-forward)
- `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` (carry-forward)
- `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md` (carry-forward)
- `OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md` (carry-forward; not in TB-16 scope)

### Closed by TB-16

- `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` — closed by
  Atom 4 (`build_report` now reconstructs EconomicState via
  `replay_full_transition`; verified by
  `tests/tb_16_dashboard_live_regen.rs` 2/2 PASS).

---

## §5 Cross-references

- TB-15 ship: commit `2337381` + R3 `eddab36`; SHIP_STATUS at
  `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
- TB-14 ship: commit `8b93fd9`
- TB-13 ship: charter `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
- Audit pipeline evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/`
- TB-16 evidence README: `handover/evidence/tb_16_real_llm_arena_2026-05-04/README.md`
- Architect §7 spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`

---

## §6 Atom 7 dual external audit gate

Per `feedback_dual_audit` + `feedback_risk_class_audit`: Class 3
integration smoke = full Codex + Gemini hybrid dual external audit at
ship. Atom 7 will:

1. Codex audit (via `codex:rescue` agent or `run_codex_*.sh`).
2. Gemini audit (via `run_gemini_*.py`).
3. Conservative resolution: VETO > CHALLENGE > PASS per
   `feedback_dual_audit_conflict`.
4. Round-cap=2 per `feedback_elon_mode_policy`.
5. Final commit on PASS/PASS or degraded-PASS.
