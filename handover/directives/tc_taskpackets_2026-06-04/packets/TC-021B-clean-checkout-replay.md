# TC-021B Clean Checkout Replay

Status: ready
Owner lane: audit
Risk class: Class 2 final reliability
FC nodes: FC2 boot/replay, FC3 audit
Dependencies: TC-021A
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `scripts/tc_clean_checkout_replay.sh`
- `handover/reports/TC_CLEAN_CHECKOUT_REPLAY_2026-06-04.md`
- `tests/tc_clean_checkout_replay_contract.rs`

Forbidden paths: source mutation during final replay, network during replay.

Task:

Verify final evidence from a fresh checkout at final SHA.

Test first:

- `clean_checkout_replay_script_requires_no_network`
- `clean_checkout_replay_compares_exported_hashes`
- `clean_checkout_replay_requires_obl_all_closed`

Required final run:

1. fresh checkout final SHA.
2. run real binary entry.
3. replay from Git/CAS only.
4. re-run crash matrix.
5. verify no network/LLM during replay.
6. compare exported packet hashes.
7. run obligation witness.

Ship gate:

```bash
cargo test --test tc_clean_checkout_replay_contract --no-fail-fast
bash scripts/tc_clean_checkout_replay.sh --check
```

Expected: both commands exit 0; obligation witness emits `OBL-ALL-CLOSED`.

Audit: Reliability Auditor and Obligation Witness.
Verdict: `NO-VIOLATION` and `OBL-ALL-CLOSED`.
