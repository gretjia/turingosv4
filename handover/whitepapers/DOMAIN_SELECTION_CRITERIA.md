# DOMAIN_SELECTION_CRITERIA — TuringOS v4 P7 candidate-domain classification

**Status**: TB-17 atom 2 — autonomous Class 0 draft.
**Filed**: 2026-05-05.
**Authority**: TB-17 charter §3 atom 2 + 2026-05-05 architect verdict §B.5 (FR-17.1, FR-17.2, FR-17.7, FR-17.14) + §B.7 (SG-17.2 ≥3 domains classified, SG-17.3 ≥1 pilot approved).
**Filing convention**: classification frozen at TB-17 SHIP; later TB charters MAY add new candidates but MUST NOT silently re-tier an existing entry.

---

## §1 Domain risk-tier definitions (FR-17.2 verbatim)

Per architect §B.5 verbatim taxonomy. Each tier is defined by the **verification asymmetry** between solve-cost and verify-cost — the founding principle of TuringOS's monitorable black-box approach (Lean Proof = "hard generate, easy verify").

| Tier | Solve cost | Verify cost | Goodhart risk | Real-world admissibility |
|---|---|---|---|---|
| **T1** | easy | easy | low | trivial pilot acceptable; usually not interesting enough to warrant TB-cycle |
| **T2** | hard | easy | low | **canonical TuringOS sweet spot**; Lean/Coq/Isabelle formal verification lives here |
| **T3** | hard | hard | medium | requires multi-oracle quorum + extended challenge window; pilot only with explicit oracle |
| **T4** | deceptive (looks easy / looks correct) | hard | **high** | **FORBIDDEN as initial pilot**; requires architect ratification + adversarial-oracle design + extended human escalation |

### §1.1 Verification asymmetry — why this tiering

The unforgeable advantage of T2 (and the reason TuringOS chose Lean as its first real-world-adjacent surface) is that the **verifier can reject in O(seconds)** what the **solver may have spent hours generating**. This asymmetry is what makes Boltzmann observation, ChainTape audit, and challenge-court-resolution **mechanically meaningful** — there is a deterministic predicate that bounds disagreement.

T3 weakens this: verification still beats solving in expected cost, but is no longer trivial; oracle disagreement becomes possible; the challenge window must scale.

T4 **inverts** the asymmetry: superficially-correct outputs may be wrong, and verification may itself be a hard problem. This is where Goodhart wins by default. Real-world domains with T4 properties (e.g., "does this medical recommendation produce good outcomes?") MUST NOT be pilot candidates without explicit architect-authored oracle architecture.

### §1.2 Tier-to-oracle binding (forward-link to atom 3)

```
T1 → simple deterministic predicate; single oracle source.
T2 → strong deterministic predicate (e.g. proof-checker exit code); single trusted oracle adequate.
T3 → multi-oracle quorum (median or majority); explicit disagreement protocol per FR-17.5.
T4 → multi-oracle + adversarial-oracle + human escalation default-on; PROHIBITED as pilot.
```

### §1.3 Tier-to-irreversibility binding (forward-link to atom 6)

```
T1 → reversible-only actions allowed; no public publication.
T2 → reversible-or-compensable actions allowed; delayed-settlement window per FR-17.7.
T3 → reversible-or-compensable + human escalation required for any action class >= "publication"; longer settlement.
T4 → no irreversible action permitted under any condition; pilot prohibited regardless.
```

---

## §2 Allowed real-world domain categories — candidate enumeration (FR-17.1; SG-17.2 ≥3)

Per architect §B.8.3 hint list + this charter's §B.4 candidate selection criteria. Each candidate carries: T-tier / oracle-design-pointer / irreversibility-profile / Goodhart-risk-call.

### §2.1 Candidate D1 — Lean / Coq / Isabelle formal verification (T2; **PILOT-APPROVED — see §6**)

