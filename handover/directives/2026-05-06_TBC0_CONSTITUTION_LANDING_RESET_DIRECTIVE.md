# TuringOS TB-C0 Constitution Landing Reset Directive

**Date:** 2026-05-06
**Source:** Architect (zephryj@icloud.com)
**Class:** Project-level reset (supersedes all in-flight feature TBs until TB-C0 closes)
**Authority:** Autonomous-mode authorization. AI coder is to "implement in auto mode until you meet the requirements listed in the following content."
**Storage policy:** Lossless archive per `feedback_kolmogorov_compression`. Original architect message preserved verbatim below; `## Annotation` section added by Claude AI coder for indexing only.

---

## Architect Message — Verbatim (lossless)

你这个判断是对的，而且要用更硬的话说：

> **现在不能继续把"功能 TB"往前推。必须先做一次 Constitution Landing Reset：把宪法变成可执行 CI、可运行 harness、可复现真实测试。**
> 只要宪法三张 flowchart 还有任何一张没有被代码硬闸门覆盖，后续所有 NodeMarket、Polymarket、MiniF2F、大规模 benchmark、真实世界测试都应该冻结。

这不是"再写一份文档"。这是一次工程重置。

---

# 1. 当前根因：不是功能没写够，而是宪法没有变成第一层 harness

现在的问题不是：

```text
功能少了；
审计少了；
文档不够细；
AI coder 不努力。
```

真正问题是：

```text
宪法仍然主要以 directive / charter / audit language 存在，
而不是以 repo 里的 executable gates 存在。
```

所以每次都在发生这种漂移：

```text
先写功能
再审计
发现不合宪
再补 directive
再修
再审计
又发现更底层不合宪
```

这就是为什么 TB-18 拖这么久。

你要的图灵机不是"最终答案上链"。你要的是：

```text
Agent 的外部化计算活动
在 tape 上被 read / write / predicate / ledger 组织起来。
```

而 TB-18 M1 审计已经明确指出，当前 benchmark pipeline 把 N 次外部化 LLM proposal 压缩成 1 个 ChainTape WorkTx；最重的 P49 是 `N=32 -> M=1`，等于把 31 次有意义的计算轨迹从 authoritative ledger 中抹掉。 这不是小 bug，而是 Flowchart 1 级别的违宪。

---

# 2. 第一裁决：冻结所有功能 TB，先做 Constitution Landing Gate

当前状态我裁决为：

```text
TB-18R = CANDIDATE REMEDIATION
not shipped
not eligible for M2/M3
not eligible for NodeMarket
not eligible for real-world readiness
```

你提供的状态也明确写了：TB-18R 目前是 candidate remediation，不是 shipped；Phase 3 真实 rerun 和 final ship 都仍需要显式架构师指令。

所以现在只允许做一件事：

```text
Constitution Landing Reset
```

我建议命名为：

```text
TB-C0 — Constitution Landing Gate
```

或如果要嵌入当前 TB：

```text
TB-18R-C — Constitutional Harness Completion
```

我更建议用 **TB-C0**，因为它不是 TB-18 的一个小修补，而是整个项目的元闸门。

---

# 3. 什么叫"宪法完全落地"

注意，"宪法完全落地"不是说未来所有产品功能都写完。
它的意思是：

> **宪法的每条系统边界都有可执行测试、可复现 smoke、可失败的 kill gate。**

也就是：

```text
自然语言宪法
-> executable invariants
-> tests
-> harness
-> smoke
-> CI
-> audit evidence
```

从现在起，任何 TB 都必须先通过这些 constitution gates，才能继续。

---

# 4. 三张 Flowchart 的可执行落地

无损宪法整编版明确保留三张 flowchart，并给出哈希校验：Flowchart 1 是基础运行循环，Flowchart 2 是 Boot 与完整架构，Flowchart 3 是元架构 / InitAI / JudgeAI / ArchitectAI。

这三张图必须变成三组 tests。

---

## 4.1 FC1 Runtime Loop Gate

Flowchart 1 的工程语义：

```text
Q_t
-> rtool / context
-> Agent output
-> predicates
-> wtool
-> Q_{t+1}
```

