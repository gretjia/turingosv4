# CODEX REAL-6B Implementation Review R1

Date: 2026-05-15

Reviewer: clean-context Codex `gpt-5.5` / `xhigh`

Verdict: `CHALLENGE`

## Findings

### [P1] Trust Root rehash is not narrow in the recorded REAL-6B diff

The run's `diff.patch` updates many `genesis_payload.toml` Trust Root entries unrelated to REAL-6B, including restricted/high-risk surfaces such as `src/kernel.rs`, `src/bus.rs`, `src/bottom_white/ledger/system_keypair.rs`, `src/bottom_white/ledger/transition_ledger.rs`, and `src/bottom_white/ledger/rejection_evidence.rs`.

The report claims a narrow `src/runtime/mod.rs` rehash, but the artifact does not support that. Current `genesis_payload.toml` also leaves the `src/runtime/mod.rs` comment describing REAL-5, not REAL-6B.

### [P2] The fixture can violate sealed-oracle order at `opened_at_logical_t == 0`

The decision record requires:

```text
SubmitCandidate -> AttemptPredictionMarketOpen -> ...
```

but the builder uses `opened_at_logical_t.saturating_sub(1)` for `SubmitCandidate`, so `open_t = 0` makes SubmitCandidate and Open share `logical_t = 0`. Validation checks close/oracle order and K window timing, but does not check SubmitCandidate exists strictly before market open.

## Verified

The REAL-6B runtime helper itself does not add production `TypedTx`, `TxKind`, sequencer admission, canonical signing payload, or live LLM scheduling. The fixture uses deterministic logical ticks, no sleep fields, close before oracle, price-as-signal, and per-step visibility markers.

The reviewer reran:

```text
cargo test --test constitution_real6_attempt_prediction_market -> 6 passed
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -> 1 passed
```

## Required Remediation

Isolate the REAL-6B ship diff so `genesis_payload.toml` only carries the required `src/runtime/mod.rs` hash change for this atom, or explicitly split/document the unrelated Trust Root rehashes in their own reviewed package. Update the `src/runtime/mod.rs` Trust Root comment to mention REAL-6B.

Also reject `opened_at_logical_t == 0` or use checked arithmetic plus a validation assertion that SubmitCandidate is strictly before market open.

## Verdict

CHALLENGE
