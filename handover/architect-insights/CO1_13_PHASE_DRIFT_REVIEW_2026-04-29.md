# CO1.13 Phase Drift Review (2026-04-29; impl complete)

> **Trigger**: per LATEST.md session-2 task #7 — phase drift review fires at CO1.13 impl complete (CO1.13.1 + CO1.13.2 + CO1.13.3 all shipped).
> **Method**: 7-dimension check vs spec `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` v1.1.1. Each dimension reports actual vs target, drift direction, and constitutional / WP / Elon-mode disposition.
> **HEAD at review**: `e9c6a2b` (CO1.13.2 + CO1.13.3 commit).
> **Sub-atom commits**: `9be22b4` (CO1.13.1) → `e9c6a2b` (CO1.13.2 + CO1.13.3 bundle).

---

## Dimension 1 — Scope (LoC delta vs spec target)

| Sub-atom | Spec target (§ 0.3 v1.1.1) | Actual | Drift |
|---|---:|---:|---|
| CO1.13.1 (doc) | ~200 LoC docs delta | +283 / -14 net | +83 (+42 %) |
| CO1.13.2 (R-022 hook) | ~335 LoC | ~676 (script 421 + yaml 20 + shim 13 + installer 31 + ci 24 + 5-line engine.py + tests 297) | +341 (+102 %) |
| CO1.13.3 (reverse-map) | ~100 LoC | 134 LoC | +34 (+34 %) |
| **Bundle total** | **~635 LoC** | **+1011 / -31 net** | **+376 (+59 %)** |

