//! TRACE_MATRIX FC2-N16 + FC1a-rtool + FC1a-judge_pi:
//! turingos tdma — TDMA-Bounded production runner.
//!
//! Atom 18 thin-shim refactor: the kernel-driving loop now lives in
//! `turingosv4::tdma_runner`. This subcommand contributes only:
//!   * workspace config + API-key resolution via cmd_llm helpers
//!   * `AnyJudge` selection by `--judge` flag
//!   * per-judge prompt builders
//!   * a closure adapter from chat_client::chat_complete_blocking
//!     (production path) to the runner's LlmResponse contract
//!
//! Compared with Atoms 12-14 (which targeted DeepSeek via the local
//! `llm_proxy.py`), this binary path routes through the same
//! `chat_client::chat_complete_blocking` that `turingos llm complete`
//! and `turingos generate` use — production traffic.
//!
//! K10 + K11 fix (Atom 18): the JudgeDriver trait + 3 impls and the
//! kernel/probe/evidence helpers that previously lived here have all moved
//! to `src/tdma_runner.rs`. Net LOC reduction: ~600 lines.
//!
//! On-disk §8: handover/directives/2026-05-22_TDMA_BOUNDED_RC1_DIRECTIVE_AND_§8.md

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::chat_client::{chat_complete_blocking, require_api_key, ChatMessage, LlmError};
use crate::cmd_llm;
use turingosv4::judges::swebench_test_judge::SwebenchTestJudge;
use turingosv4::tdma_runner::{run_proof, AnyJudge, LlmResponse, RunConfig};

/// TRACE_MATRIX FC1a-rtool_input: Minimal SWE-bench sample shape needed to
/// drive the loop. Deliberately omits `gold_patch` / `test_patch` so the
/// shielded prompt builder can never read them (CLAUDE.md §4 / AGENTS.md §12).
/// `serde` ignores extra fields, so a full SWE-bench sample JSON parses fine.
#[derive(Debug, Clone, serde::Deserialize)]
struct SwebenchSampleInput {
    instance_id: String,
    repo: String,
    base_commit: String,
    problem_statement: String,
    #[serde(default)]
    hints_text: Option<String>,
    #[serde(default)]
    fail_to_pass: Vec<String>,
}

/// TRACE_MATRIX FC2-N16: `tdma` short-help (registry display).
pub(crate) const SHORT_HELP: &str =
    "Drive the TDMA-Bounded memory kernel against a step-by-step proof via production LLM";

/// TRACE_MATRIX FC2-N16: `tdma` full --help text.
pub(crate) const FULL_HELP: &str = r#"turingos tdma — TDMA-Bounded production runner

USAGE:
    turingos tdma run --workspace <PATH>
                       [--judge <nesbitt|putnam_a1|putnam_2025_b3|swebench>]
                       [--role <meta|blackbox>]
                       [--evidence-dir <PATH>]
                       [--max-attempts-per-stage <N>]
                       [--temperature <FLOAT>]
                       [--tape-backend <memory|git>]
                       [--swebench-sample <PATH>]
                       [--swebench-python <PATH>]
                       [--swebench-dataset <NAME>]
                       [--swebench-workdir <PATH>]

ACTIONS:
    run    Boot a TDMA-Bounded memory kernel, drive it stage-by-stage
           through a structured proof using the SAME SiliconFlow client
           that `turingos llm complete` uses (production endpoint, not
           the local test proxy). Capture ChainTape + per-attempt probes
           to <evidence-dir> (default: <workspace>/artifacts/tdma/<TS>/).

JUDGES:
    nesbitt          Nesbitt's inequality (8 stages; default)
    putnam_a1        Putnam 2024 A1 — 2-adic infinite descent (8 stages)
    putnam_2025_b3   Putnam 2025 B3 — divisor closure (5 stages; post-cutoff)
    swebench         SWE-bench coding repair (1 stage `Repair`, retried). Each
                     candidate patch is verified by the REAL official swebench
                     python harness (hidden-test execution); on failure the
                     real failing-test names are fed back so the loop retries.
                     Requires --swebench-sample.

