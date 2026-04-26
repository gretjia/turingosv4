# Tape Canonical Axiom Audit — TuringOS v4 (read-only)

**Auditor**: JudgeAI sub-agent (Claude `auditor` subagent, read-only)
**Date**: 2026-04-26
**Scope**: `/home/zephryj/projects/turingosv4/src/` + `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/` + `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/tests/`
**Standard under audit**: Pending Art. 0 *Tape Canonical Axiom* — "all signals must be reconstructible from tape; RunCostAccumulator-style parallel ledgers may only be derived views."

> Companion to `TAPE_CANONICAL_AUDIT_2026-04-26_CODEX.md` (independent codex:codex-rescue review). Cross-validation across two independent auditors + Claude's own first-pass.

---

## Executive Summary — most severe violations first

1. **V-01 dormant `Node.completion_tokens`** (most severe: schema lies). Every production tape Node carries `completion_tokens: 0`. The field exists on the canonical struct + serialised to WAL, so an external auditor reading a JSONL replay will see zeros and conclude the run was free. The real cost lives in the parallel `RunCostAccumulator`. **Phase E reproducibility: BROKEN**. Even a perfect WAL cannot recover C_i.
2. **V-02 RunCostAccumulator is the canonical cost ledger, not the tape** (architectural). `prompt_tokens`, `completion_tokens`, `tool_tokens`, `proposal_count`, `failed_branch_count` exist only in process memory. They are emitted ONCE into the JSONL aggregate row and then the process exits. No tape Node, no WAL record, no per-node attribution. **Phase D ArchitectAI per-node cost attribution is impossible** — only the run total survives.
3. **V-03 Failed branches are unrecoverable from tape**. `acc.failed_branch_count` counts every parsed proposal that didn't verify. Vetoed appends are dropped at `bus.rs:186/199/206`; OMEGA-rejected payloads appear only as a `record_rejection` graveyard class label (`veto:forbidden`/`err:tactic_linarith`/...) which is also a separate `HashMap`, NOT on the tape. Even the parsed-but-rejected payload text vanishes. The 5-step compile loop's "Logging" claim collapses for the rejected 95%+ of attempts.
4. **V-04 `MarketCreate` and `MarketResolve` event types are defined but never emitted** (`src/ledger.rs:153-154` enum variants — production tape contains zero such records). `bus.rs:289` and `kernel.rs:155-178/83-103` mutate market state without any ledger append. Reconstructing market price history from WAL alone is impossible.
5. **V-05 Investment amount + direction are dropped on the tape**. `bus.rs:244` writes `EventType::Invest` with `detail: None`. Amount, direction (long/short), shares received are all gone. Plus the swarm path bypasses the bus entirely (`evaluator.rs:1318-1322` calls `bus.kernel.buy_yes/buy_no` directly), so even the empty Invest event is skipped on production runs.
6. **V-06 Wall-clock T_i bracket is parallel ledger**. `RunWallClock` uses `Instant` (process-monotonic, non-serialisable). `Node.created_at: u64` is **seconds resolution** (`bus.rs:264-267`); a 5-second LLM call and a 5.999-second LLM call get the same timestamp on the tape. Phase B B3 millisecond budget assertions cannot be reconstructed from tape alone.
7. **V-07 `synthetic_short_circuit` is an evaluator-only flag**. When SIMULATE_ROLLBACK_AT_TX_50 fires, no tape Node and no ledger event records that the calibration treatment was active. The flag exists only on the JSONL row. WAL replay → reproduces a 50-tx run, no signal that the next 150 tx were synthetically vetoed.
8. **V-08 Mr-tick and Boltzmann routing are off-tape**. The "Art. IV mermaid: clock → mr → tape" comment promises tick events on tape; the implementation emits them only to FC_TRACE stderr+`info!()` log. Boltzmann parent selection (per-tx routing decision) is never recorded — replay with the same seed reconstructs it, but only if `BOLTZMANN_SEED` was logged out-of-band (it currently lives in env, propagated to the JSONL row, NOT to the tape).

The unifying pattern: **the JSONL aggregate row has become the de facto canonical artifact, and the tape is a degraded shadow.**

---

## Inventory Table

