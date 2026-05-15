# REAL-5S -> REAL-9 Comprehensive Progress Report

Date: 2026-05-15 UTC

Purpose: provide architect-facing status, analysis, Codex judgment, and forward recommendations after the REAL-5S / REAL-6 / REAL-7 / REAL-8 / REAL-9 route landed on `main`.

Reading rule: every substantive judgment is followed by an HTML comment beginning with `DATA:`. Those comments are part of the report and point to the concrete command output, evidence file, audit, or matrix row supporting the claim.

## 1. Executive Status

The current repository is no longer in a dirty-tree implementation state for the REAL-5S -> REAL-9 package; the package was committed and pushed to `main`.
<!-- DATA: `git status --short` returned empty; `git rev-parse --short HEAD` returned `0f22843`; `git ls-remote origin refs/heads/main` returned `0f22843e19b2c3a8b2a98abb0365844bbcd49a85 refs/heads/main`; `git rev-list --left-right --count HEAD...origin/main` returned `0 0`. -->

The landing consists of one substantive ship commit plus one follow-up hook-log commit.
<!-- DATA: `git log --oneline -3` returned `0f22843 Record REAL-5S REAL-9 R-022 skip log`, `d87ad6c Ship REAL-5S through REAL-9 market activation`, `5239f4b Record REAL-4 prompt-only market activation evidence`. -->

The strongest accurate claim is that TuringOS now has a chain-backed role scaffold and lawful market-pressure substrate through REAL-9, not that spontaneous market emergence has been proven.
<!-- DATA: `handover/ai-direct/LATEST.md` session #49 states REAL-5S narrows to scaffold plus clean-negative evidence, REAL-6 adds lawful pressure, REAL-7 proves v3-structural pressure without v3 equivalence, REAL-8 is formal A/B, REAL-9 is launch synthesis. `handover/evidence/real5_overnight_20260514/REAL5_SCAFFOLD_RATIFICATION_REPORT.md` states `REAL-5 proves role scaffolding. REAL-5 does not prove market emergence.` -->

My view is that the project has crossed an important threshold: the market mechanism is now visible and structurally exercised on ChainTape/CAS, but the causal question "does market pressure improve agent performance or induce spontaneous trading?" remains open.
<!-- DATA: REAL-8 FINALZ `real8_arm_summary.tsv` has all arms `exit=0`, `audit=PROCEED`, `tasks=3`; market tx counts are A=0, B=4, C=10, D=10. The REAL-8 benchmark report shows solve_rate is `2/3` for A, B, C, and D, so current evidence shows more market activity in market arms without solve-rate separation. -->

The immediate next decision is architectural, not mechanical: decide whether to run a larger controlled REAL-8 family, ratify live REAL-6B, or perform a TRACE_MATRIX backlink cleanup before further feature atoms.
<!-- DATA: `handover/ai-direct/LATEST.md` Next Steps says give the REAL-5S -> REAL-9 bundle to the architect, run larger multi-model REAL-8 variants only with pinned-input discipline, and keep REAL-6B live real-LLM AttemptPrediction gated on explicit Class-4 ratification. `handover/alignment/OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` forward action requires a TRACE_MATRIX backlink pass before the next feature atom. -->

## 2. Route-Level Status

The architect's renamed route was followed: REAL-5S, REAL-6, REAL-7, REAL-8, and REAL-9.
<!-- DATA: `handover/directives/2026-05-15_REAL5S_REAL6_REAL7_REAL8_REAL9_ARCHITECT_ORIGINAL.md` preserves the route: `REAL-5S Scaffold ratification / clean-negative closure`, `REAL-6 Event Timing & Lawful Pressure`, `REAL-7 V3-Equivalent Structural Smoke`, `REAL-8 Formal Market A/B Benchmark`, `REAL-9 Launch synthesis / Whitepaper update`. -->

