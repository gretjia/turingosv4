//! TRACE_MATRIX FC2-N16: turingos user CLI entry (Phase 6.0/6.1).
//!
//! Phase 6.0 ship: `turingos init` only.
//! Phase 6.1 ship: ~17 subcommands via append-only registry (see SUBCOMMANDS array).
//!
//! Constraints:
//! - Single binary (this file as entry; submodules under `src/bin/turingos/`).
//! - No clap (manual `std::env::args` parsing; preserves Trust Root: no
//!   Cargo.toml touch).
//! - All `cmd_*.rs` items `pub(crate)` + `/// TRACE_MATRIX FC2-N16:`
//!   doc-comments (R-022 reverse-map; see
//!   handover/directives/2026-05-17_TISR_PHASE6_R022_CLI_DISPATCH_OBS.md).
//! - §8 packet 2026-05-17 (TISR Phase 6.0/6.1 separate charter).

use std::env;
use std::process::ExitCode;

// Submodule path attributes: Rust's default module-file resolver for
// `src/bin/X.rs` searches `src/bin/`. The Phase 6.1 atomic layout puts
// submodules under `src/bin/turingos/` — point each `mod` declaration
// there with `#[path = ...]`.
#[path = "turingos/cmd_init.rs"]
mod cmd_init;
#[path = "turingos/common.rs"]
mod common;
#[path = "turingos/siliconflow_client.rs"]
mod siliconflow_client;
// A6 (2026-05-19): `spec_capsule` was library-ized to
// `src/runtime/spec_capsule.rs`; bin callers now use
// `turingosv4::runtime::spec_capsule`. The previous `#[path = "turingos/spec_capsule.rs"] mod spec_capsule;`
// declaration was removed (file no longer exists at that path).
// MODULES-REGISTRY-BEGIN
// (each Wave 1-3 atom appends `#[path = ...] mod cmd_<name>;` lines here, before END anchor)
#[path = "turingos/cmd_agent.rs"]
mod cmd_agent;
#[path = "turingos/cmd_audit_dashboard.rs"]
mod cmd_audit_dashboard;
#[path = "turingos/cmd_audit_tamper.rs"]
mod cmd_audit_tamper;
#[path = "turingos/cmd_audit_tape.rs"]
mod cmd_audit_tape;
#[path = "turingos/cmd_batch.rs"]
mod cmd_batch;
#[path = "turingos/cmd_config.rs"]
mod cmd_config;
#[path = "turingos/cmd_export_evidence.rs"]
mod cmd_export_evidence;
#[path = "turingos/cmd_generate.rs"]
mod cmd_generate;
#[path = "turingos/cmd_llm.rs"]
mod cmd_llm;
#[path = "turingos/cmd_preflight.rs"]
mod cmd_preflight;
#[path = "turingos/cmd_render.rs"]
mod cmd_render;
#[path = "turingos/cmd_replay.rs"]
mod cmd_replay;
#[path = "turingos/cmd_report_bankruptcy.rs"]
mod cmd_report_bankruptcy;
#[path = "turingos/cmd_report_markov.rs"]
mod cmd_report_markov;
#[path = "turingos/cmd_report_positions.rs"]
mod cmd_report_positions;
#[path = "turingos/cmd_report_run.rs"]
mod cmd_report_run;
#[path = "turingos/cmd_report_wallet.rs"]
mod cmd_report_wallet;
#[path = "turingos/cmd_spec.rs"]
mod cmd_spec;
#[path = "turingos/cmd_task_open.rs"]
mod cmd_task_open;
#[path = "turingos/cmd_task_tick.rs"]
mod cmd_task_tick;
#[path = "turingos/cmd_task_view.rs"]
mod cmd_task_view;
#[path = "turingos/cmd_verify_chaintape.rs"]
mod cmd_verify_chaintape;
#[path = "turingos/cmd_verify_e2_candidate.rs"]
mod cmd_verify_e2_candidate;
#[path = "turingos/cmd_welcome.rs"]
mod cmd_welcome;
#[path = "turingos/cmd_spec_audit.rs"]
mod cmd_spec_audit;
#[path = "turingos/cmd_wizard.rs"]
mod cmd_wizard;
#[path = "turingos/cmd_tdma.rs"]
mod cmd_tdma;
#[path = "turingos/cmd_tape_migrate.rs"]
mod cmd_tape_migrate;
// MODULES-REGISTRY-END

