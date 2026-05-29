//! Smoke test for `.claude/hooks/judge.sh` — proves the 2026-05-29
//! false-positive fix while showing the destructive guards still fire.
//!
//! Root cause fixed: the destructive-command grep patterns matched the
//! command-name token (`sed`/`awk`/`perl`/`tee`/`rm`) as a bare *substring*,
//! so they tripped inside ordinary words — "u-sed", "par-sed", "perfo-rm",
//! "transfo-rm". A benign read like `grep -n "parsed" src/kernel.rs` or
//! `rg -n "transform" run.jsonl` was therefore blocked. The fix left-anchors
//! each token with `(^|[^[:alnum:]_])` so it only matches a real command.
//!
//! Why this lives in a Rust integration test and not a shell smoke script:
//! the destructive fixture strings (`rm -rf`, `git push --force`,
//! `sed … constitution.md`) are baked into compiled source here and reach
//! `judge.sh` over a spawned-process stdin pipe. They are NEVER part of a
//! Bash *tool* invocation, so the production PreToolUse `judge.sh` hook that
//! wraps Bash tool calls cannot intercept them. `cargo test --test
//! judge_hook_smoke` is itself a benign command. (The hook blocking its own
//! smoke fixtures is exactly the false positive under test.)

#![cfg(unix)]

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Locate `.claude/hooks/judge.sh` by walking up from the crate manifest dir.
fn judge_sh() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        let candidate = dir.join(".claude/hooks/judge.sh");
        if candidate.is_file() {
            return candidate;
        }
        if !dir.pop() {
            panic!("could not locate .claude/hooks/judge.sh above CARGO_MANIFEST_DIR");
        }
    }
}

/// Minimal JSON string escaping (no serde_json dev-dep needed for fixtures).
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Feed one hook payload to judge.sh on stdin; return (exit_code, stdout+stderr).
fn run_hook(payload: &str) -> (i32, String) {
    let mut child = Command::new("bash")
        .arg(judge_sh())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn `bash judge.sh`");
    {
        // Bind + drop stdin so judge.sh's `cat` sees EOF before we wait.
        let mut stdin = child.stdin.take().expect("child stdin");
        stdin.write_all(payload.as_bytes()).expect("write payload");
    }
    let out = child.wait_with_output().expect("wait judge.sh");
    let mut log = String::new();
    log.push_str(&String::from_utf8_lossy(&out.stdout));
    log.push_str(&String::from_utf8_lossy(&out.stderr));
    (out.status.code().unwrap_or(-1), log)
}

/// Run judge.sh as if Claude Code called the Bash tool with `command`.
fn run_bash(command: &str) -> (i32, String) {
    let payload = format!(
        r#"{{"tool_name":"Bash","tool_input":{{"command":"{}"}}}}"#,
        json_escape(command)
    );
    run_hook(&payload)
}

/// Run judge.sh as if Claude Code called the Edit tool on `file_path`.
fn run_edit(file_path: &str, new_string: &str) -> (i32, String) {
    let payload = format!(
        r#"{{"tool_name":"Edit","tool_input":{{"file_path":"{}","new_string":"{}"}}}}"#,
        json_escape(file_path),
        json_escape(new_string)
    );
    run_hook(&payload)
}

// Benign, read-only commands whose *text* contains a command-name substring.
// Each one was blocked by the pre-fix patterns; each must now pass (exit 0).
const BENIGN_READS: &[&str] = &[
    "grep -n \"parsed\" src/kernel.rs",  // 'sed' in 'parsed' + kernel.rs  (old L122)
    "rg -n \"used\" constitution.md",    // 'sed' in 'used' + constitution (old L69)
    "rg -n \"transform\" logs/run.jsonl", // 'rm' in 'transform' + .jsonl  (old L117)
    "rg -n \"perform -rf\" /etc/hosts",  // 'rm -rf ' in 'perform -rf' + / (old L100+L101)
    "for f in src/*.rs; do echo $f; done", // original for-loop symptom
];

// Genuinely destructive commands / constitution mutations. Each must STILL
// block (exit 2). This is the "narrowed, not fail-open" guarantee.
const DESTRUCTIVE_BLOCKS: &[&str] = &[
    "rm -rf /home/zephryj/projects/turingosv4-harness-upgrade/src", // L100+L101
    "rm -rf ~/.claude",                  // L100+L101 (~/ and .claude)
    "rm build/run.jsonl",                // L117 WAL/ledger deletion
    "sed -i 's/x/y/' src/kernel.rs",     // L122 kernel constant mutation
    "sed -i 's/a/b/' constitution.md",   // L69 constitution mutation
    "git push --force origin main",      // L107 force push
    "git reset --hard HEAD~1",           // L112 hard reset
];

#[test]
fn benign_reads_pass_after_false_positive_fix() {
    for cmd in BENIGN_READS {
        let (code, log) = run_bash(cmd);
        assert_eq!(
            code, 0,
            "benign read-only command must pass (exit 0); judge.sh returned {code}\n  cmd: {cmd:?}\n  output:\n{log}"
        );
    }
}

#[test]
fn destructive_commands_still_block() {
    for cmd in DESTRUCTIVE_BLOCKS {
        let (code, log) = run_bash(cmd);
        assert_eq!(
            code, 2,
            "destructive command must still block (exit 2); judge.sh returned {code}\n  cmd: {cmd:?}\n  output:\n{log}"
        );
    }
}

#[test]
fn edit_targeting_constitution_still_blocks() {
    // R-018 sudo guard: any Edit whose basename is constitution.md blocks,
    // regardless of directory (basename + realpath match in judge.sh).
    let (code, log) = run_edit("/tmp/anywhere/constitution.md", "tampered");
    assert_eq!(
        code, 2,
        "Edit targeting constitution.md must block (exit 2); judge.sh returned {code}\n  output:\n{log}"
    );
}
