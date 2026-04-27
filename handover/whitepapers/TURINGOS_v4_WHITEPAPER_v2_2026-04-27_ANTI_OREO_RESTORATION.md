# TuringOS v4 Whitepaper v2 — Anti-Oreo Restoration

> **状态**: 战术宪法级对齐 (tactical constitutional-level alignment)
> **批准**: 2026-04-27, 用户（独狼研究员 / 人类宪法架构师）
> **关系**: `constitution.md` 仍为唯一最高 alignment；本文件在所有衍生文档（Plan v3.2 / Blueprint / Deepthink / 历史 v1）冲突时为准
> **不替代**: 不重写宪法 flowchart；不触发 Phase Z′ 6-stage rerun；不重建 TRACE_MATRIX 或 conformance test
> **作用**: 衍生文档校准镜 + Wave 6+ 决策最高对齐参照

---

## 用户独立审稿结论摘要

> Deepthink 版本方向是对的，但重心需要校正。它把 "ChainTape / 状态账本" 写成了主角，而宪法真正的主角是反奥利奥架构：顶层白盒做信号管理，中层黑盒做生成，底层白盒做可执行工具。区块链只能作为 tape 的一种可验证实现，不能成为 TuringOS 的本体。

### 必须修正的六点（v2 已落实）

1. **白皮书标题与叙事重心偏链化** → 副标题改为「反奥利奥架构下的可验证状态账本操作系统」；ChainTape 置于反奥利奥架构内部
2. **底层白盒工具层被弱化** → 三层而非两层，bottom tools 与 top predicates 等权
3. **信号管理不够完整** → 量化 / 广播 / 屏蔽 三主轴贯通全文
4. **Goodhart 屏蔽不严** → 明确 public / private / commit-reveal 三分；对人类与审计者白盒，对 Agent 分级可见
5. **PCP 谓词数学过度承诺** → 改为工程化 PCP 谓词原则
6. **经济系统对齐 on_init + CTF 守恒** → on_init 是唯一合法铸币点

---

# TuringOS

## 反奥利奥架构下的可验证状态账本操作系统

### A Verifiable State-Ledger Operating System under the Anti-Oreo Architecture

---

## 摘要

TuringOS 是一种面向自治智能体群体的操作系统架构。它的核心不是让单个 Agent 更聪明，也不是把大模型推理过程强行搬上区块链，而是建立一个能让大量黑盒 Agent 在严格白盒环境中可靠工作的计算世界。

TuringOS 采用 **反奥利奥架构**：

```
⚪ 顶层白盒：信号管理、谓词验证、统计聚合、宪法约束
⚫ 中层黑盒：异质 Agent 群体，负责生成、探索、提出候选状态转移
⚪ 底层白盒：read/write tools、CAS、sandbox、ledger、权限、执行器
```

在该架构中，智能体不直接修改世界状态。它们只能提交候选状态转移。顶层白盒通过确定性谓词与统计信号对候选转移进行量化、广播与屏蔽；底层白盒工具负责读取、写入、执行与记录。系统状态 Q_t 由 tape、HEAD、状态根、账本根与物化视图共同表示。

本文提出 **ChainTape**：一种受宪法约束的可验证 tape 实现。ChainTape 不等同于区块链。区块链只是 ChainTape 在多主体不互信环境下的一种实现。**TuringOS 的第一性原理仍然是反奥利奥架构，而不是区块链。**

---

## 0. 设计公理

TuringOS 建立在六条公理上。

### 公理 0：黑盒负责生成，白盒负责纪律

Agent 可以是概率模型，可以是不透明系统，可以有幻觉。但系统的纪律层不能是幻觉。纪律必须以可执行谓词、可审计工具、确定性状态转移和可追溯日志存在。

### 公理 1：顶层白盒不做微观管理

顶层白盒不是全知独裁者。它不试图理解每个 Agent 的全部内部推理。它只做三件事：

- 量化 quantization
- 广播 broadcasting
- 屏蔽 shielding

### 公理 2：中层黑盒必须保持异质性与隔离性

群体智慧依赖样本独立。若所有 Agent 共享同一上下文、同一中间错误、同一推理轨迹，一万个黑盒会退化为一个黑盒。因此，TuringOS 必须主动屏蔽 Agent 之间的横向相关性。

### 公理 3：底层工具必须白盒、可测试、可拒绝

read tool、write tool、CAS、sandbox、ledger、executor、permission checker 都必须是确定性白盒。工具不解释意图，只执行规则。遇到权限、签名、hash、schema、budget、predicate 不一致时，默认拒绝。

### 公理 4：所有世界状态变化都必须被提案化

Agent 不写入世界。Agent 只提交：

```
proposal / patch / transaction / candidate transition
```

系统只有在谓词验证通过后，才允许 write tool 将其提交为新的状态。

### 公理 5：区块链不是本体，只是 tape 的一种实现

TuringOS 的 tape 是一个可验证状态账本。它可以由本地 hash chain、Git-like CAS、permissioned ledger、rollup、public chain settlement 实现。**区块链不是 TuringOS 的核心，反奥利奥才是核心。**

---

## 1. 问题：为什么现有 Agent 系统会崩溃

大语言模型让 Agent 获得了前所未有的生成能力。但当前 Agent 系统通常犯三个结构性错误。

