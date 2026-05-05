# ORACLE_REQUIREMENTS — TuringOS v4 P7 oracle architecture

**Status**: TB-17 atom 3 — autonomous Class 0 draft.
**Filed**: 2026-05-05.
**Authority**: TB-17 charter §3 atom 3 + 2026-05-05 architect verdict §B.5 (FR-17.3, FR-17.4, FR-17.5) + §B.6 (CR-17.1, CR-17.5) + §B.7 (SG-17.4) + Q6.1 verbatim attack-surface list.

---

## §1 Oracle definition (CR-17.1 + CR-17.5)

An **oracle** is a deterministic predicate or human-attested verdict that maps a (task, candidate-solution) pair to a boolean (or finite verdict set) outcome. Oracles in TuringOS:

1. **MUST be deterministic OR canonically attested**.
2. **MUST NOT be price-as-truth** (CR-17.5 verbatim from TB-14 carry-forward).
3. **MUST be replayable** from canonical chain state + CAS evidence.
4. **MUST have a CAS-resident output object** (the oracle output IS evidence).
5. **MUST produce a verdict OR a `Refuse` non-availability response** (NEVER silent fallback to default-pass; CR-17.14 fail-safe).

```
Predicate signature (conceptual):
  oracle: (TaskSpec, CandidateSolution, OracleContext) → OracleVerdict
  OracleVerdict ∈ {Verified, Refuted, Refuse(reason)}

Verified  — predicate holds; admission allowed
Refuted   — predicate fails; rejection to L4.E with evidence
Refuse    — oracle non-available / outside-scope; admission BLOCKS until resolved
```

**Anti-pattern explicitly forbidden**: returning `Verified` on internal error. The `Refuse` branch exists so the system never has to lie.

---

## §2 Per-tier oracle architecture (FR-17.3; SG-17.4)

Cross-references DOMAIN_SELECTION_CRITERIA.md §3 tier × oracle matrix.

### §2.1 T1 — single deterministic predicate

- Architecture: one process / function call.
- Source: pinned binary or pure-fn implementation.
- Output: single OracleVerdict written to CAS as `ObjectType::OracleAttestation`.
- Disagreement: N/A (single source).
- Attack surface: oracle implementation bug (mitigated by C-012 oracle freeze + version pinning).

### §2.2 T2 — single trusted deterministic oracle (canonical TuringOS sweet spot)

- Architecture: external proof-checker / hash-pinned-corpus / pinned-environment harness.
- **Reference example**: `lean --run` with pinned Lean version + Mathlib via `lake exe cache get`.
- Source identity: oracle binary's hash + version string MUST be recorded in the OracleAttestation.
- Output: OracleVerdict + verifier-runtime-metadata (exit code, stdout hash, wall-clock duration) written to CAS.
- Disagreement: N/A (single canonical source).
- Attack surface: oracle implementation bug; oracle-version drift (mitigated by pinning); solver targets oracle-implementation rather than task (mitigated by predicate-determinism).

### §2.3 T3 — multi-oracle quorum

- Architecture: ≥3 independent oracle instances; each produces an OracleAttestation.
- Quorum rule: median (for ordinal verdicts) or 2-of-3 majority (for binary verdicts).
- Disagreement protocol: per FR-17.5; unresolved-disagreement → human escalation per atom 5.
- Output: each OracleAttestation written to CAS; quorum verdict written as `ObjectType::OracleQuorumVerdict` referencing the constituent attestations by Cid.
- Attack surface: collusion (mitigated by oracle-source diversity requirement); oracle-implementation correlated bug (mitigated by source diversity); cost asymmetry (multi-oracle is expensive — settlement window must permit it).

### §2.4 T4 — multi-oracle + adversarial-oracle + default-on human review

- **Status**: PROHIBITED as initial pilot per DOMAIN_SELECTION_CRITERIA.md §2.5.
- Specification placeholder for future architect-authored design:
  - ≥3 collaborative oracles + ≥1 adversarial oracle (specifically incentivized to find counterexamples).
  - Human escalation default-on for non-trivial verdicts.
  - Extended challenge window per atom 4.
  - Sandbox-vs-LIVE label MUST remain SANDBOX or SHADOW until extensive smoke evidence accrues.
- This document does NOT prescribe a T4 architecture; it documents the requirement that a T4 architecture cannot be drafted by AI-coder alone.

---

## §3 Oracle provenance (FR-17.4)

Every OracleAttestation MUST carry:

| Field | Description | Required? |
|---|---|---|
| `oracle_id` | identifier for the oracle source (binary path / human attester / corpus-snapshot Cid) | ✅ |
| `oracle_version` | version string OR binary hash | ✅ |
| `task_spec_cid` | Cid of the TaskSpec being verified | ✅ |
| `candidate_solution_cid` | Cid of the candidate solution | ✅ |
| `verdict` | `Verified` / `Refuted` / `Refuse(reason)` | ✅ |
| `attested_at_logical_t` | logical timestamp in chain | ✅ |
| `attested_at_wallclock_unix` | wallclock for forensics; NOT canonical | ✅ |
| `attestation_signature` | signature by oracle key (for human or remote oracles) | ✅ for non-deterministic oracles |
| `evidence_cids` | Vec<Cid> of supporting evidence (e.g., proof-checker stdout/stderr) | ≥1 required |
| `replayable` | bool: can oracle output be re-derived from `(oracle_version, task_spec_cid, candidate_solution_cid)` | ✅ (must be true for T1/T2) |