必须落成以下硬不变量：

```text
Every externalized Agent proposal must become:
  L4 accepted transition
  OR L4.E rejection evidence
  OR EvidenceCapsule item anchored by L4 terminal tx.
```

### 必须新增测试

```text
fc1_every_externalized_attempt_is_tape_visible
fc1_predicate_pass_goes_l4
fc1_predicate_fail_goes_l4e
fc1_no_legacy_authoritative_append
fc1_dashboard_not_source_of_truth
fc1_attempt_count_equals_tape_count
fc1_no_fake_accepted_nodes
```

### 必须新增真实 smoke

```text
P23 one-shot:
  evaluator tx_count = 1
  chain_attempt_count = 1

P38 multi-attempt:
  evaluator tx_count = N
  chain_attempt_count = N

P49 heavy:
  evaluator tx_count = 32 或实际 N
  chain_attempt_count = 同 N
```

如果 P49 仍然是：

```text
evaluator tx_count = 32
chain_attempt_count = 1
```

就不许继续任何 benchmark。

---

## 4.2 FC2 Boot Gate

Flowchart 2 的工程语义：

```text
human spec / constitution
-> predicates / tools
-> Q_0 / on_init
-> runtime loop
```

必须落成：

```text
所有初始化都是 ChainTape 事件；
没有 memory-only preseed；
没有 ghost genesis；
没有 global latest pointer。
```

### 必须新增测试

```text
fc2_genesis_report_exists
fc2_on_init_only_mint
fc2_no_post_init_mint
fc2_no_memory_only_preseed
fc2_taskopen_escrowlock_are_chain_events
fc2_run_replayable_from_genesis_tape_cas
fc2_system_pubkeys_verify
fc2_agent_registry_resolves
```

### 必须禁止

```text
q.economic_state_t.insert(...) 作为正式 evidence；
手工预置 task market / escrow；
运行后才补 genesis_report；
global filesystem pointer 当 source of truth。
```

---

## 4.3 FC3 Meta / Markov Gate

Flowchart 3 的工程语义：

```text
constitution as ground truth
logs archive as ground truth
ArchitectAI proposes
JudgeAI vetoes
tools / logs / Q update
```

必须落成：

```text
EvidenceCapsule / MarkovCapsule 是 ChainTape + CAS 派生视图；
不是隐藏真相；
不是全局指针；
不是 prompt 污染源。
```

### 必须新增测试

```text
fc3_capsule_derived_from_tape_cas
fc3_no_global_markov_pointer
fc3_raw_logs_not_in_agent_read_view
fc3_latest_capsule_context_only
fc3_deep_history_requires_override
fc3_no_automatic_predicate_mutation
fc3_architectai_proposal_not_direct_write
fc3_judgeai_veto_only
```

TB-16 已经暴露过 `LATEST_MARKOV_CAPSULE.txt` 这种全局 pointer 会变成 Art. 0.2 平行账本；类似问题绝不能再出现。

---

# 5. 宪法主文的执行闸门

宪法开篇已经说，顶层白盒的职责不是微观操纵，而是信号管理：量化、广播、屏蔽。
所以除了三张 flowchart，还要加四组 constitutional gates。

---

## 5.1 Predicate Gate

宪法定义谓词是只输出真/假的判定机器；布尔信号的本质是验证有没有通过。

测试：

```text
predicate_result_is_binary
predicate_failure_cannot_enter_l4
predicate_pass_required_for_l4
lean_verified_required_for_verified_worktx
price_never_overrides_predicate
```

---

## 5.2 Shielding Gate

宪法要求屏蔽污染性信息，不把 raw failure logs 广播给所有 Agent。
测试：

```text
raw_lean_stderr_not_in_agent_read_view
l4e_public_summary_low_pollution
private_diagnostic_cid_not_serialized_publicly
evidence_capsule_raw_logs_audit_only
dashboard_does_not_leak_private_failure_detail
```

---

## 5.3 Economy Gate

宪法经济基本法：

```text
Information is Free
Only Investment Costs Money
1 Coin = 1 YES + 1 NO
on_init 是唯一合法铸币点
```

