//! TB-16 Atom 3 — `audit_tape` CLI (architect §7.5 + design §6).
//!
//! Pure audit-from-tape binary. Reads ONLY on-disk artifacts:
//!   - runtime_repo/ (Git2 L4 chain + L4.E rejections.jsonl)
//!   - cas/ (CAS store)
//!   - agent_pubkeys.json + pinned_pubkeys.json (per-run manifests)
//!   - genesis_payload.toml + constitution.md
//!   - [optional] --markov-pointer <path>  per-run Markov capsule cid hex
//!   - [optional] --prior-chain-runtime-repo <path>  inherit Markov cid
//!     from a prior chain's per-runtime tip pointer (NOT a global pointer)
//!   - [optional] handover/alignment/ (OBS scan)
//!
//! TB-16.x.fix (2026-05-04; architect OBS_R022 ruling Option α):
//! `--markov-pointer` is no longer required. Absence → genesis chain
//! (constitutional per architect Q2.b — `previous_capsule_cid: None` on
//! fresh isolated chain; Layer G assertions Skipped). Present-but-FS-
//! absent or present-but-CAS-unresolvable → fail-closed BLOCK (architect
//! Q2.c last paragraph; TB-16.x.1 semantic preserved).
//!
//! NEVER reads:
//!   - live Sequencer state
//!   - state.db (whitebox cache; auditor rebuilds via replay_full_transition)
//!   - process logs
//!   - handover/ai-direct/
//!   - handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
//!     (de-canonicalized; was Art. 0.2 parallel ledger — see OBS_R022)
//!
//! Emits verdict.json per design §6.3 schema (38 assertions × 8 layers,
//! tape_root, tx_kind_counts, feature_coverage, verdict ∈ {PROCEED, BLOCK}).
//!
//! Usage:
//!   audit_tape \
//!     --runtime-repo  <path> \
//!     --cas-dir       <path> \
//!     --agent-pubkeys <path> \
//!     --pinned-pubkeys <path> \
//!     --genesis       <path> \
//!     --constitution  <path> \
//!     [--markov-pointer <path>] \
//!     [--prior-chain-runtime-repo <path>] \
//!     [--alignment-dir <path>] \
//!     --out <verdict.json>
//!
//! Exit code:
//!   0  — verdict.json verdict == "PROCEED"
//!   1  — verdict.json verdict == "BLOCK" (≥1 fail/halt)
//!   2  — invalid args / I/O failure before audit could begin
//!
//! TRACE_MATRIX FC1-N34 (audit_tape binary) + FC2-N31 (verdict.json schema v1).

use std::path::PathBuf;
use std::process::ExitCode;

use turingosv4::runtime::audit_assertions::{
    run_all_assertions, summarize_results, AuditInputs,
};

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
    out: PathBuf,
}

fn parse_args(argv: &[String]) -> Result<Args, String> {
    let mut runtime_repo: Option<PathBuf> = None;
    let mut cas_dir: Option<PathBuf> = None;
    let mut agent_pubkeys: Option<PathBuf> = None;
    let mut pinned_pubkeys: Option<PathBuf> = None;
    let mut genesis: Option<PathBuf> = None;
    let mut constitution: Option<PathBuf> = None;
    let mut markov_pointer: Option<PathBuf> = None;
    let mut prior_chain_runtime_repo: Option<PathBuf> = None;
    let mut alignment_dir: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--runtime-repo" => {
                i += 1;
                runtime_repo = Some(argv.get(i).ok_or("--runtime-repo needs path")?.into());
            }
            "--cas-dir" => {
                i += 1;
                cas_dir = Some(argv.get(i).ok_or("--cas-dir needs path")?.into());
            }
            "--agent-pubkeys" => {
                i += 1;
                agent_pubkeys = Some(argv.get(i).ok_or("--agent-pubkeys needs path")?.into());
            }
            "--pinned-pubkeys" => {
                i += 1;
                pinned_pubkeys = Some(argv.get(i).ok_or("--pinned-pubkeys needs path")?.into());
            }
            "--genesis" => {
                i += 1;
                genesis = Some(argv.get(i).ok_or("--genesis needs path")?.into());
            }
            "--constitution" => {
                i += 1;
                constitution = Some(argv.get(i).ok_or("--constitution needs path")?.into());
            }
            "--markov-pointer" => {
                i += 1;
                markov_pointer = Some(argv.get(i).ok_or("--markov-pointer needs path")?.into());
            }
            "--prior-chain-runtime-repo" => {
                i += 1;
                prior_chain_runtime_repo = Some(
                    argv.get(i)
                        .ok_or("--prior-chain-runtime-repo needs path")?
                        .into(),
                );
            }
            "--alignment-dir" => {
                i += 1;
                alignment_dir = Some(argv.get(i).ok_or("--alignment-dir needs path")?.into());
            }
            "--out" => {
                i += 1;
                out = Some(argv.get(i).ok_or("--out needs path")?.into());
            }
            "-h" | "--help" => {
                eprint!("{}", help_text());
                std::process::exit(0);
            }
            other => return Err(format!("unknown arg: {other}")),
        }
        i += 1;
    }
    if markov_pointer.is_some() && prior_chain_runtime_repo.is_some() {
        return Err(
            "--markov-pointer and --prior-chain-runtime-repo are mutually exclusive \
             (pass one or the other; absence of both = genesis chain)"
                .into(),
        );
    }
    Ok(Args {
        runtime_repo: runtime_repo.ok_or("--runtime-repo required")?,
        cas_dir: cas_dir.ok_or("--cas-dir required")?,
        agent_pubkeys: agent_pubkeys.ok_or("--agent-pubkeys required")?,
        pinned_pubkeys: pinned_pubkeys.ok_or("--pinned-pubkeys required")?,
        genesis: genesis.ok_or("--genesis required")?,
        constitution: constitution.ok_or("--constitution required")?,
        markov_pointer,
        prior_chain_runtime_repo,
        alignment_dir,
        out: out.ok_or("--out required")?,
    })
}

