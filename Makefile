# TuringOS v4 — Makefile
#
# Convenience entry points. Authoritative tooling is `cargo` + scripts/.
# Per TB-C0 (Constitution Landing Gate): `make constitution` is the canonical
# pre-merge smoke; CI runs the same script via `scripts/run_constitution_gates.sh`.

.PHONY: help constitution constitution-watch test workspace-test fmt clippy clean

help:
	@echo "TuringOS v4 Makefile targets:"
	@echo ""
	@echo "  make constitution        Run all 8 TB-C0 constitution gate test files"
	@echo "                           (writes target/constitution_gate_report.json)"
	@echo ""
	@echo "  make constitution-watch  Run constitution gates on file change (cargo-watch)"
	@echo ""
	@echo "  make test                cargo test --workspace --no-fail-fast"
	@echo "  make workspace-test      Same as 'make test' (alias)"
	@echo "  make fmt                 cargo fmt --all"
	@echo "  make clippy              cargo clippy --workspace --tests"
	@echo "  make clean               cargo clean"
	@echo ""
	@echo "TB-C0 charter:    handover/tracer_bullets/TB-C0_charter_2026-05-06.md"
	@echo "TB-C0 directive:  handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md"

constitution:
	@bash scripts/run_constitution_gates.sh

constitution-watch:
	@cargo watch -x 'test --test constitution_fc1_runtime_loop --test constitution_fc2_boot --test constitution_fc3_meta --test constitution_predicate_gate --test constitution_shielding_gate --test constitution_economy_gate --test constitution_tape_canonical_gate --test constitution_no_parallel_ledger'

test:
	@cargo test --workspace --no-fail-fast

workspace-test: test

fmt:
	@cargo fmt --all

clippy:
	@cargo clippy --workspace --tests --no-deps

clean:
	@cargo clean
