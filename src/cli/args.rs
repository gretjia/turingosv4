//! clap argument structures for the unified turingos CLI.
//!
//! TISR Phase 6.0/6.1 alpha — initial subcommand: `init` only.
//! Phase 6.1+ will add `batch`, `agent`, `task`, `audit`, `verify`, `report`,
//! `preflight`, `replay`, `export`, `watch`, `market trigger` per the
//! UNIFIED_CLI_SPEC 37-subcommand tree.

use clap::{Args, Parser, Subcommand, ValueEnum};

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — TuringOS unified user CLI parser root.
///
/// TuringOS unified user CLI (Phase 6.0/6.1 alpha narrow MVP scope).
///
/// Phase 6 implementation source-of-truth:
/// `handover/research/interaction_substrate/50_deliverables/00_UNIFIED_CLI_SPEC.md`.
/// Per §8 packet `2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md` §2.
#[derive(Parser, Debug)]
#[command(
    name = "turingos",
    version,
    about = "TuringOS unified user CLI (Phase 6.0/6.1 alpha)",
    long_about = None,
)]
/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — TuringOS unified user CLI parser root (struct).
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — top-level subcommand discriminant.
/// All top-level turingos subcommands.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new TuringOS workspace directory.
    Init(InitArgs),
}

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — `turingos init` arguments.
/// Arguments for `turingos init`.
#[derive(Args, Debug)]
pub struct InitArgs {
    /// Project path (created if missing); becomes the workspace root.
    #[arg(long)]
    pub project: String,

    /// Genesis spec template to scaffold.
    #[arg(long, value_enum, default_value_t = Template::Proof)]
    pub template: Template,

    /// Overwrite existing files inside the project directory if it already exists.
    ///
    /// Without this flag, init aborts when the project directory exists. Note
    /// that even with `--force`, init only writes scaffold files; it never
    /// deletes user content already present.
    #[arg(long)]
    pub force: bool,
}

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — `turingos init` template selector.
/// Spec template selector for `turingos init --template`.
#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum Template {
    /// Lean proof market template (REAL-5/REAL-12 baseline).
    Proof,
    /// Polymarket event-resolution template.
    Polymarket,
    /// Multi-agent arena template (BullTrader / BearTrader / Librarian roles).
    MultiAgent,
}
