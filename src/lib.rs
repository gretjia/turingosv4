pub mod boot;
pub mod ledger;
pub mod kernel;
pub mod bus;
pub mod sdk;
pub mod drivers;
pub mod wal;
pub mod economy;
pub mod top_white;
pub mod bottom_white;
pub mod state;
/// TRACE_MATRIX FC3-N1: production-path ChainTape runtime — connects evaluator binary to Sequencer + Git2LedgerWriter so LLM-driven runs produce on-disk LedgerEntry chain (TB-6).
pub mod runtime;
