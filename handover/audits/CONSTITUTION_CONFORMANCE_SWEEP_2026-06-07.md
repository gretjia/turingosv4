# 全宪法对抗式一致性扫描（M07-class bypass sweep）2026-06-07

**方法**：M07-class 完备性不变式（"属性 P 必须在类别 S 的*每一个*站点成立"）。失效形态 =
存在一个**并行/新增的 S 类站点**，其上 P 未被强制执行。逐站点枚举（rg 穷举所有
writer/caller/path），逐站点检查 P 是否真正强制，未强制 = 候选 bypass。**不信任
"反正有个 gate 管这件事"**——gate 往往只覆盖一个站点（这正是 M07 的幻觉）。

**审计范围**：`/home/zephryj/projects/turingosv4-conformance/src`。
M07（kernel predicate admission）在本树**已修复**，不重复上报。
所有 CONFIRMED 项均已对源码逐条 file:line 复核（false positive 摧毁信任，只列对抗式
verify 确认者）。

---

## 1. Executive Summary

本轮对 14 个宪法不变式做了穷举站点扫描。对抗式 verify 后的结论：

| 严重度 | 确认数 | 不变式 / 站点 |
|--------|--------|----------------|
| **MAJOR** | **3** | boot-trust-root（多 runner / dispatcher 入口无验证）、append-only-rubber（tdma_runner llm_err 不落 tape）、evidence-cas-anchor（market_external_agent 失败分支不写 CAS） |
| **MODERATE** | **1** | raw-diagnostic-shield（swebench judge 把原始 subprocess stderr tail 透传进 LLM 重试 prompt） |
| **LATENT（命名 guard 死代码 / 零站点）** | **1** | goodhart-shield（`assert_no_metric_leak` 零生产调用方，pinned 却 wire 到 nothing） |

**genuine M07-class bypass（真并行旁路，sibling 站点强制 P 而本站点跳过）= 3**
（append-only-rubber、evidence-cas-anchor、boot-trust-root；后者为"文档声明 every
binary launch 验证 vs 实际只有 1 个 verify-only 入口"的多站点并行缺口）。

外加 **1 个 MODERATE 真旁路**（raw-diagnostic-shield，sibling judge 全部 clean，唯
swebench 泄漏）、**1 个 LATENT 零站点**（命名 runtime guard 死代码）。

被对抗式 verify **驳回**（不计入）的候选共 12 项：dead-code 不可达
（EscrowVault、legacy bus graveyard、append_oracle_accepted）、tape-derived 视图
（PriceBroadcast、TaskMemoryStore、coverage_state）、ingress-barrier 设计边界
（submit_agent_tx pre-submit_id rejection）、未接线但有下游真权威的 capability
metadata（allowed_tools / ToolRegistry / WalletTool hook）、同 battery 内 replay
backstop 兜底（FC3 system-only assert_09）。详见 §4 与末尾驳回清单。

---

## 2. CONFIRMED bypasses（ranked）

