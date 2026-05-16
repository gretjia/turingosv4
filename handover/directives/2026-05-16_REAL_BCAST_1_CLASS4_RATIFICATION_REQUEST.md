# REAL-BCAST-1 Class-4 Ratification Request

Date: 2026-05-16

## Request

Approve REAL-BCAST-1 as a Trust-Root-rehashed package for evaluator-level
prompt injection.

The original REAL-BCAST-1 plan defaulted to Class 3 and required stop/escalate
on restricted authority surfaces. Implementation did not touch TypedTx,
sequencer admission, canonical signing payloads, constitution/flowcharts,
genesis authority logic, or CAS ObjectType schema. However, it did touch a
Trust-Root-pinned executable file:

```text
experiments/minif2f_v4/src/bin/evaluator.rs
```

The runner correctly refused to start real evidence:

```text
handover/evidence/real_bcast_1_ab_A_20260516T000000Z/
TRUST_ROOT_TAMPERED: evaluator.rs hash mismatch
```

Therefore real A/B evidence and ship cannot proceed until explicit Class-4
Trust Root ratification and rehash.

## What This Package Ratifies

```text
- LibrarianDigest / RoleNotificationView as CAS Generic sidecars.
- schema_id = "turingosv4.librarian_digest.v1" for digest objects.
- No CAS ObjectType enum edit.
- No new TypedTx.
- No out-of-band message system.
- Prompt delivery through existing PromptCapsuleV2 visible_context_cid/read_set.
- Half-async MarketReviewWindow/Response/Summary digest/epoch binding fields.
- Dashboard/run-report Librarian Broadcast materialized-view section.
```

## What This Package Does Not Ratify

```text
- live REAL-6B;
- full async ship path;
- price-as-truth;
- forced trade;
- raw Lean stderr/prompt/completion/CoT/diagnostics broadcast;
- dashboard/report as source of truth;
- spontaneous market emergence / E2;
- persistent role differentiation / E3;
- causal performance gain / E4.
```

## Verification Already Run Pre-Rehash

Targeted tests:

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

Result: pass
```

Affected binary checks:

```text
cargo check -p minif2f_v4 --bin evaluator
cargo check --bin audit_dashboard

Result: pass
```

Recorded in:

```text
handover/evidence/dev_self_hosting/dev_1778923029432_2760832/
```

## Required Post-Ratification Gates

```text
1. Trust Root rehash includes evaluator.rs and any other pinned modified file.
2. cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo -- --test-threads=1
3. bash scripts/run_constitution_gates.sh
4. REAL-BCAST A/B smoke:
   - Arm A: Librarian OFF
   - Arm B: Librarian ON
   - same tasks/models/budgets/seeds; only Librarian ON/OFF differs
5. audit_tape PROCEED for both arms.
6. clean-context Codex ship audit.
```