### 1.1 用自然语言软约束监管黑盒

在提示词里写"不要犯错""遵守架构规范""不要泄露秘密"，不是硬约束。自然语言仍然是黑盒输入的一部分，无法构成系统边界。

宪法已指出：顶层白盒不能依赖语言去约束另一个黑盒，必须把约束转化为机器可执行的硬约束，例如 linter、CI、结构化数据校验和状态谓词。

### 1.2 让 Agent 同时做生成者、记忆者、裁判者

如果 Agent 既生成答案，又修改共享内存，又解释自己是否正确，系统就没有真正的纪律。幻觉一旦写入上下文，会被后续 Agent 当作正确示例继续学习，形成技术债漂移。

### 1.3 把所有计算强行上链

另一个极端是把大模型推理、长上下文、完整日志、全部中间状态都写进区块链。这会造成吞吐量崩溃、隐私泄露、上下文污染和不可删除的数据债。

正确做法是：链只记录状态转移的承诺、验证结果、签名、state root、经济结算与审计证据。**推理在链下，纪律在白盒，提交在账本。**

---

## 2. 图灵机隐喻：纸、铅笔、橡皮、纪律

> "A person provided with paper, pencil, rubber, and subject to strict discipline, is in effect a universal machine."
> 一个被提供了纸、铅笔、橡皮，并受到严格纪律约束的人，本质上就是一台通用机器。

在 TuringOS 中，这四个元素被重新实例化。

```
人       = 中层黑盒 Agent
纸       = tape / world state / content-addressed memory
铅笔     = proposal / patch / write transaction
橡皮     = revert / branch / compensation / bounded key destruction (*)
严格纪律 = predicates / constitution / deterministic tools / signal rules
```

> (\*) **Key destruction caveat (Art 0.2 重建性合规)**: "bounded key destruction" 仅可作用于**链下加密载荷的可达性** —— 链上 / tape 必须保留承诺 (commitment)、metadata、可重建的 public state，使 Art 0.2 "所有信号必须可从 tape 重建" 不被破坏。销毁的对象是访问能力 (access capability)，不是 provenance 记录。详见 § 16.4。

关键不是把"纸"神秘化成区块链，而是把四个要素重新隔离。Agent 是执行计算的人；tape 是外部记忆；write tool 是铅笔；revert 机制是橡皮；谓词与宪法是纪律。

---

## 3. 反奥利奥架构

TuringOS 的核心架构是：

```
⚪ Top White Layer
    predicates
    statistical aggregators
    signal routers
    constitution checks
    budget and price rules

⚫ Middle Black Layer
    heterogeneous agents
    planners
    coders
    testers
    critics
    explorers

⚪ Bottom White Layer
    read tool
    write tool
    CAS object store
    sandbox executor
    ledger
    permission system
    state materializer
```

### 3.1 顶层白盒：管理信号，不管理细节

顶层白盒的任务不是"理解每个 Agent 为什么这么想"，而是把高维黑盒输出压缩成低维、确定、可执行的信号。

它处理两类信号：

- **布尔信号**：0 / 1，是否通过验证
- **统计信号**：[0,∞)，相对价值、信誉、价格、效用

布尔信号确立系统边界。统计信号引导系统演化。

### 3.2 中层黑盒：生成候选状态转移

Agent 不需要完全可解释。它们的价值在于高吞吐量、多样性、探索能力和模式生成能力。

但 Agent 的权力必须被限制为：

```
read context
think privately
produce candidate output
submit proposal
receive feedback
```

Agent 不能直接修改全局状态，不能绕过 write tool，不能读取所有私有谓词，不能污染其他 Agent 的上下文。

### 3.3 底层白盒：把抽象纪律落实为物理动作

底层白盒负责把"规则"变成不可绕过的系统行为。

```
read tool     决定 Agent 能看什么
write tool    决定什么能被写入
CAS           决定内容如何被寻址
ledger        决定历史如何被记录
sandbox       决定动作如何被隔离执行
permission    决定谁能调用什么能力
materializer  决定当前状态如何从历史推导出来
```

如果没有底层白盒，顶层谓词只是口号。如果没有顶层白盒，底层工具只是机械执行器。如果没有中层黑盒，系统没有生成能力。

**三者缺一不可。**

---

## 4. 系统状态

基础状态定义为：

```
Q_t = <q_t, HEAD_t, tape_t>
```

其中：

```
q_t      = 当前控制状态
HEAD_t   = 当前任务路径 / 指针 / cursor
tape_t   = 外部记忆 / 文件系统 / 世界状态
```

> **宪法保留语句 (Art 0.4 不变)**: `Q_t = ⟨q_t, HEAD_t, tape_t⟩` 仍然是 TuringOS 的宪法概念 schema。下方的 ChainTape 8-字段视图**不替代**该 3-元组；它仅把 `tape_t` 在工程实现层投影为若干**可加密承诺的派生根 (cryptographic commitments / projections)**，使外部审计者能在不读完整账本的情况下验证 tape 内部一致性。8-字段中没有任何字段是新增的"额外状态"——每一个都是 `tape_t` 已蕴含的子结构的显式承诺。

在 ChainTape 实现中，`Q_t` 的工程视图扩展为：

