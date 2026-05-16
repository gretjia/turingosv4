//! REAL-13D — report-side economic signal purification.
//!
//! This module classifies existing transaction/economic surfaces for reports.
//! It does not rename WorkTx stake or change any wire schema.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// TRACE_MATRIX FC3: report-side classification that separates forced bonds
/// from voluntary market positions for E2 claim boundaries.
pub enum EconomicSignalClass {
    CreatorBond,
    ChallengeBond,
    VerificationBond,
    VoluntaryMarketPosition,
    StructuralMarketTx,
    Other,
}

impl EconomicSignalClass {
    /// TRACE_MATRIX FC3: E2 can count only voluntary market positions, never
    /// WorkTx creator bonds or structural market transactions.
    pub const fn counts_for_e2(self) -> bool {
        matches!(self, EconomicSignalClass::VoluntaryMarketPosition)
    }
}

/// TRACE_MATRIX FC3: classifies existing transaction labels for dashboards
/// without changing wire schema or economic state.
pub fn classify_economic_signal(tx_kind: &str) -> EconomicSignalClass {
    match tx_kind {
        "work" | "work_tx" | "WorkTx" => EconomicSignalClass::CreatorBond,
        "challenge" | "challenge_tx" | "ChallengeTx" => EconomicSignalClass::ChallengeBond,
        "verify" | "verify_tx" | "VerifyTx" => EconomicSignalClass::VerificationBond,
        "buy_with_coin_router" | "BuyWithCoinRouterTx" => {
            EconomicSignalClass::VoluntaryMarketPosition
        }
        "market_seed" | "cpmm_pool" | "event_resolve" | "task_open" | "escrow_lock" => {
            EconomicSignalClass::StructuralMarketTx
        }
        _ => EconomicSignalClass::Other,
    }
}
