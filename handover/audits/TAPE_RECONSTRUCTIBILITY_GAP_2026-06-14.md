# Gate D — H-HET-1 Carrier Tape-Reconstructibility Gap Analysis

> **✅ GAP CLOSED 2026-06-15** — resolved by the architect-ratified §8 change:
> `ProposalTelemetry` schema v1→v2 adds `model_id`, so per-proposal model provenance +
> cost = rate(model_id)×tokens now recompute from the frozen ChainTape+CAS ALONE (no
> Manifest roster, no round-robin inference). The negative-witness
> `model_id_is_not_a_field_on_carrier_cas_objects` flipped to the positive recompute
> closure `model_id_is_tape_canonical_on_carrier_cas_objects`. Historical v1 CAS still
> replays byte-equivalent via the legacy fallback decoder. Verified: full constitution
> gate suite 167 / 3-known-reds (zero new), schema lib 10/10, Gate-D 5/5. See
> `ART_0_2_FULL_CLOSE_DESIGN_2026-06-15.md`. The analysis below is the historical record.

**Date:** 2026-06-14 (test added; analysis carried 2026-06-15)
**Gate:** D — tape-reconstructibility (architect audit, constitution **Art 0.2** "所有信号必须可从 tape 重建 / any field that cannot be rebuilt from the frozen tape is excluded from the headline metric"). This is the HARDEST, partly-DIAGNOSTIC gate.
**Carrier:** `src/bin/lean_market_agent.rs` (H-HET-1 heterogeneous autonomous market).
**Risk class:** 2 (evaluator/benchmark mechanism + additive test; no §6 restricted surface touched).
**Repo pin:** `/Users/zephryj/work/turingosv4` @ `claude/p1-realvalue` (HEAD `4cfbc41e`). ⚠ ≥2 clones on this machine — do NOT confuse with `~/Developer/turingosv4-port`.
**Test artifact:** `tests/constitution_het_tape_reconstructibility.rs` (5 tests, all PASS — see §3).

> **Honest verdict up front:** this is a **PARTIAL PASS**, which is the expected, honest Gate-D outcome. The structural backbone (nodes, lineage, tokens, judge verdict, tx-type-encoded action, price snapshot, WorkTx↔ChallengeTx linkage) IS reconstructible from the frozen canonical ChainTape + CAS alone. The **LLM-call provenance layer** (per-call `model_id`/`provider`/`rate-table-version`, real prompt-byte hash, `finish_reason`/`truncation`, and therefore **cost**) is **NOT** on the frozen tape — it lives only in the `Manifest` sidecar JSON / runtime accumulator, or is not captured at all. Those fields must be excluded from any Art-0.2 headline until the schema work in §5 lands.

---

## 1. What the carrier actually writes (the substrate map)

A critical framing correction the architect should note: **the H-HET-1 carrier does NOT emit the `lean_hayek_market` JSONL `MarketTape`** that `src/market_tape_shared.rs::derive_*` + `src/bin/verify_market_tape.rs` were built for. There is **no `LLMCall` event, no `GenesisPin`, no `--tape-out` JSONL** on this carrier's path. The `market_tape_shared` import in `lean_market_agent.rs` is only `call_micro_usd` + the fallback rate constants (line 90) — the `MarketTape`/`MarketEvent`/`derive_cost` machinery is **not used here**.

Instead the carrier writes three distinct stores:

| Store | What | Reconstructible substrate? |
|---|---|---|
| **Canonical ChainTape (L4 + L4.E)** | `TaskOpen`, `EscrowLock`, **`WorkTx`** (one per solve attempt), **`ChallengeTx`** (one per short), `VerifyTx`, system `EventResolve`. Replayed via `Git2LedgerWriter` + `RejectionEvidenceWriter`. | **YES** — frozen, append-only, hash-chained, replay-deterministic. |
| **CAS** (git-backed, content-addressed) | `ProposalTelemetry` (token_counts, parent_tx, candidate_tactic, prompt_context_hash), `LeanResult` (exit_code, verified, verdict_kind, error_class), `VerificationResult` (lean_exit_code, verified, hashes), counterexample blobs, proof artifacts. | **YES** — content-addressed; tamper is fail-visible (the CAS commit-chain integrity check fires, see §3 anti-tamper test). |
| **`Manifest` JSON** (`lean_market_manifest.json`, the `--out` sidecar) | `nodes: Vec<AttemptNode>` (incl. `chosen_action`, `reject_class`, `price_yes_num/den`, `axioms`, `body_preview`, `tokens`), `models` roster, all the `*_llm_calls` / `*_tokens` accounting, `pput`, `cost`-shaped headline integers. | **NO** — this is a derived view (Tier-3 / below). Per Art 0.2 + AGENTS.md §17.2, any headline field that lives **only** here is NOT tape-reconstructible. |

