//! TISR Phase 6.0/6.1 alpha — unified TuringOS user CLI entry point.
//!
//! Per §8 packet
//! `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`:
//! narrow CLI MVP scope. Implementation lives in `turingosv4::cli` (see
//! `src/cli/`). This file is the thin clap-based entry point only.
//!
//! 0 Class 4 surface modifications. typed_tx / sequencer / cas/schema 0-touch.

use std::process::ExitCode;

use clap::Parser;
use turingosv4::cli::{run, Cli};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("turingos: {err}");
            ExitCode::from(err.exit_code())
        }
    }
}
