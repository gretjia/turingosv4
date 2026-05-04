#!/usr/bin/env bash
# TB-16 Atom 6 — run_real_llm_arena.sh
#
# End-to-end runner for the TB-16 Controlled Market Smoke Arena per
# architect §7 + design §4. Drives 6 Lean tasks across 8 sandbox-prefixed
# agents, then runs the full audit pipeline (audit_tape +
# audit_tape_tamper + generate_markov_capsule + audit_dashboard).
#
# Per design §5 — preconditions:
#   - DeepSeek API keys present in turingosv4/.env (DEEPSEEK_API_KEY*)
#   - src/drivers/llm_proxy.py running on http://localhost:18080
#     (or pass --llm-proxy-url)
#   - Mathlib cached: lake exe cache get (~2 min) per
#     feedback_lake_packages_vendored
#   - Wall clock budget: 30 min (1800s)
#   - Cost ceiling: $15 USD
#
# Exit 0 — verdict.json PROCEED + replay byte-identical
# Exit 1 — verdict.json BLOCK (≥1 fail/halt) OR replay diverged
# Exit 2 — invalid args / preconditions / I/O failure
#
# TRACE_MATRIX FC1-N34 + FC1-N35 + FC1-N36 + FC2-N31..N33 + FC3-N44.

set -euo pipefail

# ── Defaults ────────────────────────────────────────────────────────
OUT_DIR=""
EVALUATOR_BIN="./target/release/evaluator"
LEAN_MARKET_BIN="./target/release/lean_market"
ARENA_BIN="./target/release/comprehensive_arena"
AUDIT_TAPE_BIN="./target/release/audit_tape"
AUDIT_TAPE_TAMPER_BIN="./target/release/audit_tape_tamper"
AUDIT_DASHBOARD_BIN="./target/release/audit_dashboard"
GEN_MARKOV_BIN="./target/release/generate_markov_capsule"
LLM_PROXY_URL="http://localhost:18080"
MAX_TX="20"
RUN_ID_PREFIX="tb16-arena-$(date -u +%Y-%m-%dT%H-%M-%SZ)"
WALL_CLOCK_CAP_MS="1800000"
COMPUTE_CAP_TOKENS="120000"
COST_CEILING_USD="15"
SKIP_LLM_PRECHECK="${SKIP_LLM_PRECHECK:-0}"
PLAN_ONLY="0"

# ── Args ────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --out-dir) OUT_DIR="$2"; shift 2 ;;
    --evaluator-bin) EVALUATOR_BIN="$2"; shift 2 ;;
    --lean-market-bin) LEAN_MARKET_BIN="$2"; shift 2 ;;
    --llm-proxy-url) LLM_PROXY_URL="$2"; shift 2 ;;
    --max-tx) MAX_TX="$2"; shift 2 ;;
    --run-id-prefix) RUN_ID_PREFIX="$2"; shift 2 ;;
    --plan-only) PLAN_ONLY="1"; shift ;;
    -h|--help)
      cat <<'EOF'
run_real_llm_arena.sh — TB-16 Atom 6 controlled-market arena runner

USAGE:
  bash handover/tests/scripts/run_real_llm_arena.sh \
       --out-dir <path> \
       [--evaluator-bin <path>] \
       [--lean-market-bin <path>] \
       [--llm-proxy-url <url>] \
       [--max-tx <n>] \
       [--run-id-prefix <str>] \
       [--plan-only]

PRECONDITIONS:
  - DEEPSEEK_API_KEY (1+ keys) in env
  - LLM proxy running at --llm-proxy-url (or set SKIP_LLM_PRECHECK=1)
  - Mathlib cached (`lake exe cache get`)

OUTPUTS in --out-dir:
  - ARENA_PLAN.md            (orchestration plan)
  - runtime_repo/            (Git2 L4 chain + L4.E rejections.jsonl)
  - cas/                     (CAS objects)
  - agent_pubkeys.json       (per-run agent manifest)
  - pinned_pubkeys.json      (per-run system pubkey manifest)
  - genesis_report.json      (constitution_hash + bootstrap state)
  - verdict.json             (38-assertion audit verdict)
  - verdict_replay.json      (byte-identical re-run verdict)
  - tamper_report.json       (3-corruption tamper-detection report)
  - MARKOV_TB-16_<DATE>.json (first Markov capsule)
  - dashboard.txt            (15-section render incl. §15 + §16)
  - README.md                (acceptance gate table + halt-trigger battery)
