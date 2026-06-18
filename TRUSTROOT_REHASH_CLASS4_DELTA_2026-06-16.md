# TRUSTROOT_REHASH_CLASS4_DELTA — 2026-06-16

Class-4 manifest delta for the H-HET-2 convergence Phase-4 trust-root rehash.
Protocol (§6 Step 4): recompute pins from FINAL MERGED BYTES only; no side-pick; constitution.md untouched; Veto-AI PASS gate.

Pins recomputed: 20 of 135. `constitution.md` pin MATCHES current bytes (unchanged) — NOT in this delta.
All 20 files below are the de-Lean §8-ratified generic renames + the standing-rule-authorized H-HET-2
modules (routing/budget telemetry, proposal_telemetry v2 model_id). Each new hash = sha256(current file bytes).

| file | old pin | new pin (recomputed) |
|---|---|---|
| `src/bus.rs` | `e1e8e3777d2431fa…` | `dfbdadc93498c96c…` |
| `src/runtime/mod.rs` | `838dd8c81460b3a5…` | `86e6b2ce862a6ffb…` |
| `src/runtime/librarian_broadcast.rs` | `156633a752635b2e…` | `204dd6695405f2f0…` |
| `tests/tb_6_verify_chaintape.rs` | `5dc5f5f5d4c0dcee…` | `149cfbb08ff3d684…` |
| `src/runtime/audit_assertions.rs` | `b7837b3bcd88c372…` | `ca40593e2e280fd9…` |
| `src/runtime/evidence_capsule.rs` | `ce8089d54825b79c…` | `39e736d3b4019d74…` |
| `src/bin/audit_dashboard.rs` | `4a82d9164bb70818…` | `07a81bceef5886ae…` |
| `tests/tb_7_atom6_chain_backed_smoke.rs` | `ea296352f2a6e5ac…` | `8c5fbb10142dc974…` |
| `src/runtime/proposal_telemetry.rs` | `04dbc9b3610cfaed…` | `9140369bf52bd752…` |
| `src/runtime/chain_derived_run_facts.rs` | `e88112e75c979e2e…` | `6407958b415dfe54…` |
| `src/runtime/verification_result.rs` | `6b58d0ca1d09104f…` | `5401e9d59ab97572…` |
| `src/top_white/predicates/registry.rs` | `f27a169693b22fdf…` | `11d600efe7aad892…` |
| `src/bottom_white/cas/schema.rs` | `b7070a7f9a31adca…` | `eb50cc211e87ee11…` |
| `src/bottom_white/cas/store.rs` | `942792a2ae4cb3c7…` | `4733576e65ea8b98…` |
| `src/bottom_white/tools/registry.rs` | `d91c03a392120100…` | `5f59dc9ef2665e25…` |
| `src/state/price_index.rs` | `474c1e2309625804…` | `e67120c448147583…` |
| `src/state/q_state.rs` | `1c54705fb00af987…` | `ba41cc0212faf6bc…` |
| `src/state/sequencer.rs` | `31d1eff93ff3dbed…` | `e021143625bfc9b6…` |
| `src/state/typed_tx.rs` | `55f80dc7560b6be4…` | `b1bbc419fc3686d1…` |
| `src/bottom_white/ledger/rejection_evidence.rs` | `a1767cc32c767e26…` | `121e2444949c541c…` |

## Constitutional self-checks (for Veto-AI)
- Art 0.2: pins certify the CURRENT merged source; recompute-from-bytes (not a chosen side's stale hash).
- constitution.md: untouched (pin already matches; excluded from delta).
- No semantic change: only sha256 values updated to match existing committed source.
- §6 restricted surfaces among the 20 (bus/sequencer/typed_tx/cas/schema): content already committed via
  the §8-ratified de-Lean migration (3fb8cb68) + H-HET-2 standing-rule modules; this delta only re-pins them.
- llm_http.rs (GA-9): NOT in delta (pin matches; untouched).
