//! TRACE_MATRIX FC2-N16 + FC3 evidence binding: turingos spec handler (Phase 6.3)
//!
//! The "spec grill" — an 8-question customer-development interview that
//! extracts software requirements from a NON-DEVELOPER user via natural
//! language. Pulls from JTBD (Moesta), Mom Test (Fitzpatrick), Voss
//! mirroring/labeling, 5-Whys (Toyoda), and IDEO empathy interviewing.
//! Synthesised by independent research agent 2026-05-17.
//!
//! Output artifacts:
//!   - `<workspace>/spec.md`              (human-readable spec)
//!   - `<workspace>/spec_transcript.jsonl` (every Q/A turn + LLM usage)
//!   - CAS EvidenceCapsule of spec.md      (via spec_capsule.rs; CID printed)
//!
//! Modes:
//!   - INTERACTIVE (default): reads answers from stdin, one per question,
//!     blank line ends an answer block.
//!   - SCRIPTED (`--answers-file <PATH>`): reads a JSON array of 8 strings,
//!     uses them in order. Enables reproducible demo simulations.
//!
//! Class 1: filesystem write to workspace + Class 2 CAS wire via the
//! existing CasStore public surface. No Class-4 schema change.

use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::cmd_llm;
use crate::siliconflow_client::{chat_complete_blocking, require_api_key, ChatMessage, LlmError};
use crate::spec_capsule;

/// TRACE_MATRIX FC2-N16: `spec` short-help
pub(crate) const SHORT_HELP: &str =
    "Interview the user (8-question grill) and emit a spec.md anchored in CAS";

/// TRACE_MATRIX FC2-N16: `spec` full --help text
pub(crate) const FULL_HELP: &str = r#"turingos spec — Customer-development interview, emit spec.md + CAS capsule

USAGE:
    turingos spec --workspace <PATH> [--answers-file <PATH>] [--lang <zh|en>]

ACTIONS:
    (default)   Run the 8-question grill against the configured Meta LLM,
                emit spec.md + spec_transcript.jsonl + CAS EvidenceCapsule.

OPTIONS:
    --workspace <PATH>       Workspace directory (required; must contain
                             turingos.toml from `turingos llm config`).
    --answers-file <PATH>    JSON array of 8 strings — used non-interactively
                             for scripted runs / demos / regression tests.
    --lang <zh|en>           Interview language. Default: zh (中文).
    --skip-llm               Skip LLM calls (use the canonical 8 questions
                             verbatim + emit a minimal spec.md). Useful when
                             SILICONFLOW_API_KEY is unset and you only want
                             to test the CAS wire.
    -h, --help               Print this help.

INTERVIEW FLOW (assumes user is NOT a developer):
    Q1  The Job (JTBD opener): tell me about a recent moment when you
        thought "I wish I had a tool for this".
    Q2  The Anchor: any website / app that's even a little like what you want?
    Q3  What it Remembers: what should the program still know tomorrow morning?
    Q4  First-Click Walk-Through: what does the user see / click first?
    Q5  Weird-User Test: what should NOT break it?
    Q6  Disappointment Boundary: which features would feel like "scope creep"?
    Q7  Success Test: how will you KNOW it's doing its job after a month?
    Q8  Playback (mirror): seven-row fridge note — user confirms or corrects.

METHODOLOGY (sources): Customer Development (Blank), JTBD switch interview
    (Moesta), The Mom Test three sins (Fitzpatrick), Voss mirroring &
    labeling, 5-Whys (Toyoda), IDEO empathy interview, EARS syntax (Mavin),
    user story mapping (Patton). LLMREI arXiv 2507.02564.

OUTPUTS:
    <workspace>/spec.md                       Human-readable spec.
    <workspace>/spec_transcript.jsonl         Every Q/A turn + LLM usage.
    CAS EvidenceCapsule (schema=turingos-spec-capsule-v1) with CID printed.
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lang {
    Zh,
    En,
}

