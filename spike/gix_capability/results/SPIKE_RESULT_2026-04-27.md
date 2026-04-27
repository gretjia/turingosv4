# CO1.3.1 git Substrate Spike — Result Report

> **Date**: 2026-04-27
> **Operator**: ArchitectAI (Claude) per user `A.fast` directive
> **Spec authority**: `handover/specs/CO1_3_1_GIX_SPIKE_PREFLIGHT_v1_2026-04-27.md`
> **Substrate library**: **git2-rs 0.20** (libgit2 binding)
> **Initial substrate attempted**: `gix` 0.66.0 (pivoted to git2-rs per pre-flight § 4 due to high-level commit API gaps; see § 3 below)
> **Result**: **8/8 PASS** — git2-rs is FIT FOR PURPOSE for v4 ChainTape Path B substrate.

---

## § 1 Executive Summary

All 8 capabilities (C1-C8) PASS on first run after pivoting from gix 0.66 to git2-rs 0.20. Performance is **7.16× target** (716 commits/sec vs target 100/sec). Concurrent init scales to 4 threads in 14ms. Replay determinism + hooks isolation both work as needed.

**Recommendation**: lock `git2-rs 0.20` as v4 ChainTape Path B substrate library. Update Plan v3.2 atom CO1.3 to specify git2-rs (was generic "gix or git2-rs fallback"). Proceed with CO1.3.2 (per-cell runtime_repo init in evaluator) using git2-rs.

---

## § 2 Per-Capability Results

| # | Capability | Pass | Key Data | Pre-flight Target |
|---|---|---|---|---|
| C1 | Init repo programmatically | ✅ | first commit SHA `61b12feb...` | non-shell init |
| C2 | Multi-parent commit | ✅ | 2 parents on merge commit `9b87c96a...` | DAG citation feasible |
| C3 | Deterministic tree+parent read | ✅ | 2 reads of same commit yield identical tree_sha | byte-identical |
| C4 | Concurrent init (4 threads) | ✅ | 4/4 success in **14 ms** | no lock contention |
| C5 | CAS blob round-trip | ✅ | 5/5 blobs byte-identical retrieval; git SHA-1 + external SHA-256 both available | content-addressable |
| C6 | Perf 100 commits <1s | ✅ | **139 ms** = 716 commits/sec (target 100/sec; **7.16× margin**) | < 1000 ms |
| C7 | Deterministic replay | ✅ | 2 walks each yield same 10 commits in same order | reproducible |
| C8 | Hooks compat (separate repos) | ✅ | user-facing pre-commit hook NOT invoked by runtime_repo commit | full isolation |

**Total: 8/8 PASS**.

---

## § 3 Pivot Rationale: gix → git2-rs

### 3.1 What was attempted with gix 0.66

Initial spike implementation used `gix = "0.66"` per pre-flight default. Compilation surfaced ~31 errors across 8 modules:

- `Tree::edit()` method does not exist (high-level tree-builder API)
- `Repository::rev_walk()` method does not exist (use `gix-revwalk` lower-level API)
- `config_snapshot_mut().set_value()` returns `Option<T>` (not chainable as expected)
- `&BStr` from `&String` trait bound not satisfied (string conversion friction)

### 3.2 Diagnosis

gix 0.66 prioritizes a stable lower-level (plumbing) API; high-level (porcelain) ergonomic APIs are still maturing. For v4's needs (init / commit / read / walk), the lower-level API would require ~3-5x more verbose code.

### 3.3 Pivot decision

Pre-flight § 4 explicitly authorizes git2-rs fallback:
> If gix fails, fallback to git2-rs (libgit2 bindings).

git2-rs 0.20 is mature, well-documented, has the high-level porcelain API needed. The 8 capability tests rewrote in ~2 hours total vs estimated ~6-8 hours for lower-level gix. Per "尽快拿数据" directive, git2-rs is the correct choice.

### 3.4 Constitutional check

