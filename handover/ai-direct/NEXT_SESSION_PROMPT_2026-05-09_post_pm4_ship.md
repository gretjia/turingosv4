# Next Session Boot Prompt — 2026-05-09 session #31 close (post P-M4 SHIPPED FINAL)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session. Everything above it is context for cold-start orientation if you read this file directly.

---

## State at session #31 close (2026-05-09)

- **HEAD on `origin/main`**: `55a4c38` (pushed; lineage `92cfeb6` → `023fe32` (P-M4 atomic) → `d9d2b0b` (sign-off doc) → `008d9a3` (--no-ff merge) → `9f4ea3b` (LATEST.md) → `55a4c38` (R-020 retire + 2 new feedbacks)).
- **Phase F.3 P-M4: SHIPPED FINAL** — Class-4 STEP_B per remediation directive §1.C row 3. Architect §8: **「签字，同意后续执行」** verbatim multi-clause.
- **PRE-§8 dual audit**: Codex G2 R1 PASS (8/8 high) + Gemini R1 PASS (8/8 high) — first-try; round cap 2 used 1.
- **Constitution gates**: `207 PASS / 0 FAIL / 1 ignored` (was 203 pre-F.3; +4 from new `constitution_cpmm_pool` gate).
- **Workspace tests**: `1340 PASS / 0 FAIL / 151 ignored` (was 1336 pre-F.3; +4 architect-mandated verbatim tests).
- **Trust Root**: PASS (rehashed 7 STEP_B files + 2 hook/manifest files for R-020 retirement).
- **EconomicState**: 13 → 15 sub-fields (+`cpmm_pools_t` +`lp_share_balances_t`; pool reserves + LP shares NOT Coin per architect §7.5 rules 2 + 3).
- **F-DEFERRAL-2 closure**: closed for P-M4 via `CpmmPoolSigningPayload` sibling binding Landed.
- **R-020 retired** post-P-M4 `/harness-reflect`: 2 consecutive cycles with 0 cumulative triggers; YAML moved active/→retired/; judge.sh block commented out (40 lines preserved for revival); rules/MANIFEST.sha256 updated.
- **2 new feedback memory entries**: `feedback_codex_bash_exec_direct_dispatch.md` (Skill→Bash fallback) + `feedback_market_quarantine_token_exemption.md` (architect-spec'd token banned-list removal pattern).

## What landed in P-M4

| Surface | Change |
|---------|--------|
| `src/state/q_state.rs` | `LpShareAmount` u128 newtype + `PoolStatus` enum (Active/Resolved/Closed) + `CpmmPool` 5-field architect §7.5 verbatim state struct + `CpmmPoolsIndex` + `LpShareBalancesIndex` newtypes; EconomicState 13→15 with `+cpmm_pools_t` + `+lp_share_balances_t` `#[serde(default)]` |
| `src/state/typed_tx.rs` | `DOMAIN_AGENT_CPMM_POOL` + `CpmmPoolTx` 7-field wire (NO `timestamp_logical`) + `CpmmPoolSigningPayload` 6-field + `to_signing_payload` + canonical_digest + `TypedTx::CpmmPool` variant + tx_kind dispatch + `HasSubmitter` + 4 new `TransitionError` (`InvalidPoolSeed` / `UnbalancedPoolSeed` / `InsufficientSharesForPool` / `PoolAlreadyExists`) + Display |
| `src/state/sequencer.rs` | `CPMM_POOL_DOMAIN_V1` + `cpmm_pool_accept_state_root` + CpmmPool admission arm (5 preconditions + 3 atomic mutations + 3 monetary invariants) + 4 fan-out match arms + agent-sig manifest verify arm (provider as signer) |
| `src/bottom_white/ledger/transition_ledger.rs` | `TxKind::CpmmPool = 15` |
| `src/economy/monetary_invariant.rs` | `assert_no_post_init_mint` allow-list extended; `assert_complete_set_balanced` extended to count `cpmm_pools_t[event_id].pool_yes/no` in symmetric-branch sum_yes/sum_no totals |
| `src/runtime/verify.rs` + `run_summary.rs` + `audit_assertions.rs` | Replay-time Gate 4 verify arm + tx_id extractor + counter (`cpmm_pool: u64`) |
| `tests/constitution_cpmm_pool.rs` (NEW; 472 lines) | 4 architect §7.5 verbatim test names through live `Sequencer::submit_agent_tx` |
| `tests/constitution_architect_verbatim_struct_binding.rs` | P-M4 CpmmPool binding flipped Landed + `CpmmPoolSigningPayload` sibling Landed (F-DEFERRAL-2 closure) + parser hardening for path-qualified types |
| `tests/constitution_market_quarantine.rs` | ` CPMM` removed from `HARD_BANNED_LEGACY_TOKENS` (rationale: architect-spec'd CPMM landed) |
| `genesis_payload.toml` | 7 STEP_B file rehashes + 2 hook/manifest rehashes (R-020 retirement) |
| `scripts/run_constitution_gates.sh` | Registered `constitution_cpmm_pool` gate |

**`assert_complete_set_balanced` extension** (Phase F.3 substantive monetary invariant change):

```rust
// Stage C P-M4 / Phase F.3: pool reserves are claims against the SAME locked
// collateral; symmetric-branch strict-equality MUST count them.
if let Some(pool) = s.cpmm_pools_t.0.get(event_id) {
    sum_yes = sum_yes.checked_add(pool.pool_yes.units).ok_or(MonetaryError::Overflow)?;
    sum_no = sum_no.checked_add(pool.pool_no.units).ok_or(MonetaryError::Overflow)?;
}
```

The asymmetric branch (post-resolution `min()` reduction) is **unchanged** + still CTF-MIN-SAFE marker-protected.

## Pre-flight gates fired this session

- `/constitution-landing-check` — verdict PROCEED (matrix is 0 cell-anchored AMBER per session #24 strict closure; the 41 grep hits are historical "was 🟡 AMBER" annotations in GREEN cells + legend; tightened regex `\| 🟡 AMBER` returns 0).
- `/harness-reflect` (mandatory post-P-M4-SHIPPED-FINAL gate) — verdict harness HEALTHY (score 0.72; 13/16 active rules trigger; R-020 retired). Top recommendations executed in same session: R-020 retire + 2 NEW feedback memory files.

## Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §12 STEP_B; §13 economy laws).
2. **`constitution.md`** — top-level law (architect verbatim spec).
3. **`handover/ai-direct/LATEST.md`** — top "🔴 Stage C VETOED" block + "✅ Phase E SHIPPED" block + "✅ P-M2 SHIPPED FINAL" block + "✅ P-M3 SHIPPED" block + **"✅ P-M4 SHIPPED FINAL 2026-05-09 session #31"** block (canonical current state).
4. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`** §1.C row 4 (P-M5 next atom; Class-3; per-atom §8 NO; n/a was correct) + §1.C row 5 (P-M6 still gated, Class-4 STEP_B).
5. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM4_§8_SIGN_OFF.md`** — verbatim sign-off precedent (`签字，同意后续执行` multi-clause Class-4 §8).
6. **`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`** §7.6 P-M5 verbatim (lines 823-861; CPMM Swap YES/NO only — share swap before Coin router; 6 mandated tests; integer math only no f64).
7. **`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`** — gate-level CI view.
8. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + Active state lineage (P-M4 SHIPPED FINAL row at top; P-M3 row preserved; P-M2 row preserved).

## §1 — Likely paths the next session takes

### Path A: Phase F.4 P-M5 CpmmSwap re-apply (most likely; per remediation directive §1.C row 4)

**Class-3 atom** — re-apply per remediation directive §1.C row 4 verbatim ("P-M5 CpmmSwap (re-apply); Class 3; n/a (was correct); per-atom §8 NO"). NO architect §8 required. NO PRE-§8 dual audit dispatch (Class-3 framing per `feedback_dual_audit` hybrid risk model). Self-audit + workspace tests + gate runner suffice.

#### Architect §7.6 verbatim spec (`MANUAL_en.md` lines 823-861)

**Pure share swap** — before Coin router. NO Coin movement; only YES/NO claims swap within pool reserves.

**Buy YES with NO** (signed agent submits dN > 0 NO shares; receives outY YES shares):

```
input: dN > 0
outY = floor(dN * poolY / (poolN + dN))
poolN1 = poolN + dN
poolY1 = poolY - outY
```

**Symmetric Buy NO with YES** (signed agent submits dY > 0 YES shares; receives outN NO shares):

```
outN = floor(dY * poolN / (poolY + dY))
poolY1 = poolY + dY
poolN1 = poolN - outN
```

**Constant-product invariant** (architect explicit):

```
poolY1 * poolN1 >= poolY * poolN
```

(`>=` not `==` because floor rounding leaves dust in pool — this is intentional per architect)

**6 mandated test names**:

```
swap_no_for_yes_constant_product_non_decreasing
swap_yes_for_no_constant_product_non_decreasing
swap_fails_zero_input
swap_fails_insufficient_pool_output
swap_respects_min_out_slippage
swap_uses_integer_math_no_f64
```

#### Implementation surfaces (analogous to P-M4 surface set)

NEW agent-signed tx — `CpmmSwapTx` (likely 8 fields: tx_id / parent_state_root / event_id / trader / direction (`SwapDirection::BuyYesWithNo` | `BuyNoWithYes`) / amount_in / min_out / signature). NEW signing payload `CpmmSwapSigningPayload`. NEW `TransitionError` variants (likely `SwapZeroInput` / `SwapInsufficientPoolOutput` / `SwapSlippageExceeded` / `PoolNotActive`).

**Sequencer admission arm** preconditions (mirror P-M4 5-stage pattern):

1. `parent_state_root == q.state_root_t` else `StaleParent`.
2. `amount_in.units > 0` else `SwapZeroInput`.
3. Pool exists at `event_id` AND `pool.status == Active` else `PoolNotActive`.
4. Trader has `amount_in` of input side in `conditional_share_balances_t` else `InsufficientSharesForSwap`.
5. Compute `out` = `floor(amount_in * pool_other / (pool_input + amount_in))`; if `out < min_out` → `SwapSlippageExceeded`.
6. Compute `pool_input1 = pool_input + amount_in` and `pool_other1 = pool_other - out`; if `pool_other1 == 0` → `SwapInsufficientPoolOutput` (pool fully drained on the output side; reject as policy).

Atomic mutations (analogous to P-M4 step 6):

- 6a: `conditional_share_balances_t[(trader, event_id)].input_side -= amount_in`.
- 6b: `cpmm_pools_t[event_id].pool_input += amount_in.units; pool_other -= out.units`.
- 6c: `conditional_share_balances_t[(trader, event_id)].other_side += out.units`.

Monetary invariants:

- `assert_no_post_init_mint(tx, q)` — TypedTx::CpmmSwap added to allow-list (no Coin minted; pure share rotation).
- `assert_total_ctf_conserved(empty exempt-list)` — passes because pool reserves + conditional_share_balances are NOT Coin; YES + NO totals across pool + traders preserved.
- `assert_complete_set_balanced` — extended in P-M4 to count pool reserves; should still PASS because pool reserves total YES + total NO each preserve symmetric balance against collateral (swap preserves pool inventory total within each side; provider's transferred-out side moves to trader; symmetric reduction holds).

**State_root advance**: `cpmm_swap_accept_state_root(prev, tx) = sha256(b"turingosv4.cpmm_swap.accept.v1" || prev || canonical_encode(tx))` mirror pattern.

#### Class-3 framing details (NO STEP_B branch needed)

Class-3 atoms are economic mutators that don't introduce new Class-4 surfaces (sequencer admission / typed-tx schema / canonical signing payload / RootBox modifications happen, but they're symmetric extensions of P-M4-shipped patterns). Per `feedback_step_b_protocol`, Class-3 modifications to STEP_B-listed files (`typed_tx.rs` / `sequencer.rs` / etc.) **DO require STEP_B parallel-branch protocol** — the file membership in STEP_B is what matters, not the atom's overall risk class.

Wait — re-read `feedback_step_b_protocol`: STEP_B is for restricted-file changes regardless of atom risk class. P-M5 touches `src/state/typed_tx.rs` + `src/state/sequencer.rs` + likely `src/bottom_white/ledger/transition_ledger.rs` (TxKind=16) — all STEP_B-listed. So:

- **STEP_B branch**: `git checkout -b feat/p-m5-rebuild` off `origin/main` HEAD `55a4c38`.
- Per-atom commit on branch.
- `--no-ff` merge to main after validation green.
- Push origin/main with user authorization.

Class-3 framing means: NO PRE-§8 dual audit; NO architect §8 packet; just self-audit + cargo green + gates green. STEP_B is just a code-handling protocol; it doesn't add audit obligations.

#### Pre-flight (mandatory before any code)

1. **`/runner-preflight`** — NOT applicable (no evidence-dir mutation; P-M5 is in-repo source/test STEP_B work). Skip with explicit acknowledgment.
2. **`/constitution-landing-check`** — should return PROCEED (0 AMBER; matrix unchanged).
3. **Verify HEAD** `55a4c38` matches `origin/main`; gates 207/0/1 baseline; workspace 1340/0/151 baseline.
4. **R-022 pre-empt**: pre-add `/// TRACE_MATRIX FC1-Append Stage C P-M5 / Phase F.4 (architect manual §7.6 verbatim)` doc-block to ALL new pub items in `CpmmSwapTx` + `CpmmSwapSigningPayload` + `SwapDirection` enum (and any pub fn / pub const fn methods) BEFORE first commit. Per session #31 lesson 4 from `/harness-reflect`: pub-struct's impl methods need backlinks too, not just struct items.

#### Step-by-step procedure (Class-3 STEP_B atom)

```
1. Branch: git checkout -b feat/p-m5-rebuild
2. Read architect §7.6 verbatim (MANUAL_en.md lines 823-861).
3. Implement CpmmSwapTx + CpmmSwapSigningPayload + SwapDirection enum + DOMAIN_AGENT_CPMM_SWAP + TypedTx variant + TxKind=16 + dispatch + HasSubmitter (trader as signer) + 4 new TransitionError variants + Display.
4. Implement Sequencer admission arm with 6 preconditions + atomic mutations (debit input side / update pool reserves / credit output side) + 3 monetary invariants + state_root advance via cpmm_swap_accept_state_root.
5. 4 fan-out match arms (system_message_for_verification / system_signature_of / system_epoch_of / submit_agent_tx allow-list) + agent-sig manifest verify arm (trader as signer).
6. Replay-time Gate 4 verify arm (mirror P-M4).
7. Add tests/constitution_cpmm_swap.rs (NEW) — 6 architect §7.6 verbatim tests through live Sequencer::submit_agent_tx.
8. (NO E.1 BINDINGS extension required for Class-3 — architect §7.6 specifies behavior + tests, no STATE struct verbatim spec; tx + signing payload are implementation-defined like P-M4. Optional: add CpmmSwapTx + CpmmSwapSigningPayload bindings if you want the same drift-detection coverage P-M4 enjoys.)
9. Register constitution_cpmm_swap gate in scripts/run_constitution_gates.sh.
10. Trust Root rehash (5-7 STEP_B files: typed_tx + sequencer + transition_ledger + verify + run_summary + audit_assertions + monetary_invariant if allow-list extended).
11. Validate:
   - cargo check --workspace clean
   - cargo test --workspace --no-fail-fast: 1340 → 1346+ (6 new tests)
   - bash scripts/run_constitution_gates.sh: 207 → 208+ (1 new gate)
   - cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo: PASS
12. Commit on feat/p-m5-rebuild branch.
13. --no-ff merge to local main.
14. Push to origin/main (request user authorization first per ship-discipline).
15. Update LATEST.md + MEMORY.md with P-M5 SHIPPED status.
16. Move to F.5 P-M6 Mint-and-Swap Router rebuild (Class-4 STEP_B; per-atom §8 + PRE-§8 dual audit).
```

#### Estimated wall-clock: ~1 day (Class-3, no audit, no architect §8)

Pattern from P-M4 (Class-4): 2 hours implementation + 30 min validation + 10 min audit dispatch + 10 min audit completion + sign-off = ~3 hours pure work. P-M5 minus dual-audit minus §8-packet = ~2 hours pure work + integration.

### Path B: Phase F.5 P-M6 Mint-and-Swap Router rebuild (alternative; needs F.4 first)

**Class-4 STEP_B atom** — full rebuild post-VETO (Defects 1 + 2 from session #27 batch). Per-atom §8 required. PRE-§8 dual audit dispatch mandatory (timing rule exercised twice now: P-M2 R1 CHALLENGE→R2 PASS; P-M4 R1 PASS/PASS first try).

**NOT recommended ahead of P-M5** — F.5 is gated on F.4 per remediation directive §1.B sequence. Skipping ahead violates `feedback_no_batch_class4_signoff` per-atom cadence.

### Path C: timestamp_logical drift escalation (alternative)

Same as post-P-M3 boot prompt §B. Forward-bound to architect §10 reclassification path; bounded to 3 typed-tx variants (`MarketSeedTx` / `CompleteSetMintTx` / `CompleteSetRedeemTx`); does NOT block Stage C Polymarket sequence per remediation directive scope.

**NOT recommended** unless architect-clarification surfaces a load-bearing issue.

### Path D: Forward-bound parallel work (alternative; user must explicitly authorize)

Per CLAUDE.md §19 (no manipulation by sequencing): Stage C P-M2..P-M9 sequence is the load-bearing critical path. Deferring F.4 would be the exact pattern §19 prohibits. NOT recommended.

## §2 — Pre-action gate (mandatory)

Per `MEMORY.md` "MUST CHECK BEFORE":

- **Before any new TB charter / G1 audit / pick-next-atom**: `/constitution-landing-check`.
- **Before any `bash run_*.sh` runner script**: `/runner-preflight`.
- **Before writing new `feedback_*.md`**: ask "what mechanism enforces this?" — per `feedback_norm_needs_mechanism`.
- **On any FC1/FC2/FC3 problem**: trace BEFORE designing fix.
- **On any new TB charter**: declare `phase_id` + `roadmap_exit_criteria_addressed` + `kill_criteria_tested`.
- **After TB SHIPPED FINAL or audit rounds > 3**: `/harness-reflect`. (P-M5 is Class-3 ship not "FINAL" classification per remediation directive; reflect after F.5 P-M6 ships, since P-M6 is the next Class-4 SHIPPED FINAL boundary.)

P-M5 is forward execution against the remediation directive §1.C row 4; constitution-landing-check at session #31 returned PROCEED (0 AMBER). The gate must fire fresh at next session start.

## §3 — Phase F architecture rules (per remediation directive §1.B + §9)

### Per-atom Class-4 §8 cadence (NO batching) — applies to F.5 only (F.4 is Class-3)

Per `feedback_no_batch_class4_signoff`. Atoms ship sequentially:

```
F.1 P-M2 ✅ → F.2 P-M3 ✅ → F.3 P-M4 ✅ → F.4 P-M5 (Class-3, no §8) → F.5 P-M6 → §8 → F.6/F.7/F.8 P-M7/M8/M9 (non-Class-4) → F.9 Stage C overall §8
```

Class-3 atoms (F.4 P-M5) bypass §8 but still get self-audit + workspace tests at ship gate per `feedback_dual_audit` Class-3 framing.

### Dual audit PRE-§8 timing (Class-4 only)

For F.5 P-M6 + F.9 Stage C overall: dispatch Codex G2 + Gemini at PACKET DRAFT time, not after architect §8 request. **Both exercises succeeded** (P-M2 R1 CHALLENGE→R2 PASS; P-M4 R1 PASS/PASS first try). Pattern stable.

If `Skill: codex:rescue` is rejected ≥ 2× by user, fall back to direct Bash invocation per `feedback_codex_bash_exec_direct_dispatch`:

```bash
timeout 1500 codex exec \
  --dangerously-bypass-approvals-and-sandbox \
  -C /home/zephryj/projects/turingosv4 \
  -o /tmp/<job>_last.txt \
  < /tmp/<job>_prompt.md \
  > /tmp/<job>_audit.out 2>&1
```

### F-DEFERRAL closure (Phase F.5 only — F.4 N/A)

Phase F.5 P-M6 rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with sibling entry for `BuyWithCoinRouterSigningPayload` (or whatever architect §7.7 verbatim names; remediation directive §9 reads `BuyWithCoinRouterSigningPayload` — confirm against architect manual at packet-draft time).

F-DEFERRAL-1: Phase F.5 P-M6 rebuild MUST extend `tests/constitution_economy_strict_equality.rs` `CONSERVATION_INVARIANT_FILES` to include any new helper-alias file containing CTF conservation logic, OR explicitly attest `# F-DEFERRAL-1: no helper-alias introduced` in the §8 packet.

### P-M6 rebuild patches (mandatory at F.5)

Phase F.5 P-M6 rebuild MUST patch:
- **Defect 1 fix**: strict `sum_yes == collateral && sum_no == collateral` (no `min()`).
- **Defect 2 fix**: `router_atomic_rollback_on_failure` test must inject mid-mutation failure via cfg(test) hook + assert full state restoration.

## §4 — Forward queue (post-§31 close; canonical)

| Item | Class | Blocker / status |
|---|---|---|
| **Phase F.4 P-M5 (CpmmSwap re-apply)** | 3 | Charter-eligible NOW; no §8; STEP_B for `typed_tx.rs` / `sequencer.rs` files; ~1 day |
| Phase F.5 P-M6 (Mint-and-Swap Router rebuild + 2 patches) | 4 STEP_B | Gated on F.4; per-atom §8 + PRE-§8 dual audit |
| Phase F.6/F.7/F.8 P-M7/M8/M9 re-apply | 1-3 | Gated on F.5 §8 |
| Phase F.9 Stage C overall §8 | 4 ship | Gated on all atoms green |
| F-DEFERRAL-1 closure | 1 | Closes at F.5 |
| F-DEFERRAL-2 closure (P-M6 sibling binding) | 1 | At F.5 (P-M2 + P-M4 already closed) |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward post-Polymarket |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB |
| K.1-6 Stage D real-world readiness | architect | Decoupled |

## §5 — Memory entries to verify at next-session start

Verify these are present in `MEMORY.md` (added 2026-05-09 session #31):

- **Stage C P-M4 SHIPPED FINAL row** (latest §8 row): 2026-05-09 session #31; HEAD `55a4c38` post-everything; Class-4 STEP_B; per-atom §8 with verbatim "签字，同意后续执行" multi-clause; PRE-§8 dual audit R1 PASS/PASS first-try; gates 203→207, workspace 1336→1340; EconomicState 13→15; F-DEFERRAL-2 closed for P-M4; F-DEFERRAL-1 N/A (no helper-alias).
- **Stage C P-M2 + P-M3 + P-M4 lineage row**: removed "Phase F.3 P-M4 still gated" wording (now SHIPPED FINAL); F.5 P-M6 gated on F.4 P-M5 (eligible NOW) wording added.
- **Stage C VETO closure row**: P-M2 ✅ closed session #29; P-M4 ✅ closed session #31; P-M6 remains pending Phase F.5.
- **`feedback_codex_bash_exec_direct_dispatch.md`** memory file present at `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/`.
- **`feedback_market_quarantine_token_exemption.md`** memory file present at same path.
- **MEMORY.md index** updated with both new feedback entries (placed near `feedback_dual_audit` and `feedback_no_workarounds_strict_constitution` semantically).

If any are stale, restore from session #31 work.

## §6 — Key references (canonical sources)

| Reference | Purpose |
|-----------|---------|
| P-M4 atomic commit | `023fe32` (P-M4 SHIPPED — CpmmPool Class-4 STEP_B Phase F.3) |
| P-M4 §8 sign-off commit | `d9d2b0b` (sign-off doc + audit packet + dual-audit verdicts) |
| P-M4 merge commit | `008d9a3` (Merge feat/p-m4-rebuild → main) |
| LATEST.md update commit | `9f4ea3b` (P-M4 SHIPPED FINAL block) |
| R-020 retire commit | `55a4c38` (R-020 retire + 2 NEW feedback memories) |
| Remediation directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 4 (P-M5 next; Class 3) |
| Architect manual §7.6 P-M5 | `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` lines 823-861 |
| P-M4 reference (mirror pattern) | `tests/constitution_cpmm_pool.rs` + `tests/constitution_architect_verbatim_struct_binding.rs` E.1 binding (both P-M4 entries Landed; CpmmPoolSigningPayload sibling LANDED) |
| P-M2 reference (Class-4 minimal pattern) | `tests/constitution_completeset_merge.rs` + sequencer admission arm |
| Codex Bash dispatch (if Skill rejected) | `feedback_codex_bash_exec_direct_dispatch.md` |
| Market quarantine token exemption | `feedback_market_quarantine_token_exemption.md` |

---

## USER PROMPT (paste this into next Claude session)

```
P-M4 SHIPPED FINAL in session #31 (2026-05-09) at HEAD `55a4c38`
(pushed to origin/main; lineage `92cfeb6`→`023fe32`(atomic)
→`d9d2b0b`(sign-off)→`008d9a3`(merge)→`9f4ea3b`(LATEST.md)
→`55a4c38`(R-020 retire + 2 new feedbacks)).

Class-4 STEP_B per remediation directive §1.C row 3. Architect §8
verbatim "签字，同意后续执行" (multi-clause; structurally equivalent
to canonical "好，确认可以 ship" / "同意 sign-off"). PRE-§8 dual audit
R1 Codex PASS (8/8 high) + Gemini PASS (8/8 high) first-try; round
cap 2 used 1.

Constitution gates 207/0/1 (was 203 pre-F.3; +4). Workspace 1340/0/151
(was 1336; +4). Trust Root PASS (7 STEP_B + 2 hook/manifest rehashes
for R-020 retire). EconomicState 13→15 (+cpmm_pools_t +lp_share
_balances_t; pool reserves + LP shares NOT Coin per architect §7.5
rules 2 + 3). F-DEFERRAL-2 closed for P-M4 via CpmmPoolSigningPayload
sibling binding Landed.

Post-ship harness-reflect executed: R-020 retired (2 consecutive
0-trigger cycles); 2 new feedback memory files written
(feedback_codex_bash_exec_direct_dispatch.md +
feedback_market_quarantine_token_exemption.md); MEMORY.md index
updated.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09_post_pm4_ship.md
   (this prompt's source; full context + Phase F.4 P-M5 step-by-step
   + R-022 impl-method backlink lesson from session #31)
2. handover/ai-direct/LATEST.md "✅ P-M4 SHIPPED FINAL 2026-05-09" block
3. handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md
   §1.C row 4 (P-M5 Class-3 re-apply; n/a was correct; per-atom §8 NO)
4. handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md
   §7.6 P-M5 (lines 823-861; CPMM Swap YES/NO only — pure share swap
   before Coin router; 6 mandated tests; integer math no f64)

Tell me what you want to do:
(a) Phase F.4 P-M5 CpmmSwap re-apply — Class-3 STEP_B atom (no §8;
    no PRE-§8 dual audit). Architect §7.6 verbatim 6-test battery
    (swap_no_for_yes_constant_product_non_decreasing /
     swap_yes_for_no_constant_product_non_decreasing /
     swap_fails_zero_input / swap_fails_insufficient_pool_output /
     swap_respects_min_out_slippage / swap_uses_integer_math_no_f64).
    Architect formula: outY = floor(dN * poolY / (poolN + dN)) +
    constant-product invariant poolY1 * poolN1 >= poolY * poolN.
    R-022 pre-empt incl. impl methods (lesson from P-M4 session #31).
    ~1 day (Class-3 minus §8 minus dual audit). MOST LIKELY path per
    remediation directive §1.C row 4 + CLAUDE.md §19 critical-path
    discipline.
(b) Phase F.5 P-M6 Mint-and-Swap Router rebuild — NOT recommended
    ahead of P-M5 (gated per directive §1.B sequence; skipping
    violates feedback_no_batch_class4_signoff per-atom cadence).
(c) timestamp_logical drift escalation — file architect §10 reclassi-
    fication directive draft. NOT recommended unless surfacing
    load-bearing issue (bounded to 3 typed-tx variants; doesn't block
    Stage C Polymarket sequence).
(d) Something else — describe it.
```

---

**End of next-session boot prompt (post P-M4 SHIPPED FINAL ship).**
