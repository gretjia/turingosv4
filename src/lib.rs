pub mod boot;
pub mod bottom_white;
pub mod bus;
/// TRACE_MATRIX FC2-N16: TISR Phase 6.0/6.1 alpha — unified turingos CLI module.
/// Per §8 packet `handover/directives/2026-05-17_TISR_PHASE6_SEPARATE_CHARTER_SECTION8_PACKET.md`:
/// narrow CLI MVP + local generative UI IR spike scope. 0 Class 4 surface modifications.
pub mod cli;
pub mod drivers;
pub mod economy;
pub mod kernel;
pub mod ledger;
/// TRACE_MATRIX FC3-N1: production-path ChainTape runtime — connects evaluator binary to Sequencer + Git2LedgerWriter so LLM-driven runs produce on-disk LedgerEntry chain (TB-6).
pub mod runtime;
pub mod sdk;
pub mod state;
pub mod top_white;
pub mod wal;
