# External Audit Result: GEMINI
**Date**: 2026-04-21
**Elapsed**: 115.8s
**Prompt size**: 183,782 chars

---

# Gemini External Audit — 2026-04-21

## Q1. Law 2 CTF 守恒的完备性证明

**结论**: Law 2 守恒在核心逻辑上得到遵守，但关键路径 `invest refund` 和 `Hayek bounty payout` 确实未被 `reward_pull_conservation.rs` 中的测试覆盖。虽然代码审查未发现明显漏洞，但缺乏显式测试覆盖是一个风险。

---

-   **所有可能改变 total_supply 的代码点**:
    1.  **铸币 (Minting)**: `src/sdk/tools/wallet.rs:141` (`on_init`)。这是唯一合法的铸币点，通过 `if self.genesis_done { return; }` 卫语句确保只执行一次。
    2.  **借记 (Debit)**: `src/sdk/tools/wallet.rs:52` (`deduct`)，由 `src/bus.rs:410` (`debit_wallet`) 调用。
    3.  **贷记 (Credit)**: `src/sdk/tools/wallet.rs:67` (`credit`)，由 `src/bus.rs:426` (`credit_wallet`) 调用。

-   **各路径守恒证明**:
    -   **`on_init`**: 这是唯一的铸币事件，符合宪法 "on_init 是唯一合法铸幣点"。
    -   **`invest` (bus.rs:235-253)**:
        1.  `self.debit_wallet(author, amount)`: 从 agent 账户扣除 `amount`。
        2.  `self.kernel.buy_yes/buy_no(...)`: 将 `amount` 注入市场储备池。
        3.  **失败路径 (refund)**: 如果 `buy_yes/buy_no` 失败，`self.credit_wallet(author, amount)` 会将等额 `amount` 退还给 agent。
        -   **守恒分析**: 在此路径中，`Σ debit = Σ credit`。资金要么从 agent 转移到市场（守恒），要么在失败时被退还（净效应为零，守恒）。
    -   **`settle_portfolios` (bus.rs:386-408)**:
        1.  从已解析的市场 (`self.kernel.markets`) 中计算每个 agent 的 `payout`。
        2.  `wallet.credit(&agent, amount)`: 将 `payout` 贷记到 agent 账户。
        -   **守恒分析**: `payout` 的资金来源是市场创建时注入的流动性池（LP）。这是一个**转移支付**，将 LP 资金根据市场结果重新分配给获胜的持股人。没有新币被创造。
    -   **`halt_and_settle` / Hayek bounty (bus.rs:357-360)**:
        1.  `self.kernel.resolve_bounty(&gp_authors)`: 计算奖金池支付。
        2.  `kernel.rs:103`: `payouts` 的总和被严格限制为 `self.bounty_lp_seed`，即市场创建时注入的 LP。
        3.  `self.credit_wallet(&agent, amount)`: 将奖金支付给 agent。
        -   **守恒分析**: 这同样是**转移支付**，将预先承诺的奖金池 LP 分配给黄金路径的贡献者。没有新币被创造。

-   **测试覆盖审计**:
    -   `tests/reward_pull_conservation.rs` 包含 5 个测试，主要关注 "founder grant" 机制（TAPE_ECONOMY_V2）的贷记和结算。
    -   **未覆盖路径**:
        1.  **`invest refund` 路径**: 没有任何测试模拟 `kernel.buy_yes` 失败并验证 `credit_wallet` 是否被正确调用以实现资金返还。
        2.  **`Hayek bounty payout` 路径**: 没有任何测试设置 `HAYEK_BOUNTY=1` 环境变量，然后调用 `halt_and_settle` 并验证黄金路径作者的余额是否正确增加。
    -   Claude 内部审计的结论是正确的。

