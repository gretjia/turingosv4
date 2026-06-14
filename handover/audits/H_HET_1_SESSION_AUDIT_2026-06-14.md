# H-HET-1 会话审计文档（2026-06-14，单一自含审计文件）

> **用途**：外部审计师可凭本文件独立复核本会话全部工作，无需对话记录。每条结论附证据路径、实验数据、关键原代码片段（带 `file:line`）。
> **诚实纪律**：本文件不含任何 `PROVEN`/`DEFINITIVE`/`causal` 头条；H-HET-1 的科学结论（异质性能否点活信号）**尚未取得**——载体已建成+审计通过，但实验未跑。已验证 / 推断 / 未决三者全程区分。
> **风险类别**：Class 2（评估器/benchmark 机制；未触 §6 受限面 kernel/bus/sequencer/typed_tx/wallet/cas-schema）。
> **治理义务**：OBL-018（`OBLIGATIONS.md`），状态 `in_progress`。
> **仓库**：`/Users/zephryj/work/turingosv4` @ `claude/p1-realvalue`。⚠ 机器上有 ≥2 个 clone（另有 `~/Developer/turingosv4-port`）；审计请 pin 路径。

---

## 0. 一键复现命令（审计师几分钟核心复核）

```bash
cd /Users/zephryj/work/turingosv4
git rev-parse --show-toplevel      # 必须 = /Users/zephryj/work/turingosv4

# 判决层（lean_judge）+ 探针修复
cargo test --bin het_capability_probe -- --test-threads=1          # 15 passed（含 realign/IP1/IP2/truncation/think-tag + 真 Lean 决定性）
cargo test --test het_probe_pool_reference_bodies_verify -- --test-threads=1   # E1 正控 6/6 Verified+axiom-clean
cargo test --test het_third_bug_dealign_decisive -- --test-threads=1           # 第3-bug judge-boundary witness（绿=记录边界）
cargo test --test lean_judge_realign_regression -- --test-threads=1            # 纯 realign 回归（无 Lean，CI 可跑）
cargo test --test lean_market_agent_shares_dealign_bug -- --test-threads=1     # D1：生产路径同 bug + realign 治愈

# H-HET-1 载体（lean_market_agent）
cargo check --bin lean_market_agent
cargo test  --bin lean_market_agent -- --test-threads=1            # 26/26（含全部新机制 gate）
bash scripts/run_constitution_gates.sh 2>&1 | tail -3              # total=166 failed=3（仅 3 个已知 pre-existing 红，零新红）

# 关键源码
grep -n 'pub fn realign\|fn opens_nested_block\|pub fn dedent' src/judges/lean_judge.rs
grep -n 'fn is_truncated\|const THINK_TAGS\|fn strip_think_tags' src/bin/het_capability_probe.rs
grep -n 'fn build_autonomous_decision_prompt\|Policy::AutonomousMarket\|chosen_action\|fn call_micro_usd' src/bin/lean_market_agent.rs src/market_tape_shared.rs

# Q2 历史复核工具 + 数据
ls handover/evidence/het_probe_pilot_2026-06-14/ handover/evidence/het_probe_pilot_smoke_2026-06-14/
cat /tmp/q2_flip.json   # （会话临时；数据已抄入本文 §7）
```

---

## 1. 执行摘要（TL;DR 裁决表）

| # | 工作 | 裁决 | 证据强度 |
|---|---|---|---|
| 起点 | 接手交接：het 探针"0 跨厂破解 / 6 道 never-solved" | **该结论不可信** | 见下 §2、§7、§9 |
| 门0 | dedent 去对齐 bug 根因 | **交接 §5 配方双重错误；正确修复在提取层（realign）** | 真跑 |
| §11.1 | 对抗式第4-bug hunt | **15 种真实输出格式全过真 Lean，无残留去对齐 bug** | 真跑 |
| 门1 | 截断检测 + think-tag 硬化 | 完成 | 真跑 14/14 |
| 门2 | 非思考体制 | **架构上正确（非妥协）** | §9 架构纠正背书 |
| D1 | 生产 `lean_market_agent` 是否同 bug | **是，真测确证** | 真 Lean |
| Q1 | de-align 修复上移 lib + 修生产 | 完成，§9 审计 **NO-VIOLATION** | 真跑+审计 |
| Q2 | de-align 历史假阴性影响 | **反直觉：暴露 ~5% 但实际 flip=0/110；谐 harness 正控 3/3 背书** | 真跑+正控 |
| 门5 探针 pilot | 非思考裸探针校准 | **裸探针是错框架；regime binds 但测错对象** | 真跑 |
| **架构纠正** | 反奥利奥架构 | **真 H-HET-1 载体=全系统异质自主市场,非裸探针** | 架构师定夺 |
| **载体 build** | 异质模型+价格注入+自主选角 | **建成+verify+§9 审计 NO-VIOLATION** | 真跑+审计 |
| **H-HET-1 答案** | 异质性能否点活 | **未取得——载体就绪,实验未跑** | 待运行 |

