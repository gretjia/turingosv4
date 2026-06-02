# MARKET BINARY — CONSTITUTIONAL FC COMPLIANCE (FINAL SYNTHESIS)

**Date:** 2026-06-02
**Subject:** `src/bin/lean_market_agent.rs` as a faithful instance of the TuringOS constitution (`constitution.md`)
**Type:** READ-ONLY constitutional-compliance audit. No code was edited; FC1/FC2/FC3 unchanged.
**Evidence corpus:** `handover/evidence/p1_v4_2026-06-02/` — 732 primary manifests, 16,758 total nodes (35 Verified + 16,723 Failed-with-feedback), 732 `.replay.json` siblings, per-run CAS evidence stores.
**Inputs synthesized:** the MAP + G1 (FC1 runtime loop), G2 (广播 broadcast/librarian), G3 (屏蔽 shielding + 谓词 predicates), G4 (Veto / Veto-AI), G5 (FC2 boot/halt + FC3 feedback + §4).

---

## OVERALL VERDICT: COMPLIANT-WITH-GAPS

**In principle, the market binary obeys the constitution.** It is a faithful **FC1 runtime instance** sitting on a real **FC2 boot** and producing the **FC3 within-run feedback substrate**. Every FC1 node (N1–N15) and both FC2 boot/halt branches are **LIT** — wired in code AND observed firing in a real tape/manifest field. The four top-level white-box mechanisms the architect named are effectively LIT:

- **广播 Broadcast (Librarian)** — **LIT**: fires every proposal, fed by real CAS-derived `lean_result.v2` ×24 sidecars (`stderr_cid=None`), source-scoped + shielded, projected to the Solver role. The price-signal half is independently LIT (`distinct_price_ratios` up to 18, `price_discovery=true`).
- **屏蔽 Shielding** — **DECISIVELY LIT**: across **16,723** nodes-with-feedback the **max newline count is 0** and max length is 160 — a raw multi-line Lean stderr dump would have many newlines; 0 across the whole corpus proves the shield fired on every failing verify. Three shield sites (judge `shield_lean_diagnostic`, route summary error-CLASS-only, librarian source-scope) all confirmed.
- **谓词 Predicate (Π_p)** — **LIT** as the dominant white-box gate: Lean kernel Verified (sorry/admit/native_decide source-rejected) AND a SECOND `#print axioms` run requiring the transitive set ⊆ `{propext, Classical.choice, Quot.sound}`. All **35** omega nodes carry a non-empty whitelisted `axioms[]`; **0** are non-clean. Price never overrides the predicate.
- **Veto** — split correctly into two orthogonal, BOTH-honored vetoes: (a) the **in-loop price veto** (loss-bearing Bear short ChallengeTx) is **LIT** (`bear_calls` 4–24 on every price-family arm, exactly 0 on every Bulls-only control, `counterexample.v1` on tape); (b) the constitutional **Veto-AI** ({PASS,VETO} over architecture) is **correctly ABSENT** from this FC1 binary (0 grep hits) and lives at FC3.

The single material weakness keeping this from unqualified COMPLIANT is a set of **DORMANT true-positive paths** (an adversarial gap, not a wiring gap): the axiom-gate's non-whitelist-axiom catch, the SorryBlocked reject arm, and the librarian-injection per-run tell were not exercised/telemetered in this corpus. They are wired + unit-tested but lack a live positive in p1_v4. None of these flip a verdict — but each is a gap a careful auditor must name.

---

## COMPLIANCE MATRIX

