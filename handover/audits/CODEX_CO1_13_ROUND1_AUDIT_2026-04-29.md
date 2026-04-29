# Codex CO1.13 Round-1 Audit
**Date**: 2026-04-29
**Target**: spec v1 (greenfield; first Elon-mode 2-round-cap atom)
**HEAD**: 8d88f2d5b3431795741b584f5417c8184f984f5e
**Prompt size**: 62556 chars

---

Reading prompt from stdin...
OpenAI Codex v0.125.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019dd7ad-3396-7f82-b8f1-d45c92e9e68e
--------
user
# Codex Adversarial Audit — CO1.13 v1 TRACE_MATRIX Impl + R-022 Hook (Round 1; greenfield + Elon-mode)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini round-1 (parallel).

**Mandate**: round-1 dual external audit on CO1.13 v1 — a 3-sub-atom factory bundle that lands TRACE_MATRIX_v3 doc completion + R-022 commit-time hook + reverse-map § F population. Per CLAUDE.md "Audit Standard": Generator ≠ Evaluator. Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS. **Per NEW Elon-mode policy (this v1 is the FIRST application)**: round cap = 2; ship-with-OBS if r2 still CHALLENGE.

**Background**:
- CO1.7-extra (final L4 atom; STEP_B closed) shipped 2026-04-29 (commit `4a978f0`)
- CO1.8 v1 spec drafted (commit `6cc5cc9`) but its round-1 audit was DEFERRED in favor of CO1.13 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms)
- CO1.13 is the canonical "TRACE_MATRIX impl + R-022 hook" atom per SPRINT_DEPENDENCY_GRAPH line 129 (3 atoms)
- TRACE_MATRIX_v3 is the existing 324-line doc; CO1.13.1 closes its empty/stub fields
- 15 active YAML rules already exist; CO1.13.2 lands R-022 as the 16th (within 30-rule cap)
- ~75% of src/ pub symbols lack TRACE_MATRIX backlinks (87/354 = 24.6% baseline coverage)

## What you're reviewing

1. **Spec doc**: `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` (~283 lines)
2. **Frozen primary references**:
   - `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` — 324 lines; the doc CO1.13.1 closes
   - `docs/rules.md` — rule engine architecture
   - `rules/SCHEMA.yaml` — YAML rule schema
   - `rules/active/R-015_trace_matrix_pub_symbol.yaml` — existing pre-edit warn variant; R-022 is its commit-time block sibling
   - `rules/engine.py` (145 LoC) — rule evaluator (CO1.13.2 extends with `external_script` check type)
   - `.claude/hooks/judge.sh` — existing pre-edit hook (R-022 lands separate `.git/hooks/pre-commit` shim)

## Round-1 audit questions (7)

**Q1. R-022 enforcement boundary correctness**:
- Spec § 2.1 says: "for each NEW pub line, reads 5 lines preceding... greps for /// TRACE_MATRIX ... if not found, greps TRACE_MATRIX_v3.md § 3... else BLOCK"
- Edge case: pub symbol inside a `#[cfg(test)]` mod tests block — should R-022 apply, or are test pub items exempt?
- Edge case: pub symbol re-exported from a child module via `pub use` — already-traced child symbol shouldn't double-require trace at re-export site
- Edge case: macro-generated pub items — backlink can't be added (no source location)
- Spec § 2.2 escape hatch `// R-022-skip: <reason>` — is the reason captured in stats / logged for quarterly audit, or just permitted silently?

**Q2. The 5 open questions** (spec § 5):
- **Q1 forward-only vs edit-also**: spec § 0.5 #2 says "does NOT enforce backlinks on legacy pub symbols". But what about pub symbols that EXISTED but are MODIFIED (e.g., signature change adding a parameter)? Author lean: forward-only (NEW additions). Is this the right call, or does signature change deserve re-tracing?
- **Q2 5-line window**: is this empirically validated? What % of current backlinks are within 5 lines vs further away? If the convention isn't 95%+ within 5 lines, the heuristic is wrong.
- **Q3 escape hatch rigor**: `// R-022-skip: <reason>` permissive in v1. Should it require `cases/Cxxx` reference now, or accept abuse risk for v1?
- **Q4 reverse-map source of truth**: doc-comments authoritative (spec lean) — but what if a doc-comment is removed in a later edit? Does R-022 catch the *removal* of an existing backlink, or only missing on creation?
- **Q5 R-015 retention**: defense in depth (spec lean). Or does R-015 (warn) become noise if R-022 (block) is the real gate?

**Q3. The 3-sub-atom decomposition**:
- CO1.13.1 doc completion: ~150 LoC docs delta. Is this an audit gate or just a "document hygiene" task? If hygiene, why is it in the audited spec scope?
- CO1.13.2 R-022 hook: ~165 LoC across YAML + Python + shell hook. Does this belong in `rules/active/` (with the other rules) or in a dedicated `hooks/` dir?
- CO1.13.3 reverse-map: ~100 LoC bash. Why bash and not Python (since check_trace_matrix.py is already Python)? Code hygiene risk: 2 languages for similar logic.

**Q4. The `external_script` YAML check.type extension**:
- Spec § 1.2 says: "delegates to script via new `check.type: external_script` extension". This is a new extension to `rules/SCHEMA.yaml` not currently supported.
- Is this extension architecturally clean? Alternative: keep `check.type: grep` and have engine.py detect `R-022` rule_id specifically + dispatch to script. (Avoids schema change.)
- Does the extension need to update SCHEMA.yaml documentation as part of CO1.13.2? Spec doesn't mention schema.yaml update.
- Backward compat: what happens to existing rules if `external_script` type is added — do they still load cleanly?

**Q5. Test plan adequacy** (spec § 3, 5 tests):
- 3.1 `r_022_blocks_missing_backlink.rs`: stages a fake pub symbol; runs check_trace_matrix.py; asserts exit 2. **Is this a Rust test or a shell test?** Rust tests can't easily invoke a Python script + git hooks. Should be `tests/r_022_*.sh` integration test instead.
- 3.4 `trace_matrix_reverse_map_idempotent.rs`: idempotency check. But the script is bash (CO1.13.3). Mismatch: Rust testing a bash script. Better to make it `tests/trace_matrix_reverse_map_idempotent.sh`.
- 3.5 `trace_matrix_v3_doc_coverage.rs`: tests that every Constitution Article + WP § has a populated Class column. **What does this test actually do — read the markdown file and parse rows?** Brittle to format changes.
- Missing tests: no test for the escape hatch path (§ 2.2). No test for the legacy-symbol-modified case (Q1 above).

**Q6. Forward sustainability**:
- Spec § 0.4 #1-#3 lists scaffold scripts as "non-constitutional devtools, no audit". But these scripts (scaffold_co_spec.sh / scaffold_audit_launcher.sh / rehash_trust_root.sh) directly affect future spec-drafting + audit-launching cycles. They're load-bearing for the Elon-mode hypothesis ("cycle time 14d → 2d"). Should they be in the audited scope to ensure quality?
- If they're not audited and turn out to be buggy, every subsequent atom suffers. Risk vs cost trade-off.

**Q7. Strategic risks not yet flagged**:
- Per memory `project_thesis`: "Frozen 5-step compile loop". Does CO1.13 advance the loop, or pure infrastructure? If infrastructure, what's its 11-atomic-claim audit score?
- Per Elon-mode: this is the FIRST atom under the 2-round audit cap. If r1 is CHALLENGE/CHALLENGE and r2 is still CHALLENGE, does ship-with-OBS actually capture the unresolved issues, or does it sweep them under the rug?
- Cycle time: spec says "2-day target". If the 2-day target is missed (e.g., takes 5 days due to round-2 patches), does the Elon-mode policy auto-relax or does the user need to reauthorize?

## Verdict format

Section A: Verdict (PASS/CHALLENGE/VETO) with conviction (LOW/MED/HIGH).
Section B: P0 blockers (must-fix before round-2).
Section C: Open questions raised (architectural).
Section D: Suggested patches (specific spec line/section edits).
Section E: Forward-sustainability notes.

Be concrete. Cite spec § + line where possible.



---

# XREF: spec — handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md

# CO1.13: TRACE_MATRIX_v3 Implementation + R-022 Hook v1 ⏳ pre-audit (round-1 pending)

**Status**: v1 (2026-04-29; **PENDING round-1 dual external audit** per CLAUDE.md "Audit Standard"; Elon-mode `feedback_no_fake_menus` policy: round cap 2). Wave 6 #2 PRE-CO1.8 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms; recorded in `feedback_no_fake_menus` precedent). User auto-execute mode authorization 2026-04-29.

**Author**: ArchitectAI (Claude); session 2026-04-29.

**Companion specs (frozen, read first)**:
- `TRACE_MATRIX_v3_2026-04-27.md` — 324-line existing doc with N/M/D classification + § A-§ I structure. CO1.13 IMPLEMENTS this doc, doesn't replace it.
- `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.2.2 — most recent atom; precedent for spec format + Elon-mode tiered byte-identity (§ 2.2 v1.2.2 amendment).
- `docs/rules.md` + `rules/SCHEMA.yaml` — existing rule engine with 15 active rules; CO1.13 adds R-022 as 16th (within 30-rule cap).
- `CLAUDE.md` "Alignment Standard" — TRACE_MATRIX_v3 is the authority document referenced.

**Single sentence**: ship the 3-sub-atom factory bundle that closes the existing TRACE_MATRIX_v3 doc (currently § F empty + ~80 conformance test rows stub) + adds R-022 commit-time hook (blocks pub symbols without TRACE_MATRIX backlink) + boots the reverse-map population workflow — leaving generic scaffold scripts + Trust Root rehash automation to non-spec devtools (no audit gate; supporting `scripts/` deliverables landed alongside this atom).

---

## § 0 Scope decision

### 0.1 Why this atom exists (Elon-mode ROI)

Current state (recon 2026-04-29):
- **TRACE_MATRIX_v3 backlink coverage**: 87 backlinks / 354 pub items = **24.6%** in `src/`. Means 75% of constitutional alignment is implicit / drift-prone.
- **R-015 (existing)**: pre-edit *warn*; `times_triggered: 0` in YAML stats but enforcement.log shows actual triggers — stats not propagating. Warn-only ≠ block-at-commit; doesn't actually prevent untraced merges.
- **R-022 (referenced but not landed)**: TRACE_MATRIX_v3 § I says "R-022 hook enforces at commit time" — yet `rules/active/R-022*.yaml` does not exist (5 active R-NNN files reference R-022 but none implement it).
- **Reverse-map § F**: explicitly empty per TRACE_MATRIX_v3 § I "until CO P1 lands". With CO1.7-extra closure 2026-04-29 (4a978f0), enough atoms have shipped to start populating.

CO1.13 lands these closures. Each subsequent atom (CO1.8 / CO1.9 / ... / CO1.14 / CO P2.0 / ... / CO P2.12 = ~150 atoms) saves 30-60 min/atom on alignment hygiene under R-022 enforcement = **~75-150 hr amortization** over remaining sprint.

### 0.2 What this atom inherits (frozen)

| Frozen by | Surface CO1.13 consumes |
|---|---|
| `TRACE_MATRIX_v3_2026-04-27.md` | § A-§ I structure (Constitution → Code; WP → Code; sub-atom → test; orphan justification taxonomy; ~80-test conformance target) |
| `rules/engine.py` (145 LoC) + `rules/SCHEMA.yaml` | YAML rule loader; `grep` / `grep_inverse` / `compound` check types; `block` / `warn` enforcement levels |
| `.claude/hooks/judge.sh` | pre-edit hook entry point (R-022 commit-time variant lands at `.git/hooks/pre-commit` or via Lefthook config — see § 1.2) |
| 15 active rules R-001..R-020 | conventions for YAML schema; R-015 specifically as the pre-edit warn variant (CO1.13 keeps R-015 active; R-022 adds defense in depth at commit time) |

### 0.3 What this atom delivers (3 sub-atoms per sprint graph line 129)

| Sub-atom | Deliverable | LoC est | Cycle time target |
|---|---|---|---|
| **CO1.13.1** | TRACE_MATRIX_v3 doc completion: § A complete N-rows; § B complete WP rows; § E coverage stats; § F reverse-map populated for all shipped atoms (CO1.0a / CO1.4 / CO1.4-extra / CO1.7 / CO1.7-impl A1-A4 / CO1.7-extra) — **document-side closure of the v3 doc** | ~150 LoC docs delta | 0.5 day |
| **CO1.13.2** | R-022 commit-time hook: `rules/active/R-022_trace_matrix_pub_symbol_block.yaml` (block-enforcement variant of R-015) + `scripts/check_trace_matrix.py` (multi-line context grep tool the YAML check delegates to) + git pre-commit hook installation | ~120 LoC (script) + ~30 LoC (yaml) + ~15 LoC (pre-commit shim) = ~165 LoC | 1 day |
| **CO1.13.3** | reverse-map § F population workflow: `scripts/update_trace_matrix_reverse_map.sh` (idempotent re-population from src/* doc-comments); CI hook calls it; first-run populates from current src/* HEAD | ~100 LoC | 0.5 day |

**Total**: ~415 LoC; **2-day target wall-clock** (Elon-mode benchmark — first real test of cycle time hypothesis 14d → 2d).

### 0.4 Out of scope (devtools — landed alongside, no spec gate)

These are NOT in the audited spec scope but ship in the same git working tree (separate commits):
1. `scripts/scaffold_co_spec.sh` — generate spec template from atom-id + fc-anchor (saves ~30 min/atom; not constitutional)
2. `scripts/scaffold_audit_launcher.sh` — generate codex+gemini round-N launcher pair (saves ~20 min/round; not constitutional)
3. `scripts/rehash_trust_root.sh` — auto-rehash Trust Root manifest for changed src/* files (saves ~10 min/atom; runs cargo test boot::verify_trust_root post-rehash)

These are pure devtools; no constitutional surface; no PASS/PASS gate required. They land as a single follow-up commit "CO1.13-devtools" after CO1.13 PASS/PASS.

### 0.5 What this atom does NOT do

1. **Does NOT replace R-015**: R-015 (pre-edit warn) remains active; R-022 (commit-time block) is defense-in-depth. R-015 stats-not-propagating is a separate bug filed as `OBS_R_015_STATS_TIMES_TRIGGERED_DRIFT` follow-up.
2. **Does NOT enforce backlinks on legacy pub symbols** (the existing 75% gap): R-022 is a forward-only enforcer (blocks NEW untraced pub symbols). Legacy gap closure is a separate cleanup arc (CO1.13-extra; targets ~250 missing backlinks; ~10-15 hr work).
3. **Does NOT modify TRACE_MATRIX_v3 normative content** (the § A Constitution row mappings + § B WP row mappings): only fills in stub fields + populates § F reverse-map. Constitutional changes require sudo per Art V.3.

---

## § 1 Module structure

### 1.1 CO1.13.1 — TRACE_MATRIX_v3 doc completion

Direct edits to `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`:
- § A Constitution rows: verify each Article has Code symbol + Conformance test + Plan v3.2 atom columns populated (or flagged D with reason)
- § B WP rows: same coverage check for WP architecture (21 §) + economic (8 §) + RSP appendix
- § E Coverage stats: actual measured counts for shipped atoms (currently rough estimate; rerun after CO1.13.3 ships the measurement script)
- § F Reverse-map: NEW section listing every shipped src/*.rs pub symbol → which TRACE_MATRIX row maps it

### 1.2 CO1.13.2 — R-022 commit-time hook

Three pieces:

```
rules/active/R-022_trace_matrix_pub_symbol_block.yaml
scripts/check_trace_matrix.py         # multi-line context grep
.git/hooks/pre-commit                  # shim that calls check_trace_matrix.py
```

YAML rule (delegates to script via new `check.type: external_script` extension):
```yaml
id: "R-022"
name: "trace_matrix_pub_symbol_block"
source_incidents:
  - "F-2026-04-25-04"  # B7 alignment retroactive fix
  - "feedback_fc_first_problem_handling"  # FC-trace required in commit msg
fc_trace: "CLAUDE.md Alignment Standard — every NEW src/ pub symbol must have TRACE_MATRIX backlink AT COMMIT TIME (not retroactive)"
axiom: "every NEW pub fn/struct/enum/trait/const/mod added under src/ in this commit must have a /// TRACE_MATRIX <id>: <role> doc-comment within 5 lines preceding the pub line, OR be filed in TRACE_MATRIX_v3.md § 3 (orphan extensions) with explicit constitutional justification"
trigger: "pre_commit"
check:
  type: "external_script"
  script: "scripts/check_trace_matrix.py"
  args: ["--mode", "commit", "--enforce", "block"]
file_glob: "*.rs"
enforcement: "block"
message: "BLOCK (R-022 / Alignment Standard): NEW pub symbol(s) added under src/ without TRACE_MATRIX backlink. See script output for specific locations. Either (a) add /// TRACE_MATRIX <FC-id>: <role> doc-comment within 5 lines preceding each new pub symbol, (b) file in handover/alignment/TRACE_MATRIX_v3.md § 3 with orphan justification, or (c) explicit `// R-022-skip: <reason>` on the same commit (audited at quarterly review)."
stats:
  times_triggered: 0
  last_triggered: ""