The Constitution Execution Matrix records all five rows as GREEN under the new REAL route section.
<!-- DATA: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §S rows mark REAL-5S, REAL-6, REAL-7, REAL-8, and REAL-9 as `🟢 GREEN`; the section also states the claim boundary that REAL-5S does not prove E2/E3 emergence and REAL-8 does not claim causality. -->

The TB log records the route as shipped with validation and audit evidence.
<!-- DATA: `handover/tracer_bullets/TB_LOG.tsv` row `REAL-5S-REAL9` is marked `shipped`; it records REAL-8/REAL-9 targeted tests `8/0`, Trust Root `1/0`, constitution gates `458/0/1`, workspace exit 0, and clean-context Codex PROCEED. -->

## 3. REAL-5S: Scaffold Closure And Clean Negative

REAL-5S correctly narrows the claim to role scaffolding and explicitly rejects E2/E3 emergence claims.
<!-- DATA: `handover/evidence/real5_overnight_20260514/REAL5_SCAFFOLD_RATIFICATION_REPORT.md` states `REAL-5 proves role scaffolding. REAL-5 does not prove market emergence.` It also says the report does not claim E2 spontaneous live agent market action or E3 persistent role-differentiated market behavior. -->

The role gateway now blocks Trader proof-style leakage rather than letting Trader agents silently act like proof solvers.
<!-- DATA: `REAL5_SCAFFOLD_RATIFICATION_REPORT.md` states the post-VETO trader-first run witnessed `accepted VerifyTx by Agent_0 Trader: 0`, `accepted WorkTx by Agent_0 Trader: 0`, and `Agent_0 Trader role_turn outcomes: PolicyRejected=5` at `handover/evidence/g_phase_real_5_trader_first_b8_rolegate_20260514T192523Z`. -->

Verifier behavior was observed, so REAL-5 did not merely produce inert roles.
<!-- DATA: `REAL5_SCAFFOLD_RATIFICATION_REPORT.md` lists non-zero verifier counts: R5-CORE3-B20 verify=6, R5-TRADER-FIRST-B12 verify=5, R5-MARKET-K0-B12 verify=3, R5-ADV5-B8 verify=5, R5-TRADER-FIRST-B8-ROLEGATE verify=1. -->

Trader buy remained zero during REAL-5S, which is a valid clean negative rather than a failed scaffold.
<!-- DATA: `REAL5_SCAFFOLD_RATIFICATION_REPORT.md` says both post-VETO rolegate runs have `buy_with_coin_router=0`; `REAL5_OVERNIGHT_EXPERIMENT_LEDGER.md` records `0 router buys` across successful bounded REAL-5 true-problem runs. -->

The dominant no-trade diagnosis is NoPool, not prompt wording alone.
<!-- DATA: `REAL5_CLEAN_NEGATIVE_NO_TRADE_REPORT.md` answers `Why no trade? NoPool dominates. Post-accept node market timing too late. Prompt-only exhausted.` It states the market K=0 adversary did not become `PromptBudgetExceeded`; Trader NoTrade stayed `NoPool`. -->

My interpretation: REAL-5S proves the system can organize role-scoped generation and block role leakage, but it also proves that proof-solver prompt variants are the wrong lever for market emergence.
<!-- DATA: REAL-5 overnight ledger shows prompt/role variants such as normal role order, trader-first, market K=0, adversarial5, and post-VETO rolegate enforcement all remained at `0 router buys`; architect original says stop prompt-only variants and move to lawful pressure. -->

## 4. REAL-6: Event Timing And Lawful Pressure

REAL-6A moved the market earlier with TaskOutcomeMarket, addressing the NoPool/post-accept timing diagnosis.
<!-- DATA: `handover/evidence/real6_task_outcome/REAL6A_TASK_OUTCOME_MARKET_REPORT.md` says REAL-6A implements `TaskOpenTx / EscrowLockTx -> MarketSeedTx for task_outcome event`, where the event is `task will be solved within budget/deadline`. -->