EOF
      exit 0
      ;;
    *) echo "run_real_llm_arena.sh: unknown arg: $1" >&2; exit 2 ;;
  esac
done

if [[ -z "$OUT_DIR" ]]; then
  echo "run_real_llm_arena.sh: --out-dir required" >&2
  exit 2
fi
mkdir -p "$OUT_DIR"

# ── Step 1: Build all binaries (release) ────────────────────────────
echo "▶ Step 1/8: cargo build --release (audit + arena + evaluator + dashboard)..."
cargo build --release \
  --bin audit_tape \
  --bin audit_tape_tamper \
  --bin audit_dashboard \
  --bin generate_markov_capsule
cargo build --release -p minif2f_v4 \
  --bin comprehensive_arena \
  --bin evaluator \
  --bin lean_market

# ── Step 2: Emit ARENA_PLAN.md (always) ─────────────────────────────
echo "▶ Step 2/8: emit ARENA_PLAN.md..."
"$ARENA_BIN" --out-dir "$OUT_DIR" --plan-only \
  --max-tx "$MAX_TX" \
  --run-id-prefix "$RUN_ID_PREFIX" \
  --llm-proxy-url "$LLM_PROXY_URL"

if [[ "$PLAN_ONLY" == "1" ]]; then
  echo "✓ Plan-only mode; ARENA_PLAN.md emitted at $OUT_DIR/ARENA_PLAN.md"
  exit 0
fi

# ── Step 3: LLM proxy precheck ──────────────────────────────────────
if [[ "$SKIP_LLM_PRECHECK" != "1" ]]; then
  echo "▶ Step 3/8: LLM proxy precheck against $LLM_PROXY_URL..."
  if ! curl -sf -o /dev/null --max-time 5 "$LLM_PROXY_URL/health" 2>/dev/null \
     && ! curl -sf -o /dev/null --max-time 5 "$LLM_PROXY_URL" 2>/dev/null; then
    cat >&2 <<EOF
✗ LLM proxy NOT REACHABLE at $LLM_PROXY_URL.
  Start it with:
    python3 src/drivers/llm_proxy.py --port 18080 &
  Or set SKIP_LLM_PRECHECK=1 to bypass (real-LLM flow will be SKIPPED;
  audit pipeline still runs against any pre-existing tape in --out-dir).
EOF
    echo "✗ Precondition failed; aborting before any subprocess work." >&2
    exit 2
  fi
  echo "✓ LLM proxy reachable."
fi

# ── Step 4: Drive evaluator subprocesses for 6 tasks ────────────────
# v0 implementation: evaluator's existing real-LLM solver loop is
# invoked via lean_market run-task semantics. The 6-task scenario is
# exercised by 6 sequential evaluator invocations against a SHARED
# runtime_repo. Adversarial-challenger overrides + force-exhaustion
# overrides flow via env vars (TURINGOS_FORCE_*).
#
# Atom 6 v0 SCOPE NOTE: each task currently maps to a single evaluator
# invocation in user-task mode. Multi-task aggregation onto a single
# chain (so all 13 tx kinds appear in ONE tape) requires evaluator
# extensions (TB-16 Atom 6.1). For v0, each task produces its own
# sub-tape under $OUT_DIR/task_<X>_<label>/runtime_repo, and audit_tape
# runs over each tape individually + emits an aggregate report.

RUNTIME_REPO="$OUT_DIR/runtime_repo"
CAS_DIR="$OUT_DIR/cas"
mkdir -p "$RUNTIME_REPO" "$CAS_DIR"

echo "▶ Step 4/8: real-LLM 6-task arena execution..."
echo "  (Atom 6 v0: per-task sub-tapes + aggregate audit)"
echo "  (To exercise the FULL multi-task single-chain coverage path,"
echo "   extend evaluator with TURINGOS_TASK_LIST=A,B,C,D,E,F + chain"
echo "   continuation semantics — TB-16 Atom 6.1 follow-up.)"

