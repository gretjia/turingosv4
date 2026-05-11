# TB-N2 B2 — Codex G2 PRE-§8 Audit R2 (post race-fix)

> **Dispatched + concluded 2026-05-11** post R2 fix on parallel branch
> `feat/n2-b2-event-resolve`. Full Codex CLI log:
> `handover/audits/CODEX_G2_TB_N2_B2_PRE8_AUDIT_R2_FULL_LOG.log`.
> Dispatch prompt: `/tmp/codex_b2_r2_audit_prompt.md` (transient).
>
> R1 dispatch prompt: `handover/audits/CODEX_G2_TB_N2_B2_PRE8_AUDIT_R1.md`
> R1 full log: `handover/audits/CODEX_G2_TB_N2_B2_PRE8_AUDIT_R1_FULL_LOG.log`

## Audit context (R2)

R1 returned VETO on Q8 (adapter race causing StaleParent L4.E) + Q9
(audit_assertions.rs missing from Trust Root manifest). R2 fix applied:
- `src/runtime/adapter.rs` helper now accepts `verify_tx_id: &TxId` 3rd
  parameter; polls `claims_t[claim_id].status == ClaimStatus::Finalized`
  ALONGSIDE `task_markets_t.state == Open` before emit
- 3 call sites updated to pass `&vid` / `&verify_tx_id`
- `genesis_payload.toml`: ADDED `src/runtime/audit_assertions.rs` entry;
  REHASHED adapter.rs + evaluator.rs
- NEW `SG-N2-B2.9` source-grep binding gate

## R2 verdicts (verbatim from Codex G2 output 2026-05-11)