const VERSION_STR: &str = concat!("turingos ", env!("CARGO_PKG_VERSION"));

/// TRACE_MATRIX FC2-N16: CLI dispatch table entry type
pub(crate) struct Subcommand {
    pub(crate) name: &'static str,
    pub(crate) short_help: &'static str,
    pub(crate) run: fn(&[String]) -> ExitCode,
}

const SUBCOMMANDS: &[Subcommand] = &[
    // SUBCOMMANDS-REGISTRY-BEGIN
    Subcommand {
        name: "init",
        short_help: cmd_init::SHORT_HELP,
        run: cmd_init::run,
    },
    Subcommand {
        name: "report run",
        short_help: cmd_report_run::SHORT_HELP,
        run: cmd_report_run::run,
    },
    Subcommand {
        name: "report wallet",
        short_help: cmd_report_wallet::SHORT_HELP,
        run: cmd_report_wallet::run,
    },
    Subcommand {
        name: "report positions",
        short_help: cmd_report_positions::SHORT_HELP,
        run: cmd_report_positions::run,
    },
    Subcommand {
        name: "report bankruptcy",
        short_help: cmd_report_bankruptcy::SHORT_HELP,
        run: cmd_report_bankruptcy::run,
    },
    Subcommand {
        name: "report markov",
        short_help: cmd_report_markov::SHORT_HELP,
        run: cmd_report_markov::run,
    },
    Subcommand {
        name: "verify chaintape",
        short_help: cmd_verify_chaintape::SHORT_HELP,
        run: cmd_verify_chaintape::run,
    },
    Subcommand {
        name: "verify e2-candidate",
        short_help: cmd_verify_e2_candidate::SHORT_HELP,
        run: cmd_verify_e2_candidate::run,
    },
    Subcommand {
        name: "audit dashboard",
        short_help: cmd_audit_dashboard::SHORT_HELP,
        run: cmd_audit_dashboard::run,
    },
    Subcommand {
        name: "audit tape",
        short_help: cmd_audit_tape::SHORT_HELP,
        run: cmd_audit_tape::run,
    },
    Subcommand {
        name: "audit tamper",
        short_help: cmd_audit_tamper::SHORT_HELP,
        run: cmd_audit_tamper::run,
    },
    Subcommand {
        name: "preflight",
        short_help: cmd_preflight::SHORT_HELP,
        run: cmd_preflight::run,
    },
    Subcommand {
        name: "replay",
        short_help: cmd_replay::SHORT_HELP,
        run: cmd_replay::run,
    },
    Subcommand {
        name: "task open",
        short_help: cmd_task_open::SHORT_HELP,
        run: cmd_task_open::run,
    },
    Subcommand {
        name: "task view",
        short_help: cmd_task_view::SHORT_HELP,
        run: cmd_task_view::run,
    },
    Subcommand {
        name: "task tick",
        short_help: cmd_task_tick::SHORT_HELP,
        run: cmd_task_tick::run,
    },
    Subcommand {
        name: "config",
        short_help: cmd_config::SHORT_HELP,
        run: cmd_config::run,
    },
    Subcommand {
        name: "agent",
        short_help: cmd_agent::SHORT_HELP,
        run: cmd_agent::run,
    },
    Subcommand {
        name: "batch",
        short_help: cmd_batch::SHORT_HELP,
        run: cmd_batch::run,
    },
    Subcommand {
        name: "export evidence",
        short_help: cmd_export_evidence::SHORT_HELP,
        run: cmd_export_evidence::run,
    },
    Subcommand {
        name: "render",
        short_help: cmd_render::SHORT_HELP,
        run: cmd_render::run,
    },
    Subcommand {
        name: "welcome",
        short_help: cmd_welcome::SHORT_HELP,
        run: cmd_welcome::run,
    },
    Subcommand {
        name: "llm",
        short_help: cmd_llm::SHORT_HELP,
        run: cmd_llm::run,
    },
    Subcommand {
        name: "spec",
        short_help: cmd_spec::SHORT_HELP,
        run: cmd_spec::run,
    },
    Subcommand {
        name: "generate",
        short_help: cmd_generate::SHORT_HELP,
        run: cmd_generate::run,
    },
    Subcommand {
        name: "spec audit",
        short_help: cmd_spec_audit::SHORT_HELP,
        run: cmd_spec_audit::run,
    },
    Subcommand {
        name: "wizard",
        short_help: cmd_wizard::SHORT_HELP,
        run: cmd_wizard::run,
    },
    Subcommand {
        name: "tdma",
        short_help: cmd_tdma::SHORT_HELP,
        run: cmd_tdma::run,
    },
    Subcommand {
        name: "tape-migrate",
        short_help: cmd_tape_migrate::SHORT_HELP,
        run: cmd_tape_migrate::run,
    },
    // SUBCOMMANDS-REGISTRY-END
];