REAL-6A also repaired important safety edges before closure: EventResolve compatibility, malformed-tail fail-closed behavior, fail-closed market-decision CAS writes, and build fail-closed behavior.
<!-- DATA: `REAL6A_TASK_OUTCOME_MARKET_REPORT.md` records R1, R2, and R3 VETO remediations: old EventResolve bincode YES compatibility, malformed outcome tails fail closed, MarketDecisionTrace CAS writes fail closed, and `scripts/run_g_phase_batch.sh` exits on release build failure before `batch_evaluator`. -->

REAL-6B is deliberately limited to design plus scripted fixture; it should not be treated as a live real-LLM AttemptPrediction ship.
<!-- DATA: Architect original and `handover/evidence/real6_attempt_prediction/REAL6B_ATTEMPT_PREDICTION_FIXTURE_REPORT.md` both state `REAL-6B = design + scripted fixture only. No live real-LLM ship until explicit Class-4 ratification.` -->

The REAL-6B fixture establishes deterministic sealed-oracle ordering without sleep, with MarketClose before OracleResolve and Lean remaining absolute truth.
<!-- DATA: `REAL6B_ATTEMPT_PREDICTION_FIXTURE_REPORT.md` defines SubmitCandidate at `open_t - 1`, AttemptPredictionMarketOpen at `open_t`, role window ticks `open_t + 1 .. open_t + K`, MarketClose at `open_t + K + 1`, and OracleResolve after MarketClose. It maps SG-6B.1 through SG-6B.7 to tests. -->

REAL-6C restored economic pressure as a ChainTape-derived ConvictionBudget/PnL view, not as a sidecar source of truth.
<!-- DATA: `handover/evidence/real6_conviction_budget/REAL6C_CONVICTION_BUDGET_REPORT.md` says `derive_conviction_budget` calls canonical replay/QState-derived helpers and that REAL-6C has no sidecar PnL table or map-backed PnL source; it also says below risk cap cannot Trader/MarketMaker/Challenger high-risk actions but can still observe/read/abstain/solve/possibly verify. -->

REAL-6D introduced scheduler observation without admission control changes.
<!-- DATA: `handover/evidence/real6_scheduler_observe_only/REAL6D_OPPORTUNITY_SCHEDULER_REPORT.md` says SchedulerDecisionTrace is observe-only, dashboard renders `## §J.1 Opportunity Scheduler recommendation (observe-only)`, and not changed includes no sequencer admission change, no L4/L4.E predicate change, no typed transaction schema/discriminant/signing payload change, no price-as-truth, no ghost liquidity, and no f64 money path. -->

My interpretation: REAL-6 is the turning point from prompt tuning to institutional mechanics. It does not guarantee emergence, but it creates the missing causal channels that REAL-5 lacked: earlier uncertainty, visible price/PnL, role budgets, and non-binding scheduler signals.
<!-- DATA: REAL-5S clean negative says post-accept node market timing was too late and prompt-only exhausted; REAL-6A creates TaskOutcomeMarket at task opening; REAL-6C surfaces scoped PnL; REAL-6D surfaces price/PnL scheduler traces with `observe_only=true`. -->

## 5. REAL-7: Structural Pressure Smoke

REAL-7 achieved the architect's minimum structural pattern without chasing v3 transaction volume.
<!-- DATA: `handover/evidence/real7_structural_smoke/REAL7_V3_EQUIVALENT_STRUCTURAL_SMOKE_REPORT.md` primary evidence is `handover/evidence/g_phase_real_7_structural_smoke_r11_20260515T1032Z/`; command outcome records `batch_exit=0`, `audit_exit=0`, `audit_verdict=PROCEED`, `persistence_exit=0`, `persistence_passing=true`, `persistence_n_witnessed=5`. -->

REAL-7 produced a compact but complete market-pressure shape: task markets, buys, verification, challenges, resolutions, PnL delta, and autopsy-if-loss satisfaction.
<!-- DATA: REAL-7 report aggregate counts: L4 entries 36, L4.E entries 21, CAS objects 194; tx_kind_counts include task_open=3, escrow_lock=3, market_seed=3, cpmm_pool=3, buy_with_coin_router=6, work=3, verify=3, challenge=3, finalize_reward=3, terminal_summary=3, event_resolve=3. Dashboard §K shows task_outcome_market_count=3, scripted_attempt_prediction_market_count=3, buy_yes_router_count=3, buy_no_or_short_count=6, verify_tx_count=3, challenge_tx_or_no_challenge_reason_count=3, event_resolve_count=3, pnl_delta_count=6, autopsy_if_loss_satisfied=true. -->

