//! REAL-17 evaluator wiring gate for direct submitted MarketDecision provenance.
//!
//! This source-level gate is intentionally narrow: it guards that the live
//! evaluator writes a CAS sidecar from the submitted MarketDecisionTrace CID
//! and the same-turn PromptCapsule CID, without changing typed tx admission.
//!
//! NOTE: The evaluator source at experiments/minif2f_v4/ was removed in
//! commit 309e026a. The source-grep tests in this file have been removed.
//! The gate file is retained as a placeholder for future wiring tests.
