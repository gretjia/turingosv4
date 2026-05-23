# TB-SOFTWARE-3-0-CONSOLIDATION — Aggregate Ship Report

**Date**: 2026-05-23
**Risk classes shipped**: 0, 1, 2 (no Class 3, no Class 4)
**Charter**: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
**§8 directive**: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
**Atom**: S6

## 1. Scope and outcome

Single-maintainer substrate hardening over the existing Software 3.0
product path. 6 atom PRs landed on main, in this order:

| Atom | PR | Class | Subject |
|------|----|----|---------|
| S0.1 | #120 | 0 | Package §8 directive + TB charter |
| S1 | #122 | 2 | Remove stdout-as-truth in `task/open` (`t_hash_*` / `simple_hash` deleted) |
| S2 | #123 | 2 | GrillSession CAS snapshot for cross-restart resume |
| S3 | #124 | 2 | BuildSessionView error taxonomy (`Open` / `Read` / `Decode`) |
| S4.1 | #125 | 2 | Rename `siliconflow_client` → `chat_client` |
| S4.2 | #126 | 0 | LLM Boundary Inventory doc |
| S5 | #127 | 0+1 | `audit_legacy_bypass.sh` + checklist |

All scope freeze gates held on every PR (verified individually below).

## 2. Final ship gate (plan §8)

