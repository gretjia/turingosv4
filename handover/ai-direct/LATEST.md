# TuringOS v4 — Handover State

> Agent cold start: read `AGENTS.md`, `HARNESS_PLAYBOOK.md`, and
> `skills/SUBAGENT_HARNESS.md` before this file. This file is a derived view,
> not a source of truth. ChainTape/CAS and executable gates win on conflict.
>
> Hard rules: PR-only workflow, no `git push origin main`, no wildcard staging,
> no sidecar staging. See `AGENTS.md` §14a.

---

## Current Snapshot

**Session**: #58 open (probe + audit), 2026-05-22. Previous: #57 close 2026-05-21.

**Main tip**: `97c8169b` (post pull `300fb563..97c8169b`, 90+ PRs since session #57's `38adc108`).

**State**: P7.z complete + Boundary-Ratification-Hygiene shipped. **Session #58 ran a
generative HTML kernel-integrity probe + Software 3.0 audit on branch
`claude/generative-html-kernel-probe-20260522`**, surfacing 5 real kernel bugs in
`src/web/spec.rs` + `src/web/generate.rs` (2 LANDED via parallel sessions with tests; 2
in tree pending formal validation; 1 forward-bound). Software 3.0 conformance: 3 PASS /
6 WARN / 2 FAIL (rubric C1-C11). FAIL = C8 no cross-session agent memory + C10 no
generative HTML IR. Probe research + bug fixes pending PR open. See
[handover/research/generative_html_kernel_integrity_2026-05-22/synthesis/REPORT.md](../research/generative_html_kernel_integrity_2026-05-22/synthesis/REPORT.md).

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
- Do not treat MiniF2F as a live root-workspace package; it was removed from
  this repository during P7.z.

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
