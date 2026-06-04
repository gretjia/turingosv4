# TuringOS v4 — Handover State

> Agent cold start: read `AGENTS.md`, `HARNESS_PLAYBOOK.md`, and
> `skills/SUBAGENT_HARNESS.md` before this file. This file is a derived view,
> not a source of truth. ChainTape/CAS and executable gates win on conflict.
>
> Hard rules: PR-only workflow, no `git push origin main`, no wildcard staging,
> no sidecar staging. See `AGENTS.md` §14a.

---

## Current Snapshot (2026-06-04)

**Session**: OBL-005 reopened re-audit on current main; fresh market A/B
current-source evidence is being prepared after PR #257 merged the OSWorld
source receipt.

**Main tip**: `1643005f` (PR #257 — fresh OSWorld source evidence).
Current working branch is `codex/obl005-market-ab-g0-activation`, not yet
merged.

**Truth boundary**: this file is a derived handover view. If it conflicts with
`constitution.md`, ChainTape/CAS, deterministic replay, or executable gates,
trust those sources first.

Current state:

- `OBLIGATIONS.md` is **not globally complete**. OBL-001, OBL-004, OBL-006,
  OBL-007, OBL-008, and OBL-009 are satisfied in the current ledger, while
  OBL-005 remains `in_progress (reopened 2026-06-04)`.
- PR #245 through #256 hardened OBL-005 final-closure accounting: closure
  blocker inventory, replay-artifact GREEN checks, missing domain-closure
  blockers, source-tree fingerprint blockers, source-tree receipt identity, and
  source-receipt final-closure eligibility, plus fresh boot/replay, FC3,
  market/generate, Cybench, and OSWorld source evidence.
- PR #250 did **not** rewrite historical true-suite evidence. It makes future
  source-tree-bound current reruns produce closure-eligible source receipts
  when replay and source identity are green.
- Current reconciliation fixture binds 9 rows to older source receipts and 12
  rows to fresh current-source receipts from
  `obl005_fresh_boot_replay_20260604T143328Z` and
  `obl005_fresh_fc3_20260604T150936Z`, plus
  `obl005_fresh_market_20260604T153500Z` market evidence and
  `obl005_fresh_generate_20260604T171500Z` generate/artifact evidence, plus
  `obl005_fresh_cybench_20260604T164533Z` Cybench evidence,
  `obl005_fresh_osworld_20260604T171857Z` OSWorld evidence, and
  `obl005_fresh_market_ab_20260604T175932Z` market A/B evidence:
  `source_receipt_final_closure_false=9`,
  `source_tree_fingerprint_missing=9`,
  `fresh_final_closure_witness_missing=21`,
  `domain_receipt_final_closure_false=13`,
  `benchmark_capability_not_solved=10`,
  `domain_receipt_final_closure_missing=5`, and
  `market_no_or_short_side_missing=2`.
- Fresh deterministic evidence added this session: `boot_cli_current_kernel_fresh`
  and `replay_cas_tamper_repair_current` now point at
  `handover/evidence/true_suite/obl005_fresh_boot_replay_20260604T143328Z/`.
  Both receipts are `final_closure_possible=true` with source commit
  `024dfd2d75817f7c0e52004fd3fca8122e9981d9`; replay/CAS still keeps its
  domain-closure-missing blocker and all rows still require a fresh final
  closure witness.
- Fresh FC3 evidence added this session: `fc3_governance_reinit_fresh` and
  `memory_feedback_reinit` now point at
  `handover/evidence/true_suite/obl005_fresh_fc3_20260604T150936Z/`.
  The receipt is `final_closure_possible=true` with source commit
  `5a2c74c46c9060256410c07a4e79ee2b0331212b`; both rows still require a
  fresh final closure witness. Clean-context Claude witness
  `handover/audits/OBL005_FRESH_FC3_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION` for the Class 2 reconciliation/evidence update.
- Fresh market evidence added this session: `market_external_agent_fresh` and
  `market_economy_polymarket` now point at
  `handover/evidence/true_suite/obl005_fresh_market_20260604T153500Z/`.
  The receipt is `final_closure_possible=true` with source commit
  `33d88be8b7835f6bde7eb2fc2e5f0301996efcab`, signed router/WorkTx evidence,
  packaged restore replay artifacts, and replay indicators green. The live
  external model chose `direction=yes`, so `market_no_or_short_side_missing`
  deliberately remains; both market rows also keep domain/fresh-witness
  blockers. Clean-context Claude witness
  `handover/audits/OBL005_FRESH_MARKET_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION`.
- Fresh market A/B evidence prepared and audited on the current branch:
  `market_ab_performance_fresh` now points at
  `handover/evidence/true_suite/obl005_fresh_market_ab_20260604T175932Z/`.
  The source receipt is `final_closure_possible=true` with source commit
  `4d11124f4c2e562d17d8790742988384c17d6151`, `FULL_SYSTEM_LIT`,
  `missing=[]`, green replay indicators, audit_tape `PROCEED`, and 12 packaged
  evidence stores. The deterministic G0 receipt is scoped to
  `g0_core_market_price_discovery_conditions_1_2_3_6_7_8_9`: one WorkTx node,
  one ChallengeTx short side, `buy_yes_count=1`, `buy_no_count=1`, price change,
  and green ChainTape replay. It explicitly records c4/c5 priced-DAG branching
  and c10/c11 reward-claim settlement closure as constrained/stage-2; no c1-11
  closure is claimed. `market_ab_performance_fresh` still keeps
  `domain_receipt_final_closure_missing` and `fresh_final_closure_witness_missing`.
  Verification passed: focused market/reconciliation/flowchart package, direct
  Trust-Root intact boot unit, touched-file `rustfmt --check`, shell syntax plus
  `git diff --check`, constitution matrix drift, constitution gates
  (`[k-1-5] total=165 failed=0`), and `cargo test --workspace --no-fail-fast`.
  Clean-context Claude witness
  `handover/audits/OBL005_FRESH_MARKET_AB_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION`.
- Fresh generate/artifact evidence added on main by PR #255:
  `generate_artifact_chain_fresh` now points at
  `handover/evidence/true_suite/obl005_fresh_generate_20260604T171500Z/`.
  The production generate prompt now forbids remote fonts/CDNs/external runtime
  URLs; the fresh run records `final_closure_possible=true`, source commit
  `7dd5b3d80b0313520b01c9c0fc56bd7117ff8b63`, no remote dependency matches in
  the generated artifact, admitted WorkTx + MarketSeed, packaged evidence
  stores, and restore replay indicators green. The row still keeps
  `domain_receipt_final_closure_missing` and
  `fresh_final_closure_witness_missing`; no final closure is claimed.
  Clean-context Claude witness
  `handover/audits/OBL005_FRESH_GENERATE_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION` for the Class 2 reconciliation/evidence update.
- Fresh Cybench source evidence prepared and audited on the current branch:
  `cybench_security_sandbox_fresh` and `cybench_security_sandbox` now point at
  `handover/evidence/true_suite/obl005_fresh_cybench_20260604T164533Z/`.
  The receipt is `final_closure_possible=true` with source commit
  `0f38026e4d03177ad4b6641086e9f1e98f751e8b`, `FULL_SYSTEM_LIT`,
  `missing=[]`, admitted WorkTx, packaged evidence stores, and green
  replay/restore indicators. The model result remains
  `safe_action_mismatch`, so both Cybench rows deliberately keep
  `domain_receipt_final_closure_false`, `benchmark_capability_not_solved`, and
  `fresh_final_closure_witness_missing`; no final closure is claimed.
  Clean-context Claude witness
  `handover/audits/OBL005_FRESH_CYBENCH_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION` for the Class 2 reconciliation/evidence update.
- Fresh OSWorld source evidence prepared and audited on the current branch:
  `osworld_computer_use_fresh` and `osworld_computer_use` now point at
  `handover/evidence/true_suite/obl005_fresh_osworld_20260604T171857Z/`.
  The receipt is `final_closure_possible=true` with source commit
  `1254212e10afe939b466d8404889106383d9bdb8`, `FULL_SYSTEM_LIT`,
  `missing=[]`, admitted WorkTx, packaged evidence stores, and green
  replay/restore indicators. The model result remains
  `sandbox_action_mismatch`, so both OSWorld rows deliberately keep
  `domain_receipt_final_closure_false`, `benchmark_capability_not_solved`, and
  `fresh_final_closure_witness_missing`; no final closure is claimed.
  Verification passed: real runner, focused OSWorld/reconciliation/
  final-closure/matrix tests, local secret/raw-response scan, `git diff
  --check`, constitution gates (`[k-1-5] total=164 failed=0`), and
  `cargo test --workspace --no-fail-fast`. Clean-context Claude witness
  `handover/audits/OBL005_FRESH_OSWORLD_SOURCE_EVIDENCE_CLEAN_CONTEXT_AUDIT_2026-06-04.md`
  returned `NO-VIOLATION`.
- WorkTx/escrow boundary: `constitution.md` does not explicitly state a
  WorkTx-accept uniqueness rule for one rewardable WorkTx per task escrow.
  Current kernel admission allows multiple WorkTxs for the same task; TB-8
  single-solver settlement/claim sweeping is what prevents multiple same-task
  full escrow payouts. Current market liveness stays single-WorkTx-node with
  multi-agent YES/NO router-side activity.
- Current worktree note: successful market A/B evidence
  `obl005_fresh_market_ab_20260604T175932Z` is intended for this PR. Do not
  treat local failed/intermediate evidence directories as GREEN evidence:
  `obl005_fresh_generate_20260604T160500Z`,
  `obl005_fresh_market_ab_20260604T174500Z`, and
  `obl005_fresh_market_ab_20260604T175726Z`.

Recent verification:

```text
cargo test -p turingosv4 \
  --test constitution_g0_market_activation_boundary \
  --test constitution_real16_market_performance \
  --test constitution_true_suite_broad_agi_batch_runner \
  --test constitution_realworld_liveness_coverage \
  --test constitution_script_liveness_inventory \
  --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_matrix_drift \
  --test fc_alignment_conformance -- --nocapture
# exit 0

cargo test -p turingosv4 --lib \
  boot::tests::verify_trust_root_passes_on_intact_repo -- --nocapture
# exit 0

rustfmt --edition 2021 --check \
  src/bin/g0_market_activation_current_kernel.rs \
  tests/constitution_g0_market_activation_boundary.rs \
  tests/constitution_real16_market_performance.rs
# exit 0

bash -n scripts/run_true_suite_market_ab_current_kernel.sh \
  scripts/run_true_suite_broad_agi_batch.sh && git diff --check
# exit 0

cargo test -p turingosv4 --test constitution_matrix_drift -- --nocapture
# exit 0

bash scripts/run_constitution_gates.sh
# [k-1-5] total=165 failed=0

cargo test --workspace --no-fail-fast
# exit 0

cargo test --test constitution_true_suite_osworld_runner \
  --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_matrix_drift -- --nocapture
# exit 0

cargo test --test constitution_true_suite_generate_artifact_runner \
  --test constitution_true_suite_evidence_reconciliation \
  --test constitution_obl005_final_closure_witness \
  --test constitution_matrix_drift -- --nocapture
# exit 0

cargo test --bin turingos blackbox_system_prompt -- --nocapture
# exit 0

cargo test --bin turingos blackbox_system_prompt_forbids_remote_font_dependencies -- --nocapture
# exit 0

git diff --check
# exit 0

PR #250 checks
# r022_check SUCCESS
# validate PR has no sidecar contamination SUCCESS
# Constitution gate suite SUCCESS
# Feature freeze check SUCCESS
```

Next steps:

- Continue generating fresh current-source true-suite evidence for the 14
  remaining source-blocked rows, without mutating historical evidence.
- Attack remaining domain/benchmark blockers honestly: a row may close only
  when its domain manifest, benchmark result, market NO/short side, replay/CAS
  evidence, and fresh witness all agree.
- Keep OBL-005 open until a current final-closure witness proves every retained
  module/script group is replay-lit, necessary support, or removed/superseded.

## Previous Snapshot

**Session**: 2026-05-23 close — TB-SOFTWARE-3-0 + TB-STRESS-PHASE-2 SHIPPED.

**Main tip**: `6c12e092` (PR #132 stress ship report + audits, 2026-05-23T13:00Z).

### TB-SOFTWARE-3-0-CONSOLIDATION (8 atoms, 8 PRs merged 2026-05-23)

Single-maintainer substrate hardening on top of Phase E cutover. Atoms +
PRs:

| Atom | PR | Class | What |
|------|----|----|----|
| S0.1 | #120 | 0 | Package §8 directive + TB charter |
| S1   | #122 | 2 | Remove stdout-as-truth in `task/open` (`t_hash_*` + `simple_hash` deleted; 502 BAD_GATEWAY on parse failure) |
| S2   | #123 | 2 | Private `GrillSessionSnapshot` in per-session CAS for cross-restart resume |
| S3   | #124 | 2 | `BuildSessionViewError { Open, Read, Decode }` taxonomy; empty stays `Ok(SpecPending)` |
| S4.1 | #125 | 2 | Rename `siliconflow_client` → `chat_client` (file + 7 cmd_*.rs imports); NO `ChatProvider` enum (deferred per K10) |
| S4.2 | #126 | 0 | `LLM_BOUNDARY_INVENTORY_2026-05-23.md` documenting 17 chat_complete* sites + deferred abstraction packet |
| S5   | #127 | 1+0 | `scripts/audit_legacy_bypass.sh` (reporting-only, NOT a constitution gate) + checklist doc |
| S6.1 + S6.2 | #128 | 0 | Aggregate ship report + cumulative audits (Constitution: NO-VIOLATION; Karpathy: PASS) |

Ship report: `handover/reports/SOFTWARE_3_0_CONSOLIDATION_2026-05-23.md`
Audits: `handover/audits/SOFTWARE_3_0_VAL_{CONSTITUTION,KARPATHY}_2026-05-23.md`
Charter: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`

Scope freeze (held across all 8 commits): NO touch to `src/state/typed_tx.rs`,
`src/state/sequencer.rs`, `src/bus.rs`, `src/bottom_white/cas/schema.rs`,
`constitution.md`, `genesis_payload.toml`, `src/runtime/mod.rs` export,
no new CAS `ObjectType`, no provider abstraction layer.

### TB-STRESS-PHASE-2 (3 PRs merged 2026-05-23)

Adversarial 10-test battery on top of Phase E + TB-SOFTWARE-3-0.

| Atom | PR | Class | What |
|------|----|----|----|
| STRESS-0 | #129 | 0+1 | Charter + §8 + 10 runner scripts under `scripts/stress/` |
| STRESS-1..10 | #131 | 1+2 | Execution evidence + runner robustness fixes |
| STRESS-SHIP | #132 | 0 | Aggregate ship report + cumulative audits |

**Final tally**: 8 PASS / 1 PARTIAL (ST-04) / 1 NOT-EXECUTED (ST-08) / 0 FAIL.

Substantive finding (ST-04 PARTIAL): S2's `write_snapshot` VERIFIED writing
418-byte capsules with schema_id `turingos-web-grill-session-snapshot-v1`
to per-session CAS. Multi-turn resume blocked by upstream triage promotion
guard requiring `PromptPromotionReceipt` — workspace bootstrap dependency,
NOT S2 defect. Production guard correctly fail-closes on unconfigured
workspaces.

Audits: `handover/audits/STRESS_PHASE_2_VAL_{CONSTITUTION,KARPATHY}_2026-05-23.md`
Ship report: `handover/reports/STRESS_PHASE_2_SHIP_REPORT_2026-05-23.md`
Charter: `handover/tracer_bullets/TB-STRESS-PHASE-2_charter_2026-05-23.md`

LLM cost: ~$0 (mock providers throughout). Wall time: ~3 hr.

### Memory updates from this session

- `feedback_defer_abstraction_until_second_impl` — don't propose
  ChatProvider/ModelCallReceipt-style framework before 2nd concrete impl
  lands. Rename to generic naming OK; abstraction layer deferred.
- `feedback_git_hygiene_no_bulk_ops` — forbidden: `git stash -u`, `git add -A`;
  default execution base = fresh worktree from origin/main.
- `feedback_conservative_error_semantics` — empty IS normal (`Ok(SpecPending)`,
  not `Err(EmptySession)`); HTTP failures use 502/500, not 200-with-warning.

---

## Pre-session-#60 snapshot (for forensic continuity)

**Session**: #60 close, 2026-05-22 — TDMA-Generate + Phase E libgit2 cutover SHIPPED.

PR #116 (Atom 25 full cutover) at 2026-05-22T18:36Z. `turingos generate
--tdma-bounded` and `turingos tdma run` both default to TDMA-Bounded +
GitTapeLedger (Phase E Path B). 8 atoms (19–26) merged to main.
Constitution Art. 0.4 Path B obligations (all 6) materially satisfied.
MemoryTapeLedger retired from production paths. Ship report:
`handover/tracer_bullets/TB-TDMA-GENERATE-PHASE-E_ship_report_2026-05-22.md`.
Package §8: `handover/directives/2026-05-22_TDMA_GENERATE_PHASE_E_DIRECTIVE_AND_§8.md`.

PRs in that ship: #109 (gen wire-up), #110 (skeleton), #111 (roundtrip),
#112 (head+BBS), #113 (migrate + single-chain fix), #115 (opt-in flag),
#116 (full cutover), #117 (Atom 26 ship report + §8 template + Path A retirement).

---

## Pre-session #60 snapshot (for forensic continuity)

**Session**: #59 close, 2026-05-22 — TDMA-Bounded-RC1 ship candidate.

**Main tip**: `865b4c14` — `fix(harness): update constitution gate test
after R-022 hook migration` (PR #89 squash). RC1 awaits architect GA §8
signature before merging `feature/tdma-bounded-rc1` to main.

**Active feature branch**: `feature/tdma-bounded-rc1` HEAD `f6e35aeb`
(PR #93). 10 atoms shipped (0..7+7.5+8); 9-gate harness GREEN; bug7
regression GREEN; real-evidence run captured at
`handover/evidence/tdma_rc1_real_evidence_20260522T095144Z/`
(invariants_passed=true). Ship report:
`handover/tracer_bullets/TB-TDMA-BOUNDED-RC1_ship_report_2026-05-22.md`.
GA §8 template awaiting architect signature:
`handover/directives/2026-05-22_TDMA_BOUNDED_RC1_GA_§8_TEMPLATE.md`.

---

## Pre-session #59 snapshot (for forensic continuity)

**Session**: #58 close, 2026-05-21 (late evening).

**State**: P7.z + Boundary-Ratification-Hygiene remain complete; session
#58 shipped three increments:
1. **Plan v7 — MiniF2F partial recovery** (PR #82/#83/#84 + hotfix as `cff03a28`):
   restored `lean_market` binary (`experiments/minif2f_v4/`, separate
   Cargo workspace) and promoted `batch_orchestrator.rs` to `src/runtime/`.
   Tier 3 deleted files remain unrestored per architect's strict
   no-innovation directive.
2. **R-022 hook architectural fix** (PR #88 `1cfad1a4` + PR #89 `865b4c14`):
   moved the TRACE_MATRIX backlink check from `pre-commit.r022` to a new
   `commit-msg.r022` hook (gives the in-flight commit message regardless
   of `-m`/`-F`/interactive), fixing the COMMIT_EDITMSG read footgun
   discovered during the Plan v7 hotfix. Constitution gate test parity
   updated alongside.
3. **Generative HTML kernel-integrity probe + Software 3.0 audit** (PR #91,
   branch `claude/generative-html-kernel-probe-20260522`): surfaced 5 real
   kernel bugs in `src/web/spec.rs` + `src/web/generate.rs` (2 LANDED via
   parallel sessions with tests; 2 in tree; 1 forward-bound). Software 3.0
   conformance: 3 PASS / 6 WARN / 2 FAIL (rubric C1-C11). FAIL = C8 no
   cross-session agent memory + C10 no generative HTML IR. See
   [handover/research/generative_html_kernel_integrity_2026-05-22/synthesis/REPORT.md](../research/generative_html_kernel_integrity_2026-05-22/synthesis/REPORT.md).

There is no active charter PR in flight at this handover.

**Archive**: sessions #1-#54 remain at
`handover/ai-direct/LATEST_ARCHIVE_PRE_2026-05-20_sessions_1_to_54.md`.
Session #56 audit/remediation records live under `handover/audits/`.

---

## What Changed In PR #78

PR #78 deliberately did **not** start the full v2.0 predicate layer. It shipped
the smaller transition framework: boundary facts, §8 ratification, process
hygiene, truthfulness hygiene, and meaning fixtures.

Load-bearing artifacts:

- `docs/architecture/FC_REAL_WORLD_BOUNDARY.md`
  - Class 0 fact record for FC1/FC2/FC3 real-world boundaries.
  - Names the four architect decisions: Art. 0.4 path, hermetic mechanism,
    predicate process locality, and LLM call topology.
- `handover/directives/2026-05-21_FC_BOUNDARY_RATIFICATION_DIRECTIVE.md`
  - Ratifies the boundary choices without auto-authorizing sequencer,
    typed-tx, trust-root, or signing-payload implementation.
- `handover/evidence/sandbox_boundary_baseline_2026-05-21.md`
  - Before-state evidence for naked shell-out, weak sandbox claims, and stale
    boundary facts.
- `src/sdk/sanitized_runner.rs`
  - `env_clear`, env allowlist, explicit cwd, stdout/stderr capture, timeout
    kill, argv/cwd/allowed-env/exit/timed-out evidence.
  - `NetworkPolicyClaim::NotEnforced`; phase 0 does not claim `DenyAll`.
- Product shell-out wiring through the sanitized runner.
- P7.z truthfulness hygiene:
  - prompt hash binds canonical provider request bytes;
  - raw-output CID uses provider response bytes;
  - `world_head_unchanged` is observed rather than production-literal;
  - offline/sandbox/browser wording is downgraded to what the code can prove.
- Real-world meaning fixtures:
  - compile failure,
  - regression two-phase,
  - preview DOM contract rather than screenshot oracle,
  - privacy secret-env non-leak,
  - ambiguous requirement hold/non-accept.

Non-claim: TuringOS still does **not** have OS-level hermetic/no-network
sandboxing. The shipped claim is production shell-out process hygiene.

---

## Verification Snapshot

Local orchestrator checks:

```bash
git diff --check
cargo test --test constitution_matrix_drift
RUST_TEST_THREADS=1 bash scripts/run_constitution_gates.sh
```

Constitution gate result:

```text
[k-1-5] total=133 failed=0
```

GitHub checks on PR #78:

- `Constitution gate suite`: pass
- `Feature freeze check`: pass
- `r022_check`: pass
- `validate PR has no sidecar contamination`: pass

Clean-context audits:

- Lovelace: `NO-VIOLATION`
- Curie: `NO-VIOLATION`
- Euler supplemental audit on the gate-runner optimization: `NO-VIOLATION`

---

## Current Main Status

`main` includes:

- PR #3 CAS Git constitutional repair.
- PR #4 Phase 6.0-6.3 alpha CLI stack.
- PR #6 Phase 7 Web MVP.
- PR #11 Phase 6.3.y grill-driven Generative UI ship unit.
- PR #43-#54 Product-CAK Hardening P7.z atoms C0-C11.
- Cz cumulative Trust Root realignment at `9bdaddee`.
- PR #56 session #56 audit/remediation records.
- PR #78 Boundary-Ratification-Hygiene increment at `38adc108`.
- **Plan v7 (MiniF2F partial recovery, 2026-05-21):**
  - PR #82 R0 — `lean_market` binary restored at `2bf282ca` (Tier 1).
  - PR #83 R1 — `batch_orchestrator` promoted to `src/runtime/` at `6148a0cd` (Tier 2).
  - PR #84 R2+Cz — root `Cargo.toml` `exclude = ["experiments/minif2f_v4"]`
    + Trust Root rehash (Cz cycle 3) at `7f61605d`.
  - Hotfix at `cff03a28` — removed Codex Polymarket WIP leak from `src/runtime/mod.rs`
    (R-022 OBS `OBS_R022_R1_EXTERNAL_MARKET_SNAPSHOT_LEAK_2026-05-22.md`).
  - PR #87 archive at `97c8169b` — research bundle at
    `handover/research/PLAN_V7_MINIF2F_RECOVERY_2026-05-22/`.
- **R-022 hook architectural fix (2026-05-21):**
  - PR #88 at `1cfad1a4` — R-022 trace-matrix check moved from
    `pre-commit.r022` to new `commit-msg.r022` (fixes COMMIT_EDITMSG
    read footgun). Postmortem: `handover/architect-insights/R022_HOOK_FIX_2026-05-22.md`.
  - PR #89 at `865b4c14` — constitution gate parity update
    (`l8_pre_commit_hook_chains_k_harden_2_block` flipped + 2 new
    gate tests bind the new architecture).

Migration: existing clones must re-run `bash scripts/install_hooks.sh`
to pick up the new `commit-msg` symlink. Idempotent.

P7.z produced the CAS-backed product evidence chain:

```text
SpecCapsule
  -> GenerationAttemptCapsule
  -> ArtifactBundleManifest
      -> PreviewRunCapsule
      -> TestRunCapsule
      -> GenerateRejectionCapsule (L4.E)
      -> BuildSessionView (derived)
      -> offline replay/spec audit
```

PR #78 then tightened how the project talks about that chain: no fake
hermetic claim, no fake `DenyAll`, no literal world-head self-report, no
dashboard/screenshot/LLM-reviewer truth claim.

---

## Active Non-Claims

- Do not claim complete v2.0 predicate layer.
- Do not claim OS-level hermetic sandbox.
- Do not claim runtime network denial.
- Do not treat screenshots, dashboards, cache, web sessions, or LLM reviews as
  acceptance truth.
- Do not treat MiniF2F as a live root-workspace package; the root workspace
  excludes it. Plan v7 (2026-05-21) restored a partial subset:
  `experiments/minif2f_v4/` is again a separate Cargo workspace housing the
  `lean_market` binary only (Tier 1), and `batch_orchestrator.rs` was
  promoted to `src/runtime/` (Tier 2). All other deleted MiniF2F files
  (Tier 3) remain unrestored.

Allowed wording:

```text
TuringOS has shipped process hygiene for production shell-outs: env allowlist,
explicit cwd, timeout, stdout/stderr capture, and unified runner wiring. This
is not OS-level hermetic/no-network sandboxing.
```

---

## Recommended Next Work

Original 3 options (session #57):

1. Decide whether the next charter is OS-level sandbox phase 1, P7.z
   truthfulness follow-up, or a tiny replayable-decision smoke test.
2. If choosing sandbox phase 1, make the mechanism explicit first:
   process-only, bwrap/unshare/seccomp, or VM/Wasmtime. Do not smuggle this
   into a generic "predicate layer" task.
3. If choosing replayable decision, do not call it the predicate layer yet.
   Keep it to deterministic boolean decision record/replay with no schema
   catalog, oracle, cooldown, or predicate taxonomy.

Additional charters surfaced by session #58 generative HTML probe + Software 3.0 audit
(detail in [synthesis/REPORT.md §6](../research/generative_html_kernel_integrity_2026-05-22/synthesis/REPORT.md)):

4. **Charter A — Generative HTML IR** (closes C10 FAIL, highest-impact). Define
   `GenerativeHtmlIr` JSON schema → generate emits IR first then renders → IR CID into
   `GenerationAttemptCapsule` tail-additive → new `ir_to_html` renderer + test gate.
   Class 2-3. Orthogonal to all 3 options above. Gives TuringOS a unique formally
   auditable + content-addressed IR no commercial comparator has.
5. **Charter B — Web Driven-Mode default + generate prompt hash** (closes C1/C2/C9 WARN).
   Class 1-2. Supersedes P7.z truthfulness on the generate-prompt-hash dimension.
6. **Charter C — Layered eval + sandbox static analysis** (closes C6/C11 WARN + BUG-5
   verifier no fetch detection + BUG-6 new W8 `JsSyntaxValid` gate). Class 2.
   Complementary to OS sandbox phase 1.
7. **Follow-up parallel sessions** for BUG-3a (`generate.rs` step 4b error propagation
   matching spec.rs) + BUG-3b (env allowlist regression test) — both Class 1-2.

---

## Cold-Start File Order

1. `AGENTS.md`
2. `HARNESS_PLAYBOOK.md`
3. `HARNESS_MANUAL.md`
4. `constitution.md`
5. `handover/ai-direct/LATEST.md`
6. `docs/architecture/FC_REAL_WORLD_BOUNDARY.md`
7. `handover/directives/2026-05-21_FC_BOUNDARY_RATIFICATION_DIRECTIVE.md`
8. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
9. `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
