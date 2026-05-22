# Phase E TODO: TDMA-Bounded RC1 → Path B Migration Obligation

**Status**: OUTSTANDING (declared 2026-05-22 in TB-TDMA-BOUNDED-RC1)
**Authoritative reference**: `constitution.md` Art. 0.4 (lines 114–151)

Per constitution Art. 0.4, `Q_t = ⟨q_t, HEAD_t, tape_t⟩` is a version-controlled
three-tuple with three implementation paths (A semantic, B real-git, C hybrid).
TDMA-Bounded-RC1 explicitly ships **Path A** (semantic version-control substrate:
`Vec<Node>` + hash + `verified_head` + `AttemptScope` + explicit `rtool`/`wtool`
three-tuple signatures) per architect §8 on 2026-05-22.

The architect's long-term intent is **Path B (libgit2 / git2-rs)**. Phase E is
the forced gate where TDMA must migrate to Path B unless the architect issues
an explicit sudo lowering the fidelity requirement.

---

## What Phase E migration must deliver

1. **Migrate `MemoryTapeLedger` → `GitTapeLedger`** behind the existing
   `ImmutableTapeLedger` trait in `src/ledger.rs`. The trait IS the seam —
   addresses Karpathy K10 single-impl-trait concern by realizing the planned
   second impl.

2. **Map `TapeNode.hash` to git commit OIDs.** Each `commit()` becomes a
   `git commit -m <node payload sha>` against a runtime git repo dedicated to
   the run.

3. **Map `verified_head` to a git HEAD ref.** Failed proposals (verified=false)
   live on orphan refs or branch tips that the kernel never advances HEAD to.

4. **Map `derive_latest_belief_state_from_tape(scope)` to a `git log --grep` /
   `git for-each-ref` query.** The pure-function contract is preserved.

5. **Auto-gain Merkle DAG.** Path B retroactively gives us:
   - Merkle DAG auto-validation
   - 30-year hash collision resistance (SHA-256 git is standard)
   - Immutable object store (git's content-addressable design)
   - `git replay` audit-tape reconstruction without bespoke serializers

6. **Either: keep the kernel transparent to the swap.** Workers, distiller,
   rtool, CharterCore should not need to know which tape backend is in use.
   This is enabled by the `ImmutableTapeLedger` trait.

---

## Estimated effort

6–8 weeks per constitution.md:136–148 Path B sizing.

---

## Re-affirmation discipline

This file SHALL exist until Phase E migration ships. Every TB ship report that
mentions TDMA-Bounded MUST re-affirm this obligation. Atom 8 ship report
(in feature/tdma-bounded-rc1) carries this re-affirmation by reference.

Until Phase E lands, the properties Phase B retroactively grants are
**PROMISES enforced by the kernel's hard asserts + 9 gates** — not yet
structural. The kernel SHALL continue to pass the 9 gates + 5 real-tape
invariants on every release tag.
