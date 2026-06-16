# H-HET-1 探针 Debug 裁决 + 交接（2026-06-14 晚）

> 写给**新 session（Fable）接手者**的全貌交接。前一份夜间交接是
> `H_HET_1_*` 之外的内联 prompt（已归档于对话）；**本文件取代它，是当前权威状态。**
> 本文自含——不依赖任何 `/private/tmp/...` 会话临时文件存活；决定性结论已逐字落盘于此 + 两个测试文件 + evidence 目录。

---

## 0. 给 Fable 的第一指令（最高优先，先读）

1. **方法纪律（架构师明令）**：调研（内部代码 + 外部 best-practice）→ 思辨（对抗）→ 总结 → 再行动。**不要仓促重跑。** 每修一个 bug ≠ debug 完成。
2. **当前不要重跑实验**（不要拿模型跑 het 探针）。重跑前必须先过 §10 的有序前置门。已确认**第 3 个 bug**，对抗假设**可能还有第 4 个**——任何"干净"结论都要用**正控 + 已知好证明真跑**实证，不靠 review。
3. **模型路由**：本任务是 **C 级**（开放式取证 / 专家域 / 对抗裁决）→ Opus/**Fable** + xhigh，**永不降档**，清洁上下文，真题真跑。用 Fable 正合适。
4. **安全**：`~/work/turingosv4/.env` 的 `SILICONFLOW_API_KEY` **绝不回显/落盘/进 prompt**；不打 `api.siliconflow.cn`；不碰 trust-root（`runtime/mod.rs`、`lib.rs`、`constitution_*.rs`）。本次取证全程 read-only + 仅新增 tests/ 加性测试。
5. **本文件相对位置**：`handover/H_HET_1_DEBUG_HANDOFF_2026-06-14.md`（repo 根 = `~/work/turingosv4`）。

---

## 1. 科学问题（不变）

**H-HET-1 能力扩张**：在**修复后的代码**上，跨实验室异质模型（Qwen3-32B / GLM-4.5-Air / Qwen3.5-397B…）能否解开同质 deepseek-v4-pro 解不开的 Lean 定理？= 「用户(路由∧担保)→涌现」因果链里「异质性能否点活信号」那一环的实证。
- 探针 bin：`src/bin/het_capability_probe.rs`（standalone；复用 `LeanJudge` + `lean_theorem_bank` + `llm_http`；不碰 `lean_market_agent`）。
- judge：`src/judges/lean_judge.rs`（`assemble` + `dedent` + `verify` + 公理门）。
- 题库：`tests/fixtures/lean_theorems_pool.jsonl`（44 题，每题带 `reference_body` 已知好证明，标注 "SELF-TEST ONLY"）。
- Lean 工具链：`~/.elan/bin/lean`（pinned 4.24.0），mathlib4 在 `~/work/mathlib4`。

---

## 2. TL;DR 裁决表

| 问题 | 裁决 | 证据强度 |
|---|---|---|
| **Q1 debug 干净了吗？** | **否** | 真跑实锤（非 review） |
| ├ judge 主路径（uniform 缩进体） | **干净** | E1 正控 6/6 Verified+axiom-clean，我独立重跑 57s |
| └ extract→dedent（first-line-shallow 体） | **第 3 个 bug，实锤** | 2/2 已知好证明 Verified→Failed；报错与真实记录逐字吻合 |
| **Q thinking/token 体制混淆** | **确认，"同质非思考"前提是假的** | 源码 + WebFetch + ct==2048 确定性指纹 |
| **"control"（lm_det_zero）是 judge 正控吗？** | **不是**，是模型生成的随机 cell | grep 全 probe 零特判 |
| **「6 道 never-solved」可信吗？** | **否，NULL prior** | lm_det_zero 实为 EASY；lm_nt_cop_cubic n=0 |
| **另一 session 的难度真值（27/44、bear-triage null）** | **污染，NULL prior** | 全过同一 buggy assemble |
| **现在能重跑吗？** | **不能** | 先过 §10 有序前置门 |
| **baseline 方案** | **Option A：4 模型自含一跑** | within-run 配对，隔离最强 |

---

## 3. 我做了什么（工作全貌）

三段式，全程不重跑实验：

**(A) 内联取证（ground truth）**——`ps`/读 records/读源码，确认：
- PID 79070 **已死**；无 probe/lean/cargo 在跑。
- 5 个 run 目录都在 `handover/evidence/`（夜间交接写成 `handoff/` 是笔误）。
- `MAX_TOKENS=2048`；请求体只有 4 字段、**无 thinking 抑制**；"control" 主循环**零特判**。

**(B) Workflow `het1-debug-resummary`（8 agents，885K tok，28min）**——research(5 并行) → 模型自由 judge 正控 + baseline 设计 → 对抗 debug 裁决。
- 脚本（会话内）：`~/.claude/projects/-Users-zephryj-work-turingosv4/08ba1c4f-.../workflows/scripts/het1-debug-resummary-wf_6c2539e6-e40.js`
- 完整输出（**临时，可能被清**）：`/private/tmp/claude-501/-Users-zephryj/08ba1c4f-7d.../tasks/wsg92gmzp.output`（339 行 JSON；关键结论已逐字搬进本文，不依赖它存活）。

**(C) 我亲自真跑复核（不信子 agent 自评）**——独立重跑 E1 正控（6/6 绿），并**新写一条判决测试把第 3 个 bug 从"声称"变成 PASS/FAIL 事实**（2/2 实锤）。

---

## 4. 决定性证据（真跑原文）

### 4.1 E1 正控（judge 主路径干净，uniform 体）
`cargo test --test het_probe_pool_reference_bodies_verify -- --nocapture`，独立重跑 57.31s：
```
OK lm_det_zero:    Verified, axioms=["Classical.choice","Quot.sound","propext"]
OK lm_c:           Verified, axioms=[...]
OK lm_coeff_mul:   Verified, axioms=[...]
OK lm_e:           Verified, axioms=[...]
OK lm_lim1:        Verified, axioms=[...]
OK lm_nt_cop_cubic:Verified, axioms=["propext"]
test result: ok. 1 passed
```
→ 把 reference_body（uniform 2-space 缩进）直灌真 `LeanJudge::verify`，全 Verified + 公理白名单内。**"judge 坏了"对 well-formed 体被排除。**

### 4.2 第 3 个 bug 判决测试（extract→dedent，first-line-shallow 体）
`cargo test --test het_third_bug_dealign_decisive -- --nocapture`，21.83s：
```
[lm_det_zero]      uniform → is_verified=true | shallow(同一证明) → is_verified=false
                   feedback: ...lean:10:50: error: unsolved goals
[lm_nt_cop_cubic]  uniform → is_verified=true | shallow(同一证明) → is_verified=false
                   feedback: ...lean:4:74: error: unsolved goals
THIRD BUG CONFIRMED: 2/2 known-good proofs mislabeled Failed by first-line-shallow de-alignment.
```
两臂是**同一证明内容**（逐行 trim 后相等），唯一差别是首行缩进 → 判决翻转**只能归因于 dedent 去对齐**。

### 4.3 冒烟枪（bug 当晚就在发作）
我故意 de-align 的 lm_det_zero 已知好证明，报错 = `10:50 unsolved goals`。
当晚真实 `handover/evidence/het_probe_v4_3recs/records.jsonl` 的 lm_det_zero：
```
attempt 0 → ...79070-0.lean:10:50: error: unsolved goals   ← 与我的复现逐字相同
attempt 1 → ...79070-1.lean:10:50: error: unsolved goals   ← 同
attempt 2 → ...79070-2.lean:10:50: error: unsolved goals   ← 同
attempt 3 → ...79070-3.lean:16:6:  error: unexpected token (另一种输出形)
```
**4 次里 3 次**带第 3-bug 签名。即当晚 lm_det_zero 的 all-Failed **至少部分是 bug 不是能力**（且 lm_det_zero 本就 EASY，见 §7）。
> 诚实边界：records 只存了 error note、没存每次模型原文，故不能逐条断言"全是 bug"；但**机制已确认 + 签名逐字吻合 + 该题实为 EASY** 三者叠加，足以判定当晚 Failed 计数**被污染、不可读作能力**。

---

## 5. 第 3 个 bug：根因 + 精确修复配方

**位置**：`src/judges/lean_judge.rs:414 pub fn dedent`（被 probe 提取 5 处 + assemble 调用）。
**根因**：`dedent` 只剥**最长公共前导空白前缀**。当证明体**首行比兄弟行浅**时，兄弟行仍深于 col-0 锚点 → 落 `by` 块外 → 正确证明被误判 Failed。这是**模型最自然的输出形**，两个注入点：
1. **JSON proof_body**：`"simp [...]\n  ring"`（首行 col0、后续 col2）→ 公共前缀 `""` → dedent 原样返回。
2. **`extract_after_by` inline**：`":= by tac1\n  tac2"` → `clean[pos+5..]` = `" tac1\n  tac2"`（首行 1-space、后续 2-space）→ 公共前缀 1-space → dedent 后 col0/col1，仍错位。

上一夜修的两个 bug 都**没覆盖**这一类：
- bug#1 doubled-signature（`het_capability_probe.rs:417` `BUGFIX (audit wpgyhkjxc)`）——已修。
- bug#2 de-align **uniform** 情形（`lean_judge.rs` `dedent`，另一 session 修）——只处理"所有行同缩进"，**对首行更浅无效**。
- bug#3（本次）= de-align **first-line-shallow** 情形——**未修**。
- **附带 latent**：tab/space 混用时公共前缀=空 → 两行都不剥（`"\tsimp\n  ring"`）。当前 6 题 reference 无 tab，潜伏。

**精确修复配方**（建议）：把 dedent 从"剥公共前缀"改为**回锚到所有非空行的最浅列**——
- 计算所有非空行的缩进宽度（**先做 tab→空格展开**，按固定宽度），取最小值 `m`；每行剥 `m` 列（按展开后宽度），使最浅行落 col0、相对嵌套保持。
- 改完必须 **E1 正控 + 第3-bug判决测试双绿**才算修好（后者届时应从"确认 bug"反转为"shallow 也 Verified"）。
> ⚠️ cascade 风险：`dedent` 是 `lean_judge.rs` 共享函数，改动会影响所有走 assemble 的路径（含 lean_market_agent）。有两条判决测试兜底，但改完要跑全 `cargo test -p turingosv4` 看回归。lean_judge.rs 接近 trust-root 敏感区，按 §8 守宪法 gate。

---

## 6. thinking / token 体制混淆（确认，前提是假的）

- 请求体（`het_capability_probe.rs:155-214`）只有 `{model, messages, temperature=0.7, max_tokens=2048}`，**无 `enable_thinking`/`thinking_budget`/`chat_template_kwargs`**。line 53 注释自称 "non-thinking" 与结构体**自相矛盾**。
- **WebFetch 实证（verified_on 2026-06-14，已入卡）**：
  - SiliconFlow `enable_thinking` **默认 true**（官方 API ref）；`thinking_budget` 默认 4096。
  - Qwen3-32B / GLM-4.5-Air / Qwen3.5-397B-A17B **三者都默认思考**（HF model cards / SiliconFlow 目录；GLM 旧 `{"thinking":{"type":"enabled"}}` 已废，须用 flat `enable_thinking`）。
  - `max_tokens` **把 reasoning 一并计入并截断**（reasoning 与 content 共享预算）。
  - `Qwen3.5-397B-A17B` id **真实存在**、SiliconFlow 在售 → 那些记录是真实尝试，非"假成功"。
- **确定性指纹**：每条 ParseError 的 `completion_tokens` **正好 == 2048**（127 条数据 8/8；smoke 1/1）；其余 ct 高达 12491(>>2048) 证明 reasoning 没被 cap。`MAX_TOKENS` 常量从未变 → ct 矛盾是 **API 语义**，非构建漂移。
- **量化**：截断伪 Failed ≈ 真实非-Verified 的 **10–15%（下界）**；另 42/90 Failed 的 ct≥1900（视觉证明可能被悄悄截断）。其余 ~85% 是真 Lean 语义拒绝（unsolved goals / tactic failed / type mismatch）。
- **`strip_think_tags`**（`het_capability_probe.rs:484`）在 `</think>` 缺失（截断case）时是 **no-op** → 残留 reasoning 文本喂给提取器 → 系统性偏向 ParseError，**对思考模型有偏**。

**结论**：「同质非思考体制 / 0 跨厂破解」**前提不成立**，当晚结果不可作能力推断。

---

## 7. baseline + 靶题：全部 NULL prior

- **「6 道 never-solved」唯一来源** = `het_capability_probe.rs:50-51` 一句**本会话未提交**的代码注释，在 buggy 码上定的。**lm_det_zero 实为 EASY**（`evidence/p1_v4_2026-06-02/STAGE_A_HARD_FLOOR_FINDINGS.md` 标 6/6 EXCLUDED；deepseek pilot 解过；smoke 弱 Qwen 都 Verified；DEALIGN 目录 397B 两次 Verified），是被错误换进来顶替 **lm_probe1**。**lm_nt_cop_cubic 当晚 n=0**（零真实尝试，never-solved 标签无数据支撑）。
- **另一 session**（Session A，bear-triage 同质 deepseek-v4-pro，见 `SESSION_COORDINATION_2026-06-13.md`）的 A/B null + 「27/44 封顶、17 不可约」**全过同一 buggy assemble**；pilot 证明 deepseek 真解了很多 → provenance **混合**，per-cell Failed **不可整批信**。
- **NULL prior（不可当先验）**：const 题集、SUMMARY 题表、bear-triage per-cell 判决、27/17 难度划分。
- **可复用（未污染）**：bank reference_body（E1 已核）、pre-session STAGE_A floor、**任何 Verified+axiom-clean 记录**（bug 只产生**假阴性** → Verified 不可能假阳）。

**baseline 推荐 Option A：deepseek-v4-pro + 3 跨厂模型，同一 binary / judge / regime / K / 题集 / 时间窗，一次跑完**（within-run 配对对照）。隔离最强、决定性最高（同跑 deepseek 0/K vs 跨厂 Verified+axiom-clean = 自含铁证）。B（懒按需）/C（先建 never-solved 再跑）都重新引入跨跑 seam（正是当初污染 baseline 的失败模式），驳回。probe 现已含 `deepseek-ai/DeepSeek-V4-Pro`（MODELS[0]，K=3）。

---

## 8. 文件 / 产物地图

**源码（全部本会话工作，无一在 HEAD）**
| 路径 | 状态 | 说明 |
|---|---|---|
| `src/bin/het_capability_probe.rs` | untracked (??) | 探针，937 行。含 bug#1 修复（line 417）；bug#3 在提取/dedent 路径未修 |
| `src/judges/lean_judge.rs` | modified (M) | **另一 session 的** de-align(uniform) 修改；`dedent`(414) 即 bug#3 根因 |
| `src/judges/lean_theorem_bank.rs` | tracked | `LeanTheorem.reference_body`（SELF-TEST ONLY），`load_bank` |

**测试（我新增的加性回归门，untracked，可直接接 gate）**
| 路径 | 作用 | 当前结果 |
|---|---|---|
| `tests/het_probe_pool_reference_bodies_verify.rs` | E1 模型自由 judge 正控（6 题 reference_body 过真 judge） | **绿**（应保持绿） |
| `tests/het_third_bug_dealign_decisive.rs` | 第 3-bug 判决（同一好证明 uniform vs shallow） | **当前"绿"=bug 已复现**；修好 dedent 后应改判逻辑/反转 |

**证据目录** `handover/evidence/`
| 目录 | 内容 | 可信度 |
|---|---|---|
| `het_probe_run/` | SUMMARY.json(run1,49rec)、SUMMARY2.json(run2,127rec)、QC_STATUS.json、run{,2,3,4,5}.log、finisher.* | 跑历史；records.jsonl 本体已移走 |
| `het_probe_v4_3recs/` | 4×lm_det_zero Failed（**冒烟枪 10:50**） | 模型生成、含 bug |
| `het_probe_smoke/` | lm_det_zero 1×Verified(运气)+1×ParseError(ct=2048) | 污染 |
| `het_probe_CONTAMINATED_bug/` | 127rec（doubled-sig 污染 + grid-mix） | 隔离留证 |
| `het_probe_DEALIGN_contaminated/` | 14rec（de-align 污染；含 397B×2 Verified lm_det_zero） | 隔离留证 |
| `p1_v4_2026-06-02/` | STAGE_A floor、buggy never-solved manifests | floor 可用，manifests 污染 |

**Workflow 临时产物（可能被清，内容已搬进本文）**
- 脚本：`~/.claude/projects/-Users-zephryj-work-turingosv4/08ba1c4f-.../workflows/scripts/het1-debug-resummary-wf_6c2539e6-e40.js`
- 输出：`/private/tmp/claude-501/-Users-zephryj/08ba1c4f-.../tasks/wsg92gmzp.output`

---

## 9. 仓库状态 / dirty-tree 归属

- 分支 `claude/p1-realvalue`，HEAD `4cfbc41e`；工作树 **~1000 脏文件**（长期 dirty tree，多 session 并行）。
- **本次取证**：未改任何 source；仅**新增 2 个 tests/ 加性文件**；`lean_judge.rs` 的 M 是**别人的修改不是我的**（按 dirty-tree 归属纪律：`git status` 该文件 → 非我所改）。
- **没有任何相关代码在 HEAD**——probe 整支 untracked、test untracked、judge 改动未提交。任何重跑必须用**当前新构建的 binary**，不是 stale build。

---

## 10. 未来规划：有序前置门（act-with-data，**不重跑**直到全绿）

> 顺序是硬依赖，逐项门控。前 4 项**不涉及任何模型/网络**，可立即做。

- **门 0（根因，最高杠杆）修 dedent**：按 §5 配方回锚到最浅列 + tab 展开。验收 = E1 正控**保持绿** + 第3-bug判决测试**反转为 shallow 也 Verified**。再跑全 `cargo test -p turingosv4` 查 cascade 回归。
- **门 1 加截断检测**：`call_model` 读 `finish_reason=="length"` → 记成独立 `Truncated` verdict，别再悄悄算 ParseError/Failed。
- **门 2 定死单一体制**：二选一并对**所有模型**一致——(a) `enable_thinking:false` @ ~2048（真非思考）；或 (b) 思考开 @ `max_tokens≥16k`（难题 32k）/ 显式 `thinking_budget`。deepseek vs Qwen/GLM 的默认 reasoning 差异本身是新 confound，必须中和。
- **门 3 接 gate**：把两条测试写进 `scripts/constitution_gates.manifest.toml`（+ 矩阵/allowlist），当**绑定回归门**，不是一次性。
- **门 4 靶题重定**：剔 lm_det_zero（实 EASY）、补 lm_nt_cop_cubic 真尝试；对 lean_market_agent 做 equal-budget 公平性（AGENTS §17 G1–G6）；新跑用 fresh records + 单一固定 grid + prereg + 命名输出 + 披露 ApiError(~22%)/截断(~15%)份额。
- **门 5（拍板后）重跑**：Option A 四模型自含跑，K≥6（STAGE_A 证 0/3 在真率~60% 时有 ~5% 假阴），从该跑自身 tape 重建 baseline，不导入任何污染先验。

**我的下一步建议**：先做**门 0–3**（纯本地、无重跑、cascade 有测试兜底），全绿后再请架构师拍板体制(门2具体值)与 Option A 重跑。

---

## 11. 现存问题 / 未决（对抗式：假设还有坑）

1. **第 4 个 bug?** 已确认 bug#3，按纪律对抗假设还有。门 0 修完 dedent 后，应再写一条"模型真实输出形覆盖"测试（多种缩进/JSON/inline/`<think>`残留组合过 `extract_proof_body`），别只信 dedent 单点修好就干净。
2. **截断 vs 能力的精确切分**未做：现有 records 没存模型原文，只能从 ct/error note 估 ~15% 下界。门 1 之后的新跑才能精确归因。
3. **strip_think 只匹配小写 `<think>`**，不处理 `<thinking>`/`<reasoning>`/provider reasoning_content 通道——换模型时是新偏置。
4. **证据卫生**：`het_probe_run/records.jsonl` 本体已移走（log 路径与实际不符）；两个 `*_contaminated` 目录是 resume/grid-mix 漂移。任何数字结论必须 scope 到单一命名 records 文件 + 其 grid。
5. **equal-budget 公平性未立**：探针单 shot、无 retry feedback、无 system prompt，与 lean_market_agent 多步 loop 不对等 → "never solved by the swarm" 非苹果对苹果。
6. **deepseek-v4-pro 在修复码上的真 baseline 尚未跑**（门 5）——"never-solved" 在重建前都是 NULL。

---

## 12. 一键复现命令（让 Fable 几分钟内重核，无需重跑实验）

```bash
cd ~/work/turingosv4

# (1) 进程态 + 冒烟枪签名
ps -p 79070 -o pid,stat,command 2>/dev/null || echo "dead"
python3 -c "import json;[print(r['attempt'],json.loads(l)['note'][-45:]) for l in open('handover/evidence/het_probe_v4_3recs/records.jsonl') for r in [json.loads(l)]]"

# (2) E1 正控（judge 主路径干净，~60s）
cargo test --test het_probe_pool_reference_bodies_verify -- --nocapture --test-threads=1

# (3) 第 3-bug 判决（~22s；当前"绿"=bug 复现）
cargo test --test het_third_bug_dealign_decisive -- --nocapture --test-threads=1

# (4) 关键源码定位
grep -n 'MAX_TOKENS\|enable_thinking\|max_tokens\|finish_reason' src/bin/het_capability_probe.rs
grep -n 'fn dedent' src/judges/lean_judge.rs            # 414 = bug#3 根因
grep -n 'lm_det_zero' src/bin/het_capability_probe.rs    # 仅 51(数组)+886(注释)=零特判
```

---

## 13. 一句话状态

**debug 未完成（第 3 个 bug 实锤，judge 主路径干净）；体制混淆确认（thinking 默认开 + 2048 截断）；never-solved 集与跨 session 难度真值全污染；现在不重跑——先过门 0–4（纯本地修复+控规+接 gate），再拍板 Option A 四模型自含重跑。** 模型用 Fable，C 级永不降档，真题真跑别靠 review。
