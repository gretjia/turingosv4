# TuringOS v4 — Handover State

> 📍 **PROJECT DECISION MAP** (read this first if cold-starting): `handover/architect-insights/PROJECT_DECISION_MAP_2026-04-27.md`
> Tracks every decision + every skipped option + every atom status + forward roadmap.
> Anti-forget pledge: no skipped option is silently retired without explicit fate logged.

---

## 🔍 2026-05-01 — TB-5 post-ship self-audit + chaintape gap surfaced (architect review awaiting)

**Authorization**: user "没有针对烟测的tape进行审计，由你负责审计，不需要外审" → single-AI self-audit (no external auditor). Follow-up: "现在 turingos 具有真正的 chaintape 了吗？你是在 chaintape 上读取的测试全部信息进行审计的吗？" surfaced the substantive finding.

### What landed

| File | Purpose |
|---|---|
| `handover/audits/SELF_AUDIT_TB_5_SMOKE_TAPE_2026-05-01.md` | Smoke-tape self-audit. §1: 8 verified claims PASS. §2: cosmetic test-count under-report (464→617). §3: substantive chaintape gap. §4 verdict + remedy. §5 audit caveats. |
| `handover/audits/STAGE_AUDIT_TB_1_TO_TB_5_2026-05-01.md` | Cumulative stage audit TB-1..TB-5. §1 per-TB summary table. §2 what's structurally green (kernel, Anti-Oreo, RSP-1/2/3.0/3.1, anti-drift CI). §3 what's gap (production-binary chaintape wire-up, smoke evidence is paper trail not chain, RSP-3.2/4/5/6/7 RED, P2/P4 RED). §4 5 open debts. §5 8 production claims rolling forward. |
| `handover/directives/2026-05-01_TB6_ARCHITECT_REVIEW_REQUEST.md` | Architect review request with 5 binding decision items D1-D5 for TB-6 sequencing + audit-mode standard + chaintape gap remedy. Awaiting `2026-05-XX_TB6_DIRECTIVE.md` response. |
| (patch) | 5 living docs corrected from 464/464 → 617/617 (`README` + `RECURSIVE_AUDIT` + `TB_LOG` × 2 + `NOTEPAD`). Merge commit `1bdc55a` body cannot be amended; superseded by reference. |

### Key findings (one-liner each)

1. **(cosmetic) Test-count under-report**: TB-5 ship-gate "464/464" was bare `cargo test`; actual `cargo test --workspace` is **617/617** (46 suites, 0 failed). Off by 153 tests across 5 docs. Patch commit on main.

2. **(substantive) Chaintape gap**: TB-5 "smoke tape" evidence (`oneshot_run.log` + `n1_run.log` + `proof_n1.lean` + `README.md`) is **paper trail, not chain**. No production binary drives `Sequencer::apply_one`. `bus.rs` sequencer field is `None` in main.rs. The evaluator does not import `turingosv4::state::sequencer`. The chaintape machinery only runs inside `cargo test` (InMemoryLedgerWriter). No on-disk chain has ever been produced from any LLM-driven run in TuringOS history. **5-TB cumulative debt** (TB-1..TB-5 each shipped kernel improvement; none exercised by an LLM-driven binary).

3. **Audit performed was paper-tape level**: I read 4 files + cross-grepped 5 evidence dirs + sha256-matched the proof artifact + re-ran Lean v4.24.0 + re-ran `cargo test --workspace`. The cargo test re-run IS a chain audit (in-memory chain) — but for the cargo test suite, not for the smoke runs themselves. The .log files are bounded by conventional file-system trust, not cryptographic chain trust.

4. **"Smoke tape" naming is a v3 PaperTape-era metaphor**, not a structural property. Recommend rename → "smoke evidence" (architect review D5).

### What architect needs to rule on (D1-D5 in review request)

- **D1**: TB-6 = RSP-3.2 slash (current ROADMAP plan) vs P2 Agent Runtime atom (close chaintape gap first; recommended). Stake: 5-TB chaintape debt vs additional kernel-only TB.
- **D2**: smoke gate evolution — should chaintape traversal become required from TB-X onward?
- **D3**: audit-mode standard — TB-3/TB-4 Option B (self-audit + smoke) vs TB-5 Codex-only vs hybrid by constitutional risk class.
- **D4**: lock down `cargo test --workspace` as canonical ship-gate test command.
- **D5**: rename "smoke tape" → "smoke evidence" across docs.

### What's substantively defensible at TB-5 ship (despite the gap)

- 8 production claims (Anti-Oreo, RSP-0/1/2/3.0/3.1 chain, defense-in-depth pinned-pubkeys, CTF conservation, 9-sub-field invariant) all GREEN under `cargo test --workspace` (617 tests).
- Lean re-verification holds end-to-end on the one proof produced.
- Smoke runs were genuine (timestamps + run_ids verified session-fresh, not stale repeats).

### What's NOT proven by smoke evidence (despite ship docs language)

- That TB-5 runtime spine was reachable from the evaluator
- That any TypedTx ever traversed `dispatch_transition` during the smoke runs
- That any LedgerEntry was produced
- That the runtime kernel's Anti-Oreo barriers were ever exercised at LLM-driven runtime

These belong to **P2 Agent Runtime** wire-up, deferred from TB-1..TB-5 by design. Architect ruling on D1 determines when this debt closes.

---

## 🚢 2026-04-30 — TB-5 SHIPPED (P3 RSP-3.0 + RSP-3.1 System-Emitted Resolution Gate, WP-canonical)

**Authorization**: user "继续直到本轮次所有plan中的事项完成" → executed Atoms 4-8 + ship + book-keeping in one session post-context-compaction.

### What landed (12 commits)

| Commit | Atom | Summary |
|---|---|---|
| `42fd45c` | Atom 2 | TB-5.0 substrate: `submit_agent_tx` + agent-ingress barrier (4 system variants rejected pre-queue) |
| `4a33b1a` | Atom 3 | TB-5 ABI: `ChallengeResolveTx` + `ChallengeStatus` (q_state.rs) + `ChallengeResolution` (typed_tx.rs) + `monetary_invariant` cascade |
| `9ff8179` | Atom 4 | `emit_system_tx` + apply_one stage 1.5 (defense-in-depth pinned-pubkey verification) + `record_rejection` helper |
| `06a7fcf` | Atom 5 | `ChallengeResolve` dispatch arm (Released path) + `CHALLENGE_RESOLVE_DOMAIN_V1` state-root domain + 4 new TransitionError variants |
| `c7dfef9` | Atom 6 | UpheldDeferred path + boundary tests (I75-I77 + I78-I79 + I88-I89) |
| `cc72d61` | Atom 7 | Replay (I80) + property (I81) + anti-drift CI (I82-I87, `tests/tb_5_anti_drift.rs`) |
| `2fb4ed9` | Atom 8 | Recursive self-audit + 真实烟测 evidence |
| `1bdc55a` | merge | `--no-ff` merge experiment branch into main |
| `c472823` | book-keeping | TB_LOG / NOTEPAD / ROADMAP post-merge updates |

**Acceptance battery**: **617/617** `cargo test --workspace` passing, 0 failed (corrected 2026-05-01 from original 464/464 ship-time figure). 46 net new TB-5 tests vs TB-4 baseline 571.

### Production claim adds

1. Anti-Oreo agent-vs-system ingress separation **structurally enforced** (was documented norm without live enforcement through TB-3 + TB-4).
2. `emit_system_tx` constructs + signs system-emitted typed txs INTERNALLY; callers cannot pass forged signatures.
3. apply_one stage 1.5 re-verifies against `PinnedSystemPubkeys` (defense-in-depth catches stale-sig replay → `InvalidSystemSignatureLive` + 1 L4.E PolicyViolation row, no logical_t advance — K1).
4. `ChallengeResolve` dispatch enforces idempotent single-shot resolution: Released refunds + zeros bond (entry preserved); UpheldDeferred is marker-only (bond preserved for TB-6 slash routing).

