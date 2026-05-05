# TB-18 Atom B Phase 4 — Class 3 Dual External Audit Request (2026-05-05)

## Filed for

Per TB-18 charter §6.6 + ratification §1 atom B (Class 3 dual audit mandatory) + `feedback_dual_audit` (Codex + Gemini full dual for production wire-up).

## Commit scope

- Phase 1 commit: SharedChain init lift (`chain_runtime.rs::from_env`)
- Phase 2 commit: synthetic L4/L4.E gate + genesis_report lift (`chain_runtime.rs::write_synthetic_l4_l4e_gate_and_genesis_report`)
- Phase 3 commit: drive_task substantive body (`drive_task.rs::drive_task`)
- Phase 4 commit: comprehensive_arena.rs single-process multi-task driver (`bin/comprehensive_arena.rs`)
- Phase 5 commit: this audit-request doc + ship status update + commit

(All five phases land in one TB-18.B-impl commit per `feedback_iteration_cap_24h` Class 3 production wire-up exception 72h-to-feedback-loop.)

## Risk-class declaration

**Class 3** per `feedback_risk_class_audit`. Production wire-up surface, but stays inside the Class 3 envelope:

- ✅ NO modification to `src/state/sequencer.rs` (admission)
- ✅ NO modification to `src/state/typed_tx.rs` (schema)
- ✅ NO modification to canonical-signing-payload digests (Phase 1 + 2 + 3 + 4 only consume existing `make_real_*_signed_by` + `emit_system_tx` APIs)
- ✅ NO modification to `src/{kernel,bus}.rs` or `src/sdk/tools/wallet.rs` (no STEP_B_PROTOCOL trigger)
- ✅ NO new TypedTx variants
- ✅ Genesis QState shape unchanged (uses existing `default_pput_preseed_pairs` factory)

Per `feedback_class4_cannot_hide_in_class3`: explicit confirmation no Class 4 surface introduced.

## Files to audit

### NEW (added by this commit)
- `experiments/minif2f_v4/src/chain_runtime.rs` (Phase 1 + 2 lifts; ~430 lines including doc-comments)