impl Lang {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "zh" | "cn" | "中文" => Ok(Self::Zh),
            "en" | "english" => Ok(Self::En),
            other => Err(format!("invalid --lang '{other}': expect zh|en")),
        }
    }
}

#[derive(Debug)]
enum SpecError {
    MissingFlag(&'static str),
    WorkspaceNotFound(String),
    BadAnswersFile(String),
    Io(String),
    Llm(LlmError),
    Capsule(spec_capsule::CapsuleError),
    NeedAnswersFileWhenSkippingLlm,
}

impl std::fmt::Display for SpecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFlag(flag) => write!(f, "missing required flag: {flag}"),
            Self::WorkspaceNotFound(p) => write!(f, "workspace not found: {p}"),
            Self::BadAnswersFile(e) => write!(f, "bad --answers-file: {e}"),
            Self::Io(e) => write!(f, "io: {e}"),
            Self::Llm(e) => write!(f, "{e}"),
            Self::Capsule(e) => write!(f, "{e}"),
            Self::NeedAnswersFileWhenSkippingLlm => write!(
                f,
                "--skip-llm requires --answers-file (cannot run an interactive grill without an LLM)"
            ),
        }
    }
}

impl From<LlmError> for SpecError {
    fn from(e: LlmError) -> Self {
        Self::Llm(e)
    }
}

impl From<spec_capsule::CapsuleError> for SpecError {
    fn from(e: spec_capsule::CapsuleError) -> Self {
        Self::Capsule(e)
    }
}

/// TRACE_MATRIX FC2-N16: `spec` dispatch entry
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{FULL_HELP}");
        return ExitCode::SUCCESS;
    }
    match run_inner(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("turingos spec: {e}");
            ExitCode::from(2)
        }
    }
}

