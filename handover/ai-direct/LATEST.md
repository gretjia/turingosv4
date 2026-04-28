# TuringOS v4 — Handover State

> 📍 **PROJECT DECISION MAP** (read this first if cold-starting): `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md`
> Tracks every decision + every skipped option + every atom status + forward roadmap.
> Anti-forget pledge: no skipped option is silently retired without explicit fate logged.

---

## 📜 v2 Whitepaper — Tactical Constitutional-Level Alignment (2026-04-27, RATIFIED ✅)

**Status**: **RATIFIED** after 3-round dual external audit converged (R1 VETO → R2 CHALLENGE → R3 PASS). Constitution.md unchanged; v2 acts as supreme校准 mirror over all derivative docs (Plan v3.2 / Blueprint / v1 / Deepthink).

**Subject (v2.2 in-place)**: `handover/whitepapers/TURINGOS_v4_WHITEPAPER_v2_2026-04-27_ANTI_OREO_RESTORATION.md` (filename unchanged; content patched to v2.2 via 7 must-fix + 1 single-line fix)
**Alignment note**: `handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md` (with new § 9 sunset clause + § 10 conflict-resolution)

**Core ruling**: TuringOS = **反奥利奥架构 (body) + ChainTape (tape implementation)**. Blockchain is NOT the body; ChainTape is one possible implementation of the verifiable state-ledger tape, living within Anti-Oreo's three-layer structure (top-white predicates / middle-black agents / bottom-white tools).

**ChainTape Directive**: 项目全面向区块链前进 = ChainTape vertical (**Trust Anchor Layer 0 + ChainTape Layers 1–6**) becomes primary engineering thrust for Wave 6+. NOT "blockchain becomes body" (would invalidate v2 § 公理 5).

### Dual-audit history (3 rounds, conservative-wins)
| Round | Codex | Gemini | Conservative | Outcome |
|---|---|---|---|---|
| R1 | VETO (Q3 sudo scope drift; 7 must-fix) | CHALLENGE (Q10 governance debt) | **VETO** | v2.1 patch in same session |
| R2 | CHALLENGE (1/7 PARTIAL: stale "Layers 0–5") | PASS | **CHALLENGE** | v2.2 single-line patch |
| R3 | **PASS** (R2-NEW-1 CLOSED) | **PASS** (Q10 mitigated) | **PASS** ✅ | RATIFICATION HOLDS |

Total v2 audit cost: ~$20 (R1 $8.50 + R2 $8.50 + R3 $3.50). Cumulative project ~$100–150 / $890 mid-budget (~11–17%).

### Wave 6 priorities re-ordered under ChainTape lens
1. **CO1.7 transition_ledger** (Layer 4) — promoted: central artifact connecting agents → state
2. **CO1.1.4-pre1.b fixture corpus** — STEP_B byte-comparison engineering pre-req
3. **INV8 spec v2 revision** — close 4 VETO + 5 CHALLENGE; now scoped under Layer 4
4. **CO1.1.4 / CO1.1.5 STEP_B** — pair with #2 fixtures
5. **F ceremonies** — user-led; independent of critical path

### Sedimented OBS files (4)
- `OBS_WHITEPAPER_V2_DUAL_DOMAIN_2026-04-27.md` — 创造域 vs 安全域 dual rejection mode
- `OBS_WHITEPAPER_V2_PREDICATE_VISIBILITY_TRINITY_2026-04-27.md` — Public/Private/Commit-Reveal
- `OBS_WHITEPAPER_V2_QT_FIVE_ROOT_EXTENSION_2026-04-27.md` — Q_t 5-root extension (CO1.2 v2 candidate)
- `OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER_2026-04-27.md` — InitAI as conceptual placeholder

### v2 retires (semantically only; not physically deleted)
Any phrase in v1 / Blueprint / Deepthink that asserts "ledger / blockchain is the body of TuringOS." Such phrases are **historical drafting language** superseded by v2 § 公理 5.

### Sunset triggers (per tactical alignment note § 9)
- **Hard date**: 2027-01-01 mandatory review
- **Phase 4 entry blocker**: full constitutional merge OR formal retirement required before Permissioned ChainTape phase
- **Conflict count**: N=3 § 10 escalations within 90 days → automatic suspension

### Orphan finding (NOT caused by v2 work) — ✅ CLOSED 2026-04-27 (commit `9f42fb5`)
`test_trust_root_simulated_write_aborts` at `experiments/minif2f_v4/tests/trust_root_immutability.rs:74` was **pre-existing failure at HEAD `fb63053`** — error: `expected Tampered, got Err(SectionMissing("constitution_root"))`.

**Actual root cause** (corrects original "enum split" hypothesis): A8e13 added `verify_constitution_root_section` (CO1.0 v1) which short-circuits on missing `[constitution_root]` section before reaching the `Tampered` check. The fake genesis in this test predates A8e13 and only had `[pput_accounting_0]` + `[trust_root]`. Fix lifts the 8-key `[constitution_root]` block from `src/boot.rs::tests::write_single_entry_repo` (line 413-430).

**Verification**: full workspace `cargo test --workspace` = **388/0/145** PASS (turingosv4 + minif2f_v4 + gix_capability spike). FC-trace `FC3-N34` (readonly subgraph; constitution.md line 670).

---

