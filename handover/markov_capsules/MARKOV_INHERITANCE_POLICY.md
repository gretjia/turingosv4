# MARKOV_INHERITANCE_POLICY — TuringOS v4 Markov capsule chain-of-trust contract

**Status**: AUTHORITATIVE
**Date**: 2026-05-05
**Filed by**: TB-16.x.2 umbrella shipping closure (final sub-atom 2.6 commit `35a4e9b`).
**Architect mandate**: `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md` §A.5 (lossless ruling at §B.6 + §B.7).
**Required artifact** declared by ruling §A.5 verbatim: "New required artifact: `MARKOV_INHERITANCE_POLICY.md`."
**Hard-precondition** for TB-17: this document must exist + be tested before TB-17 ratification per `feedback_o1_chain_on_auditability` + memory `feedback_markov_inheritance_tape_derived` + this session's TB-17 PRE-17.5 + PRE-17.6 declarations.

---

## §1 — Purpose

Markov capsules (`MARKOV_TB-N_<DATE>.json` files in `handover/markov_capsules/`) compress the cross-run signal that the FC3 (meta) layer uses for cross-tracer-bullet learning: typical-error clusters, unresolved OBS rollups, next-session-context derivations. Each capsule has a `previous_capsule_cid: Option<Cid>` field (CAS reference to the prior capsule).

This document defines the **canonical contract** for how the `previous_capsule_cid` field is populated, validated, and consumed — so that:

- **Replay-determinism (Art. 0.2 Tape Canonical)** is preserved: a capsule's `previous_capsule_cid` MUST be reconstructible from the canonical L4 chain + CAS. No filesystem-only pointer (`LATEST_MARKOV_CAPSULE.txt` style) may serve as canonical input.
- **Genesis vs. inherited vs. unresolvable** are distinguishable at audit time per architect ruling PRE-17.4.
- **Append-Only DAG (Art. 0.1)** invariant holds: capsule chains form a DAG, never overwrite, and never branch silently.
- **No hidden parallel ledger** runs alongside the L4 chain (closes OBS_R022 Option α permanently; rejects Option γ provenance-sidecar per memory `feedback_markov_inheritance_tape_derived`).

---

## §2 — Three semantically-distinct cases (per architect PRE-17.4)

The `audit_tape` binary MUST distinguish:

### §2.1 Case A — Genesis (no prior Markov capsule)

- **Definition**: this run's capsule is the FIRST capsule for its chain lineage. There is no prior capsule, intentionally.
- **Capsule field**: `previous_capsule_cid: None`.
- **CLI invocation**: `audit_tape` invoked WITHOUT `--markov-pointer` argument. Per OBS_R022 Option α (RATIFIED 2026-05-04), `--markov-pointer` is optional; absence ≡ genesis chain.
- **Audit verdict**: `feature_coverage.TB-15_autopsy_markov` may be GREEN if a Markov capsule is generated this run; `previous_capsule_cid` is recorded as null in verdict.json.
- **Constitutional alignment**: Art. 0.2 (Tape Canonical) — genesis state IS the canonical starting point; no parallel-ledger inheritance is invoked.

### §2.2 Case B — Inherited (continuation from valid prior chain)

- **Definition**: this run's capsule cites a prior capsule via `previous_capsule_cid: Some(prior_cid)`. The prior capsule MUST be reachable in CAS AND its bytes MUST canonical-decode to a valid `MarkovEvidenceCapsule`.
- **Two sub-modes for sourcing the prior chain**:

  **B.α (transitional, current default)**: prior capsule resolved by reading the prior chain's `runtime_repo` + `cas` directories explicitly passed to `audit_tape` / `generate_markov_capsule` via CLI args (`--prior-chain-runtime-repo` + `--prior-chain-cas-dir` are forward-compatible flags; not yet wired but reserved). The smoke runner provides these paths from a deterministic source (e.g., the prior tracer-bullet's archived evidence dir).

  **B.β (long-term canonical, per Art. 0.4 path B)**: prior capsule resolved IN-TAPE — the chain's L4 ledger contains a `TerminalSummaryTx` with an `evidence_capsule_cid` field pointing to a CAS object whose `markov_capsule_cid` field is the `previous_capsule_cid` for the next chain. No CLI argument needed; the chain itself is the authoritative source.

- **Validation contract** (both sub-modes):
  1. `cas.get(previous_capsule_cid)` MUST resolve.
  2. The resolved bytes MUST canonical-decode to `MarkovEvidenceCapsule`.
  3. The decoded capsule's `tb_id` MUST be lexicographically PRIOR to the current capsule's `tb_id` (e.g., `MARKOV_TB-15_*` is a valid predecessor of `MARKOV_TB-16_*`).
  4. The decoded capsule's `constitution_hash_hex` MUST match the current run's constitution hash (per audit assertion id=32 `markov_constitution_hash_matches`).
