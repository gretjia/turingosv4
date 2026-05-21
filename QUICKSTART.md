# TuringOS v4 — Quick Start (人工测试指南)

> 5 分钟内从零到交付一个浏览器小游戏。所有命令复制粘贴即可。

## 你需要什么

- Linux / macOS shell (bash 或 zsh)
- 网络能连 `api.deepseek.com`
- `.env` 文件里已有的两把 DeepSeek key (Plan v4 已经放好)

## 二进制位置

```bash
TURINGOS=/home/zephryj/projects/turingosv4/target/debug/turingos
```

或者直接用绝对路径调用：`/home/zephryj/projects/turingosv4/target/debug/turingos`

如果 `target/debug/turingos` 不存在，先编译：
```bash
cd /home/zephryj/projects/turingosv4 && cargo build --bin turingos
```

---

## 完整使用流程（5 步）

### Step 0: 加载 API key + 设 endpoint

每次新开 shell 都要做一次（这是 Plan v4 next charter 要修的 setup friction）：

```bash
# 1. 加载 .env 里的 DeepSeek 双 key
set -a
source /home/zephryj/projects/turingosv4/.env
set +a

# 2. 告诉 turingos 用 DeepSeek 而不是 SiliconFlow
export TURINGOS_SILICONFLOW_ENDPOINT="https://api.deepseek.com/v1/chat/completions"

# 3. （可选）准备 workspace 路径变量
export WS=/tmp/my-turingos-test-$$
mkdir -p $WS
```

### Step 1: 看 welcome（了解你在哪一步）

```bash
$TURINGOS welcome --workspace $WS
```

输出会告诉你下一步该跑什么命令。**任何时候卡住，跑 welcome 看进度**。

### Step 2: 初始化 workspace + 配 DeepSeek 双 key

```bash
$TURINGOS init --project $WS --provider deepseek
```

它会：
- 在 `$WS` 里建几个目录（`cas/`, `runtime_repo/`）
- 写 `turingos.toml`，把 Meta 角色配成 `deepseek-v4-pro`（reasoning on），Worker 角色配成 `deepseek-v4-flash`（reasoning off），分别用 `DEEPSEEK_API_KEY` 和 `DEEPSEEK_API_KEY_WORKER` 两把 key
- 印出"Next steps"提示你 export 3 个变量（你 Step 0 已经做了，不用重复）

**Tip**: 再跑一次 `$TURINGOS welcome --workspace $WS` —— 应该看到 `⚠ TURINGOS_SILICONFLOW_ENDPOINT overridden to: https://api.deepseek.com/v1/chat/completions`。看到这行就说明 endpoint 配对了。

### Step 3: 写需求（"我想做什么游戏"）

最快方式：把 8 个答案写进一个 JSON 文件：

```bash
cat > $WS/my_answers.json << 'EOF'
[
  "I want to make a small browser game where users click colored squares to match a pattern. Single HTML file, no backend.",
  "Similar to Lights Out puzzle game from old Flash days.",
  "Best score saved in localStorage, persists across sessions.",
  "Open index.html in browser, see a 4x4 grid, click squares to toggle, try to match the target pattern shown above.",
  "If user clicks too fast (multi-click bursts), debounce to 100ms. Touch events work on mobile.",
  "No multiplayer, no leaderboard, no images, no sounds.",
  "30 days from now: 20 people opened the page and played at least one full puzzle.",
  "OK, you understand: a 4x4 toggle puzzle game in one HTML file with localStorage score."
]
EOF
```

然后跑 spec：

```bash
$TURINGOS spec --workspace $WS --answers-file $WS/my_answers.json --lang en --mode static
```

它会调 Meta LLM（`deepseek-v4-pro`，带 thinking）把 8 个答案合成成结构化的 `spec.md`。这一步大概花 10-30 秒 + ~3000 tokens。

**如果你想交互式回答**：去掉 `--answers-file`，直接跑 `$TURINGOS spec --workspace $WS --lang en`。它会一题一题问你。

**如果你不想烧 LLM token 在 spec 阶段**：加 `--skip-llm`，它会用模板拼出来 spec.md（功能等价，少了"叙述润色"）。

### Step 4: 生成游戏

```bash
$TURINGOS generate --workspace $WS
```

它会：
- 调 Worker LLM（`deepseek-v4-flash`，no thinking，更快）按你的 spec 写 HTML
- 把生成的文件放到 `$WS/artifacts/`
- 跑内部测试（EntrypointExists + HtmlParses + 可选的 SandboxPolicyPreserved）
- 印 `Internal tests: PASS (N/N scenarios) — names` 一行你能看懂的结果

最后会告诉你怎么打开：
```
xdg-open ./artifacts/index.html
```

如果你用 macOS，用 `open` 替代 `xdg-open`。如果你用 Windows，双击文件即可。

### Step 5: 打开看游戏

```bash
# Linux
xdg-open $WS/artifacts/index.html

# macOS
open $WS/artifacts/index.html

# 或直接拷路径到浏览器
echo "file://$WS/artifacts/index.html"
```

---

## 失败时怎么办

### "Blackbox LLM emitted no parseable files"

DeepSeek API 偶尔返回空 body（瞬态问题，不是你的错）。**直接重跑 generate**：

```bash
$TURINGOS generate --workspace $WS
```

R4-1 修复后这条错误后面会自动印一行：`(Transient API error? Try running 'turingos generate' again.)` —— 看到这行就重跑。

### "HTTP 401 Authentication Fails"

key 错了。检查：
```bash
echo "Meta key  : ${DEEPSEEK_API_KEY:0:10}..."
echo "Worker key: ${DEEPSEEK_API_KEY_WORKER:0:10}..."
```
应该都是 `sk-` 开头。如果是空的，回到 Step 0 重 source `.env`。