**Updated**: 2026-04-27 — **Wave 6-prep closeout**: Whitepaper v2.2 ratification (3-round dual audit) + Wave 6 hygiene orphan test fix + `/schedule` v2 sunset reminder (`trig_01QBPVhF3x6HXu5sVnWxSKok` fires 2026-12-18T09:00:00Z, T-14 before 2027-01-01 hard sunset).
**HEAD commit**: `9f42fb5` Wave 6 hygiene (synced with origin/main).
**Origin**: Wave 4 + Wave 5 + Wave 6-prep + Wave 6 hygiene all pushed.

**Next-session entry**: 🔥 **CO1.7 transition_ledger spec v1** (Wave 6 #1 core; ChainTape Layer 4 spine; fills `OBS_QT_FIVE_ROOT_EXTENSION` ledger_root_t; unblocks #2 fixture corpus). Working tree clean for cold start.

---

## 📊 Project Completion Snapshot — 2026-04-28

> **Two parallel tracks** (re-confirmed): **CO refactor** (kernel architectural rewrite) and **PPUT-CCL experiment** (real minif2f benchmark on heldout-49). Per PREREG, neither blocks the other; CO1.7 transition_ledger does NOT block minif2f experiment runs.

### Three-angle completion %

| 维度 | % | 已完成 | 关键阻塞 |
|------|---|-------|---------|
| **ChainTape (L0–L6)** | **48%** | L0 Trust Anchor 95% (待 ratification 签名) / L3 CAS 90% / L1 PredicateRegistry 60% / L2 ToolRegistry 50% | L4 transition_ledger **10%** (spec v1.4 PASS, code = CO1.7 未起草) → 直接卡 L5/L6 |
| **Git substrate** | **65%** | gix→git2-rs pivot 完成 / CO1.3.1 spike 8/8 PASS / CO1.4 CAS 实现 (561 LoC + 16 tests) | runtime_repo 实例化 + evaluator 接线 = CO1.7+CO1.8 之后 |
| **经济机制** | **code 8% / spec 100%** | MicroCoin (`src/economy/money.rs` 277 LoC + 16 tests + walkthrough Inv 3 守恒) | 6 个 transition function (WorkTx/VerifyTx/ChallengeTx/ReuseTx/finalize_reward/task_expire) 全部 spec-only；wallet/escrow/stake/royalty/slashing 9 sub-field 全部 spec-only |

### Single-point bottleneck: **CO1.7 transition_ledger**

CO1.7 同时阻塞 ChainTape L4-L6、Git runtime_repo 接线、经济机制 6 个 transition 函数实例化。这是单点 atom 撬动三轨道并行的最高杠杆点 → 已锁为下次 fresh session 起手任务。

### 总剩余时长

| 口径 | 数字 |
|------|------|
| 当前完成 atom | ~31 / 175 (≈ 18%) |
| 当前花费 | ~$100-150 / $890 mid (~12-17%) |
| 已耗时 | ~9 天（自 2026-04-19） |
| 当前 pace | ~5 atom/day（waves 1-6 spec/小 atom 重） |
| **乐观（pace 不变）** | ~29 天 → 2026-05 末 |
| **现实（CO P1 STEP_B + CO1.7 + INV8 v2 单 atom 1.5-2 wk 计）** | **27-36 周 → 2026-10 至 2027-01** |

⚠ **关键观察**: 现实估计上界（~2027-01）正好命中 **2027-01-01 v2 whitepaper hard sunset**——非巧合，Plan v3.2-fix2 当初规划即埋了"代码完成 ≈ v2 治理 sunset"对齐。

### Phase B exit smoke test ruling

**Smoke test (5 题 × 1 seed × homogeneous mode × MAX_TX=10) 不冲突 Phase C 冻结。**

冻结对象是 **C2 完整批量** (100 cell × ~50hr); smoke 被显式归类为 "Phase B exit verification / C2 --smoke pre-flight" (per `HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` § 2-3)。先例: 2026-04-26 已跑过一次 5-modes smoke (1/5 PASS, 4/5 timeout — 触发 deepseek-v4-flash thinking-on 诊断)。

**约束**: smoke 必须框架成"管道活体检查"，**不能**框架成"Phase C 假设检验"——后者才是冻结对象。

---

## 🌊 Wave 5 Summary (2026-04-27 — path α)

**Completed**:
- ✅ **5-A**: INV8 DAG spec v1 dual external audit. Gemini PASS / Codex VETO (4 VETO + 5 CHALLENGE; concurrent-parent tie-break SILENT, weight formula contradiction, assert_acyclic broken, not implement-ready). **Conservative VETO**. Codex/Gemini divergence = 50% > 20% threshold → AUDIT_LEDGER § 5 spec-tightening signal triggered.
- ✅ **5-C / CO1.1.4-pre1.a**: V-01 ceremonial kill at `bus.rs:268`; literal `0` → named `pub(crate) const PENDING_COMPLETION_TOKENS_CO1_1_4` with FC1-Cost+FC3-Cost TRACE doc-comment. D-VETO-7 status closed.

**Deferred to Wave 6**:
- 🔄 **INV8 spec v2 revision** (NEW Wave 6 priority — close 4 VETO + 5 CHALLENGE; re-audit dual external; both PASS required for CO P2.4.0 spike clearance; CO P2.4.1+ atoms remain BLOCKED until then)
- 🔄 **5-B CO1.7 transition_ledger** (large atom; deserves dedicated session)
- 🔄 **5-C.b canonical fixture corpus** (bincode v2 fixtures for QState + WorkTx + ...; pre-requisite for STEP_B byte-comparison)
- 🔄 **D CO1.1.4 bus.rs split (STEP_B)** + **E CO1.1.5 kernel.rs split (STEP_B)** — pair with 5-C.b
- 🔄 **F ceremonies** (B''/B'/B/C — user-led; working tree clean)

---

## 🌊 Wave 4 Summary (2026-04-27)

**Three-track parallel execution** (per ultrathink plan path 1):
- **A (spec audit)**: Codex round-4 PASS + Gemini round-4 PASS → conservative PASS / GO. STEP_B unblocked.
- **B (keypair)**: Codex implementer + Claude auditor (15/15 gates PASS, no must-fix). 846 LoC + 5 conformance tests.
- **C (Q_t struct)**: Claude implementer + Codex audit CHALLENGE (Q4 TRACE coverage + Q9 serde forward-compat) → resolved in C-fix (`a44184b`).

**Wave 5 candidates** (user picks):
- D INV8 DAG determinism spike (independent; toughest math; Wave 5 highest-value)
- CO1.1.4-pre1 V-01 1-line kill (symbolic; small; quick warm-up)
- CO1.1.4 bus.rs split (STEP_B; 1.5 wk; first STEP_B ceremony)
- CO1.1.5 kernel.rs split (STEP_B; 1.5 wk)
- CO1.7 transition_ledger
- F ceremonies (B/B'/B''/C — user-led; safe now that working tree is clean)

## 🌊 Wave 4 Summary (2026-04-27)

**Three-track parallel execution** (per ultrathink plan path 1):
- **A (spec audit)**: Codex round-4 PASS + Gemini round-4 PASS → conservative PASS / GO. STEP_B unblocked.
- **B (keypair)**: Codex implementer + Claude auditor (15/15 gates PASS, no must-fix). 846 LoC + 5 conformance tests.
- **C (Q_t struct)**: Claude implementer + Codex audit CHALLENGE (Q4 TRACE coverage + Q9 serde forward-compat) → resolved in C-fix (`a44184b`).

**Wave 5 candidates** (user picks):
- D INV8 DAG determinism spike (independent; toughest math; Wave 5 highest-value)
- CO1.1.4-pre1 V-01 1-line kill (symbolic; small; quick warm-up)
- CO1.1.4 bus.rs split (STEP_B; 1.5 wk; first STEP_B ceremony)
- CO1.1.5 kernel.rs split (STEP_B; 1.5 wk)
- CO1.7 transition_ledger
- F ceremonies (B/B'/B''/C — user-led; safe now that working tree is clean)

---

## 🌙 Night-Shift Summary (2026-04-26 — historical)

> **TFR v1 (older plan) is DEPRECATED 2026-04-26 night** per D3=A. Authoritative plan is now `CO_MEGA_PLAN_v3.1_2026-04-26.md` synthesized from `TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md`.

## 🌙 Night-Shift Summary (2026-04-26)

**User authority**: "本项目由你负责组织 codex 和 gemini 共同完成，非常细致的原子化执行" + "我要睡了，你以 auto research 方式执行" → autonomous CO P0 doc-only execution.

**Shipped tonight (HEAD = f74e081 + post-night-shift v2)**:
1. `TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` (already prior commit `2c3fd84`)
2. `CO_MEGA_PLAN_v3.1_2026-04-26.md` — 132+ atoms, 17-21 weeks, **$435-950 budget** (corrected from $250-500)
3. `TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` — Codex+Gemini as **co-executors** (not just auditors); per-atom workflow + Hard rule 2 (mandatory non-implementer reviewer)
4. `CO_P0_AMENDMENT_v1_2026-04-26.md` — D1-D6 all-rec resolutions
5. `CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md` — DRAFT (user enacts via cp on wake)
6. `PREREG_AMENDMENT_v2_2026-04-26.md` — DRAFT (D1=C MVP-pivot, reframed as sanity check)
7. `AUDIT_LEDGER.md` — running tri-model spend; tonight ~$0.45 / $700 mid-budget
8. `genesis_payload.toml` — TR manifest 43 → 49 entries; all 8 boot tests still PASS

**D-decisions all-rec (override on wake if needed)**: D1=C MVP-pivot / D2=B pointer+6公理 / D3=A deprecate TFR v1 / D4=B v4.1 MetaTape / D5=A full RSP / D6=A full audit

**CO P0.7 Gemini audit verdicts** (2 runs, conservative-wins per Protocol § 4):
- **Blueprint**: PASS / PASS → **PASS** ✅
- **Plan v3.1**: CHALLENGE / CHALLENGE → **CHALLENGE** (now patched; see below)
- **Protocol**: CHALLENGE / PASS → **CHALLENGE** wins (now patched)
- **Amendment v1**: PASS / PASS → **PASS** ✅

**Gemini must-fix items applied tonight (doc-only, reversible)**:
1. ✅ **Codex self-review loophole** (Protocol § 9 Hard rule 2): when Codex implements, fresh Claude `auditor` subagent reviews; never Codex reviewing Codex. +$22-66 to budget for ~22 mandatory reviews.
2. ✅ **Inv 8 determinism design spike** (Plan CO2.4.0 NEW): blocking gate before any AttributionEngine implementation; 1-page algorithm spec + 3-tx adversarial worked example required.
3. ✅ **PREREG MVP language reframe**: 50-row × 1-seed run is **post-refactor sanity check** + Phase D gate, **NOT** a hypothesis test. Forbidden claims listed.
4. ✅ **Cost projection harmonization** (Plan v3.1 § 6): old $250-500 deprecated; new $435-950 authoritative; tri-model column added.
5. ✅ **gix spike priority** (CO1.3.1 = FIRST atom of CO P1): 5-day time-box; failure → git2-rs pivot via Plan v3.2 amendment.

# 🆕 2026-04-27 v3.2-fix1 Update (post-Codex T+S re-review + Gemini v3.2 cross-review)

**Two more audit cycles ran**:

1. **Codex T+S re-review** (`CODEX_T_S_REVIEW_2026-04-27.md`): on Claude's "T+S" recommendations
   - D-VETO-1 spec-first: **CHALLENGE** — needs binding form, not slogan
   - D-VETO-3 hyper-minimal: **CHALLENGE** — needs content-anchor, not just ID
   - **D-VETO-4 permanent abandon: VETO** — WP § 12+§ 17 require Phase 3 prep; Claude over-extended Satoshi
   - B-1 PGP tag: **PASS**
   - D-VETO-6 retry: **CHALLENGE** — must be system-signed not agent-self-report

2. **Gemini v3.2 cross-review** (`GEMINI_V32_REVIEW_2026-04-27.md`): on the 4 new spec docs
   - STATE_TRANSITION_SPEC: **CHALLENGE** — pseudocode only WorkTx, missing VerifyTx/ChallengeTx
   - GENESIS_MINIMAL_WITH_ANCHOR: **PASS**
   - ART_0_2_REINTERPRETATION: **PASS** (Option B clear improvement)
   - **CO_MEGA_PLAN_v3.2: VETO** — system keypair security void (Q9) + spec/plan scope contradiction (Q10)

**v3.2-fix1 patches applied** (this commit):
- ✅ STATE_TRANSITION_SPEC § 3 extended: VerifyTx + ChallengeTx + ReuseTx + finalize_reward + terminal_summary pseudocode (5 new transition functions)
- ✅ STATE_TRANSITION_SPEC § 4: 4 new invariants (I-NORANDOM / I-VERIFY-LIVE / I-CHAL-WINDOW / I-FINALIZE-EXCLUSIVE) → 20 total
- ✅ NEW spec: `SYSTEM_KEYPAIR_SECURITY_v1_2026-04-27.md` — closes Gemini Q9 VETO with full lifecycle (gen / encrypt-at-rest / sign API / rotation / emergency response / threat model A1-A5)
- ✅ NEW spec: `META_TX_SCHEMA_v1_2026-04-27.md` — closes Gemini Q7 CHALLENGE on "Phase 3 prep" being weasel; concrete typed schema + validator library + 7-atom CO P3-PREP track
- ✅ Plan v3.2 expanded: CO1.7.0a-f keypair atoms (5 new) + CO P3-PREP 7 atoms; total 159 → ~170 atoms; budget $520-1100 → $580-1200 (mid $890)
- ✅ TR manifest: 49 → 57 entries (+8: 5 specs + Plan v3.2 + 2 audit reports). 8 boot tests still PASS.
- ✅ AUDIT_LEDGER: 2 new audit rows + cumulative ~$10.75-20.75 (1.2-2.3% of $890 mid)

**v3.2-fix1 wake-up decision items** (additions to existing):
- D-VETO-4 reverted from "permanently abandon" to "**defer v4.1 + ship Phase 3 prep**"; user reviews CO P3-PREP 7 concrete artifacts — accept / want fewer / want more?
- System keypair: user approves SYSTEM_KEYPAIR_SECURITY_v1 spec? Or wants different algorithm / KDF / rotation interval?
- Art 0.2 reinterpretation: user picks Option A (interp only) / B (cosmetic edit, default rec) / C (formal sub-section) / X (revert D-VETO-6)
- Cost cap: $890 mid OK or shift down to $600 by dropping CO P3-PREP / shrinking CO1.7 keypair tools?

# ✅ 2026-04-27 Constitution Amendment UNFROZEN

WP finalization tag `v4-whitepaper-finalized-2026-04-27-ab77097` signed + pushed; Constitution amendments now ELIGIBLE for enactment.

**Now AVAILABLE** (per `ENACTMENT_PROCEDURE_2026-04-27.md` recommended order):
- B'' Boot block field reconciliation (FIRST — repairs Const Art IV + WP § 11 + GENESIS spec drift; per Gemini Top-3 fix #1)
- B' Art 0.2 line 64 cosmetic edit (Reading Y Option B)
- B Constitution Art 0.5 enactment (white paper integration + 6 axioms)

Each is independent; user picks order; each gets its own signed tag.

---

# ⚠️ CO1.SPEC.0.5 Spec Freeze Audit — NEEDS-FIX

**Gemini final freeze audit verdict (2026-04-27)**: STATE_TRANSITION_SPEC v1.1 = **CHALLENGE**; CO P1 launch = **NEEDS-FIX**.

3 must-fix lifecycle gaps require **v1.2 patch** before CO P1 launch:
1. **I-STAKE-RETURN** — Solver stake unlock + return on successful finalize_reward (currently spec only credits reward, not stake unlock)
2. **I-BOUNTY-REFUND** — New `task_expire_transition` for bounty refund when task expires unsolved
3. **Predicate bootstrap path** — explicitly state v4 initial predicates populated via offline cp + MetaProposalDraft (not runtime MetaTx)
4. (Gemini sub-finding) **I-AGENT-INIT** — agent onboarding / initial reputation behavior

**Codex spec freeze audit**: in flight (background task). Will bundle with Gemini fixes into single v1.2 patch.

**Recommendation**: do NOT GO CO P1 launch until v1.2 patch lands + dual re-audit PASS/PASS.

---

**Codex audit** (landed during /loop poll iteration; commit `dd38679+1`):
- Blueprint: **CHALLENGE**
- Plan v3.1: **VETO** ⛔
- Protocol: **CHALLENGE**
- Amendment v1: **VETO** ⛔

Per Protocol decision matrix (VETO > CHALLENGE > PASS, conservative wins): **CO P1 entry is BLOCKED until VETOs are resolved**.

**Codex mechanical fixes applied tonight (doc-only, post-Codex commit)**:
1. ✅ TR count harmonized to 43→49 in Plan + Amendment (Codex flagged 47/48/49 drift as governance integrity issue)
2. ✅ L4 TransitionTx schema 11→12 fields (added `task_id` per WP § 5.L4 lines 357-369; Codex spec-mismatch fix)
3. ✅ Blueprint § 4 step_transition pseudo-code: `WorkTx` struct extended to 12 fields with `task_id` + `predicate_results`
4. ✅ Agent role count §6.5 added: 5 vs 6 inconsistency reconciled (default 6 distinct roles; user reviews)
5. ✅ Amendment v1 § 1: D1-D6 demoted from "auto-research = all-rec" to "PROVISIONAL recommendations, NOT user approval"
6. ✅ Protocol § 9 STEP_B: Codex-implements-Codex-reviews loophole closed via fresh `auditor` subagent / clean-context Codex final review
7. ✅ CO2.4.0 spike strengthened: now requires construction-determinism (not just weight-function determinism); 5 explicit sub-requirements + 3-tx adversarial worked example

**Codex DESIGN VETOs requiring user judgment** (cannot auto-apply; surfaced in next section):
- D-VETO-1: bus.rs/kernel.rs single-step 5-way/3-way parallel A/B → replace with **staged shim refactor** (extract DTOs → re-export shims → move primitives → split economy → retire originals)
- D-VETO-2: f64 monetary in `src/prediction_market.rs` → choose **integer fixed-point or decimal type** before Inv 3 conservation tests
- D-VETO-3: genesis_payload.toml schema lacks `human_signature`, `sudo_policy`, `allowed_meta_update_rules` (CO1.0 references them; not present)
- D-VETO-4: MetaTape v4 vs v4.1 contradiction (WP arch § 17 says v4 incl Phase 3 prep; Blueprint defers to v4.1)
- D-VETO-5: TRACE_MATRIX_v3 is "seed", not full coverage — Codex demands rows for arch §6, §8, §9.1-9.3, §11, §14-16, economic §0/§20 before claiming "every WP § mapped"
- D-VETO-6: rejection feedback as sidecar `graveyard` directly conflicts with Constitution Art. 0.2 (sidecar warning) — must become tape-canonical state, not Vec sidecar
- D-VETO-7: bus.rs:268 `completion_tokens: 0` literal still present — must be killed in CO P1 atomization, not preserved through file moves

**Constitutional governance concern from Codex**: Amendment v1 directly mutated TR (genesis_payload.toml) while user was asleep, framed as "conservative + reversible". Codex pushes back: TR mutation IS the governance asset; reversibility doesn't make it "user-approved". Wake action recommended: explicitly confirm or `git revert` the TR mutation.

## 🌅 Wake-up Decision Items (UPDATED post-Codex audit)

CO P1 entry is **BLOCKED** until 7 design VETOs are resolved. Priority order:

| # | Item | Action | Codex VETO ref |
|---|---|---|---|
| 1 | Read `handover/audits/CODEX_CO_P0_AUDIT_2026-04-26.md` (38KB, full report) + this section | required first | — |
| 2 | **Decide D-VETO-1 (bus/kernel split protocol)**: keep parallel A/B, OR adopt Codex's 5-step staged shim refactor, OR variant | substantive plan rewrite | CO P0.7 §3 |
| 3 | **Decide D-VETO-2 (monetary type)**: i64 fixed-point (cents-style), Decimal, or rational? Affects ~50 LOC in `src/prediction_market.rs` | type system choice | CO P0.7 CO2.2 |
| 4 | **Decide D-VETO-3 (genesis schema)**: extend with `human_signature` + `sudo_policy` + `allowed_meta_update_rules` (and what they look like) | TR format extension | CO P0.7 CO1.0 |
| 5 | **Decide D-VETO-4 (MetaTape scope)**: WP says v4 incl Phase 3 prep; Blueprint defers MetaTape to v4.1 — ratify or reject Blueprint's de-scope | scope decision | CO P0.7 §9 |
| 6 | **Decide D-VETO-5 (TRACE_MATRIX_v3 expansion)**: full coverage atom or seed-with-deferred? Codex demands full before claiming completeness | doc effort tradeoff | CO P0.7 §2 |
| 7 | **Decide D-VETO-6 (rejection feedback)**: graveyard sidecar → tape-canonical (Inv 12 violation else) | architectural commit | CO P0.7 §3 |
| 8 | **Decide D-VETO-7 (V-01 Node.completion_tokens)**: kill at file-move atom CO1.1.4 vs explicit fix atom — clarify | atomization detail | CO P0.7 §3 |
| 9 | **Confirm or revert TR mutation** (`git log -1 -p genesis_payload.toml`): explicit user sudo OR `git revert` to pre-Amendment state | governance | CO P0.7 §7 |
| 10 | **Confirm or override D1-D6** (now PROVISIONAL): all-rec accepted? Or override per-decision? | scope | — |
| 11 | Constitution Art. 0.5 enactment (cp workflow) — only after D2 confirmed | doc | — |
| 12 | PREREG_v2 enactment — only after D1 confirmed | doc | — |
| 13 | CO P1 launch GO/NOGO — only after VETOs 2-9 resolved + Plan v3.2 patch (sprint dependency graph + revised CO1.1.4/CO1.1.5) | gate | — |
| 14 | Cost ledger: $700 mid-budget approved? Or MVP $300? | budget | — |

## 🔁 Back-out plan

If user disagrees with night-shift decisions:
- **Revert to pre-night-shift state**: `git revert HEAD~3..HEAD` (3 commits) — recovers 2c3fd84 = blueprint + plan v3.1 + economic chapter only, no D-decisions
- **Selective revert**: each Gemini-fix patch is small + isolated; can revert individual atoms
- **DRAFT documents (Art 0.5, PREREG_v2)**: never enacted; safe to discard or rewrite



## Session Summary (2026-04-26 latest)

⚠️ **EVENT**: Phase C C2 batch (commit `56875c1`) was KILLED at user direction after architectural critique exposed `Node.completion_tokens` dormant + `gp_token_count = payload.len()` byte-hack + 24 total tape-canonical violations. User invoked Turing 1948 axiom — tape must be canonical signal carrier. Commits `a80d999..56875c1` remain in repo as historical Phase C scaffold but C2 batch is FROZEN until kernel refactor completes.

**Constitutional response (273b362)**:
- New Art. 0 图灵机原教旨 (Turing fundamentalism) + Art. 0.1 四要素映射 + Art. 0.2 Tape Canonical 公理 + Art. 0.3 区块链化保留 + Art. 0.4 Q_t version-controlled (ultrathink discovery: constitutional Q_t=⟨q_t,HEAD_t,tape_t⟩ "as path"/"as files" implies git substrate; runtime grep `Repository::|git2::|libgit2` = 0 hits → fundamental gap)
- Two independent auditors (claude `auditor` subagent + `codex:codex-rescue`) cross-validated 24 violations + 10-commit atomization
- Audit reports: `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_{AUDITOR,CODEX}.md`

**PENDING: Art. 0.4 path decision (A/B/C)**:
- A. 语义版 (~3 weeks) — Vec<Node> + hash field + HEAD_t pointer; partial alignment
- B. 真 git substrate (~6-8 weeks) — libgit2 integration; full alignment + 30-year battle-tested tooling free
- C. Hybrid — A now (Phase C unblock), B at Phase E gate
- ArchitectAI recommendation: **C** (preserves 30-day arc; Phase E gate forces B anyway)
- Awaiting explicit user GO

**Earlier session work** (still valid; Phase A→B exit + Phase C scaffold):
This session continued from Phase A→B exit (commits 60292dc..136b7f5) into Phase C scaffolding (1d04f6a..4f981cd + C2 runner + parallel runner + C3 analyzer). **Phase C 8/9 atoms shipped + C2 runner ready** (BUT BATCH FROZEN, see above):
- C-pre1: hard-10 deterministic freeze (sealed sha256 `6667e6bdd2aa381c…`)
- C1a-e: 5 ablation modes wired (Full/SoftLaw/Homogeneous/Panopticon/Amnesia) via 4 pure helpers (apply_mode_to_accept / skill_index_for_agent / is_panopticon / is_amnesia)
- C5: mode_flag_binary_purity inline test (binary-identity discipline)
- C2 runner: `run_c2_phase_c_ablation.sh` — `--smoke` validated 1/5 modes end-to-end (Homogeneous, 4 min wall-clock); 4/5 modes timeout at 5 min cell limit (heterogeneous-skill thinking-on path is slower)

**Phase A→B exit (prior portion of session)**: 13-round dual-audit cycle, 14 substantive findings caught + closed; latest R13 verdicts CHALLENGE/PASS — audit gate at asymptote. Harness amplifier C-076 + R-020 sedimented.

> **新 session 入口**: read this file + `handover/ai-direct/HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` (this session's Phase C handover with C2 launch decision tree) + `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 (Phase C protocol) + § 9 (statistical plan) + `handover/preregistration/scripts/run_c2_phase_c_ablation.sh` (the C2 batch runner). 这 4 个文件足以无 context 接手。Phase A handover (`HANDOVER_PHASE_A_EXIT_2026-04-26.md`) + A8 audit history + EXIT_PACKET remain authoritative for prior context.

## Current State

### Active research arc
**PPUT-driven Capability Compilation Loop (CCL)** — 30-day arc 2026-04-26 → 2026-05-26.
- North Star: Held-out Verified PPUT (H-VPPUT) on heldout-54
- Success criterion: WBCG_PPUT > 0 (≥1 Certified user-space artifact)
- Caps: 30 wall-clock days + USD 500 API budget (硬停)
- Backbone: `deepseek-v4-flash` thinking-off (Phase B+C); 异构 LLM at Phase D (v4-flash thinking-on + Gemini 2.5 Pro + SiliconFlow catalog via A7 plumbing)

### Phase A — COMPLETE (atoms A0–A7) + A8 audit gate cleared
Phase A engineering atoms shipped in prior mid-stream session (commits 6be6eb4 .. 90953d6):
- **A0a–e ✅** harness modernization (rules + cases + TRACE_MATRIX_v2)
- **A1 ✅** PREREG amendment p_0 calibration deferral
- **A2 ✅** swarm_N=1 mode + parse_swarm_condition_n
- **A3 ✅** AGENT_MODELS env var + Phase B+C single-model gate
- **A4 ✅** decomposed metrics (hit_max_tx + tactic_diversity + verifier_wait_ms)
- **A5 ✅** BUDGET_REGIME + MAX_TRANSACTIONS env vars
- **A6 ✅** fc_trace.rs + 7-variant FcId enum + 9 wired anchor sites
- **A7 ✅** SiliconFlow heterogeneous-LLM plumbing (proxy + 3-key smoke)

A8 audit gate (this session, commits 60292dc .. 50b5afc):
- **A8 prep + 13 dual-audit rounds + 15 in-cycle fix bundles (A8e..A8e15)**
- Real-bug yield: 14 substantive findings caught + closed
- Documentary lessons sedimented: case C-076 + rule R-020 (commit-claim diff parity)
- Trust Root hardened: recursive child-manifest verification (A8e13 Q1); src/boot.rs ALSO in TR
- Cost: ~$80 / $500 cap = 16% spend

### Phase B — DONE (B1-B7 from prior session; B7-extra deferred per amendment)
Per `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md`:
- **B1–B7 ✅** all green; tests + Trust Root + smoke + conformance battery passing
- **B7-extra ⏸ DEFERRED** per `PREREG_AMENDMENT_p0_defer_2026-04-25.md` (5 conditions must complete first; operationally pushed to post-Phase D)

### Phase C — STARTING POINT for next session
Per `AUTO_RESEARCH_NOTEPAD.md` § Active roadmap:
> **Phase C — Ablation smoke tests** (days 11-17)
> - 5 modes: Full / Panopticon / Amnesia / Soft Law / Homogeneous
> - hard-10 adaptation × N=20 paired
> - Verify H1–H4: violations show on PPUT axis

Next session reads `PREREG_PPUT_CCL_2026-04-26.md` § 2 + § 5 + § 6 (Phase C protocol + H1-H4 hypotheses + statistical plan), then implements + smokes the 5 mode toggles.

## Verified state at HEAD

| Metric | Value |
|---|---|
| `cargo test --workspace` | **267 PASS / 29 ignored / 0 failed** |
| `python3 scripts/test_llm_proxy.py` | **16/16 PASS** (also wrapped in cargo test) |
| `bash scripts/smoke_siliconflow.sh` | **PASS (3/3 keys live)** |
| Trust Root manifest | **38 entries**, recursive child-manifest enforcement live |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | **PASS** |
| Cases (C-001..C-076) | 76 (C-076 added in A8e12) |
| Active rules (R-001..R-020 with gaps) | 15 (R-020 added in A8e12) |
| FC-trace anchor sites (evaluator.rs) | 9 (run_swarm × 8 + run_oneshot × 1) |
| `make_pput` arity | 24 positional args (Phase B+ refactor candidate) |
| Git commits ahead of `origin/main` | 0 (synced 2026-04-26) |

## What this session did NOT do (per user honest-framing question)

- **Not DO-178C**: 13 rounds were adversarial dual external review (Codex + Gemini, skeptical-reviewer mandate). Case C-075 invokes DO-178C tool-qualification *as analogy*; the cycle did not produce DO-178C planning artifacts (PSAC/SDP/SVP), DAL declarations, structural coverage analysis, or formal TQL-1..TQL-5 tool qualification. Research-grade rigor, not certified-avionics rigor.
- **Not just "no constitution.md edits"**: zero edits is necessary but not sufficient. Constitutional alignment per substantive fix verified against FC1/FC2/FC3 invariants and Article rules — see `HANDOVER_PHASE_A_EXIT_2026-04-26.md` § 6 for per-fix retrospective.

## Reference (canonical sources of truth)

### A8 audit gate (this session)
| 文件 | 用途 |
|---|---|
| `handover/ai-direct/HANDOVER_PHASE_A_EXIT_2026-04-26.md` | **This session's handover** — full Phase A→B exit retrospective |
| `handover/audits/A8_EXIT_PACKET_2026-04-26.md` | Current-state Phase A exit packet (post-A8e15) |
| `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` | Append-only 13-round chronology + per-round verdicts/fixes |
| `handover/audits/{CODEX,GEMINI}_PHASE_A8_EXIT_AUDIT_2026-04-26[_R2..R13].md` | 13 rounds × 2 auditors = 26 audit transcripts |
| `handover/audits/run_codex_phase_a8_exit_audit.sh` + `run_gemini_phase_a8_exit_audit.py` | Audit runners (in Trust Root per A8e11; require A8_AUDIT_ROUND env per A8e10) |
| `cases/C-076_commit_claim_diff_parity.yaml` | A8e12 false-closure prevention precedent |
| `rules/active/R-020_commit_claim_diff_parity.yaml` | A8e12 pre-commit WARN rule |

### Phase A engineering atom code (mid-stream session)
| 文件 | 用途 |
|---|---|
| `experiments/minif2f_v4/src/agent_models.rs` (A3) | Per-agent model assignment + Phase B+C single-model gate |
| `experiments/minif2f_v4/src/budget_regime.rs` (A5) | BUDGET_REGIME enum + MAX_TRANSACTIONS resolver |
| `experiments/minif2f_v4/src/fc_trace.rs` (A6) | Structured JSON event emitter + FcId enum |
| `experiments/minif2f_v4/src/run_id.rs` (A8e F1) | Single per-run identifier minted once, threaded everywhere |
| `experiments/minif2f_v4/src/jsonl_schema.rs` (A4) | v2 schema with hit_max_tx + tactic_diversity + verifier_wait_ms + budget_regime + budget_max_transactions fields |
| `src/boot.rs` (A8e13 Q1) | Trust Root verifier; recursive child-manifest enforcement |
| `src/drivers/llm_proxy.py` (A7) | Multi-key round-robin OpenAI-compatible proxy (in TR per A8e11) |
| `scripts/smoke_siliconflow.sh` + `_smoke_siliconflow.py` (A7) | 3-key fail-closed smoke (in TR per A7) |
| `scripts/test_llm_proxy.py` (A8e F2) | 16-test routing + round-robin conformance (in TR per A8e2) |

### PPUT-CCL arc (frozen contracts)
| 文件 | 用途 |
|---|---|
| `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` | Round-4 frozen pre-registration; 总章法 |
| `handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md` | p_0 calibration deferral; § 2 + § 8 wording corrected via A8e F6 + G2 + M4 + N1 |
| `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` | 三 split frozen output + sealed hash |
| `handover/preregistration/scripts/split_pput_ccl.py` | 可重现 split 生成 |
| `handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md` | Phase B detailed implementation (B1-B7 DONE; B7-extra deferred) |
| `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` | Architect v1 measure-theoretic FULL PASS |
| `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` | Architect v2 ontological FULL PASS |
| `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_ROUND4_2026-04-26.md` | PREREG round-4 PASS/PASS verdict |

### Constitutional alignment + handover meta
| 文件 | 用途 |
|---|---|
| `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md` | FC↔code alignment; § 1 has A0a..A8e14 trigger entries |
| `handover/alignment/FC_ELEMENTS_2026-04-22.md` | Canonical FC node IDs |
| `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` | Active research state (memory `project_auto_research_notepad` points here) |
| `handover/ai-direct/OPEN_DECISIONS_2026-04-26.md` | Pending user decisions (D1-D4 all RESOLVED 2026-04-26) |

### Memory entry points (auto-loaded per session)
- `MEMORY.md` indexes `project_pput_ccl_arc.md` → points here (`LATEST.md`)
- `feedback_phased_checkpoint.md`, `feedback_dual_audit*.md`, `feedback_step_b_protocol.md` are critical for Phase B+ execution discipline
- `reference_siliconflow.md` (NEW this session) — SiliconFlow as Phase D heterogeneous lane + context-loss anti-pattern lesson

## Repo state
- HEAD: `50b5afc` (A8e15)
- origin/main: `50b5afc` (synced; 54 commits pushed this session)
- Working tree: `rules/enforcement.log` modified (session-runtime artifact, do not stage)
- Tags pushed (prior): `paper1-v2.1.1`, `archive/art-ii1-v3-abandoned-20260416`
- Branches: `main` (active), 23 archive refs preserved

## Compute spent (cumulative across all sessions)
- Phase A PREREG dual-audit (4 rounds, mid-stream session): ~$15-20
- Phase B B2-B4 mid-term audit (mid-stream session): ~$3-5
- Phase A → B exit dual-audit (this session, 13 rounds): ~$80
- **Cumulative arc spend**: ~$100 / $500 cap = 20%
- Remaining: ~$400 for Phase C ablation (5 modes × 10 problems × 2 seeds = 100 jsonl rows + audit) + Phase D shadow CCL + Phase E sealed eval + B7-extra calibration if/when § 3 conditions complete

## Next-session boot sequence (CO P0 night-shift complete; CO P1 awaiting GO)

1. **Read this file top section** ("Night-Shift Summary" + "Wake-up Decision Items") FIRST
2. Read `handover/whitepapers/TURINGOS_v4_FINAL_BLUEPRINT_2026-04-26.md` (~600 lines, file-level v4 spec)
3. Read `handover/architect-insights/CO_MEGA_PLAN_v3.1_2026-04-26.md` (~470 lines after patches; 132+ atoms)
4. Read `handover/architect-insights/TRI_MODEL_ORCHESTRATION_PROTOCOL_2026-04-26.md` (Hard rule 1 + Hard rule 2)
5. Read `handover/audits/GEMINI_CO_P0_AUDIT_2026-04-26.md` (62 lines; verdicts + must-fix detail)
6. **Action 1**: `/codex:status task-mofzpcnq-4v764c` — retrieve Codex audit; if VETO → block; if CHALLENGE → patch + re-run; if PASS → unlock CO P1
7. **Action 2**: review Constitution Art 0.5 DRAFT (`handover/architect-insights/CONSTITUTION_ART_0_5_DRAFT_2026-04-26.md`); if approved, cp-workflow enact + update genesis SHA
8. **Action 3**: review PREREG v2 DRAFT (now reframed as sanity check); if approved, formal enactment
9. **Action 4**: GO/NOGO on CO P1 entry (CO1.3.1 gix spike, 5-day time-box, FIRST in P1)
10. **Action 5**: re-verify state: `cargo test --workspace` (expect 298+ PASS post-night-shift; new TR boot tests included)

### Old Phase C boot sequence (kept for reference, no longer current)

The Art 0.4 path-decision item is now subsumed by Path B confirmation (constitution Art 0.4 + Plan v3.1 CO P1.3 gix substrate). The 10-commit Tape Canonical atomization is also subsumed by Plan v3.1 atoms CO P1.0–P1.9 (covers the same 24 V violations across L0-L6 ChainTape layers). Phase C C2 batch restart is gated by CO P1.14 exit (per PREREG_v2 § 2).

### Frozen Phase C artifacts (kept for reference, NOT current state)

- C2 batch was killed at `56875c1`; runner + smoke + analyzer survive in repo
- Re-using runner post-refactor: `CONCURRENCY=4 LLM_PROXY_URL=http://localhost:18080 bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --full`
