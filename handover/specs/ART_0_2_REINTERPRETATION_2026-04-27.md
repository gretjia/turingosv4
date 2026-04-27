# Art 0.2 Operation Item 5 — Reinterpretation Proposal

> **Date**: 2026-04-27
> **Purpose**: Resolve tension between user's D-VETO-6 = B (don't record reject details on tape) and Codex CHALLENGE that flagged Constitution Art. 0.2 line 64 as currently requiring it.
> **Status**: PROPOSAL. User decides: (a) accept reinterpretation (lighter), (b) accept reinterpretation + cosmetic edit (clearer), or (c) full Art V.3 mini-amendment (heaviest).
> **Authority**: Per Constitution Art. V.3, any constitutional amendment requires explicit human architect sudo. ArchitectAI cannot self-enact.

---

## § 1 Current Text (verbatim, constitution.md line 64)

> **5. 失败分支（vetoed / parse-fail / Lean-rejected）必须以 `kind=AgentProposal, verified=false, reject_class=...` Node 形态进入 tape——失败也是状态；Phase B 之前在 `bus.graveyard: HashMap` 中的设计是 anti-pattern**

Two distinct claims compressed into one sentence:
- **Claim 5-A**: "失败也是状态" — failure information must be tape-canonical (reconstructible from tape, not from sidecar)
- **Claim 5-B**: "失败分支必须以 `kind=AgentProposal, verified=false, reject_class=...` Node 形态进入 tape" — the specific implementation: each rejected proposal becomes its own tape Node

Codex CO P0.7 review accurately identified that the Claim 5-B literal reading conflicts with user's D-VETO-6 = B (no raw rejected payloads on tape, Satoshi-style).

## § 2 Two Readings of Art 0.2 Item 5

### Reading X (literal, current code-level reading)
"Every rejected proposal → exactly one tape Node with `verified=false`."

Pros:
- Most direct reconstruction: full reject history visible per-tape-position
- Anti-Goodhart clear: agent can read its own past rejects