---

## 2. 起点与 ground-truth 核实

接手的夜间交接（`handover/H_HET_1_DEBUG_HANDOFF_2026-06-14.md`）声称：在 het 能力探针上，跨厂异质模型 0 破解、6 道定理 never-solved。fable-5 工作法第一刀不是接受结论，而是先用真跑核实（"real test beats review"）。

- **进程态**：PID 79070 已死；无残留 probe/market 进程。
- **冒烟枪**（`handover/evidence/het_probe_v4_3recs/records.jsonl`）：`lm_det_zero` attempt 0/1/2 的 note 均 `…:10:50: error: unsolved goals`，attempt 3 `16:6 unexpected token`——与我故意 de-align 的复现逐字吻合（4 中 3 带去对齐签名）。
- **E1 正控真跑**（`cargo test --test het_probe_pool_reference_bodies_verify`，独立 59s）：6 个 bank reference_body 全 `Verified` + 公理白名单内 `{Classical.choice, Quot.sound, propext}`。→ **judge 主路径对 well-formed 证明体是干净的**。

---

## 3. 门0 — dedent 去对齐 bug：交接配方错了

### 3.1 机制（真跑确认）
`cargo test --test het_third_bug_dealign_decisive`：同一已知好证明，uniform 缩进 → `Verified`；首行变浅（first-line-shallow）→ `Failed`（`unsolved goals`）。两臂同内容、仅首行缩进差 → 判决翻转**只能归因去对齐**。

### 3.2 交接 §5 配方的双重错误（本会话更正）
交接让我去改共享 `dedent` 为"回锚最浅列(min-indent)"。真跑追踪证伪：
1. **min-indent 对两个注入点都无效**：IP1 JSON `"simp\n  ring"`（公共前缀`""`、min-indent=0）、IP2 inline-by slice `" tac\n  tac"`（公共前缀`" "`、min-indent=1）——都治不了。
2. **把激进 reflow 放进共享 judge 不 sound**：`dedent` 被生产 `lean_market_agent` 复用；激进重排会改变真嵌套证明的结构（潜在假阳/假阴）。保守 `dedent` 是**正确设计**。

### 3.3 正确修复（提取层 realign）
源码（`src/judges/lean_judge.rs`，Q1 已上移 lib）：

```rust
// src/judges/lean_judge.rs  (pub fn realign)
pub fn realign(body: &str) -> String {
    let expanded = body.replace('\t', "  ");
    if opens_nested_block(&expanded) {
        return dedent(&expanded);          // 含嵌套 → 交保守 dedent（不重排结构）
    }
    let lines: Vec<&str> = expanded.lines().map(str::trim).collect();  // flat 序列 → flush col0
    let start = lines.iter().position(|l| !l.is_empty()).unwrap_or(0);
    let end = lines.iter().rposition(|l| !l.is_empty()).map_or(0, |i| i + 1);
    if start >= end { return String::new(); }
    lines[start..end].join("\n")
}
```

**SOUND 论据**：flat 序列 flush 不重排结构；嵌套体 defer 保守 dedent；Lean 终裁真目标 → realign 只可能治假阴性，绝不造假阳性。

### 3.4 证据
- `cargo test --bin het_capability_probe`：10→15/15（含 `realign_flushes_first_line_shallow_json_sibling`[IP1]、`realign_inline_by_first_tactic_sibling`[IP2]、`realign_preserves_genuine_nesting`、`extract_then_verify_first_line_shallow_real_lean`[真 Lean 决定性]）。
- E1 仍绿、第3-bug witness 仍绿（共享 dedent 字节未改 → 零 cascade）。
- 纯回归 `tests/lean_judge_realign_regression.rs`（无 Lean，CI 可跑）锁定行为。

---

## 4. §11.1 — 对抗式第4-bug hunt（不止单点修好）

