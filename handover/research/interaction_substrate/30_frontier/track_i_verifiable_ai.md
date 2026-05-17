# Phase 3 / Track I — 可验证 AI / 去中心化基础设施

> Frontier research for TISR 2026-05-17. All findings are **forward-bound**: they describe candidate extension surfaces for TuringOS, NOT current landed scope. Time window: 2025-Q1 → 2026-Q2.

---

## 0. forward-bound 警告

本文件全部内容是 **TISR Phase 3 调研** 产物，用于 Phase 4-5 设计输入。

- 任何"TuringOS 应该 X"形式的陈述都是 **proposal / extension surface**，不是 ratified scope。
- 任何"zkML / TEE / DID 已成熟"陈述都是 **outside-of-tape evidence**，进入 TuringOS 前必须经 FC3 Architect/Veto-AI 路径。
- 本文件不修改 ChainTape / CAS / constitution / 任何运行时代码。
- 本文件与 constitution.md / FC1-3 / Constitution Landing Gate **零冲突**：所有候选扩展项必须在引入前通过 Constitution Execution Matrix 落地为 LANDED 行。

---

## 1. 任务 scope + WebSearch 清单

**主线问题** (5):

- Q1 zkML 进展 vs TuringOS Merkle commit
- Q2 TEE for agent runtime (Apple PCC / AWS Nitro / Intel TDX)
- Q3 Bittensor / Fetch.ai 经济模型 vs TuringOS 内生市场
- Q4 Anthropic Constitutional AI training-time vs TuringOS runtime constitution
- Q5 AI agent 身份可携带性 + 跨实例联邦

**WebSearch executed** (10 queries, 2025-2026 window):

1. zkML EZKL Modulus Labs Orochi 2025-2026 benchmark
2. Apple Private Cloud Compute AI agent TEE 2025
3. Bittensor TAO subnet 2025-2026 dTAO 经济模型
4. Fetch.ai ASI Alliance 2025-2026 agent marketplace
5. Ritual Network Anthropic decentralized AI inference
6. Anthropic Constitutional AI 2025-2026 training-time vs runtime
7. W3C DID AI agent identity portability soulbound NFT
8. EU AI Act 2025 enforcement high-risk audit trail
9. ERC-8004 trustless agent Ethereum January 2026
10. Lagrange DeepProve zkML LLM inference latency
11. AWS Nitro / Intel TDX confidential AI 2025-2026
12. China 生成式 AI Interim Measures audit / 标识
13. Agent federation cross-platform reputation portability 2026
14. MCP Model Context Protocol agent identity auth 2025-2026

预算: ~25 分钟; 14 queries 命中, 充分覆盖。

---

## 2. 总览 (含外部链接)

**2025-2026 关键转折**: 可验证 AI 从"研究 demo"跨越到"生产 standard / mainnet"阶段。

