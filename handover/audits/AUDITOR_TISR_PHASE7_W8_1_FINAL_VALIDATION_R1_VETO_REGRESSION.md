# TISR Phase 7 W8.1 Final Validation — Round 1 — VETO_REGRESSION

- **Audit timestamp**: 2026-05-18T04:59:18Z
- **HEAD under test**: `f879f4f3` (W8.1: relax has_playfield + harmonize workspace default)
- **Server PID**: 92364 (W8.1 binary, fresh start, no `TURINGOS_WEB_WORKSPACE` env)
- **Evidence dir**: `handover/evidence/stage_phase7_w8_1_final_validation_20260518T044722Z/`
- **Final session**: `tmp/phase7_active/sessions/1779080045_3e212b67/` (spec phase only — generate never reached)
- **Verdict**: **VETO_REGRESSION**

---

## 1. Verdict semantics

W8.1 introduced a partial workspace harmonization that **broke the spec→generate
hand-off**. `/spec/submit` now writes the session to `tmp/phase7_active/sessions/`
(the new W8.1 default), but `/generate` still resolves the workspace via
`std::env::current_dir()` and looks for the session at
`<repo-root>/sessions/`. The two endpoints disagree on where the session
lives. Result: every user reaching the "生成代码 →" button receives HTTP 400
"session not found", with no path forward. The W8 retry loop and the W8.1
has_playfield relaxation cannot be exercised at all — the regression sits
upstream of both.

This is a strict regression vs Round 2 (W8 validation):
- Round 2: spec+generate both worked, W8 retry fired 3 times, has_playfield
  was the bug.
- Round 3 (W8.1): spec works, generate is a 400-wall.

VETO_REGRESSION (not CHALLENGE_FIXABLE) because the failure mode prevents the
real-LLM substrate from running end-to-end at all. The fix is one-line trivial
(see §6), but the binary currently in production cannot deliver any artifact.

---

## 2. W8.1 efficacy on Qwen output: NOT TESTED

The has_playfield relaxation could not be exercised this round. Qwen was never
called. The regression-test artifact (sha256 a857599b…d666 from Round 2) still
demonstrates the unit-level fix is correct, but end-to-end behavior is unknown.

False-positive count for has_playfield this round: **N/A — never invoked**.

---

## 3. W8 retry behavior: NOT TESTED

- Total `/api/generate` POST attempts this round: 2
- Both returned **HTTP 400** before any LLM call
- WS attempt envelopes emitted: **0**
  - `generate_attempt_started`: 0
  - `generate_attempt_failed`: 0
  - `generate_complete`: 0
- No malformed envelopes (because no envelopes)

The W8 retry pipeline has zero coverage this round — the BadRequest fires in
`generate_handler` at `src/web/generate.rs:147-158`, which is *before*
the retry-loop entry point at line 190+.

---

## 4. Final artifact 8 mechanical tests: ALL N/A

No final artifact was produced. The 8 mechanical tests cannot be executed.

1. iframe sandbox = "allow-scripts" only — N/A
2. 10×20 playfield rendered — N/A
3. Keyboard input handling — N/A
4. Line clear scoring — N/A
5. Game over detection — N/A
6. localStorage high-score persistence — N/A
7. No login/network/ads — N/A
8. Restart button visible — N/A

---

## 5. P2 verification (sessions/ location)

**Partial PASS, partial FAIL — split brain**.

- Spec submit endpoint (W8.1) correctly wrote session under
  `tmp/phase7_active/sessions/1779080045_3e212b67/` ✅
- Backend observer confirmed across 42 iterations: `repo_root_sessions=0`,
  `workspace_sessions=1` throughout ✅
- Repo root remained clean of stray `sessions/`/`cas/` directories ✅
- Generate endpoint (NOT touched by W8.1) still expects
  `<cwd>/sessions/...` ✗
- Net: P2 is half-fixed; the user-facing flow is broken because the two halves
  don't agree.

---

## 6. Root cause (one-line trivial fix)

`src/web/generate.rs:558-567` was NOT updated by commit `f879f4f3`:

```rust
fn resolve_workspace() -> String {
    if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            return v;
        }
    }
    std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| ".".to_string())
}
```

Compare with the harmonized `src/web/spec.rs:521-528`:

```rust
fn resolve_workspace() -> String {
    if let Ok(v) = std::env::var("TURINGOS_WEB_WORKSPACE") {
        if !v.is_empty() {
            return v;
        }
    }
    "tmp/phase7_active".to_string()
}
```

W8.1's commit message claims:
> `resolve_workspace()` defaults harmonized to `tmp/phase7_active` across
> spec.rs/write.rs/artifact.rs (was cwd; caused sessions/ to land in repo root)

But `src/web/generate.rs` was overlooked. `git show f879f4f3 -- src/web/generate.rs`
produces empty diff.

The fix is a 4-line change: replace lines 564-566 of generate.rs with
`"tmp/phase7_active".to_string()`, matching the other three modules.

---

## 7. Tape + CAS integrity (spec phase)

The spec phase that DID complete shows clean tape integrity:

| Check | Result |
|---|---|
| spec.md sha256 | `7ff97a2a9796fd4460e71e46f3befd6fc78999412433d61caaca493d96679032` |
| UI-broadcasted CAS CAPSULE CID prefix | `7ff97a2a...96679032` |
| Match | **BIT-EXACT** ✅ |
| CAS dir structure | git-backed (`.git/objects/`, `.git/refs/chaintape/cas/`) ✅ |
| `turingos welcome --workspace tmp/phase7_active` cross-check | Steps 1-3 complete, step 4 (spec) marked incomplete (CLI looks for state.json not present), step 5 (generate) incomplete — consistent with FS state ✅ |

---

## 8. Cost + wall clock

| Metric | Value |
|---|---|
| Wall clock start | 2026-05-18T04:47:22Z |
| Wall clock end | 2026-05-18T04:59:18Z |
| Total | ~12 minutes |
| SiliconFlow calls | 1× DeepSeek V3.2 (spec synthesis) |
| Qwen calls | **0** (regression prevented generate path) |
| Estimated cost | ~¥0.15 (DeepSeek-only; budget of ¥0.45 was preserved by failure) |

---

## 9. WS envelope summary

| Envelope type | Count this round |
|---|---|
| `generate_attempt_started` | 0 |
| `generate_attempt_failed` | 0 |
| `generate_complete` | 0 |
| `generate_warning` | 0 |
| `welcome_*` | several (Phase 1 init flow) |
| `spec_*` | several (Phase 2 interview flow) |

The two `/api/generate` POSTs both failed at the validation step before any
WS broadcasting commenced.

---

## 10. Single-line recommendation

**one-more-fix** — apply the 4-line patch to `src/web/generate.rs:558-567`,
rebuild, re-run Round 4 final validation.

---

## 11. Evidence inventory

```
handover/evidence/stage_phase7_w8_1_final_validation_20260518T044722Z/
├── audit/
│   └── tape_cas_audit.txt
├── backend_observer/
│   ├── log_stream.txt        (server stdout — boot only; binary doesn't log LLM)
│   ├── notable_events.txt    (empty — grep filter found no notable lines)
│   ├── observer.sh           (the polling script)
│   ├── p2_check.txt          (42 iterations, all show clean repo root)
│   ├── process_tree.txt      (server PID 92364 stable throughout)
│   └── workspace_evolution.txt  (full directory tree per iter)
└── user_simulator/
    ├── regression_evidence.txt
    └── user_simulator_phase1_spec.txt
```

---

## 12. Honest summary for architect

W8.1 ships with a self-introduced regression that takes the user-facing flow
from "broken but reaches the LLM" (W8 had has_playfield bug, but did invoke
Qwen and produce artifacts) to "broken before reaching the LLM" (W8.1 returns
400 before Qwen is even called). The unit-level fixes in W8.1 are correct in
isolation — `has_playfield()` correctly accepts the previously-rejected
artifact, and the regression test pins this behavior. The integration error
is a forgotten 4-line change in one of four web modules.

Net for the architect: this binary is NOT shippable. The fix is trivial (apply
the same replacement to generate.rs that was applied to spec.rs/write.rs/
artifact.rs), but it must be applied, built, and re-validated before any
end-user ever sees the system.

The architect should also consider: this is the second time in the W8 series
that a partial harmonization has caused a hand-off failure. A single
`tos_workspace::resolve()` helper (or a `WorkspaceConfig` extracted at server
start and shared via `Arc<AppState>`) would prevent the recurring drift. That
is a Phase 7.z candidate, not a blocker for the immediate fix.
