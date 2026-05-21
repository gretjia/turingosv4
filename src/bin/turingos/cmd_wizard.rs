//! TRACE_MATRIX FC2-N16: Phase-1 TUI wizard for non-programmer users (Atom-W).
//!
//! Entry point: bare `turingos` (no subcommand) when stdin+stdout are TTY.
//! Calls existing cmd_init / cmd_spec / cmd_generate in-process.
//! Zero new Cargo deps. Pure stdlib + ANSI escape codes + serde_json (already
//! in the workspace).
//!
//! Design rationale (preserved in handover/research/TUI_PHASE1_2026-05-21/):
//!   - 3 research agents converged on Karpathy-lens zero-dep approach
//!   - Rejected cliclack/indicatif/console crates (3 deps, Cz cycle 3 cost,
//!     Windows rendering risk) in favour of bare ANSI + std::io::IsTerminal
//!     (Rust 1.70+)
//!   - Reuses existing cmd_init / cmd_spec / cmd_generate via in-process calls
//!
//! Risk class: 1 (additive, zero new deps, no architecture change).

use std::io::{self, BufRead, IsTerminal, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode};

// ─── ANSI helpers ────────────────────────────────────────────────────────────
// Bare escape sequences — no `console` crate required.
const C_RESET: &str = "\x1b[0m";
const C_BOLD: &str = "\x1b[1m";
const C_DIM: &str = "\x1b[2m";
const C_CYAN: &str = "\x1b[36m";
const C_GREEN: &str = "\x1b[32m";
const C_YELLOW: &str = "\x1b[33m";
const C_RED: &str = "\x1b[31m";

/// TRACE_MATRIX FC2-N16: `wizard` short-help
pub(crate) const SHORT_HELP: &str =
    "Interactive TUI wizard — describe a game, get a playable HTML file (non-developer onboarding)";

/// TRACE_MATRIX FC2-N16: `wizard` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos wizard — Phase-1 TUI onboarding for non-programmer users

USAGE:
    turingos              (bare invocation — auto-wizard when stdin+stdout are TTY)
    turingos wizard       (explicit invocation)

DESCRIPTION:
    Guides a non-developer through the full TuringOS flow without requiring
    knowledge of env vars, JSON files, or workspace paths:
      1. Game idea (one sentence)
      2. Workspace location (suggested default)
      3. LLM provider selection (numbered menu)
      4. API key input (hidden with stty -echo on POSIX)
      5. `turingos init` in-process
      6. 8-question spec grill (same questions as `turingos spec`)
      7. `turingos spec --mode static --answers-file` in-process
      8. `turingos generate` in-process
      9. Platform-aware open command + optional browser launch

OPTIONS:
    -h, --help   Print this help.
"#;

/// TRACE_MATRIX FC2-N16: `wizard` entry — Phase-1 TUI onboarding for non-programmer users.
///
/// Called when user invokes bare `turingos` with TTY stdin+stdout,
/// or explicitly via `turingos wizard`.
pub(crate) fn run(args: &[String]) -> ExitCode {
    // Handle explicit --help / -h for wizard subcommand.
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        // Not a TTY — fall back to existing welcome behaviour.
        return crate::cmd_welcome::run(&[String::from("welcome")]);
    }
    match run_wizard() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("\n{C_RED}turingos wizard: {e}{C_RESET}");
            ExitCode::from(1)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Wizard flow
// ─────────────────────────────────────────────────────────────────────────────

