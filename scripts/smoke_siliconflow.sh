#!/usr/bin/env bash
# Phase A atom A7 — SiliconFlow integration smoke.
#
# Probes each of the 3 SiliconFlow keys (primary / secondary / tertiary)
# with a minimal chat call against a cheap model. Reports per-key OK or
# FAIL without printing key material. Exits non-zero if any key fails.
#
# Why direct SDK probes (not via the proxy round-robin): we want a
# per-key verdict — the proxy's round-robin would obscure which specific
# key failed if one is rate-limited / revoked. After A7 PASSes, the
# evaluator's runtime path goes through llm_proxy.py.
#
# Cost: 3 calls × ~50 tokens each = ¥0.001 - ¥0.005 total. SiliconFlow
# Qwen2.5-7B-Instruct free tier covers this; backstop is the user's
# key budget. Aborts after the first key fails to bound spend.
#
# Usage:
#   bash scripts/smoke_siliconflow.sh
#
# Reads keys from .env (auto-loaded) or current shell env.

set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$(pwd)"

# Source .env if present; do not echo any value.
if [ -f .env ]; then
    set -a
    # shellcheck disable=SC1091
    . .env
    set +a
fi

# Sanity: at least the primary key must be set.
: "${SILICONFLOW_API_KEY:?SILICONFLOW_API_KEY not set; configure .env first}"

python3 "$ROOT/scripts/_smoke_siliconflow.py"
