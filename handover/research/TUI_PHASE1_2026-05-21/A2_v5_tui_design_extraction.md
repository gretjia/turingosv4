# A2 — v5 TUI design extraction (design only, no logic lift)

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 2 of 3 |
| Agent | Explore (read-only) |
| Sources | `/home/zephryj/projects/turingosv5/` working tree |
| Word count | ~2000 |
| Constraint | DESIGN-EXTRACTION ONLY. v5's command grammar, MetaAiConfig, DevTape-as-truth, hardcoded provider profiles, opaque CID minLength=8 are explicitly forbidden in v4 per `handover/architect-insights/V4_PRODUCT_CAK_HARDENING_EXECUTION_PLAN_2026-05-20.md` §3.2. |

## TL;DR

TuringOS v5's TUI is a **minimal, ANSI-escape-code-only console** without external UI dependencies. It features two static screens (Welcome and Console), a raw terminal mode fallback, and manual keyboard handling. The design prioritizes read-only DevTape projection and provider configuration over interactive workflows. For v4 Phase-1 TUI, v5 demonstrates **ANSI framing patterns, state-driven screen navigation, and fallback keyboard modes**, but v5's specific MetaAI config logic and DevTape business-only flows must not transfer.

## v5 TUI Architecture Map

| File | Lines | Purpose |
|------|-------|---------|
| `/src/bin/turingos.rs` | 1-358 | Main TUI entry point: RawTerminal control, Screen enum, keyboard loop, Welcome/Console screens, meta reconcile integration |
| `/src/devtool/mod.rs` | 380-481 | `meta_ai_welcome_frame_with_selection()`, `console_frame()`: ANSI frame builders with colored output (cyan, blue, green, yellow borders) |
| `Cargo.toml` | all | **Zero TUI dependencies**; serde_json + sha2 only |
| `/docs/v5_dev/TURINGOS_CONSOLE_MVP.md` | all | Console design contract: read-only DevTape view, MetaAI setup, adapter boundaries |

## D1-D10 Findings + Classifications

### D1. Entry Model

**Finding:** `turingos` (bare) auto-detects if stdin/stdout are terminals. If both are TTY, enters `run_tui()` (turingos.rs:74-78). Otherwise prints plain text. Non-interactive overrides: `--plain`, `--tui-frame`, `--welcome-frame`.

**Classification:** **REUSABLE DESIGN**. The pattern of "auto-detect TTY vs. fallback to non-interactive" is practical for v4 Phase-1. v4 can detect TTY and enter a native TUI for interactive use, fall back to streaming JSON for piped/script use.

### D2. Screen Layout

**Finding:** Two screens (turingos.rs:311-314):
- **Welcome**: Bordered frame with three numbered options (turingos.rs:68, devtool/mod.rs:384-481)
  - Uses ANSI colors (cyan border, blue selection pointer, muted detail text)
  - Shows "records N" in green, title + detail per action
  - Multi-line, centered, no scrolling
- **Console**: Bordered table-like view (devtool/mod.rs:298-378)
  - Header: store status, record count, tip hash
  - Task rows (≤12 visible): atom_id, status, title, PR# if present
  - Footer: command bar `[m] [w] [r] [h] [q]`
  - Optional help section below

Both use clear terminal (`\x1b[2J\x1b[H`) and 71-char-wide borders.

**Classification:** **REUSABLE DESIGN**. The bordered-frame pattern is simple, terminal-wide, and fits v4's simple spec/grill workflow. v4 can adopt similar ASCII framing without ratatui. The 12-row visible window is practical for task lists. Reject: do not port v5's exact "Console" view (DevTape-only); v4 needs different data (specs, grills, generated artifacts).

### D3. First-Run Experience

**Finding:** Default screen is Welcome (turingos.rs:83). On first run with empty config, v5 shows OpenAI OAuth, DeepSeek setup, and DevTape console options. If store does not exist, console_text() reports "DevTape not initialized" and suggests `turingos-dev event append` (devtool/mod.rs:261-268). No wizard.

**Classification:** **REUSABLE DESIGN (with caution)**. The Welcome-first pattern is good. v4 Phase-1 can show a Setup/Welcome screen on first run. **REJECT**: v4's first-run should NOT be API key setup (v4 uses different auth flow per constitution); instead, show a "Quick Start: what is TuringOS?" screen with links to docs.

### D4. API Key Setup

**Finding:** v5 separates API key handling into two MetaAI subcommands (turingos.rs:29-50):
- `turingos meta set-deepseek --api-key-env ENV_NAME [--from-env-file PATH]` writes `~/.turingos/provider-profiles.json` (not repo) and optionally `~/.turingos/secrets.env` with `0600` perms
- Config stores only env var name and provider profile cache, never the secret value itself
- Welcome screen shows status (env present/missing) and allows user to trigger setup from TUI (turingos.rs:145-156)