```

`scripts/check_trace_matrix.py` (~120 LoC; uses git diff to identify NEW pub items vs base; for each, walks 5 lines preceding to verify backlink; falls back to TRACE_MATRIX_v3.md § 3 lookup; exits 2 on any unjustified addition).

Pre-commit shim (~15 LoC) — installs in `.git/hooks/pre-commit`; reads staged diff; pipes to engine.py with `--rule R-022`.

### 1.3 CO1.13.3 — Reverse-map § F population

`scripts/update_trace_matrix_reverse_map.sh` walks `src/*.rs`, extracts every `/// TRACE_MATRIX <id>: <role>` doc-comment + the immediately-following pub line, formats as `| <pub_symbol> | <id> | <role> |`, writes to TRACE_MATRIX_v3.md § F (idempotent — replaces section content). First run populates from current HEAD; subsequent runs (e.g., post-CO1.8 land) refresh.

Optional: CI cron job runs it weekly + opens PR if section content drifts. Out of v1 scope; manual run is fine for now.

---

## § 2 Implementation contract

### 2.1 R-022 enforcement boundary

R-022 fires on `pre_commit` when `git diff --cached` shows NEW `pub fn|struct|enum|trait|const|mod` lines under `src/`. For each new pub line, `check_trace_matrix.py`:
1. Reads 5 lines preceding the pub line in the same file
2. Greps for `/// TRACE_MATRIX `
3. If found: PASS for this symbol
4. If not found: greps `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` § 3 for the symbol path; if found in orphan list with justification, PASS
5. If not found in either: BLOCK

### 2.2 R-022 escape hatch

Per spec § 1.2 message: `// R-022-skip: <reason>` comment on the same commit allows bypass. Audited at quarterly review (manual). Use cases: experimental atoms; refactor cleanup that lands backlinks in a follow-up commit (within 1 week deadline).

### 2.3 Invariants (audited at sub-atom level)

| Invariant | Statement | Test |
|---|---|---|
| **I-FORWARD** | R-022 triggers on NEW pub symbols only; legacy pub symbols (already shipped pre-CO1.13.2) are exempt | `tests/r_022_no_legacy_block.rs` |
| **I-DOC** | TRACE_MATRIX_v3 § F reverse-map is auto-generated; no manual edits to § F (manual edits are overwritten on next run) | `scripts/update_trace_matrix_reverse_map.sh --dry-run` produces stable output |
| **I-LIST** | Active rules count ≤ 30 (docs/rules.md cap); CO1.13.2 lands R-022 as 16th rule (within cap) | `ls rules/active/*.yaml \| wc -l` |
| **I-ENFORCE** | R-022 enforcement actually blocks; cargo test demonstrates a test commit with missing backlink fails pre-commit | `tests/r_022_blocks_missing_backlink.rs` (uses `git -c hooks.pre-commit=enabled commit --dry-run`) |

---

## § 3 Test plan (substrate-independent + integration)

5 tests:

### 3.1 `tests/r_022_blocks_missing_backlink.rs`
Stages a fake new pub symbol without backlink; runs `scripts/check_trace_matrix.py --mode commit`; asserts exit 2.

### 3.2 `tests/r_022_no_legacy_block.rs`
Verifies that already-shipped pub symbols (without backlink) do NOT trigger R-022 on subsequent commits that don't modify them.

### 3.3 `tests/r_022_orphan_justification_passes.rs`
Stages a new pub symbol; adds entry to TRACE_MATRIX_v3 § 3 with `cases/Cxxx` justification; asserts script PASS.

### 3.4 `tests/trace_matrix_reverse_map_idempotent.rs`
Runs `scripts/update_trace_matrix_reverse_map.sh` twice; verifies § F content byte-identical between runs.

### 3.5 `tests/trace_matrix_v3_doc_coverage.rs`
Reads TRACE_MATRIX_v3.md § A + § B; asserts every Constitution Article + every WP § has at least one Class column populated (no all-empty rows).

---

## § 4 Out of scope (deferred per Anti-Oreo three-layer boundary)

1. **Legacy backlink gap closure** (the ~250 untraced legacy pub symbols): a separate CO1.13-extra atom; ~10-15 hr; ships as bulk doc-comment patch.
2. **Reverse-map CI cron**: manual run is sufficient for v1; CI integration is a CO1.13-extra concern.
3. **R-015 stats-not-propagating bug**: filed as separate OBS; orthogonal to R-022.
4. **80-conformance-test population**: TRACE_MATRIX_v3 § H lists ~80 target test files; populating is per-atom work (each Plan v3.2 atom ships its conformance test). CO1.13 only verifies the LIST is complete, not the tests themselves.
5. **Generic scaffold scripts** (§ 0.4): non-constitutional devtools; ship in follow-up commit, no audit.

---

## § 5 Open questions (audit-resolved)

| Q | Statement | Author lean |
|---|---|---|
| Q1 | Should R-022 fire on edits to EXISTING pub symbols (e.g., signature change) or only NEW ones? | NEW only (`I-FORWARD`). Edits are out of scope to keep enforcement tractable; existing R-015 (warn) covers edits. |
| Q2 | Is the 5-line preceding context window correct for backlink detection, or should it be flexible (e.g., entire doc-comment block above)? | 5 lines is a heuristic; works for ~95% of conventions in current src/. Edge case: multi-paragraph doc-comments where TRACE_MATRIX line is >5 lines above. Mitigation: convention enforced via R-022 message — keep TRACE_MATRIX line within 5 lines OR escape hatch. |
| Q3 | Should `// R-022-skip: <reason>` escape hatch be more rigorous (e.g., require `cases/Cxxx` reference) to prevent abuse? | v1 ships permissive; quarterly audit catches abuse. Tightening to require `cases/Cxxx` is a CO1.13-extra concern. |
| Q4 | Should reverse-map § F be the source-of-truth for backlinks, or are doc-comments authoritative? | Doc-comments authoritative (single source). § F is auto-derived view. This matches R-022 enforcement (blocks at doc-comment level). |
| Q5 | Should this atom also retire R-015 (downgrade to deprecated)? | NO. R-015 (pre-edit warn) catches issues earlier; R-022 (commit-time block) is the hard gate. Defense in depth. |

---

## § 6 Audit gates (Elon-mode round cap = 2)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 (this v1) | ⏳ pending | ⏳ pending | TBD | round-1 dual external audit on CO1.13 v1 |
| 2 (if r1 = CHALLENGE) | ⏳ | ⏳ | TBD | 1 round of patches → r2 final |
| 3+ | **CAPPED** | **CAPPED** | — | If r2 still CHALLENGE → ship as PASS-with-OBS_*.md per Elon-mode policy |

**Pre-implementation gate**: spec must reach PASS/PASS (or PASS-with-OBS) before any code in `rules/active/R-022*` / `scripts/check_trace_matrix.py` / `scripts/update_trace_matrix_reverse_map.sh` / `.git/hooks/pre-commit` is written. Per CLAUDE.md "Audit Standard". No STEP_B-restricted files touched (kernel.rs / bus.rs / wallet.rs UNTOUCHED).

---

## § 7 Estimated scope

- **Spec rounds**: round-1 expected CHALLENGE/CHALLENGE (5 OQs absorb both audits); round-2 PASS-or-CHALLENGE; cap at 2. Round budget ~$5-10.
- **Implementation scope** (post-PASS/PASS or PASS-with-OBS):
  - CO1.13.1: ~150 LoC docs delta in TRACE_MATRIX_v3.md (no Rust code)
  - CO1.13.2: ~165 LoC across YAML + Python + shell hook
  - CO1.13.3: ~100 LoC bash script
- **Total atom budget**: ~415 LoC; **target wall-clock 2 day** (Elon-mode hypothesis test).
- **Cumulative project audit spend after CO1.13 PASS/PASS**: ~$210-330 / $890 mid-budget.

---

## § 8 Honest acknowledgements

1. **Scope correction from Elon-mode framing**: the user's "factory tooling" framing in conversation suggested broader scope (scaffold scripts + Trust Root rehash); the canonical CO1.13 sprint-graph scope is narrower (3 sub-atoms: TRACE_MATRIX impl + R-022 + reverse-map). v1 honors the narrower scope; broader devtools land separately as non-spec follow-up commits per § 0.4.
2. **R-015 retention**: R-015 (existing pre-edit warn) is NOT retired by this atom. R-022 adds defense-in-depth at commit time; R-015 catches issues earlier in the editing flow. Both coexist within the 30-rule cap.
3. **Forward-only enforcement**: R-022 blocks NEW untraced pub symbols only. The ~250 legacy untraced symbols (pre-CO1.13.2) are handled in a separate CO1.13-extra atom. v1 does NOT close the legacy gap.
4. **Escape hatch permissive in v1**: `// R-022-skip: <reason>` allows bypass without `cases/Cxxx` reference. Quarterly audit catches abuse. Tightening to require justification is a CO1.13-extra concern.
5. **Test coverage**: 5 tests cover R-022 enforcement boundary + idempotency + doc coverage. Does NOT test the script's robustness against edge cases (e.g., pub symbol inside a comment block — should be ignored). v1 ships best-effort regex; refinement is out of v1 scope.
6. **No STEP_B-restricted file touches**: kernel + bus + wallet untouched. Pure-additive at `rules/active/`, `scripts/`, `.git/hooks/`, `tests/`. No STEP_B parallel-branch ceremony required.
7. **FC-trace requirements for CO1.13 implementation**: `scripts/check_trace_matrix.py` + `scripts/update_trace_matrix_reverse_map.sh` are tooling, not src/ pub symbols; they don't need TRACE_MATRIX backlinks. The R-022 YAML rule itself is documented via `fc_trace:` field (already a YAML schema convention).
8. **Elon-mode round cap is a NEW project policy** (audit cap @ 2 rounds; ship-with-OBS if not PASS/PASS by round-2). This v1 spec is the FIRST application; itself a real-test of the policy. Drift review at phase end will measure: did the cap actually fire? Did ship-with-OBS happen? What was the cycle time?

---

## § 9 Pre-audit smoke test plan

Per memory `feedback_smoke_before_batch`. Smoke run before round-1 audit launch.

| # | Claim | Smoke command | Pass criterion |
|---|---|---|---|
| S1 | TRACE_MATRIX_v3 doc exists | `wc -l handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` | 324 lines |
| S2 | Rule engine + 15 active rules | `ls rules/active/*.yaml \| wc -l` | 15 (R-022 not yet present) |
| S3 | R-022 absent | `ls rules/active/R-022*.yaml 2>&1` | "No such file or directory" |
| S4 | docs/rules.md describes mechanics | `grep -c 'judge.sh\|engine.py' docs/rules.md` | ≥2 |
| S5 | judge.sh hook exists | `ls .claude/hooks/judge.sh` | exists |
| S6 | 30-rule cap not exceeded | `ls rules/active/*.yaml \| wc -l` | ≤30 |
| S7 | TRACE_MATRIX backlink coverage baseline | `grep -rln 'TRACE_MATRIX' src/ \| wc -l` then `grep -rn 'pub fn\|pub struct\|pub enum\|pub trait\|pub const' src/ \| wc -l` | ratio reported (currently 22/42 files; 87/354 pub items = 24.6%) |
| S8 | TRACE_MATRIX_v3 § F status | `grep -c '## § F' handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` | 1 (section exists, body empty) |
| S9 | engine.py loadable | `python3 rules/engine.py --help 2>&1 \| head -3` | help text or empty (no error) |
| S10 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239/0/1 (matches HEAD `6cc5cc9`) |

---

**END v1 DRAFT body.**

## Pre-audit smoke results

### Round-1 smoke (HEAD `6cc5cc9`; v1)

| # | Claim | Result | Status |
|---|---|---|---|
| S1 | TRACE_MATRIX_v3 doc line count | 324 lines | ✅ PASS |
| S2 | active rules count | 15 (within 30-rule cap) | ✅ PASS |
| S3 | R-022 absent | "No such file or directory" | ✅ PASS (greenfield confirmed) |
| S4 | docs/rules.md describes mechanics | 4 mentions of judge.sh/engine.py | ✅ PASS |
| S5 | judge.sh exists | `.claude/hooks/judge.sh` 12899 bytes (executable) | ✅ PASS |
| S6 | 30-rule cap | 15/30 (R-022 lands as 16th = within cap) | ✅ PASS |
| S7 | backlink coverage baseline | files w/ TRACE_MATRIX: 22/42 (52%); pub items: 354; approx backlinked: 87 (24.6%) — confirms ~75% gap | ✅ PASS (gap quantified) |
| S8 | § F section exists, body empty | 1 occurrence of "## § F"; intro line "This section is populated incrementally as code lands (currently empty for v4 since CO P1 has not started)" | ✅ PASS |
| S9 | engine.py loadable | help text printed cleanly | ✅ PASS |
| S10 | cargo baseline | check clean (1 pre-existing gix_capability_spike warning); test 239/0/1 ignored | ✅ PASS |

**Smoke gate v1**: 10/10 PASS at HEAD `6cc5cc9`. Spec v1 ready for round-1 dual external audit.

## Patch log

**v1 (2026-04-29; greenfield draft, post-Elon-mode reframing)** — initial spec draft from primary sources:
- TRACE_MATRIX_v3_2026-04-27.md § A-§ I (existing 324-line doc)
- docs/rules.md + rules/SCHEMA.yaml + rules/active/R-015* (existing rule engine + R-015 precedent)
- SPRINT_DEPENDENCY_GRAPH_v1 line 129 ("CO1.13 TRACE_MATRIX_v3 implementation (3 atoms incl R-022 hook)")
- Elon-mode constraint (round cap = 2; ship-with-OBS allowed if not PASS/PASS by r2)
- Recon snapshot: 87 backlinks / 354 pub items in src/ = 24.6% coverage; R-022 referenced but not landed; reverse-map § F empty.

3 sub-atoms (CO1.13.1 doc completion + CO1.13.2 R-022 hook + CO1.13.3 reverse-map population). 5 substrate-independent tests. 5 open questions for round-1 audit (Q1 R-022 forward-only vs edit-also being most consequential).

### Awaiting

1. ⏳ pre-audit smoke run at v1 commit HEAD (S1-S10 from § 9)
2. ⏳ round-1 dual external audit (Codex + Gemini; Elon-mode round cap = 2)
3. ⏳ if CHALLENGE → 1 round of patches → r2 final; if still CHALLENGE → ship as PASS-with-OBS_R022_<topic>.md per Elon-mode
4. ⏳ implementation start (target 2-day wall-clock per Elon-mode hypothesis)
5. ⏳ phase drift review at impl complete (7-dimension check)
6. ⏳ Phase C smoke regression check at phase end (5/5 cells expected)


---

# XREF: TRACE_MATRIX_v3 (existing doc; CO1.13.1 closes stub fields)

# TRACE_MATRIX_v3 — Bidirectional Mapping with N/M/D Classification

> **Date**: 2026-04-27
> **Purpose**: D-VETO-5 final form; Codex CO P0.7 §2 demanded full coverage beyond seed; Gemini v3.2 Q1 PASS pending complete trace.
> **Authority**: Constitution + WP architecture (21 §) + WP economic (8 §, numbered 0/2/7/15/18/19/20/21) + RSP appendix.
> **Classification**:
> - **[N]ormative** = MUST map to ≥1 code symbol AND ≥1 conformance test
> - **[M]otivational** = explanatory text; no code mapping required
> - **[D]eferred** = out-of-v4 scope; lists target version + reason
>
> **Scope rule**: every WP § + every Constitution Article = one row in this matrix. If a § contains multiple normative claims, sub-rows allowed.

---

## § A — Constitution → Code Symbol Map

| Article | Class | Code symbol | Conformance test | Plan v3.2 atom |
|---|---|---|---|---|
| Art 0 — 图灵机原教旨 | N | `bottom_white::tape::chain_tape::ChainTape` + `tools::wtool::*` + `wal::Wal` + `top_white::predicates::registry::PredicateRegistry` (4-element mapping) | `tests/turing_fundamentalism.rs` | CO1.0 / CO1.5 / CO1.6 / CO1.7 |
| Art 0.1 — 四要素映射 | N | (same as Art 0) + `state::q_state::QState` for tape/control mapping | `tests/four_element_mapping.rs` | CO1.2 |
| Art 0.2 — Tape Canonical 公理 | N | `bottom_white::tape::tape_canonical_check::*` + 24 V-violation tests | `tests/tape_canonical_V01..V24.rs` (24 tests) | CO1.5-1.9 |
| Art 0.2 item 5 — failure-on-tape interpretation | N | `bottom_white::ledger::retry_metadata::{RejectedAttemptSummary, TerminalSummaryTx}` (Reading Y per Art 0.2 reinterpretation) | `tests/l6_reconstructibility.rs` + `tests/failure_histogram_reconstruct.rs` | CO1.7.0 + CO1.9.5 |
| Art 0.3 — 区块链化保留 | D (Path A semantic) / N (Path B git substrate) | `bottom_white::tape::git_substrate::*` (Path B chosen) | `tests/git_substrate_runtime_repo.rs` | CO1.3 |
| Art 0.4 — Q_t version-controlled | N | `state::q_state::QState` + `bottom_white::tape::git_substrate::on_cell_start` | `tests/q_state_reconstruct.rs` | CO1.2 + CO1.3 |
| Laws (基本法 1: Coin 守恒) | N | `economy::escrow_vault::*` + `economy::settlement_engine::*` | `tests/economic_invariant_INV3_escrow_only.rs` | CO P2.2 + CO P2.6 |
| Laws (基本法 2: founder grant) | N | `economy::escrow_vault::founder_grant_at_task_create` | `tests/economic_audit_E04_founder_grant_law2.rs` | CO P2.10 |
| Art I — 信号的量化 (top-level) | N | `top_white::signals::{boolean,statistical}` | `tests/signal_dichotomy.rs` | CO1.10 |
| Art I.1 — 布尔信号 | N | `top_white::signals::boolean::*` + `top_white::predicates::runner::run_acceptance` | `tests/boolean_signal_pass_fail.rs` | CO1.10 + CO1.5 |
| Art I.1.1 — PCP 谓词疑罪从无 | N | `top_white::predicates::registry::SafetyOrCreation` enum | `tests/safety_creation_dichotomy.rs` | CO1.11 |
| Art I.2 — 统计信号 | N | `top_white::signals::statistical::*` + `bottom_white::signal_index::stat_index` + `economy::reputation_index::*` + PPUT report | `tests/statistical_signals_complete.rs` | CO1.10 + CO1.9 |
| Art I.2 — PPUT/H-VPPUT/CI 报告强制项 | N | `experiments/.../bin/evaluator.rs::emit_summary` | `tests/report_standard_pput_ci_required.rs` | (existing; preserve through CO1.1.5 split) |
| Art II — 选择性广播 (top-level) | N | `top_white::signals::price_broadcast::emit` + L6 indices | `tests/broadcast_emits_to_l6.rs` | CO1.9 |
| Art II.1 — 广播典型错误 | N | `bottom_white::signal_index::failure_histogram` (system-derived, NOT agent self-report) | `tests/failure_histogram_reconstruct.rs` | CO1.9.5 |
| Art II.2 — 广播价格信号 | N | `top_white::signals::price_broadcast::emit_price` | `tests/price_broadcast_l6.rs` | CO1.9 |
| Art II.2.1 — 探索/利用 + parent_selection_entropy + payload_diversity | N | `experiments/.../bin/evaluator.rs::compute_entropy_and_diversity` | `tests/entropy_diversity_thresholds.rs` (per CLAUDE.md alert at < 0.25) | (existing; preserve) |
| Art III — 选择性屏蔽 (top-level) | N | `top_white::predicates::visibility::*` + `bottom_white::materializer::agent_view` | `tests/visibility_filter.rs` | CO1.5 + CO1.8 |
| Art III.1 — 屏蔽错误 | N | `top_white::predicates::visibility::Visibility::Private` for error contents | `tests/private_predicate_error_no_leak.rs` | CO1.5.7 |
| Art III.2 — 封装细节 | N | `bottom_white::materializer::agent_view::project_for_agent` | `tests/agent_view_filters_internals.rs` | CO1.8.6 |
| Art III.3 — 屏蔽相关性 | N | `economy::price_index::aggregation_filter` (top-K only; no fine-grain) | `tests/price_aggregation_correlation_shield.rs` | CO P2.1 (TaskMarket price publish) |
| Art III.4 — 屏蔽 Goodhart | N | `top_white::predicates::visibility::Visibility::{Public,Private,CommitReveal}` | `tests/goodhart_shield.rs` + `tests/economic_invariant_INV10_signal_vs_evaluator.rs` | CO1.5.2 + CO1.5.7 |
| Art IV — Boot (Bootstrap 公理) | N | `boot::verify_trust_root` + `boot::verify_constitution_root` (NEW per genesis spec) + `state::q_state::QState::genesis` | `tests/boot_genesis_minimal_with_anchor.rs` + 5 new genesis tests | CO1.0 |
| Art IV — terminal categorization (halt_reason 5种) | N | `experiments/.../bin/evaluator.rs::HaltReason` enum + summary | `tests/halt_reason_distribution.rs` | (existing; preserve) |
| Art V — Go Meta (top-level) | N (offline path) / D-v4.1 (runtime path) | `governance::meta_validator::validate_meta_proposal` (offline) / runtime ArchitectAI deferred to v4.1 | `tests/meta_validator_correctness.rs` (CO P3-prep) + v4.1 runtime tests | CO P3-PREP 1-7 |
| Art V.1.1 — Constitution 唯一基准 | N | `genesis_payload::constitution_root::constitution_hash` + `boot::verify_constitution_root` | `tests/genesis_constitution_root_verify.rs` | CO1.0.4 |
| Art V.1.2 — ArchitectAI 提出者 | N (offline v4) | `governance::amendment_predicate::evaluate` + cp workflow | `tests/architect_proposal_offline.rs` | CO P3-prep.4 |
| Art V.1.3 — Veto-AI 验证者 | N | dual external audit (Codex + Gemini) per `TRI_MODEL_ORCHESTRATION_PROTOCOL` | `tests/dual_audit_protocol_existence.rs` (meta-test) | (existing) |
| Art V.2 — 宪法界限与示例 | M | (no code mapping; explanatory) | n/a | n/a |
| Art V.3 — 宪法修订日志 | N | `handover/architect-insights/RATIFICATION_*.md` chain + signed git tags | `tests/ratification_chain_verifies.rs` | (governance gate; existing per B-1) |

---

## § B — WP Architecture → Code Symbol Map

| § | Title | Class | Code symbol | Conformance test | Plan atom |
|---|---|---|---|---|---|
| Abstract | (TuringOS = …) | M | n/a | n/a | n/a |
| § 0 设计公理 | 6 axioms | N (bridge to Const Art 0.5 + 6 公理) | `state::q_state::QState` (axiom 1) + `top_white::predicates::*` (axiom 2) + `economy::*` (axiom 3) etc. | `tests/six_axioms_alignment.rs` | CO0.8 + CO1.* |
| § 1 问题 | why agents crash | M | n/a | n/a | n/a |
| § 2 图灵机隐喻 | paper/pencil/rubber | N (mirrors Const Art 0) | (same as Const Art 0) | (same) | CO1.0/1.6/1.7 |
| § 3 反奥利奥三层 | top/middle/bottom white | N | `src/{top_white,middle_black,bottom_white,economy}/*` directory structure | `tests/anti_oreo_layer_audit.rs` | CO1.1.* |
| § 4 系统状态 Q_t | 8 components | N | `state::q_state::QState` (9 fields incl economic_state_t) | `tests/q_state_reconstruct.rs` + `tests/economic_state_reconstruct.rs` | CO1.2 |
| § 5.L0 Constitution Root | hash + sig + sudo + amendment_rules + attestation | N | `genesis_payload::constitution_root::*` (8 fields per `GENESIS_MINIMAL_WITH_ANCHOR_v1`) | `tests/genesis_constitution_root_*.rs` (5) | CO1.0.* |
| § 5.L1 Predicate Registry | id + version + code_hash + schema + visibility + owner + test_suite | N | `top_white::predicates::registry::PredicateRegistry` | `tests/chain_tape_L1_predicate_registry.rs` | CO1.5 |
| § 5.L2 Tool Registry | id + capability + permission + determinism + side_effect | N | `bottom_white::tools::registry::ToolRegistry` | `tests/chain_tape_L2_tool_registry.rs` | CO1.6 |
| § 5.L3 CAS | cid + hash + type + creator + visibility | N | `bottom_white::cas::store::*` | `tests/chain_tape_L3_cas.rs` | CO1.4 |
| § 5.L4 Transition Ledger | 12 fields | N | `bottom_white::ledger::transition::TransitionTx` (12 fields incl task_id) | `tests/chain_tape_L4_transition_ledger.rs` + `tests/transition_tx_12_fields.rs` | CO1.7 |
| § 5.L5 Materialized State + Agent View | indices + permission_view | N | `bottom_white::materializer::{state_db, indices, agent_view}` | `tests/chain_tape_L5_materialized_state.rs` | CO1.8 |
| § 5.L6 Signal Indices | boolean + price + reputation + scarcity + explore/exploit | N | `bottom_white::signal_index::*` + `top_white::signals::*` | `tests/chain_tape_L6_signal_indices.rs` | CO1.9 |
| § 6 状态转移协议 | step_transition 7 stages | N | `transition::step_transition` + verify/challenge/reuse/finalize per `STATE_TRANSITION_SPEC_v1` | 20 invariants → 20 tests `tests/transition_*.rs` | CO1.SPEC.0 + CO1.7.5 |
| § 7 信号的量化 | boolean vs statistical dichotomy | N | (same as Const Art I) | `tests/signal_dichotomy.rs` | CO1.10 |
| § 7.2 安全 vs 创造 fail-policy | safety fail-closed; creation fail-open-with-signal | N | `top_white::predicates::registry::SafetyOrCreation` | `tests/safety_creation_dichotomy.rs` | CO1.11 |
| § 8 选择性广播 | broadcast price + boolean signal aggregates | N | `top_white::signals::price_broadcast::*` | `tests/price_broadcast_l6.rs` | CO1.9 + CO1.10 |
| § 9.1 屏蔽错误 (per Codex demand) | error hiding | N | `top_white::predicates::visibility::Visibility::Private` error filter | `tests/private_predicate_error_no_leak.rs` | CO1.5.7 |
| § 9.2 最小上下文 | minimal agent context window | N | `bottom_white::materializer::agent_view::project_for_agent` (visibility-filtered) | `tests/agent_view_minimal_context.rs` | CO1.8.6 + CO1.8.7 |
| § 9.3 屏蔽相关性 | correlation shielding | N | `economy::price_index::aggregation_filter` | `tests/price_aggregation_correlation_shield.rs` | CO P2.1 |
| § 9.4 Goodhart 屏蔽 (public/private/commit-reveal) | three visibility classes | N | `top_white::predicates::visibility::Visibility` enum | `tests/goodhart_shield.rs` + `tests/economic_invariant_INV10_signal_vs_evaluator.rs` | CO1.5.2 |
| § 10 Laws of Money | monetary discipline → economic chapter elaborates | N | (links to economic chapter Inv 1-12) | (12 INV tests) | CO P2.* |
| § 11 Boot — 创世状态 | genesis block fields | N | `genesis_payload::*` (8 fields per GENESIS_MINIMAL_WITH_ANCHOR_v1) | `tests/genesis_*.rs` (5) | CO1.0 |
| § 12 Go Meta | meta_tx semantics | N (offline) / D-v4.1 (runtime) | `META_TX_SCHEMA_v1` typed schema + `governance::meta_validator::*` (offline); runtime ArchitectAI/JudgeAI deferred to v4.1 | `tests/meta_tx_schema_serialization.rs` + `tests/meta_validator_*.rs` | CO P3-PREP.1, .3, .5, .6 |
| § 12.2 meta_tx schema | parent_root + patches + evidence + reversibility + check + sigs + human_sig | N (schema) / D-v4.1 (L4 acceptance) | `META_TX_SCHEMA_v1` § 2 typed schema | `tests/meta_tx_schema_serialization.rs` | CO P3-prep.1 |
| § 13 区块链位置 | local→permissioned→rollup→public | partial: N (local hashchain/git → v4); D-v4.1+ (Hyperledger / rollup / public) | `bottom_white::tape::git_substrate` (local Path B); permissioned/rollup deferred | `tests/git_substrate_*.rs` | CO1.3 |
| § 14 数据结构示例 | illustrative TOML/Rust snippets | M | n/a | n/a | n/a |
| § 15 MVP | minimum viable phase | N | (links to § 17 Phase 1+2; v4 scope) | (per phase exit gates) | CO P0/P1/P2 exits |
| § 16 安全边界与失败模式 | threat model, failure classes | N | `SYSTEM_KEYPAIR_SECURITY_v1` § 2 threat model + `top_white::predicates::*` failure classification | `tests/system_keypair_*.rs` (5) + per-failure-class tests | CO1.7.0a + CO1.5 |
| § 17 实施路线 5-Phase | Phase 1+2 (v4) + Phase 3 prep + Phase 4-5 deferred | N (v4 Phase 1+2 + Phase 3 prep) / D-v4.1+ (Phase 4-5) | Plan v3.2 atoms CO P0+P1+P2 + CO P3-PREP track | `tests/phase_1_2_complete.rs` (synthetic) | CO P0-P2 + CO P3-PREP |
| § 18 结论 | summary | M | n/a | n/a | n/a |
| RSP § 1-16 (appendix) | RSP details, mostly redundant with economic chapter | N (redundant; map via economic chapter rows) | (see § C below) | (see § C) | CO P2 |

---

## § C — WP Economic → Code Symbol Map

| § | Title | Class | Code symbol | Conformance test | Plan atom |
|---|---|---|---|---|---|
| § 0 核心校准 | "经济不是发币" negative invariant | N | `economy::*` (no `mint_post_init` API surface; Inv 4 + cargo-deny) + negative test | `tests/economic_audit_E03_naming.rs` (no token-issuance APIs) + `tests/no_post_init_mint.rs` | CO P2.0 + CO P2.10 |
| § 2 Q_t 扩展 | economic_state_t 9 sub-fields | N | `state::q_state::EconomicState` 9 sub-fields | `tests/economic_state_reconstruct.rs` | CO1.2.2 |
| § 7 Agent 5 经济角色 | Solver/Verifier/Challenger/Builder/ArchitectAI/JudgeAI (6 roles, "5 + Judge meta" interpretation) | N | `experiments/.../agents/{solver,verifier,challenger,builder,architect_ai,judge_ai}.rs` (6 files) | `tests/agent_role_economic.rs` (6 roles dispatch) | CO P2.7 |
| § 15 区块链技术定位 | local/permissioned/rollup/ZK/oracle | partial: N (local) / D-v4.1+ (rest) | (see arch § 13 row) | (same) | CO1.3 |
| § 18 12 Economic Invariants | Inv 1-12 | N (each invariant is its own conformance test) | `economy::invariants::inv01..inv12` | `tests/economic_invariant_INV1..12.rs` (12 tests) | CO P2.* |
| § 19 RSP-1 modules (9) | TaskMarket / EscrowVault / ContributionLedger / PredicateRunner / AttributionEngine / ChallengeCourt / SettlementEngine / ReputationIndex / PriceIndex | N | `economy::{task_market, escrow_vault, contribution_ledger, attribution_engine, challenge_court, settlement_engine, reputation_index, price_index}::*` (8 dirs; PredicateRunner lives in `top_white::predicates::runner`) | `tests/rsp1_modules_smoke.rs` + per-module tests | CO P2.1-2.9 |
| § 20 5-Phase 部署 | Phase 1 (Local Ledger) / Phase 2 (Internal Task Market) / Phase 3-5 deferred | N (v4 Phase 1+2) / D-v4.x (Phase 3-5) | (Plan v3.2 atoms) | (per phase gates) | CO P0-P2 |
| § 21 最终公式 | reward_i = Finalize(Escrow × Accept × Attribution × Survival × Utility × Constitution) | N | `economy::settlement_engine::finalize_reward` (per `STATE_TRANSITION_SPEC_v1` § 3.4) | `tests/final_reward_formula.rs` | CO P2.6.4 |
| (cross-ref to architecture) | mapping table | M | n/a | n/a | n/a |

---

## § D — RSP Appendix (architecture WP § 1050-1066) → Code Symbol Map

The RSP appendix in architecture WP largely overlaps the economic chapter. Cross-references:

| Appendix § | Architecture WP line | Economic chapter equivalent | Class |
|---|---|---|---|
| RSP § 1-3 (intro) | line 1050-1066 | § 0-19 | M (intro) |
| RSP § 4-8 (mechanisms) | line 1067+ (in WP) | § 21 final formula | N (mapped via econ § 21) |
| RSP § 9-12 (economic state, escrow, settlement) | line 1100+ (in WP) | § 19 RSP-1 modules | N (mapped via econ § 19) |
| RSP § 13-16 (governance, monetary base, signals) | line 1180+ | § 18 invariants + § 21 formula | N (mapped via econ § 18/21) |

**Note**: per Codex CO P0.7 §2 row (RSP appendix), economic chapter § 19 lists 9 modules but architecture appendix lists 8. Discrepancy resolved: PriceIndex is the 9th module in economic chapter; architecture appendix groups PriceIndex under Signal Indices L6 (still mapped, just split across two layers in architecture WP). Both are normative; both implemented.

---

## § E — Coverage Statistics

| Source | Total rows | [N] | [M] | [D] |
|---|---|---|---|---|
| Constitution Articles + sub-articles | 27 | 24 | 1 (Art V.2) | 2 (Art 0.3 partial Path A; Art 0.5 future) |
| WP architecture §§ | 21 (incl 0/1/2 plus subsections 5.L0-L6, 7.2, 9.1-4, 12.2, 17 phases) | 17 (full) + 4 (partial / phase-conditional) | 4 (Abstract, § 1, § 14, § 18) | embedded in partial rows |
| WP economic §§ | 8 (numbered 0/2/7/15/18/19/20/21) | 7 | 1 (cross-ref table) | embedded in partial rows |
| RSP appendix | 4 sub-§ | 3 | 1 | — |

**Total Normative coverage**: ~51 rows. Each Normative row has at least 1 conformance test path (existing or planned in Plan v3.2 atoms).

**Test count from this matrix**: ~60-70 distinct conformance tests (some rows share tests; e.g., Goodhart shield).

**Forbidden state**: any Normative row with empty "code symbol" or empty "conformance test" column. Pre-commit hook R-022 (added per Plan v3.2 CO P0.8) enforces.

---

## § F — Bidirectional Reverse: Code Symbol → Source

This section is populated incrementally as code lands (currently empty for v4 since CO P1 has not started). Format:

```
src/path/to/symbol.rs::function_name
  ↓
  TRACE_MATRIX_v3 row: <Constitution Art X | WP arch § Y | WP econ § Z>
```

This reverse map is auto-generated by `scripts/check_trace_matrix_updated.sh` per Plan v3.2 CO1.13.2 atom. Pre-commit hook R-022 enforces "every `pub` symbol in src/{top_white,middle_black,bottom_white,economy,state,transition,governance}/*.rs MUST have a `/// TRACE_MATRIX <id>: <role>` doc-comment". Build fails if missing.

**Initial state at v4 ratification (2026-04-27)**: section is **empty by design** — code does not yet exist. v4 will populate it commit-by-commit during CO P1+P2.

---

## § G — Deferred Items Justification

Items classified [D]eferred MUST list target version + reason. Audit gate: every [D] tag is reviewable; no opaque "later".

| Item | Target version | Reason |
|---|---|---|
| Constitution Art 0.3 Path A semantic version | NEVER (Path B chosen instead) | Art 0.4 commit selected Path B (real git substrate); Path A description in Art 0.3 marked obsolete by Art 0.4 caveat (line 110) |
| Constitution Art 0.5 (white paper integration) | CO P0 enactment (post-ratification cp ceremony) | DRAFT exists; awaits user cp + signed tag |
| WP architecture § 13 permissioned/rollup phases | v4.x or v5 | per WP § 17 explicit roadmap |
| WP architecture § 17 Phase 4-5 (public chaincode/rollup) | v5 | scope decision; WP says "post-v4" |
| WP architecture § 12 runtime ArchitectAI/JudgeAI | v4.1 | D-VETO-4 ratified resolution; v4 ships Phase 3 prep (CO P3-PREP.1-7) |
| WP economic § 15 ZK/Validity Proof predicates | v4.x or v5 | requires substantive cryptographic infrastructure beyond v4 Path B |
| WP economic § 15 Oracle integration | v4.x | external fact input substrate; v4 is closed-system |

---

## § H — Conformance Test Master List (output for cargo test wiring)

Tests required to claim 100% Normative coverage (organized by domain):

```
# Anti-Oreo + Q_t + Tape Canonical (CO1.1, CO1.2, CO1.5-1.9)
tests/anti_oreo_layer_audit.rs
tests/q_state_reconstruct.rs
tests/economic_state_reconstruct.rs
tests/four_element_mapping.rs
tests/turing_fundamentalism.rs
tests/tape_canonical_V01..V24.rs                    (24 tests)

# ChainTape layers (CO1.0-1.9)
tests/chain_tape_L0_constitution_root.rs
tests/chain_tape_L1_predicate_registry.rs
tests/chain_tape_L2_tool_registry.rs
tests/chain_tape_L3_cas.rs
tests/chain_tape_L4_transition_ledger.rs
tests/chain_tape_L5_materialized_state.rs
tests/chain_tape_L6_signal_indices.rs

# State transition spec invariants I-1 through I-20 (CO1.SPEC.0)
tests/transition_determinism.rs                    (I-DET)
tests/no_hidden_inputs.rs                          (I-NOSIDE)
tests/stale_parent_rejection.rs                    (I-PARENT)
tests/signature_verification.rs                    (I-SIG)
tests/stake_atomicity.rs                           (I-STAKE)
tests/no_wall_clock_in_tx.rs                       (I-LOGTIME)
tests/no_f64_money.rs                              (I-MICROCOIN)
tests/q_state_uses_btree.rs                        (I-BTREE)
tests/no_rejection_sidecar.rs                      (I-NOSIDECAR)
tests/retry_summary_runner_signed.rs               (I-RETRY)
tests/run_terminal_invariant.rs                    (I-TERMINAL)
tests/no_env_in_transition.rs                      (I-NOENV)
tests/task_config_frozen_at_publish.rs             (I-FREEZE-CONFIG)
tests/no_runtime_entropy.rs                        (I-NORANDOM)
tests/verify_target_liveness.rs                    (I-VERIFY-LIVE)
tests/challenge_window_enforced.rs                 (I-CHAL-WINDOW)
tests/finalize_or_slash_exclusive.rs               (I-FINALIZE-EXCLUSIVE)

# Genesis (CO1.0)
tests/genesis_constitution_root_verify.rs
tests/genesis_amendment_predicate_resolves.rs
tests/genesis_initial_registry_empty.rs
tests/genesis_boot_attestation_self_referential.rs
tests/genesis_creator_signature_verifies.rs

# Predicates + Visibility (CO1.5, CO1.11)
tests/safety_creation_dichotomy.rs
tests/private_predicate_error_no_leak.rs
tests/agent_view_filters_internals.rs
tests/agent_view_minimal_context.rs
tests/goodhart_shield.rs

# Signals (CO1.9, CO1.10)
tests/signal_dichotomy.rs
tests/boolean_signal_pass_fail.rs
tests/statistical_signals_complete.rs
tests/price_broadcast_l6.rs
tests/price_aggregation_correlation_shield.rs

# Reports (CLAUDE.md Report Standard)
tests/report_standard_pput_ci_required.rs
tests/halt_reason_distribution.rs
tests/entropy_diversity_thresholds.rs

# Economic invariants (CO P2.*)
tests/economic_invariant_INV1_no_thinking_reward.rs
tests/economic_invariant_INV2_no_direct_collect.rs
tests/economic_invariant_INV3_escrow_only.rs
tests/economic_invariant_INV4_no_post_mint.rs
tests/economic_invariant_INV5_yes_no_event_bound.rs
tests/economic_invariant_INV6_predicate_gated.rs
tests/economic_invariant_INV7_provisional_then_final.rs
tests/economic_invariant_INV8_dag_attribution.rs
tests/economic_invariant_INV9_reputation_immutable.rs
tests/economic_invariant_INV10_signal_vs_evaluator.rs
tests/economic_invariant_INV11_chain_record_only.rs
tests/economic_invariant_INV12_consensus_not_truth.rs

# Economic audit (CO P2.10)
tests/economic_audit_E01_production_default_on.rs
tests/economic_audit_E02_jsonl_summary.rs
tests/economic_audit_E03_naming.rs
tests/economic_audit_E04_founder_grant_law2.rs
tests/no_post_init_mint.rs

# RSP modules + final formula (CO P2.*)
tests/rsp1_modules_smoke.rs
tests/agent_role_economic.rs
tests/final_reward_formula.rs
tests/ctf_stake_symmetry.rs
tests/attribution_engine_determinism.rs

# Retry metadata (CO1.7.0, CO1.9.5)
tests/l6_reconstructibility.rs
tests/failure_histogram_reconstruct.rs

# System keypair (CO1.7.0a-f)
tests/system_keypair_generation.rs
tests/system_keypair_load_and_decrypt.rs
tests/system_keypair_sign_only_from_runner.rs
tests/system_keypair_verify_correctness.rs
tests/system_keypair_rotation_proof.rs

# MetaTx schema (CO P3-prep)
tests/meta_tx_schema_serialization.rs
tests/meta_validator_pass_cases.rs
tests/meta_validator_veto_cases.rs
tests/meta_validator_correctness.rs
tests/amendment_flow_format_validate.rs

# Substrate (CO1.3)
tests/git_substrate_runtime_repo.rs

# Trace matrix self-conformance (CO1.13)
tests/trace_matrix_v3_bidirectional.rs
tests/six_axioms_alignment.rs

# Governance (B-1)
tests/ratification_chain_verifies.rs
tests/dual_audit_protocol_existence.rs

# Cross-domain
tests/architect_proposal_offline.rs
tests/transition_tx_12_fields.rs
tests/anti_oreo_layer_audit.rs
tests/safety_creation_dichotomy.rs (already listed)
```

**Total target test count**: ~80 distinct test files. Some are stubs at v4 ratification (test exists, tests `unimplemented!()`); each will be implemented at the corresponding atom. v4 ship gate: 100% non-stubbed.

---

## § I — Honest Acknowledgements

What this matrix achieves:
- Closes Codex CO P0.7 §2 demand for full Normative coverage
- Closes Gemini v3.2 Q1 PASS qualifier ("every § mapped" claim now actually verifiable)
- Provides ~80-test target for v4 ship + bidirectional code↔doc traceability

What this matrix is honest about:
- §B/§C "Code symbol" column references modules that DON'T YET EXIST in v4 (the matrix anchors future code, which is OK per DO-178C; the test column gives the verification target)
- §F reverse map is empty until CO P1 lands
- Some [N] rows currently fail conformance because corresponding code doesn't exist (this is BY DESIGN — tests are the spec)
- Coverage statistics in §E count rows, not invariants; some [N] rows share invariants

What this matrix does NOT do:
- Generate the conformance tests automatically (each test is a Plan v3.2 atom CO P1.* / CO P2.*)
- Validate that tests actually catch the violation they claim (Codex/Gemini per-atom audits handle that)
- Replace the per-atom doc-comment `/// TRACE_MATRIX <id>: <role>` in each `pub` symbol (R-022 hook enforces at commit time)

— ArchitectAI, 2026-04-27


---

# XREF: docs/rules.md (rule engine architecture)

# TuringOS v4 Rule Engine

## Architecture
```
CLAUDE.md instructions (~70% compliance)
  + hooks/judge.sh (closes gap to ~100%)
    + rules/engine.py (evaluates YAML rules)
      + rules/active/*.yaml (dynamic, add/remove = add/remove file)
```

## How It Works
1. Claude edits a file → `judge.sh` receives JSON on stdin
2. `judge.sh` calls `rules/engine.py` with file path + content
3. Engine loads all YAML rules, filters by `file_glob`
4. For each matching rule: runs `check.pattern` regex against content
5. Block rule matches → exit 2 (edit rejected)
6. Warn rule matches → exit 0 + log to enforcement.log + trace

## Rule Schema
See `rules/SCHEMA.yaml` for full spec.

## Active Rules

### Block Level (exit 2 — hard enforcement)
| ID | Name | Axiom |
|----|------|-------|
| R-001 | kernel_purity | Law 1: zero domain knowledge |
| R-002 | no_coin_minting | Law 2: no post-genesis printing |
| R-003 | no_wal_deletion | Tape append-only |
| R-004 | lean_syntax_in_prompts | Rule 22: black-box |
| R-005 | forced_investment | Law 2: voluntary staking |

### Warn Level (exit 0 — advisory + log)
| ID | Name | Axiom |
|----|------|-------|
| R-006 | kernel_modification | Law 1 |
| R-007 | bus_lifecycle | Engine separation |
| R-008 | market_constants | Law 2 |
| R-009 | payload_limits | Rule 21 |
| R-013 | format_contract | Bitter Lesson (V-009) |

## Adding a Rule
1. Create `rules/active/R-xxx_name.yaml` following SCHEMA
2. Done. Engine picks it up automatically.
Hard cap: 30 rules maximum.

## Traces
Rule triggers are logged to `traces/sessions/{date}.jsonl` for analysis.
Use `/harness-reflect` to review rule effectiveness.


---

# XREF: rules/SCHEMA.yaml

```yaml
# TuringOS v4 Rule Schema
# Each YAML file in rules/active/ must follow this schema.

# Required fields:
#   id: string          — Unique rule identifier (e.g. R-001)
#   name: string        — Human-readable name
#   axiom: string       — Constitutional basis (e.g. "Law 1", "Rule 22")
#   file_glob: string   — File pattern to match (e.g. "kernel.rs", "*.rs")
#   check:
#     type: string      — "grep" | "grep_inverse" | "compound"
#     pattern: string   — Regex pattern to search for
#   enforcement: string — "block" (exit 2) | "warn" (exit 0 + log)
#   message: string     — Human-readable violation message

# Optional fields:
#   source_incidents: list  — Incident IDs that motivated this rule
#   stats:
#     triggers: int         — Total trigger count (updated by engine)
#     last_triggered: string — ISO timestamp

```


---

# XREF: rules/active/R-015_trace_matrix_pub_symbol.yaml (existing pre-edit warn)

```yaml
id: "R-015"
name: "trace_matrix_pub_symbol_warn"
source_incidents:
  - "F-2026-04-25-04"  # B7 alignment retroactive fix — pub symbols shipped without TRACE_MATRIX backlink
fc_trace: "CLAUDE.md Alignment Standard — every src/ pub symbol must map to FC1/FC2/FC3 element"
axiom: "Every pub fn / struct / enum / trait / const in src/ must carry /// TRACE_MATRIX FCx-Nx: <role> doc-comment OR be filed as orphan with explicit cases/Cxxx or PREREG-§n.m justification in TRACE_MATRIX_v?.md"
trigger: "pre_edit"
check:
  type: "grep"
  pattern: "pub (fn|struct|enum|trait|const|mod) "
file_glob: "*.rs"
enforcement: "warn"
message: "REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift."
stats:
  times_triggered: 0
  last_triggered: ""

```


---

# XREF: rules/engine.py (rule evaluator; ~145 LoC)

```python
#!/usr/bin/env python3
"""TuringOS v4 Rule Engine — Pure Python predicate evaluator.

Reads content from stdin, checks against YAML rules in --rules-dir,
outputs warnings/blocks. Exit 0 = pass, exit 2 = block.

Called by: .claude/hooks/judge.sh
"""
import argparse
import fnmatch
import json
import os
import re
import sys
from datetime import datetime, timezone


def load_yaml_simple(path: str) -> dict:
    """Minimal YAML parser for flat rule files. No PyYAML dependency."""
    data = {}
    current_key = None
    with open(path) as f:
        for line in f:
            line = line.rstrip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("  ") and current_key:
                # Nested value (for check.type, check.pattern, stats.*)
                kv = line.strip().split(":", 1)
                if len(kv) == 2:
                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
                    if current_key not in data or not isinstance(data[current_key], dict):
                        data[current_key] = {}
                    data[current_key][k] = v
            else:
                kv = line.split(":", 1)
                if len(kv) == 2:
                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
                    current_key = k
                    data[k] = v
    return data


def check_rule(rule: dict, content: str) -> bool:
    """Returns True if the rule triggers (violation detected)."""
    check = rule.get("check", {})
    if not isinstance(check, dict):
        return False

    check_type = check.get("type", "grep")
    pattern = check.get("pattern", "")

    if not pattern:
        return False

    if check_type == "grep":
        return bool(re.search(pattern, content, re.IGNORECASE))
    elif check_type == "grep_inverse":
        return not bool(re.search(pattern, content, re.IGNORECASE))
    elif check_type == "compound":
        # All sub-patterns must match
        parts = [p.strip() for p in pattern.split("&&")]
        return all(re.search(p, content, re.IGNORECASE) for p in parts)
    return False


def write_trace(traces_dir: str, rule_id: str, file_path: str, message: str, verdict: str):
    """Append a trace entry as JSONL."""
    if not traces_dir:
        return
    os.makedirs(traces_dir, exist_ok=True)
    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    trace_file = os.path.join(traces_dir, f"{today}.jsonl")
    entry = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "event": verdict,
        "rule": rule_id,
        "file": file_path,
        "message": message,
    }
    with open(trace_file, "a") as f:
        f.write(json.dumps(entry) + "\n")


def write_log(log_path: str, rule_id: str, enforcement: str, file_path: str, message: str):
    """Append to enforcement.log."""
    if not log_path:
        return
    ts = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S")
    with open(log_path, "a") as f:
        f.write(f"[{ts}] {enforcement.upper()} {rule_id} | {file_path} | {message}\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", required=True)
    parser.add_argument("--rules-dir", required=True)
    parser.add_argument("--log", default="")
    parser.add_argument("--traces-dir", default="")
    args = parser.parse_args()

    content = sys.stdin.read()
    if not content:
        sys.exit(0)

    file_path = args.file
    blocked = False

    # Load and evaluate all matching rules
    if not os.path.isdir(args.rules_dir):
        sys.exit(0)

    for fname in sorted(os.listdir(args.rules_dir)):
        if not fname.endswith((".yaml", ".yml")):
            continue
        rule_path = os.path.join(args.rules_dir, fname)
        rule = load_yaml_simple(rule_path)

        # Check file_glob match
        file_glob = rule.get("file_glob", "*")
        # Normalize: match against basename and full path
        basename = os.path.basename(file_path)
        if not (fnmatch.fnmatch(basename, file_glob) or fnmatch.fnmatch(file_path, f"*{file_glob}")):
            continue

        if check_rule(rule, content):
            rule_id = rule.get("id", fname)
            enforcement = rule.get("enforcement", "warn")
            message = rule.get("message", f"Rule {rule_id} triggered")

            if enforcement == "block":
                print(f"BLOCKED by {rule_id}: {message}")
                write_log(args.log, rule_id, "block", file_path, message)
                write_trace(args.traces_dir, rule_id, file_path, message, "block")
                blocked = True
            else:
                print(f"WARNING {rule_id}: {message}")
                write_log(args.log, rule_id, "warn", file_path, message)
                write_trace(args.traces_dir, rule_id, file_path, message, "warn")

    sys.exit(2 if blocked else 0)


if __name__ == "__main__":
    main()

```


---

Now give your INDEPENDENT round-1 audit. Cite spec § + line where possible.

codex
I’ll verify the repo-local guidance and the spec’s actual line numbers before issuing the audit. I’m treating this as a review only, so I won’t modify files.
exec
/bin/bash -lc "pwd && rg --files -g 'CLAUDE.md' -g 'handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md' -g 'handover/alignment/TRACE_MATRIX_v3_2026-04-27.md' -g 'rules/engine.py' -g 'rules/SCHEMA.yaml' -g 'docs/rules.md' -g 'rules/active/R-015_trace_matrix_pub_symbol.yaml' -g '.claude/hooks/judge.sh'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4
handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md
handover/alignment/TRACE_MATRIX_v3_2026-04-27.md
docs/rules.md
rules/active/R-015_trace_matrix_pub_symbol.yaml
rules/engine.py
rules/SCHEMA.yaml
CLAUDE.md

exec
/bin/bash -lc 'find .. -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 1586ms:
../turingos/AGENTS.md
../omega_kepler_discovery/AGENTS.md
../omega/AGENTS.md
../turingosv2/AGENTS.md
../turingclaw/AGENTS.md

codex
I don’t see a repository `AGENTS.md` under this checkout; the user-provided AGENTS scope is the active guidance for this review. Next I’m pulling the spec sections with line numbers and checking the rule engine/schema surfaces that CO1.13 proposes to extend.
exec
/bin/bash -lc "nl -ba handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 1ms:
     1	# CO1.13: TRACE_MATRIX_v3 Implementation + R-022 Hook v1 ⏳ pre-audit (round-1 pending)
     2	
     3	**Status**: v1 (2026-04-29; **PENDING round-1 dual external audit** per CLAUDE.md "Audit Standard"; Elon-mode `feedback_no_fake_menus` policy: round cap 2). Wave 6 #2 PRE-CO1.8 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms; recorded in `feedback_no_fake_menus` precedent). User auto-execute mode authorization 2026-04-29.
     4	
     5	**Author**: ArchitectAI (Claude); session 2026-04-29.
     6	
     7	**Companion specs (frozen, read first)**:
     8	- `TRACE_MATRIX_v3_2026-04-27.md` — 324-line existing doc with N/M/D classification + § A-§ I structure. CO1.13 IMPLEMENTS this doc, doesn't replace it.
     9	- `CO1_7_EXTRA_HEAD_T_WIRING_v1_2026-04-29.md` v1.2.2 — most recent atom; precedent for spec format + Elon-mode tiered byte-identity (§ 2.2 v1.2.2 amendment).
    10	- `docs/rules.md` + `rules/SCHEMA.yaml` — existing rule engine with 15 active rules; CO1.13 adds R-022 as 16th (within 30-rule cap).
    11	- `CLAUDE.md` "Alignment Standard" — TRACE_MATRIX_v3 is the authority document referenced.
    12	
    13	**Single sentence**: ship the 3-sub-atom factory bundle that closes the existing TRACE_MATRIX_v3 doc (currently § F empty + ~80 conformance test rows stub) + adds R-022 commit-time hook (blocks pub symbols without TRACE_MATRIX backlink) + boots the reverse-map population workflow — leaving generic scaffold scripts + Trust Root rehash automation to non-spec devtools (no audit gate; supporting `scripts/` deliverables landed alongside this atom).
    14	
    15	---
    16	
    17	## § 0 Scope decision
    18	
    19	### 0.1 Why this atom exists (Elon-mode ROI)
    20	
    21	Current state (recon 2026-04-29):
    22	- **TRACE_MATRIX_v3 backlink coverage**: 87 backlinks / 354 pub items = **24.6%** in `src/`. Means 75% of constitutional alignment is implicit / drift-prone.
    23	- **R-015 (existing)**: pre-edit *warn*; `times_triggered: 0` in YAML stats but enforcement.log shows actual triggers — stats not propagating. Warn-only ≠ block-at-commit; doesn't actually prevent untraced merges.
    24	- **R-022 (referenced but not landed)**: TRACE_MATRIX_v3 § I says "R-022 hook enforces at commit time" — yet `rules/active/R-022*.yaml` does not exist (5 active R-NNN files reference R-022 but none implement it).
    25	- **Reverse-map § F**: explicitly empty per TRACE_MATRIX_v3 § I "until CO P1 lands". With CO1.7-extra closure 2026-04-29 (4a978f0), enough atoms have shipped to start populating.
    26	
    27	CO1.13 lands these closures. Each subsequent atom (CO1.8 / CO1.9 / ... / CO1.14 / CO P2.0 / ... / CO P2.12 = ~150 atoms) saves 30-60 min/atom on alignment hygiene under R-022 enforcement = **~75-150 hr amortization** over remaining sprint.
    28	
    29	### 0.2 What this atom inherits (frozen)
    30	
    31	| Frozen by | Surface CO1.13 consumes |
    32	|---|---|
    33	| `TRACE_MATRIX_v3_2026-04-27.md` | § A-§ I structure (Constitution → Code; WP → Code; sub-atom → test; orphan justification taxonomy; ~80-test conformance target) |
    34	| `rules/engine.py` (145 LoC) + `rules/SCHEMA.yaml` | YAML rule loader; `grep` / `grep_inverse` / `compound` check types; `block` / `warn` enforcement levels |
    35	| `.claude/hooks/judge.sh` | pre-edit hook entry point (R-022 commit-time variant lands at `.git/hooks/pre-commit` or via Lefthook config — see § 1.2) |
    36	| 15 active rules R-001..R-020 | conventions for YAML schema; R-015 specifically as the pre-edit warn variant (CO1.13 keeps R-015 active; R-022 adds defense in depth at commit time) |
    37	
    38	### 0.3 What this atom delivers (3 sub-atoms per sprint graph line 129)
    39	
    40	| Sub-atom | Deliverable | LoC est | Cycle time target |
    41	|---|---|---|---|
    42	| **CO1.13.1** | TRACE_MATRIX_v3 doc completion: § A complete N-rows; § B complete WP rows; § E coverage stats; § F reverse-map populated for all shipped atoms (CO1.0a / CO1.4 / CO1.4-extra / CO1.7 / CO1.7-impl A1-A4 / CO1.7-extra) — **document-side closure of the v3 doc** | ~150 LoC docs delta | 0.5 day |
    43	| **CO1.13.2** | R-022 commit-time hook: `rules/active/R-022_trace_matrix_pub_symbol_block.yaml` (block-enforcement variant of R-015) + `scripts/check_trace_matrix.py` (multi-line context grep tool the YAML check delegates to) + git pre-commit hook installation | ~120 LoC (script) + ~30 LoC (yaml) + ~15 LoC (pre-commit shim) = ~165 LoC | 1 day |
    44	| **CO1.13.3** | reverse-map § F population workflow: `scripts/update_trace_matrix_reverse_map.sh` (idempotent re-population from src/* doc-comments); CI hook calls it; first-run populates from current src/* HEAD | ~100 LoC | 0.5 day |
    45	
    46	**Total**: ~415 LoC; **2-day target wall-clock** (Elon-mode benchmark — first real test of cycle time hypothesis 14d → 2d).
    47	
    48	### 0.4 Out of scope (devtools — landed alongside, no spec gate)
    49	
    50	These are NOT in the audited spec scope but ship in the same git working tree (separate commits):
    51	1. `scripts/scaffold_co_spec.sh` — generate spec template from atom-id + fc-anchor (saves ~30 min/atom; not constitutional)
    52	2. `scripts/scaffold_audit_launcher.sh` — generate codex+gemini round-N launcher pair (saves ~20 min/round; not constitutional)
    53	3. `scripts/rehash_trust_root.sh` — auto-rehash Trust Root manifest for changed src/* files (saves ~10 min/atom; runs cargo test boot::verify_trust_root post-rehash)
    54	
    55	These are pure devtools; no constitutional surface; no PASS/PASS gate required. They land as a single follow-up commit "CO1.13-devtools" after CO1.13 PASS/PASS.
    56	
    57	### 0.5 What this atom does NOT do
    58	
    59	1. **Does NOT replace R-015**: R-015 (pre-edit warn) remains active; R-022 (commit-time block) is defense-in-depth. R-015 stats-not-propagating is a separate bug filed as `OBS_R_015_STATS_TIMES_TRIGGERED_DRIFT` follow-up.
    60	2. **Does NOT enforce backlinks on legacy pub symbols** (the existing 75% gap): R-022 is a forward-only enforcer (blocks NEW untraced pub symbols). Legacy gap closure is a separate cleanup arc (CO1.13-extra; targets ~250 missing backlinks; ~10-15 hr work).
    61	3. **Does NOT modify TRACE_MATRIX_v3 normative content** (the § A Constitution row mappings + § B WP row mappings): only fills in stub fields + populates § F reverse-map. Constitutional changes require sudo per Art V.3.
    62	
    63	---
    64	
    65	## § 1 Module structure
    66	
    67	### 1.1 CO1.13.1 — TRACE_MATRIX_v3 doc completion
    68	
    69	Direct edits to `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md`:
    70	- § A Constitution rows: verify each Article has Code symbol + Conformance test + Plan v3.2 atom columns populated (or flagged D with reason)
    71	- § B WP rows: same coverage check for WP architecture (21 §) + economic (8 §) + RSP appendix
    72	- § E Coverage stats: actual measured counts for shipped atoms (currently rough estimate; rerun after CO1.13.3 ships the measurement script)
    73	- § F Reverse-map: NEW section listing every shipped src/*.rs pub symbol → which TRACE_MATRIX row maps it
    74	
    75	### 1.2 CO1.13.2 — R-022 commit-time hook
    76	
    77	Three pieces:
    78	
    79	```
    80	rules/active/R-022_trace_matrix_pub_symbol_block.yaml
    81	scripts/check_trace_matrix.py         # multi-line context grep
    82	.git/hooks/pre-commit                  # shim that calls check_trace_matrix.py
    83	```
    84	
    85	YAML rule (delegates to script via new `check.type: external_script` extension):
    86	```yaml
    87	id: "R-022"
    88	name: "trace_matrix_pub_symbol_block"
    89	source_incidents:
    90	  - "F-2026-04-25-04"  # B7 alignment retroactive fix
    91	  - "feedback_fc_first_problem_handling"  # FC-trace required in commit msg
    92	fc_trace: "CLAUDE.md Alignment Standard — every NEW src/ pub symbol must have TRACE_MATRIX backlink AT COMMIT TIME (not retroactive)"
    93	axiom: "every NEW pub fn/struct/enum/trait/const/mod added under src/ in this commit must have a /// TRACE_MATRIX <id>: <role> doc-comment within 5 lines preceding the pub line, OR be filed in TRACE_MATRIX_v3.md § 3 (orphan extensions) with explicit constitutional justification"
    94	trigger: "pre_commit"
    95	check:
    96	  type: "external_script"
    97	  script: "scripts/check_trace_matrix.py"
    98	  args: ["--mode", "commit", "--enforce", "block"]
    99	file_glob: "*.rs"
   100	enforcement: "block"
   101	message: "BLOCK (R-022 / Alignment Standard): NEW pub symbol(s) added under src/ without TRACE_MATRIX backlink. See script output for specific locations. Either (a) add /// TRACE_MATRIX <FC-id>: <role> doc-comment within 5 lines preceding each new pub symbol, (b) file in handover/alignment/TRACE_MATRIX_v3.md § 3 with orphan justification, or (c) explicit `// R-022-skip: <reason>` on the same commit (audited at quarterly review)."
   102	stats:
   103	  times_triggered: 0
   104	  last_triggered: ""
   105	```
   106	
   107	`scripts/check_trace_matrix.py` (~120 LoC; uses git diff to identify NEW pub items vs base; for each, walks 5 lines preceding to verify backlink; falls back to TRACE_MATRIX_v3.md § 3 lookup; exits 2 on any unjustified addition).
   108	
   109	Pre-commit shim (~15 LoC) — installs in `.git/hooks/pre-commit`; reads staged diff; pipes to engine.py with `--rule R-022`.
   110	
   111	### 1.3 CO1.13.3 — Reverse-map § F population
   112	
   113	`scripts/update_trace_matrix_reverse_map.sh` walks `src/*.rs`, extracts every `/// TRACE_MATRIX <id>: <role>` doc-comment + the immediately-following pub line, formats as `| <pub_symbol> | <id> | <role> |`, writes to TRACE_MATRIX_v3.md § F (idempotent — replaces section content). First run populates from current HEAD; subsequent runs (e.g., post-CO1.8 land) refresh.
   114	
   115	Optional: CI cron job runs it weekly + opens PR if section content drifts. Out of v1 scope; manual run is fine for now.
   116	
   117	---
   118	
   119	## § 2 Implementation contract
   120	
   121	### 2.1 R-022 enforcement boundary
   122	
   123	R-022 fires on `pre_commit` when `git diff --cached` shows NEW `pub fn|struct|enum|trait|const|mod` lines under `src/`. For each new pub line, `check_trace_matrix.py`:
   124	1. Reads 5 lines preceding the pub line in the same file
   125	2. Greps for `/// TRACE_MATRIX `
   126	3. If found: PASS for this symbol
   127	4. If not found: greps `handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` § 3 for the symbol path; if found in orphan list with justification, PASS
   128	5. If not found in either: BLOCK
   129	
   130	### 2.2 R-022 escape hatch
   131	
   132	Per spec § 1.2 message: `// R-022-skip: <reason>` comment on the same commit allows bypass. Audited at quarterly review (manual). Use cases: experimental atoms; refactor cleanup that lands backlinks in a follow-up commit (within 1 week deadline).
   133	
   134	### 2.3 Invariants (audited at sub-atom level)
   135	
   136	| Invariant | Statement | Test |
   137	|---|---|---|
   138	| **I-FORWARD** | R-022 triggers on NEW pub symbols only; legacy pub symbols (already shipped pre-CO1.13.2) are exempt | `tests/r_022_no_legacy_block.rs` |
   139	| **I-DOC** | TRACE_MATRIX_v3 § F reverse-map is auto-generated; no manual edits to § F (manual edits are overwritten on next run) | `scripts/update_trace_matrix_reverse_map.sh --dry-run` produces stable output |
   140	| **I-LIST** | Active rules count ≤ 30 (docs/rules.md cap); CO1.13.2 lands R-022 as 16th rule (within cap) | `ls rules/active/*.yaml \| wc -l` |
   141	| **I-ENFORCE** | R-022 enforcement actually blocks; cargo test demonstrates a test commit with missing backlink fails pre-commit | `tests/r_022_blocks_missing_backlink.rs` (uses `git -c hooks.pre-commit=enabled commit --dry-run`) |
   142	
   143	---
   144	
   145	## § 3 Test plan (substrate-independent + integration)
   146	
   147	5 tests:
   148	
   149	### 3.1 `tests/r_022_blocks_missing_backlink.rs`
   150	Stages a fake new pub symbol without backlink; runs `scripts/check_trace_matrix.py --mode commit`; asserts exit 2.
   151	
   152	### 3.2 `tests/r_022_no_legacy_block.rs`
   153	Verifies that already-shipped pub symbols (without backlink) do NOT trigger R-022 on subsequent commits that don't modify them.
   154	
   155	### 3.3 `tests/r_022_orphan_justification_passes.rs`
   156	Stages a new pub symbol; adds entry to TRACE_MATRIX_v3 § 3 with `cases/Cxxx` justification; asserts script PASS.
   157	
   158	### 3.4 `tests/trace_matrix_reverse_map_idempotent.rs`
   159	Runs `scripts/update_trace_matrix_reverse_map.sh` twice; verifies § F content byte-identical between runs.
   160	
   161	### 3.5 `tests/trace_matrix_v3_doc_coverage.rs`
   162	Reads TRACE_MATRIX_v3.md § A + § B; asserts every Constitution Article + every WP § has at least one Class column populated (no all-empty rows).
   163	
   164	---
   165	
   166	## § 4 Out of scope (deferred per Anti-Oreo three-layer boundary)
   167	
   168	1. **Legacy backlink gap closure** (the ~250 untraced legacy pub symbols): a separate CO1.13-extra atom; ~10-15 hr; ships as bulk doc-comment patch.
   169	2. **Reverse-map CI cron**: manual run is sufficient for v1; CI integration is a CO1.13-extra concern.
   170	3. **R-015 stats-not-propagating bug**: filed as separate OBS; orthogonal to R-022.
   171	4. **80-conformance-test population**: TRACE_MATRIX_v3 § H lists ~80 target test files; populating is per-atom work (each Plan v3.2 atom ships its conformance test). CO1.13 only verifies the LIST is complete, not the tests themselves.
   172	5. **Generic scaffold scripts** (§ 0.4): non-constitutional devtools; ship in follow-up commit, no audit.
   173	
   174	---
   175	
   176	## § 5 Open questions (audit-resolved)
   177	
   178	| Q | Statement | Author lean |
   179	|---|---|---|
   180	| Q1 | Should R-022 fire on edits to EXISTING pub symbols (e.g., signature change) or only NEW ones? | NEW only (`I-FORWARD`). Edits are out of scope to keep enforcement tractable; existing R-015 (warn) covers edits. |
   181	| Q2 | Is the 5-line preceding context window correct for backlink detection, or should it be flexible (e.g., entire doc-comment block above)? | 5 lines is a heuristic; works for ~95% of conventions in current src/. Edge case: multi-paragraph doc-comments where TRACE_MATRIX line is >5 lines above. Mitigation: convention enforced via R-022 message — keep TRACE_MATRIX line within 5 lines OR escape hatch. |
   182	| Q3 | Should `// R-022-skip: <reason>` escape hatch be more rigorous (e.g., require `cases/Cxxx` reference) to prevent abuse? | v1 ships permissive; quarterly audit catches abuse. Tightening to require `cases/Cxxx` is a CO1.13-extra concern. |
   183	| Q4 | Should reverse-map § F be the source-of-truth for backlinks, or are doc-comments authoritative? | Doc-comments authoritative (single source). § F is auto-derived view. This matches R-022 enforcement (blocks at doc-comment level). |
   184	| Q5 | Should this atom also retire R-015 (downgrade to deprecated)? | NO. R-015 (pre-edit warn) catches issues earlier; R-022 (commit-time block) is the hard gate. Defense in depth. |
   185	
   186	---
   187	
   188	## § 6 Audit gates (Elon-mode round cap = 2)
   189	
   190	| Round | Codex | Gemini | Conservative | Action |
   191	|---|---|---|---|---|
   192	| 1 (this v1) | ⏳ pending | ⏳ pending | TBD | round-1 dual external audit on CO1.13 v1 |
   193	| 2 (if r1 = CHALLENGE) | ⏳ | ⏳ | TBD | 1 round of patches → r2 final |
   194	| 3+ | **CAPPED** | **CAPPED** | — | If r2 still CHALLENGE → ship as PASS-with-OBS_*.md per Elon-mode policy |
   195	
   196	**Pre-implementation gate**: spec must reach PASS/PASS (or PASS-with-OBS) before any code in `rules/active/R-022*` / `scripts/check_trace_matrix.py` / `scripts/update_trace_matrix_reverse_map.sh` / `.git/hooks/pre-commit` is written. Per CLAUDE.md "Audit Standard". No STEP_B-restricted files touched (kernel.rs / bus.rs / wallet.rs UNTOUCHED).
   197	
   198	---
   199	
   200	## § 7 Estimated scope
   201	
   202	- **Spec rounds**: round-1 expected CHALLENGE/CHALLENGE (5 OQs absorb both audits); round-2 PASS-or-CHALLENGE; cap at 2. Round budget ~$5-10.
   203	- **Implementation scope** (post-PASS/PASS or PASS-with-OBS):
   204	  - CO1.13.1: ~150 LoC docs delta in TRACE_MATRIX_v3.md (no Rust code)
   205	  - CO1.13.2: ~165 LoC across YAML + Python + shell hook
   206	  - CO1.13.3: ~100 LoC bash script
   207	- **Total atom budget**: ~415 LoC; **target wall-clock 2 day** (Elon-mode hypothesis test).
   208	- **Cumulative project audit spend after CO1.13 PASS/PASS**: ~$210-330 / $890 mid-budget.
   209	
   210	---
   211	
   212	## § 8 Honest acknowledgements
   213	
   214	1. **Scope correction from Elon-mode framing**: the user's "factory tooling" framing in conversation suggested broader scope (scaffold scripts + Trust Root rehash); the canonical CO1.13 sprint-graph scope is narrower (3 sub-atoms: TRACE_MATRIX impl + R-022 + reverse-map). v1 honors the narrower scope; broader devtools land separately as non-spec follow-up commits per § 0.4.
   215	2. **R-015 retention**: R-015 (existing pre-edit warn) is NOT retired by this atom. R-022 adds defense-in-depth at commit time; R-015 catches issues earlier in the editing flow. Both coexist within the 30-rule cap.
   216	3. **Forward-only enforcement**: R-022 blocks NEW untraced pub symbols only. The ~250 legacy untraced symbols (pre-CO1.13.2) are handled in a separate CO1.13-extra atom. v1 does NOT close the legacy gap.
   217	4. **Escape hatch permissive in v1**: `// R-022-skip: <reason>` allows bypass without `cases/Cxxx` reference. Quarterly audit catches abuse. Tightening to require justification is a CO1.13-extra concern.
   218	5. **Test coverage**: 5 tests cover R-022 enforcement boundary + idempotency + doc coverage. Does NOT test the script's robustness against edge cases (e.g., pub symbol inside a comment block — should be ignored). v1 ships best-effort regex; refinement is out of v1 scope.
   219	6. **No STEP_B-restricted file touches**: kernel + bus + wallet untouched. Pure-additive at `rules/active/`, `scripts/`, `.git/hooks/`, `tests/`. No STEP_B parallel-branch ceremony required.
   220	7. **FC-trace requirements for CO1.13 implementation**: `scripts/check_trace_matrix.py` + `scripts/update_trace_matrix_reverse_map.sh` are tooling, not src/ pub symbols; they don't need TRACE_MATRIX backlinks. The R-022 YAML rule itself is documented via `fc_trace:` field (already a YAML schema convention).

exec
/bin/bash -lc "nl -ba handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md | sed -n '221,360p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   221	8. **Elon-mode round cap is a NEW project policy** (audit cap @ 2 rounds; ship-with-OBS if not PASS/PASS by round-2). This v1 spec is the FIRST application; itself a real-test of the policy. Drift review at phase end will measure: did the cap actually fire? Did ship-with-OBS happen? What was the cycle time?
   222	
   223	---
   224	
   225	## § 9 Pre-audit smoke test plan
   226	
   227	Per memory `feedback_smoke_before_batch`. Smoke run before round-1 audit launch.
   228	
   229	| # | Claim | Smoke command | Pass criterion |
   230	|---|---|---|---|
   231	| S1 | TRACE_MATRIX_v3 doc exists | `wc -l handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` | 324 lines |
   232	| S2 | Rule engine + 15 active rules | `ls rules/active/*.yaml \| wc -l` | 15 (R-022 not yet present) |
   233	| S3 | R-022 absent | `ls rules/active/R-022*.yaml 2>&1` | "No such file or directory" |
   234	| S4 | docs/rules.md describes mechanics | `grep -c 'judge.sh\|engine.py' docs/rules.md` | ≥2 |
   235	| S5 | judge.sh hook exists | `ls .claude/hooks/judge.sh` | exists |
   236	| S6 | 30-rule cap not exceeded | `ls rules/active/*.yaml \| wc -l` | ≤30 |
   237	| S7 | TRACE_MATRIX backlink coverage baseline | `grep -rln 'TRACE_MATRIX' src/ \| wc -l` then `grep -rn 'pub fn\|pub struct\|pub enum\|pub trait\|pub const' src/ \| wc -l` | ratio reported (currently 22/42 files; 87/354 pub items = 24.6%) |
   238	| S8 | TRACE_MATRIX_v3 § F status | `grep -c '## § F' handover/alignment/TRACE_MATRIX_v3_2026-04-27.md` | 1 (section exists, body empty) |
   239	| S9 | engine.py loadable | `python3 rules/engine.py --help 2>&1 \| head -3` | help text or empty (no error) |
   240	| S10 | cargo baseline | `cargo check --workspace && cargo test --workspace --lib` | clean compile + 239/0/1 (matches HEAD `6cc5cc9`) |
   241	
   242	---
   243	
   244	**END v1 DRAFT body.**
   245	
   246	## Pre-audit smoke results
   247	
   248	### Round-1 smoke (HEAD `6cc5cc9`; v1)
   249	
   250	| # | Claim | Result | Status |
   251	|---|---|---|---|
   252	| S1 | TRACE_MATRIX_v3 doc line count | 324 lines | ✅ PASS |
   253	| S2 | active rules count | 15 (within 30-rule cap) | ✅ PASS |
   254	| S3 | R-022 absent | "No such file or directory" | ✅ PASS (greenfield confirmed) |
   255	| S4 | docs/rules.md describes mechanics | 4 mentions of judge.sh/engine.py | ✅ PASS |
   256	| S5 | judge.sh exists | `.claude/hooks/judge.sh` 12899 bytes (executable) | ✅ PASS |
   257	| S6 | 30-rule cap | 15/30 (R-022 lands as 16th = within cap) | ✅ PASS |
   258	| S7 | backlink coverage baseline | files w/ TRACE_MATRIX: 22/42 (52%); pub items: 354; approx backlinked: 87 (24.6%) — confirms ~75% gap | ✅ PASS (gap quantified) |
   259	| S8 | § F section exists, body empty | 1 occurrence of "## § F"; intro line "This section is populated incrementally as code lands (currently empty for v4 since CO P1 has not started)" | ✅ PASS |
   260	| S9 | engine.py loadable | help text printed cleanly | ✅ PASS |
   261	| S10 | cargo baseline | check clean (1 pre-existing gix_capability_spike warning); test 239/0/1 ignored | ✅ PASS |
   262	
   263	**Smoke gate v1**: 10/10 PASS at HEAD `6cc5cc9`. Spec v1 ready for round-1 dual external audit.
   264	
   265	## Patch log
   266	
   267	**v1 (2026-04-29; greenfield draft, post-Elon-mode reframing)** — initial spec draft from primary sources:
   268	- TRACE_MATRIX_v3_2026-04-27.md § A-§ I (existing 324-line doc)
   269	- docs/rules.md + rules/SCHEMA.yaml + rules/active/R-015* (existing rule engine + R-015 precedent)
   270	- SPRINT_DEPENDENCY_GRAPH_v1 line 129 ("CO1.13 TRACE_MATRIX_v3 implementation (3 atoms incl R-022 hook)")
   271	- Elon-mode constraint (round cap = 2; ship-with-OBS allowed if not PASS/PASS by r2)
   272	- Recon snapshot: 87 backlinks / 354 pub items in src/ = 24.6% coverage; R-022 referenced but not landed; reverse-map § F empty.
   273	
   274	3 sub-atoms (CO1.13.1 doc completion + CO1.13.2 R-022 hook + CO1.13.3 reverse-map population). 5 substrate-independent tests. 5 open questions for round-1 audit (Q1 R-022 forward-only vs edit-also being most consequential).
   275	
   276	### Awaiting
   277	
   278	1. ⏳ pre-audit smoke run at v1 commit HEAD (S1-S10 from § 9)
   279	2. ⏳ round-1 dual external audit (Codex + Gemini; Elon-mode round cap = 2)
   280	3. ⏳ if CHALLENGE → 1 round of patches → r2 final; if still CHALLENGE → ship as PASS-with-OBS_R022_<topic>.md per Elon-mode
   281	4. ⏳ implementation start (target 2-day wall-clock per Elon-mode hypothesis)
   282	5. ⏳ phase drift review at impl complete (7-dimension check)
   283	6. ⏳ Phase C smoke regression check at phase end (5/5 cells expected)

exec
/bin/bash -lc "nl -ba rules/engine.py | sed -n '1,190p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env python3
     2	"""TuringOS v4 Rule Engine — Pure Python predicate evaluator.
     3	
     4	Reads content from stdin, checks against YAML rules in --rules-dir,
     5	outputs warnings/blocks. Exit 0 = pass, exit 2 = block.
     6	
     7	Called by: .claude/hooks/judge.sh
     8	"""
     9	import argparse
    10	import fnmatch
    11	import json
    12	import os
    13	import re
    14	import sys
    15	from datetime import datetime, timezone
    16	
    17	
    18	def load_yaml_simple(path: str) -> dict:
    19	    """Minimal YAML parser for flat rule files. No PyYAML dependency."""
    20	    data = {}
    21	    current_key = None
    22	    with open(path) as f:
    23	        for line in f:
    24	            line = line.rstrip()
    25	            if not line or line.startswith("#"):
    26	                continue
    27	            if line.startswith("  ") and current_key:
    28	                # Nested value (for check.type, check.pattern, stats.*)
    29	                kv = line.strip().split(":", 1)
    30	                if len(kv) == 2:
    31	                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
    32	                    if current_key not in data or not isinstance(data[current_key], dict):
    33	                        data[current_key] = {}
    34	                    data[current_key][k] = v
    35	            else:
    36	                kv = line.split(":", 1)
    37	                if len(kv) == 2:
    38	                    k, v = kv[0].strip(), kv[1].strip().strip('"').strip("'")
    39	                    current_key = k
    40	                    data[k] = v
    41	    return data
    42	
    43	
    44	def check_rule(rule: dict, content: str) -> bool:
    45	    """Returns True if the rule triggers (violation detected)."""
    46	    check = rule.get("check", {})
    47	    if not isinstance(check, dict):
    48	        return False
    49	
    50	    check_type = check.get("type", "grep")
    51	    pattern = check.get("pattern", "")
    52	
    53	    if not pattern:
    54	        return False
    55	
    56	    if check_type == "grep":
    57	        return bool(re.search(pattern, content, re.IGNORECASE))
    58	    elif check_type == "grep_inverse":
    59	        return not bool(re.search(pattern, content, re.IGNORECASE))
    60	    elif check_type == "compound":
    61	        # All sub-patterns must match
    62	        parts = [p.strip() for p in pattern.split("&&")]
    63	        return all(re.search(p, content, re.IGNORECASE) for p in parts)
    64	    return False
    65	
    66	
    67	def write_trace(traces_dir: str, rule_id: str, file_path: str, message: str, verdict: str):
    68	    """Append a trace entry as JSONL."""
    69	    if not traces_dir:
    70	        return
    71	    os.makedirs(traces_dir, exist_ok=True)
    72	    today = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    73	    trace_file = os.path.join(traces_dir, f"{today}.jsonl")
    74	    entry = {
    75	        "ts": datetime.now(timezone.utc).isoformat(),
    76	        "event": verdict,
    77	        "rule": rule_id,
    78	        "file": file_path,
    79	        "message": message,
    80	    }
    81	    with open(trace_file, "a") as f:
    82	        f.write(json.dumps(entry) + "\n")
    83	
    84	
    85	def write_log(log_path: str, rule_id: str, enforcement: str, file_path: str, message: str):
    86	    """Append to enforcement.log."""
    87	    if not log_path:
    88	        return
    89	    ts = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%S")
    90	    with open(log_path, "a") as f:
    91	        f.write(f"[{ts}] {enforcement.upper()} {rule_id} | {file_path} | {message}\n")
    92	
    93	
    94	def main():
    95	    parser = argparse.ArgumentParser()
    96	    parser.add_argument("--file", required=True)
    97	    parser.add_argument("--rules-dir", required=True)
    98	    parser.add_argument("--log", default="")
    99	    parser.add_argument("--traces-dir", default="")
   100	    args = parser.parse_args()
   101	
   102	    content = sys.stdin.read()
   103	    if not content:
   104	        sys.exit(0)
   105	
   106	    file_path = args.file
   107	    blocked = False
   108	
   109	    # Load and evaluate all matching rules
   110	    if not os.path.isdir(args.rules_dir):
   111	        sys.exit(0)
   112	
   113	    for fname in sorted(os.listdir(args.rules_dir)):
   114	        if not fname.endswith((".yaml", ".yml")):
   115	            continue
   116	        rule_path = os.path.join(args.rules_dir, fname)
   117	        rule = load_yaml_simple(rule_path)
   118	
   119	        # Check file_glob match
   120	        file_glob = rule.get("file_glob", "*")
   121	        # Normalize: match against basename and full path
   122	        basename = os.path.basename(file_path)
   123	        if not (fnmatch.fnmatch(basename, file_glob) or fnmatch.fnmatch(file_path, f"*{file_glob}")):
   124	            continue
   125	
   126	        if check_rule(rule, content):
   127	            rule_id = rule.get("id", fname)
   128	            enforcement = rule.get("enforcement", "warn")
   129	            message = rule.get("message", f"Rule {rule_id} triggered")
   130	
   131	            if enforcement == "block":
   132	                print(f"BLOCKED by {rule_id}: {message}")
   133	                write_log(args.log, rule_id, "block", file_path, message)
   134	                write_trace(args.traces_dir, rule_id, file_path, message, "block")
   135	                blocked = True
   136	            else:
   137	                print(f"WARNING {rule_id}: {message}")
   138	                write_log(args.log, rule_id, "warn", file_path, message)
   139	                write_trace(args.traces_dir, rule_id, file_path, message, "warn")
   140	
   141	    sys.exit(2 if blocked else 0)
   142	
   143	
   144	if __name__ == "__main__":
   145	    main()

exec
/bin/bash -lc "nl -ba rules/SCHEMA.yaml docs/rules.md rules/active/R-015_trace_matrix_pub_symbol.yaml | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 Rule Schema
     2	# Each YAML file in rules/active/ must follow this schema.
     3	
     4	# Required fields:
     5	#   id: string          — Unique rule identifier (e.g. R-001)
     6	#   name: string        — Human-readable name
     7	#   axiom: string       — Constitutional basis (e.g. "Law 1", "Rule 22")
     8	#   file_glob: string   — File pattern to match (e.g. "kernel.rs", "*.rs")
     9	#   check:
    10	#     type: string      — "grep" | "grep_inverse" | "compound"
    11	#     pattern: string   — Regex pattern to search for
    12	#   enforcement: string — "block" (exit 2) | "warn" (exit 0 + log)
    13	#   message: string     — Human-readable violation message
    14	
    15	# Optional fields:
    16	#   source_incidents: list  — Incident IDs that motivated this rule
    17	#   stats:
    18	#     triggers: int         — Total trigger count (updated by engine)
    19	#     last_triggered: string — ISO timestamp
    20	# TuringOS v4 Rule Engine
    21	
    22	## Architecture
    23	```
    24	CLAUDE.md instructions (~70% compliance)
    25	  + hooks/judge.sh (closes gap to ~100%)
    26	    + rules/engine.py (evaluates YAML rules)
    27	      + rules/active/*.yaml (dynamic, add/remove = add/remove file)
    28	```
    29	
    30	## How It Works
    31	1. Claude edits a file → `judge.sh` receives JSON on stdin
    32	2. `judge.sh` calls `rules/engine.py` with file path + content
    33	3. Engine loads all YAML rules, filters by `file_glob`
    34	4. For each matching rule: runs `check.pattern` regex against content
    35	5. Block rule matches → exit 2 (edit rejected)
    36	6. Warn rule matches → exit 0 + log to enforcement.log + trace
    37	
    38	## Rule Schema
    39	See `rules/SCHEMA.yaml` for full spec.
    40	
    41	## Active Rules
    42	
    43	### Block Level (exit 2 — hard enforcement)
    44	| ID | Name | Axiom |
    45	|----|------|-------|
    46	| R-001 | kernel_purity | Law 1: zero domain knowledge |
    47	| R-002 | no_coin_minting | Law 2: no post-genesis printing |
    48	| R-003 | no_wal_deletion | Tape append-only |
    49	| R-004 | lean_syntax_in_prompts | Rule 22: black-box |
    50	| R-005 | forced_investment | Law 2: voluntary staking |
    51	
    52	### Warn Level (exit 0 — advisory + log)
    53	| ID | Name | Axiom |
    54	|----|------|-------|
    55	| R-006 | kernel_modification | Law 1 |
    56	| R-007 | bus_lifecycle | Engine separation |
    57	| R-008 | market_constants | Law 2 |
    58	| R-009 | payload_limits | Rule 21 |
    59	| R-013 | format_contract | Bitter Lesson (V-009) |
    60	
    61	## Adding a Rule
    62	1. Create `rules/active/R-xxx_name.yaml` following SCHEMA
    63	2. Done. Engine picks it up automatically.
    64	Hard cap: 30 rules maximum.
    65	
    66	## Traces
    67	Rule triggers are logged to `traces/sessions/{date}.jsonl` for analysis.
    68	Use `/harness-reflect` to review rule effectiveness.
    69	id: "R-015"
    70	name: "trace_matrix_pub_symbol_warn"
    71	source_incidents:
    72	  - "F-2026-04-25-04"  # B7 alignment retroactive fix — pub symbols shipped without TRACE_MATRIX backlink
    73	fc_trace: "CLAUDE.md Alignment Standard — every src/ pub symbol must map to FC1/FC2/FC3 element"
    74	axiom: "Every pub fn / struct / enum / trait / const in src/ must carry /// TRACE_MATRIX FCx-Nx: <role> doc-comment OR be filed as orphan with explicit cases/Cxxx or PREREG-§n.m justification in TRACE_MATRIX_v?.md"
    75	trigger: "pre_edit"
    76	check:
    77	  type: "grep"
    78	  pattern: "pub (fn|struct|enum|trait|const|mod) "
    79	file_glob: "*.rs"
    80	enforcement: "warn"
    81	message: "REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift."
    82	stats:
    83	  times_triggered: 0
    84	  last_triggered: ""

exec
/bin/bash -lc 'rg -n "Audit Standard|Alignment Standard|TRACE_MATRIX|Generator|Evaluator|VETO|CHALLENGE|PASS" CLAUDE.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
17:## Audit Standard (Art. V.1 + C-010 + C-023 + C-035)
18:- Generator ≠ Evaluator：代码作者不可是唯一审计者
19:- 所有 merge / phase 决策双外审（Codex + Gemini）；VETO > CHALLENGE > PASS
34:## Alignment Standard (Art. IV + C-069)
35:- 权威对齐文件: `handover/alignment/TRACE_MATRIX_v0_2026-04-22.md`
36:  (后续 rev: `TRACE_MATRIX_vN.md`)
38:  或 BLOCK merge。doc-comment backlink 格式: `/// TRACE_MATRIX <FC-id>: <role>`

exec
/bin/bash -lc "nl -ba docs/rules.md | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 Rule Engine
     2	
     3	## Architecture
     4	```
     5	CLAUDE.md instructions (~70% compliance)
     6	  + hooks/judge.sh (closes gap to ~100%)
     7	    + rules/engine.py (evaluates YAML rules)
     8	      + rules/active/*.yaml (dynamic, add/remove = add/remove file)
     9	```
    10	
    11	## How It Works
    12	1. Claude edits a file → `judge.sh` receives JSON on stdin
    13	2. `judge.sh` calls `rules/engine.py` with file path + content
    14	3. Engine loads all YAML rules, filters by `file_glob`
    15	4. For each matching rule: runs `check.pattern` regex against content
    16	5. Block rule matches → exit 2 (edit rejected)
    17	6. Warn rule matches → exit 0 + log to enforcement.log + trace
    18	
    19	## Rule Schema
    20	See `rules/SCHEMA.yaml` for full spec.
    21	
    22	## Active Rules
    23	
    24	### Block Level (exit 2 — hard enforcement)
    25	| ID | Name | Axiom |
    26	|----|------|-------|
    27	| R-001 | kernel_purity | Law 1: zero domain knowledge |
    28	| R-002 | no_coin_minting | Law 2: no post-genesis printing |
    29	| R-003 | no_wal_deletion | Tape append-only |
    30	| R-004 | lean_syntax_in_prompts | Rule 22: black-box |
    31	| R-005 | forced_investment | Law 2: voluntary staking |
    32	
    33	### Warn Level (exit 0 — advisory + log)
    34	| ID | Name | Axiom |
    35	|----|------|-------|
    36	| R-006 | kernel_modification | Law 1 |
    37	| R-007 | bus_lifecycle | Engine separation |
    38	| R-008 | market_constants | Law 2 |
    39	| R-009 | payload_limits | Rule 21 |
    40	| R-013 | format_contract | Bitter Lesson (V-009) |
    41	
    42	## Adding a Rule
    43	1. Create `rules/active/R-xxx_name.yaml` following SCHEMA
    44	2. Done. Engine picks it up automatically.
    45	Hard cap: 30 rules maximum.
    46	
    47	## Traces
    48	Rule triggers are logged to `traces/sessions/{date}.jsonl` for analysis.
    49	Use `/harness-reflect` to review rule effectiveness.

exec
/bin/bash -lc "nl -ba rules/SCHEMA.yaml | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TuringOS v4 Rule Schema
     2	# Each YAML file in rules/active/ must follow this schema.
     3	
     4	# Required fields:
     5	#   id: string          — Unique rule identifier (e.g. R-001)
     6	#   name: string        — Human-readable name
     7	#   axiom: string       — Constitutional basis (e.g. "Law 1", "Rule 22")
     8	#   file_glob: string   — File pattern to match (e.g. "kernel.rs", "*.rs")
     9	#   check:
    10	#     type: string      — "grep" | "grep_inverse" | "compound"
    11	#     pattern: string   — Regex pattern to search for
    12	#   enforcement: string — "block" (exit 2) | "warn" (exit 0 + log)
    13	#   message: string     — Human-readable violation message
    14	
    15	# Optional fields:
    16	#   source_incidents: list  — Incident IDs that motivated this rule
    17	#   stats:
    18	#     triggers: int         — Total trigger count (updated by engine)
    19	#     last_triggered: string — ISO timestamp

exec
/bin/bash -lc "nl -ba rules/active/R-015_trace_matrix_pub_symbol.yaml | sed -n '1,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	id: "R-015"
     2	name: "trace_matrix_pub_symbol_warn"
     3	source_incidents:
     4	  - "F-2026-04-25-04"  # B7 alignment retroactive fix — pub symbols shipped without TRACE_MATRIX backlink
     5	fc_trace: "CLAUDE.md Alignment Standard — every src/ pub symbol must map to FC1/FC2/FC3 element"
     6	axiom: "Every pub fn / struct / enum / trait / const in src/ must carry /// TRACE_MATRIX FCx-Nx: <role> doc-comment OR be filed as orphan with explicit cases/Cxxx or PREREG-§n.m justification in TRACE_MATRIX_v?.md"
     7	trigger: "pre_edit"
     8	check:
     9	  type: "grep"
    10	  pattern: "pub (fn|struct|enum|trait|const|mod) "
    11	file_glob: "*.rs"
    12	enforcement: "warn"
    13	message: "REMINDER (R-015 / Alignment Standard): edit touches a pub symbol in a Rust file. New OR modified pub items MUST carry `/// TRACE_MATRIX FC?-N?: <role>` doc-comment AND have an entry in handover/alignment/TRACE_MATRIX_v?.md (current: v1_2026-04-25). If this is genuinely orphan, file under TRACE_MATRIX § 3 with explicit Constitutional Justification (cases/Cxxx or PREREG-§n.m). Untraced pub symbols cause silent constitutional drift."
    14	stats:
    15	  times_triggered: 0
    16	  last_triggered: ""

codex
The first hard issue is already visible: the spec’s R-022 YAML uses a new `external_script` type, but the current engine only supports `grep`, `grep_inverse`, and `compound`, and its CLI has no `--rule` selector. I’m also checking whether the 5-line backlink window is true in the current `src/` corpus instead of taking the 95% claim on faith.
exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import re
pub_re = re.compile(r'"'^'"\\s*pub(\\(["'^)]*'"\\))?\\s+(fn|struct|enum|trait|const|mod)\\b')
counts = {\"pub\":0,\"with_trace_5\":0,\"with_trace_10\":0,\"with_trace_20\":0,\"with_trace_any_prev_doc\":0}
dist_counts = {}
examples_far=[]
examples_none=[]
for path in sorted(Path('src').rglob('*.rs')):
    lines = path.read_text(errors='replace').splitlines()
    for i,line in enumerate(lines):
        if not pub_re.search(line):
            continue
        counts['pub'] += 1
        dist = None
        # Look up to 50 lines above, but stop after first non-attr/non-doc/comment/blank? Also compute raw distance.
        for j in range(i-1, max(-1,i-51), -1):
            if 'TRACE_MATRIX' in lines[j] and lines[j].lstrip().startswith('///'):
                dist = i-j
                break
        if dist is not None:
            dist_counts[dist] = dist_counts.get(dist,0)+1
            if dist <= 5: counts['with_trace_5'] += 1
            if dist <= 10: counts['with_trace_10'] += 1
            if dist <= 20: counts['with_trace_20'] += 1
            counts['with_trace_any_prev_doc'] += 1
            if dist > 5 and len(examples_far)<20:
                examples_far.append(f\"{path}:{i+1} dist={dist}: {line.strip()}\")
        else:
            if len(examples_none)<20:
                examples_none.append(f\"{path}:{i+1}: {line.strip()}\")
print(counts)
print('dist_counts', sorted(dist_counts.items())[:30])
print('far_examples')
print('\\n'.join(examples_far))
print('none_examples')
print('\\n'.join(examples_none))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
{'pub': 409, 'with_trace_5': 102, 'with_trace_10': 115, 'with_trace_20': 130, 'with_trace_any_prev_doc': 148}
dist_counts [(1, 35), (2, 34), (3, 13), (4, 12), (5, 8), (6, 6), (7, 1), (8, 2), (9, 3), (10, 1), (11, 4), (12, 1), (14, 2), (15, 3), (16, 2), (17, 2), (18, 1), (21, 3), (23, 2), (24, 1), (27, 1), (35, 1), (37, 3), (39, 1), (42, 2), (44, 1), (48, 1), (49, 1), (50, 1)]
far_examples
src/boot.rs:36 dist=6: pub enum TrustRootError {
src/boot.rs:71 dist=9: pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> {
src/boot.rs:289 dist=8: pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> {
src/bottom_white/ledger/system_keypair.rs:89 dist=6: pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]);
src/bottom_white/ledger/system_keypair.rs:104 dist=21: pub(crate) mod serde_bytes_64 {
src/bottom_white/ledger/system_keypair.rs:110 dist=27: pub fn serialize<S: Serializer>(bytes: &[u8; 64], s: S) -> Result<S::Ok, S::Error> {
src/bottom_white/ledger/system_keypair.rs:118 dist=35: pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 64], D::Error> {
src/bottom_white/ledger/system_keypair.rs:578 dist=10: pub(crate) mod terminal_summary_emitter {
src/bottom_white/ledger/system_keypair.rs:641 dist=6: pub(crate) mod transition_ledger_emitter {
src/bottom_white/ledger/transition_ledger.rs:69 dist=6: pub struct LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:108 dist=9: pub struct LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:122 dist=23: pub fn canonical_digest(&self) -> Hash {
src/bottom_white/ledger/transition_ledger.rs:149 dist=50: pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:189 dist=6: pub trait LedgerWriter: Send + Sync {
src/bottom_white/ledger/transition_ledger.rs:210 dist=16: pub enum LedgerWriterError {
src/bottom_white/ledger/transition_ledger.rs:231 dist=37: pub struct InMemoryLedgerWriter {
src/bottom_white/ledger/transition_ledger.rs:236 dist=42: pub fn new() -> Self {
src/bus.rs:28 dist=9: pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0;
src/bus.rs:33 dist=14: pub struct BusConfig {
src/bus.rs:56 dist=37: pub struct TuringBus {
none_examples
src/bottom_white/cas/mod.rs:11: pub mod schema;
src/bottom_white/cas/mod.rs:12: pub mod store;
src/bottom_white/cas/schema.rs:14: pub struct Cid(pub [u8; 32]);
src/bottom_white/cas/schema.rs:18: pub fn from_content(content: &[u8]) -> Self {
src/bottom_white/cas/schema.rs:25: pub fn hex(&self) -> String {
src/bottom_white/cas/schema.rs:42: pub enum ObjectType {
src/bottom_white/cas/schema.rs:60: pub struct CasObjectMetadata {
src/bottom_white/cas/schema.rs:79: pub fn canonical_hash(&self) -> [u8; 32] {
src/bottom_white/cas/store.rs:38: pub enum CasError {
src/bottom_white/cas/store.rs:136: pub struct CasStore {
src/bottom_white/cas/store.rs:148: pub fn open(repo_path: &Path) -> Result<Self, CasError> {
src/bottom_white/cas/store.rs:163: pub fn put(
src/bottom_white/cas/store.rs:199: pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
src/bottom_white/cas/store.rs:225: pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
src/bottom_white/cas/store.rs:229: pub fn len(&self) -> usize {
src/bottom_white/cas/store.rs:233: pub fn is_empty(&self) -> bool {
src/bottom_white/cas/store.rs:238: pub fn merkle_root(&self) -> [u8; 32] {
src/bottom_white/ledger/transition_ledger.rs:285: pub enum ReplayMode {
src/bottom_white/ledger/transition_ledger.rs:291: pub enum ReplayError {
src/bottom_white/ledger/transition_ledger.rs:355: pub trait LedgerCasView {

exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import re
patterns = {
'exact_spec': re.compile(r'"'^'"\\s*pub\\s+(fn|struct|enum|trait|const|mod)\\b'),
'visibility_any': re.compile(r'"'^'"\\s*pub(\\(["'^)]*'"\\))?\\s+(fn|struct|enum|trait|const|mod)\\b'),
}
for name,pub_re in patterns.items():
    dists=[]; total=0
    for path in sorted(Path('src').rglob('*.rs')):
        lines = path.read_text(errors='replace').splitlines()
        for i,line in enumerate(lines):
            if not pub_re.search(line):
                continue
            total += 1
            dist=None
            for j in range(i-1, max(-1,i-51), -1):
                if 'TRACE_MATRIX' in lines[j] and lines[j].lstrip().startswith('///'):
                    dist=i-j;break
            if dist is not None: dists.append(dist)
    print(name, 'total',total,'traced<=50',len(dists),'<=5',sum(x<=5 for x in dists),'pct of traced <=5', (sum(x<=5 for x in dists)/len(dists)*100 if dists else 0),'<=10',sum(x<=10 for x in dists),'<=20',sum(x<=20 for x in dists))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exact_spec total 393 traced<=50 134 <=5 93 pct of traced <=5 69.40298507462687 <=10 102 <=20 117
visibility_any total 409 traced<=50 148 <=5 102 pct of traced <=5 68.91891891891892 <=10 115 <=20 130

exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import re
pub_re = re.compile(r'"'^'"\\s*pub(\\(["'^)]*'"\\))?\\s+(fn|struct|enum|trait|const|mod)\\b')
traced=[]; total=0; far=[]
for path in sorted(Path('src').rglob('*.rs')):
    lines = path.read_text(errors='replace').splitlines()
    for i,line in enumerate(lines):
        if not pub_re.search(line):
            continue
        total += 1
        block=[]
        j=i-1
        while j>=0:
            s=lines[j].strip()
            if s=='' or s.startswith('///') or s.startswith('#[') or s.startswith('//'):
                block.append((j, lines[j]))
                j-=1
                continue
            break
        trace_lines=[j for j,l in block if 'TRACE_MATRIX' in l and l.lstrip().startswith('///')]
        if trace_lines:
            dist=i-max(trace_lines)  # closest trace above
            traced.append(dist)
            if dist>5 and len(far)<30:
                far.append(f\"{path}:{i+1} dist={dist}: {line.strip()} | trace_line={max(trace_lines)+1}\")
print('total', total, 'traced_in_pre_item_block', len(traced), '<=5', sum(d<=5 for d in traced), 'pct', (sum(d<=5 for d in traced)/len(traced)*100 if traced else 0), '<=10', sum(d<=10 for d in traced), '<=20', sum(d<=20 for d in traced))
from collections import Counter
print('dists', sorted(Counter(traced).items()))
print('far')
print('\\n'.join(far))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
total 409 traced_in_pre_item_block 120 <=5 101 pct 84.16666666666667 <=10 113 <=20 117
dists [(1, 35), (2, 33), (3, 13), (4, 12), (5, 8), (6, 6), (7, 1), (8, 1), (9, 3), (10, 1), (11, 3), (15, 1), (21, 1), (23, 1), (24, 1)]
far
src/boot.rs:36 dist=6: pub enum TrustRootError { | trace_line=30
src/boot.rs:71 dist=9: pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> { | trace_line=62
src/boot.rs:289 dist=8: pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> { | trace_line=281
src/bottom_white/ledger/system_keypair.rs:89 dist=6: pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]); | trace_line=83
src/bottom_white/ledger/system_keypair.rs:578 dist=10: pub(crate) mod terminal_summary_emitter { | trace_line=568
src/bottom_white/ledger/system_keypair.rs:641 dist=6: pub(crate) mod transition_ledger_emitter { | trace_line=635
src/bottom_white/ledger/transition_ledger.rs:69 dist=6: pub struct LedgerEntry { | trace_line=63
src/bottom_white/ledger/transition_ledger.rs:108 dist=9: pub struct LedgerEntrySigningPayload { | trace_line=99
src/bottom_white/ledger/transition_ledger.rs:189 dist=6: pub trait LedgerWriter: Send + Sync { | trace_line=183
src/bus.rs:28 dist=9: pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4: u32 = 0; | trace_line=19
src/state/sequencer.rs:47 dist=7: pub(crate) fn dispatch_transition( | trace_line=40
src/state/sequencer.rs:91 dist=23: pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) { | trace_line=68
src/state/sequencer.rs:230 dist=24: pub struct Sequencer { | trace_line=206
src/state/typed_tx.rs:52 dist=11: pub struct ClaimId(pub TxId); | trace_line=41
src/state/typed_tx.rs:223 dist=11: pub struct WorkTx { | trace_line=212
src/state/typed_tx.rs:301 dist=21: pub struct FinalizeRewardTx { | trace_line=280
src/state/typed_tx.rs:342 dist=15: pub struct TerminalSummaryTx { | trace_line=327
src/state/typed_tx.rs:717 dist=11: pub enum TransitionError { | trace_line=706
src/state/typed_tx.rs:830 dist=6: pub struct SignalBundle { | trace_line=824

exec
/bin/bash -lc "python3 - <<'PY'
from pathlib import Path
import re
pub_re = re.compile(r'"'^'"\\s*pub\\s+(fn|struct|enum|trait|const|mod)\\b')
traced=[]; total=0; far=[]
for path in sorted(Path('src').rglob('*.rs')):
    lines = path.read_text(errors='replace').splitlines()
    for i,line in enumerate(lines):
        if not pub_re.search(line):
            continue
        total += 1
        block=[]; j=i-1
        while j>=0:
            s=lines[j].strip()
            if s=='' or s.startswith('///') or s.startswith('#[') or s.startswith('//'):
                block.append((j,lines[j])); j-=1; continue
            break
        trace_lines=[j for j,l in block if 'TRACE_MATRIX' in l and l.lstrip().startswith('///')]
        if trace_lines:
            dist=i-max(trace_lines); traced.append(dist)
            if dist>5 and len(far)<20:
                far.append(f\"{path}:{i+1} dist={dist}: {line.strip()} | trace_line={max(trace_lines)+1}\")
print('exact_spec total', total, 'traced_in_pre_item_block', len(traced), '<=5', sum(d<=5 for d in traced), 'pct', (sum(d<=5 for d in traced)/len(traced)*100 if traced else 0), '<=10', sum(d<=10 for d in traced), '<=20', sum(d<=20 for d in traced))
print('\\n'.join(far))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 121ms:
exact_spec total 393 traced_in_pre_item_block 107 <=5 92 pct 85.98130841121495 <=10 100 <=20 104
src/boot.rs:36 dist=6: pub enum TrustRootError { | trace_line=30
src/boot.rs:71 dist=9: pub fn verify_trust_root(repo_root: &Path) -> Result<(), TrustRootError> { | trace_line=62
src/boot.rs:289 dist=8: pub fn parse_trust_root_section(text: &str) -> Result<Vec<(String, String)>, TrustRootError> { | trace_line=281
src/bottom_white/ledger/system_keypair.rs:89 dist=6: pub struct SystemSignature(#[serde(with = "serde_bytes_64")] [u8; SIGNATURE_LEN]); | trace_line=83
src/bottom_white/ledger/transition_ledger.rs:69 dist=6: pub struct LedgerEntry { | trace_line=63
src/bottom_white/ledger/transition_ledger.rs:108 dist=9: pub struct LedgerEntrySigningPayload { | trace_line=99
src/bottom_white/ledger/transition_ledger.rs:189 dist=6: pub trait LedgerWriter: Send + Sync { | trace_line=183
src/state/sequencer.rs:91 dist=23: pub fn advance_head_t(q: &mut QState, writer: &dyn LedgerWriter) { | trace_line=68
src/state/sequencer.rs:230 dist=24: pub struct Sequencer { | trace_line=206
src/state/typed_tx.rs:52 dist=11: pub struct ClaimId(pub TxId); | trace_line=41
src/state/typed_tx.rs:223 dist=11: pub struct WorkTx { | trace_line=212
src/state/typed_tx.rs:301 dist=21: pub struct FinalizeRewardTx { | trace_line=280
src/state/typed_tx.rs:342 dist=15: pub struct TerminalSummaryTx { | trace_line=327
src/state/typed_tx.rs:717 dist=11: pub enum TransitionError { | trace_line=706
src/state/typed_tx.rs:830 dist=6: pub struct SignalBundle { | trace_line=824

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v3_2026-04-27.md | sed -n '180,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	tests/four_element_mapping.rs
   181	tests/turing_fundamentalism.rs
   182	tests/tape_canonical_V01..V24.rs                    (24 tests)
   183	
   184	# ChainTape layers (CO1.0-1.9)
   185	tests/chain_tape_L0_constitution_root.rs
   186	tests/chain_tape_L1_predicate_registry.rs
   187	tests/chain_tape_L2_tool_registry.rs
   188	tests/chain_tape_L3_cas.rs
   189	tests/chain_tape_L4_transition_ledger.rs
   190	tests/chain_tape_L5_materialized_state.rs
   191	tests/chain_tape_L6_signal_indices.rs
   192	
   193	# State transition spec invariants I-1 through I-20 (CO1.SPEC.0)
   194	tests/transition_determinism.rs                    (I-DET)
   195	tests/no_hidden_inputs.rs                          (I-NOSIDE)
   196	tests/stale_parent_rejection.rs                    (I-PARENT)
   197	tests/signature_verification.rs                    (I-SIG)
   198	tests/stake_atomicity.rs                           (I-STAKE)
   199	tests/no_wall_clock_in_tx.rs                       (I-LOGTIME)
   200	tests/no_f64_money.rs                              (I-MICROCOIN)
   201	tests/q_state_uses_btree.rs                        (I-BTREE)
   202	tests/no_rejection_sidecar.rs                      (I-NOSIDECAR)
   203	tests/retry_summary_runner_signed.rs               (I-RETRY)
   204	tests/run_terminal_invariant.rs                    (I-TERMINAL)
   205	tests/no_env_in_transition.rs                      (I-NOENV)
   206	tests/task_config_frozen_at_publish.rs             (I-FREEZE-CONFIG)
   207	tests/no_runtime_entropy.rs                        (I-NORANDOM)
   208	tests/verify_target_liveness.rs                    (I-VERIFY-LIVE)
   209	tests/challenge_window_enforced.rs                 (I-CHAL-WINDOW)
   210	tests/finalize_or_slash_exclusive.rs               (I-FINALIZE-EXCLUSIVE)
   211	
   212	# Genesis (CO1.0)
   213	tests/genesis_constitution_root_verify.rs
   214	tests/genesis_amendment_predicate_resolves.rs
   215	tests/genesis_initial_registry_empty.rs
   216	tests/genesis_boot_attestation_self_referential.rs
   217	tests/genesis_creator_signature_verifies.rs
   218	
   219	# Predicates + Visibility (CO1.5, CO1.11)
   220	tests/safety_creation_dichotomy.rs
   221	tests/private_predicate_error_no_leak.rs
   222	tests/agent_view_filters_internals.rs
   223	tests/agent_view_minimal_context.rs
   224	tests/goodhart_shield.rs
   225	
   226	# Signals (CO1.9, CO1.10)
   227	tests/signal_dichotomy.rs
   228	tests/boolean_signal_pass_fail.rs
   229	tests/statistical_signals_complete.rs
   230	tests/price_broadcast_l6.rs
   231	tests/price_aggregation_correlation_shield.rs
   232	
   233	# Reports (CLAUDE.md Report Standard)
   234	tests/report_standard_pput_ci_required.rs
   235	tests/halt_reason_distribution.rs
   236	tests/entropy_diversity_thresholds.rs
   237	
   238	# Economic invariants (CO P2.*)
   239	tests/economic_invariant_INV1_no_thinking_reward.rs
   240	tests/economic_invariant_INV2_no_direct_collect.rs

exec
/bin/bash -lc 'rg -n "§ F|R-022|check_trace_matrix|orphan|§ 3|Orphan" handover/alignment/TRACE_MATRIX_v3_2026-04-27.md' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
60:| § 3 反奥利奥三层 | top/middle/bottom white | N | `src/{top_white,middle_black,bottom_white,economy}/*` directory structure | `tests/anti_oreo_layer_audit.rs` | CO1.1.* |
102:| § 21 最终公式 | reward_i = Finalize(Escrow × Accept × Attribution × Survival × Utility × Constitution) | N | `economy::settlement_engine::finalize_reward` (per `STATE_TRANSITION_SPEC_v1` § 3.4) | `tests/final_reward_formula.rs` | CO P2.6.4 |
135:**Forbidden state**: any Normative row with empty "code symbol" or empty "conformance test" column. Pre-commit hook R-022 (added per Plan v3.2 CO P0.8) enforces.
139:## § F — Bidirectional Reverse: Code Symbol → Source
149:This reverse map is auto-generated by `scripts/check_trace_matrix_updated.sh` per Plan v3.2 CO1.13.2 atom. Pre-commit hook R-022 enforces "every `pub` symbol in src/{top_white,middle_black,bottom_white,economy,state,transition,governance}/*.rs MUST have a `/// TRACE_MATRIX <id>: <role>` doc-comment". Build fails if missing.
322:- Replace the per-atom doc-comment `/// TRACE_MATRIX <id>: <role>` in each `pub` symbol (R-022 hook enforces at commit time)

exec
/bin/bash -lc 'git status --short && git rev-parse --short HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
?? handover/audits/CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md
?? handover/audits/GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md
?? handover/audits/run_codex_co1_13_round1_audit.sh
?? handover/audits/run_codex_co1_8_round1_audit.sh
?? handover/audits/run_gemini_co1_13_round1_audit.py
?? handover/audits/run_gemini_co1_8_round1_audit.py
8d88f2d

exec
/bin/bash -lc "nl -ba handover/alignment/TRACE_MATRIX_v3_2026-04-27.md | sed -n '128,156p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   128	| WP economic §§ | 8 (numbered 0/2/7/15/18/19/20/21) | 7 | 1 (cross-ref table) | embedded in partial rows |
   129	| RSP appendix | 4 sub-§ | 3 | 1 | — |
   130	
   131	**Total Normative coverage**: ~51 rows. Each Normative row has at least 1 conformance test path (existing or planned in Plan v3.2 atoms).
   132	
   133	**Test count from this matrix**: ~60-70 distinct conformance tests (some rows share tests; e.g., Goodhart shield).
   134	
   135	**Forbidden state**: any Normative row with empty "code symbol" or empty "conformance test" column. Pre-commit hook R-022 (added per Plan v3.2 CO P0.8) enforces.
   136	
   137	---
   138	
   139	## § F — Bidirectional Reverse: Code Symbol → Source
   140	
   141	This section is populated incrementally as code lands (currently empty for v4 since CO P1 has not started). Format:
   142	
   143	```
   144	src/path/to/symbol.rs::function_name
   145	  ↓
   146	  TRACE_MATRIX_v3 row: <Constitution Art X | WP arch § Y | WP econ § Z>
   147	```
   148	
   149	This reverse map is auto-generated by `scripts/check_trace_matrix_updated.sh` per Plan v3.2 CO1.13.2 atom. Pre-commit hook R-022 enforces "every `pub` symbol in src/{top_white,middle_black,bottom_white,economy,state,transition,governance}/*.rs MUST have a `/// TRACE_MATRIX <id>: <role>` doc-comment". Build fails if missing.
   150	
   151	**Initial state at v4 ratification (2026-04-27)**: section is **empty by design** — code does not yet exist. v4 will populate it commit-by-commit during CO P1+P2.
   152	
   153	---
   154	
   155	## § G — Deferred Items Justification
   156	

exec
/bin/bash -lc "nl -ba .claude/hooks/judge.sh | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# TuringOS v4 — Judge Hook (PreToolUse)
     3	# Combines v3's block-destructive.sh + rule-engine.sh into one entry point.
     4	# Interface: JSON on stdin (Claude Code hook protocol). Exit 0 = allow, exit 2 = block.
     5	#
     6	# A0e-fix 2026-04-25 (post Phase A0 dual audit, both auditors CHALLENGE):
     7	# - Fixed multiple constitution.md guard bypass paths (Bash sed -i,
     8	#   symlink basename, empty-content edit). Now constitution.md is the
     9	#   FIRST guard, with realpath resolution.
    10	# - Fixed R-016 git commit -F /tmp/msg bypass: read message from -F file
    11	#   if present.
    12	# - FC-trace: FC3-S3 readonly subgraph + Art. V.1.1 sudo gate + C-074
    13	#   FC-first commit discipline.
    14	
    15	set -uo pipefail
    16	
    17	SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    18	PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
    19	TRACES_DIR="$PROJECT_ROOT/traces/sessions"
    20	
    21	INPUT=$(cat)
    22	TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // empty' 2>/dev/null)
    23	FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty' 2>/dev/null)
    24	COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty' 2>/dev/null)
    25	CONTENT=$(echo "$INPUT" | jq -r '.tool_input.new_string // .tool_input.content // empty' 2>/dev/null)
    26	
    27	# ──────────────────────────────────────────────────────────────────────
    28	# CONSTITUTION GUARD (FIRST — before any tool-specific branch)
    29	# ──────────────────────────────────────────────────────────────────────
    30	# Per Codex A0e finding #1: previous version had Bash branch exit before
    31	# this guard, allowing `sed -i constitution.md`, `tee constitution.md`,
    32	# etc. to bypass R-018. Fixed by hoisting the guard to the top.
    33	#
    34	# Per Gemini A0e Q1.c: previous version used basename() which is symlink-
    35	# vulnerable. Fixed by realpath resolution before basename comparison.
    36	#
    37	# Per Codex A0e finding #1 (continued): previous version required
    38	# nonempty CONTENT, allowing empty-replacement edits to bypass. Fixed
    39	# by NOT gating on CONTENT for constitution.md.
    40	
    41	constitution_target() {
    42	    # Returns 0 if any of the inputs target constitution.md (resolved
    43	    # through symlinks), 1 otherwise.
    44	    local target="$1"
    45	    if [ -z "$target" ]; then return 1; fi
    46	    # Use realpath -m (allow non-existent paths) to handle both existing
    47	    # files and to-be-created scenarios.
    48	    local resolved
    49	    resolved=$(realpath -m -- "$target" 2>/dev/null || echo "$target")
    50	    local expected
    51	    expected=$(realpath -m -- "$PROJECT_ROOT/constitution.md" 2>/dev/null || echo "$PROJECT_ROOT/constitution.md")
    52	    if [ "$resolved" = "$expected" ]; then return 0; fi
    53	    # Also catch by basename for safety (in case realpath fails)
    54	    if [ "$(basename -- "$target")" = "constitution.md" ]; then return 0; fi
    55	    return 1
    56	}
    57	
    58	bash_targets_constitution() {
    59	    # Returns 0 if a Bash command is mutating constitution.md.
    60	    local cmd="$1"
    61	    if [ -z "$cmd" ]; then return 1; fi
    62	    # A0e-fix-2 2026-04-25: skip if command is `git ...`. Git itself never
    63	    # mutates constitution.md inline; quoted commit messages may contain
    64	    # literal mutation-pattern text (e.g., "sed -i constitution.md" in a
    65	    # changelog) that would false-positive.
    66	    if echo "$cmd" | grep -qE '^[[:space:]]*git[[:space:]]'; then return 1; fi
    67	    # Common mutation patterns: sed -i, tee, awk -i, > redirect, >> append,
    68	    # python/perl/ruby file write, etc.
    69	    if echo "$cmd" | grep -qE '(sed|awk|perl|tee)[[:space:]].*constitution\.md'; then return 0; fi
    70	    if echo "$cmd" | grep -qE '(>|>>)[[:space:]]*[^|&;]*constitution\.md'; then return 0; fi
    71	    if echo "$cmd" | grep -qE 'cat[[:space:]].*>[[:space:]]*[^|&;]*constitution\.md'; then return 0; fi
    72	    if echo "$cmd" | grep -qE 'rm[[:space:]].*constitution\.md'; then return 0; fi
    73	    if echo "$cmd" | grep -qE 'mv[[:space:]].*[[:space:]]constitution\.md'; then return 0; fi
    74	    return 1
    75	}
    76	
    77	# 1. Edit/Write targeting constitution.md → BLOCK (R-018)
    78	if [ -n "$FILE_PATH" ] && constitution_target "$FILE_PATH"; then
    79	    echo "BLOCKED by R-018 (constitution_amendment_sudo): edit targets constitution.md"
    80	    echo "  Per Art. V.1.1 amendment 2026-04-25: sudo applies *only* to constitution.md."
    81	    echo "  To proceed: USER must explicitly type 'I authorize this constitution amendment' (verbatim) in chat."
    82	    echo "  See cases/C-071_constitution_amendment_process.yaml for the 4-step workflow."
    83	    exit 2
    84	fi
    85	
    86	# 2. Bash command mutating constitution.md → BLOCK
    87	if [ "$TOOL_NAME" = "Bash" ] && bash_targets_constitution "$COMMAND"; then
    88	    echo "BLOCKED by R-018 (constitution_amendment_sudo): Bash command mutates constitution.md"
    89	    echo "  Detected pattern: command targets constitution.md via sed/tee/awk/redirect/rm/mv"
    90	    echo "  Per Art. V.1.1 + C-071: constitution.md is sudo-only. Use Edit tool with explicit user authorization."
    91	    echo "  Command (truncated): $(echo "$COMMAND" | head -c 200)"
    92	    exit 2
    93	fi
    94	
    95	# ──────────────────────────────────────────────────────────────────────
    96	# Bash: destructive command checks + R-016 fc_trace
    97	# ──────────────────────────────────────────────────────────────────────
    98	if [ "$TOOL_NAME" = "Bash" ] && [ -n "$COMMAND" ]; then
    99	    # Block rm -rf on dangerous paths
   100	    if echo "$COMMAND" | grep -qE 'rm\s+(-[a-zA-Z]*r[a-zA-Z]*f|--recursive\s+--force|-[a-zA-Z]*f[a-zA-Z]*r)\s'; then
   101	        if echo "$COMMAND" | grep -qE '(^|\s)(\/|~\/|\.\.\/|\.claude)'; then
   102	            echo "BLOCKED: rm -rf on dangerous path: $COMMAND"
   103	            exit 2
   104	        fi
   105	    fi
   106	    # Block git push --force
   107	    if echo "$COMMAND" | grep -qE 'git\s+push\s+.*--force|git\s+push\s+-f'; then
   108	        echo "BLOCKED: git push --force is prohibited."
   109	        exit 2
   110	    fi
   111	    # Block git reset --hard
   112	    if echo "$COMMAND" | grep -qE 'git\s+reset\s+--hard'; then
   113	        echo "BLOCKED: git reset --hard is prohibited."
   114	        exit 2
   115	    fi
   116	    # Block WAL deletion
   117	    if echo "$COMMAND" | grep -qE 'rm\s.*\.(wal|jsonl)'; then
   118	        echo "BLOCKED: WAL/ledger file deletion is prohibited."
   119	        exit 2
   120	    fi
   121	    # Block sed/awk on kernel constants
   122	    if echo "$COMMAND" | grep -qE '(sed|awk).*kernel\.rs'; then
   123	        echo "BLOCKED: sed/awk on kernel.rs is prohibited. Use Edit tool."
   124	        exit 2
   125	    fi
   126	
   127	    # R-016 fc_trace_in_commit (added 2026-04-25, A0e-fix expanded):
   128	    # warn on git commit without FC-trace anywhere in resolvable message body.
   129	    # FC-trace: feedback_fc_first_problem_handling memory + case C-074.
   130	    if echo "$COMMAND" | grep -qE 'git\s+commit(\s|$)'; then
   131	        # Resolve message body from multiple sources:
   132	        #   1. -m "..." (HEREDOC or inline)
   133	        #   2. -F /path/to/file
   134	        #   3. interactive editor (no -m, no -F) — can't inspect; warn anyway
   135	        #
   136	        # A0e-fix 2026-04-25 (Codex Q1.d + Gemini Q1.d): previous version
   137	        # only greppedthe COMMAND string for `FC-trace:`. `git commit -F /tmp/msg`
   138	        # would silently bypass. Now we extract and check the actual message.
   139	        msg_check_passed=0
   140	
   141	        # Inline -m or HEREDOC: COMMAND itself contains the message
   142	        if echo "$COMMAND" | grep -qE 'FC-trace:'; then
   143	            msg_check_passed=1
   144	        fi
   145	
   146	        # -F file: extract path, read content
   147	        if [ "$msg_check_passed" -eq 0 ]; then
   148	            msg_file=$(echo "$COMMAND" | grep -oE '\-F[[:space:]]+[^[:space:];|&]+' | head -1 | sed 's/^-F[[:space:]]*//')
   149	            if [ -n "$msg_file" ] && [ -f "$msg_file" ]; then
   150	                if grep -qE 'FC-trace:' "$msg_file" 2>/dev/null; then
   151	                    msg_check_passed=1
   152	                fi
   153	            fi
   154	        fi
   155	
   156	        if [ "$msg_check_passed" -eq 0 ]; then
   157	            echo "WARNING R-016 / FC-first: git commit without 'FC-trace: <FC?-N?>' in message body." >&2
   158	            echo "  Per memory feedback_fc_first_problem_handling + case C-074: every code commit must trace to a FC1/FC2/FC3 element OR explicitly cite orphan justification (cases/Cxxx, PREREG-§n.m)." >&2
   159	            echo "  If fix legitimately doesn't map to flowchart, write 'FC-trace: orphan / <ref>' in message." >&2
   160	            echo "  (A0e-fix: hook now also reads -F file; if you used neither -m nor -F (interactive editor), inspection is best-effort only.)" >&2
   161	            # warn only — does not block; user decides whether to amend
   162	        fi
   163	
   164	        # R-020 commit_claim_diff_parity (added 2026-04-26, A8e12 — case C-076):
   165	        # warn on multi-fix-bundle commit messages without per-claim
   166	        # `Verified:` proof lines. Source: A8e9 N3 false-closure (Codex R9#1).
   167	        # Counts fix-tag occurrences (F1/F2/M1/M3/N1/P2/K1/etc) AND bullet
   168	        # headers ("fixed:" / "closed:" / "corrected:" / "updated:"). ≥2
   169	        # distinct claims and zero `Verified:` lines → WARN.
   170	        msg_body=""
   171	        if echo "$COMMAND" | grep -qE '\-m[[:space:]]'; then
   172	            # capture inline / HEREDOC body from COMMAND (best-effort —
   173	            # bash quoting makes perfect extraction hard; we accept
   174	            # false-positive from over-matching on the COMMAND string)
   175	            msg_body="$COMMAND"
   176	        fi
   177	        if [ -z "$msg_body" ]; then
   178	            msg_file=$(echo "$COMMAND" | grep -oE '\-F[[:space:]]+[^[:space:];|&]+' | head -1 | sed 's/^-F[[:space:]]*//')
   179	            if [ -n "$msg_file" ] && [ -f "$msg_file" ]; then
   180	                msg_body=$(cat "$msg_file" 2>/dev/null)