fn run_inner(args: &[String]) -> Result<(), SpecError> {
    let mut workspace = PathBuf::from(".");
    let mut answers_file: Option<PathBuf> = None;
    let mut lang = Lang::Zh;
    let mut skip_llm = false;

    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--workspace" => {
                workspace =
                    PathBuf::from(iter.next().ok_or(SpecError::MissingFlag("--workspace"))?);
            }
            "--answers-file" => {
                answers_file = Some(PathBuf::from(
                    iter.next()
                        .ok_or(SpecError::MissingFlag("--answers-file"))?,
                ));
            }
            "--lang" => {
                let v = iter.next().ok_or(SpecError::MissingFlag("--lang"))?;
                lang = Lang::parse(v).map_err(SpecError::BadAnswersFile)?;
            }
            "--skip-llm" => {
                skip_llm = true;
            }
            _ => {}
        }
    }

    if !workspace.exists() {
        return Err(SpecError::WorkspaceNotFound(
            workspace.display().to_string(),
        ));
    }
    if skip_llm && answers_file.is_none() {
        return Err(SpecError::NeedAnswersFileWhenSkippingLlm);
    }

    let questions = canonical_questions(lang);

    // Gather 8 answers — either from --answers-file or via interactive stdin.
    let answers: Vec<String> = if let Some(path) = answers_file.as_ref() {
        load_answers_from_file(path)?
    } else {
        interactive_gather(&questions)?
    };

    if answers.len() != 8 {
        return Err(SpecError::BadAnswersFile(format!(
            "expected exactly 8 answers, got {}",
            answers.len()
        )));
    }

    // Build the LLM-facing transcript (one system + 8 Q/A user turns).
    let mut transcript = Vec::new();
    transcript.push(TurnRecord {
        role: "system".into(),
        content: system_prompt(lang),
        model: None,
        usage_total_tokens: 0,
    });
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        transcript.push(TurnRecord {
            role: "user".into(),
            content: format!("Q{}: {}\nA{}: {}", i + 1, q, i + 1, a),
            model: None,
            usage_total_tokens: 0,
        });
    }

    let model_id = cmd_llm::read_meta_model(&workspace);
    let api_key_env = cmd_llm::read_api_key_env_var(&workspace);

    let (synthesis, total_tokens) = if skip_llm {
        // CAS-wire-only path: synthesise spec.md without LLM (uses canonical
        // question phrasings + the raw user answers; no playback critique).
        let synth = synthesise_spec_md_no_llm(lang, &questions, &answers);
        (synth, 0u64)
    } else {
        let api_key = require_api_key(&api_key_env)?;
        let synth_user_msg = build_synthesis_user_message(lang, &questions, &answers);
        let messages = vec![
            ChatMessage::system(system_prompt(lang)),
            ChatMessage::user(synth_user_msg.clone()),
        ];
        eprintln!("[spec] calling Meta LLM ({model_id}) to synthesise spec.md...");
        let result = chat_complete_blocking(&api_key, &model_id, &messages, Some(3000), Some(0.3))?;
        transcript.push(TurnRecord {
            role: "user".into(),
            content: synth_user_msg,
            model: Some(model_id.clone()),
            usage_total_tokens: 0,
        });
        transcript.push(TurnRecord {
            role: "assistant".into(),
            content: result.content.clone(),
            model: Some(model_id.clone()),
            usage_total_tokens: result.usage.total_tokens,
        });
        (result.content, result.usage.total_tokens)
    };

    let spec_md = wrap_spec_md(&synthesis, &questions, &answers, &model_id, skip_llm);
    let spec_md_path = workspace.join("spec.md");
    fs::write(&spec_md_path, &spec_md).map_err(|e| SpecError::Io(format!("write spec.md: {e}")))?;

    let transcript_path = workspace.join("spec_transcript.jsonl");
    write_transcript_jsonl(&transcript_path, &transcript)?;

    let logical_t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let cid_hex = spec_capsule::write_spec_capsule(&workspace, &spec_md, "user", logical_t)?;

    println!();
    println!("Spec interview complete.");
    println!("  spec.md            -> {}", spec_md_path.display());
    println!("  spec_transcript    -> {}", transcript_path.display());
    println!("  CAS capsule CID    -> {cid_hex}");
    println!(
        "                       (schema: {})",
        spec_capsule::SPEC_CAPSULE_SCHEMA_ID
    );
    if total_tokens > 0 {
        println!("  LLM total tokens   -> {total_tokens}");
    }
    println!();
    println!(
        "Next step: turingos generate --workspace {}",
        workspace.display()
    );
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Canonical 8-question flow (research-derived)
// ─────────────────────────────────────────────────────────────────────────────

