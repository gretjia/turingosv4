# TB-SOFTWARE-3-0-CONSOLIDATION — Package §8 Directive

**Class**: 2 (multiple Class 2 atoms; one Class 1 audit script; remainder Class 0)
**Scope**: feature branches `feature/sw3-s0-charter` through `feature/sw3-s6-ship`
**Binding**: package-level §8 covering all atoms S0–S6
**Predecessor**: TB-TDMA-GENERATE-PHASE-E (Atoms 19–27) + TB-PHASE-E-REAL-VALIDATION (6 tests) — all merged
**Architect**: user (gretjia)
**Date**: 2026-05-23

---

## 1. Architect direction (verbatim, 2026-05-23)

> 目标：收紧现有 Software 3.0 产品路径，而不是引入平台化基础设施。
>
> Core illusion:
> `user answers → slot evidence snapshot → SpecCapsule/CAS → artifact → derived view/replay`
>
> 保留方向，删除过度工程：不做 `ChatProvider` rename + abstraction in same atom，
> 不新增 `ModelCallReceipt` runtime module，不改 `run_constitution_gates.sh`，
> 不碰 Trust Root / Class 4 surface.

**v1→v2 corrections recorded as memory** (
[`feedback_defer_abstraction_until_second_impl`](../../../home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_defer_abstraction_until_second_impl.md),
[`feedback_git_hygiene_no_bulk_ops`](../../../home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_git_hygiene_no_bulk_ops.md),
[`feedback_conservative_error_semantics`](../../../home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_conservative_error_semantics.md)).

---

## 2. Hard scope freeze (do NOT touch in this package)

The following surfaces MUST remain untouched across every atom of this package.
A PR that modifies any of these files will be REJECTED:

- `src/state/typed_tx.rs`
- `src/state/sequencer.rs`
- `src/bus.rs`
- `src/bottom_white/cas/schema.rs`
- `constitution.md`
- `genesis_payload.toml` (Trust Root)
- `src/runtime/mod.rs` (no new export)
- Any new CAS `ObjectType` registration
- Any provider abstraction layer (deferred until 2nd real provider lands)

Enforcement: pre-merge gate per PR (see §6).

---

## 3. Atom decomposition (12 atoms across 6 phases)

| Phase | Atom | Class | Scope summary |
|-------|------|-------|---------------|
| 0 | S0.1 | 0 | §8 directive + TB charter (this file + charter) |
| 0 | S0.2 | 0 | Worktree bootstrap + baseline-repair-if-needed |
| 1 | S1.1 | 2 | Remove t_hash_*/simple_hash from src/web/write.rs; parse_fail → 502 |
| 1 | S1.2 | 2 | Tests covering JSON receipt / legacy stdout / malformed → 502 / TaskCreated NOT broadcast on fail |
| 2 | S2.1 | 2 | Private GrillSessionSnapshot CAS write after every handler mutation |
| 2 | S2.2 | 2 | Cache-miss path: load latest snapshot; rebuild GrillSession cache; preserve invalid-input behavior on miss |
| 2 | S2.3 | 2 | Integration test `tests/spec_session_resume_smoke.rs` |
| 3 | S3.1 | 2 | `BuildSessionViewError` only for corrupt/read/decode; empty stays `Ok(SpecPending)` |
| 3 | S3.2 | 2 | Callers updated; web maps to 500 only on corrupt |
| 3 | S3.3 | 2 | 3 tests: empty-ok, corrupt-error, bad-capsule-error |
| 4 | S4.1 | 2 | Rename `siliconflow_client.rs` → `chat_client.rs` (5 import sites + registry); NO abstraction layer |
| 4 | S4.2 | 0 | `LLM_BOUNDARY_INVENTORY_2026-05-23.md` doc |
| 5 | S5.1 | 1 | `scripts/audit_legacy_bypass.sh` standalone |
| 5 | S5.2 | 0 | `NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md` doc |
| 6 | S6.1 | 0 | Aggregate ship report |
| 6 | S6.2 | — | Cumulative constitution + Karpathy audits |

