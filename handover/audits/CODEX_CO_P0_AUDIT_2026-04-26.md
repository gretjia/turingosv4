# Codex CO P0.7 Co-Executor Audit

## 1. Implementability check

Scope note: I did not run `cargo build/test/check` and did not modify `src/`. Evidence below is from read-only file inspection plus current upstream docs for `gix`/`git2`.

### CO P0 atom group verdicts

| Atom group | Verdict | Findings |
|---|---|---|
| CO0.1-CO0.2 blueprint + plan save | PASS | The referenced docs exist and are line-numbered here: `/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:1` and `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:1`. |
| CO0.3 Constitution Art. 0.5 draft/enactment | CHALLENGE | Plan says edit `constitution.md` and test `tests/constitution_root_amendment.rs` (`CO0.3`, `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:31`), but Amendment says only a draft is created and enactment waits for user wake (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:52`, `:61-63`). Conformance cannot mechanically pass against `constitution.md` until the cp workflow is done. |
| CO0.4 PREREG amendment v2 | CHALLENGE | Plan says `CO0.4` is a doc atom (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:32`), while Amendment says the PREREG v2 draft remains unenacted (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:53`, `:63`). This is implementable as a draft, not as a frozen governance input. |
| CO0.5 TFR v1 deprecation | PASS | The requested legacy banner exists in `/home/zephryj/projects/turingosv4/handover/architect-insights/TFR_MASTER_PLAN_2026-04-26.md:3-10`; this matches `CO0.5` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:33`). |
| CO0.6 Trust Root migration | VETO | Count mismatch: Plan says 43 -> 47 (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:34`); Amendment says 43 -> 48 (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:55`) and "5 new entries" (`:76`); actual manifest comment says 43 -> 49, +6 (`/home/zephryj/projects/turingosv4/genesis_payload.toml:158-167`). This must be reconciled before any PASS/PASS gate, because Trust Root count drift is exactly the class of governance rot the TR is supposed to prevent. |
| CO0.7 dual audit gate | CHALLENGE | The ledger seed records this Codex audit and a Gemini audit as pending (`/home/zephryj/projects/turingosv4/handover/audits/AUDIT_LEDGER.md:26-30`). Gate is implementable, but cannot pass until both reports exist and the CO0.6 count mismatch is resolved. |

### CO P1 atom group verdicts