| # | 不变式 | 站点 file:line | 为何未守住（why-unguarded） | 严重度 | sibling 是否强制 P？（真并行旁路？） |
|---|--------|----------------|------------------------------|--------|--------------------------------------|
| 1 | append-only-rubber | `src/tdma_runner.rs:594-603`（llm_call `Err` 臂 → `break 'outer`） | 该臂只 push 到内存 Vec 后 `break 'outer`，**永不调用** `kernel.step_forward_with_claims`（唯一调用在 line 655，在 `Ok` 分支判 judge 之后）。其它每一类失败（judge-reject / parse_fail / escalate）都经 judge → `step_forward_with_claims` → `handle_rejection`（`memory_kernel.rs:361-375`）落 `NodeKind::AgentProposal verified:false`。仅 llm_err 在入 kernel 前短路。生产接线：`cmd_tdma.rs:603-620`（`tdma run --tape-backend git`）以 durable `GitTapeLedger` 调 `run_proof_with_ledger`，且 closure 对 API 失败只 `.map_err(..)?`、不写 capsule → durable tape 对该 llm_err **零行**。`llm_err` 是 FC1 正典计数项（`attempt_telemetry.rs:172` `AttemptOutcome::LlmErr=4` → `RejectionClass::LlmError=9` L4.E）。 | MAJOR | **是（真并行旁路）**。同循环内每个 sibling 失败类都落 verified=false；仅 llm_err 被静默丢弃。注：这丢失只伤 evidence-completeness，不伤 state（同一臂也 `break 'outer`，verified head 不会被假推进）。`cmd_generate.rs` 的 `write_generation_attempt_capsule` 在**非-TDMA** generate 函数里；TDMA 自身 llm_call closure（`cmd_generate.rs:1206-1237`）失败不写 capsule，故 shared TDMA runner 的 llm_err 从 tape 不可数。 |
| 2 | evidence-cas-anchor | `src/bin/market_external_agent_current_kernel.rs:648-654` | `ask_external_agent`（line 646）已完成真实 LLM 调用（token 已花费）并返回。随后 `parse_decision(&agent_content)?`（648）在任何 parse/extract/amount-range 失败时 early-return Err；direction-mismatch（649-654）`return Err`。本次 attempt 的**首个 CAS 写**是 `MarketDecisionCapsule put_json`（667）——**在两个 guard 之后**。失败分支下：无 CAS object、无 L4 WorkTx、无 L4.E。run() 回 main() 仅 `eprintln!` + `ExitCode::from(1)`（stderr-only）。本 binary manifest 自声明 `full_system_participation_required=true`（1030-1031），**非** smoke 免责。 | MAJOR | **是（真并行旁路）**。同类站点（已完成 LLM 后的 parse/guard）在 sibling `swebench_live_coding_repair_current_kernel.rs:342-388` 被锚定（`parse_patch_claim` **不带** `?`，捕获 `parse_error`，代入空 `ParsedPatchClaim`，**总是**写 `SweBenchPatchClaimCapsule`，含 `parse_error` 字段，即使失败分支）；`lean_market_agent.rs:484` `Err(_) =>` 落入 sequencer-anchored 路径不 early-exit；正典 `external_call.rs` outbox 对每次 attempt 记录 Intent→Terminal{Failed/TimedOut/Abandoned}。market binary 仅在失败分支跳过 P。 |
| 3 | boot-trust-root | `src/bin/turingos.rs:336`（dispatcher `main`→`dispatch`:297）+ in-proc canonical writer `cmd_tdma.rs:603-620`；以及 ~16 个 `*_current_kernel.rs` runner（`main`→`run`→`build_chaintape_sequencer_with_initial_q`）；`fc3_governance_reinit_current_kernel.rs:144`；`turingos_web.rs:28` | `rg verify_trust_root src/` 仅命中 `src/main.rs:14`（**verify-only binary `turingosv4`，无子命令**）与 `cmd_boot.rs:66`（`boot` 子命令）。`turingos` 二进制 = `src/bin/turingos.rs`，其 `main()`(336)/`dispatch()`(297) **不验证 trust root**，却把 `tdma run` 路由到 in-process `GitTapeLedger::open/init_bare`（canonical tape writer）。`build_chaintape_sequencer_with_initial_q`（`runtime/mod.rs:724`）内**无** trust-root 验证；~16 个 runner 各自 `put_json(EvidenceCapsule)` + 提交签名 WorkTx 推进 `state_root`，全部无验证。`fc3_governance_reinit` 经 live Sequencer 发 `MapReduceTick`（最高信任面）。篡改 `constitution.md` 或 pinned-hash manifest **不会** halt 任何 runner / `turingos tdma run`。 | MAJOR | **是（文档 every-binary-launch 声明 vs 单 verify-only 站点的并行缺口）**。`art_v3_amendment_log.rs:18` + gate `constitution_tc_boot_trust_root_manifest.rs:129-140` 只断言 `main.rs`+`cmd_boot.rs` 调 verify——正是 M07 single-site 幻觉。先跑 `turingos boot` 是 operator convention，非 enforcement；无任何机制让 runner 在篡改树上拒绝启动。`turingos_web.rs` 为低严重度变体（读/投影面，写委托给已不设防的 `turingos` 后端）。 |
| 4 | raw-diagnostic-shield | `src/judges/swebench_test_judge.rs:346-350`（`harness_failure_reason` 的 `tail_chars(stderr,400)` fallback）→ `tdma_runner.rs:248-251` → `distiller.rs:149-152`（`extract_first_failed_predicate` fallback）→ `memory_kernel.rs:532,542`（BBS 序列化进重试 prompt） | 已用探针测试端到端复现：放进 `harness_failure_reason` stderr-tail 的哨兵（`RAW_PYTHON_TRACEBACK_SECRET_…`）**逐字到达** LLM 重试 prompt。链路：(1) 无 report.json 且 log 不含 "Patch Apply Failed"/"malformed patch" → reason = `swebench harness error (exit ..); stderr tail: <400 raw chars>`（line 346-350）。(2) tdma_runner `swebench_failed_predicate(reason)` 保留前 200 字含原始 tail。(3) judge Fail 使 `success==false`，跳过 Proceed 臂，命中 `memory_kernel.rs:313`→`handle_rejection`，用 LLM 自己的 header（`SWEBENCH_SYSTEM_PROMPT` 强制 `failed_predicate:null`）。(4) 故 `deterministic_trace_slicer` 回退到 `extract_first_failed_predicate(raw_stderr)`，取首个含 "predicate" 行 = 带原始 stderr tail 的那行。(5) `compress_belief_state` copy 进 `failure_signature.failed_predicate`，BBS 整体序列化进 `assemble_o1_prompt`。`leak_sentinel` guard（`tdma_runner.rs:674`）只查 blob start-marker，捕不到经 `failed_predicate` 正交通道的泄漏。 | MODERATE | **是**。sibling judge 全部 clean：`lean_judge` `shield_lean_diagnostic` 限 240 字 error line；nesbitt/putnam/generate 发确定性结构化字符串。**唯 swebench** 的 `Fail.reason` 携带无界原始 subprocess stderr tail。可达条件：non-apply 的 harness error（缺预构镜像、harness 内部异常、OOM），judge docstring 自证镜像须预构否则 harness error，是真实运维条件。界限 ~150-200 字、同循环内（非跨租户）。 |
| 5 | goodhart-shield（命名 runtime guard 死代码 / 零站点） | `src/sdk/prompt_guard.rs:50`（`assert_no_metric_leak`） | `rg assert_no_metric_leak` 全树仅命中其自身 `#[cfg(test)]` 模块（83-142）；历史生产调用方在已删除的 `experiments/minif2f_v4/src/bin/evaluator.rs`，现仅存 `lean_market.rs` 且不调本 guard。无任何 sibling 重实现打分子串扫描。静态门 `tests/constitution_shielding_gate.rs` 只覆盖 III.1 raw_stderr / III.2 private_diagnostic_cid，其 III.4 测试 grep dashboard 的 raw_stderr/private_diagnostic，**不**扫 prompt 内 PPUT/scoring 子串。该 guard **trust-root pinned**（`genesis_payload.toml:152`）却 wire 到 nothing——pinned-but-dead。 | LATENT | **否（零站点缺口，非并行旁路）**。Art.III.4 metric-leak 不变式的 RUNTIME 强制站点**真空**。当前活跃 prompt 装配路径（`assemble_o1_prompt` / `build_agent_prompt` / market bin 模板）经独立 trace **无** scoring-to-prompt 数据流，故为**缺失的纵深防御层 / latent risk**（未来任何 tape/board/memory 面若携 PPUT 值即暴露），非当前在用泄漏。 |

