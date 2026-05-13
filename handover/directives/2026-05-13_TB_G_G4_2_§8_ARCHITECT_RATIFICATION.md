# TB-G G4.2 — Architect §8 Ratification

> **Status**: RATIFIED. This file preserves the architect/user verdict
> supplied in conversation on 2026-05-13. It is the local fact source for
> G4.2 implementation and audit. Do not replace this with a summary.

---

下面是我的裁决和完整后续安排。你可以直接转发给 AI coder。

---

# 1. 总体审计结论

我接受当前状态的主线判断：

```text
G3 已经进入 GREEN / 已足够关闭当前经济牙齿问题。
下一步进入 G4.2：agent_model_assignment genesis schema + no-hidden-model-switch binding。
```

G4.2 的 charter 定位是正确的：它不是“多模型跑一跑”，而是把 **每个 Agent 的模型身份** 从 runtime env 配置升级为 **可 replay、可审计、可阻止隐藏换模的宪法事实**。你给的 G4.2 packet 明确写了目标链路：

```text
Agent_i
-> genesis-assigned model identity
-> AttemptTelemetry actual model
-> audit assertion: no hidden model switch
-> dashboard/report divergence by model family
```

而且 packet 明确说明它是 Class 4 STEP_B，因为它改变 genesis / replay authority、Trust Root pinned files，并定义 model identity 如何成为 replayable truth。

所以我的总体裁决是：

> **批准 G4.2 进入 §8 ratification。**
> **批准按 packet 的 Q1–Q6 执行，但要补两条额外要求：模型身份必须进入 PromptCapsule / AttemptTelemetry 的一致链路；run_report 必须能按 model family 做行为差异分析。**

---

# 2. 回答 AI coder 的 Q1–Q6

## Q1：agent_model_assignment 存哪里？

裁决：

```text
存 GenesisReport。
不要存 EconomicState。
不要存 global pointer。
```

理由：

```text
model assignment 是 Boot / Genesis 事实；
不是经济状态；
也不是可变运行时配置；
更不是 filesystem global latest pointer。
```

正确结构：

```rust
pub agent_model_assignment: Vec<AgentModelAssignment>
```

并且：

```text
deterministic order: sorted by agent_id
serde(default) for historical genesis_report
```

G4.2 packet 已经推荐这个方向，我接受。

---

## Q2：temperature 如何持久化？

裁决：

```text
使用 integer temperature_milli。
禁止 f64 / f32。
```

示例：

```text
temperature = 0.7 -> temperature_milli = 700
temperature = 0.0 -> temperature_milli = 0
```

理由：

```text
所有 replay-critical / persisted schema 都应避免 float；
尤其不能让模型身份和 prompt config 出现不可复现的小数漂移。
```

这也符合项目长期的 no-f64 money-path / deterministic-schema 约束。

---

## Q3：AttemptTelemetry 如何记录 actual model？

裁决：

```text
tail-add actual model identity fields to AttemptTelemetry。
```

允许字段：

```rust
#[serde(default)]
pub model_name: Option<String>,
#[serde(default)]
pub model_family: Option<String>,
#[serde(default)]
pub model_provider: Option<String>,
#[serde(default)]
pub model_version: Option<String>,
#[serde(default)]
pub temperature_milli: Option<i64>,
```

但必须证明二选一：

```text
1. old AttemptTelemetry bytes still decode with serde(default);
或
2. schema bump + dual-reader / grandfathering path exists。
```

不允许：

```text
旧 evidence 变得不可读。
```

---

## Q4：actual-vs-genesis mismatch 怎么处理？

裁决：

```text
任何 actual model 与 genesis assignment 不一致，都视为 hidden model switch。
默认 fail audit。
```

G4.2 不引入 model switch event。

如果未来要允许换模型，必须单独做：

```text
ModelSwitchTx / ModelSwitchEvidence
或 CAS-visible ModelSwitch event
Class 4 §8 packet
```

G4.2 当前阶段只做：

```text
no hidden model switch binding
```

---

## Q5：是否 bundle G4.1 / G4.3 / G4.4？

