# TB 施工 CHARTER — G0 单实例宪法市场激活

> 类型：Tracer-Bullet 实现 charter（buildable, cold-start-ready）
> 目标门：**G0**（v4 §7）— 单实例宪法市场激活，证明 priced-DAG 市场"活着"（11 条件，resolve=0 也算成功）。**不证能力。**
> 仓库：`/Users/zephryj/work/turingosv4` · 基线 HEAD：`1f00012d`(main) · 分支：`claude/swebench-agi-benchmark`
> 设计来源：`handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md`(v4)
> §8 授权：`handover/directives/2026-05-29_MARKET_ACTIVATION_M0_M2_M3_SECTION8_PACKET.md`（M0/M2/M3 全 SIGNED 2026-05-29 by gretjia）
> 撰写依据：本 charter 内所有 file:line 已对照 `1f00012d` 现场核实（见各 atom"现场校正"框）。

## 前置着陆检查（已执行，记录在此）

```
=== CONSTITUTION LANDING CHECK (2026-05-29) ===
1. AMBER row count:        0 active（grep 命中 40 处 🟡 均为 "was 🟡 AMBER → now GREEN" 历史注释 + legend 散文；
                            awk 按状态列首字符判定：0 行 current-status 以 🟡/🔴 起头，78 行以 🟢 起头）
2. RED row count:          0 active（11 处 🔴 命中同理为 legend + kill-condition 描述文字）
3. 开放 PR:                gh pr list --state open = 空（无路径冲突；§4.1 check PASS）
4. 分支状态:               claude/swebench-agi-benchmark = main+3（全 docs/gov，无 src 触碰），0 behind
5. Anti-pattern match:     none —— OBL-010(Level=must, in_progress) 即本 G0 campaign 自身，是当前活跃义务，
                            charter 工作不是 mode regression（它是 OBL-010 的直接推进）
VERDICT: PROCEED with charter
已知前向缺口（不阻塞 G0，显式登记）:
  - Art. 0.4 CAS strict-Merkle 重设计 → 前向绑定 Stage A3.6（matrix:37, 门级 GREEN, B.4 KNOWN-GAP）
  - PromptCapsule evaluator wire-up → 前向绑定 post-Polymarket（matrix:68, C.5 PARTIAL-S）
  - Art. 0.4 宪法文本漂移（文本说"未决"，代码已 Path B git2）→ 独立 Class 4 修宪 task，本 campaign 不修宪
```

---

## 1. 范围与 FC trace

### 1.1 G0 建什么（一句话）

造**最小激活层**，把已建好的宪法市场原语（EconomicState/CPMM/YES-NO/price_index/boltzmann-parent-select/Bull-Bear 角色强制）跑起来，对**1 个 hermetic SWE-bench 实例**、**5-10 个角色分化 agent**、用**真 Docker 结算单一 sealed 候选**，满足 v4 §7 的 11 条 G0 通过条件。**resolve=0 通过。**

### 1.2 G0 涵盖的 atom（v4 §8 顺序的子集）

| Atom | G0 是否需要 | 在 G0 的角色 |
|---|---|---|
| **M7** | ✅ | 确定性 diff materializer / 共享 output adapter（所有臂共用一个 parser，条件 #10 候选 patch 物化的前置） |
| **M5** | ✅ | SWE-bench issue→task-market + 三层屏蔽 + target-file 上下文（条件 #1/#9） |
| **M4** | ✅ | roster 扩到 N（preseed 5-10 钱包/keypair）（条件 #2） |
| **M1** | ✅ | 角色装配进运行循环（Solver/Bull/Bear）（条件 #3） |
| **M0** | ✅ | N-agent 并发编排器（条件 #2/#4/#5/#6/#7/#8 的发动机） |
| **minimal-M2** | ✅ | 真 Docker 结算**单一** sealed 候选（条件 #10/#11）。**G0 仅需 settlement（产出 verdict 落 CAS），不需 full payout** |
| **M3** | ❌ G0 不需要 | payout/redeem。**见 §1.6 论证：11 条件不含 payout** |
| M6 | ❌ G0 不需要 | 指标/报告/坐标 lockfile（G1+ 才需 scale curve） |

### 1.3 逐 atom FC 节点 / Risk class / §8 状态

| Atom | FC1 节点 | FC2/FC3 节点 | Risk class | §8 状态 |
|---|---|---|---|---|
| **M7** | FC1-N? (output adapter，rtool→input 解析) | — | **Class 1**（纯 additive helper，新 module wrap 两个既有 parser） | 无需 §8（Class 1） |
| **M5** | FC1-N5 (rtool，UniverseSnapshot 屏蔽)、FC1-N6 (input bundle，SwebenchSampleInput 结构性屏蔽) | — | **Class 2**（prompt 构造 + 屏蔽结构，无 §6 触碰） | 无需 §8（Class 2，走常规 clean-context 审计） |
| **M4** | — (genesis preseed) | FC2-N19? (boot roster) | **Class 2**（genesis 模板 + preseed 扩展，无 sequencer admission） | 无需 §8（Class 2） |
| **M1** | FC1-N10 (a_o，role-action 路由) | — | **Class 2**（仅填 `agent_role_assignment` + 复用既有 `route_role_action`，不改角色 admission）。**§8 packet §5 明示：若仅填 vec + 复用 real5_roles 路由 = Class 2 无需 §8；若须改基于角色的 admission 则升 Class 3 补签** | 无需 §8（前提：不改角色 admission） |
| **M0** | FC1-N7 (delta/AI，每 agent 1 LLM call)、FC1-N9 (q_o，parent_tx 引用)、FC1-N10 (a_o)、FC1-N13 (wtool，submit_agent_tx)、FC1-N14 (Q_{t+1}) | FC2 map-reduce tick（N 并发 = "map"） | **Class 3 + Class-4 trip-wire** | ✅ SIGNED（packet §2/§7，trigger 句已签；trip-wire 命中即停重签） |
| **minimal-M2** | FC1-N11 (predicate bundle admission)、FC1-N12 (executable predicate verification，主触点) | FC2-N22 (HALT，OMEGA settlement) | **Class 4** | ✅ SIGNED（packet §3/§7，知悉 predicate registry / settlement admission 级改动） |

**FC1 不变量（最硬门，全 atom 适用）**：
```
evaluator_reported_completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```
LHS **不得**用 `evaluator_reported_tx_count`（被非-LLM admin scaffold tx 充胀）。M0 下 N agent 各自计数须正确聚合。失败即 HALT（CLAUDE.md §4；OBS_TB18R_INV1_NONLLM_TX_2026-05-07）。

### 1.4 §6 受限面触及矩阵

| §6 surface | M7 | M5 | M4 | M1 | M0 | min-M2 |
|---|---|---|---|---|---|---|
| `src/state/sequencer.rs` admission | — | — | — | — | **仅调用** `submit_agent_tx`，不改 admission | **仅接入** `verify_proof` 到既有 `verify_predicate_claim`(:1310)，不改 admission 语义 |
| `src/state/typed_tx.rs` 判别式 | — | — | — | — | 不改（用既有 Work/Verify/BuyWithCoinRouter/TaskOpen/EscrowLock） | 不改（用既有 `PredicateProofKind::ReExecute`） |
| `src/bus.rs` 串行 reactor | — | — | — | — | 遵守 V3L-11，不改 | — |
| `src/bottom_white/cas/schema.rs` ObjectType | — | — | — | — | — | **OPEN（OQ1）**：须确认是否有可复用 ObjectType 承载 SwebenchVerdictCapsule；若须新增变体 = Class 4 trip-wire，停 |
| `predicates/registry.rs` | — | — | — | — | — | **新增** `BootPredicateKind::SwebenchDocker`（v8→v9，不动既有 5 谓词） |

### 1.5 section-6 surfaces（核实锚点，对照 `1f00012d`）

- M0 编排：`run_chaintape_driver` 单串行消费者 `src/runtime/mod.rs:1071`；`submit_agent_tx` `src/state/sequencer.rs:5180`；`tb8_await_state_root_advance` `src/runtime/adapter.rs:591`；`boltzmann_select_parent_v2` `src/sdk/actor.rs:46`；`route_role_action` `src/runtime/real5_roles.rs:605`；`AgentRole` enum `real5_roles.rs:29`。
- min-M2 谓词：`v8_production` `registry.rs:607`（**现场核实**：5 谓词，仅 `acc1`=`[Acceptance]`，其余 4 个 `[]`，Settlement bundle 空）；`load_replay_registry` `predicate_registry_loader.rs:9`（**现场核实**：调 `v8_production()`）；`verify_work_predicates` `sequencer.rs:1225`；`verify_predicate_claim` `sequencer.rs:1310`；`SwebenchTestJudge::verdict` `swebench_test_judge.rs:159`（resolved 检查 :288）。
- M5 屏蔽：`SwebenchSampleInput` `cmd_tdma.rs:37`（6 字段结构性屏蔽）；`SWEBENCH_SYSTEM_PROMPT` `cmd_tdma.rs:252`；`make_swebench_user_prompt` `cmd_tdma.rs:259`；价格广播 `prompt.rs:91`。
- M4/M1 roster：`POLYMARKET_WORKER_IDS` `cmd_generate.rs:111`（**现场核实**：`[&str;3]`）；`agent_role_assignment: vec![]` **`cmd_generate.rs:2397`（现场核实，紧邻 `role_assignment_manifest_cid: None` :2398）**；`role_assignment_from_csv` `real5_roles.rs:175`；`write_role_assignment_manifest_to_cas` `real5_roles.rs:214`。

