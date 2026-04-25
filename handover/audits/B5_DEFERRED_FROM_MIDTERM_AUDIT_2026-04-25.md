# Deferred from B2-B4 mid-term audit — pickup in B5

**Date**: 2026-04-25
**Source**: dual-audit verdict CHALLENGE/CHALLENGE on B2-B4 (`CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md` + `GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md`)
**Status of audit verdict items**: P0-A + P0-C fixed in B2-B4 commit; P0-B + P0-D + P0-E deferred to B5 scope per user directive (option 2).

This is a binding checklist — do not start B5 conformance battery work without resolving these items first or explicitly re-deferring them with a written rationale.

---

## P0-B — Schema v2 emit alignment

**Audit framing**:
- Codex: PputResult lacks `schema_version`, `progress: u8`, `run_id`, `split`, `rollback_count`, guardrails, model_snapshot, mode, etc. New B2-B4 rows lack `schema_version`, so B1's `RunRecord::from_json` dispatcher classifies them as **Legacy + extras**, not v2. The B1 schema contract is effectively unenforced on emitted artifacts.
- Gemini: narrower frame — `verified: Option<bool>` should be `progress_verified: Option<u8>` (or `progress: u8`) per B1's `RunAggregate` contract.

**Fix options for B5**:

1. **(preferred) Switch evaluator emission from `PputResult` to `RunAggregate`**.
   - Replace `PputResult` struct in evaluator.rs with `RunAggregate` (already defined in jsonl_schema.rs).
   - Populate every required RunAggregate field at make_pput time:
     - `schema_version: "v2.0"`
     - `run_id: String` (generate per-call from timestamp + theorem hash)
     - `split: String` (read from `SPLIT` env var: "adaptation" / "meta_validation" / "heldout")
     - `progress: u8` (= 1 if `runtime_accepted && post_hoc_verified` else 0)
     - `rollback_count: u32` (B-phase: always 0; Phase C+ when ArtifactState rollbacks land)
     - `model_snapshot: String` (= active_model env + API revision per F-2026-04-22-08)
     - `binary_sha256: String` (compute at boot; tied to Trust Root)
     - `mode: String` (`"full" | "panopticon" | "amnesia" | "soft_law" | "homogeneous"` — Phase C wires the toggle)
   - Drop legacy PputResult entirely.
   - hybrid_v1 condition either drops or aggregates as below (P0-D).

2. **(fallback) Add missing fields to PputResult, stamp schema_version**.
   - Less work but persists schema fragmentation. Only valid if RunAggregate switchover would block B5 cycle.
   - Must add at minimum: `schema_version: Option<String>` (set to "v2.0"), `progress: Option<u8>`, `run_id: Option<String>`, `split: Option<String>`.
   - Rename `verified: Option<bool>` → `progress_verified: Option<u8>` for naming consistency with RunAggregate.

**Acceptance criteria**:
- New B2-B5 rows pass `RunRecord::from_json` and dispatch as `RunRecord::V2(_)`, NOT `RunRecord::Legacy(_)`.
- A new conformance test `test_emit_dispatches_as_v2` reads a freshly-emitted jsonl line and asserts `matches!(rec, RunRecord::V2(_))`.
- Round-trip serialize → deserialize yields equal struct.
- Legacy v1 jsonl rows (pre-Paper-1) still parse via `RunRecord::Legacy` path (already covered by `test_legacy_jsonl_still_readable`).

---

## P0-D — hybrid_v1 drops failed-leg C_i

**Audit framing** (Codex): hybrid_v1 condition runs `run_oneshot` first; on failure runs `run_swarm`. The merged result construction at evaluator.rs:128-141 uses `..r2` field-spread to inherit, keeping only `r2`'s C_i / T_i / failed_branch_count / total_run_token_count / total_wall_time_ms. The failed oneshot leg's tokens vanish from the merged total.

**Fix options for B5**:

1. **Aggregate**: combine r1 + r2 cost fields explicitly. Pseudocode:
   ```rust
   let combined = PputResult {
       condition: "hybrid_v1".into(),
       time_secs: elapsed,  // already correct
       pput: ...,           // already correct
       tx_count: 1 + r2.tx_count,
       total_run_token_count: match (r1.total_run_token_count, r2.total_run_token_count) {
           (Some(a), Some(b)) => Some(a + b),
           (Some(a), None) | (None, Some(a)) => Some(a),
           _ => None,
       },
       failed_branch_count: match (r1.failed_branch_count, r2.failed_branch_count) {
           (Some(a), Some(b)) => Some(a + b),
           ...
       },
       total_wall_time_ms: match (r1.total_wall_time_ms, r2.total_wall_time_ms) {
           (Some(a), Some(b)) => Some(a + b),
           ...
       },
       ..r2
   };
   ```
2. **Disable hybrid_v1 for PPUT-CCL**: simpler. The PREREG arc only uses `oneshot` and `n*` conditions. Hybrid_v1 was a Paper 1 era condition. If no PPUT-CCL phase plans to use it, returning an error or skipping the condition keeps B5+ clean.

**Recommendation**: option 2 (disable) unless a downstream phase explicitly needs hybrid_v1.

**Acceptance criteria**:
- If aggregation chosen: a synthetic test where r1.total_run_token_count = 1000, r2.total_run_token_count = 5000 → combined.total_run_token_count == 6000 (NOT 5000).
- If disabled: hybrid_v1 condition returns an error or warning; documentation notes its deprecation.

---

## P0-E — `flip_last_failed_to_accepted` silent saturation

**Audit framing**: `cost_aggregator.rs:97-103` currently:
```rust
pub fn flip_last_failed_to_accepted(&mut self) {
    if self.failed_branch_count > 0 {
        self.failed_branch_count -= 1;
    }
}
```
Saturates at 0. Codex framed this as "silently masks over-flip wiring bugs" — a future call site that fires `flip_last_failed_to_accepted()` more times than `record_proposal(false)` would corrupt the count without panic. Gemini framed it as "robust against wiring bugs". Conservative reading wins.

**Fix for B5**:
```rust
pub fn flip_last_failed_to_accepted(&mut self) {
    assert!(
        self.failed_branch_count > 0,
        "flip_last_failed_to_accepted called with no failed proposal to flip — \
         wiring bug: caller fired flip more times than record_proposal(false)"
    );
    self.failed_branch_count -= 1;
}
```

**Or fallible**:
```rust
pub fn flip_last_failed_to_accepted(&mut self) -> Result<(), &'static str> {
    if self.failed_branch_count == 0 {
        return Err("no failed proposal to flip");
    }
    self.failed_branch_count -= 1;
    Ok(())
}
```

**Acceptance criteria**:
- A new conformance test `test_flip_underflow_panics` (or `_returns_err`) feeds the accumulator a flip without prior record_proposal(false) → panic / Err.
- Existing `test_failed_branches_counted_in_total_cost` still PASS (it only flips when there's a failed proposal to flip).

**Recommendation**: assert variant — surfaces wiring bugs at debug time without forcing every caller to handle a Result.

---

## B7 Trust Root recommendation (not blocking — record for B7 spec freeze)

Both auditors recommend adding the new B2-B4 metering files to the `[trust_root]` section of `genesis_payload.toml` (PREREG § 1.8). Codex adds three more.

**Files to add to Trust Root manifest in B7**:
- `experiments/minif2f_v4/src/cost_aggregator.rs` (B2)
- `experiments/minif2f_v4/src/wall_clock.rs` (B3)
- `experiments/minif2f_v4/src/post_hoc_verifier.rs` (B4)
- `experiments/minif2f_v4/src/jsonl_schema.rs` (B1 — emit-side schema)
- `experiments/minif2f_v4/src/bin/evaluator.rs` (the wiring; tampering with this defeats every layer)
- `src/drivers/llm_http.rs` (cost-source-of-truth — tampering with prompt_tokens reading defeats C_i accuracy)

When B7 lands, update `PREREG § 1.8` Trust Root file list AND `genesis_payload.toml [trust_root]` SHA-256 entries to include all six. The PREREG already specifies fallback enforcement; this just expands the manifest.

---

## Tracking

This document is the binding checklist. Do not close until B5 commit lands all three P0s OR re-defers each with explicit rationale.