这些必须有 CI。

测试：

```text
economy_read_is_free
economy_write_requires_stake_or_escrow
economy_no_post_init_mint
economy_total_coin_conserved
economy_complete_set_yes_no_not_coin
economy_no_ghost_liquidity
economy_wallet_read_only_projection
economy_no_f64_money_path
system_tx_not_agent_submittable
```

---

## 5.4 Tape Canonical Gate

这是现在最要命的。

测试：

```text
no_parallel_ledger_source_of_truth
no_shadow_tape_authoritative_parent
canonical_txid_not_shadow_id
dashboard_regenerates_from_tape_cas
chain_derived_facts_not_evaluator_stdout
all_externalized_attempts_have_cas_payload
all_lean_results_have_cas_payload
```

这组测试直接防止 TB-18 M1 那种 N→1 collapse。

---

# 6. 立即建立 repo 里的 Constitution CI

AI coder 下一步不要再开功能 TB。
先建这些文件：

```text
handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
handover/alignment/TRACE_FLOWCHART_MATRIX.md
tests/constitution/fc1_runtime_loop.rs
tests/constitution/fc2_boot.rs
tests/constitution/fc3_meta.rs
tests/constitution/predicate_gate.rs
tests/constitution/shielding_gate.rs
tests/constitution/economy_gate.rs
tests/constitution/tape_canonical_gate.rs
tests/constitution/no_parallel_ledger.rs
```

然后新增一个命令：

```bash
cargo test --workspace constitution_
```

或者：

```bash
cargo test --workspace --test constitution_gates
```

如果目前 Rust test organization 不支持，就先加 integration test files。

---

# 7. Constitution Execution Matrix 格式

`CONSTITUTION_EXECUTION_MATRIX.md` 必须长这样：

```text
Constitution Clause / Flowchart Node
  -> Code surface
  -> Test name
  -> Smoke evidence
  -> Current status
  -> Kill condition
```

示例：

```text
FC1: Agent output -> predicates -> wtool
Code:
  evaluator.rs
  bus.submit_typed_tx
  sequencer.apply_one
  L4 / L4.E writers

Tests:
  fc1_every_externalized_attempt_is_tape_visible
  fc1_predicate_fail_goes_l4e

Smoke:
  P38 / P49 Phase 3

Kill:
  evaluator_tx_count != chain_attempt_count
```

没有 test 的宪法条款，状态必须是：

```text
RED
```

不能写"covered by docs"。

---

# 8. 第一轮必须通过的最小宪法闸门

不要一口气把所有宪法都写满。
先设一个 **Constitution Landing MVP**。

## MVP Gate 1：真实 attempt 上 tape

```text
P38 / P49:
  evaluator_reported_tx_count == chain_attempt_count
```

## MVP Gate 2：predicate 分流

```text
Lean pass -> L4
Lean fail / parse fail / sorry -> L4.E or anchored EvidenceCapsule
```

## MVP Gate 3：dashboard 不是事实源

```text
delete dashboard
regenerate from ChainTape + CAS
same result
```

## MVP Gate 4：Boot 可 replay

```text
fresh repo + CAS + genesis_report -> reconstruct run
```

## MVP Gate 5：经济守恒

```text
no post-init mint
total coin conserved
wallet read-only
system tx not agent submitted
```

这 5 个过不了，不许继续任何功能开发。

---

# 9. 对 TB-18R 的新执行指令

现在 TB-18R 不再是：

```text
修 P38/P49 然后继续 benchmark
```

而是：

```text
用 P38/P49 验证 Constitution Landing MVP。
```

执行顺序：

```text
1. 建 Constitution Execution Matrix。
2. 建 FC1/FC2/FC3 test skeleton。
3. 修 evaluator per-attempt tape visibility。
4. 跑 P38/P49。
5. 生成 constitution_gate_report.json。
6. 如果全部 green，再跑 Codex/Gemini。
7. 然后才 ship TB-18R。
```

---

# 10. 为什么这会更快

你现在慢，是因为每个问题都靠审计后发现。
这个 reset 后会变成：

