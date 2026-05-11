# TB-N2 B2 — Gemini DeepThink PRE-§8 Audit R1

> **Dispatched + concluded 2026-05-11** PRE-§8 dual audit R1 on parallel
> branch `feat/n2-b2-event-resolve` HEAD `7dc2aa0`. Dispatched via
> general-purpose Agent subagent with explicit Gemini DeepThink framing.

## Audit context

Parallel to Codex G2 R1 audit. R1 evidence baseline:
- Branch: `feat/n2-b2-event-resolve` HEAD `7dc2aa0`
- Validation: cargo check clean, workspace 1447/0/151, constitution gates 287/0/1, trust_root 4/4
- Smoke: `stage_b3_smoke_b2_20260511T012401Z` (6/6 verdict=Ok delta=0; cell 2 OmegaAccepted + EventResolve emit log)

## Verdicts (verbatim from Gemini DT output 2026-05-11)

```
Q1: PASS - `EventResolveTx` at `src/state/typed_tx.rs:1597-1604` has exactly 6 wire fields (`tx_id`, `parent_state_root`, `task_id`, `epoch`, `timestamp_logical`, `system_signature`); `EventResolveSigningPayload` at `:1794-1800` has 5 fields (`system_signature` excluded); `to_signing_payload` at `:1923-1935` is lossless 1:1 copy; `canonical_digest` at `:1805-1807` calls `domain_prefixed_digest(DOMAIN_SYSTEM_EVENT_RESOLVE, self)`. Sibling pattern matches 5 prior system-tx.

Q2: PASS - `DOMAIN_SYSTEM_EVENT_RESOLVE = b"turingosv4.system_sig.event_resolve.v1"` at `src/state/typed_tx.rs:894` is byte-distinct from the 5 prior `system_sig.*.v1` constants at `:880-884`. `CanonicalMessage::EventResolveSigning` discriminator `b"EventResolveSigning"` at `src/bottom_white/ledger/system_keypair.rs:525` is unique in the digest match (`:485-527`).

Q3: PASS - `TypedTx::EventResolve(er)` arm at `src/state/sequencer.rs:1832-1869`: Step 2 (`:1845-1852`) explicit match on `TaskMarketState::Open => proceed` vs `Finalized | Bankrupt | Expired => return Err(EventAlreadyResolved)`. No `_ =>` catch-all — all 4 q_state variants enumerated. Verified by `sg_n2_b2_3_re_emit_on_finalized_rejects` + `sg_n2_b2_4_emit_on_bankrupt_market_rejects` (both PASS).

Q4: PASS - Two defense layers verified: (1) emit-time at `src/state/sequencer.rs:4268-4270` — `if !q_snap.economic_state_t.task_markets_t.0.contains_key(&task_id) { return Err(EmitSystemError::EventResolveTaskNotFound); }` (strict `!contains_key`, no `unwrap_or`); (2) dispatch-time at `:1839-1842` — `match ... { Some(e) => e.clone(), None => return Err(TransitionError::EventResolveTaskNotFound) }`. Fail-closed honors `feedback_admission_fail_closed_default`.

Q5: PASS - Agent ingress rejection at `src/state/sequencer.rs:3830-3836`: `TypedTx::EventResolve(_)` arm of `submit_agent_tx` returns `SubmitError::SystemTxForbiddenOnAgentIngress` pre-queue alongside ChallengeResolve / TaskBankruptcy. `impl HasSubmitter for EventResolveTx` at `src/state/typed_tx.rs:2269-2273` returns `None`. SG-N2-B2.1 PASS.

Q6: PASS - Dispatch arm Step 4 at `src/state/sequencer.rs:1862-1865` calls BOTH `assert_no_post_init_mint(tx, q)` (additive Ok arm at `src/economy/monetary_invariant.rs:406-413`) AND `assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])` (empty exempt list — strictest mode). Both checks fire BEFORE state_root advance (`:1867`); failure maps to `MonetaryInvariantViolation`. SG-N2-B2.6 confirms pre/post equality.

Q7: PASS - `EVENT_RESOLVE_DOMAIN_V1 = b"turingosv4.event_resolve.accept.v1"` at `src/state/sequencer.rs:259-260` is byte-distinct from all 18 other `*_DOMAIN_V1` accept-state-root constants (`:59-381` enumerated). Crucially distinct from `b"turingosv4.system_sig.event_resolve.v1"` (signing domain) — separate `.accept.` vs `.system_sig.` namespaces prevent cross-context replay.

Q8: PASS - Adapter at `src/runtime/adapter.rs:673-710`: poll-then-emit with `poll_budget_ms`; returns `Ok(true)` on emit, `Ok(false)` when task observed non-Open or poll expired (idempotent), `Err` only on unexpected `emit_system_tx` error. Called at 2 evaluator sites: `experiments/minif2f_v4/src/bin/evaluator.rs:2834` (full-proof OMEGA) + `:3518` (per-tactic OMEGA). Smoke evidence at `handover/evidence/stage_b3_smoke_b2_20260511T012401Z/`: 6 cells × `chain_invariant.json` all `verdict=Ok delta=0`; exactly 1 `EventResolve emitted` in deepseek/aime_1983_p2/evaluator.stderr; 0 `EventAlreadyResolved`/`EventResolveTaskNotFound` across all 6 cells.

Q9: PASS - `genesis_payload.toml` diff at `7dc2aa0` rehashes exactly 8 STEP_B files: `evaluator.rs` + `system_keypair.rs` + `transition_ledger.rs` + `monetary_invariant.rs` + `adapter.rs` + `run_summary.rs` + `sequencer.rs` + `typed_tx.rs`. `audit_assertions.rs` is touched but not in the STEP_B hash list (consistent with its non-protocol-critical status). `cargo test -p minif2f_v4 --test trust_root_immutability` PASS (4/4 including `test_trust_root_manifest_includes_b2_b4_files`).

**Class-4 boundary deep review**: `src/bottom_white/ledger/system_keypair.rs` diff (3 hunks at `:265-272`, `:524-527`, `:676-689`) is strictly additive — new `CanonicalMessage::EventResolveSigning([u8; 32])` variant (opaque-digest pattern matching 5 prior system-tx variants), new digest-discriminator arm `b"EventResolveSigning"`, new `pub(crate) fn sign_event_resolve` helper (no `pub` widening; same `pub(crate)` scope as siblings). No modification to existing 5 system-tx sign helpers / digests / discriminators / variants. Boundary properly bounded.

## VERDICT: PASS
Conviction: high
Recommendation: PROCEED
```

## R1 conflict resolution

Codex G2 R1 returned VETO (Q8 race condition + Q9 manifest gap) — see
`handover/audits/CODEX_G2_TB_N2_B2_PRE8_AUDIT_R1_FULL_LOG.log`.
Gemini DT R1 returned PASS-all-9. Per `feedback_dual_audit_conflict`
conservative-merge: VETO > PASS → aggregate VETO. Gemini DT R1 missed
the Q8 race because it relied on absence-of-string grep
(`EventAlreadyResolved` / `EventResolveTaskNotFound`) rather than
deep-walking `rejections.jsonl`. Codex G2 R1 caught it via deep CAS +
ChainTape evidence walk.

This R1 contrast is documented in `feedback_dual_audit` and informed
the R2 audit prompts (which explicitly direct deep evidence walks on
rejections.jsonl).