### 1.6 为什么 G0 不需要 M3（payout）— 论证

逐条对照 v4 §7 的 11 条件：

| # | 条件 | 需要 payout? |
|---|---|---|
| 1 | genesis→初始化 task market/wallets/roster | ❌ M4+stage A/C |
| 2 | ≥5 agent 参与 | ❌ M0+M4 |
| 3 | ≥3 角色 Solver/Bull/Bear | ❌ M1 |
| 4 | 非线性 DAG branching_factor>1 | ❌ M0 (parent_tx) |
| 5 | ≥1 agent 选非最新节点作 parent | ❌ M0 (boltzmann) |
| 6 | ≥1 次 YES/NO 双边交易（各 ≥1） | ❌ M0 (BuyWithCoinRouterTx) |
| 7 | ≥1 节点价格显著变化 | ❌ CPMM 自动（交易触发 k=yes×no 重定价） |
| 8 | price/wallet/parent/boltzmann 可从 tape 重建 | ❌ tape canonicity（已有） |
| 9 | hidden tests 未进任何 prompt | ❌ M5 屏蔽 |
| 10 | 最终只选 1 个 sealed candidate 跑真 Docker settlement | ✅ **min-M2 settlement** |
| 11 | settlement 结果写 tape/CAS，报告仅派生 | ✅ **min-M2 CAS 落地** |

