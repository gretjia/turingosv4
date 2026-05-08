# CODEX Stage C Polymarket Charter Ratification Audit

## 1. Header

- auditor: Codex
- date: 2026-05-07
- target: `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md`
- gate: Gate 1 strict pre-execution charter ratification
- HEAD: `1e0c97c61d18dfffe573d9c381b432afc1711a3f`
- scope: charter ratification only; companion-doc cross-checks and prerequisite source/test existence checks only; no cargo/make/script execution
- authority: parent authorization `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md` says Stage C is gated on Stage A green and Stage B1 green at lines 131-135, lists P-M0..P-M9 authorization classes at lines 137-148, and preserves Class-4 STEP_B plus per-atom sign-off at lines 199-203.

## 2. Executive Summary

Aggregate verdict: VETO. The 8-question count is 1 PASS / 7 CHALLENGE / 0 VETO, but the sanity check is VETO because the charter downgrades Stage A3 HEAD_t C2 from the Stage A green hard gate into a recommendation before P-M2 STEP_B. That is not ratifiable for a Class-4 market stage that will add typed-tx/sequencer/CAS state and then rely on replayable G2 evidence. The prerequisite source claims checked out: `CompleteSetMintTx`, `CompleteSetRedeemTx`, and `MarketSeedTx` exist in `src/state/typed_tx.rs:1171`, `src/state/typed_tx.rs:1202`, and `src/state/typed_tx.rs:1233`, with `TypedTx` variants at `src/state/typed_tx.rs:1542-1544`; the legacy CPMM fence guards imports at `tests/tb_13_legacy_cpmm_forward_fence.rs:435-464` and asserts `src/prediction_market.rs` absence at `tests/tb_13_legacy_cpmm_forward_fence.rs:573-584`; and `price_never_overrides_predicate` exists at `tests/constitution_predicate_gate.rs:169-205`.

## 3. Per-Question Verdicts

### Q1 - Per-Phase Class Classification Correctness

Verdict: CHALLENGE

Reasoning: The charter's top-level class block mostly states the intended mixed-class envelope: P-M0 is Class 1, P-M1/P-M3/P-M5/P-M7/P-M8 are Class 3, and P-M2/P-M4/P-M6 are Class 4 STEP_B on `typed_tx.rs`, `sequencer.rs`, and `cas/schema.rs` at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:20-24`. However, the body reintroduces ambiguity: P-M4 is titled "Class 4 STEP_B if typed_tx surface" at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:109`; the atom sequence says `P-M4 (Class 3 / 4 STEP_B if typed_tx)` and downgrades P-M7/P-M8 to Class 2 / Class 1-2 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:239-242`. CR-StageC-PM.15 also scopes STEP_B to atoms "touching" the three restricted files at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:183`, while CR-StageC-PM.16 says "P-M4 if needed / P-M6 if needed" at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:184`. Per `/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_class4_cannot_hide_in_class3.md:7-15` and `/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_class4_cannot_hide_in_class3.md:21-23`, typed-tx schema or sequencer admission work is Class 4 and requires separate ratification; hedging is itself the smell this memory warns against.

Cited evidence: charter classification `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:20-24`; P-M4 hedge at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:109`; STEP_B/if-needed language at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:183-184`; atom-sequence inconsistency at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:239-242`; CLAUDE STEP_B restricted files and Class-4 candidate rule at `CLAUDE.md:604-614`; class-hide feedback at `/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_class4_cannot_hide_in_class3.md:7-15`.

Remediation: Make P-M4 unconditionally Class 4 STEP_B if the atom introduces canonical pool state, typed-tx admission, CAS object schema, or sequencer mutation; otherwise split it into a Class-3 derived-view atom plus a separate Class-4 pool-state atom. Harmonize P-M7/P-M8 classification across §0 and §7.

### Q2 - Atom Sequencing vs CR-StageC-PM.11 / CR-StageC-PM.12

Verdict: PASS

Reasoning: CR-StageC-PM.11 is internally satisfied: the atom sequence puts P-M1 before P-M4 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:239-240`, and CR-StageC-PM.11 explicitly says P-M1 ships before P-M4 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:179`. The apparent CR-StageC-PM.12 conflict is resolved by the CR text itself: `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:180` says "P-M8 ships before any non-smoke external use," not before internal implementation or phase tests. This allows P-M5 share-only swap and P-M6 router code to be implemented before P-M8, while still requiring P-M8 before P-M9 controlled smoke or any non-smoke external trading. The architect manual has the same order: share-only swap before router at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:823-861`, audit tools later at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:950-976`, and controlled smoke "Only after all above" at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:978-980`.

