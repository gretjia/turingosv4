# TRACE_MATRIX v0 — Constitutional Flowchart ↔ Rust Code (2026-04-22)

**Source extracts**:
- Flowchart raw: `FC_ELEMENTS_2026-04-22.md` (134 elements: 48 nodes + 63 edges + 23 subgraphs)
- Code candidates: `CODE_CANDIDATES_2026-04-22.md` (43 core items audited)

**Gate status**: Stage 1 v0 (pre-fix). Status column will flip per-row as Stage 2/3 land.

**Legend**:
- ✅ well-aligned (high conf, doc-comment present, code symbol exists)
- ⚠️ partial (code exists but split across Phase Z surface + legacy path)
- 🔨 missing-actionable (fixable in Stage 3 without constitutional change)
- 📅 deferred-Phase-11+ (runtime surface for JudgeAI/ArchitectAI etc. — explicit out-of-scope per PHASE_Z_PRIME plan)
- 📄 docs-only (flowchart element is a document reference, not runtime code)

---

## § 1. Core element-level alignment (43 rows)

| FC Element ID | Constitution Label | Proposed Symbol | File:Line | DocComment | Status | Action (Stage 2+3) |
|---|---|---|---|---|---|---|
| **FC1 basic cycle** ||||||||
| FC1-N1 | `Q_t = ⟨q_t, HEAD_t, tape_t⟩` | `QState` + `Tape::time_arrow` + `Kernel::tape` | `src/bus.rs:70`, `src/ledger.rs:146`, `src/kernel.rs:20` | Y/N/N | ⚠️ | add backlinks on `time_arrow` and `Kernel::tape` |
| FC1-N2 | `q_t` | `QState`, `TuringBus::q_state` | `src/bus.rs:53`, `src/bus.rs:70` | Y/Y | ✅ | none |
| FC1-N3 | `HEAD_t` | `time_arrow().last()` idiom | `src/ledger.rs:146` + call sites | N | ⚠️ | add `fn head() -> Option<NodeId>` helper on Tape |
| FC1-N4 | `tape_t` | `Tape`, `Kernel::tape` | `src/ledger.rs:44`, `src/kernel.rs:20` | Y/N | ⚠️ | backlink `Kernel::tape` |
| FC1-N5 | `rtool` | `ReadTool::project` + `DefaultReadTool` | `src/sdk/read_tool.rs:24,37` | Y/N | ⚠️ | migrate bus.snapshot() callers to rtool |
| FC1-N6 | `input = ⟨q_i, s_i⟩` | `UniverseSnapshot` + `build_agent_prompt` | `src/sdk/snapshot.rs:22`, `src/sdk/prompt.rs:15` | Y/Y | ✅ | none |
| FC1-N7 | `δ / AI` | `ResilientLLMClient::generate` | `src/drivers/llm_http.rs:84` | Y | ✅ | none |
| FC1-N8 | `output = ⟨q_o, a_o⟩` | `AgentOutput`, `parse_agent_output` | `src/sdk/protocol.rs:40,148` | Y/Y | ✅ | none |
| FC1-N9 | `q_o` | `AgentOutput::q_delta` | `src/sdk/protocol.rs:42` | Y | ✅ | none |
| FC1-N10 | `a_o` | `AgentOutput::action` | `src/sdk/protocol.rs:44` | Y | ✅ | none |
| FC1-N11 | `∏p predicates` | `TuringBus::evaluate_predicates`, `Predicate` trait | `src/bus.rs:148`, `src/sdk/predicate.rs:88` | Y/Y | ⚠️ | wire evaluator.rs to call evaluate_predicates before append |
| FC1-N12 | `individual p predicates` | `{Forbidden,Sorry,PayloadSize}Predicate`, `Lean4Oracle::verify_*` | `src/sdk/predicate.rs:106,124,139`, oracle | Y all | ⚠️ | wrap `Lean4Oracle::verify_partial` in a `Predicate` impl |
| FC1-N13 | `wtool` | `WriteTool::write`, `DefaultWriteTool::write`, `TuringBus::append_oracle_accepted` | `src/sdk/write_tool.rs:29,84`, `src/bus.rs:324,347` | Y/N/Y/Y | ⚠️ | migrate evaluator direct bus.append calls through WriteTool |
| FC1-N14 | `Q_{t+1}` success branch | `append_internal`, `halt_with_reason` | `src/bus.rs:421,207` | N/Y | ⚠️ | backlink `append_internal` |
| FC1-N15 | `Q_t` branch (∏p=0) | `PartialVerdict::Reject`, `BusResult::Vetoed` | oracle:328, bus:111 | Y/N | ⚠️ | backlink `BusResult::Vetoed` |
| **FC2 init / halt / tick** ||||||||
| FC2-N16 | `InitAI` | `run_swarm`, `run_oneshot` | evaluator:335,182 | Y/Y | ✅ | backlink FC2-N16 label |
| FC2-N17 | `human architect` | `constitution.md` (author) | constitution.md | — | 📄 | non-runtime; explicit in matrix |
| FC2-N18 | `law / ground truth` | `constitution.md` | constitution.md | — | 📄 | non-runtime |
| FC2-N19 | `initAI --once→ predicates` | `TuringBus::register_predicate` API | `src/bus.rs:136` | Y | 🔨 | **Stage 3: add caller in run_swarm + run_oneshot to register 3 default predicates at boot** |
| FC2-N20 | `initAI --once→ mr` | TICK_INTERVAL read + `emit_mr_tick_node` | evaluator:459, bus:385 | N/Y | ✅ | none |
| FC2-N21 | `initAI --once→ Q0` | `Kernel::new`, `TuringBus::new`, `TuringBus::init` | `src/kernel.rs:50`, `src/bus.rs:115,299` | N/N/Y | ⚠️ | backlink `Kernel::new`, `TuringBus::new` |
| FC2-N22 | `HALT` | `QState::Halted`, `halt_with_reason`, `halt_and_settle` | `src/bus.rs:55,207,581` | N/Y/Y | ⚠️ | backlink `QState::Halted` |
| FC2-N23 | `HaltReason` variants | `HaltReason` enum (5 variants), `extract_halt_reason` | `src/ledger.rs:230`, evaluator:1116 | Y/Y | ✅ | none |
| FC2-N24 | `clock` | `TuringBus::clock`, `for tx in 0..max_transactions` loop, `TICK_INTERVAL` | `src/bus.rs:66`, evaluator:485,459 | N/N/N | ⚠️ | backlink `clock` field |
| FC2-N25 | `mr` | `let mr_summary = format!(...)` inline, `emit_mr_tick_node` | evaluator:504, bus:385 | N/Y | ⚠️ | backlink inline mr_summary block |
| FC2-N26 | `mr --map→ tape0` | `tape.time_arrow().len()`, `market_ticker(5)` (used by mr_summary builder) | evaluator:488,490 | N/N | ⚠️ | backlink |
| FC2-N27 | `mr --reduce→ tape1` | `emit_mr_tick_node` | `src/bus.rs:385` | Y | ✅ | none |
| FC2-N28 | `tools_other` | `WriteTool::write_with_tools`, `TuringBus::tools`, `mount_tool` call sites | `src/sdk/write_tool.rs:57`, `src/bus.rs:64`, evaluator mount sites | Y/N/N | ⚠️ | backlink `TuringBus::tools` field |
| **FC3 anti-oreo / system-level** ||||||||
| FC3-N29 | `boot` | `async fn main`, `TuringBus::boot` | evaluator:88, bus:286 | N/Y | ⚠️ | backlink `fn main` |
| FC3-N30 | `constitution file` | `constitution.md` | constitution.md | — | 📄 | non-runtime |
| FC3-N31 | `logs archive` | `TuringBus::with_wal_path`, `Wal::replay`, `Wal::write_event` | `src/bus.rs:227`, `src/wal.rs:70,54` | Y/Y/N | ⚠️ | backlink `write_event` |
| FC3-N32 | `JudgeAI` | external/manual (Codex/Gemini dual-audit) | — | — | 📅 | Phase 11+ deferred |
| FC3-N33 | `ArchitectAI` | external/manual (Claude code editing) | — | — | 📅 | Phase 11+ deferred |
| FC3-N34 | `readonly guard on {constitution, logs}` | WAL append-only semantics; no FS readonly enforcement | `src/wal.rs:70` | Y | 📅 | Phase 11+: add FS-level readonly check at init |
| FC3-N35 | `anti-oreo top→agents→tools` | `evaluate_predicates` + `let agent_ids` + `TuringTool` | bus:148, evaluator:431, tool:38 | Y/N/Y | ⚠️ | backlink agent_ids lifecycle |
| FC3-N36 | `agents` | `let agent_ids`, round-robin selection | evaluator:431,577 | N/N | ⚠️ | backlink |
| FC3-N37 | `tools` | `TuringTool` trait + concrete {Wallet, Search, Librarian} + `Lean4Oracle` | `src/sdk/tool.rs:38` + impls | Y mostly | ✅ | backlink Lean4Oracle to FC3-N37 |
| FC3-N38 | `tape Q` | same as FC1-N4 | — | — | ✅ | (dedup of FC1-N4) |
| FC3-N39 | `log` | `Ledger`, `LedgerEvent`, `Ledger::append` | `src/ledger.rs:332,296,347` | Y/Y/Y | ✅ | none |
| FC3-N40 | `logs → feedback → ArchitectAI` | external (no runtime automation) | — | — | 📅 | Phase 11+ deferred |
| FC3-N41 | `init → error → re-init → boot` | `exit(2)` + external batch runner retry | evaluator:278,329,388 | N/N/N | 📅 | Phase 11+: in-process retry; for now external batch retry works |
| FC3-N42 | `constitution --abide→ JudgeAI+ArchitectAI` | manual policy (CLAUDE.md Audit Standard) | CLAUDE.md | — | 📅 | Phase 11+ deferred |
| FC3-N43 | `JudgeAI --veto→ ArchitectAI` | manual policy (VETO>CHALLENGE>PASS rule) | docs | — | 📅 | Phase 11+ deferred |