Note: numbering shows atoms-per-phase from the plan; actual TaskCreate IDs and
PR numbers are recorded in `handover/tracer_bullets/TB_LOG.tsv` at ship time.

---

## 4. KILL criteria

- **KILL-stdout-1**: `! grep -E "t_hash_|simple_hash" src/web/write.rs` returns empty post-merge
- **KILL-resume-1**: clearing `AppState.sessions` after one accepted turn still allows next turn to continue from CAS snapshot
- **KILL-buildview-1**: 3 tests distinguish empty-ok, corrupt-error, bad-capsule-error
- **KILL-provider-rename-1**: `! grep -E "siliconflow_client" src/bin/turingos/cmd_*.rs` AND `! grep -E "siliconflow_client" src/bin/turingos.rs`
- **KILL-scope-freeze**: per PR, `! git diff --name-only origin/main...HEAD | grep -E "<§2 list>"`

---

## 5. Audit cadence

Per AGENTS.md §14:
- **Class 0 atoms** (S0.1, S0.2, S4.2, S5.2, S6.1, S6.2): no per-atom witness
- **Class 1 atom** (S5.1): no per-atom witness; included in cumulative
- **Class 2 atoms** (S1.1–S3.3, S4.1): single post-impl Codex code review per atom
- **Cumulative (S6.2)**: constitution audit (`auditor` Opus) + Karpathy audit (`general-purpose` Sonnet)

Audit verdict domains:
- Implementation review: `PROCEED | CHALLENGE | VETO`
- Constitutional witness: `NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | SECOND-SOURCE-DRIFT`
- Karpathy review: `PASS | CHALLENGE | VETO`

Conservative resolution per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

---

## 6. Pre-merge gates (every PR)

```bash
cargo fmt --all -- --check
cargo test --workspace --no-fail-fast
cargo test --test constitution_matrix_drift
bash scripts/run_constitution_gates.sh

# Scope-freeze gate (HARD):
git diff --name-only origin/main...HEAD | \
  grep -E "^(src/state/typed_tx|src/state/sequencer|src/bus|src/bottom_white/cas/schema|constitution|genesis_payload)\.rs$" && exit 1 || true

# Phase-specific KILL grep guards (per §4)
```

---

## 7. Architect signature

By committing this file under the architect's git identity, the architect signs:

1. The 12-atom scope above
2. The Class 2 / Class 0 / Class 1 classification per atom
3. The hard scope freeze in §2 (Class 4 surfaces untouched)
4. The explicit deferral of provider abstraction (ChatProvider enum, ModelCallReceipt module) to a future Class 3/4 packet
5. The pre-merge gates in §6

After this commit, every atom PR references this file in its description.

Architect: gretjia <zephryj@icloud.com>
Date: 2026-05-23

---

## 8. Relationship to in-flight work

This package SUPERSEDES the in-flight Phase E follow-up direction:
- Phase E F1 (Atom 27 cross-CLI resume) — already shipped (PR #118)
- Phase E F3 (Atom 28 cargo test) — uncommitted on `feature/phase-e-fix-f3-cargo-test-judge`; targeted-stashed under message `phase-e-atom-28-deferred-2026-05-23`; recoverable via `git stash list`
- Phase E F4 (Atom 29 template-aware) — not started; deferred
- Phase 2 stress tests (T7–T15) — not started; deferred

If user wishes to return: `git stash list` shows the saved stash entry by message.

---

## 9. Final ship gate

After S6.2:

1. All atoms cargo tests + constitution gates GREEN on every PR
2. KILL-stdout-1 verified
3. KILL-resume-1 verified
4. KILL-buildview-1 verified (3 tests pass)
5. KILL-provider-rename-1 verified
6. KILL-scope-freeze held on every PR
7. `LLM_BOUNDARY_INVENTORY_2026-05-23.md` committed and ship-report-referenced
8. `audit_legacy_bypass.sh` + `NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md` committed
9. Cumulative constitution audit NO-VIOLATION
10. Cumulative Karpathy audit PASS (or NON-BLOCKING violations only)

Until ALL 10 GREEN, not declared shipped.
