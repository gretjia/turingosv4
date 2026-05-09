# Next Session Boot Prompt — 2026-05-09 session #30 close (post P-M3 ship)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session. Everything above it is context for cold-start orientation if you read this file directly.

---

## State at session #30 close (2026-05-09)

- **HEAD on `origin/main`**: `73b42d7` (pushed; merge `feat/p-m3-rebuild` → main; 2 commits on top of `0db1ec2` post-P-M2 boot prompt).
- **Phase F.2 P-M3: SHIPPED** — Class-3 re-apply per remediation directive §1.C row 2 verbatim "n/a (was correct); per-atom §8 NO".
- **No architect §8 required for P-M3** (Class-3 framing).
- **Constitution gates**: `203 PASS / 0 FAIL / 1 ignored` (was 198 pre-F.2; +5 from `constitution_market_seed_hardening`).
- **Workspace tests**: `1336 PASS / 0 FAIL / 151 ignored` (was 1331; +5 architect-mandated verbatim tests).
- **Trust Root**: PASS (unchanged — no STEP_B src edits in P-M3).
- **`timestamp_logical` drift**: deliberately deferred (Sub-option A2 framing; would cascade into CompleteSetMintTx + CompleteSetRedeemTx if reopened).

## What landed in P-M3

| Surface | Change |
|---------|--------|
| `tests/constitution_market_seed_hardening.rs` (NEW, 320 lines) | 5 architect §7.4 verbatim test names — all live through `Sequencer::submit_agent_tx`: `market_seed_debits_provider` / `market_seed_creates_yes_no_inventory` / `market_seed_fails_insufficient_balance` / `market_seed_no_ghost_liquidity` / `market_seed_conserves_total_coin` |
| `scripts/run_constitution_gates.sh` | Registered `constitution_market_seed_hardening` gate (11 lines) |

**No `src/` changes.** TB-13-era 7-field `MarketSeedTx` impl (`src/state/typed_tx.rs::MarketSeedTx` line 1234) preserved as ratified state. Pre-P-M4, inventory is held on provider's `conditional_share_balances_t`; post-P-M4 it will shift to `CpmmPool` reserves. Both cases satisfy the location-agnostic `assert_complete_set_balanced` invariant (1 collateral = 1 YES + 1 NO).

## Pre-flight gates fired this session