**Classification:** **REJECT (v5-only logic)**. v5's MetaAI config flow is explicitly banned in v4 per CLAUDE.md. Also, v5 hardcodes DeepSeek profiles in code (devtool/mod.rs:515-533: model names, URLs, legacy deprecation dates). v4 must NOT hardcode provider config or store provider metadata outside the repo. v4's API key setup is orthogonal to the TUI phase.

### D5. Spec/Grill Flow

**Finding:** v5 TUI has no spec interview or grill flow. Console is read-only projection of DevTape. There is no multi-step form, no question-at-a-time UX, no spec capture. (TURINGOS_CONSOLE_MVP.md: "read-only materialized view over DevTape".)

**Classification:** **REJECT (v5 has no spec/grill at all)**. This is out of scope for v5's console. v4 Phase-1 TUI MUST implement spec/grill. v5 is not a source here; look to v4's `--answers-file` JSON approach for non-interactive baseline, then add TUI wrapper.

### D6. Generate Flow

**Finding:** v5 console does not show LLM generation. It has no progress bar, spinner, or token stream. Meta reconcile is a one-shot dry-run that prints a report (turingos.rs:178-207; meta_reconcile_report is in devtool/mod.rs:1035-1119).

**Classification:** **REJECT (v5 has no generation UX)**. v5 is DevTape / board reconciliation only. v4 Phase-1 TUI MUST show "generating game..." feedback. This is a genuine design gap in v5 that v4 must address. Recommend: spinner + token count or progress % (if available from LLM).

### D7. Output / Delivery

**Finding:** v5 has no output surface. Console never shows "here's your game." The DevTape console projects task status (claimed, pr_open, merged, etc.) but does not show game artifacts, no browser launch, no in-TUI preview.

**Classification:** **REJECT (v5 has no output delivery)**. v4 Phase-1 MUST show "Your game is ready at `<path>`" or similar. This is another gap v5 doesn't fill. Recommend: after generation, show a brief summary + file path with a copy-to-clipboard button.

### D8. Error Handling

**Finding:** v5 captures errors in `status` variable (turingos.rs:85-89) and prints it below the frame (turingos.rs:99-101). On unknown command, sets `show_help = true` and `status = "unknown command: {char}"` (turingos.rs:164-167). Errors from functions return early with `.map_err(|err| err.to_string())` (e.g., turingos.rs:51-53). No modal dialogs, no color-coded severity.

**Classification:** **REUSABLE DESIGN**. The pattern of a single-line `status` bar below the frame is practical. v4 can adopt this for error messages (e.g., "API key missing" or "file write failed"). The status is cleared on successful action (turingos.rs:117, 121, etc.). Reject: do not copy v5's specific error messages (DevTape-only); v4 needs different error context.

### D9. State Persistence

**Finding:** v5 TUI reads from `.turingos_system/devtape/turingosv5/events.jsonl` (default store, turingos.rs:336). Session state (Welcome selected index, current screen, status message) is ephemeral, held only in local `run_tui()` variables (turingos.rs:82-84). On ctrl-C or quit, state is lost. No session save.

**Classification:** **REUSABLE DESIGN (with caveats)**. Ephemeral state is fine for v4 Phase-1 TUI; users expect `ctrl-C` to exit. For v4: if using a REPL-like loop (spec interview multi-screen), consider persisting the current question index or spec draft to a temp file so relaunch can resume. But for Phase-1, full-loss-on-ctrl-C is acceptable.

### D10. Keyboard Shortcuts