| # | File:lines | Violation type | Phase E recovery | Phase D per-node attr. | Notes |
|---|---|---|---|---|---|
| V-01 | `src/ledger.rs:24` (def); `src/bus.rs:268`, `src/kernel.rs:255` (test), `src/sdk/actor.rs:181, 255` (test), `src/wal.rs:118, 171` (test only); also `src/ledger.rs:355` (test) | dormant_tape_field | **No** | **No** | `Node.completion_tokens` is hardcoded `0` on every production write. WAL faithfully serialises the zero. |
| V-02 | `experiments/minif2f_v4/src/cost_aggregator.rs` (entire module) | parallel_ledger | **No** (totals only) | **No** | Per-LLM-call costs aggregate run-wide; no per-Node attribution preserved. |
| V-03 | `src/bus.rs:182-208` (Phase 0 vetoed branches drop payload); `src/bus.rs:444-450` (`record_rejection` writes to `graveyard: HashMap`); `evaluator.rs:1278, 1281, 1527, 1545, 1547` (rejected payloads → tool_stdout estimate, then dropped) | parallel_ledger + missing_provenance | **No** | **No** | Failed branch count survives in JSONL; the actual rejected payload text + author + timestamp + parent is permanently lost. |
| V-04 | `src/ledger.rs:153-154, 168-169` (enum + Display defined); never appended in `bus.rs` or `kernel.rs` | missing_provenance | **No** | n/a | Markets created/resolved silently. WAL has no MarketCreate/MarketResolve lines, so price-history reconstruction requires replaying every Invest event in order — but the LP seed and bounty market open-event are also missing. |
| V-05 | `src/bus.rs:244-245` (Invest event with `detail: None`); `experiments/minif2f_v4/src/bin/evaluator.rs:1318-1336` (direct `bus.kernel.buy_yes`/`buy_no` bypassing bus) | reproducibility_break + missing_provenance | **Partial** (only existence of investment is captured, not amount/direction/shares) | **No** | Two layers: (a) Invest event has no amount, (b) the production swarm code path skips even that. |
| V-06 | `experiments/minif2f_v4/src/wall_clock.rs` (entire module); `src/ledger.rs:23` (`created_at: u64` seconds) | parallel_ledger | **No** (ms resolution); Yes (s resolution, lossy) | **Partial** | T_i is wall-clock-of-machine-now, not derivable from `Node.created_at` deltas at sub-second granularity. |
| V-07 | `experiments/minif2f_v4/src/rollback_sim.rs` (entire); `experiments/minif2f_v4/src/bin/evaluator.rs:778-826` (short-circuit path) | hack_proxy + reproducibility_break | **No** | n/a | The flag emits an FC_TRACE event but no tape entry. Module header explicitly enumerates 5 dimensions where the synthetic path is non-equivalent (cost, wall-clock, WAL ledger sequence, predicate traversal, tx_count). |
| V-08a | `experiments/minif2f_v4/src/bin/evaluator.rs:836-916` (mr tick: log+FC_TRACE only) | parallel_ledger + blockchain_reservation_gap | **No** (FC_TRACE optional + stderr) | n/a | Comment claims "clock → mr → tape per Art. IV" but the implementation never writes to `bus.kernel.tape`. |
| V-08b | `experiments/minif2f_v4/src/bin/evaluator.rs:1055-1057` (`boltzmann_select_parent`); `evaluator.rs:697` (seed only in env→JSONL row) | missing_provenance | **No** unless seed is on tape | n/a | Routing is reproducible-with-seed but the seed isn't in the tape; depends on JSONL row staying paired with WAL. |
| V-09 | `src/bus.rs:444-450` (graveyard); `src/bus.rs:494-539` (`recent_rejections_scoped`) — feeds `errors` into next prompt at `evaluator.rs:940` | parallel_ledger | **No** | **No** | `errors` (TopK class labels) inject into every agent prompt, shaping behaviour. Replay from tape alone cannot reconstruct what each agent saw. |
| V-10 | `experiments/minif2f_v4/src/bin/evaluator.rs:759, 1374` (search_cache: `HashMap<String, Vec<String>>` per-agent in evaluator local scope); `evaluator.rs:983-984` (injected into prompt as `hits_ref`) | parallel_ledger | **No** | **No** | Search results actively change the prompt. Vanish on process exit. |
| V-11 | `src/sdk/tools/librarian.rs:80-118` (`write_board`/`post_to_board`/`read_board` write to `_board.md` files); `evaluator.rs:861-903, 993-1000, 1387` | parallel_ledger | **Partial** (board file persists if `EXPERIMENT_DIR` is set) | **No** | Agent posts and the periodic team-board snapshot live in sidecar markdown files, not on tape. The librarian's `learned.md` per-agent memory has the same problem (`librarian.rs:62-67`). |
| V-12 | `experiments/minif2f_v4/src/bin/evaluator.rs:1038-1040` (gp_tokens hack) and `:1217, 1465` (`tape_tokens.max(response.completion_tokens)`) | hack_proxy | **No** (BYTE count, not token count) | **No** | Computes `gp_tokens` as `Σ payload.len()` (bytes) over tape, then `.max(...)` against the API completion_tokens. The `.max(...)` masks V-01: when API count > byte count, the API count wins; otherwise the byte count is reported as a token count. |
| V-13 | `src/bus.rs:48`, `src/bus.rs:444-450` (`graveyard: HashMap<String, Vec<String>>`); never serialised to WAL | parallel_ledger | **No** | n/a | Even if WAL is on, the rejection graveyard is in-memory only. |
| V-14 | `src/sdk/tools/wallet.rs:24-29` (entire WalletTool); `wallet.rs:83-97` (`save_to_disk`/`load_from_disk`) | parallel_ledger | **Partial** (wallet JSON persists if WALLET_STATE env set) | **No** | Wallet balances + portfolios live in their own JSON file, not on tape. Settle-portfolio call (`bus.rs:383-413`) mutates this side table; tape sees nothing. |
| V-15 | `src/bus.rs:298-312` (founder grant `wallet.record_shares` from inside append flow); `bus.rs:307` | parallel_ledger | **No** | **No** | When TAPE_ECONOMY_V2=1, every append silently mints YES shares to the author. No ledger event, no Invest record. The shares vanish if WALLET_STATE not set. |
| V-16 | `src/bus.rs:147-149` (`open_bounty_market` from env at init); `src/kernel.rs:63-103` (bounty resolve writes payouts to wallet, no ledger event) | missing_provenance | **No** | **No** | Bounty market existence + LP seed + per-author payouts are entirely off-tape. Only the wallet credit shows up — and only in the wallet sidecar. |
| V-17 | `src/wal.rs` (entire); `evaluator.rs:585-616` (WAL is opt-in via `WAL_DIR` env) | reproducibility_break | **No** when WAL not enabled | **No** | Default Phase B production runs do NOT set `WAL_DIR`, so on process death the tape vanishes. The Turing axiom "paper persists" is honoured only as opt-in. |
| V-18 | `src/wal.rs:60-64` (sync after every line, no checksum on lines themselves); `src/wal.rs:88-92` (malformed lines silently skipped during replay) | reproducibility_break | **Partial** | n/a | WAL has no per-line hash chain (the `Ledger.hash` chain stops at the in-memory ledger; WAL replay rebuilds hashes, discarding originals — see `bus.rs:113-115`). A truncated/tampered WAL line cannot be detected; it just gets skipped. |
| V-19 | `experiments/minif2f_v4/src/lean4_oracle.rs:112+` (`verify_omega_detailed` returns `(bool, error_string)`) — error string is consumed at `evaluator.rs:1281, 1529` (record_tool_stdout estimate) and dropped | missing_provenance | **No** | **No** | Lean's full stderr/stdout is never on tape. Only the *length* (chars/4) feeds C_i, then the bytes vanish. Phase E cannot replay why a particular OMEGA was rejected. |
| V-20 | `experiments/minif2f_v4/src/bin/evaluator.rs:1184-1188` (`persist_proof_artifact`) writes `proofs/*.lean` to disk — sidecar to JSONL row, not on tape | parallel_ledger | **Partial** (file exists separately) | n/a | The accepted GP payload IS persisted on disk, but as a sidecar `.lean` file referenced by `gp_proof_file: Option<String>`. The tape Node payload IS the same text (good — `bus.append_oracle_accepted` at `evaluator.rs:1201`), but the decision to archive is recorded only on JSONL. |
| V-21 | `experiments/minif2f_v4/src/bin/evaluator.rs:1220-1223` (`bus.halt_and_settle(&gp)` resolves all markets after OMEGA); `bus.rs:336-376` | missing_provenance | **No** halt details | n/a | RunEnd ledger event has no detail of *what halted us*: OMEGA accept reason, GP set, payouts. All exists in FC_TRACE if enabled, but FC_TRACE is opt-in stderr stream — not the tape. |
| V-22 | `experiments/minif2f_v4/src/bin/evaluator.rs:415, 1037` (LLM call → record_llm_call); `evaluator.rs:419-437, 469, 797, 1174, 1446, 1580` (mark_final_accept) | parallel_ledger | **No** | **No** | Every LLM call's `(prompt_tokens, completion_tokens, model, latency, agent_id, tx_idx)` tuple should be a tape Node — instead it's split between RunCostAccumulator (totals only) and the response object (discarded after parse). |
| V-23 | `src/sdk/actor.rs:151-158` (`MinerTx { completion_tokens: u32 }`) — appears unused outside test fixtures | dormant_tape_field | n/a | n/a | Defined as if it were the canonical agent → bus channel record; in practice the bus invocation doesn't take a MinerTx, so this struct is also a dormant schema artifact mirroring V-01. |
| V-24 | `experiments/minif2f_v4/src/bin/evaluator.rs:1031, 412` (`assert_no_metric_leak`) — runtime guard, no on-tape evidence | missing_provenance | **Partial** (panics aborts run) | n/a | If the guard ever fires, the run aborts; if it doesn't, no positive evidence of the check is written to tape. Phase E auditor cannot tell whether the guard was disabled. |

