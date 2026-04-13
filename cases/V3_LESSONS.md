# V3 Lessons — 完整教训列表

从 TuringOS v3 三个月开发中提取的全部教训。每条教训有唯一 ID，映射到宪法条款和判例。

> 本文件是判例库的可审计基础。任何新增判例必须在此文件中有来源追溯。

---

## Art. I — 信号的量化

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-01 | sorry 作弊：Lean 接受 sorry 伪造证明，需三层防御 (upstream/middle/fallback) | Art. I.1 | C-015 | bus.rs forbidden_patterns |
| V3L-02 | Oracle 非确定性：同一输入 Lean 返回不同结果，验证器必须确定性 | Art. I.1 | C-016 | experiments/minif2f_v2 |
| V3L-03 | decide/omega 暴力穷举伪装成证明 (Finset.range 100) | Art. I.1 | C-011 | experiments/number_theory_min |
| V3L-04 | 度量工具的正确性 > 被度量对象的优化 (parse_tape 空行 bug 导致 10 次实验追错) | Art. I.2 | C-012 | scripts/parse_tape.py |
| V3L-05 | 虚假深度：depth=25 实际 1% novelty，去重后 depth=14 novelty=100% | Art. I.2 | C-013 | experiments/zeta_sum_proof |
| V3L-06 | 全局去重杀死跨分支复用 (DCT lemma 无法在不同证明分支中重复使用) | Art. I.2 | C-013 | experiments/minif2f_v2 |
| V3L-07 | 身份盗用：DeepSeek 证明了错误定理 (induction_11div10tonmn1ton)，Lean 验证通过 | Art. I.1.1 | C-014 | experiments/minif2f_swarm |
| V3L-08 | LLM 输出格式三层静默失败 (JSON prefix / LaTeX escape / bare tag) | Art. I.1.1 | C-009 | sdk/protocol.rs |

## Art. II — 信号的选择性广播

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-09 | 静默失败比崩溃更危险：parse 返回 None、JoinSet 耗竭、JSON 吞错 | Art. II.1 | C-017 | sdk/protocol.rs, actor.rs |
| V3L-10 | 钱包语义污染：balance=5990 报 "Bankrupt"，混淆真破产与保证金不足 | Art. II.1 | C-018 | sdk/tools/wallet.rs |
| V3L-11 | 价格波形坍缩：并发定价造成振荡，串行 reactor 保证因果 | Art. II.2 | C-019 | bus.rs tick_map_reduce |
| V3L-12 | 阈值过时：1.5x surge 阈值冻结价格在 9000，底层成本已降应移除 | Art. II.2 | C-020 | bus.rs ThermodynamicHeartbeatTool |
| V3L-13 | 聪明模型产生浅证明：7B depth=14, 14B depth=4，智力无约束则发散 | Art. II.2.1 | C-021 | experiments/zeta_sum_proof |
| V3L-14 | 贪心 ArgMax 路由 → 星形拓扑 → 所有 agent 收敛同一局部最优 | Art. II.2.1 | C-005 | sdk/actor.rs |

## Art. III — 信号的选择性屏蔽

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-15 | 上下文自毒：raw `<think>` 块泄入下一 agent 上下文 | Art. III.1 | C-022 | sdk/protocol.rs |
| V3L-16 | 双腔架构：不约束认知 (user space)，在膜处提取确定性 (kernel space) | Art. III.2 | C-023 | sdk/protocol.rs rfind |
| V3L-17 | 截断悖论：2048 token 硬限截断推理链，挤压物理层而非能力层 | Art. III.2 | C-023 | drivers/llm_http.rs |
| V3L-18 | KV Cache 雪崩：4 agents x 8192 ctx = VRAM 溢出，量化缓存而非降上下文 | Art. III.2 | C-023 | llama-server config |
| V3L-19 | 投资挤出建设：300 tx 中 286 投资、33 append，需独立资源池 | Art. III.3 | C-024 | bus.rs append vs invest |
| V3L-20 | 注意力稀释：混合模型格式碎片化，同质化买认知清晰度 | Art. III.3 | C-025 | experiments/ multi-model |
| V3L-21 | 多步打包/前置运行：agent 在单 append 中打包多步骤垄断路径 | Art. III.4 | C-026 | bus.rs payload limits |
| V3L-22 | Falsifier 可买 YES token — Goodhart 漏洞，角色激励冲突 | Art. III.4 | C-006 | sdk/tools/wallet.rs |

## Art. IV — Boot

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-23 | 硬编码参数诅咒：120s timeout、500 coin 税、example 值成锚点 | Art. IV | C-027 | bus.rs, prompt.rs |
| V3L-24 | 临时文件系统数据丢失：Run 6 OMEGA tape 在 /tmp/ 被覆盖 | Art. IV | C-028 | experiments/run_experiment.py |
| V3L-25 | reqwest+rustls macOS 死锁 (6 种方案均失败，改用 Python proxy) | Art. IV | C-007 | drivers/llm_http.rs |
| V3L-26 | 单线程 proxy 502：15 agent 并发只有 2 成功 | Art. IV | C-008 | drivers/llm_proxy.py |
| V3L-27 | SiliconFlow API N=30 → 401/429 崩溃 | Art. IV | C-027 | drivers/llm_http.rs |
| V3L-28 | Gemini CLI Agent Node.js OOM (subprocess 无限内存) | Art. IV | C-027 | external tooling |
| V3L-29 | KV cache slot 不匹配 swarm 并发数 (llama-server -np) | Art. IV | C-027 | llama-server config |
| V3L-30 | 网络拓扑 Tailscale MTU 卡死，需 LAN bridge 中转 | Art. IV | — | ops/network |
| V3L-31 | JoinSet 耗竭/静默退出：全 agent None → exit 0，需 supervisor loop | Art. IV | C-029 | sdk/actor.rs |
| V3L-32 | 贪心独裁+流动性陷阱组合拳：Run 1 35min 0 OMEGA 完全崩溃 | Art. IV | C-030 | Run 1 postmortem |

