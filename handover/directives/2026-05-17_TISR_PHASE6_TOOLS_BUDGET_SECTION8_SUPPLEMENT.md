# TISR Phase 6.0/6.1 — Tools Budget Section 8 Supplement

**Date**: 2026-05-17
**Status**: RATIFIED
**Supplements**:
- `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md` (Phase 6.0/6.1 scope §8 packet)
- `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_SIGN_OFF.md` (Phase 6.0/6.1 scope §8 sign-off)
**Activates**: pre-condition #7 (WebSearch / 实施工具预算授权) listed in `handover/research/interaction_substrate/50_deliverables/00_UNIFIED_CLI_SPEC.md` §13.
**Branch at ratification**: `worktree-tisr-2026-05-17`
**Parent commit**: `4e71cca2` (TISR-001 hygiene fixes + Phase 6.0/6.1 §8 ratification)

This document is the Section 8 supplement that ratifies the tools budget for the already-ratified Phase 6.0/6.1 scope. It does NOT extend or modify the Phase 6.0/6.1 scope; it only authorizes the tools needed to execute that scope.

---

## 1. Architect Section 8 Verbatim (Tools Budget)

The user-as-architect provided the following explicit multi-clause authorization on 2026-05-17:

```text
我批准 TISR Phase 6.0/6.1 使用必要的 WebSearch/WebFetch/本地实现与验证工具预算，
仅限 narrow CLI MVP + local generative UI IR spike；
不授权 Phase 7 Web、CAS schema、typed_tx、sequencer、signing、Trust Root rehash
或任何 Class 4 surface 修改。
```

## 2. Multi-clause §8 Analysis

Per CLAUDE.md §10 + AGENTS.md §5 multi-clause §8 analysis, the verbatim contains:

| Element | Found | Value |
|---|---|---|
| Named act | ✓ | "批准" (approve / ratify) |
| Named target | ✓ | "TISR Phase 6.0/6.1" |
| Named resource | ✓ | "必要的 WebSearch/WebFetch/本地实现与验证工具预算" |
| Named scope binding | ✓ | "仅限 narrow CLI MVP + local generative UI IR spike" (matches original PACKET Section 1) |
| Named exclusions | ✓ | "Phase 7 Web、CAS schema、typed_tx、sequencer、signing、Trust Root rehash 或任何 Class 4 surface 修改" (matches original PACKET Section 7) |
| Escalation rule | (inherits from original PACKET) | Section 3/4 禁区 → 必须停止 |

Structurally equivalent to the canonical multi-clause §8 sign-off forms (e.g., Stage C P-M4 "签字，同意后续执行" / Stage C overall "授权自主执行直到polymarket全部落地").

## 3. Ratified Tools (within Phase 6.0/6.1 scope only)

### 3.1 Research tools
- **WebSearch** — default allowed; reasonable budget (~50 query order of magnitude per atom; verbatim phrasing "必要的" suggests not unlimited)
- **WebFetch** — default allowed; for official spec / docs / reference reading

### 3.2 Local implementation tools (within Section 4 allowed paths of original PACKET)
- `cargo build` / `cargo check` / `cargo test --workspace --no-fail-fast`
- `rustfmt` / `cargo clippy --workspace --tests --no-deps`
- `git` operations on `worktree-tisr-2026-05-17` or `codex/tisr-phase6-cli` branches
- Read / Edit / Write tools on allowed paths (per original PACKET Section 4)
- `clap` crate (CLI parser, default-allowed dependency addition)
- Standard Rust ecosystem dependencies needed for the narrow scope (e.g., serde, tokio, axum is NOT allowed since axum belongs to Phase 7 Web)

### 3.3 Verification tools
- `bash scripts/run_constitution_gates.sh`
- `make constitution`
- `turingos_dev` (dev evidence sidecar per CLAUDE.md §11 / AGENTS.md §10)
- `Skill: runner-preflight` (before any runner script mutating `handover/evidence/`)
- Clean-context Codex audit per AGENTS.md §9 default single audit (before ship claim)

## 4. Explicit Exclusions (still NOT authorized)

This tools budget supplement does NOT extend the Phase 6.0/6.1 scope. The following remain forbidden (consistent with the original PACKET Section 3.E + Section 7):

