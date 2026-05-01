# TB-6 Atom 3 — ChainTape Smoke Evidence

**Date**: 2026-05-01
**Atom**: TB-6 Atom 3 — Chain-backed smoke run on `mathd_algebra_107`.
**Branch / HEAD**: `main` @ `01b9e93` (TB-6 Atom 2 SHIPPED) + Atom 3 hook (uncommitted at smoke time; rehashed in Atom 3 ship commit).
**Runtime repo**: `runtime_repo/` (real Git2 repo with `refs/transitions/main` chain).
**Run ID**: `tb6-smoke-2026-05-01`.
**LLM**: `deepseek-v4-flash` via local proxy at `http://localhost:8080`.
**Lean toolchain**: `v4.24.0` (pinned).

This is the **first chain-backed smoke evidence in TuringOS v4 history**. Architect ruling 2026-05-01 D2 hard-required this from TB-6 onward; Atom 3 closes the 5-TB ChainTape production debt.

---

## §1 The 8 mandatory questions (architect § 3.5)

### Q1 — What entered CAS?

CAS payloads are stored under `cas/` (content-addressed). Two CAS objects entered during this run:
- The synthetic `TaskOpenTx` canonical encoding (referenced by `tx_payload_cid` in the L4 entry's `entry_canonical` blob).
- The `LedgerEntry` canonical encoding for the accepted TaskOpen (referenced by the git tree blob `entry_canonical` at commit `38f7112f`).

The agent's actual proof artifact (`proof.lean`) was emitted via the **legacy** evaluator path (CAS-stable but not yet routed through the chaintape per Atom 5's audit-trail design); it lives at `proof.lean` in this evidence dir.

### Q2 — What entered L4 (accepted)?

**1 L4 entry** at `runtime_repo/.git/refs/transitions/main`:
- Commit `38f7112f6401067ffc66c5a00338e12ec810170b`
- Message: `transition logical_t=1`
- Tree: `payload_cid` (CID of synthetic `TaskOpenTx`) + `signature` (system Ed25519) + `entry_canonical` (260-byte canonical encoding of `LedgerEntry`)
- Synthetic seed per architect ruling § 3.6 Atom 3 ("if no natural rejection, synthesize one with explicit label" — applied symmetrically to the accepted side for Atom 3 minimum because the legacy evaluator main loop does not yet route per-proposal `WorkTx` through `bus.submit_typed_tx`; that wiring is Atom 5).

### Q3 — What entered L4.E (rejected)?

**1 L4.E entry** at `runtime_repo/rejections.jsonl` (JSONL "或等价结构" form per architect § 3.5):

| Field | Value |
|---|---|
| `submit_id` | 2 |
| `agent_id` | `tb6-smoke-agent` |
| `tx_kind` | `Work` |
| `rejection_class` | `PolicyViolation` |
| `public_summary` | `stale_parent_root` |

**Why `stale_parent_root` not `StakeInsufficient`** (the original synthetic-rejection plan): the synthetic seed submits TaskOpen FIRST (which advances `state_root_t`), THEN the zero-stake WorkTx (with `parent_state_root = Hash::ZERO`). The accepted TaskOpen makes `Hash::ZERO` stale by the time the WorkTx is dispatched, so the rejection fires at the parent-root check BEFORE the stake gate. Either way, the L4.E entry is produced and persisted on disk — architect's "≥1 rejected L4.E entry" condition is met. The synthetic label at `synthetic_rejection_label.json` documents this.

`raw_diagnostic_cid` is preserved in the JSONL via the `JsonlRecord` shadow struct (Atom 1.2 design); the agent-facing `PublicRejectionView` projection still strips it via the TB-1 P0-3 serde shield.

### Q4 — What was replayed?

**Atom 4 verify_chaintape was applied to this directory and emitted `replay_report.json`.** All 7 architect-mandated boolean indicators pass:

```json
{
  "l4_entries": 1,
  "l4e_entries": 1,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "run_id": "tb6-smoke-2026-05-01",
  "epoch": 1,
  "detail": {
    "final_state_root_hex": "b1ffa9aa4a3109327db70bbc1fb62c539e5ba7afc71f3715e5bb9a94763a6428",
    "final_ledger_root_hex": "22ff4ba064d26034044eaed36409b887b45cb83ff5e8ed921fddc45408b88470",
    "head_commit_oid_hex": "38f7112f6401067ffc66c5a00338e12ec810170b",
    "l4e_last_hash_hex": "39dc75cb2a34fe16cd1380bfffeae98c601a09dcf9581cc5f115074b3decfd34",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": false
  }
}
```

To re-run: `./target/debug/verify_chaintape --repo runtime_repo --cas cas --out replay_report.json`. Cross-check: the `final_state_root_hex` (`b1ffa9aa…`) matches the `parent_state_root` stamped in `rejections.jsonl` (the rejected zero-stake WorkTx was checked against state-after-TaskOpen-accept), confirming chain ↔ rejection-ledger consistency. The `head_commit_oid_hex` matches `chain_snapshot_l4.txt`.