Cited evidence: charter CR-StageC-PM.11/12 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:179-180`; atom sequence at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:239-245`; manual P-M5/P-M6/P-M8/P-M9 order at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:823-861`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:863-928`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:950-980`.

Remediation: Optional clarity edit only: add after charter line 245 that P-M5/P-M6 are internal implementation and test atoms until P-M8 ships; external/non-smoke trading remains forbidden before P-M8.

### Q3 - Universal Forbidden List Completeness

Verdict: CHALLENGE

Reasoning: The six universal forbidden items are present in §6 at charter lines 206-213 and all six have at least a CR anchor: no f64 to CR.1 line 169, no ghost liquidity to CR.2 line 170, no price-as-truth to CR.3/CR.8 lines 171 and 176, no dashboard source-of-truth to CR.4 line 172, no public chain to CR.13 line 181, and no real money/funds to CR.14 line 182. The challenge is test pinning. SG-StageC-PM.4 only says "grep-style tests in `tests/tb_18d_*`" at line 196; only f64/no-import test files are named in FR-PM0.2/0.3 at lines 79-80. There is no charter-named universal grep test for real funds, public chain, dashboard source-of-truth, or price-to-verdict shortcuts. The existing predicate-price guard is real (`tests/constitution_predicate_gate.rs:169-205`), but the charter does not pin the full universal list to specific Stage C tests.

Cited evidence: universal list at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:206-213`; CR anchors at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:169-182`; SG-StageC-PM.4 generic test pointer at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:196`; only named universal grep files at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:79-80`; parent universal list at `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:157-169`; existing predicate-price test at `tests/constitution_predicate_gate.rs:169-205`.

Remediation: Add a forbidden-list audit map in §6 or §5 that enumerates each universal item, its CR, and exact test name. Real funds/public chain can be grep tests over Stage C modules plus directive forward checks; dashboard source-of-truth should pin to `dashboard_regenerates_market_view` and a chain/CAS regeneration assertion.

### Q4 - Polymarket-Specific 11-Item Forbidden List

Verdict: CHALLENGE

Reasoning: The charter copies the 11 Polymarket-specific forbidden items from the architect manuals. The English manual lists them at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:1007-1021`; the Chinese manual lists the same set at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md:834-850`; the charter mirrors them at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:214-225`. The challenge is again pinning. Some items are well covered by FRs/tests: automatic per-node YES/NO injection is prohibited by FR-PM3.3/3.4 at lines 106-107; f64 is covered by FR-PM0.3/P-M5/P-M6 tests at lines 80, 124, and 134; price settlement is covered by FR-PM7/P-M9 at lines 140-143 and 159-163. But Treasury magic seed, agent-submitted MarketResolveTx, agent-submitted system resolution, public chain, and real money remain declarative CRs without exact Stage C test names. CR-StageC-PM.9/10 exist at lines 177-178, but no test name is assigned to fail on agent-authored MarketResolve/system resolution.