fn canonical_questions(lang: Lang) -> Vec<String> {
    match lang {
        Lang::Zh => vec![
            // Q1 — The Job (JTBD opener; no jargon)
            "先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？\
比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？".into(),
            // Q2 — The Anchor (let user supply anchor)
            "有没有哪个网站 / App / 小工具，跟你想要的『有点像』？不用一模一样，一两个相似的地方就行。\
（如果想不出来：那纸笔 / Excel / 微信群里现在是怎么做的？）".into(),
            // Q3 — Data model in plain words
            "想象关掉电脑明天再打开，这个工具应该还『记得』哪些东西？比如团购账本会记得：\
每个人的名字、买了什么、付了多少、还欠多少。你的工具要记得什么？".into(),
            // Q4 — First-click walkthrough
            "假设我是你的用户，第一次打开这个工具——我看到什么？然后我点什么？然后呢？\
一步一步告诉我，直到我完成一件事。".into(),
            // Q5 — Weird-user test (Mom-Test sin-3 antidote, specifics)
            "如果有个奇怪的用户，故意乱点乱填——比如把『金额』填成『哈哈哈』，\
或者同一个名字录入 50 遍——你希望工具怎么办？报错？忽略？还是有别的反应？".into(),
            // Q6 — Disappointment boundary (inverse framing surfaces real priorities)
            "如果这个工具突然多了一个功能，你反而会觉得『搞这个干嘛，反而把简单的事弄复杂了』——\
是什么功能？说两三个。".into(),
            // Q7 — Success test (past-cost framing)
            "用了一个月之后，你怎么判断『这个工具是有用的』？不是『感觉不错』那种——\
是具体能数出来或看得见的事。比如：『我妈现在不用每周日花两小时算账了。』".into(),
            // Q8 — Playback / mirror (Voss labeling)
            "（最后一题）下面我会把前面听到的复述一遍，请你看看哪里我听错了或听漏了——\
别客气，挑错就是帮我。如果你想直接补充什么，请在这里写出来。".into(),
        ],
        Lang::En => vec![
            "Forget about code for now. Tell me about a recent moment when you thought \
'I wish I had a tool for this.' For example: 'My mom does community group-buy accounting \
every week in Excel and it's painful.' What's your story?".into(),
            "Is there a website, app, or tool that's even a little bit like what you want? \
Doesn't have to be exact — just one or two similar pieces. (If you can't name one: \
'How do you do this today with paper, Excel, or a chat group?')".into(),
            "Imagine you close the program and open it tomorrow — what should it still \
'remember'? A group-buy tracker remembers: each person's name, what they bought, how \
much they paid, what they still owe. What does yours remember?".into(),
            "Pretend I'm your user opening this for the first time. What do I see? What do \
I click? Then what? Walk me through, step by step, until I finish one task.".into(),
            "If a weird user messes around — types 'lolol' into the price field, or enters \
the same name 50 times — what should the tool do? Show an error? Ignore it? Something else?".into(),
            "If the tool grew a new feature and your reaction was 'why did you add this, \
you've made the simple thing complicated' — name two or three such features.".into(),
            "After one month of using it, how do you know it's actually working? Not 'feels \
nice' — something countable or visible. Like: 'My mom no longer spends two hours every \
Sunday doing the math.'".into(),
            "(Last question) I'll play back what I heard. Tell me which line is wrong or \
incomplete — corrections help me. If you want to add anything directly, write it here.".into(),
        ],
    }
}

fn system_prompt(lang: Lang) -> String {
    match lang {
        Lang::Zh => r#"你是 TuringOS Meta AI，一名以非开发者用户为对象的需求引导专家。
你的任务：根据下面的 8 个问题 + 用户的 8 个回答，综合出一份 spec.md。

**严格要求**：
1. 假设用户**不是程序员**。不要使用任何技术术语（"数据模型"、"用户流"、"API"、"schema"、"endpoint"、"validation"等）。
2. 输出 **Markdown** 格式，分为以下小节（章节标题用中文）：
   - `## 一句话目标`（用户原话提炼）
   - `## 我们要做什么 (Goal)`（一段话，2-4 句）
   - `## 像谁 (Reference)`（最像的现成产品 / 现有做法）
   - `## 程序要记住的东西 (Memory)`（项目符号列表，每条 ≤ 12 字）
   - `## 第一次使用 (First Run)`（编号步骤，最多 7 步）
   - `## 不能搞坏的情况 (Robustness)`（项目符号；每条≤ 20 字）
   - `## 故意不做的 (Out of Scope)`（项目符号）
   - `## 算成功 (Acceptance)`（可测量的成功指标 1-3 条）
   - `## Given/When/Then 用例`（3-5 个验收用例，BDD 格式）
   - `## 一句话给 AI 编程员`（给下一步 codegen 的纯文本提示，一段话）
3. 如果你**发现矛盾**（比如用户说"要简单"但列出 17 个功能），在 spec 末尾加 `## 我听到的矛盾` 小节，用 Voss-label 方式描述：
   "听起来 X 对你很重要，同时你也说了 Y。如果要砍掉一个，你会保留哪个？"
4. 不要扩写用户没说的功能。如果某项信息缺失，在 spec 末尾加 `## 还没问到` 小节。
5. 最后一行必须是单独一行的 `<!-- TURINGOS_SPEC_END -->`。

输出**只有 spec.md 正文**，不要前后加任何 "好的我来帮你"之类的客套话。"#.into(),
        Lang::En => r#"You are TuringOS Meta AI, a requirements-elicitation specialist for non-developer users.
Task: from the 8 questions + 8 answers below, synthesise a spec.md.

**Strict rules**:
1. Assume the user is **not a programmer**. NO jargon ("data model", "user flow", "API",
   "schema", "endpoint", "validation"). Translate to plain English.
2. Output **Markdown** with these exact sections:
   - `## One-line Goal`
   - `## What We're Building (Goal)` (2-4 sentences)
   - `## Like What (Reference)`
   - `## What the Program Remembers (Memory)` (bullets, ≤ 8 words each)
   - `## First Run (First Click Walk)` (numbered, ≤ 7 steps)
   - `## What It Must Not Break On (Robustness)` (bullets)
   - `## Deliberately NOT Doing (Out of Scope)` (bullets)
   - `## Success Looks Like (Acceptance)` (1-3 measurable lines)
   - `## Given/When/Then Examples` (3-5 BDD scenarios)
   - `## One-line Brief to the AI Coder`
3. If you spot contradictions, append a `## Contradictions I Heard` section using
   Voss labeling: "It sounds like X matters AND you also said Y — which one wins?"
4. Don't invent features the user didn't mention. If something is missing, append
   `## Not Yet Asked` listing what.
5. Final line MUST be `<!-- TURINGOS_SPEC_END -->` alone.

Output ONLY the spec.md body, no preamble."#.into(),
    }
}