Constitution Art 0.4 Path B says "真 git substrate" — does NOT specify implementation library. git2-rs binds libgit2 (Linus's original C library); semantically identical to gix (both implement git protocol). ✅ no constitutional violation.

### 3.5 Gemini v3.2 Q5 / Codex Q5 STEP_B serialization concern

Both audits flagged that signature verification across STEP_B branches A and B requires identical canonical serialization. Pivoting library does NOT affect this; git tree/commit/blob byte format is **standardized by git protocol**, identical regardless of binding library. STATE_TRANSITION_SPEC § 2.5 (canonical serialization) is unaffected by this pivot.

---

## § 4 Performance Profile

### 4.1 100 commits perf detail (C6)

```
target: 100 commits in < 1000 ms
actual: 100 commits in 139 ms
margin: 7.16×
ops: ~7.16 commits per millisecond
extrapolated: 716 commits/sec sustained
```

### 4.2 Phase C scaling estimate

Phase C ablation: 5 modes × 10 problems × 2 seeds × ~100 work_tx per cell = 10,000 commits total.

At 716 commits/sec (single-threaded; cold filesystem; tempfs):
- 10,000 commits / 716 = **~14 seconds total commit time**
- Wall clock for full Phase C dominated by LLM API calls (minutes per work_tx), NOT git substrate

**Conclusion**: substrate is not a Phase C bottleneck. Will revisit if production work_tx generation becomes faster than sub-millisecond per tx.

### 4.3 Concurrent init scaling

```
4 threads × 1 repo init each + 1 commit each = 4 repos
elapsed: 14 ms
per-repo: ~3.5 ms (parallelized)
```

Concurrent init does NOT serialize on filesystem locks at this scale. Scales to v4's needed parallelism (Phase C 5-mode parallel cells).

---

## § 5 Capability vs Pre-Flight Acceptance Criteria

Per pre-flight § 1 acceptance: C1-C8 all pass within 5 days. Performance C6 < 1 second.

| Pre-flight criterion | Actual |
|---|---|
| 5-day time-box | **Completed in single session** (~3 hours including pivot) |
| C1-C8 all pass | ✅ 8/8 |
| C6 perf < 1000 ms | ✅ 139 ms |
| Failure → escalate | n/a |

**No escalation needed**.

---

## § 6 Out-of-Scope (Per Pre-Flight § 6, Confirmed)

These were explicitly out-of-scope for spike + remain so for v4:
- Network transport (push/pull): runtime_repo is local-only ✓
- Reference notes / git notes: speculative ✓
- Garbage collection: per-cell runtime_repo is short-lived ✓
- Worktree operations beyond simple init ✓
- SubModule support ✓
- Protocol v2 transport ✓

If any becomes needed during CO1.3.2+, escalate via Plan amendment, not silent expansion.

---

## § 7 Implications for Plan v3.2

Plan v3.2 atom CO1.3 specifies "gix substrate (Path B)". This spike confirms:
- ✅ git substrate is viable; Path B confirmed
- ❌ gix specifically is NOT used; git2-rs IS
- → **Plan v3.2 atom CO1.3 needs cosmetic patch** to specify git2-rs as the library (was: "gix; git2-rs as fallback"; now: "git2-rs primary; gix considered+rejected per spike")

`CO1.3.2` (per-cell runtime_repo init in evaluator.rs) is **UNBLOCKED** by this spike. Critical-path-wise, CO P1 can launch CO1.3.2 + CO1.0 + CO1.0a in parallel with spec v1.3 dual re-audit (which is the lingering NO-GO concern).

---

## § 8 Audit Pack for Dual Review

If Codex/Gemini audit this spike result, here's what they'd verify:

1. ✅ Spike compiles (`cargo build -p gix_capability_spike`)
2. ✅ Spike runs (`cargo run -p gix_capability_spike --bin gix_capability_spike`)
3. ✅ All 8 capabilities pass with concrete data above
4. ✅ Spike code lives at `spike/gix_capability/`; does NOT touch `src/`
5. ✅ Workspace builds clean (1 unused-mut warning; not blocking)
6. ✅ Constitutional check (Art 0.4 Path B; pre-flight § 4 fallback authorized)

---

## § 9 Honest Acknowledgements

What this spike achieves:
- Closes CO1.3.1 atom with concrete data
- Validates substrate choice (git2-rs)
- Provides perf baseline (716 commits/sec)
- Unblocks CO1.3.2 implementation

What this spike is honest about:
- It's a smoke test, not full integration. Real bus.rs / kernel.rs split (CO1.1.4 / CO1.1.5) may surface library quirks not visible in 8-test spike.
- Pivot from gix to git2-rs was pragmatic; gix may mature in future and could revisit in v4.x or v5.
- Concurrent test was 4 threads on disjoint paths; concurrent commits to SAME repo not tested (Phase C uses disjoint cells, so not needed for v4).

What this spike does NOT prove:
- That bus.rs/kernel.rs split will be smooth (still NO-GO until spec re-audit PASS)
- That signing throughput at scale (>1000 system-signed retry summaries/sec) is workable
- That gix could not eventually be used (just that v4 will use git2-rs)

---

## § 10 Sign-off

**Spike status**: ✅ PASS  
**Recommendation**: **PROCEED with git2-rs as v4 substrate**. CO1.3.2 unblocked. CO1.1.4/1.1.5/1.7 still gated on spec v1.3 dual re-audit PASS/PASS (separate concern; not this spike's scope).

— ArchitectAI, 2026-04-27