REAL-7 should be treated as structural evidence, not as spontaneous market-emergence evidence.
<!-- DATA: REAL-7 report claim boundary states no forced live LLM investment claim, no E2/E3 spontaneous emergence claim from scripted fixture activity, and not v3-identical volume. -->

My interpretation: REAL-7 is valuable because it proves the end-to-end economic loop can be made to exist lawfully. Its limitation is equally important: the loop is still structurally assisted, not yet naturally self-sustaining.
<!-- DATA: REAL-7 command uses `TURINGOS_REAL7_SCRIPTED_TASK_OUTCOME_BUYS=Agent_1:Agent_2:10000`, `TURINGOS_REAL7_SCRIPTED_ATTEMPT_PREDICTION_FIXTURE=1`, and `TURINGOS_REAL7_SCRIPTED_VERIFY_CHALLENGE=Agent_2:Agent_3`; the report explicitly says no E2/E3 spontaneous emergence claim. -->

## 6. REAL-8: Formal Market A/B Benchmark

REAL-8 produced a clean four-arm benchmark with pinned problem set, model assignment, and budgets.
<!-- DATA: REAL-8 benchmark report pinned SHA-256 hashes: same problem set `a7bbb29cec0726769e5bb39a602c54713e7007bb717731f31298437d4b2367e8`, same model assignment `62d1e5862881ff8124ffa0159df78c62f91dde52cedbdd5fb966774440051526`, same budgets `70d88fcf2cf0e0b8826145b9176237be58820e9006faaa3fe9435f418859a42e`. -->

All four REAL-8 arms completed with chain-backed audit success.
<!-- DATA: `handover/evidence/real8_market_ab_20260515T_REAL8_FINALZ/real8_arm_summary.tsv` shows A/B/C/D each `exit_code=0`, `audit_verdict=PROCEED`, and `task_count=3`. -->

The market mechanism visibly increases market activity under market-enabled arms, especially with TaskOutcomeMarket.
<!-- DATA: REAL-8 FINALZ arm summary: A market disabled market_tx_count=0; B market visible no TaskOutcomeMarket market_tx_count=4; C TaskOutcomeMarket market_tx_count=10; D TaskOutcomeMarket plus scripted AttemptPrediction fixture market_tx_count=10. -->

The benchmark does not show a solve-rate improvement at current size.
<!-- DATA: REAL-8 report metrics table shows solve_rate `2/3` for A, B, C, and D. -->

The benchmark does not support causal claims yet.
<!-- DATA: REAL-8 report opens with `This report is descriptive benchmark evidence only. It does not claim causality.` SG-8.5 says no overclaim of causality PASS. -->

The strongest quantitative signal is not performance improvement but activation of market-visible machinery under earlier market timing.
<!-- DATA: A/B/C/D solve_rate all `2/3`, but market_tx_count changes from A=0 and B=4 to C=10 and D=10; C and D also record NoTradeReason distribution `invest_no_trade_no_perceived_edge=5`, while A and B record `none_observed`. -->

My interpretation: REAL-8 is exactly the kind of benchmark discipline this project needed. It turns "market emergence" from a narrative into measurable arms, but the sample size is too small and scripted elements are still present, so the correct conclusion is descriptive progress plus open causality.
<!-- DATA: REAL-8 uses 3 tasks per arm; REAL-8 report says negative result is valid and documented; REAL-7 and REAL-8 claim boundaries reject spontaneous emergence and causal overclaim. -->

## 7. REAL-9: Launch Synthesis