fn build_synthesis_user_message(lang: Lang, questions: &[String], answers: &[String]) -> String {
    let intro = match lang {
        Lang::Zh => "下面是 8 个 Q/A，请按系统提示综合 spec.md：",
        Lang::En => "Here are the 8 Q/A pairs — synthesise spec.md per the system prompt:",
    };
    let mut s = String::new();
    s.push_str(intro);
    s.push_str("\n\n");
    for i in 0..8 {
        s.push_str(&format!("Q{}: {}\n", i + 1, questions[i]));
        s.push_str(&format!("A{}: {}\n\n", i + 1, answers[i]));
    }
    s
}

// ─────────────────────────────────────────────────────────────────────────────
// Interactive stdin gather (TTY)
// ─────────────────────────────────────────────────────────────────────────────

fn interactive_gather(questions: &[String]) -> Result<Vec<String>, SpecError> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut answers = Vec::with_capacity(8);

    println!("turingos spec — 8-question grill");
    println!("Type your answer; press ENTER on an empty line to submit each answer.");
    println!();

    for (i, q) in questions.iter().enumerate() {
        println!("Q{}: {}", i + 1, q);
        print!("> ");
        stdout.flush().map_err(|e| SpecError::Io(e.to_string()))?;
        let mut buf = String::new();
        loop {
            let mut line = String::new();
            let n = stdin
                .lock()
                .read_line(&mut line)
                .map_err(|e| SpecError::Io(e.to_string()))?;
            if n == 0 {
                break;
            }
            if line.trim().is_empty() {
                break;
            }
            buf.push_str(&line);
        }
        answers.push(buf.trim().to_string());
        println!();
    }
    Ok(answers)
}

fn load_answers_from_file(path: &Path) -> Result<Vec<String>, SpecError> {
    let raw = fs::read_to_string(path)
        .map_err(|e| SpecError::BadAnswersFile(format!("read {}: {e}", path.display())))?;
    let parsed: serde_json::Value = serde_json::from_str(&raw)
        .map_err(|e| SpecError::BadAnswersFile(format!("JSON parse: {e}")))?;
    let arr = parsed
        .as_array()
        .ok_or_else(|| SpecError::BadAnswersFile("expected top-level JSON array".into()))?;
    let mut out = Vec::with_capacity(arr.len());
    for (i, v) in arr.iter().enumerate() {
        let s = v
            .as_str()
            .ok_or_else(|| SpecError::BadAnswersFile(format!("element {i} is not a string")))?;
        out.push(s.to_string());
    }
    Ok(out)
}