`extraction_adversarial_formatting_variants_all_verify_real_lean`：同一已知好证明渲染成 **15 种真实模型输出格式**（uniform/flush/IP1-shallow/4space/tab/mixed-tab-space/CRLF/blank-lines/fenced-json/prose-wrapped/`<think>`-prefix/inline-by-IP2/lean-fence-full-decl/lean-fence-bare/focus-dots-nested），全部 `extract_proof_body` → 真 Lean `Verified`（1 passed，4.47s）。→ **提取/对齐路径无第 4 个去对齐 bug**。

---

## 5. 门1 — 截断检测 + think-tag 硬化

- **截断**：`LlmResponse` 捕获 `finish_reason`；新增纯函数判据：
```rust
// src/bin/het_capability_probe.rs
fn is_truncated(finish_reason: &str, completion_tokens: u32, max_tokens: u32) -> bool {
    finish_reason == "length"
        || (finish_reason.is_empty() && completion_tokens >= max_tokens)
}
```
主循环 truncated && !verified → `verdict="Truncated"`（不再悄悄算 ParseError/Failed）。
- **think-tag**：`strip_think_tags` 扩到 `THINK_TAGS = [think,thinking,thought,reasoning]` + 未闭合 opener（截断 reasoning）截到 EOF。
- 证据：`cargo test --bin het_capability_probe` 14/14（含 `is_truncated_*`、`strip_think_*`）；门0/对抗回归无破。

---

## 6. 门2 — 非思考体制（后被架构纠正确认为"正确"）

`enable_thinking:false` 统一加入请求体（`ChatRequest`），max_tokens=2048。序列化单测 `chat_request_pins_uniform_non_thinking_regime` PASS。我当时提的"非思考太弱→重开思考"的 flag 在 §9 被架构师纠正为**反方向**（见 §9）。

---

## 7. Q2 — de-align 历史假阴性影响（反直觉、实证扎实）

### 7.1 D1：生产路径确证同 bug
`tests/lean_market_agent_shares_dealign_bug.rs`（真 Lean）：复刻 `lean_market_agent` 路径（`extract_json_object`→裸 proof_body→`LeanJudge::verify`，无 realign），同一已知好证明 first-line-shallow → `is_verified=false`；col0-flush → `is_verified=true`。→ **生产 benchmark 判决层同享假阴性 de-align bug**（`src/bin/lean_market_agent.rs:1692-1719`）。Q1 已修（realign 上移 lib，生产调用之）。

### 7.2 Q2 数据（P1 laterday campaign = OBL-017 的全系统同质跑）
工具 `src/bin/het_dealign_exposure.rs`（用 lib realign/dedent 无漂移）+ `scripts/q2_extract_p1_*.py`：

| 指标 | 数值 |
|---|---|
| P1 cells / nodes | 1272 / 29289 |
| 全程 Verified | **仅 32** |
| lean-reject Failed nodes | 26921 |
| de-align 暴露（`realign(body) != dedent(body)`） | **1260 / 26921 ≈ 4.7%（下界）** |
| body_preview cap | 120（**89% 截断**→真实暴露更高） |
| 可复原（完整<120 多行）失败体 | 1059，其中 **110 exposed** |
| **真 Lean flip-rate（110 exposed 重验）** | **0 / 110**（realign 后仍全 fail） |
| **绑定正控**（known-good 体 first-line-shallow） | **3 / 3 flip，flip_rate=1.0** |
| 110 失败样本 feedback | 10/10 = `unsolved goals`（语义失败=证明本身错，非 import 伪影） |

### 7.3 结论 + 我被证伪的预期
**de-align 暴露 ~5% 但实际假阴性影响 ≈ 0**——exposed 证明本就是错的，realign 修对齐但模型战术仍不闭合目标。**绑定正控 3/3 翻转**证明 flip-harness 有判别力（0/110 非坏 harness 伪零）。→ **P1 hard 定理的 near-zero solve / directional-only 结论 STANDS，非 de-align 伪影**。影响是**难度依赖**：集中在 EASY 定理（模型能出正确证明、被去对齐误杀，如 het 夜跑的 lm_det_zero）。**我先前"暴露高→影响大"的预期被真跑证伪——real test beats review。**

> 诚实边界：0/110 是完整短体子集；~1150 截断长体未经 CAS 二进制反序列化评估（exposed-but-wrong 模式 + 难度论据使大量隐藏假阴性可能性低，但形式上未测）。het 夜跑无 raw 证明体不可复原 → H-HET-1 原结论只能靠重跑修、不能 Q2 回溯。**历史真正的污染源更可能是截断(门1)+思考体制混淆(门2)，而非 de-align。**

