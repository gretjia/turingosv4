# TB-1 Day-4 h_vppu live evidence

**Date**: 2026-04-29
**Provenance**: 2 live evaluator runs of `mathd_algebra_107` (n3 mode, `deepseek-chat`, MAX_TRANSACTIONS=10), executed during TB-1 Day-4. Originally captured at `/tmp/tb1_day4_smoke_v2/`; migrated here per TB-1 Path A++ / Codex P1-2 (Codex audit `2026-04-29` flagged untracked `/tmp` evidence as not survivable).

**Source commit**: `50a1d67` (TB-1 Day-4 — P6 h_vppu_history instrumentation).

## Files

| File | Purpose |
|---|---|
| `run1.jsonl` | First evaluator JSONL row — `solved=true`, `h_vppu=null` (no prior history). |
| `run2.jsonl` | Second evaluator JSONL row — `solved=true`, `h_vppu=6.215891726697228` (current `pput_verified` divided by mean of prior history of size N=1). |
| `h_vppu_history.json` | Per-problem rolling history snapshot after both runs (capacity 3). |

## Why this matters

- **AT-3 live form** — TB-1 Tier-B AT-3 (`test_at3_h_vppu_non_null_on_second_run`) is registered in `tests/tb_1_acceptance.rs` as `#[ignore]` because the integration test cannot import the `experiments/minif2f_v4` crate. The live evidence ABOVE is the canonical proof that `make_pput → record/query/stamp via HVppuHistory` works end-to-end.
- **Spec divergence record** — `h_vppu` is stamped post-hoc inside `experiments/minif2f_v4/src/main.rs` (load → query → stamp → record → save), NOT inside `make_pput`. This was an intentional engineering call: `h_vppu` depends on history (I/O + side effect); keeping `make_pput` pure was preferred. See TB-1 Day-4 commit body for the rationale; flagged as "approved spec divergence" in the Path A++ ruling.

## Reproducibility

The original runs used `experiments/minif2f_v4/target/release/evaluator` invoked twice with identical flags; history file was reused across the two runs. To reproduce:

```sh
cd experiments/minif2f_v4
cargo build --release --bin evaluator
mkdir -p /tmp/tb1_day4_repro
HVPPU_HISTORY_PATH=/tmp/tb1_day4_repro/h_vppu_history.json \
  ./target/release/evaluator \
    --condition n3 --model deepseek-chat \
    --problem ../minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean \
    --max-transactions 10 \
  > /tmp/tb1_day4_repro/run1.jsonl 2>/tmp/tb1_day4_repro/run1.err
# Repeat with same HVPPU_HISTORY_PATH for run2; second row should carry h_vppu != None.
```

(Exact env names + flag set per `experiments/minif2f_v4/src/main.rs` Day-4 wire-up; check that file for current defaults if reproducing later.)

## Cross-references

- TB-1 recharter: `handover/tracer_bullets/TB-1_recharter_2026-04-29.md` Day-4
- TB-1 dual-audit verdict: `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md` (Codex P1-2)
- Day-4 commit: `50a1d67` (TB-1 Day-4 P6 h_vppu_history instrumentation)
- Tier-A AT-3 stub: `tests/tb_1_acceptance.rs` (`test_at3_h_vppu_non_null_on_second_run`)