---

## 3. 每个 confirmed bypass 的 COMPLETENESS/INVARIANT GATE 提案

均采 `tests/constitution_single_admission_contract.rs` 风格：**enumerate-all-sites + assert**，
SOURCE-STRUCTURAL witness（grep 正典源文件，不能被 `assert!(true)` 满足，移除/旁路/再复制即 RED）。
每个 gate 须遵守 triple-coupling（manifest + CONSTITUTION_EXECUTION_MATRIX + `ls tests/constitution_*.rs` glob）。

### Gate 1 — `constitution_llm_err_lands_on_tape`（append-only-rubber，#1）
**修复风险类：Class 3**（evidence-completeness / tape；改 `tdma_runner.rs` 失败臂使其经 verified=false commit）。
- **不变式**：`tdma_runner` 内层循环中**每一个 `llm_call` 失败臂**在 `break`/`continue` 前必须经过一次 tape/CAS commit（verified=false 的 `AgentProposal` 或 `AttemptTelemetry`，`reject_class=llm_err`），与 `handle_rejection` 对称。
- **enumerate-all-sites**：grep `tdma_runner.rs` 中所有 `Err(` 臂位于 `llm_call(` 的 match 内者；断言每个这样的臂体在到达 `break 'outer`/`continue` 前包含一次 commit 调用（`kernel.step_forward_*` 或显式 `tape.commit`/`put_*` capsule）。
- **fails-RED 条件**：存在 `llm_call` 的 `Err` 臂直接 `break`/`return` 而无 commit。
- **修复方向**：把 `Err` 臂路由经 verified=false 的 tape/CAS commit，再 `break 'outer`，使 FC1 `evaluator_reported_completed_llm_calls = step + parse_fail + llm_err` 可从 tape 重建。

