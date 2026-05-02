# TB-9 Charter Ratification — Atom 0.5

**Date**: 2026-05-02 (same day as TB-8 ship `43aa288`).
**Purpose**: Resolve the 5 open ratification questions in `handover/tracer_bullets/TB-9_charter_2026-05-02.md` §7 before Atom 1 begins.
**Authority**: architect directive 2026-05-02 Part C canonical (`handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md` line 1574-1592) — the verbatim TB-9 minimum requirements:

```text
TB-9：Durable AgentRegistry + Wallet Projection

目标：
  持仓、payout、future NodeMarket 都必须归属于 durable identity。

必须：
  agent durable key registry
  wallet read-only projection
  EconomicState canonical
  no f64 mutation
  cross-run identity
```

Plus directive top-level table line 156 (`...lossless_constitution_polymarket_directive.md`):

```text
TB-9   Durable AgentRegistry + Wallet projection
        — persistent agent pubkey registry
        — WalletTool becomes read-only projection
        — EconomicState canonical; no f64 mutation
        — agent identity survives run restart
```

Plus class declaration at line 436: `TB-9 = Class 3 (durable identity affects payout authority)`.
Plus TRACE_FLOWCHART_MATRIX TB-9 row: `✅ identity in input/output of Agent δ` + `✅ persistent registry initialized at boot`.

---

## §0 Scope trim

**Original draft charter §3 proposed 8 atoms including a new `q.agent_registry_t` top-level QState field, a new `TypedTx::AgentRegister` variant, and a system-emit AgentRegister-on-first-WorkTx hook.**

**Architect minimum spec does NOT require these.** The 5 mandates collapse to:

1. Durable on-disk **keystore** — load-or-generate at boot, secrets persist across runs
2. WalletTool surface unchanged but **owned f64 state deleted** — projection from `EconomicState.balances_t`
3. `EconomicState` retains its canonical role — no schema changes
4. Bus market f64 mutation path **deleted** — no shadow ledger
5. Same `AgentId → AgentPublicKey` binding survives evaluator restart

**Per system-prompt directive "don't add features beyond what the task requires"**, the over-scope ratchet (q.agent_registry_t + AgentRegisterTx + dispatch arm + STEP_B preflights) is removed. **Scope is trimmed to 6 atoms**:

```text
Atom 0.5  This document — ratification (Class 0)
Atom 1    src/runtime/agent_keypairs.rs durable-keystore extension (Class 3) — STEP_B preflight required
Atom 2    Evaluator boot wire to durable keystore (Class 2)
Atom 3    WalletTool collapse to read-only projection (Class 2)
Atom 4    Bus market f64 path deletion (Class 2)
Atom 5    Cross-run smoke evidence (Class 1)
Atom 6    Audit dashboard durable-identity section (Class 0/1)
Atom 7    Self-audit + dual external audits (Class 3)
Atom 8    LATEST + TB_LOG + ship commit (Class 0)
```

(Atom IDs 5-8 retained from original charter; the deleted on-chain registry atoms collapsed into Atom 1.)

The forbidden list in charter §8 remains binding; items #11 (Q-derived agent_id), #17 (AgentRegister at challenge-resolve time) are dropped because there is no AgentRegister tx — but their spirit (no agent-submitted system tx; system retains sole crypto authority) is preserved by keeping the keystore strictly host-side, never on-chain.

---

## §1 Q1 — Keystore file shape — RATIFIED: single encrypted file

**Decision**: single encrypted file at `~/.turingos/keystore/agent_keystore.enc` containing serialized `BTreeMap<AgentId, AgentSecretBytes>`.

**Spec mapping**: 必须 #1 "agent durable key registry" — singular registry, not per-agent files.

