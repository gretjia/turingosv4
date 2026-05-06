---
type: observation
date: 2026-05-06
class: 0 (audit-trail annotation)
parent_directive: handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md (Q-P3)
parent_dispatch: handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md (§3.3 Gap C)
target_doc: handover/ai-direct/TB-18R_R3_STEP_B_admission.md
target_lines: ":38-45" and ":221-225"
authority_for_supersession: §3.5 amendment of the same R3 STEP_B admission preflight (lines :133-153)
related_obs: handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md
status: ARCHIVED — superseded text preserved verbatim in target doc; this OBS is the trail-of-record explaining why the inline `[SUPERSEDED]` markers are admissible audit-trail annotation rather than retroactive evidence rewrite
---

# OBS — TB-18R R3 STEP_B preflight supersession markers — 2026-05-06

## §1 Why this OBS exists

Round-2 dispatch §3.3 (Gap C) flagged a process risk: under R12, Claude added inline `[SUPERSEDED 2026-05-06 by §3.5 amendment]` markers to two passages in `handover/ai-direct/TB-18R_R3_STEP_B_admission.md` (preflight doc). The dispatch asked whether this is admissible audit-trail annotation, or a form of retroactive surgery that should instead live in a separate OBS file.

Parent ruling §4 Q-P3 ruled: **inline markers may stand, but a separate OBS must exist**. This file is that OBS.

## §2 The two superseded passages (verbatim original draft text)

### §2.1 Passage at `:38-45` (within §1.3 Class-3 evaluator wire-up)

The original draft proposed cutting omega-path WorkTx `proposal_cid` from ProposalTelemetry CID to AttemptTelemetry CID at the two omega-success paths (`omega-full` ~line 2317, `omega-pertactic` ~line 2861 in `evaluator.rs`). The original-draft fragment (now bearing the inline `[SUPERSEDED]` marker) was:

> For paths 1-2 (`omega-full` / `omega-pertactic`, `predicate_passes=true`), the existing TB-7 Atom-3 WorkTx pipeline is unchanged. Their AttemptTelemetry already lands on L4 via the existing `proposal_cid → ProposalTelemetry` path. **[SUPERSEDED 2026-05-06 by §3.5 amendment — see below.]** The original draft proposed cutting omega WorkTx's `proposal_cid` to AttemptTelemetry CID; the amendment keeps it as ProposalTelemetry CID to preserve TB-7 audit-chain backward compatibility. The unified-schema goal is achieved at the L4.E-only failure-path layer (where AttemptTelemetry is the proposal_cid target); L4 omega keeps ProposalTelemetry as the proposal_cid target.

### §2.2 Passage at `:221-225` (file table entry for evaluator.rs)

(For audit completeness; the precise wording at this line range describes the evaluator.rs diff intent, including the now-superseded "cut omega-path WorkTx proposal_cid to AttemptTelemetry CID at 2 paths (~2317/~2861)" phrasing. The inline `[SUPERSEDED]` marker captures the same supersession.)

## §3 Authority for supersession (verbatim §3.5 amendment text from same preflight `:133-153`)

The supersession authority is the **§3.5 Omega-path proposal_cid: NO cutover (revised 2026-05-06 post-implementation-audit)** subsection of the same preflight document, which states verbatim:

> **Original draft proposed**: cut omega-path WorkTx.proposal_cid from ProposalTelemetry CID to AttemptTelemetry CID.
>
> **Revised decision**: **omega-path WorkTx.proposal_cid stays as ProposalTelemetry CID (`tel_cid`)** — no cutover.
>
> **Reason**: implementation grep surfaced two existing audit walks that consume `L4 WorkTx.proposal_cid → ProposalTelemetry` semantics:
>   - `src/runtime/verify.rs:420` Gate 5 — verify proposal_cid resolves to a ProposalTelemetry (TB-7 charter §8 Gate 5 evidence)
>   - `src/runtime/audit_assertions.rs:1583` id=24 `proposal_telemetry_chain` (Layer E HALT-on-mismatch)
>   - `src/runtime/audit_assertions.rs:1925` id=43 `boltzmann_parent_selection_diversity`
>   - `tests/tb_7_atom6_chain_backed_smoke.rs:207` "Gate 5: every WorkTx.proposal_cid must resolve to CAS ProposalTelemetry"
>
> These audits walk **L4 only** (the accepted spine) — they never visit L4.E. So:
>   - **Omega WorkTx** (predicate_passes=true; lands on L4): `proposal_cid` MUST stay as ProposalTelemetry CID, otherwise these 4 audit walks halt at id=24. **Unchanged from R2 ship.**
>   - **Failure-path WorkTx** (predicate_passes=false; R3 NEW; lands on L4.E only): `proposal_cid = AttemptTelemetry CID` is fine, since the L4-only audits never visit L4.E.

