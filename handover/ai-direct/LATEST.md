# TuringOS v4 — Handover State

> Agent cold start: read `AGENTS.md`, `HARNESS_PLAYBOOK.md`, and
> `skills/SUBAGENT_HARNESS.md` before this file. This file is a derived view,
> not a source of truth. ChainTape/CAS and executable gates win on conflict.
>
> Hard rules: PR-only workflow, no `git push origin main`, no wildcard staging,
> no sidecar staging. See `AGENTS.md` §14a.

---

## Current Snapshot

**Session**: #58 close, 2026-05-21 (late evening).

**Main tip**: `865b4c14` — `fix(harness): update constitution gate test
after R-022 hook migration` (PR #89 squash).

**State**: P7.z + Boundary-Ratification-Hygiene remain complete; session
#58 shipped two additional increments:
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

1. Decide whether the next charter is OS-level sandbox phase 1, P7.z
   truthfulness follow-up, or a tiny replayable-decision smoke test.
2. If choosing sandbox phase 1, make the mechanism explicit first:
   process-only, bwrap/unshare/seccomp, or VM/Wasmtime. Do not smuggle this
   into a generic "predicate layer" task.
3. If choosing replayable decision, do not call it the predicate layer yet.
   Keep it to deterministic boolean decision record/replay with no schema
   catalog, oracle, cooldown, or predicate taxonomy.

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