### "llm.blackbox.api_key_env is not set in turingos.toml"

`turingos.toml` 没写好。重跑 `init` 加 `--force`：
```bash
$TURINGOS init --project $WS --provider deepseek --force
```

### welcome 显示卡在某一步

跑 `welcome --workspace $WS` 它会告诉你下一步该跑什么。

---

## 进阶用法

### 不用 DeepSeek，用 SiliconFlow

```bash
$TURINGOS init --project $WS  # 不加 --provider，默认 siliconflow
unset TURINGOS_SILICONFLOW_ENDPOINT  # 用默认 endpoint
export SILICONFLOW_API_KEY="sk-..."
```

### 不用 DeepSeek，用 OpenRouter

```bash
$TURINGOS llm config --workspace $WS \
    --meta-api-key-env OPENROUTER_API_KEY \
    --blackbox-api-key-env OPENROUTER_API_KEY \
    --meta-model anthropic/claude-opus-4-1 \
    --blackbox-model openai/gpt-4o-mini

export OPENROUTER_API_KEY="sk-or-..."
export TURINGOS_SILICONFLOW_ENDPOINT="https://openrouter.ai/api/v1/chat/completions"
```

更多例子：`$TURINGOS llm config --help` 看 ANTHROPIC / OPENAI 块。

### 看 capsule 链（开发者）

```bash
ls -la $WS/cas/
cat $WS/cas/.turingos_cas_index.jsonl | head
```

`.jsonl` 每行一个 CID，按时间顺序。schema_id 包含：
- `turingos-spec-capsule-v1` (你的 spec)
- `turingos-generation-attempt-v1` (每次 LLM call)
- `turingos-artifact-bundle-v1` (生成的文件清单)
- `turingos-test-run-v1` (内部测试结果)
- `turingos-generate-rejection-v1` (失败的 attempt)

### 离线 replay（开发者）

```bash
$TURINGOS replay --offline --workspace $WS --session <session_id>
```

完全离线重建你的 build session，不调 LLM。

### 看所有子命令

```bash
$TURINGOS --help
```

26 个子命令，包括 `agent deploy` / `task open/view/tick` / `batch` / `audit dashboard` / `verify chaintape` / `report run` / `render` / `replay` / `export evidence` 等。

---

## 一键完整测试脚本

```bash
#!/bin/bash
set -e

# Step 0
set -a; source /home/zephryj/projects/turingosv4/.env; set +a
export TURINGOS_SILICONFLOW_ENDPOINT="https://api.deepseek.com/v1/chat/completions"
TURINGOS=/home/zephryj/projects/turingosv4/target/debug/turingos
WS=/tmp/turingos-manual-$$
mkdir -p $WS
echo "Workspace: $WS"

# Step 1-2
$TURINGOS welcome --workspace $WS
$TURINGOS init --project $WS --provider deepseek
$TURINGOS welcome --workspace $WS

# Step 3: 自己写 my_answers.json 然后...
cat > $WS/my_answers.json << 'EOF'
["A coin-collecting platformer where a tiny pixel character jumps between platforms.","Like a 2-minute version of old Mario Bros.","High score in localStorage.","Open page, press arrow keys to move and space to jump, collect 10 coins to win.","If user holds arrow keys, character should not glitch through platforms.","No enemies, no power-ups, no levels — just one screen.","After 30 days: 30 people played one full round.","OK, single-screen pixel platformer with 10 coins to collect."]
EOF

$TURINGOS spec --workspace $WS --answers-file $WS/my_answers.json --lang en --mode static
$TURINGOS generate --workspace $WS

# Step 5
echo ""
echo "Game delivered. Open in browser:"
echo "  xdg-open $WS/artifacts/index.html"
echo "  (macOS: open $WS/artifacts/index.html)"
echo ""
$TURINGOS welcome --workspace $WS  # 看最终状态
```

---

## 一句话总结

```
welcome → init --provider deepseek → 写 answers.json → spec → generate → 开 index.html
```

5 步，~30 秒命令时间 + ~30 秒 LLM 处理时间 = 1 分钟从零到游戏。

## 哪些已知 quirks

1. **DeepSeek API 偶尔返回空 body**（约 10-20% 概率）→ 重跑 generate 即可，R4-1 retry hint 会提醒你
2. **每开新 shell 要重 export 3 个变量** → 解决方案是 next charter 的 `turingos llm config --interactive` wizard
3. **`HtmlParses` 测试可能 false-PASS 在截断的 HTML 上**（R5-1，next charter 修）→ 拿到游戏后建议自己在浏览器里点几下确认 JS 没断
4. **`spec audit --session <X>` 需要 session ID 但没给提示**（NB5, next charter 修）→ 暂时不用这个子命令
5. **`xdg-open` 是 Linux 命令**（NB4, next charter 修）→ macOS 用 `open`，Windows 双击文件

## 关于 setup friction（最大遗留问题）

每次开新 shell 你都得重复 Step 0 的 3 个 export。临时绕开：把这 4 行加进你的 `~/.bashrc` 或 `~/.zshrc`：

```bash
# TuringOS v4 quick setup
export TURINGOS=/home/zephryj/projects/turingosv4/target/debug/turingos
alias load-turingos='set -a; source /home/zephryj/projects/turingosv4/.env; set +a; export TURINGOS_SILICONFLOW_ENDPOINT="https://api.deepseek.com/v1/chat/completions"'
```

然后每次只要 `load-turingos` 就齐活。

---

**Plan v4 SHIPPED on 2026-05-21. 5 轮 user-sim 全过，18 bugs 找到 17 修了。** 详见 `handover/architect-insights/PLAN_V4_SHIPPED_2026-05-21.md`。
