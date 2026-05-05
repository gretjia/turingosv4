# Codex TB-16.x.2.4 Audit — Multi-WorkTx + Boltzmann RUNTIME (Class 3 dual external audit)

**Round**: R1
**Date**: 2026-05-05
**Audit target**: commit `b5118fd17b0f8666a25453239104e54406e9f80b` (TB-16.x.2.4 pre-audit)
**Auditor**: Codex via codex:codex-rescue subagent (impl-paranoid angle)
**Test baseline**: `cargo test --workspace` = 915 PASS / 0 FAILED / 150 ignored
**Trust Root**: GREEN
**Audit envelope**: Class 3 (per umbrella charter §0; high-impact V3L-14 anti-collapse mechanism)
**Audit mode**: implementation paranoia (Gemini covers architectural strategic in parallel; see GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R1.md)

---

## OVERALL VERDICT

**VETO** (conviction: HIGH; recommendation: FIX-THEN-PROCEED)

Counts: VETO=4, CHALLENGE=4, PASS=5

**.fix commit must address VETO #1, #2, #3, and #4 before re-audit. Also resolve CHALLENGE #1 explicitly: either implement the required admission-path alignment or land a ratified architecture note; do not ship the current comment-only deviation as a workaround.**

---

## VETO

- **VETO #1** — id=43 counts ROOT as diversity. `src/runtime/audit_assertions.rs:1933` starts the parent key match, `:1934` maps `None` to `"ROOT"`, and `:1940-1944` includes that category in Shannon entropy. The smoke evidence is exactly `{None: 1, iter-0: 3}` at `handover/evidence/tb_16_x_2_4_smoke_2026-05-05/P12_boltzmann_runtime/README.md:28-35`, yet `verdict.json` reports `id=43` Pass at `:239-242`. This passes on root-vs-one-parent, not non-root parent-selection diversity.

- **VETO #2** — `produced_worktx_ids.push` occurs before commit confirmation. The hook pushes at `experiments/minif2f_v4/src/bin/evaluator.rs:1423`, then awaits commit at `:1425-1430`. `bus.submit_typed_tx` explicitly returns immediately while commit is asynchronous at `src/bus.rs:136-138`, and the sequencer logs/skips rejected txs at `src/state/sequencer.rs:2882-2896`. The trace shows prior-commit awaits timing out at `evaluator.stderr:7, :9, :11`.

- **VETO #3** — Smoke harness fail-closed gaps. The script uses `set -uo pipefail` without `-e` at `handover/tests/scripts/run_tb_16_x_2_4_smoke_2026-05-05.sh:31`, captures evaluator `RC` but never gates on it at `:80-84`, reuses existing output directories at `:34-57`, and pipes `audit_tape` through `tail` without an explicit status check at `:94-103`. Static consequence: stale `verdict.json` can satisfy the final Python gate at `:141-168`.

- **VETO #4** — The runtime hook itself is warn-and-continue on inputs and critical write/commit failures. Malformed env parsing degrades to warnings at `evaluator.rs:1267-1281`; CAS/telemetry failures return `None` and skip at `:1359-1379`; WorkTx construction failure only warns at `:1400-1403`; commit wait failure only warns at `:1425-1430`; final reporting calls queued ids "accepted" at `:1434-1437`.

## CHALLENGE

- **CHALLENGE #1** — STEP_B/admission-path deviation remains unresolved in code. The block declares no sequencer-side admission change at `evaluator.rs:1251-1261`; the cited existing selector call is actually an unused `_v2_canonical_pick` at `:2086-2089`; `WorkTx` has no `parent_tx` field at `src/state/typed_tx.rs:223-235`. This conflicts with the "no workarounds, strict constitutional alignment" instruction and needs explicit fix or ratified architecture.

- **CHALLENGE #2** — Pre-iteration wait is placed before the tx it is supposed to observe. The loop snapshots `pre_root` and waits at `evaluator.rs:1296-1306`, but the helper contract says callers pass a pre-snapshot, await **after** submission, then capture the new root at `src/runtime/adapter.rs:570-576`. The committed trace shows these waits failing at `evaluator.stderr:7, :9, :11`.

- **CHALLENGE #3** — Evidence README does not match committed verdict artifacts. README claims `l4e_count = 1` and `cas_object_count = 24` at `README.md:60-62`; actual `verdict.json` says `l4e_count = 2` and `cas_object_count = 23` at `:4-8`.

