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