REAL-9 correctly narrows the launch narrative: v4 rebuilds v3 pressure under law rather than copying v3.
<!-- DATA: `handover/whitepapers/TURINGOS_GENERATIVE_ECONOMY_WHITEPAPER_UPDATE_REAL9.md` states `v4 does not copy v3`, `v4 rebuilds v3's economic pressure under constitution`, `price = signal, not truth`, and `market = role-specific institution, not prompt decoration`. -->

The developer manual repeats the hard operating boundaries that should govern the next phase.
<!-- DATA: `handover/whitepapers/TURINGOS_MARKET_DEVELOPER_MANUAL_REAL9.md` lists `no forced trades`, `no price-as-truth`, `no ghost liquidity`, `no f64 economy`, `no off-tape WAL as truth`, `no private CoT recording`, and `no raw-log broadcast`. -->

My interpretation: REAL-9 is ready as a launch synthesis only if it remains humble. It should present lawful pressure and chain-backed experimentation, not autonomous market emergence.
<!-- DATA: REAL-9 whitepaper says it is not claiming spontaneous trading, model ranking, price-as-truth, or unconstrained DeFi behavior; REAL-8 report says no causal claim. -->

## 8. Validation And Audit State

The final Class-4 Harness for REAL-8/REAL-9 closed with audit required and PROCEED.
<!-- DATA: `handover/evidence/dev_self_hosting/dev_1778842938421_1788018/DevRunSummary.json` records `close_status=closed`, `acceptance_passed=true`, `audit_required=true`, `audit_verdict=PROCEED`, `effective_risk_class=4`, and `restricted_surface_hits=[]`. -->

The clean-context implementation audit found no production defects for REAL-8/REAL-9.
<!-- DATA: `handover/audits/CODEX_REAL8_REAL9_IMPLEMENTATION_REVIEW.md` has `Verdict: PROCEED` and states `No production defects found in the reviewed REAL-8/REAL-9 surfaces.` -->

The verification record is broad enough for handover, including formatting, targeted tests, Trust Root, REAL-8 benchmark, constitution gates, workspace tests, and audit.
<!-- DATA: `handover/ai-direct/LATEST.md` validation lists `cargo fmt --all -- --check exit 0`, targeted REAL-8/REAL-9 tests REAL-8 6/0 and REAL-9 2/0, Trust Root 1/0, REAL-8 benchmark exit 0, constitution gates `458 passed / 0 failed / 1 ignored`, workspace test exit 0, and clean-context Codex review PROCEED. -->

The R-022 skip was a deliberate cleanup exception, not an architectural waiver.
<!-- DATA: `handover/alignment/OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md` states the hook correctly blocked the first commit because the staged package had many new public symbols and removed backlinks; it says this is not a policy relaxation and future atoms should carry immediate TRACE_MATRIX backlinks or §J registration. -->

## 9. Current Risks And Open Questions

Risk 1: E2 and E3 are still unproven.
<!-- DATA: REAL-5S reports explicitly do not claim E2 or E3; REAL-7 says no E2/E3 spontaneous emergence claim from scripted fixture activity; REAL-8 is descriptive and does not claim causality. -->

Risk 2: REAL-8 sample size is too small for performance conclusions.
<!-- DATA: REAL-8 FINALZ has `task_count=3` for each arm; solve_rate is `2/3` across A/B/C/D. -->

Risk 3: REAL-6B live real-LLM AttemptPrediction is still gated.
<!-- DATA: Architect original and REAL6B fixture report both state no live real-LLM ship until explicit Class-4 ratification. -->

Risk 4: R-022 TRACE_MATRIX debt must be paid before further public API expansion.
<!-- DATA: OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md forward action requires an explicit TRACE_MATRIX backlink pass over REAL-5 / REAL-6 public API surfaces before the next feature atom. -->

Risk 5: The benchmark currently shows more market activity but no solve-rate separation.
<!-- DATA: REAL-8 FINALZ market_tx_count A=0, B=4, C=10, D=10; solve_rate A=B=C=D=`2/3`. -->

