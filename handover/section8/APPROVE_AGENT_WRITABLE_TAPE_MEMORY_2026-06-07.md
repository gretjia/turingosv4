# §8 Decision Packet — Agent-Writable Tape-Anchored Memory (Tier-2)

**Status**: AWAITING ARCHITECT RATIFICATION. NOT AUTHORIZED HERE. No
implementation happens until the architect supplies the token below. This
document is **Class-0 documentation only** — it describes a Class-3/4
agent-write-authority capability, requests per-atom §8 ratification, and
authorizes nothing by itself. The current Tier-1 work (system-authored
`SkillCapsule`) is **within authority and needs no token**; this packet exists
solely to fence the Tier-2 escalation OUT until the architect rules on it.

**Date**: 2026-06-07
**Branch**: `claude/tier1-skill-memory` (Tier-1 substrate worktree)
**Risk class**: **Class 3/4** — introduces a NEW agent write-authority
capability (an `archival_memory_write` tool letting an agent edit its OWN
tape-anchored memory namespace). New capabilities + any sequencer/capability/
admission surface touch are Class-4 candidates; per `AGENTS.md §5–§6` this
requires explicit **per-atom §8** ratification before any implementation or
ship. Short replies (`go`, `ok`, `continue`, `can`, `完成`) do **NOT**
constitute Class-4 sign-off (`feedback_no_batch_class4_signoff`).

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-AGENT-WRITABLE-TAPE-MEMORY
```

**Authority chain**:
- Tier-1 substrate (precondition, already live): `src/runtime/skill_capsule.rs`
  (SYSTEM-authored, AGENT-read-only `SkillCapsule`; CAS-resident, chained via
  `previous_capsule_cid`, anchored on `refs/chaintape/cas`); gate
  `tests/skill_capsule_tier1_memory.rs`.
- Tier-1 consolidation source: `src/runtime/autopsy_capsule.rs`
  (`cluster_autopsies` → `TypicalErrorSummary`, shielded, tape-derived).
- Taint substrate (a required guard): `src/predicate_admission/arg_taint.rs`
  (`arg_taint_v1`, `LabeledArg`, `has_tainted_privileged_flow`).
- Tape-canonical axiom: `constitution.md` Art. 0.2 (lines 52–95) — "所有信号
  必须可从 tape 重建"; canonical store = tape/CAS, parallel ledgers are derived
  views only.
- Constitution binding: `AGENTS.md §5–§6, §9, §12, §14`; `CLAUDE.md §4`;
  `constitution.md` Art. 0.2 + FC1/FC2/FC3.

---

## §1. Decision statement

**Should an AGENT be granted authority to WRITE its own tape-anchored memory
namespace?**

The decision under request is the introduction of an `archival_memory_write`
tool: a wtool (or equivalent privileged call) that lets a running agent author,
amend, or append entries to a memory namespace that is its OWN — i.e. the agent
edits the memory it will later read, rather than the SYSTEM distilling that
memory from failure/feedback evidence.

This is the **Tier-2** capability. It is categorically distinct from the
already-live Tier-1 path:

| | Tier-1 (LIVE, within authority) | Tier-2 (THIS packet, NOT authorized) |
|---|---|---|
| Author | `SkillAuthor::System` (stamped at the only write entry point) | the AGENT itself |
| Write trigger | system consolidation of shielded `TypicalErrorSummary` evidence | an agent tool call (`archival_memory_write`) |
| Agent surface | read-only `project_for_agent` → owned, immutable projection | a write/mutate tool the agent invokes in-loop |
| Capability needed | none new (system already has CAS `put` authority) | a NEW agent write-authority capability |
| Constitutional status | within authority (additive, system-authored) | NEW capability → needs a sub-article + per-atom §8 |

**The recommendation is to NOT grant this capability at this time.** The
constitutional default for a new agent capability is closed: an agent-writable
memory path is not derivable from any existing article, and self-authored
memory is the canonical attack surface for reward-hacking / self-poisoning
(an agent can write a "rule" that licenses its own future shortcut). Tier-1
already delivers the legitimate benefit (distilled reusable rules feeding the
next session's boot context, FC3-N43) WITHOUT handing the agent write authority.
Tier-2 should remain fenced out until the architect both (a) amends the
constitution with a sub-article that names agent write-authority as a granted
power, and (b) ratifies this packet's hard guards.

---

## §2. Precondition (why this packet is drafted now)

The precondition for even *considering* Tier-2 is now met: the Tier-1
system-consolidation path and the `SkillCapsule` substrate are **live**.

- `src/runtime/skill_capsule.rs` exists: a `SkillCapsule` is CAS-resident,
  stored as `ObjectType::Generic` with `schema_id = "v1/skill_capsule"` (no
  pinned `cas/schema.rs` edit), chained via `previous_capsule_cid` (mirrors
  `MarkovEvidenceCapsule`), and the only write entry point
  (`consolidate_skill_capsule`) stamps `author = SkillAuthor::System` from
  shielded `TypicalErrorSummary` input.
- The agent read surface (`project_for_agent` → `AgentSkillProjection`) is owned
  and immutable; it cannot write back. The module deliberately exposes **no**
  agent-callable mutate path.
- Gate `tests/skill_capsule_tier1_memory.rs` proves the Tier-1 boundary
  non-vacuously (system-author stamp, read-only projection, tape
  reconstructability).

Because the Tier-1 substrate is the thing a Tier-2 write tool would build on
(same CAS object family, same `previous_capsule_cid` lineage), the architect now
has a concrete decision surface. Drafting this packet at substrate-live time —
rather than after someone has built the write tool — is the
`feedback_audit_after_evidence` discipline applied to a *capability* decision:
fence the escalation before it lands, not after.

---

## §3. The Tier-2 decision under request (what APPROVE would license)

If — and only if — the architect supplies `APPROVE-AGENT-WRITABLE-TAPE-MEMORY`
AND lands the constitutional sub-article (§4 guard 2), the following becomes
permissible to design:

- **D-1 — an `archival_memory_write` tool.** A wtool that an agent may call to
  author/append a memory entry into a namespace scoped to that agent
  (an agent-authored sibling of `SkillCapsule`, e.g. a `SkillAuthor::Agent`
  variant or a separate `agent_memory` capsule schema_id).
- **D-2 — agent-authored lineage on the same tape chain.** The entry is
  CAS-resident and `previous_capsule_cid`-chained exactly like the Tier-1
  capsule, so it is fully reconstructable from CAS + tape (Art. 0.2). No
  filesystem store.
- **D-3 — the agent reads back its own authored memory** through a scoped,
  shielded projection (same shape as `AgentSkillProjection`), closing the
  self-authored read↔write loop the Tier-1 path deliberately leaves open.

Nothing in D-1/D-2/D-3 is authorized by this document. They are listed so the
architect knows exactly what the token unlocks.

---

## §4. Hard guards (binding on ANY future Tier-2 implementation)

These are non-negotiable preconditions on the eventual implementation. They are
listed in the packet so that a later implementer cannot quietly drop one.

1. **Canonical store = tape/CAS, never the filesystem (Art. 0.2).** An
   agent-authored memory entry must persist ONLY through `CasStore::put`
   (whose `refs/chaintape/cas` commit-chain IS the L4 anchor). No
   `std::fs::write`, no sidecar file, no global-latest filesystem pointer. The
   entry must be fully reconstructable from CAS + tape alone — every field
   derivable from the chain (CR-15.5). Any parallel store is a derived view and
   must carry a `assert_eq!(view, derive_from_tape(tape))` conservation test.

2. **Agent write-authority is a NEW capability needing a sub-article.** No
   existing constitutional article grants an agent the power to author its own
   memory. Per `AGENTS.md §5` Class-4, this capability cannot be conjured by an
   implementation PR; it requires the architect to **first amend the
   constitution** with a sub-article that explicitly names agent
   write-authority as a granted, bounded power (with its veto/withdrawal
   conditions). The §8 token here is necessary but not sufficient — the
   sub-article must land too.

3. **Capability-token scoped.** Any `archival_memory_write` call must be gated
   by an explicit, scoped capability token (the agent cannot self-grant). The
   token scopes the write to that agent's own namespace; cross-namespace writes
   are rejected. Canonical IDs and shadow IDs must not mix
   (`AGENTS.md §12`). The capability check is a sequencer-admission-class
   concern — touching it is itself a Class-4 surface and may need its own §8
   leg (Class-4 cannot hide in Class-3, `feedback_class4_cannot_hide_in_class3`).

4. **Taint / poisoning checks via `arg_taint_v1`.** Every
   `archival_memory_write` argument value must pass the value-level provenance
   analysis in `src/predicate_admission/arg_taint.rs`
   (`arg_taint_v1` / `has_tainted_privileged_flow`). A tainted value flowing
   into the privileged memory-write sink must hard-block — an agent must not be
   able to launder an untrusted/poisoned value into a future-read "rule". The
   memory-write key is a privileged sink and must be registered with
   `classify_write_key_sink`.

5. **Replayable from tape.** The full self-authored memory state at any point
   must be deterministically reconstructable by replaying the ChainTape + CAS
   (the audit verifier's `derive_from_tape`). No in-memory-only canonical state;
   no agent-authored entry that exists only in a live session.

6. **Read projection stays scoped + shielded (Inv 10 Goodhart shield).** The
   agent read-back (D-3) must omit raw failure bytes / private detail, exactly
   as `project_for_agent` does for Tier-1. Self-authored memory does not relax
   the shielding contract.

7. **No workaround closure.** A failing guard must not be downgraded to a skip,
   null pointer, empty-evidence path, or dashboard-only proof
   (`AGENTS.md §12`, `feedback_no_workarounds_strict_constitution`). If a guard
   cannot be satisfied, the capability is not shipped.

---

## §5. Risk classification & FC trace

**Risk class: Class 3/4.** Introducing agent write-authority is a new
capability. The capability gate and the memory-write admission path are
sequencer-admission-class surfaces (`AGENTS.md §6` restricted list:
`src/state/sequencer.rs`, capability surfaces). Per `AGENTS.md §5`, Class-4
requires explicit per-atom §8 ratification before implementation or ship, and
cannot hide inside a Class-3 umbrella.

**FC trace:**
- **FC1** runtime loop (`Q_t → rtool → input → Agent δ → output → predicates →
  wtool → Q_{t+1}`): an `archival_memory_write` is a NEW privileged `wtool`
  edge; it must pass the predicate/taint node before any write, never bypass it.
- **FC2** boot / capability admission: the capability-token check (guard 3) and
  the taint hard-gate (guard 4) live on the admission surface (L5/L7).
- **FC3** meta-architecture feedback → re-init (N43): the legitimate goal —
  distilled reusable rules feeding the next session's boot context — is ALREADY
  served by Tier-1 (system-authored). Tier-2 changes the *author* of that
  feedback from SYSTEM to AGENT, which is the constitutional escalation this
  packet fences.

**STEP_B protocol** (`feedback_step_b_protocol`): any capability/sequencer
change is a restricted-file change and must be developed on a parallel branch
with the gate suite GREEN before commit. If the capability touches the canonical
signing payload or typed-tx schema, that is a separate Class-4 surface with its
own ratification (not assumed here).

---

## §6. What this packet does NOT authorize

- It does NOT authorize any `archival_memory_write` tool, any
  `SkillAuthor::Agent` variant, or any agent-callable mutate path.
- It does NOT authorize a constitutional sub-article — that is a separate
  architect amendment action (guard 2).
- It does NOT touch the Tier-1 substrate, which is within authority and ships
  without a token.
- It does NOT permit a filesystem memory store under any circumstance
  (Art. 0.2 is absolute on canonical store = tape/CAS).

---

## §7. Architect ratification (to be filled at user verbatim)

**Status: AWAITING ARCHITECT RATIFICATION.** Reply with the exact token to
ratify. No implementation begins until the token AND the constitutional
sub-article (guard 2) both land; this is its own atomic Class-3/4 §8 cycle
(no batching).

```text
Grant agent write-authority (requires accompanying constitutional sub-article):
  APPROVE-AGENT-WRITABLE-TAPE-MEMORY

Reject / defer (recommended default — keep Tier-1 system-authored memory only):
  REJECT-AGENT-WRITABLE-TAPE-MEMORY
```

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: `<pending user verbatim §8 quote>`
- Token consumed: `<pending>`
- Constitutional sub-article landed: `<pending — required before any impl>`
- Ratified scope: `<granted with guards | rejected | deferred>`
- Date: `<pending>`
- Branch at ratification: `claude/tier1-skill-memory`
- Sign-off doc (created at user verbatim §8): `handover/section8/APPROVE_AGENT_WRITABLE_TAPE_MEMORY_§8_SIGN_OFF_2026-06-XX.md`

---

`FC-trace: FC1 archival_memory_write is a NEW privileged wtool edge that must pass the predicate/taint node (never bypass it) + FC2 capability-token + arg_taint_v1 hard-gate on the admission surface (L5/L7) + FC3-N43 the feedback→re-init author changes SYSTEM→AGENT (the escalation fenced here). Class-3/4 new-capability change; per-atom §8 + constitutional sub-article required; NOT authorized — no implementation until token supplied.`

**End of Agent-Writable Tape-Anchored Memory §8 decision packet (PENDING ARCHITECT RATIFICATION; Tier-2 NOT authorized; documentation only).**