### Gate 2 — `constitution_external_attempt_anchored_on_failure`（evidence-cas-anchor，#2）
**修复风险类：Class 2**（runner 二进制接线；非 sequencer/typed_tx/cas-schema）。
- **不变式**：在任一 `*_current_kernel.rs` runner 中，**已完成的外部 LLM 调用**（token 已花费）到其首个 attempt-evidence `put_json` 之间，**不得存在带 `?` 或 `return Err` 的 parse/guard 提前退出**；失败必须 swebench-style 锚定（捕获 error 字段、代入空 claim、仍写 capsule）。
- **enumerate-all-sites**：grep 全部 16 个 runner 中 `ask_external_agent(`/`llm proxy generation`/`::generate(` 之后到首个 `put_json(...EvidenceCapsule...)` 之前的 span；断言该 span 内**无** `parse_*(...)?` 或 `return Err`。对 swebench 的 `parse_patch_claim`（不带 `?`、写 `parse_error` 字段）作为 reference 通过样例。
- **fails-RED 条件**：market_external_agent 当前形态（`parse_decision(..)?` + direction-mismatch `return Err` 在 `put_json` 之前）即 RED。
- **修复方向**：market binary 改为捕获 parse/direction error 进 capsule 字段并总是写 `MarketDecisionCapsule`（或专用 rejection capsule），再决定是否 abort——与 swebench sibling 对齐。

### Gate 3 — `constitution_all_canonical_writers_verify_trust_root`（boot-trust-root，#3）
**修复风险类：Class 4**（trust-root authority surface；触及"every binary launch 验证"宪法面，须 per-atom §8）。
- **不变式**：**每一个会 canonical-write（CAS put_json / GitTapeLedger / 提交签名 WorkTx 推进 state_root / 发 SystemEmitCommand）的二进制入口**，在做任何 work 前必须调 `verify_trust_root`。
- **enumerate-all-sites**：grep `src/bin/**` 中所有定义 `fn main`/`fn run` 且其可达调用图含 `put_json`/`GitTapeLedger::`/`build_chaintape_sequencer_with_initial_q`/`SystemEmitCommand` 的文件；断言每个这样的入口在 work 前出现 `verify_trust_root`。
- **fails-RED 条件**：`turingos.rs`、`*_current_kernel.rs`(~16)、`fc3_governance_reinit_*`、`turingos_web.rs` 当前均无 → RED。
- **修复方向**：在共享 runner 工厂 `build_chaintape_sequencer_with_initial_q`（或各 `main` 顶部）插入一次 `verify_trust_root(repo_root)?`，失败即 panic/abort（复用 `src/main.rs:14` 语义）。**注**：这是 Class 4，须 per-atom 架构师 §8 ratify；当前 gate `constitution_tc_boot_trust_root_manifest.rs` 只断言 2 个站点，须扩为全枚举。

### Gate 4 — `constitution_judge_reason_no_raw_subprocess_stderr`（raw-diagnostic-shield，#4）
**修复风险类：Class 2**（judge 输出整形；非 trust-root，但触及屏蔽不变式建议 §8 知会）。
- **不变式**：**每个 judge 的 `Fail.reason`/返回给 agent 的诊断字符串**必须是确定性/有界 class 字符串，不得含未整形的原始 subprocess stderr tail。
- **enumerate-all-sites**：grep `src/judges/*.rs` 中所有构造返回字符串的 fail 路径；断言无 `tail_chars(<stderr>, N)` 或等价的原始 stderr 透传进 reason。lean/nesbitt/putnam/generate 作为 reference 通过样例。
- **fails-RED 条件**：`swebench_test_judge.rs:346` 的 `tail_chars(&String::from_utf8_lossy(stderr), 400)` fallback → RED。
- **修复方向**：把 fallback 换成有界结构化 class 字符串（如 `swebench harness error (exit N): <one-line class>`），或令 kernel 在 swebench class 上总用可信 claim set 覆写 `header.failed_predicate` 而非信任 LLM 的 null。