The reconstructibility question is therefore: **for each architect-named field, is it on the canonical ChainTape + CAS (reconstructible), or only in the Manifest sidecar / runtime accumulator (GAP)?**

The canonical derive function that answers this — and the one the production replay verifier path uses — is `runtime::chain_derived_run_facts::compute_run_facts_from_chain(runtime_repo, cas)`. The Gate-D test exercises it (and a manual L4+L4.E+CAS walk) over a frozen carrier-shaped chain.

---

## 2. Field-by-field determination

Legend:
- **RECONSTRUCTIBLE** — recoverable byte-equal from frozen L4/L4.E + CAS alone, with a derive fn + test.
- **RECONSTRUCTIBLE (by inference / degraded)** — recoverable from the tape but via a derived rule, not as a literal stored field; honestly weaker than a stored byte.
- **GAP** — lives only in the Manifest sidecar / runtime accumulator, or is not captured at all. Cannot enter an Art-0.2 headline until schema work lands.

| # | Architect-named field | Verdict | Where it lives TODAY / how derived |
|---|---|---|---|
| D-1 | **per-LLM-call `model_id` / `provider`** | **GAP** | Only `Manifest.models` (sidecar) + the deterministic round-robin rule `agent_models[i] = models[i % len]` (`lean_market_agent.rs:1697`). **NOT** on `WorkTx`, `ProposalTelemetry`, or `LeanResult`. The CAS objects the carrier writes carry **no model field** (proven by `model_id_is_not_a_field_on_carrier_cas_objects`). The comment at `lean_market_agent.rs:110` ("recorded on tape") is **inaccurate** — the model id is recorded only in the sidecar. |
| D-2 | **rate-table-version** | **GAP** | `MODEL_RATES` is a compile-time table in `market_tape_shared.rs:146` with per-row `verified_on` dates in comments only. No version/hash is pinned to any tape/CAS object for this carrier (the `lean_hayek` path pins it via `GenesisPin.axiom_whitelist`-style provenance + `verify_market_tape`; the carrier has no GenesisPin). |
| D-3 | **prompt hash** | **RECONSTRUCTIBLE (degraded)** | `ProposalTelemetry.prompt_context_hash` IS on CAS (read via `WorkTx.proposal_cid`). **Caveat:** it is `sha256(run_id ‖ agent_id ‖ proposal_index)` (`proposal_telemetry.rs` `build_for_evaluator_append_with_parent`), **not** a hash of the actual prompt BYTES. So it binds *which* (run, agent, attempt) slot, not *what prompt text* was sent. A true prompt-byte hash is a GAP. |
| D-4 | **`completion_tokens` (and prompt/completion split)** | **RECONSTRUCTIBLE** | `ProposalTelemetry.token_counts.{prompt_tokens, completion_tokens, tool_tokens}` on CAS. Test `tokens_reconstructible_from_chain_and_cas` recomputes `Σ token_counts.total` from L4+CAS byte-equal to ground truth via `compute_run_facts_from_chain().golden_path_token_count`. The split is present per node. |
| D-5 | **`finish_reason` / truncation** | **GAP** | Not captured by the carrier at all. `is_truncated()`/`finish_reason` were added to `het_capability_probe` (the test harness), **not** to `lean_market_agent` — and `LlmResponse.finish_reason` is never persisted to any CAS object. No tape/CAS field exists. |
| D-6 | **agent `chosen_action` (solve/short)** | **RECONSTRUCTIBLE (by inference)** | The literal `"solve"/"short"` string lives ONLY in `Manifest.nodes[].chosen_action` (sidecar). BUT it is inferable from the canonical **tx type**: a self-chosen solve emits a `WorkTx` (`lean_market_agent.rs:2272`); a self-chosen short emits a standalone agent `ChallengeTx` with no WorkTx (`:1990`, `chosen_action=Some("short")` at `:2044`). So `WorkTx by Agent_i ⟹ solve`, `agent-submitted ChallengeTx ⟹ short` is derivable from L4 alone. The comment at `:155` ("AttemptNode.chosen_action … tape-recorded") conflates the sidecar AttemptNode with the canonical tape — the literal field is sidecar; the action is *inferable* from tape. |
| D-7 | **price snapshot before decision** | **RECONSTRUCTIBLE** | The price the agent saw is `compute_price_index(q.economic_state_t)` (`lean_market_agent.rs:1825`). `compute_price_index` is a **pure, replay-deterministic** fn over `EconomicState` (`price_index.rs:164`, doc: "no env input, no clock, no randomness", Art.0.2-pinned), and `EconomicState` is itself a replay reconstruction of the L4 chain (precedent: `tests/economic_state_reconstruct.rs`, G0 condition-8 `economic_state_reconstructed`). **Caveat:** reconstructible at each **state-root boundary** (the price index is a function of the committed `node_positions_t` + share balances at that head); intra-round per-agent ordering is pinned by the WorkTx/ChallengeTx `timestamp_logical` + L4 order. The literal `price_yes_num/den` the agent saw is *also* mirrored into `Manifest.nodes[]`, but it does NOT depend on the sidecar — it re-derives from the chain. |
| D-8 | **wallet delta** | **RECONSTRUCTIBLE** | Wallet/economic state is the canonical `EconomicState.balances_t` + `escrows_t`, reconstructed by L4 replay (same G0 condition-8 precedent + `tests/economic_state_reconstruct.rs` / `q_state_reconstruct.rs`). A per-agent delta = (state after settlement − genesis preseed), both replay-derivable. The carrier does NOT write a bespoke "wallet delta" field anywhere — it is purely a chain-derived view, which is the Art-0.2-correct design. |
| D-9 | **WorkTx / ChallengeTx linkage** | **RECONSTRUCTIBLE** | `ChallengeTx.target_work_tx` (`typed_tx.rs:335`) is a first-class field on the canonical tape, pointing at the shorted WorkTx. Node→parent lineage is `ProposalTelemetry.parent_tx` on CAS (read via `WorkTx.proposal_cid`). Test `parent_lineage_reconstructible_from_cas` recovers the full `(tx_id, parent_tx)` DAG from L4+L4.E+CAS with no manifest; `proposal_count_reconstructible_from_chain` recovers node count. |
| D-10 | **judge result / reject_class** | **RECONSTRUCTIBLE** | Judge verdict is double-witnessed on the canonical substrate: (a) `VerifyTx.verdict` (Confirm/Doubt) on L4, and (b) `VerificationResult{lean_exit_code, verified}` on CAS (`lean_market_agent.rs:2380`), plus `LeanResult{exit_code, verified, verdict_kind, error_class}` on CAS (`:2214`). `compute_run_facts_from_chain` derives `chain_oracle_verified` from VerifyTx::Confirm + VerificationResult.verified. **reject_class:** the literal `AttemptNode.reject_class` string is sidecar, but it is a pure function of `LeanResult.{verdict_kind, error_class}` which ARE on CAS (`reject_class_of()` maps them) — so reconstructible by inference from the CAS LeanResult. |
| D-11 | **cost derivation (micro-USD, cost-of-pass)** | **GAP** (blocked by D-1/D-2) | Cost = `Σ call_micro_usd(model, prompt_tok, completion_tok)`. The tokens (D-4) ARE on tape, but `call_micro_usd` needs the **model id** to pick the rate row — and the model id is a GAP (D-1). So cost is NOT recomputable from the frozen tape alone; it needs `Manifest.models` + the round-robin rule. Additionally the carrier does **not currently write any cost field** to the Manifest either (no `micro_usd`/`cost_of_pass` field in the `Manifest` struct), unlike the `lean_hayek` path. The `lean_hayek` precedent (`market_tape_shared::derive_cost` over `LLMCall{model,…}` events) does NOT apply here because the carrier emits no `LLMCall` event carrying the model id. |

