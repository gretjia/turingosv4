# TC-003a Ref Contract Consolidation

Status: ready
Owner lane: substrate
Risk class: Class 2
FC nodes: FC1 `HEAD_t`, FC2 boot refs
Dependencies: TC-002
Active obligations: OBL-014(open) -> this atom

Allowed write paths:

- `src/git_tape_ledger.rs`
- `tests/tc_git_tape_ledger_hardening.rs`

Forbidden paths: restricted surfaces.

Task:

Keep `TcHeadRefs` as the single TC ref contract shim and ensure it matches
canonical TDMA constants. Do not conflate TDMA tape with canonical ChainTape.

Test first:

Add or maintain test:
`tc_head_refs_match_locked_contract_and_transition_constants`.

Assertions:

- `accepted_l4 == "refs/chaintape/l4"`
- `rejected_l4e == "refs/chaintape/l4e"`
- `cas_root == "refs/chaintape/cas"`
- `tdma_verified == GIT_LEDGER_HEAD_REF`
- `tdma_tail == GIT_LEDGER_LEDGER_TAIL_REF`

Ship gate:

```bash
cargo test --test tc_git_tape_ledger_hardening --no-fail-fast
cargo test --test constitution_tc_boot_trust_root_manifest --no-fail-fast
```

Expected: both commands exit 0.

Audit: Data-integrity Auditor.
Verdict: `NO-VIOLATION` or `SECOND-SOURCE-DRIFT <view>`.