-   **反例构造**:
    -   当前代码逻辑审查未发现可构造反例的缺陷。`invest refund` 路径明确地 `credit` 了与 `debit` 相同的 `amount`。`Hayek bounty` 的总支付额在 `kernel.rs` 中被其初始 `lp_seed` 约束。
    -   然而，由于缺乏测试，未来对这些路径的重构可能会无意中引入破坏守恒的 bug。例如，如果在 refund 逻辑中错误地乘以一个系数，或者 `resolve_bounty` 的计算超出 `lp_seed`，守恒就会被破坏。

## Q2. Phase 7 DAG depth 分布的"涌现"显著性

**结论**: 分布具有真实构造的特征，无法通过简单背诵得到。`imo_1964_p2` 等深层证明的策略选择体现了必须通过运行时反馈才能获得的洞察。然而，N=20 的样本量不足以做出强统计学结论。

**判决**: **CHALLENGE** (证据方向正确，但统计强度不足)

---

-   **单 agent 背诵 vs. 多 agent 构造**:
    -   该分布**不能**从简单的"单 agent LLM 背诵已知证明"得到。
    -   原因是实验环境设置了 `TURING_STEP_ONLY=1`。这意味着 agent 每次只能提交一个 tactic (`step`)，该 tactic 必须通过 `lean4_oracle.rs:249` 的 `PartialVerdict::PartialOk` 或 `PartialVerdict::Complete` 验证，才能被写入 tape (`wtool`)。
    -   这个机制强制 agent 必须**逐步构造**一个有效的证明链。它不能一次性提交一个完整的、可能包含错误的23行证明文本。每一步都是一个状态转换 `Q_t -> Q_{t+1}`，并由 Lean oracle 这一谓词 (`∏p`) 严格守卫。因此，深度大于1的 DAG 是真实构造的结果。

-   **Tactic 证据 (LLM-plausible vs. runtime feedback)**:
    -   **depth-23 (imo_1964_p2)**:
        -   **Tactic 证据**: `have h12 : (a - b)^2 * c + (b - c)^2 * a + (c - a)^2 * b ≥ 0 := by nlinarith`
        -   **分析**: `nlinarith` (non-linear integer arithmetic) 是一个强大的决策过程。LLM 可以"猜测"这个引理并尝试用 `nlinarith` 证明它。然而，这个引理并非显而易见，且 `nlinarith` 能否成功取决于上下文中已有的其他假设（如 `a,b,c` 的正性）。LLM 很可能尝试了多个无效的代数重排或引理，被 oracle 的 `PartialVerdict::Reject` 拒绝后，才最终生成了这个可被 `nlinarith` 解决的有效形式。这体现了需要运行时反馈的探索过程。
    -   **depth-20 (mathd_algebra_332)**:
        -   **Tactic 证据**: `have h3 : x * y = 19 := by apply (pow_inj ...)`
        -   **分析**: 日志 (`templadder_n8_20260421T164014.jsonl`) 显示了 agent 在此之前的多次失败尝试，例如 `rw [eq_of_sqrt_eq_sqrt h₁] at h₁; exact?`。最终成功的 `pow_inj` (power injectivity) 策略是一个更复杂、更严谨的解决 `sqrt(xy) = sqrt(19)` 的方法。这种从简单、失败的策略转向复杂、成功的策略的演进，是典型的需要运行时反馈（Lean oracle 的拒绝信息）才能引导的修正过程。
    -   **depth-17 (imo_1981_p6)**:
        -   **Tactic 证据**: `have hf1_spec' : f 1 0 = 2 := by rw [h₁ 0, h₀ 1]; simp`
        -   **分析**: `PHASE7_DAG_ANALYSIS.md` 指出此问题的拒绝率高达 73%。这个 tactic 链 `rw [h₁ 0, h₀ 1]; simp` 是一个精确的操作序列。LLM 很难一次性生成这个完美的序列。更有可能的是，它尝试了 `rw [h₁ 0]`，被 oracle 拒绝或发现目标没有进展，然后根据新的状态添加了 `h₀ 1` 和 `simp`。这种精确的、多步骤的重写链是试错和运行时反馈的强有力证据。

