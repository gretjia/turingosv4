# VIA NEGATIVA — 已证伪路径

已知的失败路径，勿重蹈覆辙。

## 1. Greedy Router (ArgMax)

无法从死胡同回退，已替换为 Boltzmann Softmax。

## 2. 固定 500 币税

通货紧缩死锁，已替换为自由浮动质押。

## 3. kernel.rs 硬编码 "[OMEGA]"

违反零领域知识原则，已移除。

## 4. SiliconFlow API at N=30

HTTP 401/429 崩溃，已迁移 Volcengine。

## 5. Gemini CLI Agent

Node.js OOM，已迁移 Claude Code。

## 6. reqwest+rustls 直连中国 HTTPS (V-007)

macOS 上 reqwest+rustls 调用 DashScope/Aliyun HTTPS 端点永久挂起。尝试 6 种方案均失败:
http1_only / no_proxy / native-tls / curl subprocess (64KB pipe deadlock) / tokio::process / spawn_blocking。
根因: macOS 64KB pipe buffer + 中国 HTTPS 端点 TLS 协商异常。
已替换为 Python OpenAI SDK 本地 HTTP 代理 (llm_proxy.py)。教训: **永远不要让 Rust 直连云端 HTTPS，用本地代理解耦。**

## 7. 单线程 HTTP Server + 并发 Agent (V-008)

Python `HTTPServer` 默认单线程，15 agent 并发请求时 13 个返回 502。
单请求 curl 测试通过 ≠ 生产可用。已修复为 `ThreadingMixIn`。
教训: **任何 HTTP 服务必须 ThreadingMixIn/async，单线程永远不可接受。**

## 8. LLM 输出格式契约脆性 (V-009)

`parse_agent_output()` 静默返回 `None`，Agent 看似活跃实则空转。三层解析失败:
(1) JSON 前缀 `append: {` (2) LaTeX 反斜杠 `\cdot` 是非法 JSON 转义 (3) 裸标签 `<action>append</action>` 无 JSON。
所有三层均静默失败，无日志，无错误。教训: **LLM 输出是概率信号，不是确定性 API — 解析器必须 Postel 法则 (宽进严出) + 永不静默失败。**