```
Q_t (实现视图) = <
  q_t,                          ← Art 0.4 原 q_t
  HEAD_t,                       ← Art 0.4 原 HEAD_t
  state_root_t,                 ← tape_t 当前世界状态的承诺
  tape_view_t,                  ← tape_t 物化视图（agent 可见局部）
  ledger_root_t,                ← tape_t 历史账本的承诺
  budget_state_t,               ← tape_t 预算子结构的承诺
  predicate_registry_root_t,    ← tape_t 谓词注册表的承诺
  tool_registry_root_t          ← tape_t 工具注册表的承诺
>
```

含义如下：

```
state_root_t              当前世界状态根
tape_view_t               对 Agent 可见的物化视图
ledger_root_t             历史账本根
budget_state_t            Coin / YES / NO / stake 状态
predicate_registry_root_t 谓词注册表根
tool_registry_root_t      工具注册表根
```

> **注意**：tape_view_t 不是完整账本。Agent 不应直接读取全量历史。Agent 只能读取经过索引、权限过滤、摘要与去污染处理后的局部视图。

---

## 5. ChainTape：可验证 tape，而非普通区块链

ChainTape 是 TuringOS 的 tape 实现层。它解决的问题不是"如何发币"，而是：

```
谁在什么时候读了什么
谁提出了什么状态转移
哪些谓词接受或拒绝了它
状态是否真的改变
改变是否可回滚
经济成本如何结算
错误是否应被屏蔽或广播
```

### 5.1 ChainTape 的结构：Trust Anchor (Layer 0) + 六层 ChainTape (Layer 1–6)

> **结构说明**: Layer 0 (Constitution Root) 是 ChainTape 之外的**信任锚 (trust anchor)**——它定义了 ChainTape 必须服从的物理法则与价值观，但不是 ChainTape 本身的一层。**ChainTape 真正的实现六层是 Layer 1–6**。这区分对应宪法 Art V.1.1：宪法 (Layer 0) 是 ground truth，ArchitectAI 的演化空间在 ChainTape 六层 (Layer 1–6) 内部。

#### Layer 0：Constitution Root (Trust Anchor — 不属于 ChainTape 六层)

最高信任根。

```
constitution_hash
human_signature
sudo_policy             ← 仅 constitution.md 修改 (Art V.1.1 line 715)
allowed_meta_update_rules
physical_or_hardware_attestation
```

宪法规定的是价值观和物理法则。人类架构师不再规定系统每一步怎么做，而是维护最顶层的 Ground Truth。宪法存放在只读文件系统上，**人类 sudo 权限仅且只作用于 `constitution.md` 本身** (per Art V.1.1)。Trust Root 清单中的其他载荷（kernel.rs / 谓词 / 工具 / cases / 实验等）属于 ArchitectAI 的合法升级范围，经 Veto-AI 审查后无需 human sudo。

#### Layer 1：Predicate Registry

谓词注册表。

```
predicate_id
version
code_hash
input_schema
output_schema
visibility_policy
owner
test_suite_hash
```

谓词分为三类：

- **public predicates**       对 Agent 可见，如 schema、permission、basic tests
- **private predicates**      对 Agent 屏蔽，如隐藏 benchmark、反 Goodhart 评估器
- **commit-reveal predicates** 先提交 hash，后揭示部分测试样本

核心原则：

```
对人类和审计者白盒
对 Agent 分级可见
对状态转移确定执行
```

#### Layer 2：Tool Registry

工具注册表。

```
tool_id
version
capability
input_schema
output_schema
permission_policy
determinism_level
side_effect_class
```

工具分为：

- read-only tools
- write tools
- sandbox tools
- external oracle tools
- execution tools
- meta tools

只有工具能产生系统级副作用。Agent 自身不能绕过工具层直接操作 tape。

#### Layer 3：Content-Addressed Object Store

大文件、代码、文档、模型输出、测试报告、patch 原文，都不应直接写入链。它们应进入内容寻址对象存储。

ChainTape 只需要记录：

```
cid
hash
schema
type
creator
visibility
encryption_policy
```

而不是记录全部内容。

#### Layer 4：Transition Ledger

追加式状态转移账本。

```
tx_id
parent_state_root
agent_id
task_id
read_set
write_set
proposal_cid
predicate_results
stake
signature
timestamp
status
```

这层可以是：

- local hash chain
- append-only JSONL
- Git commit graph
- permissioned blockchain
- public-chain settlement proof

#### Layer 5：Materialized State and Agent Read View

物化状态层。

```
current_state_db
task_index
agent_reputation_index
error_taxonomy_index
price_signal_index
permission_view
```

Agent 实际读取的是这一层，而不是底层完整账本。

```
read_tool(agent_i, task_j, Q_t)
  -> minimal sufficient context
```

这正是选择性屏蔽与渐进式披露。

#### Layer 6：Signal Indices

信号索引层。

```
boolean pass/fail history
typical error clusters
price signals
reputation counters
resource scarcity indicators
exploration/exploitation statistics
```

