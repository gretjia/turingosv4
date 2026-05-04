# post_r3_full_test/ — VINTAGE / NON-CANONICAL

> **Status**: PRE-RUNNER-FIX EVIDENCE — DO NOT USE FOR R3 SHIP CITATION.
> **Canonical R3 evidence**: `../post_r3_round2/SUMMARY.md` (8 chains × N=5 × MAX_TX=20).

## Why this dir is non-canonical

These chains were produced by `scripts/run_real_llm_arena.sh` BEFORE the runner
fix at commit `8b5c94a` (2026-05-04). The pre-fix runner shelled

```bash
--task-mode user --problem mathd_algebra_171 --max-transactions $MAX_TX
```

into `evaluator` — but those are **phantom CLI flags** the binary does not
parse. The result: `MAX_TX` was silently defaulted, the requested condition
did not bind, and `TURINGOS_CHAINTAPE_PRESEED=1` was missing → no preseed →
no `EscrowLock` tx in the produced chains.

Concretely: chains in this directory cover **6 tx kinds** (TaskOpen +
WorkTx + VerifyTx + Challenge + CompleteSetMint + MarketSeed + TerminalSummary)
but lack the architect-required `EscrowLock` row, so they fail to demonstrate
the full TB-13 economic-mutator sequence on real-LLM substrate.

The **canonical** R3 conformance evidence is `../post_r3_round2/`, which
covers **9 of 13 architect tx kinds** (full union including `EscrowLock`,
`FinalizeReward`) across 8 chains × 5 swarm × MAX_TX=20 with preseed bound,
and PROCEEDs 271/0/0 with byte-identical replay 8/8 + tamper 3/3 on every
chain.

## Why this dir is preserved (not deleted)

Per `feedback_no_retroactive_evidence_rewrite`:

> New evidence requirements (genesis_report, on-chain TaskOpenTx, oracle
> source rules, L4 purity) apply going-forward only. NEVER rewrite old
> ledger roots, migrate L4↔L4.E, fabricate genesis_report into old dirs,
> or relabel old `evaluator-attested` results as `chain-oracle-derived`.

These chains are real artifacts of the pre-fix runner. Deleting them or
re-labeling their tx coverage would be retroactive evidence rewrite. They
are kept verbatim with this README as the documentation surface.

## Scope of this annotation

- **This README** (created TB-16.x.1 atom A, 2026-05-04): the only mutation.
- All chain files (`P{1..5}_*/`): UNCHANGED, byte-identical to original commit.
- `tamper_report.json` files in subdirs: pre-runner-fix vintage; do not regen here.

## Cross-references

- Runner fix: commit `8b5c94a` ("TB-16 post-R3 — fix `run_real_llm_arena.sh`
  phantom CLI flags + smoke evidence").
- Canonical R3 evidence: `../post_r3_round2/SUMMARY.md` (1131 lines, 11
  sections incl. v3-style scaling table + per-mechanism × FC matrix + per-
  problem chain DAG + NodePositions + PriceIndex per problem).
- TB-16.x.1 charter: `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md`.