Cited evidence: manual forbidden list in English at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:1007-1021`; manual forbidden list in Chinese at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md:834-850`; charter forbidden list at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:214-225`; CR anchors at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:173-182`; FR/test anchors at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:80`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:105-107`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:124`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:134`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:140-143`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:159-163`; SG-StageC-PM.5 generic pointer at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:197`.

Remediation: Add exact tests for the declarative-only forbidden items, especially `agent_cannot_submit_market_resolve_tx`, `agent_cannot_submit_system_resolution_tx`, `treasury_seed_requires_balance_debit`, `no_public_chain_stage_c`, and `no_real_money_stage_c`.

### Q5 - 1 Coin = 1 YES + 1 NO Conservation

Verdict: CHALLENGE

Reasoning: The core CTF rule is present and aligns with the constitution and current code. CLAUDE §13 states "1 Coin = 1 YES + 1 NO" and that YES/NO shares are claims, not Coin at `CLAUDE.md:618-640`; constitution Law 2 states the same CTF conservation at `constitution.md:155-160`. The charter tests mint conservation and shares-not-Coin in P-M1 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:88`, merge conservation in P-M2 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:97`, MarketSeed conservation/no-ghost in P-M3 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:105`, and end-to-end total coin conservation in P-M9 at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:163`. Source support is real: `conditional_share_balances_t` is explicitly not Coin in `src/state/q_state.rs:228-239` and `src/state/q_state.rs:585-612`; total supply includes conditional collateral but omits shares in `src/economy/monetary_invariant.rs:202-215`; `assert_complete_set_balanced` states the CTF core at `src/economy/monetary_invariant.rs:445-467`.

Challenge: not every share-creating phase has an explicit "shares not counted as Coin" test. P-M6 mints a complete set inside the router at charter lines 130-134 and manual lines 871-884, but FR-PM6.5 lacks a named router total-coin conservation or router shares-not-Coin test. P-M3 has `market_seed_conserves_total_coin`, but not an explicit `market_seed_shares_not_counted_as_coin` test. P-M4 does cover pool reserves/LP shares not counted as Coin at charter line 115.

Cited evidence: charter P-M1/P-M2/P-M3/P-M4/P-M6/P-M9 requirements at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:86-89`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:95-98`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:104-107`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:113-115`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:130-134`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:159-163`; CLAUDE economy law at `CLAUDE.md:618-640`; constitution Law 2 at `constitution.md:155-160`; share-not-Coin source at `src/state/q_state.rs:228-239`, `src/state/q_state.rs:585-612`; monetary invariant at `src/economy/monetary_invariant.rs:202-215`, `src/economy/monetary_invariant.rs:445-467`.

Remediation: Add explicit P-M3 and P-M6 tests named `market_seed_shares_not_counted_as_coin`, `buy_yes_with_coin_conserves_total_coin`, `buy_no_with_coin_conserves_total_coin`, and `router_minted_shares_not_counted_as_coin`.

### Q6 - Integer Math Invariant

Verdict: CHALLENGE

Reasoning: The formulas and invariant are stated in the charter and match the architect manual. P-M5 states `poolY1 * poolN1 >= poolY * poolN` and names constant-product tests at charter lines 121-124, matching manual lines 827-861. P-M6 states the same invariant at charter line 133, matching manual lines 892-903; the P-M6 formula test list matches manual lines 919-927. The no-float direction is also present: FR-PM0.3 names a no-f64/no-f32 grep test at charter line 80; P-M5/P-M6 include no-f64 tests at lines 124 and 134; current `MicroCoin` is an integer newtype around `i64` with checked arithmetic expectations in `src/economy/money.rs:1-17`.

Challenge: this is currently specified as formula tests plus grep discipline, not type-level enforcement for all pool math. The charter does not require checked wide multiplication for `poolY * poolN`, does not name a P-M6 invariant test such as `router_constant_product_non_decreasing`, and only names `buy_yes_no_f64` rather than symmetric router no-float coverage. The invariant is testable, but the charter should make it a recurring assertion on every swap/router transaction, not just an implied consequence of formula tests.

Cited evidence: charter no-f64 and invariant requirements at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:80`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:121-124`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:130-134`; manual P-M5/P-M6 formulas at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:827-861`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:871-903`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:916-927`; integer money type at `src/economy/money.rs:1-17`; legacy no-f64 fence baseline at `tests/tb_13_legacy_cpmm_forward_fence.rs:521-550`.

Remediation: Add mandatory checked-math language for pool formulas (`checked_mul` / widened integer / overflow rejection) and add explicit P-M6 invariant tests for both router directions.

### Q7 - Per-Phase SG Falsifiability and Verbatim FR-PM Mapping

Verdict: CHALLENGE

Reasoning: Most charter test names match the authoritative English manual §7. P-M1 matches manual lines 699-706; P-M3 matches lines 782-786; P-M4 matches lines 817-820; P-M5 matches lines 855-860; P-M6 is verbatim for all 9 named tests at charter line 134 and manual lines 919-927; P-M7 matches lines 944-947; P-M8 test names match lines 973-975. There are still falsifiability gaps. First, the P-M2 final test name is not verbatim: manual line 750 says `merge_unavailable_after_final_redeem_if shares exhausted`, while charter line 97 says `merge_unavailable_after_final_redeem_if_shares_exhausted`. This may be a manual typo, but the charter should explicitly normalize it. Second, manual P-M8 says audit tools must show conditional collateral at lines 961-968; charter P-M8 lines 149-153 omit conditional collateral. Third, SG-StageC-PM.9 line 201 is under-specified for a Class-3 evidence atom: manual P-M9 gates require no ghost liquidity, total coin conserved, no price-as-truth, no raw log broadcast, and replayability at lines 997-1003, while charter line 201 only names FC1, economic conservation, and price-not-truth.

Cited evidence: charter FR test names at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:88-89`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:97`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:105`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:115`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:124`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:134`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:143`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:153`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:159-163`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:193-201`; manual test/gate names at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:696-707`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:743-751`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:779-787`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:814-821`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:852-861`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:916-927`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:941-947`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:961-976`, `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:997-1003`.

Remediation: Add a per-phase SG table to the charter instead of relying only on manual references. Normalize the P-M2 test name with an explicit note, add conditional collateral to FR-PM8, and expand SG-StageC-PM.9 to include the full five manual P-M9 gates plus exact evidence manifest fields/commands.

### Q8 - Class-4 Hide-in-Class-3 and Per-Atom §8 Boundaries

Verdict: CHALLENGE

Reasoning: The charter has the right skeleton: CR-StageC-PM.15 requires STEP_B per touching atom at line 183; CR-StageC-PM.16 forbids bundling typed-tx schema bumps and requires per-atom architect §8 sign-off at line 184; SG-StageC-PM.8 repeats per-Class-4-atom sign-off at line 200; overall Stage C §8 sign-off is separate at lines 247-253. The parent authorization independently says Class-4 surfaces still require STEP_B and per-atom sign-off at `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:199-203`, and explicitly says typed-tx schema bumps without STEP_B are not granted at lines 216-224.

Challenge: the same "if needed" language weakens the boundary. P-M4 is hedged in §3.5 and §7 at charter lines 109 and 240; P-M6 is titled "Class 4 STEP_B if typed_tx surface" at line 126 even though §0 line 23 declares it Class 4 on the restricted files. The charter also does not explicitly state that P-M2's `CompleteSetMergeTx` typed-tx bump is limited to that one variant and cannot pre-authorize `CpmmPool`, router txs, or later CAS schema in P-M4/P-M6. Since `CompleteSetMergeTx`, `CpmmPool`, and router txs are absent in current restricted surfaces (no hits for those names in `src/state/typed_tx.rs`, `src/state/sequencer.rs`, or `src/bottom_white/cas/schema.rs`), each future addition needs its own STEP_B evidence and sign-off.

Cited evidence: charter Class-4 boundaries at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:23`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:95-99`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:109`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:126-134`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:183-184`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:199-201`, `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:239-253`; parent Class-4 preservation at `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:199-203`, `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:216-224`; feedback rule at `/home/zephryj/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_class4_cannot_hide_in_class3.md:19-29`.

Remediation: Add explicit per-atom write boundaries: P-M2 may add only `CompleteSetMergeTx`; P-M4 may add only pool state/schema/admission required for `CpmmPool`; P-M6 may add only router tx/schema/admission. State that P-M2 STEP_B cannot authorize P-M4/P-M6 schema or signing payload changes.

## 4. Sanity Check

### §1 Scope, §2 Pre-Conditions, §6 Forbidden List, §7 Atom Sequence vs §4/§5 Ship Gates

Verdict: VETO

Reasoning: §1 scope, §4 CRs, §6 forbidden list, and §7 atom sequence are directionally coherent, but §2 has a hard-gate defect. The charter says Stage A green is satisfied by TB-18R Final, AMBER closure progress, and HEAD_t C1, while HEAD_t C2 is only "RECOMMENDED before P-M2 STEP_B work" at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:65-68`. The parent authorization says Stage C is gated on Stage A green and Stage B1 green at `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:131-135`; Stage A includes A3 HEAD_t C2 as a Class-4 STEP_B item at lines 115-122. The architect manual says C2 remains future at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:147-154` and defines A3 required refs and replay gates at lines 380-402. The A3 charter states C1 has L4 commits but L4.E and CAS roots are still in-memory at `handover/tracer_bullets/STAGE_A3_HEAD_T_C2_charter_2026-05-07.md:40-45`, while C2 requires L4/L4.E/CAS refs and replay reconstruction from refs alone at lines 47-68. Running P-M2 STEP_B on C1 and migrating to C2 mid-Stage-C would make P-M2/P-M6/P-M9 evidence depend on a pre-migration substrate, exactly where SG-StageC-PM.7 says G2 happens after substrate green at charter line 199. For a Class-4 market stage, "recommended" is not sufficient.

Remediation: Make Stage A3 HEAD_t C2 FINAL GREEN a hard precondition before any P-M2/P-M4/P-M6 STEP_B and before P-M9 evidence. If the architect wants P-M0/P-M1 preparatory work before C2, carve that out explicitly as non-Class-4, non-external, non-final preparation and block Class-4 atoms until C2 sign-off.

## 5. Aggregate Verdict

Aggregate verdict: VETO

The 8-question audit found no missing prerequisite source objects and no direct per-question VETO, but the sanity check is a hard gate failure: the charter weakens Stage A3 HEAD_t C2 from a Stage A green requirement into a recommendation before Class-4 market work. With seven CHALLENGEs and a sanity-check VETO, this charter should not be treated as G1-ratified for execution. After the hard-gate and text remediations below, the core Stage C design is recoverable without source changes.

VETO blocker quote: charter §2 says "HEAD_t C2 (Stage A3) RECOMMENDED before P-M2 STEP_B work" at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:67`, while parent authorization gates Stage C on Stage A green at `handover/directives/2026-05-07_ARCHITECT_ALIGNMENT_AUTONOMOUS_EXECUTION_AUTHORIZATION.md:131-135`.

