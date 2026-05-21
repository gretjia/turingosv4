use std::collections::BTreeMap;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
/// TRACE_MATRIX FC1-N7 + FC1-N12: evidence truthfulness marker for the
/// process-hygiene runner; phase 0 records that network isolation is not
/// physically enforced.
pub enum NetworkPolicyClaim {
    NotEnforced,
}

#[derive(Debug, Clone)]
/// TRACE_MATRIX FC1-N7 + FC1-N12: explicit process boundary input shape for
/// production shell-outs crossing from agent/tool code into child processes.
pub struct SanitizedCommand {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, String>,
    pub stdin: Option<Vec<u8>>,
    pub timeout: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// TRACE_MATRIX FC1-N12 + Art. 0.2: reconstructable child-process evidence
/// captured by the runner without relying on stdout-only self-report.
pub struct SanitizedOutput {
    pub argv: Vec<String>,
    pub cwd: PathBuf,
    pub allowed_env_keys: Vec<String>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub network_policy_claim: NetworkPolicyClaim,
}

impl SanitizedOutput {
    /// TRACE_MATRIX FC1-N12: deterministic local predicate over captured
    /// runner evidence; no dashboard or LLM judgment involved.
    pub fn success(&self) -> bool {
        !self.timed_out && self.exit_code == Some(0)
    }
}

/// TRACE_MATRIX FC1-N7: copies only named host environment keys into the
/// runner allowlist so child processes do not inherit ambient secrets.
pub fn env_allowlist_from_current(keys: &[&str]) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    for key in keys {
        if let Ok(value) = std::env::var(key) {
            env.insert((*key).to_string(), value);
        }
    }
    env
}

/// TRACE_MATRIX FC1-N7 + FC1-N12: executes one sanitized child process with
/// explicit cwd, env allowlist, stdio capture, timeout, and honest
/// `NotEnforced` network-policy evidence.
pub fn run_sanitized(command: SanitizedCommand) -> io::Result<SanitizedOutput> {
    let argv = command_argv(&command);
    let allowed_env_keys = command.env.keys().cloned().collect();

    let mut child = Command::new(&command.program)
        .args(&command.args)
        .current_dir(&command.cwd)
        .env_clear()
        .envs(&command.env)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdout_reader = thread::spawn(move || read_child_pipe(stdout));
    let stderr_reader = thread::spawn(move || read_child_pipe(stderr));

    if let Some(input) = command.stdin {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(&input)?;
        }
    } else {
        drop(child.stdin.take());
    }

    let started = Instant::now();
    let (exit_code, timed_out) = loop {
        if let Some(status) = child.try_wait()? {
            break (status.code(), false);
        }
        if started.elapsed() >= command.timeout {
            child.kill()?;
            let _ = child.wait()?;
            break (None, true);
        }
        std::thread::sleep(Duration::from_millis(10));
    };

    let stdout = join_reader(stdout_reader)?;
    let stderr = join_reader(stderr_reader)?;

    Ok(SanitizedOutput {
        argv,
        cwd: command.cwd,
        allowed_env_keys,
        stdout,
        stderr,
        exit_code,
        timed_out,
        network_policy_claim: NetworkPolicyClaim::NotEnforced,
    })
}

fn command_argv(command: &SanitizedCommand) -> Vec<String> {
    let mut argv = Vec::with_capacity(1 + command.args.len());
    argv.push(command.program.to_string_lossy().into_owned());
    argv.extend(command.args.iter().cloned());
    argv
}

fn read_child_pipe<R: Read>(pipe: Option<R>) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(mut pipe) = pipe {
        pipe.read_to_end(&mut bytes)?;
    }
    Ok(bytes)
}

fn join_reader(handle: thread::JoinHandle<io::Result<Vec<u8>>>) -> io::Result<Vec<u8>> {
    handle
        .join()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "stdio reader thread panicked"))?
}
