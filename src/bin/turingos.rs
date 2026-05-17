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
//!   handover/alignment/OBS_R022_TISR_PHASE6_1_CLI_DISPATCH.md).
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
// MODULES-REGISTRY-BEGIN
// (each Wave 1-3 atom appends `#[path = ...] mod cmd_<name>;` lines here, before END anchor)
#[path = "turingos/cmd_report_run.rs"]
mod cmd_report_run;
#[path = "turingos/cmd_report_wallet.rs"]
mod cmd_report_wallet;
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
    // SUBCOMMANDS-REGISTRY-END
];

fn print_top_help() {
    println!("turingos — TuringOS user CLI (Phase 6.0/6.1)");
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

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().collect();
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
            eprintln!("turingos: unknown subcommand: {sub}");
            eprintln!("Run `turingos --help` for available subcommands.");
            ExitCode::from(2)
        }
    }
}