OPTIONS:
    --workspace <PATH>          Workspace directory containing turingos.toml
    --judge <NAME>              Judge selector (default: nesbitt)
    --role <meta|blackbox>      Which configured model to use (default: meta)
    --evidence-dir <PATH>       Override evidence output directory
    --max-attempts-per-stage <N>  Hard cap per stage (default: 6)
    --temperature <FLOAT>       Sampling temperature (default: 0.7)
    --swebench-sample <PATH>    [swebench] JSON of one SWE-bench sample
                                (instance_id, repo, base_commit,
                                problem_statement, hints_text, fail_to_pass).
                                gold_patch/test_patch are NEVER read into the
                                prompt even if present.
    --swebench-python <PATH>    [swebench] python with the `swebench` package
                                installed (default:
                                /Users/zephryj/.venv-swebench/bin/python)
    --swebench-dataset <NAME>   [swebench] HF dataset name
                                (default: princeton-nlp/SWE-bench_Lite)
    --swebench-workdir <PATH>   [swebench] harness work dir (default: a
                                `swebench_work` subdir of the evidence dir)
    --tape-backend <NAME>       Tape substrate. **`git` is the DEFAULT**
                                as of Atom 25 (Phase E full cutover):
                                GitTapeLedger at
                                `<workspace>/tdma_tape.git` (Path B; real-git
                                via git2-rs). `memory` = MemoryTapeLedger
                                (in-process; Path A) — still accepted for
                                tests + emergency in-process rollback.
    -h, --help                  Print this help

DESCRIPTION:
    Production wire-up of the TDMA-Bounded RC1 kernel (Atoms 0-17).
    Reuses the `chat_client::chat_complete_blocking` path so
    requests go to the configured SiliconFlow endpoint with the API key
    in the env var named in turingos.toml.

    Per-attempt evidence (probe + ChainTape + manifest) is written into
    the evidence directory. Failures DO NOT escape into the next prompt;
    the distiller compresses each rejection into a bounded BBS entry.
"#;

const LEAK_SENTINEL: &str = "TURINGOS_TDMA_PROD_LEAK_CANARY_R3K8M";

// ── Per-judge problem texts ────────────────────────────────────────

const PROBLEM_TEXT_NESBITT: &str = r#"Prove Nesbitt's inequality for positive reals:
    a/(b+c) + b/(a+c) + c/(a+b) >= 3/2

Canonical proof (8 stages):
  Stage 1: Substitute x=b+c, y=a+c, z=a+b.
  Stage 2: Rewrite each a/(b+c) in terms of x,y,z.
  Stage 3: Expand into six separate fractions.
  Stage 4: Group into three pairs (x/y + y/x), etc.
  Stage 5: Apply AM-GM: each pair >= 2.
  Stage 6: Sum the three pairs: total >= 6.
  Stage 7: Subtract 3 to recover the original form.
  Stage 8: Conclude >= 3/2 with equality iff a=b=c."#;

const PROBLEM_TEXT_PUTNAM_A1: &str = r#"Putnam 2024 A1.
Determine all positive integers n such that there exist positive integers
a, b, c with 2*a^n + 3*b^n = 4*c^n.

Canonical proof (8 stages):
  Stage 1: Verify n = 1 works via witness (a,b,c) = (1, 2, 2).
  Stage 2: For n >= 2, WLOG assume gcd(a,b,c) = 1.
  Stage 3: Case n = 2 - derive a^2 + c^2 ≡ 0 (mod 3); 0, 1 are squares mod 3.
  Stage 4: Show b is also multiple of 3, contradicting gcd = 1.
  Stage 5: Case n >= 3 - from 3*b^n = 4*c^n - 2*a^n, derive b is even.
  Stage 6: Rewriting forces a to be even.
  Stage 7: One more rewriting forces c to be even.
  Stage 8: All three even contradicts gcd = 1; therefore n = 1 is unique."#;

const PROBLEM_TEXT_PUTNAM_B3: &str = r#"Putnam 2025 B3 (post-cutoff, December 2025).
Suppose S is a nonempty set of positive integers with the property that if
n is in S, then every positive divisor of 2025n - 15n is in S. Must S
contain all positive integers?

