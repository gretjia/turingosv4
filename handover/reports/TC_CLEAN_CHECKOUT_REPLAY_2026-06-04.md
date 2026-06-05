# TC Clean Checkout Replay 2026-06-04

Status: pre-final replay contract. This report records the required final
replay shape and the exact final-gate tokens. It is not a completion claim
until the script gate and obligation witness pass.

fresh_checkout: required
final_sha: required
network_policy: disabled
llm_replay_policy: disabled
hash_compare: required
git_cas_only_restart: required
obligation_witness_required: OBL-ALL-CLOSED
obligation_witness_verdict: PENDING

## Required Final Steps

1. Create a fresh checkout at the final source SHA.
2. Rebuild from source in that checkout.
3. Replay from Git/CAS only.
4. Re-run crash matrix and witness contracts.
5. Verify no network or LLM is used during replay.
6. Compare exported packet hashes.
7. Run the obligation witness and require `OBL-ALL-CLOSED`.

## Expected Commands

```bash
cargo test --test tc_audit_packet_export --no-fail-fast
bash scripts/export_tc_audit_packet.sh --check
cargo test --test tc_clean_checkout_replay_contract --no-fail-fast
bash scripts/tc_clean_checkout_replay.sh --check
```

The final `tc_clean_checkout_replay.sh --check` gate is intentionally strict:
it must fail until OBL-014 is marked satisfied and this report records
`obligation_witness_verdict: OBL-ALL-CLOSED`.
