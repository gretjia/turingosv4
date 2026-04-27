# OBS — Q_t Five-Root Extension (Whitepaper v2 § 4)

**Date**: 2026-04-27
**Type**: alignment observation
**Source**: v2 whitepaper § 4

---

## Observation

Constitution Art. 0.4 defines:

```
Q_t = ⟨q_t, HEAD_t, tape_t⟩
```

v2 § 4 extends this for ChainTape implementation:

```
Q_t = <
  q_t,
  HEAD_t,
  state_root_t,
  tape_view_t,
  ledger_root_t,
  budget_state_t,
  predicate_registry_root_t,
  tool_registry_root_t
>
```

The original three-component form is preserved — `q_t`, `HEAD_t`, `tape_t` are still there (with `tape_t` realized as `state_root_t` + `tape_view_t`). The extension adds **five additional roots** that make the registry / ledger / budget state cryptographically committable.

---

## Why this is not a constitutional violation

Art. 0.4 defines the **schema** of Q_t. The extension is an **implementation refinement** that:
- Preserves the three top-level components (q_t, HEAD_t, tape_t-derived).
- Adds derived roots that any tape implementation already has implicitly (every CAS-backed system has a state root; every predicate registry has a Merkle root).

The five new roots are not new state — they are **cryptographic commitments to existing state**. Adding them to Q_t makes the state explicitly verifiable; it does not change what the state IS.

---

## Current state in code

Wave 4-C / CO1.2 implemented Q_t struct with:
- `q_t` ✅
- `HEAD_t` ✅
- `tape_t` ✅
- five extension fields as `#[serde(default)]` stubs (forward-compat per Codex audit Q9) — present but typed as placeholders

This means **CO1.2 already left room for v2 § 4 extension** without requiring a struct rewrite.

### Forward-compat status

| Field | Stub status | When implemented |
|---|---|---|
| `state_root_t` | placeholder `Option<sha256>` | when CAS/store wires into runtime (CO1.4 done; integration pending) |
| `tape_view_t` | placeholder | Layer 5 materializer (later wave) |
| `ledger_root_t` | placeholder | Layer 4 transition_ledger (CO1.7 — Wave 6) |
| `budget_state_t` | already partly populated via MicroCoin | CO1.7.2 budget integration |
| `predicate_registry_root_t` | placeholder | CO1.5 already computes Merkle root; wiring into Q_t pending |
| `tool_registry_root_t` | placeholder | CO1.6 already computes Merkle root; wiring into Q_t pending |

---

## Implementation path (forward-looking)

If this OBS ratifies into action, a likely atom sequence:

1. **CO1.2 v2** — populate the five stub fields with real types (no-op semantics first; just typed slots)
2. **CO1.7 transition_ledger** (Wave 6) — `ledger_root_t` wired to actual append
3. **CO1.5/1.6 root wiring** — `predicate_registry_root_t` + `tool_registry_root_t` populated from existing Merkle computations
4. **CO1.4 → Q_t wiring** — `state_root_t` populated from CAS store
5. **Layer 5 materializer atom (later)** — `tape_view_t` populated

Each of these is small and atomic; can be done one at a time without breaking the Wave 4-C contract.

---

## Open questions

1. **Genesis Q_0 specification** — v2 § 11.1 lists genesis_block fields. Should `genesis_payload.toml` (current 8-field) be extended to seed all five roots at boot?
2. **Hash-chain semantics** — is Q_{t+1} ↔ Q_t linkage via `prev_state_root` chain, or via append-only ledger only?
3. **Snapshot vs delta** — does each Q_t carry full root, or only delta-from-parent? (Affects storage cost.)

---

## Status

**Captured. CO1.2 stubs already present.** Promote to atomic-action plan when:
- Wave 6 CO1.7 starts (would naturally fill `ledger_root_t`), OR
- CO1.5/1.6 registry roots need to be wired into a verification flow (e.g., genesis attestation, boot trust-root recursive check)

— ArchitectAI, 2026-04-27