### 真实烟测 (handover/evidence/tb_5_smoke_2026-04-30/) — NOTE: see 2026-05-01 audit above

- oneshot `prompt_context_hash="a1f43584a17d1226"` — bit-identical across **5 sessions** (TB-1/2/3/4/5)
- n1 `solved=true`, `verified=true`, `gp_payload="nlinarith"` on `mathd_algebra_107` with `budget_max_transactions=20`
- ⚠️ **Per 2026-05-01 self-audit § 3**: this is paper-trail evidence, NOT chain audit. The kernel structural claims live in `cargo test --workspace`; smoke evidence proves prompt-build pipeline compat + capability replicability.

### Self-audit (handover/audits/RECURSIVE_AUDIT_TB_5_2026-04-30.md)

6/6 directive Q1-Q6 + 10/10 charter v2 § 4 decision blocks + 4/4 anti-drift renames + 3/3 ship gate proofs all GREEN. Test count corrected to 617/617 in-place 2026-05-01.

### Audit-mode (TB-5 specific)

Directive § 4 Q4 mandated Option A (dual external) — Gemini strategic-tier `MODEL_CAPACITY_EXHAUSTED` across 4 rounds; supplement `2026-04-30_TB5_audit_mode_supplement.md` documented Codex-only mode; round-4 fell back to **grep self-verification** when Codex agent infra failed mid-audit.

### Next TB candidate (awaiting architect ruling D1)

- **Default per ROADMAP**: TB-6 = RSP-3.2 slash execution (`SlashTx` system-emitted; balances/stakes/challenge_cases mutations conditional on `ChallengeCase.status == UpheldDeferred`)
- **Recommended per 2026-05-01 audit**: TB-6 = P2 Agent Runtime atom (close 5-TB chaintape gap first; slash defers to TB-7)

---

## 🌙 OVERNIGHT 2026-04-29 — TB-1 Days 4-6 shipped autonomously; **CHALLENGE verdict, user decision needed**

**Authorization**: user "进行到送双外审并收集双外审结果给我睡觉回来看" → ran TB-1 Day 4 + Day 5 + Day 6 (dual external audit) end-to-end. **Did NOT ship Day 7** — that requires user decision.

### What landed (3 commits)
| Commit | Day | Summary | Tests |
|---|---|---|---|
| `50a1d67` | Day 4 | P6 `h_vppu_history` instrumentation (NEW file) — capacity-3 rolling window, persisted JSON store, post-hoc stamped in evaluator main(); live verified on 2× mathd_algebra_107 n3 runs (run 2: `h_vppu=6.21`) | 9/9 unit; live signal ✅ |
| `6c04c26` | Day 5 | Tier-A 9-acceptance battery consolidated into `tests/tb_1_acceptance.rs`; superseded `tb_1_p1_acceptance.rs` | **9/9 Tier-A green** + 4 Tier-B ignored as designed |
| (none) | Day 6 | Dual external audit launched (Codex + Gemini parallel) | Reports landed |

Full workspace: **491 passed / 0 failed / 150 ignored** at HEAD `6c04c26`.

### Dual audit verdicts (round 1)

| Auditor | Verdict | Conviction | Latency | Cost |
|---|---|---|---|---|
| Codex | **CHALLENGE** | high | ~6 min | ~$3-4 |
| Gemini DeepThink | PASS | 5/5 | 53s | ~$1-2 |

**Merged verdict** per `feedback_dual_audit_conflict` (VETO > CHALLENGE > PASS): **CHALLENGE**. TB-1 must NOT auto-ship Day-7.

Full merged write-up: **`handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md`** (read this first when reviewing).

### Codex P0s (the gap)

The 9 Tier-A tests are technically green and prove the **primitives**, but Codex argues they don't prove the **central ship claim** ("the v4 GitTape kernel honors the L4/L4.E split + RSP-0 invariants enforced") because:

1. **Sequencer dispatch is `NotYetImplemented`** for all K5 variants → L4/L4.E disjointness is asserted at primitive level, NEVER through a real `dispatch_transition` route. Tier-A bypasses dispatch entirely.
2. **Monetary guards (assert_no_post_init_mint / assert_total_ctf_conserved / assert_read_is_free) have no production call sites** — only unit + Tier-A tests reference them. A future dispatch path that forgets to call them would silently bypass.
3. **`RejectedSubmissionRecord` raw shielding is convention, not type-enforced** — `pub` struct, derives `Serialize`, `pub raw_diagnostic_cid`, `records()` returns raw refs. The `PublicRejectionView` projection is correct, but any code path that goes around it leaks the raw cid.
4. **`AcceptedLedger::load_from_path` skips `verify_chain`** — `prev_hash`/`hash`/`logical_t`-only tampers can load successfully unless caller separately verifies. Tier-A bypass test catches one specific tamper shape but misses fake-genesis, row-reorder, parent-state-root-only.

Gemini explicitly disagreed on 1 + 2: "primitives ready for TB-2 wiring is the right tracer-bullet level." This is a SCOPE-OF-CLAIM divergence, not a bug-vs-no-bug divergence.

### 3 paths (user decides)