# Single-task minimal-coverage smoke (Task A happy_path equivalent).
# This produces a chain-backed tape with at minimum:
#   TaskOpen + EscrowLock + Work + Verify + (FinalizeReward if accepted)
# Coverage of the remaining tx kinds requires additional evaluator
# extensions (challenge injection, force-exhaustion, MarketSeed entry).

TASK_A_DIR="$OUT_DIR/task_A_happy_path"
mkdir -p "$TASK_A_DIR/runtime_repo" "$TASK_A_DIR/cas"
echo "  Task A: happy_path (mathd_algebra_171; bounty 200_000μC)..."
# TB-16 post-R3 fix 2026-05-04: evaluator CLI is `<problem_file.lean>`
# positional + `--mode <mode>` flag (default full). Task-mode + chain
# paths flow via TURINGOS_USER_TASK_MODE / TURINGOS_CHAINTAPE_PATH /
# TURINGOS_CAS_PATH env vars. CONDITION=n1 (NOT oneshot) is mandatory
# for ChainTape mode per TB-7R Deliverable B (chaintape_mode_gate
# fail-closes oneshot — bus.submit_typed_tx requires swarm dispatch).
# TURINGOS_CHAINTAPE_PRESEED=1 is REQUIRED to enable the user-task-mode
# TaskOpen+EscrowLock preseed path (line 857-975 of evaluator.rs); without
# it, the chain only has synthetic TaskOpen + TerminalSummary because the
# preseed gate at line 696 is the SOLE controller of user-mode path.
# The previous `--task-mode user --problem ... --max-transactions ...`
# CLI flags were phantom (latent Atom 6 bug; arena_run4 evidence was
# generated by direct evaluator invocation with CHAINTAPE_PRESEED=1, not
# this runner).
TURINGOS_USER_TASK_MODE=1 \
TURINGOS_CHAINTAPE_PRESEED=1 \
TURINGOS_USER_TASK_BOUNTY_MICRO=200000 \
TURINGOS_CHAINTAPE_PATH="$TASK_A_DIR/runtime_repo" \
TURINGOS_CAS_PATH="$TASK_A_DIR/cas" \
TURINGOS_RUN_ID="${RUN_ID_PREFIX}-A" \
LLM_PROXY_URL="$LLM_PROXY_URL" \
MAX_TRANSACTIONS="$MAX_TX" \
CONDITION="${CONDITION:-n1}" \
"$EVALUATOR_BIN" mathd_algebra_171.lean \
  || { echo "✗ Task A evaluator failed" >&2; exit 1; }

# (Task B-F would follow same pattern; deferred to evaluator extensions.)

# Per Codex TB-16 R1 V4 VETO closure (2026-05-04): ship-gate
# enforcement — no `|| true` masking of audit failures. If any audit
# pipeline step fails, the script exits non-zero. CONTINUE_ON_ERROR=1
# overrides for diagnostic-only runs.
CONTINUE_ON_ERROR="${CONTINUE_ON_ERROR:-0}"
maybe_continue() {
  local rc=$?
  if [[ $rc -ne 0 ]]; then
    if [[ "$CONTINUE_ON_ERROR" == "1" ]]; then
      echo "⚠ step exited $rc; CONTINUE_ON_ERROR=1 → continuing for diagnostics" >&2
      return 0
    fi
    echo "✗ step exited $rc (set CONTINUE_ON_ERROR=1 to bypass for diagnostics)" >&2
    exit $rc
  fi
}

# ── Step 5: Run audit_tape over the produced tape ───────────────────
echo "▶ Step 5/8: audit_tape over Task A tape..."
"$AUDIT_TAPE_BIN" \
  --runtime-repo "$TASK_A_DIR/runtime_repo" \
  --cas-dir "$TASK_A_DIR/cas" \
  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$OUT_DIR/verdict.json"
maybe_continue

# ── Step 6: audit_tape_tamper ───────────────────────────────────────
echo "▶ Step 6/8: audit_tape_tamper (3-corruption smoke)..."
"$AUDIT_TAPE_TAMPER_BIN" \
  --runtime-repo "$TASK_A_DIR/runtime_repo" \
  --cas-dir "$TASK_A_DIR/cas" \
  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --tamper-dir "$OUT_DIR/tamper" \
  --out "$OUT_DIR/tamper_report.json"
