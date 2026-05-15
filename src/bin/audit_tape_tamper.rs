//! TB-16 Atom 3 — `audit_tape_tamper` CLI (architect §7.7 + design §6.2 H).
//!
//! Tamper-detection harness. Forks the input tape into 3 temp copies,
//! introduces a single corruption per copy, then re-runs `audit_tape`
//! over each:
//!
//!   1. flip 1 byte in a random L4 row (via Git2 ledger commit blob)
//!      → verdict.json must emit `BLOCK` with a Layer B fail/halt.
//!   2. flip 1 byte in a random CAS object → verdict.json must emit
//!      `BLOCK` with a Layer B fail/halt.
//!   3. remove a random L4 row by truncating the Git2 ref to N-1
//!      → verdict.json must emit `BLOCK` (replay state-root mismatch).
//!
//! Each corruption is applied to a TEMP COPY of the tape; the original
//! is untouched. Emits `tamper_report.json` summarizing the 3 attempts.
//!
//! TB-16.x.fix (2026-05-04; architect OBS_R022 ruling Option α):
//! `--markov-pointer` is now optional (mirrors `audit_tape`); absence →
//! genesis (Layer G Skipped on tampered + untampered copies).
//! `--prior-chain-runtime-repo` added for explicit per-runtime
//! inheritance (NOT a global pointer).
//!
//! Usage:
//!   audit_tape_tamper \
//!     --runtime-repo  <path> \
//!     --cas-dir       <path> \
//!     --agent-pubkeys <path> \
//!     --pinned-pubkeys <path> \
//!     --genesis       <path> \
//!     --constitution  <path> \
//!     [--markov-pointer <path>] \
//!     [--prior-chain-runtime-repo <path>] \
//!     [--alignment-dir <path>] \
//!     --tamper-dir    <work-dir> \
//!     --out           <tamper_report.json>
//!
//! Exit code:
//!   0  — all 3 corruptions detected (each verdict was BLOCK)
//!   1  — at least one corruption NOT detected (HALT TRIGGER per architect §7.7)
//!   2  — invalid args / I/O failure
//!
//! TRACE_MATRIX FC1-N35 (audit_tape_tamper binary; design §6.2 #36-#38).
//!
//! TRACE_MATRIX TB-16 Atom 7 R1 (Codex Q7/V5 closure 2026-05-04):
//! the destructive zlib-decode-failure tamper primitives originally lived in
//! this binary as `flip_byte_in_first_blob` + `flip_byte_in_first_cas_object`.
//!
//! TRACE_MATRIX TB-16 Atom 7 R1 (Codex Q7/V5 closure 2026-05-04):
//! moved to library `turingosv4::runtime::audit_tamper` 2026-05-10 session
//! #33 after M0 batch surfaced multi-ref drift (orphan-blob silent
//! corruption + alias-only ref truncation; M0 P01 evidence showed 1/3 vs
//! architect §B.9.3 mandated 3/3). Library `pub fn` carry the same anchor
//! (FC1-N35 + FC2-INV1 + architect §B.9.3 prove-no-fake-accepted). See
//! `feedback_no_workarounds_strict_constitution`.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use turingosv4::runtime::audit_assertions::{
    run_all_assertions, summarize_results, AuditInputs, TapeAuditVerdict,
};
use turingosv4::runtime::audit_tamper::{
    corrupt_chain_refs, flip_largest_cas_object, flip_largest_reachable_l4_blob,
};

#[derive(Debug, Clone)]
struct Args {
    runtime_repo: PathBuf,
    cas_dir: PathBuf,
    agent_pubkeys: PathBuf,
    pinned_pubkeys: PathBuf,
    genesis: PathBuf,
    constitution: PathBuf,
    markov_pointer: Option<PathBuf>,
    prior_chain_runtime_repo: Option<PathBuf>,
    alignment_dir: Option<PathBuf>,
    tamper_dir: PathBuf,
    out: PathBuf,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut p: std::collections::BTreeMap<&str, PathBuf> = Default::default();
    let mut i = 0;
    let keys = [
        "--runtime-repo",
        "--cas-dir",
        "--agent-pubkeys",
        "--pinned-pubkeys",
        "--genesis",
        "--constitution",
        "--markov-pointer",
        "--prior-chain-runtime-repo",
        "--alignment-dir",
        "--tamper-dir",
        "--out",
    ];
    while i < argv.len() {
        let k = argv[i].as_str();
        if k == "-h" || k == "--help" {
            eprint!("{}", help_text());
            std::process::exit(0);
        }
        if !keys.contains(&k) {
            return Err(format!("unknown arg: {k}"));
        }
        i += 1;
        let v = argv.get(i).ok_or_else(|| format!("{k} needs path"))?;
        let static_k: &'static str = match k {
            "--runtime-repo" => "--runtime-repo",
            "--cas-dir" => "--cas-dir",
            "--agent-pubkeys" => "--agent-pubkeys",
            "--pinned-pubkeys" => "--pinned-pubkeys",
            "--genesis" => "--genesis",
            "--constitution" => "--constitution",
            "--markov-pointer" => "--markov-pointer",
            "--prior-chain-runtime-repo" => "--prior-chain-runtime-repo",
            "--alignment-dir" => "--alignment-dir",
            "--tamper-dir" => "--tamper-dir",
            "--out" => "--out",
            _ => unreachable!(),
        };
        p.insert(static_k, PathBuf::from(v));
        i += 1;
    }
    let mut take = |k: &str| p.remove(k).ok_or_else(|| format!("{k} required"));
    let runtime_repo = take("--runtime-repo")?;
    let cas_dir = take("--cas-dir")?;
    let agent_pubkeys = take("--agent-pubkeys")?;
    let pinned_pubkeys = take("--pinned-pubkeys")?;
    let genesis = take("--genesis")?;
    let constitution = take("--constitution")?;
    let tamper_dir = take("--tamper-dir")?;
    let out = take("--out")?;
    let markov_pointer = p.remove("--markov-pointer");
    let prior_chain_runtime_repo = p.remove("--prior-chain-runtime-repo");
    if markov_pointer.is_some() && prior_chain_runtime_repo.is_some() {
        return Err(
            "--markov-pointer and --prior-chain-runtime-repo are mutually exclusive".into(),
        );
    }
    let alignment_dir = p.remove("--alignment-dir");
    Ok(Args {
        runtime_repo,
        cas_dir,
        agent_pubkeys,
        pinned_pubkeys,
        genesis,
        constitution,
        markov_pointer,
        prior_chain_runtime_repo,
        alignment_dir,
        tamper_dir,
        out,
    })
}