### MODIFIED (lifted code + helper-wiring)
- `experiments/minif2f_v4/src/lib.rs` (registered `pub mod chain_runtime`)
- `experiments/minif2f_v4/src/bin/evaluator.rs` (Phase 1 lift + Phase 2 helper-call replacement; ~175 lines deleted, ~25 lines call-site)
- `experiments/minif2f_v4/src/drive_task.rs` (Phase 3 substantive body replacing atom A.1 stub; `DriveTaskError::PendingAtomB` removed; new `ChaintapeRequired/AgentKeypairsRequired/SigningFailed/SubmitFailed` variants)
- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` (Phase 4 full rewrite; subprocess-spawn pattern eliminated)
- `genesis_payload.toml` (evaluator.rs SHA-256 rehash for Phase 1 + Phase 2)

### NEW evidence (committed)
- `handover/evidence/tb_18_b_phase4_2026-05-05/README.md`
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/runtime_repo.dotgit.tar.gz` (chain bytes; 31 L4 entries)
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/cas.dotgit.tar.gz` (CAS bytes)
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/runtime_repo/{agent_pubkeys,agent_audit_trail,genesis_report,initial_q_state,pinned_pubkeys,rejections,synthetic_rejection_label}.json[l]`
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/cas/.turingos_cas_index.jsonl`
- `handover/evidence/tb_18_b_phase4_2026-05-05/r1/evidence/{SHARED_CHAIN_RUNS_REPORT,tx_kind_distribution}.json`

## Architect-mandated questions for the auditors

### Codex (security + schema rigor)

1. **Single-chain mandate (§2.8)**: does the run-time graph genuinely show ONE Sequencer / ONE bundle / ONE runtime_repo / ONE CAS across all 6 tasks? Inspect `comprehensive_arena.rs::run_arena` ownership — confirm no subprocess fork, no second bundle creation.
2. **Class 4 hidden-in-Class-3 (CR-18.7)**: do any of Phase 1-4 modifications inadvertently bump sequencer admission / typed-tx schema / signing-payload digest? Compare git diff of `src/state/typed_tx.rs` + `src/state/sequencer.rs` (should be empty).
3. **Multi-chain UNION (CR-18.8)**: confirm `tx_kind_distribution.json` is computed across ONE chain (single `refs/transitions/main` history), not stitched from N chains.
4. **`drive_task` re-entrancy (FR-18.1)**: simulate calling `drive_task` 10× against one chain with distinct `TaskSpec.theorem_name`s. Are tx_id collisions impossible? Are there race conditions between successive `bundle.sequencer.q_snapshot` reads and `bus.submit_typed_tx` queuing?
5. **Phase 1 byte-identical (mechanical extraction)**: confirm the 175-line lift from `evaluator.rs::run_swarm` lines 659-833 into `SharedChain::from_env` preserves all failure-mode semantics (chaintape bootstrap fail → `exit(2)`; keystore init fail → `expect()`-panic; WAL open fail → in-memory fallback).
6. **`drive_task::PendingAtomB` removal**: confirm no stale callers reference the removed error variant.
7. **agent_keystore password handling**: `comprehensive_arena.rs::run_arena` sets `TURINGOS_AGENT_KEYSTORE_PASSWORD = "tb18-arena-localdev"` if unset. Acceptable for solo-research / sandbox use but ship-blocker if the binary is ever deployed in a multi-tenant context. Recommend `feedback_kolmogorov_compression` MVP pattern be carried forward (the same pattern evaluator uses).
8. **Synthetic-rejection L4.E gate**: in Phase 2 free function, the `synthetic_rejection_label.json` rationale field references "TB-6 Atom 3" + "architect ruling 2026-05-01 § 3.6 Atom 3". Confirm the message is non-misleading when `seed_id` is a chain-level identifier (not the original per-task `run_id`).

### Gemini (architectural compliance + drift)

1. **Architect §2.8 verbatim**: read `comprehensive_arena.rs` end-to-end. Does it satisfy "one process / one runtime_repo / one CAS / one chain / multiple tasks" in spirit AND letter? Are there any architectural cracks where a future contributor might re-introduce subprocess-spawn semantics?
2. **`feedback_chaintape_externalized_proposal` alignment**: synthetic real-signed envelopes vs LLM-driven envelopes — does this represent a meaningful chain-shape equivalence, or is it a quality regression vs the M0 retry?
3. **`feedback_no_workarounds_strict_constitution` ("我不要凑活")**: is the synthetic envelope path 凑活, or is it a principled separation of (a) chain-tx-pipeline coverage from (b) LLM-driven solve evidence, with (b) already saturated by the M0 retry?
4. **`feedback_audit_loop_roi_flip`**: this is the FIRST audit on Phase 4 evidence. Iteration count 0 — full audit applies. If subsequent rounds shift to test-scaffold edge cases, expect ROI flip.
5. **EvidencePackagingPolicy compliance**: r1 evidence dir uses `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per TB-7R/TB-8/TB-9 precedent. The `_dotgit_post_tar/` in-place restore copies are local-only (NOT git-tracked). Is this acceptable, or should they be deleted before commit? (User previously denied `rm -rf` — kept on disk for verification.)
6. **`r0_wrong_cas_env/` non-canonical artifact**: kept as troubleshooting artifact (CAS env-var typo `TURINGOS_CHAINTAPE_CAS_PATH` instead of `TURINGOS_CAS_PATH`). Should it be deleted from the commit? (Pre Gemini Q5 caution above.)
7. **`feedback_no_retroactive_evidence_rewrite`**: confirm Phase 4 does not modify any pre-Phase-4 evidence (TB-7R / TB-13 / TB-14 / TB-15 / TB-16 / TB-17 evidence dirs); only adds new `tb_18_b_phase4_2026-05-05/`.
8. **Phase 5 ship-claim**: should the commit message say "Atom B Phase 4 SHIPPED, single-chain 13/13 substantive build complete" or narrow further per architect Q2 ratification? The 13/13 claim is structurally true; the missing piece for full TB-18 ship is M1/M2 (atom H sub-stages 2/3).

## Conservative-resolution rule

Per `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. If Codex VETO conflicts with Gemini PASS, conservative wins.

Per `feedback_audit_obs_bias`: cheap fixes get fixed; multi-hour future-arch get OBS-deferred. Don't bucket-OBS round 1.

## Sync mode

Mode (ii) per TB-18 ratification — ratify-then-run-to-ship-gate. The TB-18.B-impl commit is filed with this audit request; user invokes Codex + Gemini externally; auditor verdicts trigger either ship-clean or remediation commits.

## Filed by

AI-coder (Claude) — TB-18 Atom B Phase 1+2+3+4 — 2026-05-05.

Filed under user blanket auto-mode authority "自主执行一直到 TB-18 ship".