Risk 6: Some current market activity is scripted, so it cannot be used as spontaneous emergence evidence.
<!-- DATA: REAL-7 planned evidence command includes scripted flags for task-outcome buys, AttemptPrediction fixture, and verify/challenge; REAL-7 claim boundary rejects E2/E3 spontaneous emergence. -->

Risk 7: Reports and dashboards are useful but remain materialized views.
<!-- DATA: AGENTS.md truth order ranks ChainTape + CAS above dashboards/reports; REAL-7 report and REAL-9 manual both state dashboards/reports are materialized views and ChainTape/CAS remains authoritative. -->

## 10. Codex Opinion

My first opinion is that the project should not return to prompt-only market activation as the main line.
<!-- DATA: REAL5_CLEAN_NEGATIVE_NO_TRADE_REPORT.md says prompt-only exhausted; architect original says stop prompt-only variants; REAL-5 variants with role order, market K, and adversarial tasks all stayed at 0 router buys. -->

My second opinion is that the next scientific question should be "does earlier, lawful market timing create measurable behavior differences under controlled budgets?" rather than "can we make a trade happen?"
<!-- DATA: REAL-6A creates TaskOutcomeMarket before WorkTx; REAL-8 shows market_tx_count increases in C/D while solve_rate remains unchanged at 2/3; REAL-8 explicitly defines metrics including solve rate, verified PPUT, false accept rate, cost per verified proof, market tx count, NoTradeReason distribution, PnL dispersion, role diversity index, and audit failure rate. -->

My third opinion is that REAL-7 is the right demonstration for architecture and REAL-8 is the right demonstration for science.
<!-- DATA: REAL-7 meets structural pressure criteria with buys, verify, challenge, event resolve, PnL, and autopsy satisfaction; REAL-8 pins same inputs across A/B/C/D and rejects causal overclaim. -->

My fourth opinion is that live REAL-6B should not be merged into the next experiment unless the architect explicitly ratifies it as Class 4.
<!-- DATA: REAL-6B report and architect original both state current REAL-6B is design plus scripted fixture only and no live real-LLM ship until explicit Class-4 ratification. -->

My fifth opinion is that the next engineering cleanup should be TRACE_MATRIX/R-022 hardening, not another market feature.
<!-- DATA: The ship needed `OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md`; that OBS says it is a one-time bulk cleanup exception and future atoms should use immediate backlinks or §J rows. -->

## 11. Recommended Next Steps

Recommendation 1: Ask the architect to ratify the current bundle under a narrow claim: role scaffold plus lawful pressure plus descriptive A/B, not emergence.
<!-- DATA: Matrix §S claim boundary and REAL-8 report both reject emergence/causal claims; LATEST Next Step 1 says give the REAL-5S -> REAL-9 bundle and REAL-8 report to the architect. -->

Recommendation 2: Before any new feature atom, run a TRACE_MATRIX backlink pass over REAL-5/REAL-6 public APIs.
<!-- DATA: OBS_R022_REAL5S_REAL9_BULK_SHIP_2026-05-15.md forward action explicitly requires this before opening the next feature atom. -->

Recommendation 3: If the architect wants more empirical evidence, run a larger REAL-8 benchmark with the same pinned-input discipline.
<!-- DATA: LATEST Next Step 2 says larger multi-model REAL-8 variants should preserve the same pinned-input discipline; REAL-8 SG-8.1, SG-8.2, and SG-8.3 require same problem set, same model assignment, and same budgets. -->

Recommendation 4: Expand REAL-8 in stages: first 10 to 15 tasks per arm, then 30 tasks per arm only if the first extension remains audit-clean.
<!-- DATA: Current REAL-8 evidence uses 3 tasks per arm; the workspace has previous scale ladder practice in LATEST older sections, but this recommendation is conservative because current A/B has no solve-rate separation and should not jump directly to launch claims. -->

