# Next Session Boot Prompt — 2026-05-09 session #29 close (post P-M2 ship)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session. Everything above it is context for cold-start orientation if you read this file directly.

---

## State at session #29 close (2026-05-09)

- **HEAD on `origin/main`**: `06102ca` (pushed; 9 commits ahead of `ff2d401` pre-Phase-F.1 baseline).
- **Phase F.1 P-M2: SHIPPED FINAL** — first per-atom Class-4 §8 post-Stage-C-VETO.
- **Architect §8 verbatim**: 「好，确认可以 ship」 (canonical multi-clause Class-4 form, 2026-05-09 session #29).
- **Constitution gates**: `198 PASS / 0 FAIL / 1 ignored` (was 193 pre-F.1; +5 from `constitution_completeset_merge`).
- **Workspace tests**: `1331 PASS / 0 FAIL / 151 ignored` (was 1326; +5 architect-mandated verbatim tests).
- **Trust Root**: PASS (6 files rehashed: `typed_tx.rs` / `sequencer.rs` / `transition_ledger.rs` / `monetary_invariant.rs` / `verify.rs` / `run_summary.rs`).
- **E.1 P-M2 binding**: LANDED (strict 6-field `(name, type)` pair-equality enforced; Defect 3 `timestamp_logical` recurrence mechanically prevented).
- **F-DEFERRAL-2** (signing-payload binding): CLOSED for P-M2; remains open for P-M4 + P-M6 future rebuilds.

## Audit chain summary (PRE-§8 dual audit, first exercise of new Class-4 timing rule)

| Round | HEAD | Codex G2 | Gemini | Aggregate | Action |
|-------|------|----------|--------|-----------|--------|
| R1 | `66f4e34` | CHALLENGE (Q2 fixture-forge + Q3 zero-amount drift) | PASS (all 8) | CHALLENGE → FIX-THEN-PROCEED | Remediated `444c470` |
| R2 | `851364a` | **PASS** (Q2 + Q3 closed; Q1/Q4-Q8 carried) | **PASS** (all 8) | **PASS → PROCEED** | Architect §8 ratification |

R1 reports: `handover/audits/CODEX_STAGE_C_PM2_AUDIT_2026-05-09_R1.md` + `handover/audits/GEMINI_STAGE_C_PM2_AUDIT_2026-05-09_R1.md`.
R2 reports: `handover/audits/CODEX_STAGE_C_PM2_AUDIT_2026-05-09_R2.md` + `handover/audits/GEMINI_STAGE_C_PM2_AUDIT_2026-05-09_R2.md`.
Round cap 2 used (within `feedback_elon_mode_policy`); R3 not required.

## Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §6 FC1 invariant; §10 Class-4 authorization; §12 STEP_B; §13 economy laws).
2. **`constitution.md`** — top-level law (architect verbatim spec).
3. **`handover/ai-direct/LATEST.md`** — top "🔴 Stage C VETOED" block + "✅ Phase E SHIPPED" block + **"✅ P-M2 SHIPPED FINAL 2026-05-09 session #29"** block (canonical current state).
4. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_§8_SIGN_OFF.md`** — P-M2 §8 sign-off (verbatim "好，确认可以 ship").
5. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`** §1.C row 2 (P-M3 next atom; Class-3; no §8 needed) + §9 F-DEFERRAL-1/2.
6. **`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`** §7.4 P-M3 verbatim (lines 753-787; MarketSeedTx 6-field spec + 5 mandated tests).
7. **`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`** — gate-level CI view.
8. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + Active state lineage (latest §8 row updated to P-M2 SHIPPED).

## §1 — Likely paths the next session takes

### Path A: Phase F.2 P-M3 MarketSeedTx hardening (most likely; per remediation directive §1.C row 2)

**Class-3 atom** — no per-atom §8 required (CLAUDE.md §9 Class 3 = "auth/money/CAS integrity" with dual audit at ship gate; §8 only mandated for Class 4). Per remediation directive §1.C row 2: "P-M3 MarketSeedTx hardening (re-apply); Class 3; per-atom §8 NO".

#### Architect §7.4 verbatim spec (`MANUAL_en.md` lines 753-787)

**Semantic mandate**: "MarketSeedTx must be collateral-backed" (option A: provider deposits seedC Coin; CompleteSetMint-like operation creates seedC YES + seedC NO; YES/NO shares go to pool inventory; collateral locks seedC).

**Struct (6 fields)**:
```rust
pub struct MarketSeedTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub signature: AgentSignature,
}
```

**5 mandated tests**: `market_seed_debits_provider` + `market_seed_creates_yes_no_inventory` + `market_seed_fails_insufficient_balance` + `market_seed_no_ghost_liquidity` + `market_seed_conserves_total_coin`.

#### Decision point: existing MarketSeedTx is 7 fields (with `timestamp_logical`)

The current `src/state/typed_tx.rs::MarketSeedTx` (line ~1233) is the TB-13-era impl with 7 fields:

```rust
pub struct MarketSeedTx {
    pub tx_id: TxId,
    pub parent_state_root: Hash,
    pub event_id: EventId,
    pub provider: AgentId,
    pub collateral_amount: MicroCoin,
    pub signature: AgentSignature,
    pub timestamp_logical: u64,         // <-- TB-13-era extra; NOT in §7.4 verbatim
}
```

Same shape exists for `CompleteSetMintTx` (7 fields) and `CompleteSetRedeemTx` (8 fields).

E.1 binding gate notes (lines 81-85) deliberately deferred this question:

> §7.4 MarketSeedTx — manual gives a 6-field struct, but the TB-13 era impl carries a `timestamp_logical` field that predates Stage C. The drift is real and worth addressing, but not via Phase E (which would scope-creep into either an architect-manual amendment or a TB-13 era refactor). Phase F.3 [actually F.2 — comment numbering pre-dated remediation directive] (P-M3 re-apply) is the natural decision point.

**Three sub-options for next session to evaluate with user**:

- **Sub-option A1 (strict-spec)**: Treat MarketSeedTx as Class-4 STEP_B and remove `timestamp_logical` to align with §7.4 verbatim 6 fields. This would also touch `CompleteSetMintTx` + `CompleteSetRedeemTx` for consistency. **Requires §8** because schema bump on STEP_B file. Risk: cascade into 3 typed-tx schema changes; bigger blast radius than P-M2; signing-payload bindings need revisit.

- **Sub-option A2 (TB-13 era preserved)**: Keep current 7-field impl as TB-13-ratified state; treat §7.4 verbatim as semantic mandate (collateral-backed) rather than schema mandate. Add the 5 mandated tests if missing; verify "must be collateral-backed" semantics; ship as Class-3 (no §8). **Most likely framing per remediation directive §1.C row 2** ("re-apply" suggests existing impl is correct; "hardening" suggests semantic strengthening, not schema bump).

- **Sub-option A3 (architect-clarification-first)**: Pause P-M3 to ask architect whether `timestamp_logical` on TB-13-era typed-tx variants is intentional pre-Stage-C state or a drift to fix. This blocks P-M3 implementation but resolves the deeper question once.

**Recommended starting framing**: Sub-option A2 (TB-13-era preserved) per remediation directive §1.C row 2 verbatim. If architect-clarification-first is preferred, escalate before any code change.

#### Pre-flight (mandatory before any code)

1. **Invoke `/runner-preflight`** (NOT applicable — P-M3 is in-repo source/test work, no evidence dir mutation; skip with explicit acknowledgment).
2. **Invoke `/constitution-landing-check`** — surface AMBER rows; should return PROCEED since matrix is 0 AMBER.
3. **Verify HEAD** `06102ca` matches `origin/main`; gates 198/0/1 baseline; workspace 1331/0/151 baseline.
4. **Inventory existing MarketSeed tests** in `tests/tb_13_complete_set.rs` + `tests/tb_18d_*.rs` against the 5 architect §7.4 mandated names. Likely already covers SG-13.3 (`market_seed_fails_if_provider_lacks_balance` ≈ `market_seed_fails_insufficient_balance`) + SG-13.4 (`market_seed_cannot_create_liquidity_without_collateral` ≈ `market_seed_no_ghost_liquidity`); identify gaps.

#### Step-by-step procedure (assuming Sub-option A2 framing)

```
1. Branch: `git checkout -b feat/p-m3-rebuild`

2. Inventory existing MarketSeed coverage; map architect §7.4 mandated tests
   to existing SG-13.* tests; identify gaps.

3. Add missing gates (likely 1-3 new tests in
   tests/constitution_market_seed_hardening.rs NEW gate file):
   - market_seed_debits_provider (verify balances_t mutation)
   - market_seed_creates_yes_no_inventory (verify
     conditional_share_balances_t both sides credited)
   - market_seed_conserves_total_coin (assert_total_ctf_conserved across)
   Map existing TB-13 tests for the other 2:
   - market_seed_fails_insufficient_balance ≈ SG-13.3 (existing)
   - market_seed_no_ghost_liquidity ≈ SG-13.4 (existing)

4. Register new gate in scripts/run_constitution_gates.sh.

5. NO Trust Root rehash needed (Class-3 = no STEP_B file edit).

6. Verify:
   - cargo check --workspace clean
   - cargo test --workspace --no-fail-fast: 1331 → 1334+ (3 new gates)
   - bash scripts/run_constitution_gates.sh: 198 → 201+
   - existing TB-13 tests preserved

7. Commit on branch `feat/p-m3-rebuild` (Class-3 atomic, no §8 packet needed).

8. Self-merge to main with --no-ff:
   `git checkout main && git merge --no-ff feat/p-m3-rebuild`

9. Push to origin/main (request user authorization first per ship-discipline).

10. Update LATEST.md (P-M3 SHIPPED row appended to "✅ P-M2 SHIPPED FINAL"
    block as a sub-status; or new "✅ P-M3 SHIPPED" block).

11. Update MEMORY.md Active state.

12. Move to F.3 P-M4 CpmmPool rebuild (Class-4 STEP_B; per-atom §8 cycle
    same as P-M2 — rename `event_id_kind` → `event_id` per architect §7.5
    verbatim).
```

#### Estimated wall-clock: ~0.5-1 day work (Sub-option A2; Class-3 = no audit cycle)

If Sub-option A1 (strict-spec Class-4 STEP_B): ~2-3 days + dual-audit cycle + architect §8.

### Path B: Forward-bound work (alternative; user must explicitly authorize)

If user wants to defer Phase F.2 start, candidates from "🚧 Open after Polymarket" block in `LATEST.md`:

| Item | Class | ETA | Reason non-blocking |
|---|---|---|---|
| C.5 PromptCapsule evaluator wire-up | 3 | ~1-2 days | Affects LLM-Lean attempt path; Polymarket sequencer doesn't read PromptCapsule |
| B.4 CAS Merkle redesign (Stage A3.6 enhancement TB) | 3-4 | ~3-5 days | Replay reconstructs via cas/.git/objects + sidecar; market L4 anchor unaffected |
| J.5 4 replay sampling tests | 1 | ~1 day | Gate-level only; gated on M2 evidence |
| K.1-6 Stage D real-world readiness | architect | architect-side | Decoupled from Polymarket per manifest §11 |

**DO NOT START** Path B unless user explicitly authorizes. Per CLAUDE.md §19 (no manipulation by sequencing): Stage C P-M2..P-M9 sequence is the load-bearing critical path; deferring F.2 to do C.5/B.4/J.5 would be the exact pattern §19 prohibits.

### Path C: Architect §10 reclassification of TB-13-era timestamp_logical

If architect-clarification-first (Sub-option A3) is the chosen route, file a directive draft requesting §10 reclassification of CompleteSetMintTx / CompleteSetRedeemTx / MarketSeedTx wire schema vs §7.4 verbatim. Wait for verbatim ratification before any code change.

## §2 — Pre-action gate (mandatory)

Per `MEMORY.md` "MUST CHECK BEFORE":

- **Before any new TB charter / G1 audit / pick-next-atom**: invoke `/constitution-landing-check`.
- **Before any `bash run_*.sh` runner script**: invoke `/runner-preflight`.
- **Before writing new `feedback_*.md`**: ask "what mechanism enforces this?" — per `feedback_norm_needs_mechanism`.
- **On any FC1/FC2/FC3 problem**: trace BEFORE designing fix.
- **On any new TB charter**: declare `phase_id` + `roadmap_exit_criteria_addressed` + `kill_criteria_tested`.
- **After TB SHIPPED FINAL or audit rounds > 3**: invoke `/harness-reflect` (P-M2 just SHIPPED FINAL; consider firing this skill at session start to extract Phase F.1 lessons).

Phase F.2 is forward execution against the remediation directive §1.C row 2; the constitution-landing-check at session #29 returned PROCEED (0 AMBER). The gate must fire fresh at next session start if charter work is opened.

## §3 — Phase F architecture rules (per remediation directive §1.B + §9)

### Per-atom Class-4 §8 cadence (NO batching) — applies to F.3 + F.5

Per `feedback_no_batch_class4_signoff`. Atoms ship sequentially:

```
F.1 P-M2 ✅ → F.2 P-M3 (Class-3, no §8) → F.3 P-M4 → §8 → F.4 P-M5 (Class-3, no §8) → F.5 P-M6 → §8 → F.6/F.7/F.8 P-M7/M8/M9 (non-Class-4) → F.9 Stage C overall §8
```

Class-3 atoms (F.2 P-M3 + F.4 P-M5) bypass §8 but still get dual-audit at ship gate per `feedback_dual_audit` Class-3 framing (or self-audit + workspace tests if no auth/money/CAS surface touched — gauge by what P-M3 actually touches).

### Dual audit PRE-§8 timing (Class-4 only)

For F.3 P-M4 + F.5 P-M6 + F.9 Stage C overall: dispatch Codex G2 + Gemini at PACKET DRAFT time, not after architect §8 request. P-M2 was the first exercise of this rule (worked: R1 CHALLENGE caught + remediated in working tree at zero rollback cost).

### F-DEFERRAL-1 closure (Phase F.5 P-M6 only)

Phase F.5 P-M6 rebuild MUST extend `tests/constitution_economy_strict_equality.rs` `CONSERVATION_INVARIANT_FILES` to include any new helper-alias file containing CTF conservation logic, OR explicitly attest `# F-DEFERRAL-1: no helper-alias introduced`. P-M2 closed this trivially (no alias).

### F-DEFERRAL-2 closure (Phase F.3 + F.5)

Phase F.{3,5} rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with a sibling entry for `*SigningPayload` (one per Class-4 atom). Audit witness: `grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs` shows ≥3 entries post-Phase-F (P-M2 already added; P-M4 + P-M6 to follow).

### P-M6 rebuild patches (mandatory at F.5)

Phase F.5 P-M6 rebuild MUST also patch:

- **Defect 1 fix**: when CPMM pool reserves enter `assert_complete_set_balanced` sums, the symmetric/asymmetric branch split (Phase E.3 source refactor) handles it; F.5 verifies Branch B's `min()` doesn't admit pool-induced asymmetry. May require explicit resolution-state tracking (out-of-Phase-E scope; F.5 design call).
- **Defect 2 fix**: `router_atomic_rollback_on_failure` test must inject mid-mutation failure (e.g. via `inject_failure_after_step()` helper or `set_var("ROUTER_FAIL_AT_STEP", N)` env-var) and assert full state restoration. Sequencer cfg(test) injection point is part of F.5 STEP_B per plan §C.E.2.

## §4 — Forward queue (post-§29 close; canonical)

| Item | Class | Blocker / status |
|---|---|---|
| **Phase F.2 P-M3 (MarketSeedTx hardening)** | 3 | Charter-eligible NOW; no §8 needed |
| Phase F.3 P-M4 rebuild (CpmmPool) | 4 STEP_B | Gated on F.2; per-atom §8 |
| Phase F.4 P-M5 re-apply | 3 | Gated on F.3 §8 |
| Phase F.5 P-M6 rebuild + 2 patches | 4 STEP_B | Gated on F.4; per-atom §8 |
| Phase F.6/F.7/F.8 P-M7/M8/M9 re-apply | 1-3 | Gated on F.5 §8 |
| Phase F.9 Stage C overall §8 | 4 ship | Gated on all atoms green |
| F-DEFERRAL-1 closure (E.3 helper-alias scope) | 1 | Closes at F.5 |
| F-DEFERRAL-2 closure (E.1 signing-payload binding) | 1 | Closes per atom at F.3 + F.5 (F.1 already closed) |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward post-Polymarket |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB |
| K.1-6 Stage D real-world readiness | architect | Decoupled |

## §5 — Memory entries to update at next-session start (verify)

Verify these are present in `MEMORY.md` (added 2026-05-09 session #29):

- **Latest architect §8 row**: P-M2 SHIPPED FINAL 2026-05-09 (verbatim "好，确认可以 ship"); previous Stage A3 row demoted to "Prior architect §8".
- **Stage C P-M2 rebuild COMPLETE row**: Phase F.1 SHIPPED FINAL session #29; F.2 P-M3 (Class-3) eligible NOW; F.3 P-M4 + F.5 P-M6 still gated on dual-audit-PRE-§8 cycle.
- **Stage C VETO status updated**: VETO target P-M2 closed; P-M4 + P-M6 remain pending Phase F.3 / F.5.

If any are stale, restore from session #29 work.

## §6 — Key references (canonical sources)

| Reference | Purpose |
|-----------|---------|
| §8 sign-off directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_§8_SIGN_OFF.md` (verbatim "好，确认可以 ship") |
| §8 candidate packet | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_§8_PACKET.md` (charter ship gates + audit chain) |
| Remediation directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 2 (P-M3 next) + §9 deferrals |
| Architect manual | `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.4 P-M3 verbatim |
| Audit reports R1 | `handover/audits/CODEX_STAGE_C_PM2_AUDIT_2026-05-09_R1.md` + `handover/audits/GEMINI_STAGE_C_PM2_AUDIT_2026-05-09_R1.md` |
| Audit reports R2 | `handover/audits/CODEX_STAGE_C_PM2_AUDIT_2026-05-09_R2.md` + `handover/audits/GEMINI_STAGE_C_PM2_AUDIT_2026-05-09_R2.md` |
| Gemini audit script | `handover/audits/run_gemini_stage_c_pm2_audit_2026-05-09.py` (template for P-M3+ audits if Class-3 also audits) |
| Gate test files | `tests/constitution_completeset_merge.rs` (P-M2 5 architect tests) + `tests/constitution_architect_verbatim_struct_binding.rs` (P-M2 + P-M2-signing both Landed) |

---

## USER PROMPT (paste this into next Claude session)

```
P-M2 SHIPPED FINAL in session #29 (2026-05-09) at HEAD `06102ca` (pushed
to origin/main). Architect §8 verbatim "好，确认可以 ship". Constitution
gates 198/0/1 (was 193 pre-F.1; +5). Workspace 1331/0/151. Trust Root
PASS. E.1 P-M2 binding LANDED + F-DEFERRAL-2 closed for P-M2.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09_post_pm2_ship.md
   (this prompt's source; full context + Phase F.2 P-M3 step-by-step
   + timestamp_logical drift decision point)
2. handover/ai-direct/LATEST.md "✅ P-M2 SHIPPED FINAL 2026-05-09" block
3. handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_§8_SIGN_OFF.md
4. handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md
   §1.C row 2 (P-M3 Class-3 — no §8 needed)
5. handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md
   §7.4 P-M3 (lines 753-787; MarketSeedTx 6-field verbatim + 5 tests)

Tell me what you want to do:
(a) Phase F.2 P-M3 MarketSeedTx hardening — Class-3 atom (no §8 needed).
    Inventory existing TB-13 SG-13.3/4 coverage against architect §7.4
    5-mandated-tests; add missing gates; ship via dual-audit-at-ship-gate
    (or self-audit + workspace tests if no auth/money/CAS surface). ~0.5-1
    day. Sub-option A2 (TB-13 era preserved) recommended starting framing
    per remediation directive §1.C row 2 verbatim.
(b) Phase F.2 P-M3 with timestamp_logical strict-spec alignment — Class-4
    STEP_B (would touch typed_tx.rs schema for MarketSeedTx + likely
    CompleteSetMintTx + CompleteSetRedeemTx for consistency). Requires
    per-atom §8 cycle. ~2-3 days + dual-audit + architect §8. Sub-option
    A1 — heavier but resolves TB-13-era drift fully.
(c) Pause to clarify timestamp_logical question with architect via §10
    reclassification directive. Sub-option A3 — escalate before any code.
(d) Forward-bound parallel work — defer Phase F.2 (NOT recommended per
    CLAUDE.md §19 no-manipulation-by-sequencing; Polymarket atom sequence
    is critical path). C.5 PromptCapsule wire-up / B.4 CAS Merkle / J.5
    replay sampling.
(e) /harness-reflect — fire post-SHIPPED-FINAL skill to extract Phase F.1
    lessons before next atom. Recommended given P-M2 was the FIRST per-atom
    Class-4 cycle post-VETO and the FIRST exercise of PRE-§8 dual-audit
    timing rule.
(f) Something else — describe it.
```

---

**End of next-session boot prompt (post P-M2 ship).**
