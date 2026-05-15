// Tier 2: Isolated process sandbox — external verifier with timeout + SIGKILL
// Constitutional basis: Art. I.1 (boolean predicates, deterministic verification)
// V3L-01: sorry 3-layer defense. V3L-02: oracle determinism. V3L-07: identity theft.

use std::fmt;
use std::time::Duration;

// ── Core types ──────────────────────────────────────────────────

/// Result of sandbox execution.
#[derive(Debug, Clone)]
pub enum SandboxResult {
    /// Execution completed within timeout.
    Completed {
        stdout: String,
        stderr: String,
        exit_code: i32,
    },
    /// Execution exceeded timeout — killed.
    Timeout,
}

/// V3L-09: explicit error, never silent.
#[derive(Debug, Clone)]
pub enum SandboxError {
    SpawnFailed(String),
    IoError(String),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SandboxError::SpawnFailed(msg) => write!(f, "Sandbox spawn failed: {}", msg),
            SandboxError::IoError(msg) => write!(f, "Sandbox I/O error: {}", msg),
        }
    }
}

impl std::error::Error for SandboxError {}

/// Trait for sandboxed execution engines.
/// V3L-02: implementations must be deterministic (same input → same output).
pub trait SandboxEngine: Send + Sync {
    fn engine_name(&self) -> &str;

    /// Execute code in isolation with timeout.
    /// Returns SandboxResult (never panics, never hangs).
    fn execute(&self, code: &str, timeout: Duration) -> Result<SandboxResult, SandboxError>;
}

/// Local process sandbox — spawns an ephemeral child process.
/// Uses SIGKILL on timeout (no negotiation).
pub struct LocalProcessSandbox {
    pub command: String,
    pub args: Vec<String>,
}

impl LocalProcessSandbox {
    pub fn new(command: &str, args: &[&str]) -> Self {
        LocalProcessSandbox {
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl SandboxEngine for LocalProcessSandbox {
    fn engine_name(&self) -> &str {
        "local_process"
    }

    fn execute(&self, code: &str, timeout: Duration) -> Result<SandboxResult, SandboxError> {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let mut child = Command::new(&self.command)
            .args(&self.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| SandboxError::SpawnFailed(e.to_string()))?;

        // Write code to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(code.as_bytes())
                .map_err(|e| SandboxError::IoError(e.to_string()))?;
        }

        // Wait with timeout
        match child.wait_timeout(timeout) {
            Ok(Some(status)) => {
                let stdout = child
                    .stdout
                    .take()
                    .map(|mut r| {
                        let mut s = String::new();
                        std::io::Read::read_to_string(&mut r, &mut s).unwrap_or(0);
                        s
                    })
                    .unwrap_or_default();
                let stderr = child
                    .stderr
                    .take()
                    .map(|mut r| {
                        let mut s = String::new();
                        std::io::Read::read_to_string(&mut r, &mut s).unwrap_or(0);
                        s
                    })
                    .unwrap_or_default();

                Ok(SandboxResult::Completed {
                    stdout,
                    stderr,
                    exit_code: status.code().unwrap_or(-1),
                })
            }
            Ok(None) => {
                // Timeout — SIGKILL (no negotiation)
                let _ = child.kill();
                let _ = child.wait();
                Ok(SandboxResult::Timeout)
            }
            Err(e) => Err(SandboxError::IoError(e.to_string())),
        }
    }
}

// Note: wait_timeout is not in std — we need to use a polling approach
// or add the wait-timeout crate. For now, this uses a simplified sync approach.
// The async version (tokio::process) will be used in actor.rs.

/// Trait extension for std::process::Child — simplified timeout.
trait WaitTimeout {
    fn wait_timeout(
        &mut self,
        timeout: Duration,
    ) -> std::io::Result<Option<std::process::ExitStatus>>;
}

impl WaitTimeout for std::process::Child {
    fn wait_timeout(
        &mut self,
        timeout: Duration,
    ) -> std::io::Result<Option<std::process::ExitStatus>> {
        let start = std::time::Instant::now();
        loop {
            match self.try_wait()? {
                Some(status) => return Ok(Some(status)),
                None => {
                    if start.elapsed() >= timeout {
                        return Ok(None);
                    }
                    std::thread::sleep(Duration::from_millis(50));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_echo_command() {
        let sandbox = LocalProcessSandbox::new("echo", &["hello"]);
        let result = sandbox.execute("", Duration::from_secs(5)).unwrap();
        match result {
            SandboxResult::Completed {
                stdout, exit_code, ..
            } => {
                assert!(stdout.contains("hello"));
                assert_eq!(exit_code, 0);
            }
            SandboxResult::Timeout => panic!("Should not timeout"),
        }
    }

    #[test]
    fn test_sandbox_timeout_kills_process() {
        // sleep 10 should be killed in 1 second
        let sandbox = LocalProcessSandbox::new("sleep", &["10"]);
        let result = sandbox.execute("", Duration::from_secs(1)).unwrap();
        assert!(matches!(result, SandboxResult::Timeout));
    }

    #[test]
    fn test_sandbox_captures_stderr() {
        let sandbox = LocalProcessSandbox::new("bash", &["-c", "echo err >&2"]);
        let result = sandbox.execute("", Duration::from_secs(5)).unwrap();
        match result {
            SandboxResult::Completed { stderr, .. } => {
                assert!(stderr.contains("err"));
            }
            _ => panic!("Should complete"),
        }
    }

    #[test]
    fn test_sandbox_nonzero_exit() {
        let sandbox = LocalProcessSandbox::new("bash", &["-c", "exit 42"]);
        let result = sandbox.execute("", Duration::from_secs(5)).unwrap();
        match result {
            SandboxResult::Completed { exit_code, .. } => {
                assert_eq!(exit_code, 42);
            }
            _ => panic!("Should complete"),
        }
    }
}