裁决：

```text
只 bundle 最小 scaffolding。
```

允许：

```text
G4.1 activation minimal:
  env resolver -> genesis assignment

G4.3 dashboard view minimal:
  model-family activity report

G4.4 no-hidden-switch detector:
  genesis assignment vs AttemptTelemetry actual model assertion
```

禁止：

```text
G5 scheduler
G6 pricing feedback
G7 run6-equivalent role arena
WalletBackend
role classifier
opportunity scheduler
new market mechanics
```

换句话说，G4.2 是：

```text
model identity replay + hidden switch prevention
```

不是：

```text
multi-model market behavior research
```

---

## Q6：audit cadence 是否只用 Codex？

裁决：

```text
采用 latest Harness default：
one clean-context Codex review after gates/evidence。
Gemini 不强制。
```

但加一个条件：

```text
如果 Codex 给 CHALLENGE / VETO，或发现 genesis / Trust Root / replay authority 相关不确定性，
则升级为 Codex round-2 + optional Gemini architecture review。
```

理由：

G4.2 虽是 Class 4，但它没有改：

```text
sequencer admission
typed tx schema
canonical signing payload
economic state
system tx authorization
```

它触碰的是 genesis / replay authority / Trust Root pinned surfaces，所以一个 clean-context Codex 可以作为默认，但 VETO > CHALLENGE > PASS 仍然适用。

---

# 3. 我建议的 §8 ratification 文案

你可以直接签：

```text
I ratify G4.2 as a Class-4 STEP_B atom under the following decisions:

Q1: agent_model_assignment is stored in GenesisReport, not EconomicState and not a global pointer.
Q2: persisted temperature uses integer temperature_milli; no f64/f32.
Q3: AttemptTelemetry records actual model identity, with compatibility proof or dual-reader grandfathering.
Q4: actual-vs-genesis mismatch is a hidden model switch and fails audit; no model-switch event is introduced in G4.2.
Q5: bundle only minimal G4.1/G4.3/G4.4 scaffolding needed to prove G4.2; do not batch G5/G6/G7.
Q6: use one clean-context Codex audit after evidence; no Gemini by default unless Codex returns CHALLENGE/VETO or architect requests it.

Allowed:
- paths listed in packet §4;
- Trust Root rehash for modified pinned files;
- turingos_dev evidence run;
- minimal 3-problem multi-model smoke;
- dashboard/report model-family activity view;
- no-hidden-model-switch audit assertion.

Forbidden:
- sequencer admission changes;
- typed tx schema/discriminant/signing-domain changes;
- global latest model pointer;
- raw prompt/completion/CoT storage;
- runtime model switch without future ChainTape/CAS event packet;
- batching with G5/G6/G7 or unrelated Class-4 atoms.

Ship only after:
- targeted G4 tests pass;
- Trust Root verify passes;
- constitution gates pass;
- workspace tests pass;
- minimal multi-model smoke witnesses at least 3 model families;
- clean-context Codex verdict is PROCEED.
```

---

# 4. 当前状态 Gap Analysis

## 4.1 宪法层：G4.2 是 FC2 + FC1 + FC3 的交界点

G4.2 看起来只是“记录模型身份”，但其实同时触碰三张 flowchart。

### FC2 Boot / Genesis

模型身份必须是 genesis fact：

```text
Agent_i -> assigned_model
```

这必须出现在：

```text
genesis_report.json
```

不能只在：

```text
AGENT_MODELS env
runner script
dashboard report
stdout
```

否则 replay 无法证明某个 Agent 原本被分配的是哪个模型。

### FC1 Runtime Loop

每次 LLM attempt 都必须记录实际 model：

```text
AttemptTelemetry.actual_model
```

否则无法回答：

```text
这次 attempt 是不是由 genesis-assigned model 产生？
是否发生 hidden switch？
不同模型是否产生不同策略？
```

### FC3 Meta / Reporting

Dashboard / report 只能作为 materialized view：

```text
model divergence report
hidden-switch verdict
model-family activity
```

