# Architect Ruling — TB-6 Frame-A acceptance + TB-7 Frame-B authorization + post-TB-7 Lean-Proof-Task-Market MVP priority

**Date**: 2026-05-01
**Source**: User-relayed architect ruling (response to `handover/directives/2026-05-01_TB7_ARCHITECT_REVIEW_REQUEST.md`, charter draft commit `cee78f9`).
**Branch at ingest**: `main` @ `cee78f9` (TB-7 charter DRAFT + architect review request).
**Form**: ChatGPT-style headered narrative, 9 numbered sections (§1 TB-6 audit verdict / §2 TB-7 authorization / §3 charter amendments D1-D5 / §4 seven ship gates / §5 alignment to constitution+WP+roadmap / §6 process evaluation / §7 risk-class audit / §8 launch path / §9 final judgment), plus an explicit "下面是可以直接给 AI coder 的正式裁决" header that frames this entire document as a binding directive.
**Scope reach**: TB-6 verdict re-classification (Frame A only) + TB-7 charter ratification with 7 mandatory amendments + 2 new memory rules + post-TB-7 MVP sequencing reset (Lean Proof Task Market MVP > NodeMarket).
**Status**: RATIFIED. Authorized for execution per user message "都确认" 2026-05-01 (post-`/clear` reload).

---

## §0 Headline verdict (architect's own words)

> **TB-6 可以接受，但只能接受为 Frame A closure。**
> Frame A: GREEN.
> Frame B: RED.

> **TB-7 应该启动，而且 TB-7 必须是 Frame B。**
> Every meaningful LLM proposal → WorkTx / VerifyTx → bus.submit_typed_tx → Sequencer::apply_one → L4 or L4.E → CAS-linked proposal artifact → replay-verifiable run facts.

> 真正的 Frame B closure 是：**ChainTape 成为真实 LLM proposal 的 authoritative path，而不是附加日志。**

Reframed from the charter-draft framing:

```
TB-6 charter draft says: "wire bus.append + bus.append_oracle_accepted sites
                          to also emit WorkTx via bus.submit_typed_tx"
Architect ruling   says: "All authoritative proposal writes route through
                          bus.submit_typed_tx. Legacy bus.append must become:
                          (1) removed from proposal state mutation path; OR
                          (2) read-only derived projection from ChainTape; OR
                          (3) explicitly marked shadow-only — in which case
                          TB-7 cannot claim Frame B closure."
```

This is the single most load-bearing amendment. "也 emit" 是错的。

---

## §1 TB-6 audit verdict — Frame A only

### §1.1 What TB-6 actually shipped (verified)

- Production binary (`runtime::driver`) drives `Sequencer::apply_one` to on-disk ChainTape (commit `b0a6039` Atom 3).
- `refs/transitions/main` git chain + CAS payloads + L4.E rejections.jsonl exist (`38f7112f6...`).
- `verify_chaintape` library + CLI (Atom 4 `f594f83`) reconstructs QState + EconomicState from L4 alone; tampering detection via I90c.
- AgentProposalRecord 9-field shape + JSONL prev_hash→hash chain (Atom 5 `fcbb827`); structural witness I91d blocks future schema migrations from adding chain-of-thought fields.
- RunSummary aggregator (Atom 6 `8e5ddb3`) records proposal-level fork visibility.
- Atom 7 ship audit + book-keeping closed (`17c5e73`); 660/0/150 `cargo test --workspace`.
- 5-TB ChainTape production debt CLOSED at narrow Frame A.

### §1.2 What TB-6 did NOT ship (charter-acknowledged)

The TB-7 charter draft `§0 Why this TB exists` already concedes:

```
TB-6 chain: 2 entries total (1 synthetic TaskOpen + 1 synthetic zero-stake WorkTx).
None of the 20 LLM proposals on mathd_algebra_107 traverse the chain.
PputResult is computed by evaluator's in-memory accumulator; NOT chain-derivable.
```

### §1.3 Verdict re-classification

```text
Frame A: GREEN  (production binary triggers Sequencer; on-disk chain exists; replay verifier exists; synthetic seed enters ChainTape)
Frame B: RED    (real LLM proposal generation, Lean verification, OMEGA accept still route through legacy bus.append / bus.append_oracle_accepted)
```

**Permitted phrasing**: "生产二进制已经能触发 ChainTape；真实 LLM proposal 还没有全部进入 ChainTape。"
**Forbidden phrasing**: "真实 Agent 行为已经全部上 ChainTape。"