- **Audit verdict**: id=32, id=33 (`markov_typical_errors_recompute`), id=34 (`markov_unresolved_obs_recompute`), id=35 (`markov_next_session_context_resolves`) all PASS.

### §2.3 Case C — Invalid / unresolvable Markov pointer

- **Definition**: a `previous_capsule_cid` is supplied (non-None) but FAILS one of the validation contracts in §2.2.
- **Concrete failure modes**:
  - `cas.get(previous_capsule_cid)` returns NotFound.
  - Bytes decode-fail (canonical_decode + serde_json fallback both fail).
  - `tb_id` ordering violation (cyclic or out-of-order — e.g., `MARKOV_TB-17_*` claims to inherit from `MARKOV_TB-19_*`).
  - `constitution_hash_hex` mismatch (the prior chain ran on a different constitution version).
- **Audit verdict**: id=32 HALT (one of: `MarkovCapsuleNotFound`, `MarkovCapsuleDecodeError`, `MarkovTbIdOrderingViolation`, `MarkovConstitutionHashMismatch`). The audit verdict is BLOCK; chain MUST NOT ship until the pointer is fixed.

---

## §3 — Forbidden patterns

The following patterns are EXPLICITLY FORBIDDEN per architect ruling §B.7 (FC1/FC2/FC3 alignment) + memory `feedback_markov_inheritance_tape_derived`:

### §3.1 Global filesystem pointer (Option γ DEAD)

- ❌ **`handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`** (or any "latest pointer" file) MUST NOT exist in `handover/markov_capsules/` as a canonical source.
- The OBS_R022 Option α closure (commit `f2bb871`, TB-16.x.fix) deleted this file and removed all canonical references. This document RECONFIRMS the deletion; any future commit re-introducing such a file is a **constitutional violation** per Art. 0.2 (Tape Canonical) — must be reverted immediately.

### §3.2 Provenance sidecar (Option γ also DEAD)

- ❌ A separate `provenance.json` / `chain_lineage.json` file alongside the chain that records "this chain inherits from X" via filesystem path is FORBIDDEN. The Markov inheritance MUST be either:
  - In-tape (B.β long-term), or
  - Explicit CLI arg pointing at a runtime_repo (B.α transitional).
- Provenance sidecars create the "hidden parallel ledger" pattern that OBS_R022 ruled against.

### §3.3 Last-writer-wins inheritance

- ❌ A run's Markov capsule MUST NOT inherit from "whichever capsule was generated most recently across all chains". Inheritance is chain-scoped: a chain's capsule inherits from its lineage predecessor, not from the global wallclock-latest.

### §3.4 Cross-constitution inheritance without explicit override

- ❌ A run with `constitution_hash_hex = X` MUST NOT silently inherit a prior capsule with `constitution_hash_hex = Y ≠ X`. The constitution hash mismatch HALTs the audit.
- An EXPLICIT override is allowed (and required) for `TURINGOS_MARKOV_OVERRIDE=1` deep-history ingest mode, where the operator deliberately bridges constitution versions. This mode is fenced by audit assertion id=35 + Atom 6 binary smoke per TB-16 main charter §1.2 H11.

---

## §4 — α/β deprecation calendar

| Phase | Mode | TB | Status |
|---|---|---|---|
| α (transitional) | CLI arg `--prior-chain-runtime-repo` + `--prior-chain-cas-dir` (currently: `--markov-pointer` is optional; absence ≡ genesis) | TB-16.x.fix → TB-17 | **CURRENT** |
| β (long-term canonical) | In-tape `previous_capsule_cid` derived from the chain's own `TerminalSummaryTx → EvidenceCapsule.markov_capsule_cid` | TB-17+ multi-task arena (`comprehensive_arena.rs` substantive build per PRE-17.6) | **TARGET** |
| γ (filesystem pointer / provenance sidecar) | Any global "latest" file or sidecar JSON | DEAD — explicitly rejected by OBS_R022 ruling §B.6 | **FORBIDDEN** |

α → β migration is the architectural intent of TB-17 PRE-17.6 (multi-task arena driving multiple tasks within ONE chain, with `TerminalSummaryTx → EvidenceCapsule → markov_capsule_cid` pipeline established). When β lands:
- α CLI args remain available for cross-chain inheritance (e.g., manual archeology), but β path is the production default.
- Audit assertion id=32 + id=35 stay strict on both paths.

---

## §5 — Fail-closed semantics on prior-chain-missing

Per architect ruling §B.6 PRE-17.4 + this document's §2.3 (Case C):