**Implementation**: mirror `src/bottom_white/ledger/system_keypair.rs:417-463` pattern exactly:
- `default_agent_keystore_path()` resolves `~/.turingos/keystore/agent_keystore.enc`
- `TURINGOS_AGENT_KEYSTORE_PATH` env override
- KDF (Argon2id m=64MiB, t=3, p=4) + ChaCha20-Poly1305 AEAD encryption-at-rest
- KDF parameters from env: `TURINGOS_KDF_MEMORY_KIB / TURINGOS_KDF_ITER / TURINGOS_KDF_LANES` (shared with system keystore)
- Atomic write 0600 (`write_keystore_0600` precedent)
- Format magic `TOS4AGTKEY1` (distinguishes from system keystore's `TOS4SYSKEY1`)

**Password source**: env var `TURINGOS_AGENT_KEYSTORE_PASSWORD`. If absent at evaluator boot AND `TURINGOS_AGENT_KEYSTORE_PATH` is unset (i.e. caller wants the default durable path), evaluator MUST refuse to start with an explicit error. Tests use `tempfile::TempDir` + `TURINGOS_AGENT_KEYSTORE_PATH` override + a fixed test password — no production-path fallback.

**Backwards-compat with TB-7 run-local manifest**: per-run `<runtime_repo>/agent_pubkeys.json` continues to be written exactly as TB-7 ships it (per `feedback_no_retroactive_evidence_rewrite`); this is now a **defense-in-depth replay sidecar**, not the source of truth. The durable keystore is the source of truth for cross-run identity.

---

## §2 Q2 — Registry placement in QState — RATIFIED: NOT in QState

**Decision**: registry lives **on the host filesystem** (`~/.turingos/keystore/agent_keystore.enc`) and **inside the `AgentKeypairRegistry` runtime struct**. Zero new fields in `QState` or `EconomicState`.

**Spec mapping**: 必须 #3 "EconomicState canonical" — the spec asserts EconomicState's existing role is canonical for **economic** state. Agent identity is host-side metadata, NOT economic state. Adding a `q.agent_registry_t` field would be over-scope.

**Consequence for replay**: `verify_chaintape` continues to read `<runtime_repo>/agent_pubkeys.json` (the per-run manifest), as it does in TB-7. Cross-run signature verifiability is provided by the durable keystore producing the **same pubkey** for the **same AgentId** across runs — so any per-run manifest will list the same `agent_id → pubkey` row, replay-determined.

**Future**: if a future TB needs on-chain registry (e.g. for Public Chain anchoring or NodeMarket position attribution that survives even keystore loss), that becomes a separate TB. Not blocking this one.

---

## §3 Q3 — Emission trigger — RATIFIED: N/A (no on-chain registration)

**Decision**: there is **no on-chain AgentRegister event** in TB-9. Identity registration is implicit in the WorkTx signature.

**Spec mapping**: 必须 #5 "cross-run identity" + 必须 #4 "no f64 mutation" + class 3 "durable identity affects payout authority". The architect's "durable identity" goal is satisfied as long as the **same AgentId continues to map to the same pubkey across runs**. WorkTx already carries `(agent_id, agent_signature, agent_pubkey)`; replay verifiability is end-to-end.

---

## §4 Q4 — WalletTool collapse depth — RATIFIED: read-only projection wrapper

**Decision**: WalletTool retains its `TuringTool` trait surface but **deletes all owned f64 state**. Mutators (`deduct`, `credit`, `record_shares`, `ensure_agents`, `save_to_disk`, `load_from_disk`) are removed. New API:

```rust
impl WalletTool {
    pub fn balance(&self, agent: &AgentId, econ: &EconomicState) -> MicroCoin {
        econ.balances_t.0.get(agent).copied().unwrap_or_else(MicroCoin::zero)
    }
}
```

`on_init` becomes a no-op (genesis is `q_state::genesis()`). `on_pre_append` returns `Pass` unconditionally (typed_tx admission gates own all veto logic).

**Spec mapping**: 必须 #2 "wallet read-only projection" + 必须 #4 "no f64 mutation" — exact match.

**Open detail (resolved here)**: `query_state(key)` keeps its TuringTool-trait signature (`fn query_state(&self, key: &str) -> Option<String>`), but for the `balance_<agent>` key family it returns `None` (since no `EconomicState` reference is plumbed through TuringTool by design). Callers that need a balance read MUST go through `WalletTool::balance(&AgentId, &EconomicState)` directly. Test `test_query_balance` is updated to assert `None` for `balance_*` keys.

This avoids a TuringTool trait signature change (which would ripple through every tool implementor). Trait change is post-v1.0 polish.

---

## §5 Q5 — Legacy bus market path — RATIFIED: DELETE

**Decision**: delete the v3 simulation-era market shares grant code and `debit_wallet/credit_wallet/settle_resolved_markets` helpers in `src/bus.rs`.

**Spec mapping**: 必须 #4 "no f64 mutation" — exact match. The bus market path is the ONLY remaining f64 mutator outside `WalletTool` itself; deletion is necessary for the spec.

**Scope**: lines ~287-490 of `bus.rs` (market shares investment loop + `debit_wallet` + `credit_wallet` + `settle_resolved_markets`). All tests exercising these paths are deleted in the same atom.

**Future market layer (TB-12 / TB-13)**: when `MarketSeedTx` + `CompleteSetMintTx` + `CPMMRouter` ship, they will use typed_tx dispatch arms mutating `EconomicState.share_balances_t / liquidity_t`, NOT `WalletTool` and NOT `bus.rs` f64 helpers. This deletion is permanent.

---

## §6 Ratification verdict

All 5 questions resolved per architect Part C minimum spec. Self-ratified by Claude Code per user authority "你根据要求自主决策" (you decide autonomously per requirements; 2026-05-02). User explicitly cited the Part C TB-9 spec and waits for the closing report.

**Atom 1 may proceed.**

**Architect override channel**: if any of Q1-Q5 require a different decision, raise during the recursive self-audit at Atom 7 or via `/architect-ingest` after ship — re-charter under TB-9.1.

---

## §7 Updated atom plan (8 atoms; replaces charter §3)

| Atom | Class | Action |
|---|---|---|
| 0.5 | 0 | THIS document |
| 1 | 3 | Extend `src/runtime/agent_keypairs.rs` with durable-keystore primitives (encrypt/decrypt/load/save mirroring `system_keypair`); add `generate_or_load_agent_keystore` factory; persist on every new keypair generation. STEP_B preflight required (auth-primitive). |
| 2 | 2 | Wire `experiments/minif2f_v4/src/bin/evaluator.rs:765` to call `generate_or_load_agent_keystore` instead of `AgentKeypairRegistry::open`. Read password from env. |
| 3 | 2 | Refactor `src/sdk/tools/wallet.rs` per §4 above. |
| 4 | 2 | Delete bus market path per §5 above. |
| 5 | 1 | Cross-run smoke evidence: run-A signs WorkTx → exit → run-B loads same keystore → signs WorkTx-2 with same pubkey. Evidence dir `handover/evidence/tb_9_durable_identity_smoke_2026-05-02/`. |
| 6 | 0/1 | Audit dashboard §10 durable-identity section: shows agent_id + pubkey + (TB-9-new) `keystore_origin = durable / run-local`. |
| 7 | 3 | Recursive self-audit + Codex impl-paranoid + Gemini architectural (degraded fallback OK per `feedback_dual_audit`). |
| 8 | 0 | LATEST.md TB-9 section + TB_LOG.tsv row 29 + TRACE_FLOWCHART_MATRIX TB-9 row planned→shipped + ship commit. |

---

## §8 Iteration cap re-stated

72h with 24h checkpoints. Atoms 1+2 by T+24h, Atoms 3+4 by T+48h, Atoms 5-8 by T+72h. Mandatory escalation if Atom 1 or Atom 2 slips past 72h (per `feedback_iteration_cap_24h` production-wire-up exception).

---

## §9 Forbidden in TB-9 (re-stated, narrowed per scope trim)

```text
1.  Multi-org / cross-host keystore federation                       (TB-16+)
2.  Public-chain anchoring of keystore                                (post-v1.0)
3.  KDF password rotation / re-encryption                             (TB-16+ polish)
4.  Production-grade password prompt / zeroize on stack               (post-v1.0; env-var MVP)
5.  Reputation-as-pubkey-attribute / reputation slashing              (RSP-3.2 / post-TB-15)
6.  NodeMarket position binding to AgentId                            (TB-11)
7.  CompleteSet / MarketSeedTx / CPMM / PriceIndex / AMM             (TB-12 / TB-13 / TB-14)
8.  Lamarckian Autopsy / EvidenceCapsule / Markov Log Loom           (TB-15)
9.  Boltzmann masking / read-view filtering                           (TB-14)
10. New typed_tx variant for AgentRegister                            (NOT minimum-required; out)
11. New q.agent_registry_t top-level QState field                     (NOT minimum-required; out)
12. New SystemEmitCommand variant                                     (NOT minimum-required; out)
13. Agent-submitted system tx                                         (RSP-3.0 inheritance — system-only)
14. WalletTool re-introduction of any owned f64 state                 (collapse is permanent post-TB-9)
15. Bus market path resurrection                                       (deletion is permanent)
16. f64 in any new monetary code path                                 (MicroCoin only)
17. Migration step for pre-TB-9 ledger entries                        (per `feedback_no_retroactive_evidence_rewrite`)
18. Keystore stored under `runtime_repo/`                              (runtime_repo is per-run; durable keystore is host-level)
19. Keystore committed to git                                          (gitignore enforced)
20. Constitution.md edit                                                (sudo-only per directive ruling 15)
```

---

## §10 Sign-off

```text
ratified_by      : claude-opus-4-7[1m] (autonomous per user authority 2026-05-02)
ratified_at      : 2026-05-02 (same day as TB-8 ship 43aa288)
spec_authority   : handover/directives/2026-05-02_lossless_constitution_polymarket_directive__part_C_updated_final_ruling.md:1574
charter_revision : 1 (scope trimmed from 8 atoms to 8 atoms with 2 atoms collapsed; net atom count unchanged but on-chain registry surface removed)
next_step        : Atom 1 STEP_B preflight + dispatch
```
