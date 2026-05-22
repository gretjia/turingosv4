# TB-TDMA-BOUNDED-RC1 Charter — 2026-05-22

**Branch**: `feature/tdma-bounded-rc1` (NEW; no direct main merge per K-HARDEN-7)
**Risk class**: 4 (Class 4 substrate; per-atom dual-audit gated; package-level §8 via on-disk directive)
**Architect §8 sign-off**: `handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md`
  — explicit package-level RC1 authorization; explicit override of `feedback_no_batch_class4_signoff` for RC1 only
**Orchestrator plan**: `/home/zephryj/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md`
**Source directive date**: 2026-05-22 (OMEGA-final TDMA-Bounded-RC1 落地规格)

---

## phase_id

P3 (economy ledger forward) + P4 (information loom prep)

## roadmap_exit_criteria_addressed

- **P3:7** — kernel canonical retry path (retry as kernel state transition, not prompt accumulation)
- **P4:1** — memory canonical from tape (no mutable sidecar; BBS derivable as pure function over tape)
- **P4:2** — version-control three-tuple state `Q_t = ⟨q_t, HEAD_t, tape_t⟩` per constitution Art. 0.4

## kill_criteria_tested

| KILL ID | Description | Gate / Mechanism |
|---------|-------------|-----------------|
| KILL-tdma-1 | raw_stderr leaks into active prompt | Gate 6 + grep guard `! grep raw_stderr src/memory_kernel.rs ...` |
| KILL-tdma-2 | BBS sidecar mutable HashMap | Gate 5 + grep guard `! grep update_belief_state src/` |
| KILL-tdma-3 | byte-proxy as token count | grep guard `! grep payload.len() src/` + token_budget_no_byte_proxy test |
| KILL-tdma-4 | closing-tag parser dependency | Gate 7 + grep guards `! grep </STATE_UPDATE> src/` and `! grep <STATE_UPDATE> src/` |
| KILL-tdma-5 | verified_head advances on failure | Gate 4 + Gate 7 + Gate 9 |
| KILL-tdma-6 | constitution.md injected into worker prompt | Gate 8 + grep guard `! grep constitution.md src/memory_kernel.rs` |
| KILL-tdma-7 | CharterCore SHA drift undetected | Gate 8 |
| KILL-tdma-8 | zero-gain infinite retry | Gate 3 + Gate 6 |
| KILL-tdma-9 | prompt unbounded growth across retries | Gate 1 |

---

## Scope

**In scope** (RC1, this charter):
- Commit/Atom 1–7 per directive §13 (ledger schema, state-first parser, token budget, distiller, CharterCore, rtool, kernel keystone)
- Atom 7.5 — real-evidence run via JudgeAI math problem (added per constitution-audit C12)
- Atom 0 (this charter) + Atom 8 (ship report + GA §8 template) governance
- TuringOS-Memory-Harness-V1 (9 synthetic gates) + bug7_regression_suite + 5 real-tape invariants

**Out of scope** (Phase E, future):
- libgit2 / git2-rs true-git substrate (constitution Art. 0.4 Path B)
- See `handover/architect-insights/PHASE_E_TODO_TDMA.md` (created in Atom 5) for migration obligation

## Atom decomposition

| Atom | Class | Owner model | Files touched | PR |
|------|-------|-------------|---------------|----|
| 0 | 0 | Opus 4.7 | this charter + on-disk §8 + TB_LOG.tsv | PR #1 |
| 1 | **4** | Opus 4.7 xhigh | `src/ledger.rs`, `src/bus.rs` (§6 restricted), `src/wal.rs` | PR #2 |
| 2 | 1 | Codex CLI + Opus review | `src/state_update.rs` (NEW), `src/memory_kernel.rs` (NEW scaffold) | PR #3 |
| 3 | 1 | Sonnet 4.6 | `src/tokenizer.rs` (NEW), `src/token_budget.rs` (NEW) | PR #4 |
| 4 | 2 | Opus 4.7 xhigh | `src/distiller.rs` (NEW) | PR #5 |
| 5 | 2 | Sonnet 4.6 | `src/charter_core.rs` (NEW), `src/boot.rs` | PR #6 |
| 6 | 2 | Codex CLI + Opus review | `src/rtool.rs` (NEW) | PR #7 |
| 7 | **4** | Opus 4.7 xhigh | `src/memory_kernel.rs` (finalize), `tests/tdma_memory_harness_v1.rs`, `tests/bug7_regression_suite.rs` | PR #8 |
| 7.5 | 2 | Opus 4.7 xhigh | `src/judges/math_step_judge.rs` (NEW), `src/bin/tdma_rc1_real_evidence.rs` (NEW), `scripts/run_tdma_rc1_real_evidence.py` (NEW) | PR #9 |
| 8 | 0 | Opus 4.7 | ship report + GA §8 template | PR #10 |

## CI grep guards (forbidden patterns; enforced pre-merge)

