# SOFTWARE_3_0_VAL_KARPATHY audit — 2026-05-23

**Auditor**: clean-context Karpathy-discipline witness
**Verdict**: PASS

Commits audited (all on `main`, in order):
- 7130cf91 (#122) Atom S1 — remove stdout-as-truth
- 486adaa2 (#123) Atom S2 — GrillSession CAS snapshot resume
- 1d35058d (#124) Atom S3 — BuildSessionView error taxonomy
- c2b6d954 (#125) Atom S4.1 — rename siliconflow_client → chat_client
- ac95ac12 (#126) Atom S4.2 — LLM_BOUNDARY_INVENTORY doc
- 32e30d97 (#127) Atom S5 — audit_legacy_bypass.sh + checklist

## K-checks

- **K10 defer abstraction**: PASS.
  `grep -rn "enum ChatProvider" src/` → empty.
  `grep -rn "trait ChatProvider" src/` → empty.
  `grep -rn "ModelCallReceipt" src/` → empty.
  S4.1 performed a pure file-and-module rename (`src/bin/turingos/{siliconflow_client.rs → chat_client.rs}`,
  18 files touched: 1 rename + import updates + doc-comments + test static-check string lists).
  S4.2 (`handover/architect-insights/LLM_BOUNDARY_INVENTORY_2026-05-23.md:70-83, 108-126`) explicitly
  defers the `ChatProvider` enum + `ModelCallReceipt` capsule to a future Class 3/4 packet AFTER
  the 2nd concrete provider lands, citing K10 by name.

- **K11 direct computation**: PASS.
  `src/web/session_snapshot.rs`:
    * `GrillSessionSnapshot` is a `pub(crate)` plain struct (lines 49-75) — mirrors only cache-rebuild fields.
    * `from_session` (lines 95-123) and `into_session` (lines 126-155) are direct field copies — no codec layer,
      no Builder, no Factory.
    * `GRILL_SESSION_SNAPSHOT_SCHEMA_ID` is module-private `const` (line 41, no visibility modifier),
      as the WEB-CLI kernel invariant requires.
  `src/runtime/build_session_view.rs`:
    * `BuildSessionViewError` is a 3-variant enum (`Open` / `Read` / `Decode`) at lines 27-31.
    * `From<BuildSessionViewError> for CapsuleError` propagator at lines 45-53 — no trait objects,
      no generic-error scaffolding (`Box<dyn Error>`, `anyhow`, etc.).

- **K14 no escape hatches**: PASS.
  * S1 removed the `simple_hash` FNV helper and the `t_hash_<hex>` synthesized id fallback outright;
    `grep -rn "t_hash_\|simple_hash" src/web/write.rs` returns empty. No feature gate, no compat alias.
  * S5's `scripts/audit_legacy_bypass.sh` is reporting-only:
    `grep -n "audit_legacy_bypass" scripts/run_constitution_gates.sh` returns empty;
    `grep -rn "audit_legacy_bypass" .github/` returns empty.
    Script header (lines 7-11) and PR body explicitly state it is NOT a constitution gate and NOT wired
    into CI. No new feature flag introduced (`grep -nE "^(compat_|legacy_)" Cargo.toml` → empty).

- **Sum-type over trait+single-impl**: PASS.
  * `BuildSessionViewError` is an `enum`, not a trait + single impl.
  * `grep -n "^trait\b\|^pub trait\b" src/runtime/build_session_view.rs src/web/session_snapshot.rs` → empty.
  * `src/web/write.rs:79-82` — `TaskOpenResponse.task_id: String` (NOT `Option<String>`). Parse failure is
    signaled out-of-band via HTTP 502 + `TaskOpenError.kind = "task_id_parse_failed"`
    (lines 365-376), not by Option absence.

- **Empty-state-is-normal**: PASS.
  `src/runtime/build_session_view.rs:91-107` — when `cas_dir.exists()` is false, `derive_build_session_view`
  returns `Ok(BuildSessionView { current_status: BuildStatus::SpecPending, .. })`, not `Err`. Doc-comment at
  lines 17-19 + 87-90 makes the empty-is-normal contract explicit. Matches
  `feedback_conservative_error_semantics` rule 1 verbatim.

- **Ceremony-free per commit**: PASS.
  Per-commit `git show --stat`:
    * S1: 2 files (`src/web/write.rs`, `tests/cli_web_write_smoke.rs`).
    * S2: 3 files (new `src/web/session_snapshot.rs`, `src/web/mod.rs` mod registration, `src/web/spec.rs`
      caller wiring). No drive-by edits.
    * S3: 2 files (`src/runtime/build_session_view.rs`, new distinction test).
    * S4.1: 18 files, all strictly the rename mechanic (1 file rename + 7 cmd imports + 3 doc-comment refreshes +
      6 test static-check string list updates + 1 binary mod declaration). No code logic change.
    * S4.2: 1 file (the new inventory doc).
    * S5: 2 files (1 script + 1 checklist doc).
  No PR carried Class 4 surface edits, no PR carried surrounding cleanup or helper proliferation.

- **Inventory doc grep match**: PASS.
  Counted with the exact recipe in the audit brief:
  ```
  cmd_init: 0
  cmd_llm: 5
  cmd_generate: 2
  cmd_spec: 3
  cmd_tdma: 5
  cmd_welcome: 0
  cmd_wizard: 2
  ```
  Sum = 17 = the figure claimed in `LLM_BOUNDARY_INVENTORY_2026-05-23.md` §2 line 54
  ("Total chat_complete* sites: 17 (across 5 cmd files).").
  Per-file numbers also match the doc's table exactly.

## Notes

- S2's snapshot uses the existing `EvidenceCapsule` `ObjectType` and registers no new CAS schema — the
  schema id is a string tag (`"turingos-web-grill-session-snapshot-v1"`) kept module-private to respect
  the `tests/constitution_web_cli_kernel_invariant.rs` invariant (web layer must not own `*_SCHEMA_ID`
  pub constants). This is the right Karpathy posture: the derived view stays derived.
- S2's `write_snapshot` returns `Result<String, String>` and the call site treats failures as
  best-effort cache instrumentation (commented as such at `src/web/session_snapshot.rs:158-163` and
  in the §2 PR body). This is consistent with K11 (transparent data flow) and `feedback_audit_after_evidence`
  — cache rebuild failure is not an admission event.
- S3's `From<BuildSessionViewError> for CapsuleError` collapses `Decode → CapsuleError::Read("decode: …")`
  rather than adding a new `CapsuleError` variant. Minor information loss is preserved in the string
  payload; this is a deliberate "don't widen the upstream enum for one consumer" call, consistent with
  K11 surgical-change discipline.
- S5's script greps for `t_hash_*` / `simple_hash` / `// removed` stubs / `panic!()` in src/ /
  `.unwrap()` in src/web/ / `compat_*|legacy_*` Cargo features. Patterns 4 and 5 (panic + unwrap)
  are explicitly labeled "informational" in the script body — they are smell counters, not violation
  gates, which is the correct K14 posture (no escape hatch, but also no over-strict reporting that
  would create review noise).
- No `Manager` / `Factory` / `Engine` / `Platform` / `Framework` types introduced anywhere in this
  consolidation. No new dependency added. No background daemon. No global mutable state.

## Final verdict

PASS