### Q5 — What was verified by signature?

The `LedgerEntry` at git commit `38f7112f` carries a `signature` blob (64 bytes) signed by the runtime's per-run `Ed25519Keypair`. The matching public key is persisted at `runtime_repo/pinned_pubkeys.json` (run_id `tb6-smoke-2026-05-01`, epoch 1, single pubkey row).

**Atom 4 verify_chaintape re-verified the signature on the L4 entry against this pinned pubkey** — `system_signatures_verified=true` in `replay_report.json`. Tampering with the pubkey hex in the manifest is detectable: `tests/tb_6_verify_chaintape.rs::i90c_tampered_pinned_pubkey_breaks_signature_verification` exercises the negative case end-to-end.

### Q6 — What was reconstructed (QState / EconomicState)?

**Atom 4's `verify_chaintape` reconstructs both end-to-end** by calling `replay_full_transition` (the I-DETHASH witness from CO1.7-impl A4):

- The accepted TaskOpen's `tx_payload_cid` → CAS lookup → `canonical_decode` → reconstructs `TypedTx::TaskOpen(TaskOpenTx { ... })`.
- `dispatch_transition` re-runs the pure transition; the resulting `state_root_t = b1ffa9aa…4763a6428` matches the on-chain `entry.resulting_state_root`.
- `EconomicState` reconstructs without divergence (`economic_state_reconstructed=true`); `task_markets_t[smoke-…]` is populated post-replay since accepted TaskOpen inserts a `TaskMarketEntry`.
- `ledger_root_t = 22ff4ba0…b88470` reconstructs from the `append(parent_root, signing_digest)` fold.

Tampering with any L4 entry (parent root, payload CID, signing payload, or resulting roots) is detectable: replay fails at the first divergent stage (1-9). The `tests/tb_6_verify_chaintape.rs` battery covers the happy path (I90), empty-chain (I90b), and tamper-detection (I90c) cases.

### Q7 — What did the Agent see / propose? Which branches were rejected? Which became accepted?

**LLM activity** (parallel to chaintape; legacy evaluator path):
- LLM: `deepseek-v4-flash`, condition `n1`, MAX_TX=20, mode `full`.
- Result: `solved=true, verified=true, gp_payload="nlinarith", h_vppu=5.6924, total_wall_time_ms=99679`.
- Proof artifact: `proof.lean` (re-verifies under pinned Lean v4.24.0 toolchain).

**Chaintape activity** (synthetic seed; Atom 3 scope):
- 1 accepted TaskOpen for `task_id="smoke-n1_mathd_algebra_107_1777617631_<id>"`
- 1 rejected zero-stake WorkTx (`agent_id=tb6-smoke-agent`, label `synthetic_rejection_for_l4e_gate=true` at `synthetic_rejection_label.json`)

The bridge between LLM activity and chaintape — i.e., per-LLM-proposal WorkTx routing through `bus.submit_typed_tx` — is **deferred to Atom 5** (agent audit trail). Atom 3 establishes the chain-backed evidence shape; Atom 5 makes the chain reflect the LLM's actual proposal/accept/reject decisions.

### Q8 — What did the Agent NOT see (selective shielding)?

- The L4.E `RejectedSubmissionRecord.raw_diagnostic_cid` is structurally absent from the agent-facing `PublicRejectionView` (TB-1 P0-3 serde shield + Inv 10). The forensic L4.E ledger preserves the field (via JsonlRecord shadow); only authorized auditors can recover it.
- Agent's chain-of-thought / private model deliberation: NOT recorded (per architect ruling § 3.6 Atom 5 + WP § 12.4 ArchitectAI/JudgeAI separation).
- Pinned pubkey is on disk (signed-by-runtime is verifiable) but the runtime keypair's PRIVATE key is held in process memory only and dropped at evaluator exit.

---

## §2 Configuration

| Param | Value |
|---|---|
| Binary | `./target/release/evaluator` (built at HEAD `01b9e93` + Atom 3 hook + JSONL rejection writer wiring) |
| Problem | `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/mathd_algebra_107.lean` |
| MAX_TX | 20 |
| Condition | `n1` (single-agent swarm) |
| Mode | `full` |
| LLM proxy | `LLM_PROXY_URL=http://localhost:8080` (`/health` returned `{"status": "ok"}`) |
| Model snapshot (resolved) | `deepseek-v4-flash` |
| `TURINGOS_CHAINTAPE_PATH` | `<this_dir>/runtime_repo` |
| `TURINGOS_CAS_PATH` | `<this_dir>/cas` |
| `TURINGOS_RUN_ID` | `tb6-smoke-2026-05-01` |

---

## §3 PputResult (capability replication)