fn run_wizard() -> Result<(), String> {
    print_banner();

    // Step 1: Game idea
    let game_idea = prompt("你想做什么游戏？请用一句话描述 (one sentence — your game idea):")?;
    if game_idea.trim().is_empty() {
        return Err("no game idea provided".to_string());
    }

    // Step 2: Workspace location
    let default_ws = suggest_workspace(&game_idea);
    let workspace = prompt_with_default(
        &format!(
            "保存到哪里？(workspace path, or Enter for {})",
            default_ws.display()
        ),
        &default_ws.to_string_lossy(),
    )?;
    let workspace_path = PathBuf::from(&workspace);
    std::fs::create_dir_all(&workspace_path)
        .map_err(|e| format!("create workspace: {e}"))?;

    // Step 3: Provider choice (numbered menu)
    let provider_idx = numbered_choice(
        "选择 LLM 提供商 (Which LLM provider?):",
        &[
            "SiliconFlow (api.siliconflow.cn) — 国内推荐",
            "DeepSeek (api.deepseek.com)      — 快速、便宜",
        ],
    )?;
    let provider = if provider_idx == 0 { "siliconflow" } else { "deepseek" };

    // Step 4: API key (stty -echo masking on POSIX)
    let key_label = if provider == "deepseek" {
        "DEEPSEEK_API_KEY"
    } else {
        "SILICONFLOW_API_KEY"
    };
    let meta_key = prompt_password(&format!(
        "粘贴你的 {key_label} (paste key — input hidden):"
    ))?;
    if meta_key.trim().is_empty() {
        return Err(format!("no {key_label} provided"));
    }

    let worker_key = if provider == "deepseek" {
        let resp = prompt_password(
            "粘贴第二个 DeepSeek key 作为 Worker 角色 (或直接 Enter 复用第一个 / press Enter to reuse):",
        )?;
        if resp.trim().is_empty() {
            meta_key.clone()
        } else {
            resp
        }
    } else {
        meta_key.clone()
    };

    // Step 5: Init workspace in-process
    println!("\n{C_DIM}Initializing workspace...{C_RESET}");
    let init_args = vec![
        String::from("init"),
        String::from("--project"),
        workspace.clone(),
        String::from("--provider"),
        String::from(provider),
        String::from("--force"),
    ];
    let init_rc = crate::cmd_init::run(&init_args);
    if init_rc != ExitCode::SUCCESS {
        return Err(format!("init failed (exit {init_rc:?})"));
    }

    // Step 6: Set env vars in-process so subsequent cmd_spec / cmd_generate pick
    // them up without requiring the user to touch their shell profile.
    // SAFETY: single-threaded at this point; no concurrent threads touching env.
    #[allow(clippy::disallowed_methods)]
    if provider == "deepseek" {
        // DeepSeek: use the DeepSeek endpoint via the SILICONFLOW_ENDPOINT override
        // pattern that cmd_generate / cmd_spec already honours.
        std::env::set_var("DEEPSEEK_API_KEY", &meta_key);
        std::env::set_var("DEEPSEEK_API_KEY_WORKER", &worker_key);
        // Point the SiliconFlow client at the DeepSeek endpoint so the existing
        // require_api_key() and chat_complete_blocking() machinery works unchanged.
        std::env::set_var(
            "TURINGOS_SILICONFLOW_ENDPOINT",
            "https://api.deepseek.com/v1/chat/completions",
        );
    } else {
        std::env::set_var("SILICONFLOW_API_KEY", &meta_key);
    }

    // Step 7: 8 spec questions (same as cmd_spec FULL_HELP interview flow)
    println!("\n{C_BOLD}现在我来问你 8 个关于游戏的问题。{C_RESET}");
    println!("{C_DIM}每题 1-2 句就好；回答后按 Enter。{C_RESET}\n");

    // Question texts pulled verbatim from cmd_spec.rs FULL_HELP INTERVIEW FLOW.
    let questions: &[(&str, &str)] = &[
        (
            "Q1 The Job (JTBD)",
            "你最近什么时候想过「要是有个工具帮我做这件事就好了」？(When did you last wish you had a tool for something?)",
        ),
        (
            "Q2 The Anchor",
            "有没有哪个网站或 App 和你想做的有一点像？(Any website / app even a little like what you want?)",
        ),
        (
            "Q3 What it Remembers",
            "这个程序明天早上还应该记得什么？(What should the program still know tomorrow morning? e.g. high score)",
        ),
        (
            "Q4 First-Click Walk-Through",
            "用户打开页面后，第一步做什么？请一步步描述。(Step-by-step: what does the user see / click first?)",
        ),
        (
            "Q5 Weird-User Test",
            "如果用户做了什么奇怪的事，它还应该正常工作吗？(What should NOT break it?)",
        ),
        (
            "Q6 Disappointment Boundary",
            "哪些功能会让你觉得「这超出范围了」？(Which features would feel like scope creep?)",
        ),
        (
            "Q7 Success Test",
            "一个月后，你怎么判断它是否有用？多少人用了？(How will you KNOW it's doing its job after a month?)",
        ),
        (
            "Q8 Playback (mirror)",
            "用一句话告诉我：你希望我帮你做什么？(Describe back what you want me to build in 1 sentence.)",
        ),
    ];

    let mut answers: Vec<String> = Vec::with_capacity(8);
    for (i, (title, hint)) in questions.iter().enumerate() {
        println!("{C_CYAN}问题 {}/{} — {title}{C_RESET}", i + 1, questions.len());
        let ans = prompt(hint)?;
        answers.push(ans);
        println!();
    }

    // Write answers.json for cmd_spec --answers-file
    let answers_path = workspace_path.join("wizard_answers.json");
    let answers_json = serde_json::to_string_pretty(&answers)
        .map_err(|e| format!("serialize answers: {e}"))?;
    std::fs::write(&answers_path, &answers_json)
        .map_err(|e| format!("write wizard_answers.json: {e}"))?;

    // Step 8: Spec (in-process)
    println!(
        "{C_DIM}正在生成 spec.md（调用 Meta LLM，约 10-30 秒）...{C_RESET}"
    );
    let spec_args = vec![
        String::from("spec"),
        String::from("--workspace"),
        workspace.clone(),
        String::from("--answers-file"),
        answers_path.to_string_lossy().into_owned(),
        String::from("--lang"),
        String::from("zh"),
        String::from("--mode"),
        String::from("static"),
    ];
    let spec_rc = crate::cmd_spec::run(&spec_args);
    if spec_rc != ExitCode::SUCCESS {
        return Err(format!("spec failed (exit {spec_rc:?})"));
    }

    // Step 9: Generate (in-process)
    println!(
        "\n{C_DIM}正在生成游戏代码（约 30-60 秒）...{C_RESET}"
    );
    let gen_args = vec![
        String::from("generate"),
        String::from("--workspace"),
        workspace.clone(),
    ];
    let gen_rc = crate::cmd_generate::run(&gen_args);
    if gen_rc != ExitCode::SUCCESS {
        eprintln!(
            "\n{C_YELLOW}generate 这次没能交付。{C_RESET}"
        );
        eprintln!("{C_DIM}你可以稍后重试：{C_RESET}");
        eprintln!("  cd {} && turingos generate", workspace);
        return Err(String::from("generate exited non-zero"));
    }

    // Step 10: Show artifact path + platform-aware open command
    let artifact = workspace_path.join("artifacts").join("index.html");
    println!("\n{C_GREEN}{C_BOLD}✓ 你的游戏已生成！{C_RESET}");
    println!("\n{C_BOLD}游戏文件:{C_RESET} {}", artifact.display());
    println!("\n{C_BOLD}在浏览器中打开:{C_RESET}");
    if cfg!(target_os = "macos") {
        println!("  open {}", artifact.display());
    } else if cfg!(target_os = "windows") {
        println!("  start {}", artifact.display());
    } else {
        println!("  xdg-open {}", artifact.display());
        println!("  {C_DIM}或者在文件管理器中双击该文件{C_RESET}");
    }

    // Step 11: Offer to open now
    let do_open = prompt_yes_no("现在打开吗？(Open it now?)", true)?;
    if do_open {
        let opener = if cfg!(target_os = "macos") {
            "open"
        } else if cfg!(target_os = "windows") {
            "start"
        } else {
            "xdg-open"
        };
        // Best-effort; ignore failure (e.g. no display server in CI).
        let _ = Command::new(opener).arg(&artifact).status();
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// UI helpers — pure stdlib + ANSI, zero new deps
// ─────────────────────────────────────────────────────────────────────────────

fn print_banner() {
    println!("{C_CYAN}{C_BOLD}");
    println!("  ████████  TuringOS  — 描述一个游戏，获得可玩的 HTML 文件");
    println!("  ████████  TuringOS  — Describe a game, get a playable HTML file");
    println!("{C_RESET}");
}

fn suggest_workspace(idea: &str) -> PathBuf {
    let slug: String = idea
        .chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c.is_whitespace() || c == '-' || c == '_' {
                Some('-')
            } else {
                // Skip non-ASCII and punctuation for a filesystem-safe slug.
                None
            }
        })
        .take(40)
        .collect();
    let slug = slug.trim_matches('-').to_string();
    let slug = if slug.is_empty() {
        String::from("my-game")
    } else {
        slug
    };
    PathBuf::from(format!("/tmp/turingos-{slug}"))
}