这些必须从：

```text
GenesisReport + AttemptTelemetry + ChainTape + CAS
```

派生，而不是从 stdout 或 run script 派生。

G4.2 packet 已经明确把这三层映射出来：FC2 genesis replay、FC1 AttemptTelemetry、FC1 runtime identity、FC3 audit/report view、Art. III shielding。

---

## 4.2 当前方案与白皮书愿景的 gap

我们的长期目标是：

```text
multi-LLM agents 在同一 tape 上持续竞争、合作、交易、验证、挑战。
```

但如果没有 G4.2，就无法证明：

```text
Agent_0 一直是 Claude？
Agent_1 一直是 GPT？
Agent_2 一直是 Qwen？
某个高收益 agent 是否中途换成更强模型？
某个 bad actor 是否隐藏切换？
```

这会破坏：

```text
role differentiation
model-family behavior comparison
market behavior attribution
reputation
PnL
autopsy
future scheduler
```

所以 G4.2 是进入真正 generative phase 的必要基础。
它不是可选 polish。

---

## 4.3 与 G3.2 的关系

G3.2 解决：

```text
经济后果：
  risk cap
  AutopsyCapsule
  reputation +1
  bond return
```

G4.2 解决：

```text
身份真实性：
  agent 被分配的模型是谁
  actual attempt 的模型是谁
  是否 hidden switch
```

两者合起来才有：

```text
可归因的经济后果。
```

否则 PnL / reputation / bankruptcy 都会变成模糊的：

```text
某个 agent id 赢了 / 输了
```

但你不知道它背后到底是不是同一个模型策略。

最新 full-landing audit 中，G3.2 被定位为关闭当前 §R G3 经济 AMBER 的关键 packet；G4.2 则是后续 multi-LLM generative arena 的身份基座。

---

## 4.4 与 TB-N3 / Polymarket 的关系

TB-N3 已经证明 Polymarket / CPMM bridge 的 substrate 与 gate-level 证据是 green，但 Phase 2 real-LLM batch 仍依赖 API availability；TB-N3 packet 中也说它是 ship candidate，等待 Phase 2 real-LLM batch。

而 TB-N3 charter 的 Phase 1 evidence 明确显示：

```text
5/9 solved
CPMM kernel complete
EventResolveTx witnessed
但 market activity = 0
```

这个 gap 被定义为 wire，而不是 capability。

G4.2 对 TB-N3 的后续非常关键，因为如果要解释：

```text
为什么 deepseek 不交易？
换成 Claude/GPT/Qwen 会不会交易？
哪个 model family 更像 Bull / Bear / Solver / Verifier？
```

你必须先让 model identity 成为 replayable truth。
否则多模型实验不可审计。

---

# 5. 当前最重要的新增建议

除了回答 Q1–Q6，我认为必须把下面 6 条加入 G4.2 / 后续方案。

---

## 5.1 增加 ModelAssignmentManifest CID

除了 `GenesisReport.agent_model_assignment`，建议把完整 assignment 作为 CAS manifest：

```rust
ModelAssignmentManifest {
    batch_id,
    agent_model_assignment,
    resolver_source,
    agent_models_env_hash,
    phase_d_hetero_ok,
    created_at_head_t,
}
```

然后：

```text
genesis_report.model_assignment_manifest_cid = ...
```

好处：

```text
GenesisReport 可保持简洁；
CAS manifest 可扩展；
后续 dashboard / audit 可重建。
```

如果太大，就放 CAS；如果不大，可两者都放。

---

## 5.2 记录 model resolver provenance

G4.2 不应该只记录最终模型名，还要记录：

```text
AGENT_MODELS input hash
PHASE_D_HETERO_OK
fallback behavior
proxy/provider chosen
```

否则 audit 无法知道：

```text
为什么 Agent_3 是 deepseek 而不是 qwen？
env resolver 是否 fallback 到 unknown？
```

建议字段：

```rust
resolver_provenance_cid: Option<Cid>
```

或者放入 manifest。

---

## 5.3 处理 unavailable model 的 fail-closed 规则