Cons:
- Tape size explodes if agent is noisy
- Raw rejected payload contents on tape → potential leakage of agent's failed reasoning into other agents' read views (unless visibility-filtered, but that's an extra layer)
- Conflict with Satoshi-style minimal canonical history
- Is what `bus.graveyard` was IN SPIRIT trying to do, just in wrong location (sidecar)

### Reading Y (purpose-level reinterpretation)
"Failure SIGNAL must be tape-reconstructible. The IMPLEMENTATION may be: (a) one Node per reject (Reading X), (b) bounded summary stamped on next accepted tx + terminal summary tx for no-accept runs (D-VETO-6 = B), (c) any other reconstructible mechanism."

Pros:
- Matches Claim 5-A "失败也是状态" purpose (signal is on tape)
- Allows D-VETO-6 = B implementation (system-stamped bounded retry metadata)
- Smaller tape footprint
- Avoids raw payload leak

Cons:
- "Reconstructible from tape" may lose granularity (e.g., can't see the EXACT failed payload)
- For PPUT-CCL the loss is acceptable: only failure CLASS frequencies matter, not contents

## § 3 ArchitectAI Recommendation

**Adopt Reading Y** as official interpretation. This requires:

### Option A: Reinterpretation only (no constitution edit)
- Add an interpretive note to `handover/architect-insights/CONSTITUTION_INTERPRETATIONS_2026-04-27.md` (NEW)
- This file acknowledges the two readings and locks Reading Y as canonical
- Constitution.md text stays unchanged
- All downstream code (CO1.7+CO1.9 retry metadata + TerminalSummaryTx) implements Reading Y
- **Pro**: no Art V.3 amendment ceremony, fastest path
- **Con**: future readers may default to Reading X literal interpretation; interpretation file might drift from constitution

### Option B: Cosmetic edit to Art 0.2 line 64
Replace the current single sentence with two clearer sentences:
> ~~5. 失败分支（vetoed / parse-fail / Lean-rejected）必须以 `kind=AgentProposal, verified=false, reject_class=...` Node 形态进入 tape——失败也是状态；Phase B 之前在 `bus.graveyard: HashMap` 中的设计是 anti-pattern~~
>
> 5. 失败信号必须可从 tape 重建（"失败也是状态"）。具体实现可以是：每个 reject 单独成 Node，或在下一条接受的 work_tx 上 stamp 系统签名的 bounded `RejectedAttemptSummary`（系统不是 agent 自报），或在零 accept 的 run 末尾 emit `TerminalSummaryTx`。原 `bus.graveyard: HashMap` 设计是 anti-pattern（不能从 tape 重建）；上述任一系统签名机制均合规。Phase B 之前所有平行账本同样违规。

- **Pro**: text clearly accommodates Reading Y; future readers don't need separate interpretation file
- **Con**: requires Art V.3 cp workflow + signature + Trust Root SHA refresh

### Option C: Full new amendment (heaviest)
- Write a new "Art 0.2.1 Failure Signal Reconstructibility Clarification" sub-section
- Cite this proposal doc
- Require formal Codex+Gemini dual audit on the amendment
- **Pro**: maximum legal clarity for future audits
- **Con**: ~1-2 days slower; overkill for what is essentially a clarification

---

## § 4 ArchitectAI Default Recommendation

**Option B (cosmetic edit + cp workflow)**.

Reasoning:
- Reading Y is materially the right purpose-level reading
- Option A leaves a future drift risk
- Option C is heavyweight for a clarification, not a substantive policy change
- Option B is reversible (one cp workflow) if user later decides Option A or C

**If user is in a hurry**: Option A acceptable, with TODO marker in CONSTITUTION_INTERPRETATIONS file to upgrade to B at the next constitution amendment cycle.

---

## § 5 Implementation impact

If Reading Y is locked (any of A/B/C):

1. `RejectedAttemptSummary` (per `STATE_TRANSITION_SPEC_v1` § 1.4) is the canonical failure-signal carrier
2. `TerminalSummaryTx` (per spec § 1.5) handles no-accept runs
3. Both are **system-signed**, NOT agent-signed (Codex's correct trust-boundary fix)
4. L6 derived signal index reconstructs `failure_class_histogram` from these tape entries via `derive_l6_from_tape(tape) → SignalIndex`
5. Conformance test `derive_l6_from_tape(tape) == runtime_sidecar_snapshot` proves equivalence
6. `bus.graveyard` is finally retired in atom CO1.1.4-pre1 along with `completion_tokens: 0` literal

If Reading X is locked instead (user chooses to keep literal reading):

1. Every rejected proposal → tape Node with `verified=false, reject_class, payload_cid`
2. Tape size grows ~10x for noisy agents (estimate from Phase A logs)
3. Visibility filter at L5 must hide other agents' reject payloads from current agent
4. D-VETO-6 user decision overridden; user must reverse to "log reject details" path

ArchitectAI strongly recommends Reading Y because:
- It honors user's D-VETO-6 intent
- It matches Satoshi-flavored T+S analysis
- It still satisfies "失败也是状态" purpose
- It is implementable cleanly without Goodhart leak surface

---

## § 6 Decision required from user (on wake)

| Choice | Effect |
|---|---|
| **A** — interpret-only (no constitution edit) | Fastest; Reading Y locked via interpretation file; some future drift risk |
| **B** — cosmetic edit Art 0.2 line 64 (default rec) | Reading Y permanent; one cp workflow + SHA refresh |
| **C** — new sub-section Art 0.2.1 amendment | Maximum clarity; ~1-2 day audit ceremony |
| **X** — keep literal Reading X (override D-VETO-6) | User reverses earlier "不记录拒绝细节" decision; D-VETO-6 path discarded |

ArchitectAI default: **B**. Awaiting user choice.

— ArchitectAI, 2026-04-27
