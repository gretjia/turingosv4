# TB-7 Atom 6 — chain-backed smoke evidence

**Date**: 2026-05-01
**Source**: `tests/tb_7_atom6_chain_backed_smoke.rs::i110_chain_backed_smoke_end_to_end_synthetic_llm`
**Mode**: synthetic-LLM (real DeepSeek + Lean run is documented as manual procedure in the test header).
**Charter §13.4 closure**: Codex audit cc7b3dd action items #2 / #4 / #5 / #6 / #7 — the on-disk evidence demonstrates the full TB-7 pipeline (Atoms 1 / 1.5 / 1.7 / 2 / 3 / 4 / 5) end-to-end.

## Headline

- L4 entries: 1
- L4.E entries: 6
- All 7 ReplayReport indicators GREEN: true
- chain_derived_run_facts.json: tx_count = 7, failed_branch_count = 6
- agent_pubkeys.json: 3 agents pinned

## What this evidence proves (Frame B closure structural witness)

1. **Gate 1 + Gate 7** (authoritative path): every WorkTx + VerifyTx submitted via bus.submit_typed_tx; no legacy bus.append used as authoritative state mutation.
2. **Gate 3** (≥1 L4 + ≥1 L4.E): synthetic TaskOpen → L4 accept; zero-stake WorkTx → L4.E reject.
3. **Gate 4** (agent signatures): every WorkTx + VerifyTx signature verifies against agent_pubkeys.json on replay.
4. **Gate 5** (ProposalTelemetry CAS): every WorkTx.proposal_cid resolves to a CAS ProposalTelemetry object.
5. **Gate 6** (chain-derived run facts): structural facts computed from L4 + L4.E + CAS alone match expected shape.

## What is NOT in scope here

- **Real LLM proposals**: the synthetic agents (`n1` / `swarm_a` / `swarm_b`) emit deterministic WorkTx + VerifyTx pairs to exercise the routing, NOT real DeepSeek-generated Lean proofs. The full real-LLM smoke is a manual procedure (see test header).
- **Accepted-L4 economic settlement**: zero-stake WorkTx by design routes to L4.E. The TaskOpen at L4 is the natural accept; no FinalizeRewardTx (RSP-4 / TB-9 territory).
- **gp_proof_file**: chain doesn't bind file paths (charter §4.4 excluded); stays in evaluator stdout.
