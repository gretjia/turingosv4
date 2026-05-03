OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019ded4a-5693-7ca3-8c86-4fedd677d92a
--------
user
# Codex TB-13 Ship Audit — round-6 (post round-7 R5 fix closure)

**Role**: skeptical adversarial implementer-reviewer. Independent of Gemini ship audit. Class 3 (money/collateral surface).

**Round context**: this is **round-6** in audit numbering. The project's round-7 commit `8efffa8` closes the two NEW Q9/RQ6 sub-challenges your R5 audit raised against the round-6 fix:

- **TB13-Q9/RQ6-PARTIAL-MARKER (R5)**: round-6's `tb_13_scan_lines()` short-circuited to marker-spans-only when ANY marker existed. A marker-bearing file could hide non-marker TB-13 type-use plus `f64`/AMM/price tokens outside marker spans.
- **TB13-Q9-DASHBOARD-FLOOR (R5)**: round-6 removed `src/bin/audit_dashboard.rs` from `FENCE_SCOPE_FLOOR` to dodge a Layer 2 false-positive on its negative-list test fixture. But the false-positive was Layer 2-specific; removal also disabled Layer 1 hard-import scanning on that file.

**Mandate**: per memory `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS) Codex's R5 CHALLENGE outweighed Gemini's R5 PASS. Per memory `feedback_audit_obs_bias`: both qualified for surgical fix, not OBS.

## Round-7 closures (this session) — what to re-verify

```text
8efffa8  TB-13 Atom 6 round-7 — Codex R5 remediation (PARTIAL-MARKER + DASHBOARD-FLOOR)
```

`cargo test --workspace = 794 passed / 0 failed / 150 ignored` (round-6 baseline 792 + 2 new round-7 tests: `tb_13_scan_lines_partial_marker_catches_stealth_type_use` + `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope`).

### PARTIAL-MARKER fix — what to re-verify

`tests/tb_13_legacy_cpmm_forward_fence.rs` `tb_13_scan_lines()` rewritten:

- Marker-files: return marker-spans UNION non-comment lines containing any TB-13 type name (`TB_13_TYPE_NAMES`). Union via `BTreeMap<usize, String>` (deduplicates).
- Unmarked-discovered files: keep round-6 behavior (all non-comment lines).

Re-evaluate the closure:

- Does the union catch your R5 attack (marker-bearing file with stealth `CompleteSetMintTx` + `f64` outside marker spans)? `CompleteSetMintTx` matches TB_13_TYPE_NAMES, line returned, Layer 2 sees `f64`. Confirm.
- Acknowledged residual gap (per code-comment in `tb_13_scan_lines`): a TB-13 helper that uses zero TB-13 type names AND lives outside marker spans. Manual code review remains the fallback. Acceptable, or stronger fix needed?
- Different residual: `f64` on a non-marker non-type-use line in a marker-bearing file (e.g., `let x: f64 = 0.5;` with no TB-13 type name on the line). Round-7 walker would not return it. Acceptable?

### DASHBOARD-FLOOR fix — what to re-verify

Two-tier scope split:

- `effective_fence_scope()` (Layer 1) = `FENCE_SCOPE_FLOOR` ∪ `discover_tb_13_files()`. `audit_dashboard.rs` RESTORED to FLOOR.
- `effective_layer_2_scope()` (NEW) = `discover_tb_13_files()` only. Excludes `audit_dashboard.rs` until it gains TB-13 contributions.
- Layer 2 scans (`legacy_cpm_api_not_imported_by_complete_set` Layer 2 + `no_f64_in_complete_set_or_market_seed`) use `effective_layer_2_scope()`.
- Layer 1 scan (`legacy_cpm_api_not_imported_by_complete_set` Layer 1) uses `effective_fence_scope()`.

Re-evaluate:

- Does Layer 1 still scan `audit_dashboard.rs` for `HARD_BANNED_LEGACY_IMPORTS`? Yes — `effective_fence_scope` includes it via FLOOR. Confirm.
- Does Layer 2 still false-positive on `audit_dashboard.rs:1628-1629`? No — `effective_layer_2_scope` excludes it. Confirm.
- New unit test `audit_dashboard_in_layer_1_scope_but_not_layer_2_scope` asserts the tier-split shape. Sufficient?
- Tradeoff: until `audit_dashboard.rs` gains TB-13 markers or type uses, Layer 2 won't scan it. A contributor could land f64/AMM tokens in `audit_dashboard.rs` without Layer 2 catching them — but TB-13 fence's purpose is "TB-13 contributions don't pull in legacy CPMM"; tokens in non-TB-13-contributing audit_dashboard code are out of TB-13 scope. Is this right?

## Q1-Q8 + Q10-Q13

Round-7 doesn't touch dispatch arms, monetary invariants, share semantics, seed solvency, or smoke evidence. PASS expected. Re-evaluate Q9 (forward-fence) given the R7 changes.

## Specifically scrutinize

**Trust Root**: round-7 modified only `tests/`. No Rust src/ change. No manifest rehash.
**No new pub symbols**: `effective_layer_2_scope` is a private fn in the test file. No R-022 surface.
**Test count drift**: 792 → 794 (+2 from R7).

## Verdict format

```text
## VERDICT: PASS
- conviction: low/medium/high
- recommendation: PROCEED to SHIP
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
- conviction: low/medium/high
- recommendation: FIX-THEN-PROCEED  (if cheap fix exists) or PROCEED-WITH-OBS (if architecturally deferred)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
- conviction: low/medium/high
```

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.

warning: Codex could not find bubblewrap on PATH. Install bubblewrap with your OS package manager. See the sandbox prerequisites: https://developers.openai.com/codex/concepts/sandboxing#prerequisites. Codex will use the vendored bubblewrap in the meantime.
codex
I’ll verify the local audit instructions first, then inspect the round-7 diff and the TB-13 fence tests around the two claimed closures. Because this session is read-only, I’ll also check whether saving the handoff file is possible before treating it as done.
exec
/bin/bash -lc "rg --files -g AGENTS.md -g '"'!target'"'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc pwd in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingosv4

exec
/bin/bash -lc 'git show --stat --oneline --decorate --no-renames 8efffa8' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
8efffa8 (HEAD -> main) TB-13 Atom 6 round-7 — Codex R5 remediation (PARTIAL-MARKER + DASHBOARD-FLOOR)
 tests/tb_13_legacy_cpmm_forward_fence.rs | 185 +++++++++++++++++++++++++------
 1 file changed, 151 insertions(+), 34 deletions(-)

 exited 1 in 123ms:
 succeeded in 136ms:
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md
?? handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.raw
?? handover/audits/GEMINI_TB_13_SHIP_AUDIT_2026-05-03_R5.md
?? handover/audits/run_codex_tb_13_ship_audit_r5.sh
?? handover/audits/run_codex_tb_13_ship_audit_r6.sh
?? handover/audits/run_gemini_tb_13_ship_audit_r5.py
?? handover/audits/run_gemini_tb_13_ship_audit_r6.py
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/

codex
The repository itself did not report an `AGENTS.md`, so I’m checking the parent scopes explicitly before I draw conclusions from the code. I also see existing untracked audit artifacts, including an R6 raw file; I’ll treat them as user-owned and avoid overwriting anything unless I can write the requested final path cleanly.
exec
/bin/bash -lc "test -f /AGENTS.md && sed -n '1,220p' /AGENTS.md" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc 'find /home/zephryj/projects/turingosv4 -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'find /home/zephryj/projects -maxdepth 2 -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/home/zephryj/projects/turingos/AGENTS.md
/home/zephryj/projects/omega_kepler_discovery/AGENTS.md
/home/zephryj/projects/omega/AGENTS.md
/home/zephryj/projects/turingosv2/AGENTS.md
/home/zephryj/projects/turingclaw/AGENTS.md

 succeeded in 344ms:
exec
/bin/bash -lc "test -f /home/AGENTS.md && sed -n '1,220p' /home/AGENTS.md" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "test -f /home/zephryj/AGENTS.md && sed -n '1,220p' /home/zephryj/AGENTS.md" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "test -f /home/zephryj/projects/AGENTS.md && sed -n '1,220p' /home/zephryj/projects/AGENTS.md" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
codex
No in-scope `AGENTS.md` was found for this repo path. I’m moving into the actual fence implementation now: function boundaries first, then the exact line-level behavior of the new scans and tests.
exec
/bin/bash -lc 'rg -n "fn tb_13_scan_lines|fn effective_fence_scope|fn effective_layer_2_scope|legacy_cpm_api_not_imported_by_complete_set|no_f64_in_complete_set_or_market_seed|TB_13_TYPE_NAMES|FENCE_SCOPE_FLOOR|audit_dashboard_in_layer_1_scope_but_not_layer_2_scope|partial_marker" tests/tb_13_legacy_cpmm_forward_fence.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
62:/// `audit_dashboard.rs` in `FENCE_SCOPE_FLOOR` (Layer 1 always scans for
65:const FENCE_SCOPE_FLOOR: &[&str] = &[
191:fn tb_13_scan_lines(source: &str) -> Vec<(usize, String)> {
204:            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
221:/// requires every file in `FENCE_SCOPE_FLOOR` to exist.
239:/// occurrence in `src/` outside `FENCE_SCOPE_FLOOR`.
240:const TB_13_TYPE_NAMES: &[&str] = &[
285:/// (`TB_13_TYPE_NAMES`) on a non-comment line. Codex round-3 RQ6
304:            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
345:/// Used by `legacy_cpm_api_not_imported_by_complete_set` Layer 1
353:fn effective_fence_scope() -> Vec<String> {
355:    for s in FENCE_SCOPE_FLOOR {
365:/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
377:fn effective_layer_2_scope() -> Vec<String> {
399:/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
414:fn legacy_cpm_api_not_imported_by_complete_set() {
471:/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
480:fn no_f64_in_complete_set_or_market_seed() {
663:fn tb_13_scan_lines_handles_marker_and_unmarked_files() {
711:fn tb_13_scan_lines_partial_marker_catches_stealth_type_use() {
744:fn audit_dashboard_in_layer_1_scope_but_not_layer_2_scope() {

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-13 Atom 0.5 — Legacy CPMM forward-fence + label ship-gate.
     2	//!
     3	//! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2;
     4	//! SG-13.0.1 / SG-13.0.2 / SG-13.0.3).
     5	//!
     6	//! These three tests enforce the forward-binding fence that NEW TB-13
     7	//! modules cannot import or reuse the legacy `src/prediction_market.rs`
     8	//! f64 CPMM scaffolding. Architect §4.2 halting triggers:
     9	//!
    10	//!   HALT if new TB-13 code imports legacy prediction_market.rs.
    11	//!   HALT if f64 appears in new CompleteSet / MarketSeed code.
    12	//!   HALT if any AMM / CPMM router function is introduced in TB-13.
    13	//!
    14	//! ## What is "TB-13 code"?
    15	//!
    16	//! A span of Rust source belongs to TB-13 iff it is a contiguous block
    17	//! of non-blank lines whose first non-blank line contains an authoring
    18	//! marker that identifies TB-13 as the contributing tracer-bullet (NOT
    19	//! a forward-reference from an earlier-TB doc-comment to TB-13's future
    20	//! work). Authoring markers:
    21	//!
    22	//!   - `TRACE_MATRIX TB-13 ` (TB-12 convention used by every shipped TB).
    23	//!   - A line that begins with `// TB-13 ` after stripping leading
    24	//!     whitespace + comment markers.
    25	//!   - A line that begins with `//! TB-13 ` (module-level doc).
    26	//!   - A line that begins with `/// TB-13 ` (item-level doc).
    27	//!
    28	//! A span ends at the next blank line OR end-of-file. Cross-references
    29	//! to TB-13 from inside a TB-12 (or earlier) span do NOT pull that span
    30	//! into TB-13 scope — only the *first non-blank line* of a span is
    31	//! checked for the authoring marker.
    32	//!
    33	//! ## File set in scope
    34	//!
    35	//! - `src/state/typed_tx.rs` — TB-13 typed-tx variant additions (Atom 1).
    36	//! - `src/state/q_state.rs` — TB-13 EconomicState extensions (Atom 2).
    37	//! - `src/state/sequencer.rs` — TB-13 dispatch-arm additions (Atom 2).
    38	//! - `src/economy/monetary_invariant.rs` — TB-13 conservation extensions (Atom 3).
    39	//! - `src/bin/audit_dashboard.rs` — TB-13 §14 dashboard rendering (Atom 4).
    40	//!
    41	//! At Atom 0.5 ship time, none of these files contain `TB-13` markers
    42	//! (TB-12 is the latest contributor). The fence passes trivially. As
    43	//! Atom 1..4 land, markers appear and the fence enforces the rule.
    44	
    45	use std::fs;
    46	use std::path::PathBuf;
    47	
    48	/// Statically-listed in-scope source files for the TB-13 forward-fence
    49	/// Layer 1 (unconditional whole-file scan for hard-banned imports).
    50	/// Codex round-2 CHALLENGE remediation 2026-05-03: this list is now
    51	/// a *floor* — `discover_tb_13_files()` walks `src/` for any additional
    52	/// file containing a TB-13 authoring marker and adds it to the
    53	/// effective scope.
    54	///
    55	/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
    56	/// `src/bin/audit_dashboard.rs` was briefly removed from this list in
    57	/// round-6 to dodge a Layer 2 false-positive on its negative-list test
    58	/// fixture (string literals "price_yes" / "price_no" at line 1628-1629).
    59	/// Codex R5 correctly pointed out that removing it from FLOOR also
    60	/// removed it from Layer 1 hard-import scanning — but the false-positive
    61	/// is Layer 2-specific. The right fix is two-tier scope: keep
    62	/// `audit_dashboard.rs` in `FENCE_SCOPE_FLOOR` (Layer 1 always scans for
    63	/// hard-banned imports), but exclude it from Layer 2's
    64	/// effective-discovered scope until it gains TB-13 markers or type uses.
    65	const FENCE_SCOPE_FLOOR: &[&str] = &[
    66	    "src/state/typed_tx.rs",
    67	    "src/state/q_state.rs",
    68	    "src/state/sequencer.rs",
    69	    "src/economy/monetary_invariant.rs",
    70	    "src/bin/audit_dashboard.rs",
    71	    "src/runtime/verify.rs",
    72	];
    73	
    74	/// Tokens forbidden inside any TB-13-marker span (architect §4.2 halting
    75	/// triggers + §4.7 forbidden list).
    76	///
    77	/// Each entry is a literal substring that must NOT appear in TB-13 code.
    78	const FORBIDDEN_LEGACY_TOKENS: &[&str] = &[
    79	    // Direct legacy CPMM imports / type names.
    80	    "prediction_market::",
    81	    "BinaryMarket",
    82	    // Legacy CPMM API method names.
    83	    ".buy_yes(",
    84	    ".buy_no(",
    85	    "open_bounty_market",
    86	    "bounty_market",
    87	    "bounty_lp_seed",
    88	    "bounty_yes_price",
    89	    "resolve_bounty",
    90	    "market_ticker(",
    91	    "market_ticker_full(",
    92	    // f64 in money-path context (see SG-13.0.2 dedicated test for the
    93	    // primary check; this entry catches `f64` in any TB-13-marked span).
    94	    " f64",
    95	    "f64,",
    96	    "f64;",
    97	    "f64)",
    98	    // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
    99	    "MarketOrderTx",
   100	    "MarketTradeTx",
   101	    "MarketBuyTx",
   102	    "MarketSellTx",
   103	    "AMM",
   104	    "CPMM",
   105	    "DPMM",
   106	    "orderbook",
   107	    // Price-as-truth concepts (deferred to TB-14 per §5).
   108	    "price_yes",
   109	    "price_no",
   110	    "PriceIndex",
   111	    "yes_price",
   112	    "no_price",
   113	    "RationalPrice",
   114	];
   115	
   116	fn workspace_root() -> PathBuf {
   117	    let manifest = env!("CARGO_MANIFEST_DIR");
   118	    PathBuf::from(manifest)
   119	}
   120	
   121	/// Returns true if `line` is an authoring marker for TB-13 (i.e., the
   122	/// line declares that the following block is TB-13 code, NOT a forward-
   123	/// reference from an earlier-TB doc-comment to TB-13's future work).
   124	fn is_tb_13_authoring_marker(line: &str) -> bool {
   125	    if line.contains("TRACE_MATRIX TB-13 ") {
   126	        return true;
   127	    }
   128	    let trimmed = line.trim_start();
   129	    let body = trimmed
   130	        .strip_prefix("//! ")
   131	        .or_else(|| trimmed.strip_prefix("/// "))
   132	        .or_else(|| trimmed.strip_prefix("// "))
   133	        .unwrap_or("");
   134	    body.starts_with("TB-13 ")
   135	}
   136	
   137	/// Extract line ranges that belong to TB-13 additions. A span is a
   138	/// contiguous block of non-blank lines; it is in-scope iff the first
   139	/// non-blank line is an authoring marker per `is_tb_13_authoring_marker`.
   140	fn tb_13_spans(source: &str) -> Vec<(usize, String)> {
   141	    let mut out: Vec<(usize, String)> = Vec::new();
   142	    let lines: Vec<&str> = source.lines().collect();
   143	    let n = lines.len();
   144	    let mut i = 0;
   145	    while i < n {
   146	        if lines[i].trim().is_empty() {
   147	            i += 1;
   148	            continue;
   149	        }
   150	        let span_start = i;
   151	        let mut span_end = i;
   152	        while span_end < n && !lines[span_end].trim().is_empty() {
   153	            span_end += 1;
   154	        }
   155	        let span = &lines[span_start..span_end];
   156	        if is_tb_13_authoring_marker(span[0]) {
   157	            for (offset, line) in span.iter().enumerate() {
   158	                out.push((span_start + offset + 1, (*line).to_string()));
   159	            }
   160	        }
   161	        i = span_end;
   162	    }
   163	    out
   164	}
   165	
   166	/// Lines to scan for Layer 2 forbidden tokens.
   167	///
   168	/// **Codex round-4 RQ6 (2026-05-03)**: `tb_13_spans()` returns nothing
   169	/// for files added to scope by `discover_by_type_use` (no marker = no
   170	/// span), so the marker-only Layer 2 missed unmarked TB-13 contributors.
   171	///
   172	/// **Codex round-5 (R5) PARTIAL-MARKER (2026-05-03)**: round-6's
   173	/// either/or rule (marker-file → spans-only; unmarked file → all
   174	/// non-comment lines) left a hole: a marker-bearing file could hide
   175	/// non-marker TB-13 type-use plus f64/AMM tokens outside any marker
   176	/// span. Fix: for marker-files, scan marker-spans UNION any non-comment
   177	/// line that contains a TB-13 type name (catches stealth TB-13 type-uses
   178	/// outside marker spans — those lines ARE TB-13 contributions by
   179	/// definition because they reference TB-13-introduced types).
   180	///
   181	/// Final rules:
   182	/// - Marker-file: marker-spans ∪ non-comment lines containing TB-13 type names.
   183	/// - Unmarked-discovered file: all non-comment lines (round-6 behavior).
   184	///
   185	/// Residual gap (acknowledged): a TB-13 helper that uses zero TB-13 type
   186	/// names AND lives outside marker spans (e.g., a generic math helper
   187	/// called only by TB-13 code). Without a code-marker AND without a
   188	/// type-name signal, the fence has no way to identify it as TB-13. This
   189	/// is a defense-in-depth limit of marker+type-name discipline; manual
   190	/// code review remains the residual halt-trigger guard.
   191	fn tb_13_scan_lines(source: &str) -> Vec<(usize, String)> {
   192	    use std::collections::BTreeMap;
   193	    let has_marker = source.lines().any(is_tb_13_authoring_marker);
   194	    if has_marker {
   195	        // Marker-file: marker-spans ∪ non-comment lines with TB-13 type names.
   196	        let mut acc: BTreeMap<usize, String> = BTreeMap::new();
   197	        for (n, l) in tb_13_spans(source) {
   198	            acc.insert(n, l);
   199	        }
   200	        for (i, line) in source.lines().enumerate() {
   201	            if is_pure_comment_line(line) {
   202	                continue;
   203	            }
   204	            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
   205	                acc.insert(i + 1, line.to_string());
   206	            }
   207	        }
   208	        return acc.into_iter().collect();
   209	    }
   210	    // Unmarked-discovered file: all non-comment lines.
   211	    source
   212	        .lines()
   213	        .enumerate()
   214	        .filter(|(_, line)| !is_pure_comment_line(line))
   215	        .map(|(i, line)| (i + 1, line.to_string()))
   216	        .collect()
   217	}
   218	
   219	/// Read a source file relative to the workspace root, returning its
   220	/// content as a String. Panics with a clear message if missing — fence
   221	/// requires every file in `FENCE_SCOPE_FLOOR` to exist.
   222	fn read_scope_file(rel_path: &str) -> String {
   223	    let full = workspace_root().join(rel_path);
   224	    fs::read_to_string(&full)
   225	        .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
   226	}
   227	
   228	/// TB-13 type names — any non-comment use of one of these in `src/`
   229	/// marks the file as a TB-13 contributor. Codex round-3 RQ6 remediation
   230	/// 2026-05-03: the round-2 marker-only discovery caught files whose
   231	/// authors followed the `TRACE_MATRIX TB-13 ` / `// TB-13 ` convention,
   232	/// but a contributor could `use crate::state::typed_tx::CompleteSetMintTx;`
   233	/// in a fresh file without adding a marker — and the fence would miss
   234	/// it. The type-use walk closes that gap by checking for the distinctive
   235	/// TB-13-introduced symbol set itself; the marker discipline becomes a
   236	/// hint for human readers, not the only line of defense.
   237	///
   238	/// All names below are TB-13-introduced and have no pre-existing
   239	/// occurrence in `src/` outside `FENCE_SCOPE_FLOOR`.
   240	const TB_13_TYPE_NAMES: &[&str] = &[
   241	    "CompleteSetMintTx",
   242	    "CompleteSetRedeemTx",
   243	    "MarketSeedTx",
   244	    "ConditionalCollateralIndex",
   245	    "ConditionalShareBalances",
   246	    "ShareSidePair",
   247	    "EventNotOpen",
   248	    "EventId",
   249	    "OutcomeSide",
   250	    "ShareAmount",
   251	];
   252	
   253	/// True iff `line` is a pure comment (line starts with `//`, `///`, or
   254	/// `//!` after leading whitespace). Used by type-use discovery to skip
   255	/// legacy doc-comment cross-references (e.g., TB-12 `kernel.rs` /// doc
   256	/// strings that mention `CompleteSetMintTx` as future work).
   257	fn is_pure_comment_line(line: &str) -> bool {
   258	    line.trim_start().starts_with("//")
   259	}
   260	

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '260,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	
   261	/// Walk `walk_root` for any `.rs` file containing a TB-13 authoring
   262	/// marker on at least one line. Codex round-2 CHALLENGE remediation
   263	/// 2026-05-03: makes the fence self-extending against marked but
   264	/// not-yet-listed contributors.
   265	fn discover_by_marker(walk_root: &std::path::Path) -> Vec<String> {
   266	    let mut found: Vec<String> = Vec::new();
   267	    walk_rs_files(walk_root, &mut |path| {
   268	        let rel = path
   269	            .strip_prefix(workspace_root())
   270	            .unwrap_or(path)
   271	            .to_string_lossy()
   272	            .into_owned();
   273	        let body = match fs::read_to_string(path) {
   274	            Ok(s) => s,
   275	            Err(_) => return,
   276	        };
   277	        if body.lines().any(is_tb_13_authoring_marker) {
   278	            found.push(rel);
   279	        }
   280	    });
   281	    found
   282	}
   283	
   284	/// Walk `walk_root` for any `.rs` file that USES a TB-13 type name
   285	/// (`TB_13_TYPE_NAMES`) on a non-comment line. Codex round-3 RQ6
   286	/// remediation 2026-05-03: catches contributors who imported TB-13
   287	/// types without remembering the authoring-marker convention.
   288	fn discover_by_type_use(walk_root: &std::path::Path) -> Vec<String> {
   289	    let mut found: Vec<String> = Vec::new();
   290	    walk_rs_files(walk_root, &mut |path| {
   291	        let rel = path
   292	            .strip_prefix(workspace_root())
   293	            .unwrap_or(path)
   294	            .to_string_lossy()
   295	            .into_owned();
   296	        let body = match fs::read_to_string(path) {
   297	            Ok(s) => s,
   298	            Err(_) => return,
   299	        };
   300	        for line in body.lines() {
   301	            if is_pure_comment_line(line) {
   302	                continue;
   303	            }
   304	            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
   305	                found.push(rel);
   306	                return;
   307	            }
   308	        }
   309	    });
   310	    found
   311	}
   312	
   313	/// Discover every TB-13-contributing file in `src/`. Union of
   314	/// marker-walk (round-2) + type-use-walk (round-3 RQ6). Either path
   315	/// alone would leave a loophole; together they enforce the fence even
   316	/// when the human-followed marker convention slips.
   317	fn discover_tb_13_files() -> Vec<String> {
   318	    let src_root = workspace_root().join("src");
   319	    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
   320	    for f in discover_by_marker(&src_root) {
   321	        set.insert(f);
   322	    }
   323	    for f in discover_by_type_use(&src_root) {
   324	        set.insert(f);
   325	    }
   326	    set.into_iter().collect()
   327	}
   328	
   329	fn walk_rs_files(dir: &std::path::Path, visitor: &mut dyn FnMut(&std::path::Path)) {
   330	    let entries = match fs::read_dir(dir) {
   331	        Ok(e) => e,
   332	        Err(_) => return,
   333	    };
   334	    for entry in entries.flatten() {
   335	        let path = entry.path();
   336	        if path.is_dir() {
   337	            walk_rs_files(&path, visitor);
   338	        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
   339	            visitor(&path);
   340	        }
   341	    }
   342	}
   343	
   344	/// Layer 1 fence scope = FLOOR ∪ discovered. Deduplicated, sorted.
   345	/// Used by `legacy_cpm_api_not_imported_by_complete_set` Layer 1
   346	/// (HARD_BANNED_LEGACY_IMPORTS unconditional whole-file scan).
   347	///
   348	/// Layer 1 is broader than Layer 2 because legacy imports are forbidden
   349	/// EVERYWHERE in TB-13-relevant scope, regardless of whether the file
   350	/// carries TB-13 markers or type uses today. `audit_dashboard.rs` lives
   351	/// here because it is TB-13-relevant scope (Atom 4 §13/§14 dashboard
   352	/// renders TB-13 state), even though its current contributions are TB-12.
   353	fn effective_fence_scope() -> Vec<String> {
   354	    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
   355	    for s in FENCE_SCOPE_FLOOR {
   356	        set.insert((*s).to_string());
   357	    }
   358	    for s in discover_tb_13_files() {
   359	        set.insert(s);
   360	    }
   361	    set.into_iter().collect()
   362	}
   363	
   364	/// Layer 2 fence scope = discovered only (marker OR type-use).
   365	/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
   366	///
   367	/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
   368	/// narrower than Layer 1 because Layer 2 tokens (f64 / AMM / orderbook /
   369	/// price names) can legitimately appear in non-TB-13 files for unrelated
   370	/// reasons (e.g., negative-list test fixtures in `audit_dashboard.rs`
   371	/// at line 1628 that BAN those tokens — not USE them). Restricting
   372	/// Layer 2 to discovered files (i.e., files that actually contribute
   373	/// TB-13 code via marker OR TB-13 type use) prevents false positives on
   374	/// non-TB-13 baseline code that happens to mention forbidden token
   375	/// names. `audit_dashboard.rs` will auto-enter this scope when TB-14
   376	/// ships dashboard contributions there.
   377	fn effective_layer_2_scope() -> Vec<String> {
   378	    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
   379	    for s in discover_tb_13_files() {
   380	        set.insert(s);
   381	    }
   382	    set.into_iter().collect()
   383	}
   384	
   385	/// Hard-banned legacy CPMM imports — these strings MUST NOT appear in
   386	/// any FENCE_SCOPE file regardless of TB-13-marker discipline. Codex
   387	/// round-1 Q9 CHALLENGE remediation (2026-05-03): the marker-only fence
   388	/// could be bypassed by writing a legacy import outside a TB-13 doc-
   389	/// comment span. These tokens are unconditionally banned (a `use
   390	/// crate::prediction_market::BinaryMarket` anywhere in scope is an
   391	/// architectural regression even in non-TB-13 sections).
   392	const HARD_BANNED_LEGACY_IMPORTS: &[&str] = &[
   393	    "use crate::prediction_market::",
   394	    "use crate::prediction_market;",
   395	    "crate::prediction_market::BinaryMarket",
   396	    "crate::prediction_market::MarketError",
   397	];
   398	
   399	/// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
   400	///
   401	/// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
   402	/// `prediction_market.rs`. Two layers of enforcement:
   403	///
   404	/// **Layer 1 (unconditional, Codex round-1 Q9 remediation)**: scan every
   405	/// FENCE_SCOPE file for `HARD_BANNED_LEGACY_IMPORTS` regardless of
   406	/// TB-13-marker discipline. Catches any new use-statement or type
   407	/// reference that pulls legacy CPMM into a TB-13-scope module.
   408	///
   409	/// **Layer 2 (TB-13-marker-scoped)**: scan TB-13-marked spans for the
   410	/// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
   411	/// concepts). The marker discipline allows benign references in
   412	/// historical doc-comments while keeping new TB-13 code clean.
   413	#[test]
   414	fn legacy_cpm_api_not_imported_by_complete_set() {
   415	    let mut violations: Vec<String> = Vec::new();
   416	    let scope = effective_fence_scope();
   417	
   418	    // Layer 1: unconditional whole-file scan for hard-banned imports.
   419	    for rel in &scope {
   420	        let source = read_scope_file(rel);
   421	        for (line_no, line) in source.lines().enumerate() {
   422	            for token in HARD_BANNED_LEGACY_IMPORTS {
   423	                if line.contains(token) {
   424	                    violations.push(format!(
   425	                        "{rel}:{}: hard-banned legacy import `{token}` — {line}",
   426	                        line_no + 1
   427	                    ));
   428	                }
   429	            }
   430	        }
   431	    }
   432	
   433	    // Layer 2: scan for trading/AMM concepts. Restricted to discovered
   434	    // (Codex round-5 DASHBOARD-FLOOR remediation 2026-05-03): Layer 2
   435	    // tokens (f64 / AMM / orderbook / price names) can appear legitimately
   436	    // in non-TB-13 baseline code (e.g., negative-list test fixtures); only
   437	    // files that actually contribute TB-13 (via marker OR TB-13 type use)
   438	    // should be Layer-2-scanned. `tb_13_scan_lines` then resolves the
   439	    // PARTIAL-MARKER case: marker-spans ∪ non-marker TB-13-type-use lines.
   440	    let layer_2_scope = effective_layer_2_scope();
   441	    for rel in &layer_2_scope {
   442	        let source = read_scope_file(rel);
   443	        for (line_no, line) in tb_13_scan_lines(&source) {
   444	            for token in FORBIDDEN_LEGACY_TOKENS {
   445	                // The `f64` family entries are checked in SG-13.0.2 — skip
   446	                // them here so the failure message is unambiguous.
   447	                if token.starts_with(" f64")
   448	                    || token.starts_with("f64,")
   449	                    || token.starts_with("f64;")
   450	                    || token.starts_with("f64)")
   451	                {
   452	                    continue;
   453	                }
   454	                if line.contains(token) {
   455	                    violations.push(format!(
   456	                        "{rel}:{line_no}: TB-13-scope contains forbidden token `{token}` — {line}"
   457	                    ));
   458	                }
   459	            }
   460	        }
   461	    }
   462	    assert!(
   463	        violations.is_empty(),
   464	        "TB-13 SG-13.0.1 forward-fence violated (Layer 1 scope: {} files; Layer 2 scope: {} files):\n{}",
   465	        scope.len(),
   466	        layer_2_scope.len(),
   467	        violations.join("\n")
   468	    );
   469	}
   470	
   471	/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
   472	/// effective_layer_2_scope() (discovered-only, per Codex R5 DASHBOARD-FLOOR
   473	/// remediation 2026-05-03 — Layer 2 tokens like f64 can appear in
   474	/// non-TB-13 baseline code for unrelated reasons).
   475	///
   476	/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
   477	/// CompleteSet / MarketSeed code. Money-path types must use integer
   478	/// `MicroCoin` / `ShareAmount`.
   479	#[test]
   480	fn no_f64_in_complete_set_or_market_seed() {
   481	    let mut violations: Vec<String> = Vec::new();
   482	    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
   483	    for rel in &effective_layer_2_scope() {
   484	        let source = read_scope_file(rel);
   485	        // tb_13_scan_lines: marker-files → spans ∪ non-marker TB-13-type-use
   486	        // lines (Codex R5 PARTIAL-MARKER); unmarked-discovered files →
   487	        // all non-comment lines (Codex R4 RQ6).
   488	        for (line_no, line) in tb_13_scan_lines(&source) {
   489	            for token in &f64_tokens {
   490	                if line.contains(token) {
   491	                    violations.push(format!(
   492	                        "{rel}:{line_no}: TB-13-scope contains f64 (`{token}`) — {line}"
   493	                    ));
   494	                }
   495	            }
   496	        }
   497	    }
   498	    assert!(
   499	        violations.is_empty(),
   500	        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
   501	        violations.join("\n")
   502	    );
   503	}
   504	
   505	/// SG-13.0.3 — `prediction_market_legacy_quarantined`.
   506	///
   507	/// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
   508	/// enforce that `src/prediction_market.rs` carries the LEGACY module-
   509	/// header doc-comment with the four required tokens (`legacy`,
   510	/// `not constitutional`, `not RSP-M`, `not production market path`)
   511	/// AND that `src/kernel.rs` market-bearing fields carry the `LEGACY`
   512	/// label tying them to the migration path.
   513	#[test]
   514	fn prediction_market_legacy_quarantined() {
   515	    let pm = read_scope_file("src/prediction_market.rs");
   516	    let header = pm
   517	        .lines()
   518	        .take(60)
   519	        .collect::<Vec<_>>()
   520	        .join("\n");
   521	
   522	    let required_label_tokens = [
   523	        "LEGACY",
   524	        "not constitutional",
   525	        "not RSP-M",
   526	        "not production market path",
   527	    ];
   528	    for token in &required_label_tokens {
   529	        assert!(
   530	            header.contains(token),
   531	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
   532	             label token `{token}`. Header:\n{header}"
   533	        );
   534	    }
   535	
   536	    // Architect §4.2 also requires the doc to name the migration path so
   537	    // future maintainers don't reintroduce the legacy API.
   538	    let migration_tokens = [
   539	        "TB-13",
   540	        "TB-14",
   541	        "CompleteSetMintTx",
   542	        "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
   543	    ];
   544	    for token in &migration_tokens {
   545	        assert!(
   546	            header.contains(token),
   547	            "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
   548	             token `{token}`. Header:\n{header}"
   549	        );
   550	    }
   551	
   552	    // Defense-in-depth: kernel.rs market-bearing fields carry LEGACY.
   553	    let kernel = read_scope_file("src/kernel.rs");
   554	    let kernel_struct_idx = kernel
   555	        .find("pub struct Kernel {")
   556	        .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
   557	    let kernel_struct_end = kernel[kernel_struct_idx..]
   558	        .find("\n}\n")
   559	        .map(|off| kernel_struct_idx + off + 2)
   560	        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");

exec
/bin/bash -lc 'git show --no-ext-diff --unified=80 -- tests/tb_13_legacy_cpmm_forward_fence.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
commit 8efffa85159998a4f100ea7cdcc4e2e87fd9bee3
Author: gretjia <gretjia@users.noreply.github.com>
Date:   Sun May 3 09:57:57 2026 +0000

    TB-13 Atom 6 round-7 — Codex R5 remediation (PARTIAL-MARKER + DASHBOARD-FLOOR)
    
    Codex R5 surfaced two NEW Q9/RQ6 challenges against round-6 (no VETO;
    Gemini R5 PASS again; auditor agent's clean-room architect-directive
    read PASSed independently — both 'tighter-than-spec' deviations
    endorsed). Per feedback_dual_audit_conflict Codex CHALLENGE wins.
    Per feedback_audit_obs_bias both qualify for surgical fix:
    
    **PARTIAL-MARKER** (R5)
    
    tb_13_scan_lines() short-circuited to marker-spans-only when ANY marker
    existed in the file. A marker-bearing file could hide non-marker TB-13
    contributions (stealth type-use + f64 / AMM tokens outside marker
    spans) — Codex's exact attack vector.
    
    Fix: rewrite tb_13_scan_lines so marker-files return marker-spans UNION
    non-comment lines containing TB-13 type names. Catches stealth type-use
    because any line referencing a TB-13-introduced type IS a TB-13
    contribution by definition. Unmarked-discovered files keep the round-6
    behavior (all non-comment lines).
    
    Residual gap (acknowledged in code-comment): a TB-13 helper that uses
    zero TB-13 type names AND lives outside marker spans (e.g., a generic
    math helper called only by TB-13 code). Manual code review remains the
    fallback halt-trigger guard for that pattern.
    
    **DASHBOARD-FLOOR** (R5)
    
    Round-6 removed src/bin/audit_dashboard.rs from FENCE_SCOPE_FLOOR to
    dodge a Layer 2 false-positive on its negative-list test fixture
    (strings 'price_yes' / 'price_no' at line 1628). Codex correctly noted
    this also removed it from Layer 1 hard-import scanning, even though
    the false-positive was Layer 2-specific.
    
    Fix: two-tier scope split.
    - effective_fence_scope() (Layer 1) = FLOOR ∪ discovered.
      audit_dashboard.rs RESTORED to FLOOR — Layer 1 hard-banned-import
      scan always runs (legacy CPMM imports forbidden everywhere in scope).
    - effective_layer_2_scope() (NEW) = discovered only (marker OR type-use).
      Excludes audit_dashboard.rs until it gains TB-13 contributions.
    
    Two new tests:
    - tb_13_scan_lines_partial_marker_catches_stealth_type_use: marker-file
      with stealth CompleteSetMintTx + f64 outside marker spans → returned.
    - audit_dashboard_in_layer_1_scope_but_not_layer_2_scope: asserts the
      tier-split shape directly.
    
    cargo test --workspace = 794 passed / 0 failed / 150 ignored
      (round-6 baseline 792 + 2 new round-7 tests).
    cargo test --test tb_13_legacy_cpmm_forward_fence = 7/7.
    
    Trust Root: no Rust src/ change; only tests/. No manifest rehash.
    No new pub symbols.
    
    FC-trace: FC3-Alignment (forward-fence Layer 2 PARTIAL-MARKER closure +
    DASHBOARD-FLOOR Layer 1 retention).
    
    Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>

diff --git a/tests/tb_13_legacy_cpmm_forward_fence.rs b/tests/tb_13_legacy_cpmm_forward_fence.rs
index 9aa904c..6a16762 100644
--- a/tests/tb_13_legacy_cpmm_forward_fence.rs
+++ b/tests/tb_13_legacy_cpmm_forward_fence.rs
@@ -1,500 +1,567 @@
 //! TB-13 Atom 0.5 — Legacy CPMM forward-fence + label ship-gate.
 //!
 //! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2;
 //! SG-13.0.1 / SG-13.0.2 / SG-13.0.3).
 //!
 //! These three tests enforce the forward-binding fence that NEW TB-13
 //! modules cannot import or reuse the legacy `src/prediction_market.rs`
 //! f64 CPMM scaffolding. Architect §4.2 halting triggers:
 //!
 //!   HALT if new TB-13 code imports legacy prediction_market.rs.
 //!   HALT if f64 appears in new CompleteSet / MarketSeed code.
 //!   HALT if any AMM / CPMM router function is introduced in TB-13.
 //!
 //! ## What is "TB-13 code"?
 //!
 //! A span of Rust source belongs to TB-13 iff it is a contiguous block
 //! of non-blank lines whose first non-blank line contains an authoring
 //! marker that identifies TB-13 as the contributing tracer-bullet (NOT
 //! a forward-reference from an earlier-TB doc-comment to TB-13's future
 //! work). Authoring markers:
 //!
 //!   - `TRACE_MATRIX TB-13 ` (TB-12 convention used by every shipped TB).
 //!   - A line that begins with `// TB-13 ` after stripping leading
 //!     whitespace + comment markers.
 //!   - A line that begins with `//! TB-13 ` (module-level doc).
 //!   - A line that begins with `/// TB-13 ` (item-level doc).
 //!
 //! A span ends at the next blank line OR end-of-file. Cross-references
 //! to TB-13 from inside a TB-12 (or earlier) span do NOT pull that span
 //! into TB-13 scope — only the *first non-blank line* of a span is
 //! checked for the authoring marker.
 //!
 //! ## File set in scope
 //!
 //! - `src/state/typed_tx.rs` — TB-13 typed-tx variant additions (Atom 1).
 //! - `src/state/q_state.rs` — TB-13 EconomicState extensions (Atom 2).
 //! - `src/state/sequencer.rs` — TB-13 dispatch-arm additions (Atom 2).
 //! - `src/economy/monetary_invariant.rs` — TB-13 conservation extensions (Atom 3).
 //! - `src/bin/audit_dashboard.rs` — TB-13 §14 dashboard rendering (Atom 4).
 //!
 //! At Atom 0.5 ship time, none of these files contain `TB-13` markers
 //! (TB-12 is the latest contributor). The fence passes trivially. As
 //! Atom 1..4 land, markers appear and the fence enforces the rule.
 
 use std::fs;
 use std::path::PathBuf;
 
 /// Statically-listed in-scope source files for the TB-13 forward-fence
 /// Layer 1 (unconditional whole-file scan for hard-banned imports).
 /// Codex round-2 CHALLENGE remediation 2026-05-03: this list is now
 /// a *floor* — `discover_tb_13_files()` walks `src/` for any additional
 /// file containing a TB-13 authoring marker and adds it to the
 /// effective scope.
 ///
-/// **Codex round-4 RQ6 follow-up (2026-05-03)**: `src/bin/audit_dashboard.rs`
-/// was previously listed here for forward dashboard coverage but currently
-/// carries 0 TB-13 markers + 0 TB-13 type uses (Atom 4 §13 dashboard
-/// rendering is DEFERRED per charter; consolidated to TB-14 PriceIndex).
-/// Listing it here while my Layer 2 walker now scans non-comment lines on
-/// unmarked files surfaces a false positive on the dashboard's own
-/// negative-list test fixture (line 1628-1629: string literals "price_yes"
-/// / "price_no" in a forbidden-token assertion). Remove from FLOOR; the
-/// file will be auto-rediscovered by `discover_by_marker` when Atom 4
-/// ships TB-13 contributions in TB-14.
+/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
+/// `src/bin/audit_dashboard.rs` was briefly removed from this list in
+/// round-6 to dodge a Layer 2 false-positive on its negative-list test
+/// fixture (string literals "price_yes" / "price_no" at line 1628-1629).
+/// Codex R5 correctly pointed out that removing it from FLOOR also
+/// removed it from Layer 1 hard-import scanning — but the false-positive
+/// is Layer 2-specific. The right fix is two-tier scope: keep
+/// `audit_dashboard.rs` in `FENCE_SCOPE_FLOOR` (Layer 1 always scans for
+/// hard-banned imports), but exclude it from Layer 2's
+/// effective-discovered scope until it gains TB-13 markers or type uses.
 const FENCE_SCOPE_FLOOR: &[&str] = &[
     "src/state/typed_tx.rs",
     "src/state/q_state.rs",
     "src/state/sequencer.rs",
     "src/economy/monetary_invariant.rs",
+    "src/bin/audit_dashboard.rs",
     "src/runtime/verify.rs",
 ];
 
 /// Tokens forbidden inside any TB-13-marker span (architect §4.2 halting
 /// triggers + §4.7 forbidden list).
 ///
 /// Each entry is a literal substring that must NOT appear in TB-13 code.
 const FORBIDDEN_LEGACY_TOKENS: &[&str] = &[
     // Direct legacy CPMM imports / type names.
     "prediction_market::",
     "BinaryMarket",
     // Legacy CPMM API method names.
     ".buy_yes(",
     ".buy_no(",
     "open_bounty_market",
     "bounty_market",
     "bounty_lp_seed",
     "bounty_yes_price",
     "resolve_bounty",
     "market_ticker(",
     "market_ticker_full(",
     // f64 in money-path context (see SG-13.0.2 dedicated test for the
     // primary check; this entry catches `f64` in any TB-13-marked span).
     " f64",
     "f64,",
     "f64;",
     "f64)",
     // Trading / AMM / orderbook concepts forbidden in TB-13 (per §4.7).
     "MarketOrderTx",
     "MarketTradeTx",
     "MarketBuyTx",
     "MarketSellTx",
     "AMM",
     "CPMM",
     "DPMM",
     "orderbook",
     // Price-as-truth concepts (deferred to TB-14 per §5).
     "price_yes",
     "price_no",
     "PriceIndex",
     "yes_price",
     "no_price",
     "RationalPrice",
 ];
 
 fn workspace_root() -> PathBuf {
     let manifest = env!("CARGO_MANIFEST_DIR");
     PathBuf::from(manifest)
 }
 
 /// Returns true if `line` is an authoring marker for TB-13 (i.e., the
 /// line declares that the following block is TB-13 code, NOT a forward-
 /// reference from an earlier-TB doc-comment to TB-13's future work).
 fn is_tb_13_authoring_marker(line: &str) -> bool {
     if line.contains("TRACE_MATRIX TB-13 ") {
         return true;
     }
     let trimmed = line.trim_start();
     let body = trimmed
         .strip_prefix("//! ")
         .or_else(|| trimmed.strip_prefix("/// "))
         .or_else(|| trimmed.strip_prefix("// "))
         .unwrap_or("");
     body.starts_with("TB-13 ")
 }
 
 /// Extract line ranges that belong to TB-13 additions. A span is a
 /// contiguous block of non-blank lines; it is in-scope iff the first
 /// non-blank line is an authoring marker per `is_tb_13_authoring_marker`.
 fn tb_13_spans(source: &str) -> Vec<(usize, String)> {
     let mut out: Vec<(usize, String)> = Vec::new();
     let lines: Vec<&str> = source.lines().collect();
     let n = lines.len();
     let mut i = 0;
     while i < n {
         if lines[i].trim().is_empty() {
             i += 1;
             continue;
         }
         let span_start = i;
         let mut span_end = i;
         while span_end < n && !lines[span_end].trim().is_empty() {
             span_end += 1;
         }
         let span = &lines[span_start..span_end];
         if is_tb_13_authoring_marker(span[0]) {
             for (offset, line) in span.iter().enumerate() {
                 out.push((span_start + offset + 1, (*line).to_string()));
             }
         }
         i = span_end;
     }
     out
 }
 
-/// Lines to scan for forbidden tokens (Layer 2). Codex round-4 RQ6
-/// remediation 2026-05-03: `tb_13_spans()` returns nothing for files
-/// added to scope by `discover_by_type_use` (no marker = no span), so
-/// the marker-only Layer 2 missed unmarked TB-13 contributors. Fix:
+/// Lines to scan for Layer 2 forbidden tokens.
 ///
-/// - If the file carries any TB-13 authoring marker → return
-///   `tb_13_spans()` (marker behavior preserved; legacy doc-xref
-///   continues to be skipped).
-/// - Otherwise (file in scope only via type-use discovery) → return
-///   every non-comment line. Unmarked TB-13 contributors are scanned
-///   wholesale because we cannot rely on marker-discipline to
-///   delineate "their" code.
+/// **Codex round-4 RQ6 (2026-05-03)**: `tb_13_spans()` returns nothing
+/// for files added to scope by `discover_by_type_use` (no marker = no
+/// span), so the marker-only Layer 2 missed unmarked TB-13 contributors.
+///
+/// **Codex round-5 (R5) PARTIAL-MARKER (2026-05-03)**: round-6's
+/// either/or rule (marker-file → spans-only; unmarked file → all
+/// non-comment lines) left a hole: a marker-bearing file could hide
+/// non-marker TB-13 type-use plus f64/AMM tokens outside any marker
+/// span. Fix: for marker-files, scan marker-spans UNION any non-comment
+/// line that contains a TB-13 type name (catches stealth TB-13 type-uses
+/// outside marker spans — those lines ARE TB-13 contributions by
+/// definition because they reference TB-13-introduced types).
+///
+/// Final rules:
+/// - Marker-file: marker-spans ∪ non-comment lines containing TB-13 type names.
+/// - Unmarked-discovered file: all non-comment lines (round-6 behavior).
+///
+/// Residual gap (acknowledged): a TB-13 helper that uses zero TB-13 type
+/// names AND lives outside marker spans (e.g., a generic math helper
+/// called only by TB-13 code). Without a code-marker AND without a
+/// type-name signal, the fence has no way to identify it as TB-13. This
+/// is a defense-in-depth limit of marker+type-name discipline; manual
+/// code review remains the residual halt-trigger guard.
 fn tb_13_scan_lines(source: &str) -> Vec<(usize, String)> {
-    if source.lines().any(is_tb_13_authoring_marker) {
-        return tb_13_spans(source);
+    use std::collections::BTreeMap;
+    let has_marker = source.lines().any(is_tb_13_authoring_marker);
+    if has_marker {
+        // Marker-file: marker-spans ∪ non-comment lines with TB-13 type names.
+        let mut acc: BTreeMap<usize, String> = BTreeMap::new();
+        for (n, l) in tb_13_spans(source) {
+            acc.insert(n, l);
+        }
+        for (i, line) in source.lines().enumerate() {
+            if is_pure_comment_line(line) {
+                continue;
+            }
+            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
+                acc.insert(i + 1, line.to_string());
+            }
+        }
+        return acc.into_iter().collect();
     }
+    // Unmarked-discovered file: all non-comment lines.
     source
         .lines()
         .enumerate()
         .filter(|(_, line)| !is_pure_comment_line(line))
         .map(|(i, line)| (i + 1, line.to_string()))
         .collect()
 }
 
 /// Read a source file relative to the workspace root, returning its
 /// content as a String. Panics with a clear message if missing — fence
 /// requires every file in `FENCE_SCOPE_FLOOR` to exist.
 fn read_scope_file(rel_path: &str) -> String {
     let full = workspace_root().join(rel_path);
     fs::read_to_string(&full)
         .unwrap_or_else(|e| panic!("TB-13 fence: failed to read {rel_path}: {e}"))
 }
 
 /// TB-13 type names — any non-comment use of one of these in `src/`
 /// marks the file as a TB-13 contributor. Codex round-3 RQ6 remediation
 /// 2026-05-03: the round-2 marker-only discovery caught files whose
 /// authors followed the `TRACE_MATRIX TB-13 ` / `// TB-13 ` convention,
 /// but a contributor could `use crate::state::typed_tx::CompleteSetMintTx;`
 /// in a fresh file without adding a marker — and the fence would miss
 /// it. The type-use walk closes that gap by checking for the distinctive
 /// TB-13-introduced symbol set itself; the marker discipline becomes a
 /// hint for human readers, not the only line of defense.
 ///
 /// All names below are TB-13-introduced and have no pre-existing
 /// occurrence in `src/` outside `FENCE_SCOPE_FLOOR`.
 const TB_13_TYPE_NAMES: &[&str] = &[
     "CompleteSetMintTx",
     "CompleteSetRedeemTx",
     "MarketSeedTx",
     "ConditionalCollateralIndex",
     "ConditionalShareBalances",
     "ShareSidePair",
     "EventNotOpen",
     "EventId",
     "OutcomeSide",
     "ShareAmount",
 ];
 
 /// True iff `line` is a pure comment (line starts with `//`, `///`, or
 /// `//!` after leading whitespace). Used by type-use discovery to skip
 /// legacy doc-comment cross-references (e.g., TB-12 `kernel.rs` /// doc
 /// strings that mention `CompleteSetMintTx` as future work).
 fn is_pure_comment_line(line: &str) -> bool {
     line.trim_start().starts_with("//")
 }
 
 /// Walk `walk_root` for any `.rs` file containing a TB-13 authoring
 /// marker on at least one line. Codex round-2 CHALLENGE remediation
 /// 2026-05-03: makes the fence self-extending against marked but
 /// not-yet-listed contributors.
 fn discover_by_marker(walk_root: &std::path::Path) -> Vec<String> {
     let mut found: Vec<String> = Vec::new();
     walk_rs_files(walk_root, &mut |path| {
         let rel = path
             .strip_prefix(workspace_root())
             .unwrap_or(path)
             .to_string_lossy()
             .into_owned();
         let body = match fs::read_to_string(path) {
             Ok(s) => s,
             Err(_) => return,
         };
         if body.lines().any(is_tb_13_authoring_marker) {
             found.push(rel);
         }
     });
     found
 }
 
 /// Walk `walk_root` for any `.rs` file that USES a TB-13 type name
 /// (`TB_13_TYPE_NAMES`) on a non-comment line. Codex round-3 RQ6
 /// remediation 2026-05-03: catches contributors who imported TB-13
 /// types without remembering the authoring-marker convention.
 fn discover_by_type_use(walk_root: &std::path::Path) -> Vec<String> {
     let mut found: Vec<String> = Vec::new();
     walk_rs_files(walk_root, &mut |path| {
         let rel = path
             .strip_prefix(workspace_root())
             .unwrap_or(path)
             .to_string_lossy()
             .into_owned();
         let body = match fs::read_to_string(path) {
             Ok(s) => s,
             Err(_) => return,
         };
         for line in body.lines() {
             if is_pure_comment_line(line) {
                 continue;
             }
             if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
                 found.push(rel);
                 return;
             }
         }
     });
     found
 }
 
 /// Discover every TB-13-contributing file in `src/`. Union of
 /// marker-walk (round-2) + type-use-walk (round-3 RQ6). Either path
 /// alone would leave a loophole; together they enforce the fence even
 /// when the human-followed marker convention slips.
 fn discover_tb_13_files() -> Vec<String> {
     let src_root = workspace_root().join("src");
     let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
     for f in discover_by_marker(&src_root) {
         set.insert(f);
     }
     for f in discover_by_type_use(&src_root) {
         set.insert(f);
     }
     set.into_iter().collect()
 }
 
 fn walk_rs_files(dir: &std::path::Path, visitor: &mut dyn FnMut(&std::path::Path)) {
     let entries = match fs::read_dir(dir) {
         Ok(e) => e,
         Err(_) => return,
     };
     for entry in entries.flatten() {
         let path = entry.path();
         if path.is_dir() {
             walk_rs_files(&path, visitor);
         } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
             visitor(&path);
         }
     }
 }
 
