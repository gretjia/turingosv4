# TB-STRESS-PHASE-2 — Adversarial substrate stress battery

**Date**: 2026-05-23
**Risk classes**: 0 (charter/docs), 1 (runner scripts), 2 (evidence-bearing real runs)
**Phase_id**: post-TB-SOFTWARE-3-0-CONSOLIDATION (S6 merged 2026-05-23)
**Roadmap exit criteria addressed**: Phase E Phase 2 / TDMA-Bounded substrate adversarial coverage; TB-SOFTWARE-3-0 surface durability under load
**Kill criteria tested**: see §5

## 1. Why now

Phase E Phase 1 shipped 6 happy-path validation tests (T1–T6 in
`handover/reports/PHASE_E_REAL_VALIDATION_2026-05-23.md`). TB-SOFTWARE-3-0
shipped 6 atoms that hardened the web/runtime boundary (S1–S6). Neither
package exercised production-load adversarial conditions: concurrent
writers, mid-commit crashes, sidecar index corruption, LLM-provider
5xx storms, long-run drift.

This package is the explicit adversarial follow-up. It is **not** a new
feature; it produces evidence under deliberately hostile inputs.

## 2. Scope freeze

NO source change to:
- `src/state/typed_tx.rs`, `src/state/sequencer.rs`, `src/bus.rs`
- `src/bottom_white/cas/schema.rs`
- `constitution.md`, `genesis_payload.toml`
- `src/runtime/mod.rs` export surface
- any new CAS `ObjectType`
- any provider abstraction layer

Allowed changes:
- new files under `scripts/stress/` (Class 1 runner scripts)
- new evidence directories under `handover/evidence/stress_<id>_<ts>/` (Class 2 evidence)
- charter / ship report / audit docs (Class 0)

If a stress test surfaces a real production defect that **must** be fixed
to make the test pass, the fix is escalated as a separate Class 2/3 PR
**outside this package**. Stress tests are observation, not implementation.

## 3. The 10 stress tests

| ID | Surface | Adversarial input | Intensity | LLM cost | Wall est |
|----|---------|-------------------|-----------|----------|----------|
| ST-01 | GitTapeLedger libgit2 backend | 20× SIGKILL mid-commit | severe | $0 | 15 min |
| ST-02 | Kernel + GitTapeLedger | 3 concurrent writers × 100 attempts | severe | ~$4 | 1 hr |
| ST-03 | CAS sidecar index | truncate `.turingos_cas_index.jsonl` mid-byte | severe | $0 | 10 min |
| ST-04 | S2 GrillSessionSnapshot | server restart every 5 turns × 50 turns | severe | ~$2 | 30 min |
| ST-05 | S3 BuildSessionView | 1000 capsules across 10 sessions, 10% corrupt bytes | severe | $0 | 15 min |
| ST-06 | chat_client | 50% HTTP 5xx storm (mock provider) | severe | ~$2 | 45 min |
| ST-07 | S1 task/open | 100 concurrent task/open, 30% malformed CLI stdout | severe | $0 | 15 min |
| ST-08 | Long-run grill | single 1000-turn session, monitor heap/cas count/snapshot size | severe | ~$10 | 2 hr |
| ST-09 | chat_client | prompt > 100KB + mid-stream response truncation | severe | ~$0.50 | 10 min |
| ST-10 | Double-backend cross-process | A writes via memory backend; B reads via git backend | severe | $0 | 10 min |

**Total estimates**: ~6 hr wall, ~$18.50 LLM cost.

## 4. Execution order (optimal for failure-detection × cost)

```
phase A — no-LLM, fast (1 hr total):
  ST-01 → ST-03 → ST-05 → ST-07 → ST-10
phase B — LLM cheap, medium:
  ST-09 → ST-04 → ST-06
phase C — LLM expensive, long (3 hr):
  ST-02 → ST-08
```

Phase A first so any bug in a runner (e.g. evidence path mistakes) is
found before burning LLM budget on Phase B/C.

## 5. KILL criteria per test (machine-checkable)