如果 `AGENT_MODELS` 要求 3 个 families，但 runtime 只成功连到 1 个，不能 silent downgrade。

新增：

```text
model_family_count_required = 3
model_family_count_observed >= 3
```

否则：

```text
fail-closed
```

除非 run 明确是：

```text
single-model diagnostic
```

但不能叫 G4.2 multi-model evidence。

---

## 5.4 Dashboard 必须报告 model-family divergence，但不能下结论过度

Dashboard 输出：

```text
attempt_count_by_model_family
accepted_worktx_by_model_family
l4e_rejection_by_model_family
verify_count_by_model_family
challenge_count_by_model_family
invest_count_by_model_family
pnl_by_model_family
```

但禁止：

```text
model X is better
```

除非有足够 benchmark protocol。

G4.2 只是 identity replay，不是 model eval leaderboard。

---

## 5.5 PromptCapsule 必须链接 model assignment

每个 attempt 的 PromptCapsule 应能回答：

```text
这个 prompt 是发给哪个 assigned model？
model-specific template 是否不同？
template hash 是什么？
```

所以 PromptCapsule 应至少有：

```text
assigned_model_family
prompt_template_hash
```

但不要存 raw prompt body。

这能补 Art. III shielding 与 model identity 的交叉 gap。

---

## 5.6 Hidden switch 也要区分 harmless vs malicious cause

Audit verdict 需要输出原因分类：

```text
HiddenSwitchCause:
  EnvResolverMismatch
  ProviderFallback
  ManualOverride
  RuntimeProxyReroute
  MissingAttemptTelemetry
  Unknown
```

如果发现 mismatch，不是直接删 evidence，而是：

```text
BLOCK + cause classification
```

这对后续 debugging 很重要。

---

# 6. 后续完整执行安排

## Step 1 — Ratify G4.2 §8

用上面的 Q1–Q6 文案。

产物：

```text
handover/directives/2026-05-12_TB_G_G4_2_§8_RATIFICATION.md
```

---

## Step 2 — G4.2 implementation atoms

### Atom G4.2-A — tests first

新增：

```text
tests/constitution_g4_multi_llm.rs
tests/constitution_g4_no_hidden_model_switch.rs
```

这些测试先红。

---

### Atom G4.2-B — Genesis schema

实现：

```rust
GenesisReport.agent_model_assignment: Vec<AgentModelAssignment>
```

要求：

```text
serde(default)
sorted by agent_id
integer temperature_milli
prompt_template_hash only
```

---

### Atom G4.2-C — env resolver integration

把：

```text
AGENT_MODELS
PHASE_D_HETERO_OK=1
```

解析结果写入 genesis report / ModelAssignmentManifest。

---

### Atom G4.2-D — AttemptTelemetry actual model

每个 LLM call 的 AttemptTelemetry 写入 actual model。

要求：

```text
old telemetry still decodes
or dual-reader implemented
```

---

### Atom G4.2-E — no-hidden-switch assertion

Audit walker 比较：

```text
GenesisReport.assignment[agent_id]
vs
AttemptTelemetry.actual_model
```

mismatch：

```text
BLOCK
```

---

### Atom G4.2-F — script / smoke manifest

`run_g_phase_batch.sh` 必须记录：

```text
AGENT_MODELS
PHASE_D_HETERO_OK
assignment summary
model-family count
```

---

### Atom G4.2-G — dashboard view

Dashboard 输出：

```text
model-family activity
hidden-switch verdict
attempt / accepted / rejected / market activity by model family
```

---

### Atom G4.2-H — Trust Root rehash

因为触碰 genesis / pinned files，需要更新 Trust Root。

---

### Atom G4.2-I — evidence + audit

最小 smoke：

```text
PHASE_D_HETERO_OK=1
AGENT_MODELS="<model_a>,<model_b>,<model_c>,..."
bash scripts/run_g_phase_batch.sh g_phase_g4_2_mini_<UTC> mini
```

要求：

```text
至少 3 distinct model families
或 fail-closed reason prevents ship
```

