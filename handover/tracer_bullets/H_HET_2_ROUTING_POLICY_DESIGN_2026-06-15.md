# H-HET-2 Routing-Policy Design — recommendation for architect review (2026-06-15)

> **SUPERSEDED BY THE RULING.** Architect ruled APPROVED AS AMENDED →
> `VERIFY_UCB_PRICE_PRIOR_FLOOR_V1`. Authoritative contract:
> `H_HET_2_ROUTING_POLICY_RULING_2026-06-15.md`. **Amendment-1 correction:** any
> phrasing below implying "H-HET-1 already showed price aggregation misroutes" is an
> OVERCLAIM — H-HET-1 only proved fixed round-robin STARVES the uniquely capable model
> (it never ran a price allocator); aggregate price is a wrong-granularity *proxy
> risk*, not a falsified mechanism.

> Produced by the het2-routing-policy-design workflow (wf_bb967936; 4 proposers + 3 judges + synthesis). DESIGN ONLY — nothing implemented. The judges SPLIT (charter-fit→UCB; constitution-fit + implementability→PRICE_PROP), so the policy choice is an architect decision. Paid run remains BLOCKED.

## Judge rankings (the split)

- **charter-fit (does the design convert H-HET-1's complementary-coverage signal int…** best=`ucb-bandit` rank=['ucb-bandit', 'thompson-beta', 'priced-softmax-reuse', 'price-proportional (PRICE_PROP)']
  - ucb-bandit — STRONGEST on the actual lever. Reward = per-(model,target) Lean Verify{true} (the SAME predicate the §4 headline scores on), so the routing signal IS complementary coverage at the target granularity the charter defines (DS→{det_zero,det_3x3}, Q397→{det_2x2}). Goodhart-immune by construction (metric == goal, Art III.4). Price is a bounded W_PR refinement, never the primary signal, so it cannot route budget AWAY from the niche specialist the way PRICE_PROP does. Integer LOG_TABLE + isqrt is honestly motivated (f64 sqrt breaks §5.4 byte-equality); no name-lie exposure (UCB is determi

- **constitution-fit — alignment with Art II.2 (price broadcast drives allocation, n…** best=`price-proportional (PRICE_PROP)` rank=['price-proportional (PRICE_PROP)', 'ucb-bandit', 'priced-softmax-reuse', 'thompson-beta']
  - PRICE_PROP — STRONGEST constitution-fit (best on this lens). Art II.2: literally "the priced market IS the routing signal" — score_raw[m] aggregates compute_price_index price_yes (other agents'/bears' stakes, not a self-reported field), so price broadcast directly drives allocation with zero micro-manage; this is the purest reading of Art II.2 "price drives emergence." Art II.2.1: ε-floor is an ADDITIVE integer term (eps_num,eps_den)=(min(10k,40),100k) guaranteeing W[m]/total ≥ ε for every exploring model — explicitly the BOLTZMANN_MIN_EPSILON anti-collapse property (price_index.rs:293-307) it

- **implementability (Karpathy-simple + reuses existing substrates + integer-math + …** best=`price-proportional (PRICE_PROP)` rank=['price-proportional (PRICE_PROP)', 'priced-softmax-reuse', 'ucb-bandit', 'thompson-beta']
  - PRICE_PROP — 9/10. The only family whose entire money/budget decision path is expressible with arithmetic THAT ALREADY EXISTS in price_index.rs: per-model aggregation is `(num*SCALE)/den` with `saturating_mul/add` (verified idioms at price_index.rs:96-104), selection is `rng.gen::<u64>() % total_W` over the carrier's existing seeded StdRng (line 1859), exploration floor is an integer-rational additive term reusing the BOLTZMANN_MIN_EPSILON concept (price_index.rs:298-306). NO new transcendental/fixed-point primitive. `derive_from_tape` byte-equality (§5.4) is trivially satisfiable because ever


## RECOMMENDED: ucb-bandit (integer-UCB model-budget router) — grafted with PRICE_PROP's bounded price term as a secondary modulator and with classic UCB1's ln/sqrt replaced by an isqrt-only count-bonus to delete the riskiest new primitive.


**Why:** The decisive lens is charter-fit (§2: "dynamic budget routing converts complementary coverage into capability BestHOMO cannot match"). ucb-bandit is the ONLY family whose routing signal IS that lever: its reward is the per-(model,target) Lean Verify{true} verdict — literally the §4 headline predicate at the granularity the charter defines complementary coverage (DS→{det_zero,det_3x3}, Q397→{det_2x2}). PRICE_PROP, the runner-up on the other two lenses, structurally MISROUTES the lever: aggregate price_yes summed over a model's nodes rewards a prolific generalist that is cheaply-right on many easy nodes over the specialist that is the ONLY model able to crack the hard target — it can fund the model BestHOMO already represents (its own weakness #2 concedes this), and cold-start all-zero-price degenerates to uniform = CONTROL-2 exactly in the experiment's budget regime. Decisive second factor: §17.3 name-fidelity. The live gate tests/constitution_router_name_matches_mechanism.rs REDs a distribute-named router (softmax/thompson) that collapses to argmax — the documented 2026-06-01 name-lie that produced wrong PROVEN headlines and passed every gate. priced-softmax-reuse and thompson-beta both ARM that gate by name AND place a brand-new unvalidated integer stochastic sampler (exp_fixed / Q32.32 Beta LUT) on the money path that their OWN weaknesses admit can silently behave like ε-greedy. ucb-bandit's named mechanism (deterministic argmax-over-integer-score + ε-floor) MATCHES its behavior with zero distribute-naming exposure, and its reward == the scored predicate (no Goodhart gap, Art III.4). Its two real costs are (a) new integer numeric primitives and (b) UCB asymptotics don't fit tx/agent≈20. I neutralize (a) by grafting: keep PRICE_PROP's price as the bounded W_PR modulator (so price still drives per Art II.2 but cannot override kernel-verify evidence), and replace UCB1's √(2 ln t / n) with a count-based isqrt-only bonus — eliminating the LOG_TABLE, the one primitive the constitution judge feared could collapse the bonus to constant. Net new money-path primitive = a single integer isqrt (Newton, u128), fully testable for monotonicity + byte-determinism, satisfying §5.4 derive_from_tape byte-equality (an f64 sqrt would break it). (b) is a BUDGET-REGIME risk shared by every family and an explicit §3 pilot obligation, not a design DQ; the price modulator + N=3 decommission steer budget fast within small budgets, which is exactly when UCB asymptotics are weakest.


**Algorithm:**

UCB-BUDGET-ROUTER. Replaces the static round-robin `agent_models[ai] = models[i % len]` (lean_market_agent.rs:1728) with a per-(round r, agent ai, target T) integer model pick on a NEW Policy::UcbBudget arm; all other arms keep the static rule so the §17.3 substrate refs (compute_price_index + boltzmann_softmax_select_parent for NODE routing) stay untouched.

ROSTER. eligible_models = the FROZEN deduped roster M (charter §2 errata#4), |M|=k. NOT the n_agents round-robin expansion — the bandit routes the roster SET, and the produced ProposalTelemetry.model_id (put_proposal, line 732) carries the chosen model so cost stays tape-canonical.

PER-(MODEL,TARGET) BANDIT STATE — all integer, all recomputed FROM THE TAPE each tick (never memory-canonical, Art 0.2). For each m∈M on target T:
  pulls[m]   = # prior ticks this run that selected m on T (counted from prior BudgetAllocationTelemetry events for T).
  verifies[m]= # of those pulls whose paired VerificationResult.verified==true (read via ProposalTelemetry.verification_result_cid → VerificationResult; model attribution via ProposalTelemetry.model_id).
  hardfail_streak[m] = trailing run of consecutive HARD failures since m's last reward/non-hard event on T. HARD = VerificationResult.verified==false with reject_class ∈ frozen hard set {lean-reject, unsolved_goals, type-mismatch, axiom-rejected, sorry-blocked}; EXCLUDES parse_fallback/provider/timeout/rate-limit/tool-infra (§8/errata#5 — infra noise must not burn the floor). Reset to 0 on any reward.
  decommissioned[m] = hardfail_streak[m] >= N (N=3).

PRICE SIGNAL (bounded modulator, the PRICE_PROP graft). pi = compute_price_index(&q.economic_state_t) (line 1856). price_signal_micro[m] = integer-rational mean YES-price of m's own attempt-nodes folded to per-million = (Σ_{n∈nodes(m)} price_yes.num * SCALE / price_yes.den) / max(1,n_nodes(m)), SCALE=1_000_000, u128 saturating (the price_index.rs cross-multiply discipline). Models with no priced node → 0.

PER-MODEL SCORE (integer, i128; no f64). Let global_pulls = Σ_m pulls[m], n_m = pulls[m].
  1. mean_verify_micro[m] = if n_m==0 {0} else {verifies[m]*SCALE/n_m}            // ∈[0,SCALE]
  2. value_micro[m] = (W_VR*mean_verify_micro[m] + W_PR*price_signal_micro[m]) / (W_VR+W_PR)   // W_VR=3,W_PR=1 frozen — kernel verdict dominant, price refines
  3. EXPLORATION BONUS (isqrt-only, NO ln/LOG_TABLE — the key simplification vs classic UCB1): bonus_micro[m] = C_NUM * isqrt( (global_pulls+1) * SCALE * SCALE / max(1,n_m) ) / C_DEN. This is count-based optimism (a √(t/n) family, monotone ↑ in t and ↓ in n — the only properties the gate needs), realized with a single integer Newton isqrt over u128. C_NUM=1414, C_DEN=1000 (≈√2), frozen. (Rationale: classic UCB1's ln dampening is asymptotic flavour; at this budget the count-bonus ordering is what matters, and dropping ln removes the one primitive the constitution judge flagged as collapse-prone.)
  4. score_micro[m] = value_micro[m] + bonus_micro[m]

SELECTION.
  A. eligible_now = {m | NOT decommissioned[m]}. If empty → reason=ALL_DECOMMISSIONED, target is unsolved-at-budget (a real §3 outcome); emit telemetry, stop allocating to T. (Never zero-probability a model permanently while it has budget — decommission is the only exit.)
  B. EXPLORATION FLOOR precedes argmax. ε = min(0.10, 0.40/k) as exact integer pair (eps_num,eps_den)=( (5k<=20 ? 10 : 40), (5k<=20 ? 100 : 100*k) ) — for k<=4 → 1/10, for k>4 → 40/(100k)=0.40/k (no f64). floor_share[m] = ceil_div(eps_num*B_T, eps_den). under_floor = {m∈eligible_now | pulls[m] < floor_share[m]}. If under_floor nonempty → FORCED-EXPLORE tick: draw uniformly among under_floor with the carrier's existing seeded rng = StdRng::seed_from_u64(args.seed + round*131 + ai) (line 1859), extended with a stable hash(T) so each (seed,round,ai,target) tick is independently reproducible; record draw + range. reason=EXPLORE_FLOOR.
  C. else EXPLOIT: selected = argmax_{m∈eligible_now} score_micro[m]; ties by sorted model_id (the actor.rs:46 first-seen-wins cross-multiply tie rule). NO rng consumed on a pure-argmax tick (rng_draw=None) so replay needn't reproduce an unused draw. reason=EXPLOIT_UCB.

ALLOCATE. Budget unit = proposal-call grant (integer). This tick grants 1 call to `selected`, B_T -= 1; record budget_remaining_before/after. `selected` REPLACES agent_models[ai] at the GenerateRequest call sites (lines 1914/1975/2178) for this slot; ProposalTelemetry.model_id := selected (line 732). Token/microUSD fall out ex-post via market_tape_shared::derive_cost over the model_id-tagged events (already recomputable) — the bandit allocates CALLS, cost = which model got the calls × its frozen MODEL_RATES rate.

FEEDBACK + DETERMINISM. After Lean runs, the Verify event + VerificationResult land on tape; the NEXT tick re-derives bandit state from tape. One BudgetAllocationTelemetry CAS object is written per tick and its CID appended next to the funded WorkTx (§7 DAG reconstruction). A replay rebuilds state from Verify+ProposalTelemetry(model_id) events, recomputes score_micro with the same integer isqrt + pinned policy, re-runs selection (replaying rng_draw over the recorded range on forced-explore ticks), and asserts recomputed selected == recorded → allocation_view == derive_from_tape(tape).


**BudgetAllocationTelemetry struct:**

// src/runtime/budget_allocation_telemetry.rs — mirrors proposal_telemetry.rs §8 EXACTLY:
// ObjectType::Generic + schema-id const, canonical_encode/decode (positional bincode-BE,
// non-self-describing), write_to_cas/read_from_cas/decode_bytes trio, and a v1→Vn
// legacy-fallback decoder (BudgetAllocationTelemetryV1 + From) reserved per the proven
// positional-schema discipline (extend only via version bump + legacy decoder). ALL money/
// score fields are integer (u64/u128/i128/i64/Hash) — NO f64 anywhere.

const BUDGET_ALLOCATION_TELEMETRY_SCHEMA_ID: &str = "turingosv4.budget_allocation_telemetry.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelPrice {           pub model_id: String, pub price_signal_micro: u128, pub n_nodes: u32 }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelFailure {         pub model_id: String, pub verifies: u64, pub pulls: u64,
                                  pub hardfail_streak: u32, pub decommissioned: bool }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelScore {           pub model_id: String, pub value_micro: i128,
                                  pub bonus_micro: i128, pub score_micro: i128 }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplorationFloor {     pub eps_num: u64, pub eps_den: u64, pub n_cutoff: u32 }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAllocationTelemetry {
    pub policy_hash: Hash,                  // [u8;32] sha256 of frozen UcbBudgetPolicy canonical bytes
    pub policy_version: String,             // "turingosv4.ucb_budget.v1"
    pub input_state_cid: Cid,               // CID of the EconomicState/price snapshot this tick read (state_root_t)
    pub target_id: String,                  // theorem T — the bandit state is per-(model,target)
    pub round: u64,
    pub agent_index: u64,
    pub eligible_model_set_hash: Hash,      // errata#6: sha256 over SORTED roster (drift detector)
    pub price_vector: Vec<ModelPrice>,      // Art II.2 visible price signal, per model
    pub failure_features: Vec<ModelFailure>,// Art II.1 abstracted failure features, per model
    pub per_model_score: Vec<ModelScore>,   // integer UCB internals (value+bonus+score), per eligible model
    pub exploration_floor: ExplorationFloor,// frozen ε pair + N as integers
    pub rng_seed: u64,                      // args.seed-derived tick seed basis (+ derivation noted in policy)
    pub rng_draw: Option<u64>,              // Some(draw) on EXPLORE_FLOOR tick; None on pure argmax (no draw consumed)
    pub rng_range: Option<u64>,             // the gen_range upper bound the draw was taken from (replay)
    pub selected_model_id: String,          // funded model == the ProposalTelemetry.model_id set this tick
    pub allocated_budget_calls: u64,        // = 1 (this tick funds one proposal call)
    pub allocated_budget_tokens: u64,       // token cap granted (0 = uncapped-per-call; tokens billed ex-post)
    pub reason_code: String,                // "EXPLOIT_UCB" | "EXPLORE_FLOOR" | "ALL_DECOMMISSIONED"
    pub budget_remaining_before: u64,       // errata#6
    pub budget_remaining_after: u64,        // errata#6 (= before - allocated_budget_calls)
    pub router_overhead_cid: Cid,           // errata#6 MANDATORY: CID of a RouterOverhead object accounting
                                            //   ALL model-selection LLM/tool cost this tick. UCB router is
                                            //   pure-integer (no scoring LLM call) ⇒ points to a zero-cost
                                            //   RouterOverhead so the §4 "all router overhead counted" rule
                                            //   is PRESENT-and-provably-zero, never silently absent.
    #[serde(default)]
    pub tick: Option<u64>,                  // additive/optional flattened (round,ai,target) logical tick for ordering
}
// RouterOverhead {schema:"turingosv4.router_overhead.v1", model_id:String, prompt_tokens:u64,
//   completion_tokens:u64, micro_usd:i64} — folded into derive_cost so router cost enters §4
//   budget-fairness. Hash/Cid reuse state::q_state::Hash + bottom_white::cas::schema::Cid (both
//   [u8;32], same as ProposalTelemetry). write_to_cas/read_from_cas/decode_bytes copy
//   proposal_telemetry.rs 1:1; tests: cid_determinism, write_read_round_trip, distinct_records_
//   distinct_cids, schema_validity (field-count + forbidden-field guard: no chain_of_thought/raw_
//   prompt/internal_reasoning/etc.), v1_record_still_decodes_after_future_bump, and a
//   decimal-float fence (assert no '.' in any serialized numeric field — the price_index.rs:1164
//   pattern) to mechanically forbid f64 creep.


**Freeze / sha-pin plan:** A `UcbBudgetPolicy` struct holds EVERY tunable that affects an allocation decision: { policy_version:"turingosv4.ucb_budget.v1", scale:u64(1_000_000), w_vr:u64(3), w_pr:u64(1), c_num:u64(1414), c_den:u64(1000), n_cutoff:u32(3), eps_rule:"min(1/10, 40/(100k))" with the literal pair constants (10,100) and (40,100), hard_failure_classes:Vec<String> (the 5 hard reject_class strings; the 5 EXCLUDED infra classes documented), isqrt_algo_id:"newton_u128.v1", score_formula_id:"value+isqrt_bonus.v1", tie_break:"sorted_model_id", draw_basis:"seed+round*131+ai+hash(target)", eligible_model_roster:Vec<String> (sorted, frozen per §2) }. FREEZE = canonical_encode(&policy) → policy_hash = sha256(bytes) computed ONCE at run start.

Four hashes sha-pinned BEFORE the paid run (charter §11.7 / gate 5.3): (1) policy_hash (the UcbBudgetPolicy bytes); (2) eligible_model_set_hash = sha256(sorted_roster joined by 0x00) — separate so roster drift (errata#4: no fallback/alias/post-calibration admission) is independently detectable; (3) head_commit_sha (reuse market_tape_shared::head_commit_sha) so "loaded bytes == commit bytes" for the carrier+policy; (4) MODEL_RATES table hash (already in market_tape_shared) folded into the budget-fairness manifest.

All four are (a) written into the GenesisPin first-tape record alongside seed/roster/axiom_whitelist/head_commit_sha (GenesisPin gains a policy_hash + policy_version field — a small market-tape-event extension), (b) stamped on EVERY BudgetAllocationTelemetry tick, and (c) git-committed + recorded in the prereg artifact before the confirmatory run. Because canonical_encode is positional bincode over an all-integer/string struct, policy_hash is byte-stable and a standalone verifier reconstructs the same UcbBudgetPolicy for that version and re-hashes — any post-hoc retune flips the hash (anti-p-hacking / Art III.4). The bandit policy is deliberately NOT from_env (contrast BoltzmannMaskPolicy's fail-soft env loader) — it is frozen-file-only, so no env var can override a frozen field at run time.

Paid-run preflight gate (registered in scripts/constitution_gates.manifest.toml like the §17.4 gates, runnable via the runner-preflight skill): a pre-run check recomputes all four hashes and refuses to launch on any mismatch with the prereg'd values; a CI fixture-tape gate asserts the carrier emits a non-default policy_hash AND that derive_from_tape's recomputed selection == recorded selection. Per §8/Art 0.4, the BudgetAllocationTelemetry schema commit carries its own Path-B declaration (same Git2LedgerWriter-backed L4 substrate as commit 51c1d602; the HEAD_t/q_t debt explicitly noted still-open, not closed).


**Tape reconstruction:** derive_budget_allocation_from_tape(tape, cas) -> Result<(), ReconstructionFailure>. Walk BudgetAllocationTelemetry CIDs in tick order (round, agent_index, target_id). For each tick:
1. Verify provenance: policy_hash == GenesisPin.policy_hash (one frozen policy ran the whole run); eligible_model_set_hash == GenesisPin roster hash (no silent roster drift, errata#4); input_state_cid resolves in CAS.
2. Re-derive bandit state from the tape PREFIX: replay all prior ticks' Verify events + ProposalTelemetry.model_id (model attribution) + VerificationResult.verified/reject_class up to this tick, recomputing pulls[m]/verifies[m]/hardfail_streak[m]/decommissioned[m] per (model,target). Assert these EXACTLY match the tick's recorded failure_features (catches the in-loop-sidemap vs CAS-derived divergence the design's weakness flags — caught at the field level, not just post-hoc on the final pick).
3. Recompute per_model_score: value_micro (W_VR/W_PR blend over mean_verify_micro + price_signal_micro recomputed from the recorded price_vector) + isqrt bonus_micro, with the SAME integer ops (u128 Newton isqrt, SCALE truncation). Assert byte-equal to the recorded per_model_score.
4. Re-run selection: apply the ε-floor (recompute floor_share from exploration_floor + counted pulls), then either EXPLOIT argmax (deterministic, no rng) or, on EXPLORE_FLOOR, replay StdRng::seed_from_u64(rng_seed) and reproduce rng_draw over rng_range. Assert recomputed selected == recorded selected_model_id AND reason_code matches.
5. Cross-check the funded WorkTx: the ProposalTelemetry the tick's WorkTx points to MUST have model_id == selected_model_id (budget decision actually drove which model proposed — the §7 "did budget flow to the winning model" visual test).
6. Budget conservation: budget_remaining_after == budget_remaining_before - allocated_budget_calls; the running B_T decrement is consistent across ticks for each target.
The assertion allocation_view == derive_from_tape(tape) holds iff all six pass for every tick. Because the ONLY new numeric primitive is an integer isqrt (deterministic, byte-portable) and all else is integer arithmetic already in price_index.rs, recompute is byte-exact across platforms (an f64 sqrt would make step 3 platform-dependent and break this gate — the reason the design refuses f64 on the decision path). RouterOverhead CIDs are summed into market_tape_shared::derive_cost so the §4 budget-fairness microUSD total includes router cost. Critical replay invariant the verifier must independently prove (the candidate-set-ordering trap thompson-beta exposed): the eligible_now / under_floor set construction is order-deterministic (sorted model_id), so rng-stream desync between live and replay is structurally impossible.


**Module placement:** NEW module src/runtime/budget_allocation_telemetry.rs (≈250 lines, the proposal_telemetry.rs template 1:1: BudgetAllocationTelemetry + ModelPrice/ModelFailure/ModelScore/ExplorationFloor + RouterOverhead structs, BUDGET_ALLOCATION_TELEMETRY_SCHEMA_ID const, write_to_cas/read_from_cas/read_from_cas_path/decode_bytes, a BudgetAllocationTelemetryV1 legacy struct + From for the reserved version-bump path, and the full test module incl. CID-determinism + schema-field-count + forbidden-field guard + decimal-float fence + v1-still-decodes). Declared with `pub mod budget_allocation_telemetry;` in src/runtime/mod.rs.

Trust-root note: src/runtime/mod.rs is a §8/Class-4 trust-root surface (adding a `mod` = constitution touch, per the design-discipline rule that trust-root pins runtime/mod.rs + lib.rs). Under the architect standing-rule, this module declaration is an authorized Class-4 trust-root touch; the trust-root manifest hash must be PIN-REHASHED per-atom when the mod line lands (the same Class-4 discipline as the §8 model_id commit 51c1d602). Treat the schema module + its mod.rs declaration as one atom; the carrier wiring (new Policy::UcbBudget arm + the per-tick bandit selector replacing agent_models[ai] indexing + the GenesisPin policy_hash field) is a separate Class 2-3 atom against src/bin/lean_market_agent.rs (gate-first + real evidence + clean-context audit per §9.4). The new constitution gate (preflight hash check + fixture-tape derive_from_tape recompute) registers in scripts/constitution_gates.manifest.toml. Known pre-existing Class-4 item to NOT touch in this work: the .claude/hooks/judge.sh stale trust-root manifest hash (clean@HEAD from commit 92c6ffe6, charter §11 footnote) — attribute it as pre-existing, never fix a trust-root hash mismatch as a side effect (§8 Class-4).


## OPEN QUESTIONS FOR ARCHITECT (must rule before freeze)

1. UCB1 ln-dampening dropped: I replace classic √(2 ln t / n) with a count-based isqrt-only bonus √(t/n)-family to delete the LOG_TABLE primitive (the one the constitution-fit judge flagged as collapse-prone) and keep only a single audited integer isqrt on the money path. This is a deliberate fidelity-vs-implementability trade: the bonus keeps UCB's monotonicity (↑ in global pulls, ↓ in per-arm pulls) but is more optimistic than log-dampened UCB1 (slower to stop exploring). Does the architect accept the count-bonus, or require true integer ln (a frozen LOG_TABLE, +1 primitive +1 monotonicity self-test)? If the bonus is named 'ucb' it should arguably be log-dampened to be name-faithful — or rename the arm to e.g. 'price_count_optimism' to avoid a soft name-fidelity claim that 'ucb' implies the ln form.

2. W_VR=3 / W_PR=1 price weight is a free design parameter. It sets how much market price can refine the kernel-verify value before the first reward lands (when mean_verify=0 for all arms, selection is price-bonus-floor only). Too high W_PR re-introduces PRICE_PROP's misroute risk (generalist out-prices specialist); too low and price contributes nothing pre-reward and the arm is pure isqrt-floor round-robin until a verify lands. Should the architect pin (3,1), or prereg a small calibration sweep on DISJOINT seeds to land it in the Goldilocks zone? (Note: any sweep must finish + freeze before the confirmatory K≥12.)

3. Cross-target transfer: the bandit state is per-(model,target), reset each theorem (honest, no leakage, but every seed re-explores from scratch — wastes early budget K times over). A frozen cross-target prior (a model good across the det-family starts above uniform) would convert complementary coverage faster but adds Goodhart surface and must be derived ONLY from disjoint calibration seeds (leakage guard §3) and pinned by policy_hash. Defer (within-target only) or include a calibration-derived prior?

4. BestHOMO control granularity vs the bandit's per-target state: §2 leaves BestHOMO as either (a) all-homo-arms-at-equal-budget max, or (b) disjoint-calibration single model. The treatment routes per-target, so 'equal budget' fairness needs the prereg to fix whether BestHOMO's budget = the bandit's TOTAL proposal-call cap summed across targets, or per-target. This affects the §4 economic-dominance denominator — architect should rule before freeze.

5. GenesisPin schema extension: adding policy_hash + policy_version to the GenesisPin market-tape event is a small but real tape-schema change (market_tape_shared.rs). Confirm this rides the same Path-B declaration as the BudgetAllocationTelemetry commit (one Art 0.4 declaration covering both), or whether the architect wants GenesisPin treated as a separate sudo-approved tape-schema touch.

6. Decommission + floor interaction at small B_T: with k=4, ε=10%/model, only 40% of B_T is floor-guaranteed; the other 60% goes to whichever arm scores first. A fast-but-wrong early arm can spend 3 calls before N=3 decommission frees its budget, potentially starving the slow-but-uniquely-capable arm before it gets past its floor. Is N=3 right at small budgets, or should the prereg lower N (faster decommission) or raise the floor (more guaranteed exploration) for the Goldilocks targets? This is the single most likely place the lever fails to express within budget — the §3 pilot must specifically measure it.


## The four candidate designs (one-liners)

- **price-proportional (PRICE_PROP) — budget-share ∝ each model's aggregated market price_yes,**: At each proposal slot the carrier aggregates `compute_price_index` price_yes over each eligible model's own attempt-nodes into an integer weight vector, applies the exploration floor as an integer prior, and selects the funded model_id by a deterministic seeded weighted draw — replacing the fixed `a

- **priced-softmax-reuse**: Per-iteration model-budget router that picks the proposer model by an INTEGER Boltzmann softmax over a price/success score (success-per-microUSD), reusing the existing integer-rational price + BoltzmannMaskPolicy substrate, with every routing tick emitting a CAS-resident BudgetAllocationTelemetry ob

- **ucb-bandit**: Treat each frozen-roster model as a bandit arm; at every proposal tick pick the arm maximizing an integer-rational UCB score (integer empirical verify-rate per 1e6 + integer exploration bonus from a fixed integer-sqrt table over global pulls / per-arm pulls), with the charter's ε_model floor + N=3 c

- **thompson-beta**: Per-model Beta(α,β) posterior over Lean-verify probability, updated only from tape-recorded hard outcomes; each routing tick draws one integer-rational sample per eligible model from a tape-canonical seeded RNG, then allocates the next proposal call's budget to the argmax sample — with the ε_model e