> **L5 vs L6 区分 (Art 0.2 重建性合规)**: Layer 5 是**当前状态的物化视图**（agent 直接 read_tool 读到的最小充分上下文，含 `agent_reputation_index` / `price_signal_index`）。Layer 6 是**统计聚合 / 趋势压缩**（`reputation counters` 是窗口内增量统计，`price signals` 是市场微观结构压缩）。**L6 严格 derivable from L4 + L5**：删掉 L6 不破坏 Art 0.2 重建性，因为 L4 (transition ledger) 是 ground truth，L5 是 L4 物化，L6 是 L5 统计变换。L6 存在的唯一目的是**降低顶层白盒访问成本**（从 O(账本扫描) 降到 O(预聚合查询)）。

这层让顶层白盒能够持续进行量化、广播、屏蔽。

---

## 6. 状态转移协议

一次标准计算循环如下。

### 6.1 Read

```
input_i = rtool(Q_t, agent_i, task_j)
```

read tool 只返回当前任务需要的最小充分上下文。

```
input_i = <q_i, s_i, budget_view_i, allowed_tools_i>
```

其中：

```
q_i = 当前控制状态摘要
s_i = 局部 tape view
```

### 6.2 Generate

```
output_i = δ_i(input_i)
```

δ_i 是中层黑盒 Agent。它可以推理、搜索、写草稿、运行私有模拟，但这些内部过程不自动成为世界状态。

### 6.3 Propose

Agent 将输出封装为候选状态转移：

```json
{
  "parent_state_root": "...",
  "agent_id": "...",
  "task_id": "...",
  "read_set": [...],
  "write_set": [...],
  "proposal_cid": "...",
  "requested_tools": [...],
  "stake": {...},
  "signature": "..."
}
```

### 6.4 Verify

顶层白盒运行谓词集合：

```
valid_i = Π p_k(tx_i | Q_t)
```

谓词包括：

```
schema_predicate
permission_predicate
budget_predicate
version_predicate
no_forbidden_write_predicate
sandbox_test_predicate
reversibility_predicate
constitution_predicate
goodhart_shield_predicate
```

若完美谓词可得，直接使用完美谓词。若完美谓词不可得，则使用工程化 PCP 谓词：**正确候选应尽量不被误杀，错误候选应高概率被拦截。**

### 6.5 Commit or Reject

若通过：

```
Q_{t+1} = wtool(Q_t, tx_i)
ledger.append(accepted_tx_i)
materialized_view.update(Q_{t+1})
```

若失败：

```
Q_{t+1} = Q_t
ledger.append(rejected_tx_i)
agent_reputation.update(agent_i, penalty)
error_router.route(error)
```

失败记录可以进入底层账本，但不能自动进入所有 Agent 的上下文。是否广播，要由顶层白盒判断。

---

## 7. 信号的量化

量化是顶层白盒的第一职责。

### 7.1 布尔信号

布尔信号回答：

```
是否通过？
```

形式为：

```
p: X -> {0,1}
```

它不解释，不安慰，不妥协。

```
0 = 拒绝
1 = 接受
```

布尔谓词适用于：

```
schema 合法性
权限合法性
签名合法性
预算合法性
测试是否通过
是否违反宪法
是否可回滚
是否触碰 forbidden path
```

### 7.2 安全域与创造域的不同拒绝原则

TuringOS 必须区分两个域。

**创造域**：

```
代码生成
UI 方案
文本方案
研究假设
开放式搜索
```

创造域中，系统应偏向"疑罪从无"：不要轻易误杀正确候选，让大量候选进入后续统计筛选。

**安全域**：

```
宪法修改                  ← 仅此一类需 human sudo (Art V.1.1)
权限提升                  ← Veto-AI + multi-sig + challenge window
外部转账                  ← Veto-AI + signature + 经济抵押 (Art II.2)
不可逆状态变更            ← reversibility predicate + two-phase commit + time lock
密钥访问                  ← typed restricted API + Veto-AI 审 + Argon2id KDF (CO1.7.0a-f)
生产部署                  ← deterministic policy + Veto-AI + canary + rollback predicate
```

> **Sudo 范围严格收窄 (per Art V.1.1 line 715)**: 在所有"安全域"事件中，**只有 `宪法修改` 一项**需要 human sudo。其余条目使用**deterministic policy 谓词 + Veto-AI 否决审 + 多重签名 / 挑战窗口 / 外部法律控制**等机器化纪律。把 human sudo 扩展到非宪法领域 = 把人类拉回工程细节，违反 Art V.1.1 "人类只立宪法" 的 Meta 原则。

安全域中，系统必须 fail closed：不确定就拒绝。

**这不是矛盾，而是风险分层。**

### 7.3 统计信号

统计信号回答：

```
相对价值是多少？
```

形式为：

```
s: X -> [0,∞)
```

统计信号包括：

```
reputation
reuse_count
median_consensus
expected_utility
variance
price
scarcity
latency
failure_rate
```

统计信号不是主观评价，而是白盒算法对群体行为的压缩。市场价格就是典型统计信号：系统不关心个体为什么出价，只记录大量交易交汇形成的数值。

---

## 8. 信号的选择性广播

广播不是群发日志。广播是对群体搜索方向的调节。

### 8.1 广播个体错误

当某个 Agent 失败时，系统应向该 Agent 返回具体错误：

```
schema mismatch
test failed
permission denied
budget insufficient
predicate p_k = 0
```

这类错误只给失败者，不给所有 Agent。

### 8.2 广播典型错误

若多个 Agent 在同一类问题上反复失败，顶层白盒应执行：

