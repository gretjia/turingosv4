# Phase 3 Re-Apply Report — H-HET convergence onto origin/main

**Date:** 2026-06-16 · **Class:** convergence (de-Lean re-apply; no trust-root rehash).
**Branch:** `claude/het-converge-2026-06-16` (fresh, off `origin/main` @ `61ec26c7`).
**HEAD before commit:** `61ec26c7` (= origin/main; this is a clean re-apply, **not** an
in-place merge commit — no het merge-parent in the lineage).
**Staged value delta:** 215 files. **Not pushed.**

## What was done
- Re-applied het-carrier-freeze net-new onto a fresh origin/main branch using git's 3-way
  engine, then resolved by hand. **No merge-parent** (MERGE_HEAD cleared before commit).
- **Honored main's retirements:** `src/bin/lean_{hayek,hetero,tree}_market.rs` stay deleted
  (main retired them; verified main never carries them). Not resurrected.
- **Trust-root deferred (hard stop):** `genesis_payload.toml` left at main's pins — **0**
  pin edits. No `rules/MANIFEST.sha256` edits.
- **GA-9 untouched (hard stop):** `src/drivers/llm_http.rs` not staged (0).
- **Evidence excluded:** 0 `handover/evidence/` files staged (net-new gates embed fixtures).
- **No paid experiment run.**

## Conflict resolutions (9/9)
| file | resolution |
|---|---|
| `skills/no-proven-checklist.md` | kept het's 101-line forensic version |
| `tests/constitution_router_name_matches_mechanism.rs` | kept het's §17.3 forensic version |
| `src/runtime/audit_assertions.rs` | de-Lean names (`VerifierResult`, `read_verifier_result_from_cas`) |
| `src/judges/lean_judge.rs` | kernel enums → `Verifier*`; local judge `AxiomCheckStatus::LeanFailed` kept |
| `src/bin/lean_market_agent.rs` (29 hunks) | het superset (already contained main's price logic) + de-Lean; §17.3 price/softmax substrate, model_id-on-tape, action_source/decision_source preserved |
| `scripts/constitution_gates.manifest.toml` | UNION (198 gates; main's ~194 kept + 4 net-new; `router_name_matches_mechanism` de-duped to het's forensic authority) |
| `OBLIGATIONS.md` | UNION (het OBL-014..018 + main's renumbered to OBL-019..021; no obligation dropped) |
| `handover/ai-direct/LATEST.md` | het H-HET snapshot (derived view) |
| `tests/agents_md_keep_anchors.rs` | UNION of both sides' anchor checks |

## De-Lean reconciliation (the real tail — completed)
Main's 194 commits used kernel names het renamed. Drove `cargo check --all-targets` to green by
renaming main's usages per `handover/tracer_bullets/DE_LEAN_KERNEL_MIGRATION_SPEC_2026-06-15.md`:
`LeanResult→VerifierResult`, `LeanVerdictKind→VerifierVerdictKind`, `LeanErrorClass→VerifierErrorClass`,
`from_lean_run→from_verifier_run`, `read_lean_result_from_cas→…verifier…`, `candidate_tactic→candidate_label`,
`lean_{exit_code,stdout_hash,stderr_hash}→{exit_code,…}`, `lean_result_cid/partial_lean_result_cid→verifier_*`,
`RejectionClass::{LeanFailed→CheckerFailed, SorryBlocked→IncompleteProofBlocked}`,
`Capability::LeanOracle→DomainOracle`, `PredicateVerifyError::LeanChecker*→External*`,
`AbortCause::WallClockCapDuringLean→…Verify`, + `model_id: None` on main's `ProposalTelemetry` test literals,
+ 2 `LeanOutcome` literals' new axiom fields. Math-domain LOCAL names (`LeanOutcome`, `LeanJudge`,
`lean_judge`, `het_capability_probe` Lean-toolchain calls) kept per policy.

**Self-caught error (no fake green):** a broad guarded `perl` over-reached and rewrote the
string literal `"LeanResult"`→`"VerifierResult"` inside the `de_lean_legacy_decode` wire-string
fixture (guard caught `#[serde(...)]` but not `serde_json::from_str("LeanResult")`). The
`de_lean_legacy_decode` **gate caught it**; fixed by restoring het's authoritative clean-add files.

## Verification (run, not asserted)
- `cargo check --all-targets` → **exit 0** (lib + all bins + all tests compile).
- `cargo fmt` → clean.
- Merge-relevant gates GREEN (cargo test, direct): `de_lean_legacy_decode`,
  `router_name_matches_mechanism`, `budget_decision_tape_canonical`, `headline_recompute_from_tape`,
  `matrix_drift`, `pput_anti_goodhart_battery`, `vpput_reconstructed_from_tape`, `fc3_meta_loop_closure`,
  `fc3_proposer_canary_observable`, `fc_liveness_observer`, `agents_md_keep_anchors`,
  `tb_7r_parent_tx_conformance`, `lean_judge_realign_regression`, `lean_market_agent_shares_dealign_bug`,
  `market_tape_canonical_roundtrip`.
- **Full `run_constitution_gates.sh` NOT run locally** — it uses GNU `grep -P` (macOS BSD grep
  rejects); the manifest discover-vs-manifest cross-check passes compile-free. Full ~198-gate suite
  green is **Linux/CI-deferred** (also avoids the 46-min broad-AGI `true_suite_*` runners).

## Known reds (4 — characterized; NONE are de-Lean reconciliation defects)
1. **`het_tape_reconstructibility` (5)** — MERGE-INDUCED: main's sequencer added
   `AgentManifestRequired` admission that het's clean-add Gate-D bootstrap (synthetic unsigned
   `TaskOpen` + lazy agent registration) predates. Fix: register `reg.manifest()` via
   `sequencer.set_agent_pubkeys(...)` + properly sign the `TaskOpen`. Bootstrap update, not a
   tape-reconstruction defect.
2. **`t2_microstructure_conformance` (2)** — CONSEQUENCE OF HONORED RETIREMENT: het's test reads
   `src/bin/lean_hayek_market.rs` (main retired it). Fix: repoint t2 source-reads to the canonical
   substrate `src/bin/lean_market_agent.rs`.
3. **`het_probe_pool_reference_bodies_verify` + `het_third_bug_dealign_decisive` (2)** —
   ENVIRONMENT/SCIENCE: real Lean ran (27s/12s) and rejected `Classical.choice` as non-whitelisted.
   het-1 carrier axiom-whitelist behavior, Lean/Mathlib-environment-dependent — out of de-Lean scope.

## Unresolved — for Phase 4 (architect-gated, NOT done here)
- **Trust-root rehash** on final merged bytes (the ~22 conflicting `genesis_payload.toml` pins +
  3 new H-HET-2 module pins). **Requires its own Class-4 §8** (today's de-Lean §8 covered stale
  bytes only) + Veto-AI PASS. No pin recomputed here.
- **GA-9** (`enable_thinking:false` on pinned `llm_http.rs`) — separate Class-4 §8.
- The 4 reds above (het-1 carrier-test bootstrap / retired-bin read / Lean-axiom science) — fix
  during Phase 4/5 or as follow-ups; none block H-HET-2 gate/mechanism work.

## Explicit confirmations
- **No `genesis_payload.toml` pin touched. No GA-9 / `llm_http.rs` touched. No paid run. Not pushed.**
- de-Lean §8 = architect-confirmed PASS for `3fb8cb68` (stale-base); merged-byte rehash deferred to Phase-4 §8.
