# Codex TB-16.x.2.4.fix R2 Audit (commit 4dd82c1)

**Round**: R2
**Date**: 2026-05-05
**Audit target**: commit `4dd82c123fd466eb9441fafeef9eda89b49df5e2` (TB-16.x.2.4.fix r1)
**Auditor**: Codex via codex:codex-rescue subagent (impl-paranoid angle)
**Test baseline**: `cargo test --workspace` = 922 PASS / 0 FAILED / 150 ignored (+7 from id=43 unit tests vs 915 R1 baseline)

## OVERALL VERDICT

**CHALLENGE** (conviction: high; recommendation: SHIP CLEAN)

Justification: All four R1 VETOs are closed in code, and CH1/CH2/CH4 are closed. The only live challenge is documentation/test-evidence drift: the r4 README mismatched `verdict.json` on `cas_object_count` (claimed 25, actual 23), plus stale comments around the old fallback/threshold. The runtime hook is fail-closed, entropy is computed over non-None parents with a 0.5 gate, commit-confirmed IDs are ordered correctly, and no production defect from `.fix r1` was observed.

---

## Per-Finding Verdicts

### VETO #1 closure (id=43 entropy non-None filter)
- **Verdict**: PASS
- **Evidence**: `src/runtime/audit_assertions.rs:1941-1942` quotes `let non_none: Vec<&crate::state::q_state::TxId> = parents.iter().filter_map(|p| p.as_ref()).collect();`; `:1999-2000` quotes `const SHIP_GATE_ENTROPY_BITS: f64 = 0.5; if h < SHIP_GATE_ENTROPY_BITS {`.
- **Reasoning**: Entropy input is now the `Some(tx_id)` subset only; Shannon entropy computed over counts from that subset at `:1951-1961`. ROOT/None false-diversity defect is closed. Added pure-fn tests cover the star-topology reproducer + threshold cases at `:2761-2839`.

### VETO #2 closure (produced_worktx_ids.push in Ok arm)
- **Verdict**: PASS
- **Evidence**: `experiments/minif2f_v4/src/bin/evaluator.rs:1547-1559` quotes `match ... tb8_await_state_root_advance(...).await { Ok(_) => { ... produced_worktx_ids.push(wt_id); }`; `:1561-1569` quotes `Err(_) => { error!(...); std::process::exit(3); }`.
- **Reasoning**: `push` is inside the post-submit commit-confirmation `Ok(_)` arm. A submit that queues but does not commit cannot populate `produced_worktx_ids`.

### VETO #3 closure (smoke harness fail-closed gaps)
- **Verdict**: PASS
- **Evidence**: `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh:36` quotes `set -euo pipefail`; `:83-86` refuses existing `verdict.json` unless `ALLOW_REUSE=1`; `:104-126` captures evaluator RC with `|| RC=$?` and exits nonzero; `:140-148` captures `audit_tape` output into `AT_LOG=$("$AUDIT_TAPE_BIN" ... 2>&1)`.
- **Reasoning**: Cited gaps closed for the named failure modes: evaluator nonzero aborts, audit_tape nonzero aborts under `set -e`, stale verdict reuse refused, audit_tape no longer loses RC through `tail`.

### VETO #4 closure (warn-and-continue runtime critical paths)
- **Verdict**: PASS
- **Evidence**: Env parse exits at `evaluator.rs:1272-1300`; preseed/iter `q_snapshot` failures exit at `:1338-1345` and `:1381-1389`; CAS open exits at `:1441-1449`; ProposalTelemetry build/write exits at `:1451-1489`; WorkTx construction/non-Work exits at `:1498-1530`; submit exits at `:1532-1538`; commit-await timeout exits at `:1547-1569`.
- **Reasoning**: FORCE_BOLTZMANN hook's critical paths are now `error!` plus `std::process::exit(3)`. Remaining `warn!` sites in `evaluator.rs` are outside this .2.4 hook.

### CHALLENGE #1 closure (STEP_B deviation explicit position)
- **Verdict**: PASS
- **Evidence**: `evaluator.rs:1251-1261` quotes the architectural note: STEP_B's sequencer-side interpretation is not used, `parent_tx` lives in ProposalTelemetry, and "No sequencer.rs touch needed"; `src/state/typed_tx.rs:223-236` shows `WorkTx` has `proposal_cid` but no `parent_tx` field; README repeats the position at `handover/evidence/.../README.md:90`.
- **Reasoning**: Acceptable. For TB-16.x.2.4's ship gate, the measured surface is ProposalTelemetry parent selection, not sequencer enforcement. A sequencer admission gate would require schema/admission semantics beyond this atom. The TB-17 prerequisite claim is unverified from file:line, but the explicit .2.4 position is file-backed.