- **Path A (recommended; ~1h)**: narrow the central claim in recharter + commit messages — "TB-1 ships PRIMITIVES + INVARIANTS, NOT dispatch enforcement". Optional sweeteners: P0-2 (~30min, all-six-subindex Tier-A test) + P0-3 (~30min, `#[serde(skip_serializing)]` on raw_diagnostic_cid). Ship Day-7 with narrowed claim; **skip round-2** (Codex's CHALLENGE was about claim scope, not bugs; narrowing addresses it directly).
- **Path B (heavier; ~3-6h)**: fix all 4 P0s (incl. wiring `dispatch_transition` for at least one variant + 3 more tamper tests + manifest-level shielding patch); then run round-2 audit per Elon-mode 2-round cap.
- **Path C**: defer ship; fold dispatch_transition into TB-2 RSP-1 scope.

**Default if no decision**: do nothing — TB-1 stays at HEAD `6c04c26`. No further auto-action.

### Compute spend
- TB-1 Days 4-5 (build): ~$0 (local cargo + 2 small live runs ≤ $0.10)
- TB-1 Day 6 (dual audit r1): **~$5-6 total** (Codex 154K-token prompt + Gemini 197K-char prompt). Within TB-1 $30 audit budget; ~$24 reserved for round-2 if Path B.

### Where to start when reviewing
1. `handover/audits/DUAL_AUDIT_TB_1_VERDICT_2026-04-29.md` — merged verdict + the 3 paths
2. Skim `handover/audits/CODEX_TB_1_AUDIT_2026-04-29.md` Section A-E (last ~100 lines of the file; preceding lines are Codex's exec investigation log, not the verdict)
3. `handover/audits/GEMINI_TB_1_AUDIT_2026-04-29.md` (full 80 lines — concise PASS verdict)
4. `tests/tb_1_acceptance.rs` — the 9 Tier-A tests under audit

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

**Updated**: 2026-04-28 — **Wave 6 #1 CO1.7 spec PASS/PASS gate cleared** (`a946820` v1.2). Three rounds of dual external audit converged: R1 CHALLENGE/CHALLENGE → R2 PASS/CHALLENGE → R3 PASS/PASS. Spec + skeleton + system_keypair extension all audit-cleared; CO1.7 implementation start now unblocked.
**HEAD commit**: `7bd02ad` round-3 audit runners (post-`a946820` v1.2).
**Origin**: through `5829e32` pushed; rest local-only (push when user ready).

**Next-session entry**: 🚀 **CO1.7 implementation** (now unblocked per `handover/audits/CO1_7_DUAL_AUDIT_VERDICT_R3_2026-04-28.md` PASS/PASS). Per spec § 13: 3 downstream atoms estimated 5-9 days total for Wave 6 #1 closure:
1. CO1.7-impl proper (~600-900 LoC + 4 CO1.7.5-stage tests)
2. CO1.4-extra (NEW atom; ~150-300 LoC + 3-4 tests; CAS index persistence — required for full-mode replay across cold restart)
3. CO1.7.5+ wiring (head_t mutation; integration with bus.rs/kernel.rs — STEP_B required per CLAUDE.md "Code Standard")

CO1.7 audit cost: ~$25-42 (3 rounds; cumulative project ~$135-202 / $890 mid). Working tree clean.

---

## 🚨 2026-04-29 Session-3 — CAPABILITY-FIRST PIVOT + ✅ FIRST V4-NATIVE SOLVE (~80 min after pivot)

**Status**: User raised "no confidence in dev capability" challenge after 7-day atom-spec wave. Web research + internal eval confirmed spec-craft drift. Pivot codified at commit `a906886`. **B target met within 80 min**: `mathd_algebra_107` solved end-to-end at HEAD `a906886` via v4 evaluator binary, OMEGA accept depth=1, 10.0s wall-clock, single tactic `nlinarith`. Independently re-verified via `lean --stdin` exit 0. **Evidence**: `handover/evidence/first_v4_solve_2026-04-29/`.

### B result — first v4-native solve

| Metric | Value |
|---|---|
| Problem | `mathd_algebra_107` (adaptation split) |
| Condition / Mode / Model | `n3` / `full` / `deepseek-chat` |
| `MAX_TRANSACTIONS` | 50 |
| `solved` / `verified` | true / true |
| Golden-path tactic | `nlinarith` |
| `tx_count` / `gp_token_count` | 1 / 12 |
| Wall-clock | 9.95s |
| `pput_runtime` | 0.000215 |
| `pput` (PPUT/s) | 10.04 |
| HEAD | `a906886` |
| Independent re-verify | ✅ exit 0 |

**Closes**: 7-day "0 v4-native solves" gap. Capability path is alive at HEAD; CO1.x substrate atoms did NOT break the pre-v4 evaluator path.

### Auxiliary finding — `oneshot` regression bug (file separately; not B-blocking)

Two `condition=oneshot` retries failed deterministically in 9-11s with identical Lean parse error: `<stdin>:10:33: error: unexpected token 'by'; expected '{' or tactic`. Same model/problem/HEAD with `condition=n3` solved cleanly. **Implication**: `run_oneshot` code path in evaluator.rs has prompt-template or output-parsing bug; `n3` swarm path uses different scaffolding and works. Filed for ≤1-day follow-up atom.

### Landing eval (delivered 2026-04-29 12:25 by Explore agent)

**Architectural completion ~28%** (defensible measure):
- L0 Constitution: ✅ wired (boot.rs + genesis_payload + Trust Root)
- L1 Predicate Registry: ✅ wired (146 pub items + 18 conformance tests)
- L2 Tool Registry: ⚠️ scaffold only (registry struct; tool dispatch stubs)
- L3 CAS: ✅ wired (git2 blobs + JSONL sidecar; 4 round-trip tests)
- L4 Transition Ledger: ✅ wired (LedgerEntry + Git2LedgerWriter; CO1.7-extra closed)
- L5 Materializer: 🛑 SPEC-ONLY DEFERRED (CO1.8 v1 r1 found 2 P0s)
- L6 Signal Indices: ❌ not started
- L7 Read View: ⚠️ partial (snapshot.rs + prompt_guard; no full rtool/wtool trio)

**5-step compile loop**: 3/5 wired (Proposal, Ground-Truth Feedback, Logging) + 2/5 stubbed (Capability Compilation, ↑H-VPPUT feedback)
**Capability path**: 0% → 0.4% (1 solve / ~244 problems = 0.4% baseline; H-VPPUT not yet measured)
**Substrate path**: 65% (per LATEST.md prior; git2-rs CAS + L4 commits wired; HEAD_t path abstraction + Art 0.4 rtool/wtool trio missing; Path A/B/C election deferred)
**Economic mechanism (§ 21 final reward)**: 10% computable (Constitution gates ✅; Utility partial; Escrow/Accept/Attribution/Survival all schema-only stubs)

**ChainTape end-to-end Verify-tx flow**: stalls at step 3 (sequencer dispatch returns NotImplementedError; CO1.7.5 transition bodies deferred). Steps 1-2 (proposal, predicate verdict) and 6-8 (ledger commit, CAS index, system signature) work; steps 3-5 (state mutation, materializer, signal broadcast) deferred.

**Top 3 gaps if pursuing substrate-path capability** (8-12 days estimate from agent — but **B already proved capability via pre-v4 evaluator path so this is FUTURE work, not blocking**):
1. CO1.8 v2 spec rework (3-5 days)
2. Evaluator → v4 ledger wiring (1-2 days)
3. L6 signal indices (2-3 days)

### Constraint hierarchy (post-B-success update)

1. **Constitution**
2. **Whitepaper v2**
3. **24h iteration cap** ← validated this session (pivot decision → first solve in 80 min)
4. **Standing memories** (with re-scoped dual-audit + phased-checkpoint)

### Outstanding follow-ups (priority order)

1. **`oneshot` regression bug** — file as ≤1-day atom; identify prompt-template/parser divergence
2. **Solve breadth check** — re-run n3 + MAX_TX=50 against 5-10 more adaptation problems for solve-rate estimate
3. **CO1.7-impl A5+ continuation** (real implementation work; not new spec)
4. **CO1.7.5 spec draft** (when started: single-round audit, accept-or-defer-with-OBS per session-3 policy)
5. **CO1.8 v2 spec** (deferred until CO1.7.5 lands; per OBS doc)
6. **AUTO_RESEARCH_NOTEPAD.md cleanup** (TFR stale ref; bloat ≤ 200 lines target)
7. **LATEST.md compression** (target ≤ 100 lines; after pivot stabilizes)

### Session-3 commits (chronological)

| # | Commit | Action |
|---|---|---|
| 1 | `a906886` | Session-3 pivot codification: OBS_CO1_8_V1_DEFERRED + iteration-cap memory + LATEST.md session-3 + Codex/Gemini r1 audit MDs |
| 2 | (this commit) | First v4-native solve evidence: handover/evidence/first_v4_solve_2026-04-29/ + LATEST.md session-3 update with B result + landing eval integration |

### Original 🚀 Next-session entry point (B was the gate; B is now done)

~~**B: run v4 evaluator on `mathd_algebra_107` (HEAD) by 2026-05-06.**~~ ✅ done in 80 min, not 1 week.

**New next-session entry point**:
1. Diagnose + fix `oneshot` regression bug (atom)
2. Run n3 batch on 5-10 adaptation problems for solve-rate baseline
3. Decide whether to resume substrate work (CO1.7.5/CO1.8) or expand capability batch first

**Do NOT** restart spec-atom mass production. Capability path is now the default; substrate work earns its way back via concrete capability-loop progress (per `feedback_iteration_cap_24h` memory).

### Hard data that triggered the pivot (2026-04-22 → 2026-04-29, 7 days post-TRACE_MATRIX_v0 baseline)

| Metric | Value | Signal |
|---|---:|---|
| Total commits | 203 | |
| spec/audit | **95 (47%)** | |
| impl/test | 24 (12%) | |
| eval/experiment | **13 (6%)** | |
| Audit reports total LoC | **367,555** | single audit MD ~150KB |
| Production LoC (`src/*.rs`) | 11,701 | |
| **Audit:Production ratio** | **31.4 : 1** | smoking gun |
| v4-native new solves since 2026-04-22 | **0** | proofs/ are inherited pre-v4 (untracked) |
| Last batch experiment artifact | 2026-04-24 E1v2 | used pre-v4 evaluator (build SHA `29ab43a`) |
| 5-step compile loop wired | 3/5 | steps 4+5 (Capability Compilation, ↑H-VPPUT) deferred to v4.1 |
| H-VPPUT empirical measurements | **0** | formula defined, never measured |

### Web research evidence (full sources in session-3 transcript)

- DeepSeek-Prover-V2 (88.9% MiniF2F SOTA): **2 public commits**, prototype-first
- Goedel-Prover: 24 commits / 64 days; Kimina-Prover: 12 / 87 days. **Zero** peer LLM-prover team uses atom-spec + per-atom dual-LLM-audit
- Porter & Votta (TSE 1997) + Jureczko 2020: **2 reviewers is empirical optimum**; rounds-per-change beyond 2 mostly surface paper tigers
- TDD/spec-first **explicitly discouraged** for exploratory ML/research code (Manning ML Eng, CMU MLIP)
- Atomic-decomp + dual-audit DOES work in DO-178C avionics + seL4 microkernel — **decade timelines, life-stakes**. Not solo LLM research

### Pivot decisions (executed this session)

**A. Stopped spec-craft loop**
- **CO1.8 v1 DEFERRED**, not patched. r1 verdict: **Codex VETO/HIGH + Gemini CHALLENGE/HIGH** (conservative merge = VETO). Real architectural P0s found:
  - Codex P0 #1: sprint graph overclaim — `[CO1.7.5] blocks: CO1.8` per SPRINT line 106-108; CO1.8 not unblocked by CO1.7-extra alone
  - Codex P0 #2: `apply(prior_root: &Hash, tx: &TypedTx) -> Result<Hash, _>` interface contradiction — VerifyTx has only target+verifier, can't increment reputation without prior Work/Claim state. "Pure function with implicit BTreeMap I/O" is internally inconsistent
  - Gemini P0: `project_for_agent` no-op stub violates Inv 10 (Goodhart shield) by default-allow
- All findings archived to `handover/alignment/OBS_CO1_8_V1_DEFERRED_2026-04-29.md`. CO1.8 spec header updated with 🛑 DEFERRED status. **NO r2 audit run.** Original v1 text preserved as evidence.
- **CO1.13-extra (250 backlinks; ~10-15 hr) downgraded** from "MUST before Phase D" to "v4.1 gate" — Phase D is itself v4.1 scope per PROJECT_DECISION_MAP D4
- 1.7-impl + future spec atoms switch from per-atom dual-audit-with-rounds → **single audit round, accept-or-defer-with-OBS**, no r2/r3

**C. New iteration-cap policy** (memory entry `feedback_iteration_cap_24h.md`)
- Every PR must produce evaluator pass/fail signal (smoke or single-problem real run) within 24h
- Spec/audit/scaffold work that doesn't shortest-path to runnable feedback loop = **default-reject** unless explicit user authorization
- Replaces atom-only Elon-mode round-cap framing for non-spec work
- Dual-audit + phased-checkpoint + smoke-before-batch memories still apply, but NOT as default for every change — only when capability loop is actively producing solves
- Red flags: 3+ days without evaluator signal, 2+ days without test, "round 3+" being proposed, audit:prod LoC ratio growing weekly

**B. Capability-first execution begins**
- Target: `mathd_algebra_107` (adaptation split; pre-solved 8+ times in inherited `proofs/`; medium difficulty; regression-test-as-first-solve)
- Constraint: Mathlib rebuild must clean first (currently 99%, ~20 min)
- Mode: `--mode full` (baseline, no ablation), `CONDITION=oneshot`, `ACTIVE_MODEL=deepseek-chat`
- Wall-clock budget: 24h iteration cap; if not solved in 24h, debug to specific blocker, raise to user
- Deadline: **2026-05-06** for either first-solve confirmation OR documented infrastructure gap

**D. Audit sunk-cost recovery (CO1.8 r1)**
- Codex r1 (174s, $5-10): VETO/HIGH, 2 P0s — both real architectural defects
- Gemini r1 (40s, $3-5): CHALLENGE/HIGH, 1 P0 — Goodhart shield (real)
- **0 paper tigers in r1** — audit was efficient, $10-15 well-spent
- Pivot lesson: r1 earned its keep; r2/r3 would have entered diminishing returns. The system's working at 1 round; we just stop overspending

### Updated constraint hierarchy (effective session-3)

1. **Constitution** (constitution.md)
2. **Whitepaper v2** (load-bearing for ChainTape + economic mechanism)
3. **24h iteration cap** (NEW; replaces atom-only Elon-mode framing)
4. **Standing memories** — but with `dual_audit` + `phased_checkpoint` re-scoped to "active capability loop" only, not "every spec change"

### Outstanding follow-ups (post-pivot priority order)

1. **B: mathd_algebra_107 first solve attempt** (in flight; gated on Mathlib)
2. **CO1.7-impl A5+ continuation** (real implementation work; not new spec)
3. **CO1.7.5 spec draft** (when started: single-round audit, accept-or-defer-with-OBS)
4. **CO1.8 v2 spec** (deferred until CO1.7.5 lands; per OBS doc)
5. **AUTO_RESEARCH_NOTEPAD.md cleanup** (TFR stale ref; bloat ≤ 200 lines target)
6. **LATEST.md compression** (target ≤ 100 lines; after pivot stabilizes)

### Session-3 commits (chronological)

| # | Commit | Action |
|---|---|---|
| pending | (this commit) | Session-3 pivot codification: OBS_CO1_8_V1_DEFERRED + CO1.8 spec status update + iteration_cap memory + LATEST.md session-3 entry |

### CO1.8 r1 audit residue

- `handover/audits/CODEX_CO1_8_ROUND1_AUDIT_2026-04-29.md` (362KB; VETO/HIGH; 2 P0s)
- `handover/audits/GEMINI_CO1_8_ROUND1_AUDIT_2026-04-29.md` (5.8KB; CHALLENGE/HIGH; 1 P0; gemini-3.1-pro-preview after stale-model fix to launcher)
- `handover/audits/run_gemini_co1_8_round1_audit.py`: model id patched from `gemini-2.0-flash-thinking-exp-01-21` → `gemini-3.1-pro-preview` (drift fix; same as CO1.13 r1/r2 working launchers)

---

## 🎯 2026-04-29 Session-2 CLOSURE — CO1.13 atom bundle COMPLETE ✅

**Status**: CO1.13.1 + CO1.13.2 + CO1.13.3 all shipped + drift review = NO MATERIAL DRIFT. Wave 6 #2 PRE-CO1.8 alignment factory now LIVE.
**HEAD commit**: `1a5849f` (CO1.13 phase drift review + --half factory upgrade).
**Origin**: through `5829e32` pushed; rest local-only.

### 🚀 Next-session entry point

**Pick up at one of two priorities** (user direction required):

1. **CO1.8 spec round-1 audit launch** — spec drafted at `6cc5cc9`; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py`; not yet run. CO1.13 factory is now LIVE so audits will benefit from R-022 + § F.2 auto-refresh + § J orphan registry + the `--half` Phase C regression check.
2. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks) — MUST schedule before Phase D per spec § 0.5 Gemini r1 Q7. With R-022 LIVE, every NEW pub symbol since `e9c6a2b` is enforced; legacy gap is the remaining substantive debt.

### Three commits this CO1.13 closure arc

| # | Commit | Action |
|---|---|---|
| 1 | `9be22b4` | CO1.13.1 — TRACE_MATRIX_v3 doc completion (§ E.2/E.3 measured stats; § F.2 manual snapshot 135 backlinks; § J Orphan Extensions schema; cross-ref reconciliation). +283 / -14 doc delta. Trust Root rehash for TRACE_MATRIX_v3. |
| 2 | `e9c6a2b` | CO1.13.2 + CO1.13.3 — R-022 hook (rules YAML + custom_commit_hook check_trace_matrix.py 421 LoC + tracked pre-commit shim + install_hooks.sh + .github/workflows/co1_13_r022_ci.yml + 5-line engine.py patch + 9 shell integration tests + Rust orchestrator) + auto-refreshing § F.2 reverse-map (update_trace_matrix_reverse_map.py 134 LoC; shares parser with R-022 check). +1011 / -31. Trust Root rehash for engine.py + TRACE_MATRIX_v3. |
| 3 | `1a5849f` | CO1.13 phase drift review (`handover/architect-insights/CO1_13_PHASE_DRIFT_REVIEW_2026-04-29.md` 215 LoC) + `--half` factory upgrade to `run_c2_phase_c_ablation.sh` (3 problems × 5 modes × 1 seed × MAX_TX=20; lives between cheap `--smoke` and full Phase C batch). Trust Root rehash for runner script. |

### CO1.13 final spec compliance (vs v1.1.1 § 0.3)

| Sub-atom | Spec target LoC | Actual LoC | Verdict |
|---|---:|---:|---|
| CO1.13.1 | ~200 | +283 / -14 | ACCEPTABLE (table content + § J schema; quality spending) |
| CO1.13.2 | ~335 | ~676 (script 421 + yaml 20 + shim 13 + installer 31 + ci 24 + 5-line engine.py + tests 297) | ACCEPTABLE (test-isolation hardening forced by real pollution incident) |
| CO1.13.3 | ~100 | 134 | ACCEPTABLE (--check / --dry-run modes added) |
| Bundle total | ~635 | +1011 / -31 net | ACCEPTABLE per Elon-mode "scope unchanged, process streamlined" |

### Real-test data points (5)

1. **Test pollution** — `r_022_ci_mode_catches_unhooked_pr.sh` initially leaked an empty `b60556d main baseline` commit + `feature` branch into the live repo because `tmp=$(setup_temp_repo)` ran `cd` in a subshell; `set -uo pipefail` (no `-e`) was silent on the failure. **Fixed**: introduced `enter_tmp_repo` (no subshell; sets TMP_DIR global; asserts `realpath $PWD` does NOT resolve inside PROJECT_ROOT before any git command). All 9 tests re-run without pollution.
2. **Disk-space exhaustion** — `cargo test --test r_022_integration_orchestrator` triggered `ld: signal 7 (Bus error)` during link; bash subprocess infrastructure entered degraded state (every command returned non-zero with empty stdout/stderr; Write tool reported ENOSPC). User manually freed ~12G of cargo `target/`. Future drift reviews should `df -h` before launching `cargo test --workspace`.
3. **CO1.13.3 idempotency** — `python3 scripts/update_trace_matrix_reverse_map.py --check` exits 0 immediately after first run.
4. **Phase C smoke 5/5 PASS in 95s** post-CO1.13 (consistent with 97s baseline at `8d88f2d`); soft_law H2 fake-accept signature preserved. Per user 2026-04-29 challenge: `--smoke` is pipeline-liveness only — for CO1.13 (0 lines of `src/` changed) it confirms only that Trust Root rehashes didn't break evaluator boot.
5. **Mathlib collateral damage** — disk-cleanup recommendation (`rm -rf .lake`) was too aggressive: `.lake/packages/Mathlib/` is a vendored dependency requiring `lake exe cache get` (~2 min) or `lake build` (30-60 min) to recover. Lake project skeleton (`lakefile.lean` / `lake-manifest.json` / `lean-toolchain`) preserved; recovery via `lake update && lake exe cache get` running in background at session-closure time. **New memory entry**: `feedback_lake_packages_vendored` codifies the `.lake/build` (regen) vs `.lake/packages` (vendored) distinction.

### `--half` factory upgrade landed in this session

User direction "1+2 结合，2 等大节点再做" → added `--half` mode to `handover/preregistration/scripts/run_c2_phase_c_ablation.sh`: 3 problems × 5 modes × 1 seed × MAX_TRANSACTIONS=20 (~10-15 min wall-clock; ~$0.20-0.40 API cost). Lives between `--smoke` (pipeline-liveness; ~95s) and `--full` (scientific regression; ~12 hr; 100 cells). First invocation surfaced data point #5 above; needs Mathlib recovery before next use.

### Outstanding follow-ups (priority order)

1. **CO1.8 spec round-1 audit launch** — drafted at `6cc5cc9`; ready under new factory regime
2. **Mathlib recovery** — running in background via `lake update && lake exe cache get`; ETA ~5-10 min from session-2 CLOSURE start
3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 backlinks; MUST before Phase D per Gemini r1 Q7)
4. **CO1.13-devtools-mathlib-mirror** (new follow-up sub-atom; this session): file-mirror endpoint on linux1 hosting Mathlib v4.24.0 `.lake/packages` tarball; omega-vm hydration script; Trust Root sha256 registration. Constitutionally clean (Lean stays local). Estimated ~1-2 day work; collapses future Mathlib re-fetch from 10-30 min to ~5 min internal-network rsync. Defer to between CO1.8 and CO1.9 atoms.
5. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; per spec § 0.4) — non-spec; lands as separate commit
6. **AUTO_RESEARCH_NOTEPAD.md cleanup** — TFR stale reference per LATEST.md session-2 outstanding-debt; defer to next session
7. **CO1.7.5** (transition bodies; gated on CO P2.x substrate atoms) — Wave 2 work; weeks-to-months out

### New Constitutionally-clean Mathlib mirror architecture (CO1.13-devtools-mathlib-mirror; this session candidate spec)

**Why**: Today's disk-cleanup → Mathlib loss → 10+ min recovery debt is preventable. linux1-lx (128G AMD AI Max 395, primary compute node) is the natural Mathlib source-of-truth.
**What**: tarball `.lake/packages` ~5G on linux1 → exposed via internal HTTPS (or even simpler: via existing WireGuard rsync access) → omega-vm hydrate-on-provision script.
**Constitutionally clean**: Lean still runs locally on omega-vm (Art 0.2 oracle locality unchanged); network only used for one-time provisioning hydration.
**Trust Root**: tarball sha256 registered in `genesis_payload.toml`; FC3-N34 verification on hydrate.
**NOT**: a network verifier API (option B in 2026-04-29 user discussion) — that would change Art 0.2 oracle locality + raise sudo gate.

### Sedimented memory entries this session

- `feedback_lake_packages_vendored` (NEW; .lake/build vs .lake/packages distinction)
- (existing memories unchanged: `feedback_oracle_preflight`, `project_phase_c_living_regression`, `feedback_elon_mode_policy`, `feedback_no_fake_menus` all reaffirmed by this session's events)

### Cumulative project audit spend after CO1.13 closure

- This session's CO1.13 r1+r2 dual audits + cap-exception: ~$16-24 (per drift review § 7)
- Project cumulative: ~$220-340 / $890 mid-budget (~25-38%); ~$550-670 runway
- Per atom going forward: $5-10 expected (single-round + targeted patches; R-022 + auto-refresh + § J registry now amortize the spec-cycle prep cost)

### Constraint hierarchy (active per Elon-mode + user 2026-04-29 explicit instruction)

User explicit instruction 2026-04-29 session-2:
> "我要求你在遵守宪法、白皮书和我们刚才讨论的elon-mode下自动执行..."

Operationalized priority order:
1. Constitution
2. Whitepaper v2
3. Elon-mode (round cap=2, OBS threshold=3, cap-exception via auto-execute on determinate-best surgical patch)
4. Standing memories (dual-audit, smoke-before-batch, no-fake-menus, FC-first, NEW lake-packages-vendored)

When facing decision: 1→2→3→4 order; if no resolution → state determinate-best + execute (no fake menus). Per-phase drift review at atom-complete boundary. When lacking data: run real tests, don't speculate.

---

## 🌊 2026-04-29 Session-2 — CO1.7-extra Branch B closure + CO1.13 spec PASS-with-cap-exception (Elon-mode launch)

**Updated**: 2026-04-29 (session-2)
**Status**: spec phase **DONE** (CO1.7-extra ceremony closed + CO1.13 cleared for impl); implementation phase **READY TO START** in fresh session.

### 🚀 Next-session entry point

**Pick up at CO1.13 implementation phase per spec § 0.3 v1.1.1**. Three sub-atoms in dependency order:

1. **CO1.13.1** TRACE_MATRIX_v3 doc completion (~200 LoC docs delta; 0.5 day target)
   - § A complete N-rows; § B complete WP rows; § E coverage stats
   - § F reverse-map populated for shipped atoms (CO1.0a / CO1.4 / CO1.4-extra / CO1.7-impl A1-A4 / CO1.7-extra)
   - **NEW § J "Orphan Extensions"** with table schema (lands BEFORE script can fall back to it)
2. **CO1.13.2** R-022 commit-time hook (~335 LoC; 1.5 day target)
   - `rules/active/R-022_trace_matrix_pub_symbol_block.yaml` (declarative tombstone; engine.py BYPASSED)
   - `scripts/check_trace_matrix.py` (multi-line context grep + diff parser)
   - `scripts/hooks/pre-commit.r022` (tracked shim)
   - `scripts/install_hooks.sh` (symlinks tracked shim → `.git/hooks/pre-commit`)
   - **`.github/workflows/co1_13_r022_ci.yml`** (tracked CI workflow; required merge gate; closes Codex r2 fresh-clone bypass)
   - 5-line patch to `rules/engine.py` (gracefully ignore `trigger == pre_commit`)
3. **CO1.13.3** reverse-map § F populator (~100 LoC Python; 0.5 day target)
   - `scripts/update_trace_matrix_reverse_map.py` shares parser with CO1.13.2 (per Codex r1 § D "one parser shared")

Plus 9 shell integration tests under `tests/integration/co1_13/` + 1 Rust orchestrator (`tests/r_022_integration_orchestrator.rs`) per spec § 3 v1.1.

**Authoritative spec**: `handover/specs/CO1_13_TRACE_MATRIX_IMPL_v1_2026-04-29.md` v1.1.1 (commit `813414c`). Read § 0.3 + § 1.2 + § 1.3 + § 2.1 + § 3 first; § 8 acknowledgements before coding.

**Total target**: ~665 LoC; **3-day wall-clock target** (Elon-mode benchmark; first real-test of cycle-time hypothesis).

**Phase drift review** fires at impl complete (per session task #7); 7-dimension check (scope / process / constraint / doc / critical-path / cycle-time / budget). Pre-flagged drift to confirm:
- Scope drift: +60% LoC v1→v1.1.1 (audit-driven; acceptable)
- Process drift: 3 audit rounds vs 2-round-cap (cap-exception per Codex r2 § E own recommendation; acceptable)
- Constitution + WP alignment: STRENGTHENED (R-022 enforcement now actually works via tracked CI)

### Session arc (3 commits this session-2)

| # | Commit | Action |
|---|---|---|
| 0 | `4a978f0` | CO1.7-extra v1.2.2: STEP_B Branch B re-derivation closed at T1 executable-substance byte-identity (per amended § 2.2 tiered byte-identity). Ceremony CLOSED for `src/bus.rs`. STATE_TRANSITION_SPEC v1.5 housekeeping issue committed earlier (`5b53c6b`). |
| 1 | `6cc5cc9` | CO1.8 L5 Materializer v1 spec drafted (300 lines, 10/10 smoke). **AUDIT DEFERRED** in favor of CO1.13 per Elon-mode ROI analysis (factory amortization 20-50x over 150+ remaining atoms). |
| 2 | `8d88f2d` → `1423b90` → `813414c` | CO1.13 v1 → v1.1 (r1 9 patches) → v1.1.1 (r2 cap-exception 4 patches; Codex CHALLENGE-ESCALATE / Gemini PASS; conservative CHALLENGE-ESCALATE → cap-exception per Codex r2 § E recommendation). Spec at 420 lines; PASS-with-cap-exception. |

### NEW Elon-mode policy framework codified this session

The user authorized "Elon-mode" framing for project management (factory > scope; cycle-time > round-count; constitution + whitepaper line-by-line preserved as scope, but PROCESS streamlined). Round-1 audit on CO1.13 v1 forced the policy to be CONCRETE rather than aspirational. v1.1.1 codified:

1. **Audit round cap = 2** (vs prior 4-5 rounds): r1 + 1 patch round + r2 final. Round-3+ requires cap-exception authorization.
2. **OBS hard threshold = max 3 unresolved `OBS_*.md` files** project-wide (Gemini r1 Q4): threshold breach = factory halt + force-resolve before next atom. Prevents 2-round-cap from accumulating debt.
3. **Ship-with-OBS NOT applicable to enforcement gates themselves** (Codex r1 § E): "If round 2 still has non-enforcing R-022, do not ship-with-OBS; that would convert a hard alignment gate into theater." → escalate to user.
4. **Cap-exception authorized via auto-execute mode** when r2 split verdict produces a determinate-best surgical patch (not OBS theater). Codex r2 itself recommended this for v1.1.1.
5. **Phase C smoke as living regression test** (parallel weekly): verifies architecture-in-progress hasn't broken experiment harness. First run THIS session: 5/5 cells PASS @ HEAD `8d88f2d` in 97s vs 146s baseline (33% faster); soft_law H2 ablation signal preserved. **No regression**.

Memory entries created (see MEMORY.md):
- `feedback_no_fake_menus.md` — when project plan determines next atom, state and execute; don't surface 3-5 option menus
- `feedback_elon_mode_policy.md` — round cap + OBS threshold + cap-exception conditions (this session)
- `project_phase_c_living_regression.md` — Phase C smoke as architecture-in-progress regression check (this session)

### Constraint hierarchy (auto-execute mode interpretation)

User explicit instruction 2026-04-29 session-2:
> "我要求你在遵守宪法、白皮书和我们刚才讨论的elon-mode下自动执行，遇到选择题先检查以上约束，每个phase完成后对项目计划做review看drift，缺少做决策人来的数据就去跑真是测试找问题和解决方案"

Operationalized as priority order:
1. **Constitution** (constitution.md; load-bearing for thesis)
2. **Whitepaper v2** (load-bearing for ChainTape + Anti-Oreo + economic mechanism coverage)
3. **Elon-mode** (round cap, OBS threshold, factory > scope, cycle-time > round-count)
4. **Standing memories** (dual-audit, smoke-before-batch, no-fake-menu, FC-first-problem-handling, etc.)

When facing a decision: check 1→2→3→4 in order; if no resolution → state determinate-best action + execute (no fake menus). Per-phase drift review at atom-complete boundary. When lacking data: run real tests (Phase C smoke, cargo test, empirical measurements) — don't speculate.

### Real-test data points produced this session

| Test | Result | Significance |
|---|---|---|
| Phase C smoke @ HEAD `8d88f2d` | 5/5 cells PASS in 97s; soft_law H2 ablation preserved | architecture-in-progress hasn't broken experiment harness; **freeze rationale ("Node.completion_tokens=0 discovery; TFR S3.9 5-7 weeks out") is STALE** — TFR v1 was deprecated 2026-04-26 (see TFR_MASTER_PLAN_2026-04-26.md preface) and Phase C smoke was already 5/5 PASS @ 146s on 2026-04-28. Phase C is operationally unfreezable on demand. |
| CO1.13 spec-cycle wall-clock | ~2.5 hr (vs 14-day median pre-Elon-mode = ~134x compression on spec phase) | first real-test of Elon-mode "factory IS product" hypothesis; spec phase validated; impl phase pending |
| Backlink coverage baseline | 87/354 = 24.6% | 75% legacy gap quantified; CO1.13-extra (gap closure) MUST schedule before Phase D per Gemini r1 Q7 |

### Cumulative project audit spend after CO1.13 v1.1.1

- This session r1+r2 dual audits (4 calls): ~$16-24
- Project cumulative: ~$220-340 / $890 mid-budget (~25-38%); ~$550-670 runway
- Per atom going forward (post-CO1.13 factory deployed): expected $5-10 (single round + targeted patches; CO1.13's R-022 + scaffold devtools amortize spec-cycle prep cost)

### Open follow-ups (priority order)

1. **CO1.13 implementation** (next-session entry; this is THE priority)
2. **CO1.8 spec round-1 audit** (deferred this session; spec drafted at `6cc5cc9` ready to launch; launchers exist at `handover/audits/run_{codex,gemini}_co1_8_round1_audit.sh|py` but were NOT run)
3. **CO1.13-extra** (legacy backlink closure; ~10-15 hr; ~250 missing backlinks; MUST schedule before Phase D per Gemini r1 Q7)
4. **CO1.13-devtools** (scaffold scripts + Trust Root rehash automation; non-spec follow-up; lands after CO1.13 PASS impl)
5. **Phase C unfreeze decision**: smoke is now consistently passing; should we relaunch C2 full batch (5 modes × 10 problems × 2 seeds = 100 cells; ~12 hr wall-clock; ~$15-25)? **User decision required**.
6. **CO1.7.5 future spec** (transition bodies; gated on CO P2.x substrate atoms — Wave 2 work; ~50 atoms 6-8 wk)
7. **CO P2.x family roadmap** (TaskMarket / EscrowVault / ContributionLedger / etc.; per user requirement "宪法和白皮书逐行落地，包括但不限于经济制度")

### Outstanding architectural debt acknowledged

- **TFR v1 deprecated** at its own launch day (2026-04-26 night) per CO_P0_AMENDMENT_v1; successor is `CO_MEGA_PLAN_v3.1_2026-04-26.md`. AUTO_RESEARCH_NOTEPAD line 66 still describes TFR as "🚀 LAUNCHED" — STALE; needs cleanup but defer to next session.
- **AUTO_RESEARCH_NOTEPAD bloat**: ~600 lines; per Elon-mode "delete process redundancy", target ≤ 200 lines. Defer to next session.
- **LATEST.md bloat**: ~600+ lines; per Elon-mode, target ≤ 100 lines. Defer to next session.

These are bookkeeping items; no constitutional or scientific impact.

---

## 🌊 2026-04-29 Session-1 — Wave 6 #1 RECALIBRATION (CO1.7.5 split → CO1.7-extra; Branch A landed)

**Updated**: 2026-04-29
**Session arc**: dual-audit drove a **scope correction** on the prior 2026-04-28 "80% complete" framing. Round-1 dual external audit on CO1.7.5 v1 (Codex+Gemini, both CHALLENGE/High) found that D1 transition bodies have heavyweight FC1 (top-white predicate execution) + FC2 (middle-black state schemas) substrate dependencies that don't exist in shipped code (CO P2.x family per `PROJECT_DECISION_MAP § 3.4`). ArchitectAI applied an Occam-driven scope split (B2 by dependency profile) under "无损压缩即智能 + Anti-Oreo + 不违宪 + 不违白皮书" principles, yielding two atoms:

| Atom | Owns | Substrate dep | Status |
|---|---|---|---|
| **CO1.7-extra** (NEW bridge atom; CO1.4-extra precedent) | D2 head_t close + D3 TuringBus single-file STEP_B + 5 substrate-independent tests | None | ✅ spec PASS/PASS r4 + v1.2.2 § 2.2 amendment; **Branch A landed** `5ce01b1`; **Branch B closed** at T1 byte-identity (separate session 2026-04-29; tiered byte-identity per spec § 2.2 v1.2.2) — **STEP_B ceremony CLOSED** |
| **CO1.7.5** (restored to CO1.7 § 13 original meaning) | D1 transition bodies (7) + 3 D4 tests + un-ignore replay byte-identity | CO P2.1 / 2.2 / 2.3 / 2.5 / 2.6 / 2.7 / 2.9 + CO1.11 + (NEW) PredicateRegistry execution-methods atom | 📅 GATED on substrate atoms |

### Wave 6 #1 actual progress: ~30-40% (NOT 80%)

The prior 2026-04-28 "80% complete" claim was **false-precision** based on a mis-scoped atom (D1 substrate dependencies hidden inside CO1.7.5 v1 bundle). True state at HEAD `5ce01b1`:

- ✅ CO1.7 spec + CO1.7-impl A1-A4 bundle + CO1.4-extra (prior session)
- ✅ CO1.7-extra spec PASS/PASS (4 rounds; this session)
- ✅ CO1.7-extra Branch A landed (D2 head_t close + D3 TuringBus wiring + 5 tests)
- ✅ CO1.7-extra Branch B closed (T1 executable-substance byte-identical; spec § 2.2 amended v1.2.2 to formalize 3-tier byte-identity rule for future STEP_B atoms)
- 📅 CO1.7.5 gated on Wave-2 substrate (~7 prerequisite atoms + 1 NEW PredicateRegistry exec atom)

ChainTape vertical: L4 ~50-55% (storage + ABI + machinery + head_t close + Sequencer entry-point; transition bodies still pending). Estimate "Wave 6 #1 fully closed" = **after CO P2.x substrate ships** (multiple atoms; weeks-to-months out).

### CO1.7-extra audit arc (4 rounds)

| Round | Codex | Gemini | Conservative | Action |
|---|---|---|---|---|
| r1 (bundled CO1.7.5 v1) | CHALLENGE/H | CHALLENGE/H | CHALLENGE | Occam scope split → CO1.7-extra carved out |
| r2 (CO1.7-extra v1) | CHALLENGE/H | CHALLENGE/H | CHALLENGE | 10 MFs (MF1-MF10) → v1.1 |
| r3 (v1.1) | CHALLENGE/H | PASS/H | CHALLENGE | 4 mechanical (B1-B4) → v1.2 |
| r4 (v1.2) | **PASS/H** | **PASS/H** | ✅ **PASS/PASS** | 2 nits (N1+N2) → v1.2.1 (final) + Branch A impl |

CO1.7-extra atom-only audit cost: ~$13-26 across r2+r3+r4. Cumulative project: ~$196-314 / $890 mid-budget (~22-35%).

### Architectural improvements landed (vs prior bundled v1)

1. **TuringBus owns Sequencer directly** (round-2 MF4) — Kernel UNTOUCHED; "pure topology" doctrine preserved. STEP_B reduced from combined-ceremony to single-file (bus.rs only).
2. **Required trait method** (round-2 MF3) — `LedgerWriter::head_commit_oid_hex` has no default impl; Rust compiler enforces every implementation declares. Both audits' safety arguments (silent stagnation prevention + no-panic) satisfied via this third-option synthesis.
3. **`advance_head_t` helper extraction** (round-2 MF2) — D2 logic at module level + apply_one stage 9 calls helper; makes the constitutional anchor advance directly testable via mock writer (without injecting dispatch_transition).
4. **Kernel "pure topology" doctrine preserved** — no new fields on Kernel; runtime drivers (Sequencer + future) live at TuringBus level.

### Sedimented OBS files (2 new this session)

- `OBS_STEP_B_RESTRICTED_FILE_LIST_DRIFT_2026-04-29.md` — CLAUDE.md + STEP_B_PROTOCOL.md path drift (`src/wallet.rs` → `src/sdk/tools/wallet.rs`); fixed inline + sediment.

### Pending follow-ups

1. ✅ ~~CO1.7-extra Branch B~~ — closed 2026-04-29 separate session at T1 byte-identity per spec § 2.2 v1.2.2 amendment.
2. ✅ ~~STATE_TRANSITION_SPEC v1.5 housekeeping issue~~ — committed `5b53c6b` per CO1.7-extra spec § 0.4 commitment.
3. **Future CO1.7.5 spec drafting** — gated on CO P2.x substrate atoms reaching individual PASS/PASS.
4. **Wave 6 #2 next-atom selection** — Wave 6 #1 (CO1.7 family) ceremony-closed; § 3.2 menu of unblocked atoms includes CO1.8 L5 materializer / CO1.9 L6 signal indices / CO1.10 signal dichotomy / CO1.11 safety vs creation / CO1.13 TRACE_MATRIX impl. Pending user direction on which Wave 6 #2 atom to spec next.

### Open Questions

- **Q1 (sequencing)**: with Wave 6 #1 substrate now exposed as critical path, should the project reorder to ship CO P2.1/2.2/2.3/2.5/2.6/2.7/2.9 + CO1.11 before resuming CO1.7.5? Or continue Wave 6 #2/#3 affordances (CO1.8/CO1.9) in parallel?
- **Q2 (PROJECT_DECISION_MAP)**: should CO1.7-extra be codified into the decision map alongside CO1.4-extra precedent (this session's bridge-atom landing pattern)?

---

## 🌊 2026-04-28 Session-2 Final — Wave 6 #1 IMPLEMENTATION PHASE COMPLETE ✅

**Updated**: 2026-04-28 14:12 UTC
**Session summary**: Auto-execute mode shipped CO1.1.4-pre1 ABI atom (PASS/PASS) + CO1.7-impl A1+A2+A3+A4 bundle (PASS/PASS-equivalent) + CO1.4-extra in one continuous run. 17 commits pushed. 199/0 → 239/0 lib PASS + 1 ignored (CO1.7.5-stage). Audit spend ~$40-75. Single carry-forward: G-1 head_t Art 0.4 alignment closes in CO1.7.5.

### Current State

**Wave 6 #1 (L4 Transition Ledger family) — 80% complete**:
- ✅ CO1.7 spec PASS/PASS (3 rounds, prior session, ~$25-42)
- ✅ **CO1.1.4-pre1 v1.2.2 ABI surface PASS/PASS** (5 rounds, ~$26-50; commit `c1226e2`) — 7-variant TypedTx + 6 SigningPayload + 13 locked golden hex + ClaimId + 22-variant TransitionError
- ✅ **CO1.7-impl A1+A2+A3+A4 bundle PASS/PASS-equivalent** (3 rounds, ~$14-25; commit `2461fe6`) — Git2LedgerWriter + Sequencer + dispatch_transition stubs + replay_full_transition (9-stage I-DETHASH witness with tx_kind + decode separation)
- ✅ **CO1.4-extra** sidecar JSONL CAS index persistence (commit `b6b7574`) — closes Art 0.2 cold-replay gate
- 📅 **CO1.7.5** (per-kind transition bodies + STEP_B bus.rs/kernel.rs wiring) — final L4 atom, NOT STARTED

**ChainTape vertical position**:
- L0 Trust Anchor ✅ / L1 PredicateRegistry ✅ / L2 ToolRegistry ✅ / L3 CAS ✅ (incl. cold-replay) / L4 ⏳ 80% (storage + ABI + machinery done; transition bodies pending) / L5 📅 NOT STARTED / L6 📅 NOT STARTED

**Cumulative project audit spend**: ~$175-273 / $890 mid-budget (~20-31%).

### Next Steps

1. **CO1.7.5** (single critical path) — final L4 atom. Inherits frozen ABI + Sequencer machinery; must deliver:
   - Real per-kind transition bodies for 7 TypedTx variants (currently `Err(NotYetImplemented)` stubs)
   - Close G-1 head_t Art 0.4: wire `q.head_t = NodeId(commit_oid_hex)` after Git2LedgerWriter.commit (`head_commit_oid()` already exposed)
   - STEP_B parallel-branch ceremony for bus.rs/kernel.rs wiring (per CLAUDE.md "Code Standard")
   - Remove `#[ignore]` from `sequencer_serial_replay_byte_identity` test; verify end-to-end state_root reconstruction
   - Estimated: ~5-9 days; ~$25-50 audit
2. **Then** Wave 6 #2/#3 unblocks (CO1.8 L5 materializer + CO1.9 L6 signal indices)
3. **PPUT-CCL Phase C unfreeze** at TFR S3.9 — still ~5-7 weeks out

### Open Questions

- **Q1 (architectural drift)**: TFR_MASTER_PLAN_2026-04-26 uses old paths (`src/tape/`, `src/wal.rs`, `src/ledger.rs`); actual work is under `src/bottom_white/ledger/` + `src/state/` per Anti-Oreo restoration. Worth a one-line "SUPERSEDED by Wave 6 framing" header, or leave as historical artifact?
- **Q2 (process)**: 7 sedimented lessons across CO1.1.4-pre1 + CO1.7-impl bundle audits (esp. "claim-vs-code parity drift recurs" — caught 2× this session). Should pre-audit grep be codified into `validate` skill, or stay informal habit?
- **Q3 (next-session entry)**: CO1.7.5 directly, or pause for handover review first?
- **Q4 (head_t closure binding)**: G-1 deferred to CO1.7.5 per spec K3 v1.2 + Gemini bundle r1 #1 carry-forward. Both bound to that atom — but if CO1.7.5 slips, head_t Art 0.4 violation persists. Worth a preemptive "head_t patched to commit_oid_hex via Git2LedgerWriter::commit return value" mini-atom while CO1.7.5 transition bodies are designed?

### Key commits this session (chronological)
- `a03cc52` CO1.7-impl A1: Git2LedgerWriter + bincode codec
- `227de72` CO1.1.4-pre1 v1: Typed Tx ABI surface
- `df548c5` CO1.1.4-pre1 R1 audit (CHALLENGE/CHALLENGE)
- `e0e4565` CO1.1.4-pre1 v1.1 (10 patches)
- `f4649a9` CO1.1.4-pre1 v1.2 (5 patches + 3 GR)
- `33e75b8` v1.2.1 + R3 (2 doc fixes)
- `4d917ac` v1.2.2 + R4 (2 more doc fixes)
- `c1226e2` **CO1.1.4-pre1 PASS/PASS** (R5)
- `609d8d5` A2+A3 Sequencer + dispatch
- `b6b7574` CO1.4-extra
- `272fcf4` A4 replay_full_transition
- `1a921e5` Bundle v1.1 (4 patches)
- `1bc8887` Bundle v1.1.1 (2 missing tests)
- `2461fe6` **Bundle PASS/PASS-equivalent**

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

### Phase B exit smoke test ruling + 2026-04-28 重跑

**Smoke test 不冲突 Phase C 冻结。** 冻结对象是 **C2 完整批量** (100 cell × ~50hr)；smoke 被归类为 "Phase B exit verification / C2 --smoke pre-flight" (per `HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md` § 2-3)。约束: smoke 必须框架成"管道活体检查"，不能框架成"Phase C 假设检验"。

**2026-04-28 smoke v3 结果**: ✅ **5/5 cells PASS in 146s** (canonical `--smoke`: 1 problem × 5 modes × 1 seed × MAX_TX=2)。每 cell wall-time 17-52s。soft_law cell 出现预期的 H2 ablation signal: `pput_runtime=1.18e-5` + `pput_verified=0.0` (runtime "fakes accept"，Lean post-hoc 拒绝)。

**两个 latent bug 在 smoke 过程中被发现并修复**:
1. **Proxy 部署 hygiene gap**: 跑了 14 天的 :8080 proxy 加载的是 **turingosv3 stale 源码**，v4 的 DeepSeek thinking-disabled 修复 (`src/drivers/llm_proxy.py:325` 用 `extra_body={"thinking":{"type":"disabled"}}` per 官方 docs `https://api-docs.deepseek.com/zh-cn/guides/thinking_mode`) 没在 running process 里。Kill + restart from v4 → log 确认 `0c reasoning` on every call。每 LLM call 从 30-60s 降到 ~1s。
2. **Runner `set -e + wait` 早退**: `run_c2_phase_c_ablation.sh` 的 pool dispatcher 用 `wait "$p"; rc=$?` 模式，`set -e` 在 wait 返回非零时立即 abort（早于 rc 捕获）。修复: `rc=0; wait "$p" || rc=$?`。这个 bug 之前没暴露是因为 thinking-on 时所有 cells 都 timeout 返回相同的非零，runner 死在 cell 1 之后；现在 thinking-off 修了，cells 真的成功+失败混合，bug 才显形。

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
