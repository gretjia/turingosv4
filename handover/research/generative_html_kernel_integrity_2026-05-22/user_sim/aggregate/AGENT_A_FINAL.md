# Agent A Final Report (late-arriving, post-salvage)

**Returned at**: 2026-05-22 ~15:50 (7 hours 16 minutes from dispatch)
**Status**: completed (natural termination; orchestrator had already salvaged + opened PR #91 at this point)

Agent A's authoritative final verdict supersedes the salvage-agent estimates in
[verdict_summary.md](verdict_summary.md) for any conflict. Salvage estimates remain
preserved for provenance.

## Final per-persona verdict (corrects salvage estimates)

| Persona | Salvage verdict | **Agent A final** | Delta |
|---|---|---|---|
| P01 小学五年级学生（挂机游戏） | PARTIAL | PARTIAL | (consistent — JS syntax error blocks Canvas) |
| P02 退休教师（英语单词卡） | PARTIAL | PARTIAL | (consistent — artifact exists, mid-script truncation per A) |
| P03 奶茶店老板（销量日报） | PARTIAL | **FAIL** | A 实测 generate 无 body 落盘 (tape-relay 截断) |
| P04 越南留学生（生日贺卡） | PARTIAL | **PASS** ⭐ | **A 真 iframe 交互验证：name + 祝福语 → 卡片生成动画——本 probe 唯一完整 end-to-end PASS** |
| P05 产品经理（匿名投票） | PARTIAL | **FAIL** | A 实测 generate 无 body 落盘 (同 P03) |
| P06 焦虑用户（日记工具） | PARTIAL | PARTIAL | runtime 行为 (onclick/localStorage) 未测 |
| P07 对抗输入 (XSS/注入) | PARTIAL | PARTIAL | spec 已 defang (Class 3 shielding PASS)；generate 未运行 |
| P08 决断工程师（计时器） | PARTIAL | PARTIAL | runtime 行为未测 |

**总计**: 1 PASS / 5 PARTIAL / 2 FAIL（替代 synthesis REPORT §3 中"3/8 artifact + 0/8 真交互 PASS"的估计）

## 新发现 (synthesis 未捕获)

### NEW-A1: Tape-relay 累积 prompt 截断 (架构层)

Agent A 实测 Generate pipeline 失败的根因比 BUG-3a/3b 更深：

> 多次失败的 tape-relay 累积使 prompt 变长 → LLM 截断输出 → p03/p05 无 body，p02 mid-script 截断

含义：W8 retry 不是无成本的——每次失败的 attempt 会被加进下次 prompt 的 context。3 次 retry 后 prompt 可能已 30K+ tokens，触发 LLM `max_tokens` 截断。

**Forward charter 候选**：W8 retry 应该 reset prompt 而非 accumulate；或 retry 切换到更短上下文 prompt；或显式 max_tokens 检测。

### NEW-A2: `deepseek-v4-pro + thinking=off` 组合在 DeepSeek 直连返回 HTTP transport error

Agent A 调试中发现：blackbox 模型 turingos.toml 配置存在某种组合错误。**这一点需要核查**——我（orchestrator）的 turingos.toml 显示 `blackbox.model = "deepseek-v4-flash"`（正确的 flash worker），但 A 可能在 debug 过程中临时改过某个 session 的 toml。或这是另一个真实 bug，需要让 follow-up parallel session 核实。

### W8 验证 gap 更细化 (扩展 BUG-6)

A 列出 3 项 W8 gap：

1. **JS SyntaxError 未被 `HtmlParses` 检测** (p01) — 与 synthesis BUG-6 一致
2. **无 body 的 HTML 通过 `HtmlParses`** (p03/p05) — **新**，HtmlParses 应至少检查 `<body>` 存在 + 非空
3. **Runtime 行为（onclick / localStorage）未测试** (p02/p06/p07/p08) — 对应 synthesis Charter C 范围

## P04 PASS 详情（synthesis 未充分突出）

**这是本次 probe 唯一完整 end-to-end PASS 的 persona**，值得在 synthesis §3 升级强调：

- Spec：越南留学生英文 + 中文混合输入，Meta AI 正确合成 spec.md
- Generate：deepseek-v4-flash 一次性产出可用 artifact（12.2KB index.html）
- Iframe 交互：Agent A 在 Chrome 输入姓名 + 祝福语，触发卡片生成动画——artifact 实际可玩
- Capsule chain：CAS 内 git-backed，`git cat-file -t` 全 blob

**含义**：generate pipeline 在某些路径下确实工作；不是"全 broken"。Charter B (driven mode + prompt hash) 启动后期望成功率显著提升。

## CAS chain 验证 (synthesis 已 cover, 此处确认)

> All CAS chains are git-backed and readable (`git cat-file -t` returns `blob` for all first capsules). The evidence is durable.

8 个 session 的 capsule chain 全部从 `cas/.turingos_cas_index.jsonl` + git substrate 可重建。FC3 capsule-from-tape 在 generative HTML 域 GREEN 的结论加强。

## 对 synthesis/REPORT.md 的修正

| §| 原结论 | 应修正为 |
|---|---|---|
| §0 Executive Summary | "real generation pipeline BROKEN" | "real generation pipeline PARTIALLY WORKS — 1/8 真 end-to-end PASS, 5/8 PARTIAL, 2/8 FAIL；BUG-3a/3b 已 land 后期望 PASS 比例上升" |
| §3 User experience | "0/8 真交互 PASS" | "1/8 真 iframe 交互 PASS (P04 生日贺卡)；5/8 PARTIAL；2/8 FAIL" |
| §2 Bug table | BUG-6 仅"JS syntax error" | BUG-6a JS syntax, BUG-6b no-body HTML, BUG-6c no runtime behavior test |
| §6 Charter C scope | "fetch/XHR 静态检测" | + "W8 prompt-accumulation reset" + "no-body HTML check" |
| §9 Ship readiness | "Generate pipeline ⚠️ CHALLENGE" | "Generate pipeline ⚠️ CHALLENGE — 1/8 真 PASS 证明非全 broken；BUG-3a/3b land 后期望升至 5+/8" |
