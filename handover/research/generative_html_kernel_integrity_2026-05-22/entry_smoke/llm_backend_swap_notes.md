# LLM Backend Swap — DeepSeek 直连 (Phase 1 verification notes)

**结论**：**零 src/ 代码改动**。SiliconFlow client 设计已显式支持端点替换 + thinking flag + 自定义模型 ID。仅需在 web 启动前手动 `turingos llm config` 写入 DeepSeek 模型，让 W7 web onboarding step 2 自动跳过。

## 验证证据

### 1. Endpoint override 已支持

[src/bin/turingos/siliconflow_client.rs:34-37](../../../src/bin/turingos/siliconflow_client.rs#L34-L37):
```rust
pub(crate) fn endpoint() -> String {
    std::env::var("TURINGOS_SILICONFLOW_ENDPOINT")
        .unwrap_or_else(|_| SILICONFLOW_ENDPOINT.to_string())
}
```
通过 `TURINGOS_SILICONFLOW_ENDPOINT=https://api.deepseek.com/v1/chat/completions` 覆盖。

### 2. Thinking mode 已透传

[src/bin/turingos/siliconflow_client.rs:45-50, 87-101](../../../src/bin/turingos/siliconflow_client.rs#L45-L101):
- `ThinkingConfig { kind: "enabled" }` 序列化为 `{"thinking":{"type":"enabled"}}`
- `ChatRequest.thinking: Option<ThinkingConfig>` 透传给 chat_complete
- `read_meta_thinking` / `read_blackbox_thinking` 从 turingos.toml `llm.meta.thinking = "on"` 读取

### 3. 模型 ID 完全可配置

[src/bin/turingos/cmd_llm.rs:143-151](../../../src/bin/turingos/cmd_llm.rs#L143-L151) 内置 DeepSeek dual-key 完整范例：
```
turingos llm config --workspace <PATH> \
    --meta-api-key-env DEEPSEEK_API_KEY \
    --blackbox-api-key-env DEEPSEEK_API_KEY_WORKER \
    --meta-model deepseek-v4-pro \
    --blackbox-model deepseek-v4-flash \
    --meta-thinking on \
    --blackbox-thinking off
```
**注意**：FULL_HELP 列了 `--meta-api-key-env` 与 `--blackbox-api-key-env`，但 [`run_inner`](../../../src/bin/turingos/cmd_llm.rs#L284-L351) 实际只解析单一 `--api-key-env`。文档与实现存在轻微 drift。对本次运行影响不大——meta/blackbox 都用同一个 `SILICONFLOW_API_KEY` env var 名即可（值放 DeepSeek key）。

### 4. B1 UX 助手已为 DeepSeek 直连写好错误指引

[src/bin/turingos/siliconflow_client.rs:192-213](../../../src/bin/turingos/siliconflow_client.rs#L192-L213) `maybe_rewrite_deepseek_model_error` 在错误体含 `deepseek-v4-` 时主动提示用户用 `deepseek-v4-pro` / `deepseek-v4-flash`，说明这两个模型 ID 是项目研究后的官方推荐。

### 5. W7 web onboarding 行为

[src/web/welcome.rs:510-573](../../../src/web/welcome.rs#L510-L573) `welcome_llm_config_handler` 调用 `turingos llm config --workspace <ws>` 时**不传任何 model/endpoint flag**，因此会写 Phase 6.3 默认 (SiliconFlow + DeepSeek-V3.2 + Qwen3-Coder-30B)。

**绕过方案**：手动在 web 启动前预先 `turingos llm config` 写好 DeepSeek 配置。然后 [welcome.rs:527](../../../src/web/welcome.rs#L527) 的 `if pre.llm_configured` 检查（基于 `inspect_workspace` 检测 `llm.meta.model` + `llm.blackbox.model` 是否存在）会让 W7 step 2 自动跳过。

### 6. API key 流转

[src/web/welcome.rs:15-28](../../../src/web/welcome.rs#L15-L28) docstring：
- W7 把用户在 `/welcome` step 3 输入的 key 存到 `AppState.api_key` (in-memory)
- 通过 `Command::env("SILICONFLOW_API_KEY", value)` 注入到 spec/generate 子进程
- 永不持久化到 disk，永不 echo，永不 log

**含义**：env var 名硬编码为 `SILICONFLOW_API_KEY`（welcome 路径）。即使 turingos.toml 写 `api_key_env = "DEEPSEEK_API_KEY"`，web spec/generate 子进程拿到的环境变量名仍是 `SILICONFLOW_API_KEY`。

**规避**：把 turingos.toml 里 `llm.meta.api_key_env` 与 `llm.blackbox.api_key_env` 都保持默认值 `SILICONFLOW_API_KEY`，DeepSeek key 值放进这个环境变量名即可。env var 名是历史命名，不要纠结。

## 执行配方

```bash
# 1. 准备工作区
mkdir -p tmp/generative_html_probe_20260522
./target/debug/turingos init \
    --project tmp/generative_html_probe_20260522 \
    --template multi-agent

# 2. 配置 DeepSeek 直连模型
./target/debug/turingos llm config \
    --workspace tmp/generative_html_probe_20260522 \
    --meta-model deepseek-v4-pro \
    --blackbox-model deepseek-v4-flash \
    --meta-thinking on \
    --blackbox-thinking off

# 3. 启动 web (endpoint + key 通过环境变量)
export TURINGOS_SILICONFLOW_ENDPOINT=https://api.deepseek.com/v1/chat/completions
export SILICONFLOW_API_KEY=sk-<deepseek-key-here>
export TURINGOS_WEB_WORKSPACE=$(pwd)/tmp/generative_html_probe_20260522
./target/debug/turingos_web
```

启动后：
- W7 step 1 (init): 跳过（init_done）
- W7 step 2 (llm-config): 跳过（llm_configured=true，已含 deepseek-v4-pro/flash）
- W7 step 3 (api-key): 用户在浏览器 `/welcome` 输入 DeepSeek key（须 sk- 前缀，DeepSeek 自然符合）
- W7 step 4 (agent-deploy): 一键创建 agent_001 Solver
- → `/build` 显示 8 题 `<tos-spec-grill>`

## 风险与 fallback

| 风险 | 检测 | Fallback |
|---|---|---|
| `deepseek-v4-pro` / `deepseek-v4-flash` 不是 DeepSeek API 当前在线的 ID | spec 调用返回 4xx with `supported API model names` | 切到 `deepseek-chat` (V3) + `deepseek-reasoner` (R1) 公开 ID |
| DeepSeek 直连 API 不接受 `thinking:{"type":"enabled"}` 字段（OpenAI-compat 不完整） | 4xx with thinking-field-rejected 字样 | 改用 `--meta-thinking off`；reasoning 通过 `deepseek-reasoner` model ID 自动启用 |
| W7 step 3 不接受 DeepSeek key（前缀校验失败） | 浏览器报 invalid_input | 验证 [welcome.rs:306](../../../src/web/welcome.rs#L306) 要求 `sk-` 前缀，DeepSeek key 满足 |

## 验证后状态

- ✅ Phase 1 配置路径：零代码改动
- ⏳ Phase 2 待用户提供 DEEPSEEK_API_KEY 值（请在 shell 中 `export SILICONFLOW_API_KEY=sk-...`）
- ⏳ Phase 2 待 cargo build 完成（运行中）
