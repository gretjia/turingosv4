# Polymarket Core User Flow - Directive and Section 8 Packet (2026-05-23)

**Charter**:
`handover/tracer_bullets/TB-POLYMARKET-CORE-FLOW_charter_2026-05-23.md`

**Risk class**: 3.

**Authorization**: user message on 2026-05-23: "PLEASE IMPLEMENT THIS PLAN" for
the multi-agent audit team and delivery plan. The same instruction explicitly
sets Codex as Orchestrator Oversee and replaces Gemini with `agy` for the second
independent witness.

**Predecessor gate**: PR #125 was checked and is merged. The overlapping
`cmd_llm.rs`, `cmd_generate.rs`, `cmd_init.rs`, and `market_view.rs` surfaces
are no longer in-flight.

## Implementation Directive

Implement the flow as one Class 3 user-flow atom without widening typed tx,
sequencer admission, or signing payload schemas:

- Triage must use `max_tokens=512` and force thinking off/absent.
- Welcome init must pick DeepSeek model names only for DeepSeek-direct
  endpoints; SiliconFlow endpoints keep SiliconFlow defaults.
- In-memory welcome API keys may only be passed to CLI children as env aliases.
- Web generate defaults to `N=3`; CLI generate defaults to `N=1`.
- Old workspaces missing worker preseed must degrade to `N=1`.
- `cmd_generate` owns outer fan-out orchestration; `tdma_runner` remains a
  single proof-run shape.
- Candidate artifacts, rejection capsules, test summaries, and telemetry must
  be CAS-backed and reconstructable from the root workspace CAS.
- Canonical lifecycle must be:
  `TaskOpen -> EscrowLock -> WorkTx* -> MarketSeed -> VerifyTx -> FinalizeReward -> EventResolve`.
- Winner must be derived only from finalized chain replay.
- Websocket `agent_attempt_update` is a hint. The panel must refetch
  `/api/market/by-session/:id` and treat replay as authority.

## Section 8 Classification

This packet authorizes Class 3 implementation only. It does not authorize:

- new typed transaction schema or discriminants;
- sequencer admission changes;
- canonical signing payload changes;
- trust-root or constitution edits;
- root `genesis_payload.toml` changes.

Touching any of those surfaces upgrades the atom to Class 4 and blocks ship
until a fresh per-atom Section 8 packet lands.

## Required Witnesses

- Codex clean-context witness, verdict domain:
  `NO-VIOLATION | VIOLATION-FOUND | RECONSTRUCTION-FAILURE | SECOND-SOURCE-DRIFT`.
- AGY independent witness, same verdict domain, replacing Gemini for this atom
  per user instruction.

## Evidence Commands

Required before PR:

```bash
CARGO_BUILD_JOBS=1 cargo check --features web
npm --prefix frontend run build
npm --prefix frontend test
CARGO_BUILD_JOBS=1 cargo test --test generate_emits_work_tx_smoke -- --nocapture
CARGO_BUILD_JOBS=1 cargo test --features web --test cli_web_generate_smoke -- --nocapture
CARGO_BUILD_JOBS=1 cargo test --features web --test cli_web_welcome_smoke -- --nocapture
CARGO_BUILD_JOBS=1 cargo test --test cmd_llm_triage_stub triage_stub_uses_large_budget_and_forces_thinking_off -- --nocapture
CARGO_BUILD_JOBS=1 cargo test --features web --test web_spec_turn_endpoint -- --nocapture
CARGO_BUILD_JOBS=1 cargo test --bin turingos_web --features web top_level_winner_agent_id_is_null_until_market_finalized -- --nocapture
RUST_TEST_THREADS=1 CARGO_BUILD_JOBS=1 cargo test --workspace --no-fail-fast
RUST_TEST_THREADS=1 CARGO_BUILD_JOBS=1 bash scripts/run_constitution_gates.sh
CARGO_BUILD_JOBS=1 cargo test --test constitution_matrix_drift -- --nocapture
python3 scripts/check_trace_matrix.py --mode ci --base-ref origin/main
git diff --check
```

## Verdict Record

Final witness record is stored in
`handover/audits/POLYMARKET_CORE_FLOW_MULTI_AGENT_AUDIT_2026-05-23.md`.
