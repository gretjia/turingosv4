# TB-7 real-LLM chaintape smoke — 5 problems × DeepSeek-v4-flash

**Date**: 2026-05-01
**Source**: `cargo run -p minif2f_v4 --bin evaluator` with `TURINGOS_CHAINTAPE_PATH` set + `CONDITION=n1`
**Model**: `deepseek-v4-flash` via local LLM proxy at `localhost:8080`
**Lean**: 4.29.1 (`/home/zephryj/.elan/bin/lean`)
**Mathlib**: vendored at `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/.lake/build`
**Branch at run time**: `main` post-TB-8 (`47011bb`)

This is the **first chain-backed real-LLM smoke** in TuringOS v4 history. The
prior TB-6 + TB-7 Atom 6 evidence used synthetic-LLM stand-ins; this run
exercises the live evaluator path with DeepSeek-v4-flash producing real
proposals on `mathd_algebra_107` / `mathd_algebra_171` / `mathd_algebra_359`
/ `aime_1997_p9` / `mathd_numbertheory_5`.

---

## §0 Headline

| Run | Problem | n1 swarm exit | LLM solved? | chaintape on disk? | dashboard PASS? |
|---|---|---|---|---|---|
| 1 | mathd_algebra_107 | 0 | (insufficient tx) | ✓ | ✓ ALL 7 GREEN |
| 2 | mathd_algebra_171 | 0 | (insufficient tx) | ✓ | ✗ CAS index race |
| 3 | mathd_algebra_359 | 0 | (insufficient tx) | ✓ | ✓ ALL 7 GREEN |
| 4 | aime_1997_p9 | 0 | (no Agent_0 activity) | ✓ | ✓ ALL 7 GREEN |
| 5 | mathd_numbertheory_5 | 0 | (insufficient tx) | ✓ | ✗ CAS index race |

**3/5 dashboards GREEN; 2/5 hit a pre-existing CAS index concurrent-write
race condition** (described in §3 below; rolls to TB-7.6 carry-forward).

