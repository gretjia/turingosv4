# Explicit-ID Hallucination Exposure Audit (READ-ONLY)

**Date**: 2026-06-08
**Scope**: where EXPLICIT (non-hash) ids leak into LLM-visible paths, per two user
directives (2026-06-08). **No code was changed** — this is the ground-truth map
that precedes deciding what to fix.
**Base**: `origin/main` 4974dd17. **Method**: 4 read-only lens scans
(prompt-ingress / tape-payload / agent-output / model-run-id) + adversarial
dedup/classify synthesis; top findings re-verified against source by the
orchestrator.

## Two binding design constraints (user, 2026-06-08)

1. **Kernel/canonical-tape model-GENERIC** — no LLM brand name
   (deepseek/Qwen/SiliconFlow…) or model-specific invocation detail on
   ChainTape/CAS. Brand→invocation mapping lives in an EXTERNAL sidecar. See
   `feedback_kernel_model_generic_no_brand`.
2. **All id pass-through must be HASH-based** — never explicit `id="1234"` /
   `Agent_0` / `slot_0` style ids in any LLM-visible path; explicit/guessable ids
   are LLM-hallucination bait. The generic model handle itself must be a
   `sha256(descriptor)`, not a slot index. See `feedback_hash_ids_not_explicit`.

"LLM-visible" = (R) interpolated into a prompt the agent reads, (W) parsed from
LLM output and used as a canonical id, or (S) rendered into an agent-facing view.
Content hashes (`Cid`, `*_root`, self-addressing capsule ids) are the SAFE target
form and are NOT flagged.

## Result: 14 distinct LLM-visible explicit-id surfaces (zero false positives)

- **9 AUTONOMOUS** (Class 1–2, unpinned, no §8) — render/parse-layer fixes.
- **5 CLASS-4** (pinned surface / canonical identity / wire-schema / signing
  payload / admission) — need per-atom §8.

**Key architectural lever:** the RENDER layer can show hash handles while
canonical state still keys on the legacy strings. So the autonomous render/parse
redactions ship INDEPENDENTLY of, and BEFORE, the Class-4 canonical-identity
reshape.

### Exposure map (R=prompt-read, W=agent-echoed-as-canonical, S=report-shown)

LIVE-WIRED (id reaches a real LLM call today):
- `g1_market_live_agent.rs` (live DeepSeek bin): R×2 (`Agent_{i}` self-id :455;
  `node_tx` 16-char prefix :428-430) + W×1 (`parent_node` prefix-match +
  `node_tx_ids.last()` fallback :509-519).
- `market_external_agent_current_kernel.rs` (live external bin): R×1
  (`true-suite-market-{run_id}` :484).
- `rtool.rs` level_4 → `memory_kernel.rs` O1 worker-retry prompt: R×1 (`task.id`
  at MinimalHeadOnly :222-227).
- `state_update.rs` worker-output parse (SHARED judge adapter): W×1 (agent-echoed
  `task_id` persisted to StateAccepted tape with only a non-empty check :38/:134).

LATENT-AFFORDANCE (LLM-facing schema/render contract, not yet wired to a live bin
in this worktree — close before any runner wires them live):
- `protocol.rs` `AgentAction.node` :20 (W), `AgentAction.target_work_tx_id` :62 (W)
- `sdk/market_context.rs` :134-136, `sdk/pending_peer_reviews.rs` :96-99,
  `sdk/your_position.rs` :108-150 (render fns currently test-only-wired)
- `sdk/prompt.rs` team_board :508-512 (fed by `lean_market_agent.rs:550` `Agent_{i}`)
- `runtime/real5_roles.rs` :430-441, `runtime/tc_agent_view.rs` :80-100 (agent_id/
  task_id in hashed visible-context tuple), `runtime/librarian_broadcast.rs`
  :705/938 (raw task_id → notices; PINNED file)

## AUTONOMOUS findings (Class 1–2, fixable without §8 — render/parse membrane)

1. **`state_update.rs:38` (+validate :134) — HIGHEST RISK, LIVE, W.** Agent-echoed
   `StateUpdate.task_id` is persisted to the StateAccepted tape via the shared
   `parse_prefix_json`/judge adapter (Nesbitt/Swebench/market all route through it)
   with ONLY an `is_empty()` check — no equality vs the system task id. **Verified
   in source.** Remedy: stamp the system `task.id`, or assert
   `header.task_id == system task.id` and reject mismatch (canonical
   `AttemptScope.task_id` already uses the system id). Pure additive equality gate,
   no wire change.
2. **`g1_market_live_agent.rs:509-519` — LIVE, W.** LLM-echoed `parent_node` is
   loose-prefix-matched to a `TxId` AND falls through to `node_tx_ids.last()` on
   miss. **Verified in source.** A fabricated/typoed prefix silently binds the
   wrong node or last(). Remedy: require exact handle match, REJECT on miss (drop
   the `last()` fallback).
3. **`g1:455` `Agent_{i}` self-id (R)** — replace the per-prompt identity token
   with a per-run opaque handle `blake3(run_id||agent_index)[..8]`; canonical
   `AgentId` stays internal.