```text
开发前 test 已经定义了宪法边界；
实现不合宪会直接红；
不需要等 Codex/Gemini 才发现；
真实测试更早暴露问题。
```

所以这不是增加流程，而是减少浪费。

以前：

```text
写功能 2 天
审计 1 天
发现架构错
重写 2 天
再审计
```

现在：

```text
宪法 harness 先红
实现直到绿
真实 smoke
再审计
```

这才是快。

---

# 11. 给 AI coder 的直接指令

可以直接发：

```text
Architect reset:

Stop all feature work.

Open TB-C0: Constitution Landing Gate.

Goal:
Turn constitution + 3 flowcharts into executable CI before any further TB.

Required files:
- handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md
- handover/alignment/TRACE_FLOWCHART_MATRIX.md
- tests/constitution/fc1_runtime_loop.rs
- tests/constitution/fc2_boot.rs
- tests/constitution/fc3_meta.rs
- tests/constitution/predicate_gate.rs
- tests/constitution/shielding_gate.rs
- tests/constitution/economy_gate.rs
- tests/constitution/tape_canonical_gate.rs
- tests/constitution/no_parallel_ledger.rs

Hard gates:
1. FC1:
   every externalized LLM-Lean attempt is tape-visible.
   evaluator_tx_count == chain_attempt_count.

2. FC2:
   run replayable from genesis_report + ChainTape + CAS.
   no memory-only preseed.

3. FC3:
   EvidenceCapsule / MarkovCapsule derived from ChainTape + CAS.
   no global latest pointer.

4. Predicate:
   predicate pass -> L4.
   predicate fail -> L4.E.

5. Shielding:
   raw diagnostics not in ordinary Agent read view.

6. Economy:
   no post-init mint.
   total coin conserved.
   no ghost liquidity.
   wallet read-only.
   system tx not agent submittable.

7. Tape:
   dashboard is materialized view only.
   no legacy authoritative append.
   no shadow/canonical ID mix.

Do not proceed to M2/M3, NodeMarket, PriceIndex, Polymarket, real-world readiness, or any new feature until TB-C0 is green.

P38/P49 Phase 3 are now constitution gate tests, not benchmark tests.

If any constitution gate fails, stop and fix harness/implementation.
```

---

# 12. 我会如何定义"宪法完全落地完成"

我只认下面这个状态：

```text
1. Every constitution clause has a matrix row.
2. Every critical row has a test.
3. Every test can fail.
4. P38/P49 real runs pass FC1.
5. Fresh replay passes FC2.
6. Markov / EvidenceCapsule passes FC3.
7. Economy laws pass.
8. Dashboard regeneration passes.
9. No high-risk feature can merge without constitution gates green.
10. The project can answer:
   "What did the Agent externalize?"
   "What passed predicates?"
   "What failed?"
   "What is on tape?"
   "What is only in CAS?"
   "What is only dashboard?"
```

到这一步，才可以说：

> **宪法先落地了。**

---

# 13. 最后一句话

你不是要"更多 agentic engineering"。
你要的是：

```text
一台有纸、有笔、有橡皮、有严格纪律的图灵机。
```

所以现在的最高优先级不是写更多功能，而是让 repo 本身具备一个不可绕过的宪法 harness：

```text
没有 tape activity，不算通过；
没有 predicate 分流，不算通过；
没有 replay，不算通过；
没有经济守恒，不算通过；
没有屏蔽，不算通过。
```

先把这个做完。然后后面开发速度会自然变快，因为每次漂移都会在第一小时失败，而不是在第十天审计时失败。

---

## Annotation (added by Claude AI coder, 2026-05-06)

Indexing only; original text above is source of truth.

### Effect summary
1. **Supersedes** the prior emergency harness reset directive's "run Phase 3 immediately" step. Phase 3 of TB-18R is now reframed as constitution-gate verification, not as a benchmark rerun.
2. **TB-18R reverts to CANDIDATE REMEDIATION**, not eligible for ship until TB-C0 green.
3. **Opens TB-C0** as the project's META-GATE — supersedes all in-flight feature TBs until closed.
4. **Operating mode reaffirmed**: Constitutional Harness Engineering (already installed 2026-05-06 first reset).
5. **Five MVP gates** define minimum closure criteria. P38/P49 now serve as constitution-gate evidence, not benchmark scoring.