### CHALLENGE #2 closure (pre-iter await wrong side)
- **Verdict**: PASS
- **Evidence**: `evaluator.rs:1333-1369` adds one pre-loop settle barrier; `:1371-1381` enters the loop and only snapshots `post_root`; `:1547-1551` calls `tb8_await_state_root_advance` after `bus.submit_typed_tx` at `:1532`.
- **Reasoning**: Per-iteration pre-submit await removed. The only per-iteration await is post-submit, matching the helper contract quoted in `src/runtime/adapter.rs:570-576`.

### CHALLENGE #3 closure (README mismatched verdict)
- **Verdict**: CHALLENGE
- **Evidence**: README claims `tape_root.cas_object_count = 25` at `handover/evidence/.../README.md:64` and claims the section reads r4 verdict "verbatim" at `:92`; actual `verdict.json` says `"cas_object_count": 23` at `:8`.
- **Reasoning**: Prior `l4e_count` mismatch is fixed (`README.md:63` matches `verdict.json:5`), and work/id43 match (`README.md:65-69`, `verdict.json:12`, `:239-243`). The CAS count remains mismatched. Evidence documentation only, not production behavior. **Closed in .fix r2**: `README.md` updated to read `cas_object_count = 23` with annotation citing this CH3-residual.

### CHALLENGE #4 closure (fallback parent_tx)
- **Verdict**: PASS
- **Evidence**: `evaluator.rs:1402-1419` documents fallback removal and quotes `let parent_tx = v2_pick.clone();`.
- **Reasoning**: The old `produced_worktx_ids.last()` fallback is gone. `parent_tx` is now exactly the selector output or `None`.

---

## R2 Supplemental Checks

### Preseed-settle barrier (50×200ms budget + FAIL-CLOSED)
- **Verdict**: PASS
- **Evidence**: `evaluator.rs:1337-1353` polls up to `50u32` with `Duration::from_millis(200)`; `:1355-1362` exits 3 if not settled; `:1547-1569` exits 3 if first WorkTx submit succeeds but commit not observed.
- **Reasoning**: 10s active-drain budget is reasonable. A continuously advancing queue triggers the barrier's exit path; a non-advancing wedged queue can pass the stability check, but the post-submit commit await still fail-closes the hook. No silent proceed observed.

### proposal_index uniqueness inline rationale (Gemini Q5)
- **Verdict**: PASS
- **Evidence**: `evaluator.rs:1429-1440` documents `.2.4` as `5 + iter_i`; `.2.5` uses `4u64` at `evaluator.rs:1145-1158`; ProposalTelemetry hashes `(run_id, agent_id, proposal_index)` at `src/runtime/proposal_telemetry.rs:229-234` and branch_id is `agent_id.b{proposal_index}` at `:254`; swarm agents are `Agent_0..` at `evaluator.rs:1724`.
- **Reasoning**: Correct under current execution order and agent namespace.

### BOLTZMANN_SEED=12345 in smoke env vs binary default
- **Verdict**: PASS
- **Evidence**: Hook reads env seed or falls back to `0xB01_72A_4_u64` at `evaluator.rs:1306-1309`; smoke script documents seed `12345` choice at `run_tb_16_x_2_4_smoke_2026-05-05.sh:64-75` and passes it at `:105-108`; README records env at `:20`.
- **Reasoning**: Defensible. Canonical smoke configuration, not a binary default change. Determinism preserved because seed is explicit in script/evidence.

### STEP_B deviation explicit-position closure (Codex CH1)
- **Verdict**: PASS — for .2.4's scope.
- **Position**: acceptable
- **Reasoning**: For .2.4, OBSERVE via CAS-backed ProposalTelemetry is sufficient. ENFORCE would be a separate sequencer/schema change, not a hidden requirement for this atom.

---

## NEW Defects Introduced by .fix r1

- **Production code**: None observed.
- **Documentation/evidence defects**:
  - README CAS count mismatch (`README.md:64` says 25, `verdict.json:8` says 23) — **CLOSED in .fix r2**.
  - Stale comments mentioning removed fallback / old 0.25 threshold at `evaluator.rs:1241-1249` and `run_tb_16_x_2_4_smoke_2026-05-05.sh:7-16` — **deferred to .fix r2 cleanup pass** (not blocking; doc drift only).

---

## Round cap

R2 of .fix cycle (per `feedback_elon_mode_policy` round-cap=2; this is FINAL). Per `feedback_audit_loop_roi_flip` ROI-flip stop-rule: only doc/evidence drift remains; iteration ROI has flipped to test-scaffold edges. Ship clean.

## Cross-reference

- R1 audit: `handover/audits/CODEX_TB_16_X_2_4_AUDIT_2026-05-05_R1.md` (parent commit `b5118fd`; OVERALL VERDICT VETO; 4 VETO + 4 CHALLENGE + 5 PASS)
- Paired Gemini R2 audit: `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R2.md` (OVERALL VETO on Q1+Q2 architectural enforcement gap; defers via OBS_R024)
- OBS filed for Gemini R2 Q1+Q2: `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`
