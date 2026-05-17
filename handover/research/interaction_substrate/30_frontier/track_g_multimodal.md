# Phase 3 / Track G — 多模态生成式交互

> **Status**: forward-bound 调研笔记
> **Date**: 2026-05-17
> **Author**: TISR Phase 3 Track G agent
> **Scope**: 调研 2025-2026 年多模态 agent / generative UI 前沿, 评估 TuringOS 当前 0 多模态基底应如何演化

---

## 0. forward-bound 警告

本文档是 **前瞻性外部调研笔记**, 不是 TuringOS 的 spec, 不是 charter, 也不是 commitment。

- TuringOS 当前 (2026-05-17) 100% 文本输入输出, 没有任何多模态代码。
- 所有"建议"是 Phase 4-5 阶段的候选输入, 需要架构师 §8 ratification 才可以进入实施轨道。
- 引用的产品 (GPT-4o Realtime / Gemini Live / Vision Pro / OpenVLA / Genie / Marble / ElevenLabs) 都在快速迭代; 数据点截至 2026-05-17 调研时刻。
- "TuringOS 缺什么" 清单是 **gap analysis**, 不是 TODO list。落地任何一项都至少 Class-3, 部分 (CAS schema 扩展、sequencer 多模态 tx admission) 是 Class-4。

不要把本文当 source of truth; ChainTape/CAS/constitution 仍然是项目唯一权威。

---

## 1. 任务 scope

### 1.1 调研方法

- WebSearch (~12 次, 见 §10) 覆盖 5 个 Q 方向
- 重点关注 2024 H2 至 2026 Q2 的产品/论文/SDK 公告
- 优先官方文档 + 顶会 (CVPR / NeurIPS) + 主流厂商 blog
- 不深度阅读 paper, 只取架构要点 + 性能/成本数据点 + SDK 抽象层

### 1.2 调研对象清单 (实际覆盖)

| 方向 | 代表产品 / 论文 |
|---|---|
| Realtime API | OpenAI gpt-realtime, Gemini 2.0 Live API |
| 空间计算 | Apple visionOS 26, Meta Quest XR Interaction SDK |
| 具身智能 | OpenVLA, RT-2, RT-X, PaLM-E, SayCan, Open-EQA |
| 生成式 simulation | Genie 2/3, SIMA 2, World Labs Marble |
| 语音优先 agent | ElevenLabs Agents (Conversational AI 2.0), Deepgram State of Voice AI 2025 |
| 多模态 CAS / diff | BLAKE3, pHash/dhash, videohash, MIRIX 多模态 memory, glTF Git LFS |
| Generative UI | Vercel AI SDK 3.0+, json-render |

### 1.3 不调研

- 数字孪生、CAD 工具链 (与 TuringOS scope 距离过远)
- 中文厂商 (字节/讯飞/科大): 资料公开度低且与英语主流接口不对称
- 通用 ASR/TTS 模型选型 (产品层细节, 不影响架构决策)

---

## 2. 总览

### 2.1 行业格局快照 (2025-2026)

**两个独立趋势线收敛**:

1. **Realtime 多模态 API 商品化**: GPT-4o Realtime ([gpt-realtime](https://openai.com/index/introducing-gpt-realtime/), 2024-10) 和 Gemini 2.0 Live API ([ai.google.dev](https://ai.google.dev/gemini-api/docs/live-api), 2024-12) 把 voice-to-voice + vision 单次 forward pass 做到 <300ms 延迟, 价格降到 $32/1M audio in / $64/1M audio out。本地 LLM 套 STT→LLM→TTS 流水线已经被双方在端到端延迟和自然度上甩开。
2. **3D / 空间 / 具身 三线分叉**:
   - **3D 生成**: Genie 3 (2025-08) 实时 24 fps 720p, World Labs Marble (2025-11) 商业化 text→3D
   - **空间 UI**: visionOS 26 (2025-06) + Quest Spatial SDK 2.0 = 头显成熟期但用户量 << 屏幕
   - **具身 VLA**: OpenVLA (7B, 2024-06 开源) 在 Open-X-Embodiment 970K episodes 上击败 RT-2-X (55B, 闭源) 16.5%; SIMA 2 (2025-11) 在 Genie 3 生成的世界里自我改进
3. **语音优先 agent 商业化**: ElevenLabs Agents (CAI 2.0, 2025) 提供 MCP 工具调用 + Scribe v2 Realtime (<150ms STT), Deepgram 报告 50% customer-service 交互 2025 年由 voice AI 处理 ([State of Voice AI 2025](https://deepgram.com/learn/state-of-voice-ai-2025))。
4. **Generative UI**: Vercel AI SDK 3.0+ (React Server Components 流式渲染) 把 LLM tool-call 直连组件; v0/json-render 走 JSON-schema-driven 路径。

### 2.2 与 TuringOS 的距离

| 维度 | 主流前沿 | TuringOS 现状 | 距离 |
|---|---|---|---|
| 输入模态 | text + voice + vision (实时) | text only | 大 |
| 输出模态 | text + speech + UI components | text + tape 写入 | 中-大 |
| Artifact 存储 | 多模态 + tiered (热/冷) | CAS (text/JSON only) | 中 |
| Agent 通信介质 | text + voice + tool-call schema | text + WorkTx wire | 中 |
| 空间 UI | visionOS / Quest 原生 | 无 | 极大 (战略选择) |
| 具身 action | VLA model + 机器人 (Open-X) | 无物理执行器 | 极大 (scope 外) |

TuringOS 的 **核心 invariant — tape-first / FC1 hard equality / no-fake-accepted** 与多模态扩展不冲突, 但需要 schema 层的非平凡演化。

---

## 3. Q1: Realtime API 架构

### 3.1 核心架构 (双家共同模式)

两家都采用 **WebSocket 双向流 + 单一模型 voice-to-voice forward pass**:

- **OpenAI gpt-realtime**: 端到端 voice-to-voice 单 forward pass, 不再 STT→LLM→TTS 三段。2024-12-17 版本延迟 <200ms (典型 300ms)。会话期间可中途插入 tool-call (函数调用); session config 里 tools array 等同 Chat Completions 工具调用。([openai.com/index/introducing-gpt-realtime](https://openai.com/index/introducing-gpt-realtime/))
- **Gemini 2.0 Live API**: 同样 WebSocket 状态化, 同时收发 text+audio+video; 内建 VAD (Voice Activity Detection) + function calling + code execution + search grounding + ephemeral tokens (客户端直连安全)。Pipecat / LiveKit Agents 是主流框架。([ai.google.dev/gemini-api/docs/live-api](https://ai.google.dev/gemini-api/docs/live-api))

### 3.2 延迟和成本数据点

| 项目 | OpenAI gpt-realtime | Gemini 2.0 Live |
|---|---|---|
| 端到端延迟 | <200ms (典型 300ms) | <500ms (类似量级) |
| Audio input | $32 / 1M tokens (\~$0.06/min) | 类似量级 |
| Audio output | $64 / 1M tokens (\~$0.24/min) | 类似量级 |
| Cached input | $0.40 / 1M tokens | -- |
| mini 变体 | $10/$20 (gpt-4o-mini-realtime) | -- |

**成本启示**: 一个 5 分钟的 voice agent 对话 (典型客服场景) 约 $0.30-1.50。这比 chat-only 贵 10-30 倍, 但 UX 提升远超线性。

### 3.3 对 TuringOS 的含义

- **不冲突**: Realtime API 完全可以作为 **PromptCapsule.visible_context** 上游 (audio 转录 + 视觉理解结果) 进入 FC1 Q_t -> rtool 链路。LLM-Lean 主循环可继续 text-only, voice 只是输入层。
- **新风险**: 实时 voice 引入 **延迟敏感** 路径; 当前 TuringOS 不强制 latency budget, 但加 voice 后 FC1 的端到端 latency 会成为 first-class concern。
- **PromptCapsule 扩展**: 当前 PromptCapsule 是 text-only 哈希; 多模态需要扩展为
  ```
  PromptCapsule {
    text_prompt_hash,
    audio_input_cid: Option<Cid>,   // 转录前原始音频
    image_input_cids: Vec<Cid>,     // 视觉输入 frame
    transcription_cid: Option<Cid>, // 转录结果, 与 audio_input_cid 哈希链
    ...
  }
  ```
  这是 Class-3 CAS schema 扩展候选, 需要 §10 重新分类是否触及 Class-4 canonical signing payload。

---

## 4. Q2: VR / 空间 UI

### 4.1 平台格局

**Apple visionOS 26 (2025-06 WWDC)**:
- Xcode + SwiftUI + RealityKit + ARKit + Reality Composer Pro 工具链。([developer.apple.com/visionos](https://developer.apple.com/visionos/))
- 2025 更新: hand tracking 速度 ×3, Sony PSVR2 控制器 + Logitech Muse 笔输入支持, 内容可"locked"到物理表面持久化跨重启。
- 没有专门"spatial agent UI" 框架——开发者用 SwiftUI 容器 (Window/Volume/ImmersiveSpace) 自建。

**Meta Quest Spatial SDK 2.0 (2025)**:
- XR Interaction SDK (Unity + Unreal); Spatial Editor 2.0 引入 XML 组件系统; Quest Link 加速热重载。([developers.meta.com/horizon](https://developers.meta.com/horizon/documentation/unity/unity-isdk-interaction-sdk-overview/))
- "**Agentic tools for Meta Quest**" — Meta 已经把 AI coding assistant + headset 文档 + 性能工具串联, 暗示开发者-agent 协作场景。

### 4.2 共同抽象

不论 Apple 还是 Meta, **空间 UI 都还是面向人类用户的渲染层**, 不是面向 agent 的接口层。Agent 在空间 UI 中的角色目前是:

- 内容生成 (3D 模型 / 场景描述)
- 语音交互 (Siri / Meta AI on Quest)
- 工具调用 (在沉浸式应用内调度后端服务)

**Agent-to-Agent 走空间通道** 没有任何主流产品支持; 这仍是文本/JSON over WebSocket。

### 4.3 TISR 是否应该考虑空间 UI

**回答**: Phase 4-7 范围内 **不建议** 直接做空间 UI 适配。理由:

1. **用户规模不匹配**: Vision Pro 全球累计销量百万级, Quest 千万级, 屏幕端百亿级。TuringOS 作为研究 OS 锁定到 <1% 的潜在用户硬件是策略失误。
2. **抽象层错位**: visionOS/Quest SDK 是 UI 渲染框架, 不是 agent 协议; TuringOS 缺的是 **WebSocket 多模态 stream 协议**, 不是 3D 渲染。
3. **正确介入点**: 如果 Phase 8+ 真要做空间, 走 **WebXR + browser-native** 路径 (Marble 已示范 web 端 3D), 而不是绑定 Apple/Meta 私有 SDK。

**保留窗口**: Phase 7 Web UI 设计时, 输出格式 (HTML/CSS/SVG/Markdown) 保留对 WebXR 的前向兼容, 但不实现。

---

## 5. Q3: 具身 AI vs TuringOS tape

### 5.1 VLA 模型现状

**OpenVLA (2024-06)**: 7B 参数, Llama 2 + DINOv2 + SigLIP, 在 Open-X-Embodiment 970K episodes 上训练, 击败 RT-2-X 55B 闭源 16.5%。可在消费级 GPU 上 LoRA 微调, 量化部署。([openvla.github.io](https://openvla.github.io/))

**RT-2 (2023, Google DeepMind)**: PaLI-X / PaLM-E 微调到机器人 demonstration 数据。RT-X 是 Open-X-Embodiment 多 embodiment 协作版本。

**PaLM-E (2023)**: 562B 参数, 把图像作为 language token 输入, 与 SayCan 组合后 84% 正确技能序列、74% 成功执行率。

### 5.2 Open-EQA Benchmark (CVPR 2024)

Meta AI 发布的 embodied QA 数据集: 1600+ 题目 / 180+ 真实环境, 测属性识别 / 空间理解 / 功能推理 / 世界知识。GPT-4V / Claude 3 / Gemini Pro 全部显著低于人类基线。([open-eqa.github.io](https://open-eqa.github.io/))

### 5.3 与 TuringOS tape 的根本张力

**短链路 vs 长链路**:

- 具身 AI 是 **50Hz 闭环**: 摄像头 → VLA → 电机控制 → 物理反馈, 每 20ms 一个 tick。
- TuringOS tape 是 **离散事件链**: WorkTx → predicate → L4/L4.E, 每个 tick 涉及 hash + signature + sequencer admission。

**强行让 VLA 50Hz 写 tape 会爆炸**: 每秒 50 个 L4 transition × 持续运行 = 不可行。

**正确融合路径**:

1. **VLA 在 sub-tape 层运行**: VLA 内部循环私有, 不进 ChainTape。
2. **里程碑事件进 L4**: "拿起咖啡杯" 这种语义级动作完成时, 提交一个 WorkTx 锚定 EvidenceCapsule (含完整轨迹 video + sensor log + VLA 决策)。
3. **失败也是 evidence**: 物理失败 (掉杯子) 走 L4.E, 类型同 LeanFailed。
4. **PCP-style soundness 不适用**: Lean 验证有 ground truth (类型检查), 物理动作没有 — 需要新的 oracle (人类标注 / vision-judge)。

### 5.4 对 TuringOS 的含义

- **Phase 4-7 不做具身**: TuringOS 是 LLM 操作系统, 不是机器人 OS。
- **保留接口**: AgentRegistry 应该支持注册"具身 agent" 类型 (未来), 但当前不实现。
- **可借鉴**: Open-X-Embodiment 的多 embodiment 协作模式 = TuringOS 多 agent 协议设计参考。VLA 把 action 当作 language token 的设计 = TuringOS 把 multimodal artifact 当作 tape token 的设计参考。

---

## 6. Q4: 多模态 artifact diff + 去重

### 6.1 哈希算法选择

**BLAKE3** ([github.com/BLAKE3-team/BLAKE3](https://github.com/BLAKE3-team/BLAKE3)):
- 加密哈希, 主打速度 + 并行 + 增量验证
- 适合 **精确去重**: 同一文件多次写入 = 同一哈希
- 不适合: 视觉相似图像、有损压缩变体识别

**Perceptual hashes (pHash / dHash / aHash)**:
- 64-bit 哈希, 比较 pixel-brightness 邻差; 对压缩、缩放、轻度修改鲁棒
- 适合: 找近似图片、视频去重 (帧采样)
- 不适合: 加密属性 (容易碰撞, 不能作为内容认证)

**videohash** ([github.com/akamhy/videohash](https://github.com/akamhy/videohash)): 64-bit 视频 perceptual 哈希, 比逐帧 imagehash 快。

**DiffVC (2025)**: diffusion-based 视频压缩, Temporal Diffusion Information Reuse 复用前帧——前沿但太重, 不适合 CAS 层。

### 6.2 Track B 提议的 2-tier 策略评估

Track B 已设计 ArtifactStorageManifest (multimodal 2-tier 存储)。结合调研:

**合理性**: ✓
- 工业界共识: AI 数据管道分 3-tier (capacity / performance / production) — 2-tier 是简化但方向对。
- MIRIX 多模态 memory 系统 (ScreenshotVQA 上 +35% acc / -99.9% 存储) 验证 **分类型分层** 比 flat 存储更优。

**建议加强**:

1. **dual-hash 策略**: 每个 artifact 同时存 BLAKE3 (精确去重 / 内容认证) + perceptual hash (相似度查询)。BLAKE3 进 ChainTape 作为 CID, perceptual hash 进 sidecar index (非 canonical, 可重建)。
2. **frame-level video CAS**: 视频不整体哈希, 而是 GOP-level (group of pictures) 分块 BLAKE3 + 整体 videohash。这样小修改只重写改动 GOP。
3. **不要做语义 diff**: 图像/视频/3D 模型的"语义 diff" (例如"删除了背景中的椅子") 是开放问题, 不要在 CAS 层做; 留给 derived view。
4. **冷热分层标准**: 热 = 近 7 天写入 + 当前 run-id 引用; 冷 = 7 天前 + 无活跃引用。引用计数 (refcount) 由 ChainTape 派生, 不存为独立 ledger。

### 6.3 3D 模型版本控制

Git LFS + glTF 是主流, 但 **没有特化 diff 算法** — glTF 是 binary, Git LFS 只是指针存储。

**TuringOS 不需要 3D diff**: Phase 4-7 范围内, 3D artifact (如果有) 当作 opaque CAS object 处理即可。语义级 diff 留给 derived view (如果用户需要可视化)。

---

## 7. Q5: voice-first UX

### 7.1 行业数据

**Deepgram State of Voice AI 2025**:
- 50%+ customer service 2025 由 voice AI 处理
- 85% 企业采用 hybrid (人 + AI), 仅 15% 纯 AI
- Financial Services 91% / Healthcare 87% 采用率最高

**ElevenLabs Agents (CAI 2.0)**:
- Scribe v2 Realtime STT <150ms
- 100ms 音频块处理 (从 250ms 降下来)
- MCP 工具调用 + Slack/Salesforce 集成
- 支持 multimodality (text + voice 共用一个 agent definition)

**Vercel AI SDK 3.0+ generative UI**:
- React Server Components 流式渲染 tool-call 结果
- json-render: AI 输出 JSON, 客户端按 schema 渲染

### 7.2 voice-first 是 Software 3.0 主流吗?

**结论**: 是 **重要扩展面**, 但不是 **主流**。

- voice 在 **高场景特化** (客服 / 车载 / 智能眼镜 / 医疗护理) 是主流。
- voice 在 **通用开发 / 知识工作 / 复杂任务编排** 不是主流——文本仍然是 SOTA。原因:
  - 多任务并行 (语音是串行的, 文本可以多窗口)
  - 引用代码 / 数学符号 / 图表困难
  - 隐私 + 共享办公环境噪音

**TuringOS 当前定位** (formal proof / 自我演化 OS / LLM 协调) **不属于 voice-first 强场景**。

### 7.3 Phase 7+ Web UI 是否应支持 voice

**建议**: **可选层 + 不优先**。

- Phase 7 Web UI 主流仍是文本 + generative UI (Vercel SDK 类型)
- 留接口: WebSocket bidirectional stream 抽象, 允许将来插入 audio codec, 但不实现 voice
- 如果 Phase 8+ 真要做 voice, 直接接 Gemini 2.0 Live API 或 OpenAI Realtime, 而不是自建 voice 栈

### 7.4 Generative UI 更重要

相比 voice, **generative UI** (LLM 直接输出可交互组件) 对 TuringOS Phase 7 更有价值:

- TuringOS 的 evidence (HEAD_t / ChainTape / CAS / EconomicState) 是结构化数据, generative UI 可以让 LLM 根据查询动态生成可视化。
- 例如: 用户问"过去 24 小时所有 BankruptcyTx", LLM 输出 React 组件 (表格 + 时间线 + 链接到 evidence CID), 而不是 Markdown。
- 实现路径: AI SDK 3.0 模式; LLM 在 tool-call 返回时绑定组件。

---

## 8. TuringOS 缺什么清单 (Top 5)

按 **优先级** + **风险类别** 排序:

### Top 1: PromptCapsule 多模态扩展 (Class-3)

**问题**: 当前 PromptCapsule 是 text-only 哈希。如果 Phase 4+ 引入任何多模态输入 (语音转录 / 图像理解), PromptCapsule 不能描述输入的多模态来源, 违反 FC1 hard equality 的可审计性。

**建议**: 加 `audio_input_cid` / `image_input_cids` / `transcription_cid` 字段, 全部走 CAS, ChainTape 只存 hash。

**Class**: 3 (CAS schema 扩展, 不动 canonical signing payload)。

### Top 2: ArtifactStorageManifest dual-hash 加强 (Class-2/3)

**问题**: Track B 设计的 2-tier 存储缺 **perceptual hash sidecar**, 导致近似 artifact 去重不可行。

**建议**: 每个多模态 CAS 对象同时存 BLAKE3 (canonical, 进 ChainTape) + pHash/dHash/videohash (derived sidecar, 可重建)。BLAKE3 作为 CID; perceptual hash 作为辅助 index。

**Class**: 2 (sidecar 是 derived view) 或 3 (如果 sidecar 被 sequencer 引用)。

### Top 3: Realtime stream 协议抽象 (Class-3)

**问题**: 当前 TuringOS 的 agent-tool 接口是请求-响应; 没有 **bidirectional streaming** 抽象。多模态 (audio frame / video frame / VLA 50Hz action) 需要 stream。

**建议**: 在 SDK tool 层引入 `StreamTool` trait, 与现有 `RTool`/`WTool` 平行。Stream 内的微帧不进 ChainTape, 整段流的 manifest (起止时间 + 帧数 + BLAKE3 / pHash 摘要) 进 CAS, L4 锚定。

**Class**: 3 (新 tool abstraction 触及 admission)。

### Top 4: Generative UI 输出协议 (Class-1/2)

**问题**: Phase 7 Web UI 计划缺 **结构化 UI 输出协议**; 如果 LLM 输出 React 组件 JSON, 当前没有 schema 验证。

**建议**: 借鉴 Vercel json-render 模式, 定义 `RenderableArtifact` schema (component / props / data-binding / capability whitelist)。LLM 输出 schema-conforming JSON; 客户端验证 + 渲染。

**Class**: 1-2 (UI 层, 不动 sequencer / 经济)。

### Top 5: Voice-first 接口 deferred (no-op)

**结论**: Phase 4-7 不做 voice。Phase 8+ 接 Gemini Live / OpenAI Realtime, 不自建栈。当前**保留 WebSocket bidirectional stream 抽象** (Top 3 同一通道) 作为前向兼容点即可。

---

## 9. 给 Phase 4-5 建议

### 9.1 Phase 4 (具体实现层) 不做什么

- **不做**: 语音 / 视觉 / 3D / 空间 UI / 具身 — 锁死 Phase 4 范围为 text-only + structured artifact。
- **不做**: 重写 PromptCapsule — 任何 schema 演化在 Phase 5+ 谋划。
- **不做**: 多模态 CAS — 当前 CAS 已能存 opaque binary, 不需要在 Phase 4 做 schema 演化。

### 9.2 Phase 4 (具体实现层) 应做的"保留接口"

1. **CAS schema 留 `mime_type` 字段** (即使始终是 `application/json` / `text/plain`)。这样 Phase 5+ 引入 `image/png` / `audio/wav` 不需要破坏性变更。
2. **PromptCapsule 留 `extension` map** (即使始终空)。同上逻辑。
3. **AgentRegistry 留 `capabilities` 数组** (text-input, text-output, ...)。未来加 `voice-input` / `image-input` / `streaming` 不需要 schema 演化。
4. **WebSocket-ready 通信层**: 即使 Phase 4 全部走 request/response, internal API 抽象成 `Stream<Frame>` (单 frame stream 退化为 request/response)。

### 9.3 Phase 5 (架构演化层) 决策点

Phase 5 需要架构师 §8 ratify 以下二选一战略方向:

**方向 A: 文本基底 + 外挂多模态网关**
- TuringOS 内核保持 text-only
- 多模态走外部 gateway (Gemini Live / OpenAI Realtime) — gateway 输出文本 + 引用 CAS artifact
- 优点: 内核简洁, 经济模型不变, FC1 hard equality 保持
- 缺点: 多模态体验依赖第三方; 失去 vertical integration

**方向 B: 原生多模态 substrate**
- 扩展 PromptCapsule / CAS / AttemptTelemetry 全栈多模态
- 引入 StreamTool 抽象, 自建 audio/video codec 集成
- 优点: 端到端可控, evidence 完整
- 缺点: 内核复杂度爆炸, 需要重做 PCP soundness 体系 (多模态没有 Lean-like ground truth)

**当前 (2026-05-17) 建议**: **方向 A**。理由:
- TuringOS 的 thesis (formal proof + ChainTape 完整 evidence + economic compilation) 在 text + Lean 域已经够难, 加多模态会稀释焦点
- 多模态前沿迭代极快 (GPT-4o → gpt-realtime 半年一次大版本), 外挂可以低成本跟随
- 真要做方向 B, 等到 Phase 8+ 多模态成本降到 chat 同量级再说

### 9.4 不可逆决策警告

以下决策一旦做了就难回退, **必须 §8**:

- **CAS canonical CID 算法**: 当前 BLAKE3; 改换其他哈希 = 历史 evidence 重哈希 = 违反 evidence immutability
- **PromptCapsule wire schema**: 增字段是 forward-compat (binary 解析允许 unknown field); 改字段语义不是
- **Sequencer admission rule**: 任何新 TxKind / 新 admission 路径都是 Class-4
- **EconomicState 多模态扩展**: 如果给 multimodal artifact 加经济成本 (例如视频生成 escrow), 这是 §13 经济宪法层修改

---

## 10. References

### 10.1 Realtime API

- [OpenAI gpt-realtime 公告 (2024-10)](https://openai.com/index/introducing-gpt-realtime/)
- [Gemini Live API 文档](https://ai.google.dev/gemini-api/docs/live-api)
- [Google Developers Blog: Gemini 2.0 multimodal interactions](https://developers.googleblog.com/en/gemini-2-0-level-up-your-apps-with-real-time-multimodal-interactions/)
- [OpenAI API Pricing](https://openai.com/api/pricing/)
- [Gemini Live API examples (GitHub)](https://github.com/google-gemini/gemini-live-api-examples)

### 10.2 空间 UI

- [Apple visionOS Developer](https://developer.apple.com/visionos/)
- [Apple visionOS 26 (newsroom)](https://www.apple.com/newsroom/2025/06/visionos-26-introduces-powerful-new-spatial-experiences-for-apple-vision-pro/)
- [WWDC25: What's new in visionOS 26](https://developer.apple.com/videos/play/wwdc2025/317/)
- [Meta XR Interaction SDK overview](https://developers.meta.com/horizon/documentation/unity/unity-isdk-interaction-sdk-overview/)
- [Meta Spatial SDK mid-2025 upgrades](https://www.uploadvr.com/meta-spatial-sdk-mid-2025-upgrades/)

### 10.3 具身 AI

- [OpenVLA paper (arXiv)](https://arxiv.org/abs/2406.09246)
- [OpenVLA project page](https://openvla.github.io/)
- [OpenVLA on GitHub](https://github.com/openvla/openvla)
- [RT-2 project page](https://robotics-transformer2.github.io/)
- [SayCan project page](https://say-can.github.io/)
- [SayCan PaLM grounding paper](https://say-can.github.io/assets/palm_saycan.pdf)
- [OpenEQA CVPR 2024 paper](https://openaccess.thecvf.com/content/CVPR2024/papers/Majumdar_OpenEQA_Embodied_Question_Answering_in_the_Era_of_Foundation_Models_CVPR_2024_paper.pdf)
- [OpenEQA project page](https://open-eqa.github.io/)
- [OpenEQA on GitHub (facebookresearch)](https://github.com/facebookresearch/open-eqa)
- [VLA Survey for Embodied AI (arXiv)](https://www.arxiv.org/pdf/2405.14093v4)

### 10.4 生成式 simulation / 世界模型

- [Genie 2 (DeepMind blog)](https://deepmind.google/blog/genie-2-a-large-scale-foundation-world-model/)
- [Genie 3 (DeepMind blog)](https://deepmind.google/blog/genie-3-a-new-frontier-for-world-models/)
- [SIMA 2 (DeepMind blog)](https://deepmind.google/blog/sima-2-an-agent-that-plays-reasons-and-learns-with-you-in-virtual-3d-worlds/)
- [SIMA 2 paper (arXiv)](https://arxiv.org/html/2512.04797v1)
- [World Labs research](https://www.worldlabs.ai/blog)
- [World Labs Marble (TechCrunch)](https://techcrunch.com/2025/11/12/fei-fei-lis-world-labs-speeds-up-the-world-model-race-with-marble-its-first-commercial-product/)
- [World Labs TIME profile](https://time.com/7339513/ai-fei-fei-li-virtual-worlds/)

### 10.5 语音优先 agent

- [ElevenLabs Conversational AI 2.0](https://elevenlabs.io/blog/conversational-ai-2-0)
- [ElevenLabs Agents (product)](https://elevenlabs.io/agents)
- [Deepgram State of Voice AI 2025](https://deepgram.com/learn/state-of-voice-ai-2025)
- [ElevenLabs 2026 developer trends](https://elevenlabs.io/blog/voice-agents-and-conversational-ai-new-developer-trends-2025)
- [a16z AI Voice Agents 2025](https://a16z.com/ai-voice-agents-2025-update/)

### 10.6 Generative UI

- [Vercel AI SDK 3.0 generative UI](https://vercel.com/blog/ai-sdk-3-generative-ui)
- [AI SDK UI: Generative User Interfaces](https://ai-sdk.dev/docs/ai-sdk-ui/generative-user-interfaces)
- [Vercel json-render (TheNewStack)](https://thenewstack.io/vercels-json-render-a-step-toward-generative-ui/)
- [What is Generative UI?](https://memo.d.foundation/llm/generative-ui)

### 10.7 多模态 artifact diff + 存储

- [BLAKE3 official repo](https://github.com/BLAKE3-team/BLAKE3)
- [BLAKE3 vs dhash comparison](https://mojoauth.com/compare-hashing-algorithms/blake3-vs-dhash/)
- [videohash repo (akamhy)](https://github.com/akamhy/videohash)
- [PHVSpec: Video Hash Benchmark](https://technologycoalition.org/wp-content/uploads/Tech-Coalition-Video-Hash-Benchmark-Paper.pdf)
- [DiffVC perceptual video compression (arXiv 2025)](https://arxiv.org/html/2501.13528v1)
- [MIRIX multi-agent memory (arXiv)](https://arxiv.org/abs/2507.07957)
- [Omdia: data ingest storage for AI/LLM pipelines](https://omdia.tech.informa.com/blogs/2025/sep/data-ingest-storage-the-unsung-hero-in-ai-and-llm-pipelines)

---

**End of Track G frontier scan.**
**Next**: 综合到 `40_synthesis/` 阶段, 与 Track A (CLI) / Track B (artifact) / Track D-F 其他 track 输出交叉验证。