---

## 8. 门5 探针 pilot + regime 发现

`handover/evidence/het_probe_pilot_2026-06-14/`（84 calls，K=3，非思考@2048，4 厂）：

| 定理 | 解出情况 |
|---|---|
| lm_smoke（trivial floor） | **4 厂全解**（DeepSeek/Qwen3-32B/Qwen3.5-397B 各 3/3，GLM 1/3） |
| lm_det_zero（easy） | GLM 1/3，其余 0/3 ← **异质性信号雏形：跨厂解出同质 deepseek 解不出** |
| lm_c/lm_coeff_mul/lm_e/lm_lim1/lm_nt_cop_cubic（hard） | **全 null（0/12 each）** |

- regime **binds**（ct min12/p50 83/max2048；非思考真关）；门1 截断检测命中 6 个 Truncated。
- **关键发现**：非思考裸探针把可解带限在低难度；候选 hard 靶题在带之上 → 裸探针跑它们必 null。**但裸探针本身是错框架**（见 §9）。

---

## 9. 架构纠正（反奥利奥）— 本会话最重要的认知转折

架构师指出我没抓住系统精髓，纠正如下（我之前两次 mis-frame 被更正）：

**反奥利奥架构（白-黑-白，倒置常规黑-白-黑）**：
- **顶层白盒** = Meta AI + Predicates：透明、可治理的控制。
- **中层黑盒** = 各家异质模型，**非思考**：廉价提议者，不内部推理。
- **底层白盒** = 工具（搜索等）。

**为什么非思考是特性而非妥协**：ChainTape 本身就是**外显化的 chain-of-thought**；推理轨迹在 tape 上（白盒、可治理、可重放），内部 thinking 冗余且昂贵 → 砍掉=大效率赢。我"非思考太弱→重开 thinking"恰是反方向（想把推理塞回黑盒，拆了架构）。中层 agent 的"推理上下文"来自白盒脚手架：market 给全节点价格、librarian 广播抽象失败、工具给能力。

**理想态（宪法明定，非人为划分角色）**：agent 据价格 + prompt 角色菜单**哈耶克式自主选角**（investor/short/solver/challenger），顶层只广播信号、不派角色不给步骤。

**我之前探针框架的错误**：`het_capability_probe` 是裸中层（单发/无 market/无 librarian/无 ChainTape 迭代/无工具）→ 剥光脚手架的廉价非思考模型单发解 hard 题必败，对 H-HET-1 无意义。探针价值仅为 debug 工具（逼出 de-align bug）。**真 H-HET-1 载体=全系统异质自主市场。**

### 9.1 宪法依据（无需修宪，逐条引用）
- **Art II.2**（`constitution.md:343`）：「这种广播不对黑盒指手画脚，也不提供解决问题的具体步骤。它只广播信号」；黑盒「自发调整行为倾向」。
- **Art II.2.1**（:349-356）：价格广播须平衡探索/利用——既引导方向「又不能抹杀群体异质性」。
- **Art II.1**（:317-328）：典型错误**抽象后**广播全员（剪枝）；「顶层白盒绝不能把具体报错日志群发给所有人」。
- **Art III.3**（:399-409）：「只有当个体样本相互独立时，群体统计信号才具有收敛的数学意义」；否则「一万个黑盒的智慧，退化为一个黑盒的智慧」→ 顶层**刻意屏蔽横向相关、强制异质**。（这是同质化无效的宪法根据。）
- **Art III.4**：Goodhart 屏蔽——黑盒不得窥探打分算法。
- **Art 0.2**（:52-65）：tape-canonical——所有信号可从 tape 重建；失败分支以 `verified=false` 上 tape。

---

## 10. 现有市场机制盘点（mapping Workflow 结论）

| binary | 角色 | 模型 | 裁决 |
|---|---|---|---|
| `lean_hayek_market.rs`(3504) | ASSIGNED（tactic-family specialty） | 单厂 DeepSeek | 价格因果消融台，非自主市场 |
| `lean_hetero_market.rs`(329) | ASSIGNED（family round-robin） | 单厂 | 名不副实，SKIP 自分配 |
| **`lean_market_agent.rs`(2780)** | ASSIGNED（--policy Bull/Bear/Solver/Challenger） | 单模型 | **独有真机制：priced ChainTape market + CPMM/softmax + 真 librarian failure-broadcast + 真 Lean verifier + 诚实 telemetry + prompt-parity gate → 正确载体** |

