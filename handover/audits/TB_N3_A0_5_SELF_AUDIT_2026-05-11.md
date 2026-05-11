# TB-N3 A0.5 — MarketMakerBudget Genesis Preseed — Self-Audit Dossier

**Class**: 4 STEP_B (genesis preseed schema change; bootstrap.rs in trust_root)
**Date**: 2026-05-11
**Branch**: `feat/tb-n3-autorun-20260511T051910Z`
**Authority**: Architect ruling 2026-05-11 (`TuringOS_TB_N3_Polymarket_Agent_Bridge_Ruling_2026-05-11.md`)
amendment 6 ("如果没有：新增 A0.5 MarketMakerBudget precondition atom. 改 on_init / genesis schema = Class 4")
+ Q1 ("DEFAULT_POOL_SEED = 100_000 microCoin … 建议 MarketMakerBudget 至少 5_000_000 microCoin")
+ Q2 ("A3 不能开始，直到 MarketMakerBudget 已验证存在").

User authorization: 2026-05-11 verbatim "approve the plan and auto run" (autonomous execution
grant for TB-N3 plan as written).

## Surface diff

| File | Change |
|------|--------|
| `src/runtime/bootstrap.rs` | `default_pput_preseed_pairs()` 12 → 13 entries; +`MarketMakerBudget` 5_000_000 micro. Tests `returns_12_entries` → `returns_13_entries`; `total_preseed_supply_30m` → `total_preseed_supply_35m`; +`market_maker_budget_present_with_5m_micro` (U9). |
| `tests/tb_16_halt_triggers.rs` | `h13_total_supply_mutates_halts` updated 30_000_000 → 35_000_000. |
| `src/bin/audit_dashboard.rs` | §16 sandbox banner string `30_000_000 μC` → `35_000_000 μC`. |
| `genesis_payload.toml` | Trust Root rehash for `src/runtime/bootstrap.rs`, `src/bin/audit_dashboard.rs`, `src/state/typed_tx.rs` (TB-N3 A0 helper). |

## Self-audit (Codex G2 + Gemini DT patterns; in-process)

### (i) No new mint path

`assert_no_post_init_mint` continues to fire on every typed_tx after on_init; the
new `MarketMakerBudget` row is part of the genesis QState construction
(`runtime::adapter::genesis_with_balances` consumes the preseed list at boot;
`evaluator::run_swarm` and `lean_market` bootstrap both call this single factory).
No new mint code path was introduced. **PASS.**

### (ii) `assert_no_post_init_mint` still passes

Verified via `cargo test -p turingosv4 --test tb_16_halt_triggers
h13_total_supply_mutates_halts` (PASS at 35_000_000) and
`cargo test -p turingosv4 --lib runtime::bootstrap::tests` (9/9 PASS including
new `market_maker_budget_present_with_5m_micro` and updated total assertion).
No post-init mint surface added. **PASS.**

### (iii) Total supply equals sum of new preseed list

| Entry | Micro |
|-------|-------|
| `tb7-7-sponsor` | 10_000_000 |
| `Agent_user_0` | 10_000_000 |
| `Agent_0`..`Agent_9` | 10 × 1_000_000 = 10_000_000 |
| `MarketMakerBudget` (NEW) | 5_000_000 |
| **Total** | **35_000_000** |

Verified by `total_preseed_supply_35m` test. **PASS.**

### (iv) Replay determinism preserved

`default_pput_preseed_pairs()` remains pure (no env reads, no clock, no
randomness); `deterministic_across_calls` test (U7) PASS. Two calls produce
byte-identical Vec output. Past chains (with prior 30_000_000 genesis) continue
to replay from their on-disk genesis_report.json regardless of this edit; only
fresh bootstraps consume the current factory version (TB-10 Atom 1 design
contract preserved). **PASS.**

### (v) No shadow ledger

`MarketMakerBudget` is a single ordinary preseed row: it appears in
`balances_t`, never in any other ledger, never in dashboard cache, never in
markov capsule. The TB-N3 A3 emit helper signs canonical TaskOpen/MarketSeed/
CpmmPool tx via the agent ingress path — no system-emit bypass; no direct
mutation of `cpmm_pools_t` / `task_markets_t`. **PASS.**

## Trust Root verification

```
cargo test -p turingosv4 --lib boot::tests::verify_trust_root_passes_on_intact_repo
→ test boot::tests::verify_trust_root_passes_on_intact_repo ... ok
```

All three TB-N3 file rehashes (bootstrap.rs, audit_dashboard.rs, typed_tx.rs)
match repo state. **PASS.**

## Caller impact

Direct callers of `default_pput_preseed_pairs()` — verified via
`grep -rn default_pput_preseed_pairs src/`:

- `src/runtime/adapter.rs::genesis_with_balances` (consumed at chaintape
  bootstrap; no semantic change — just one extra balance entry).
- `experiments/minif2f_v4/src/bin/evaluator.rs::run_swarm` `--task-mode self|both`
  preseed branch (no change required; agent identities iterate over the Vec).
- `lean_market` user CLI bootstrap (no change required; same iteration pattern).

No caller hardcodes count or total supply; both relevant test sites
(`bootstrap.rs::tests` and `tb_16_halt_triggers.rs::h13`) updated.

## Forward dependency

A3 (`tb_n3_emit_node_market_after_work_accept`) signs TaskOpen/MarketSeed/CpmmPool
tx with `provider="MarketMakerBudget"`. `AgentKeypairRegistry::sign` auto-generates
a keypair on first call (existing behavior at `src/runtime/agent_keypairs.rs:294`),
so MarketMakerBudget needs no explicit registration. The 5M micro budget covers
~50 pool seeds at the architect Q1 default 100k micro / pool — well above the
expected ~5 accepted WorkTx per 9-problem batch (Phase 1 baseline).

## Verdict

A0.5 SHIP-READY. Class-4 STEP_B self-audit clean across (i)-(v); trust root
verified; tests green. A3 may proceed against this substrate.