Answer: NO.
Canonical proof (5 stages):
  Stage 1: Simplify 2025n - 15n = (2025-15)n = 2010n.
  Stage 2: Factor 2010 = 2 * 3 * 5 * 67 into its four prime factors.
  Stage 3: Argue: divisors of 2010n introduce only primes from {2,3,5,67}.
  Stage 4: Construct counterexample S (closure of {1} under "n -> divisors of 2010n").
  Stage 5: Conclude NO: e.g., 7 is never in S."#;

fn make_system_prompt(judge_name: &str, stage_label: &str) -> String {
    let (problem_label, problem_specific) = match judge_name {
        "nesbitt" => (
            "Nesbitt's inequality",
            "Use concrete algebra; reference the AM-GM substitution outline.",
        ),
        "putnam_a1" => (
            "Putnam 2024 A1",
            "Be RIGOROUS — explicit modular arithmetic, explicit WLOG gcd(a,b,c)=1, explicit 'b is even'.",
        ),
        "putnam_2025_b3" => (
            "Putnam 2025 B3",
            "Be RIGOROUS — explicit algebraic simplification, explicit prime factorization, explicit counterexample.",
        ),
        _ => ("the proof", ""),
    };
    let stage_specific = match (judge_name, stage_label) {
        ("putnam_2025_b3", "Stage4-Counterexample-Construction") => {
            "For Stage 4, explicitly write all three facts: Define S from 1; S is closed under positive divisors of 2010n; and include the exact sentence fragment 'the prime 7 is not in S'."
        }
        ("putnam_2025_b3", "Stage5-Conclude-NO") => {
            "For Stage 5, begin with the literal conclusion 'The answer is NO' and state that S need not contain all positive integers."
        }
        _ => "",
    };
    format!(
        r#"You are a mathematics worker proving {problem_label} step-by-step.
Output EXACTLY ONE next step.

Your output MUST start with this JSON object on the FIRST line:
{{"schema_version":"tdma-state-update/v1","status":"Proceed","task_id":"<STAGE>","action":"PROPOSE","failed_predicate":null,"reject_class":null,"next_action_hint":null,"evidence_hash":null}}

Replace <STAGE> with the current stage label (e.g. "{stage_label}").
After the JSON write on a new line:
---BODY---
Then write your step in 1-5 sentences. {problem_specific}
{stage_specific}

Current stage: {stage_label}"#,
        problem_label = problem_label,
        problem_specific = problem_specific,
        stage_specific = stage_specific,
        stage_label = stage_label
    )
}

fn make_user_prompt(judge_name: &str, stage_label: &str, accepted_steps: &[String]) -> String {
    let problem_text = match judge_name {
        "nesbitt" => PROBLEM_TEXT_NESBITT,
        "putnam_a1" => PROBLEM_TEXT_PUTNAM_A1,
        "putnam_2025_b3" => PROBLEM_TEXT_PUTNAM_B3,
        _ => "(unknown problem)",
    };
    let mut s = String::new();
    s.push_str(&format!("Problem:\n{}\n\n", problem_text));
    s.push_str(&format!("Current stage: {}\n\n", stage_label));
    if judge_name == "putnam_2025_b3" && stage_label == "Stage4-Counterexample-Construction" {
        s.push_str(
            "Stage 4 checklist: define S from 1; state S is closed under positive divisors of 2010n; include the exact sentence fragment 'the prime 7 is not in S'.\n\n",
        );
    } else if judge_name == "putnam_2025_b3" && stage_label == "Stage5-Conclude-NO" {
        s.push_str(
            "Stage 5 checklist: begin with 'The answer is NO'; explain that S need not contain all positive integers because the Stage 4 set omits 7.\n\n",
        );
    }
    if accepted_steps.is_empty() {
        s.push_str("No prior steps yet. Write Stage 1.");
    } else {
        s.push_str("Prior accepted steps:\n");
        for (i, st) in accepted_steps.iter().enumerate() {
            s.push_str(&format!("  Step {}: {}\n", i + 1, st));
        }
        s.push_str("\nWrite the next single step (do NOT repeat prior steps).");
    }
    s
}