-   **统计判定**:
    -   **(a) N=20 样本是否足够?**: **不足够**。虽然观察到 `{1:5, 3:1, 17:1, 20:1, 23:1}` 这样的多样性比 `{1:9}` 更能支持"涌现"的说法，但 N=20 的样本量太小，容易受到随机性的影响。这可能是一次"幸运"的运行，恰好抽到了一些模型擅长逐步解决的难题。无法排除可能性：在另一批20个问题上，结果可能是 9/20 的 solve 且深度全为1。
    -   **(b) 需要多少 seeds?**: 为了达到 95% 的置信度，需要显著增加样本量以平滑掉单次运行的噪声。一个合理的方案是进行 **3-5 次独立 seed 的运行，每次 N≥50**。如果多样化的深度分布在多次运行中稳定复现，那么"真涌现"的结论将具有很高的置信度。

## Q3. Karpathy TOP-10 性能数值复核

| 条目 | Claude 估计 | Gemini 独立估计 | 差异说明 |
|---|---|---|---|
| `bus.rs:244-246` 三重 clone | ~10-20% per tx | 每 `invest` tx 节省 **~200B** | Claude 的百分比估计过于乐观。节省一次 clone 的成本远低于整个 `invest` 交易的 CPU 成本（包括 map lookup, market math 等）。节省的绝对字节数是更精确的度量。 |
| `bus.rs:256-262` `to_string()` | 20-30% heap | 总节省 **~4.1 MB** | 估算基于 N=50 problems × 8 agents × 180 tx avg = 72,000 tx。假设 `author` 平均 8B，`payload` 平均 50B，总节省 `72000 * (8+50) ≈ 4.1MB`。这部分内存是永久驻留的（在 `kernel.tape` 中），因此节省是显著的。Claude 的百分比估计在合理范围内，但绝对数值更具可操作性。 |
| `bus.rs:416-424` `Box<dyn>` downcast | ~5× 加速 | **~1.05× - 1.1×** 加速 | Claude 的估计严重失真。`downcast` 本质上是一次 vtable 查找加一次类型 ID 比较，耗时仅为几个 CPU cycle。在只有 3-5 个 tool 的情况下，`enum` dispatch（通常是 jump table）相比动态分派的优势微乎其微。5倍的加速意味着 downcast 占了函数 80% 的执行时间，这在物理上是不可能的。实际加速效果几乎无法测量。 |

## Q4. reputation-weighted γ 曲线（P0-2 女巫防御）数学设计

-   **γ(n) 的数学形式**:
    采用 **S型函数 (Sigmoid)**，因为它光滑、单调递增且有饱和区，符合所有要求。
    `γ(n_solved) = γ_max / (1 + exp(-k * (n_solved - n_0)))`
    其中 `n_solved` 是该 agent 解决的问题数量。

-   **参数选择**:
    -   `γ_max`: 宪法规定的最大 founder grant 率。从代码 `bus.rs:307` 中 `FOUNDER_GRANT_GAMMA` 的默认值推断，可设为 **`0.05`** (即 5%)。
    -   `n_0`: 曲线的拐点，即达到 50% grant 所需的解决题数。设为 **`n_0 = 25`**。
    -   `k`: 曲线的陡峭度。为了满足 "首 10 题只能拿 ≤10% 正常 grant" 的要求，我们求解：
        `0.1 * γ_max = γ_max / (1 + exp(-k * (10 - 25)))`
        `10 = 1 + exp(15k)`
        `9 = exp(15k)`
        `k = ln(9) / 15 ≈ 2.197 / 15 ≈ 0.146`
    -   **最终函数**: `γ(n) = 0.05 / (1 + exp(-0.146 * (n - 25)))`

