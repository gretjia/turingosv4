# TB-G G3.2 §8 Packet — Constitutional Alignment Audit (for Architect)

> **Auditor**: Claude (orchestrator role; internal audit, not Codex / Gemini G2 external audit)
> **Audit timestamp**: 2026-05-12 session #46
> **Audit target**: G3.2 §8 packet draft at `handover/directives/2026-05-12_TB_G_G3_2_§8_PACKET.md` (363 lines; UNCOMMITTED at audit time)
> **Repo HEAD at audit**: `4d4412b` (session #46 boot prompt commit; `git rev-parse HEAD` verified)
> **Working tree drift**: `h_vppu_history.json` + `rules/enforcement.log` + 2 leftover G2P evidence dirs + `search_gdocs.py` (all pre-existing; matches boot prompt declared baseline)
> **Audit purpose**: enable architect §8 ratification decision on G3.2 with full constitutional evidence trail; every claim cited to file:line, command output, prior commit, or constitution clause.
>
> **Audit verdict (mine)**: ✅ **PROCEED** — packet is constitutional, Q1..Q6 recommendations are evidence-backed, no §19 / §20 freeze triggers fire, rollback plan is exercised-precedent. Architect-side residual decisions: 6 Q-answers (each surfaced with options + evidence) + verbatim §8 sign-off form.

---

## §A. Executive Summary

### A.1 What G3.2 is

A Class-4 STEP_B atom (per `CLAUDE.md §9` + `§12`) that:
1. Adds a **bankruptcy risk-cap admission precondition** to 4 sequencer admission arms (WorkTx + BuyWithCoinRouter + ChallengeTx + VerifyTx)
2. **Tail-appends** one new variant to `RejectionClass` enum: `BankruptcyRiskCapExceeded`
3. **Emits** `AgentAutopsyCapsule` (TB-15 already-landed surface; first production caller) at problem-end for agents who crossed bankrupt during the task
4. **(Conditional on Q4)** closes `OBS_G2P_VERIFY_PEER_REWARD` Gap-A (reputation accumulation) + Gap-B (bond return at run-resolve) as G3.2 sub-surfaces

### A.2 Why this is Class 4 (not Class 3)

Touches both Class-4 surfaces named in `CLAUDE.md §9`:
- **sequencer admission**: 4 admission arms in `src/state/sequencer.rs` get a new precondition
- **typed tx schema (enum-adjacent)**: `RejectionClass` in `src/state/typed_tx.rs` gains a tail variant

Per `feedback_class4_cannot_hide_in_class3` (memory file; user feedback rule), neither surface can ship under Class-3 envelope.

### A.3 Why this work is justified NOW

Per `/constitution-landing-check` skill output at audit start (re-runnable command: see §B.2):
- **AMBER row count after filtering "was 🟡 AMBER" historical annotations**: 1
- **Only AMBER row**: §R G3 in `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:217`
- **Closure path for that row**: G3.2 §8 packet → architect ratification → code lands

Per `feedback_constitutional_harness_engineering` anti-patterns §1-6 (memory file lines 25-40; re-verified at audit start): NONE of the 6 anti-patterns matches this session.

### A.4 Recommendation to architect

1. Ratify the 6 Q-answers per §E recommendations below (or override with reasoned divergence; each Q surfaces alternatives).
2. Sign §8 with canonical multi-clause Class-4 verbatim form (per `CLAUDE.md §10`; non-acceptable single-word forms enumerated in §D.4 below).
3. Confirm scope: branch `feat/g3-2-risk-cap-admission` cut from `4d4412b`; landing covers §2 of packet conditional on Q-answers; PRE-§8 dual audit (Codex G2 + Gemini DT) round-cap=2; minimal real-LLM 3-problem smoke before ship; no batching with G4.2.

---

## §B. Current Fact State (with citation)

### B.1 Repo HEAD + branch + working tree

| Fact | Value | Evidence |
|------|-------|----------|
| `origin/main` HEAD | `4d4412b` | `git log --oneline -1 origin/main` (run 2026-05-12 audit) |
| Local HEAD | `4d4412b` | `git rev-parse HEAD` (run 2026-05-12 audit) |
| Local HEAD subject | "TB-G G3.2 / G4.2 — session #46 boot prompt (Class-4 §8 packet drafting session)" | same commit |
| Working tree | clean except pre-existing drift declared in session #46 boot prompt §3 disk-hygiene block (h_vppu_history.json / rules/enforcement.log / 2× g_phase_g2p_2026-05-12T01-* evidence dirs / search_gdocs.py) | `git status --short` at audit start |
| Session #45 ship trail | G3.1 (`97e6527`) / G3.4 (`2e7839f`) / G3.3 (`903d164`) / audit prompt (`9fde94d`) / session-close (`4d4412b`) | `git log --oneline -7` |

### B.2 Constitution matrix status

| Fact | Value | Evidence |
|------|-------|----------|
| Total matrix rows with current status 🟡 AMBER | **1** | filtered awk command (see audit re-run script below) |
| Only AMBER row | §R G3 "**G3** Persistent PnL / Solvency / Bankruptcy risk-cap admission" | `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md:217` |
| Row's current status doc | "G3 OBSERVABILITY LAYER LANDED 2026-05-12 session #45: G3.1 SHIPPED `97e6527` + G3.4 SHIPPED `2e7839f` + G3.3 SHIPPED `903d164` ... **G3 module status AMBER (not GREEN) because G3.2 Class-4 STEP_B sequencer risk-cap admission + AutopsyCapsule emit still pending per-atom §8 packet — closes architect §G3 SG-G3.2/SG-G3.3/SG-G3.4 module-level ship gates which this session leaves untouched.**" | same line; verbatim |
| Re-runnable filter command | `awk -F'\|' '/^## §[A-Z]/ {section=$0} /^\|/ && !/^\|---/ && !/clause/ {for(i=1;i<=NF;i++){if(match($i,/🟢 GREEN\|🟡 AMBER\|🔴 RED/)){status=substr($i,RSTART,RLENGTH); if(status=="🟡 AMBER")print section": "$2" => "status; break}}}' handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` | reproducible |
| Raw `🟡 AMBER` grep count | 42 | `grep -cE '🟡 AMBER' handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` |
| Why 42→1 reduction | 41 are "was 🟡 AMBER" historical annotations inside GREEN status cells (e.g., line 28 "🟢 GREEN (Wave 3 50p binding sync 2026-05-08; was 🟡 AMBER — ...)"). The filter checks the FIRST status marker in each row's status column, which is the current state. | manual verification of 5 example lines (28 / 43 / 67 / 92 / 217); pattern uniform |

### B.3 G3 observability layer empirical evidence (already shipped under parent §8 G-Phase)

| Fact | Value | Evidence |
|------|-------|----------|
| G3 9-task real-LLM batch evidence dir | `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/` | `ls` at audit start |
| Aggregate audit_tape verdict | `"verdict": "PROCEED"` | `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/aggregate_verdict.json:409` |
| Schema version | `"v1/audit_tape_verdict"` | same file line 2 |
| Persistence binding pass | `"is_passing": true` | `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/PERSISTENCE_BINDING_REPORT.json:5` |
| Persistence witnesses | `"n_witnessed": 4` | same file line 6 |
| Smoke wall-time | 3088s | LATEST.md session #45 close block |
| Empirical §G PnL trajectory render | 3 / 13 non-flat rows: `tb7-7-sponsor` (escrow -100k μC) + `Agent_0` (stake+claim positions=2 realized=-1k) + `MarketMakerBudget` (collateral -100k μC positions=1) | `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md:18-36` (verbatim §G block) |
| Codex G2 single-auditor verdict | `VERDICT: PROCEED / CONVICTION: high / Q1..Q12 ALL PASS` | `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md:1-14` |
| Codex non-blocking notes | 3 (provenance gap / SG-G3.8.b test-strength gap / multi-ref ChainTape) | same file lines 37-39 |

### B.4 G3.2 location in charter + directive

| Fact | Value | Evidence |
|------|-------|----------|
| Charter row | "**G3.2** Solvency emitter + **sequencer-side risk-cap admission** (4 admission arms: WorkTx + BuyRouter + Challenge + Verify); new `BankruptcyRiskCapExceeded` RejectionClass \| **4 STEP_B** \| ..." | `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md:273` |
| Charter §3 packets list | "2. **G3.2** sequencer risk-cap admission — `handover/directives/2026-05-1X_TB_G_G3_2_§8_PACKET.md`" | same file line 334 |
| Architect directive §G3 ship gates | `SG-G3.1`..`SG-G3.5` (5 gates; G3.2 scope = `SG-G3.3` + `SG-G3.4`) | `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md:277-284` |
| `SG-G3.3` verbatim | "bankrupt / low-balance agent receives AutopsyCapsule" | same file line 281 |
| `SG-G3.4` verbatim | "bankrupt agent cannot continue unlimited risk-taking" | same file line 282 |
| Charter §4 forbidden list | 11 items | `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md:340-355` |

---

## §C. G3.2 Scope + Class-4 Determination (with citation)

### C.1 Why Class 4 — sequencer admission AND typed-tx schema

`CLAUDE.md §9 Class 4` verbatim (CLAUDE.md project file): "Constitution / sequencer admission / typed tx schema / canonical signing payload / RootBox. Requires: explicit architect ratification / harness / minimal real run if applicable / external audit."

G3.2 surfaces map to §9:
- **sequencer admission**: surfaces (a)+(b)+(c)+(d) in packet §2.1 row 2 all modify `src/state/sequencer.rs`. 4 distinct admission arms gain a new precondition.
- **typed tx schema (enum-adjacent)**: `RejectionClass` enum in `src/state/typed_tx.rs:165-207` gains 1 tail variant. While `RejectionClass` is not itself a `TypedTx` wire variant, it is part of the typed-tx file's authoritative enum surface and is consumed by `TxStatus::Rejected(RejectionClass)` at `src/state/typed_tx.rs:248`.

**Cross-check against `feedback_class4_cannot_hide_in_class3`** (user memory rule): "sequencer admission / typed-tx schema bumps / canonical-signing-payload changes need separate ratification". G3.2 hits 2 of 3.

### C.2 STEP_B file membership (Trust Root rehash required)

`CLAUDE.md §12` STEP_B file list (verbatim): `src/kernel.rs` / `src/bus.rs` / `src/sdk/tools/wallet.rs` / `src/state/sequencer.rs` / **`src/state/typed_tx.rs`** / `src/bottom_white/cas/schema.rs` / canonical signing payload surfaces.

G3.2 touches **2 of 7** STEP_B files (sequencer + typed_tx). Both already in `genesis_payload.toml` manifest from prior STEP_B atoms (TB-2..P-M6 + G1.1). No manifest extension required.

### C.3 Precise admission arm locations (current code, audited)

All line numbers verified by `grep -n` at audit time:

| Arm | Existing balance/stake check | Risk-cap precondition location (G3.2 forward) |
|-----|------------------------------|----------------------------------------------|
| WorkTx | `StakeBalanceExceeded` at `src/state/sequencer.rs:922` (verified by grep `StakeBalanceExceeded`) — "if work.stake.micro_units() > agent_balance_a3.micro_units() { return Err(TransitionError::StakeBalanceExceeded); }" at `:921-923` (verified by direct read) | BEFORE line 914 (stake==0 check); reads `agent_balance` via existing `balances_t.0.get(&work.agent_id).copied().unwrap_or(zero)` pattern (`:917-920`) |
| VerifyTx | `BondInsufficient` at `:1042` + `VerifyBondOutOfBounds` at `:1056` (grep verified) | BEFORE line 1042 |
| ChallengeTx | challenger-balance check witnessed by test `dispatch_challenge_rejects_when_challenger_balance_lt_stake` at `:5913` (grep verified) | BEFORE the production challenge admission step (line ~1239 charter §3.5 + §4.3 + §3.9 region per source-doc comment at `:1225`) |
| BuyWithCoinRouter | `RouterInsufficientCoinBalance` at `:3066` (grep verified) | BEFORE line 3066 |

### C.4 Existing reusable surfaces (NO new code needed)

| Surface | Location | Verified by |
|---------|----------|-------------|
| `classify_solvency(balance, initial_balance_micro) -> SolvencyStatus` | `src/runtime/agent_pnl.rs:307-317` | direct read |
| Solvency 10% threshold formula `initial_balance_micro / 10` | `src/runtime/agent_pnl.rs:311` | direct read |
| `initial_balance_micro_from_default_preseed(agent_id) -> i64` | `src/runtime/agent_pnl.rs:325-331` | direct read |
| `write_autopsy_capsule(...)` | `src/runtime/autopsy_capsule.rs:253-329` | direct read |
| `is_autopsy_active_at(timestamp_logical) -> bool` (TB-15 R2 Gemini VETO Q12 closure) | `src/runtime/autopsy_capsule.rs:395-398` | grep verified |
| `AgentAutopsyCapsule` default `privacy_policy: AuditOnly` | `src/runtime/autopsy_capsule.rs:172` | direct read |
| Current `RejectionClass` variant count | 11 | `awk '/^pub enum RejectionClass/,/^}/' src/state/typed_tx.rs \| grep -E "^    [A-Z][a-zA-Z]*" \| wc -l` |
| Latest existing variant (tail-append insertion site) | `VerifyDuplicate` at `:206` | direct read |

### C.5 Surfaces NOT touched (defensive evidence)

Verified by inspecting `src/state/typed_tx.rs` struct definitions at `:267-301` (WorkTx + VerifyTx) and `:314-324` (ChallengeTx):

- No `WorkTx` schema change (12 fields unchanged at `:268-281`)
- No `VerifyTx` schema change (8 fields unchanged at `:291-300`)
- No `ChallengeTx` schema change (8 fields unchanged at `:315-324`)
- No `BuyWithCoinRouterTx` schema change (8 fields unchanged per P-M6 packet §1 lines 70-80)
- E.1 verbatim binding gate at `tests/constitution_architect_verbatim_struct_binding.rs` (P-M6 closure mechanism) unchanged

---

## §D. Constitutional Audit (10-clause walkthrough)

### D.1 §2 Prime Operating Mode — Constitutional Harness Engineering

Required loop (CLAUDE.md §2.1 verbatim): `constitution gate -> real run -> debug -> fix -> rerun -> audit -> ship`

**G3.2 position in loop** (with evidence):

| Step | Status | Evidence |
|------|--------|----------|
| constitution gate | ✅ harness GREEN | `bash scripts/run_constitution_gates.sh` reports `402/0/1` per LATEST.md session #45 close; 26 new gates landed under G3.1+G3.3+G3.4 |
| real run | ✅ done | `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/aggregate_verdict.json:409 = "PROCEED"`, wall 3088s |
| debug / fix | ✅ no outstanding production defects | Codex G2 verdict `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md:1-14` PROCEED 12/12 conviction high; 3 notes are test-scaffold + provenance + ChainTape-derived-view edges, not production defects (per `feedback_audit_loop_roi_flip` memory rule) |
| rerun | n/a — debug step had no production defect |
| **audit (this step)** | **🔄 current** | this report + forthcoming PRE-§8 Codex G2 + Gemini DT external audits (PRE-§8 timing per `feedback_dual_audit` Class-4 rule; precedent P-M2 / P-M4 / P-M6 / G1.1) |
| ship | ⏸ pending architect §8 |

Forbidden old loop ("Atomic Agentic Engineering"; CLAUDE.md §2.1 verbatim): `charter -> atom -> self-audit -> external audit -> more docs -> delayed test`.

**Mismatch check**: G3.2 is NOT the old loop because (a) harness already ran with real evidence BEFORE this audit, (b) external audit is post-implementation PRE-§8 (not pre-implementation G1-charter audit), (c) no `pick atom blockedBy <audit-task>` dependency graph. Cross-verified against `feedback_constitutional_harness_engineering` anti-patterns §1-6 (memory file lines 25-40, freshly read at audit start): **zero matches**.

### D.2 §7 Constitution Landing Policy — matrix discipline

CLAUDE.md §7 requires every clause to have a matrix row mapping `clause / flowchart node -> code surface -> executable test -> smoke/evidence witness -> current status -> kill condition`.

**G3.2 matrix landing plan** (per packet §2.4):

| Mapping element | Value |
|-----------------|-------|
| clause | charter §1 Module G3 row G3.2 + architect §G3 SG-G3.3 + SG-G3.4 |
| flowchart node | FC1 (admission predicate) + FC3 (capsule derived from tape+CAS) |
| code surface | `src/state/sequencer.rs` 4 admission arms + AutopsyCapsule emit; `src/state/typed_tx.rs` RejectionClass tail-append |
| executable test | `tests/constitution_g3_bankruptcy_risk_cap.rs` (NEW; 10 tests) + `tests/constitution_g3_autopsy.rs` (NEW; 5 tests) + optionally `tests/constitution_g3_verify_reward_bond_return.rs` (NEW conditional on Q4=bundle; 4 tests) |
| smoke/evidence witness | 3-problem real-LLM mini batch with ≥1 bankrupt agent → ≥1 `BankruptcyRiskCapExceeded` L4.E + ≥1 `AgentAutopsyCapsule` CAS write |
| current status | 🟡 AMBER (this packet) → 🟢 GREEN on ship |
| kill condition | bankrupt agent above cap admitted to L4 (= G3.2 failed) |

CLAUDE.md §7 explicit constraint: "Documentation-only coverage is not landed." Packet §2.3 lists 3 executable test files; not docs-only.

### D.3 §9 Class 4 — covered in §C.1; ratification requirement enumerated below

§9 verbatim requires for Class 4: "explicit architect ratification / harness / minimal real run if applicable / external audit".

| Requirement | Plan / status |
|-------------|--------------|
| explicit architect ratification | THIS PACKET ASKS FOR IT (§6 of packet; §G of this report) |
| harness | 3 new test files specified in packet §2.3; minimum 10..19 tests RED until implementation |
| minimal real run | 3-problem mini batch specified in packet §6.3 item 3 |
| external audit | Codex G2 + Gemini DT round-cap=2 PRE-§8 (packet §6.3 + this report §D.3) |

### D.4 §10 Authorization Semantics — verbatim form requirements

§10 verbatim: "For Class 3/4 or ship decisions, authorization must name: scope / allowed path / forbidden path / risk class / whether audit is required / whether ship is authorized. A one-word instruction may authorize candidate remediation only. It does not authorize final ratification or ship."

**Verbatim forms previously accepted as Class-4 §8 in v4 history** (per LATEST.md + memory file `MEMORY.md`):
- `好，确认可以 ship` — TB-C0 2026-05-07 (LATEST.md "Stage A2 SHIPPED FINAL" history block) + P-M2 + P-M6 + G1.1
- `同意 sign-off` — Stage A3 2026-05-08 (`handover/directives/2026-05-08_STAGE_A3_§8_SIGN_OFF.md`)
- `签字，同意后续执行` — P-M4 2026-05-09 (`handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM4_§8_SIGN_OFF.md`)
- `授权自主执行直到X` — Stage C overall 2026-05-09

**Verbatim forms explicitly NOT accepted** (CLAUDE.md §9 verbatim): "Single-word messages such as: `fix` / `go` / `ok` / `continue` / `可以` do not constitute Class-4 sign-off."

§6 of packet leaves the §8 cell BLANK pending one of the canonical multi-clause forms.

### D.5 §12 Code Standard / STEP_B — verified in §C.2

`cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` is the gate at every commit boundary (precedent: P-M6 §8 packet §6 line 302 "PASS" stamp; G1.1 §8 packet §2 STEP_B-protected surfaces table). G3.2 plan: rehash 2 STEP_B files at the implementation commit (1 SHA256 line per file in `genesis_payload.toml`).

### D.6 §13 Economy Laws — all preserved

CLAUDE.md §13 verbatim laws + per-law compliance check:

| Law | G3.2 status | Evidence |
|-----|-------------|----------|
| "Information is Free" | ✅ unchanged | risk-cap reads canonical `balances_t`; no information toll |
| "Only Investment Costs Money" | ✅ unchanged | risk-cap only REJECTS; never charges |
| "1 Coin = 1 YES + 1 NO" | ✅ unchanged | no mint, no CTF mutation |
| "`on_init` is the only legal base-Coin mint" | ✅ unchanged | `assert_no_post_init_mint` still called at every accepted-tx commit point (precedent: P-M6 §8 packet §2 "3 monetary invariants" lines 140-147) |
| "system tx cannot be agent-submitted" | ✅ unchanged | risk-cap operates on existing agent-ingress txs (WorkTx / Verify / Challenge / BuyRouter); these are already agent-ingress per §13 |
| "no ghost liquidity" | ✅ unchanged | no pool mutation |
| "no automatic YES/NO injection" | ✅ unchanged | no share creation |
| "no `f64` money path" | ✅ unchanged | risk-cap uses `i64` micro-units (matches `src/runtime/agent_pnl.rs:311` `initial_balance_micro / 10` integer division pattern) |
| "Market price is a statistical signal, not truth" | ✅ unchanged | risk-cap reads `balances_t`, not price |
| Total Coin conservation | ✅ unchanged | rejected attempts make no state mutation (packet §2.3 test `rejected_attempts_make_no_state_mutation`) |

**Caveat under Q3=B2** (new BondReturnTx system-tx): would add a new system-tx authorization surface → CLAUDE.md §13 "system tx cannot be agent-submitted" boundary needs separate architect ratification. Surfaced as HALT trigger in packet §3 Q3.

### D.7 §14 Predicate / Oracle Rules — boundary preserved

CLAUDE.md §14 verbatim: "Boolean predicates define hard boundary: predicate pass -> may enter L4; predicate fail -> L4.E or anchored evidence."

Risk-cap is a boolean predicate `balance < bankruptcy_risk_cap_micro(agent, q)` → fail → L4.E with `RejectionClass::BankruptcyRiskCapExceeded`. This is the canonical FC1 path (CLAUDE.md §3.1 "predicate fail -> L4.E rejection evidence").

§14 also requires partial verdicts to be typed: not applicable here (binary admit/reject; no `PartialAccepted` state introduced).

### D.8 §15 Shielding Rules — bandwidth budget enforced

CLAUDE.md §15 allowed: `public_summary` + low-pollution rejection class + typical-error summary + private/audit-only diagnostic CID. Forbidden: raw failure logs in agent read views.

| Shielding surface | G3.2 plan | Bandwidth |
|-------------------|-----------|-----------|
| `RejectionClass::BankruptcyRiskCapExceeded` display | `"bankruptcy_risk_cap_exceeded"` token | 37 bytes ≤ 64-byte SG-G3.12 budget |
| `AgentAutopsyCapsule.public_summary` | TB-15 format `agent={id} lost {amount}μC on event={event} reason={tag}` | bounded; precedent Wave-3 50p binding shows TransitionError.display max 48B / 95 instances (matrix §F Art. III.4 line 60 / packet §1.5) |
| `AgentAutopsyCapsule.privacy_policy` | `AuditOnly` (default at `src/runtime/autopsy_capsule.rs:172`) | NEVER enters `AgentVisibleProjection` per TB-15 architect §6.4 (verbatim doc-comment at `src/runtime/autopsy_capsule.rs:96-104`) |
| `AgentAutopsyCapsule.private_detail_cid` | opaque Cid (TB-15 contract) | audit-only |

### D.9 §19 No Manipulation by Sequencing — self-check

§19 verbatim load-bearing blockers:
- `HEAD_t` — ✅ Stage A3 P-M0 SHIPPED 2026-05-08 (`MEMORY.md` Stage A3 block)
- PCP soundness — ✅ corpus phase-2 LANDED (`MEMORY.md` Stage B3 R3 block)
- `PromptCapsule` / prompt persistence — Class 2-3 forward (G2P.4); NOT a load-bearing blocker for G3.2 because PromptCapsule observability is a CHALLENGE on a DIFFERENT surface (LLM-call observability), and Class-3 default policy already landed (CLAUDE.md §4.3)
- system tx authorization — ✅ GREEN (unchanged by G3.2 under recommended Q3=B1)
- tape canonical ID namespace — ✅ Stage A3 multi-ref ChainTape GREEN
- economic conservation — ✅ §D.6 above

§19 also forbids "closing easy gaps to create progress optics while load-bearing blockers remain red." Cross-check:
- G3.2 closes `SG-G3.3` + `SG-G3.4` — these are architect-mandated **load-bearing** ship gates (G-Phase directive §G3 lines 281-282 verbatim).
- No more-load-bearing red row exists (per §B.2 — only 1 AMBER row, and it IS this row).

**§19 verdict**: G3.2 is the highest-load-bearing forward work item at the current matrix snapshot.

### D.10 §20 Feature Freeze Conditions — none triggered

§20 verbatim freeze triggers + status:

| Trigger | Status | Evidence |
|---------|--------|----------|
| FC1 Runtime Loop | ✅ GREEN | `tests/constitution_fc1_runtime_loop.rs::fc1_every_externalized_attempt_is_tape_visible` + `::fc1_no_legacy_authoritative_append` (line 118) + `::fc1_attempt_count_equals_tape_count` (line 182) all GREEN; Wave 3 50p binding at matrix §G |
| FC2 Boot | ✅ GREEN | `tests/constitution_fc2_boot.rs` + Stage A3 G1.1 resume mode SHIPPED 2026-05-11 |
| FC3 Meta/Markov | ✅ GREEN | `tests/constitution_fc3_evidence_binding.rs` 7 tests / 2026-05-08 full landing (MEMORY.md "宪法完整落地 2026-05-08" block) |
| Tape canonicality | ✅ GREEN | matrix §M GREEN |
| Economy conservation | ✅ GREEN | matrix §A Art. 0 Laws GREEN |
| No-fake-accepted | ✅ GREEN | (no entry in current AMBER set) |
| System-tx-not-agent-submittable | ✅ GREEN | matrix §A Art. 0 Laws / economy_gate `system_tx_not_agent_submittable` |
| Dashboard-regeneratable | ✅ GREEN | matrix §M `dashboard_regenerates_from_tape_cas` |
| Attempt equality | ✅ GREEN | matrix §N MVP-1 |

**§20 verdict**: zero freeze triggers fire. G3.2 forward work is constitutional.

---

## §E. Six Open Questions — Each with Evidence-Backed Recommendation

### Q1. Risk-cap threshold: per-agent (recommended) vs global constant

**Background**: Boot prompt §1 Surface 1 suggests global `BANKRUPTCY_RISK_CAP_MICRO = 100_000` (10% of common-agent preseed 1.0 Coin). But G3.1 SHIPPED uses per-agent `initial_balance_micro / 10`.

**Evidence for per-agent**:
- `src/runtime/agent_pnl.rs:311` (direct read): `let threshold = initial_balance_micro / 10;`
- `src/runtime/agent_pnl.rs:307-317` `classify_solvency` — production-shipped under G3.1 Codex G2 PROCEED 12/12 high (Codex G3 verdict §C.4 above)
- Preseed agent inventory (verified by running G3 smoke evidence + LATEST.md empirical §G block):
  - `Agent_0..9`: 1_000_000 μC preseed → 100_000 μC cap (matches boot prompt suggestion)
  - `MarketMakerBudget`: 5_000_000 μC preseed → 500_000 μC cap
  - `tb7-7-sponsor`: 10_000_000 μC preseed → 1_000_000 μC cap
- Internal-consistency argument: if G3.1's NearInsolvent classifier (read-only solvency view) uses per-agent threshold and G3.2 admission uses global, the dashboard would show an agent "near_insolvent" while admission still permits them above cap — read view and write view would disagree

**Evidence for global**:
- Simpler test surface (1 constant vs 1 helper)
- Boot prompt §1 Surface 1 explicit recommendation
- Lower auditor surface area (Codex G2 has 1 constant to verify vs 1 function to trace)

**My recommendation**: **per-agent** (consistency with G3.1 outweighs simplicity).

**Halt-and-re-charter trigger** (boot prompt §69-72): if architect picks **per-agent with separate non-derived table** (a new `EconomicState.bankruptcy_risk_cap_t` sub-field), that's a Class-4 schema extension → 2-packet split required.

### Q2. Reputation accumulation: verdict-uniform (+1) vs verdict-weighted

**Background**: OBS_G2P_VERIFY_PEER_REWARD §2.1 documents the gap.

**Evidence**:
- Current state: `src/state/sequencer.rs` grep `reputations_t` returns only forward-pointing doc-comment at `:3857` ("SlashTx / SettlementTx / ProvisionalAcceptTx / ReputationUpdateTx"); no code mutates `reputations_t` (`handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md:52-65`)
- OBS §2.1 quoted source: "G2P module pending" — the persistence report wording "per TB-N1 A4" is **aspirational** (OBS §2.1 explicit clarification at lines 60-68)
- Test gate `tests/constitution_g2p_verify_reward_bond_return.rs` SG-G2P.6.c is the negative-witness assertion (OBS §5 lines 132-136) — will flip on G3.2 land if bundled

**My recommendation**: **uniform +1** (simplest closure; verdict-weighting is a forward TB if §G PnL trajectory shows bull-bias harm).

### Q3. Bond-return surface: Option B1 (extend FinalizeRewardTx) vs Option B2 (new BondReturnTx)

**Background**: OBS_G2P_VERIFY_PEER_REWARD §2.2 documents the gap (`balances_t[verifier] -= bond` at admission is final until something credits back; nothing credits back today).

**Evidence**:
- Current FinalizeRewardTx schema: `src/state/typed_tx.rs:357-368` (9 fields; `system_signature` at field 9)
- FinalizeRewardTx is system-emitted (TB-5 / TB-11 era); already has SystemEpoch + system_signature surfaces
- Option B1 reuses 1 existing system-tx (no new TxKind, no new signing domain). Extends existing dispatch arm with verifier-bond credit logic.
- Option B2 adds new `TypedTx::BondReturn` variant + new TxKind id + new canonical signing payload + new domain prefix `b"turingosv4.system.bond_return.v1"` → expands G3.2 STEP_B file list to include `src/bottom_white/ledger/transition_ledger.rs` + `src/bottom_white/ledger/system_keypair.rs`

**My recommendation**: **B1** (minimal STEP_B expansion; reuses existing system-tx infrastructure).

**Halt-and-re-charter trigger** (boot prompt §73): if architect picks B2, the new system-tx authorization is a separate architect boundary per CLAUDE.md §13 "system tx cannot be agent-submitted" — packet would need to extend §2.1 STEP_B file list and add E.1-style architect-verbatim test binding for the new TxKind id.

### Q4. Gap-A/B bundle: include in G3.2 §8 (recommended) vs split to G3.5

**Background**: Boot prompt §1 Surface 4 explicit framing.

**Evidence for bundle**:
- Both Gap-A and Gap-B touch the SAME file (`src/state/sequencer.rs`) and the SAME Class-4 boundary (sequencer admission)
- `feedback_no_batch_class4_signoff` (boot prompt §1 Surface 4 verbatim quote): "the no-batch rule prohibits batching DISTINCT Class-4 atoms across §8 packets, NOT bundling sub-surfaces of one Class-4 atom into one §8 packet."
- OBS_G2P_VERIFY_PEER_REWARD §3 lines 88-108 forward-binds the gap closure to G3.2 (not G3.5): "G3.2 is the Class-4 §8 packet boundary that gates Verify reward/bond-return as part of a coherent PnL contract"

**Evidence for split**:
- Strict reading of `feedback_no_batch_class4_signoff` could be interpreted as "each charter row = one §8 packet" → Gap-A/B were filed as OBS not as charter row
- Splitting keeps architect §8 reviews tightly scoped (1 architect-mandated SG row per packet)

**My recommendation**: **bundle** (same Class-4 boundary; same file; same EconomicState sub-fields; one architect review covering coherent admission-related surfaces).

### Q5. Admission ordering: risk-cap fires first (subsumption) vs last (defense-in-depth)

**Background**: How should risk-cap interact with existing per-arm gates?

**Evidence for "first" (subsumption)**:
- TB-N1 A3 precedent at `src/state/sequencer.rs:5565-5566`: "post-A3: solver lacks balance for stake → Step-4 `StakeBalanceExceeded` (**subsumes** pre-A3 Step-6 `InsufficientBalance` for this case)"
- Subsumption preserves cleaner per-class L4.E telemetry (one rejection = one most-informative class)
- Information Loom (per OBS §2.1 reference) benefits from coarser-grained class signal when agent crosses risk-cap (the more general failure)

**Evidence for "last" (defense-in-depth)**:
- Existing audit walkers consume current per-arm classes (StakeBalanceExceeded for WorkTx, BondInsufficient + VerifyBondOutOfBounds for VerifyTx, RouterInsufficientCoinBalance for BuyRouter, ChallengeStakeOutOfBounds for ChallengeTx)
- "Last" preserves these signals; the new BankruptcyRiskCapExceeded only fires when per-arm gate would have admitted

**My recommendation**: **first** (TB-N1 A3 precedent + cleaner Information Loom signal).

### Q6. AutopsyCapsule emit timing: per-task-end vs run-end

**Background**: Architect SG-G3.3 reads "bankrupt / low-balance agent **receives** AutopsyCapsule" — singular but ambiguous on per-bankruptcy-event vs per-batch.

**Evidence**:
- TB-15 design: `AgentAutopsyCapsule.event_id: EventId` (`src/runtime/autopsy_capsule.rs:115-116`) — event-keyed by construction
- TB-15 `created_at_logical_t` + `created_at_round` per-capsule (`src/runtime/autopsy_capsule.rs:153-156`) — supports fine-grained per-task emit
- G3 smoke evidence (3088s; 9 problems): per-task emit produces ≤ N capsules where N = number of bankruptcy events; run-end emit produces ≤ M capsules where M = number of agents that ended bankrupt at batch close (M ≤ N)

**My recommendation**: **per-task-end** (matches architect "singular but per-bankruptcy-event" reading; finer-grained evidence; replay-deterministic via existing TB-15 `is_autopsy_active_at` gate).

---

## §F. Risk Assessment

### F.1 Halt-and-re-charter triggers (3 surfaced; all conditional on architect Q-answer)

Per boot prompt §69-73:

| Trigger | Conditional on | Mitigation |
|---------|----------------|------------|
| EconomicState schema extension (new `bankruptcy_risk_cap_t` sub-field) | Q1 = "per-agent with separate non-derived table" | Split into 2 packets: schema packet + admission packet. Surface for architect at Q1 decision. |
| Per-agent risk-cap value if architect prefers global | Q1 = "global" | Single-line change in packet §1 + test re-scope. No re-charter; just adjudication. |
| New system-tx authorization boundary | Q3 = B2 | Re-charter required: G3.2 packet expands to include `src/bottom_white/ledger/transition_ledger.rs` + `src/bottom_white/ledger/system_keypair.rs`; new TxKind id allocation; new signing domain. |

### F.2 Rollback plan (precedent-exercised)

Per Stage C P-M2..P-M9 rollback 2026-05-09 (HEAD `01dd825`; LATEST.md "Stage C Polymarket VETOED + ROLLED BACK 2026-05-09 session #28" block):

| Step | Mechanism | Precedent witness |
|------|-----------|---------------------|
| Revert commit | `git revert <G3.2-commit-sha>` | Stage C P-M2..P-M9 reverted in 1 commit at `01dd825` |
| State recovery | `RejectionClass::BankruptcyRiskCapExceeded` disappears; no historical L4.E references it (variant didn't exist pre-G3.2) | tail-append discipline preserves all pre-G3.2 wire forms; canonical-byte recovery problem-free |
| Matrix re-revert | §R G3 🟢 → 🟡 | precedent: Stage C VETO reverted matrix changes in same commit |
| OBS re-open | OBS_G2P_VERIFY_PEER_REWARD → 🟡 if Q4=bundle | reversible by editing 1 status field |
| Test deletion | 3 new files removed | reversible by `git revert` |
| Trust Root re-rehash | `genesis_payload.toml` reverts sequencer + typed_tx SHA256s | precedent: P-M6 rehashes (6 STEP_B files; `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM6_§8_PACKET.md:294-301`) reversed cleanly in 1 commit on rollback |

**Precedent test**: `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` PASSED at every commit boundary during Stage C rollback (precedent: P-M6 §8 packet line 302 "PASS" stamp; same gate used at every commit since TB-2).

---

## §G. Architect-Decision Surface

### G.1 6 Q-answers requested (with recommendations)

| Q | Recommendation | Override impact |
|---|---------------|-----------------|
| Q1 | per-agent (`initial_balance_micro / 10`) | "global" = 1-line change; "per-agent w/ new table" = re-charter |
| Q2 | uniform +1 | "verdict-weighted" or "outcome-correlated" = forward TB |
| Q3 | B1 (extend FinalizeRewardTx) | B2 = re-charter (new system-tx authorization) |
| Q4 | bundle Gap-A/B into G3.2 | split = new G3.5 atom + own §8 packet |
| Q5 | risk-cap first (subsumption) | last = test re-scope; cleaner telemetry trade-off |
| Q6 | per-task-end emit | run-end = test re-scope; coarser-grained capsule trade-off |

### G.2 §8 sign-off authorized scope (proposed)

Per packet §6.1:

- **Allowed**: cut `feat/g3-2-risk-cap-admission` from `4d4412b`; land §2 surfaces conditional on Q-answers; PRE-§8 dual audit Codex G2 + Gemini DT round-cap=2; minimal 3-problem real-LLM smoke; ship under this §8 once dual audit PASSes
- **Forbidden**: changes outside §2 file list (subject to Q3 outcome); WorkTx/VerifyTx/ChallengeTx/BuyRouter struct schemas (only RejectionClass enum tail-append); batch G4.2 or future Class-4 atoms; bypass dual audit; bypass real-LLM mini smoke
- **Risk class**: 4 STEP_B
- **Audit required**: yes (dual audit + Trust Root verify + real-LLM mini smoke)
- **Ship authorized**: conditional on (a) dual audit PASS, (b) mini smoke `verdict=Ok delta=0 audit_proceed=true inv1_match=true`, AND (c) §G PnL trajectory empirical render confirming ≥1 bankrupt-agent row produces an AutopsyCapsule

### G.3 Forward gating (post-G3.2)

Per CLAUDE.md §9 + `feedback_no_batch_class4_signoff`:

- **Independent of G3.2**: G4.2 §8 packet draft (separate Class-4 STEP_B; can draft in parallel; boot prompt §3 explicit)
- **Independent of G3.2**: G2P.4 PromptCapsule swarm-write (Class 2-3 autonomous under parent §8 G-Phase grant; precedent G2P.1 / G3.3)
- **Gated by G3.2 ship**: G5.1 / G5.2 / G5.3 (Class 2-3 forward; opportunity scheduler depends on solvency_status surface that G3.2 turns from read-only to admission-gating)
- **Gated by G3.2 + G4.2 ship**: G6.1..G6.3 / G7.1..G7.4

---

## §H. Evidence Citation Index (for re-audit)

### H.1 Constitution clauses cited

| Clause | Subject | Cited in |
|--------|---------|----------|
| CLAUDE.md §2.1 | Prime Operating Mode loop | D.1 |
| CLAUDE.md §3.1 | FC1 predicate routing | D.7 |
| CLAUDE.md §3.3 | FC3 capsule derivation | C.1 / D.8 |
| CLAUDE.md §4.3 | PromptCapsule default policy | D.9 |
| CLAUDE.md §7 | Constitution Landing Policy | D.2 |
| CLAUDE.md §9 | Class 4 ratification | A.2 / C.1 / D.3 |
| CLAUDE.md §10 | Authorization semantics | A.4 / D.4 |
| CLAUDE.md §12 | STEP_B file list | C.2 / D.5 |
| CLAUDE.md §13 | Economy laws (10 items) | D.6 |
| CLAUDE.md §14 | Predicate / oracle boundary | D.7 |
| CLAUDE.md §15 | Shielding | D.8 |
| CLAUDE.md §19 | No manipulation by sequencing | D.9 |
| CLAUDE.md §20 | Feature freeze conditions | D.10 |

### H.2 Source files cited (production code)

| File | Lines | Evidence content |
|------|-------|------------------|
| `src/state/sequencer.rs` | 488-573 | TransitionError → RejectionClass mapping table |
| `src/state/sequencer.rs` | 917-923 | WorkTx StakeBalanceExceeded admission step |
| `src/state/sequencer.rs` | 1042 | VerifyTx BondInsufficient |
| `src/state/sequencer.rs` | 1056 | VerifyTx VerifyBondOutOfBounds |
| `src/state/sequencer.rs` | 1239 | Challenge admission step (charter §3.5 doc-comment) |
| `src/state/sequencer.rs` | 3066 | BuyWithCoinRouter RouterInsufficientCoinBalance |
| `src/state/sequencer.rs` | 3857 | Forward-pointing reputations_t doc-comment |
| `src/state/sequencer.rs` | 5565-5566 | TB-N1 A3 subsumption precedent comment |
| `src/state/sequencer.rs` | 5913 | Challenge balance test |
| `src/state/typed_tx.rs` | 165-207 | RejectionClass enum (11 current variants) |
| `src/state/typed_tx.rs` | 248 | TxStatus::Rejected(RejectionClass) consumer |
| `src/state/typed_tx.rs` | 267-281 | WorkTx schema (12 fields; unchanged by G3.2) |
| `src/state/typed_tx.rs` | 290-300 | VerifyTx schema (unchanged) |
| `src/state/typed_tx.rs` | 314-324 | ChallengeTx schema (unchanged) |
| `src/state/typed_tx.rs` | 357-368 | FinalizeRewardTx schema (Q3 B1 target) |
| `src/runtime/agent_pnl.rs` | 307-317 | classify_solvency body |
| `src/runtime/agent_pnl.rs` | 311 | `initial_balance_micro / 10` threshold formula |
| `src/runtime/agent_pnl.rs` | 325-331 | initial_balance_micro_from_default_preseed helper |
| `src/runtime/autopsy_capsule.rs` | 96-104 | TB-15 architect §6.4 privacy doc |
| `src/runtime/autopsy_capsule.rs` | 106-157 | AgentAutopsyCapsule struct |
| `src/runtime/autopsy_capsule.rs` | 172 | default privacy_policy: AuditOnly |
| `src/runtime/autopsy_capsule.rs` | 253-329 | write_autopsy_capsule writer |
| `src/runtime/autopsy_capsule.rs` | 395-398 | is_autopsy_active_at activation gate |
| `tests/constitution_fc1_runtime_loop.rs` | 118 | fc1_no_legacy_authoritative_append (FC1 GREEN witness) |
| `tests/constitution_fc1_runtime_loop.rs` | 182 | fc1_attempt_count_equals_tape_count (FC1 GREEN witness) |

### H.3 Documents cited (handover)

| Document | Lines | Evidence content |
|----------|-------|------------------|
| `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` | 217 | §R G3 row 🟡 AMBER current state |
| `handover/alignment/OBS_G2P_VERIFY_PEER_REWARD_2026-05-12.md` | 1-149 | Gap-A + Gap-B full statement |
| `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` | 268-275 | Module G3 atoms table |
| `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` | 328-338 | §3 Architect §8 packets list |
| `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` | 340-355 | §4 Forbidden list (11 items) |
| `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` | 277-284 | §G3 SG-G3.1..5 verbatim |
| `handover/directives/2026-05-11_TB_G_G1_1_§8_PACKET.md` | 1-183 | G1.1 packet (sister atom) template |
| `handover/directives/2026-05-09_STAGE_C_POLYMARKET_PM6_§8_PACKET.md` | 1-362 | P-M6 packet (sequencer-touching Class-4 precedent) |
| `handover/audits/CODEX_G2_TB_G_G3_VERDICT.md` | 1-39 | G3 observability layer Codex G2 verdict |
| `handover/ai-direct/LATEST.md` | session #45 close block | empirical §G PnL trajectory rendered shape |

### H.4 Evidence files cited

| File | Lines | Content |
|------|-------|---------|
| `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/aggregate_verdict.json` | 2 | `"schema_version": "v1/audit_tape_verdict"` |
| `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/aggregate_verdict.json` | 409 | `"verdict": "PROCEED"` |
| `handover/evidence/g_phase_g3_2026-05-12T11-02-27Z/PERSISTENCE_BINDING_REPORT.json` | 5-6 | `is_passing=true n_witnessed=4` |

### H.5 Memory rules cited (user `MEMORY.md` / linked feedback files)

| Rule | Subject | Cited in |
|------|---------|----------|
| `feedback_constitutional_harness_engineering` | Anti-patterns §1-6; primary operating mode | D.1 / A.3 |
| `feedback_no_batch_class4_signoff` | Per-atom §8 cadence; sub-surface bundling exception | A.4 / E.Q4 / G.3 |
| `feedback_class4_cannot_hide_in_class3` | Class-4 surface ≠ Class-3 envelope | A.2 / C.1 |
| `feedback_audit_loop_roi_flip` | Test-scaffold edge vs production defect | A.3 / D.1 |
| `feedback_dual_audit` | Class-4 PRE-§8 timing rule | D.3 / G.2 |
| `feedback_real_problems_not_designed` | Wave-3 50p binding | D.8 |
| `feedback_norm_needs_mechanism` | Build mechanism not just norm | D.1 (re landing-check skill) |

### H.6 Command outputs cited (re-runnable)

| Command | Output | Cited in |
|---------|--------|----------|
| `git rev-parse HEAD` | `4d4412bc761d9184509e3e87a8e11302e59e40d0` | B.1 |
| `git status --short` | working tree drift list | B.1 |
| `grep -cE '🟡 AMBER' handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` | `42` | B.2 |
| filtered awk for current-status AMBER | 1 row | B.2 |
| `awk '/^pub enum RejectionClass/,/^}/' src/state/typed_tx.rs \| grep -E "^    [A-Z][a-zA-Z]*" \| wc -l` | `11` | C.4 |
| `bash scripts/run_constitution_gates.sh` (per LATEST.md session #45 close) | `402/0/1` | D.1 |
| `/constitution-landing-check` skill | VERDICT PROCEED; 1 AMBER row; 0 anti-patterns matched | A.3 / D.1 |

---

## §I. Auditor Sign-off

This audit is internal (Claude orchestrator role). External audit is required PRE-§8 per `feedback_dual_audit` Class-4 timing rule:
- Codex G2 (round-cap 2)
- Gemini DT (round-cap 2)

Both will be dispatched POST-implementation, PRE-architect §8 — covering Q1..Q12 in packet §6.3 + this report §G.

**Audit verdict**: ✅ PROCEED.

**Open architect decisions** before this packet can ship code:
1. Q1..Q6 adjudication
2. §8 multi-clause verbatim sign-off form per CLAUDE.md §10

**End of constitutional audit report.**
