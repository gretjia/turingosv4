# Step-B Phase 0 — Art. II.1 Fix v3 (addressing Codex VETO on v2)

## What changed vs v2

### v2 → v3 fixes per Codex per-Q:

**Q3 bounded taxonomy**: enum is now **pure; NO String carriers**. 8 fixed variants:

```rust
pub enum OracleErrClass {
    // TACTIC family: bounded known-tactic list (if unknown tactic → Other)
    TacticLinarith,        // "linarith failed"
    TacticSimpNoProgress,  // "simp made no progress"
    TacticRingFailed,
    TacticNormNum,
    TacticOther,           // any tactic-failed we don't specifically track
    // STRUCTURAL
    UnknownConstant,       // "unknown constant `X`"
    UnsolvedGoals,
    UnexpectedToken,       // parse-fail equivalent
    TypeMismatch,
    RewriteNoMatch,
    Heartbeat,             // maxHeartbeats exceeded
    Other,                 // catchall (NEVER String-carrying)
}
```

Top-K broadcast renders as:
```
"err:tactic_linarith(8), err:unknown_const(5), err:rewrite_no_match(3)"
```
Label space is **finite and fixed at the type level**. No drift.

### Q7 freeze contract

- `const CLASSIFIER_VERSION: &str = "v1_2026-04-16-a";` baked in classifier.rs
- Artifact (jsonl) rows carry `classifier_version: "v1_2026-04-16-a"`
- Fixture corpus: `experiments/minif2f_v4/analysis/classifier_fixtures.txt` (expected class per line; real Lean outputs from v31/v32 stderr)
- Fixture test = new `#[test]` in classifier.rs; panics if any miscategorized
- **Pre-A/B smoke** runs classifier on all fixtures; abort batch if any mismatch
- TopKClasses: k=3 fixed; sort by count DESC, tiebreak by enum variant ordinal ASC

### Q8 confound controls

| Source | Control | v3 action |
|---|---|---|
| Model family | Same (deepseek-chat) | ✓ |
| Prompt/schema | Same | ✓ |
| Timeout | Same | ✓ |
| Boltzmann RNG | `rand::thread_rng()` | **Pin to `StdRng::seed_from_u64(74677)`** — same seed both arms |
| LLM sampling (temp=0.2) | **NOT fully controllable** | Documented residual; use paired-test logic (same problem, both arms) to absorb |
| Proxy URL | env (same on both) | ✓ |
| Retry policy | Same code | ✓ |
| Treatment scope | bus.rs + evaluator.rs + new classifier.rs | Bundle as single commit SHA; document in metrics.yaml |

Non-LLM determinism IS now reproducible. LLM stochasticity remains but affects both arms equally; paired-problem comparison (same 50 problems on both sides) reduces it to near zero for primary verdict.

## Treatment definition (precise)

- **Control**: commit `e58e021` (v3.1 M4) — main HEAD with existing bus.rs
- **Treatment**: `experiment/art-ii1-v3` branch built from `e58e021`, containing:
  - `src/bus.rs` (add `RejectionScope` + `recent_rejections_scoped`)
  - `src/sdk/error_abstraction.rs` (new module)
  - `src/sdk/mod.rs` (register the new module)
  - `experiments/minif2f_v4/src/bin/evaluator.rs` (ingest OMEGA/parse errors + use new API)
  - `experiments/minif2f_v4/analysis/classifier_fixtures.txt` (fixture corpus)
- **Both arms use**: `deepseek-chat`, `sample_N50_S74677.txt`, same prompt, Boltzmann seed=74677

Classifier identical across arms (same code, same commit — treatment adds it; control doesn't use it at all, bus.rs returns empty Vec as today).

## A/B run protocol

1. Checkout worktree `experiment/art-ii1-v3`
2. Commit v3 implementation (tag as treatment SHA)
3. Run classifier fixture tests (`cargo test classifier`) — must pass
4. Smoke probe on fixture corpus before batch start
5. Treatment batch: `ACTIVE_MODEL=deepseek-chat ./run_interleaved.sh` on worktree
6. Control batch: `ACTIVE_MODEL=deepseek-chat ./run_interleaved.sh` on main (v3.2 already exists — reuse if metrics.yaml matches; else re-run)

Actually reuse: **v3.2 resume-in-progress IS the control** — it's chat on main bus.rs. No need to re-run control. Apples-to-apples.

## Decision rule (from metrics.yaml)

Primary: SolveRate pairwise per condition.
- Treatment WIN: `ΔSolveRate(treatment − control) ≥ 3` on n3 AND no regression ≥ 2 on oneshot/n1
- Treatment LOSS: n3 regresses ≥ 2 OR n1 regresses ≥ 2
- GRAY: otherwise

Merge criterion: WIN required.

## Budget

- Treatment batch: ~3h (same as v3.2 on chat)
- Control already exists (v3.2) — ~$0 more
- Total: 1× batch cost = ~$15-20, ~3h wall

## Re-audit request

Per-Q:
- Q3 (bounded): pure enum, no Strings. Sufficient abstraction per Art. II.1 "抽象出来"?
- Q7 (freeze): classifier version + fixtures + pre-A/B smoke + per-row version stamp. Sufficient for C-012?
- Q8 (confounds): Boltzmann seed pinned; LLM stochasticity absorbed via paired comparison. Residual confounds acceptable?
- NEW: is "reuse v3.2 resume as control" valid? Same code, same model, same prompt, same analyzer — only missing: we're calling an in-progress batch "control" mid-flight. Alternative: wait for v3.2 resume to finish, freeze it, then run treatment.

Final verdict: PROCEED to Phase 1 / HOLD / VETO.
