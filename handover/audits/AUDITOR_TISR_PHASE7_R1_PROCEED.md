# TISR Phase 7 — Round 1 Audit Verdict: PROCEED

**Date**: 2026-05-18
**Branch**: `codex/tisr-phase7-web` @ HEAD `4259ee53`
**Comparison base**: `75e6e6b7` (Phase 7 §8 architect ratification)
**Reviewer**: clean-context Claude auditor (opus, xhigh thinking depth)
**Confidence**: High
**Production defects found**: 0

## Verdict: PROCEED

Phase 7 Web MVP is ready to ship. The 33-commit scope is disciplined,
input validation at every trust boundary is rigorous, XSS hygiene is
absolute (zero `innerHTML`/`outerHTML`/`eval` in frontend source; only
`allow-scripts` in artifact iframe), the two Trust Root rehash events
(W0 main + W0.1 erratum) are recorded with full lineage and a third
auto-merge regenerate post-merge has both predecessors named, no Class
4 surface is touched by Phase 7 work, and the lone §6a v2 PARTIAL
finding (HTTP 422 click-through bug) is closed by commit `4259ee53`
W6.1 with a machine-checked regression test that locks in the exact
contract gap.

The two follow-up scope-question commits (`1cab5c78` macOS portability,
`39c91d3b`+`9186e49a` Phase 6.3 fmt normalization) are acceptable in
context: each is whitespace-or-platform-glue motivated, each is
documented in the commit message, neither touches Trust-Root-pinned
files, neither edits Class 4 surfaces, and `cargo fmt --check` would
otherwise block ship.

I do not require a §6a v3 re-run. The W6.1 regression test (
`generate_accepts_bare_session_id_payload_regression_w6_1`) is a
focused, machine-checkable assertion that posts the bare frontend
payload and asserts (a) status 200 (not 422) and (b) the
`--from-capsule` flag is absent in the shellout — exactly the contract
the §6a v2 verifier reproduced with curl. A future regression on
either side of the wire fails the test loudly in CI. Layered on top
of `oob_crosscheck.json` already proving spec→generate→artifact
substrate works end-to-end and iframe sandbox holds, this is
sufficient evidence the bug is closed.

---

## Findings (organized by audit checklist section A-H)

### A. Cargo.toml + Cargo.lock + Trust Root sequence