**Provenance identity rule**: any field whose change would alter the verdict MUST be in the OracleAttestation. The Cid of the OracleAttestation IS its provenance hash.

**Replayability requirement** (CR-17.10 + SG-17.20): T1/T2 oracle outputs MUST be reproducible byte-identical given pinned-version oracle and the same input Cids. T3 outputs need not be byte-identical (multi-oracle quorum has cost-and-time asymmetry) but MUST be statistically reproducible (same quorum verdict on rerun).

**Hash/CID evidence chain**:

```
TaskSpec (Cid A)  +  CandidateSolution (Cid B)
  ↓ (oracle invocation)
OracleAttestation (Cid C, references A + B)
  ↓ (admission)
L4 entry references C
```

Anyone replaying the chain can re-derive C from (oracle_version, A, B) for T1/T2 and verify equality.

---

## §4 Oracle disagreement policy (FR-17.5)

(T3 + T4 only; T1/T2 single-source by definition.)

### §4.1 Multi-oracle quorum rules

- **Binary verdict**: 2-of-3 majority. If 1-of-3 or 0-of-3, escalate.
- **Ordinal verdict** (e.g., quality scores): median; if interquartile range > threshold (config), escalate.
- **Multi-oracle non-availability**: if any oracle returns `Refuse`, treat the missing oracle as non-vote; quorum requires ≥(⌈n/2⌉+1) of the remaining respondents to agree.

### §4.2 Median / majority rule formal spec

```
fn quorum_verdict(attestations: Vec<OracleAttestation>, rule: QuorumRule)
    -> QuorumOutcome
{
    let votes = attestations.iter().filter(|a| !is_refuse(a.verdict));
    let respondent_count = votes.len();
    let total = attestations.len();
    let refuse_count = total - respondent_count;

    if respondent_count < ceil_div(total, 2) + 1 {
        return QuorumOutcome::EscalateNonAvailability;
    }

    match rule {
        QuorumRule::BinaryMajority => binary_majority(votes),
        QuorumRule::OrdinalMedian  => ordinal_median(votes),
    }
}
```

(This is a spec; not implemented in TB-17. Atom 8 comprehensive_arena MAY exercise a stub if a multi-oracle scenario is engineered; otherwise this lands in a future TB targeting T3 admission.)

### §4.3 Manual escalation conditions (FR-17.5 + FR-17.8 + CR-17.6)

Escalation to human RootBox triggers when ANY of:

| Condition | Action |
|---|---|
| Quorum non-availability (insufficient respondents) | freeze pending; SAFETY_BOUNDARY.md timeout starts |
| Quorum produced but interquartile range > threshold | freeze pending; flag for manual review |
| One or more oracles signed contradictory verdicts on identical input | freeze pending; investigate oracle key compromise / version drift |
| Oracle output is non-replayable when it should be (T1/T2 byte-mismatch on rerun) | freeze pending; investigate non-determinism source |
| Adversarial oracle (T4) reports counterexample | freeze pending; human review mandatory |

Escalation defaults: per CR-17.14 "fail-safe, not fail-open" — pending tasks BLOCK until escalation resolves; settlement WAITS; no silent fallback to "auto-pass".

---

## §5 Oracle CAS contract

```
ObjectType::OracleAttestation       — single attestation
ObjectType::OracleQuorumVerdict     — T3 quorum result, references constituent attestations
ObjectType::OracleVersionManifest   — pinned-version manifest for the oracle binary / corpus snapshot

Storage rules:
  - All three are CAS-resident, content-addressed, immutable.
  - Version manifests are emitted ONCE per oracle deployment + on each version bump;
    OracleAttestations reference manifest by Cid.
  - L4 admission entries cite the OracleQuorumVerdict (T3) or OracleAttestation (T1/T2)
    Cid in the admission record.
```

CAS rooting integrates with TB-15 MarkovEvidenceCapsule.cas_root; oracle outputs are part of the chain's auditable surface.

---

## §6 Oracle non-availability protocol (CR-17.14)

When the oracle cannot be invoked (network error, pinned binary missing, corpus snapshot unfetchable):

