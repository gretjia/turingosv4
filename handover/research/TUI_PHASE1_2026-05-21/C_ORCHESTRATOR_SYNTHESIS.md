# C — Orchestrator synthesis & binding decision (TUI Phase 1)

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Phase | C (synthesis) |
| Orchestrator | Claude opus 4.7 |
| Authority | User-delegated 2026-05-21 |
| User decision | "Karpathy zero-dep 方案 (推荐)" |
| Status | BINDING — implemented as Atom-W in PR (pending) |

## Decision

**Karpathy zero-dep counter-design adopted.** Reject cliclack/indicatif/console crates entirely. Ship single new file `src/bin/turingos/cmd_wizard.rs` (~150-200 LoC) + minimal dispatch wire-up in `src/bin/turingos.rs`. Use stdlib `IsTerminal` + bare ANSI escape codes + `stty -echo` for password masking.

## Where Karpathy won

| Issue | Verdict | Reason |
|-------|---------|--------|
| 3 new Cargo crates for Phase-1 | **Karpathy** | v5 zero-dep TUI is existence proof; 3 crates = Cz cycle 3 Trust Root rehash cost |
| Background CAS polling thread feeding `indicatif` | **Karpathy** | KARPATHY_SIMPLE_CODE §5 explicitly forbids hidden background workers; race with `cmd_generate` stdout |
| `cliclack::select` scrollable menu for 2 providers | **Karpathy** | Fake future extensibility; `1) deepseek 2) siliconflow` numbered choice is clearer |
| Terminal compatibility (Windows/WSL/Chromebook SSH) | **Karpathy** | cliclack cursor-movement breaks on non-VTE terminals — exact terminals non-programmers use |
| ~500-800 LoC vs ~150-200 LoC for same outcome | **Karpathy** | Smallest correct thing today (KARPATHY_SIMPLE_CODE §Worker Checklist) |

## Where the cliclack proposal kept ground (3 concessions)

These bullets from the original proposal were correct and remain in the adopted design:

1. **TTY detection via `std::io::IsTerminal`** — correct gate; auto-route bare `turingos` to wizard when stdin+stdout are TTY, else fall back to existing welcome.
2. **In-process calls to existing `cmd_*.rs::run`** — KARPATHY_ARCHITECT §3 (Micro-Implementation) explicitly prescribes this. No subprocess, no shell interpolation. Just build `Vec<String>` args and call.
3. **Platform-aware open-game completion signal** — print absolute path + `xdg-open` / `open` / `start` command per OS detection. Three lines of `std::process::Command`, no crate needed.

## What ships

### Atom-W (Class 1)

`src/bin/turingos/cmd_wizard.rs` (NEW, ~150-200 LoC):

- ANSI color constants (`\x1b[36m` cyan, `\x1b[32m` green, etc.)
- `prompt(label)` — bare `print!` + `stdin().read_line()`
- `prompt_password(label)` — same with `stty -echo` POSIX bracketing (cfg-gated; Windows falls back to visible echo with documented known limitation)
- `numbered_choice(label, options)` — print numbered list, validate digit input
- `prompt_yes_no(label, default)` — Y/n single-char with default
- `run(args)` — orchestrates the 11-step wizard

Wizard flow:
1. Banner + welcome message
2. Ask game idea (one sentence)
3. Ask workspace path (default `/tmp/turingos-{slug}`)
4. Ask provider (numbered: 1 DeepSeek / 2 SiliconFlow)
5. Ask Meta API key (password-masked)
6. If DeepSeek: ask Worker API key (or reuse Meta)
7. Call `cmd_init::run` with synthesized args (`--project`, `--provider`, `--force`)
8. Set env vars in-process: `DEEPSEEK_API_KEY`, `DEEPSEEK_API_KEY_WORKER`, `TURINGOS_SILICONFLOW_ENDPOINT`
9. Ask 8 spec questions (the canonical Q1-Q8 from `cmd_spec.rs` FULL_HELP); write to `answers.json` in workspace
10. Call `cmd_spec::run` with `--answers-file` pointing at the JSON
11. Call `cmd_generate::run`
12. On success: print platform-aware open command + ask "Open now?" → optionally exec opener

`src/bin/turingos.rs` (MINIMAL EDIT, ~5 LoC):
- Add `mod cmd_wizard;`
- Dispatch: if `args.is_empty()` AND stdin is TTY → `cmd_wizard::run(&[])`
- Dispatch: if `args[0] == "wizard"` → `cmd_wizard::run(&args)` (explicit alias)
- Otherwise dispatch as today

### Tests

`tests/wizard_smoke.rs` (NEW):
- Compile-only: assert `cmd_wizard::run` exists and returns `ExitCode`
- Optional behavioral: pipe answers into stdin, expect exit non-zero with mock keys (LLM call fails)

### What does NOT ship

| Item | Defer trigger |
|------|---------------|
| `cliclack` crate | First non-programmer reports rendering issue NOT solvable by bare ANSI |
| `indicatif` spinner | "Attempt N/5" text confirmed insufficient by field report |
| `console` crate | `\x1b[32m` breaks on a real terminal |
| `crossterm` (raw mode beyond `stty`) | Need true cursor control / mouse / multi-pane |
| CAS-polling background thread | A second consumer of `ArtifactBundleManifest` (not just `cmd_generate` stdout) materializes |
| Full-screen alt-buffer | Never — KARPATHY says no, industry consensus is inline streaming |
| Session resume after Ctrl-C | First real field complaint that lost-work-on-Ctrl-C is a blocker |

## Costs avoided

| If we had taken the cliclack proposal | Avoided |
|---------------------------------------|---------|
| New Cargo deps | 3 (cliclack + indicatif + console) |
| Cz cycle 3 Trust Root rehash | Yes (Cargo.lock + Cargo.toml pin update) |
| LoC | ~500-800 source + ~200 test = ~1000 |
| Background worker / threading complexity | CAS-polling race condition with cmd_generate |
| Terminal compatibility risk | cliclack rendering breakage on Windows/WSL/Chromebook |
| Maintenance cost of UI-only crate updates triggering Trust Root events | High |

| What we pay instead (Atom-W) | Cost |
|------------------------------|------|
| New Cargo deps | **0** |
| Cz Trust Root churn | **None** |
| LoC | ~150-200 source + ~50 test = ~200 |
| Threading | None (synchronous wizard) |
| Visual polish vs cliclack | Lower — bare ANSI, numbered choices instead of arrow-select menus. But adequate for Phase-1 "usable" goal. |

## How this archive ages

The decision is correct **for Phase-1 ("just usable" per user)**. The deferred items have explicit trigger conditions. Future debates should:
1. Reference the trigger conditions in §"What does NOT ship".
2. Only re-litigate the architectural choice if a trigger fires.
3. Avoid re-running web research unless the **industry has shifted** (e.g., a new dominant AI-TUI pattern emerges that supersedes inline streaming).
4. Avoid re-running v5 extraction (v5's TUI is frozen design).
5. v4 integration map (A3) may need refresh if cmd_*.rs entry signatures change.

## Closing note

The cliclack proposal followed industry convergence (A1 finding: "cliclack is the modern wizard standard"). The Karpathy critique exposed that **industry convergence is a guide, not a mandate**. v5's existence proof of zero-dep ANSI working in production, combined with the constitutional Trust Root cost, tipped the balance to the minimum design.

Phase-2 (if it ever materializes) might add `console` for color portability after a real Windows compat report. Phase-1 ships without it.
