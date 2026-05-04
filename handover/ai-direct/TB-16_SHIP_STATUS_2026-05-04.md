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
| FR-16.1 | At least 3 agents participate | ✓ Sandbox preseed defines 8 sandbox-prefixed agents; arena_run4 exercises Agent_solver_0/Agent_verifier_0/Agent_3/Agent_user_0 |
| FR-16.2 | At least one WorkTx creates FirstLongPosition | ✓ arena_run4 (commit `d1c1af2`): WorkTx accepted, FirstLong NodePosition created |
| FR-16.3 | At least one ChallengeTx creates ShortPosition | ✓ arena_run4: Agent_3 ChallengeTx accepted; FR-16.3 env-var trigger added in commit `05e3e86` (`TURINGOS_FORCE_CHALLENGER`) |
| FR-16.4 | At least one CompleteSetMintTx exists | ✓ arena_run4: Agent_user_0 MarketSeedTx + CompleteSetMintTx; FR-16.4 env-var trigger `TURINGOS_COMPLETE_SET_SEED` |
| FR-16.5 | At least one price update occurs | ✓ arena_run4: PriceIndex view derives from chain at audit time (verified by Layer C) |
| FR-16.6 | At least one Boltzmann mask event occurs | ✓ arena_run4: Boltzmann mask computed; verified Layer C |
| FR-16.7 | At least one AutopsyCapsule is generated | ⚠ arena_run6_exhaust: TaskBankruptcyTx fired but no autopsy emitted because no agent had stake (no accepted WorkTx in exhaust run); needs chain with BOTH accepted WorkTx + subsequent TaskBankruptcy on same task — single-arena env-var combo cannot achieve this without multi-task chain continuation (Atom 6.1). FR-16.7 env-var trigger `TURINGOS_FORCE_BANKRUPTCY` is wired |

**Step 4 reality (commit `d1c1af2`)**: 9 of 13 architect-required tx
kinds delivered across 2 chains (`arena_run4` happy: 7 tx kinds;
`arena_run6_exhaust`: 4 tx kinds incl. TaskBankruptcy). Missing in
both runs: ChallengeResolve (system-emit not wired), FinalizeReward
(challenge blocks Finalize per challenge-window semantic), TaskExpire
(no env-var trigger), CompleteSetRedeem (post-resolution path not
wired). Fresh arena run with all 13 tx kinds in a single chain
remains gated on Atom 6.1 (multi-task chain continuation).

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
command         = cargo test --workspace --no-fail-fast
workspace_count = 907 passed
failed          = 0
ignored         = 150
```

Per `feedback_workspace_test_canonical`: `cargo test --workspace` is
the canonical ship-gate test count (mandated 2026-05-01 D4).

**Test-count math** (TB-15 R3 ship 882 → TB-16 SHIPPED 907 = +25):
| Source | Δ | Notes |
|---|---|---|
| TB-15 R3 final ship | 882 | commit `eddab36` |
| TB-16 Atom 1 halt-trigger fixture | +13 | `tests/tb_16_halt_triggers.rs` (H1..H13) |
| TB-16 Atom 2 audit_assertions module tests | +5 | inline tests in `src/runtime/audit_assertions.rs` |
| TB-16 Atom 3 audit_tape binary tests | +3 | smoke + tamper integration tests |
| TB-16 Atom 4 dashboard live-regen tests | +2 | `tests/tb_16_dashboard_live_regen.rs` |
| TB-16 Atom 5 comprehensive_arena tests | +2 | scaffold smoke tests |
| TB-16 Atoms 6+7 (scripts + audit-only) | 0 | runner scripts; no new test code |
| **TB-16 SHIPPED (pre-audit)** | **905** | commit `3300fe2` |
| TB-16 Atom 7 R1 Step 1 surgical fixes | +2 | sandbox-prefix-canonical + tape-fence tests |
| **TB-16 Atom 7 R1** | **907** | commit `3cf4c36` |
| TB-16 Atom 7 R3 surgical fixes | 0 | new audit assertions exercised by binary, not unit-test layer |
| **TB-16 Atom 7 R3** | **907** | this commit |

The R3 supplementals (`assert_d_total_supply_conserved_per_block`,
`assert_a_chain_agent_ids_sandbox_prefixed`) are exercised
end-to-end by the `audit_pipeline_smoke` fixture (verdict.json) and
do not add `#[cfg(test)]` cases.

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

---

## §7 Atom 7 audit cycle log