**Finding:** v5 has two keyboard modes (turingos.rs:238-273):
- **Raw mode** (if available): single-byte input; ESC-[A/B for up/down, CR for enter, Ctrl-C for quit, printable chars as-is
- **Line mode fallback**: read full line, match "up"/"down"/"k"/"j"/"quit"/"exit", single-char commands

Welcome screen commands (turingos.rs:110-169):
- `↑/↓` or line `up`/`down`/`k`/`j`: move selection
- `Enter`: confirm
- `o`: OpenAI OAuth
- `d`: DeepSeek setup
- `c`: Console
- `w`: Welcome (from console)
- `m`: Meta reconcile (from console)
- `h`: Toggle help
- `r`: Refresh
- `q`/`Ctrl-C`: Quit

**Classification:** **REUSABLE DESIGN**. The keyboard grammar is minimal, vi-like (k/j for movement), and falls back gracefully. v4 can adopt:
- Arrow keys + enter for navigation
- Single-char shortcuts (`q` for quit, `?` for help, etc.)
- A fallback mode for non-raw terminals (just a safeguard)

Reject: v5's exact command list is MetaAI-specific; v4's will differ based on TUI structure.

## v5's Cargo Dependencies (TUI-Related)

| Crate | Role | Reusable? |
|-------|------|-----------|
| `serde`, `serde_json` | JSON marshaling for DevTape records, config, board projection | **YES** — v4 can reuse for internal state serialization |
| `sha2` | Hash computation for DevTape record chain integrity | **NO** — v5-specific; v4 does not inherit DevTape |
| Standard library `std::io`, `std::process::Command` | TTY detection, raw terminal control via stty, stdin reading | **YES** — v4 can use for raw mode, keyboard capture, fork-exec |

**Key observation:** v5 has **zero external TUI crates**. No ratatui, cursive, or crossterm. All terminal control is done via ANSI escape sequences + stty syscalls. This is both a strength (minimal deps, portable) and a limitation (no automatic layout, no mouse support, no true alt-buffer).

## Direct Design Patterns v4 Phase-1 Should ADOPT

1. **TTY Auto-Detection + Fallback**: Detect if stdin/stdout are TTY; enter interactive TUI if yes, fall back to JSON streaming if no. This unifies interactive and headless use.
2. **ANSI-Escaped Bordered Frames**: Use `\x1b[2J\x1b[H` to clear, simple `+---+` borders, 256-color ANSI (e.g., `\x1b[38;5;81m` for cyan). No ratatui needed for Phase-1.
3. **Screen Enum + Loop Pattern**: Use a Rust enum for screens (Welcome, Spec, Grill, Review, Output, etc.). Single event loop dispatches on keyboard input and re-renders the current screen.
4. **Single-Line Status Bar**: Print a muted-colored status line below the frame for errors, confirmations, and meta info (e.g., "Saved to game.json").
5. **Raw Terminal Fallback**: Detect stty availability; if not available, gracefully fall back to reading full lines and accepting text commands ("up", "down", "c", etc.). Never crash on raw mode failure.
6. **Command Shorthand**: Single-letter shortcuts (q for quit, ? for help, enter to confirm) are faster than arrow-key-only navigation. Combine with arrow keys for accessibility.

## Direct Things v4 Must NOT Lift

1. **MetaAiConfig struct and hardcoded provider profiles** (devtool/mod.rs:38-56, 515-533): v4 explicitly rejects MetaAiConfig in the constitution. Do not hardcode DeepSeek model names, URLs, or legacy deprecation dates in v4 source. This is a v5 architectural choice, not portable.
2. **DevTape as single truth** (TURINGOS_CONSOLE_MVP.md): v5's entire console is a read-only projection over DevTape records. v4 does not use DevTape for runtime truth (per constitution). v4's TUI will project from a different source (e.g., board.json, spec artifacts, game state).
3. **Meta reconcile dry-run as a TUI command** (turingos.rs:158-160, devtool/mod.rs:1035-1119): This is v5-specific orchestration logic. v4 does not have this flow. Do not port `meta_reconcile_report()` or the `[m]` key binding.
4. **Environment variable name validation as a UI concern** (devtool/mod.rs:611-630): v5 validates env var names (must end in `_KEY`, `_API_KEY`, etc.). This is MetaAI-specific paranoia. v4's UI should not validate secret naming conventions.

## Open Questions (For A1 Industry Survey)

1. **Progress feedback for LLM generation**: v5 has none. Industry standard for code/game generators: what UX do users expect? Spinner only, token count, ETA, chunked artifact preview? → A1 answer: token-by-token streaming or simple spinner; non-programmer expects "Attempt N/M" text lines, which v4 already prints.
2. **Multi-window / panel layouts**: v5 uses single-screen modal navigation. Should v4 Phase-1 have a sidebar (for spec outline, task list, etc.) + main panel (current question, game preview), or stick to full-screen sequential screens? → A1 answer: inline streaming dominates over full-screen panels.
3. **Keyboard-only vs. mouse support**: v5 is keyboard-only. Should v4 support mouse clicks for buttons, scrolling? (Requires ratatui or similar; adds complexity.) → Phase 1: no.
4. **Persistence across reruns**: v5 loses session state on ctrl-C. For spec interview, should v4 persist the current step + answers so relaunch resumes, or full restart? Industry practice? → Phase 1: write answers to disk as they're collected (Karpathy-lens concession against transient state).
5. **Branching / conditional spec flows**: v5 has no spec, so no branching. v4 Phase-1 may need "does the game have NPCs?" → yes/no → different questions. How deep should the branching go? Linear questionnaire or tree? → Phase 1: linear, no branching. The existing 8 questions are linear.

---

**Report complete.** v5's TUI is a **minimal, template-like baseline** for Phase-1. It demonstrates TTY control, ANSI framing, and graceful fallbacks without external deps. However, v5's MetaAI config, DevTape business logic, and orchestration commands are v5-only and must not transfer. v4's TUI will need to add generation feedback, output delivery, and spec interview flows that v5 does not have.