---

## 3. The reconstructibility test (the reconstructible subset)

`tests/constitution_het_tape_reconstructibility.rs` — 5 tests, all PASS:

```
cargo test --test constitution_het_tape_reconstructibility -- --test-threads=1
# running 5 tests
# test cas_tamper_cannot_silently_preserve_token_recompute ... ok
# test model_id_is_not_a_field_on_carrier_cas_objects ... ok
# test parent_lineage_reconstructible_from_cas ... ok
# test proposal_count_reconstructible_from_chain ... ok
# test tokens_reconstructible_from_chain_and_cas ... ok
# test result: ok. 5 passed; 0 failed
```

Each test builds a **frozen carrier-shaped chain** (real-signature `WorkTx` whose `proposal_cid` resolves to a `ProposalTelemetry` carrying `token_counts` + `parent_tx`, plus an agent `ChallengeTx` linked by `target_work_tx`) using the SAME runtime sequencer + CAS the carrier uses, shuts the sequencer down, then reconstructs **from the frozen `runtime_repo` (L4) + `cas` ALONE** via `compute_run_facts_from_chain` (and a manual L4+L4.E+CAS walk). It mirrors the proven `tests/tb_18r_chain_derived_facts_exact_accounting.rs` pattern.

| Test | Proves (Art 0.2) | Covers field |
|---|---|---|
| `tokens_reconstructible_from_chain_and_cas` | `Σ ProposalTelemetry.token_counts.total` reconstructed from L4+CAS == in-memory total | D-4 |
| `proposal_count_reconstructible_from_chain` | node count == WorkTx count on the canonical tape | D-9 |
| `parent_lineage_reconstructible_from_cas` | full `(tx_id, parent_tx)` DAG recovered from L4+L4.E+CAS, no manifest | D-9 |
| `cas_tamper_cannot_silently_preserve_token_recompute` | dropping a ProposalTelemetry index entry either fail-closes (CAS commit-chain integrity check fires) OR moves the token total — never silently identical. §17.2: recompute is a function of frozen CAS content, not a manifest read-back | D-4 anti-tamper |
| `model_id_is_not_a_field_on_carrier_cas_objects` | **GAP pin** — `ProposalTelemetry` has no model field; the carrier never writes `AttemptTelemetry.model_name`. Flips (fails) the day the gap is closed, forcing this doc to be updated | D-1 |