fn help_text() -> String {
    "audit_tape — TB-16 Atom 3 audit-from-tape binary\n\
     \n\
     USAGE:\n  \
       audit_tape --runtime-repo <p> --cas-dir <p> --agent-pubkeys <p>\n  \
                  --pinned-pubkeys <p> --genesis <p> --constitution <p>\n  \
                  [--markov-pointer <p>] [--prior-chain-runtime-repo <p>]\n  \
                  [--alignment-dir <p>] --out <verdict.json>\n\
     \n\
     MARKOV INHERITANCE (TB-16.x.fix; architect OBS_R022 Option α):\n  \
       absent both flags        → genesis chain; Layer G Skipped\n  \
       --markov-pointer <p>     → per-run pointer file (NOT global);\n  \
                                  must exist + cid must resolve in --cas-dir\n  \
       --prior-chain-runtime-repo <p>\n  \
                                → resolves Markov tip from prior chain's\n  \
                                  per-runtime pointer at <p>/markov_tip.cid\n  \
                                  (in-tape; no global parallel ledger)\n\
     \n\
     EXIT:\n  \
       0  verdict == PROCEED (38/38 assertions GREEN)\n  \
       1  verdict == BLOCK (≥1 fail/halt)\n  \
       2  invalid args / I/O failure\n"
        .into()
}

/// TB-16.x.fix (architect OBS_R022 Option α) resolver:
/// `--prior-chain-runtime-repo <path>` reads `<path>/markov_tip.cid`
/// (a per-runtime-repo pointer file, NOT global) and returns its
/// path so it can be passed downstream as `markov_pointer`. The
/// pointer file is OPTIONAL inside the prior runtime_repo: when
/// absent, the prior chain has no Markov tip → returns `None`
/// (genesis-equivalent inheritance).
///
/// This is the minimum-viable α resolver. The full Art. 0.4 path B
/// in-tape resolver (walk the chain to find the most recent Markov
/// emit; resolve its cid from the supplied `--cas-dir`) lands with
/// TB-16.x.2.4 / TB-16.x.2.6 (architect ruling §A.6 β).
fn resolve_prior_chain_markov(prior: &std::path::Path) -> Result<Option<PathBuf>, String> {
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
        // Genesis-equivalent: prior chain ran but never emitted a
        // Markov capsule. Caller proceeds as if no Markov inheritance.
        Ok(None)
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("audit_tape: {e}\n\n{}", help_text());
            return ExitCode::from(2);
        }
    };

    let resolved_pointer = if let Some(p) = args.markov_pointer.clone() {
        Some(p)
    } else if let Some(prior) = args.prior_chain_runtime_repo.as_ref() {
        match resolve_prior_chain_markov(prior) {
            Ok(opt) => opt,
            Err(e) => {
                eprintln!("audit_tape: {e}");
                return ExitCode::from(2);
            }
        }
    } else {
        None
    };

    let inputs = AuditInputs {
        runtime_repo: args.runtime_repo,
        cas_dir: args.cas_dir,
        agent_pubkeys: args.agent_pubkeys,
        pinned_pubkeys: args.pinned_pubkeys,
        genesis: args.genesis,
        constitution: args.constitution,
        markov_pointer: resolved_pointer,
        alignment_dir: args.alignment_dir,
    };

    let results = match run_all_assertions(&inputs) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("audit_tape: load failed: {e}");
            return ExitCode::from(2);
        }
    };

    let verdict = match summarize_results(&inputs, results) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("audit_tape: summarize failed: {e}");
            return ExitCode::from(2);
        }
    };

    let json = match serde_json::to_string_pretty(&verdict) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("audit_tape: json serialize failed: {e}");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = std::fs::write(&args.out, json) {
        eprintln!("audit_tape: write {:?} failed: {e}", args.out);
        return ExitCode::from(2);
    }

    let proceed = verdict.verdict == "PROCEED";
    println!(
        "audit_tape: verdict={} passed={} failed={} halted={} skipped={} (out={:?})",
        verdict.verdict, verdict.passed, verdict.failed, verdict.halted, verdict.skipped, args.out
    );
    if proceed {
        ExitCode::from(0)
    } else {
        ExitCode::from(1)
    }
}