| 维度 | 2024 状态 | 2025-2026 跃迁 | 关键证据 |
| --- | --- | --- | --- |
| zkML | MLP/小 CNN demo | **GPT-2 完整 LLM 推理证明** (Lagrange DeepProve-1) | [DeepProve-1](https://lagrange.dev/blog/deepprove-1) |
| TEE for AI | SGX/Nitro 通用 | **Apple PCC 上线 + WhatsApp 跟进**;TDX/SEV-SNP 支持完整 VM | [Apple PCC](https://security.apple.com/blog/private-cloud-compute/), [WhatsApp PCC](https://9to5mac.com/2025/04/30/whatsapp-borrowing-apples-private-cloud-compute-approach-to-ai-privacy/) |
| 链上 agent 市场 | 概念期 | **Bittensor dTAO**(2025-02)+128 subnets;Templar 完成 72B 去中心化预训练 (2026-03) | [Bittensor 2026 guide](https://www.tao.media/the-ultimate-guide-to-bittensor-2026/) |
| Agent 标准 | 各自为战 | **MCP→AAIF (2025-12)** (Anthropic+OpenAI+Block 联合捐赠至 Linux Foundation);A2A v1.0 (2026 早期);**ERC-8004 mainnet** (2026-01-29) | [MCP→AAIF](https://workos.com/blog/everything-your-team-needs-to-know-about-mcp-in-2026), [ERC-8004 EIP](https://eips.ethereum.org/EIPS/eip-8004) |
| 监管 | EU AI Act 通过 | **2025-02 prohibited practices 生效**;2025-08 GPAI 规则;**2026-08-02 全面生效** (€35M/7% 罚款) | [EU AI Act 2026](https://www.raconteur.net/global-business/eu-ai-act-compliance-a-technical-audit-guide-for-the-2026-deadline) |
| 训练 vs 运行时 alignment | RLHF 主导 | **Anthropic 新 Constitution (2026-01)**: 4 层优先 (safety>ethics>guidelines>helpfulness);Constitutional Classifiers 运行时分类器 | [Anthropic Constitutional Classifiers](https://www.anthropic.com/research/next-generation-constitutional-classifiers) |

**核心 finding**: TuringOS 现有 ChainTape + CAS Merkle commit 是 **本地可验证** 基底; 2025-2026 行业标准 (zkML / TEE attestation / ERC-8004 / DID) 提供 **对外公证 + 跨实例可携带** 能力。两者正交, 互补。

---

## 3. Q1: zkML 进展 vs TuringOS Merkle

### 3.1 zkML 2025-2026 状态

**主要项目 + 性能跃迁**:

- **EZKL** (Modulus Labs 早期; 现独立): ONNX → ZK-SNARK; benchmark 65.88x faster than RISC Zero, 2.92x faster than Orion ([EZKL benchmarks](https://blog.ezkl.xyz/post/benchmarks/))
- **Lagrange DeepProve-1** (2025-08): **首个完整 LLM 推理 zkML**, GPT-2 端到端 proof;比 EZKL-on-GPU verification 671x faster (MLP) / 521x faster (CNN); 验证降至 0.5s ([DeepProve-1](https://lagrange.dev/blog/deepprove-1))
- **zkPyTorch** (2025-03): VGG-16 inference proof 2.2 秒
- **JOLT Atlas / RISC Zero**: 通用 zkVM 路线
- **预期 2026**: 全面 GPU 优化 → 5-10x 提速; GPT-2 proof 从 10min → 1min ([ICME 2025 guide](https://blog.icme.io/the-definitive-guide-to-zkml-2025/))

**经济现实**: zkML 当前仍是 **CPU/GPU 100x-1000x 开销**; LLM 大规模实时推理 zk 证明在 2026 仍是"分钟级"而非"毫秒级"; 适合 **低频高价值 attestation** (合规审计 / 跨链事件) 而非实时 agent loop。

### 3.2 vs TuringOS Merkle commit

| 维度 | TuringOS ChainTape Merkle | zkML SNARK |
| --- | --- | --- |
| 证明对象 | "transaction 序列哈希一致" | "模型在输入 X 上输出 Y" |
| 信任假设 | 已发布 Merkle root + 可验证 sequencer 签名 | 仅信任 SNARK 数学 + setup |
| 开销 | O(N log N) hash, 毫秒级 | O(模型参数)<sup>2-3</sup>, 秒-分钟 |
| 公开公证能力 | **本地可重放, 外部需信任 Merkle root 来源** | **本地不需要, 外部 0.5-1s 验证即可** |
| 当前 TuringOS 落地度 | ✅ LANDED (ChainTape + CAS + replay verifier) | ❌ NOT-LANDED |

**Phase 4-5 建议**: TuringOS Merkle root 本身没有 zk 性质; 若要对外公证某个 agent 的 attempt 输出, 可叠加 **可选的 zkML proof 层** 作为 EvidenceCapsule 子项 (Class-3 extension), 但 **不应替代 Merkle commit**。
两者关系: Merkle = 内部 chronicle; zkML = 外部 oracle。

### 3.3 关键限制

- zkML 当前主要对**模型 inference**, 不对 **LLM agent 多轮 reasoning** 做 end-to-end proof (那需要 zkVM-级整链证明)
- Setup phase (KZG / FRI trusted setup) 仍是显式攻击面
- 经济上 50-1000 USD per LLM proof at 2026 prices, 不可用于每次 step

---

## 4. Q2: TEE for agents

### 4.1 2025-2026 状态

**Apple Private Cloud Compute (PCC)**:

- 2024 年 WWDC 发布; 2025 年商用
- 自研芯片 + Secure Enclave; 内存加密 + ephemeral processing
- **核心创新**: **public verifiability** — Apple 公开发布 PCC binary artifacts 供独立验证 ([Apple PCC blog](https://security.apple.com/blog/private-cloud-compute/))
- 与传统 confidential computing 区别: PCC 强调 **软件透明度 + 公开可验证**, 不只是硬件隔离 ([Edera 对比](https://edera.dev/stories/apples-private-cloud-compute-vs-confidential-computing))
- 2025-04 WhatsApp 宣布跟进 PCC 模式

**AWS Nitro Enclaves / Intel TDX / AMD SEV-SNP**:

- 第二代 TEE 支持完整 VM (而非仅小 enclave), 突破 ML 内存限制 ([Trezalabs TEE guide](https://www.trezalabs.com/blog/what-is-a-trusted-execution-environment-tee-complete-guide))
- **Remote Attestation (RA)**: 安全芯片签名 PCR (Platform Configuration Registers) 链, 客户端可验证服务运行在 TEE 内
- 2025-2026 研究: **Attestable Audits** — auditor + provider 共同 load 模型+审计代码至 TEE, 跑 benchmark, 加密签名结果发布至公开 registry ([arXiv 2506.23706](https://arxiv.org/html/2506.23706v1))

### 4.2 Phala / Edgeless 等 TEE-Web3 桥

- **Phala Cloud**: 提供 dstack 等开源 TEE-on-cloud 抽象 ([Phala docs](https://docs.phala.com/dstack-cloud/overview))
- **Edgeless Systems**: 开源版 Apple PCC alternative

### 4.3 vs TuringOS

| 维度 | TuringOS 当前 | TEE 扩展 |
| --- | --- | --- |
| 跨节点信任 | ❌ 仅本地 sequencer 签名 | ✅ Remote Attestation chain |
| 防恶意主机 | ❌ 信任本机 OS | ✅ TEE 隔离 + 内存加密 |
| 加密计算 | ❌ 明文 prompt | ✅ 端到端加密 (ephemeral) |
| 落地复杂度 | — | 🟡 中等 (libcrate + 部署模板) |

**Phase 4-5 建议**: TEE 是 TuringOS **跨节点联邦** 的最低门槛信任基底:

1. **短期 (Phase 4-5 candidate)**: Sequencer + Lean checker 在 TDX/SEV-SNP VM 内运行; 发布 RA quote 作为 genesis_report 的一部分。
2. **中期**: ChainTape 跨节点同步时, 每个 node 用 RA quote 证明自己运行 unmodified TuringOS binary。
3. **长期**: 学习 Apple PCC 公开 binary artifact 模式 — TuringOS 发布 deterministic build + 公开 binary hash + RA-bound execution。

**警告**: TEE 提供 **runtime confidentiality + integrity**, 不提供 **输出正确性证明**。要后者仍需 zkML 或独立 replay。

---

## 5. Q3: Bittensor / Fetch.ai 经济模型 vs TuringOS

### 5.1 Bittensor (TAO)

**架构**:

- 128 个 subnet (2026-04), 计划扩到 256 ([2026 guide](https://www.tao.media/the-ultimate-guide-to-bittensor-2026/))
- 每个 subnet = 一类 AI 任务 (data / 图像 / 代码 / API / 推理) 的 focused market
- 2025-02 启动 **dTAO**: 每个 subnet 发自己的 alpha 代币, 用户 stake TAO 进 liquidity pool 获取 alpha
- TAO 硬顶 21M, 2025-12 首次减半 7200→3600/day, 下次 2026-12 → 1800

**激励机制**:

- Miner 提供 inference;Validator 评分;reward 按贡献分配
- **争议**: 2026 年分析指出 TAO 年发行 $328M 但年收入仅 $15M, "subsidy-driven 而非 demand-driven"([Bittensor 批评分析](https://www.ainvest.com/news/bittensor-tao-faces-centralization-backlash-volatility-ai-network-expansion-2604/))

**亮点事件**:

- 2026-03 Templar subnet 完成 Covenant-72B, **历史首次完整去中心化 72B LLM 预训练**, TAO 价格 +90%

### 5.2 Fetch.ai / ASI Alliance

- ASI Alliance = Fetch.ai + SingularityNET + Ocean Protocol + CUDOS (2024 合并)
- **Agentverse** (live 2025-2026): agent marketplace + dev tooling
- **ASI:One / ASI-1 Mini**: 类似 "Google Search of AI agents"
- **ASI:Chain TestNet 2026**: 基于 blockDAG 的 Layer 1 for AI workloads
- **ASI:Create Open Beta 2026**: 部署/扩展 agent 平台 ([VentureBeat ASI:One](https://venturebeat.com/ai/the-google-search-of-ai-agents-fetch-launches-asi-one-and-business-tier-for))

### 5.3 Ritual Network

- EVM-compatible L1 + Infernet (decentralized AI oracle network)
- 与 Allora, Celestia (DA), EigenLayer (AVS) 集成 ([Ritual](https://ritual.net/), [Allora x Ritual](https://www.allora.network/blog/allora-x-ritual-powering-crowdsourced-models-e590b))
- 主张: "AI coprocessor of the Web3 world"

### 5.4 vs TuringOS 内生市场

| 维度 | TuringOS (Stage C 后) | Bittensor/Fetch.ai/Ritual |
| --- | --- | --- |
| 市场对象 | task 内部 YES/NO 信念 (Polymarket-style CPMM) | agent 服务 / 推理算力 / 训练贡献 |
| 经济原则 | "Information Free, Investment Costs" | Subsidy-driven inflation (TAO 案例) |
| 货币层 | 单一 Coin + 完整 set 守恒 | Multi-token (TAO + 每 subnet alpha) |
| 链上身份 | 暂无外部 ID | ERC-721 NFT identity (Bittensor neuron / Fetch.ai DID) |
| 公开公证 | 仅 Merkle root | 链上 settlement |

**核心差异**:

- TuringOS 经济是 **task-internal 信念市场** (谁的 attempt 对); Bittensor 经济是 **service-marketplace** (谁的模型好)
- 两者 **互补不冲突**: TuringOS subnet 可作为 Bittensor "task subnet" 的一种, 而 Bittensor 不提供 TuringOS 的 task-internal CPMM 机制

**Phase 4-5 建议**:

- **不应** 复制 Bittensor subsidy 模式 (违反 TuringOS Coin 守恒原则)
- **可参考** Bittensor dTAO 的 subnet-alpha 分层 (每 task 独立 conditional market) — 但 TuringOS YES/NO + CompleteSet 已是更强约束
- **可学习** Fetch.ai Agentverse 的 agent discovery + capability registry 模式 → 对应 TuringOS 的 `AgentRegistry`
- **跨链外溢**: TuringOS 若需对外卖 attestation, Ritual-style EVM bridge 是合理路径

---

## 6. Q4: Anthropic CAI vs TuringOS runtime constitution

### 6.1 Anthropic Constitutional AI (CAI)

**历史**:

- 2022 提出 RLAIF: 模型按 constitution 给自己回答打分
- 2024 Constitutional Classifiers: 训 input/output 分类器, 数据由 constitution 合成 ([Anthropic CC](https://www.anthropic.com/research/constitutional-classifiers))
- **2026-01 新 Claude Constitution**: 4 层优先级 (safety > ethics > guidelines > helpfulness) ([TIME 报道](https://time.com/7354738/claude-constitution-ai-alignment/))
- **dynamic constitutions**: 模型识别 ambiguity 并提议修正; alignment failure 比 static 降低 40% ([Claude5 2026 hub](https://claude5.com/news/ai-safety-2026-alignment-progress-and-open-challenges))

**特征**: training-time + 部署期 classifier 双层; constitution **嵌入模型参数 + 推理输入/输出过滤**, 不进入 transaction tape。

### 6.2 TuringOS Constitutional Harness

**特征**:

- Constitution 是 **executable gate** (CI tests in `tests/constitution_*.rs`), 不是 prompt
- 通过 **predicate pass / fail → L4 / L4.E** 路由实现 hard boundary
- 模型 (黑盒 LLM) 提交 WorkTx → predicate 拒绝 → 不进入 canonical state
- Constitution Landing Gate (Stage A2/A3 + FC3 evidence binding) 是 mechanical CI 强制

### 6.3 互补 vs 替代

| 维度 | Anthropic CAI | TuringOS Harness |
| --- | --- | --- |
| 边界位置 | 模型 weights + classifier | sequencer admission + predicate |
| 失败模式 | jailbreak / prompt injection | predicate reject + L4.E evidence |
| 可验证性 | 模型行为统计 | tape replay 完全确定性 |
| 跨实例可移植 | classifier weights | constitution.md + executable gates |
| 训练依赖 | ✅ 必须重训 | ❌ 完全 model-agnostic |

**关键 finding**: 两者 **完全正交、强互补**:

- Anthropic CAI 保护 **模型 vs 用户** (有害指令 / jailbreak)
- TuringOS 保护 **系统 vs 模型黑盒** (经济守恒 / 证明可验证 / tape 完整性)
- 一个 Claude 3.7/4/5 + Constitutional Classifiers 可以无缝作为 TuringOS Agent — 双层保护叠加

**警告**: TuringOS **绝不应** 走 Anthropic CAI 路径替换 predicate (那会失去 mechanical gate); 反之, TuringOS 也不应替代 model-level alignment。两者各管一头。

### 6.4 Anthropic 的 runtime 转向

值得注意: Anthropic 2025-2026 的 Constitutional Classifiers 实际上 **承认了 training-only 不够**, 需要 **runtime 分类器**。这与 TuringOS "runtime constitutional harness" 哲学一致。可能在 2026-2027 走向收敛。

---

## 7. Q5: Agent 身份可携带性 + 跨实例联邦

### 7.1 标准格局 (2025-2026)

**最关键单一事件**: **ERC-8004 mainnet** 2026-01-29 上线, 3 天内 22,900+ agent 注册 ([ERC-8004 EIP](https://eips.ethereum.org/EIPS/eip-8004))

**架构**:

- **Identity Registry** (ERC-721 base): agent → registration file 解析
- **Reputation Registry**: 类似 credit score; 链上可查 transaction history
- **Validation Registry**: **可插拔信任模型** — reputation / stake-secured re-execution / **zkML proof** / **TEE oracle**

**关键属性**: ERC-8004 把 **reputation 与 wallet/agent 绑定**, 让 "声誉跨平台 portable"。

**配套**:

- **Soulbound NFT (RNWY)**: 数学上不可转让, 防 reputation laundering
- **W3C DID v1.1** (2026 spec update): DID portable + verifiable credentials
- **arXiv 2511.02841**: AI Agents + DID + VC 学术框架

### 7.2 协议层联邦

**MCP → AAIF** (Linux Foundation, 2025-12 成立):

- Anthropic + OpenAI + Block 联合捐赠 MCP
- 4 个月增至 170+ 成员
- MCP authentication 演进: OAuth 2.1 + Resource Indicators (RFC 8707, 2025-06 强制) + CIMD (Client ID Metadata Documents, 2025-11) ([WorkOS MCP 2026](https://workos.com/blog/everything-your-team-needs-to-know-about-mcp-in-2026))

**A2A (Agent-to-Agent)**:

- v1.0 2026 早期; 跨平台 agent 间通信
- Google Universal Commerce Protocol v1.0: agent 经济交易层

### 7.3 vs TuringOS

| 维度 | TuringOS 当前 | 行业标准 (2026) |
| --- | --- | --- |
| Agent ID | local AgentRegistry pubkey | ERC-8004 NFT + DID |
| 跨实例可携带 | ❌ | ✅ Soulbound NFT |
| 声誉跨实例 | ❌ (仅本 ChainTape) | ✅ Reputation Registry |
| 跨实例通信 | ❌ | MCP + A2A |
| 公开公证 attestation | Merkle root | Validation Registry (插入 zkML/TEE) |

**Phase 4-5 建议** (forward-bound):

1. **TuringOS Agent ID → DID 桥**: 每个 AgentRegistry entry 可选地携带 `did:turingos:<agent_pubkey>` URI, 兼容 W3C DID v1.1
2. **跨实例 reputation export**: H-VPPUT / capability-compilation metrics 可作为 Verifiable Credential 签发到 ERC-8004 Reputation Registry
3. **TuringOS as Validation Registry option**: 反向 — TuringOS Merkle commit + replay 本身可作为 ERC-8004 "validation 后端"之一 (类似 zkML/TEE 选项)
4. **MCP server 接入**: TuringOS 可暴露 MCP server, 让外部 client (Claude/GPT) 通过标准协议读写 tape

**风险**: ERC-8004 在 Ethereum 主网, 引入需面对 gas 成本 + 链状态 confirmation; TuringOS Coin 守恒规则与 ETH 经济不兼容 — 必须保持 **桥接层而非融合层**。

---

## 8. TuringOS 对外公证扩展面 (Top 5)

按"门槛低 + 价值高 + 与 constitution 不冲突"排序:

| # | 扩展面 | 短描述 | 风险类 | 时间线建议 |
| --- | --- | --- | --- | --- |
| 1 | **DID 桥 / `did:turingos:` URI scheme** | 每个 AgentRegistry entry 可解析为 W3C DID; 不修改 sequencer | Class-1 (additive parser) | Phase 4 短期 |
| 2 | **TEE remote attestation as genesis_report** | Sequencer 进 TDX/SEV-SNP, 启动时发布 RA quote, 作为 `genesis_report` 子项 | Class-3 (production wire-up + crypto) | Phase 5 中期 |
| 3 | **ChainTape Merkle root 周期性外锚** | 把 `head_t.state_root` 周期性 commit 到外部公链 (Ethereum / Bitcoin) 作为时间戳锚 | Class-3 (外部接口); 不影响 internal flow | Phase 5 中期 |
| 4 | **MCP server 暴露 read-only tape view** | 让外部 client 通过 MCP 协议读 tape (写仍需 sequencer 内部 admission) | Class-2 (read-only adapter) | Phase 4 短期 |
| 5 | **可选 zkML proof in EvidenceCapsule** | 高价值 attempt 可附加 zkML proof; 不强制 (经济成本高) | Class-3 (CAS schema 扩展) | Phase 6+ 长期 |

**故意不进入 Top 5 的**:

- Bittensor / Fetch.ai 链桥 (经济模型冲突, 远期再议)
- ERC-8004 上链 (gas 成本 + 链状态依赖, 用 DID 桥替代更轻)
- 全量 zkML replace Merkle (开销不可接受)

---

## 9. 给 Phase 4-5 建议

### 9.1 设计原则

1. **Merkle 是核心, zkML/TEE 是外延**: TuringOS 内部仍以 ChainTape Merkle commit 为唯一 canonical evidence; 对外公证才走 zkML/TEE 层。
2. **协议兼容优先于实现融合**: 走 MCP / DID / ERC-8004 标准接口, 不绑死特定链。
3. **Constitution 边界严格**: 任何外部 attestation 进入前必须经 FC3 (Architect/Veto-AI), 不能绕过 Constitution Landing Gate。

### 9.2 优先级建议

- **必做 (Phase 4)**: DID 桥 + MCP read-only server (低风险高互操作性)
- **应做 (Phase 5)**: TEE attestation + Merkle 外锚 (跨实例信任)
- **可选 (Phase 6+)**: zkML 选择性 proof (高价值 attempt only)
- **不做**: 复制 Bittensor 经济; 链上身份直接上 ETH 主网

### 9.3 监管对齐 (前瞻)

- **EU AI Act** (2026-08 全面生效): "每个决策可重构 from audit log" — TuringOS ChainTape **天然满足**, 但需文档化对接 EU 高风险 AI 系统记录要求 ([EU AI Act compliance guide](https://www.surecloud.com/resource-hub/eu-ai-act-complete-compliance-guide))
- **中国《生成式 AI 服务管理暂行办法》+ 标识办法** (2025-09-01 强制): 双重标注 (visible + metadata) — TuringOS CAS 可作为 metadata 容器
- **US AI Bill of Rights**: 原则性, 未强制 audit trail

**核心论点**: TuringOS 的 **tape-first** 设计在 2025-2026 监管对齐窗口 **意外踩在风口上** — EU AI Act 第 12 条 (record-keeping) 几乎在 spec 上要求 ChainTape 等价物。

### 9.4 风险与限制

- zkML 经济不可行用于实时 step; **强行集成 = bloat**
- TEE 依赖硬件供应商 (Apple/Intel/AMD), 单点信任风险
- ERC-8004 上链 = gas + ETH 经济耦合 + 不可逆;DID 桥更轻
- Anthropic CAI 与 TuringOS 关系若被外部误读为"替代关系"会损害定位 — 需文档明确"正交互补"

---

## 10. References (markdown 链接)

### zkML

- [Lagrange: DeepProve-1 — First zkML for Full LLM](https://lagrange.dev/blog/deepprove-1)
- [EZKL Blog: Benchmarking ZKML Frameworks](https://blog.ezkl.xyz/post/benchmarks/)
- [ICME: The Definitive Guide to ZKML (2025)](https://blog.icme.io/the-definitive-guide-to-zkml-2025/)
- [State of EZKL: 2025](https://blog.ezkl.xyz/post/state_of_ezkl/)
- [Worldcoin: Intro to zkML](https://world.org/blog/engineering/intro-to-zkml)
- [Spectral: State of zkML](https://blog.spectral.finance/the-state-of-zero-knowledge-machine-learning-zkml/)

### TEE / Confidential Computing

- [Apple Security Research: Private Cloud Compute](https://security.apple.com/blog/private-cloud-compute/)
- [Edera: Apple PCC vs Confidential Computing](https://edera.dev/stories/apples-private-cloud-compute-vs-confidential-computing)
- [WhatsApp Borrowing Apple's PCC Approach (2025-04)](https://9to5mac.com/2025/04/30/whatsapp-borrowing-apples-private-cloud-compute-approach-to-ai-privacy/)
- [AWS Nitro Enclaves](https://aws.amazon.com/ec2/nitro/nitro-enclaves/)
- [Cryptographic Attestation - AWS Nitro](https://docs.aws.amazon.com/enclaves/latest/user/set-up-attestation.html)
- [Trezalabs: What Is a TEE (2026)](https://www.trezalabs.com/blog/what-is-a-trusted-execution-environment-tee-complete-guide)
- [arXiv 2506.23706: Attestable Audits](https://arxiv.org/html/2506.23706v1)
- [Phala Cloud Documentation](https://docs.phala.com/dstack-cloud/overview)
- [Edgeless Systems: PCC Core Concepts and Open Alternative](https://www.edgeless.systems/blog/apple-private-cloud-compute-core-concepts-and-an-open-alternative)

### 链上 Agent 市场 / 经济

- [The Ultimate Guide to Bittensor 2026](https://www.tao.media/the-ultimate-guide-to-bittensor-2026/)
- [Bittensor's DeepSeek Moment (BlockEden 2026-04)](https://blockeden.xyz/blog/2026/04/11/bittensor-decentralized-ai-deepseek-moment-tao/)
- [Bittensor Centralization Backlash (AInvest)](https://www.ainvest.com/news/bittensor-tao-faces-centralization-backlash-volatility-ai-network-expansion-2604/)
- [Top Bittensor Subnets in 2026](https://ourcryptotalk.com/blog/top-bittensor-subnets-2026)
- [Fetch.ai & ASI Alliance (Greythorn)](https://greythorn.com/fetch-ai-the-asi-alliance-decentralized-ai-powerhouse/)
- [VentureBeat: Fetch launches ASI:One](https://venturebeat.com/ai/the-google-search-of-ai-agents-fetch-launches-asi-one-and-business-tier-for)
- [Ritual Network](https://ritual.net/)
- [Allora x Ritual](https://www.allora.network/blog/allora-x-ritual-powering-crowdsourced-models-e590b)
- [Inside Ritual Network (Mitosis)](https://university.mitosis.org/inside-ritual-network-the-architecture-use-cases-and-community-powering-ritualnet/)

### Anthropic Constitutional AI

- [Anthropic: Next-gen Constitutional Classifiers](https://www.anthropic.com/research/next-generation-constitutional-classifiers)
- [Anthropic: Constitutional Classifiers (Original)](https://www.anthropic.com/research/constitutional-classifiers)
- [Anthropic: Collective Constitutional AI](https://www.anthropic.com/research/collective-constitutional-ai-aligning-a-language-model-with-public-input)
- [Anthropic CAI v2 paper PDF](https://www-cdn.anthropic.com/7512771452629584566b6303311496c262da1006/Anthropic_ConstitutionalAI_v2.pdf)
- [TIME: Anthropic Publishes Claude's New Constitution (2026)](https://time.com/7354738/claude-constitution-ai-alignment/)
- [AI Safety 2026 (Claude5 Hub)](https://claude5.com/news/ai-safety-2026-alignment-progress-and-open-challenges)

### Agent 身份 / 联邦标准

- [ERC-8004 EIP](https://eips.ethereum.org/EIPS/eip-8004)
- [What is ERC-8004 (eco.com)](https://eco.com/support/en/articles/13221214-what-is-erc-8004-the-ethereum-standard-enabling-trustless-ai-agents)
- [ERC-8004 Developer's Guide (QuickNode)](https://blog.quicknode.com/erc-8004-a-developers-guide-to-trustless-ai-agent-identity/)
- [ERC-8004 Launches on Ethereum (Gate Learn)](https://www.gate.com/learn/articles/erc-8004-launches-on-ethereum-to-power-identity-and-trust-for-autonomous-ai-agents)
- [arXiv 2511.02841: AI Agents with DIDs and VCs](https://arxiv.org/abs/2511.02841)
- [RNWY: AI Agent Passport Soulbound Identity](https://rnwy.com/learn/ai-agent-passport)
- [W3C DID v1.1 (Biometric Update)](https://www.biometricupdate.com/202603/w3c-releases-updated-decentralized-identifiers-spec-for-comment)
- [Stealthcloud: Soulbound Tokens Privacy](https://stealthcloud.ai/web3-identity/soulbound-tokens/)
- [Vouched: Decentralized Identity & MCP-I](https://www.vouched.id/learn/blog/decentralized-identity-did-and-blockchain-a-future-vision-for-user-controlled-identity)
- [AWS: AI Agents, Web3, Post-Quantum Trust](https://aws.amazon.com/blogs/industries/securing-the-future-how-ai-agents-web3-and-post-quantum-cryptography-are-helping-redefine-digital-trust/)
- [INATBA: Building Trust — AI, Blockchain, Digital Identity (Nov 2025 PDF)](https://inatba.org/wp-content/uploads/2025/11/Building-Trust_-Integrating-AI-Blockchain-and-Digital-Identity_NOVEMBER-2025.docx.pdf)

### Agent 协议 / 联邦

- [WorkOS: Everything Your Team Needs to Know About MCP in 2026](https://workos.com/blog/everything-your-team-needs-to-know-about-mcp-in-2026)
- [Model Context Protocol Specification 2025-11-25](https://modelcontextprotocol.io/specification/2025-11-25)
- [Anthropic: Introducing MCP](https://www.anthropic.com/news/model-context-protocol)
- [Agentic AI Foundation (Linux Foundation)](https://intuitionlabs.ai/articles/agentic-ai-foundation-open-standards)
- [AI Agent Protocols 2026 Complete Guide](https://www.ruh.ai/blogs/ai-agent-protocols-2026-complete-guide)

### 监管 / Compliance

- [EU AI Act Official (EC Digital Strategy)](https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai)
- [EU AI Act Compliance Checker](https://artificialintelligenceact.eu/assessment/eu-ai-act-compliance-checker/)
- [Raconteur: EU AI Act 2026 Technical Audit Guide](https://www.raconteur.net/global-business/eu-ai-act-compliance-a-technical-audit-guide-for-the-2026-deadline)
- [Trilateral Research: EU AI Act Timeline 2025-2027](https://trilateralresearch.com/responsible-ai/eu-ai-act-implementation-timeline-mapping-your-models-to-the-new-risk-tiers)
- [SureCloud: EU AI Act 2025-26 Compliance Guide](https://www.surecloud.com/resource-hub/eu-ai-act-complete-compliance-guide)
- [Goteleport: EU AI Act Requirements & What to Document](https://goteleport.com/blog/eu-ai-act-requirements/)
- [DEV: EU AI Act Compliance Checklist for AI Agents](https://dev.to/verisigilai/eu-ai-act-compliance-checklist-for-ai-agents-87-days-until-enforcement-3m1a)
- [China Briefing: Interim Measures for Generative AI](https://www.china-briefing.com/news/how-to-interpret-chinas-first-effort-to-regulate-generative-ai-measures/)
- [Securiti: China's AI Regulatory Landscape 2025](https://securiti.ai/china-ai-regulatory-landscape/)
- [White & Case: AI Watch China](https://www.whitecase.com/insight-our-thinking/ai-watch-global-regulatory-tracker-china)
- [Holistic AI: Making Sense of China's AI Regulations](https://www.holisticai.com/blog/china-ai-regulation)

---

*End of Track I frontier research. ~3,400 words. All claims forward-bound. No code, sequencer, or constitution modified.*