The directive §15 spec uses repo-wide `grep -R ... src/`. Per Karpathy surgical-changes
discipline (audit K7) and the directive intent (KILL-tdma-3: "do not use bytes as token
proxy") the guards are **scoped to TDMA-Bounded modules** to avoid false-positives on
legitimate legacy usage (e.g., `src/bus.rs:266` V3L-21 payload-byte-length validation
is a `payload.len()` use that has nothing to do with token counting).

TDMA-scoped modules: `src/memory_kernel.rs`, `src/distiller.rs`, `src/token_budget.rs`,
`src/tokenizer.rs`, `src/state_update.rs`, `src/rtool.rs`, `src/charter_core.rs`,
`src/judges/`.

```bash
# Module-scoped (TDMA only) — the intent of KILL-tdma-1..7
TDMA_MODULES='src/memory_kernel.rs src/distiller.rs src/token_budget.rs src/tokenizer.rs src/state_update.rs src/rtool.rs src/charter_core.rs src/judges/'

! grep -R "raw_stderr" src/memory_kernel.rs 2>/dev/null | grep -E "format!|push_str|assemble|prompt"   # KILL-tdma-1
! grep -R "update_belief_state" src/                                                                   # KILL-tdma-2 (repo-wide; new sidecar pattern is always forbidden)
! grep -RE "HashMap<.*Belief" src/                                                                     # KILL-tdma-2
! grep -R "payload.len()" $TDMA_MODULES 2>/dev/null                                                    # KILL-tdma-3 (TDMA-scoped — legacy bus.rs V3L-21 exempt)
! grep -RE ".len\(\) as.*token" $TDMA_MODULES 2>/dev/null                                              # KILL-tdma-3
! grep -R "</STATE_UPDATE>" src/                                                                       # KILL-tdma-4 (repo-wide)
! grep -R "<STATE_UPDATE>" src/                                                                        # KILL-tdma-4
! grep -R "constitution.md" src/memory_kernel.rs                                                       # KILL-tdma-6
```

Repo-wide guards (KILL-tdma-2, KILL-tdma-4): sidecar-BBS and closing-tag-parser are
genuinely new anti-patterns and must NEVER appear anywhere in `src/`.

Module-scoped guards (KILL-tdma-3, KILL-tdma-6, KILL-tdma-1): apply only inside TDMA
substrate where the directive's prohibition is load-bearing.

## Hard budgets (compile-time constants in `src/token_budget.rs`)

```rust
B_G          = 500;
B_S          = 3000;
B_D          = 400;
B_T          = 1500;
B_H          = 100;
B_CTL        = 300;
B_HEADER     = 256;
B_HEADER_SCAN= 512;
B_DISTILL_IN = 2048;
MAX_RETRIES  = 5;
ZERO_GAIN_K  = 3;
EPSILON_GAIN = 0.01;
```

## Ship gate (RC1 → GA, 14 criteria)

See orchestrator plan §9. Until ALL 14 are GREEN, GA is BLOCKED and the branch remains `feature/tdma-bounded-rc1`.

## FC trace

| Atom | FC nodes touched |
|------|------------------|
| 1 | FC1a (tape_t), FC1b (Q_{t+1}), FC2 (Q_0 substrate) |
| 2 | FC1a (output edge state-first parser) |
| 3 | FC1a / FC1b (budget enforcement) |
| 4 | FC1a (rtool input), FC3 (replay determinism) |
| 5 | FC2 (Q_0 init), FC3 (constitution binding) |
| 6 | FC1a (rtool with HEAD_t) |
| 7 | FC1a (∏p), FC1b (wtool → Q_{t+1}), FC2 (boot loop), FC3 (replay) |
| 7.5 | FC1 full loop on real tape; FC3 audit |

Canonical FC hashes per `handover/alignment/TRACE_FLOWCHART_MATRIX.md`:
- FC1a: `a474c6b9ded766504a4f644a4a1b3c545316d418f0250f36ec692fcdf98f09f5`
- FC1b: `b822717b10332a2d8e789ba6af96fd4da4ff43a74afab679d1b82add9c32b64d`
- FC2: `6a4bc9195bafd55bde968fd445cdd2926d6906a7f6a2b38071d4774a7f0de333`
- FC3: `c159413984d0c6c5daa06605fea3a86a2ad4ab9c4284d0d20e0e525bf03aa9cd`

---

## Plan-level audit findings (already addressed in orchestrator plan Appendix A)

Three plan-level audits were dispatched against the orchestrator plan before execution began:

1. **Recursive audit** (Sonnet 4.6): `GAPS-FOUND` (4 NON-BLOCKING; all addressed inline in plan).
2. **Constitution audit** (Opus 4.7 / `auditor`): `VIOLATION-FOUND` BLOCKING — C7 (batch §8), C9 (Atom 7 verdict domain), C12 (no real evidence). All three FIXED — C7 via this on-disk §8 artifact; C9 via constrained verdict domain in plan §5 Atom 7; C12 via Atom 7.5 real-evidence run.
3. **Karpathy audit** (Sonnet 4.6): `VIOLATION-FOUND` minor — K10 (single-impl trait). FIXED via inline Phase E justification for `ImmutableTapeLedger` in plan §5 Atom 1.

## Predecessor / dependency

No active charter PR in flight at this charter creation (per `handover/ai-direct/LATEST.md` session #58 close 2026-05-21). No overlap with open PRs (#91, #92) per AGENTS §4.1 parallel-write check executed pre-charter.
