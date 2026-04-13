# TuringOS v4 Architecture

## Core OS (`src/`)

| Module | Role |
|--------|------|
| `kernel.rs` | Sacred microkernel — pure topology + zero-profit treasury + Oracle settlement. **Zero domain knowledge.** |
| `prediction_market.rs` | BinaryMarket CPMM (YES/NO constant product + LP tracking) |
| `bus.rs` | TSP Event Bus — SKILL lifecycle (on_boot → on_init → on_pre_append → on_post_append → on_halt) |
| `ledger.rs` | Append-only ledger with tamper detection |
| `sdk/actor.rs` | Lock-free concurrent agent model, Boltzmann softmax frontier routing (T=0.5) |
| `sdk/snapshot.rs` | Immutable universe snapshot — agents read, never mutate |
| `sdk/protocol.rs` | Agent output parser — JSON `<action>{...}</action>`, reject-only (no repair) |
| `sdk/prompt.rs` | Minimal prompt template — state-only |
| `sdk/tool.rs` | TuringTool trait + AntiZombiePruning + OverwhelmingGapArbitrator |
| `sdk/tools/wallet.rs` | WalletTool — balance + YES/NO/LP portfolios |
| `sdk/tools/search.rs` | Free SearchTool — zero-cost Mathlib search |
| `sdk/tools/librarian.rs` | Librarian — compresses tape into agent memory |
| `sdk/sandbox.rs` | Isolated process sandbox — Lean 4 with timeout + SIGKILL |
| `drivers/llm_http.rs` | Resilient HTTP client — multi-provider routing |

## Four Engines (from Magna Carta)

| Engine | Purpose | Implementation |
|--------|---------|---------------|
| Epistemic Engine | Free search/view tools (Law 1) | `sdk/tools/search.rs` |
| Pure Capital Engine | Invest-only economy (Law 2) | `sdk/tools/wallet.rs` + `prediction_market.rs` |
| Semantic Guillotine | OMEGA detection | Experiment-specific oracle tools |
| Speciation Engine | Per-agent DNA evolution (Law 3) | `sdk/tools/librarian.rs` |

## Actor Model

```
Event Reactor (serial) → watch::channel (broadcast) → N agents (parallel)
Boltzmann softmax routing (T=0.5)
30s timeout → generation rebirth
Superfluid clearing (MapReduce every append)
```
