# GA §8 Sign-Off Template — TDMA-Generate + Phase E libgit2 Cutover

**Package**: Atoms 19–26 (TDMA-Generate-Wire-Up + Phase E libgit2 substrate)
**Predecessor**: TB-TDMA-BOUNDED-RC1 (Atoms 0–18; already on main)
**Ship report**: `handover/tracer_bullets/TB-TDMA-GENERATE-PHASE-E_ship_report_2026-05-22.md`
**Package §8**: `handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md`

---

## What the architect is signing

By signing this template (via committing it to main with the signature block
below filled in), the architect ratifies:

1. **All 8 atoms (19–26) merged to main between 2026-05-22T16:58Z and 2026-05-22 evening UTC**.
2. **Constitution Art. 0.4 Path B obligations (all 6) materially satisfied**:
   - GitTapeLedger replaces MemoryTapeLedger as the default substrate
   - TapeNode.hash maps to git commit OID
   - verified_head maps to refs/tdma/verified_head
   - derive_latest_belief_state_from_tape walks per-scope git refs
   - Merkle DAG auto-gained via git2-rs native content-addressing
   - Kernel remains transparent (MemoryKernel<L: ImmutableTapeLedger> generic preserved)
3. **Production cutover complete**: `turingos generate` and `turingos tdma run`
   both default to TDMA-Bounded + git backend; legacy single-pass path deleted.
4. **Karpathy + Constitution plan-level audit violations remediated**:
   - C12 (KILL-gen-1 wording) → tightened
   - K14 (--legacy permanent flag) → flag dropped, legacy path deleted
   - K15 (Atom 23 E.3 scope mixing) → extracted to parallel `TB-ECON-E3-STRICT-EQ` package
5. **Explicit override of `feedback_no_batch_class4_signoff` for THIS PACKAGE ONLY**.
   Future Class 3+ work returns to per-atom §8 unless covered by a new package directive.
6. **Outstanding obligations** acknowledged:
   - Cumulative per-atom Codex+Gemini audit dispatch (deferred to next session)
   - TB-ECON-E3-STRICT-EQ parallel package (separate orchestration)
   - Phase E.1 + E.2 engineering hardening (separate atoms if needed)

## Acknowledgment of Path A retirement

After this signature, `MemoryTapeLedger` is **retired from production code paths**.
It survives ONLY in:
- Test fixtures (`cargo test --lib tdma_runner`, etc.)
- Explicit `--tape-backend=memory` flag for in-process emergency rollback
- The standalone `tdma_rc1_deepseek_*` evidence binaries (which the next
  hardening pass may also migrate)

Rollback of the cutover (if ever required) is via `git revert` of PRs #109–#116 in reverse order, per the standard irreversible-cutover discipline used elsewhere in this codebase.

## Architect signature

```
Signed-by: <architect name + git identity>
Signed-on: <date YYYY-MM-DD>
Git-commit-of-this-file:  <commit OID hex>
Plan-revision-acknowledged: Revision 2 (post-audit, 2026-05-22)
```

After signature, the architect commits this file with the populated block
above; that commit IS the GA ratification.
