# TB-18R R3.fix STEP_B Preflight — CasStore Split-Brain Reload

**Atom**: TB-18R R3.fix (post-R3 surgical correction).
**Class**: **4** (sequencer.rs touched; CLAUDE.md restricted-file list applies). STEP_B parallel-branch worktree required.
**Date**: 2026-05-06 (same day as R3 ship).
**Author**: Claude orchestrator.
**Predecessor**: TB-18R R3 SHIPPED 2026-05-06 (commit `72a1b75` via merge `66dde84`; main HEAD post-R3 = `4a38524`).
**Trigger**: L0 smoke run on `numbertheory_2pownm1prime_nprime` 2026-05-06 surfaced a CasStore split-brain bug — `refine_rejection_class_via_attempt_telemetry` returns `Err(CidNotFound)` and falls back to `RejectionClass::PredicateFailed` instead of refining to `LeanFailed=6` / `SorryBlocked=8`. Charter promise of fine-grained class on L4.E records was not met (chain still got 5 Work tx records — Art.III.4 satisfied — but their `rejection_class` discriminator was wrong).

## §1 Diagnosis (binding fact-pack from L0 smoke)

L0 smoke evidence at `/tmp/r3_l0_smoke_p49_1778052575/`:
  - 5 step_reject events on chain (`rejections.jsonl` submit_id 5..9; tx_kind=Work; agent_id=Agent_0)
  - All 5 records: `rejection_class: "PredicateFailed"` (=0), `public_summary: "predicate_failed"`
  - CAS sidecar (`cas/.turingos_cas_index.jsonl`): contains 5 `AttemptTelemetry` objects with `schema_id=turingosv4.attempt_telemetry.v1` (CIDs `b9ed620b…` `876a724b…` `461aa9f9…` `5ed96b94…` `16166b7f…`)
  - 5 `LeanResult` objects also present
  - **Conclusion**: R2 wire-up writes AttemptTelemetry to disk correctly; R3 evaluator wire-up submits failure-path WorkTx to chain correctly; but R3 sequencer admission helper `refine_rejection_class_via_attempt_telemetry` cannot SEE the AttemptTelemetry objects.

### §1.1 Root cause: in-memory CAS index split-brain

`src/bottom_white/cas/store.rs::CasStore::get(&self, cid)` (line 230) checks `self.index.get(cid)` FIRST and returns `Err(CidNotFound)` if absent — never falls through to disk.

`CasStore::put` (line 183) updates `self.index` AND appends to disk sidecar `.turingos_cas_index.jsonl`. Other handles holding their own `CasStore` instances do NOT see the new entry until they re-read the sidecar.

In R3 wire-up:
  - **Evaluator**: opens its own `CasStore::open(&bundle.cas_path)` per failure path. Writes AttemptTelemetry. Disk sidecar gets the entry.
  - **Sequencer**: holds `self.cas: Arc<RwLock<CasStore>>`, opened at startup. Its in-memory index is a snapshot from startup time. New entries written by the evaluator's separate handle are invisible.
  - **Refine helper**: queries via `self.cas.read()`. Hits `Err(CidNotFound)`. Falls back to base `PredicateFailed`.

This is a stale-cache bug, not a constitutional defect. Chain still has the records. Just lost the fine-grained discriminator.

### §1.2 Why TB-7 audits never hit this

Existing TB-7 audits (`audit_assertions.rs id=24`, `verify.rs Gate 5`, `tb_7_atom6_chain_backed_smoke`) all walk L4 **post-run** by re-opening CAS from disk. Their `CasStore::open(...)` reads the full sidecar at construction → in-memory index has every entry. They never query through the live sequencer's stale handle.

R3's refine helper is the **first in-run sequencer-side CAS read** of an evaluator-written object type. It exposes a latent issue that has been benign until now.

## §2 Scope (binding)

### §2.1 In scope (Class 4 STEP_B)

- **`src/bottom_white/cas/store.rs`** (Class 3 per CLAUDE.md restricted list — only `cas/schema.rs` is STEP_B-restricted, NOT `cas/store.rs`). Add:
  ```rust
  /// TB-18R R3.fix (preflight handover/ai-direct/TB-18R_R3FIX_STEP_B_cas_reload.md):
  /// re-read the on-disk sidecar `.turingos_cas_index.jsonl` into `self.index`,
  /// merging entries written by other CasStore handles after this one was opened.
  /// Idempotent. Used by long-lived sequencer.cas to pick up writes performed by
  /// short-lived evaluator-side CasStore instances on the same disk path.
  pub fn reload_index_from_sidecar(&mut self) -> Result<(), CasError> { … }
  ```
