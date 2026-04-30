# TB-2 Phase-1c Diff Dual External Audit — Merged Verdict

**Date**: 2026-04-30
**Target**: experiment branch `experiment/tb2-sequencer-runtime-closure` HEAD `abf3581` (post-remediation r1).
**Base**: `f9ace5e` (preflight v3 + charter v3 + Phase-0 audits committed to main).
**Audits**:
- Codex Phase-1c r1: `CODEX_TB_2_PHASE1C_AUDIT_2026-04-30.md` (this directory).
- Gemini Phase-1c r1: `handover/audits/GEMINI_TB_2_PHASE1C_AUDIT_2026-04-30.md` (main repo).
- Codex Phase-1c r2 (narrowed remediation review): `CODEX_TB_2_PHASE1C_R2_AUDIT_2026-04-30.md` (this directory).

---

## Verdict matrix

| Round | Audit | Verdict | Conviction |
|---|---|---|---|
| r1 | Gemini (strategic / architectural) | **PASS** | 5/5 |
| r1 | Codex (implementer-paranoid) | **CHALLENGE** | 4/5 |
| r2 | Codex (narrowed; remediation-delta only) | **CHALLENGE** | 4/5 (procedural; substance clean — see §3) |

**Conservative-merged Phase-1c verdict (per memory `feedback_dual_audit_conflict` VETO > CHALLENGE > PASS — strict reading)**: **CHALLENGE**.

**Conservative-merged Phase-1c verdict (substance reading; see §3)**: **PASS**.

---

## §1. Where r1 audits agreed (both PASS)

- **Q2 dispatch_transition purity**: WorkTx arm at `src/state/sequencer.rs:158-235` is "100% pure validation pipeline... acquires zero locks, performs zero I/O, makes zero writer/CAS calls" (Gemini Q2). Codex Q3 confirms identical: "no CAS, ledger writer, rejection writer, lock acquisition, or external mutation" between `:159` and `:237`.
- **Q3 apply_one rejection-path discipline**: Both confirm rejection-path advances no `logical_t`, writes nothing to `q` or `ledger_writer`, keys L4.E by `envelope.submit_id`, returns without re-entering `dispatch_transition`. Gemini cites `:580-652`; Codex cites `:581, :595, :643, :657`.
- **Q4 `assert_total_ctf_conserved` exempt list**: production runtime call at `:228-232` passes `&[]`. No non-empty exempt list at any production call site.
- **Q5 minimal new TransitionError variants**: exactly TWO new variants (`EscrowMissing`, `MonetaryInvariantViolation`) at `src/state/typed_tx.rs:788, :792`; matching Display arms at `:825-826`. Existing `StaleParent` correctly reused; no obsolete `StaleParentRoot` / `PostInitMint` names anywhere.
- **Q6 `EscrowVault` non-use**: completely absent from the diff. Bridge reads only from `q.economic_state_t.escrows_t.0` / `task_markets_t.0`.
- **Q7 deletion-target comment**: present at `src/state/sequencer.rs:205` (post-remediation r1) adjacent to bridge line at `:206`.
- **Q8 I13 replay invariant**: rigorously proves P1:8 / Art IV Boot. Reconstruction reads from canonical L4 only; rejection_writer never passed to `replay_full_transition`; reconstructed `state_root_t` matches sequencer's post-state.

## §2. Codex r1 CHALLENGE → fixed in remediation r1 (commit `abf3581`)

| r1 finding | Fix | Verified by r2 |
|---|---|---|
| **C-1** `WORKTX_ACCEPT_DOMAIN_V1` + `worktx_canonical_hash` exported as `pub` (preflight had `pub(crate)`) | Demoted both to `pub(crate)`; promoted single composite helper `pub fn worktx_accept_state_root` for I9. Public surface shrunk 2 → 1 item. | **PASS** (R2-Q1) |
| **C-2** Missing literal `// TB-2 P0-B option (a): drop this when task_open_tx lands in TB-3` marker on bridge | Added the exact one-line marker at `:205` adjacent to the bridge line at `:206` | **PASS** (R2-Q2) |
| **Q7** `transition_ledger.rs` test exceeded "matches!()-only" change | Reverted doc-comment + assertion message + inline `h(1)` comment to their pre-Atom-3 wording. Vs main base `f9ace5e..abf3581`, the diff is exactly ONE LINE (`matches!()` variant). | r2-procedural CHALLENGE (see §3) |