exec
/bin/bash -lc "nl -ba .claude/hooks/judge.sh | sed -n '180,340p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	                msg_body=$(cat "$msg_file" 2>/dev/null)
   181	            fi
   182	        fi
   183	        if [ -n "$msg_body" ]; then
   184	            # Count distinct fix-tag occurrences. Tag pattern = uppercase
   185	            # letter + digits, word-bounded, ALL-CAPS (avoids matching
   186	            # "F1" inside random words but accepts F1, F2, M1, M2, M3,
   187	            # N1, N2, P1, P2, K1, K2, etc.) plus the explicit
   188	            # bullet/header words.
   189	            fix_tag_count=$(echo "$msg_body" | grep -oE '\b[A-Z][0-9]+\b' | sort -u | wc -l)
   190	            bullet_count=$(echo "$msg_body" | grep -ciE '^[[:space:]]*[-*+]?[[:space:]]*\*\*?(fix|closed?|corrected|updated)[[:space:]a-z0-9_-]*:?\*?\*?:' || true)
   191	            total_claims=$((fix_tag_count + bullet_count))
   192	            verified_count=$(echo "$msg_body" | grep -cE 'Verified:' || true)
   193	
   194	            if [ "$total_claims" -ge 2 ] && [ "$verified_count" -eq 0 ]; then
   195	                echo "WARNING R-020 / case C-076: multi-fix-bundle commit detected (≥2 distinct fix tags or bullets) without per-claim 'Verified:' proof line." >&2
   196	                echo "  Per C-076 Rule 1: every fix-claim in commit body MUST be paired with a 'Verified:' line — typically 'grep -n' / 'sed -n' showing the diff contains the asserted change at the asserted location." >&2
   197	                echo "  Source: A8e9 N3 false-closure (Codex R9#1) — commit message claimed runner-default fix that the diff didn't ship; caught next round at ~\$5–7 audit-API cost." >&2
   198	                echo "  Cost of compliance: ~30s per claim. Cost of bypass: ~\$5–7 + 1 round delay if next dual audit catches the false-closure." >&2
   199	                # warn only — false-positive accepted (legitimate
   200	                # single-claim commit that happens to mention 2+
   201	                # tag-shaped tokens in unrelated context)
   202	            fi
   203	        fi
   204	    fi
   205	    exit 0
   206	fi
   207	
   208	# ──────────────────────────────────────────────────────────────────────
   209	# Edit/Write: rule engine
   210	# ──────────────────────────────────────────────────────────────────────
   211	if [ -n "$FILE_PATH" ] && [ -n "$CONTENT" ]; then
   212	    # Skip non-code files (docs, incidents, rules, handover, tests, audit)
   213	    case "$FILE_PATH" in
   214	        *.md|*/incidents/*|*/rules/*|*/handover/*|*/tests/*|*/audit/*) exit 0 ;;
   215	    esac
   216	
   217	    # Call the Python rule engine
   218	    if [ -d "$PROJECT_ROOT/rules/active" ]; then
   219	        RESULT=$(echo "$CONTENT" | python3 "$PROJECT_ROOT/rules/engine.py" \
   220	            --file "$FILE_PATH" \
   221	            --rules-dir "$PROJECT_ROOT/rules/active" \
   222	            --log "$PROJECT_ROOT/rules/enforcement.log" \
   223	            --traces-dir "$TRACES_DIR" 2>&1)
   224	        EXIT_CODE=$?
   225	        if [ $EXIT_CODE -eq 2 ]; then
   226	            echo "$RESULT"
   227	            exit 2
   228	        elif [ -n "$RESULT" ]; then
   229	            echo "$RESULT"
   230	        fi
   231	    fi
   232	fi
   233	
   234	exit 0

codex
**Section A: Verdict**

**CHALLENGE / HIGH conviction.** Not VETO because the shape is salvageable, but v1 cannot proceed to implementation as written. The R-022 path is currently non-functional, the hook surface is not reproducible, the orphan fallback points to a non-existent section, and the 5-line heuristic is empirically false for this repo.

**Section B: P0 Blockers**

1. **R-022 cannot run through the current engine.** Spec §1.2 lines 85-99 introduces `check.type: external_script`, but `rules/engine.py` only supports `grep`, `grep_inverse`, and `compound` at lines 50-64. It also has no `--rule R-022` CLI despite spec line 109 saying the shim pipes to engine.py with `--rule R-022`. `rules/SCHEMA.yaml` lines 9-11 also excludes `external_script`.

2. **`.git/hooks/pre-commit` is not a shippable artifact.** Spec lines 79-83, 109, and 196 put the hook in `.git/hooks/pre-commit`; that is untracked local state. It will not protect fresh clones, CI, or merges. If R-022 is meant to prevent untraced merges, add a tracked hook template plus installer and a CI/merge check.

3. **The orphan fallback target is undefined.** Spec §2.1 line 127 says to grep TRACE_MATRIX_v3 “§ 3” for orphan justification. The current matrix has no orphan §3; `§ 3` is only a WP table row. TRACE_MATRIX §F lines 139-151 discusses reverse map, not orphan exceptions.

4. **The 5-line window is not validated and is wrong for current style.** Spec §2.1 lines 123-125 and §5 line 181 rely on 5 lines. I measured current `src/`: for exact `pub fn|struct|enum|trait|const|mod`, only **92/107 = 86%** of traced pre-item doc blocks have `TRACE_MATRIX` within 5 lines; including `pub(crate)` style, **101/120 = 84%**. Use the entire immediately preceding doc/attribute block, not a raw 5-line window.

5. **Escape hatch is a silent bypass.** Spec lines 101, 132, 182, and 217 allow `// R-022-skip: <reason>` with quarterly manual audit only. A hard gate needs structured logging at minimum: symbol, file, line, reason, staged tree hash, timestamp. I would also require `cases/Cxxx`, `PREREG-§`, or `OBS_R022_*`.

6. **Boundary cases are not specified enough to implement.** Spec §2.1 line 123 says NEW pub lines, but does not define `#[cfg(test)]`, `pub use`, macro-generated APIs, `pub(crate)`, `pub type`, or removal of an existing backlink. These must be resolved before coding.

7. **Test plan is mismatched to the implementation.** Spec §3 lines 149-162 names Rust tests for shell/Python/git-hook behavior. Also, `git commit --dry-run` in line 141 is not a reliable proof that the hook blocks. Use shell integration tests or Rust temp-repo tests that invoke the script and one real temp commit.

**Section C: Open Questions Raised**

- R-022 should block **new production pub definitions** and **removal of existing TRACE_MATRIX backlinks**. It should not block untouched legacy symbols. Modified legacy pub signatures without backlinks should warn/list debt, not block, until the legacy cleanup atom lands.
- `#[cfg(test)]` pub items should be exempt unless they expose production API. Test-only helpers are not constitutional surface.
- `pub use` should be explicitly out of R-022 v1 unless you want re-export surfaces traced separately. If excluded, say canonical definitions only.
- Macro-generated pub API needs a rule: trace the macro invocation or require a logged skip/orphan. Expanded symbols have no stable source line.
- `external_script` is architecturally cleaner than hard-coding `R-022` in engine.py, but only if SCHEMA, docs, parser, CLI, exit-code handling, and tests are updated. R-022-specific dispatch is brittle.

**Section D: Suggested Patches**

- Amend §0.3 / §1.2 lines 42-44 and 77-109 to include `rules/engine.py`, `rules/SCHEMA.yaml`, `docs/rules.md`, and a tracked `hooks/` or `scripts/hooks/` shim in scope.
- Replace `.git/hooks/pre-commit` with `scripts/hooks/pre-commit.r022` plus `scripts/install_hooks.sh`, and add a CI command such as `scripts/check_trace_matrix.py --mode ci`.
- Rewrite §2.1 lines 123-128: scan staged diffs for new Rust pub definitions, then inspect the immediately preceding doc/attribute block for `/// TRACE_MATRIX`, not 5 raw lines.
- Add an explicit “R-022 Scope Table”: include/exclude `pub(crate)`, `pub use`, `#[cfg(test)]`, macro invocations, `pub type`, `pub static`, and backlink removals.
- Add a real orphan section to TRACE_MATRIX_v3, or change the fallback to the actual section name and table schema.
- Rewrite §3 tests as integration tests: block missing backlink, pass backlink, pass orphan, pass/log skip, exempt cfg(test), ignore/handle pub use, block backlink removal, legacy modified warning, reverse-map idempotency.
- Prefer one parser shared by check and reverse-map generation. Python is the better fit here; bash for §F risks divergent parsing.

**Section E: Forward Sustainability**

The scaffold scripts in §0.4 lines 48-55 are not constitutional surface, but they are load-bearing process tooling. They do not need full dual audit, but they need smoke tests before being trusted. `rehash_trust_root.sh` is closer to constitutional surface because it mutates trust-root material; treat it more strictly.

CO1.13 is infrastructure, not direct progress on the frozen 5-step compile loop. It is justified only if the gate is real. If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater. Ship-with-OBS is acceptable only for bounded edge cases with detection commands and expiry.
2026-04-29T05:22:50.256113Z ERROR codex_core::session: failed to record rollout items: thread 019dd7ad-3396-7f82-b8f1-d45c92e9e68e not found
tokens used
67,736
**Section A: Verdict**

**CHALLENGE / HIGH conviction.** Not VETO because the shape is salvageable, but v1 cannot proceed to implementation as written. The R-022 path is currently non-functional, the hook surface is not reproducible, the orphan fallback points to a non-existent section, and the 5-line heuristic is empirically false for this repo.

**Section B: P0 Blockers**

1. **R-022 cannot run through the current engine.** Spec §1.2 lines 85-99 introduces `check.type: external_script`, but `rules/engine.py` only supports `grep`, `grep_inverse`, and `compound` at lines 50-64. It also has no `--rule R-022` CLI despite spec line 109 saying the shim pipes to engine.py with `--rule R-022`. `rules/SCHEMA.yaml` lines 9-11 also excludes `external_script`.

2. **`.git/hooks/pre-commit` is not a shippable artifact.** Spec lines 79-83, 109, and 196 put the hook in `.git/hooks/pre-commit`; that is untracked local state. It will not protect fresh clones, CI, or merges. If R-022 is meant to prevent untraced merges, add a tracked hook template plus installer and a CI/merge check.

3. **The orphan fallback target is undefined.** Spec §2.1 line 127 says to grep TRACE_MATRIX_v3 “§ 3” for orphan justification. The current matrix has no orphan §3; `§ 3` is only a WP table row. TRACE_MATRIX §F lines 139-151 discusses reverse map, not orphan exceptions.

4. **The 5-line window is not validated and is wrong for current style.** Spec §2.1 lines 123-125 and §5 line 181 rely on 5 lines. I measured current `src/`: for exact `pub fn|struct|enum|trait|const|mod`, only **92/107 = 86%** of traced pre-item doc blocks have `TRACE_MATRIX` within 5 lines; including `pub(crate)` style, **101/120 = 84%**. Use the entire immediately preceding doc/attribute block, not a raw 5-line window.

5. **Escape hatch is a silent bypass.** Spec lines 101, 132, 182, and 217 allow `// R-022-skip: <reason>` with quarterly manual audit only. A hard gate needs structured logging at minimum: symbol, file, line, reason, staged tree hash, timestamp. I would also require `cases/Cxxx`, `PREREG-§`, or `OBS_R022_*`.

6. **Boundary cases are not specified enough to implement.** Spec §2.1 line 123 says NEW pub lines, but does not define `#[cfg(test)]`, `pub use`, macro-generated APIs, `pub(crate)`, `pub type`, or removal of an existing backlink. These must be resolved before coding.

7. **Test plan is mismatched to the implementation.** Spec §3 lines 149-162 names Rust tests for shell/Python/git-hook behavior. Also, `git commit --dry-run` in line 141 is not a reliable proof that the hook blocks. Use shell integration tests or Rust temp-repo tests that invoke the script and one real temp commit.

**Section C: Open Questions Raised**

- R-022 should block **new production pub definitions** and **removal of existing TRACE_MATRIX backlinks**. It should not block untouched legacy symbols. Modified legacy pub signatures without backlinks should warn/list debt, not block, until the legacy cleanup atom lands.
- `#[cfg(test)]` pub items should be exempt unless they expose production API. Test-only helpers are not constitutional surface.
- `pub use` should be explicitly out of R-022 v1 unless you want re-export surfaces traced separately. If excluded, say canonical definitions only.
- Macro-generated pub API needs a rule: trace the macro invocation or require a logged skip/orphan. Expanded symbols have no stable source line.
- `external_script` is architecturally cleaner than hard-coding `R-022` in engine.py, but only if SCHEMA, docs, parser, CLI, exit-code handling, and tests are updated. R-022-specific dispatch is brittle.

**Section D: Suggested Patches**

- Amend §0.3 / §1.2 lines 42-44 and 77-109 to include `rules/engine.py`, `rules/SCHEMA.yaml`, `docs/rules.md`, and a tracked `hooks/` or `scripts/hooks/` shim in scope.
- Replace `.git/hooks/pre-commit` with `scripts/hooks/pre-commit.r022` plus `scripts/install_hooks.sh`, and add a CI command such as `scripts/check_trace_matrix.py --mode ci`.
- Rewrite §2.1 lines 123-128: scan staged diffs for new Rust pub definitions, then inspect the immediately preceding doc/attribute block for `/// TRACE_MATRIX`, not 5 raw lines.
- Add an explicit “R-022 Scope Table”: include/exclude `pub(crate)`, `pub use`, `#[cfg(test)]`, macro invocations, `pub type`, `pub static`, and backlink removals.
- Add a real orphan section to TRACE_MATRIX_v3, or change the fallback to the actual section name and table schema.
- Rewrite §3 tests as integration tests: block missing backlink, pass backlink, pass orphan, pass/log skip, exempt cfg(test), ignore/handle pub use, block backlink removal, legacy modified warning, reverse-map idempotency.
- Prefer one parser shared by check and reverse-map generation. Python is the better fit here; bash for §F risks divergent parsing.

**Section E: Forward Sustainability**

The scaffold scripts in §0.4 lines 48-55 are not constitutional surface, but they are load-bearing process tooling. They do not need full dual audit, but they need smoke tests before being trusted. `rehash_trust_root.sh` is closer to constitutional surface because it mutates trust-root material; treat it more strictly.

CO1.13 is infrastructure, not direct progress on the frozen 5-step compile loop. It is justified only if the gate is real. If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater. Ship-with-OBS is acceptable only for bounded edge cases with detection commands and expiry.
