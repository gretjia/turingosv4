# Phase 7 W7.1 hotfix — full E2E walkthrough log (round 1)

Date: 2026-05-18
Server: PID 74613, http://127.0.0.1:8080
Workspace: `tmp/phase7_active` (fresh clean start)
Backend override: `/tmp/turingos_passthrough_stub.sh` (passthrough for init/llm/agent; stub for spec/generate)

## Hotfix applied this round

W7.1 — `frontend/src/turingos-root.ts`. The W7 onboarding soft-redirect fired
whenever `next_step !== 'Done'`, which incorrectly redirected /build → /welcome
the moment the user finished onboarding (since the next_step then becomes
`Spec`, then `Generate` — both of which are user-driven steps that *happen on
/build*). Narrowed the redirect to only fire when next_step is one of the four
onboarding-wizard-controlled states: `Init`, `LlmConfig`, `ApiKey`,
`AgentDeploy`.

## 15-step walkthrough results

1. PASS — GET / → 303 redirect to /welcome (Location header confirmed via curl + JS `location.pathname=/welcome`)
2. PASS — Welcome page step 1 active (`STEP 1 / 5`, `第一步 · 准备工作站`)
3. PASS — Click 准备工作站 → POST /api/welcome/init 200 → state moves to `step_llm_config`, `init_done: true`
4. PASS — Click 写入 turingos.toml → POST /api/welcome/llm-config 200 → `step_api_key`, `llm_config_done: true`
5. PASS — Type `sk-stub-test-architect-walkthrough-fake-key-9999` into password input + click 保存密钥 → POST /api/welcome/api-key 200 → `step_agent_deploy`, `api_key_set: true`
6. PASS — Click 注册 agent_001 → POST /api/welcome/agent-deploy 200 → `step_ready`, `next_step: Spec`, `agents_count: 1`
7. PASS — Click 开始 spec 访谈 → URL becomes `/build`, `<tos-spec-grill>` is mounted, no redirect-back-to-/welcome (hotfix verified)
8. PASS — Filled 8 plausible Chinese answers; grill advanced Q1→Q2→…→Q8 then transitioned to `spec_ready`
9. PASS — POST /api/spec/submit returned 200 with spec_md
10. PASS — `<tos-spec-result>` rendered the stub spec.md (title 用户工具需求, paragraphs visible)
11. PASS — Click 生成代码 → POST /api/generate returned 200
12. PASS — `<tos-artifact-viewer>` mounted with iframe sandbox value EXACTLY `"allow-scripts"` (no allow-same-origin)
13. PASS — iframe src `/api/artifact/1779071921_1c74b8d5/index.html` returned 200 and rendered the stub UI including the JS-driven timestamp
14. PASS — 0 app-origin console.error / EXCEPTION messages. Only 1 app-origin console message: `INFO  TuringOS frontend ready, view: build`. The 5 "[EXCEPTION]" entries logged are Chrome extension `message channel closed` errors at `(http://127.0.0.1:8080/build:0:0)` (line:col 0:0 is the marker of extension content-script errors, not app JS)
15. PASS — Network log: in the fresh-start segment, /api/welcome/* (5 calls) all 200, /api/spec/questions 200, /api/spec/submit 200, /api/generate 200, /api/artifact/1779071921_1c74b8d5/index.html 200. No 4xx or 5xx in the fresh-start segment

## Iframe sandbox verification (verbatim)

```js
const iframe = document.querySelector('iframe.artifact-viewer-iframe');
iframe.getAttribute('sandbox') === 'allow-scripts'  // true
iframe.getAttribute('sandbox').split(/\s+/).includes('allow-same-origin')  // false
is_xss_safe: true
```

## Final server state

- PID 74613 listening on http://127.0.0.1:8080
- Workspace: /Users/zephryj/work/turingosv4/tmp/phase7_active
- Backend override: /tmp/turingos_passthrough_stub.sh (architect should clear this for real LLM)