```
Q1: PASS - `EventResolveTx` is 6 wire fields and its signing payload is the 5 signed fields excluding `system_signature`; projection is field-for-field stable. `src/state/typed_tx.rs:1597`, `src/state/typed_tx.rs:1794`, `src/state/typed_tx.rs:1923`

Q2: PASS - `DOMAIN_SYSTEM_EVENT_RESOLVE` is unique among system signing domains, and `CanonicalMessage::EventResolveSigning` has its own discriminator and signer. `src/state/typed_tx.rs:880`, `src/state/typed_tx.rs:894`, `src/bottom_white/ledger/system_keypair.rs:272`, `src/bottom_white/ledger/system_keypair.rs:524`, `src/bottom_white/ledger/system_keypair.rs:684`

Q3: PASS - Dispatch admits only `TaskMarketState::Open`; `Finalized | Bankrupt | Expired` explicitly reject as `EventAlreadyResolved`, so monotonic resolution is preserved. `src/state/sequencer.rs:1832`, `src/state/sequencer.rs:1845`

Q4: PASS - Missing task is fail-closed at construction and apply: `emit_system_tx` returns `EventResolveTaskNotFound`, and dispatch also rejects absent `task_markets_t`. `src/state/sequencer.rs:1839`, `src/state/sequencer.rs:4268`, `tests/constitution_n2_event_resolve.rs:311`

Q5: PASS - Anti-Oreo boundary holds: `EventResolveTx` has no submitter and `submit_agent_tx` rejects it pre-queue as `SystemTxForbiddenOnAgentIngress`. `src/state/typed_tx.rs:2269`, `src/state/sequencer.rs:3817`, `src/state/sequencer.rs:3830`, `tests/constitution_n2_event_resolve.rs:170`

Q6: PASS - EventResolve only flips task state, then runs `assert_no_post_init_mint` and strict `assert_total_ctf_conserved(..., &[])` before state-root advance; SG-N2-B2.6 checks balances/collateral/LP/pools/shares/supply unchanged. `src/state/sequencer.rs:1856`, `src/state/sequencer.rs:1862`, `tests/constitution_n2_event_resolve.rs:328`

Q7: PASS - accept-state root uses the distinct `b"turingosv4.event_resolve.accept.v1"` domain, separate from system signing and sibling accept domains. `src/state/sequencer.rs:259`, `src/state/sequencer.rs:266`

Q8: PASS - R2 closes the observed race. `claim_id_inner` byte-matches TB8's `format!("claim-{}", verify_tx_id.0)`; the helper waits for `claims_t[claim_id].status == Finalized` plus task state `Open`, keeps the 20ms sleep, explicitly early-outs on `Finalized | Bankrupt | Expired`, then emits unchanged `SystemEmitCommand::EventResolve`. `src/runtime/adapter.rs:622`, `src/runtime/adapter.rs:710`, `src/runtime/adapter.rs:719`, `src/runtime/adapter.rs:728`, `src/runtime/adapter.rs:745`, `src/runtime/adapter.rs:753`, `src/runtime/adapter.rs:758`. The apply witness is real: Verify creates the claim Open, FinalizeReward flips it Finalized and advances root. `src/state/sequencer.rs:1169`, `src/state/sequencer.rs:1507`, `src/state/sequencer.rs:1545`. Both evaluator call sites are under `if let Some(vid)` and pass `&vid`. `experiments/minif2f_v4/src/bin/evaluator.rs:2814`, `experiments/minif2f_v4/src/bin/evaluator.rs:2837`, `experiments/minif2f_v4/src/bin/evaluator.rs:3508`, `experiments/minif2f_v4/src/bin/evaluator.rs:3527`

Q9: PASS - Manifest now includes `src/runtime/audit_assertions.rs = cb6acb8e...`; computed hashes match adapter/evaluator/audit_assertions exactly. `genesis_payload.toml:164`, `genesis_payload.toml:222`, `genesis_payload.toml:233`. `cargo test -p minif2f_v4 --test trust_root_immutability -- --test-threads=1` passed 4/4.

Q-NEW R2 binding: PASS - SG-N2-B2.9 passes and catches R1-shape revert on signature/call-site form (`verify_tx_id: &TxId`, claim-id derivation, `claims_t`, `ClaimStatus::Finalized`, `&vid` 3rd arg). `tests/constitution_n2_event_resolve.rs:477`. `cargo test --test constitution_n2_event_resolve -- --test-threads=1` passed 9/9.

## VERDICT: PASS
Conviction: high  
Recommendation: PROCEED

Verification run: `cargo check --workspace`, `cargo test --test constitution_n2_event_resolve -- --test-threads=1`, `cargo test -p minif2f_v4 --test trust_root_immutability -- --test-threads=1`, `sha256sum` for the three Q9 files, and R2 smoke grep confirming zero `"tx_kind":"EventResolve"` rejections.
```

## R1 verdicts (for reference; verbatim from Codex G2 R1 output)

```
Q1: PASS - <schema bit-stability verified>
Q2: PASS - <signing domain non-collision verified>
Q3: PASS - <monotonic resolution gate verified>
Q4: PASS - <fail-closed verified>
Q5: PASS - <Anti-Oreo verified>
Q6: PASS - <monetary invariant defense-in-depth verified>
Q7: PASS - <state_root domain separation verified>
Q8: VETO - "tx_kind:EventResolve rejected with public_summary:stale_parent_root at handover/evidence/stage_b3_smoke_b2_20260511T012401Z/.../rejections.jsonl:9" (adapter race)
Q9: VETO - "src/runtime/audit_assertions.rs is modified but has no direct manifest entry"

## VERDICT: VETO
Conviction: high
Recommendation: VETO
```

## R1 → R2 closure

| Aspect | R1 | R2 |
|---|---|---|
| Q1-Q7 verdict | PASS | PASS (preserved) |
| Q8 verdict | **VETO** (race) | **PASS** (race fixed via claim_finalized poll) |
| Q9 verdict | **VETO** (audit_assertions.rs missing) | **PASS** (entry added) |
| Q-NEW R2 binding | N/A | **PASS** (SG-N2-B2.9 added) |
| Aggregate | **VETO** | **PASS** (high conviction, PROCEED) |
| Conviction | high | high |

Round-cap=2 per `feedback_elon_mode_policy` satisfied: R1 VETO → R2 fix → R2 PASS.