// ── SWE-bench coding-repair prompts (shielded) ──────────────────────

/// TRACE_MATRIX FC1a-rtool_input: System prompt for the SWE-bench repair loop.
/// Demands strict JSON so the judge's patch extractor can parse it reliably.
const SWEBENCH_SYSTEM_PROMPT: &str = "You are a software engineer. Output ONLY strict JSON {\"patch\":\"<unified git diff>\",\"rationale\":\"...\"}.";

/// TRACE_MATRIX FC1a-rtool_input: SHIELDED user prompt. Exposes only the public
/// issue fields + target failing-test NAMES. NEVER reads gold_patch/test_patch
/// (those fields are absent from `SwebenchSampleInput` by construction). On a
/// retry, the kernel/runner already appends the verifier's real failing-test
/// feedback — this builder produces only the base prompt.
fn make_swebench_user_prompt(sample: &SwebenchSampleInput) -> String {
    let hints = sample
        .hints_text
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|h| format!("\n\nMaintainer hints:\n{}", h))
        .unwrap_or_default();
    let failing = if sample.fail_to_pass.is_empty() {
        "(none listed)".to_string()
    } else {
        sample.fail_to_pass.join("\n")
    };
    format!(
        "Repository: {repo}\nBase commit: {base_commit}\n\nProblem statement:\n{problem}{hints}\n\nTarget failing tests that your patch must make pass:\n{failing}\n\nReturn a unified git diff patch (standard `git diff` format, file paths relative to the repository root, beginning with `diff --git`) that resolves the issue so the failing tests pass. Output ONLY the strict JSON object {{\"patch\":\"...\",\"rationale\":\"...\"}} with the diff as the `patch` value. Do not include or quote any hidden test code, reference solution, or benchmark patch.",
        repo = sample.repo,
        base_commit = sample.base_commit,
        problem = sample.problem_statement,
        hints = hints,
        failing = failing,
    )
}

/// TRACE_MATRIX FC2-N16: `turingos tdma` subcommand entry-point.
pub(crate) fn run(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("{FULL_HELP}");
        return ExitCode::from(2);
    }
    match args[0].as_str() {
        "run" => run_run(&args[1..]),
        "-h" | "--help" => {
            println!("{FULL_HELP}");
            ExitCode::SUCCESS
        }
        other => {
            eprintln!("turingos tdma: unknown action '{}'", other);
            eprintln!("{FULL_HELP}");
            ExitCode::from(2)
        }
    }
}