```
cluster errors
abstract rule
update documentation
update predicate
broadcast distilled rule
```

广播的是抽象规则，不是污染性报错全文。

### 8.3 广播价格信号

价格信号告诉 Agent：

```
系统现在需要什么
什么资源稀缺
什么任务价值更高
什么方向已有过度拥挤
```

价格信号不告诉 Agent 具体怎么做。它只改变群体注意力分布。

### 8.4 维持探索与利用平衡

若价格信号过强，所有 Agent 会涌向局部最优。若价格信号过弱，群体无法收敛。TuringOS 需要通过配额、随机化、冷门任务补贴、agent diversity policy 来维持探索与利用平衡。

---

## 9. 信号的选择性屏蔽

屏蔽是 TuringOS 的生命线。没有屏蔽，账本会变成污染源。

### 9.1 屏蔽错误

失败记录可以写入底层 ledger，但不能默认进入 Agent 上下文。

```
ledger stores evidence
read view hides contamination
error index stores abstraction
```

### 9.2 屏蔽细节

Agent 不需要读完整宪法、完整账本、完整工具源码、完整历史失败轨迹。它需要的是：

```
当前任务所需的最小充分上下文
```

顶层应提供目录接口，而不是百科全书式上下文。

### 9.3 屏蔽相关性

并发 Agent 之间不应共享实时中间状态。否则群体输出会高度相关，统计信号失效。

```
same task
different seeds
different contexts
different tool subsets
different exploration priors
```

### 9.4 屏蔽 Goodhart

当度量成为目标，它会被优化到失效。

因此：

```
public predicates       处理基础合法性
private predicates      处理质量与隐藏评估
commit-reveal tests     防止 benchmark 泄漏
rotating adversarial sets 防止长期过拟合
```

谓词对系统审计者白盒，但对中层黑盒分级屏蔽。

---

## 10. 经济纪律

TuringOS 的经济系统不是为了金融化，而是为了让写入行为承担成本。

宪法基本法：

```
Law 1: Information is Free
Law 2: Only Investment Costs Money
1 Coin = 1 YES + 1 NO
on_init 是唯一合法铸币点
```

### 10.1 Information is Free

读取、搜索、观察、内部思考原则上不消耗核心资产。

原因是：探索必须便宜。若读取也昂贵，群体会过早收敛，搜索空间会萎缩。

### 10.2 Only Investment Costs Money

写入、提交、占用全局状态、请求外部副作用、改变未来计算轨迹，必须付费或质押。

```
read              = free
think             = free
draft             = free
commit            = costly                              (stake + Veto-AI)
deploy            = costly + Veto-AI canary             (rollback predicate)
constitutional sudo = highly costly + human-only gate   (Art V.1.1, 仅此项触发 human sudo)
non-constitutional irreversible = costly + Veto-AI + multi-sig + challenge window + time lock
                                                        (无 human sudo)
```

> **价目表与 Art V.1.1 一致**: human sudo 是单一价目项，专属 `constitution.md` 修改。所有其他"昂贵"操作（包括 deploy / 不可逆写 / 外部副作用）使用 Veto-AI + 经济 + 时间锁组合，不消耗人类带宽。

### 10.3 Coin / YES / NO 守恒

`1 Coin = 1 YES + 1 NO` 表示每次投资都必须面对正反两面的经济约束。Agent 不能凭空制造信心，也不能只购买赞成而不承担失败风险。

### 10.4 on_init 唯一铸币点

系统启动时，通过 on_init 初始化预算、任务市场和初始权重。此后，经济状态只能通过合法状态转移改变，不能由 Agent 任意增发。

---

## 11. Boot：创世状态

Boot 不是启动一个聊天机器人。Boot 是把人类规范编译为机器可执行的顶层白盒规则，并初始化第一份世界状态。

```
Boot = 初始化顶层白盒规则 + 初始世界状态
```

其本质是把人类 spec 编译成 predicates，并写入系统信任根，使黑盒 Agent 能在验证约束下持续演化世界状态。

### 11.1 Genesis Block

ChainTape 的创世块包含：

```
genesis_block = {
  constitution_hash,
  human_signature,
  initial_predicate_registry_root,
  initial_tool_registry_root,
  initial_state_root,
  initial_budget_state,
  on_init_coin_supply,
  boot_time,
  boot_attestation
}
```

### 11.2 初始化过程

```
human architect provides spec
InitAI compiles spec into predicates and tools
Veto-AI checks constitutional constraints
system creates Q_0
on_init mints initial Coin state
ledger writes genesis block
runtime enters loop
```

### 11.3 Boot 后的原则

Boot 后，系统不依赖人类微观干预。人类不再批准每个业务动作。**人类只维护宪法、sudo、信任根与物理边界。**

---

## 12. Go Meta：架构的架构

TuringOS 不能永远受限于初始人类 spec。系统必须能从自身失败中提取白盒知识，并升级工具、谓词与流程。

```
Go Meta：架构的架构
```

其核心是：系统要能"自己给自己搭架构"。过去黑盒的试错与教训，应被提取为更明确的提示词、更清晰的工具、更完备的验证代码。

### 12.1 三权分立

#### Constitution：唯一 Ground Truth

宪法规定价值观与物理法则。

