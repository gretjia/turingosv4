# Next Session Boot Prompt — 2026-05-09 session #28 close (post-Phase-E)

> Paste the **`USER PROMPT`** block at the bottom into the next Claude session. Everything above it is context for cold-start orientation if you read this file directly.

---

## State at session #28 close (2026-05-09)

- **HEAD on `origin/main`**: `fcb19c9` (pushed; 7 commits ahead of `a734cbe` baseline)
- **Stage C status**: Class-4 batch §8 VETOED + fully rolled back; Phase E (3 mechanism gates) + Phase E' (Codex-driven hardening) + Phase E'' (deferral docs) all SHIPPED
- **Constitution gates**: `193 PASS / 0 FAIL / 1 ignored` (was 175 pre-Stage-C; +18 net from new gates' main + self-check tests)
- **Workspace tests**: `1326 PASS / 0 FAIL / 151 ignored` (was 1308; +18 from new gate self-tests)
- **Trust Root**: `src/economy/monetary_invariant.rs` rehashed `91f66421` → `c4e1e258` for the symmetric/asymmetric `assert_complete_set_balanced` split
- **Two architect-pending items**: F-DEFERRAL-1 (helper-alias scan) + F-DEFERRAL-2 (signing-payload binding) — both deferred to Phase F.x with ship-block binding rules per remediation directive §9

## Audit chain summary

| Round | Target | Codex agent | Verdict | Action |
|---|---|---|---|---|
| G2 #1 | Stage C batch §8 packet @ `7395aaa` | `a1e5cd6edeb8377bc` | **AGGREGATE VETO** (4 defects, P-M6 load-bearing) | Architect VETO + 11-commit rollback |
| Re-audit #1 | Phase E @ `4b9ea6b` | (unrecorded) | **CHALLENGE** (per-defect PASS; 3 hardening items) | Phase E' patch |
| Re-audit #2 | Phase E' @ `7995846` | `a5b367d486e1cebde` | **CHALLENGE** (6 of 8 sub-items CLOSED; 2 doc residuals → F-DEFERRAL-1/2) | Phase E'' deferral docs (round-cap met) |

## Critical files for cold-start orientation

Read in this order:

1. **`CLAUDE.md`** — project constitution (§4 strategic decisions; §6 FC1 invariant; §10 Class-4 authorization; §12 STEP_B; §13 economy laws)
2. **`constitution.md`** — top-level law (architect verbatim spec)
3. **`handover/ai-direct/LATEST.md`** — top "🔴 Stage C Polymarket VETOED" block + "✅ Phase E SHIPPED 2026-05-09 session #28" block (canonical current state)
4. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md`** — operative directive §1.B Phase E + Phase F binding rules + §9 acknowledged deferrals
5. **`handover/architect-insights/2026-05-09_PHASE_E_PLAN_cached-noodle.md`** — architect-approved plan (in-repo archive; supersedes `cozy-waddling-raven.md`)
6. **`handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md`** — VETO directive (Codex audit findings + architect verbatim "我是要 VETO + 全 rollback")
7. **`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`** — verbatim engineering spec for Phase F atoms (§7.3 P-M2 / §7.5 P-M4 / §7.7 P-M6 + others)
8. **`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`** — gate-level CI view
9. **`MEMORY.md`** — "MUST CHECK BEFORE" pre-action gates + Active state lineage

## §1 — Likely paths the next session takes

### Path A: Phase F.1 P-M2 rebuild (most likely; per remediation directive §1.C)

**Class-4 STEP_B atom** — first per-atom Class-4 rebuild after Stage C VETO.

#### Pre-flight (mandatory before any code)

1. **Invoke `/runner-preflight`** (NOT applicable here — runner-preflight is for `bash run_*.sh` scripts mutating evidence dirs; Phase F.1 is in-repo source/test work, no evidence mutation. Skip with explicit acknowledgment.)
2. **Invoke `/constitution-landing-check`** — surfaces any AMBER rows; should return PROCEED since matrix is 0 AMBER.
3. **Verify HEAD `fcb19c9`** matches `origin/main`.
4. **Verify `bash scripts/run_constitution_gates.sh`** returns `193 / 0 / 1` baseline.

#### Step-by-step procedure

```
1. Branch: `git checkout -b feat/p-m2-rebuild`

2. Implement CompleteSetMergeTx per architect §7.3 VERBATIM:
   - src/state/typed_tx.rs: pub struct CompleteSetMergeTx { tx_id: TxId,
     parent_state_root: Hash, event_id: EventId, owner: AgentId, amount:
     ShareAmount, signature: AgentSignature } — EXACTLY 6 fields, NO
     timestamp_logical, NO additional fields. Codex defect 3 was the
     timestamp_logical drift.
   - Add CompleteSetMergeSigningPayload per remediation directive §9
     F-DEFERRAL-2: 6-field architect-verbatim projection (typically
     5 fields without `signature` + 1 domain prefix).
   - src/state/sequencer.rs: admission arm per architect §7.3 verbatim
     semantics (require owner YES ≥ amount AND owner NO ≥ amount; burn
     amount of each; debit collateral; credit owner Coin).
   - src/bottom_white/cas/schema.rs: CAS schema bump (Class-4 STEP_B file).

3. Add 5 verbatim tests per architect §7.3:
   tests/constitution_completeset_merge.rs:
   - merge_yes_no_returns_coin
   - merge_requires_both_sides
   - merge_conserves_total_coin
   - merge_reduces_collateral
   - merge_unavailable_after_final_redeem_if_shares_exhausted

4. **Flip E.1 binding for P-M2 to Landed** (in
   tests/constitution_architect_verbatim_struct_binding.rs):
   - Change `landing_status: LandingStatus::NotYetLanded` →
     `LandingStatus::Landed` for the P-M2 §7.3 binding entry.
   - This forces strict (name, type) pair equality enforcement.

5. **Add CompleteSetMergeSigningPayload binding per F-DEFERRAL-2**:
   - New BINDINGS entry for the signing-payload struct with verbatim
     field set.
   - landing_status: Landed (since P-M2 rebuild ships them together).

6. Trust Root rehash:
   - `sha256sum src/state/typed_tx.rs src/state/sequencer.rs
     src/bottom_white/cas/schema.rs`
   - Update genesis_payload.toml [trust_root] entries with new hashes
     + comment trail.

7. Verify (Class-4 atom; STEP_B branch):
   - `cargo check --workspace` clean
   - `cargo test --workspace --no-fail-fast` GREEN; count 1326 → 1331
     (+5 verbatim tests)
   - `bash scripts/run_constitution_gates.sh` 193 → 198 (+5 if
     constitution_completeset_merge registered) — register the gate in
     scripts/run_constitution_gates.sh
   - `cargo test --lib verify_trust_root_passes_on_intact_repo` PASS
   - E.1 binding (P-M2 Landed) PASSES (verbatim 6-field match)
   - F-DEFERRAL-2 closure: signing-payload binding PASSES

8. Commit on branch `feat/p-m2-rebuild`. Single atomic commit:
   `P-M2 SHIPPED — CompleteSetMergeTx 6-field verbatim per architect §7.3`

9. Self-merge to main with --no-ff:
   `git checkout main && git merge --no-ff feat/p-m2-rebuild`

10. **Draft §8 packet** at
    `handover/directives/2026-05-XX_STAGE_C_POLYMARKET_PM2_§8_PACKET.md`:
    - All charter §3.3 ship gates verified
    - Trust Root rehash log
    - STEP_B branch evidence
    - FC1 invariant statement (P-M2 doesn't touch externalized-attempt
      accounting)
    - Genesis-replayability statement
    - Per-atom §8 sign-off request (NO BATCHING per
      feedback_no_batch_class4_signoff)

11. **AUTO-DISPATCH DUAL AUDIT PRE-§8** per
    feedback_dual_audit Class-4 timing rule:
    - Codex G2 audit: dispatch via `Agent` tool with subagent_type
      `codex:codex-rescue` and a focused prompt (template: pre-§8
      packet review on commit hash). Audit target: verify (a) struct
      verbatim per §7.3, (b) test bodies exercise real paths, (c) no
      F-DEFERRAL-1/2 violations.
    - Gemini parallel: dispatch via existing TuringOS Gemini wrapper
      (or `WebFetch` if available; degraded label per
      feedback_dual_audit if unavailable).
    - Wait both verdicts. Conservative-wins: VETO > CHALLENGE > PASS.
    - VETO → roll back P-M2 commit; reopen with patches.
    - CHALLENGE → patch in-place; re-dispatch (round 2 within elon-mode
      cap; round 3 needs explicit user authorization).
    - PASS (both Codex AND Gemini) → file packet for architect §8.

12. Architect verbatim §8 — wait. NEVER self-grant. Wait for verbatim
    multi-clause form like `好，确认可以 ship` or `同意 sign-off`.

13. On architect §8 ratification:
    - File `handover/directives/2026-05-XX_STAGE_C_POLYMARKET_PM2_§8_SIGN_OFF.md`
    - Push to origin/main (request user authorization)
    - Update LATEST.md (mark P-M2 SHIPPED)
    - Update MEMORY.md (Active state)
    - Move to F.2 (P-M3 re-apply, non-Class-4, no §8 needed)
```

#### Estimated wall-clock: ~2-3 days work + architect §8 wait

### Path B: Forward-bound work (alternative; user must explicitly authorize)

If user wants to defer Phase F start, candidate items from "🚧 Open after Polymarket" block in LATEST.md:

| Item | Class | ETA | Reason non-blocking |
|---|---|---|---|
| C.5 PromptCapsule evaluator wire-up | 3 | ~1-2 days | Affects LLM-Lean attempt path; Polymarket sequencer doesn't read PromptCapsule |
| B.4 CAS Merkle redesign (Stage A3.6 enhancement TB) | 3-4 | ~3-5 days | Replay reconstructs via cas/.git/objects + sidecar; market L4 anchor unaffected |
| J.5 4 replay sampling tests | 1 | ~1 day | Gate-level only; gated on M2 evidence |
| K.1-6 Stage D real-world readiness | architect | architect-side | Decoupled from Polymarket per manifest §11 |

**DO NOT START** Path B unless user explicitly authorizes. Phase F.1 P-M2 rebuild is the canonical critical-path forward atom.

### Path C: Architect needs to ratify Phase E retroactively (low likelihood)

Phase E + E' + E'' are Class-2 production wire-up (gate additions + source refactor of `assert_complete_set_balanced` not touching schema/signing/admission). Per CLAUDE.md §9 Class 2: dual audit at ship gate is the requirement, which we did. No architect §8 required for Class-2.

If architect (user) wants to retroactively ratify Phase E for the audit trail, file `handover/directives/2026-05-XX_PHASE_E_§8_SIGN_OFF.md` with verbatim ratification text. Optional — Phase F can proceed without it since Phase F atoms are independent per-atom.

## §2 — Pre-action gate (mandatory)

Per `MEMORY.md` "MUST CHECK BEFORE":

- **Before any new TB charter / G1 audit / pick-next-atom**: invoke `/constitution-landing-check`.
- **Before any `bash run_*.sh` runner script**: invoke `/runner-preflight`.
- **Before writing new `feedback_*.md`**: ask "what mechanism enforces this?" — per `feedback_norm_needs_mechanism`.
- **On any FC1/FC2/FC3 problem**: trace BEFORE designing fix.
- **On any new TB charter**: declare `phase_id` + `roadmap_exit_criteria_addressed` + `kill_criteria_tested`.

Phase F.1 is forward execution against an already-ratified plan (`cached-noodle.md`); the constitution-landing-check at session #28 returned PROCEED. If next session opens new charter work, the gate must fire fresh.

## §3 — Phase F architecture rules (per remediation directive §1.B + §9)

### Per-atom Class-4 §8 cadence (NO batching)

Per `feedback_no_batch_class4_signoff`. Atoms ship sequentially:
- F.1 P-M2 → §8 → F.2 P-M3 (non-Class-4) → F.3 P-M4 → §8 → F.4 P-M5 (non-Class-4) → F.5 P-M6 → §8 → F.6/F.7/F.8 P-M7/M8/M9 (non-Class-4) → F.9 Stage C overall §8

### Dual audit PRE-§8 timing

Per `feedback_dual_audit` Class-4 timing rule (added 2026-05-09 per remediation directive §1.B.5). Dispatch Codex + Gemini at PACKET DRAFT time, not after architect §8 request. Audit findings cycle in working tree (rollback is free); not on origin/main.

### F-DEFERRAL-1 closure (Phase F.5 P-M6)

Phase F.5 P-M6 rebuild MUST extend `tests/constitution_economy_strict_equality.rs` `CONSERVATION_INVARIANT_FILES` to include any new helper-alias file containing CTF conservation logic, OR explicitly attest `# F-DEFERRAL-1: no helper-alias introduced`.

### F-DEFERRAL-2 closure (Phase F.1, F.3, F.5)

Phase F.{1,3,5} rebuild MUST extend `tests/constitution_architect_verbatim_struct_binding.rs` BINDINGS array with a sibling entry for `*SigningPayload` (one per Class-4 atom). Audit witness: `grep "SigningPayload" tests/constitution_architect_verbatim_struct_binding.rs` shows ≥3 entries post-Phase-F.

### P-M6 rebuild patches (mandatory)

Phase F.5 P-M6 rebuild MUST also patch:
- **Defect 1 fix**: when CPMM pool reserves enter `assert_complete_set_balanced` sums, the symmetric/asymmetric branch split (Phase E.3 source refactor) handles it; F.5 verifies Branch B's `min()` doesn't admit pool-induced asymmetry. May require explicit resolution-state tracking (out-of-Phase-E scope; F.5 design call).
- **Defect 2 fix**: `router_atomic_rollback_on_failure` test must inject mid-mutation failure (e.g. via `inject_failure_after_step()` helper or `set_var("ROUTER_FAIL_AT_STEP", N)` env-var) and assert full state restoration. Sequencer cfg(test) injection point is part of F.5 STEP_B per plan §C.E.2.

## §4 — Forward queue (post-§28 close; canonical)

| Item | Class | Blocker / status |
|---|---|---|
| **Phase F.1 P-M2 rebuild** | 4 STEP_B | Charter-eligible NOW; first per-atom Class-4 |
| Phase F.2 P-M3 re-apply | 3 | Gated on F.1 §8 |
| Phase F.3 P-M4 rebuild | 4 STEP_B | Gated on F.2 |
| Phase F.4 P-M5 re-apply | 3 | Gated on F.3 §8 |
| Phase F.5 P-M6 rebuild + 2 patches | 4 STEP_B | Gated on F.4 |
| Phase F.6/F.7/F.8 P-M7/M8/M9 re-apply | 1-3 | Gated on F.5 §8 |
| Phase F.9 Stage C overall §8 | 4 ship | Gated on all atoms green |
| F-DEFERRAL-1 closure (E.3 helper-alias scope) | 1 | Closes at F.5 |
| F-DEFERRAL-2 closure (E.1 signing-payload binding) | 1 | Closes per atom at F.1/F.3/F.5 |
| C.5 PromptCapsule evaluator wire-up | 3 | Forward post-Polymarket |
| B.4 CAS Merkle redesign | 3-4 | Stage A3.6 enhancement TB |
| K.1-6 Stage D real-world readiness | architect | Decoupled |

## §5 — Memory entries to update at next-session start (verify)

Verify these are present in `MEMORY.md` (added 2026-05-09 session #28):
- "Stage C Polymarket VETOED + ROLLED BACK 2026-05-09 session #28" row in Active state
- Updated "Latest VETO (active)" pointer to Stage C batch §8 VETO directive
- "[No batch §8 for Class-4](feedback_no_batch_class4_signoff.md)" entry under AUDIT & SHIP DISCIPLINE
- Dual audit timing rule appended in `feedback_dual_audit.md`

If any are missing or stale, restore from session #28 work.

## §6 — Key references (canonical sources)

| Reference | Purpose |
|-----------|---------|
| `cached-noodle.md` archive | `handover/architect-insights/2026-05-09_PHASE_E_PLAN_cached-noodle.md` (in-repo per Codex Recommendation 3) |
| Remediation directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md` (§1 authorized changes / §9 acknowledged deferrals) |
| VETO directive | `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM2_PM4_PM6_BATCH_§8_VETO.md` |
| Architect manual | `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md` §7.3/§7.5/§7.7 verbatim specs |
| OBS_R022 | `handover/alignment/OBS_R022_STAGE_C_VETO_ROLLBACK_2026-05-09.md` (TRACE_MATRIX bulk-removal justification) |
| Gate test files | `tests/constitution_architect_verbatim_struct_binding.rs` (E.1) / `tests/constitution_class4_atomic_rollback_witness.rs` (E.2) / `tests/constitution_economy_strict_equality.rs` (E.3) |
| Codex audit transcripts | agent IDs `a1e5cd6edeb8377bc` (round-1 VETO) / `a5b367d486e1cebde` (round-2 verification) — replayable via tool history |

---

## USER PROMPT (paste this into next Claude session)

```
Stage C VETO + Phase E (mechanism gates + monetary_invariant split) shipped
in session #28 (2026-05-09) at HEAD `fcb19c9` (pushed to origin/main).
Constitution gates 193/0/1 (was 175 pre-Stage-C). All 4 historical defects
from session #27 audit are caught at gate-time per Codex round-1 PASS.

Read first:
1. handover/ai-direct/NEXT_SESSION_PROMPT_2026-05-09_post_phase_e.md
   (this prompt's source; full context + Phase F.1 step-by-step)
2. handover/ai-direct/LATEST.md top "🔴 Stage C VETOED" + "✅ Phase E SHIPPED" blocks
3. handover/directives/2026-05-09_STAGE_C_POLYMARKET_VETO_REMEDIATION_DIRECTIVE.md
   (especially §1.B Phase F binding rules + §9 F-DEFERRAL-1/2)
4. handover/architect-insights/2026-05-09_PHASE_E_PLAN_cached-noodle.md
   (architect-approved plan archive)

Tell me what you want to do:
(a) Phase F.1 P-M2 rebuild — first per-atom Class-4. ~2-3 days work +
    architect §8 wait. Strict §7.3 verbatim 6-field spec; flip E.1
    binding to Landed; add F-DEFERRAL-2 signing-payload binding;
    Trust Root rehash; STEP_B branch; auto-dispatch dual audit
    PRE-§8; per-atom §8 sign-off.
(b) Forward-bound work in parallel — pick from "Open after Polymarket"
    (C.5 PromptCapsule wire-up / B.4 CAS Merkle / J.5 replay sampling).
    Recommended only if Phase F.1 is blocked or you want a parallel
    track. Phase F.1 is canonical critical path.
(c) Phase E retroactive §8 ratification — optional Class-2
    architect ratification for audit trail. Not required for Phase F
    progression.
(d) Something else — describe it.
```

---

**End of next-session boot prompt.**
