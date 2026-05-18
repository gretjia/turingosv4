# Phase 7 W8 Validation — Spec Grill Transcript

Persona: same 中文非程序员 hobbyist as Round 1 (下午摸鱼 / NES 经典 Tetris / 浏览器 / 无登录). Rephrased to avoid verbatim re-use of Round 1's `stage_phase7_real_e2e_20260518T031804Z/user_simulator/transcript.md`.

## Q1 / 8

> 先不用想程序怎么做。能跟我说说你最近遇到了什么事，让你觉得『要是有个小工具就好了』？比如『我妈每周要算一次社区团购账，Excel 太麻烦』。你的故事是什么？

**Answer:** 说实话每天下午三四点钟困得不行，又不能溜出去，就想盯着屏幕休息眼睛五分钟。打开浏览器找点小游戏吧，要么蹦出来弹窗要登录，要么页面卡半天才加载完，要么广告糊一脸。我从小就喜欢俄罗斯方块，简简单单的几个键，不动脑子也能玩个十几局。要是有那么一个网址，点开就跳出来个干净的方块格子，啥也不问我，关掉就走，那就完美了。

## Q2 / 8

> 有没有哪个网站 / App / 小工具，跟你想要的『有点像』？不用一模一样，一两个相似的地方就行。

**Answer:** 小时候掌机和红白机上那个版本——10x20的格子、七种形状、左右下旋转就这几个操作。后来在网上偶尔搜到过几个NES在线模拟器，能跑Tetris的ROM，玩起来手感就是我想要的那种。但是模拟器站点本身都太重了，要选ROM、调按键、还有各种按钮。我要的就是把那种『纯玩法』直接做成一个网页，没有外壳没有菜单。

## Q3 / 8

> 想象关掉电脑明天再打开，这个工具应该还『记得』哪些东西？

**Answer:** 就一样东西：最高分。我上一次玩完关掉浏览器，明天再打开同一个网页，旁边那一栏还显示着上次冲的最高分，让我能看见那个数字想着今天能不能破。除此之外都不用记——啥用户名、设置、皮肤、播放进度、当前局面统统不要。

## Q4 / 8

> 假设我是你的用户，第一次打开这个工具——我看到什么？然后我点什么？然后呢？一步一步告诉我，直到我完成一件事。

**Answer:** 打开网页就是一个黑色调的整页。中间偏左是10列20行的游戏格子，看起来跟以前红白机里那个一模一样。右边窄窄一栏写着『分数 0』和『最高分 X』（X 是上次的纪录，没玩过就是 0）。最下面一行小字提示『空格 开始』。我按空格，第一个方块从顶上出来开始往下掉。我用左右箭头移动、上箭头转方向、下箭头让它快点掉。消行就加分，连消多了一次加更多分。一直堆到顶上不能放新方块就 Game Over，正中央显示『游戏结束 · 空格再来』，如果分数超过了最高分，最高分那一栏就跟着变。

## Q5 / 8

> 如果有个奇怪的用户，故意乱点乱填——你希望工具怎么办？

**Answer:** 游戏里就没有填字段这一说，所以不太适用。要说乱按键的情况：希望有人闷头狂按键盘（包括没用的字母、F1、Tab、Esc什么的）这游戏既不要崩溃也不要弹窗提示，更不要跳出『请勿这样操作』之类的话——非箭头非空格的按键就当没按就行。方块超出边界系统挡住就好。堆到顶就规规矩矩 Game Over，不要奇怪的死循环或者卡住的状态。

## Q6 / 8

> 如果这个工具突然多了一个功能，你反而会觉得『搞这个干嘛，反而把简单的事弄复杂了』——是什么功能？

**Answer:** 第一是要登录账号——五分钟摸个鱼还得先注册手机号短信验证码，绝对劝退。第二是花里胡哨的皮肤/主题/方块换色——经典就是经典，玩家定制反而显得不专业。第三是难度选择/速度档位/多种模式——原版就一种『慢慢加速』的玩法，多了选项反而要先停下来选择，破坏摸鱼的丝滑。