### TB-C0 deliverables (10 hard files)
| File | Purpose |
|------|---------|
| `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` | Clause → code → test → smoke → status → kill matrix |
| `handover/alignment/TRACE_FLOWCHART_MATRIX.md` | FC1/FC2/FC3 node → code → test mapping |
| `tests/constitution/fc1_runtime_loop.rs` | 7 named tests (see §4.1) |
| `tests/constitution/fc2_boot.rs` | 8 named tests (see §4.2) |
| `tests/constitution/fc3_meta.rs` | 8 named tests (see §4.3) |
| `tests/constitution/predicate_gate.rs` | 5 named tests (see §5.1) |
| `tests/constitution/shielding_gate.rs` | 5 named tests (see §5.2) |
| `tests/constitution/economy_gate.rs` | 9 named tests (see §5.3) |
| `tests/constitution/tape_canonical_gate.rs` | 7 named tests (see §5.4) |
| `tests/constitution/no_parallel_ledger.rs` | dedicated parallel-ledger Art. 0.2 fence |

Cargo organization note: per directive §6, if Rust integration-test layout requires flat `tests/` files, the `tests/constitution/` subdirectory pattern needs to be wired through a `tests/constitution/mod.rs` or `lib.rs` rollup. Will follow existing TB-18R test-file conventions.

### MVP closure (the 5 minimum gates)
1. **MVP-1**: `evaluator_reported_tx_count == chain_attempt_count` on P38 + P49
2. **MVP-2**: predicate pass → L4; predicate fail → L4.E (or anchored EvidenceCapsule)
3. **MVP-3**: Dashboard regenerable from ChainTape + CAS alone
4. **MVP-4**: Fresh replay from `genesis_report + ChainTape + CAS`
5. **MVP-5**: Economy conservation (no post-init mint, total-coin-conserved, no ghost liquidity, wallet read-only, system-tx not agent-submittable)

### Closure criteria (10 conditions, directive §12)
1. Every constitution clause has a matrix row
2. Every critical row has a test
3. Every test can fail
4. P38/P49 real runs pass FC1
5. Fresh replay passes FC2
6. Markov / EvidenceCapsule passes FC3
7. Economy laws pass
8. Dashboard regeneration passes
9. No high-risk feature can merge without constitution gates green
10. Project answers: "What did Agent externalize? What passed predicates? What failed? What is on tape? What is only in CAS? What is only dashboard?"

### Class
- TB-C0 charter creation = **Class 0** (docs)
- Test-skeleton creation = **Class 1** (additive tests; no production behavior change)
- Evaluator hot-path repair (FC1 N→1 collapse) = **Class 3** (already on TB-18R R2 territory; not new STEP_B-restricted file)
- typed_tx / sequencer / CAS schema bumps if needed = **Class 4 STEP_B** (architect ratification required)

### Authorization audit trail
- 2026-05-06 (this directive): TB-C0 explicitly authorized; auto-mode through closure
- 2026-05-06 (prior): TB-18R Emergency Harness Reset — `2026-05-06_TB18R_EMERGENCY_HARNESS_RESET_DIRECTIVE.md` (this supersedes its "run Phase 3 immediately" step)
- 2026-05-06 (prior): TB-18R Round-2 ruling — `2026-05-06_TB18R_ROUND_2_ARCHITECT_RULING.md`

### Hard freeze (until TB-C0 closes)
NodeMarket, Polymarket, MiniF2F M1/M2/M3, large-scale benchmark, real-world testing, PriceIndex, public-chain, TB-19 pilot, formal H-VPPUT claim, "formal benchmark passed" externalization. The TB-18R freeze list is subsumed; TB-18R itself is also subordinate to TB-C0.

### Filename / charter naming
- This directive: `handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md`
- Charter to be authored: `handover/tracer_bullets/TB-C0_charter_2026-05-06.md`
- TB_LOG.tsv row to be added: TB-C0
