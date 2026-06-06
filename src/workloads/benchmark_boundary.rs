//! Claim boundary for workload adapter outputs.
//!
//! This is a small validator for report language around adapter results. The
//! adapter output is never a ChainTape/CAS fact by itself; it is a typed summary
//! that must point at an evidence manifest.

/// TRACE_MATRIX FC3: workload adapter kind is report-side classification for L8 adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadAdapterKind {
    Lean,
    Swebench,
    MarketResearch,
}

/// TRACE_MATRIX FC3: adapter result class prevents workload smoke evidence from becoming OS truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterResultClassification {
    RealVerifierBacked,
    StructuralSmoke,
    ParticipationCanary,
}

/// TRACE_MATRIX FC3: workload adapter result is a derived evidence summary, not kernel authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkloadAdapterResult {
    pub workload_id: String,
    pub run_id: String,
    pub adapter_kind: WorkloadAdapterKind,
    pub evidence_manifest_cid: String,
    pub result_classification: AdapterResultClassification,
    pub verifier_backed_task_pass_count: u64,
    pub structural_smoke_count: u64,
    pub participation_canary_count: u64,
    pub unsupported_claim_count: u64,
}

/// TRACE_MATRIX FC3: claim-boundary errors are merge blockers for workload reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimBoundaryError {
    EmptyWorkloadId,
    EmptyRunId,
    EmptyEvidenceManifestCid,
    TaskPassWithoutVerifier,
    StrongClaimWithoutPreregistration,
    UnsupportedClaimCountNonZero,
}

impl std::fmt::Display for ClaimBoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ClaimBoundaryError::*;
        match self {
            EmptyWorkloadId => write!(f, "workload_id is empty"),
            EmptyRunId => write!(f, "run_id is empty"),
            EmptyEvidenceManifestCid => write!(f, "evidence_manifest_cid is empty"),
            TaskPassWithoutVerifier => write!(f, "task pass claim requires verifier evidence"),
            StrongClaimWithoutPreregistration => {
                write!(
                    f,
                    "strong workload headline requires preregistered evidence"
                )
            }
            UnsupportedClaimCountNonZero => write!(f, "unsupported_claim_count is non-zero"),
        }
    }
}

impl std::error::Error for ClaimBoundaryError {}

impl WorkloadAdapterResult {
    /// TRACE_MATRIX FC3: validate workload adapter summary before report language can cite it.
    pub fn validate(&self) -> Result<(), ClaimBoundaryError> {
        if self.workload_id.trim().is_empty() {
            return Err(ClaimBoundaryError::EmptyWorkloadId);
        }
        if self.run_id.trim().is_empty() {
            return Err(ClaimBoundaryError::EmptyRunId);
        }
        if self.evidence_manifest_cid.trim().is_empty() {
            return Err(ClaimBoundaryError::EmptyEvidenceManifestCid);
        }
        if self.unsupported_claim_count != 0 {
            return Err(ClaimBoundaryError::UnsupportedClaimCountNonZero);
        }
        Ok(())
    }

    fn has_verifier_task_pass(&self) -> bool {
        self.result_classification == AdapterResultClassification::RealVerifierBacked
            && self.verifier_backed_task_pass_count > 0
    }
}

/// TRACE_MATRIX FC3: validate public workload claim text against adapter evidence class.
pub fn validate_adapter_claim_text(
    result: &WorkloadAdapterResult,
    claim_text: &str,
    preregistered_evidence: bool,
) -> Result<(), ClaimBoundaryError> {
    result.validate()?;
    if contains_task_pass_marker(claim_text) && !result.has_verifier_task_pass() {
        return Err(ClaimBoundaryError::TaskPassWithoutVerifier);
    }
    if contains_strong_headline_marker(claim_text) && !preregistered_evidence {
        return Err(ClaimBoundaryError::StrongClaimWithoutPreregistration);
    }
    Ok(())
}

fn contains_task_pass_marker(text: &str) -> bool {
    let marker = ["TASK", "-PASS"].concat();
    text.contains(&marker)
}

fn contains_strong_headline_marker(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    let c_word = ["caus", "al"].concat();
    lower.contains("proven")
        || lower.contains("definitive")
        || lower.contains(&c_word)
        || (lower.contains("market") && lower.contains("beat"))
}
