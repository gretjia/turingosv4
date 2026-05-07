# Architect Alignment + Autonomous-Execution Authorization (2026-05-07)

**Status**: ACTIVE — Class-3/4 ship-eligible authorization for the Stage A→D plan in the
two architect alignment documents archived earlier today
(`handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md` +
`..._en.md`).

**Authority**: User-as-architect explicit multi-clause authorization, in-conversation 2026-05-07.

**Storage policy**: Lossless archive per `feedback_kolmogorov_compression`. Original architect
message preserved verbatim below.

**Companion documents**:
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_zh.md`
- `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_en.md`
- `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md` (TB-18R FINAL ship sign-off, derived from this authorization)
- `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md` (precedent for §8 sign-off format)

---

## §1. Architect message (verbatim)

```
严格对齐以上两个架构师文件，自动执行，你可以在这两份文件中找到所有的决策依据，
如果处于边缘问题，通过宪法解决。授权你完全自主执行指导完成全部任务，
包括调取外部审计和LLM真题测试： Current project state:
Constitutional harness is now primary.
Do not resume old Atomic Agentic Engineering.

Immediate priorities:
1. Finish TB-18R final sign-off with current-head evidence.
2. Close remaining constitution AMBER rows.
3. Keep FC1/FC2/FC3 gates green.
4. Do not start executable Polymarket features until constitution gates and diagnostic benchmarks are stable.

When starting Polymarket:
1. Quarantine legacy f64 CPMM.
2. Harden CompleteSet Mint/Redeem.
3. Implement CompleteSetMergeTx.
4. Harden MarketSeedTx.
5. Implement integer CpmmPool.
6. Implement share-only swap.
7. Implement Mint-and-Swap router exactly as specified.
8. Add audit views.
9. Run controlled market smoke.