-/// Effective fence scope = FLOOR ∪ discovered. Deduplicated, sorted.
+/// Layer 1 fence scope = FLOOR ∪ discovered. Deduplicated, sorted.
+/// Used by `legacy_cpm_api_not_imported_by_complete_set` Layer 1
+/// (HARD_BANNED_LEGACY_IMPORTS unconditional whole-file scan).
+///
+/// Layer 1 is broader than Layer 2 because legacy imports are forbidden
+/// EVERYWHERE in TB-13-relevant scope, regardless of whether the file
+/// carries TB-13 markers or type uses today. `audit_dashboard.rs` lives
+/// here because it is TB-13-relevant scope (Atom 4 §13/§14 dashboard
+/// renders TB-13 state), even though its current contributions are TB-12.
 fn effective_fence_scope() -> Vec<String> {
     let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
     for s in FENCE_SCOPE_FLOOR {
         set.insert((*s).to_string());
     }
     for s in discover_tb_13_files() {
         set.insert(s);
     }
     set.into_iter().collect()
 }
 
+/// Layer 2 fence scope = discovered only (marker OR type-use).
+/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
+///
+/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
+/// narrower than Layer 1 because Layer 2 tokens (f64 / AMM / orderbook /
+/// price names) can legitimately appear in non-TB-13 files for unrelated
+/// reasons (e.g., negative-list test fixtures in `audit_dashboard.rs`
+/// at line 1628 that BAN those tokens — not USE them). Restricting
+/// Layer 2 to discovered files (i.e., files that actually contribute
+/// TB-13 code via marker OR TB-13 type use) prevents false positives on
+/// non-TB-13 baseline code that happens to mention forbidden token
+/// names. `audit_dashboard.rs` will auto-enter this scope when TB-14
+/// ships dashboard contributions there.
+fn effective_layer_2_scope() -> Vec<String> {
+    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
+    for s in discover_tb_13_files() {
+        set.insert(s);
+    }
+    set.into_iter().collect()
+}
+
 /// Hard-banned legacy CPMM imports — these strings MUST NOT appear in
 /// any FENCE_SCOPE file regardless of TB-13-marker discipline. Codex
 /// round-1 Q9 CHALLENGE remediation (2026-05-03): the marker-only fence
 /// could be bypassed by writing a legacy import outside a TB-13 doc-
 /// comment span. These tokens are unconditionally banned (a `use
 /// crate::prediction_market::BinaryMarket` anywhere in scope is an
 /// architectural regression even in non-TB-13 sections).
 const HARD_BANNED_LEGACY_IMPORTS: &[&str] = &[
     "use crate::prediction_market::",
     "use crate::prediction_market;",
     "crate::prediction_market::BinaryMarket",
     "crate::prediction_market::MarketError",
 ];
 
 /// SG-13.0.1 — `legacy_cpm_api_not_imported_by_complete_set`.
 ///
 /// Architect §4.2 halting trigger: HALT if NEW TB-13 code imports legacy
 /// `prediction_market.rs`. Two layers of enforcement:
 ///
 /// **Layer 1 (unconditional, Codex round-1 Q9 remediation)**: scan every
 /// FENCE_SCOPE file for `HARD_BANNED_LEGACY_IMPORTS` regardless of
 /// TB-13-marker discipline. Catches any new use-statement or type
 /// reference that pulls legacy CPMM into a TB-13-scope module.
 ///
 /// **Layer 2 (TB-13-marker-scoped)**: scan TB-13-marked spans for the
 /// broader `FORBIDDEN_LEGACY_TOKENS` set (API names, trading/AMM
 /// concepts). The marker discipline allows benign references in
 /// historical doc-comments while keeping new TB-13 code clean.
 #[test]
 fn legacy_cpm_api_not_imported_by_complete_set() {
     let mut violations: Vec<String> = Vec::new();
     let scope = effective_fence_scope();
 
     // Layer 1: unconditional whole-file scan for hard-banned imports.
     for rel in &scope {
         let source = read_scope_file(rel);
         for (line_no, line) in source.lines().enumerate() {
             for token in HARD_BANNED_LEGACY_IMPORTS {
                 if line.contains(token) {
                     violations.push(format!(
                         "{rel}:{}: hard-banned legacy import `{token}` — {line}",
                         line_no + 1
                     ));
                 }
             }
         }
     }
 
-    // Layer 2: scan for trading/AMM concepts. Marker-discovered files use
-    // `tb_13_spans()` (marker behavior); type-use-discovered files use
-    // `tb_13_scan_lines()` which falls back to all non-comment lines for
-    // unmarked contributors (Codex round-4 RQ6 remediation 2026-05-03).
-    for rel in &scope {
+    // Layer 2: scan for trading/AMM concepts. Restricted to discovered
+    // (Codex round-5 DASHBOARD-FLOOR remediation 2026-05-03): Layer 2
+    // tokens (f64 / AMM / orderbook / price names) can appear legitimately
+    // in non-TB-13 baseline code (e.g., negative-list test fixtures); only
+    // files that actually contribute TB-13 (via marker OR TB-13 type use)
+    // should be Layer-2-scanned. `tb_13_scan_lines` then resolves the
+    // PARTIAL-MARKER case: marker-spans ∪ non-marker TB-13-type-use lines.
+    let layer_2_scope = effective_layer_2_scope();
+    for rel in &layer_2_scope {
         let source = read_scope_file(rel);
         for (line_no, line) in tb_13_scan_lines(&source) {
             for token in FORBIDDEN_LEGACY_TOKENS {
                 // The `f64` family entries are checked in SG-13.0.2 — skip
                 // them here so the failure message is unambiguous.
                 if token.starts_with(" f64")
                     || token.starts_with("f64,")
                     || token.starts_with("f64;")
                     || token.starts_with("f64)")
                 {
                     continue;
                 }
                 if line.contains(token) {
                     violations.push(format!(
                         "{rel}:{line_no}: TB-13-scope contains forbidden token `{token}` — {line}"
                     ));
                 }
             }
         }
     }
     assert!(
         violations.is_empty(),
-        "TB-13 SG-13.0.1 forward-fence violated (scope: {} files):\n{}",
+        "TB-13 SG-13.0.1 forward-fence violated (Layer 1 scope: {} files; Layer 2 scope: {} files):\n{}",
         scope.len(),
+        layer_2_scope.len(),
         violations.join("\n")
     );
 }
 
 /// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
-/// effective_fence_scope() (auto-discovers new TB-13 files).
+/// effective_layer_2_scope() (discovered-only, per Codex R5 DASHBOARD-FLOOR
+/// remediation 2026-05-03 — Layer 2 tokens like f64 can appear in
+/// non-TB-13 baseline code for unrelated reasons).
 ///
 /// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
 /// CompleteSet / MarketSeed code. Money-path types must use integer
 /// `MicroCoin` / `ShareAmount`.
 #[test]
 fn no_f64_in_complete_set_or_market_seed() {
     let mut violations: Vec<String> = Vec::new();
     let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
-    for rel in &effective_fence_scope() {
+    for rel in &effective_layer_2_scope() {
         let source = read_scope_file(rel);
-        // tb_13_scan_lines: marker-files → spans; unmarked-discovered files
-        // → all non-comment lines (Codex round-4 RQ6 remediation 2026-05-03).
+        // tb_13_scan_lines: marker-files → spans ∪ non-marker TB-13-type-use
+        // lines (Codex R5 PARTIAL-MARKER); unmarked-discovered files →
+        // all non-comment lines (Codex R4 RQ6).
         for (line_no, line) in tb_13_scan_lines(&source) {
             for token in &f64_tokens {
                 if line.contains(token) {
                     violations.push(format!(
                         "{rel}:{line_no}: TB-13-scope contains f64 (`{token}`) — {line}"
                     ));
                 }
             }
         }
     }
     assert!(
         violations.is_empty(),
         "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
         violations.join("\n")
     );
 }
 
 /// SG-13.0.3 — `prediction_market_legacy_quarantined`.
 ///
 /// Architect §4.2 ship gate: legacy CPMM "must be clearly labeled". We
 /// enforce that `src/prediction_market.rs` carries the LEGACY module-
 /// header doc-comment with the four required tokens (`legacy`,
 /// `not constitutional`, `not RSP-M`, `not production market path`)
 /// AND that `src/kernel.rs` market-bearing fields carry the `LEGACY`
 /// label tying them to the migration path.
 #[test]
 fn prediction_market_legacy_quarantined() {
     let pm = read_scope_file("src/prediction_market.rs");
     let header = pm
         .lines()
         .take(60)
         .collect::<Vec<_>>()
         .join("\n");
 
     let required_label_tokens = [
         "LEGACY",
         "not constitutional",
         "not RSP-M",
         "not production market path",
     ];
     for token in &required_label_tokens {
         assert!(
             header.contains(token),
             "TB-13 SG-13.0.3: src/prediction_market.rs module header missing required \
              label token `{token}`. Header:\n{header}"
         );
     }
 
     // Architect §4.2 also requires the doc to name the migration path so
     // future maintainers don't reintroduce the legacy API.
     let migration_tokens = [
         "TB-13",
         "TB-14",
         "CompleteSetMintTx",
         "OBS_TB_12_LEGACY_CPMM_QUARANTINE",
     ];
     for token in &migration_tokens {
         assert!(
             header.contains(token),
             "TB-13 SG-13.0.3: src/prediction_market.rs module header missing migration-path \
              token `{token}`. Header:\n{header}"
         );
     }
 
     // Defense-in-depth: kernel.rs market-bearing fields carry LEGACY.
     let kernel = read_scope_file("src/kernel.rs");
     let kernel_struct_idx = kernel
         .find("pub struct Kernel {")
         .expect("TB-13 SG-13.0.3: cannot locate `pub struct Kernel {` in src/kernel.rs");
     let kernel_struct_end = kernel[kernel_struct_idx..]
         .find("\n}\n")
         .map(|off| kernel_struct_idx + off + 2)
         .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
     let kernel_struct_block = &kernel[kernel_struct_idx..kernel_struct_end];
 
     for field in ["markets", "bounty_market", "bounty_lp_seed"] {
         let field_marker = format!("pub {field}");
         let field_idx = kernel_struct_block
             .find(&field_marker)
             .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
@@ -559,80 +626,130 @@ fn discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref() {
         found.iter().map(|s| s.as_str()).collect();
 
     let unmarked_str = unmarked_path.to_string_lossy().into_owned();
     let docxref_str = docxref_path.to_string_lossy().into_owned();
     let neutral_str = neutral_path.to_string_lossy().into_owned();
 
     assert!(
         found_set.contains(unmarked_str.as_str()),
         "RQ6: unmarked TB-13 type-use file must be discovered. Got: {found:?}"
     );
     assert!(
         !found_set.contains(docxref_str.as_str()),
         "RQ6: doc-xref-only file must NOT be discovered. Got: {found:?}"
     );
     assert!(
         !found_set.contains(neutral_str.as_str()),
         "RQ6: neutral file must NOT be discovered. Got: {found:?}"
     );
 
     // Also assert that the marker walk alone would have missed (1) —
     // proves type-use is the path that catches it.
     let marker_only = discover_by_marker(&tmp);
     let marker_set: std::collections::BTreeSet<&str> =
         marker_only.iter().map(|s| s.as_str()).collect();
     assert!(
         !marker_set.contains(unmarked_str.as_str()),
         "RQ6: marker walk alone should NOT have caught the unmarked file (otherwise the type-use layer is redundant). Got: {marker_only:?}"
     );
 
     let _ = fs::remove_dir_all(&tmp);
 }
 
 /// Round-6 R4-Codex remediation 2026-05-03: `tb_13_scan_lines` returns
 /// marker-spans for marker-files (preserves the doc-xref skip) and all
 /// non-comment lines for unmarked files (closes the Layer 2 gap where
 /// type-use-discovered files could ship f64 / AMM tokens unscanned).
 #[test]
 fn tb_13_scan_lines_handles_marker_and_unmarked_files() {
     // Case A — marker-file: scan lines come from `tb_13_spans`. A
     // /// TB-12 line referencing TB-13 in passing is OUTSIDE any TB-13
     // span (because the span's first non-blank line is the TB-12 marker,
     // not a TB-13 marker), so it must be skipped.
     let marker_src = "\
 //! TB-13 module header.\n\
 pub fn tb13_thing() -> i32 { 42_f64 as i32 }\n\
 \n\
 /// TB-12 doc xref to TB-13 future work.\n\
 pub fn tb12_legacy() -> i32 { 0 }\n\
 ";
     let scanned = tb_13_scan_lines(marker_src);
     let scanned_text: Vec<&str> =
         scanned.iter().map(|(_, l)| l.as_str()).collect();
     assert!(
         scanned_text.iter().any(|l| l.contains("tb13_thing")),
         "marker-file: TB-13 span lines must be returned"
     );
     assert!(
         scanned_text.iter().all(|l| !l.contains("tb12_legacy")),
         "marker-file: TB-12 span lines must NOT be returned (preserves doc-xref skip)"
     );
 
     // Case B — unmarked file: scan lines fall back to ALL non-comment
     // lines. The f64 / AMM scan must see the violating line.
     let unmarked_src = "\
 use crate::state::typed_tx::CompleteSetMintTx;\n\
 fn forbidden() -> f64 { 0.5_f64 }\n\
 // trailing comment\n\
 ";
     let scanned = tb_13_scan_lines(unmarked_src);
     let scanned_text: Vec<&str> =
         scanned.iter().map(|(_, l)| l.as_str()).collect();
     assert!(
         scanned_text.iter().any(|l| l.contains("f64")),
         "unmarked-file: non-comment lines must be returned (Layer 2 must see f64)"
     );
     assert!(
         scanned_text.iter().all(|l| !l.contains("trailing comment")),
         "unmarked-file: pure-comment lines must still be filtered out"
     );
 }
+
+/// Round-7 R5-Codex PARTIAL-MARKER remediation 2026-05-03: a
+/// marker-bearing file with stealth TB-13 type-use OUTSIDE any marker
+/// span must still have those non-marker type-use lines scanned.
+#[test]
+fn tb_13_scan_lines_partial_marker_catches_stealth_type_use() {
+    // Marker-file: one marker-span at top + a TB-13 type use OUTSIDE the
+    // marker span (no TB-13 marker on the second function). Round-6
+    // helper would have only scanned the marker span; round-7 must also
+    // return the non-marker line containing `CompleteSetMintTx`.
+    let src = "\
+//! TB-13 module header.\n\
+pub fn tb13_marked() -> i32 { 0 }\n\
+\n\
+fn stealth(_: CompleteSetMintTx) -> f64 { 0.0_f64 }\n\
+";
+    let scanned = tb_13_scan_lines(src);
+    let scanned_text: Vec<&str> =
+        scanned.iter().map(|(_, l)| l.as_str()).collect();
+    assert!(
+        scanned_text.iter().any(|l| l.contains("tb13_marked")),
+        "marker-span line must be returned"
+    );
+    assert!(
+        scanned_text
+            .iter()
+            .any(|l| l.contains("CompleteSetMintTx") && l.contains("f64")),
+        "non-marker line containing TB-13 type name must also be returned (PARTIAL-MARKER closure)"
+    );
+}
+
+/// Round-7 R5-Codex DASHBOARD-FLOOR remediation 2026-05-03: Layer 1
+/// (hard-banned-imports) scope retains `audit_dashboard.rs`; Layer 2
+/// (forbidden-token) scope omits it because it currently has no TB-13
+/// markers and no TB-13 type uses. The split prevents Layer 2 false-
+/// positives on negative-list test fixtures while preserving Layer 1
+/// hard-import enforcement.
+#[test]
+fn audit_dashboard_in_layer_1_scope_but_not_layer_2_scope() {
+    let layer_1 = effective_fence_scope();
+    let layer_2 = effective_layer_2_scope();
+    assert!(
+        layer_1.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
+        "DASHBOARD-FLOOR: audit_dashboard.rs must remain in Layer 1 scope (hard-imports always banned). Got: {layer_1:?}"
+    );
+    assert!(
+        !layer_2.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
+        "DASHBOARD-FLOOR: audit_dashboard.rs must NOT be in Layer 2 scope until it gains TB-13 markers / type uses (otherwise its negative-list test fixture false-positives). Got: {layer_2:?}"
+    );
+}

codex
The patch does implement the claimed two changes in the test fence file. I’m checking current `src/` discovery inputs next, because the tier split only holds if `audit_dashboard.rs` is currently undiscovered and Layer 1 still reads it from the floor.
exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '560,790p'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc 'rg -n "TRACE_MATRIX TB-13 |//! TB-13 |/// TB-13 |// TB-13 |CompleteSetMintTx|CompleteSetRedeemTx|MarketSeedTx|ConditionalCollateralIndex|ConditionalShareBalances|ShareSidePair|EventNotOpen|EventId|OutcomeSide|ShareAmount" src -g '"'*.rs'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '1600,1650p'" in /home/zephryj/projects/turingosv4
 succeeded in 229ms:
   560	        .expect("TB-13 SG-13.0.3: cannot locate end of Kernel struct");
   561	    let kernel_struct_block = &kernel[kernel_struct_idx..kernel_struct_end];
   562	
   563	    for field in ["markets", "bounty_market", "bounty_lp_seed"] {
   564	        let field_marker = format!("pub {field}");
   565	        let field_idx = kernel_struct_block
   566	            .find(&field_marker)
   567	            .unwrap_or_else(|| panic!("TB-13 SG-13.0.3: cannot locate field `{field}` in Kernel struct"));
   568	        // Look for `LEGACY` in the 600 chars preceding the field
   569	        // declaration (covers a multi-line doc-comment block).
   570	        let doc_window_start = field_idx.saturating_sub(600);
   571	        let doc_window = &kernel_struct_block[doc_window_start..field_idx];
   572	        assert!(
   573	            doc_window.contains("LEGACY"),
   574	            "TB-13 SG-13.0.3: Kernel.{field} missing LEGACY doc-comment label. \
   575	             Doc window:\n{doc_window}"
   576	        );
   577	    }
   578	}
   579	
   580	/// Round-5 RQ6 unit test: `discover_by_type_use` catches a fresh file
   581	/// that imports a TB-13 type without an authoring marker, and the
   582	/// pure-comment skip prevents a TB-12 doc-comment cross-reference
   583	/// from being misclassified as a TB-13 contributor.
   584	#[test]
   585	fn discover_by_type_use_catches_unmarked_imports_and_skips_doc_xref() {
   586	    use std::io::Write;
   587	    let tmp = std::env::temp_dir().join(format!(
   588	        "tb13_fence_discovery_test_{}",
   589	        std::process::id()
   590	    ));
   591	    let _ = fs::remove_dir_all(&tmp);
   592	    fs::create_dir_all(&tmp).expect("mkdir tmp");
   593	
   594	    // (1) An unmarked file that USES a TB-13 type → must be discovered.
   595	    let unmarked_path = tmp.join("unmarked_user.rs");
   596	    {
   597	        let mut f = fs::File::create(&unmarked_path).expect("create unmarked");
   598	        writeln!(
   599	            f,
   600	            "use crate::state::typed_tx::CompleteSetMintTx;\nfn touch() -> CompleteSetMintTx {{ CompleteSetMintTx::default() }}"
   601	        )
   602	        .unwrap();
   603	    }
   604	
   605	    // (2) A file with TB-13 type names ONLY in /// doc-comment lines →
   606	    // must NOT be discovered (TB-12 legacy doc-xref pattern).
   607	    let docxref_path = tmp.join("doc_xref_only.rs");
   608	    {
   609	        let mut f = fs::File::create(&docxref_path).expect("create docxref");
   610	        writeln!(
   611	            f,
   612	            "/// Replaced by TB-13 `CompleteSetMintTx` (canonical mint).\n//! see ConditionalShareBalances for the future shape.\npub struct Unrelated;"
   613	        )
   614	        .unwrap();
   615	    }
   616	
   617	    // (3) A control file with no TB-13 references → not discovered.
   618	    let neutral_path = tmp.join("neutral.rs");
   619	    {
   620	        let mut f = fs::File::create(&neutral_path).expect("create neutral");
   621	        writeln!(f, "pub fn add(a: i64, b: i64) -> i64 {{ a + b }}").unwrap();
   622	    }
   623	
   624	    let found = discover_by_type_use(&tmp);
   625	    let found_set: std::collections::BTreeSet<&str> =
   626	        found.iter().map(|s| s.as_str()).collect();
   627	
   628	    let unmarked_str = unmarked_path.to_string_lossy().into_owned();
   629	    let docxref_str = docxref_path.to_string_lossy().into_owned();
   630	    let neutral_str = neutral_path.to_string_lossy().into_owned();
   631	
   632	    assert!(
   633	        found_set.contains(unmarked_str.as_str()),
   634	        "RQ6: unmarked TB-13 type-use file must be discovered. Got: {found:?}"
   635	    );
   636	    assert!(
   637	        !found_set.contains(docxref_str.as_str()),
   638	        "RQ6: doc-xref-only file must NOT be discovered. Got: {found:?}"
   639	    );
   640	    assert!(
   641	        !found_set.contains(neutral_str.as_str()),
   642	        "RQ6: neutral file must NOT be discovered. Got: {found:?}"
   643	    );
   644	
   645	    // Also assert that the marker walk alone would have missed (1) —
   646	    // proves type-use is the path that catches it.
   647	    let marker_only = discover_by_marker(&tmp);
   648	    let marker_set: std::collections::BTreeSet<&str> =
   649	        marker_only.iter().map(|s| s.as_str()).collect();
   650	    assert!(
   651	        !marker_set.contains(unmarked_str.as_str()),
   652	        "RQ6: marker walk alone should NOT have caught the unmarked file (otherwise the type-use layer is redundant). Got: {marker_only:?}"
   653	    );
   654	
   655	    let _ = fs::remove_dir_all(&tmp);
   656	}
   657	
   658	/// Round-6 R4-Codex remediation 2026-05-03: `tb_13_scan_lines` returns
   659	/// marker-spans for marker-files (preserves the doc-xref skip) and all
   660	/// non-comment lines for unmarked files (closes the Layer 2 gap where
   661	/// type-use-discovered files could ship f64 / AMM tokens unscanned).
   662	#[test]
   663	fn tb_13_scan_lines_handles_marker_and_unmarked_files() {
   664	    // Case A — marker-file: scan lines come from `tb_13_spans`. A
   665	    // /// TB-12 line referencing TB-13 in passing is OUTSIDE any TB-13
   666	    // span (because the span's first non-blank line is the TB-12 marker,
   667	    // not a TB-13 marker), so it must be skipped.
   668	    let marker_src = "\
   669	//! TB-13 module header.\n\
   670	pub fn tb13_thing() -> i32 { 42_f64 as i32 }\n\
   671	\n\
   672	/// TB-12 doc xref to TB-13 future work.\n\
   673	pub fn tb12_legacy() -> i32 { 0 }\n\
   674	";
   675	    let scanned = tb_13_scan_lines(marker_src);
   676	    let scanned_text: Vec<&str> =
   677	        scanned.iter().map(|(_, l)| l.as_str()).collect();
   678	    assert!(
   679	        scanned_text.iter().any(|l| l.contains("tb13_thing")),
   680	        "marker-file: TB-13 span lines must be returned"
   681	    );
   682	    assert!(
   683	        scanned_text.iter().all(|l| !l.contains("tb12_legacy")),
   684	        "marker-file: TB-12 span lines must NOT be returned (preserves doc-xref skip)"
   685	    );
   686	
   687	    // Case B — unmarked file: scan lines fall back to ALL non-comment
   688	    // lines. The f64 / AMM scan must see the violating line.
   689	    let unmarked_src = "\
   690	use crate::state::typed_tx::CompleteSetMintTx;\n\
   691	fn forbidden() -> f64 { 0.5_f64 }\n\
   692	// trailing comment\n\
   693	";
   694	    let scanned = tb_13_scan_lines(unmarked_src);
   695	    let scanned_text: Vec<&str> =
   696	        scanned.iter().map(|(_, l)| l.as_str()).collect();
   697	    assert!(
   698	        scanned_text.iter().any(|l| l.contains("f64")),
   699	        "unmarked-file: non-comment lines must be returned (Layer 2 must see f64)"
   700	    );
   701	    assert!(
   702	        scanned_text.iter().all(|l| !l.contains("trailing comment")),
   703	        "unmarked-file: pure-comment lines must still be filtered out"
   704	    );
   705	}
   706	
   707	/// Round-7 R5-Codex PARTIAL-MARKER remediation 2026-05-03: a
   708	/// marker-bearing file with stealth TB-13 type-use OUTSIDE any marker
   709	/// span must still have those non-marker type-use lines scanned.
   710	#[test]
   711	fn tb_13_scan_lines_partial_marker_catches_stealth_type_use() {
   712	    // Marker-file: one marker-span at top + a TB-13 type use OUTSIDE the
   713	    // marker span (no TB-13 marker on the second function). Round-6
   714	    // helper would have only scanned the marker span; round-7 must also
   715	    // return the non-marker line containing `CompleteSetMintTx`.
   716	    let src = "\
   717	//! TB-13 module header.\n\
   718	pub fn tb13_marked() -> i32 { 0 }\n\
   719	\n\
   720	fn stealth(_: CompleteSetMintTx) -> f64 { 0.0_f64 }\n\
   721	";
   722	    let scanned = tb_13_scan_lines(src);
   723	    let scanned_text: Vec<&str> =
   724	        scanned.iter().map(|(_, l)| l.as_str()).collect();
   725	    assert!(
   726	        scanned_text.iter().any(|l| l.contains("tb13_marked")),
   727	        "marker-span line must be returned"
   728	    );
   729	    assert!(
   730	        scanned_text
   731	            .iter()
   732	            .any(|l| l.contains("CompleteSetMintTx") && l.contains("f64")),
   733	        "non-marker line containing TB-13 type name must also be returned (PARTIAL-MARKER closure)"
   734	    );
   735	}
   736	
   737	/// Round-7 R5-Codex DASHBOARD-FLOOR remediation 2026-05-03: Layer 1
   738	/// (hard-banned-imports) scope retains `audit_dashboard.rs`; Layer 2
   739	/// (forbidden-token) scope omits it because it currently has no TB-13
   740	/// markers and no TB-13 type uses. The split prevents Layer 2 false-
   741	/// positives on negative-list test fixtures while preserving Layer 1
   742	/// hard-import enforcement.
   743	#[test]
   744	fn audit_dashboard_in_layer_1_scope_but_not_layer_2_scope() {
   745	    let layer_1 = effective_fence_scope();
   746	    let layer_2 = effective_layer_2_scope();
   747	    assert!(
   748	        layer_1.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
   749	        "DASHBOARD-FLOOR: audit_dashboard.rs must remain in Layer 1 scope (hard-imports always banned). Got: {layer_1:?}"
   750	    );
   751	    assert!(
   752	        !layer_2.iter().any(|s| s == "src/bin/audit_dashboard.rs"),
   753	        "DASHBOARD-FLOOR: audit_dashboard.rs must NOT be in Layer 2 scope until it gains TB-13 markers / type uses (otherwise its negative-list test fixture false-positives). Got: {layer_2:?}"
   754	    );
   755	}

exec
/bin/bash -lc 'rg -n "TRACE_MATRIX TB-13 |//! TB-13 |/// TB-13 |// TB-13 |CompleteSetMintTx|CompleteSetRedeemTx|MarketSeedTx|ConditionalCollateralIndex|ConditionalShareBalances|ShareSidePair|EventNotOpen|EventId|OutcomeSide|ShareAmount" src/bin/audit_dashboard.rs' in /home/zephryj/projects/turingosv4
 succeeded in 333ms:
src/economy/monetary_invariant.rs:79:        side: crate::state::typed_tx::OutcomeSide,
src/economy/monetary_invariant.rs:192:    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
src/economy/monetary_invariant.rs:195:    // 5-holding sum to 6. Without this, CompleteSetMintTx (which migrates
src/economy/monetary_invariant.rs:353:        // TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
src/economy/monetary_invariant.rs:428:// TB-13 Atom 3 — assert_complete_set_balanced (architect 2026-05-03 post-
src/economy/monetary_invariant.rs:432:/// TRACE_MATRIX TB-13 Atom 3 (architect §4.3 + SG-13.1): the
src/economy/monetary_invariant.rs:460:    use crate::state::typed_tx::OutcomeSide;
src/economy/monetary_invariant.rs:480:                (OutcomeSide::Yes, sum_yes)
src/economy/monetary_invariant.rs:482:                (OutcomeSide::No, sum_no)
src/economy/monetary_invariant.rs:495:fn hex_event_id(event_id: &crate::state::typed_tx::EventId) -> String {
src/bottom_white/ledger/transition_ledger.rs:80:    /// TB-13 (2026-05-03 architect post-TB-12 ruling Part A §4.3) —
src/bottom_white/ledger/transition_ledger.rs:86:    /// TB-13 (architect §4.3) — agent-signed conditional-share redeem
src/bottom_white/ledger/transition_ledger.rs:94:    /// TB-13 (architect §4.3) — agent-signed protocol-owned share
src/state/q_state.rs:193:    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
src/state/q_state.rs:213:    pub conditional_collateral_t: ConditionalCollateralIndex,
src/state/q_state.rs:214:    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
src/state/q_state.rs:215:    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
src/state/q_state.rs:225:    pub conditional_share_balances_t: ConditionalShareBalances,
src/state/q_state.rs:529:// TB-13 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 + §4.4):
src/state/q_state.rs:530:// ConditionalCollateralIndex + ConditionalShareBalances — Polymarket / CTF
src/state/q_state.rs:537:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.4): per-event Coin
src/state/q_state.rs:549:pub struct ConditionalCollateralIndex(
src/state/q_state.rs:550:    pub BTreeMap<crate::state::typed_tx::EventId, MicroCoin>,
src/state/q_state.rs:553:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2): per-
src/state/q_state.rs:557:/// `ConditionalCollateralIndex[event_id]`. Architect CR-13.3 / SG-13.2
src/state/q_state.rs:560:/// Wire shape: `BTreeMap<AgentId, BTreeMap<EventId, ShareSidePair>>`.
src/state/q_state.rs:566:pub struct ConditionalShareBalances(
src/state/q_state.rs:569:        BTreeMap<crate::state::typed_tx::EventId, ShareSidePair>,
src/state/q_state.rs:573:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + FR-13.3): YES + NO share
src/state/q_state.rs:577:pub struct ShareSidePair {
src/state/q_state.rs:578:    pub yes: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:579:    pub no: crate::state::typed_tx::ShareAmount,
src/state/q_state.rs:792:        // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3):
src/state/sequencer.rs:254:/// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
src/state/sequencer.rs:259:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetMintTx`
src/state/sequencer.rs:270:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): CompleteSetRedeem-accept
src/state/sequencer.rs:275:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `CompleteSetRedeemTx`
src/state/sequencer.rs:286:/// TRACE_MATRIX TB-13 Atom 2 (architect §4.3): MarketSeed-accept state-root
src/state/sequencer.rs:291:/// TRACE_MATRIX TB-13 Atom 2: state-root mutator on `MarketSeedTx` accept.
src/state/sequencer.rs:1571:        // TB-13 Atom 2 — CompleteSetMintTx accept arm (architect 2026-05-03
src/state/sequencer.rs:1603:            // requires a task to exist; EventId is 1:1 with TaskId in
src/state/sequencer.rs:1604:            // TB-13 per architect §4.3).
src/state/sequencer.rs:1613:                return Err(TransitionError::EventNotOpen);
src/state/sequencer.rs:1654:                .or_insert(crate::state::q_state::ShareSidePair::default());
src/state/sequencer.rs:1655:            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1658:            pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1686:        // TB-13 Atom 2 — CompleteSetRedeemTx accept arm (architect §4.3 +
src/state/sequencer.rs:1715:                 crate::state::typed_tx::OutcomeSide::Yes) => { /* ok — YES wins */ }
src/state/sequencer.rs:1717:                 crate::state::typed_tx::OutcomeSide::No) => { /* ok — NO wins */ }
src/state/sequencer.rs:1737:                crate::state::typed_tx::OutcomeSide::Yes => pair.yes.units,
src/state/sequencer.rs:1738:                crate::state::typed_tx::OutcomeSide::No => pair.no.units,
src/state/sequencer.rs:1768:                    .or_insert(crate::state::q_state::ShareSidePair::default());
src/state/sequencer.rs:1770:                    crate::state::typed_tx::OutcomeSide::Yes => {
src/state/sequencer.rs:1771:                        pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1775:                    crate::state::typed_tx::OutcomeSide::No => {
src/state/sequencer.rs:1776:                        pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1829:        // TB-13 Atom 2 — MarketSeedTx accept arm (architect §4.3 + FR-13.6..7 +
src/state/sequencer.rs:1856:                return Err(TransitionError::EventNotOpen);
src/state/sequencer.rs:1895:                .or_insert(crate::state::q_state::ShareSidePair::default());
src/state/sequencer.rs:1896:            pair.yes = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1899:            pair.no = crate::state::typed_tx::ShareAmount::from_units(
src/state/sequencer.rs:1998:    /// TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH remediation 2026-05-03):
src/state/sequencer.rs:2000:    /// TB-13 variant (CompleteSetMint / CompleteSetRedeem / MarketSeed)
src/state/sequencer.rs:2303:    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH remediation
src/state/sequencer.rs:2376:    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
src/state/sequencer.rs:2450:        // TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
src/kernel.rs:23:    /// Replaced by TB-13 `ConditionalShareBalances` (canonical YES/NO claims)
src/kernel.rs:39:    /// Replaced by TB-13 `MarketSeedTx` (explicit provider funds, integer
src/runtime/verify.rs:442:            // TRACE_MATRIX TB-13 Atom 6 round-2 (Codex VETO TB13-V2
src/prediction_market.rs:3://! TRACE_MATRIX TB-13 Atom 0.5 (architect 2026-05-03 ruling Part A §4.2):
src/prediction_market.rs:9://!   `CompleteSetMintTx` + `ConditionalShareBalances` for that.
src/state/typed_tx.rs:603://   No CompleteSet / MarketSeedTx / AMM / CPMM (TB-13/14 territory).
src/state/typed_tx.rs:828:// TB-13 — CompleteSet + MarketSeedTx (architect 2026-05-03 post-TB-12 ruling Part A §4.3).
src/state/typed_tx.rs:1057:// § 5c-TB-13 — CompleteSet + MarketSeedTx conditional shares
src/state/typed_tx.rs:1059:// TRACE_MATRIX TB-13 Atom 1 (architect 2026-05-03 post-TB-12 ruling Part A
src/state/typed_tx.rs:1063:// `CompleteSetMintTx` debits Coin balance, locks it as `conditional_collateral_t`,
src/state/typed_tx.rs:1064:// mints equal YES_E + NO_E shares to the same owner. `CompleteSetRedeemTx`
src/state/typed_tx.rs:1066:// 1:1 against `conditional_collateral_t`. `MarketSeedTx` requires explicit
src/state/typed_tx.rs:1074:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): event identifier for
src/state/typed_tx.rs:1075:/// conditional shares. TB-13 maps `EventId` 1:1 to `TaskId` (the event
src/state/typed_tx.rs:1080:pub struct EventId(pub TaskId);
src/state/typed_tx.rs:1082:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): outcome-side discriminator
src/state/typed_tx.rs:1087:pub enum OutcomeSide {
src/state/typed_tx.rs:1092:impl Default for OutcomeSide {
src/state/typed_tx.rs:1098:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): non-negative share count.
src/state/typed_tx.rs:1105:pub struct ShareAmount {
src/state/typed_tx.rs:1109:impl ShareAmount {
src/state/typed_tx.rs:1110:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): zero share amount —
src/state/typed_tx.rs:1115:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): build a `ShareAmount`
src/state/typed_tx.rs:1124:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.1..3): mint conditional
src/state/typed_tx.rs:1137:pub struct CompleteSetMintTx {
src/state/typed_tx.rs:1140:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1147:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.4..5 + SG-13.5..6):
src/state/typed_tx.rs:1168:pub struct CompleteSetRedeemTx {
src/state/typed_tx.rs:1171:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1173:    pub outcome: OutcomeSide,                 //  5
src/state/typed_tx.rs:1174:    pub share_amount: ShareAmount,            //  6
src/state/typed_tx.rs:1179:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3 + FR-13.6..7): explicit
src/state/typed_tx.rs:1193:/// The shape is identical to `CompleteSetMintTx` post-effect; the
src/state/typed_tx.rs:1199:pub struct MarketSeedTx {
src/state/typed_tx.rs:1202:    pub event_id: EventId,                    //  3
src/state/typed_tx.rs:1211:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
src/state/typed_tx.rs:1212:/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1217:    pub event_id: EventId,
src/state/typed_tx.rs:1224:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
src/state/typed_tx.rs:1225:    /// canonical digest for agent-signed CompleteSetMintTx. Domain
src/state/typed_tx.rs:1234:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
src/state/typed_tx.rs:1235:/// `CompleteSetRedeemTx` (8 fields → 7 fields; signature excluded).
src/state/typed_tx.rs:1240:    pub event_id: EventId,
src/state/typed_tx.rs:1242:    pub outcome: OutcomeSide,
src/state/typed_tx.rs:1243:    pub share_amount: ShareAmount,
src/state/typed_tx.rs:1248:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
src/state/typed_tx.rs:1249:    /// canonical digest for agent-signed CompleteSetRedeemTx. Domain
src/state/typed_tx.rs:1256:/// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): signing payload for
src/state/typed_tx.rs:1257:/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1262:    pub event_id: EventId,
src/state/typed_tx.rs:1269:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): domain-prefixed
src/state/typed_tx.rs:1270:    /// canonical digest for agent-signed MarketSeedTx. Domain prefix
src/state/typed_tx.rs:1435:// TB-13 — projection impls.
src/state/typed_tx.rs:1437:impl CompleteSetMintTx {
src/state/typed_tx.rs:1438:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
src/state/typed_tx.rs:1452:impl CompleteSetRedeemTx {
src/state/typed_tx.rs:1453:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
src/state/typed_tx.rs:1468:impl MarketSeedTx {
src/state/typed_tx.rs:1469:    /// TRACE_MATRIX TB-13 Atom 1 (architect §4.3): wire → signing payload
src/state/typed_tx.rs:1508:    CompleteSetMint(CompleteSetMintTx),   // TB-13 agent-signed conditional-share mint
src/state/typed_tx.rs:1509:    CompleteSetRedeem(CompleteSetRedeemTx), // TB-13 agent-signed conditional-share redeem
src/state/typed_tx.rs:1510:    MarketSeed(MarketSeedTx),             // TB-13 agent-signed protocol-owned share seed
src/state/typed_tx.rs:1613:// TB-13 — agent-signed conditional-share variants. Submitter is the
src/state/typed_tx.rs:1616:impl HasSubmitter for CompleteSetMintTx {
src/state/typed_tx.rs:1622:impl HasSubmitter for CompleteSetRedeemTx {
src/state/typed_tx.rs:1628:impl HasSubmitter for MarketSeedTx {
src/state/typed_tx.rs:1837:    /// `CompleteSetMintTx` admission: `balances_t[owner] < amount`.
src/state/typed_tx.rs:1841:    /// `CompleteSetRedeemTx` admission: the referenced event is in
src/state/typed_tx.rs:1847:    /// `CompleteSetRedeemTx` admission: the owner's
src/state/typed_tx.rs:1852:    /// `MarketSeedTx` admission: `collateral_amount.micro_units() == 0`.
src/state/typed_tx.rs:1854:    /// collateral. Also fired defensively at `CompleteSetRedeemTx` time
src/state/typed_tx.rs:1859:    /// `CompleteSetRedeemTx` admission: the redeem's `outcome` does not
src/state/typed_tx.rs:1865:    /// TB-13 Atom 6 round-2 (Gemini CHALLENGE Q13 remediation 2026-05-03):
src/state/typed_tx.rs:1866:    /// `CompleteSetMintTx` / `MarketSeedTx` admission rejected because
src/state/typed_tx.rs:1873:    EventNotOpen,
src/state/typed_tx.rs:1941:                "CompleteSetMintTx: owner's balances_t entry is below the requested mint amount"
src/state/typed_tx.rs:1945:                "CompleteSetRedeemTx: event task_markets_t state is Open or Expired (no system-emitted resolution yet)"
src/state/typed_tx.rs:1949:                "CompleteSetRedeemTx: owner's conditional share balance is below the requested redeem amount"
src/state/typed_tx.rs:1957:                "CompleteSetRedeemTx: outcome does not match task_markets_t[event_id.0] state"
src/state/typed_tx.rs:1959:            Self::EventNotOpen => write!(
src/state/typed_tx.rs:3099:    // TB-13 Atom 1 unit tests — CompleteSetMint / CompleteSetRedeem /
src/state/typed_tx.rs:3103:    fn fixture_complete_set_mint_tx() -> CompleteSetMintTx {
src/state/typed_tx.rs:3104:        CompleteSetMintTx {
src/state/typed_tx.rs:3107:            event_id: EventId(TaskId("task-fixture-tb13-mint".into())),
src/state/typed_tx.rs:3115:    fn fixture_complete_set_redeem_tx() -> CompleteSetRedeemTx {
src/state/typed_tx.rs:3116:        CompleteSetRedeemTx {
src/state/typed_tx.rs:3119:            event_id: EventId(TaskId("task-fixture-tb13-redeem".into())),
src/state/typed_tx.rs:3121:            outcome: OutcomeSide::Yes,
src/state/typed_tx.rs:3122:            share_amount: ShareAmount::from_units(7_000_000),
src/state/typed_tx.rs:3128:    fn fixture_market_seed_tx() -> MarketSeedTx {
src/state/typed_tx.rs:3129:        MarketSeedTx {
src/state/typed_tx.rs:3132:            event_id: EventId(TaskId("task-fixture-tb13-seed".into())),
src/state/typed_tx.rs:3140:    /// TB-13 U1: CompleteSetMintTx round-trips through canonical encode.
src/state/typed_tx.rs:3146:        assert_eq!(tx, decoded, "CompleteSetMintTx round-trip mismatch");
src/state/typed_tx.rs:3153:    /// TB-13 U2: CompleteSetRedeemTx round-trips through canonical encode.
src/state/typed_tx.rs:3159:        assert_eq!(tx, decoded, "CompleteSetRedeemTx round-trip mismatch");
src/state/typed_tx.rs:3166:    /// TB-13 U3: MarketSeedTx round-trips through canonical encode.
src/state/typed_tx.rs:3172:        assert_eq!(tx, decoded, "MarketSeedTx round-trip mismatch");
src/state/typed_tx.rs:3179:    /// TB-13 U4: OutcomeSide repr discriminants stable.
src/state/typed_tx.rs:3182:        assert_eq!(OutcomeSide::Yes as u8, 0);
src/state/typed_tx.rs:3183:        assert_eq!(OutcomeSide::No as u8, 1);
src/state/typed_tx.rs:3186:    /// TB-13 U5: ShareAmount default is zero.
src/state/typed_tx.rs:3189:        assert_eq!(ShareAmount::default(), ShareAmount::zero());
src/state/typed_tx.rs:3190:        assert_eq!(ShareAmount::default().units, 0u128);
src/state/typed_tx.rs:3193:    /// TB-13 U6: deterministic canonical_digest — same payload twice yields
src/state/typed_tx.rs:3211:    /// TB-13 U7: signing payloads exclude the `signature` field — exact
src/state/typed_tx.rs:3234:    /// TB-13 U8: HasSubmitter projects to the wire owner / provider.

 succeeded in 268ms:
  1600	        assert!(s_same.contains("Total Short: 20000 micro"));
  1601	        assert!(s_same.contains("exposure rows: 2"));
  1602	
  1603	        // Case 4: 2 nodes → per-node aggregation block renders.
  1604	        let two_nodes = vec![
  1605	            make_long("work-C", "work-C", "solver-C", 75_000),
  1606	            make_long("work-D", "work-D", "solver-D", 25_000),
  1607	            make_short("chal-D", "work-D", "challenger-D", 10_000),
  1608	        ];
  1609	        let s_two = render_section_13(&two_nodes);
  1610	        assert!(s_two.contains("Per-node exposure aggregation"));
  1611	        // node "work-C": long=75000, short=0, net=75000
  1612	        assert!(s_two.contains("work-C"));
  1613	        // node "work-D": long=25000, short=10000, net=15000
  1614	        assert!(s_two.contains("work-D"));
  1615	        assert!(s_two.contains("Total Long: 100000 micro"));
  1616	        assert!(s_two.contains("Total Short: 10000 micro"));
  1617	        assert!(s_two.contains("exposure rows: 3"));
  1618	
  1619	        // FORBIDDEN tokens (architect §9.4): must NOT appear in dashboard
  1620	        // (this catches accidental drift if a future patch adds price/trading
  1621	        // language to §13 rendering).
  1622	        for forbidden in &[
  1623	            "Open market balances",
  1624	            "MarketBuy",
  1625	            "MarketSell",
  1626	            "MarketOrder",
  1627	            "MarketTrade",
  1628	            "price_yes",
  1629	            "price_no",
  1630	            "automatic liquidity",
  1631	            "ghost liquidity",
  1632	        ] {
  1633	            assert!(
  1634	                !s_two.contains(forbidden),
  1635	                "architect §9.4 forbidden token '{forbidden}' must NOT appear in §13 render"
  1636	            );
  1637	        }
  1638	    }
  1639	}

exec
/bin/bash -lc 'rg -n "use crate::prediction_market::|use crate::prediction_market;|crate::prediction_market::BinaryMarket|crate::prediction_market::MarketError" src/bin/audit_dashboard.rs src/state/typed_tx.rs src/state/q_state.rs src/state/sequencer.rs src/economy/monetary_invariant.rs src/runtime/verify.rs' in /home/zephryj/projects/turingosv4
 exited 1 in 160ms:
 exited 1 in 224ms:
codex
There is a new bypass shape: the closure returns only the individual non-marker line that contains a TB-13 type name, so a multiline item can put `CompleteSetMintTx` on one line and `-> f64` or `let x: f64` on a neighboring line outside the returned set. I’m checking whether that should be raised as the round-6 verdict rather than treating the acknowledged “zero type-name helper” gap as the only residual.
exec
/bin/bash -lc 'test -e handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md && ls -l handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc 'test -e handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.raw && ls -l handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.raw' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
-rw-r--r-- 1 zephryj zephryj 103260 May  3 10:04 handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md.raw

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '166,216p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   166	/// Lines to scan for Layer 2 forbidden tokens.
   167	///
   168	/// **Codex round-4 RQ6 (2026-05-03)**: `tb_13_spans()` returns nothing
   169	/// for files added to scope by `discover_by_type_use` (no marker = no
   170	/// span), so the marker-only Layer 2 missed unmarked TB-13 contributors.
   171	///
   172	/// **Codex round-5 (R5) PARTIAL-MARKER (2026-05-03)**: round-6's
   173	/// either/or rule (marker-file → spans-only; unmarked file → all
   174	/// non-comment lines) left a hole: a marker-bearing file could hide
   175	/// non-marker TB-13 type-use plus f64/AMM tokens outside any marker
   176	/// span. Fix: for marker-files, scan marker-spans UNION any non-comment
   177	/// line that contains a TB-13 type name (catches stealth TB-13 type-uses
   178	/// outside marker spans — those lines ARE TB-13 contributions by
   179	/// definition because they reference TB-13-introduced types).
   180	///
   181	/// Final rules:
   182	/// - Marker-file: marker-spans ∪ non-comment lines containing TB-13 type names.
   183	/// - Unmarked-discovered file: all non-comment lines (round-6 behavior).
   184	///
   185	/// Residual gap (acknowledged): a TB-13 helper that uses zero TB-13 type
   186	/// names AND lives outside marker spans (e.g., a generic math helper
   187	/// called only by TB-13 code). Without a code-marker AND without a
   188	/// type-name signal, the fence has no way to identify it as TB-13. This
   189	/// is a defense-in-depth limit of marker+type-name discipline; manual
   190	/// code review remains the residual halt-trigger guard.
   191	fn tb_13_scan_lines(source: &str) -> Vec<(usize, String)> {
   192	    use std::collections::BTreeMap;
   193	    let has_marker = source.lines().any(is_tb_13_authoring_marker);
   194	    if has_marker {
   195	        // Marker-file: marker-spans ∪ non-comment lines with TB-13 type names.
   196	        let mut acc: BTreeMap<usize, String> = BTreeMap::new();
   197	        for (n, l) in tb_13_spans(source) {
   198	            acc.insert(n, l);
   199	        }
   200	        for (i, line) in source.lines().enumerate() {
   201	            if is_pure_comment_line(line) {
   202	                continue;
   203	            }
   204	            if TB_13_TYPE_NAMES.iter().any(|t| line.contains(t)) {
   205	                acc.insert(i + 1, line.to_string());
   206	            }
   207	        }
   208	        return acc.into_iter().collect();
   209	    }
   210	    // Unmarked-discovered file: all non-comment lines.
   211	    source
   212	        .lines()
   213	        .enumerate()
   214	        .filter(|(_, line)| !is_pure_comment_line(line))
   215	        .map(|(i, line)| (i + 1, line.to_string()))
   216	        .collect()

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '471,503p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   471	/// SG-13.0.2 — `no_f64_in_complete_set_or_market_seed`. Now uses
   472	/// effective_layer_2_scope() (discovered-only, per Codex R5 DASHBOARD-FLOOR
   473	/// remediation 2026-05-03 — Layer 2 tokens like f64 can appear in
   474	/// non-TB-13 baseline code for unrelated reasons).
   475	///
   476	/// Architect §4.2 halting trigger: HALT if `f64` appears in NEW
   477	/// CompleteSet / MarketSeed code. Money-path types must use integer
   478	/// `MicroCoin` / `ShareAmount`.
   479	#[test]
   480	fn no_f64_in_complete_set_or_market_seed() {
   481	    let mut violations: Vec<String> = Vec::new();
   482	    let f64_tokens = [" f64", "f64,", "f64;", "f64)"];
   483	    for rel in &effective_layer_2_scope() {
   484	        let source = read_scope_file(rel);
   485	        // tb_13_scan_lines: marker-files → spans ∪ non-marker TB-13-type-use
   486	        // lines (Codex R5 PARTIAL-MARKER); unmarked-discovered files →
   487	        // all non-comment lines (Codex R4 RQ6).
   488	        for (line_no, line) in tb_13_scan_lines(&source) {
   489	            for token in &f64_tokens {
   490	                if line.contains(token) {
   491	                    violations.push(format!(
   492	                        "{rel}:{line_no}: TB-13-scope contains f64 (`{token}`) — {line}"
   493	                    ));
   494	                }
   495	            }
   496	        }
   497	    }
   498	    assert!(
   499	        violations.is_empty(),
   500	        "TB-13 SG-13.0.2 no-f64-in-money-path violated:\n{}",
   501	        violations.join("\n")
   502	    );
   503	}

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '707,735p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   707	/// Round-7 R5-Codex PARTIAL-MARKER remediation 2026-05-03: a
   708	/// marker-bearing file with stealth TB-13 type-use OUTSIDE any marker
   709	/// span must still have those non-marker type-use lines scanned.
   710	#[test]
   711	fn tb_13_scan_lines_partial_marker_catches_stealth_type_use() {
   712	    // Marker-file: one marker-span at top + a TB-13 type use OUTSIDE the
   713	    // marker span (no TB-13 marker on the second function). Round-6
   714	    // helper would have only scanned the marker span; round-7 must also
   715	    // return the non-marker line containing `CompleteSetMintTx`.
   716	    let src = "\
   717	//! TB-13 module header.\n\
   718	pub fn tb13_marked() -> i32 { 0 }\n\
   719	\n\
   720	fn stealth(_: CompleteSetMintTx) -> f64 { 0.0_f64 }\n\
   721	";
   722	    let scanned = tb_13_scan_lines(src);
   723	    let scanned_text: Vec<&str> =
   724	        scanned.iter().map(|(_, l)| l.as_str()).collect();
   725	    assert!(
   726	        scanned_text.iter().any(|l| l.contains("tb13_marked")),
   727	        "marker-span line must be returned"
   728	    );
   729	    assert!(
   730	        scanned_text
   731	            .iter()
   732	            .any(|l| l.contains("CompleteSetMintTx") && l.contains("f64")),
   733	        "non-marker line containing TB-13 type name must also be returned (PARTIAL-MARKER closure)"
   734	    );
   735	}

exec
/bin/bash -lc "nl -ba tests/tb_13_legacy_cpmm_forward_fence.rs | sed -n '344,383p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   344	/// Layer 1 fence scope = FLOOR ∪ discovered. Deduplicated, sorted.
   345	/// Used by `legacy_cpm_api_not_imported_by_complete_set` Layer 1
   346	/// (HARD_BANNED_LEGACY_IMPORTS unconditional whole-file scan).
   347	///
   348	/// Layer 1 is broader than Layer 2 because legacy imports are forbidden
   349	/// EVERYWHERE in TB-13-relevant scope, regardless of whether the file
   350	/// carries TB-13 markers or type uses today. `audit_dashboard.rs` lives
   351	/// here because it is TB-13-relevant scope (Atom 4 §13/§14 dashboard
   352	/// renders TB-13 state), even though its current contributions are TB-12.
   353	fn effective_fence_scope() -> Vec<String> {
   354	    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
   355	    for s in FENCE_SCOPE_FLOOR {
   356	        set.insert((*s).to_string());
   357	    }
   358	    for s in discover_tb_13_files() {
   359	        set.insert(s);
   360	    }
   361	    set.into_iter().collect()
   362	}
   363	
   364	/// Layer 2 fence scope = discovered only (marker OR type-use).
   365	/// Used by Layer 2 forbidden-token scan + `no_f64_in_complete_set_or_market_seed`.
   366	///
   367	/// **Codex round-5 (R5) DASHBOARD-FLOOR remediation 2026-05-03**:
   368	/// narrower than Layer 1 because Layer 2 tokens (f64 / AMM / orderbook /
   369	/// price names) can legitimately appear in non-TB-13 files for unrelated
   370	/// reasons (e.g., negative-list test fixtures in `audit_dashboard.rs`
   371	/// at line 1628 that BAN those tokens — not USE them). Restricting
   372	/// Layer 2 to discovered files (i.e., files that actually contribute
   373	/// TB-13 code via marker OR TB-13 type use) prevents false positives on
   374	/// non-TB-13 baseline code that happens to mention forbidden token
   375	/// names. `audit_dashboard.rs` will auto-enter this scope when TB-14
   376	/// ships dashboard contributions there.
   377	fn effective_layer_2_scope() -> Vec<String> {
   378	    let mut set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
   379	    for s in discover_tb_13_files() {
   380	        set.insert(s);
   381	    }
   382	    set.into_iter().collect()
   383	}

codex
I’m going to write the audit report with a CHALLENGE verdict. The dashboard split verifies as claimed, but the partial-marker closure is still line-granular rather than item/block-granular, so a direct multiline TB-13 type-use plus `f64` bypass remains cheap to express.
2026-05-03T10:05:43.965767Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
codex
I could not save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md`: the write attempt was rejected by the read-only sandbox. Audit verdict:

## VERDICT: CHALLENGE
- TB13-Q9/RQ6-PARTIAL-MARKER-MULTILINE CHALLENGE: round-7 only adds the non-marker line containing a TB-13 type name to Layer 2 scan output, so a marker-bearing file can split `CompleteSetMintTx` and `f64` across adjacent lines and evade SG-13.0.2. See [tests/tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:191), line 204, line 488, and line 490.
- conviction: high
- recommendation: FIX-THEN-PROCEED

The added same-line test at [line 720](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:720) proves `fn stealth(_: CompleteSetMintTx) -> f64` is caught, but not this common multiline shape:

```rust
fn stealth(
    _: CompleteSetMintTx,
) -> f64 {
    0.0_f64
}
```

That is not the acknowledged “zero TB-13 type names” helper gap at [line 185](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:185); the item does use a TB-13 type, the forbidden token is just on a neighboring line.

DASHBOARD-FLOOR verifies: `audit_dashboard.rs` is back in Layer 1 floor scope at [line 70](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:70), Layer 1 scans `effective_fence_scope()` at [line 416](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:416), and Layer 2 is discovered-only at [line 377](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:377). The false-positive source remains the dashboard negative-list fixture at [src/bin/audit_dashboard.rs:1622](/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs:1622).

I did not rerun `cargo test --workspace` in this read-only session.
tokens used
58,470
I could not save `handover/audits/CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md`: the write attempt was rejected by the read-only sandbox. Audit verdict:

## VERDICT: CHALLENGE
- TB13-Q9/RQ6-PARTIAL-MARKER-MULTILINE CHALLENGE: round-7 only adds the non-marker line containing a TB-13 type name to Layer 2 scan output, so a marker-bearing file can split `CompleteSetMintTx` and `f64` across adjacent lines and evade SG-13.0.2. See [tests/tb_13_legacy_cpmm_forward_fence.rs](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:191), line 204, line 488, and line 490.
- conviction: high
- recommendation: FIX-THEN-PROCEED

The added same-line test at [line 720](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:720) proves `fn stealth(_: CompleteSetMintTx) -> f64` is caught, but not this common multiline shape:

```rust
fn stealth(
    _: CompleteSetMintTx,
) -> f64 {
    0.0_f64
}
```

That is not the acknowledged “zero TB-13 type names” helper gap at [line 185](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:185); the item does use a TB-13 type, the forbidden token is just on a neighboring line.

DASHBOARD-FLOOR verifies: `audit_dashboard.rs` is back in Layer 1 floor scope at [line 70](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:70), Layer 1 scans `effective_fence_scope()` at [line 416](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:416), and Layer 2 is discovered-only at [line 377](/home/zephryj/projects/turingosv4/tests/tb_13_legacy_cpmm_forward_fence.rs:377). The false-positive source remains the dashboard negative-list fixture at [src/bin/audit_dashboard.rs:1622](/home/zephryj/projects/turingosv4/src/bin/audit_dashboard.rs:1622).

I did not rerun `cargo test --workspace` in this read-only session.
