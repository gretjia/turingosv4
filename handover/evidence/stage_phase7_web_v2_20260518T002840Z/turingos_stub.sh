#!/usr/bin/env bash
# Records argv to $EVID_ABS/stub_argv.txt; emits spec.md or artifacts/ as appropriate; exits 0.
{
  echo "--- invocation $(date -u +%FT%TZ) ---"
  printf "%s\n" "$@"
} >> "$EVID_ABS/stub_argv.txt"

SUBCMD="${1:-}"

if [[ "$SUBCMD" == "spec" ]]; then
  # parse --workspace <PATH> argument
  WORKSPACE=""
  for ((i=1; i<=$#; i++)); do
    if [[ "${!i}" == "--workspace" ]]; then
      next=$((i+1))
      WORKSPACE="${!next}"
      break
    fi
  done
  if [[ -z "$WORKSPACE" ]]; then
    echo "stub: missing --workspace" >&2
    exit 1
  fi
  mkdir -p "$WORKSPACE/cas"
  cat > "$WORKSPACE/spec.md" <<'MDEOF'
# 用户工具需求 (verifier stub spec)

## 故事
一个用于追踪每日运动的小工具。

## 数据模型
- 每条记录: 日期 + 运动类型 + 时长

## 首次使用
打开后看到日历，点今天添加一条。

## 成功标准
连续使用 30 天，每周累计 ≥ 5 条记录。
MDEOF
  echo "spec.md written to $WORKSPACE/spec.md"
  echo "CID: stub_cid_abc123def456"
  echo "transcript: $WORKSPACE/spec_transcript.jsonl"
  exit 0
fi

if [[ "$SUBCMD" == "generate" ]]; then
  WORKSPACE=""
  for ((i=1; i<=$#; i++)); do
    if [[ "${!i}" == "--workspace" ]]; then
      next=$((i+1))
      WORKSPACE="${!next}"
      break
    fi
  done
  if [[ -z "$WORKSPACE" ]]; then
    echo "stub: missing --workspace" >&2
    exit 1
  fi
  mkdir -p "$WORKSPACE/artifacts"
  cat > "$WORKSPACE/artifacts/index.html" <<'HTMLEOF'
<!doctype html>
<html lang="zh">
<head>
  <meta charset="utf-8" />
  <title>Verifier Stub — Generated UI</title>
  <style>
    body { font-family: system-ui, sans-serif; padding: 32px; }
    h1 { font-size: 24px; }
    p { color: #555; }
  </style>
</head>
<body>
  <h1>TuringOS Phase 7 — Generated UI (verifier stub)</h1>
  <p data-testid="stub-marker">If you can read this inside the iframe, the artifact-viewer mounted correctly with sandbox="allow-scripts".</p>
  <p>Generated at <span id="ts"></span>.</p>
  <script>document.getElementById('ts').textContent = new Date().toISOString();</script>
</body>
</html>
HTMLEOF
  echo "Generated $WORKSPACE/artifacts/index.html"
  exit 0
fi

echo "stub: unknown subcommand '$SUBCMD'" >&2
exit 0