三者均"派角色+同质"，违反理想；`lean_market_agent` 是要扩的载体（§17.3 亦命名其为 canonical substrate）。

---

## 11. H-HET-1 载体 build（建成 + verify + §9 审计 NO-VIOLATION）

build Workflow（11 agents；Sonnet 构机械改动、capable 构机制+审计；递归宪法检查）对 `lean_market_agent.rs`(+1239) + `market_tape_shared.rs`(+170)。架构师定夺：change#3 自主选角=**最小 2 动作**（solve[YES=WorkTx] vs short[NO=ChallengeTx]，路由现有 tx 不动不变量）。异质阵容=探针那 4 厂。

### 11.1 三个加性改动
1. **逐 agent 异质模型**：`Args.models: Vec<String>`（`--models` 逗号分隔，round-robin 到 n_agents）→ `agent_models[ai]` 接 4 个 LLM call sites；`MODEL_RATES` 补 GLM-4.5-Air / Qwen3.5-397B-A17B（含 verified_on 2026-06-14 来源）；roster 入 Manifest（0.2）。
2. **价格注入 prompt**：`build_prompt` 加 `price_context` 块（有界、signal-only），含 `broadcasts_price()` A/B 控制门防 NoPrice 偷价。
3. **自主选角**：`Policy::AutonomousMarket`——每 agent 据价格+librarian 自选 solve/short。

### 11.2 关键源码（审计师直读）
决策 prompt（无派角色、signal-only、II.2.1 防全挤、II.1 error-class-only、III.4 无 scoring leak）：
```rust
// src/bin/lean_market_agent.rs  build_autonomous_decision_prompt
"You are an autonomous agent in a Lean 4 proof-search MARKET ... \
 No role has been assigned to you. You are shown ONLY market signals — you decide for \
 yourself what to do. CHOOSE ONE of exactly two actions:\n\
 - \"solve\": ... take the LONG (YES) side ...\n\
 - \"short\": ... take the SHORT (NO) side ... You propose NO proof.\n\
 ... Be selective — do not all crowd onto the single highest-priced node ..."
// 开放节点行只给 price_yes(num/den)+conf+coarse error-CLASS（classify_lean_error，非 raw 行）
```
路由 + tape（仅现有 tx、整数 money、chosen_action 上 tape、confound-B parity 保留：advisory proof_body 不拼入）：
```rust
// short → 现有 make_real_challengetx_signed_by（不出证、不 verify），AttemptNode{chosen_action:"short", is_verified:false}
// solve → 现有 stage2_proof_prompt → judge.verify → make_real_worktx_signed_by，chosen_action:"solve"
```
cost honesty 修复（递归检查抓到的 OBL-012 真 bug）：
```rust
// src/market_tape_shared.rs  call_micro_usd — 大小写不敏感，cased slash-form 不再落 deepseek catch-all
let model_lc = model.to_ascii_lowercase();
for &(id, in_upmt, out_upmt) in MODEL_RATES { if model_lc.contains(&id.to_ascii_lowercase()) { ... break; } }
```

### 11.3 验证 + 审计
- `cargo check --workspace` 净；`cargo test --bin lean_market_agent` **26/26**（含 `autonomous_decision_prompt_is_signal_only_and_decorrelated`、`price_broadcast_gated_to_market_family_only`、`default_roster_never_resolves_to_bare_deepseek_fallback` 等新机制 gate）；constitution gates 166/仅 3 已知红零新红。
- **§9 独立清洁审计 = NO-VIOLATION**（pinned `/Users/zephryj/work/turingosv4`，逐条证伪全失败：II.2/II.2.1/II.1/III.2/III.3/III.4/0.2/§17/不变量；`lean_judge.rs +187` 正确归属前序 realign）。
  - ⚠ 首次 Workflow 审计跑错 clone（`~/Developer/turingosv4-port`）→ 空 diff → 诚实报 RECONSTRUCTION-FAILURE；已 re-run pinned 得 NO-VIOLATION。

---

