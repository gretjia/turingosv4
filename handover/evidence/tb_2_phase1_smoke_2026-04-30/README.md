# TB-2 Phase-1 smoke evidence — 2026-04-30

**Branch**: `experiment/tb2-sequencer-runtime-closure`
**HEAD**: `cf32735` (Atom 6 + I13 — replay invariant + 16/16 battery green)
**Pre-audit gate**: live-LLM smoke verifies the Atom 2–6 changes did not break the experiment harness.

## Configuration

| Param | Value |
|---|---|
| Binary | `./target/debug/evaluator` (built from worktree) |
| Problem | `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean` |
| Condition | `oneshot` |
| Mode | `full` |
| Model | `deepseek-chat` (via `LLM_PROXY_URL=http://localhost:8080`) |
| `MAX_TRANSACTIONS` | 5 (one tx actually consumed: `tx_count=1`, `budget_max_transactions=1`) |

## Result

```json
{
  "schema_version": "v2.0",
  "solved": false,
  "verified": false,
  "progress": 0,
  "total_run_token_count": 140,
  "tx_count": 1,
  "prompt_context_hash": "a1f43584a17d1226",
  "model_snapshot": "deepseek-chat",
  "mode": "full",
  "condition": "oneshot",
  "failed_branch_count": 1
}
```

Full log: `oneshot_run.log` (this directory).

## Interpretation

- **PASS — pipeline-liveness**: evaluator built, ran end-to-end, emitted a v2.0 PPUT row, exited cleanly. Atom 2–6 changes to `src/state/sequencer.rs` + `src/state/typed_tx.rs` + `src/bottom_white/ledger/transition_ledger.rs` did NOT break the experiment harness.

- **`prompt_context_hash = a1f43584a17d1226` is bit-identical to TB-1 Day-1 spike** (`handover/tracer_bullets/TB-1_day1_spike_2026-04-29.md`). Identical prompt-context bytes mean the prompt-build path is unperturbed by my edits — the evaluator's PputResult struct, run_oneshot path, and all upstream prompt assembly are untouched. This is a stronger signal than just "no crash".

- **`solved=false` is expected**: the `oneshot` condition has a known prompt-template regression at HEAD per `handover/evidence/first_v4_solve_2026-04-29/README.md` (independent of TB-2). The Day-1 spike documented the same outcome under the same configuration. The TB-2 smoke is testing pipeline integrity, not solve correctness.

- **The harness still does NOT route through the new TB-2 runtime spine.** TB-1's narrowed claim explicitly deferred runtime dispatch enforcement to TB-2; the evaluator's PutResult emit path remains pre-runtime. TB-2's 16/16 acceptance battery proves the sequencer + L4 + L4.E spine works END-TO-END, but the evaluator hasn't been re-routed onto it yet (that's a future TB — likely P2 Agent Runtime). The smoke confirms the harness is still functional alongside the new spine, not that the spine is reaching the harness.

## Comparison vs TB-1 Day-1 spike (2026-04-29)

| Metric | TB-1 Day-1 (`f0b659f`-class) | TB-2 Phase-1 (`cf32735`) | Notes |
|---|---|---|---|
| `prompt_context_hash` | `a1f43584a17d1226` | `a1f43584a17d1226` | **Identical** — prompt-build path unperturbed. |
| `schema_version` | `v2.0` | `v2.0` | v2 dispatch contract preserved. |
| `solved` | `false` | `false` | Same documented oneshot regression. |
| `condition` | `oneshot` | `oneshot` | |
| `model_snapshot` | `deepseek-chat` | `deepseek-chat` | |
| `mode` | `full` | `full` | |

## Boot + workspace tests (separately verified)

- `cargo test --workspace`: all suites green, including `boot::verify_trust_root_*` after R-014 manifest rehash.
- `cargo test --test tb_2_runtime_boundary`: 13/13 PASS (I1–I13).
- `cargo test --lib state::sequencer`: 9/9 PASS (U1–U3 + 6 pre-existing).

## Next

Phase-1c diff dual audit on the experiment branch (vs `f9ace5e`).