I deliberately did **not** force the GAP fields (D-1, D-2, D-3-bytes, D-5, D-11) to pass and did **not** fabricate derive fns for them. The `model_id_is_not_a_field…` test is a *negative* witness that pins the gap so it cannot silently close unnoticed.

---

## 4. The Art-0.2 gap (the merge-blocker analysis)

Per Art 0.2 (constitution.md:62, :157) and AGENTS.md §17.1-G1: **any field that cannot be rebuilt from the frozen tape must NOT enter a `PputResult` / Art-0.2 headline metric.** For the H-HET-1 carrier, that excludes — until §5 schema work lands — the following from any `PROVEN`/headline claim:

1. **Per-LLM-call `model_id` / `provider`** (D-1) — the heterogeneity axis itself. ⚠ This is the most consequential gap: H-HET-1's entire scientific question is "does *heterogeneity* (different models) light up the price signal?" If the model each node used is **not on the frozen tape**, then a heterogeneity headline is reconstructed from the sidecar roster + a round-robin assumption, not from the tape. A skeptical auditor cannot verify *from the frozen evidence alone* that Agent_2's node was actually GLM-4.5-Air and not DeepSeek.
2. **rate-table-version** (D-2) and therefore **cost / cost-of-pass / banked-per-dollar** (D-11) — any efficiency headline (`cost_per_solved`, `banked_per_dollar`) is a §17.1 `X > Y` efficiency claim that requires G1 recompute-from-tape; cost is not tape-recomputable here.
3. **real prompt-byte hash** (D-3) — the stored hash binds the slot, not the prompt text; a "the agents saw shielded signals only" claim is only partially tape-verifiable.
4. **`finish_reason` / truncation** (D-5) — a truncated completion currently leaves no tape trace on the carrier path; a "no truncation contaminated the result" claim is not tape-backed.