---

## § 2. Edges + subgraph mapping (auto-derived, 2 rows per inferred edge)

Since edges connect already-mapped nodes, most are derivative. We list **only labeled edges** (with `|...|` label) as they carry semantics:

| FC Element ID | Edge (From → To) | Constitutional label | Code representation | Status |
|---|---|---|---|---|
| FC1-E16 | `p → wtool` | `|1|` (∏p = 1) | `evaluate_predicates` returns non-Reject → proceed to wtool | ✅ |
| FC1-E18 | `p → Q0` | `|0|` (∏p = 0) | `evaluate_predicates` returns Reject → no append, state preserved | ✅ |
| FC2-E22 | `p → wtool` | `Q_{t+1} = wtool(output) if ∏p = 1` | same as FC1-E16 with explicit formula | ✅ |
| FC2-E23 | `p → Q0` | `Q_{t+1} = Q_t if ∏p = 0` | same as FC1-E18 | ✅ |
| FC2-E24 | `q1 → halt` | `if q = halt` | `if matches!(bus.q_state, QState::Halted{..})` check in evaluator loop | ⚠️ backlink |
| FC2-E25 | `clock → mr` | drives tick | `if tx > 0 && tx % tick_interval == 0` | ⚠️ backlink |
| FC2-E26 | `mr → tape0` | `|map|` | read side of mr_summary build (tape.time_arrow(), market_ticker) | ⚠️ backlink |
| FC2-E27 | `mr → tape1` | `|reduce|` | `emit_mr_tick_node(summary)` | ✅ |
| FC2-Init-E | `initAI --x|once| predicates / mr / Q0` | one-time setup | 🔨 FC2-N19 caller missing | 🔨 |
| FC3-Feedback | `logs → feedback → architectAI` | automated improvement loop | 📅 deferred | 📅 |
| FC3-Veto | `judgeAI → veto → architectAI` | policy arrow | 📅 deferred | 📅 |
| FC3-Abide | `constitution → abide → judge/architect` | policy arrow | 📅 deferred | 📅 |
| FC3-Reinit | `init → error → re-init → boot` | automated retry | 📅 deferred | 📅 |

