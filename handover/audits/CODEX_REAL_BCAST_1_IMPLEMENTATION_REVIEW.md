# CODEX_REAL_BCAST_1_IMPLEMENTATION_REVIEW

Date: 2026-05-16

Reviewer: clean-context Codex / GPT-5.5 xhigh

Verdict: `VETO`

## Findings

### S0 — Ship / real A-B is blocked

`experiments/minif2f_v4/src/bin/evaluator.rs` is Trust-Root pinned in
`genesis_payload.toml`.

Evidence:

```text
genesis_payload.toml:164
expected evaluator.rs hash:
40f645af21d0a908e3b49d9f8aa9d1810304a39462baa6fba6106572bae0ad01

handover/evidence/real_bcast_1_ab_A_20260516T000000Z/batch_evaluator.log:1
actual evaluator.rs hash:
38e281876a95d51d993ca2c0a7aeac02cbabc818742574b45887680da0837d12
```

The A-arm runner exited before conclusion-bearing execution:

```text
handover/evidence/real_bcast_1_ab_A_20260516T000000Z/run_log.txt
batch_exit=2
audit_exit=2
audit_verdict=<empty>
```

Independent re-run of the Trust Root unit gate also failed:

```text
cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
exit_code=101
```

Recorded in:

```text
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/artifacts/command_0004_stdout.txt
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/artifacts/command_0004_stderr.txt
```

Conclusion: REAL-BCAST-1 cannot ship and REAL-BCAST A/B cannot proceed until
explicit Class-4 Trust Root rehash ratification is granted, the Trust Root is
rehashed, and the real A/B smoke is rerun.

### S1 — Dev harness risk classification is stale

The active dev harness was opened as Class 3:

```text
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/DevTaskManifest.json
risk_class=3
restricted_surface_hits=[]
ratification=null
allowed_paths includes experiments/minif2f_v4/src/bin/evaluator.rs
```

The implementation later reached a Trust-Root-pinned executable surface. This
does not invalidate the unit tests, but it blocks ship and conclusion-bearing
evidence under the current harness classification.

## Checked Items

### PromptCapsule binding

The evaluator adds Librarian digest/crop CIDs to `PromptCapsuleV2.read_set` and
validates that the visible-context bytes/hash include the notices:

```text
experiments/minif2f_v4/src/bin/evaluator.rs:163-200
src/runtime/librarian_broadcast.rs:775-790
```

### CAS strategy

The MVP uses `ObjectType::Generic` plus schema IDs for Librarian objects:

```text
src/runtime/librarian_broadcast.rs:582-599
src/runtime/librarian_broadcast.rs:737-753
```

No diff was found to:

```text
src/bottom_white/cas/schema.rs
src/state/typed_tx.rs
src/state/sequencer.rs
```

### No raw broadcast leak

The new paths use forbidden-material checks, safe redaction labels, and a
dashboard count/CID materialized view:

```text
src/runtime/librarian_broadcast.rs:179
src/runtime/librarian_broadcast.rs:727-733
src/bin/audit_dashboard.rs:2873-2905
```

### Half-async binding

The barriered async validator exists and targeted tests pass:

```text
src/runtime/market_review.rs:141-174
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/artifacts/command_0001_stdout.txt
```

The reviewer noted there is not yet a production callsite for
`validate_market_review_broadcast_contract`; barriered-async ship would need
additional callsite evidence. REAL-BCAST-1 currently keeps full async out of
ship scope.

## Passing Pre-Rehash Checks

Recorded in:

```text
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/events.jsonl
```

Commands:

```text
cargo test --test constitution_librarian_source_scope
           --test constitution_librarian_selector
           --test constitution_librarian_digest
           --test constitution_librarian_role_projector
           --test constitution_librarian_prompt_injection
           --test constitution_librarian_half_async
           --test constitution_librarian_no_raw_leakage
           --test constitution_librarian_real_evidence_binding
           --test constitution_real13b_market_review_window

cargo check -p minif2f_v4 --bin evaluator
cargo check --bin audit_dashboard
```

Result: all above commands exited 0.

## Ship Boundary

`VETO` blocks ship. Ship / real A-B cannot proceed without:

```text
1. explicit Class-4 Trust Root rehash ratification;
2. Trust Root rehash for evaluator.rs and any other pinned modified file;
3. verify_trust_root passing;
4. REAL-BCAST A/B rerun with both arms audit_tape PROCEED;
5. clean-context ship audit after post-rehash evidence.
```