/// TB-16.x.fix mirror of `audit_tape::resolve_prior_chain_markov`.
fn resolve_prior_chain_markov(prior: &Path) -> Result<Option<PathBuf>, String> {
    if !prior.is_dir() {
        return Err(format!(
            "--prior-chain-runtime-repo {:?} is not a directory",
            prior
        ));
    }
    let tip = prior.join("markov_tip.cid");
    if tip.exists() {
        Ok(Some(tip))
    } else {
        Ok(None)
    }
}

fn help_text() -> String {
    "audit_tape_tamper — TB-16 Atom 3 tamper-detection harness\n\
     \n\
     USAGE:\n  \
       audit_tape_tamper --runtime-repo <p> --cas-dir <p> --agent-pubkeys <p>\n  \
                         --pinned-pubkeys <p> --genesis <p> --constitution <p>\n  \
                         [--markov-pointer <p>] [--prior-chain-runtime-repo <p>]\n  \
                         [--alignment-dir <p>] --tamper-dir <p> --out <p>\n\
     \n\
     MARKOV INHERITANCE (TB-16.x.fix; architect OBS_R022 Option α):\n  \
       both flags absent        → genesis chain; Layer G Skipped\n  \
       --markov-pointer <p>     → per-run pointer file (NOT global)\n  \
       --prior-chain-runtime-repo <p>\n  \
                                → resolves <p>/markov_tip.cid (in-tape)\n\
     \n\
     EXIT:\n  \
       0  all 3 corruptions detected (BLOCK on each tampered copy)\n  \
       1  at least 1 corruption NOT detected (HALT per architect §7.7)\n  \
       2  invalid args / I/O failure\n"
        .into()
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else if file_type.is_symlink() {
            // Follow symlinks: copy underlying file content.
            if let Ok(meta) = std::fs::metadata(&from) {
                if meta.is_file() {
                    std::fs::copy(&from, &to)?;
                }
            }
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn fork_tape(args: &Args, label: &str) -> Result<(PathBuf, PathBuf), String> {
    let dir = args.tamper_dir.join(label);
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(|e| format!("clear {dir:?}: {e}"))?;
    }
    let runtime_dst = dir.join("runtime_repo");
    let cas_dst = dir.join("cas");
    copy_dir_recursive(&args.runtime_repo, &runtime_dst)
        .map_err(|e| format!("copy runtime_repo: {e}"))?;
    copy_dir_recursive(&args.cas_dir, &cas_dst).map_err(|e| format!("copy cas_dir: {e}"))?;
    Ok((runtime_dst, cas_dst))
}

fn resolve_markov_input(args: &Args) -> Result<Option<PathBuf>, String> {
    if let Some(p) = args.markov_pointer.clone() {
        return Ok(Some(p));
    }
    if let Some(prior) = args.prior_chain_runtime_repo.as_ref() {
        return resolve_prior_chain_markov(prior);
    }
    Ok(None)
}

fn run_audit(args: &Args, runtime: &Path, cas: &Path) -> Result<TapeAuditVerdict, String> {
    let markov_pointer = resolve_markov_input(args)?;
    let inputs = AuditInputs {
        runtime_repo: runtime.to_path_buf(),
        cas_dir: cas.to_path_buf(),
        agent_pubkeys: args.agent_pubkeys.clone(),
        pinned_pubkeys: args.pinned_pubkeys.clone(),
        genesis: args.genesis.clone(),
        constitution: args.constitution.clone(),
        markov_pointer,
        alignment_dir: args.alignment_dir.clone(),
    };
    let results = run_all_assertions(&inputs).map_err(|e| format!("run: {e}"))?;
    summarize_results(&inputs, results).map_err(|e| format!("summarize: {e}"))
}

// Tamper apply primitives moved to library `turingosv4::runtime::audit_tamper`
// (M0 batch 2026-05-10 surfaced TB-16-era multi-ref drift; backlinks at
// module-doc head). Library imports above; binary now thin orchestrator.

#[derive(serde::Serialize)]
struct TamperReport {
    schema_version: String,
    label: String,
    detected: bool,
    detail: String,
    verdict: Option<TapeAuditVerdict>,
}

fn run_tamper(
    label: &str,
    args: &Args,
    apply: impl FnOnce(&Path, &Path) -> Result<String, String>,
) -> TamperReport {
    let (runtime, cas) = match fork_tape(args, label) {
        Ok(p) => p,
        Err(e) => {
            return TamperReport {
                schema_version: "v1/audit_tape_tamper".into(),
                label: label.into(),
                detected: false,
                detail: format!("fork failed: {e}"),
                verdict: None,
            };
        }
    };

    // TRACE_MATRIX TB-16 Atom 7 R1 (Codex Q7/V5 VETO closure):
    // pre-tamper baseline check. The forked tape must verify
    // PROCEED on the UNTAMPERED copy first; otherwise any post-
    // tamper BLOCK could be the SAME pre-existing halt (e.g.
    // upstream evidence gap), giving false-positive detection.
    let pre_tamper = match run_audit(args, &runtime, &cas) {
        Ok(v) => v,
        Err(e) => {
            return TamperReport {
                schema_version: "v1/audit_tape_tamper".into(),
                label: label.into(),
                detected: false,
                detail: format!(
                    "pre-tamper baseline audit failed (cannot validate \
                     fence efficacy on a tape that's already broken): {e}"
                ),
                verdict: None,
            };
        }
    };
    if pre_tamper.verdict != "PROCEED" {
        return TamperReport {
            schema_version: "v1/audit_tape_tamper".into(),
            label: label.into(),
            detected: false,
            detail: format!(
                "pre-tamper baseline verdict={} (not PROCEED); cannot \
                 prove tamper-fence efficacy on a tape that already \
                 BLOCKs. Use a tape with a clean PROCEED baseline.",
                pre_tamper.verdict
            ),
            verdict: Some(pre_tamper),
        };
    }

    let detail = match apply(&runtime, &cas) {
        Ok(d) => d,
        Err(e) => {
            return TamperReport {
                schema_version: "v1/audit_tape_tamper".into(),
                label: label.into(),
                detected: false,
                detail: format!("apply failed: {e}"),
                verdict: None,
            };
        }
    };
    let verdict_res = run_audit(args, &runtime, &cas);
    let (detected, verdict) = match verdict_res {
        // TRACE_MATRIX TB-16 Atom 7 R1 (V5 closure): true detection ONLY when
        // pre-tamper was PROCEED AND post-tamper is BLOCK. The pre-tamper
        // check above guarantees pre==PROCEED at this point.
        Ok(v) => (v.verdict == "BLOCK", Some(v)),
        Err(e) => (true, {
            // Audit refused to load the tape at all post-tamper; that
            // itself counts as detection (the binary can't proceed past
            // corruption — pre-tamper succeeded so this is a tamper-
            // induced load failure, not a pre-existing defect).
            eprintln!("audit_tape_tamper: load itself failed for `{label}` post-tamper → counted as detected ({e})");
            None
        }),
    };
    TamperReport {
        schema_version: "v1/audit_tape_tamper".into(),
        label: label.into(),
        detected,
        detail,
        verdict,
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("audit_tape_tamper: {e}\n\n{}", help_text());
            return ExitCode::from(2);
        }
    };
    if let Err(e) = std::fs::create_dir_all(&args.tamper_dir) {
        eprintln!("audit_tape_tamper: mkdir tamper-dir: {e}");
        return ExitCode::from(2);
    }

    let r1 = run_tamper("flip_l4_byte", &args, |runtime, _cas| {
        flip_largest_reachable_l4_blob(runtime)
    });
    let r2 = run_tamper("flip_cas_byte", &args, |_runtime, cas| {
        flip_largest_cas_object(cas)
    });
    let r3 = run_tamper("truncate_l4_ref", &args, |runtime, _cas| {
        corrupt_chain_refs(runtime)
    });
    let detected = [r1.detected, r2.detected, r3.detected];
    let total_detected = detected.iter().filter(|x| **x).count();

    let report = serde_json::json!({
        "schema_version": "v1/audit_tape_tamper",
        "tamper_results": [r1, r2, r3],
        "detected_count": total_detected,
        "expected": 3,
        "all_detected": total_detected == 3,
    });
    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
    if let Err(e) = std::fs::write(&args.out, json) {
        eprintln!("audit_tape_tamper: write {:?} failed: {e}", args.out);
        return ExitCode::from(2);
    }

    println!(
        "audit_tape_tamper: detected {}/3 (out={:?})",
        total_detected, args.out
    );
    if total_detected == 3 {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    }
}
