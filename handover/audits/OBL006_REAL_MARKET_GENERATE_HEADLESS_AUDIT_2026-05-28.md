# OBL-006 Real Market Generate Headless Audit — 2026-05-28

## Scope

- Task: OBL-006 web/generate session path must use one real market kernel.
- Risk class: Class 3, market/economic state.
- FC nodes: FC1 wtool/ChainTape typed tx economic route; FC2 web/CLI unified generate path. FC3 unchanged.
- Audited branch: `codex/real-market-generate-kernel`.
- Touched files: `OBLIGATIONS.md`, `src/bin/turingos/cmd_generate.rs`, `src/web/market_view.rs`, `tests/generate_emits_work_tx_smoke.rs`.

## Deterministic Evidence

- `git diff --check`: exit 0.
- `rustfmt --edition 2021 --check src/bin/turingos/cmd_generate.rs src/web/market_view.rs tests/generate_emits_work_tx_smoke.rs`: exit 0.
- `cargo check --workspace`: exit 0.
- `cargo test --test generate_emits_work_tx_smoke -- --nocapture`: 4 passed.
- `cargo test --bin turingos_web --features web derive_yes_signal -- --nocapture`: 2 passed.
- `cargo test --test constitution_web_cli_kernel_invariant -- --nocapture`: 2 passed.
- `cargo test --test constitution_real6_task_outcome_market -- --nocapture`: 14 passed.
- `cargo test --test constitution_router_buy_with_coin -- --nocapture`: 10 passed.
- `cargo test --test constitution_matrix_drift -- --nocapture`: 3 passed.
- `cargo test --test constitution_obligation_repair_reconciliation -- --nocapture`: 3 passed.
- `bash scripts/run_constitution_gates.sh`: `[k-1-5] total=165 failed=0`.
- `cargo test --workspace --no-fail-fast`: exit 0.

## Headless Witnesses

Claude no-tool diff audit:

```json
{"agent":"claude","task_id":"OBL-006-real-generate-market","workspace":"/tmp/turingosv4-real-market-clean","verdict":"NO-VIOLATION"}
```

Claude checked that `generate` threads roots as `current_root -> MarketSeed -> CpmmPool -> BuyWithCoinRouter`, sets `VerifyTx.parent_state_root` to the router root, keeps web as a read-only canonical replay projection, and introduces no Class 4 restricted edits or float money math. Claude flagged one non-blocking OBL branch-label typo; the ledger text was corrected before final verification.

AGY retry audit:

```json
{"agent":"agy","task_id":"OBL-006-real-generate-market","workspace":"/tmp/turingosv4-real-market-clean","verdict":"NO-VIOLATION"}
```

AGY checked that web calls the shared CLI generate route rather than a second generate kernel, the ChainTape economics now admit `MarketSeed -> CpmmPool -> BuyWithCoinRouter -> Verify`, `yes_signal_bp` is derived from replayed CPMM state, no Class 4 schema/signing/admission files were edited, and changed market math remains integer-only.

The first AGY audit attempt timed out waiting for response and is not counted as audit evidence.

## Verdict

`NO-VIOLATION`