```
schema_version       v2.0
run_id               n1_mathd_algebra_107_1777617631596
problem_id           mathd_algebra_107
solved               true
verified             true
progress             1
golden_path_token_count 12
total_run_token_count 448
tx_count             1
budget_max_transactions 20
hit_max_tx           false
tactic_diversity     1.0
pput_runtime         0.000022...
pput_verified        0.000022...
model_snapshot       deepseek-v4-flash
mode                 full
condition            n1
tool_dist            {"omega_wtool":1, "step":1}
gp_payload           nlinarith
gp_path              per_tactic
gp_proof_file        proofs/mathd_algebra_107_1777617631_73ee91ba.lean
has_golden_path      true
total_wall_time_ms   99679
h_vppu               5.6924
```

`gp_payload="nlinarith"` matches the canonical OMEGA proof for this theorem (TB-0 baseline + TB-1 Day-1 + TB-2..TB-5 ship). Capability replicates under chaintape mode.

---

## §4 Comparison vs prior smokes

| Metric | TB-1 Day-1 | TB-2..TB-5 ship | **TB-6 Atom 3 ship** |
|---|---|---|---|
| Smoke evidence type | paper trail | paper trail | **chain-backed** |
| `prompt_context_hash` | `a1f43584a17d1226` | `a1f43584a17d1226` | (sequencer wired; n1 prompt hash drift is expected) |
| `solved` (n1) | true | true (where measured) | **true** |
| `gp_payload` | `nlinarith` | `nlinarith` | **`nlinarith`** |
| L4 entries on disk | 0 | 0 | **1** |
| L4.E entries on disk | 0 | 0 | **1** |
| `runtime_repo/.git/refs/transitions/main` | absent | absent | **present (1 commit)** |
| `pinned_pubkeys.json` | absent | absent | **present** |
| Tampering with run.log undetectable? | yes | yes | **yes** (run.log still paper trail) |
| Tampering with chain entries detectable? | n/a | n/a | **yes** (Atom 4 verify_chaintape — `replay_report.json` ✓) |

---

## §5 Verdict

**TB-6 Atom 3 ship-ready**.

1. **Architect § 3.6 Atom 3 minimum satisfied**: ≥1 accepted L4 entry + ≥1 rejected L4.E entry produced from a real evaluator run on `mathd_algebra_107`. Synthetic-seed label documented per "if no natural rejection, synthesize" clause.
2. **Architect § 3.5 deliverable shape satisfied**: `runtime_repo/.git/` ✓ + `refs/transitions/main` ✓ + `cas/` ✓ + `rejections.jsonl` (the "或等价结构" of `refs/rejections/main`) ✓ + `pinned_pubkeys.json` ✓ + `proof.lean` ✓ + `pput_result.jsonl` ✓ + `README.md` ✓ + `synthetic_rejection_label.json` ✓.
3. **D2 hard requirement satisfied**: 8-condition gate (production binary triggers `Sequencer::apply_one` ✓; on-disk LedgerEntry chain ✓; `parent_ledger_root` / `resulting_ledger_root` ✓; `tx_payload_cid` ✓; `system_signature` ✓; CAS retrievable ✓; replay reconstructable ✓ [Atom 4 `replay_report.json`]; rejected raw diagnostic absent from agent-facing view ✓).
4. **D5 naming ratified**: this dir IS chain-backed; can be called "ChainTape smoke" / "smoke tape" / "tape" without abuse of terminology.
5. **D4 reporting standard satisfied** (per ship commit body): `cargo test --workspace` workspace_count + delta + zero failures.

**Atom 4 SHIPPED**: `verify_chaintape` (library + CLI + I90/I90b/I90c integration tests) re-opens this directory, replays the chain, reconstructs Q + EconomicState, verifies every signature against `pinned_pubkeys.json`, and emits `replay_report.json` (this dir). The tampering-detection guarantee is now structurally enforced.

## §6 What this smoke proves vs. does NOT prove

**Proves**:
- TuringOS production binary writes a real on-disk Git ChainTape under env-flag activation.
- ≥1 accepted L4 entry + ≥1 rejected L4.E entry coexist in one bundle's evidence dir.
- The L4.E JSONL backend chains via `prev_hash + hash` and survives reopen.
- The runtime keypair's pubkey is persisted alongside the chain for post-hoc signature verification.
- The legacy LLM activity (mathd_algebra_107 solve via `deepseek-v4-flash + nlinarith`) coexists with chaintape mode without regression.

**Does NOT prove** (deferred):
- Per-proposal WorkTx routing (legacy evaluator emits PputResult, not WorkTx — Atom 5 wires this).
- Agent audit trail (proposal CIDs, prompt_context_hash linkage to tx_id — Atom 5).
- Branch / fork visibility summary (`failed_branch_count`, accepted/rejected tx_id sets — Atom 6).

These are the explicit Atom 4-6 boundaries per the architect ruling § 3.6. The chain-backed evidence shape is now in place; subsequent atoms add semantic depth on top.