```
what must never be violated
what must remain reversible
what must remain deterministic
what requires human sudo
```

#### ArchitectAI：提出者

ArchitectAI 是激进改革派。它读取日志、失败簇和系统瓶颈，提出：

```
new predicates
new tools
new storage layout
new read view
new sandbox policy
new signal routing rule
```

它是系统熵减引擎。

#### Veto-AI：否决者

> **命名说明**: 此前在 v2 round-1 草稿中称 `JudgeAI`；按 constitution.md Art V.3 line 812 (2026-04-25 amendment) 的正式重命名，本版统一使用 **Veto-AI**。`JudgeAI` 仅作为历史 alias 保留语义可追溯性。

Veto-AI 是保守守门人。它不做全面主观评价，只做一件事：

```
否决违宪提案
```

Veto-AI 的唯一工作是拿着宪法逐条校验 ArchitectAI 生成的新架构代码是否违宪；它不承担其他主观评价。输出域严格 = `{PASS, VETO}`，不做主观质量 / 性能 / 可读性评判（per constitution.md Art V.1.3 白名单严格排除）。

#### Human Architect：宪法维护者

人类架构师不再规定系统每一步怎么做，而是规定最顶层目标与价值观。人类维护宪法、sudo、物理边界和最终 Ground Truth。

### 12.2 Meta Transition

架构升级本身也是状态转移。

```json
{
  "parent_architecture_root": "...",
  "proposed_predicate_patch": "...",
  "proposed_tool_patch": "...",
  "evidence_from_logs": "...",
  "reversibility_plan": "...",
  "constitution_check": "...",
  "veto_signature": "...",
  "human_signature_if_required": "..."
}
```

只有通过 Meta 谓词后，系统才允许升级自身。

### 12.3 不可破坏的不变量

任何 Meta 升级都不得破坏：

```
反奥利奥三层结构
Agent 不直接写世界状态
read/write 分离
谓词可审计
核心状态可回滚
错误屏蔽机制
Goodhart 屏蔽机制
on_init 唯一铸币点
人类宪法 sudo
```

---

## 13. 区块链在 TuringOS 中的位置

区块链不是 TuringOS 的灵魂。它是 ChainTape 的一种部署形态。

### 13.1 本地 HashChainTape

单人或单组织阶段：

```
CAS object store
append-only ledger.jsonl
prev_hash chain
SQLite/Postgres materialized view
Python predicates
local signatures
```

这是最小可行版本。

### 13.2 GitTape

当系统以文件与 patch 为主时：

```
Git object store
commit graph
branch
merge
revert
worktree materialized view
```

GitTape 适合早期 TuringOS，因为它天然支持内容寻址、版本控制和回滚。

### 13.3 Permissioned ChainTape

当多个团队、组织、节点之间不完全互信时，引入许可链。

在这种形态中：

```
endorsement policy = 多方背书谓词
world state        = 当前 tape materialized view
blockchain         = 状态转移历史
channel            = 权限隔离域
```

### 13.4 Public Settlement

开放生态阶段，公链只做最终结算：

```
stake lock
reward settlement
cross-domain reputation anchor
state root checkpoint
dispute resolution
```

高频 Agent 推理、上下文读取、代码执行、测试运行，仍然在链下完成。

### 13.5 禁止事项

TuringOS 禁止以下设计：

```
把完整 prompt 上链
把完整推理日志上链
把私有谓词上链公开
把用户隐私数据上链
把密钥或 secret 上链
把错误上下文广播给所有 Agent
把全部 Agent 计算串行化到一条链
把区块链共识误认为事实真理
```

**共识只能证明"大家同意这条记录被写入"，不能证明"这条记录描述的现实是真的"。**

---

## 14. 数据结构示例

### 14.1 Proposal Object

```json
{
  "type": "proposal",
  "agent_id": "agent:coder:017",
  "task_id": "task:build-parser",
  "parent_state_root": "sha256:...",
  "read_set": [
    "cid:spec-v3",
    "cid:parser-tests-v2"
  ],
  "write_set": [
    {
      "path": "/src/parser.py",
      "cid": "cid:new-parser"
    }
  ],
  "requested_tools": [
    "sandbox.python.pytest",
    "wtool.patch.apply"
  ],
  "stake": {
    "coin": 1,
    "yes": 1,
    "no": 1
  },
  "signature": "agent-signature"
}
```

### 14.2 Predicate Result

```json
{
  "predicate_id": "pytest_passes",
  "version": "1.4.2",
  "input_tx": "tx:...",
  "result": 1,
  "evidence_cid": "cid:test-report",
  "runtime_ms": 1832
}
```

### 14.3 Ledger Entry

```json
{
  "tx_id": "tx:sha256:...",
  "prev_hash": "sha256:...",
  "parent_state_root": "sha256:...",
  "next_state_root": "sha256:...",
  "agent_id": "agent:coder:017",
  "proposal_cid": "cid:proposal",
  "predicate_results": [
    "cid:predicate-result-1",
    "cid:predicate-result-2"
  ],
  "status": "accepted",
  "timestamp": "2026-04-26T00:00:00Z",
  "signature": "system-signature"
}
```

### 14.4 Rejection Entry

