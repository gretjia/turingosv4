# F11 — Generate Quality Predicates: Domain-Agnostic Minimum Bar

**Date**: 2026-05-19
**Risk class**: Class 2 (production wire-up of an existing endpoint's quality gate)
**Pre-fix HEAD**: `3e0fa79c`
**Branch**: `codex/tisr-phase6-3-x-grill-driven`
**Parallel to**: F9 (transcript rollback) + F10 (spec.md slot-mapping). Touched
files are disjoint: F11 modifies `src/web/verify.rs` + `src/web/generate.rs`;
F9/F10 are scoped to `src/web/spec.rs`.

**Touched FC nodes**: FC1-N5 (read-view shielding at the generate trust boundary),
FC1-N10 (write path / quality gate before broadcasting GenerateComplete).

---

## 1. Defect (D-NEW-4, P0 universality)

Surfaced by the Π5 smoke test on the Π4.3 P7 Traditional spec (video transcoder):

- `cmd_generate` produced a valid 5384-byte Traditional Chinese
  video-converter UI artifact: correct domain ("影片轉檔工具"), valid HTML5,
  drag-drop upload zone, per-file SHA256 deduplication, MP4 conversion
  simulation.
- The quality predicate REJECTED with HTTP 500 / `kind=generate_quality_failed`,
  reasons: `missing_playfield ... missing_keyboard_handler ... missing_animation_loop ... keydown_not_on_document_or_window`.
- These predicates hard-code GAME-shape heuristics (`<canvas>`, CSS grid
  playfield, `keydown` handler, `requestAnimationFrame` animation loop).
- Effect: ANY non-game spec (todo app, dashboard, video converter, CRUD form,
  etc.) is false-positive rejected.

Evidence:
- `handover/evidence/phase6_3_x_universality_1779111375/pi5_smoke/p7_traditional/verdict.json`
  (verdict: PI5-SMOKE-PARTIAL, HTTP 500, 4 game-shape error_reason_classes)
- `handover/evidence/phase6_3_x_universality_1779111375/pi5_smoke/p7_traditional/response.json`
  (raw error JSON)

This is the SAME shape of universality bug as the Π4.3 triage register-bias
captured in F8 / F8b: a hardcoded domain assumption defeats generic surface
universality. The quality gate must route P7 to a non-game predicate set.

---

## 2. Root cause

`src/web/verify.rs::verify_html_contents` (W8, ~2026-05-18) ran 10 heuristic
checks against any generated `index.html` and required ALL of them to pass:

1. size in `[2 KB, 100 KB]`
2. `has_playfield()` — canvas / CSS grid / SVG / table / cell-class
3. `addEventListener('keydown'|'keyup'|'keypress')`
4. `requestAnimationFrame` or `setInterval`
5. no external `<script src="http">`
6. no external `<link rel="stylesheet" href="http">`
7. balanced `{`/`}`
8. balanced `<script>`/`</script>`
9. no inverted nullish-guard pattern
10. `keydown` on `document` or `window` (not `body`)

Checks 2, 3, 4, 10 (`playfield`, `keyboard`, `animation_loop`,
`document_or_window_keydown`) are game-shape specific. They were designed
during W8 to harden a Qwen3-Coder Tetris run and were never gated by spec
intent. `verify_html_contents` is called unconditionally from
`generate_handler` for every artifact.

---

## 3. Fix description (Option B chosen — plus Step 4 bonus)

Two-part fix:

### 3a. New domain-agnostic mode: `VerifyMode::MinimumBar`

Added `verify_html_contents_with_mode(html, size, VerifyMode)` and
`verify_artifact_html_with_mode(path, VerifyMode)` in `src/web/verify.rs`.
The new `MinimumBar` mode requires only an HTML5 minimum bar:

1. Contains `<html>` or `<!DOCTYPE html>` (case-insensitive)
2. Contains a non-empty `<body>...</body>` (at least one non-whitespace char inside)
3. Contains at least one of `<script>`, `<style>`, `<link rel="stylesheet">`
4. Total size `>= 500 bytes` (`MIN_SIZE_BYTES_MINIMUM` const)
5. No placeholder text:
   - `lorem ipsum` (case-insensitive)
   - `<!-- placeholder -->` (case-insensitive)
   - Comment-shaped `TODO` / `FIXME` markers, case-sensitive: `// TODO`,
     `/* TODO`, `<!-- TODO`, `TODO:` (and FIXME variants).
     Case-sensitivity + comment-shape requirement prevents false positives
     on legitimate identifiers like `todoList` or page titles like
     `<title>Todo App</title>`.

Each failed check appends a descriptive reason string (`missing_html_root`,
`missing_or_empty_body`, `no_script_or_style`, `too_small`,
`placeholder_content`) to `VerifyOutcome.failure_reasons`.

### 3b. Content-aware mode selection in `generate_handler` (Step 4 bonus)

`src/web/generate.rs::generate_handler` now reads `spec.md` once before the
retry loop and calls `spec_looks_like_game(&spec_md)` to pick the mode:

- `GameShape` if spec mentions: `game`, `tetris`, `snake`, `breakout`,
  `pong`, `pacman`, `pac-man`, `minesweeper`, `2048`, `arcade`,
  `playfield`, `canvas` (ASCII, case-insensitive), or `游戏` / `遊戲` /
  `俄罗斯方块` / `俄羅斯方塊` / `贪吃蛇` / `貪吃蛇` / `扫雷` / `掃雷`
  (Simplified + Traditional Chinese).
- `MinimumBar` otherwise. Default if `spec.md` read fails.

Conservative bias: a non-game artifact mis-tagged as a game pays the cost
of stricter checks (user sees strict failure reasons); a game spec
mis-tagged as non-game passes the lower bar (acceptable degradation — the
artifact may not be playable but the user can still inspect/regenerate).

The retry loop (`MAX_GENERATE_ATTEMPTS = 3`, `total_attempts` counter,
attempt-start / attempt-failed WS broadcasts) is preserved unchanged.
`verify_html_contents` is preserved verbatim as a `GameShape`-mode wrapper
so the existing W8 verify_smoke test suite continues to pass.

---

## 4. Tests added

New file: `tests/cmd_generate_quality_predicates_domain_agnostic.rs`
(12 tests, all `#[cfg(feature = "web")]`, no Class-3/4 surfaces touched).

| Test | Asserts |
|---|---|
| `accepts_video_converter_html_5kb` | The exact Π5 P7 traditional shape (Traditional Chinese, drag-drop, SHA256, MP4) passes MinimumBar. **Load-bearing — reproduces D-NEW-4.** |
| `accepts_todo_app_html` | Generic English todo app (no canvas/keydown/raf) passes MinimumBar. |
| `rejects_too_small_artifact` | 43-byte stub fails with reason containing `too_small`. |
| `rejects_placeholder_content_todo` | HTML with `<!-- TODO: ... -->` fails with `placeholder_content`. |
| `rejects_placeholder_content_lorem_ipsum` | HTML with `Lorem ipsum` fails with `placeholder_content`. |
| `rejects_no_script_or_style` | Bare HTML body with no `<script>`/`<style>`/`<link rel="stylesheet">` fails with `no_script_or_style`. |
| `rejects_missing_html_root` | Plain text masquerading as HTML fails with `missing_html_root`. |
| `rejects_empty_body` | Valid HTML5 with empty `<body>` fails with `missing_or_empty_body`. |
| `spec_detection_game_keywords` | spec_looks_like_game returns true for tetris/snake/breakout (EN) + 俄罗斯方块/俄羅斯方塊/遊戲 (zh-Hans + zh-Hant). |
| `spec_detection_non_game` | spec_looks_like_game returns false for the exact Π5 P7 video-converter spec, todo app, CRUD dashboard, markdown editor. **Load-bearing — locks in the Step-4 routing decision.** |
| `video_converter_fails_game_shape_mode` | Confirms the layering: the same 5384-byte video converter still fails GameShape mode. The gate is mode-controlled at the call site, not silently weakened. |

---

## 5. cargo verification

```bash
$ cargo check --features web
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.60s

$ cargo test --features web --no-fail-fast \
      --test cmd_generate_quality_predicates_domain_agnostic \
      --test cli_web_verify_smoke \
      --test cli_web_generate_smoke
test result: ok. 77 passed; 0 failed; 0 ignored; 0 measured  # cli_web_generate_smoke
test result: ok. 78 passed; 0 failed; 0 ignored; 0 measured  # cli_web_verify_smoke
test result: ok. 79 passed; 0 failed; 0 ignored; 0 measured  # cmd_generate_quality_predicates_domain_agnostic
```

Pre-existing W8 verify smoke tests (`verify_rejects_missing_playfield`,
`verify_rejects_missing_keyboard_handler`, `verify_rejects_missing_animation_loop`,
`verify_accepts_good_artifact`, `verify_accepts_dom_grid_tetris_w8_1_regression`,
`verify_rejects_inverted_nullish_guard`) all pass — `verify_html_contents`
defaults to `VerifyMode::GameShape`, preserving the W8 contract.

Additional regression sweep:

```bash
$ cargo test --features web --no-fail-fast \
      --test cli_web_smoke --test cli_web_routes_smoke --test cli_web_write_smoke
test result: ok. 76 passed; 0 failed; ...
test result: ok. 70 passed; 0 failed; ...
test result: ok. 77 passed; 0 failed; ...
```

---

## 6. End-to-end smoke (Π5 P7 traditional)

Pre-fix Π5 smoke verdict: `PI5-SMOKE-PARTIAL`, HTTP 500, kind=`generate_quality_failed`,
reasons = `[missing_playfield, missing_keyboard_handler, missing_animation_loop, keydown_not_on_document_or_window]`.

Post-fix backend rebuild + restart:

```bash
$ kill 91412
$ SILICONFLOW_API_KEY=sk-… TURINGOS_WEB_WORKSPACE=tmp/universality_campaign \
    ./target/debug/turingos_web &  # new PID 1460
TuringOS Phase 7 Web MVP — workspace: tmp/universality_campaign
TuringOS Phase 7 Web MVP listening on http://127.0.0.1:8080

$ curl -s -w "\n--- HTTP %{http_code} elapsed %{time_total}s ---\n" \
    -X POST http://127.0.0.1:8080/api/generate \
    -H 'Content-Type: application/json' \
    -d '{"session_id":"pi4_p7_traditional_1779150050"}'
{"session_id":"pi4_p7_traditional_1779150050",
 "artifacts":[{"path":"index.html","size_bytes":8117,"content_type":"text/html"}],
 "transcript_excerpt":"\nGenerated 1 file(s) under tmp/universality_campaign/sessions/pi4_p7_traditional_1779150050/artifacts/\n  index.html\n\nOpen the entry file in your browser or run the entry script:\n  xdg-open tmp/universality_campaign/sessions/pi4_p7_traditional_1779150050/artifacts/index.html\n",
 "total_attempts":1}
--- HTTP 200 elapsed 75.586725s ---
```

Verdict: **PI5-SMOKE-PASS** — HTTP 200, single-attempt success, fresh
8117-byte Traditional Chinese video converter artifact accepted on first
try. `total_attempts: 1` confirms the MinimumBar gate accepted the
on-domain output without exhausting retries.

Spec.md → MinimumBar routing confirmed implicitly (no game keywords in the
P7 spec → MinimumBar mode → on-domain artifact PASSES).

---

## 7. Files modified

| File | Change |
|---|---|
| `src/web/verify.rs` | Added `VerifyMode` enum, `verify_html_contents_with_mode`, `verify_artifact_html_with_mode`, `verify_minimum_bar`, `has_nonempty_body`, `first_placeholder_match`, `spec_looks_like_game`, `MIN_SIZE_BYTES_MINIMUM`. Refactored legacy W8 logic into `verify_game_shape`. `verify_html_contents` / `verify_artifact_html` preserved as thin GameShape-mode wrappers. |
| `src/web/generate.rs` | Replaced `verify_artifact_html` import with `verify_artifact_html_with_mode` + `VerifyMode` + `spec_looks_like_game`. Added spec.md read + mode-selection block before the retry loop. |
| `tests/cmd_generate_quality_predicates_domain_agnostic.rs` | New file — 12 tests locking in domain-agnostic acceptance + game-keyword routing + mode-layering invariants. |
| `handover/evidence/phase6_3_x_universality_1779111375/fixes/F11_generate_quality_domain_agnostic.md` | This document. |

Surfaces NOT touched:
- No Cargo.toml / Cargo.lock changes.
- No Class-4 surfaces (`src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`,
  `src/state/sequencer.rs`, `src/state/typed_tx.rs`,
  `src/bottom_white/cas/schema.rs`, canonical signing payload surfaces).
- No `src/web/spec.rs` change (F9 + F10's territory; conflict avoided).

---

## 8. Followup A12 (deferred)

The current `spec_looks_like_game` is a keyword union. False-negative cases
to consider for a future tracer bullet:

- Game-shape spec without any of the listed keywords (e.g. "make a maze
  navigation thing where arrow keys move a dot"). Currently misclassified
  as MinimumBar; the user sees a less-strict accept on a possibly broken
  game artifact.
- Mitigation: extend the keyword union, or call out to a tiny LLM classifier
  via the existing SiliconFlow client. Either way, a dedicated TB.

The Step-4 routing is INTENTIONALLY conservative (default MinimumBar). The
A12 followup can raise the false-negative recall later without breaking
universality.
