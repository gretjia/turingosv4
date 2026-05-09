# Stage C Polymarket — VETO Remediation Directive (post-rollback rebuild)

**Date**: 2026-05-09 session #28
**Authority**: User architect-role verbatim "我是要 VETO + 全 rollback" (2026-05-09 session #28)
**Companion VETO directive**: `2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md`
**Per CLAUDE.md §10**: Class-4 VETO remediation requires this directive stating authorized changes / forbidden changes / rollback requirement / allowed files-surfaces / ship gates.

---

## §1. Authorized changes

### A. Rollback (Phase B — execute now)
- 11 sequential `git revert` commits as enumerated in VETO directive §4
- Single composite rollback commit pushing HEAD back to the state of commit `b468140` semantically (12-commit revert atop main, additive)
- No force-push, no reset, no `git filter-branch`, no history rewrite

### B. Mechanism additions (Phase E — separate Class-2 atom, file before any P-M2 rebuild)
1. **Verbatim spec binding gate** — new `tests/constitution_architect_verbatim_struct_binding.rs`:
   - Parses architect manual §7.x struct definitions (regex/fixture)
   - Asserts implementation struct field set == verbatim field set (strict equality, not subset)
   - Catches drift like P-M2's `timestamp_logical` and P-M4's `event_id_kind` mechanically
2. **Atomic rollback test pattern enforcement** — new `tests/constitution_class4_atomic_rollback_witness.rs`:
   - For each Class-4 composite tx (router, future composites), assert that its atomic-rollback test triggers failure AFTER `q_next` mutation begins (introspection of test body or runtime witness)
   - Catches vacuous tests like the P-M6 router's pre-mutation rejection
3. **Strict-equality invariant lint** — new gate in `tests/constitution_economy_strict_equality.rs`:
   - Grep `monetary_invariant.rs` for `min(` / `max(` patterns near collateral comparisons
   - Assert conservation laws use `==`, not `min()`
4. **Charter rule codification** — new `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_no_batch_class4_signoff.md`:
   - Hard rule: no batch §8 for Class-4 atoms regardless of wall-clock pressure
   - Cite Stage C session #27 evidence
5. **Pre-§8 dual-audit timing rule** — update `feedback_dual_audit.md`:
   - Codex + Gemini dispatch happens at PACKET DRAFT time (before architect §8 request), not after
   - Rationale: Stage C session #27 dispatched audit AFTER packet was drafted + pushed → caught issue but rollback cost was real

### C. Per-atom rebuild (Phase F — sequential, after Phase E gates land)

Strict per-atom STEP_B + per-atom architect §8 cadence. NO batching. Order:

| # | Atom | Class | Schema fix | Test fix | §8 required |
|---|------|-------|-----------|----------|-------------|
| 1 | P-M2 CompleteSetMergeTx (rebuild) | 4 STEP_B | Remove `timestamp_logical`; strict 6-field per architect §7.3 | n/a | YES per-atom |
| 2 | P-M3 MarketSeed (re-apply) | 3 | n/a (was correct) | n/a | NO |
| 3 | P-M4 CpmmPool (rebuild) | 4 STEP_B | Rename `event_id_kind` → `event_id` per architect §7.5 | n/a | YES per-atom |
| 4 | P-M5 CpmmSwap (re-apply) | 3 | n/a (was correct) | n/a | NO |
| 5 | P-M6 Mint-and-Swap Router (rebuild with patches) | 4 STEP_B | Per architect §7.7 verbatim | **Defect 1 fix**: `monetary_invariant.rs` strict `sum_yes == collateral && sum_no == collateral` (not `min()`). **Defect 2 fix**: `router_atomic_rollback_on_failure` test must inject mid-mutation failure (e.g., pool reserves run out mid-swap; or sequencer panic injection at step 5/6/7 of the 9-step composite) and assert full state restoration | YES per-atom |
| 6 | P-M7 PriceIndex (re-apply) | 2 | n/a (was correct) | n/a | NO |
| 7 | P-M8 audit_tape views (re-apply) | 1 | n/a (was correct) | n/a | NO |
| 8 | P-M9 controlled smoke (re-apply) | 3 evidence | n/a | n/a (regenerate evidence) | NO (Class-3 evidence; no §8 per-atom) |
| 9 | Stage C overall §8 packet | — | — | — | YES overall |

---

## §2. Forbidden changes

1. **NO force-push** to main/origin/main. All rollback via `git revert` (additive).
2. **NO `git reset --hard`** to a pre-Stage-C commit. History preserved.
3. **NO batch §8** for the rebuild — even if rebuild work goes faster than expected.
4. **NO schema deviation** from architect manual §7.3 / §7.5 / §7.7. Field names verbatim. Field count verbatim.
5. **NO `min()` / `max()` in conservation invariants** — strict equality on both sides.
6. **NO vacuous atomic-rollback tests** — test body must exercise mid-mutation failure path.
7. **NO ratification of P-M6 rebuild without Phase E gates landed** — strict-equality gate must catch the pre-fix `min()` if it were reintroduced; verbatim-binding gate must catch field drift.
8. **NO modification of session #25 P-M0 work** (`d33c25a` and prior — already SHIPPED with separate architect §8).
9. **NO retroactive evidence rewrite** of session #27 evidence files (`handover/evidence/stage_c_pm9_controlled_smoke_20260509T042633Z/` etc. remain in tree as historical artifacts; do NOT delete; per `feedback_no_retroactive_evidence_rewrite`).

---

## §3. Rollback requirement

**Mandatory** before Phase E or Phase F work:
1. 11-commit revert chain executed (VETO directive §4)
2. Single composite rollback commit pushed to origin/main
3. Constitution gates verify GREEN at expected baseline (175/0/1)
4. Workspace tests verify GREEN (~1308 passed)
5. Trust Root verify PASS
6. `LATEST.md` updated to reflect rollback
7. `MEMORY.md` updated with Stage C VETO row

If any verification step fails post-rollback (e.g., test count diverges from baseline by more than session-#27-attributable delta), HALT and report — do NOT proceed to Phase E.

---

## §4. Allowed files / surfaces

### Phase B (rollback) — touches:
- `src/state/typed_tx.rs` (revert P-M2 + P-M6 schema additions)
- `src/state/sequencer.rs` (revert admission arms)
- `src/state/q_state.rs` (revert CpmmPool sub-field)
- `src/state/price_index.rs` (revert P-M7 changes)
- `src/economy/monetary_invariant.rs` (revert pool-reserve extension + the min() weakening)
- `src/runtime/run_summary.rs` (revert affected counters)
- `src/bottom_white/cas/schema.rs` (revert P-M2/P-M6 CAS schema bumps)
- `src/bin/audit_tape.rs` (revert P-M8 view subcommands)
- `src/bottom_white/ledger/transition_ledger.rs` (revert affected dispatch)
- `genesis_payload.toml` (revert Trust Root rehash chain)
- `tests/constitution_completeset_merge.rs` (delete via revert)
- `tests/constitution_marketseed_hardening.rs` (delete via revert)
- `tests/constitution_cpmm_pool.rs` (delete via revert)
- `tests/constitution_cpmm_swap.rs` (delete via revert)
- `tests/constitution_router_buy_with_coin.rs` (delete via revert)
- `tests/constitution_price_index_signal_only.rs` (delete via revert)
- `tests/audit_tape_views.rs` (delete via revert)
- `tests/stage_c_pm9_controlled_smoke.rs` (delete via revert)
- `scripts/run_constitution_gates.sh` (revert gate registrations for the above)
- `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md` (revert SHIPPED labels P-M2..P-M9; preserve P-M0 SHIPPED label from session #25)
- `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (revert row promotions for §9 L9 I.2..I.9)
- `handover/ai-direct/LATEST.md` (revert "Open after Polymarket" block from session #27 — re-add after rollback as forward-bound block)
- `handover/decisions/2026-05-09_M2_KILL_AND_SUBSTRATE_STABLE_DECLARATION.md` (KEEP — not part of Stage C ratification; M2 kill decision stands; user verbatim "kill M2" from 2026-05-09 separate from Stage C §8)
- `handover/evidence/stage_c_pm9_controlled_smoke_20260509T042633Z/` (KEEP — historical evidence; per `feedback_no_retroactive_evidence_rewrite`)
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_OVERALL_§8_PACKET.md` (revert via deletion of file)
- `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_PACKET.md` (revert via deletion of file)
- `handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09.md` (revert via deletion — boot prompt was predicated on ship)

### Phase E (mechanism) — net-new files only:
- `tests/constitution_architect_verbatim_struct_binding.rs` (NEW)
- `tests/constitution_class4_atomic_rollback_witness.rs` (NEW)
- `tests/constitution_economy_strict_equality.rs` (NEW)
- `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_no_batch_class4_signoff.md` (NEW)
- `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_dual_audit.md` (UPDATE timing rule)
- `scripts/run_constitution_gates.sh` (register the 3 new gates)

### Phase F (rebuild) — same surfaces as session #27 but with patches per §1.C above.

---

## §5. Ship gates

### Phase B (rollback) ship gates
- SG-VETO.B.1 11-commit revert chain applies cleanly (no merge conflicts)
- SG-VETO.B.2 `cargo check --workspace` clean
- SG-VETO.B.3 `cargo test --workspace --no-fail-fast` GREEN with count matching pre-Stage-C baseline (~1308) ± session-#28 baseline drift
- SG-VETO.B.4 `bash scripts/run_constitution_gates.sh` returns 175/0/1
- SG-VETO.B.5 `cargo test --lib verify_trust_root_passes_on_intact_repo` PASS
- SG-VETO.B.6 `git diff <pre-rollback-HEAD> <post-rollback-HEAD>` shows ONLY revert content (no semantic surprises)

### Phase E (mechanism) ship gates
- SG-VETO.E.1 verbatim-binding gate added; runs on each architect-spec'd struct; current Polymarket spec assertion stub passes
- SG-VETO.E.2 atomic-rollback witness gate added; demonstrates failure on a vacuous-test fixture and pass on a properly-injected fixture
- SG-VETO.E.3 strict-equality lint gate added; demonstrates failure on a `min(sum_yes, sum_no) == collateral` fixture and pass on `sum_yes == collateral && sum_no == collateral`
- SG-VETO.E.4 `feedback_no_batch_class4_signoff.md` filed; `feedback_dual_audit.md` timing-rule updated

### Phase F (rebuild) ship gates per atom — same as session #27 charter §8 ship gates PLUS:
- SG-VETO.F.x.1 verbatim-binding gate (E.1) PASSES on the rebuilt struct
- SG-VETO.F.x.2 strict-equality lint gate (E.3) PASSES on monetary_invariant.rs
- SG-VETO.F.6.3 (P-M6 specific) atomic-rollback witness gate (E.2) PASSES on `router_atomic_rollback_on_failure`
- SG-VETO.F.6.4 (P-M6 specific) Codex G2 + Gemini DUAL audit (full dual per `feedback_dual_audit` Class-4 strict) BEFORE per-atom §8 packet
- SG-VETO.F.x.5 architect verbatim §8 sign-off PER ATOM (Class-4) — strict cadence, no batching

### Stage C overall ship (Phase F.9) ship gates — same as session #27 SG-StageC-PM.1..9 PLUS:
- SG-VETO.F.9.6 NO batch §8 anywhere in the rebuild trail
- SG-VETO.F.9.7 architect overall §8 ratification verbatim multi-clause

---

## §6. Audit cadence (forward)

For Stage C rebuild AND for any future Class-4 work:
1. Packet draft → AUTO-DISPATCH Codex + Gemini dual audit (parallel)
2. Wait for both audits
3. Resolve conflicts per `feedback_dual_audit_conflict` (conservative wins)
4. If aggregate VETO: stop, do not file packet for architect §8
5. If aggregate CHALLENGE: patch in-place, re-dispatch dual audit
6. If aggregate PASS: file architect §8 packet for verbatim ratification
7. Architect §8 verbatim → ship

This codifies dual-audit-PRE-§8 timing (vs session #27 dual-audit-POST-§8-request).

---

## §7. Estimated wall-clock

| Phase | Work | Wall |
|-------|------|------|
| B | Rollback execution + verify | 0.5 day |
| C | LATEST + MEMORY + state docs | 0.25 day |
| D | (subsumed in C) | — |
| E | 3 mechanism gates + 2 memory updates | 1-2 days |
| F.1 | P-M2 rebuild + per-atom §8 wait | 2-3 days work + architect wait |
| F.2 | P-M3 re-apply | 0.25 day |
| F.3 | P-M4 rebuild + per-atom §8 wait | 2-3 days work + architect wait |
| F.4 | P-M5 re-apply | 0.5 day |
| F.5 | P-M6 rebuild with patches + per-atom §8 wait | 4-5 days work (most complex; 2 defects to fix; new failure-injection test pattern) + architect wait |
| F.6-8 | P-M7+P-M8+P-M9 re-apply | 3-4 days |
| F.9 | Stage C overall §8 | 0.5 day work + architect wait |
| **Total** | | **~3-4 weeks (vs. session #27's 1-day batch attempt)** |

The "saved" 3-4 weeks of session #27 batch path is paid back here with interest. Net cost of batch deviation: ~1 week vs strict per-atom from start.

---

## §8. Architect attestation request

This directive operationalizes the VETO + remediation. It does NOT itself require additional architect §8 — it is governed by the VETO directive companion file. However, the user (architect-role) should review §1-§7 above and confirm:

1. Phase B rollback method (full revert; not per-atom selective revert) — confirmed by user verbatim "全 rollback"
2. Phase E mechanism additions (3 new gates + 2 memory updates) — recommended by Codex audit findings; user has not explicitly authorized but they directly address the defects
3. Phase F per-atom rebuild order (strict cadence; no batching) — strict-letter charter compliance per CR-StageC-PM.16

If §1.B (mechanism additions) needs separate authorization or if the rebuild order requires adjustment, architect can note here before Phase E begins.

**Default proceed**: in absence of architect challenge, AI-coder proceeds Phase B → Phase C → Phase E → Phase F sequentially.

---

**Status at file**: 🔵 ACTIVE — governs all Stage C remediation work post-rollback.

---

## §9. Codex Round-2 Acknowledged Deferrals (added 2026-05-09 session #28 post-Phase E')

Codex re-audit round-2 on Phase E' (commit `7995846`) returned CHALLENGE on two residual documentation items. Both are **explicitly deferred to Phase F** with the binding rules below; they are NOT shippable gaps if Phase F honors the deferrals.

### F-DEFERRAL-1 — Helper-alias reduction scan (scope expansion)

**Origin**: Codex round-2 finding F (E.3 helper-alias bypass).

**Bypass shape**: a Phase F implementer could move a `min()`-style reduction into a helper function (e.g. `pub fn safe_min(a: u128, b: u128) -> u128 { a.min(b) }`) defined in a file that E.3 does not currently scan, then call the helper from `monetary_invariant.rs`. The E.3 lint scans only `monetary_invariant.rs` line-by-line; helper-aliased reductions in other files would slip through.

**Phase F binding rule** (mandatory; ship-block on violation):
- Phase F.5 P-M6 rebuild MUST extend `tests/constitution_economy_strict_equality.rs` `CONSERVATION_INVARIANT_FILES` to include any new file containing CTF conservation logic introduced by the rebuild.
- If the rebuild introduces a helper function called from `monetary_invariant.rs` that reduces sum-aggregates, the helper's defining file MUST be added to the lint scan list. Failure to do so is a Phase F.5 ship-blocker per per-atom §8 cadence.
- This deferral is closed at Phase F.5 PASS-time, not Phase E ship-time. Phase E ships with the narrower one-file scan and this binding rule documented.

**Self-witness**: Phase F.5 G2 audit packet MUST cite this deferral and confirm `CONSERVATION_INVARIANT_FILES` was extended (or attest that no helper-alias was introduced). Audit witness:
```
git diff <pre-F.5> <post-F.5> -- tests/constitution_economy_strict_equality.rs
```
must show either `CONSERVATION_INVARIANT_FILES` modification or a `# F-DEFERRAL-1: no helper-alias introduced` comment.

### F-DEFERRAL-2 — Signing-payload field-set binding gate (E.1 expansion)

**Origin**: Codex round-1 finding I + round-2 finding I (signing-payload binding MISSING from E.1).

**Bypass shape**: Codex session #27 defect 3 was P-M2's `timestamp_logical` field added to `CompleteSetMergeTx` AND signed via `CompleteSetMergeSigningPayload`. E.1 currently binds the typed-tx struct fields verbatim, but does NOT bind the signing-payload struct fields. A future Phase F implementer could keep the typed-tx struct correct per architect §7.x, but pollute the signing payload with extra fields (e.g. timestamp_logical only in the signing payload). E.1 would not catch.

**Phase F binding rule** (mandatory; ship-block on violation):
- Phase F.1 P-M2 rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with a sibling entry for `CompleteSetMergeSigningPayload` mirroring the architect §7.3 verbatim 6-field spec (with one additional architect-permitted field for the signing-payload-specific projection: typically the same 6 fields with `signature` removed and `domain_prefix` or `version` added per the existing TB-13 signing-payload pattern).
- Phase F.3 P-M4 / Phase F.5 P-M6 SAME requirement for their respective signing payloads (`CpmmSwapSigningPayload`, `BuyWithCoinRouterSigningPayload`).
- The signing-payload binding entry uses the same `landing_status: Landed | NotYetLanded` mechanism as the typed-tx struct binding. Both bindings must be `Landed` for the per-atom §8 to ship.

**Self-witness**: Phase F.{1,3,5} G2 audit packet MUST cite this deferral and confirm a sibling binding entry was added for the signing-payload projection. Audit witness:
```
grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs
```
must show ≥3 entries (one per Class-4 atom) post-Phase-F.

### Why these are deferral-acceptable rather than Phase E blockers

Both F-DEFERRAL-1 and F-DEFERRAL-2 are **future-bypass risks**, not current defects. The current Phase E + E' implementation:
- catches all 4 historical session #27 defects (Codex round-1 PASS confirmed)
- tightens the 3 round-1 CHALLENGE items (Codex round-2 CLOSED on 6 of 8 sub-items)
- the 2 residuals (F + I) require active malice or scope expansion to hit
- closing them now via additional gates would require either (a) cross-file scan logic (helper-alias) or (b) signing-payload struct extraction (architectural overlap with E.1) — both are cleaner to do at Phase F.x boundary when the actual concrete signing payload struct lands

`feedback_elon_mode_policy` 2-round cap is binding for THIS audit chain. Round-3 would be required to ship a deeper-hardened gate at Phase E ship-time, and round-3 needs explicit user authorization. The user (architect-role) authorized Phase E' + round-2 only; deferring residuals to Phase F honors that authorization scope.

**Status of §9 deferrals**: 🟡 ACTIVE-DEFERRAL — closed by per-atom Phase F.{1,3,5} ship gates per binding rules above.