- **Solve task**: produce a proof object for a stated theorem in a chosen formal system.
- **Verify task**: run the formal-system's proof checker; exit code is the predicate.
- **Verification asymmetry**: extreme. Proof generation may take hours of LLM compute; verification is sub-second to seconds.
- **T-tier**: **T2** (hard solve, easy verify, low Goodhart).
- **Oracle**: deterministic — proof checker binary (`lean`, `coqc`, `isabelle`) under a pinned version (constitution C-012 oracle freeze).
- **Irreversibility profile**: pure compute artifact; the proof itself is an inert text file. Publication requires explicit human gate per atom 5.
- **Goodhart risk**: low. The proof checker is total; one cannot construct a "fake-looking" valid proof without it being a valid proof.
- **Mapping to current TuringOS capability**: TB-7 chain-backed Lean evaluator already produces ChainTape-witnessed proof attempts on MiniF2F-derived problems; TB-16 sandbox arena demonstrated the full attempt → challenge → resolve → finalize-or-bankrupt loop.
- **Why this is the pilot**: lowest-risk path that exercises real TuringOS economic + autopsy + market machinery without crossing the sandbox-vs-production label boundary in a non-trivial way. A successful TB-19 pilot here is the best evidence for further P7 expansion.

### §2.2 Candidate D2 — Document-citation verification (T2 borderline T3)

- **Solve task**: given a statement S and a corpus C, identify the citation in C (or assert none exists).
- **Verify task**: re-fetch the cited document at a fixed URL/Cid; check that the asserted-citation-passage matches the document's actual content.
- **Verification asymmetry**: moderate. Solve = retrieve and pattern-match across a large corpus. Verify = single fetch + text comparison.
- **T-tier**: **T2 if the corpus is fixed and content-addressed (CAS); T3 if the corpus is the live web** (because the document may have changed between solve and verify).
- **Oracle**: hash-pinned corpus snapshot OR multi-fetch quorum if live web is required.
- **Irreversibility profile**: pure read; emitting a citation is a publication-class action requiring human gate per atom 5 (if external).
- **Goodhart risk**: moderate. A solver could fabricate citations that look plausible; verification breaks fabrication trivially as long as the corpus is canonical.
- **Why this is a candidate (not pilot)**: useful complement to D1 because it tests the "corpus snapshot" oracle pattern; adoption deferred until atom 3 oracle-provenance section is firm.

### §2.3 Candidate D3 — Open-source issue reproduction (T3)

- **Solve task**: given a GitHub issue (or equivalent), produce a minimal reproducer that triggers the reported bug in a pinned environment.
- **Verify task**: run the reproducer in a clean sandbox; check that the failure mode matches.
- **Verification asymmetry**: variable. Often hard-to-verify because the bug may be flaky or environment-dependent.
- **T-tier**: **T3** (hard solve, hard verify, moderate Goodhart).
- **Oracle**: deterministic-environment harness (Docker / nix-shell / pinned dependencies); multi-run consistency check; flaky-bug threshold (e.g., "≥3 of 5 runs reproduce" passes).
- **Irreversibility profile**: requires sandboxed execution; reproducer code itself is reversible artifact, but execution may have side effects (network calls, file writes) that need sandbox boundary.
- **Goodhart risk**: moderate. Solver may craft a "reproducer" that triggers a different failure mode that looks similar; verification depends on the harness specificity.
- **Why this is a candidate (not pilot)**: T3 requires multi-oracle-quorum + extended challenge window per atom 4 — both architectural surfaces that are not yet shipped. Defer to TB-19+ once D1 pilot ratifies the broader pattern.

### §2.4 Candidate D4 — Web-benchmark deterministic extraction (T1-T2)

- **Solve task**: given a structured page (e.g., a fixed-format dataset), extract a specific field.
- **Verify task**: re-extract; compare; flag mismatch.
- **Verification asymmetry**: high if the page format is stable; low if format drifts.
- **T-tier**: **T1 if the page is hash-pinned + deterministic-format; T2 if format is stable but page is live-fetched**.
- **Oracle**: hash-pinned snapshot (preferred) or multi-fetch quorum.
- **Irreversibility profile**: pure read; output is data-class artifact; no external write needed.
- **Goodhart risk**: low if pages are pinned; high if pages are live and adversarially editable.
- **Why this is a candidate (not pilot)**: too trivial as a meaningful TuringOS exercise; the system's economic + audit machinery is overkill. May be useful as an M0 / M1 harness-prep stress test, but not as a P7 pilot.

### §2.5 Excluded categories (architect §B.8.3 verbatim BAN list)

These categories are **PROHIBITED** as initial pilots regardless of any architectural readiness claim:

| Excluded domain | Why excluded |
|---|---|
| Medical | T4 — outcomes are hard to verify; irreversible-harm risk is high; regulatory burden |
| Legal | T4 — verification depends on expert interpretation; output is publication-class; jurisdictional asymmetry |
| Financial trading | T4 — adversarial environment; oracle manipulation is the dominant attack surface; irreversible payment subtype |
| Physical robotics | T4 — physical-actuation irreversibility subtype is by-default banned (atom 6 §2 subtype #5) |
| Security exploit deployment | T4 — adversarial; weaponization risk; irreversible damage potential |
| Autonomous API actuation | T4 — external-API-write irreversibility subtype (atom 6 §2 subtype #1); requires explicit human gate |

These exclusions are **bound forward** by CR-17.3 + atom 6 IRREVERSIBLE_ACTION_POLICY.md §3 (forbidden actions). A future architect ratification could change this list, but no AI-coder action may circumvent the exclusion.

---

## §3 Tier × oracle requirement matrix (FR-17.3; forward-link to atom 3)

(Detailed oracle architecture per tier is in `ORACLE_REQUIREMENTS.md` atom 3 §2; this matrix is the cross-reference.)

| Tier | Oracle architecture | Min sources | Disagreement protocol |
|---|---|---|---|
| T1 | single deterministic predicate | 1 | strict — any disagreement = bug, halt |
| T2 | single trusted deterministic oracle (proof checker / hash-pinned-corpus / pinned-environment) | 1 | strict — checker output is canonical |
| T3 | multi-oracle quorum | ≥3 | median or 2-of-3 majority; mismatch → human escalation per FR-17.5 |
| T4 | multi-oracle + adversarial-oracle + default human review | ≥3 + adversarial | human authority always; oracle agreement insufficient |

---

## §4 Tier × irreversibility matrix (FR-17.9; forward-link to atom 6)

| Tier | Reversible | Compensable | Publication | Irreversible-external |
|---|---|---|---|---|
| T1 | allow | allow | require-delay | deny |
| T2 | allow | allow | require-delay | deny |
| T3 | allow | require-human | require-human | deny |
| T4 | require-human | require-human | deny | deny (always) |

Verdicts use atom 6 §5 four-class verdict: `allow / deny / require-human / require-delay`. The `irreversible-external` column maps to atom 6 §2 subtypes #1, #2, #5, #6 — all `deny` regardless of tier.

---

## §5 Goodhart shield + adversarial-tier protections

Per Art. III.4 (Goodhart shield) + CR-17.13 (MiniF2F not real-world). Each tier carries an explicit Goodhart-defense strategy:

```
T1: predicate completeness (oracle covers all possible outputs).
T2: predicate determinism + version pinning (oracle behavior is byte-identical across runs).
T3: multi-source verification + cross-evidence challenge window per FR-17.6.
T4: pilot prohibited; if ever opened, requires architect-authored adversarial oracle.
```

**Goodhart attack surfaces explicitly cataloged**:
- Solver fabricates valid-looking output that doesn't actually solve task → predicate-determinism breaks fabrication.
- Solver targets oracle-implementation bug rather than task → bound by C-012 oracle freeze + atom 3 oracle attack surface section.
- Solver exploits verification-cost asymmetry by submitting adversarial inputs that crash the verifier → handled by atom 4 challenge-court resolution-authority section.

---

## §6 PILOT DOMAIN APPROVAL — D1 Lean/Coq/Isabelle formal verification (SG-17.3)

### §6.1 Pilot selection rationale

D1 is the only candidate that meets ALL of the following:
1. **T2 verification asymmetry** (low Goodhart risk).
2. **Existing TuringOS substrate**: TB-7 + TB-16 already produce chain-backed Lean evaluation; sandbox arena exercises the full economic + autopsy + market loop.
3. **Deterministic oracle**: proof-checker exit code (no quorum needed).
4. **Pure-compute artifact**: no irreversible external action subtype triggered.
5. **No regulated-domain overlap**: not medical / legal / financial / robotics.
6. **Reproducibility**: proof objects are inert text files, replayable byte-identical given pinned Lean + Mathlib.

### §6.2 Pilot specification (initial)

| Aspect | Spec |
|---|---|
| Problem set | MiniF2F heldout subset (already used in TuringOS sandbox); curated 50–100 problems for TB-19 pilot |
| Solve oracle | LLM swarm via TB-15-shipped substrate (`evaluator.rs` + agent message board) |
| Verify oracle | `lean --run` with pinned Lean version + Mathlib via `lake exe cache get` (per `feedback_lake_packages_vendored`); exit-code 0 = predicate true; non-zero = predicate false |
| Challenge window | TB-13 ChallengeTx + TB-14 ChallengeResolveTx (already shipped); window length per atom 4 |
| Settlement | NO REAL FUNDS in pilot; sandbox-prefixed agents only (per CR-16.5 carry-forward); final TB-19 charter may relax to delayed-settlement small-bounty per architect §B.10.3 |
| Failure handling | TB-15-shipped AgentAutopsyCapsule on bankruptcy; TB-11-shipped EvidenceCapsule on exhaust |
| Markov inheritance | per `MARKOV_INHERITANCE_POLICY.md` §2 (B.α transitional or B.β long-term per TB-17 atom 9 outcome) |
| Sandbox / production label | **SANDBOX** for TB-19 pilot; promotion to **SHADOW** then **LIVE** per atom 5 SAFETY_BOUNDARY label discipline; promotion requires architect ratification |

### §6.3 Pilot reject criteria (FR-17.14)

A D1 TB-19 pilot is **REJECTED** (must re-charter, not silently proceed) if any of the following holds:

| Criterion | Reject signal |
|---|---|
| MiniF2F batch fake-accept rate > 0 | atom 11 conformance battery + audit_tape verdict ≠ PROCEED |
| Replay non-determinism | cargo test --workspace introduces non-deterministic test; or audit_tape replay byte-mismatch |
| Markov inheritance ambiguity | any chain produces invalid `previous_capsule_cid` per `MARKOV_INHERITANCE_POLICY.md` §2.3 |
| Boltzmann observation ≠ enforcement disagreement detected | atom 7 design surfaces a Class 4 bug that observation cannot detect (forward-trigger TB-18 atom) |
| OBS_R023 hardcoded MaxTxExhausted regression | EvidenceCapsule emit on non-exhaust-path uses wrong RunOutcome (deferral cap = TB-18 per architect Q4) |
| Single-chain 13-of-13 unrealizable post-atom-8 | architectural-exclusion deviation rejected by architect |
| Audit dual-external VETO not closed | Codex / Gemini Class 3 audit produces VETO with no in-scope closure path |

Reject = file new TB charter; do NOT degrade pilot scope silently.

---

## §7 Future TB sequencing (forward triggers)

```
TB-17 SHIP (this charter)
  ⇩
TB-18 = Formal Benchmark Scale-Up
  full controlled MiniF2F (M2/M3/M4 per `feedback_minif2f_scaling_policy`)
  chain-backed; sandbox; no real money; no real-world claim
  closes OBS_R023 hardcoded MaxTxExhausted (architect Q4 deferral cap)
  ⇩
TB-19 = Low-Risk Real-World Pilot Design
  D1 Lean/Coq/Isabelle pilot per §6 spec
  sandbox-labeled; possibly delayed-settlement small-bounty
  ⇩
TB-20 = Pilot Sandbox  (per architect §B.8.4 — design future)
TB-21 = Limited Real-World Beta  (per architect §B.8.5 — design future)
```

Other candidates D2/D3/D4 deferred until D1 pilot ratifies the pattern.

---

## §8 Cross-references

- TB-17 charter §3 atom 2 + §1.4 FR-17 list.
- 2026-05-05 architect verdict §B.5 (FR-17.1, FR-17.2, FR-17.7, FR-17.14) + §B.6 (CR-17.1, CR-17.13, CR-17.14) + §B.8.3 candidate hint list + §B.8.5 ban list.
- ORACLE_REQUIREMENTS.md (atom 3) — per-tier oracle architecture detail.
- IRREVERSIBLE_ACTION_POLICY.md (atom 6) — irreversibility subtype catalog (8 subtypes per architect Q6.2).
- SAFETY_BOUNDARY.md (atom 5) — sandbox/SHADOW/LIVE label discipline.
- `feedback_minif2f_scaling_policy` — M0-M4 ladder; full benchmark = TB-18 only.
- `project_tb_16_ratified_with_scope_limits` — TB-16 ratified only as sandbox-controlled-market-smoke.
- Constitution C-012 (oracle freeze).