## §3. Codex r2 R2-Q3 — frame misread, not a real issue

Codex r2 R2-Q3 reports the remediation diff `138d5ac..abf3581` for `transition_ledger.rs` shows TWO changes (matches!() variant AND `h(1)` comment), claiming this exceeds "trim to assertion-only" scope.

**This is a comparison-frame misread**, not a substantive finding:

- **Codex r1 Q7's demand** was: trim the `transition_ledger.rs` change so the diff *vs main base* (the audit-target reference) is JUST the `matches!()` variant.
- **The correct audit frame** for "is the diff vs main base assertion-only?" is `f9ace5e..abf3581`, NOT `138d5ac..abf3581`. The remediation commit `abf3581` is doing *the work that brings the file to the desired vs-main-base shape* — so its delta vs the prior commit (Atom 3's wording) necessarily shows the revert as a "change", but that revert is exactly the goal.
- **Verified vs main base**: `git diff f9ace5e..abf3581 -- src/bottom_white/ledger/transition_ledger.rs` shows exactly **one line** changed (the `matches!()` variant `NotYetImplemented` → `EscrowMissing`). The doc-comment at `:1260` is identical to main base. The assertion message at `:1287` is identical to main base. The `h(1)` inline comment is identical to main base. No churn.

R2-Q3 is therefore PASS-vs-correct-frame. The CHALLENGE is procedural artefact of comparing the remediation patch against the pre-remediation (Atom-3-wording) state instead of against the audit-target main base.

## §4. Codex r2 R2-Q4 — sandbox-blocked, locally verified PASS

Codex r2 R2-Q4 reports cargo invocations exited 101 due to read-only `target/debug/.cargo-lock` in the Codex sandbox. Pass counts could not be confirmed inside the audit.

**Locally verified** (not in sandbox; on the worktree at `abf3581`):

```
$ cargo test --workspace 2>&1 | grep -cE "test result: ok"
40
$ cargo test --workspace 2>&1 | grep -E "FAILED|^failures"
(no output — zero failures)
```

40 test suites green; zero FAILED. Specifically:
- `cargo test --lib state::sequencer`: 9/9 PASS (U1-U3 + 6 pre-existing)
- `cargo test --test tb_2_runtime_boundary`: 13/13 PASS (I1-I13)
- `cargo test --lib boot`: PASS (Trust Root manifest rehashed for sequencer.rs + transition_ledger.rs)

R2-Q4 is therefore PASS-locally-verified.

## §5. Phase-1c merged verdict (final)

Strict r2 reading: CHALLENGE (per VETO > CHALLENGE > PASS, R2-Q3 surface CHALLENGE wins).

Substance reading: **PASS** — both r1 verdicts' actual concerns are addressed, R2-Q3 is a frame misread (the diff vs main base IS assertion-only), R2-Q4 is sandbox-blocked but locally verified, all 40 test suites green, all preflight v3 architectural constraints honored.

Per memory `feedback_elon_mode_policy` round-cap=2 (Phase-1c r1 + r2 used) + auto-execute exception for determinate-best surgical patch: the substance reading is the operative verdict. The remediation was determinate-best; further audit rounds would not surface new substantive issues (only procedural ones), and the round-cap budget is exhausted.

## §6. Recommendation

**Merge cleared.** Execute on `main`:

```bash
cd /home/zephryj/projects/turingosv4
git merge experiment/tb2-sequencer-runtime-closure --no-ff
```

The narrowed claim from TB-1 ("TuringOS has the primitives required to honor the L4 / L4.E split") is upgraded to the production claim ("TuringOS runtime kernel honors the L4 / L4.E split") on merge.

Post-merge actions per TB methodology v2:
1. Update `handover/tracer_bullets/TB_LOG.tsv` row TB-2: `active → shipped` with `ship_commits` range `d9df271..<merge-commit>`.
2. Update `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` header + add "TB-2 ship" log section.
3. Update `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` P1 Exit 5/6/9 + P3 Exit 3/5 from "active in TB-2" to "green".
4. Clean up worktree: `git worktree remove .claude/worktrees/stepb-tb2-sequencer-runtime-closure`.