-   **女巫攻击总获利上限**:
    假设攻击者用 M 个账号解决 N 个总问题，平均每个账号解决 `n = N/M` 个。
    -   **单账号总收益**: `Profit(1, N) = C * Σ_{i=0}^{N-1} γ(i)` (C 为常数)
    -   **M 个账号总收益**: `Profit(M, N) = M * C * Σ_{i=0}^{N/M - 1} γ(i)`
    -   由于 `γ(n)` 是一个 S 型增长函数（在初期是凸函数，后期是凹函数），将解题数分散到多个账户会使每个账户都停留在曲线的低收益起始段。因此，`Profit(M, N) < Profit(1, N)`。
    -   攻击者为了最大化收益，必须将所有算力集中在一个账号上。因此，开设 M 个账号的总获利与 M 的关系是 **`O(1)`** (对于固定的总解题数 N)，因为最优策略是 M=1。这使得女巫攻击在经济上无利可图。

-   **与 C-001 (post-genesis minting forbidden) 的兼容性**:
    **兼容**。Founder grant 并非铸造新币。如 `bus.rs:301-305` 注释所述，这些份额是从市场创建时预先注入的流动性池（LP）中划分出来的。这是一个**转移支付**，将一部分系统控制的 LP 份额的所有权转移给节点创建者，总币量不变。`γ(n)` 函数仅用于动态计算这次转移的份额大小，完全不涉及铸币。

## Q5. fingerprint cache（P0-5 oracle DoS 防御）数学模型

-   **Cache hit rate 估计**:
    -   **总调用量**: 50 problems × 8 agents × 180 tx × 50% Lean 调用率 = **36,000 次** Lean 调用。
    -   **重复来源**:
        1.  **Agent 内重复**: Agent 在一个证明中可能会多次尝试相同的失败 tactic。
        2.  **Agent 间重复**: 多个 agent 独立解决同一问题时，可能会发现相同的 tactic。
        3.  **问题间重复**: 像 `simp`, `linarith`, `norm_num` 这样的通用 tactic 在许多问题中都会被使用。
    -   **估计**: 这是一个重尾分布。少数通用 tactics 会被频繁调用。保守估计，至少有 **20-35%** 的调用是重复的。这个数字依赖于 agent 策略的多样性。
    -   **结论**: 预估 cache hit rate 在 **20-35%** 范围。

-   **总 memory 开销**:
    -   10,000 entries × 100B/entry = 1,000,000 Bytes = **1 MB**。
    -   这个开销对于现代服务器来说是微不足道的。

-   **与 2x oracle worker 对比**:
    -   **Cache**:
        -   成本: ~1MB RAM + 实现复杂度（一次性）。
        -   收益: 假设 30% hit rate，将 oracle 负载降低 30%，有效吞吐量提升至 `1 / (1 - 0.3) ≈ 1.43x`。
    -   **2x Oracle Workers**:
        -   成本: 增加一倍的进程/容器资源（CPU, RAM），持续性开销。
        -   收益: 吞吐量提升至 `2x`。
    -   **结论**: **Cache 方案的成本效益远高于增加 worker**。它用极低的资源成本实现了显著的性能提升。只有当系统负载超过 `1.43x` 且 cache 命中率无法再提高时，增加 worker 才是必要的下一步。

## Q6. dual-mode N=50 实验的 power analysis

-   **N=50 下的统计功效 (power)**:
    -   **假设检验**:
        -   H0 (零假设): `p_dual = p_mono = 17/20 = 0.85`
        -   H1 (备择假设): `p_dual ≤ 15/20 = 0.75` (效果至少下降2个solve)
    -   **参数**: `n=50`, `α=0.05` (显著性水平), `power=0.8` (目标功效)。
    -   **计算**: 使用标准功效分析工具或公式，对于 `p1=0.85`, `p2=0.75`, `n=50`, `α=0.05` (单尾检验)，计算出的统计功效约为 **46%**。
    -   **结论**: 这个功效远低于 80% 的可接受标准。**N=50 的样本量不足以**有信心地检测出 2 个 solve 的性能下降。有超过 50% 的概率我们会犯第二类错误（即，即使性能真的下降了，实验结果也无法显示出统计显著性）。