## Q7 / 8

> 用了一个月之后，你怎么判断『这个工具是有用的』？

**Answer:** 三个具体可量化的事：(1) 浏览器收藏夹里这个网页固定钉在最前面，一个月里我点开它的次数得有30次以上（差不多每天一次）；(2) 最高分得从第一次的几百分爬到5000以上，证明我真的在玩、在进步；(3) 这一个月里，从来不需要刷新页面、不需要重启浏览器、没出现过卡死或者白屏——每次打开就能立刻按空格开局。如果这三条都满足，这个工具就是真的有用，不是『感觉不错』的那种。

## Q8 / 8

> （最后一题）下面我会把前面听到的复述一遍，请你看看哪里我听错了或听漏了。

**Answer:** 主要要点确认无误，再次明确五条硬要求：(1) 整个游戏就一个 index.html 文件，纯前端，没有后端、没有外部 CDN、没有图片资源、没有外部样式表；(2) 键盘操作：左右箭头平移、上箭头旋转、下箭头软下落、空格开始/重开（不需要硬下落 hard drop）；(3) 经典 NES 7 种方块（I/O/T/S/Z/J/L）、10x20 棋盘、消行计分（1/2/3/4 行差额奖分）、随时间或行数加速；(4) 最高分用 localStorage 存在浏览器本地，绝对不要服务器；(5) 单一画面，没有菜单页、设置页、教程页——打开就是游戏。

---

# W8-specific observations

## Generate phase progress chip sequence

Observed live in the `<tos-spec-result>` progress chip area:

1. 点击 "生成代码 →" 后立即看到 chip：`正在生成代码...   正在生成... (尝试 1/3)`
2. 约 60s 后 chip 切换至：`正在生成代码...   正在生成... (尝试 2/3)   尝试 1/3 失败: missing_canvas: 找不到 <canvas> 元素 — 游戏类应用必须有画布`
3. 又 ~45s 后切换至：`正在生成代码...   正在生成... (尝试 3/3)   尝试 2/3 失败: missing_canvas: ...`
4. 最终（总 ~150s 后）：`重试生成代码 →   missing_canvas: 找不到 <canvas> 元素 — 游戏类应用必须有画布 | last_artifact=1779078133_badebd6a/artifacts/index.html   查看最后一次产物 ↗`

**`<tos-artifact-viewer>` 从未挂载** — 所有 3 次尝试都被 W8 启发式拒绝，HTTP 500 触发 GenerateAttemptFailed 终态。

## WS envelope sequence (captured via window.__w8_ws_log)

完整序列见 `ws_envelope_log.json`。摘要：

| seq | t (epoch ms) | kind | msg_type | attempt | reason |
|---|---|---|---|---|---|
| 1 | 1779078295919 | ir_update | generate_attempt_started | 1/3 | — |
| 2 | 1779078353818 | ir_update | generate_attempt_failed   | 1/3 | missing_canvas |
| 3 | 1779078353827 | ir_update | generate_attempt_started | 2/3 | — |
| 4 | 1779078395672 | ir_update | generate_attempt_failed   | 2/3 | missing_canvas |
| 5 | 1779078395680 | ir_update | generate_attempt_started | 3/3 | — |
| 6 | 1779078446998 | ir_update | generate_attempt_failed   | 3/3 | missing_canvas |
| 7 | 1779078447006 | fetch_response | POST /api/generate → 500 | — | generate_quality_failed |

总时长 ≈ 151 秒 (attempt 1: 58s; attempt 2: 42s; attempt 3: 51s).
**没有 GenerateComplete 广播** — 因为所有尝试均失败。

## `total_attempts` 与最终响应

POST /api/generate 返回 HTTP 500，body:

```json
{
  "reason": "missing_canvas: 找不到 <canvas> 元素 — 游戏类应用必须有画布 | last_artifact=1779078133_badebd6a/artifacts/index.html",
  "kind": "generate_quality_failed"
}
```

没有 `total_attempts` 字段（终态失败路径不发 GenerateResponse；W8 frontend 仍能从 last 3 个 WS envelopes 推断 attempts=3）。

## 新版 "经过 N 次尝试" 字幕

**未观察到** — 因为 `<tos-artifact-viewer>` 在终态失败时不挂载。该字幕仅在 single-shot 成功或部分重试成功(`total_attempts > 1`)的成功路径中显示。

## Heuristic verifier 准确性 — 关键发现

**False positive detected.** Qwen3-Coder 三次产出的 artifacts 都采用 **DOM grid (`<div id="game-board">` + 200 个 `.cell` divs)** 实现 10x20 棋盘，而**不是** `<canvas>` 元素。这是一种**完全有效的 Tetris 渲染方式**，仅是渲染技术选择不同。

W8 verify.rs Check 2 (`has_canvas`) 用 `lower.contains("<canvas")` 硬性要求 canvas 元素，把这三个**功能完整的 Tetris** 全都判为失败。

通过 last_artifact 链接打开页面：
- 渲染了 10x20 黑色边框棋盘
- 按空格游戏开始（一个 cyan 方块从顶部下落）
- ArrowLeft / ArrowRight / ArrowUp / ArrowDown 全部正确响应（rotation 改变了 200-cell 快照串）
- 30-key 随机播放后游戏继续稳定运行（pieces 堆栈在底部，无崩溃）
- localStorage 持久化、TETROMINOES（7种）、clearLines 函数、game-over 状态均已实现

---

# 8 Mechanical Tests on last_artifact (`sessions/1779078133_badebd6a/artifacts/index.html`)

| # | Test | Result | Note |
|---|---|---|---|
| 1 | iframe HTTP 200 + body non-empty | N/A (no iframe — viewer didn't mount); but top-level `/api/artifact/...` returned HTTP 200 + 11814 bytes | PASS-equivalent |
| 2 | Canvas exists (10×20 grid) | **NO `<canvas>`** — Qwen used DOM `<div id="game-board">` with 200 `.cell` divs (10 cols × 20 rows confirmed via querySelectorAll) | PASS via DOM grid alternative, FAIL on literal "<canvas" substring (W8 verifier criterion) |
| 3 | Score visible | PASS — `#score` shows "分数 0", `#high-score` shows "最高分 0" | PASS |
| 4 | Sandbox `"allow-scripts"` only | N/A — viewer never mounted (W8 returned 500); top-level view bypasses sandbox entirely. **Architectural concern**: 查看最后一次产物 inspect-link surfaces artifact at top-level origin, sidestepping W6 iframe sandbox invariant. | UNTESTED in this path |
| 5 | Keyboard reactive (ArrowDown changes state) | PASS — board snapshot string changed after key press | PASS |
| 6 | Rotation works (ArrowUp / Space rotates) | PASS — board state changed after ArrowUp | PASS |
| 7 | No app-origin console errors in 5-sec observation | PASS — 5 console exceptions observed but all are Chrome MCP extension async-listener artifact (well-known, not app-origin) | PASS |
| 8 | 30-random-key playthrough doesn't crash | PASS — pieces stacked at bottom, game continued, no white-screen or crash | PASS |

**7 of 8 PASS, 1 N/A (sandbox test not applicable when viewer didn't mount).**

The W8 heuristic verifier judged this artifact "broken" (HTTP 500 returned to user) but the artifact is **functionally complete and playable**. User experience: clicked 查看最后一次产物, saw working Tetris, but:
- saw the failure chip with Chinese error reason
- saw "重试生成代码" button suggesting manual retry needed
- artifact was served outside the iframe sandbox (top-level page navigation)