- **CHALLENGE #4** — Hypothesis: fallback parent selection bypasses Boltzmann when `v2_pick` is `None`. The code explicitly sets `parent_tx = v2_pick OR produced_worktx_ids.last()` at `evaluator.rs:1322-1327`. This path is not triggered for iter 1+ in the committed smoke (`evaluator.stderr:8, :10, :12` show `v2_pick=Some(iter-0)`), but the hook can manufacture parent edges without scheduler selection under empty/masked price-index conditions.

## PASS

- **PASS #1** — Hook is properly env-gated under `TURINGOS_FORCE_BOLTZMANN_SEED_WORKTXS` at `evaluator.rs:1238-1266`.

- **PASS #2** — Hook calls `boltzmann_select_parent_v2` on `bus.snapshot().price_index` and `mask_set` at `evaluator.rs:1315-1321`; the selector surface is the integer-rational v2 scheduler at `src/sdk/actor.rs:57-65`.

- **PASS #3** — ProposalTelemetry is built with `parent_tx` before WorkTx construction at `evaluator.rs:1332-1350`, then written through `write_to_cas` at `:1353-1359`; the CAS writer canonical-encodes and stores the record at `src/runtime/proposal_telemetry.rs:311-326`.

- **PASS #4** — Seeded WorkTxs are real signed WorkTxs with `predicate_passes=true` at `evaluator.rs:1389-1398`; the adapter signs the canonical digest at `src/runtime/adapter.rs:197-200`.

- **PASS #5** — Static smoke artifacts show the chain did contain 4 WorkTxs and reported `PROCEED`: `verdict.json:11-13` and `:330-348`. Tamper summary reports 3/3 detected at `tamper_report.json:2-4`.

## Implementation paranoia checklist

- **FAIL: off-by-one / fence semantics**. Loop `0..count` is exact at `evaluator.rs:1295`, but the entropy fence is invalid because ROOT is counted as a category at `audit_assertions.rs:1933-1937`.
- **FAIL: commit-confirmation ordering**. `produced_worktx_ids.push` precedes await at `evaluator.rs:1423-1430`.
- **FAIL: env-var parse failure modes**. `parse().unwrap_or(0)` converts bad count/stake into warn-only behavior at `evaluator.rs:1275-1281`.
- **FAIL: fail-closed on missing inputs**. CAS/telemetry/constructor failures warn and skip at `evaluator.rs:1359-1403`; smoke script lacks `set -e` at `run_tb_16_x_2_4_smoke_2026-05-05.sh:31`.
- **FAIL: idempotency / stale outputs**. Script uses persistent output dirs without cleanup at `run_tb_16_x_2_4_smoke_2026-05-05.sh:34-57`.
- **PASS: double-rehash / trust-root update**. `genesis_payload.toml:164` pins the new evaluator hash for this hook.
- **PASS: replay-determinism evidence**, static-read only. README records byte-identical replay at `README.md:80`; no runtime execution was performed in this audit.
- **FAIL: entropy/diversity formula correctness**. ROOT-counted entropy at `audit_assertions.rs:1933-1944` is the primary VETO.
- **CHALLENGE: thread/parallelism**. Submission is async by contract at `src/bus.rs:136-138`; the hook treats queued tx ids as usable before confirmed commit.
- **PASS: ChainTape externalization purity** per FC1/FC2/FC3. ProposalTelemetry is externalized to CAS before WorkTx cid use at `evaluator.rs:1332-1359`, and id=24 passes at `verdict.json:203-208`.
- **PASS: no f64 / integer-rational selector surface**. `src/sdk/actor.rs:57-65` takes price index/mask/policy/RNG, and price module states integer-rational arithmetic at `src/state/price_index.rs:13-16`.
- **N/A: cargo/runtime execution**. Workspace is read-only for this audit; findings are static-read only against commit `b5118fd17b0f8666a25453239104e54406e9f80b`.

---

## Cross-reference

- Paired Gemini audit: `handover/audits/GEMINI_TB_16_X_2_4_AUDIT_2026-05-05_R1.md` (also VETO; primary angle: Q8 zero unit tests).
- Conservative resolution per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. Both auditors VETO → must fix.
- Memory `feedback_no_workarounds_strict_constitution`: user explicitly rejects null-pointer / OBS-bucket workarounds. CHALLENGE #1 (STEP_B deviation) cannot be OBS-deferred; must take explicit position with code change OR ratified architecture note.
- Memory `feedback_audit_obs_bias`: cheap fixes get fixed; the 4 VETOs + 4 CHALLENGEs are all cheap-to-fix code-level issues; no OBS-defer justified.
