# TB-N2 B2 — Codex G2 PRE-§8 Audit (R1)

> **Dispatched 2026-05-11** at packet-draft time per `feedback_dual_audit`
> Class-4 timing rule (PRE-§8 dispatch, not AFTER architect §8 request).
> Branch: `feat/n2-b2-event-resolve` HEAD `7dc2aa0`.

---

## Audit context

**TB**: TB-N2-POLYMARKET-CPMM-LIFECYCLE atom B2 (EventResolveTx system-emit).
**Class**: 3 with Class-4 canonical-signing-payload boundary touched
(`src/bottom_white/ledger/system_keypair.rs`). STEP_B parallel branch.
**Charter**: `handover/tracer_bullets/TB_N2_POLYMARKET_CPMM_LIFECYCLE_charter_2026-05-10.md` §3 B2.
**Gap audit**: `handover/audits/STAGE_C_POLYMARKET_CPMM_LIFECYCLE_GAP_AUDIT_2026-05-10.md` §3.3.
**§8 packet**: `handover/directives/2026-05-11_TB_N2_B2_§8_PACKET.md`.

**Validation at HEAD `7dc2aa0`**:
- `cargo check --workspace`: clean
- `cargo test --workspace --test-threads=1`: 1447 / 0 / 151 (+8 vs 1439 pre-B2)
- `bash scripts/run_constitution_gates.sh`: 287 / 0 / 1 (+8 vs 279 pre-B2)
- `cargo test -p minif2f_v4 --test trust_root_immutability`: PASS (8 files rehashed)

**Real-LLM smoke** (`stage_b3_smoke_b2_<TS>` 3 problems × 2 models × seed=1 × rep=1 = 6 cells; aime_1983_p2 deepseek = guaranteed OmegaAccepted → B2 emit trigger): TBD — fill in evidence section before audit dispatch.

---

## Audit questions

For each question: **PASS** / **CHALLENGE** / **VETO**, with reasoning + concrete file:line refs. Conviction: high / medium / low. Recommendation: PROCEED / RE-AUDIT / VETO.

### Q1 — EventResolveTx schema bit-stability

`EventResolveTx` defined at `src/state/typed_tx.rs` (search "pub struct EventResolveTx"). 6 wire fields: tx_id / parent_state_root / task_id / epoch / timestamp_logical / system_signature.

`EventResolveSigningPayload` is the 5-field projection (excludes system_signature). `to_signing_payload` projection is lossless on the 5 signed fields.

`canonical_digest` uses `domain_prefixed_digest(DOMAIN_SYSTEM_EVENT_RESOLVE, self)`.

**Verify**: schema is canonical-encodable + signing payload projection is bit-stable + matches 5 prior system-tx sibling pattern (FinalizeReward / TaskExpire / TerminalSummary / ChallengeResolve / TaskBankruptcy).

### Q2 — sign_event_resolve domain prefix non-collision

Signing domain: `DOMAIN_SYSTEM_EVENT_RESOLVE` = `b"turingosv4.system_sig.event_resolve.v1"` at `src/state/typed_tx.rs`.

5 prior system-tx signing domains:
- `b"turingosv4.system_sig.finalize_reward.v1"`
- `b"turingosv4.system_sig.task_expire.v1"`
- `b"turingosv4.system_sig.terminal_summary.v1"`
- `b"turingosv4.system_sig.challenge_resolve.v1"`
- `b"turingosv4.system_sig.task_bankruptcy.v1"`

`sign_event_resolve` at `src/bottom_white/ledger/system_keypair.rs` uses `CanonicalMessage::EventResolveSigning` discriminator `b"EventResolveSigning"`.

**Verify**: domain prefix is unique across all 6 system-tx signing domains; CanonicalMessage discriminator is unique across all variants.

### Q3 — Dispatch arm monotonic resolution gate

`dispatch_transition` EventResolve arm at `src/state/sequencer.rs` (search `TypedTx::EventResolve(er)`).

Step 2 monotonic gate:
- Open → proceed
- Finalized → `EventAlreadyResolved`
- Bankrupt → `EventAlreadyResolved`
- Expired → `EventAlreadyResolved`

**Verify**: gate enumerates ALL 4 TaskMarketState variants explicitly (no `_ =>` catch-all that would silently admit unknown variants); Open is the SOLE source state; cross-system-tx monotonicity respected (TaskBankruptcy → Bankrupt is parallel terminal NO-side; EventResolve cannot overwrite).

Verified by `tests/constitution_n2_event_resolve.rs` SG-N2-B2.3 (idempotent re-emit on Finalized) + SG-N2-B2.4 (rejection on Bankrupt).

### Q4 — fail-closed on missing task_markets_t entry

Two defense-in-depth layers:
- Construction (caller-side): `Sequencer::emit_system_tx(SystemEmitCommand::EventResolve)` returns `EmitSystemError::EventResolveTaskNotFound` when `task_markets_t.0.get(&task_id)` is `None`.
- Dispatch (apply-time): EventResolve arm Step 1 also matches `None → TransitionError::EventResolveTaskNotFound`.

**Verify**: BOTH layers exist + are wired (not just one); admission is fail-CLOSED on missing entry (no fail-open `unwrap_or(Open)` per `feedback_admission_fail_closed_default` lesson from Stage C overall §8 R2 Q10).

