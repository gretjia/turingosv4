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
/// TRACE_MATRIX FC1a-rtool_input + FC3-replay: TDMA-Bounded-RC1 distiller (Atom 4).
pub mod distiller;
/// TRACE_MATRIX FC2-Q_0 + FC3-constitution_binding: TDMA-Bounded-RC1 CharterCore (Atom 5).
pub mod charter_core;
/// TRACE_MATRIX FC1a-rtool: TDMA-Bounded-RC1 rtool checkout_digest (Atom 6).
pub mod rtool;
/// TRACE_MATRIX FC1a-judge_pi: TDMA-Bounded-RC1 JudgeAI predicates (Atom 7.5).
pub mod judges;
/// TRACE_MATRIX FC1a-rtool + FC1a-judge_pi + FC3-replay:
/// TDMA-Bounded shared runner library (Atom 18 — K10+K11 refactor).
pub mod tdma_runner;
/// TRACE_MATRIX FC1a-substrate_seam + FC3-replay:
/// Phase E libgit2 substrate skeleton (Atom 20; bodies in Atoms 21/22).
pub mod git_tape_ledger;
pub mod top_white;
pub mod wal;
