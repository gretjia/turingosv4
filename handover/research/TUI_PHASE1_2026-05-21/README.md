# TUI Phase-1 Research Archive — 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Triggered by | User-sim Rounds 1-5 found Claude sub-agents (with cheating shell competence) could complete the CLI flow, but user pointed out real non-programmers cannot — they need a TUI. |
| Phase | A (research × 3) + B (Karpathy lens critique × 1) + C (orchestrator synthesis) |
| Decision outcome | Karpathy zero-dep approach adopted — single `cmd_wizard.rs`, ~150-200 LoC, zero new Cargo deps. cliclack/indicatif/console proposal REJECTED. |
| Shipped from this research | (pending PR — Atom-W) |

## Why this archive exists

Future TuringOS design questions about TUI complexity (full-screen alt-buffer, multi-pane layouts, animated progress, etc.) should **read this archive first** before re-running web-research on Claude Code / Aider / OpenCode / etc. Preserve the convergent findings + the Karpathy critique that bounded the Phase-1 scope.

## Files in this archive

| File | Coverage |
|------|----------|
| `A1_modern_ai_tui_patterns.md` | Industry survey: Claude Code, OpenCode, Aider, Codex CLI, Cursor, gh copilot, Charm.sh, ratatui, inquire/dialoguer/cliclack. 5 converged patterns, 5 diverged, 5 anti-patterns. |
| `A2_v5_tui_design_extraction.md` | Design-only read of `~/projects/turingosv5/` TUI: zero TUI deps, ANSI escape codes, Welcome/Console screens, raw-mode fallback. Reusable patterns + v5-only logic to reject. |
| `A3_v4_cli_wrap_points.md` | v4 integration map: 4/5 subcommands callable in-process (welcome/init/llm/spec); only `generate` needs subprocess. CAS capsule structs available for structured display (no stdout parsing). Zero refactor cost. |
| `B_KARPATHY_LENS_CRITIQUE.md` | Karpathy attack on the cliclack proposal: 5 Simple Code / Architect violations, fake-future-extensibility ledger, predictions about what breaks if proposal ships, zero-dep counter-design. |
| `C_ORCHESTRATOR_SYNTHESIS.md` | Binding decision. Karpathy minimum design wins. 3 concessions to industry pattern preserved (TTY detection, in-process calls, end-of-flow open hint). Explicit DEFER triggers for cliclack/indicatif/full-screen. |

## Trigger conditions that would reopen this debate

1. **First real non-programmer field-tests the wizard and reports a specific gap that bare ANSI cannot fill** → consider adding `console` crate (single dep) for color portability.
2. **3+ providers ship in production** → consider `cliclack::select` for the now-unwieldy provider menu (currently 2 options handled by `1) deepseek 2) siliconflow`).
3. **`turingos generate` "Attempt N/M" text is confirmed not informative enough by field reports** → consider adding `indicatif` spinner.
4. **Windows / WSL / Chromebook SSH terminal compat report fails on bare `\x1b[32m`** → consider `crossterm` (lower-level than `console`).

Until any trigger fires, the zero-dep wizard is the canonical answer.

## How to consume this archive on a future session

1. Read this README + `C_ORCHESTRATOR_SYNTHESIS.md` for the binding decision.
2. If your current question is about TUI complexity: read `B_KARPATHY_LENS_CRITIQUE.md` §4 for the deferred-with-triggers ledger.
3. If asking about industry patterns: `A1_modern_ai_tui_patterns.md` table is the reference.
4. If asking why we DIDN'T port v5 TUI directly: `A2_v5_tui_design_extraction.md` lists what's reusable vs rejected.
5. Only re-run new research agents if the **industry has actually shifted** (new dominant AI-TUI pattern, new Rust TUI crate that's clearly better than cliclack/ratatui).
