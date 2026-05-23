# STRESS_PHASE_2_VAL_KARPATHY audit — 2026-05-23

**Auditor**: clean-context Karpathy-discipline witness
**Verdict**: PASS

## K-checks

- **K10 defer abstraction**: PASS.
  - `scripts/stress/_common.py` is 85 LOC of flat functions (`ts_utc`,
    `evidence_dir`, `write_summary`, `sha256_file`, `run_cmd`,
    `cargo_bin_path`, `ensure_built`). No `StressRunner` class, no plugin
    loader, no decorator framework.
  - Each `st0*.py` is self-contained ~120-230 LOC with module-level
    `def main()` at the bottom. No inheritance from a base test class.
  - `grep -rn "enum ChatProvider\|ModelCallReceipt" src/` → 0 hits.

- **K11 direct computation; minimal abstractions**: PASS.
  - `_mock_llm_server.py` is 83 LOC: a single
    `MockHandler(BaseHTTPRequestHandler)` driven by env vars
    (`MOCK_FAIL_RATE`, `MOCK_LATENCY_MS`, `MOCK_RESPONSE_BODY`,
    `MOCK_MAX_TOKENS`, `MOCK_SEED`). No factory/strategy/plugin layer.
  - The only other handler class in `scripts/stress/` is
    `TruncatedHandler` in `st09` — a second concrete handler with
    distinct behavior (truncated-body), not a redundant abstraction.

- **K14 no escape hatches**: PASS.
  - **ST-04 PARTIAL**: not papered over. Evidence dir's `summary.md`
    ends with `KILL: FAIL`; ship report §3 documents the finding openly
    (triage promotion guard left intact; classified as workspace
    bootstrap dependency, not a production defect).
  - **ST-08 NOT-EXECUTED**: `scripts/stress/st08_long_grill_drift.py`
    exists at full 233 LOC with no `skip` / `SKIP` / feature-flag /
    return-0-defer stub (grep clean). The deferral lives in the ship
    report, not in a stub-out of the runner.

- **Ceremony-free per commit**: PASS.
  - `22812db8` touches only `handover/{tracer_bullets,directives}/` +
    `scripts/stress/`.
  - `1ea99a2d` touches only `handover/evidence/` + `scripts/stress/`
    (the latter for the runner-robustness fixes called out in the commit
    message: schema / workspace bootstrap / port handling).

- **Charter discipline (no src/ touch)**: PASS.
  `git diff --name-only 22812db8^..1ea99a2d -- 'src/' 'tests/'` → empty.
  Charter §2 ("stress tests are observation, not implementation") held.

- **Real-tape evidence**: PASS. Spot-checked
  `stress_st01_gittape_sigkill_20260523T134030Z` (summary 276 B,
  `KILL: PASS`, real workspace/ subdir),
  `stress_st04_snapshot_restart_storm_20260523T141832Z` (summary 14 lines +
  `server_cycle1.log` captured, `KILL: FAIL` truthfully),
  `stress_st10_double_backend_20260523T134346Z` (summary present +
  `_helper/` Cargo crate, `KILL: PASS`). All summaries non-empty and end
  with machine-greppable KILL line.

## Final verdict

PASS