/// Single-line prompt. Prints `> label` then reads one line from stdin.
fn prompt(label: &str) -> Result<String, String> {
    print!("{C_BOLD}> {label}{C_RESET}\n  ");
    io::stdout().flush().ok();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .lock()
        .read_line(&mut line)
        .map_err(|e| format!("stdin read: {e}"))?;
    Ok(line.trim().to_string())
}

fn prompt_with_default(label: &str, default: &str) -> Result<String, String> {
    let raw = prompt(label)?;
    Ok(if raw.is_empty() {
        default.to_string()
    } else {
        raw
    })
}

fn prompt_yes_no(label: &str, default_yes: bool) -> Result<bool, String> {
    let suffix = if default_yes { " [Y/n]" } else { " [y/N]" };
    let raw = prompt(&format!("{label}{suffix}"))?.to_lowercase();
    if raw.is_empty() {
        return Ok(default_yes);
    }
    Ok(matches!(raw.as_str(), "y" | "yes" | "是" | "好"))
}

fn numbered_choice(label: &str, options: &[&str]) -> Result<usize, String> {
    println!("{C_BOLD}> {label}{C_RESET}");
    for (i, opt) in options.iter().enumerate() {
        println!("  {}) {opt}", i + 1);
    }
    loop {
        print!("  ");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin()
            .lock()
            .read_line(&mut line)
            .map_err(|e| format!("stdin read: {e}"))?;
        let trimmed = line.trim();
        if let Ok(n) = trimmed.parse::<usize>() {
            if n >= 1 && n <= options.len() {
                return Ok(n - 1);
            }
        }
        println!(
            "{C_YELLOW}  请输入 1 到 {} 之间的数字。{C_RESET}",
            options.len()
        );
    }
}

