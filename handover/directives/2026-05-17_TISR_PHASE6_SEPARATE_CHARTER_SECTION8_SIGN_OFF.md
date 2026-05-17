# TISR Phase 6.0/6.1 — Section 8 Sign-Off

**Date**: 2026-05-17.
**Status**: RATIFIED.
**Ratified packet**:
`handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`.
**Branch at ratification**: `worktree-tisr-2026-05-17`.
**TISR parent commit**: `ff71406c`.

## 1. Architect Section 8 Verbatim

The user-as-architect provided the following explicit multi-clause
authorization:

```text
我以架构师身份批准 TISR Phase 6.0/6.1 separate charter Section 8:
授权在 `worktree-tisr-2026-05-17` 或后续 `codex/tisr-phase6-cli` 分支上实施
Phase 6.0/6.1 narrow CLI MVP + local generative UI IR spike。
授权范围仅限本 packet Sections 1-6 的 allowed paths 和 gates。
不授权 Phase 7 Web、CAS schema、typed_tx、sequencer、signing、Trust Root rehash、
或任何 Class 4 surface 修改。
若实现触碰 Section 3/4 禁区，必须停止并另开独立 §8 packet。
```

## 2. Ratified Scope

This sign-off activates only:

1. TISR Phase 6.0/6.1 narrow CLI MVP.
2. Local generative UI IR spike.
3. Implementation on `worktree-tisr-2026-05-17` or a later
   `codex/tisr-phase6-cli` branch.
4. The allowed paths and gates listed in the ratified packet Sections 1-6.

## 3. Explicit Exclusions

This sign-off does not authorize:

- Phase 7 Web.
- CAS schema changes.
- `typed_tx` changes.
- sequencer admission changes.
- signing or canonical signing payload changes.
- Trust Root rehash.
- any Class 4 surface modification.

If any excluded surface becomes necessary, implementation must stop and a new
independent Section 8 packet must be drafted and ratified.

## 4. Activation Decision

The ratification is active because the user message contains:

- a named act: "批准" / "授权";
- a named scope: "TISR Phase 6.0/6.1 narrow CLI MVP + local generative UI IR
  spike";
- an allowed execution branch/worktree;
- explicit allowed-path and gate binding;
- explicit Class 4 and Phase 7 exclusions;
- an escalation rule requiring a separate Section 8 packet on restricted
  surface contact.

## 5. Next Actions

After this sign-off:

1. Repair known TISR Phase 0-5 documentation hygiene blockers or explicitly
   carry them as non-shipping debt before implementation.
2. Create or switch to a dedicated implementation branch if leaving the TISR
   research worktree.
3. Open a `turingos_dev` run before evidence-bearing implementation work.
4. Implement the smallest Phase 6.0/6.1 slice.
5. Run the ratified packet's verification gates.
6. Request clean-context Codex audit before any ship claim.

**End of TISR Phase 6.0/6.1 Section 8 sign-off.**