TB-6 is **accepted** at Frame A boundary; production claim from `TB_LOG.tsv` line 23 stands as written (it correctly says "verifiable from on-disk artifacts produced by an LLM-driven run" — the Atom 3 smoke run was LLM-driven; the chain entries on it were synthetic seeds writing the run's metadata, not the LLM proposals themselves; this distinction is preserved by Frame A vs Frame B language).

---

## §2 TB-7 authorization — Frame B = per-LLM-proposal WorkTx routing

### §2.1 Selected option

**D1 = Option A (Frame B)**. NOT RSP-3.2 Slash. NOT RSP-M0/M1. NOT P6 capability work.

Rationale: if real LLM proposals do not enter ChainTape, every downstream economic mechanism (NodeMarket, Slash, Settlement, Polymarket) lacks a real audit base. Frame B is the natural and necessary successor to Frame A.

### §2.2 Frame B closure definition (authoritative)

```text
Every meaningful LLM proposal
  -> WorkTx / VerifyTx
  -> bus.submit_typed_tx                  (authoritative path; not "also emit")
  -> Sequencer::apply_one
  -> L4 or L4.E                           (accepted XOR rejected per WP § 18 Inv-5)
  -> CAS-linked proposal artifact         (ProposalTelemetry CAS object)
  -> replay-verifiable run facts          (ChainDerivedRunFacts == evaluator structural facts)
```

If `bus.append` / `bus.append_oracle_accepted` remain as authoritative state mutation paths post-TB-7, **TB-7 cannot claim Frame B closure** regardless of how many other criteria pass.

### §2.3 Out of scope (deferred, NOT cancelled)

- `FinalizeRewardTx` settlement — RSP-4 / TB-11 territory.
- `SlashTx` upheld-challenge punishment — RSP-3.2 / TB-9 territory.
- NodeMarket position semantics, AMM, market trading — deferred to **post-MVP** (see §6 below; RSP-M is no longer the immediate next track).
- Cross-run agent identity persistence — needs separate TB (`Persistent AgentRegistry + agent keystore`).
- New TypedTx variants — existing WorkTx + VerifyTx suffice (charter §4.1 inherited).
- Q schema mutation, agent chain-of-thought broadcast (TB-6 §6 forbidden inherited).

---

## §3 Charter amendment matrix (D1-D5 binding)

The TB-7 charter draft (`handover/tracer_bullets/TB-7_charter_draft_2026-05-01.md` @ `cee78f9`) requires **seven** specific amendments before Atom 1 begins. All are derived from this ruling.

### D1 — Authoritative path requirement (§4.0 NEW + §4.1 + §5.1 rewrite)

**Old text** (charter §4.1 + §5.1 row 4):
> "wire `bus.append` + `bus.append_oracle_accepted` sites to **also emit** WorkTx via `bus.submit_typed_tx`"

**New text** (binding):
> All authoritative proposal writes route through `bus.submit_typed_tx`. Legacy `bus.append` / `bus.append_oracle_accepted` MUST become one of:
> 1. **Removed** from authoritative proposal state mutation path;
> 2. **Read-only derived projection** from ChainTape (e.g., reconstructed view materialized by replay);
> 3. **Explicitly marked `shadow_only`** at the call site — in which case TB-7 §0 / §1 / §8 MUST NOT claim Frame B closure (the charter would degrade to "Frame B-shadow only" and require a follow-up TB).

Rationale: only options (1) or (2) discharge kill criterion **P1:1 — Agent cannot bypass `submit_typed_tx` / wtool to mutate state** under real LLM activity. Option (3) is a documented degradation, not a closure.

### D2 — Run-local keypair caveat (§4.2 amendment)

**Add** to charter §4.2 final paragraph:
> **Caveat**: This is **run-local identity, not durable reputation identity**. The keypair lifecycle is process-bound; private key dropped at evaluator exit; public manifest `agent_pubkeys.json` resides in run's `runtime_repo/`. Cross-run reputation, NodeMarket identity, or long-term agent economic identity REQUIRE a separate TB (`Persistent AgentRegistry + agent keystore`), NOT covered here.

Rationale: prevents future overclaim of "Agent identity is on-chain" — true only within run scope.

### D3 — OMEGA-accept narrowed scope (§4.3 confirmed; no edit)

Charter §4.3 already states:
- Accepted OMEGA → `WorkTx` with non-zero stake + signature
- Lean verification → `VerifyTx` with bond + verdict
- ChallengeWindow stays OPEN at TB-7 ship
- `FinalizeRewardTx` DEFERRED to TB-11 RSP-4
- `SlashTx` DEFERRED to TB-9 RSP-3.2

**Ratified as written.** No change required.

### D4 — Rename to ChainDerivedRunFacts (§4.4 + §5.1 row 3 + §7 Atom 5 rename)

**Old text** (charter §4.4):
> "Atom 5 introduces `runtime::chain_derived_pput::compute_pput_from_chain(runtime_repo, cas) -> PputResult`. The chain-derived value MUST match the in-memory PputResult..."

**New text** (binding):
> Atom 5 introduces `runtime::chain_derived_run_facts::compute_run_facts_from_chain(runtime_repo, cas) -> ChainDerivedRunFacts`.
>
> **Bit-exact field set** (verifiable from L4 + L4.E + CAS alone):
> - `solved`, `verified`, `tx_count`, `proposal_count`, `golden_path_token_count`, `gp_payload`, `gp_path`, `gp_proof_file`, `tactic_diversity`, `tool_dist`, `failed_branch_count`
>
> **Excluded fields** (NOT chain-derivable; remain in evaluator stdout):
> - `total_wall_time_ms`, `verifier_wait_ms`, `pput_runtime`, `pput_verified`, `pput_m_verified`, `h_vppu`
>
> **Pre-condition for `golden_path_token_count` chain-derivation**: token counts MUST be persisted as `ProposalTelemetry` CAS objects (see D5 / new Atom). Without this, `golden_path_token_count` cannot be claimed chain-derived and MUST be excluded from the bit-exact set.

Module rename: `src/runtime/chain_derived_pput.rs` → `src/runtime/chain_derived_run_facts.rs`. Atom 5 title updates to match. README Q10 updates: "Does chain-derived **run facts** match in-memory structural facts?"

### D5 — ProposalTelemetry CAS object (§5.1 + §7 NEW Atom)

**New file row** in §5.1 build surface table:

| File | Touch | Audit-class | STEP_B? |
|---|---|---|---|
| `src/runtime/proposal_telemetry.rs` (NEW) | Atom 1.5 — write per-proposal `ProposalTelemetry` CAS objects (agent_id, prompt_context_hash, proposal_artifact_cid, candidate_tactic, token_counts, tool_calls, branch_id, parent_tx) | additive | no |

**ProposalTelemetry shape** (binding):

```json
{
  "agent_id": "<string>",
  "prompt_context_hash": "<hex>",
  "proposal_artifact_cid": "<cid>",
  "candidate_tactic": "<string>",
  "token_counts": {
    "prompt_tokens": 0,
    "completion_tokens": 0,
    "tool_tokens": 0
  },
  "tool_calls": [],
  "branch_id": "<string>",
  "parent_tx": "<TxId or null>"
}
```

`WorkTx.proposal_cid` MUST point to this CAS object. Without this, `golden_path_token_count` cannot be claimed chain-derived (per D4).

### Atom plan amendment (§7 rewrite)

```text
Atom 0 — Charter ratification + ruling archive (THIS RULING + amendments encoded in charter)
Atom 1 — Agent keypair management (src/runtime/agent_keypairs.rs + agent_pubkeys.json manifest)
Atom 1.5 — ProposalTelemetry CAS writer (src/runtime/proposal_telemetry.rs)            [NEW per D5]
Atom 2 — Evaluator append-branch routing (per-LLM-proposal WorkTx via bus.submit_typed_tx as AUTHORITATIVE path; legacy append removed or shadow-marked per D1)
Atom 3 — Evaluator OMEGA-branch routing (WorkTx + VerifyTx pair; ChallengeWindow OPEN; no settlement)
Atom 4 — verify_chaintape extension (agent-signature verification path + ProposalTelemetry CAS lookup)
Atom 5 — chain_derived_run_facts.rs (renamed per D4; ChainDerivedRunFacts not full PputResult)
Atom 6 — chain-backed real-LLM smoke run on mathd_algebra_107
         (≥1 accepted L4 + ≥1 rejected L4.E per Gate 3; chain-derived run facts == evaluator structural facts)
Atom 7 — Audit + ship (Codex impl + Gemini arch with degraded fallback; recursive self-audit)
```

### Audit mode (§4.5 confirmed; no edit)

Charter §4.5 already mandates production wire-up class — Codex impl + Gemini arch with `degraded` fallback. **Ratified as written.** Bundling of TB-6 follow-up Codex impl audit at Atom 7 also retained.

---

## §4 Seven ship gates (binding; new §8 of charter)

The charter §8 (three success proofs) is REPLACED with the seven ship gates below. Atom 7 ship cannot claim closure unless all seven gates are GREEN with line-grounded evidence.

### Gate 1 — Authoritative path

```
Every meaningful LLM proposal MUST route through bus.submit_typed_tx
as authoritative state mutation.
No authoritative proposal mutation via legacy bus.append /
bus.append_oracle_accepted.
```

**Evidence shape**: grep / conformance test in `tests/tb_7_no_legacy_authoritative_append.rs` proves no evaluator hot path uses legacy append for authoritative proposal writes (see Gate 7 for the regression-test counterpart).

### Gate 2 — Proposal count equality

```
chain_proposal_count == evaluator_proposal_count
```

Both must come from instrumented emission paths (NOT stdout parsing). On `mathd_algebra_107` single-shot MAX_TX=20, both counts equal the same N.

### Gate 3 — L4 / L4.E coverage

```
≥1 accepted WorkTx in L4
≥1 rejected WorkTx in L4.E
All real proposal branches represented as accepted-or-rejected tx evidence.
```

**If `mathd_algebra_107` produces no natural rejected branch** (e.g., all 20 proposals route to accepted): the smoke run MUST add a labeled forced rejection, with explicit field `forced_rejection_for_gate_3 = true` on the rejection record. **Forced rejections MUST NOT masquerade as natural rejections.** (See §6 forbidden #33.)

### Gate 4 — Signature verification

```
Every WorkTx signature verifies against agent_pubkeys.json (per-run agent registry).
Every system-emitted tx signature verifies against PinnedSystemPubkeys
(unchanged from TB-5).
```

**Evidence shape**: `verify_chaintape` extended to verify both agent and system signature paths; both must return `true` on the smoke evidence chain.

### Gate 5 — ProposalTelemetry CAS retrievability

```
Every WorkTx.proposal_cid resolves to a CAS ProposalTelemetry object.
The object includes proposal artifact + telemetry per D5 shape.
```

**Evidence shape**: smoke evidence dir contains `cas/<cid>` blobs for every chain WorkTx; lookup test asserts retrievability + schema-shape validity.

### Gate 6 — Chain-derived run facts equality

```
chain_derived_run_facts == evaluator_run_facts
```

Compared on the bit-exact structural field set defined in D4. Time-sensitive fields excluded. Drift = Atom 5 test failure = ship blocker.

### Gate 7 — Legacy-bypass regression test

```
A new conformance test asserts that no proposal-producing evaluator
site can call legacy append as authoritative state mutation.
```

If legacy append is retained for a permitted purpose, the call site MUST carry `// shadow_only:` annotation that the conformance test recognizes and exempts. Any append call without this annotation in evaluator hot paths fails CI.

**Evidence shape**: `tests/tb_7_legacy_append_regression.rs` — repo-wide grep gate analogous to TB-1 P0-3 serde shield + TB-3 bridge invariant + TB-4 anti-drift CI scanner.

---

## §5 Alignment with constitution + WP + 9-phase roadmap

### §5.1 Constitution alignment

Constitution mandates the top-level white-box do three things:

```
量化 / 广播 / 屏蔽
```

and converts soft natural-language constraints into hard machine constraints. Foundational laws (`Information is Free`, `Only Investment Costs Money`, `1 Coin = 1 YES + 1 NO`, `on_init` sole mint) remain hard system boundaries.

If real LLM proposals do not enter ChainTape, the top-level white-box can quantize only the evaluator's self-report — not real Agent behavior. This is a violation of the Anti-Oreo architecture: the runtime claims to manage Agent ingress, but the actual proposal stream bypasses runtime ingress.

**TB-7 is therefore a constitutional necessity, not an enhancement.** Frame B closure is the structural step that brings real Agent activity under the ingress barrier proven by TB-5.

### §5.2 Whitepaper alignment

The whitepaper does not claim "we have logs"; it claims:

```
Agent does not directly write world state.
Agent submits state transitions.
Predicate / wtool / ledger decide accepted / rejected.
ChainTape is the state ledger.
Blockchain is one implementation among others.
```

TB-7 only achieves WP alignment when ChainTape becomes the **authoritative** path for real proposals — not a shadow mirror. Shadow-mirror configurations remain "legacy evaluator + audit log", structurally identical to pre-TB-6 architecture.

### §5.3 9-phase roadmap alignment

TB-7 belongs to **P2 Agent Runtime** with **P1 ledger invariants** and **P3 economic invariants** carried forward (re-discharged on real LLM activity). It MUST NOT drift into:

```
P3 Slash         (RSP-3.2 = TB-9)
P4 Information Loom
P5 MetaTape
P6 capability research
RSP-M NodeMarket trading
```

NodeMarket / Polymarket mechanics REQUIRE real proposal-on-chain anchor before `WorkTx.stake = first-long exposure` has economic meaning. Without TB-7, those mechanics would build positions on synthetic seed events, not real Agent behavior.

---

## §6 Post-TB-7 launch priority — Lean Proof Task Market MVP > NodeMarket

### §6.1 Sequencing reset

**Old plan** (TB-6 ARCHITECT_RULING §4.5, TB-7 charter §11):

```
TB-7:  P2 Agent proposal trail OR RSP-M0/M1 NodePosition
TB-8:  RSP-M2 NodeMarketEntry + PriceIndex v0
TB-9:  RSP-3.2 Slash execution
TB-10: RSP-M3 CompleteSet accounting
TB-11: RSP-4 SettlementEngine / ContributionDAG
TB-12: RSP-M4 MarketOrder / trading layer
```

**New plan** (binding):

```
TB-7:  P2 Frame B (per-LLM-proposal WorkTx routing)         ← THIS TB
TB-8:  Audit dashboard (UI / CLI to inspect what the Agent saw + submitted + how the system judged, on a per-run basis)
TB-9:  Minimal payout — TaskOpenTx + EscrowLockTx + WorkTx + VerifyTx + ChallengeTx (optional) + minimal FinalizeRewardTx (single solver, single verifier, no royalty, no ContributionDAG, no NodeMarket)
TB-10: Beta launch — narrow Lean problem set, real ChainTape, real replay, real escrow / payout
TB-11: NodeMarket v0 — WorkTx.stake = FirstLongPosition, ChallengeTx.stake = ShortPosition, PriceIndex v0 (no tradable market)
TB-12+: Polymarket-like full market — CompleteSet / MarketOrder / MarketTrade / MarketResolve / Redeem / LP liquidity
```

### §6.2 Rationale

```
正式上线 MVP = Lean Proof Task Market on ChainTape
```

NOT full TuringOS final form. The MVP is a vertical slice that:
1. Has authoritative ChainTape proposal path (TB-7);
2. Lets the user inspect what's happening (TB-8);
3. Closes a minimal economic loop (TB-9);
4. Runs on real LLM activity at small scale (TB-10);

NodeMarket / AMM / Polymarket / multi-org / public chain / royalty / ContributionDAG / long-term reputation are deferred until **after** the MVP launches. Reasoning: each of these adds structural surface that requires real-proposal anchor + real-payout grounding. Building them on synthetic anchor expands kernel-only debt instead of closing it (the same trap that produced "5-TB ChainTape production debt" pre-TB-6).

### §6.3 Memory codification

This priority shift will be saved as `feedback_launch_priority.md`:

```
Post-TB-7 default sequencing prioritizes Lean Proof Task Market MVP
(Audit dashboard → Minimal payout → Beta launch on narrow Lean problem set)
over full NodeMarket / Polymarket / multi-org / public chain.
Why: ruling §6 / §8 — NodeMarket / Slash / Settlement need real proposal-on-chain
anchor first; building them earlier expands kernel-only debt instead of closing it.
How to apply: when sequencing post-TB-7 TBs, default to MVP-vertical-slice path;
explicitly justify any TB that goes to NodeMarket / Slash / public-chain before
MVP ships.
```

---

## §7 Risk-class audit standard (process amendment)

### §7.1 The class system

| Class | Scope | Audit mode |
|---|---|---|
| **Class 0** | Documentation / charter / plan | No external audit. User / architect approval + TB_LOG / NOTEPAD sync. |
| **Class 1** | Additive module / isolated tests / non-state-touching | Self-audit + `cargo test --workspace`. No dual audit. |
| **Class 2** | Production wire-up (binary → kernel; evaluator hot path) | Codex implementation audit MANDATORY. Gemini architecture audit if available; `degraded` label if exhausted. |
| **Class 3** | Auth / crypto / money movement / system-emitted tx / settlement | Dual audit MANDATORY. Negative tests MANDATORY. Self-audit alone NEVER sufficient. |
| **Class 4** | Constitution / sudo / rootbox | Human sudo only. Agent cannot auto-advance. |

### §7.2 TB-7 placement

TB-7 = **Class 2** (production wire-up). Codex impl audit + Gemini arch with degraded fallback is sufficient.

If TB-7 expanded into FinalizeRewardTx wiring, it would escalate to **Class 3** — but D3 explicitly narrows scope to keep it Class 2. Any future PR that touches `FinalizeRewardTx` / `SlashTx` / `MarketResolveTx` / system keypair management is Class 3 by default.

### §7.3 Memory codification

This standard will be saved as `feedback_risk_class_audit.md`:

```
Audit mode by risk class:
  Class 0 (docs/charter/plan)     -> user/architect approval only
  Class 1 (additive module)       -> self-audit + workspace tests
  Class 2 (production wire-up)    -> Codex impl required + Gemini arch (degraded fallback OK)
  Class 3 (auth/crypto/money)     -> dual audit + negative tests mandatory
  Class 4 (constitution/sudo)     -> human sudo only

Why: ruling §7 process evaluation - prior framing of "production wire-up = full
audit" was correct but coarse; finer classification prevents over-applying heavy
audit on additive work AND prevents under-applying it on Class 3 surface.
How to apply: classify TB risk at charter time; declare class in §4 of charter
alongside phase_id / kill_criteria_tested. Class 3+ requires explicit user
sign-off before audit mode degradation.
```

This generalizes (and refines) the existing `feedback_dual_audit.md` "hybrid by risk class" rule.

---

## §8 Process evaluation — keep heavy audit, refine targeting

### §8.1 What's working

The current TB + charter + audit + kill-criteria flow has caught real drift:

- L4 / L4.E confusion (TB-3+).
- Ghost liquidity attempts (TB-3 / TB-4).
- WalletTool double-ledger risk.
- Runtime enforcement overclaim (TB-1 → TB-2 narrowing).
- System-emitted tx forgery surface (TB-5 stage 1.5).
- Smoke evidence not actually ChainTape (TB-6 directive D5).
- P6 capability metrics displacing P1/P3 infra (P6 re-classified 2026-04-29).

These are non-trivial saves. The flow is not ceremonial.

### §8.2 Where it's expensive

- Documentation overlap (charter / review request / audit prompt / self-audit / ruling all cover overlapping ground).
- Codex / Gemini prompts grow over time; signal-to-noise drops.
- Low-risk additive work occasionally pays full Class-2 audit cost.
- Atom over-fragmentation can produce ceremony commits.
- P6 smoke evidence has interrupted P1/P3 sequencing more than once.

### §8.3 Speed-up policy

**Do NOT cut audit.** Cut documentation duplication and tighten audit targeting via the §7 risk-class system. Specifically:

- Class 0 / Class 1 work bypasses external audit entirely.
- Class 2 audits must restrict prompt to changed surface + kill criteria, not "read 15 files first".
- Class 3 audits keep current heavy mode.
- Charter / review-request / audit-prompt may share a single source-of-truth markdown body, with role-specific framing in the header (analogous to TB-6 ARCHITECT_FULL_PROMPT pattern).

---

## §9 Final execution order (architect-pre-authored)

```
1. Accept TB-6 as Frame A only.
2. Authorize TB-7 as Frame B = per-LLM-proposal WorkTx routing.
3. Encode 7 charter amendments (D1-D5 + Atom plan + ship gates 1-7) before Atom 1.
4. Atom 1 = agent keypair management; Atom 1.5 = ProposalTelemetry CAS writer (NEW).
5. Atom 2 = evaluator append-branch routing — AUTHORITATIVE, not "also emit".
6. Atom 3 = evaluator OMEGA-branch routing — WorkTx + VerifyTx pair only.
7. Atom 4 = verify_chaintape extension (agent-signature verification + ProposalTelemetry retrieval).
8. Atom 5 = chain_derived_run_facts (renamed; bit-exact structural fields only).
9. Atom 6 = chain-backed real-LLM smoke run (≥1 L4 + ≥1 L4.E; forced rejection labeled if needed).
10. Atom 7 = Codex impl + Gemini arch (degraded fallback) audit + ship.
11. Save 2 new memory rules: feedback_risk_class_audit.md, feedback_launch_priority.md.
12. Post-TB-7 sequencing: TB-8 audit dashboard, TB-9 minimal payout, TB-10 beta launch.
    NO NodeMarket / Slash / public-chain before MVP ships.
13. After TB-7 ship audit, flag TB-6 follow-up Codex impl audit closure status; bundle
    if still pending.
```

---

## §10 Ingest-time impact analysis (Layer 1 invariants)

| Layer 1 invariant | Directive impact | Status |
|---|---|---|
| `kernel.rs` zero-domain-knowledge | TB-7 wires evaluator → existing kernel; adds NO domain knowledge to kernel | ✅ preserved |
| Append-Only DAG | TB-7 strengthens authoritative path enforcement: real proposals MUST traverse `Sequencer::apply_one`; legacy bypass = ship blocker (Gate 1 + Gate 7) | ✅ STRENGTHENED |
| Economic conservation (5-holding CTF, `1 Coin = 1 YES+1 NO`, on_init sole mint) | TB-7 forbids new economic mutators; FinalizeRewardTx / SlashTx explicitly deferred; NodeMarket explicitly post-MVP | ✅ preserved |
| Agent ≠ direct state writer (Anti-Oreo) | TB-7 brings real LLM proposal stream under ingress barrier proven by TB-5; converts the Anti-Oreo claim from "synthetic-seed-tested" to "real-LLM-tested" | ✅ STRENGTHENED |
| Append-Only DAG: rejected → L4.E (separate from L4) | Gate 3 mandates ≥1 L4 + ≥1 L4.E on real-LLM smoke; forced rejection allowed only with `forced_rejection_for_gate_3 = true` label | ✅ STRENGTHENED |
| System-emitted tx vs agent-emitted tx separation | Run-local agent keypair adds new ed25519 surface; signatures verified against `agent_pubkeys.json` (Gate 4); system pubkey path unchanged | ✅ preserved |
| Constitution.md hygiene | Ruling explicitly does NOT amend constitution. This is a roadmap / production-path gap, not a constitutional gap. | ✅ preserved |

**Verdict**: Directive does NOT violate Layer 1. It TIGHTENS Layer 1 enforcement on real Agent activity (production-path proof on real LLM proposals, not synthetic seeds). Safe to authorize execution. **No constitutional amendment.**

---

## §11 Original directive content (verbatim, 2026-05-01 user message post-`/clear`)

The original ruling was delivered as a single long message structured in 9 numbered sections plus a final judgment. Reproduced below in full so that future-self can re-read the architect's framing without reconstruction loss.

> ultrathink 已完成一份本地审计报告，包含 TB-6 审计、TB-7 方案审核、流程性价比评估，以及如果我独立推进正式上线会采用的开发路线
>
> 下面是可以直接给 AI coder 的正式裁决。
>
> ---
>
> # 1. TB-6 审计结论
>
> **TB-6 可以接受，但只能接受为 Frame A closure。**
>
> TB-6 的真实完成边界是：production binary triggers Sequencer::apply_one + on-disk LedgerEntry chain exists + replay verifier exists + synthetic seed enters ChainTape.
>
> 这确实是一个重要进展。它修复了 TB-1 到 TB-5 最大的"烟测不在链上"的诚实命名问题。
>
> 但 TB-7 charter 自己已经承认：TB-6 的链只有 **2 条 entry**，分别是 1 synthetic TaskOpen + 1 synthetic zero-stake WorkTx；真实 LLM proposal generation、Lean verification、OMEGA accept 仍然走 legacy `bus.append / bus.append_oracle_accepted`，没有经过 `bus.submit_typed_tx → Sequencer::apply_one`；`PputResult` 仍由 evaluator 的内存 accumulator 计算，不是从 L4 / L4.E / CAS 派生。
>
> 所以 TB-6 的准确 verdict 是：Frame A: GREEN / Frame B: RED.
>
> 不能说："真实 Agent 行为已经全部上 ChainTape。"
> 只能说："生产二进制已经能触发 ChainTape；真实 LLM proposal 还没有全部进入 ChainTape。"
>
> # 2. TB-7 是否应该启动？
>
> **应该启动，而且 TB-7 必须是 Frame B。**
>
> 我同意 TB-7 Architect Review Request 的 D1 推荐项：TB-7 选择 **Option A：Frame B = per-LLM-proposal WorkTx routing**。原因是，如果真实 LLM proposal 仍然不进入 ChainTape，那么后面的 NodeMarket、Slash、Settlement、Polymarket 机制都没有真实审计基础。
>
> TB-7 的正确目标是：Every meaningful LLM proposal -> WorkTx / VerifyTx -> bus.submit_typed_tx -> Sequencer::apply_one -> L4 or L4.E -> CAS-linked proposal artifact -> replay-verifiable run facts.
>
> 这比继续做 RSP-3.2 Slash 或 RSP-M NodeMarket 更重要。
>
> # 3. TB-7 charter 必须修改的关键点
>
> ## 3.1 "also emit WorkTx" 这个措辞必须改
>
> TB-7 draft 的 build table 里有一个危险措辞："wire evaluator sites to also emit WorkTx via bus.submit_typed_tx". 如果只是 "also emit"，那意味着 legacy `bus.append` 仍然是事实写入路径，ChainTape 只是旁路镜像。这不能证明 P1:1 Agent cannot bypass submit_typed_tx / wtool。
>
> 因此必须改成：All authoritative proposal writes route through bus.submit_typed_tx. Legacy bus.append must become either: (1) removed from proposal state mutation path; (2) read-only derived projection from ChainTape; (3) explicitly marked shadow-only, in which case TB-7 cannot claim Frame B closure.
>
> 这是 TB-7 最重要的修改。如果 TB-7 只是"旁路记录"，那它不叫 Frame B closure。真正的 Frame B closure 是：**ChainTape 成为真实 LLM proposal 的 authoritative path，而不是附加日志。**
>
> ## 3.2 Agent keypair 选择
>
> 接受 TB-7 D2 推荐：runtime-generated per-agent Ed25519 keypair / agent_pubkeys.json persisted in runtime_repo / private key lives in process memory.
>
> 这对 Frame B 足够。但必须加一句 caveat：**This is run-local identity, not durable reputation identity.**
>
> 不能把这种 ephemeral key 当作长期 reputation / market identity。如果后续要做跨 run reputation、NodeMarket、Agent 经济身份，需要独立 TB：Persistent AgentRegistry + agent keystore.
>
> ## 3.3 OMEGA accept path scope
>
> 接受 TB-7 D3 推荐的 narrowed scope：OMEGA accept -> WorkTx + VerifyTx / ChallengeWindow stays open / No FinalizeRewardTx / No SlashTx / No settlement.
>
> 这与整体路线一致。`FinalizeRewardTx` 属于 RSP-4，`SlashTx` 属于 RSP-3.2，不能塞进 TB-7。
>
> ## 3.4 Chain-derived PPUT 要改名或收窄
>
> TB-7 D4 说：chain-derived PPUT must match in-memory PputResult. 但它又排除了时间字段 (total_wall_time_ms, verifier_wait_ms, pput_runtime, pput_verified, pput_m_verified, h_vppu). 这就不是真正完整的 PPUT.
>
> 我建议改成：ChainDerivedRunFacts 或 ChainDerivedPputStructuralFields. 可以 bit-exact 的字段包括：solved, verified, tx_count, proposal_count, golden_path_token_count, gp_payload, gp_path, gp_proof_file, tactic_diversity, tool_dist, failed_branch_count.
>
> 但还有一个重要前提：**如果 token counts 没有进入 CAS / ChainTape，那么 `golden_path_token_count` 也不能叫 chain-derived。**
>
> 所以 TB-7 必须把每个 proposal 的 telemetry 作为 CAS object：[ProposalTelemetry shape]. 然后 WorkTx.proposal_cid -> ProposalTelemetry CAS object. 否则 TB-7 仍然要相信 evaluator stdout，不是真正 chain-derived.
>
> ## 3.5 TB-7 audit mode
>
> 选择 D5 Option A：production wire-up class / Codex implementation audit / Gemini architecture audit if available / degraded fallback allowed but must be labeled. TB-7 改 evaluator hot path，不能按 low-risk additive module 处理.
>
> # 4. TB-7 必须增加的 ship gates
>
> Gate 1: authoritative path gate. Gate 2: proposal count equality. Gate 3: L4/L4.E coverage (≥1 accepted + ≥1 rejected; forced rejection allowed but must be labeled). Gate 4: Agent + system signature verification. Gate 5: CAS proposal telemetry retrievability. Gate 6: chain-derived run facts equality. Gate 7: legacy bypass regression (CI/grep/conformance test).
>
> # 5. 与宪法 / 白皮书 / 整体方案的对齐
>
> [§5.1 Constitution: 顶层白盒做 量化/广播/屏蔽; 自然语言软约束转机器硬约束; on_init 是唯一合法铸币点; 反奥利奥架构. 真实 LLM proposal 不进入 ChainTape 则不满足.]
> [§5.2 Whitepaper: Agent 不直接写 world state; Agent 提交状态转移; Predicate / wtool / ledger 决定 accepted / rejected; ChainTape 是状态账本; 区块链只是实现之一.]
> [§5.3 9-phase roadmap: TB-7 = P2 Agent Runtime + P1/P3 carry-forward. 不要变成 P3 Slash / P4 Loom / P5 MetaTape / P6 capability / RSP-M NodeMarket.]
>
> # 6. 当前 TB 流程性价比评估
>
> 效果：高. 抓住过 L4/L4.E 混淆、ghost liquidity、WalletTool 双账本、runtime enforcement overclaim、system-emitted tx 伪造、smoke evidence 不是 ChainTape、P6 抢占基础设施.
>
> 成本：偏高. 文档重复; self-audit/charter/review request 信息重叠; Codex/Gemini prompt 太长, 信噪比下降; 低风险 additive 也可能套高风险流程; 过度 atomization; P6 smoke evidence 多次干扰基础设施.
>
> 加速方法：风险分级审计 (Class 0-4). NOT 取消审计.
>
> # 7. 如果我独立开发，以正式上线为目标，我会怎么做
>
> 目标改为：**正式上线 MVP = Lean Proof Task Market on ChainTape.** 不是完整 TuringOS 终局版.
>
> 暂缓: NodeMarket trading / AMM / public chain / MetaTape / multi-org / full RSP settlement / royalty / long-term reputation / P6 PPUT research expansion / h_vppu polish.
>
> 优先: ChainTape authoritative proposal path / Lean proof task market / escrow / WorkTx stake / VerifierTx / ChallengeTx optional / basic challenge window / minimal payout / replay CLI / audit dashboard.
>
> 路线: Phase A (Frame B closure = TB-7) -> Phase B (Audit dashboard) -> Phase C (Minimal payout) -> Phase D (Beta launch on narrow Lean problem set) -> Phase E (NodeMarket v0) -> Phase F (Polymarket-like full market).
>
> # 8. 给 AI coder 的直接裁决
>
> [Architect ruling block: 12 numbered items — TB-6 Frame A only / TB-7 Frame B authorized / charter amendments D1-D5 / D1=Option A / D2=run-local keypair / D3=narrowed OMEGA / D4=ChainDerivedRunFacts / D5=production wire-up / ProposalTelemetry CAS objects / 7 ship gates / no NodeMarket-Slash-Settlement-P6 in TB-7 / post-TB-7 priority Lean Proof Task Market MVP.]
>
> # 9. 最终判断
>
> 当前流程不是错，反而非常有价值；但它已经进入 **高安全、低速度** 模式. 不会建议取消审计. 会建议: 按风险分级审计 / 减少重复文档 / 缩小外部审计 prompt / 把开发目标收窄到正式上线 vertical slice.
>
> TB-7 是一个分水岭. 如果 TB-7 只是把 legacy evaluator 的输出"额外记一份链"，那 TuringOS 仍然没有真正上线核心机制. 如果 TB-7 做到 **只看 ChainTape + CAS + replay report，就能知道 Agent 真实提交了什么、哪些失败、哪些成功、Lean 验证了什么、run facts 是什么**，那 TuringOS 才真正从 kernel prototype 进入 operating substrate.

(End verbatim. The reconstructed canonical structure in §0-§10 above is the authoritative ruling form for execution; this §11 preserves the original framing for archival completeness.)