/// TRACE_MATRIX FC2-N16: `tdma run` action handler.
fn run_run(args: &[String]) -> ExitCode {
    let mut workspace: Option<PathBuf> = None;
    let mut judge_name = "nesbitt".to_string();
    let mut role = "meta".to_string();
    let mut evidence_dir: Option<PathBuf> = None;
    let mut max_attempts_per_stage: usize = 6;
    let mut temperature: f32 = 0.7;
    // SWE-bench judge inputs (only used when --judge swebench).
    let mut swebench_sample: Option<PathBuf> = None;
    let mut swebench_python: PathBuf = PathBuf::from("/Users/zephryj/.venv-swebench/bin/python");
    let mut swebench_dataset = "princeton-nlp/SWE-bench_Lite".to_string();
    let mut swebench_workdir: Option<PathBuf> = None;
    // Atom 25: Phase E full cutover. Default tape backend is now `git`
    // (GitTapeLedger at <workspace>/tdma_tape.git). `memory` remains
    // accepted via explicit --tape-backend=memory for tests + emergency
    // rollback (in-process scope only).
    let mut tape_backend = "git".to_string();

    let mut it = args.iter();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--workspace" => workspace = it.next().map(PathBuf::from),
            "--judge" => {
                if let Some(v) = it.next() {
                    judge_name = v.clone();
                }
            }
            "--role" => {
                if let Some(v) = it.next() {
                    role = v.clone();
                }
            }
            "--evidence-dir" => evidence_dir = it.next().map(PathBuf::from),
            "--max-attempts-per-stage" => {
                if let Some(v) = it.next() {
                    if let Ok(n) = v.parse() {
                        max_attempts_per_stage = n;
                    }
                }
            }
            "--temperature" => {
                if let Some(v) = it.next() {
                    if let Ok(f) = v.parse() {
                        temperature = f;
                    }
                }
            }
            "--tape-backend" => {
                if let Some(v) = it.next() {
                    tape_backend = v.clone();
                }
            }
            "--swebench-sample" => swebench_sample = it.next().map(PathBuf::from),
            "--swebench-python" => {
                if let Some(v) = it.next() {
                    swebench_python = PathBuf::from(v);
                }
            }
            "--swebench-dataset" => {
                if let Some(v) = it.next() {
                    swebench_dataset = v.clone();
                }
            }
            "--swebench-workdir" => swebench_workdir = it.next().map(PathBuf::from),
            "-h" | "--help" => {
                println!("{FULL_HELP}");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("turingos tdma run: unexpected flag '{}'", other);
                return ExitCode::from(2);
            }
        }
    }

    if tape_backend != "memory" && tape_backend != "git" {
        eprintln!(
            "turingos tdma run: --tape-backend must be 'memory' or 'git'; got '{}'",
            tape_backend
        );
        return ExitCode::from(2);
    }

    let workspace = match workspace {
        Some(w) => w,
        None => {
            eprintln!("turingos tdma run: --workspace required");
            return ExitCode::from(2);
        }
    };

    // Parse the SWE-bench sample early (validation) so config errors surface
    // before any model/api-key resolution. gold_patch/test_patch are NOT in
    // SwebenchSampleInput, so they can never reach the prompt.
    let swebench_sample_parsed: Option<SwebenchSampleInput> = if judge_name == "swebench" {
        let path = match &swebench_sample {
            Some(p) => p.clone(),
            None => {
                eprintln!(
                    "turingos tdma run: --judge swebench requires --swebench-sample <PATH>"
                );
                return ExitCode::from(2);
            }
        };
        let raw = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "turingos tdma run: cannot read --swebench-sample {}: {}",
                    path.display(),
                    e
                );
                return ExitCode::from(2);
            }
        };
        match serde_json::from_str::<SwebenchSampleInput>(&raw) {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!(
                    "turingos tdma run: --swebench-sample {} is not a valid SWE-bench sample JSON: {}",
                    path.display(),
                    e
                );
                return ExitCode::from(2);
            }
        }
    } else {
        match judge_name.as_str() {
            "nesbitt" | "putnam_a1" | "putnam_2025_b3" => {}
            other => {
                eprintln!(
                    "turingos tdma run: unknown --judge '{}'. Supported: nesbitt | putnam_a1 | putnam_2025_b3 | swebench",
                    other
                );
                return ExitCode::from(2);
            }
        }
        None
    };

    let (model, env_var_result, thinking) = match role.as_str() {
        "meta" => (
            cmd_llm::read_meta_model(&workspace),
            cmd_llm::read_meta_api_key_env(&workspace),
            cmd_llm::read_meta_thinking(&workspace),
        ),
        "blackbox" => (
            cmd_llm::read_blackbox_model(&workspace),
            cmd_llm::read_blackbox_api_key_env(&workspace),
            cmd_llm::read_blackbox_thinking(&workspace),
        ),
        _ => {
            eprintln!("turingos tdma run: --role must be 'meta' or 'blackbox'");
            return ExitCode::from(2);
        }
    };
    let env_var = match env_var_result {
        Ok(v) => v,
        Err(e) => {
            eprintln!("turingos tdma run: cannot resolve api-key env var: {:?}", e);
            return ExitCode::from(2);
        }
    };
    let api_key = match require_api_key(&env_var) {
        Ok(k) => k,
        Err(e) => {
            eprintln!("turingos tdma run: API key error: {:?}", e);
            return ExitCode::from(2);
        }
    };

    let evidence_dir: PathBuf = evidence_dir.unwrap_or_else(|| default_evidence_dir(&workspace));
    if let Err(e) = fs::create_dir_all(&evidence_dir) {
        eprintln!("turingos tdma run: cannot create evidence-dir: {}", e);
        return ExitCode::from(2);
    }

    eprintln!(
        "[turingos tdma run] workspace={} model={} role={} thinking={} judge={} evidence-dir={} max_attempts={} temp={}",
        workspace.display(),
        model,
        role,
        if thinking.is_some() { "on" } else { "off" },
        judge_name,
        evidence_dir.display(),
        max_attempts_per_stage,
        temperature
    );

    // Build the judge. For swebench, resolve the harness work dir (default: a
    // subdir of the evidence dir) and construct the real-harness verifier.
    let (mut judge, swebench_prompt_inputs) = match judge_name.as_str() {
        "nesbitt" => (AnyJudge::nesbitt(), None),
        "putnam_a1" => (AnyJudge::putnam_a1(), None),
        "putnam_2025_b3" => (AnyJudge::putnam_b3(), None),
        "swebench" => {
            let sample = swebench_sample_parsed
                .expect("swebench sample parsed above when judge_name == swebench");
            let workdir = swebench_workdir
                .clone()
                .unwrap_or_else(|| evidence_dir.join("swebench_work"));
            if let Err(e) = fs::create_dir_all(&workdir) {
                eprintln!(
                    "turingos tdma run: cannot create --swebench-workdir {}: {}",
                    workdir.display(),
                    e
                );
                return ExitCode::from(2);
            }
            eprintln!(
                "[turingos tdma run] swebench instance={} dataset={} python={} workdir={}",
                sample.instance_id,
                swebench_dataset,
                swebench_python.display(),
                workdir.display()
            );
            let judge = SwebenchTestJudge::new(
                sample.instance_id.clone(),
                swebench_dataset.clone(),
                swebench_python.clone(),
                workdir,
                "turingos-loop".to_string(),
            );
            (AnyJudge::swebench(judge), Some(sample))
        }
        // Unreachable: validated in swebench_sample_parsed match above.
        _ => unreachable!("judge_name validated above"),
    };

    let judge_name_for_sys = judge_name.clone();
    let judge_name_for_usr = judge_name.clone();
    let cfg = if let Some(sample) = swebench_prompt_inputs {
        // SHIELDED coding-repair prompts. The sample's problem_statement, repo,
        // base_commit, and fail_to_pass test NAMES are the ONLY fields exposed.
        // gold_patch/test_patch are not even present in SwebenchSampleInput.
        let sys = SWEBENCH_SYSTEM_PROMPT.to_string();
        let user = make_swebench_user_prompt(&sample);
        RunConfig {
            run_id: format!("turingos-tdma-{}", judge_name),
            model_label: model.clone(),
            problem_label: format!("turingos tdma --judge swebench {}", sample.instance_id),
            leak_sentinel: LEAK_SENTINEL.into(),
            system_prompt_for_stage: Box::new(move |_stage_label: &str| sys.clone()),
            user_prompt_for_stage: Box::new(move |_stage_label: &str, _accepted: &[String]| {
                user.clone()
            }),
            problem_text: String::new(),
            evidence_dir: evidence_dir.clone(),
            temperature,
            // Generous cap: with thinking="on" the reasoning_content counts toward
            // completion tokens, so a 4000 cap is fully consumed by reasoning and
            // leaves the patch (the `content`) empty/truncated. 16000 leaves ample
            // room for reasoning + the unified diff.
            max_tokens: 16000,
            max_attempts_per_stage,
        }
    } else {
        RunConfig {
            run_id: format!("turingos-tdma-{}", judge_name),
            model_label: model.clone(),
            problem_label: format!("turingos tdma --judge {}", judge_name),
            leak_sentinel: LEAK_SENTINEL.into(),
            system_prompt_for_stage: Box::new(move |stage_label: &str| {
                make_system_prompt(&judge_name_for_sys, stage_label)
            }),
            user_prompt_for_stage: Box::new(move |stage_label: &str, accepted: &[String]| {
                make_user_prompt(&judge_name_for_usr, stage_label, accepted)
            }),
            problem_text: String::new(),
            evidence_dir: evidence_dir.clone(),
            temperature,
            max_tokens: 600,
            max_attempts_per_stage,
        }
    };

    let model_for_llm = model.clone();
    let max_tokens_for_llm = cfg.max_tokens;
    let thinking_for_llm = thinking.clone();
    let llm_call = |sys: &str, user: &str| -> Result<LlmResponse, String> {
        let messages = vec![ChatMessage::system(sys), ChatMessage::user(user)];
        let resp = chat_complete_blocking(
            &api_key,
            &model_for_llm,
            &messages,
            Some(max_tokens_for_llm),
            Some(temperature),
            thinking_for_llm.clone(),
        )
        .map_err(|e: LlmError| format!("{:?}", e))?;
        Ok(LlmResponse {
            content: resp.content,
            completion_tokens: resp.usage.completion_tokens as u32,
            prompt_tokens: resp.usage.prompt_tokens as u32,
        })
    };

    // Atom 24: select tape backend at runtime.
    let run_result = if tape_backend == "git" {
        let repo_path = workspace.join("tdma_tape.git");
        let ledger = match turingosv4::git_tape_ledger::GitTapeLedger::open(&repo_path)
            .or_else(|_| turingosv4::git_tape_ledger::GitTapeLedger::init_bare(&repo_path))
        {
            Ok(l) => l,
            Err(e) => {
                eprintln!(
                    "turingos tdma run: cannot open/init git tape at {}: {}",
                    repo_path.display(),
                    e
                );
                return ExitCode::from(3);
            }
        };
        eprintln!(
            "[turingos tdma run] --tape-backend=git rooted at {}",
            repo_path.display()
        );
        turingosv4::tdma_runner::run_proof_with_ledger(cfg, &mut judge, ledger, llm_call)
    } else {
        run_proof(cfg, &mut judge, llm_call)
    };

    match run_result {
        Ok(summary) => {
            // Write a small human-readable report alongside the runner's manifest.
            let mut r = String::new();
            r.push_str("# turingos tdma run — TDMA-Bounded Production Report\n\n");
            r.push_str(&format!(
                "**Model**: {} (temperature {})\n\n",
                model, temperature
            ));
            r.push_str(&format!("**Role**: {}\n\n", role));
            r.push_str(&format!("**Judge**: {}\n\n", judge_name));
            r.push_str("## Outcome\n\n");
            r.push_str(&format!(
                "- Stages completed: **{}/{}**\n- Stages escalated/aborted: {:?}\n- Total attempts: **{}**\n- Wall clock: **{:.1}s**\n",
                summary.stages_completed,
                summary.stages_total,
                summary.stages_escalated,
                summary.probes.len(),
                summary.total_wall_clock_ms as f64 / 1000.0,
            ));
            r.push_str(&format!(
                "- Raw stderr leak in any prompt: **{}**\n\n",
                summary.leak_anywhere
            ));
            r.push_str("## Per stage\n\n| Stage | Attempts | Final BBS constraints | Outcome |\n|---|---|---|---|\n");
            for (s, a, c, o) in &summary.per_stage_attempts {
                r.push_str(&format!("| {} | {} | {} | {} |\n", s, a, c, o));
            }
            r.push_str(&format!(
                "\n## SiliconFlow tokens consumed\n\n- Prompt: {}\n- Completion: {}\n",
                summary.total_llm_prompt_tokens, summary.total_llm_completion_tokens
            ));
            let _ = fs::write(evidence_dir.join("ProductionTdmaReport.md"), r);

            println!(
                "turingos tdma run: completed {}/{} stages in {:.1}s. Evidence at {}",
                summary.stages_completed,
                summary.stages_total,
                summary.total_wall_clock_ms as f64 / 1000.0,
                evidence_dir.display()
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("turingos tdma run: run_proof failed: {}", e);
            ExitCode::from(3)
        }
    }
}

fn default_evidence_dir(workspace: &Path) -> PathBuf {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    workspace
        .join("artifacts")
        .join("tdma")
        .join(format!("tdma_run_{}", ts))
}
