#!/usr/bin/env bash
set -euo pipefail
cd /Users/zephryj/work/turingosv4-probe-gpqa
set -a; . ./.env; set +a
exec python3 src/drivers/llm_proxy.py --port 8123