Recommendation 5: Keep the same core arms A/B/C/D, but add a recorded seed/model-family manifest if moving beyond single-assignment evidence.
<!-- DATA: REAL-8 current arms are A market disabled, B market visible no TaskOutcomeMarket, C TaskOutcomeMarket enabled, D TaskOutcomeMarket plus scripted AttemptPrediction; G4.2/REAL route history requires replayable model identity and no hidden switch, and REAL-8 pins one model assignment hash. -->

Recommendation 6: Treat spontaneous market action as a separate E2 gate, with a strict definition: live agent-generated router buy or short, no forced trade, ChainTape/CAS-visible, audit PROCEED.
<!-- DATA: REAL-5S explicitly did not claim E2; REAL-7 scripted buys cannot count as spontaneous emergence; REAL-9 manual forbids forced trades and requires ChainTape/CAS-backed market work. -->

Recommendation 7: Treat E3 role differentiation as a later gate requiring persistent differences across roles/families and PnL, not a dashboard label.
<!-- DATA: REAL-5S rejects E3; REAL-8 measures role_diversity_index but does not claim role differentiation; G4.2/REAL route claim boundaries require model/role attribution from evidence, not reports as source of truth. -->

Recommendation 8: Keep REAL-6B live real-LLM AttemptPrediction sealed until the architect issues explicit Class-4 ratification.
<!-- DATA: Architect original says `REAL-6B = design + scripted fixture only. No live real-LLM ship until explicit Class-4 ratification.` -->

Recommendation 9: Preserve negative results as first-class evidence.
<!-- DATA: REAL-5S clean negative documents NoPool/post-accept timing/prompt-only exhaustion; REAL-8 SG-8.6 says negative result is valid and documented; REAL-8 report retains same solve_rate across arms instead of rewriting the conclusion. -->

## 12. Architect Discussion Questions

Question 1: Is the current REAL-5S -> REAL-9 bundle acceptable as "lawful pressure scaffold shipped" with no emergence claim?
<!-- DATA: All route rows are GREEN in Matrix §S, but Matrix and reports explicitly narrow claims. -->

Question 2: Should the next experimental step be a larger REAL-8 A/B, or should the team first ratify live REAL-6B?
<!-- DATA: REAL-8 is currently descriptive with only 3 tasks per arm; REAL-6B live path is explicitly Class-4 gated. -->

Question 3: What minimum E2 criterion should the architect require before saying "spontaneous trading emergence"?
<!-- DATA: Current evidence has REAL-5 Trader buy=0 and REAL-7 scripted buys; neither supports E2. -->

Question 4: What minimum E3 criterion should the architect require before saying "role differentiation"?
<!-- DATA: REAL-8 records role_diversity_index=5 in each arm but does not claim persistent role differentiation or causality. -->

Question 5: Should R-022 backlink cleanup be a mandatory next atom before any further market code?
<!-- DATA: The current ship used a one-time OBS_R022 skip due bulk public surface; OBS forward action recommends backlink pass before the next feature atom. -->

## 13. Bottom Line

The project now has a lawful, chain-backed generative market scaffold: roles, typed gateways, TaskOutcomeMarket, scripted AttemptPrediction fixture, ChainTape-derived PnL, observe-only scheduler signals, structural pressure smoke, and a formal A/B benchmark.
<!-- DATA: REAL-5S report, REAL-6A/B/C/D reports, REAL-7 report, REAL-8 report, REAL-9 docs, Matrix §S GREEN rows, and TB_LOG shipped row collectively record these components. -->

The project does not yet have proof of spontaneous market emergence or causal performance gain from markets.
<!-- DATA: REAL-5S says Trader buy=0 and NoPool dominates; REAL-7 buys are scripted; REAL-8 solve_rate is 2/3 in all arms and report says no causality. -->

The right next move is a disciplined architect decision: either ratify the scaffold and run a larger pinned REAL-8, or ratify live REAL-6B as a separate Class-4 experiment. In either case, do not re-enter prompt-only tuning as the main strategy.
<!-- DATA: LATEST Next Steps recommend architect review, larger REAL-8 only with pinned-input discipline, and REAL-6B live gating; REAL-5S clean-negative says prompt-only exhausted. -->
