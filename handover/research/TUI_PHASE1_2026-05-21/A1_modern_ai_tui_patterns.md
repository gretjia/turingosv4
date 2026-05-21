# A1 — Modern AI-era TUI patterns survey

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | A (research) — dispatch 1 of 3 |
| Agent | sonnet, general-purpose, WebFetch + WebSearch |
| Sources | Claude Code, OpenCode, Aider, Codex CLI, Cursor, gh copilot, Charm.sh ecosystem, ratatui, inquire/dialoguer/cliclack |
| Word count | ~2200 |

## TL;DR

The two patterns that would land best in TuringOS v4 Phase 1 are: (1) **inline streaming REPL with slash-command discovery** — the model used by Claude Code, Aider, and Codex CLI, where `turingos` alone drops the user into a guided conversation without any flags or JSON editing; and (2) **inquire/cliclack-style sequential prompts** for the one-time init wizard, which hides env vars and file paths behind "choose from a list" and "type your API key here" affordances. A third pattern worth borrowing is **`@`-prefix fuzzy file search** (used by both Codex CLI and OpenCode) as a replacement for exposing raw file-path arguments. Together these three patterns let a non-programmer complete the 5-step flow — welcome → init → spec → generate → see game — with zero shell knowledge required.

## Per-System Findings Table

| System | Q1 Entry model | Q2 Visual paradigm | Q3 Input modes | Q4 Progress/streaming | Q5 State persistence | Q6 Non-programmer notes / anti-patterns |
|---|---|---|---|---|---|---|
| **Claude Code** | `claude` alone → interactive REPL; `claude "task"` for one-shot | Inline streaming below prompt; full-screen `/config` TUI overlay | `/`-prefixed commands (filter-as-you-type), `@` file fuzzy, natural language | Token-by-token stream; `Ctrl+B` background task; status line updates every 60 s | Per-directory history; `-c` to resume last session | **Good**: `/config` hides JSON; tab-complete hides paths. **Anti-pattern**: first-run needs `ANTHROPIC_API_KEY` env var set manually |
| **OpenCode** | `curl \| bash` install; `opencode` starts TUI session | Full-screen multi-pane conversational TUI (Rust/Go native) | Natural language; `@` fuzzy file search; `!` for shell passthrough; `ctrl+x` leader keybinds; `ctrl+p` command palette | Token streaming embedded in pane | Sessions persist, listable/resumable via `ctrl+x l`; undo via git | **Good**: session export to Markdown; themes. **Anti-pattern**: `tui.json` / `tui.jsonc` config exposed; undo requires git repo |
| **Aider** | `pip install` → `aider` → REPL | Inline streaming; markdown rendered in terminal (no alt-buffer) | 44 slash-commands; `/voice`; `/editor`; up-arrow history | Token-by-token with colored diff sections | Git commits after every change; no built-in session resume | **Good**: voice input; git as persistence. **Anti-pattern**: first-run needs model flag + API key; 44 commands is overwhelming; exposes `--model deepseek` flag syntax to non-programmers |
| **Codex CLI** | `codex` → full-screen TUI; `codex "task"` one-shot; `codex resume` | Full-screen TUI with syntax-highlighted diffs; light/dark themes | `/review`, `/model`, `/theme`, `/copy`; `@` fuzzy workspace search; `!` shell; `Ctrl+R` history | Streaming + queued follow-ups via Tab; sub-agent trees | `codex resume` reopens previous transcript locally | **Good**: approval modes (read-only/auto/full). **Anti-pattern**: API key or ChatGPT login still required upfront; image attach uses `-i flag` |
| **Cursor** | GUI IDE with integrated terminal agent; no terminal-only entry | GUI panels: side-by-side panes for multi-agent, tiled layout; editor-native, NOT terminal-native | Natural language in chat panel; `/model`, approval modals; Shift+Tab for plan mode | Step-by-step approval of each file edit + command; streaming in panel | GUI workspace/session state; no terminal session resume | **Good**: plan mode before acting; tiled agent overview. **Anti-pattern**: requires a full Electron IDE; terminal tab inside IDE is still programmer-facing |
| **gh copilot** | `gh copilot` → install on first run → interactive REPL | Inline below prompt; no alt-buffer | Natural language; `/model`; Shift+Tab for plan mode vs autopilot mode | Streaming; specialist sub-agents (Explore, Task) run in background | Auto-compaction for ~infinite sessions | **Good**: `gh copilot` is a single memorable entry; plan-before-build reduces surprise. **Anti-pattern**: still requires `gh auth login`; exposing `/model` choice to non-programmer is unnecessary friction |
| **Charm.sh (gum/glow/huh)** | Scripts call `gum choose` / `gum input` etc. inline; no standalone app | Each gum call replaces one shell line: spinner, confirm dialog, fuzzy list, form | Keyboard arrow-navigation; fuzzy-filter for lists; form fields | `gum spin --title "..."` spinner; no built-in LLM streaming | No session persistence; each invocation is stateless | **Good**: gum scripts can wrap any CLI into a wizard with zero Go code. **Anti-pattern**: still shell-scripted so a non-programmer would need the wrapper written for them |
| **ratatui** | Library, not end-user product; app authors call `ratatui::run()` | Full-screen alt-buffer: panels, charts, gauges, sparklines — immediate-mode rendering | Keyboard-driven; app defines all keymaps | Widgets: gauge, sparkline, progress bar — all purely visual | App-defined; framework is stateless | **Good**: rich visual affordances (gauges, trees) with sub-millisecond rendering. **Anti-pattern**: 3–8k LoC for a real app; no out-of-box wizard, forms, or help system |
| **inquire / dialoguer / cliclack** | Library; app calls `Select::new()`, `Text::new()`, etc. | Inline prompts rendered below current cursor position; no alt-buffer | Arrow keys for select; type-to-filter; text input; confirm Y/n | None built-in (inquire has no spinner); pair with indicatif | Stateless — form data returned as Rust values | **Good**: cliclack/inquire give polished "onboarding wizard" feel in ~50-200 LoC. **Anti-pattern**: no streaming, no persistent session, must roll your own |