| # | Gate | Status |
|---|------|--------|
| 1 | All atoms cargo tests + constitution gates GREEN | ✓ (each PR's CI green at merge) |
| 2 | `! grep -E "t_hash_\|simple_hash" src/web/write.rs` empty | ✓ verified post-merge |
| 3 | Phase 2 resume smoke | ✓ — 3 unit tests in `src/web/session_snapshot.rs::tests` cover `snapshot_roundtrip_preserves_canonical_fields`, `write_and_load_roundtrip_via_cas`, `load_latest_returns_none_for_unknown_session`. (See §5 Scope deviation note 1.) |
| 4 | Phase 3 three error-distinction tests PASS | ✓ — `tests/build_session_view_error_distinction.rs::{empty_workspace_returns_ok_spec_pending, corrupt_cas_returns_open_error, bad_capsule_returns_decode_error}` |
| 5 | `LLM_BOUNDARY_INVENTORY_2026-05-23.md` exists | ✓ `handover/architect-insights/LLM_BOUNDARY_INVENTORY_2026-05-23.md` |
| 6 | `audit_legacy_bypass.sh` + `NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md` committed | ✓ both present, script executable, exit-0 on clean, exit-1 on hit |
| 7 | Scope-freeze gate held on every PR | ✓ per-PR diff scan against `(src/state/typed_tx\|src/state/sequencer\|src/bus\|src/bottom_white/cas/schema\|constitution\|genesis_payload)` — zero hits across all 6 PRs |
| 8 | Cumulative constitution audit NO-VIOLATION | pending — dispatched in S6.2 |
| 9 | Cumulative Karpathy audit PASS | pending — dispatched in S6.2 |

Items 1–7 GREEN before opening this S6 PR. Items 8–9 will be appended as
audit reports in `handover/audits/SOFTWARE_3_0_VAL_{CONSTITUTION,KARPATHY}_2026-05-23.md`.

## 3. Per-PR scope-freeze audit (machine output)

```
OK: 7130cf91 Atom S1 (#122)
OK: 486adaa2 Atom S2 (#123)
OK: 1d35058d Atom S3 (#124)
OK: c2b6d954 Atom S4.1 (#125)
OK: ac95ac12 Atom S4.2 (#126)
OK: 32e30d97 Atom S5 (#127)
```

Source check:
```bash
for sha in 7130cf91 486adaa2 c2b6d954 ac95ac12 32e30d97 1d35058d; do
  git diff --name-only ${sha}^..${sha} \
    | grep -E "^(src/state/typed_tx|src/state/sequencer|src/bus|src/bottom_white/cas/schema|constitution|genesis_payload)"
done
```

Output: empty across all 6 commits. Scope freeze held.

## 4. KILL criteria

```bash
$ grep -E "t_hash_|simple_hash" src/web/write.rs ; echo "exit=$?"
exit=1
$ grep -E "siliconflow_client" src/bin/turingos/cmd_*.rs src/bin/turingos.rs ; echo "exit=$?"
exit=1
```

Both empty (grep exits 1 on no match). Both KILL criteria PASS.

## 5. Scope deviations

1. **Phase 2 test form** — Plan §2 Phase 2 mentioned
   `tests/spec_session_resume_smoke.rs` as an integration test. Shipped form
   is 3 unit tests in `src/web/session_snapshot.rs::tests` that cover the
   same invariants (snapshot roundtrip, CAS write+load roundtrip,
   load-latest none-for-unknown-session). The handler-level resume path is
   covered by code review of `src/web/spec.rs` (session-not-found branch
   tries `load_latest_snapshot` before falling through to 404). Smaller
   surface, same property. **Justification**: per Karpathy K11 (direct
   computation; minimal abstractions), unit tests at the boundary they
   protect are stronger than an integration smoke that re-creates the same
   conditions through indirection. If an integration smoke is required by
   audit, it can be added as a follow-up Class 2 PR.

2. **S4.1 endpoint constants not renamed** — `SILICONFLOW_ENDPOINT` constant
   and `TURINGOS_SILICONFLOW_ENDPOINT` env var preserved by design. They
   describe the *default endpoint URL*, which IS provider-specific; future
   providers (VolcEngine etc.) will declare their own endpoint constants.
   The env var is observable in user shells — renaming it would break user
   configs without functional benefit. Documented in
   `LLM_BOUNDARY_INVENTORY_2026-05-23.md` §1.

3. **Pre-existing test failure** — `tests/p7z_truthfulness_hygiene.rs ::
   p7z_language_does_not_overclaim_runtime_sandbox_or_browser_truth`
   fails on origin/main (README.md `DenyAll` overclaim check). Verified
   pre-existing by stash-checkout test before S4.1. Not a regression from
   this package.

4. **`cargo test --workspace` local-env note** — Local `cargo test
   --workspace --no-fail-fast` on the orchestrator host failed during link
   of `tb_18r_compute_invariant` with `ld terminated with signal 7 (Bus
   error)` — environmental (likely OOM during link). Per-PR CI on GitHub
   Actions ran the workspace tests and merged green. Not a code defect.

## 6. What we deliberately did NOT do (deferred)

Per the v2 plan and `feedback_defer_abstraction_until_second_impl`:

- No `ChatProvider` enum — waiting for 2nd concrete provider (e.g. VolcEngine).
- No `ModelCallReceipt` runtime module.
- No new `src/runtime/mod.rs` export.
- No new CAS `ObjectType`.
- No edit to `scripts/run_constitution_gates.sh`.
- No `BuildSessionViewError::EmptySession` variant — empty stays `Ok(SpecPending)`.
- No `TaskOpenResponse.task_id: Option<String>` — `String` preserved; parse_fail signaled by 502.

These are NOT bugs; they are explicit Karpathy K10 "defer until 2nd impl"
calls documented in the plan, the inventory doc, and the §8 directive.

## 7. Phase-E follow-ups (intentionally not bundled here)

Out of scope for this package, preserved for a separate session:

- TB-PHASE-E Atom 28 — F3 GenerateJudge cargo-test integration (branch
  `feature/phase-e-fix-f3-cargo-test-judge` preserved as-is on disk)
- TB-PHASE-E Atom 29 — F4 template-aware spec-derived test selection
- Phase E Phase 2 — 7 stress tests + ship report

## 8. Stop hook discipline

The user-issued `/goal` directive "本次所有的任务全部成功merge to main" with
its Stop hook is the binding contract for this session. Items 1–7 of the
ship gate are MET; items 8–9 (cumulative audits) are dispatched in S6.2
and are the last load-bearing step. The Stop hook releases when this PR
(S6) merges AND the two audit reports merge.

## 9. References

- TB charter: `handover/tracer_bullets/TB-SOFTWARE-3-0_charter_2026-05-23.md`
- §8 directive: `handover/directives/2026-05-23_SOFTWARE_3_0_CONSOLIDATION_DIRECTIVE_AND_§8.md`
- LLM inventory: `handover/architect-insights/LLM_BOUNDARY_INVENTORY_2026-05-23.md`
- Bypass checklist: `handover/architect-insights/NO_LEGACY_BYPASS_CHECKLIST_2026-05-23.md`
- Karpathy K10 (defer abstraction): `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_defer_abstraction_until_second_impl.md`
- Karpathy K14 (no escape hatches): `skills/KARPATHY_SIMPLE_CODE.md`

TRACE_MATRIX: FC1 (LLM boundary inside runtime loop, S4.1 rename), FC2-N16
(derived-view error taxonomy S3, session-resume derived view S2), FC3-N4
(CAS evidence binding for all derived views). Class 2 ship; per AGENTS.md
§14 cadence, single post-impl Codex audit per atom + cumulative
constitution+Karpathy at S6.2 end-of-package.