-   **需要的 seed 数量**:
    -   要达到 80% 的功效来区分 0.85 和 0.75 的比率，需要的总样本量约为 **N=185**。
    -   因此，需要进行 **4 个独立的 N=50 seed 配对** (`4 * 50 = 200`) 才能压制伯努利噪声并获得足够的统计功效。

-   **对 `{35/50, 37/50, 33/50}` 结果的解读**:
    -   **数据**: 三个 seed 的成功率分别为 `0.70`, `0.74`, `0.66`。
    -   **总计**: 总共解决了 `35+37+33 = 105` 个问题，总尝试 `50*3 = 150` 次。平均成功率为 `105/150 = 0.70`。
    -   **统计推断**: 我们可以为这个 `0.70` 的平均成功率计算一个 95% 置信区间。`CI = p ± 1.96 * sqrt(p(1-p)/n) = 0.70 ± 1.96 * sqrt(0.7*0.3/150) ≈ 0.70 ± 0.073`。
    -   置信区间为 `(0.627, 0.773)`。
    -   **结论**: 这个置信区间**完全不包含** monolithic baseline 的成功率 `0.85`。因此，如果得到这个实验结果，我们**不能说"已恢复到 monolithic 水平"**。相反，我们可以有超过 95% 的信心说，dual-mode 的性能显著低于 monolithic 水平。

## Q7. prediction_market.rs f64 epsilon 风险评估

-   **`yes_price + no_price != 1.0` 风险**:
    -   `yes_price()`: `self.no_reserve / (self.yes_reserve + self.no_reserve)`
    -   `no_price()`: `self.yes_reserve / (self.yes_reserve + self.no_reserve)`
    -   数学上，二者之和为 `(no_reserve + yes_reserve) / (yes_reserve + no_reserve) = 1.0`。
    -   在浮点数运算中，`a/c + b/c` 可能不完全等于 `(a+b)/c`。然而，这里的计算是 `a/(a+b) + b/(a+b)`。只要 `a+b` 不为零（由 `lp_coins > 0.0` 保证），这个计算是非常稳定的。与 `1.0` 的偏差会远小于典型的 `f64` epsilon。**此路径风险极低**。

-   **`settle_portfolios` 累积误差**:
    -   该函数在 `bus.rs:407` 中对 `wallet.credit` 进行循环调用，这会导致在 `wallet.balances` 中反复进行 `f64` 加法。
    -   浮点数累加误差的增长大致与 `sqrt(N)` 成正比，其中 N 是加法次数。一个粗略的误差上限是 `N * ε * max_value`。一个更紧的界是 `sqrt(N) * ε * RMS_value`。
    -   假设 `ε ≈ 2.22e-16` (f64 machine epsilon)，最大单次 `payout` 为 1000.0 coins。
    -   我们需要找到 N，使得 `N * 2.22e-16 * 1000 > 1e-6`。
    -   `N > 1e-6 / (2.22e-13) ≈ 4.5 * 10^6`。
    -   这意味着需要进行**超过 450 万次**的结算贷记操作，才可能累积超过 `1e-6` (一个 micro-coin) 的误差。在系统的当前和预期规模下，这个风险是**可以忽略不计的**。

-   **建议 (量化到 fixed-point)**:
    -   **是否必要?**: 从纯粹的数学风险角度看，在当前规模下**不是绝对必要的**。`f64` 的精度足以应对预期的交易量。
    -   **是否推荐?**: **强烈推荐**。在任何处理价值（即使是模拟价值）的系统中，使用定点数（例如，将所有金额存储为 `u64` 类型的微币 `1 coin = 1,000,000 micro-coins`）是金融和共识系统的标准最佳实践。这可以完全消除浮点数带来的不确定性、舍入误差和平台依赖性，使系统行为完全确定，从而简化测试和审计。这是一个工程鲁棒性问题，而非迫在眉睫的数学危机。