At every step:
- no f64,
- no ghost liquidity,
- no price-as-truth,
- no dashboard source-of-truth,
- no real funds,
- no public chain.
```

(Translation, for non-Chinese auditors: "Strictly align to the two architect documents
above. Auto-execute. You can find all decision basis in those two files. For edge cases,
resolve via constitution. Authorized to fully autonomously execute and guide completion
of all tasks, including invoking external audits and LLM real-problem testing." Followed
by an English block restating priorities + Polymarket sequencing + universal forbidden
list.)

**Multi-clause analysis** (per `feedback_class4_cannot_hide_in_class3` + CLAUDE.md §10):
the message contains FIVE distinct authorization clauses:

1. **Scope binding**: "严格对齐以上两个架构师文件" — execute against the two zh + en alignment docs.
2. **Mode binding**: "自动执行" — autonomous execution authorized.
3. **Edge-case rule**: "如果处于边缘问题，通过宪法解决" — constitution is the tie-breaker.
4. **Authority grant**: "授权你完全自主执行指导完成全部任务" — full autonomy on the alignment-doc task list.
5. **External-resource grant**: "调取外部审计和LLM真题测试" — external Codex/Gemini audit dispatch + real-LLM benchmark runs explicitly authorized.

This is not a single-word `fix` / `go` / `ok` style approval (see
`feedback_class4_cannot_hide_in_class3`). The user named scope, allowed paths, the
forbidden list (verbatim 6-item universal forbidden list at the tail), and granted
external-resource access. CLAUDE.md §10 authorization semantics are satisfied:

| §10 Field | This authorization |
|-----------|-------------------|
| Scope | Stage A → Stage D in zh + en alignment docs (TB-18R Final → AMBER closure → HEAD_t C2 → benchmark scale-up → Polymarket P-M0..P-M9 → real-world readiness) |
| Allowed path | Constitutional Harness Engineering; harness-first; on-tape; constitution as tie-breaker |
| Forbidden path | "no f64, no ghost liquidity, no price-as-truth, no dashboard source-of-truth, no real funds, no public chain" + freeze list from alignment §2.1 |
| Risk class | Class 3/4 (ship-authorized for substantive items including TB-18R Final) |
| Audit required | YES — external Codex + Gemini explicitly authorized |
| Ship authorized | YES — "完成全部任务" with risk-class-appropriate gates |

## §2. Authorization context

This authorization comes after:

1. **Two architect alignment documents archived** (lossless, `/architect-ingest`)
   — `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md`.
   The user reviewed both (after they themselves drafted them) and asked them to be saved as
   "顶级架构师对齐文档" (top-tier architect alignment documents).

2. **TB-C0 SHIPPED FINAL 2026-05-07** — Constitutional Harness Engineering established as
   primary operating mode; Constitution Landing Gate is the canonical pre-merge invariant.
   FREEZE list lifted (TB-18R Final / TB-19+ / NodeMarket / Polymarket-signal / etc.) per
   `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md` §4.

3. **TB-18R FINAL ship report packaged** at `feec129` — `TB-18R_FINAL_SHIP_REPORT_2026-05-07.md`
   (FINAL-CANDIDATE status; awaits §8 sign-off; SG-18R.1..13 closure tabulated).

4. **Wave 3 evidence binding** at `feec129` — 7 matrix AMBER rows promoted to GREEN; gates
   90 → 97; workspace 1174 → 1181. Constitution Matrix at ~50/64 GREEN + ~13/64 AMBER + 0
   RED + 1 N/A.

5. **PROJECT_PLAN §3 = 10/10 GREEN** post-Wave-3-50p. §5 TB sequence resume eligibility
   door fully closed.

The authorization arrives after the user explicitly understood the full state and chose
"auto-execute the full plan with constitution as tie-breaker."

## §3. What this authorization grants

### §3.1. Stage A — Constitution landing closure (this session ship-eligible)

| Item | Class | Authorization |
|------|-------|---------------|
| **A1 / TB-18R FINAL ship** | Class 3/4 (already-packaged ship report; §8 the door condition) | YES — derives `2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md` from this authorization |
| **A2 / Constitution AMBER closure** | Class 1 (pure tests; harness hardening; no src/ writes if avoidable) | YES — execute as ship-eligible Class-1 work |
| **A3 / HEAD_t C2 multi-ref ChainTape** | Class 4 STEP_B (ledger surface) | Charter draft authorized; STEP_B execution requires per-atom architect sign-off going forward |

### §3.2. Stage B — Formal benchmark scale-up

| Item | Class | Authorization |
|------|-------|---------------|
| **B1 / 20p diagnostic** | Class 2 (already shipped 2026-05-07, ffb6ebd / a612cc9) | n/a — done |
| **B2 / 50p controlled** | Class 2 (already shipped 2026-05-07, a612cc9) | n/a — done |
| **B3 / 100p / M2** | Class 3 (LLM real-problem testing; multi-condition; scaled-benchmark) | YES — explicitly authorized; forms TB-18B charter |

### §3.3. Stage C — Polymarket / RSP-M

Authorization is GATED on Stage A green AND Stage B1 green (per zh-doc §3 / en-doc §4
Stage C; the alignment docs explicitly forbid starting executable trading "before
constitution gates and diagnostic benchmarks are stable").

| Phase | Class | Authorization |
|-------|-------|---------------|
| P-M0 quarantine legacy f64 CPMM | Class 1 (no-import gate test) | YES — pre-condition, charter-eligible after A1 ships |
| P-M1 CompleteSet hardening | Class 3 (production wire-up) | Charter-eligible after A green + B1 green |
| P-M2 CompleteSetMergeTx | Class 4 STEP_B (typed-tx + sequencer admission) | Charter-eligible; STEP_B per atom |
| P-M3 MarketSeed hardening | Class 3 | Charter-eligible |
| P-M4 LiquidityPool state | Class 3/4 | Charter-eligible |
| P-M5 share-only swap | Class 3 | Charter-eligible |
| P-M6 Mint-and-Swap router | Class 3/4 | Charter-eligible |
| P-M7 PriceIndex from CPMM | Class 2 (signal-only, must not override predicate) | Charter-eligible |
| P-M8 audit views | Class 1-2 | Charter-eligible |
| P-M9 controlled market smoke | Class 3 (real evidence run) | Charter-eligible after P-M0..P-M8 green |

### §3.4. Stage D — Real-world readiness

Authorization scope: **directive draft only this session**. Real-world activation requires
forward architect-side path decisions on oracle / challenge-court design / safety boundary.
Per zh-doc §3 / en-doc §4 Stage D: "no real-world task before oracle / challenge / delayed
settlement / human escalation / irreversible-action ban / safety boundary."

## §4. Universal forbidden list (architect verbatim)

This forbidden list applies at every phase and overrides any feature-development
convenience:

```
- no f64
- no ghost liquidity
- no price-as-truth
- no dashboard source-of-truth
- no real funds
- no public chain
```

Plus, per zh-doc §6 / en-doc §8 Polymarket forbidden list:

```
- no automatic per-node 100 YES + 100 NO without collateral
- no Treasury magic seed without debit
- no DPMM / pro-rata payout inside CTF track
- no price-based settlement
- no agent-submitted MarketResolveTx
- no agent-submitted system resolution
- no AMM before CompleteSet
- no trading before audit tools
- no public chain before sandbox
- no real money before readiness gate
```

A violation of any forbidden item REVERTS this authorization for the offending atom and
requires explicit re-ratification.

## §5. Edge-case rule (constitution-as-tiebreaker)

Per architect message §1 clause 3 ("如果处于边缘问题，通过宪法解决"): when the alignment docs
under-specify an engineering decision, resolve via constitution.md in the supreme
source-of-truth order from CLAUDE.md §1:

```
constitution > flowcharts > ChainTape/CAS > executable gates > reports
```

Operationally: AI coder may make non-Class-4 design decisions autonomously when both
alignment docs are silent AND constitution.md / FC1/FC2/FC3 / executable gates do not
contradict. For Class-4 surfaces (sequencer admission / typed-tx schema / canonical
signing payload / RootBox), STEP_B + per-atom architect sign-off is still required —
this authorization does NOT collapse Class-4 ratification gates.

## §6. External resources explicitly authorized

Per architect message §1 clause 5 ("调取外部审计和LLM真题测试"):

| Resource | Status |
|----------|--------|
| Codex external audit dispatch | AUTHORIZED — Class-3/4 atoms must still receive dual audit per `feedback_dual_audit` (kernel-only Class-1 = self OK; production wire-up + economic mutator = full dual) |
| Gemini external audit dispatch | AUTHORIZED — pending Track C carry-over (`run_gemini_constitution_landing_first_sanity_2026-05-07.py` Codex task `task-movpt3ux-qfvx8l`) and forward dispatches |
| LLM real-problem testing (DeepSeek / SiliconFlow / etc.) | AUTHORIZED — for benchmark Stage B3 + Polymarket P-M9 controlled smoke + diagnostic runs as needed |
| MiniF2F / Mathlib / Putnam / IMO / research-paper / web-derived problem sets | AUTHORIZED — per `feedback_real_problems_not_designed`: real public problems preferred; synthesis forbidden |

## §7. What this authorization does NOT grant

- **Constitution edits (Art. V.1.1 sudo)** — still requires explicit human-architect-only
  authorization on `constitution.md` itself + Phase Z′ rerun + §5.3 amendment log.
- **Class-4 typed-tx schema bumps without STEP_B** — Class-4 surfaces in CLAUDE.md §12
  STEP_B list (`src/kernel.rs`, `src/bus.rs`, `src/sdk/tools/wallet.rs`,
  `src/state/sequencer.rs`, `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`,
  canonical signing payloads) still require parallel-branch A/B per CLAUDE.md Code
  Standard.
- **Real-money / public-chain / external-domain activation** — see Stage D §3.4 above.
- **Bypass of `/runner-preflight` 7-stage gate** — runner scripts that mutate
  `handover/evidence/` or run real evaluation still require `/runner-preflight` per
  CLAUDE.md §11.
- **Bypass of `/harness-reflect`** — after TB SHIPPED FINAL or audit rounds > 3, the
  reflection cadence remains binding per `feedback_harness_reflect_cadence`.
- **Retroactive evidence rewrite** — `feedback_no_retroactive_evidence_rewrite` remains in
  force; new requirements apply going-forward only.

## §8. Forward-bound (this session and beyond)

This session (2026-05-07 session #18) priority order, derived from architect "Immediate
priorities" 1-4:

1. **TB-18R FINAL §8 sign-off** (immediate priority #1) — derive
   `handover/directives/2026-05-07_TB18R_FINAL_§8_SIGN_OFF.md` from this authorization.
2. **Constitution AMBER closure** (immediate priority #2) — Class-1 harness hardening.
3. **FC1/FC2/FC3 gates green** (immediate priority #3) — re-validate after each change via
   `cargo test --workspace` + `scripts/run_constitution_gates.sh`.
4. **Polymarket gating** (immediate priority #4) — frozen until A green + B1 green; Stage C
   charter draft permitted but executable feature work is forward.
5. **External audits** — dispatch Gemini sanity pass (Track C carry-over from session #17).
6. **Forward charters** — A3 (HEAD_t C2) + B3 (TB-18B M1/M2) + C (Polymarket P-M0..P-M9)
   + D (real-world readiness) drafted as separate documents under
   `handover/tracer_bullets/` or `handover/directives/`.

## §9. Cross-references

- Architect alignment docs: `handover/architect-insights/2026-05-07_ARCHITECT_ALIGNMENT_AUDIT_LAUNCH_POLYMARKET_MANUAL_{zh,en}.md`
- Constitution: `constitution.md`
- Three-flowchart matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- Trace flowchart matrix: `handover/alignment/TRACE_FLOWCHART_MATRIX.md`
- TB-C0 §8 precedent: `handover/directives/2026-05-07_TBC0_ARCHITECT_§8_SIGN_OFF.md`
- TB-18R FINAL ship report: `handover/tracer_bullets/TB-18R_FINAL_SHIP_REPORT_2026-05-07.md`
- LATEST handover: `handover/ai-direct/LATEST.md`
- TB log: `handover/tracer_bullets/TB_LOG.tsv`

---

**This authorization is ACTIVE 2026-05-07.** AI coder proceeds with Stage A1..D under the
scope, allowed paths, forbidden list, edge-case rule, and external-resource grant
documented above.
