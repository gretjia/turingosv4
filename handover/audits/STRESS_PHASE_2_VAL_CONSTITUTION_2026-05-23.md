# STRESS_PHASE_2_VAL_CONSTITUTION audit — 2026-05-23

**Auditor**: clean-context auditor (read-only; report materialized inline by orchestrator)
**Verdict**: NO-VIOLATION

## Audited commits

| SHA | Atom | PR |
|-----|------|----|
| 22812db8 | STRESS-0 (charter + §8 + 10 runners) | #129 |
| 1ea99a2d | STRESS-1..10 (execution evidence) | #131 |

## Checks

1. **Risk-class declaration**: PASS. `22812db8 "(Class 0+1)"` and
   `1ea99a2d "(Class 1+2)"` both state risk class in PR titles.

2. **Restricted surface freeze**: PASS.
   `git diff --name-only 22812db8^..1ea99a2d | grep -E "^(src/state/typed_tx|src/state/sequencer|src/bus|src/bottom_white/cas/schema|constitution\.md|genesis_payload\.toml|src/runtime/mod\.rs)$"` → empty.
   All `genesis_payload.toml` hits in the diff are evidence-internal
   workspace copies (e.g. `handover/evidence/stress_st02_concurrent_writers_20260523T141526Z/workspace/genesis_payload.toml`),
   not the repo-root file.

3. **Money path integrity**: PASS.
   `git diff 22812db8^..1ea99a2d -- 'src/**/*.rs'` is empty; no src/ Rust
   files were touched.

4. **No retroactive evidence rewrite**: PASS. Non-`stress_st*` evidence
   diff is exactly one file (`handover/evidence/.gitignore`, newly
   created, additive only with two ignore patterns for `_helper/target/`
   and `cas/.git/objects/`). No edits to pre-existing evidence dirs.

5. **Audit witness boundary**: PASS. `scripts/audit_legacy_bypass.sh`
   and `scripts/run_constitution_gates.sh` are NOT in the diff; all
   scripts changes are confined to `scripts/stress/` (10 runners +
   `_common.py` + `_mock_llm_server.py` + `_ws_bootstrap.sh`).

6. **Source-code separation (charter §2)**: PASS.
   `git diff --name-only 22812db8^..1ea99a2d | grep '^src/'` → empty.
   No production source file was modified.

7. **Evidence `summary.md` KILL line**: PASS. Sampled 23
   `stress_st*/summary.md` files; every one ends with `## KILL`
   followed by a single `PASS` or `FAIL` token (machine-readable).
   ST-01 134030Z=PASS, ST-04 141832Z=FAIL, ST-10 final=PASS (sample of 3
   confirmed; full set 8 PASS, 0 FAIL final-per-test after retries; ST-04
   and ST-08 PARTIAL noted in ship-report §3).

8. **ST-04 PARTIAL attribution**: PASS. Ship report §3 attributes the
   PARTIAL to a workspace-bootstrap dependency (missing
   `PromptPromotionReceipt`) and asserts S2's `write_snapshot` is
   correct. Independent evidence confirms:
   `handover/evidence/stress_st04_snapshot_restart_storm_20260523T141832Z/workspace/sessions/probe_st04b/cas/.turingos_cas_index.jsonl`
   and `.../st04_session_2e6cc5f4/cas/.turingos_cas_index.jsonl` each
   contain exactly one entry with
   `"schema_id":"turingos-web-grill-session-snapshot-v1"`,
   `"creator":"web_grill_session_snapshot"`, `"size_bytes":418`.
   The WRITE half is on tape; the LOAD half blocked by promotion-guard
   fail-closed (correct production behavior). Attribution is consistent
   with evidence.

## Final verdict

NO-VIOLATION
