# SOFTWARE_3_0_VAL_CONSTITUTION audit — 2026-05-23

**Auditor**: clean-context auditor agent (read-only; report materialized
inline by orchestrator)
**Verdict**: NO-VIOLATION

## Audited commits

| Atom | SHA | PR |
|------|-----|----|
| S1 | 7130cf91 | #122 |
| S2 | 486adaa2 | #123 |
| S3 | 1d35058d | #124 |
| S4.1 | c2b6d954 | #125 |
| S4.2 | ac95ac12 | #126 |
| S5 | 32e30d97 | #127 |

## Checks

1. **Risk-class declaration**: PASS. Every PR title carries an explicit
   Class tag — `#122 (Class 2)`, `#123 (Class 2)`, `#124 (Class 2)`,
   `#125 (Class 2)`, `#126 (Class 0)`, `#127 (Class 0 + Class 1)`.
   Source: `git log --oneline 7130cf91^..32e30d97`.

2. **Restricted surface freeze**: PASS. Per-commit
   `git diff --name-only ${sha}^..${sha} | grep -E "^(src/state/typed_tx|src/state/sequencer|src/bus|src/bottom_white/cas/schema|constitution|genesis_payload)"`
   returns empty for all six audited shas. Confirmed `genesis_payload.toml`
   modification belongs to PR#121 (349342c4 polymarket) which is
   out-of-scope per directive.

3. **KILL criteria**: PASS.
   - `grep -nE "t_hash_|simple_hash" src/web/write.rs` → no matches
     (S1 stdout-as-truth removed).
   - `grep -nE "siliconflow_client" src/bin/turingos/cmd_*.rs src/bin/turingos.rs`
     → no matches (S4.1 rename complete).

4. **WEB-CLI kernel invariant**: PASS. `src/web/session_snapshot.rs:41`
   declares `const GRILL_SESSION_SNAPSHOT_SCHEMA_ID: &str = ...` as
   **module-private** (no `pub`/`pub(crate)`). The test
   `tests/constitution_web_cli_kernel_invariant.rs::web_layer_never_defines_capsule_schema_ids`
   (lines 132-138) only flags `pub const` or `pub(crate) const` SCHEMA_ID
   patterns — the private `const` is intentionally exempt and the commit
   message of 486adaa2 documents this rationale. Direct-LLM-call grep
   over `src/web/` returned no `chat_complete_blocking` /
   `chat_client::require_api_key` / `chat_client::chat_complete` hits in
   code (only one doc-comment mention in `src/web/market_view.rs:21`).

5. **Derived-view discipline**: PASS. `src/runtime/build_session_view.rs:91-107`
   reconstructs view from CAS; module-doc `src/web/session_snapshot.rs:8-17`
   explicitly states "derived view, NOT a truth source. ChainTape + CAS
   remain canonical". `load_latest_snapshot` at
   `src/web/session_snapshot.rs:197-227` scans per-session CAS for highest
   `logical_t` — no filesystem-side global pointer used as canonical input.

6. **Empty-as-error anti-pattern**: PASS.
   `src/runtime/build_session_view.rs:96-107` returns
   `Ok(BuildSessionView { current_status: BuildStatus::SpecPending, .. })`
   for missing CAS dir. `src/web/write.rs:365-376` returns
   `StatusCode::BAD_GATEWAY` (502) with `kind: "task_id_parse_failed"` on
   stdout parse failure — no 200-with-warning.

7. **Money path integrity**: PASS. Per-commit diff filter
   `git diff ${sha}^..${sha} -- 'src/**/*.rs' | grep -E "^\+" | grep -E "\bf(32|64)\b"`
   is empty for all six audited shas. The `derive_price_yes(...) -> f64`
   and `num/den as f64` lines in `src/web/market_view.rs:346` are
   attributable to PR#121 (349342c4 polymarket) which is out-of-scope.
   No new memory-only canonical state and no new dashboard-only source of
   truth in the six audited commits.

8. **Audit-witness boundary**: PASS.
   `grep -n "audit_legacy_bypass" scripts/run_constitution_gates.sh` →
   no matches. No wiring in `scripts/`, `Makefile`, `.github/`, or
   `tests/` other than the script's own self-reference at
   `scripts/audit_legacy_bypass.sh:22,23,52`. The script remains a
   standalone reporting baseline.

9. **No retroactive evidence rewrite**: PASS. Per-commit
   `git diff --name-only ${sha}^..${sha} -- 'handover/evidence/*'` is
   empty for all six shas.

10. **FC-trace presence**: PASS.
    - `7130cf91 (S1)`: commit body contains `FC-trace: FC1-N10
      (TaskOpen acceptance edge)`.
    - `486adaa2 (S2)`: commit body contains `FC-trace: FC1-N12 ...,
      FC3-replay`.
    - `1d35058d (S3)`: commit body contains `FC-trace: FC2-N16 ..., FC3`.
    - `c2b6d954 (S4.1)`: pure rename refactor; renamed file
      `src/bin/turingos/chat_client.rs` preserves `TRACE_MATRIX FC2-N16`
      code comments throughout (lines 1, 21, 24, 28, 32, ...). Directive
      accepts code-comment references; PASS.
    - `ac95ac12 (S4.2)`: Class 0 docs — exempt per directive.
    - `32e30d97 (S5)`: Class 0/1 docs+script — exempt per directive.

## Final verdict

NO-VIOLATION