---

## Recommended Atomization (10 commit units)

I propose splitting the fix into the following independent atomic commits, ordered by dependency:

### Commit 1 — **Tape schema upgrade (foundational)**
- Replace `Node.completion_tokens: u32` with a structured `Node.cost: NodeCost { prompt_tokens, completion_tokens, tool_stdout_tokens, latency_ms }` (per-node, exact).
- Add `Node.kind: NodeKind` enum: `{ AgentProposal, OracleVerdict, MrTick, BoltzmannPick, MarketCreate, MarketResolve, Invest, FounderGrant, BountyOpen, BountyResolve, Halt, SyntheticTreatment }`.
- Add `Node.kind_payload: serde_json::Value` for kind-specific structured detail.
- Bump `Node.created_at` to `u128` ms (or keep `u64` ms).
- Update WAL to v2 format with per-line SHA-256 chain hash (close V-18).
- Conformance test: `tests/tape_schema_v2_round_trip.rs` (round-trip every NodeKind variant).
- Addresses: V-01, V-06 (partial), V-18.

### Commit 2 — **Promote RunCostAccumulator → derived view**
- Re-implement `total_run_token_count()` as `tape.iter().filter(_kind=Proposal|OracleVerdict).map(n.cost).sum()`.
- Keep `RunCostAccumulator` only as a fast-path cache, recomputable from tape; mark in module docs.
- Conformance test: synthesize 5 failed + 1 success NodeKind::AgentProposal nodes; assert `RunCostAccumulator::from_tape(tape) == manual_acc` for both totals and `failed_branch_count` (so V-03 also closed: failed branches MUST become tape nodes with `verified: false` field).
- Addresses: V-02, V-03, V-22.