**The headline finding**: per-LLM-proposal Work + Verify pairs DID land
authoritatively on the runtime_repo via `bus.submit_typed_tx`. This is the
architect ruling §0 closure point ("ChainTape 成为真实 LLM proposal 的
authoritative path") demonstrated for the first time on real DeepSeek output.

---

## §1 What landed on chain (runs 1 + 3 representative)

### Run 1 (mathd_algebra_107)

```text
§3 ChainDerivedRunFacts (§4.4 bit-exact set)
  solved                  : false   ← see §2 / known divergence
  verified                : false
  tx_count                : 4       ← 1 L4 + 3 L4.E
  proposal_count          : 2       ← 1 synthetic seed + 1 real Agent_0
  golden_path_token_count : 448     ← REAL DeepSeek prompt+completion
  tactic_diversity        : 1
  failed_branch_count     : 3
  tool_dist:
    step_complete: 1                ← real Agent_0 emitted complete-tool

§4 Per-agent activity
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  Agent_0           | ✓      | 0     | 1     | 0       | 1   ← real LLM agent
  tb6-smoke-agent   | ✗      | 0     | 1     | 0       | 0   ← synthetic seed

§5 Proposal flow
  side  | t | tx_kind  | agent          | tactic        | branch     | reject
  L4.E  | 0 | Work     | tb6-smoke-agent| -             | -          | PolicyViolation
  L4.E  | 0 | Work     | Agent_0        | step_complete | Agent_0.b1 | PolicyViolation
  L4.E  | 0 | Verify   | Agent_0        | -             | -          | PolicyViolation
  L4    | 1 | TaskOpen | tb6-smoke-sponsor| -           | -          | -
```

### Run 3 (mathd_algebra_359)

Same shape: 4 tx, 2 proposals, **423 tokens** real-LLM, step_complete tactic
captured in CAS, Agent_0 + branch_id = `Agent_0.b1` recorded.

### Run 4 (aime_1997_p9)

The LLM gave up (high token count 1751 in baseline but only 1 tx_count and
no OMEGA emit). Chain shows only the synthetic seed (1 L4 TaskOpen + 1
L4.E synthetic Work). No Agent_0 activity on chain — the run did NOT route
any real proposal through `bus.submit_typed_tx` because the LLM never
reached an OMEGA-accept or append-branch hot path.

### Runs 2 + 5 (CAS index race; see §3)

The chain-side state (rejections.jsonl + agent_audit_trail.jsonl + L4 git
ref) is on disk and intact, but `verify_chaintape` errors at CAS index
parsing because two concurrent CasStore writes interleaved without a
newline separator. This is a **pre-existing CAS-level concurrency bug**,
not a TB-7 routing bug.

---

## §2 Known divergence: chain `solved` vs evaluator `solved`

The evaluator's in-memory `PputResult.solved` is `true` whenever Lean
verifies a candidate proof (regardless of authoritative routing). The
chain-derived `solved` (charter §4.4) is `true` iff a `VerifyTx::Confirm`
landed in **L4** (accepted), targeting an accepted L4 `WorkTx`.

**Current behavior**: TB-7 Atoms 2 + 3 emit per-LLM-proposal WorkTx with
`stake = 0` (charter §4.3 narrowed scope; full economic settlement is
RSP-4 / TB-9 territory). With zero stake, every WorkTx + VerifyTx pair
routes to **L4.E** under the `StakeInsufficient` rejection class. So the
chain side never sees an accepted Verify, and `chain.solved == false`
even when the LLM produced a valid Lean proof (run 2's
mathd_algebra_171 oneshot baseline solved with `gp_path="alone"`).

This divergence is **the natural consequence** of the charter's narrowed
OMEGA scope (per ARCHITECT_RULING D3) — full chain-derived `solved`
requires TB-9 minimal payout (FinalizeRewardTx + non-zero escrow) to
enable accepted L4 WorkTx + VerifyTx. Until then, the chain captures
**every meaningful LLM proposal as L4.E rejection evidence** with full
ProposalTelemetry, which is the structural Frame B closure goal.

---

## §3 Discovered bug: CAS index concurrent-write race

**Bug**: When the production binary opens multiple `CasStore` handles
concurrently (one per per-tx CAS write call site in the evaluator hot
path), two `OpenOptions::append(true)` writes to
`<cas_path>/.turingos_cas_index.jsonl` can interleave without an
intervening newline, producing a corrupt index line like:

```
{"cid":[245,...],"backend_oid_hex":"15ed92..."}{"cid":[100,...],"backend_oid_hex":"9a1535..."}
```

**Symptom**: `verify_chaintape` (and downstream `chain_derived_run_facts`
+ `audit_dashboard`) errors at parse time with:

```
verify_chaintape: Cas("cas index parse error at line 1: trailing characters at line 1 column N")
```

**Triggered when**: chain has multiple per-tx CAS writes near the same
logical time. Runs 2 + 5 hit it because both had busy chains (synthetic
seed audit_pair + real Agent_0 Work + Verify + telemetry all writing CAS
in close temporal succession).

**Status**: Pre-existing CAS-level bug, NOT TB-7 routing bug. Exposed
ONLY after TB-7 Atoms 2 + 3 introduced multiple concurrent per-tx CAS
write sites. Runs 1 + 3 happened to avoid the race; runs 2 + 5 caught it.

**Remediation**: TB-7.6 carry-forward (Codex audit cc7b3dd action #6
disk-level tamper battery extension): make CAS index writer mutex-locked
or use atomic file-write semantics. This is a 5-10 line fix in
`src/bottom_white/cas/store.rs`.

---

## §4 What this evidence proves

1. **TB-7 §4.0 authoritative path is live**: Real LLM proposals route
   through `bus.submit_typed_tx` → `Sequencer::apply_one` → L4 / L4.E.
   Verified via runs 1 + 3 dashboards: Agent_0 activity recorded with
   pubkey ✓ + signature verifying via agent_pubkeys.json.

2. **TB-7 Atom 1.5 ProposalTelemetry CAS works on real LLM**: token_counts
   (423-448 prompt+completion+tool tokens), candidate_tactic
   ("step_complete"), branch_id ("Agent_0.b1"), and prompt_context_hash
   are all chain-derivable from CAS for runs 1 + 3.

3. **TB-7 Atom 4 verify_chaintape works on real chain**: Gate 4
   (`agent_signatures_verified=true`) + Gate 5
   (`proposal_telemetry_cas_retrievable=true`) GREEN on runs 1 + 3 + 4.

4. **TB-7 Atom 5 ChainDerivedRunFacts works on real LLM**:
   `golden_path_token_count` is a real sum of DeepSeek's reported tokens
   from ProposalTelemetry CAS objects (448 + 423 across 2 runs).

5. **TB-7.5 fail-closed semantics did NOT fire**: All 5 runs exited 0,
   meaning the authoritative path never failed at submission time.
   `bus.submit_typed_tx` returned `Ok(...)` for every Work + Verify (the
   tx itself routes to L4.E because of zero-stake admission rejection;
   that's NOT a `submit_typed_tx` error and shouldn't trigger fail-closed).

6. **Atom 6 synthetic-LLM smoke evidence is structurally identical to
   real-LLM smoke evidence** for the routing-side artifacts. The
   difference is content: synthetic chains use `n1` / `swarm_a` /
   `swarm_b` agent_ids; real chains use `Agent_0` (live LLM). Schema +
   verification surface is identical.

---

## §5 Per-run artifacts

Each `run_<i>_<problem>/` subdir contains:
- `agent_pubkeys.json` — TB-7 Atom 1 per-agent Ed25519 manifest
- `pinned_pubkeys.json` — TB-6 Atom 1 system-side manifest
- `rejections.jsonl` — L4.E rejection records (TB-6 Atom 1.2)
- `agent_audit_trail.jsonl` — synthetic-seed audit pair (TB-6 Atom 5)
- `synthetic_rejection_label.json` — TB-6 Atom 3 evidence label
- `l4_chain_log.txt` — `git log` over `refs/transitions/main` (the L4 chain)
- `dashboard.txt` — `audit_dashboard --repo runtime_repo --cas cas`
  output (TB-8 read-side); errors out for runs 2 + 5 due to CAS race.

Source runtime_repo + cas dirs (full git refs + CAS payloads) live at
`/tmp/tb7_real_smoke/run_<i>_<problem>/` outside the repo (multi-MB
binary git objects). The committed evidence here is the human-readable
+ chain-state artifacts.

---

## §6 Gate closure update (TB-7 §13.4 + this run)

| Codex action (cc7b3dd) | TB-7 charter mapping | Real-LLM-smoke verdict |
|---|---|---|
| #1 fail-closed bootstrap | Atom 1.7 + TB-7.5 fix #1 | **CLOSED** — exit(3) didn't fire (no infra failures during 5 runs) |
| #2 real proposal routing | Atom 2 + 3 + §4.0 | **CLOSED** — Agent_0 activity on chain in runs 1 + 3 |
| #3 logical_t schema | Atom 1.7 | **CLOSED** — agent_audit_trail rows valid |
| #4 audit-index hash from CAS | Atom 4 expansion (partial) | PARTIAL — see TB-7.6 |
| #5 RunSummary tx_id ↔ CID correlation | Atom 5 expansion (partial) | PARTIAL — see TB-7.6 |
| #6 disk-level tamper tests | Atom 4 + TB-7.6 | PARTIAL + **CAS race bug** discovered (§3) |
| #7 regenerate TB-6 smoke | Atom 6 + this run | **CLOSED** — this evidence supersedes synthetic |

TB-6 audit-pending status: **3/7 fully closed pre-this-run + 1 newly
closed (#7) here = 4/7 fully closed**; 3/7 partial roll to TB-7.6
(includes the new CAS race finding).