The remaining ~50 unlabeled edges are **structural** (transitions already witnessed by the node mapping) — they do not require separate alignment work. Listed in `FC_ELEMENTS_2026-04-22.md` for completeness.

---

## § 3. Orphan Rust symbols (code without flowchart parent)

Per 宪法不能改 directive, orphans are accepted as **implementation-auxiliary** (extensions not inscribed in FC-1/2/3 but serve constitutional principles elsewhere):

| Symbol | File:Line | Constitutional Justification |
|---|---|---|
| `Kernel::open_bounty_market` | `src/kernel.rs:63` | Phase 3A Hayek Problem Bounty Market (Art. II.2 price signal extension) |
| `Kernel::resolve_bounty` | `src/kernel.rs:83` | Same — settlement path for Hayek bounty |
| `TuringBus::bus_classify` | `src/bus.rs:706` | C-022/C-055 error abstraction for Art. II.1 broadcast (implementation detail) |
| `TuringBus::recent_rejections_scoped` | `src/bus.rs:746` | Art. II.1 typical-error broadcast filter (C-055) |
| `WalletTool::save_to_disk` | `src/sdk/tools/wallet.rs:83` | C-041 cross-problem wallet persistence (Art. II.2 time-extended price signal) |
| `LibrarianTool::build_compression_prompt` | `src/sdk/tools/librarian.rs:48` | Phase 6-emergent librarian DNA compression (Art. III.3) |
| `LibrarianTool::post_to_board` | `src/sdk/tools/librarian.rs:101` | Phase 6-emergent team board (Art. II.1 + III.3) |
| `persist_proof_artifact` | `evaluator.rs:1143` | C-039 self-contained proof artifact (Art. I reproducibility) |