### Commit 3 — **Emit MarketCreate + MarketResolve + structured Invest**
- `bus.rs:289` and `kernel.rs:155-178/83-103`: append `EventType::MarketCreate { node_id, lp_seed }` and `EventType::MarketResolve { node_id, yes_wins, payouts: Vec<(agent, amount)> }`.
- `bus.rs:244`: `Invest` event detail must include amount, direction, shares (serialise to JSON in `detail: Some(...)`).
- Convert direct `bus.kernel.buy_yes/buy_no` calls in `evaluator.rs:1318-1322` to a wallet-aware bus action (V-05).
- Conformance test: `tests/market_event_completeness.rs` — every `kernel.create_market` call emits a MarketCreate event; every `resolve_all` emits MarketResolve for each market.
- Addresses: V-04, V-05, V-15 (founder grant becomes a typed Invest), V-16 (bounty open/resolve are typed events).

### Commit 4 — **Veto/rejection nodes are tape-canonical**
- `bus.rs:182-208`: instead of returning `BusResult::Vetoed { reason }` and dropping the payload, append a `Node { kind: AgentProposal, kind_payload: { verified: false, reject_class: "veto:forbidden", payload }, ... }` to the tape, then return.
- Same for OMEGA-reject (`evaluator.rs:1278, 1527`) and parse-fail (`evaluator.rs:1545`).
- Remove `bus.graveyard: HashMap` (now derivable). `recent_rejections` becomes a tape iterator.
- Conformance test: `tests/all_proposals_on_tape.rs` — for every `record_proposal` call (winning or losing), assert a tape Node exists with matching hash.
- Addresses: V-03, V-09, V-13.