If a run claims a prior chain (`previous_capsule_cid != None`) but the prior `runtime_repo` is not accessible (CLI arg path doesn't exist for α; in-tape resolution returns missing CAS for β):

1. `audit_tape` MUST verdict=BLOCK with halted=1, citing `MarkovCapsuleNotFound` or `MarkovChainRuntimeRepoMissing`.
2. `generate_markov_capsule` MUST refuse to produce the new capsule (exit non-zero, refuse to write the JSON file).
3. The smoke runner MUST treat this as a hard error (set -e + exit non-zero per .2.4.fix r1 hardening pattern).
4. NO fallback to "treat as genesis" silently. NO retry with different sources. NO best-effort recovery.

This is the Art. 0.2 (Tape Canonical) absolute: if the chain claims continuation, continuation MUST be VERIFIED, not faked.

---

## §6 — Ship-gate test obligations (TB-17 SG-17.9 + SG-17.10)

Per architect ruling §A.5 verbatim:

> "New ship gates: SG-17.9 (Markov inheritance policy documented + tested) + SG-17.10 (no global filesystem pointer source-of-truth remains)."

### §6.1 SG-17.9 — Markov inheritance policy documented + tested

- **Documentation requirement**: this file (`handover/markov_capsules/MARKOV_INHERITANCE_POLICY.md`) exists, current, cross-references the architect ruling.
- **Test requirement**: TB-17 charter MUST add a test that exercises all three cases of §2 (Genesis / Inherited / Invalid) + the four forbidden patterns of §3 (each pattern triggers BLOCK).
  - Recommended location: `tests/markov_inheritance_policy.rs` (NEW, TB-17 scope).
  - Recommended fixtures: synthetic mini-chains with each case + each forbidden pattern.

### §6.2 SG-17.10 — No global filesystem pointer source-of-truth remains

- **Verification**: `find handover/markov_capsules/ -name "LATEST*" -type f` MUST return empty.
- **Forward-trigger**: if this file is reintroduced (e.g., by an experiment branch merge), TB-17 ratification fails until removed + this policy file's §3.1 is re-enforced.

---

## §7 — Cross-references

- **Architect ruling** (lossless): `handover/directives/2026-05-04_TB16_OBS_R022_ARCHITECT_RULING.md`
- **OBS_R022** (Option α closure): `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md`
- **TB-16.x.fix** (commit `f2bb871`): executed Option α — deleted `LATEST_MARKOV_CAPSULE.txt` + de-canonicalized references
- **TB-16.x.2.1** (commit `fab2977`): strict-alignment patch — `--markov-pointer` made optional; absence ≡ genesis
- **OBS_R024** (TB-17 PRE-17.5 forward trigger): `handover/alignment/OBS_R024_TB_16_X_2_4_BOLTZMANN_OBSERVE_VS_ENFORCE.md`
- **TB-16.x.2.6 README** (TB-17 PRE-17.6 forward trigger): `handover/evidence/tb_16_x_2_6_smoke_2026-05-05/README.md`
- **Constitution Art. 0.2** (Tape Canonical): `constitution.md` lines 52–95
- **Constitution Art. 0.4** (Q_t version-controlled, path B chain continuation): `constitution.md` lines 114–152
- **Memory** `feedback_markov_inheritance_tape_derived` (per OBS_R022 ruling 2026-05-04: "Markov inheritance must be tape-derived; no filesystem-side global pointers")
- **Memory** `feedback_o1_chain_on_auditability` (state facts → L4; high-dim evidence → CAS; never per-attempt L4 spam)
- **TB-17 charter** (when written, MUST cite this policy as PRE-17.1..17.4 + PRE-17.5 + PRE-17.6 + SG-17.9 + SG-17.10 hard precondition)
- **Future test target**: `tests/markov_inheritance_policy.rs` (TB-17 scope)

---

## §8 — Closure of TB-17 hard preconditions PRE-17.1 ... PRE-17.4

Per the architect ruling §B.6 verbatim text (transcribed):

| PRE | Architect text | Closure status (2026-05-05) | Evidence |
|---|---|---|---|
| PRE-17.1 | TB-16 global Markov pointer issue closed | ✅ CLOSED | TB-16.x.fix commit `f2bb871` deleted `LATEST_MARKOV_CAPSULE.txt`; SG-16.7..16.10 added |
| PRE-17.2 | All run-to-run inheritance is either (a) in-tape continuation, or (b) explicit prior-chain-runtime-repo input | ✅ CLOSED via documentation | This policy §2.2 documents both modes (B.α current, B.β long-term); test enforcement at TB-17 SG-17.9 |
| PRE-17.3 | No global latest pointer acts as source of truth | ✅ CLOSED | Same as PRE-17.1 + this policy §3.1 forbids reintroduction |
| PRE-17.4 | audit_tape can distinguish genesis / inherited / invalid Markov pointer | ✅ CLOSED for verdict semantics | This policy §2.1, §2.2, §2.3 enumerate the three cases; audit assertion id=32+33+34+35 already implement Genesis / Inherited / Invalid distinction (Inherited+Invalid path covered by id=32 HALT detail strings) |

This document IS the artifact required by architect ruling §A.5 + §A.8 ("Update TB-17 memory entry with PRE-17.1..PRE-17.4 + SG-17.9..17.10 + `MARKOV_INHERITANCE_POLICY.md`"). Filed as part of TB-16.x.2 umbrella shipping closure — concrete forward path for TB-17.
