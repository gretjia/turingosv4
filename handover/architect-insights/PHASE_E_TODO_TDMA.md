# Phase E TODO: TDMA-Bounded → Path B Migration Obligation

**Status**: **SHIPPED 2026-05-22** (TB-TDMA-GENERATE-PHASE-E package; Atoms 19–26 on main)
**Authoritative reference**: `constitution.md` Art. 0.4 (lines 114–151)
**Ship report**: `handover/tracer_bullets/TB-TDMA-GENERATE-PHASE-E_ship_report_2026-05-22.md`
**Package §8**: `handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md`

---

## Status update (2026-05-22)

Phase E Path B migration is **COMPLETE**. The TB-TDMA-GENERATE-PHASE-E
package (Atoms 19–26) materially satisfies all 6 substrate-swap obligations
from constitution Art. 0.4:

| # | Obligation | Satisfied by |
|---|------------|--------------|
| 1 | Migrate MemoryTapeLedger → GitTapeLedger behind ImmutableTapeLedger trait seam | Atom 20 (PR #110) |
| 2 | Map TapeNode.hash to git commit OID | Atom 21 (PR #111) |
| 3 | Map verified_head to git HEAD ref (refs/tdma/verified_head) | Atom 22 (PR #112) |
| 4 | Map derive_latest_belief_state_from_tape to git log query | Atom 22 (PR #112) |
| 5 | Auto-gain Merkle DAG | git2-rs native content-addressing |
| 6 | Keep kernel transparent (MemoryKernel<L: ImmutableTapeLedger>) | Preserved end-to-end |

**Production cutover landed 2026-05-22T18:36Z (Atom 25, PR #116)**:
`turingos tdma run` and `turingos generate --tdma-bounded` both default to
GitTapeLedger. MemoryTapeLedger is retired from production paths (kept in
tests + explicit `--tape-backend=memory` emergency in-process rollback).

---

## Original phase-E obligations (preserved for historical reference)

Per constitution Art. 0.4, `Q_t = ⟨q_t, HEAD_t, tape_t⟩` is a version-controlled
three-tuple with three implementation paths (A semantic, B real-git, C hybrid).
TDMA-Bounded-RC1 (Atoms 0–18) explicitly shipped **Path A** (semantic
version-control substrate: `Vec<Node>` + hash + `verified_head` + `AttemptScope`
+ explicit `rtool`/`wtool` three-tuple signatures) per architect §8 on
2026-05-22.

The architect's long-term intent was **Path B (libgit2 / git2-rs)**. Phase E
was the forced gate. **As of 2026-05-22, Path B is the default.**

## What Phase E migration delivered

1. **Migrated `MemoryTapeLedger` → `GitTapeLedger`** behind the
   `ImmutableTapeLedger` trait. Karpathy K10 single-impl-trait concern
   resolved: the trait now has 2 real impls.

2. **Mapped `TapeNode.hash` to git commit OIDs.** Each `commit()` writes
   an 8-blob tree per plan §5 Atom 21 + canonical-JSON commit message.

3. **Mapped `verified_head` to `refs/tdma/verified_head`.** Failed proposals
   live as commits in the unified ledger_tail chain; verified_head only
   advances when set_verified_head is explicitly called by the kernel on
   StateAccepted.

4. **Mapped `derive_latest_belief_state_from_tape(scope)` to a git log
   walk** over the per-scope ref. Pure-function contract preserved
   (no sidecar, no mutable cache).

5. **Merkle DAG auto-gained.** git2-rs gives us:
   - Merkle DAG auto-validation
   - SHA-1 (and SHA-256 in newer git formats) hash chain
   - Immutable object store (git's content-addressable design)
   - `git log --reverse` audit-tape reconstruction without bespoke serializers

6. **Kernel remains transparent.** `MemoryKernel<L: ImmutableTapeLedger>`
   generic preserved end-to-end. Workers, distiller, rtool, CharterCore
   are unchanged — only the trait impl differs.

---

## Actual effort

Single session, 8 atoms, ~5 hours of focused execution. Faster than the
constitution.md:136–148 estimate of 6–8 weeks because (a) the spike work
in `spike/gix_capability/` had already validated git2-rs at 716 commits/sec,
(b) the `Git2LedgerWriter` in `src/bottom_white/ledger/transition_ledger.rs`
proved the git2-rs production pattern, and (c) the `ImmutableTapeLedger`
trait abstraction (added in TDMA-Bounded RC1 Atom 1) was already in place
as the seam.

---

## Outstanding follow-ups (NON-BLOCKING)

These are NOT obligations from constitution Art. 0.4 — they are engineering
hardening items deferred from the 2026-05-09 Phase E plan:

- **Phase E.1 (verbatim struct binding gate)** — engineering hardening; ships
  separately if/when needed.
- **Phase E.2 (atomic rollback witness gate)** — engineering hardening;
  Atom 21 + 22 roundtrip tests partially cover; full atomic-injection harness
  is a separate hardening atom.
- **Cumulative per-atom Codex+Gemini audit dispatch** — plan §8.2 specifies
  the table; execution deferred to next-session orchestration.
- **TB-ECON-E3-STRICT-EQ parallel package** — Karpathy K15 extracted; user
  chose parallel execution; awaits separate planning cycle.
- **Migrate the 3 standalone `tdma_rc1_deepseek_*` evidence binaries** to
  default GitTapeLedger — currently they still use MemoryTapeLedger via the
  Atom 18 thin-shim path; safe because they're disposable evidence-capture
  tools, but a future cleanup atom can update them.

---

## File closes

This TODO is **CLOSED as of 2026-05-22**. Future TB ship reports that mention
TDMA-Bounded may reference this file as the historical record of Phase E
delivery, but the obligation is no longer outstanding.

For new tape-substrate work post-Phase-E, see the ship report
`TB-TDMA-GENERATE-PHASE-E_ship_report_2026-05-22.md`.