### Gate 5 — `constitution_metric_leak_guard_wired`（goodhart-shield，#5）
**修复风险类：Class 2**（接线 + 静态 grep gate；guard 本身 pinned，接线不改 pinned 内容）。
- **不变式**：命名 runtime guard `assert_no_metric_leak` **必须至少有一个生产 prompt-装配站点调用方**（否则 Art.III.4 RUNTIME 强制站点真空）。
- **enumerate-all-sites**：grep prompt 最终装配处（`assemble_o1_prompt` / `build_agent_prompt` / market bin 模板）；断言在 prompt 交付 LLM 前调用了 `assert_no_metric_leak(&final_prompt)`。
- **fails-RED 条件**：当前 `rg assert_no_metric_leak` 仅命中 `#[cfg(test)]` → RED。
- **修复方向**：在每个 prompt-交付点加 `assert_no_metric_leak(&prompt)`（纵深防御）。**注**：此为 latent，可作较低优先级；但"pinned 却 wire 到 nothing"本身值得作为 hygiene gate 固化，防止未来 PPUT 经新 tape/board 面回流。

---

## 4. UNCERTAIN（human review 必需）

| 不变式 | 站点 | 为何 UNCERTAIN | 建议 |
|--------|------|----------------|------|
| evidence-cas-anchor | `src/bin/math_competition_reasoning_current_kernel.rs:315-322`（+8 个 sibling domain binary：gpqa:317/320、gaia:335/338、toolbench:351/354、webarena:440/443、osworld:455/458、cybench:464/467、mind2web:435） | 代码模式与 #2（market）**完全同形**：真实 LLM 调用后 `parse_answer_claim(..)?`（315）+ rationale-length guard（317-322）early-return，`MathAnswerClaimCapsule` 在 guard 之后（336）。失败分支下完成的 attempt response 无 CAS/L4/L4.E。**但**：这些 binary 自声明 `closure_scope=domain_adapter_smoke_only`、`final_closure_possible=false`（560-562），且正典 FC1 `attempt_count_invariant`（`chain_derived_run_facts.rs::n()`）**不跑**在这些 smoke tape 上——无 gate 被击穿。故"是否构成对*已强制*完备性不变式的旁路"边界模糊：泄漏真实（token 已花、失败分支 response 未锚定），但不击穿任何 ship-gate。 | 人审定性：若 domain_adapter smoke 未来升格为 benchmark/closure，须先按 swebench-style 锚定失败 attempt。建议**前向修复**（与 Gate 2 同形扩展到这 9 个 binary），但当前非 active ship-path bypass。 |

---

## 5. 对 verification-strategy redesign 的一段式裁决

本轮证实 **M07 不是孤例而是一类系统性失效模式**：本树至少 3 个独立 MAJOR 真并行旁路
（llm_err 不落 tape、market 失败分支不锚 CAS、~18 个 canonical-write 入口不验 trust
root）都呈现同一结构——**某个完备性不变式只在一个"明显"站点被强制，而 gate 恰好也只
断言那一个站点**，于是 gate GREEN 与不变式实际成立之间出现缺口。当前的 gate 设计普遍是
*single-site witness*（"检查 X 在 sequencer 强制" / "检查 main.rs 调 verify"），这正是
M07 illusion 的温床。**verification-strategy 必须从 single-site assertion 转向
enumerate-all-sites completeness gate**：对每个 class-S 不变式，gate 先用 grep 穷举类
S 的*全部*站点（all writers / all binary entries / all judges / all failure arms），再
对每个站点断言 P——使新增一个并行站点而忘记接线时 gate 必然 RED。`constitution_single_admission_contract`
已是这种风格的范本，应推广为模板。次要但重要：trust-root pinning 与 runtime wiring
解耦（pinned-but-dead 的 `assert_no_metric_leak` 证明 pin 一个文件 ≠ 它在生产被调用），
因此完备性 gate 还须验证"被 pin 的 guard 确有生产调用方"，而非仅验证其哈希。