**结论**：11 条全部由 M7/M5/M4/M1/M0/**min-M2** 满足。条件 #10/#11 只要求 **settlement**（跑 Docker + verdict 落 CAS），**不要求 payout/redeem**。M3（赎回赢家份额→Coin、FinalizeReward、reputation）在 G0 之后再做。**唯一例外**：若实测发现 settlement 谓词的 admission 链要求 WorkTx 先经 FinalizeReward 才能"落地为 OMEGA"（见 min-M2 OQ7：WorkTx settlement 谓词失败→不入 stakes_t→VerifyTx 报 TargetWorkTxNotFound，**该链是上游天然 gate，不需额外 payout**），则确认无需 M3。**G0 需 settlement，不需 full payout——此为本 charter 的明确范围决策。**

---

## 2. 施工顺序与依赖

### 2.1 序列（低风险 additive 先行 → Class 升级 → 跑门）

```
[1] M7  确定性 diff materializer / 共享 output adapter   (Class 1, 无 §6)
        ↓ unblocks: 单一候选 patch 的确定性物化（条件 #10 前置）；所有臂 parser parity
[2] M5  SWE-bench task adapter + 三层屏蔽 + target-file    (Class 2, 无 §6)
        ↓ unblocks: agent prompt 合法构造（条件 #9 屏蔽）；issue→task-market
[3] M4  roster preseed N agents                           (Class 2, 无 §6)
        ↓ unblocks: ≥5 agent keypair/wallet 存在（条件 #2）；M0 的 agent 池来源
[4] M1  角色装配进运行循环                                 (Class 2, 不改 admission)
        ↓ unblocks: Solver/Bull/Bear 角色绑定（条件 #3）；M0 的 role mix 来源
─────────── cargo-gate #1（M7+M5+M4+M1 落地后必须全绿）───────────
[5] M0  N-agent 并发编排器                                 (Class 3 + trip-wire, §8 SIGNED)
        ↓ unblocks: 条件 #2/#4/#5/#6/#7/#8（市场动力学发动机）
─────────── cargo-gate #2（M0 落地后必须全绿 + FC1 不变量）───────────
[6] min-M2  真 Docker 结算单一 sealed 候选                 (Class 4, §8 SIGNED)
        ↓ unblocks: 条件 #10/#11（settlement verdict 落 CAS）
─────────── cargo-gate #3（min-M2 落地后全绿 + matrix_drift）───────────
[7] G0 真跑（§4 运行计划）→ 11 条件逐条产出+测量+取证
─────────── G0 真跑后：clean-context 审计（min-M2/M0 = Class 4/3 PRE-ship）───────────
[8] M3（payout）—— G0 之后，独立 atom，不在本 G0 charter ship 范围
```

### 2.2 为什么是这个序

- **M7→M5→M4→M1 先行**：全 Class 1-2 additive，无 §6 触碰，把 G0 的"输入面"（parser/prompt/roster/role）建好且独立可测。**任一失败不污染 sequencer**，是最低风险的起点。
- **M1 在 M0 之前**：M0 的 `NAgentOrchestrationConfig.agent_roles` 直接消费 M1 产出的 `Vec<AgentRoleAssignment>`。角色未装配，M0 无法做 Bull=BuyYes/Bear=BuyNo 路由（条件 #3+#6）。
- **M0 在 min-M2 之前**：min-M2 结算的"候选 patch"必须由 M0 跑出来的市场选出（最高 price_yes 路径终点）。无市场 = 无候选 = settlement 无对象。
- **min-M2 最后**：它是唯一 Class 4 + 唯一改 predicate registry，风险最高，且依赖前面所有产出。最后做使得前面的 cargo-gate 已稳定，min-M2 的回归边界最小。

### 2.3 哪些步骤需要 cargo-gate 反馈才能进下一步

| gate | 触发点 | 必须全绿命令 | 不绿则 |
|---|---|---|---|
| **gate #1** | M1 完成后、动 M0 前 | `cargo test --workspace --no-fail-fast` + `bash scripts/run_constitution_gates.sh` + 各 atom 新单测 | 停，修 M7/M5/M4/M1，**不得**进 M0 |
| **gate #2** | M0 完成后、动 min-M2 前 | gate #1 全部 + `cargo test --test n_agent_concurrent_orchestrator` + **FC1 不变量等式**（dry-run/stub 上验证聚合计数） | 停，修 M0；**trip-wire 命中即回 §8** |
| **gate #3** | min-M2 完成后、跑 G0 前 | gate #2 全部 + `cargo test --test constitution_swebench_docker_predicate` + `cargo test --test constitution_matrix_drift` + `cargo test --test constitution_predicate_registry_immutability` | 停，修 min-M2 |

**强制**：M0 是并发对 sequencer 的交互，min-M2 改 admission gate 形状——**这两步必须各自拿到 cargo-gate 反馈（绿）才能进下一步**。前 4 步（M7-M1）可在 gate #1 处一次性验证（它们互不依赖 sequencer）。

---

## 3. 逐 atom 施工规格

> 通用：所有 file:line 对照 `1f00012d`。新模块/脚本**先注册 liveness**（§7）。整数货币、守恒、tape canonicity、屏蔽、additive-only、串行 apply、FC1——全 atom 红线。

---

### 3.1 M7 — 确定性 diff materializer / 共享 output adapter（Class 1）

**精确 additive 编辑点**
1. 新文件 `src/judges/shared_output_adapter.rs`，并在 `src/judges/mod.rs` 末尾加 `pub mod shared_output_adapter;`（additive 行）。
2. `src/tdma_runner.rs:175`（`AnyJudge::verdict` Swebench 臂，现 :237-260 内联 extract_body+extract_patch）改为调 `parse_tdma_output`，verdict 逻辑不变。
3. `swebench_live_coding_repair_current_kernel.rs` 的 patch 提取路径**范围确认（OQ4）**：该 standalone 二进制有自己的 full-field `SweBenchSample`（含 gold/test，归档 runner），M7 parity **仅作用于 `AnyJudge`/`run_proof` 路径**，是否同步改 standalone 由实现者按 OQ4 决定（默认：不改 standalone，它是 archival）。

**新接口/类型**
```rust
// src/judges/shared_output_adapter.rs（全 additive）
pub struct ParsedOutput {
    pub header: StateUpdate,          // 来自 state_update.rs 既有 parser
    pub body: String,                 // extract_body 结果（tdma_runner.rs:426）
    pub body_sha256: String,          // sha256_hex(body.as_bytes())
    pub schema_version: &'static str, // = "tdma-state-update/v1"
}
pub fn parse_tdma_output(raw: &str) -> Result<ParsedOutput, HeaderParseError>;
pub const SCHEMA_VERSION: &str = "tdma-state-update/v1"; // re-export state_update.rs:31
```

**不变量**
- **tape canonicity / schema pin**：`"tdma-state-update/v1"`（state_update.rs:31）单一 pin；任一臂 schema_version 不符即 `HeaderParseError::SchemaInvalid`，body 消费前拒绝。
- **FC1 attempt-count（OQ5）**：`parse_tdma_output` 是**纯函数**，无副作用，**不**碰 `SwebenchTestJudge::attempt`（swebench_test_judge.rs:51 的 `Cell<usize>`）。`AnyJudge::verdict` 每 LLM cycle 仍只调一次 `verdict`（tdma_runner.rs:241）——adapter 不得改变此调用计数（防双计）。

**单测 + 集成测**（新建，inline `#[cfg(test)]` 或 `tests/shared_output_adapter.rs`）
- `test_parse_tdma_output_roundtrip`：合法 header+`---BODY---`+JSON body → `schema_version=="tdma-state-update/v1"`，body 非空，body_sha256 匹配。
- `test_parse_tdma_output_schema_mismatch_errors`：`v2` header → `Err(SchemaInvalid)`。
- `test_shared_adapter_parity_nesbitt_swebench`：Nesbitt-style 与 Swebench-style 响应同 `schema_version` 常量。
- `test_version_hash_constant`：`SCHEMA_VERSION == "tdma-state-update/v1"`（版本漂移回归守卫）。

**predicate-GREEN 配方**
```bash
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
# 全 exit 0
```

**trip-wire**：若须改 `src/state_update.rs` 的 `StateStatus` 判别式或 `StateUpdate` 字段（如新增 `status: Swebench`）= Class 4 wire-schema 变更，**停，重签 §8**。M7 只 wrap，不加判别式。

---

### 3.2 M5 — SWE-bench task adapter + 三层屏蔽 + target-file 上下文（Class 2）

**精确 additive 编辑点**
1. `cmd_tdma.rs:46` 后加字段 `#[serde(default)] target_files: Vec<String>` 到 `SwebenchSampleInput`（serde(default) 保后向兼容）。
2. `cmd_tdma.rs:280` 后加 `fn make_target_file_context(repo, base_commit, target_files) -> String`。**读 base_commit 处的源文件（`git show <commit>:<path>`），排除 test 路径（test_*、*_test.py、tests/**、conftest.py），拼接 `=== <path> ===` 块，封顶 ~4000 字符。**
3. `make_swebench_user_prompt`（cmd_tdma.rs:259）内联扩展（或 v2），在 `=== Target File Context (base_commit) ===` 头下注入 context，注入点 cmd_tdma.rs:273（`{hints}` 之后）。
4. `SwebenchTaskMarketAdapter` 加到 `swebench_test_judge.rs:52` 后，wrap `SwebenchTestJudge` + task-market event ID，暴露 `adapt(&JudgeVerdict) -> TaskMarketSignal`（纯 additive，不改既有 `impl MathStepJudge`）。

**三层屏蔽（结构性 + prompt-literal 双层强制）**
```
Tier 1 价格全广播：  build_agent_prompt（prompt.rs:91）"=== Market ===" 块，不改
Tier 2 摘要选择披露：problem_statement + fail_to_pass 测试 NAME only + target-file content @ base_commit
                     —— 由 SwebenchSampleInput 6 字段构造排除 gold/test 强制
Tier 3 严格屏蔽：    gold_patch / test_patch / raw harness stderr / hidden test source
                     —— SwebenchSampleInput 字段缺失（编译期保证）+ SWEBENCH_SYSTEM_PROMPT 明文禁止
                     + SwebenchTestJudge::verdict 只回 failing test NAMES（swebench_test_judge.rs:306）
```

**不变量**
- **屏蔽（FC1-N5/N6, Art. III）**：`SwebenchSampleInput` **无** gold_patch/test_patch 字段——serde 静默丢弃（编译期不可加，结构性盾）。target-file 读 **base_commit**（git object，非 HEAD、非 test-patch 状态），结构排除测试 oracle 修改。
- **target-file 来源（OQ1，承重未决）**：`make_swebench_user_prompt` 现**无 checked-out repo 访问**，只有 sample JSON（repo 名 + base_commit SHA）。要 `git show` 需本地 clone 或预取 tarball——现 `cmd_tdma.rs` 无 `--swebench-repo-dir` flag。**实现者动手前必须决定**：(a) 加 `--swebench-repo-dir` flag，或 (b) 从 swebench Docker 镜像派生（harness 原生方式）。函数签名随此决定变化。

**单测**
- `test_make_swebench_user_prompt_no_gold_leak`：result 不含 "gold_patch"/"test_patch"/LEAK_SENTINEL。
- `test_target_file_context_excludes_test_paths`：含 `src/foo.py`，不含 `tests/test_foo.py`。
- `test_3tier_shield_tier3_fields_absent`：`serde_json::to_value` 默认 struct，断言无 "gold_patch"/"test_patch"/"test_source"/"harness_stderr" 键。

**predicate-GREEN 配方**
```bash
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
rg -n "gold_patch|test_patch" src/bin/turingos/cmd_tdma.rs   # 期望 0 匹配（结构屏蔽检查）
```

**trip-wire**：① 任何 `SwebenchSampleInput` 加 gold/test 字段 = 屏蔽破。② target-file 读需网络且被 `SanitizedCommand` allowlist（swebench_test_judge.rs:225，`HF_HUB_OFFLINE=1`）拦——**不得绕 hermetic env**，绕过 = 屏蔽违规，停。

---

### 3.3 M4 — roster preseed N agents（Class 2）

**精确 additive 编辑点**
1. `cmd_init.rs:105` `GENESIS_MULTI_AGENT` 模板：line 155 后（闭合 `"#` 前）加新 `[[agents]]` 条目激活 N>3 roster（additive 内容，不触 sequencer）。
2. `cmd_generate.rs:115` 后加 `fn polymarket_worker_ids_from_preseed(preseed: &[(AgentId, MicroCoin)]) -> Vec<String>`（返回所有 worker-* preseed key，按 AgentId 排序，不再硬限 3 元数组）。既有 `POLYMARKET_WORKER_IDS`(:111) 保留向后兼容。
3. `cmd_generate.rs:2883` `polymarket_workers_for_preseed` 后加 `build_role_assignment_from_preseed(preseed, role_csv) -> Vec<AgentRoleAssignment>`（调既有 `role_assignment_from_csv` real5_roles.rs:175），结果在 :2397 前赋给 `agent_role_assignment`（与 M1 衔接）。

**新接口**
```rust
fn polymarket_worker_ids_from_preseed(preseed: &[(AgentId, MicroCoin)]) -> Vec<String>;
fn build_role_assignment_from_preseed(preseed: &[(AgentId, MicroCoin)], role_csv: Option<&str>) -> Vec<AgentRoleAssignment>;
```

**不变量**
- **整数货币**：`initial_balances`/`risk_budget_micro` 全 `MicroCoin`(i64)。`role_assignment_from_csv` 设 `risk_budget_micro: MicroCoin::from_micro_units(100_000)`（real5_roles.rs:206，纯整数）。`GENESIS_MULTI_AGENT` 注释 "1 Coin = 1_000_000 micro"（cmd_init.rs:111）为真相源注。
- **守恒**：sum(invest_coin_micro) ≤ sum(agent_balances)；既有 `StakeInsufficient`/`InsufficientBalance` admission gate 逐 tx 强制，无新机制。
- **Observer 隔离**：不给无 preseed 余额的 agent 分角色——`build_role_assignment_from_preseed` 对 preseed 缺席的 agent 返回空，不捏造。

**单测**
- `test_polymarket_worker_ids_from_preseed_n5`：5 worker（alpha-epsilon）全返回（N>3 扩展验证）。
- `test_polymarket_worker_ids_from_preseed_excludes_treasury`：treasury/verifier-alpha 不在结果。

**predicate-GREEN 配方**：`cargo test --workspace --no-fail-fast` + `bash scripts/run_constitution_gates.sh` exit 0。

**trip-wire**：若 N>3 须新 `AgentRole` 变体或改 sequencer admission = 升 Class，停。M4 只 preseed + 调度。

---

### 3.4 M1 — 角色装配进运行循环（Class 2）

**精确 additive 编辑点**
1. `real5_roles.rs:212` 后加 `pub fn build_role_assignment_from_genesis(genesis_text: &str) -> Result<Vec<AgentRoleAssignment>, String>`（调既有 `parse_treasury_and_worker_preseed` + `role_assignment_from_csv` :175；无 `[role_assignments]` 表时返回 `Ok(vec![])` 保后向兼容）。
2. `cmd_generate.rs:2397`（**现场核实**：`agent_role_assignment: vec![]`，紧邻 `role_assignment_manifest_cid: None` :2398）—— :2397 前调 `build_role_assignment_from_genesis` 赋给该字段。`GenesisReport` struct 字段已存在，无需加字段。
3. `write_role_assignment_manifest_to_cas`（real5_roles.rs:214）从 genesis bootstrap 路径 additive 调用，填 `role_assignment_manifest_cid`（解 OQ3：确认 :2401 `write_to_runtime_repo` 前 CasStore 已开）。

**不变量**
- **tape canonicity**：`GenesisReport`（含 `agent_role_assignment`）经 `report.write_to_runtime_repo`(cmd_generate.rs:2401) 写 ChainTape-backed runtime repo；角色 manifest 经 `write_role_assignment_manifest_to_cas` 落 CAS。`vec![]` 占位是 M1 填的缺口；填后该字段在 `write_to_runtime_repo` 前已 populate，tape 记录权威。
- **role-action 强制（FC1-N10）**：`route_role_action`(real5_roles.rs:605) 是唯一强制 gate，返回 `L4E{PolicyViolation}` 对不允许的 (role,action)。M1 **只 wire preseed→assignment，绝不改** `match (role,action)` 判别式表（real5_roles.rs:611-626）。强行加 arm = Class 4 判别式改，trip-wire。

**单测**
- `test_build_role_assignment_from_genesis_empty_returns_empty_vec`：无 `[role_assignments]` 表 → `Ok(vec![])`（后向兼容）。
- `test_build_role_assignment_from_genesis_csv`：preseed worker-{alpha,beta,gamma} + `role_assignments_csv="Solver,Verifier,Trader"` → 三 assignment 匹配，且 `route_role_action` 对各角色正典 action 返回 `L4`(admit)。
- `test_route_role_action_solver_cannot_invest`（既有，纳入回归）：`route_role_action(Solver, Invest{..})` → `L4E{PolicyViolation}`。

**predicate-GREEN 配方**：`cargo test --workspace --no-fail-fast` + `bash scripts/run_constitution_gates.sh` exit 0。

**trip-wire**：须改基于角色的 sequencer admission = 升 Class 3 补签（§8 packet §5 明示）。须加 `AgentRole` 变体（CAS-serialized manifest 共享 enum）= Class 4，停。

---

### 3.5 M0 — N-agent 并发编排器（Class 3 + Class-4 trip-wire；§8 SIGNED）

**现场校正（对照 `1f00012d`，覆盖 surface map 的 line 漂移）**
> surface map 称 `pub mod concurrent_orchestrator;` 加在 `src/runtime/mod.rs` "约 line 166"——**核实有误**：`src/runtime/mod.rs` 最后一批 `pub mod` 声明在 ~line 328-379（`real5_roles` 在 :338）。新声明应加在 runtime mod 声明区（建议紧邻 :338 `real5_roles` 或末尾 :379），**不是 166**。
> `swebench_live_coding_repair_current_kernel.rs` **不在 Cargo.toml**（从 `src/bin/` 自动发现）——新 bin 同理无需 `[[bin]]` 条目，但**仍须在 liveness manifest 加 `bin::<name>` module_id**（见 §7）。

**精确 additive 编辑点**
1. 新文件 `src/runtime/concurrent_orchestrator.rs`；`src/runtime/mod.rs` runtime mod 声明区加 `pub mod concurrent_orchestrator;`（additive，**约 :338-379 区，非 166**）。
2. （可选）新 bin `src/bin/n_agent_concurrent_orchestrator.rs`（自动发现，无需 Cargo.toml 改；须注册 liveness）。
3. 新模块仅含 `pub async fn run_n_agent_orchestration(...)` + 支撑 struct，**不改任何既有 type/function**。

**新接口/类型（全 additive）**
```rust
pub struct NAgentOrchestrationConfig {
    pub agent_roles: Vec<(AgentId, AgentRole)>,   // N agent + 角色（来自 M1）
    pub task_id: TaskId, pub sponsor_agent: AgentId,
    pub sequencer: Arc<Sequencer>,                 // 从 ChaintapeBundle.sequencer clone
    pub keypairs: AgentKeypairRegistry,            // mut：生成 N keypair
    pub cas_path: PathBuf, pub llm_proxy_url: String, pub model: String,
    pub task_escrow_micro: i64, pub work_stake_micro: i64, pub invest_coin_micro: i64,
    pub boltzmann_policy: BoltzmannMaskPolicy,
}
pub struct NAgentOrchestrationReport {
    pub submitted_work_tx_ids: Vec<(AgentId, TxId)>,
    pub submitted_invest_tx_ids: Vec<(AgentId, TxId)>,
    pub submitted_verify_tx_ids: Vec<(AgentId, TxId)>,
    pub errors: Vec<(AgentId, String)>,
}
```
**调用的既有 API（全不改）**：`submit_agent_tx`(sequencer.rs:5180)、`make_real_{task_open,escrow_lock,worktx,verifytx}_signed_by`(adapter.rs)、`tb8_await_state_root_advance`(adapter.rs:591)、`boltzmann_select_parent_v2`(actor.rs:46)、`route_role_action`(real5_roles.rs:605)、`build_for_evaluator_append_with_parent`(proposal_telemetry.rs:217)、`set_agent_pubkeys`(sequencer.rs:5142, OnceLock)、`BuyWithCoinRouterTx`(typed_tx.rs:1605, `BuyDirection::{BuyYes,BuyNo}`)。

**不变量（须证立）**
1. **串行 apply（最硬）**：`apply_one` 由单一 `run_chaintape_driver`(mod.rs:1071) 独占消费 mpsc，逐 envelope 处理；Stage 9 commit 在 `self.q` write lock(sequencer.rs:7053)。并发 `submit_agent_tx` 只 `try_send`(多生产者安全)，driver 单消费者。**并发只在 LLM 层，sequencer FIFO-arrival 不变。**
2. **StaleParent 处理**：每个经济 tx 带 `parent_state_root`，apply_one dispatch 检查 `== q.state_root_t`（WorkTx 在 sequencer.rs:1862）。N agent 并发 → 除首个外全 StaleParent 落 L4.E（**正确行为，非违规**）。编排器读 `q_snapshot().state_root` → 签 → `tb8_await_state_root_advance` 捕获新 root 供下一 tx；优雅处理 SubmitError/StaleParent + 可选 refresh 重试。
3. **守恒**：每 BuyWithCoinRouterTx 在 dispatch 扣 `pay_coin`(sequencer.rs:4333 9-step atomic)；sum(invest) ≤ sum(balances)；既有 admission gate 强制，无新机制。
4. **屏蔽（shielding）**：`route_role_action`(real5_roles.rs:605) 强制 Bull→BuyYes only、Bear→BuyNo only；编排器构造 BuyWithCoinRouterTx 前映射 role→BuyDirection。
5. **FC1 attempt-count**：每 `build_for_evaluator_append_with_parent` 对应**一次 LLM 响应**，不是每 tx；非-LLM admin scaffold（TaskOpen/EscrowLock）**不增** LLM call 计数。N agent 各自计数聚合须满足等式。
6. **tape canonicity**：ProposalTelemetry.parent_tx 设为该 agent 分支最近 accepted WorkTx 的 TxId（首次 None）；编排器追踪 per-agent `last_accepted_work_tx_id`。
7. **OnceLock manifest**：`set_agent_pubkeys`(sequencer.rs:5142) 单次 OnceLock；N keypair 全生成后**一次性**设全 N agent manifest；已设则 `Err(existing)`，须检查并明确处理（OQ6）。

**单测（`tests/n_agent_concurrent_orchestrator.rs`）**
1. `test_n_agent_concurrent_submit_all_roles_no_panic`：5 agent [Solver,Bull,Bear,Verifier,Trader]，LLM stub，全入队不 panic，守恒不变。
2. `test_stale_parent_retry_succeeds`：同 pre-root 两 agent，一成功、一 StaleParent，refresh 重试成功。
3. `test_bull_trader_direction_gating`：`route_role_action(BullTrader, Invest{side:No})` → `L4E{PolicyViolation}`；只 BuyYes 时提交。
4. `test_bear_trader_direction_gating`：同 BearTrader→BuyNo。
5. `test_boltzmann_parent_selection_reads_price_index`：3 节点 price_index，每 agent 返回非 None parent。
6. `test_serial_apply_invariant`：N 并发 tokio task 提交同 sequencer，logical_t 单调，最终 state_root 字节等于 replay 期望。
7. `test_fc1_attempt_count_equality`：N=3 各 1 LLM call → CAS ProposalTelemetry == 3，`completed_llm_calls == sum(step+parse_fail+llm_err)`。
8. `test_agent_pubkey_manifest_set_once`：设一次全 N；提交不在 manifest 的 agent WorkTx → `SubmitError::AgentSignatureInvalid`。

**predicate-GREEN 配方**
```bash
cargo test -p turingosv4 --test n_agent_concurrent_orchestrator -- --nocapture   # 8 测全 exit 0
cargo test --workspace --no-fail-fast
bash scripts/run_constitution_gates.sh
# 全 exit 0
git diff src/state/sequencer.rs | grep -iE 'admission|discriminant'   # 期望空
```

**Class-4 trip-wire（命中即停，回 §8 packet 加签）**
1. 改 `sequencer.rs` 任一 `dispatch_transition` arm / `apply_one` stage / `TransitionError` 变体。
2. 改 `typed_tx.rs` 任一既有 TypedTx 判别式或新增变体。
3. 改 `typed_tx.rs` 签名 payload struct / `canonical_digest()`。
4. 须并发调用 `bus.rs` `TuringBus::append`（违 V3L-11）。
5. 改 `agent_keypairs.rs` on-disk manifest 格式或引入多写并发。
6. StaleParent 重试要求 pre-empt/re-order sequencer 队列。

**OPEN（实现者动手前须定）**
- OQ1（**承重**）：parent 选择 per-agent 还是共享 snapshot？影响分支多样性。
- OQ2：`AgentKeypairRegistry` 非 Send；in-flight 签名用 `Arc<Mutex<...>>`（推荐）。
- OQ3：1 task / N solver 还是 N task？须验证 WorkTx admission 对同 task 多 WorkTx 是否首个后拒（查 `tasks_t[task_id]` state machine, sequencer.rs:1862+）。
- OQ4：BuyWithCoinRouterTx.event_id 须指既有活跃 CPMM 池（sequencer.rs:3994）——市场由编排器 seed 还是外部 MarketMaker 预 seed？
- OQ5：VerifyTx.target 须先 accepted WorkTx；编排器须 `tb8_await_state_root_advance` + q_snapshot 等 WorkTx apply 后再提交 VerifyTx。

---

### 3.6 minimal-M2 — 真 Docker 结算单一 sealed 候选（Class 4；§8 SIGNED）

**精确 additive 编辑点**
1. `registry.rs:704`（`LeanArtifact` 后）`BootPredicateKind` 加 `SwebenchDocker { instance_id: String, dataset_name: String }`。
2. `registry.rs:1010` 后加 `struct SwebenchDockerPredicate` + `impl Predicate`。
3. `registry.rs:1051` 后 `predicate_impl_from_spec` match 加 `SwebenchDocker` arm。
4. `registry.rs:655` 后加 `pub fn v9_production() -> Self { let mut m = Self::v8_production(); m.entries.push(BootPredicateSpec::new("swebench_docker_v1", BootPredicateKind::SwebenchDocker{instance_id:String::new(), dataset_name:String::new()}, [PredicateBundleMap::Settlement])); m }`。
5. `predicate_registry_loader.rs:10`：`v8_production()` → `v9_production()`（**现场核实**：现 :10 调 `v8_production()`，:9 是 `pub fn load_replay_registry`）。
6. 新 CAS-verdict capsule 类型（新文件 `swebench_verdict_capsule.rs` 或 inline）。

**新接口/类型**
```rust
impl Predicate for SwebenchDockerPredicate {
    fn predicate_id(&self) -> &str { &self.id }
    fn code_hash(&self) -> [u8;32] { self.code_hash }
    fn evaluate(&self, _ctx) -> BoolWithProof { BoolWithProof{value:false, proof_cid:None} } // 非结算路径，保守
    fn verify_proof(&self, ctx, claim) -> Result<bool, PredicateVerifyError> {
        // 1. decode_and_validate_proof_capsule(ctx, .., PredicateProofKind::ReExecute, claim)  // registry.rs:1056
        // 2. 从 capsule.proof_result_cid 读 SwebenchVerdictCapsule
        // 3. 绑定检查：verdict.work_context_hash == ctx.work.context_hash(registry_root)；predicate_code_hash == self.code_hash
        // 4. 屏蔽：只暴露 resolved bool + failing test NAMES
        Ok(verdict.resolved)
    }
}
pub struct SwebenchVerdictCapsule {
    pub schema_id: String,               // "turingos-swebench-verdict-capsule-v1"
    pub instance_id: String, pub dataset_name: String,
    pub resolved: bool,                  // report.json[instance_id]["resolved"]
    pub failing_test_names: Vec<String>, // 屏蔽：只名字，无内容
    pub work_context_hash: Hash, pub predicate_code_hash: [u8;32],
}
```
**复用既有（不改）**：`PredicateProofKind::ReExecute`（registry.rs:119，harness 即 re-execution）、`PredicateProofCapsule.proof_result_cid`（registry.rs:125）、`decode_and_validate_proof_capsule`(registry.rs:1056)、`verify_predicate_claim`(sequencer.rs:1310)、`verify_work_predicates`(sequencer.rs:1225)。**无新 TypedTx 变体**（verdict 走 WorkTx.predicate_results.settlement 既有字段）。

**Runner 侧生产者**：`SwebenchTestJudge::verdict`(swebench_test_judge.rs:159) 收到 `JudgeVerdict::Pass` 后：(a) 构 `SwebenchVerdictCapsule{resolved:true,..}` 入 CAS；(b) 构 `PredicateProofCapsule{proof_kind:ReExecute, proof_result_cid:<verdict_cid>}` 入 CAS；(c) CID 放 WorkTx.predicate_results.settlement 的 `BoolWithProof{value:true, proof_cid:Some(cap_cid)}`。

**不变量**
1. **屏蔽**：capsule 只 `resolved:bool` + `failing_test_names`（名字，镜像 swebench_test_judge.rs:292-310 屏蔽）；report.json 字节/gold/test_patch 永不入 CAS；verify_proof 只读 resolved。
2. **verdict 可从 CAS 重建**：proof_result_cid→SwebenchVerdictCapsule 在 CAS；work_context_hash 绑特定 WorkView；replay 经 verify_work_predicates→verify_predicate_claim→verify_proof 确定性重执绑定检查。
3. **OMEGA gated on 真 resolved=true**：verify_predicate_claim(sequencer.rs:1335-1349) 调 verify_proof 后查 `recomputed != claim.value` → `SettlementPredicateProofMismatch`。WorkTx 声明 value=true 但 CAS capsule resolved≠true → admission 失败。**无自我宣称旁路**：WorkTx settlement 谓词失败→不入 stakes_t→VerifyTx 报 TargetWorkTxNotFound（上游天然 gate, OQ7）。
4. **串行 apply / tape canonicity**：verify_work_predicates 在 dispatch_transition(sequencer.rs:1866) 内、apply_one 内（单线程）。新 verify_proof **无 I/O**（只读传入 proof_store=tx-time CAS snapshot）。Docker 执行在 WorkTx 构造时由 runner 做，**不在 verify_proof 内**。
5. **predicate registry 不变性**：5 个 v8 谓词不动；v9 调 v8 + append 一条。`production_replay_paths_use_shared_registry_loader` + `sequencer_consumes_registry_by_shared_reference` 继续过。
6. **整数守恒**：SwebenchDockerPredicate 不碰 money 路径；FinalizeReward(sequencer.rs:2416) 用硬编码 PredicateId 串，不用 registry，货币不变量不受影响。
7. **FC1 attempt-count**：verify_proof 纯 CAS-read，不推进 `SwebenchTestJudge::attempt`（只在 runner 侧 verdict() 增）。

**单测（registry.rs `#[cfg(test)]` + `tests/constitution_swebench_docker_predicate.rs`）**
1. `swebench_docker_predicate_verify_proof_pass`：capsule resolved:true + matching claim → `Ok(true)`。
2. `swebench_docker_predicate_verify_proof_resolved_false_fails`：resolved:false → `Ok(false)`（→ verify_predicate_claim 在 claim.value=true 时返 `SettlementPredicateProofMismatch`）。
3. `swebench_docker_predicate_verify_proof_wrong_context_hash` → `Err(ContextHashMismatch)`。
4. `swebench_docker_predicate_verify_proof_missing_proof_cid`：claim.proof_cid=None → `Err(MissingProofCid)`。
5. `swebench_docker_predicate_shielding_no_test_content_in_capsule`：serde round-trip 确认只有期望字段（无 test_patch/gold 承载字段）。
6. `v9_production_manifest_has_swebench_docker_in_settlement_bundle`：`required_predicates(Settlement).contains("swebench_docker_v1")`；`required_predicates(Acceptance)` 不含；既有 5 谓词名不变。
7. `v9_production_registry_merkle_root_differs_from_v8`：v8 root ≠ v9 root（确保新条目真改 Merkle root）。
8. `swebench_docker_predicate_round_trip`：canonical encode/decode 幂等。
9. 宪法门测（`tests/constitution_swebench_docker_predicate.rs`）：v9 注册正确；verify_proof 拒 `BoolWithProof{value:true, proof_cid:None}`；Settlement bundle `required_settlement.len()==1`。

**predicate-GREEN 配方**
```bash
cargo test --workspace --no-fail-fast
cargo test --test constitution_swebench_docker_predicate
cargo test --test constitution_matrix_drift
cargo test --test constitution_predicate_registry_immutability
bash scripts/run_constitution_gates.sh
# 全 exit 0
git diff src/top_white/predicates/registry.rs   # 既有 5 谓词 (acc1/forbidden/sorry_free/size/lean) 块无改动，仅新增
```

**Class-4 trip-wire**
1. 若任何下游强制新 TypedTx 变体（如独立 SwebenchSettleTx）= wire-schema 级，停，重签。
2. **OQ1（最关键）**：若 `bottom_white/cas/schema.rs` `ObjectType` 无可复用变体（generic Blob/StructuredData）承载 SwebenchVerdictCapsule，须新增 ObjectType 判别式 = **Class 4 restricted surface**（§6），**停，重签 §8**。**实现者动手前必须先查 ObjectType 是否有通用变体。**
3. **OQ2/Tripwire4**：`instance_id`/`dataset_name` 嵌入 `BootPredicateKind` 会让每实例改 code_hash（破单-binary-impl 不变量）。**resolution**：SwebenchDocker 变体在 BootPredicateKind 带空/sentinel 串（code_hash 只从 kind 形状导），真 per-WorkTx instance_id 从 WorkTx proposal CAS payload 在 verify_proof 内提取。若现 PredicateContext API 不暴露 task_id/instance 元数据，须 PredicateWorkView 加字段或 capsule 加字段——**绝不可改 WorkTx wire schema**，先报架构师。

**OPEN（动手前须定）**
- OQ1（ObjectType，**阻塞**）：见 trip-wire 2。
- OQ2（instance_id 提取，**阻塞**）：见 trip-wire 4，推荐"嵌 capsule + 与 WorkTx proposal payload 交叉核对"。
- OQ3：runner 在哪取 CAS store handle 写 capsule（cmd_tdma.rs CAS 写访问须审）。
- OQ5（**G0 关键**）：Docker 返 resolved=false 是否作合法 sealed settlement（非 error），写 resolved=false 的 PredicateProofCapsule？**G0 要求**：resolve=0 通过，故 verdict()/verify_proof 须正确处理 resolved=false 为合法 verdict（CAS 须含完整 verdict 记录）。
- Tripwire5：4 个测试文件直接引 `v8_production()`——**采用 option (b)**：v8_production 原样保留，v9_production 新建，只 load_replay_registry 切 v9；测试 fixture 显式用 v8 或 v9（最安全，不破既有测试）。

---

## 4. G0 运行计划

### 4.1 单实例运行 config

| 项 | 值 | 依据 |
|---|---|---|
| **协议** | Standard-clean（隐藏测试不进 prompt/retry/搜索；sealed 只跑一次 Docker） | v4 §2 |
| **instance 选取** | 1 个 **hermetic** SWE-bench 实例（django/sympy/sphinx/pytest/xarray/pylint 白名单之一；**requests 非 hermetic 须排除**）。建议 **django** 单实例（最稳 hermetic Docker） | v4 §6 hermetic 白名单 |
| **agents** | **6 个**（落在 5-10 区间，留冗余防 StaleParent 损耗） | 条件 #2 ≥5 |
| **role mix** | 2 Solver + 2 Bull + 2 Bear（≥3 角色，含双边交易方）。Verifier 视 M0 OQ5 决定是否加 | 条件 #3/#6 |
| **tx budget** | 15 tx/agent（落 10-20 区间），总 ~90 tx | v4 §7（10-20 tx/agent） |
| **model** | `deepseek-v4-flash`，显式 `{"thinking":{"type":"disabled"}}`（默认 thinking-ON, LiteLLM #27453） | v4 §6 |
| **market seed** | 单一 CPMM 池 seed（MarketSeedTx + CpmmPoolTx）；由 sponsor/MarketMaker 预 seed（M0 OQ4 定） | Stage C |
| **settlement** | 市场选最高 price_yes 路径终点 patch → **单次** 真 Docker（SwebenchTestJudge::verdict）→ verdict 落 CAS | min-M2 |

### 4.2 11 条 G0 条件：产出 + 测量 + 取证（全部 tape/CAS）

| # | 条件 | 如何产出 | 如何测量 | 证据路径（tape/CAS） |
|---|---|---|---|---|
| 1 | genesis→init task market/wallets/roster | Stage A genesis + M4 preseed 6 钱包 + Stage C TaskOpen/EscrowLock/MarketSeed | genesis_report.json 含 6 wallet + task_market 条目 | runtime_repo ChainTape L4 genesis 段；`genesis_report.json` |
| 2 | ≥5 agent 参与 | M0 并发 6 agent 各提交 ≥1 WorkTx 经 submit_agent_tx | tape 上 distinct signer (AgentId) ≥5 的 WorkTx | L4 WorkTx 记录的 signer pubkey；`NAgentOrchestrationReport.submitted_work_tx_ids` |
| 3 | ≥3 角色 Solver/Bull/Bear | M1 角色装配 + M0 按角色路由 | role_assignment_manifest 含 ≥3 distinct role；tx 行为符角色 | CAS role_assignment_manifest_cid；L4 BuyYes(Bull)/BuyNo(Bear)/WorkTx(Solver) |
| 4 | 非线性 DAG branching_factor>1 | M0 多 agent 出 WorkTx 带不同 parent_tx | dag_view BFS：某节点 ≥2 子（branching_factor>1） | ProposalTelemetry.parent_tx (CAS)；`dag_view.rs` BFS 派生 |
| 5 | ≥1 agent 选非最新节点作 parent | M0 boltzmann_select_parent_v2 选历史节点（非 tip） | ∃ WorkTx.parent_tx ≠ 该时刻最新 accepted WorkTx | parent_tx vs logical_t 序列比对（tape 派生） |
| 6 | ≥1 次 YES/NO 双边交易（各 ≥1） | M0 Bull 出 BuyYes + Bear 出 BuyNo | tape 上 ≥1 BuyDirection::BuyYes + ≥1 BuyNo 落账 | L4 BuyWithCoinRouterTx 记录 direction 字段 |
| 7 | ≥1 节点价格显著变化 | 交易触发 CPMM k=yes×no 重定价（sequencer.rs:4010） | price_index 派生：某节点 price_yes 前后 Δ 超阈值 | cpmm_pools_t / node_positions_t (tape) → compute_price_index 派生 |
| 8 | price/wallet/parent/boltzmann 可从 tape 重建 | 全信号落 typed tx / CAS | replay verifier 从 genesis tape+CAS 重建 price_index/balances/parent 链字节一致 | `verify_chaintape` bin replay；tape_root/cas_root |
| 9 | hidden tests 未进任何 prompt | M5 三层屏蔽（SwebenchSampleInput 6 字段 + SWEBENCH_SYSTEM_PROMPT） | grep 全 search-phase prompt（CAS ProposalTelemetry payload）= 0 gold/test/FAIL_TO_PASS 名匹配 | CAS ProposalTelemetry prompt 字节 grep |
| 10 | 最终只选 1 sealed candidate 跑真 Docker | min-M2：选最高 price_yes 终点 → 单次 SwebenchTestJudge::verdict | settlement 调用计数 == 1；Docker 真跑（report.json 存在） | CAS SwebenchVerdictCapsule（resolved 字段，可为 false） |
| 11 | settlement 结果写 tape/CAS，报告仅派生 | min-M2 PredicateProofCapsule + SwebenchVerdictCapsule 入 CAS | verdict capsule 在 CAS 可寻址；报告含 tape_root/cas_root/pricing_table_sha256 | CAS PredicateProofCapsule.proof_result_cid → SwebenchVerdictCapsule |

**横贯门**：FC1 不变量 `evaluator_reported_completed_llm_calls = step+parse_fail+llm_err`——G0 真跑后必须成立，失败即 HALT（不继续、不审计为 pass）。**resolve=0 通过 G0**（条件 #10 只要求"跑了真 Docker 且 verdict 落 CAS"，不要求 resolved=true）。

---

## 5. 门与验证

### 5.1 必须保持 GREEN 的门命令

```bash
# 横贯（每 atom + G0 前）
cargo check
cargo test --workspace --no-fail-fast            # ship-level，非 bare cargo test
bash scripts/run_constitution_gates.sh           # 2026-05-27 报 [k-1-5] total=165 failed=0；任一回归=blocker
cargo test --test constitution_matrix_drift       # K-2.3 漂移门；allowlist cap K23_SHIP_ALLOWLIST_SIZE=67

# min-M2 专属（Class 4）
cargo test --test constitution_predicate_registry_immutability
cargo test --test constitution_swebench_docker_predicate

# M0 专属
cargo test -p turingosv4 --test n_agent_concurrent_orchestrator

# liveness（新模块/脚本注册后）
cargo test --test constitution_production_module_liveness
cargo test --test constitution_script_liveness_inventory
```

### 5.2 FC1 不变量（最硬门）

```
evaluator_reported_completed_llm_calls = tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```
LHS 不得用 tx_count（被非-LLM admin scaffold 充胀）。M0 下 N agent 计数正确聚合。**失败即 HALT。**

### 5.3 G0 pass 标准

```
G0 PASS ⟺
  (11 条件全满足，逐条有 tape/CAS 证据，见 §4.2)
  ∧ (FC1 不变量等式成立)
  ∧ (cargo test --workspace + run_constitution_gates.sh + constitution_matrix_drift 全 exit 0)
  ∧ (min-M2/M0 clean-context 审计 verdict ∈ {NO-VIOLATION} ∧ obligation witness == OBL-ALL-CLOSED 或 OBL-010 satisfied)
  ∧ (resolve 可为 0)
```

---

## 6. clean-context 审计 packet 骨架（AGENTS.md §9）

> Class 4（min-M2）= **PRE-ship 审计**；Class 3（M0）= 审计（witness, one independent）。审计员**不给实现 transcript**。下面骨架在实现+真跑后填空，交新审计员（任何 capable 平台）。

```markdown
## CLEAN-CONTEXT AUDIT PACKET — G0 Market Activation (min-M2 / M0)

### 1. Task brief
激活宪法 priced-DAG 市场，对 1 hermetic SWE-bench 实例、6 agent、真 Docker 结算单一 sealed 候选，
满足 v4 §7 的 11 条 G0 通过条件（resolve=0 通过）。本 packet 覆盖 min-M2(真验证器结算谓词)
+ M0(N-agent 编排器)。

### 2. Risk class
- min-M2: Class 4（predicate registry + settlement admission）。§8 SIGNED 2026-05-29(gretjia)。
- M0:     Class 3 + Class-4 trip-wire。§8 SIGNED 2026-05-29(gretjia)。
- (M7/M5/M4/M1: Class 1-2，附带说明，非本 packet 焦点)

### 3. Touched FC nodes / invariants
- min-M2: FC1-N12(executable predicate verification, 主)、FC1-N11(predicate bundle admission)、
          FC2-N22(HALT)；不变量：屏蔽、verdict 可重建、OMEGA gated on 真 resolved、串行 apply、
          registry 不变性、FC1 attempt-count。
- M0:     FC1-N7/N9/N10/N13/N14、FC2 map-reduce tick；不变量：串行 apply、守恒、StaleParent、
          屏蔽(Bull/Bear direction)、FC1 attempt-count 聚合。

### 4. Diff / commit
git diff main --name-only  →  [填实际：src/runtime/concurrent_orchestrator.rs,
  src/runtime/mod.rs(+1 pub mod), src/top_white/predicates/registry.rs(+SwebenchDocker),
  src/runtime/predicate_registry_loader.rs(v8→v9), tests/..., tests/fixtures/liveness/*.toml]
引用 §8 directive: handover/directives/2026-05-29_MARKET_ACTIVATION_M0_M2_M3_SECTION8_PACKET.md

### 5. Evidence paths（tape/CAS，无 sidecar）
- runtime_repo ChainTape L4/L4.E（genesis + N WorkTx + BuyYes/BuyNo + settlement）
- CAS：ProposalTelemetry(token_counts/parent_tx)、SwebenchVerdictCapsule、PredicateProofCapsule、
       role_assignment_manifest
- genesis_report.json（6 wallet + role assignment + manifest CID）
- tape_root / cas_root / pricing_table_sha256

### 6. Exact verification command output（填真实输出）
$ cargo test --workspace --no-fail-fast         → [exit 0, test result 行]
$ bash scripts/run_constitution_gates.sh        → [k-1-5 total=N failed=0]
$ cargo test --test constitution_matrix_drift   → [exit 0]
$ cargo test --test constitution_predicate_registry_immutability → [exit 0]
$ cargo test --test constitution_swebench_docker_predicate       → [exit 0]
$ FC1 invariant: completed_llm_calls=X, step+parse_fail+llm_err=X → [相等]
$ git diff src/state/sequencer.rs | grep -iE 'admission|discriminant' → [空]
$ git diff registry.rs 既有 5 谓词块 → [无改动]
$ rg gold_patch|test_patch <search-phase prompt CAS dump> → [0 匹配]

### 7. Required verdict format
clean-context audit: { NO-VIOLATION | VIOLATION-FOUND <clause> <file>:<line>
                     | RECONSTRUCTION-FAILURE <path> | SECOND-SOURCE-DRIFT <view> }
obligation witness:  { OBL-ALL-CLOSED | OBL-OPEN-MUST <id> | OBL-EVIDENCE-MISSING <id>
                     | OBL-BLOCKER-UNVERIFIED <id> }
（ship gate = predicates GREEN ∧ audit verdict ≠ unresolved violation ∧ obligation witness 非 open-must）
```

---

## 7. liveness 登记清单

> **liveness gate 用 `fs::read_dir` + `pub mod` 扫描——新模块/脚本未注册即 PANIC。先注册、干净 worktree 验证（#212 教训）。**

**现场核实的真路径**（覆盖 surface map 的笼统说法）：
- 模块 manifest：**`tests/fixtures/liveness/production_module_liveness.toml`**（**不是** repo-root）。测试 `tests/constitution_production_module_liveness.rs`：(1) `declared_source_files()` 走 `src/lib.rs`/`src/main.rs` 的 `pub mod` 链——新 `.rs` 须经 `pub mod` 可达；(2) `every_exported_module_has_exactly_one_liveness_group` 要求每 declared `module_id` 在该 TOML 恰好一个 `[[group]]` 的 `module_ids` 数组出现。
- 脚本 manifest：**`tests/fixtures/liveness/script_liveness_inventory.toml`**（测试 `constitution_script_liveness_inventory.rs:11`）。
- bins 从 `src/bin/` **自动发现**（`swebench_live_coding_repair_current_kernel` **不在 Cargo.toml**，现场核实）——新 bin 无需 `[[bin]]`，但须在 liveness manifest 加 `bin::<name>` module_id（与既有 `bin::swebench_live_coding_repair_current_kernel` 同组）。

**须注册的新模块/脚本**

| 新增 | module_id（mirror 既有命名） | 注册到 |
|---|---|---|
| `src/runtime/concurrent_orchestrator.rs` | `runtime::concurrent_orchestrator` | `production_module_liveness.toml` 持有 runtime:: 的 catch-all `[[group]]`（现 `runtime`/`runtime::adapter`/... 那组，~line 324+），加进 `module_ids` |
| `src/judges/shared_output_adapter.rs` | `judges::shared_output_adapter` | 同 manifest，judges:: 所在 group |
| `src/top_white/predicates/swebench_verdict_capsule.rs`（若拆文件） | `top_white::predicates::swebench_verdict_capsule` | 同 manifest 对应 group |
| `src/bin/n_agent_concurrent_orchestrator.rs`（若建 bin） | `bin::n_agent_concurrent_orchestrator` | 同 manifest，bins 所在 group（与 `bin::swebench_live_coding_repair_current_kernel` 同组，~line 310-324） |
| 任何新 `scripts/*.sh`（如 G0 run 脚本） | 按 inventory 格式 | `script_liveness_inventory.toml` |

**clean-worktree 验证**：注册 TOML 须**与源文件同 commit 或更早**。每次加模块后立即跑：
```bash
cargo test --test constitution_production_module_liveness
cargo test --test constitution_script_liveness_inventory
```
不绿 = 注册缺漏，补全再继续。**OQ6（M0 命名）**：实现者须先确认新模块的精确 `pub mod` 路径（如 `runtime::concurrent_orchestrator`）与测试扫描 `src/lib.rs` 的 `pub mod` 声明一致，否则 `every_exported_module_has_exactly_one_liveness_group` PANIC。

---

## 8. 风险与 trip-wire（合并）

### 8.1 Class-4 trip-wire（命中即停，回 §8 packet 加签）— 全 atom 汇总

| 编号 | 条件 | 涉及 atom |
|---|---|---|
| TW-1 | 改 sequencer.rs 任一 admission arm / apply_one stage / TransitionError 变体 | M0, min-M2 |
| TW-2 | 改/新增 typed_tx.rs 既有判别式（新增独立变体也算） | M0, min-M2, M3 |
| TW-3 | 改 typed_tx.rs 签名 payload / canonical_digest() | M0, min-M2 |
| TW-4 | 并发调 bus.rs TuringBus::append（违 V3L-11 串行） | M0 |
| TW-5 | 改 agent_keypairs.rs on-disk manifest 格式 / 多写并发 | M0 |
| TW-6 | StaleParent 重试 pre-empt/re-order sequencer 队列 | M0 |
| **TW-7（阻塞）** | `bottom_white/cas/schema.rs` ObjectType 无可复用变体，须新增判别式 | min-M2（OQ1） |
| TW-8 | instance_id 嵌 BootPredicateKind 破单-binary code_hash；或为提取 instance 改 WorkTx wire schema | min-M2（OQ2） |
| TW-9 | 改 state_update.rs StateStatus 判别式 / StateUpdate 字段 | M7 |
| TW-10 | 加 AgentRole 变体（CAS-serialized 共享 enum） | M1, M4, M5 |
| TW-11 | M1 须改基于角色的 sequencer admission（升 Class 3 补签） | M1 |

### 8.2 AMBER-row findings（landing check）— 全 DEFERRED-FORWARD，不阻塞 G0

| finding | 状态 | 对 G0 影响 |
|---|---|---|
| Art. 0.4 CAS strict-Merkle 重设计（matrix:37, B.4） | 门级 GREEN，前向绑定 Stage A3.6 | **无**（market L4 anchor 不变；CAS 重建走 cas/.git/objects + sidecar index） |
| PromptCapsule evaluator wire-up（matrix:68, C.5 PARTIAL-S） | 门级 GREEN，前向绑定 post-Polymarket | **无**（G0 用 ProposalTelemetry token_counts，已 wired） |

### 8.3 Art.0.4 drift flag（显式）

宪法文本（constitution.md:149）说 git substrate "未决/0 git hits"，但代码已实现 Path B（`git_tape_ledger.rs` git2）。**本 G0 campaign 不修宪**——报告显式写 drift + 给机器证据（git fsck/replay/tape_root/cas_root/state_root/price 派生）。**独立 Class 4 task 修订 Art. 0.4 + amendment log（不阻塞 G0，OBLIGATIONS.md:130 已登记）。**

### 8.4 什么强制返回 §8

任一 TW-1..TW-11 命中 → 停 → 回 §8 packet 加签（该 atom 升 Class 或扩范围）。**Class 4 不得藏在 Class 3 伞下**——M0 若发现须改 admission，立即从 Class 3 升 Class 4，不得"顺手做掉"。范围外发现 → 停 → 重签。

### 8.5 其它运行期 trip-wire

- **mode regression**：禁 `charter→atom→self-audit→external audit→more docs→delayed test`。任一 atom "完成" 前须 (a) 宪法门测存在且过、(b) 最小真跑、(c) clean-context 审计返回。OBL-010(Level=must) in_progress 时不得 `done/完成/PROCEED`。
- **self-claimed OMEGA bypass**：min-M2 若 Docker 可被任一自我宣称 VerifyTx 旁路 = 非宪法。kill：OMEGA admitted without Docker resolved=true。
- **shielding breach**：hidden tests/gold/test_patch/FAIL_TO_PASS 名/raw Docker stderr 进任一 prompt/search/market view = Art. III 违宪，G0 claim 前须 grep 证据。
- **money 非守恒**（M3 才相关，G0 不触）：total_supply_micro 改变 = 无条件 HALT。
- **second-source-drift**：任一新模块持权威 price/wallet/verdict 状态于 ChainTape/CAS 之外 = SECOND-SOURCE-DRIFT，no_parallel_ledger_source_of_truth 门失败。
- **in-flight PR overlap**：landing check 已确认 `gh pr list --state open` 空——无冲突。开工前若有新 PR 出现，重跑 §4.1 check。

---

## 9. PR 计划

> **PR-only**（K-HARDEN-7）：分支 `claude/swebench-agi-benchmark`，禁 merge main；不提交 `handover/evidence/**` sidecar（no-sidecar CI）；merge 由 orchestrator 持 `GIT_HARDEN_ALLOW_MAIN=1`。每 PR title 标 risk class，body 含 acceptance-criteria 命令块 + 期望输出。

### 9.1 PR 切分（按 Class 边界 + cargo-gate 边界）

| PR | 内容 | Class | §8 引用 | 审计要求 |
|---|---|---|---|---|
| **PR-A**（低风险捆绑） | M7 + M5 + M4 + M1（全 Class 1-2，互不依赖 sequencer，gate #1 一次性验证） | Class 2（取最高） | 无（Class 1-2） | clean-context 审计（Class 2 witness，platform-agnostic）；M1 须证"未改角色 admission" |
| **PR-B** | M0（N-agent 编排器，新 module + 1 行 `pub mod` + liveness 注册） | Class 3 + trip-wire | packet §2 M0 trigger | clean-context 审计（Class 3，one independent，AFTER 真跑证据） |
| **PR-C** | min-M2（SwebenchDocker 谓词 + v8→v9 + verdict capsule + loader 1 行 + liveness） | Class 4 | packet §3 M2 trigger | clean-context 审计 **PRE-ship**（Class 4，AGENTS.md §14） |

**为何 3 PR 而非 1**：
- Class 边界对齐审计 cadence（Class 2 vs 3 vs 4 审计要求不同）。
- min-M2 单独 PR 使 Class 4 改动（predicate registry）的 `git diff main --name-only` 触 §6 surface 时**显式引用 §8 directive**（AGENTS.md §14 pre-merge checklist 要求），且既有 5 谓词"git diff = 空"的证据边界最干净。
- M0 trip-wire 若命中（升 Class 4），只影响 PR-B，不污染 PR-A/PR-C。

### 9.2 每 PR 包含

**PR-A（M7+M5+M4+M1）**
- 源：shared_output_adapter.rs、cmd_tdma.rs（target_files + make_target_file_context + prompt 注入）、swebench_test_judge.rs（SwebenchTaskMarketAdapter）、cmd_init.rs（GENESIS_MULTI_AGENT N agent）、cmd_generate.rs（polymarket_worker_ids_from_preseed + build_role_assignment_from_*）、real5_roles.rs（build_role_assignment_from_genesis）、judges/mod.rs（+pub mod）。
- 测试：M7/M5/M4/M1 全部新单测。
- liveness：`judges::shared_output_adapter` 注册。
- body：`cargo test --workspace --no-fail-fast` + `run_constitution_gates.sh` + `rg gold_patch|test_patch cmd_tdma.rs`(=0) 输出。

**PR-B（M0）**
- 源：concurrent_orchestrator.rs、runtime/mod.rs（+1 pub mod，~:338-379 区）、（可选）n_agent_concurrent_orchestrator.rs。
- 测试：`tests/n_agent_concurrent_orchestrator.rs`（8 测）。
- liveness：`runtime::concurrent_orchestrator`（+ `bin::n_agent_concurrent_orchestrator` 若建 bin）注册。
- body：8 测输出 + workspace + gates + FC1 不变量等式 + `git diff sequencer.rs | grep admission|discriminant`(=空)。
- 引用 packet §2 M0 §8 trigger。

**PR-C（min-M2）**
- 源：registry.rs（SwebenchDocker kind + struct + impl + match arm + v9_production）、predicate_registry_loader.rs（v8→v9）、swebench_verdict_capsule.rs（若拆）、runner 侧 capsule 写入（cmd_tdma.rs/judge）。
- 测试：registry inline + `tests/constitution_swebench_docker_predicate.rs`（9 测）。
- liveness：`top_white::predicates::swebench_verdict_capsule`（若拆文件）注册。
- body：9 测 + workspace + gates + matrix_drift + predicate_registry_immutability + `git diff registry.rs`(既有 5 谓词无改) 输出。
- 引用 packet §3 M2 §8 trigger（知悉 predicate registry / settlement admission 级 Class 4）。

### 9.3 no-sidecar evidence

G0 真跑证据全在 **ChainTape L4/L4.E + CAS + genesis_report.json**（runtime_repo 内），**不进** `handover/evidence/**`。报告（若产）仅派生视图，含 tape_root/cas_root/pricing_table_sha256，**禁** dashboard/sidecar 作真相源（SECOND-SOURCE-DRIFT）。G0 PASS 的取证按 §4.2 全部指向 tape/CAS 路径。

### 9.4 ship 序

```
PR-A merge（gate #1 绿 + Class 2 审计）
  → PR-B merge（gate #2 绿 + FC1 不变量 + M0 Class 3 审计 AFTER 真跑）
  → PR-C merge（gate #3 绿 + min-M2 Class 4 审计 PRE-ship）
  → G0 真跑（§4）→ 11 条件取证 → final clean-context 审计 packet（§6）→ 架构师 ship-confirm
  → M3（payout）独立 atom，G0 之后
```

---

## 附：cold-start 实现者 checklist（按序）

```
[ ] 0. 读 AGENTS.md → HARNESS_PLAYBOOK.md → constitution.md → LATEST.md；读本 charter
[ ] 0. gh pr list --state open（重确认无路径冲突）
[ ] 1. PR-A: M7（shared_output_adapter）→ 单测绿
[ ] 1. PR-A: M5（adapter + 三层屏蔽 + target-file）→ 先定 OQ1(repo 来源)→ 单测绿 + rg gold_patch=0
[ ] 1. PR-A: M4（preseed N）→ 单测绿
[ ] 1. PR-A: M1（角色装配，确认不改 admission）→ 单测绿
[ ] === gate #1: cargo test --workspace + run_constitution_gates.sh exit 0 ===
[ ] 1. PR-A liveness 注册（judges::shared_output_adapter）→ liveness 测绿 → 开 PR-A
[ ] 2. PR-B: M0 → 先定 OQ1-OQ5 → 新 module + pub mod(~:338-379, 非166) → 8 单测绿
[ ] === gate #2: gate#1 + n_agent_orchestrator 测 + FC1 不变量(stub) ===
[ ] 2. PR-B liveness 注册（runtime::concurrent_orchestrator [+bin]）→ liveness 测绿 → 开 PR-B
[ ] 3. PR-C: min-M2 → 先解 OQ1(ObjectType,阻塞) + OQ2(instance_id,阻塞) → 谓词 + v9 + capsule → 9 单测绿
[ ] === gate #3: gate#2 + swebench_docker_predicate + matrix_drift + registry_immutability ===
[ ] 3. PR-C liveness 注册 → liveness 测绿 → 开 PR-C
[ ] 4. G0 真跑（6 agent, 2Solver/2Bull/2Bear, 15tx/agent, 1 hermetic django 实例, deepseek-v4-flash thinking-disabled）
[ ] 4. 逐条产出+测量 11 条件（§4.2），全证据指向 tape/CAS
[ ] 4. 验 FC1 不变量等式（真跑后，失败即 HALT）
[ ] 5. 填 §6 审计 packet → 交 clean-context 审计员（M0=Class3 AFTER / min-M2=Class4 PRE-ship）
[ ] 6. 架构师 ship-confirm → orchestrator merge（GIT_HARDEN_ALLOW_MAIN=1）
[ ] 注：M3(payout) 不在 G0；11 条件不含 payout（§1.6 论证）
```

---

**charter 诚实声明**：以下为**真·未决**，cold-start 实现者动手前须解决，charter 不假装确定：
- **min-M2 OQ1（阻塞）**：`bottom_white/cas/schema.rs` ObjectType 是否有可复用变体承载 SwebenchVerdictCapsule。无 = Class 4 §6 surface，停重签。
- **min-M2 OQ2（阻塞）**：verify_proof 内如何取 instance_id（不可改 WorkTx wire schema）。
- **M5 OQ1（承重）**：target-file 的 repo 来源（`--swebench-repo-dir` flag vs Docker 镜像派生）——决定 `make_target_file_context` 签名。
- **M0 OQ1-OQ5**：parent 选择共享/per-agent、keypair Mutex、1-task/N-task、市场谁 seed、VerifyTx 时序。
- **min-M2 OQ5（G0 关键）**：resolved=false 是否作合法 sealed settlement 写 CAS（G0 要求 resolve=0 通过，故须正确处理）。

charter 关键文件路径（绝对）：本 charter 应落 `/Users/zephryj/work/turingosv4/handover/tracer_bullets/TB-G0_charter_2026-05-29.md`；设计源 `/Users/zephryj/work/turingosv4/handover/SWEBENCH_MARKET_ACTIVATION_BENCHMARK_v4_2026-05-29.md`；§8 packet `/Users/zephryj/work/turingosv4/handover/directives/2026-05-29_MARKET_ACTIVATION_M0_M2_M3_SECTION8_PACKET.md`；liveness manifest `/Users/zephryj/work/turingosv4/tests/fixtures/liveness/production_module_liveness.toml`。