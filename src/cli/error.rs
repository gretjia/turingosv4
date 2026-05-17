//! Error abstraction for the turingos CLI.
//!
//! Exit code convention per UNIFIED_CLI_SPEC §7:
//!   0 — success
//!   1 — verification / precondition fail (e.g., project already exists without --force)
//!   2 — argument or IO error

use std::io;

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos CLI error abstraction.
/// Errors raised by turingos CLI subcommands.
#[derive(Debug)]
pub enum TuringosCliError {
    /// Filesystem or io error from std::io operations.
    Io(io::Error),
    /// User attempted to initialize a project where the directory already exists
    /// and `--force` was not provided.
    ProjectExists(String),
    /// Invalid CLI argument or precondition.
    InvalidArgument(String),
}

impl std::fmt::Display for TuringosCliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::ProjectExists(path) => write!(
                f,
                "project directory already exists: {path} (use --force to overwrite scaffold files)"
            ),
            Self::InvalidArgument(msg) => write!(f, "invalid argument: {msg}"),
        }
    }
}

impl std::error::Error for TuringosCliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for TuringosCliError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl TuringosCliError {
    /// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — process exit code mapping per UNIFIED_CLI_SPEC §7.
    /// Returns the process exit code for this error variant.
    pub fn exit_code(&self) -> u8 {
        match self {
            Self::ProjectExists(_) => 1,
            Self::Io(_) | Self::InvalidArgument(_) => 2,
        }
    }
}

/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — turingos CLI subcommand result alias.
/// Convenience result alias for CLI subcommand handlers.
pub type CliResult<T> = Result<T, TuringosCliError>;