/// Password prompt: uses `stty -echo` on POSIX to hide input.
/// On non-POSIX (Windows), input is shown — known limitation documented in wizard help.
fn prompt_password(label: &str) -> Result<String, String> {
    println!("{C_BOLD}> {label}{C_RESET}");
    print!("  ");
    io::stdout().flush().ok();

    // Echo-off guard: restores terminal echo on drop even if we return early.
    #[cfg(unix)]
    let _guard = EchoGuard::new();

    let mut line = String::new();
    io::stdin()
        .lock()
        .read_line(&mut line)
        .map_err(|e| format!("stdin read: {e}"))?;

    // Print newline since the user's Return key was consumed silently.
    #[cfg(unix)]
    println!();

    Ok(line.trim().to_string())
}

// ─────────────────────────────────────────────────────────────────────────────
// POSIX echo guard
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(unix)]
struct EchoGuard;

#[cfg(unix)]
impl EchoGuard {
    fn new() -> Self {
        // Suppress stty errors (e.g. in CI with a pty but no real terminal).
        let _ = Command::new("stty").arg("-echo").status();
        EchoGuard
    }
}

#[cfg(unix)]
impl Drop for EchoGuard {
    fn drop(&mut self) {
        let _ = Command::new("stty").arg("echo").status();
    }
}