The reconstructible subset (D-4, D-6 by inference, D-7, D-8, D-9, D-10) is sufficient to reconstruct the **market dynamics** (who attempted what, in what lineage, at what price, with what judge verdict, who shorted whom, and the wallet outcome) — but NOT the **per-model provenance + cost economics**.

---

## 5. Schema work that would close each gap (forward-only, no historical rewrite)

These are additive, no-§6-touch changes (Class 2; `ProposalTelemetry` is binding per its doc-comment "do NOT add fields without architect ratification" — so D-1 via ProposalTelemetry needs a §8, whereas D-1 via the already-existing `AttemptTelemetry` slots does not):

- **D-1 / D-2 (model_id, provider, rate-table-version):** the schema slot ALREADY EXISTS — `AttemptTelemetry.{model_name, model_family, model_provider, model_version}` (`attempt_telemetry.rs:354`, "G4.2 actual model identity"). The carrier imports `attempt_telemetry` but only for `LeanResult`/`LeanVerdictKind` — it never writes an `AttemptTelemetry` record. **Close:** have the carrier write one `AttemptTelemetry` per node populated from `agent_models[ai]` (+ a `rate_table_version`/`MODEL_RATES` content-hash field, additive). Then `model_id`/`provider` become reconstructible and D-11 cost becomes recomputable via `call_micro_usd(model_from_tape, tokens_from_tape)` — a real `derive_cost`-style fn over the canonical CAS, with a recompute-vs-Manifest gate (the §17.1-G1 form). This is the single highest-leverage gap-closer (unblocks D-1, D-2, D-11).
- **D-3 (real prompt-byte hash):** change `build_for_evaluator_append_with_parent` to set `prompt_context_hash = sha256(actual_prompt_bytes)` (or add a sibling `prompt_bytes_hash` field), instead of `sha256(run_id ‖ agent_id ‖ idx)`.
- **D-5 (finish_reason / truncation):** add `finish_reason: Option<String>` + `truncated: bool` to the per-attempt CAS record (`AttemptTelemetry` or `ProposalTelemetry`), captured from `LlmResponse` at proposal time — the carrier already has `resp.finish_reason` available; it is simply discarded.
- **D-6 (chosen_action literal):** optional — the action is already inferable from tx type (WorkTx vs agent ChallengeTx). If a literal on-tape marker is wanted, it could ride the new `AttemptTelemetry` record. Low priority (inference is sound).

Until D-1/D-2 land, any H-HET-1 headline must be scoped: e.g. "market dynamics + judge verdicts + lineage are tape-reconstructible; per-model attribution and cost economics are sidecar-derived (roster + round-robin), pending the AttemptTelemetry model-identity write."

---

## 6. Honest scope notes (what this gate did NOT do)

- It did **not** run the real H-HET-1 carrier with live LLMs (no provider, and reconstructibility is a property of the *schema/substrate*, not of any particular run's content — the test builds a faithful frozen chain with the identical types/writers).
- It did **not** assert any scientific H-HET-1 conclusion (heterogeneity lighting up signal) — that experiment is still unrun per the session audit.
- The price-snapshot reconstructibility (D-7) is asserted at the **substrate** level (pure `compute_price_index` over replay-reconstructed `EconomicState`, per the established G0 condition-8 + `economic_state_reconstruct.rs` precedent); this Gate-D test does not add a new end-to-end price-replay assertion (that precedent already exists and is green).
- The `cas_tamper` test revealed the carrier's CAS has its **own** sidecar↔commit-chain integrity check that fail-closes on tamper — a stronger anti-tamper property than the JSONL `MarketTape` (which relies purely on recompute drift). This is recorded as a positive finding.