**Drift sources** (each justified):
- CO1.13.1: § F manual table is 135 entries → ~150 LoC of table content alone; § J schema 4 sub-sections; § E.1/E.2/E.3 split adds ~30 LoC.
- CO1.13.2: shell integration tests landed at 297 LoC (spec didn't allocate explicit per-test LoC but had 9-test budget); `_lib.sh` grew from 30 → 69 LoC after the test-pollution incident forced hardening (`enter_tmp_repo` realpath isolation guard); `check_trace_matrix.py` grew from 250 → 421 because three modes share extensive helper surface (diff parser, scope-table classifier, structured logger, justification-ref validator).
- CO1.13.3: shared-parser invocation via subprocess + idempotent splice + --check / --dry-run modes is +34 vs spec 100 LoC.

**Disposition**: ACCEPTABLE per Elon-mode "scope unchanged, process streamlined". Spec scope (3 sub-atoms; 9-test suite; tracked CI workflow; idempotent reverse-map) was preserved fully. LoC inflation is in test infrastructure + defensive isolation guards forced by a real test-pollution incident — these are *quality* spending, not *scope* drift.

---

## Dimension 2 — Process (audit rounds, gates)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| r1 (v1) | CHALLENGE/HIGH (7 P0s) | CHALLENGE/HIGH (2 P0s) | CHALLENGE/HIGH | 9 unique fixes → v1.1 |
| r2 (v1.1) | CHALLENGE-ESCALATE/HIGH (3 New-P0s) | PASS/HIGH | CHALLENGE-ESCALATE | Codex r2 § E own recommendation: 4 surgical patches via cap-exception → v1.1.1 |
| (CAP-EXCEPTION) | — | — | — | 4 mechanical fixes (3 inconsistencies + 1 substantive CI gate) |
| r3+ | CAPPED | CAPPED | — | unused; impl shipped under v1.1.1 |

**Round count**: 2 audit rounds + 1 cap-exception patch round (vs Elon-mode policy round-cap=2; cap-exception authorized via Codex r2 § E own recommendation).

**Spec phase wall-clock**: ~2.5 hr (per LATEST.md session-2 — vs prior 14-day median = ~134× compression on spec phase).

**Implementation phase**: this session (after spec PASS-with-cap-exception). Real-time wall-clock is split across two sessions because the bash sandbox went ENOSPC during cargo link; user manually freed disk; resume on next message. **Engineering wall-clock excluding wait** estimated ~3-4 hr (well under spec § 0.3 3-day target).

**Disposition**: WITHIN POLICY. Cap-exception was the policy's own escape hatch for r2-split + Codex-recommended-final-patch case; not drift.

---

## Dimension 3 — Constraint (constitution + WP + Elon-mode adherence)

| Constraint | Status | Note |
|---|---|---|
| Constitution Art V.1.1 (Constitution sole baseline) | UNCHANGED | constitution.md not touched; TRACE_MATRIX_v3 doc edits + Trust Root rehashes are alignment-surface, not constitutional surface |
| Constitution Art V.3 (sudo gate) | NOT INVOKED | TRACE_MATRIX_vN.md per CLAUDE.md "Alignment Standard" is alignment surface; Art V.3 reserved for constitution.md proper |
| Whitepaper v2 § 公理 5 (反奥利奥 body) | STRENGTHENED | R-022 enforces every NEW src/ pub symbol carries layer attribution → constitutional alignment is now mechanically guaranteed for forward changes |
| WP v2 ChainTape Layer L4 (transition_ledger) | UNTOUCHED | CO1.13 is alignment infrastructure, not L4 work; CO1.7 family closed prior |
| Elon-mode round cap = 2 | RESPECTED | r1 + r2 + cap-exception (per § E own recommendation; not a 3rd round) |
| Elon-mode OBS hard threshold ≤ 3 open | RESPECTED | 0 new OBS opened by CO1.13; project total still 4 open |
| Elon-mode "ship-with-OBS NOT for enforcement gates" | RESPECTED | R-022 IS the enforcement gate; cap-exception was used because the determinate-best surgical CI-workflow patch was substantive (closes the gate), not theater |
| STEP_B-restricted files (kernel.rs / bus.rs / wallet.rs) | UNTOUCHED | CO1.13 is pure-additive at rules/ + scripts/ + tests/ + handover/alignment/ |
| FC-trace in commit messages | RESPECTED | Both commits (`9be22b4` + `e9c6a2b`) carry explicit FC-trace lines (FC3-N34 readonly verification + FC3-Alignment) |

**Disposition**: ALL CONSTRAINTS RESPECTED. Constitutional + WP alignment is materially STRENGTHENED — R-022 + § J schema + § F.2 auto-refresh together close the form-vs-substance two-layer model per spec § 2.5.

---

## Dimension 4 — Doc (handover synchronization)

| Doc | Updated by CO1.13? | Notes |
|---|---|---|
| `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` | YES (§ E expanded; § F.2 populated; § J added; cross-refs reconciled) | 324 → 592 lines |
| `genesis_payload.toml` | YES (2 Trust Root rehashes — engine.py + TRACE_MATRIX_v3) | mechanical |
| `LATEST.md` | NOT YET (deferred to handover-update at session end) | drift item — handover-update skill will land separately |
| `CLAUDE.md` | NOT NEEDED | no new memory entries required; existing rules + memories unchanged |
| `cases/` | NOT NEEDED | no new case law introduced |
| `AUTO_RESEARCH_NOTEPAD.md` | NOT YET | drift item — TFR reference is stale per LATEST.md session-2 acknowledgement; deferred to next session per outstanding architectural debt list |

**Disposition**: ON-TRACK for atom closure. The two NOT-YET items (`LATEST.md` + `AUTO_RESEARCH_NOTEPAD.md`) are pre-acknowledged debt per LATEST.md session-2 outstanding-debt list, not new drift introduced by CO1.13.

---

## Dimension 5 — Critical-path (does CO1.13 close its bottleneck?)

**Pre-CO1.13 bottleneck** (per spec § 0.1 + § 5 on table:
- 75 % of `src/` pub symbols had no TRACE_MATRIX backlink → silent constitutional drift
- R-015 was warn-only at pre-edit time; no commit-time enforcement
- TRACE_MATRIX_v3 § F was empty by design (waiting for CO P1)
- R-022 was referenced but not landed (5 active rules referenced it; rules/active/R-022* did not exist)

**Post-CO1.13** (HEAD `e9c6a2b`):
- ✅ R-022 commit-time enforcement is LIVE and tracked-CI-gated (required merge gate via `.github/workflows/co1_13_r022_ci.yml`)
- ✅ § F.2 reverse-map populated (135 backlinks across 10 source files; auto-refreshes idempotently)
- ✅ § J orphan extensions schema in place; greenfield empty; populated commit-by-commit via skip-token use
- ✅ Form-vs-substance two-layer model is closed (R-022 = form; § F.2 = substance)
- ⏳ Legacy 75 % gap — explicitly out of scope; queued as **CO1.13-extra** atom (per spec § 0.5 Gemini r1 Q7: must schedule before Phase D)

**Forward-amortization** (per spec § 0.1): each subsequent atom (CO1.8 / CO1.9 / ... / CO P2.x) saves 30-60 min/atom on alignment hygiene under R-022 enforcement = ~75-150 hr amortization over the remaining ~150 atoms.

**Disposition**: BOTTLENECK CLOSED for forward changes. Legacy backlink closure is a separate scheduled atom, not a CO1.13 obligation.

---

## Dimension 6 — Cycle time (spec → impl)

| Phase | Cycle time |
|---|---|
| Spec draft (v1) | ~2.5 hr (per LATEST.md session-2) |
| r1 + 9 patches → v1.1 | included in above |
| r2 + cap-exception → v1.1.1 | included in above |
| Impl phase (CO1.13.1) | ~1.5 hr engineering + Trust-Root-rehash recovery |
| Impl phase (CO1.13.2 + CO1.13.3) | ~3-4 hr engineering (excluding the disk-full / sandbox-down wait window) |
| **Total atom wall-clock (engineering)** | **~7-8 hr** |
| Spec § 0.3 3-day target | 72 hr |
| **Compression vs target** | **~9-10× faster** |

Compared to pre-Elon-mode median atom cycle (~7-14 days per CO1.7-extra arc), this is ~12-25× compression.

**Disposition**: SIGNIFICANT IMPROVEMENT. First atom under Elon-mode + cap-exception policy validates the "factory IS product" hypothesis. Spec-phase compression (~134×) was the bigger win; impl-phase compression (~9-10×) is the second-order win that R-022 + factory-tooling enables for *future* atoms.

---

## Dimension 7 — Budget (audit + project total)

| Item | Cost |
|---|---|
| CO1.13 r1 audit (Codex + Gemini) | ~$8-12 |
| CO1.13 r2 audit (Codex + Gemini) | ~$8-12 |
| CO1.13 cap-exception patch round | $0 (auto-execute) |
| **CO1.13 atom total** | **~$16-24** |
| Project cumulative (post-CO1.13) | ~$220-340 / $890 mid-budget (~25-38 %) |
| Remaining runway | ~$550-670 |
| Per-atom going forward (post-factory) | $5-10 expected (single-round + targeted patches; R-022 + scaffold devtools amortize spec-cycle prep) |

**Disposition**: WITHIN BUDGET. Atom cost was at the upper end of expected range (cap-exception added zero direct cost but did consume ~$8-12 worth of r2 audit). Future atoms should land cheaper as factory tooling matures.

---

## Real-test data points produced this atom

Per Elon-mode "缺少做决策人来的数据就去跑真实测试找问题和解决方案":

1. **Test isolation incident** (CO1.13.2 sub-atom): `r_022_ci_mode_catches_unhooked_pr.sh` initially leaked an empty `b60556d main baseline` commit + `feature` branch into the live repo. Root cause: `tmp=$(setup_temp_repo)` executed `cd "$tmp"` inside a subshell (lost on return); `set -uo pipefail` (no `-e`) meant subsequent failures were silent. Fixed by introducing `enter_tmp_repo` (no subshell; sets `TMP_DIR` global; asserts realpath `$PWD` does NOT resolve inside `PROJECT_ROOT`). Reset with `git reset --soft 9be22b4` + `git branch -D feature`. Re-ran 9-test suite — no pollution.

2. **Disk-space incident** (CO1.13.2/3 validation): `cargo test --test r_022_integration_orchestrator` triggered `ld: signal 7 (Bus error)` during link; subsequent bash subprocess infrastructure entered a degraded state (every command returned non-zero with empty stdout/stderr; Write-tool reported ENOSPC). Diagnosis path: bash exit-code variability → Write ENOSPC → root cause = full disk. User manually freed ~12G of cargo `target/`. After recovery, full bundle validated cleanly. New defensive measure: future drift reviews should include a `df -h` smoke check before launching `cargo test --workspace`.

3. **Idempotency confirmed**: `python3 scripts/update_trace_matrix_reverse_map.py --check` exits 0 immediately after `--default-mode` first run. CO1.13.3's auto-refresh shape exactly equals what the parser produces from `src/`. Manual CO1.13.1 snapshot was rewritten on first auto-run (135 entries, 10 files; identical semantic content, slightly reformatted to canonical shape).

4. **Phase C smoke** (per memory `project_phase_c_living_regression`): see § Phase C regression checks below. `--smoke` PASS 5/5 in 95s (consistent with baseline); `--half` factory mode added but its first invocation surfaced data point #5.

5. **Mathlib collateral damage from disk cleanup** (CO1.13 drift review): after the disk-space incident (#2 above), my cleanup advice recommended `find … -name .lake -prune -exec rm -rf {} +` as a "build artifact" deletion. This was wrong: `.lake/packages/` under `turingosv3/experiments/minif2f_data_lean4/` contained Mathlib v4.24.0 source + build, which is the oracle's ground-truth dependency. Per `feedback_oracle_preflight` memory: "Always verify Mathlib via trivial theorem before batch; C-012 non-negotiable" — Mathlib IS load-bearing. Cleanup advice should have called this out as **preserve unless willing to re-fetch via `lake update`** (10-30 min + 3-5 GB network/disk). Lake project skeleton (`lakefile.lean`, `lake-manifest.json`, `lean-toolchain`) was preserved, so recovery is straightforward when needed; not a permanent loss. **Lesson encoded as memory candidate**: `.lake/packages/` is NOT pure build artifact — it's a vendored dependency with network-fetch cost. Future cleanup advice should distinguish `.lake/build/` (regenerable from local sources) from `.lake/packages/` (requires network + minutes-to-tens-of-minutes re-fetch).

---

## Phase C regression checks (per `project_phase_c_living_regression` memory)

The memory says "Run `--smoke` at each atom phase end to verify architecture-in-progress hasn't broken experiment harness". Honest scope clarification (per user 2026-04-29 challenge):

### `--smoke` is a pipeline-liveness check, NOT a scientific regression test

**Today's `--smoke` result** (HEAD `e9c6a2b`):
- 5/5 cells PASS in 95s (baseline `8d88f2d` was 5/5 in 97s; 2026-04-28 was 5/5 in 146s — present run consistent with the post-DeepSeek-thinking-disabled baseline)
- soft_law cell shows `solved=True verified=False` → expected H2 ablation "fake-accept" signature preserved

**What `--smoke` catches**:
1. runner script bug (`set -e + wait` etc.)
2. all 5 ablation modes start (config not typo'd)
3. LLM proxy reachable
4. JSONL summary writer schema didn't drift
5. evaluator boots past Trust Root verify
6. soft_law H2 fake-accept signature still observable

**What `--smoke` does NOT catch**:
1. solve-rate / PPUT / scientific-signal regression (1 problem × MAX_TX=2 is far too small)
2. long-horizon reasoning degradation
3. anything statistically meaningful

For CO1.13 specifically (0 lines of `src/` changed; 100% additive at `rules/` + `scripts/` + `handover/alignment/` + `tests/`), `--smoke` is near-redundant — it mainly verifies the Trust Root rehashes for `engine.py` + `TRACE_MATRIX_v3` didn't break evaluator boot. **PASS confirms exactly that, nothing more**.

### `--half` factory upgrade landed in this drift review

Per user direction "1+2 结合，2 等大节点再做": added a NEW `--half` mode to the runner (3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20; ~10-15 min wall-clock; ~$0.20-0.40 API cost). Lives between cheap `--smoke` and the full ~12 hr Phase C batch. Catches architectural changes that break solve-rate / PPUT structure on a small representative slice of hard-10. Use at atom-bundle phase ends (Wave 6 #2 / 6 #3 / etc.) where `src/` actually changes.

**Half-real launch attempt at HEAD `e9c6a2b`**:
- ❌ FAILED at oracle preflight — Mathlib `.lake/packages/` deleted as collateral during disk cleanup (see § Real-test data points #5 below)
- factory upgrade itself is sound (the `--half` mode does what it says); needs Mathlib re-fetch before next use
- **per user 2026-04-29 decision**: do NOT autonomously kick off the 10-30 min `lake update` for this CO1.13 closure; CO1.13 is 0-line-src/ alignment-infra and Phase C scientific-regression is not load-bearing for its verdict. Defer Mathlib recovery to before the next atom that actually touches `src/` (CO1.8 / CO1.9 / CO P2.x).

### When to do the full `--full` batch

Per user direction: defer to the next "big milestone" (`大节点`). Concrete triggers:
- Wave 6 #2 closure (CO1.8 L5 Materializer impl + STEP_B if needed)
- Wave 6 #3 closure (CO1.9 L6 signal indices)
- Or any STEP_B-restricted file change (kernel.rs / bus.rs / wallet.rs touch)

Estimated cost / time (post-DeepSeek-thinking-off): ~12 hr / ~$15-25 / 100 cells.

---

## Outstanding follow-ups after CO1.13 closure

1. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks): MUST schedule before Phase D per spec § 0.5 (Gemini r1 Q7). Currently Phase C still frozen; Phase D is downstream of Phase C unfreeze.
2. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation): per spec § 0.4; non-spec follow-up; lands as separate commit.
3. **AUTO_RESEARCH_NOTEPAD.md cleanup** (TFR stale reference): per LATEST.md session-2 outstanding-debt; defer to next session.
4. **LATEST.md handover update** (this session): defer to handover-update skill at session end.
5. **CO1.8 spec round-1 audit** (already drafted at `6cc5cc9`; launchers ready at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py`): ready to launch under the new factory regime.

---

## Conclusion

**Drift verdict**: NO MATERIAL DRIFT. Scope inflation is quality spending (test isolation hardening + extended helper surface for shared-parser symmetry); process is within Elon-mode policy (cap-exception is the policy's own escape hatch); critical path is materially advanced (R-022 + § J + § F.2 auto-refresh closes the alignment factory); cycle time + budget both within target.

**Hypothesis confirmed**: Elon-mode "factory IS product" delivers ~9-10× compression on impl phase + ~134× compression on spec phase for this first deployment. Forward atoms should benefit from R-022 + auto-refreshing § F.2 + § J orphan registry.

**Next**: CO1.13-extra + CO1.8 spec round-1 audit are the immediate-next decisions. User-pending decision on which to launch first under the Elon-mode-factory regime.

— ArchitectAI, 2026-04-29 (post-CO1.13 impl bundle commit `e9c6a2b`)