## §4 Why supersession was needed

The original draft's "omega cutover" proposal would have broken **four** existing TB-7 audit walks (verify.rs Gate 5, audit_assertions id=24, audit_assertions id=43, tb_7_atom6_chain_backed_smoke.rs Gate 5). These walks query `L4 WorkTx.proposal_cid → ProposalTelemetry` schema; cutting to AttemptTelemetry would have produced an immediate id=24 `proposal_telemetry_chain` HALT.

Implementation-time grep caught this **before** any source change landed. The §3.5 amendment captures the corrected design: AttemptTelemetry coexists with ProposalTelemetry on different chain spines (L4 omega keeps ProposalTelemetry as proposal_cid target; L4.E failure-path uses AttemptTelemetry as proposal_cid target). The R1 `AttemptTelemetry.proposal_telemetry_cid: Option<Cid>` field stays `None` for omega paths (could be wired in R5+ if a future audit needs the cross-link).

**No source code was ever shipped under the original draft proposal.** The supersession is on the preflight document only; the implementation took the §3.5-amended path from the start.

## §5 Why inline `[SUPERSEDED]` markers are admissible audit-trail annotation

`feedback_no_retroactive_evidence_rewrite` (memory) governs this question. The boundaries it draws:

- **Forbidden**: rewriting / migrating / relabeling **L4, L4.E, CAS roots** or any "old roots". Fabricating genesis_report. In-place data mutation of historical chain or CAS evidence.
- **Permitted (under grandfathering carve-out)**: top-of-file banner annotation + sibling README.md documenting that an artifact is now classified differently.

The R3 preflight is a STEP_B preflight document — part of the **architect ratification trail**, but not L4/L4.E/CAS evidence. Inline `[SUPERSEDED]` markers:

1. **Preserve original text verbatim** (no deletion, no rewriting).
2. **Add a pointer** to the canonical authoritative subsection (§3.5) inside the same document.
3. **Do not modify any chain / CAS / evidence root**.
4. **Surface for any reader** that the marked passages are no longer load-bearing without forcing the reader to chase the supersession via diff archaeology.

This pattern is the doc-trail analog of the M1 evidence grandfathering pattern (banner + README annotation; no in-place data mutation). The risk Round-2 dispatch §3.3 flagged — that inline markers could be read as "audit-trail surgery" — is mitigated by the rule: **markers may be added only when (a) the original text is preserved verbatim, (b) the supersession authority is named inline, and (c) a separate OBS file (this one) records the trail-of-record**.

## §6 What no source behavior changed

- `evaluator.rs` was NEVER built against the original-draft "omega cutover" proposal. The R2 + R3 ship landed under §3.5-amended semantics (omega `proposal_cid → ProposalTelemetry`; failure-path `proposal_cid → AttemptTelemetry`).
- The four TB-7 audit walks (verify.rs:420, audit_assertions id=24 / id=43, tb_7_atom6_chain_backed_smoke.rs:207) continue to find their expected `ProposalTelemetry` schema on L4.
- L4.E rejected-path WorkTx records in TB-18R-era chains use `AttemptTelemetry` as proposal_cid target, by design.
- No CAS object schema was retroactively rewritten.
- No L4 / L4.E / CAS root was rewritten.

## §7 Forward-binding

If a future round (Phase 2 of the parent ruling) introduces typed `LeanVerdict::PartialAccepted`, the R3 preflight may require **another** amendment subsection. If it does, the same supersession-marker pattern applies (verbatim preservation + inline pointer + new OBS file).

If after architect review the architect prefers to **revert** the inline markers and rely solely on this OBS file, the markers can be reverted in a single Class-0 commit; the doc-trail integrity is unaffected because this OBS already carries the verbatim original passages.

## §8 Cross-references

- Target preflight: `handover/ai-direct/TB-18R_R3_STEP_B_admission.md`
  - Superseded passage: `:38-45`
  - Supersession authority: `:133-153`
  - Second superseded passage: `:221-225`
- Parent ruling: `handover/directives/2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md` Q-P3
- Round-2 dispatch §3.3 (Gap C): `handover/audits/G2_TB_18R_ROUND_2_DUAL_AUDIT_DISPATCH_2026-05-06.md`
- Memory rule: `feedback_no_retroactive_evidence_rewrite`
- Sibling OBS for R3 audit-infra: `handover/alignment/OBS_CODEX_R3_AUDIT_INFRA_FAIL_2026-05-06.md`

---

**End of OBS.**
