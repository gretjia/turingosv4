#!/usr/bin/env bash
SUBCMD="${1:-}"
REAL_BIN="/Users/zephryj/work/turingosv4/target/debug/turingos"
ARGV_LOG="/tmp/stub_argv.log"
{
  echo "--- $(date -u +%FT%TZ) $$"
  printf "argv[%d]: %s\n" 0 "$REAL_BIN"
  i=1
  for a in "$@"; do printf "argv[%d]: %s\n" "$i" "$a"; i=$((i+1)); done
  echo "env SILICONFLOW_API_KEY_set=$([ -n "${SILICONFLOW_API_KEY:-}" ] && echo yes || echo no)"
} >> "$ARGV_LOG"

case "$SUBCMD" in
  spec)
    # Parse --workspace
    WORKSPACE=""
    for ((i=1; i<=$#; i++)); do
      if [[ "${!i}" == "--workspace" ]]; then n=$((i+1)); WORKSPACE="${!n}"; break; fi
    done
    [[ -z "$WORKSPACE" ]] && { echo "stub: spec missing --workspace" >&2; exit 1; }
    mkdir -p "$WORKSPACE/cas"
    cat > "$WORKSPACE/spec.md" <<'MDEOF'
# 用户工具需求 (stub spec produced by passthrough_stub.sh)

## 故事 (Q1)
一个简单的桌面工具，帮我跟踪每天的运动。

## 锚点 (Q2)
类似 Strava 但更简单。

## 数据模型 (Q3)
每条记录：日期、运动类型、持续时间。

## 第一次点击 (Q4)
打开 → 看到日历 → 点今天添加一条。

## 怪用户 (Q5)
乱填的话给友善提示，不阻止录入。

## 失望边界 (Q6)
社交功能会让我失望。

## 成功标准 (Q7)
连续 30 天每周累计 ≥ 5 条记录。

## 复述 (Q8)
跟踪日常运动，简单日历界面，本地记录，不要社交。
MDEOF
    cat > "$WORKSPACE/spec_transcript.jsonl" <<'JSONLEOF'
{"role":"user","q":1,"a":"stub answer 1"}
{"role":"user","q":2,"a":"stub answer 2"}
JSONLEOF
    # Write a CAS-shaped capsule file so welcome.rs::find_first_cas_cid picks it up.
    CID=$(printf '%064x' "$(date +%s)")
    printf '{"stub":true,"ts":"%s"}\n' "$(date -u +%FT%TZ)" > "$WORKSPACE/cas/$CID.json"
    echo "[stub] spec.md written to $WORKSPACE/spec.md"
    # spec.rs::parse_capsule_cid_from_stdout looks for "CAS capsule CID    -> <hex>".
    echo "  CAS capsule CID    -> $CID"
    exit 0
    ;;
  generate)
    WORKSPACE=""
    for ((i=1; i<=$#; i++)); do
      if [[ "${!i}" == "--workspace" ]]; then n=$((i+1)); WORKSPACE="${!n}"; break; fi
    done
    [[ -z "$WORKSPACE" ]] && { echo "stub: generate missing --workspace" >&2; exit 1; }
    mkdir -p "$WORKSPACE/artifacts"
    cat > "$WORKSPACE/artifacts/index.html" <<'HTMLEOF'
<!doctype html>
<html lang="zh">
<head>
<meta charset="utf-8">
<title>Stub Generated UI</title>
<style>
  body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; padding: 32px; max-width: 600px; margin: 0 auto; line-height: 1.6; }
  h1 { font-size: 24px; color: #1F6E6B; }
  .marker { padding: 16px; border: 1px solid #E5E3DC; border-radius: 2px; margin: 24px 0; }
  small { color: #666; }
</style>
</head>
<body>
<h1>TuringOS Phase 7 — Stub Generated UI</h1>
<p data-testid="stub-marker">If you see this inside the iframe, the artifact-viewer mounted correctly and the sandbox="allow-scripts" lets this small JS-driven timestamp render below.</p>
<div class="marker">
  <p>Generated at <span id="ts"></span></p>
  <small>This is a stub. The architect's real run will produce real Qwen3-Coder-30B output.</small>
</div>
<script>document.getElementById('ts').textContent = new Date().toISOString();</script>
</body>
</html>
HTMLEOF
    echo "[stub] artifacts/index.html written ($(wc -c < "$WORKSPACE/artifacts/index.html") bytes)"
    exit 0
    ;;
  *)
    # Passthrough — let the real turingos handle init / llm / agent / etc.
    exec "$REAL_BIN" "$@"
    ;;
esac
