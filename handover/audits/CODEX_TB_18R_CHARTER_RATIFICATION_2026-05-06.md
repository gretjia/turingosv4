# CODEX TB-18R Charter Ratification Audit

## 1. Header

- auditor: Codex
- date: 2026-05-06
- target: `handover/tracer_bullets/TB-18R_charter_2026-05-06.md`
- gate: Gate 1 pre-R1 Class-4 charter ratification
- HEAD: `46d79ca`
- scope: charter ratification only; no implementation review beyond cited source cross-checks

## 2. Inputs Reviewed

- `handover/tracer_bullets/TB-18R_charter_2026-05-06.md` - 635 lines, read in full.
- `handover/architect-insights/TB18_TAPE_NON_EXTERNALIZATION_VETO_2026-05-06.md` - 649 lines, read in full.
- `handover/evidence/tb_18_minif2f_m1_2026-05-05T18-31-55Z/MINIF2F_M1_BENCHMARK_REPORT.md` - 239 lines, read in full.
- `experiments/minif2f_v4/src/bin/evaluator.rs` - cited evaluator line sites verified on disk. Note: requested `src/eval/evaluator.rs` does not exist in this checkout; the charter and VETO cite `experiments/minif2f_v4/src/bin/evaluator.rs` (charter lines 208-209, 615; VETO lines 565, 636).
- `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, `src/state/sequencer.rs`, `src/bottom_white/ledger/rejection_evidence.rs`, `src/runtime/chain_derived_run_facts.rs`, `src/runtime/proposal_telemetry.rs`, `constitution.md` - targeted cross-checks only.

## 3. Per-Question Verdicts

### Q1 - Class-4 Classification

Verdict: CHALLENGE

Reasoning: TB-18R is correctly a Class-4 charter. The charter declares R1/R3/R4 Class-4 surfaces at lines 25-27 and blocks R1 on Codex Gate 1 at lines 39, 251, and 459-462. VETO Section C.4 independently grounds the same classification: new CAS schema, L4.E admission expansion, WorkTx canonical-payload semantics, and hard chain-derived invariant at VETO lines 594-602. Source confirms the touched surfaces are load-bearing: `ObjectType` is the CAS schema enum at `src/bottom_white/cas/schema.rs:40-42`; `WorkTx.proposal_cid` is part of the typed WorkTx schema at `src/state/typed_tx.rs:223-247`; sequencer rejection admission writes L4.E records at `src/state/sequencer.rs:2918-2991` and routes dispatch rejects at `src/state/sequencer.rs:3049-3064`.

Challenge: the charter is internally inconsistent about R4. It says R4 is Class 4 in the risk envelope (line 26), but later labels R4 as "Class 3 + hard invariant" at lines 42, 217-218, and the atom table row at line 452.

Remediation: change R4's atom classification to Class 4, or split it into a Class-4 ratified invariant-spec atom plus a Class-3 implementation atom that cannot alter the ratified equation. Update lines 42, 217-218, and 452.

### Q2 - 1 LLM Call = 1 AttemptTelemetry

Verdict: PASS

Reasoning: TB-18R is consistent with `feedback_chaintape_externalized_proposal`. The charter states "1 LLM call -> 1 compound payload = 1 Attempt Node" and explicitly says TB-18R does not decompose into tactic-level nodes at lines 15 and 334-336. VETO Section C.3 reaches the same conclusion: current M1 violates the primary clause, while TB-18R enforces one AttemptTelemetry per LLM call and does not invoke the per-tactic caveat (VETO lines 583-592). The current evaluator asymmetry also supports the target: omega success paths externalize through `submit_typed_tx` at `experiments/minif2f_v4/src/bin/evaluator.rs:2457`, `2500`, `2999`, and `3046`; failure paths at `3236`, `3263`, `3275`, and `3289` do not.

Remediation: none for Gate 1. Preserve the one-AttemptTelemetry-per-externalized-LLM-cycle rule and keep per-tactic decomposition out of TB-18R.

### Q3 - No Private-CoT Externalization

Verdict: CHALLENGE

Reasoning: The privacy rule is present but underspecified for R1/R2 implementation. The charter says TB-18R externalizes only model output parsed by evaluator and/or sent to Lean, while private model thinking remains private (lines 337-339). VETO Section B distinguishes private thought from externalized attempts and says parsed, Lean-checked, proof-prefix, or future-context-influencing outputs must be represented (VETO lines 270-280). Existing `ProposalTelemetry` has a good privacy pattern: tool calls store hashes only and forbid raw deliberation/prompt/completion strings at `src/runtime/proposal_telemetry.rs:80-88`; `prompt_context_hash` is a hash at lines 229-234; payload bytes written to CAS are the proposal artifact at lines 236-245. The current omega paths store parsed payload/tactic bytes, not a full raw response (`evaluator.rs:2358-2390`, `2902-2932`).

Challenge: FR-18R.1 names `candidate_payload_cid` without defining its permitted preimage (charter lines 258-263), and R2 says "write candidate payload to CAS" before the Lean run without saying this is post-parse Lean candidate/tool-argument bytes only (charter lines 208-212). That leaves a possible implementation path where a raw LLM response containing hidden or prompted reasoning is stored under `candidate_payload_cid`.

Remediation: amend FR-18R.1/R2 to say `candidate_payload_cid` MUST be the parsed external candidate actually sent to Lean or used as the next external proof state, never the full raw model response; `prompt_context_hash` MUST remain hash-only; any raw prompt/completion transcript, if retained at all, is AuditOnly and not part of AttemptTelemetry. Add a test that fails if raw model response text is stored in AttemptTelemetry CAS.

### Q4 - chain_derived_run_facts Invariant Robustness

Verdict: CHALLENGE

Reasoning: The exact equation is constitutionally well-motivated. Art.0.2 requires all signals to be reconstructable from tape and specifically requires rejected/parse-failed/Lean-rejected branches on tape (`constitution.md:52-65`). Existing `ChainDerivedRunFacts` already defines proposal count as accepted plus rejected WorkTx (`src/runtime/chain_derived_run_facts.rs:15-24`) and includes L4.E Work records in the count (`src/runtime/chain_derived_run_facts.rs:403-417`). Sequencer rejection writes L4.E without state-root/logical-t advance (`src/state/sequencer.rs:2918-2991`, `3049-3064`).

Challenge: the charter conflicts with itself by making the equation hard at FR-18R.3 (lines 271-275) while allowing +/-2 in FR-18R.4 and SG-18R.4 (lines 277-281 and 382-385). The tolerance is not yet principled. The M1 report demonstrates real external halt races: WallClockCap can leave only TaskOpen/EscrowLock, no TerminalSummary or EvidenceCapsule (M1 report lines 56-60). The charter also does not specify a post-run sequencer settlement/drain barrier even though sequencer application is asynchronous and per-tx rejection does not halt the loop (`src/state/sequencer.rs:2882-2895`).

Remediation: replace +/-2 with deterministic accounting. Recommended charter text: exact invariant applies after a mandatory sequencer drain barrier over completed LLM-Lean attempts; externally killed or mid-Lean aborted attempts must be counted in a separate explicit `attempt_aborted_count`/terminal abort record and excluded from `evaluator_reported_completed_llm_calls` by definition. EvidenceCapsule batching may summarize logs but cannot substitute per-attempt L4/L4.E records, matching VETO lines 396-397.

### Q5 - R1 Schema Generality / Future Tactic Variant

Verdict: CHALLENGE

Reasoning: The current schema direction is close, but the charter under-specifies generalization. FR-18R.1 lists AttemptTelemetry fields but no `attempt_kind`/`schema_version` field (lines 258-263). The VETO repair plan's `AttemptEnvelope` includes `candidate_kind: AttemptKind` with variants Step, StepPartialOk, StepReject, ParseFail, SorryBlock, OmegaWtool (VETO lines 282-306). Those variants mix attempt shape with outcome/path labels and are evaluator-current. CR-18R.3 correctly forbids per-tactic decomposition in TB-18R while leaving it to TB-8+ (charter lines 334-336), but the R1 schema should not make that future extension require another semantic rewrite.

Remediation: make outcome and attempt kind separate. Add an explicitly tail-extensible `attempt_kind` with the TB-18R value(s) for externalized LLM-Lean cycles and a reserved future `Tactic`/`ExternalToolCall` style variant if the charter wants TB-8+ compatibility. Add `schema_version`. Keep per-tactic emission disabled in R2 tests.

### Q6 - Grandfathering Policy

Verdict: CHALLENGE

Reasoning: The policy substance aligns with `feedback_no_retroactive_evidence_rewrite`. The M1 report's top annotation says the evidence is grandfathered, not benchmark evidence, preserves evidence as-is, and points to post-TB-18R reruns (M1 report lines 3-11). A local diff shows an annotation-only change to `MINIF2F_M1_BENCHMARK_REPORT.md`; no L4/L4.E/CAS root rewrite was observed in that diff. This matches VETO Section C.5's going-forward-only rule (VETO lines 604-609).

Challenge: the charter says the annotation is a README annotation, but the evidence directory has no `README.md`; the actual annotation is in `MINIF2F_M1_BENCHMARK_REPORT.md`. Affected text: FR-18R.10 lines 317-324, SG-18R.10 lines 409-412, and R0 closure intent line 480.

Remediation: either add a `README.md` with the same grandfathering annotation, or amend the charter/SG to cite `MINIF2F_M1_BENCHMARK_REPORT.md` as the grandfathering artifact. Keep the annotation-only/no-root-rewrite requirement unchanged.

### Q7 - SG-TAPE-1..9 Typing and Falsifiability

Verdict: CHALLENGE

Reasoning: SG-TAPE-1..3 and 5..9 are mostly well-typed and falsifiable: each names a concrete expected fact and a test or replay source (charter lines 365-380 and 387-407). SG-TAPE-1 also covers the six relevant evaluator paths (lines 365-370), matching the verified source asymmetry at `evaluator.rs:3236`, `3263`, `3275`, and `3289`.

Challenge: SG-18R.4/SG-TAPE-4 is ambiguous: "P49-class", "approx N", and "+/-2 for nondeterministic halt edge cases" do not define a deterministic expected value or the admissible halt states (charter lines 382-385; R6 repeats the same +/-2 at line 454). This weakens the same invariant SG-18R.3 made exact.

Remediation: rewrite SG-18R.4 as exact per-run evidence fields: `expected_completed_attempts`, `l4_work_attempt_count`, `l4e_work_attempt_count`, `attempt_aborted_count`, `delta`, and terminal halt class. Permit nonzero aborts only for named terminal states with an explicit abort record; otherwise `delta == 0`.

### Q8 - Class 4 Hiding Inside Class 3

Verdict: CHALLENGE

Reasoning: The charter mostly prevents hiding: G1 blocks R1 (lines 251, 459-462), and R1/R3 precede R2/R6/R7 (lines 462-468). However, two Class-4-adjacent changes are not owned cleanly. First, FR-18R.8 requires final composite proof payloads to carry `attempt_chain_root` (lines 304-309), but R5 is only Class 3 audit/assertion work (line 453); the implementation owner for that canonical proposal-payload schema change is not explicit. Second, R3 says it extends `rejection_class` and claims LeanFailed already exists (line 451), but the current L4.E `RejectionClass` enum only has PredicateFailed, PolicyViolation, EscrowMissing, InvariantViolation, MalformedPayload, and InsufficientBalance (`src/bottom_white/ledger/rejection_evidence.rs:145-165`). That enum file is not listed in R3's key surface line 214 or cross-reference lines 616-618.

Remediation: move `attempt_chain_root` payload schema definition into R1 as a Class-4 ratified schema surface, with R5 only testing it. Add `src/bottom_white/ledger/rejection_evidence.rs` to R3's Class-4 surface and correct the false "LeanFailed already exists" note. R2 remains Class 3 only if it populates already-ratified R1/R3 surfaces.

### Sanity Check - Sections 1, 2, 6 vs Ship Gates

Verdict: CHALLENGE

Reasoning: The one-line goal, atom sequence, and freeze list are directionally consistent: the goal freezes M2/M3/NodeMarket/Polymarket/public-chain/readiness until TB-18R final ship (charter lines 181-189), the atom sequence blocks R1 behind G1 and G2 behind evidence (lines 443-469), and the FREEZE list carries those blocks forward (lines 597-607). SG-18R.12 also correctly says a VETO retriggers charter revision before R1 (lines 422-424).

Challenge: internal consistency is weakened by the same three charter issues: R4 is both Class 4 and Class 3 (lines 26, 42, 217-218, 452), SG-18R.3 exactness conflicts with SG-18R.4 +/-2 (lines 376-385), and the grandfathering artifact is called README even though the actual annotation is in the benchmark report (lines 409-412; M1 report lines 3-11).

Remediation: fix those three text inconsistencies before treating G1 as remediated.

## 4. Aggregate Verdict

Aggregate verdict: CHALLENGE-but-ship-clean

No VETO finding was found. R1 should not start under the current draft because SG-18R.12 requires PASS or remediated CHALLENGE before R1 (charter lines 422-424). After the listed text remediations are made and the user grants the explicit post-Gate-1 "go", the charter is ratifiable without changing its core TB-18R design.

VETO blocker quotes: none; no VETO verdict issued.

## 5. Top-3 Remediations

1. Replace the +/-2 attempt-count tolerance with exact deterministic accounting plus an explicit aborted/in-flight attempt category and a mandatory sequencer drain barrier. This addresses Q4/Q7 and the central Art.0.2 tape invariant.

2. Cleanly assign Class-4 ownership: make R4's hard invariant Class 4 or split it; move `attempt_chain_root` payload schema into R1; add `src/bottom_white/ledger/rejection_evidence.rs` to R3 and correct the `LeanFailed` claim.

3. Tighten R1/R2 schema privacy and future-proofing: define `candidate_payload_cid` as parsed external candidate bytes only, add `schema_version`, and separate `attempt_kind` from `outcome` with a tail-extensible enum.

## 6. Open Questions / Hypotheses

- Input-path hypothesis: the task requested `src/eval/evaluator.rs`, but no such file exists. The audit used the charter/VETO path `experiments/minif2f_v4/src/bin/evaluator.rs`, where all cited evaluator line numbers matched on disk.
- Evidence-count hypothesis: P49 quantitative counts were verified from the M1 report annotation and P49 `evaluator.stdout:1`; this audit did not recompute L4/L4.E counts from raw chain bytes beyond targeted grep/cross-checks.
- README hypothesis: no `README.md` exists in the M1 evidence directory at audit time. The grandfathering annotation is present in `MINIF2F_M1_BENCHMARK_REPORT.md`.