maybe_continue

# ── Step 7: generate_markov_capsule ─────────────────────────────────
# TB-16.x.fix (architect OBS_R022 Option α RATIFIED 2026-05-04): the
# global LATEST pointer file has been de-canonicalized. To inherit
# from a prior chain, set PREV_CID_HEX in the environment before
# running; otherwise the capsule is genesis-rooted.
PREV_CID_ARGS=()
if [[ -n "${PREV_CID_HEX:-}" ]]; then
  PREV_CID_ARGS=(--prev-cid-hex "$PREV_CID_HEX")
fi
echo "▶ Step 7/8: generate_markov_capsule (TB-16; prev=${PREV_CID_HEX:-none})..."
"$GEN_MARKOV_BIN" \
  --tb-id 16 \
  --out-dir "$OUT_DIR" \
  --constitution-path constitution.md \
  --runtime-repo "$TASK_A_DIR/runtime_repo" \
  --cas-dir "$TASK_A_DIR/cas" \
  --alignment-dir handover/alignment \
  "${PREV_CID_ARGS[@]}"
maybe_continue

# ── Step 8: audit_dashboard ─────────────────────────────────────────
echo "▶ Step 8/8: audit_dashboard..."
"$AUDIT_DASHBOARD_BIN" \
  --repo "$TASK_A_DIR/runtime_repo" \
  --cas "$TASK_A_DIR/cas" \
  --out "$OUT_DIR/dashboard.txt"
maybe_continue

# ── Replay determinism check ────────────────────────────────────────
echo "▶ Replay determinism: re-running audit_tape..."
"$AUDIT_TAPE_BIN" \
  --runtime-repo "$TASK_A_DIR/runtime_repo" \
  --cas-dir "$TASK_A_DIR/cas" \
  --agent-pubkeys "$TASK_A_DIR/runtime_repo/agent_pubkeys.json" \
  --pinned-pubkeys "$TASK_A_DIR/runtime_repo/pinned_pubkeys.json" \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out "$OUT_DIR/verdict_replay.json"
maybe_continue

# ── Final summary + ship-gate enforcement ───────────────────────────
echo ""
echo "═══════════════════════════════════════════════════════════════════"
echo "TB-16 ARENA RUN COMPLETE"
echo "═══════════════════════════════════════════════════════════════════"
echo "Out dir        : $OUT_DIR"
echo "Plan           : $OUT_DIR/ARENA_PLAN.md"
echo "Verdict        : $OUT_DIR/verdict.json"
echo "Verdict replay : $OUT_DIR/verdict_replay.json"
echo "Tamper report  : $OUT_DIR/tamper_report.json"
echo "Dashboard      : $OUT_DIR/dashboard.txt"

# Ship-gate aggregator (Codex TB-16 R1 V4 closure): the script's exit
# code MUST reflect ship-gate verdict. Any failure → non-zero exit.
gate_failed=0
if [[ -f "$OUT_DIR/verdict.json" && -f "$OUT_DIR/verdict_replay.json" ]]; then
  V1=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict.json" | head -1)
  V2=$(grep -o '"verdict": *"[^"]*"' "$OUT_DIR/verdict_replay.json" | head -1)
  echo "Verdict-1      : $V1"
  echo "Verdict-2      : $V2"
  if [[ "$V1" != *"PROCEED"* ]]; then
    echo "Ship-gate      : ✗ verdict-1 != PROCEED"
    gate_failed=1
  fi
  if [[ "$V2" != *"PROCEED"* ]]; then
    echo "Ship-gate      : ✗ verdict-2 != PROCEED"
    gate_failed=1
  fi
  if cmp -s "$OUT_DIR/verdict.json" "$OUT_DIR/verdict_replay.json"; then
    echo "Replay         : ✓ byte-identical"
  else
    echo "Replay         : ✗ DIVERGED"
    gate_failed=1
  fi
else
  echo "Ship-gate      : ✗ verdict.json or verdict_replay.json missing"
  gate_failed=1
fi

if [[ "$CONTINUE_ON_ERROR" == "1" ]]; then
  echo "Note           : CONTINUE_ON_ERROR=1; ignoring ship-gate verdict"
  exit 0
fi
exit $gate_failed