| Mechanism | FC node | Status | Evidence (cite) |
|---|---|---|---|
| Q_t carrier ⟨q_t,HEAD_t,tape_t⟩ (ChainTape substrate) | FC1-N1 | **LIT** | Real per-run git `runtime_repo` + `final_state_root_hex=cce9122f…` (lm_fact s2); 732/732 runs have a git substrate, not memory-only |
| q_t slice + HEAD_t read at loop top | FC1-N2/N3 | **LIT** | `seq.q_snapshot()` (lm_agent:1314); every one of 16,758 nodes carries `price_yes_num` derived from `q.economic_state_t`; final root ≠ genesis → HEAD advanced |
| rtool — shielded read view (price + librarian + parent body/feedback) | FC1-N5 | **LIT** | `compute_price_index` (1316) + `real_librarian_solver_notice` (1349, `stderr_cid=None`); node feedback is a single shielded error-class line |
| input bundle ⟨q_i,s_i⟩ (shared `stage2_proof_prompt`) | FC1-N6 | **LIT** | `proof_prompt_tokens` non-zero on every counted run; byte-identical prompt builder across arms (confound-B fix); `route_prompt_tokens=0` on market arm proves path split |
| Agent δ (external LLM) | FC1-N7 | **LIT** | `proposal_llm_calls=24` / `llm_calls=24` (lm_fact s2); autonomous arm fires a 2nd Stage-1 route δ (`route_llm_calls=24`, lm_c autonomous s1) |
| output ⟨q_o,a_o⟩ (parsed proof_body + confidence) | FC1-N8/N9/N10 | **LIT** | `extract_json_object` (1446); `node[].body_preview` on all 16,758 nodes; `parse_fails` counted (corpus total 347) |
| 谓词 Π_p — Lean kernel + #print-axioms whitelist gate | FC1-N11/N12 | **LIT** | `judge.verify` (1473) → `axiom_gate` (lean_judge.rs:219); whitelist=`{propext,Classical.choice,Quot.sound}` (lean_judge.rs:71); 35 Verified nodes ⊆ whitelist, 16,723 Failed have `axioms=[]`; `lean_verifies=24`==nodes |
| wtool — sequencer-mediated typed-tx commit | FC1-N13 | **LIT** | `submit_await` (432) fail-closed on no-root-advance; CAS index lm_fact s2 = `TypedTx.v1`×126 + `verification_result.v1`×24 + `proposal_telemetry.v1`×24 — every attempt on tape |
| Q_{t+1} accept branch (advance + OMEGA settlement) | FC1-N14 | **LIT** | `omega_reached=true`, `omega_node=worktx-lm-node23-…` (lm_fact s2); EventResolve(Yes); 35/732 runs reached omega |
| Q_t reject branch (Π_p=0 → Failed node on tape) | FC1-N15 | **LIT** | `failed_count=23` (lm_fact s2); Failed nodes still WorkTx+VerifyTx(false); `failed_count==false-node-count` in **732/732** runs |
| 广播 Broadcast — Librarian collective digest | FC1-N5 / FC3-N31 (Art.II) | **LIT** | `real_librarian_solver_notice` unconditional per proposal (1349); CAS `lean_result.v2`×24 = broadcast SOURCE sidecars; recompute over real CAS → non-empty Solver notice, cluster count≥2 (6/23/3 in three runs); `assert_no_forbidden_broadcast_material` on every event/cluster/line |
| 广播 Broadcast — price signal + price index | FC1-N5 (Art.I.2/II.2) | **LIT** | `compute_price_index` integer-rational (price_index.rs:164); `distinct_price_ratios=13`, `price_discovery=true` (lm_fact s2); 312/732 runs have >1 distinct ratio; `price_yes_num/den` per node |
| 屏蔽 Shielding — raw Lean diag → opaque error CLASS | FC1-N5 / FC3 (Art.III) | **LIT** | `shield_lean_diagnostic` (lean_judge.rs:487, ONLY `from_utf8_lossy` of stderr); **16,723** nodes feedback max-newlines=**0**, max-len=160 — single-line shield fired on every fail |
| 屏蔽 — route summary (error-CLASS only, no body) | FC1-N5 (Art.II.1) | **LIT** | `build_route_summary` (667) emits only `[idx,price,conf,class,age,hash]`; `classify_lean_error` coarsens; parity self-test asserts no body/shielded-error leak; autonomous `route_hallucinated_out_of_range=0` corpus-wide → compact channel sufficient & used |
| Veto (a) — in-loop price veto (loss-bearing Bear short) | FC1-N10/N12 (Art.I.2/II.2) | **LIT** | `bear_doubt_short`+`emits_challenges`→ real `ChallengeTx` (1594) + `put_counterexample`; `bear_calls=24` (lm_fact s2); CAS `lm.counterexample.v1`×24; Bulls-only arms = **0** bear, price-family = **never 0** (informed short, not constant); replay reconstructs the on-tape ChallengeTx |
| 谓词 (b) — axiom-gate soundness veto (#print axioms) | FC1-N12 (Art.I.1.1 PCP) | **LIT (with dormant true-positive)** | `axiom_gate` 2nd Lean run + subset filter (lean_judge.rs:290-296); OMEGA-guard proven NEGATIVELY (35/35 omega nodes axiom-clean, 0 dirty). **Gap:** 14 firings are all the fail-closed name-miss branch (one probe manifest); **zero** TRUE non-whitelist-axiom catches in p1_v4 — subset half is unit-tested only |
| Non-local routing — TRUE Boltzmann softmax | FC1 (Art.II.2.1) | **LIT** | imports `boltzmann_softmax_select_parent` (actor.rs), NOT argmax; cumulative-weight sampling; golden_path non-contiguous (node0→1→14→20→23 lm_fact s2) = non-local re-expansion actually occurred |
| Boot — InitAI + Q_0 minted once | FC2-N16/N21 | **LIT** | `genesis_with_balances`(1211)→`build_chaintape_sequencer_with_initial_q`(1219), `resume_existing_chain=false`; every CAS dir has `initial_q_state.json`; integer MicroCoin balances |
| Boot — predicate registration + map-reduce tick | FC2-N19/N20 | **LIT** | `activate_predicate_binding_for_boot`+`activate_map_reduce_tick_for_boot` (runtime/mod.rs:872/875); CAS `turingos-predicate-registry-snapshot-v1`×1 physically written for lm_fact s2 |
| Map-reduce tick over (n_agents×n_rounds) grid | FC2-N20/clock/mr | **LIT** | `for round … for ai …` (1311-1312); n_agents=4×n_rounds=6 → 24-node grid materialized (lm_ineq2 s6). *Honest note: no scalar `mr_tick` field in JSON; firing via boot primitive + grid count + L4 gate, not a manifest scalar* |
| Finalization — HALT (success: omega) | FC2-N22/N23 | **LIT** | `break 'outer` on axiom-clean Verified (1703); `omega_reached=true`, `time_to_first_proof_s`, EventResolve(Yes); 35 runs |
| Finalization — HALT (budget exhaustion: No) | FC2-N22/N23 | **LIT** | natural loop end when `omega_node=None`; `omega_reached=false`, `failed_count=24`, `pput=0.0`, `golden_path=[]` (lm_ineq2 s6) — paired same-family example |
| tools_other — CAS evidence + market scaffold | FC2-N28 | **LIT** | signed `TaskOpen/EscrowLock/cpmm_pool`; CAS `proposal_payload.v1`×20 + `proof_artifact.v1`×20 + `verification_result.v1`×24 |
| FC3 within-run feedback (failure→tape→librarian→next prompt) | FC3 edge | **LIT (injection tell partial)** | feed LIT (`lean_result.v2`×24, `stderr_cid=None`); consume `real_librarian_solver_notice` unconditional; autonomous routing is the DIRECT tell — `route_valid_index_hit>0` in 78 runs, can only be >0 if prior-failure tape drove a route. **Gap:** no `librarian_notice_non_empty` scalar — Stage-2 injection firing is inferred, not directly counted |
| 谓词 — kernel-bypass source rejection (sorry/admit/native_decide) | FC1-N12 (Art.I.1) | **LIT (reject arm dormant)** | `KERNEL_BYPASS_TOKENS` source-scan before any kernel run (lean_judge.rs:51,153); on the live `judge.verify` path for all 16,723 attempts. **Gap:** 0 `SorryBlocked` nodes in the sampled corpus — the specific reject arm has no live tape tell (model emitted none) |
| §4 failed-branch rule (Lean-reject → verified=false node) | §4 / Art.0.2#5 | **LIT** | WorkTx for EVERY attempt regardless of verdict (1508); `nodes.push(AttemptNode{…})` unconditional; 16,723 false nodes are real signed WorkTx, not a graveyard HashMap |
| §4 parse-fail branch | §4 (reject_class=parse-fail) | **LIT (schema divergence)** | `parse_fails` counter (1449/1460), corpus total 347, tape-derived. *Honest note: parse-fail `continue`s BEFORE the WorkTx push (no body to anchor), and `reject_class` is not a literal node field — info is captured via `verdict`+`parse_fails` counters instead* |
| **Constitutional Veto-AI ({PASS,VETO} over architecture)** | **FC3** | **CORRECTLY ABSENT** | **0** grep hits in `lean_market_agent.rs`; lives at FC3 (`real5_roles.rs` AgentRole::Veto, `VetoDecisionTx`). The market emits only WorkTx/ChallengeTx/VerifyTx/EventResolve — nothing for Veto-AI to gate, so absence is CONFORMANT, not a bypass |
| **FC3 meta-loop (ArchitectAI / re-init / log-archive→architect)** | **FC3** | **MISSING-by-design** | 0 emissions of ArchitectProposal/Commit/VetoDecision/Reinit in this binary; exercised by `tests/constitution_fc3_closure.rs`, outside one market run. Flagged so the "faithful FC instance" claim is scoped to FC1-runtime + FC2-boot/halt + FC3-within-run |

**Independent cross-manifest verification (732 manifests, this synthesis):** shielding max-newlines=0 / max-len=160 across 16,723 feedback nodes · omega 35/35 axiom-clean (0 bad) · 35 Verified all ⊆ whitelist (0 non-white) · Bulls-only bear=0 (0 exceptions) · price-family bear≠0 (0 exceptions) · route_hallucinated_out_of_range=0 corpus-wide · failed_count==false-node-count 732/732 · 732 replay siblings reconstruct (ledger_root_verified=true).

---

## PER-MECHANISM EVIDENCE NOTES

### 广播 Broadcast (Librarian) — LIT
Wired: `real_librarian_solver_notice` (lm_agent:546) → `select_librarian_events`/`build_librarian_digest`/`project_role_notifications(Solver)`/`validate_librarian_source_scope`/`assert_no_forbidden_broadcast_material` (librarian_broadcast.rs). Called UNCONDITIONALLY every proposal (1349), fed into `build_prompt`. Fired: the broadcast SOURCE sidecars (`turingosv4.lean_result.v2`) are physically on tape — ×24 in `cas_p1v4_lm_fact_market_s2`, ×7 in autonomous_s3 — with `stderr_cid=None` so only the opaque error CLASS transits. G2's deterministic recompute over the preserved CAS yielded a non-empty Solver notice with a ≥2-member `err:LeanFailed` cluster. **Caveat:** prompts are ephemeral (the rendered section is a transient String, never persisted), so the *injection* of a non-empty notice into a given Stage-2 prompt has no direct manifest scalar; it is proven structurally (always-on call site) + by the SOURCE-sidecar count + by the autonomous route tell.

### 屏蔽 Shielding — DECISIVELY LIT
`shield_lean_diagnostic` (lean_judge.rs:487) is the ONLY place stderr is decoded (`from_utf8_lossy` at :491), takes the first `error:` line, truncates ≤240B. The strongest single number in this audit: **across all 16,723 nodes-with-feedback the maximum newline count is 0**. A raw Lean stderr (goal state + hypotheses) is inherently multi-line; 0 newlines corpus-wide is a hard structural proof the shield fired every time. Two further shield sites confirmed: route summary carries only `classify_lean_error` class tokens (no body); the librarian feed is pre-shielded (`stderr_cid=None`, lm_agent:1494). The `/tmp/…leanjudge` path prefix on the one-line feedback is the Lean compiler's own location tag on the agent's OWN candidate (legitimate retry signal per CLAUDE.md §4), not a private-diagnostic leak.

### 谓词 Predicate Π_p — LIT (dominant white-box gate)
Two-stage: (1) source-scan rejects `KERNEL_BYPASS_TOKENS={sorry,admit,native_decide}` (lean_judge.rs:51) BEFORE any kernel run — critically forcing `native_decide` (which would otherwise exit-0 and pull `Lean.ofReduceBool`) out of the Verified path; (2) `lean -DwarningAsError=true` exit-0 THEN a SECOND `#print axioms <name>` run whose transitive set must ⊆ `AXIOM_WHITELIST` (lean_judge.rs:71). Binary via `is_verified()`. Price-never-a-gate enforced by `tests/constitution_predicate_gate.rs`. Live: all 35 omega nodes carry `axioms=[Classical.choice,Quot.sound,propext]` (⊆ whitelist); all 16,723 Failed carry `axioms=[]` (gate not reached on compile-fail); omega fires ONLY when `is_verified` (lm_agent:1692). This is an honest soundness certificate (a real 2nd Lean invocation per Verified), not a grep.

### Veto — two orthogonal vetoes, both honored
**(a) In-loop price veto (FC1, economic/epistemic):** `bear_doubt_short` skeptic-LLM → integer `short_micro` → real signed `ChallengeTx` on `OutcomeSide::No` + a `lm.counterexample.v1` CAS object. `bear_calls=24`/`bear_llm_calls=24` (lm_fact s2). Genuinely loss-bearing: settles to Yes only if a kernel-Verified omega exists, else No; `bear_tokens` folded into the PPUT denominator (real accounted cost). The control structure is the proof it is *informed*: Bulls-only arms (single/parallel/restart/tree) land exactly 0 bear calls; every price-family arm (market/autonomous/no_price/shuffled_price) fires — 0 exceptions across 732 runs. Replay reconstructs the ChallengeTx (l4_entries=111, economic_state_reconstructed=true).
**(b) Constitutional Veto-AI (FC3, {PASS,VETO} over architecture):** CORRECTLY absent (0 grep hits). It gates ArchitectAI mutations of the Trust-Root — an FC3 meta event. The market never proposes a constitution/trust-root change, so there is nothing for Veto-AI to gate; its absence here is conformant, and no §17.3 name-lie exists (nothing in the binary is mislabeled "Veto-AI").

---

## PRIORITIZED GAPS

| # | Gap | Severity | Minimal Fix |
|---|---|---|---|
| 1 | **Axiom-gate true-positive path DORMANT.** All 14 axiom_gate firings in p1_v4 are the fail-closed *name-miss* branch (one probe manifest); ZERO proofs actually compiled WITH a forbidden axiom (e.g. `Lean.ofReduceBool`/`sorryAx`) and got caught by the subset check. The load-bearing soundness half is unit-tested (lean_judge.rs axiom-dirty test) but has no live positive. | **Medium** | Add ONE deliberate red-team task whose `proof_body` smuggles `native_decide`/a hand-axiom into the corpus, and assert a manifest node with `axiom_rejected=true` + non-whitelist axioms appears. Turns a code+unit claim into a live tape tell. |
| 2 | **Librarian Stage-2 injection has no direct manifest tell.** The broadcast SOURCE sidecars and the always-on call site are LIT, but whether a *non-empty* notice actually entered a given proof prompt is inferred, not counted. | **Medium** | Emit a per-run scalar `librarian_notice_nonempty_count` (and/or `librarian_cluster_count`) from the bin — a 1-line telemetry add that makes 广播 injection directly tape-verifiable instead of recompute-only. |
| 3 | **SorryBlocked reject arm DORMANT.** The kernel-bypass source rejection is on the live verify path, but the model never emitted a bare `sorry`/`admit`/`native_decide` in this corpus, so the `SorryBlocked` verdict has no live node. | **Low** | Include one task/seed where a stub proof emits `sorry`; assert a `verdict=SorryBlocked, is_verified=false` node lands. (Lower severity: the arm's positive *contribution* — forcing native_decide out of Verified — is already evidenced by the 35 clean omega nodes.) |
| 4 | **§4 `reject_class` literal field divergence.** The constitution's §4 wording is `reject_class=…` as a node field; the bin instead carries `verdict` + a `parse_fails` counter. Information is fully tape-reconstructable but the literal schema differs, and parse-fail produces no WorkTx node (counter only). | **Low** | Optional: add a `reject_class` enum field to the AttemptNode (lean/axiom/parse-fail) for literal §4 schema parity. Documentation-or-schema choice; no behavioral defect. |
| 5 | **FC2 map-reduce *tick row* not re-derivable from JSON alone.** Boot mr-tick lands on L4 and is gate-asserted, but the JSON manifest evidences the grid only via node count, not the tick row. | **Low** | Surface the boot MapReduceTick L4 row id (or a `boot_mr_tick_cid`) into the manifest so the FC2-N20 tick is JSON-re-derivable, not only L4/gate-level. |
| 6 | **FC3 meta-loop (ArchitectAI/Veto-AI/re-init) MISSING in this binary — by design.** Not a defect for an FC1 runtime instance, but the "faithful FC instance" claim must be explicitly scoped. | **Informational** | None for the market binary. Keep the scoping note: meta-loop closure is covered by `tests/constitution_fc3_closure.rs`, not a proof-search run. |

**No HIGH-severity gaps.** No mechanism is mislabeled, no price-overrides-predicate path exists, no raw-stderr leak, no argmax-masquerading-as-softmax, no silent-drop of failed branches. The gaps are all "a wired+tested mechanism lacks a live positive in this particular corpus" or "literal-schema vs information-equivalent" — exactly the dormant-but-compiles class the architect asked to be named.

---

## CONCLUSION
The market binary is a **faithful FC1 runtime instance** on a real FC2 boot, producing the FC3 within-run feedback substrate. 广播 / 屏蔽 / 谓词 / (price) Veto are all effectively **LIT** with live tape evidence; the constitutional Veto-AI is correctly **absent** at this layer. Verdict: **COMPLIANT-WITH-GAPS** — ship-faithful in principle, with six prioritized gaps (2 Medium, 3 Low, 1 Informational), the most important being to give the axiom-gate true-positive and the librarian-injection a live tape tell rather than code+unit+recompute proof.

---

## ADDENDUM — gaps M2 / L4 closed; M1 / L3 surfaced (2026-06-02)

The two Medium gaps + the L4 schema divergence are now fixed in `src/bin/lean_market_agent.rs` (+ tests),
re-verified on a **live `market`/lm_e cell** (12 proposals, 12 failed, bear_calls=12):

| gap | fix (code) | LIVE tape tell |
|---|---|---|
| **M2** broadcast-injection had no manifest tell | `Manifest.librarian_notice_nonempty_count` + `librarian_notice_chars`, counted at the Stage-2 injection point; `f2_manifest_has_compute_telemetry_fields` asserts presence | **`librarian_notice_nonempty_count = 10/12`, `librarian_notice_chars = 2642`** — the 广播 collective notice is now a DIRECT manifest scalar (was recompute-only). The 2 misses are the first proposals, before any failure exists to summarize. |
| **L4** §4 `reject_class` literal divergence | `reject_class_of()` + `AttemptNode.reject_class` (`lean-reject` / `axiom-rejected` / `sorry-blocked`, `None` iff Verified); deterministic `l4_reject_class_taxonomy` test | **`reject_class` on all 12 failed nodes**; invariant `reject_class==None ⟺ is_verified` holds. The constitutional reject CLASS is now a first-class node field. |
| **M1** axiom-gate true-positive / **L3** SorryBlocked arm | already real-tested (`lean_judge.rs:668` hand-axiom→`axiom_rejected` under real Lean; `:552` `verify("sorry")→SorryBlocked`); `reject_class` now surfaces them on tape if they fire | a future production axiom-reject / sorry-block is now directly tape-visible via `reject_class` (no CAS archaeology) |

**Deferred (by design / out of this binary):** **L5** (FC2 mr-tick JSON re-derivability) would touch
`runtime/mod.rs` (trust-root) → left to a dedicated trust-root atom; **Info6** (FC3 meta-loop) is covered by
`tests/constitution_fc3_closure.rs`, not a market run. Verify: `cargo test --bin lean_market_agent` 14/0,
`--self-test` OK, build clean, no §6/trust-root surface touched, integer money intact.