| Round | Codex | Gemini | Conservative | Closure |
|---|---|---|---|---|
| R1 | VETO × 5 (V2 sandbox-canonical, V3 audit_pipeline_smoke fixture, V4 BLOCK exit, V5 destructive tamper, V6 conservation drift, V7 Markov chain) | VETO × 1 (Q11 TRACE_MATRIX precision) + CHALLENGE | VETO | Step 1 commit `3cf4c36` (V2-audit/V3/V4/V5/V6/V7 + Q11) + Step 3 commit `05e3e86` (env-var triggers) + Step 4 commit `d1c1af2` (fresh arena runs + TB-11 writer-pattern bug fix) |
| R2 | NOT YET RUN | VETO × 5 (4/5 stale: Q5/Q8/Q9 audit predates Step 4 commit; 1 real Q2 JSON byte-run) + CHALLENGE × 5 | VETO | R3 prep this commit: Q2 (JSON-array decimal byte-run check on assertion #28), Q1 (Layer D #18b per-block conservation walker, id=40), Q10 (Layer A chain-walk sandbox-prefix walker, id=41), Q11 (file-level TRACE_MATRIX doc precision), Q12 (this §3 doc), Q8 evidence regen (smoke MARKOV regenerated chained to TB-15 head) |
| R3 | TBD (run with `TB16_AUDIT_ROUND=R3`) | TBD (run with `TB16_AUDIT_ROUND=R3`) | TBD | — |

### R3 surgical fixes (this commit)

| ID | Fix | Surface | LoC |
|---|---|---|---|
| Q2 (Gemini R2 VETO) | Mirror TB-15 halt-trigger #5: `assert_28_projection_no_autopsy_bytes` now checks BOTH (a) raw 32-byte run in canonical_encode AND (b) JSON-array decimal text form via `serde_json::to_string` | `src/runtime/audit_assertions.rs` | ~30 |
| Q1 (Gemini R2 CHALLENGE) | Layer D supplemental `assert_d_total_supply_conserved_per_block` (id=40): incrementally walks L4, replays `entries[..=i]` for every i, asserts `total_supply_micro == initial` at every step. O(N²) tolerable for our chain sizes | `src/runtime/audit_assertions.rs` | ~55 |
| Q10 (Gemini R2 CHALLENGE) | Layer A supplemental `assert_a_chain_agent_ids_sandbox_prefixed` (id=41): walks every L4 entry, decodes TypedTx, asserts `HasSubmitter::submitter_id()` (when Some) satisfies `sandbox_prefix`. Closes machine-verifiable CR-16.7 gap | `src/runtime/audit_assertions.rs` | ~50 |
| Q11 (Gemini R2 CHALLENGE) | File-level TRACE_MATRIX doc-comment now precise per-layer (Layers A-G + supplementals → FC1-N34; Layer H tamper stubs → FC1-N35; verdict.json → FC2-N31) | `src/runtime/audit_assertions.rs` | ~13 |
| Q12 (Gemini R2 CHALLENGE) | Test-count math table (above) shows per-step delta from TB-15 R3 ship through TB-16 R3; canonical `cargo test --workspace` count | this doc §3 | ~20 |
| Q8 evidence regen (closure) | `audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json` regenerated with `--prev-cid-hex $(cat handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt)` so chain link is in artifact, not just in runner script. Chains to TB-15 head `f9e701b4...` | `handover/evidence/.../audit_pipeline_smoke/` | regen |

### Q4 position held (carry-forward to architect ratification)

R2 Q4 ("non-sandbox funds used" — sandbox HALT vs banner)
**position held**: architect §7.7's "non-sandbox funds used" HALT is
parallel-structurally an **audit-time** detection (Layer A #3 for
manifest + new id=41 for chain), NOT a sequencer-level admission gate.
Reading it as runtime gate would modify hot-path `submit_agent_tx`
(Class 3+/4) and exceeds architect spec. Recommendation: keep
audit-time HALT; if architect ratification of stronger guarantee
desired, escalate to charter §5.x amendment.

### Audit pipeline smoke evidence (post R3)

```text
verdict.json:        PROCEED  passed=38  failed=0  halted=0  skipped=3
verdict_replay.json: byte-identical
tamper_report.json:  detected_count=3/3
MARKOV_TB-16_*.json: capsule_id=737b4d22..., previous_capsule_cid=f9e701b4... (TB-15 head)
```

Skipped 3 = Layer H tamper stubs (assertions #36/#37/#38; per-fn
doc-comment binds them to FC1-N35; actual tamper detection happens
in `audit_tape_tamper` binary which reports 3/3 detected).