```json
{
  "tx_id": "tx:sha256:...",
  "prev_hash": "sha256:...",
  "parent_state_root": "sha256:...",
  "next_state_root": "sha256:parent-unchanged",
  "agent_id": "agent:coder:018",
  "proposal_cid": "cid:bad-proposal",
  "failed_predicates": [
    {
      "predicate_id": "no_forbidden_write",
      "result": 0,
      "reason": "attempted write to /constitution"
    }
  ],
  "status": "rejected",
  "broadcast_policy": "private_to_agent",
  "timestamp": "2026-04-26T00:00:00Z"
}
```

---

## 15. 最小可行实现

第一版 TuringOS 不应从公链开始。

### 15.1 目录结构

```
/turingos
  /constitution
    constitution.md
    constitution.hash
    human.sig
  /predicates
    schema_valid.py
    permission_valid.py
    budget_valid.py
    tests_pass.py
    reversible.py
    no_goodhart_leak.py
  /tools
    rtool.py
    wtool.py
    sandbox.py
    cas.py
    ledger.py
    materializer.py
  /objects
    sha256/
      ...
  /ledger
    ledger.jsonl
  /state
    current_state.db
    task_index.db
    reputation.db
    price_index.db
  /views
    agent_scoped_contexts/
```

### 15.2 主循环

```python
while True:
    task = scheduler.next_task()
    input_view = rtool.read(
        state=Q_t,
        agent=agent_i,
        task=task
    )
    proposal = agent_i.generate(input_view)
    proposal_cid = cas.put(proposal)
    tx = build_transaction(
        parent_state_root=Q_t.state_root,
        proposal_cid=proposal_cid,
        agent=agent_i,
        stake=stake
    )
    results = predicate_runner.verify(tx, Q_t)
    if all(r.result == 1 for r in results):
        Q_t = wtool.commit(Q_t, tx, results)
        ledger.append_accepted(tx, results, Q_t)
    else:
        ledger.append_rejected(tx, results, Q_t)
        signal_router.route_failure(agent_i, results)
```

### 15.3 MVP 验收标准

MVP 不以"智能多强"为验收标准，而以**结构是否正确**为验收标准。

必须证明：

```
Agent 无法直接写 tape
Agent 无法读取完整 ledger
失败输出不会污染其他 Agent
通过谓词的 tx 会推进 state_root
未通过谓词的 tx 不改变 state_root
ledger 删除任意中间项会破坏 hash chain
状态可从 ledger 重建
典型错误能被抽象广播
私有谓词不会暴露给 Agent
on_init 之外不能铸币
```

---

## 16. 安全边界与失败模式

### 16.1 Ledger Poisoning

错误内容若永久写入账本，可能成为长期污染源。

**缓解**：

```
链上只写 commitment
敏感内容链下加密
Agent read view 默认不读 rejected raw logs
错误只以抽象形式进入广播
```

### 16.2 Predicate Gaming

Agent 可能优化公开谓词。

**缓解**：

```
public/private predicate split
hidden benchmark
rotating tests
commit-reveal evaluation
multi-agent adversarial review
```

### 16.3 Correlated Collapse

Agent 共享错误上下文会导致群体同质化。

**缓解**：

```
context isolation
randomized read views
independent sampling
delayed cross-agent sharing
diversity-preserving scheduler
```

### 16.4 Irreversible Writes

某些状态变更无法真正回滚。

**缓解 (per Art V.1.1 sudo 严格收窄 + Art 0.2 重建性合规)**：

```
reversibility predicate
two-phase commit
time lock
Veto-AI review + multi-signature + challenge window  ← 替代旧版 "human sudo for irreversible class"
                                                       (human sudo 仅适用于 constitution.md 修改)
compensating transaction
external legal / contractual control                  ← 链外不可逆操作 (e.g., 真实世界转账)
bounded key destruction (off-chain payload only)      ← 仅销毁加密载荷可达性；
                                                       链上 / tape 保留 commitment + metadata，
                                                       Art 0.2 "所有信号必须可从 tape 重建" 不被破坏
```

> **设计原理**: Codex 双审 round-1 (2026-04-27) Q3 标记"human sudo for irreversible class" 与宪法 Art V.1.1 line 715 冲突 (sudo 仅作用于 constitution.md)。v2.1 patch 用 **Veto-AI + 多重签名 + 挑战窗口 + 外部法律控制** 组合替代 human sudo，使非宪法不可逆操作不消耗人类带宽，同时仍 fail-closed 安全域纪律。

### 16.5 Consensus Fallacy

多节点同意不等于事实为真。

**缓解**：

```
oracle provenance
multi-source evidence
delayed settlement
challenge window
human escalation for high-impact claims
```

---

## 17. 实施路线

### Phase 1：GitTape

**目标**：验证反奥利奥闭环。

```
local CAS
append-only ledger
Python predicates
SQLite state
two or more isolated Agents
```

### Phase 2：LedgerTape

**目标**：加入身份、信誉、经济成本。

```
agent signatures
stake
YES/NO accounting
reputation counters
task bounty
```

### Phase 3：MetaTape

**目标**：启动架构自我改进。

```
ArchitectAI proposes predicates/tools
Veto-AI vetoes unconstitutional changes
human sudo gates constitution-level changes
meta ledger records architecture transitions
```

### Phase 4：Permissioned ChainTape

**目标**：支持多组织协作。