| Atom group | Verdict | Findings |
|---|---|---|
| CO1.0 L0 Constitution Root | CHALLENGE | `ChainTape::genesis()` is specified to read `constitution_hash + signature + sudo_policy` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:49`; Blueprint maps the same fields at `/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:204`). Current `genesis_payload.toml` has `[pput_accounting_0]` and `[trust_root]`, but no explicit `human_signature`, `sudo_policy`, or `allowed_meta_update_rules` fields (`/home/zephryj/projects/turingosv4/genesis_payload.toml:100-167`). Hidden dependency: genesis schema must be extended before the atom is mechanically testable. |
| CO1.1.1 skeleton dirs | PASS | Directory skeleton creation is implementable as written (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:58`). |
| CO1.1.2 wal/ledger move | CHALLENGE | Plan says preserve content and "only re-export from src/lib.rs" (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:59`), but current imports are rooted at `crate::ledger` across `bus`, `kernel`, and `wal` (`/home/zephryj/projects/turingosv4/src/bus.rs:9`, `/home/zephryj/projects/turingosv4/src/kernel.rs:8`, `/home/zephryj/projects/turingosv4/src/wal.rs:15`), and `src/lib.rs` currently exports flat modules (`/home/zephryj/projects/turingosv4/src/lib.rs:1-8`). This is more than a move; it is a compatibility-shim migration. |
| CO1.1.3 sandbox move | CHALLENGE | Mechanically plausible, but current `lean4_oracle.rs` imports `turingosv4::sdk::sandbox::*` from the experiment crate (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:7-10`). Moving sandbox without a re-export breaks experiment code. |
| CO1.1.4 bus.rs 5-way split | VETO | Anti-Oreo split is necessary, but the proposed single full 5-way split is too coarse. `TuringBus` currently owns kernel, ledger, tools, config, clock, tx_count, generation, graveyard, and WAL in one stateful struct (`/home/zephryj/projects/turingosv4/src/bus.rs:40-51`), and `append_internal` interleaves predicate checks, tool hooks, investment routing, tape append, WAL, market creation, founder grant, post-hooks, event ledger, and counters (`/home/zephryj/projects/turingosv4/src/bus.rs:179-333`). Parallel A/B full rewrites are likely to produce divergent semantics that tests cannot arbitrate. See §3. |
| CO1.1.5 kernel.rs 3-way split | VETO | `Kernel` mixes topology, tape ownership, markets, bounty market, price ticker, and settlement resolution (`/home/zephryj/projects/turingosv4/src/kernel.rs:19-32`, `:63-126`, `:156-206`). Splitting into `state/`, `transition/`, and `economy/` is feasible only after shims and extracted interfaces exist. See §3. |
| CO1.1.6 layer-leak conformance | CHALLENGE | A filesystem audit can catch imports after the split, but it cannot catch semantic leaks through shared data shapes such as `UniverseSnapshot` unless the allowed dependency graph and allowed DTOs are specified. Current snapshot exposes full tape clone plus market ticker (`/home/zephryj/projects/turingosv4/src/bus.rs:542-582`). |
| CO1.2 QState 9 components | CHALLENGE | Blueprint names undefined future types (`AgentSwarmState`, `AgentVisibleProjection`, `BudgetSnapshot`, etc.) in `/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:31-43`. `replay_from_genesis` cannot be byte-identical while current nodes stamp wall-clock seconds (`/home/zephryj/projects/turingosv4/src/bus.rs:264-268`). |
| CO1.3 gix substrate | CHALLENGE | No `gix` or `git2` dependency exists today (`/home/zephryj/projects/turingosv4/Cargo.toml:7-15`, `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/Cargo.toml:11-18`). Plan references `evaluator.rs::on_cell_start` (`CO1.3.2`, `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:83`), but current evaluator has `run_swarm` (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:544`) and no observed `on_cell_start` symbol. Hidden dependency: lifecycle hook design. |
| CO1.4 CAS layer | CHALLENGE | Git-blob CAS depends on CO1.3 and on visibility policy from CO1.5. It is implementable after substrate and schema decisions, not independently (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:94-99`). |
| CO1.5 Predicate Registry + visibility | CHALLENGE | Moving `lean4_oracle.rs` from experiment to root requires dependency inversion: current file imports the root crate from the experiment crate (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/lean4_oracle.rs:7-10`). The Goodhart airgap test claims it will catch leaks via error/log/retry counts (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:111`), but retry-count/log-channel observability must be specified first. |
| CO1.6 Tool Registry | CHALLENGE | Current `TuringTool` has lifecycle hooks but no capability, permission, determinism, side-effect, or schema fields (`/home/zephryj/projects/turingosv4/src/sdk/tool.rs:38-69`). Implementable, but migration affects every mounted tool and prompt/tool descriptors. |
| CO1.7 Transition Ledger | VETO | White paper L4 has 12 fields including `task_id` (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:357-369`). Plan CO1.7.1 lists an "11-field struct" and omits `task_id` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:132`). Blueprint pseudo-code `WorkTx` also omits `task_id` (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:241-250`). This is a direct spec-to-plan mismatch. |
| CO1.8 Materialized State + Agent View | CHALLENGE | Plan points prompt-builder work at `experiments/.../agents/*.rs` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:151-153`), but current prompt assembly is monolithic in evaluator (`/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:920-1004`). The `agents/` directory is future architecture, not current refactor target. |
| CO1.9 Signal Indices | CHALLENGE | Determinism is under-specified. Current market ticker sorts only by floating price, with no stable tie-break over `HashMap` iteration (`/home/zephryj/projects/turingosv4/src/kernel.rs:187-192`, `:199-204`). A conformance test for L6 must force equal-price ties to catch nondeterministic broadcast order. |
| CO1.10 Signal dichotomy | PASS | Future symbols and test are concrete enough (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:173-177`), provided CO1.9 deterministic ordering is fixed. |
| CO1.11 Safety vs creation fail policy | CHALLENGE | Implementable after `PredicateRegistry` exists. Test can catch fail-open/fail-closed behavior, but only if predicate domain classification is part of registry schema (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:184-186`). |
| CO1.12 V/E closure | CHALLENGE | Plan uses wildcard test notation and says V/E closures are side effects of P1.5-P1.9 (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:190-199`). This will not mechanically catch each V/E invariant unless every V/E has an explicit atom, owner, production invocation, and test assertion. |
| CO1.13 TRACE_MATRIX_v3 | CHALLENGE | Pre-commit hook in `.git/hooks/pre-commit` is not versioned by default (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:205-207`). A tracked script is good; relying on local hook installation for DO-178C enforcement is not enough. |
| CO1.14 Phase 1 exit | PASS | Gate atom is clear (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:211-215`). |

### CO P2 atom group verdicts

| Atom group | Verdict | Findings |
|---|---|---|
| CO2.0 Inv 4 precondition | CHALLENGE | The atom targets `src/economy/escrow_vault.rs` before P2.2 creates it (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:223-228`). Use a real skeleton dependency, not a placeholder hidden in a future atom. |
| CO2.1 TaskMarket | CHALLENGE | Plan says price broadcast consumed by TaskMarket (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:233-236`), but price broadcast is P1.9 and current markets are tied to node IDs in `Kernel` (`/home/zephryj/projects/turingosv4/src/kernel.rs:114-126`). Requires an adapter. |
| CO2.2 EscrowVault | VETO | Economic invariants require monetary conservation (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:63-65`), but current market/economy code uses `f64` reserves and amounts (`/home/zephryj/projects/turingosv4/src/prediction_market.rs:21-27`, `:48-67`, `:87-109`). Before CO P2 entry, choose integer fixed-point or a decimal type; do not build escrow invariants on float equality. |
| CO2.3 ContributionLedger | CHALLENGE | `WorkTx`/`VerifyTx`/`ChallengeTx` overlaps CO1.7 `TransitionTx` but no unification rule is given (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:254-260`). The missing `task_id` problem in CO1.7 also propagates here. |
| CO2.4 AttributionEngine DAG | VETO | Plan admits subjectivity risk and says build from L4 read/write sets only (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:277`), but the atom list still defines edges as "builds-on/cites/reuses" and git multi-parent merge commits (`:267-273`). The determinism test "same DAG -> same weights" (`:272`) will not catch nondeterministic or self-serving DAG construction; it only tests the weight function after the hard part is already assumed. |
| CO2.5 ChallengeCourt | CHALLENGE | Rollback/slash (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:283-287`) depends on deterministic materialized-state rewind, challenge-window clock semantics, and settlement transaction ordering. Those are forward references to CO1.8/CO2.6 designs. |
| CO2.6 SettlementEngine | CHALLENGE | Deferred bonus window is explicitly unresolved in Plan §10 (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:451`). `finalize_reward` cannot be fully specified until that policy is frozen. |
| CO2.7 Roles | CHALLENGE | Economic file labels this "Agent 5" but lists six roles (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:91-100`). Blueprint repeats the inconsistency as "5 agent roles" while listing six (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:481`). Plan targets future `experiments/.../agents/*.rs` files (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:307-312`), but current implementation is evaluator-centric. |
| CO2.8 CTF stake symmetry | CHALLENGE | Plan asserts "Solver-YES = Challenger-NO at task close" (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:320-322`). The economic spec says YES/NO are event-bound rights, not equal balances at close (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:65`, `:94-96`). Test may encode a false invariant. |
| CO2.9 ReputationIndex | CHALLENGE | "Reputation cannot substitute predicates" (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:69`) is not caught by immutability alone. Test must try a high-reputation agent bypassing a failing predicate. |
| CO2.10 E-01..E-04 | CHALLENGE | Current production toggles still exist (`TAPE_ECONOMY_V2` in `/home/zephryj/projects/turingosv4/src/bus.rs:298` and `:345`). Renaming/default-on is implementable, but Plan provides no final canonical env/config name (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:340-343`). |
| CO2.11 RSP MVP-1 deployment | CHALLENGE | Economic §20 Phase 1 says local ledger economy uses `ledger.jsonl + SQLite + Python predicates` (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:102-112`), while Plan goes to in-process LedgerTape without a SQLite/Python compatibility/deprecation path (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:347-354`). |
| CO2.12 Phase 2 exit | PASS | Gate atom is clear and conservative (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:356-358`). |

## 2. Coverage check (white paper → Plan v3.1 atoms)

Blueprint §6 is explicitly only a seed, not the full trace matrix (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:344-370`). Therefore the claim "every white paper § -> at least one named atom" in Plan self-audit is not yet proven (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:455-461`), and the stronger self-claim that this is the first plan mapping every white paper § to code is too flattering unless the missing rows below are fixed (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:469-470`).

### Architecture white paper coverage

| White paper section | Blueprint §6 row? | Plan atom(s) / code symbol | Coverage verdict |
|---|---|---|---|
| Abstract, §0 design axioms (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:13-72`) | Partial via Constitution Art. 0 row | CO1.1, CO1.5, CO1.6, CO1.7 | CHALLENGE: Axiom 2 isolation (`:51-53`) lacks a direct trace row/test. |
| §1 problem statement (`:75-94`) | No | Indirect through predicates/layering | CHALLENGE: mostly motivational, but "chain records commitments not full reasoning" at `:89-94` should backlink to CO1.7/Inv11. |
| §2 Turing metaphor (`:97-115`) | Yes via Constitution Art. 0 row (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:352`) | CO1.0/CO1.6/CO1.7 | PASS. |
| §3 Anti-Oreo (`:118-193`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:358`) | CO1.1.* | PASS for mapping; implementation protocol VETO in §3. |
| §4 System state (`:197-239`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:359`) | CO1.2.* `state::q_state::QState` | CHALLENGE: architecture WP has 8 components; economic file adds `economic_state_t` (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:26-54`). Mapping is sound, but byte-identical replay needs deterministic timestamps. |
| §5 ChainTape L0-L6 (`:243-419`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:360`) | CO1.0, CO1.3-CO1.9 | CHALLENGE: L4 field mismatch (`task_id`) noted in §1. |
| §6 transition protocol (`:422-515`) | No seed row | CO1.7.5 `src/transition/mod.rs` (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:136`) | ORPHAN-IN-TRACE, not orphan-in-plan. Add Blueprint §6 row and test for reject path preserving `Q_t` while recording rejected tx (`:505-514`). |
| §7 signal quantification (`:518-618`) | Yes for §7 (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:361`) | CO1.9, CO1.10, CO1.11 | PASS with deterministic tie-order caveat. |
| §8 selective broadcast (`:620-668`) | No seed row | CO1.9.4 price broadcast; partial current bus rejection classes | ORPHAN-PARTIAL: §8.1 individual error, §8.2 typical error, and §8.4 exploration/exploitation need explicit symbols/tests, not just price broadcast. |
| §9 selective shielding (`:671-721`) | Only §9.4 Goodhart row (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:362`) | CO1.5.2/1.5.7, CO1.8.6/1.8.7 | CHALLENGE: §9.1-9.3 are not in Blueprint §6 seed; add rows for error hiding, minimal context, and correlation shielding. |
| §10 economic discipline (`:724-765`) | No architecture seed row; economic invariants row covers part | CO2.0, CO2.2, CO2.3, CO2.8 | CHALLENGE: needs direct mapping for Law 1/Law 2 and numeric type. |
| §11 Boot (`:768-813`) | No seed row | CO1.0, CO0.6 | CHALLENGE: genesis fields in spec (`:780-795`) exceed current manifest fields. |
| §12 Go Meta (`:816-903`) | Yes, but marked Phase 3 deferred (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:363`) | No CO P1/P2 implementation except economic roles | ORPHAN-FOR-v4: architecture §17 says v4 includes Phase 3 prep (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:1013-1024`), but Blueprint de-scopes MetaTape to v4.1 (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:404`). This requires explicit user decision, not just ArchitectAI default. |
| §13 blockchain placement (`:907-985`) | No seed row except Art. 0.4 | CO1.3 and Plan §5 out-of-scope | CHALLENGE: local HashChainTape and GitTape are mapped; permissioned/public phases correctly out-of-scope, but consensus fallacy test maps only via Inv12. |
| §14 data structures (`:989-992`) | No | CO1.7/P2.3 implied | ORPHAN: file says full examples are elsewhere; Plan references "appendix A" but v3.1 has no appendix A. |
| §15 MVP (`:995-998`) | No | CO P1/P2 broad plan | CHALLENGE: too thin in current source file to verify. |
| §16 failure modes (`:1001-1009`) | No | Risk register only | ORPHAN-PARTIAL: Ledger Poisoning, Predicate Gaming, Correlated Collapse, Irreversible Writes, Consensus Fallacy need explicit tests or trace rows. |
| §17 roadmap (`:1013-1024`) | Blueprint §7 | Plan P1/P2 + out-of-scope §5 | CHALLENGE: Phase 3 prep discrepancy above. |
| §18 conclusion (`:1027-1046`) | No | Broad architecture | PASS as summary, no unique code symbol required. |
| RSP appendix (`:1050-1066`) | Economic rows | CO P2.* | CHALLENGE: appendix lists 8 core components at `:1058`, economic chapter lists 9 modules including PriceIndex (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:77-89`). |

### Economic white paper coverage

| Economic section | Blueprint §6 row? | Plan atom(s) / code symbol | Coverage verdict |
|---|---|---|---|
| §0 core calibration (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md:9-24`) | No | CO2.* broad | CHALLENGE: "economy is signal/risk/attribution/settlement, not token speculation" needs a negative conformance test for no direct post-accept mint/speculation path. |
| §2 Q_t extension (`:26-54`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:364`) | CO1.2.1-CO1.2.4 | PASS mapping; implementation depends on deterministic replay. |
| §18 invariants (`:56-75`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:365`) | CO2.0-CO2.10 + CO1.5/1.7 | CHALLENGE: Inv8 and Inv9 tests as planned are insufficient (§1). |
| §19 RSP modules (`:77-89`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:366`) | CO2.1-CO2.9 plus CO1.5 PredicateRunner | PASS mapping; numeric and DAG caveats remain. |
| §7 agent roles (`:91-100`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:367`) | CO2.7.* | CHALLENGE: "Agent 5" vs six roles inconsistency must be corrected. |
| §20 deployment (`:102-112`) | Blueprint §7 | Plan P1/P2, Plan §5 | CHALLENGE: Phase 1 mentions SQLite/Python predicates; Plan does not map or explicitly deprecate them. |
| §15 blockchain tech positioning (`:114-124`) | No seed row | Plan §5 out-of-scope | PASS for scope, but add trace row to avoid future re-litigation. |
| §21 final formula (`:125-137`) | Yes (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:368`) | CO2.6.4/CO2.6.5 | CHALLENGE until Utility window and Constitution(Q_t) check are specified. |
| Architecture relationship (`:139-149`) | No | Cross-reference only | PASS as explanatory section, but use it to add missing rows for Goodhart/economic Laws. |

## 3. Anti-Oreo split feasibility (CO1.1.4 + CO1.1.5)

Observed current state:

- `src/bus.rs` is not just a bus. It holds top-layer predicate-ish checks (`forbidden_patterns`, payload caps), middle-facing tool hooks, bottom-layer WAL/ledger writes, and economy/market side effects in one serial reactor (`/home/zephryj/projects/turingosv4/src/bus.rs:40-51`, `:179-333`).
- `src/kernel.rs` is not just state. It holds `Tape`, per-node markets, a run-level bounty market, market trading, golden-path trace, and settlement resolution (`/home/zephryj/projects/turingosv4/src/kernel.rs:19-32`, `:63-206`).

Answer (a): the proposed 5-way / 3-way split is directionally right but not clean as a single atom. Cross-cutting concerns resisting clean splitting:

- Atomic transition boundary: `bus.append_internal` commits a node to kernel, writes WAL, creates market, grants founder shares, runs post-hooks, appends ledger event, and increments counters in one function (`/home/zephryj/projects/turingosv4/src/bus.rs:271-330`). Splitting these without a transaction object risks partial state.
- Current kernel market methods assume node IDs already exist in tape before market creation (`/home/zephryj/projects/turingosv4/src/kernel.rs:114-126`), tying economy to topology.
- Rejection feedback is a sidecar `graveyard`, not tape-canonical state (`/home/zephryj/projects/turingosv4/src/bus.rs:48`, `:439-539`), directly conflicting with Constitution Art. 0.2's sidecar warning (`/home/zephryj/projects/turingosv4/constitution.md:60-66`).
- Read view currently clones full tape into `UniverseSnapshot` (`/home/zephryj/projects/turingosv4/src/bus.rs:574-582`), so shielding cannot be achieved by moving files alone.

Answer (b): parallel branch A/B is the wrong first protocol for CO1.1.4/CO1.1.5. Use sequential staged refactor with shims:

1. Extract pure DTOs (`TransitionDraft`, `PredicateDecision`, `LedgerAppendRequest`, `MarketSideEffect`) while keeping `TuringBus` public API stable.
2. Add re-export shims so current imports keep compiling during moves.
3. Move WAL/ledger/tape primitives first, with compatibility aliases.
4. Split economy from kernel behind an interface.
5. Only then retire `src/bus.rs`/`src/kernel.rs`.

Reason: A/B full-branch implementations can both pass shallow conformance while choosing incompatible transaction boundaries. The correct comparison target is a staged semantic golden master, not two independent large rewrites.

Answer (c), highest-risk single line:

`/home/zephryj/projects/turingosv4/src/bus.rs:268`

```rust
            completion_tokens: 0,
```

Why this line: Constitution Art. 0.2 already names this class of defect as a canonical tape violation (`/home/zephryj/projects/turingosv4/constitution.md:67-76`). If the split moves this line into a new `TransitionTx`/CAS/tape layer without fixing schema and replay, it preserves the old rot under new directories.

## 4. gix Path B viability spike

Observed from current code:

- No runtime git integration exists: `Cargo.toml` lacks `gix`/`git2` (`/home/zephryj/projects/turingosv4/Cargo.toml:7-15`, `/home/zephryj/projects/turingosv4/experiments/minif2f_v4/Cargo.toml:11-18`), matching Constitution Art. 0.4's grep-based gap report (`/home/zephryj/projects/turingosv4/constitution.md:124-135`).

Current upstream docs:

- `gix::Repository::commit` takes `parents: impl IntoIterator<Item = impl Into<ObjectId>>`; `new_commit` also accepts an arbitrary parents iterator. This supports multi-parent commits as an API shape (docs.rs `gix::Repository` lines 1327-1342, https://docs.rs/gix/latest/gix/struct.Repository.html).
- Caveat: when updating a reference, the first parent is expected to be the current ref target, and commit fails if a reference lock cannot be acquired (docs.rs `gix::Repository` lines 1331-1334, https://docs.rs/gix/latest/gix/struct.Repository.html). Multi-parent object creation is viable; concurrent ref updates need serialization or per-cell isolated refs.
- `gix::ThreadSafeRepository::init` creates a repo in a directory and fails if `.git` already exists (docs.rs `gix::ThreadSafeRepository` lines 296-303, https://docs.rs/gix/latest/gix/struct.ThreadSafeRepository.html). `ThreadSafeRepository` exists for `Sync` contexts and converts to thread-local repos (docs.rs lines 198-200).

Answer (a): yes, gix appears to support multi-parent commits. This is observed API evidence, not a local spike result.

Answer (b): unverified hypothesis: concurrent runtime repo init across many subprocess workers should work if each worker uses a unique `experiments/<problem>/<run_id>/runtime_repo/` path as Plan CO1.3.2 specifies (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:83`). Same-path concurrent init or same-ref concurrent commit is not safe without locks/serialization, because gix documents reference-lock failure.

Answer (c): `git2-rs` is a viable fallback, not a clean drop-in. It supports commit creation with `parents: &[&Commit<'_>]` (docs.rs `git2::Repository` lines 791-795, https://docs.rs/git2/latest/git2/struct.Repository.html), but it introduces libgit2 bindings (`libgit2-sys` dependency in docs.rs `git2` lines 20-23), and `Repository` is `Send` but `!Sync` (docs.rs `git2::Repository` lines 239-246). That means it may be mature for git object operations but still needs explicit thread/process isolation and build/dependency care.

## 5. Tri-model protocol critique

Generator != Evaluator is not fully preserved for STEP_B.

Evidence:

- STEP_B says Branch B is Codex implementation (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:65-68`).
- The same STEP_B lifecycle then says "Codex final review" (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:72-74`).
- Protocol §9 table lists code generator as "Claude OR Codex" and evaluator as "Codex (review), Gemini (cross-§)" (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:159-164`), while the hard rule only forbids the code-writing model from writing the gating test (`:166`).

This is workable only if Branch A wins and Codex's Branch B is treated as an implementation probe, not as the evaluated artifact. If Branch B wins, Codex reviewing it violates the spirit of Generator != Evaluator. Even if Branch A wins, Codex has anchoring bias from having built a competing solution.

Required protocol amendment: for STEP_B atoms where Codex implements any candidate branch, final code review must be Gemini plus either Claude or a fresh Codex invocation with no branch-author context. The model that wrote the winning branch must not be the final reviewer of that branch.

## 6. Cost honesty

Protocol §5 estimates:

- Standard atoms: ~110 x $2-5 = $220-550 (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:99-101`)
- STEP_B atoms: ~22 x $5-10 = $110-220 (`:101`)
- Gemini heavy reviews: ~30 x $1-2 = $30-60 (`:102`)
- Phase exits: $75-120 (`:104-111`)
- Total: $435-950 (`:108-113`)

Modern API-rate sanity check:

- OpenAI official pricing shows GPT-5.1 Codex at $1.25 / 1M input tokens and $10 / 1M output tokens; GPT-5.2 Codex at $1.75 / 1M input and $14 / 1M output (OpenAI pricing page, https://platform.openai.com/docs/pricing/).
- Gemini 2.5 Pro official paid pricing is $1.25 / 1M input and $10 / 1M output for prompts <=200k tokens; $2.50 / 1M input and $15 / 1M output for prompts >200k (Google AI pricing, https://ai.google.dev/gemini-api/docs/pricing).

Math:

- A standard atom review with 60k input + 6k output on GPT-5.1 Codex costs `0.060 * 1.25 + 0.006 * 10 = $0.135`. On GPT-5.2 Codex: `0.060 * 1.75 + 0.006 * 14 = $0.189`.
- A heavy STEP_B implementation/review with 500k input + 50k output on GPT-5.2 Codex costs `0.500 * 1.75 + 0.050 * 14 = $1.575`. It reaches $5-10 only if repeated tool loops/failed attempts push total billed traffic above roughly 1.5M-3M mixed tokens or if a more expensive pro-class model is used.
- A Gemini heavy review with 220k input + 8k output costs `0.220 * 2.50 + 0.008 * 15 = $0.67` because it crosses the >200k bracket. $1-2 is plausible for full-phase packets or multiple rounds, but high for a single atom.

Verdict: CHALLENGE. The Protocol budget is conservative/high for raw API pricing, not underpriced, unless it is silently counting repeated retries, full-context uploads every time, hidden reasoning/output tokens, or non-API Codex credit accounting. Cost ledgers must record actual input/output token counts per invocation, not just dollar guesses.

Additional budget bug: Amendment says v4 budget is $435-950 with midpoint $700, but sets 80% threshold to $560 and 100% threshold to $700 (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:35-38`). That is 80%/100% of the midpoint, not of the stated ceiling. Audit ledger repeats this (`/home/zephryj/projects/turingosv4/handover/audits/AUDIT_LEDGER.md:56-60`).

## 7. Holistic verdict

Blueprint: CHALLENGE.

The Blueprint is a useful synthesis and catches real risk itself, including gix uncertainty and AttributionEngine determinism (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:493-497`). But its trace matrix is only a seed (`:344-370`), it omits `task_id` from the transition pseudo-code despite white paper L4 requiring it (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:357-369`; Blueprint `WorkTx` at `/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:241-250`), and its self-claim that it "does not invent new architectural concepts" is too strong for a doc that introduces a concrete four-root file taxonomy and de-scopes MetaTape (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:166-194`, `:404`, `:513`). Fixable, not passable as final.

Plan v3.1: VETO.

Do not enter CO P1 from this plan as written. The plan admits no sprint dependency graph yet (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:446-451`), sends the two riskiest atoms through full parallel A/B rewrites (`:61-62`, `:400-402`), omits `task_id` from the L4 transition schema (`:132`), and gives Inv8 a test that cannot catch the stated failure mode (`:267-277`). This is exactly the kind of "looks atomized but hides undefined contracts" plan that can recreate the prior V/E rot.

Protocol: CHALLENGE.

The conservative VETO > CHALLENGE > PASS rule is good (`/home/zephryj/projects/turingosv4/handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md:81-96`), and externalizing disagreement is good (`:145-153`). But STEP_B violates Generator != Evaluator when Codex implements Branch B and then performs final review (`:65-74`, `:159-166`). Cost estimates are also not tied to actual token accounting (`:97-113`). Amend before using it on source-code atoms.

Amendment v1: VETO.

The Amendment correctly freezes `src/` until user wake (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:61-65`), but it authorizes direct Trust Root mutation while calling the shift conservative (`:54-57`, `:83-100`), and the count now conflicts across Plan, Amendment, and actual manifest (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:34`; `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:55`; `/home/zephryj/projects/turingosv4/genesis_payload.toml:158`). It also records all D1-D6 as "auto-research = all-rec" while user is asleep (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:10-21`). That may be reversible, but it should not be treated as user approval for CO P1.

## 8. Top-3 must-fix before CO P1 entry

1. Fix the executable plan before source refactor: add a real dependency graph, correct L4 `TransitionTx` to include `task_id`, replace CO1.1.4/CO1.1.5 parallel full rewrites with staged shim refactors, and define numeric money type before P2. Evidence: missing `task_id` in Plan (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:132`) vs white paper L4 (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:357-369`), and current f64 economy (`/home/zephryj/projects/turingosv4/src/prediction_market.rs:21-27`).

2. Reconcile Trust Root governance before trusting CO0 PASS/PASS: Plan says 43->47, Amendment says 43->48, actual manifest says 43->49. Evidence: `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:34`, `/home/zephryj/projects/turingosv4/handover/architect-insights/CO_P0_AMENDMENT_v1_2026-04-26.md:55`, `/home/zephryj/projects/turingosv4/genesis_payload.toml:158-167`.

3. Repair trace coverage and test claims: Blueprint §6 must stop being a seed before anyone claims every white paper § is mapped. Add rows/tests for architecture §6, §8, §9.1-9.3, §11, §14-16, and economic §0/§20 details. Evidence: Blueprint seed status (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:344-370`) vs Plan self-claim (`/home/zephryj/projects/turingosv4/handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md:455-461`, `:469-470`).

## 9. What you'd want Gemini to look at independently

- Strategic correctness of the v4/v4.1 MetaTape de-scope: architecture white paper says v4 scope includes Phase 3 prep (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md:1013-1024`), Blueprint de-scopes MetaTape (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:404`). I can flag the contradiction; Gemini should judge whether it breaks the user's strategic intent.
- Economic game design, especially CO2.8's claimed YES/NO equality at task close and whether full Challenger/royalty/RSP is actually required for all 12 invariants. This needs game-theoretic review beyond code feasibility.
- Whether the Anti-Oreo "economy as own root" is conceptually sound. Blueprint says `src/economy/*` is structurally Top White but its own root for atom hygiene (`/home/zephryj/projects/turingosv4/handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md:166-194`). Gemini should test if that creates a fourth layer in practice.
- Whether the conformance suite design catches semantic violations, not just file existence. Highest-priority examples: Inv8 DAG construction, Goodhart private-predicate leak channels, L6 deterministic tie ordering, and reputation-not-predicate-substitute.
- Whether all-rec D1-D6 while the user is asleep is acceptable governance or should be demoted to "recommendations pending user confirmation" before any CO P1 gate.
