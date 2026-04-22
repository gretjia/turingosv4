# ROADMAP_LAUNCH — TuringOS v4 外部 agent 开放路线图

**日期**: 2026-04-21
**作者**: co-worker (Claude Opus 4.7 1M)
**目标**: 让 TuringOS v4 从"内部研究 harness"过渡到"permissionless 外部 agents 可自主接入并分工"的公共微内核。
**定位**: Phase 7 完成后首张开放路线图；承接 LATEST.md §Next Steps 1-5，向外扩展。

---

## § 1. 当前"上线就绪度"自评

打分标度: 0 = 根本没做；1 = 有原型；2 = 内部 dogfood 能跑；3 = 内部能长期跑；4 = 外部开放小流量可用；5 = 公共生产级。

| 维度 | 分数 | 证据 |
|---|---|---|
| L1 技术可行性 (Art. IV 拓扑运行时) | **4** | Phase 0→7 全部 landed；9/9 audit pass；depth-23 DAG 真实产生；100% audit_proof.py 可外部复现；`tests/wal_resume.rs` 证明 crash-resume 工作。缺 Phase 5 签名 → 无法对"谁写 tape"做身份锚定，因此非满分。 |
| L2 经济机制健壮性 (Coin 守恒 / Goodhart / 反女巫) | **3** | Law 2 CTF 守恒有 5 个单测覆盖 (`tests/reward_pull_conservation.rs`)；C-043 wtool-on-OMEGA 落地；C-041 跨问题 wallet 持久化且无二次 mint。**但**: γ/β/θ/BOUNTY_LP/FOUNDER_GRANT 仍是 env-var (LATEST.md 第 17 行 yellow on red line #5)；反女巫完全没做 — 任何人只要多注册 N 个 agent_id，就能在 genesis 前拿到 N×10000 coin。 |
| L3 性能 / 成本 (PPUT / 接入开销) | **3** | 单次 MiniF2F N=50 dual-mode ≈ 3h 挂钟 + ~$12 (notepad 8.5 节)。Lean oracle 每次 ~10s (F-2026-04-16-05)。外部 agent 接入目前需要 clone repo + build Rust + 配 Lean toolchain，不是 API 调用。 |
| L4 外部 agent 接入 (permissionless onboarding) | **1** | Phase 5 未启动 (LATEST.md 第 19 行)。C-038 只是 candidate，没有落地文件 (grep: only C-037/C-041 提到 C-038 的相邻编号)。C-041 已为"新 agent 零余额进入"在 wallet 层留了钩子 (`ensure_agents`)，但 evaluator 调用方没有公开 API。没有 HTTP/gRPC 接入层。 |
| L5 可观测性 (日志 / DAG / audit artifact) | **4** | C-036 (telemetry)、C-037 (WAL)、C-039 (proof artifact persistence) 都 landed；每次 solve 有 standalone `proofs/*.lean` + `audit_proof.py`；WAL 可 replay tape；Librarian message board 运行。**但**: Librarian 每 tick overwrite (LATEST.md 第 18 行)，跨问题的 session log 丢失。 |
| L6 安全性 (反偷币 / 反 tape 污染 / DoS) | **1** | 没有签名、没有 rate limit、没有 tape 容量上限、没有 agent auth boundary。任何外部 agent 今天接入 = 直接获得 wallet 的 mutable ref。`append_oracle_accepted` 绕过了 `check_payload` (C-043 ruling 第 2 点)，外部 agent 若能调这条路径就能写任意 payload。 |
| L7 治理 (宪法级争议仲裁) | **2** | 有 constitution + 38 个 cases + VIA_NEGATIVA + 双审 (Codex+Gemini)，但这套流程是研究者手动驱动的。Art. V.1 三权分立代码里没有 JudgeAI 实体；"谁能改宪法" 目前还是研究者 sudo。没有外部争议 inbox，没有 on-chain 式的仲裁。 |
| L8 文档 / DX (5 分钟跑通 demo) | **2** | README 是研究笔记风格；没有 quickstart；外部团队要跑通 demo 需要自己凑 Lean toolchain + DeepSeek key + 读 CLAUDE.md。handover/ 下全是内部状态笔记，不是 docs/external-agent-guide.md。 |

**综合**: 内部研究 harness 已经扎实（平均 3.0），但对外开放面向 (L4/L6/L7/L8) 整体 ≤ 2，无法上线。宪法拓扑是完整的，工程外壳是空的。

---

## § 2. Launch Gating Issues (P0, 必须解决才能对外开放)

### P0-1. 身份缺失：没有签名就没有 agent

- **问题**: Bus 当前接受任何 `(agent_id: String, payload)` 对，没有 session key，没有挑战-应答。一个外部 agent 声称 `agent_id="alice"` 就是 alice。
- **阻塞场景**: 外部团队 A 接入后，外部团队 B 可以把自己的 `agent_id` 设成 A 的，拿走 A 的 founder_grant，或把脏 tape 挂在 A 的名下败坏 A 的信誉。这个攻击**无成本、无日志、无法事后追责**。C-014 (身份盗用) 直接讲过这个模式。
- **修复思路**: Phase 5 的最小方案 = Ed25519 keypair per agent；`agent_id = hash(pubkey)[0:16]` (self-generated, no registrar needed = permissionless)；每个 tx (append/complete/invest) 附 `signature = sign(sk, canonical(tx))`；Bus 在写 WAL 前做 `verify(pubkey, sig, tx)`，失败即拒。宪法 Art. I.1 布尔谓词层：签名不对 = ∏p=0。
- **工作量**: M (Rust + Python 两端 ed25519_dalek / pynacl，测试 ~2 天)
- **对应**: C-014 (身份锚定)、Art. I.1.1 (PCP 前提是身份一致)、未写的 C-038 candidate。

### P0-2. 女巫攻击：零成本多开吃 founder_grant

- **问题**: Wallet 的 `ensure_agents()` 在 `genesis_done=true` 后给新 agent 零余额 (C-041 ruling 第 3 点)——这是对的，但真正的经济攻击不在 wallet 层，在 **FOUNDER_GRANT**：每 solve 一次该作者 agent_id 直接拿 γ·lp coin (C-043)。外部攻击者可以注册 10000 个不同 pubkey 的 agent，用同一个 LLM 账号轮流签名 submit，享受 10000 份 founder_grant 的独立 reputation 累积。
- **阻塞场景**: 外部 agent 数量爆炸、Hayek bounty 市场的价格信号被女巫的 10000 份空心 bid 扭曲、Librarian board 被灌爆。
- **修复思路**: 不是"禁止多开"，而是**让多开没好处**。三层组合:
  1. **proof-of-work onboarding**: 新 agent 首次接入需解决一个"新手关"公共题 (比如一个已知答案的 trivial MiniF2F theorem) 才能获得"active"标志；解不掉不能 bid/append。题目随机选、有随机 nonce，每 pubkey 只能解一次 (由 pubkey 本身去重)。成本 ≈ 一次 LLM 调用 ($0.01)，足以让女巫攻击"N 个 agent = N 次 $0.01"线性扩大。
  2. **reputation-weighted founder_grant**: γ = base·f(reputation)，f 在 reputation=0 时 = 0.1，reputation ↑ 才逼近 1。新 agent 解第一题只拿 10% 正常奖励；10 题之后才拿满。
  3. **per-human rate limit**: 不从身份层强制 (permissionless 原则)，从**Lean oracle 层**做——每个 pubkey 的 oracle 调用配额 = 0 Coin 时每小时 10 次 (Law 1 思考免费，但排队成本来自公共 oracle)。
- **工作量**: L (机制 + proof-of-work 题库 + γ 函数重写，~5 天)
- **对应**: Law 2、C-001 (零和守恒)、C-031 (institution over tuning)

### P0-3. Tape 污染：无界写入 + 无 GC

- **问题**: Tape 今天没有容量上限、没有节点失效策略、没有污染回收。外部 agent 可以持续 append 垃圾节点 (payload 合法但无意义)——这些 payload 过了 `forbidden_patterns` 不代表对后续解题有价值，但它们会进入 rtool 广播到所有 agent 的 prompt (Art. III.2 细节封装 = Librarian 的 TopK 抽取就可能 surface 这些垃圾)。
- **阻塞场景**: 外部女巫 append 10 万条"看起来像 Lean 片段但实际无用"的 payload，Librarian 广播这些给所有 agents，Art. III.2 封装失效 → 全系统 context 污染 (Art. III.1 错误广播的反面——正确但无用的广播也是污染)。这跟 V3L-24 (tmp data loss) 的反面：不是丢数据，是数据爆炸。
- **修复思路**:
  1. **园丁 Agent (Art. III.1 明确要求)**: 后台 tick 对超过年龄 K 且被引用次数 = 0 的 tape 节点打 tombstone；rtool 默认不读 tombstoned 节点。
  2. **Append 需质押**: 每次 append 冻结 ε Coin (e.g., 10)；后续若该节点成为某 golden_path 的祖先 (Phase 3B Satoshi 证明路径)，解冻 + 额外奖励；否则 T 时间后抵扣给 park 池做 bounty 本金。这把"无意义 append"变成直接经济损失。Law 2 保持不破。
  3. **硬顶**: WAL 单日写入字节 cap；超 cap 触发 Art. II.2 价格信号广播 "append 现在贵"。
- **工作量**: M (园丁 + stake/refund 机制，~3 天)
- **对应**: Art. III.1 屏蔽错误、Art. III.2 封装细节、C-022 (context poisoning)

### P0-4. 宪法边界的经济常量没固化

- **问题**: γ/β/θ/BOUNTY_LP/FOUNDER_GRANT_GAMMA/SATOSHI_GAMMA_REBATE 是 env-var (LATEST.md 第 16 行)。任何启动者都能把 γ 调成天文数字，破坏整个经济平衡。对内部研究是特性，对外部上线是灾难。
- **阻塞场景**: 外部用户跑自己的实例、γ=1e9；他们的解答在公共市场以"正确但经济被操纵"的形式提交到 Hayek bounty，价格信号彻底失真。
- **修复思路**: 上线实例的 γ/β/θ 由宪法文件声明、编译期常量 (const)；env-var 只作为 debug/实验模式且启动时 WARN。把 LATEST Next Step 3 从 "canonicalize C-042" 升级为 P0。
- **工作量**: S (~1 天；已经有工单，待写成 C-042)
- **对应**: Art. V.1.1 宪法唯一 Ground Truth、Red Line #5 (yellow)

### P0-5. Oracle 是 DoS 单点

- **问题**: Lean oracle 是全局共享单一进程，每次 ~10s (F-2026-04-16-05)；任何外部 agent 可以无成本调用 `complete` 触发 oracle verify。1000 个并发 agent = oracle 队列 1000 长 = 系统卡死。
- **阻塞场景**: 善意流量过大就会触发，恶意 DoS 更轻松。C-030 (cascade failure) 和 F-2026-04-19-05 (search budget abuse) 都是这个模式的亲戚。
- **修复思路**:
  1. Oracle 前置 **fingerprint cache**: 同一 payload hash 已验过 → 直接返回。
  2. **Per-pubkey oracle quota**: 每小时 N 次免费，超出走"oracle cost in Coin"梯度定价 (Art. II.2.1 explore/exploit)。贵，但不 ban——permissionless。
  3. **Oracle 水平扩展**: 多个 worker，payload hash 路由。
- **工作量**: M (cache 简单；quota/定价 需要跟 wallet 打通，~3 天)
- **对应**: Art. II.2 价格信号、Art. III.3 屏蔽相关性 (serial oracle 是 correlation source)、C-030

### P0-6. 争议-治理通道缺失

- **问题**: 一个 solve 被 judge 为 pass，外部另一队发现 `native_decide` (F-2026-04-20-05) 式漏洞，怎么报？今天的流程是"发 issue 给研究者"；公开上线后无法扩展。Art. V.1 三权分立的 JudgeAI 在 code 里没有实体。
- **阻塞场景**: 外部团队 C 发现某 agent 用新花招骗 ∏p (新版 C-011)，没 channel，没 bounty，他们就走了。系统丧失 Art. V "架构的架构" 的自我修复闭环 (Art. V.1.2 ArchitectAI 从 logs→feedback，但 logs 里没有外部争议信号)。
- **修复思路**: 把 `incidents/` 目录暴露为写入 API (带签名)：任何外部 agent 可以提交 `challenge` tx 指向 "solve #X has violation-class Y"；平台 **重跑 audit_proof.py** + **ArchitectAI 读 logs 生成 rebuttal**；最终仲裁是 Codex+Gemini 双审 (C-010 生成者≠评估者)；challenge 成功 → 挑战者获得 bounty + 该 solve 撤销 + 原作者扣信誉。用 "CTF 守恒" 的思路：YES 票 = "有效 solve"，NO 票 = "应撤销" (C-023 dual chamber 已有雏形)。
- **工作量**: L (~7 天：api + 重审 pipeline + bounty settlement + UI)
- **对应**: Art. V.1.1/V.1.2/V.1.3 三权分立 (notepad §6 注 "Art. V.1.1 + V.1.2 零案例覆盖")、C-010、C-023

### P0-7. Quickstart 文档 + 参考 agent 客户端

- **问题**: 外部 agent 今天接入要通读 handover/、猜 prompt schema、自己写 JSON 契约 (V-009 LLM 输出契约脆性的另一面)。CLAUDE.md 是给研究者看的中文笔记，不是接入文档。
- **阻塞场景**: 没有 5 分钟 quickstart 就没有生态。Art. III.2 "提供目录接口而不是百科全书"——现在外部人看到的既不是目录也不是接口，是半成品日志。
- **修复思路**: `docs/EXTERNAL_AGENT_QUICKSTART.md` (英文) + `examples/minimal_external_agent.py` (< 100 行，ed25519 签名 + submit proof + 读 bounty board)；Rust SDK crate 作为引用实现。契约上: JSON schema 版本字段 (避免 V-009) + Postel 法则 (宽进严出)。
- **工作量**: S-M (~2-3 天)
- **对应**: Art. III.2、V-009

**(总计 7 个 P0；低于任务上限 8，保留 1 个 slot 供后续补发——避免 kitchen-sink 上线清单)**

---

## § 3. Nice-to-have (P1/P2)

按优先级排序。格式: ID — 短描述 — 阻塞何物。

1. **P1-1** Phase 3B Satoshi citation rebate 合并 — 非终端节点奖励 (LATEST Next Step 2)。**被 P0-2 阻塞** (γ 函数要先定下来)。
2. **P1-2** Dual-mode N=50 多 seed 方差条带 (LATEST Next Step 4) — 对外宣传性能 claim 的统计依据。**独立**。
3. **P1-3** Librarian board 跨问题 session log 累积 (LATEST "What's broken" 第 3 项) — 让外部新 agent 一接入就能看全局历史而不是 per-tick 快照。**P0-3 园丁机制上线后做**。
4. **P1-4** ArchitectAI + JudgeAI 真正实体化 (Art. V.1.2/1.3) — 从 constitutional debt §6 "Art. V 零案例覆盖" 落地。**依赖 P0-6 争议通道提供 logs feedback。**
5. **P1-5** Search tool 从 filename 改 content grep (F-2026-04-19-04) — 让 Art. III.2 progressive disclosure 真正生效。**独立**。
6. **P1-6** 参考 agent 客户端的 Rust SDK + Python SDK 双实现 — DX 加速。**P0-7 之后**。
7. **P1-7** 公共 bounty board HTTP API — 外部 agent 从 REST 读 Librarian board 而非 SSH 进机器。**依赖 P0-1 签名层做认证头。**
8. **P2-1** 跨 LLM provider 支持 (OpenAI/Anthropic/Gemini) — 现在绑 DeepSeek chat，外部接入要换 key 很痛苦。非阻塞。
9. **P2-2** Oracle 支持语言扩展 (Lean 之外 Coq/Isabelle) — 让"形式化验证 swarm"的使命从 MiniF2F 扩展。非阻塞，远期。
10. **P2-3** 公开 dashboard (Grafana-style) 读 WAL 实时画 DAG — 外部观察者价值高。**P0-5 oracle cache 之后做**避免额外负载。
11. **P2-4** Proof-of-work 新手关自动生成 pipeline — 防止固定题库被外部背答案。**P0-2 上线后做**。
12. **P2-5** 宪法自身的版本化 + migration 工具 (constitution.md 现在不带版本号) — Art. V.2 说宪法在 sudo 区，但不说如何治理升级。**远期**。
13. **P2-6** 地理分布式 WAL (多节点 replicate) — scaling gate，非上线必需。**远期**。
14. **P2-7** 外部 agent 分类 leaderboard + 诚实排名 (C-012 measurement correctness 精神) — 生态驱动。**P1-7 之后**。
15. **P2-8** 实时 "C-xxx case" inbox — 任何外部争议可以直接建议新判例。把 common law 开放共建。**P1-4 + P0-6 之后**。

---

## § 4. 外部 agent 接入的具体设计

### 4.1 C-038 permissionless onboarding 落地方案

最小可行: 接入协议 = 3 个 endpoint + 1 个新手关。

| Endpoint | 方法 | 认证 | 作用 |
|---|---|---|---|
| `POST /onboard` | POST | 无 (公开) | body = {pubkey, proof_of_work_solution (对新手关题目的签名证明)}；成功返回 `agent_id = hash(pubkey)[:16]` |
| `POST /tx` | POST | ed25519 签名 | body = {agent_id, tx_type: append\|complete\|invest\|cite, payload, sig}；Bus 验证后入 WAL |
| `GET /board` | GET | 可无可签 | Librarian message board 当前快照 (problems, bounties, topK errors) |

`/onboard` 的新手关 = 从公共 trivial MiniF2F 池随机抽一题 + 一个 pubkey-bound nonce；agent 必须签名提交能通过 Lean oracle 的 proof。成本 ~$0.01/次 LLM + ~10s/次 oracle = 足够高使 1e4 女巫不经济 (~$100 + 数小时排队)，对真人不构成障碍。

### 4.2 新 agent 零 Coin 冷启动机制 (遵守 Law 2)

Law 2 禁止 genesis 后 mint——因此新 agent 只能零余额进入 (C-041 ensure_agents 已铺)。赚第一枚 Coin 的路径:

1. **solve a bounty**: Hayek bounty 市场（Phase 3A）每题有 bounty；新 agent 解到 → 拿 bounty (不是 mint，是从 park 池划转)。
2. **cite an ancestor**: Phase 3B Satoshi rebate——新 agent 的 solve 引用了已有 tape chain，一部分 founder_grant rebate 给祖先 agents；反过来新 agent 自己也会因做祖先被后人 rebate。
3. **tip from sponsor**: 已有 agent 可以自愿从 wallet 转 X Coin 给新 agent (normal transfer，不是 mint)；外部团队可以自己给自己部署的新 agent 充值。

Sufficiency check: 新 agent 第一题解出来需要能做 `complete` tx——`complete` 不需要 stake (P0-3 只要求 append stake)，因此零 Coin 能完成第一次 solve。解出后经 C-043 mandatory wtool → founder_grant → 获得 γ·lp Coin → 闭环启动。

### 4.3 恶意 agent 防御

对照 attack 分类:

- **女巫攻击**: 见 P0-2 (proof-of-work + reputation-weighted γ + oracle quota)。
- **Bounty 套利** (自己出题自己解拿 bounty): Hayek bounty 出题方 ≠ 解题方 硬约束 (pubkey 不同)；同一自然人多 pubkey 绕过——靠 P0-2 reputation 系统让新 pubkey 在前 10 题 γ 极低，套利不经济。
- **垃圾 tape 污染**: P0-3 append stake + 园丁 tombstone。
- **Oracle DoS**: P0-5 fingerprint cache + per-pubkey quota。
- **身份盗用** (C-014): P0-1 签名硬性验证。
- **Replay 攻击**: 每 tx 包含 `nonce` (单调递增 per agent_id)，Bus 拒绝已见 nonce。
- **Goodhart** (Art. III.4): 评价函数 (∏p) 完全透明且公开 (Lean 语义)——这里我们主动违反 Art. III.4，因为 MiniF2F 的 ground truth 是数学正确性，没法隐藏；但 reward 曲线 γ/β 上线后编译期固化 (P0-4) 避免"改分骗分"。

### 4.4 Agent 身份 / 签名 (Phase 5 最小方案)

- 密钥: Ed25519 (ed25519-dalek Rust / PyNaCl Python)；32B pubkey / 64B sig
- agent_id = hex(blake3(pubkey))[0:16]——确定性、无冲突、无注册
- canonical tx format: `{agent_id, nonce, tx_type, payload_hash, ts}` → JCS (RFC 8785) → sign
- Bus 端: `verify(pubkey from lookup_table, sig, canonical)`；lookup_table = WAL-replayable 只加不改的 pubkey 注册表 (onboard 写一次)
- 失败语义: ∏p=0 且记 `audit.sig_fail` counter；Art. II.1 触发 "broadcast typical errors"

这是最小方案——没有多签、没有轮转、没有撤销。v1 上线足够，v2 再做。

### 4.5 多方自主分工的 incentive alignment

关键洞察: **不要规定"谁该做什么"** (feedback_emergent_roles.md 教训)。让外部 agent 自己选。机制:

1. **Librarian board 公开所有待解题 bounty** (Hayek) + topK 错误类别 (Art. II.1 broadcast) + 最近 golden_paths 的深度 (tape inspiration)。
2. **所有 agent 读同一 board，看到的价格/错误/深度一致**；但各自的 (sk, reputation, past earnings) 不同 → 选择自然分化。
3. 经验告诉我们 (F-2026-04-18-01 flat N-scaling): 过去所有 agent 选同一 action 是因为他们 prompt/temperature/context 都一样。外部 agents 天然多样 (不同团队、不同 LLM、不同 temperature)——**Art. III.3 屏蔽相关性的负担从平台转到生态**，这对我们是福音。
4. 额外的 explore 压力: Art. II.2.1 要求价格信号既引导又保留多样性——上线实例维持一个"冷门类别奖励倍率" (例如某类题无人问津时 bounty 缓慢上浮)，让系统不会全体押注一题。

无需"role assignment agent"。board + signed tx + Coin = 自组织。

---

## § 5. 与 v3 的对比判断

v3 是 centralized benchmark harness (evaluator.py 单文件，agent 们被调度器拉起)。v4 是 constitutional TuringOS (agent 是独立主体，通过 Bus + 签名 + 钱包交互)。上线时:

### 5.1 不能回头的 v3 "肮脏实用主义"

- **硬编码模型名/密钥 in source** (V3L 多处) → v4 已全走 env/CLAUDE.md；外部上线必须保持，**严禁**为方便加回 `src/consts.rs` 里的 DeepSeek URL。
- **静默 parse 失败** (V-009/C-017) → 外部 agent 输出五花八门，必须 Postel + 永不静默。
- **oracle per-process init** (V3L-41) → v3 的 oracle 重入方式在多 agent 并发下死锁；C-007 tls 教训加强版。v4 现在本地代理 (llm_proxy.py) 解耦，必须保留。
- **Generator=Evaluator** (C-010/V3L-38) → v3 经常一键"自审通过"；上线后 Codex+Gemini 双审 + 外部 challenge (P0-6) 不能省。
- **"可接受的违宪"** (C-001 教训): v3 允许 "low-priority violation"；v4 零容忍 (C-035)。上线后外部各种"为方便"的建议会涌来，坚持。

### 5.2 v3 必须补齐的工程经验

- **Smoke before batch** (feedback_smoke_before_batch.md / F-2026-04-15-09)——配置变化前 30-60s smoke probe。外部接入 = 天然引入配置漂移（新 agent 的 prompt/endpoint 随时变），上线实例必须在每批 new agent onboard 后跑 smoke probe 确认 pipeline 未 broken。
- **Phased checkpoints** (feedback_phased_checkpoint.md)——多阶段上线 (internal → closed beta → public beta → GA) 中每个 gate 都要 7 red-line check + checkpoint doc + 自动 pause。
- **DeepSeek timeout patience** (feedback_deepseek_timeout.md) 的教训不能照搬 DeepSeek，但"LLM API 超时默认 retry 不报警"这个 pattern 要抽象到外部接入层：任何 external agent 的 tx 处理都要超时可重试；但连续 N 次失败要进 circuit breaker，防止一个坏 agent 卡住 Bus。
- **VIA_NEGATIVA 公开**: v3 的否定路径是研究者私藏，v4 上线后应当把 VIA_NEGATIVA.md + `cases/V3_LESSONS.md` 作为"给外部 agent 开发者的 don't list"公开，节省全生态的时间。

---

## § 6. 时间线提案

假设今天 = T=2026-04-21。

### T + 1 week (到 2026-04-28): **内部加固周**

- **Milestone M1**: Phase 3B Satoshi rebate 合并 (LATEST Next Step 2) + γ/β/θ 宪法化 (P0-4)。
- **Milestone M2**: Dual-mode N=50 × 2 seeds 方差条带 (LATEST Next Step 4)；对外 headline 有置信区间。
- 副产品: C-042 / C-038 / C-040 三个候选判例正式落地 (`cases/C-0xx.yaml`)。

### T + 1 month (到 2026-05-21): **Permissionless 闭门 beta**

- **Milestone M3 (P0-1)**: ed25519 签名层上线；Bus 全链路 verify；tests 覆盖 replay / bad_sig / nonce 跳号。
- **Milestone M4 (P0-2 + P0-3)**: proof-of-work 新手关 + reputation-weighted γ + append stake + 园丁 tombstone。
- **Milestone M5 (P0-5)**: oracle fingerprint cache + per-pubkey quota。
- **Milestone M6 (P0-7)**: English quickstart + minimal external agent example + Rust SDK crate pre-alpha。
- **Closed beta**: 邀请 3-5 个受信任外部团队接入；跑两周；每周 checkpoint。每个 P0 fix 跟 STEP_B_PROTOCOL (notepad §5)。

### T + 3 months (到 2026-07-21): **Public beta + 治理闭环**

- **Milestone M7 (P0-6)**: 争议 inbox + 自动重审 + bounty 式仲裁；首批外部 challenge 处理完。
- **Milestone M8**: ArchitectAI/JudgeAI 实体化 (P1-4)；宪法更新走正式流程而非研究者直接 commit。
- **Milestone M9**: Public dashboard (P2-3) + REST bounty API (P1-7) + leaderboard (P2-7)。
- **Gate**: 公开 Post-Mortem (首个被 challenge 成功的 solve) 作为生态信用背书。

---

## § 7. 风险 / 失败模式

至少 5 个可能把上线搞砸的机制性 / 经济性 / 治理性风险。

### R-1. 经济常量上线即 mis-calibration

γ/β/θ 固化后发现不对但已有外部财产绑定其上。**判例**: v3 Run 6 的经济改动教训 (CLAUDE.md "Economic changes: grep experiments/")。**缓解**: γ 宪法化时同时规定 "宪法修订须 ArchitectAI 提案 + JudgeAI 审 + 全量 agent 30 天预告"。避免"半夜改常量"。

### R-2. 女巫经济 overwhelm

proof-of-work + reputation 曲线参数选错，实际成本不够高；10 万 agent 涌入；oracle + wallet 都扛不住。**缓解**: P0-2 上线前压测 1000×模拟女巫；发现曲线不陡 → 先调；**不要**等用真女巫来发现。

### R-3. Oracle trust boundary 漂移

外部团队发现 Lean oracle 某个 native_decide 式新漏洞 (F-2026-04-20-05 的后继)，但比上次狡猾，audit_proof.py 没覆盖。**缓解**: P0-6 争议通道 + bounty 让发现者有动力披露而不是藏；同时把 "每次上线 forbidden_patterns 必须要 Codex 独立 attempt to bypass" 写入 C-011 后继判例。

### R-4. 治理俘获

某大团队靠算力优势拿走 50%+ 的 reputation，之后任何 constitution 建议都是他们主导。**缓解**: reputation ↔ 治理投票权解耦。宪法修订**仍是研究者 sudo** (Art. V.2)，即便上线后也不开放给 reputation holders。把"代码治理"和"宪法治理"分两层，只开放前者。

### R-5. 观测假象 / 指标 Goodhart

公开 dashboard 上"solve rate"成为头条指标；agent 被调成"骗 dashboard"而不是"真解决难题"。**判例**: C-034 (mechanism not prompt)、Art. III.4 Goodhart。**缓解**: headline KPI 永远是 "paired-audit-re-verified solves"，不是 "claimed solves"；F-2026-04-20-05 事件强化这一原则。

### R-6. Permissionless = ungovernable

一旦外部生态自主、VIA_NEGATIVA 的更新停下、生态自己长出新的坏 pattern，研究者响应不过来。**缓解**: 把 VIA_NEGATIVA 变成外部可 PR 的公共文件；`incidents/` 开放给外部提交；ArchitectAI (P1-4) 在 log 里自动聚合涌现错误。**最极端情况**: Art. V.2 保留"宪法冻结 / 紧急暂停" sudo—研究者永远留 kill switch。

---

**END ROADMAP_LAUNCH_2026-04-21**