| ID | Pass iff |
|----|----------|
| ST-01 | After 20× SIGKILL, `git2::Repository::open(workspace/cas)` succeeds AND `refs/tdma/verified_head` is either H0 or a previously-committed sha (no half-written sha) |
| ST-02 | All 300 attempts in `r2_write_attempt_telemetry`; FC1 invariant holds across all 3 writers: `completed_llm_calls == step + parse_fail + llm_err` |
| ST-03 | After truncation, `CasStore::open` either succeeds (lenient sidecar) OR returns a clean `CasError`; in the success case, `list_cids_by_object_type` returns entries written before the truncation point |
| ST-04 | All 50 turns succeed; AppState.sessions cleared between groups of 5 turns; snapshot restore rebuilds GrillSession with `turn_count` continuity |
| ST-05 | Bucket distribution: ~90% `Ok(SpecPending)` or normal `Ok(...)`, ~10% `Err(Decode)` for corrupted bodies; zero `Err` on the clean 900 |
| ST-06 | Of 200 chat_complete_blocking attempts: ~100 succeed, ~100 fail with `LlmError::HttpStatus`; zero `r2_write_attempt_telemetry` step entries for failed attempts (only llm_err entries) |
| ST-07 | Of 100 concurrent /api/task/open: 70× 200 (with `task_id: String`), 30× 502 (with `kind: "task_id_parse_failed"`); zero TaskEntry written for the 30 |
| ST-08 | 1000 turns complete; heap grows < 2× linear in turn count; CAS object count ≤ 2N (snapshot + per-turn capsule); snapshot file size < 1 MB at turn 1000 |
| ST-09 | 150KB prompt either rejected by provider with clean error OR succeeds with `usage_total_tokens` reported; truncated response returns `LlmError::Decode` or `Schema`, never panic |
| ST-10 | Process B reads all 10 sessions written by A; canonical `state_root` matches between backends |

## 6. Audit cadence

Per AGENTS.md §14 Class 2 cadence:
- Each runner script: predicate self-test only (no full witness audit)
- Cumulative end-of-package: clean-context Codex constitution + Karpathy audit on the aggregate evidence

## 7. Atom decomposition

| Atom | Scope | PR cadence |
|------|-------|------------|
| STRESS-0 | This charter + §8 directive + 10 runner scripts | 1 prep PR (Class 1) |
| STRESS-1..5 | Execute Phase A (no-LLM); commit evidence | 1 PR (Class 2 evidence) |
| STRESS-6..10 | Execute Phase B + C (LLM); commit evidence | 1 PR (Class 2 evidence) |
| STRESS-SHIP | Aggregate ship report + cumulative audits | 1 PR (Class 0) |

Total: 4 PRs. (Smaller PR count than TB-SOFTWARE-3-0 because there are
no per-atom code changes — runners are written once, evidence accumulates.)

## 8. §8 architect ratification

Per CLAUDE.md §3 / AGENTS.md §5: this package is Class 0/1/2 only. No
Class 3/4 surfaces touched. Standard package §8 (not per-atom) is
sufficient. The package §8 acknowledgement is:

> Author/architect (gretjia, 2026-05-23): TB-STRESS-PHASE-2 is authorized
> as a Class 0/1/2 evidence-only package on top of the just-shipped
> TB-SOFTWARE-3-0 + Phase E Phase 1 substrate. Scope freeze list in §2 is
> binding. Real LLM calls under §3 intensity limits authorized.

## 9. Done definition

ALL of:
1. 10 runner scripts exist under `scripts/stress/` and are executable
2. 10 evidence dirs under `handover/evidence/stress_*` exist with non-empty `summary.md`
3. Each KILL criterion in §5 either PASS or has a documented defer reason
4. Cumulative constitution audit: `NO-VIOLATION`
5. Cumulative Karpathy audit: `PASS`
6. Aggregate ship report `handover/reports/STRESS_PHASE_2_SHIP_REPORT_2026-05-23.md` exists
7. All 4 PRs merged to main

TRACE_MATRIX: FC1 (LLM boundary + kernel attempt loop under load),
FC2 (derived views under corruption), FC3 (CAS evidence integrity).