## Patterns That Converge Across Modern AI-era TUIs

**1. Single-word entry, zero flags.** Every system that achieved broad adoption (`claude`, `aider`, `codex`, `gh copilot`) starts with a single bare command. No subcommand, no `--model`, no `--api-key`. The first interaction is a prompt, not a configuration screen.

**2. Inline streaming below the prompt.** The dominant visual pattern is NOT a full-screen alt-buffer. Claude Code, Aider, gh copilot, and Codex CLI all render LLM output streaming inline in the current terminal scroll buffer. This means the user can scroll up, copy text with the normal terminal, and ctrl-C safely. Only OpenCode and Cursor go full-screen; both are more programmer-facing.

**3. Slash-command discovery with filter-as-you-type.** Every system (Claude Code, Codex CLI, OpenCode, gh copilot, Aider) uses `/` as the command prefix. The list appears on `/` and narrows as you type. This is the de-facto standard for "discoverability without a manual."

**4. `@` for file/context injection.** Claude Code, Codex CLI, and OpenCode all use `@` to attach files or search the workspace fuzzily. This replaces raw file-path arguments and is understood by non-programmers as "referencing something."

**5. Session resume, not restart.** `claude -c`, `codex resume`, OpenCode's `ctrl+x l` — every system treats the session as a persistent artifact. Non-programmers should never lose work on accidental Ctrl-C.

## Patterns That Diverge

**1. Full-screen alt-buffer vs. inline scroll.** OpenCode and ratatui apps take over the whole terminal (alt-buffer). Claude Code, Aider, Codex CLI stay inline. Inline is friendlier to non-programmers who don't know "press q to exit vim."

**2. Plan mode vs. act immediately.** gh copilot and Cursor both add an explicit "plan before build" mode (Shift+Tab). Claude Code and Aider act immediately on the first message. For non-programmers, a plan-first step reduces surprise about irreversible changes.

**3. Approval granularity.** Codex CLI has explicit modes: read-only / auto / full-access. gh copilot has a command allowlist. Claude Code and Aider trust the user more by default. Non-programmers likely want coarser controls ("approve this step" rather than a per-command allowlist).

**4. Config surface.** Claude Code wraps settings behind `/config` (TUI overlay). OpenCode exposes a raw `tui.json`. Aider surfaces dozens of `--flag` options. The hidden-config model (Claude Code) is strictly better for non-programmers.

**5. First-run authentication.** All systems require an API key or auth token. None solves this elegantly for non-programmers. gh copilot comes closest: `gh auth login` is at least a guided browser flow rather than "set OPENAI_API_KEY in your shell profile."

## Rust TUI Implementation Trade-offs