4. **`g1:428-430` `node_tx` 16-char prefix (R)** — render nodes by an accepted
   content-hash/Cid prefix instead of the truncated `worktx-` string.
5. **`market_external_agent_current_kernel.rs:484` `true-suite-market-{run_id}` (R)**
   — agent outputs only direction+amount; drop the id from the prompt or use a
   content-hash handle.
6. **`rtool.rs:222-227 → memory_kernel.rs:574-608` `task.id` MinimalHeadOnly (R)**
   — omit the bare id (`task.prompt` already carries the task) or render a
   content-hash handle; only the level_4 fallback exposes it.
7. **`sdk/protocol.rs:20` `AgentAction.node` (W, latent)** — resolve agent choice
   by index/short-handle into the rendered chain; reject any node not byte-equal to
   the snapshot the agent was shown.
8. **`sdk/protocol.rs:62` `AgentAction.target_work_tx_id` (W, latent)** — resolve
   by index into the rendered candidate set / reject any target not in the agent's
   rendered candidates BEFORE building `VerifyTx` (close the affordance at the
   unpinned membrane; the downstream sequencer lookup is pinned — keep the fix at
   the membrane).
9. **`sdk/your_position.rs:108-150` `tx_id`/`event_id`/`node_id` (R/S, latent)** —
   render each position by a stable content-hash handle in the unpinned renderer.

## CLASS-4 findings (need per-atom §8 — pinned / canonical-identity / wire / signing / admission)

1. **`sdk/market_context.rs:134-136`** — render-only redaction is autonomous, but
   making the CANONICAL `node` identity hash-based (vs `node_survive:{string}`)
   touches `TxId` (q_state.rs pin) + the node-survive encoding + sequencer
   admission resolution.
2. **`sdk/pending_peer_reviews.rs:96-99`** — the echoed `verify_peer.target_work_tx_id`
   is a sequencer-admitted lookup key folded into `VerifySigningPayload.target_work_tx`
   (canonical digest); changing the accepted reference form is a wire/admission +
   signing-payload change.
3. **`runtime/real5_roles.rs:430-441`** — `agent_id`+`task_id` are serialized into
   the canonical visible-context tuple whose sha256 IS `prompt_context_hash`
   (capsule contract; AttemptTelemetry pins it). Altering the hashed-field shape
   breaks a hash-pinned capsule contract; AgentId/TaskId types pinned in q_state.rs.
4. **`sdk/prompt.rs:508-512` team_board (`Agent_{i}`)** — render-only redaction is
   autonomous, but minting agent identity as a hash changes the canonical `AgentId`
   minting form (a pinned q_state key across stakes_t/balances_t/sequencer admission).
5. **`runtime/librarian_broadcast.rs:705/938`** — raw `task_id` → task_tags/
   class_label rendered into `=== Librarian Notices ===`; the file is genesis-pinned,
   so projecting task identity as a handle rehashes a trust-root file.

**Recommendation:** package the Class-4 items as ONE §8 "canonical-id → content-hash
handle" atom, executed AFTER the autonomous render/parse redactions land and prove
out. The render layer can show handles while canonical state still keys on legacy
strings, so the autonomous layer is shippable independently.

## Recommended sequencing

1. `state_update.rs` equality gate (most dangerous; agent value persisted to tape).
2. `g1:509-519` drop `last()` fallback + exact handle match.
3. The 4 live R-only ids (`g1:455`, `g1:428-430`, `market_external:484`,
   `rtool` level_4) → opaque/hash handles or drop.
4. Latent affordances (`protocol.rs` node/target_work_tx_id + test-only render fns)
   — close before any live wire-up.
5. §8 Class-4 canonical-id → content-hash atom (the 5 above), after 1–4 land.

## Coordination — brand-on-tape is a SEPARATE track

`genesis_report.rs:62-92` `model_name`/`model_provider`/`agent_model_assignment` +
`ModelAssignmentManifest` are NOT an id-hallucination ingress surface — grep
confirms NO prompt-render path surfaces them (they reach JSON/CAS provenance only).
They are the **generic-no-brand** concern, owned by the session
**"Wire model id onto canonical ChainTape"**. That session must keep brand strings
OFF the canonical tape and carry a generic `sha256(descriptor)` handle (NOT a slot
index), per both constraints above.

## Confirmed-SAFE (correctly NOT flagged)

`verified_head=accepted.hash` (all rtool levels except the level_4 task.id),
all `*_cid.hex()`/`state_root`/`ledger_root`/`prompt_context_hash`/`node.hash`
(content addresses = the SAFE target form), external-dataset ids in benchmark bins
(math sample_id, toolbench query_id, mind2web node ids — the legitimate answer
space the LLM must echo), keystore lookup keys, CAS schema-id discriminators,
observe-only dashboards. No row was dropped as an outright false positive; every
flagged row is genuinely LLM-visible AND genuinely a non-hash id.

`FC-trace: FC1a output_edge (state_update agent-echoed id), FC1 rtool/wtool read-view shielding (Art.III) — explicit ids in agent-visible projections are a hallucination-ingress surface; remedy = content-hash handles at the render/parse membrane (autonomous) + a canonical-id→hash §8 atom (Class-4). No FC node semantics change; render-layer redaction preserves canonical state.`