// ─────────────────────────────────────────────────────────────────────────────
// Transcript JSONL persistence
// ─────────────────────────────────────────────────────────────────────────────

struct TurnRecord {
    role: String,
    content: String,
    model: Option<String>,
    usage_total_tokens: u64,
}

fn write_transcript_jsonl(path: &Path, turns: &[TurnRecord]) -> Result<(), SpecError> {
    let mut out = String::new();
    for t in turns {
        let model = t.model.as_deref().unwrap_or("");
        let obj = serde_json::json!({
            "role": t.role,
            "content": t.content,
            "model": model,
            "usage_total_tokens": t.usage_total_tokens,
        });
        out.push_str(&obj.to_string());
        out.push('\n');
    }
    fs::write(path, out).map_err(|e| SpecError::Io(format!("write transcript: {e}")))?;
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// LLM-less synthesis fallback (for --skip-llm CAS-wire smoke tests)
// ─────────────────────────────────────────────────────────────────────────────

fn synthesise_spec_md_no_llm(lang: Lang, questions: &[String], answers: &[String]) -> String {
    let mut s = String::new();
    match lang {
        Lang::Zh => {
            s.push_str("## 一句话目标\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 我们要做什么 (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## 像谁 (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## 程序要记住的东西 (Memory)\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## 第一次使用 (First Run)\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## 不能搞坏的情况 (Robustness)\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## 故意不做的 (Out of Scope)\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## 算成功 (Acceptance)\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## 用户补充\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## 一句话给 AI 编程员\n\n");
            s.push_str("根据上面的 Goal / Memory / First Run 实现一个最小可用版本。");
        }
        Lang::En => {
            s.push_str("## One-line Goal\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## What We're Building (Goal)\n\n");
            s.push_str(&answers[0]);
            s.push_str("\n\n## Like What (Reference)\n\n");
            s.push_str(&answers[1]);
            s.push_str("\n\n## What the Program Remembers\n\n");
            s.push_str(&answers[2]);
            s.push_str("\n\n## First Run\n\n");
            s.push_str(&answers[3]);
            s.push_str("\n\n## What It Must Not Break On\n\n");
            s.push_str(&answers[4]);
            s.push_str("\n\n## Deliberately NOT Doing\n\n");
            s.push_str(&answers[5]);
            s.push_str("\n\n## Success Looks Like\n\n");
            s.push_str(&answers[6]);
            s.push_str("\n\n## User Additions\n\n");
            s.push_str(&answers[7]);
            s.push_str("\n\n## One-line Brief to AI Coder\n\n");
            s.push_str("Implement a minimal version using the Goal / Memory / First Run above.");
        }
    }
    let _ = questions; // suppress unused warning
    s.push_str("\n\n<!-- TURINGOS_SPEC_END -->\n");
    s
}

/// Wrap the LLM-synthesised body with a header (model id + timestamp) and an
/// appendix (raw Q/A for audit). The CAS capsule hashes this WHOLE blob, so
/// future replay can derive both the formatted spec and the raw transcript
/// from the single capsule CID.
fn wrap_spec_md(
    body: &str,
    questions: &[String],
    answers: &[String],
    model_id: &str,
    skipped_llm: bool,
) -> String {
    let mut s = String::new();
    s.push_str("# TuringOS Spec (Phase 6.3)\n\n");
    s.push_str(&format!(
        "> Generated by `turingos spec` — meta model: `{model_id}`"
    ));
    if skipped_llm {
        s.push_str(" (skip-llm: no synthesis call made)");
    }
    s.push_str("\n\n");
    s.push_str(body.trim_end());
    s.push_str("\n\n---\n\n");
    s.push_str("## Appendix — Raw Q/A (for audit)\n\n");
    for (i, (q, a)) in questions.iter().zip(answers.iter()).enumerate() {
        s.push_str(&format!("**Q{}**: {q}\n\n", i + 1));
        s.push_str(&format!("**A{}**: {a}\n\n", i + 1));
    }
    s
}
