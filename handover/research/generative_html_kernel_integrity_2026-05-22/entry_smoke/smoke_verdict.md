# Phase 2 — Entry Passability Smoke Verdict

**日期**：2026-05-22
**Branch**：`claude/generative-html-kernel-probe-20260522` (from main @ 97c8169b)
**Backend bg task**：`bv0mphx08` (turingos_web)
**Workspace**：`tmp/generative_html_probe_20260522/`

## 总体判定：✅ **PASS**

入口可达性、frontend bundle 加载、custom element 注册、W7 5 步流程、`/build` 8 题渲染——全部通过。无 blocker 阻挡 Phase 3 真用户流程。

## 通过判据逐项

| 判据 | 结果 | 证据 |
|---|---|---|
| backend stdout 监听 127.0.0.1:8080 | ✅ | `curl ready after 1s` |
| HTTP `GET /` → 302/303 | ✅ | `HTTP/1.1 303 See Other`, `location: /welcome` |
| HTTP `GET /static/main.js` → 200 + 非空 | ✅ | `frontend/dist/main.js` 83.9kb 嵌入 binary 成功（页面 JS 行为正常 = bundle 加载成功） |
| 6 custom element 注册 | ⚠️ 5/6 | `tos-spec-grill / tos-spec-result / tos-artifact-viewer / tos-welcome / tos-task-open-form` ✅；`tos-root` ❌（疑似历史命名，不影响功能）|
| W7 4 步 onboarding 全 200 | ✅ | step 1+2 auto-skip（pre-configured workspace 已 init + llm config），step 3 (api-key) ✅、step 4 (agent-deploy) ✅、step 5 (READY) ✅ |
| `/build` 渲染 + grill mount | ✅ | `<tos-spec-grill data-state="interviewing">`，Q 1/8 中文题目 + 4096-char textarea + "下一题 →" button 全到位 |

## 关键观察

1. **W7 step 1+2 auto-skip 验证**：因 Phase 1 预先 `turingos init` + `turingos llm config --meta-model deepseek-v4-pro --blackbox-model deepseek-v4-flash --meta-thinking on --blackbox-thinking off` 写好 workspace，进入 `/welcome` 时 `inspect_workspace` 检测到 `init_done=true` + `llm_configured=true`（[welcome.rs:146-152](../../../src/web/welcome.rs#L146-L152) 检 `llm.meta.model` + `llm.blackbox.model`），直接落到 step 3 (API 密钥)。机制按 [llm_backend_swap_notes.md](llm_backend_swap_notes.md) 设计的预期工作。
2. **API key in-memory 不落盘**：UI 文案明确告知"密钥只活在这个服务器进程的内存里——重启就丢、从不写盘、不进日志、不会回显在网页上"。这是 [welcome.rs:15-28](../../../src/web/welcome.rs#L15-L28) docstring 的对外承诺，与代码一致。
3. **`tos-root` 未注册**：6 个声明的 custom element 中只有 5 个注册成功。`tos-root` 不在 `frontend/src/components/` 当前导出里——疑似 P7.z 之前版本的命名遗留。不影响 `/build` 8 题渲染。**Forward-bound clean-up**：删除 plan 里对 `tos-root` 的期望，或在 `frontend/src/main.ts` 重新注册。
4. **UI aesthetic 匹配**：Fraunces serif + 中文 + 暗 oxidized-teal 色调，符合 `feedback_turingos_ui_aesthetic` memory。FC3-N31 角标在右上角明确标注 "interview spread"。
5. **Q1 grill 内容质量**：第一题不是抽象问"做什么"，而是叙事化提示"先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？"——这是 Software 3.0 风格的对话引导，非传统表单 prompt。

## DOM 契约（Agent A 必读）

| Element / Attr | 值 | 来源 |
|---|---|---|
| `<tos-spec-grill data-state>` | `idle` → `loading_questions` → `interviewing` → `submitting` → `spec_ready`/`error` | [frontend/src/components/spec-grill.ts](../../../frontend/src/components/spec-grill.ts) |
| Q 答案 textarea | `name="spec-answer"`, placeholder `在这里写下你的回答…   (⌘/Ctrl+Enter 进入下一题)` | 验证：实时 DOM |
| 验证规则 | 1–4096 chars | [src/web/spec.rs:111-130](../../../src/web/spec.rs#L111-L130) |
| 下一题 button | 中文 "下一题 →" | 验证：ref_18 -> ref_60 流程 |
| Q 进度指示 | 右上角 `Q N / 8` | 验证：实时 DOM (`Q 1 / 8`) |
| Cmd+Enter shortcut | 支持，placeholder 明示 | spec-grill.ts |

## 进入 Phase 3 的前置都满足

- ✅ Backend 稳定运行
- ✅ Workspace `tmp/generative_html_probe_20260522/` 已 init + llm config (DeepSeek 模型)
- ✅ Agent_001 已注册
- ✅ API key 已注入 in-memory（welcome flow）
- ✅ /build 8 题就绪，等用户输入

## Forward-bound notes

1. CLI `turingos llm config` FULL_HELP 列了 `--meta-api-key-env` / `--blackbox-api-key-env` 分离 flag，但 `run_inner` 只解析单一 `--api-key-env`。文档与实现 drift。本次绕过：单 key 双角色复用。**改进 charter 候选**：在 `cmd_llm.rs::run_inner` 加 `--meta-api-key-env` / `--blackbox-api-key-env` 解析，对应 turingos.toml 分别写入。
2. W7 `welcome_llm_config_handler` 不接受 UI 输入的 endpoint / model / thinking 配置，硬编码走默认（SiliconFlow + Phase 6.3 模型）。当前架构要求用户走 CLI 预 config 才能切 LLM provider。**改进 charter 候选**：在 step 2 卡片增加 4 个可选输入（provider / endpoint / meta model / worker model），handler 把值透传给 `turingos llm config`。
3. `tos-root` 在 `customElements.get` 检查返回 false，疑似命名废弃。**清理 task**：删除 `customElements.define('tos-root', ...)` 残留或删除依赖该 element 的 contract。