### Commit 5 — **Mandatory WAL + tick events on tape**
- Default `WAL_DIR` to `experiments/<run>/wal/` instead of opt-in. (Phase E reproducibility cannot be opt-in.)
- `evaluator.rs:836-916`: write a `Node { kind: MrTick, ... }` to the tape at every tick instead of only `info!()` + FC_TRACE.
- Addresses: V-08a, V-17.

### Commit 6 — **Synthetic treatment is on-tape**
- `evaluator.rs:778-826`: append `Node { kind: SyntheticTreatment, kind_payload: { treatment: "rollback_sim", threshold: 50, expected_skipped_tx: 150 } }` BEFORE the early return.
- Remove `PputResult::synthetic_short_circuit` (now a derived flag from tape scan).
- Conformance test: `tests/synthetic_treatment_provenance.rs` — assert that any run where `SIMULATE_ROLLBACK_AT_TX_50=1` produces exactly one SyntheticTreatment tape node before the Halt node.
- Addresses: V-07.

### Commit 7 — **Boltzmann pick + LLM call are tape nodes**
- `evaluator.rs:1055-1057`: write a `Node { kind: BoltzmannPick, kind_payload: { selected_parent, frontier_size, scores: Vec<(node_id, score, weight)>, seed_after_call } }` per pick.
- Every `client.generate(...)` success creates a `Node { kind: AgentProposal, kind_payload: { agent_id, model, prompt_hash, response_text, prompt_tokens, completion_tokens, latency_ms } }`.
- Addresses: V-08b, V-22.

### Commit 8 — **Search/board/wallet sidecars become derived projections**
- `evaluator.rs:759, 1374`: search-hit injection moves through the bus as a typed Node (kind: Search, payload: { agent, query, hits, prompt_tokens_added }).
- `librarian.rs:62-67, 80-118`: writing `learned.md` and `_board.md` becomes a projection from tape, written periodically by a derived view (delete on cold start; rebuild from tape).
- `wallet.rs:83-97`: `save_to_disk` becomes optional debug aid; the canonical balance is `replay(tape).balances`.
- Conformance test: `tests/wallet_replay_invariance.rs` — given a tape, the reconstructed wallet equals the wallet sidecar (or the sidecar is absent and the test reconstructs from tape alone).
- Addresses: V-10, V-11, V-14.

### Commit 9 — **Lean error string + halt detail on tape**
- `evaluator.rs:1281, 1529`: capture the FULL Lean stderr/stdout (truncated to a configured cap, e.g. 4 KB) into the OracleVerdict Node's payload, not just the length estimate.
- `bus.rs:336-376` `halt_and_settle`: append `Node { kind: Halt, kind_payload: { reason, gp: Vec<NodeId>, payouts, runtime_accepted, post_hoc_verified } }` before RunEnd.
- Conformance test: `tests/halt_provenance.rs` — every solved run has exactly one Halt node with `reason="OmegaAccepted"`; every max-tx run has exactly one Halt with `reason="MaxTxExhausted"`; both cite the GP NodeIds.
- Addresses: V-19, V-21.

### Commit 10 — **WAL hash-chain + audit guard provenance**
- `wal.rs`: per-line SHA-256 over `(prev_hash, line)`; replay verifies. Malformed line aborts replay (no silent skip — change V-18 default).
- `prompt_guard.rs`: `assert_no_metric_leak` writes a `Node { kind: AuditCheck, kind_payload: { check: "no_metric_leak", prompt_hash, passed: true } }` per call. Phase E can verify the guard was actually executed.
- Addresses: V-18, V-24.

---

## Suggested Cross-Validation Tests

These belong in `experiments/minif2f_v4/tests/` and `tests/` and would prevent regression of any future "convenience parallel ledger":