然后：

```text
clean-context Codex review
```

---

# 7. G4.2 的 Ship Gates

```text
SG-G4.2.1
GenesisReport contains deterministic sorted agent_model_assignment.

SG-G4.2.2
No f64 / f32 in persisted temperature.

SG-G4.2.3
AttemptTelemetry records actual model identity for every new LLM attempt.

SG-G4.2.4
Historical AttemptTelemetry remains parseable.

SG-G4.2.5
No-hidden-switch audit detects mismatch.

SG-G4.2.6
Minimal multi-model smoke witnesses >=3 model families.

SG-G4.2.7
Dashboard model-family view regenerates from GenesisReport + CAS + ChainTape.

SG-G4.2.8
No raw prompt / completion / CoT stored in model identity records.

SG-G4.2.9
Trust Root verify passes.

SG-G4.2.10
Constitution gates pass.

SG-G4.2.11
Workspace tests pass.

SG-G4.2.12
Clean-context Codex verdict = PROCEED.
```

---

# 8. G4.2 禁止事项

```text
No sequencer admission changes.
No TypedTx schema / discriminant / signing domain changes.
No global latest model pointer.
No raw prompt / raw completion / CoT storage.
No hidden runtime model switch.
No G5/G6/G7 batching.
No WalletBackend changes.
No scheduler changes.
No role classifier.
No market behavior claims.
No Gemini requirement unless future directive says so.
```

---

# 9. 项目整体后续顺序

我的推荐顺序：

```text
1. G4.2 model identity replay / no hidden switch
2. G5 opportunity scheduler observe-only
3. G6 price feedback observe-only
4. G7 run6-structural smoke
5. TB-GD Gardener Agent
6. Larger multi-model / market batch
7. Only then real-world pilot design
```

但要强调：

```text
G4.2 之前，不要做 multi-model behavior claims。
G5/G6/G7 之前，不要 claim emergent role differentiation。
```

---

# 10. 给 AI coder 的直接执行指令

```text
Architect verdict:

Ratify G4.2 with the following decisions:

Q1:
  Store agent_model_assignment in GenesisReport, not EconomicState and not global pointer.

Q2:
  Persist temperature as integer temperature_milli. No f64/f32.

Q3:
  Tail-add actual model identity to AttemptTelemetry, with old-evidence compatibility proof or dual-reader grandfathering.

Q4:
  Any actual-vs-genesis mismatch is hidden model switch and fails audit. No model-switch event introduced in G4.2.

Q5:
  Bundle only minimal G4.1 activation, G4.3 dashboard view, and G4.4 no-hidden-switch detector. Do not batch G5/G6/G7.

Q6:
  Use one clean-context Codex audit after evidence. Gemini not required unless Codex returns CHALLENGE/VETO or architect asks.

Additional requirements:
  - Add ModelAssignmentManifest or equivalent provenance.
  - Record AGENT_MODELS / resolver provenance.
  - Fail closed if required >=3 model families are not witnessed.
  - Link PromptCapsule to assigned model family / prompt_template_hash.
  - Dashboard shows model-family activity but does not claim model ranking.
  - Hidden switch mismatch should classify cause if possible.

Proceed with atoms A-I as packet describes.
Stop immediately if implementation needs sequencer.rs, typed_tx.rs, signing payloads, kernel.rs, or bus.rs.
```

---

# 11. 最终判断

G4.2 是非常值得做的，而且应该现在做。

原因不是“多模型实验有趣”，而是：

```text
没有可 replay 的 model identity，
就没有可审计的 multi-agent cognition。
```

TuringOS 的下一阶段想要研究：

```text
multi-LLM disagreement
role differentiation
bull / bear / solver / verifier
market behavior
agent reputation
PnL
Autopsy
```

那么第一步必须是：

```text
谁是谁，谁用了什么模型，是否中途偷换。
```

G4.2 就是把这件事从 env 配置升级成 constitution-grade fact。
所以我建议：**批准 G4.2，严格按 Class-4 §8 执行，但不要夹带 G5/G6/G7。**