| Option | Approx LoC for Phase 1 | Phase-1 fit | Learning curve | Notes |
|---|---|---|---|---|
| **ratatui** | 3 000–8 000 | Low | High (immediate-mode rendering, event loop, full widget system) | Excellent for dashboards/monitors; overkill for a 5-step onboarding wizard; no built-in form/prompt primitives |
| **inquire** | 200–500 | High | Low (prompt-per-question API) | Covers wizard step 1–3 well; no LLM streaming display; pair with `indicatif` for spinners |
| **cliclack** | 150–400 | High | Low (inspired by npm @clack/prompts; opinionated modern style) | Best out-of-box aesthetic for a wizard; `intro/outro`, themed steps, spinners included |
| **clap REPL loop** | 400–800 | Medium | Medium (reuses existing clap; add a `turingos repl` subcommand with a read-eval-print loop) | Natural fit if you already have `cmd_*` functions; no fancy UI but works today |
| **Inline streaming pattern** | 100–300 (delta on existing CLI) | High | Low | Add colored streaming output to existing command handlers; use `crossterm` or `console` for color; pairs with cliclack for init wizard |

**Smallest viable Phase-1:** `cliclack` (or `inquire`) for the one-time init wizard (steps 1–3), plus **inline streaming** using `indicatif` + `console` for the generate step (step 4). No full-screen alt-buffer needed. Estimated total new LoC: ~500–800 on top of existing `cmd_*` plumbing.

> Karpathy-lens counter (see `B_KARPATHY_LENS_CRITIQUE.md`): adopting cliclack costs 3 new Cargo deps + Cz cycle 3 Trust Root rehash + Windows/WSL/Chromebook rendering risk. v5 demonstrated zero-dep ANSI works. Adopted counter-design instead.

## Recommended Phase-1 Stack (industry-research consensus, before Karpathy critique)

- **Crate pick: `cliclack` for the init wizard.** Its `intro/outro/input/select/spinner/confirm` primitives map directly to the 5-step flow. No alt-buffer, no learning `ratatui`'s immediate-mode loop. Add `inquire` only if you need fuzzy-autocomplete for the spec step.

- **Visual paradigm: inline streaming, not full-screen.** The `console` crate (or `crossterm` directly) handles colored streaming output. Each `generate` step prints tokens as they arrive, with an `indicatif` spinner before the first token. This mirrors Claude Code's paradigm and works in every terminal including SSH.

- **Entry model: `turingos` alone.** On first run, detect missing config and launch the `cliclack` wizard automatically — no flags, no "please set three env vars first." API key prompt is a `cliclack::password()` call that writes to `~/.config/turingos/config.toml` once.

- **Command discovery: `turingos /help` or `?` during wizard steps.** Borrow the slash-command filter pattern: typing `/` in the main REPL shows available commands with a live-filtering list (can be implemented with `inquire::Select` or a small `crossterm` event loop).

- **Wrap existing `cmd_*` logic with thin display adapters.** Each existing command handler returns structured data; Phase-1 adds a `display::stream()` layer that renders it progressively. No architectural changes to tape/kernel/CAS surfaces — this is a pure Class 1 additive change.

## Anti-patterns to Avoid

**1. "Set this env var first."** Every system that exposes `export OPENAI_API_KEY=...` to non-programmers loses them at step 0. Replace with a first-run `cliclack::password()` prompt that persists to a config file.

**2. Long flag forests on the entry command.** `aider --model deepseek --api-key sk-... --no-auto-commit` tells a non-programmer they are in the wrong place. All configuration must be hidden behind wizard questions or `/config`.

**3. Raw JSON/TOML config exposed as the primary interface.** OpenCode's `tui.json` and TuringOS's current "edit genesis_payload.toml" flow are the same anti-pattern. Config files are implementation details; the TUI must be the only required interface.

**4. Full-screen alt-buffer without an escape hatch.** ratatui apps that don't print "press Q to quit" prominently, or that use non-standard keymaps, strand non-programmers. If you use alt-buffer at all, the first visible line must say how to exit.

**5. Exposing exit codes and error backtraces to end users.** `cargo test` output, Lean stderr, and Rust panics with stack traces are programmer surfaces. All error paths in the Phase-1 TUI must catch these and render a single human sentence like "Something went wrong. Run `turingos /doctor` for details."

## Key sources

- OpenCode TUI docs: https://opencode.ai/docs/tui/
- Claude Code Interactive Mode Reference: https://claudefa.st/blog/guide/mechanics/interactive-mode
- Codex CLI features: https://developers.openai.com/codex/cli/features
- Aider commands: https://aider.chat/docs/usage/commands.html
- GitHub Copilot CLI enhanced agents changelog: https://github.blog/changelog/2026-01-14-github-copilot-cli-enhanced-agents-context-management-and-new-ways-to-install/
- Cursor 2.0 changelog: https://cursor.com/changelog/2-0
- Charm.sh: https://charm.land/
- Ratatui showcase: https://ratatui.rs/showcase/apps/
- Rust CLI prompt comparison: https://fadeevab.com/comparison-of-rust-cli-prompts/
