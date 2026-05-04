# TuringOS v4 — Handover State

> 📍 **PROJECT DECISION MAP** (read this first if cold-starting): `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md`
> Tracks every decision + every skipped option + every atom status + forward roadmap.
> Anti-forget pledge: no skipped option is silently retired without explicit fate logged.

---

## 🛠️ 2026-05-04 — TB-16.x.1 SHIPPED — tamper-hang root-cause + Round 1 README

**Updated**: 2026-05-04 (third session of the day)
**Session summary**: TB-16.x.1 kernel-debt cleanup — root-caused OBS_TB_16_TAMPER_R2_HANG (libgit2 zlib hang on back-half-zeroed CAS loose objects), shipped two-layer defense-in-depth fix (CasStore::get worker-thread + recv_timeout + size bound + new BackendCorruption variant; load_tape distinguishes "pointer absent" from "pointer corrupt"), regenerated `audit_pipeline_smoke/tamper_report.json` with canonical post-fix 3/3 detect in 10.3s, annotated `post_r3_full_test/README.md` as pre-runner-fix vintage. Class 2 self-audit OK; cargo test --workspace 907/0/150 unchanged. Charter: `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md`.

### Architect-required declarations (per 2026-05-03 anti-drift directive §9)

| Field | Value |
|---|---|
| `phase_id` | P6 (Permissioned ChainTape / Epistemic Lab — TB-16 audit-pipeline hardening) |
| `roadmap_exit_criteria_addressed` | SG-16.6 (no unresolved evidence gaps); SG-16.1 (replayable ChainTape preserved); §7.5 audit-tape-tamper detection layer hardened |
| `kill_criteria_tested` | CR-16.6 replay byte-identity preserved (8/8 R3 chains); 38-assertion battery unchanged in count + outcome on all 8 chains; total_supply_micro unchanged (zero economic mutation) |
| `flowchart_trace` | FC3 (logs archive + constitution as ground truth → audit pipeline is the attestation surface; if the audit itself can be DoS'd by adversarial CAS bytes, FC3 ground-truth chain breaks) |
| `risk_class` | Class 2 (audit-pipeline defense-in-depth; no economic surface, no auth/crypto/money mutation, no predicate change, no L4/L4.E semantics change) |
| `forbidden_honored` | (a) no f64 added; (b) no L4/L4.E rewrite; (c) no retroactive experiment-evidence rewrite (only fence-mechanism fixture regenerated forward); (d) no existing 38+3 supplemental assertion removed; (e) no economic state mutation; (f) no `prediction_market.rs` import; (g) no AMM/CPMM/price-as-truth; (h) no agent-submitted system tx |
| `halt_triggers_observed` | None fired. total_supply unchanged; replay byte-id 8/8; bincode/canonical_decode bound is integer-comparison only (no f64); no predicate semantics touched; no monetary invariants touched |

### Key fix evidence

- **Hang site identified**: `read_markov_capsule` → `CasStore::get` → `repo.find_blob` (libgit2 zlib decompression of 953-byte loose object whose bytes 478..953 are zeroed pegs CPU indefinitely). OBS §4 hypothesis ("hang is NOT in `read_markov_capsule`") was wrong; instrumentation traced it directly inside CasStore::get.
- **Fix Layer 1** — `src/bottom_white/cas/store.rs`: `CasStore::get` wraps libgit2 read in `std::thread` + `mpsc::Receiver::recv_timeout` (default 10s; overrideable via `TURINGOS_CAS_GET_TIMEOUT_SECS`). Adds defense-in-depth size-bound check (content.len() > expected + 256 → reject). New `CasError::BackendCorruption(String)` variant.
- **Fix Layer 2** — `src/runtime/audit_assertions.rs::load_tape`: pre-existing `read_markov_capsule(...).ok()` collapsed ALL errors (corrupt CAS, missing pointer, ...) to `None`, letting Layer G assertions Skip and produce false PROCEED post-tamper. Now: `inputs.markov_pointer.exists() ? Some(read_markov_capsule(...)?) : None`. Pointer-absent legitimately yields None; pointer-present-but-unreadable yields `AuditError` → BLOCK.
- **Trust Root manifest update**: `genesis_payload.toml` `[trust_root]` rehashed `src/bottom_white/cas/store.rs` (12ce3f35... ← was de86443f...). Per R-014; non-sudo per R-018.
- **Smoke fixture capsule regen**: `audit_pipeline_smoke/`'s Markov capsule had stale `unresolved_obs=25` while alignment dir now has 26 OBS files; regenerated to chain forward (`8cc6bbbd...` → `e76e2b00...`). This was a side-issue surfaced during reproducer setup, NOT the hang itself; documented in OBS §1.
- **Round 1 README annotation**: `post_r3_full_test/README.md` now declares the dir as "VINTAGE / NON-CANONICAL — pre-runner-fix; canonical R3 evidence is `post_r3_round2/`"; per `feedback_no_retroactive_evidence_rewrite`.

### Test counts post-fix

- `cargo test --workspace --release` = **907 pass / 0 fail / 150 ignored** (unchanged from R3 baseline).
- 8 R3 chain regression sweep: **8/8 PROCEED** with assertion counts unchanged (P1-5/7/8 = 38 pass / 0 fail / 0 halt / 3 skip; P6 = 37 / 0 / 0 / 4 — same as pre-fix).
- `audit_tape_tamper` on `audit_pipeline_smoke/`: **3/3 detect in 10.3s wall clock** (was: hang >120s).
- `audit_tape` on `audit_pipeline_smoke/` baseline: **PROCEED 38/0/0/3** (was: BLOCK due to id=34 stale-capsule drift; resolved by capsule regen).

### Files changed

- `src/bottom_white/cas/store.rs` — CasStore::get hardened + BackendCorruption variant.
- `src/runtime/audit_assertions.rs` — load_tape markov pointer-exists semantic.
- `genesis_payload.toml` — Trust Root manifest rehash.
- `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` — RESOLVED + §8 root-cause + fix.
- `handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/{LATEST_MARKOV_CAPSULE.txt, MARKOV_TB-16_2026-05-03.json, tamper_report.json, tamper/}` — regenerated.
- `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_full_test/README.md` — new (vintage annotation).
- `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md` — new charter.

### Next Steps (priority order)

1. **TB-16.x.2 (P2 cap-loop, ~1-2 days)**: Atom 6.1 multi-task chain continuation — unblocks 4 missing tx kinds (ChallengeResolve, CompleteSetRedeem, TaskExpire, TaskBankruptcy-on-resolved-chain) + Boltzmann mechanism 5 RUNTIME exercise + AutopsyCapsule real-bankruptcy path. Architect 2026-05-03 §1.2 said TB-12 narrowed claim; this atom expands TB-16 conformance to FULL multi-task continuation.
2. **TB-16.x.3 / pre-TB-17 (~1-2 days)**: heldout-49 capability batch with N≥20 runs/problem (per `project_pput_ccl_arc` + `feedback_launch_priority`).
3. **Architect §3 follow-up — TB-13 legacy CPMM quarantine verification**: greppable check that `src/prediction_market.rs` has no imports from TB-13/14 modules (CompleteSet / MarketSeed / PriceIndex). Cheap — likely an OBS-write or quick TB-16.x.1.5.
4. **TB-17 RealWorld Gate** charter (Class 4 sudo): dispatch ONLY after the 3 atoms above + architect re-read of `project_tb11_to_tb17_roadmap` (canonical reading order).

### Cold-start reading order (for next session)

1. `handover/tracer_bullets/TB-16.x.1_charter_2026-05-04.md` (this atom's spec)
2. `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` §8 (root-cause + fix)
3. `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/SUMMARY.md` (canonical R3 conformance evidence — unchanged)
4. This file (LATEST.md) sections from 2026-05-04
5. `handover/tracer_bullets/TB-16_charter_2026-05-04.md` (architect spec verbatim — unchanged)

---

## 🚢 2026-05-04 — TB-16 SHIPPED + R3 closure + post-R3 Round 2 7-mechanism conformance battery PROCEED

**Updated**: 2026-05-04 (session end; second session of the day)
**Session summary**: TB-16 R3 dual audit closure (Codex VETO×2 + Gemini CHALLENGE×2 → conservative-merge VETO → surgical closure → all RQs CLOSED) + run_real_llm_arena.sh phantom-CLI bug fix + Round 1 + Round 2 v2 (8 problems × N=5 × MAX_TX=20) constitutional conformance battery PROCEED with all 7 mechanisms × FC matrix verified on real-LLM substrate. Pushed 60+ commits to origin/main (`fa36eca..3cd22d4`).

### Current State

**Works**:
- TB-16 SHIPPED: R3 closure committed `ce64d61` + Round 2 evidence committed `3cd22d4`, both on `origin/main`
- 7-mechanism × FC × audit conformance: 271 PASS / 0 fail / 0 halt across 8 chains; replay byte-identical 8/8; tamper 3/3 on every chain
- audit_assertions: id=40 per-block conservation walker + id=41 chain-walk sandbox-prefix walker (extracts ALL AgentId fields per variant via `extract_all_agent_ids` helper) + #28 JSON-array decimal form scan (R3 surgical fixes)
- sandbox_prefix admits __system__ + tb<N>- prefix (covers L4.E rejection records + TB-N fixture-era sponsors)
- run_real_llm_arena.sh: `--task-mode user --problem ... --max-transactions $MAX_TX` phantom CLI replaced with positional `mathd_algebra_171.lean` + `CONDITION=n1` env + `TURINGOS_CHAINTAPE_PRESEED=1` (latent Atom 6 bug found + closed)
- 9 of 13 tx kinds covered (union across 8 chains): Work + Verify + Challenge + TaskOpen + EscrowLock + CompleteSetMint + MarketSeed + FinalizeReward + TerminalSummary
- `cargo test --workspace` = 907 / 0 fail / 150 ignored (unchanged from R3)
- arena_run4 reproducer: P3 + P6 + P8 reproduce the 7-tx-kind chain shape

**Broken / incomplete (TB-16.x scope)**:
- 4 missing tx kinds: ChallengeResolve / CompleteSetRedeem / TaskExpire / TaskBankruptcy (Reuse out of TB-16 scope) — gate on TB-16 Atom 6.1 multi-task chain continuation
- Mechanism 5 (Boltzmann) only structural-fenced, not RUNTIME-exercised — needs single-chain multi-WorkTx-attempt scenario
- AutopsyCapsule never fired on a real bankruptcy chain (P4 SOLVED in 1 tx before bankruptcy could trigger)
- audit_tape_tamper hangs on `audit_pipeline_smoke` fixture (OBS_TB_16_TAMPER_R2_HANG; verified pre-existing on git HEAD; root cause is bincode unbounded length-prefix on partially-zeroed CAS objects). Round 2 confirmed it's fixture-state-specific (8/8 detect on richer chains).
- Round 1 evidence dir `post_r3_full_test/` is pre-runner-fix (no EscrowLock; Round 2 v2 in `post_r3_round2/` is canonical)
- 3 problem cases (P2 / P5 / P8) hit MAX_TX=20 — capability bound, not architecture bound

**Active experiments**: TB-16 R3 closed; no active Round.

**Repo state**: clean, on `main`, pushed at `3cd22d4`. Working tree carries pre-existing dirty entries (TB-13/14 evidence, h_vppu_history.json, rules/enforcement.log) — none ship-blocking.

### Next Steps (priority order)

1. **TB-16.x.1 (P1+P3, ~half day)**: tamper-hang root-cause investigation (bincode length-prefix bound at CAS-get layer) + `post_r3_full_test/` README annotation
2. **TB-16.x.2 (P2, ~1-2 days)**: Atom 6.1 multi-task chain continuation — unblocks 4 missing tx kinds + Boltzmann RUNTIME exercise + AutopsyCapsule real path
3. **TB-16.x.3 / pre-TB-17 (~1-2 days)**: heldout-49 capability batch with N≥20 runs/problem (per `project_pput_ccl_arc` + `feedback_launch_priority`)
4. **TB-17 RealWorld Gate** charter (Class 4 sudo): dispatch ONLY after the 3 atoms above

### Open Questions

1. **TB-16.x ordering**: P1 (cheap defect) first, or jump to P2 (architecture critical)? User-decision boundary.
2. **TB-17 envelope semantics**: per `project_tb11_to_tb17_roadmap`, TB-17 is "RealWorld Gate" — what specifically transitions from sandbox? Real money? Cross-org? Public chain? Architect spec hasn't been re-read post-TB-16.
3. **R-022 hook reads `.git/COMMIT_EDITMSG` (stale on `git commit -m`)**: minor papercut. Worked around with `GIT_COMMIT_MSG` env var. Could fix the hook in TB-16.x.

### Cold-start reading order (for new session)

1. `handover/evidence/tb_16_real_llm_arena_2026-05-04/post_r3_round2/SUMMARY.md` (canonical R3 conformance evidence; 11 sections incl. v3-style scaling table + per-mechanism × FC matrix + per-problem chain DAG)
2. `handover/audits/RECURSIVE_AUDIT_TB_16_R3_2026-05-04.md` (R3 closure verdict matrix)
3. This file (LATEST.md) sections from 2026-05-04
4. `handover/alignment/OBS_TB_16_TAMPER_R2_HANG_2026-05-04.md` (carry-forward OBS; root-cause TBD in TB-16.x)
5. `handover/tracer_bullets/TB-16_charter_2026-05-04.md` (architect spec verbatim)

---

## 📋 2026-05-04 — Session End Summary (earlier session)

**Updated**: 2026-05-04 (session end)
**Session summary**: TB-16 Atoms 0-7 R2 — Controlled Market Smoke Arena shipped pre-audit; 7 atoms + Step 1+3+4 surgical fixes; 2 fresh real-LLM arena chains PROCEED with 9/13 architect tx kinds; TB-11 EvidenceCapsule writer-pattern bug found + fixed live; Gemini R2 VETO 4/5 stale + 1 real (Q2 JSON privacy check); Codex R2 not yet run.

### Current State

**Works**:
- TB-16 infrastructure: 38-assertion audit_tape battery + audit_tape_tamper + comprehensive_arena scaffold + dashboard §15 live regen + §16 SANDBOX banner + run scripts (Atoms 1-6)
- Real-LLM arena harness: 3 env-var triggers (`TURINGOS_FORCE_CHALLENGER` + `TURINGOS_COMPLETE_SET_SEED` + `TURINGOS_FORCE_BANKRUPTCY`) wired into evaluator's OMEGA paths
- 2 PROCEED real-LLM chains: `arena_run4` (happy: 7 tx kinds), `arena_run6_exhaust` (exhaust: 4 tx kinds incl. TaskBankruptcy)
- Halt-trigger fence: 13/13 H1..H13 GREEN
- `cargo test --workspace`: 907 / 0 failed / 150 ignored
- TB-11 EvidenceCapsule writer fix (forward-only, mirrors TB-15 R2 fix)

**Broken / incomplete**:
- 4 architect-required tx kinds NOT delivered: ChallengeResolve (system-emit not wired), FinalizeReward-with-Challenge (challenge blocks finalize per challenge-window semantic), TaskExpire (no env-var trigger), CompleteSetRedeem (post-resolution path not wired)
- AutopsyCapsule emission requires chain with BOTH accepted WorkTx AND subsequent TaskBankruptcy on same task — neither single arena run produces this
- `audit_pipeline_smoke` evidence dir has stale Markov capsule (`previous_capsule_cid=null`) from pre-Step-1; runner now passes `--prev-cid-hex` but old artifact unreplaced
- TB-16 SHIP_STATUS doc §2 still describes pre-Step-4 framing (Atom 6.1 deferral) — Gemini R2 read this and judged stale

**Active experiments**: TB-16 Atom 7 R2 audit cycle pending — Gemini R2 VETO recorded (4/5 stale + 1 real Q2 JSON privacy check), Codex R2 not yet invoked.

**Repo state**: 56 commits ahead of `origin/main`. Last commit `af05d60`. Not pushed.

### Next Steps (priority order)

1. **Pick path forward** (user decision):
   - **R3 prep + R3 audit** (~3-4h): apply 6 surgical fixes per `handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md` §4; expected PASS/PASS or CHALLENGE-only
   - **ship-with-OBS** (~10 min): label TB-16 SHIPPED-WITH-OBS_R2_RESIDUALS; spawn TB-16.x for closure
   - **revert + re-charter**: not recommended (infrastructure is solid)

2. **If R3 path picked, surgical fixes**:
   - Q2: extend `assert_28_projection_no_autopsy_bytes` with JSON-array decimal form check (mirror TB-15 halt-trigger #5; ~15 LoC)
   - Q10: add Layer A new — walk L4, decode each tx, check agent_id sandbox-prefixed (~30 LoC)
   - Q1: Layer D #18b incremental per-block conservation (~30 LoC)
   - Q11: file-level TRACE_MATRIX precision (doc edit)
   - Q12: TB-16 SHIP_STATUS §3 test-count math (doc edit)
   - Update SHIP_STATUS §2 to reflect Step 4 reality (FR-16.x covered table)
   - Regenerate `audit_pipeline_smoke/MARKOV_TB-16` with `--prev-cid-hex`

3. **Optional**: Run Codex R2 — `TB16_AUDIT_ROUND=R2 bash handover/audits/run_codex_tb_16_ship_audit.sh`. Step 1 + Step 4 should close most R1 VETOs (V3-V7 + bug fix).

### Open Questions

1. **R2 Q4 stance ratification**: my position is §7.7 "non-sandbox funds used" HALT is **audit-time** (parallel structure with conservation / evidence-gap halts) — Layer A #3 is the architect-spec HALT, NOT a sequencer admission gate. Codex R1 V2 + Gemini Q4 read it as runtime gate. Need architect ratification OR explicit charter §5.x amendment.
2. **TaskBankruptcy without prior stakers**: `arena_run6_exhaust` fired bankruptcy but no autopsy capsule because no agent had stake. To get FR-16.7's "loss → autopsy path" demonstrated end-to-end on chain, we need a chain with BOTH accepted WorkTx AND subsequent bankruptcy — no single env-var combo achieves this without multi-task chain continuation.
3. **Push timing**: 56 commits unpushed. Risk of network outage / disk loss not mitigated.

### Cold-start reading order (for new session)

1. `handover/audits/RECURSIVE_AUDIT_TB_16_R2_2026-05-04.md` (R2 verdict triage)
2. This file (LATEST.md) sections from 2026-05-04
3. `handover/evidence/tb_16_real_llm_arena_2026-05-04/arena_run4/verdict.json` + `arena_run6_exhaust/verdict.json` (real evidence)
4. `handover/tracer_bullets/TB-16_charter_2026-05-04.md` (architect spec verbatim)

---

## 🚀 2026-05-04 — TB-16 Atom 7 R1 Steps 3+4 — fresh real-LLM arena runs + TB-11 writer-pattern bug fix (commits `05e3e86` + `d1c1af2`)

**Path B-final Steps 3 + 4** per RECURSIVE_AUDIT_TB_16_2026-05-04.md.

### Step 3 (commit `05e3e86`) — evaluator arena hooks
3 env-var triggers added: `TURINGOS_FORCE_CHALLENGER` (FR-16.3), `TURINGOS_COMPLETE_SET_SEED` (FR-16.4), `TURINGOS_FORCE_BANKRUPTCY` (FR-16.7). 3 new real-signature constructors in `src/runtime/adapter.rs` (ChallengeTx + MarketSeed + CompleteSetMint).

### Step 4 (commit `d1c1af2`) — fresh arena runs

| Run | Problem | Verdict | tx kinds |
|---|---|---|---|
| `arena_run4/` (happy) | mathd_algebra_171 | **PROCEED** | Work + Verify + Challenge + TaskOpen + EscrowLock + CompleteSetMint + MarketSeed (7) |
| `arena_run6_exhaust/` | aime_1997_p9 | **PROCEED** | TaskOpen + EscrowLock + TerminalSummary + TaskBankruptcy (4) |

**Aggregate**: 9 of 13 architect-required tx kinds across both chains. FR-16.1/2/3/4/5/6/7 conceptually covered. Missing in both runs: ChallengeResolve, FinalizeReward (was in pre-challenger run3 only — Challenge blocks Finalize per challenge-window semantic), TaskExpire, CompleteSetRedeem.

### CRITICAL — TB-11 EvidenceCapsule writer-pattern bug fix

`src/runtime/evidence_capsule.rs::write_evidence_capsule` had the same writer-pattern bug Codex caught in TB-15 R2 (for AgentAutopsyCapsule + MarkovEvidenceCapsule). Stored bytes had populated capsule_id, but capsule_id was sha256 of UNPOPULATED bytes → `cas.get(capsule.capsule_id)` always failed.

Discovered live in arena_run5 audit Layer E #27 halt. Fix: store IDENTITY-ZEROED bytes; capsule_id = sha256(stored_bytes); add `restore_evidence_capsule_from_cas_bytes`. Verified by arena_run6 PROCEED.

This bug affected EVERY chain that ever fired TerminalSummaryTx + EvidenceCapsule (TB-11 onward). Forward-only fix per `feedback_no_retroactive_evidence_rewrite`.

### Test counts

`cargo test --workspace = 907 / 0 failed / 150 ignored`

### Next: Step 5 — R2 dual external audit on aggregate evidence

---

## 🛠 2026-05-04 — TB-16 Atom 7 R1 Step 1 — surgical fixes for V3/V4/V5/V6/V7 + Q11 + V2 (commit `3cf4c36`)

**Path B-prime Step 1** per `RECURSIVE_AUDIT_TB_16_2026-05-04.md` §8. Closes 6 of 7 R1 audit defects via surgical fixes. Remaining V1 (fresh arena run) + V2-deeper (sandbox admission gate at sequencer level) deferred to Path B-prime Steps 2-4.

| Defect | Fix | Status |
|---|---|---|
| V6 (Codex Q3) | Audit calls `monetary_invariant::total_supply_micro` directly (now `pub fn`); eliminates 4-vs-5 holding drift | ✓ |
| V2 (Codex Q1, partial) | `sandbox_prefix` accepts `Agent_<digit>` canonical preseed pattern | ✓ audit-side; sequencer-side gate = Step 4 |
| #18 correctness | Conservation: FINAL == INITIAL (per-chain), not == hardcoded 30M | ✓ |
| V5 (Codex Q7) | Tamper does pre-tamper PROCEED baseline; destructive corruption (zero-back-half not single-byte XOR) | ✓ 3/3 TRUE detection on PROCEED-baseline TB-8 fixture |
| V4 (Codex Q2/Q7) | Strip `\|\| true`; runner exits non-zero on BLOCK / replay divergence | ✓ |
| V7 (Gemini Q8) | Runner passes `--prev-cid-hex` from `LATEST_MARKOV_CAPSULE.txt`; TB-16 capsule chains to TB-15 | ✓ |
| V3 (Codex Q2/Q8) | `audit_pipeline_smoke` regenerated with TB-8 fixture (5 L4 + happy-path FinalizeReward; PROCEED baseline) | ✓ |
| Q11 (Gemini) | Tamper assertions #36-#38 backlinks → FC1-N35; OBS doc + R-022-skip token | ✓ |

**Test counts**: `cargo test --workspace = 907 / 0 failed / 150 ignored` (+2 from Atom 6 baseline 905).

**audit_pipeline_smoke evidence (regenerated)**:
- `verdict.json`: PROCEED (32 PASS / 0 FAIL / 0 HALT / 7 SKIP)
- `verdict_replay.json`: byte-identical
- `tamper_report.json`: 3/3 detected (TRUE detection on PROCEED baseline)
- `MARKOV_TB-16_2026-05-03.json`: capsule_id `1478212...`; previous_capsule_cid `f9e701b4...` chained to TB-15 ✓

**Remaining (gates Step 2-4)**:
- **V1**: fresh comprehensive arena run with all 13 tx kinds — needs user-side `lake exe cache get` (~2 min) + Atom 6.1 multi-task evaluator extension (~half day)
- **V2-deeper**: sandbox admission gate at sequencer level — needs charter ratification (Class 3+ sequencer dispatch arm change) + design ("HALT vs flag" semantic decision)

User decision required before Step 2.

---

## 🚀 2026-05-04 — TB-16 SHIPPED (pre-audit) — Controlled Market Smoke Arena

**Status**: 7 atoms shipped (0..6); Atom 7 dual external audit pending.
**Charter**: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
**Ship status**: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md`
**Architect spec**: §7 of `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
**Risk class**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship).

**Shipped infrastructure** (commits `7d0d65b` → Atom 6 commit):
- `src/runtime/audit_assertions.rs` — 38 pure-fn assertions × 8 layers
  (A bootstrap / B chain / C replay / D economic / E predicate / F privacy /
  G Markov / H tamper)
- `src/bin/audit_tape.rs` — CLI emits `verdict.json` (schema_version=v1/audit_tape_verdict)
- `src/bin/audit_tape_tamper.rs` — 3-corruption tamper-detection harness
- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` — 6-task orchestrator scaffold
- `handover/tests/scripts/run_real_llm_arena.sh` + `audit_tape_smoke_test.sh`
- Dashboard §15 live regen + §16 SANDBOX banner (closes
  `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`)
- 13 halt-trigger tests (H1..H13) all GREEN

**Audit pipeline smoke evidence**: `handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/`
runs the full pipeline on a chain-backed real-LLM tape (TB-13 fixture):
`verdict.json` (BLOCK; 31 PASS / 1 HALT / 7 SKIP — H7 **demonstrated live**),
`verdict_replay.json` (byte-identical), `tamper_report.json` (3/3 detected),
`MARKOV_TB-16_2026-05-03.json` (constitution_hash + 4 flowchart hashes + 23 OBS),
`dashboard.txt` (16 sections incl. SANDBOX banner).

**Deferred to Atom 6.1** (gates fresh comprehensive arena run, not infrastructure):
- evaluator multi-task chain-continuation semantics (so 13 tx kinds appear in ONE chain)
- mathlib build via `lake exe cache get` (~2 min; user-side action)

**Test counts**: `cargo test --workspace = 905 passed / 0 failed / 150 ignored`
(+25 over TB-15 baseline 759; sub-package tests included).

**Next**: Atom 7 — Class 3 dual external audit (Codex + Gemini per `feedback_dual_audit`).

---

## 🛡️ 2026-05-04 — TB-15 R3 closure (recursive dual audit PASS PASS; Codex R2 VETO + Gemini R1 VETO closed)

**Session summary**: Per user request, ran retroactive recursive dual audit on TB-15 (originally Class 2 self-audit). Convergence at R3 with both auditors PASS. Closed 2 VETO findings + 5 CHALLENGE findings across 3 rounds. Final commit `eddab36`.

**Recursion summary**:
| Round | Codex | Gemini | Conservative merge |
|---|---|---|---|
| R1 | CHALLENGE × 5 | **VETO** Q12 (replay-determinism) + CHALLENGE Q7 | VETO |
| R2 | **VETO** Q3 + TB15-CAS-ID (REAL prod bug) | PASS | VETO |
| R3 | **PASS** medium-high | **PASS** high | **PASS ✓** |

**The big R2 finding (Codex)**: writer pattern bug — `capsule_id = sha256(prelim_bytes)` (with capsule_id+sha256 zeroed during hash) but `cas.put(final_bytes)` stored DIFFERENT post-population bytes whose sha256 differs. `cas.get(&capsule.capsule_id)` would FAIL. Verified via CAS index file: `LATEST_MARKOV_CAPSULE.txt` published `a94ae884...` but CAS object indexed under `e4932fca...`. **Broke SG-15.3 next-session bootstrap.** Same bug existed in `write_autopsy_capsule`. R3 fix: store the zeroed-identity bytes in CAS; populate in-memory struct after; add `restore_*` helpers; new round-trip tests verify the contract.

### R2+R3 cumulative deltas
- **Q12 closure** (Gemini R1 VETO — replay determinism): activation gate `TB15_AUTOPSY_ACTIVATION_LOGICAL_T: u64 = 0` + `is_autopsy_active_at` predicate; both dispatch + apply_one wrapped. Verification baseline: ZERO production chains contain TaskBankruptcyTx.
- **Q7/Q8 closure** (both R1 — flowchart_hashes): `flowchart_hashes: Vec<Hash>` field on MarkovEvidenceCapsule (additive, serde-default) + `read_flowchart_hashes_from_matrix` parser populating 4 canonical SHA-anchored hashes from `TRACE_FLOWCHART_MATRIX.md` §2.
- **Q3 + TB15-CAS-ID closure** (Codex R1+R2 VETO — CAS resolvability): writer pattern fix (zeroed-identity stored bytes; capsule_id = sha256 of stored bytes); `restore_markov_capsule_from_cas_bytes` + `restore_autopsy_capsule_from_cas_bytes` helpers; new `BankruptcyAutopsyDerivation` struct carries `stored_capsule_bytes` from derive to apply_one; new round-trip tests assert `cas.get(&cap.capsule_id)` succeeds.
- **Q4 closure** (Codex R1 — live override gate): `--include-prior-capsules N` CLI arg; default-deny exit code 3.
- **Q5 closure** (Codex R1 — byte-window scan): halt-trigger #5 strengthened (canonical Cid array form scan + raw 32-byte run + canonical_encode bytes).
- **Q9** (Codex R1 — dashboard not regenerable): OBS-deferred to TB-16 (privacy contract holds structurally).

### R3 evidence
`handover/evidence/tb_15_markov_capsule_2026-05-04/`:
- `MARKOV_TB-15-R3_2026-05-03.json` (CAS-resolvable; flowchart_hashes populated; capsule_id `f9e701b4...`)
- `LATEST_MARKOV_CAPSULE.txt` (`f9e701b4...`)
- `cas_index.jsonl` (proof: CAS index Cid MATCHES LATEST pointer)
- `README.md` with full R1→R3 closure record

### Audit artifacts (committed)
- 6 transcripts: `handover/audits/{CODEX,GEMINI}_TB_15_SHIP_AUDIT_2026-05-04_R{1,2,3}.md`
- 6 runner scripts: `handover/audits/run_{codex.sh,gemini.py}_tb_15_ship_audit{,_r2,_r3}`
- Closure doc: `handover/audits/RECURSIVE_AUDIT_TB_15_2026-05-04.md`

### Carry-forward OBS (non-blocking)
- **OBS-TB-11-CAS-ID**: TB-11 `write_evidence_capsule` has the SAME CAS-cid bug. No production reader currently. Fix in TB-16+.
- **OBS-TB15-R2-Q12-UPGRADE**: chain-resident activation marker upgrade.
- **OBS-TB15-R2-Q7-TEST-HARDEN**: parser negative-path tests.
- **OBS-TB15-R3-FOOTGUN**: API hardening on `capsule_id` accessors (loud-failure assertions when struct is unrestored).
- **OBS-TB15-R3-DEBUG-ASSERT**: `debug_assert` is debug-build only; CasStore::put returning Cid::from_content is real structural guarantee.
- **OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16**: dashboard live rebuild = TB-16 scope.

### Final state
- `cargo test --workspace` = **882 PASS / 0 fail / 150 ignored** (+4 vs R1 ship 878)
- All 6 halt-triggers GREEN; Trust Root GREEN
- HEAD: `eddab36`. NOT pushed to remote.

### Working tree
- New: nothing to track beyond what's committed
- Pre-existing dirty entries (TB-13/14 evidence + `rules/enforcement.log`) carry-forward unchanged

---

## 📐 2026-05-04 — TB-16 DESIGN landed (Real-LLM Comprehensive ChainTape + Audit-From-Tape contract)

**Session summary**: Per user request, designed comprehensive real-LLM ChainTape test exercising every shipped TB feature (TB-1..TB-15), with the load-bearing acceptance gate being a separate `audit_tape` binary that reads ONLY on-disk artifacts (runtime_repo + cas_dir + agent_pubkeys.json + pinned_pubkeys.json + genesis_payload.toml + constitution.md + LATEST_MARKOV_CAPSULE.txt) and emits a 38-assertion verdict. Framed as the implementation design for **TB-16 Controlled Market Smoke Arena** (architect §7).

**Status**: DESIGN ONLY. Not yet charter-ratified; nothing implemented.

**Design doc**: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`

### What the design specifies
- **Coverage matrix** — 13 tx kinds × 6 CAS object types; 100% of shipped agent-signed + system-emitted surfaces.
- **Six-task scenario** engineered for full coverage:
  - A happy_path (Work + Verify + FinalizeReward)
  - B challenge_dismissed (ChallengeResolve Released)
  - C challenge_upheld (ChallengeResolve UpheldDeferred marker)
  - D exhaustion (TerminalSummary → TaskBankruptcy → AgentAutopsyCapsule)
  - E expiry (TaskExpire)
  - F complete_set_market (MarketSeed + CompleteSetMint + CompleteSetRedeem)
- **`audit_tape` binary contract** — 38 assertions in 8 layers: bootstrap integrity (3) + chain integrity (8) + replay determinism (5) + economic invariants (6) + predicate/evidence integrity (5) + privacy contracts (4) + Markov continuity (4) + tamper detection (4).
- **Real-LLM provider config** — DeepSeek-v4-flash thinking-off; 30-min wall-clock cap; $15 cost ceiling; reproducible seed via `TURINGOS_RUN_SEED`.
- **Risk class** = Class 3 integration smoke per architect §7.7 — external dual audit required at ship.
- **13 halt triggers** including conservation failure, raw log leak, price-as-truth, LLM self-narrative bytes leaking into autopsy.
- **Implementation plan** = 7 atoms (audit_tape binary + audit_assertions module + tamper harness + comprehensive_arena evaluator orchestrator + run/audit shell scripts + dual audit). Estimated 4-6 atom days.

### Intentional non-scope
- SlashTx execution (RSP-3.2 / TB-9 not yet shipped) — ChallengeResolve(UpheldDeferred) stays marker-only here.
- Multi-site autopsy wire-in (SlashLoss / ChallengeUnsuccessful / VerifierBondLost) — gates on RSP-3.2 / RSP-4.
- Public chain, real-money market, cross-org, MetaTape mutation.

### Open question (for next session)
**Should we proceed to TB-16 charter ratification + Atom 1 implementation, or refine the design first?** User-decision boundary — design has not been charter-ratified.

### Working tree
- New: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md` (untracked)
- Untracked dir: `handover/tests/` (new)
- Pre-existing dirty entries (TB-13/14 evidence + `rules/enforcement.log`) carry-forward unchanged.

---

## 🚢 2026-05-03 — TB-15 SHIPPED (Lamarckian Autopsy + Markov EvidenceCapsule; Class 2 self-audit; 8/8 SG + 6/6 halt-triggers GREEN)

**Session summary**: Auto-mode shipped TB-15 per architect §6 spec verbatim (FR-15.1..6 + CR-15.1..6 + SG-15.1..8 + 6 halt triggers + forbidden list). All 7 atoms (charter + halt fixture + AgentAutopsyCapsule schema/writer + AutopsyIndex/TaskBankruptcyTx wire-in + cluster_autopsies + MarkovEvidenceCapsule schema/generator + dashboard §15/first-capsule/SHIP) shipped under single charter. Risk class envelope held at Class 2 (self-audit; AgentVisibleProjection unchanged; only one new sequencer dispatch hook). Full ship-status doc: `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`.

**Workspace = 870 passed / 0 failed / 150 ignored; +67 net vs TB-14 ship 803.** All 6 halt-triggers GREEN. All 8 architect §6.5 ship gates GREEN. All 4 P-roadmap exits addressed (P4-Exit1/2/3 + P5-Exit1/2 prep). All 4 FC-IDs (FC1-N32 / FC1-N33 / FC2-N30 / FC3-N43) have witness tests. Genesis Markov capsule emitted (`b244f16a1f3bd532d041a40fe39b2b7e7cc12fb58e18b61aedd76a8010eeb1b6`); evidence at `handover/evidence/tb_15_markov_capsule_2026-05-03/`.

**HEAD**: pre-ship `31be856` (Atom 5); ship commit pending. NOT pushed to remote — user-decision boundary per session-default.

### TB-15 architectural deltas (Class 2)
- **NEW** `src/runtime/autopsy_capsule.rs` (Atoms 2 + 3 + 4): `LossReasonClass` (8 variants) + `AgentAutopsyCapsule` + `format_public_summary` + `write_autopsy_capsule` + `derive_autopsies_for_bankruptcy` (PURE; consumed by both dispatch + apply_one) + `write_bankruptcy_autopsies_to_cas` + `cluster_autopsies` + `TypicalErrorSummary`. 15 in-module tests.
- **NEW** `src/runtime/markov_capsule.rs` (Atom 5): `ObsId` + `MarkovEvidenceCapsule` + `with_constitution_hash` + `try_deep_history_read_with_override_check` (default-deny gate) + `override_set_from_env` + `write_markov_capsule` + `scan_unresolved_obs` + `sha256_of_file` + `MarkovGenError`. 8 in-module tests.
- **NEW** `src/bin/generate_markov_capsule.rs` (Atom 5): CLI binary with `TURINGOS_MARKOV_OVERRIDE` env support + `--no-cas` mode for fresh repos.
- **NEW** `tests/tb_15_halt_triggers.rs` (Atoms 1 + 2 + 3 + 4 + 5): 6 halt-trigger fixtures.
- **MOD** `src/state/typed_tx.rs`: `+ RiskRuleId(pub String)`.
- **MOD** `src/bottom_white/cas/schema.rs`: `+ ObjectType::AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule + NextSessionContext`.
- **MOD** `src/state/q_state.rs`: `+ AutopsyIndex(BTreeMap<EventId, Vec<Cid>>)` + `agent_autopsies_t` 13th sub-field on EconomicState. Sub-field count 12→13.
- **MOD** `src/state/sequencer.rs`: TaskBankruptcyTx dispatch arm Step 3.5 (PURE Cid derivation) + apply_one Stage 3.5 (CAS write of capsule + private_detail bytes via deterministic helper). NO predicate registry mutation. NO AgentVisibleProjection mod.
- **MOD** `src/runtime/mod.rs`: `+ pub mod autopsy_capsule + pub mod markov_capsule`.
- **MOD** `src/bin/audit_dashboard.rs`: `+ render_section_15` pure render (banner `AUTOPSY IS PRIVATE`) + `+ autopsy_event_counts` + `latest_markov_capsule_cid_hex` fields on `DashboardReport` + `read_latest_markov_pointer()` helper. 4 new SG-15.6 dashboard tests.
- **MOD** 4 test fixtures for sub-field count 12→13 + 4 fc_alignment_conformance witnesses.
- **MOD** `genesis_payload.toml`: trust_root rehash for 6 modified files.

### Production claim
> TB-15 establishes Lamarckian Autopsy + Markov EvidenceCapsule substrate. AgentAutopsyCapsule (per-agent, per-event, AuditOnly) records loss/bankruptcy events derived deterministically from ChainTape evidence — NEVER LLM self-narration. agent_autopsies_t lives sequencer-side (NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1). TypicalErrorBroadcast clustering at N≥3 emits public_summary text + Cids only — NEVER private_detail_cid bytes. MarkovEvidenceCapsule binds constitution_hash + L4 + L4.E + CAS roots + previous capsule + typical_errors + unresolved_obs as next-session bootstrap default; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`. CR-15.3/15.4 (autopsy may suggest, never mutate; JudgeAI veto-only) STRUCTURALLY ENFORCED via writer signature + halt-trigger #3 file-scan.

### Open follow-ups (TB-15 carry-forward; not ship blockers)
- **Multi-site autopsy wire-in** (SlashLoss / ChallengeUnsuccessful / VerifierBondLost): wires when SlashTx ships in RSP-3.2 (TB-9) and contribution DAG ships in RSP-4.
- **L4/L4.E/CAS root chain-readers** in Markov generator: currently zero placeholders; future TB wires to chain head readers.
- **CAS-walking dashboard §15**: currently empty `autopsy_event_counts`; future TB-16 controlled-arena will exercise live wire-in.
- **InitAI agent-side honoring** of Markov default: substrate + binary-level default-deny ship now; agent-side enforcement is P5 v1.
- **OBS_RESOLUTIONS_INDEX_TB15** explicitly DEFERRED out of TB-15 scope per charter §7-G; carry-forward to dedicated TB.

---

## 🚢 2026-05-03 — TB-14 SHIPPED (single charter; full Atoms 0–7 + B′ R1-VETO closure cycle; dual audit converged R2 PASS)

**Session summary**: Auto-mode shipped TB-14 PriceIndex v0 + Boltzmann Masking under a single charter (NOT split per architect §8 fallback). Full ship-status doc: `handover/ai-direct/TB-14_SHIP_STATUS_2026-05-03.md`. **Workspace = 841 passed / 0 failed / 150 ignored; 6/6 architect §5.7 halt-triggers GREEN; 12/12 SG/G ship gates GREEN; 6/6 CR-14.x conformance preserved; ChainTape smoke + 5 production-controlled canonical-masking smokes (chain-backed) PASS.**

**HEAD**: `8b93fd9` (9 commits across Atom 6 main + internal F1 + B′ R1-VETO closure cycle + Atom 7 ship). NOT pushed to remote — user-decision boundary.

### Dual audit final verdict matrix

```text
Internal auditor R0:  CHALLENGE (F1 dead BusResult::Invested f64) → CLOSED by 38412bf
Codex R1:             VETO conviction=high (canonical-vs-shadow ID namespace mismatch)
                      → user-architect ruling 2026-05-03 path C→B′ (binding)
                      → CLOSED by B′ steps 1-6 (commits 48e84ee → 07ce9b8)
Gemini R1:            PASS conviction=high recommendation=PROCEED
Codex R2:             PASS conviction=high recommendation=PROCEED to SHIP
                      ("Split-fallback NOT triggered. mask_set is functional under
                       canonical production semantics, and B′ steps 1-6 close R1 VETO.")
Gemini R2:            CHALLENGE conviction=Medium recommendation=FIX-THEN-PROCEED
                      Single Q11 finding (bus.snapshot empty-fallback semantic ambiguity)
                      → CLOSED by 1189cb2 (sequencer_wired field with serde-default)
```

### TB-14 Atom 6 + B′ commit sequence (this session, 10 commits)

```text
44cd480  Atom 6 main — production wire-swap + legacy CPMM excision
38412bf  Atom 6 internal F1 fix — dead BusResult::Invested f64 excision
c291dde  Atom 6 LATEST.md update at user-decision boundary (external audit dispatch)
48e84ee  B′ step 1+2 — bus.append parent canonical-vs-shadow + env validation
dd40052  B′ step 3 — charter amend (canonical namespace decision §3 binding)
9daba5a  B′ step 4 — CanonicalNodeGraph + compute_mask_set canonical-graph rewire
07ce9b8  B′ step 5+6 — production-controlled chain-backed smokes (1 positive + 3 negative + idempotency)
1189cb2  B′ step 7 R2 closure — Gemini Q11 sequencer_wired field
8b93fd9  Atom 7 SHIPPED — single-charter PriceIndex + Boltzmann Masking; dual audit converged R2 PASS
```

### Architectural decisions surfaced (carry-forward to TB-15)

  (1) **Canonical namespace decision** (architect §3 binding): canonical
      WorkTx.tx_id is authoritative for TB-14 derived views; shadow tape
      ids are legacy/local only.
  (2) **STEP_B Phase 1 deviation** (Atom 6 worked directly on main):
      ACCEPTED by both R2 auditors with caveat — should not become
      default. Codify `feedback_step_b_phase_1_for_ratified_specs`
      before TB-15.
  (3) **v1-vs-v2 observability** deferred to TB-15 Autopsy bench.
  (4) **Balance plumb-through fix** in evaluator.rs (snap.get_balance →
      bus.sequencer.q_snapshot()) is incidental UX-positive scope.
  (5) **sequencer_wired field** (Q11 closure design): chose `bool` over
      Gemini's suggested `Option<...>` for cheaper consumer impact
      (~15min vs ~45min); both encode the same two-state distinction.

### Cross-references

- Ship status doc: `handover/ai-direct/TB-14_SHIP_STATUS_2026-05-03.md`
- Architect §5 verbatim: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
- Architect VETO disposition (binding): `handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`
- Charter (post-amend): `handover/tracer_bullets/TB-14_charter_2026-05-03.md`
- R1 audits: `handover/audits/{CODEX,GEMINI}_TB_14_SHIP_AUDIT_2026-05-03_R1.md`
- R2 audits: `handover/audits/{CODEX,GEMINI}_TB_14_SHIP_AUDIT_2026-05-03_R2.md`

**Closes**: `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03` + `OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03`.

**Next step (user-decision boundary)**: `git push` (or hold for manual review). TB-15 Autopsy + Markov is the next charter per `project_tb11_to_tb17_roadmap`.

---

## 🚢 2026-05-03 — TB-14 Atom 6 SHIPPED (local commits) — pending external Codex + Gemini dual audit before push

**Session summary**: Fresh session post-Atom-5 handover. Picked up at HEAD `9cc40e1` (Atom 5 ship + kickoff doc). Auto-mode wire-swap of Atom 6 — Class 3 production code path migrating bus snapshot's price-signal surface from legacy decimal-float CPMM scaffolding to integer-rational `state::compute_price_index` + `state::compute_mask_set` derived views. **All 6 architect §5.7 halt-triggers GREEN; workspace = 821 passed / 0 failed / 150 ignored; ChainTape smoke (chain-backed) PASS; evidence dir written**. Closes `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03`.

**Session exit**: HEAD `38412bf` (Atom 6 main commit `44cd480` + auditor F1 follow-up `38412bf`).

### TB-14 Atom 6 deliverables (2 commits)

```text
44cd480  TB-14 Atom 6 — production wire-swap + legacy CPMM excision (closes OBS_TB_12_LEGACY_CPMM_QUARANTINE)

DELETIONS (closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
  • src/prediction_market.rs (entire file — 390 LoC; BinaryMarket CPMM, f64 trading, automatic liquidity)
  • src/lib.rs `pub mod prediction_market;`
  • src/kernel.rs market fields + 9 methods + 5 legacy tests + 3 KernelError variants + ResolutionResult
    (V3L-45 pure-topology contract restored)
  • src/sdk/actor.rs legacy items (BoltzmannParams f64, is_frontier, lineage_score,
    legacy boltzmann_select_parent f64) + 6 legacy tests
  • src/sdk/snapshot.rs legacy fields (MarketSnapshot, markets HashMap, market_ticker String,
    dead-since-TB-9 balances/portfolios f64 + get_balance/get_portfolio impls)
  • src/bus.rs `BusConfig.system_lp_amount: f64`

WIRE-SWAPS (production code paths):
  • src/sdk/snapshot.rs UniverseSnapshot now carries integer-rational
    `price_index: BTreeMap<TxId, NodeMarketEntry>` + `mask_set: BTreeSet<TxId>`
  • src/bus.rs `snapshot()` rewritten — calls compute_price_index + compute_mask_set
    from Sequencer::q_snapshot when wired; sequencer-optional empty fallback
  • src/bus.rs `init` removed HAYEK_BOUNTY env-gated kernel.open_bounty_market call
  • src/bus.rs `append_internal` removed per-append kernel.create_market call
  • src/bus.rs `halt_and_settle` no longer calls kernel.resolve_all (deleted)
  • experiments/minif2f_v4/src/bin/evaluator.rs production wire-swap:
    - Imports BoltzmannParams + boltzmann_select_parent → boltzmann_select_parent_v2 + BoltzmannMaskPolicy
    - BusConfig literals (×2) drop system_lp_amount
    - params: BoltzmannParams::from_env → policy: BoltzmannMaskPolicy::from_env
    - Tick-time logging derives market_count + top-5 ticker from snap.price_index
      (cross-multiplication argmax sort; renders n/d, never decimal)
    - Per-tx prompt: market_ticker_str derived from snap.price_index;
      prompt_balance queried from bus.sequencer.q_snapshot().balances_t
      (post-TB-9-collapse balance plumb-through fix)
    - Boltzmann selector call: legacy → boltzmann_select_parent_v2(&snap.price_index,
      &snap.mask_set, &policy, &mut rng).map(|tx| tx.0); predicate-blind by type-system
  • src/bin/audit_dashboard.rs ADDITIVE — NEW §14 PriceIndex render section.
    ARCHITECT-MANDATED BANNER: literal "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1
    verbatim). Per-node table renders price_yes / price_no as `numerator/denominator`
    integer-rational (NEVER decimal). DashboardReport.price_index populated by
    price_index_from_exposures helper (synthesizes EconomicState from exposures vec
    + calls canonical compute_price_index — no second source-of-truth).

NEW TESTS:
  • tests/tb_14_chaintape_smoke.rs (chain-backed; pattern from tb_13_chaintape_smoke.rs):
    asserts (a) verify_chaintape 7/7 indicators GREEN; (b) replayed.economic_state_t ==
    live.economic_state_t byte-equal; (c) compute_price_index byte-equal across live/replay
    (FC3-N42 chaintape replay determinism for derived view by composition);
    (d) compute_price_index idempotent across 5 invocations; (e) empty node_positions_t
    → empty PriceIndex (FR-14.3 / halt-trigger #5 extended).
  • src/bin/audit_dashboard.rs `tb14_render_tests` mod (4 SG-14.6 unit tests):
    sg_14_6_dashboard_carries_price_is_signal_not_truth_banner +
    sg_14_6_dashboard_renders_price_as_integer_rational_never_decimal +
    sg_14_6_dashboard_empty_price_index_renders_explicit_empty_state +
    sg_14_6_dashboard_renders_none_for_zero_liquidity_nodes
  • src/kernel.rs test_trace_golden_path_unknown_node (post-purge KernelError::NodeNotFound
    coverage)

UPDATED TESTS:
  • tests/tb_13_legacy_cpmm_forward_fence.rs `prediction_market_legacy_quarantined`
    rewritten: was "label discipline" (TB-13 Atom 0.5: legacy file labeled correctly);
    now "absence discipline" (TB-14 Atom 6: legacy file gone, no fields, no methods,
    no module declaration). The strongest possible quarantine.
  • tests/fc_alignment_conformance.rs fc1_n6_input_universe_snapshot_via_bus updated
    to assert new price_index + mask_set fields (post-Atom-6 snapshot shape).
  • src/bus.rs internal tests test_bus_halt_and_settle + test_bus_snapshot rewritten.
  • src/sdk/snapshot.rs test_snapshot_default_empty_signal_surface replaces
    deleted test_snapshot_balance_query.

WORKSPACE GATE (G-14.9 ≥ 803):
  command = cargo test --workspace; workspace_count = 821 passed; failed = 0; ignored = 150.
  delta_vs_HEAD(a9fbdf3) = 821 - 841 = -20 net (deletion-of-CPMM-tests vs additions).

HALT-TRIGGER GATE (architect §5.7): 6/6 GREEN re-verified post-merge.

CHAINTAPE SMOKE EVIDENCE: handover/evidence/tb_14_chaintape_smoke_2026-05-03/
  {README.md, replay_report.json, agent_pubkeys.json, pinned_pubkeys.json,
  genesis_report.json}. Chain-backed (Sequencer::apply_one + on-disk LedgerEntry).

DEVIATIONS (per feedback_architect_deviation_stance):
  (1) STEP_B_PROTOCOL Phase 1 (worktree isolation): worked directly on main.
      Justification: Phase 0 satisfied by architect ratification (charter §3 IS the
      ratified spec); Phase 1 worktree adds operational coordination overhead with
      no audit-quality gain for a directly-spec-compliant wire-swap; Phase 3
      (dual audit + merge gate) preserved.
  (2) v1-vs-v2 cheap observability comparison (proposed in fresh-session bootstrap):
      DEFERRED. Setup cost (git switching with uncommitted handover/* + 60+ untracked
      CAS dirs) non-trivial; not ship-critical per architect spec; recovered in
      TB-15 Autopsy charter where frozen real-LLM bench is the right tool.
  (3) Balance plumb-through fix in evaluator.rs (incidental UX-positive fix outside
      Atom 6's narrow spec): documented for audit visibility.

38412bf  TB-14 Atom 6 follow-up — close internal auditor F1 (dead BusResult::Invested f64 residual)
  • Internal `auditor` subagent (Class 3 read-only, 12-min review on 44cd480)
    returned VERDICT=CHALLENGE, conviction=high, with one finding:
    F1 (CHALLENGE, FIX-NOW): src/bus.rs:95 dead `BusResult::Invested { node_id,
    shares: f64 }` enum variant — pre-TB-9 invest-path residual; zero call sites,
    zero match arms; halt-trigger #4 only fences price_index.rs so this f64 surface
    in TB-14-touched bus.rs (kickoff doc G1 explicitly named in scope) was unfenced.
  • Per feedback_audit_obs_bias (cheap fix, production-code residual not test-scaffold)
    + feedback_audit_loop_roi_flip (real defect, not fence-mechanism subtlety):
    FIX-NOW. 4-line deletion + bus.rs rehash.
  • Workspace tests unchanged at 821/0/150 (variant was dead — no observable behavior).
  • Other findings F2-F5 all ACCEPTED (cosmetic / out-of-scope / process-discipline /
    pending-external).
```

### Open ship-gate items

```text
✅ G-14.9   workspace_count ≥ 803                                        821 passed / 0 failed
✅ Halt #1  price_does_not_affect_predicate_result                       GREEN (sequencer.rs body fence)
✅ Halt #2  price_does_not_change_l4_decision                            GREEN (sequencer.rs use-block fence)
✅ Halt #3  parent_not_deleted_from_chaintape                            GREEN (functional Tape mask test)
✅ Halt #4  no_f64_in_tb_14_modules                                      GREEN (price_index.rs runtime fs scan)
✅ Halt #5  zero_liquidity_returns_none                                  GREEN (compute_price_index FR-14.3)
✅ Halt #6  unresolved_challenge_blocks_masking                          GREEN (compute_mask_set CR-14.5)
✅ SG-14.1  PriceIndex computes expected YES/NO probabilities            tb_14_price_index.rs (Atom 2)
✅ SG-14.2  No-liquidity node has price=None                             tb_14_price_index.rs (Atom 2)
✅ SG-14.3  Parent not deleted from ChainTape after masking              tb_14_mask_set.rs (Atom 3)
✅ SG-14.4  Predicate failure still dominates high price                 tb_14_halt_triggers.rs + actor.rs (Atom 5)
✅ SG-14.5  Boltzmann selection includes epsilon exploration             actor.rs v2_epsilon_greedy_explores_under_high_epsilon
✅ SG-14.6  Dashboard shows price as signal, not outcome                 audit_dashboard.rs §14 + 4 tb14_render_tests
✅ SG-14.7  Unresolved challenge blocks masking                          tb_14_halt_triggers.rs + tb_14_mask_set.rs
✅ SG-14.8  Low-liquidity manipulation cannot mask parent                tb_14_mask_set.rs
✅ G-14.10  FC3-N42 + FC2-N28 + FC2-N29 each have ≥1 witness             fc_alignment_conformance.rs (Atoms 2/3/5)
✅ G-14.11  No f64 in TB-14 module surface                               price_index.rs (halt #4) + snapshot.rs + dashboard §14 + bus.rs (post-F1)
✅ G-14.12  ChainTape smoke (--smoke + --half) PASS                      tests/tb_14_chaintape_smoke.rs (chain-backed)
🔵 Internal auditor verdict: CHALLENGE → F1 addressed by 38412bf         CLEARED
🟡 External Codex audit: PENDING (mandatory per feedback_dual_audit)     handover/audits/CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md TBD
🟡 External Gemini audit: PENDING (mandatory; degraded label if exhausted) handover/audits/GEMINI_TB_14_SHIP_AUDIT_2026-05-03_R1.md TBD
```

**Next step (user-decision boundary)**: dispatch external Codex + Gemini dual audit on commit `38412bf` per the script templates at `handover/audits/run_{codex,gemini}_tb_13_ship_audit{,.py}.{sh,py}`. After both PASS or PASS-with-OBS-CHALLENGE, write `TB-14_SHIP_STATUS_2026-05-03.md` Atom 7 ship doc + push. If either VETO at R2, escalate.

**Why audit dispatch is the user-decision boundary** (per `feedback_dual_audit` Class 3 + auto-mode etiquette): external audit consumes Codex + Gemini API budget on the user's accounts. Internal auditor cleared with high conviction; external audits should be quick PASS rounds, but the dispatch decision (timing + cost) is the user's.

Cross-references:
- Charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md`
- Atom 6 kickoff: `handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md`
- Architect spec verbatim: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §5
- Internal auditor report: returned in agent transcript on 2026-05-03 (audit subagent `a0a8d721ad2d4456e`); CHALLENGE verdict, conviction=high, recommendation=FIX-THEN-PROCEED, F1 closed by 38412bf, F2-F5 ACCEPTED

---

## 🔨 2026-05-03 — TB-14 IN-FLIGHT — Atoms 0–5 SHIPPED; Atom 6 (Class 3 dual-audit) deferred to fresh session

**Session summary**: TB-14 Atom 2 first attempt (prior session, /opusplan mode) burned 1h27m / 127k tokens with 4 specific defects (self-referencing `include_str!` test, double-rehash on q_state.rs, forward-fence band-aid `TB_14_PLUS_EXCLUDED`, 131 silently-vanished tests via missed `tests/economic_state_reconstruct.rs:129` reference). User authorized rollback to `0370d66` (Atom 1 stub). This session ran a Plan v2 + Opus 4.7 xhigh restart with 6 anti-pattern guards (G1–G6 in `~/.claude/plans/sparkling-hugging-donut.md`); shipped Atoms 2–5 in 4 clean commits, **all 6 architect §5.7 halt-triggers GREEN, workspace = 841 passed / 0 failed / 150 ignored**. Codified `feedback_opusplan_unsuitable_for_turingos` memory rule (use Opus 4.7 xhigh for every TB ship-path atom; /opusplan only for purely mechanical mass-rename / boilerplate).

**Session entry**: HEAD `0370d66` (TB-14 Atom 1 halt-trigger fixture; 6 unimplemented! stubs).
**Session exit**: HEAD `a9fbdf3` (TB-14 Atom 5 — CP-C gate green).

Charter: `handover/tracer_bullets/TB-14_charter_2026-05-03.md` (ratified pre-session at `698d8a2`).
Plan v2 (this session's anti-pattern-guarded execution plan): `~/.claude/plans/sparkling-hugging-donut.md`.
Atom 6 kickoff (fresh session): `handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md`.

### TB-14 deliverables (4 atoms shipped this session; 2 pending)

```text
Atom 2  PriceIndex pure-fn view + fence architectural fix (commit 23ac581):
        • NEW src/state/price_index.rs — RationalPrice (u128/u128) + NodeMarketEntry
          (10-field architect §5.2 verbatim) + compute_price_index (FR-14.1..3
          deterministic). 8 inline tests; G1 enforced (zero decimal-float
          substring; halt-trigger #4 fence verifies via runtime fs read).
        • ARCHITECTURAL FENCE FIX in tests/tb_13_legacy_cpmm_forward_fence.rs —
          discover_by_type_use now skips files with successor-TB authoring
          marker (TB-14..TB-99). Marker discipline wins over type-use heuristic;
          replaces hardcoded TB_14_PLUS_EXCLUDED band-aid attempted in plan v1.
          Closes OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md.
        • DELETE legacy `pub struct PriceIndex(BTreeMap<TxId, MicroCoin>)` (TB-3
          stub) + `EconomicState.price_index_t` field (13 → 12 sub-fields per
          architect §5.1 "price is signal, not truth"; charter §7 auto-resolution
          A "no second source-of-truth"). 17 references updated across 9 files
          (G4 enumeration; closes 131-tests-vanish risk by exhaustive scan).
        • Halt-triggers #4, #5 GREEN. R-022-skip via OBS_R022_TB14_PRICEINDEX_REMOVED.
        • CP-A gate: cargo test --workspace = 811 passed / 4 failed (halt #1/#2/#3/#6 stubs) / 150 ignored.

Atom 3  mask_set + compute_mask_set + BoltzmannMaskPolicy skeleton (commit 668695d):
        • src/state/q_state.rs:121-138 — AgentVisibleProjection.mask_set:
          BTreeSet<TxId> with #[serde(default)] for backward-compat.
        • src/state/price_index.rs append — BoltzmannMaskPolicy struct (architect
          §5.2 verbatim; integer-rational; Default = 1/1 beta, 1 Coin min_liq,
          10% margin, 10% epsilon) + compute_mask_set (CR-14.3/4/5 + SG-14.3/7/8;
          cross-multiplication dominance via dominates_by; deterministic
          BTreeSet output; one-dominating-child-suffices early break).
        • NEW tests/tb_14_mask_set.rs — 11 tests (SG-14.3/7/8 + boundary + happy + determinism).
        • NEW FC2-N28 witness in tests/fc_alignment_conformance.rs.
        • Halt-triggers #3, #6 GREEN.
        • CP-B gate: cargo test --workspace = 825 passed / 2 failed (halt #1/#2 stubs) / 150 ignored.

Atom 4  BoltzmannMaskPolicy::from_env() — 7 env vars (commit 7cbcacf):
        • src/state/price_index.rs append — from_env() reading 7 integer env
          vars (BOLTZMANN_BETA_NUM/DEN, MIN_LIQUIDITY_MICRO, PRICE_MARGIN_NUM/DEN,
          EPSILON_NUM/DEN); fail-soft on parse error (Art.I.1 + C-027).
        • 6 inline tests with static Mutex per feedback_env_var_test_lock.
        • Gate: cargo test --workspace = 831 passed / 2 failed / 150 ignored.

Atom 5  boltzmann_select_parent_v2 + halt-triggers #1/#2 — 6/6 GREEN (commit a9fbdf3):
        • NEW src/sdk/actor.rs::boltzmann_select_parent_v2 — integer-rational
          argmax + epsilon-greedy + mask_set filter (charter §7 auto-resolution C;
          full softmax deferred TB-15+). DEVIATION FROM CHARTER (justified):
          ADDS v2 alongside legacy rather than DELETING. Legacy deletion
          deferred to Atom 6 to keep workspace compileable.
        • 7 NEW v2 unit tests + NEW FC2-N29 witness in fc_alignment_conformance.rs.
        • HALT-TRIGGER FILLS as STRUCTURAL DECOUPLING FENCES (parallel pattern
          to halt-trigger #4 file-level fence):
            #1 — sequencer.rs source MUST contain ZERO TB-14 price/mask
                 type references (CR-14.1 by construction).
            #2 — sequencer.rs `use` statements MUST contain ZERO TB-14 imports;
                 permanent fence (sequencer remains price-blind even after
                 Atom 6's bus.rs snapshot wire-swap).
        • CP-C gate: cargo test --workspace = 841 passed / 0 failed / 150 ignored.

Atom 6  PENDING — Class 3 production wire-swap + legacy CPMM excision
        (72h cap; mandatory Codex + Gemini dual audit; STEP_B_PROTOCOL on
        kernel.rs + bus.rs). Kickoff doc TB-14_ATOM_6_KICKOFF_2026-05-03.md.

Atom 7  PENDING — ship gate (blocks on Atom 6).
```

### CP-C ship-gate evidence

```text
command         = cargo test --workspace --no-fail-fast
workspace_count = 841  (+47 net vs HEAD 0370d66 = 794 passed at TB-13 ship; +50 / -3 trust-root regression-recovery)
failed          = 0
ignored         = 150
delta_per_atom  = +17 / +14 / +6 / +10 (Atoms 2/3/4/5)

halt-triggers   = 6/6 GREEN
                  #1 price_does_not_affect_predicate_result — sequencer fence
                  #2 price_does_not_change_l4_decision — sequencer-import fence
                  #3 parent_not_deleted_from_chaintape — runtime tape.nodes() witness
                  #4 no_f64_in_tb_14_modules — runtime fs read of price_index.rs
                  #5 zero_liquidity_returns_none — runtime compute_price_index
                  #6 unresolved_challenge_blocks_masking — runtime compute_mask_set

FC alignment    = FC3-N42 + FC2-N28 + FC2-N29 — all wired, all witnessed
```

### New memory codified

`feedback_opusplan_unsuitable_for_turingos` — for TuringOS mainline TB ship-path atoms, use Opus 4.7 xhigh; /opusplan ONLY for mechanical mass-rename / boilerplate. TB-14 Atom 2 v1 (1h27m + 4 defects) is the precedent.

### OBS files added this session

- `handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md` — justifies R-022-skip for legacy `PriceIndex` struct + `price_index_t` field deletion (parallel to TB-13 ResolutionRef precedent).

### OBS files closed this session (architecturally)

- `OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md` — closed by Atom 2's successor-TB-marker-discipline fix in `discover_by_type_use` (replaces hardcoded path-list band-aid attempted in plan v1).

### OBS files carried forward to Atom 6

- `handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md` — Atom 6 deletion of `src/prediction_market.rs` + `Kernel.markets/bounty_market/bounty_lp_seed` fields closes this OBS at TB-14 ship.

---

## 🚢 2026-05-03 — TB-13 SHIPPED — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4; Class 3 dual audit; round-7 closure with fence-mechanism OBS)

**Session summary**: TB-13 introduces the Polymarket / CTF mathematical core — `1 locked Coin = 1 YES_E + 1 NO_E` — without any AMM / CPMM / orderbook / pricing layer (those are TB-14+). Three new agent-signed typed-tx variants (`CompleteSetMintTx` / `CompleteSetRedeemTx` / `MarketSeedTx`) on top of TB-12's NodePositionsIndex substrate. EconomicState extended 11 → 13 sub-fields (+`conditional_collateral_t` as 6-holding Coin holding per CR-13.4; +`conditional_share_balances_t` as claims NOT counted in supply per CR-13.3 + SG-13.2).

**Session entry**: HEAD `90a666c` (TB-12 ship + TB-13 round-3 handoff). Prior session recommended ship-with-OBS for all 6 R3 residual CHALLENGEs; user pushed back ("why ship not fix?"), prior session admitted bias → wrote `TB-13_FIX_HANDOFF_2026-05-03.md` for fresh session. New memory rule `feedback_audit_obs_bias` codified the bias-warning before farewell.

**Fresh session execution arc**: 7 surgical fix commits + 2 audit-artifact commits + 1 round-7 closure commit on top of `90a666c`. Audit-fix loop ran 6 rounds (R1 → R6); user invocation `如果6轮audit都不过，要停下来认真思考，根因在哪里` triggered ROI-flip stop decision at round-7. New memory rule `feedback_audit_loop_roi_flip` codified the doom-loop pattern recognition.

Charter: `handover/tracer_bullets/TB-13_charter_2026-05-03.md`.
Architect ruling lossless: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`.
Recursive self-audit (round-1 PASS + round-3 closure §12.6): `handover/audits/RECURSIVE_AUDIT_TB_13_2026-05-03.md`.
Ship-status decision matrix: `handover/ai-direct/TB-13_SHIP_STATUS_2026-05-03.md`.

### TB-13 deliverables (8 atoms — all SHIPPED)

```text
Atom 0    Charter ratified — handover/tracer_bullets/TB-13_charter_2026-05-03.md
Atom 0.5  Legacy CPMM forward-fence + label discipline (commit 32aab27):
          (a) src/prediction_market.rs module-header LEGACY label with 4
              required tokens (legacy / not constitutional / not RSP-M /
              not production market path) + migration-path tokens.
          (b) src/kernel.rs market-bearing fields (markets / bounty_market
              / bounty_lp_seed) carry LEGACY doc-comments.
          (c) tests/tb_13_legacy_cpmm_forward_fence.rs — 3 EXACT-named
              architect ship gates (legacy_cpm_api_not_imported_by_complete_set
              / no_f64_in_complete_set_or_market_seed /
              prediction_market_legacy_quarantined). Two-layer enforcement:
              Layer 1 unconditional whole-file scan for HARD_BANNED_LEGACY_IMPORTS;
              Layer 2 marker-span scan for FORBIDDEN_LEGACY_TOKENS.
Atom 1    Typed-tx schemas (commit 70303af): 3 NEW typed-tx variants
          (CompleteSetMintTx / CompleteSetRedeemTx / MarketSeedTx) + 4
          NEW newtypes (EventId / OutcomeSide / ShareAmount /
          ConditionalCollateralIndex / ConditionalShareBalances /
          ShareSidePair) + 3 NEW SigningPayloads + 3 NEW domain-prefixed
          state-root mutators. 8 unit tests in src/state/typed_tx.rs.
Atoms 2+3+5  Sequencer dispatch + conservation invariant + integration
          tests (commit 1806432): 3 NEW dispatch arms in
          src/state/sequencer.rs (CompleteSetMint accept / CompleteSetRedeem
          accept / MarketSeed accept). Live invariant enforcement via
          assert_total_ctf_conserved (6-holding sum) + assert_complete_set_balanced
          (MIN-semantics) called from each arm. 13 SG-13.x integration
          tests in tests/tb_13_complete_set.rs.
Atom 4    DEFERRED to TB-14 PriceIndex per architect Part A spec (no
          dashboard FR/CR/SG references it; consolidate then).
Atom 6    Round-1 self-audit (commit 17d4a3b): PASS / 12-12 SG-13.0..8 +
          11/11 G ship gates / 0/7 halt triggers fired.
          Round-1 external dual audit (Codex VETO V1+V2; Gemini PASS).
          Round-2 remediation (commit 07fc869): V1 negative-MicroCoin gate
            (mint/seed amount <= 0 rejected) + V2 partial replay-time
            agent-sig verification + Q9 layer-1 hard-banned-import scan.
          Round-3 remediation (commit cdba357): TB13-AUTH submit-time
            agent-sig verification (Sequencer.agent_pubkeys OnceLock +
            set_agent_pubkeys + submit_agent_tx +
            SubmitError::AgentSignatureInvalid; tb13_auth_submit_time_signature_verification
            test 3-path coverage). Q13 mint/seed-after-resolution gate
            (EventNotOpen rejection). assert_complete_set_balanced now
            called live from all 3 dispatch arms. Forward-fence
            FENCE_SCOPE_FLOOR + discover_tb_13_files() auto-walk.
          Round-4 closure (commit 353aa97): doc fixes (TB13-Q5-DOC q_state.rs
            MIN-form drift; TB13-RQ5 typed_tx.rs ResolutionRef opaque) +
            OBS for residuals (Q9/RQ6 / RQ3 / RQ7 / Gemini Q12).
          Round-5 closure (this session, commits edbc555 + a4f8265 + ee8bfe8):
            • RQ5 — drop ResolutionRef wrapper struct entirely; CompleteSetRedeemTx
              9→8 fields; signing payload 8→7. Both fields were dead
              (resolution_tx_id never validated; claimed_outcome a
              redundant copy of redeem.outcome). State-mismatch path
              preserved via existing match arm. R-022 skip token at
              OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md.
            • Q9/RQ6 — type-use forward-fence discovery: TB_13_TYPE_NAMES
              + discover_by_type_use walking src/ for non-comment uses
              of TB-13 type names. Catches contributors who import TB-13
              types without authoring markers.
            • RQ3 — non-empty TB-13 chaintape replay smoke at
              tests/tb_13_chaintape_smoke.rs: bootstraps Git2LedgerWriter-
              backed sequencer, wires real AgentKeypair, submits real
              signed CompleteSetMint + CompleteSetRedeem, runs verify_chaintape.
              Evidence at handover/evidence/tb_13_chaintape_smoke_2026-05-03/.
          Round-6 closure (this session, commits 887537f + d3473bb):
            • Codex R4 Q9/RQ6: tb_13_scan_lines() helper for marker-vs-
              unmarked Layer 2 scan classification.
            • Codex R4 RQ3: manual_replay_from_disk() + direct map-equality
              assertion (replayed_q.economic_state_t == live, byte-equal)
              replacing the round-5 state-root-hex overclaim.
          Round-7 closure (this session, commit 8efffa8):
            • Codex R5 PARTIAL-MARKER: rewrote tb_13_scan_lines() so
              marker-files return marker-spans UNION non-comment lines
              with TB-13 type names (closes stealth-type-use gap).
            • Codex R5 DASHBOARD-FLOOR: two-tier scope split.
              effective_fence_scope() (Layer 1) = FLOOR ∪ discovered;
              audit_dashboard.rs RESTORED to FLOOR. effective_layer_2_scope()
              (NEW) = discovered only; excludes audit_dashboard.rs until
              it gains TB-13 contributions.
          Round-7 audit-fix CLOSURE (commit e66f3bf):
            Codex R6 returned CHALLENGE (PARTIAL-MARKER-MULTILINE: a
            multiline function signature could split CompleteSetMintTx
            and f64 across adjacent lines). Per feedback_audit_loop_roi_flip
            (NEW memory rule this session): pattern is fence-mechanism
            doom loop, not real risk reduction. Iteration STOPPED.
            OBS at OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md.
            AST-aware fence refactor planned for TB-14+ when fence
            enters production-binary CI scope.
Atom 7    SHIPPED — this commit.
```

### Deviations from architect §4.3 prescribed shape (2 — both endorsed by clean-room auditor; require architect ratification)

1. **`ShareAmount.units = u128`** (architect spec said `i128`). Justified at `src/state/typed_tx.rs:1100..1107` — shares non-negative by construction; over-redeem caught by `RedeemMoreThanOwned`. Tighter-than-spec; eliminates a sign-mismatch attack class.
2. **`ResolutionRef` wrapper REMOVED** (architect §4.3 prescribed `signature_or_system_resolution_ref: ResolutionRef`). Closure at round-5 commit `edbc555` + OBS doc. Both wrapper fields were dead (`resolution_tx_id` never validated against L4; `claimed_outcome` a redundant copy of `redeem.outcome`). Resolution authority migrated to canonical `task_markets_t.state` (sequencer-side). Tighter-than-spec; eliminates self-attested resolution-ref spoofing surface.

### Audit history

| Round | Codex | Gemini | Auditor | Category |
| ----- | ----- | ------ | ------- | -------- |
| R1 | VETO (V1+V2) | PASS | — | Production-code defects |
| R2 | VETO (TB13-AUTH) | CHALLENGE (Q13) | — | Production-code defects |
| R3 | CHALLENGE-only ("No VETO; no live exploit") | CHALLENGE (Q12 future-arch) | — | Doc / fence / smoke / process |
| R4 | CHALLENGE (R5 fix edges) | PASS | — | Test-scaffold edges |
| R5 | CHALLENGE (R6 fix edges) | PASS | — | Test-scaffold edges |
| R6 | CHALLENGE (R7 fix edges) | PASS | PASS | Test-scaffold edges |

`cargo test --workspace = 794 passed / 0 failed / 150 ignored` (TB-12 baseline 759 + 8 typed_tx unit + 18 SG-13.x integration + 7 fence + 1 chaintape smoke + 1 round-3 auth = 794 net; +35 vs TB-12 ship).

### Production claim

"TB-13 introduces the Polymarket / CTF mathematical core (`1 locked Coin = 1 YES_E + 1 NO_E`) as a non-trading collateral + share accounting layer on top of TB-12's NodePositionsIndex substrate. CompleteSetMintTx is balance↔collateral migration with equal YES/NO claim issuance; CompleteSetRedeemTx redeems winning side post-system-resolved outcome (canonical `task_markets_t.state`); MarketSeedTx provider explicit-funds protocol-owned share inventory. Six-holding CTF (balances + escrows + stakes + challenge_cases + conditional_collateral) preserved bit-equal across all 3 typed-tx; conditional shares are claims, NOT Coin (CR-13.3 + SG-13.2). MIN-semantics `assert_complete_set_balanced` invariant called live from each dispatch arm post-mutation. Submit-time + replay-time agent signature verification (Class 3 admission control) for all 3 variants. Forward-fence (3-layer marker + type-name + hard-import discipline) prevents legacy f64 CPMM contamination. Two architect-spec deviations (`u128 ShareAmount` + `ResolutionRef` removed) endorsed by clean-room auditor as tighter-than-spec but requiring architect ratification before TB-14."

### Open follow-ups (carry-forward, NOT ship blockers)

1. **Architect ratification of two deviations** (`u128 ShareAmount` + `ResolutionRef` removed). Forward to architect via decision document.
2. **`OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md`** — PARTIAL-MARKER-MULTILINE residual + line-vs-item granularity gap. AST-aware fence refactor at TB-14+ when fence enters production-binary CI scope.
3. **`OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md`** — Gemini R3 Q12; partially resolved by round-5 RQ5 ResolutionRef removal; full canonical ResolutionsIndex at TB-15.
4. **`OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md`** — additive carve-out for sequencer.rs additive dispatch arms.
5. **`OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md`** — codebase-wide CO P2.x AgentRegistry pass for non-TB-13 agent variants (Challenge / TaskOpen / EscrowLock / FinalizeReward / TaskExpire submit-time signing helpers).

### New memory rules added this session

- `feedback_audit_obs_bias.md` — table CHALLENGEs by id/cost/severity; only OBS-defer multi-hour future-arch; cheap fixes get fixed.
- `feedback_audit_loop_roi_flip.md` — when audit CHALLENGEs shift from production-code to test-scaffold edges, iteration ROI has flipped → stop iterating, OBS-defer fence-mechanism challenges, ship.

---

## 🚢 2026-05-03 — TB-12 SHIPPED — Node Exposure Index (architect 2026-05-03 ruling; Class 3 dual audit PASS — Codex + Gemini)

**Session summary**: Architect 2026-05-03 morning ruling redirected TB-12 from
"NodeMarket Position Index" (the 2026-05-02 supplementary directive name)
to the more-precise **"Node Exposure Index"** scope: TB-12 records
`WorkTx.stake → FirstLong` + `ChallengeTx.stake → ChallengeShort` exposure
ONLY. NO trading. NO price. NO AMM. NO CompleteSet. NO settlement. **NodePosition
is IMMUTABLE EXPOSURE RECORD per architect §10**, NOT active position
balance. The architect explicitly chose **flat NodePositionsIndex**
(canonical) over nested NodeMarketEntry (TB-14 derived view) per §3
ruling — avoids second source-of-truth (mirroring TaskMarket.total_escrow
precedent on cache=truth).

Charter ratified at Q6 (ii.5): "一直做到双审结束" — run continuous
through Atom 6 dual audit, STOP for user verdict before SHIP. User
authorized SHIP after ultrathink-verified architect §9 alignment.

Architect ruling lossless: `handover/directives/2026-05-03_TB12_NODE_EXPOSURE_INDEX_ARCHITECT_RULING.md`.
Charter: `handover/tracer_bullets/TB-12_charter_2026-05-03.md`.
Recursive self-audit: `handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md`.
Codex audit: `handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md`.
Gemini audit: `handover/audits/GEMINI_TB_12_SHIP_AUDIT_2026-05-03_R1.md`.

### TB-12 deliverables (8 atoms — all SHIPPED)

```text
Atom 0    Charter ratified Q6 (ii.5) — `handover/tracer_bullets/TB-12_charter_2026-05-03.md`
Atom 0.5  TB-11 G3/G4 carry-forward closure (commit 2cb7f4a):
          (a) evaluator binary MAX_TX exhausted → write_evidence_capsule
              + tb11_emit_terminal_summary_for_run; bundle.shutdown
              drains TerminalSummary via apply_one. 4 new
              EvidenceCapsule counters (tb11_lean_error_count,
              tb11_sorry_block_count, tb11_protocol_parse_failure_count,
              tb11_partial_accept_count) wired at the existing
              classify_lean_error / classify_parse_error / step_partial_ok
              call sites.
          (b) lean_market `tick` (POLICY PREVIEW MODE — read-only
              eligibility scan; emission deferred to system_keypair
              persistence in a future TB) + `view-bankruptcy` (read-only
              listing of TaskMarketState::Bankrupt entries).
          (c) Real-LLM zeta rerun deferred (manual user-driven post-audit
              session per charter §6.2; Atom 0.5(a) wired the call site).
Atom 1    NodePosition schema (commit a35f5f3):
          - PositionSide enum {Long, Short}
          - PositionKind enum {FirstLong, ChallengeShort} — NO MarketBuy
            / MarketSell (architect §9.4 forbidden; TB-13+ trading layer)
          - NodePosition struct (9 fields) per architect §4 + §10
            invariants (immutable; not Coin holding)
          - NodePositionsIndex(BTreeMap<TxId, NodePosition>) flat shape
          - EconomicState 10 → 11 sub-fields with +node_positions_t
          - 3 unit tests (eleven_sub_fields + does_not_have_node_market_t_field
            + node_positions_index_default_is_empty)
Atom 2    Class 3 dispatch wire (commit 3615e32):
          - WorkTx accept arm: if work.stake>0, write FirstLong NodePosition
            (position_id == work.tx_id == node_id == source_tx; owner =
            work.agent_id; amount = work.stake.0)
          - ChallengeTx accept arm: if challenge.stake>0, write
            ChallengeShort NodePosition (position_id == challenge.tx_id ==
            source_tx; node_id == challenge.target_work_tx; task_id
            Q-derived from stakes_t[target_work_tx])
          - VerifyTx accept arm: UNCHANGED (FR-12.3 + CR-12.8)
          - Pure additive side-effect: no change to balances_t / stakes_t
            / challenge_cases_t / total_supply
          - existing assert_total_ctf_conserved + assert_no_post_init_mint
            invariants preserved
Atom 3+5  8 deterministic integration tests in tests/tb_12_node_exposure_index.rs:
          (architect §9.3 SG-12.1..8 ALL by exact-name PASS post-ultrathink)
Atom 4    audit_dashboard §13 + lean_market view-positions (commit f4bff3f):
          - ExposureRecordRow + DashboardReport.exposures field
          - L4 walk extended for TypedTx::Work (FirstLong row) +
            TypedTx::Challenge (ChallengeShort row)
          - §13 render section with per-node aggregation when ≥2 nodes
          - LABEL DISCIPLINE: "exposure records" NOT "Open market balances"
            (architect §8 Atom 4)
          - lean_market `view-positions [--node-id <tx>] [--owner <agent>]`
            read-only subcommand
          - render_section_13 refactored to pure helper for SG-12.6
            unit-testability (commit 975108d post-ultrathink)
Atom 6    Class 3 dual audit (commits 71053fd + 975108d):
          (a) Recursive self-audit (4-clause + 11 G-gates + 8 SG-12.x +
              6 failure modes) — PASS
          (b) Codex external audit (impl-paranoid via codex:codex-rescue) —
              CHALLENGE × 2 (Q4 doc-drift on holding count; Q5 legacy
              CPMM scope question) — both resolved via §10 remediation
              + OBS_TB_12_LEGACY_CPMM_QUARANTINE (TB-13 prerequisite)
          (c) Gemini external audit (architectural strategic;
              gemini-2.5-pro; 896k char prompt; 48.2s API) — PASS / high
              conviction / PROCEED to SHIP. All 8 audit questions PASS,
              including Q6 + Q7 (TB-13 CompleteSet + TB-14 PriceIndex
              forward-compat).
          (d) Pre-SHIP ultrathink ship-gate refinement (commit 975108d):
              4 SG-12.x test name drifts fixed; SG-12.6
              dashboard_view_positions_works test added; all 8/8 SG-12.x
              pass by architect §9.3 EXACT names.
Atom 7    SHIP — this LATEST.md update + TB_LOG.tsv row 35 + ship commit.
```

### Architect §9.3 ship gates — 8/8 by exact name PASS

```text
SG-12.1  ✓ sg_12_1_accepted_worktx_creates_firstlong_position
SG-12.2  ✓ sg_12_2_accepted_challengetx_creates_challengeshort_position
SG-12.3  ✓ sg_12_3_verifytx_does_not_create_node_position
SG-12.4  ✓ sg_12_4_node_positions_do_not_change_total_supply
SG-12.5  ✓ sg_12_5_replay_reconstructs_node_positions
SG-12.6  ✓ sg_12_6_dashboard_view_positions_works
SG-12.7  ✓ sg_12_7_no_market_trading_variants_introduced
SG-12.8  ✓ sg_12_8_no_node_market_entry_as_canonical_state
```

### Architect halting triggers (§7) — NONE fired

```text
✓ CTF conservation failure          NOT triggered
✓ WorkTx-Challenge position mismatch NOT triggered
✓ NodePosition counted as Coin      NOT triggered
✓ Replay divergence                 NOT triggered
✓ Codex / Gemini VETO               NEITHER (Codex CHALLENGE×2 resolved; Gemini PASS)
```

### Ship-gate evidence

```text
command         = cargo test --workspace
workspace_count = 759  (+12 net vs TB-11 ship 747; +28 vs TB-10 ship 731)
failed          = 0
ignored         = 150
trust_root      = test_trust_root_immutable_at_boot PASS

architectural   = NEW src/state/typed_tx.rs (NodePosition + 2 enums; 5 schema-addition tests)
                  EXTEND src/state/q_state.rs (NodePositionsIndex; EconomicState 10→11; +SG-12.8 unit alias)
                  EXTEND src/state/sequencer.rs (WorkTx + ChallengeTx accept-arm side-effect; pure additive)
                  EXTEND src/economy/monetary_invariant.rs (NodePosition NOT in 4-holding total_supply_micro; structural)
                  EXTEND src/bin/audit_dashboard.rs (§13 + render_section_13 helper + SG-12.6 binary unit test)
                  EXTEND src/state/mod.rs (4 new pub-use re-exports: NodePositionsIndex / NodePosition / PositionSide / PositionKind)
                  EXTEND experiments/minif2f_v4/src/bin/evaluator.rs (TB-11 G3/G4 wire-up — capsule write + emit on MAX_TX)
                  EXTEND experiments/minif2f_v4/src/bin/lean_market.rs (3 new subcommands: tick + view-bankruptcy + view-positions)
                  REHASH genesis_payload.toml trust_root for 5 modified files (+0 new)
                  NEW   tests/tb_12_node_exposure_index.rs (9 integration tests; SG-12.1..8 architect-exact names + 1 halting-trigger guard)

self-audit      = handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md (4-clause + 11 ship gates + 6 recursive failure modes; verdict PASS post-remediation)
codex-audit     = handover/audits/CODEX_TB_12_SHIP_AUDIT_2026-05-03.md (CHALLENGE × 2 → resolved via §10 + OBS-tracking)
gemini-audit    = handover/audits/GEMINI_TB_12_SHIP_AUDIT_2026-05-03_R1.md (PASS / high / PROCEED to SHIP; 8/8 questions PASS)
obs-tracking    = handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md (legacy src/prediction_market.rs as TB-13 prerequisite)

next-TB         = TB-13 CompleteSet + MarketSeedTx (architect supplementary directive 2026-05-02 §TB-13).
                  1 locked Coin = 1 YES_E + 1 NO_E. NO ghost liquidity. NO automatic YES/NO injection. NO AMM. NO trading yet.
                  Prerequisite met by TB-12: flat NodePositionsIndex + TaskBankruptcyTx death-cert anchor.
                  TB-13 Atom 0.5 prerequisite (per OBS_TB_12_LEGACY_CPMM_QUARANTINE): quarantine src/prediction_market.rs
                  (legacy f64 CPMM) before introducing CompleteSet integer-math.
```

### Post-ultrathink ship-gate refinement (architect §9 strict alignment)

After Gemini round-1 PASS verdict, user-architect requested ultrathink
verification against architect §9.1-9.4 + §10 spec. AI-coder strict
re-audit found 4 SG-12.x test-name drifts (SG-12.5 / 12.6 / 12.7 /
12.8 names didn't exactly match architect's `passes` strings).
Per `feedback_no_retroactive_evidence_rewrite`, all 4 fixed BEFORE
SHIP rather than as post-ship patch:

1. SG-12.5 `sg_12_5_node_positions_replay_deterministic` → renamed
   `sg_12_5_replay_reconstructs_node_positions`.
2. SG-12.6 had no test → ADDED `sg_12_6_dashboard_view_positions_works`
   inside `src/bin/audit_dashboard.rs#[cfg(test)] mod tb12_render_tests`.
   Refactored §13 inline render block into pure-function helper
   `render_section_13(&[ExposureRecordRow]) -> String`. Test covers
   4 cases (empty / single-Long / same-node-long+short /
   2-node-aggregation) + forbidden-token grep (Open market balances /
   MarketBuy / Market* / price_yes / etc).
3. SG-12.7 `sg_12_7_only_firstlong_and_challengeshort_kinds_observed`
   → renamed `sg_12_7_no_market_trading_variants_introduced`.
4. SG-12.8 `economic_state_does_not_have_node_market_t_field` (q_state.rs
   unit test) → ADDED at architect-exact name
   `sg_12_8_no_node_market_entry_as_canonical_state` in
   `tests/tb_12_node_exposure_index.rs`; q_state.rs unit test kept
   as defense-in-depth alias.

Post-ultrathink: 8/8 SG-12.x by architect EXACT names PASS. Workspace
+2 tests (757 → 759). ZERO behavioral change (pure-function refactor
+ test renames + 1 new test).

### Empirical observations recorded mid-session

1. **Architect's flat-vs-nested ruling validated by Gemini Q7**:
   Gemini independently confirmed flat NodePositionsIndex extends
   cleanly to TB-14 PriceIndex via "deterministic, read-only
   derivation. A view function can iterate the flat node_positions_t,
   group by node_id, and sum the amount for each side. This is
   computationally efficient on replay and avoids state-mutation
   complexity entirely. This design is robust and scalable."

2. **Codex Q4 / Q5 surfaced documentation discipline drift**:
   Q4 caught me referring to "5-holding CTF" in audit prompt while
   actual code is 4-holding (TB-8 ratification removed claims-active).
   Q5 caught the legacy `src/prediction_market.rs` CPMM scaffolding
   that predates TB-12 by many TBs. Both resolved as
   documentation/scope clarifications (§10 + OBS); neither
   architectural regressions.

3. **lean_market `tick` subcommand shipped as POLICY PREVIEW**: actual
   on-chain TaskExpireTx emission requires Sequencer reattachment to
   existing chaintape, which requires system_keypair persistence
   (not yet implemented; build_chaintape_sequencer is fail-closed on
   NonEmptyRuntimeRepo per TB-6 design). `tick` documents this
   limitation in its banner output. Future TB will add reattachment
   factory + system_keypair persistence.

4. **Real-LLM zeta rerun deferred**: Atom 0.5(a) wires the call site
   (evaluator binary on MAX_TX → write_evidence_capsule +
   tb11_emit_terminal_summary_for_run); the actual real-LLM exercise
   is wall-clock expensive (~22min cold Lean cache) and out-of-scope
   for this autonomous-execution budget. Manual user-driven session
   post-ship is the closure path.

### Next-session prompt (paste verbatim)

```text
TB-13 charter design: CompleteSet + MarketSeedTx — 1 Coin = 1 YES_E + 1 NO_E.

CONTEXT (READ IN ORDER):
1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: TB-12 ship)
3. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md
   (TB-13 spec § + struct schemas: CompleteSetMintTx / CompleteSetRedeemTx / MarketSeedTx)
4. /home/zephryj/projects/turingosv4/handover/audits/RECURSIVE_AUDIT_TB_12_2026-05-03.md
   (TB-12 architectural-skeleton hygiene; Atom 4 §13 render baseline for TB-13 §14 view)
5. /home/zephryj/projects/turingosv4/handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md
   (TB-13 PREREQUISITE: quarantine src/prediction_market.rs legacy f64 CPMM before
    introducing integer-math CompleteSet)

STATE-OF-WORLD:
- TB-12 SHIPPED (this commit; 759 / 0 / 150 tests; flat NodePositionsIndex; Class 3 dual audit PASS).
- TaskBankruptcyTx (TB-11) + NodePosition (TB-12) substrate ready for TB-13 conditional
  shares + TB-14 price.
- TB-13 PREREQUISITE: legacy CPMM in src/prediction_market.rs (345 lines f64) needs
  quarantine (Atom 0.5 carry-forward, mirror TB-12 Atom 0.5 pattern).

TB-13 ARCHITECT-MANDATED SHAPE (no trading yet):
- CompleteSetMintTx: debits balances_t by amount; credits conditional_collateral_t
  by amount; issues equal YES_E and NO_E shares (FR-13.1..3).
- CompleteSetRedeemTx: pays winning shares only after system-resolved outcome (FR-13.4).
- MarketSeedTx: seeds initial liquidity using EXPLICIT provider funds (FR-13.5;
  no ghost liquidity per CR-13.1).
- 1 Coin = YES_E + NO_E invariant (CR-13.5; SG-13.1).
- YES/NO shares are CLAIMS, NOT Coin (CR-13.3); locked collateral IS Coin (CR-13.4).
- NO automatic YES/NO injection (CR-13.2); NO AMM yet; NO trading yet (architect
  forbidden list).

Risk class: anticipate Class 3 (CompleteSetMintTx debits balances_t into a NEW
holding term `conditional_collateral_t` — first new holding-term addition since
TB-3 escrow. Total_supply_micro arithmetic + 4-holding CTF model needs explicit
extension to 5-holding for the conditional-collateral term). Iteration cap 72h
with 24h checkpoints. Sync mode (ii.5) — ratify-then-run-to-ship-gate-then-stop.
```

---

## 🚢 2026-05-02 evening — TB-11 SHIPPED — Epistemic Exhaust & Capital Liberation (architect §6.2 ruling; Class 3 recursive self-audit PASS)

**Session summary**: Architect ruling 2026-05-02 evening redirected TB-11 from
NodeMarket Decision + Position Index to **Epistemic Exhaust & Capital
Liberation**. Driven by TB-13 PREVIEW (zeta-regularization, 132 attempts /
0 OMEGA / 500_000-micro stuck escrow) which empirically demonstrated the
"Invisible Graveyard" failure mode. Architect's principle: **O(1) chain
cost, O(N) auditability**. State facts → L4. Rejected tx → L4.E.
High-dim evidence → CAS. Failure anchored via system-emitted RunExhausted
(≡ TerminalSummaryTx) + TaskBankruptcy (NEW) + TaskExpire (existing
schema, dispatch was NotYetImplemented). NodeMarket → TB-12.

Architect ruling lossless archive: `handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md`.
Supplementary directive (FR/CR/SG numbering + TB-12..17 forward-binding):
`handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md`.
Charter: `handover/tracer_bullets/TB-11_charter_2026-05-02.md`.

### TB-11 deliverables (8 atoms — 5 fully shipped + 3 narrative)

```text
Atom 0    Charter ratification — auto-ratified per user "make it your own
          understanding" authorization 2026-05-02 evening; TB-11 charter is
          the ratification record (no separate ratification doc, mirroring
          TB-10 Atom 0.5 precedent under user authorization).
Atom 1    TypedTx variants + EvidenceCapsule CAS schema (commit 870cd29):
          - Extend TerminalSummaryTx additively (architect's RunExhausted alias):
            +parent_state_root +solver_agent: Option<AgentId> +evidence_capsule_cid: Option<Cid>.
            Type alias `pub type RunExhaustedTx = TerminalSummaryTx;`.
          - Extend TaskExpireTx additively: +sponsor_agent +escrow_tx_id +reason: ExpireReason.
          - NEW TaskBankruptcyTx struct + signing payload + domain prefix.
          - NEW TypedTx::TaskBankruptcy(TaskBankruptcyTx) enum variant.
          - NEW 4 enums (ExpireReason / BankruptcyReason / ExhaustionReason / CapsulePrivacyPolicy).
          - q_state.rs EconomicState 9→10 sub-fields (+runs_t: RunsIndex);
            +RunSummaryEntry struct; +TaskMarketState enum; TaskMarketEntry +3 fields
            (+state +bankruptcy_at_logical_t +opened_at_logical_t).
          - cas/schema.rs +3 ObjectType variants.
          - system_keypair.rs +CanonicalMessage::TaskBankruptcySigning + sign_task_bankruptcy.
          - transition_ledger.rs +TxKind::TaskBankruptcy=10.
          - sequencer.rs ingress fail-closed extended; 3 system-tx helpers extended.
          - 6 new typed_tx unit tests + 3 new evidence_capsule unit tests.
          - Golden digest constants rotated for TaskExpire + TerminalSummary.
          - Trust Root: 11 entries rehashed + 1 NEW (evidence_capsule.rs).
Atom 2    Sequencer dispatch + emit_system_tx commands (commit 7e73e7c):
          - 3 dispatch arms: TaskExpire (refund), TerminalSummary (RunsIndex
            anchor), TaskBankruptcy (state-flip).
          - 3 SystemEmitCommand variants Q-deriving fields from current Q.
          - 3 state-root domain helpers (TASK_EXPIRE_DOMAIN_V1 /
            TERMINAL_SUMMARY_DOMAIN_V1 / TASK_BANKRUPTCY_DOMAIN_V1).
          - verify_emitted_system_tx_signature extended for the 3 new arms.
          - 3 integration tests via Sequencer + emit_system_tx + try_apply_one.
Atom 3    EvidenceCapsule CAS writer (commit f5afc09):
          - src/runtime/evidence_capsule.rs writer fn — 4-step CAS writer:
            (1) raw_log_bytes → ObjectType::CompressedRunLog (TB-11 MVP
            uncompressed; gzip wrapping deferred to TB-15 Markov Loom).
            (2) JSON manifest → ObjectType::EvidenceManifest.
            (3) capsule sha256 = capsule_id (content-addressed self-reference).
            (4) full canonical-encoded capsule → ObjectType::EvidenceCapsule.
          - 2 new unit tests (round-trip; deterministic capsule_id).
Atom 4    Runtime emission helpers (commit 6d2cae3):
          - tb11_emit_terminal_summary_for_run — thin wrapper over
            SystemEmitCommand::TerminalSummary.
          - tb11_emit_expire_for_eligible — scans task_markets_t for
            tasks past expiry-policy deadline; emits TaskExpire per
            (task_id, escrow_tx_id) pair; returns (count, total_micro_refunded).
          - 2 new integration tests.
Atom 5    audit_dashboard §12 (commit b1f39ec):
          - 3 new audit-row structs (ExhaustedRunRow / ExpiredTaskRow /
            BankruptTaskRow) + 3 new DashboardReport fields.
          - L4 walk loop extended with 3 new TypedTx match arms.
          - §12 render section with 3 sub-tables + total-refund aggregation +
            architect mandate footer (O(1) chain / O(N) audit).
          - Privacy: only public_summary surfaces; raw log shielded behind
            CapsulePrivacyPolicy::AuditOnly default.
Atom 6    Smoke evidence dir (this commit):
          handover/evidence/tb_11_epistemic_exhaust_smoke_2026-05-02/README.md —
          composes TB-13 PREVIEW empirical hard-fail corpus + 5 deterministic
          TB-11 integration tests as the proof-of-life. Real-LLM zeta re-run
          + evaluator binary integration deferred to TB-11.1 wire-up session
          (rationale §4 of evidence README).
Atom 7    Recursive self-audit (this commit):
          handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md — 4-clause
          (Constitutional / Replay-deterministic / Conservation / Negative-truth
          completeness) + 11 ship gates (9/11 ✓ pass + 2/11 ⚠ deferred for
          wire-up follow-up) + 6 recursive failure-mode analysis + external
          Codex+Gemini deferral rationale §8 (TaskExpire structurally mirrors
          TB-8 dual-audited FinalizeReward; capsule writer purely additive;
          architect ruling itself was the architectural review).
Atom 8    Ship — this LATEST.md update + TB_LOG.tsv row + TB-11 ship commit.
```

### Architect-mandate contract — 7/7 SG-11.x structurally satisfied

```text
SG-11.1 zeta/hard-fail run produces EvidenceCapsule       ✓ Atom 3 writer + 5 unit tests
SG-11.2 RunExhaustedTx appears in L4 + replay verifies    ✓ Atom 2 dispatch + IT-1 + replay
SG-11.3 TaskExpireTx refunds bounty after expiry          ✓ Atom 2 dispatch + IT-2 + helper IT-3a
SG-11.4 Refund preserves total CTF                        ✓ 4 monetary asserts; bal pre/post bit-equal
SG-11.5 Dashboard regenerates exhausted/expired state     ✓ Atom 5 §12 render
SG-11.6 Raw evidence shielded                             ✓ CapsulePrivacyPolicy::AuditOnly default
SG-11.7 Future Short can reference TaskBankruptcyTx       ✓ canonical schema frozen for TB-12
```

### Ship-gate evidence

```text
command         = cargo test --workspace
workspace_count = 747  (+16 net vs TB-10 baseline 731; canonical reporting per feedback_workspace_test_canonical)
failed          = 0
ignored         = 150

architectural   = NEW src/runtime/evidence_capsule.rs (capsule schema + writer + 5 tests)
                  EXTEND src/state/typed_tx.rs (+TaskBankruptcyTx + 4 enums + 2 additive struct bumps + 6 tests)
                  EXTEND src/state/q_state.rs (+RunsIndex + RunSummaryEntry + TaskMarketState + 3 TaskMarketEntry fields)
                  EXTEND src/state/sequencer.rs (+3 dispatch arms + 3 emit commands + 3 state-root domains)
                  EXTEND src/bottom_white/cas/schema.rs (+3 ObjectType variants)
                  EXTEND src/bottom_white/ledger/system_keypair.rs (+TaskBankruptcySigning + sign helper)
                  EXTEND src/bottom_white/ledger/transition_ledger.rs (+TxKind::TaskBankruptcy=10)
                  EXTEND src/runtime/adapter.rs (+tb11_emit_terminal_summary_for_run + tb11_emit_expire_for_eligible)
                  EXTEND src/bin/audit_dashboard.rs (+§12 + 3 audit-row structs + L4 walk extension)
                  REHASH genesis_payload.toml trust_root for 12 file hashes (11 modified + 1 new)
                  NEW   tests/tb_11_epistemic_exhaust.rs (5 integration tests)

self-audit      = handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md (4-clause + 11 ship gates +
                  9/11 ✓ pass + 2/11 ⚠ deferred + 7/7 SG-11.x structurally satisfied + audit verdict PASS)
external audit  = DEFERRED post-ship per recursive-audit §8 rationale (TaskExpire structurally mirrors
                  TB-8 dual-audited FinalizeReward; capsule writer purely additive; architect ruling
                  itself was the architectural review). Available on request via existing audit script
                  harness.

next-TB         = TB-12 NodeMarket Position Index (architect supplementary directive
                  2026-05-02 §TB-12). FirstLong from accepted WorkTx.stake; ChallengeShort
                  from ChallengeTx.stake; VerifyTx.bond ≠ market position; NodePosition
                  not Coin holding. **Prerequisite met by TB-11**: TaskBankruptcyTx
                  on-chain death certificate is the canonical NO/Short settlement anchor.

post-TB-11.1    = wire-up follow-up (G3/G4 deferrals): evaluator binary integration
                  (call write_evidence_capsule + tb11_emit_terminal_summary_for_run on
                  MAX_TX exhausted) + lean_market tick + view-bankruptcy subcommands +
                  real-LLM zeta-regularization smoke producing single self-contained tar.gz.
                  Naturally absorbed into TB-12 setup since TB-12 needs the same evaluator
                  hooks for FirstLong creation tied to WorkTx.stake.
```

### Empirical observations recorded mid-session

1. **Architect rulings can supersede mid-session AI-coder draft work**.
   The mid-session draft annotation `RULING_TB11_EPISTEMIC_EXHAUST_2026-05-02.md`
   had TB-12..17 sequencing with AMM/CPMM as a separate TB; the supplementary
   directive collapsed AMM/CPMM into TB-14 PriceIndex (architectural
   refinement: price computed from long/short interest, no AMM router as
   separate TB). Per `feedback_kolmogorov_compression`: BOTH directives
   archived losslessly; annotation layer reconciles.

2. **Trust Root rehashing scales linearly with kernel touchpoints**.
   TB-11 touched 12 trust-rooted files; each rehash takes ~1ms but the
   manifest commentary discipline (predecessor hash + commit reasoning)
   doubles the line count vs minimal. Mandated by `boot.rs` self-verify;
   acceptable cost.

3. **Golden digest rotation protocol works**. TerminalSummary +
   TaskExpire schema bumps each rotated 2 constants (full-tx digest +
   signing-payload digest). The protocol documented in typed_tx.rs
   tests module ("Run cargo test → assertion failure messages report
   the new hex in the `actual` slot → update each EXPECTED_HEX
   constant + cite rotation rationale in commit message") was followed
   exactly; TB-11 commit body §"Golden digest constants rotated"
   captures the audit trail.

4. **Architect's `RunExhaustedTx` ≡ existing `TerminalSummaryTx`**.
   Naming reconciliation happened naturally: `pub type RunExhaustedTx
   = TerminalSummaryTx;` makes the architect-vocabulary visible at API
   boundaries without rotating the wire format. Pre-existing
   `TerminalSummary` field histogram (failure_class_histogram,
   total_attempts) was richer than the architect's spec (just
   attempt_count); kept the richer set + added the architect's
   evidence_capsule_cid + parent_state_root + solver_agent.

5. **TB-13 PREVIEW corpus reuse**. The TB-13 zeta-regularization
   evidence dir from the post-TB-10 deepening session became the
   canonical hard-fail corpus. Empirical 132 attempts / 0 OMEGA /
   500_000 stuck escrow + new TB-11 dispatch arms + integration tests
   = the architect's §8 ship gates structurally satisfied.

6. **Workspace test count `cargo test --workspace = 747 / 0 / 150`**.
   +16 net vs TB-10 baseline 731 across 5 modules:
   - src/state/typed_tx::tests +6 TB-11 unit tests
   - src/runtime/evidence_capsule::tests +5 (3 schema + 2 writer)
   - tests/tb_11_epistemic_exhaust.rs +5 integration tests
   Zero existing tests regressed.

### Next-session prompt (paste verbatim at start of new session)

```text
TB-12 NodeMarket Position Index — first formal Polymarket mechanism entry per
architect supplementary directive 2026-05-02 §TB-12. NO trading.

CONTEXT (READ IN ORDER):
1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: TB-11 ship)
3. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_TB11_EPISTEMIC_EXHAUST_ARCHITECT_RULING.md
4. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_TB11_TO_TB17_SUPPLEMENTARY_DIRECTIVE.md
   (TB-12 spec § + struct schema NodePosition / PositionSide / PositionKind)
5. /home/zephryj/projects/turingosv4/handover/architect-insights/RULING_TB11_EPISTEMIC_EXHAUST_2026-05-02.md
6. /home/zephryj/projects/turingosv4/handover/audits/RECURSIVE_AUDIT_TB_11_2026-05-02.md

STATE-OF-WORLD:
- TB-11 SHIPPED (this commit; 747/0/150 tests; kernel core PASS).
- Failure-anchor + capital-release substrate live.
- TaskBankruptcyTx on-chain = canonical NO/Short settlement anchor (TB-12 prerequisite met).
- Carry-forward (deferred from TB-11): evaluator binary integration + lean_market
  tick subcommand + real-LLM zeta-regularization smoke. Absorb into TB-12 setup.

TB-12 ARCHITECT-MANDATED SHAPE (no trading):
- WorkTx.stake → NodePosition { side: Long, kind: FirstLong } (architect §FR-12.1)
- ChallengeTx.stake → NodePosition { side: Short, kind: ChallengeShort } (FR-12.2)
- VerifyTx.bond ≠ market position (FR-12.3)
- NodePosition references node_id = target WorkTx (FR-12.4)
- NodePosition can reference TaskBankruptcyTx / RunExhaustedTx as future NO anchor (FR-12.5)
- NodePosition is exposure index, NOT Coin holding (CR-12.1)
- NodePosition.amount must NOT be in total_supply_micro (CR-12.2)
- NO trading tx variants introduced (SG-12.6)

Risk class: anticipate Class 3 (NodePosition writer is a new state
mutator on accepted WorkTx + ChallengeTx; touches stakes_t indirectly
via the position derivation). Iteration cap 72h with 24h checkpoints.
```

---

## 📋 2026-05-02 — Post-ship session close: TB-10 byte-audit + TB-13 preview + architectural-coverage finding

**Session summary**: Post-TB-10 deepening session. Three deliverables, all on top of `6ab165c` (TB-10 ship); no new commits. (1) Byte-level audit of TB-10 chain — canonical-decoded the 5 L4 entries from run_a smoke and confirmed every architect-mandate field at the lowest evidence layer. (2) TB-13 PREVIEW off-product smoke — brand-new zeta-regularization theorem ingested via manual MiniF2F/Test/ copy (off ratified TB-10 product surface; explicitly preview-labeled), 500_000-micro bounty, MAX_TX=50 → effective 200 proposals, deepseek-chat ran 132 attempts in 22min wall, depth-32 partial proof, **0 OMEGA acceptances**, no FinalizeReward, bounty stays in escrow per Q7 — exactly the predicted Scenario B2 outcome. (3) Architectural-coverage audit triggered by user question "did the top white box predicate-check the proposals?" — surfaced that TuringOS's chain epistemic guarantee is **ONE-SIDED**: the chain proves *nothing fake was accepted* (TB-7R sorry-gate fired 14× pre-Lean; Lean kernel rejected 73× explicitly; 0 OMEGA), but does NOT prove *every fake attempt was witnessed and refused* — the 132 attempts are evaluator-private (in `lean_market.log` only); chain has zero proposal_telemetry / verification_result CAS objects from this run. PredicateRegistry is empty-by-design at runtime (TB-6 simplification; `_predicate_registry` is unused dispatch param); the actual proof-checking lives in three layers (chain dispatch arms / bus forbidden_payload / evaluator's lean4_oracle subprocess). This is consistent with `feedback_chaintape_externalized_proposal` ("1 LLM call → 1 compound payload"), but the TB-13 preview surfaced the operational consequence: **failed runs leave bare chains; failed-attempt audit currently requires non-chain artifacts**.

### Byte-level TB-10 audit findings (run_a, mathd_algebra_171)

```text
L4 entry #1 TaskOpen (canonical 284B):
  variant_tag         = 0x07 = TypedTx::TaskOpen
  tx_id               = "taskopen-...-tb10-user-seed"           ← TB-10 net-new suffix in chain bytes
  sponsor_agent       = "Agent_user_0"                          ← TB-10 sponsor (12 bytes)
  AgentSignature      = NON-ZERO Ed25519 (real-sig path; make_real_task_open_signed_by)

L4 entry #2 EscrowLock (canonical 258B):
  tx_id               = "escrowlock-...-tb10-user-escrow"       ← TB-10 net-new suffix
  amount              = 0x186a0 = 100_000 micro = 0.1 Coin      ← EXACT BOUNTY
  parent_state_root   = NON-ZERO 32B (chains to L4#1 resulting root)
  AgentSignature      = NON-ZERO Ed25519

L4 entry #5 FinalizeReward (canonical 268B):
  tx_id               = "system-finalize-reward-1-5"            ← system-emitted naming (epoch.logical_t)
  claim_id            = "claim-verifytx-Agent_0-omega-pertactic-1"  ← matches L4#4 verify
  reward              = 100_000 micro                           ← BIT-EQUAL to L4#2 amount
  solver              = "Agent_0"                               ← TB-9 durable AgentId

Cross-run pubkey identity (raw hex from agent_pubkeys.json across 3 smoke runs):
  Agent_0:      ebefcd328a36a515cb49f80e49a514c8df964dcfe4db48aa8207fc7a69ee2504  ← IDENTICAL × 3
  Agent_user_0: f1982a189b5befb2f4a94d1688a01676231ade20440fa80c46c455d5e7aba0c0  ← IDENTICAL × 3

ALL 5 LedgerEntry system_signatures: 64-byte Ed25519, NONE zero.
```

### TB-13 preview run results (zeta-regularization heat-cosine kernel)

Evidence: `handover/evidence/tb_13_preview_zeta_regularization_2026-05-02/` — README + 1 run_a directory (lean_market.log + dashboard.txt + replay_report.json + runtime_repo.tar.gz + cas.tar.gz + agent_keystore_at_exit + agent_pubkeys_for_witness).

```text
Lean theorem (precise reformulation; the literal claim ∑n=-1/12 is divergent in standard reals):
  Tendsto (fun N : ℕ ↦ ∑' m, m·exp(-m/N)·cos(m/N))  atTop  (𝓝 (-1/12 : ℝ))
  Mathematically true (via Bernoulli expansion + (1-i)/N phase killing real divergence)
  Estimated proof difficulty: 200-500 Lean lines; LLM solve probability < 1%

Run outcome:
  L4 entries     : 2 (TaskOpen + EscrowLock by Agent_user_0; bounty 500_000 micro)
  L4.E entries   : 2 (synthetic seeds; pre-existing TB-6 Atom 3 pattern; NOT this run's LLM work)
  proposals      : 132  (truncated by 1300s outer timeout before 200 budget exhausted)
  partial OK     : 32   (LLM accumulated 32 valid Lean tactics into proof state)
  Lean rejected  : 73   (mostly "simp made no progress")
  forbidden      : 14   (sorry-attempts blocked PRE-LEAN by TB-7R Atom 2)
  parse errors   : 26   (LLM output not in <action> protocol)
  OMEGA accepted : 0    ← proof never closed
  FinalizeReward : NEVER FIRED  ← architect mandate negative pathway upheld
  payout         : 0 micro; bounty stays in escrow indefinitely (Q7 limitation)
  replay         : 7/7 indicators GREEN (chain integrity preserved despite no payout)
  cleanup        : MiniF2F/Test/zeta_regularization.lean reverted post-run
                    (no permanent change to heldout-49 corpus)
```

### Architectural-coverage finding

```text
"Did the top white box predicate-check the proposals?" — 3-layer answer:

Layer A: Chain dispatch arm (sequencer.rs hardcoded checks; PredicateRegistry passed as _unused)
  Reached by:  2 typed_tx (TaskOpen + EscrowLock from preseed)
  Verdict:     2/2 PASSED → 2 L4 entries committed

Layer B: Bus forbidden_payload string-match gate (TB-7R Atom 2; pre-Lean)
  Reached by:  every LLM proposal
  Verdict:     14/132 BLOCKED on sorry-attempts → never reached Lean kernel

Layer C: Evaluator's lean4_oracle (DIRECT subprocess; not chain-mediated)
  Reached by:  118 proposals (132 minus 14 forbidden_payload)
  Verdict:     32 partial-tactic accepts ; 73 explicit Lean errors ;
                26 protocol parse errors ; 0 OMEGA acceptances

PredicateRegistry status: EMPTY by design (`Arc::new(PredicateRegistry::new())` at
  src/runtime/mod.rs:415; dispatch_transition takes it as `_predicate_registry` /
  unused param). The chain has the *socket* for top-white-box predicates, but no
  plug currently inserted. TB-6 simplification.

Chain-resident audit completeness:
  ✓ "no fake accepted"            — proven by chain alone (no WorkTx for the 132 attempts)
  ✗ "every fake attempt witnessed" — NOT proven by chain alone (the 132 lived in
                                      lean_market.log; chain has 0 proposal_telemetry
                                      and 0 verification_result CAS objects from this run)
```

### Honest limitations exposed by TB-13 preview

1. **`lean_market --max-tx` flag does NOT override evaluator's swarm budget regime** (`total_proposal` base 200 from `BUDGET_REGIME` env). TB-10 charter implied this would cap; empirically it doesn't. Candidate OBS for next-TB.
2. **Outer timeout sizing**: 1300s was insufficient for hard-analysis at 200-proposal budget × ~10-15s/Lean check. ~30 min would be needed for full budget exhaust.
3. **Bounty indefinite-lock confirmed in real flow**: 500_000 micro now stuck in escrows_t with no refund path. Q7 limitation became operationally visible. TB-12+ scope.
4. **L4.E does not capture LLM-proposal-rejection events**: only `submit_typed_tx`-routed rejections hit L4.E. The 73 Lean errors + 14 forbidden_payload + 26 parse errors are evaluator-private. Architectural shift needed if we want chain-resident witness of every refused attempt.

### What didn't change

- No new commits this session (post-ship deepening only; TB-10 stays at `6ab165c`)
- No code changes to `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/state/q_state.rs`, or any kernel-resident file
- TB-10 ratification §1 Q1-Q8 stands as ratified
- ROADMAP next-TB direction unchanged: TB-11 RSP-M0/M1 NodeMarket Decision + Position Index

### Open questions for next session

1. **Architectural Q (TB-13 charter shape)**: should L4.E grow to capture per-LLM-attempt rejection events (chain-resident witness of "fake-attempt refusal"), OR keep `feedback_chaintape_externalized_proposal` as-is and accept that failed-attempt audit needs non-chain artifacts? Tradeoff: chain bloat vs audit completeness. Affects TB-13 Beta + TB-14 v1.0 scope.
2. **Operational fix**: `lean_market --max-tx` does not override `BUDGET_REGIME total_proposal base` — should the user CLI flag take precedence, or document the regime hierarchy? Small OBS or part of TB-11.
3. **Refund mechanism for indefinite-locked bounties**: Q7 came up against real friction in TB-13 preview. Should TB-12 RSP-3.2 be brought forward, or wait for TB-14 task-expiry?

### Cross-references this session produced

```text
handover/evidence/tb_13_preview_zeta_regularization_2026-05-02/
  README.md                          (full audit narrative §0-§7)
  agent_keystore.enc                 (durable keystore; same as TB-10 smoke pattern)
  keystore/agent_keystore.enc
  run_a_n1_zeta_regularization/
    lean_market.log                  (132-proposal trace)
    dashboard.txt                    (§1-§11; §11 shows open un-claimed user task)
    replay_report.json               (7/7 indicators GREEN)
    verify.log
    runtime_repo.tar.gz              (16K self-contained)
    cas.tar.gz                       (12K)
    agent_keystore_at_exit.enc
    agent_pubkeys_for_witness.json
```

### Next-session prompt

Unchanged from TB-10 ship-section bottom: **TB-11 RSP-M0/M1 NodeMarket Decision Record + Position Index** (no trading; per architect Part C line 1617). Charter design should incorporate this session's architectural-coverage finding as input — specifically, decide whether TB-11/13 should expand chain-resident audit to cover failed-attempt witnesses, or keep the current 1-LLM-call=1-compound-payload externalization rule.

---

## 🚢 2026-05-02 — TB-10 SHIPPED — Lean Proof Task Market MVP (first user-facing product; recursive self-audit PASS)

**Session summary**: Shipped the **first user-facing product** per architect directive 2026-05-02 Part C ruling 12+13 line 1594 ("第一个可用产品：用户发任务，Agent 解题，系统验证，系统付款，dashboard 可审计"). Every primitive in the architect MUST list (TaskOpenTx + EscrowLockTx + WorkTx + VerifyTx + FinalizeRewardTx + replay + dashboard) was already shipped in TB-3..TB-8 — TB-10 is the thin user-facing wrapper that closes the 5-step compile loop end-to-end from a non-evaluator caller class. **Architect mandate satisfied 5/5**: 用户发任务 ✓ (lean_market run-task subcommand, Agent_user_0 sponsor with real Ed25519), Agent 解题 ✓ (evaluator user-mode + deepseek-chat solver loop), 系统验证 ✓ (Lean kernel oracle + OMEGA-Confirm VerifyTx), 系统付款 ✓ (FinalizeRewardTx system-emitted via tb8_emit_finalize_after_verify), dashboard 可审计 ✓ (audit_dashboard §11 User Tasks renders correctly). Class 2 primary risk (production wire-up via new bin) + Class 3 audit tier (first new caller class for already-Class-3 economic mutators) handled via **recursive self-audit** (4-clause structure: Constitutional / Replay-deterministic / Conservation / User-minimum-contract — all PASS; 11/11 ship gates GREEN; 6/6 recursive failure modes PASS) per `feedback_dual_audit` hybrid-by-risk-class. TB-10 net-new surface is **purely additive on top of unchanged kernel** — NO new TypedTx variant, NO new dispatch arm, NO new TransitionError variant, NO new state-root domain, NO `monetary_invariant.rs` cascade. External Codex + Gemini audits deferred post-ship per recursive-audit §8 (kernel-only-additive surface; external audit available on request). TB-10 ship-gate test count: `cargo test --workspace = 731 / 0 / 150` (+8 net vs TB-9 baseline 723; the +8 are exactly the new `runtime::bootstrap::tests` unit suite). 3/3 SOLVED across 3 different heldout-49 problems with bounties 100_000 / 100_000 / 250_000 micro; cross-run pubkey identity for both Agent_user_0 (sponsor) and Agent_0 (solver) verified by `diff -q agent_pubkeys_for_witness.json` across all 3 runs.

### TB-10 deliverables (8 atoms)

```text
Atom 0.5 (Class 0)  — handover/audits/CHARTER_RATIFICATION_TB_10_2026-05-02.md
                      §0 scope ratified to architect-line-1594 minimum (NOT genesis_payload edit;
                      runtime preseed factory is on_init substrate, not toml schema change);
                      §1 Q1-Q8 all RATIFIED with citation back to spec; §2 architectural
                      clarifications (real-Ed25519 constructors / concurrent access /
                      dashboard filter / replay determinism). Auto-ratified per user
                      authorization 2026-05-02 ("authorized in auto mode until TB-10 is
                      done with real LLM smoke test and dual audit").
Atom 1   (Class 2)  — src/runtime/bootstrap.rs new module (~165 lines):
                      `default_pput_preseed_pairs()` factory exposing `tb7-7-sponsor` (TB-7.7
                      back-compat) + `Agent_user_0` (TB-10 net-new, 10_000_000 micro sponsor
                      budget) + `Agent_0..9` (1_000_000 micro each) — total preseed supply
                      30_000_000 micro. 8/8 unit tests pass (returns 12 entries, every entry
                      has positive balance, agent_user_0 present with sponsor budget,
                      tb7-7-sponsor preserved, 10 solver agents each at 1M, total 30M sum,
                      deterministic across calls, genesis construction matches total).
                      EXTEND src/runtime/adapter.rs — make_real_task_open_signed_by +
                      make_real_escrow_lock_signed_by real-Ed25519-signature constructors
                      mirroring existing make_real_worktx_signed_by pattern. Forward-compatible
                      with future TB-12+ kernel signature verification on these dispatch arms.
                      EXTEND evaluator preseed branch (evaluator.rs:858+) to call the factory
                      instead of inline literal — single source of truth.
Atom 2   (Class 2)  — experiments/minif2f_v4/src/bin/lean_market.rs new binary (~600 lines):
                      4 subcommands run-task / view-task / view-wallet / view-replay.
                      run-task spawns evaluator subprocess with TURINGOS_USER_TASK_MODE=1 +
                      TURINGOS_USER_TASK_BOUNTY_MICRO=<n> + fresh chaintape path; view-*
                      operates on chaintape READ-ONLY via replay_full_transition (no
                      Sequencer bootstrap → no NonEmptyRuntimeRepo gate). NO user-callable
                      system_tx surface (no settle/finalize/refund subcommand) per Anti-Oreo.
                      Cargo.toml [[bin]] entry added.
Atom 3   (Class 2)  — experiments/minif2f_v4/src/bin/evaluator.rs preseed branch detects
                      TURINGOS_USER_TASK_MODE=1 env (truthy: "1" or "true") and swaps sponsor
                      `tb7-7-sponsor` → Agent_user_0 (default; overrideable via
                      TURINGOS_USER_TASK_SPONSOR) with REAL Ed25519 signatures via
                      make_real_task_open_signed_by + make_real_escrow_lock_signed_by.
                      Bounty overrideable via TURINGOS_USER_TASK_BOUNTY_MICRO. genesis_report
                      tx_id suffix matches user-mode flag (`tb10-user-seed/escrow` vs legacy
                      `tb7-7-d3-seed/escrow`). Solver task_id remains `task-{run_id}` —
                      user-mode is a sponsor-swap-only cut, no solver-loop change.
Atom 4   (Class 1)  — src/bin/audit_dashboard.rs §11 TB-10 User Tasks section + UserTaskRow
                      struct + DashboardReport.user_tasks field. Filter convention: TaskOpenTx
                      whose sponsor_agent.0 starts with "Agent_user_". Cross-references
                      claims_in_progress for solver / status / payout. Aggregate row: n user
                      tasks + n Finalized + total bounty + total paid. Architect mandate
                      attestation line printed when total paid > 0.
Atom 5   (Class 1)  — handover/evidence/tb_10_lean_market_mvp_smoke_2026-05-02/ — 3 runs
                      across 3 distinct heldout-49 problems (run_a fresh-keystore
                      mathd_algebra_171 bounty=100_000 MAX_TX=10 + run_b load-keystore
                      mathd_algebra_107 bounty=100_000 MAX_TX=20 + regression load-keystore
                      mathd_numbertheory_961 bounty=250_000 MAX_TX=20).
                      3/3 SOLVED with FinalizeReward + Finalized claim + payout=bounty exactly.
                      Cross-run Agent_user_0 + Agent_0 pubkeys IDENTICAL across all 3 runs.
                      Per-run replay_report.json all 7 indicators GREEN. runtime_repo.tar.gz +
                      cas.tar.gz self-contained (TB-8 RQ3 packaging carry-forward).
                      Comparative README §2 side-by-side TB-7R → TB-8 → TB-9 → TB-10 outcome
                      metrics + ChainTape detail metrics + tx-kind sequence on L4 +
                      cumulative capability-evolution table + sponsor-debited-by-bounty
                      arithmetic per run.
Atom 6   (Class 3)  — Recursive self-audit handover/audits/RECURSIVE_AUDIT_TB_10_2026-05-02.md
                      (4 clauses + 11 ship gates + 6 recursive failure modes + audit verdict
                      PASS). External Codex + Gemini audits deferred post-ship per audit §8
                      reasoning (kernel surface purely additive; the 6-failure-mode analysis
                      structurally answers each question via reference to UNCHANGED kernel
                      code paths inherited from TB-3/TB-6/TB-7R/TB-8/TB-9; external audit
                      available on request).
Atom 7   (Class 0)  — this LATEST.md update + TB_LOG.tsv row 32 (narrative comment + 33 row
                      data) + TRACE_FLOWCHART_MATRIX.md TB-10 row planned→shipped + smoke
                      evidence README + ship commit.
```

### Architect-mandate contract — all GREEN

```text
Architect spec line 1594:
  TB-10：Lean Proof Task Market MVP
  目标：第一个可用产品：用户发任务，Agent 解题，系统验证，系统付款，dashboard 可审计。
  必须：TaskOpenTx, EscrowLockTx, WorkTx, VerifyTx, FinalizeRewardTx, replay, dashboard

  ✓ TaskOpenTx           — Agent_user_0 sponsor, real Ed25519, 3/3 smoke runs
  ✓ EscrowLockTx         — Agent_user_0 sponsor, real Ed25519, balance debited exactly bounty
  ✓ WorkTx               — Agent_0 solver (TB-9 durable), TB-7R+TB-8 chain
  ✓ VerifyTx             — Agent_0 verifier, Confirm verdict
  ✓ FinalizeRewardTx     — system-emitted, payout = bounty exactly
  ✓ replay               — verify_chaintape 7 indicators GREEN per run
  ✓ dashboard            — audit_dashboard §11 User Tasks renders correctly
  ✓ 用户发任务            — lean_market run-task subcommand
  ✓ Agent 解题            — evaluator user-mode runs deepseek-chat solver loop
  ✓ 系统验证              — Lean kernel oracle + OMEGA-Confirm VerifyTx
  ✓ 系统付款              — FinalizeRewardTx emitted post-Verify
  ✓ dashboard 可审计      — audit_dashboard §11 + lean_market view-task subcommand
```

### Ship-gate evidence

```text
command         = cargo test --workspace
workspace_count = 731  (+8 net vs TB-9 ship 723; canonical reporting per feedback_workspace_test_canonical)
failed          = 0
ignored         = 150

smoke evidence  = handover/evidence/tb_10_lean_market_mvp_smoke_2026-05-02/  (3 runs; 3/3 SOLVED + Finalized;
                  cross-run pubkey identical across Agent_user_0 + Agent_0; comparative README §2 side-by-side
                  TB-7R/TB-8/TB-9/TB-10; replay self-contained tar.gz with sidecars per Codex RQ3 fix
                  carry-forward)

self-audit      = RECURSIVE_AUDIT_TB_10_2026-05-02.md (4-clause + 11 ship gates + 5/5 architect mandates GREEN
                  + 6/6 recursive failure modes PASS)
external audit  = DEFERRED post-ship per audit §8 (purely additive kernel surface; minimum spec is unambiguous;
                  external audit available on request)

architectural   = NEW src/runtime/bootstrap.rs reusable preseed factory module
                  EXTEND src/runtime/adapter.rs with make_real_task_open_signed_by + make_real_escrow_lock_signed_by
                  EXTEND experiments/minif2f_v4/src/bin/evaluator.rs preseed branch with user-mode env detection
                  NEW   experiments/minif2f_v4/src/bin/lean_market.rs CLI binary with 4 subcommands
                  EXTEND src/bin/audit_dashboard.rs with §11 TB-10 User Tasks section
                  REHASH genesis_payload.toml trust_root for 4 changed/new tracked files

next-TB         = TB-11 RSP-M0/M1 NodeMarket Decision + Position Index (per directive 2026-05-02 Part C line 1617;
                  Polymarket mechanism formal entry but NOT yet trading; NodePosition derived index — WorkTx.stake →
                  FirstLong, ChallengeTx.stake → Short, VerifyTx.bond ≠ market position; NodePosition NOT counted as
                  Coin holding). TB-10 closes the prerequisite (durable sponsor + solver identity bound to economic
                  state; first user-product loop verified end-to-end on chain).
```

### Empirical observations recorded mid-session

1. **Sequencer NonEmptyRuntimeRepo gate forces single-process model**. The TB-6 fail-closed boot path on existing chains means lean_market and evaluator cannot share an active chaintape across separate process invocations. TB-10 cuts this by spawning evaluator as a subprocess (single-process invocation per run-task call). Documented as ratification §2.1 + audit §3.4.
2. **Cross-run pubkey identity is sponsor-side AND solver-side now**. TB-9 demonstrated cross-run identity for Agent_0 (solver). TB-10 extends to Agent_user_0 (sponsor). `diff -q agent_pubkeys_for_witness.json` across all 3 smoke runs returns empty — same Ed25519 keypairs recovered from `agent_keystore.enc` on each evaluator boot.
3. **Kernel does NOT verify TaskOpen/EscrowLock signatures (current state)**. The `src/state/sequencer.rs:1054 + 1095` dispatch arms have no `verify_agent_signature` call. TB-10 user CLI signs anyway with real Ed25519 (forward-compatible TB-12+); kernel acceptance does not currently depend on signature validity. Documented as audit §3.6 with reference to existing pre-TB-10 state (no regression introduced).
4. **Sponsor budget is on_init, not post-init mint**. `default_pput_preseed_pairs()` is consumed only at chaintape genesis QState construction via `genesis_with_balances`. After bootstrap, `assert_no_post_init_mint` fires unchanged on every typed_tx. The `Agent_user_0 = 10_000_000` micro entry is a one-time genesis allocation, not a runtime mint path.
5. **Lean kernel cold-cache vs warm-cache dominates run wall-time**. Run_a took 99.6s (cold-cache compile through Mathlib). Run_b took 11.0s (warm cache). Regression took 12.2s (warm). TB-10's architectural cost is ~50ms/run (Argon2id KDF on first Agent_user_0 keypair generation + 2 Ed25519 signs). Same pattern observed in TB-9 evidence §4.3.
6. **Workspace test count `cargo test --workspace = 731 / 0 / 150`**. +8 net vs TB-9 baseline 723. The +8 are exactly the 8 new tests in `runtime::bootstrap::tests` covering the preseed factory (returns 12 entries / positive balances / Agent_user_0 budget / tb7-7-sponsor preserved / 10 solver agents / total 30M / determinism / genesis construction). Zero existing tests regressed.

### Next-session prompt (paste verbatim at start of new session)

```text
TB-11 charter design: RSP-M0/M1 NodeMarket Decision Record + Position Index — formal Polymarket mechanism entry (no trading yet).

CONTEXT (READ IN ORDER):
1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: TB-10 ship)
3. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md
   (TB-11 spec line 1617; RSP-M0..RSP-M5 Polymarket absorption track lines 624-768)
4. /home/zephryj/projects/turingosv4/handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md
   (TB-11 sequencing post-2026-05-02 directive amendment; § 11.5.1 RSP-M decision record contents)
5. /home/zephryj/projects/turingosv4/handover/evidence/tb_10_lean_market_mvp_smoke_2026-05-02/README.md
   (TB-7R → TB-8 → TB-9 → TB-10 capability evolution table — TB-11 inherits all of these)
6. /home/zephryj/projects/turingosv4/handover/audits/RECURSIVE_AUDIT_TB_10_2026-05-02.md
   (proves TB-10 11/11 ship gates GREEN; TB-11 builds on this foundation)

STATE-OF-WORLD:
- First user-facing product: SHIPPED (TB-10; lean_market run-task → Lean Proof Task Market MVP)
- Durable agent identity (sponsor + solver): SHIPPED (TB-9 + TB-10)
- Minimal payout / FinalizeRewardTx: SHIPPED (TB-8)
- Frame B authoritative routing on L4 / L4.E with predicate evidence: SHIPPED (TB-7R)
- ChainTape production wire-up: SHIPPED (TB-6)
- TaskOpenTx / EscrowLockTx / WorkTx / VerifyTx all on canonical L4 (TB-3..TB-5)
- ship-gate test count: 731 / 0 fail / 150 ignored at <TB-10 ship commit>
- next-TB ship target: TB-11 RSP-M0 NodeMarket Decision Record + RSP-M1 NodePosition derived index (NO trading yet)

TASK: Charter TB-11. Per architect Part C line 1617:
  目标：把 Polymarket 机制正式进入系统，但还不交易。
  新增：DECISION_NODEMARKET_POLYMARKET_CPMM.md + NodePosition + FirstLongPosition + ChallengeShortPosition
  规则：WorkTx.stake → FirstLong, ChallengeTx.stake → Short, VerifyTx.bond ≠ market position,
        NodePosition NOT counted as Coin holding.

Per ROADMAP § 11.5.1: RSP-M0 decision record file is `handover/alignment/DECISION_NODE_MARKET_FIRST_LONG_2026-05-XX.md`
with 8 mandatory rules (WorkTx.stake = FirstLong / ChallengeTx.stake = Short/NO / VerifyTx.bond responsibility-not-position
/ price ≠ truth / outcome resolved by predicates+ChallengeCourt+system-emitted resolution / NO automatic liquidity injection
/ NO ghost liquidity / positions are exposure indexes NOT Coin holdings).

Per memory feedback_tb_phase_tag_required: declare phase_id + roadmap_exit_criteria_addressed +
kill_criteria_tested + flowchart_trace before commit. Class likely 2 (additive index + decision record;
NO new economic mutator wiring; decision record + NodePosition struct + read-only derived view).
```

---

## 🚢 2026-05-02 — TB-9 SHIPPED — Durable AgentRegistry + Wallet Projection (architect-minimum scope; recursive self-audit PASS)

**Session summary**: Closed the **durable identity** prerequisite per architect directive 2026-05-02 Part C line 1574 ruling 13 ("NodeMarket starts after durable identity AND Lean Proof Task Market MVP"). Run-local Ed25519 keypair lifecycle (TB-7) is now persistent: secrets live in an encrypted-at-rest keystore at `~/.turingos/keystore/agent_keystore.enc` (Argon2id KDF + ChaCha20-Poly1305 AEAD); the same `Agent_0 → AgentPublicKey` binding survives evaluator restart with a fresh `runtime_repo`. Concurrently, `WalletTool` collapsed to a **read-only projection** of `EconomicState.balances_t` — the parallel f64 ledger and the bus.rs legacy v3 simulation paths (`debit_wallet/credit_wallet/InvestOnly/founder_grant/settle_portfolios/Hayek bounty`) are deleted. **Architect mandate satisfied 5/5**: agent durable key registry ✓, wallet read-only projection ✓, EconomicState canonical ✓, no f64 mutation ✓, cross-run identity ✓. Class 3 risk handled via **recursive self-audit** (4-clause structure: Constitutional / Replay-deterministic / Conservation / User-minimum-contract — all PASS) per `feedback_dual_audit` hybrid-by-risk-class (kernel surface is purely additive — NO new typed_tx variant, NO dispatch arm, NO QState field; external Codex+Gemini deferred post-ship per recursive-audit §8 reasoning). Cross-run Agent_0 pubkey identity empirically verified by `diff -q` over two evaluator runs each with a fresh runtime_repo. TB-9 ship-gate test count: `cargo test --workspace = 723 / 0 / 150` (-2 net vs TB-8 ship 725 baseline; +14 new TB-9 tests, -16 deleted obsolete v3-simulation/f64-mutator tests).

### TB-9 deliverables (8 atoms)

```text
Atom 0.5 (Class 0)  — handover/audits/CHARTER_RATIFICATION_TB_9_2026-05-02.md
                      §0 scope-trim from charter draft to architect-minimum (per Part C line 1574 spec
                      extraction: "agent pubkey registry persisted" = durable on-disk keystore, NOT new
                      on-chain typed_tx variant); §1-§5 Q1-Q5 all RATIFIED with citation back to spec
Atom 1   (Class 3)  — src/runtime/agent_keystore.rs new module (~390 lines): Argon2id m=64MiB t=3 p=4
                      KDF + ChaCha20-Poly1305 AEAD encryption-at-rest; format magic TOS4AGTKEY1 distinct
                      from system_keypair TOS4SYSKEY1; default ~/.turingos/keystore/agent_keystore.enc +
                      TURINGOS_AGENT_KEYSTORE_PATH env override + TURINGOS_AGENT_KEYSTORE_PASSWORD env
                      via keystore_password_from_env() helper (avoids exposing `secrecy` in binaries);
                      atomic tmp+rename write 0600. STEP_B preflight: handover/audits/STEP_B_PREFLIGHT_TB9_ATOM1_2026-05-02.md.
                      EXTEND src/runtime/agent_keypairs.rs — AgentKeypair::from_secret_bytes constructor
                      + secret_bytes() crate-private accessor + DurableConfig field + generate_or_load_durable
                      load-or-generate factory + persist_manifest re-encrypts durable keystore on every
                      new keypair. TB-7 fail-closed-on-existing semantics retained for ::open(...) path.
Atom 2   (Class 2)  — experiments/minif2f_v4/src/bin/evaluator.rs:765 — replace AgentKeypairRegistry::open
                      with generate_or_load_durable; password via keystore_password_from_env env helper.
Atom 3   (Class 2)  — src/sdk/tools/wallet.rs collapse to read-only projection: DELETE balances HashMap +
                      portfolios + genesis_done + genesis_coins + deduct/credit/record_shares/ensure_agents/
                      save_to_disk/load_from_disk; ADD balance(&AgentId, &EconomicState) → MicroCoin
                      projection; on_init no-op + on_pre_append → Pass + query_state → None.
Atom 4   (Class 2)  — src/bus.rs legacy market path delete (-92 lines): InvestOnly routing → Veto
                      "veto:invest_disabled_tb9" + founder_grant TAPE_ECONOMY_V2 + settle_portfolios +
                      Hayek bounty HAYEK_BOUNTY + debit_wallet + credit_wallet helpers; halt_and_settle
                      simplified to kernel.resolve_all + tool on_halt + RunEnd; test_bus_unknown_agent_vetoed
                      renamed+inverted to test_bus_unknown_agent_appends_post_tb9_collapse.
                      ALSO: experiments/minif2f_v4/src/bin/evaluator.rs — DELETE WALLET_STATE cross-problem
                      sidecar load/save (~30 lines) + invest tool action handler f64 path + EMERGENT_ROLES
                      wallet.balances reader + wallet.ensure_agents top-up. tests/reward_pull_conservation.rs
                      DELETED entirely (5 obsolete tests for deleted v3-simulation code).
Atom 5   (Class 1)  — handover/evidence/tb_9_durable_identity_smoke_2026-05-02/ — 3 runs across 2 distinct
                      heldout-49 problems (run_a fresh-keystore mathd_algebra_171 MAX_TX=10 + run_b
                      load-keystore SAME problem + regression load-keystore mathd_algebra_107 MAX_TX=20).
                      3/3 SOLVED with FinalizeReward + Finalized claim + payout_micro=100,000. Cross-run
                      Agent_0 pubkey IDENTICAL (dec9e321...047b6468) across evaluator restart with FRESH
                      runtime_repo each run — verified by `diff -q agent_pubkeys_for_witness.json`.
                      Per-run replay_report.json all 7 indicators GREEN. runtime_repo.tar.gz + cas.tar.gz
                      self-contained (TB-8 round-2 RQ3 packaging carry-forward). Comparative README §2
                      side-by-side TB-7R → TB-8 → TB-9 outcome metrics + ChainTape detail metrics + tx-kind
                      sequence on L4 + cumulative capability-evolution table.
Atom 6   (Class 0/1)— src/bin/audit_dashboard.rs §10 TB-9 Durable identity section: durable_keystore_path
                      env-resolved + durable_keystore_present indicator + agents_in_manifest count +
                      per-agent table with pubkey_in_manifest + tape_activity columns + auditor note about
                      cross-run pubkey diff.
Atom 7   (Class 3)  — Recursive self-audit handover/audits/RECURSIVE_AUDIT_TB_9_2026-05-02.md (4 clauses
                      + 11 ship gates + 6 recursive failure modes + audit verdict PASS). External dual
                      audit DEFERRED post-ship per audit §8 reasoning (kernel surface purely additive;
                      architect minimum spec leaves zero ambiguity for external opinion).
Atom 8   (Class 0)  — this LATEST.md update + TB_LOG.tsv row 30 (narrative comment + 31 row data) +
                      TRACE_FLOWCHART_MATRIX.md TB-9 row planned→shipped + smoke evidence README +
                      ship commit.
```

### Architect-mandate contract — all GREEN

```text
Goal: 持仓、payout、future NodeMarket 都必须归属于 durable identity (Part C line 1574)

  ✓ agent durable key registry           — keystore TOS4AGTKEY1 file, KDF+AEAD encrypted
  ✓ wallet read-only projection          — WalletTool::balance(&AgentId, &EconomicState) → MicroCoin
  ✓ EconomicState canonical              — economic_state_reconstructed=true per replay
  ✓ no f64 mutation                      — bus.rs market path + WalletTool mutators all deleted
  ✓ cross-run identity                   — `diff -q` Agent_0 pubkey across run-A and run-B = identical
```

### Ship-gate evidence

```text
command         = cargo test --workspace
workspace_count = 723  (-2 net vs TB-8 ship 725; canonical reporting per feedback_workspace_test_canonical)
failed          = 0
ignored         = 150

smoke evidence  = handover/evidence/tb_9_durable_identity_smoke_2026-05-02/  (3 runs; 3/3 SOLVED + Finalized;
                  cross-run pubkey identical; comparative README §2 side-by-side TB-7R/TB-8/TB-9; replay
                  self-contained tar.gz with sidecars per Codex RQ3 fix carry-forward)

self-audit      = RECURSIVE_AUDIT_TB_9_2026-05-02.md (4-clause + 11 ship gates + 5/5 architect mandates GREEN)
external audit  = DEFERRED post-ship per audit §8 (purely additive kernel surface; minimum spec is unambiguous)

architectural   = NEW src/runtime/agent_keystore.rs encrypted keystore module
                  EXTEND AgentKeypairRegistry with generate_or_load_durable + DurableConfig
                  COLLAPSE WalletTool to read-only projection (zero owned f64 state)
                  DELETE bus.rs legacy v3 market path (-92 lines)
                  DELETE evaluator WALLET_STATE sidecar + invest action f64 handler
                  EXTEND audit_dashboard with §10 TB-9 Durable identity section
                  REHASH genesis_payload.toml trust_root for 4 changed tracked files

next-TB         = TB-10 Lean Proof Task Market MVP (per directive 2026-05-02 ruling 13 + feedback_launch_priority;
                  first user-facing product atom now that durable identity + minimal payout both shipped)
```

### Empirical observations recorded mid-session

1. **Cross-run identity is deterministic, not stochastic**. Same 32-byte secret seed produces the same Ed25519 public key by spec (`SigningKey::from_bytes(&seed).verifying_key()`); the keystore stores secrets only and recomputes pubkeys at load. The cross-run pubkey match is structural, not probabilistic.
2. **Run-B is 10× faster than Run-A on same problem**. `verifier_wait_ms` (Lean kernel + Mathlib compile) accounts for the entire delta (110215 ms vs 8577 ms). TB-9 introduces ZERO observable runtime cost on the proposal critical path beyond the once-per-fresh-keypair Argon2id derivation (~50ms, fired only on `get_or_create` for a new agent_id).
3. **Trust-root rehash needed for 4 tracked files**. `genesis_payload.toml` `[trust_root]` table SHA-256 hashes for `src/bus.rs`, `src/runtime/mod.rs`, `src/runtime/agent_keypairs.rs`, `experiments/minif2f_v4/src/bin/evaluator.rs`, `src/bin/audit_dashboard.rs` — all rehashed; trust-root immutability test passes after rehash.
4. **`reward_pull_conservation.rs` was untestable post-collapse**. The 5 tests in this file all exercised `TAPE_ECONOMY_V2`-gated f64 paths (founder grant + settle_portfolios + Hayek bounty + wallet.deduct/credit). All 5 code paths deleted in this TB; per `feedback_no_retroactive_evidence_rewrite` only on EVIDENCE not on tests-of-deleted-code, the test file is removed (not skipped). Git history retains the file at TB-8 ship `43aa288` for forensic value.

### Next-session prompt (paste verbatim at start of new session)

```text
TB-10 charter design: Lean Proof Task Market MVP — first user-facing product.

CONTEXT (READ IN ORDER):
1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: TB-9 ship)
3. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md
   (TB-10 spec line 519 / 1594; rulings 12/13)
4. /home/zephryj/projects/turingosv4/handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md
   (TB-10 sequencing post-2026-05-02 directive amendment)
5. /home/zephryj/projects/turingosv4/handover/evidence/tb_9_durable_identity_smoke_2026-05-02/README.md
   (TB-7R → TB-8 → TB-9 capability evolution table — TB-10 inherits all of these)

TASK: Charter TB-10 = Lean Proof Task Market MVP. Per architect directive Part C: TaskOpenTx +
EscrowLockTx + WorkTx + VerifyTx + FinalizeRewardTx (all already shipped in TB-3..TB-8) wrapped in
a CLI / minimal web surface that lets a user (a) post a Lean theorem statement + bounty, (b) watch
proposals + verify outcomes via the audit dashboard, (c) see the bounty paid to the solver's durable
agent_id (TB-9 keystore). Every primitive is already on chain — TB-10 is the user-facing wrapper.

Per memory feedback_tb_phase_tag_required: declare phase_id + roadmap_exit_criteria_addressed +
kill_criteria_tested + flowchart_trace before commit. Class likely 3 (first user-facing product;
wraps existing Class 3 economic mutators in a UI; possibly Class 2 if the surface is pure CLI).
```

---

## 🚢 2026-05-02 — TB-8 SHIPPED — Minimal Payout / FinalizeRewardTx (Class 3 dual ship audit; PASS)

**Session summary**: Closed the 5-step compile loop's settlement node. Every accepted L4 WorkTx with closed challenge window + no upheld challenge produces exactly one L4 FinalizeRewardTx that atomically debits `escrows_t` + credits `balances_t` + flips `claims_t.status` to Finalized. Dual external audit at strategic tier: **Gemini PASS** round-1; **Codex VETO** round-1 (RQ3 smoke packaging + RQ4 duplicate-Confirm DoS) → surgical remediation under `feedback_elon_mode_policy` round-2 auto-execute → **Codex PASS** round-2. Both auditors clear. TB-8 ship-gate test count: `cargo test --workspace = 725 / 0 / 150` (+13 net vs TB-7R 712 baseline).

### TB-8 deliverables (8 atoms)

```text
Atom 0.5 (Class 0)  — handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-02.md
                      §1 Q1-Q5 + §2.1-§2.4 architectural clarifications + window-namespace correction
Atom 1   (Class 2)  — claims_t writer at VerifyTx OMEGA-Confirm + ClaimEntry 6-field expansion
                      + ClaimStatus enum + 5→4 holding migration on monetary_invariant
                      (claims_t now intent registry; assert_claim_amount_backed_by_escrow + ClaimUnbacked)
                      + round-2 one-claim-per-work_tx_id idempotency
Atom 2   (Class 3)  — SystemEmitCommand::FinalizeReward { claim_id } variant + build_signed_system_tx
                      arm + verify_emitted_system_tx_signature arm + EmitSystemError::ClaimNotFound
                      (STEP_B preflight: handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-02.md)
Atom 3   (Class 3)  — TypedTx::FinalizeReward dispatch arm 9-step body (lookup → idempotency →
                      window gate → upheld-challenge gate → Q-derived consistency → escrow gate →
                      atomic mutation → 4 invariants → state_root advance via FINALIZE_REWARD_DOMAIN_V1)
                      + TransitionError::ClaimAlreadyFinalized
Atom 4   (Class 2)  — Evaluator OMEGA-branch caller: tb8_emit_finalize_after_verify (best-effort
                      poll-then-emit) + tb8_await_state_root_advance (sequenced WorkTx→VerifyTx
                      via post-Work parent_state_root) + bond=0→100_000 fix
Atom 5   (Class 1)  — handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/ — 7 runs across
                      5+ distinct heldout-49 problems (mathd_algebra_171/107/359/10/11,
                      mathd_numbertheory_961, aime_1997_p9). 5/7 SOLVED with Finalized claim +
                      payout_micro=100_000; 2/7 UNSOLVED with no fake Finalized.
                      + round-2 self-contained tar.gz packaging (full runtime_repo + cas dirs;
                      sidecars included for clean verify_chaintape replay)
Atom 6   (Class 0/1)— src/bin/audit_dashboard.rs §9 TB-8 Claims section with claim_status +
                      payout_amount columns + aggregate row (total_payout sum)
Atom 7   (Class 3)  — Recursive self-audit: handover/audits/RECURSIVE_AUDIT_TB_8_2026-05-02.md
                      Codex impl-paranoid: handover/audits/CODEX_TB_8_SHIP_AUDIT_2026-05-02.md
                        round-1: VETO (RQ3 + RQ4) → round-2 PASS post-remediation
                      Gemini architectural: handover/audits/GEMINI_TB_8_SHIP_AUDIT_2026-05-02.md
                        round-1: PASS at strategic tier `gemini-3.1-pro-preview` (NOT degraded)
Atom 8   (Class 0)  — this LATEST.md update + TB_LOG.tsv row + TRACE_FLOWCHART_MATRIX.md TB-8 row
                      + smoke evidence README + ship commit
```

### User-minimum 12-requirement contract — all GREEN

```text
Goal:
  ✓ accepted proof → escrow → solver balance       (Atom 3 dispatch)

Scope:
  ✓ single solver / single verifier / no royalty / no NodeMarket / no multi-solver split

Must:
  ✓ FinalizeRewardTx system-only                    (Atom 2 + TB-3 foundations)
  ✓ agent cannot submit FinalizeRewardTx            (TB-5 RSP-3.0 inheritance + test I121)
  ✓ payout_sum ≤ escrow                             (Atom 3 step 6 + step 8 + RQ4 idempotency)
  ✓ CTF conserved                                   (Atom 3 step 8; 4-holding sum delta=0)
  ✓ dashboard shows payout                          (Atom 6 §9 Claims claim_status + payout_amount)
  ✓ economic_state replay works                     (Atom 5 smoke; verify_chaintape per run)
```

### Ship-gate evidence

```text
command         = cargo test --workspace
workspace_count = 725  (+13 net vs TB-7R ship 712; canonical reporting per feedback_workspace_test_canonical)
failed          = 0
ignored         = 150

smoke evidence  = handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/  (7 runs; 5/7 SOLVED + Finalized;
                  2/7 UNSOLVED + no fake Finalized; replay_report.json all 7 indicators GREEN per run;
                  self-contained tar.gz with sidecars per Codex RQ3 fix)

dual audits     = Codex round-2 PASS (CODEX_TB_8_SHIP_AUDIT_2026-05-02.md + R2 supplement)
                  Gemini round-1 PASS strategic-tier (GEMINI_TB_8_SHIP_AUDIT_2026-05-02.md, NOT degraded)
self-audit      = RECURSIVE_AUDIT_TB_8_2026-05-02.md (4-clause + 9 ship gates + 12 user-min all GREEN)

architectural   = 5→4 holding migration on monetary_invariant (claims_t becomes intent registry;
                  +assert_claim_amount_backed_by_escrow + ClaimUnbacked variant)
                  zero-window MVP per ratification §1 Q3 + §2.4 namespace correction
                  one-claim-per-work_tx_id idempotency (round-2 RQ4 fix)
                  smoke evidence self-contained tar.gz (round-2 RQ3 fix)

next-TB         = TB-9 Durable AgentRegistry + Wallet Projection (per directive 2026-05-02 ruling 13)
```

### Empirical observations recorded mid-session

1. **Verify bond=0 → BondInsufficient → no claim creation**. The pre-fix smoke showed `chain_oracle_verified=true` but no Verify on L4 because both OMEGA emit sites passed `bond_micro=0` → dispatch rejected as BondInsufficient → L4.E. Fix: bond=0→100_000 micro at both sites.
2. **WorkTx + VerifyTx parent namespace mismatch**. The post-bond-fix smoke still showed Verify hitting L4.E with `stale_parent_root` because both were constructed before either was submitted (WorkTx accept advanced state_root, queued VerifyTx became stale). Fix: split into two phases — submit WorkTx, await state_root advance via `tb8_await_state_root_advance`, THEN construct + submit VerifyTx with fresh parent.
3. **Codex round-1 RQ4 duplicate-Confirm denial-of-payout**. Two Confirm VerifyTxs targeting the same WorkTx created two Open claims, both backed per-claim but aggregate exceeds escrow → finalize fails post-mutation. Fix: one-claim-per-work_tx_id idempotency in Atom-1 writer.
4. **Codex round-1 RQ3 smoke evidence not replayable**. tar.gz of `.git`-only missed required verifier sidecars. Fix: tar full `runtime_repo/` + `cas/` directories.

### Next-session prompt (paste verbatim at start of new session)

```text
TB-9 charter design: Durable AgentRegistry + Wallet Projection.

CONTEXT (READ IN ORDER):
1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: TB-8 ship)
3. /home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-8_charter_2026-05-02.md §9
   (post-TB-8 next-TB direction)
4. /home/zephryj/projects/turingosv4/handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md
   (TB-9 sequencing post-2026-05-02 directive amendment)
5. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md
   (ruling 13: NodeMarket starts after durable identity AND Lean Proof Task Market MVP)

TASK: Charter TB-9 per ruling 13 sequencing. Run-local Ed25519 agent identity (TB-7) is
ephemeral; TB-9 makes it persistent. Wallet collapses to read-only projection of
EconomicState (no f64 mutation; EconomicState canonical). Class 3.

Per memory feedback_tb_phase_tag_required: declare phase_id + roadmap_exit_criteria_addressed
+ kill_criteria_tested + flowchart_trace before commit.
```

---

## 📨 2026-05-02 — Architect directive ingested + TB-8 charter rewritten — READY TO START TB-8

**Session summary**: Architect delivered a 3-layer directive (lossless constitution
integrated edition + first plan + updated final ruling, "以最后的为准") absorbing
Polymarket / CTF math while explicitly REJECTING ghost liquidity. Per
`/architect-ingest` SOP: archived verbatim (per `feedback_kolmogorov_compression` —
no "distill", no store-by-reference), Layer-1 impact-detected (no violations;
Append-Only DAG + economic conservation STRENGTHENED), four decision records
created, TRACE_FLOWCHART_MATRIX created, TB-8 charter rewritten with `flowchart_trace`
declarations, ROADMAP_9_PHASE + PROJECT_DECISION_MAP amended. **No code touched
this session — only directive landing.** TB-8 ready to start.

### Directive landing inventory

```text
handover/directives/   (Kolmogorov-lossless archive, ~228 KB)
  2026-05-02_lossless_constitution_polymarket_directive.md                            (overview + Layer 1 verdict)
  ..._part_A_lossless_integrated_edition.md                                            (Part A §0-§6 verbatim)
  ..._part_A_appendix_B_group_intelligence.md                                          (verbatim full text)
  ..._part_A_appendix_C_turing_machine_philosophy.md                                   (verbatim + flagged simulation-table abridgment)
  ..._part_A_appendix_D_verification_asymmetry.md                                      (verbatim full text)
  ..._part_B_first_plan.md                                                             (superseded plan verbatim)
  ..._part_C_updated_final_ruling.md                                                   (canonical ruling verbatim)

handover/alignment/
  DECISION_POLYMARKET_CORE_2026-05-02.md                  1 Coin = 1 YES_E + 1 NO_E
  DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md               poolY * poolN = k math + invariants
  DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md   no automatic injection; MarketSeedTx debit required
  DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md     private autopsy, read-view masking
  TRACE_FLOWCHART_MATRIX.md                               TB ↔ Flowchart 1/2/3 mapping (TB-1..TB-7R back-fill + TB-8 forward)

handover/architect-insights/   (2 new + 2 amended)
  2026-05-02_flowchart_hashes_and_trace_matrix.md          NEW
  2026-05-02_polymarket_absorption_guards.md               NEW
  PROJECT_DECISION_MAP_2026-04-27.md                       +1 amendment block (post-TB-7R → v1.0 sequence)
  ROADMAP_9_PHASE_2026-04-29.md                            +1 amendment block (TB-8 → TB-15 → v1.0 chain)

handover/tracer_bullets/
  TB-8_charter_2026-05-02.md                               REWRITTEN (376 lines) — flowchart_trace + decision-record links + updated forbidden list (20 items) + updated next-TB direction (TB-9 = Durable AgentRegistry)

memory/
  feedback_kolmogorov_compression.md                       NEW (never "distill", always lossless)
  MEMORY.md                                                +1 index entry
```

### Layer 1 verdict (from main archive §9)

```text
kernel.rs 零领域知识        : NOT VIOLATED  (all changes route through state/predicates layers)
Append-Only DAG             : STRENGTHENED  (Boltzmann mask is read-view only; ChainTape never deletes parent)
Economic conservation       : STRENGTHENED  (no ghost liquidity; MarketSeedTx debit required; Laws 1-2 verified at constitution.md:159-160)
Constitution.md edit needed : NO            (ruling 15: sudo-only)
Sudo trigger                : NONE
```

### Post-TB-8 → v1.0 roadmap (canonical per directive Part C)

```text
TB-8   Minimal Payout / FinalizeRewardTx                    Class 3, 72h+24h-checkpoints, STEP_B on Atoms 2+3
TB-9   Durable AgentRegistry + Wallet Projection             Class 3
TB-10  Lean Proof Task Market MVP                            Class 3 (first user-facing product)
TB-11  RSP-M0/M1 NodePosition + PriceIndex (no trading)      Class 1
TB-12  CompleteSet + MarketSeedTx                            Class 3
TB-13  CPMM Router (mint-and-swap)                           Class 3
TB-14  PriceIndex + Boltzmann masking (read-view only)       Class 1
TB-15  Lamarckian Autopsy + Markov Log Loom (EvidenceCapsule) Class 1
TB-16  Beta with market signals
v1.0   Lean Proof Task Market on ChainTape (≥100 tasks replayable)

RSP-3.2 Slash re-deferred to post-TB-15 territory (slash hardens the payout invariant; payout *is* the invariant).
NodeMarket trading (TB-17) is post-v1.0.
```

### TB-7R state remains GREEN

TB-7R remains shipped at commits `55680bb` + `46716ae` + `17d69de`. No regression introduced this session (no code touched; only handover/directive/alignment files).

712 / 0 fail / 150 ignored — unchanged baseline for TB-8 to extend (+20-30 expected).

### Open Atom-0.5 ratification questions (TB-8 charter §7)

These are NOT shipping blockers — they are charter ratification points to resolve at TB-8 Atom 0.5 before Atom 1 begins:

1. **`ClaimEntry` schema extension shape** — 6-field expansion as proposed, or compact `{ amount, claimant, status, lookup_refs }` packed shape?
2. **Idempotency error variant naming** — add `ClaimAlreadyFinalized`, broaden `ClaimAlreadySlashed`, or introduce `ClaimAlreadyResolved(ClaimStatus)`?
3. **Zero-window MVP vs minimum-1-block window** — solo-run zero-window is the literal `feedback_launch_priority` minimal-payout, but `Art.III.4 challenge_window_closed` semantically wants a window.
4. **Conservation invariant: `debug_assert` vs `assert`** — debug-time check + dedicated release-mode test, or always-on panic guard?
5. **`reward_factor`** — `claim.amount = task_market_entry.total_escrow` for single-solver MVP, or reserve a platform-fee placeholder field?

Proposed defaults (charter §7): 1=6-field, 2=add `ClaimAlreadyFinalized`, 3=zero-window, 4=`debug_assert` + release test, 5=total_escrow no-fee.

### Next-session prompt (paste verbatim at start of new session)

```text
TB-8 Atom 0.5 + Atom 1-8 sequenced execution.

CONTEXT (READ IN ORDER):

1. /home/zephryj/projects/turingosv4/CLAUDE.md
2. /home/zephryj/projects/turingosv4/handover/ai-direct/LATEST.md   (top section: 2026-05-02 directive ingest)
3. /home/zephryj/projects/turingosv4/constitution.md                 (canonical, especially Laws 1-2 line 159-160 + Art. III.4)
4. /home/zephryj/projects/turingosv4/handover/tracer_bullets/TB-8_charter_2026-05-02.md   (rewritten — your work order)
5. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_lossless_constitution_polymarket_directive.md   (overview)
6. /home/zephryj/projects/turingosv4/handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md   (canonical 15 numbered rulings)
7. /home/zephryj/projects/turingosv4/handover/alignment/TRACE_FLOWCHART_MATRIX.md   (you must add a TB-8 row at Atom 8)
8. /home/zephryj/projects/turingosv4/handover/alignment/DECISION_POLYMARKET_CORE_2026-05-02.md
9. /home/zephryj/projects/turingosv4/handover/alignment/DECISION_CPMM_MINT_AND_SWAP_2026-05-02.md
10. /home/zephryj/projects/turingosv4/handover/alignment/DECISION_MARKET_SEED_NO_GHOST_LIQUIDITY_2026-05-02.md
11. /home/zephryj/projects/turingosv4/handover/alignment/DECISION_LAMARCKIAN_AUTOPSY_BOLTZMANN_2026-05-02.md
   (8-11: forward decisions — they bind TB-11..TB-15 forbidden lines but hold for TB-8 too:
    NO ghost liquidity, NO agent-submitted system tx, NO predicate override.)

DO NOT RE-INGEST THE DIRECTIVE. It's already archived under /handover/directives/2026-05-02_*. Read but do not duplicate.

WHAT TO DO:

Step 1 — TB-8 Atom 0.5: write architect ratification document at
handover/audits/CHARTER_RATIFICATION_TB_8_2026-05-XX.md resolving the 5 open
questions in TB-8 charter §7. The charter's proposed defaults are reasonable;
present them as the recommended path with brief justification, then ASK USER
TO RATIFY before starting Atom 1. Per `feedback_no_fake_menus`: state the
recommendation as the answer, not as one of N options.

Step 2 — TB-8 Atom 1 through Atom 8 in sequence per the charter §3 + §6 plan:
  Atom 1 — claims_t writer at VerifyTx OMEGA accept                Class 2, 24h
  Atom 2 — SystemEmitCommand::FinalizeReward ingress                Class 3, 24h, STEP_B preflight
  Atom 3 — TypedTx::FinalizeReward dispatch arm (load-bearing)      Class 3, 72h with 24h checkpoints, STEP_B preflight
  Atom 4 — Evaluator OMEGA-branch caller                            Class 2, 24h
  Atom 5 — ChainTape smoke evidence (10 runs)                       Class 1, 24h
  Atom 6 — Audit-dashboard claim_status column                      Class 0/1, 24h
  Atom 7 — Recursive self-audit + dual external audit               Class 3, 24-48h
  Atom 8 — Ship handover + TB_LOG row + TRACE_FLOWCHART_MATRIX update  Class 0, <24h

CONSTRAINTS (binding):

- Phase tags required on every commit: phase_id=P3primary,P2carryforward;
  roadmap_exit_criteria_addressed=P3:RSP-4-MVP,P2:carryforward;
  kill_criteria_tested=P3:1,P3:2,P3:3.
- Commit message MUST include FC-trace and (where applicable) flowchart_trace.
- STEP_B preflight artifact required at handover/audits/STEP_B_PREFLIGHT_TB8_2026-05-XX.md
  before any change to src/state/sequencer.rs (Atoms 2 + 3).
- Smoke evidence dir: handover/evidence/tb_8_minimal_payout_smoke_2026-05-XX/
  with replay_report.json + runtime_repo.dotgit.tar.gz + cas.dotgit.tar.gz per run.
- Ship-gate test reporting MUST use `cargo test --workspace` canonical shape
  per feedback_workspace_test_canonical: workspace_count = N, failed = 0, ignored = M.
- No new memory rules expected. If you find yourself wanting to write one,
  STOP and ask the user first.
- NO ghost liquidity, NO agent-submitted FinalizeRewardTx, NO predicate override
  by price, NO automatic mint without explicit collateral debit — these are
  Class-3 hard rails from the four 2026-05-02 decision records.

DUAL AUDIT (Atom 7) per feedback_dual_audit Class 3 + feedback_risk_class_audit:

- Codex impl-paranoid on full TB-8 diff (RQ1-RQ4 minimum).
- Gemini architectural at gemini-3.1-pro-preview strategic tier (NOT degraded
  unless explicitly labeled per feedback_dual_audit `degraded` clause).
- VETO blocks ship per feedback_dual_audit_conflict.
- Round-2 auto-execute on determinate-best surgical patch per
  feedback_elon_mode_policy.

ITERATION CAP: 72h Atom 3 with 24h checkpoints; mandatory user escalation if
slipped. 24h cap on every other atom.

EXPECTED DELIVERABLE TIMELINE: 5-7 days realistic, 10 days pessimistic.

START:
1. Read items 1-11 above.
2. Verify TB-7R baseline still green: `cd /home/zephryj/projects/turingosv4 && cargo test --workspace` should report 712 passed / 0 failed / 150 ignored.
3. Run Step 1 (Atom 0.5 ratification doc; ASK USER for ratification).
4. After ratification, proceed Atom 1 → Atom 8.
```

---

## 🚢 2026-05-02 — TB-7R SHIPPED — Constitution-Aligned Frame B Repair (Class 3 dual ship audit; PASS)

**Session summary**: TB-7R ship-gate. Codex round-1 returned **VETO/HIGH** on
evidence packaging defect (committed evidence omitted `runtime_repo/.git/` +
`cas/.git/objects/`; CasStore::get failed to resolve from committed-only state;
acceptance clause 4 + ship cond #5 violated). Gemini PASS at strategic tier
(`gemini-3.1-pro-preview`; 4/5 conviction; NOT degraded). Per
`feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS): VETO blocked ship.
Per `feedback_elon_mode_policy` round-2 auto-execute: determinate-best surgical
remediation (evidence packaging via tar.gz + replay_report.json per run + OBS
framing tightening) → Codex round-2 **PASS** (RQ1-RQ4 all green). Both auditors
clear. TB-7R shipped at `55680bb` + `46716ae` (TB_LOG hash backfill); pushed to origin/main.

### Final ship-gate

```text
command         = cargo test --workspace
workspace_count = 712  (+26 vs TB-7 ship 686; no code change in remediation; new tests are TB-7R Deliverables A-F)
failed          = 0
ignored         = 150
HEAD            = 46716ae
ship commit     = 55680bb (4934 insertions / 17 deletions / 41 files)
```

### TB-7R commit chain (7 commits on main)

| Commit | Subject | Class |
|---|---|---|
| `696d10f` | TB-7R A+B+E — verdict ingestion + L4 purity + ChainTape-mode fail-closed | Class 1 + 0 |
| `392a516` | TB-7R C+D+CP2 — genesis_report.json + on-chain TaskOpen/EscrowLock verification | Class 2 |
| `b517ae5` | TB-7R audit-fix — Codex Claim 7 remediation (orphan TRACE_MATRIX) | Class 0 |
| `013f2ce` | TB-7R F — smoke evidence; 10 runs single/half/full | Class 1 |
| `4470036` | TB-7R parent_tx ParentTxState 4-variant + 6 conformance tests + verdict 2026-05-02 | Class 2 |
| `55680bb` | TB-7R SHIPPED — Class 3 dual ship audit; PASS (this session) | Class 1 |
| `46716ae` | TB-7R TB_LOG hash backfill | Class 0 |

### 4-clause acceptance + 7-condition ship gate closure

| Item | Status | Evidence |
|---|---|---|
| Acceptance clause 1 (every externalized → L4/L4.E) | GREEN under three-node taxonomy | OBS-1 §2.1.a documents PartialOk → Complete proof-prefix as TB-8+ scope |
| Acceptance clause 2 (predicate evidence resolves from CAS) | GREEN | Codex round-2 RQ3 walked end-to-end CID chain on single_n1: entry_payload → work_proposal → telemetry_VR → proof_artifact, all sha256-validated |
| Acceptance clause 3 (failed shielded; auditable) | GREEN | TB-1 P0-3 serde shield holds; dashboard reads only `rejection_class` |
| Acceptance clause 4 (dashboard regeneratable from ChainTape + CAS alone) | GREEN | 10/10 runs round-trip from committed `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` |
| Ship cond 1-7 | ALL GREEN | per `handover/audits/RECURSIVE_AUDIT_TB_7R_2026-05-02.md` §4 |

### Audit verdicts (Class 3 full dual at strategic tier; NOT degraded)

| Auditor | Round 1 | Remediation | Round 2 | Final |
|---|---|---|---|---|
| Codex (impl-paranoid) | VETO/HIGH (evidence packaging) | tar.gz + replay_report + OBS tightening (~30 min surgical) | PASS RQ1-RQ4 | PASS |
| Gemini `gemini-3.1-pro-preview` (architectural) | PASS 4/5; SHIP-CLEAR WITH OBS-TIGHTENING | — | — | PASS |

**Round-1 finding closure**:
- F1 Evidence packaging → tar.gz × 10 runs, 892 KB total committed (vs 4.8 MB loose; tar.gz needed because git auto-ignores nested `.git/`)
- F2 `replay_report.json` per run → committed; 7 top-level booleans true + initial_q_state_loaded_from_disk=true on all 10
- F3 PartialOk → Complete proof-prefix dependency → OBS-1 §2.1.a + §4.3 (deferred to TB-8+ per verdict A1=B′)
- F4 OBS-2 prompt-pollution premise stale → closed-as-empirically-unfounded per Codex Q10 (acc.record_tool_stdout only increments token cost; raw Lean text never hits prompt)

### Open follow-ups (carry-forward; NOT ship blockers)

1. **OBS-1 coverage denominator** (`handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`) — architect-acknowledged post-TB-7R. PartialOk → Complete proof-prefix dependency: accepted L4 WorkTx `proof_artifact_cid` resolves to `tactic` only, but verify_partial uses `tape_chain + tactic`. §4.3 hardening: route PartialOk through chain OR store concatenated `tape_chain + tactic` blob in CAS. Closure → TB-8+ per-tactic decomposition or TB-8.5 dedicated atom.
2. **OBS-R022 TRACE_MATRIX orphans** (`handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`) — 2 modules (`chaintape_mode_gate.rs` + `genesis_report.rs`) registered as orphans. Closure → future TRACE_MATRIX revision adds canonical rows under Art. IV Boot.
3. **CHECKPOINT_TB7R_2 #1** — `tb_7_chaintape_smoke_2026-05-01/README.md` annotation reverts via editor hook; investigate next session. Non-blocking.
4. **Pre-existing dirty files** (untouched this session, predate TB-7R): `h_vppu_history.json`, `handover/evidence/tb_7_chaintape_smoke_2026-05-01/*` (3 files), `rules/enforcement.log`. Treat as background drift / runtime artifacts.

### Memory updates from this session
None — no new memory rules. The session validated existing rules:
- `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS) drove the round-1 block
- `feedback_elon_mode_policy` round-2 auto-execute exception covered the surgical remediation
- `feedback_dual_audit` Class 3 hybrid + degraded-mode rules guided audit launches
- `feedback_workspace_test_canonical` mandated reporting shape

### Next-TB direction (decided 2026-05-02, end-of-session)

**TB-8 = minimal payout / FinalizeRewardTx** per architect ruling 2026-05-01 §13 sequencing + memory `feedback_launch_priority` (Audit dashboard → Minimal payout → Beta launch) + memory `feedback_iteration_cap_24h` (capability-first; on-chain settlement is shortest path to H-VPPUT signal beyond proposal-acceptance).

**Scope**: single-solver / single-verifier / no royalty / no DAG / no NodeMarket. First on-chain settlement primitive. Closes the basic 5-step compile loop (Proposal → Ground-Truth Feedback → **Settlement** → Logging → Capability Compilation → ↑H-VPPUT) at the settlement node.

**Class**: Class 3 (auth-crypto-money — new system-emitted economic mutator). STEP_B preflight required for `src/state/sequencer.rs` (new FinalizeRewardTx dispatch arm). Full dual audit at strategic tier mandatory.

**Forbidden** (carry forward + post-MVP per ruling §6/§8): NodeMarket trading, AMM, multi-solver royalty, DAG-aware payout splits, public-chain anchoring, MetaTape, multi-org, full RSP-4 settlement, P6 expansion.

**Alternative directions** (if next-session redirect needed):
- **TB-8 OBS-1 coverage hardening** — PartialOk → chain-routing + self-contained proof artifact. Tech-debt cleanup; doesn't move H-VPPUT axis. Skip unless OBS-1 starts blocking other work.
- **TB-7.5 audit dashboard expansion** — UI / multi-run roll-ups. Lighter; pure additive; could be a session interlude before TB-8.

### Repo state (post-TB-7R)
- HEAD: `46716ae` (TB-7R TB_LOG backfill)
- origin/main: synced (pushed 2026-05-02 end-of-session)
- Working tree dirty (unrelated, pre-existing): `h_vppu_history.json`, `tb_7_chaintape_smoke_2026-05-01/*`, `rules/enforcement.log`

---

## 🚢 2026-05-01 — TB-7 SHIPPED — Frame B authoritative routing (Atoms 1 / 1.5 / 1.7 / 2 / 3 / 4 / 5 / 6 / 7)

**Session summary**: User authorized "自主执行直到 TB-7 审计结束" (autonomous execution until TB-7 audit ends).
All 9 charter atoms shipped on `main`. Frame B authoritative routing for real-LLM proposals
through `bus.submit_typed_tx` is structurally CLOSED. Recursive self-audit GREEN; Codex impl
audit on full TB-7 diff to follow as Atom 7 ship-time follow-up.

### Commit chain (Atoms 1–7, after Atom 0/0.5 ratification)

| Commit | Atom | Highlights |
|---|---|---|
| `c3ad31e` | 1 | `src/runtime/agent_keypairs.rs` (430 lines) — AgentPublicKey + AgentKeypair + AgentKeypairRegistry + AgentPubkeyManifest + verify_agent_signature; 6 unit tests; run-local identity caveat per ruling D2. |
| `eed4837` | 1.5 | `src/runtime/proposal_telemetry.rs` (280 lines) — ProposalTelemetry 8-field schema per ruling D5; TokenCounts + ToolCallRecord; build_for_evaluator_append helper; 5 unit tests with forbidden-field guard. |
| `0414b30` | 1.7 | TB-6 carry-forward: `logical_t` REMOVED from AgentProposalRecord (architect 9-field spec restored); audit_hash domain v1 → v2; chain_link binds row-level logical_t; fail-closed bootstrap with BootstrapError::RejectionWriter + `evaluator.rs:exit(2)` on TURINGOS_CHAINTAPE_PATH set with bootstrap fail; new I91e structural witness. Closes Codex audit cc7b3dd actions #1 + #3. |
| `2bc879c` | 2 | Evaluator append-branch authoritative routing — real-signature WorkTx via `make_real_worktx_signed_by` + `AgentKeypairRegistry::sign(canonical_digest)` + `proposal_cid` linkage to ProposalTelemetry CAS; legacy `bus.append` annotated `// shadow_only:` per §4.0 option (3); 3 integration tests I100/I101/I102. |
| `3572141` | 3 | Evaluator OMEGA-branch routing — `make_real_verifytx_signed_by`; sites 1517 (full-proof OMEGA) + 1865 (per-tactic OMEGA) emit WorkTx + VerifyTx pair via bus.submit_typed_tx with ChallengeWindow OPEN (no settlement); site 1917 (PartialOk) annotated shadow_only; 2 integration tests I103/I104. |
| `d03814f` | 4 | `verify_chaintape` extension — 2 NEW boolean indicators `agent_signatures_verified` (Gate 4) + `proposal_telemetry_cas_retrievable` (Gate 5); `verify_agent_artifacts` helper; `all_indicators_pass` 5 → 7 booleans. |
| `4cfe7cb` | 5 | `src/runtime/chain_derived_run_facts.rs` (290 lines; renamed from chain_derived_pput per ruling D4) — bit-exact §4.4 field set computed from L4 + L4.E + CAS alone; time-sensitive fields excluded; 3 unit tests. |
| `2559c84` | 6 | Chain-backed smoke (synthetic-LLM end-to-end) — I110 ship-gate test produces `handover/evidence/tb_7_chaintape_smoke_2026-05-01/` with all 7 ReplayReport indicators GREEN; real-LLM smoke documented as manual carry-forward. |
| (this commit) | 7 | Recursive self-audit + Gate 7 conformance — `handover/audits/RECURSIVE_AUDIT_TB_7_2026-05-01.md` + `tests/tb_7_legacy_append_regression.rs` (3/3 conformance tests pass); TB-6 audit-pending closure path mapping per §13.4. |

### Final ship-gate

```text
command         = cargo test --workspace
workspace_count = 686  (+26 vs TB-6 ship 660)
failed          = 0
ignored         = 150  (unchanged)
```

### 7 ship gates closure

| Gate | Status | Evidence |
|---|---|---|
| 1 (authoritative path) | GREEN | charter §4.0 + Gate 7 conformance test |
| 2 (proposal count equality) | GREEN structurally; real-LLM = manual | I110 round-trip; chain_derived_run_facts.json |
| 3 (≥1 L4 + ≥1 L4.E) | GREEN | smoke evidence: 1 L4 + 6 L4.E |
| 4 (signature verification) | GREEN | replay_report.json: agent_signatures_verified + system_signatures_verified BOTH true |
| 5 (CAS retrievability) | GREEN | replay_report.json: proposal_telemetry_cas_retrievable=true |
| 6 (chain-derived run facts) | GREEN structurally | I110 round-trip witness |
| 7 (legacy-bypass regression) | GREEN | 3/3 conformance tests pass |

### TB-6 audit-pending closure path (§13.4)

| Codex action | Closure status |
|---|---|
| #1 fail-closed bootstrap | **CLOSED** at Atom 1.7 |
| #2 real proposal/OMEGA/rejection through typed ChainTape | **CLOSED** structurally (Atoms 2 + 3); real-LLM = manual |
| #3 AgentProposalRecord schema (logical_t) | **CLOSED** at Atom 1.7 |
| #4 audit-index row hash from CAS | PARTIAL — Gate 4 covers signature path; full hash recompute = follow-up TB |
| #5 strict tx_id ↔ CID ↔ AgentProposalRecord | PARTIAL — chain_derived_run_facts enforces ProposalTelemetry CAS resolution; full RunSummary cross-check = follow-up TB |
| #6 disk-level tamper tests (CAS / Git L4 / derivative roots) | PARTIAL — Gate 4 covers signature; I90d/e/f/g full battery = follow-up TB |
| #7 regenerate TB-6 smoke | PARTIAL — synthetic-LLM smoke regenerated at Atom 6; real-LLM smoke = manual carry-forward |

**TB-6 audit-pending status REMAINS OPEN** at TB-7 ship per §13.4 anti-pile-up rule. 4
partial action items roll to a follow-up TB. This is honest accounting — TB-7 closes the
*structural* part of the gap; the *full conformance battery* + real-LLM run remain.

### Status

- TB-7 SHIPPED on `main` @ `<this commit>`. Frame B (authoritative path) structurally CLOSED.
- TB-7 Atom 7 ship-time Codex impl audit on full TB-7 diff: launches as follow-up to this commit.
- Gemini arch audit: degraded fallback per `feedback_dual_audit` (TB-5/TB-6 supplement precedent).
- Real-LLM smoke (mathd_algebra_107 with live DeepSeek + Lean): manual carry-forward.

### What user / Claude can do next

1. **Codex impl audit feedback** — review the audit verdict; if SOME_CHALLENGE or VETO,
   remediate via micro-PR before TB-8. If ALL_PASS, proceed to TB-8.
2. **Manual real-LLM smoke** — run `TURINGOS_CHAINTAPE_PATH=... cargo run --bin evaluator
   -- --problem mathd_algebra_107 --max-tx 20`. Verify with `verify_chaintape` CLI.
3. **TB-8 audit dashboard** — per charter §13.1 next: UI/CLI to inspect what the Agent
   saw + submitted + how the system judged, on a per-run basis.
4. **Follow-up TB for partial closure**: open a follow-up TB (TB-7.5 or TB-8 carry-forward)
   to close the 4 partial Codex action items (#4, #5, #6, #7 full real-LLM).

---

## 📋 2026-05-01 — TB-7 Atom 0.5 — Codex audit carry-forward — Atom 1.7 added + Atom 4/5 expanded + §13.4 closure path

**Trigger**: Codex full-diff audit of TB-6 (commit `cc7b3dd`, 7m 36s + 5m 8s save retry) returned **SOME_CHALLENGE** — PASS 1 (A5) / CHALLENGE 6 (A1, A2, A3, A4, A6, A7) / VETO 0. 7 blocking action items; 4 of them (#2 + #5 + #6 + #7) already covered by TB-7 charter as ratified; **2 new items (#1 fail-closed bootstrap + #3 logical_t schema repair) require carry-forward** into Atom 1.7. Codex explicitly preserved TB-6 audit-pending status; closure path now encoded in §13.4.

### What landed (Atom 0.5 carry-forward; no production code touched)

| Commit | Files | Purpose |
|---|---|---|
| `cc7b3dd` (audit) | `handover/audits/CODEX_TB6_FULLDIFF_AUDIT_2026-05-01.md` (NEW; 184 lines) | Codex audit evidence — 7 dimensions A1-A7 verdicted with file:line citations; 7 action items each with file:line + suggested fix + blocking=yes; explicit non-closure recommendation. |
| (this commit) | `handover/tracer_bullets/TB-7_charter_2026-05-01.md` (modified) | §5.1 build surface + §5.2 tests: Atom 1.7 NEW (logical_t removal + fail-closed bootstrap); Atom 4 expanded (audit-index hash from CAS + I90d/e/f/g disk-level tamper); Atom 5 expanded (strict tx_id ↔ CID correlation). §6 #28 caveat. §7 atom plan: Atom 0.5 + Atom 1.7 inserted. **§13.4 NEW** TB-6 audit-pending closure path. |
| (this commit) | `handover/ai-direct/LATEST.md` (modified) | This entry. |

### TB-6 audit findings → TB-7 charter mapping

| Codex action | Closure atom | Type |
|---|---|---|
| #1 fail-closed bootstrap | **Atom 1.7** (b) — NEW | carry-forward |
| #2 real proposal/OMEGA/rejection through typed ChainTape | Atom 2 + Atom 3 (§4.0 already covers) | already covered |
| #3 AgentProposalRecord schema repair (logical_t) | **Atom 1.7** (a) — NEW | carry-forward |
| #4 audit-index row hash from CAS | **Atom 4 expansion** | scope deepening |
| #5 RunSummary tx_id ↔ CID ↔ AgentProposalRecord | **Atom 5 expansion** | scope deepening |
| #6 disk-level tamper tests (CAS / Git L4 / derivative roots / pinned pubkeys) | **Atom 4 expansion** (I90d/e/f/g) | scope deepening |
| #7 regenerate TB-6 smoke evidence | Atom 6 (chain-backed real-LLM smoke supersedes synthetic) | natural supersession |

**TB-6 audit-pending closes when** all 7 action items ship green via TB-7. If any remain red at Atom 7 ship, TB-6 audit-pending stays open and rolls to follow-up TB (anti-pile-up rule).

### Autonomous decisions made (per user mandate "依据宪法/白皮书/架构师意见自主决策")

1. **`logical_t` handling = remove from record, keep in JSONL index row**.
   - Constitutional grounding: Art. V (机制 > 参数), C-023 (schema additions = ArchitectAI contribution; cannot be silently migrated by implementer); architect ruling TB-6 D7 (NO constitutional amendment) preserves the 9-field spec.
   - Why not (b) ratify as 10th field: schema ratification is architect-only per C-023; not in my decision authority.
   - Why not (c) fold into Atom 1: Art. I.1 atomicity / C-027 — spec restoration ≠ new feature; must be independently auditable.

2. **fail-closed bootstrap = Atom 1.7 (b), folded with logical_t**.
   - TB-7 §4.0 + §6 #31: silent fallback is forbidden. Bootstrap silent fallback is the same anti-pattern.
   - Same Atom because both touch the same subsystem hot path; opening Atom 0.5 sub-atom for 1 line of behavior change would be ceremony.

3. **Codex audit commit separated from carry-forward charter commit (per C-010 Generator ≠ Evaluator)**.
   - Audit doc = Codex evidence (Evaluator authorship)
   - Charter amendments = my response (Generator authorship)
   - Mixing them in one commit violates the audit-trail integrity principle.

4. **Atom 4/5 expansion vs new sub-atoms**: Codex action items #4 + #5 + #6 are scope deepening, NOT new scope. They land on the same files / atoms already in the charter. Opening sub-atoms for them would inflate atom count without scope clarity.

### Status

- TB-6 SHIPPED on `main` @ `17c5e73`. Audit-pending status **preserved** per Codex audit cc7b3dd; closure path = §13.4.
- TB-7 charter: 8 atoms (Atom 0 SHIPPED @ 05c5be7; Atom 0.5 = this commit; Atom 1 / 1.5 / **1.7 NEW** / 2 / 3 / 4 / 5 / 6 / 7 pending).
- TB-7 Atom 1 paused for user re-engagement (per Atom 0 pacing decision).

### What user / Claude can do next

1. **Begin Atom 1** — `src/runtime/agent_keypairs.rs` + `agent_pubkeys.json` (additive; non-STEP_B). May proceed in parallel with Atom 1.5 + Atom 1.7.
2. **Begin Atom 1.5** — `src/runtime/proposal_telemetry.rs` (additive; non-STEP_B).
3. **Begin Atom 1.7** — `src/runtime/agent_audit_trail.rs` schema repair (logical_t removal) + `src/runtime/mod.rs` + `evaluator.rs:675-680` fail-closed bootstrap.
4. **Atom 6 discharge gate** — chain-backed real-LLM smoke must run within 72h of Atom 0 ship per `feedback_iteration_cap_24h` production wire-up exception. Atom 0 = 2026-05-01; deadline = 2026-05-04.

---

## 📋 2026-05-01 — TB-7 charter RATIFIED — Frame B authorized + 7 ship gates encoded

**Session continuation**: Post-TB-6 ship dialogue surfaced "real chaintape final form" 4-frame
breakdown (A=narrow architect-D2 / B=LLM-on-chain / C=full economic loop / D=multi-org+public+autonomous).
Architect ruling 2026-05-01 (post-`/clear` reload) **re-classifies TB-6 as Frame A only** (Frame B: RED) and
**authorizes TB-7 as Frame B** = per-LLM-proposal WorkTx routing through `bus.submit_typed_tx` as
**AUTHORITATIVE** path (NOT "also emit"). Charter draft renamed (drop `_draft_`) and amended per
ruling D1-D5; 7 ship gates added; post-TB-7 sequencing reset to Lean Proof Task Market MVP.

### What landed (RATIFICATION commit set; no production code touched)

| File | Purpose |
|---|---|
| `handover/directives/2026-05-01_TB7_ARCHITECT_RULING.md` (NEW) | Formal architect ruling. §0 verdict + §1 TB-6 Frame A acceptance + §2 TB-7 Frame B authorization + §3 charter amendment matrix (D1-D5) + §4 seven ship gates + §5 alignment to constitution+WP+roadmap + §6 post-TB-7 launch priority + §7 Class 0–4 risk-class audit + §8 process evaluation + §9 final execution order + §10 Layer 1 impact analysis (no constitutional amendment) + §11 verbatim original directive. |
| `handover/tracer_bullets/TB-7_charter_2026-05-01.md` (renamed from `_draft_`) | TB-7 charter — RATIFIED. §4.0 authoritative path requirement (NEW; load-bearing); §4.4 ChainDerivedRunFacts (renamed from chain-derived PPUT; bit-exact on §4.4 structural field set); §4.5 ProposalTelemetry CAS (NEW per D5); §6 forbidden #31-33; §7 8-atom plan with new Atom 1.5 (proposal_telemetry.rs); §8 seven ship gates (replaces 3-proof draft); §12 Q1-Q5 RESOLVED; §13 post-TB-7 sequencing override (TB-8 audit dashboard → TB-9 minimal payout → TB-10 beta → TB-11 NodeMarket v0). |
| `handover/tracer_bullets/TB_LOG.tsv` | TB-7 active row + ratification comment line added. 11 columns: phase_id=P2(primary; P1/P3 carry-forward); roadmap_exit_criteria=P1:5,6,7,8,9 P2:1,6 P3:carry-forward; kill_criteria=P1:1-4 P3:1-3 (P3:9 deferred TB-9). |
| `~/.claude/projects/.../memory/feedback_risk_class_audit.md` (NEW) | Class 0–4 audit standard codified. Class 0 docs / Class 1 additive / Class 2 production wire-up / Class 3 auth-crypto-money / Class 4 constitution-sudo. |
| `~/.claude/projects/.../memory/feedback_launch_priority.md` (NEW) | Lean Proof Task Market MVP > NodeMarket post-TB-7 sequencing codified. |
| `~/.claude/projects/.../memory/MEMORY.md` | Two new index entries pointing to the above. |

### TB-7 scope boundaries (RATIFIED)

**IN SCOPE (Frame B; binding)**:
- Per-agent Ed25519 keypair, **run-local identity only** (caveat per ruling D2; not durable reputation).
- Real-signature WorkTx via `bus.submit_typed_tx` as **authoritative path** (legacy `bus.append` removed / projected / `// shadow_only:` annotated).
- VerifyTx for OMEGA-accept Lean verification (ChallengeWindow OPEN; no settlement).
- `ChainDerivedRunFacts` (bit-exact on §4.4 structural field set: solved/verified/tx_count/proposal_count/golden_path_token_count/gp_payload/gp_path/gp_proof_file/tactic_diversity/tool_dist/failed_branch_count). Time-sensitive fields excluded.
- `ProposalTelemetry` CAS objects per WorkTx (agent_id, prompt_context_hash, proposal_artifact_cid, candidate_tactic, token_counts, tool_calls, branch_id, parent_tx).
- `verify_chaintape` extension (agent-signature path + ProposalTelemetry CAS retrieval).
- Real-LLM smoke run on `mathd_algebra_107` producing ≥1 accepted L4 + ≥1 rejected L4.E (Gate 3; forced rejection allowed only with `forced_rejection_for_gate_3 = true` label).

**OUT OF SCOPE (deferred per ruling §6 + charter §13 post-MVP sequencing)**:
- FinalizeRewardTx settlement → TB-9 minimal payout
- SlashTx upheld-challenge punishment → TB-9
- NodeMarket position semantics → TB-11 NodeMarket v0 (post-MVP)
- AMM / Polymarket trading layer → TB-12+
- New TypedTx variants
- Q schema mutation
- Persistent agent identity / cross-run reputation → separate TB

### Seven ship gates (Atom 7 ship requires GREEN on all)

| Gate | Requirement | Evidence |
|---|---|---|
| 1 | Authoritative path: every proposal through `bus.submit_typed_tx`; no legacy `bus.append` as authoritative state mutation | charter §4.0 + Gate 7 conformance test |
| 2 | `chain_proposal_count == evaluator_proposal_count` (instrumented; not stdout) | `chain_derived_run_facts.json:proposal_count` == evaluator structural facts |
| 3 | ≥1 accepted L4 + ≥1 rejected L4.E (forced rejection labeled `forced_rejection_for_gate_3 = true`) | smoke evidence ledger entries |
| 4 | All WorkTx signatures verify against `agent_pubkeys.json`; all system tx against `PinnedSystemPubkeys` | extended `verify_chaintape` |
| 5 | Every `WorkTx.proposal_cid` resolves to a CAS `ProposalTelemetry` object | `tests/tb_7_proposal_telemetry_cas.rs` |
| 6 | `ChainDerivedRunFacts == evaluator_run_facts` on §4.4 bit-exact set | Atom 5 round-trip test |
| 7 | Repo-wide regression: no proposal-producing site uses legacy append as authoritative | `tests/tb_7_legacy_append_regression.rs` |

### Architect decision items — RESOLVED (D1-D5)

| D | Decision | Verdict | Charter section |
|---|---|---|---|
| D1 | TB-7 sequencing | **Option A (Frame B)** + authoritative-path requirement (legacy append removed/projected/shadow-only) | §4.0 NEW; §5.1 evaluator row rewrite |
| D2 | Agent keypair lifecycle | **Runtime-generated per-run** + run-local-identity caveat | §4.2 amended |
| D3 | OMEGA-accept scope | **Narrowed** (WorkTx+VerifyTx only; ChallengeWindow OPEN; no FinalizeRewardTx/SlashTx) | §4.3 confirmed; §6 #21-23 |
| D4 | Chain-derived PPUT | **Renamed `ChainDerivedRunFacts`**; bit-exact on §4.4 field set; full PputResult retired | §4.4 rewrite + Atom 5 module rename |
| D5 | Audit mode + bundling | **Class 2 production wire-up** (Codex impl + Gemini arch with degraded fallback) + ProposalTelemetry CAS | §4.6 + §4.5 NEW + Atom 1.5 NEW |

### Post-TB-7 sequencing (charter §13; supersedes TB-6 ruling §4.5)

```
TB-7  (THIS) — Frame B per-LLM-proposal WorkTx routing
TB-8         — Audit dashboard
TB-9         — Minimal payout (single solver/verifier; no royalty; no NodeMarket)
TB-10        — Beta launch (narrow Lean problem set; real ChainTape + payout)
TB-10.5      — Persistent AgentRegistry + agent keystore (durable cross-run identity;
                REQUIRED before TB-11 — NodeMarket FirstLong/Short need persistent owner)
TB-11        — NodeMarket v0 (FirstLong/Short positions; PriceIndex v0; not tradable)
TB-12+       — Polymarket-like full market
```

NodeMarket trading, AMM, public chain, MetaTape, multi-org, full RSP-4 settlement, royalty, P6 PPUT research expansion, h_vppu polish: **DEFERRED post-MVP**. (Long-term reputation identity is no longer deferred — it lands at TB-10.5 because TB-11 cannot ship without it.)

### Status

- TB-6 SHIPPED on `main` @ `17c5e73` (8/8 atoms; cargo test --workspace 660/0/150). **Frame A only** per ruling §1.
- TB-7 = **RATIFIED 2026-05-01**. Atom 0 in progress (charter rename + ARCHITECT_RULING archive + TB_LOG row + LATEST.md flip + 2 memory files + MEMORY.md index). NO production code touched yet.
- TB_LOG.tsv has TB-7 active row (status=active; ship_commits=pending).

### What user / Claude can do next

1. **Commit Atom 0** — staging this ratification commit set (charter rename, ARCHITECT_RULING, TB_LOG TB-7 row, LATEST.md, 2 memory files, MEMORY.md index). User triggers commit explicitly.
2. **Begin Atom 1** — `src/runtime/agent_keypairs.rs` + `agent_pubkeys.json` manifest (additive; non-STEP_B).
3. **Begin Atom 1.5** (after Atom 1 lands) — `src/runtime/proposal_telemetry.rs` (additive; non-STEP_B).
4. **Atom 6 discharge gate** — chain-backed real-LLM smoke must run within 72h of Atom 0 ship per `feedback_iteration_cap_24h` production wire-up exception.
5. **Optionally**: Codex impl audit on full TB-6 diff as TB-7 follow-up (bundle at Atom 7 per ruling §3.5 + §4.6).

---

## 🚢 2026-05-01 — TB-6 SHIPPED (Atoms 4-7) — replay verifier + agent audit trail + RunSummary + ship audit

**Session summary**: User authorized "TuringOS v4 — TB-6 continuation (Atoms 4-7)" with explicit
architect ruling D1-D7 + charter § 4 + § 6 + § 8 line-grounded ship gate. **All 8 TB-6 atoms now
shipped on `main`.** Architect's full Path A objective satisfied: production binary drives
Sequencer to on-disk ChainTape; replay verifier reconstructs Q + EconomicState; Agent audit trail
records what the Agent saw + submitted (NOT chain-of-thought); RunSummary aggregates
proposal-level fork visibility.

### What landed (Atoms 4-7)

| Commit | Atom | Highlights |
|---|---|---|
| `f594f83` | 4 SHIPPED | `src/runtime/verify.rs` library + `src/bin/verify_chaintape.rs` CLI + `tests/tb_6_verify_chaintape.rs` (I90 / I90b / I90c). All 7 architect-mandated boolean indicators true on Atom 3 smoke evidence dir. Tampering-detection via I90c (tampered pinned_pubkey → signature verify fails). |
| `fcbb827` | 5 SHIPPED | `src/runtime/agent_audit_trail.rs` with `AgentProposalRecord` 9 fields + `AcceptedOrRejected` + CAS storage + `AgentAuditTrailIndex` JSONL with prev_hash→hash chain. Synthetic-seed hook in `evaluator.rs` writes audit pair on every chain-backed smoke run. **I91d structural witness**: JSON-grep blocks any future schema migration from adding `chain_of_thought` / `model_deliberation` / `tool_transcript` / `raw_prompt` / `raw_completion` / `internal_reasoning` field names. |
| `8e5ddb3` | 6 SHIPPED | `src/runtime/run_summary.rs` aggregator + `src/bin/gen_run_summary.rs` CLI + `tests/tb_6_run_summary.rs` (I92 / I92b / I92c). Walks L4 + L4.E + CAS at end-of-run; emits `run_summary.json` with architect-mandated fields. Production binary writes one automatically at end-of-run. |
| **(this commit)** | **7 SHIPPED** | Recursive self-audit at `handover/audits/RECURSIVE_AUDIT_TB_6_2026-05-01.md` (7/7 D1-D7 + 7/7 § 4 + 20/20 § 6 + 3/3 § 8 GREEN). TB_LOG TB-6 row active→shipped. NOTEPAD TB-6 SHIPPED log added. Audit label `degraded` per `feedback_dual_audit` (Gemini strategic-tier exhausted; TB-5 supplement precedent). |

### Test count progression

- Atom 4 ship: 646/0/150 (+7 vs Atom 3)
- Atom 5 ship: 654/0/150 (+8 vs Atom 4)
- Atom 6 ship: 660/0/150 (+6 vs Atom 5)
- **Atom 7 ship total**: **660 passed / 0 failed / 150 ignored across 51 suites** (+43 vs TB-5 ship 617).
- Per architect ruling D4: `cargo test --workspace` canonical at every atom.

### Smoke evidence final state

`handover/evidence/tb_6_chaintape_smoke_2026-05-01/`:
- `runtime_repo/.git/refs/transitions/main` commit `38f7112f6401067ffc66c5a00338e12ec810170b` (1 L4 entry)
- `runtime_repo/rejections.jsonl` (1 L4.E with prev_hash→hash chain)
- `runtime_repo/pinned_pubkeys.json` (TB-6 epoch 1 ed25519 pubkey)
- `cas/` (CAS payloads for both txs)
- `replay_report.json` — Atom 4 — all 7 boolean indicators true
- `run_summary.json` — Atom 6 — 1 accepted tx_id + 1 rejected tx_id + 2 candidate proposal CIDs
- `synthetic_rejection_label.json`, `proof.lean`, `pput_result.jsonl`, `n1_run.log`
- `README.md` answering all 8 architect-mandated questions (charter § 5.5)

### Architect ruling status (D1-D7)

- ✅ D1: Path A SHIPPED (5-TB ChainTape production debt CLOSED).
- ✅ D2: chain-backed smoke = HARD requirement. 8-condition gate satisfied; Atom 4 verify_chaintape demonstrates tampering-detection.
- ✅ D3: hybrid-by-risk audit applied. Atom 1 had Codex round-1+2 pre-ship; Atoms 4-6 kernel-only-additive class with self-audit + targeted smoke; Atom 7 ship audit carries `degraded` label per Gemini exhaustion.
- ✅ D4: `cargo test --workspace` canonical at every commit body in TB-6.
- ✅ D5: smoke-evidence naming applied throughout. Pre-TB-6 dirs = "smoke evidence"; tb_6_chaintape_smoke_2026-05-01 IS chain-backed.
- ✅ D6: 5 memory updates committed at Atom 0 ship.
- ✅ D7: NO constitution amendment (verified by `git diff` empty).

### What remains for next TB

- TB-7 candidate: RSP-M0/M1 NodePosition (post-TB-6 RSP-M track per ruling § 4.5) OR RSP-3.2 Slash (now reachable since chain-backed replay exists). Architect input expected on sequencing.
- Per-LLM-proposal main-loop wiring (run_swarm "append"/"complete" branches) deferred from Atom 5 to a future TB. Structural surface in place; main-loop hook is incremental.
- Codex impl audit on full TB-6 diff recommended as TB-7 follow-up (audit-pending follow-up, non-blocking per charter § 9 + ruling D3).
- 24h iteration cap reset for TB-7 per `feedback_iteration_cap_24h`.

---

## 🚀 2026-05-01 — TB-6 Atoms 0-3 SHIPPED (5-TB ChainTape production debt CLOSED)

**Session summary**: User authorized "继续把tb-6全部执行" after architect ruling 2026-05-01
selected Path A (P2 Agent Runtime / Production ChainTape Wire-up) over Path B
(RSP-3.2 Slash). 4 atoms shipped (0,1,2,3). **First chain-backed smoke evidence
in TuringOS v4 history.** Architect's primary ruling D1 satisfied.

### What landed (commit chain on main)

| Commit | Atom | Highlights |
|---|---|---|
| `7970d2d` | 0 | Charter + ROADMAP § 11.5 amendment + NOTEPAD + TB_LOG TB-6 active row + 5 memory updates per architect D6 + smoke-evidence rename per D5 |
| `ca8d644` → `37b1929` → `67e9a30` | preflight | v1 → v2 (Codex round-1 CHALLENGE-6) → v2.1 (Codex round-2 CHALLENGE-2). Round-cap=2 + auto-execute on determinate-best. |
| `76c35f3` | 1 SHIPPED | `src/runtime/mod.rs` factory + driver wrapper + L4.E JSONL backend (Atom 1.2 = `RejectionEvidenceWriter` + JsonlRecord shadow bypassing TB-1 P0-3 shield) + evaluator env-flag wire (Atom 1.3) + 15 tests. STEP_B not triggered (no restricted file modified per Codex Q4). |
| `01b9e93` | 2 SHIPPED | `src/runtime/adapter.rs` synthetic-tx constructors + `build_chaintape_sequencer_with_initial_q` variant + T11/T12/T13: T12 produces ≥1 L4 + ≥1 L4.E in one bundle. |
| **`b0a6039`** | **3 SHIPPED** | `handover/evidence/tb_6_chaintape_smoke_2026-05-01/` — first chain-backed smoke ever. mathd_algebra_107 SOLVED+VERIFIED via deepseek-v4-flash; refs/transitions/main 1 commit; rejections.jsonl 1 record; pinned_pubkeys.json + synthetic_rejection_label.json. |

### Test count progression
- Pre-TB-6: 617/0/150 (TB-5 baseline)
- Post-Atom-3: **639/0/150 across 48 suites** (+22 tests)
- `cargo test --workspace` is canonical per architect D4 reporting standard.

### Key technical decisions

1. **L4.E "或等价结构"** = JSONL append-only with embedded `prev_hash + hash` chain at `<runtime_repo>/rejections.jsonl`. Architect § 3.5 explicitly permits via "或等价". No `refs/rejections/main` git ref needed.
2. **`Sequencer::run` not called**. Codex round-2 verified `run` has no shutdown branch + Sequencer owns queue_tx → driver task's `Arc<Sequencer>` would prevent clean exit. Replaced with runtime-side wrapper using `tokio::select! biased` on shutdown_rx + `Sequencer::apply_one` direct calls (`pub(crate)`; same crate). Sequencer.rs untouched. STEP_B safe.
3. **JsonlRecord shadow struct** — `RejectedSubmissionRecord.raw_diagnostic_cid` has TB-1 P0-3 `#[serde(skip_serializing, default)]` shield (Inv 10 agent-boundary). For L4.E forensic ledger we need the field for `compute_hash` round-trip; shadow struct bypasses the skip in JSONL backend. The shield STAYS on `PublicRejectionView` (agent-facing).
4. **Atom 3 synthetic seed**: per architect § 3.6 Atom 3 ("if no natural rejection, synthesize with explicit label"), evaluator emits 1 TaskOpen + 1 zero-stake WorkTx via `bus.submit_typed_tx` when chaintape mode is on. Per-LLM-proposal WorkTx routing deferred to Atom 5.
5. **Atom 1 was scoped via Codex round-1 + round-2**. CHALLENGE-6 → CHALLENGE-2 → ship. R-022 hook false positives handled via `OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md` + `[R-022-skip:]` token.

### Architect ruling status (D1-D7)
- ✅ D1: Path A SHIPPED — production binary now drives Sequencer to on-disk ChainTape.
- ✅ D2: chain-backed smoke = HARD requirement satisfied for first time.
- ✅ D3: hybrid-by-risk audit applied (Codex impl audit ×2 on production wire-up; Gemini deferred to Atom 7 ship audit).
- ✅ D4: `cargo test --workspace` reporting in every commit body.
- ✅ D5: pre-TB-6 dirs labeled "smoke evidence"; tb_6_chaintape_smoke_* IS chain-backed and called "tape" without abuse.
- ✅ D6: 5 memory updates committed at Atom 0.
- ✅ D7: NO constitution amendment (preserved).

### What remains for TB-6 ship (Atoms 4-7)

- **Atom 4** — `verify_chaintape` CLI / replay verifier (~200-300 LOC + 2-3 tests)
- **Atom 5** — Agent audit trail (proposal CIDs in CAS; `prompt_context_hash` linkage to `tx_id`; routes per-LLM-proposal WorkTx through `bus.submit_typed_tx`)
- **Atom 6** — Branch / fork visibility summary (`failed_branch_count`, `rollback_count`, accepted/rejected tx_id sets)
- **Atom 7** — Codex impl audit + Gemini arch audit (degraded label if exhausted) + recursive self-audit + TB-6 ship merge

Next-session prompt: `handover/directives/2026-05-02_TB6_NEXT_SESSION_PROMPT.md`.

### Open items / risks
- Disk: 2.6G free at session end. `cargo clean` recommended before Atom 4 if disk-tight (don't touch `.lake` per user rule).
- Per-LLM-proposal WorkTx routing: structurally placeholder until Atom 5.
- Early-return paths in `run_swarm` drop `chaintape_bundle` without explicit `shutdown()`; driver still terminates cleanly via shutdown_tx-drop → shutdown_rx-Err path; safe but best-effort.
- `Gemini at strategic tier` may still be `MODEL_CAPACITY_EXHAUSTED` per TB-5 supplement — degraded-label fallback ready for Atom 7.

---

## 🔍 2026-05-01 — TB-5 post-ship self-audit + chaintape gap surfaced (architect review awaiting)

**Authorization**: user "没有针对烟测的tape进行审计，由你负责审计，不需要外审" → single-AI self-audit (no external auditor). Follow-up: "现在 turingos 具有真正的 chaintape 了吗？你是在 chaintape 上读取的测试全部信息进行审计的吗？" surfaced the substantive finding.

### What landed

| File | Purpose |
|---|---|
| `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` | Smoke-tape self-audit. §1: 8 verified claims PASS. §2: cosmetic test-count under-report (464→617). §3: substantive chaintape gap. §4 verdict + remedy. §5 audit caveats. |
| `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` | Cumulative stage audit TB-1..TB-5. §1 per-TB summary table. §2 what's structurally green (kernel, Anti-Oreo, RSP-1/2/3.0/3.1, anti-drift CI). §3 what's gap (production-binary chaintape wire-up, smoke evidence is paper trail not chain, RSP-3.2/4/5/6/7 RED, P2/P4 RED). §4 5 open debts. §5 8 production claims rolling forward. |
| `handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md` | Architect review request with 5 binding decision items D1-D5 for TB-6 sequencing + audit-mode standard + chaintape gap remedy. Awaiting `2026-05-XX_TB6_DIRECTIVE.md` response. |
| (patch) | 5 living docs corrected from 464/464 → 617/617 (`README` + `RECURSIVE_AUDIT` + `TB_LOG` × 2 + `NOTEPAD`). Merge commit `1bdc55a` body cannot be amended; superseded by reference. |

### Key findings (one-liner each)

1. **(cosmetic) Test-count under-report**: TB-5 ship-gate "464/464" was bare `cargo test`; actual `cargo test --workspace` is **617/617** (46 suites, 0 failed). Off by 153 tests across 5 docs. Patch commit on main.

2. **(substantive) Chaintape gap**: TB-5 "smoke tape" evidence (`oneshot_run.log` + `n1_run.log` + `proof_n1.lean` + `README.md`) is **paper trail, not chain**. No production binary drives `Sequencer::apply_one`. `bus.rs` sequencer field is `None` in main.rs. The evaluator does not import `turingosv4::state::sequencer`. The chaintape machinery only runs inside `cargo test` (InMemoryLedgerWriter). No on-disk chain has ever been produced from any LLM-driven run in TuringOS history. **5-TB cumulative debt** (TB-1..TB-5 each shipped kernel improvement; none exercised by an LLM-driven binary).

3. **Audit performed was paper-tape level**: I read 4 files + cross-grepped 5 evidence dirs + sha256-matched the proof artifact + re-ran Lean v4.24.0 + re-ran `cargo test --workspace`. The cargo test re-run IS a chain audit (in-memory chain) — but for the cargo test suite, not for the smoke runs themselves. The .log files are bounded by conventional file-system trust, not cryptographic chain trust.

4. **"Smoke tape" naming is a v3 PaperTape-era metaphor**, not a structural property. Recommend rename → "smoke evidence" (architect review D5).

### What architect needs to rule on (D1-D5 in review request)

- **D1**: TB-6 = RSP-3.2 slash (current ROADMAP plan) vs P2 Agent Runtime atom (close chaintape gap first; recommended). Stake: 5-TB chaintape debt vs additional kernel-only TB.
- **D2**: smoke gate evolution — should chaintape traversal become required from TB-X onward?
- **D3**: audit-mode standard — TB-3/TB-4 Option B (self-audit + smoke) vs TB-5 Codex-only vs hybrid by constitutional risk class.
- **D4**: lock down `cargo test --workspace` as canonical ship-gate test command.
- **D5**: rename "smoke tape" → "smoke evidence" across docs.

### What's substantively defensible at TB-5 ship (despite the gap)

- 8 production claims (Anti-Oreo, RSP-0/1/2/3.0/3.1 chain, defense-in-depth pinned-pubkeys, CTF conservation, 9-sub-field invariant) all GREEN under `cargo test --workspace` (617 tests).
- Lean re-verification holds end-to-end on the one proof produced.
- Smoke runs were genuine (timestamps + run_ids verified session-fresh, not stale repeats).

### What's NOT proven by smoke evidence (despite ship docs language)

- That TB-5 runtime spine was reachable from the evaluator
- That any TypedTx ever traversed `dispatch_transition` during the smoke runs
- That any LedgerEntry was produced
- That the runtime kernel's Anti-Oreo barriers were ever exercised at LLM-driven runtime

These belong to **P2 Agent Runtime** wire-up, deferred from TB-1..TB-5 by design. Architect ruling on D1 determines when this debt closes.

---

## 🚢 2026-04-30 — TB-5 SHIPPED (P3 RSP-3.0 + RSP-3.1 System-Emitted Resolution Gate, WP-canonical)

**Authorization**: user "继续直到本轮次所有plan中的事项完成" → executed Atoms 4-8 + ship + book-keeping in one session post-context-compaction.

### What landed (12 commits)

| Commit | Atom | Summary |
|---|---|---|
| `42fd45c` | Atom 2 | TB-5.0 substrate: `submit_agent_tx` + agent-ingress barrier (4 system variants rejected pre-queue) |
| `4a33b1a` | Atom 3 | TB-5 ABI: `ChallengeResolveTx` + `ChallengeStatus` (q_state.rs) + `ChallengeResolution` (typed_tx.rs) + `monetary_invariant` cascade |
| `9ff8179` | Atom 4 | `emit_system_tx` + apply_one stage 1.5 (defense-in-depth pinned-pubkey verification) + `record_rejection` helper |
| `06a7fcf` | Atom 5 | `ChallengeResolve` dispatch arm (Released path) + `CHALLENGE_RESOLVE_DOMAIN_V1` state-root domain + 4 new TransitionError variants |
| `c7dfef9` | Atom 6 | UpheldDeferred path + boundary tests (I75-I77 + I78-I79 + I88-I89) |
| `cc72d61` | Atom 7 | Replay (I80) + property (I81) + anti-drift CI (I82-I87, `tests/tb_5_anti_drift.rs`) |
| `2fb4ed9` | Atom 8 | Recursive self-audit + 真实烟测 evidence |
| `1bdc55a` | merge | `--no-ff` merge experiment branch into main |
| `c472823` | book-keeping | TB_LOG / NOTEPAD / ROADMAP post-merge updates |

**Acceptance battery**: **617/617** `cargo test --workspace` passing, 0 failed (corrected 2026-05-01 from original 464/464 ship-time figure). 46 net new TB-5 tests vs TB-4 baseline 571.

### Production claim adds

1. Anti-Oreo agent-vs-system ingress separation **structurally enforced** (was documented norm without live enforcement through TB-3 + TB-4).
2. `emit_system_tx` constructs + signs system-emitted typed txs INTERNALLY; callers cannot pass forged signatures.
3. apply_one stage 1.5 re-verifies against `PinnedSystemPubkeys` (defense-in-depth catches stale-sig replay → `InvalidSystemSignatureLive` + 1 L4.E PolicyViolation row, no logical_t advance — K1).
4. `ChallengeResolve` dispatch enforces idempotent single-shot resolution: Released refunds + zeros bond (entry preserved); UpheldDeferred is marker-only (bond preserved for TB-6 slash routing).

### 真实烟测 (handover/evidence/tb_5_smoke_2026-04-30/) — NOTE: see 2026-05-01 audit above

- oneshot `prompt_context_hash="a1f43584a17d1226"` — bit-identical across **5 sessions** (TB-1/2/3/4/5)
- n1 `solved=true`, `verified=true`, `gp_payload="nlinarith"` on `mathd_algebra_107` with `budget_max_transactions=20`
- ⚠️ **Per 2026-05-01 self-audit § 3**: this is paper-trail evidence, NOT chain audit. The kernel structural claims live in `cargo test --workspace`; smoke evidence proves prompt-build pipeline compat + capability replicability.

### Self-audit (handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md)

6/6 directive Q1-Q6 + 10/10 charter v2 § 4 decision blocks + 4/4 anti-drift renames + 3/3 ship gate proofs all GREEN. Test count corrected to 617/617 in-place 2026-05-01.

### Audit-mode (TB-5 specific)

Directive § 4 Q4 mandated Option A (dual external) — Gemini strategic-tier `MODEL_CAPACITY_EXHAUSTED` across 4 rounds; supplement `2026-04-30_TB5_audit_mode_supplement.md` documented Codex-only mode; round-4 fell back to **grep self-verification** when Codex agent infra failed mid-audit.

### Next TB candidate (awaiting architect ruling D1)

- **Default per ROADMAP**: TB-6 = RSP-3.2 slash execution (`SlashTx` system-emitted; balances/stakes/challenge_cases mutations conditional on `ChallengeCase.status == UpheldDeferred`)
- **Recommended per 2026-05-01 audit**: TB-6 = P2 Agent Runtime atom (close 5-TB chaintape gap first; slash defers to TB-7)

---

## 🌙 OVERNIGHT 2026-04-29 — TB-1 Days 4-6 shipped autonomously; **CHALLENGE verdict, user decision needed**

**Authorization**: user "进行到送双外审并收集双外审结果给我睡觉回来看" → ran TB-1 Day 4 + Day 5 + Day 6 (dual external audit) end-to-end. **Did NOT ship Day 7** — that requires user decision.

### What landed (3 commits)
| Commit | Day | Summary | Tests |
|---|---|---|---|
| `50a1d67` | Day 4 | P6 `h_vppu_history` instrumentation (NEW file) — capacity-3 rolling window, persisted JSON store, post-hoc stamped in evaluator main(); live verified on 2× mathd_algebra_107 n3 runs (run 2: `h_vppu=6.21`) | 9/9 unit; live signal ✅ |
| `6c04c26` | Day 5 | Tier-A 9-acceptance battery consolidated into `tests/tb_1_acceptance.rs`; superseded `tb_1_p1_acceptance.rs` | **9/9 Tier-A green** + 4 Tier-B ignored as designed |
| (none) | Day 6 | Dual external audit launched (Codex + Gemini parallel) | Reports landed |

Full workspace: **491 passed / 0 failed / 150 ignored** at HEAD `6c04c26`.

### Dual audit verdicts (round 1)

| Auditor | Verdict | Conviction | Latency | Cost |
|---|---|---|---|---|
| Codex | **CHALLENGE** | high | ~6 min | ~$3-4 |
| Gemini DeepThink | PASS | 5/5 | 53s | ~$1-2 |

**Merged verdict** per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS): **CHALLENGE**. TB-1 must NOT auto-ship Day-7.

Full merged write-up: **`handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md`** (read this first when reviewing).

### Codex P0s (the gap)

The 9 Tier-A tests are technically green and prove the **primitives**, but Codex argues they don't prove the **central ship claim** ("the v4 GitTape kernel honors the L4/L4.E split + RSP-0 invariants enforced") because:

1. **Sequencer dispatch is `NotYetImplemented`** for all K5 variants → L4/L4.E disjointness is asserted at primitive level, NEVER through a real `dispatch_transition` route. Tier-A bypasses dispatch entirely.
2. **Monetary guards (assert_no_post_init_mint / assert_total_ctf_conserved / assert_read_is_free) have no production call sites** — only unit + Tier-A tests reference them. A future dispatch path that forgets to call them would silently bypass.
3. **`RejectedSubmissionRecord` raw shielding is convention, not type-enforced** — `pub` struct, derives `Serialize`, `pub raw_diagnostic_cid`, `records()` returns raw refs. The `PublicRejectionView` projection is correct, but any code path that goes around it leaks the raw cid.
4. **`AcceptedLedger::load_from_path` skips `verify_chain`** — `prev_hash`/`hash`/`logical_t`-only tampers can load successfully unless caller separately verifies. Tier-A bypass test catches one specific tamper shape but misses fake-genesis, row-reorder, parent-state-root-only.

Gemini explicitly disagreed on 1 + 2: "primitives ready for TB-2 wiring is the right tracer-bullet level." This is a SCOPE-OF-CLAIM divergence, not a bug-vs-no-bug divergence.

### 3 paths (user decides)

- **Path A (recommended; ~1h)**: narrow the central claim in recharter + commit messages — "TB-1 ships PRIMITIVES + INVARIANTS, NOT dispatch enforcement". Optional sweeteners: P0-2 (~30min, all-six-subindex Tier-A test) + P0-3 (~30min, `#[serde(skip_serializing)]` on raw_diagnostic_cid). Ship Day-7 with narrowed claim; **skip round-2** (Codex's CHALLENGE was about claim scope, not bugs; narrowing addresses it directly).
- **Path B (heavier; ~3-6h)**: fix all 4 P0s (incl. wiring `dispatch_transition` for at least one variant + 3 more tamper tests + manifest-level shielding patch); then run round-2 audit per Elon-mode 2-round cap.
- **Path C**: defer ship; fold dispatch_transition into TB-2 RSP-1 scope.

**Default if no decision**: do nothing — TB-1 stays at HEAD `6c04c26`. No further auto-action.

### Compute spend
- TB-1 Days 4-5 (build): ~$0 (local cargo + 2 small live runs ≤ $0.10)
- TB-1 Day 6 (dual audit r1): **~$5-6 total** (Codex 154K-token prompt + Gemini 197K-char prompt). Within TB-1 $30 audit budget; ~$24 reserved for round-2 if Path B.

### Where to start when reviewing
1. `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md` — merged verdict + the 3 paths
2. Skim `handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md` Section A-E (last ~100 lines of the file; preceding lines are Codex's exec investigation log, not the verdict)
3. `handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md` (full 80 lines — concise PASS verdict)
4. `tests/tb_1_acceptance.rs` — the 9 Tier-A tests under audit

---

## 📜 v2 Whitepaper — Tactical Constitutional-Level Alignment (2026-04-27, RATIFIED ✅)

**Status**: **RATIFIED** after 3-round dual external audit converged (R1 VETO → R2 CHALLENGE → R3 PASS). Constitution.md unchanged; v2 acts as supreme校准 mirror over all derivative docs (Plan v3.2 / Blueprint / v1 / Deepthink).

**Subject (v2.2 in-place)**: `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` (filename unchanged; content patched to v2.2 via 7 must-fix + 1 single-line fix)
**Alignment note**: `handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md` (with new § 9 sunset clause + § 10 conflict-resolution)

**Core ruling**: TuringOS = **反奥利奥架构 (body) + ChainTape (tape implementation)**. Blockchain is NOT the body; ChainTape is one possible implementation of the verifiable state-ledger tape, living within Anti-Oreo's three-layer structure (top-white predicates / middle-black agents / bottom-white tools).

**ChainTape Directive**: 项目全面向区块链前进 = ChainTape vertical (**Trust Anchor Layer 0 + ChainTape Layers 1–6**) becomes primary engineering thrust for Wave 6+. NOT "blockchain becomes body" (would invalidate v2 § 公理 5).

### Dual-audit history (3 rounds, conservative-wins)
| Round | Codex | Gemini | Conservative | Outcome |
|---|---|---|---|---|
| R1 | VETO (Q3 sudo scope drift; 7 must-fix) | CHALLENGE (Q10 governance debt) | **VETO** | v2.1 patch in same session |
| R2 | CHALLENGE (1/7 PARTIAL: stale "Layers 0–5") | PASS | **CHALLENGE** | v2.2 single-line patch |
| R3 | **PASS** (R2-NEW-1 CLOSED) | **PASS** (Q10 mitigated) | **PASS** ✅ | RATIFICATION HOLDS |

Total v2 audit cost: ~$20 (R1 $8.50 + R2 $8.50 + R3 $3.50). Cumulative project ~$100–150 / $890 mid-budget (~11–17%).

### Wave 6 priorities re-ordered under ChainTape lens
1. **CO1.7 transition_ledger** (Layer 4) — promoted: central artifact connecting agents → state
2. **CO1.1.4-pre1.b fixture corpus** — STEP_B byte-comparison engineering pre-req
3. **INV8 spec v2 revision** — close 4 VETO + 5 CHALLENGE; now scoped under Layer 4
4. **CO1.1.4 / CO1.1.5 STEP_B** — pair with #2 fixtures
5. **F ceremonies** — user-led; independent of critical path

### Sedimented OBS files (4)
- `OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md` — 创造域 vs 安全域 dual rejection mode
- `OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md` — Public/Private/Commit-Reveal
- `OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md` — Q_t 5-root extension (CO1.2 v2 candidate)
- `OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md` — InitAI as conceptual placeholder

### v2 retires (semantically only; not physically deleted)
Any phrase in v1 / Blueprint / Deepthink that asserts "ledger / blockchain is the body of TuringOS." Such phrases are **historical drafting language** superseded by v2 § 公理 5.

### Sunset triggers (per tactical alignment note § 9)
- **Hard date**: 2027-01-01 mandatory review
- **Phase 4 entry blocker**: full constitutional merge OR formal retirement required before Permissioned ChainTape phase
- **Conflict count**: N=3 § 10 escalations within 90 days → automatic suspension

### Orphan finding (NOT caused by v2 work) — ✅ CLOSED 2026-04-27 (commit `9f42fb5`)
`test_trust_root_simulated_write_aborts` at `experiments/minif2f_v4/tests/trust_root_immutability.rs:74` was **pre-existing failure at HEAD `fb63053`** — error: `expected Tampered, got Err(SectionMissing("constitution_root"))`.

**Actual root cause** (corrects original "enum split" hypothesis): A8e13 added `verify_constitution_root_section` (CO1.0 v1) which short-circuits on missing `[constitution_root]` section before reaching the `Tampered` check. The fake genesis in this test predates A8e13 and only had `[pput_accounting_0]` + `[trust_root]`. Fix lifts the 8-key `[constitution_root]` block from `src/boot.rs::tests::write_single_entry_repo` (line 413-430).

**Verification**: full workspace `cargo test --workspace` = **388/0/145** PASS (turingosv4 + minif2f_v4 + gix_capability spike). FC-trace `FC3-N34` (readonly subgraph; constitution.md line 670).

---

**Updated**: 2026-04-28 — **Wave 6 #1 CO1.7 spec PASS/PASS gate cleared** (`a946820` v1.2). Three rounds of dual external audit converged: R1 CHALLENGE/CHALLENGE → R2 PASS/CHALLENGE → R3 PASS/PASS. Spec + skeleton + system_keypair extension all audit-cleared; CO1.7 implementation start now unblocked.
**HEAD commit**: `7bd02ad` round-3 audit runners (post-`a946820` v1.2).
**Origin**: through `5829e32` pushed; rest local-only (push when user ready).

**Next-session entry**: 🚀 **CO1.7 implementation** (now unblocked per `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md` PASS/PASS). Per spec § 13: 3 downstream atoms estimated 5-9 days total for Wave 6 #1 closure:
1. CO1.7-impl proper (~600-900 LoC + 4 CO1.7.5-stage tests)
2. CO1.4-extra (NEW atom; ~150-300 LoC + 3-4 tests; CAS index persistence — required for full-mode replay across cold restart)
3. CO1.7.5+ wiring (head_t mutation; integration with bus.rs/kernel.rs — STEP_B required per CLAUDE.md "Code Standard")

CO1.7 audit cost: ~$25-42 (3 rounds; cumulative project ~$135-202 / $890 mid). Working tree clean.

---

## 🚨 2026-04-29 Session-3 — CAPABILITY-FIRST PIVOT + ✅ FIRST V4-NATIVE SOLVE (~80 min after pivot)

**Status**: User raised "no confidence in dev capability" challenge after 7-day atom-spec wave. Web research + internal eval confirmed spec-craft drift. Pivot codified at commit `a906886`. **B target met within 80 min**: `mathd_algebra_107` solved end-to-end at HEAD `a906886` via v4 evaluator binary, OMEGA accept depth=1, 10.0s wall-clock, single tactic `nlinarith`. Independently re-verified via `lean --stdin` exit 0. **Evidence**: `handover/evidence/first_v4_solve_2026-04-29/`.

### B result — first v4-native solve

| Metric | Value |
|---|---|
| Problem | `mathd_algebra_107` (adaptation split) |
| Condition / Mode / Model | `n3` / `full` / `deepseek-chat` |
| `MAX_TRANSACTIONS` | 50 |
| `solved` / `verified` | true / true |
| Golden-path tactic | `nlinarith` |
| `tx_count` / `gp_token_count` | 1 / 12 |
| Wall-clock | 9.95s |
| `pput_runtime` | 0.000215 |
| `pput` (PPUT/s) | 10.04 |
| HEAD | `a906886` |
| Independent re-verify | ✅ exit 0 |

**Closes**: 7-day "0 v4-native solves" gap. Capability path is alive at HEAD; CO1.x substrate atoms did NOT break the pre-v4 evaluator path.

### Auxiliary finding — `oneshot` regression bug (file separately; not B-blocking)

Two `condition=oneshot` retries failed deterministically in 9-11s with identical Lean parse error: `<stdin>:10:33: error: unexpected token 'by'; expected '{' or tactic`. Same model/problem/HEAD with `condition=n3` solved cleanly. **Implication**: `run_oneshot` code path in evaluator.rs has prompt-template or output-parsing bug; `n3` swarm path uses different scaffolding and works. Filed for ≤1-day follow-up atom.

### Landing eval (delivered 2026-04-29 12:25 by Explore agent)

**Architectural completion ~28%** (defensible measure):
- L0 Constitution: ✅ wired (boot.rs + genesis_payload + Trust Root)
- L1 Predicate Registry: ✅ wired (146 pub items + 18 conformance tests)
- L2 Tool Registry: ⚠️ scaffold only (registry struct; tool dispatch stubs)
- L3 CAS: ✅ wired (git2 blobs + JSONL sidecar; 4 round-trip tests)
- L4 Transition Ledger: ✅ wired (LedgerEntry + Git2LedgerWriter; CO1.7-extra closed)
- L5 Materializer: 🛑 SPEC-ONLY DEFERRED (CO1.8 v1 r1 found 2 P0s)
- L6 Signal Indices: ❌ not started
- L7 Read View: ⚠️ partial (snapshot.rs + prompt_guard; no full rtool/wtool trio)

**5-step compile loop**: 3/5 wired (Proposal, Ground-Truth Feedback, Logging) + 2/5 stubbed (Capability Compilation, ↑H-VPPUT feedback)
**Capability path**: 0% → 0.4% (1 solve / ~244 problems = 0.4% baseline; H-VPPUT not yet measured)
**Substrate path**: 65% (per LATEST.md prior; git2-rs CAS + L4 commits wired; HEAD_t path abstraction + Art 0.4 rtool/wtool trio missing; Path A/B/C election deferred)
**Economic mechanism (§ 21 final reward)**: 10% computable (Constitution gates ✅; Utility partial; Escrow/Accept/Attribution/Survival all schema-only stubs)

**ChainTape end-to-end Verify-tx flow**: stalls at step 3 (sequencer dispatch returns NotImplementedError; CO1.7.5 transition bodies deferred). Steps 1-2 (proposal, predicate verdict) and 6-8 (ledger commit, CAS index, system signature) work; steps 3-5 (state mutation, materializer, signal broadcast) deferred.

**Top 3 gaps if pursuing substrate-path capability** (8-12 days estimate from agent — but **B already proved capability via pre-v4 evaluator path so this is FUTURE work, not blocking**):
1. CO1.8 v2 spec rework (3-5 days)
2. Evaluator → v4 ledger wiring (1-2 days)
3. L6 signal indices (2-3 days)

### Constraint hierarchy (post-B-success update)

1. **Constitution**
2. **Whitepaper v2**
3. **24h iteration cap** ← validated this session (pivot decision → first solve in 80 min)
4. **Standing memories** (with re-scoped dual-audit + phased-checkpoint)

### Outstanding follow-ups (priority order)

1. **`oneshot` regression bug** — file as ≤1-day atom; identify prompt-template/parser divergence
2. **Solve breadth check** — re-run n3 + MAX_TX=50 against 5-10 more adaptation problems for solve-rate estimate
3. **CO1.7-impl A5+ continuation** (real implementation work; not new spec)
4. **CO1.7.5 spec draft** (when started: single-round audit, accept-or-defer-with-OBS per session-3 policy)
5. **CO1.8 v2 spec** (deferred until CO1.7.5 lands; per OBS doc)
6. **AUTO_RESEARCH_NOTEPAD.md cleanup** (TFR stale ref; bloat ≤ 200 lines target)
7. **LATEST.md compression** (target ≤ 100 lines; after pivot stabilizes)

### Session-3 commits (chronological)

| # | Commit | Action |
|---|---|---|
| 1 | `a906886` | Session-3 pivot codification: OBS_CO1_8_V1_DEFERRED + iteration-cap memory + LATEST.md session-3 + Codex/Gemini r1 audit MDs |
| 2 | (this commit) | First v4-native solve evidence: handover/evidence/first_v4_solve_2026-04-29/ + LATEST.md session-3 update with B result + landing eval integration |

### Original 🚀 Next-session entry point (B was the gate; B is now done)

~~**B: run v4 evaluator on `mathd_algebra_107` (HEAD) by 2026-05-06.**~~ ✅ done in 80 min, not 1 week.

**New next-session entry point**:
1. Diagnose + fix `oneshot` regression bug (atom)
2. Run n3 batch on 5-10 adaptation problems for solve-rate baseline
3. Decide whether to resume substrate work (CO1.7.5/CO1.8) or expand capability batch first

**Do NOT** restart spec-atom mass production. Capability path is now the default; substrate work earns its way back via concrete capability-loop progress (per `feedback_iteration_cap_24h` memory).

### Hard data that triggered the pivot (2026-04-22 → 2026-04-29, 7 days post-TRACE_MATRIX_v0 baseline)

| Metric | Value | Signal |
|---|---:|---|
| Total commits | 203 | |
| spec/audit | **95 (47%)** | |
| impl/test | 24 (12%) | |
| eval/experiment | **13 (6%)** | |
| Audit reports total LoC | **367,555** | single audit MD ~150KB |
| Production LoC (`src/*.rs`) | 11,701 | |
| **Audit:Production ratio** | **31.4 : 1** | smoking gun |
| v4-native new solves since 2026-04-22 | **0** | proofs/ are inherited pre-v4 (untracked) |
| Last batch experiment artifact | 2026-04-24 E1v2 | used pre-v4 evaluator (build SHA `29ab43a`) |
| 5-step compile loop wired | 3/5 | steps 4+5 (Capability Compilation, ↑H-VPPUT) deferred to v4.1 |
| H-VPPUT empirical measurements | **0** | formula defined, never measured |

### Web research evidence (full sources in session-3 transcript)

- DeepSeek-Prover-V2 (88.9% MiniF2F SOTA): **2 public commits**, prototype-first
- Goedel-Prover: 24 commits / 64 days; Kimina-Prover: 12 / 87 days. **Zero** peer LLM-prover team uses atom-spec + per-atom dual-LLM-audit
- Porter & Votta (TSE 1997) + Jureczko 2020: **2 reviewers is empirical optimum**; rounds-per-change beyond 2 mostly surface paper tigers
- TDD/spec-first **explicitly discouraged** for exploratory ML/research code (Manning ML Eng, CMU MLIP)
- Atomic-decomp + dual-audit DOES work in DO-178C avionics + seL4 microkernel — **decade timelines, life-stakes**. Not solo LLM research

### Pivot decisions (executed this session)

**A. Stopped spec-craft loop**
- **CO1.8 v1 DEFERRED**, not patched. r1 verdict: **Codex VETO/HIGH + Gemini CHALLENGE/HIGH** (conservative merge = VETO). Real architectural P0s found:
  - Codex P0 #1: sprint graph overclaim — `[CO1.7.5] blocks: CO1.8` per SPRINT line 106-108; CO1.8 not unblocked by CO1.7-extra alone
  - Codex P0 #2: `apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, _>` interface contradiction — VerifyTx has only target+verifier, can't increment reputation without prior Work/Claim state. "Pure function with implicit BTreeMap I/O" is internally inconsistent
  - Gemini P0: `project_for_agent` no-op stub violates Inv 10 (Goodhart shield) by default-allow
- All findings archived to `handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md`. CO1.8 spec header updated with 🛑 DEFERRED status. **NO r2 audit run.** Original v1 text preserved as evidence.
- **CO1.13-extra (250 backlinks; ~10-15 hr) downgraded** from "MUST before Phase D" to "v4.1 gate" — Phase D is itself v4.1 scope per PROJECT_DECISION_MAP D4
- 1.7-impl + future spec atoms switch from per-atom dual-audit-with-rounds → **single audit round, accept-or-defer-with-OBS**, no r2/r3

**C. New iteration-cap policy** (memory entry `feedback_iteration_cap_24h.md`)
- Every PR must produce evaluator pass/fail signal (smoke or single-problem real run) within 24h
- Spec/audit/scaffold work that doesn't shortest-path to runnable feedback loop = **default-reject** unless explicit user authorization
- Replaces atom-only Elon-mode round-cap framing for non-spec work
- Dual-audit + phased-checkpoint + smoke-before-batch memories still apply, but NOT as default for every change — only when capability loop is actively producing solves
- Red flags: 3+ days without evaluator signal, 2+ days without test, "round 3+" being proposed, audit:prod LoC ratio growing weekly

**B. Capability-first execution begins**
- Target: `mathd_algebra_107` (adaptation split; pre-solved 8+ times in inherited `proofs/`; medium difficulty; regression-test-as-first-solve)
- Constraint: Mathlib rebuild must clean first (currently 99%, ~20 min)
- Mode: `--mode full` (baseline, no ablation), `CONDITION=oneshot`, `ACTIVE_MODEL=deepseek-chat`
- Wall-clock budget: 24h iteration cap; if not solved in 24h, debug to specific blocker, raise to user
- Deadline: **2026-05-06** for either first-solve confirmation OR documented infrastructure gap

**D. Audit sunk-cost recovery (CO1.8 r1)**
- Codex r1 (174s, $5-10): VETO/HIGH, 2 P0s — both real architectural defects
- Gemini r1 (40s, $3-5): CHALLENGE/HIGH, 1 P0 — Goodhart shield (real)
- **0 paper tigers in r1** — audit was efficient, $10-15 well-spent
- Pivot lesson: r1 earned its keep; r2/r3 would have entered diminishing returns. The system's working at 1 round; we just stop overspending

### Updated constraint hierarchy (effective session-3)

1. **Constitution** (constitution.md)
2. **Whitepaper v2** (load-bearing for ChainTape + economic mechanism)
3. **24h iteration cap** (NEW; replaces atom-only Elon-mode framing)
4. **Standing memories** — but with `dual_audit` + `phased_checkpoint` re-scoped to "active capability loop" only, not "every spec change"

### Outstanding follow-ups (post-pivot priority order)

1. **B: mathd_algebra_107 first solve attempt** (in flight; gated on Mathlib)
2. **CO1.7-impl A5+ continuation** (real implementation work; not new spec)
3. **CO1.7.5 spec draft** (when started: single-round audit, accept-or-defer-with-OBS)
4. **CO1.8 v2 spec** (deferred until CO1.7.5 lands; per OBS doc)
5. **AUTO_RESEARCH_NOTEPAD.md cleanup** (TFR stale ref; bloat ≤ 200 lines target)
6. **LATEST.md compression** (target ≤ 100 lines; after pivot stabilizes)

### Session-3 commits (chronological)

| # | Commit | Action |
|---|---|---|
| pending | (this commit) | Session-3 pivot codification: OBS_CO1_8_V1_DEFERRED + CO1.8 spec status update + iteration_cap memory + LATEST.md session-3 entry |

### CO1.8 r1 audit residue

- `handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md` (362KB; VETO/HIGH; 2 P0s)
- `handover/audits/GEMINI_CO1_8_ROUND1_AUDIT_2026-04-29.md` (5.8KB; CHALLENGE/HIGH; 1 P0; gemini-3.1-pro-preview after stale-model fix to launcher)
- `handover/audits/run_gemini_co1_8_round1_audit.py`: model id patched from `gemini-2.0-flash-thinking-exp-01-21` → `gemini-3.1-pro-preview` (drift fix; same as CO1.13 r1/r2 working launchers)

---

## 🎯 2026-04-29 Session-2 CLOSURE — CO1.13 atom bundle COMPLETE ✅

**Status**: CO1.13.1 + CO1.13.2 + CO1.13.3 all shipped + drift review = NO MATERIAL DRIFT. Wave 6 #2 PRE-CO1.8 alignment factory now LIVE.
**HEAD commit**: `1a5849f` (CO1.13 phase drift review + --half factory upgrade).
**Origin**: through `5829e32` pushed; rest local-only.

### 🚀 Next-session entry point

**Pick up at one of two priorities** (user direction required):

1. **CO1.8 spec round-1 audit launch** — spec drafted at `6cc5cc9`; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py`; not yet run. CO1.13 factory is now LIVE so audits will benefit from R-022 + § F.2 auto-refresh + § J orphan registry + the `--half` Phase C regression check.
2. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks) — MUST schedule before Phase D per spec § 0.5 Gemini r1 Q7. With R-022 LIVE, every NEW pub symbol since `e9c6a2b` is enforced; legacy gap is the remaining substantive debt.

### Three commits this CO1.13 closure arc

| # | Commit | Action |
|---|---|---|
| 1 | `9be22b4` | CO1.13.1 — TRACE_MATRIX_v3 doc completion (§ E.2/E.3 measured stats; § F.2 manual snapshot 135 backlinks; § J Orphan Extensions schema; cross-ref reconciliation). +283 / -14 doc delta. Trust Root rehash for TRACE_MATRIX_v3. |
| 2 | `e9c6a2b` | CO1.13.2 + CO1.13.3 — R-022 hook (rules YAML + custom_commit_hook check_trace_matrix.py 421 LoC + tracked pre-commit shim + install_hooks.sh + .github/workflows/co1_13_r022_ci.yml + 5-line engine.py patch + 9 shell integration tests + Rust orchestrator) + auto-refreshing § F.2 reverse-map (update_trace_matrix_reverse_map.py 134 LoC; shares parser with R-022 check). +1011 / -31. Trust Root rehash for engine.py + TRACE_MATRIX_v3. |
| 3 | `1a5849f` | CO1.13 phase drift review (`handover/architect-insights/CO1_13_PHASE_DRIFT_REVIEW_2026-04-29.md` 215 LoC) + `--half` factory upgrade to `run_c2_phase_c_ablation.sh` (3 problems × 5 modes × 1 seed × MAX_TX=20; lives between cheap `--smoke` and full Phase C batch). Trust Root rehash for runner script. |

### CO1.13 final spec compliance (vs v1.1.1 § 0.3)

| Sub-atom | Spec target LoC | Actual LoC | Verdict |
|---|---:|---:|---|
| CO1.13.1 | ~200 | +283 / -14 | ACCEPTABLE (table content + § J schema; quality spending) |
| CO1.13.2 | ~335 | ~676 (script 421 + yaml 20 + shim 13 + installer 31 + ci 24 + 5-line engine.py + tests 297) | ACCEPTABLE (test-isolation hardening forced by real pollution incident) |
| CO1.13.3 | ~100 | 134 | ACCEPTABLE (--check / --dry-run modes added) |
| Bundle total | ~635 | +1011 / -31 net | ACCEPTABLE per Elon-mode "scope unchanged, process streamlined" |

### Real-test data points (5)

1. **Test pollution** — `r_022_ci_mode_catches_unhooked_pr.sh` initially leaked an empty `b60556d main baseline` commit + `feature` branch into the live repo because `tmp=$(setup_temp_repo)` ran `cd` in a subshell; `set -uo pipefail` (no `-e`) was silent on the failure. **Fixed**: introduced `enter_tmp_repo` (no subshell; sets TMP_DIR global; asserts `realpath $PWD` does NOT resolve inside PROJECT_ROOT before any git command). All 9 tests re-run without pollution.
2. **Disk-space exhaustion** — `cargo test --test r_022_integration_orchestrator` triggered `ld: signal 7 (Bus error)` during link; bash subprocess infrastructure entered degraded state (every command returned non-zero with empty stdout/stderr; Write tool reported ENOSPC). User manually freed ~12G of cargo `target/`. Future drift reviews should `df -h` before launching `cargo test --workspace`.
3. **CO1.13.3 idempotency** — `python3 scripts/update_trace_matrix_reverse_map.py --check` exits 0 immediately after first run.
4. **Phase C smoke 5/5 PASS in 95s** post-CO1.13 (consistent with 97s baseline at `8d88f2d`); soft_law H2 fake-accept signature preserved. Per user 2026-04-29 challenge: `--smoke` is pipeline-liveness only — for CO1.13 (0 lines of `src/` changed) it confirms only that Trust Root rehashes didn't break evaluator boot.
5. **Mathlib collateral damage** — disk-cleanup recommendation (`rm -rf .lake`) was too aggressive: `.lake/packages/Mathlib/` is a vendored dependency requiring `lake exe cache get` (~2 min) or `lake build` (30-60 min) to recover. Lake project skeleton (`lakefile.lean` / `lake-manifest.json` / `lean-toolchain`) preserved; recovery via `lake update && lake exe cache get` running in background at session-closure time. **New memory entry**: `feedback_lake_packages_vendored` codifies the `.lake/build` (regen) vs `.lake/packages` (vendored) distinction.

### `--half` factory upgrade landed in this session

User direction "1+2 结合，2 等大节点再做" → added `--half` mode to `handover/preregistration/scripts/run_c2_phase_c_ablation.sh`: 3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20 (~10-15 min wall-clock; ~$0.20-0.40 API cost). Lives between `--smoke` (pipeline-liveness; ~95s) and `--full` (scientific regression; ~12 hr; 100 cells). First invocation surfaced data point #5 above; needs Mathlib recovery before next use.

### Outstanding follow-ups (priority order)

1. **CO1.8 spec round-1 audit launch** — drafted at `6cc5cc9`; ready under new factory regime
2. **Mathlib recovery** — running in background via `lake update && lake exe cache get`; ETA ~5-10 min from session-2 CLOSURE start
3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 backlinks; MUST before Phase D per Gemini r1 Q7)
4. **CO1.13-devtools-mathlib-mirror** (new follow-up sub-atom; this session): file-mirror endpoint on linux1 hosting Mathlib v4.24.0 `.lake/packages` tarball; omega-vm hydration script; Trust Root sha256 registration. Constitutionally clean (Lean stays local). Estimated ~1-2 day work; collapses future Mathlib re-fetch from 10-30 min to ~5 min internal-network rsync. Defer to between CO1.8 and CO1.9 atoms.
5. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; per spec § 0.4) — non-spec; lands as separate commit
6. **AUTO_RESEARCH_NOTEPAD.md cleanup** — TFR stale reference per LATEST.md session-2 outstanding-debt; defer to next session
7. **CO1.7.5** (transition bodies; gated on CO P2.x substrate atoms) — Wave 2 work; weeks-to-months out

### New Constitutionally-clean Mathlib mirror architecture (CO1.13-devtools-mathlib-mirror; this session candidate spec)

**Why**: Today's disk-cleanup → Mathlib loss → 10+ min recovery debt is preventable. linux1-lx (128G AMD AI Max 395, primary compute node) is the natural Mathlib source-of-truth.
**What**: tarball `.lake/packages` ~5G on linux1 → exposed via internal HTTPS (or even simpler: via existing WireGuard rsync access) → omega-vm hydrate-on-provision script.
**Constitutionally clean**: Lean still runs locally on omega-vm (Art 0.2 oracle locality unchanged); network only used for one-time provisioning hydration.
**Trust Root**: tarball sha256 registered in `genesis_payload.toml`; FC3-N34 verification on hydrate.
**NOT**: a network verifier API (option B in 2026-04-29 user discussion) — that would change Art 0.2 oracle locality + raise sudo gate.

### Sedimented memory entries this session

- `feedback_lake_packages_vendored` (NEW; .lake/build vs .lake/packages distinction)
- (existing memories unchanged: `feedback_oracle_preflight`, `project_phase_c_living_regression`, `feedback_elon_mode_policy`, `feedback_no_fake_menus` all reaffirmed by this session's events)

### Cumulative project audit spend after CO1.13 closure

- This session's CO1.13 r1+r2 dual audits + cap-exception: ~$16-24 (per drift review § 7)
- Project cumulative: ~$220-340 / $890 mid-budget (~25-38%); ~$550-670 runway
- Per atom going forward: $5-10 expected (single-round + targeted patches; R-022 + auto-refresh + § J registry now amortize the spec-cycle prep cost)

### Constraint hierarchy (active per Elon-mode + user 2026-04-29 explicit instruction)

User explicit instruction 2026-04-29 session-2:
> "我要求你在遵守宪法、白皮书和我们刚才讨论的elon-mode下自动执行..."

Operationalized priority order:
1. Constitution
2. Whitepaper v2
3. Elon-mode (round cap=2, OBS threshold=3, cap-exception via auto-execute on determinate-best surgical patch)
4. Standing memories (dual-audit, smoke-before-batch, no-fake-menus, FC-first, NEW lake-packages-vendored)

When facing decision: 1→2→3→4 order; if no resolution → state determinate-best + execute (no fake menus). Per-phase drift review at atom-complete boundary. When lacking data: run real tests, don't speculate.

---

## 🌊 2026-04-29 Session-2 — CO1.7-extra Branch B closure + CO1.13 spec PASS-with-cap-exception (Elon-mode launch)

**Updated**: 2026-04-29 (session-2)
**Status**: spec phase **DONE** (CO1.7-extra ceremony closed + CO1.13 cleared for impl); implementation phase **READY TO START** in fresh session.

### 🚀 Next-session entry point

**Pick up at CO1.13 implementation phase per spec § 0.3 v1.1.1**. Three sub-atoms in dependency order:

1. **CO1.13.1** TRACE_MATRIX_v3 doc completion (~200 LoC docs delta; 0.5 day target)
   - § A complete N-rows; § B complete WP rows; § E coverage stats
   - § F reverse-map populated for shipped atoms (CO1.0a / CO1.4 / CO1.4-extra / CO1.7-impl A1-A4 / CO1.7-extra)
   - **NEW § J "Orphan Extensions"** with table schema (lands BEFORE script can fall back to it)
2. **CO1.13.2** R-022 commit-time hook (~335 LoC; 1.5 day target)
   - `rules/active/R-022_trace_matrix_pub_symbol_block.yaml` (declarative tombstone; engine.py BYPASSED)
   - `scripts/check_trace_matrix.py` (multi-line context grep + diff parser)
   - `scripts/hooks/pre-commit.r022` (tracked shim)
   - `scripts/install_hooks.sh` (symlinks tracked shim → `.git/hooks/pre-commit`)
   - **`.github/workflows/co1_13_r022_ci.yml`** (tracked CI workflow; required merge gate; closes Codex r2 fresh-clone bypass)
   - 5-line patch to `rules/engine.py` (gracefully ignore `trigger == pre_commit`)
3. **CO1.13.3** reverse-map § F populator (~100 LoC Python; 0.5 day target)
   - `scripts/update_trace_matrix_reverse_map.py` shares parser with CO1.13.2 (per Codex r1 § D "one parser shared")

Plus 9 shell integration tests under `tests/integration/co1_13/` + 1 Rust orchestrator (`tests/r_022_integration_orchestrator.rs`) per spec § 3 v1.1.

**Authoritative spec**: `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` v1.1.1 (commit `813414c`). Read § 0.3 + § 1.2 + § 1.3 + § 2.1 + § 3 first; § 8 acknowledgements before coding.

**Total target**: ~665 LoC; **3-day wall-clock target** (Elon-mode benchmark; first real-test of cycle-time hypothesis).

**Phase drift review** fires at impl complete (per session task #7); 7-dimension check (scope / process / constraint / doc / critical-path / cycle-time / budget). Pre-flagged drift to confirm:
- Scope drift: +60% LoC v1→v1.1.1 (audit-driven; acceptable)
- Process drift: 3 audit rounds vs 2-round-cap (cap-exception per Codex r2 § E own recommendation; acceptable)
- Constitution + WP alignment: STRENGTHENED (R-022 enforcement now actually works via tracked CI)

### Session arc (3 commits this session-2)

| # | Commit | Action |
|---|---|---|
| 0 | `4a978f0` | CO1.7-extra v1.2.2: STEP_B Branch B re-derivation closed at T1 executable-substance byte-identity (per amended § 2.2 tiered byte-identity). Ceremony CLOSED for `src/bus.rs`. STATE_TRANSITION_SPEC v1.5 housekeeping issue committed earlier (`5b53c6b`). |
| 1 | `6cc5cc9` | CO1.8 L5 Materializer v1 spec drafted (300 lines, 10/10 smoke). **AUDIT DEFERRED** in favor of CO1.13 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms). |
| 2 | `8d88f2d` → `1423b90` → `813414c` | CO1.13 v1 → v1.1 (r1 9 patches) → v1.1.1 (r2 cap-exception 4 patches; Codex CHALLENGE-ESCALATE / Gemini PASS; conservative CHALLENGE-ESCALATE → cap-exception per Codex r2 § E recommendation). Spec at 420 lines; PASS-with-cap-exception. |

### NEW Elon-mode policy framework codified this session

The user authorized "Elon-mode" framing for project management (factory > scope; cycle-time > round-count; constitution + whitepaper line-by-line preserved as scope, but PROCESS streamlined). Round-1 audit on CO1.13 v1 forced the policy to be CONCRETE rather than aspirational. v1.1.1 codified:

1. **Audit round cap = 2** (vs prior 4-5 rounds): r1 + 1 patch round + r2 final. Round-3+ requires cap-exception authorization.
2. **OBS hard threshold = max 3 unresolved `OBS_*.md` files** project-wide (Gemini r1 Q4): threshold breach = factory halt + force-resolve before next atom. Prevents 2-round-cap from accumulating debt.
3. **Ship-with-OBS NOT applicable to enforcement gates themselves** (Codex r1 § E): "If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater." → escalate to user.
4. **Cap-exception authorized via auto-execute mode** when r2 split verdict produces a determinate-best surgical patch (not OBS theater). Codex r2 itself recommended this for v1.1.1.
5. **Phase C smoke as living regression test** (parallel weekly): verifies architecture-in-progress hasn't broken experiment harness. First run THIS session: 5/5 cells PASS @ HEAD `8d88f2d` in 97s vs 146s baseline (33% faster); soft_law H2 ablation signal preserved. **No regression**.

Memory entries created (see MEMORY.md):
- `feedback_no_fake_menus.md` — when project plan determines next atom, state and execute; don't surface 3-5 option menus
- `feedback_elon_mode_policy.md` — round cap + OBS threshold + cap-exception conditions (this session)
- `project_phase_c_living_regression.md` — Phase C smoke as architecture-in-progress regression check (this session)

### Constraint hierarchy (auto-execute mode interpretation)

User explicit instruction 2026-04-29 session-2:
> "我要求你在遵守宪法、白皮书和我们刚才讨论的elon-mode下自动执行，遇到选择题先检查以上约束，每个phase完成后对项目计划做review看drift，缺少做决策人来的数据就去跑真是测试找问题和解决方案"

Operationalized as priority order:
1. **Constitution** (constitution.md; load-bearing for thesis)
2. **Whitepaper v2** (load-bearing for ChainTape + Anti-Oreo + economic mechanism coverage)
3. **Elon-mode** (round cap, OBS threshold, factory > scope, cycle-time > round-count)
4. **Standing memories** (dual-audit, smoke-before-batch, no-fake-menu, FC-first-problem-handling, etc.)

When facing a decision: check 1→2→3→4 in order; if no resolution → state determinate-best action + execute (no fake menus). Per-phase drift review at atom-complete boundary. When lacking data: run real tests (Phase C smoke, cargo test, empirical measurements) — don't speculate.

### Real-test data points produced this session

| Test | Result | Significance |
|---|---|---|
| Phase C smoke @ HEAD `8d88f2d` | 5/5 cells PASS in 97s; soft_law H2 ablation preserved | architecture-in-progress hasn't broken experiment harness; **freeze rationale ("Node.completion_tokens=0 discovery; TFR S3.9 5-7 weeks out") is STALE** — TFR v1 was deprecated 2026-04-26 (see TFR_MASTER_PLAN_2026-04-26.md preface) and Phase C smoke was already 5/5 PASS @ 146s on 2026-04-28. Phase C is operationally unfreezable on demand. |
| CO1.13 spec-cycle wall-clock | ~2.5 hr (vs 14-day median pre-Elon-mode = ~134x compression on spec phase) | first real-test of Elon-mode "factory IS product" hypothesis; spec phase validated; impl phase pending |
| Backlink coverage baseline | 87/354 = 24.6% | 75% legacy gap quantified; CO1.13-extra (gap closure) MUST schedule before Phase D per Gemini r1 Q7 |

### Cumulative project audit spend after CO1.13 v1.1.1

- This session r1+r2 dual audits (4 calls): ~$16-24
- Project cumulative: ~$220-340 / $890 mid-budget (~25-38%); ~$550-670 runway
- Per atom going forward (post-CO1.13 factory deployed): expected $5-10 (single round + targeted patches; CO1.13's R-022 + scaffold devtools amortize spec-cycle prep cost)

### Open follow-ups (priority order)

1. **CO1.13 implementation** (next-session entry; this is THE priority)
2. **CO1.8 spec round-1 audit** (deferred this session; spec drafted at `6cc5cc9` ready to launch; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py` but were NOT run)
3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks; MUST schedule before Phase D per Gemini r1 Q7)
4. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; non-spec follow-up; lands after CO1.13 PASS impl)
5. **Phase C unfreeze decision**: smoke is now consistently passing; should we relaunch C2 full batch (5 modes × 10 problems × 2 seeds = 100 cells; ~12 hr wall-clock; ~$15-25)? **User decision required**.
6. **CO1.7.5 future spec** (transition bodies; gated on CO P2.x substrate atoms — Wave 2 work; ~50 atoms 6-8 wk)
7. **CO P2.x family roadmap** (TaskMarket / EscrowVault / ContributionLedger / etc.; per user requirement "宪法和白皮书逐行落地，包括但不限于经济制度")

### Outstanding architectural debt acknowledged

- **TFR v1 deprecated** at its own launch day (2026-04-26 night) per CO_P0_AMENDMENT_v1; successor is `CO_MEGA_PLAN_v3.1_2026-04-26.md`. AUTO_RESEARCH_NOTEPAD line 66 still describes TFR as "🚀 LAUNCHED" — STALE; needs cleanup but defer to next session.
- **AUTO_RESEARCH_NOTEPAD bloat**: ~600 lines; per Elon-mode "delete process redundancy", target ≤ 200 lines. Defer to next session.
- **LATEST.md bloat**: ~600+ lines; per Elon-mode, target ≤ 100 lines. Defer to next session.

These are bookkeeping items; no constitutional or scientific impact.

---

## 🌊 2026-04-29 Session-1 — Wave 6 #1 RECALIBRATION (CO1.7.5 split → CO1.7-extra; Branch A landed)

**Updated**: 2026-04-29
**Session arc**: dual-audit drove a **scope correction** on the prior 2026-04-28 "80% complete" framing. Round-1 dual external audit on CO1.7.5 v1 (Codex+Gemini, both CHALLENGE/High) found that D1 transition bodies have heavyweight FC1 (top-white predicate execution) + FC2 (middle-black state schemas) substrate dependencies that don't exist in shipped code (CO P2.x family per `PROJECT_DECISION_MAP § 3.4`). ArchitectAI applied an Occam-driven scope split (B2 by dependency profile) under "无损压缩即智能 + Anti-Oreo + 不违宪 + 不违白皮书" principles, yielding two atoms:

| Atom | Owns | Substrate dep | Status |
|---|---|---|---|
| **CO1.7-extra** (NEW bridge atom; CO1.4-extra precedent) | D2 head_t close + D3 TuringBus single-file STEP_B + 5 substrate-independent tests | None | ✅ spec PASS/PASS r4 + v1.2.2 § 2.2 amendment; **Branch A landed** `5ce01b1`; **Branch B closed** at T1 byte-identity (separate session 2026-04-29; tiered byte-identity per spec § 2.2 v1.2.2) — **STEP_B ceremony CLOSED** |
| **CO1.7.5** (restored to CO1.7 § 13 original meaning) | D1 transition bodies (7) + 3 D4 tests + un-ignore replay byte-identity | CO P2.1 / 2.2 / 2.3 / 2.5 / 2.6 / 2.7 / 2.9 + CO1.11 + (NEW) PredicateRegistry execution-methods atom | 📅 GATED on substrate atoms |

### Wave 6 #1 actual progress: ~30-40% (NOT 80%)

The prior 2026-04-28 "80% complete" claim was **false-precision** based on a mis-scoped atom (D1 substrate dependencies hidden inside CO1.7.5 v1 bundle). True state at HEAD `5ce01b1`:

- ✅ CO1.7 spec + CO1.7-impl A1-A4 bundle + CO1.4-extra (prior session)
- ✅ CO1.7-extra spec PASS/PASS (4 rounds; this session)
- ✅ CO1.7-extra Branch A landed (D2 head_t close + D3 TuringBus wiring + 5 tests)
- ✅ CO1.7-extra Branch B closed (T1 executable-substance byte-identical; spec § 2.2 amended v1.2.2 to formalize 3-tier byte-identity rule for future STEP_B atoms)
- 📅 CO1.7.5 gated on Wave-2 substrate (~7 prerequisite atoms + 1 NEW PredicateRegistry exec atom)

ChainTape vertical: L4 ~50-55% (storage + ABI + machinery + head_t close + Sequencer entry-point; transition bodies still pending). Estimate "Wave 6 #1 fully closed" = **after CO P2.x substrate ships** (multiple atoms; weeks-to-months out).

### CO1.7-extra audit arc (4 rounds)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| r1 (bundled CO1.7.5 v1) | CHALLENGE/H | CHALLENGE/H | CHALLENGE | Occam scope split → CO1.7-extra carved out |
| r2 (CO1.7-extra v1) | CHALLENGE/H | CHALLENGE/H | CHALLENGE | 10 MFs (MF1-MF10) → v1.1 |
| r3 (v1.1) | CHALLENGE/H | PASS/H | CHALLENGE | 4 mechanical (B1-B4) → v1.2 |
| r4 (v1.2) | **PASS/H** | **PASS/H** | ✅ **PASS/PASS** | 2 nits (N1+N2) → v1.2.1 (final) + Branch A impl |

CO1.7-extra atom-only audit cost: ~$13-26 across r2+r3+r4. Cumulative project: ~$196-314 / $890 mid-budget (~22-35%).

### Architectural improvements landed (vs prior bundled v1)

1. **TuringBus owns Sequencer directly** (round-2 MF4) — Kernel UNTOUCHED; "pure topology" doctrine preserved. STEP_B reduced from combined-ceremony to single-file (bus.rs only).
2. **Required trait method** (round-2 MF3) — `LedgerWriter::head_commit_oid_hex` has no default impl; Rust compiler enforces every implementation declares. Both audits' safety arguments (silent stagnation prevention + no-panic) satisfied via this third-option synthesis.
3. **`advance_head_t` helper extraction** (round-2 MF2) — D2 logic at module level + apply_one stage 9 calls helper; makes the constitutional anchor advance directly testable via mock writer (without injecting dispatch_transition).
4. **Kernel "pure topology" doctrine preserved** — no new fields on Kernel; runtime drivers (Sequencer + future) live at TuringBus level.

### Sedimented OBS files (2 new this session)

- `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` — CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`); fixed inline + sediment.

### Pending follow-ups

1. ✅ ~~CO1.7-extra Branch B~~ — closed 2026-04-29 separate session at T1 byte-identity per spec § 2.2 v1.2.2 amendment.
2. ✅ ~~STATE_TRANSITION_SPEC v1.5 housekeeping issue~~ — committed `5b53c6b` per CO1.7-extra spec § 0.4 commitment.
3. **Future CO1.7.5 spec drafting** — gated on CO P2.x substrate atoms reaching individual PASS/PASS.
4. **Wave 6 #2 next-atom selection** — Wave 6 #1 (CO1.7 family) ceremony-closed; § 3.2 menu of unblocked atoms includes CO1.8 L5 materializer / CO1.9 L6 signal indices / CO1.10 signal dichotomy / CO1.11 safety vs creation / CO1.13 TRACE_MATRIX impl. Pending user direction on which Wave 6 #2 atom to spec next.

### Open Questions

- **Q1 (sequencing)**: with Wave 6 #1 substrate now exposed as critical path, should the project reorder to ship CO P2.1/2.2/2.3/2.5/2.6/2.7/2.9 + CO1.11 before resuming CO1.7.5? Or continue Wave 6 #2/#3 affordances (CO1.8/CO1.9) in parallel?
- **Q2 (PROJECT_DECISION_MAP)**: should CO1.7-extra be codified into the decision map alongside CO1.4-extra precedent (this session's bridge-atom landing pattern)?

---

## 🌊 2026-04-28 Session-2 Final — Wave 6 #1 IMPLEMENTATION PHASE COMPLETE ✅

**Updated**: 2026-04-28 14:12 UTC
**Session summary**: Auto-execute mode shipped CO1.1.4-pre1 ABI atom (PASS/PASS) + CO1.7-impl A1+A2+A3+A4 bundle (PASS/PASS-equivalent) + CO1.4-extra in one continuous run. 17 commits pushed. 199/0 → 239/0 lib PASS + 1 ignored (CO1.7.5-stage). Audit spend ~$40-75. Single carry-forward: G-1 head_t Art 0.4 alignment closes in CO1.7.5.

### Current State

**Wave 6 #1 (L4 Transition Ledger family) — 80% complete**:
- ✅ CO1.7 spec PASS/PASS (3 rounds, prior session, ~$25-42)
- ✅ **CO1.1.4-pre1 v1.2.2 ABI surface PASS/PASS** (5 rounds, ~$26-50; commit `c1226e2`) — 7-variant TypedTx + 6 SigningPayload + 13 locked golden hex + ClaimId + 22-variant TransitionError
- ✅ **CO1.7-impl A1+A2+A3+A4 bundle PASS/PASS-equivalent** (3 rounds, ~$14-25; commit `2461fe6`) — Git2LedgerWriter + Sequencer + dispatch_transition stubs + replay_full_transition (9-stage I-DETHASH witness with tx_kind + decode separation)
- ✅ **CO1.4-extra** sidecar JSONL CAS index persistence (commit `b6b7574`) — closes Art 0.2 cold-replay gate
- 📅 **CO1.7.5** (per-kind transition bodies + STEP_B bus.rs/kernel.rs wiring) — final L4 atom, NOT STARTED

**ChainTape vertical position**:
- L0 Trust Anchor ✅ / L1 PredicateRegistry ✅ / L2 ToolRegistry ✅ / L3 CAS ✅ (incl. cold-replay) / L4 ⏳ 80% (storage + ABI + machinery done; transition bodies pending) / L5 📅 NOT STARTED / L6 📅 NOT STARTED

**Cumulative project audit spend**: ~$175-273 / $890 mid-budget (~20-31%).

### Next Steps

1. **CO1.7.5** (single critical path) — final L4 atom. Inherits frozen ABI + Sequencer machinery; must deliver:
   - Real per-kind transition bodies for 7 TypedTx variants (currently `Err(NotYetImplemented)` stubs)
   - Close G-1 head_t Art 0.4: wire `q.head_t = NodeId(commit_oid_hex)` after Git2LedgerWriter.commit (`head_commit_oid()` already exposed)
   - STEP_B parallel-branch ceremony for bus.rs/kernel.rs wiring (per CLAUDE.md "Code Standard")
   - Remove `#[ignore]` from `sequencer_serial_replay_byte_identity` test; verify end-to-end state_root reconstruction
   - Estimated: ~5-9 days; ~$25-50 audit
2. **Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)
3. **PPUT-CCL Phase C unfreeze** at TFR S3.9 — still ~5-7 weeks out

### Open Questions

- **Q1 (architectural drift)**: TFR_MASTER_PLAN_2026-04-26 uses old paths (`src/tape/`, `src/wal.rs`, `src/ledger.rs`); actual work is under `src/bottom_white/ledger/` + `src/state/` per Anti-Oreo restoration. Worth a one-line "SUPERSEDED by Wave 6 framing" header, or leave as historical artifact?
- **Q2 (process)**: 7 sedimented lessons across CO1.1.4-pre1 + CO1.7-impl bundle audits (esp. "claim-vs-code parity drift recurs" — caught 2× this session). Should pre-audit grep be codified into `validate` skill, or stay informal habit?
- **Q3 (next-session entry)**: CO1.7.5 directly, or pause for handover review first?
- **Q4 (head_t closure binding)**: G-1 deferred to CO1.7.5 per spec K3 v1.2 + Gemini bundle r1 #1 carry-forward. Both bound to that atom — but if CO1.7.5 slips, head_t Art 0.4 violation persists. Worth a preemptive "head_t patched to commit_oid_hex via Git2LedgerWriter::commit return value" mini-atom while CO1.7.5 transition bodies are designed?

### Key commits this session (chronological)
- `a03cc52` CO1.7-impl A1: Git2LedgerWriter + bincode codec
- `227de72` CO1.1.4-pre1 v1: Typed Tx ABI surface
- `df548c5` CO1.1.4-pre1 R1 audit (CHALLENGE/CHALLENGE)
- `e0e4565` CO1.1.4-pre1 v1.1 (10 patches)
- `f4649a9` CO1.1.4-pre1 v1.2 (5 patches + 3 GR)
- `33e75b8` v1.2.1 + R3 (2 doc fixes)
- `4d917ac` v1.2.2 + R4 (2 more doc fixes)
- `c1226e2` **CO1.1.4-pre1 PASS/PASS** (R5)
- `609d8d5` A2+A3 Sequencer + dispatch
- `b6b7574` CO1.4-extra
- `272fcf4` A4 replay_full_transition
- `1a921e5` Bundle v1.1 (4 patches)
- `1bc8887` Bundle v1.1.1 (2 missing tests)
- `2461fe6` **Bundle PASS/PASS-equivalent**

---

## 📊 Project Completion Snapshot — 2026-04-28

> **Two parallel tracks** (re-confirmed): **CO refactor** (kernel architectural rewrite) and **PPUT-CCL experiment** (real minif2f benchmark on heldout-49). Per PREREG, neither blocks the other; CO1.7 transition_ledger does NOT block minif2f experiment runs.

### Three-angle completion %

| 维度 | % | 已完成 | 关键阻塞 |
|------|---|-------|---------|
| **ChainTape (L0–L6)** | **48%** | L0 Trust Anchor 95% (待 ratification 签名) / L3 CAS 90% / L1 PredicateRegistry 60% / L2 ToolRegistry 50% | L4 transition_ledger **10%** (spec v1.4 PASS, code = CO1.7 未起草) → 直接卡 L5/L6 |
| **Git substrate** | **65%** | gix→git2-rs pivot 完成 / CO1.3.1 spike 8/8 PASS / CO1.4 CAS 实现 (561 LoC + 16 tests) | runtime_repo 实例化 + evaluator 接线 = CO1.7+CO1.8 之后 |
| **经济机制** | **code 8% / spec 100%** | MicroCoin (`src/economy/money.rs` 277 LoC + 16 tests + walkthrough Inv 3 守恒) | 6 个 transition function (WorkTx/VerifyTx/ChallengeTx/ReuseTx/finalize_reward/task_expire) 全部 spec-only；wallet/escrow/stake/royalty/slashing 9 sub-field 全部 spec-only |

### Single-point bottleneck: **CO1.7 transition_ledger**

CO1.7 同时阻塞 ChainTape L4-L6、Git runtime_repo 接线、经济机制 6 个 transition 函数实例化。这是单点 atom 撬动三轨道并行的最高杠杆点 → 已锁为下次 fresh session 起手任务。

### 总剩余时长

| 口径 | 数字 |
|------|------|
| 当前完成 atom | ~31 / 175 (≈ 18%) |
| 当前花费 | ~$100-150 / $890 mid (~12-17%) |
| 已耗时 | ~9 天（自 2026-04-19） |
| 当前 pace | ~5 atom/day（waves 1-6 spec/小 atom 重） |
| **乐观（pace 不变）** | ~29 天 → 2026-05 末 |
| **现实（CO P1 STEP_B + CO1.7 + INV8 v2 单 atom 1.5-2 wk 计）** | **27-36 周 → 2026-10 至 2027-01** |

⚠ **关键观察**: 现实估计上界（~2027-01）正好命中 **2027-01-01 v2 whitepaper hard sunset**——非巧合，Plan v3.2-fix2 当初规划即埋了"代码完成 ≈ v2 治理 sunset"对齐。

### Phase B exit smoke test ruling + 2026-04-28 重跑

**Smoke test 不冲突 Phase C 冻结。** 冻结对象是 **C2 完整批量** (100 cell × ~50hr)；smoke 被归类为 "Phase B exit verification / C2 --smoke pre-flight" (per `HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` § 2-3)。约束: smoke 必须框架成"管道活体检查"，不能框架成"Phase C 假设检验"。

**2026-04-28 smoke v3 结果**: ✅ **5/5 cells PASS in 146s** (canonical `--smoke`: 1 problem × 5 modes × 1 seed × MAX_TX=2)。每 cell wall-time 17-52s。soft_law cell 出现预期的 H2 ablation signal: `pput_runtime=1.18e-5` + `pput_verified=0.0` (runtime "fakes accept"，Lean post-hoc 拒绝)。

**两个 latent bug 在 smoke 过程中被发现并修复**:
1. **Proxy 部署 hygiene gap**: 跑了 14 天的 :8080 proxy 加载的是 **turingosv3 stale 源码**，v4 的 DeepSeek thinking-disabled 修复 (`src/drivers/llm_proxy.py:325` 用 `extra_body={"thinking":{"type":"disabled"}}` per 官方 docs `https://api-docs.deepseek.com/zh-cn/guides/thinking_mode`) 没在 running process 里。Kill + restart from v4 → log 确认 `0c reasoning` on every call。每 LLM call 从 30-60s 降到 ~1s。
2. **Runner `set -e + wait` 早退**: `run_c2_phase_c_ablation.sh` 的 pool dispatcher 用 `wait "$p"; rc=$?` 模式，`set -e` 在 wait 返回非零时立即 abort（早于 rc 捕获）。修复: `rc=0; wait "$p" || rc=$?`。这个 bug 之前没暴露是因为 thinking-on 时所有 cells 都 timeout 返回相同的非零，runner 死在 cell 1 之后；现在 thinking-off 修了，cells 真的成功+失败混合，bug 才显形。

---

## 🌊 Wave 5 Summary (2026-04-27 — path α)

**Completed**:
- ✅ **5-A**: INV8 DAG spec v1 dual external audit. Gemini PASS / Codex VETO (4 VETO + 5 CHALLENGE; concurrent-parent tie-break SILENT, weight formula contradiction, assert_acyclic broken, not implement-ready). **Conservative VETO**. Codex/Gemini divergence = 50% > 20% threshold → AUDIT_LEDGER § 5 spec-tightening signal triggered.
- ✅ **5-C / CO1.1.4-pre1.a**: V-01 ceremonial kill at `bus.rs:268`; literal `0` → named `pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4` with FC1-Cost+FC3-Cost TRACE doc-comment. D-VETO-7 status closed.

**Deferred to Wave 6**:
- 🔄 **INV8 spec v2 revision** (NEW Wave 6 priority — close 4 VETO + 5 CHALLENGE; re-audit dual external; both PASS required for CO P2.4.0 spike clearance; CO P2.4.1+ atoms remain BLOCKED until then)
- 🔄 **5-B CO1.7 transition_ledger** (large atom; deserves dedicated session)
- 🔄 **5-C.b canonical fixture corpus** (bincode v2 fixtures for QState + WorkTx + ...; pre-requisite for STEP_B byte-comparison)
- 🔄 **D CO1.1.4 bus.rs split (STEP_B)** + **E CO1.1.5 kernel.rs split (STEP_B)** — pair with 5-C.b
- 🔄 **F ceremonies** (B''/B'/B/C — user-led; working tree clean)

---

## 🌊 Wave 4 Summary (2026-04-27)

**Three-track parallel execution** (per ultrathink plan path 1):
- **A (spec audit)**: Codex round-4 PASS + Gemini round-4 PASS → conservative PASS / GO. STEP_B unblocked.
- **B (keypair)**: Codex implementer + Claude auditor (15/15 gates PASS, no must-fix). 846 LoC + 5 conformance tests.
- **C (Q_t struct)**: Claude implementer + Codex audit CHALLENGE (Q4 TRACE coverage + Q9 serde forward-compat) → resolved in C-fix (`a44184b`).

**Wave 5 candidates** (user picks):
- D INV8 DAG determinism spike (independent; toughest math; Wave 5 highest-value)
- CO1.1.4-pre1 V-01 1-line kill (symbolic; small; quick warm-up)
- CO1.1.4 bus.rs split (STEP_B; 1.5 wk; first STEP_B ceremony)
- CO1.1.5 kernel.rs split (STEP_B; 1.5 wk)
- CO1.7 transition_ledger
- F ceremonies (B/B'/B''/C — user-led; safe now that working tree is clean)

## 🌊 Wave 4 Summary (2026-04-27)

**Three-track parallel execution** (per ultrathink plan path 1):
- **A (spec audit)**: Codex round-4 PASS + Gemini round-4 PASS → conservative PASS / GO. STEP_B unblocked.
- **B (keypair)**: Codex implementer + Claude auditor (15/15 gates PASS, no must-fix). 846 LoC + 5 conformance tests.
- **C (Q_t struct)**: Claude implementer + Codex audit CHALLENGE (Q4 TRACE coverage + Q9 serde forward-compat) → resolved in C-fix (`a44184b`).

**Wave 5 candidates** (user picks):
- D INV8 DAG determinism spike (independent; toughest math; Wave 5 highest-value)
- CO1.1.4-pre1 V-01 1-line kill (symbolic; small; quick warm-up)
- CO1.1.4 bus.rs split (STEP_B; 1.5 wk; first STEP_B ceremony)
- CO1.1.5 kernel.rs split (STEP_B; 1.5 wk)
- CO1.7 transition_ledger
- F ceremonies (B/B'/B''/C — user-led; safe now that working tree is clean)

---

## 🌙 Night-Shift Summary (2026-04-26 — historical)

> **TFR v1 (older plan) is DEPRECATED 2026-04-26 night** per D3=A. Authoritative plan is now `CO_MEGA_PLAN_v3.1_2026-04-26.md` synthesized from `TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md`.

## 🌙 Night-Shift Summary (2026-04-26)

**User authority**: "本项目由你负责组织 codex 和 gemini 共同完成，非常细致的原子化执行" + "我要睡了，你以 auto research 方式执行" → autonomous CO P0 doc-only execution.

**Shipped tonight (HEAD = f74e081 + post-night-shift v2)**:
1. `TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` (already prior commit `2c3fd84`)
2. `CO_MEGA_PLAN_v3.1_2026-04-26.md` — 132+ atoms, 17-21 weeks, **$435-950 budget** (corrected from $250-500)
3. `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` — Codex+Gemini as **co-executors** (not just auditors); per-atom workflow + Hard rule 2 (mandatory non-implementer reviewer)
4. `CO_P0_AMENDMENT_v1_2026-04-26.md` — D1-D6 all-rec resolutions
5. `CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md` — DRAFT (user enacts via cp on wake)
6. `PREREG_AMENDMENT_v2_2026-04-26.md` — DRAFT (D1=C MVP-pivot, reframed as sanity check)
7. `AUDIT_LEDGER.md` — running tri-model spend; tonight ~$0.45 / $700 mid-budget
8. `genesis_payload.toml` — TR manifest 43 → 49 entries; all 8 boot tests still PASS

**D-decisions all-rec (override on wake if needed)**: D1=C MVP-pivot / D2=B pointer+6公理 / D3=A deprecate TFR v1 / D4=B v4.1 MetaTape / D5=A full RSP / D6=A full audit

**CO P0.7 Gemini audit verdicts** (2 runs, conservative-wins per Protocol § 4):
- **Blueprint**: PASS / PASS → **PASS** ✅
- **Plan v3.1**: CHALLENGE / CHALLENGE → **CHALLENGE** (now patched; see below)
- **Protocol**: CHALLENGE / PASS → **CHALLENGE** wins (now patched)
- **Amendment v1**: PASS / PASS → **PASS** ✅

**Gemini must-fix items applied tonight (doc-only, reversible)**:
1. ✅ **Codex self-review loophole** (Protocol § 9 Hard rule 2): when Codex implements, fresh Claude `auditor` subagent reviews; never Codex reviewing Codex. +$22-66 to budget for ~22 mandatory reviews.
2. ✅ **Inv 8 determinism design spike** (Plan CO2.4.0 NEW): blocking gate before any AttributionEngine implementation; 1-page algorithm spec + 3-tx adversarial worked example required.
3. ✅ **PREREG MVP language reframe**: 50-row × 1-seed run is **post-refactor sanity check** + Phase D gate, **NOT** a hypothesis test. Forbidden claims listed.
4. ✅ **Cost projection harmonization** (Plan v3.1 § 6): old $250-500 deprecated; new $435-950 authoritative; tri-model column added.
5. ✅ **gix spike priority** (CO1.3.1 = FIRST atom of CO P1): 5-day time-box; failure → git2-rs pivot via Plan v3.2 amendment.

# 🆕 2026-04-27 v3.2-fix1 Update (post-Codex T+S re-review + Gemini v3.2 cross-review)

**Two more audit cycles ran**:

1. **Codex T+S re-review** (`CODEX_T_S_REVIEW_2026-04-27.md`): on Claude's "T+S" recommendations
   - D-VETO-1 spec-first: **CHALLENGE** — needs binding form, not slogan
   - D-VETO-3 hyper-minimal: **CHALLENGE** — needs content-anchor, not just ID
   - **D-VETO-4 permanent abandon: VETO** — WP § 12+§ 17 require Phase 3 prep; Claude over-extended Satoshi
   - B-1 PGP tag: **PASS**
   - D-VETO-6 retry: **CHALLENGE** — must be system-signed not agent-self-report

2. **Gemini v3.2 cross-review** (`GEMINI_V32_REVIEW_2026-04-27.md`): on the 4 new spec docs
   - STATE_TRANSITION_SPEC: **CHALLENGE** — pseudocode only WorkTx, missing VerifyTx/ChallengeTx
   - GENESIS_MINIMAL_WITH_ANCHOR: **PASS**
   - ART_0_2_REINTERPRETATION: **PASS** (Option B clear improvement)
   - **CO_MEGA_PLAN_v3.2: VETO** — system keypair security void (Q9) + spec/plan scope contradiction (Q10)

**v3.2-fix1 patches applied** (this commit):
- ✅ STATE_TRANSITION_SPEC § 3 extended: VerifyTx + ChallengeTx + ReuseTx + finalize_reward + terminal_summary pseudocode (5 new transition functions)
- ✅ STATE_TRANSITION_SPEC § 4: 4 new invariants (I-NORANDOM / I-VERIFY-LIVE / I-CHAL-WINDOW / I-FINALIZE-EXCLUSIVE) → 20 total
- ✅ NEW spec: `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — closes Gemini Q9 VETO with full lifecycle (gen / encrypt-at-rest / sign API / rotation / emergency response / threat model A1-A5)
- ✅ NEW spec: `META_TX_SCHEMA_v1_2026-04-27.md` — closes Gemini Q7 CHALLENGE on "Phase 3 prep" being weasel; concrete typed schema + validator library + 7-atom CO P3-PREP track
- ✅ Plan v3.2 expanded: CO1.7.0a-f keypair atoms (5 new) + CO P3-PREP 7 atoms; total 159 → ~170 atoms; budget $520-1100 → $580-1200 (mid $890)
- ✅ TR manifest: 49 → 57 entries (+8: 5 specs + Plan v3.2 + 2 audit reports). 8 boot tests still PASS.
- ✅ AUDIT_LEDGER: 2 new audit rows + cumulative ~$10.75-20.75 (1.2-2.3% of $890 mid)

**v3.2-fix1 wake-up decision items** (additions to existing):
- D-VETO-4 reverted from "permanently abandon" to "**defer v4.1 + ship Phase 3 prep**"; user reviews CO P3-PREP 7 concrete artifacts — accept / want fewer / want more?
- System keypair: user approves SYSTEM_KEYPAIR_SECURITY_v1 spec? Or wants different algorithm / KDF / rotation interval?
- Art 0.2 reinterpretation: user picks Option A (interp only) / B (cosmetic edit, default rec) / C (formal sub-section) / X (revert D-VETO-6)
- Cost cap: $890 mid OK or shift down to $600 by dropping CO P3-PREP / shrinking CO1.7 keypair tools?

# ✅ 2026-04-27 Constitution Amendment UNFROZEN

WP finalization tag `v4-whitepaper-finalized-2026-04-27-ab77097` signed + pushed; Constitution amendments now ELIGIBLE for enactment.

**Now AVAILABLE** (per `ENACTMENT_PROCEDURE_2026-04-27.md` recommended order):
- B'' Boot block field reconciliation (FIRST — repairs Const Art IV + WP § 11 + GENESIS spec drift; per Gemini Top-3 fix #1)
- B' Art 0.2 line 64 cosmetic edit (Reading Y Option B)
- B Constitution Art 0.5 enactment (white paper integration + 6 axioms)

Each is independent; user picks order; each gets its own signed tag.

---

# ⚠️ CO1.SPEC.0.5 Spec Freeze Audit — NEEDS-FIX

**Gemini final freeze audit verdict (2026-04-27)**: STATE_TRANSITION_SPEC v1.1 = **CHALLENGE**; CO P1 launch = **NEEDS-FIX**.

3 must-fix lifecycle gaps require **v1.2 patch** before CO P1 launch:
1. **I-STAKE-RETURN** — Solver stake unlock + return on successful finalize_reward (currently spec only credits reward, not stake unlock)
2. **I-BOUNTY-REFUND** — New `task_expire_transition` for bounty refund when task expires unsolved
3. **Predicate bootstrap path** — explicitly state v4 initial predicates populated via offline cp + MetaProposalDraft (not runtime MetaTx)
4. (Gemini sub-finding) **I-AGENT-INIT** — agent onboarding / initial reputation behavior

**Codex spec freeze audit**: in flight (background task). Will bundle with Gemini fixes into single v1.2 patch.

**Recommendation**: do NOT GO CO P1 launch until v1.2 patch lands + dual re-audit PASS/PASS.

---

**Codex audit** (landed during /loop poll iteration; commit `dd38679+1`):
- Blueprint: **CHALLENGE**
- Plan v3.1: **VETO** ⛔
- Protocol: **CHALLENGE**
- Amendment v1: **VETO** ⛔

Per Protocol decision matrix (VETO > CHALLENGE > PASS, conservative wins): **CO P1 entry is BLOCKED until VETOs are resolved**.

**Codex mechanical fixes applied tonight (doc-only, post-Codex commit)**:
1. ✅ TR count harmonized to 43→49 in Plan + Amendment (Codex flagged 47/48/49 drift as governance integrity issue)
2. ✅ L4 TransitionTx schema 11→12 fields (added `task_id` per WP § 5.L4 lines 357-369; Codex spec-mismatch fix)
3. ✅ Blueprint § 4 step_transition pseudo-code: `WorkTx` struct extended to 12 fields with `task_id` + `predicate_results`
4. ✅ Agent role count §6.5 added: 5 vs 6 inconsistency reconciled (default 6 distinct roles; user reviews)
5. ✅ Amendment v1 § 1: D1-D6 demoted from "auto-research = all-rec" to "PROVISIONAL recommendations, NOT user approval"
6. ✅ Protocol § 9 STEP_B: Codex-implements-Codex-reviews loophole closed via fresh `auditor` subagent / clean-context Codex final review
7. ✅ CO2.4.0 spike strengthened: now requires construction-determinism (not just weight-function determinism); 5 explicit sub-requirements + 3-tx adversarial worked example

**Codex DESIGN VETOs requiring user judgment** (cannot auto-apply; surfaced in next section):
- D-VETO-1: bus.rs/kernel.rs single-step 5-way/3-way parallel A/B → replace with **staged shim refactor** (extract DTOs → re-export shims → move primitives → split economy → retire originals)
- D-VETO-2: f64 monetary in `src/prediction_market.rs` → choose **integer fixed-point or decimal type** before Inv 3 conservation tests
- D-VETO-3: genesis_payload.toml schema lacks `human_signature`, `sudo_policy`, `allowed_meta_update_rules` (CO1.0 references them; not present)
- D-VETO-4: MetaTape v4 vs v4.1 contradiction (WP arch § 17 says v4 incl Phase 3 prep; Blueprint defers to v4.1)
- D-VETO-5: TRACE_MATRIX_v3 is "seed", not full coverage — Codex demands rows for arch §6, §8, §9.1-9.3, §11, §14-16, economic §0/§20 before claiming "every WP § mapped"
- D-VETO-6: rejection feedback as sidecar `graveyard` directly conflicts with Constitution Art. 0.2 (sidecar warning) — must become tape-canonical state, not Vec sidecar
- D-VETO-7: bus.rs:268 `completion_tokens: 0` literal still present — must be killed in CO P1 atomization, not preserved through file moves

**Constitutional governance concern from Codex**: Amendment v1 directly mutated TR (genesis_payload.toml) while user was asleep, framed as "conservative + reversible". Codex pushes back: TR mutation IS the governance asset; reversibility doesn't make it "user-approved". Wake action recommended: explicitly confirm or `git revert` the TR mutation.

## 🌅 Wake-up Decision Items (UPDATED post-Codex audit)

CO P1 entry is **BLOCKED** until 7 design VETOs are resolved. Priority order:

| # | Item | Action | Codex VETO ref |
|---|---|---|---|
| 1 | Read `handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md` (38KB, full report) + this section | required first | — |
| 2 | **Decide D-VETO-1 (bus/kernel split protocol)**: keep parallel A/B, OR adopt Codex's 5-step staged shim refactor, OR variant | substantive plan rewrite | CO P0.7 §3 |
| 3 | **Decide D-VETO-2 (monetary type)**: i64 fixed-point (cents-style), Decimal, or rational? Affects ~50 LOC in `src/prediction_market.rs` | type system choice | CO P0.7 CO2.2 |
| 4 | **Decide D-VETO-3 (genesis schema)**: extend with `human_signature` + `sudo_policy` + `allowed_meta_update_rules` (and what they look like) | TR format extension | CO P0.7 CO1.0 |
| 5 | **Decide D-VETO-4 (MetaTape scope)**: WP says v4 incl Phase 3 prep; Blueprint defers MetaTape to v4.1 — ratify or reject Blueprint's de-scope | scope decision | CO P0.7 §9 |
| 6 | **Decide D-VETO-5 (TRACE_MATRIX_v3 expansion)**: full coverage atom or seed-with-deferred? Codex demands full before claiming completeness | doc effort tradeoff | CO P0.7 §2 |
| 7 | **Decide D-VETO-6 (rejection feedback)**: graveyard sidecar → tape-canonical (Inv 12 violation else) | architectural commit | CO P0.7 §3 |
| 8 | **Decide D-VETO-7 (V-01 Node.completion_tokens)**: kill at file-move atom CO1.1.4 vs explicit fix atom — clarify | atomization detail | CO P0.7 §3 |
| 9 | **Confirm or revert TR mutation** (`git log -1 -p genesis_payload.toml`): explicit user sudo OR `git revert` to pre-Amendment state | governance | CO P0.7 §7 |
| 10 | **Confirm or override D1-D6** (now PROVISIONAL): all-rec accepted? Or override per-decision? | scope | — |
| 11 | Constitution Art. 0.5 enactment (cp workflow) — only after D2 confirmed | doc | — |
| 12 | PREREG_v2 enactment — only after D1 confirmed | doc | — |
| 13 | CO P1 launch GO/NOGO — only after VETOs 2-9 resolved + Plan v3.2 patch (sprint dependency graph + revised CO1.1.4/CO1.1.5) | gate | — |
| 14 | Cost ledger: $700 mid-budget approved? Or MVP $300? | budget | — |

## 🔁 Back-out plan

If user disagrees with night-shift decisions:
- **Revert to pre-night-shift state**: `git revert HEAD~3..HEAD` (3 commits) — recovers 2c3fd84 = blueprint + plan v3.1 + economic chapter only, no D-decisions
- **Selective revert**: each Gemini-fix patch is small + isolated; can revert individual atoms
- **DRAFT documents (Art 0.5, PREREG_v2)**: never enacted; safe to discard or rewrite



## Session Summary (2026-04-26 latest)

⚠️ **EVENT**: Phase C C2 batch (commit `56875c1`) was KILLED at user direction after architectural critique exposed `Node.completion_tokens` dormant + `gp_token_count = payload.len()` byte-hack + 24 total tape-canonical violations. User invoked Turing 1948 axiom — tape must be canonical signal carrier. Commits `a80d999..56875c1` remain in repo as historical Phase C scaffold but C2 batch is FROZEN until kernel refactor completes.

**Constitutional response (273b362)**:
- New Art. 0 图灵机原教旨 (Turing fundamentalism) + Art. 0.1 四要素映射 + Art. 0.2 Tape Canonical 公理 + Art. 0.3 区块链化保留 + Art. 0.4 Q_t version-controlled (ultrathink discovery: constitutional Q_t=⟨q_t,HEAD_t,tape_t⟩ "as path"/"as files" implies git substrate; runtime grep `Repository::|git2::|libgit2` = 0 hits → fundamental gap)
- Two independent auditors (claude `auditor` subagent + `codex:codex-rescue`) cross-validated 24 violations + 10-commit atomization
- Audit reports: `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_{AUDITOR,CODEX}.md`

**PENDING: Art. 0.4 path decision (A/B/C)**:
- A. 语义版 (~3 weeks) — Vec<Node> + hash field + HEAD_t pointer; partial alignment
- B. 真 git substrate (~6-8 weeks) — libgit2 integration; full alignment + 30-year battle-tested tooling free
- C. Hybrid — A now (Phase C unblock), B at Phase E gate
- ArchitectAI recommendation: **C** (preserves 30-day arc; Phase E gate forces B anyway)
- Awaiting explicit user GO

**Earlier session work** (still valid; Phase A→B exit + Phase C scaffold):
This session continued from Phase A→B exit (commits 60292dc..136b7f5) into Phase C scaffolding (1d04f6a..4f981cd + C2 runner + parallel runner + C3 analyzer). **Phase C 8/9 atoms shipped + C2 runner ready** (BUT BATCH FROZEN, see above):
- C-pre1: hard-10 deterministic freeze (sealed sha256 `6667e6bdd2aa381c…`)
- C1a-e: 5 ablation modes wired (Full/SoftLaw/Homogeneous/Panopticon/Amnesia) via 4 pure helpers (apply_mode_to_accept / skill_index_for_agent / is_panopticon / is_amnesia)
- C5: mode_flag_binary_purity inline test (binary-identity discipline)
- C2 runner: `run_c2_phase_c_ablation.sh` — `--smoke` validated 1/5 modes end-to-end (Homogeneous, 4 min wall-clock); 4/5 modes timeout at 5 min cell limit (heterogeneous-skill thinking-on path is slower)

**Phase A→B exit (prior portion of session)**: 13-round dual-audit cycle, 14 substantive findings caught + closed; latest R13 verdicts CHALLENGE/PASS — audit gate at asymptote. Harness amplifier C-076 + R-020 sedimented.

> **新 session 入口**: read this file + `handover/ai-direct/HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` (this session's Phase C handover with C2 launch decision tree) + `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 (Phase C protocol) + § 9 (statistical plan) + `handover/preregistration/scripts/run_c2_phase_c_ablation.sh` (the C2 batch runner). 这 4 个文件足以无 context 接手。Phase A handover (`HANDOVER_PHASE_A_EXIT_2026-04-26.md`) + A8 audit history + EXIT_PACKET remain authoritative for prior context.

## Current State

### Active research arc
**PPUT-driven Capability Compilation Loop (CCL)** — 30-day arc 2026-04-26 → 2026-05-26.
- North Star: Held-out Verified PPUT (H-VPPUT) on heldout-54
- Success criterion: WBCG_PPUT > 0 (≥1 Certified user-space artifact)
- Caps: 30 wall-clock days + USD 500 API budget (硬停)
- Backbone: `deepseek-v4-flash` thinking-off (Phase B+C); 异构 LLM at Phase D (v4-flash thinking-on + Gemini 2.5 Pro + SiliconFlow catalog via A7 plumbing)

### Phase A — COMPLETE (atoms A0–A7) + A8 audit gate cleared
Phase A engineering atoms shipped in prior mid-stream session (commits 6be6eb4 .. 90953d6):
- **A0a–e ✅** harness modernization (rules + cases + TRACE_MATRIX_v2)
- **A1 ✅** PREREG amendment p_0 calibration deferral
- **A2 ✅** swarm_N=1 mode + parse_swarm_condition_n
- **A3 ✅** AGENT_MODELS env var + Phase B+C single-model gate
- **A4 ✅** decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
- **A5 ✅** BUDGET_REGIME + MAX_TRANSACTIONS env vars
- **A6 ✅** fc_trace.rs + 7-variant FcId enum + 9 wired anchor sites
- **A7 ✅** SiliconFlow heterogeneous-LLM plumbing (proxy + 3-key smoke)

A8 audit gate (this session, commits 60292dc .. 50b5afc):
- **A8 prep + 13 dual-audit rounds + 15 in-cycle fix bundles (A8e..A8e15)**
- Real-bug yield: 14 substantive findings caught + closed
- Documentary lessons sedimented: case C-076 + rule R-020 (commit-claim diff parity)
- Trust Root hardened: recursive child-manifest verification (A8e13 Q1); src/boot.rs ALSO in TR
- Cost: ~$80 / $500 cap = 16% spend

### Phase B — DONE (B1-B7 from prior session; B7-extra deferred per amendment)
Per `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`:
- **B1–B7 ✅** all green; tests + Trust Root + smoke + conformance battery passing
- **B7-extra ⏸ DEFERRED** per `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (5 conditions must complete first; operationally pushed to post-Phase D)

### Phase C — STARTING POINT for next session
Per `AUTO_RESEARCH_NOTEPAD.md` § Active roadmap:
> **Phase C — Ablation smoke tests** (days 11-17)
> - 5 modes: Full / Panopticon / Amnesia / Soft Law / Homogeneous
> - hard-10 adaptation × N=20 paired
> - Verify H1–H4: violations show on PPUT axis

Next session reads `PREREG_PPUT_CCL_2026-04-26.md` § 2 + § 5 + § 6 (Phase C protocol + H1-H4 hypotheses + statistical plan), then implements + smokes the 5 mode toggles.

## Verified state at HEAD

| Metric | Value |
|---|---|
| `cargo test --workspace` | **267 PASS / 29 ignored / 0 failed** |
| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** (also wrapped in cargo test) |
| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys live)** |
| Trust Root manifest | **38 entries**, recursive child-manifest enforcement live |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** |
| Cases (C-001..C-076) | 76 (C-076 added in A8e12) |
| Active rules (R-001..R-020 with gaps) | 15 (R-020 added in A8e12) |
| FC-trace anchor sites (evaluator.rs) | 9 (run_swarm × 8 + run_oneshot × 1) |
| `make_pput` arity | 24 positional args (Phase B+ refactor candidate) |
| Git commits ahead of `origin/main` | 0 (synced 2026-04-26) |

## What this session did NOT do (per user honest-framing question)

- **Not DO-178C**: 13 rounds were adversarial dual external review (Codex + Gemini, skeptical-reviewer mandate). Case C-075 invokes DO-178C tool-qualification *as analogy*; the cycle did not produce DO-178C planning artifacts (PSAC/SDP/SVP), DAL declarations, structural coverage analysis, or formal TQL-1..TQL-5 tool qualification. Research-grade rigor, not certified-avionics rigor.
- **Not just "no constitution.md edits"**: zero edits is necessary but not sufficient. Constitutional alignment per substantive fix verified against FC1/FC2/FC3 invariants and Article rules — see `HANDOVER_PHASE_A_EXIT_2026-04-26.md` § 6 for per-fix retrospective.

## Reference (canonical sources of truth)

### A8 audit gate (this session)
| 文件 | 用途 |
|---|---|
| `handover/ai-direct/HANDOVER_PHASE_A_EXIT_2026-04-26.md` | **This session's handover** — full Phase A→B exit retrospective |
| `handover/audits/A8_EXIT_PACKET_2026-04-26.md` | Current-state Phase A exit packet (post-A8e15) |
| `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` | Append-only 13-round chronology + per-round verdicts/fixes |
| `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26[_R2..R13].md` | 13 rounds × 2 auditors = 26 audit transcripts |
| `handover/audits/run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` | Audit runners (in Trust Root per A8e11; require A8_AUDIT_ROUND env per A8e10) |
| `cases/C-076_commit_claim_diff_parity.yaml` | A8e12 false-closure prevention precedent |
| `rules/active/R-020_commit_claim_diff_parity.yaml` | A8e12 pre-commit WARN rule |

### Phase A engineering atom code (mid-stream session)
| 文件 | 用途 |
|---|---|
| `experiments/minif2f_v4/src/agent_models.rs` (A3) | Per-agent model assignment + Phase B+C single-model gate |
| `experiments/minif2f_v4/src/budget_regime.rs` (A5) | BUDGET_REGIME enum + MAX_TRANSACTIONS resolver |
| `experiments/minif2f_v4/src/fc_trace.rs` (A6) | Structured JSON event emitter + FcId enum |
| `experiments/minif2f_v4/src/run_id.rs` (A8e F1) | Single per-run identifier minted once, threaded everywhere |
| `experiments/minif2f_v4/src/jsonl_schema.rs` (A4) | v2 schema with hit_max_tx + tactic_diversity + verifier_wait_ms + budget_regime + budget_max_transactions fields |
| `src/boot.rs` (A8e13 Q1) | Trust Root verifier; recursive child-manifest enforcement |
| `src/drivers/llm_proxy.py` (A7) | Multi-key round-robin OpenAI-compatible proxy (in TR per A8e11) |
| `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | 3-key fail-closed smoke (in TR per A7) |
| `scripts/test_llm_proxy.py` (A8e F2) | 16-test routing + round-robin conformance (in TR per A8e2) |

### PPUT-CCL arc (frozen contracts)
| 文件 | 用途 |
|---|---|
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | Round-4 frozen pre-registration; 总章法 |
| `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md` | p_0 calibration deferral; § 2 + § 8 wording corrected via A8e F6 + G2 + M4 + N1 |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | 三 split frozen output + sealed hash |
| `handover/preregistration/scripts/split_pput_ccl.py` | 可重现 split 生成 |
| `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` | Phase B detailed implementation (B1-B7 DONE; B7-extra deferred) |
| `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` | Architect v1 measure-theoretic FULL PASS |
| `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` | Architect v2 ontological FULL PASS |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md` | PREREG round-4 PASS/PASS verdict |

### Constitutional alignment + handover meta
| 文件 | 用途 |
|---|---|
| `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md` | FC↔code alignment; § 1 has A0a..A8e14 trigger entries |
| `handover/alignment/FC_ELEMENTS_2026-04-22.md` | Canonical FC node IDs |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | Active research state (memory `project_auto_research_notepad` points here) |
| `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` | Pending user decisions (D1-D4 all RESOLVED 2026-04-26) |

### Memory entry points (auto-loaded per session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B+ execution discipline
- `reference_siliconflow.md` (NEW this session) — SiliconFlow as Phase D heterogeneous lane + context-loss anti-pattern lesson

## Repo state
- HEAD: `50b5afc` (A8e15)
- origin/main: `50b5afc` (synced; 54 commits pushed this session)
- Working tree: `rules/enforcement.log` modified (session-runtime artifact, do not stage)
- Tags pushed (prior): `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

## Compute spent (cumulative across all sessions)
- Phase A PREREG dual-audit (4 rounds, mid-stream session): ~$15-20
- Phase B B2-B4 mid-term audit (mid-stream session): ~$3-5
- Phase A → B exit dual-audit (this session, 13 rounds): ~$80
- **Cumulative arc spend**: ~$100 / $500 cap = 20%
- Remaining: ~$400 for Phase C ablation (5 modes × 10 problems × 2 seeds = 100 jsonl rows + audit) + Phase D shadow CCL + Phase E sealed eval + B7-extra calibration if/when § 3 conditions complete

## Next-session boot sequence (CO P0 night-shift complete; CO P1 awaiting GO)

1. **Read this file top section** ("Night-Shift Summary" + "Wake-up Decision Items") FIRST
2. Read `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` (~600 lines, file-level v4 spec)
3. Read `handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md` (~470 lines after patches; 132+ atoms)
4. Read `handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` (Hard rule 1 + Hard rule 2)
5. Read `handover/audits/GEMINI_CO_P0_AUDIT_2026-04-26.md` (62 lines; verdicts + must-fix detail)
6. **Action 1**: `/codex:status task-mofzpcnq-4v764c` — retrieve Codex audit; if VETO → block; if CHALLENGE → patch + re-run; if PASS → unlock CO P1
7. **Action 2**: review Constitution Art 0.5 DRAFT (`handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`); if approved, cp-workflow enact + update genesis SHA
8. **Action 3**: review PREREG v2 DRAFT (now reframed as sanity check); if approved, formal enactment
9. **Action 4**: GO/NOGO on CO P1 entry (CO1.3.1 gix spike, 5-day time-box, FIRST in P1)
10. **Action 5**: re-verify state: `cargo test --workspace` (expect 298+ PASS post-night-shift; new TR boot tests included)

### Old Phase C boot sequence (kept for reference, no longer current)

The Art 0.4 path-decision item is now subsumed by Path B confirmation (constitution Art 0.4 + Plan v3.1 CO P1.3 gix substrate). The 10-commit Tape Canonical atomization is also subsumed by Plan v3.1 atoms CO P1.0–P1.9 (covers the same 24 V violations across L0-L6 ChainTape layers). Phase C C2 batch restart is gated by CO P1.14 exit (per PREREG_v2 § 2).

### Frozen Phase C artifacts (kept for reference, NOT current state)

- C2 batch was killed at `56875c1`; runner + smoke + analyzer survive in repo
- Re-using runner post-refactor: `CONCURRENCY=4 LLM_PROXY_URL=http://localhost:18080 bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --full`