- **`src/state/sequencer.rs`** (Class 4 STEP_B). Modify `refine_rejection_class_via_attempt_telemetry`:
  - Existing fast path: `cas.read()` lock + `read_attempt_telemetry_from_cas(&cas_g, &proposal_cid)`.
  - On `Err(_)`: drop read lock → acquire write lock → call `cas_w.reload_index_from_sidecar()` → drop write lock → retry read with fresh read lock.
  - On second `Err(_)`: fall back to `base_class` (legacy ProposalTelemetry CID resolves NOT-an-AttemptTelemetry; documented behavior preserved).

### §2.2 Out of scope

- Any change to typed-tx schema, RejectionClass enum, AttemptTelemetry struct (R1 / R3 already shipped).
- Any change to evaluator wire-up (R2 / R3 already correct on the write side; bug is read-side only).
- General CasStore architecture rework (sequencer-owned-vs-shared CAS handle is a future TB).

## §3 Design decisions

### §3.1 Why `reload_index_from_sidecar` not `auto-fallback-to-disk-on-miss`

Auto-fallback would require `CasStore::get(&self, cid)` to fall through to disk on every miss. Two issues:
  - Hot path penalty: every legitimate CidNotFound (e.g., audit on a tampered chain) does an extra disk read.
  - Lock contention: `self` is `&self` (read-only); falling through would need interior mutability or lock promotion.

Explicit reload is conservative: caller decides when to re-sync. Cost is one disk read per call site; refine helper is on the rejection arm only (low frequency).

### §3.2 Lock promotion in refine helper

The refine helper currently takes `cas: &Arc<RwLock<CasStore>>` and acquires read lock. To call `reload_index_from_sidecar(&mut self)` we need write lock. Sequence:

```rust
let initial_read = { let g = cas.read()?; read_attempt_telemetry_from_cas(&g, &cid) };
match initial_read {
    Ok(att) => map_outcome(att.outcome),
    Err(_) => {
        // R3.fix retry: reload sidecar then re-read.
        if let Ok(mut w) = cas.write() {
            let _ = w.reload_index_from_sidecar();
        }
        let retry = { let g = cas.read()?; read_attempt_telemetry_from_cas(&g, &cid) };
        match retry {
            Ok(att) => map_outcome(att.outcome),
            Err(_) => base_class,  // legacy ProposalTelemetry → fall-through
        }
    }
}
```

Lock-poison handling: if `cas.write()` returns `Err`, treat as a hard miss → fall back to base_class. Rejection-arm latency degrades gracefully without panicking the chain.

### §3.3 Why retry-once not retry-loop

`reload_index_from_sidecar` is idempotent and complete (reads the entire sidecar). One retry suffices: either the entry is now present (race resolved) or it never will be (legacy ProposalTelemetry / corrupt CID / wrong type). Looping would waste time on permanent misses.

### §3.4 No `auto_reload_on_miss` env var

A "production toggle" would be soft alignment. Per `feedback_no_workarounds_strict_constitution`: the helper either has correct behavior (refine when AttemptTelemetry present) or doesn't. R3.fix makes correct behavior the only behavior.

## §4 Files touched

| File | Status | Class | Diff intent |
|---|---|---|---|
| `src/bottom_white/cas/store.rs` | MOD (additive) | 3 (NOT in STEP_B restricted list per CLAUDE.md) | NEW `pub fn reload_index_from_sidecar(&mut self) -> Result<(), CasError>` (~15 lines body + doc-comment). Reuses existing private `load_index_from_sidecar` free fn (line 99). |
| `src/state/sequencer.rs` | MOD (additive) | 4 (STEP_B restricted) | Modify `refine_rejection_class_via_attempt_telemetry` to retry on initial CidNotFound via reload; ~10 line surgical patch. |
| `genesis_payload.toml` | MOD (rehash) | TR pin | Update SHA pins for `src/bottom_white/cas/store.rs` + `src/state/sequencer.rs`; comment-chain entries cite this preflight. |
| `tests/tb_18r_cas_reload_split_brain.rs` | NEW | Witness | 2-3 tests: split-brain repro (handle A writes, handle B sees nothing, B.reload sees it); refine_rejection_class via stale cas + reload retry succeeds. |

Net delta target: ≥ +2 tests. Workspace 1017 → ≥ 1019.

## §5 Test plan

### §5.1 `cas_store_split_brain_repro`

  ```rust
  let dir = TempDir::new().unwrap();
  let mut a = CasStore::open(dir.path()).unwrap();
  let mut b = CasStore::open(dir.path()).unwrap();
  let cid = a.put(b"hello", ObjectType::Generic, "a", 0, None).unwrap();
  // B opened before A's write; B's index does NOT see it.
  assert!(matches!(b.get(&cid), Err(CasError::CidNotFound(_))));
  // R3.fix: reload picks up A's write.
  b.reload_index_from_sidecar().unwrap();
  assert_eq!(b.get(&cid).unwrap(), b"hello");
  ```

