# TC-105 Wallet Derived Fact

Status: ready
Owner lane: substrate
Risk class: Class 1 derived-view test
FC nodes: FC1 tape-derived economic state
Dependencies: TC-101
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/runtime/tc_tape_canonical.rs`
- `tests/tc_tape_canonical_repairs.rs`

Forbidden paths:

- `src/sdk/tools/wallet.rs`
- money/economy authority surfaces
- typed-tx schema

Task:

Prove wallet facts are derived from ChainTape/economic replay, not SDK wallet
state or dashboard cache.

Test first:

`wallet_fact_replays_from_chaintape_not_tool_state`.

Assertions:

- wallet fact kind is `wallet_derived`.
- fact includes source head and payload hash.
- constructor rejects `source = "sdk_wallet_state"`.

Ship gate:

```bash
cargo test --test tc_tape_canonical_repairs --no-fail-fast
git diff --name-only origin/main...HEAD | grep 'src/sdk/tools/wallet.rs'
```

Expected: cargo test exits 0; grep has no output.

Audit: Constitution Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