## 12. 发现汇总
1. 交接 §5 修复配方双重错误（recipe 无效 + locus 不 sound）——更正后修复落提取层。
2. de-align bug 真实（机制证实），但**历史假阴性影响 ≈0**（P1 可复原子集 flip 0/110，正控背书）；影响难度依赖（咬 easy 题）。
3. 历史真正污染源更可能是**截断 + 思考体制混淆**，非 de-align。
4. **非思考是架构正确**（ChainTape=外显 CoT）；同质化无效有宪法根据（III.3）。
5. 裸探针是**框架错误**；capability 来自系统（market+librarian+price+工具），非裸模型。
6. 三个现有市场 binary 全"派角色+同质"，违宪；`lean_market_agent` 是正确载体。
7. 已现**异质性信号雏形**（GLM 解 lm_det_zero、deepseek 不解）——但在裸探针下、弱信号，须全系统真跑定论。
8. H-HET-1 载体建成 + verify + §9 审计通过（NO-VIOLATION）。

---

## 13. 问题 / 诚实边界
1. het 夜跑无 raw 证明体 → 原结论不可回溯，只能重跑。
2. Q2 的 0/110 仅覆盖完整短体；~1150 截断长体未测（需 CAS 二进制反序列化）。
3. 3 个 pre-existing constitution-gate 红（**他 session 漂移，非本工作**）：`constitution_obligation_repair_reconciliation`（OBL-004 headline 未更）、`constitution_production_module_liveness`（het_calibration_probe[BearTriage]+het_capability_probe[本探针] 未登记 liveness group）、`constitution_script_liveness_inventory`（BearTriage untracked 脚本）。
4. 机器有 ≥2 clone → agent 易跑错仓；须 pin 路径。
5. 载体 + 全部本会话改动**未提交**，在长期 dirty 多 session 树上。
6. 非思考可能把可测带限在 easy 题（hard 题异质性能否测出，须真跑看系统是否抬升解率）。

---

## 14. 待共同研究与判断（决策项）
1. **真 H-HET-1 实验启动**（最大、不可逆、花钱：4 厂 × 深度定理 × 轮 × agent，多小时）：是否现在跑？先 prereg + 绑定 pilot（难度 span 防全 null）。
2. **靶题集 / Goldilocks 带**：全系统市场（多轮+librarian+重试）可达难度高于裸探针；需校准选"deepseek 失败但系统可解"的带，否则全 null 不可判别。
3. **基线**：是否采同质自主市场作对照（架构师判断同质行业实践无效，倾向直接异质）？
4. **载体提交策略**：dirty 树上如何 commit/PR 这个 Class-2 生产机制改动（含其余多 session 未提交项）。
5. **生产其余历史役（G0/G1/G2）**：是否复核 de-align 影响，还是前向修正即可（P1 已为主结论：影响 ≈0）。
6. **4 角色全量（BuyYes/BuyNo）**：当前 2 动作（solve/short）核心已建；是否后续扩到纯 invest/short 市场下注。
7. **3 个 pre-existing 红**：OBL-004 headline 一行修复 + het 探针 liveness 登记，归谁/何时清。

---

## 15. 产物 / 证据地图
- 源码：`src/judges/lean_judge.rs`（realign/opens_nested_block/dedent，pub）、`src/bin/het_capability_probe.rs`（探针+is_truncated+strip_think_tags+env override）、`src/bin/lean_market_agent.rs`（载体：异质+价格+AutonomousMarket）、`src/market_tape_shared.rs`（MODEL_RATES+call_micro_usd）、`src/bin/het_dealign_exposure.rs`（Q2 工具）。
- 测试：`tests/het_probe_pool_reference_bodies_verify.rs`(E1)、`tests/het_third_bug_dealign_decisive.rs`(judge boundary)、`tests/lean_judge_realign_regression.rs`(纯回归)、`tests/lean_market_agent_shares_dealign_bug.rs`(D1)、+ 两 bin 的 `#[cfg(test)]`。
- 脚本：`scripts/q2_extract_p1_failed_bodies.py`、`scripts/q2_extract_p1_verify_set.py`、`scripts/q2_positive_control.py`。
- evidence：`handover/evidence/het_probe_pilot_2026-06-14/`(K=3 pilot 84rec)、`het_probe_pilot_smoke_2026-06-14/`(regime smoke)、`het_probe_calib_sweep_2026-06-14/`(134 partial，killed)、`het_probe_v4_3recs/`(冒烟枪)、`p1_v4_laterday_full_2026-06-03/`(Q2 源数据)。
- 治理：`OBLIGATIONS.md` OBL-018；交接 `handover/H_HET_1_DEBUG_HANDOFF_2026-06-14.md`。

---

*生成：2026-06-14。本文件自含；数字均来自上述真跑命令，可独立复现。无 PROVEN/causal 头条；H-HET-1 科学结论待实验。*
