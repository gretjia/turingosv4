# M0 batch 2026-05-10 — 4/20 ERROR triage

**Investigated**: 2026-05-10 session #34 (post L4.E body integrity landing).
**Triaged by**: Claude (autonomous; Class 1-2; no money spend; no production change).
**Class of finding**: OPERATIONAL workflow concurrency. NOT a TuringOS bug.

## §1 — Identification

The 4 ERROR problems from `M0_BATCH_SUMMARY.json` (`audit_verdict.error: 4`) are the
last 4 problems by alphabetical / numerical order:

| Slot | Problem | verdict.json | runtime_repo/ contents |
|------|---------|--------------|------------------------|
| P17  | `imo_1962_p2`                                                      | absent | empty (no `pinned_pubkeys.json`, no genesis, no JSONL) |
| P18  | `induction_11div10tonmn1ton`                                       | absent | empty |
| P19  | `induction_12dvd4expnp1p20`                                        | absent | empty |
| P20  | `algebra_2varlineareq_fp3zeq11_3tfm1m5zeqn68_feqn10_zeq7`          | absent | empty |

P01-P16 all have populated `runtime_repo/` (7 files: agent_audit_trail, agent_pubkeys,
genesis_report, initial_q_state, pinned_pubkeys, rejections, synthetic_rejection_label)
and `verdict.json: PROCEED`.

## §2 — Root cause (single shared)

All 4 evaluator runs panicked at boot with **identical** message:

```
thread 'main' (...) panicked at experiments/minif2f_v4/src/bin/evaluator.rs:451:9:
TRUST_ROOT_TAMPERED at evaluator boot:
  TRUST_ROOT_TAMPERED: /home/zephryj/projects/turingosv4/src/runtime/mod.rs hash mismatch
  (expected 33ff089779f8cb63fea14c57a117bfe30397382eda47bc06221d95881688332b,
   actual   8cde3e8aafdffa928a66b8e2be5abc367f2f7d13859833d02e1d2bba15a58b7d)
```

Sequence reconstruction:

1. M0 batch started at HEAD `5e6d7c7` (post-(d) `constitution_admission_no_fail_open_default`,
   pre-tamper-fix). At this HEAD `genesis_payload.toml` had `src/runtime/mod.rs = 33ff0897`
   and the file on disk also hashed to `33ff0897`. **Manifest in sync with file.**
2. Problems P01-P16 ran fine — each evaluator boot found Trust Root intact
   (manifest `33ff0897` == disk `33ff0897`).
3. Mid-batch (between P16 boot and P17 boot, somewhere in the ~30 min window),
   `src/runtime/mod.rs` was modified on disk to add `pub mod audit_tamper;` (the new
   module declaration the tamper-3-of-3 fix introduced). At that moment the file's
   hash flipped to `8cde3e8a` BUT `genesis_payload.toml` still had `33ff0897` —
   the rehash entry was added later in the same dev session, but not at the moment
   the source change happened.
4. P17-P20 each booted, read the stale `genesis_payload.toml` (`33ff0897`),
   recomputed the on-disk file's hash (`8cde3e8a`), saw the mismatch, and panicked.
5. Evaluator panic is uncatchable from the batch driver's perspective beyond
   "exit code != 0 / no verdict.json written" — which is exactly the symptom we
   observe.

`audit_tape.stderr` and `audit_tape_tamper.stderr` for the 4 problems show the
*downstream* symptom — `pinned_pubkeys.json: No such file or directory` — because
the evaluator never ran, so it never wrote the runtime_repo artifacts the audit
binaries expect.

## §3 — Why this is OPERATIONAL, not a bug

The harness behaved correctly:

- `verify_trust_root` (`src/boot.rs::71`) is a fail-closed pre-init gate. It MUST
  panic on any mismatch — that's the entire point of pinning the Trust Root.
- The mismatch was real: source file was at one hash, manifest claimed another.
- A passing-with-stale-manifest behaviour would be a constitutional violation
  (tampered file would silently boot).

The bug, if any, is in the **dev workflow**: long-running batch + concurrent code
changes that touched a Trust-Root-pinned file. The mechanism that should have
prevented this — `/runner-preflight` clean-tree check — only fires AT BATCH START,
not mid-batch.

## §4 — Forward-defense options

| Option | Class | Cost | When |
|--------|-------|------|------|
| A. Operational discipline only — don't modify source during a batch run. | 0 (rule) | 0 | Now (memory entry below). |
| B. Batch driver re-validates Trust Root before each problem boot (independent of evaluator's own check); abort batch with a clear "Trust Root drifted mid-batch at problem N" error. | 1-2 | ~1-2 hours | Forward TB or follow-up Class-1 commit. |
| C. Snapshot the entire repo to a tempdir at batch start; run all problems against the snapshot; original repo can be modified concurrently. | 2-3 | ~2-4 hours | Future batch infra TB. |

**Recommendation**: A immediately (cheapest, addresses 100% of current incidents);
B as a forward defense if M1+ batches become routine (mid-batch protection becomes
load-bearing once batches are 1+ hour). C is overkill for current scale.

A is encoded as a new memory entry below.

## §5 — Empirical re-validation

After my session #34 work landed (`src/runtime/mod.rs` unchanged from `8cde3e8a`),
the `genesis_payload.toml` manifest matches the on-disk file:

```
sha256sum src/runtime/mod.rs
= 8cde3e8aafdffa928a66b8e2be5abc367f2f7d13859833d02e1d2bba15a58b7d
genesis_payload.toml line 218
= "src/runtime/mod.rs" = "8cde3e8aafdffa928a66b8e2be5abc367f2f7d13859833d02e1d2bba15a58b7d"
```

A re-run of the M0 20-problem batch on the current clean tree, without concurrent
dev work, would succeed for all 20 (modulo whatever pre-existing per-problem
LLM/Lean variance contributes to the `error_or_no_pput` evaluator-outcome bucket;
that's a separate class).

## §6 — Verdict

- **Bug**: NONE in TuringOS code.
- **Workflow violation**: 1 — concurrent source modification of a Trust-Root-pinned
  file during an active 30-minute M0 batch.
- **Failure mode is correct**: fail-closed Trust Root panic is the constitutionally
  mandated behaviour.
- **Code fix required**: NONE.
- **Operational fix**: memory entry "do not modify source during long-running
  batch runs; if you must, abort and restart" — encoded as
  `feedback_no_concurrent_dev_during_batch.md` (added this session).
- **Forward enhancement (optional)**: batch driver mid-batch Trust Root re-check
  per Option B above. Forward-bind as a low-priority Class-1 follow-up.

`FC-trace: FC2-INV1 (boot integrity).`
