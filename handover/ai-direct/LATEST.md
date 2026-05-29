# TuringOS v4 — Handover State

> Agent cold start: read `AGENTS.md`, `HARNESS_PLAYBOOK.md`, and
> `skills/SUBAGENT_HARNESS.md` before this file. This file is a derived view,
> not a source of truth. ChainTape/CAS and executable gates win on conflict.
>
> Hard rules: PR-only workflow, no `git push origin main`, no wildcard staging,
> no sidecar staging. See `AGENTS.md` §14a.

---

## Current Snapshot (2026-05-27)

**Session**: OBL-001 DeepSeek Chrome E2E closure merged via PR #206.

**Main tip**: `bb364f35` (PR #206 — Class 2 OBL-001 DeepSeek Chrome E2E closure).

**Doctrine ratification 2026-05-29**: audit doctrine generalized to
platform-agnostic clean-context audit — one clean-context audit by a fresh
agent on any capable platform (Claude / Codex / Antigravity / …), auditor must
not hold the implementation transcript. Supersedes single-Codex (2026-05-24)
and dual Codex+Gemini. See `AGENTS.md §9` + `memory/feedback_dual_audit.md`.
(Landing via the `harness/platform-agnostic-unification` PR.)

Current state:

- PR #206 merged `codex/obl001-deepseek-chrome-e2e` into `main`, closing
  OBL-001 with a real Chrome E2E runner, final evidence, and audit. The runner lives at
  `scripts/obl001_deepseek_chrome_e2e.mjs` and is classified as dev-only
  evidence tooling in `tests/fixtures/liveness/script_liveness_inventory.toml`.
- Final evidence root:
  `handover/evidence/obl001_deepseek_chrome_20260527T171150Z/`. `metrics.json`
  records `ok=true`, `status=complete`, `Completed 15/15 personas`, 18 attempted
  personas, and 15 completed personas. Per-persona transcripts/screenshots,
  redacted configs, workspaces, ChainTape/L4 refs, and CAS artifacts are under
  the same root.
- Secret hygiene is clean for the final run: `redaction_audit.json` has
  `secrets_found=false` and `findings=[]`; an independent `rg` scan for
  `hf_...`/`sk-...` patterns over the final evidence and runner script returned
  no matches.
- Clean-context audit artifact:
  `handover/audits/OBL001_DEEPSEEK_CHROME_E2E_CLEAN_CONTEXT_AUDIT_2026-05-27.md`
  returned `NO-VIOLATION`. Auxiliary AGY evidence retry also found no blocker.
- `OBLIGATIONS.md` now marks OBL-001 as `satisfied`; OBL-002 through OBL-005
  are already satisfied.
- The stale OBL-004 reconciliation gate was updated to accept the new global
  `COMPLETE` ledger headline while retaining the OBL-004 section, audit, stale
  placeholder, and merged-PR receipt checks.
- PR #206 GitHub check `validate PR has no sidecar contamination` passed before
  merge. No OBL-001 closure action remains open.

Recent verification:

```text
rustfmt --edition 2021 --check src/bin/turingos/cmd_generate.rs src/web/generate.rs src/web/spec.rs src/web/welcome.rs src/web/market_view.rs tests/constitution_obl005_final_closure_witness.rs tests/constitution_obligation_repair_reconciliation.rs
# exit 0

node --check scripts/obl001_deepseek_chrome_e2e.mjs
# exit 0

git diff --check
# exit 0

cargo check --features web --bin turingos --bin turingos_web
# exit 0

cargo test --features web --bin turingos blackbox_system_prompt_contains_tdma_state_update_contract
cargo test --features web --bin turingos blackbox_system_prompt_tdma_example_matches_parser_schema
cargo test --features web --bin turingos_web web_subprocess_timeout_is_at_least_1800_secs
cargo test --features web --bin turingos_web accepted_turns_force_synthesis_above_threshold
cargo test --features web --test cli_web_generate_smoke web_generate_args_include_entrypoint_index_html
cargo test --features web --test cli_web_welcome_smoke welcome_init
cargo test --test constitution_obligation_repair_reconciliation -- --nocapture
cargo test --test constitution_obl005_final_closure_witness -- --nocapture
cargo test --test constitution_matrix_drift
cargo test --test constitution_script_liveness_inventory -- --nocapture
# all exit 0

bash scripts/run_constitution_gates.sh
# [k-1-5] total=165 failed=0
```

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
