# TDMA-Generate-Wire-Up + Phase E libgit2 Cutover — Package §8 Directive

**Class**: 3 (multiple Class 3 atoms; one Class 2; one Class 0)
**Scope**: feature branches `feature/tdma-atom19-generate-wireup` through `feature/tdma-atom26-ship-report`
**Binding**: package-level §8 (explicit override of `feedback_no_batch_class4_signoff` for this package only)
**Architect**: user (gretjia)
**Date**: 2026-05-22
**Predecessor**: TB-TDMA-BOUNDED-RC1 (Atoms 0–18, §8 directive
`handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md`)

---

## 1. Architect direction (verbatim, 2026-05-22 plan-mode dialog)

> "继续推进剩下的（4 = wire turingos generate through TDMA-Bounded；5 = Phase E libgit2),
> 为了避免低效，用我们之前成熟的multi-agents方式，你作为orchestrator,做好详细计划，
> 计划要详细到可以派sonnet和你认为合适的思考深度模型去做基础的代码工作，所以指令要详细，
> 验收标准要清晰。另外在方案出来后，安排宪法agent和Karpathy agent进行对抗后再输出
> 完整最终方案。然后我批准后执行"

**Decisions** (multi-question dialog 2026-05-22):
1. Atom 19 landing mode → "如果你是 Karpathy 怎么选" → opt-in `--tdma-bounded` flag, with explicit default-flip atom inside this same package (no permanent flag debt).
2. Phase E gates → "首先要符合宪法，然后我不要凑活，我要一次到位，我也要听 Karpathy 意见" → execute the 6 constitutional Path B obligations; skip 2026-05-09 plan's E.1/E.2 (engineering hardening, not constitutional); extract E.3 (real conservation defect) to a parallel single-atom package after Karpathy K15 scope-mixing finding.
3. Phase E scope → "Push to full production cutover (default GitTapeLedger)".
4. §8 ratification → "Bundle into a new package §8 directive" (this file).

## 2. Package contents — 8 atoms (post-Karpathy-revision)

| Atom | Scope | Class | FC trace |
|------|-------|-------|----------|
| 19 | `turingos generate --tdma-bounded` opt-in wire-up | 3 | FC1a-rtool, FC1a-predicate_pi, FC3-replay |
| 20 | `GitTapeLedger` skeleton + `run_proof_with_ledger` generic | 2 | FC1a substrate seam, FC3-replay |
| 21 | `GitTapeLedger` commit/retrieve roundtrip | 3 | FC1a, FC3-replay |
| 22 | `verified_head` + BBS derivation via git refs/log | 3 | FC1a, FC1b, FC3-replay |
| 23 | `turingos tape-migrate` subcommand | 2 | FC1a substrate migration, FC3-replay |
| 24 | `--tape-backend=git\|memory` opt-in | 3 | FC1a, FC2-N16 |
| 25 | Full production cutover, legacy path deleted | 3 | FC1a, FC2-N16 |
| 26 | Ship report + Phase E §8 packet + Path A retirement | 0 | — |

**Class 4 atoms**: NONE. Restricted §6 surfaces (kernel.rs, bus.rs, wallet.rs,
sequencer.rs, typed_tx.rs, cas/schema.rs) are NOT touched by any atom in this
package.

## 3. Constitution Art. 0.4 Path B obligation coverage

This package satisfies all 6 Path B obligations from constitution.md lines 130–148:

| # | Obligation | Atom |
|---|------------|------|
| 1 | Migrate `MemoryTapeLedger` → `GitTapeLedger` behind `ImmutableTapeLedger` trait seam | 20 |
| 2 | Map `TapeNode.hash` to git commit OIDs | 21 |
| 3 | Map `verified_head` to git HEAD ref | 22 |
| 4 | Map `derive_latest_belief_state_from_tape` to `git log --grep` query | 22 |
| 5 | Auto-gain Merkle DAG (git's native content-addressable storage) | 21 |
| 6 | Keep kernel transparent to substrate swap (no kernel-side changes) | 20–22 |

## 4. KILL criteria (9 total, all gates testable)

KILL-gen-1, KILL-gen-2, KILL-gen-3 (Atom 19);
KILL-git-1 (Atom 21); KILL-git-2, KILL-git-3 (Atom 22);
KILL-migrate-1 (Atom 23); KILL-cutover-1 (Atom 25).

See plan file `/home/zephryj/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md`
§3 for full kill-criteria text and gate mapping.

## 5. Extracted parallel package

The E.3 conservation strict-equality fix at `src/economy/monetary_invariant.rs`
was originally proposed as Atom 23 of a 9-atom package. Karpathy K15 audit
flagged scope mixing (FC1b-wtool surface, disjoint from this package's
FC1a-rtool/substrate surface). It has been extracted to a parallel single-atom
package `TB-ECON-E3-STRICT-EQ` that runs concurrently (user decision 2026-05-22).
That package gets its own §8 ratification, not this one.

## 6. Audit cadence (per atom)

Per AGENTS.md §14 table:

| Class | Pre-impl | Post-impl |
|-------|----------|-----------|
| 0 (Atom 26) | — | Final Codex audit |
| 2 (Atoms 20, 23) | — | Codex code review |
| 3 (Atoms 19, 21, 22, 24, 25) | Codex spec + Gemini cross-§ (parallel) | Codex clean-context + Gemini (parallel) |

All audits use the AGENTS §15 verdict domain `{NO-VIOLATION, VIOLATION-FOUND <clause> <file>:<line>, RECONSTRUCTION-FAILURE <path>, SECOND-SOURCE-DRIFT <view>}`.

Conservative resolution per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## 7. Plan-level audit findings (executed 2026-05-22 before this §8 was signed)

- **Constitution audit (Opus auditor)**: 1 VIOLATION-FOUND (C12 KILL-gen-1 wording) — REMEDIATED in plan revision 2.
- **Karpathy audit (Sonnet general-purpose)**: 2 VIOLATION-FOUND (K14 `--legacy` flag; K15 Atom 23 scope mixing) — both REMEDIATED in plan revision 2.

Plan revision 2 incorporates all three fixes; this §8 ratifies the revised plan.

## 8. Architect signature

This file IS the package-level §8 ratification. By committing this file under
the architect's git identity, the architect signs:

1. The 8-atom scope above.
2. The Class 3 / Class 2 / Class 0 classification per atom.
3. The explicit override of `feedback_no_batch_class4_signoff` for THIS PACKAGE
   ONLY. Future Class 3+ work returns to per-atom §8 unless covered by a new
   package directive.
4. The full plan at
   `/home/zephryj/.claude/plans/harness-orchestrator-multi-agent-agent-typed-diffie.md`
   (Revision 2, post-audit).

Architect: gretjia <zephryj@icloud.com>
Date: 2026-05-22

## 9. Ship-gate criteria (RC1.Δ → GA)

See plan §9 — 11 criteria. Until ALL GREEN, merging to main is BLOCKED per
K-HARDEN-7. PR-only flow enforced.