- **Phase 7 Web tools**: axum, tower-http, React/TypeScript frontend toolchain, websocket implementation, JSON-RPC dispatch
- **CAS schema modification tools**: any tool that produces edits to `src/bottom_white/cas/schema.rs` ObjectType enum
- **typed_tx tools**: any tool that produces edits to `src/state/typed_tx.rs` (variant additions, signing payload changes)
- **Sequencer tools**: any tool that produces edits to `src/state/sequencer.rs` admission paths
- **Signing tools**: canonical signing payload modification tools, new signature type generators
- **Trust Root tools**: rehash scripts, pinned-file updates, Trust Root manifest mutations
- **Gemini DeepThink dual audit**: NOT default since 2026-05-14 per AGENTS.md §9; require explicit user request to enable
- **Any tool that produces or implies Class 4 surface modification**

If implementation discovers that the narrow scope requires any excluded tool, implementation must stop and a new independent Section 8 packet must be drafted and ratified, per original PACKET Section 6 escalation rule.

## 5. Effect on TISR Phase 6 Pre-condition Status

Per `handover/research/interaction_substrate/50_deliverables/00_UNIFIED_CLI_SPEC.md` §13:

| # | Pre-condition | Status before this supplement | Status after this supplement |
|---:|---|---|---|
| 1 | G-Phase 收口完成 (SG-G overall §8) | ❌ pending | (bypassed for Phase 6.0/6.1 by §8 packet worktree-isolation carve-out) |
| 2 | REAL-13 ship | ❌ in flight | (same — bypassed) |
| 3 | REAL-BCAST-1 ship | ❌ in flight | (same — bypassed) |
| 4 | REAL-13A ship | ❌ in flight | (same — bypassed) |
| 5 | typed_tx.rs no schema drift since 2026-05-17 | ❌ uncontrollable | (same — bypassed; §8 excludes typed_tx modifications anyway) |
| 6 | Phase 6 separate charter §8 ratification | ✅ RATIFIED 2026-05-17 | ✅ (unchanged) |
| 7 | WebSearch / implementation tools budget authorization | ❌ unauthorized | ✅ **RATIFIED by this supplement** |

**Net effect**: Phase 6.0/6.1 narrow scope is now fully unblocked. All 7 pre-conditions are either RATIFIED or bypassed by §8 worktree-isolation carve-out.

## 6. Implementation Authorization

After this supplement:

- TISR Phase 6.0/6.1 separate charter §8 + tools budget §8 both RATIFIED
- Implementation may proceed per original PACKET Section 5 Next Actions
- Tools budget is bounded by "必要的" verbatim — agent must justify each non-trivial WebSearch / dependency addition against narrow scope
- Any excluded-tool need → stop + new §8 packet (per original PACKET Section 6 escalation rule)

## 7. Next Actions (carry forward from original SIGN_OFF §5)

1. ~~Repair TISR Phase 0-5 documentation hygiene blockers or carry as non-shipping debt~~ ✅ **Completed in commit `4e71cca2`**
2. Decide implementation branch:
   - **Option A**: Stay on `worktree-tisr-2026-05-17` (current; mixes research docs + implementation)
   - **Option B**: Branch off to `codex/tisr-phase6-cli` (separation of concerns; recommended per architect-style cleanliness)
3. Open `turingos_dev` run before evidence-bearing implementation work
4. Implement smallest Phase 6.0/6.1 slice (suggested first slice options):
   - **Option α**: `turingos init` (Class 1; pure filesystem; ~150 LOC) — lowest risk, fastest happy-path
   - **Option β**: `turingos audit dashboard` wrap (Class 1; thin lib wrap; ~100 LOC + lib extraction) — fastest user value, demonstrates lib-化 pattern
   - **Option γ**: `turingos task view` wrap of `lean_market view-task` (Class 1; thin wrap; ~80 LOC) — preserves lean_market compatibility
5. Run ratified packet's verification gates:
   - `cargo test --workspace --no-fail-fast`
   - `bash scripts/run_constitution_gates.sh`
   - `git diff --check` clean
6. Request clean-context Codex audit before any ship claim

## 8. Activation Decision

This supplement is active because the user verbatim contains:

- a named act ("批准") and named target ("TISR Phase 6.0/6.1") ✓
- a named resource scope ("必要的 WebSearch/WebFetch/本地实现与验证工具预算") ✓
- a named scope limit matching the already-ratified §8 packet ("仅限 narrow CLI MVP + local generative UI IR spike") ✓
- explicit exclusions matching the already-ratified §8 packet's Section 7 ✓
- inherited escalation rule from original PACKET ✓

Structurally equivalent to canonical multi-clause Class-4 §8 forms per CLAUDE.md §10 multi-clause analysis.

**End of TISR Phase 6.0/6.1 Tools Budget §8 Supplement.**
