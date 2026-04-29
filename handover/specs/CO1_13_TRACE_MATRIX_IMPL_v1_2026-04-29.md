# CO1.13: TRACE_MATRIX_v3 Implementation + R-022 Hook v1.1 ⏳ pre-audit (round-2 pending)

**Status**: v1.1 (2026-04-29; round-1 = **CHALLENGE / CHALLENGE** per `CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md` + `GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md`. 9 patches applied per merged verdict. Per Elon-mode 2-round-cap: this is the FINAL patch round before r2 ship-or-ship-with-OBS decision; per Codex r1 § E warning: "If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater" — R-022 itself MUST be enforcing at r2 or escalate to user). Wave 6 #2 PRE-CO1.8 per Elon-mode ROI analysis. User auto-execute mode authorization 2026-04-29.

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

### 0.3 What this atom delivers (3 sub-atoms per sprint graph line 129; v1.1 expanded scope)

| Sub-atom | Deliverable | LoC est | Cycle time target |
|---|---|---|---|
| **CO1.13.1** | TRACE_MATRIX_v3 doc completion: § A complete N-rows; § B complete WP rows; § E coverage stats; § F reverse-map populated for all shipped atoms (CO1.0a / CO1.4 / CO1.4-extra / CO1.7 / CO1.7-impl A1-A4 / CO1.7-extra); **NEW v1.1: § G real orphan section with table schema** (per Codex r1 P0-3: spec § 2.1 fallback target was undefined); **NEW v1.1: reconcile § F line 149 cross-references** (rename script per spec § 1.2 + drop stale "CO P0.8" attribution → "CO1.13.2") | ~200 LoC docs delta | 0.5 day |
| **CO1.13.2** | R-022 commit-time hook (v1.1: SHIPPABLE TRACKED FORM per Codex r1 P0-2): `rules/active/R-022_trace_matrix_pub_symbol_block.yaml` (declarative tombstone — engine.py BYPASSED for this rule per Gemini P0-G1) + `scripts/check_trace_matrix.py` (multi-line context grep + diff parser) + **NEW v1.1: `scripts/hooks/pre-commit.r022` tracked shim** (NOT `.git/hooks/pre-commit` which is local-only per Codex P0-2) + **NEW v1.1: `scripts/install_hooks.sh`** (installer that symlinks tracked shim → `.git/hooks/pre-commit`) + **NEW v1.1: CI command `scripts/check_trace_matrix.py --mode ci`** (catches PRs where install_hooks.sh wasn't run; protects fresh clones + merges) + **NEW v1.1: engine.py 5-line patch** to gracefully ignore rules where `trigger == pre_commit` | ~250 LoC (script) + ~30 LoC (yaml) + ~30 LoC (tracked shim + installer + engine.py patch) = ~310 LoC | 1.5 day |
| **CO1.13.3** | reverse-map § F population workflow: `scripts/update_trace_matrix_reverse_map.sh` → **v1.1: REWRITTEN as Python `scripts/update_trace_matrix_reverse_map.py`** (per Codex r1 § D suggestion: "one parser shared by check and reverse-map generation. Python is the better fit") sharing parser with CO1.13.2 check_trace_matrix.py; idempotent re-population from src/* doc-comments; CI hook calls it; first-run populates from current src/* HEAD | ~100 LoC Python (shares parser module with CO1.13.2) | 0.5 day |

**Total v1.1**: ~640 LoC (was ~415 in v1; expansion driven by P0-2 tracked-hook surface + P0-7 integration tests + Python rewrite); **3-day target wall-clock** (revised up from 2 day per Elon-mode honest cycle-time projection — this v1.1 first real test).

### 0.4 Out of scope (devtools — landed alongside, no spec gate)

These are NOT in the audited spec scope but ship in the same git working tree (separate commits):
1. `scripts/scaffold_co_spec.sh` — generate spec template from atom-id + fc-anchor (saves ~30 min/atom; not constitutional)
2. `scripts/scaffold_audit_launcher.sh` — generate codex+gemini round-N launcher pair (saves ~20 min/round; not constitutional)
3. `scripts/rehash_trust_root.sh` — auto-rehash Trust Root manifest for changed src/* files (saves ~10 min/atom; runs cargo test boot::verify_trust_root post-rehash). **Per Codex r1 § E (v1.1)**: this script is "closer to constitutional surface because it mutates trust-root material; treat it more strictly" — landing requires smoke test demonstrating the rehash is byte-identical to manual rehash output.

These are pure devtools (except #3 which has elevated discipline per Codex r1 § E); no constitutional surface; no PASS/PASS gate required. They land as a single follow-up commit "CO1.13-devtools" after CO1.13 PASS/PASS.

### 0.5 Elon-mode policy refinements (NEW v1.1; per Gemini r1 Q4 + Codex r1 § E)

The v1 spec invoked Elon-mode 2-round-cap with ship-with-OBS as the r2-still-CHALLENGE escape. v1.1 refines this based on round-1 audit findings:

1. **OBS accumulation hard threshold** (Gemini r1 Q4): max 3 unresolved `OBS_*.md` files at any time. Threshold breach = factory halt + force-resolve before next atom. This prevents 2-round-cap from accumulating debt indefinitely.

2. **Ship-with-OBS NOT applicable to R-022 gate itself** (Codex r1 § E): "If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater. Ship-with-OBS is acceptable only for bounded edge cases with detection commands and expiry." → If R-022 doesn't actually enforce by r2, escalate to user — do NOT auto-ship-with-OBS.

3. **CO1.13-extra (legacy backlink closure) sequencing** (Gemini r1 Q7): "75% legacy gap becomes critical at Phase D (ArchitectAI). CO1.13-extra MUST be scheduled before Phase D begins." Currently Phase C unfreeze ~5-7 weeks out (TFR S3.9); Phase D is downstream of Phase C. → CO1.13-extra acceptable as ~2-3 weeks-out follow-up; NOT optional indefinite-defer.

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

### 1.2 CO1.13.2 — R-022 commit-time hook (v1.1: ARCHITECTURALLY CORRECTED per Codex r1 P0-1+P0-2 + Gemini r1 P0-G1)

**v1 architectural mismatch** (round-1 P0): the v1 plan piped through `engine.py --rule R-022`, but `engine.py` only supports `grep` / `grep_inverse` / `compound` check types and is per-file (not diff-aware) per pre-edit `judge.sh` invocation. R-022 needs cross-file diff awareness which engine.py doesn't provide. v1 also placed the hook at `.git/hooks/pre-commit` which is untracked local state — doesn't protect fresh clones, CI, or merges (Codex P0-2).

**v1.1 corrected architecture**: pre-commit shim calls `scripts/check_trace_matrix.py` DIRECTLY (engine.py BYPASSED for R-022). YAML rule becomes a declarative tombstone for the 30-rule cap + documentation. Tracked shim under `scripts/hooks/pre-commit.r022` + `scripts/install_hooks.sh` symlink installer + CI command for fresh-clone / merge protection.

Five pieces:

```
rules/active/R-022_trace_matrix_pub_symbol_block.yaml    # declarative tombstone (engine.py bypassed)
scripts/check_trace_matrix.py                             # core logic; shared by pre-commit + CI + reverse-map
scripts/hooks/pre-commit.r022                             # tracked shim (NOT .git/hooks/...)
scripts/install_hooks.sh                                  # symlinks tracked shim → .git/hooks/pre-commit
rules/engine.py                                           # 5-line patch: gracefully ignore trigger==pre_commit rules
```

YAML rule (declarative tombstone; engine.py ignores rules with `trigger: "pre_commit"` per v1.1 engine.py patch):
```yaml
id: "R-022"
name: "trace_matrix_pub_symbol_block"
source_incidents:
  - "F-2026-04-25-04"           # B7 alignment retroactive fix
  - "feedback_fc_first_problem_handling"  # FC-trace required in commit msg
  - "CO1_13_DUAL_AUDIT_R1_2026-04-29"     # this audit cycle (architectural correction)
fc_trace: "CLAUDE.md Alignment Standard — every NEW src/ pub symbol must have TRACE_MATRIX backlink AT COMMIT TIME (not retroactive)"
axiom: "every NEW pub fn/struct/enum/trait/const/mod added under src/ in this commit must have /// TRACE_MATRIX <id>: <role> doc-comment in the immediately preceding contiguous doc/attribute/comment/blank-line block, OR be filed in TRACE_MATRIX_v3.md § G (orphan extensions) with explicit constitutional justification"
trigger: "pre_commit"
check:
  type: "custom_commit_hook"     # NEW v1.1: declarative; engine.py ignores; shim invokes script directly
  script: "scripts/check_trace_matrix.py"
  invocation_note: "engine.py BYPASSED for this rule because R-022 requires cross-file diff awareness which engine.py per-file architecture does not provide"
file_glob: "*.rs"
enforcement: "block"
message: "BLOCK (R-022 / Alignment Standard): NEW pub symbol(s) added under src/ without TRACE_MATRIX backlink (or REMOVAL of an existing backlink). See script output for specific locations. Either (a) add /// TRACE_MATRIX <FC-id>: <role> doc-comment in the immediately preceding doc/attribute block of each new pub symbol, (b) file in handover/alignment/TRACE_MATRIX_v3.md § G with orphan justification (cases/Cxxx | PREREG-§n.m | OBS_R022_*.md required), or (c) include `[R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]` token in the commit message."
stats:
  times_triggered: 0
  last_triggered: ""
```

`scripts/check_trace_matrix.py` (~250 LoC v1.1; was ~120 in v1):
- Mode `--mode commit`: reads `git diff --cached`, identifies NEW pub items added in this commit vs base; for each, walks **immediately preceding contiguous doc/attribute/comment/blank-line block** (NOT raw 5-line window per Codex P0-4 empirical finding 86%/107 vs 92%/107 = semantic block walk wins) for `/// TRACE_MATRIX `; ALSO detects removals of existing TRACE_MATRIX lines (per Codex § C: R-022 should block removal too); falls back to TRACE_MATRIX_v3 § G orphan section lookup; falls back to commit-message `[R-022-skip: ...]` token (REQUIRES `cases/Cxxx | PREREG-§n.m | OBS_R022_*` reference per Codex P0-5 — silent skip rejected); structured logs every block/skip event to `rules/enforcement.log` with symbol/file/line/reason/staged-tree-hash/timestamp; exits 2 on any unjustified violation.
- Mode `--mode ci`: same logic but operates on PR diff (HEAD..origin/main) not staged diff. Catches PRs where `install_hooks.sh` wasn't run + protects merges into main.
- Mode `--mode reverse-map`: shared parser used by CO1.13.3 to populate § F.

`scripts/hooks/pre-commit.r022` (~15 LoC tracked shim) — invoked by `.git/hooks/pre-commit` (which is itself a symlink installed by `install_hooks.sh`); execs `python3 scripts/check_trace_matrix.py --mode commit`; passes through exit code.

`scripts/install_hooks.sh` (~15 LoC) — idempotent: removes existing `.git/hooks/pre-commit` if present (warns if non-symlink); creates symlink `.git/hooks/pre-commit -> ../../scripts/hooks/pre-commit.r022`. Runs as part of dev-setup; documented in CLAUDE.md.

`rules/engine.py` (5-line patch v1.1): in main rule iteration loop, add early-continue `if rule.get('trigger') == 'pre_commit': continue` — engine.py is pre-edit only; pre_commit-triggered rules bypass entirely. SCHEMA.yaml updated to document `trigger: "pre_commit"` valid value.

### 1.3 R-022 Scope Table (NEW v1.1; per Codex r1 P0-6 boundary cases)

Explicit policy for every Rust pub-style construct:

| Pub style | R-022 enforcement | Rationale |
|---|---|---|
| `pub fn / struct / enum / trait / const / mod` | **BLOCK if missing** (forward-only on NEW) | Canonical constitutional surface. |
| `pub(crate) <kind>` | **BLOCK if missing** (forward-only on NEW) | Crate-internal but still load-bearing for FC alignment within crate. Same 75% gap problem applies. |
| `pub use <path>` | **EXEMPT** (canonical definitions only) | Re-exports inherit alignment from canonical definition; double-trace is redundant. CO1.13-extra may revisit if re-export surface needs separate attribution. |
| `#[cfg(test)] mod tests { pub fn ... }` | **EXEMPT** | Test-only helpers; not constitutional surface. (Codex § C: "Test-only helpers are not constitutional surface".) |
| `pub type <alias>` | **BLOCK if missing** (forward-only on NEW) | Type aliases ARE constitutional (e.g. `pub type AgentId = String`). |
| `pub static <NAME>` | **BLOCK if missing** (forward-only on NEW) | Static state is constitutional. |
| Macro-generated pub items | **EXEMPT** if trace at macro-invocation site OR via `[R-022-skip: macro-generated]` commit-msg token | No stable source line for expansion; trace at the invocation. |
| Modified existing pub signature without backlink | **WARN** (legacy debt; no block) | Codex § C: "Modified legacy pub signatures without backlinks should warn/list debt, not block, until the legacy cleanup atom lands". CO1.13-extra closes legacy gap. |
| **Removal of existing TRACE_MATRIX line** | **BLOCK** unless commit-msg has `[R-022-skip: ...]` with justification | Codex § C: "R-022 should block ... removal of existing TRACE_MATRIX backlinks". |

### 1.3 CO1.13.3 — Reverse-map § F population

`scripts/update_trace_matrix_reverse_map.sh` walks `src/*.rs`, extracts every `/// TRACE_MATRIX <id>: <role>` doc-comment + the immediately-following pub line, formats as `| <pub_symbol> | <id> | <role> |`, writes to TRACE_MATRIX_v3.md § F (idempotent — replaces section content). First run populates from current HEAD; subsequent runs (e.g., post-CO1.8 land) refresh.

Optional: CI cron job runs it weekly + opens PR if section content drifts. Out of v1 scope; manual run is fine for now.

---

## § 2 Implementation contract

### 2.1 R-022 enforcement boundary (v1.1: semantic block walk per Codex r1 P0-4 empirical finding)

R-022 fires on `pre_commit` when `git diff --cached` shows NEW pub-style declarations under `src/` (per § 1.3 R-022 Scope Table: `pub fn`/`struct`/`enum`/`trait`/`const`/`mod`/`type`/`static`, `pub(crate)` variants; exempts `pub use`, `#[cfg(test)]`, macro-generated). For each new pub line, `check_trace_matrix.py --mode commit`:

1. **Walks the immediately preceding contiguous doc/attribute/comment/blank-line block** above the pub line. Algorithm: starting at `pub_line - 1`, walk back while line matches `///` OR `#[` OR `//` OR is empty (whitespace only); stop on first non-doc/non-attr/non-comment/non-blank line. (This is the SEMANTIC algorithm; Codex r1 empirical: 86% of currently-traced symbols are within this block, vs 69.4% under raw 5-line heuristic. Far-distance examples include `transition_ledger.rs:149 to_signing_payload` at dist=50 — all valid existing backlinks separated by long doc-comment blocks.)
2. **Searches the walked block for `/// TRACE_MATRIX `**. If found → PASS for this symbol.
3. **If not found**: search `TRACE_MATRIX_v3.md § G` (NEW v1.1 orphan section per CO1.13.1 — Codex r1 P0-3: v1 fallback target "§ 3" was undefined) for `<file_path>:<symbol_name>` orphan entry; if found with justification (cases/Cxxx | PREREG-§ | OBS_R022_*), PASS.
4. **If still not found**: search the staged commit message for `[R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]` token (per § 2.2 v1.1); if found AND reason has a valid justification reference, PASS-with-LOG.
5. **Else**: BLOCK with structured log entry (symbol, file, line, reason, staged tree hash, timestamp) appended to `rules/enforcement.log`.

**Removal detection** (NEW v1.1; per Codex r1 § C "R-022 should block ... removal of existing TRACE_MATRIX backlinks"): script also detects `git diff --cached` deletions of `/// TRACE_MATRIX ` lines under `src/`. Each removal is BLOCKED unless commit-message has `[R-022-skip: ...]` token with valid justification. This prevents silent backlink decay.

### 2.2 R-022 escape hatch (v1.1: COMMIT-MESSAGE TOKEN + STRUCTURED LOG per Gemini r1 P0-G2 + Codex r1 P0-5)

**v1 silent-bypass risk**: `// R-022-skip: <reason>` (Rust code comment) was permissive (no required justification reference; quarterly audit only). Codex r1 P0-5: "A hard gate needs structured logging at minimum: symbol, file, line, reason, staged tree hash, timestamp. I would also require cases/Cxxx, PREREG-§, or OBS_R022_*."

**v1.1 escape hatch**: `[R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]` token in the COMMIT MESSAGE (NOT a Rust code comment, per Gemini P0-G2: "prevents polluting Rust source code with temporary bypass comments that will inevitably be forgotten"). Structured logging mandatory:

```
2026-04-29T12:34:56Z R-022-SKIP commit=<staged_tree_hash> file=src/foo.rs line=42 symbol=pub_fn_foo reason="cases/C-xxx <description>"
```

Skip rejected (BLOCK) if:
- No `[R-022-skip: ...]` token in commit message
- Token present but `<reason>` lacks `cases/Cxxx | PREREG-§n.m | OBS_R022_*` reference
- Token present but cited reference doesn't exist in repo (`cases/C-xxx.yaml` file missing, etc.)

Quarterly audit reviews accumulated R-022-SKIP log entries; flags suspicious patterns.

### 2.3 Atomicity (UNCHANGED from v1)

R-022 is invoked at `git commit` pre-commit time, before the commit object is created. If BLOCK fires, the commit is aborted (exit 2) and the working tree state is preserved (developer can amend). No atomicity concerns since git's commit-or-abort is atomic.

### 2.4 Invariants (v1.1 expanded)

| Invariant | Statement | Test |
|---|---|---|
| **I-FORWARD** | R-022 triggers on NEW pub symbols only; legacy untouched pub symbols exempt; legacy MODIFIED pub symbols WARN (not block) until CO1.13-extra | `tests/r_022_no_legacy_block.sh` (NEW v1.1: shell test, not Rust) |
| **I-REMOVAL** | R-022 also blocks REMOVAL of existing TRACE_MATRIX backlinks unless escape-hatch justified | `tests/r_022_blocks_backlink_removal.sh` (NEW v1.1) |
| **I-SCOPE** | R-022 scope table (§ 1.3) honored: pub use exempt, cfg(test) exempt, macro-generated exempt-with-token | `tests/r_022_scope_table.sh` (NEW v1.1) |
| **I-DOC** | TRACE_MATRIX_v3 § F reverse-map is auto-generated; no manual edits (overwritten on next run) | `scripts/update_trace_matrix_reverse_map.py --dry-run` produces stable output |
| **I-LIST** | Active rules count ≤ 30 (docs/rules.md cap); CO1.13.2 lands R-022 as 16th rule (within cap) | `ls rules/active/*.yaml \| wc -l` |
| **I-ENFORCE** | R-022 enforcement actually blocks; integration test demonstrates a real temp-repo commit with missing backlink fails pre-commit | `tests/r_022_blocks_missing_backlink.sh` (NEW v1.1: shell integration test, not Rust per Codex P0-7) |
| **I-LOG** | Every R-022 event (block, skip, pass) appends structured log entry to rules/enforcement.log | `tests/r_022_structured_log.sh` (NEW v1.1) |
| **I-CI** | CI command (`scripts/check_trace_matrix.py --mode ci`) catches PRs that bypass install_hooks.sh | `tests/r_022_ci_mode_catches_unhooked_pr.sh` (NEW v1.1) |

### 2.5 Form vs Substance two-layer model (NEW v1.1; per Gemini r1 Q5)

R-022 enforces **FORM**: every new pub symbol has a `/// TRACE_MATRIX <id>: <role>` doc-comment in the preceding block. R-022 is a SYNTACTIC check; it cannot verify whether the backlink correctly assigns the symbol to its true FC layer (e.g., a top-white symbol incorrectly tagged FC3 would pass R-022).

TRACE_MATRIX_v3 § F (reverse-map, auto-generated by CO1.13.3) provides **SUBSTANCE** review: every shipped symbol → its claimed TRACE_MATRIX row → human/AI reviewer can verify alignment correctness. § F is the "eventual consistency" loop — R-022 enforces form at commit time; § F enables substance review post-merge.

This two-layer model is acknowledged as a known limitation: R-022 alone cannot prevent constitutionally misaligned backlinks. Mitigations:
- Per-atom dual-audit (CLAUDE.md "Audit Standard") catches misalignment in code review
- § F reverse-map enables periodic alignment audits
- Future CO1.13-extra-extra (NOT v1; deferred): semantic alignment checker that consumes TRACE_MATRIX_v3 N-row code-symbol column and validates backlinks match expected FC layer

---

## § 3 Test plan (v1.1: rewritten as shell integration tests per Codex r1 P0-7)

**v1 mismatch**: v1 tests were Rust files (`r_022_*.rs`) but Rust unit tests can't easily invoke Python scripts + create temp git repos + drive real commits. Codex r1 P0-7: "Use shell integration tests or Rust temp-repo tests that invoke the script and one real temp commit."

**v1.1 test plan**: 9 shell integration tests (under `tests/integration/co1_13/`) + 1 Rust temp-repo orchestration test for cargo-test-runnable coverage.

### 3.1 `tests/integration/co1_13/r_022_blocks_missing_backlink.sh`
Creates temp git repo; copies minimal `scripts/check_trace_matrix.py` + `scripts/hooks/pre-commit.r022` + `scripts/install_hooks.sh`; runs installer; stages `src/test_module.rs` containing a `pub fn` without backlink; runs `git commit`; asserts exit code 2 + structured log entry in temp `enforcement.log`.

### 3.2 `tests/integration/co1_13/r_022_passes_traced_pub.sh`
Same setup; `pub fn` PRECEDED by `/// TRACE_MATRIX FC3-Nx: test`; asserts commit succeeds + log entry shows PASS.

### 3.3 `tests/integration/co1_13/r_022_orphan_justification_passes.sh`
Same setup; adds row to temp `TRACE_MATRIX_v3.md § G` with `cases/C-test.yaml` justification; pub line has no backlink; asserts commit succeeds via § G fallback.

### 3.4 `tests/integration/co1_13/r_022_skip_token_with_justification.sh`
Same setup; commit message contains `[R-022-skip: cases/C-test refactor cleanup]`; asserts commit succeeds + structured log entry shows SKIP with justification.

### 3.5 `tests/integration/co1_13/r_022_skip_token_without_justification_blocks.sh`
Same setup; commit message contains `[R-022-skip: <empty>]` or skip token without `cases/* | PREREG-§ | OBS_R022_*` reference; asserts BLOCK + log entry.

### 3.6 `tests/integration/co1_13/r_022_blocks_backlink_removal.sh`
Setup: existing file with `/// TRACE_MATRIX FC3-Nx: foo` + `pub fn foo`; commit removes the TRACE_MATRIX line; asserts BLOCK unless skip-token justified.

### 3.7 `tests/integration/co1_13/r_022_scope_table.sh`
Multi-test bundle covering scope-table policy: cfg(test) pub items don't block, pub use re-exports don't block, macro-generated marked via skip-token passes, pub(crate) DOES block, pub type/pub static DO block.

### 3.8 `tests/integration/co1_13/r_022_no_legacy_block.sh`
Pre-stages files with legacy untraced pub symbols at HEAD; subsequent commit modifies an unrelated file; asserts no spurious R-022 block from legacy.

### 3.9 `tests/integration/co1_13/r_022_ci_mode_catches_unhooked_pr.sh`
Setup: temp repo WITHOUT install_hooks.sh run; PR-style branch with bad commit pushed; runs `scripts/check_trace_matrix.py --mode ci`; asserts non-zero exit + structured log identifying the bad commit.

### 3.10 `tests/r_022_integration_orchestrator.rs` (Rust harness)
Rust integration test that calls `Command::new("bash").arg("tests/integration/co1_13/run_all.sh")` to invoke all 9 shell integration tests; asserts all PASS. Provides `cargo test` discoverability for the shell integration suite.

**Coverage matrix**:

| Test ID | Codex Q1 edge case | Codex P0 covered |
|---|---|---|
| 3.1 | NEW pub no backlink | P0-1, I-ENFORCE |
| 3.2 | NEW pub w/ backlink | I-ENFORCE |
| 3.3 | Orphan § G fallback | P0-3 |
| 3.4 | Skip-token w/ justification | P0-5 |
| 3.5 | Skip-token w/o justification | P0-5 |
| 3.6 | Backlink REMOVAL | Codex § C insight |
| 3.7 | Scope table | P0-6 |
| 3.8 | Legacy non-block | I-FORWARD |
| 3.9 | CI mode | P0-2 |
| 3.10 | Rust discoverability | P0-7 |

---

## § 4 Out of scope (deferred per Anti-Oreo three-layer boundary)

1. **Legacy backlink gap closure** (the ~250 untraced legacy pub symbols): a separate CO1.13-extra atom; ~10-15 hr; ships as bulk doc-comment patch.
2. **Reverse-map CI cron**: manual run is sufficient for v1; CI integration is a CO1.13-extra concern.
3. **R-015 stats-not-propagating bug**: filed as separate OBS; orthogonal to R-022.
4. **80-conformance-test population**: TRACE_MATRIX_v3 § H lists ~80 target test files; populating is per-atom work (each Plan v3.2 atom ships its conformance test). CO1.13 only verifies the LIST is complete, not the tests themselves.
5. **Generic scaffold scripts** (§ 0.4): non-constitutional devtools; ship in follow-up commit, no audit.

---

## § 5 Open questions (v1.1: ALL RESOLVED by round-1 audit)

| Q | v1 lean | round-1 resolution |
|---|---|---|
| Q1 | NEW only (forward-only) | **CONFIRMED** by Gemini Q1 (with R-015 catching edits) + Codex § C (legacy-modified should warn not block, until CO1.13-extra). § 1.3 R-022 Scope Table codifies. |
| Q2 | 5-line raw window | **REJECTED** by Codex P0-4 empirical: 5-line raw = 69.4%; semantic block walk = 86%. § 2.1 v1.1 uses semantic block walk algorithm. |
| Q3 | Permissive escape hatch (no required justification) | **REJECTED** by Codex P0-5: silent bypass risk. § 2.2 v1.1 requires `cases/Cxxx | PREREG-§n.m | OBS_R022_*.md` reference + structured logging. |
| Q4 | Doc-comments authoritative | **CONFIRMED** by Gemini Q4. § 2.5 v1.1 codifies as form-vs-substance two-layer model. |
| Q5 | Keep R-015 (defense in depth) | **CONFIRMED** by Gemini Q2 (with future-patch note: R-015 should warn on *modified* untraced symbols only, not all pub edits, to prevent alarm fatigue under R-022). v1 ships both unchanged; CO1.13-extra patches R-015. |
| Q6 (new) | engine.py integration via `external_script` | **REJECTED** by Codex P0-1 + Gemini P0-G1. § 1.2 v1.1: engine.py BYPASSED for R-022; pre-commit shim calls script directly. |
| Q7 (new) | `.git/hooks/pre-commit` placement | **REJECTED** by Codex P0-2 (untracked local state). § 1.2 v1.1: tracked `scripts/hooks/pre-commit.r022` + `scripts/install_hooks.sh` + CI mode. |
| Q8 (new) | Orphan fallback target "§ 3" | **REJECTED** by Codex P0-3 (target undefined in current matrix). § 1.1 + § 2.1 v1.1: real "§ G — Orphan Extensions" section in TRACE_MATRIX_v3 with table schema. |
| Q9 (new) | Boundary cases (cfg(test) / pub use / macro) | **CODIFIED** in § 1.3 R-022 Scope Table per Codex P0-6. |
| Q10 (new) | Test plan as Rust unit tests | **REJECTED** by Codex P0-7. § 3 v1.1: 9 shell integration tests + 1 Rust orchestrator. |

---

## § 6 Audit gates (Elon-mode round cap = 2; v1.1 round-1 = CHALLENGE/CHALLENGE)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| 1 (v1) | **CHALLENGE/HIGH** (7 P0s) | **CHALLENGE/HIGH** (2 P0s) | **CHALLENGE/HIGH** | 9 unique fixes synthesized → v1.1 (this revision) |
| 2 (this v1.1) | ⏳ pending | ⏳ pending | TBD | round-2 final |
| 3+ | **CAPPED** | **CAPPED** | — | If r2 still CHALLENGE on R-022 ENFORCEMENT (gate itself non-functional): **escalate to user, NO ship-with-OBS** per Codex r1 § E. If r2 CHALLENGE on bounded edge cases only: ship-with-OBS allowed per § 0.5 hard-threshold policy (max 3 OBS open at once). |

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

**v1.1 (2026-04-29; round-1 CHALLENGE/CHALLENGE → 9 patches synthesized)** — per `CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md` (CHALLENGE/HIGH, 7 P0s) + `GEMINI_CO1_13_ROUND1_AUDIT_2026-04-29.md` (CHALLENGE/HIGH, 2 P0s); conservative merge CHALLENGE per `feedback_dual_audit_conflict`:

- **P1** (Codex P0-1 + Gemini P0-G1): engine.py architectural bypass → § 1.2 reworked. YAML `check.type: external_script` → `custom_commit_hook` (declarative tombstone); pre-commit shim calls `scripts/check_trace_matrix.py` DIRECTLY, NOT via engine.py. engine.py 5-line patch added to gracefully ignore `trigger == pre_commit` rules.
- **P2** (Codex P0-2): `.git/hooks/pre-commit` is local-only, not shippable → § 1.2 splits into tracked `scripts/hooks/pre-commit.r022` + `scripts/install_hooks.sh` + CI mode `scripts/check_trace_matrix.py --mode ci` for fresh-clone / merge protection.
- **P3** (Codex P0-3): orphan fallback "§ 3" undefined in TRACE_MATRIX_v3 → § 1.1 (CO1.13.1 scope) adds NEW § G "Orphan Extensions" with table schema; § 2.1 fallback target updated.
- **P4** (Codex P0-4): 5-line raw heuristic empirically wrong (Codex measured 69.4% under raw; 86% under semantic block walk) → § 2.1 v1.1 uses semantic algorithm: walk back through contiguous `///` / `#[` / `//` / blank lines, stop on first non-doc/attr/comment/blank line.
- **P5** (Codex P0-5 + Gemini P0-G2): silent escape hatch → § 2.2 v1.1: commit-message token `[R-022-skip: <reason; cases/Cxxx | PREREG-§n.m | OBS_R022_*.md REQUIRED>]` (NOT Rust code comment) + structured log entry mandatory + skip rejected if reference missing/invalid.
- **P6** (Codex P0-6): boundary cases unspecified → NEW § 1.3 "R-022 Scope Table" codifies policy for `pub fn/struct/enum/trait/const/mod/type/static`, `pub(crate)`, `pub use`, `#[cfg(test)]`, macro-generated, signature-modified, backlink-removal.
- **P7** (Codex P0-7): test plan implementation mismatch → § 3 v1.1 rewritten as 9 shell integration tests under `tests/integration/co1_13/` + 1 Rust orchestrator for cargo discoverability.
- **P8** (Codex § C): R-022 should also block REMOVAL of existing backlinks → § 2.1 v1.1 adds removal detection in `git diff --cached`; § 2.4 NEW invariant `I-REMOVAL`.
- **P9** (Gemini Q4 + Q7): Elon-mode policy refinement → NEW § 0.5 codifies (a) OBS hard-threshold max 3 open files, (b) ship-with-OBS NOT applicable to R-022 gate itself per Codex r1 § E, (c) CO1.13-extra (legacy backlink closure) MUST schedule before Phase D.

Plus § 2.5 NEW (form vs substance two-layer model per Gemini Q5); § 5 OQs all marked resolved with reference to round-1 source; § 6 round table updated; LoC budget revised from ~415 to ~640 (3-day target vs 2-day v1).

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