## 6. Top-3 Remediations

1. Change §2 line 67 from "HEAD_t C2 (Stage A3) RECOMMENDED before P-M2 STEP_B work to avoid ledger storage-form change mid-feature" to "HEAD_t C2 (Stage A3) FINAL GREEN is HARD-required before any P-M2/P-M4/P-M6 STEP_B work and before P-M9 evidence; P-M0/P-M1 preparatory tests may proceed only if they do not touch Class-4 surfaces or claim Stage C ship progress."

2. Change §7 lines 239-242 and §3.5/§3.7 titles so classifications are single-valued: `P-M4 (Class 4 STEP_B + §8 sign-off if any canonical pool state/typed-tx/sequencer/CAS surface is added; otherwise split out as derived-view Class 3)`; `P-M6 (Class 4 STEP_B + §8 sign-off)`; `P-M7/P-M8` must match the chosen charter classification everywhere. Add after CR-StageC-PM.16 line 184: "P-M2 STEP_B authorizes only CompleteSetMergeTx; it does not authorize P-M4 pool schema or P-M6 router schema/signing/admission changes."

3. Add after §6 line 234 a forbidden-list audit map with columns `Forbidden item | CR | exact test(s) | phase`, and add the missing named tests for real funds, public chain, Treasury seed debit, agent-submitted MarketResolveTx/system resolution, router conservation, and router shares-not-Coin. Change FR-PM8 lines 149-153 to include conditional collateral, and change SG-StageC-PM.9 line 201 to enumerate the manual P-M9 gates: no ghost liquidity, total coin conserved, no price-as-truth, no raw log broadcast, and all activity replayable.

## 7. Open Questions / Hypotheses

- Unverified: future Stage C test files named `tests/tb_18d_*` are charter targets and were not expected to exist before execution. I did not assert they pass.
- Unverified: `CompleteSetMergeTx`, `CpmmPool`, `BuyYesWithCoinRouter`, and `BuyNoWithCoinRouter` are not present in current `typed_tx.rs`/`sequencer.rs`/`cas/schema.rs`; that is expected for a before-execution charter, but it reinforces the need for independent per-atom STEP_B.
- Unverified: I did not run `cargo test`, constitution gates, or scripts, per the action-safety rule.
- Hypothesis: the manual P-M2 test name `merge_unavailable_after_final_redeem_if shares exhausted` at `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md:750` is likely a formatting typo; the charter should either quote it verbatim with a normalization note or record the corrected test name as an explicit charter normalization.
- Hypothesis: `SG-StageC-PM.6` and `.7` still use `TB_18D` audit/evidence naming at `handover/tracer_bullets/STAGE_C_POLYMARKET_PM0_PM9_charter_2026-05-07.md:198-199`, while the charter naming note says Stage C has no TB ID at lines 3-7. This should be corrected to `STAGE_C_POLYMARKET` paths before execution evidence is generated.