1. **`tape_canonical_round_trip`**: serialise tape to WAL → kill process → replay WAL → reconstructed `(RunCostAccumulator, RunWallClock, wallet, market_prices, golden_path)` must bit-identically match the in-memory state. (Phase E gate.)
2. **`no_zero_completion_tokens_in_production`**: scan WAL files in `experiments/*/wal/`; assert no `Node` has `cost.completion_tokens == 0` AND `kind == AgentProposal`. (Closes V-01 regression.)
3. **`every_llm_call_has_tape_node`**: instrument `ResilientLLMClient::generate` with a counter; assert at run end `tape.iter().filter(kind=AgentProposal).count() == llm_call_counter`. (Closes V-22.)
4. **`every_market_mutation_has_event`**: assert `kernel.create_market`/`resolve` calls match `EventType::MarketCreate`/`MarketResolve` events 1:1 in the ledger. (V-04.)
5. **`failed_branches_appear_on_tape`**: for any run with `failed_branch_count > 0`, assert `tape.iter().filter(_.verified == false).count() == failed_branch_count`. (V-03.)
6. **`graveyard_is_derived_view`**: `bus.graveyard` must equal `derive_graveyard_from_tape(tape)`. Snapshot test. (V-09.)
7. **`wal_hash_chain_uninterrupted`**: replay → verify each line's hash chains to the prior. (V-18.)
8. **`synthetic_treatment_must_have_provenance_node`**: when `SIMULATE_ROLLBACK_AT_TX_50=1`, assert exactly one `kind=SyntheticTreatment` Node in the tape. (V-07.)
9. **`mr_tick_count_matches`**: at run end, `tape.iter().filter(kind=MrTick).count() == max_transactions / TICK_INTERVAL`. (V-08a.)
10. **`metric_leak_guard_is_provably_executed`**: every run has at least one `kind=AuditCheck, check=no_metric_leak` Node per LLM call. (V-24.)

---

## Open Questions for Human Decision

1. **Does the new Node.cost field break legacy Phase B JSONL artifacts?**
   `discarded_12way_run_2026-04-24/E1v2_Abl_*.jsonl` and Paper-1 era runs contain only `gp_token_count` (legacy schema, see `LegacyRunAggregate` at `jsonl_schema.rs:200`). Phase E cannot replay those (the tape was not preserved). Should we (a) accept that legacy artifacts are forever unverifiable, (b) write a one-time forensic accumulator tool that *generates* a synthetic tape from existing JSONL + log files, or (c) deprecate everything before Commit 1's effective date?
2. **WAL-mandatory default**: making `WAL_DIR` default disk write may surprise users running on RAM-only test environments. Recommend gating with `RUN_MODE=production` rather than purely `WAL_DIR`. Architect call.
3. **Failed-branch tape entries**: storing every parse-fail / bus-veto as a Node will inflate the tape ~3-5x in typical Phase B runs. Acceptable, or should we batch-compress every N rejection nodes into a single roll-up Node? My recommendation: **don't compress.** Compression IS the parallel-ledger anti-pattern.
4. **Should the search results corpus be on tape?** Each search hit is a *file path* to a MiniF2F problem statement that the agent could have read directly. Recording the full hit list per call grows tape. Compromise: record `(query, hit_count, hit_hashes: Vec<sha256>)`; the actual hit text is recoverable from the immutable MiniF2F dataset.
5. **Boltzmann seed re-derivation**: should we record the RNG state vector (all 256 bytes of `StdRng`) per pick (deterministic but heavy), or just the seed at run start + tx_index (compact, requires deterministic call sequence)? Phase E reproducibility is satisfied either way; the question is debug ergonomics.
6. **Lean error-string caps**: capping at 4 KB per OracleVerdict node loses info on long type-mismatch errors. Should we keep the full string when `Node.kind == OracleVerdict` and only cap on graveyard rejection labels?
7. **Architectural amendment scope**: the user said the Tape Canonical Axiom "may only be derived views" — does that block the `RunCostAccumulator` cache existing at all, or only if it's the *sole* writer? My read: cache is fine, but a conformance test must enforce equality with a tape-replay reconstruction at every emit site. Need explicit ruling.

---

## Files referenced in this audit

Source (read-only, no edits):

- `/home/zephryj/projects/turingosv4/src/ledger.rs`
- `/home/zephryj/projects/turingosv4/src/bus.rs`
- `/home/zephryj/projects/turingosv4/src/kernel.rs`
- `/home/zephryj/projects/turingosv4/src/wal.rs`
- `/home/zephryj/projects/turingosv4/src/prediction_market.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/snapshot.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/actor.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/tools/wallet.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/tools/librarian.rs`
- `/home/zephryj/projects/turingosv4/src/sdk/tools/search.rs`
- `/home/zephryj/projects/turingosv4/src/drivers/llm_http.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/wall_clock.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/rollback_sim.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/post_hoc_verifier.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/jsonl_schema.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/fc_trace.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs`
- `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs`