## Art. V — Go Meta

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-33 | 制度设计 > 参数调优：参数 <20% 增益，制度变更 >50% | Art. V.1 | C-031 | experiments/ comparison |
| V3L-34 | Oracle 模式混淆：formal/non-formal 混用断裂因果链 (Rule 22 v2) | Art. V.1 | C-032 | bus.rs oracle logic |
| V3L-35 | 因果归因缺失：外部组件做数学却宣称"群体涌现" | Art. V.1 | C-033 | experiments/minif2f_v2 |
| V3L-36 | 过度对齐教条：复制旧 Oracle 未从第一性原理重审 | Art. V.1 | C-034 | v2→v3 migration |
| V3L-37 | 宪法违规零容忍：Run 6 100B mint，4 次审计标注"可接受" (Rule 18) | Art. V.2 | C-001 | Run 6 audit |
| V3L-38 | Generator=Evaluator：同一 AI 写码+审计，4 次放过违宪 (Rule 23) | Art. V.1 | C-010 | v3 audit process |
| V3L-39 | prompt 规则解释无效：LLM 跟随激励而非解释，规则编码到 tool 约束 | Art. V.1 | C-034 | sdk/prompt.rs |
| V3L-40 | hand-coded example 值成锚点：agent 复制示例金额而非自主发现 | Art. V.2 | C-027 | sdk/prompt.rs |

## 经济引擎 — Law 1 & Law 2

| ID | 教训 | 宪法条款 | 判例 | 来源 |
|----|------|---------|------|------|
| V3L-41 | fund_agent 创世后印钞违反 CTF 守恒 | Law 2 | C-001 | bus.rs fund_agent |
| V3L-42 | redistribute_pool 重生注入 10K 是隐性铸币 | Law 2 | C-002 | bus.rs redistribute |
| V3L-43 | 重生无限注入 (fund_agent(10000)) 每代都印钱 | Law 2 | C-002 | bus.rs rebirth |
| V3L-44 | 固定 500 coin 税 → 通缩死锁 | Law 2 | C-027 | bus.rs tax logic |
| V3L-45 | kernel.rs 硬编码 "[OMEGA]" 违反零领域知识 | Law 1 | C-004 | kernel.rs |
| V3L-46 | Oracle 拦截中间步骤 → Engine 分离破坏 | Law 1 | C-003 | bus.rs oracle |
| V3L-47 | Engine 2 (市场) 逻辑混入 Engine 3 (Oracle) | Law 1 | C-003 | bus.rs |
| V3L-48 | Kelly 准则缺失：agent 全押高价节点，一次失败全灭 | Law 2 | C-026 | sdk/prompt.rs |
| V3L-49 | Lamarckian autopsy 幻觉：个体"学习"=后验合理化，群体 DNA 才有效 | Art. I.2 | C-012 | sdk/actor.rs graveyard |
| V3L-50 | 涌现声称无因果证明：外部组件触碰数学却归功于 swarm | Art. V.1 | C-033 | experiments/ |

---

## 覆盖统计

| 宪法条款 | 教训数 | 判例数 |
|---------|--------|--------|
| Art. I.1 | 3 (V3L-01,02,03) | 3 (C-011,C-015,C-016) |
| Art. I.1.1 | 2 (V3L-07,08) | 2 (C-009,C-014) |
| Art. I.2 | 3 (V3L-04,05,06) + V3L-49 | 2 (C-012,C-013) |
| Art. II.1 | 2 (V3L-09,10) | 2 (C-017,C-018) |
| Art. II.2 | 2 (V3L-11,12) | 2 (C-019,C-020) |
| Art. II.2.1 | 2 (V3L-13,14) | 2 (C-005,C-021) |
| Art. III.1 | 1 (V3L-15) | 1 (C-022) |
| Art. III.2 | 3 (V3L-16,17,18) | 1 (C-023) |
| Art. III.3 | 2 (V3L-19,20) | 2 (C-024,C-025) |
| Art. III.4 | 2 (V3L-21,22) | 2 (C-006,C-026) |
| Art. IV | 10 (V3L-23~32) | 6 (C-007,C-008,C-027~C-030) |
| Art. V.1 | 6 (V3L-33~36,38,39) | 5 (C-010,C-031~C-034) |
| Art. V.2 | 2 (V3L-37,40) | 1 (C-001*) |
| Law 1 | 3 (V3L-45,46,47) | 2 (C-003,C-004) |
| Law 2 | 5 (V3L-41~44,48) | 3 (C-001,C-002,C-026) |

*C-001 同时覆盖 Law 2 和 Art. V.2 (零容忍原则)

**总计**: 50 教训 → 35 判例 (11 现有 + 24 新增)
**零覆盖条款**: 0
