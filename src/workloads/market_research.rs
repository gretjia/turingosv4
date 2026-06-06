//! Market research preregistration contract.
//!
//! This validator gates workload research reports. It does not authorize real
//! provider spending, settlement, wallet movement, or kernel writes.

/// TRACE_MATRIX FC3: market research track label for preregistered workload experiments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketTrack {
    A,
    B,
    C,
    D,
}

/// TRACE_MATRIX FC3: market preregistration is report-side guardrail, not runtime authority.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketPreregistration {
    pub track: MarketTrack,
    pub hypothesis: String,
    pub mde: String,
    pub sample_size: u64,
    pub budget_equalization: String,
    pub ablations: Vec<String>,
    pub hidden_verifier_shielding: String,
    pub route_decision_tape_policy: String,
    pub replay_command: String,
    pub headline_claim_allowed: bool,
    pub clean_context_audit_required: bool,
}

/// TRACE_MATRIX FC3: market preregistration errors are ship-blocks before workload headlines.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketPreregistrationError {
    EmptyHypothesis,
    EmptyMde,
    ZeroSampleSize,
    EmptyBudgetEqualization,
    MissingAblations,
    MissingHiddenVerifierShielding,
    RouteDecisionNotTapeVisible,
    EmptyReplayCommand,
    HeadlineWithoutAudit,
}

impl std::fmt::Display for MarketPreregistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use MarketPreregistrationError::*;
        match self {
            EmptyHypothesis => write!(f, "hypothesis is empty"),
            EmptyMde => write!(f, "mde is empty"),
            ZeroSampleSize => write!(f, "sample_size == 0"),
            EmptyBudgetEqualization => write!(f, "budget_equalization is empty"),
            MissingAblations => write!(f, "ablations are missing"),
            MissingHiddenVerifierShielding => write!(f, "hidden verifier shielding is missing"),
            RouteDecisionNotTapeVisible => write!(f, "route decision policy is not tape-visible"),
            EmptyReplayCommand => write!(f, "replay_command is empty"),
            HeadlineWithoutAudit => write!(f, "headline claim requires clean-context audit"),
        }
    }
}

impl std::error::Error for MarketPreregistrationError {}

impl MarketPreregistration {
    /// TRACE_MATRIX FC3: validate preregistration before market workload reports can make headlines.
    pub fn validate(&self) -> Result<(), MarketPreregistrationError> {
        if self.hypothesis.trim().is_empty() {
            return Err(MarketPreregistrationError::EmptyHypothesis);
        }
        if self.mde.trim().is_empty() {
            return Err(MarketPreregistrationError::EmptyMde);
        }
        if self.sample_size == 0 {
            return Err(MarketPreregistrationError::ZeroSampleSize);
        }
        if self.budget_equalization.trim().is_empty() {
            return Err(MarketPreregistrationError::EmptyBudgetEqualization);
        }
        if self.ablations.is_empty() || self.ablations.iter().any(|s| s.trim().is_empty()) {
            return Err(MarketPreregistrationError::MissingAblations);
        }
        let shielding = self.hidden_verifier_shielding.to_ascii_lowercase();
        let shielded = shielding.contains("shield")
            || shielding.contains("outside")
            || shielding.contains("redact");
        if !(shielding.contains("hidden") && shielded) {
            return Err(MarketPreregistrationError::MissingHiddenVerifierShielding);
        }
        let route_policy = self.route_decision_tape_policy.to_ascii_lowercase();
        if !(route_policy.contains("tape") || route_policy.contains("schedulerdecision")) {
            return Err(MarketPreregistrationError::RouteDecisionNotTapeVisible);
        }
        if self.replay_command.trim().is_empty() {
            return Err(MarketPreregistrationError::EmptyReplayCommand);
        }
        if self.headline_claim_allowed && !self.clean_context_audit_required {
            return Err(MarketPreregistrationError::HeadlineWithoutAudit);
        }
        Ok(())
    }
}
