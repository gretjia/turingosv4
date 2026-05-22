pub mod boot;
pub mod bottom_white;
pub mod bus;
pub mod drivers;
pub mod economy;
pub mod kernel;
pub mod ledger;
/// TRACE_MATRIX FC1a-Q_t + FC1b-Q_{t+1}: TDMA-Bounded-RC1 memory kernel scaffold (Atom 2).
/// Full body lands in Atom 7. Per directive §5.1 routing entry-point and Proceed
/// branch are implemented now; handle_rejection is a deliberate `unimplemented!`
/// stub until Atom 7.
pub mod memory_kernel;
/// TRACE_MATRIX FC3-N1: production-path ChainTape runtime — connects evaluator binary to Sequencer + Git2LedgerWriter so LLM-driven runs produce on-disk LedgerEntry chain (TB-6).
pub mod runtime;
pub mod sdk;
pub mod state;
/// TRACE_MATRIX FC1a-output_edge: TDMA-Bounded-RC1 state-first prefix parser (Atom 2).
pub mod state_update;
/// TRACE_MATRIX FC1a-budget_gate: TDMA-Bounded-RC1 tokenizer (Atom 3).
pub mod tokenizer;
/// TRACE_MATRIX FC1a-budget_gate: TDMA-Bounded-RC1 token budget subsystem (Atom 3).
pub mod token_budget;
pub mod top_white;
pub mod wal;