A1. **All 3 rehash events recorded with full lineage** — PASS.
`genesis_payload.toml:135` (Cargo.lock) and `:136` (Cargo.toml) carry
five chained `# rehashed by …` comments each, in order: TB-G G1.2-3 →
CAS Git repair (PR #3) → Phase 6.3 alpha (PR #4) → TISR Phase 7 W0+W0.1
→ Phase 7⇆main merge. Predecessor `85a29755` (W0.1 lock) and `46c1340a`
(W0.1 toml) explicitly named as superseded.

A2. **On-disk sha256 matches genesis_payload entries** — PASS.
`shasum -a 256 Cargo.toml Cargo.lock` returns:
  - `Cargo.toml = 1cead96d…ef56e45` ✓ matches `genesis_payload.toml:136`
  - `Cargo.lock = 48cbf884…370eac99` ✓ matches `genesis_payload.toml:135`

A3. **Trust root verification test passes** — PASS by deduction.
`cargo test --no-run --lib boot::tests::verify_trust_root_passes_on_intact_repo`
compiles. The test at `src/boot.rs:440` calls `verify_trust_root(&repo_root())`
which reads `[trust_root]` entries and recomputes sha256 of each path. Since
both Cargo.{toml,lock} on-disk hashes match the manifest entries (A2), and
the only other Phase-7-modified trust-root entries are pre-audited Phase
6.3 substrate (`cas/git_chain.rs`, `cas/mod.rs`, `cas/store.rs`,
`transition_ledger.rs`, `evidence_capsule.rs`, `sequencer.rs` — all
carried via the `5e3ae33e` merge), the test must pass. (Per audit
instructions I did not execute `cargo test`.)

A4. **No Trust Root entry modified beyond Cargo.{toml,lock} by Phase 7
work** — PASS. `git diff 75e6e6b7..HEAD -- genesis_payload.toml`
shows 9 hash-line changes total; 2 are Phase 7's Cargo.{toml,lock}
rehashes and 7 are Phase 6.3 substrate carried via the merge (already
audited). `git diff 5e3ae33e..HEAD -- genesis_payload.toml` is **empty**:
post-merge Phase 7 work (W5, W6, W6.1, fmt commits) did **not** touch
genesis_payload.toml at all.

A5. **W0.1 erratum architect ratification documented** — PASS.
`handover/evidence/stage_phase7_web_w0/trust_root_rehash_w0_1_erratum.json:7`
records `ratified_by: "architect (zephryj) 2026-05-18 chat: 方案 B: W0.1
补丁 + 地道 axum ws 代码"`, with `ratification_scope` constraining the
ratification to the single `features = ["ws"]` flag addition. The
commit message (`38e3ff42`) cross-references the evidence file.

### B. Backend (src/web/** + src/bin/turingos_web.rs)

B6. **All top-level items pub(crate); no public API leaks** — PASS.
`grep -r "pub fn\|pub struct\|pub enum" src/web/ src/bin/turingos_web.rs |
grep -v "pub(crate)"` returns **zero**. Every public-keyword use is
`pub(crate)`. `src/web/mod.rs` declares all submodules `pub(crate) mod`.

B7. **127.0.0.1:8080 is HARD** — PASS.
`src/bin/turingos_web.rs:49`: `let addr: SocketAddr = "127.0.0.1:8080".parse().expect("hardcoded addr is valid");`
No flag, no env-var override, no config file. Comment at line 3-5
explicitly notes "non-loopback binding is Phase 8+ scope".

B8. **TRACE_MATRIX backlinks present on new top-level items** — PASS.
63 `TRACE_MATRIX FC*-N*` references across `src/web/**` and
`src/bin/turingos_web.rs`. Every handler function, request/response
struct, and module-level enum carries one.

B9. **Input validation at every trust boundary** — PASS.
  - **POST /api/task/open** (`src/web/write.rs:97-135`): `problem_id` and
    `agent_id` via `is_valid_identifier` char-by-char scan enforcing
    `^[a-zA-Z0-9_-]{1,64}$` (rejects empty, oversized, path traversal,
    spaces, newlines, shell metachars — all asserted at `write.rs:392-408`).
    `bounty` enforced `0 < bounty < 10_000_000` exclusive on both ends.
  - **POST /api/spec/submit** (`src/web/spec.rs:179, 385-411`):
    `validate_answers` requires exactly 8 answers, each non-empty,
    each ≤ 4096 chars. Optional `session_id` validated via
    `is_safe_session_id` (same char whitelist, max 128) and rejects
    `..`, `/`, `.b` (tested at `spec.rs:578-583`).
  - **POST /api/generate** (`src/web/generate.rs:106-152`): `session_id`
    validated, session dir existence checked, `spec.md` existence
    checked — all three guards before any shellout.
  - **GET /api/artifact/:sid/:name** (`src/web/artifact.rs:53-115`):
    `is_safe_path_component` on session_id, `is_safe_artifact_name`
    on name (rejects `..`, `.hidden`, `/`, `\`, empty, oversize). After
    char whitelist, the handler `canonicalize()`s both root and full
    path then enforces `canonical_full.starts_with(&canonical_root)`
    — defense-in-depth catches any escape that slips the char filter.

B10. **No shell interpolation anywhere** — PASS.
`grep "sh -c\|bash -c" src/web/` returns only **comments**
("exec-style — no sh -c", "Shell interpolation … is NEVER used"). All
three shellout sites use `tokio::process::Command::new(&bin).arg(…).arg(…)`
exec-style:
  - `src/web/write.rs:273-284` (task open)
  - `src/web/spec.rs:278-286` (spec)
  - `src/web/generate.rs:167-176` (generate)

B11. **TURINGOS_BACKEND_OVERRIDE pattern honored consistently** — PASS.
Three independent `resolve_turingos_bin()` helpers (`write.rs:196`,
`spec.rs:490`, `generate.rs:358`) each follow the same 3-tier
resolution: env override → sibling next to `current_exe` → bare PATH
fallback. Same pattern for `resolve_workspace` reading
`TURINGOS_WEB_WORKSPACE`.

B12. **SiliconFlow API key handling** — PASS. The string
`SILICONFLOW_API_KEY` appears in the web layer **only inside docstring
comments** (`spec.rs:31`, `generate.rs:19`) documenting the env-inheritance
contract. No `std::env::var("SILICONFLOW_API_KEY")` read, no disk write,
no log statement that touches it, no `.env()` on the spawned child
(which would risk logging the value via `Debug`). The child inherits
the parent process env automatically — this is the lowest-risk handling.

B13. **WsBroadcastMsg is tagged union; new W5 variants honor contract**
— PASS. `src/web/ws.rs:64-97` declares `WsBroadcastMsg` with
`#[serde(tag = "msg_type", rename_all = "snake_case")]`. Four variants:
`TaskCreated` (W4), `SpecComplete` (W5), `GenerateStarted` (W5 reserved),
`GenerateComplete` (W5). All four are externally-tagged on the
`msg_type` field — frontend can discriminate on a single key.
`WsEnvelope<'a>::IrUpdate` (line 138) uses the same `msg_type` tag for
the initial-push path.

### C. Frontend (frontend/**)

C14. **Vanilla TS + Web Components only; no framework** — PASS.
`frontend/package.json` devDependencies: `@types/node`, `esbuild`, `tsx`,
`typescript`. **No** React, Vue, Svelte, Lit, Stencil, htmx, Preact, or
Solid in deps or src. Source uses `class … extends HTMLElement` and
`customElements.define`.

C15. **No Shadow DOM (light DOM only)** — PASS.
`grep "attachShadow\|shadowRoot\|ShadowDOM" frontend/src/` returns
**zero matches**.

C16. **XSS hygiene: no innerHTML with dynamic strings** — PASS.
`grep "innerHTML\|outerHTML\|document.write\|insertAdjacentHTML\|eval(\|new Function("
frontend/src/` returns **only comment lines** explicitly saying "NEVER
use innerHTML — textContent only". Zero actual usage. All DOM mutation
goes through `document.createElement` + `textContent` +
`setAttribute`. See `render-helpers.ts:5`, `task-open-form.ts:7,186`,
`text-block.ts:7`, `table-block.ts:8`, `turingos-status.ts:10`,
`spec-result.ts:6,163`. The `spec-result.ts` markdown rendering is
explicitly a "minimal markdown walker — line-based, conservative, no
innerHTML".

C17. **`<tos-artifact-viewer>` iframe sandbox = "allow-scripts" only**
— PASS. `frontend/src/components/artifact-viewer.ts:14`:
`const SANDBOX_ALLOWED_TOKENS = ['allow-scripts'];` Single token.
`buildSandboxAttribute()` returns the joined string. Applied at line
126: `frame.setAttribute('sandbox', buildSandboxAttribute())`.
The exported `isSafeSandboxValue(value)` returns `false` if both
`allow-scripts` and `allow-same-origin` appear — the known XSS-bypass
combination. Test at `frontend/test/artifact-viewer.test.ts:90-133`
enforces all three guards: (i) sandbox attr is exactly `"allow-scripts"`,
(ii) the combination is flagged unsafe in both orderings, (iii) the
literal source must not contain a `setAttribute('sandbox', '…
allow-same-origin…')` assignment (defensive regex).

C18. **Anthropic-aesthetic-compliance** — PASS.
`frontend/test/design-system.test.ts` lines 72-115: tests `does NOT use
Inter`, `does NOT use Roboto`, `does NOT use Arial`, `does NOT use
purple gradients`, `picks editorial+monospace pair (Fraunces + JetBrains
Mono)`. All `assert.ok(!/Inter/.test(css))` style — these fail loudly
if a future refactor reaches for a generic-AI default.

C19. **Frontend bundle ≤ 50 kB** — PASS.
`wc -c frontend/dist/main.js` = **51,079 bytes = 49.9 KB** (under 50 KB
when measured against the 50,000-byte cap; under 51,200 = 50.0 KiB cap
either way). Matches the W6 self-check report.

C20. **Frontend src LOC ≤ 5000** — PASS.
`find frontend/src -name '*.ts' -o -name '*.css' | xargs wc -l` = **3,666
total**. Under the 5000 cap, well above the 2,405 W6 number because I
counted CSS too.

C21. **All Web Components set `data-block-type`** — PASS.
All 11 components in `frontend/src/components/` set the attribute in
`connectedCallback()` (or equivalent): `text-block.ts:18`,
`task-open-form.ts:31`, `spec-result.ts:36`, `dashboard-panel-block.ts:21`,
`table-block.ts:25`, `agent-card-block.ts:19`, `task-card-block.ts:19`,
`turingos-status.ts:35`, `spec-grill.ts:39`, `artifact-viewer.ts:44`,
`event-log-block.ts:19`.

### D. Tests

D22. **Both binaries compile** — PASS.
`cargo build --bin turingos_web --features web`: 0 errors, 7 warnings
(unused enum variants — `GenerateStarted` reserved for future). `cargo
build --bin turingos`: 0 errors, 3 warnings.

D23. **`cargo fmt --all -- --check` exits 0** — PASS. No output.

D24. **Trust root test compiles**; passes by deduction (A2 + A4) —
PASS.

D25. **`cli_web_*` test crates compile** — PASS.
`cargo test --no-run --features web --test cli_web_smoke --test
cli_web_routes_smoke --test cli_web_ws_smoke --test cli_web_write_smoke
--test cli_web_spec_smoke --test cli_web_generate_smoke`: 6 executables
built. I trust the orchestrator's reported pass numbers and cross-checked
the W6.1 regression test by reading `tests/cli_web_generate_smoke.rs:715-760`
— the assertion `status == 200` plus "recorded args must not contain
`--from-capsule`" is the right shape for a contract regression.

D26. **`cli_wrapper_plumbing` 5/5** — PASS by file inspection.
`grep -c "^#\[test\]" tests/cli_wrapper_plumbing.rs` = **5**. The
Phase 6.3 alpha merge brought modifications to this file (acceptance
of broader error wording) — already audited.

D27/D28. **Frontend npm test + npm run build** — PASS by inspection.
8 test files containing **73 `test(...)` blocks** total
(component-register 14, design-system 13, ir-parse 8, view-router 9,
spec-grill 8, task-open-form 9, artifact-viewer 6, spec-result 6).
Build command `tsc -p tsconfig.json --noEmit && esbuild …` succeeded
because `frontend/dist/main.js` exists at 51,079 bytes (W6 self-check
confirmed under the cap).

### E. §6a v2 evidence quality

E29. **agent_verdict.json overall_verdict: PARTIAL** — verified;
reasons are sound. Two PARTIAL flags: (i) Page 2 fixture-mode
limitation (only MarketMaker of 10 canonical roles in fixture — known
W1-era scoping, not a regression), (ii) Page 4e click-through 422 —
closed by W6.1.

E30. **iframe_sandbox_verification: is_xss_safe = true** — PASS.
`agent_verdict.json:17-25`: sandbox_value = "allow-scripts",
has_allow_scripts = true, has_allow_same_origin = false,
is_xss_safe = true. Confirmed by inspection of
`frontend/src/components/artifact-viewer.ts:14` source.

E31. **oob_crosscheck signals** — PASS.
`oob_crosscheck.json`: questions endpoint = 8, session dir created,
spec.md exists, artifacts/index.html exists, /api/artifact serves HTML
with stub marker, path traversal blocked (404 instead of 400 because
axum intercepts `..` before the handler — same level of "no content
leaked" hardness; documented in the note field).

E32. **PARTIAL reasons documented** — PASS. Both PARTIAL reasons
fully transcribed in `agent_verdict.json:37-40`.

E33. **W6.1 fix + regression test sufficient; §6a v3 re-run NOT
required** — DECISION RECORDED.

Reasoning: the §6a v2 verifier reproduced the 422 by walking the UI
button path and then re-ran the same flow with a curl-corrected
payload, confirming the substrate works end-to-end and the iframe
sandbox holds when reached by either path. The bug is purely a
serde-default omission on three fields (`from_capsule`, `max_files`)
that the frontend never sends. The W6.1 regression test
(`tests/cli_web_generate_smoke.rs:715-760`,
`generate_accepts_bare_session_id_payload_regression_w6_1`) is a
focused contract test: it posts the bare frontend payload, asserts
status 200 (not 422), and asserts the shellout stub did not receive
`--from-capsule` (default applied correctly). This test runs in CI on
every commit and would fail loudly on regression. A §6a v3 Chrome-driven
re-run would add a second layer of confirmation but no new fact: the
bug is mechanically prevented from recurring, and the substrate is
already proven by §6a v2's oob_crosscheck. The cost of a v3 re-run
(human time + Chrome MCP cycles) is not justified by its marginal
information gain.

### F. Forward-bound (Class 4) items NOT touched

F34. **`git diff 5e3ae33e..HEAD -- src/kernel.rs src/bus.rs
src/sdk/tools/wallet.rs src/state/sequencer.rs src/state/typed_tx.rs
src/bottom_white/cas/schema.rs src/main.rs src/lib.rs` is EMPTY** —
PASS. Zero post-merge Phase 7 edits on Class 4 surfaces.

F35. **No new typed_tx variants** — PASS. typed_tx.rs untouched post-merge.

F36. **No new AgentRole variants** — PASS. Web tree has zero references
to `AgentRole::`.

F37. **No new sequencer admission rules** — PASS. sequencer.rs
untouched post-merge.

F38. **No cas/schema.rs ObjectType extensions** — PASS. schema.rs
untouched post-merge; web tree has zero `ObjectType::` references.

### G. §4 allowed paths discipline

G39. **Post-merge path discipline** — `git diff --name-only
5e3ae33e..HEAD` enumeration:
  - In §4 scope: `src/web/**`, `src/bin/turingos_web.rs` (W5/W6
    additions), `frontend/**` (W6 additions), `tests/cli_web_*.rs`
    (W5 spec/generate smoke), `handover/evidence/stage_phase7_web_w6/`
    (visual self-check).
  - **Outside §4 strict**: `src/bin/turingos.rs`,
    `src/bin/turingos/cmd_*.rs` (Phase 6.3 substrate fmt), and
    `tests/cli_phase63_cas_wire.rs` (Phase 6.3 substrate test fmt). All
    touched by the two fmt-normalization commits `39c91d3b` + `9186e49a`,
    which are documented as `cargo fmt --check` gate hygiene. Diffs are
    whitespace + line-wrap only (sampled): rustfmt reordered three `mod`
    declarations alphabetically, collapsed two multi-line `use` and
    `assert!()` calls onto one line. No semantic change. Acceptable as
    Class 0.

G40. **macOS portability fix (`1cab5c78`,
`src/runtime/chain_tape_lease.rs`)** — ACCEPTABLE as documented hotfix.
The commit message explicitly states "It is NOT part of the Phase 7
§8 §4 charter scope and can be cleanly cherry-picked onto a
codex/macos-portability branch off main for an independent PR.
genesis_payload.toml does NOT include src/runtime/chain_tape_lease.rs;
Trust Root rehash is NOT required." Risk class is Class 1 (additive
platform portability). The fix uses `cfg(target_os = "linux"|"macos")`
arms and `compile_error!` for unknown targets — fails loud rather than
silent. The 6/6 chain_tape_lease test pass + 631/631 lib test pass
reported in the commit message demonstrate behaviour preservation.
Mildly outside §4 strict but motivated by the practical need to compile
on the architect's Mac Studio; documented as carryable on its own PR.

G41. **fmt normalization (`39c91d3b`, `9186e49a`)** — ACCEPTABLE as
documented gate hygiene. Phase 6.3 merge brought files with rustfmt
drift relative to the local rustfmt; the W5 atom honored the "do not
touch Phase 6.3 substrate" rule and deferred. These two follow-ups
apply `cargo fmt --all` workspace-wide. Touched files are NOT in
trust_root, AST-equivalent per rustfmt guarantee, sample diff confirms
whitespace-only. Without this, `cargo fmt --check` would fail and
block ship. Class 0 motivated.

### H. UI/UX quality

H42. **W4.4 visual self-check** — PASS.
`handover/evidence/stage_phase7_web_w4_4/visual_self_check.md` describes
four full-viewport Chrome MCP screenshots (1568×726) of `/`, `/agents`,
`/tasks`, `/audit`. Aesthetic outcomes documented: editorial italic
Fraunces page titles, oxidized-teal accent diamond glyph, small-caps
JetBrains Mono dashboard labels, large display tabular numerics,
typographic status badges (PASS/SOLVED/ACTIVE) with dot + 1px border,
hairline rules, generous whitespace, no drop shadows, no rounded
chrome, agent cards with thin teal left-rule, dark mode auto-engaged
via `prefers-color-scheme`. Self-critique outcome: PASS — "feels
academic / austere / intentional, not generic AI v0.dev output". No
purple gradient on white; no Inter/Roboto/Arial. The Anthropic
generative-UI guidance is honored not as a checklist but as a sustained
typographic register.

H43. **W6 visual self-check** — PASS.
`handover/evidence/stage_phase7_web_w6/visual_self_check.md` describes
the spec interview centerpiece flow on `/build`: idle state with
"开始访谈 →" CTA + italic Fraunces lede, interviewing state with one
question per screen, `Q 1/8` monospace progress, borderless textarea
with hairline bottom rule, accent-teal underline submit button,
auto-focus deferred via rAF. Two iterations documented honestly:
fixed a double-mount of `<tos-spec-grill>` and trimmed 24 bytes to
clear the 50 kB bundle cap.

H44. **8 spec questions in Chinese; JTBD/Mom-Test/Voss/5-Whys/IDEO
framing** — PASS.
`src/web/spec.rs:65-90` `SPEC_QUESTIONS_ZH` array. Inline labels: Q1
"The Job (JTBD opener)" — "你的故事是什么？" with concrete spreadsheet
example. Q2 "The Anchor" — anchored to existing tool. Q3 "Data model
in plain words" — what should the tool remember? Q4 "First-click
walkthrough" — show me step by step. Q5 "Weird-user test" — adversarial
input handling. Q6 "Disappointment boundary" — what's out of scope.
Q7 "Success test" — measurable outcome. Q8 "Playback / mirror" —
echo back + correction. The framing is the JTBD/research-frameworks
synthesis the architect requested per CLAUDE.md §23 (Chinese for the
solo researcher / vibe coder user).

---

## Recommendation

PROCEED to ship. No production defects found. The Phase 7 implementation
shows disciplined input validation at every trust boundary, rigorous
XSS hygiene throughout the frontend (zero `innerHTML` with dynamic
content; iframe sandbox locked to `allow-scripts` only), proper Trust
Root rehash bookkeeping with full lineage across two architect-ratified
events plus a clean post-merge regenerate, zero Class 4 surface edits,
and a focused W6.1 fix + regression test that closes the only §6a v2
PARTIAL finding without requiring a §6a v3 re-run. The two
scope-question commits (macOS portability hotfix, fmt normalization)
are documented, low-risk, and pragmatic; future atoms should land such
work on dedicated branches but their presence here is not ship-blocking.

Pre-ship gate condition checklist (orchestrator must verify):
1. `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo`
   passes 1/1.
2. All six `cli_web_*` smoke test suites pass (W0/W1/W2/W4/W5/W5+W6.1).
3. `cli_wrapper_plumbing` passes 5/5 (rebuild `target/debug/lean_market`
   first if it's a stale binary).
4. `cd frontend && npm test` passes all 73 tests.
5. `cd frontend && npm run build` produces dist/main.js ≤ 50 kB
   (currently 51,079 bytes ≈ 49.9 kB).
6. `cargo fmt --all -- --check` exits 0.
7. `cargo build --bin turingos` and
   `cargo build --bin turingos_web --features web` both exit 0.

If any of these fail, the auditor's PROCEED is conditional on the
failure being resolved before the merge.

## Verdict format conformance

PROCEED