```
endorsement policies
multi-party validators
separate channels
shared state roots
auditable governance
```

### Phase 5：Public Settlement

**目标**：开放生态结算。

```
public checkpoints
stake settlement
cross-domain reputation
dispute resolution
rollup / L2 / state channel settlement
```

### 17.6 Plan-of-Record 边界声明 (Plan v3.2 crosswalk)

> **§ 17 5-phase 叙事 vs CO_MEGA_PLAN_v3.2 atom-level plan**: 二者**不是替代关系，是层叠关系**。
>
> | 维度 | § 17 (本节) | CO_MEGA_PLAN_v3.2 (handover/architect-insights/) |
> |---|---|---|
> | 抽象层 | 叙事 / phase 序列 / 工程目标 | 原子 / 依赖 / 调度 / 预算 |
> | 控制范围 | "下一个 wave 应朝哪里走" | "下一个 atom 是哪个 CO-id" |
> | 单位 | 5 phase | ~170 atoms |
> | 可变性 | v2.x 修订即可 | 需 Plan v3.x 单独修订 + dual audit |
> | 当 § 17 与 Plan 冲突时 | § 17 控制叙事方向 | Plan v3.2 控制 atom 顺序与依赖 |
>
> **明文裁决**: CO_MEGA_PLAN_v3.2 仍是**唯一 atom-level plan of record**。 § 17 不重写、不重排、不重编号其 ~170 atom；它只为 atom 群体提供 phase 标签（GitTape / LedgerTape / MetaTape / PermissionedChainTape / PublicSettlement）。
>
> 例如：CO1.7 transition_ledger atom 在 Plan v3.2 中保持原 CO-id 不变，但在 § 17 narrative 中归入 **Phase 2 LedgerTape**。这是**标签**关系，不是**重命名**关系。
>
> 如果未来 § 17 与 Plan v3.2 发生不可调和的冲突，按 `WHITEPAPER_v2_TACTICAL_ALIGNMENT § 10` 的冲突解决流程处理（升级到 human architect 仲裁，不允许任一方隐式覆盖另一方）。

---

## 18. 结论

TuringOS 的核心不是"让 AI 上链"。它的核心是：

```
用白盒环境约束黑盒智能
用信号工程管理群体搜索
用状态账本记录世界转移
用宪法限制系统自我演化
```

**区块链可以增强 TuringOS，但不能定义 TuringOS。**
**ChainTape 可以成为 tape 的实现，但不能取代反奥利奥架构。**
**Agent 可以生成智能，但不能拥有最终写入权。**
**谓词可以裁决状态转移，但不能退化为另一个黑盒。**
**人类可以退出微观管理，但不能放弃宪法根。**

最终定义如下：

> TuringOS 是一个宪法约束下的反奥利奥操作系统。中层黑盒 Agent 负责提出候选状态转移；顶层白盒负责将其量化、广播与屏蔽；底层白盒工具负责读取、写入、执行与记录。其 tape 是可验证状态账本，区块链只是该账本在不互信多主体环境下的一种实现。

---

## 附录 A：与现有实现对应表（信息性，非规范性）

| v2 §  | 现有实现 / 计划 | 状态 |
|---|---|---|
| § 5.1 Layer 0 Constitution Root | `src/boot.rs::verify_constitution_root_section` + `genesis_payload.toml` | ✅ Wave 1 已实现 |
| § 5.1 Layer 1 Predicate Registry | `src/top_white/predicates/{visibility,registry}` | ✅ Wave 2 已实现 (CO1.5) |
| § 5.1 Layer 2 Tool Registry | `src/bottom_white/tools/registry` | ✅ Wave 2 已实现 (CO1.6) |
| § 5.1 Layer 3 CAS | `src/bottom_white/cas/{schema,store}` | ✅ Wave 3 已实现 (CO1.4) |
| § 5.1 Layer 4 Transition Ledger | CO1.7 transition_ledger | 🔄 Wave 6 计划 |
| § 5.1 Layer 5 Materialized View | CO1.7.5 materializer | 🔄 后续 wave |
| § 5.1 Layer 6 Signal Indices | reputation / price / error taxonomy | 🔄 后续 wave |
| § 7.2 创造域 vs 安全域 | OBS_WHITEPAPER_V2_DUAL_DOMAIN — 待沉淀 case | 📅 待裁决 |
| § 9.4 Public/Private/CommitReveal | CO1.5 visibility enum | ✅ 已存在；commit-reveal 路径未实现 |
| § 10.4 on_init 唯一铸币点 | `src/economy/money.rs` MicroCoin + Inv 3 守恒 | ✅ Wave 1 已验证 |
| § 11.2 InitAI | OBS_WHITEPAPER_V2_INITAI_PLACEHOLDER | 📅 conceptual placeholder |
| § 17 5-Phase Roadmap | CO_MEGA_PLAN_v3.2 (~170 atoms) — **Plan-of-record** | 📅 § 17 控叙事，Plan v3.2 控原子；冲突解决见 § 17.6 + Tactical Alignment Note § 10 |

> 详细差异分析见 `handover/alignment/WHITEPAPER_v2_TACTICAL_ALIGNMENT_2026-04-27.md`

---

**文档结束**

文件 SHA256（自填，commit 时计算）: `<computed-on-commit>`
