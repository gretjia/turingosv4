# OBS: CO1.8 v1 DEFERRED — r1 dual audit found foundational interface contradiction

**Date**: 2026-04-29 (session-3, post capability-first pivot)
**Origin**: Codex r1 VETO/HIGH + Gemini r1 CHALLENGE/HIGH on `handover/specs/CO1_8_MATERIALIZER_v1_2026-04-29.md` (commit `6cc5cc9`)
**Status**: spec NOT patched to v1.1; deferred until either (a) CO1.7.5 ships transition bodies giving sequencer state-pass parameter, or (b) materializer interface redesigned to take prior State by reference, not just `prior_root: &Hash`.

## Why this OBS exists (not r2 patch cycle)

Per **2026-04-29 capability-first pivot** (LATEST.md session-3): no more spec-craft round-2/round-3 cycles. Audits earned their cost in r1 by surfacing a real architectural contradiction; the appropriate response is to **archive the finding** and route around CO1.8 until the precondition (CO1.7.5 transition bodies) lands.

## Codex r1 P0 #1 — sprint graph overclaim

Spec § 0.1 line 22 cites SPRINT line 109 as authority for "CO1.8 is the unique unblocked next L-layer atom". But the same sprint graph at lines 106-108 carries `[CO1.7.5] ... blocks: CO1.8`. CO1.7.5 transition bodies are **future work gated on CO P2.x substrate** per CO1.7-extra verdict lines 101-104 + 157-160. CO1.8 is therefore **not** unblocked — it is downstream of CO1.7.5.

**Action**: do not respec; defer until CO1.7.5 lands.

## Codex r1 P0 #2 — apply() interface contradiction (foundational)

Spec § 0.2 line 30 says CO1.8 dispatches on `TxKind` only, "no per-variant body interpretation". But spec § 0.3 line 46 + test § 4.3 lines 159-164 require reputation changes from Verify/Challenge txs. Examining `src/state/typed_tx.rs:240-247`: `VerifyTx { target_work_tx, verifier_agent }` only. **Incrementing the target solver/agent's reputation requires prior Work/Claim state**, not just the discriminator + tx body.

The pure `apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, _>` cannot:
- Reconstruct prior state from `&Hash` alone (BTreeMap cache lookup is implicit I/O against a process-local mutable shared map)
- Survive process restart (no durability per § 0.4 #2)
- Match the frozen STATE invocation surface as written (7 sites at lines 399/466/560/624/700/758/852)

The "pure function" framing is internally inconsistent.

**Action**: when CO1.7.5 lands, redesign as `apply(prior_state: &State, tx: &TypedTx) -> Result<State, _>` — explicit state-pass, not implicit cache. Snapshot CAS interaction moves to caller (Sequencer).

## Gemini r1 P0 — Goodhart shield default-allow stub

Spec § 0.4 #4: `project_for_agent` ships as no-op filter (returns full view) until CO1.5 visibility tags. **Violates Inv 10** (Goodhart shield) by default. If early agents/tests build against this stub, they couple to evaluator internals; eventual CO1.5 integration becomes a breaking change.

**Action**: when CO1.8 is unblocked, mandate restrictive stub (returns only agent's own `PerAgentState`, public `TaskMarketEntry` fields, global budget/round). § 4.4 test must assert `oracle_seed`-class internal fields are dropped.

## What does NOT change

- WP § 5.L5 module structure stands (state_db / 6 indices / agent_view + read_tool)
- L5 SHA-256-of-snapshot semantics confirmed correct (Gemini Q1 PASS author's lean)
- L5 vs L6 boundary correct (Gemini Q3 PASS)
- 8-atom decomposition approved (Gemini Q5 PASS)

The materializer architecture is sound; only the v1 *interface* and *unblocked-next-atom* claims are wrong.

## What goes in next-CO1.8-spec when CO1.7.5 ships

When CO1.7.5 transition bodies are drafted, fold these constraints into CO1.8 v2:

1. § 0.1: cite CO1.7.5 as transitive dependency, not just CO1.7-extra
2. § 0.2: drop "no per-variant body interpretation" — materializer must be aware of variant-specific state needs (reputation needs Work/Claim history; price_signal needs market history; etc.)
3. § 2.1: signature redesign — `apply(prior_state: &State, tx: &TypedTx) -> Result<State, MaterializerError>` (state-by-reference, not hash-by-reference)
4. § 0.4 #4: replace no-op stub with default-deny stub
5. § 4.3: keep test, but adjust to feed prior State containing relevant Work tx history

## Audit cost recovered

Codex r1 + Gemini r1 single round = $10-20. Two real architectural P0s found (interface contradiction + sprint graph overclaim), one substantive P0 (Goodhart default-allow). **0 paper tigers in r1**. Audit-as-defect-finder was efficient at 1 round; pivoting to capability-first means we don't extract diminishing returns from r2/r3.

## Forward action

CO1.8 work is deferred. Next L-layer work resumes when CO1.7.5 has a draft. Meanwhile, capability-first work proceeds: reach v4-native MiniF2F solve by 2026-05-06 using existing pre-v4 evaluator + CO1.7-impl ledger work.
