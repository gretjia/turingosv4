OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: read-only
reasoning effort: xhigh
reasoning summaries: none
session id: 019df15a-2ebf-7042-9697-e4dbd13f6a40
--------
user
# Codex TB-16 Ship Audit — implementation-paranoid

**Role**: skeptical adversarial implementer-reviewer for the TB-16
(Controlled Market Smoke Arena) ship-gate dual external audit.
Independent of Gemini ship audit (parallel, architectural strategic
angle).

**Mandate**: TB-16 shipped under **Class 3 integration smoke envelope**
per architect §7.7 ("AI coder may implement autonomously, but ship
requires external audit"). **Per memory feedback_dual_audit_conflict**:
VETO > CHALLENGE > PASS. **Round cap = 2** per
feedback_elon_mode_policy. **ROI flip stop** per
feedback_audit_loop_roi_flip if R2 challenges shift to test-scaffold
edges.

## Audit target — architect §7 spec verbatim (THIS IS THE GROUND TRUTH)

```text
7. TB-16 — Controlled Market Smoke Arena

7.1 目标
在受控沙盒中跑通: compute + position + complete set + price + mask + autopsy.
仍不开放真实市场。

7.2 Scenario
Lean task; multiple Agents; WorkTx FirstLong; ChallengeTx Short;
CompleteSet share inventory; PriceIndex updates; Boltzmann scheduler
selects next candidate; some agents lose positions; Autopsy generated.

7.3 Functional requirements
FR-16.1 At least 3 agents participate.
FR-16.2 At least one WorkTx creates FirstLongPosition.
FR-16.3 At least one ChallengeTx creates ShortPosition.
FR-16.4 At least one CompleteSetMintTx exists.
FR-16.5 At least one price update occurs.
FR-16.6 At least one Boltzmann mask event occurs.
FR-16.7 At least one AutopsyCapsule is generated.

7.4 Constitutional requirements
CR-16.1 Total Coin conserved.
CR-16.2 No ghost liquidity.
CR-16.3 No price overriding predicates.
CR-16.4 No raw failure broadcast.
CR-16.5 No real user funds.
CR-16.6 All activity replayable from ChainTape + CAS.
CR-16.7 All market activity is sandbox-labeled.

7.5 Ship gates
SG-16.1 Controlled market smoke produces replayable ChainTape.
SG-16.2 Dashboard shows positions, prices, masks, autopsies.
SG-16.3 No fake accepted nodes.
SG-16.4 Unsolved tasks show failure evidence / bankruptcy anchors.
SG-16.5 All market balances conserved.
SG-16.6 No unresolved evidence gaps.
SG-16.7 At least one loss -> autopsy path.
SG-16.8 Sandbox flag prevents real-money interpretation.

7.6 Forbidden
No public chain. No real-money market. No external domain.
No unbounded leverage. No AMM trading unless explicitly scoped.
No DPMM / pro-rata. No medical/legal/financial domains.
No production user funds.

7.7 Loop-mode instruction
Risk class: Class 3 integration smoke.
AI coder may implement autonomously, but ship requires external audit.
Halt if: any conservation failure; raw log leak; price-as-truth
behavior; non-sandbox funds used; unresolved evidence gap.
```

## TB-16 ship anchors

- Charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
- Ship status: `handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md`
- Design (38 audit assertions): `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
- Audit pipeline evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/`

```text
TB-16 commit chain:
  7d0d65b Atom 0 — charter ratified
  f7e5f0a Atom 1 — halt-trigger fixture (13 H1..H13 stubs)
  c0c890a Atom 2 — audit_assertions module (38 assertions × 8 layers)
  b4480d7 Atom 3 — audit_tape + audit_tape_tamper binaries
  4a7863e Atom 4 — dashboard §15 live regen + §16 SANDBOX banner
  36413c0 Atom 5 — comprehensive_arena evaluator harness scaffold
  3300fe2 Atom 6 SHIP (pre-audit) — run scripts + audit pipeline smoke

HEAD (3300fe2):
  cargo test --workspace = 905 passed / 0 failed / 150 ignored
  13/13 halt-triggers GREEN (tests/tb_16_halt_triggers.rs)
  Trust Root: GREEN (1 rehash: src/runtime/mod.rs Atom 2; src/bin/audit_dashboard.rs Atom 4)
```

## Files in scope

```text
NEW src/runtime/audit_assertions.rs       (Atom 2; ~960 LoC; 51 pub symbols)
NEW src/bin/audit_tape.rs                  (Atom 3; CLI ~180 LoC)
NEW src/bin/audit_tape_tamper.rs           (Atom 3; CLI ~280 LoC)
NEW experiments/minif2f_v4/src/bin/comprehensive_arena.rs (Atom 5; orchestrator scaffold ~370 LoC)
NEW tests/tb_16_halt_triggers.rs           (Atom 1; 13 halt fixtures)
NEW tests/tb_16_audit_tape_binary.rs       (Atom 3; 3 binary smoke tests)
NEW tests/tb_16_dashboard_live_regen.rs    (Atom 4; 2 dashboard tests)
NEW experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs (Atom 5; 2 tests)
NEW handover/tests/scripts/run_real_llm_arena.sh    (Atom 6; runner)
NEW handover/tests/scripts/audit_tape_smoke_test.sh (Atom 6; ship-gate)
MOD src/runtime/mod.rs                     (Atom 2; +pub mod audit_assertions)
MOD src/bin/audit_dashboard.rs             (Atom 4; live regen + §16 SANDBOX banner + DashboardReport.sandbox_run)
MOD genesis_payload.toml                   (rehash mod.rs + audit_dashboard.rs)
NEW handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/
    (verdict.json + verdict_replay.json + tamper_report.json +
     MARKOV_TB-16_2026-05-03.json + LATEST_MARKOV_CAPSULE.txt + dashboard.txt)
```

## Atom 6 v0 SCOPE NOTE (you are auditing this honestly)

The Atom 6 evidence shipped uses the TB-13 real-LLM smoke fixture as
the chain-backed substrate for the audit-pipeline smoke. **A fresh
6-task arena run that exercises FR-16.2..7 on a single chain is
deferred to Atom 6.1** (multi-task chain-continuation evaluator
extension) + mathlib build (user-side `lake exe cache get`). The
infrastructure (audit_assertions, audit_tape, audit_tape_tamper,
comprehensive_arena scaffold, dashboard live regen + §16 banner) is
ship-grade; the **fresh arena execution** is gated.

## Architect-mandated audit questions (FR-16.x + CR-16.x + SG-16.x)

**Q1 (FR-16.1)**: 8 sandbox-prefixed agents in `comprehensive_arena.rs`
preseed manifest (sandbox_preseed_pairs). Verify these IDs are sandbox-
prefixed AND that `runtime::audit_assertions::sandbox_prefix` admits
them all. Confirm Layer A #3 (sandbox_agent_prefix) PASSES on the
preseed.

**Q2 (FR-16.2..7)**: These are EXERCISE requirements — the arena must
emit each tx kind. Atom 6 ships infrastructure ONLY; fresh arena run
is Atom 6.1. Audit: is "infrastructure ready + fresh arena gated"
acceptable for ship per architect §7.7, or does §7.7 require an actual
13-tx-kind tape before architect signoff? Lookup architect §7.7
verbatim and verify.

**Q3 (CR-16.1 — conservation)**: `audit_assertions::assert_18_total_supply_conserved`
sums balances_t + escrows_t + stakes_t + conditional_collateral_t
against GENESIS_TOTAL_MICRO=30_000_000. Verify this matches
runtime::bootstrap::default_pput_preseed_pairs (architect §6.4
inheritance). Verify Layer D #21 (node_positions_excluded) + #22
(conditional_shares_excluded) STRUCTURALLY PROVE positions are NOT in
the supply sum (they would need to ALWAYS make `with_X != baseline`,
which requires non-empty index OR vacuous truth). Is the vacuous-truth
branch a real fence or a logic gap?

**Q4 (CR-16.4 / Layer F privacy)**: `assert_28_projection_no_autopsy_bytes`
canonical-encodes `q.tape_view_t` (the AgentVisibleProjection) and
checks for 32-byte runs of any private_detail_cid. Verify (a) the
encoding is the canonical one Agents would actually see; (b) the
32-byte run check catches Cid bytes if serialized (hint: serde_json
serializes [u8;32] as JSON array of decimals, NOT raw bytes). Is the
canonical_encode path the one that flows to Agents, or is there a
serde_json bypass? Trace `tape_view_t` flow from QState through
AgentVisibleProjection serialization in `runtime/{adapter,verify}.rs`.

**Q5 (CR-16.6 / replay)**: `assert_12_replay_state_root_matches_head`
+ `assert_16_replay_idempotent_across_calls`. Verify these run
`replay_full_transition` against the SAME entries twice and assert
identical state_root + ledger_root. Layer C #16 verdict_replay
byte-identity is the system-level acceptance gate. Is the replay path
in `audit_tape` identical to the replay path in production
`Sequencer::apply_one`? If they diverge, audit-from-tape verdict can
be PROCEED while production is broken.

**Q6 (CR-16.7 / SG-16.8 sandbox banner)**: `audit_dashboard.rs`
`render_section_16` renders SANDBOX banner when `sandbox_run=true`.
`detect_sandbox_run` walks L4 + agent_pubkeys for sandbox prefixes.
Verify: (a) the prefix list in `detect_sandbox_run` matches
`audit_assertions::sandbox_prefix` (consistency); (b) any
NON-sandbox agent_id in the chain causes sandbox_run=true to remain
true (the function early-returns on first match — is that the right
policy?). Architect §7.6 says "no production user funds" — what
prevents a sandbox-prefix agent from being conflated with a
production wallet? Confirm sandbox_run=true is an OR (any sandbox
match suffices) — but the deeper question: should a chain with MIXED
sandbox and non-sandbox IDs ALSO trip the banner, OR force a halt?

**Q7 (Layer H tamper / audit_tape_tamper binary)**: 3 corruption
modes — flip_l4_byte, flip_cas_byte, truncate_l4_ref. Verify each
mode actually corrupts something verify_chaintape's pipeline detects.
The `flip_byte_in_first_blob` helper picks ".git/objects/" first
non-empty file: this could be a tree object, not the L4 commit blob
itself. Does verify_chaintape detect tree-blob corruption (vs commit-
blob corruption)? If not, flip_l4_byte may sometimes produce
"detected" without actually testing L4 row tampering.

**Q8 (audit_tape verdict.json schema)**: Inspect
`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json`.
Verify schema_version="v1/audit_tape_verdict"; tape_root has 6 fields
(l4_count, l4e_count, head_state_root_hex, head_ledger_root_hex,
cas_object_count, constitution_hash_hex); tx_kind_counts has 14 fields
(13 architect-required + reuse); assertions has ≥38 entries each with
{id, name, layer, result, detail}; passed+failed+halted+skipped sum
to assertions.len(); feature_coverage maps each TB to GREEN/YELLOW/RED;
verdict ∈ {PROCEED, BLOCK}.

**Q9 (R-022 backlinks)**: All 53 pub symbols in
`src/runtime/audit_assertions.rs` carry
`/// TRACE_MATRIX FC1-N34 + FC2-N31` doc-comments per R-022. Verify
this is genuine traceability (vs. boilerplate) — does any pub symbol
have a more specific FC role that should be cited? E.g. tamper
assertions are FC1-N35, not FC1-N34.

## Audit follow-ups (RQ — recursive questions; if R1 PASSES address these)

**RQ1 — Atom 6.1 readiness gap**: ship status §4 explicitly defers
multi-task chain continuation. Is this a CHARTER deficit (charter §3
Atom 6 implied a fresh 13-kind chain) or a legitimate phasing
(infrastructure first, fresh chain second)? Per
feedback_no_fake_menus, did the charter set the right expectation?

**RQ2 — H7 demonstration mode**: ship status §6 + evidence README
claim "H7 demonstrated live via TB-13 fixture's E #27 halt". This is
unintended demonstration — the fixture happens to have an evidence
gap because TB-13 didn't write evidence_capsule_cid to CAS. Should
ship status be more cautious in framing this as "demonstration" vs.
"accidentally-detected gap in upstream fixture"?

**RQ3 — Test count**: ship status §3 says 905 / 0 / 150 with +25 from
TB-15 baseline 759. Walk the math: 759 + 25 = 784, NOT 905. Does the
905 figure include sub-package counts (minif2f_v4 + gix_capability)
that weren't in the 759 baseline? If yes, the +25 framing is
misleading. Audit the ship-status §3 wording for fidelity.

**RQ4 — Architect §7.6 forbidden enforcement**: SG-16.8 verifies
sandbox flag PREVENTS real-money interpretation. But do any of the
audit assertions or fence mechanisms ACTIVELY BLOCK a non-sandbox
agent from entering the tape? Or is sandbox prefix advisory? Trace
through: if someone submits a TaskOpen with sponsor_agent="0xPROD",
what stops it? Layer A #3 would FAIL audit, but does it BLOCK
admission? (Architect §7.7 says HALT — "non-sandbox funds used".)
Verify: dispatch arms / submission gate enforce sandbox prefix at
sequencer level, OR does it only manifest at audit-tape time?

**RQ5 — Markov capsule continuity**: TB-16 audit_pipeline_smoke
emitted `MARKOV_TB-16_2026-05-03.json` with previous_capsule_cid=null
(genesis) — but TB-15 already shipped a Markov capsule chained from
its own genesis. Why didn't TB-16 chain previous_capsule_cid to TB-15
capsule? Is this a Markov-chain break or expected (different evidence
dirs, different chain heads)?

## Verdict format

End your audit with one of:

```text
## VERDICT: PASS
(All Q1-Q9 + RQ1-RQ5 cleared; ship is clean for Class 3 envelope.)
```

```text
## VERDICT: CHALLENGE
- <claim id> CHALLENGE: <one-line reason + line refs>
(round-2 will trigger feedback_elon_mode_policy auto-execute on
determinate-best surgical patch.)
```

```text
## VERDICT: VETO
- <claim id> VETO: <one-line BLOCKING reason + line refs>
(VETO blocks ship per feedback_dual_audit_conflict; VETO > CHALLENGE > PASS.)
```

Include conviction (low/medium/high) + recommendation (PROCEED to SHIP /
FIX-THEN-PROCEED / REDESIGN / RETRO-CLASS-4-PROMOTION).

Cite file:line for every finding.

Save your audit to: handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md.


warning: Codex could not find bubblewrap on PATH. Install bubblewrap with your OS package manager. See the sandbox prerequisites: https://developers.openai.com/codex/concepts/sandboxing#prerequisites. Codex will use the vendored bubblewrap in the meantime.
codex
I’ll audit from the repository evidence and source, then produce the R1 verdict. The workspace is read-only in this session, so I can inspect and cite files but cannot create the requested audit file unless the environment changes.
  → Read governing AGENTS and TB-16 anchor docs
  • Trace source paths for audit assertions, dashboard, replay, privacy, tamper
  • Validate evidence schema and ship-status claims
  • Write R1 verdict content and report save limitation
exec
/bin/bash -lc 'git status --short && git rev-parse --short HEAD' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_13_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/agent_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/genesis_report.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/pinned_pubkeys.json
 M handover/evidence/tb_14_chaintape_smoke_2026-05-03/replay_report.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M rules/enforcement.log
?? experiments/minif2f_v4/h_vppu_history.json
?? handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md.raw
?? handover/audits/GEMINI_TB_16_SHIP_AUDIT_2026-05-04_R1.md
?? handover/audits/run_codex_tb_16_ship_audit.sh
?? handover/audits/run_gemini_tb_16_ship_audit.py
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/runtime_repo/
?? handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/cas/
?? handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/
?? handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_aime_1997_p9/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_algebra_11/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/full_n1_mathd_numbertheory_961/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_10/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_107/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/half_n1_mathd_algebra_359/runtime_repo/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/
?? handover/evidence/tb_8_minimal_payout_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/
?? handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md
3300fe2

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"' -g '"'!**/.git/**'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
The worktree is already dirty, including an untracked audit raw file under `handover/audits`; I’ll treat those as user/generated state and avoid relying on or overwriting them. I’m checking governing instructions and then I’ll trace the specific code paths named in the audit questions.
exec
/bin/bash -lc 'for d in / /home /home/zephryj /home/zephryj/projects /home/zephryj/projects/turingosv4; do test -f "$d/AGENTS.md" && printf '"'%s/AGENTS.md\\n' \""'$d"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc 'find handover experiments src tests -name AGENTS.md -print' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
exec
/bin/bash -lc 'rg -n "sandbox_prefix|assert_18_total_supply_conserved|assert_21|node_positions|assert_22|conditional_shares|assert_28_projection_no_autopsy_bytes|assert_12_replay_state_root_matches_head|assert_16_replay_idempotent_across_calls|GENESIS_TOTAL_MICRO|TRACE_MATRIX|pub (fn|struct|enum|const|type)|pub(crate)|pub mod" src/runtime/audit_assertions.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
26://! TRACE_MATRIX FC1-N34 (audit_tape binary) + FC2-N31 (verdict.json
65:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
66:pub struct AuditInputs {
72:    pub constitution: PathBuf,
78:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
79:pub enum AssertionLayer {
91:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
92:pub enum AssertionVerdict {
100:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
101:pub struct AssertionResult {
149:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
150:pub struct TapeRoot {
156:    pub constitution_hash_hex: String,
160:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
161:pub struct TxKindCounts {
179:    /// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
180:    pub fn from_entries(entries: &[LedgerEntry]) -> Self {
202:    /// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
203:    pub fn missing_required(&self) -> Vec<&'static str> {
230:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
231:pub struct TapeAuditVerdict {
249:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
250:pub enum AuditError {
300:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
301:pub struct LoadedTape {
311:    pub constitution_bytes: Vec<u8>,
312:    pub constitution_hash: Hash,
321:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
322:pub fn load_tape(inputs: &AuditInputs) -> Result<LoadedTape, AuditError> {
538:fn sandbox_prefix(agent: &str) -> bool {
551:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
552:pub fn assert_01_constitution_hash_matches_genesis(t: &LoadedTape) -> AssertionResult {
574:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
575:pub fn assert_02_pinned_pubkey_loaded(t: &LoadedTape) -> AssertionResult {
587:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
588:pub fn assert_03_sandbox_agent_prefix(t: &LoadedTape) -> AssertionResult {
591:        if !sandbox_prefix(agent) {
611:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
612:pub fn assert_04_l4_hash_chain_valid(t: &LoadedTape) -> AssertionResult {
650:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
651:pub fn assert_05_l4_parent_state_continuity(t: &LoadedTape) -> AssertionResult {
667:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
668:pub fn assert_06_l4e_chain_integrity(t: &LoadedTape) -> AssertionResult {
680:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
681:pub fn assert_07_genesis_row_zero_parents(t: &LoadedTape) -> AssertionResult {
702:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
703:pub fn assert_08_system_tx_signatures_verify(t: &LoadedTape) -> AssertionResult {
736:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
737:pub fn assert_09_agent_tx_signatures_verify(t: &LoadedTape) -> AssertionResult {
787:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
788:pub fn assert_10_payload_cid_resolves(t: &LoadedTape) -> AssertionResult {
802:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
803:pub fn assert_11_tx_kind_envelope_matches_payload(t: &LoadedTape) -> AssertionResult {
847:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
848:pub fn assert_12_replay_state_root_matches_head(t: &LoadedTape) -> AssertionResult {
884:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
885:pub fn assert_13_replay_economic_state_canonical(t: &LoadedTape) -> AssertionResult {
907:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
908:pub fn assert_14_replay_autopsy_index_chains(t: &LoadedTape) -> AssertionResult {
933:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
934:pub fn assert_15_canonical_edges_replay_deterministic(t: &LoadedTape) -> AssertionResult {
963:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
964:pub fn assert_16_replay_idempotent_across_calls(t: &LoadedTape) -> AssertionResult {
1037:const GENESIS_TOTAL_MICRO: i128 = 30_000_000;
1039:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1040:pub fn assert_17_no_post_init_mint(t: &LoadedTape) -> AssertionResult {
1057:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1058:pub fn assert_18_total_supply_conserved(t: &LoadedTape) -> AssertionResult {
1071:    if total == GENESIS_TOTAL_MICRO {
1078:            format!("total={total}μC; expected={GENESIS_TOTAL_MICRO}μC"),
1083:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1084:pub fn assert_19_complete_set_min_balanced(t: &LoadedTape) -> AssertionResult {
1126:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1127:pub fn assert_20_task_market_total_escrow_matches_locks(t: &LoadedTape) -> AssertionResult {
1158:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1159:pub fn assert_21_node_positions_excluded_from_supply(t: &LoadedTape) -> AssertionResult {
1160:    // Structural: source-level fence — node_positions_t entries are NOT
1163:    // showing it would diverge whenever node_positions_t is non-empty.
1169:                "node_positions_excluded_from_supply",
1177:    for (_, pos) in &q.economic_state_t.node_positions_t.0 {
1180:    if q.economic_state_t.node_positions_t.0.is_empty()
1185:        AssertionResult::pass(21, "node_positions_excluded_from_supply", AssertionLayer::D)
1189:            "node_positions_excluded_from_supply",
1191:            "including node_positions did not change total — implies they were already counted (CR-12.1 violation)".into(),
1196:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1197:pub fn assert_22_conditional_shares_excluded_from_supply(t: &LoadedTape) -> AssertionResult {
1203:                "conditional_shares_excluded_from_supply",
1221:            "conditional_shares_excluded_from_supply",
1227:            "conditional_shares_excluded_from_supply",
1238:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1239:pub fn assert_23_accepted_work_predicate_results_true(t: &LoadedTape) -> AssertionResult {
1275:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1276:pub fn assert_24_proposal_telemetry_chain(t: &LoadedTape) -> AssertionResult {
1361:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1362:pub fn assert_25_l4e_rejection_class_redispatch(_t: &LoadedTape) -> AssertionResult {
1371:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1372:pub fn assert_26_price_index_is_view_only(_t: &LoadedTape) -> AssertionResult {
1381:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1382:pub fn assert_27_terminal_summary_evidence_capsule(t: &LoadedTape) -> AssertionResult {
1440:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1441:pub fn assert_28_projection_no_autopsy_bytes(t: &LoadedTape) -> AssertionResult {
1489:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1490:pub fn assert_29_autopsy_private_detail_creator_is_system(t: &LoadedTape) -> AssertionResult {
1534:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1535:pub fn assert_30_typical_error_summary_no_private_detail(t: &LoadedTape) -> AssertionResult {
1609:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1610:pub fn assert_31_autopsy_index_value_type_is_vec_cid() -> AssertionResult {
1624:    let needle = "pub struct AutopsyIndex";
1651:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1652:pub fn assert_f_no_llm_self_narrative_in_autopsy(t: &LoadedTape) -> AssertionResult {
1709:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1710:pub fn assert_32_markov_constitution_hash_matches(t: &LoadedTape) -> AssertionResult {
1738:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1739:pub fn assert_33_markov_typical_errors_recompute(t: &LoadedTape) -> AssertionResult {
1792:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1793:pub fn assert_34_markov_unresolved_obs_recompute(
1846:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1847:pub fn assert_35_markov_next_session_context_resolves(t: &LoadedTape) -> AssertionResult {
1887:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1888:pub fn assert_36_tamper_l4_flip_detected() -> AssertionResult {
1897:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1898:pub fn assert_37_tamper_cas_flip_detected() -> AssertionResult {
1907:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1908:pub fn assert_38_tamper_l4_remove_detected() -> AssertionResult {
1921:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1922:pub fn run_all_assertions(inputs: &AuditInputs) -> Result<Vec<AssertionResult>, AuditError> {
1939:    r.push(assert_12_replay_state_root_matches_head(&tape));
1943:    r.push(assert_16_replay_idempotent_across_calls(&tape));
1946:    r.push(assert_18_total_supply_conserved(&tape));
1949:    r.push(assert_21_node_positions_excluded_from_supply(&tape));
1950:    r.push(assert_22_conditional_shares_excluded_from_supply(&tape));
1958:    r.push(assert_28_projection_no_autopsy_bytes(&tape));
1975:/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1976:pub fn summarize_results(
2065:    fn sandbox_prefix_accepts_known_patterns() {
2066:        assert!(sandbox_prefix("Agent_solver_0"));
2067:        assert!(sandbox_prefix("Agent_verifier_0"));
2068:        assert!(sandbox_prefix("Agent_user_0"));
2069:        assert!(sandbox_prefix("tb7-7-sponsor"));
2070:        assert!(sandbox_prefix("system"));
2071:        assert!(!sandbox_prefix("0xDEADBEEF"));
2072:        assert!(!sandbox_prefix("Mainnet_Wallet"));

exec
/bin/bash -lc 'rg -n "sandbox_preseed_pairs|sandbox|FirstLong|ChallengeTx|CompleteSet|PriceIndex|Boltzmann|Autopsy|WorkTx" experiments/minif2f_v4/src/bin/comprehensive_arena.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
24://!                              CompleteSetMint + (resolution) + redeem.
26://! Bootstrap: 8 sandbox-prefixed agents per design §4. Provider:
189:fn sandbox_preseed_pairs() -> Vec<(&'static str, i64)> {
262:            expected_outcome: "TerminalSummary + EvidenceCapsule; TaskBankruptcy + AgentAutopsyCapsule",
265:                "AgentAutopsyCapsule",
279:            description: "Agent_user_0 sponsors; MarketSeed + CompleteSetMint + redeem",
283:            expected_outcome: "MarketSeed + CompleteSetMint + (resolution) + CompleteSetRedeem",
285:                "MarketSeed", "CompleteSetMint", "CompleteSetRedeem",
308:    s.push_str("Agent IDs are sandbox-prefixed: `tb7-7-sponsor`, `Agent_user_0`,\n");
329:    s.push_str("Atom 5 (this binary) v0 scope: emit this plan + sandbox preseed manifest.\n");
361:    s.push_str("- Non-sandbox funds used (production wallet pattern).\n");
386:    eprintln!("comprehensive_arena: sandbox preseed = {:?}", sandbox_preseed_pairs());

exec
/bin/bash -lc 'rg -n "render_section_16|detect_sandbox_run|sandbox_run|sandbox|agent_pubkeys|SANDBOX|DashboardReport" src/bin/audit_dashboard.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
97:struct DashboardReport {
157:    /// walk OR in the agent_pubkeys.json manifest matches a sandbox-only
160:    /// interpreting sandbox prices/positions as production signals.
161:    sandbox_run: bool,
383:fn build_report(repo: &std::path::Path, cas_path: &std::path::Path) -> Result<DashboardReport, String> {
402:    let manifest_path = repo.join("agent_pubkeys.json");
933:    Ok(DashboardReport {
974:        sandbox_run: detect_sandbox_run(&entries, &cas, manifest.as_ref()),
986:/// SG-16.8): scan all L4 entries + agent_pubkeys manifest for any
987:/// agent_id matching a sandbox-only prefix.
988:fn detect_sandbox_run(
993:    let is_sandbox = |id: &str| -> bool {
1002:            if is_sandbox(k) {
1027:        if is_sandbox(&id) {
1199:fn render_text(r: &DashboardReport) -> String {
1486:    // §10 TB-9 Durable identity (Atom 6) — surface the agent_pubkeys manifest
1532:    s.push_str("  comparing this run's `agent_pubkeys.json` to a sibling run\n");
1692:    // matches a sandbox-only prefix (Agent_solver_*, Agent_verifier_*,
1697:    s.push_str(&render_section_16(r));
1703:/// SG-16.8): SANDBOX banner render. Source-fence — emit when
1704:/// `report.sandbox_run` is true (computed in build_report by scanning
1705:/// the L4 walk + agent_pubkeys manifest); otherwise no banner.
1706:fn render_section_16(r: &DashboardReport) -> String {
1707:    if !r.sandbox_run {
1712:    s.push_str("§16 TB-16 SANDBOX BANNER (architect 2026-05-03 §7.4 CR-16.7 + §7.5 SG-16.8)\n");
1714:    s.push_str("  ⚠ SANDBOX-RUN — NOT PRODUCTION — NO REAL FUNDS\n");
1715:    s.push_str("    Agent IDs are sandbox-prefixed (Agent_solver_/Agent_verifier_/\n");

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '520,610p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   520	    Ok(capsule)
   521	}
   522	
   523	fn is_system_tx_kind(k: TxKind) -> bool {
   524	    matches!(
   525	        k,
   526	        TxKind::FinalizeReward
   527	            | TxKind::ChallengeResolve
   528	            | TxKind::TerminalSummary
   529	            | TxKind::TaskExpire
   530	            | TxKind::TaskBankruptcy
   531	    )
   532	}
   533	
   534	fn is_agent_tx_kind(k: TxKind) -> bool {
   535	    !is_system_tx_kind(k) && !matches!(k, TxKind::Reuse)
   536	}
   537	
   538	fn sandbox_prefix(agent: &str) -> bool {
   539	    agent.starts_with("Agent_solver_")
   540	        || agent.starts_with("Agent_verifier_")
   541	        || agent.starts_with("Agent_user_")
   542	        || agent == "tb7-7-sponsor"
   543	        || agent.starts_with("tb16-")
   544	        || agent == "system"
   545	}
   546	
   547	// ─────────────────────────────────────────────────────────────────────
   548	// Layer A — bootstrap integrity (3 assertions)
   549	// ─────────────────────────────────────────────────────────────────────
   550	
   551	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   552	pub fn assert_01_constitution_hash_matches_genesis(t: &LoadedTape) -> AssertionResult {
   553	    let live = hex_encode(&t.constitution_hash.0);
   554	    match &t.genesis_constitution_root_hex {
   555	        None => AssertionResult::skipped(
   556	            1,
   557	            "constitution_hash_matches_genesis",
   558	            AssertionLayer::A,
   559	            "genesis [constitution_root] not present or unparseable; sha256 left unchecked"
   560	                .into(),
   561	        ),
   562	        Some(want) if want == &live => {
   563	            AssertionResult::pass(1, "constitution_hash_matches_genesis", AssertionLayer::A)
   564	        }
   565	        Some(want) => AssertionResult::fail(
   566	            1,
   567	            "constitution_hash_matches_genesis",
   568	            AssertionLayer::A,
   569	            format!("genesis: {want}; live: {live}"),
   570	        ),
   571	    }
   572	}
   573	
   574	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   575	pub fn assert_02_pinned_pubkey_loaded(t: &LoadedTape) -> AssertionResult {
   576	    if t.pinned_manifest.pubkeys.is_empty() {
   577	        return AssertionResult::fail(
   578	            2,
   579	            "pinned_pubkey_loaded",
   580	            AssertionLayer::A,
   581	            "pinned_pubkeys.json empty".into(),
   582	        );
   583	    }
   584	    AssertionResult::pass(2, "pinned_pubkey_loaded", AssertionLayer::A)
   585	}
   586	
   587	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   588	pub fn assert_03_sandbox_agent_prefix(t: &LoadedTape) -> AssertionResult {
   589	    let mut violations = Vec::new();
   590	    for agent in t.agent_manifest.agents.keys() {
   591	        if !sandbox_prefix(agent) {
   592	            violations.push(agent.clone());
   593	        }
   594	    }
   595	    if violations.is_empty() {
   596	        AssertionResult::pass(3, "sandbox_agent_prefix", AssertionLayer::A)
   597	    } else {
   598	        AssertionResult::halt(
   599	            3,
   600	            "sandbox_agent_prefix",
   601	            AssertionLayer::A,
   602	            format!("non-sandbox agent IDs: {violations:?}"),
   603	        )
   604	    }
   605	}
   606	
   607	// ─────────────────────────────────────────────────────────────────────
   608	// Layer B — chain integrity (8 assertions)
   609	// ─────────────────────────────────────────────────────────────────────
   610	

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '830,990p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   830	                "tx_kind_envelope_matches_payload",
   831	                AssertionLayer::B,
   832	                format!(
   833	                    "envelope {:?} != decoded {:?} at index {i}",
   834	                    e.tx_kind,
   835	                    typed.tx_kind()
   836	                ),
   837	            );
   838	        }
   839	    }
   840	    AssertionResult::pass(11, "tx_kind_envelope_matches_payload", AssertionLayer::B)
   841	}
   842	
   843	// ─────────────────────────────────────────────────────────────────────
   844	// Layer C — replay determinism (5 assertions)
   845	// ─────────────────────────────────────────────────────────────────────
   846	
   847	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   848	pub fn assert_12_replay_state_root_matches_head(t: &LoadedTape) -> AssertionResult {
   849	    let final_q = match &t.replayed_q {
   850	        Some(q) => q,
   851	        None => {
   852	            let detail = match &t.replay_error {
   853	                Some(e) => format!("replay error: {e}"),
   854	                None => "replay produced no QState".into(),
   855	            };
   856	            return AssertionResult::halt(
   857	                12,
   858	                "replay_state_root_matches_head",
   859	                AssertionLayer::C,
   860	                detail,
   861	            );
   862	        }
   863	    };
   864	    let head_root = t
   865	        .entries
   866	        .last()
   867	        .map(|e| e.resulting_state_root)
   868	        .unwrap_or(t.initial_q.state_root_t);
   869	    if final_q.state_root_t != head_root {
   870	        return AssertionResult::halt(
   871	            12,
   872	            "replay_state_root_matches_head",
   873	            AssertionLayer::C,
   874	            format!(
   875	                "replayed={} head={}",
   876	                hex_encode(&final_q.state_root_t.0),
   877	                hex_encode(&head_root.0)
   878	            ),
   879	        );
   880	    }
   881	    AssertionResult::pass(12, "replay_state_root_matches_head", AssertionLayer::C)
   882	}
   883	
   884	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   885	pub fn assert_13_replay_economic_state_canonical(t: &LoadedTape) -> AssertionResult {
   886	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   887	    if t.replayed_q.is_none() {
   888	        return AssertionResult::skipped(
   889	            13,
   890	            "replay_economic_state_canonical",
   891	            AssertionLayer::C,
   892	            "no replayed_q".into(),
   893	        );
   894	    }
   895	    let q = t.replayed_q.as_ref().unwrap();
   896	    match canonical_encode(&q.economic_state_t) {
   897	        Ok(_) => AssertionResult::pass(13, "replay_economic_state_canonical", AssertionLayer::C),
   898	        Err(e) => AssertionResult::fail(
   899	            13,
   900	            "replay_economic_state_canonical",
   901	            AssertionLayer::C,
   902	            format!("canonical_encode: {e}"),
   903	        ),
   904	    }
   905	}
   906	
   907	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   908	pub fn assert_14_replay_autopsy_index_chains(t: &LoadedTape) -> AssertionResult {
   909	    if let Some(q) = &t.replayed_q {
   910	        for (event_id, cids) in &q.economic_state_t.agent_autopsies_t.0 {
   911	            for cid in cids {
   912	                if t.cas.get(cid).is_err() {
   913	                    return AssertionResult::halt(
   914	                        14,
   915	                        "replay_autopsy_index_chains",
   916	                        AssertionLayer::C,
   917	                        format!("CAS missing autopsy {} for {:?}", hex_encode(&cid.0), event_id),
   918	                    );
   919	                }
   920	            }
   921	        }
   922	        AssertionResult::pass(14, "replay_autopsy_index_chains", AssertionLayer::C)
   923	    } else {
   924	        AssertionResult::skipped(
   925	            14,
   926	            "replay_autopsy_index_chains",
   927	            AssertionLayer::C,
   928	            "no replayed_q".into(),
   929	        )
   930	    }
   931	}
   932	
   933	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   934	pub fn assert_15_canonical_edges_replay_deterministic(t: &LoadedTape) -> AssertionResult {
   935	    // Structural fence: re-derive twice from the same entries; assert
   936	    // identical. (The full canonical_edges builder lives in TB-14 and
   937	    // is replay-deterministic by construction; here we assert the
   938	    // replayed economic_state_t is byte-stable across two calls.)
   939	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
   940	    if t.replayed_q.is_none() {
   941	        return AssertionResult::skipped(
   942	            15,
   943	            "canonical_edges_replay_deterministic",
   944	            AssertionLayer::C,
   945	            "no replayed_q".into(),
   946	        );
   947	    }
   948	    let q = t.replayed_q.as_ref().unwrap();
   949	    let a = canonical_encode(&q.economic_state_t).unwrap_or_default();
   950	    let b = canonical_encode(&q.economic_state_t).unwrap_or_default();
   951	    if a == b {
   952	        AssertionResult::pass(15, "canonical_edges_replay_deterministic", AssertionLayer::C)
   953	    } else {
   954	        AssertionResult::fail(
   955	            15,
   956	            "canonical_edges_replay_deterministic",
   957	            AssertionLayer::C,
   958	            "two canonical_encode calls disagree (catastrophic; would imply non-deterministic serialization)".into(),
   959	        )
   960	    }
   961	}
   962	
   963	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   964	pub fn assert_16_replay_idempotent_across_calls(t: &LoadedTape) -> AssertionResult {
   965	    let predicate_registry = PredicateRegistry::new();
   966	    let tool_registry = ToolRegistry::new();
   967	    let cas_view = CasStoreRef(&t.cas);
   968	    let q1 = match replay_full_transition(
   969	        &t.initial_q,
   970	        &t.entries,
   971	        &cas_view,
   972	        &t.pinned,
   973	        &predicate_registry,
   974	        &tool_registry,
   975	    ) {
   976	        Ok(q) => q,
   977	        Err(e) => {
   978	            return AssertionResult::halt(
   979	                16,
   980	                "replay_idempotent_across_calls",
   981	                AssertionLayer::C,
   982	                format!("replay-1 failed: {e}"),
   983	            );
   984	        }
   985	    };
   986	    let q2 = match replay_full_transition(
   987	        &t.initial_q,
   988	        &t.entries,
   989	        &cas_view,
   990	        &t.pinned,

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1030,1232p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1030	    }
  1031	    for (_, mc) in &q.economic_state_t.conditional_collateral_t.0 {
  1032	        total += mc.micro_units() as i128;
  1033	    }
  1034	    total
  1035	}
  1036	
  1037	const GENESIS_TOTAL_MICRO: i128 = 30_000_000;
  1038	
  1039	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1040	pub fn assert_17_no_post_init_mint(t: &LoadedTape) -> AssertionResult {
  1041	    // structural: every accepted tx has been re-dispatched by replay;
  1042	    // sequencer-side `assert_no_post_init_mint` fires inline. If replay
  1043	    // succeeded, no mint occurred.
  1044	    match &t.replayed_q {
  1045	        Some(_) => AssertionResult::pass(17, "no_post_init_mint", AssertionLayer::D),
  1046	        None => {
  1047	            let detail = t
  1048	                .replay_error
  1049	                .as_ref()
  1050	                .map(|e| format!("replay error: {e}"))
  1051	                .unwrap_or_else(|| "no replayed_q".into());
  1052	            AssertionResult::halt(17, "no_post_init_mint", AssertionLayer::D, detail)
  1053	        }
  1054	    }
  1055	}
  1056	
  1057	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1058	pub fn assert_18_total_supply_conserved(t: &LoadedTape) -> AssertionResult {
  1059	    let q = match &t.replayed_q {
  1060	        Some(q) => q,
  1061	        None => {
  1062	            return AssertionResult::skipped(
  1063	                18,
  1064	                "total_supply_conserved",
  1065	                AssertionLayer::D,
  1066	                "no replayed_q".into(),
  1067	            );
  1068	        }
  1069	    };
  1070	    let total = replayed_total_supply_micro(q);
  1071	    if total == GENESIS_TOTAL_MICRO {
  1072	        AssertionResult::pass(18, "total_supply_conserved", AssertionLayer::D)
  1073	    } else {
  1074	        AssertionResult::halt(
  1075	            18,
  1076	            "total_supply_conserved",
  1077	            AssertionLayer::D,
  1078	            format!("total={total}μC; expected={GENESIS_TOTAL_MICRO}μC"),
  1079	        )
  1080	    }
  1081	}
  1082	
  1083	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1084	pub fn assert_19_complete_set_min_balanced(t: &LoadedTape) -> AssertionResult {
  1085	    use crate::state::typed_tx::OutcomeSide;
  1086	    let q = match &t.replayed_q {
  1087	        Some(q) => q,
  1088	        None => {
  1089	            return AssertionResult::skipped(
  1090	                19,
  1091	                "complete_set_min_balanced",
  1092	                AssertionLayer::D,
  1093	                "no replayed_q".into(),
  1094	            );
  1095	        }
  1096	    };
  1097	    let _ = OutcomeSide::Yes;
  1098	    let mut yes_sum: BTreeMap<_, i128> = BTreeMap::new();
  1099	    let mut no_sum: BTreeMap<_, i128> = BTreeMap::new();
  1100	    for (_owner, by_event) in &q.economic_state_t.conditional_share_balances_t.0 {
  1101	        for (event_id, pair) in by_event {
  1102	            *yes_sum.entry(event_id.clone()).or_default() += pair.yes.units as i128;
  1103	            *no_sum.entry(event_id.clone()).or_default() += pair.no.units as i128;
  1104	        }
  1105	    }
  1106	    for (event_id, mc) in &q.economic_state_t.conditional_collateral_t.0 {
  1107	        let collateral = mc.micro_units() as i128;
  1108	        let y = *yes_sum.get(event_id).unwrap_or(&0);
  1109	        let n = *no_sum.get(event_id).unwrap_or(&0);
  1110	        let min_side = y.min(n);
  1111	        if min_side != collateral {
  1112	            return AssertionResult::halt(
  1113	                19,
  1114	                "complete_set_min_balanced",
  1115	                AssertionLayer::D,
  1116	                format!(
  1117	                    "event={:?} min(yes={y}, no={n}) != collateral={collateral}",
  1118	                    event_id
  1119	                ),
  1120	            );
  1121	        }
  1122	    }
  1123	    AssertionResult::pass(19, "complete_set_min_balanced", AssertionLayer::D)
  1124	}
  1125	
  1126	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1127	pub fn assert_20_task_market_total_escrow_matches_locks(t: &LoadedTape) -> AssertionResult {
  1128	    let q = match &t.replayed_q {
  1129	        Some(q) => q,
  1130	        None => {
  1131	            return AssertionResult::skipped(
  1132	                20,
  1133	                "task_market_total_escrow_matches_locks",
  1134	                AssertionLayer::D,
  1135	                "no replayed_q".into(),
  1136	            );
  1137	        }
  1138	    };
  1139	    let mut sum_per_task: BTreeMap<_, i128> = BTreeMap::new();
  1140	    for (_, e) in &q.economic_state_t.escrows_t.0 {
  1141	        *sum_per_task.entry(e.task_id.clone()).or_default() += e.amount.micro_units() as i128;
  1142	    }
  1143	    for (task_id, market) in &q.economic_state_t.task_markets_t.0 {
  1144	        let want = market.total_escrow.micro_units() as i128;
  1145	        let got = *sum_per_task.get(task_id).unwrap_or(&0);
  1146	        if want != got {
  1147	            return AssertionResult::halt(
  1148	                20,
  1149	                "task_market_total_escrow_matches_locks",
  1150	                AssertionLayer::D,
  1151	                format!("task={task_id:?} cache={want} sum_locks={got}"),
  1152	            );
  1153	        }
  1154	    }
  1155	    AssertionResult::pass(20, "task_market_total_escrow_matches_locks", AssertionLayer::D)
  1156	}
  1157	
  1158	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1159	pub fn assert_21_node_positions_excluded_from_supply(t: &LoadedTape) -> AssertionResult {
  1160	    // Structural: source-level fence — node_positions_t entries are NOT
  1161	    // summed into our total_supply helper above. If they were, #18 would
  1162	    // fail. Re-affirm by computing a "what if we included it" total and
  1163	    // showing it would diverge whenever node_positions_t is non-empty.
  1164	    let q = match &t.replayed_q {
  1165	        Some(q) => q,
  1166	        None => {
  1167	            return AssertionResult::skipped(
  1168	                21,
  1169	                "node_positions_excluded_from_supply",
  1170	                AssertionLayer::D,
  1171	                "no replayed_q".into(),
  1172	            );
  1173	        }
  1174	    };
  1175	    let baseline = replayed_total_supply_micro(q);
  1176	    let mut with_positions = baseline;
  1177	    for (_, pos) in &q.economic_state_t.node_positions_t.0 {
  1178	        with_positions += pos.amount.micro_units() as i128;
  1179	    }
  1180	    if q.economic_state_t.node_positions_t.0.is_empty()
  1181	        || with_positions != baseline
  1182	    {
  1183	        // either no positions to include (vacuous), or including them
  1184	        // would diverge — both confirm exclusion.
  1185	        AssertionResult::pass(21, "node_positions_excluded_from_supply", AssertionLayer::D)
  1186	    } else {
  1187	        AssertionResult::fail(
  1188	            21,
  1189	            "node_positions_excluded_from_supply",
  1190	            AssertionLayer::D,
  1191	            "including node_positions did not change total — implies they were already counted (CR-12.1 violation)".into(),
  1192	        )
  1193	    }
  1194	}
  1195	
  1196	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1197	pub fn assert_22_conditional_shares_excluded_from_supply(t: &LoadedTape) -> AssertionResult {
  1198	    let q = match &t.replayed_q {
  1199	        Some(q) => q,
  1200	        None => {
  1201	            return AssertionResult::skipped(
  1202	                22,
  1203	                "conditional_shares_excluded_from_supply",
  1204	                AssertionLayer::D,
  1205	                "no replayed_q".into(),
  1206	            );
  1207	        }
  1208	    };
  1209	    let baseline = replayed_total_supply_micro(q);
  1210	    let mut with_shares = baseline;
  1211	    for (_owner, by_event) in &q.economic_state_t.conditional_share_balances_t.0 {
  1212	        for (_, pair) in by_event {
  1213	            with_shares += pair.yes.units as i128 + pair.no.units as i128;
  1214	        }
  1215	    }
  1216	    if q.economic_state_t.conditional_share_balances_t.0.is_empty()
  1217	        || with_shares != baseline
  1218	    {
  1219	        AssertionResult::pass(
  1220	            22,
  1221	            "conditional_shares_excluded_from_supply",
  1222	            AssertionLayer::D,
  1223	        )
  1224	    } else {
  1225	        AssertionResult::fail(
  1226	            22,
  1227	            "conditional_shares_excluded_from_supply",
  1228	            AssertionLayer::D,
  1229	            "including shares did not change total — implies CR-13.3 violation".into(),
  1230	        )
  1231	    }
  1232	}

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '990,1038p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   990	        &t.pinned,
   991	        &predicate_registry,
   992	        &tool_registry,
   993	    ) {
   994	        Ok(q) => q,
   995	        Err(e) => {
   996	            return AssertionResult::halt(
   997	                16,
   998	                "replay_idempotent_across_calls",
   999	                AssertionLayer::C,
  1000	                format!("replay-2 failed: {e}"),
  1001	            );
  1002	        }
  1003	    };
  1004	    if q1.state_root_t == q2.state_root_t && q1.ledger_root_t == q2.ledger_root_t {
  1005	        AssertionResult::pass(16, "replay_idempotent_across_calls", AssertionLayer::C)
  1006	    } else {
  1007	        AssertionResult::halt(
  1008	            16,
  1009	            "replay_idempotent_across_calls",
  1010	            AssertionLayer::C,
  1011	            "two replays produced different roots".into(),
  1012	        )
  1013	    }
  1014	}
  1015	
  1016	// ─────────────────────────────────────────────────────────────────────
  1017	// Layer D — economic invariants (6 assertions)
  1018	// ─────────────────────────────────────────────────────────────────────
  1019	
  1020	fn replayed_total_supply_micro(q: &QState) -> i128 {
  1021	    let mut total: i128 = 0;
  1022	    for (_, mc) in &q.economic_state_t.balances_t.0 {
  1023	        total += mc.micro_units() as i128;
  1024	    }
  1025	    for (_, e) in &q.economic_state_t.escrows_t.0 {
  1026	        total += e.amount.micro_units() as i128;
  1027	    }
  1028	    for (_, s) in &q.economic_state_t.stakes_t.0 {
  1029	        total += s.amount.micro_units() as i128;
  1030	    }
  1031	    for (_, mc) in &q.economic_state_t.conditional_collateral_t.0 {
  1032	        total += mc.micro_units() as i128;
  1033	    }
  1034	    total
  1035	}
  1036	
  1037	const GENESIS_TOTAL_MICRO: i128 = 30_000_000;
  1038	

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1430,1508p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1430	            },
  1431	        };
  1432	    }
  1433	    AssertionResult::pass(27, "terminal_summary_evidence_capsule", AssertionLayer::E)
  1434	}
  1435	
  1436	// ─────────────────────────────────────────────────────────────────────
  1437	// Layer F — privacy contracts (4 assertions; TB-15 specific)
  1438	// ─────────────────────────────────────────────────────────────────────
  1439	
  1440	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1441	pub fn assert_28_projection_no_autopsy_bytes(t: &LoadedTape) -> AssertionResult {
  1442	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
  1443	    let q = match &t.replayed_q {
  1444	        Some(q) => q,
  1445	        None => {
  1446	            return AssertionResult::skipped(
  1447	                28,
  1448	                "projection_no_autopsy_bytes",
  1449	                AssertionLayer::F,
  1450	                "no replayed_q".into(),
  1451	            );
  1452	        }
  1453	    };
  1454	    let proj_bytes = canonical_encode(&q.tape_view_t).unwrap_or_default();
  1455	    // Collect autopsy private_detail_cid byte-runs from CAS and ensure
  1456	    // none appear in projection serialization.
  1457	    let mut private_cids: BTreeSet<[u8; 32]> = BTreeSet::new();
  1458	    for (_, cids) in &q.economic_state_t.agent_autopsies_t.0 {
  1459	        for cid in cids {
  1460	            let caps_bytes = match t.cas.get(cid) {
  1461	                Ok(b) => b,
  1462	                Err(_) => continue,
  1463	            };
  1464	            // Best-effort decode; if it fails, skip — tampered CAS
  1465	            // bytes will be flagged elsewhere.
  1466	            if let Ok(autopsy) = canonical_decode::<crate::runtime::autopsy_capsule::AgentAutopsyCapsule>(&caps_bytes) {
  1467	                private_cids.insert(autopsy.private_detail_cid.0);
  1468	            } else if let Ok(autopsy) = serde_json::from_slice::<crate::runtime::autopsy_capsule::AgentAutopsyCapsule>(&caps_bytes) {
  1469	                private_cids.insert(autopsy.private_detail_cid.0);
  1470	            }
  1471	        }
  1472	    }
  1473	    for run in &private_cids {
  1474	        for window in proj_bytes.windows(32) {
  1475	            if window == run {
  1476	                return AssertionResult::halt(
  1477	                    28,
  1478	                    "projection_no_autopsy_bytes",
  1479	                    AssertionLayer::F,
  1480	                    "AgentVisibleProjection serialization contains a private_detail_cid byte run"
  1481	                        .into(),
  1482	                );
  1483	            }
  1484	        }
  1485	    }
  1486	    AssertionResult::pass(28, "projection_no_autopsy_bytes", AssertionLayer::F)
  1487	}
  1488	
  1489	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1490	pub fn assert_29_autopsy_private_detail_creator_is_system(t: &LoadedTape) -> AssertionResult {
  1491	    let q = match &t.replayed_q {
  1492	        Some(q) => q,
  1493	        None => {
  1494	            return AssertionResult::skipped(
  1495	                29,
  1496	                "autopsy_private_detail_creator_is_system",
  1497	                AssertionLayer::F,
  1498	                "no replayed_q".into(),
  1499	            );
  1500	        }
  1501	    };
  1502	    for (_, cids) in &q.economic_state_t.agent_autopsies_t.0 {
  1503	        for cid in cids {
  1504	            let caps_bytes = match t.cas.get(cid) {
  1505	                Ok(b) => b,
  1506	                Err(_) => continue,
  1507	            };
  1508	            let autopsy: crate::runtime::autopsy_capsule::AgentAutopsyCapsule =

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1880,1995p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1880	    }
  1881	}
  1882	
  1883	// ─────────────────────────────────────────────────────────────────────
  1884	// Layer H — tamper detection (3 assertions; exercised via separate binary)
  1885	// ─────────────────────────────────────────────────────────────────────
  1886	
  1887	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1888	pub fn assert_36_tamper_l4_flip_detected() -> AssertionResult {
  1889	    AssertionResult::skipped(
  1890	        36,
  1891	        "tamper_l4_flip_detected",
  1892	        AssertionLayer::H,
  1893	        "exercised by audit_tape_tamper binary (Atom 3)".into(),
  1894	    )
  1895	}
  1896	
  1897	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1898	pub fn assert_37_tamper_cas_flip_detected() -> AssertionResult {
  1899	    AssertionResult::skipped(
  1900	        37,
  1901	        "tamper_cas_flip_detected",
  1902	        AssertionLayer::H,
  1903	        "exercised by audit_tape_tamper binary (Atom 3)".into(),
  1904	    )
  1905	}
  1906	
  1907	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1908	pub fn assert_38_tamper_l4_remove_detected() -> AssertionResult {
  1909	    AssertionResult::skipped(
  1910	        38,
  1911	        "tamper_l4_remove_detected",
  1912	        AssertionLayer::H,
  1913	        "exercised by audit_tape_tamper binary (Atom 3)".into(),
  1914	    )
  1915	}
  1916	
  1917	// ─────────────────────────────────────────────────────────────────────
  1918	// Battery + verdict
  1919	// ─────────────────────────────────────────────────────────────────────
  1920	
  1921	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1922	pub fn run_all_assertions(inputs: &AuditInputs) -> Result<Vec<AssertionResult>, AuditError> {
  1923	    let tape = load_tape(inputs)?;
  1924	    let mut r = Vec::with_capacity(40);
  1925	    // Layer A (3)
  1926	    r.push(assert_01_constitution_hash_matches_genesis(&tape));
  1927	    r.push(assert_02_pinned_pubkey_loaded(&tape));
  1928	    r.push(assert_03_sandbox_agent_prefix(&tape));
  1929	    // Layer B (8)
  1930	    r.push(assert_04_l4_hash_chain_valid(&tape));
  1931	    r.push(assert_05_l4_parent_state_continuity(&tape));
  1932	    r.push(assert_06_l4e_chain_integrity(&tape));
  1933	    r.push(assert_07_genesis_row_zero_parents(&tape));
  1934	    r.push(assert_08_system_tx_signatures_verify(&tape));
  1935	    r.push(assert_09_agent_tx_signatures_verify(&tape));
  1936	    r.push(assert_10_payload_cid_resolves(&tape));
  1937	    r.push(assert_11_tx_kind_envelope_matches_payload(&tape));
  1938	    // Layer C (5)
  1939	    r.push(assert_12_replay_state_root_matches_head(&tape));
  1940	    r.push(assert_13_replay_economic_state_canonical(&tape));
  1941	    r.push(assert_14_replay_autopsy_index_chains(&tape));
  1942	    r.push(assert_15_canonical_edges_replay_deterministic(&tape));
  1943	    r.push(assert_16_replay_idempotent_across_calls(&tape));
  1944	    // Layer D (6)
  1945	    r.push(assert_17_no_post_init_mint(&tape));
  1946	    r.push(assert_18_total_supply_conserved(&tape));
  1947	    r.push(assert_19_complete_set_min_balanced(&tape));
  1948	    r.push(assert_20_task_market_total_escrow_matches_locks(&tape));
  1949	    r.push(assert_21_node_positions_excluded_from_supply(&tape));
  1950	    r.push(assert_22_conditional_shares_excluded_from_supply(&tape));
  1951	    // Layer E (5)
  1952	    r.push(assert_23_accepted_work_predicate_results_true(&tape));
  1953	    r.push(assert_24_proposal_telemetry_chain(&tape));
  1954	    r.push(assert_25_l4e_rejection_class_redispatch(&tape));
  1955	    r.push(assert_26_price_index_is_view_only(&tape));
  1956	    r.push(assert_27_terminal_summary_evidence_capsule(&tape));
  1957	    // Layer F (4 + 1 supplemental)
  1958	    r.push(assert_28_projection_no_autopsy_bytes(&tape));
  1959	    r.push(assert_29_autopsy_private_detail_creator_is_system(&tape));
  1960	    r.push(assert_30_typical_error_summary_no_private_detail(&tape));
  1961	    r.push(assert_31_autopsy_index_value_type_is_vec_cid());
  1962	    r.push(assert_f_no_llm_self_narrative_in_autopsy(&tape));
  1963	    // Layer G (4)
  1964	    r.push(assert_32_markov_constitution_hash_matches(&tape));
  1965	    r.push(assert_33_markov_typical_errors_recompute(&tape));
  1966	    r.push(assert_34_markov_unresolved_obs_recompute(inputs, &tape));
  1967	    r.push(assert_35_markov_next_session_context_resolves(&tape));
  1968	    // Layer H (3)
  1969	    r.push(assert_36_tamper_l4_flip_detected());
  1970	    r.push(assert_37_tamper_cas_flip_detected());
  1971	    r.push(assert_38_tamper_l4_remove_detected());
  1972	    Ok(r)
  1973	}
  1974	
  1975	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1976	pub fn summarize_results(
  1977	    inputs: &AuditInputs,
  1978	    results: Vec<AssertionResult>,
  1979	) -> Result<TapeAuditVerdict, AuditError> {
  1980	    let tape = load_tape(inputs)?;
  1981	    let head = tape.entries.last();
  1982	    let head_state_root_hex = head
  1983	        .map(|e| hex_encode(&e.resulting_state_root.0))
  1984	        .unwrap_or_else(|| hex_encode(&tape.initial_q.state_root_t.0));
  1985	    let head_ledger_root_hex = head
  1986	        .map(|e| hex_encode(&e.resulting_ledger_root.0))
  1987	        .unwrap_or_else(|| hex_encode(&tape.initial_q.ledger_root_t.0));
  1988	    let tape_root = TapeRoot {
  1989	        l4_count: tape.entries.len() as u64,
  1990	        l4e_count: tape.l4e_writer.len() as u64,
  1991	        head_state_root_hex,
  1992	        head_ledger_root_hex,
  1993	        cas_object_count: tape.cas.len() as u64,
  1994	        constitution_hash_hex: hex_encode(&tape.constitution_hash.0),
  1995	    };

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '60,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    60	// ─────────────────────────────────────────────────────────────────────
    61	
    62	/// Inputs to the audit binary. Paths only — live process state is
    63	/// forbidden per CR-16.6 (replayability) + Art.0.2 (Tape Canonical).
    64	#[derive(Debug, Clone)]
    65	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
    66	pub struct AuditInputs {
    67	    pub runtime_repo: PathBuf,
    68	    pub cas_dir: PathBuf,
    69	    pub agent_pubkeys: PathBuf,
    70	    pub pinned_pubkeys: PathBuf,
    71	    pub genesis: PathBuf,
    72	    pub constitution: PathBuf,
    73	    pub markov_pointer: PathBuf,
    74	    pub alignment_dir: Option<PathBuf>,
    75	}
    76	
    77	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    78	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
    79	pub enum AssertionLayer {
    80	    A, // bootstrap integrity
    81	    B, // chain integrity
    82	    C, // replay determinism
    83	    D, // economic invariants
    84	    E, // predicate / evidence
    85	    F, // privacy contracts
    86	    G, // Markov continuity
    87	    H, // tamper detection (separate binary)
    88	}
    89	
    90	#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    91	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
    92	pub enum AssertionVerdict {
    93	    Pass,
    94	    Fail,
    95	    Halt,
    96	    Skipped,
    97	}
    98	
    99	#[derive(Debug, Clone, Serialize, Deserialize)]
   100	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   101	pub struct AssertionResult {
   102	    pub id: u32,
   103	    pub name: String,
   104	    pub layer: AssertionLayer,
   105	    pub result: AssertionVerdict,
   106	    pub detail: Option<String>,
   107	}
   108	
   109	impl AssertionResult {
   110	    fn pass(id: u32, name: &'static str, layer: AssertionLayer) -> Self {
   111	        Self {
   112	            id,
   113	            name: name.into(),
   114	            layer,
   115	            result: AssertionVerdict::Pass,
   116	            detail: None,
   117	        }
   118	    }
   119	    fn fail(id: u32, name: &'static str, layer: AssertionLayer, detail: String) -> Self {
   120	        Self {
   121	            id,
   122	            name: name.into(),
   123	            layer,
   124	            result: AssertionVerdict::Fail,
   125	            detail: Some(detail),
   126	        }
   127	    }
   128	    fn halt(id: u32, name: &'static str, layer: AssertionLayer, detail: String) -> Self {
   129	        Self {
   130	            id,
   131	            name: name.into(),
   132	            layer,
   133	            result: AssertionVerdict::Halt,
   134	            detail: Some(detail),
   135	        }
   136	    }
   137	    fn skipped(id: u32, name: &'static str, layer: AssertionLayer, detail: String) -> Self {
   138	        Self {
   139	            id,
   140	            name: name.into(),
   141	            layer,
   142	            result: AssertionVerdict::Skipped,
   143	            detail: Some(detail),
   144	        }
   145	    }
   146	}
   147	
   148	#[derive(Debug, Serialize, Deserialize, Clone)]
   149	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   150	pub struct TapeRoot {
   151	    pub l4_count: u64,
   152	    pub l4e_count: u64,
   153	    pub head_state_root_hex: String,
   154	    pub head_ledger_root_hex: String,
   155	    pub cas_object_count: u64,
   156	    pub constitution_hash_hex: String,
   157	}
   158	
   159	#[derive(Debug, Serialize, Deserialize, Clone, Default)]
   160	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   161	pub struct TxKindCounts {
   162	    pub work: u64,
   163	    pub verify: u64,
   164	    pub challenge: u64,
   165	    pub reuse: u64,
   166	    pub task_open: u64,
   167	    pub escrow_lock: u64,
   168	    pub complete_set_mint: u64,
   169	    pub complete_set_redeem: u64,
   170	    pub market_seed: u64,
   171	    pub finalize_reward: u64,
   172	    pub challenge_resolve: u64,
   173	    pub terminal_summary: u64,
   174	    pub task_expire: u64,
   175	    pub task_bankruptcy: u64,
   176	}
   177	
   178	impl TxKindCounts {
   179	    /// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   180	    pub fn from_entries(entries: &[LedgerEntry]) -> Self {
   181	        let mut c = Self::default();
   182	        for e in entries {
   183	            match e.tx_kind {
   184	                TxKind::Work => c.work += 1,
   185	                TxKind::Verify => c.verify += 1,
   186	                TxKind::Challenge => c.challenge += 1,
   187	                TxKind::Reuse => c.reuse += 1,
   188	                TxKind::TaskOpen => c.task_open += 1,
   189	                TxKind::EscrowLock => c.escrow_lock += 1,
   190	                TxKind::CompleteSetMint => c.complete_set_mint += 1,
   191	                TxKind::CompleteSetRedeem => c.complete_set_redeem += 1,
   192	                TxKind::MarketSeed => c.market_seed += 1,
   193	                TxKind::FinalizeReward => c.finalize_reward += 1,
   194	                TxKind::ChallengeResolve => c.challenge_resolve += 1,
   195	                TxKind::TerminalSummary => c.terminal_summary += 1,
   196	                TxKind::TaskExpire => c.task_expire += 1,
   197	                TxKind::TaskBankruptcy => c.task_bankruptcy += 1,
   198	            }
   199	        }
   200	        c
   201	    }
   202	    /// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   203	    pub fn missing_required(&self) -> Vec<&'static str> {
   204	        let mut missing = Vec::new();
   205	        let pairs: [(u64, &'static str); 13] = [
   206	            (self.work, "Work"),
   207	            (self.verify, "Verify"),
   208	            (self.challenge, "Challenge"),
   209	            (self.task_open, "TaskOpen"),
   210	            (self.escrow_lock, "EscrowLock"),
   211	            (self.complete_set_mint, "CompleteSetMint"),
   212	            (self.complete_set_redeem, "CompleteSetRedeem"),
   213	            (self.market_seed, "MarketSeed"),
   214	            (self.finalize_reward, "FinalizeReward"),
   215	            (self.challenge_resolve, "ChallengeResolve"),
   216	            (self.terminal_summary, "TerminalSummary"),
   217	            (self.task_expire, "TaskExpire"),
   218	            (self.task_bankruptcy, "TaskBankruptcy"),
   219	        ];
   220	        for (v, name) in pairs {
   221	            if v == 0 {
   222	                missing.push(name);
   223	            }
   224	        }
   225	        missing
   226	    }
   227	}
   228	
   229	#[derive(Debug, Serialize, Deserialize, Clone)]
   230	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   231	pub struct TapeAuditVerdict {
   232	    pub schema_version: String,
   233	    pub tape_root: TapeRoot,
   234	    pub tx_kind_counts: TxKindCounts,
   235	    pub assertions: Vec<AssertionResult>,
   236	    pub passed: u32,
   237	    pub failed: u32,
   238	    pub halted: u32,
   239	    pub skipped: u32,
   240	    pub feature_coverage: BTreeMap<String, String>,
   241	    pub verdict: String, // "PROCEED" | "BLOCK"
   242	}
   243	
   244	// ─────────────────────────────────────────────────────────────────────
   245	// Errors
   246	// ─────────────────────────────────────────────────────────────────────
   247	
   248	#[derive(Debug)]
   249	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   250	pub enum AuditError {
   251	    Io(std::io::Error),
   252	    PinnedManifest(String),
   253	    AgentManifest(String),
   254	    Cas(String),
   255	    L4eOpen(RejectionEvidenceError),
   256	    GenesisRead(String),
   257	    ConstitutionRead(String),
   258	    MarkovRead(String),
   259	    ReplayBlocked(String),
   260	}

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1975,2058p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1975	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1976	pub fn summarize_results(
  1977	    inputs: &AuditInputs,
  1978	    results: Vec<AssertionResult>,
  1979	) -> Result<TapeAuditVerdict, AuditError> {
  1980	    let tape = load_tape(inputs)?;
  1981	    let head = tape.entries.last();
  1982	    let head_state_root_hex = head
  1983	        .map(|e| hex_encode(&e.resulting_state_root.0))
  1984	        .unwrap_or_else(|| hex_encode(&tape.initial_q.state_root_t.0));
  1985	    let head_ledger_root_hex = head
  1986	        .map(|e| hex_encode(&e.resulting_ledger_root.0))
  1987	        .unwrap_or_else(|| hex_encode(&tape.initial_q.ledger_root_t.0));
  1988	    let tape_root = TapeRoot {
  1989	        l4_count: tape.entries.len() as u64,
  1990	        l4e_count: tape.l4e_writer.len() as u64,
  1991	        head_state_root_hex,
  1992	        head_ledger_root_hex,
  1993	        cas_object_count: tape.cas.len() as u64,
  1994	        constitution_hash_hex: hex_encode(&tape.constitution_hash.0),
  1995	    };
  1996	    let tx_kind_counts = TxKindCounts::from_entries(&tape.entries);
  1997	    let mut passed = 0u32;
  1998	    let mut failed = 0u32;
  1999	    let mut halted = 0u32;
  2000	    let mut skipped = 0u32;
  2001	    for r in &results {
  2002	        match r.result {
  2003	            AssertionVerdict::Pass => passed += 1,
  2004	            AssertionVerdict::Fail => failed += 1,
  2005	            AssertionVerdict::Halt => halted += 1,
  2006	            AssertionVerdict::Skipped => skipped += 1,
  2007	        }
  2008	    }
  2009	    let mut feature_coverage: BTreeMap<String, String> = BTreeMap::new();
  2010	    let cov = |present: bool| -> &'static str {
  2011	        if present { "GREEN" } else { "RED" }
  2012	    };
  2013	    let c = &tx_kind_counts;
  2014	    feature_coverage.insert("TB-1_monetary".into(), "GREEN".into());
  2015	    feature_coverage.insert("TB-2_work".into(), cov(c.work > 0).into());
  2016	    feature_coverage.insert("TB-3_task_open_escrow".into(), cov(c.task_open > 0 && c.escrow_lock > 0).into());
  2017	    feature_coverage.insert("TB-4_verify_challenge".into(), cov(c.verify > 0 && c.challenge > 0).into());
  2018	    feature_coverage.insert("TB-5_challenge_resolve".into(), cov(c.challenge_resolve > 0).into());
  2019	    feature_coverage.insert("TB-6_chain".into(), "GREEN".into());
  2020	    feature_coverage.insert("TB-7_agent_pubkeys".into(), "GREEN".into());
  2021	    feature_coverage.insert("TB-8_finalize_reward".into(), cov(c.finalize_reward > 0).into());
  2022	    feature_coverage.insert("TB-11_terminal_bankruptcy_expire".into(), cov(c.terminal_summary > 0 || c.task_bankruptcy > 0 || c.task_expire > 0).into());
  2023	    feature_coverage.insert("TB-13_complete_set".into(), cov(c.complete_set_mint > 0 || c.market_seed > 0).into());
  2024	    feature_coverage.insert("TB-14_price_mask".into(), "GREEN".into());
  2025	    feature_coverage.insert("TB-15_autopsy_markov".into(), cov(tape.markov_capsule.is_some()).into());
  2026	    let verdict = if failed == 0 && halted == 0 {
  2027	        "PROCEED".into()
  2028	    } else {
  2029	        "BLOCK".into()
  2030	    };
  2031	    Ok(TapeAuditVerdict {
  2032	        schema_version: "v1/audit_tape_verdict".into(),
  2033	        tape_root,
  2034	        tx_kind_counts,
  2035	        assertions: results,
  2036	        passed,
  2037	        failed,
  2038	        halted,
  2039	        skipped,
  2040	        feature_coverage,
  2041	        verdict,
  2042	    })
  2043	}
  2044	
  2045	#[cfg(test)]
  2046	mod tests {
  2047	    use super::*;
  2048	
  2049	    #[test]
  2050	    fn assertion_result_constructors_set_layer() {
  2051	        let p = AssertionResult::pass(1, "x", AssertionLayer::A);
  2052	        assert!(matches!(p.result, AssertionVerdict::Pass));
  2053	        let h = AssertionResult::halt(2, "y", AssertionLayer::F, "leak".into());
  2054	        assert!(matches!(h.result, AssertionVerdict::Halt));
  2055	    }
  2056	
  2057	    #[test]
  2058	    fn tx_kind_counts_missing_required_lists_all_thirteen_when_empty() {

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/comprehensive_arena.rs | sed -n '180,410p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   180	       --plan-only                Emit ARENA_PLAN.md only; do not subprocess-run\n\
   181	     \n\
   182	     EXIT:\n  \
   183	       0  — plan emitted (or all 6 tasks completed if not --plan-only)\n  \
   184	       2  — invalid args / I/O failure\n"
   185	        .into()
   186	}
   187	
   188	/// Sandbox preseed manifest emitted into ARENA_PLAN.md per design §4.
   189	fn sandbox_preseed_pairs() -> Vec<(&'static str, i64)> {
   190	    vec![
   191	        ("tb7-7-sponsor",  24_000_000),
   192	        ("Agent_user_0",    6_000_000),
   193	        ("Agent_solver_0",    100_000),
   194	        ("Agent_solver_1",    100_000),
   195	        ("Agent_solver_2",    100_000),
   196	        ("Agent_solver_3",    100_000),
   197	        ("Agent_verifier_0",  100_000),
   198	        // tb7-7-sponsor + Agent_user_0 + 4 solver + 1 verifier = 7 entries
   199	        // total = 24M + 6M + 4*0.1M + 0.1M = 30.5M? Adjust:
   200	        // Re-checked design §4: total is 30M to match default_pput_preseed_pairs.
   201	        // Architect spec keeps the 30M genesis on_init total. The above sums
   202	        // to 24+6+0.5 = 30.5M which is wrong; default_pput_preseed_pairs
   203	        // already provides the 30M baseline. comprehensive_arena does NOT
   204	        // mint new coin — it reuses default_pput_preseed_pairs verbatim.
   205	        // The "8 distinct agents" framing is a logical naming overlay
   206	        // (Agent_solver_0..3 maps onto Agent_0..3 from preseed; Agent_user_0
   207	        // is preseed; tb7-7-sponsor is preseed; verifier alias is preseed).
   208	    ]
   209	}
   210	
   211	#[derive(Debug, Clone)]
   212	struct TaskSpec {
   213	    label: &'static str,
   214	    description: &'static str,
   215	    sponsor: &'static str,
   216	    solver: &'static str,
   217	    challenger: Option<&'static str>,
   218	    expected_outcome: &'static str,
   219	    /// Architect-mandated tx kinds this task EXERCISES (acceptance shape).
   220	    exercises: &'static [&'static str],
   221	}
   222	
   223	fn arena_tasks() -> Vec<TaskSpec> {
   224	    vec![
   225	        TaskSpec {
   226	            label: "A_happy_path",
   227	            description: "trivial Lean theorem; solver_0 finds proof; verifier confirms",
   228	            sponsor: "tb7-7-sponsor",
   229	            solver: "Agent_solver_0",
   230	            challenger: None,
   231	            expected_outcome: "OmegaAccepted -> FinalizeReward",
   232	            exercises: &[
   233	                "TaskOpen", "EscrowLock", "Work", "Verify",
   234	                "FinalizeReward", "ProposalTelemetry", "VerificationResult",
   235	                "NodePosition(Long)",
   236	            ],
   237	        },
   238	        TaskSpec {
   239	            label: "B_challenge_dismissed",
   240	            description: "correct proof; solver_3 incorrectly challenges; verifier re-confirms",
   241	            sponsor: "tb7-7-sponsor",
   242	            solver: "Agent_solver_0",
   243	            challenger: Some("Agent_solver_3"),
   244	            expected_outcome: "ChallengeResolve(Released); challenger bond refunded",
   245	            exercises: &["Work", "Verify", "Challenge", "ChallengeResolve(Released)", "NodePosition(ChallengeShort)"],
   246	        },
   247	        TaskSpec {
   248	            label: "C_challenge_upheld",
   249	            description: "invalid proof; solver_3 correctly challenges; verifier confirms",
   250	            sponsor: "tb7-7-sponsor",
   251	            solver: "Agent_solver_0",
   252	            challenger: Some("Agent_solver_3"),
   253	            expected_outcome: "ChallengeResolve(UpheldDeferred); slash deferred to RSP-3.2",
   254	            exercises: &["Work", "Verify", "Challenge", "ChallengeResolve(UpheldDeferred)"],
   255	        },
   256	        TaskSpec {
   257	            label: "D_exhaustion",
   258	            description: "hard Lean theorem; solver_1 exhausts MAX_TX; bankruptcy triggers autopsy",
   259	            sponsor: "tb7-7-sponsor",
   260	            solver: "Agent_solver_1",
   261	            challenger: None,
   262	            expected_outcome: "TerminalSummary + EvidenceCapsule; TaskBankruptcy + AgentAutopsyCapsule",
   263	            exercises: &[
   264	                "TerminalSummary", "EvidenceCapsule", "TaskBankruptcy",
   265	                "AgentAutopsyCapsule",
   266	            ],
   267	        },
   268	        TaskSpec {
   269	            label: "E_expiry",
   270	            description: "sponsor opens; no solver picks up; deadline elapses",
   271	            sponsor: "tb7-7-sponsor",
   272	            solver: "(none)",
   273	            challenger: None,
   274	            expected_outcome: "TaskExpire; sponsor refund",
   275	            exercises: &["TaskOpen", "EscrowLock", "TaskExpire"],
   276	        },
   277	        TaskSpec {
   278	            label: "F_complete_set_market",
   279	            description: "Agent_user_0 sponsors; MarketSeed + CompleteSetMint + redeem",
   280	            sponsor: "Agent_user_0",
   281	            solver: "Agent_solver_2",
   282	            challenger: None,
   283	            expected_outcome: "MarketSeed + CompleteSetMint + (resolution) + CompleteSetRedeem",
   284	            exercises: &[
   285	                "MarketSeed", "CompleteSetMint", "CompleteSetRedeem",
   286	                "ConditionalCollateral", "ConditionalShareBalances",
   287	            ],
   288	        },
   289	    ]
   290	}
   291	
   292	fn write_arena_plan(cfg: &ArenaConfig) -> Result<PathBuf, std::io::Error> {
   293	    std::fs::create_dir_all(&cfg.out_dir)?;
   294	    let plan_path = cfg.out_dir.join("ARENA_PLAN.md");
   295	    let mut s = String::new();
   296	    s.push_str("# TB-16 Comprehensive Arena Plan\n\n");
   297	    s.push_str(&format!("**Run ID prefix**: `{}`\n", cfg.run_id_prefix));
   298	    s.push_str(&format!("**Out dir**: `{}`\n", cfg.out_dir.display()));
   299	    s.push_str(&format!("**Wall-clock cap**: {} ms ({} min)\n",
   300	        cfg.wall_clock_cap_ms, cfg.wall_clock_cap_ms / 60_000));
   301	    s.push_str(&format!("**Compute cap**: {} tokens\n", cfg.compute_cap_tokens));
   302	    s.push_str(&format!("**Cost ceiling**: ${}\n", cfg.cost_ceiling_usd));
   303	    s.push_str(&format!("**LLM proxy**: {}\n", cfg.llm_proxy_url));
   304	    s.push_str(&format!("**Max-tx per task**: {}\n\n", cfg.max_tx));
   305	
   306	    s.push_str("## Sandbox preseed (architect §7.4 CR-16.5 + CR-16.7)\n\n");
   307	    s.push_str("Reuses `runtime::bootstrap::default_pput_preseed_pairs()` (30_000_000 μC on_init mint).\n");
   308	    s.push_str("Agent IDs are sandbox-prefixed: `tb7-7-sponsor`, `Agent_user_0`,\n");
   309	    s.push_str("`Agent_solver_0..3`, `Agent_verifier_0`. Production-wallet patterns forbidden.\n\n");
   310	
   311	    s.push_str("## 6-Task plan (design §4)\n\n");
   312	    for (i, t) in arena_tasks().iter().enumerate() {
   313	        s.push_str(&format!("### Task {} — {}\n\n", i, t.label));
   314	        s.push_str(&format!("- **Description**: {}\n", t.description));
   315	        s.push_str(&format!("- **Sponsor**: {}\n", t.sponsor));
   316	        s.push_str(&format!("- **Solver**: {}\n", t.solver));
   317	        if let Some(c) = t.challenger {
   318	            s.push_str(&format!("- **Challenger**: {}\n", c));
   319	        }
   320	        s.push_str(&format!("- **Expected outcome**: {}\n", t.expected_outcome));
   321	        s.push_str("- **Exercises**:\n");
   322	        for ex in t.exercises {
   323	            s.push_str(&format!("    - `{}`\n", ex));
   324	        }
   325	        s.push('\n');
   326	    }
   327	
   328	    s.push_str("## Execution model\n\n");
   329	    s.push_str("Atom 5 (this binary) v0 scope: emit this plan + sandbox preseed manifest.\n");
   330	    s.push_str("Atom 6 (`handover/tests/scripts/run_real_llm_arena.sh`) executes the plan:\n");
   331	    s.push_str("1. Bootstrap a fresh `runtime_repo/` + `cas/` via `evaluator --bootstrap-only`.\n");
   332	    s.push_str("2. For each task A..F, subprocess `evaluator` with task-specific env vars\n");
   333	    s.push_str("   (`TURINGOS_USER_TASK_MODE`, `TURINGOS_USER_TASK_BOUNTY_MICRO`,\n");
   334	    s.push_str("   `TURINGOS_FORCE_CHALLENGE`, `TURINGOS_FORCE_EXHAUSTION`, etc.).\n");
   335	    s.push_str("3. After all 6 tasks complete, run `audit_tape` over the resulting tape.\n");
   336	    s.push_str("4. Run `audit_tape_tamper` (3 corruptions) over copies.\n");
   337	    s.push_str("5. Run `generate_markov_capsule` to emit MARKOV_TB-16_<DATE>.json.\n");
   338	    s.push_str("6. Run `audit_dashboard` to render dashboard.txt.\n");
   339	    s.push_str("7. Re-run `audit_tape` to assert byte-identical verdict.json.\n\n");
   340	
   341	    s.push_str("## Ship gate (design §7.1)\n\n");
   342	    s.push_str("PASS iff:\n");
   343	    s.push_str("1. Evaluator subprocess completes within 30-min wall clock + cost ceiling.\n");
   344	    s.push_str("2. All 13 expected tx_kinds appear in tape_root.tx_kind_counts.\n");
   345	    s.push_str("3. All 6 CAS object types reachable.\n");
   346	    s.push_str("4. verdict.json `verdict == \"PROCEED\"` with all 38 assertions PASS.\n");
   347	    s.push_str("5. Dashboard renders all 16 sections (incl. §15 live regen + §16 SANDBOX banner).\n");
   348	    s.push_str("6. First Markov capsule emitted; constitution_hash matches.\n");
   349	    s.push_str("7. Replay determinism: byte-identical verdict.json across two runs.\n\n");
   350	
   351	    s.push_str("## Forbidden (architect §7.6 verbatim)\n\n");
   352	    s.push_str("- No public chain. No real-money market. No external domain.\n");
   353	    s.push_str("- No unbounded leverage. No AMM trading. No DPMM / pro-rata.\n");
   354	    s.push_str("- No medical/legal/financial domains. No production user funds.\n\n");
   355	
   356	    s.push_str("## Halt triggers (architect §7.7)\n\n");
   357	    s.push_str("Instant stop (no round-2):\n");
   358	    s.push_str("- Conservation failure (Layer D #17/18/19/20).\n");
   359	    s.push_str("- Raw log leak (Layer F #28/29/30/31).\n");
   360	    s.push_str("- Price-as-truth (re-dispatch reads compute_price_index).\n");
   361	    s.push_str("- Non-sandbox funds used (production wallet pattern).\n");
   362	    s.push_str("- Unresolved evidence gap (CAS missing for any L4 CID).\n");
   363	    std::fs::write(&plan_path, s)?;
   364	    Ok(plan_path)
   365	}
   366	
   367	fn main() -> ExitCode {
   368	    let argv: Vec<String> = std::env::args().skip(1).collect();
   369	    let cfg = match ArenaConfig::from_args(&argv) {
   370	        Ok(c) => c,
   371	        Err(e) => {
   372	            eprintln!("comprehensive_arena: {e}\n\n{}", help_text());
   373	            return ExitCode::from(2);
   374	        }
   375	    };
   376	
   377	    // Always emit the plan first.
   378	    let plan_path = match write_arena_plan(&cfg) {
   379	        Ok(p) => p,
   380	        Err(e) => {
   381	            eprintln!("comprehensive_arena: write plan failed: {e}");
   382	            return ExitCode::from(2);
   383	        }
   384	    };
   385	    eprintln!("comprehensive_arena: plan emitted -> {plan_path:?}");
   386	    eprintln!("comprehensive_arena: sandbox preseed = {:?}", sandbox_preseed_pairs());
   387	
   388	    if cfg.plan_only {
   389	        eprintln!("comprehensive_arena: --plan-only set; skipping subprocess execution");
   390	        return ExitCode::from(0);
   391	    }
   392	
   393	    // Atom 5 v0: subprocess execution path is provided by Atom 6 shell
   394	    // script (handover/tests/scripts/run_real_llm_arena.sh). The Rust
   395	    // binary itself only emits the plan + invokes the script if present.
   396	    let script_path = PathBuf::from("handover/tests/scripts/run_real_llm_arena.sh");
   397	    if !script_path.exists() {
   398	        eprintln!(
   399	            "comprehensive_arena: Atom 6 runner script not yet present at {script_path:?}; \
   400	             plan emitted, no execution. Re-run with --plan-only or wait for Atom 6 ship."
   401	        );
   402	        return ExitCode::from(0);
   403	    }
   404	
   405	    eprintln!(
   406	        "comprehensive_arena: Atom 6 script present; this binary delegates execution to: \
   407	         bash {script_path:?} --out-dir {:?}",
   408	        cfg.out_dir
   409	    );
   410	    let mut cmd = std::process::Command::new("bash");

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/comprehensive_arena.rs | sed -n '410,470p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   410	    let mut cmd = std::process::Command::new("bash");
   411	    cmd.arg(&script_path)
   412	        .arg("--out-dir")
   413	        .arg(&cfg.out_dir)
   414	        .arg("--evaluator-bin")
   415	        .arg(&cfg.evaluator_bin)
   416	        .arg("--lean-market-bin")
   417	        .arg(&cfg.lean_market_bin)
   418	        .arg("--max-tx")
   419	        .arg(cfg.max_tx.to_string())
   420	        .arg("--llm-proxy-url")
   421	        .arg(&cfg.llm_proxy_url)
   422	        .arg("--run-id-prefix")
   423	        .arg(&cfg.run_id_prefix);
   424	    let status = match cmd.status() {
   425	        Ok(s) => s,
   426	        Err(e) => {
   427	            eprintln!("comprehensive_arena: spawn script failed: {e}");
   428	            return ExitCode::from(2);
   429	        }
   430	    };
   431	    if status.success() {
   432	        ExitCode::from(0)
   433	    } else {
   434	        ExitCode::from(status.code().unwrap_or(2) as u8)
   435	    }
   436	}

exec
/bin/bash -lc "nl -ba handover/tests/scripts/run_real_llm_arena.sh | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# TB-16 Atom 6 — run_real_llm_arena.sh
     3	#
     4	# End-to-end runner for the TB-16 Controlled Market Smoke Arena per
     5	# architect §7 + design §4. Drives 6 Lean tasks across 8 sandbox-prefixed
     6	# agents, then runs the full audit pipeline (audit_tape +
     7	# audit_tape_tamper + generate_markov_capsule + audit_dashboard).
     8	#
     9	# Per design §5 — preconditions:
    10	#   - DeepSeek API keys present in turingosv4/.env (DEEPSEEK_API_KEY*)
    11	#   - src/drivers/llm_proxy.py running on http://localhost:18080
    12	#     (or pass --llm-proxy-url)
    13	#   - Mathlib cached: lake exe cache get (~2 min) per
    14	#     feedback_lake_packages_vendored
    15	#   - Wall clock budget: 30 min (1800s)
    16	#   - Cost ceiling: $15 USD
    17	#
    18	# Exit 0 — verdict.json PROCEED + replay byte-identical
    19	# Exit 1 — verdict.json BLOCK (≥1 fail/halt) OR replay diverged
    20	# Exit 2 — invalid args / preconditions / I/O failure
    21	#
    22	# TRACE_MATRIX FC1-N34 + FC1-N35 + FC1-N36 + FC2-N31..N33 + FC3-N44.
    23	
    24	set -euo pipefail
    25	
    26	# ── Defaults ────────────────────────────────────────────────────────
    27	OUT_DIR=""
    28	EVALUATOR_BIN="./target/release/evaluator"
    29	LEAN_MARKET_BIN="./target/release/lean_market"
    30	ARENA_BIN="./target/release/comprehensive_arena"
    31	AUDIT_TAPE_BIN="./target/release/audit_tape"
    32	AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
    33	AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"
    34	GEN_MARKOV_BIN="./target/release/generate_markov_capsule"
    35	LLM_PROXY_URL="http://localhost:18080"
    36	MAX_TX="20"
    37	RUN_ID_PREFIX="tb16-arena-$(date -u +%Y-%m-%dT%H-%M-%SZ)"
    38	WALL_CLOCK_CAP_MS="1800000"
    39	COMPUTE_CAP_TOKENS="120000"
    40	COST_CEILING_USD="15"
    41	SKIP_LLM_PRECHECK="${SKIP_LLM_PRECHECK:-0}"
    42	PLAN_ONLY="0"
    43	
    44	# ── Args ────────────────────────────────────────────────────────────
    45	while [[ $# -gt 0 ]]; do
    46	  case "$1" in
    47	    --out-dir) OUT_DIR="$2"; shift 2 ;;
    48	    --evaluator-bin) EVALUATOR_BIN="$2"; shift 2 ;;
    49	    --lean-market-bin) LEAN_MARKET_BIN="$2"; shift 2 ;;
    50	    --llm-proxy-url) LLM_PROXY_URL="$2"; shift 2 ;;
    51	    --max-tx) MAX_TX="$2"; shift 2 ;;
    52	    --run-id-prefix) RUN_ID_PREFIX="$2"; shift 2 ;;
    53	    --plan-only) PLAN_ONLY="1"; shift ;;
    54	    -h|--help)
    55	      cat <<'EOF'
    56	run_real_llm_arena.sh — TB-16 Atom 6 controlled-market arena runner
    57	
    58	USAGE:
    59	  bash handover/tests/scripts/run_real_llm_arena.sh \
    60	       --out-dir <path> \
    61	       [--evaluator-bin <path>] \
    62	       [--lean-market-bin <path>] \
    63	       [--llm-proxy-url <url>] \
    64	       [--max-tx <n>] \
    65	       [--run-id-prefix <str>] \
    66	       [--plan-only]
    67	
    68	PRECONDITIONS:
    69	  - DEEPSEEK_API_KEY (1+ keys) in env
    70	  - LLM proxy running at --llm-proxy-url (or set SKIP_LLM_PRECHECK=1)
    71	  - Mathlib cached (`lake exe cache get`)
    72	
    73	OUTPUTS in --out-dir:
    74	  - ARENA_PLAN.md            (orchestration plan)
    75	  - runtime_repo/            (Git2 L4 chain + L4.E rejections.jsonl)
    76	  - cas/                     (CAS objects)
    77	  - agent_pubkeys.json       (per-run agent manifest)
    78	  - pinned_pubkeys.json      (per-run system pubkey manifest)
    79	  - genesis_report.json      (constitution_hash + bootstrap state)
    80	  - verdict.json             (38-assertion audit verdict)
    81	  - verdict_replay.json      (byte-identical re-run verdict)
    82	  - tamper_report.json       (3-corruption tamper-detection report)
    83	  - MARKOV_TB-16_<DATE>.json (first Markov capsule)
    84	  - dashboard.txt            (15-section render incl. §15 + §16)
    85	  - README.md                (acceptance gate table + halt-trigger battery)
    86	EOF
    87	      exit 0
    88	      ;;
    89	    *) echo "run_real_llm_arena.sh: unknown arg: $1" >&2; exit 2 ;;
    90	  esac
    91	done
    92	
    93	if [[ -z "$OUT_DIR" ]]; then
    94	  echo "run_real_llm_arena.sh: --out-dir required" >&2
    95	  exit 2
    96	fi
    97	mkdir -p "$OUT_DIR"
    98	
    99	# ── Step 1: Build all binaries (release) ────────────────────────────
   100	echo "▶ Step 1/8: cargo build --release (audit + arena + evaluator + dashboard)..."
   101	cargo build --release \
   102	  --bin audit_tape \
   103	  --bin audit_tape_tamper \
   104	  --bin audit_dashboard \
   105	  --bin generate_markov_capsule
   106	cargo build --release -p minif2f_v4 \
   107	  --bin comprehensive_arena \
   108	  --bin evaluator \
   109	  --bin lean_market
   110	
   111	# ── Step 2: Emit ARENA_PLAN.md (always) ─────────────────────────────
   112	echo "▶ Step 2/8: emit ARENA_PLAN.md..."
   113	"$ARENA_BIN" --out-dir "$OUT_DIR" --plan-only \
   114	  --max-tx "$MAX_TX" \
   115	  --run-id-prefix "$RUN_ID_PREFIX" \
   116	  --llm-proxy-url "$LLM_PROXY_URL"
   117	
   118	if [[ "$PLAN_ONLY" == "1" ]]; then
   119	  echo "✓ Plan-only mode; ARENA_PLAN.md emitted at $OUT_DIR/ARENA_PLAN.md"
   120	  exit 0
   121	fi
   122	
   123	# ── Step 3: LLM proxy precheck ──────────────────────────────────────
   124	if [[ "$SKIP_LLM_PRECHECK" != "1" ]]; then
   125	  echo "▶ Step 3/8: LLM proxy precheck against $LLM_PROXY_URL..."
   126	  if ! curl -sf -o /dev/null --max-time 5 "$LLM_PROXY_URL/health" 2>/dev/null \
   127	     && ! curl -sf -o /dev/null --max-time 5 "$LLM_PROXY_URL" 2>/dev/null; then
   128	    cat >&2 <<EOF
   129	✗ LLM proxy NOT REACHABLE at $LLM_PROXY_URL.
   130	  Start it with:
   131	    python3 src/drivers/llm_proxy.py --port 18080 &
   132	  Or set SKIP_LLM_PRECHECK=1 to bypass (real-LLM flow will be SKIPPED;
   133	  audit pipeline still runs against any pre-existing tape in --out-dir).
   134	EOF
   135	    echo "✗ Precondition failed; aborting before any subprocess work." >&2
   136	    exit 2
   137	  fi
   138	  echo "✓ LLM proxy reachable."
   139	fi
   140	
   141	# ── Step 4: Drive evaluator subprocesses for 6 tasks ────────────────
   142	# v0 implementation: evaluator's existing real-LLM solver loop is
   143	# invoked via lean_market run-task semantics. The 6-task scenario is
   144	# exercised by 6 sequential evaluator invocations against a SHARED
   145	# runtime_repo. Adversarial-challenger overrides + force-exhaustion
   146	# overrides flow via env vars (TURINGOS_FORCE_*).
   147	#
   148	# Atom 6 v0 SCOPE NOTE: each task currently maps to a single evaluator
   149	# invocation in user-task mode. Multi-task aggregation onto a single
   150	# chain (so all 13 tx kinds appear in ONE tape) requires evaluator
   151	# extensions (TB-16 Atom 6.1). For v0, each task produces its own
   152	# sub-tape under $OUT_DIR/task_<X>_<label>/runtime_repo, and audit_tape
   153	# runs over each tape individually + emits an aggregate report.
   154	
   155	RUNTIME_REPO="$OUT_DIR/runtime_repo"
   156	CAS_DIR="$OUT_DIR/cas"
   157	mkdir -p "$RUNTIME_REPO" "$CAS_DIR"
   158	
   159	echo "▶ Step 4/8: real-LLM 6-task arena execution..."
   160	echo "  (Atom 6 v0: per-task sub-tapes + aggregate audit)"
   161	echo "  (To exercise the FULL multi-task single-chain coverage path,"
   162	echo "   extend evaluator with TURINGOS_TASK_LIST=A,B,C,D,E,F + chain"
   163	echo "   continuation semantics — TB-16 Atom 6.1 follow-up.)"
   164	
   165	# Single-task minimal-coverage smoke (Task A happy_path equivalent).
   166	# This produces a chain-backed tape with at minimum:
   167	#   TaskOpen + EscrowLock + Work + Verify + (FinalizeReward if accepted)
   168	# Coverage of the remaining tx kinds requires additional evaluator
   169	# extensions (challenge injection, force-exhaustion, MarketSeed entry).
   170	
   171	TASK_A_DIR="$OUT_DIR/task_A_happy_path"
   172	mkdir -p "$TASK_A_DIR"
   173	echo "  Task A: happy_path (mathd_algebra_171; bounty 200_000μC)..."
   174	TURINGOS_USER_TASK_MODE=1 \
   175	TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
   176	TURINGOS_CHAINTAPE_PATH="$TASK_A_DIR/runtime_repo" \
   177	TURINGOS_CAS_PATH="$TASK_A_DIR/cas" \
   178	TURINGOS_RUN_ID="${RUN_ID_PREFIX}-A" \
   179	LLM_PROXY_URL="$LLM_PROXY_URL" \
   180	MAX_TRANSACTIONS="$MAX_TX" \
   181	"$EVALUATOR_BIN" \
   182	  --task-mode user \
   183	  --problem mathd_algebra_171 \
   184	  --max-transactions "$MAX_TX" \
   185	  || { echo "✗ Task A evaluator failed (continue to next task)"; }
   186	
   187	# (Task B-F would follow same pattern; deferred to evaluator extensions.)
   188	
   189	# ── Step 5: Run audit_tape over the produced tape ───────────────────
   190	echo "▶ Step 5/8: audit_tape over Task A tape..."
   191	"$AUDIT_TAPE_BIN" \
   192	  --runtime-repo "$TASK_A_DIR/runtime_repo" \
   193	  --cas-dir "$TASK_A_DIR/cas" \
   194	  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
   195	  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
   196	  --genesis genesis_payload.toml \
   197	  --constitution constitution.md \
   198	  --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
   199	  --alignment-dir handover/alignment \
   200	  --out "$OUT_DIR/verdict.json" || true
   201	
   202	# ── Step 6: audit_tape_tamper ───────────────────────────────────────
   203	echo "▶ Step 6/8: audit_tape_tamper (3-corruption smoke)..."
   204	"$AUDIT_TAPE_TAMPER_BIN" \
   205	  --runtime-repo "$TASK_A_DIR/runtime_repo" \
   206	  --cas-dir "$TASK_A_DIR/cas" \
   207	  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
   208	  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
   209	  --genesis genesis_payload.toml \
   210	  --constitution constitution.md \
   211	  --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
   212	  --alignment-dir handover/alignment \
   213	  --tamper-dir "$OUT_DIR/tamper" \
   214	  --out "$OUT_DIR/tamper_report.json" || true
   215	
   216	# ── Step 7: generate_markov_capsule ─────────────────────────────────
   217	echo "▶ Step 7/8: generate_markov_capsule (TB-16)..."
   218	"$GEN_MARKOV_BIN" \
   219	  --tb-id 16 \
   220	  --out-dir "$OUT_DIR" \
   221	  --constitution-path constitution.md \
   222	  --runtime-repo "$TASK_A_DIR/runtime_repo" \
   223	  --cas-dir "$TASK_A_DIR/cas" \
   224	  --alignment-dir handover/alignment || true
   225	
   226	# ── Step 8: audit_dashboard ─────────────────────────────────────────
   227	echo "▶ Step 8/8: audit_dashboard..."
   228	"$AUDIT_DASHBOARD_BIN" \
   229	  --repo "$TASK_A_DIR/runtime_repo" \
   230	  --cas "$TASK_A_DIR/cas" \
   231	  --out "$OUT_DIR/dashboard.txt" || true
   232	
   233	# ── Replay determinism check ────────────────────────────────────────
   234	echo "▶ Replay determinism: re-running audit_tape..."
   235	"$AUDIT_TAPE_BIN" \
   236	  --runtime-repo "$TASK_A_DIR/runtime_repo" \
   237	  --cas-dir "$TASK_A_DIR/cas" \
   238	  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
   239	  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
   240	  --genesis genesis_payload.toml \
   241	  --constitution constitution.md \
   242	  --markov-pointer handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt \
   243	  --alignment-dir handover/alignment \
   244	  --out "$OUT_DIR/verdict_replay.json" || true
   245	
   246	# ── Final summary ───────────────────────────────────────────────────
   247	echo ""
   248	echo "═══════════════════════════════════════════════════════════════════"
   249	echo "TB-16 ARENA RUN COMPLETE"
   250	echo "═══════════════════════════════════════════════════════════════════"
   251	echo "Out dir        : $OUT_DIR"
   252	echo "Plan           : $OUT_DIR/ARENA_PLAN.md"
   253	echo "Verdict        : $OUT_DIR/verdict.json"
   254	echo "Verdict replay : $OUT_DIR/verdict_replay.json"
   255	echo "Tamper report  : $OUT_DIR/tamper_report.json"
   256	echo "Dashboard      : $OUT_DIR/dashboard.txt"
   257	if [[ -f "$OUT_DIR/verdict.json" && -f "$OUT_DIR/verdict_replay.json" ]]; then
   258	  V1=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict.json" | head -1)
   259	  V2=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict_replay.json" | head -1)
   260	  echo "Verdict-1      : $V1"

exec
/bin/bash -lc "nl -ba handover/tests/scripts/audit_tape_smoke_test.sh | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	#!/usr/bin/env bash
     2	# TB-16 Atom 6 — audit_tape_smoke_test.sh
     3	#
     4	# Wrapper around run_real_llm_arena.sh that asserts:
     5	#   1. verdict.json contains verdict="PROCEED"
     6	#   2. All 13 expected tx_kinds appear in tape_root.tx_kind_counts
     7	#      (when the underlying tape exercises them — Atom 6 v0 ships with
     8	#      Task-A-only coverage; full 13-kind coverage requires the
     9	#      multi-task evaluator extension noted as TB-16 Atom 6.1)
    10	#   3. tamper_report.json reports detected_count == 3
    11	#   4. verdict.json and verdict_replay.json are byte-identical
    12	#
    13	# Usage:
    14	#   bash handover/tests/scripts/audit_tape_smoke_test.sh \
    15	#        --out-dir <existing-evidence-dir-from-run_real_llm_arena.sh>
    16	#
    17	# Exit codes:
    18	#   0  — all asserts pass (PROCEED + 13/13 tx kinds + 3/3 tamper +
    19	#        replay byte-identical)
    20	#   1  — at least one assert fails
    21	#   2  — invalid args / missing inputs
    22	
    23	set -euo pipefail
    24	
    25	OUT_DIR=""
    26	REQUIRE_FULL_COVERAGE="${REQUIRE_FULL_COVERAGE:-0}"
    27	
    28	while [[ $# -gt 0 ]]; do
    29	  case "$1" in
    30	    --out-dir) OUT_DIR="$2"; shift 2 ;;
    31	    --require-full-coverage) REQUIRE_FULL_COVERAGE="1"; shift ;;
    32	    -h|--help)
    33	      cat <<'EOF'
    34	audit_tape_smoke_test.sh — TB-16 Atom 6 ship-gate wrapper
    35	
    36	USAGE:
    37	  bash handover/tests/scripts/audit_tape_smoke_test.sh --out-dir <path>
    38	
    39	ENV:
    40	  REQUIRE_FULL_COVERAGE=1   Fail if any of 13 architect-required tx kinds
    41	                            absent. Default: warn only (Atom 6 v0 ships
    42	                            with Task-A-only coverage; full 13-kind path
    43	                            needs evaluator multi-task extension).
    44	
    45	EXIT:
    46	  0  all asserts PASS
    47	  1  at least one assert FAIL
    48	  2  missing inputs
    49	EOF
    50	      exit 0
    51	      ;;
    52	    *) echo "audit_tape_smoke_test: unknown arg: $1" >&2; exit 2 ;;
    53	  esac
    54	done
    55	
    56	if [[ -z "$OUT_DIR" ]]; then
    57	  echo "audit_tape_smoke_test: --out-dir required" >&2; exit 2
    58	fi
    59	if [[ ! -d "$OUT_DIR" ]]; then
    60	  echo "audit_tape_smoke_test: --out-dir $OUT_DIR not a directory" >&2; exit 2
    61	fi
    62	
    63	VERDICT="$OUT_DIR/verdict.json"
    64	VERDICT_REPLAY="$OUT_DIR/verdict_replay.json"
    65	TAMPER="$OUT_DIR/tamper_report.json"
    66	
    67	failed=0
    68	warn=0
    69	
    70	# Assert 1: verdict.json exists + verdict==PROCEED
    71	if [[ ! -f "$VERDICT" ]]; then
    72	  echo "✗ verdict.json missing at $VERDICT"; failed=$((failed+1))
    73	else
    74	  V=$(python3 -c "import json,sys; print(json.load(open('$VERDICT'))['verdict'])")
    75	  if [[ "$V" == "PROCEED" ]]; then
    76	    echo "✓ verdict.json verdict=PROCEED"
    77	  else
    78	    echo "✗ verdict.json verdict=$V (expected PROCEED)"; failed=$((failed+1))
    79	  fi
    80	fi
    81	
    82	# Assert 2: all 13 tx kinds present
    83	if [[ -f "$VERDICT" ]]; then
    84	  MISSING=$(python3 - <<PYEOF
    85	import json
    86	v = json.load(open("$VERDICT"))
    87	c = v.get("tx_kind_counts", {})
    88	required = ["work","verify","challenge","task_open","escrow_lock",
    89	            "complete_set_mint","complete_set_redeem","market_seed",
    90	            "finalize_reward","challenge_resolve","terminal_summary",
    91	            "task_expire","task_bankruptcy"]
    92	missing = [k for k in required if c.get(k, 0) == 0]
    93	print(",".join(missing))
    94	PYEOF
    95	  )
    96	  if [[ -z "$MISSING" ]]; then
    97	    echo "✓ all 13 tx kinds present in tape"
    98	  else
    99	    if [[ "$REQUIRE_FULL_COVERAGE" == "1" ]]; then
   100	      echo "✗ missing tx kinds: $MISSING"; failed=$((failed+1))
   101	    else
   102	      echo "⚠ missing tx kinds (Atom 6 v0 expected): $MISSING"; warn=$((warn+1))
   103	    fi
   104	  fi
   105	fi
   106	
   107	# Assert 3: tamper detected 3/3
   108	if [[ ! -f "$TAMPER" ]]; then
   109	  echo "⚠ tamper_report.json missing"; warn=$((warn+1))
   110	else
   111	  D=$(python3 -c "import json; print(json.load(open('$TAMPER'))['detected_count'])")
   112	  if [[ "$D" == "3" ]]; then
   113	    echo "✓ tamper detection 3/3"
   114	  else
   115	    echo "✗ tamper detection $D/3"; failed=$((failed+1))
   116	  fi
   117	fi
   118	
   119	# Assert 4: replay byte-identical
   120	if [[ -f "$VERDICT" && -f "$VERDICT_REPLAY" ]]; then
   121	  if cmp -s "$VERDICT" "$VERDICT_REPLAY"; then
   122	    echo "✓ replay byte-identical"
   123	  else
   124	    echo "✗ replay diverged"; failed=$((failed+1))
   125	  fi
   126	else
   127	  echo "⚠ replay verdict missing; skipping byte-identity assert"; warn=$((warn+1))
   128	fi
   129	
   130	echo ""
   131	echo "═══════════════════════════════════════════════════════════════════"
   132	echo "audit_tape_smoke_test: failed=$failed warn=$warn"
   133	echo "═══════════════════════════════════════════════════════════════════"
   134	exit $failed

exec
/bin/bash -lc "nl -ba handover/tests/scripts/run_real_llm_arena.sh | sed -n '257,340p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   257	if [[ -f "$OUT_DIR/verdict.json" && -f "$OUT_DIR/verdict_replay.json" ]]; then
   258	  V1=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict.json" | head -1)
   259	  V2=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict_replay.json" | head -1)
   260	  echo "Verdict-1      : $V1"
   261	  echo "Verdict-2      : $V2"
   262	  if cmp -s "$OUT_DIR/verdict.json" "$OUT_DIR/verdict_replay.json"; then
   263	    echo "Replay         : ✓ byte-identical"
   264	  else
   265	    echo "Replay         : ✗ DIVERGED (HALT)"
   266	  fi
   267	fi

exec
/bin/bash -lc 'wc -l handover/tests/scripts/run_real_llm_arena.sh' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
267 handover/tests/scripts/run_real_llm_arena.sh

codex
The runner itself confirms the fresh arena is not shipped as a single 13-kind chain: it only runs Task A, downgrades missing tx kinds to warnings by default, and the main runner does not return nonzero after a bad verdict or replay divergence. I’m now checking the shipped evidence and status wording against that behavior.
exec
/bin/bash -lc "nl -ba handover/tracer_bullets/TB-16_charter_2026-05-04.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-16 Charter — Controlled Market Smoke Arena
     2	
     3	**Status**: RATIFIED 2026-05-04 (architect spec verbatim per `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7).
     4	**Date**: 2026-05-04.
     5	**Predecessor**: TB-15 SHIPPED 2026-05-03 (`2337381` + R3 closure `eddab36`).
     6	**Authority**: architect §7 (operative spec; FR-16.1..7 + CR-16.1..7 + SG-16.1..8 + forbidden list + halt triggers + Class 3 envelope).
     7	**Implementation contract (design)**: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md` (38 audit assertions; 6-task scenario; audit-from-tape contract).
     8	**User authorization (autonomous loop mode)**: 2026-05-04 — *"proceed to TB-16 implementation with architect's directive, auto mode on until dual audit and real comprehensive LLM smoke test passed"*.
     9	**Phase**: **P4 Information Loom** (capacity exit — CompleteSet redeem + autopsy emission run end-to-end on a chain-backed multi-agent run) + **P6 Multi-Org Epistemic Lab v0 prep** (3+ agents in a single sandbox arena = pre-multi-org rehearsal). NOT P7 (no real-world; no public chain; sandbox only).
    10	**Risk class envelope**: **Class 3 integration smoke** (architect §7.7 verbatim). Spans production-wire-up code paths (sequencer dispatch + git2 chain + LLM solver attestation + dashboard regen) → external dual audit MANDATORY at ship per `feedback_dual_audit` + `feedback_risk_class_audit`.
    11	**Iteration cap**: 24h per atom for non-spec atoms; production wire-up exception = 72h-to-feedback-loop on Atom 5+6 per `feedback_iteration_cap_24h` (the real-LLM run IS the feedback loop).
    12	
    13	**FC-trace**: `Art.0.2` (Tape Canonical — audit verdict reproducible from L4 + L4.E + CAS only) + `Art.I.1` (cargo workspace tests pass) + `Art.II.1` (typical-error broadcast under N≥3 cluster threshold reused from TB-15) + `Art.III.1..4` (privacy contracts on autopsy private_detail; raw failure shielding; broadcast pollution prevention; Goodhart shield) + `Art.IV` (terminal anchor distribution: OmegaAccepted / MaxTxExhausted / ChallengeUpheldDeferred / TaskExpired / TaskBankrupted) + `Art.V.1` (Generator ≠ Evaluator: comprehensive_arena drives evaluator; SEPARATE audit_tape binary derives verdict; auditor never reads live process state).
    14	
    15	**Flowchart-trace**:
    16	- **Flowchart 1 (runtime)**: `FC1-N34` = `audit_tape` binary (NEW; reads runtime_repo + cas_dir + bootstrap files; emits `verdict.json`). `FC1-N35` = `audit_tape_tamper` harness (NEW; tamper-detection harness over copies). `FC1-N36` = `comprehensive_arena` evaluator (NEW; orchestrates 6-task scenario across multi-agent + adversarial-challenger pool).
    17	- **Flowchart 2 (signal)**: `FC2-N31` = audit verdict.json schema v1 (NEW; 38 assertions × 8 layers). `FC2-N32` = dashboard §15 live regen (closes OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16 by walking replayed `agent_autopsies_t`). `FC2-N33` = dashboard §16 sandbox banner (CR-16.7 / SG-16.8).
    18	- **Flowchart 3 (meta)**: `FC3-N44` = real-LLM-driven Markov capsule (first MARKOV_TB-16_2026-05-04.json with non-empty `typical_errors` because TaskBankruptcyTx fires in Task D → autopsies clustered).
    19	
    20	**Phase declarations** (per `feedback_tb_phase_tag_required`):
    21	
    22	```text
    23	phase_id: P4 Information Loom (capacity exit — first end-to-end run that
    24	                               emits all 13 shipped tx kinds + 6 CAS object
    25	                               types in a single chain-backed multi-agent
    26	                               arena; first cluster-eligible cluster of
    27	                               autopsies because Task D forces ≥1 bankruptcy)
    28	          + P6 Multi-Org Epistemic Lab v0 prep (3+ agents in a single
    29	                               sandbox; sandbox-labeled IDs; multi-org gate
    30	                               is TB-17+ — TB-16 is the rehearsal)
    31	
    32	roadmap_exit_criteria_addressed:
    33	  P1-Exit8  state.db deletable; rebuild from L4 alone reproduces head
    34	            state_root. audit_tape binary EXERCISES this exit criterion
    35	            as a system-level acceptance gate (Layer C — replay determinism,
    36	            assertions 12+13+16). MUST PASS.
    37	  P4-Exit1  Single-Agent failure → local error (not global broadcast).
    38	            Carry-forward from TB-15 verified end-to-end across real LLM
    39	            run: Task D solver_1 exhaustion → autopsy in agent_autopsies_t
    40	            for solver_1 only; never in solver_0/2/3 AgentVisibleProjection.
    41	            Layer F assertions 28+29+30+31. MUST PASS.
    42	  P4-Exit2  Multiple Agents same-class failure → typical-error cluster.
    43	            Task D forces ≥3 same-class (Bankruptcy) autopsies (per
    44	            stake-distribution; design §4 specifies ≥3 stakers on the
    45	            bankrupted task). cluster_autopsies(threshold=3) emits a
    46	            TypicalErrorSummary embedded in the Markov capsule. SG-16.7
    47	            verifies. MUST PASS.
    48	  P4-Exit3  Broadcast = abstract rule, not raw failure log.
    49	            cluster_autopsies output contains ONLY public_summary strings;
    50	            never private_detail_cid bytes. Layer F #30 (deserialize
    51	            TypicalErrorSummary; assert no Cid match). MUST PASS.
    52	  P6-Exit1  Multi-org rehearsal in sandbox: 4 solver agents + 1 verifier
    53	            + 1 special CompleteSet operator + 2 sponsors = 8 distinct
    54	            agent-key pairs in a single arena. Sandbox flag in agent IDs
    55	            ("Agent_solver_*" / "tb7-7-sponsor" prefixes; CR-16.7
    56	            sandbox-labeled). Real multi-org (separate cryptographic
    57	            roots-of-trust per org) is P6 v1 / TB-17+; TB-16 is the
    58	            single-org rehearsal under one trust_root.
    59	  P5-prep   First Markov capsule with non-empty typical_errors (TB-15
    60	            ship-time capsule had typical_errors=[] because no ship-time
    61	            bankruptcy). TB-16's capsule is the FIRST cluster-eligible
    62	            Markov head. Layer G #33 verifies recomputation determinism.
    63	
    64	kill_criteria_tested:
    65	  conservation_failure
    66	            Layer D #17 (assert_no_post_init_mint over every accepted tx)
    67	            + #18 (total_supply_micro constant across all L4 rows) +
    68	            #19 (CompleteSet MIN-balanced invariant) + #20 (cache=truth
    69	            for task_markets_t.total_escrow). HALT on any failure.
    70	  raw_log_leak
    71	            Layer F #28 (AgentVisibleProjection contains no autopsy/
    72	            private_detail bytes) + #29 (private_detail creator is
    73	            sequencer-epoch only) + #30 (TypicalErrorSummary contains no
    74	            private_detail_cid) + #31 (AutopsyIndex value type is Vec<Cid>
    75	            structural fence). HALT on any leak.
    76	  price_as_truth
    77	            Layer E #25 (every L4.E rejection_class re-derivable from
    78	            re-dispatch — no path through compute_price_index changes
    79	            the rejection class) + #26 (PriceIndex is derived view only,
    80	            never an input to dispatch). architect §7.7 explicit halt.
    81	  non_sandbox_funds_used
    82	            Bootstrap preseed agent IDs match sandbox prefix pattern
    83	            ("Agent_solver_*" / "Agent_verifier_*" / "Agent_user_*" /
    84	            "tb7-7-sponsor"); audit_tape Layer A asserts no production-
    85	            wallet pattern. CR-16.5 + CR-16.7 + SG-16.8.
    86	  unresolved_evidence_gap
    87	            Layer B #9 (every tx_payload_cid resolves in CAS) +
    88	            Layer E #24 (every accepted WorkTx has reachable
    89	            ProposalTelemetry → VerificationResult) + #27 (every
    90	            TerminalSummaryTx has reachable EvidenceCapsule). HALT on
    91	            unresolved CID per architect §7.7.
    92	
    93	flowchart_trace:
    94	  Flowchart 1 (runtime):
    95	    FC1-N34 = audit_tape binary (38 assertions; verdict.json output;
    96	                forbidden inputs: live Sequencer / state.db / process
    97	                logs / handover/ai-direct/).
    98	    FC1-N35 = audit_tape_tamper harness (3 temp copies; flip + remove
    99	                + flip-cas; assert TamperDetected on each).
   100	    FC1-N36 = comprehensive_arena evaluator orchestrator (6-task
   101	                scenario; multi-agent + adversarial; sandbox-labeled).
   102	  Flowchart 2 (signal):
   103	    FC2-N31 = verdict.json schema v1 (8 layers × 38 assertions;
   104	                tape_root + tx_kind_counts + assertion table +
   105	                feature_coverage + verdict).
   106	    FC2-N32 = dashboard §15 live regen (replay_full_transition →
   107	                replayed_econ.agent_autopsies_t → autopsy_event_counts).
   108	    FC2-N33 = dashboard §16 sandbox banner (SANDBOX-RUN — NOT
   109	                PRODUCTION — NO REAL FUNDS — SOURCE: tb_16_real_llm_arena_*).
   110	  Flowchart 3 (meta):
   111	    FC3-N44 = real-LLM-driven MARKOV_TB-16_2026-05-04 capsule
   112	                (first capsule with non-empty typical_errors;
   113	                previous_capsule_cid chains to TB-15 capsule).
   114	```
   115	
   116	---
   117	
   118	## §0 Why TB-16 exists (architect §7 — verbatim)
   119	
   120	```text
   121	TB-16 — Controlled Market Smoke Arena
   122	
   123	7.1 目标
   124	在受控沙盒中跑通：
   125	compute + position + complete set + price + mask + autopsy
   126	仍不开放真实市场。
   127	
   128	7.2 Scenario
   129	Lean task
   130	multiple Agents
   131	WorkTx FirstLong
   132	ChallengeTx Short
   133	CompleteSet share inventory
   134	PriceIndex updates
   135	Boltzmann scheduler selects next candidate
   136	some agents lose positions
   137	Autopsy generated
   138	
   139	7.7 Loop-mode instruction
   140	Risk class: Class 3 integration smoke
   141	AI coder may implement autonomously, but ship requires external audit.
   142	Halt if:
   143	  any conservation failure;
   144	  raw log leak;
   145	  price-as-truth behavior;
   146	  non-sandbox funds used;
   147	  unresolved evidence gap.
   148	```
   149	
   150	---
   151	
   152	## §1 One-line goal
   153	
   154	```text
   155	Goal: Drive a single real-LLM evaluator run over 6 engineered Lean
   156	      tasks that exercises EVERY shipped tx type (TB-1..TB-15: Work +
   157	      Verify + Challenge + TaskOpen + EscrowLock + CompleteSetMint +
   158	      CompleteSetRedeem + MarketSeed + FinalizeReward + ChallengeResolve
   159	      + TerminalSummary + TaskExpire + TaskBankruptcy = 13) on a
   160	      multi-agent (≥3) Lean-proof market. Persist the run as a
   161	      chain-backed ChainTape (Sequencer::apply_one + on-disk
   162	      LedgerEntry chain + L4.E rejection ledger + CAS objects). Then
   163	      run a SEPARATE audit_tape binary over ONLY the persisted
   164	      artifacts (runtime_repo + cas_dir + bootstrap files) and emit a
   165	      verdict over 38 enumerated assertions covering chain integrity,
   166	      replay determinism, monetary invariants, predicate fidelity,
   167	      privacy contracts, Markov continuity, and tamper detection.
   168	      Verdict MUST be PROCEED with all 38 assertions GREEN before
   169	      external dual audit gate (Class 3, architect §7.7).
   170	
   171	      NO public chain. NO real-money market. NO external domain. NO
   172	      production user funds. NO AMM trading. NO DPMM / pro-rata. NO
   173	      Goodhart leak of private predicates. NO live-process state in
   174	      audit input set.
   175	
   176	Key objects (new in TB-16):
   177	  AuditInputs { runtime_repo, cas_dir, agent_pubkeys, pinned_pubkeys,
   178	                genesis, constitution, markov_pointer, alignment_dir }
   179	  AssertionResult { id, name, layer, result: PASS|FAIL|HALT, detail }
   180	  TapeAuditVerdict {
   181	    schema_version: "v1/audit_tape_verdict",
   182	    tape_root: { l4_count, l4e_count, head_state_root_hex,
   183	                 head_ledger_root_hex, cas_object_count,
   184	                 constitution_hash_hex },
   185	    tx_kind_counts: { Work, Verify, Challenge, TaskOpen, EscrowLock,
   186	                      CompleteSetMint, CompleteSetRedeem, MarketSeed,
   187	                      FinalizeReward, ChallengeResolve, TerminalSummary,
   188	                      TaskExpire, TaskBankruptcy },
   189	    assertions: { 1..38 },
   190	    passed: u32, failed: u32,
   191	    feature_coverage: { TB-1..TB-15: GREEN|YELLOW|RED },
   192	    verdict: PROCEED|BLOCK
   193	  }
   194	```
   195	
   196	## §1.1 Sandbox contract (architect §7.4 + §7.6 — binding)
   197	
   198	```text
   199	- Agent IDs MUST start with one of: "Agent_solver_", "Agent_verifier_",
   200	  "Agent_user_", or be one of the named sponsor IDs ("tb7-7-sponsor").
   201	  Production-wallet patterns are forbidden in TB-16 (CR-16.5).
   202	- Bootstrap preseed total = 30_000_000 μC (matches default_pput_preseed_pairs).
   203	- Dashboard renders SANDBOX banner on every section that surfaces market
   204	  signals; SANDBOX flag in run metadata; verdict.json `feature_coverage`
   205	  carries sandbox=true (CR-16.7, SG-16.8).
   206	- No real-money market: prediction_market.rs legacy CPMM remains
   207	  quarantined per TB-13 Atom 0.5; TB-16 imports nothing from it.
   208	- No external domain: Lean-only (mathd_algebra subset of MiniF2F, in
   209	  trust_root). No medical / legal / financial domain.
   210	```
   211	
   212	## §1.2 Halt-trigger contract (architect §7.7 + design §10)
   213	
   214	13 halt triggers from design §10 (H1..H13). Each MUST be a unit test in
   215	`tests/tb_16_halt_triggers.rs` that asserts the structural fence holds.
   216	Atom 1 stubs all 13 with `unimplemented!()`; later atoms backfill to GREEN.
   217	
   218	| ID | Trigger | Backfill atom |
   219	|---|---|---|
   220	| H1 | Pinned-pubkey verify failure on system tx | Atom 2 (audit_assertions Layer A #3) |
   221	| H2 | Agent-pubkey verify failure on agent tx | Atom 2 (Layer B #8) |
   222	| H3 | Replay state_root mismatch | Atom 2 (Layer C #12) |
   223	| H4 | L4 hash chain broken link | Atom 2 (Layer B #4) |
   224	| H5 | L4.E hash chain broken link | Atom 2 (Layer B #6) |
   225	| H6 | L4.E entry advances logical_t or state_root | Atom 2 (Layer B #6 negative) |
   226	| H7 | L4 row references unresolved CAS Cid | Atom 2 (Layer B #9) |
   227	| H8 | AgentAutopsyCapsule private_detail leak in projection | Atom 2 (Layer F #28) |
   228	| H9 | TypicalErrorSummary contains private_detail_cid | Atom 2 (Layer F #30) |
   229	| H10 | Markov capsule constitution_hash mismatch | Atom 2 (Layer G #32) |
   230	| H11 | Deep-history ingest without override | Atom 2 (Layer G #35 + Atom 6 binary smoke) |
   231	| H12 | LLM self-narrative bytes in autopsy evidence_cids | Atom 2 (Layer F new) |
   232	| H13 | total_supply_micro mutates across L4 rows | Atom 2 (Layer D #18) |
   233	
   234	---
   235	
   236	## §2 What's already shipped (substrate TB-16 builds on)
   237	
   238	| Foundation | Source | TB |
   239	|---|---|---|
   240	| Sequencer dispatch (10 typed-tx arms) | `src/state/sequencer.rs` | TB-1..TB-13 |
   241	| Git2-backed L4 ledger writer | `src/bottom_white/ledger/git2_writer.rs` | TB-6 |
   242	| L4.E rejection ledger | `src/bottom_white/ledger/rejection_evidence.rs` | TB-2 |
   243	| CAS object schema + store | `src/bottom_white/cas/{schema,store}.rs` | foundational |
   244	| Agent Ed25519 pubkey manifest | `src/runtime/agent_keypairs.rs` + `agent_pubkeys.json` | TB-7 |
   245	| Pinned system pubkeys | `pinned_pubkeys.json` | TB-5 |
   246	| `verify_chaintape` (replay verifier) | `src/runtime/verify.rs` | TB-6 |
   247	| `runtime::bootstrap::default_pput_preseed_pairs` | `src/runtime/bootstrap.rs` | TB-10 |
   248	| EconomicState 13 sub-fields incl. `agent_autopsies_t: AutopsyIndex` | `src/state/q_state.rs:170+` | TB-15 |
   249	| `AgentVisibleProjection { views, mask_set }` | `src/state/q_state.rs:131-135` | TB-14 |
   250	| `compute_price_index` pure-fn | `src/state/price_index.rs` | TB-14 |
   251	| `compute_mask_set` (Boltzmann v2) | `src/state/q_state.rs` | TB-14 |
   252	| `cluster_autopsies` + `derive_autopsies_for_bankruptcy` | `src/runtime/autopsy_capsule.rs` | TB-15 |
   253	| `MarkovEvidenceCapsule` + `generate_markov_capsule` binary | `src/runtime/markov_capsule.rs` + `src/bin/generate_markov_capsule.rs` | TB-15 |
   254	| `audit_dashboard` §1-§14 + §15 (skeleton, hard-coded empty autopsy_event_counts) | `src/bin/audit_dashboard.rs` | TB-14+15 |
   255	| `evaluator` real-LLM driver (single-task) | `experiments/minif2f_v4/src/bin/evaluator.rs` | TB-7+9 |
   256	| `lean_market` mode (multi-agent prior smoke) | `experiments/minif2f_v4/src/bin/lean_market.rs` | TB-10+13 |
   257	| First Markov capsule (`MARKOV_TB-15_2026-05-04.json`) | `handover/markov_capsules/` | TB-15 |
   258	
   259	**Open follow-ups absorbed by TB-16**:
   260	- `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` — closed by Atom 4 (replay-driven autopsy_event_counts).

exec
/bin/bash -lc "nl -ba handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-16 Ship Status — 2026-05-04
     2	
     3	**Status**: SHIPPED (pre-audit) — Atom 6 commit pending; Atom 7 dual external audit next.
     4	**Charter**: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
     5	**Architect spec**: §7 of `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
     6	**Risk class**: Class 3 integration smoke (architect §7.7 — external audit MANDATORY at ship).
     7	
     8	---
     9	
    10	## §1 Ship summary
    11	
    12	8 atoms shipped over commits `7d0d65b` (Atom 0) → `<this commit>` (Atom 6):
    13	
    14	| Atom | Commit | Subject | Class |
    15	|---|---|---|---|
    16	| 0 | `7d0d65b` | Charter ratification | 0 |
    17	| 1 | `f7e5f0a` | Halt-trigger fixture (13 H1..H13 stubs) | 2 |
    18	| 2 | `c0c890a` | `audit_assertions` module (38 assertions × 8 layers) | 2 |
    19	| 3 | `b4480d7` | `audit_tape` + `audit_tape_tamper` binaries | 3 |
    20	| 4 | `4a7863e` | Dashboard §15 live regen + §16 SANDBOX banner | 2 |
    21	| 5 | `36413c0` | `comprehensive_arena` orchestrator scaffold | 3 |
    22	| 6 | `<pending>` | Run scripts + audit pipeline smoke evidence | 3 |
    23	| 7 | TBD | Class 3 dual external audit | 3 |
    24	
    25	---
    26	
    27	## §2 Architect §7 spec coverage
    28	
    29	### FR-16.x (functional requirements)
    30	
    31	| ID | Requirement | Status |
    32	|---|---|---|
    33	| FR-16.1 | At least 3 agents participate | ✓ Sandbox preseed defines 8 sandbox-prefixed agents (4 solver + 1 verifier + 1 CompleteSet operator + 2 sponsors) |
    34	| FR-16.2 | At least one WorkTx creates FirstLongPosition | ⚠ Atom 6.1 (multi-task aggregation needed for fresh arena run; infrastructure ready) |
    35	| FR-16.3 | At least one ChallengeTx creates ShortPosition | ⚠ Atom 6.1 |
    36	| FR-16.4 | At least one CompleteSetMintTx exists | ⚠ Atom 6.1 |
    37	| FR-16.5 | At least one price update occurs | ⚠ Atom 6.1 |
    38	| FR-16.6 | At least one Boltzmann mask event occurs | ⚠ Atom 6.1 |
    39	| FR-16.7 | At least one AutopsyCapsule is generated | ⚠ Atom 6.1 |
    40	
    41	**FR-16.2 .. FR-16.7 status**: infrastructure ready (audit_assertions
    42	verifies all 13 tx kinds when present; dashboard renders price + mask;
    43	autopsy emission wired in TB-15). Fresh arena run that exercises
    44	**all** 6 task scenarios on a single chain requires evaluator
    45	multi-task aggregation extension (TB-16 Atom 6.1; not in current
    46	ship).
    47	
    48	### CR-16.x (constitutional requirements)
    49	
    50	| ID | Requirement | Status |
    51	|---|---|---|
    52	| CR-16.1 | Total Coin conserved | ✓ Layer D #18 enforces; verdict.json reports total_supply_conserved PASS |
    53	| CR-16.2 | No ghost liquidity | ✓ Inherited from TB-13 (legacy CPMM quarantined) |
    54	| CR-16.3 | No price overriding predicates | ✓ Layer E #26 (PriceIndex is view-only; not in dispatch path) |
    55	| CR-16.4 | No raw failure broadcast | ✓ Layer F #28-#31 (privacy contracts; AutopsyIndex Vec<Cid>; no private_detail bytes in projection) |
    56	| CR-16.5 | No real user funds | ✓ Layer A #3 sandbox-prefix scan; only `Agent_solver_*`/`Agent_verifier_*`/`Agent_user_*`/`tb7-7-sponsor`/`tb16-*` permitted |
    57	| CR-16.6 | All activity replayable from ChainTape + CAS | ✓ Layer C #12 + #16 (replay byte-identical; verdict_replay.json verifies determinism) |
    58	| CR-16.7 | All market activity is sandbox-labeled | ✓ Dashboard §16 SANDBOX banner renders when sandbox_run=true |
    59	
    60	### SG-16.x (ship gates)
    61	
    62	| ID | Gate | Status |
    63	|---|---|---|
    64	| SG-16.1 | Controlled market smoke produces replayable ChainTape | ✓ audit_pipeline_smoke verdict_replay byte-identical |
    65	| SG-16.2 | Dashboard shows positions, prices, masks, autopsies | ✓ §13/§14/§15 render; §15 live regen via replay |
    66	| SG-16.3 | No fake accepted nodes | ✓ Layer E #23 enforces every accepted WorkTx has all predicate_results.acceptance.* = true |
    67	| SG-16.4 | Unsolved tasks show failure evidence / bankruptcy anchors | ✓ Layer E #25 + #27; halt-trigger H7 exercised |
    68	| SG-16.5 | All market balances conserved | ✓ Layer D #17-#22 |
    69	| SG-16.6 | No unresolved evidence gaps | ✓ Layer B #9 + Layer E #24+#27; H7 fires when violated |
    70	| SG-16.7 | At least one loss → autopsy path | ⚠ Atom 6.1 (gated on fresh chain with TaskBankruptcyTx) |
    71	| SG-16.8 | Sandbox flag prevents real-money interpretation | ✓ Dashboard §16 SANDBOX banner; Layer A #3 sandbox-prefix scan |
    72	
    73	### Halt triggers (architect §7.7 + design §10 H1..H13)
    74	
    75	13/13 halt-trigger fixtures GREEN (`tests/tb_16_halt_triggers.rs`).
    76	H7 (unresolved evidence gap) **demonstrated live** via TB-13 fixture's
    77	Layer E #27 halt — confirms the halt-trigger architecture detects real
    78	evidence gaps.
    79	
    80	---
    81	
    82	## §3 Test counts
    83	
    84	```text
    85	cargo test --workspace = 905 passed / 0 failed / 150 ignored
    86	```
    87	
    88	Workspace baseline at TB-15 ship: 759. Net additions for TB-16:
    89	- 13 halt-trigger tests (Atom 1)
    90	- 5 audit_assertions module tests (Atom 2)
    91	- 3 audit_tape binary smoke tests (Atom 3)
    92	- 2 dashboard live-regen tests (Atom 4)
    93	- 2 comprehensive_arena smoke tests (Atom 5)
    94	- (Atom 6 ships scripts only — no new tests)
    95	
    96	= +25 from TB-15. (Total 905 includes accumulated additions across
    97	sub-packages; per-package counting matches `cargo test --workspace`.)
    98	
    99	---
   100	
   101	## §4 Open follow-ups
   102	
   103	### Atom 6.1 — multi-task chain continuation (HIGH; gates fresh arena run)
   104	
   105	The current `lean_market run-task` semantics produce ONE chain per
   106	task. To produce a single chain with all 13 tx kinds, evaluator needs
   107	to support continuing an existing `runtime_repo` across multiple
   108	task invocations. This is a moderate refactor (sequencer's
   109	`NonEmptyRuntimeRepo` fail-closed gate per
   110	`src/runtime/mod.rs:216-220` would need a guarded resume path with
   111	explicit user opt-in via env var, e.g. `TURINGOS_CHAINTAPE_RESUME=1`).
   112	
   113	Until 6.1 ships:
   114	- audit_pipeline can validate any existing chain-backed tape
   115	- comprehensive arena evidence is per-task sub-tapes (not aggregated)
   116	- 13-tx-kind coverage must be assessed across the union of sub-tapes,
   117	  not within a single tape
   118	
   119	### Mathlib build (precondition for fresh real-LLM run)
   120	
   121	`experiments/minif2f_v4/.lake/packages/` is missing — required for
   122	Lean oracle evaluation. Run:
   123	```bash
   124	cd experiments/minif2f_v4 && lake exe cache get   # ~2 min
   125	```
   126	per `feedback_lake_packages_vendored`. This is a **user-side action**
   127	because the cache fetch is network-bound and not deterministic across
   128	sessions.
   129	
   130	### TB-15 carry-forward (deferred from TB-15 charter §1.2)
   131	
   132	- `OBS_TB_13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md` (carry-forward)
   133	- `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` (carry-forward)
   134	- `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md` (carry-forward)
   135	- `OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md` (carry-forward; not in TB-16 scope)
   136	
   137	### Closed by TB-16
   138	
   139	- `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` — closed by
   140	  Atom 4 (`build_report` now reconstructs EconomicState via
   141	  `replay_full_transition`; verified by
   142	  `tests/tb_16_dashboard_live_regen.rs` 2/2 PASS).
   143	
   144	---
   145	
   146	## §5 Cross-references
   147	
   148	- TB-15 ship: commit `2337381` + R3 `eddab36`; SHIP_STATUS at
   149	  `handover/ai-direct/TB-15_SHIP_STATUS_2026-05-03.md`
   150	- TB-14 ship: commit `8b93fd9`
   151	- TB-13 ship: charter `handover/tracer_bullets/TB-13_charter_2026-05-03.md`
   152	- Audit pipeline evidence: `handover/evidence/tb_16_real_llm_arena_2026-05-04/`
   153	- TB-16 evidence README: `handover/evidence/tb_16_real_llm_arena_2026-05-04/README.md`
   154	- Architect §7 spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`
   155	
   156	---
   157	
   158	## §6 Atom 7 dual external audit gate
   159	
   160	Per `feedback_dual_audit` + `feedback_risk_class_audit`: Class 3
   161	integration smoke = full Codex + Gemini hybrid dual external audit at
   162	ship. Atom 7 will:
   163	
   164	1. Codex audit (via `codex:rescue` agent or `run_codex_*.sh`).
   165	2. Gemini audit (via `run_gemini_*.py`).
   166	3. Conservative resolution: VETO > CHALLENGE > PASS per
   167	   `feedback_dual_audit_conflict`.
   168	4. Round-cap=2 per `feedback_elon_mode_policy`.
   169	5. Final commit on PASS/PASS or degraded-PASS.

exec
/bin/bash -lc "nl -ba handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# Real-LLM Comprehensive ChainTape — Audit-From-Tape Test Design — 2026-05-04
     2	
     3	**Status**: DESIGN (not yet implemented).
     4	**Authority**: user request 2026-05-04 ("design comprehensive real LLM chaintape test on all TB features so far. the audit need to be done from the tape it produced").
     5	**Naming**: this design IS the proposed implementation of **TB-16 Controlled Market Smoke Arena** per architect §7 (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md`). Architect spec specifies "compute + position + complete set + price + mask + autopsy" minimum scope; this design extends to **all 13 TB-shipped tx types** and adds the **audit-from-tape contract** as the load-bearing acceptance gate.
     6	**Phase**: P4 + P5 v0 prep (anchors all phases since P1 chain integrity is the audit-from-tape contract).
     7	**Risk class**: **Class 3 integration smoke** (architect §7.7 verbatim). External audit required at ship.
     8	
     9	---
    10	
    11	## §1 One-line goal
    12	
    13	```text
    14	Drive a single real-LLM evaluator run that exercises EVERY shipped tx
    15	type (TB-1..TB-15) on a live multi-agent Lean-proof market. Persist
    16	the run as a chain-backed ChainTape (Sequencer::apply_one + on-disk
    17	LedgerEntry chain + L4.E rejection ledger + CAS objects). Then run a
    18	SEPARATE audit binary (`audit_tape`) that reads ONLY the persisted
    19	artifacts (runtime_repo + cas_dir + bootstrap files) and emits a
    20	verdict over 38 enumerated assertions covering chain integrity,
    21	replay determinism, monetary invariants, predicate fidelity, privacy
    22	contracts, Markov continuity, tamper detection, and dashboard
    23	regenerability — proving the system's own evidence is sufficient to
    24	re-derive every shipped invariant without consulting the running
    25	process.
    26	```
    27	
    28	---
    29	
    30	## §2 Why "audit from the tape"
    31	
    32	Per `handover/architect-insights/ROADMAP_9_PHASE_2026-04-29.md` P1 Exit-8: *"state.db 删除后，可以从 L4 (accepted only) 重建 — L4.E 不参与 state_root 重建"*. This test elevates that exit criterion from a kernel unit test to a system-level acceptance gate: the auditor must reach the same conclusions as the live sequencer using only the on-disk evidence.
    33	
    34	Per `feedback_smoke_evidence_naming` (binding 2026-05-01 D5): only chain-backed production runs may be called "ChainTape smoke / smoke tape / tape". Pre-TB-6 stdout-only paper trail is "smoke evidence" — not eligible for this gate.
    35	
    36	Per `feedback_o1_chain_on_auditability`: state facts → L4; rejected tx → L4.E; high-dim evidence → CAS; failure anchored via system-emitted RunExhausted/Bankruptcy/Expire. This test exercises ALL THREE substrates simultaneously and demands the auditor reconstruct every invariant from them.
    37	
    38	---
    39	
    40	## §3 Coverage matrix — every shipped TB feature gets a tx in the tape
    41	
    42	| TB | Feature | Tx / Object | Driver | Gate it discharges |
    43	|---|---|---|---|---|
    44	| TB-1 | Monetary invariant | (genesis on_init) | bootstrap | total_supply_micro frozen at on_init |
    45	| TB-1 | L4 hash chain | every accepted tx | sequencer | hash chain replay-deterministic |
    46	| TB-1 | L4.E rejection chain | every rejected tx | sequencer | L4.E hash chain valid; not in state_root |
    47	| TB-2 | WorkTx admission | `WorkTx` | solver agents (LLM) | Sequencer::submit + apply_one accept |
    48	| TB-3 | Task escrow / RSP-1 | `TaskOpenTx` + `EscrowLockTx` | sponsor agents | task_markets_t.total_escrow funded; cache=truth |
    49	| TB-3 | Stake lock-on-accept | `WorkTx.stake` field | solver | balances_t debit + stakes_t credit |
    50	| TB-4 | Verifier bond | `VerifyTx.bond` field | verifier agent | balances_t debit + stakes_t[verify_tx_id] credit |
    51	| TB-4 | Challenger NO stake | `ChallengeTx.stake` field | adversarial agent | balances_t debit + challenge_cases_t entry |
    52	| TB-5 | System-emitted gate | `ChallengeResolveTx` | sequencer (system) | Released path refunds; UpheldDeferred is marker |
    53	| TB-6 | Production wire-up | `Git2LedgerWriter` on disk | evaluator binary | LedgerEntry chain on disk + replay-verifiable |
    54	| TB-7 | Per-agent Ed25519 | `agent_pubkeys.json` | evaluator bootstrap | every agent tx signature verifies live + replay |
    55	| TB-7 | Agent audit trail | `ProposalTelemetry` CAS object | sequencer post-accept | every accepted WorkTx → telemetry CID linked |
    56	| TB-7.7 | Lean oracle attestation | `VerificationResult` CAS object | verifier | every accepted WorkTx → verified=true |
    57	| TB-7R | Genesis report | `genesis_report.json` | bootstrap | constitution_hash + initial balances on disk |
    58	| TB-8 | Minimal payout | `FinalizeRewardTx` | sequencer (system) | claim → finalize → balances_t credit |
    59	| TB-10 | Preseed factory | `runtime::bootstrap` | bootstrap | 12-entry preseed sums to 30M micro |
    60	| TB-11 | RunExhausted anchor | `TerminalSummaryTx` | sequencer (system) | failure anchored on L4 with EvidenceCapsule CID |
    61	| TB-11 | Capital release | `TaskExpireTx` | sequencer (system) | sponsor escrow refunded post-deadline |
    62	| TB-11 | Death certificate | `TaskBankruptcyTx` | sequencer (system) | task state → Bankrupt; >= N exhausted runs |
    63	| TB-11 | EvidenceCapsule | CAS `EvidenceCapsule` + `EvidenceManifest` + `CompressedRunLog` | sequencer | O(1) chain / O(N) audit |
    64	| TB-12 | NodePosition (Long) | side-effect on accepted Work | sequencer | NOT counted in total_supply (CR-12.1/2) |
    65	| TB-12 | NodePosition (Short) | side-effect on accepted Challenge | sequencer | exposure index, not balance |
    66	| TB-13 | CompleteSet mint | `CompleteSetMintTx` | special agent | 1 Coin → 1 YES + 1 NO; conditional_collateral_t |
    67	| TB-13 | Market seed | `MarketSeedTx` | sponsor | provider funds, no ghost liquidity |
    68	| TB-13 | CompleteSet redeem | `CompleteSetRedeemTx` | special agent (post-resolution) | winning side paid; min-balanced invariant |
    69	| TB-14 | PriceIndex | derived view (`compute_price_index`) | dashboard / scheduler | "price is signal, not truth"; integer-rational |
    70	| TB-14 | Boltzmann mask | `mask_set` on `AgentVisibleProjection` | scheduler | parent not deleted from chaintape |
    71	| TB-14 | CanonicalNodeGraph | derived from L4 + ProposalTelemetry | sequencer | replay-deterministic edge map |
    72	| TB-15 | Autopsy emission | side-effect on TaskBankruptcyTx | sequencer | per-staker `AgentAutopsyCapsule` Cid in `agent_autopsies_t` |
    73	| TB-15 | TypicalErrorBroadcast | `cluster_autopsies` | end-of-run / dashboard | N≥3 cluster → public_summary surface |
    74	| TB-15 | Markov capsule | `MarkovEvidenceCapsule` | end-of-run binary | constitution_hash + L4 + L4.E + CAS roots + previous capsule |
    75	| TB-15 | Default-deny gate | `TURINGOS_MARKOV_OVERRIDE` | binary | deeper history denied without override |
    76	
    77	**Coverage = 100%** of agent-signed tx types (Work / Verify / Challenge / TaskOpen / EscrowLock / CompleteSetMint / CompleteSetRedeem / MarketSeed) + 5 system-emitted tx types (FinalizeReward / ChallengeResolve / TerminalSummary / TaskExpire / TaskBankruptcy) + 6 CAS object types (ProposalPayload / ProposalTelemetry / VerificationResult / EvidenceCapsule / AgentAutopsyCapsule / MarkovEvidenceCapsule).
    78	
    79	---
    80	
    81	## §4 Scenario design — six tasks engineered for full coverage
    82	
    83	```text
    84	Bootstrap (runtime::bootstrap::default_pput_preseed_pairs):
    85	  tb7-7-sponsor   : 24_000_000 μC  (sponsor of all Lean tasks)
    86	  Agent_user_0    :  6_000_000 μC  (user-task sponsor for one task)
    87	  Agent_solver_0  :    100_000 μC  (Lean solver, baseline)
    88	  Agent_solver_1  :    100_000 μC  (Lean solver, challenger-bait)
    89	  Agent_solver_2  :    100_000 μC  (CompleteSet operator)
    90	  Agent_solver_3  :    100_000 μC  (adversarial: posts ChallengeTx)
    91	  Agent_verifier_0:    100_000 μC  (independent verifier, posts VerifyTx)
    92	  Total: 30_000_000 μC = on_init mint; assert_no_post_init_mint enforces
    93	
    94	Tasks (sponsored by tb7-7-sponsor unless noted):
    95	  Task A "happy_path"      : trivial Lean theorem; solver_0 finds proof;
    96	                             verifier_0 confirms; no challenge;
    97	                             Sequencer emits FinalizeRewardTx.
    98	                             EXERCISES: TaskOpen + EscrowLock + Work +
    99	                             Verify + FinalizeReward + ProposalTelemetry +
   100	                             VerificationResult + NodePosition(Long)
   101	
   102	  Task B "challenge_dismissed": correct proof; Agent_solver_3 challenges
   103	                             (incorrectly); verifier_0 re-confirms;
   104	                             Sequencer emits ChallengeResolveTx{Released};
   105	                             challenger bond refunded.
   106	                             EXERCISES: Work + Verify + Challenge +
   107	                             ChallengeResolve(Released) +
   108	                             NodePosition(ChallengeShort) +
   109	                             cache=truth across challenge bond movement
   110	
   111	  Task C "challenge_upheld": invalid proof (wrong Lean form);
   112	                             Agent_solver_3 challenges (correctly);
   113	                             verifier_0 confirms challenge; Sequencer
   114	                             emits ChallengeResolveTx{UpheldDeferred};
   115	                             bond preserved (slash deferred to RSP-3.2).
   116	                             EXERCISES: ChallengeResolve(UpheldDeferred)
   117	                             marker path; bond accumulation in
   118	                             challenge_cases_t
   119	
   120	  Task D "exhaustion"      : hard Lean theorem; solver_1 runs out of
   121	                             MAX_TX without finding proof; Sequencer
   122	                             emits TerminalSummaryTx with
   123	                             ExhaustionReason::MaxTxExhausted +
   124	                             EvidenceCapsule CID;
   125	                             after N (=2) such RunExhausted, Sequencer
   126	                             emits TaskBankruptcyTx → triggers TB-15
   127	                             autopsy emission for solver_1's stake.
   128	                             EXERCISES: TerminalSummary + EvidenceCapsule
   129	                             + TaskBankruptcy + AgentAutopsyCapsule +
   130	                             agent_autopsies_t insertion
   131	
   132	  Task E "expiry"          : sponsor opens task; no solver picks it up
   133	                             before deadline elapses; Sequencer emits
   134	                             TaskExpireTx with sponsor refund.
   135	                             EXERCISES: TaskExpire + capital release
   136	
   137	  Task F "complete_set_market" (Agent_user_0 sponsor):
   138	                             Agent_user_0 posts MarketSeedTx (provider
   139	                             funds 1_000_000 μC into conditional inventory);
   140	                             Agent_solver_2 posts CompleteSetMintTx
   141	                             (1 Coin → 1 YES + 1 NO);
   142	                             solver_0 finds proof for the gating Lean task;
   143	                             FinalizeReward resolves event_id YES;
   144	                             Agent_solver_2 posts CompleteSetRedeemTx for
   145	                             YES side; winning side paid 1:1 against
   146	                             collateral.
   147	                             EXERCISES: MarketSeed + CompleteSetMint +
   148	                             CompleteSetRedeem + ConditionalCollateral +
   149	                             ConditionalShareBalances + MIN-balanced
   150	                             invariant + ResolutionRef path
   151	
   152	End-of-run (post-evaluator-exit):
   153	  generate_markov_capsule binary fires.
   154	  EXERCISES: Markov capsule generation + LATEST_MARKOV_CAPSULE.txt +
   155	             handover/markov_capsules/MARKOV_TB-16_2026-05-04.json
   156	```
   157	
   158	**Why these six**: Tasks A/D give 100% solver-side coverage (success + exhaustion → bankruptcy → autopsy). Tasks B/C cover both ChallengeResolve paths (Released + UpheldDeferred). Task E covers TaskExpire. Task F covers the entire CompleteSet (TB-13) substrate. Solver_3's adversarial role plus verifier_0's independent verification cover the verifier/challenger market.
   159	
   160	---
   161	
   162	## §5 Real-LLM provider configuration
   163	
   164	```text
   165	Solver agents (Agent_solver_0..3):
   166	  Provider:    deepseek-v4-flash thinking-off (per project_chat_over_reasoner)
   167	  Endpoint:    src/drivers/llm_proxy.py multi-key round-robin
   168	  Reason:      30-day arc backbone; deterministic thinking-off mode
   169	  Concurrency: 4 (per `feedback_routines_entropy` — earn the cost)
   170	  Token cap:   per-attempt 1024; per-task MAX_TRANSACTIONS=20
   171	
   172	Verifier agent (Agent_verifier_0):
   173	  Provider:    deepseek-v4-flash thinking-off (separate routing pool)
   174	  Reason:      Independent from solver pool to avoid same-LLM correlation
   175	
   176	Reproducibility:
   177	  - Set TURINGOS_RUN_SEED=2026-05-04 (passes through to RunId mint)
   178	  - Lock evaluator --schedule-seed 0
   179	  - LLM providers seed via X-Provider-Seed header where supported
   180	  - Pin DeepSeek model snapshot via X-DS-Model-Hash header
   181	  - Document in handover/evidence/tb_16_*/README.md any drift observed
   182	    per project_deepseek_drift_2026-04-24
   183	
   184	Caps:
   185	  Wall clock:  TURINGOS_WALL_CLOCK_CAP_MS=1800000 (30 min)
   186	  Compute:     TURINGOS_COMPUTE_CAP_TOKENS=120000
   187	  Cost:        TURINGOS_COST_CEILING_USD=15
   188	```
   189	
   190	**Lean oracle**: `experiments/minif2f_v4/src/lean4_oracle.rs` (frozen per Phase A; in trust_root). Mathlib via `lake exe cache get` (~2 min) per `feedback_lake_packages_vendored`.
   191	
   192	---
   193	
   194	## §6 Audit-from-tape contract — `audit_tape` binary specification
   195	
   196	### §6.1 Inputs (the only inputs)
   197	
   198	```text
   199	audit_tape \
   200	  --runtime-repo  <path/to/runtime_repo>                  (TB-6 Git2 chain)
   201	  --cas-dir       <path/to/cas>                           (TB-6 CAS store)
   202	  --agent-pubkeys <path/to/agent_pubkeys.json>            (TB-7)
   203	  --pinned-pubkeys <path/to/pinned_pubkeys.json>          (TB-5)
   204	  --genesis       <path/to/genesis_payload.toml>          (P0)
   205	  --constitution  <path/to/constitution.md>               (P0)
   206	  --markov-pointer <path/to/LATEST_MARKOV_CAPSULE.txt>    (TB-15)
   207	  [--alignment-dir <path/to/handover/alignment>]          (TB-15 OBS scan)
   208	  --out <verdict.json>
   209	```
   210	
   211	The auditor MUST NOT read:
   212	- live `Sequencer` state (no shared memory)
   213	- `state.db` (whitebox cache; auditor rebuilds from L4)
   214	- evaluator process logs (transient; not in tape)
   215	- `handover/ai-direct/` (live working notes; not part of tape contract)
   216	
   217	Anything reachable from these inputs alone is in-scope; anything not reachable is OUT.
   218	
   219	### §6.2 38 audit assertions (pass = all green)
   220	
   221	**Layer A — bootstrap integrity** (3 assertions)
   222	1. `constitution_hash` computed via sha256(constitution.md) matches `[constitution_root]` in genesis_payload.toml.
   223	2. `verify_trust_root` passes — every entry in `[trust_root]` matches its file's current sha256.
   224	3. `pinned_pubkeys.json` contains the same Ed25519 pubkey that `system_signature_of` would verify against on every system-emitted tx in the tape.
   225	
   226	**Layer B — chain integrity** (8 assertions)
   227	4. L4 hash chain valid: for each row r at logical_t=t, `r.parent_ledger_root == prior.resulting_ledger_root` and `append(parent, signing_digest) == r.resulting_ledger_root`.
   228	5. L4 parent_state_root continuity: `r.parent_state_root == prior.resulting_state_root`.
   229	6. L4.E hash chain valid: same recurrence over the rejection_evidence ledger; never advances logical_t; never advances state_root.
   230	7. Every system-emitted tx (FinalizeReward / ChallengeResolve / TerminalSummary / TaskExpire / TaskBankruptcy) verifies against `pinned_pubkeys.json`.
   231	8. Every agent-signed tx (Work / Verify / Challenge / TaskOpen / EscrowLock / CompleteSetMint / CompleteSetRedeem / MarketSeed) verifies against `agent_pubkeys.json`.
   232	9. Every `tx_payload_cid` resolves to a CAS object whose canonical_decode produces a TypedTx whose `tx_kind()` matches the L4 row's `tx_kind`.
   233	10. No agent-signed tx has tx_kind ∈ system-only set (negative — admission-control structural).
   234	11. Genesis row (logical_t=1) has `parent_ledger_root == Hash::ZERO` and `parent_state_root == Hash::ZERO`.
   235	
   236	**Layer C — replay determinism** (5 assertions)
   237	12. `replay_full_transition` over L4 alone reaches the same final `state_root_t` recorded in the chain head's `resulting_state_root`.
   238	13. `replay_full_transition` produces the same `EconomicState` object byte-for-byte (canonical encode == canonical encode).
   239	14. For each `TaskBankruptcyTx` row, `derive_autopsies_for_bankruptcy` re-run with the row's pre-snapshot returns Cids identical to those stored in `agent_autopsies_t[event_id]`.
   240	15. `compute_canonical_edges_at_head` re-derived from L4 + CAS-resident ProposalTelemetry produces the same map as the one bus.snapshot() would publish.
   241	16. Replay is deterministic across runs: invoke the auditor twice, assert the two `verdict.json` outputs are byte-identical.
   242	
   243	**Layer D — economic invariants** (6 assertions)
   244	17. `assert_no_post_init_mint` passes for every accepted tx (no Coin minted post on_init).
   245	18. `assert_total_ctf_conserved` holds at every L4 row (6-holding `total_supply_micro` constant equal to genesis on_init total = 30_000_000 μC).
   246	19. `assert_complete_set_balanced` (MIN-form: `min(Σ_yes, Σ_no) == collateral`) holds at every L4 row that touched conditional_collateral_t / conditional_share_balances_t.
   247	20. `assert_task_market_total_escrow_matches_locks` (cache=truth) holds at every L4 row.
   248	21. `node_positions_t` total amount NOT counted in `total_supply_micro` (CR-12.1 + CR-12.2 structural — assertion is "would inclusion break #18; assert it does").
   249	22. `conditional_share_balances_t` total NOT counted in `total_supply_micro` (CR-13.3 + SG-13.2 same shape).
   250	
   251	**Layer E — predicate / evidence integrity** (5 assertions)
   252	23. Every accepted WorkTx: every entry in `work.predicate_results.acceptance.*.value == true`.
   253	24. Every accepted WorkTx: ProposalTelemetry CAS object reachable via `work.proposal_cid`; ProposalTelemetry's `verification_result_cid` resolves to a `VerificationResult{verified: true}`.
   254	25. Every L4.E row: `rejection_class` matches the `TransitionError` that the dispatch arm would have produced from the tx (re-dispatch the rejected tx; assert error variant matches recorded class).
   255	26. PriceIndex computed via `compute_price_index(replayed_econ)` matches what dashboard `render_section_14` would render at HEAD.
   256	27. Every `TerminalSummaryTx.evidence_capsule_cid` resolves to a CAS-resident `EvidenceCapsule` whose `attempt_count` + `terminal_reason` match the run.
   257	
   258	**Layer F — privacy contracts** (4 assertions; TB-15 specific)
   259	28. AgentVisibleProjection serialization (rebuilt from `tape_view_t` in replayed QState) contains no agent_autopsies_t / AutopsyIndex / private_detail_cid / AgentAutopsyCapsule byte runs.
   260	29. Every `AgentAutopsyCapsule.private_detail_cid` resolves to a CAS object whose creator is `sequencer-epoch-{epoch}` (system-emitted; never an agent string).

exec
/bin/bash -lc "rg -n \"Atom 6|6\\.1|multi-task|single chain|all 13|default_pput_preseed_pairs|previous_capsule|Markov\" handover/tracer_bullets/TB-16_charter_2026-05-04.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
6:**Authority**: architect §7 (operative spec; FR-16.1..7 + CR-16.1..7 + SG-16.1..8 + forbidden list + halt triggers + Class 3 envelope).
18:- **Flowchart 3 (meta)**: `FC3-N44` = real-LLM-driven Markov capsule (first MARKOV_TB-16_2026-05-04.json with non-empty `typical_errors` because TaskBankruptcyTx fires in Task D → autopsies clustered).
24:                               emits all 13 shipped tx kinds + 6 CAS object
25:                               types in a single chain-backed multi-agent
46:            TypicalErrorSummary embedded in the Markov capsule. SG-16.7
59:  P5-prep   First Markov capsule with non-empty typical_errors (TB-15
62:            Markov head. Layer G #33 verifies recomputation determinism.
113:                previous_capsule_cid chains to TB-15 capsule).
167:      privacy contracts, Markov continuity, and tamper detection.
202:- Bootstrap preseed total = 30_000_000 μC (matches default_pput_preseed_pairs).
216:Atom 1 stubs all 13 with `unimplemented!()`; later atoms backfill to GREEN.
229:| H10 | Markov capsule constitution_hash mismatch | Atom 2 (Layer G #32) |
230:| H11 | Deep-history ingest without override | Atom 2 (Layer G #35 + Atom 6 binary smoke) |
247:| `runtime::bootstrap::default_pput_preseed_pairs` | `src/runtime/bootstrap.rs` | TB-10 |
253:| `MarkovEvidenceCapsule` + `generate_markov_capsule` binary | `src/runtime/markov_capsule.rs` + `src/bin/generate_markov_capsule.rs` | TB-15 |
257:| First Markov capsule (`MARKOV_TB-15_2026-05-04.json`) | `handover/markov_capsules/` | TB-15 |
261:- TB-15 Markov capsule's `typical_errors=[]` empty (no ship-time bankruptcy) — closed by TB-16 capsule (Task D forces TaskBankruptcyTx → ≥3 autopsies cluster).
281:**Halt triggers green**: H1, H2, H3, H4, H5, H6, H7, H8, H9, H10, H12, H13 (12 of 13; H11 is a binary-level fence verified in Atom 6 smoke).
302:- Bootstrap: read `default_pput_preseed_pairs` (30M μC total); 4 solver + 1 verifier + 1 special CompleteSet operator + 2 sponsors = 8 sandbox-labeled agents.
308:- mock-LLM smoke (no network): 6-task scenario completes; all 13 tx_kinds appear.
311:### Atom 6 — Run scripts + first real-LLM ship (Class 3, 72h-to-feedback-loop exception)
321:**NEW**: `handover/tests/scripts/audit_tape_smoke_test.sh` — wraps run_real_llm_arena.sh + asserts all 13 expected tx_kinds appear + verdict.json `verdict == "PROCEED"`.
333:**Final gate**: All ship gates SG-16.1..8 GREEN; `cargo test --workspace` PASS; verdict.json verdict=PROCEED.
336:**Final commit (pre-audit)**: `TB-16 SHIPPED (pre-audit) — Controlled Market Smoke Arena (Class 3 integration smoke; 8/8 SG; 13/13 halt-triggers; all 13 tx kinds + 6 CAS object types exercised on chain-backed real-LLM run; verdict.json PROCEED; closes architect §7 spec FR-16.1..7 + CR-16.1..7 + SG-16.1..8 + closes OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16)`. FC-trace: `FC1-N34 + FC1-N35 + FC1-N36 + FC2-N31 + FC2-N32 + FC2-N33 + FC3-N44`.
394:| SG-16.1 | Controlled market smoke produces replayable ChainTape | `tb_16_audit_tape_binary` Layer C #12+#13+#16 |
395:| SG-16.2 | Dashboard shows positions, prices, masks, autopsies | `tb_16_dashboard_live_regen.rs` + Atom 6 dashboard.txt §13+§14+§15 |
400:| SG-16.7 | At least one loss → autopsy path | Atom 6 Task D forces TaskBankruptcyTx → autopsy emission; verdict.json tx_kind_counts.TaskBankruptcy ≥ 1 + agent_autopsies_t non-empty |
402:| G-16.9 | `cargo test --workspace` ≥ TB-15 baseline / 0 fail / ≤150 ignored | Atom 6 ship report |
403:| G-16.10 | All 13 tx_kinds present in tape | verdict.json tx_kind_counts each ≥ 1 |
404:| G-16.11 | All 6 CAS object types reachable | verdict.json layer counts |
405:| G-16.12 | Replay determinism: byte-identical verdict.json across runs | Layer C #16 + Atom 6 step 8 |
406:| G-16.13 | First Markov capsule with non-empty typical_errors persisted | `handover/evidence/tb_16_*/MARKOV_TB-16_2026-05-04.json` typical_errors.length ≥ 1 |
407:| G-16.14 | Tamper detection: 3/3 corruptions detected | Atom 6 tamper_report.json |
408:| G-16.15 | FC1-N34 + FC1-N35 + FC1-N36 + FC2-N31..N33 + FC3-N44 each have ≥1 witness | `tests/fc_alignment_conformance.rs` |
417:| B | Audit binary input set | runtime_repo + cas + agent_pubkeys + pinned_pubkeys + genesis + constitution + markov_pointer + alignment_dir | per design §6.1 verbatim; live-process state excluded by construction (CR-16.6 replayability + Art.0.2 Tape Canonical). |
423:| H | Sandbox prefix list | ["Agent_solver_", "Agent_verifier_", "Agent_user_", "tb7-7-sponsor"] | matches default_pput_preseed_pairs identifiers; Layer A assertion scans this prefix list. |
426:| K | Markov override semantics | TURINGOS_MARKOV_OVERRIDE=1 enables deeper history; default-deny | TB-15 decision; TB-16 inherits; verified at Layer G #35 + Atom 6 binary smoke. |

exec
/bin/bash -lc "rg -n \"7\\.1|7\\.2|7\\.3|7\\.4|7\\.5|7\\.6|7\\.7|TB-16|Controlled Market Smoke Arena|Halt if\" handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
157:TB-16  Controlled Market Smoke Arena
583:Halt if:
751:Halt if:
762:# 7. TB-16 — Controlled Market Smoke Arena
764:## 7.1 目标
776:## 7.2 Scenario
792:## 7.3 Functional requirements
819:## 7.4 Constitutional requirements
846:## 7.5 Ship gates
876:## 7.6 Forbidden
891:## 7.7 Loop-mode instruction
901:Halt if:
939:FR-17.1
942:FR-17.2
945:FR-17.3
948:FR-17.4
951:FR-17.5
954:FR-17.6
957:FR-17.7
966:CR-17.1
969:CR-17.2
972:CR-17.3
975:CR-17.4
978:CR-17.5
981:CR-17.6
984:CR-17.7
993:SG-17.1
996:SG-17.2
999:SG-17.3
1002:SG-17.4
1005:SG-17.5
1008:SG-17.6
1011:SG-17.7
1106:| TB-16 | Controlled Market Smoke Arena       | 受控市场演习                           | 不接真实世界       |
1146:TB-16:
1147:Controlled Market Smoke Arena.
1212:TB-16 必须是 sandbox controlled market，不接真实资金/公链/真实世界。
1245:parallel families for TB-14 / TB-15 / TB-16 / TB-17). Part A is the canonical
1312:| AMM/CPMM router | Forbidden in TB-13 | Forbidden through TB-15; first allowed scope is TB-16 sandbox arena (and only "AMM trading unless explicitly scoped") |
1348:4. Drafting TB-14 / TB-15 / TB-16 / TB-17 stub charters.

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/README.md | sed -n '1,240p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# TB-16 Comprehensive Arena — Evidence (2026-05-04)
     2	
     3	**Ship**: TB-16 Atom 6 (pre-audit; full Class 3 dual external audit at Atom 7).
     4	**Charter**: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
     5	**Architect spec**: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7
     6	**Implementation contract**: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
     7	
     8	---
     9	
    10	## What was shipped
    11	
    12	**Atom 6 v0** ships the full audit-from-tape infrastructure + verifies it
    13	end-to-end on a real chain-backed tape:
    14	
    15	### Infrastructure (commit `36413c0` — Atom 5; commit `<this commit>` — Atom 6):
    16	
    17	- `src/runtime/audit_assertions.rs` — 38-assertion pure-fn battery
    18	- `src/bin/audit_tape.rs` — CLI wrapper emitting `verdict.json`
    19	- `src/bin/audit_tape_tamper.rs` — 3-corruption tamper-detection harness
    20	- `experiments/minif2f_v4/src/bin/comprehensive_arena.rs` — 6-task orchestrator scaffold
    21	- `handover/tests/scripts/run_real_llm_arena.sh` — end-to-end runner
    22	- `handover/tests/scripts/audit_tape_smoke_test.sh` — ship-gate wrapper
    23	- Dashboard §15 live regen + §16 SANDBOX banner (closes
    24	  `OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md`)
    25	
    26	### Audit pipeline smoke evidence (`audit_pipeline_smoke/`):
    27	
    28	End-to-end validation that the full pipeline (audit_tape +
    29	audit_tape_tamper + generate_markov_capsule + audit_dashboard +
    30	replay-determinism) works on a chain-backed real-LLM tape. Tape source:
    31	`handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171/`
    32	(TB-13 chain with 3 L4 rows + 2 L4.E rows + 11 CAS objects).
    33	
    34	**Pipeline output**:
    35	
    36	| Artifact | Status |
    37	|---|---|
    38	| `verdict.json` | `verdict=BLOCK passed=31 failed=0 halted=1 skipped=7` |
    39	| `verdict_replay.json` | byte-identical to `verdict.json` (replay determinism ✓) |
    40	| `tamper_report.json` | `detected_count=3/3` (all 3 corruptions detected ✓) |
    41	| `MARKOV_TB-16_2026-05-03.json` | first TB-16 Markov capsule; `capsule_id=5da53602...`; constitution_hash + 4 flowchart hashes + 23 unresolved OBS |
    42	| `LATEST_MARKOV_CAPSULE.txt` | local pointer (capsule_id hex) |
    43	| `dashboard.txt` | 15-section render (incl. live-regen §15 + SANDBOX §16 banner) |
    44	
    45	**Why verdict=BLOCK**: the TB-13 fixture chain has 1 Halt at Layer E #27
    46	(`evidence_capsule_cid not in CAS at L4 index 2`) — the TB-13 smoke
    47	emitted a `TerminalSummaryTx` whose `evidence_capsule_cid` was not
    48	written to CAS. This is **correct detection** by audit_tape — the
    49	fixture has a real evidence gap. A fresh TB-16 arena run on a chain
    50	that emits a complete TerminalSummary + EvidenceCapsule pair will
    51	satisfy assertion #27 and emit verdict=PROCEED.
    52	
    53	**Halt-trigger H7 (architect §7.7 unresolved_evidence_gap)**: this
    54	audit run **demonstrates the halt-trigger fires correctly** when an
    55	evidence gap exists. ✓
    56	
    57	---
    58	
    59	## What's deferred
    60	
    61	**Fresh real-LLM arena execution** (Task A..F end-to-end on a fresh
    62	multi-task chain producing all 13 architect-required tx kinds) is
    63	gated on user-side preconditions:
    64	
    65	1. ✓ DeepSeek API keys in `.env` (5 keys present)
    66	2. ✓ LLM proxy running at `http://localhost:18080` (verified live)
    67	3. ✗ **Mathlib NOT cached**: `experiments/minif2f_v4/.lake/packages/`
    68	   missing. Required: `cd experiments/minif2f_v4 && lake exe cache get`
    69	   (~2 min download + decompression per `feedback_lake_packages_vendored`).
    70	4. ⚠ Multi-task aggregation: each task currently maps to its own
    71	   sub-tape via the `lean_market run-task` pattern. Aggregating Tasks
    72	   A..F onto a single shared chain (so all 13 tx kinds appear in ONE
    73	   tape) requires evaluator extensions tagged TB-16 Atom 6.1 — namely
    74	   chain-continuation semantics across multiple `lean_market run-task`
    75	   invocations against the same `runtime_repo`.
    76	
    77	**Unblocking Atom 6.1** = ship a fresh tape with all 13 tx kinds + ≥1
    78	TaskBankruptcy → autopsy emission. Until then, audit_pipeline_smoke
    79	in this dir is the integration witness that the audit-from-tape
    80	contract holds end-to-end.
    81	
    82	---
    83	
    84	## Acceptance gate (design §7.1; assessed against this evidence)
    85	
    86	| Gate | Status | Note |
    87	|---|---|---|
    88	| 1. Evaluator within 30 min + cost ceiling | N/A | no fresh evaluator run |
    89	| 2. All 13 tx_kinds present | ⚠ partial | TB-13 fixture has 5 of 13 (TaskOpen + EscrowLock + TerminalSummary + 2 others); fresh arena run gated on Atom 6.1 |
    90	| 3. All 6 CAS object types reachable | ⚠ partial | TB-13 fixture lacks AgentAutopsyCapsule + AutopsyPrivateDetail + MarkovEvidenceCapsule (now generated locally) |
    91	| 4. verdict.json verdict=PROCEED | ✗ BLOCK | 1 halt at Layer E #27 (correct detection — TB-13 fixture has evidence gap) |
    92	| 5. Dashboard renders all 16 sections | ✓ | dashboard.txt incl. §15 + §16 |
    93	| 6. First TB-16 Markov capsule emitted; constitution_hash matches | ✓ | capsule_id=5da53602...; SG-15.7 PASS |
    94	| 7. Replay byte-identical | ✓ | `cmp -s verdict.json verdict_replay.json` PASS |
    95	| 8. Tamper detection 3/3 | ✓ | tamper_report.json detected_count=3 |
    96	
    97	**Verdict on infrastructure**: PROCEED to Atom 7 dual external audit.
    98	**Verdict on fresh arena run**: BLOCKED on mathlib build + Atom 6.1
    99	multi-task aggregation; user-side action needed.
   100	
   101	---
   102	
   103	## Halt-trigger battery (architect §7.7 + design §10)
   104	
   105	| ID | Trigger | Status |
   106	|---|---|---|
   107	| H1 | Pinned-pubkey verify failure halts | ✓ tested via Layer B #8 |
   108	| H2 | Agent-pubkey verify failure halts | ✓ tested via Layer B #9 |
   109	| H3 | Replay state_root mismatch halts | ✓ tested via Layer C #12 |
   110	| H4 | L4 hash chain broken link halts | ✓ tested via Layer B #4 + tamper_report flip_l4_byte |
   111	| H5 | L4.E hash chain broken link halts | ✓ tested via Layer B #6 |
   112	| H6 | L4.E entry advances logical_t halts | ✓ tested via Layer B #6 negative |
   113	| H7 | Unresolved CAS Cid halts | ✓ **demonstrated** via TB-13 fixture's E #27 halt |
   114	| H8 | Projection contains autopsy private_detail halts | ✓ tested via Layer F #28 |
   115	| H9 | TypicalErrorSummary contains private_detail halts | ✓ tested via Layer F #30 |
   116	| H10 | Markov constitution_hash mismatch halts | ✓ tested via Layer G #32 + capsule generation |
   117	| H11 | Markov deep-history without override halts | ✓ verified via `try_deep_history_read_with_override_check` + binary smoke (`generate_markov_capsule` log: "TURINGOS_MARKOV_OVERRIDE not set — deep-history reads DEFAULT-DENIED (FR-15.5 + halt-trigger #6)") |
   118	| H12 | LLM self-narrative in autopsy evidence halts | ✓ tested via Layer F supplemental |
   119	| H13 | total_supply_micro mutates halts | ✓ tested via Layer D #18 |
   120	
   121	**13/13 halt-trigger fence active.** Atom 1's `tests/tb_16_halt_triggers.rs`
   122	13 tests all PASS.
   123	
   124	---
   125	
   126	## Cross-references
   127	
   128	- TB-16 charter: `handover/tracer_bullets/TB-16_charter_2026-05-04.md`
   129	- Architect spec: `handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md` §7
   130	- Design doc: `handover/tests/REAL_LLM_COMPREHENSIVE_AUDIT_FROM_TAPE_DESIGN_2026-05-04.md`
   131	- Predecessor smoke evidence (chain-backed): `handover/evidence/tb_13_real_llm_smoke_2026-05-03/`
   132	- Closes: `handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md` (dashboard §15 live regen)

exec
/bin/bash -lc "nl -ba handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | sed -n '762,906p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   762	# 7. TB-16 — Controlled Market Smoke Arena
   763	
   764	## 7.1 目标
   765	
   766	在受控沙盒中跑通：
   767	
   768	```text
   769	compute + position + complete set + price + mask + autopsy
   770	```
   771	
   772	仍不开放真实市场。
   773	
   774	---
   775	
   776	## 7.2 Scenario
   777	
   778	```text
   779	Lean task
   780	multiple Agents
   781	WorkTx FirstLong
   782	ChallengeTx Short
   783	CompleteSet share inventory
   784	PriceIndex updates
   785	Boltzmann scheduler selects next candidate
   786	some agents lose positions
   787	Autopsy generated
   788	```
   789	
   790	---
   791	
   792	## 7.3 Functional requirements
   793	
   794	```text
   795	FR-16.1
   796	At least 3 agents participate.
   797	
   798	FR-16.2
   799	At least one WorkTx creates FirstLongPosition.
   800	
   801	FR-16.3
   802	At least one ChallengeTx creates ShortPosition.
   803	
   804	FR-16.4
   805	At least one CompleteSetMintTx exists.
   806	
   807	FR-16.5
   808	At least one price update occurs.
   809	
   810	FR-16.6
   811	At least one Boltzmann mask event occurs.
   812	
   813	FR-16.7
   814	At least one AutopsyCapsule is generated.
   815	```
   816	
   817	---
   818	
   819	## 7.4 Constitutional requirements
   820	
   821	```text
   822	CR-16.1
   823	Total Coin conserved.
   824	
   825	CR-16.2
   826	No ghost liquidity.
   827	
   828	CR-16.3
   829	No price overriding predicates.
   830	
   831	CR-16.4
   832	No raw failure broadcast.
   833	
   834	CR-16.5
   835	No real user funds.
   836	
   837	CR-16.6
   838	All activity replayable from ChainTape + CAS.
   839	
   840	CR-16.7
   841	All market activity is sandbox-labeled.
   842	```
   843	
   844	---
   845	
   846	## 7.5 Ship gates
   847	
   848	```text
   849	SG-16.1
   850	Controlled market smoke produces replayable ChainTape.
   851	
   852	SG-16.2
   853	Dashboard shows positions, prices, masks, autopsies.
   854	
   855	SG-16.3
   856	No fake accepted nodes.
   857	
   858	SG-16.4
   859	Unsolved tasks show failure evidence / bankruptcy anchors.
   860	
   861	SG-16.5
   862	All market balances conserved.
   863	
   864	SG-16.6
   865	No unresolved evidence gaps.
   866	
   867	SG-16.7
   868	At least one loss -> autopsy path.
   869	
   870	SG-16.8
   871	Sandbox flag prevents real-money interpretation.
   872	```
   873	
   874	---
   875	
   876	## 7.6 Forbidden
   877	
   878	```text
   879	No public chain.
   880	No real-money market.
   881	No external domain.
   882	No unbounded leverage.
   883	No AMM trading unless explicitly scoped.
   884	No DPMM / pro-rata.
   885	No medical/legal/financial domains.
   886	No production user funds.
   887	```
   888	
   889	---
   890	
   891	## 7.7 Loop-mode instruction
   892	
   893	Risk class:
   894	
   895	```text
   896	Class 3 integration smoke
   897	```
   898	
   899	AI coder may implement autonomously, but ship requires external audit.
   900	
   901	Halt if:
   902	
   903	```text
   904	any conservation failure;
   905	raw log leak;
   906	price-as-truth behavior;

exec
/bin/bash -lc "nl -ba handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md | sed -n '899,914p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   899	AI coder may implement autonomously, but ship requires external audit.
   900	
   901	Halt if:
   902	
   903	```text
   904	any conservation failure;
   905	raw log leak;
   906	price-as-truth behavior;
   907	non-sandbox funds used;
   908	unresolved evidence gap.
   909	```
   910	
   911	---
   912	
   913	# 8. TB-17 — Real-World Readiness Gate
   914	

exec
/bin/bash -lc 'rg -n "default_pput_preseed_pairs|30_000_000|tb7-7-sponsor|Agent_user_0|Agent_solver|Agent_verifier|bootstrap" src/runtime/bootstrap.rs src/runtime/mod.rs genesis_payload.toml tests/tb_16_halt_triggers.rs experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:108:    assert!(plan.contains("tb7-7-sponsor"));
experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:109:    assert!(plan.contains("Agent_solver_"));
experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs:110:    assert!(plan.contains("Agent_user_0"));
src/runtime/bootstrap.rs:4://! fresh chaintape is bootstrapped. Both the evaluator binary and the new
src/runtime/bootstrap.rs:6://! bootstraps the chain first produces the SAME genesis QState — ensuring
src/runtime/bootstrap.rs:11://! this factory is consumed ONLY at chaintape bootstrap (genesis QState
src/runtime/bootstrap.rs:19://! future edits to this factory; only fresh bootstraps consume the current
src/runtime/bootstrap.rs:33:/// 1. `tb7-7-sponsor` (10_000_000 micro = 10 Coin) — TB-7.7 D3 self-funded
src/runtime/bootstrap.rs:37:/// 2. `Agent_user_0` (10_000_000 micro = 10 Coin) — **TB-10 Atom 1 net-new**;
src/runtime/bootstrap.rs:44:/// Total preseed supply = 10_000_000 + 10_000_000 + 10 × 1_000_000 = 30_000_000 micro
src/runtime/bootstrap.rs:48:/// (genesis QState would depend on env at bootstrap time). The factory is
src/runtime/bootstrap.rs:52:pub fn default_pput_preseed_pairs() -> Vec<(AgentId, MicroCoin)> {
src/runtime/bootstrap.rs:55:            AgentId("tb7-7-sponsor".into()),
src/runtime/bootstrap.rs:59:            AgentId("Agent_user_0".into()),
src/runtime/bootstrap.rs:76:    /// U1 — factory returns 12 entries: 1 tb7-7-sponsor + 1 Agent_user_0 + 10 Agent_i.
src/runtime/bootstrap.rs:79:        let pairs = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:86:        for (agent, balance) in default_pput_preseed_pairs() {
src/runtime/bootstrap.rs:95:    /// U3 — Agent_user_0 is present with the documented sponsor budget.
src/runtime/bootstrap.rs:98:        let pairs = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:101:            .find(|(a, _)| a.0 == "Agent_user_0")
src/runtime/bootstrap.rs:102:            .expect("Agent_user_0 must be in preseed list");
src/runtime/bootstrap.rs:106:            "Agent_user_0 sponsor budget"
src/runtime/bootstrap.rs:110:    /// U4 — tb7-7-sponsor is preserved (back-compat with TB-7.7 D3 evaluator preseed).
src/runtime/bootstrap.rs:113:        let pairs = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:116:            .find(|(a, _)| a.0 == "tb7-7-sponsor")
src/runtime/bootstrap.rs:117:            .expect("tb7-7-sponsor must be in preseed list");
src/runtime/bootstrap.rs:121:            "tb7-7-sponsor budget"
src/runtime/bootstrap.rs:128:        let pairs = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:139:    /// U6 — total preseed supply is 30_000_000 micro.
src/runtime/bootstrap.rs:142:        let total: i64 = default_pput_preseed_pairs()
src/runtime/bootstrap.rs:146:        assert_eq!(total, 30_000_000, "total preseed micro");
src/runtime/bootstrap.rs:152:        let a = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:153:        let b = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:166:        let pairs = default_pput_preseed_pairs();
src/runtime/bootstrap.rs:175:        assert_eq!(total, 30_000_000, "genesis balances Σ");
tests/tb_16_halt_triggers.rs:193:    let autopsies = vec![mk("Agent_solver_0", bytes[0]), mk("Agent_solver_1", bytes[1]), mk("Agent_solver_2", bytes[2])];
tests/tb_16_halt_triggers.rs:261:    // divergence from genesis 30_000_000μC. Layer D verified at
tests/tb_16_halt_triggers.rs:268:    // Genesis preseed total = 30_000_000 (verified by bootstrap module).
tests/tb_16_halt_triggers.rs:269:    use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
tests/tb_16_halt_triggers.rs:270:    let total: i64 = default_pput_preseed_pairs()
tests/tb_16_halt_triggers.rs:274:    assert_eq!(total, 30_000_000, "H13: genesis preseed total micro must equal 30_000_000μC");
genesis_payload.toml:219:# 2026-05-02 TB-10 Atom 1 — `runtime::bootstrap` module (NEW file). Single source of truth preseed factory `default_pput_preseed_pairs()` consumed by both evaluator's `--task-mode self|both` preseed branch and `lean_market` user CLI bootstrap. Pure function; replay-deterministic; produces 12 entries (tb7-7-sponsor + Agent_user_0 + Agent_0..9) summing to 30_000_000 micro genesis supply.
genesis_payload.toml:220:"src/runtime/bootstrap.rs" = "78c09eabd3a0b226c84539f414965347eb6928861141bb82af3bbaa5986cd58e"
genesis_payload.toml:244:# 2026-05-01 TB-7 Atom 6 — chain-backed smoke (synthetic-LLM end-to-end) integration test (NEW file). I110 ship-gate: bootstrap chaintape, submit 3 synthetic-agent WorkTx + VerifyTx pairs through bus.submit_typed_tx, run verify_chaintape (all 7 indicators GREEN — Gates 4 + 5 wired evidence), compute_run_facts_from_chain (Gate 6 round-trip), persist smoke evidence to handover/evidence/tb_7_chaintape_smoke_2026-05-01/. Real-LLM smoke documented as manual procedure in test header.
src/runtime/mod.rs:1://! TB-6 Atom 1 — Production ChainTape runtime bootstrap.
src/runtime/mod.rs:56:/// TRACE_MATRIX FC3-N43 (TB-15 Atom 5; architect §6.2 + FR-15.4 + FR-15.5): `MarkovEvidenceCapsule` schema + writer + default-deny deep-history gate. End-of-TB rollup binding constitution_hash + L4 root + L4.E root + CAS root + previous capsule + typical_errors + unresolved_obs + next_session_context_cid. Default next-session bootstrap source; deeper history requires `TURINGOS_MARKOV_OVERRIDE=1`.
src/runtime/mod.rs:59:/// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState. Single source of truth for `tb7-7-sponsor` + `Agent_user_0` + `Agent_0..9` initial balances. Consumed by both evaluator (`--task-mode self|both`) and `lean_market` user CLI bootstrap. Pure function; replay-deterministic.
src/runtime/mod.rs:60:pub mod bootstrap;
src/runtime/mod.rs:203:/// TRACE_MATRIX FC3-N1: bootstrap error class — fail-closed admission gate for production ChainTape.
src/runtime/mod.rs:205:/// Errors produced by the factory at bootstrap time.
src/runtime/mod.rs:212:    /// Atom 1 fail-closed: refuse to bootstrap a `Sequencer` (which always
src/runtime/mod.rs:221:    /// refuse to bootstrap when the L4.E rejection writer cannot open its
src/runtime/mod.rs:283:/// at bootstrap so `verify_chaintape` (Atom 4) can re-verify `system_signature`
src/runtime/mod.rs:524:// `tests/tb_6_runtime_chaintape_bootstrap.rs` and land in Atom 1.3.
src/runtime/mod.rs:544:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
src/runtime/mod.rs:558:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
src/runtime/mod.rs:575:        // First bootstrap on empty repo — succeeds.
src/runtime/mod.rs:576:        let bundle = build_chaintape_sequencer(&cfg).expect("first bootstrap");
src/runtime/mod.rs:581:        // the FRESH bootstrap on an EMPTY but git-init'd repo still succeeds
src/runtime/mod.rs:585:        // cleaner to exercise in tb_6_runtime_chaintape_bootstrap.rs (Atom 1.3).
src/runtime/mod.rs:587:        // For Atom 1.1 in-crate coverage: confirm the second bootstrap (with
src/runtime/mod.rs:589:        let bundle2 = build_chaintape_sequencer(&cfg).expect("second bootstrap on empty refs");
src/runtime/mod.rs:597:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");

exec
/bin/bash -lc "nl -ba src/runtime/bootstrap.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-10 Atom 1 — Reusable preseed factory for chaintape genesis QState.
     2	//!
     3	//! Single source of truth for the initial `balances_t` map populated when a
     4	//! fresh chaintape is bootstrapped. Both the evaluator binary and the new
     5	//! `lean_market` user CLI call this factory so that whichever process
     6	//! bootstraps the chain first produces the SAME genesis QState — ensuring
     7	//! the user CLI and evaluator can both attach to the same on-disk chaintape
     8	//! and observe consistent balances.
     9	//!
    10	//! **Constitutional gate** (Art. III.4 / P3 kill #1 — "no post-init mint"):
    11	//! this factory is consumed ONLY at chaintape bootstrap (genesis QState
    12	//! construction via `runtime::adapter::genesis_with_balances`). It is NOT a
    13	//! runtime mint path. `assert_no_post_init_mint` continues to fire on every
    14	//! subsequent typed_tx and rejects any non-genesis mint attempt.
    15	//!
    16	//! **Replay determinism**: the function is pure (no env reads, no clock,
    17	//! no randomness). Two calls produce byte-identical Vec output. Past chains
    18	//! continue to replay from their on-disk genesis_report.json regardless of
    19	//! future edits to this factory; only fresh bootstraps consume the current
    20	//! version.
    21	//!
    22	//! Per `handover/audits/CHARTER_RATIFICATION_TB_10_2026-05-02.md` §1 Q2 +
    23	//! §2.4. Consolidates the inline literal previously at
    24	//! `experiments/minif2f_v4/src/bin/evaluator.rs:716-731`.
    25	
    26	use crate::economy::money::MicroCoin;
    27	use crate::state::q_state::AgentId;
    28	
    29	/// TRACE_MATRIX FC2 Boot: TB-10 Atom 1 — sponsor + user-sponsor + 10 solver agent budgets.
    30	///
    31	/// The 12 entries (in stable insertion order):
    32	///
    33	/// 1. `tb7-7-sponsor` (10_000_000 micro = 10 Coin) — TB-7.7 D3 self-funded
    34	///    sponsor used by evaluator's `--task-mode self|both` preseed branch
    35	///    (`evaluator.rs:864-922`). Preserved for back-compat with TB-7+
    36	///    smoke harness.
    37	/// 2. `Agent_user_0` (10_000_000 micro = 10 Coin) — **TB-10 Atom 1 net-new**;
    38	///    sponsor identity used by `lean_market post-task` subcommand.
    39	///    `Agent_user_` prefix is the audit_dashboard §11 filter convention
    40	///    (per ratification §2.3).
    41	/// 3-12. `Agent_0..9` (1_000_000 micro = 1 Coin each) — solver budgets;
    42	///    plenty for ~1000 WorkTx.stake at 1_000 each.
    43	///
    44	/// Total preseed supply = 10_000_000 + 10_000_000 + 10 × 1_000_000 = 30_000_000 micro
    45	/// = 30 Coin.
    46	///
    47	/// **Why not env-driven**: env-driven preseed would break replay determinism
    48	/// (genesis QState would depend on env at bootstrap time). The factory is
    49	/// the deterministic substrate; specific runs that need different starting
    50	/// balances should construct their own preseed Vec and call
    51	/// `genesis_with_balances` directly.
    52	pub fn default_pput_preseed_pairs() -> Vec<(AgentId, MicroCoin)> {
    53	    let mut pairs: Vec<(AgentId, MicroCoin)> = vec![
    54	        (
    55	            AgentId("tb7-7-sponsor".into()),
    56	            MicroCoin::from_micro_units(10_000_000),
    57	        ),
    58	        (
    59	            AgentId("Agent_user_0".into()),
    60	            MicroCoin::from_micro_units(10_000_000),
    61	        ),
    62	    ];
    63	    for i in 0..10 {
    64	        pairs.push((
    65	            AgentId(format!("Agent_{i}")),
    66	            MicroCoin::from_micro_units(1_000_000),
    67	        ));
    68	    }
    69	    pairs
    70	}
    71	
    72	#[cfg(test)]
    73	mod tests {
    74	    use super::*;
    75	
    76	    /// U1 — factory returns 12 entries: 1 tb7-7-sponsor + 1 Agent_user_0 + 10 Agent_i.
    77	    #[test]
    78	    fn returns_12_entries() {
    79	        let pairs = default_pput_preseed_pairs();
    80	        assert_eq!(pairs.len(), 12, "expected 12 preseed entries");
    81	    }
    82	
    83	    /// U2 — every entry has positive balance (no zero-funded agent).
    84	    #[test]
    85	    fn every_entry_has_positive_balance() {
    86	        for (agent, balance) in default_pput_preseed_pairs() {
    87	            assert!(
    88	                balance.micro_units() > 0,
    89	                "agent {} has zero balance",
    90	                agent.0
    91	            );
    92	        }
    93	    }
    94	
    95	    /// U3 — Agent_user_0 is present with the documented sponsor budget.
    96	    #[test]
    97	    fn agent_user_0_present_with_sponsor_budget() {
    98	        let pairs = default_pput_preseed_pairs();
    99	        let user_entry = pairs
   100	            .iter()
   101	            .find(|(a, _)| a.0 == "Agent_user_0")
   102	            .expect("Agent_user_0 must be in preseed list");
   103	        assert_eq!(
   104	            user_entry.1.micro_units(),
   105	            10_000_000,
   106	            "Agent_user_0 sponsor budget"
   107	        );
   108	    }
   109	
   110	    /// U4 — tb7-7-sponsor is preserved (back-compat with TB-7.7 D3 evaluator preseed).
   111	    #[test]
   112	    fn tb_7_7_sponsor_preserved() {
   113	        let pairs = default_pput_preseed_pairs();
   114	        let sponsor_entry = pairs
   115	            .iter()
   116	            .find(|(a, _)| a.0 == "tb7-7-sponsor")
   117	            .expect("tb7-7-sponsor must be in preseed list");
   118	        assert_eq!(
   119	            sponsor_entry.1.micro_units(),
   120	            10_000_000,
   121	            "tb7-7-sponsor budget"
   122	        );
   123	    }
   124	
   125	    /// U5 — 10 solver agents Agent_0..Agent_9 each at 1_000_000 micro.
   126	    #[test]
   127	    fn ten_solver_agents_each_one_coin() {
   128	        let pairs = default_pput_preseed_pairs();
   129	        for i in 0..10 {
   130	            let id = format!("Agent_{i}");
   131	            let entry = pairs
   132	                .iter()
   133	                .find(|(a, _)| a.0 == id)
   134	                .unwrap_or_else(|| panic!("Agent_{i} must be in preseed list"));
   135	            assert_eq!(entry.1.micro_units(), 1_000_000, "Agent_{i} budget");
   136	        }
   137	    }
   138	
   139	    /// U6 — total preseed supply is 30_000_000 micro.
   140	    #[test]
   141	    fn total_preseed_supply_30m() {
   142	        let total: i64 = default_pput_preseed_pairs()
   143	            .iter()
   144	            .map(|(_, m)| m.micro_units())
   145	            .sum();
   146	        assert_eq!(total, 30_000_000, "total preseed micro");
   147	    }
   148	
   149	    /// U7 — factory is deterministic: two calls produce byte-identical output.
   150	    #[test]
   151	    fn deterministic_across_calls() {
   152	        let a = default_pput_preseed_pairs();
   153	        let b = default_pput_preseed_pairs();
   154	        assert_eq!(a.len(), b.len());
   155	        for ((a_id, a_m), (b_id, b_m)) in a.iter().zip(b.iter()) {
   156	            assert_eq!(a_id.0, b_id.0);
   157	            assert_eq!(a_m.micro_units(), b_m.micro_units());
   158	        }
   159	    }
   160	
   161	    /// U8 — feeding the factory output into genesis_with_balances yields a
   162	    /// QState whose balances_t Σ matches the documented 30M total.
   163	    #[test]
   164	    fn genesis_construction_matches_total() {
   165	        use crate::runtime::adapter::genesis_with_balances;
   166	        let pairs = default_pput_preseed_pairs();
   167	        let q = genesis_with_balances(&pairs);
   168	        let total: i64 = q
   169	            .economic_state_t
   170	            .balances_t
   171	            .0
   172	            .values()
   173	            .map(|m| m.micro_units())
   174	            .sum();
   175	        assert_eq!(total, 30_000_000, "genesis balances Σ");
   176	    }
   177	}

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/tests/tb_16_comprehensive_arena_smoke.rs | sed -n '1,220p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-16 Atom 5 — `comprehensive_arena` smoke test.
     2	//!
     3	//! Verifies:
     4	//! - The binary builds + responds to --help.
     5	//! - --plan-only mode emits an ARENA_PLAN.md with the 6-task block,
     6	//!   sandbox preseed manifest, and architect §7.7 halt-trigger summary.
     7	//! - All 13 architect-required tx kinds are referenced in the emitted plan.
     8	//!
     9	//! Real-LLM end-to-end execution is exercised by Atom 6's
    10	//! handover/tests/scripts/run_real_llm_arena.sh; that path is NOT
    11	//! covered by this unit test (gated on LLM proxy availability + 30 min
    12	//! wall clock).
    13	//!
    14	//! TRACE_MATRIX FC1-N36.
    15	
    16	use std::path::PathBuf;
    17	use std::process::Command;
    18	
    19	fn manifest_dir() -> PathBuf {
    20	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    21	}
    22	
    23	fn target_bin(name: &str) -> PathBuf {
    24	    // The minif2f_v4 package builds binaries into the WORKSPACE target/
    25	    // dir, not the package-local target/. Use parent traversal.
    26	    let workspace_root = manifest_dir()
    27	        .parent()
    28	        .and_then(|p| p.parent())
    29	        .map(|p| p.to_path_buf())
    30	        .unwrap_or_else(|| manifest_dir());
    31	    let dbg = workspace_root.join("target").join("debug").join(name);
    32	    if dbg.exists() {
    33	        return dbg;
    34	    }
    35	    panic!("binary {name} not built at {dbg:?}");
    36	}
    37	
    38	#[test]
    39	fn comprehensive_arena_help_succeeds() {
    40	    let bin = target_bin("comprehensive_arena");
    41	    let out = Command::new(&bin)
    42	        .arg("--help")
    43	        .output()
    44	        .expect("comprehensive_arena --help");
    45	    let combined = format!(
    46	        "{}{}",
    47	        String::from_utf8_lossy(&out.stderr),
    48	        String::from_utf8_lossy(&out.stdout)
    49	    );
    50	    assert!(
    51	        combined.contains("comprehensive_arena") && combined.contains("USAGE"),
    52	        "help text malformed: {combined}"
    53	    );
    54	}
    55	
    56	#[test]
    57	fn comprehensive_arena_plan_only_emits_plan() {
    58	    let bin = target_bin("comprehensive_arena");
    59	    let out_dir = std::env::temp_dir().join(format!(
    60	        "tb16_arena_smoke_{}",
    61	        std::process::id()
    62	    ));
    63	    let _ = std::fs::remove_dir_all(&out_dir);
    64	    let status = Command::new(&bin)
    65	        .arg("--out-dir")
    66	        .arg(&out_dir)
    67	        .arg("--plan-only")
    68	        .status()
    69	        .expect("comprehensive_arena run");
    70	    assert!(status.success(), "comprehensive_arena exited {status:?}");
    71	    let plan_path = out_dir.join("ARENA_PLAN.md");
    72	    assert!(plan_path.exists(), "ARENA_PLAN.md missing at {plan_path:?}");
    73	
    74	    let plan = std::fs::read_to_string(&plan_path).expect("read plan");
    75	
    76	    // 6-task block present
    77	    for label in [
    78	        "A_happy_path",
    79	        "B_challenge_dismissed",
    80	        "C_challenge_upheld",
    81	        "D_exhaustion",
    82	        "E_expiry",
    83	        "F_complete_set_market",
    84	    ] {
    85	        assert!(plan.contains(label), "task `{label}` missing from plan");
    86	    }
    87	
    88	    // 13 architect-required tx kinds referenced
    89	    for tx_kind in [
    90	        "Work",
    91	        "Verify",
    92	        "Challenge",
    93	        "TaskOpen",
    94	        "EscrowLock",
    95	        "CompleteSetMint",
    96	        "CompleteSetRedeem",
    97	        "MarketSeed",
    98	        "FinalizeReward",
    99	        "ChallengeResolve",
   100	        "TerminalSummary",
   101	        "TaskExpire",
   102	        "TaskBankruptcy",
   103	    ] {
   104	        assert!(plan.contains(tx_kind), "tx_kind `{tx_kind}` missing from plan");
   105	    }
   106	
   107	    // Sandbox preseed sandbox-labeled
   108	    assert!(plan.contains("tb7-7-sponsor"));
   109	    assert!(plan.contains("Agent_solver_"));
   110	    assert!(plan.contains("Agent_user_0"));
   111	
   112	    // Architect §7.6 forbidden + §7.7 halt triggers section present
   113	    assert!(plan.contains("Forbidden") || plan.contains("forbidden"));
   114	    assert!(plan.contains("Halt trigger") || plan.contains("halt trigger"));
   115	}

exec
/bin/bash -lc "python3 - <<'PY'
import json, pathlib
p=pathlib.Path('handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json')
v=json.load(open(p))
print('schema',v.get('schema_version'))
print('verdict',v.get('verdict'),'passed',v.get('passed'),'failed',v.get('failed'),'halted',v.get('halted'),'skipped',v.get('skipped'),'assertions',len(v.get('assertions',[])))
print('tape_root_keys',list(v.get('tape_root',{}).keys()))
print('tx_kind_counts_keys',list(v.get('tx_kind_counts',{}).keys()))
print('tx_kind_counts',v.get('tx_kind_counts'))
print('feature_coverage',v.get('feature_coverage'))
from collections import Counter
print('assertion fields', Counter(tuple(sorted(a.keys())) for a in v.get('assertions',[])))
print('bad sum?', v.get('passed',0)+v.get('failed',0)+v.get('halted',0)+v.get('skipped',0))
for a in v.get('assertions',[]):
    if a.get('result')"'!='"'Pass': print(a)
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
schema v1/audit_tape_verdict
verdict BLOCK passed 31 failed 0 halted 1 skipped 7 assertions 39
tape_root_keys ['l4_count', 'l4e_count', 'head_state_root_hex', 'head_ledger_root_hex', 'cas_object_count', 'constitution_hash_hex']
tx_kind_counts_keys ['work', 'verify', 'challenge', 'reuse', 'task_open', 'escrow_lock', 'complete_set_mint', 'complete_set_redeem', 'market_seed', 'finalize_reward', 'challenge_resolve', 'terminal_summary', 'task_expire', 'task_bankruptcy']
tx_kind_counts {'work': 0, 'verify': 0, 'challenge': 0, 'reuse': 0, 'task_open': 1, 'escrow_lock': 1, 'complete_set_mint': 0, 'complete_set_redeem': 0, 'market_seed': 0, 'finalize_reward': 0, 'challenge_resolve': 0, 'terminal_summary': 1, 'task_expire': 0, 'task_bankruptcy': 0}
feature_coverage {'TB-11_terminal_bankruptcy_expire': 'GREEN', 'TB-13_complete_set': 'RED', 'TB-14_price_mask': 'GREEN', 'TB-15_autopsy_markov': 'RED', 'TB-1_monetary': 'GREEN', 'TB-2_work': 'RED', 'TB-3_task_open_escrow': 'GREEN', 'TB-4_verify_challenge': 'RED', 'TB-5_challenge_resolve': 'RED', 'TB-6_chain': 'GREEN', 'TB-7_agent_pubkeys': 'GREEN', 'TB-8_finalize_reward': 'RED'}
assertion fields Counter({('detail', 'id', 'layer', 'name', 'result'): 39})
bad sum? 39
{'id': 27, 'name': 'terminal_summary_evidence_capsule', 'layer': 'E', 'result': 'Halt', 'detail': 'evidence_capsule_cid not in CAS at L4 index 2'}
{'id': 32, 'name': 'markov_constitution_hash_matches', 'layer': 'G', 'result': 'Skipped', 'detail': 'no Markov capsule'}
{'id': 33, 'name': 'markov_typical_errors_recompute', 'layer': 'G', 'result': 'Skipped', 'detail': 'no Markov capsule'}
{'id': 34, 'name': 'markov_unresolved_obs_recompute', 'layer': 'G', 'result': 'Skipped', 'detail': 'no Markov capsule'}
{'id': 35, 'name': 'markov_next_session_context_resolves', 'layer': 'G', 'result': 'Skipped', 'detail': 'no Markov capsule'}
{'id': 36, 'name': 'tamper_l4_flip_detected', 'layer': 'H', 'result': 'Skipped', 'detail': 'exercised by audit_tape_tamper binary (Atom 3)'}
{'id': 37, 'name': 'tamper_cas_flip_detected', 'layer': 'H', 'result': 'Skipped', 'detail': 'exercised by audit_tape_tamper binary (Atom 3)'}
{'id': 38, 'name': 'tamper_l4_remove_detected', 'layer': 'H', 'result': 'Skipped', 'detail': 'exercised by audit_tape_tamper binary (Atom 3)'}

exec
/bin/bash -lc "python3 - <<'PY'
import json, pathlib
for f in ['agent_pubkeys.json','pinned_pubkeys.json','genesis_report.json']:
  paths=list(pathlib.Path('handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke').rglob(f))
  print('\\n',f, [str(p) for p in paths[:5]], 'count',len(paths))
  for p in paths[:1]:
    try:
      obj=json.load(open(p)); print(json.dumps(obj, indent=2)[:4000])
    except Exception as e: print('ERR',e)
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

 agent_pubkeys.json ['handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/agent_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_cas_byte/runtime_repo/agent_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/agent_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/truncate_l4_ref/runtime_repo/agent_pubkeys.json'] count 4
{
  "agents": {}
}

 pinned_pubkeys.json ['handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/pinned_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_cas_byte/runtime_repo/pinned_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/pinned_pubkeys.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/truncate_l4_ref/runtime_repo/pinned_pubkeys.json'] count 4
{
  "run_id": "tb13-smoke",
  "tb_id": "TB-6",
  "epoch": 1,
  "pubkeys": [
    {
      "epoch": 1,
      "pubkey_hex": "934d9419198bc929a9b9e8b29665035cf482f481122057a990465436f9144fbe"
    }
  ]
}

 genesis_report.json ['handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/genesis_report.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_cas_byte/runtime_repo/genesis_report.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/genesis_report.json', 'handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/truncate_l4_ref/runtime_repo/genesis_report.json'] count 4
{
  "constitution_hash": null,
  "runtime_repo": "/tmp/tb13_smoke_1777790073/runtime_repo",
  "cas_path": "/tmp/tb13_smoke_1777790073/cas",
  "system_pubkey_hash": "0060098efa84b5f113d15cdc978865fed5149ad1ef9c91cefd6aeb5b79a4a59f",
  "agent_pubkeys_path": "agent_pubkeys.json",
  "initial_balances": [
    [
      "tb7-7-sponsor",
      10000000
    ],
    [
      "Agent_user_0",
      10000000
    ],
    [
      "Agent_0",
      1000000
    ],
    [
      "Agent_1",
      1000000
    ],
    [
      "Agent_2",
      1000000
    ],
    [
      "Agent_3",
      1000000
    ],
    [
      "Agent_4",
      1000000
    ],
    [
      "Agent_5",
      1000000
    ],
    [
      "Agent_6",
      1000000
    ],
    [
      "Agent_7",
      1000000
    ],
    [
      "Agent_8",
      1000000
    ],
    [
      "Agent_9",
      1000000
    ]
  ],
  "task_id": "task-n1_mathd_algebra_171_1777790080974",
  "task_open_tx": "taskopen-task-n1_mathd_algebra_171_1777790080974-tb7-7-d3-seed",
  "escrow_lock_tx": "escrowlock-task-n1_mathd_algebra_171_1777790080974-tb7-7-d3-escrow"
}

exec
/bin/bash -lc "python3 - <<'PY'
import json, pathlib
p=pathlib.Path('handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json')
if p.exists():
 v=json.load(open(p)); print(json.dumps(v, indent=2)[:8000])
else: print('missing')
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
{
  "all_detected": true,
  "detected_count": 3,
  "expected": 3,
  "schema_version": "v1/audit_tape_tamper",
  "tamper_results": [
    {
      "detail": "flipped byte 41 in \"handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/.git/objects/68/ba8a6d2eac44efabe41c7bcddc1b9fe778f7c2\"",
      "detected": true,
      "label": "flip_l4_byte",
      "schema_version": "v1/audit_tape_tamper",
      "verdict": {
        "assertions": [
          {
            "detail": null,
            "id": 1,
            "layer": "A",
            "name": "constitution_hash_matches_genesis",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 2,
            "layer": "A",
            "name": "pinned_pubkey_loaded",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 3,
            "layer": "A",
            "name": "sandbox_agent_prefix",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 4,
            "layer": "B",
            "name": "l4_hash_chain_valid",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 5,
            "layer": "B",
            "name": "l4_parent_state_continuity",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 6,
            "layer": "B",
            "name": "l4e_chain_integrity",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 7,
            "layer": "B",
            "name": "genesis_row_zero_parents",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 8,
            "layer": "B",
            "name": "system_tx_signatures_verify",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 9,
            "layer": "B",
            "name": "agent_tx_signatures_verify",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 10,
            "layer": "B",
            "name": "payload_cid_resolves",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 11,
            "layer": "B",
            "name": "tx_kind_envelope_matches_payload",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 12,
            "layer": "C",
            "name": "replay_state_root_matches_head",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 13,
            "layer": "C",
            "name": "replay_economic_state_canonical",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 14,
            "layer": "C",
            "name": "replay_autopsy_index_chains",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 15,
            "layer": "C",
            "name": "canonical_edges_replay_deterministic",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 16,
            "layer": "C",
            "name": "replay_idempotent_across_calls",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 17,
            "layer": "D",
            "name": "no_post_init_mint",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 18,
            "layer": "D",
            "name": "total_supply_conserved",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 19,
            "layer": "D",
            "name": "complete_set_min_balanced",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 20,
            "layer": "D",
            "name": "task_market_total_escrow_matches_locks",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 21,
            "layer": "D",
            "name": "node_positions_excluded_from_supply",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 22,
            "layer": "D",
            "name": "conditional_shares_excluded_from_supply",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 23,
            "layer": "E",
            "name": "accepted_work_predicate_results_true",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 24,
            "layer": "E",
            "name": "proposal_telemetry_chain",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 25,
            "layer": "E",
            "name": "l4e_rejection_class_redispatch",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 26,
            "layer": "E",
            "name": "price_index_is_view_only",
            "result": "Pass"
          },
          {
            "detail": "evidence_capsule_cid not in CAS at L4 index 2",
            "id": 27,
            "layer": "E",
            "name": "terminal_summary_evidence_capsule",
            "result": "Halt"
          },
          {
            "detail": null,
            "id": 28,
            "layer": "F",
            "name": "projection_no_autopsy_bytes",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 29,
            "layer": "F",
            "name": "autopsy_private_detail_creator_is_system",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 30,
            "layer": "F",
            "name": "typical_error_summary_no_private_detail",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 31,
            "layer": "F",
            "name": "autopsy_index_value_type_is_vec_cid",
            "result": "Pass"
          },
          {
            "detail": null,
            "id": 39,
            "layer": "F",
            "name": "no_llm_self_narrative_in_autopsy",
            "result": "Pass"
          },
          {
            "detail": "no Markov capsule",
            "id": 32,
            "layer": "G",
            "name": "markov_constitution_hash_matches",
            "result": "Skipped"
          },
          {
            "detail": "no Markov capsule",
            "id": 33,
            "layer": "G",
            "name": "markov_typical_errors_recompute",
            "result": "Skipped"
          },
          {
            "detail": "no Markov capsule",
            "id": 34,
            "layer": "G",
            "name": "markov_unresolved_obs_recompute",
            "result": "Skipped"
          },
          {
            "detail": "no Markov capsule",
            "id": 35,
            "layer": "G",
            "name": "markov_next_session_context_resolves",
            "result": "Skipped"
          },
          {
            "detail": "exercised by audit_tape_tamper binary (Atom 3)",
            "id": 36,
            "layer": "H",
            "name": "tamper_l4_flip_detected",
            "result": "Skipped"
          },
          {
            "detail": "exercised by audit_tape_tamper binary (Atom 3)",
            "id": 37,
            "layer": "H",
            "name": "tamper_cas_flip_detected",
            "result": "Skipped"
          },
          {
            "detail": "exercised by audit_tape_tamper binary (Atom 3)",
            "id": 38,
            "layer": "H",
            "name": "tamper_l4_remove_detected",
            "result": "Skipped"
          }
        ],
        "failed": 0,
        "feature_coverage": {
          "TB-11_terminal_bankruptcy_expire": "GREEN",
          "TB-13_complete_se

exec
/bin/bash -lc "python3 - <<'PY'
import json
p='handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json'
v=json.load(open(p))
for r in v['tamper_results']:
 print('\\n',r['label'], 'detected',r['detected'])
 print('detail',r.get('detail'))
 verdict=r.get('verdict')
 if verdict:
   print('verdict', verdict['verdict'], 'passed',verdict['passed'],'failed',verdict['failed'],'halted',verdict['halted'],'skipped',verdict['skipped'])
   print('non-pass', [(a['id'],a['name'],a['result'],a.get('detail')) for a in verdict['assertions'] if a['result']"'!='"'Pass'][:10])
 else:
   print('err', r.get('error'))
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

 flip_l4_byte detected True
detail flipped byte 41 in "handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/.git/objects/68/ba8a6d2eac44efabe41c7bcddc1b9fe778f7c2"
verdict BLOCK passed 31 failed 0 halted 1 skipped 7
non-pass [(27, 'terminal_summary_evidence_capsule', 'Halt', 'evidence_capsule_cid not in CAS at L4 index 2'), (32, 'markov_constitution_hash_matches', 'Skipped', 'no Markov capsule'), (33, 'markov_typical_errors_recompute', 'Skipped', 'no Markov capsule'), (34, 'markov_unresolved_obs_recompute', 'Skipped', 'no Markov capsule'), (35, 'markov_next_session_context_resolves', 'Skipped', 'no Markov capsule'), (36, 'tamper_l4_flip_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)'), (37, 'tamper_cas_flip_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)'), (38, 'tamper_l4_remove_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)')]

 flip_cas_byte detected True
detail flipped byte 92 in "handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_cas_byte/cas/.git/objects/1e/d7d36d55098313ca3d53859d100b91d8aa24bd"
verdict BLOCK passed 31 failed 0 halted 1 skipped 7
non-pass [(27, 'terminal_summary_evidence_capsule', 'Halt', 'evidence_capsule_cid not in CAS at L4 index 2'), (32, 'markov_constitution_hash_matches', 'Skipped', 'no Markov capsule'), (33, 'markov_typical_errors_recompute', 'Skipped', 'no Markov capsule'), (34, 'markov_unresolved_obs_recompute', 'Skipped', 'no Markov capsule'), (35, 'markov_next_session_context_resolves', 'Skipped', 'no Markov capsule'), (36, 'tamper_l4_flip_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)'), (37, 'tamper_cas_flip_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)'), (38, 'tamper_l4_remove_detected', 'Skipped', 'exercised by audit_tape_tamper binary (Atom 3)')]

 truncate_l4_ref detected True
detail zeroed last 4 hex chars in "handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/truncate_l4_ref/runtime_repo/.git/refs/transitions/main"
err None

exec
/bin/bash -lc "nl -ba src/bin/audit_tape_tamper.rs | sed -n '1,340p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-16 Atom 3 — `audit_tape_tamper` CLI (architect §7.7 + design §6.2 H).
     2	//!
     3	//! Tamper-detection harness. Forks the input tape into 3 temp copies,
     4	//! introduces a single corruption per copy, then re-runs `audit_tape`
     5	//! over each:
     6	//!
     7	//!   1. flip 1 byte in a random L4 row (via Git2 ledger commit blob)
     8	//!      → verdict.json must emit `BLOCK` with a Layer B fail/halt.
     9	//!   2. flip 1 byte in a random CAS object → verdict.json must emit
    10	//!      `BLOCK` with a Layer B fail/halt.
    11	//!   3. remove a random L4 row by truncating the Git2 ref to N-1
    12	//!      → verdict.json must emit `BLOCK` (replay state-root mismatch).
    13	//!
    14	//! Each corruption is applied to a TEMP COPY of the tape; the original
    15	//! is untouched. Emits `tamper_report.json` summarizing the 3 attempts.
    16	//!
    17	//! Usage:
    18	//!   audit_tape_tamper \
    19	//!     --runtime-repo  <path> \
    20	//!     --cas-dir       <path> \
    21	//!     --agent-pubkeys <path> \
    22	//!     --pinned-pubkeys <path> \
    23	//!     --genesis       <path> \
    24	//!     --constitution  <path> \
    25	//!     --markov-pointer <path> \
    26	//!     [--alignment-dir <path>] \
    27	//!     --tamper-dir    <work-dir> \
    28	//!     --out           <tamper_report.json>
    29	//!
    30	//! Exit code:
    31	//!   0  — all 3 corruptions detected (each verdict was BLOCK)
    32	//!   1  — at least one corruption NOT detected (HALT TRIGGER per architect §7.7)
    33	//!   2  — invalid args / I/O failure
    34	//!
    35	//! TRACE_MATRIX FC1-N35 (audit_tape_tamper binary; design §6.2 #36-#38).
    36	
    37	use std::path::{Path, PathBuf};
    38	use std::process::ExitCode;
    39	
    40	use turingosv4::runtime::audit_assertions::{
    41	    run_all_assertions, summarize_results, AuditInputs, TapeAuditVerdict,
    42	};
    43	
    44	#[derive(Debug, Clone)]
    45	struct Args {
    46	    runtime_repo: PathBuf,
    47	    cas_dir: PathBuf,
    48	    agent_pubkeys: PathBuf,
    49	    pinned_pubkeys: PathBuf,
    50	    genesis: PathBuf,
    51	    constitution: PathBuf,
    52	    markov_pointer: PathBuf,
    53	    alignment_dir: Option<PathBuf>,
    54	    tamper_dir: PathBuf,
    55	    out: PathBuf,
    56	}
    57	
    58	fn parse_args(argv: &[String]) -> Result<Args, String> {
    59	    let mut p: std::collections::BTreeMap<&str, PathBuf> = Default::default();
    60	    let mut i = 0;
    61	    let keys = [
    62	        "--runtime-repo",
    63	        "--cas-dir",
    64	        "--agent-pubkeys",
    65	        "--pinned-pubkeys",
    66	        "--genesis",
    67	        "--constitution",
    68	        "--markov-pointer",
    69	        "--alignment-dir",
    70	        "--tamper-dir",
    71	        "--out",
    72	    ];
    73	    while i < argv.len() {
    74	        let k = argv[i].as_str();
    75	        if k == "-h" || k == "--help" {
    76	            eprint!("{}", help_text());
    77	            std::process::exit(0);
    78	        }
    79	        if !keys.contains(&k) {
    80	            return Err(format!("unknown arg: {k}"));
    81	        }
    82	        i += 1;
    83	        let v = argv.get(i).ok_or_else(|| format!("{k} needs path"))?;
    84	        // unsafe leak via static — OK here, args parsing only.
    85	        let static_k: &'static str = match k {
    86	            "--runtime-repo" => "--runtime-repo",
    87	            "--cas-dir" => "--cas-dir",
    88	            "--agent-pubkeys" => "--agent-pubkeys",
    89	            "--pinned-pubkeys" => "--pinned-pubkeys",
    90	            "--genesis" => "--genesis",
    91	            "--constitution" => "--constitution",
    92	            "--markov-pointer" => "--markov-pointer",
    93	            "--alignment-dir" => "--alignment-dir",
    94	            "--tamper-dir" => "--tamper-dir",
    95	            "--out" => "--out",
    96	            _ => unreachable!(),
    97	        };
    98	        p.insert(static_k, PathBuf::from(v));
    99	        i += 1;
   100	    }
   101	    let mut take = |k: &str| p.remove(k).ok_or_else(|| format!("{k} required"));
   102	    let runtime_repo = take("--runtime-repo")?;
   103	    let cas_dir = take("--cas-dir")?;
   104	    let agent_pubkeys = take("--agent-pubkeys")?;
   105	    let pinned_pubkeys = take("--pinned-pubkeys")?;
   106	    let genesis = take("--genesis")?;
   107	    let constitution = take("--constitution")?;
   108	    let markov_pointer = take("--markov-pointer")?;
   109	    let tamper_dir = take("--tamper-dir")?;
   110	    let out = take("--out")?;
   111	    let alignment_dir = p.remove("--alignment-dir");
   112	    Ok(Args {
   113	        runtime_repo,
   114	        cas_dir,
   115	        agent_pubkeys,
   116	        pinned_pubkeys,
   117	        genesis,
   118	        constitution,
   119	        markov_pointer,
   120	        alignment_dir,
   121	        tamper_dir,
   122	        out,
   123	    })
   124	}
   125	
   126	fn help_text() -> String {
   127	    "audit_tape_tamper — TB-16 Atom 3 tamper-detection harness\n\
   128	     \n\
   129	     USAGE:\n  \
   130	       audit_tape_tamper --runtime-repo <p> --cas-dir <p> ... --tamper-dir <p> --out <p>\n\
   131	     \n\
   132	     EXIT:\n  \
   133	       0  all 3 corruptions detected (BLOCK on each tampered copy)\n  \
   134	       1  at least 1 corruption NOT detected (HALT per architect §7.7)\n  \
   135	       2  invalid args / I/O failure\n"
   136	        .into()
   137	}
   138	
   139	fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
   140	    std::fs::create_dir_all(dst)?;
   141	    for entry in std::fs::read_dir(src)? {
   142	        let entry = entry?;
   143	        let file_type = entry.file_type()?;
   144	        let from = entry.path();
   145	        let to = dst.join(entry.file_name());
   146	        if file_type.is_dir() {
   147	            copy_dir_recursive(&from, &to)?;
   148	        } else if file_type.is_symlink() {
   149	            // Follow symlinks: copy underlying file content.
   150	            if let Ok(meta) = std::fs::metadata(&from) {
   151	                if meta.is_file() {
   152	                    std::fs::copy(&from, &to)?;
   153	                }
   154	            }
   155	        } else {
   156	            std::fs::copy(&from, &to)?;
   157	        }
   158	    }
   159	    Ok(())
   160	}
   161	
   162	fn fork_tape(args: &Args, label: &str) -> Result<(PathBuf, PathBuf), String> {
   163	    let dir = args.tamper_dir.join(label);
   164	    if dir.exists() {
   165	        std::fs::remove_dir_all(&dir).map_err(|e| format!("clear {dir:?}: {e}"))?;
   166	    }
   167	    let runtime_dst = dir.join("runtime_repo");
   168	    let cas_dst = dir.join("cas");
   169	    copy_dir_recursive(&args.runtime_repo, &runtime_dst)
   170	        .map_err(|e| format!("copy runtime_repo: {e}"))?;
   171	    copy_dir_recursive(&args.cas_dir, &cas_dst).map_err(|e| format!("copy cas_dir: {e}"))?;
   172	    Ok((runtime_dst, cas_dst))
   173	}
   174	
   175	fn run_audit(args: &Args, runtime: &Path, cas: &Path) -> Result<TapeAuditVerdict, String> {
   176	    let inputs = AuditInputs {
   177	        runtime_repo: runtime.to_path_buf(),
   178	        cas_dir: cas.to_path_buf(),
   179	        agent_pubkeys: args.agent_pubkeys.clone(),
   180	        pinned_pubkeys: args.pinned_pubkeys.clone(),
   181	        genesis: args.genesis.clone(),
   182	        constitution: args.constitution.clone(),
   183	        markov_pointer: args.markov_pointer.clone(),
   184	        alignment_dir: args.alignment_dir.clone(),
   185	    };
   186	    let results = run_all_assertions(&inputs).map_err(|e| format!("run: {e}"))?;
   187	    summarize_results(&inputs, results).map_err(|e| format!("summarize: {e}"))
   188	}
   189	
   190	fn make_writable(path: &Path) -> std::io::Result<()> {
   191	    let mut perms = std::fs::metadata(path)?.permissions();
   192	    #[cfg(unix)]
   193	    {
   194	        use std::os::unix::fs::PermissionsExt;
   195	        perms.set_mode(0o644);
   196	    }
   197	    #[cfg(not(unix))]
   198	    {
   199	        perms.set_readonly(false);
   200	    }
   201	    std::fs::set_permissions(path, perms)
   202	}
   203	
   204	fn flip_byte_in_first_blob(repo: &Path) -> Result<String, String> {
   205	    // Walk the .git/objects/ tree; pick the first non-empty file; flip
   206	    // a random byte. This corrupts a Git2 object — likely an L4 commit
   207	    // tree or blob. The auditor's verify-side will detect via failed
   208	    // canonical_decode / Cid mismatch / hash chain break.
   209	    let objects = repo.join(".git").join("objects");
   210	    let mut victim: Option<PathBuf> = None;
   211	    fn walk(dir: &Path, victim: &mut Option<PathBuf>) -> std::io::Result<()> {
   212	        for entry in std::fs::read_dir(dir)? {
   213	            let e = entry?;
   214	            let p = e.path();
   215	            if p.is_dir() {
   216	                walk(&p, victim)?;
   217	            } else if victim.is_none() {
   218	                let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
   219	                if len > 0 {
   220	                    *victim = Some(p);
   221	                    return Ok(());
   222	                }
   223	            }
   224	        }
   225	        Ok(())
   226	    }
   227	    walk(&objects, &mut victim).map_err(|e| format!("walk: {e}"))?;
   228	    let victim = victim.ok_or("no objects to corrupt")?;
   229	    let bytes = std::fs::read(&victim).map_err(|e| format!("read victim: {e}"))?;
   230	    let mut bytes = bytes;
   231	    if bytes.is_empty() {
   232	        return Err("empty victim".into());
   233	    }
   234	    let idx = bytes.len() / 2;
   235	    bytes[idx] ^= 0xFF;
   236	    make_writable(&victim).map_err(|e| format!("chmod victim: {e}"))?;
   237	    std::fs::write(&victim, bytes).map_err(|e| format!("write tampered: {e}"))?;
   238	    Ok(format!("flipped byte {idx} in {victim:?}"))
   239	}
   240	
   241	fn flip_byte_in_first_cas_object(cas: &Path) -> Result<String, String> {
   242	    let objects = cas.join(".git").join("objects");
   243	    let dir = if objects.exists() { objects } else { cas.to_path_buf() };
   244	    let mut victim: Option<PathBuf> = None;
   245	    fn walk(dir: &Path, victim: &mut Option<PathBuf>) -> std::io::Result<()> {
   246	        for entry in std::fs::read_dir(dir)? {
   247	            let e = entry?;
   248	            let p = e.path();
   249	            if p.is_dir() {
   250	                walk(&p, victim)?;
   251	            } else if victim.is_none() {
   252	                let len = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
   253	                if len > 32 {
   254	                    *victim = Some(p);
   255	                    return Ok(());
   256	                }
   257	            }
   258	        }
   259	        Ok(())
   260	    }
   261	    walk(&dir, &mut victim).map_err(|e| format!("walk: {e}"))?;
   262	    let victim = victim.ok_or("no CAS objects to corrupt")?;
   263	    let mut bytes = std::fs::read(&victim).map_err(|e| format!("read victim: {e}"))?;
   264	    let idx = bytes.len() / 2;
   265	    bytes[idx] ^= 0xFF;
   266	    make_writable(&victim).map_err(|e| format!("chmod victim: {e}"))?;
   267	    std::fs::write(&victim, bytes).map_err(|e| format!("write tampered: {e}"))?;
   268	    Ok(format!("flipped byte {idx} in {victim:?}"))
   269	}
   270	
   271	fn corrupt_l4_truncate_ref(repo: &Path) -> Result<String, String> {
   272	    // Easiest deterministic-ish corruption: truncate the L4 chain by
   273	    // moving the refs/transitions/main ref back one commit. We don't
   274	    // try to walk parent OIDs in pure Rust here; instead we corrupt
   275	    // the ref file's hex by zeroing the last 4 hex chars — which makes
   276	    // the ref unresolvable, causing Git2LedgerWriter::open() or
   277	    // .read_at() to error → audit_tape returns BLOCK.
   278	    let ref_path = repo.join(".git").join("refs").join("transitions").join("main");
   279	    let alt_ref = repo.join(".git").join("HEAD");
   280	    let target = if ref_path.exists() { ref_path } else { alt_ref };
   281	    let s = std::fs::read_to_string(&target).map_err(|e| format!("read ref: {e}"))?;
   282	    if s.len() < 5 {
   283	        return Err("ref too short to corrupt".into());
   284	    }
   285	    let mut chars: Vec<char> = s.chars().collect();
   286	    let n = chars.len();
   287	    for i in (n - 5)..(n - 1) {
   288	        chars[i] = '0';
   289	    }
   290	    let zeroed: String = chars.into_iter().collect();
   291	    std::fs::write(&target, zeroed).map_err(|e| format!("write ref: {e}"))?;
   292	    Ok(format!("zeroed last 4 hex chars in {target:?}"))
   293	}
   294	
   295	#[derive(serde::Serialize)]
   296	struct TamperReport {
   297	    schema_version: String,
   298	    label: String,
   299	    detected: bool,
   300	    detail: String,
   301	    verdict: Option<TapeAuditVerdict>,
   302	}
   303	
   304	fn run_tamper(
   305	    label: &str,
   306	    args: &Args,
   307	    apply: impl FnOnce(&Path, &Path) -> Result<String, String>,
   308	) -> TamperReport {
   309	    let (runtime, cas) = match fork_tape(args, label) {
   310	        Ok(p) => p,
   311	        Err(e) => {
   312	            return TamperReport {
   313	                schema_version: "v1/audit_tape_tamper".into(),
   314	                label: label.into(),
   315	                detected: false,
   316	                detail: format!("fork failed: {e}"),
   317	                verdict: None,
   318	            };
   319	        }
   320	    };
   321	    let detail = match apply(&runtime, &cas) {
   322	        Ok(d) => d,
   323	        Err(e) => {
   324	            return TamperReport {
   325	                schema_version: "v1/audit_tape_tamper".into(),
   326	                label: label.into(),
   327	                detected: false,
   328	                detail: format!("apply failed: {e}"),
   329	                verdict: None,
   330	            };
   331	        }
   332	    };
   333	    let verdict_res = run_audit(args, &runtime, &cas);
   334	    let (detected, verdict) = match verdict_res {
   335	        Ok(v) => (v.verdict == "BLOCK", Some(v)),
   336	        Err(e) => (true, {
   337	            // Audit refused to load the tape at all; that itself counts
   338	            // as detection (the binary can't proceed past corruption).
   339	            // Emit a synthetic verdict for traceability.
   340	            eprintln!("audit_tape_tamper: load itself failed for `{label}` → counted as detected ({e})");

exec
/bin/bash -lc "nl -ba src/bin/audit_tape.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-16 Atom 3 — `audit_tape` CLI (architect §7.5 + design §6).
     2	//!
     3	//! Pure audit-from-tape binary. Reads ONLY on-disk artifacts:
     4	//!   - runtime_repo/ (Git2 L4 chain + L4.E rejections.jsonl)
     5	//!   - cas/ (CAS store)
     6	//!   - agent_pubkeys.json + pinned_pubkeys.json (per-run manifests)
     7	//!   - genesis_payload.toml + constitution.md
     8	//!   - LATEST_MARKOV_CAPSULE.txt (Markov pointer)
     9	//!   - [optional] handover/alignment/ (OBS scan)
    10	//!
    11	//! NEVER reads:
    12	//!   - live Sequencer state
    13	//!   - state.db (whitebox cache; auditor rebuilds via replay_full_transition)
    14	//!   - process logs
    15	//!   - handover/ai-direct/
    16	//!
    17	//! Emits verdict.json per design §6.3 schema (38 assertions × 8 layers,
    18	//! tape_root, tx_kind_counts, feature_coverage, verdict ∈ {PROCEED, BLOCK}).
    19	//!
    20	//! Usage:
    21	//!   audit_tape \
    22	//!     --runtime-repo  <path> \
    23	//!     --cas-dir       <path> \
    24	//!     --agent-pubkeys <path> \
    25	//!     --pinned-pubkeys <path> \
    26	//!     --genesis       <path> \
    27	//!     --constitution  <path> \
    28	//!     --markov-pointer <path> \
    29	//!     [--alignment-dir <path>] \
    30	//!     --out <verdict.json>
    31	//!
    32	//! Exit code:
    33	//!   0  — verdict.json verdict == "PROCEED"
    34	//!   1  — verdict.json verdict == "BLOCK" (≥1 fail/halt)
    35	//!   2  — invalid args / I/O failure before audit could begin
    36	//!
    37	//! TRACE_MATRIX FC1-N34 (audit_tape binary) + FC2-N31 (verdict.json schema v1).
    38	
    39	use std::path::PathBuf;
    40	use std::process::ExitCode;
    41	
    42	use turingosv4::runtime::audit_assertions::{
    43	    run_all_assertions, summarize_results, AuditInputs,
    44	};
    45	
    46	struct Args {
    47	    runtime_repo: PathBuf,
    48	    cas_dir: PathBuf,
    49	    agent_pubkeys: PathBuf,
    50	    pinned_pubkeys: PathBuf,
    51	    genesis: PathBuf,
    52	    constitution: PathBuf,
    53	    markov_pointer: PathBuf,
    54	    alignment_dir: Option<PathBuf>,
    55	    out: PathBuf,
    56	}
    57	
    58	fn parse_args(argv: &[String]) -> Result<Args, String> {
    59	    let mut runtime_repo: Option<PathBuf> = None;
    60	    let mut cas_dir: Option<PathBuf> = None;
    61	    let mut agent_pubkeys: Option<PathBuf> = None;
    62	    let mut pinned_pubkeys: Option<PathBuf> = None;
    63	    let mut genesis: Option<PathBuf> = None;
    64	    let mut constitution: Option<PathBuf> = None;
    65	    let mut markov_pointer: Option<PathBuf> = None;
    66	    let mut alignment_dir: Option<PathBuf> = None;
    67	    let mut out: Option<PathBuf> = None;
    68	    let mut i = 0;
    69	    while i < argv.len() {
    70	        match argv[i].as_str() {
    71	            "--runtime-repo" => {
    72	                i += 1;
    73	                runtime_repo = Some(argv.get(i).ok_or("--runtime-repo needs path")?.into());
    74	            }
    75	            "--cas-dir" => {
    76	                i += 1;
    77	                cas_dir = Some(argv.get(i).ok_or("--cas-dir needs path")?.into());
    78	            }
    79	            "--agent-pubkeys" => {
    80	                i += 1;
    81	                agent_pubkeys = Some(argv.get(i).ok_or("--agent-pubkeys needs path")?.into());
    82	            }
    83	            "--pinned-pubkeys" => {
    84	                i += 1;
    85	                pinned_pubkeys = Some(argv.get(i).ok_or("--pinned-pubkeys needs path")?.into());
    86	            }
    87	            "--genesis" => {
    88	                i += 1;
    89	                genesis = Some(argv.get(i).ok_or("--genesis needs path")?.into());
    90	            }
    91	            "--constitution" => {
    92	                i += 1;
    93	                constitution = Some(argv.get(i).ok_or("--constitution needs path")?.into());
    94	            }
    95	            "--markov-pointer" => {
    96	                i += 1;
    97	                markov_pointer = Some(argv.get(i).ok_or("--markov-pointer needs path")?.into());
    98	            }
    99	            "--alignment-dir" => {
   100	                i += 1;
   101	                alignment_dir = Some(argv.get(i).ok_or("--alignment-dir needs path")?.into());
   102	            }
   103	            "--out" => {
   104	                i += 1;
   105	                out = Some(argv.get(i).ok_or("--out needs path")?.into());
   106	            }
   107	            "-h" | "--help" => {
   108	                eprint!("{}", help_text());
   109	                std::process::exit(0);
   110	            }
   111	            other => return Err(format!("unknown arg: {other}")),
   112	        }
   113	        i += 1;
   114	    }
   115	    Ok(Args {
   116	        runtime_repo: runtime_repo.ok_or("--runtime-repo required")?,
   117	        cas_dir: cas_dir.ok_or("--cas-dir required")?,
   118	        agent_pubkeys: agent_pubkeys.ok_or("--agent-pubkeys required")?,
   119	        pinned_pubkeys: pinned_pubkeys.ok_or("--pinned-pubkeys required")?,
   120	        genesis: genesis.ok_or("--genesis required")?,
   121	        constitution: constitution.ok_or("--constitution required")?,
   122	        markov_pointer: markov_pointer.ok_or("--markov-pointer required")?,
   123	        alignment_dir,
   124	        out: out.ok_or("--out required")?,
   125	    })
   126	}
   127	
   128	fn help_text() -> String {
   129	    "audit_tape — TB-16 Atom 3 audit-from-tape binary\n\
   130	     \n\
   131	     USAGE:\n  \
   132	       audit_tape --runtime-repo <p> --cas-dir <p> --agent-pubkeys <p>\n  \
   133	                  --pinned-pubkeys <p> --genesis <p> --constitution <p>\n  \
   134	                  --markov-pointer <p> [--alignment-dir <p>] --out <verdict.json>\n\
   135	     \n\
   136	     EXIT:\n  \
   137	       0  verdict == PROCEED (38/38 assertions GREEN)\n  \
   138	       1  verdict == BLOCK (≥1 fail/halt)\n  \
   139	       2  invalid args / I/O failure\n"
   140	        .into()
   141	}
   142	
   143	fn main() -> ExitCode {
   144	    let argv: Vec<String> = std::env::args().skip(1).collect();
   145	    let args = match parse_args(&argv) {
   146	        Ok(a) => a,
   147	        Err(e) => {
   148	            eprintln!("audit_tape: {e}\n\n{}", help_text());
   149	            return ExitCode::from(2);
   150	        }
   151	    };
   152	
   153	    let inputs = AuditInputs {
   154	        runtime_repo: args.runtime_repo,
   155	        cas_dir: args.cas_dir,
   156	        agent_pubkeys: args.agent_pubkeys,
   157	        pinned_pubkeys: args.pinned_pubkeys,
   158	        genesis: args.genesis,
   159	        constitution: args.constitution,
   160	        markov_pointer: args.markov_pointer,
   161	        alignment_dir: args.alignment_dir,
   162	    };
   163	
   164	    let results = match run_all_assertions(&inputs) {
   165	        Ok(r) => r,
   166	        Err(e) => {
   167	            eprintln!("audit_tape: load failed: {e}");
   168	            return ExitCode::from(2);
   169	        }
   170	    };
   171	
   172	    let verdict = match summarize_results(&inputs, results) {
   173	        Ok(v) => v,
   174	        Err(e) => {
   175	            eprintln!("audit_tape: summarize failed: {e}");
   176	            return ExitCode::from(2);
   177	        }
   178	    };
   179	
   180	    let json = match serde_json::to_string_pretty(&verdict) {
   181	        Ok(s) => s,
   182	        Err(e) => {
   183	            eprintln!("audit_tape: json serialize failed: {e}");
   184	            return ExitCode::from(2);
   185	        }
   186	    };
   187	    if let Err(e) = std::fs::write(&args.out, json) {
   188	        eprintln!("audit_tape: write {:?} failed: {e}", args.out);
   189	        return ExitCode::from(2);
   190	    }
   191	
   192	    let proceed = verdict.verdict == "PROCEED";
   193	    println!(
   194	        "audit_tape: verdict={} passed={} failed={} halted={} skipped={} (out={:?})",
   195	        verdict.verdict, verdict.passed, verdict.failed, verdict.halted, verdict.skipped, args.out
   196	    );
   197	    if proceed {
   198	        ExitCode::from(0)
   199	    } else {
   200	        ExitCode::from(1)
   201	    }
   202	}

exec
/bin/bash -lc "nl -ba src/bin/audit_tape_tamper.rs | sed -n '333,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   333	    let verdict_res = run_audit(args, &runtime, &cas);
   334	    let (detected, verdict) = match verdict_res {
   335	        Ok(v) => (v.verdict == "BLOCK", Some(v)),
   336	        Err(e) => (true, {
   337	            // Audit refused to load the tape at all; that itself counts
   338	            // as detection (the binary can't proceed past corruption).
   339	            // Emit a synthetic verdict for traceability.
   340	            eprintln!("audit_tape_tamper: load itself failed for `{label}` → counted as detected ({e})");
   341	            None
   342	        }),
   343	    };
   344	    TamperReport {
   345	        schema_version: "v1/audit_tape_tamper".into(),
   346	        label: label.into(),
   347	        detected,
   348	        detail,
   349	        verdict,
   350	    }
   351	}
   352	
   353	fn main() -> ExitCode {
   354	    let argv: Vec<String> = std::env::args().skip(1).collect();
   355	    let args = match parse_args(&argv) {
   356	        Ok(a) => a,
   357	        Err(e) => {
   358	            eprintln!("audit_tape_tamper: {e}\n\n{}", help_text());
   359	            return ExitCode::from(2);
   360	        }
   361	    };
   362	    if let Err(e) = std::fs::create_dir_all(&args.tamper_dir) {
   363	        eprintln!("audit_tape_tamper: mkdir tamper-dir: {e}");
   364	        return ExitCode::from(2);
   365	    }
   366	
   367	    let r1 = run_tamper("flip_l4_byte", &args, |runtime, _cas| {
   368	        flip_byte_in_first_blob(runtime)
   369	    });
   370	    let r2 = run_tamper("flip_cas_byte", &args, |_runtime, cas| {
   371	        flip_byte_in_first_cas_object(cas)
   372	    });
   373	    let r3 = run_tamper("truncate_l4_ref", &args, |runtime, _cas| {
   374	        corrupt_l4_truncate_ref(runtime)
   375	    });
   376	    let detected = [r1.detected, r2.detected, r3.detected];
   377	    let total_detected = detected.iter().filter(|x| **x).count();
   378	
   379	    let report = serde_json::json!({
   380	        "schema_version": "v1/audit_tape_tamper",
   381	        "tamper_results": [r1, r2, r3],
   382	        "detected_count": total_detected,
   383	        "expected": 3,
   384	        "all_detected": total_detected == 3,
   385	    });
   386	    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
   387	    if let Err(e) = std::fs::write(&args.out, json) {
   388	        eprintln!("audit_tape_tamper: write {:?} failed: {e}", args.out);
   389	        return ExitCode::from(2);
   390	    }
   391	
   392	    println!(
   393	        "audit_tape_tamper: detected {}/3 (out={:?})",
   394	        total_detected, args.out
   395	    );
   396	    if total_detected == 3 {
   397	        ExitCode::from(0)
   398	    } else {
   399	        ExitCode::from(1)
   400	    }
   401	}

exec
/bin/bash -lc "rg -n \"AgentVisibleProjection|tape_view_t|canonical_encode\\(&q\\.tape_view_t|serde_json::to_string|serialize|view|mask_set|project\" src/runtime src/state src/bin experiments/minif2f_v4/src -g '*.rs'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/lean4_oracle.rs:150:                let err_preview: String = combined.lines()
experiments/minif2f_v4/src/lean4_oracle.rs:155:                let detail = if err_preview.is_empty() { combined.chars().take(800).collect::<String>() } else { err_preview };
experiments/minif2f_v4/src/lean4_oracle.rs:230:                let err_preview: String = combined.lines()
experiments/minif2f_v4/src/lean4_oracle.rs:235:                let detail = if err_preview.is_empty() {
experiments/minif2f_v4/src/lean4_oracle.rs:238:                    err_preview
experiments/minif2f_v4/src/lean4_oracle.rs:311:    // Also add the project's own build output
experiments/minif2f_v4/src/lean4_oracle.rs:312:    let project_lib = PathBuf::from(minif2f_dir).join(".lake/build/lib/lean");
experiments/minif2f_v4/src/lean4_oracle.rs:313:    if project_lib.is_dir() {
experiments/minif2f_v4/src/lean4_oracle.rs:314:        paths.push(project_lib.display().to_string());
src/bin/gen_run_summary.rs:45:    let json = match serde_json::to_string_pretty(&summary) {
src/bin/gen_run_summary.rs:48:            eprintln!("gen_run_summary: serialize failed: {e}");
src/bin/verify_chaintape.rs:48:    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|e| {
src/bin/verify_chaintape.rs:49:        eprintln!("verify_chaintape: serialize report failed: {e}");
src/bin/audit_tape_tamper.rs:386:    let json = serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".into());
src/bin/audit_dashboard.rs:136:    /// derived price-index view per `compute_price_index` over a synthetic
src/bin/audit_dashboard.rs:307:    /// TB-7.7 D6: payload preview from CAS (first 80 bytes of proposal_artifact_cid content).
src/bin/audit_dashboard.rs:308:    proposal_artifact_preview: Option<String>,
src/bin/audit_dashboard.rs:328:    payload_preview: String,
src/bin/audit_dashboard.rs:362:        match serde_json::to_string_pretty(&report) {
src/bin/audit_dashboard.rs:365:                eprintln!("audit_dashboard: serialize failed: {e}");
src/bin/audit_dashboard.rs:439:        (String, String, String), // (agent_id, candidate_tactic, payload_preview)
src/bin/audit_dashboard.rs:461:                let mut payload_preview: Option<String> = None;
src/bin/audit_dashboard.rs:468:                        // TB-7.7 D6: payload preview from CAS via proposal_artifact_cid.
src/bin/audit_dashboard.rs:470:                            let preview = String::from_utf8_lossy(&payload)
src/bin/audit_dashboard.rs:474:                            payload_preview = Some(preview);
src/bin/audit_dashboard.rs:486:                                            payload_preview.clone().unwrap_or_default(),
src/bin/audit_dashboard.rs:511:                    proposal_artifact_preview: payload_preview,
src/bin/audit_dashboard.rs:565:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:581:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:644:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:673:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:697:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:729:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:751:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:773:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:787:                    proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:827:                    p.proposal_artifact_preview.clone().unwrap_or_default(),
src/bin/audit_dashboard.rs:843:                payload_preview: pl,
src/bin/audit_dashboard.rs:890:            proposal_artifact_preview: None,
src/bin/audit_dashboard.rs:1100:    let cas_view = AuditCasRef(cas);
src/bin/audit_dashboard.rs:1104:        &cas_view,
src/bin/audit_dashboard.rs:1161:/// the long/short aggregation here, the dashboard's price view is canonically
src/bin/audit_dashboard.rs:1162:/// identical to the bus snapshot's price view (architect §5.1 "no second
src/bin/audit_dashboard.rs:1348:            // TB-7.7 D6: payload preview from CAS (per-Work entries that have it).
src/bin/audit_dashboard.rs:1349:            if let Some(prev) = entry.proposal_artifact_preview.as_deref() {
src/bin/audit_dashboard.rs:1413:            if !step.payload_preview.is_empty() {
src/bin/audit_dashboard.rs:1414:                let one_line = step.payload_preview.replace('\n', " ⏎ ");
src/bin/audit_dashboard.rs:1758:    s.push_str("    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +\n");
src/bin/audit_dashboard.rs:1826:/// §14 PriceIndex render. Pure function over the derived view; extracted for
src/bin/audit_dashboard.rs:1848:    s.push_str("    absolute bounds; the price view is for relative-effectiveness\n");
src/bin/audit_dashboard.rs:1890:    s.push_str("    Price is signal, not truth. NodeMarketEntry is a derived view —\n");
src/bin/audit_dashboard.rs:1973:    s.push_str("    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.\n");
src/bin/audit_dashboard.rs:2029:    /// architect §5.1 ("Price is signal, not truth.") at the read-view
src/bin/audit_dashboard.rs:2211:    /// SG-12.6 (architect §9.3 exact name): dashboard view-positions /
src/bin/audit_dashboard.rs:2223:    fn sg_12_6_dashboard_view_positions_works() {
src/bin/audit_tape.rs:180:    let json = match serde_json::to_string_pretty(&verdict) {
src/bin/audit_tape.rs:183:            eprintln!("audit_tape: json serialize failed: {e}");
experiments/minif2f_v4/src/bin/lean_market.rs:19://! The `view-*` subcommands open the post-run chaintape READ-ONLY via
experiments/minif2f_v4/src/bin/lean_market.rs:22://! is bootstrapped during view operations (Sequencer fail-closes on
experiments/minif2f_v4/src/bin/lean_market.rs:52:        "view-task" => cmd_view_task(&sub_args),
experiments/minif2f_v4/src/bin/lean_market.rs:53:        "view-wallet" => cmd_view_wallet(&sub_args),
experiments/minif2f_v4/src/bin/lean_market.rs:54:        "view-replay" => cmd_view_replay(&sub_args),
experiments/minif2f_v4/src/bin/lean_market.rs:57:        "view-bankruptcy" => cmd_view_bankruptcy(&sub_args),
experiments/minif2f_v4/src/bin/lean_market.rs:59:        "view-positions" => cmd_view_positions(&sub_args),
experiments/minif2f_v4/src/bin/lean_market.rs:83:  lean_market view-task   --chaintape <path>
experiments/minif2f_v4/src/bin/lean_market.rs:88:  lean_market view-wallet --chaintape <path> [--agent <id>]
experiments/minif2f_v4/src/bin/lean_market.rs:92:  lean_market view-replay --chaintape <path>
experiments/minif2f_v4/src/bin/lean_market.rs:284:// view-task: read-only chain replay + print task_markets_t + claims_t status.
experiments/minif2f_v4/src/bin/lean_market.rs:287:fn cmd_view_task(args: &[String]) {
experiments/minif2f_v4/src/bin/lean_market.rs:289:        eprintln!("lean_market view-task: --chaintape <path> is required");
experiments/minif2f_v4/src/bin/lean_market.rs:296:            println!("[lean_market] view-task");
experiments/minif2f_v4/src/bin/lean_market.rs:302:            eprintln!("[lean_market] view-task replay failed: {e}");
experiments/minif2f_v4/src/bin/lean_market.rs:309:// view-wallet: read-only chain replay + print balances_t for one agent.
experiments/minif2f_v4/src/bin/lean_market.rs:312:fn cmd_view_wallet(args: &[String]) {
experiments/minif2f_v4/src/bin/lean_market.rs:314:        eprintln!("lean_market view-wallet: --chaintape <path> is required");
experiments/minif2f_v4/src/bin/lean_market.rs:322:            println!("[lean_market] view-wallet");
experiments/minif2f_v4/src/bin/lean_market.rs:351:            eprintln!("[lean_market] view-wallet replay failed: {e}");
experiments/minif2f_v4/src/bin/lean_market.rs:358:// view-replay: delegates to verify::verify_chaintape (7-indicator report).
experiments/minif2f_v4/src/bin/lean_market.rs:372:/// **policy preview mode**: replays QState read-only, computes which tasks
experiments/minif2f_v4/src/bin/lean_market.rs:455:/// `lean_market view-positions` — TB-12 Atom 4 (architect 2026-05-03 §8 Atom 4).
experiments/minif2f_v4/src/bin/lean_market.rs:463:fn cmd_view_positions(args: &[String]) {
experiments/minif2f_v4/src/bin/lean_market.rs:465:        eprintln!("lean_market view-positions: --chaintape <path> is required");
experiments/minif2f_v4/src/bin/lean_market.rs:474:            println!("[lean_market] view-positions  (TB-12 Exposure records — NOT live market balances)");
experiments/minif2f_v4/src/bin/lean_market.rs:539:            eprintln!("[lean_market] view-positions replay failed: {e}");
experiments/minif2f_v4/src/bin/lean_market.rs:545:/// `lean_market view-bankruptcy` — read-only listing of TaskMarketState::Bankrupt
experiments/minif2f_v4/src/bin/lean_market.rs:547:fn cmd_view_bankruptcy(args: &[String]) {
experiments/minif2f_v4/src/bin/lean_market.rs:549:        eprintln!("lean_market view-bankruptcy: --chaintape <path> is required");
experiments/minif2f_v4/src/bin/lean_market.rs:556:            println!("[lean_market] view-bankruptcy");
experiments/minif2f_v4/src/bin/lean_market.rs:582:            eprintln!("[lean_market] view-bankruptcy replay failed: {e}");
experiments/minif2f_v4/src/bin/lean_market.rs:588:fn cmd_view_replay(args: &[String]) {
experiments/minif2f_v4/src/bin/lean_market.rs:590:        eprintln!("lean_market view-replay: --chaintape <path> is required");
experiments/minif2f_v4/src/bin/lean_market.rs:601:            println!("[lean_market] view-replay");
experiments/minif2f_v4/src/bin/lean_market.rs:622:            eprintln!("[lean_market] view-replay failed: {e}");
experiments/minif2f_v4/src/bin/evaluator.rs:60:const DEFAULT_MINIF2F_DIR: &str = "/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4";
experiments/minif2f_v4/src/bin/evaluator.rs:166:    // All Optional; serialize-skip when None (backward compat with v3.1/v3.2 artifacts).
experiments/minif2f_v4/src/bin/evaluator.rs:264:    // Control binary (main branch) has no such set_var → classifier_version serializes as None.
experiments/minif2f_v4/src/bin/evaluator.rs:310:    // FC-trace: FC1-N7 (δ/AI canonical identity) + memory project_deepseek_drift_2026-04-24.
experiments/minif2f_v4/src/bin/evaluator.rs:400:    let json = serde_json::to_string(&result).unwrap();
experiments/minif2f_v4/src/bin/evaluator.rs:568:                    let preview: String = response.content.chars().take(500).collect();
experiments/minif2f_v4/src/bin/evaluator.rs:569:                    info!(">>> OMEGA ACCEPTED <<< (path=alone, payload[0..500]={:?})", preview);
experiments/minif2f_v4/src/bin/evaluator.rs:1106:    // WalletTool is now a read-only projection over EconomicState; canonical
experiments/minif2f_v4/src/bin/evaluator.rs:1171:    // boundary: `boltzmann_select_parent_v2(price_index, mask_set, &policy,
experiments/minif2f_v4/src/bin/evaluator.rs:1380:                // For the EMERGENT_ROLES message-board view, fall back to "n/a"
experiments/minif2f_v4/src/bin/evaluator.rs:1381:                // until balance projection is plumbed through with an EconomicState
experiments/minif2f_v4/src/bin/evaluator.rs:1430:        // projection to problem-statement-only (the same shape used
experiments/minif2f_v4/src/bin/evaluator.rs:1537:        // when wired (chaintape mode). The TB-9 collapse "balance projection
experiments/minif2f_v4/src/bin/evaluator.rs:1621:                                    &snap.price_index, &snap.mask_set,
experiments/minif2f_v4/src/bin/evaluator.rs:1639:                                // BELOW is shadow_only (kernel.tape view sync for the next
experiments/minif2f_v4/src/bin/evaluator.rs:1758:                                // shadow_only: kernel.tape view sync for next-agent prompt
experiments/minif2f_v4/src/bin/evaluator.rs:1890:                                        let preview: String = full_proof.chars().take(500).collect();
experiments/minif2f_v4/src/bin/evaluator.rs:1892:                                              path_choice, preview);
experiments/minif2f_v4/src/bin/evaluator.rs:2116:                                        // shadow_only: kernel.tape view sync for halt-and-settle +
experiments/minif2f_v4/src/bin/evaluator.rs:2209:                                        let preview: String = payload.chars().take(300).collect();
experiments/minif2f_v4/src/bin/evaluator.rs:2210:                                        warn!("[tx {}] OMEGA rejected ({}). payload[0..300]={:?}", tx, class.label(), preview);
experiments/minif2f_v4/src/bin/evaluator.rs:2543:                                        // shadow_only: kernel.tape view sync; L4 chain above is
experiments/minif2f_v4/src/bin/evaluator.rs:2636:                                        let preview = reason.chars().take(200).collect::<String>();
experiments/minif2f_v4/src/bin/evaluator.rs:2637:                                        warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
experiments/minif2f_v4/src/bin/evaluator.rs:3146:    // vars must serialize to survive cargo's parallel runner.
experiments/minif2f_v4/src/bin/evaluator.rs:3176:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:3283:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:3341:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/jsonl_schema.rs:16:use serde::{Deserialize, Serialize};
experiments/minif2f_v4/src/jsonl_schema.rs:24:#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
experiments/minif2f_v4/src/jsonl_schema.rs:81:#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
experiments/minif2f_v4/src/jsonl_schema.rs:199:#[derive(Debug, Clone, Deserialize, Serialize)]
experiments/minif2f_v4/src/jsonl_schema.rs:277:        let line = serde_json::to_string(&original).expect("serialize");
experiments/minif2f_v4/src/jsonl_schema.rs:278:        let parsed: RunAggregate = serde_json::from_str(&line).expect("deserialize");
experiments/minif2f_v4/src/jsonl_schema.rs:281:                "serialized line must stamp schema_version");
experiments/minif2f_v4/src/jsonl_schema.rs:326:        let v2_line = serde_json::to_string(&sample_run()).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:344:        let line = serde_json::to_string(&r).unwrap();
experiments/minif2f_v4/src/jsonl_schema.rs:370:        // both serialize/deserialize cleanly, including the
experiments/minif2f_v4/src/jsonl_schema.rs:375:        let line = serde_json::to_string(&r).unwrap();
experiments/minif2f_v4/src/h_vppu_history.rs:33:use serde::{Deserialize, Serialize};
experiments/minif2f_v4/src/h_vppu_history.rs:43:#[derive(Debug, Clone, Default, Serialize, Deserialize)]
experiments/minif2f_v4/src/h_vppu_history.rs:80:        let serialized = serde_json::to_string_pretty(self).map_err(io::Error::other)?;
experiments/minif2f_v4/src/h_vppu_history.rs:87:        fs::write(&tmp_path, serialized)?;
experiments/minif2f_v4/src/experiment_mode.rs:33://     agent-facing proof chain projection (L_t) is suppressed
experiments/minif2f_v4/src/rollback_sim.rs:4:// 2026-04-25 dual-audit re-review): the `--simulate-rollback-at-tx-50`
src/state/q_state.rs:21:use serde::{Deserialize, Serialize};
src/state/q_state.rs:30:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/q_state.rs:50:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:66:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:70:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:83:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:88:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/q_state.rs:97:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:105:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:113:// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
src/state/q_state.rs:116:/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
src/state/q_state.rs:119:/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
src/state/q_state.rs:121:/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
src/state/q_state.rs:123:/// in the agent read-view because a child node dominates them by
src/state/q_state.rs:124:/// `BoltzmannMaskPolicy.price_margin` (FR-14.5 / FR-14.6). **Read-view
src/state/q_state.rs:127:/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
src/state/q_state.rs:128:/// for backward-compat with pre-TB-14 chain snapshots (deserialize as
src/state/q_state.rs:130:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:131:pub struct AgentVisibleProjection {
src/state/q_state.rs:132:    pub views: BTreeMap<AgentId, NodeId>,
src/state/q_state.rs:134:    pub mask_set: BTreeSet<TxId>,
src/state/q_state.rs:144:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:169:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:180:    // The TB-14 derived view is `compute_price_index(econ)` in
src/state/q_state.rs:196:    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
src/state/q_state.rs:249:    /// **NOT projected to `AgentVisibleProjection`** (CR-15.1 + halt-
src/state/q_state.rs:252:    /// Agents cannot retrieve the bytes through their `tape_view_t`
src/state/q_state.rs:261:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:265:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:275:/// serde-default — pre-TB-3 serialized rows deserialize with the empty TaskId.
src/state/q_state.rs:276:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:293:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:303:/// serialized rows deserialize with the empty TaskId.
src/state/q_state.rs:304:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:321:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:329:/// wrote a ClaimEntry — claims_t was a never-written stub) deserialize
src/state/q_state.rs:336:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:395:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/q_state.rs:411:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:420:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:441:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:459:    /// (backward-compat: pre-TB-11 task_markets_t entries deserialize as
src/state/q_state.rs:475:    /// Backward-compat: pre-TB-11 entries deserialize at 0; the deadline
src/state/q_state.rs:517:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/q_state.rs:540:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:547:// that's TB-14 derived view. Avoids second source-of-truth risk.
src/state/q_state.rs:555:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:580:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:597:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:608:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:617:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:639:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:643:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:652:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:673:/// Additive serde-default — pre-TB-5 serialized rows deserialize with
src/state/q_state.rs:675:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/q_state.rs:703:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/q_state.rs:730:// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
src/state/q_state.rs:741:/// Sequencer-side index ONLY. NOT projected to `AgentVisibleProjection`
src/state/q_state.rs:743:/// through their `tape_view_t` (SG-15.2 + halt-trigger #4).
src/state/q_state.rs:747:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:758:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/q_state.rs:766:    /// Agent-visible projection of tape filtered by per-agent visibility policy.
src/state/q_state.rs:767:    pub tape_view_t: AgentVisibleProjection,
src/state/q_state.rs:824:            "tape_view_t",
src/state/q_state.rs:840:        // record state; NOT NodeMarketEntry which is TB-14 derived view).
src/state/q_state.rs:848:        // `compute_price_index` pure-fn derived view, not canonical state —
src/state/q_state.rs:854:        // NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1).
src/state/q_state.rs:873:    /// serializes to empty BTreeMap; carries no balance information.
src/state/q_state.rs:881:    /// on EconomicState. NodeMarketEntry is TB-14 derived view per architect
src/state/q_state.rs:891:             EconomicState field. NodeMarketEntry is TB-14 derived view only. \
src/state/q_state.rs:907:        let sa = serde_json::to_string(&a).unwrap();
src/state/q_state.rs:908:        let sb = serde_json::to_string(&b).unwrap();
src/state/mod.rs:19:/// derived-view price index. `compute_price_index(econ)` is the pure-fn
src/state/mod.rs:20:/// view of long / short interest + share depth per node (architect §5.2);
src/state/mod.rs:25:    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
src/state/mod.rs:32:/// TB-14 Atom 2 + Atom 3: derived-view price + mask types. Atom 4 adds
src/state/mod.rs:36:    compute_mask_set, compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph,
src/state/sequencer.rs:413:/// TRACE_MATRIX TB-5 charter v2 § 4.5 + preflight § 4.5: exhaustively project
src/state/sequencer.rs:1369:            // Cids are NOT projected to AgentVisibleProjection.
src/state/sequencer.rs:3214:    /// source of truth; this is a derived view.
src/state/price_index.rs:1://! TB-14 Atom 2 — PriceIndex v0 derived view.
src/state/price_index.rs:8://! the derived view is read-only broadcast input to the scheduler mask
src/state/price_index.rs:20:use serde::{Deserialize, Serialize};
src/state/price_index.rs:33:/// shadow `kernel.tape` consumption in `compute_mask_set` exposed by Codex
src/state/price_index.rs:38:/// children are both canonical TxIds, so `compute_mask_set` operates in
src/state/price_index.rs:53:/// mode), the graph is empty `BTreeMap::new()`. `compute_mask_set` over
src/state/price_index.rs:68:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/price_index.rs:76:    /// for FC2-N28 `compute_mask_set` in Atom 3): cross-multiplication
src/state/price_index.rs:80:    /// to avoid division. Used by Atom 3's `compute_mask_set` to enforce
src/state/price_index.rs:116:/// signal entry. **Derived view** populated by `compute_price_index`;
src/state/price_index.rs:128:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/price_index.rs:263:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/price_index.rs:307:    /// pass it as an explicit input to `compute_mask_set` /
src/state/price_index.rs:423:// compute_mask_set — derive the parent-mask set from price_index +
src/state/price_index.rs:431:/// `mask_set: BTreeSet<TxId>` of parent-attempt-nodes whose visibility
src/state/price_index.rs:432:/// is suppressed in the agent read-view because they are dominated by a
src/state/price_index.rs:435:/// **Read-view mask, not deletion** (CR-14.3 + SG-14.3 + halt-trigger #3):
src/state/price_index.rs:437:/// nominates parent IDs for filtering at the scheduler / read-view
src/state/price_index.rs:463:/// read-view) lived in a different id namespace and produced empty mask_set
src/state/price_index.rs:466:pub fn compute_mask_set(
src/state/price_index.rs:859:    // Tests mutate process-global env vars; serialize with a static Mutex
src/state/price_index.rs:972:        let json = serde_json::to_string(&p).unwrap();
src/state/price_index.rs:1143:        let json = serde_json::to_string(&p).unwrap();
src/bin/generate_markov_capsule.rs:382:    let json_body = serde_json::to_string_pretty(&capsule)
src/state/typed_tx.rs:15://! per the project's per-atom audit principle (CLAUDE.md "Audit Standard").
src/state/typed_tx.rs:19:use serde::{Deserialize, Serialize};
src/state/typed_tx.rs:38:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:50:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:64:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:68:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:74:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:78:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:89:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:113:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:124:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:133:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:149:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:164:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:176:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:185:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:196:/// from STATE spec): never serialized into a TypedTx variant's wire bytes.
src/state/typed_tx.rs:199:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:218:/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
src/state/typed_tx.rs:222:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:245:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:269:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:283:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:312:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:344:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:397:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:452:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:475:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:499:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:525:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:542:    /// TRACE_MATRIX Art.IV halt_reason taxonomy: project `ExhaustionReason`
src/state/typed_tx.rs:559:/// public_summary may be surfaced to dashboard / read view, raw
src/state/typed_tx.rs:565:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:568:    /// Default — only `public_summary` field surfaces to non-audit views;
src/state/typed_tx.rs:600://   No NodeMarketEntry as canonical EconomicState field (TB-14 derived view).
src/state/typed_tx.rs:611:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:630:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:679:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:712:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:735:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:767:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:784:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:854:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:875:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:893:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:911:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:930:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:947:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:966:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:986:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1011:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1036:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1079:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1085:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/state/typed_tx.rs:1106:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1115:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1128:    /// (Atom 2) to project `MicroCoin::micro_units() as u128` into the
src/state/typed_tx.rs:1147:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1178:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1209:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1224:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1247:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1269:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1384:    /// TRACE_MATRIX FC1-Sig + FC3-Sig: project the wire struct to the
src/state/typed_tx.rs:1430:    /// TRACE_MATRIX TB-5 charter v2 § 4.5: tx → signing payload projection
src/state/typed_tx.rs:1446:// TB-13 — projection impls.
src/state/typed_tx.rs:1450:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1465:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1481:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1506:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:1681:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:1802:    /// reject earlier at deserialize time) and from `PolicyViolation`
src/state/typed_tx.rs:1989:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/state/typed_tx.rs:1999:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/state/typed_tx.rs:2304:    fn typed_tx_kind_projection() {
src/state/typed_tx.rs:2625:    /// TB-11 U6: ExhaustionReason → RunOutcome projection covers all 5 variants.
src/state/typed_tx.rs:2715:    /// future codec / domain / projection change diffs one of these hex strings.
src/state/typed_tx.rs:3081:    // Computed first run; rotation rule: any future codec / domain / projection
src/state/typed_tx.rs:3245:    /// TB-13 U8: HasSubmitter projects to the wire owner / provider.
src/runtime/genesis_report.rs:24:use serde::{Deserialize, Serialize};
src/runtime/genesis_report.rs:34:#[derive(Debug, Clone, Serialize, Deserialize)]
src/runtime/genesis_report.rs:88:        let json = serde_json::to_string_pretty(self).map_err(|e| {
src/runtime/genesis_report.rs:91:                format!("genesis_report serialize: {e}"),
src/runtime/adapter.rs:537:    // (avoid holding a snapshot view across the await boundary).
src/runtime/mod.rs:285:#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
src/runtime/mod.rs:294:#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
src/runtime/mod.rs:324:    let json = serde_json::to_string_pretty(&manifest)
src/runtime/mod.rs:439:    let initial_q_json = serde_json::to_string_pretty(&initial_q)
src/runtime/mod.rs:440:        .map_err(|e| BootstrapError::Cas(format!("initial_q serialize: {e}")))?;
src/runtime/markov_capsule.rs:19:use serde::{Deserialize, Serialize};
src/runtime/markov_capsule.rs:29:/// file under `handover/alignment/` (the project's de-facto observation
src/runtime/markov_capsule.rs:31:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
src/runtime/markov_capsule.rs:52:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/markov_capsule.rs:320:    // returned to the caller is the ergonomic view; on-CAS bytes have
src/runtime/markov_capsule.rs:332:/// `Cid::from_content(&bytes)`, returning the ergonomic in-memory view
src/runtime/proposal_telemetry.rs:47:use serde::{Deserialize, Serialize};
src/runtime/proposal_telemetry.rs:63:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
src/runtime/proposal_telemetry.rs:89:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/runtime/proposal_telemetry.rs:126:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/proposal_telemetry.rs:428:        let json = serde_json::to_value(&record).expect("serialize");
src/runtime/evidence_capsule.rs:9://! to non-audit views per architect §6.1 屏蔽规则.
src/runtime/evidence_capsule.rs:19:use serde::{Deserialize, Serialize};
src/runtime/evidence_capsule.rs:39:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/evidence_capsule.rs:116:#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/evidence_capsule.rs:207:/// elsewhere (dashboard, agent read view).
src/runtime/audit_assertions.rs:32:use serde::{Deserialize, Serialize};
src/runtime/audit_assertions.rs:77:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/audit_assertions.rs:90:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/audit_assertions.rs:99:#[derive(Debug, Clone, Serialize, Deserialize)]
src/runtime/audit_assertions.rs:148:#[derive(Debug, Serialize, Deserialize, Clone)]
src/runtime/audit_assertions.rs:159:#[derive(Debug, Serialize, Deserialize, Clone, Default)]
src/runtime/audit_assertions.rs:229:#[derive(Debug, Serialize, Deserialize, Clone)]
src/runtime/audit_assertions.rs:384:    let cas_view = CasStoreRef(&cas);
src/runtime/audit_assertions.rs:388:        &cas_view,
src/runtime/audit_assertions.rs:472:    // hash-bearing line. Genesis schema is project-specific; accept either
src/runtime/audit_assertions.rs:967:    let cas_view = CasStoreRef(&t.cas);
src/runtime/audit_assertions.rs:971:        &cas_view,
src/runtime/audit_assertions.rs:989:        &cas_view,
src/runtime/audit_assertions.rs:1372:pub fn assert_26_price_index_is_view_only(_t: &LoadedTape) -> AssertionResult {
src/runtime/audit_assertions.rs:1378:    AssertionResult::pass(26, "price_index_is_view_only", AssertionLayer::E)
src/runtime/audit_assertions.rs:1441:pub fn assert_28_projection_no_autopsy_bytes(t: &LoadedTape) -> AssertionResult {
src/runtime/audit_assertions.rs:1448:                "projection_no_autopsy_bytes",
src/runtime/audit_assertions.rs:1454:    let proj_bytes = canonical_encode(&q.tape_view_t).unwrap_or_default();
src/runtime/audit_assertions.rs:1456:    // none appear in projection serialization.
src/runtime/audit_assertions.rs:1478:                    "projection_no_autopsy_bytes",
src/runtime/audit_assertions.rs:1480:                    "AgentVisibleProjection serialization contains a private_detail_cid byte run"
src/runtime/audit_assertions.rs:1486:    AssertionResult::pass(28, "projection_no_autopsy_bytes", AssertionLayer::F)
src/runtime/audit_assertions.rs:1571:    let json = serde_json::to_string(&summaries).unwrap_or_default();
src/runtime/audit_assertions.rs:1955:    r.push(assert_26_price_index_is_view_only(&tape));
src/runtime/audit_assertions.rs:1958:    r.push(assert_28_projection_no_autopsy_bytes(&tape));
src/runtime/verification_result.rs:24:use serde::{Deserialize, Serialize};
src/runtime/verification_result.rs:54:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/verification_result.rs:243:        let json = serde_json::to_value(&r).expect("serialize");
src/runtime/agent_keypairs.rs:30:use serde::{Deserialize, Serialize};
src/runtime/agent_keypairs.rs:49:#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
src/runtime/agent_keypairs.rs:325:        let serialized = serde_json::to_string_pretty(&manifest)
src/runtime/agent_keypairs.rs:334:            f.write_all(serialized.as_bytes())?;
src/runtime/agent_keypairs.rs:355:// ── Public manifest: deserialized read-side ──────────────────────────────────
src/runtime/agent_keypairs.rs:360:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/runtime/run_summary.rs:30:use serde::{Deserialize, Serialize};
src/runtime/run_summary.rs:82:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/run_summary.rs:190:        let json = serde_json::to_string_pretty(self)
src/runtime/run_summary.rs:250:        let json = serde_json::to_string(&s).unwrap();
src/runtime/chain_derived_run_facts.rs:49:use serde::{Deserialize, Serialize};
src/runtime/chain_derived_run_facts.rs:76:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/chain_derived_run_facts.rs:120:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
src/runtime/verify.rs:28://! - If `<runtime_repo>/initial_q_state.json` exists, deserialize it.
src/runtime/verify.rs:37:use serde::{Deserialize, Serialize};
src/runtime/verify.rs:116:#[derive(Debug, Clone, Serialize, Deserialize)]
src/runtime/verify.rs:165:#[derive(Debug, Clone, Serialize, Deserialize)]
src/runtime/autopsy_capsule.rs:18://!   NEVER enters `AgentVisibleProjection` (CR-15.1 + SG-15.2).
src/runtime/autopsy_capsule.rs:25://! (read-view scoping) + CR-15.3 (autopsy SUGGESTS via
src/runtime/autopsy_capsule.rs:28:use serde::{Deserialize, Serialize};
src/runtime/autopsy_capsule.rs:41:#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
src/runtime/autopsy_capsule.rs:105:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/autopsy_capsule.rs:143:    /// Audit-only access. NEVER enters `AgentVisibleProjection`.
src/runtime/autopsy_capsule.rs:418:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/autopsy_capsule.rs:1032:        let bytes = serde_json::to_vec(&summaries).expect("serialize summaries");
src/runtime/agent_audit_trail.rs:45:use serde::{Deserialize, Serialize};
src/runtime/agent_audit_trail.rs:61:#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/agent_audit_trail.rs:84:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/agent_audit_trail.rs:169:                .expect("PredicateResultsBundle serialize is infallible for stable shapes"),
src/runtime/agent_audit_trail.rs:180:                        .expect("RejectionClass serialize is infallible for stable shapes"),
src/runtime/agent_audit_trail.rs:287:#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
src/runtime/agent_audit_trail.rs:374:        let line = serde_json::to_string(&row).map_err(|e| AgentAuditError::Codec(e.to_string()))?;
src/runtime/agent_audit_trail.rs:534:        // state. The audit trail here is a synthetic projection of what the
src/runtime/agent_audit_trail.rs:680:            serde_json::to_string(&second).unwrap()

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '1,230p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! Q_t — system state vector per `STATE_TRANSITION_SPEC v1.4 § 1.1`.
     2	//!
     3	//! TRACE_MATRIX Art 0.1 — 四要素映射: `QState` provides the tape/control mapping.
     4	//! TRACE_MATRIX Art 0.4 — Q_t version-controlled: `head_t` = git commit SHA in Path B substrate.
     5	//! TRACE_MATRIX Art IV — Boot: `QState::genesis` is the starting state of every runtime.
     6	//! TRACE_MATRIX WP § 0 axiom 1 — state monotonicity: Q_t evolves only via accepted transitions.
     7	//! TRACE_MATRIX WP § 4 — 9-component system state.
     8	//! TRACE_MATRIX WP § 2 economic — `EconomicState` 9 sub-fields (CO1.2.2).
     9	//!
    10	//! **BTreeMap, not HashMap, everywhere** (Inv determinism;
    11	//! Codex flagged `kernel.rs:187-204` HashMap nondeterminism in round-2).
    12	//!
    13	//! Sub-types whose entry shapes are scoped to later atoms (CO P2.x economic engine,
    14	//! CO1.7 transition_ledger) are intentionally minimal here — full schemas land per atom,
    15	//! but the *index typing* (BTreeMap newtype shells) freezes here so Q_t is total.
    16	
    17	use std::collections::{BTreeMap, BTreeSet};
    18	
    19	use crate::bottom_white::cas::schema::Cid;
    20	
    21	use serde::{Deserialize, Serialize};
    22	
    23	use crate::economy::money::MicroCoin;
    24	
    25	// ────────────────────────────────────────────────────────────────────────────
    26	// Newtype primitives — minimal, deterministic, serde-ready.
    27	// ────────────────────────────────────────────────────────────────────────────
    28	
    29	/// TRACE_MATRIX § 1.1 — generic 32-byte hash (sha256). State / ledger / registry roots.
    30	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
    31	pub struct Hash(pub [u8; 32]);
    32	
    33	impl Hash {
    34	    /// TRACE_MATRIX § 1.1 — additive identity (genesis state-root, ledger-root, etc.).
    35	    pub const ZERO: Hash = Hash([0u8; 32]);
    36	
    37	    /// TRACE_MATRIX § 1.1 — construct from a 32-byte digest (sha256 output).
    38	    pub fn from_bytes(b: [u8; 32]) -> Self {
    39	        Hash(b)
    40	    }
    41	}
    42	
    43	impl Default for Hash {
    44	    fn default() -> Self {
    45	        Hash::ZERO
    46	    }
    47	}
    48	
    49	/// TRACE_MATRIX Art 0.4 — `head_t` = git commit SHA in Path B substrate (40 hex chars).
    50	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    51	pub struct NodeId(pub String);
    52	
    53	impl NodeId {
    54	    /// TRACE_MATRIX § 3 — pseudocode `NodeId::from_state_root(state_root)` constructor.
    55	    /// Concrete derivation (commit-tree-of-state-root) lands in CO1.7 transition_ledger.
    56	    pub fn from_state_root(state_root: Hash) -> Self {
    57	        let mut s = String::with_capacity(64);
    58	        for byte in state_root.0.iter() {
    59	            s.push_str(&format!("{:02x}", byte));
    60	        }
    61	        NodeId(s)
    62	    }
    63	}
    64	
    65	/// TRACE_MATRIX § 1.1 — agent identity (string, opaque to Q_t).
    66	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    67	pub struct AgentId(pub String);
    68	
    69	/// TRACE_MATRIX § 1.1 — accepted-transaction id (string, opaque to Q_t).
    70	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    71	pub struct TxId(pub String);
    72	
    73	/// TRACE_MATRIX WP § 19 RSP-1 — task-market entry id; opaque string.
    74	///
    75	/// **TB-3 home migration (2026-04-30)**: previously defined at
    76	/// `src/state/typed_tx.rs:33-35`. Per WP § 19 RSP-1 ("TaskMarket — 发布任务、
    77	/// 广播价格、锁定奖金") + the TB-3 charter § 4.2 schema migration, `TaskId`
    78	/// is now the canonical `TaskMarketsIndex` key — it belongs alongside
    79	/// `AgentId` / `TxId` in the Q_t identifier layer, not in the typed-tx ABI
    80	/// layer. The move closes a circular-dependency that would have arisen if
    81	/// `q_state.rs` imported `TaskId` from `typed_tx.rs` (which already imports
    82	/// `AgentId` / `TxId` from `q_state.rs`).
    83	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    84	pub struct TaskId(pub String);
    85	
    86	/// TRACE_MATRIX § 1.1 — reputation snapshot. Signed i64 to permit negative reputation
    87	/// (e.g. post-slash); ledger-of-record lives in `ReputationsIndex` (CO P2.9).
    88	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
    89	pub struct Reputation(pub i64);
    90	
    91	// ────────────────────────────────────────────────────────────────────────────
    92	// AgentSwarmState + PerAgentState — spec § 1.1 verbatim.
    93	// ────────────────────────────────────────────────────────────────────────────
    94	
    95	/// TRACE_MATRIX § 1.1 — agent swarm sub-state.
    96	/// MUST be reconstructible from L4 transition ledger replay.
    97	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
    98	pub struct AgentSwarmState {
    99	    pub agents: BTreeMap<AgentId, PerAgentState>,
   100	    pub current_round: u64,
   101	}
   102	
   103	/// TRACE_MATRIX § 1.1 — per-agent runtime state.
   104	/// `retry_counter_for_current_task` resets on accept; persists across rejections.
   105	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   106	pub struct PerAgentState {
   107	    pub reputation_snapshot: Reputation,
   108	    pub last_accepted_tx: Option<TxId>,
   109	    pub retry_counter_for_current_task: u32,
   110	}
   111	
   112	// ────────────────────────────────────────────────────────────────────────────
   113	// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
   114	// ────────────────────────────────────────────────────────────────────────────
   115	
   116	/// TRACE_MATRIX § 1.1 — agent-visible projection of tape filtered by per-agent
   117	/// visibility policy (Inv 10 Goodhart shield; `top_white::predicates::visibility`).
   118	///
   119	/// `views`: per-agent filtered head pointer; full filtering machinery lands in CO P2.7.
   120	///
   121	/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
   122	/// is the global per-round set of parent-attempt-node `TxId`s suppressed
   123	/// in the agent read-view because a child node dominates them by
   124	/// `BoltzmannMaskPolicy.price_margin` (FR-14.5 / FR-14.6). **Read-view
   125	/// mask only**, never deletion (CR-14.3 + halt-trigger #3): the underlying
   126	/// `Tape.nodes()` iteration always yields masked parents. Computed by
   127	/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
   128	/// for backward-compat with pre-TB-14 chain snapshots (deserialize as
   129	/// empty set).
   130	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   131	pub struct AgentVisibleProjection {
   132	    pub views: BTreeMap<AgentId, NodeId>,
   133	    #[serde(default)]
   134	    pub mask_set: BTreeSet<TxId>,
   135	}
   136	
   137	// ────────────────────────────────────────────────────────────────────────────
   138	// BudgetSnapshot — global compute / cost / wall-clock budget.
   139	// ────────────────────────────────────────────────────────────────────────────
   140	
   141	/// TRACE_MATRIX § 1.1 — global budget snapshot:
   142	/// cost ceiling (MicroCoin), wall clock remaining (ms), compute cap remaining.
   143	/// Exhaustion → halt_reason ∈ {WallClockCap, ComputeCapViolated, MaxTxExhausted}.
   144	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   145	pub struct BudgetSnapshot {
   146	    pub cost_ceiling_microcoin: MicroCoin,
   147	    pub wall_clock_remaining_ms: u64,
   148	    pub compute_cap_remaining: u64,
   149	}
   150	
   151	impl Default for BudgetSnapshot {
   152	    fn default() -> Self {
   153	        Self {
   154	            cost_ceiling_microcoin: MicroCoin::zero(),
   155	            wall_clock_remaining_ms: 0,
   156	            compute_cap_remaining: 0,
   157	        }
   158	    }
   159	}
   160	
   161	// ────────────────────────────────────────────────────────────────────────────
   162	// EconomicState — WP § 2 economic, 9 sub-fields. Atom CO1.2.2.
   163	// ────────────────────────────────────────────────────────────────────────────
   164	
   165	/// TRACE_MATRIX WP § 2 economic — 9-sub-field economic state. Each sub-index
   166	/// is a BTreeMap newtype; entry shapes (Escrow / Stake / Claim / TaskMarket /
   167	/// RoyaltyEdge / ChallengeCase) are minimal-but-typed here and fully fleshed
   168	/// in the owning atoms (CO P2.1-2.6).
   169	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   170	pub struct EconomicState {
   171	    pub balances_t: BalancesIndex,
   172	    pub escrows_t: EscrowsIndex,
   173	    pub stakes_t: StakesIndex,
   174	    pub claims_t: ClaimsIndex,
   175	    pub reputations_t: ReputationsIndex,
   176	    pub task_markets_t: TaskMarketsIndex,
   177	    pub royalty_graph_t: RoyaltyGraph,
   178	    pub challenge_cases_t: ChallengeCasesIndex,
   179	    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
   180	    // The TB-14 derived view is `compute_price_index(econ)` in
   181	    // `src/state/price_index.rs`; not canonical state per architect §5.1.
   182	    /// TB-11 (architect §6.2 ruling 2026-05-02): runs_t — `RunId` → run-summary
   183	    /// entry written by the TerminalSummaryTx dispatch arm. Anchors
   184	    /// architect's RunExhaustedTx semantics on chain-resident state. Each
   185	    /// failed evaluator run produces exactly one entry (idempotency on
   186	    /// run_id). `#[serde(default)]` for backward-compat with pre-TB-11
   187	    /// chain snapshots.
   188	    #[serde(default)]
   189	    pub runs_t: RunsIndex,
   190	    /// TRACE_MATRIX TB-12 (architect 2026-05-03 ruling §3 + §10): node_positions_t
   191	    /// — flat `BTreeMap<TxId, NodePosition>` index. **Canonical** TB-12 source
   192	    /// of truth for exposure records. **NOT a Coin holding** (CR-12.1 + CR-12.2);
   193	    /// NodePosition.amount is NOT counted in `monetary_invariant::total_supply_micro`.
   194	    ///
   195	    /// Architect §3 explicitly REJECTED the nested `node_market_t:
   196	    /// BTreeMap<NodeId, NodeMarketEntry>` shape — that's TB-14 derived view
   197	    /// (price + long_interest + short_interest aggregation), not canonical
   198	    /// state. Avoiding second source-of-truth (architect §3.2 reasoning;
   199	    /// TaskMarket.total_escrow precedent on cache=truth).
   200	    ///
   201	    /// Populated by accept-arm side-effect on accepted WorkTx (FirstLong) +
   202	    /// ChallengeTx (ChallengeShort) per architect §8 Atom 2. VerifyTx writes
   203	    /// nothing here per FR-12.3 + CR-12.8. `#[serde(default)]` for
   204	    /// backward-compat with pre-TB-12 chain snapshots.
   205	    #[serde(default)]
   206	    pub node_positions_t: NodePositionsIndex,
   207	    /// TRACE_MATRIX TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A
   208	    /// §4.3 + §4.4 FR-13.1..7 + CR-13.4): conditional collateral per event.
   209	    /// Locked Coin held against outstanding YES_E + NO_E share inventory.
   210	    ///
   211	    /// **IS** a Coin holding per CR-13.4 ("Locked collateral is Coin
   212	    /// holding"); included in the 6-holding `total_supply_micro` sum
   213	    /// (extends the TB-7R 5-holding sum). Mint/seed credit; redeem debit.
   214	    ///
   215	    /// **Complete-set balanced invariant** (Codex round-3 doc remediation
   216	    /// 2026-05-03): the live invariant enforced by
   217	    /// `monetary_invariant::assert_complete_set_balanced` is the
   218	    /// **MIN form**: `min(Σ_yes_shares, Σ_no_shares) == collateral`.
   219	    /// Pre-resolution (mint + seed only): both sides equal collateral
   220	    /// (MIN trivially equals collateral). Post-redeem: the winning side
   221	    /// equals collateral (debited 1:1 with collateral); the losing side
   222	    /// may exceed collateral as stranded zero-value claims. Strict
   223	    /// `Σ_yes == Σ_no == collateral` does NOT hold post-redemption.
   224	    ///
   225	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   226	    #[serde(default)]
   227	    pub conditional_collateral_t: ConditionalCollateralIndex,
   228	    /// TRACE_MATRIX TB-13 Atom 2 (architect §4.3 + CR-13.3 + SG-13.2):
   229	    /// conditional share balances per `(owner, event_id, OutcomeSide)`.
   230	    ///

exec
/bin/bash -lc "nl -ba src/runtime/adapter.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-6 Atom 2 — chaintape adapter helpers.
     2	//!
     3	//! Constructors + seeding helpers for routing Agent proposals / candidate
     4	//! proofs through the production `Sequencer` via `bus.submit_typed_tx`.
     5	//! Used by:
     6	//! - `tests/tb_6_runtime_chaintape_bootstrap.rs` T10+ (synthetic fixture proof
     7	//!   that L4 + L4.E entries appear on disk).
     8	//! - `experiments/minif2f_v4/src/bin/evaluator.rs` Atom 3 hook (when chaintape
     9	//!   mode is on, emit a `WorkTx` per evaluator decision).
    10	//!
    11	//! Per architect ruling 2026-05-01 § 3.6 Atom 2: "First version (do NOT
    12	//! rewrite evaluator at once). Adapter only: Agent proposal → WorkTx; Lean
    13	//! accept → accepted WorkTx path; Lean fail / predicate fail → rejected WorkTx
    14	//! / L4.E path. Minimum: 1 accepted + 1 rejected WorkTx."
    15	//!
    16	//! This module is `pub use`-d from `src/runtime/mod.rs` so callers reach it
    17	//! as `turingosv4::runtime::adapter::*`.
    18	
    19	use std::collections::{BTreeMap, BTreeSet};
    20	
    21	use crate::bottom_white::cas::schema::Cid;
    22	use crate::economy::money::{MicroCoin, StakeMicroCoin};
    23	use crate::runtime::agent_keypairs::{AgentKeypairError, AgentKeypairRegistry};
    24	use crate::state::q_state::{AgentId, Hash, QState, TaskId, TxId};
    25	use crate::state::typed_tx::{
    26	    AgentSignature, BoolWithProof, EscrowLockSigningPayload, EscrowLockTx, PredicateId,
    27	    PredicateResultsBundle, ReadKey, SafetyOrCreation, TaskOpenSigningPayload, TaskOpenTx, TypedTx,
    28	    VerifySigningPayload, VerifyTx, VerifyVerdict, WorkSigningPayload, WorkTx, WriteKey,
    29	};
    30	
    31	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — pre-seed initial QState with sponsor balances.
    32	///
    33	/// Mirrors `tests/tb_3_rsp1_formal_surface.rs::genesis_with_balances` in
    34	/// shape. Returns a `QState::genesis()` with `balances_t` pre-populated; callers
    35	/// pass this into `build_chaintape_sequencer_with_initial_q`.
    36	///
    37	/// **Test-fixture / Atom 3 smoke only**. Real production seeding goes through
    38	/// `on_init_tx` per WP § 14.1; this helper is the synthetic alternative.
    39	pub fn genesis_with_balances(pairs: &[(AgentId, MicroCoin)]) -> QState {
    40	    let mut q = QState::genesis();
    41	    for (agent, balance) in pairs {
    42	        q.economic_state_t
    43	            .balances_t
    44	            .0
    45	            .insert(agent.clone(), *balance);
    46	    }
    47	    q
    48	}
    49	
    50	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic TaskOpenTx constructor.
    51	pub fn make_synthetic_task_open(
    52	    task: &str,
    53	    sponsor: &str,
    54	    parent_state_root: Hash,
    55	    suffix: &str,
    56	) -> TypedTx {
    57	    TypedTx::TaskOpen(TaskOpenTx {
    58	        tx_id: TxId(format!("taskopen-{}-{}", task, suffix)),
    59	        task_id: TaskId(task.into()),
    60	        parent_state_root,
    61	        sponsor_agent: AgentId(sponsor.into()),
    62	        verifier_quorum: 1,
    63	        max_reuse_royalty_fraction_basis_points: 1000,
    64	        settlement_rule_hash: Hash::ZERO,
    65	        signature: AgentSignature::from_bytes([0u8; 64]),
    66	        timestamp_logical: 1,
    67	    })
    68	}
    69	
    70	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic EscrowLockTx constructor.
    71	pub fn make_synthetic_escrow_lock(
    72	    task: &str,
    73	    sponsor: &str,
    74	    amount_micro: i64,
    75	    parent_state_root: Hash,
    76	    suffix: &str,
    77	) -> TypedTx {
    78	    TypedTx::EscrowLock(EscrowLockTx {
    79	        tx_id: TxId(format!("escrowlock-{}-{}", task, suffix)),
    80	        task_id: TaskId(task.into()),
    81	        parent_state_root,
    82	        sponsor_agent: AgentId(sponsor.into()),
    83	        amount: MicroCoin::from_micro_units(amount_micro),
    84	        signature: AgentSignature::from_bytes([0u8; 64]),
    85	        timestamp_logical: 1,
    86	    })
    87	}
    88	
    89	/// TRACE_MATRIX FC3-N1: TB-6 Atom 2 adapter — synthetic WorkTx constructor.
    90	///
    91	/// `predicate_passes = true` exercises the accepted L4 path; `predicate_passes
    92	/// = false` triggers L4.E `PredicateFailed` (or `StakeInsufficient` if
    93	/// `stake_micro = 0`). For Atom 3 hooks, `predicate_passes` mirrors the
    94	/// evaluator's accept/reject decision per Lean check.
    95	pub fn make_synthetic_worktx(
    96	    task: &str,
    97	    agent: &str,
    98	    parent_state_root: Hash,
    99	    stake_micro: i64,
   100	    suffix: &str,
   101	    predicate_passes: bool,
   102	) -> TypedTx {
   103	    let mut acceptance = BTreeMap::new();
   104	    acceptance.insert(
   105	        PredicateId("acc1".into()),
   106	        BoolWithProof {
   107	            value: predicate_passes,
   108	            proof_cid: None,
   109	        },
   110	    );
   111	    TypedTx::Work(WorkTx {
   112	        tx_id: TxId(format!("worktx-{}-{}", task, suffix)),
   113	        task_id: TaskId(task.into()),
   114	        parent_state_root,
   115	        agent_id: AgentId(agent.into()),
   116	        read_set: [ReadKey("k.read".into())]
   117	            .into_iter()
   118	            .collect::<BTreeSet<_>>(),
   119	        write_set: [WriteKey("k.write".into())]
   120	            .into_iter()
   121	            .collect::<BTreeSet<_>>(),
   122	        proposal_cid: Default::default(),
   123	        predicate_results: PredicateResultsBundle {
   124	            acceptance,
   125	            settlement: BTreeMap::new(),
   126	            safety_class: SafetyOrCreation::Safety,
   127	        },
   128	        stake: StakeMicroCoin::from_micro_units(stake_micro),
   129	        signature: AgentSignature::from_bytes([0u8; 64]),
   130	        timestamp_logical: 1,
   131	    })
   132	}
   133	
   134	/// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — real-signature WorkTx constructor.
   135	///
   136	/// Builds a `WorkTx` and signs it via the per-run `AgentKeypairRegistry`.
   137	/// Mirrors `make_synthetic_worktx` shape but:
   138	///
   139	/// 1. Takes `proposal_cid` as a real CAS reference (the
   140	///    `ProposalTelemetry` object written by Atom 1.5 `proposal_telemetry`).
   141	/// 2. Computes `WorkSigningPayload::canonical_digest()` and signs via
   142	///    `AgentKeypairRegistry::sign(agent_id, digest)` — a real Ed25519
   143	///    signature, not a zero placeholder.
   144	/// 3. The `AgentSignature` is verifiable post-replay against the
   145	///    on-disk `agent_pubkeys.json` manifest (Atom 4 verify_chaintape
   146	///    extension; Gate 4).
   147	///
   148	/// This is the AUTHORITATIVE per-LLM-proposal WorkTx for TB-7 Frame B
   149	/// closure (charter §4.0 + §8 Gate 1). Atom 2 evaluator hook calls this
   150	/// for every meaningful real LLM proposal in the append branch.
   151	#[allow(clippy::too_many_arguments)]
   152	pub fn make_real_worktx_signed_by(
   153	    keypairs: &mut AgentKeypairRegistry,
   154	    task: &str,
   155	    agent: &str,
   156	    parent_state_root: Hash,
   157	    stake_micro: i64,
   158	    suffix: &str,
   159	    proposal_cid: Cid,
   160	    predicate_passes: bool,
   161	    timestamp_logical: u64,
   162	) -> Result<TypedTx, AgentKeypairError> {
   163	    let mut acceptance = BTreeMap::new();
   164	    acceptance.insert(
   165	        PredicateId("acc1".into()),
   166	        BoolWithProof {
   167	            value: predicate_passes,
   168	            proof_cid: None,
   169	        },
   170	    );
   171	
   172	    let agent_id = AgentId(agent.into());
   173	    let task_id = TaskId(task.into());
   174	    let tx_id = TxId(format!("worktx-{}-{}", task, suffix));
   175	    let read_set: BTreeSet<ReadKey> = [ReadKey("k.read".into())].into_iter().collect();
   176	    let write_set: BTreeSet<WriteKey> = [WriteKey("k.write".into())].into_iter().collect();
   177	    let predicate_results = PredicateResultsBundle {
   178	        acceptance,
   179	        settlement: BTreeMap::new(),
   180	        safety_class: SafetyOrCreation::Safety,
   181	    };
   182	    let stake = StakeMicroCoin::from_micro_units(stake_micro);
   183	
   184	    // Build the SigningPayload (10 fields; signature excluded per typed_tx.rs §3).
   185	    let payload = WorkSigningPayload {
   186	        tx_id: tx_id.clone(),
   187	        task_id: task_id.clone(),
   188	        parent_state_root,
   189	        agent_id: agent_id.clone(),
   190	        read_set: read_set.clone(),
   191	        write_set: write_set.clone(),
   192	        proposal_cid,
   193	        predicate_results: predicate_results.clone(),
   194	        stake,
   195	        timestamp_logical,
   196	    };
   197	    let digest = payload.canonical_digest();
   198	    let signature = keypairs.sign(&agent_id, digest)?;
   199	
   200	    Ok(TypedTx::Work(WorkTx {
   201	        tx_id,
   202	        task_id,
   203	        parent_state_root,
   204	        agent_id,
   205	        read_set,
   206	        write_set,
   207	        proposal_cid,
   208	        predicate_results,
   209	        stake,
   210	        signature,
   211	        timestamp_logical,
   212	    }))
   213	}
   214	
   215	/// TRACE_MATRIX FC1-N14: TB-7 Atom 3 — real-signature VerifyTx constructor for
   216	/// OMEGA-branch routing.
   217	///
   218	/// Builds a `VerifyTx` paired with an accepted `WorkTx` for the OMEGA path
   219	/// (Lean oracle accepted the proof → verifier confirms via VerifyTx). Signs
   220	/// via the same `AgentKeypairRegistry` as the WorkTx side. Produces a
   221	/// `VerifyVerdict::Confirm` when `verdict_confirms = true`.
   222	///
   223	/// **OMEGA scope NARROWED per ARCHITECT_RULING D3 + charter §4.3**: WorkTx
   224	/// + VerifyTx pair only; ChallengeWindow stays OPEN; NO FinalizeRewardTx,
   225	/// NO SlashTx, NO settlement. Settlement is RSP-4 / TB-9 territory.
   226	#[allow(clippy::too_many_arguments)]
   227	pub fn make_real_verifytx_signed_by(
   228	    keypairs: &mut AgentKeypairRegistry,
   229	    parent_state_root: Hash,
   230	    target_work_tx: TxId,
   231	    verifier_agent: &str,
   232	    bond_micro: i64,
   233	    suffix: &str,
   234	    verdict_confirms: bool,
   235	    timestamp_logical: u64,
   236	) -> Result<TypedTx, AgentKeypairError> {
   237	    let verifier_id = AgentId(verifier_agent.into());
   238	    let tx_id = TxId(format!("verifytx-{}-{}", verifier_agent, suffix));
   239	    let bond = StakeMicroCoin::from_micro_units(bond_micro);
   240	    let verdict = if verdict_confirms {
   241	        VerifyVerdict::Confirm
   242	    } else {
   243	        VerifyVerdict::Doubt
   244	    };
   245	
   246	    let payload = VerifySigningPayload {
   247	        tx_id: tx_id.clone(),
   248	        parent_state_root,
   249	        target_work_tx: target_work_tx.clone(),
   250	        verifier_agent: verifier_id.clone(),
   251	        bond,
   252	        verdict,
   253	        timestamp_logical,
   254	    };
   255	    let digest = payload.canonical_digest();
   256	    let signature = keypairs.sign(&verifier_id, digest)?;
   257	
   258	    Ok(TypedTx::Verify(VerifyTx {
   259	        tx_id,
   260	        parent_state_root,

exec
/bin/bash -lc 'rg -n "tape_view_t|AgentVisibleProjection|mask_set|snapshot|AgentVisible|serde_json::to_string" src/runtime/adapter.rs src/runtime/verify.rs src/state/sequencer.rs experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/bin/lean_market.rs src/bin/audit_dashboard.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:146:    model_snapshot: String,
experiments/minif2f_v4/src/bin/evaluator.rs:400:    let json = serde_json::to_string(&result).unwrap();
experiments/minif2f_v4/src/bin/evaluator.rs:904:            // Poll q_snapshot until state_root_t advances past ZERO, then
experiments/minif2f_v4/src/bin/evaluator.rs:914:                    if let Ok(q) = bundle.sequencer.q_snapshot() {
experiments/minif2f_v4/src/bin/evaluator.rs:1142:    // having to crack open the genesis_payload model_snapshot field.
experiments/minif2f_v4/src/bin/evaluator.rs:1171:    // boundary: `boltzmann_select_parent_v2(price_index, mask_set, &policy,
experiments/minif2f_v4/src/bin/evaluator.rs:1329:            // surface derived from `bus.snapshot().price_index` (integer-
experiments/minif2f_v4/src/bin/evaluator.rs:1332:            // Local snapshot — the per-iteration `snap` at line 1424 below
experiments/minif2f_v4/src/bin/evaluator.rs:1334:            let tick_snap = bus.snapshot();
experiments/minif2f_v4/src/bin/evaluator.rs:1427:        let snap = bus.snapshot();
experiments/minif2f_v4/src/bin/evaluator.rs:1538:        // through snapshot is post-MVP polish" comment at L1353-1357 is
experiments/minif2f_v4/src/bin/evaluator.rs:1540:        // sequencer.q_snapshot() → economic_state_t.balances_t. Falls back
experiments/minif2f_v4/src/bin/evaluator.rs:1547:            .and_then(|seq| seq.q_snapshot().ok())
experiments/minif2f_v4/src/bin/evaluator.rs:1621:                                    &snap.price_index, &snap.mask_set,
experiments/minif2f_v4/src/bin/evaluator.rs:1643:                                // q_snapshot / CAS open / proposal_telemetry write /
experiments/minif2f_v4/src/bin/evaluator.rs:1652:                                    let q = match bundle.sequencer.q_snapshot() {
experiments/minif2f_v4/src/bin/evaluator.rs:1655:                                            error!("[chaintape/atom2] FAIL-CLOSED: q_snapshot failed under ChainTape mode: {e:?}");
experiments/minif2f_v4/src/bin/evaluator.rs:1919:                                            let q = match bundle.sequencer.q_snapshot() {
experiments/minif2f_v4/src/bin/evaluator.rs:1922:                                                    error!("[chaintape/atom3-omega] FAIL-CLOSED: q_snapshot: {e:?}");
experiments/minif2f_v4/src/bin/evaluator.rs:2351:                                            let q = match bundle.sequencer.q_snapshot() {
experiments/minif2f_v4/src/bin/evaluator.rs:2354:                                                    error!("[chaintape/atom3-omega-pertactic] FAIL-CLOSED: q_snapshot: {e:?}");
experiments/minif2f_v4/src/bin/evaluator.rs:2942:    let model_snapshot = std::env::var("MODEL_SNAPSHOT")
experiments/minif2f_v4/src/bin/evaluator.rs:2983:        model_snapshot,
experiments/minif2f_v4/src/bin/evaluator.rs:3176:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:3283:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:3341:        let line = serde_json::to_string(&result).expect("serialize PputResult");
experiments/minif2f_v4/src/bin/evaluator.rs:3364:    /// **identical `git_sha`, `binary_sha256`, and `model_snapshot`
experiments/minif2f_v4/src/bin/evaluator.rs:3414:        // model_snapshot, split) must be identical to row 0's.
experiments/minif2f_v4/src/bin/evaluator.rs:3423:            assert_eq!(r.model_snapshot, r0.model_snapshot,
experiments/minif2f_v4/src/bin/evaluator.rs:3424:                "model_snapshot must be mode-invariant; mode '{}' differs",
experiments/minif2f_v4/src/bin/evaluator.rs:3434:        assert_eq!(r0.model_snapshot, "deepseek-v4-flash@2026-04-26",
src/state/sequencer.rs:13://! bodies. The structural correctness of the apply path (snapshot → dispatch →
src/state/sequencer.rs:1369:            // Cids are NOT projected to AgentVisibleProjection.
src/state/sequencer.rs:2934:        q_snapshot: &QState,
src/state/sequencer.rs:2982:                q_snapshot.state_root_t,
src/state/sequencer.rs:3020:        // Stage 1: snapshot Q_t under read lock.
src/state/sequencer.rs:3021:        let q_snapshot = {
src/state/sequencer.rs:3044:                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
src/state/sequencer.rs:3053:            &q_snapshot,
src/state/sequencer.rs:3060:                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
src/state/sequencer.rs:3101:                    &q_snapshot.economic_state_t,
src/state/sequencer.rs:3103:                    q_snapshot.q_t.current_round,
src/state/sequencer.rs:3125:            parent_state_root: q_snapshot.state_root_t,
src/state/sequencer.rs:3126:            parent_ledger_root: q_snapshot.ledger_root_t,
src/state/sequencer.rs:3143:        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
src/state/sequencer.rs:3186:    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
src/state/sequencer.rs:3198:    /// consumption at the bus snapshot's mask-set derivation site
src/state/sequencer.rs:3211:    /// rather than propagated — bus.snapshot must NEVER crash because
src/state/sequencer.rs:3217:    /// snapshot frequency is bounded by the evaluator iteration cap;
src/state/sequencer.rs:3460:    // q_snapshot.state_root_t carried in. Locks P1:6 contract.
src/state/sequencer.rs:3464:        let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;
src/state/sequencer.rs:3490:        let post = seq.q_snapshot().expect("q_snapshot").state_root_t;
src/runtime/adapter.rs:384:/// pre-snapshot (or budget expires). Caller passes the pre-snapshot,
src/runtime/adapter.rs:398:        if let Ok(q) = sequencer.q_snapshot() {
src/runtime/adapter.rs:415:/// VerifyTx to have been applied. We poll `q_snapshot` until the claim
src/runtime/adapter.rs:440:        if let Ok(q) = sequencer.q_snapshot() {
src/runtime/adapter.rs:531:    let q = match sequencer.q_snapshot() {
src/runtime/adapter.rs:536:    // Pre-collect candidates so we can drop the q_snapshot before emitting
src/runtime/adapter.rs:537:    // (avoid holding a snapshot view across the await boundary).
src/runtime/adapter.rs:582:        if let Ok(q_now) = sequencer.q_snapshot() {
src/bin/audit_dashboard.rs:117:    /// committed liquidity at this snapshot.
src/bin/audit_dashboard.rs:146:    /// at snapshot time. Empty Vec when no TaskBankruptcyTx has fired.
src/bin/audit_dashboard.rs:362:        match serde_json::to_string_pretty(&report) {
src/bin/audit_dashboard.rs:1083:    // Initial QState (genesis or persisted snapshot).
src/bin/audit_dashboard.rs:1162:/// identical to the bus snapshot's price view (architect §5.1 "no second
src/bin/audit_dashboard.rs:1490:    // keystore. The dashboard reflects the per-run manifest (snapshot) and
src/bin/audit_dashboard.rs:1758:    s.push_str("    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +\n");
src/bin/audit_dashboard.rs:1764:        s.push_str("  (no agent_autopsies_t entries in this snapshot — no\n");

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-6 Atom 4 — `verify_chaintape` replay verifier (library).
     2	//!
     3	//! Re-opens a runtime_repo + cas + pinned_pubkeys.json and replays the L4
     4	//! chain entry-by-entry through `replay_full_transition` (the I-DETHASH
     5	//! witness from CO1.7-impl A4). Reconstructs `QState` (including
     6	//! `EconomicState`) from L4 alone — L4.E is **evidence-only**, not state-bearing
     7	//! (Inv 7). Verifies every entry's `system_signature` against the persisted
     8	//! `pinned_pubkeys.json` manifest.
     9	//!
    10	//! Architect ruling 2026-05-01 § 3.5 deliverable: `replay_report.json` with the
    11	//! 7 mandated boolean indicators:
    12	//! - `l4_entries`
    13	//! - `l4e_entries`
    14	//! - `ledger_root_verified`
    15	//! - `system_signatures_verified`
    16	//! - `state_reconstructed`
    17	//! - `economic_state_reconstructed`
    18	//! - `cas_payloads_retrievable`
    19	//!
    20	//! Per architect § 3.6 Atom 4 + ruling D2 (1)-(7): chain-backed smoke from
    21	//! TB-6 onward must be replayable. This module is the structural witness.
    22	//!
    23	//! Driven by:
    24	//! - `src/bin/verify_chaintape.rs` — CLI thin wrapper
    25	//! - `tests/tb_6_verify_chaintape.rs` — I90 integration test
    26	//!
    27	//! Initial QState resolution:
    28	//! - If `<runtime_repo>/initial_q_state.json` exists, deserialize it.
    29	//! - Else default to `QState::genesis()` (matches Atom 3 smoke evidence).
    30	//!
    31	//! Bounded by `RejectionEvidenceWriter::open_jsonl` which validates the
    32	//! L4.E `prev_hash → hash` chain on load — tamper any byte of any line and
    33	//! the open call returns `RejectionEvidenceError::ChainBroken { at }`.
    34	
    35	use std::path::{Path, PathBuf};
    36	
    37	use serde::{Deserialize, Serialize};
    38	
    39	use crate::bottom_white::cas::store::CasStore;
    40	use crate::bottom_white::ledger::rejection_evidence::{
    41	    RejectionEvidenceError, RejectionEvidenceWriter,
    42	};
    43	use crate::bottom_white::ledger::system_keypair::{PinnedSystemPubkeys, SystemEpoch, SystemPublicKey};
    44	use crate::bottom_white::ledger::transition_ledger::{
    45	    replay_full_transition, Git2LedgerWriter, LedgerEntry, LedgerWriter, LedgerWriterError,
    46	    ReplayError,
    47	};
    48	use crate::runtime::{PinnedPubkeyManifest};
    49	use crate::state::q_state::{Hash, QState};
    50	use crate::top_white::predicates::registry::PredicateRegistry;
    51	use crate::bottom_white::tools::registry::ToolRegistry;
    52	
    53	const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
    54	const INITIAL_Q_STATE_FILENAME: &str = "initial_q_state.json";
    55	const REJECTIONS_JSONL_FILENAME: &str = "rejections.jsonl";
    56	
    57	// ── Errors ──────────────────────────────────────────────────────────────────
    58	
    59	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape error class.
    60	///
    61	/// Distinct from `ReplayError`: this covers I/O / config / manifest issues
    62	/// that prevent replay from even starting (vs. mid-chain divergence which is
    63	/// `ReplayError`-shaped).
    64	#[derive(Debug)]
    65	pub enum VerifyError {
    66	    Io(std::io::Error),
    67	    LedgerWriter(LedgerWriterError),
    68	    Cas(String),
    69	    PinnedPubkeysMissing(PathBuf),
    70	    PinnedPubkeysParse(String),
    71	    InitialQStateParse(String),
    72	    PubkeyDecode(String),
    73	    L4eOpen(RejectionEvidenceError),
    74	}
    75	
    76	impl std::fmt::Display for VerifyError {
    77	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    78	        match self {
    79	            Self::Io(e) => write!(f, "io error: {e}"),
    80	            Self::LedgerWriter(e) => write!(f, "ledger writer error: {e}"),
    81	            Self::Cas(e) => write!(f, "cas error: {e}"),
    82	            Self::PinnedPubkeysMissing(p) => {
    83	                write!(f, "pinned_pubkeys.json not found at {p:?}")
    84	            }
    85	            Self::PinnedPubkeysParse(s) => write!(f, "pinned_pubkeys.json parse failed: {s}"),
    86	            Self::InitialQStateParse(s) => write!(f, "initial_q_state.json parse failed: {s}"),
    87	            Self::PubkeyDecode(s) => write!(f, "pubkey hex decode failed: {s}"),
    88	            Self::L4eOpen(e) => write!(f, "rejections.jsonl open / chain-verify failed: {e}"),
    89	        }
    90	    }
    91	}
    92	
    93	impl std::error::Error for VerifyError {}
    94	
    95	impl From<std::io::Error> for VerifyError {
    96	    fn from(e: std::io::Error) -> Self {
    97	        Self::Io(e)
    98	    }
    99	}
   100	
   101	impl From<LedgerWriterError> for VerifyError {
   102	    fn from(e: LedgerWriterError) -> Self {
   103	        Self::LedgerWriter(e)
   104	    }
   105	}
   106	
   107	// ── Report shape (replay_report.json wire format) ───────────────────────────
   108	
   109	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json wire format.
   110	///
   111	/// Stable JSON shape consumed by the smoke evidence dir + CI gates. The 7
   112	/// architect-mandated indicators are top-level fields; richer detail
   113	/// (final state/ledger root hex, classification of any replay error) is
   114	/// captured under `detail` so downstream tooling can drill in without
   115	/// breaking the headline contract.
   116	#[derive(Debug, Clone, Serialize, Deserialize)]
   117	pub struct ReplayReport {
   118	    /// Number of L4 entries (length of `refs/transitions/main`).
   119	    pub l4_entries: u64,
   120	    /// Number of L4.E entries (length of `rejections.jsonl`).
   121	    pub l4e_entries: u64,
   122	    /// True iff every entry's `parent_ledger_root` chains to the previous
   123	    /// `resulting_ledger_root` and the `append()` fold is byte-stable.
   124	    pub ledger_root_verified: bool,
   125	    /// True iff every entry's `system_signature` verifies against the
   126	    /// persisted pinned-pubkey manifest at the entry's epoch.
   127	    pub system_signatures_verified: bool,
   128	    /// True iff replay produced a `QState` (no `dispatch_transition` or
   129	    /// state-root divergence). Empty chain (`l4_entries == 0`) → `true`.
   130	    pub state_reconstructed: bool,
   131	    /// True iff the replayed `QState.economic_state_t` is consistent with
   132	    /// the chain (i.e., replay completed without error). Currently coupled
   133	    /// to `state_reconstructed`; future work may split when economic-only
   134	    /// replay paths are added (NodeMarket, RSP-M).
   135	    pub economic_state_reconstructed: bool,
   136	    /// True iff every L4 entry's `tx_payload_cid` was retrievable from CAS.
   137	    pub cas_payloads_retrievable: bool,
   138	    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx / VerifyTx entry's
   139	    /// `AgentSignature` verifies against the per-run `agent_pubkeys.json`
   140	    /// manifest. Empty chain or chain with no Work/Verify entries → `true`
   141	    /// (no agent signatures to verify ≠ failure).
   142	    ///
   143	    /// This is the Gate 4 evidence (TB-7 charter §8): all WorkTx
   144	    /// signatures verify against agent_pubkeys.json. False on any
   145	    /// signature mismatch (tampering, key drift, unknown agent_id).
   146	    pub agent_signatures_verified: bool,
   147	    /// **TB-7 Atom 4 NEW**: True iff every L4 WorkTx entry's
   148	    /// `proposal_cid` resolves to a CAS-resident `ProposalTelemetry`
   149	    /// object. Empty chain or chain with zero Work entries → `true`.
   150	    ///
   151	    /// This is the Gate 5 evidence (TB-7 charter §8): every
   152	    /// `WorkTx.proposal_cid` resolves to a CAS `ProposalTelemetry`
   153	    /// object with the §4.5 schema. False if any WorkTx points to a
   154	    /// CID that's missing or decodes to non-ProposalTelemetry bytes.
   155	    pub proposal_telemetry_cas_retrievable: bool,
   156	    /// Run-id from `pinned_pubkeys.json` manifest (echoed for forensics).
   157	    pub run_id: String,
   158	    /// Epoch from `pinned_pubkeys.json` manifest.
   159	    pub epoch: u64,
   160	    /// Detail block — non-blocking forensic data.
   161	    pub detail: ReplayReportDetail,
   162	}
   163	
   164	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — replay_report.json detail block.
   165	#[derive(Debug, Clone, Serialize, Deserialize)]
   166	pub struct ReplayReportDetail {
   167	    pub final_state_root_hex: Option<String>,
   168	    pub final_ledger_root_hex: Option<String>,
   169	    /// Lowercase 40-char git commit OID at HEAD of `refs/transitions/main`,
   170	    /// or None if chain is empty.
   171	    pub head_commit_oid_hex: Option<String>,
   172	    /// L4.E chain hash at the end of `rejections.jsonl`, or `Hash::ZERO`
   173	    /// hex if empty.
   174	    pub l4e_last_hash_hex: String,
   175	    /// One-line classification of the replay error if replay failed.
   176	    pub replay_failure: Option<String>,
   177	    /// True iff `<runtime_repo>/initial_q_state.json` was found and loaded.
   178	    /// False when the verifier defaulted to `QState::genesis()`.
   179	    pub initial_q_state_loaded_from_disk: bool,
   180	}
   181	
   182	impl ReplayReport {
   183	    /// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — ship-gate aggregator over the 5
   184	    /// architect-mandated boolean indicators. The CLI uses this to drive its
   185	    /// exit code (0 when all pass, 1 otherwise).
   186	    ///
   187	    /// True iff every architect-mandated boolean indicator is `true`.
   188	    /// **TB-7 Atom 4**: also checks the new `agent_signatures_verified` (Gate 4)
   189	    /// and `proposal_telemetry_cas_retrievable` (Gate 5) indicators.
   190	    pub fn all_indicators_pass(&self) -> bool {
   191	        self.ledger_root_verified
   192	            && self.system_signatures_verified
   193	            && self.state_reconstructed
   194	            && self.economic_state_reconstructed
   195	            && self.cas_payloads_retrievable
   196	            && self.agent_signatures_verified
   197	            && self.proposal_telemetry_cas_retrievable
   198	    }
   199	}
   200	
   201	// ── Verifier entry-points ───────────────────────────────────────────────────
   202	
   203	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — verify_chaintape options.
   204	#[derive(Debug, Clone, Default)]
   205	pub struct VerifyOptions {
   206	    /// Optional run-id filter; if provided, the verifier asserts the
   207	    /// pinned-pubkey manifest's `run_id` matches before replay. None =
   208	    /// no filter (smoke evidence may legitimately not echo a run-id).
   209	    pub expected_run_id: Option<String>,
   210	}
   211	
   212	/// TRACE_MATRIX FC3-N1: TB-6 Atom 4 — single library entry-point for replay
   213	/// + signature + CAS + L4.E verification. The CLI binary at
   214	/// `src/bin/verify_chaintape.rs` is a thin wrapper around this.
   215	///
   216	/// Steps (mirrors architect § 3.6 Atom 4):
   217	/// 1. Read `pinned_pubkeys.json` from `runtime_repo_path`. Decode hex
   218	///    pubkey(s) into a `PinnedSystemPubkeys` map keyed by `SystemEpoch`.
   219	/// 2. Resolve initial `QState` from `<runtime_repo>/initial_q_state.json` if
   220	///    present; else `QState::genesis()`.
   221	/// 3. Open `Git2LedgerWriter` at `runtime_repo_path`. Read all entries.
   222	/// 4. Open `CasStore` at `cas_path`.
   223	/// 5. Open `RejectionEvidenceWriter::open_jsonl(<runtime_repo>/rejections.jsonl)`
   224	///    — this internally calls `verify_chain()` and rejects tampering.
   225	/// 6. Replay via `replay_full_transition` → either `Ok(QState)` or
   226	///    `ReplayError`. Map the error variant onto the mandated booleans.
   227	/// 7. Build `ReplayReport`. Return `Ok(report)` (errors only for I/O /
   228	///    manifest issues that block replay from starting).
   229	pub fn verify_chaintape(
   230	    runtime_repo_path: &Path,
   231	    cas_path: &Path,
   232	    options: &VerifyOptions,
   233	) -> Result<ReplayReport, VerifyError> {
   234	    // Step 1: pinned-pubkey manifest.
   235	    let manifest_path = runtime_repo_path.join(PINNED_PUBKEYS_FILENAME);
   236	    if !manifest_path.exists() {
   237	        return Err(VerifyError::PinnedPubkeysMissing(manifest_path));
   238	    }
   239	    let manifest_json = std::fs::read_to_string(&manifest_path)?;
   240	    let manifest: PinnedPubkeyManifest = serde_json::from_str(&manifest_json)
   241	        .map_err(|e| VerifyError::PinnedPubkeysParse(e.to_string()))?;
   242	    if let Some(expected) = options.expected_run_id.as_ref() {
   243	        if manifest.run_id != *expected {
   244	            return Err(VerifyError::PinnedPubkeysParse(format!(
   245	                "run_id mismatch: manifest={} expected={}",
   246	                manifest.run_id, expected
   247	            )));
   248	        }
   249	    }
   250	    let mut pinned = PinnedSystemPubkeys::new();
   251	    for entry in &manifest.pubkeys {
   252	        let bytes = decode_pubkey_hex(&entry.pubkey_hex)?;
   253	        let arr: [u8; 32] = bytes.as_slice().try_into().map_err(|_| {
   254	            VerifyError::PubkeyDecode(format!(
   255	                "expected 32-byte ed25519 pubkey, got {} bytes",
   256	                bytes.len()
   257	            ))
   258	        })?;
   259	        let pubkey = SystemPublicKey::from_bytes(arr);
   260	        pinned.insert(SystemEpoch::new(entry.epoch), pubkey);

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '230,310p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   230	    ///
   231	    /// **IS NOT** a Coin holding — shares are CLAIMS against
   232	    /// `conditional_collateral_t[event_id]`; CR-13.3 + SG-13.2 explicit:
   233	    /// shares are NOT counted in `total_supply_micro`. Mint mints equal
   234	    /// YES + NO; seed mints equal YES + NO to provider; redeem debits the
   235	    /// winning side at 1 share = 1 MicroCoin against collateral.
   236	    ///
   237	    /// `#[serde(default)]` for backward-compat with pre-TB-13 chain snapshots.
   238	    #[serde(default)]
   239	    pub conditional_share_balances_t: ConditionalShareBalances,
   240	    /// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 ruling 2026-05-02 + §6.5
   241	    /// SG-15.1 + SG-15.2): per-event autopsy index.
   242	    /// `BTreeMap<EventId, Vec<Cid>>` — for each event with at least one
   243	    /// loss-emission, accumulates the CAS Cids of `AgentAutopsyCapsule`
   244	    /// objects (one per losing agent). **Stores Cids only**, NEVER the
   245	    /// raw capsule bytes — the bytes live in CAS behind
   246	    /// `ObjectType::AgentAutopsyCapsule` (and the audit-only
   247	    /// `private_detail_cid` lives behind `ObjectType::AutopsyPrivateDetail`).
   248	    ///
   249	    /// **NOT projected to `AgentVisibleProjection`** (CR-15.1 + halt-
   250	    /// trigger #1). Sequencer-side index only; surfaces via
   251	    /// dashboard §15 (Atom 6) + ChainTape replay regeneration. Other
   252	    /// Agents cannot retrieve the bytes through their `tape_view_t`
   253	    /// (SG-15.2 + halt-trigger #4).
   254	    ///
   255	    /// `#[serde(default)]` for backward-compat with pre-TB-15 chain snapshots.
   256	    #[serde(default)]
   257	    pub agent_autopsies_t: AutopsyIndex,
   258	}
   259	
   260	/// TRACE_MATRIX WP § 2 — agent → balance ledger. Concrete entry: `MicroCoin` (CO1.0a).
   261	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   262	pub struct BalancesIndex(pub BTreeMap<AgentId, MicroCoin>);
   263	
   264	/// TRACE_MATRIX WP § 2 — tx → escrow entry. Full schema lands CO P2.2 EscrowVault.
   265	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   266	pub struct EscrowsIndex(pub BTreeMap<TxId, EscrowEntry>);
   267	
   268	/// TRACE_MATRIX WP § 2 — escrow entry shape (stub). Full fields land CO P2.2.
   269	/// `#[serde(default)]` on each field gives forward-compat: future atoms can add
   270	/// fields without breaking deserialization of historical ledger rows.
   271	///
   272	/// **TB-3 additive field**: `task_id` is the back-reference to the `TaskId`
   273	/// this escrow funds. Required by `assert_task_market_total_escrow_matches_locks`
   274	/// (the cache=truth invariant for `TaskMarketEntry.total_escrow`). Additive
   275	/// serde-default — pre-TB-3 serialized rows deserialize with the empty TaskId.
   276	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   277	pub struct EscrowEntry {
   278	    #[serde(default = "MicroCoin::zero")]
   279	    pub amount: MicroCoin,
   280	    #[serde(default)]
   281	    pub depositor: AgentId,
   282	    #[serde(default)]
   283	    pub task_id: TaskId,
   284	}
   285	
   286	impl Default for EscrowEntry {
   287	    fn default() -> Self {
   288	        Self { amount: MicroCoin::zero(), depositor: AgentId::default(), task_id: TaskId::default() }
   289	    }
   290	}
   291	
   292	/// TRACE_MATRIX WP § 2 — tx → stake entry. Full schema lands CO P2.5 ChallengeCourt.
   293	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   294	pub struct StakesIndex(pub BTreeMap<TxId, StakeEntry>);
   295	
   296	/// TRACE_MATRIX WP § 2 — stake entry shape (stub). Full fields land CO P2.5.
   297	///
   298	/// **TB-3 additive field**: `task_id` records the task this stake commits
   299	/// to. Required by the WorkTx admission gate (TB-3 § 3.4 lock-on-accept):
   300	/// when an accepted WorkTx commits its inline `stake` into `stakes_t`, the
   301	/// entry carries the task binding so future RSP-2/3 challenge resolution
   302	/// can route the slash/release. Additive serde-default — pre-TB-3
   303	/// serialized rows deserialize with the empty TaskId.
   304	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   305	pub struct StakeEntry {
   306	    #[serde(default = "MicroCoin::zero")]
   307	    pub amount: MicroCoin,
   308	    #[serde(default)]
   309	    pub staker: AgentId,
   310	    #[serde(default)]

exec
/bin/bash -lc 'rg -n "pub fn replay_full_transition|fn replay_full_transition|dispatch_transition|apply_one|submit_typed_tx|canonical_encode|canonical_decode|LedgerEntry" src/bottom_white/ledger/transition_ledger.rs src/state/sequencer.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/state/sequencer.rs:1://! L4 Sequencer + dispatch_transition (CO1.7-impl A2 + A3).
src/state/sequencer.rs:7://!   (dispatch_transition exhaustive enum match; K5 Slash dropped)
src/state/sequencer.rs:33:    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
src/state/sequencer.rs:65:/// `canonical_encode` is — and TB-2 wants a single short call site that
src/state/sequencer.rs:70:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:120:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:131:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:154:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:165:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:183:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:201:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:216:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:232:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:249:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:265:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:281:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:297:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:400:// TB-5 Atom 4 — apply_one Stage 1.5 helpers (preflight § 4.5)
src/state/sequencer.rs:415:/// verification at apply_one stage 1.5. Returns `None` for agent variants
src/state/sequencer.rs:513:// § 8 dispatch_transition — exhaustive enum match (K5: NO Slash)
src/state/sequencer.rs:523:pub(crate) fn dispatch_transition(
src/state/sequencer.rs:532:            // No I/O, no side effects, no writer calls — apply_one is the
src/state/sequencer.rs:1159:        // apply_one stage 1.5; reaches dispatch only via emit_system_tx.
src/state/sequencer.rs:1366:            // apply_one's post-dispatch hook (Stage 3.5) writes the bytes
src/state/sequencer.rs:1971:/// Called from `apply_one` stage 9 AFTER `writer.commit` succeeds. Pure
src/state/sequencer.rs:1979:/// **Atomicity** (CO1.7-extra round-2 MF9): in apply_one, called under the
src/state/sequencer.rs:2006:/// and the typed tx through to `apply_one`.
src/state/sequencer.rs:2201:/// Errors that can occur during `apply_one`. Spec § 3 implicitly assumes
src/state/sequencer.rs:2205:/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
src/state/sequencer.rs:2206:/// this implementation widens to `Result<LedgerEntry, ApplyError>` to preserve
src/state/sequencer.rs:2297:/// the typed `CanonicalMessage::LedgerEntrySigning([u8;32])` extension closes
src/state/sequencer.rs:2305:    /// K1: assigned at submit; never appears in LedgerEntry.
src/state/sequencer.rs:2331:    /// public-key map. Used by apply_one stage 1.5 to verify
src/state/sequencer.rs:2565:        // dispatch_transition discriminates by variant TYPE per preflight § 3.6).
src/state/sequencer.rs:2837:            // against the sequencer's current epoch (mirrors apply_one
src/state/sequencer.rs:2872:    /// callers (e.g., bus.rs:135-141 `TuringBus::submit_typed_tx`) keep
src/state/sequencer.rs:2882:    /// Driver loop. Drains the queue and runs `apply_one` on each tx. Errors
src/state/sequencer.rs:2883:    /// from individual `apply_one` calls are logged and skipped (per-tx
src/state/sequencer.rs:2891:            // Stub state: dispatch returns NotYetImplemented; apply_one
src/state/sequencer.rs:2894:            if let Err(e) = self.apply_one(envelope) {
src/state/sequencer.rs:2895:                log::debug!("sequencer apply_one rejected: {e}");
src/state/sequencer.rs:2903:    /// Drains at most one envelope from the queue and runs `apply_one` on it.
src/state/sequencer.rs:2908:    pub fn try_apply_one(
src/state/sequencer.rs:2911:    ) -> Option<Result<LedgerEntry, ApplyError>> {
src/state/sequencer.rs:2913:            Ok(envelope) => Some(self.apply_one(envelope)),
src/state/sequencer.rs:2919:    /// rejection-writer arm out of `apply_one` so it can be invoked from
src/state/sequencer.rs:2937:        let payload_bytes = canonical_encode(tx)
src/state/sequencer.rs:3011:    pub(crate) fn apply_one(
src/state/sequencer.rs:3014:    ) -> Result<LedgerEntry, ApplyError> {
src/state/sequencer.rs:3016:        // travels with the tx through to apply_one. Atom 4: submit_id is
src/state/sequencer.rs:3028:        // signs the message before queueing, apply_one re-verifies against
src/state/sequencer.rs:3052:        let (q_next, _signals) = match dispatch_transition(
src/state/sequencer.rs:3071:        let payload_bytes = canonical_encode(&tx)
src/state/sequencer.rs:3095:        // apply_one stay agreement-locked: pre-cutoff rows write nothing
src/state/sequencer.rs:3121:        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
src/state/sequencer.rs:3123:        let signing_payload = LedgerEntrySigningPayload {
src/state/sequencer.rs:3135:        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
src/state/sequencer.rs:3145:        // Stage 8: build LedgerEntry (the stored record).
src/state/sequencer.rs:3146:        let entry = LedgerEntry {
src/state/sequencer.rs:3209:    /// missing payload, canonical_decode error, ProposalTelemetry
src/state/sequencer.rs:3229:        use crate::bottom_white::ledger::transition_ledger::canonical_decode;
src/state/sequencer.rs:3260:            let typed_tx: TypedTx = match canonical_decode(&payload) {
src/state/sequencer.rs:3385:    // 1. dispatch_transition: NON-WORK / NON-RSP1 / NON-RSP2 / NON-RSP4
src/state/sequencer.rs:3399:    fn dispatch_transition_stubs_reuse_only() {
src/state/sequencer.rs:3412:        let result = dispatch_transition(&q, &tx, &preds, &tools);
src/state/sequencer.rs:3433:    // 3. apply_one rejected: returns Transition(EscrowMissing) with the default
src/state/sequencer.rs:3439:    fn apply_one_stub_does_not_consume_logical_t() {
src/state/sequencer.rs:3446:        let err = seq.apply_one(envelope).unwrap_err();
src/state/sequencer.rs:3452:        assert_eq!(pre, post, "logical_t MUST NOT advance on rejected apply_one");
src/state/sequencer.rs:3455:    // TB-2 Atom 4 — U2: apply_one rejected path keys L4.E by envelope.submit_id.
src/state/sequencer.rs:3457:    // Drives apply_one with a known submit_id and a WorkTx that fails the
src/state/sequencer.rs:3462:    fn apply_one_rejected_path_uses_envelope_submit_id() {
src/state/sequencer.rs:3469:        let err = seq.apply_one(envelope).unwrap_err();
src/state/sequencer.rs:3495:    // TB-2 Atom 2 — U1: apply_one consumes SubmissionEnvelope.
src/state/sequencer.rs:3498:    // through to apply_one. Charter §8 Proof 1 will further verify that the
src/state/sequencer.rs:3502:    fn apply_one_consumes_submission_envelope() {
src/state/sequencer.rs:3508:        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
src/state/sequencer.rs:3511:        let result = seq.apply_one(envelope);
src/state/sequencer.rs:3518:    // TB-2 Atom 2 — try_apply_one driver helper (P1-3 r2).
src/state/sequencer.rs:3524:    async fn try_apply_one_drains_one_envelope() {
src/state/sequencer.rs:3528:        assert!(seq.try_apply_one(&mut rx).is_none());
src/state/sequencer.rs:3530:        // Submit one tx through the public path; try_apply_one should drain it.
src/state/sequencer.rs:3535:        let drained = seq.try_apply_one(&mut rx).expect("envelope was queued");
src/state/sequencer.rs:3536:        // Default fixture lacks seeded escrow so apply_one rejects with
src/state/sequencer.rs:3538:        // from queue and apply_one ran".
src/state/sequencer.rs:3549:        assert!(seq.try_apply_one(&mut rx).is_none());
src/state/sequencer.rs:3552:    // TB-2 Atom 3 — U3: dispatch_transition WorkTx returns the interim
src/state/sequencer.rs:3555:    // Drives dispatch_transition directly (not apply_one — that's the in-crate
src/state/sequencer.rs:3561:    fn dispatch_transition_worktx_returns_state_root_via_domain_v1() {
src/state/sequencer.rs:3571:        // Build the QState by applying TaskOpen + EscrowLock through dispatch_transition,
src/state/sequencer.rs:3596:        let (q_after_open, _) = dispatch_transition(&q, &open_tx, &preds, &tools)
src/state/sequencer.rs:3608:        let (q_funded, _) = dispatch_transition(&q_after_open, &lock_tx, &preds, &tools)
src/state/sequencer.rs:3615:        let (q_next, _signals) = dispatch_transition(&q_funded, &tx, &preds, &tools)
src/state/sequencer.rs:3716:        let (q_next, _signals) = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:3747:        let (q_after_first, _) = dispatch_transition(&q, &first, &preds, &tools).expect("first");
src/state/sequencer.rs:3754:        let r = dispatch_transition(&q, &TypedTx::TaskOpen(second), &preds, &tools);
src/state/sequencer.rs:3791:        let (q_next, _) = dispatch_transition(&q, &open, &preds, &tools)
src/state/sequencer.rs:3808:        let (q_next, _signals) = dispatch_transition(&q, &lock, &preds, &tools)
src/state/sequencer.rs:3849:        let r = dispatch_transition(&q, &lock, &preds, &tools);
src/state/sequencer.rs:3865:        let r = dispatch_transition(&q, &lock, &preds, &tools);
src/state/sequencer.rs:3889:        // We modify q before any further dispatch_transition so the seed is "implicit".
src/state/sequencer.rs:3899:        let (q_next, _) = dispatch_transition(&q, &lock, &preds, &tools)
src/state/sequencer.rs:3942:        let result = dispatch_transition(&q, &work, &preds, &tools);
src/state/sequencer.rs:3964:        let result = dispatch_transition(&q, &work, &preds, &tools);
src/state/sequencer.rs:3983:        let (q_next, _) = dispatch_transition(&q, &work, &preds, &tools)
src/state/sequencer.rs:4070:        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:4106:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4125:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4145:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4159:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4221:        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:4258:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4276:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4292:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4307:        let err = dispatch_transition(&q, &tx, &preds, &tools).unwrap_err();
src/state/sequencer.rs:4497:    // TB-5 Atom 4 — apply_one stage 1.5 unit-tests (preflight § 8.4)
src/state/sequencer.rs:4501:    // apply_one stage 1.5 BEFORE dispatch_transition is invoked. Each rejection
src/state/sequencer.rs:4575:        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
src/state/sequencer.rs:4593:        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
src/state/sequencer.rs:4608:        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
src/state/sequencer.rs:4623:        let err = seq.apply_one(envelope).expect_err("forged sig must reject");
src/state/sequencer.rs:4633:    /// apply_one stage 1.5 verification (constructive correctness; pinned
src/state/sequencer.rs:4654:        let err = seq.apply_one(envelope).expect_err("target absent → reject");
src/state/sequencer.rs:4674:        // Build a WorkTx fixture and submit through apply_one directly.
src/state/sequencer.rs:4675:        // We don't care that dispatch_transition succeeds — we only assert
src/state/sequencer.rs:4681:        let result = seq.apply_one(envelope);
src/state/sequencer.rs:4693:    // U29-U33: dispatch_transition direct invocation; isolates the dispatch
src/state/sequencer.rs:4694:    // arm body from the apply_one + queue + signature pipeline.
src/state/sequencer.rs:4769:        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:4814:        let (q1, _) = dispatch_transition(&q, &tx1, &preds, &tools)
src/state/sequencer.rs:4820:        let err = dispatch_transition(&q1, &tx2, &preds, &tools)
src/state/sequencer.rs:4839:        let err = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:4863:        let (q_next, _) = dispatch_transition(&q, &tx, &preds, &tools)
src/state/sequencer.rs:4909:        let err = dispatch_transition(&q, &tx, &preds, &tools)
src/bottom_white/ledger/transition_ledger.rs:4://! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
src/bottom_white/ledger/transition_ledger.rs:25://! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
src/bottom_white/ledger/transition_ledger.rs:26://!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
src/bottom_white/ledger/transition_ledger.rs:29://! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
src/bottom_white/ledger/transition_ledger.rs:45:// § 1 LedgerEntry — the stored record (11 fields per v1.1)
src/bottom_white/ledger/transition_ledger.rs:102:/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
src/bottom_white/ledger/transition_ledger.rs:104:/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
src/bottom_white/ledger/transition_ledger.rs:108:pub struct LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:130:    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
src/bottom_white/ledger/transition_ledger.rs:135:// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
src/bottom_white/ledger/transition_ledger.rs:147:pub struct LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:159:impl LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:185:impl LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:186:    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
src/bottom_white/ledger/transition_ledger.rs:188:    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:189:        LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:229:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
src/bottom_white/ledger/transition_ledger.rs:230:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
src/bottom_white/ledger/transition_ledger.rs:271:    entries: Vec<LedgerEntry>,
src/bottom_white/ledger/transition_ledger.rs:281:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:294:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:339:    /// CO1.7-impl A4: dispatch_transition rejected the re-run. In stub state
src/bottom_white/ledger/transition_ledger.rs:355:    /// retrieved from CAS but `canonical_decode` failed (corruption /
src/bottom_white/ledger/transition_ledger.rs:375:            Self::Transition { at, inner } => write!(f, "dispatch_transition rejected at index {at}: {inner}"),
src/bottom_white/ledger/transition_ledger.rs:380:            Self::PayloadDecode { at, reason } => write!(f, "payload canonical_decode failed at index {at}: {reason}"),
src/bottom_white/ledger/transition_ledger.rs:416:/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
src/bottom_white/ledger/transition_ledger.rs:418:/// 6. canonical_decode of payload bytes → TypedTx
src/bottom_white/ledger/transition_ledger.rs:420:/// 7. dispatch_transition re-run produces (q_next, _signals)
src/bottom_white/ledger/transition_ledger.rs:426:/// so dispatch_transition can read budget / registries / balances / task markets
src/bottom_white/ledger/transition_ledger.rs:429:/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
src/bottom_white/ledger/transition_ledger.rs:434:pub fn replay_full_transition(
src/bottom_white/ledger/transition_ledger.rs:436:    entries: &[LedgerEntry],
src/bottom_white/ledger/transition_ledger.rs:445:    use crate::state::sequencer::dispatch_transition;
src/bottom_white/ledger/transition_ledger.rs:471:        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
src/bottom_white/ledger/transition_ledger.rs:486:        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
src/bottom_white/ledger/transition_ledger.rs:489:            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
src/bottom_white/ledger/transition_ledger.rs:507:        // Stage 7: re-run pure dispatch_transition.
src/bottom_white/ledger/transition_ledger.rs:509:            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
src/bottom_white/ledger/transition_ledger.rs:541:/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
src/bottom_white/ledger/transition_ledger.rs:547:    entries: &[LedgerEntry],
src/bottom_white/ledger/transition_ledger.rs:584:/// `bincode::config` used for the canonical `LedgerEntry` wire format.
src/bottom_white/ledger/transition_ledger.rs:601:pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
src/bottom_white/ledger/transition_ledger.rs:606:/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
src/bottom_white/ledger/transition_ledger.rs:608:pub fn canonical_decode<T: serde::de::DeserializeOwned>(
src/bottom_white/ledger/transition_ledger.rs:650:/// - One `LedgerEntry` = one git commit on `refs/transitions/main`.
src/bottom_white/ledger/transition_ledger.rs:655:///       `LedgerEntry` (deterministic, byte-stable; this blob IS the
src/bottom_white/ledger/transition_ledger.rs:738:    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
src/bottom_white/ledger/transition_ledger.rs:786:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:796:        let canonical = canonical_encode(entry).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:797:            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
src/bottom_white/ledger/transition_ledger.rs:855:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
src/bottom_white/ledger/transition_ledger.rs:857:        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:858:            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
src/bottom_white/ledger/transition_ledger.rs:885:    ) -> LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:886:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:899:        LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:927:        let p = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1001:    // Mutating `resulting_ledger_root` or `system_signature` of LedgerEntry must NOT
src/bottom_white/ledger/transition_ledger.rs:1002:    // change the signing payload digest (because they're not in LedgerEntrySigningPayload).
src/bottom_white/ledger/transition_ledger.rs:1031:    // Verifies: (a) typed sign API works; (b) signature verifies via CanonicalMessage::LedgerEntrySigning;
src/bottom_white/ledger/transition_ledger.rs:1046:        let payload = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1064:        let msg_clean = CanonicalMessage::LedgerEntrySigning(digest.0);
src/bottom_white/ledger/transition_ledger.rs:1074:        let msg_tamper = CanonicalMessage::LedgerEntrySigning(digest_tamper.0);
src/bottom_white/ledger/transition_ledger.rs:1089:        let msg_other_epoch = CanonicalMessage::LedgerEntrySigning(digest_other_epoch.0);
src/bottom_white/ledger/transition_ledger.rs:1232:    /// Build a real signed LedgerEntry against the given keypair + epoch,
src/bottom_white/ledger/transition_ledger.rs:1234:    /// `Sequencer::apply_one` stages 5-9 outside the runtime.
src/bottom_white/ledger/transition_ledger.rs:1244:    ) -> LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:1245:        let bytes = canonical_encode(typed_tx).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1249:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1264:        LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:1304:    fn replay_full_transition_reaches_dispatch_then_stubs() {
src/bottom_white/ledger/transition_ledger.rs:1420:        let tampered_signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1475:        let signing = LedgerEntrySigningPayload {
src/bottom_white/ledger/transition_ledger.rs:1489:        let entry = LedgerEntry {
src/bottom_white/ledger/transition_ledger.rs:1530:    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
src/bottom_white/ledger/transition_ledger.rs:1534:        let bytes = canonical_encode(&e1).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1535:        let e1_back: LedgerEntry = canonical_decode(&bytes).expect("decode");
src/bottom_white/ledger/transition_ledger.rs:1539:        let bytes_again = canonical_encode(&e1).expect("encode again");
src/bottom_white/ledger/transition_ledger.rs:1545:        let err = canonical_decode::<LedgerEntry>(&bytes_extra).unwrap_err();

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! L4 Transition Ledger (CO1.7) — implementation atom.
     2	//!
     3	//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
     4	//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
     5	//! TRACE_MATRIX § 1-§ 8 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.2): schema +
     6	//! append() + replay_chain_integrity() + replay_full_transition() + Git2LedgerWriter.
     7	//!
     8	//! **Status**: CO1.7 spec PASS/PASS (3 rounds) + CO1.7-impl bundle PASS/PASS
     9	//! (3 rounds: A1+A2+A3+A4 + CO1.4-extra). Per-kind transition function bodies
    10	//! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
    11	//!
    12	//! v1 → v1.1 changes (smoke for round-2 dual audit):
    13	//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
    14	//!   exposes `replay_chain_integrity` only (renamed for honesty).
    15	//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
    16	//!   sequencer code (deferred to CO1.7.5).
    17	//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
    18	//!   defense); new test asserts replay rejects parent_ledger_root tamper.
    19	//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
    20	//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
    21	//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
    22	//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
    23	//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
    24	//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
    25	//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
    26	//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
    27	//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
    28	//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
    29	//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
    30	//!   structurally enforces "derivatives excluded".
    31	//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).
    32	
    33	use std::collections::BTreeMap;
    34	use std::path::{Path, PathBuf};
    35	
    36	use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
    37	use serde::{Deserialize, Serialize};
    38	use sha2::{Digest, Sha256};
    39	
    40	use crate::bottom_white::cas::schema::Cid;
    41	use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
    42	use crate::state::q_state::Hash;
    43	
    44	// ────────────────────────────────────────────────────────────────────────────
    45	// § 1 LedgerEntry — the stored record (11 fields per v1.1)
    46	// ────────────────────────────────────────────────────────────────────────────
    47	
    48	/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
    49	/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
    50	/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
    51	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    52	#[repr(u8)]
    53	pub enum TxKind {
    54	    Work            = 0,
    55	    Verify          = 1,
    56	    Challenge       = 2,
    57	    Reuse           = 3,
    58	    FinalizeReward  = 4,
    59	    TaskExpire      = 5,
    60	    TerminalSummary = 6,
    61	    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted task
    62	    /// market registration; metadata-only (no money movement).
    63	    TaskOpen        = 7,
    64	    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted bounty
    65	    /// funding; the sole RSP-1 path that grows `task_markets_t.total_escrow`.
    66	    EscrowLock      = 8,
    67	    /// TB-5 RSP-3.0/3.1 system-emitted resolution (charter v2 § 4.1 + § 4.5).
    68	    /// System-only: agent ingress rejected pre-queue; emit via
    69	    /// `Sequencer::emit_system_tx`. Released refunds challenger bond + flips
    70	    /// ChallengeCase.status; UpheldDeferred is a marker only (slash is
    71	    /// RSP-3.2 / TB-6 territory).
    72	    ChallengeResolve = 9,
    73	    /// TB-11 (2026-05-02 architect ruling §6.2) — system-emitted task-level
    74	    /// failure marker. Future TB-12 NodeMarket Short / NO settlement uses
    75	    /// this as the canonical resolution anchor (death certificate).
    76	    /// System-only: agent ingress rejected pre-queue; emit via
    77	    /// `Sequencer::emit_system_tx`. No money movement (refund is a separate
    78	    /// TaskExpireTx fired by tick post-bankruptcy).
    79	    TaskBankruptcy  = 10,
    80	    /// TB-13 (2026-05-03 architect post-TB-12 ruling Part A §4.3) —
    81	    /// agent-signed conditional-share mint. Debits `balances_t[owner]`,
    82	    /// credits `conditional_collateral_t[event_id]`, mints equal YES_E +
    83	    /// NO_E shares to `conditional_share_balances_t`. CTF preserved (1
    84	    /// Coin → 1 YES + 1 NO; shares are claims, not Coin).
    85	    CompleteSetMint   = 11,
    86	    /// TB-13 (architect §4.3) — agent-signed conditional-share redeem
    87	    /// post-resolution. Resolution authority is the live
    88	    /// `task_markets_t[event_id.0].state` (Finalized → Yes wins; Bankrupt
    89	    /// → No wins). Pays winning side 1:1 against
    90	    /// `conditional_collateral_t`. Pre-resolution rejected with
    91	    /// `RedeemBeforeResolution`; outcome-vs-state mismatch rejected with
    92	    /// `InvalidResolutionRef`.
    93	    CompleteSetRedeem = 12,
    94	    /// TB-13 (architect §4.3) — agent-signed protocol-owned share
    95	    /// inventory seed. Provider explicitly funds `conditional_collateral_t`
    96	    /// + receives BOTH YES + NO shares. **No trading. No quoting. No
    97	    /// pricing.** TB-13 records only the fact of seeding, not any signal
    98	    /// derived from it.
    99	    MarketSeed        = 13,
   100	}
   101	
   102	/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
   103	///
   104	/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
   105	/// (includes derivatives + signature); the signing payload is the subset that
   106	/// the system keypair attests.
   107	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   108	pub struct LedgerEntry {
   109	    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
   110	    /// submissions never get a logical_t.
   111	    pub logical_t: u64,                          //  1
   112	    pub parent_state_root: Hash,                 //  2
   113	    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
   114	    /// prevent transplant attacks.
   115	    pub parent_ledger_root: Hash,                //  3
   116	    pub tx_kind: TxKind,                         //  4
   117	    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
   118	    pub tx_payload_cid: Cid,                     //  5
   119	    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
   120	    /// returned by transition function per K3 boundary).
   121	    pub resulting_state_root: Hash,              //  6
   122	    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
   123	    pub resulting_ledger_root: Hash,             //  7
   124	    pub timestamp_logical: u64,                  //  8
   125	    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
   126	    pub epoch: SystemEpoch,                      //  9
   127	    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
   128	    /// Bound in signed payload (G1 cannot bypass signature).
   129	    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
   130	    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
   131	    pub system_signature: SystemSignature,       // 11
   132	}
   133	
   134	// ────────────────────────────────────────────────────────────────────────────
   135	// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
   136	// ────────────────────────────────────────────────────────────────────────────
   137	
   138	/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
   139	///
   140	/// **Excludes** (Q9 cycle prevention):
   141	/// - `resulting_ledger_root` (derivative; including → cycle)
   142	/// - `system_signature` (its own input)
   143	///
   144	/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
   145	/// part of the digest to prevent cross-namespace collision.
   146	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
   147	pub struct LedgerEntrySigningPayload {
   148	    pub logical_t: u64,
   149	    pub parent_state_root: Hash,
   150	    pub parent_ledger_root: Hash,                  // K2
   151	    pub tx_kind: TxKind,
   152	    pub tx_payload_cid: Cid,
   153	    pub resulting_state_root: Hash,
   154	    pub timestamp_logical: u64,
   155	    pub epoch: SystemEpoch,                        // D1
   156	    pub extensions: BTreeMap<String, Vec<u8>>,     // G1
   157	}
   158	
   159	impl LedgerEntrySigningPayload {
   160	    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
   161	    pub fn canonical_digest(&self) -> Hash {
   162	        let mut h = Sha256::new();
   163	        h.update(b"turingosv4.ledger_entry_signing.v1");
   164	        h.update(self.logical_t.to_be_bytes());
   165	        h.update(self.parent_state_root.0);
   166	        h.update(self.parent_ledger_root.0);
   167	        h.update((self.tx_kind as u8).to_be_bytes()); // K6 #[repr(u8)] makes cast stable
   168	        h.update(self.tx_payload_cid.0);
   169	        h.update(self.resulting_state_root.0);
   170	        h.update(self.timestamp_logical.to_be_bytes());
   171	        h.update(self.epoch.get().to_be_bytes());
   172	        // Extensions: BTreeMap iterates in lex key order (deterministic);
   173	        // length-prefix every field to prevent ambiguity attacks.
   174	        h.update((self.extensions.len() as u64).to_be_bytes());
   175	        for (k, v) in &self.extensions {
   176	            h.update((k.len() as u64).to_be_bytes());
   177	            h.update(k.as_bytes());
   178	            h.update((v.len() as u64).to_be_bytes());
   179	            h.update(v);
   180	        }
   181	        Hash(h.finalize().into())
   182	    }
   183	}
   184	
   185	impl LedgerEntry {
   186	    /// Project the LedgerEntry's signed-fields-subset back into a signing payload.
   187	    /// Used by replay to recompute `signing_digest` and re-verify chain integrity.
   188	    pub fn to_signing_payload(&self) -> LedgerEntrySigningPayload {
   189	        LedgerEntrySigningPayload {
   190	            logical_t: self.logical_t,
   191	            parent_state_root: self.parent_state_root,
   192	            parent_ledger_root: self.parent_ledger_root,
   193	            tx_kind: self.tx_kind,
   194	            tx_payload_cid: self.tx_payload_cid,
   195	            resulting_state_root: self.resulting_state_root,
   196	            timestamp_logical: self.timestamp_logical,
   197	            epoch: self.epoch,
   198	            extensions: self.extensions.clone(),
   199	        }
   200	    }
   201	}
   202	
   203	// ────────────────────────────────────────────────────────────────────────────
   204	// § 4 append() — pure ledger-root fold
   205	// ────────────────────────────────────────────────────────────────────────────
   206	
   207	/// TRACE_MATRIX FC2-Append + spec § 4: pure ledger-root fold over signed digests.
   208	/// Same `(parent_root, signing_digest)` → byte-identical `new_root`.
   209	/// No I/O, no clock, no env. Witness for I-DET ledger axis.
   210	pub fn append(parent_root: &Hash, signing_digest: &Hash) -> Hash {
   211	    let mut h = Sha256::new();
   212	    h.update(b"turingosv4.ledger_root.v1");
   213	    h.update(parent_root.0);
   214	    h.update(signing_digest.0);
   215	    Hash(h.finalize().into())
   216	}
   217	
   218	// ────────────────────────────────────────────────────────────────────────────
   219	// LedgerWriter trait (K4 reconciled to skeleton signature)
   220	// ────────────────────────────────────────────────────────────────────────────
   221	
   222	/// TRACE_MATRIX FC2-Append: storage abstraction for L4.
   223	/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
   224	/// Test/skeleton impl is `InMemoryLedgerWriter` below.
   225	///
   226	/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
   227	/// deferred to CO1.7.5+ (only used by FullTransition replay; not v1 deliverable).
   228	pub trait LedgerWriter: Send + Sync {
   229	    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
   230	    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
   231	    fn len(&self) -> u64;
   232	
   233	    /// TRACE_MATRIX § 5 — L4 sequencer post-commit head_t wiring (Art 0.4).
   234	    ///
   235	    /// Canonical 40-char lowercase hex commit OID of the most recent appended
   236	    /// entry, or None if the chain is empty / backend has no commit-OID notion.
   237	    ///
   238	    /// **REQUIRED** (no default impl per CO1.7-extra round-2 MF3): Rust compiler
   239	    /// enforces every `LedgerWriter` implementation declares this method. This
   240	    /// satisfies both safety arguments raised across the audit arc:
   241	    /// - **silent stagnation prevention**: impossible to inherit a default that
   242	    ///   silently leaves head_t stale; a missing impl is a compile error.
   243	    /// - **post-commit no-panic**: impl is free to return None at runtime if the
   244	    ///   backend has no OID notion (e.g. InMemoryLedgerWriter); no panic risk.
   245	    fn head_commit_oid_hex(&self) -> Option<String>;
   246	}
   247	
   248	#[derive(Debug)]
   249	pub enum LedgerWriterError {
   250	    LogicalTGap { expected: u64, got: u64 },
   251	    NotFound { logical_t: u64 },
   252	    BackendCorruption(String),
   253	}
   254	
   255	impl std::fmt::Display for LedgerWriterError {
   256	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   257	        match self {
   258	            Self::LogicalTGap { expected, got } => {
   259	                write!(f, "logical_t gap: expected {expected}, got {got}")
   260	            }

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2920,3195p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2920	    /// BOTH dispatch failures (stage 2) AND signature-verification failures
  2921	    /// (stage 1.5). Behavior preserved exactly per the existing TB-2 Atom 4
  2922	    /// rejection-writer semantics: no logical_t / state_root / ledger_root
  2923	    /// advance. Records:
  2924	    /// - tx_payload_cid (canonical-encoded TypedTx)
  2925	    /// - raw_diagnostic_cid (TransitionError display, structurally
  2926	    ///   serde-shielded on RejectedSubmissionRecord per TB-1 P0-3)
  2927	    /// - rejection_class via `rejection_class_for(err)`
  2928	    /// - public_summary via `public_summary_for(err)`
  2929	    /// - agent_id via `tx.submitter_id().unwrap_or(SYSTEM_AGENT_ID)`
  2930	    fn record_rejection(
  2931	        &self,
  2932	        submit_id: u64,
  2933	        tx: &TypedTx,
  2934	        q_snapshot: &QState,
  2935	        err: &TransitionError,
  2936	    ) -> Result<(), ApplyError> {
  2937	        let payload_bytes = canonical_encode(tx)
  2938	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
  2939	        let creator = format!("sequencer.rejection_path.epoch-{}", self.epoch.get());
  2940	        let rejection_logical_t = self.next_logical_t.load(Ordering::SeqCst);
  2941	
  2942	        let tx_payload_cid = {
  2943	            let mut cas_w = self
  2944	                .cas
  2945	                .write()
  2946	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2947	            cas_w.put(
  2948	                &payload_bytes,
  2949	                ObjectType::ProposalPayload,
  2950	                &creator,
  2951	                rejection_logical_t,
  2952	                Some("TypedTx.v1".to_string()),
  2953	            )?
  2954	        };
  2955	
  2956	        let diag_bytes = err.to_string().into_bytes();
  2957	        let raw_diagnostic_cid = {
  2958	            let mut cas_w = self
  2959	                .cas
  2960	                .write()
  2961	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2962	            Some(cas_w.put(
  2963	                &diag_bytes,
  2964	                ObjectType::Generic,
  2965	                &creator,
  2966	                rejection_logical_t,
  2967	                Some("TransitionError.display.v1".to_string()),
  2968	            )?)
  2969	        };
  2970	
  2971	        let agent_id = tx
  2972	            .submitter_id()
  2973	            .unwrap_or_else(|| AgentId(SYSTEM_AGENT_ID_STR.to_string()));
  2974	
  2975	        {
  2976	            let mut writer_w = self
  2977	                .rejection_writer
  2978	                .write()
  2979	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  2980	            writer_w.append_rejected(
  2981	                submit_id,
  2982	                q_snapshot.state_root_t,
  2983	                agent_id,
  2984	                tx.tx_kind(),
  2985	                tx_payload_cid,
  2986	                rejection_class_for(err),
  2987	                raw_diagnostic_cid,
  2988	                public_summary_for(err),
  2989	            );
  2990	        }
  2991	        Ok(())
  2992	    }
  2993	
  2994	    /// TRACE_MATRIX FC3-S3: L4 sequencer per-tx critical section.
  2995	    ///
  2996	    /// Pure transition + CAS put + sign + commit + Q_t mutation. See spec § 3
  2997	    /// stages 1-9. TB-2 Atom 2 changes the input type from `TypedTx` to
  2998	    /// `SubmissionEnvelope` so `submit_id` travels in (charter §1 / P1:6);
  2999	    /// the apply pipeline itself is unchanged in Atom 2.
  3000	    ///
  3001	    /// **v1.1 C-2 closure (Codex bundle Q-B)**: `next_logical_t` advances
  3002	    /// **only on commit success** — the original spec § 3 stage-4
  3003	    /// `fetch_add(1)` happened BEFORE sign + writer.commit, so any infra
  3004	    /// failure (sign / commit) left `next_logical_t` advanced past a
  3005	    /// logical_t that was never written to the ledger. The next accepted
  3006	    /// tx would then be assigned a logical_t the writer rejects forever
  3007	    /// (writer enforces strict `len + 1`). Fixed by `load → use → store
  3008	    /// after commit succeeds`. Single-writer per spec § 5.2.1 makes the
  3009	    /// load+store atomic enough; if multi-writer ever lands the AtomicU64
  3010	    /// can be upgraded to a `compare_exchange` reservation pattern.
  3011	    pub(crate) fn apply_one(
  3012	        &self,
  3013	        envelope: SubmissionEnvelope,
  3014	    ) -> Result<LedgerEntry, ApplyError> {
  3015	        // TB-2 Atom 2: queue payload is SubmissionEnvelope so submit_id
  3016	        // travels with the tx through to apply_one. Atom 4: submit_id is
  3017	        // now actually used for the L4.E rejection-evidence path below.
  3018	        let SubmissionEnvelope { submit_id, tx } = envelope;
  3019	
  3020	        // Stage 1: snapshot Q_t under read lock.
  3021	        let q_snapshot = {
  3022	            let g = self.q.read().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3023	            g.clone()
  3024	        };
  3025	
  3026	        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
  3027	        // verification for system-emitted variants. Even though emit_system_tx
  3028	        // signs the message before queueing, apply_one re-verifies against
  3029	        // pinned_pubkeys here so that any future bypass of emit_system_tx
  3030	        // (or stale signature in a replay) is rejected at the apply boundary.
  3031	        // On verification failure, route to L4.E with InvalidSystemSignatureLive
  3032	        // exactly like a dispatch reject — no logical_t consumed, no state_root
  3033	        // advance.
  3034	        if let Some(msg) = system_message_for_verification(&tx) {
  3035	            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
  3036	            let sig = system_signature_of(&tx)
  3037	                .expect("system_message_for_verification implies system_signature present");
  3038	            // TerminalSummaryTx carries no epoch field (STATE § 1.5 8-field
  3039	            // schema is digest-only); fall back to the apply-time sequencer
  3040	            // epoch. Other system variants carry epoch on the wire.
  3041	            let tx_epoch = system_epoch_of(&tx).unwrap_or(self.epoch);
  3042	            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
  3043	                let err = TransitionError::InvalidSystemSignatureLive;
  3044	                self.record_rejection(submit_id, &tx, &q_snapshot, &err)?;
  3045	                return Err(ApplyError::Transition(err));
  3046	            }
  3047	        }
  3048	
  3049	        // Stage 2: dispatch (pure). On reject, route to L4.E rejection-evidence
  3050	        // ledger and return early. K1: no logical_t consumed; Inv 7: no
  3051	        // state_root_t / ledger_root_t advance.
  3052	        let (q_next, _signals) = match dispatch_transition(
  3053	            &q_snapshot,
  3054	            &tx,
  3055	            &self.predicate_registry,
  3056	            &self.tool_registry,
  3057	        ) {
  3058	            Ok(ok) => ok,
  3059	            Err(transition_err) => {
  3060	                self.record_rejection(submit_id, &tx, &q_snapshot, &transition_err)?;
  3061	                // No logical_t advance, no state_root advance, no ledger_root
  3062	                // advance. Caller observes ApplyError::Transition.
  3063	                return Err(ApplyError::Transition(transition_err));
  3064	            }
  3065	        };
  3066	
  3067	        // v1.1 C-2: TENTATIVE logical_t (do NOT fetch_add yet).
  3068	        let logical_t = self.next_logical_t.load(Ordering::SeqCst) + 1;
  3069	
  3070	        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
  3071	        let payload_bytes = canonical_encode(&tx)
  3072	            .map_err(|e| ApplyError::PayloadEncode(e.to_string()))?;
  3073	        let payload_cid = {
  3074	            let mut cas_w = self.cas.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3075	            cas_w.put(
  3076	                &payload_bytes,
  3077	                ObjectType::ProposalPayload,
  3078	                &format!("sequencer-epoch-{}", self.epoch.get()),
  3079	                logical_t,
  3080	                Some("TypedTx.v1".to_string()),
  3081	            )?
  3082	        };
  3083	
  3084	        // Stage 3.5 — TB-15 Atom 3 (architect §6.2): post-dispatch autopsy
  3085	        // CAS-write hook. For accepted TaskBankruptcyTx, derive the same
  3086	        // capsules the dispatch arm pushed Cids for + write their bytes
  3087	        // (capsule + private_detail) to CAS. Idempotent: identical bytes
  3088	        // → identical Cids → CAS dedupe. Replay-safe: re-running this
  3089	        // produces the same CAS state. Failure here is a hard error
  3090	        // (ApplyError) — autopsy bytes MUST be retrievable for SG-15.6
  3091	        // dashboard regenerability.
  3092	        // R2 closure (Gemini R1 VETO Q12): activation-gate the CAS write
  3093	        // identically to the dispatch arm. Both gates pin on the same
  3094	        // constant TB15_AUTOPSY_ACTIVATION_LOGICAL_T → dispatch and
  3095	        // apply_one stay agreement-locked: pre-cutoff rows write nothing
  3096	        // to CAS AND populate no agent_autopsies_t Cids.
  3097	        if let TypedTx::TaskBankruptcy(bk) = &tx {
  3098	            if crate::runtime::autopsy_capsule::is_autopsy_active_at(bk.timestamp_logical) {
  3099	                let _ = crate::runtime::autopsy_capsule::write_bankruptcy_autopsies_to_cas(
  3100	                    &self.cas,
  3101	                    &q_snapshot.economic_state_t,
  3102	                    bk,
  3103	                    q_snapshot.q_t.current_round,
  3104	                    bk.timestamp_logical,
  3105	                    &format!("sequencer-epoch-{}", self.epoch.get()),
  3106	                )
  3107	                .map_err(|e| match e {
  3108	                    crate::runtime::autopsy_capsule::AutopsyWriteError::Cas(c) => {
  3109	                        ApplyError::Cas(c)
  3110	                    }
  3111	                    crate::runtime::autopsy_capsule::AutopsyWriteError::Encode(s) => {
  3112	                        ApplyError::PayloadEncode(s)
  3113	                    }
  3114	                    crate::runtime::autopsy_capsule::AutopsyWriteError::InternalLockPoisoned => {
  3115	                        ApplyError::QStateLockPoisoned
  3116	                    }
  3117	                })?;
  3118	            }
  3119	        }
  3120	
  3121	        // Stage 5: build LedgerEntrySigningPayload (v1.1 — stage 4 fetch_add
  3122	        // moved to AFTER stage 9 commit success).
  3123	        let signing_payload = LedgerEntrySigningPayload {
  3124	            logical_t,
  3125	            parent_state_root: q_snapshot.state_root_t,
  3126	            parent_ledger_root: q_snapshot.ledger_root_t,
  3127	            tx_kind: tx.tx_kind(),
  3128	            tx_payload_cid: payload_cid,
  3129	            resulting_state_root: q_next.state_root_t,
  3130	            timestamp_logical: logical_t,
  3131	            epoch: self.epoch,
  3132	            extensions: std::collections::BTreeMap::new(),
  3133	        };
  3134	
  3135	        // Stage 6: C3 — sign via typed CanonicalMessage::LedgerEntrySigning.
  3136	        let signing_digest = signing_payload.canonical_digest();
  3137	        let system_signature = transition_ledger_emitter::sign_ledger_entry(
  3138	            &self.keypair,
  3139	            signing_digest.0,
  3140	        )?;
  3141	
  3142	        // Stage 7: pure ledger-root fold (deterministic).
  3143	        let resulting_ledger_root = append(&q_snapshot.ledger_root_t, &signing_digest);
  3144	
  3145	        // Stage 8: build LedgerEntry (the stored record).
  3146	        let entry = LedgerEntry {
  3147	            logical_t: signing_payload.logical_t,
  3148	            parent_state_root: signing_payload.parent_state_root,
  3149	            parent_ledger_root: signing_payload.parent_ledger_root,
  3150	            tx_kind: signing_payload.tx_kind,
  3151	            tx_payload_cid: signing_payload.tx_payload_cid,
  3152	            resulting_state_root: signing_payload.resulting_state_root,
  3153	            resulting_ledger_root,
  3154	            timestamp_logical: signing_payload.timestamp_logical,
  3155	            epoch: signing_payload.epoch,
  3156	            extensions: signing_payload.extensions,
  3157	            system_signature,
  3158	        };
  3159	
  3160	        // Stage 9: commit + mutate Q_t under write lock.
  3161	        // v1.1 C-2: next_logical_t.store(logical_t) HAPPENS ONLY AFTER
  3162	        // writer.commit succeeds — preserves K1 under infra failure.
  3163	        // CO1.7-extra D2: q.head_t = NodeId(commit_oid_hex) via advance_head_t
  3164	        // when writer surfaces a commit OID (Git2 path); no-op preservation
  3165	        // for writers that return None (InMemory path). state_root_t comes
  3166	        // from q_next as-is per K3 v1.2.
  3167	        {
  3168	            let mut q_w = self.q.write().map_err(|_| ApplyError::QStateLockPoisoned)?;
  3169	            let mut writer_w = self
  3170	                .ledger_writer
  3171	                .write()
  3172	                .map_err(|_| ApplyError::QStateLockPoisoned)?;
  3173	            writer_w.commit(&entry)?; // ← may fail; if it does, fetch_add was NOT called
  3174	            // commit succeeded → safe to advance counter.
  3175	            self.next_logical_t.store(logical_t, Ordering::SeqCst);
  3176	            *q_w = q_next;
  3177	            q_w.ledger_root_t = entry.resulting_ledger_root;
  3178	            // CO1.7-extra D2: close G-1 head_t carry-forward (Art 0.4).
  3179	            advance_head_t(&mut *q_w, &*writer_w);
  3180	        }
  3181	
  3182	        Ok(entry)
  3183	    }
  3184	
  3185	    /// Read-only accessor (testing + CO1.7.5+ wiring).
  3186	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
  3187	        self.q
  3188	            .read()
  3189	            .map(|g| g.clone())
  3190	            .map_err(|_| ApplyError::QStateLockPoisoned)
  3191	    }
  3192	
  3193	    /// TRACE_MATRIX TB-14 Atom 6 B′ step 4 (FC2-N28; architect ruling
  3194	    /// 2026-05-03 §3+§4): build a canonical-keyed parent → children edge
  3195	    /// map by walking the L4 chain and reading

exec
/bin/bash -lc "nl -ba src/bottom_white/ledger/transition_ledger.rs | sed -n '416,620p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   416	/// 4. system_signature verifies via CanonicalMessage::LedgerEntrySigning + pinned pubkeys
   417	/// 5. CAS lookup of tx_payload_cid succeeds (CO1.4-extra cold-replay capability)
   418	/// 6. canonical_decode of payload bytes → TypedTx
   419	/// 6.5 (v1.1 C-3): decoded_typed_tx.tx_kind() MUST equal entry.tx_kind
   420	/// 7. dispatch_transition re-run produces (q_next, _signals)
   421	/// 8. q_next.state_root_t matches entry.resulting_state_root
   422	/// 9. resulting_ledger_root recomputed via append() matches stored
   423	///
   424	/// **v1.1 C-1 closure**: takes a full `genesis: &QState` (was `genesis_state_root`
   425	/// + `genesis_ledger_root` only). Caller provides the complete genesis state
   426	/// so dispatch_transition can read budget / registries / balances / task markets
   427	/// — fabricating `QState::genesis()` was dropping these fields.
   428	///
   429	/// **Stub-state caveat (CO1.7.5 unblocks)**: while `dispatch_transition`
   430	/// returns `NotYetImplemented` for every variant, replay errors at stage 7
   431	/// for any non-empty chain. Conformance tests exercising stages 1-6.5
   432	/// independently are `#[test]`-runnable now; full state_root reconstruction
   433	/// gates on CO1.7.5.
   434	pub fn replay_full_transition(
   435	    genesis: &crate::state::q_state::QState,
   436	    entries: &[LedgerEntry],
   437	    cas: &dyn LedgerCasView,
   438	    pinned_pubkeys: &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys,
   439	    predicate_registry: &crate::top_white::predicates::registry::PredicateRegistry,
   440	    tool_registry: &crate::bottom_white::tools::registry::ToolRegistry,
   441	) -> Result<crate::state::q_state::QState, ReplayError> {
   442	    use crate::bottom_white::ledger::system_keypair::{
   443	        verify_system_signature, CanonicalMessage,
   444	    };
   445	    use crate::state::sequencer::dispatch_transition;
   446	
   447	    let mut q = genesis.clone();
   448	
   449	    for (i, entry) in entries.iter().enumerate() {
   450	        // Stage 1
   451	        let expected_logical_t = (i as u64) + 1;
   452	        if entry.logical_t != expected_logical_t {
   453	            return Err(ReplayError::LogicalTGap {
   454	                at: i,
   455	                expected: expected_logical_t,
   456	                got: entry.logical_t,
   457	            });
   458	        }
   459	        // Stage 2
   460	        if entry.parent_state_root != q.state_root_t {
   461	            return Err(ReplayError::ParentStateMismatch { at: i });
   462	        }
   463	        // Stage 3
   464	        if entry.parent_ledger_root != q.ledger_root_t {
   465	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   466	        }
   467	
   468	        // Stage 4: system_signature verify (FullTransition mode only).
   469	        let signing_payload = entry.to_signing_payload();
   470	        let signing_digest = signing_payload.canonical_digest();
   471	        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
   472	        if !verify_system_signature(
   473	            &entry.system_signature,
   474	            &canonical_msg,
   475	            entry.epoch,
   476	            pinned_pubkeys,
   477	        ) {
   478	            return Err(ReplayError::BadSignature { at: i });
   479	        }
   480	
   481	        // Stage 5: CAS lookup.
   482	        let payload_bytes = cas
   483	            .get_typed_payload(&entry.tx_payload_cid)
   484	            .map_err(|_| ReplayError::CasMissing { at: i })?;
   485	
   486	        // Stage 6: canonical_decode → TypedTx (v1.1 C-3-secondary: distinct
   487	        // error from CasMissing).
   488	        let typed_tx: crate::state::typed_tx::TypedTx =
   489	            canonical_decode(&payload_bytes).map_err(|e| ReplayError::PayloadDecode {
   490	                at: i,
   491	                reason: e.to_string(),
   492	            })?;
   493	
   494	        // Stage 6.5 (v1.1 C-3): tx_kind envelope vs decoded payload kind MUST match.
   495	        // Otherwise a signed envelope claiming `Work` could ride a CAS payload
   496	        // that decodes as `Verify` — sequencer would have written that
   497	        // mismatch but replay would have silently accepted it pre-v1.1.
   498	        let decoded_kind = typed_tx.tx_kind();
   499	        if decoded_kind != entry.tx_kind {
   500	            return Err(ReplayError::TxKindMismatch {
   501	                at: i,
   502	                envelope_kind: entry.tx_kind,
   503	                decoded_kind,
   504	            });
   505	        }
   506	
   507	        // Stage 7: re-run pure dispatch_transition.
   508	        let (q_next, _signals) =
   509	            dispatch_transition(&q, &typed_tx, predicate_registry, tool_registry)
   510	                .map_err(|inner| ReplayError::Transition { at: i, inner })?;
   511	
   512	        // Stage 8: state_root match.
   513	        if q_next.state_root_t != entry.resulting_state_root {
   514	            return Err(ReplayError::StateRootMismatch { at: i });
   515	        }
   516	
   517	        // Stage 9: ledger_root match (recompute via append).
   518	        let recomputed_ledger_root = append(&q.ledger_root_t, &signing_digest);
   519	        if recomputed_ledger_root != entry.resulting_ledger_root {
   520	            return Err(ReplayError::LedgerRootMismatch { at: i });
   521	        }
   522	
   523	        // Advance.
   524	        q = q_next;
   525	        q.ledger_root_t = entry.resulting_ledger_root;
   526	    }
   527	
   528	    Ok(q)
   529	}
   530	
   531	/// Skeleton-stage entry point (v1.1).
   532	///
   533	/// Validates:
   534	/// 1. logical_t monotonicity (no gaps, no duplicates)
   535	/// 2. parent_state_root chain
   536	/// 3. parent_ledger_root chain (K2 transplant defense)
   537	/// 4. resulting_ledger_root recomputed via append(prev_ledger_root, signing_digest)
   538	///
   539	/// Does NOT verify:
   540	/// - system_signature (CO1.7.5+: requires CanonicalMessage extension wired through keypair)
   541	/// - resulting_state_root (CO1.7.5+: requires dispatch_transition + CO1.4-extra CAS persistence)
   542	///
   543	/// Returns final (state_root, ledger_root) on success.
   544	pub fn replay_chain_integrity(
   545	    genesis_state_root: Hash,
   546	    genesis_ledger_root: Hash,
   547	    entries: &[LedgerEntry],
   548	) -> Result<(Hash, Hash), ReplayError> {
   549	    let mut prev_state_root = genesis_state_root;
   550	    let mut prev_ledger_root = genesis_ledger_root;
   551	
   552	    for (i, entry) in entries.iter().enumerate() {
   553	        let expected_logical_t = (i as u64) + 1;
   554	        if entry.logical_t != expected_logical_t {
   555	            return Err(ReplayError::LogicalTGap {
   556	                at: i,
   557	                expected: expected_logical_t,
   558	                got: entry.logical_t,
   559	            });
   560	        }
   561	        if entry.parent_state_root != prev_state_root {
   562	            return Err(ReplayError::ParentStateMismatch { at: i });
   563	        }
   564	        // K2 NEW: parent_ledger_root chain check
   565	        if entry.parent_ledger_root != prev_ledger_root {
   566	            return Err(ReplayError::ParentLedgerMismatch { at: i });
   567	        }
   568	        let signing_digest = entry.to_signing_payload().canonical_digest();
   569	        let recomputed = append(&prev_ledger_root, &signing_digest);
   570	        if recomputed != entry.resulting_ledger_root {
   571	            return Err(ReplayError::LedgerRootMismatch { at: i });
   572	        }
   573	        prev_state_root = entry.resulting_state_root;
   574	        prev_ledger_root = entry.resulting_ledger_root;
   575	    }
   576	
   577	    Ok((prev_state_root, prev_ledger_root))
   578	}
   579	
   580	// ────────────────────────────────────────────────────────────────────────────
   581	// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
   582	// ────────────────────────────────────────────────────────────────────────────
   583	
   584	/// `bincode::config` used for the canonical `LedgerEntry` wire format.
   585	///
   586	/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
   587	/// - **Big-endian** byte order (network order; deterministic across platforms).
   588	/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
   589	/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
   590	///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
   591	/// - **No padding, no implicit alignment.**
   592	fn bincode_canonical_config() -> impl bincode::config::Config {
   593	    bincode::config::standard()
   594	        .with_big_endian()
   595	        .with_fixed_int_encoding()
   596	}
   597	
   598	/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
   599	/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
   600	/// needing byte-stable signatures over typed payloads.
   601	pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
   602	    bincode::serde::encode_to_vec(value, bincode_canonical_config())
   603	        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
   604	}
   605	
   606	/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
   607	/// the number of bytes consumed (entire input must be consumed for a clean decode).
   608	pub fn canonical_decode<T: serde::de::DeserializeOwned>(
   609	    bytes: &[u8],
   610	) -> Result<T, CanonicalCodecError> {
   611	    let (value, consumed) =
   612	        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
   613	            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
   614	    if consumed != bytes.len() {
   615	        return Err(CanonicalCodecError::TrailingBytes {
   616	            consumed,
   617	            total: bytes.len(),
   618	        });
   619	    }
   620	    Ok(value)

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '3185,3255p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  3185	    /// Read-only accessor (testing + CO1.7.5+ wiring).
  3186	    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
  3187	        self.q
  3188	            .read()
  3189	            .map(|g| g.clone())
  3190	            .map_err(|_| ApplyError::QStateLockPoisoned)
  3191	    }
  3192	
  3193	    /// TRACE_MATRIX TB-14 Atom 6 B′ step 4 (FC2-N28; architect ruling
  3194	    /// 2026-05-03 §3+§4): build a canonical-keyed parent → children edge
  3195	    /// map by walking the L4 chain and reading
  3196	    /// `ProposalTelemetry.parent_tx` for each accepted WorkTx via its
  3197	    /// `proposal_cid`. Replaces the legacy shadow `kernel.tape`
  3198	    /// consumption at the bus snapshot's mask-set derivation site
  3199	    /// (canonical-graph rewire closes Codex R1 ship audit VETO; full
  3200	    /// detail in `handover/directives/2026-05-03_TB14_ATOM6_VETO_RULING.md`).
  3201	    ///
  3202	    /// **Replay determinism** (Art.0.2): the L4 chain + CAS payloads are
  3203	    /// both replay-deterministic per TB-13 chaintape evidence. Walking
  3204	    /// L4 in `read_at` order + reading ProposalTelemetry from CAS
  3205	    /// produces a byte-equal `BTreeMap<TxId, BTreeSet<TxId>>` across
  3206	    /// live vs replay.
  3207	    ///
  3208	    /// **Empty fallback**: failures at any layer (lock poisoned, CAS
  3209	    /// missing payload, canonical_decode error, ProposalTelemetry
  3210	    /// decode error, no parent_tx in telemetry) are silently skipped
  3211	    /// rather than propagated — bus.snapshot must NEVER crash because
  3212	    /// of an edge-map build failure (consumers handle empty as "no
  3213	    /// canonical edges yet"). The L4 chain itself is the canonical
  3214	    /// source of truth; this is a derived view.
  3215	    ///
  3216	    /// **Cost**: O(N + N·CAS_read) per call where N = L4 length. Bus
  3217	    /// snapshot frequency is bounded by the evaluator iteration cap;
  3218	    /// for a 50-iteration run this is ~50²/2 = 1250 CAS reads total.
  3219	    /// A future optimization can cache by writer.len() but is premature
  3220	    /// at B′ step 4.
  3221	    ///
  3222	    /// **TB-9 zero-CID synthetic seed**: legacy synthetic-seed WorkTx
  3223	    /// (proposal_cid = `[0u8; 32]`) has no telemetry record; skipped
  3224	    /// silently (mirrors `chain_derived_run_facts` line 340 discipline).
  3225	    pub fn compute_canonical_edges_at_head(
  3226	        &self,
  3227	    ) -> std::collections::BTreeMap<crate::state::TxId, std::collections::BTreeSet<crate::state::TxId>>
  3228	    {
  3229	        use crate::bottom_white::ledger::transition_ledger::canonical_decode;
  3230	        use crate::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
  3231	
  3232	        let mut edges: std::collections::BTreeMap<
  3233	            crate::state::TxId,
  3234	            std::collections::BTreeSet<crate::state::TxId>,
  3235	        > = std::collections::BTreeMap::new();
  3236	
  3237	        let writer_r = match self.ledger_writer.read() {
  3238	            Ok(g) => g,
  3239	            Err(_) => return edges,
  3240	        };
  3241	        let cas_r = match self.cas.read() {
  3242	            Ok(g) => g,
  3243	            Err(_) => return edges,
  3244	        };
  3245	
  3246	        let n = writer_r.len();
  3247	        for t in 1..=n {
  3248	            let entry = match writer_r.read_at(t) {
  3249	                Ok(e) => e,
  3250	                Err(_) => continue,
  3251	            };
  3252	            // Only Work entries carry parent_tx via ProposalTelemetry.
  3253	            if entry.tx_kind != crate::bottom_white::ledger::transition_ledger::TxKind::Work {
  3254	                continue;
  3255	            }

exec
/bin/bash -lc "rg -n \"struct .*Snapshot|price_index|mask_set|tape_view|AgentVisibleProjection|snapshot\\(\" src -g '*.rs' | head -n 200" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/economy/monetary_invariant.rs:142:/// - TB-14 `compute_price_index` derived view (signal-not-truth per
src/economy/monetary_invariant.rs:144:///   the sum; legacy `price_index_t` field removed in TB-14 Atom 2)
src/bin/audit_dashboard.rs:51:    compute_price_index, NodeMarketEntry, TaskId, TxId,
src/bin/audit_dashboard.rs:136:    /// derived price-index view per `compute_price_index` over a synthetic
src/bin/audit_dashboard.rs:143:    price_index: BTreeMap<TxId, NodeMarketEntry>,
src/bin/audit_dashboard.rs:965:        price_index: price_index_from_exposures(&exposures_in_progress),
src/bin/audit_dashboard.rs:1153:/// vec and call the canonical `state::compute_price_index` over it.
src/bin/audit_dashboard.rs:1159:/// (WorkTx) and ChallengeShort (ChallengeTx) — exactly the inputs `compute_price_index`
src/bin/audit_dashboard.rs:1160:/// needs. By going through `compute_price_index` rather than re-implementing
src/bin/audit_dashboard.rs:1165:/// The `kind` field is irrelevant to `compute_price_index` (which reads only
src/bin/audit_dashboard.rs:1171:fn price_index_from_exposures(
src/bin/audit_dashboard.rs:1196:    compute_price_index(&econ)
src/bin/audit_dashboard.rs:1682:    s.push_str(&render_section_14(&r.price_index));
src/bin/audit_dashboard.rs:1758:    s.push_str("    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +\n");
src/bin/audit_dashboard.rs:1837:fn render_section_14(price_index: &BTreeMap<TxId, NodeMarketEntry>) -> String {
src/bin/audit_dashboard.rs:1851:    if price_index.is_empty() {
src/bin/audit_dashboard.rs:1869:    for (node_id, entry) in price_index.iter() {
src/bin/audit_dashboard.rs:2086:    fn sg_14_6_dashboard_empty_price_index_renders_explicit_empty_state() {
src/bottom_white/cas/schema.rs:77:    /// MUST NOT enter `AgentVisibleProjection`.
src/state/q_state.rs:113:// AgentVisibleProjection — Inv 10 Goodhart shield (CO P2.7 visibility runtime).
src/state/q_state.rs:121:/// TB-14 Atom 3 (FC2-N28; architect §5.5 + charter §3 Atom 3): `mask_set`
src/state/q_state.rs:127:/// `compute_mask_set` in `src/state/price_index.rs`. `#[serde(default)]`
src/state/q_state.rs:131:pub struct AgentVisibleProjection {
src/state/q_state.rs:134:    pub mask_set: BTreeSet<TxId>,
src/state/q_state.rs:145:pub struct BudgetSnapshot {
src/state/q_state.rs:179:    // TB-14 Atom 2 (2026-05-03): `price_index_t: PriceIndex` removed.
src/state/q_state.rs:180:    // The TB-14 derived view is `compute_price_index(econ)` in
src/state/q_state.rs:181:    // `src/state/price_index.rs`; not canonical state per architect §5.1.
src/state/q_state.rs:249:    /// **NOT projected to `AgentVisibleProjection`** (CR-15.1 + halt-
src/state/q_state.rs:252:    /// Agents cannot retrieve the bytes through their `tape_view_t`
src/state/q_state.rs:730:// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
src/state/q_state.rs:731:// in `src/state/price_index.rs` (architect §5.1: "price is signal, not
src/state/q_state.rs:733:// `EconomicState.price_index_t` field also removed at architect §5.2.
src/state/q_state.rs:741:/// Sequencer-side index ONLY. NOT projected to `AgentVisibleProjection`
src/state/q_state.rs:743:/// through their `tape_view_t` (SG-15.2 + halt-trigger #4).
src/state/q_state.rs:767:    pub tape_view_t: AgentVisibleProjection,
src/state/q_state.rs:824:            "tape_view_t",
src/state/q_state.rs:847:        // with -price_index_t (legacy stub removed; TB-14 provides
src/state/q_state.rs:848:        // `compute_price_index` pure-fn derived view, not canonical state —
src/state/q_state.rs:854:        // NOT projected to AgentVisibleProjection per CR-15.1 + halt-trigger #1).
src/state/q_state.rs:869:        assert!(!obj.contains_key("price_index_t"), "TB-14 Atom 2: price_index_t MUST be removed");
src/sdk/actor.rs:29:/// epsilon-greedy exploration and `mask_set` read-view filter.
src/sdk/actor.rs:34:/// 1. Build the candidate set: every `node_id` in `price_index` whose
src/sdk/actor.rs:35:///    `price_yes` is `Some(_)` and which is NOT in `mask_set`
src/sdk/actor.rs:54:/// **Determinism**: deterministic given the same `(price_index, mask_set,
src/sdk/actor.rs:58:    price_index: &std::collections::BTreeMap<
src/sdk/actor.rs:62:    mask_set: &std::collections::BTreeSet<crate::state::TxId>,
src/sdk/actor.rs:66:    // Step 1: candidate set = {node | price_yes is Some AND node not in mask_set}
src/sdk/actor.rs:67:    let candidates: Vec<&crate::state::TxId> = price_index
src/sdk/actor.rs:70:            entry.price_yes.is_some() && !mask_set.contains(node_id)
src/sdk/actor.rs:94:        let entry = price_index.get(*cand).expect("candidate in index");
src/sdk/snapshot.rs:7:// The snapshot now carries integer-rational `price_index` + `mask_set`
src/sdk/snapshot.rs:8:// derived from canonical `EconomicState` via `state::compute_price_index`
src/sdk/snapshot.rs:9:// + `state::compute_mask_set`. Pricing is signal, not truth.
src/sdk/snapshot.rs:32:/// - `price_index` — derived `BTreeMap<TxId, NodeMarketEntry>` per
src/sdk/snapshot.rs:33:///   `compute_price_index(econ)`. Empty when bus runs sequencer-less OR
src/sdk/snapshot.rs:36:/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
src/sdk/snapshot.rs:42:///   `true` means the bus has a wired sequencer AND `q_snapshot()`
src/sdk/snapshot.rs:43:///   succeeded AND the canonical-graph builder ran. `price_index` /
src/sdk/snapshot.rs:44:///   `mask_set` may still be empty in this case — that is the "running
src/sdk/snapshot.rs:47:///   `q_snapshot()` failed (lock poisoned). In the `false` case,
src/sdk/snapshot.rs:48:///   `price_index` + `mask_set` are always empty by construction.
src/sdk/snapshot.rs:56:pub struct UniverseSnapshot {
src/sdk/snapshot.rs:58:    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
src/sdk/snapshot.rs:59:    pub mask_set: BTreeSet<TxId>,
src/sdk/snapshot.rs:73:        // price_index + mask_set; consumers (evaluator / dashboard) must
src/sdk/snapshot.rs:77:            price_index: BTreeMap::new(),
src/sdk/snapshot.rs:78:            mask_set: BTreeSet::new(),
src/sdk/snapshot.rs:83:        assert!(snap.price_index.is_empty());
src/sdk/snapshot.rs:84:        assert!(snap.mask_set.is_empty());
src/sdk/snapshot.rs:102:            "price_index": {},
src/sdk/snapshot.rs:103:            "mask_set": [],
src/sdk/snapshot.rs:118:        // Two snapshots, both with empty price_index + mask_set, are
src/sdk/snapshot.rs:122:            price_index: BTreeMap::new(),
src/sdk/snapshot.rs:123:            mask_set: BTreeSet::new(),
src/sdk/snapshot.rs:130:            price_index: BTreeMap::new(),
src/sdk/snapshot.rs:131:            mask_set: BTreeSet::new(),
src/sdk/snapshot.rs:136:        assert_eq!(unavailable.price_index, running_empty.price_index);
src/sdk/snapshot.rs:137:        assert_eq!(unavailable.mask_set, running_empty.mask_set);
src/state/mod.rs:19:/// derived-view price index. `compute_price_index(econ)` is the pure-fn
src/state/mod.rs:22:pub mod price_index;
src/state/mod.rs:25:    AgentId, AgentSwarmState, AgentVisibleProjection, BalancesIndex, BudgetSnapshot,
src/state/mod.rs:35:pub use price_index::{
src/state/mod.rs:36:    compute_mask_set, compute_price_index, BoltzmannMaskPolicy, CanonicalNodeGraph,
src/state/price_index.rs:33:/// shadow `kernel.tape` consumption in `compute_mask_set` exposed by Codex
src/state/price_index.rs:38:/// children are both canonical TxIds, so `compute_mask_set` operates in
src/state/price_index.rs:39:/// the same id namespace as `compute_price_index` (architect §3
src/state/price_index.rs:53:/// mode), the graph is empty `BTreeMap::new()`. `compute_mask_set` over
src/state/price_index.rs:65:/// constructed by `compute_price_index` (architect FR-14.1 + FR-14.2). All
src/state/price_index.rs:76:    /// for FC2-N28 `compute_mask_set` in Atom 3): cross-multiplication
src/state/price_index.rs:80:    /// to avoid division. Used by Atom 3's `compute_mask_set` to enforce
src/state/price_index.rs:82:    /// `false` on any zero denominator (`compute_price_index` never
src/state/price_index.rs:116:/// signal entry. **Derived view** populated by `compute_price_index`;
src/state/price_index.rs:143:// compute_price_index — pure fn over EconomicState
src/state/price_index.rs:164:pub fn compute_price_index(econ: &EconomicState) -> BTreeMap<TxId, NodeMarketEntry> {
src/state/price_index.rs:307:    /// pass it as an explicit input to `compute_mask_set` /
src/state/price_index.rs:423:// compute_mask_set — derive the parent-mask set from price_index +
src/state/price_index.rs:431:/// `mask_set: BTreeSet<TxId>` of parent-attempt-nodes whose visibility
src/state/price_index.rs:442:/// for each `(parent_id, parent_entry)` in `price_index`:
src/state/price_index.rs:463:/// read-view) lived in a different id namespace and produced empty mask_set
src/state/price_index.rs:466:pub fn compute_mask_set(
src/state/price_index.rs:470:    price_index: &BTreeMap<TxId, NodeMarketEntry>,
src/state/price_index.rs:486:    for (parent_id, parent_entry) in price_index.iter() {
src/state/price_index.rs:502:            let child_entry = match price_index.get(child_tx_id) {
src/state/price_index.rs:586:        let idx = compute_price_index(&econ);
src/state/price_index.rs:602:        let idx = compute_price_index(&econ);
src/state/price_index.rs:637:        let idx = compute_price_index(&econ);
src/state/price_index.rs:677:        let idx = compute_price_index(&econ);
src/state/price_index.rs:719:        let idx = compute_price_index(&econ);
src/state/price_index.rs:762:        let first = compute_price_index(&econ);
src/state/price_index.rs:765:                compute_price_index(&econ),
src/state/price_index.rs:809:        let idx = compute_price_index(&econ);
src/kernel.rs:14:// in the derived view `state::compute_price_index`; YES/NO claims live
src/bus.rs:36:/// derived view over `EconomicState` via `state::compute_price_index`; no
src/bus.rs:207:    /// (TB-12) and surface via `compute_price_index` derived view (TB-14).
src/bus.rs:329:        // (`state::compute_price_index`) populated by typed-tx admission via
src/bus.rs:482:    /// `price_index` + `mask_set` derived from canonical `EconomicState`
src/bus.rs:483:    /// via `state::compute_price_index` + `state::compute_mask_set`,
src/bus.rs:493:    /// `compute_price_index (pure derive)` → snapshot read-view →
src/bus.rs:497:    /// **Replay-deterministic** (Art.0.2): `compute_price_index` and
src/bus.rs:498:    /// `compute_mask_set` are pure over their inputs. The snapshot's
src/bus.rs:499:    /// `price_index` / `mask_set` are reproducible from any byte-equal
src/bus.rs:505:    /// price_index + mask_set are empty `BTreeMap` / `BTreeSet`. Callers
src/bus.rs:508:    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
src/bus.rs:516:        // — same namespace as `price_index` — so `compute_mask_set` can
src/bus.rs:523:        // states produce empty `price_index` + `mask_set`, but consumers
src/bus.rs:525:        let (price_index, mask_set, sequencer_wired) = match self.sequencer.as_ref() {
src/bus.rs:526:            Some(seq) => match seq.q_snapshot() {
src/bus.rs:528:                    let pi = crate::state::compute_price_index(&q.economic_state_t);
src/bus.rs:530:                    let ms = crate::state::compute_mask_set(
src/bus.rs:553:            price_index,
src/bus.rs:554:            mask_set,
src/bus.rs:704:    fn test_bus_snapshot() {
src/bus.rs:706:        // price_index: BTreeMap<TxId, NodeMarketEntry> + mask_set: BTreeSet<TxId>.
src/bus.rs:712:        let snap = bus.snapshot();
src/bus.rs:714:        assert!(snap.price_index.is_empty(), "no sequencer → empty price_index");
src/bus.rs:715:        assert!(snap.mask_set.is_empty(), "no sequencer → empty mask_set");
src/runtime/adapter.rs:398:        if let Ok(q) = sequencer.q_snapshot() {
src/runtime/adapter.rs:440:        if let Ok(q) = sequencer.q_snapshot() {
src/runtime/adapter.rs:531:    let q = match sequencer.q_snapshot() {
src/runtime/adapter.rs:582:        if let Ok(q_now) = sequencer.q_snapshot() {
src/runtime/autopsy_capsule.rs:18://!   NEVER enters `AgentVisibleProjection` (CR-15.1 + SG-15.2).
src/runtime/autopsy_capsule.rs:143:    /// Audit-only access. NEVER enters `AgentVisibleProjection`.
src/runtime/audit_assertions.rs:1372:pub fn assert_26_price_index_is_view_only(_t: &LoadedTape) -> AssertionResult {
src/runtime/audit_assertions.rs:1375:    // EconomicState struct has no `price_index_t` field; therefore
src/runtime/audit_assertions.rs:1378:    AssertionResult::pass(26, "price_index_is_view_only", AssertionLayer::E)
src/runtime/audit_assertions.rs:1454:    let proj_bytes = canonical_encode(&q.tape_view_t).unwrap_or_default();
src/runtime/audit_assertions.rs:1480:                    "AgentVisibleProjection serialization contains a private_detail_cid byte run"
src/runtime/audit_assertions.rs:1955:    r.push(assert_26_price_index_is_view_only(&tape));
src/state/sequencer.rs:1369:            // Cids are NOT projected to AgentVisibleProjection.
src/state/sequencer.rs:3186:    pub fn q_snapshot(&self) -> Result<QState, ApplyError> {
src/state/sequencer.rs:3464:        let pre = seq.q_snapshot().expect("q_snapshot").state_root_t;
src/state/sequencer.rs:3490:        let post = seq.q_snapshot().expect("q_snapshot").state_root_t;

exec
/bin/bash -lc "nl -ba src/state/q_state.rs | sed -n '720,780p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   720	            challenger: AgentId::default(),
   721	            bond: MicroCoin::zero(),
   722	            opened_at_round: 0,
   723	            target_work_tx: TxId::default(),
   724	            status: ChallengeStatus::Open,
   725	        }
   726	    }
   727	}
   728	
   729	// TB-14 Atom 2 (2026-05-03): legacy `pub struct PriceIndex(BTreeMap<TxId,
   730	// MicroCoin>)` removed. The TB-14 derived view is `compute_price_index`
   731	// in `src/state/price_index.rs` (architect §5.1: "price is signal, not
   732	// truth"; charter §7 auto-resolution A: "no second source-of-truth").
   733	// `EconomicState.price_index_t` field also removed at architect §5.2.
   734	
   735	/// TRACE_MATRIX TB-15 Atom 3 (architect §6.2 + DECISION_LAMARCKIAN §1.1):
   736	/// per-event autopsy index. `BTreeMap<EventId, Vec<Cid>>` — one Cid per
   737	/// `AgentAutopsyCapsule` emitted on a loss event (TB-15 v0 sole trigger
   738	/// = TaskBankruptcyTx; per-staker capsule emission). **Cids only** —
   739	/// the capsule bytes live in CAS behind `ObjectType::AgentAutopsyCapsule`.
   740	///
   741	/// Sequencer-side index ONLY. NOT projected to `AgentVisibleProjection`
   742	/// (CR-15.1 + halt-trigger #1). Other agents cannot retrieve the bytes
   743	/// through their `tape_view_t` (SG-15.2 + halt-trigger #4).
   744	///
   745	/// BTreeMap iteration order is sorted-by-`EventId` → deterministic →
   746	/// replay-safe.
   747	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   748	pub struct AutopsyIndex(pub BTreeMap<crate::state::typed_tx::EventId, Vec<crate::bottom_white::cas::schema::Cid>>);
   749	
   750	// ────────────────────────────────────────────────────────────────────────────
   751	// QState — § 1.1 verbatim, 9 fields.
   752	// ────────────────────────────────────────────────────────────────────────────
   753	
   754	/// TRACE_MATRIX § 1.1 — system state Q_t. 9 fields per WP § 4 + economic § 2 amendment.
   755	///
   756	/// Reconstructibility: every field is derivable from L4 transition ledger replay
   757	/// (Art IV Boot 公理).
   758	#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
   759	pub struct QState {
   760	    /// Agent swarm sub-state (tape head per agent + per-agent reputation snapshots).
   761	    pub q_t: AgentSwarmState,
   762	    /// Current ChainTape head pointer = git commit SHA in Path B substrate.
   763	    pub head_t: NodeId,
   764	    /// Materialized state Merkle root (git tree root in Path B).
   765	    pub state_root_t: Hash,
   766	    /// Agent-visible projection of tape filtered by per-agent visibility policy.
   767	    pub tape_view_t: AgentVisibleProjection,
   768	    /// L4 Transition Ledger root (Merkle root of all accepted tx so far).
   769	    pub ledger_root_t: Hash,
   770	    /// L1 Predicate Registry root.
   771	    pub predicate_registry_root_t: Hash,
   772	    /// L2 Tool Registry root.
   773	    pub tool_registry_root_t: Hash,
   774	    /// Economic state (WP § 2 amendment, 9 sub-fields).
   775	    pub economic_state_t: EconomicState,
   776	    /// Global budget snapshot.
   777	    pub budget_state_t: BudgetSnapshot,
   778	}
   779	
   780	impl QState {

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '470,560p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   470	                sorted.into_iter()
   471	                    .map(|(lbl, c)| format!("{}({})", lbl, c))
   472	                    .take(max)
   473	                    .collect()
   474	            }
   475	        }
   476	    }
   477	
   478	    /// Get a snapshot of the universe for agents to read.
   479	    ///
   480	    /// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 +
   481	    /// charter §3 Atom 6): the snapshot now carries the integer-rational
   482	    /// `price_index` + `mask_set` derived from canonical `EconomicState`
   483	    /// via `state::compute_price_index` + `state::compute_mask_set`,
   484	    /// replacing the legacy decimal-float `markets: HashMap<_, MarketSnapshot>`
   485	    /// CPMM read-view excised together with `src/prediction_market.rs`.
   486	    ///
   487	    /// **Halt-trigger #2 spirit preserved**: bus.rs imports TB-14 types
   488	    /// (this is the legitimate broadcast point per kickoff doc), but the
   489	    /// L4/L4.E classification path in `Sequencer::dispatch_transition`
   490	    /// remains free of TB-14 imports — verified by halt-trigger #2's
   491	    /// `use`-statement scan over `src/state/sequencer.rs`. The price
   492	    /// signal flows: `EconomicState (canonical)` →
   493	    /// `compute_price_index (pure derive)` → snapshot read-view →
   494	    /// scheduler / dashboard / agent prompt. It NEVER flows back into
   495	    /// `dispatch_transition`.
   496	    ///
   497	    /// **Replay-deterministic** (Art.0.2): `compute_price_index` and
   498	    /// `compute_mask_set` are pure over their inputs. The snapshot's
   499	    /// `price_index` / `mask_set` are reproducible from any byte-equal
   500	    /// `EconomicState` + `Tape` + `BoltzmannMaskPolicy` without re-running
   501	    /// the run.
   502	    ///
   503	    /// **Sequencer-optional**: when the bus runs in legacy ledger-only
   504	    /// mode (`sequencer == None`, e.g. in WAL-only smoke tests), the
   505	    /// price_index + mask_set are empty `BTreeMap` / `BTreeSet`. Callers
   506	    /// (evaluator, dashboard) treat empty as "no signal yet" — they MUST
   507	    /// NOT crash on empty.
   508	    pub fn snapshot(&self) -> crate::sdk::snapshot::UniverseSnapshot {
   509	        let policy = crate::state::BoltzmannMaskPolicy::from_env();
   510	
   511	        // TB-14 Atom 6 B′ step 4 (architect ruling 2026-05-03 §3+§4): the
   512	        // canonical-node-graph is built from L4 accepted WorkTx +
   513	        // CAS-resident ProposalTelemetry.parent_tx via
   514	        // `Sequencer::compute_canonical_edges_at_head`. The resulting
   515	        // `CanonicalNodeGraph` is keyed by canonical accepted WorkTx.tx_id
   516	        // — same namespace as `price_index` — so `compute_mask_set` can
   517	        // join them correctly (which the pre-B′ shadow `kernel.tape`
   518	        // version could NOT, per Codex R1 ship audit VETO).
   519	        // TB-14 Atom 6 B′ R2 closure (Gemini R2 Q11 architectural-clarity
   520	        // CHALLENGE): `sequencer_wired` disambiguates "sequencer
   521	        // unavailable" (legacy WAL-only mode OR q_snapshot failed) from
   522	        // "sequencer running but no canonical positions yet" — both
   523	        // states produce empty `price_index` + `mask_set`, but consumers
   524	        // that care can read this flag to distinguish.
   525	        let (price_index, mask_set, sequencer_wired) = match self.sequencer.as_ref() {
   526	            Some(seq) => match seq.q_snapshot() {
   527	                Ok(q) => {
   528	                    let pi = crate::state::compute_price_index(&q.economic_state_t);
   529	                    let edges = seq.compute_canonical_edges_at_head();
   530	                    let ms = crate::state::compute_mask_set(
   531	                        &q.economic_state_t,
   532	                        &edges,
   533	                        &policy,
   534	                        &pi,
   535	                    );
   536	                    (pi, ms, true)
   537	                }
   538	                Err(_) => (
   539	                    std::collections::BTreeMap::new(),
   540	                    std::collections::BTreeSet::new(),
   541	                    false,
   542	                ),
   543	            },
   544	            None => (
   545	                std::collections::BTreeMap::new(),
   546	                std::collections::BTreeSet::new(),
   547	                false,
   548	            ),
   549	        };
   550	
   551	        crate::sdk::snapshot::UniverseSnapshot {
   552	            tape: self.kernel.tape.clone(),
   553	            price_index,
   554	            mask_set,
   555	            sequencer_wired,
   556	            generation: self.generation,
   557	            tx_count: self.tx_count,
   558	        }
   559	    }
   560	}

exec
/bin/bash -lc "nl -ba src/sdk/snapshot.rs | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	// Tier 2: Immutable universe snapshot — agents read, never mutate
     2	// Constitutional basis: Art. III.3 (decorrelation via independent snapshots)
     3	//
     4	// TB-14 Atom 6 (2026-05-03 closing OBS_TB_12_LEGACY_CPMM_QUARANTINE):
     5	// Legacy decimal-float `MarketSnapshot` + `UniverseSnapshot.markets`
     6	// HashMap CPMM read-view was excised together with `prediction_market.rs`.
     7	// The snapshot now carries integer-rational `price_index` + `mask_set`
     8	// derived from canonical `EconomicState` via `state::compute_price_index`
     9	// + `state::compute_mask_set`. Pricing is signal, not truth.
    10	//
    11	// Dead post-TB-9-collapse `balances: HashMap<String, f64>` and
    12	// `portfolios: HashMap<String, HashMap<NodeId, (f64, f64, f64)>>` were
    13	// also retired in this atom — bus.snapshot already populated both with
    14	// empty HashMaps (no live values flowed through them). Removal is purely
    15	// additive cleanup that closes the f64 surface in this file under the
    16	// G-14.11 "no f64 in TB-14 module surface" ship gate.
    17	
    18	use crate::ledger::Tape;
    19	use crate::state::{NodeMarketEntry, TxId};
    20	use serde::{Deserialize, Serialize};
    21	use std::collections::{BTreeMap, BTreeSet};
    22	
    23	/// Complete frozen state of the universe.
    24	/// Agents receive this as read-only input — they cannot mutate it.
    25	/// Art. III.3: each agent sees the same snapshot, maintaining decorrelation.
    26	///
    27	/// TRACE_MATRIX TB-14 Atom 6 (FC2-N28 + FC3-N42; architect §5.1 + charter
    28	/// §3 Atom 6): the snapshot's price-signal surface.
    29	///
    30	/// Field semantics:
    31	/// - `tape` — the current `Tape` (DAG of attempt nodes); read-only mirror.
    32	/// - `price_index` — derived `BTreeMap<TxId, NodeMarketEntry>` per
    33	///   `compute_price_index(econ)`. Empty when bus runs sequencer-less OR
    34	///   when sequencer is wired but no canonical positions have accumulated.
    35	///   The two cases are distinguished by the `sequencer_wired` field below.
    36	/// - `mask_set` — derived `BTreeSet<TxId>` per `compute_mask_set(...)`.
    37	///   Empty when bus runs sequencer-less OR when canonical edges are empty.
    38	///   Mask is read-view only — masked parents remain in canonical state
    39	///   (CR-14.3 / SG-14.3 / halt-trigger #3).
    40	/// - `sequencer_wired` — TB-14 Atom 6 B′ R2 closure (Gemini R2 Q11
    41	///   architectural-clarity CHALLENGE): explicit two-state disambiguator.
    42	///   `true` means the bus has a wired sequencer AND `q_snapshot()`
    43	///   succeeded AND the canonical-graph builder ran. `price_index` /
    44	///   `mask_set` may still be empty in this case — that is the "running
    45	///   but no canonical positions yet" state. `false` means the bus runs
    46	///   in legacy ledger-only mode (`sequencer == None`) OR the sequencer's
    47	///   `q_snapshot()` failed (lock poisoned). In the `false` case,
    48	///   `price_index` + `mask_set` are always empty by construction.
    49	///   Consumers that need to distinguish "no signal possible" from "no
    50	///   signal yet" read this field; consumers that don't care continue to
    51	///   treat empty maps uniformly (no breaking change for the existing
    52	///   call surface). `#[serde(default)]` for backward-compat with
    53	///   pre-B′-R2 chain snapshots.
    54	/// - `generation`, `tx_count` — bus-level counters, unchanged from TB-3.
    55	#[derive(Debug, Clone, Serialize, Deserialize)]
    56	pub struct UniverseSnapshot {
    57	    pub tape: Tape,
    58	    pub price_index: BTreeMap<TxId, NodeMarketEntry>,
    59	    pub mask_set: BTreeSet<TxId>,
    60	    #[serde(default)]
    61	    pub sequencer_wired: bool,
    62	    pub generation: u32,
    63	    pub tx_count: u64,
    64	}
    65	
    66	#[cfg(test)]
    67	mod tests {
    68	    use super::*;
    69	
    70	    #[test]
    71	    fn test_snapshot_default_empty_signal_surface() {
    72	        // TB-14 Atom 6: a freshly-constructed snapshot has empty
    73	        // price_index + mask_set; consumers (evaluator / dashboard) must
    74	        // tolerate this as "no signal yet" without crashing.
    75	        let snap = UniverseSnapshot {
    76	            tape: Tape::new(),
    77	            price_index: BTreeMap::new(),
    78	            mask_set: BTreeSet::new(),
    79	            sequencer_wired: false,
    80	            generation: 0,
    81	            tx_count: 0,
    82	        };
    83	        assert!(snap.price_index.is_empty());
    84	        assert!(snap.mask_set.is_empty());
    85	        assert!(!snap.sequencer_wired);
    86	        assert_eq!(snap.generation, 0);
    87	        assert_eq!(snap.tx_count, 0);
    88	    }
    89	
    90	    /// TB-14 Atom 6 B′ R2 closure (Gemini R2 Q11): the `sequencer_wired`
    91	    /// field disambiguates "sequencer unavailable" from "sequencer running
    92	    /// but no canonical positions yet". `serde(default)` ensures
    93	    /// pre-B′-R2 serialized snapshots round-trip without explicit field
    94	    /// values (default = false; consistent with the legacy ledger-only
    95	    /// mode that produced those snapshots).
    96	    #[test]
    97	    fn test_snapshot_sequencer_wired_serde_default_false() {
    98	        // Construct a JSON payload WITHOUT the `sequencer_wired` field
    99	        // (mirrors pre-B′-R2 serialized snapshots).
   100	        let json = r#"{
   101	            "tape": {"nodes": {}, "reverse_citations": {}, "time_arrow": []},
   102	            "price_index": {},
   103	            "mask_set": [],
   104	            "generation": 0,
   105	            "tx_count": 0
   106	        }"#;
   107	        let snap: UniverseSnapshot =
   108	            serde_json::from_str(json).expect("legacy JSON deserialize");
   109	        assert!(
   110	            !snap.sequencer_wired,
   111	            "Q11 closure: serde(default) on sequencer_wired must produce \
   112	             false for legacy snapshots without the field"
   113	        );
   114	    }
   115	
   116	    #[test]
   117	    fn test_snapshot_sequencer_wired_distinguishes_two_states() {
   118	        // Two snapshots, both with empty price_index + mask_set, are
   119	        // semantically distinct via sequencer_wired.
   120	        let unavailable = UniverseSnapshot {
   121	            tape: Tape::new(),
   122	            price_index: BTreeMap::new(),
   123	            mask_set: BTreeSet::new(),
   124	            sequencer_wired: false,
   125	            generation: 0,
   126	            tx_count: 0,
   127	        };
   128	        let running_empty = UniverseSnapshot {
   129	            tape: Tape::new(),
   130	            price_index: BTreeMap::new(),
   131	            mask_set: BTreeSet::new(),
   132	            sequencer_wired: true,
   133	            generation: 0,
   134	            tx_count: 0,
   135	        };
   136	        assert_eq!(unavailable.price_index, running_empty.price_index);
   137	        assert_eq!(unavailable.mask_set, running_empty.mask_set);
   138	        assert_ne!(
   139	            unavailable.sequencer_wired, running_empty.sequencer_wired,
   140	            "Q11 closure: same empty maps + opposite sequencer_wired = \
   141	             two distinct semantic states (sequencer unavailable vs \
   142	             running-but-empty); consumers that need to disambiguate \
   143	             read this field"
   144	        );
   145	    }
   146	}

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1530,1608p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1530	    }
  1531	    AssertionResult::pass(29, "autopsy_private_detail_creator_is_system", AssertionLayer::F)
  1532	}
  1533	
  1534	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1535	pub fn assert_30_typical_error_summary_no_private_detail(t: &LoadedTape) -> AssertionResult {
  1536	    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
  1537	    let q = match &t.replayed_q {
  1538	        Some(q) => q,
  1539	        None => {
  1540	            return AssertionResult::skipped(
  1541	                30,
  1542	                "typical_error_summary_no_private_detail",
  1543	                AssertionLayer::F,
  1544	                "no replayed_q".into(),
  1545	            );
  1546	        }
  1547	    };
  1548	    // Collect all autopsy capsules from CAS for clustering.
  1549	    let mut capsules: Vec<crate::runtime::autopsy_capsule::AgentAutopsyCapsule> = Vec::new();
  1550	    let mut private_cids: BTreeSet<[u8; 32]> = BTreeSet::new();
  1551	    for (_, cids) in &q.economic_state_t.agent_autopsies_t.0 {
  1552	        for cid in cids {
  1553	            let bytes = match t.cas.get(cid) {
  1554	                Ok(b) => b,
  1555	                Err(_) => continue,
  1556	            };
  1557	            let autopsy: crate::runtime::autopsy_capsule::AgentAutopsyCapsule =
  1558	                match canonical_decode(&bytes) {
  1559	                    Ok(a) => a,
  1560	                    Err(_) => match serde_json::from_slice(&bytes) {
  1561	                        Ok(a) => a,
  1562	                        Err(_) => continue,
  1563	                    },
  1564	                };
  1565	            private_cids.insert(autopsy.private_detail_cid.0);
  1566	            capsules.push(autopsy);
  1567	        }
  1568	    }
  1569	    let summaries =
  1570	        crate::runtime::autopsy_capsule::cluster_autopsies(&capsules, 3);
  1571	    let json = serde_json::to_string(&summaries).unwrap_or_default();
  1572	    let canonical = canonical_encode(&summaries).unwrap_or_default();
  1573	    for run in &private_cids {
  1574	        for window in canonical.windows(32) {
  1575	            if window == run {
  1576	                return AssertionResult::halt(
  1577	                    30,
  1578	                    "typical_error_summary_no_private_detail",
  1579	                    AssertionLayer::F,
  1580	                    "canonical_encode of TypicalErrorSummary contains private_detail_cid run"
  1581	                        .into(),
  1582	                );
  1583	            }
  1584	        }
  1585	        // also check JSON array form
  1586	        let n = run[0] as u32;
  1587	        let same = run.iter().all(|b| (*b as u32) == n);
  1588	        if same {
  1589	            let mut form = String::with_capacity(160);
  1590	            form.push('[');
  1591	            for i in 0..32 {
  1592	                if i > 0 { form.push(','); }
  1593	                form.push_str(&n.to_string());
  1594	            }
  1595	            form.push(']');
  1596	            if json.contains(&form) {
  1597	                return AssertionResult::halt(
  1598	                    30,
  1599	                    "typical_error_summary_no_private_detail",
  1600	                    AssertionLayer::F,
  1601	                    "JSON of TypicalErrorSummary contains canonical Cid array form".into(),
  1602	                );
  1603	            }
  1604	        }
  1605	    }
  1606	    AssertionResult::pass(30, "typical_error_summary_no_private_detail", AssertionLayer::F)
  1607	}
  1608	

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '80,170p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    80	                i += 1;
    81	                out = Some(argv.get(i).ok_or("missing value after --out")?.into());
    82	            }
    83	            "--help" | "-h" => return Err("--help requested".into()),
    84	            other => return Err(format!("unknown arg: {other}")),
    85	        }
    86	        i += 1;
    87	    }
    88	    Ok(Args {
    89	        repo: repo.ok_or("--repo required")?,
    90	        cas: cas.ok_or("--cas required")?,
    91	        json,
    92	        out,
    93	    })
    94	}
    95	
    96	#[derive(Debug, serde::Serialize)]
    97	struct DashboardReport {
    98	    run_id: String,
    99	    epoch: u64,
   100	    chain: ChainStats,
   101	    indicators: IndicatorStatus,
   102	    run_facts: ChainDerivedRunFacts,
   103	    per_agent: BTreeMap<String, AgentActivity>,
   104	    proposal_flow: Vec<ProposalFlowEntry>,
   105	    branch_lineage: Vec<BranchEdge>,
   106	    /// TB-7.7 D6: golden path steps (only populated when chain_oracle_verified=true).
   107	    golden_path: Vec<GoldenPathStep>,
   108	    cross_checks: CrossCheck,
   109	    /// TB-8 Atom 6: per-claim audit-row (Open / Finalized) with payout amount.
   110	    /// Populated by walking L4 entries and matching VerifyTx{Confirm} → claim
   111	    /// derivation against any subsequent FinalizeRewardTx with the same claim_id.
   112	    claims: Vec<ClaimAuditRow>,
   113	    /// TB-10 Atom 4: per-user-task audit-row. Populated by filtering TaskOpen
   114	    /// entries whose sponsor_agent.0 starts with "Agent_user_" (lean_market
   115	    /// CLI convention) and cross-referencing with claims for payout status.
   116	    /// The aggregate sum of bounty_micro across all rows is the user's total
   117	    /// committed liquidity at this snapshot.
   118	    user_tasks: Vec<UserTaskRow>,
   119	    /// TB-11 Atom 5 (architect §6.2): exhausted runs from TerminalSummaryTx
   120	    /// L4 entries (architect's RunExhaustedTx role).
   121	    exhausted_runs: Vec<ExhaustedRunRow>,
   122	    /// TB-11 Atom 5 (architect §6.2): expired tasks from TaskExpireTx L4
   123	    /// entries (capital release path).
   124	    expired_tasks: Vec<ExpiredTaskRow>,
   125	    /// TB-11 Atom 5 (architect §6.2): bankrupt tasks from TaskBankruptcyTx
   126	    /// L4 entries (death certificate for future TB-12 NodeMarket Short / NO
   127	    /// settlement anchor).
   128	    bankrupt_tasks: Vec<BankruptTaskRow>,
   129	    /// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4): exposure
   130	    /// records derived from accepted WorkTx (FirstLong) + ChallengeTx
   131	    /// (ChallengeShort) L4 entries. Architect §10: IMMUTABLE EXPOSURE
   132	    /// RECORD, NOT active position balance. Label discipline: "Exposure
   133	    /// records", NOT "Open market balances".
   134	    exposures: Vec<ExposureRecordRow>,
   135	    /// TB-14 Atom 6 (architect 2026-05-03 ruling §5.1 + §5.5 SG-14.6):
   136	    /// derived price-index view per `compute_price_index` over a synthetic
   137	    /// `EconomicState` rebuilt from `exposures`. Renders in §14 with the
   138	    /// `PRICE IS SIGNAL, NOT TRUTH` banner per architect §5.1 ("Price is
   139	    /// signal, not truth") and SG-14.6 unit test discipline. NEVER shown
   140	    /// as decimal — every price is rendered as `numerator/denominator`
   141	    /// integer-rational pair (charter §5 forbidden list: no f64 / no
   142	    /// decimal float in TB-14 module surface).
   143	    price_index: BTreeMap<TxId, NodeMarketEntry>,
   144	    /// TB-15 Atom 6 (architect §6.5 SG-15.6): per-event autopsy Cid
   145	    /// counts derived from on-chain `EconomicState.agent_autopsies_t`
   146	    /// at snapshot time. Empty Vec when no TaskBankruptcyTx has fired.
   147	    /// Architect §6.4 privacy: dashboard surfaces COUNTS + COMPRESSED
   148	    /// `public_summary` strings only — never `private_detail_cid` bytes.
   149	    autopsy_event_counts: Vec<(String /*event_id*/, u32 /*cid_count*/)>,
   150	    /// TB-15 Atom 6: latest Markov capsule pointer (Cid hex from
   151	    /// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` if present;
   152	    /// None when no Markov capsule has been generated). FR-15.4 next-
   153	    /// session bootstrap surface.
   154	    latest_markov_capsule_cid_hex: Option<String>,
   155	    /// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 +
   156	    /// §7.5 SG-16.8): true when ANY agent_id encountered during the L4
   157	    /// walk OR in the agent_pubkeys.json manifest matches a sandbox-only
   158	    /// prefix (Agent_solver_/Agent_verifier_/Agent_user_/tb7-7-sponsor/
   159	    /// tb16-). Drives §16 banner; prevents dashboard readers from
   160	    /// interpreting sandbox prices/positions as production signals.
   161	    sandbox_run: bool,
   162	}
   163	
   164	/// TB-12 Atom 4 (architect 2026-05-03 ruling §8 Atom 4) — per-NodePosition
   165	/// audit row for §13. Architect's label discipline: "Exposure records"
   166	/// (NOT "Open market balances" — TB-12 is exposure index, not trading
   167	/// market; live share balances land in TB-13 CompleteSet).
   168	#[derive(Debug, serde::Serialize)]
   169	struct ExposureRecordRow {
   170	    position_id: String,

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '980,1035p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   980	
   981	/// TRACE_MATRIX TB-15 Atom 6 (FR-15.4 + SG-15.6): best-effort read of
   982	/// `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt` from the
   983	/// repo-root convention path. Returns None when the file is absent
   984	/// (e.g. fresh repo without TB-15 generation yet) or unreadable.
   985	/// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 + §7.5
   986	/// SG-16.8): scan all L4 entries + agent_pubkeys manifest for any
   987	/// agent_id matching a sandbox-only prefix.
   988	fn detect_sandbox_run(
   989	    entries: &[LedgerEntry],
   990	    cas: &CasStore,
   991	    manifest: Option<&AgentPubkeyManifest>,
   992	) -> bool {
   993	    let is_sandbox = |id: &str| -> bool {
   994	        id.starts_with("Agent_solver_")
   995	            || id.starts_with("Agent_verifier_")
   996	            || id.starts_with("Agent_user_")
   997	            || id == "tb7-7-sponsor"
   998	            || id.starts_with("tb16-")
   999	    };
  1000	    if let Some(m) = manifest {
  1001	        for k in m.agents.keys() {
  1002	            if is_sandbox(k) {
  1003	                return true;
  1004	            }
  1005	        }
  1006	    }
  1007	    for entry in entries {
  1008	        let payload = match cas.get(&entry.tx_payload_cid) {
  1009	            Ok(b) => b,
  1010	            Err(_) => continue,
  1011	        };
  1012	        let typed: TypedTx = match canonical_decode(&payload) {
  1013	            Ok(t) => t,
  1014	            Err(_) => continue,
  1015	        };
  1016	        let id = match &typed {
  1017	            TypedTx::Work(w) => w.agent_id.0.clone(),
  1018	            TypedTx::Verify(v) => v.verifier_agent.0.clone(),
  1019	            TypedTx::Challenge(c) => c.challenger_agent.0.clone(),
  1020	            TypedTx::TaskOpen(t) => t.sponsor_agent.0.clone(),
  1021	            TypedTx::EscrowLock(e) => e.sponsor_agent.0.clone(),
  1022	            TypedTx::CompleteSetMint(m) => m.owner.0.clone(),
  1023	            TypedTx::CompleteSetRedeem(r) => r.owner.0.clone(),
  1024	            TypedTx::MarketSeed(s) => s.provider.0.clone(),
  1025	            _ => continue,
  1026	        };
  1027	        if is_sandbox(&id) {
  1028	            return true;
  1029	        }
  1030	    }
  1031	    false
  1032	}
  1033	
  1034	fn read_latest_markov_pointer() -> Option<String> {
  1035	    let p = std::path::Path::new("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt");

exec
/bin/bash -lc "nl -ba src/bin/audit_dashboard.rs | sed -n '1690,1724p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1690	    // §16 TB-16 Sandbox banner (architect 2026-05-03 §7.4 CR-16.7 +
  1691	    // §7.5 SG-16.8). Rendered when ANY agent_id surfaced in the report
  1692	    // matches a sandbox-only prefix (Agent_solver_*, Agent_verifier_*,
  1693	    // Agent_user_*, tb7-7-sponsor, tb16-*). Scans per_agent +
  1694	    // claims.solver/sponsor + user_tasks.sponsor + exhausted_runs.solver +
  1695	    // exposures.owner so a sponsor-only chain (TaskOpen + EscrowLock +
  1696	    // TerminalSummary, no Work) still trips the banner.
  1697	    s.push_str(&render_section_16(r));
  1698	
  1699	    s
  1700	}
  1701	
  1702	/// TRACE_MATRIX FC2-N33 (TB-16 Atom 4; architect §7.4 CR-16.7 + §7.5
  1703	/// SG-16.8): SANDBOX banner render. Source-fence — emit when
  1704	/// `report.sandbox_run` is true (computed in build_report by scanning
  1705	/// the L4 walk + agent_pubkeys manifest); otherwise no banner.
  1706	fn render_section_16(r: &DashboardReport) -> String {
  1707	    if !r.sandbox_run {
  1708	        return String::new();
  1709	    }
  1710	    let mut s = String::new();
  1711	    s.push('\n');
  1712	    s.push_str("§16 TB-16 SANDBOX BANNER (architect 2026-05-03 §7.4 CR-16.7 + §7.5 SG-16.8)\n");
  1713	    s.push_str("==========================================================================\n");
  1714	    s.push_str("  ⚠ SANDBOX-RUN — NOT PRODUCTION — NO REAL FUNDS\n");
  1715	    s.push_str("    Agent IDs are sandbox-prefixed (Agent_solver_/Agent_verifier_/\n");
  1716	    s.push_str("    Agent_user_/tb7-7-sponsor/tb16-). Total Coin sourced from\n");
  1717	    s.push_str("    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC\n");
  1718	    s.push_str("    on_init mint; assert_no_post_init_mint enforced).\n");
  1719	    s.push_str("\n");
  1720	    s.push_str("    Architect §7.6 forbidden:\n");
  1721	    s.push_str("      - No public chain.\n");
  1722	    s.push_str("      - No real-money market.\n");
  1723	    s.push_str("      - No external domain (Lean only; no medical/legal/financial).\n");
  1724	    s.push_str("      - No production user funds.\n");

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '700,790p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   700	}
   701	
   702	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   703	pub fn assert_08_system_tx_signatures_verify(t: &LoadedTape) -> AssertionResult {
   704	    use crate::bottom_white::ledger::system_keypair::{
   705	        verify_system_signature, CanonicalMessage,
   706	    };
   707	    let mut count = 0u32;
   708	    for (i, e) in t.entries.iter().enumerate() {
   709	        if !is_system_tx_kind(e.tx_kind) {
   710	            continue;
   711	        }
   712	        let signing_digest = e.to_signing_payload().canonical_digest();
   713	        let canonical_msg = CanonicalMessage::LedgerEntrySigning(signing_digest.0);
   714	        if !verify_system_signature(&e.system_signature, &canonical_msg, e.epoch, &t.pinned) {
   715	            return AssertionResult::halt(
   716	                8,
   717	                "system_tx_signatures_verify",
   718	                AssertionLayer::B,
   719	                format!("bad system_signature at index {i} ({:?})", e.tx_kind),
   720	            );
   721	        }
   722	        count += 1;
   723	    }
   724	    if count == 0 {
   725	        AssertionResult::skipped(
   726	            8,
   727	            "system_tx_signatures_verify",
   728	            AssertionLayer::B,
   729	            "no system tx in tape".into(),
   730	        )
   731	    } else {
   732	        AssertionResult::pass(8, "system_tx_signatures_verify", AssertionLayer::B)
   733	    }
   734	}
   735	
   736	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   737	pub fn assert_09_agent_tx_signatures_verify(t: &LoadedTape) -> AssertionResult {
   738	    let mut count = 0u32;
   739	    for (i, e) in t.entries.iter().enumerate() {
   740	        if !is_agent_tx_kind(e.tx_kind) {
   741	            continue;
   742	        }
   743	        // Resolve payload from CAS and decode.
   744	        let payload = match t.cas.get(&e.tx_payload_cid) {
   745	            Ok(b) => b,
   746	            Err(e2) => {
   747	                return AssertionResult::halt(
   748	                    9,
   749	                    "agent_tx_signatures_verify",
   750	                    AssertionLayer::B,
   751	                    format!("CAS missing for agent tx at index {i}: {e2}"),
   752	                );
   753	            }
   754	        };
   755	        let typed: TypedTx = match canonical_decode(&payload) {
   756	            Ok(t) => t,
   757	            Err(e2) => {
   758	                return AssertionResult::halt(
   759	                    9,
   760	                    "agent_tx_signatures_verify",
   761	                    AssertionLayer::B,
   762	                    format!("decode at index {i}: {e2}"),
   763	                );
   764	            }
   765	        };
   766	        // Currently, agent signatures are validated end-to-end inside
   767	        // `replay_full_transition` (sequencer dispatch arm rejects on
   768	        // bad signature). If replay succeeded (or failed for a non-
   769	        // signature reason), we treat the structural verification as
   770	        // passing for the layer-B count and surface deeper checks via
   771	        // the dispatch path.
   772	        let _ = typed;
   773	        count += 1;
   774	    }
   775	    if count == 0 {
   776	        AssertionResult::skipped(
   777	            9,
   778	            "agent_tx_signatures_verify",
   779	            AssertionLayer::B,
   780	            "no agent tx in tape".into(),
   781	        )
   782	    } else {
   783	        AssertionResult::pass(9, "agent_tx_signatures_verify", AssertionLayer::B)
   784	    }
   785	}
   786	
   787	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   788	pub fn assert_10_payload_cid_resolves(t: &LoadedTape) -> AssertionResult {
   789	    for (i, e) in t.entries.iter().enumerate() {
   790	        if t.cas.get(&e.tx_payload_cid).is_err() {

exec
/bin/bash -lc 'rg -n "agent.*signature|AgentSignature|verify.*signature|agent_pubkeys|AgentKeypair|UnknownAgent|InvalidAgent|submitter_id|HasSubmitter|signature" src/state/sequencer.rs src/state/typed_tx.rs src/runtime/agent_keypairs.rs src/bus.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/runtime/agent_keypairs.rs:17://! | Public manifest     | `pinned_pubkeys.json`        | `agent_pubkeys.json`              |
src/runtime/agent_keypairs.rs:18://! | Signature type      | `SystemSignature`            | `AgentSignature` (typed_tx.rs)    |
src/runtime/agent_keypairs.rs:19://! | Verifier            | `verify_system_signature`    | `verify_agent_signature` (here)   |
src/runtime/agent_keypairs.rs:23://! Atom 4 (verify_chaintape extension) wires per-tx signature verification on
src/runtime/agent_keypairs.rs:27://! signature primitive for real-LLM proposal routing per TB-7 §4.0 / Gate 4).
src/runtime/agent_keypairs.rs:40:use crate::state::typed_tx::AgentSignature;
src/runtime/agent_keypairs.rs:73:    pub fn from_hex(hex: &str) -> Result<Self, AgentKeypairError> {
src/runtime/agent_keypairs.rs:75:            return Err(AgentKeypairError::InvalidFormat(
src/runtime/agent_keypairs.rs:82:                .map_err(|_| AgentKeypairError::InvalidFormat("non-utf8 hex"))?;
src/runtime/agent_keypairs.rs:84:                .map_err(|_| AgentKeypairError::InvalidFormat("non-hex digit"))?;
src/runtime/agent_keypairs.rs:95:pub struct AgentKeypair {
src/runtime/agent_keypairs.rs:101:impl AgentKeypair {
src/runtime/agent_keypairs.rs:103:    pub fn generate() -> Result<Self, AgentKeypairError> {
src/runtime/agent_keypairs.rs:105:        getrandom::getrandom(&mut seed).map_err(AgentKeypairError::Entropy)?;
src/runtime/agent_keypairs.rs:147:    /// `AgentSignature` so call sites cannot accidentally place agent
src/runtime/agent_keypairs.rs:148:    /// signatures in system fields.
src/runtime/agent_keypairs.rs:149:    pub fn sign_digest(&self, digest: [u8; 32]) -> Result<AgentSignature, AgentKeypairError> {
src/runtime/agent_keypairs.rs:151:            return Err(AgentKeypairError::InvalidFormat("bad secret length"));
src/runtime/agent_keypairs.rs:156:        let signature = signing_key.sign(&digest);
src/runtime/agent_keypairs.rs:158:        Ok(AgentSignature::from_bytes(signature.to_bytes()))
src/runtime/agent_keypairs.rs:162:impl fmt::Debug for AgentKeypair {
src/runtime/agent_keypairs.rs:164:        f.debug_struct("AgentKeypair")
src/runtime/agent_keypairs.rs:175:/// what `verify_chaintape` (Atom 4) reads to verify replayed agent signatures.
src/runtime/agent_keypairs.rs:183:pub struct AgentKeypairRegistry {
src/runtime/agent_keypairs.rs:184:    keypairs: BTreeMap<AgentId, AgentKeypair>,
src/runtime/agent_keypairs.rs:196:impl fmt::Debug for AgentKeypairRegistry {
src/runtime/agent_keypairs.rs:198:        f.debug_struct("AgentKeypairRegistry")
src/runtime/agent_keypairs.rs:213:impl AgentKeypairRegistry {
src/runtime/agent_keypairs.rs:216:    /// `<runtime_repo>/agent_pubkeys.json`. Mirrors TB-6 fail-closed
src/runtime/agent_keypairs.rs:218:    pub fn open(runtime_repo_path: &Path) -> Result<Self, AgentKeypairError> {
src/runtime/agent_keypairs.rs:219:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:221:            return Err(AgentKeypairError::ManifestAlreadyExists {
src/runtime/agent_keypairs.rs:241:    /// Per-run manifest at `<runtime_repo>/agent_pubkeys.json` is still written
src/runtime/agent_keypairs.rs:253:    ) -> Result<Self, AgentKeypairError> {
src/runtime/agent_keypairs.rs:254:        let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
src/runtime/agent_keypairs.rs:256:            return Err(AgentKeypairError::ManifestAlreadyExists {
src/runtime/agent_keypairs.rs:262:                .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
src/runtime/agent_keypairs.rs:263:        let mut keypairs: BTreeMap<AgentId, AgentKeypair> = BTreeMap::new();
src/runtime/agent_keypairs.rs:265:            keypairs.insert(AgentId(agent_id_raw), AgentKeypair::from_secret_bytes(seed));
src/runtime/agent_keypairs.rs:282:    pub fn get_or_create(&mut self, agent_id: &AgentId) -> Result<&AgentKeypair, AgentKeypairError> {
src/runtime/agent_keypairs.rs:284:            let kp = AgentKeypair::generate()?;
src/runtime/agent_keypairs.rs:298:    ) -> Result<AgentSignature, AgentKeypairError> {
src/runtime/agent_keypairs.rs:323:    fn persist_manifest(&self) -> Result<(), AgentKeypairError> {
src/runtime/agent_keypairs.rs:326:            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
src/runtime/agent_keypairs.rs:349:            .map_err(|e| AgentKeypairError::Serde(format!("durable keystore: {e}")))?;
src/runtime/agent_keypairs.rs:357:/// TRACE_MATRIX FC1-N14: on-disk shape of `agent_pubkeys.json`.
src/runtime/agent_keypairs.rs:359:/// to verify each WorkTx signature.
src/runtime/agent_keypairs.rs:368:    pub fn load(path: &Path) -> Result<Self, AgentKeypairError> {
src/runtime/agent_keypairs.rs:373:            .map_err(|e| AgentKeypairError::Serde(e.to_string()))?;
src/runtime/agent_keypairs.rs:388:/// TRACE_MATRIX FC1-N14: verify an agent signature against a manifest-pinned
src/runtime/agent_keypairs.rs:389:/// public key. Returns `Ok(())` on valid signature; `Err(...)` otherwise.
src/runtime/agent_keypairs.rs:391:pub fn verify_agent_signature(
src/runtime/agent_keypairs.rs:392:    signature: &AgentSignature,
src/runtime/agent_keypairs.rs:395:) -> Result<(), AgentKeypairError> {
src/runtime/agent_keypairs.rs:397:        .map_err(|e| AgentKeypairError::Verify(format!("from_bytes: {e}")))?;
src/runtime/agent_keypairs.rs:398:    let sig = Signature::from_bytes(signature.as_bytes());
src/runtime/agent_keypairs.rs:401:        .map_err(|e| AgentKeypairError::Verify(format!("verify: {e}")))
src/runtime/agent_keypairs.rs:408:pub enum AgentKeypairError {
src/runtime/agent_keypairs.rs:417:impl fmt::Display for AgentKeypairError {
src/runtime/agent_keypairs.rs:425:                write!(f, "agent_pubkeys.json already exists at {path:?}")
src/runtime/agent_keypairs.rs:427:            Self::Verify(e) => write!(f, "agent signature verify: {e}"),
src/runtime/agent_keypairs.rs:432:impl std::error::Error for AgentKeypairError {}
src/runtime/agent_keypairs.rs:434:impl From<std::io::Error> for AgentKeypairError {
src/runtime/agent_keypairs.rs:457:    /// U-A1.a — generate produces a non-zero public key + working signature.
src/runtime/agent_keypairs.rs:460:        let kp = AgentKeypair::generate().expect("generate");
src/runtime/agent_keypairs.rs:464:        assert!(verify_agent_signature(&sig, &digest, &kp.public_key()).is_ok());
src/runtime/agent_keypairs.rs:471:        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
src/runtime/agent_keypairs.rs:479:    /// U-A1.c — same agent reuses cached keypair across calls; signatures verify
src/runtime/agent_keypairs.rs:484:        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
src/runtime/agent_keypairs.rs:492:        assert!(verify_agent_signature(&sig1, &fresh_digest(2), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:493:        assert!(verify_agent_signature(&sig2, &fresh_digest(3), &pubkey).is_ok());
src/runtime/agent_keypairs.rs:500:        let mut reg = AgentKeypairRegistry::open(repo.path()).expect("open");
src/runtime/agent_keypairs.rs:519:        let _reg = AgentKeypairRegistry::open(repo.path()).expect("first open");
src/runtime/agent_keypairs.rs:520:        let err = AgentKeypairRegistry::open(repo.path()).expect_err("second open");
src/runtime/agent_keypairs.rs:522:            AgentKeypairError::ManifestAlreadyExists { .. } => {}
src/runtime/agent_keypairs.rs:527:    /// U-A1.f — wrong pubkey rejects valid signature (negative test).
src/runtime/agent_keypairs.rs:529:    fn wrong_pubkey_rejects_signature() {
src/runtime/agent_keypairs.rs:530:        let kp1 = AgentKeypair::generate().expect("kp1");
src/runtime/agent_keypairs.rs:531:        let kp2 = AgentKeypair::generate().expect("kp2");
src/runtime/agent_keypairs.rs:534:        assert!(verify_agent_signature(&sig, &digest, &kp2.public_key()).is_err());
src/runtime/agent_keypairs.rs:549:            AgentKeypairRegistry::generate_or_load_durable(repo.path(), &keystore_path, pwd.clone())
src/runtime/agent_keypairs.rs:571:            let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:583:        let mut reg_b = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:597:            verify_agent_signature(&sig_b, &fresh_digest(21), &pubkey_b).is_ok(),
src/runtime/agent_keypairs.rs:598:            "run B signature must verify under the durable pubkey"
src/runtime/agent_keypairs.rs:612:        let mut reg_a = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:621:        let err = AgentKeypairRegistry::generate_or_load_durable(
src/runtime/agent_keypairs.rs:628:            AgentKeypairError::Serde(msg) => assert!(
src/state/sequencer.rs:42:use crate::state::typed_tx::{HasSubmitter, SignalBundle, TransitionError, TypedTx};
src/state/sequencer.rs:307:/// whose `HasSubmitter::submitter_id()` returns `None` (system-emitted
src/state/sequencer.rs:389:        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
src/state/sequencer.rs:404:// system_signature should bind to. Agent variants return `None`. The
src/state/sequencer.rs:409:// `system_signature_of` / `system_epoch_of`: extract the signature + epoch
src/state/sequencer.rs:414:/// a system-emitted `TypedTx` to its `CanonicalMessage` for live signature
src/state/sequencer.rs:416:/// (their signatures are agent-domain `AgentSignature`, verified separately
src/state/sequencer.rs:448:        // (verified separately at admission via the agent-signature path).
src/state/sequencer.rs:463:fn system_signature_of(
src/state/sequencer.rs:467:        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
src/state/sequencer.rs:468:        TypedTx::TaskExpire(t) => Some(&t.system_signature),
src/state/sequencer.rs:469:        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
src/state/sequencer.rs:470:        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
src/state/sequencer.rs:471:        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
src/state/sequencer.rs:1158:        // Anti-Oreo: arm fires only when system_signature verified at
src/state/sequencer.rs:2034:    /// agent signature verification failed at submit-time admission for a
src/state/sequencer.rs:2036:    /// when the optional `agent_pubkeys` manifest is set. Either the
src/state/sequencer.rs:2037:    /// owner/provider is not registered in the manifest, or the signature
src/state/sequencer.rs:2041:    AgentSignatureInvalid,
src/state/sequencer.rs:2049:            Self::AgentSignatureInvalid => write!(
src/state/sequencer.rs:2051:                "agent signature verification failed for TB-13 variant; \
src/state/sequencer.rs:2052:                 owner/provider unregistered or signature does not match"
src/state/sequencer.rs:2068:/// pass a forged signature because they don't construct the typed tx.
src/state/sequencer.rs:2164:    /// Verification of the just-signed signature failed against pinned
src/state/sequencer.rs:2185:                write!(f, "system-tx signature construction failed: {e:?}")
src/state/sequencer.rs:2189:                "system_signature failed live verification against pinned pubkeys at emit time"
src/state/sequencer.rs:2205:/// 307 writes the apply_one signature as `Result<LedgerEntry, TransitionError>`;
src/state/sequencer.rs:2332:    /// `system_signature` on system-emitted variants (defense-in-depth atop
src/state/sequencer.rs:2340:    /// signature verification of the 3 TB-13 conditional-share variants
src/state/sequencer.rs:2345:    /// using placeholder `[0u8; 64]` signatures (the codebase-wide
src/state/sequencer.rs:2350:    /// **TB-13 enforcement**: when set via [`Sequencer::set_agent_pubkeys`],
src/state/sequencer.rs:2351:    /// `submit_agent_tx` verifies TB-13 variants' signatures against the
src/state/sequencer.rs:2352:    /// pinned pubkeys; failed verification → `SubmitError::AgentSignatureInvalid`.
src/state/sequencer.rs:2355:    agent_pubkeys: std::sync::OnceLock<Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>>,
src/state/sequencer.rs:2372:    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
src/state/sequencer.rs:2377:    /// `epoch` for by-construction signature-verification correctness.
src/state/sequencer.rs:2405:            agent_pubkeys: std::sync::OnceLock::new(),
src/state/sequencer.rs:2412:    /// install the agent pubkey manifest for submit-time signature
src/state/sequencer.rs:2418:    /// `<runtime_repo>/agent_pubkeys.json` after agent registration.
src/state/sequencer.rs:2421:    pub fn set_agent_pubkeys(
src/state/sequencer.rs:2425:        self.agent_pubkeys.set(manifest)
src/state/sequencer.rs:2486:        // submit-time agent-signature verification for the 3 TB-13
src/state/sequencer.rs:2487:        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
src/state/sequencer.rs:2488:        // when the manifest is set, forged or unregistered signatures
src/state/sequencer.rs:2489:        // are rejected pre-queue with `SubmitError::AgentSignatureInvalid`.
src/state/sequencer.rs:2493:        if let Some(manifest) = self.agent_pubkeys.get() {
src/state/sequencer.rs:2494:            use crate::runtime::agent_keypairs::verify_agent_signature;
src/state/sequencer.rs:2499:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
src/state/sequencer.rs:2501:                    if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
src/state/sequencer.rs:2502:                        return Err(SubmitError::AgentSignatureInvalid);
src/state/sequencer.rs:2508:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
src/state/sequencer.rs:2510:                    if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
src/state/sequencer.rs:2511:                        return Err(SubmitError::AgentSignatureInvalid);
src/state/sequencer.rs:2517:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
src/state/sequencer.rs:2519:                    if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
src/state/sequencer.rs:2520:                        return Err(SubmitError::AgentSignatureInvalid);
src/state/sequencer.rs:2545:    /// Cannot be invoked with a forged signature because the signature is
src/state/sequencer.rs:2555:        // Step 2: Defense-in-depth — verify the just-signed signature against
src/state/sequencer.rs:2559:        self.verify_emitted_system_tx_signature(&tx)?;
src/state/sequencer.rs:2607:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2614:                tx.system_signature = sig;
src/state/sequencer.rs:2656:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
src/state/sequencer.rs:2663:                tx.system_signature = sig;
src/state/sequencer.rs:2700:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2707:                tx.system_signature = sig;
src/state/sequencer.rs:2748:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2755:                tx.system_signature = sig;
src/state/sequencer.rs:2790:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
src/state/sequencer.rs:2797:                tx.system_signature = sig;
src/state/sequencer.rs:2803:    /// TRACE_MATRIX TB-5 Atom 4 (preflight § 4.5): defense-in-depth signature
src/state/sequencer.rs:2804:    /// verification at emit time. Verifies the just-signed signature against
src/state/sequencer.rs:2806:    fn verify_emitted_system_tx_signature(&self, tx: &TypedTx) -> Result<(), EmitSystemError> {
src/state/sequencer.rs:2807:        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
src/state/sequencer.rs:2812:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2821:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2830:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2843:                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2852:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:2920:    /// BOTH dispatch failures (stage 2) AND signature-verification failures
src/state/sequencer.rs:2929:    /// - agent_id via `tx.submitter_id().unwrap_or(SYSTEM_AGENT_ID)`
src/state/sequencer.rs:2972:            .submitter_id()
src/state/sequencer.rs:3026:        // TB-5 Atom 4 (preflight § 4.5): Stage 1.5 — defense-in-depth signature
src/state/sequencer.rs:3030:        // (or stale signature in a replay) is rejected at the apply boundary.
src/state/sequencer.rs:3035:            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
src/state/sequencer.rs:3036:            let sig = system_signature_of(&tx)
src/state/sequencer.rs:3037:                .expect("system_message_for_verification implies system_signature present");
src/state/sequencer.rs:3042:            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
src/state/sequencer.rs:3070:        // Stage 3: put payload to CAS. DIV-5 5-param put signature.
src/state/sequencer.rs:3137:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
src/state/sequencer.rs:3157:            system_signature,
src/state/sequencer.rs:3305:        AgentSignature, BoolWithProof, ChallengeTx, ClaimId, FinalizeRewardTx, PredicateId,
src/state/sequencer.rs:3380:            signature: AgentSignature::from_bytes([0x77u8; 64]),
src/state/sequencer.rs:3508:        // Compile-time: apply_one(SubmissionEnvelope) is the canonical signature.
src/state/sequencer.rs:3593:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3605:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3703:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3774:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:3924:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4054:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4179:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4338:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4362:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4385:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4407:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4435:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4448:            signature: AgentSignature::from_bytes([0; 64]),
src/state/sequencer.rs:4473:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4486:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4499:    // U27/U28 + I66/I66.a/b/c: forged signatures on system-emitted variants
src/state/sequencer.rs:4509:    /// Helper: forge a ChallengeResolveTx with all-zero signature.
src/state/sequencer.rs:4518:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4532:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4547:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4563:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/sequencer.rs:4569:    fn stage_1_5_rejects_forged_challenge_resolve_signature() {
src/state/sequencer.rs:4589:    fn stage_1_5_rejects_forged_finalize_reward_signature() {
src/state/sequencer.rs:4604:    fn stage_1_5_rejects_forged_task_expire_signature() {
src/state/sequencer.rs:4619:    fn stage_1_5_rejects_forged_terminal_summary_signature() {
src/state/sequencer.rs:4671:    /// "missing system_signature" errors when an agent variant is applied.
src/state/sequencer.rs:4694:    // arm body from the apply_one + queue + signature pipeline.
src/state/sequencer.rs:4744:            system_signature: SystemSignature::from_bytes([0u8; 64]),
src/state/typed_tx.rs:12://! types (identifiers, signatures, predicate-result types, status enums) were
src/state/typed_tx.rs:82:// § 3 AgentSignature (Ed25519 [u8;64], type-distinct from SystemSignature)
src/state/typed_tx.rs:86:/// signature over the per-tx canonical_digest. Distinct type from
src/state/typed_tx.rs:88:/// (Codex sec-arg: agent-vs-system signature mixing is a real hazard).
src/state/typed_tx.rs:90:pub struct AgentSignature(#[serde(with = "serde_bytes_64")] [u8; 64]);
src/state/typed_tx.rs:92:impl AgentSignature {
src/state/typed_tx.rs:101:impl Default for AgentSignature {
src/state/typed_tx.rs:217:/// `step_transition` (CO1.7.5 body atom). The `signature` is over
src/state/typed_tx.rs:219:/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
src/state/typed_tx.rs:233:    pub signature: AgentSignature,                    // 10
src/state/typed_tx.rs:253:    pub signature: AgentSignature,         //  7
src/state/typed_tx.rs:277:    pub signature: AgentSignature,         //  7
src/state/typed_tx.rs:305:/// - **C-3 / GM-2 followup**: `system_signature` is RETAINED for v1.1 — it
src/state/typed_tx.rs:322:    pub system_signature: SystemSignature, //  9 — see doc-comment on dual-sign rationale
src/state/typed_tx.rs:340:/// discriminator). Field 8/9/10 inserted **before** `system_signature` so
src/state/typed_tx.rs:363:    pub system_signature: SystemSignature, // 10  (was field 7 pre-TB-11)
src/state/typed_tx.rs:378:/// outcome on L4 with a system_signature.
src/state/typed_tx.rs:392:/// Fields inserted **before** `system_signature` so the signing payload sees
src/state/typed_tx.rs:418:    pub system_signature: SystemSignature,                    // 11 (was field 8 pre-TB-11)
src/state/typed_tx.rs:468:    pub system_signature: SystemSignature, //  9
src/state/typed_tx.rs:721:    pub signature: AgentSignature,             //  8
src/state/typed_tx.rs:742:    pub signature: AgentSignature,             //  6
src/state/typed_tx.rs:775:    pub system_signature: SystemSignature,          //  7
src/state/typed_tx.rs:801:// struct (subset of fields, EXCLUDES the signature itself) with a
src/state/typed_tx.rs:813:// **Forward dependency**: actual `verify_agent_signature(sig, payload, agent_pubkey)`
src/state/typed_tx.rs:853:/// Agent signing payload for `WorkTx` (12 fields → 11 fields; signature excluded).
src/state/typed_tx.rs:910:/// TRACE_MATRIX TB-3 — agent signing payload for `TaskOpenTx` (9 fields → 8 fields; signature excluded).
src/state/typed_tx.rs:1007:/// `TaskBankruptcyTx` (9 fields → 8 fields; system_signature excluded).
src/state/typed_tx.rs:1035:/// `ChallengeResolveTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1154:    pub signature: AgentSignature,            //  6
src/state/typed_tx.rs:1186:    pub signature: AgentSignature,            //  7
src/state/typed_tx.rs:1216:    pub signature: AgentSignature,            //  6
src/state/typed_tx.rs:1223:/// `CompleteSetMintTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1246:/// `CompleteSetRedeemTx` (8 fields → 7 fields; signature excluded).
src/state/typed_tx.rs:1268:/// `MarketSeedTx` (7 fields → 6 fields; signature excluded).
src/state/typed_tx.rs:1385:    /// signing payload subset (excludes `system_signature` to prevent
src/state/typed_tx.rs:1431:    /// (excludes system_signature; 7 fields → 6 fields). Used by
src/state/typed_tx.rs:1450:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1465:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1481:    /// projection. Excludes `signature` to prevent cycle-on-self.
src/state/typed_tx.rs:1548:// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
src/state/typed_tx.rs:1554:pub trait HasSubmitter {
src/state/typed_tx.rs:1555:    fn submitter_id(&self) -> Option<AgentId>;
src/state/typed_tx.rs:1558:impl HasSubmitter for WorkTx {
src/state/typed_tx.rs:1559:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1564:impl HasSubmitter for VerifyTx {
src/state/typed_tx.rs:1565:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1570:impl HasSubmitter for ChallengeTx {
src/state/typed_tx.rs:1571:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1576:impl HasSubmitter for ReuseTx {
src/state/typed_tx.rs:1577:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1582:impl HasSubmitter for FinalizeRewardTx {
src/state/typed_tx.rs:1583:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1588:impl HasSubmitter for TaskExpireTx {
src/state/typed_tx.rs:1589:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1594:impl HasSubmitter for TerminalSummaryTx {
src/state/typed_tx.rs:1595:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1600:impl HasSubmitter for TaskOpenTx {
src/state/typed_tx.rs:1601:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1606:impl HasSubmitter for EscrowLockTx {
src/state/typed_tx.rs:1607:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1612:impl HasSubmitter for ChallengeResolveTx {
src/state/typed_tx.rs:1613:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1618:impl HasSubmitter for TaskBankruptcyTx {
src/state/typed_tx.rs:1619:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1627:impl HasSubmitter for CompleteSetMintTx {
src/state/typed_tx.rs:1628:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1633:impl HasSubmitter for CompleteSetRedeemTx {
src/state/typed_tx.rs:1634:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1639:impl HasSubmitter for MarketSeedTx {
src/state/typed_tx.rs:1640:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1645:impl HasSubmitter for TypedTx {
src/state/typed_tx.rs:1646:    fn submitter_id(&self) -> Option<AgentId> {
src/state/typed_tx.rs:1648:            Self::Work(t) => t.submitter_id(),
src/state/typed_tx.rs:1649:            Self::Verify(t) => t.submitter_id(),
src/state/typed_tx.rs:1650:            Self::Challenge(t) => t.submitter_id(),
src/state/typed_tx.rs:1651:            Self::Reuse(t) => t.submitter_id(),
src/state/typed_tx.rs:1652:            Self::FinalizeReward(t) => t.submitter_id(),
src/state/typed_tx.rs:1653:            Self::TaskExpire(t) => t.submitter_id(),
src/state/typed_tx.rs:1654:            Self::TerminalSummary(t) => t.submitter_id(),
src/state/typed_tx.rs:1655:            Self::TaskOpen(t) => t.submitter_id(),
src/state/typed_tx.rs:1656:            Self::EscrowLock(t) => t.submitter_id(),
src/state/typed_tx.rs:1657:            Self::ChallengeResolve(t) => t.submitter_id(),
src/state/typed_tx.rs:1658:            Self::TaskBankruptcy(t) => t.submitter_id(),
src/state/typed_tx.rs:1659:            Self::CompleteSetMint(t) => t.submitter_id(),
src/state/typed_tx.rs:1660:            Self::CompleteSetRedeem(t) => t.submitter_id(),
src/state/typed_tx.rs:1661:            Self::MarketSeed(t) => t.submitter_id(),
src/state/typed_tx.rs:1683:    // ── Stale-parent & signature ───────────────────────────────────────────
src/state/typed_tx.rs:1686:    /// Agent signature verify failed (work / verify / challenge tx).
src/state/typed_tx.rs:1688:    /// System-keypair signature verify failed (system-emitted tx).
src/state/typed_tx.rs:1823:    /// live signature verification failed. Fired when a system-emitted
src/state/typed_tx.rs:1824:    /// variant reaches apply_one with a `system_signature` that does NOT
src/state/typed_tx.rs:1831:    /// forged-signature system variant in the queue. Maps to
src/state/typed_tx.rs:1833:    /// Per directive § 11.4: "system_signature 不能只是 schema 上的字段"
src/state/typed_tx.rs:1897:            Self::SignatureInvalid => write!(f, "agent signature invalid"),
src/state/typed_tx.rs:1898:            Self::InvalidSystemSignature => write!(f, "invalid system signature"),
src/state/typed_tx.rs:1937:                "system_signature failed live verification against pinned \
src/state/typed_tx.rs:2116:            signature: AgentSignature::from_bytes([0x77u8; 64]),
src/state/typed_tx.rs:2129:            signature: AgentSignature::from_bytes([0x55u8; 64]),
src/state/typed_tx.rs:2142:            signature: AgentSignature::from_bytes([0x33u8; 64]),
src/state/typed_tx.rs:2167:            system_signature: SystemSignature::from_bytes([0xaau8; 64]),
src/state/typed_tx.rs:2183:            system_signature: SystemSignature::from_bytes([0xbbu8; 64]),
src/state/typed_tx.rs:2207:            system_signature: SystemSignature::from_bytes([0xccu8; 64]),
src/state/typed_tx.rs:2222:            system_signature: SystemSignature::from_bytes([0xddu8; 64]),
src/state/typed_tx.rs:2256:    /// 100-input round-trip: random-ish AgentSignature bytes + variant choice.
src/state/typed_tx.rs:2262:            tx.signature = AgentSignature::from_bytes([(i % 256) as u8; 64]);
src/state/typed_tx.rs:2270:    /// HasSubmitter — agent-submitted vs system-emitted partitioning.
src/state/typed_tx.rs:2275:            TypedTx::Work(fixture_work_tx()).submitter_id(),
src/state/typed_tx.rs:2279:            TypedTx::Verify(fixture_verify_tx()).submitter_id(),
src/state/typed_tx.rs:2283:            TypedTx::Challenge(fixture_challenge_tx()).submitter_id(),
src/state/typed_tx.rs:2286:        assert_eq!(TypedTx::Reuse(fixture_reuse_tx()).submitter_id(), None);
src/state/typed_tx.rs:2288:            TypedTx::FinalizeReward(fixture_finalize_reward_tx()).submitter_id(),
src/state/typed_tx.rs:2292:            TypedTx::TaskExpire(fixture_task_expire_tx()).submitter_id(),
src/state/typed_tx.rs:2295:        // TB-11: TaskBankruptcy is system-emitted; HasSubmitter → None.
src/state/typed_tx.rs:2297:            TypedTx::TaskBankruptcy(fixture_task_bankruptcy_tx()).submitter_id(),
src/state/typed_tx.rs:2476:    /// Excluding the signature: mutating `tx.signature` must NOT change the
src/state/typed_tx.rs:2477:    /// signing-payload digest (the signature is its own input — a canonical
src/state/typed_tx.rs:2480:    fn signing_payload_excludes_signature() {
src/state/typed_tx.rs:2485:        tx_mut.signature = AgentSignature::from_bytes([0xff; 64]);
src/state/typed_tx.rs:2487:        assert_eq!(d_clean, d_mut_sig, "Work: mutating signature must NOT affect digest");
src/state/typed_tx.rs:2493:        v_mut.signature = AgentSignature::from_bytes([0xee; 64]);
src/state/typed_tx.rs:2497:            "Verify: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2504:        c_mut.signature = AgentSignature::from_bytes([0xdd; 64]);
src/state/typed_tx.rs:2508:            "Challenge: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2515:        f_mut.system_signature = SystemSignature::from_bytes([0x11; 64]);
src/state/typed_tx.rs:2519:            "FinalizeReward: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2524:        t_mut.system_signature = SystemSignature::from_bytes([0x22; 64]);
src/state/typed_tx.rs:2528:            "TaskExpire: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2533:        ts_mut.system_signature = SystemSignature::from_bytes([0x33; 64]);
src/state/typed_tx.rs:2537:            "TerminalSummary: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2543:        bk_mut.system_signature = SystemSignature::from_bytes([0x44; 64]);
src/state/typed_tx.rs:2547:            "TaskBankruptcy: mutating signature must NOT affect digest"
src/state/typed_tx.rs:2586:            "TaskBankruptcySigningPayload must have 8 fields (system_signature excluded), got {}",
src/state/typed_tx.rs:2589:        assert!(!obj.contains_key("system_signature"));
src/state/typed_tx.rs:2880:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/typed_tx.rs:2892:            signature: AgentSignature::from_bytes([0u8; 64]),
src/state/typed_tx.rs:2913:    /// T3 — TaskOpenSigningPayload excludes the signature field.
src/state/typed_tx.rs:2916:    fn task_open_signing_payload_excludes_signature() {
src/state/typed_tx.rs:2920:        assert_eq!(obj.len(), 8, "TaskOpenSigningPayload must have 8 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2921:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2924:    /// T4 — EscrowLockSigningPayload excludes the signature field.
src/state/typed_tx.rs:2927:    fn escrow_lock_signing_payload_excludes_signature() {
src/state/typed_tx.rs:2931:        assert_eq!(obj.len(), 6, "EscrowLockSigningPayload must have 6 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2932:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2982:    /// T3 — VerifySigningPayload excludes the signature field.
src/state/typed_tx.rs:2985:    fn verify_signing_payload_excludes_signature_field_count_7() {
src/state/typed_tx.rs:2989:        assert_eq!(obj.len(), 7, "VerifySigningPayload must have 7 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:2990:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:2994:    /// T4 — ChallengeSigningPayload excludes the signature field.
src/state/typed_tx.rs:2997:    fn challenge_signing_payload_excludes_signature_field_count_7() {
src/state/typed_tx.rs:3001:        assert_eq!(obj.len(), 7, "ChallengeSigningPayload must have 7 fields (signature excluded), got {}", obj.len());
src/state/typed_tx.rs:3002:        assert!(!obj.contains_key("signature"));
src/state/typed_tx.rs:3050:            system_signature: SystemSignature::from_bytes([0x99u8; 64]),
src/state/typed_tx.rs:3063:    /// T2 — ChallengeResolveSigningPayload excludes the signature field.
src/state/typed_tx.rs:3066:    fn challenge_resolve_signing_payload_excludes_signature_field_count_6() {
src/state/typed_tx.rs:3071:            "ChallengeResolveSigningPayload must have 6 fields (signature excluded), got {}",
src/state/typed_tx.rs:3073:        assert!(!obj.contains_key("system_signature"));
src/state/typed_tx.rs:3121:            signature: AgentSignature::from_bytes([0xddu8; 64]),
src/state/typed_tx.rs:3134:            signature: AgentSignature::from_bytes([0xeeu8; 64]),
src/state/typed_tx.rs:3146:            signature: AgentSignature::from_bytes([0xffu8; 64]),
src/state/typed_tx.rs:3222:    /// TB-13 U7: signing payloads exclude the `signature` field — exact
src/state/typed_tx.rs:3225:    fn tb_13_signing_payloads_exclude_signature_field_counts() {
src/state/typed_tx.rs:3230:        assert!(!mint_o.contains_key("signature"));
src/state/typed_tx.rs:3236:        assert!(!redeem_o.contains_key("signature"));
src/state/typed_tx.rs:3242:        assert!(!seed_o.contains_key("signature"));
src/state/typed_tx.rs:3245:    /// TB-13 U8: HasSubmitter projects to the wire owner / provider.
src/state/typed_tx.rs:3249:        assert_eq!(mint.submitter_id(), Some(mint.owner.clone()));
src/state/typed_tx.rs:3252:        assert_eq!(redeem.submitter_id(), Some(redeem.owner.clone()));
src/state/typed_tx.rs:3255:        assert_eq!(seed.submitter_id(), Some(seed.provider.clone()));
src/state/typed_tx.rs:3259:            TypedTx::CompleteSetMint(fixture_complete_set_mint_tx()).submitter_id(),
src/state/typed_tx.rs:3263:            TypedTx::MarketSeed(fixture_market_seed_tx()).submitter_id(),

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '2325,2535p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2325	    rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
  2326	
  2327	    predicate_registry: Arc<PredicateRegistry>,
  2328	    tool_registry: Arc<ToolRegistry>,
  2329	
  2330	    /// TB-5 Atom 4 (charter v2 § 4.3 + preflight § 4.2): pinned system-key
  2331	    /// public-key map. Used by apply_one stage 1.5 to verify
  2332	    /// `system_signature` on system-emitted variants (defense-in-depth atop
  2333	    /// the constructive guarantee from `emit_system_tx`). Tests pin
  2334	    /// `self.keypair`'s pubkey under `epoch` for by-construction verification;
  2335	    /// production sources from `genesis_payload.toml [system_pubkeys]`.
  2336	    pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
  2337	
  2338	    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH remediation
  2339	    /// 2026-05-03): opt-in agent pubkey manifest for submit-time
  2340	    /// signature verification of the 3 TB-13 conditional-share variants
  2341	    /// (CompleteSetMint / CompleteSetRedeem / MarketSeed).
  2342	    ///
  2343	    /// **Default state**: empty (`OnceLock::new()`) — preserves
  2344	    /// backward-compat with all TB-3..TB-12 callers + test fixtures
  2345	    /// using placeholder `[0u8; 64]` signatures (the codebase-wide
  2346	    /// agent-sig admission gap is OBS-tracked at
  2347	    /// `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md` and remains future scope
  2348	    /// for the broader codebase).
  2349	    ///
  2350	    /// **TB-13 enforcement**: when set via [`Sequencer::set_agent_pubkeys`],
  2351	    /// `submit_agent_tx` verifies TB-13 variants' signatures against the
  2352	    /// pinned pubkeys; failed verification → `SubmitError::AgentSignatureInvalid`.
  2353	    /// Closes Codex round-2 VETO TB13-AUTH for Class 3
  2354	    /// (money/collateral) admission control.
  2355	    agent_pubkeys: std::sync::OnceLock<Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>>,
  2356	
  2357	    q: RwLock<QState>,
  2358	}
  2359	
  2360	/// CO1.7-extra D3 (round-2 MF6): manual Debug impl. Uses `finish_non_exhaustive()`
  2361	/// to satisfy the Debug trait without exposing keypair / QState / CAS internals.
  2362	impl std::fmt::Debug for Sequencer {
  2363	    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  2364	        f.debug_struct("Sequencer").finish_non_exhaustive()
  2365	    }
  2366	}
  2367	
  2368	impl Sequencer {
  2369	    /// Construct. Returns the `Sequencer` plus the receiver half of the
  2370	    /// internal mpsc; pass the receiver to `run()` exactly once.
  2371	    ///
  2372	    /// **TB-5 Atom 4 signature change** (charter v2 § 4.2 + preflight § 4.2):
  2373	    /// added `pinned_pubkeys` parameter. Existing callers (7 src + tests
  2374	    /// per Codex round-2 cascade) updated to pass an `Arc<PinnedSystemPubkeys>`
  2375	    /// derived from the same keypair (test fixtures) or genesis-pinned
  2376	    /// (production). Tests typically pin `keypair.public_key()` under
  2377	    /// `epoch` for by-construction signature-verification correctness.
  2378	    #[allow(clippy::too_many_arguments)]
  2379	    pub fn new(
  2380	        cas: Arc<RwLock<CasStore>>,
  2381	        keypair: Arc<Ed25519Keypair>,
  2382	        epoch: SystemEpoch,
  2383	        ledger_writer: Arc<RwLock<dyn LedgerWriter>>,
  2384	        rejection_writer: Arc<RwLock<RejectionEvidenceWriter>>,
  2385	        predicate_registry: Arc<PredicateRegistry>,
  2386	        tool_registry: Arc<ToolRegistry>,
  2387	        pinned_pubkeys: Arc<crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys>,
  2388	        initial_q: QState,
  2389	        queue_capacity: usize,
  2390	    ) -> (Self, tokio::sync::mpsc::Receiver<SubmissionEnvelope>) {
  2391	        let (queue_tx, queue_rx) = tokio::sync::mpsc::channel(queue_capacity);
  2392	        let seq = Self {
  2393	            next_submit_id: AtomicU64::new(1),
  2394	            next_logical_t: AtomicU64::new(0), // first accepted commit advances to 1
  2395	            next_emit_id: AtomicU64::new(1),    // TB-5 Atom 4: parallel system-emit counter
  2396	            queue_tx,
  2397	            cas,
  2398	            keypair,
  2399	            epoch,
  2400	            ledger_writer,
  2401	            rejection_writer,
  2402	            predicate_registry,
  2403	            tool_registry,
  2404	            pinned_pubkeys,
  2405	            agent_pubkeys: std::sync::OnceLock::new(),
  2406	            q: RwLock::new(initial_q),
  2407	        };
  2408	        (seq, queue_rx)
  2409	    }
  2410	
  2411	    /// TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
  2412	    /// install the agent pubkey manifest for submit-time signature
  2413	    /// verification of TB-13 variants. Called once per Sequencer
  2414	    /// lifetime (post-construction, pre-first-submit). Returns the
  2415	    /// manifest back as `Err` if already set.
  2416	    ///
  2417	    /// Production binaries plumb this from
  2418	    /// `<runtime_repo>/agent_pubkeys.json` after agent registration.
  2419	    /// Tests may opt in by constructing an `AgentPubkeyManifest` from
  2420	    /// real keypairs.
  2421	    pub fn set_agent_pubkeys(
  2422	        &self,
  2423	        manifest: Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>,
  2424	    ) -> Result<(), Arc<crate::runtime::agent_keypairs::AgentPubkeyManifest>> {
  2425	        self.agent_pubkeys.set(manifest)
  2426	    }
  2427	
  2428	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek pinned_pubkeys (for tests +
  2429	    /// observability; production callers should not depend on this).
  2430	    #[cfg(test)]
  2431	    pub fn pinned_pubkeys(&self) -> &crate::bottom_white::ledger::system_keypair::PinnedSystemPubkeys {
  2432	        &self.pinned_pubkeys
  2433	    }
  2434	
  2435	    /// TRACE_MATRIX TB-5 charter v2 § 4.2: peek next_emit_id (parallel to
  2436	    /// `next_submit_id_peek` for K1-style observability).
  2437	    pub fn next_emit_id_peek(&self) -> u64 {
  2438	        self.next_emit_id.load(Ordering::SeqCst)
  2439	    }
  2440	
  2441	    /// TRACE_MATRIX FC2-Submit + § 5.2.1: TB-5.0 Atom 2 agent-only ingress
  2442	    /// barrier (charter v2 § 4.2 + § 4.9 + preflight § 3.2; Anti-Oreo Art V.1.3).
  2443	    ///
  2444	    /// Accepts ONLY agent-submitted variants. System-emitted variants
  2445	    /// (FinalizeReward / TaskExpire / TerminalSummary; ChallengeResolve added
  2446	    /// in Atom 3) are rejected pre-queue with
  2447	    /// `SubmitError::SystemTxForbiddenOnAgentIngress`. This is the
  2448	    /// constitutional Anti-Oreo "agent ≠ direct state writer" boundary,
  2449	    /// structurally enforced (was a documented norm without live enforcement
  2450	    /// through TB-3 + TB-4; TB-5.0 retires that debt for system-tx).
  2451	    ///
  2452	    /// **WP-canonical reconciliation**: ChallengeResolveTx (TB-5 Atom 3) +
  2453	    /// SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx
  2454	    /// (RSP-3.2+ / RSP-4 territory) will be added to the rejection match
  2455	    /// at their respective TB landings — each new system variant extends
  2456	    /// this list, never bypasses it.
  2457	    pub async fn submit_agent_tx(&self, tx: TypedTx) -> Result<SubmissionReceipt, SubmitError> {
  2458	        // TB-5.0 ingress barrier: reject 4 system-emitted variants
  2459	        // (FinalizeReward / TaskExpire / TerminalSummary added in Atom 2;
  2460	        // ChallengeResolve added in Atom 3 when its TypedTx variant landed).
  2461	        match &tx {
  2462	            TypedTx::FinalizeReward(_)
  2463	            | TypedTx::TaskExpire(_)
  2464	            | TypedTx::TerminalSummary(_)
  2465	            | TypedTx::ChallengeResolve(_)
  2466	            // TB-11 Atom 1 (architect §6.2 ruling 2026-05-02): TaskBankruptcyTx
  2467	            // is system-emitted only; agent ingress must reject pre-queue per
  2468	            // Anti-Oreo (Art V.1.3). Construction goes through emit_system_tx.
  2469	            | TypedTx::TaskBankruptcy(_) => {
  2470	                return Err(SubmitError::SystemTxForbiddenOnAgentIngress);
  2471	            }
  2472	            // Agent-submitted variants — proceed to queue. TB-13 conditional-
  2473	            // share variants (CompleteSetMint / CompleteSetRedeem / MarketSeed)
  2474	            // are agent-signed and admit through the same ingress path.
  2475	            TypedTx::Work(_)
  2476	            | TypedTx::Verify(_)
  2477	            | TypedTx::Challenge(_)
  2478	            | TypedTx::Reuse(_)
  2479	            | TypedTx::TaskOpen(_)
  2480	            | TypedTx::EscrowLock(_)
  2481	            | TypedTx::CompleteSetMint(_)
  2482	            | TypedTx::CompleteSetRedeem(_)
  2483	            | TypedTx::MarketSeed(_) => {}
  2484	        }
  2485	        // TRACE_MATRIX TB-13 Atom 6 round-3 (Codex VETO TB13-AUTH 2026-05-03):
  2486	        // submit-time agent-signature verification for the 3 TB-13
  2487	        // conditional-share variants. Opt-in via `set_agent_pubkeys` —
  2488	        // when the manifest is set, forged or unregistered signatures
  2489	        // are rejected pre-queue with `SubmitError::AgentSignatureInvalid`.
  2490	        // When the manifest is absent (default), this gate is bypassed
  2491	        // and replay-time `verify.rs` Gate 4 is the only line of defense
  2492	        // (see OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md).
  2493	        if let Some(manifest) = self.agent_pubkeys.get() {
  2494	            use crate::runtime::agent_keypairs::verify_agent_signature;
  2495	            match &tx {
  2496	                TypedTx::CompleteSetMint(mint) => {
  2497	                    let pubkey = manifest
  2498	                        .get(&mint.owner)
  2499	                        .ok_or(SubmitError::AgentSignatureInvalid)?;
  2500	                    let digest = mint.to_signing_payload().canonical_digest();
  2501	                    if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
  2502	                        return Err(SubmitError::AgentSignatureInvalid);
  2503	                    }
  2504	                }
  2505	                TypedTx::CompleteSetRedeem(redeem) => {
  2506	                    let pubkey = manifest
  2507	                        .get(&redeem.owner)
  2508	                        .ok_or(SubmitError::AgentSignatureInvalid)?;
  2509	                    let digest = redeem.to_signing_payload().canonical_digest();
  2510	                    if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
  2511	                        return Err(SubmitError::AgentSignatureInvalid);
  2512	                    }
  2513	                }
  2514	                TypedTx::MarketSeed(seed) => {
  2515	                    let pubkey = manifest
  2516	                        .get(&seed.provider)
  2517	                        .ok_or(SubmitError::AgentSignatureInvalid)?;
  2518	                    let digest = seed.to_signing_payload().canonical_digest();
  2519	                    if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
  2520	                        return Err(SubmitError::AgentSignatureInvalid);
  2521	                    }
  2522	                }
  2523	                // Other agent variants are not gated here — codebase-wide
  2524	                // forward-dep per OBS_AGENT_SIG_REPLAY_GAP.
  2525	                _ => {}
  2526	            }
  2527	        }
  2528	        // TB-2 P1-D r1 concurrency contract: fetch_add precedes try_send, so
  2529	        // submit_id allocation order is NOT receiver arrival order under
  2530	        // multi-producer scheduling. submit_id is always burned (never reused)
  2531	        // even when try_send fails — locked by integration test I2.
  2532	        let submit_id = self.next_submit_id.fetch_add(1, Ordering::SeqCst);
  2533	        let envelope = SubmissionEnvelope { submit_id, tx };
  2534	        match self.queue_tx.try_send(envelope) {
  2535	            Ok(()) => Ok(SubmissionReceipt { submit_id }),

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '260,440p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   260	        pinned.insert(SystemEpoch::new(entry.epoch), pubkey);
   261	    }
   262	
   263	    // Step 2: initial QState.
   264	    let initial_q_path = runtime_repo_path.join(INITIAL_Q_STATE_FILENAME);
   265	    let (initial_q, initial_q_loaded_from_disk) = if initial_q_path.exists() {
   266	        let s = std::fs::read_to_string(&initial_q_path)?;
   267	        let q: QState = serde_json::from_str(&s)
   268	            .map_err(|e| VerifyError::InitialQStateParse(e.to_string()))?;
   269	        (q, true)
   270	    } else {
   271	        (QState::genesis(), false)
   272	    };
   273	
   274	    // Step 3: open ledger writer + read all entries.
   275	    let writer = Git2LedgerWriter::open(runtime_repo_path)?;
   276	    let l4_entries = writer.len();
   277	    let head_commit_oid_hex = writer.head_commit_oid_hex();
   278	    let entries: Vec<LedgerEntry> = (1..=l4_entries)
   279	        .map(|t| writer.read_at(t))
   280	        .collect::<Result<Vec<_>, _>>()?;
   281	
   282	    // Step 4: open CAS.
   283	    let cas_store = CasStore::open(cas_path).map_err(|e| VerifyError::Cas(e.to_string()))?;
   284	
   285	    // Step 5: open + verify L4.E chain.
   286	    let rejections_path = runtime_repo_path.join(REJECTIONS_JSONL_FILENAME);
   287	    let l4e_writer = if rejections_path.exists() {
   288	        RejectionEvidenceWriter::open_jsonl(rejections_path).map_err(VerifyError::L4eOpen)?
   289	    } else {
   290	        RejectionEvidenceWriter::new()
   291	    };
   292	    let l4e_entries = l4e_writer.len() as u64;
   293	    let l4e_last_hash_hex = hash_to_hex(&l4e_writer.last_hash());
   294	
   295	    // Step 6: replay.
   296	    let predicate_registry = PredicateRegistry::new();
   297	    let tool_registry = ToolRegistry::new();
   298	    let replay_outcome = replay_full_transition(
   299	        &initial_q,
   300	        &entries,
   301	        &cas_store,
   302	        &pinned,
   303	        &predicate_registry,
   304	        &tool_registry,
   305	    );
   306	
   307	    let (
   308	        ledger_root_verified,
   309	        system_signatures_verified,
   310	        state_reconstructed,
   311	        economic_state_reconstructed,
   312	        cas_payloads_retrievable,
   313	        final_state_root_hex,
   314	        final_ledger_root_hex,
   315	        replay_failure,
   316	    ) = match replay_outcome {
   317	        Ok(final_q) => (
   318	            true,
   319	            true,
   320	            true,
   321	            true,
   322	            true,
   323	            Some(hash_to_hex(&final_q.state_root_t)),
   324	            Some(hash_to_hex(&final_q.ledger_root_t)),
   325	            None,
   326	        ),
   327	        Err(err) => classify_replay_error(&err),
   328	    };
   329	
   330	    // ── TB-7 Atom 4: agent signature verification (Gate 4) ──
   331	    //
   332	    // Walk every L4 entry; for WorkTx and VerifyTx variants, verify the
   333	    // AgentSignature against the per-run agent_pubkeys.json manifest.
   334	    // Empty chain or chain with no Work/Verify entries → trivially true
   335	    // (no agent signatures to fail).
   336	    let (agent_signatures_verified, proposal_telemetry_cas_retrievable) =
   337	        verify_agent_artifacts(runtime_repo_path, &cas_store, &entries);
   338	
   339	    Ok(ReplayReport {
   340	        l4_entries,
   341	        l4e_entries,
   342	        ledger_root_verified,
   343	        system_signatures_verified,
   344	        state_reconstructed,
   345	        economic_state_reconstructed,
   346	        cas_payloads_retrievable,
   347	        agent_signatures_verified,
   348	        proposal_telemetry_cas_retrievable,
   349	        run_id: manifest.run_id,
   350	        epoch: manifest.epoch,
   351	        detail: ReplayReportDetail {
   352	            final_state_root_hex,
   353	            final_ledger_root_hex,
   354	            head_commit_oid_hex,
   355	            l4e_last_hash_hex,
   356	            replay_failure,
   357	            initial_q_state_loaded_from_disk: initial_q_loaded_from_disk,
   358	        },
   359	    })
   360	}
   361	
   362	/// TRACE_MATRIX FC1-N14: TB-7 Atom 4 — verify Gate 4 + Gate 5 indicators by
   363	/// walking every L4 entry and (for WorkTx / VerifyTx variants) re-verifying
   364	/// agent signatures against the on-disk `agent_pubkeys.json` manifest, plus
   365	/// checking that every `WorkTx.proposal_cid` resolves to a CAS-resident
   366	/// ProposalTelemetry object.
   367	///
   368	/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
   369	/// Both default to `true` when the manifest doesn't exist or when no
   370	/// Work/Verify entries are present (no signatures to verify ≠ failure).
   371	fn verify_agent_artifacts(
   372	    runtime_repo_path: &Path,
   373	    cas_store: &CasStore,
   374	    entries: &[LedgerEntry],
   375	) -> (bool, bool) {
   376	    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
   377	    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
   378	    use crate::runtime::proposal_telemetry::read_from_cas as read_telemetry;
   379	    use crate::state::typed_tx::TypedTx;
   380	
   381	    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
   382	    if !manifest_path.exists() {
   383	        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
   384	        // trivially true since there are no agent-side artifacts to fail.
   385	        return (true, true);
   386	    }
   387	    let manifest = match AgentPubkeyManifest::load(&manifest_path) {
   388	        Ok(m) => m,
   389	        Err(_) => return (false, false), // manifest unparseable = both fail
   390	    };
   391	
   392	    let mut agent_signatures_verified = true;
   393	    let mut proposal_telemetry_cas_retrievable = true;
   394	
   395	    for entry in entries {
   396	        // Get the typed payload from CAS.
   397	        let payload_bytes = match cas_store.get(&entry.tx_payload_cid) {
   398	            Ok(b) => b,
   399	            Err(_) => continue, // cas_payloads_retrievable already covers this
   400	        };
   401	        let typed_tx: TypedTx = match canonical_decode(&payload_bytes) {
   402	            Ok(tx) => tx,
   403	            Err(_) => continue, // payload decode error already covered upstream
   404	        };
   405	
   406	        match &typed_tx {
   407	            TypedTx::Work(work) => {
   408	                // Gate 4 — verify WorkTx signature.
   409	                let payload = work.to_signing_payload();
   410	                let digest = payload.canonical_digest();
   411	                let pubkey_opt = manifest.get(&work.agent_id);
   412	                match pubkey_opt {
   413	                    None => agent_signatures_verified = false,
   414	                    Some(pubkey) => {
   415	                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
   416	                            agent_signatures_verified = false;
   417	                        }
   418	                    }
   419	                }
   420	                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
   421	                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
   422	                if work.proposal_cid.0 != [0u8; 32] {
   423	                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
   424	                        proposal_telemetry_cas_retrievable = false;
   425	                    }
   426	                }
   427	            }
   428	            TypedTx::Verify(verify) => {
   429	                // Gate 4 — verify VerifyTx signature.
   430	                let payload = verify.to_signing_payload();
   431	                let digest = payload.canonical_digest();
   432	                let pubkey_opt = manifest.get(&verify.verifier_agent);
   433	                match pubkey_opt {
   434	                    None => agent_signatures_verified = false,
   435	                    Some(pubkey) => {
   436	                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
   437	                            agent_signatures_verified = false;
   438	                        }
   439	                    }
   440	                }

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '440,500p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   440	                }
   441	            }
   442	            // TRACE_MATRIX TB-13 Atom 6 round-2 (Codex VETO TB13-V2
   443	            // remediation 2026-05-03): extend Gate 4 to cover the 3
   444	            // agent-signed TB-13 variants. The submit-time verification
   445	            // gap is codebase-wide (also affects Challenge/TaskOpen/
   446	            // EscrowLock); replay-time coverage is the existing TB-7
   447	            // ARCHITECT_RULING D3 model. TB-13 raises the bar to its
   448	            // own three variants because Class 3 money-mover.
   449	            TypedTx::CompleteSetMint(mint) => {
   450	                let payload = mint.to_signing_payload();
   451	                let digest = payload.canonical_digest();
   452	                let pubkey_opt = manifest.get(&mint.owner);
   453	                match pubkey_opt {
   454	                    None => agent_signatures_verified = false,
   455	                    Some(pubkey) => {
   456	                        if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
   457	                            agent_signatures_verified = false;
   458	                        }
   459	                    }
   460	                }
   461	            }
   462	            TypedTx::CompleteSetRedeem(redeem) => {
   463	                let payload = redeem.to_signing_payload();
   464	                let digest = payload.canonical_digest();
   465	                let pubkey_opt = manifest.get(&redeem.owner);
   466	                match pubkey_opt {
   467	                    None => agent_signatures_verified = false,
   468	                    Some(pubkey) => {
   469	                        if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
   470	                            agent_signatures_verified = false;
   471	                        }
   472	                    }
   473	                }
   474	            }
   475	            TypedTx::MarketSeed(seed) => {
   476	                let payload = seed.to_signing_payload();
   477	                let digest = payload.canonical_digest();
   478	                let pubkey_opt = manifest.get(&seed.provider);
   479	                match pubkey_opt {
   480	                    None => agent_signatures_verified = false,
   481	                    Some(pubkey) => {
   482	                        if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
   483	                            agent_signatures_verified = false;
   484	                        }
   485	                    }
   486	                }
   487	            }
   488	            // Remaining tx variants (TaskOpen / EscrowLock / Challenge /
   489	            // ChallengeResolve / ReuseTx / FinalizeReward / TaskExpire /
   490	            // TerminalSummary / TaskBankruptcy) are not covered by Gate 4
   491	            // because:
   492	            // - Some are system-emitted (signature path is system, not agent;
   493	            //   covered by system_signatures_verified above).
   494	            // - Others are agent-emitted but their signing payloads need
   495	            //   per-variant signing helpers and are deferred to a future
   496	            //   codebase-wide CO P2.x AgentRegistry pass per `OBS_AGENT_SIG_REPLAY_GAP_2026-05-03`.
   497	            _ => {}
   498	        }
   499	    }
   500	

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '500,530p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   500	
   501	    (
   502	        agent_signatures_verified,
   503	        proposal_telemetry_cas_retrievable,
   504	    )
   505	}
   506	
   507	// ── Helpers ─────────────────────────────────────────────────────────────────
   508	
   509	fn decode_pubkey_hex(hex: &str) -> Result<Vec<u8>, VerifyError> {
   510	    if hex.len() % 2 != 0 {
   511	        return Err(VerifyError::PubkeyDecode(format!(
   512	            "odd-length hex string ({})",
   513	            hex.len()
   514	        )));
   515	    }
   516	    let mut out = Vec::with_capacity(hex.len() / 2);
   517	    for chunk in hex.as_bytes().chunks(2) {
   518	        let s = std::str::from_utf8(chunk)
   519	            .map_err(|e| VerifyError::PubkeyDecode(e.to_string()))?;
   520	        let byte = u8::from_str_radix(s, 16)
   521	            .map_err(|e| VerifyError::PubkeyDecode(e.to_string()))?;
   522	        out.push(byte);
   523	    }
   524	    Ok(out)
   525	}
   526	
   527	fn hash_to_hex(h: &Hash) -> String {
   528	    h.0.iter().map(|b| format!("{:02x}", b)).collect()
   529	}
   530	

exec
/bin/bash -lc "python3 - <<'PY'
import re, pathlib
p=pathlib.Path('src/runtime/audit_assertions.rs')
lines=p.read_text().splitlines()
pat=re.compile(r'"'^(?:pub'"\\s+(?:struct|enum|const|type|fn)|\\s*pub\\s+fn|\\s*pub\\s+[A-Za-z_].*:|impl .*)')
# better collect public decls/fields/functions
for i,l in enumerate(lines,1):
    s=l.strip()
    if re.match(r'pub (struct|enum|fn|const|type)\\b', s) or re.match(r'pub fn\\b', s) or re.match(r'pub [A-Za-z_][A-Za-z0-9_]*:', s):
        # look back to previous non-attribute? include deriv attr
        prev=[]
        j=i-2
        while j>=0 and (lines[j].strip().startswith('///') or lines[j].strip().startswith('#[') or lines[j].strip()=='' ):
            if lines[j].strip().startswith('///'): prev.append(lines[j].strip())
            j-=1
        has_trace=any('TRACE_MATRIX' in x for x in prev)
        print(f'{i}: {s[:100]} | trace={has_trace} | docs={\" \".join(reversed(prev))[:120]}')
PY" in /home/zephryj/projects/turingosv4
 succeeded in 130ms:
66: pub struct AuditInputs { | trace=True | docs=/// Inputs to the audit binary. Paths only — live process state is /// forbidden per CR-16.6 (replayability) + Art.0.2 (
67: pub runtime_repo: PathBuf, | trace=False | docs=
68: pub cas_dir: PathBuf, | trace=False | docs=
69: pub agent_pubkeys: PathBuf, | trace=False | docs=
70: pub pinned_pubkeys: PathBuf, | trace=False | docs=
71: pub genesis: PathBuf, | trace=False | docs=
72: pub constitution: PathBuf, | trace=False | docs=
73: pub markov_pointer: PathBuf, | trace=False | docs=
74: pub alignment_dir: Option<PathBuf>, | trace=False | docs=
79: pub enum AssertionLayer { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
92: pub enum AssertionVerdict { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
101: pub struct AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
102: pub id: u32, | trace=False | docs=
103: pub name: String, | trace=False | docs=
104: pub layer: AssertionLayer, | trace=False | docs=
105: pub result: AssertionVerdict, | trace=False | docs=
106: pub detail: Option<String>, | trace=False | docs=
150: pub struct TapeRoot { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
151: pub l4_count: u64, | trace=False | docs=
152: pub l4e_count: u64, | trace=False | docs=
153: pub head_state_root_hex: String, | trace=False | docs=
154: pub head_ledger_root_hex: String, | trace=False | docs=
155: pub cas_object_count: u64, | trace=False | docs=
156: pub constitution_hash_hex: String, | trace=False | docs=
161: pub struct TxKindCounts { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
162: pub work: u64, | trace=False | docs=
163: pub verify: u64, | trace=False | docs=
164: pub challenge: u64, | trace=False | docs=
165: pub reuse: u64, | trace=False | docs=
166: pub task_open: u64, | trace=False | docs=
167: pub escrow_lock: u64, | trace=False | docs=
168: pub complete_set_mint: u64, | trace=False | docs=
169: pub complete_set_redeem: u64, | trace=False | docs=
170: pub market_seed: u64, | trace=False | docs=
171: pub finalize_reward: u64, | trace=False | docs=
172: pub challenge_resolve: u64, | trace=False | docs=
173: pub terminal_summary: u64, | trace=False | docs=
174: pub task_expire: u64, | trace=False | docs=
175: pub task_bankruptcy: u64, | trace=False | docs=
180: pub fn from_entries(entries: &[LedgerEntry]) -> Self { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
203: pub fn missing_required(&self) -> Vec<&'static str> { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
231: pub struct TapeAuditVerdict { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
232: pub schema_version: String, | trace=False | docs=
233: pub tape_root: TapeRoot, | trace=False | docs=
234: pub tx_kind_counts: TxKindCounts, | trace=False | docs=
235: pub assertions: Vec<AssertionResult>, | trace=False | docs=
236: pub passed: u32, | trace=False | docs=
237: pub failed: u32, | trace=False | docs=
238: pub halted: u32, | trace=False | docs=
239: pub skipped: u32, | trace=False | docs=
240: pub feature_coverage: BTreeMap<String, String>, | trace=False | docs=
241: pub verdict: String, // "PROCEED" | "BLOCK" | trace=False | docs=
250: pub enum AuditError { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
301: pub struct LoadedTape { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
302: pub entries: Vec<LedgerEntry>, | trace=False | docs=
303: pub l4e_writer: RejectionEvidenceWriter, | trace=False | docs=
304: pub cas: CasStore, | trace=False | docs=
305: pub pinned: PinnedSystemPubkeys, | trace=False | docs=
306: pub pinned_manifest: PinnedPubkeyManifest, | trace=False | docs=
307: pub agent_manifest: AgentPubkeyManifest, | trace=False | docs=
308: pub initial_q: QState, | trace=False | docs=
309: pub replayed_q: Option<QState>, | trace=False | docs=
310: pub replay_error: Option<ReplayError>, | trace=False | docs=
311: pub constitution_bytes: Vec<u8>, | trace=False | docs=
312: pub constitution_hash: Hash, | trace=False | docs=
313: pub markov_capsule: Option<MarkovEvidenceCapsule>, | trace=False | docs=
314: pub genesis_constitution_root_hex: Option<String>, | trace=False | docs=
322: pub fn load_tape(inputs: &AuditInputs) -> Result<LoadedTape, AuditError> { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
552: pub fn assert_01_constitution_hash_matches_genesis(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
575: pub fn assert_02_pinned_pubkey_loaded(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
588: pub fn assert_03_sandbox_agent_prefix(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
612: pub fn assert_04_l4_hash_chain_valid(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
651: pub fn assert_05_l4_parent_state_continuity(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
668: pub fn assert_06_l4e_chain_integrity(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
681: pub fn assert_07_genesis_row_zero_parents(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
703: pub fn assert_08_system_tx_signatures_verify(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
737: pub fn assert_09_agent_tx_signatures_verify(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
788: pub fn assert_10_payload_cid_resolves(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
803: pub fn assert_11_tx_kind_envelope_matches_payload(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
848: pub fn assert_12_replay_state_root_matches_head(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
885: pub fn assert_13_replay_economic_state_canonical(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
908: pub fn assert_14_replay_autopsy_index_chains(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
934: pub fn assert_15_canonical_edges_replay_deterministic(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
964: pub fn assert_16_replay_idempotent_across_calls(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1040: pub fn assert_17_no_post_init_mint(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1058: pub fn assert_18_total_supply_conserved(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1084: pub fn assert_19_complete_set_min_balanced(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1127: pub fn assert_20_task_market_total_escrow_matches_locks(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1159: pub fn assert_21_node_positions_excluded_from_supply(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1197: pub fn assert_22_conditional_shares_excluded_from_supply(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1239: pub fn assert_23_accepted_work_predicate_results_true(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1276: pub fn assert_24_proposal_telemetry_chain(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1362: pub fn assert_25_l4e_rejection_class_redispatch(_t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1372: pub fn assert_26_price_index_is_view_only(_t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1382: pub fn assert_27_terminal_summary_evidence_capsule(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1441: pub fn assert_28_projection_no_autopsy_bytes(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1490: pub fn assert_29_autopsy_private_detail_creator_is_system(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1535: pub fn assert_30_typical_error_summary_no_private_detail(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1610: pub fn assert_31_autopsy_index_value_type_is_vec_cid() -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1652: pub fn assert_f_no_llm_self_narrative_in_autopsy(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1710: pub fn assert_32_markov_constitution_hash_matches(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1739: pub fn assert_33_markov_typical_errors_recompute(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1793: pub fn assert_34_markov_unresolved_obs_recompute( | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1847: pub fn assert_35_markov_next_session_context_resolves(t: &LoadedTape) -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1888: pub fn assert_36_tamper_l4_flip_detected() -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1898: pub fn assert_37_tamper_cas_flip_detected() -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1908: pub fn assert_38_tamper_l4_remove_detected() -> AssertionResult { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1922: pub fn run_all_assertions(inputs: &AuditInputs) -> Result<Vec<AssertionResult>, AuditError> { | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
1976: pub fn summarize_results( | trace=True | docs=/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '300,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   301	pub struct LoadedTape {
   302	    pub entries: Vec<LedgerEntry>,
   303	    pub l4e_writer: RejectionEvidenceWriter,
   304	    pub cas: CasStore,
   305	    pub pinned: PinnedSystemPubkeys,
   306	    pub pinned_manifest: PinnedPubkeyManifest,
   307	    pub agent_manifest: AgentPubkeyManifest,
   308	    pub initial_q: QState,
   309	    pub replayed_q: Option<QState>,
   310	    pub replay_error: Option<ReplayError>,
   311	    pub constitution_bytes: Vec<u8>,
   312	    pub constitution_hash: Hash,
   313	    pub markov_capsule: Option<MarkovEvidenceCapsule>,
   314	    pub genesis_constitution_root_hex: Option<String>,
   315	}
   316	
   317	const PINNED_PUBKEYS_FILENAME: &str = "pinned_pubkeys.json";
   318	const REJECTIONS_JSONL_FILENAME: &str = "rejections.jsonl";
   319	const INITIAL_Q_STATE_FILENAME: &str = "initial_q_state.json";
   320	
   321	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
   322	pub fn load_tape(inputs: &AuditInputs) -> Result<LoadedTape, AuditError> {
   323	    // pinned manifest
   324	    let pinned_path = if inputs.pinned_pubkeys.is_file() {
   325	        inputs.pinned_pubkeys.clone()
   326	    } else {
   327	        inputs.runtime_repo.join(PINNED_PUBKEYS_FILENAME)
   328	    };
   329	    let pinned_text = std::fs::read_to_string(&pinned_path)
   330	        .map_err(|e| AuditError::PinnedManifest(format!("read {pinned_path:?}: {e}")))?;
   331	    let pinned_manifest: PinnedPubkeyManifest = serde_json::from_str(&pinned_text)
   332	        .map_err(|e| AuditError::PinnedManifest(e.to_string()))?;
   333	    let mut pinned = PinnedSystemPubkeys::new();
   334	    for entry in &pinned_manifest.pubkeys {
   335	        let bytes = hex_decode(&entry.pubkey_hex)
   336	            .map_err(|e| AuditError::PinnedManifest(format!("pubkey hex: {e}")))?;
   337	        let arr: [u8; 32] = bytes
   338	            .as_slice()
   339	            .try_into()
   340	            .map_err(|_| AuditError::PinnedManifest("expected 32-byte pubkey".into()))?;
   341	        pinned.insert(SystemEpoch::new(entry.epoch), SystemPublicKey::from_bytes(arr));
   342	    }
   343	
   344	    // agent manifest
   345	    let agent_manifest = AgentPubkeyManifest::load(&inputs.agent_pubkeys)
   346	        .map_err(|e| AuditError::AgentManifest(e.to_string()))?;
   347	
   348	    // initial QState
   349	    let initial_q_path = inputs.runtime_repo.join(INITIAL_Q_STATE_FILENAME);
   350	    let initial_q = if initial_q_path.exists() {
   351	        let s = std::fs::read_to_string(&initial_q_path)?;
   352	        serde_json::from_str(&s).map_err(|e| AuditError::ReplayBlocked(format!("initial_q: {e}")))?
   353	    } else {
   354	        QState::genesis()
   355	    };
   356	
   357	    // ledger entries
   358	    let writer = Git2LedgerWriter::open(&inputs.runtime_repo)
   359	        .map_err(|e| AuditError::ReplayBlocked(format!("git2 writer: {e}")))?;
   360	    let n = writer.len();
   361	    let mut entries = Vec::with_capacity(n as usize);
   362	    for t in 1..=n {
   363	        let entry = writer
   364	            .read_at(t)
   365	            .map_err(|e| AuditError::ReplayBlocked(format!("read_at {t}: {e}")))?;
   366	        entries.push(entry);
   367	    }
   368	
   369	    // CAS
   370	    let cas =
   371	        CasStore::open(&inputs.cas_dir).map_err(|e| AuditError::Cas(e.to_string()))?;
   372	
   373	    // L4.E
   374	    let rej_path = inputs.runtime_repo.join(REJECTIONS_JSONL_FILENAME);
   375	    let l4e_writer = if rej_path.exists() {
   376	        RejectionEvidenceWriter::open_jsonl(rej_path).map_err(AuditError::L4eOpen)?
   377	    } else {
   378	        RejectionEvidenceWriter::new()
   379	    };
   380	
   381	    // replay (best-effort; result captured for assertions)
   382	    let predicate_registry = PredicateRegistry::new();
   383	    let tool_registry = ToolRegistry::new();
   384	    let cas_view = CasStoreRef(&cas);
   385	    let (replayed_q, replay_error) = match replay_full_transition(
   386	        &initial_q,
   387	        &entries,
   388	        &cas_view,
   389	        &pinned,
   390	        &predicate_registry,
   391	        &tool_registry,
   392	    ) {
   393	        Ok(q) => (Some(q), None),
   394	        Err(e) => (None, Some(e)),
   395	    };
   396	
   397	    // constitution
   398	    let constitution_bytes = std::fs::read(&inputs.constitution)
   399	        .map_err(|e| AuditError::ConstitutionRead(format!("{:?}: {}", inputs.constitution, e)))?;
   400	    let constitution_hash = sha256_hash(&constitution_bytes);
   401	
   402	    // markov capsule (optional — chain may be pre-Markov)
   403	    let markov_capsule = read_markov_capsule(&inputs.markov_pointer, &cas).ok();
   404	
   405	    // genesis [constitution_root] hex (best-effort)
   406	    let genesis_constitution_root_hex = std::fs::read_to_string(&inputs.genesis)
   407	        .ok()
   408	        .and_then(|s| extract_constitution_root_hex(&s));
   409	
   410	    Ok(LoadedTape {
   411	        entries,
   412	        l4e_writer,
   413	        cas,
   414	        pinned,
   415	        pinned_manifest,
   416	        agent_manifest,
   417	        initial_q,
   418	        replayed_q,
   419	        replay_error,
   420	        constitution_bytes,
   421	        constitution_hash,
   422	        markov_capsule,
   423	        genesis_constitution_root_hex,
   424	    })
   425	}
   426	
   427	// ─────────────────────────────────────────────────────────────────────
   428	// Helpers
   429	// ─────────────────────────────────────────────────────────────────────
   430	

exec
/bin/bash -lc 'rg -n "InvalidSystemSignature|SignatureInvalid|AgentSignatureInvalid|system_signature_of|system_message_for_verification|verify_system_signature|verify_agent_signature|system_signature" src/state/sequencer.rs | head -n 120' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
350:        TE::InvalidSystemSignatureLive => RC::PolicyViolation,
389:        TransitionError::InvalidSystemSignatureLive => Some("invalid_system_signature_live".into()),
402:// `system_message_for_verification`: exhaustively matches the 4 system-emitted
404:// system_signature should bind to. Agent variants return `None`. The
409:// `system_signature_of` / `system_epoch_of`: extract the signature + epoch
418:fn system_message_for_verification(
463:fn system_signature_of(
467:        TypedTx::FinalizeReward(t) => Some(&t.system_signature),
468:        TypedTx::TaskExpire(t) => Some(&t.system_signature),
469:        TypedTx::TerminalSummary(t) => Some(&t.system_signature),
470:        TypedTx::ChallengeResolve(t) => Some(&t.system_signature),
471:        TypedTx::TaskBankruptcy(t) => Some(&t.system_signature),
1158:        // Anti-Oreo: arm fires only when system_signature verified at
2041:    AgentSignatureInvalid,
2049:            Self::AgentSignatureInvalid => write!(
2168:    InvalidSystemSignatureLive,
2187:            Self::InvalidSystemSignatureLive => write!(
2189:                "system_signature failed live verification against pinned pubkeys at emit time"
2332:    /// `system_signature` on system-emitted variants (defense-in-depth atop
2352:    /// pinned pubkeys; failed verification → `SubmitError::AgentSignatureInvalid`.
2489:        // are rejected pre-queue with `SubmitError::AgentSignatureInvalid`.
2494:            use crate::runtime::agent_keypairs::verify_agent_signature;
2499:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
2501:                    if verify_agent_signature(&mint.signature, &digest, &pubkey).is_err() {
2502:                        return Err(SubmitError::AgentSignatureInvalid);
2508:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
2510:                    if verify_agent_signature(&redeem.signature, &digest, &pubkey).is_err() {
2511:                        return Err(SubmitError::AgentSignatureInvalid);
2517:                        .ok_or(SubmitError::AgentSignatureInvalid)?;
2519:                    if verify_agent_signature(&seed.signature, &digest, &pubkey).is_err() {
2520:                        return Err(SubmitError::AgentSignatureInvalid);
2607:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
2614:                tx.system_signature = sig;
2656:                    system_signature: SystemSignature::from_bytes([0u8; 64]),  // placeholder
2663:                tx.system_signature = sig;
2700:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
2707:                tx.system_signature = sig;
2748:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
2755:                tx.system_signature = sig;
2790:                    system_signature: SystemSignature::from_bytes([0u8; 64]), // placeholder
2797:                tx.system_signature = sig;
2807:        use crate::bottom_white::ledger::system_keypair::{verify_system_signature, CanonicalMessage};
2812:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
2813:                    return Err(EmitSystemError::InvalidSystemSignatureLive);
2821:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
2822:                    return Err(EmitSystemError::InvalidSystemSignatureLive);
2830:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
2831:                    return Err(EmitSystemError::InvalidSystemSignatureLive);
2843:                if !verify_system_signature(&t.system_signature, &msg, self.epoch, &self.pinned_pubkeys) {
2844:                    return Err(EmitSystemError::InvalidSystemSignatureLive);
2852:                if !verify_system_signature(&t.system_signature, &msg, t.epoch, &self.pinned_pubkeys) {
2853:                    return Err(EmitSystemError::InvalidSystemSignatureLive);
3031:        // On verification failure, route to L4.E with InvalidSystemSignatureLive
3034:        if let Some(msg) = system_message_for_verification(&tx) {
3035:            use crate::bottom_white::ledger::system_keypair::verify_system_signature;
3036:            let sig = system_signature_of(&tx)
3037:                .expect("system_message_for_verification implies system_signature present");
3042:            if !verify_system_signature(sig, &msg, tx_epoch, &self.pinned_pubkeys) {
3043:                let err = TransitionError::InvalidSystemSignatureLive;
3137:        let system_signature = transition_ledger_emitter::sign_ledger_entry(
3157:            system_signature,
4338:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4362:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4385:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4407:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4500:    // are rejected with TransitionError::InvalidSystemSignatureLive at
4518:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4532:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4547:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4563:            system_signature: SystemSignature::from_bytes([0u8; 64]),
4577:            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
4578:            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
4595:            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
4596:            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
4610:            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
4611:            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
4625:            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {}
4626:            other => panic!("expected InvalidSystemSignatureLive, got {other:?}"),
4640:    /// other than InvalidSystemSignatureLive).
4656:            ApplyError::Transition(TransitionError::InvalidSystemSignatureLive) => {
4671:    /// "missing system_signature" errors when an agent variant is applied.
4676:        // we don't observe InvalidSystemSignatureLive (which would be a
4683:            Err(ApplyError::Transition(TransitionError::InvalidSystemSignatureLive)) => {
4744:            system_signature: SystemSignature::from_bytes([0u8; 64]),

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '1234,1380p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1234	// ─────────────────────────────────────────────────────────────────────
  1235	// Layer E — predicate / evidence integrity (5 assertions)
  1236	// ─────────────────────────────────────────────────────────────────────
  1237	
  1238	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1239	pub fn assert_23_accepted_work_predicate_results_true(t: &LoadedTape) -> AssertionResult {
  1240	    for (i, e) in t.entries.iter().enumerate() {
  1241	        if e.tx_kind != TxKind::Work {
  1242	            continue;
  1243	        }
  1244	        let bytes = match t.cas.get(&e.tx_payload_cid) {
  1245	            Ok(b) => b,
  1246	            Err(_) => {
  1247	                return AssertionResult::halt(
  1248	                    23,
  1249	                    "accepted_work_predicate_results_true",
  1250	                    AssertionLayer::E,
  1251	                    format!("CAS miss at index {i}"),
  1252	                );
  1253	            }
  1254	        };
  1255	        let typed: TypedTx = match canonical_decode(&bytes) {
  1256	            Ok(t) => t,
  1257	            Err(_) => continue,
  1258	        };
  1259	        if let TypedTx::Work(w) = typed {
  1260	            for (pid, bwp) in w.predicate_results.acceptance.iter() {
  1261	                if !bwp.value {
  1262	                    return AssertionResult::halt(
  1263	                        23,
  1264	                        "accepted_work_predicate_results_true",
  1265	                        AssertionLayer::E,
  1266	                        format!("WorkTx at index {i} has acceptance.{}=false", pid.0),
  1267	                    );
  1268	                }
  1269	            }
  1270	        }
  1271	    }
  1272	    AssertionResult::pass(23, "accepted_work_predicate_results_true", AssertionLayer::E)
  1273	}
  1274	
  1275	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1276	pub fn assert_24_proposal_telemetry_chain(t: &LoadedTape) -> AssertionResult {
  1277	    for (i, e) in t.entries.iter().enumerate() {
  1278	        if e.tx_kind != TxKind::Work {
  1279	            continue;
  1280	        }
  1281	        let bytes = match t.cas.get(&e.tx_payload_cid) {
  1282	            Ok(b) => b,
  1283	            Err(_) => continue,
  1284	        };
  1285	        let typed: TypedTx = match canonical_decode(&bytes) {
  1286	            Ok(t) => t,
  1287	            Err(_) => continue,
  1288	        };
  1289	        let work = match typed {
  1290	            TypedTx::Work(w) => w,
  1291	            _ => continue,
  1292	        };
  1293	        // proposal_cid must resolve to ProposalTelemetry
  1294	        let prop_bytes = match t.cas.get(&work.proposal_cid) {
  1295	            Ok(b) => b,
  1296	            Err(_) => {
  1297	                return AssertionResult::halt(
  1298	                    24,
  1299	                    "proposal_telemetry_chain",
  1300	                    AssertionLayer::E,
  1301	                    format!(
  1302	                        "proposal_cid {} not in CAS at L4 index {i}",
  1303	                        hex_encode(&work.proposal_cid.0)
  1304	                    ),
  1305	                );
  1306	            }
  1307	        };
  1308	        let telemetry: ProposalTelemetry = match canonical_decode::<ProposalTelemetry>(&prop_bytes) {
  1309	            Ok(p) => p,
  1310	            Err(_) => match serde_json::from_slice::<ProposalTelemetry>(&prop_bytes) {
  1311	                Ok(p) => p,
  1312	                Err(e2) => {
  1313	                    return AssertionResult::halt(
  1314	                        24,
  1315	                        "proposal_telemetry_chain",
  1316	                        AssertionLayer::E,
  1317	                        format!("ProposalTelemetry decode at L4 index {i}: {e2}"),
  1318	                    );
  1319	                }
  1320	            },
  1321	        };
  1322	        if let Some(vc) = telemetry.verification_result_cid {
  1323	            let vr_bytes = match t.cas.get(&vc) {
  1324	                Ok(b) => b,
  1325	                Err(_) => {
  1326	                    return AssertionResult::halt(
  1327	                        24,
  1328	                        "proposal_telemetry_chain",
  1329	                        AssertionLayer::E,
  1330	                        format!("verification_result_cid not in CAS at L4 index {i}"),
  1331	                    );
  1332	                }
  1333	            };
  1334	            let vr_opt: Option<VerificationResult> = canonical_decode(&vr_bytes)
  1335	                .ok()
  1336	                .or_else(|| serde_json::from_slice(&vr_bytes).ok());
  1337	            match vr_opt {
  1338	                Some(vr) if vr.verified => {}
  1339	                Some(_) => {
  1340	                    return AssertionResult::halt(
  1341	                        24,
  1342	                        "proposal_telemetry_chain",
  1343	                        AssertionLayer::E,
  1344	                        format!("VerificationResult.verified=false at L4 index {i}"),
  1345	                    );
  1346	                }
  1347	                None => {
  1348	                    return AssertionResult::halt(
  1349	                        24,
  1350	                        "proposal_telemetry_chain",
  1351	                        AssertionLayer::E,
  1352	                        format!("VerificationResult decode failed at L4 index {i}"),
  1353	                    );
  1354	                }
  1355	            }
  1356	        }
  1357	    }
  1358	    AssertionResult::pass(24, "proposal_telemetry_chain", AssertionLayer::E)
  1359	}
  1360	
  1361	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1362	pub fn assert_25_l4e_rejection_class_redispatch(_t: &LoadedTape) -> AssertionResult {
  1363	    // L4.E re-dispatch parity is captured at the sequencer integration
  1364	    // level (rejection_class is recorded when the rejected tx is fed
  1365	    // through dispatch_transition). A full re-dispatch loop here would
  1366	    // duplicate sequencer logic. Structural pass: L4.E chain integrity
  1367	    // (Layer B #6) already proves the recorded class is not tampered.
  1368	    AssertionResult::pass(25, "l4e_rejection_class_redispatch", AssertionLayer::E)
  1369	}
  1370	
  1371	/// TRACE_MATRIX FC1-N34 + FC2-N31 (TB-16 audit-from-tape battery).
  1372	pub fn assert_26_price_index_is_view_only(_t: &LoadedTape) -> AssertionResult {
  1373	    // Structural: PriceIndex is removed from EconomicState (TB-14
  1374	    // architectural fix; see q_state.rs line 179). The replayed
  1375	    // EconomicState struct has no `price_index_t` field; therefore
  1376	    // PriceIndex cannot be a state input. This is a source-level
  1377	    // invariant verified at compile time on `economic_state_t` shape.
  1378	    AssertionResult::pass(26, "price_index_is_view_only", AssertionLayer::E)
  1379	}
  1380	

exec
/bin/bash -lc "nl -ba tests/tb_16_halt_triggers.rs | sed -n '1,330p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	/// TB-16 Halt-Trigger Fixture (architect §7.7 + design §10 H1..H13)
     2	///
     3	/// 13 tests that must ALL be green before TB-16 ships.
     4	/// Atom 1 = stubs; Atom 2 backfills 12 (H1..H10 + H12 + H13);
     5	/// Atom 6 backfills H11 (Markov override binary fence).
     6	///
     7	/// Any atom that flips a green test to red = immediate halt (no round-2)
     8	/// per architect §7.7.
     9	///
    10	/// TRACE_MATRIX FC1-N34 + FC1-N35 + FC1-N36 + FC2-N31..N33 + FC3-N44
    11	
    12	use turingosv4::runtime::audit_assertions::{
    13	    AssertionLayer, AssertionResult, AssertionVerdict,
    14	};
    15	
    16	fn ok(r: &AssertionResult, expected_layer: AssertionLayer) {
    17	    assert!(
    18	        matches!(r.layer, l if l == expected_layer),
    19	        "halt-trigger expected layer {:?}; got {:?}",
    20	        expected_layer,
    21	        r.layer
    22	    );
    23	    assert!(
    24	        matches!(r.result, AssertionVerdict::Pass | AssertionVerdict::Skipped),
    25	        "halt-trigger {} `{}` MUST not fail/halt at fixture-time (got {:?}: {:?})",
    26	        r.id,
    27	        r.name,
    28	        r.result,
    29	        r.detail
    30	    );
    31	}
    32	
    33	// ────────────────────────────────────────────────────────────────────
    34	// H1  pinned-pubkey verify failure halts (Layer A #2 covers presence;
    35	// H1 is structural — verification path lives in #8)
    36	// ────────────────────────────────────────────────────────────────────
    37	#[test]
    38	fn h1_pinned_pubkey_verify_failure_halts() {
    39	    // Structural fence: the function `assert_08_system_tx_signatures_verify`
    40	    // must exist and have the right layer. The actual halt-on-tamper is
    41	    // exercised by audit_tape_tamper (Atom 3) over a constructed tape.
    42	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    43	    let layer = AssertionLayer::B;
    44	    assert!(matches!(layer, AssertionLayer::B));
    45	}
    46	
    47	// ────────────────────────────────────────────────────────────────────
    48	// H2  agent-pubkey verify failure halts
    49	// ────────────────────────────────────────────────────────────────────
    50	#[test]
    51	fn h2_agent_pubkey_verify_failure_halts() {
    52	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    53	    let layer = AssertionLayer::B;
    54	    assert!(matches!(layer, AssertionLayer::B));
    55	}
    56	
    57	// ────────────────────────────────────────────────────────────────────
    58	// H3  replay state_root mismatch halts
    59	// ────────────────────────────────────────────────────────────────────
    60	#[test]
    61	fn h3_replay_state_root_mismatch_halts() {
    62	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    63	    // assert_12_replay_state_root_matches_head returns Halt on divergence.
    64	    let layer = AssertionLayer::C;
    65	    assert!(matches!(layer, AssertionLayer::C));
    66	}
    67	
    68	// ────────────────────────────────────────────────────────────────────
    69	// H4  L4 hash chain broken link halts
    70	// ────────────────────────────────────────────────────────────────────
    71	#[test]
    72	fn h4_l4_hash_chain_broken_link_halts() {
    73	    // Fence: assert_04_l4_hash_chain_valid returns Halt on parent_state /
    74	    // parent_ledger / fold mismatch. The audit_tape_tamper binary
    75	    // (Atom 3) exercises this on real tampered bytes.
    76	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    77	    let layer = AssertionLayer::B;
    78	    assert!(matches!(layer, AssertionLayer::B));
    79	}
    80	
    81	// ────────────────────────────────────────────────────────────────────
    82	// H5  L4.E hash chain broken link halts
    83	// ────────────────────────────────────────────────────────────────────
    84	#[test]
    85	fn h5_l4e_hash_chain_broken_link_halts() {
    86	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    87	    let layer = AssertionLayer::B;
    88	    assert!(matches!(layer, AssertionLayer::B));
    89	}
    90	
    91	// ────────────────────────────────────────────────────────────────────
    92	// H6  L4.E entry advances logical_t/state_root halts
    93	// ────────────────────────────────────────────────────────────────────
    94	#[test]
    95	fn h6_l4e_advances_state_halts() {
    96	    use turingosv4::runtime::audit_assertions::AssertionLayer;
    97	    let layer = AssertionLayer::B;
    98	    assert!(matches!(layer, AssertionLayer::B));
    99	}
   100	
   101	// ────────────────────────────────────────────────────────────────────
   102	// H7  unresolved CAS Cid halts
   103	// ────────────────────────────────────────────────────────────────────
   104	#[test]
   105	fn h7_unresolved_cas_cid_halts() {
   106	    use turingosv4::runtime::audit_assertions::AssertionLayer;
   107	    let layer = AssertionLayer::B;
   108	    assert!(matches!(layer, AssertionLayer::B));
   109	}
   110	
   111	// ────────────────────────────────────────────────────────────────────
   112	// H8  AgentVisibleProjection contains autopsy private_detail bytes halts
   113	// (extends TB-15 halt-trigger #1)
   114	// ────────────────────────────────────────────────────────────────────
   115	#[test]
   116	fn h8_projection_contains_autopsy_private_detail_halts() {
   117	    // Re-affirm TB-15 halt-trigger #1: AgentVisibleProjection MUST NOT
   118	    // reference any autopsy types directly. Source-level fence.
   119	    let manifest = env!("CARGO_MANIFEST_DIR");
   120	    let q_state_path = format!("{}/src/state/q_state.rs", manifest);
   121	    let body = std::fs::read_to_string(&q_state_path)
   122	        .unwrap_or_else(|e| panic!("read {}: {}", q_state_path, e));
   123	    let needle = "pub struct AgentVisibleProjection";
   124	    let start = body.find(needle).expect("AgentVisibleProjection must exist");
   125	    let after = &body[start..];
   126	    let brace_open = after.find('{').expect("opening brace");
   127	    let mut depth = 0i32;
   128	    let mut end = brace_open;
   129	    for (i, ch) in after[brace_open..].char_indices() {
   130	        match ch {
   131	            '{' => depth += 1,
   132	            '}' => {
   133	                depth -= 1;
   134	                if depth == 0 {
   135	                    end = brace_open + i;
   136	                    break;
   137	                }
   138	            }
   139	            _ => {}
   140	        }
   141	    }
   142	    let projection_body = &after[brace_open..=end];
   143	    let forbidden: Vec<String> = vec![
   144	        format!("agent_autopsies{}", "_t"),
   145	        format!("Autopsy{}", "Index"),
   146	        format!("Agent{}", "AutopsyCapsule"),
   147	        format!("private_detail_{}", "cid"),
   148	    ];
   149	    for tok in &forbidden {
   150	        assert!(
   151	            !projection_body.contains(tok.as_str()),
   152	            "H8: AgentVisibleProjection MUST NOT reference autopsy type `{}`",
   153	            tok
   154	        );
   155	    }
   156	}
   157	
   158	// ────────────────────────────────────────────────────────────────────
   159	// H9  TypicalErrorSummary contains private_detail_cid halts
   160	// ────────────────────────────────────────────────────────────────────
   161	#[test]
   162	fn h9_typical_error_summary_contains_private_detail_halts() {
   163	    // Re-affirm TB-15 halt-trigger #5 via the assertion module's
   164	    // assert_30_typical_error_summary_no_private_detail. Structural
   165	    // fence: cluster_autopsies output must not contain raw 32-byte
   166	    // run of any private_detail_cid.
   167	    use turingosv4::bottom_white::cas::schema::Cid;
   168	    use turingosv4::economy::money::MicroCoin;
   169	    use turingosv4::runtime::autopsy_capsule::{
   170	        cluster_autopsies, AgentAutopsyCapsule, LossReasonClass,
   171	    };
   172	    use turingosv4::state::q_state::{AgentId, Hash, TaskId};
   173	    use turingosv4::state::typed_tx::{CapsulePrivacyPolicy, EventId};
   174	
   175	    let event = EventId(TaskId("task:tb16:h9".into()));
   176	    let mk = |agent: &str, b: u8| AgentAutopsyCapsule {
   177	        capsule_id: Cid::from_content(agent.as_bytes()),
   178	        agent_id: AgentId(agent.to_string()),
   179	        event_id: event.clone(),
   180	        loss_amount: MicroCoin::from_micro_units(1_000),
   181	        loss_reason_class: LossReasonClass::Bankruptcy,
   182	        violated_risk_rule: None,
   183	        suggested_policy_patch: None,
   184	        evidence_cids: vec![],
   185	        public_summary: format!("agent={} reason=Bankruptcy", agent),
   186	        private_detail_cid: Cid([b; 32]),
   187	        privacy_policy: CapsulePrivacyPolicy::AuditOnly,
   188	        sha256: Hash::ZERO,
   189	        created_at_logical_t: 1,
   190	        created_at_round: 0,
   191	    };
   192	    let bytes = [0xA1u8, 0xA2, 0xA3];
   193	    let autopsies = vec![mk("Agent_solver_0", bytes[0]), mk("Agent_solver_1", bytes[1]), mk("Agent_solver_2", bytes[2])];
   194	    let summaries = cluster_autopsies(&autopsies, 3);
   195	    assert_eq!(summaries.len(), 1);
   196	    let canonical = turingosv4::bottom_white::ledger::transition_ledger::canonical_encode(&summaries)
   197	        .expect("canonical_encode");
   198	    for &b in &bytes {
   199	        let run = [b; 32];
   200	        for window in canonical.windows(32) {
   201	            assert!(window != run, "H9: canonical encode contains private_detail_cid run for byte 0x{:02x}", b);
   202	        }
   203	    }
   204	}
   205	
   206	// ────────────────────────────────────────────────────────────────────
   207	// H10  Markov constitution_hash mismatch halts
   208	// ────────────────────────────────────────────────────────────────────
   209	#[test]
   210	fn h10_markov_constitution_hash_mismatch_halts() {
   211	    use sha2::{Digest, Sha256};
   212	    use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
   213	    let manifest = env!("CARGO_MANIFEST_DIR");
   214	    let constitution_path = format!("{}/constitution.md", manifest);
   215	    let bytes = std::fs::read(&constitution_path).expect("constitution");
   216	    let mut h = Sha256::new();
   217	    h.update(&bytes);
   218	    let expected: [u8; 32] = h.finalize().into();
   219	    let cap = MarkovEvidenceCapsule::with_constitution_hash(expected);
   220	    assert_eq!(cap.constitution_hash.0, expected, "H10: Markov capsule constitution_hash must match sha256(constitution.md)");
   221	}
   222	
   223	// ────────────────────────────────────────────────────────────────────
   224	// H11  Markov deep-history without override halts (binary-level fence)
   225	// Filled by Atom 6 (real-LLM smoke) — for now, structural fence only.
   226	// ────────────────────────────────────────────────────────────────────
   227	#[test]
   228	fn h11_markov_deep_history_without_override_halts() {
   229	    use turingosv4::runtime::markov_capsule::{
   230	        try_deep_history_read_with_override_check, MarkovGenError,
   231	    };
   232	    let r = try_deep_history_read_with_override_check(false);
   233	    assert!(matches!(r, Err(MarkovGenError::DeepHistoryReadDenied)),
   234	        "H11: deep-history default-deny must return DeepHistoryReadDenied without override");
   235	    let ok_path = try_deep_history_read_with_override_check(true);
   236	    assert!(ok_path.is_ok(), "H11: TURINGOS_MARKOV_OVERRIDE=1 must permit deep-history");
   237	}
   238	
   239	// ────────────────────────────────────────────────────────────────────
   240	// H12  LLM self-narrative in autopsy evidence_cids halts
   241	// ────────────────────────────────────────────────────────────────────
   242	#[test]
   243	fn h12_llm_self_narrative_in_autopsy_evidence_halts() {
   244	    // Fence: assert_f_no_llm_self_narrative_in_autopsy halts when an
   245	    // autopsy.evidence_cid resolves to a CAS object with ObjectType::ProposalPayload.
   246	    // Source-level fence: confirm the assertion exists in the module.
   247	    use turingosv4::runtime::audit_assertions::AssertionLayer;
   248	    let layer = AssertionLayer::F;
   249	    assert!(matches!(layer, AssertionLayer::F));
   250	    // also verify autopsy_capsule.rs has no path that adds ProposalPayload
   251	    // Cids to evidence_cids (would need source scan; deferred to audit_tape
   252	    // smoke runtime check on real tape).
   253	}
   254	
   255	// ────────────────────────────────────────────────────────────────────
   256	// H13  total_supply_micro mutates across L4 rows halts
   257	// ────────────────────────────────────────────────────────────────────
   258	#[test]
   259	fn h13_total_supply_mutates_halts() {
   260	    // Fence: assert_18_total_supply_conserved halts on total_supply
   261	    // divergence from genesis 30_000_000μC. Layer D verified at
   262	    // audit_tape time. Source-level fence: GENESIS_TOTAL_MICRO is
   263	    // unmoved.
   264	    use turingosv4::runtime::audit_assertions::AssertionLayer;
   265	    let layer = AssertionLayer::D;
   266	    assert!(matches!(layer, AssertionLayer::D));
   267	
   268	    // Genesis preseed total = 30_000_000 (verified by bootstrap module).
   269	    use turingosv4::runtime::bootstrap::default_pput_preseed_pairs;
   270	    let total: i64 = default_pput_preseed_pairs()
   271	        .iter()
   272	        .map(|(_, mc)| mc.micro_units())
   273	        .sum();
   274	    assert_eq!(total, 30_000_000, "H13: genesis preseed total micro must equal 30_000_000μC");
   275	}
   276	
   277	// helper to silence unused imports in trivial tests
   278	fn _suppress_unused() {
   279	    let _ = ok;
   280	}

exec
/bin/bash -lc "nl -ba tests/tb_16_audit_tape_binary.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-16 Atom 3 — `audit_tape` + `audit_tape_tamper` binary smoke test.
     2	//!
     3	//! Runs both binaries against an existing chain-backed smoke evidence
     4	//! directory (TB-13 real-LLM smoke). Asserts:
     5	//!
     6	//! - audit_tape produces a verdict.json with the expected schema and
     7	//!   the expected tape_root counts.
     8	//! - audit_tape_tamper detects 3/3 corruptions (BLOCK verdict on each
     9	//!   tampered copy).
    10	//!
    11	//! TRACE_MATRIX FC1-N34 + FC1-N35.
    12	
    13	use std::path::PathBuf;
    14	use std::process::Command;
    15	
    16	fn manifest_dir() -> PathBuf {
    17	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    18	}
    19	
    20	fn target_bin(name: &str) -> PathBuf {
    21	    let manifest = manifest_dir();
    22	    let dbg = manifest.join("target").join("debug").join(name);
    23	    if dbg.exists() {
    24	        return dbg;
    25	    }
    26	    let rel = manifest.join("target").join("release").join(name);
    27	    if rel.exists() {
    28	        return rel;
    29	    }
    30	    panic!("binary {name} not built (run cargo build --bin {name})");
    31	}
    32	
    33	fn fixture_smoke_dir() -> Option<PathBuf> {
    34	    let p = manifest_dir()
    35	        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    36	    if p.is_dir() && p.join("runtime_repo").is_dir() && p.join("cas").is_dir() {
    37	        Some(p)
    38	    } else {
    39	        None
    40	    }
    41	}
    42	
    43	#[test]
    44	fn audit_tape_binary_help_succeeds() {
    45	    let bin = target_bin("audit_tape");
    46	    let out = Command::new(&bin)
    47	        .arg("--help")
    48	        .output()
    49	        .expect("audit_tape --help");
    50	    assert!(
    51	        out.status.success() || out.status.code() == Some(0),
    52	        "audit_tape --help should succeed; got status {:?}",
    53	        out.status
    54	    );
    55	    let stderr = String::from_utf8_lossy(&out.stderr);
    56	    let stdout = String::from_utf8_lossy(&out.stdout);
    57	    let combined = format!("{stderr}{stdout}");
    58	    assert!(
    59	        combined.contains("audit_tape") && combined.contains("USAGE"),
    60	        "help text malformed"
    61	    );
    62	}
    63	
    64	#[test]
    65	fn audit_tape_tamper_binary_help_succeeds() {
    66	    let bin = target_bin("audit_tape_tamper");
    67	    let out = Command::new(&bin)
    68	        .arg("--help")
    69	        .output()
    70	        .expect("audit_tape_tamper --help");
    71	    let stderr = String::from_utf8_lossy(&out.stderr);
    72	    assert!(
    73	        stderr.contains("audit_tape_tamper") && stderr.contains("USAGE"),
    74	        "audit_tape_tamper help malformed: {stderr}"
    75	    );
    76	}
    77	
    78	#[test]
    79	fn audit_tape_runs_on_existing_chain_smoke() {
    80	    let Some(smoke) = fixture_smoke_dir() else {
    81	        eprintln!("skipping: no fixture chain at handover/evidence/tb_13_real_llm_smoke_*; binary still builds");
    82	        return;
    83	    };
    84	    let bin = target_bin("audit_tape");
    85	    let out_path = std::env::temp_dir().join("tb_16_audit_tape_smoke_verdict.json");
    86	    let _ = std::fs::remove_file(&out_path);
    87	    let manifest = manifest_dir();
    88	    let status = Command::new(&bin)
    89	        .arg("--runtime-repo")
    90	        .arg(smoke.join("runtime_repo"))
    91	        .arg("--cas-dir")
    92	        .arg(smoke.join("cas"))
    93	        .arg("--agent-pubkeys")
    94	        .arg(smoke.join("runtime_repo/agent_pubkeys.json"))
    95	        .arg("--pinned-pubkeys")
    96	        .arg(smoke.join("runtime_repo/pinned_pubkeys.json"))
    97	        .arg("--genesis")
    98	        .arg(manifest.join("genesis_payload.toml"))
    99	        .arg("--constitution")
   100	        .arg(manifest.join("constitution.md"))
   101	        .arg("--markov-pointer")
   102	        .arg(manifest.join("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt"))
   103	        .arg("--alignment-dir")
   104	        .arg(manifest.join("handover/alignment"))
   105	        .arg("--out")
   106	        .arg(&out_path)
   107	        .status()
   108	        .expect("audit_tape spawn");
   109	    assert!(
   110	        out_path.exists(),
   111	        "audit_tape did not write verdict.json (status={:?})",
   112	        status
   113	    );
   114	    let verdict_text = std::fs::read_to_string(&out_path).expect("read verdict");
   115	    let v: serde_json::Value = serde_json::from_str(&verdict_text).expect("parse verdict");
   116	    assert_eq!(v["schema_version"], "v1/audit_tape_verdict");
   117	    assert!(v["assertions"].as_array().expect("assertions array").len() >= 38);
   118	    assert!(v["tape_root"]["l4_count"].as_u64().expect("l4_count u64") >= 1);
   119	    let verdict = v["verdict"].as_str().expect("verdict str");
   120	    assert!(
   121	        verdict == "PROCEED" || verdict == "BLOCK",
   122	        "unexpected verdict: {verdict}"
   123	    );
   124	}

exec
/bin/bash -lc "nl -ba tests/tb_16_dashboard_live_regen.rs | sed -n '1,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-16 Atom 4 — dashboard §15 live regen + §16 sandbox banner smoke.
     2	//!
     3	//! Closes OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04: the dashboard
     4	//! `audit_dashboard` binary now reconstructs `EconomicState.agent_autopsies_t`
     5	//! via `replay_full_transition` and surfaces non-zero
     6	//! `autopsy_event_counts` whenever the chain has at least one
     7	//! TaskBankruptcyTx → autopsy emission.
     8	//!
     9	//! TB-16 Atom 4 §16: dashboard renders SANDBOX banner when any agent_id
    10	//! matches a sandbox-only prefix.
    11	//!
    12	//! TRACE_MATRIX FC2-N32 + FC2-N33.
    13	
    14	use std::path::PathBuf;
    15	use std::process::Command;
    16	
    17	fn manifest_dir() -> PathBuf {
    18	    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    19	}
    20	
    21	fn target_bin(name: &str) -> PathBuf {
    22	    let manifest = manifest_dir();
    23	    let dbg = manifest.join("target").join("debug").join(name);
    24	    if dbg.exists() {
    25	        return dbg;
    26	    }
    27	    panic!("binary {name} not built");
    28	}
    29	
    30	#[test]
    31	fn dashboard_renders_section_16_sandbox_banner_on_existing_smoke() {
    32	    let smoke = manifest_dir()
    33	        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    34	    if !smoke.is_dir() || !smoke.join("runtime_repo").is_dir() {
    35	        eprintln!("skipping: no fixture chain at handover/evidence/tb_13_real_llm_smoke_*");
    36	        return;
    37	    }
    38	    let bin = target_bin("audit_dashboard");
    39	    let out = Command::new(&bin)
    40	        .arg("--repo")
    41	        .arg(smoke.join("runtime_repo"))
    42	        .arg("--cas")
    43	        .arg(smoke.join("cas"))
    44	        .output()
    45	        .expect("audit_dashboard run");
    46	    let stdout = String::from_utf8_lossy(&out.stdout);
    47	    // §15 banner must always render (existing TB-15 contract)
    48	    assert!(stdout.contains("§15 TB-15 Autopsy + Markov"),
    49	            "§15 missing from dashboard output");
    50	    // §16 SANDBOX banner must render when sandbox prefix matched
    51	    let has_sandbox_id = stdout.contains("Agent_solver_")
    52	        || stdout.contains("tb7-7-sponsor")
    53	        || stdout.contains("Agent_verifier_")
    54	        || stdout.contains("Agent_user_");
    55	    if has_sandbox_id {
    56	        assert!(
    57	            stdout.contains("§16 TB-16 SANDBOX BANNER")
    58	                && stdout.contains("SANDBOX-RUN"),
    59	            "§16 banner missing despite sandbox-prefix agent IDs in tape"
    60	        );
    61	    }
    62	}
    63	
    64	#[test]
    65	fn rebuild_autopsy_event_counts_returns_empty_on_pre_tb15_chain() {
    66	    // For chains predating TB-15 (no agent_autopsies_t population),
    67	    // rebuild_autopsy_event_counts must return an empty Vec — not panic
    68	    // and not return synthetic data. The TB-13 real-LLM smoke is a good
    69	    // fixture: its chain has no TaskBankruptcyTx, so the replayed
    70	    // EconomicState's agent_autopsies_t is empty.
    71	    let smoke = manifest_dir()
    72	        .join("handover/evidence/tb_13_real_llm_smoke_2026-05-03/single_n1_mathd_algebra_171");
    73	    if !smoke.is_dir() {
    74	        eprintln!("skipping: no fixture chain");
    75	        return;
    76	    }
    77	    let bin = target_bin("audit_dashboard");
    78	    let out = Command::new(&bin)
    79	        .arg("--repo")
    80	        .arg(smoke.join("runtime_repo"))
    81	        .arg("--cas")
    82	        .arg(smoke.join("cas"))
    83	        .output()
    84	        .expect("audit_dashboard run");
    85	    let stdout = String::from_utf8_lossy(&out.stdout);
    86	    // TB-13 chain has no autopsies → §15 should report the empty branch
    87	    // text, NOT a populated table (and NOT the failure banner).
    88	    assert!(
    89	        stdout.contains("§15 TB-15 Autopsy + Markov"),
    90	        "§15 missing"
    91	    );
    92	    assert!(
    93	        stdout.contains("no agent_autopsies_t entries")
    94	            || stdout.contains("No autopsies recorded"),
    95	        "expected 'no autopsy entries' branch on TB-13 chain; got:\n{stdout}"
    96	    );
    97	}

exec
/bin/bash -lc "ls -l handover/markov_capsules handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke | sed -n '1,200p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke:
total 72
-rw-r--r-- 1 zephryj zephryj    64 May  4 04:48 LATEST_MARKOV_CAPSULE.txt
-rw-r--r-- 1 zephryj zephryj  5235 May  4 04:48 MARKOV_TB-16_2026-05-03.json
drwxr-xr-x 3 zephryj zephryj  4096 May  4 04:48 cas
-rw-r--r-- 1 zephryj zephryj  9067 May  4 04:48 dashboard.txt
drwxr-xr-x 3 zephryj zephryj  4096 May  4 04:48 runtime_repo
drwxr-xr-x 5 zephryj zephryj  4096 May  4 04:48 tamper
-rw-r--r-- 1 zephryj zephryj 19058 May  4 04:48 tamper_report.json
-rw-r--r-- 1 zephryj zephryj  7047 May  4 04:48 verdict.json
-rw-r--r-- 1 zephryj zephryj  7047 May  4 04:48 verdict_replay.json

handover/markov_capsules:
total 12
-rw-r--r-- 1 zephryj zephryj   64 May  4 02:41 LATEST_MARKOV_CAPSULE.txt
-rw-r--r-- 1 zephryj zephryj 5244 May  4 02:41 MARKOV_TB-15-R3_2026-05-03.json

exec
/bin/bash -lc "printf 'root pointer: '; test -f handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt && cat handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt || true; printf '\\nlocal pointer: '; test -f handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/LATEST_MARKOV_CAPSULE.txt && cat handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/LATEST_MARKOV_CAPSULE.txt || true; printf '\\n'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
root pointer: f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312
local pointer: 5da536022b4c56d0ed97a01f6b7aaf0a8d6c451a33f9ca0330b79acba3c5f6d8

exec
/bin/bash -lc "python3 - <<'PY'
import json, pathlib, glob
paths=['handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json']
paths += sorted(glob.glob('handover/markov_capsules/MARKOV_TB-15*.json'))[:5]
paths += sorted(glob.glob('handover/markov_capsules/MARKOV_TB-16*.json'))[:5]
for p in paths:
 print('\\n',p)
 try:
  v=json.load(open(p));
  for k in ['tb_id','capsule_id','previous_capsule_cid','constitution_hash','typical_errors','unresolved_obs','next_session_context_cid']:
   print(k, v.get(k))
 except Exception as e: print('ERR',e)
PY" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:

 handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json
tb_id None
capsule_id [93, 165, 54, 2, 43, 76, 86, 208, 237, 151, 160, 31, 107, 122, 175, 10, 141, 108, 69, 26, 51, 249, 202, 3, 48, 183, 154, 203, 163, 197, 246, 216]
previous_capsule_cid None
constitution_hash [238, 198, 149, 69, 156, 113, 251, 239, 54, 133, 88, 52, 133, 222, 180, 49, 254, 59, 86, 22, 87, 178, 242, 133, 183, 197, 231, 226, 32, 229, 158, 3]
typical_errors []
unresolved_obs ['handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md', 'handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md', 'handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md', 'handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md', 'handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md', 'handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md', 'handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md', 'handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md', 'handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md', 'handover/alignment/OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md', 'handover/alignment/OBS_ROADMAP_POST_TB7_OVERRIDE_2026-05-01.md', 'handover/alignment/OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md', 'handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md', 'handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md', 'handover/alignment/OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md', 'handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md', 'handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md', 'handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md', 'handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md', 'handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md']
next_session_context_cid [36, 222, 149, 213, 63, 128, 90, 219, 91, 149, 112, 117, 94, 75, 191, 96, 89, 178, 119, 189, 54, 120, 191, 57, 155, 6, 170, 126, 210, 54, 72, 128]

 handover/markov_capsules/MARKOV_TB-15-R3_2026-05-03.json
tb_id None
capsule_id [249, 231, 1, 180, 169, 194, 225, 217, 180, 209, 34, 44, 6, 166, 196, 228, 246, 81, 106, 161, 175, 28, 62, 210, 154, 244, 87, 209, 85, 50, 211, 18]
previous_capsule_cid None
constitution_hash [238, 198, 149, 69, 156, 113, 251, 239, 54, 133, 88, 52, 133, 222, 180, 49, 254, 59, 86, 22, 87, 178, 242, 133, 183, 197, 231, 226, 32, 229, 158, 3]
typical_errors []
unresolved_obs ['handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md', 'handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md', 'handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md', 'handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md', 'handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md', 'handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md', 'handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md', 'handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md', 'handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md', 'handover/alignment/OBS_RESOLUTIONS_INDEX_TB15_2026-05-03.md', 'handover/alignment/OBS_ROADMAP_POST_TB7_OVERRIDE_2026-05-01.md', 'handover/alignment/OBS_STATE_TRANSITION_SPEC_V1_5_HOUSEKEEPING_2026-04-29.md', 'handover/alignment/OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md', 'handover/alignment/OBS_TB13_AUDIT_RESIDUAL_CHALLENGES_2026-05-03.md', 'handover/alignment/OBS_TB13_FENCE_MECHANISM_DOOM_LOOP_2026-05-03.md', 'handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md', 'handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md', 'handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md', 'handover/alignment/OBS_TB_15_DASHBOARD_LIVE_REGEN_TB16_2026-05-04.md', 'handover/alignment/OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md', 'handover/alignment/OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md']
next_session_context_cid [83, 59, 212, 123, 234, 85, 246, 138, 24, 84, 67, 246, 164, 110, 226, 66, 202, 12, 167, 153, 115, 246, 202, 111, 234, 149, 41, 202, 26, 234, 17, 56]

exec
/bin/bash -lc 'rg -n "previous_capsule_cid|LATEST_MARKOV_CAPSULE|previous|MarkovEvidenceCapsule|with_constitution_hash|latest" src/runtime/markov_capsule.rs src/bin/generate_markov_capsule.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/bin/generate_markov_capsule.rs:8://! `cluster_autopsies`); writes a `MarkovEvidenceCapsule` to CAS +
src/bin/generate_markov_capsule.rs:14://! + previous Markov capsule + current chain heads are read.
src/bin/generate_markov_capsule.rs:60:    /// prior Markov capsules (deeper than the previous_capsule_cid) — a
src/bin/generate_markov_capsule.rs:151:        "TB-15 generate_markov_capsule — write a MarkovEvidenceCapsule to CAS \
src/bin/generate_markov_capsule.rs:220:    // previous_capsule_cid), enforce TURINGOS_MARKOV_OVERRIDE=1 BEFORE
src/bin/generate_markov_capsule.rs:290:    // Step 5: previous capsule Cid.
src/bin/generate_markov_capsule.rs:291:    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
src/bin/generate_markov_capsule.rs:305:        use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
src/bin/generate_markov_capsule.rs:310:            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
src/bin/generate_markov_capsule.rs:316:                "4. read CAS<previous_markov_capsule_cid> (if present)",
src/bin/generate_markov_capsule.rs:323:        let mut cap = MarkovEvidenceCapsule {
src/bin/generate_markov_capsule.rs:325:            previous_capsule_cid,
src/bin/generate_markov_capsule.rs:354:            previous_capsule_cid,
src/bin/generate_markov_capsule.rs:376:    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
src/bin/generate_markov_capsule.rs:385:    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");
src/bin/generate_markov_capsule.rs:386:    std::fs::write(&latest_path, capsule.capsule_id.hex())
src/bin/generate_markov_capsule.rs:387:        .map_err(|e| format!("write latest pointer: {e}"))?;
src/bin/generate_markov_capsule.rs:390:    eprintln!("wrote {}", latest_path.display());
src/runtime/markov_capsule.rs:1://! TB-15 Atom 5 — `MarkovEvidenceCapsule` schema + writer + default-deny
src/runtime/markov_capsule.rs:5://! CAS root + previous capsule + typical_errors + unresolved_obs +
src/runtime/markov_capsule.rs:47:/// **Markov chain**: `previous_capsule_cid` points to the prior capsule
src/runtime/markov_capsule.rs:50:/// rows pre-dating `previous_capsule_cid`'s `l4_root`) requires
src/runtime/markov_capsule.rs:53:pub struct MarkovEvidenceCapsule {
src/runtime/markov_capsule.rs:58:    /// Cid of the previous Markov capsule in the chain. `None` for the
src/runtime/markov_capsule.rs:60:    pub previous_capsule_cid: Option<Cid>,
src/runtime/markov_capsule.rs:94:    /// boot context (`{constitution_hash, latest_markov_cid, boot_seq}`).
src/runtime/markov_capsule.rs:107:impl Default for MarkovEvidenceCapsule {
src/runtime/markov_capsule.rs:111:            previous_capsule_cid: None,
src/runtime/markov_capsule.rs:127:impl MarkovEvidenceCapsule {
src/runtime/markov_capsule.rs:132:    pub fn with_constitution_hash(hash_bytes: [u8; 32]) -> Self {
src/runtime/markov_capsule.rs:211:/// TRACE_MATRIX TB-15 Atom 5: write a `MarkovEvidenceCapsule` to CAS.
src/runtime/markov_capsule.rs:220:///    as `ObjectType::MarkovEvidenceCapsule`.
src/runtime/markov_capsule.rs:229:    previous_capsule_cid: Option<Cid>,
src/runtime/markov_capsule.rs:240:) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
src/runtime/markov_capsule.rs:250:        "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
src/runtime/markov_capsule.rs:256:            "4. read CAS<previous_markov_capsule_cid> (if present)",
src/runtime/markov_capsule.rs:271:    let mut capsule = MarkovEvidenceCapsule {
src/runtime/markov_capsule.rs:273:        previous_capsule_cid,
src/runtime/markov_capsule.rs:296:    // sha256=Hash::ZERO). The in-memory `MarkovEvidenceCapsule`
src/runtime/markov_capsule.rs:310:        ObjectType::MarkovEvidenceCapsule,
src/runtime/markov_capsule.rs:329:/// `MarkovEvidenceCapsule` from CAS-resident bytes. Caller supplies the
src/runtime/markov_capsule.rs:341:) -> Result<MarkovEvidenceCapsule, MarkovGenError> {
src/runtime/markov_capsule.rs:343:    let mut cap: MarkovEvidenceCapsule = canonical_decode(bytes)
src/runtime/markov_capsule.rs:464:        let c = MarkovEvidenceCapsule::default();
src/runtime/markov_capsule.rs:466:        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");
src/runtime/markov_capsule.rs:470:    /// TB-15 Atom 5 — with_constitution_hash plumbs the hash through.
src/runtime/markov_capsule.rs:472:    fn with_constitution_hash_sets_field() {
src/runtime/markov_capsule.rs:474:        let c = MarkovEvidenceCapsule::with_constitution_hash(hash);
src/runtime/markov_capsule.rs:569:    /// via `previous_capsule_cid`.
src/runtime/markov_capsule.rs:571:    fn markov_chain_links_via_previous_capsule_cid() {
src/runtime/markov_capsule.rs:609:        assert_eq!(cap_2.previous_capsule_cid, Some(cap_1.capsule_id));
src/runtime/markov_capsule.rs:672:    /// "next session can bootstrap from latest Markov capsule" gate
src/runtime/markov_capsule.rs:732:        let mut c = MarkovEvidenceCapsule::default();
src/runtime/markov_capsule.rs:740:        let back: MarkovEvidenceCapsule = canonical_decode(&bytes).expect("decode");

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '270,395p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   270	        .map_err(|e| format!("read flowchart hashes: {e}"))?;
   271	    eprintln!("flowchart_hashes.len = {}", flowchart_hashes.len());
   272	
   273	    // Step 2: L4 / L4.E / CAS roots — for v0, accept zero placeholders
   274	    // when --runtime-repo/--cas-dir are absent (fresh-repo path) and
   275	    // populate from CAS metadata digest when present. Future TB will
   276	    // wire to the actual chain head readers; v0 ships the substrate.
   277	    let l4_root = Hash::ZERO;
   278	    let l4e_root = Hash::ZERO;
   279	
   280	    // Step 3: scan OBS files.
   281	    let unresolved_obs: Vec<ObsId> = scan_unresolved_obs(&args.alignment_dir)
   282	        .map_err(|e| format!("scan OBS: {e}"))?;
   283	    eprintln!("unresolved_obs.len = {}", unresolved_obs.len());
   284	
   285	    // Step 4: typical_errors — v0 accepts empty (no chain-resident
   286	    // capsules in dry-run) and TB-16+ wires to actual cluster_autopsies
   287	    // over CAS-resident AgentAutopsyCapsule objects.
   288	    let typical_errors: Vec<TypicalErrorSummary> = Vec::new();
   289	
   290	    // Step 5: previous capsule Cid.
   291	    let previous_capsule_cid: Option<Cid> = match &args.prev_cid_hex {
   292	        Some(s) => Some(parse_cid_hex(s)?),
   293	        None => None,
   294	    };
   295	
   296	    // Step 6: write capsule. Two modes:
   297	    //   (a) --no-cas: build the capsule struct directly + skip CAS put.
   298	    //       Used when no runtime CAS is available (fresh repo).
   299	    //   (b) default: open `--cas-dir` as a CasStore + put.
   300	    let cas_root = Hash::ZERO; // v0 placeholder; future wire-in via CAS metadata digest.
   301	    let capsule = if args.no_cas {
   302	        eprintln!("generate_markov_capsule: --no-cas mode — JSON pointer only");
   303	        // Compute capsule_id deterministically without CAS write.
   304	        use turingosv4::bottom_white::ledger::transition_ledger::canonical_encode;
   305	        use turingosv4::runtime::markov_capsule::MarkovEvidenceCapsule;
   306	        let next_session_json = serde_json::json!({
   307	            "schema_version": "v1/next_session_context",
   308	            "constitution_hash_hex": hex32(&constitution_hash.0),
   309	            "flowchart_hashes_hex": flowchart_hashes.iter().map(|h| hex32(&h.0)).collect::<Vec<_>>(),
   310	            "previous_markov_cid_hex": previous_capsule_cid.map(|c| c.hex()),
   311	            "tb_tag": format!("TB-{}", args.tb_id),
   312	            "boot_seq": [
   313	                "1. read constitution.md (verify sha256 == constitution_hash)",
   314	                "2. read TRACE_FLOWCHART_MATRIX.md (verify each flowchart sha256 == flowchart_hashes[i])",
   315	                "3. read CAS<this_markov_capsule_cid>",
   316	                "4. read CAS<previous_markov_capsule_cid> (if present)",
   317	                "5. DEFAULT-DENY deeper history; set TURINGOS_MARKOV_OVERRIDE=1 to override (audit-only)"
   318	            ],
   319	        });
   320	        let next_session_bytes = serde_json::to_vec(&next_session_json)
   321	            .map_err(|e| format!("next_session_context encode: {e}"))?;
   322	        let next_session_context_cid = Cid::from_content(&next_session_bytes);
   323	        let mut cap = MarkovEvidenceCapsule {
   324	            capsule_id: Cid::default(),
   325	            previous_capsule_cid,
   326	            constitution_hash,
   327	            flowchart_hashes: flowchart_hashes.clone(),
   328	            l4_root,
   329	            l4e_root,
   330	            cas_root,
   331	            typical_errors,
   332	            unresolved_obs,
   333	            next_session_context_cid,
   334	            sha256: Hash::ZERO,
   335	            created_at_logical_t: 0,
   336	            tb_tag: format!("TB-{}", args.tb_id),
   337	        };
   338	        let prelim_bytes = canonical_encode(&cap)
   339	            .map_err(|e| format!("capsule prelim encode: {e:?}"))?;
   340	        let cid = Cid::from_content(&prelim_bytes);
   341	        cap.capsule_id = cid;
   342	        cap.sha256 = Hash(cid.0);
   343	        cap
   344	    } else {
   345	        let cas_dir = args
   346	            .cas_dir
   347	            .as_ref()
   348	            .ok_or_else(|| "--cas-dir required without --no-cas".to_string())?;
   349	        let cas = Arc::new(RwLock::new(
   350	            CasStore::open(cas_dir).map_err(|e| format!("open CAS: {e}"))?,
   351	        ));
   352	        write_markov_capsule(
   353	            &cas,
   354	            previous_capsule_cid,
   355	            constitution_hash,
   356	            flowchart_hashes,
   357	            l4_root,
   358	            l4e_root,
   359	            cas_root,
   360	            typical_errors,
   361	            unresolved_obs,
   362	            format!("TB-{}", args.tb_id),
   363	            "tb15-generator",
   364	            0,
   365	        )
   366	        .map_err(|e| match e {
   367	            MarkovGenError::DeepHistoryReadDenied => {
   368	                "deep-history read denied (set TURINGOS_MARKOV_OVERRIDE=1)".to_string()
   369	            }
   370	            other => format!("write_markov_capsule: {other}"),
   371	        })?
   372	    };
   373	
   374	    eprintln!("capsule_id = {}", capsule.capsule_id.hex());
   375	
   376	    // Step 7: emit JSON pointer file + LATEST_MARKOV_CAPSULE.txt.
   377	    std::fs::create_dir_all(&args.out_dir)
   378	        .map_err(|e| format!("create out_dir: {e}"))?;
   379	    let json_path = args
   380	        .out_dir
   381	        .join(format!("MARKOV_TB-{}_2026-05-03.json", args.tb_id));
   382	    let json_body = serde_json::to_string_pretty(&capsule)
   383	        .map_err(|e| format!("capsule json encode: {e}"))?;
   384	    std::fs::write(&json_path, &json_body).map_err(|e| format!("write json: {e}"))?;
   385	    let latest_path = args.out_dir.join("LATEST_MARKOV_CAPSULE.txt");
   386	    std::fs::write(&latest_path, capsule.capsule_id.hex())
   387	        .map_err(|e| format!("write latest pointer: {e}"))?;
   388	
   389	    eprintln!("wrote {}", json_path.display());
   390	    eprintln!("wrote {}", latest_path.display());
   391	    Ok(0)
   392	}
   393	
   394	fn hex32(b: &[u8; 32]) -> String {
   395	    let mut s = String::with_capacity(64);

exec
/bin/bash -lc "nl -ba src/bin/generate_markov_capsule.rs | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	//! TB-15 Atom 5 — `generate_markov_capsule` CLI (architect §6.2 +
     2	//! FR-15.4 + FR-15.5).
     3	//!
     4	//! Reads constitution.md → SHA-256; opens the chain runtime_repo + CAS
     5	//! to derive L4 / L4.E / CAS roots; scans `handover/alignment/OBS_*.md`
     6	//! for unresolved-OBS list; clusters CAS-resident
     7	//! AgentAutopsyCapsules into TypicalErrorSummary list (Atom 4
     8	//! `cluster_autopsies`); writes a `MarkovEvidenceCapsule` to CAS +
     9	//! emits a JSON pointer file under `--out-dir`.
    10	//!
    11	//! Default-deny: deeper-history reads (older capsules; L4 rows
    12	//! pre-dating `--prev-cid`'s implied `l4_root`) require
    13	//! `TURINGOS_MARKOV_OVERRIDE=1`. Without override, only the constitution
    14	//! + previous Markov capsule + current chain heads are read.
    15	//!
    16	//! Usage:
    17	//!   generate_markov_capsule \
    18	//!     --tb-id <N> \
    19	//!     --out-dir <path> \
    20	//!     --constitution-path <path> \
    21	//!     --runtime-repo <path> \
    22	//!     --cas-dir <path> \
    23	//!     [--prev-cid-hex <hex>] \
    24	//!     [--alignment-dir <path>] \
    25	//!     [--no-cas]
    26	//!
    27	//! `--no-cas` runs in pointer-only mode (write JSON file but skip CAS
    28	//! put — useful when no runtime CAS is available, e.g. fresh repo).
    29	//!
    30	//! Exit code:
    31	//!   0  — capsule generated + persisted.
    32	//!   1  — generation failed (write error / missing constitution.md).
    33	//!   2  — invalid args.
    34	
    35	use std::path::PathBuf;
    36	use std::sync::{Arc, RwLock};
    37	
    38	use turingosv4::bottom_white::cas::schema::Cid;
    39	use turingosv4::bottom_white::cas::store::CasStore;
    40	use turingosv4::runtime::autopsy_capsule::TypicalErrorSummary;
    41	use turingosv4::runtime::markov_capsule::{
    42	    override_set_from_env, read_flowchart_hashes_from_matrix, scan_unresolved_obs, sha256_of_file,
    43	    try_deep_history_read_with_override_check, write_markov_capsule, MarkovGenError, ObsId,
    44	};
    45	use turingosv4::state::q_state::Hash;
    46	
    47	struct Args {
    48	    tb_id: String,
    49	    out_dir: PathBuf,
    50	    constitution_path: PathBuf,
    51	    flowchart_matrix_path: PathBuf,
    52	    /// v0 placeholder — future TB will read L4 chain head from this path.
    53	    #[allow(dead_code)]
    54	    runtime_repo: Option<PathBuf>,
    55	    cas_dir: Option<PathBuf>,
    56	    prev_cid_hex: Option<String>,
    57	    alignment_dir: PathBuf,
    58	    no_cas: bool,
    59	    /// R2 closure (Codex R1 Q4): when > 0, the binary attempts to read N
    60	    /// prior Markov capsules (deeper than the previous_capsule_cid) — a
    61	    /// LIVE deep-history read path that REQUIRES `TURINGOS_MARKOV_OVERRIDE=1`
    62	    /// per FR-15.5 + halt-trigger #6. Default 0 = no deep-history read.
    63	    include_prior_capsules: u32,
    64	}
    65	
    66	fn parse_args(argv: &[String]) -> Result<Args, String> {
    67	    let mut tb_id: Option<String> = None;
    68	    let mut out_dir: Option<PathBuf> = None;
    69	    let mut constitution_path: Option<PathBuf> = None;
    70	    let mut flowchart_matrix_path: Option<PathBuf> = None;
    71	    let mut runtime_repo: Option<PathBuf> = None;
    72	    let mut cas_dir: Option<PathBuf> = None;
    73	    let mut prev_cid_hex: Option<String> = None;
    74	    let mut alignment_dir: Option<PathBuf> = None;
    75	    let mut no_cas = false;
    76	    let mut include_prior_capsules: u32 = 0;
    77	
    78	    let mut i = 0;
    79	    while i < argv.len() {
    80	        match argv[i].as_str() {
    81	            "--tb-id" => {
    82	                tb_id = argv.get(i + 1).cloned();
    83	                i += 2;
    84	            }
    85	            "--out-dir" => {
    86	                out_dir = argv.get(i + 1).map(PathBuf::from);
    87	                i += 2;
    88	            }
    89	            "--constitution-path" => {
    90	                constitution_path = argv.get(i + 1).map(PathBuf::from);
    91	                i += 2;
    92	            }
    93	            "--flowchart-matrix-path" => {
    94	                flowchart_matrix_path = argv.get(i + 1).map(PathBuf::from);
    95	                i += 2;
    96	            }
    97	            "--runtime-repo" => {
    98	                runtime_repo = argv.get(i + 1).map(PathBuf::from);
    99	                i += 2;
   100	            }
   101	            "--cas-dir" => {
   102	                cas_dir = argv.get(i + 1).map(PathBuf::from);
   103	                i += 2;
   104	            }
   105	            "--prev-cid-hex" => {
   106	                prev_cid_hex = argv.get(i + 1).cloned();
   107	                i += 2;
   108	            }
   109	            "--alignment-dir" => {
   110	                alignment_dir = argv.get(i + 1).map(PathBuf::from);
   111	                i += 2;
   112	            }
   113	            "--no-cas" => {
   114	                no_cas = true;
   115	                i += 1;
   116	            }
   117	            "--include-prior-capsules" => {
   118	                include_prior_capsules = argv
   119	                    .get(i + 1)
   120	                    .ok_or_else(|| "--include-prior-capsules <N> requires arg".to_string())?

exec
/bin/bash -lc "nl -ba src/runtime/audit_assertions.rs | sed -n '430,520p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   430	
   431	fn sha256_hash(bytes: &[u8]) -> Hash {
   432	    let mut h = Sha256::new();
   433	    h.update(bytes);
   434	    Hash(h.finalize().into())
   435	}
   436	
   437	fn hex_encode(bytes: &[u8]) -> String {
   438	    const HEX: &[u8; 16] = b"0123456789abcdef";
   439	    let mut s = String::with_capacity(bytes.len() * 2);
   440	    for b in bytes {
   441	        s.push(HEX[(b >> 4) as usize] as char);
   442	        s.push(HEX[(b & 0x0f) as usize] as char);
   443	    }
   444	    s
   445	}
   446	
   447	fn hex_decode(hex: &str) -> Result<Vec<u8>, String> {
   448	    let h = hex.trim();
   449	    if h.len() % 2 != 0 {
   450	        return Err("odd hex length".into());
   451	    }
   452	    let mut out = Vec::with_capacity(h.len() / 2);
   453	    for chunk in h.as_bytes().chunks(2) {
   454	        let hi = char_hex(chunk[0])?;
   455	        let lo = char_hex(chunk[1])?;
   456	        out.push((hi << 4) | lo);
   457	    }
   458	    Ok(out)
   459	}
   460	
   461	fn char_hex(b: u8) -> Result<u8, String> {
   462	    match b {
   463	        b'0'..=b'9' => Ok(b - b'0'),
   464	        b'a'..=b'f' => Ok(b - b'a' + 10),
   465	        b'A'..=b'F' => Ok(b - b'A' + 10),
   466	        _ => Err(format!("non-hex char: {}", b as char)),
   467	    }
   468	}
   469	
   470	fn extract_constitution_root_hex(genesis_text: &str) -> Option<String> {
   471	    // crude TOML extract: looks for `[constitution_root]` header then a
   472	    // hash-bearing line. Genesis schema is project-specific; accept either
   473	    // sha256 = "..." or hash = "...".
   474	    let mut in_section = false;
   475	    for line in genesis_text.lines() {
   476	        let trimmed = line.trim();
   477	        if trimmed.starts_with('[') {
   478	            in_section = trimmed == "[constitution_root]";
   479	            continue;
   480	        }
   481	        if in_section {
   482	            for key in ["sha256", "hash", "constitution_hash"] {
   483	                if let Some(rest) = trimmed.strip_prefix(key) {
   484	                    let rest = rest.trim_start();
   485	                    if let Some(rest) = rest.strip_prefix('=') {
   486	                        let rest = rest.trim();
   487	                        let value = rest.trim_matches('"').trim_matches('\'').trim();
   488	                        return Some(value.to_lowercase());
   489	                    }
   490	                }
   491	            }
   492	        }
   493	    }
   494	    None
   495	}
   496	
   497	fn read_markov_capsule(
   498	    pointer_path: &Path,
   499	    cas: &CasStore,
   500	) -> Result<MarkovEvidenceCapsule, AuditError> {
   501	    if !pointer_path.exists() {
   502	        return Err(AuditError::MarkovRead(format!(
   503	            "pointer file not present: {pointer_path:?}"
   504	        )));
   505	    }
   506	    let cid_hex = std::fs::read_to_string(pointer_path)?;
   507	    let cid_hex = cid_hex.trim();
   508	    let bytes =
   509	        hex_decode(cid_hex).map_err(|e| AuditError::MarkovRead(format!("hex decode: {e}")))?;
   510	    let arr: [u8; 32] = bytes
   511	        .as_slice()
   512	        .try_into()
   513	        .map_err(|_| AuditError::MarkovRead("expected 32-byte cid".into()))?;
   514	    let cid = Cid(arr);
   515	    let caps_bytes = cas
   516	        .get(&cid)
   517	        .map_err(|e| AuditError::MarkovRead(format!("cas get: {e}")))?;
   518	    let capsule: MarkovEvidenceCapsule = canonical_decode(&caps_bytes)
   519	        .map_err(|e| AuditError::MarkovRead(format!("decode: {e}")))?;
   520	    Ok(capsule)

exec
/bin/bash -lc "sed -n '1,260p' handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/dashboard.txt" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
=================================================================
 TB-8 Audit Dashboard — run_id=tb13-smoke epoch=1
=================================================================

§1 Run metadata
---------------
  head_commit_oid: 38f1b3957834052aac42169598f92016d756c331
  final_state_root: 1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9
  final_ledger_root: 93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb
  initial_q_state_loaded_from_disk: true

§2 Chain stats + 7 indicators
------------------------------
  L4 entries:  3
  L4.E entries: 2
  ledger_root_verified              : ✓
  system_signatures_verified        : ✓
  state_reconstructed               : ✓
  economic_state_reconstructed      : ✓
  cas_payloads_retrievable          : ✓
  agent_signatures_verified [Gate 4]: ✓
  proposal_telemetry_cas_retrievable [Gate 5]: ✓
  ALL 7 PASS                        : GREEN

§3 ChainDerivedRunFacts (§4.4 bit-exact set)
---------------------------------------------
  solved                  : false
  verified                : false
  tx_count                : 5
  proposal_count          : 1
  golden_path_token_count : 0
  gp_payload (CID hex)    : -
  gp_path                 : -
  tactic_diversity        : 0
  failed_branch_count     : 2
  chain_oracle_verified   : false (no oracle-verified WorkTx)
  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
  tool_dist:
    (empty)

§4 Per-agent activity
---------------------
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  ------------------+--------+-------+-------+---------+--------
  tb6-smoke-agent   | ✗      | 0     | 1     | 0       | 0
  tb6-smoke-sponsor | ✗      | 0     | 0     | 0       | 0

§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |   3 | TerminalSummary | -          | -          | -          | -      | -

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)

§8 Cross-checks
---------------
  audit_trail_rows         : 2
  chain_proposal_count     : 1
  audit_rows == proposal_count: ✗ (gap)
  audit_trail_chain_valid     : ✓
  (Note: pre-TB-7.6 the agent_audit_trail.jsonl is populated only
   by the synthetic-seed hook; full per-LLM-proposal audit-trail
   wiring is part of TB-7.6 carry-forward action #4 / #5.)

§9 TB-8 Claims (claim_status + payout_amount)
----------------------------------------------
  (no Confirm-VerifyTx observed; n/a — claim_status / payout: n/a)

§10 TB-9 Durable identity (agent keystore registry)
---------------------------------------------------
  durable_keystore_path: /home/zephryj/.turingos/keystore/agent_keystore.enc
  durable_keystore_present: ✓ (cross-run identity available)
  agents_in_manifest: 0
  agent_id          | pubkey_in_manifest | tape_activity
  ------------------+--------------------+---------------
  (no agents with manifest pubkey on this run)

  Note: cross-run identity is empirically observable by
  comparing this run's `agent_pubkeys.json` to a sibling run
  that loaded the same TURINGOS_AGENT_KEYSTORE_PATH — equal
  pubkey rows ⇒ TB-9 mandate "agent identity survives run
  restart" satisfied.

§11 TB-10 User Tasks (sponsored by Agent_user_*; lean_market product surface)
------------------------------------------------------------------------------
  (no Agent_user_*-sponsored TaskOpen on chain; lean_market run-task
   not invoked, or evaluator ran in self-funded preseed mode
   [TURINGOS_USER_TASK_MODE unset]; n/a)

§12 TB-11 Epistemic Exhaust + Capital Liberation (architect §6.2; 2026-05-02)
------------------------------------------------------------------------------
  Exhausted runs (RunExhaustedTx ≡ TerminalSummaryTx):
    run_id         | task_id            | outcome         | attempts | evidence_capsule_cid (hex)
    ---------------+--------------------+-----------------+----------+--------------------------------
    n1_mathd_alge… | task-n1_mathd_alg… | MaxTxExhausted  |       10 | d2b329ee554da3e2dea1d46ecca1bf1…

  Architect mandate (§6.2 ruling 2026-05-02) ✓:
    O(1) chain cost / O(N) auditability — failure evidence anchored on L4
    via system-emitted system_signature; raw log requires audit-role access
    (CapsulePrivacyPolicy::AuditOnly default; only public_summary surfaces here).

§13 TB-12 Node exposure records (architect 2026-05-03 §3 + §10)
------------------------------------------------------------------------------
  (no NodePosition records — no accepted WorkTx/ChallengeTx with stake>0 on this chaintape)

  Architect mandate (§3 + §10 ruling 2026-05-03) ✓:
    NodePosition is an IMMUTABLE EXPOSURE RECORD, NOT active position balance.
    NodePosition.amount is NOT a Coin holding (CR-12.1) and is NOT counted in
    total_supply_micro (CR-12.2). NO trading. NO price. NO settlement in TB-12.
    NodeMarketEntry is TB-14 derived view; flat NodePositionsIndex is canonical.

§14 TB-14 PriceIndex (architect 2026-05-03 §5.1 + §5.5 SG-14.6)
---------------------------------------------------------------
  PRICE IS SIGNAL, NOT TRUTH.
    Architect §5.1 ruling 2026-05-03: the price index is a
    derived statistical broadcast over canonical NodePositionsIndex
    long/short interest. It MUST NOT influence predicate gates
    (CR-14.1 / halt-trigger #1) or L4/L4.E classification
    (CR-14.2 / halt-trigger #2). Boolean predicates establish
    absolute bounds; the price view is for relative-effectiveness
    measurement only.

  (no node positions recorded — price index is empty)
  Acceptable signal-state: a run with zero accepted WorkTx +
  ChallengeTx yields an empty PriceIndex by FR-14.3 / halt-
  trigger #5 (zero-liquidity → price=None) extended to the
  zero-position case.

§15 TB-15 Autopsy + Markov (architect 2026-05-02 §6.5 SG-15.6)
--------------------------------------------------------------
  AUTOPSY IS PRIVATE — public summary shown only when typical
  (≥3 cluster). Raw private details require audit-role access.
    Architect §6.4 ruling 2026-05-02: capsule audit detail is
    AuditOnly; NEVER enters AgentVisibleProjection (CR-15.1 +
    halt-trigger #1 + #4).
    Typical-error broadcast surface uses public_summary text
    only (CR-15.2 + halt-trigger #5).

  (no agent_autopsies_t entries in this snapshot — no
  TaskBankruptcyTx has fired during the chain window)
  Acceptable signal-state: a run with zero accepted
  TaskBankruptcyTx yields an empty AutopsyIndex by
  TB-15 Atom 3 charter scope (single trigger site).

  Markov default (FR-15.4): next-session boot reads
  constitution.md + latest Markov capsule. deeper history
  requires TURINGOS_MARKOV_OVERRIDE=1 (CR-15.6 +
  halt-trigger #6 — default-deny gate).

  Latest Markov capsule pointer (handover/markov_capsules/
  LATEST_MARKOV_CAPSULE.txt):
    f9e701b4a9c2e1d9b4d1222c06a6c4e4f6516aa1af1c3ed29af457d15532d312

  Architect mandate (§6.5 SG-15.6 + §6.4 ruling 2026-05-02) ✓:
    Dashboard regenerates capsule summary from ChainTape + CAS;
    NO raw private detail in dashboard output. Markov default
    prevents context poisoning — full failure history not auto-
    replayed; only constitution + latest capsule by default.

§16 TB-16 SANDBOX BANNER (architect 2026-05-03 §7.4 CR-16.7 + §7.5 SG-16.8)
==========================================================================
  ⚠ SANDBOX-RUN — NOT PRODUCTION — NO REAL FUNDS
    Agent IDs are sandbox-prefixed (Agent_solver_/Agent_verifier_/
    Agent_user_/tb7-7-sponsor/tb16-). Total Coin sourced from
    runtime::bootstrap::default_pput_preseed_pairs() (30_000_000 μC
    on_init mint; assert_no_post_init_mint enforced).

    Architect §7.6 forbidden:
      - No public chain.
      - No real-money market.
      - No external domain (Lean only; no medical/legal/financial).
      - No production user funds.

    Prices, positions, masks, autopsies surfaced above are SIGNAL
    only — never to be interpreted as real-money valuations.

exec
/bin/bash -lc "ls -ld handover/audits && ls -la handover/audits | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
drwxr-xr-x 2 zephryj zephryj 28672 May  4 04:58 handover/audits
total 43936
drwxr-xr-x  2 zephryj zephryj   28672 May  4 04:58 .
drwxr-xr-x 16 zephryj zephryj    4096 May  4 01:11 ..
-rw-r--r--  1 zephryj zephryj   38316 Apr 29 16:42 2026-04-29_external_audit.md
-rw-r--r--  1 zephryj zephryj   38205 Apr 26 09:07 A8_AUDIT_HISTORY_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   15789 Apr 26 09:07 A8_EXIT_PACKET_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   12657 Apr 26 15:12 AUDITOR_TFR_PLAN_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   32350 Apr 28 07:58 AUDIT_LEDGER.md
-rw-r--r--  1 zephryj zephryj    7634 Apr 25 14:27 B5_DEFERRED_FROM_MIDTERM_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   14721 May  2 15:10 CHARTER_RATIFICATION_TB_10_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   14125 May  2 11:28 CHARTER_RATIFICATION_TB_8_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   13372 May  2 13:19 CHARTER_RATIFICATION_TB_9_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    5620 Apr 27 11:40 CLAUDE_AUDITOR_CO1_7_0AF_KEYPAIR_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    7143 Apr 28 12:16 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   11255 Apr 28 11:29 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    6677 Apr 28 11:59 CO1_1_4_PRE1_DUAL_AUDIT_VERDICT_R2_2026-04-28.md
-rw-r--r--  1 zephryj zephryj   11950 Apr 29 02:41 CO1_7_5_DUAL_AUDIT_VERDICT_R1_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    9287 Apr 28 07:26 CO1_7_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    5615 Apr 28 07:56 CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    9706 Apr 29 03:17 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R2_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    6546 Apr 29 03:34 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R3_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    5159 Apr 29 03:43 CO1_7_EXTRA_DUAL_AUDIT_VERDICT_R4_2026-04-29.md
-rw-r--r--  1 zephryj zephryj    6297 Apr 28 14:14 CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_FINAL_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    7741 Apr 28 14:04 CO1_7_IMPL_BUNDLE_DUAL_AUDIT_VERDICT_R1_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  496648 Apr 25 17:07 CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  236036 Apr 25 17:59 CODEX_B7_EXTRA_REAUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  341088 Apr 25 18:10 CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  705580 Apr 25 18:21 CODEX_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  150010 Apr 29 05:22 CODEX_CO1_13_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj 1340857 Apr 29 05:35 CODEX_CO1_13_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj 1152985 Apr 28 11:26 CODEX_CO1_1_4_PRE1_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  523686 Apr 28 11:58 CODEX_CO1_1_4_PRE1_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  264778 Apr 28 12:08 CODEX_CO1_1_4_PRE1_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  413082 Apr 28 12:11 CODEX_CO1_1_4_PRE1_ROUND4_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  568226 Apr 28 12:15 CODEX_CO1_1_4_PRE1_ROUND5_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj    4180 Apr 27 11:41 CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj 2744270 Apr 29 01:03 CODEX_CO1_7_5_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  483375 Apr 29 03:13 CODEX_CO1_7_EXTRA_ROUND2_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  431835 Apr 29 03:33 CODEX_CO1_7_EXTRA_ROUND3_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  177390 Apr 29 03:42 CODEX_CO1_7_EXTRA_ROUND4_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  647022 Apr 28 13:59 CODEX_CO1_7_IMPL_BUNDLE_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  187480 Apr 28 14:07 CODEX_CO1_7_IMPL_BUNDLE_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  439017 Apr 28 14:12 CODEX_CO1_7_IMPL_BUNDLE_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  392767 Apr 28 07:24 CODEX_CO1_7_ROUND1_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  721388 Apr 28 07:45 CODEX_CO1_7_ROUND2_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  947675 Apr 28 07:55 CODEX_CO1_7_ROUND3_AUDIT_2026-04-28.md
-rw-r--r--  1 zephryj zephryj  362040 Apr 29 11:47 CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   38906 Apr 26 16:45 CODEX_CO_P0_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    3465 Apr 27 12:00 CODEX_INV8_DAG_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   15120 Apr 24 06:34 CODEX_PAPER1_AUDIT_2026-04-23.md
-rw-r--r--  1 zephryj zephryj   17473 Apr 25 03:26 CODEX_PAPER1_V2_1_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   11419 Apr 24 17:57 CODEX_PAPER1_V2_AUDIT_2026-04-24.md
-rw-r--r--  1 zephryj zephryj 1112560 Apr 26 03:05 CODEX_PHASE_A0_EXIT_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj  660680 Apr 26 05:14 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj 1764610 Apr 26 07:20 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R10.md
-rw-r--r--  1 zephryj zephryj  651156 Apr 26 08:01 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R11.md
-rw-r--r--  1 zephryj zephryj  514594 Apr 26 08:51 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R12.md
-rw-r--r--  1 zephryj zephryj 2401751 Apr 26 09:06 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R13.md
-rw-r--r--  1 zephryj zephryj  910608 Apr 26 05:30 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R2.md
-rw-r--r--  1 zephryj zephryj  455324 Apr 26 05:39 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R3.md
-rw-r--r--  1 zephryj zephryj  482745 Apr 26 05:49 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R4.md
-rw-r--r--  1 zephryj zephryj  524409 Apr 26 05:58 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R5.md
-rw-r--r--  1 zephryj zephryj 1574378 Apr 26 06:07 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R6.md
-rw-r--r--  1 zephryj zephryj  639750 Apr 26 06:51 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R7.md
-rw-r--r--  1 zephryj zephryj  595925 Apr 26 07:02 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R8.md
-rw-r--r--  1 zephryj zephryj 1603789 Apr 26 07:11 CODEX_PHASE_A8_EXIT_AUDIT_2026-04-26_R9.md
-rw-r--r--  1 zephryj zephryj  170881 Apr 25 10:59 CODEX_PPUT_CCL_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  286708 Apr 25 11:27 CODEX_PPUT_CCL_AUDIT_ROUND2_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  556403 Apr 25 11:38 CODEX_PPUT_CCL_AUDIT_ROUND3_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  351502 Apr 25 11:51 CODEX_PPUT_CCL_AUDIT_ROUND4_2026-04-26.md
-rw-r--r--  1 zephryj zephryj  674813 Apr 25 14:18 CODEX_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj   27129 Apr 27 08:06 CODEX_SPEC_FREEZE_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   12423 Apr 27 08:20 CODEX_SPEC_V12_REAUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   10348 Apr 27 09:48 CODEX_SPEC_V13_REAUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj    2163 Apr 27 11:07 CODEX_SPEC_V14_ROUND4_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   18146 May  1 10:31 CODEX_TB6_FULLDIFF_AUDIT_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   15264 May  2 03:57 CODEX_TB7R_MICRO_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   19450 May  1 13:16 CODEX_TB7_FULLDIFF_AUDIT_2026-05-01.md
-rw-r--r--  1 zephryj zephryj   11709 May  3 03:09 CODEX_TB_12_SHIP_AUDIT_2026-05-03.md
-rw-r--r--  1 zephryj zephryj  562967 May  3 07:08 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R1.md
-rw-r--r--  1 zephryj zephryj  441401 May  3 07:14 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R2.md
-rw-r--r--  1 zephryj zephryj 2853081 May  3 07:43 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R3.md
-rw-r--r--  1 zephryj zephryj 2543592 May  3 09:17 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R4.md
-rw-r--r--  1 zephryj zephryj 1466195 May  3 09:46 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R5.md
-rw-r--r--  1 zephryj zephryj  116326 May  3 10:06 CODEX_TB_13_SHIP_AUDIT_2026-05-03_R6.md
-rw-r--r--  1 zephryj zephryj  494358 May  3 20:55 CODEX_TB_14_SHIP_AUDIT_2026-05-03_R1.md
-rw-r--r--  1 zephryj zephryj  438849 May  3 22:03 CODEX_TB_14_SHIP_AUDIT_2026-05-03_R2.md
-rw-r--r--  1 zephryj zephryj  657352 May  4 01:59 CODEX_TB_15_SHIP_AUDIT_2026-05-04_R1.md
-rw-r--r--  1 zephryj zephryj  333868 May  4 02:27 CODEX_TB_15_SHIP_AUDIT_2026-05-04_R2.md
-rw-r--r--  1 zephryj zephryj  416960 May  4 02:49 CODEX_TB_15_SHIP_AUDIT_2026-05-04_R3.md
-rw-r--r--  1 zephryj zephryj  613729 May  4 05:03 CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md.raw
-rw-r--r--  1 zephryj zephryj  440063 Apr 29 18:28 CODEX_TB_1_AUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj  138233 Apr 29 19:09 CODEX_TB_1_PATH_A_PP_MICROAUDIT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   19808 Apr 30 04:06 CODEX_TB_2_PHASE0_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   17319 Apr 30 04:36 CODEX_TB_2_PHASE0_R2_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   12327 Apr 30 06:43 CODEX_TB_2_PHASE1C_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    2505 Apr 30 06:43 CODEX_TB_2_PHASE1C_R2_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   21484 Apr 30 15:08 CODEX_TB_4_SHIP_TB_5_CHARTER_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   26538 Apr 30 17:15 CODEX_TB_5_PHASE0_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   11819 Apr 30 17:33 CODEX_TB_5_PHASE0_R3_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    5424 Apr 30 19:33 CODEX_TB_5_PHASE0_R4_AUDIT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    9754 May  2 07:00 CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj  221263 May  2 07:23 CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   12143 May  2 12:26 CODEX_TB_8_SHIP_AUDIT_2026-05-02.md
-rw-r--r--  1 zephryj zephryj    6579 May  2 12:49 CODEX_TB_8_SHIP_AUDIT_R2_2026-05-02.md
-rw-r--r--  1 zephryj zephryj   33447 Apr 26 15:14 CODEX_TFR_PLAN_AUDIT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   24673 Apr 27 00:36 CODEX_T_S_REVIEW_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   13619 Apr 27 15:09 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27.md
-rw-r--r--  1 zephryj zephryj   10094 Apr 27 15:53 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R2.md
-rw-r--r--  1 zephryj zephryj    1365 Apr 27 17:20 CODEX_WHITEPAPER_V2_AUDIT_2026-04-27_R3.md
-rw-r--r--  1 zephryj zephryj   11204 Apr 22 08:25 CONSTITUTIONAL_BLINDSPOT_AUDIT_2026-04-22.md
-rw-r--r--  1 zephryj zephryj    8506 Apr 24 06:38 DUAL_AUDIT_PAPER1_VERDICT_2026-04-23.md
-rw-r--r--  1 zephryj zephryj   11048 Apr 25 11:02 DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md
-rw-r--r--  1 zephryj zephryj    5620 Apr 25 11:52 DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md
-rw-r--r--  1 zephryj zephryj   10063 Apr 29 18:30 DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md
-rw-r--r--  1 zephryj zephryj   12368 Apr 30 04:08 DUAL_AUDIT_TB_2_PHASE0_VERDICT_R1_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    7565 Apr 30 06:43 DUAL_AUDIT_TB_2_PHASE1C_VERDICT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj   11925 Apr 30 16:06 DUAL_AUDIT_TB_4_SHIP_TB_5_CHARTER_VERDICT_2026-04-30.md
-rw-r--r--  1 zephryj zephryj    5206 Apr 25 03:28 DUAL_AUDIT_V2_1_VERDICT_2026-04-25.md
-rw-r--r--  1 zephryj zephryj    5074 Apr 24 18:03 DUAL_AUDIT_V2_VERDICT_2026-04-24.md

codex
I have enough for a VETO: the shipped evidence is an admitted `BLOCK` verdict over a TB-13 fixture, not a TB-16 controlled arena; the runner and smoke gate allow that state to pass as infrastructure-only. I’m collecting exact JSON line refs now and then I’ll attempt the requested audit file write if the sandbox permits it.
exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "schema_version": "v1/audit_tape_verdict",
     3	  "tape_root": {
     4	    "l4_count": 3,
     5	    "l4e_count": 2,
     6	    "head_state_root_hex": "1a4e9793b1dedf7d83808b85f875e4cb3e3c900dd03e1d6000f1f51a6bbde2b9",
     7	    "head_ledger_root_hex": "93b4432adc5e49cc6b976e4eb182c4d9da9bb5050e8122b5697eb3d9d1fe28fb",
     8	    "cas_object_count": 11,
     9	    "constitution_hash_hex": "eec695459c71fbef3685583485deb431fe3b561657b2f285b7c5e7e220e59e03"
    10	  },
    11	  "tx_kind_counts": {
    12	    "work": 0,
    13	    "verify": 0,
    14	    "challenge": 0,
    15	    "reuse": 0,
    16	    "task_open": 1,
    17	    "escrow_lock": 1,
    18	    "complete_set_mint": 0,
    19	    "complete_set_redeem": 0,
    20	    "market_seed": 0,
    21	    "finalize_reward": 0,
    22	    "challenge_resolve": 0,
    23	    "terminal_summary": 1,
    24	    "task_expire": 0,
    25	    "task_bankruptcy": 0
    26	  },
    27	  "assertions": [
    28	    {
    29	      "id": 1,
    30	      "name": "constitution_hash_matches_genesis",
    31	      "layer": "A",
    32	      "result": "Pass",
    33	      "detail": null
    34	    },
    35	    {
    36	      "id": 2,
    37	      "name": "pinned_pubkey_loaded",
    38	      "layer": "A",
    39	      "result": "Pass",
    40	      "detail": null
    41	    },
    42	    {
    43	      "id": 3,
    44	      "name": "sandbox_agent_prefix",
    45	      "layer": "A",
    46	      "result": "Pass",
    47	      "detail": null
    48	    },
    49	    {
    50	      "id": 4,
    51	      "name": "l4_hash_chain_valid",
    52	      "layer": "B",
    53	      "result": "Pass",
    54	      "detail": null
    55	    },
    56	    {
    57	      "id": 5,
    58	      "name": "l4_parent_state_continuity",
    59	      "layer": "B",
    60	      "result": "Pass",
    61	      "detail": null
    62	    },
    63	    {
    64	      "id": 6,
    65	      "name": "l4e_chain_integrity",
    66	      "layer": "B",
    67	      "result": "Pass",
    68	      "detail": null
    69	    },
    70	    {
    71	      "id": 7,
    72	      "name": "genesis_row_zero_parents",
    73	      "layer": "B",
    74	      "result": "Pass",
    75	      "detail": null
    76	    },
    77	    {
    78	      "id": 8,
    79	      "name": "system_tx_signatures_verify",
    80	      "layer": "B",
    81	      "result": "Pass",
    82	      "detail": null
    83	    },
    84	    {
    85	      "id": 9,
    86	      "name": "agent_tx_signatures_verify",
    87	      "layer": "B",
    88	      "result": "Pass",
    89	      "detail": null
    90	    },
    91	    {
    92	      "id": 10,
    93	      "name": "payload_cid_resolves",
    94	      "layer": "B",
    95	      "result": "Pass",
    96	      "detail": null
    97	    },
    98	    {
    99	      "id": 11,
   100	      "name": "tx_kind_envelope_matches_payload",
   101	      "layer": "B",
   102	      "result": "Pass",
   103	      "detail": null
   104	    },
   105	    {
   106	      "id": 12,
   107	      "name": "replay_state_root_matches_head",
   108	      "layer": "C",
   109	      "result": "Pass",
   110	      "detail": null
   111	    },
   112	    {
   113	      "id": 13,
   114	      "name": "replay_economic_state_canonical",
   115	      "layer": "C",
   116	      "result": "Pass",
   117	      "detail": null
   118	    },
   119	    {
   120	      "id": 14,

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json | sed -n '300,430p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	    }
   301	  ],
   302	  "passed": 31,
   303	  "failed": 0,
   304	  "halted": 1,
   305	  "skipped": 7,
   306	  "feature_coverage": {
   307	    "TB-11_terminal_bankruptcy_expire": "GREEN",
   308	    "TB-13_complete_set": "RED",
   309	    "TB-14_price_mask": "GREEN",
   310	    "TB-15_autopsy_markov": "RED",
   311	    "TB-1_monetary": "GREEN",
   312	    "TB-2_work": "RED",
   313	    "TB-3_task_open_escrow": "GREEN",
   314	    "TB-4_verify_challenge": "RED",
   315	    "TB-5_challenge_resolve": "RED",
   316	    "TB-6_chain": "GREEN",
   317	    "TB-7_agent_pubkeys": "GREEN",
   318	    "TB-8_finalize_reward": "RED"
   319	  },
   320	  "verdict": "BLOCK"
   321	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "all_detected": true,
     3	  "detected_count": 3,
     4	  "expected": 3,
     5	  "schema_version": "v1/audit_tape_tamper",
     6	  "tamper_results": [
     7	    {
     8	      "detail": "flipped byte 41 in \"handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper/flip_l4_byte/runtime_repo/.git/objects/68/ba8a6d2eac44efabe41c7bcddc1b9fe778f7c2\"",
     9	      "detected": true,
    10	      "label": "flip_l4_byte",
    11	      "schema_version": "v1/audit_tape_tamper",
    12	      "verdict": {
    13	        "assertions": [
    14	          {
    15	            "detail": null,
    16	            "id": 1,
    17	            "layer": "A",
    18	            "name": "constitution_hash_matches_genesis",
    19	            "result": "Pass"
    20	          },
    21	          {
    22	            "detail": null,
    23	            "id": 2,
    24	            "layer": "A",
    25	            "name": "pinned_pubkey_loaded",
    26	            "result": "Pass"
    27	          },
    28	          {
    29	            "detail": null,
    30	            "id": 3,
    31	            "layer": "A",
    32	            "name": "sandbox_agent_prefix",
    33	            "result": "Pass"
    34	          },
    35	          {
    36	            "detail": null,
    37	            "id": 4,
    38	            "layer": "B",
    39	            "name": "l4_hash_chain_valid",
    40	            "result": "Pass"
    41	          },
    42	          {
    43	            "detail": null,
    44	            "id": 5,
    45	            "layer": "B",
    46	            "name": "l4_parent_state_continuity",
    47	            "result": "Pass"
    48	          },
    49	          {
    50	            "detail": null,
    51	            "id": 6,
    52	            "layer": "B",
    53	            "name": "l4e_chain_integrity",
    54	            "result": "Pass"
    55	          },
    56	          {
    57	            "detail": null,
    58	            "id": 7,
    59	            "layer": "B",
    60	            "name": "genesis_row_zero_parents",
    61	            "result": "Pass"
    62	          },
    63	          {
    64	            "detail": null,
    65	            "id": 8,
    66	            "layer": "B",
    67	            "name": "system_tx_signatures_verify",
    68	            "result": "Pass"
    69	          },
    70	          {
    71	            "detail": null,
    72	            "id": 9,
    73	            "layer": "B",
    74	            "name": "agent_tx_signatures_verify",
    75	            "result": "Pass"
    76	          },
    77	          {
    78	            "detail": null,
    79	            "id": 10,
    80	            "layer": "B",

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "capsule_id": [
     3	    93,
     4	    165,
     5	    54,
     6	    2,
     7	    43,
     8	    76,
     9	    86,
    10	    208,
    11	    237,
    12	    151,
    13	    160,
    14	    31,
    15	    107,
    16	    122,
    17	    175,
    18	    10,
    19	    141,
    20	    108,
    21	    69,
    22	    26,
    23	    51,
    24	    249,
    25	    202,
    26	    3,
    27	    48,
    28	    183,
    29	    154,
    30	    203,
    31	    163,
    32	    197,
    33	    246,
    34	    216
    35	  ],
    36	  "previous_capsule_cid": null,
    37	  "constitution_hash": [
    38	    238,
    39	    198,
    40	    149,
    41	    69,
    42	    156,
    43	    113,
    44	    251,
    45	    239,
    46	    54,
    47	    133,
    48	    88,
    49	    52,
    50	    133,
    51	    222,
    52	    180,
    53	    49,
    54	    254,
    55	    59,
    56	    86,
    57	    22,
    58	    87,
    59	    178,
    60	    242,
    61	    133,
    62	    183,
    63	    197,
    64	    231,
    65	    226,
    66	    32,
    67	    229,
    68	    158,
    69	    3
    70	  ],
    71	  "flowchart_hashes": [
    72	    [
    73	      164,
    74	      116,
    75	      198,
    76	      185,
    77	      222,
    78	      215,
    79	      102,
    80	      80,

exec
/bin/bash -lc "rg -n '\"id\": 27|terminal_summary_evidence_capsule|\"result\": \"Halt\"|\"id\": 32|\"id\": 39|\"result\": \"Skipped\"|\"typical_errors\"|\"tb_tag\"' handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json:311:  "typical_errors": [],
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json:406:  "tb_tag": "TB-16"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:198:            "id": 27,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:200:            "name": "terminal_summary_evidence_capsule",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:201:            "result": "Halt"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:233:            "id": 39,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:240:            "id": 32,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:243:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:250:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:257:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:264:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:271:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:278:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:285:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:525:            "id": 27,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:527:            "name": "terminal_summary_evidence_capsule",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:528:            "result": "Halt"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:560:            "id": 39,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:567:            "id": 32,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:570:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:577:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:584:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:591:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:598:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:605:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/tamper_report.json:612:            "result": "Skipped"
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:211:      "id": 27,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:212:      "name": "terminal_summary_evidence_capsule",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:214:      "result": "Halt",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:246:      "id": 39,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:253:      "id": 32,
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:256:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:263:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:270:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:277:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:284:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:291:      "result": "Skipped",
handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:298:      "result": "Skipped",

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json | sed -n '210,290p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   210	    {
   211	      "id": 27,
   212	      "name": "terminal_summary_evidence_capsule",
   213	      "layer": "E",
   214	      "result": "Halt",
   215	      "detail": "evidence_capsule_cid not in CAS at L4 index 2"
   216	    },
   217	    {
   218	      "id": 28,
   219	      "name": "projection_no_autopsy_bytes",
   220	      "layer": "F",
   221	      "result": "Pass",
   222	      "detail": null
   223	    },
   224	    {
   225	      "id": 29,
   226	      "name": "autopsy_private_detail_creator_is_system",
   227	      "layer": "F",
   228	      "result": "Pass",
   229	      "detail": null
   230	    },
   231	    {
   232	      "id": 30,
   233	      "name": "typical_error_summary_no_private_detail",
   234	      "layer": "F",
   235	      "result": "Pass",
   236	      "detail": null
   237	    },
   238	    {
   239	      "id": 31,
   240	      "name": "autopsy_index_value_type_is_vec_cid",
   241	      "layer": "F",
   242	      "result": "Pass",
   243	      "detail": null
   244	    },
   245	    {
   246	      "id": 39,
   247	      "name": "no_llm_self_narrative_in_autopsy",
   248	      "layer": "F",
   249	      "result": "Pass",
   250	      "detail": null
   251	    },
   252	    {
   253	      "id": 32,
   254	      "name": "markov_constitution_hash_matches",
   255	      "layer": "G",
   256	      "result": "Skipped",
   257	      "detail": "no Markov capsule"
   258	    },
   259	    {
   260	      "id": 33,
   261	      "name": "markov_typical_errors_recompute",
   262	      "layer": "G",
   263	      "result": "Skipped",
   264	      "detail": "no Markov capsule"
   265	    },
   266	    {
   267	      "id": 34,
   268	      "name": "markov_unresolved_obs_recompute",
   269	      "layer": "G",
   270	      "result": "Skipped",
   271	      "detail": "no Markov capsule"
   272	    },
   273	    {
   274	      "id": 35,
   275	      "name": "markov_next_session_context_resolves",
   276	      "layer": "G",
   277	      "result": "Skipped",
   278	      "detail": "no Markov capsule"
   279	    },
   280	    {
   281	      "id": 36,
   282	      "name": "tamper_l4_flip_detected",
   283	      "layer": "H",
   284	      "result": "Skipped",
   285	      "detail": "exercised by audit_tape_tamper binary (Atom 3)"
   286	    },
   287	    {
   288	      "id": 37,
   289	      "name": "tamper_cas_flip_detected",
   290	      "layer": "H",

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json | sed -n '220,260p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   220	    0,
   221	    0,
   222	    0,
   223	    0,
   224	    0,
   225	    0,
   226	    0,
   227	    0,
   228	    0,
   229	    0,
   230	    0,
   231	    0,
   232	    0,
   233	    0,
   234	    0,
   235	    0,
   236	    0,
   237	    0,
   238	    0,
   239	    0,
   240	    0,
   241	    0
   242	  ],
   243	  "l4e_root": [
   244	    0,
   245	    0,
   246	    0,
   247	    0,
   248	    0,
   249	    0,
   250	    0,
   251	    0,
   252	    0,
   253	    0,
   254	    0,
   255	    0,
   256	    0,
   257	    0,
   258	    0,
   259	    0,
   260	    0,

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/MARKOV_TB-16_2026-05-03.json | sed -n '300,320p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   300	    0,
   301	    0,
   302	    0,
   303	    0,
   304	    0,
   305	    0,
   306	    0,
   307	    0,
   308	    0,
   309	    0
   310	  ],
   311	  "typical_errors": [],
   312	  "unresolved_obs": [
   313	    "handover/alignment/OBS_AGENT_SIG_REPLAY_GAP_2026-05-03.md",
   314	    "handover/alignment/OBS_BOOT_FAIL_NOT_HALT_2026-04-25.md",
   315	    "handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md",
   316	    "handover/alignment/OBS_CONSTITUTION_MERMAID_FENCE_2026-04-22.md",
   317	    "handover/alignment/OBS_R022_TB-6_ATOM_1_2_TRACE_MATRIX_TEXT_EXTENSION_2026-05-01.md",
   318	    "handover/alignment/OBS_R022_TB-8_ATOM_1_CLAIMENTRY_TRACE_MATRIX_TEXT_EXTENSION_2026-05-02.md",
   319	    "handover/alignment/OBS_R022_TB13_RESOLUTIONREF_REMOVED_2026-05-03.md",
   320	    "handover/alignment/OBS_R022_TB14_PRICEINDEX_REMOVED_2026-05-03.md",

exec
/bin/bash -lc 'nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/agent_pubkeys.json' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "agents": {}
     3	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/runtime_repo/genesis_report.json | sed -n '1,90p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "constitution_hash": null,
     3	  "runtime_repo": "/tmp/tb13_smoke_1777790073/runtime_repo",
     4	  "cas_path": "/tmp/tb13_smoke_1777790073/cas",
     5	  "system_pubkey_hash": "0060098efa84b5f113d15cdc978865fed5149ad1ef9c91cefd6aeb5b79a4a59f",
     6	  "agent_pubkeys_path": "agent_pubkeys.json",
     7	  "initial_balances": [
     8	    [
     9	      "tb7-7-sponsor",
    10	      10000000
    11	    ],
    12	    [
    13	      "Agent_user_0",
    14	      10000000
    15	    ],
    16	    [
    17	      "Agent_0",
    18	      1000000
    19	    ],
    20	    [
    21	      "Agent_1",
    22	      1000000
    23	    ],
    24	    [
    25	      "Agent_2",
    26	      1000000
    27	    ],
    28	    [
    29	      "Agent_3",
    30	      1000000
    31	    ],
    32	    [
    33	      "Agent_4",
    34	      1000000
    35	    ],
    36	    [
    37	      "Agent_5",
    38	      1000000
    39	    ],
    40	    [
    41	      "Agent_6",
    42	      1000000
    43	    ],
    44	    [
    45	      "Agent_7",
    46	      1000000
    47	    ],
    48	    [
    49	      "Agent_8",
    50	      1000000
    51	    ],
    52	    [
    53	      "Agent_9",
    54	      1000000
    55	    ]
    56	  ],
    57	  "task_id": "task-n1_mathd_algebra_171_1777790080974",
    58	  "task_open_tx": "taskopen-task-n1_mathd_algebra_171_1777790080974-tb7-7-d3-seed",
    59	  "escrow_lock_tx": "escrowlock-task-n1_mathd_algebra_171_1777790080974-tb7-7-d3-escrow"
    60	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/dashboard.txt | sed -n '35,65p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    35	  failed_branch_count     : 2
    36	  chain_oracle_verified   : false (no oracle-verified WorkTx)
    37	  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
    38	  tool_dist:
    39	    (empty)
    40	
    41	§4 Per-agent activity
    42	---------------------
    43	  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
    44	  ------------------+--------+-------+-------+---------+--------
    45	  tb6-smoke-agent   | ✗      | 0     | 1     | 0       | 0
    46	  tb6-smoke-sponsor | ✗      | 0     | 0     | 0       | 0
    47	
    48	§5 Proposal flow (chronological by logical_t)
    49	----------------------------------------------
    50	  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
    51	  ------+-----+-----------------+------------+------------+------------+--------+-------
    52	  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
    53	  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
    54	  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
    55	  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
    56	  L4    |   3 | TerminalSummary | -          | -          | -          | -      | -
    57	
    58	§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
    59	------------------------------------------------------------------------
    60	  parent_tx_state: NoMultiAttemptObserved (DAG not exercised this run — conformance test demonstrates plumbing)
    61	  edges: (none — see parent_tx_state above for interpretation)
    62	
    63	§7 Golden path (root → oracle-verified WorkTx)
    64	------------------------------------------------
    65	  (no oracle-verified WorkTx on chain — chain_oracle_verified=false)

exec
/bin/bash -lc 'rg -n "default_pput_preseed_pairs|genesis_with_balances|AgentKeypairRegistry|agent_pubkeys|build_chaintape|initial_q_state" src/runtime/mod.rs experiments/minif2f_v4/src/bin/evaluator.rs experiments/minif2f_v4/src/bin/lean_market.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/lean_market.rs:370:/// `build_chaintape_sequencer` fails-closed on non-empty repo per
experiments/minif2f_v4/src/bin/lean_market.rs:676:    let initial_q_path = runtime_repo.join("initial_q_state.json");
experiments/minif2f_v4/src/bin/evaluator.rs:710:                    // `src/runtime/bootstrap.rs::default_pput_preseed_pairs()`.
experiments/minif2f_v4/src/bin/evaluator.rs:717:                    let pairs = turingosv4::runtime::bootstrap::default_pput_preseed_pairs();
experiments/minif2f_v4/src/bin/evaluator.rs:722:                    let initial_q = turingosv4::runtime::adapter::genesis_with_balances(&pairs);
experiments/minif2f_v4/src/bin/evaluator.rs:727:                    turingosv4::runtime::build_chaintape_sequencer_with_initial_q(
experiments/minif2f_v4/src/bin/evaluator.rs:731:                    turingosv4::runtime::build_chaintape_sequencer(&cfg)
experiments/minif2f_v4/src/bin/evaluator.rs:750:    // TB-7 Atom 2 + TB-9 Atom 2: per-run AgentKeypairRegistry holds Ed25519
experiments/minif2f_v4/src/bin/evaluator.rs:753:    // <runtime_repo>/agent_pubkeys.json (TB-7 replay sidecar; unchanged).
experiments/minif2f_v4/src/bin/evaluator.rs:766:    // run loop (interior mutability needed for AgentKeypairRegistry::sign).
experiments/minif2f_v4/src/bin/evaluator.rs:767:    let agent_keypairs: Option<Arc<Mutex<turingosv4::runtime::agent_keypairs::AgentKeypairRegistry>>> =
experiments/minif2f_v4/src/bin/evaluator.rs:772:            let reg = turingosv4::runtime::agent_keypairs::AgentKeypairRegistry::generate_or_load_durable(
experiments/minif2f_v4/src/bin/evaluator.rs:1044:        // runtime_repo, cas_path, system_pubkey, agent_pubkeys path,
experiments/minif2f_v4/src/bin/evaluator.rs:1084:            agent_pubkeys_path: "agent_pubkeys.json".into(),
src/runtime/mod.rs:47:/// TRACE_MATRIX § 3 orphan (see `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`): TB-7R Deliverable C — `genesis_report.json` emitter capturing constitution_hash + runtime_repo + cas_path + system_pubkey_hash + agent_pubkeys_path + initial_balances + (preseed only) task_id / task_open_tx / escrow_lock_tx. No canonical FC row exists yet (FC2 is Append/Submit, NOT Boot/Genesis); promotion target is a future TRACE_MATRIX revision under Article IV Boot. `FC-trace: Art.IV Boot + Art.I.1 + Art.III.4 + WP-§11`.
src/runtime/mod.rs:155:/// Bundle of runtime handles produced by `build_chaintape_sequencer`.
src/runtime/mod.rs:353:pub fn build_chaintape_sequencer(
src/runtime/mod.rs:356:    build_chaintape_sequencer_with_initial_q(config, QState::genesis())
src/runtime/mod.rs:364:/// `build_chaintape_sequencer` delegates here with `QState::genesis()`.
src/runtime/mod.rs:369:pub fn build_chaintape_sequencer_with_initial_q(
src/runtime/mod.rs:431:    // TB-7.7 D7 fix: persist initial_q to <runtime_repo>/initial_q_state.json so
src/runtime/mod.rs:438:    let initial_q_path = config.runtime_repo_path.join("initial_q_state.json");
src/runtime/mod.rs:541:    async fn build_chaintape_sequencer_returns_non_none_sequencer_with_git_writer() {
src/runtime/mod.rs:544:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
src/runtime/mod.rs:555:    async fn build_chaintape_sequencer_writes_pinned_pubkeys_json_to_runtime_repo() {
src/runtime/mod.rs:558:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");
src/runtime/mod.rs:572:    async fn build_chaintape_sequencer_fails_on_non_empty_repo() {
src/runtime/mod.rs:576:        let bundle = build_chaintape_sequencer(&cfg).expect("first bootstrap");
src/runtime/mod.rs:589:        let bundle2 = build_chaintape_sequencer(&cfg).expect("second bootstrap on empty refs");
src/runtime/mod.rs:597:        let bundle = build_chaintape_sequencer(&cfg).expect("bootstrap");

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '700,785p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   700	    // TB-7R Deliverable C: capture initial balances seeded into the genesis
   701	    // QState so the genesis_report.json can record them as the run's starting
   702	    // economic state. Empty when preseed disabled.
   703	    let mut initial_balances_for_genesis_report: Vec<(String, i64)> = Vec::new();
   704	    let chaintape_bundle: Option<turingosv4::runtime::ChaintapeBundle> =
   705	        match turingosv4::runtime::RuntimeChaintapeConfig::from_env() {
   706	            None => None, // env unset = legacy mode is the explicit choice
   707	            Some(cfg) => {
   708	                let result = if chaintape_preseed_enabled {
   709	                    // TB-10 Atom 1: preseed list extracted to runtime factory at
   710	                    // `src/runtime/bootstrap.rs::default_pput_preseed_pairs()`.
   711	                    // Single source of truth shared between evaluator and
   712	                    // `lean_market` user CLI so both processes bootstrap to the
   713	                    // same genesis QState. Includes:
   714	                    //   - tb7-7-sponsor (10_000_000 micro) — TB-7.7 D3 self-fund
   715	                    //   - Agent_user_0  (10_000_000 micro) — TB-10 user CLI sponsor
   716	                    //   - Agent_0..9    ( 1_000_000 micro each) — solver budgets
   717	                    let pairs = turingosv4::runtime::bootstrap::default_pput_preseed_pairs();
   718	                    initial_balances_for_genesis_report = pairs
   719	                        .iter()
   720	                        .map(|(a, m)| (a.0.clone(), m.micro_units()))
   721	                        .collect();
   722	                    let initial_q = turingosv4::runtime::adapter::genesis_with_balances(&pairs);
   723	                    info!(
   724	                        "[chaintape/d3] pre-seed enabled (TB-10 factory): {} entries",
   725	                        pairs.len()
   726	                    );
   727	                    turingosv4::runtime::build_chaintape_sequencer_with_initial_q(
   728	                        &cfg, initial_q,
   729	                    )
   730	                } else {
   731	                    turingosv4::runtime::build_chaintape_sequencer(&cfg)
   732	                };
   733	                match result {
   734	                    Ok(b) => Some(b),
   735	                    Err(e) => {
   736	                        error!(
   737	                            "[chaintape] bootstrap failed under TURINGOS_CHAINTAPE_PATH (declared \
   738	                             ChainTape mode); exiting non-zero per TB-7 Atom 1.7 fail-closed \
   739	                             (Codex audit action #1). Error: {e}"
   740	                        );
   741	                        std::process::exit(2);
   742	                    }
   743	                }
   744	            }
   745	        };
   746	    if chaintape_bundle.is_some() && std::env::var("WAL_DIR").is_ok() {
   747	        info!("[chaintape] WAL_DIR ignored when TURINGOS_CHAINTAPE_PATH is set");
   748	    }
   749	
   750	    // TB-7 Atom 2 + TB-9 Atom 2: per-run AgentKeypairRegistry holds Ed25519
   751	    // keypairs for every distinct agent_id that submits a real-LLM proposal
   752	    // through bus.submit_typed_tx. Public keys are persisted per-run to
   753	    // <runtime_repo>/agent_pubkeys.json (TB-7 replay sidecar; unchanged).
   754	    //
   755	    // **TB-9 (2026-05-02)**: secrets are persisted across runs to an encrypted
   756	    // durable keystore at TURINGOS_AGENT_KEYSTORE_PATH (default
   757	    // ~/.turingos/keystore/agent_keystore.enc). Cross-run identity is the
   758	    // architect TB-9 mandate ("agent durable key registry" + "cross-run
   759	    // identity"; directive 2026-05-02 Part C line 1574). The keystore password
   760	    // is read from TURINGOS_AGENT_KEYSTORE_PASSWORD; if unset, a hardcoded
   761	    // local-dev fallback is used (acceptable for solo-runs per
   762	    // feedback_kolmogorov_compression "MVP env-var; production-grade prompt is
   763	    // post-v1.0 polish"). Tests / CI set the env var explicitly.
   764	    //
   765	    // Wrapped in Arc<Mutex<>> so the registry can be shared across the async
   766	    // run loop (interior mutability needed for AgentKeypairRegistry::sign).
   767	    let agent_keypairs: Option<Arc<Mutex<turingosv4::runtime::agent_keypairs::AgentKeypairRegistry>>> =
   768	        chaintape_bundle.as_ref().map(|b| {
   769	            let durable_path = turingosv4::runtime::agent_keystore::default_agent_keystore_path()
   770	                .expect("[chaintape/tb9] resolve durable agent keystore path (set HOME or TURINGOS_AGENT_KEYSTORE_PATH)");
   771	            let pwd = turingosv4::runtime::agent_keystore::keystore_password_from_env();
   772	            let reg = turingosv4::runtime::agent_keypairs::AgentKeypairRegistry::generate_or_load_durable(
   773	                &b.runtime_repo_path,
   774	                &durable_path,
   775	                pwd,
   776	            )
   777	            .expect(
   778	                "[chaintape/tb9] agent_keypairs durable init must succeed (fresh runtime_repo guarantees \
   779	                 manifest absent; if you see this on a non-fresh dir, see TB-6 NonEmptyRuntimeRepo. \
   780	                 If you see a keystore decrypt error, check TURINGOS_AGENT_KEYSTORE_PASSWORD matches \
   781	                 the password used for the previous run.)",
   782	            );
   783	            Arc::new(Mutex::new(reg))
   784	        });
   785	

exec
/bin/bash -lc "nl -ba src/runtime/mod.rs | sed -n '369,445p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   369	pub fn build_chaintape_sequencer_with_initial_q(
   370	    config: &RuntimeChaintapeConfig,
   371	    initial_q: QState,
   372	) -> Result<ChaintapeBundle, BootstrapError> {
   373	    // Step 1: open or init runtime repo, fail-closed on existing chain.
   374	    std::fs::create_dir_all(&config.runtime_repo_path)?;
   375	    let git_writer = Git2LedgerWriter::open(&config.runtime_repo_path)
   376	        .map_err(|e| BootstrapError::LedgerWriter(e.to_string()))?;
   377	    if git_writer.head_commit_oid().is_some() {
   378	        let existing_head = git_writer
   379	            .head_commit_oid()
   380	            .map(|o| o.to_string())
   381	            .unwrap_or_default();
   382	        return Err(BootstrapError::NonEmptyRuntimeRepo {
   383	            path: config.runtime_repo_path.clone(),
   384	            existing_head,
   385	        });
   386	    }
   387	    let transition_writer: Arc<RwLock<dyn LedgerWriter>> = Arc::new(RwLock::new(git_writer));
   388	
   389	    // Step 2: open CAS.
   390	    std::fs::create_dir_all(&config.cas_path)?;
   391	    let cas_store = CasStore::open(&config.cas_path)
   392	        .map_err(|e| BootstrapError::Cas(e.to_string()))?;
   393	    let cas = Arc::new(RwLock::new(cas_store));
   394	
   395	    // Step 3: generate keypair + persist pinned-pubkey manifest.
   396	    let keypair = Arc::new(
   397	        Ed25519Keypair::generate_with_secure_entropy()
   398	            .map_err(|e| BootstrapError::Keypair(e.to_string()))?,
   399	    );
   400	    let epoch = SystemEpoch::new(1);
   401	    write_pinned_pubkey_manifest(&config.runtime_repo_path, epoch, &keypair, &config.run_id)?;
   402	    let mut pinned = PinnedSystemPubkeys::new();
   403	    pinned.insert(epoch, keypair.public_key());
   404	    let pinned_pubkeys = Arc::new(pinned);
   405	
   406	    // Step 4: rejection writer — JSONL-backed at <runtime_repo>/rejections.jsonl
   407	    // per Atom 1.2 + architect § 3.5 deliverable shape.
   408	    //
   409	    // **TB-7 Atom 1.7 (Codex audit cc7b3dd action item #1)**: fail-closed
   410	    // when JSONL open fails. Silent in-memory fallback is the same
   411	    // anti-pattern as legacy `bus.append` as authoritative state mutation:
   412	    // a chain-backed run that secretly drops L4.E writes is worse than a
   413	    // failed boot. ChainTape mode is contractually a fail-closed declaration.
   414	    let rejections_path = config.runtime_repo_path.join("rejections.jsonl");
   415	    let rejection_writer = match RejectionEvidenceWriter::open_jsonl(rejections_path.clone()) {
   416	        Ok(w) => Arc::new(RwLock::new(w)),
   417	        Err(e) => {
   418	            return Err(BootstrapError::RejectionWriter(format!(
   419	                "open_jsonl({:?}) failed: {e}",
   420	                rejections_path
   421	            )));
   422	        }
   423	    };
   424	
   425	    // Step 5: predicate + tool registries (default empty registries — production-binary
   426	    // is responsible for registering predicates / tools before submitting txs).
   427	    let predicate_registry = Arc::new(PredicateRegistry::new());
   428	    let tool_registry = Arc::new(ToolRegistry::new());
   429	
   430	    // Step 6: initial QState (caller-provided; base factory passes QState::genesis()).
   431	    // TB-7.7 D7 fix: persist initial_q to <runtime_repo>/initial_q_state.json so
   432	    // verify_chaintape can replay from the same starting point. Without this,
   433	    // pre-seeded balances / open task markets seen at runtime are absent during
   434	    // replay, causing a state divergence that classifies as
   435	    // (state_reconstructed=false, economic_state_reconstructed=false). Per
   436	    // verify.rs:264-272 this file is the authoritative initial-Q manifest;
   437	    // omitting it forces replay to start from QState::genesis().
   438	    let initial_q_path = config.runtime_repo_path.join("initial_q_state.json");
   439	    let initial_q_json = serde_json::to_string_pretty(&initial_q)
   440	        .map_err(|e| BootstrapError::Cas(format!("initial_q serialize: {e}")))?;
   441	    std::fs::write(&initial_q_path, initial_q_json)?;
   442	
   443	    // Step 7: construct Sequencer.
   444	    let (sequencer, queue_rx) = Sequencer::new(
   445	        cas,

exec
/bin/bash -lc "nl -ba src/runtime/mod.rs | sed -n '443,475p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   443	    // Step 7: construct Sequencer.
   444	    let (sequencer, queue_rx) = Sequencer::new(
   445	        cas,
   446	        keypair,
   447	        epoch,
   448	        transition_writer.clone(),
   449	        rejection_writer.clone(),
   450	        predicate_registry,
   451	        tool_registry,
   452	        pinned_pubkeys,
   453	        initial_q,
   454	        config.queue_capacity,
   455	    );
   456	    let sequencer = Arc::new(sequencer);
   457	
   458	    // Step 8: spawn driver wrapper + shutdown channel.
   459	    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
   460	    let driver_seq = sequencer.clone();
   461	    let driver_handle = tokio::spawn(async move {
   462	        run_chaintape_driver(driver_seq, queue_rx, shutdown_rx).await;
   463	    });
   464	
   465	    Ok(ChaintapeBundle {
   466	        sequencer,
   467	        transition_writer,
   468	        rejection_writer,
   469	        epoch,
   470	        runtime_repo_path: config.runtime_repo_path.clone(),
   471	        cas_path: config.cas_path.clone(),
   472	        driver_handle,
   473	        shutdown_tx,
   474	    })
   475	}

exec
/bin/bash -lc 'rg -n "assert_total|total_supply_micro|assert_no_post_init_mint|monetary_invariant|conditional_collateral|node_positions" src/economy src/state/sequencer.rs src/runtime/audit_assertions.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/runtime/audit_assertions.rs:1020:fn replayed_total_supply_micro(q: &QState) -> i128 {
src/runtime/audit_assertions.rs:1031:    for (_, mc) in &q.economic_state_t.conditional_collateral_t.0 {
src/runtime/audit_assertions.rs:1042:    // sequencer-side `assert_no_post_init_mint` fires inline. If replay
src/runtime/audit_assertions.rs:1070:    let total = replayed_total_supply_micro(q);
src/runtime/audit_assertions.rs:1106:    for (event_id, mc) in &q.economic_state_t.conditional_collateral_t.0 {
src/runtime/audit_assertions.rs:1159:pub fn assert_21_node_positions_excluded_from_supply(t: &LoadedTape) -> AssertionResult {
src/runtime/audit_assertions.rs:1160:    // Structural: source-level fence — node_positions_t entries are NOT
src/runtime/audit_assertions.rs:1163:    // showing it would diverge whenever node_positions_t is non-empty.
src/runtime/audit_assertions.rs:1169:                "node_positions_excluded_from_supply",
src/runtime/audit_assertions.rs:1175:    let baseline = replayed_total_supply_micro(q);
src/runtime/audit_assertions.rs:1177:    for (_, pos) in &q.economic_state_t.node_positions_t.0 {
src/runtime/audit_assertions.rs:1180:    if q.economic_state_t.node_positions_t.0.is_empty()
src/runtime/audit_assertions.rs:1185:        AssertionResult::pass(21, "node_positions_excluded_from_supply", AssertionLayer::D)
src/runtime/audit_assertions.rs:1189:            "node_positions_excluded_from_supply",
src/runtime/audit_assertions.rs:1191:            "including node_positions did not change total — implies they were already counted (CR-12.1 violation)".into(),
src/runtime/audit_assertions.rs:1209:    let baseline = replayed_total_supply_micro(q);
src/runtime/audit_assertions.rs:1949:    r.push(assert_21_node_positions_excluded_from_supply(&tape));
src/state/sequencer.rs:37:use crate::economy::monetary_invariant::{
src/state/sequencer.rs:38:    assert_claim_amount_backed_by_escrow, assert_no_post_init_mint, assert_read_is_free,
src/state/sequencer.rs:39:    assert_task_market_total_escrow_matches_locks, assert_total_ctf_conserved,
src/state/sequencer.rs:374:        TransitionError::MonetaryInvariantViolation => Some("monetary_invariant".into()),
src/state/sequencer.rs:592:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:645:                    .node_positions_t
src/state/sequencer.rs:656:            assert_total_ctf_conserved(
src/state/sequencer.rs:802:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:804:            assert_total_ctf_conserved(
src/state/sequencer.rs:906:                    .node_positions_t
src/state/sequencer.rs:912:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:914:            assert_total_ctf_conserved(
src/state/sequencer.rs:1117:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1122:            assert_total_ctf_conserved(
src/state/sequencer.rs:1264:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1266:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1311:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1313:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1396:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1398:            assert_total_ctf_conserved(&q.economic_state_t, &q_next.economic_state_t, &[])
src/state/sequencer.rs:1477:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1479:            assert_total_ctf_conserved(
src/state/sequencer.rs:1523:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1525:            assert_total_ctf_conserved(
src/state/sequencer.rs:1585:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1587:            assert_total_ctf_conserved(
src/state/sequencer.rs:1612:        // conditional_collateral_t[event_id] by amount; credits BOTH
src/state/sequencer.rs:1663:            // monetary_invariant extension) treats conditional_collateral_t
src/state/sequencer.rs:1664:            // as a Coin holding, so total_supply_micro is preserved
src/state/sequencer.rs:1674:                .conditional_collateral_t
src/state/sequencer.rs:1698:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1700:            assert_total_ctf_conserved(
src/state/sequencer.rs:1710:            crate::economy::monetary_invariant::assert_complete_set_balanced(
src/state/sequencer.rs:1782:                .conditional_collateral_t
src/state/sequencer.rs:1821:                    .conditional_collateral_t
src/state/sequencer.rs:1845:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1847:            assert_total_ctf_conserved(
src/state/sequencer.rs:1853:            crate::economy::monetary_invariant::assert_complete_set_balanced(
src/state/sequencer.rs:1915:                .conditional_collateral_t
src/state/sequencer.rs:1939:            assert_no_post_init_mint(tx, q)
src/state/sequencer.rs:1941:            assert_total_ctf_conserved(
src/state/sequencer.rs:1947:            crate::economy::monetary_invariant::assert_complete_set_balanced(
src/state/sequencer.rs:3887:        // point, but assert_no_post_init_mint is permissive at genesis since on_init_tx
src/economy/mod.rs:13:pub mod monetary_invariant;
src/economy/monetary_invariant.rs:42:    /// [`assert_total_ctf_conserved`] when `delta_micro > 0` and no
src/economy/monetary_invariant.rs:174:fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
src/economy/monetary_invariant.rs:195:    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
src/economy/monetary_invariant.rs:198:    // Coin from balances_t to conditional_collateral_t) would falsely
src/economy/monetary_invariant.rs:199:    // appear to burn money, failing assert_total_ctf_conserved with empty
src/economy/monetary_invariant.rs:203:    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
src/economy/monetary_invariant.rs:206:    for c in s.conditional_collateral_t.0.values() {
src/economy/monetary_invariant.rs:308:// assert_no_post_init_mint — structural guard at the tx layer
src/economy/monetary_invariant.rs:323:/// [`assert_total_ctf_conserved`] separately.
src/economy/monetary_invariant.rs:324:pub fn assert_no_post_init_mint(tx: &TypedTx, q: &QState) -> Result<(), MonetaryError> {
src/economy/monetary_invariant.rs:339:        // conservation via assert_total_ctf_conserved with empty exempt list.
src/economy/monetary_invariant.rs:346:        // CTF conservation enforced by assert_total_ctf_conserved with
src/economy/monetary_invariant.rs:352:        // CTF conservation enforced by assert_total_ctf_conserved with
src/economy/monetary_invariant.rs:362:        // assert_total_ctf_conserved with conditional_collateral_t as the
src/economy/monetary_invariant.rs:371:// assert_total_ctf_conserved — numeric conservation across a transition
src/economy/monetary_invariant.rs:387:pub fn assert_total_ctf_conserved(
src/economy/monetary_invariant.rs:392:    let total_before = total_supply_micro(before)?;
src/economy/monetary_invariant.rs:393:    let total_after = total_supply_micro(after)?;
src/economy/monetary_invariant.rs:437:/// For every event in `conditional_collateral_t`:
src/economy/monetary_invariant.rs:463:    for (event_id, collateral) in s.conditional_collateral_t.0.iter() {
src/economy/monetary_invariant.rs:545:    // ── assert_no_post_init_mint ────────────────────────────────────────────
src/economy/monetary_invariant.rs:551:        assert_eq!(assert_no_post_init_mint(&work, &q), Ok(()));
src/economy/monetary_invariant.rs:576:            assert_eq!(assert_no_post_init_mint(&t, &q), Ok(()),
src/economy/monetary_invariant.rs:581:    // ── assert_total_ctf_conserved ──────────────────────────────────────────
src/economy/monetary_invariant.rs:591:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:600:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:612:        let r = assert_total_ctf_conserved(&before, &after, &[]);
src/economy/monetary_invariant.rs:625:            assert_total_ctf_conserved(&before, &after, &[TxKind::FinalizeReward]),
src/economy/monetary_invariant.rs:641:        assert_eq!(assert_total_ctf_conserved(&before, &after, &[]), Ok(()));
src/economy/monetary_invariant.rs:654:        let total0 = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:675:            // total_supply_micro unchanged.)
src/economy/monetary_invariant.rs:691:            let total_now = total_supply_micro(&s).unwrap();
src/economy/monetary_invariant.rs:699:        assert_eq!(total_supply_micro(&s).unwrap(), total0);
src/economy/monetary_invariant.rs:750:        assert_eq!(total_supply_micro(&s).unwrap(), 55 * MICRO_PER_COIN);
src/economy/monetary_invariant.rs:758:        // must yield total_supply_micro = K, NOT 2K. If a regression adds
src/economy/monetary_invariant.rs:777:            total_supply_micro(&s).unwrap(),

exec
/bin/bash -lc "nl -ba src/economy/monetary_invariant.rs | sed -n '120,230p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   120	            Self::Overflow => write!(f, "i64 overflow while summing economic state"),
   121	        }
   122	    }
   123	}
   124	
   125	impl std::error::Error for MonetaryError {}
   126	
   127	// ────────────────────────────────────────────────────────────────────────────
   128	// total_supply — sum of all coin-holding fields in EconomicState
   129	// ────────────────────────────────────────────────────────────────────────────
   130	
   131	/// Sum of every coin-holding sub-index in `EconomicState`, in micro-units.
   132	///
   133	/// Counted (each contributes its `MicroCoin` directly) — **4 holdings** post-TB-8:
   134	/// - `balances_t` (agent-held)
   135	/// - `escrows_t` (locked under task; populated by `EscrowLockTx`)
   136	/// - `stakes_t` (locked under tx; populated by accepted WorkTx commitment)
   137	/// - `challenge_cases_t.bond` (challenger-locked under case)
   138	///
   139	/// NOT counted (not a holding):
   140	/// - `reputations_t` (signed reputation, not coin)
   141	/// - `royalty_graph_t` (edges, no coin)
   142	/// - TB-14 `compute_price_index` derived view (signal-not-truth per
   143	///   architect §5.1; not stored on `EconomicState` so trivially not in
   144	///   the sum; legacy `price_index_t` field removed in TB-14 Atom 2)
   145	/// - **`task_markets_t.total_escrow`** (derived aggregate / cached index per
   146	///   TB-3 charter § 3.2 — counting it would double-mint every locked bounty
   147	///   because the same money is also in `escrows_t`. Cache=truth is enforced
   148	///   separately by `assert_task_market_total_escrow_matches_locks`.)
   149	/// - **`claims_t.amount`** (intent metadata, NOT a holding — see TB-8 5→4 below)
   150	///
   151	/// **TB-3 6→5 holding migration** (2026-04-30): TB-1's `bounty` term over
   152	/// `task_markets_t[t].bounty` is removed. Bounty money has migrated to
   153	/// `escrows_t.amount` via accepted `EscrowLockTx`. `task_markets_t` retains
   154	/// only the cached aggregate `total_escrow` (NOT in supply sum) + admission
   155	/// metadata.
   156	///
   157	/// **TB-8 5→4 holding migration** (2026-05-02): `claims_t.amount` is removed
   158	/// from the holding sum. Per TB-8 charter §3 Atom 3 + ratification §1 Q5:
   159	/// the FinalizeReward dispatch arm moves money DIRECTLY from `escrows_t` to
   160	/// `balances_t` (not via claims_t as an intermediate holding). `claims_t` is
   161	/// the *intent registry*: claim creation at OMEGA-Confirm records "this
   162	/// solver is owed this amount" without moving money; the money still lives
   163	/// in `escrows_t` until finalize debits it. The `claim.amount` field is the
   164	/// cached intent (= `task_market.total_escrow` at claim creation per single-
   165	/// solver MVP). Counting `claims_t` here while ALSO counting the backing
   166	/// `escrows_t` rows would double-mint every claim. The intent-vs-backing
   167	/// integrity is enforced separately by
   168	/// [`assert_claim_amount_backed_by_escrow`].
   169	///
   170	/// **Pre-TB-8 baseline**: `claims_t` was always empty (the dispatch arm was
   171	/// `NotYetImplemented`); removing it from the sum changes nothing for
   172	/// historical L4 replay (forward-only schema migration per
   173	/// `feedback_no_retroactive_evidence_rewrite`).
   174	fn total_supply_micro(s: &EconomicState) -> Result<i64, MonetaryError> {
   175	    let mut total: i64 = 0;
   176	    for v in s.balances_t.0.values() {
   177	        total = total.checked_add(v.micro_units()).ok_or(MonetaryError::Overflow)?;
   178	    }
   179	    for e in s.escrows_t.0.values() {
   180	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   181	    }
   182	    for e in s.stakes_t.0.values() {
   183	        total = total.checked_add(e.amount.micro_units()).ok_or(MonetaryError::Overflow)?;
   184	    }
   185	    // claims_t is INTENTIONALLY OMITTED — intent registry, not a holding
   186	    // (TB-8 charter §3 Atom 3 + ratification §1 Q5). The backing money lives
   187	    // in escrows_t; counting claims_t here would double-mint every claim.
   188	    // task_markets_t.total_escrow is INTENTIONALLY OMITTED — derived cache,
   189	    // not a holding (TB-3 charter § 3.2). Counting it would double-mint
   190	    // every bounty: the same micro-coins are already counted in escrows_t.
   191	    for c in s.challenge_cases_t.0.values() {
   192	        total = total.checked_add(c.bond.micro_units()).ok_or(MonetaryError::Overflow)?;
   193	    }
   194	    // TB-13 Atom 2 (architect 2026-05-03 post-TB-12 ruling Part A §4.3 +
   195	    // CR-13.4): conditional_collateral_t IS a Coin holding — locked Coin
   196	    // held against outstanding YES_E + NO_E share inventory. Extends the
   197	    // 5-holding sum to 6. Without this, CompleteSetMintTx (which migrates
   198	    // Coin from balances_t to conditional_collateral_t) would falsely
   199	    // appear to burn money, failing assert_total_ctf_conserved with empty
   200	    // exempt list.
   201	    //
   202	    // conditional_share_balances_t is INTENTIONALLY OMITTED per CR-13.3 +
   203	    // SG-13.2 — shares are CLAIMS against conditional_collateral_t, not
   204	    // a holding. Counting them would triple-count (shares are derived from
   205	    // collateral; including both creates a 2x parallel ledger).
   206	    for c in s.conditional_collateral_t.0.values() {
   207	        total = total.checked_add(c.micro_units()).ok_or(MonetaryError::Overflow)?;
   208	    }
   209	    Ok(total)
   210	}
   211	
   212	// ────────────────────────────────────────────────────────────────────────────
   213	// TB-8 Atom 1 — assert_claim_amount_backed_by_escrow (intent-vs-backing)
   214	// ────────────────────────────────────────────────────────────────────────────
   215	
   216	/// TRACE_MATRIX TB-8 charter §3 Atom 1 + Atom 3 — claim-intent-vs-escrow-
   217	/// backing invariant.
   218	///
   219	/// Asserts that for every Open `claims_t` entry, the claim's intended payout
   220	/// (`claim.amount`) is ≤ the backing escrow row (`escrows_t[claim.escrow_lock_tx_id].amount`).
   221	/// Replaces the old "claims_t is a holding" semantics with the explicit
   222	/// intent-vs-backing check: a claim cannot promise more than its escrow
   223	/// holds. Finalized claims are excluded — once finalized, the escrow has been
   224	/// debited and the balance credited, so the integrity check no longer applies
   225	/// (claim.amount is now historical).
   226	///
   227	/// **Caller convention**: invoked from any dispatch arm that mutates
   228	/// `claims_t` or `escrows_t`. TB-8 dispatch sites:
   229	/// - Atom 1 (Verify-Confirm claim creation): post-mutation on `q_next`.
   230	/// - Atom 3 (FinalizeReward dispatch): post-mutation on `q_next` (the

exec
/bin/bash -lc "nl -ba src/state/sequencer.rs | sed -n '830,930p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   830	            // Step 2: stake positivity.
   831	            if challenge.stake.micro_units() == 0 {
   832	                return Err(TransitionError::StakeInsufficient);
   833	            }
   834	            // Step 3: target liveness — same gate as Verify arm.
   835	            if !q.economic_state_t.stakes_t.0.contains_key(&challenge.target_work_tx) {
   836	                return Err(TransitionError::TargetWorkInactive);
   837	            }
   838	            // Step 4: challenger solvency.
   839	            let challenger_bal = q.economic_state_t.balances_t.0
   840	                .get(&challenge.challenger_agent)
   841	                .copied()
   842	                .unwrap_or(crate::economy::money::MicroCoin::zero());
   843	            if challenger_bal.micro_units() < challenge.stake.micro_units() {
   844	                return Err(TransitionError::InsufficientBalance);
   845	            }
   846	            // Step 5: counterexample non-empty (charter § 3.5 step 6 +
   847	            // directive Q7).
   848	            if challenge.counterexample_cid == Cid([0u8; 32]) {
   849	                return Err(TransitionError::EmptyCounterexample);
   850	            }
   851	            // Step 6: q_next — atomic balance → challenge_cases_t transfer.
   852	            // opened_at_round = q.logical_t (challenge-window structural
   853	            // anchor per § 3.9; closure / deadline / auto-finalize NOT
   854	            // installed in TB-4).
   855	            let mut q_next = q.clone();
   856	            let new_bal_micro = challenger_bal.micro_units() - challenge.stake.micro_units();
   857	            q_next.economic_state_t.balances_t.0.insert(
   858	                challenge.challenger_agent.clone(),
   859	                crate::economy::money::MicroCoin::from_micro_units(new_bal_micro),
   860	            );
   861	            q_next.economic_state_t.challenge_cases_t.0.insert(
   862	                challenge.tx_id.clone(),
   863	                crate::state::q_state::ChallengeCase {
   864	                    challenger: challenge.challenger_agent.clone(),
   865	                    bond: challenge.stake.0,
   866	                    opened_at_round: q.q_t.current_round, // ← § 3.9 anchor
   867	                    target_work_tx: challenge.target_work_tx.clone(),
   868	                    status: crate::state::q_state::ChallengeStatus::Open, // TB-5 ABI default
   869	                },
   870	            );
   871	            // ──────────────────────────────────────────────────────────────
   872	            // TB-12 Atom 2 (architect 2026-05-03 ruling §3 + §8 Atom 2):
   873	            // accepted ChallengeTx with stake > 0 derives a `ChallengeShort`
   874	            // NodePosition exposure record. Pure additive index write —
   875	            // **no money mutation**, **no change** to balances_t /
   876	            // challenge_cases_t / total_supply (those are handled above by
   877	            // TB-4 economic logic). NodePosition.amount is **NOT a Coin
   878	            // holding** per CR-12.1 + CR-12.2; the 5-holding CTF sum stays
   879	            // unchanged. FR-12.2 + FR-12.5: kind = ChallengeShort; node_id
   880	            // = challenge.target_work_tx; position_id = source_tx =
   881	            // challenge.tx_id. task_id derived via stakes_t[target_work_tx]
   882	            // (the target's stake row holds the task_id backref).
   883	            // ──────────────────────────────────────────────────────────────
   884	            if challenge.stake.micro_units() > 0 {
   885	                // Q-derive task_id from the target WorkTx's stake row.
   886	                let task_id_for_position = q
   887	                    .economic_state_t
   888	                    .stakes_t
   889	                    .0
   890	                    .get(&challenge.target_work_tx)
   891	                    .map(|s| s.task_id.clone())
   892	                    .unwrap_or_default();
   893	                let position = crate::state::typed_tx::NodePosition {
   894	                    position_id: challenge.tx_id.clone(),
   895	                    node_id: challenge.target_work_tx.clone(),
   896	                    task_id: task_id_for_position,
   897	                    owner: challenge.challenger_agent.clone(),
   898	                    side: crate::state::typed_tx::PositionSide::Short,
   899	                    kind: crate::state::typed_tx::PositionKind::ChallengeShort,
   900	                    amount: challenge.stake.0,
   901	                    source_tx: challenge.tx_id.clone(),
   902	                    opened_at_round: challenge.timestamp_logical,
   903	                };
   904	                q_next
   905	                    .economic_state_t
   906	                    .node_positions_t
   907	                    .0
   908	                    .insert(challenge.tx_id.clone(), position);
   909	            }
   910	            // Step 7: monetary invariants (debit = credit; challenge_cases.bond
   911	            // is the 5th holding term).
   912	            assert_no_post_init_mint(tx, q)
   913	                .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
   914	            assert_total_ctf_conserved(
   915	                &q.economic_state_t,
   916	                &q_next.economic_state_t,
   917	                &[],
   918	            )
   919	            .map_err(|_| TransitionError::MonetaryInvariantViolation)?;
   920	            // Step 8: state_root advance via CHALLENGE_ACCEPT_DOMAIN_V1.
   921	            q_next.state_root_t = challenge_accept_state_root(&q.state_root_t, tx);
   922	
   923	            Ok((q_next, SignalBundle::default()))
   924	        }
   925	        TypedTx::Reuse(_) => Err(TransitionError::NotYetImplemented),
   926	        // ──────────────────────────────────────────────────────────────────
   927	        // TB-8 Atom 3 — FinalizeReward dispatch arm (charter §3 Atom 3 +
   928	        // ratification §1 Q2/Q3/Q4/Q5).
   929	        //
   930	        // Single-solver MVP: debit escrows_t[claim.escrow_lock_tx_id].amount

2026-05-04T05:06:37.416728Z ERROR codex_core::tools::router: error=patch rejected: writing is blocked by read-only sandbox; rejected by user approval settings
Audit is complete. The requested file write was attempted, but the session is read-only and the patch was rejected by sandbox policy.
  ✓ Read governing AGENTS and TB-16 anchor docs
  ✓ Trace source paths for audit assertions, dashboard, replay, privacy, tamper
  ✓ Validate evidence schema and ship-status claims
  ✓ Write R1 verdict content and report save limitation
codex
I completed the R1 audit, but could not save `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md`: the workspace is read-only and the patch write was rejected by sandbox policy. I did not run tests because this session cannot write build/test artifacts.

Key findings:

- Q2/RQ1 VETO: TB-16 did not ship the required arena run. Architect requires compute + position + complete set + price + mask + autopsy and FR-16.2..7 before ship (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md:764-814`), but ship status explicitly defers FR-16.2..7 to Atom 6.1 (`handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md:33-46`). The runner only executes Task A and defers B-F (`handover/tests/scripts/run_real_llm_arena.sh:141-188`).

- Q2/Q8 VETO: shipped evidence is `verdict=BLOCK`, with missing evidence capsule halt. Tx counts show no Work/Verify/Challenge/CompleteSet/MarketSeed/etc. (`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:11-25`), Layer E #27 halts (`.../verdict.json:210-216`), and final verdict is BLOCK (`.../verdict.json:300-320`). Architect §7.7 says unresolved evidence gap is a halt (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md:901-908`).

- Q2/Q7 VETO: the runner masks audit failures. `audit_tape`, tamper, Markov, dashboard, and replay are invoked with `|| true` (`handover/tests/scripts/run_real_llm_arena.sh:189-244`), and the script has no final nonzero exit on BLOCK/replay divergence (`.../run_real_llm_arena.sh:257-267`). Full tx-kind coverage is warning-only by default (`handover/tests/scripts/audit_tape_smoke_test.sh:25-31`, `:82-104`).

- Q7 VETO: tamper detection is contaminated by an already-BLOCK baseline. The harness treats any `verdict == "BLOCK"` as detected (`src/bin/audit_tape_tamper.rs:333-336`); in evidence, `flip_l4_byte` still has L4 hash chain valid passing (`.../tamper_report.json:35-40`) and is “detected” by the same E #27 halt (`.../tamper_report.json:198-201`).

- Q3 VETO: audit conservation is not production-equivalent for ChallengeTx. Audit sum omits `challenge_cases_t.bond` (`src/runtime/audit_assertions.rs:1020-1034`), while production monetary invariant includes it (`src/economy/monetary_invariant.rs:174-209`) and ChallengeTx moves bond there (`src/state/sequencer.rs:851-866`).

- Q1/Q6/RQ4 VETO: sandbox is advisory, not admission-enforced. `sandbox_prefix` excludes `Agent_0..9` (`src/runtime/audit_assertions.rs:538-545`), while default preseed uses `Agent_0..9` (`src/runtime/bootstrap.rs:52-69`). Layer A #3 only scans `agent_pubkeys.json` (`src/runtime/audit_assertions.rs:588-603`), and shipped manifest is empty (`.../runtime_repo/agent_pubkeys.json:1-3`). `submit_agent_tx` has no sandbox-prefix gate (`src/state/sequencer.rs:2457-2484`).

Additional challenges: Q4 projection check targets `q.tape_view_t`, not the actual `UniverseSnapshot` agent path (`src/runtime/audit_assertions.rs:1441-1486`, `src/bus.rs:508-558`); Q5 replay differs from production `apply_one` at system-signature and autopsy-CAS side effects (`src/bottom_white/ledger/transition_ledger.rs:468-510`, `src/state/sequencer.rs:3026-3046`, `:3084-3118`); RQ5 Markov capsule has `previous_capsule_cid: null` (`.../MARKOV_TB-16_2026-05-03.json:36`) and empty `typical_errors` (`:311`).

## VERDICT: VETO
- Q2/RQ1 VETO: required TB-16 arena execution is deferred, not shipped.
- Q2/Q8 VETO: shipped evidence is BLOCK with unresolved evidence gap.
- Q2/Q7 VETO: scripts can pass despite BLOCK and missing full coverage.
- Q3 VETO: audit conservation omits production challenge-bond holdings.
- Q1/Q6/RQ4 VETO: sandbox labeling is not enforced at admission.

Conviction: high. Recommendation: FIX-THEN-PROCEED.
tokens used
257,334
I completed the R1 audit, but could not save `handover/audits/CODEX_TB_16_SHIP_AUDIT_2026-05-04_R1.md`: the workspace is read-only and the patch write was rejected by sandbox policy. I did not run tests because this session cannot write build/test artifacts.

Key findings:

- Q2/RQ1 VETO: TB-16 did not ship the required arena run. Architect requires compute + position + complete set + price + mask + autopsy and FR-16.2..7 before ship (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md:764-814`), but ship status explicitly defers FR-16.2..7 to Atom 6.1 (`handover/ai-direct/TB-16_SHIP_STATUS_2026-05-04.md:33-46`). The runner only executes Task A and defers B-F (`handover/tests/scripts/run_real_llm_arena.sh:141-188`).

- Q2/Q8 VETO: shipped evidence is `verdict=BLOCK`, with missing evidence capsule halt. Tx counts show no Work/Verify/Challenge/CompleteSet/MarketSeed/etc. (`handover/evidence/tb_16_real_llm_arena_2026-05-04/audit_pipeline_smoke/verdict.json:11-25`), Layer E #27 halts (`.../verdict.json:210-216`), and final verdict is BLOCK (`.../verdict.json:300-320`). Architect §7.7 says unresolved evidence gap is a halt (`handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md:901-908`).

- Q2/Q7 VETO: the runner masks audit failures. `audit_tape`, tamper, Markov, dashboard, and replay are invoked with `|| true` (`handover/tests/scripts/run_real_llm_arena.sh:189-244`), and the script has no final nonzero exit on BLOCK/replay divergence (`.../run_real_llm_arena.sh:257-267`). Full tx-kind coverage is warning-only by default (`handover/tests/scripts/audit_tape_smoke_test.sh:25-31`, `:82-104`).

- Q7 VETO: tamper detection is contaminated by an already-BLOCK baseline. The harness treats any `verdict == "BLOCK"` as detected (`src/bin/audit_tape_tamper.rs:333-336`); in evidence, `flip_l4_byte` still has L4 hash chain valid passing (`.../tamper_report.json:35-40`) and is “detected” by the same E #27 halt (`.../tamper_report.json:198-201`).

- Q3 VETO: audit conservation is not production-equivalent for ChallengeTx. Audit sum omits `challenge_cases_t.bond` (`src/runtime/audit_assertions.rs:1020-1034`), while production monetary invariant includes it (`src/economy/monetary_invariant.rs:174-209`) and ChallengeTx moves bond there (`src/state/sequencer.rs:851-866`).

- Q1/Q6/RQ4 VETO: sandbox is advisory, not admission-enforced. `sandbox_prefix` excludes `Agent_0..9` (`src/runtime/audit_assertions.rs:538-545`), while default preseed uses `Agent_0..9` (`src/runtime/bootstrap.rs:52-69`). Layer A #3 only scans `agent_pubkeys.json` (`src/runtime/audit_assertions.rs:588-603`), and shipped manifest is empty (`.../runtime_repo/agent_pubkeys.json:1-3`). `submit_agent_tx` has no sandbox-prefix gate (`src/state/sequencer.rs:2457-2484`).

Additional challenges: Q4 projection check targets `q.tape_view_t`, not the actual `UniverseSnapshot` agent path (`src/runtime/audit_assertions.rs:1441-1486`, `src/bus.rs:508-558`); Q5 replay differs from production `apply_one` at system-signature and autopsy-CAS side effects (`src/bottom_white/ledger/transition_ledger.rs:468-510`, `src/state/sequencer.rs:3026-3046`, `:3084-3118`); RQ5 Markov capsule has `previous_capsule_cid: null` (`.../MARKOV_TB-16_2026-05-03.json:36`) and empty `typical_errors` (`:311`).

## VERDICT: VETO
- Q2/RQ1 VETO: required TB-16 arena execution is deferred, not shipped.
- Q2/Q8 VETO: shipped evidence is BLOCK with unresolved evidence gap.
- Q2/Q7 VETO: scripts can pass despite BLOCK and missing full coverage.
- Q3 VETO: audit conservation omits production challenge-bond holdings.
- Q1/Q6/RQ4 VETO: sandbox labeling is not enforced at admission.

Conviction: high. Recommendation: FIX-THEN-PROCEED.
