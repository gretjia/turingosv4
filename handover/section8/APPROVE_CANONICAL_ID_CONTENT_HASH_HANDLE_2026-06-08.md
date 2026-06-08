# §8 Decision Packet — Canonical Identity → Content-Hash Handle (LLM-visible / admission / signing / trust-root-pin crossings)

**Status**: **RATIFIED 2026-06-08, then CLOSED AS MEMBRANE-SATISFIES — NOT
EXECUTED.** The architect supplied the exact token `APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE`
with the directive to do a precise constitution-aligned design and execute
autonomously per the constitution. The precise design (workflow
`canonical-id-hash-design`) concluded that the canonical content-hash mint
reshape is **NOT constitution-required** (Art. 0.2 needs reconstructability, not
hashes; the LLM-hallucination target is already closed by the membrane #327/#328),
that every central mint is PINNED and the id-prefix carries load-bearing semantics
parsed by a PINNED **id=41 HALT** assertion (so a naive hash breaks historical
replay for ZERO constitutional gain), and that the constitution-faithful
(Karpathy-surgical, no-unnecessary-trust-root-churn) action is therefore to **NOT
execute the hashing**. The atom is closed as membrane-satisfies; the optional
AGENTS.md §12 role-out-of-id hygiene fix is deferred to its own per-atom §8. See
the §8 sign-off block and `handover/audits/CANONICAL_ID_HASH_DESIGN_VERDICT_2026-06-08.md`.
The original request text below is preserved verbatim for the record.

**Date**: 2026-06-08
**Base**: `origin/main` (idhash mirror worktree; clean git objects). The
predecessor read-only map is `handover/audits/EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08.md`.
**Risk class**: **Class 4 — canonical identity / wire-schema / signing-payload /
sequencer admission / trust-root pin.** Each of the 5 items below touches at
least one Class-4 restricted surface (`AGENTS.md §5–§6`): the `TxId` / `AgentId`
identity types in `src/state/q_state.rs` (genesis-pinned), the
`VerifySigningPayload` canonical digest + sequencer admission lookup in
`src/state/typed_tx.rs` / `src/state/sequencer.rs` (both genesis-pinned), the
hash-pinned `PromptCapsule` / `AttemptTelemetry` contract, or the genesis-pinned
trust-root file `src/runtime/librarian_broadcast.rs`. Per `AGENTS.md §5`, Class-4
requires explicit **per-atom §8** ratification before any implementation or ship.
Short replies (`go`, `ok`, `continue`, `can`, `完成`) do **not** constitute
Class-4 sign-off (`feedback_no_batch_class4_signoff`).

**Recommendation (operating posture):** ratify and execute as a small set of
surgical atoms (one per item), each with its OWN evidence + clean-context
Class-4 audit, AFTER the 9 autonomous render/parse-membrane fixes have landed and
proven out (§2). Do NOT batch all 5 into one PR — items 2 (signing-payload digest
shape) and 5 (trust-root pin rehash + signed tag) are the highest sub-risks and
each deserves its own audited atom. The autonomous-execution authorization
(2026-05-07) explicitly carves Class-4 + per-atom §8 OUT of auto-execute.

**Proposed §8 token** (the architect replies with this exact phrase to ratify):

```text
APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE
```

```text
Reject / defer option:
  REJECT-CANONICAL-ID-HASH-FOR-NOW   # keep canonical state keyed on the legacy strings;
                                     # the autonomous render/parse membrane already shows
                                     # hash handles, so the LLM-visible hallucination-bait
                                     # surface is closed without touching pinned canonical form.
```

**Authority chain**:
- Constitution: `constitution.md` **Art. 0.2 Tape Canonical 公理 (line 52 —
  所有信号必须可从 tape 重建)** — the canonical identity must remain
  reconstructable from tape; a deterministic `sha256`/`blake3` handle of the
  canonical id is replay-stable and reconstructable, an opaque slot index is not.
  **Art. III 信号的选择性屏蔽 (line 360; III.2 封装细节 line 383; III.4 屏蔽
  Goodhart line 413)** — read views delivered to the middle-layer black box must
  shield guessable / correlatable identity tokens; explicit `Agent_0` / raw
  `task_id` / `node_survive:{string}` strings in an LLM-visible projection are a
  hallucination-ingress + Goodhart surface that Art. III requires shielding.
- The two binding design constraints (user directive, 2026-06-08), recorded in
  `handover/audits/EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08.md` lines
  14–22: (1) kernel/canonical-tape model-GENERIC (separate track, see §7); (2)
  **all id pass-through must be HASH-based — never explicit `id="1234"` /
  `Agent_0` / `slot_0` in any LLM-visible path; the handle itself must be a
  `sha256(descriptor)`, not a slot index** (`feedback_hash_ids_not_explicit`).
- The read-only ground-truth map this packet operationalizes:
  `handover/audits/EXPLICIT_ID_HALLUCINATION_EXPOSURE_AUDIT_2026-06-08.md`
  ("CLASS-4 findings" section, items 1–5).
- Pinned identity / wire / signing / admission surfaces (verified this packet):
  - `src/state/q_state.rs:71` `pub struct TxId(pub String);` (genesis-pinned)
  - `src/state/q_state.rs:67` `pub struct AgentId(pub String);` (genesis-pinned)
  - `src/state/typed_tx.rs:1215-1216` `node_survive_event_id(work_tx_id) ->
    EventId(TaskId(format!("node_survive:{}", work_tx_id.0)))` (genesis-pinned)
  - `src/state/typed_tx.rs:998-1010` `VerifySigningPayload { … target_work_tx:
    TxId … }` + `canonical_digest()` (genesis-pinned)
  - `src/state/sequencer.rs:2152` `q.economic_state_t.stakes_t.0.get(&verify.target_work_tx)`
    verify-peer admission lookup (genesis-pinned)
  - `src/runtime/prompt_capsule.rs:83-84` `prompt_context_hash: Hash` (sha256 of
    canonical visible-context bytes) + `src/runtime/attempt_telemetry.rs:309`
    `pub prompt_context_hash: Hash` (UNpinned files, but a hash-pinned capsule
    contract)
  - `src/runtime/librarian_broadcast.rs` (genesis-pinned trust-root file:
    `grep -c '"src/runtime/librarian_broadcast.rs"' genesis_payload.toml` = **1**)
- Constitution binding: `AGENTS.md §5–§6, §9, §14`; `CLAUDE.md §3–§4`;
  `feedback_trust_root_pin_trap`; `feedback_class4_cannot_hide_in_class3`;
  `feedback_no_workarounds_strict_constitution`;
  `feedback_no_retroactive_evidence_rewrite`; `feedback_hash_ids_not_explicit`.

---

## §1. Decision statement (what this packet means)

**Make the CANONICAL identity reference form content-hash-based where it crosses
an LLM-visible / sequencer-admission / signing-payload path**, so there is ONE
hash identity end-to-end rather than an explicit guessable string at the
canonical layer and a hash handle only at the render layer.

Today the autonomous render/parse membrane (§2) can already SHOW a hash handle to
the agent while canonical state still keys on the legacy string
(`Agent_0` / raw `task_id` / `node_survive:{work_tx_id}` / raw `TxId`). That
closes the LLM-visible hallucination-bait surface. This packet is the SECOND,
optional, higher-risk step: reshape the CANONICAL form itself so the handle the
agent sees and the key the sequencer admits / the digest the signing payload
commits are the SAME content hash — no string-vs-hash dual identity that can
drift.

Per `feedback_defer_abstraction_until_second_impl` and the Karpathy surgical
posture, this is deliberately scoped to the 5 enumerated crossings, each as its
own atom; it is NOT a blanket "hash all ids" rewrite of q_state.

---

## §2. Precondition: what the autonomous render/parse membrane already did

This packet is requestable only because the LLM-visible hallucination surface is
ALREADY being closed at the unpinned render/parse membrane under the standing
`/goal` (the 9 AUTONOMOUS Class 1–2 findings in the audit). Do NOT re-authorize
those here; this packet covers ONLY the canonical-form change for the 5 Class-4
crossings.

| Membrane fix (autonomous, no §8) | Effect this packet relies on |
|---|---|
| Render layer shows a stable content-hash handle (`sha256`/`blake3` prefix) instead of `Agent_{i}` / raw `task_id` / `worktx-…` / `node_survive:{string}` in agent-facing projections. | The LLM never sees the guessable string, so the hallucination-bait surface is already closed at the membrane. |
| Parse layer resolves agent-echoed ids by exact handle match into the rendered candidate set and REJECTS on miss (drops loose-prefix + `.last()` fallback, e.g. `g1_market_live_agent.rs:509-519`; `state_update.rs:38/:134` equality gate). | An agent cannot bind a fabricated/typoed id to a canonical key; the canonical reshape does not have to defend against a guessed-string injection. |

Because the membrane already shows handles and rejects-on-miss, the **residual
risk of THIS packet is concentrated entirely in changing the CANONICAL form** —
the identity types, the signing-payload digest shape, the admission lookup key,
the hash-pinned capsule tuple, and the trust-root-pinned file. That is the only
new authority this §8 grants.

---

## §3. Allowed engineering actions (PER ITEM, only under the §8 token)

Each item below is its OWN atom. Each names the pinned surface touched and
whether it requires a typed-tx schema bump, a signing-payload digest-shape change,
and/or a trust-root rehash + signed `v4-ratify` tag (the highest sub-risks). All
are BLOCKED until the token in §8.

### A-ALLOW-1 — `node` identity → content-hash handle (item 1)
- **Surface**: `src/sdk/market_context.rs:134` renders `nid = work_tx_id.0` into
  the `- node {nid}: pool_yes=…` line the agent reads. The CANONICAL node identity
  is `TxId` (`src/state/q_state.rs:71`, **genesis-pinned**), echoed back and
  resolved through `node_survive_event_id` →
  `EventId(TaskId(format!("node_survive:{}", work_tx_id.0)))`
  (`src/state/typed_tx.rs:1215-1216`, **genesis-pinned**), and the echoed node is
  resolved in sequencer admission against `stakes_t` keyed by `TxId`.
- **Action**: make the canonical node reference a deterministic
  `sha256(work_tx_id)` handle wherever it crosses the LLM-visible / admission
  boundary, so the rendered handle == the admitted key.
- **Sub-risk**: touches the `node_survive:{string}` EventId ENCODING in pinned
  `typed_tx.rs` and the `TxId` identity type in pinned `q_state.rs`. **No new
  signing payload**, but the `node_survive_event_id` encoding is a wire-format
  contract — if its byte shape changes, it is a **typed-tx/encoding wire change
  requiring a version bump** (the existing `node_survive:` prefix contract is
  asserted by `typed_tx.rs:5590` test). **No trust-root rehash** of a pinned
  src file is required IF the change lands in the unpinned membrane + a NEW
  unpinned helper and the pinned encoding is left byte-identical; if the pinned
  `node_survive_event_id` body itself must change, that is a pinned-file edit =
  trust-root rehash + signed tag.

### A-ALLOW-2 — `verify_peer.target_work_tx_id` → handle (item 2) — HIGHEST sub-risk
- **Surface**: `src/sdk/pending_peer_reviews.rs:96-99` renders
  `- work_tx {tx_id}: …` and instructs the agent to "Submit a `verify_peer`
  action against one of the above target_work_tx_ids". That echoed id becomes
  `VerifySigningPayload.target_work_tx: TxId`
  (`src/state/typed_tx.rs:1001`, **genesis-pinned**), which is folded into the
  canonical agent signature via `canonical_digest()` →
  `domain_prefixed_digest(DOMAIN_AGENT_VERIFY, self)`
  (`src/state/typed_tx.rs:1008-1010`). It is also the sequencer-admitted lookup
  key: `q.economic_state_t.stakes_t.0.get(&verify.target_work_tx)`
  (`src/state/sequencer.rs:2152`, **genesis-pinned**; admission step-3 rejects a
  target not in `stakes_t`, `typed_tx.rs:4022`).
- **Action**: accept a content-hash handle as the canonical `target_work_tx`
  reference form (both at admission lookup and inside the signed digest).
- **Sub-risk**: **REQUIRES a signing-payload digest-shape change** — altering the
  accepted reference form of `target_work_tx` inside `VerifySigningPayload`
  changes what `canonical_digest()` commits, which is a canonical-signing-payload
  change (TB-4 already version-bumped this payload once; another digest-shape
  change needs a **signing-payload version bump**). It is ALSO a **wire/admission
  change** in pinned `sequencer.rs` (the `stakes_t.get` lookup key form).
  Both `typed_tx.rs` and `sequencer.rs` are genesis-pinned → **trust-root rehash
  + signed `v4-ratify` tag** if the pinned bodies are edited. This is the most
  invasive of the five; recommend it be a standalone audited atom with a
  versioned `VerifySigningPaylodV{n+1}` wire and a migration that does NOT rewrite
  historical signed digests (§4, §5).

### A-ALLOW-3 — `agent_id`+`task_id` in the hashed visible-context tuple (item 3)
- **Surface**: `src/runtime/real5_roles.rs:430-441` serializes
  `(&request.agent_id, &request.role, &request.task_id, &request.head_t, …)` via
  `serde_json::to_vec` → `Cid::from_content(&canonical)`. That canonical byte
  sequence's sha256 IS `prompt_context_hash`
  (`src/runtime/prompt_capsule.rs:83-84`: "SHA-256 of the canonical-encoded
  visible context bytes … Matches `AttemptTelemetry.prompt_context_hash`"), and
  `AttemptTelemetry` PINS it (`src/runtime/attempt_telemetry.rs:309`
  `pub prompt_context_hash: Hash`). Both files are **UNpinned in genesis** but
  the capsule↔telemetry hash equality is a constitution-gated contract.
- **Action**: substitute the content-hash handle for `agent_id` / `task_id`
  INSIDE the hashed tuple, keeping the tuple field count + shape stable.
- **Sub-risk**: **No genesis trust-root rehash** (both files unpinned, pin-count
  0). But altering the hashed-field SHAPE breaks the hash-pinned capsule contract
  (`PromptCapsule.prompt_context_hash` must equal
  `AttemptTelemetry.prompt_context_hash`, gated by the
  `prompt_capsule_struct_field_count_is_exactly_seven` shape gate referenced at
  `prompt_capsule.rs:70`). This is a **capsule-contract / policy-version bump**:
  the change MUST bump `PromptCapsule.policy_version` so audit replays the right
  visibility policy, and MUST NOT retroactively recompute historical
  `prompt_context_hash` values on old tape (§4). No typed-tx wire change, no
  signing-payload change.

### A-ALLOW-4 — `Agent_{i}` minting → hash (item 4)
- **Surface**: `src/sdk/prompt.rs:508-512` (`test_prompt_surfaces_team_board`,
  the team-board render contract) surfaces `Agent_0 balance=…` lines; the
  canonical `AgentId` is minted as `AgentId(format!("Agent_{i}"))`
  (`src/runtime/bootstrap.rs:90` and `:288`; also the live bins
  `g1_market_live_agent.rs:321`, `lean_market_agent.rs:550`). `AgentId`
  (`src/state/q_state.rs:67`, **genesis-pinned**) is a pinned q_state MAP KEY:
  `BalancesIndex(pub BTreeMap<AgentId, MicroCoin>)` (`q_state.rs:297`),
  `ReputationsIndex(pub BTreeMap<AgentId, Reputation>)` (`q_state.rs:455`),
  `PerAgentState` map `agents: BTreeMap<AgentId, …>` (`q_state.rs:101`), and the
  sequencer admits/credits/debits against these maps.
- **Action**: mint the canonical `AgentId` as a deterministic
  `sha256(descriptor)` (e.g. `blake3(run_id || agent_index)`), NOT a sequential
  `Agent_{i}` slot index, so the minted id is non-guessable end-to-end.
- **Sub-risk**: changes the canonical `AgentId` MINTING form, which is a pinned
  q_state key across `balances_t` / `stakes_t` / reputation / sequencer
  admission. The minting sites are mostly UNpinned (`bootstrap.rs`, bins), but the
  `AgentId` TYPE lives in pinned `q_state.rs`. If only the minting call sites
  change (string → hashed string, same `AgentId(String)` type) there is **no
  pinned-file edit and no trust-root rehash**; if `AgentId`'s representation
  itself is changed, that is a pinned q_state edit = **trust-root rehash + signed
  tag**. No typed-tx schema change, no signing-payload change required for the
  string-content-only variant. Historical tape keyed on old `Agent_{i}` ids must
  NOT be rewritten (§4, §5).

### A-ALLOW-5 — raw `task_id` → handle in Librarian Notices (item 5)
- **Surface**: `src/runtime/librarian_broadcast.rs:704-705`
  `task_tags_for` collects raw `e.task_id` into `task_tags`; `class_label` /
  `cluster_id` are derived from raw task data (e.g.
  `cluster_id: format!("cluster:{class}")` at `:617`); all flow into
  `render_librarian_notices_section` (`:975`, header `"=== Librarian Notices ==="`
  at `:981`) — an agent-facing view (called at `:938-939`). The file is
  **genesis-pinned** (`grep -c '"src/runtime/librarian_broadcast.rs"'
  genesis_payload.toml` = **1**).
- **Action**: project task identity as a content-hash handle in the rendered
  notices instead of the raw `task_id` string.
- **Sub-risk**: because `librarian_broadcast.rs` is a genesis trust-root-pinned
  file, ANY edit to it **rehashes the trust-root pin and REQUIRES the rehash in
  the same commit + a signed `v4-ratify` tag** (`feedback_trust_root_pin_trap`,
  `feedback_squash_merge_orphans_ratification_tag`: prefer `gh pr merge --merge`,
  re-sign the tag over the merge commit). No typed-tx schema change, no
  signing-payload change. Render-only handle projection preserves canonical state.

**Sourcing constraint (binding):** the handle is a deterministic hash of the
canonical id (`sha256`/`blake3`), derived — no new hardcoded behavior parameter
(`CLAUDE.md §4`), no `f64` anywhere (these are id paths, integer/byte only).

---

## §4. Hard guards (binding even under the token)

- **G-GUARD-1 — deterministic, replay-stable handle.** The handle is a pure
  function of the canonical id: `handle = sha256(canonical_id)` (or `blake3`),
  same id → same handle on replay. No slot index, no counter, no randomness, no
  probabilistic model. The handle must be reconstructable from tape so Art. 0.2
  (line 52, 所有信号必须可从 tape 重建) holds.
- **G-GUARD-2 — no historical tape rewrite (`feedback_no_retroactive_evidence_rewrite`).**
  Old ChainTape / L4 / L4.E / CAS entries keyed on legacy strings
  (`Agent_{i}`, raw `task_id`, `node_survive:{work_tx_id}`, raw `TxId`) and old
  signed `VerifySigningPayload` digests / old `prompt_context_hash` values stay
  byte-identical. New rules apply GOING FORWARD only; a backward-compat /
  migration shim maps legacy↔handle WITHOUT mutating historical evidence.
- **G-GUARD-3 — signing-payload version bump if the digest shape changes
  (item 2).** Changing what `VerifySigningPayload.canonical_digest()` commits is a
  canonical-signing-payload change; it MUST land as a NEW versioned payload
  (`VerifySigningPayloadV{n+1}`) with a version discriminator, NOT a silent
  in-place mutation of the existing digest shape (TB-4 set the precedent of
  bumping this payload). Old digests verify under the old version.
- **G-GUARD-4 — capsule policy-version bump if the hashed tuple shape changes
  (item 3).** `PromptCapsule.policy_version` MUST bump so audit replays the right
  redaction/visibility policy; the
  `prompt_capsule_struct_field_count_is_exactly_seven` shape gate stays GREEN
  (field COUNT unchanged — only the byte CONTENT of `agent_id`/`task_id` becomes
  a handle). `AttemptTelemetry.prompt_context_hash` ==
  `PromptCapsule.prompt_context_hash` equality gate stays GREEN going forward.
- **G-GUARD-5 — trust-root rehash in the same commit + signed `v4-ratify` tag for
  any pinned-file edit (`feedback_trust_root_pin_trap`).** Mandatory for item 5
  (`librarian_broadcast.rs`, pin-count 1) and for item 1/2/4 IF the pinned
  `typed_tx.rs` / `sequencer.rs` / `q_state.rs` bodies are edited rather than the
  change being confined to unpinned membrane/helper modules. The rehash lands in
  the SAME commit as the byte change; the resulting trust-root change is anchored
  by its OWN signed tag, separate from this §8 token. Prefer `gh pr merge --merge`
  for trust-root PRs (`feedback_squash_merge_orphans_ratification_tag`).
- **G-GUARD-6 — prefer the unpinned membrane / nested unpinned submodule.** Per
  `feedback_unpinned_path_submodule_pattern`, prefer landing the handle logic in
  NEW `#[path] pub mod` submodules under an UNPINNED parent and at the render/parse
  membrane, keeping pinned files byte-identical, BEFORE resorting to a pinned-file
  edit. A pinned edit is only justified when the canonical encoding itself
  (e.g. `node_survive_event_id` body, `VerifySigningPayload` field) must change.
- **G-GUARD-7 — fail-closed on resolution miss.** An agent-echoed handle that does
  not resolve to a known canonical id is REJECTED at the parse membrane (already
  the autonomous behavior, §2); the canonical reshape must not reintroduce a
  loose-prefix or `.last()` fallback (`feedback_admission_fail_closed_default`).

---

## §5. Forbidden (even under the token)

- **No historical tape rewrite, ever.** No migration of old L4 ↔ L4.E, no
  recompute of historical signed digests or `prompt_context_hash` values, no
  re-key of old `Agent_{i}` / `task_id` map entries on existing tape
  (`feedback_no_retroactive_evidence_rewrite`). New form applies forward only.
- **No silent wire-schema change without a version bump.** Item 1's
  `node_survive_event_id` encoding and item 2's `VerifySigningPayload` digest are
  wire/signing contracts; any byte-shape change is a versioned bump, never an
  in-place mutation (`feedback_class4_cannot_hide_in_class3`).
- **No second hash/id authority.** One deterministic handle function reused
  everywhere; no per-call-site ad hoc hashing that can drift
  (`feedback_no_workarounds_strict_constitution`).
- **No slot-index "handle".** The handle MUST be `sha256(descriptor)`; a
  sequential index (`slot_0`, `Agent_0`) is exactly the hallucination-bait this
  packet removes (`feedback_hash_ids_not_explicit`).
- **No batching the 5 items into one §8 token consumption / one PR.** Each is its
  own atom with its own audit (`feedback_no_batch_class4_signoff`); item 2 and
  item 5 in particular must be standalone audited atoms.
- **No brand-on-tape coupling.** The model-GENERIC (no-brand) concern is a
  SEPARATE track (§7); this packet does not touch it.
- **No audit before runnable evidence.** Each atom needs a real run / gate-green
  before its clean-context Class-4 audit (`feedback_audit_after_evidence`).

---

## §6. Risk classification & FC trace

**Risk class: Class 4.** The change reshapes canonical identity reference form
across pinned identity types (`TxId`/`AgentId` in `q_state.rs`), a pinned
signing-payload digest (`VerifySigningPayload` in `typed_tx.rs`), a pinned
sequencer admission lookup (`stakes_t.get` in `sequencer.rs`), a hash-pinned
capsule contract (`PromptCapsule`/`AttemptTelemetry`), and a genesis trust-root
file (`librarian_broadcast.rs`). Class-4 cannot hide inside a Class-3 umbrella
(`feedback_class4_cannot_hide_in_class3`). Per `AGENTS.md §5`, explicit per-atom
§8 ratification is required before any implementation or ship.

**Per-item highest sub-risk (summary table):**

| Item | Render/echo site | Pinned canonical surface | typed-tx schema bump | signing-payload bump | trust-root rehash + signed tag |
|---|---|---|---|---|---|
| 1 node | `market_context.rs:134` | `typed_tx.rs:1215-16` (node_survive enc), `q_state.rs:71` (TxId) — both pinned | YES if `node_survive_event_id` encoding bytes change | no | only if pinned `typed_tx.rs` body edited |
| 2 verify_peer | `pending_peer_reviews.rs:96-99` | `typed_tx.rs:1001` (VerifySigningPayload), `sequencer.rs:2152` (admission) — both pinned | (admission/wire change) | **YES — digest shape** | **YES if pinned typed_tx/sequencer bodies edited** |
| 3 visible-context tuple | `real5_roles.rs:430-441` | `prompt_capsule.rs:83-84` / `attempt_telemetry.rs:309` — UNpinned but hash-pinned contract | no | no (capsule policy_version bump instead) | **no** (both files pin-count 0) |
| 4 AgentId mint | `prompt.rs:508-512`; mint `bootstrap.rs:90/288` | `q_state.rs:67` (AgentId) — pinned; pinned map keys `q_state.rs:101/297/455` | no | no | only if pinned `q_state.rs` body edited (string-content-only variant avoids it) |
| 5 librarian task_id | `librarian_broadcast.rs:704-705/617/938-939` | `librarian_broadcast.rs` — **genesis-pinned (pin-count 1)** | no | no | **YES — mandatory (file is pinned)** |

**FC trace:**
- **FC1a output edge / FC1 rtool·wtool read-view shielding** — explicit ids in
  agent-visible projections (`market_context`, `pending_peer_reviews`,
  `real5_roles` visible context, `prompt.rs` team board, `librarian` notices) are
  a hallucination-ingress surface on the FC1 read/write membrane; the canonical
  reshape makes the shielded handle the canonical form so render==canonical.
- **Art. III shielding (constitution.md line 360 / III.2 line 383 / III.4 line
  413)** — read views must shield guessable / correlatable / Goodhart-prone
  identity tokens; a content-hash handle satisfies III where an explicit slot
  string violates it.
- **Art. 0.2 Tape Canonical (line 52)** — the handle is a deterministic,
  tape-reconstructable function of the canonical id, preserving 所有信号必须可从
  tape 重建.
- **FC1 admission** — item 2's `target_work_tx` is the sequencer verify-peer
  admission key (`sequencer.rs:2152`); changing its accepted form is an admission
  change that must stay fail-closed (`feedback_admission_fail_closed_default`).

---

## §7. What this packet does NOT authorize

- It does NOT authorize any `src/` edit. Every action in §3 stays BLOCKED until
  the token in §8.
- It does NOT re-authorize the 9 autonomous render/parse-membrane fixes — those
  ship independently under the standing `/goal` without §8 (§2).
- It does NOT authorize a single batched PR for all 5 items — each is its own
  audited atom (§5).
- It does NOT itself re-pin any trust-root file or sign any `v4-ratify` tag; item
  5 (and any pinned-body variant of items 1/2/4) carries its own rehash-in-commit
  + signed tag at implementation time (G-GUARD-5).
- It does NOT touch the brand-on-tape / model-GENERIC track. Per the audit's
  "Coordination" section, `genesis_report.rs:62-92`
  `model_name`/`model_provider`/`agent_model_assignment` +
  `ModelAssignmentManifest` are a SEPARATE concern owned by the "Wire model id
  onto canonical ChainTape" session (`feedback_kernel_model_generic_no_brand`);
  this packet must not couple to it.
- It does NOT permit any `constitution.md` mutation (human sudo only).
- It does NOT change any FC node semantics — render==canonical handle reshape
  preserves the FC1/FC2/FC3 graph.

---

## §8. Architect ratification (to be filled verbatim at user)

**Status: AWAITING ARCHITECT RATIFICATION.** Per `feedback_no_batch_class4_signoff`,
a short reply (`go` / `ok` / `continue` / `can` / `完成`) is NOT Class-4 sign-off.
The architect must supply the EXACT token below for the canonical-identity reshape
to begin; otherwise the canonical form stays on the legacy strings and only the
autonomous render/parse membrane handles (already shipping under `/goal`) close the
LLM-visible surface.

```text
Ratify:
  APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE

Reject / defer:
  REJECT-CANONICAL-ID-HASH-FOR-NOW   # canonical state stays keyed on legacy strings;
                                     # autonomous membrane handles already close the
                                     # LLM-visible hallucination-bait surface.
```

**Recommended posture if ratifying:** execute as 5 separate surgical atoms (one
per item), AFTER the autonomous membrane fixes prove out, each with its own real
run + clean-context Class-4 audit. Sequence the lowest-risk first
(item 3 capsule policy-version bump, item 4 string-content mint, item 1 handle)
and the highest-risk last as standalone audited atoms (item 2 signing-payload
version bump; item 5 trust-root rehash + signed `v4-ratify` tag). Per-item §8 may
be confirmed individually if the architect prefers finer granularity than one
token for all five.

**Architect §8 sign-off (FILLED IN AT USER VERBATIM):**

- Verbatim quote: `APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE`
- Token consumed: `APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE` (granted 2026-06-08)
- Execution directive (verbatim, same session): *"你用workflow进行精确设计后，对其宪法后可以自主执行，不需要我决定，我的决定就是遵守宪法"* — i.e. do a precise (workflow) design, verify it against the constitution, then execute autonomously; the user's only decision is "obey the constitution."
- **OUTCOME after the precise constitution-aligned design (workflow `canonical-id-hash-design`): the atom is CLOSED as MEMBRANE-SATISFIES — the canonical content-hash mint reshape is NOT executed.** See the verdict doc
  `handover/audits/CANONICAL_ID_HASH_DESIGN_VERDICT_2026-06-08.md`. Constitution-aligned rationale (verified in source):
  1. **Art. 0.2 needs reconstructability, NOT content-hash ids.** Legacy ids
     (`worktx-…`, `Agent_i`, `node_survive:…`) are fully tape-reconstructable, so
     they already satisfy Art. 0.2. The §8 directive's real target — LLM
     hallucination of guessable ids — is the MEMBRANE's domain, and the membrane
     (#327/#328, gate `tests/constitution_no_explicit_id_to_llm.rs`) ALREADY closes
     it (explicit ids render as opaque sha256 handles; agent echoes validated by
     exact membership; no `last()` fallback).
  2. **Every central mint is PINNED** (`src/runtime/bootstrap.rs` pin 1,
     `src/runtime/adapter.rs` pin 1 — the corrected pin status) and the id-prefix
     carries LOAD-BEARING semantics parsed by PINNED consumers
     (`audit_assertions.rs:883 sandbox_prefix` feeding the **id=41 HALT** assertion
     `assert_a_chain_agent_ids_sandbox_prefixed:1019`; `audit_dashboard.rs:1076`
     role detection). Hashing the canonical mint = pinned edits + a re-signed
     `v4-ratify` tag for **ZERO constitutional gain**, and a naive hash WITHOUT a
     dual-form legacy fallback would make every historical chain replay HALT —
     a production-breaking regression. That violates Karpathy surgical + incurs
     unnecessary trust-root churn → the constitution-faithful action is to NOT do it.
  3. The only constitution-aligned residual is an OPTIONAL AGENTS.md §12 hygiene
     fix (move role/sandbox semantics out of the id-prefix into a structured field).
     Its high-value part edits the 2 PINNED parsers (`audit_assertions.rs`,
     `audit_dashboard.rs`) → that is a SEPARATE Class-4 atom and a DIFFERENT change
     than "canonical-id → content-hash"; it is deferred to its own per-atom §8
     (`feedback_class4_cannot_hide_in_class3`), not pulled in under this token.
- Per-item granularity: n/a — atom closed as membrane-satisfies; no item executed.
- Pinned-body posture: no pinned-body edit performed; the membrane is unpinned.
- Date: 2026-06-08
- Branch at ratification: `claude/canonical-id-hash-design-verdict`
- Parent commit: `ed658750` (origin/main after the membrane PR #328)

---

`FC-trace: FC1a output edge + FC1 rtool/wtool read-view shielding (Art. III, constitution.md:360/383/413) + Art. 0.2 tape-canonical (constitution.md:52) — make the canonical identity reference form a deterministic content-hash handle (sha256/blake3 of the canonical id, replay-stable) where it crosses LLM-visible / sequencer-admission / signing-payload paths: node TxId/node_survive encoding (typed_tx.rs:1215-16, q_state.rs:71), VerifySigningPayload.target_work_tx digest + stakes_t admission (typed_tx.rs:1001, sequencer.rs:2152), prompt_context_hash visible-context tuple (real5_roles.rs:430-441 → prompt_capsule.rs:83-84 → attempt_telemetry.rs:309), AgentId minting (q_state.rs:67, bootstrap.rs:90/288), and librarian notices task_id (librarian_broadcast.rs:704-705, genesis-pinned). No FC node semantics change; render==canonical. Class-4 (canonical identity / wire-schema / signing-payload / admission / trust-root pin); per-atom §8 required; item 2 needs a signing-payload version bump and item 5 needs a trust-root rehash + signed v4-ratify tag; no historical tape rewrite per no-retroactive-evidence-rewrite; no implementation until APPROVE-CANONICAL-ID-CONTENT-HASH-HANDLE supplied.`

**End of Canonical Identity → Content-Hash Handle §8 decision packet (AWAITING ARCHITECT RATIFICATION; Class-0 documentation only — authorizes nothing; the 5 Class-4 crossings stay BLOCKED until the token is supplied; the 9 autonomous render/parse-membrane fixes ship independently under the standing /goal).**