Verified by SG-N2-B2.5 (emit on unknown task_id returns EventResolveTaskNotFound at construction).

### Q5 — Anti-Oreo agent-ingress rejection

`Sequencer::submit_agent_tx(TypedTx::EventResolve)` must reject pre-queue with `SubmitError::SystemTxForbiddenOnAgentIngress` regardless of signature validity. Construction-through-emit-system-tx is the SOLE legal path.

**Verify**: rejection is pre-queue (not in dispatch); `HasSubmitter for EventResolveTx` returns `None`; the agent ingress arm catches it via the system-tx whitelist.

Verified by SG-N2-B2.1.

### Q6 — Monetary invariant defense-in-depth

EventResolve is pure status mutation (no balances_t / conditional_collateral_t / lp_share_balances_t / pool reserves / share holdings movement). But the dispatch arm Step 4 still calls:
- `assert_no_post_init_mint(tx, q)` (trivially passes; no mint on status flip)
- `assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])` with EMPTY exempt list (strictest mode; conservation verified pre→post)

**Verify**: both invariants called BEFORE state_root advance (Step 5); empty exempt list (no exemption needed since no token movement); SG-N2-B2.6 verifies pre/post equality on 5 holding tables + total_supply_micro.

### Q7 — state_root domain separation

`EVENT_RESOLVE_DOMAIN_V1` = `b"turingosv4.event_resolve.accept.v1"` at `src/state/sequencer.rs`.

11 other state_root accept domains (search `pub(crate) const .*_DOMAIN_V1` in sequencer.rs):
- TaskOpen / EscrowLock / WorkTx / Verify / Challenge / ChallengeResolve / FinalizeReward / TaskExpire / TerminalSummary / TaskBankruptcy / CompleteSet*

Distinct from the SIGNING domain `b"turingosv4.system_sig.event_resolve.v1"` (different namespace prefix `event_resolve.accept` vs `system_sig.event_resolve`).

**Verify**: accept-domain string is unique across all 11+ siblings; sibling pattern matches `b"turingosv4.<txname>.accept.v1"` form.

### Q8 — Evaluator hook idempotency in real-LLM smoke

`tb_n2_emit_event_resolve_after_finalize` at `src/runtime/adapter.rs` is poll-then-emit with budget. Returns `Ok(true)` on emit, `Ok(false)` on already-resolved or budget-expired, `Err` on unexpected `EmitSystemError`.

Called at 2 sites in `experiments/minif2f_v4/src/bin/evaluator.rs` (full-proof OMEGA-Confirm exit + per-tactic OMEGA exit). Mirrors `tb8_emit_finalize_after_verify` 2-site pattern.

**Verify** (after smoke evidence): no `EventAlreadyResolved` L4.E entries in OMEGA-Confirm cells (would indicate double-emit race); no `EventResolveTaskNotFound` (would indicate adapter polling missed the task lifecycle); ≥1 cell with `EventResolve emitted` log.

Verified statically by SG-N2-B2.8 source-grep gate (helper defined in adapter.rs AND called ≥2 sites in evaluator.rs).

### Q9 — Trust Root rehash completeness

`genesis_payload.toml` Trust Root manifest must rehash all STEP_B + adjacent files that were modified at HEAD `7dc2aa0`:
- `src/state/typed_tx.rs` (STEP_B)
- `src/state/sequencer.rs` (STEP_B)
- `src/bottom_white/ledger/system_keypair.rs` (STEP_B Class-4 boundary)
- `src/bottom_white/ledger/transition_ledger.rs` (STEP_B)
- `src/runtime/adapter.rs` (adjacent)
- `src/economy/monetary_invariant.rs` (adjacent)
- `src/runtime/audit_assertions.rs` (adjacent)
- `src/runtime/run_summary.rs` (adjacent)

**Verify**: all 8 sha256 entries updated; `cargo test -p minif2f_v4 --test trust_root_immutability` PASS at HEAD.

---

## Expected output format

```
Q1: PASS|CHALLENGE|VETO - <reasoning + file:line>
Q2: PASS|CHALLENGE|VETO - <reasoning + file:line>
...
Q9: PASS|CHALLENGE|VETO - <reasoning + file:line>

## VERDICT: PASS | CHALLENGE | VETO
Conviction: high | medium | low
Recommendation: PROCEED | RE-AUDIT | VETO
```

## Audit policies

- **Conservative**: VETO > CHALLENGE > PASS per `feedback_dual_audit_conflict`.
- **Round cap**: 2 per `feedback_elon_mode_policy` (R1 dispatch → if CHALLENGE → R2 fix + re-audit → halt at R2; if both CHALLENGE → halt unless user explicit override).
- **Class-4 hidden in Class-3**: per `feedback_class4_cannot_hide_in_class3` — `src/bottom_white/ledger/system_keypair.rs` change is the Class-4 boundary touch (canonical signing payload extension). User authorization 2026-05-11 "candidate impl on parallel branch" framed as Class-3 STEP_B with Class-4-boundary acknowledged; per-atom architect §8 required for ship per CLAUDE.md §10. Confirm boundary touch is bounded to: (a) new CanonicalMessage variant + (b) new sign_event_resolve helper + (c) discriminator branch in canonical_digest. NO modification to existing 5 system-tx sign helpers / digests / discriminators.

---

**End of B2 R1 audit dispatch.**