1. Oracle returns `Refuse(reason)` with reason string carrying enough detail for forensic analysis (e.g., `Refuse("oracle_binary_hash_mismatch: expected X, found Y")`).
2. Sequencer admission BLOCKS the dependent transaction; rejection class = `OracleUnavailable`; entry written to L4.E with the Refuse reason in `raw_diagnostic` (privacy-shielded per `feedback_rejection_evidence_separate`).
3. NEVER auto-retry with a different oracle source unless the multi-oracle quorum protocol explicitly authorizes it (FR-17.5 §4.1).
4. NEVER silently degrade to "auto-pass" or "best-effort".
5. Operator surface: BLOCK condition raises an alert via the dashboard (atom 5 SAFETY_BOUNDARY layer); human review optional (T1/T2) or required (T3/T4).

---

## §7 No-oracle-no-domain rule (CR-17.1 verbatim)

```
CR-17.1: No real-world domain without explicit oracle design.
```

Concretely: any future TB that proposes admitting a real-world task category MUST include an oracle architecture in its charter that satisfies §2-§6 of this document. Charter without oracle architecture = REJECT before commit (per `feedback_tb_phase_tag_required` analog enforcement).

For the D1 Lean pilot (DOMAIN_SELECTION_CRITERIA.md §6), the oracle architecture is §2.2 + the existing `lean --run` harness already deployed in TB-7 / TB-16 sandbox arena. No new oracle wiring needed; only TB-19 charter must record the architecture cite.

---

## §8 Oracle attack surface (architect Q6.1 verbatim — Claude's missed concern)

Per architect Q6.1 verbatim list. Each attack class gets a defense / mitigation pointer.

### §8.1 Oracle manipulation

**Attack**: adversary modifies the oracle binary or its inputs to produce false verdicts.

**Mitigation**:
- Binary hash pinning (oracle_version field MUST match pinned manifest).
- C-012 oracle freeze (constitution): oracle binary cannot be silently upgraded mid-experiment.
- For human oracles: signature verification on every attestation.
- For corpus oracles: snapshot Cid in OracleVersionManifest.

**Detection**: replayability check (T1/T2 byte-mismatch flags drift); version drift between attestations (hash compare).

### §8.2 Oracle provenance forgery

**Attack**: adversary submits an OracleAttestation that wasn't actually produced by the named oracle (e.g., via signature forgery or compromised key).

**Mitigation**:
- Signature verification on every non-deterministic OracleAttestation.
- Key rotation per atom 6 IRREVERSIBLE_ACTION_POLICY §2 subtype #8 (`credential_key_rotation`).
- For deterministic oracles: replay-recompute on dispute; provenance forgery cannot survive replay.

**Detection**: signature verification at admission time; replay verification post-hoc.

### §8.3 Oracle replayability gap

**Attack**: oracle is non-deterministic when it should be (e.g., LLM oracle with non-zero temperature; or live-web fetch of a mutable page).

**Mitigation**:
- Pin all sources (binary versions, corpus snapshots, page hashes).
- LLM oracles: `temperature=0` + pinned-model + record full request/response in evidence_cids.
- Live web oracles: fetch a hash-pinned snapshot; log wallclock fetch time + response Cid in evidence_cids; replay cross-checks the response Cid.

**Detection**: replay-byte-mismatch; wall-clock asymmetry > threshold flags non-determinism source.

### §8.4 Oracle latency

**Attack**: oracle takes too long, causing spurious BLOCK verdicts when input is actually good.

**Mitigation**:
- Per-tier latency budget (T1: 100ms; T2: 1s; T3: 60s; T4: hours).
- Latency exceeded → `Refuse(timeout)`, NOT silent default-pass.
- TaskSpec MAY include latency tolerance override (with architect ratification).

**Detection**: oracle latency histogram in dashboard atom 5 surface; budget-exceedance count tracked in EvidenceCapsule.

### §8.5 Oracle disagreement (handled in §4)

Already covered. Cross-reference here for completeness of architect Q6.1 list.

### §8.6 Oracle challenge evidence format

**Attack**: a challenger submits a ChallengeTx but cannot produce evidence in a form the oracle understands; or evidence is intentionally malformed to exhaust challenge resolution.

**Mitigation**:
- Challenge evidence format SPEC'd per oracle in atom 4 CHALLENGE_COURT_REQUIREMENTS.md §2.
- Evidence schema validated at admission; malformed evidence → L4.E with `EvidenceMalformed` rejection class.
- Challenge bond per atom 4 §X disincentivizes spurious challenges.

**Detection**: oracle returns `Refuse(evidence_format_invalid)`; challenge ratio metric in dashboard.

---

## §9 Cross-references

- DOMAIN_SELECTION_CRITERIA.md §3 (tier × oracle matrix).
- CHALLENGE_COURT_REQUIREMENTS.md atom 4 (evidence format; resolution authority).
- SAFETY_BOUNDARY.md atom 5 (escalation timeout; human RootBox).
- IRREVERSIBLE_ACTION_POLICY.md atom 6 (subtype #8 credential rotation).
- 2026-05-05 architect verdict §B.5 (FR-17.3..5) + §B.6 (CR-17.1, CR-17.5) + §B.7 (SG-17.4) + Q6.1.
- Constitution C-012 (oracle freeze).
- TB-7 / TB-16 existing Lean oracle harness as concrete §2.2 example.