- `/harness-reflect` (mandatory post-P-M2-SHIPPED-FINAL gate). Verdict: harness healthy. Top recommendations: (1) apply E.1 binding pattern to P-M4 BINDINGS extension; (2) pre-add R-022 doc-block backlinks for P-M4 CpmmPool pub items BEFORE first commit attempt; (3) R-020 has 0 triggers ever — defer wire-up or retire decision.
- `/constitution-landing-check`. Verdict: PROCEED (matrix is 0 AMBER / 0 RED / 96 GREEN / 0 N/A per session #24 strict closure; the 41 grep hits were all "was 🟡 AMBER" historical annotations in GREEN cells + legend line). Meta-finding logged: skill's Stage 1 awk regex over-matches; tighten to `/\| 🟡 AMBER/` if patched later.

## Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §10 Class-4 authorization; §12 STEP_B; §13 economy laws).
2. **`constitution.md`** — top-level law (architect verbatim spec).
3. **`handover/ai-direct/LATEST.md`** — top "🔴 Stage C VETOED" block + "✅ Phase E SHIPPED" block + "✅ P-M2 SHIPPED FINAL" block + **"✅ P-M3 SHIPPED 2026-05-09 session #30"** block (canonical current state).
4. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`** §1.C row 3 (P-M4 next atom; Class-4 STEP_B; per-atom §8) + §9 F-DEFERRAL-2 (extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS for `CpmmPoolTx` + `CpmmPoolSigningPayload`).
5. **`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`** §7.5 P-M4 verbatim (lines 789-821; CpmmPool 5-field spec — `event_id` not `event_id_kind` — + 4 mandated tests).
6. **`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`** — gate-level CI view.
7. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + Active state lineage (P-M3 SHIPPED row appended at bottom of post-P-M2 row).

## §1 — Likely paths the next session takes

### Path A: Phase F.3 P-M4 CpmmPool rebuild (most likely; per remediation directive §1.C row 3)

**Class-4 STEP_B atom** — full rebuild post-VETO (Defect 4: `event_id_kind` → `event_id` per architect §7.5 verbatim). Per-atom §8 required. PRE-§8 dual-audit dispatch mandatory (timing rule first exercised on P-M2; worked).

#### Architect §7.5 verbatim spec (`MANUAL_en.md` lines 789-821)

**Struct (5 fields)**:
```rust
pub struct CpmmPool {
    pub event_id: EventId,        // <-- NOT event_id_kind (Defect 4 from session #27 batch)
    pub pool_yes: ShareAmount,
    pub pool_no: ShareAmount,
    pub lp_total_shares: LpShareAmount,
    pub status: PoolStatus,
}
```

**Rules**:
- pool_yes and pool_no are share balances controlled by pool
- pool reserves are not Coin
- lp shares are not Coin
- k = pool_yes * pool_no

**4 mandated tests**: `pool_created_from_seed_inventory` / `pool_reserves_not_counted_as_coin` / `lp_shares_not_counted_as_coin` / `pool_cannot_exist_without_collateralized_shares`.

#### F-DEFERRAL-2 closure for P-M4 (per remediation directive §9)

P-M4 rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with sibling entry for `CpmmPoolSigningPayload` (one per Class-4 atom). Currently P-M2 binding is Landed; P-M4 + P-M4-signing bindings need to flip from `NotYetLanded → Landed` in the same commit that lands the struct (E.1 binding gate).

#### Pre-flight (mandatory before any code)

1. **`/runner-preflight`** — NOT applicable (no evidence-dir mutation; P-M4 is in-repo source/test STEP_B work). Skip with explicit acknowledgment.
2. **`/constitution-landing-check`** — should return PROCEED (0 AMBER).
3. **Verify HEAD** `73b42d7` matches `origin/main`; gates 203/0/1 baseline; workspace 1336/0/151 baseline.
4. **R-022 pre-empt**: pre-add `/// TRACE_MATRIX FC?-N? Stage C P-M4 / Phase F.3 (architect manual §7.5 verbatim)` doc-block to ALL new pub items in `CpmmPool` + `CpmmPoolSigningPayload` BEFORE first commit. Per `/harness-reflect` lesson 3 from session #30: skip ~1 round of R-022-BLOCK noise.
5. **STEP_B branch**: `git checkout -b feat/p-m4-rebuild`. P-M4 touches `src/state/typed_tx.rs` + `src/state/sequencer.rs` + likely `src/bottom_white/ledger/transition_ledger.rs` (TxKind=15) + `src/runtime/verify.rs` + `src/runtime/run_summary.rs` + `src/runtime/audit_assertions.rs` + `genesis_payload.toml` (Trust Root rehash 5-6 files).

#### Step-by-step procedure (Class-4 STEP_B with per-atom §8)

```
1. Branch: git checkout -b feat/p-m4-rebuild
2. Read architect §7.5 verbatim (MANUAL_en.md lines 789-821).
3. Implement CpmmPool struct + CpmmPoolSigningPayload + ledger TxKind + dispatch arm + signing-payload + canonical-digest + verify-arm + run_summary + audit_assertions exhaustive match.
4. Add tests/constitution_cpmm_pool.rs (NEW) — 4 architect §7.5 verbatim tests.
5. Extend tests/constitution_architect_verbatim_struct_binding.rs BINDINGS:
   - P-M4 CpmmPool binding NotYetLanded → Landed
   - P-M4-signing CpmmPoolSigningPayload binding NEW → Landed
6. Register constitution_cpmm_pool gate in scripts/run_constitution_gates.sh.
7. Trust Root rehash: typed_tx.rs + sequencer.rs + transition_ledger.rs + verify.rs + run_summary.rs + audit_assertions.rs (and any other STEP_B file touched).
8. Verify:
   - cargo check --workspace clean
   - cargo test --workspace --no-fail-fast: 1336 → 1340+ (4 new tests)
   - bash scripts/run_constitution_gates.sh: 203 → 204+ (1 new gate)
   - cargo test --lib verify_trust_root_passes_on_intact_repo: PASS
9. Commit on feat/p-m4-rebuild branch.
10. **PRE-§8 dual audit dispatch** (Codex G2 + Gemini at packet-draft time, per
    feedback_dual_audit Class-4 timing rule):
    a. Draft §8 packet: handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM4_§8_PACKET.md
    b. Auto-dispatch Codex + Gemini (parallel)
    c. Wait for both audits
    d. Resolve conflicts per feedback_dual_audit_conflict (conservative wins)
    e. If aggregate VETO: STOP, do NOT file packet for architect §8
    f. If aggregate CHALLENGE: patch in-place, re-dispatch dual audit
    g. If aggregate PASS: file architect §8 packet
11. Architect §8 verbatim ratification → ship.
12. Self-merge to main with --no-ff after architect §8.
13. Push to origin/main (request user authorization first per ship-discipline).
14. Update LATEST.md + MEMORY.md with P-M4 SHIPPED FINAL status.
15. Move to F.4 P-M5 CpmmSwap re-apply (Class-3; no §8).
```

#### Estimated wall-clock: ~2-3 days work + dual-audit cycle + architect §8 wait

### Path B: timestamp_logical drift escalation (alternative; user must explicitly authorize)

If user wants to resolve the TB-13-era schema drift before P-M4, file an architect §10 reclassification directive draft asking whether `timestamp_logical` on `MarketSeedTx` / `CompleteSetMintTx` / `CompleteSetRedeemTx` is intentional pre-Stage-C state or a drift to fix. Blocks P-M4 implementation until verbatim ratification.

**NOT recommended** unless architect-clarification surfaces a load-bearing issue. The drift is bounded (3 typed-tx variants) and forward-bound to architect §10 path; doesn't block Stage C Polymarket sequence per remediation directive scope.

### Path C: Forward-bound work (alternative; user must explicitly authorize)

Per CLAUDE.md §19 (no manipulation by sequencing): Stage C P-M2..P-M9 sequence is the load-bearing critical path. Deferring F.3 to do other work would be the exact pattern §19 prohibits.

If authorized anyway: candidates from "🚧 Open after Polymarket" block in `LATEST.md` are C.5 (PromptCapsule wire-up; Class 3; ~1-2 days), B.4 (CAS Merkle; Class 3-4; ~3-5 days), J.5 (replay sampling; Class 1; ~1 day).

## §2 — Pre-action gate (mandatory)

Per `MEMORY.md` "MUST CHECK BEFORE":

- **Before any new TB charter / G1 audit / pick-next-atom**: `/constitution-landing-check`.
- **Before any `bash run_*.sh` runner script**: `/runner-preflight`.
- **Before writing new `feedback_*.md`**: ask "what mechanism enforces this?" — per `feedback_norm_needs_mechanism`.
- **On any FC1/FC2/FC3 problem**: trace BEFORE designing fix.
- **On any new TB charter**: declare `phase_id` + `roadmap_exit_criteria_addressed` + `kill_criteria_tested`.
- **After TB SHIPPED FINAL or audit rounds > 3**: `/harness-reflect`. (P-M3 SHIPPED but Class-3 with no audit rounds; consider firing only at next FINAL ship like P-M4.)

P-M4 is forward execution against the remediation directive §1.C row 3; constitution-landing-check at session #30 returned PROCEED (0 AMBER). The gate must fire fresh at next session start.

## §3 — Phase F architecture rules (per remediation directive §1.B + §9)

### Per-atom Class-4 §8 cadence (NO batching) — applies to F.3 + F.5

Per `feedback_no_batch_class4_signoff`. Atoms ship sequentially:

```
F.1 P-M2 ✅ → F.2 P-M3 ✅ → F.3 P-M4 → §8 → F.4 P-M5 (Class-3, no §8) → F.5 P-M6 → §8 → F.6/F.7/F.8 P-M7/M8/M9 (non-Class-4) → F.9 Stage C overall §8
```

Class-3 atoms (F.4 P-M5) bypass §8 but still get self-audit + workspace tests at ship gate per `feedback_dual_audit` Class-3 framing.

### Dual audit PRE-§8 timing (Class-4 only)

For F.3 P-M4 + F.5 P-M6 + F.9 Stage C overall: dispatch Codex G2 + Gemini at PACKET DRAFT time, not after architect §8 request. P-M2 was the first exercise (worked first try: R1 CHALLENGE caught + remediated in working tree at zero rollback cost).

### F-DEFERRAL-2 closure (Phase F.3 + F.5)

Phase F.{3,5} rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with a sibling entry for `*SigningPayload` (one per Class-4 atom). Audit witness: `grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs` shows ≥3 entries post-Phase-F (P-M2 already added; P-M4 + P-M6 to follow).

### F-DEFERRAL-1 closure (Phase F.5 P-M6 only)

Phase F.5 P-M6 rebuild MUST extend `tests/constitution_economy_strict_equality.rs` `CONSERVATION_INVARIANT_FILES` to include any new helper-alias file containing CTF conservation logic, OR explicitly attest `# F-DEFERRAL-1: no helper-alias introduced`.

### P-M6 rebuild patches (mandatory at F.5)

Phase F.5 P-M6 rebuild MUST patch:
- **Defect 1 fix**: strict `sum_yes == collateral && sum_no == collateral` (no `min()`).
- **Defect 2 fix**: `router_atomic_rollback_on_failure` test must inject mid-mutation failure via cfg(test) hook + assert full state restoration.

## §4 — Forward queue (post-§30 close; canonical)

| Item | Class | Blocker / status |
|---|---|---|
| **Phase F.3 P-M4 (CpmmPool rebuild)** | 4 STEP_B | Charter-eligible NOW; per-atom §8 + PRE-§8 dual audit |
| Phase F.4 P-M5 (CpmmSwap re-apply) | 3 | Gated on F.3 §8 |
| Phase F.5 P-M6 (Router rebuild + 2 patches) | 4 STEP_B | Gated on F.4; per-atom §8 |
| Phase F.6/F.7/F.8 P-M7/M8/M9 re-apply | 1-3 | Gated on F.5 §8 |
| Phase F.9 Stage C overall §8 | 4 ship | Gated on all atoms green |
| F-DEFERRAL-1 closure | 1 | Closes at F.5 |
| F-DEFERRAL-2 closure (P-M4 + P-M6 sibling bindings) | 1 | Per atom at F.3 + F.5 (F.1 already closed) |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward post-Polymarket |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB |
| K.1-6 Stage D real-world readiness | architect | Decoupled |

## §5 — Memory entries to verify at next-session start

Verify these are present in `MEMORY.md` (added 2026-05-09 session #30):

- **Stage C P-M3 SHIPPED row**: 2026-05-09 session #30; HEAD `73b42d7`; Class-3 re-apply; no §8 needed; gates 198→203, workspace 1331→1336; Phase F.3 P-M4 eligible NOW.
- **Phase F.2 P-M3 line in P-M2-rebuild lineage row**: removed "Phase F.2 P-M3 (Class-3) eligible NOW" sentence (now SHIPPED); Phase F.3 P-M4 + F.5 P-M6 still gated wording preserved.

If any are stale, restore from session #30 work.

## §6 — Key references (canonical sources)

| Reference | Purpose |
|-----------|---------|
| P-M3 commit | `ac06a47` (P-M3 SHIPPED — MarketSeedTx hardening Class-3) |
| P-M3 merge commit | `73b42d7` (Merge feat/p-m3-rebuild → main) |
| Remediation directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` §1.C row 3 (P-M4 next; Class 4 STEP_B) + §9 F-DEFERRAL-2 |
| Architect manual §7.5 P-M4 | `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` lines 789-821 |
| P-M2 reference (mirror pattern) | `tests/constitution_completeset_merge.rs` + `tests/constitution_architect_verbatim_struct_binding.rs` E.1 binding |
| P-M3 gate test | `tests/constitution_market_seed_hardening.rs` (5 architect §7.4 verbatim names; Class-3 pattern reference) |

---

## USER PROMPT (paste this into next Claude session)

```
P-M3 SHIPPED in session #30 (2026-05-09) at HEAD `73b42d7` (pushed
to origin/main). Class-3 re-apply per remediation directive §1.C row
2 verbatim "n/a (was correct); per-atom §8 NO". Constitution gates
203/0/1 (was 198 pre-F.2; +5). Workspace 1336/0/151. No `src/` change
(TB-13-era 7-field MarketSeedTx preserved per Sub-option A2).

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09_post_pm3_ship.md
   (this prompt's source; full context + Phase F.3 P-M4 step-by-step
   + R-022 pre-empt lesson from /harness-reflect)
2. handover/ai-direct/LATEST.md "✅ P-M3 SHIPPED 2026-05-09" block
3. handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md
   §1.C row 3 (P-M4 Class-4 STEP_B; per-atom §8) + §9 F-DEFERRAL-2
4. handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md
   §7.5 P-M4 (lines 789-821; CpmmPool 5-field verbatim — `event_id`
   NOT `event_id_kind` — + 4 mandated tests)

Tell me what you want to do:
(a) Phase F.3 P-M4 CpmmPool rebuild — Class-4 STEP_B atom (per-atom §8
    REQUIRED + PRE-§8 dual audit Codex G2 + Gemini). Pre-empt R-022
    backlinks before first commit per /harness-reflect session #30
    lesson 3. Extend E.1 BINDINGS for CpmmPool + CpmmPoolSigningPayload.
    ~2-3 days work + audit + architect §8. Most likely path per
    remediation directive §1.C row 3.
(b) timestamp_logical drift escalation — file architect §10 reclassi-
    fication directive draft for MarketSeedTx + CompleteSetMintTx +
    CompleteSetRedeemTx schema. Blocks P-M4 until verbatim ratification.
    NOT recommended unless surfacing load-bearing issue.
(c) Forward-bound parallel work — defer Phase F.3 (NOT recommended per
    CLAUDE.md §19 no-manipulation-by-sequencing; Polymarket atom
    sequence is critical path). C.5 PromptCapsule / B.4 CAS Merkle /
    J.5 replay sampling.
(d) Something else — describe it.
```

---

**End of next-session boot prompt (post P-M3 ship).**
