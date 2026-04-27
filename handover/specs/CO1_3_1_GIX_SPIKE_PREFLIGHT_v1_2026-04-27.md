# CO1.3.1 gix Substrate Spike — Pre-Flight Spec v1

> **Date**: 2026-04-27
> **Purpose**: Lock acceptance criteria + 5-day time-box gate + fallback decision tree BEFORE running any code. Gemini v3.2 review labeled gix as project's #1 technical risk. This pre-flight document removes ambiguity.
> **Authority**: Constitution Art 0.4 (Path B = real git substrate). Plan v3.2 CO1.3.1 (FIRST CO P1 atom).
> **Status**: Pre-flight DONE; spike begins on user GO. Doc-only commit.

---

## § 1 What the spike must answer

**Single binary outcome**: Does `gix` (gitoxide pure Rust) support enough git semantics to back TuringOS v4 ChainTape Path B substrate?

**Specific capabilities** the spike must verify:

| # | Capability | Why it matters | Test method |
|---|---|---|---|
| C1 | Initialize a git repo programmatically (no `git init` shell call) | per-cell `runtime_repo` initialized at evaluator startup; no shell dependency | `gix::init::Builder::new(...).create()` |
| C2 | Commit with **multi-parent** support | Contribution DAG citation = git multi-parent merge commit (per WP § 12 + Blueprint § 4 step_transition stage 7) | create commit with `parents = [a, b, c]` |
| C3 | Read commit object's tree root + parent SHAs deterministically | `state_root_t` = git tree root in Path B | `gix::objs::Commit::parse(...).tree` + `.parents` |
| C4 | Concurrent runtime_repo init from multiple subprocess workers | Phase C ablation runs 5 modes × 10 problems × N seeds in parallel; each cell needs own runtime_repo | spawn 4 subprocesses doing C1+C2 simultaneously; assert no lock contention |
| C5 | Create CAS blob + retrieve by SHA | L3 CAS uses git blob storage natively | `gix::object::Object::Blob` write + read |
| C6 | Performance: 100 commits in < 1 second | each work_tx → 1 commit; bus throughput should not bottleneck on git | benchmark loop |
| C7 | Replay: walk commits from genesis to HEAD deterministically | `replay_from_genesis` requires deterministic walk | `gix::revwalk::Walk::ancestors(...).by_commit_date()` ordered by (commit_time, sha) |
| C8 | Hooks compatibility: pre-commit hooks work or are bypassable | R-018 + R-022 pre-commit hooks operate on user-facing repo; runtime_repo is separate; verify no interference | run a runtime_repo commit while user-repo has hook configured |

**Pass criteria**: C1-C8 all pass within 5 days. Performance C6 must be < 1 second wall clock (i.e., 100 commits/sec lower bound).

**Fail criteria**: Any of C1, C2, C3, C4, C7 fails → immediate pivot to git2-rs. Any of C5, C6, C8 fails → escalate to user; either accept slower performance or pivot.

---

## § 2 Spike Code Layout (NOT to be merged into main)

```
spike/gix_capability/
├── Cargo.toml             # workspace member; pinned gix version
├── src/
│   ├── main.rs            # CLI: --capability=C1|C2|...
│   ├── c1_init.rs
│   ├── c2_multi_parent.rs
│   ├── c3_tree_parent_read.rs
│   ├── c4_concurrent_init.rs
│   ├── c5_cas_blob.rs
│   ├── c6_perf_benchmark.rs
│   ├── c7_replay.rs
│   └── c8_hooks_compat.rs
└── results/
    └── SPIKE_RESULT_2026-XX-XX.md   # one per spike attempt; immutable record
```

Spike repo lives at `spike/gix_capability/` (new top-level dir). It is **NOT** merged into `src/`. Whether the spike succeeds or fails, the spike code itself is reference-only artifact and is preserved as historical evidence.

If C1-C8 all pass: the **spike result document** is committed. Then atom CO1.3.2 (per-cell runtime_repo init in evaluator.rs) starts as a separate commit using gix in actual `src/` location.

If gix fails: pivot atom CO1.3.1' written using git2-rs; spike repeated for git2-rs.

---

## § 3 5-Day Time-Box Schedule

| Day | Focus | Deliverable |
|---|---|---|
| Day 1 | C1 + C2 + C3 (basic init, multi-parent, read) | Spike compiles + 3/8 pass |
| Day 2 | C4 + C5 (concurrent init, CAS blob) | 5/8 pass |
| Day 3 | C6 + C7 (performance, replay) | 7/8 pass |
| Day 4 | C8 (hooks compat) + integration test (full TM workflow) | 8/8 pass + integration |
| Day 5 | Write SPIKE_RESULT doc; dual audit (Codex + Gemini) | Audit reports complete |

If at any day's end pass count is below schedule, ArchitectAI surfaces to user **same day**: continue 1 more day OR pivot to git2-rs.