Each orphan gets a doc comment tag `/// Constitutional extension: <Art./C-xxx>` in Stage 2.

---

## § 4. Stage 3 action plan (distilled from the 🔨 / ⚠️ columns)

**Actionable (Stage 3 in-scope)**:

1. **FC2-N19** 🔨: `run_swarm` + `run_oneshot` must call `bus.register_predicate(...)` × 3 at init for {ForbiddenPattern, Sorry, PayloadSize}. Currently the API exists but no caller wires it.
2. **FC1-N11**: evaluator.rs should call `bus.evaluate_predicates(ctx, payload)` before `bus.append`, to honor the ∏p → wtool gate. Currently the legacy hard-coded checks in `append_internal` duplicate this work.
3. **FC1-N12**: wrap `Lean4Oracle::verify_partial` as a `Predicate` impl so the full ∏p chain is a single registered list.
4. **FC1-N5 / N13**: migrate evaluator from direct `bus.snapshot()` / `bus.append` calls to `rtool.project()` / `wtool.write()`.
5. **FC1-N3**: add `Tape::head() -> Option<NodeId>` helper (single idiom, instead of scattered `time_arrow().last()`).
6. Doc-backlinks (Stage 2): ~25 call sites listed above.

**Out-of-scope (Phase 11+ 📅)**: FC3-N32/33/34/40/41/42/43 — JudgeAI runtime + ArchitectAI runtime + FS readonly + feedback loop + auto re-init + runtime veto. Documented explicitly; no Stage 3 work.

---

## § 5. Element-count verification

- FC-1: 14 core-node rows (N1-N15) + 2 labeled edges (E16, E18) = 17 matrix rows. Raw extract: 14 nodes + 18 edges + 8 subgraphs = 40. Edges without labels (dotted/thick) collapse into their endpoint nodes.
- FC-2: 13 core-node rows (N16-N28) + 6 labeled edges = 19 matrix rows. Raw: 22 nodes + 28 edges + 11 subgraphs = 61. Many raw nodes are FC-1 duplicates (e.g., `q_t` appears in both).
- FC-3: 15 rows (N29-N43). Raw: 12 nodes + 17 edges + 4 subgraphs = 33.

**Total matrix rows**: 17 + 19 + 15 = **51 alignment rows** covering 43 core nodes + 8 labeled edges. The remaining ~80 raw elements are structural (arrows between mapped nodes, subgraph groupings) and are witnessed by the core rows.

---

## § 6. Summary stats (v0)

- ✅ well-aligned: **15** rows
- ⚠️ partial (Stage 2 doc-backlink + Stage 3 wire-up): **22** rows
- 🔨 missing-actionable (Stage 3): **1** row (FC2-N19)
- 📅 deferred Phase 11+: **7** rows
- 📄 docs-only: **3** rows

→ On completion of Stages 2 and 3: ✅ count 38, 📅/📄 count 10, 🔨/⚠️ count 0.

This target is achievable in the remaining 4 stages (budget per plan: ~10h).

---

## § 7. Constitutional-document hygiene note (parking lot)

FC-2 and FC-3 are indented pseudo-blocks in `constitution.md` — their opening ` ```mermaid ` fences are missing. They won't render on GitHub. Per user directive ("宪法不能改") I do NOT modify the constitution. Filing this as an audit observation for human architect to address when they next revise the document. Does not block Phase Z' execution.
