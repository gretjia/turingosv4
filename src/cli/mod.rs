//! TISR Phase 6.0/6.1 alpha — unified turingos CLI.
//!
//! Per §8 packet
//! `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`:
//! narrow CLI MVP + local generative UI IR spike scope.
//!
//! 0 Class 4 surface modifications. typed_tx / sequencer / cas/schema 0-touch.
//! All implementation lives in `src/bin/turingos.rs` + `src/cli/**`, per
//! PACKET §4 allowed paths.

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — clap argument types.
pub mod args;
/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos CLI subcommand handlers.
pub mod commands;
/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos CLI error abstraction.
pub mod error;
/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — genesis payload templates for `turingos init`.
pub mod templates;

pub use args::{Cli, Commands};
pub use error::{CliResult, TuringosCliError};

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos CLI subcommand dispatcher.
/// Dispatch a parsed [`Cli`] to the appropriate subcommand handler.
/// Per §8 packet `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md` §2.
pub fn run(cli: Cli) -> CliResult<()> {
    match cli.command {
        Commands::Init(args) => commands::init::run(args),
    }
}