fn print_top_help() {
    println!("turingos — TuringOS user CLI (Phase 6.3 demo)");
    println!();
    println!("USAGE:");
    println!("    turingos <SUBCOMMAND> [OPTIONS]");
    println!();
    println!("SUBCOMMANDS:");
    for sc in SUBCOMMANDS {
        println!("    {:24} {}", sc.name, sc.short_help);
    }
    println!();
    println!("    help, -h, --help   Print this help");
    println!("    -V, --version      Print version");
    println!();
    println!("Run `turingos <SUBCOMMAND> --help` for subcommand-specific help.");
}

fn dispatch(sub: &str, rest: &[String]) -> Option<ExitCode> {
    for sc in SUBCOMMANDS {
        if sc.name == sub {
            return Some((sc.run)(rest));
        }
    }
    None
}

/// If `prefix` is the first token of one or more two-token Subcommand entries
/// (e.g. "report" is a prefix of "report wallet" / "report positions" / ...),
/// print the subgroup's children. Returns true if the prefix matched any
/// children. Called BEFORE the "unknown subcommand" fallthrough so that
/// `turingos report --help` lists report's children instead of erroring.
fn print_subgroup_help(prefix: &str) -> bool {
    let needle = format!("{prefix} ");
    let children: Vec<&Subcommand> = SUBCOMMANDS
        .iter()
        .filter(|sc| sc.name.starts_with(&needle))
        .collect();
    if children.is_empty() {
        return false;
    }
    println!("turingos {prefix} — subcommand group");
    println!();
    println!("USAGE:");
    println!("    turingos {prefix} <SUBCOMMAND> [OPTIONS]");
    println!();
    println!("SUBCOMMANDS:");
    for sc in &children {
        // strip the "<prefix> " from sc.name for display
        let child_name = &sc.name[needle.len()..];
        println!("    {child_name:24} {}", sc.short_help);
    }
    println!();
    println!("Run `turingos {prefix} <SUBCOMMAND> --help` for subcommand-specific help.");
    true
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().collect();

    // Bare `turingos` with no arguments: launch TUI wizard when TTY, else help.
    // This is the non-programmer onboarding entry (Atom-W, FC2-N16).
    if argv.len() == 1 {
        use std::io::IsTerminal;
        if std::io::stdin().is_terminal() && std::io::stdout().is_terminal() {
            return cmd_wizard::run(&[]);
        } else {
            print_top_help();
            return ExitCode::SUCCESS;
        }
    }

    let sub = argv.get(1).map(String::as_str).unwrap_or("--help");
    match sub {
        "-V" | "--version" => {
            println!("{VERSION_STR}");
            ExitCode::SUCCESS
        }
        "-h" | "--help" | "help" => {
            print_top_help();
            ExitCode::SUCCESS
        }
        _ => {
            // 2-pass dispatch: try multi-token (e.g., "report run") first, then
            // single-token. Longest-match-first to avoid prefix collisions.
            if argv.len() >= 3 {
                let combined = format!("{} {}", argv[1], argv[2]);
                let rest: Vec<String> = argv.iter().skip(3).cloned().collect();
                if let Some(code) = dispatch(&combined, &rest) {
                    return code;
                }
            }
            let rest: Vec<String> = argv.iter().skip(2).cloned().collect();
            if let Some(code) = dispatch(sub, &rest) {
                return code;
            }
            // Subgroup help: `turingos report --help` / `turingos task --help`
            // etc. list the children of "report" / "task" so users can discover
            // the two-token names without scanning the top-level help.
            let asked_help = rest.iter().any(|a| a == "-h" || a == "--help") || rest.is_empty();
            if asked_help && print_subgroup_help(sub) {
                return ExitCode::SUCCESS;
            }
            eprintln!("turingos: unknown subcommand: {sub}");
            eprintln!("Run `turingos --help` for available subcommands.");
            ExitCode::from(2)
        }
    }
}
