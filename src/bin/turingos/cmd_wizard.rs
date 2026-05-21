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

    // ── Software 3.0 ordering ────────────────────────────────────────────────
    // Meta AI is set up FIRST so all subsequent ambiguous user input can be
    // routed through it for intent interpretation. The user's previous CLI run
    // (2026-05-21) typed "你来制定" for the workspace prompt, expecting the
    // wizard to understand "you decide" — but Meta AI wasn't online yet, so the
    // string was taken literally. New design: provider + keys come first;
    // workspace is auto-generated (no prompt); a single free-form intent line
    // is expanded into the 8-question spec by Meta AI itself.

    // Step 1/3 — Provider choice (numbered menu — deterministic, not ambiguous)
    let provider_idx = numbered_choice(
        "Step 1/3 — 选择 LLM 提供商 (Which LLM provider?):",
        &[
            "SiliconFlow (api.siliconflow.cn) — 国内推荐",
            "DeepSeek (api.deepseek.com)      — 快速、便宜",
        ],
    )?;
    let provider = if provider_idx == 0 { "siliconflow" } else { "deepseek" };

    // Step 2/3 — Meta API key (stty -echo masking on POSIX)
    let key_label = if provider == "deepseek" {
        "DEEPSEEK_API_KEY"
    } else {
        "SILICONFLOW_API_KEY"
    };
    let meta_key = prompt_password(&format!(
        "Step 2/3 — 粘贴你的 {key_label} (paste API key, input hidden):"
    ))?;
    if meta_key.trim().is_empty() {
        return Err(format!("{key_label} cannot be empty"));
    }

    // Step 3/3 — Worker key (DeepSeek dual-key mode only)
    let worker_key = if provider == "deepseek" {
        let resp = prompt_password(
            "Step 3/3 — 粘贴第二个 DeepSeek key 作 Worker 角色 (或直接 Enter 复用第一个):",
        )?;
        if resp.trim().is_empty() {
            meta_key.clone()
        } else {
            resp
        }
    } else {
        meta_key.clone()
    };

    // Set env vars in-process — Meta AI is now usable for the rest of the flow.
    // SAFETY: single-threaded at this point; no concurrent threads touching env.
    #[allow(clippy::disallowed_methods)]
    if provider == "deepseek" {
        std::env::set_var("DEEPSEEK_API_KEY", &meta_key);
        std::env::set_var("DEEPSEEK_API_KEY_WORKER", &worker_key);
        std::env::set_var(
            "TURINGOS_SILICONFLOW_ENDPOINT",
            "https://api.deepseek.com/v1/chat/completions",
        );
    } else {
        std::env::set_var("SILICONFLOW_API_KEY", &meta_key);
    }

    // Auto-create workspace — no user prompt. Timestamp-based path avoids the
    // "你来制定" free-text trap entirely. Wizard chooses; user does not need to.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let workspace = format!("/tmp/turingos-{timestamp}");
    let workspace_path = PathBuf::from(&workspace);
    std::fs::create_dir_all(&workspace_path)
        .map_err(|e| format!("create workspace: {e}"))?;
    println!(
        "\n{C_DIM}工作区 / Workspace: {workspace}{C_RESET}"
    );

    // Initialize workspace (in-process; cmd_init::run expects only flags).
    println!("{C_DIM}Initializing...{C_RESET}");
    let init_args = vec![
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

    // ── The single intent prompt — Meta AI does the rest ────────────────────
    println!("\n{C_BOLD}用一两句话告诉我你想做什么。{C_RESET}");
    println!(
        "{C_DIM}Tell me what you want to build, in 1-2 sentences.{C_RESET}"
    );
    println!(
        "{C_DIM}任何想法都行 — 游戏、工具页、调研页、教学 demo……{C_RESET}"
    );
    println!(
        "{C_DIM}Meta AI 会理解你的意图并扩展成完整 spec。{C_RESET}"
    );
    let intent = prompt(">")?;
    if intent.trim().is_empty() {
        return Err("no intent provided".to_string());
    }

    // Meta AI expansion: 1 sentence → 8 structured spec answers.
    println!(
        "\n{C_DIM}Meta AI 正在理解你的意图（约 15-30 秒）...{C_RESET}"
    );
    let answers = expand_intent_to_answers(provider, &meta_key, &intent)
        .unwrap_or_else(|err| {
            eprintln!(
                "{C_YELLOW}Meta AI 扩展失败：{err}{C_RESET}"
            );
            eprintln!(
                "{C_DIM}回退：把意图填入所有 8 个槽位（cmd_spec 仍可处理）。{C_RESET}"
            );
            vec![intent.clone(); 8]
        });

    // Show user what Meta AI inferred (Software 3.0 transparency).
    println!("\n{C_CYAN}Meta AI 理解的 spec：{C_RESET}");
    for (i, a) in answers.iter().enumerate() {
        let short = if a.chars().count() > 80 {
            let truncated: String = a.chars().take(80).collect();
            format!("{}…", truncated)
        } else {
            a.clone()
        };
        println!("  {C_DIM}{}. {}{C_RESET}", i + 1, short);
    }
    println!();
    let ok = prompt_yes_no("看着对吗？继续生成？(Looks right? Continue?)", true)?;
    if !ok {
        return Err(String::from(
            "user aborted at spec preview; rerun `turingos` to try again",
        ));
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
    // NOTE: cmd_spec::run expects only flags (no leading subcommand name).
    let spec_args = vec![
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
    // NOTE: cmd_generate::run expects only flags (no leading subcommand name).
    let gen_args = vec![
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
// Meta AI expansion — Software 3.0 core
// ─────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-N16: 1-sentence intent → 8-answer spec via Meta LLM.
///
/// Calls the same SiliconFlow-compatible client cmd_spec uses, with a
/// system prompt that elicits a strict JSON array of 8 strings keyed to
/// the canonical TuringOS spec interview questions. Falls back caller-side
/// if the LLM call or JSON parse fails.
fn expand_intent_to_answers(
    provider: &str,
    api_key: &str,
    intent: &str,
) -> Result<Vec<String>, String> {
    use crate::siliconflow_client::{chat_complete_blocking, ChatMessage};

    let model = if provider == "deepseek" {
        "deepseek-v4-pro"
    } else {
        crate::siliconflow_client::DEFAULT_META_MODEL
    };

    let system = "你是 TuringOS 的意图扩展器。用户给你一两句自然语言，描述他们想要什么。\
你输出一个严格的 JSON 数组，恰好 8 个字符串，每个对应一个 spec 面试问题：\n\
1. The Job — 这个东西做的一件事是什么？\n\
2. The Anchor — 它和什么已有工具/网站/App 类似？\n\
3. What it Remembers — 哪些数据要持久化？\n\
4. First-Click Walk-Through — 用户打开后第一步做什么？分步描述。\n\
5. Weird-User Test — 用户做什么奇怪操作时应该正常工作？\n\
6. Disappointment Boundary — 它明确不做什么？\n\
7. Success Test — 30 天后怎么判断它成功了？\n\
8. Playback — 用一句话复述要构建的目标。\n\n\
约束：\n\
- 每个回答 1-2 句，具体可执行；\n\
- 严格忠实于用户意图，不要发明无关功能；\n\
- 如果用户意图不是游戏，也要产出一个 HTML 可交付物的 spec（例如交互式调研页、教学 demo、工具 UI）；\n\
- 只输出 JSON 数组，不要 markdown 代码围栏，不要前后解释。";

    let user_msg = format!(
        "用户意图：{intent}\n\n现在输出 8 个回答的 JSON 数组。"
    );

    let result = chat_complete_blocking(
        api_key,
        model,
        &[ChatMessage::system(system), ChatMessage::user(&user_msg)],
        Some(2000),
        Some(0.5),
        None,
    )
    .map_err(|e| format!("LLM call: {e}"))?;

    // Strip optional ```json / ``` fences if Meta AI added any.
    let mut content = result.content.trim().to_string();
    if let Some(rest) = content.strip_prefix("```json") {
        content = rest.trim().to_string();
    } else if let Some(rest) = content.strip_prefix("```") {
        content = rest.trim().to_string();
    }
    if let Some(rest) = content.strip_suffix("```") {
        content = rest.trim().to_string();
    }

    let answers: Vec<String> = serde_json::from_str(&content).map_err(|e| {
        let preview: String = content.chars().take(200).collect();
        format!("parse JSON: {e}; content preview: {preview}")
    })?;

    if answers.len() != 8 {
        return Err(format!("expected 8 answers, got {}", answers.len()));
    }

    Ok(answers)
}

// ─────────────────────────────────────────────────────────────────────────────
// UI helpers — pure stdlib + ANSI, zero new deps
// ─────────────────────────────────────────────────────────────────────────────

fn print_banner() {
    println!("{C_CYAN}{C_BOLD}");
    println!("  ████████  TuringOS  — 描述一个想法，获得可玩的 HTML");
    println!("  ████████  TuringOS  — Describe an idea, get a playable HTML");
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
