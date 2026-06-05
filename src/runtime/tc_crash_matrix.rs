use serde::{Deserialize, Serialize};

use crate::runtime::external_call::{ExternalCallCrashState, ExternalCallTerminal};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedTcState {
    pub git_head: String,
    pub cas_root: String,
    pub snapshot: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TcRestartReport {
    pub source: RestartSource,
    pub git_head: String,
    pub cas_root: String,
    pub used_ram_cache: bool,
    pub replay_requires_network: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RestartSource {
    GitCasOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrashMatrixCase {
    pub surface: String,
    pub kill_after_committed_transition: u64,
    pub snapshot_role: SnapshotRole,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SnapshotRole {
    AccelerationOnly,
    RequiredForCorrectness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrashMatrixError {
    MissingGitHead,
    MissingCasRoot,
    InvalidGitHead,
    InvalidCasRoot,
}

impl std::fmt::Display for CrashMatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingGitHead => write!(f, "restart requires persisted git head"),
            Self::MissingCasRoot => write!(f, "restart requires persisted CAS root"),
            Self::InvalidGitHead => write!(f, "persisted git head is not a git oid"),
            Self::InvalidCasRoot => write!(f, "persisted CAS root is not a git oid"),
        }
    }
}

impl std::error::Error for CrashMatrixError {}

pub fn restart_from_persisted_state(
    persisted: &PersistedTcState,
) -> Result<TcRestartReport, CrashMatrixError> {
    validate_oid_field(
        &persisted.git_head,
        CrashMatrixError::MissingGitHead,
        CrashMatrixError::InvalidGitHead,
    )?;
    validate_oid_field(
        &persisted.cas_root,
        CrashMatrixError::MissingCasRoot,
        CrashMatrixError::InvalidCasRoot,
    )?;
    Ok(TcRestartReport {
        source: RestartSource::GitCasOnly,
        git_head: persisted.git_head.clone(),
        cas_root: persisted.cas_root.clone(),
        used_ram_cache: false,
        replay_requires_network: false,
    })
}

pub fn recover_gateway_crash(state: ExternalCallCrashState) -> ExternalCallTerminal {
    ExternalCallTerminal::from_crash_state(state)
}

impl CrashMatrixCase {
    pub fn snapshot_optional_for_correctness(&self) -> bool {
        self.snapshot_role == SnapshotRole::AccelerationOnly
    }
}

fn validate_oid_field(
    value: &str,
    missing: CrashMatrixError,
    invalid: CrashMatrixError,
) -> Result<(), CrashMatrixError> {
    if value.trim().is_empty() {
        return Err(missing);
    }
    if value.len() != 40 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(invalid);
    }
    Ok(())
}