### §5.2 `refine_rejection_class_recovers_via_reload`

  Construct: open sequencer-style CAS handle A1, evaluator-style handle A2. A2 writes AttemptTelemetry (outcome=LeanFail). Build a WorkTx with proposal_cid → that AttemptTelemetry. Call `refine_rejection_class_via_attempt_telemetry(&Arc::new(RwLock::new(a1)), &tx, RejectionClass::PredicateFailed)`. Assert returns `LeanFailed` (the helper internally reloads when initial read misses).

### §5.3 `refine_rejection_class_falls_back_when_truly_absent`

  Build a WorkTx with proposal_cid pointing at a random Cid never in CAS. Refine should still fall back to `PredicateFailed` after retry-and-still-miss.

## §6 L0 smoke re-verify (binding ship-gate, not a test)

Per `feedback_smoke_before_batch`: re-run `/tmp/r3_l0_smoke_p49` against R3.fix-built evaluator. Assert:
  - rejections.jsonl: 5 Agent_0 Work tx with `rejection_class ∈ {LeanFailed, SorryBlocked}`. Specifically expect 3× LeanFailed (2 unknown_const + 1 tactic_other) + 2× SorryBlocked (sorry-detected on tx 1 + tx 3).
  - CAS sidecar still shows 5 AttemptTelemetry + 5 LeanResult (R2 wire-up unchanged).
  - tool_dist still `{step_reject:5, step:5}` (evaluator behavior unchanged; only sequencer-side class refinement changed).

Smoke is the empirical validation that distinguishes R3.fix from R3 — without re-running smoke, we cannot prove the bug is fixed.

## §7 Risk matrix

| Risk | Probability | Mitigation |
|---|---|---|
| reload_index_from_sidecar reads stale sidecar (atomicity) | Low — sidecar appended via durable `flush()+sync_data()` per existing `append_to_sidecar` (store.rs:213) | n/a |
| Lock-promotion deadlock (read → drop → write → drop → read) | Low — sequencer is single-writer per spec; no readers held during the drop-and-promote window | Test `refine_rejection_class_recovers_via_reload` exercises the path |
| Reload mutates `index` while another reader holds read-lock | Mitigated by RwLock semantics — read locks block during write-lock acquisition | n/a |
| Performance regression on rejection arm | Low — one extra sidecar read per rejection; rejection arm already does CAS put for tx_payload + raw_diagnostic; +1 read is sub-millisecond | R5+ may switch to sequencer-owned CAS; out of scope |
| L0 smoke re-run still shows PredicateFailed (fix didn't take) | High-cost-if-true | Block ship; debug |
| Trust Root rehash for store.rs + sequencer.rs trips audit | Low — same precedent as R3 ship (3 pins rehashed) | Routine |
| New variant of split-brain at R5 audit_tape walk | Out of scope here | R5 preflight will inherit reload pattern |

## §8 STEP_B parallel-branch plan

1. Phase 0 (this preflight): scope + design + risk matrix.
2. Phase 1a: `git worktree add .claude/worktrees/stepb-tb18r-r3fix-cas-reload -b tb-18r-r3fix-cas-reload main`.
3. Phase 1b: implement (`store.rs` + `sequencer.rs` + tests + Trust Root rehash).
4. Phase 1c: `cargo test --workspace --no-fail-fast` ≥ 1019 / 0 R3.fix-attributable / 150 ignored.
5. Phase 1d (binding for R3.fix specifically): re-run L0 smoke per §6; assert rejection_class ∈ {6, 8}.
6. Phase 2 (skipped): no statistical A/B; smoke verify suffices.
7. Phase 3: merge `tb-18r-r3fix-cas-reload --no-ff` to main; TB_LOG R3.fix entry; worktree remove.

**Iteration cap**: 4h (this is a surgical bug fix, not new construction).

## §9 Forbidden in R3.fix

- Modifying CasStore::put or CasStore::get hot-path semantics (auto-fallback-on-miss rejected per §3.1).
- Adding env-var toggle for reload (rejected per §3.4).
- Modifying typed-tx schema, RejectionClass enum, AttemptTelemetry, evaluator wire-up.
- Touching omega-path proposal_cid (still ProposalTelemetry CID per R3 §3.5 amended).
- Routing legacy WorkTx to refined classes (legacy proposal_cid → ProposalTelemetry decode-fail-on-AttemptTelemetry → fall back to base; this remains correct).
- Capturing Lean stderr/stdout into CAS (still R5+ scope).
- Any code change motivated by "to pass the smoke earlier" rather than "to fix the bug correctly".

## §10 Sign-off

This preflight grants implementation authority for R3.fix within the listed scope upon user "go" (already given as "A").
- Pre-impl: this doc filed.
- Post-impl: TB_LOG R3.fix entry; worktree cleanup; OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06 §4 forward-binding to G2 still applies (G2 audits R3 + R3.fix as a unit).

**End of R3.fix preflight. Implementation starts on user "go".**