---

## § 4 Fallback Decision Tree

```text
SPIKE TRIGGER
    ↓
Day 1-4: gix C1-C8 attempts
    ↓
Day 5 evening: review checkpoint
    ├── 8/8 PASS → Codex+Gemini dual audit
    │       ├── BOTH PASS → CO1.3.2 starts using gix; Plan v3.2 stays as written
    │       └── ANY VETO → pivot consideration (likely back to gix with fix; or git2-rs)
    └── < 8/8 PASS → escalate to user
            ├── User: "continue 1-2 more days" → extend up to Day 7 max
            ├── User: "pivot to git2-rs" → CO1.3.1' git2-rs spike (~3 more days)
            └── User: "abandon Path B → Path A semantic version" → MAJOR plan rewrite (Plan v3.3)

PIVOT to git2-rs
    ↓
Day 1-3: git2-rs C1-C8 attempts (faster because git2-rs is mature libgit2 binding)
    ↓
Day 3 evening: review
    ├── 8/8 PASS → Plan v3.2 amended to v3.3 with git2-rs dep; CO1.3.2 uses git2-rs
    └── < 8/8 PASS → escalate; consider Path A semantic-only (degraded fidelity)

PIVOT to Path A (semantic-only, no real git)
    ↓
Vec<Node> + per-node hash field + HEAD_t pointer (per Constitution Art 0.4 Path A)
    ↓
Phase E gate per Art 0.4 still requires Path B; this becomes a TEMPORARY measure until Phase E.
```

---

## § 5 Acceptance Audit Pack

After 5-day spike, if 8/8 PASS, deliver to dual audit:

```
spike/gix_capability/
├── SPIKE_RESULT_2026-XX-XX.md       # detailed C1-C8 results, perf numbers
├── results/
│   └── perf_log.txt                  # raw output of C6 benchmark
└── (all spike source code)
```

Codex audit prompt: "Verify each C1-C8 test actually exercises what its name claims. Check for cargo dependency hygiene. Confirm gix version pinned + no SemVer surprise lurking."

Gemini audit prompt: "Strategic check: does the spike result demonstrate gix is fit for purpose for the *entire* CO P1 timeline? Are there capabilities outside C1-C8 that v4 will need but the spike didn't test (e.g., remotes, gc, refs/notes for trace_matrix)?"

---

## § 6 What's Out-of-Scope for This Spike

Explicitly NOT tested in CO1.3.1; deferred to CO1.3.2+ or v4.x:

- **Network transport** (push/pull to GitHub or other remotes) — runtime_repo is local-only in v4; no network sync
- **Reference notes / git notes** — speculative future use; not blocking
- **Garbage collection** — runtime_repo is short-lived (per cell); no gc needed
- **Worktree operations beyond simple init** — not used in v4
- **SubModule support** — not used
- **Protocol v2 transport** — not applicable (no remote)

If any of these turn out to be required during CO1.3.2+ implementation, escalate via Plan amendment, not silent expansion.

---

## § 7 Conformance Test Stub (lands at end of spike Day 5)

```rust
// spike/gix_capability/tests/spike_conformance.rs (NOT in main src/)
#[test] fn c1_repo_init() { /* ... */ }
#[test] fn c2_multi_parent() { /* ... */ }
#[test] fn c3_tree_parent_read() { /* ... */ }
#[test] fn c4_concurrent_init() { /* ... */ }
#[test] fn c5_cas_blob_round_trip() { /* ... */ }
#[test] fn c6_perf_100_commits_under_1s() { /* ... */ }
#[test] fn c7_replay_deterministic() { /* ... */ }
#[test] fn c8_hooks_compat() { /* ... */ }
```

Plus one synthetic "end-to-end" test:
```rust
#[test] fn integration_5_modes_concurrent_init_no_collision() {
    // simulate Phase C 5-mode parallel runtime_repo init
}
```

When `cargo test --package gix_capability_spike` shows 9/9 PASS: spike succeeds.

---

## § 8 Honest Acknowledgements

What this pre-flight achieves:
- 8 named capabilities to verify, with explicit test methods
- 5-day strict time-box with daily checkpoints
- Clear fallback tree (gix → git2-rs → Path A)
- Dual audit gate before CO1.3.2 starts

What this pre-flight is honest about:
- I don't know gix's current API surface in detail; some capabilities may need adjustment when actually attempting
- Performance C6 number (100/sec) is a guess; real lower bound depends on workload
- Concurrent init C4 is the hardest; gix's concurrency model is documented but not battle-tested
- 5-day time-box may slip; the escalation path is the safety net

What this pre-flight does NOT do:
- Actually run any code (that's the spike itself, not pre-flight)
- Produce gix version pinning recommendation (depends on what's current at spike start)
- Pre-decide gix-vs-git2-rs (the spike result decides)

— ArchitectAI, 2026-04-27
