# Art-0.2 Full-Close Design — model provenance reconstructible from frozen tape

**Status:** ✅ RATIFIED 2026-06-15 (architect audit ruling #1: "批准
`ProposalTelemetry.model_id`"). Implementation QUEUED behind the active serial
PPUT batch (no carrier-source edits during an active run — AGENTS.md §8).

### RATIFIED IMPLEMENTATION CONTRACT (architect 2026-06-15)

Exact approved form — implement, then clean-context Veto-AI ({PASS,VETO}):
1. `#[serde(default)] pub model_id: Option<String>` added to `ProposalTelemetry`.
2. Schema id bump `turingosv4.proposal_telemetry.v1` → `…v2`; v1 kept as a legacy
   decode path (try v2, fall back to v1 with `model_id=None`) — mirror
   `canonical_decode_typed_tx_current_or_legacy_event_resolve`. v1 schema-id
   string stays in EVERY gate/allowlist that matches it (add v2 alongside).
3. **Historical replay-equivalence test** (REQUIRED): a test proving pre-v2
   ProposalTelemetry CAS objects still decode + replay byte-equivalent under the
   v2 decoder (no historical-evidence break — AGENTS.md §8).
4. Carrier `put_proposal` populates `model_id = agent_models[ai]`.
5. Field-count unit test 9→10; `schema_validity_*` updated.
6. Gate-D negative-witness `model_id_is_not_a_field_on_carrier_cas_objects`
   FLIPS to a positive recompute assertion: model_id IS on the CAS object AND
   cost = rate(model_id)×tokens recomputes byte-equal from the frozen tape (a
   real §17.1-G1 recompute closure). Matrix row + GAP doc AMBER→GREEN.
7. Manifest/hash update for the new gate state.
8. **Art 0.4 path declaration** (architect requirement): the commit message must
   state which A/B/C version-control path this tape/schema change adopts, and
   must not silently lower fidelity. `model_id` is local hemostasis only; the
   Art 0.4 substrate debt (HEAD_t unimplemented, tape_t partial, no runtime git)
   remains open and is NOT closed by this change.
9. Clean-context Veto-AI audit (domain {PASS,VETO}) after implementation.

This change is hard-gate 5.1 of TB_DYNAMIC_MODEL_BUDGET_MARKET (the next
experiment cannot count per-model results without it).

---

**(original design below — superseded by the ratified contract above where they differ)**

**Status (original):** DESIGN / awaiting §8 ratification (Class 3, binding-schema change).
**Authority:** architect 2026-06-14 audit §5 Gate D + architect 2026-06-15 ruling
"最重要的是符合宪法 … 从头就合规来，哪怕我们做慢一点" (do it compliant from the
foundation, not an expedient patch that leaves a bigger hole).
**Resolves:** `handover/audits/TAPE_RECONSTRUCTIBILITY_GAP_2026-06-14.md` (the
Gate-D documented GAP: model_id / provider / per-call cost NOT reconstructible
from ChainTape+CAS alone).
**Carrier:** `src/bin/lean_market_agent.rs --policy autonomous_market`.

---

## 1. The gap (one paragraph)

The H-HET-1 central claim is per-model attribution: "cross-lab model X solved a
theorem homogeneous model Y could not." For that claim to rest on the frozen
tape (Art 0.2 Tape-Canonical), every token-bearing proposal must let a replayer
recover WHICH model produced it. Today it cannot: the carrier's token-bearing
CAS object `ProposalTelemetry` has no model field; model identity lives only in
the Manifest sidecar roster + the deterministic round-robin assignment rule.
Cost (Σ rate(model) × tokens) is therefore not recomputable from the tape alone.

## 2. Rejected alternatives (and why)

- **Scope-out** (drop model/cost from the Art-0.2 headline): rejected by the
  architect — it narrows the very claim H-HET-1 exists to make, and is the
  "凑合留下漏洞" path.
- **Roster-hash + round-robin inference (B+)**: anchor `agent_models[]` + the
  rule on the tape, reconstruct model_id by (node index → rule). Rejected:
  reconstruction-by-rule is fragile — it silently breaks the moment assignment
  deviates (a retry on a different model, a skip, a fallback). That is exactly a
  latent hole, not a foundation.

## 3. Chosen design — model_id on the per-proposal CAS object (FULL CLOSE)

Put model identity next to `token_counts` on `ProposalTelemetry`, so the chain
path `WorkTx → proposal_cid → ProposalTelemetry{ model_id, token_counts }`
makes both attribution AND cost recomputable from the frozen tape with no
sidecar and no inference.

### 3.1 Schema change (the part that needs §8)

`ProposalTelemetry` carries a binding "do NOT add fields without architect
ratification" contract. The one prior extension (`verification_result_cid`) was
ratified as TB-7.7 D4 and added as `#[serde(default)] Option<…>`. This change
follows that precedent but MUST also version the wire format, because
`canonical_encode` is bincode `standard().with_big_endian()` — non-self-
describing and positional, so a naive trailing field breaks decode of every
historical 9-field record (would violate AGENTS.md §8 "never break historical
evidence reconstructability").

Proposed:

```rust
// ProposalTelemetry, additive:
#[serde(default)]
pub model_id: Option<String>,   // vendor model string, e.g. "Qwen/Qwen3-32B".
                                // Provenance metadata (same category as agent_id),
                                // NOT deliberation/raw content — passes the §6
                                // forbidden-field guard.
```

- New schema id `turingosv4.proposal_telemetry.v2`; the v1 id is kept as a
  legacy decode path. `read_from_cas` tries v2, falls back to v1 (model_id =
  None) — mirrors `canonical_decode_typed_tx_current_or_legacy_event_resolve`.
  Historical P1/G-series telemetry still replays (model_id None = "unknown
  pre-v2"), fresh carrier telemetry is v2 with model_id populated.
- Field-count unit test 9 → 10; `schema_validity_*` updated.

### 3.2 Carrier wiring (Class 2, no §8)

`put_proposal` already receives the proposal's `agent`/`tokens`; thread the
model string (`agent_models[ai]`) into
`ProposalTelemetry::build_for_evaluator_append_with_parent` → set `model_id`.

### 3.3 Gate flip (Class 1/2)

`tests/constitution_het_tape_reconstructibility.rs`:
`model_id_is_not_a_field_on_carrier_cas_objects` (negative witness) → flips to a
POSITIVE assertion: model_id IS a field on the carrier's token-bearing CAS
object, AND cost = rate(model_id) × tokens recomputes byte-equal from the frozen
tape (a real §17.1-G1 recompute-from-tape closure, not a sidecar read). Matrix
row + GAP doc move AMBER → GREEN.

## 4. Blast radius / gate impact (must all stay green)

schema_id `turingosv4.proposal_telemetry.v1` is referenced by (verified
2026-06-15): `constitution_librarian_market_no_trade.rs:207`,
`constitution_true_suite_generate_artifact_runner.rs:363`,
`constitution_shielding_evidence_binding.rs:377` (4096B size cap — model name is
short, stays under), `constitution_het_tape_reconstructibility.rs`,
`generate_emits_work_tx_smoke.rs:465`, `src/runtime/librarian_broadcast.rs:495`,
`src/top_white/predicates/registry.rs:1186`. v2 must be added alongside v1 in
each schema-id allowlist/matcher, NOT replace it.

## 5. Risk class & required process

Class 3 (CAS object schema + production-evidence wire + canonical encoding).
Per AGENTS.md §14 cadence: TB charter + **per-atom §8** + real evidence
(fresh carrier run emitting v2 telemetry, replayed) + clean-context §9 audit.
This is NOT in the freeze commit `f73163f4`; it lands as the next atom on
`claude/het-carrier-freeze`. The full paid H-HET-1 experiment stays BLOCKED
until this closes + the hard-gated pilot passes + prereg sign-off.

## 5a. Convergence: the §8 schema change is ALSO the next experiment's instrumentation

The H-HET-1 carrier pilot (2026-06-15) + its adversarial audit (wf_fd1ba89f, all
lenses NO-VIOLATION) independently concluded the SAME thing this §8 change does:
the pilot's per-model attribution (DS 0/28, GLM 0/19, Q397 8/10) rests on a
round-robin **sidecar inference** (agent index → roster), because no per-node
model field exists on the tape. The audit's #1 hardening recommendation —
"add a true per-node {model,vendor} field to retire the sidecar inference" — is
exactly this `model_id` close. So the change is dual-purpose: it satisfies Art
0.2 (tape-canonical) AND gives the next experiment (dynamic model-budget market)
clean, non-inferred per-proposal model attribution. This strengthens, not
weakens, the case for doing it properly via §8 rather than a sidecar patch.

Note a cheaper, NON-§8 partial: adding `AttemptNode.model` (the bin-private
Manifest summary struct, like `action_source`) would make manifest attribution
explicit-not-inferred WITHOUT touching the binding CAS schema — but it does NOT
close Art 0.2 (model still absent from the frozen ProposalTelemetry CAS object).
The full tape-canonical close still requires the §8 ProposalTelemetry change.

## 6. The single decision needed now

**§8 ratification to extend the binding `ProposalTelemetry` schema with an
additive, version-bumped `model_id: Option<String>` (v1→v2, v1 kept as legacy
decode), per §3.** On sign-off I will: write the TB charter, implement gate-
first (flip Gate-D to the positive recompute assertion), produce the real
fresh-run evidence, and request the clean-context §9 audit — then return for
the pilot go/no-go. No schema bytes change before §8.
