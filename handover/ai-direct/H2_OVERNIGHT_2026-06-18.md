# H-HET-2 Step-6 校准 — 隔夜运行交接（2026-06-18 03:24 启动）

## 一句话
"卡死/全死"的根因全部查清，**不是 bug 级的代码缺陷**（除一个窄 bug，见下）。已启动
**35 个 named 非-det 定理 × 4 模型 = 140 cells** 的耐久隔夜 sweep。早上应有 Goldilocks 数据。

## 根因（完整链条，逐一实证）
1. **不是 hang/崩溃/并发/代理。** 代理健康（~5-7s/call，返回真证明）；carrier 正常跑，
   只是 **LLM 延迟主导**——每个不能秒解的定理要做几十次 ~9s 的串行调用 = 每 cell 几分钟到 ~30min。
   我之前在 25min 看到"0 cells"误判成卡死，其实在慢慢跑。
2. **唯一真正的"死" = JOBS=4 内存。** 每个 carrier 自带 ~6GB 常驻 Mathlib REPL；4 个 = 24GB+
   压垮 36GB 机器 → 被系统回收。**JOBS=2(~12GB) 安全。**
3. **窄 bug（已规避，待你定修法）:** `verify()`（src/judges/lean_judge.rs:245-249）跑
   `#print axioms <name>` 需要**命名定理**；`theorem_name_from_preamble` 对匿名 `example` 返回
   None → verify() 在编译前就 fail-closed → 该定理**永远 verified=0**（与能力无关）。
   bank 里 44 个定理中**只有 4 个是匿名 example**：`lm_B1, lm_median_probe, lm_probe1, lm_probeC3`。
   我调试时不巧一直用 lm_B1，所以前面全是假象。其余 40 个是 named，正常。
   - 验证：positive control 证明 verifier **本身是对的**——它接受正确证明（含 col-0）、正确
     拒绝错证明（rfl）。lm_smoke(named) 端到端 12s omega、verified=1、axioms=[propext]、manifest 干净。

## 这次启动的运行
- 脚本：`/tmp/overnight_sweep.sh`（自愈：caffeinate 防睡 + 代理挂了自动重启 + 逐 cell 断点续做 + 循环到 140 全完成）。
- 定理：35 个 named 非-det（**排除上面 4 个 example**）。模型：DSHOMO Q32HOMO GLMHOMO Q397HOMO。NA=4 NR=20 seed=42 JOBS=2。
- venv：`.venv-lean-verify`（稳定，REPL 已缓存，服务 ~4s 加载）。
- ETA：~12-16h。

## 4 个被排除定理的修法（待你/architect 定，我没擅自改共享 fixture/judge）
- **方案 A（低风险）**：把这 4 个 bank preamble 的 `example` 改成 `theorem <id>`（证明不变，
  positive-control 已证 named 版会过）。补回完整 39 定理池。
- **方案 B（更正确，Class 2-3 需 audit）**：修 `verify()` 让它支持匿名 example（合成一个名字给
  axiom 门）。这才是"verifier 不该对合法匿名 example fail-closed"的根治。
- 这是 2026-06-05 就存在的 pre-existing bug（gretjia），不是我引入的。

## 怎么看进度 / 结果
- 进度日志：`handover/evidence/h2_calibration_2026-06-17/overnight_runner.log`
- 每个 cell：`handover/evidence/h2_calibration_2026-06-17/<thm>__<ARM>__s42.json`（有 omega_reached 即完成）
- 跑完 sweep 脚本自带 Goldilocks 分类（per-theorem: 哪些模型 solve/fail；solve∧fail = GOLDILOCKS 目标）。
- 我会定期醒来检查+自愈，全完成后做 Goldilocks 分析 + 给你完整报告。

## 仍待你/architect 决策的（不阻塞今晚）
- 4 个 example 定理的修法（A vs B）。
- Goldilocks 目标池冻结需 architect sign-off（prereg gate #4）才能跑 PAID confirmatory。今晚只是 classify。

## 更新 05:43（debug→release 切换 + 实测，关键）
- **debug 太慢实锤**：原 debug 构建 **CPU-bound**（实测一个 carrier 82% CPU、~50s 计算/次提案），吞吐仅 **1.8 calls/min** → 全扫 ~50–100h **不现实**。
- **已切换到 release 构建**（同逻辑、仅编译优化，**prereg-safe**；release lm_smoke 已实证 omega 正确）：吞吐升到 **6.4 calls/min（3.6×）**，compute-fraction **82%→37%**。**修正 ETA ≈ 15–25h。** `calibration_sweep.sh` 的 BIN 已默认 release（env 可覆盖）。runner 05:21 在 release 上重启，断点续做（已完成 cell 保留）。
- **首个 solve**：`lm_comm_conj/DSHOMO omega=true`（321 tokens）→ Goldilocks 检测正常。
- **GLM-4.5-Air = 慢臂**：~0.9 calls/min + ~30% broken-pipe 错误 → GLM cells 每个 ~90min（能解的会早停）；**非卡死，只是慢**，占一个 JOBS 槽。属上游 provider flakiness，非我们的 bug。
- 进度（05:43）：4/140 cells。lm_c 全 4 臂 all-fail（15 行 incl-excl，确属 hard，非 Goldilocks）；lm_comm_conj/DSHOMO solve。
- 仍是 partial-by-morning：~15–25h 全扫，早上预计 ~25–45 cells，含若干完整 Goldilocks 行。

## 更新 06:19（GLM 慢臂 → 改为「快臂优先、GLM 末尾」两遍）
- 现象：GLM cells（~90min/个、~30% broken-pipe）会**占满两个 JOBS 槽**，把快臂（DS/Q32/Q397）饿死 → 吞吐掉到 4.1 calls/min。
- 改法（prereg-safe，仅**重排顺序**、所有 cell 仍全跑、JOBS 仍=2、RAM 安全）：runner 改成两遍——Pass1 跑 3 个快臂×35 定理（105 cells，无 GLM 阻塞，~5–8h），Pass2 才跑 GLM×35（慢尾，可能拖到次日）。已确认重启后在跑快臂（非 GLM）。
- 含义：早上你会先拿到**3/4 覆盖**（DS/Q32/Q397 的大部分），足以看出多数定理的 Goldilocks 倾向；GLM 那一列稍后补齐。

## 进展快照 07:55（23/140 cells，10.6 calls/min，健康）
7 个完整 3-快臂行 → **5 个 GOLDILOCKS**：
- all-fail(太难)：lm_c, lm_coeff_mul
- GOLDILOCKS：lm_comm_conj(DS-only T/F/F)、lm_commute_pow(T/F/T)、lm_cube4(T/F/T)、lm_deriv1(**Q397-only** F/F/T)、lm_deriv2(T/F/T)
- 每模型解出率：**DS 5/8、Q397 5/8、Q32 0/8**。
- **核心信号 = 互补覆盖**：lm_comm_conj 只有 DS 解出、lm_deriv1 只有 Q397 解出 → 没有单一模型通吃 → 正是 H-HET-2 论点（动态路由组合可超过任一单模型）。Q32 是稳定弱臂。
- 这是 partial 实时数据（fast arms 进行中，GLM 未跑）；完整 Goldilocks 池 + per-(model,theorem) 全表待 sweep 跑完。

## 进展快照 08:27（27/140 cells，10.1 calls/min，健康）
9 个完整 3-快臂行：
- all-fail(太难)：lm_c, lm_coeff_mul, lm_e
- **GOLDILOCKS (6)**：lm_comm_conj(**DS-only**), lm_commute_pow, lm_cube4, lm_deriv1(**Q397-only**), lm_deriv2, lm_deriv3
- per-model 解出：**DS 5/9、Q397 5/9、Q32 0/9**。
- **H-HET-2 关键量 union 覆盖**：DS∪Q397 = 6/9，单 DS=5/9，单 Q397=5/9 → **组合 > 任一单模型**（互补：lm_comm_conj 仅 DS、lm_deriv1 仅 Q397）。这正是动态路由价值的直接证据。
- Q32(Qwen3-32B) 至今 0 解出 = 这批定理难度对它偏高（稳定弱臂）。

## 进展快照 09:31（34/140 cells，~7/min，健康）
11 个完整 3-快臂行；**7 GOLDILOCKS**（+lm_fact）：lm_comm_conj, lm_commute_pow, lm_cube4, lm_deriv1, lm_deriv2, lm_deriv3, lm_fact。4 太难：lm_c, lm_coeff_mul, lm_e, lm_f。
- per-model：**DS 6/12、Q397 6/11、Q32 0/11**。
- **UNION DS∪Q397 = 8 > 单 6**（互补优势从 +1 扩大到 +2，随样本增长更稳）。
- Q32 既弱(0/11)又慢(~29min/cell、verbose 撞 900-token 上限) → Pass1 里它像 GLM 一样占槽拖速；未再 reorder（运行健康、避免过度折腾），让它跑完即可。

## 更新 09:50 — option A 已修复 + 实证 + 慢臂三遍重排
- **用户批准 option A**：bank 里 4 个匿名 `example` → `theorem <id>`（`tests/fixtures/lean_theorems_pool.jsonl`，备份 `/tmp/lean_theorems_pool.jsonl.bak`）。4 个 positive-control 全 exit 0（编译+可证）。
- **端到端实证修复**：fixed `lm_B1` 现在 **omega=True、verified=1**（DS 300tok / Q397 261tok）——verify() 的 name-gate 已过，真验证生效。
- **sweep 扩到完整 39 定理池 = 156 cells**（4 个修复定理排最前）。runner 现为 **3 遍**：Pass1 DS+Q397（核心互补对）→ Pass2 Q32（弱+慢）→ Pass3 GLM（flaky+慢）。原因：Q32 像 GLM 一样占满 JOBS 槽、速率塌到 0.7/min，故一并降级到后面。
- **option B 仍是更彻底的修法**（让 verify() 支持匿名 example，免再踩 name-gate），属 Class 2-3 需 audit；A 已解锁实验，B 留给 architect 定。
- 提醒：bank 是 tracked fixture，这次编辑应作为交付的一部分 commit/PR。

## 进展快照 13:49（84/156 cells，9.9/min，健康）
Pass1（DS+Q397）36/39 对完成，仅剩 lm_sum_cubes + 2 个未起。
- per-model：DS 21 solve/36 done、Q397 23 solve/37 done、Q32 0/11。
- **UNION DS∪Q397 = 29 > 单 DS 21 / 单 Q397 23**（互补优势升到 +6/+8，最强）。13 个 DS≠Q397 互补定理。
- 中途死过一次（~12:51 广播式进程清扫），4min 内 heal 续做，无数据损失。
- Pass1 即将完成；随后 Pass2(Q32)、Pass3(GLM) 慢尾。

## ★ Pass1 完成 14:16 — DS+Q397 全 39 定理覆盖（90/156，Pass2/Q32 已开始）
**分类（DS, Q397 over 39）：**
- BOTH solve（对路由无增益，太易）n=16
- **DS-only n=6**：lm_comm_conj, lm_finset_sup, lm_lt_min_max, lm_median_probe, lm_natdeg_pow, lm_prod
- **Q397-only n=8**：lm_deriv1, lm_fact, lm_ineq1, lm_nt_cop_cubic, lm_nt_gcd2, lm_probe1, lm_probeC3, lm_sum_cubes
- **互补总数 = 14**（恰好一个模型解出 → 路由在此优于任一单模型）
- NEITHER n=9：lm_c, lm_coeff_mul, lm_e, lm_f, lm_inf_sup_le, lm_lim1, lm_median, lm_natdeg_sum, lm_trace_prod
- **UNION DS∪Q397 = 30/39 | DS 22/39 | Q397 24/39 → oracle 路由上界比最佳单模型 +6。**
- **诚实框定**：这是 calibration（1 seed、单模型 homogeneous 臂 = per-(model,theorem) 覆盖）。union=oracle 上界，证明「互补覆盖存在」（路由有价值的*必要*前提）；真实动态路由的*实现*增益由 confirmatory（≥12 seeds + 真 router policy + paired stats）测，需 architect gate-#4 冻结池后跑。非 PROVEN。
- Q32（Pass2 进行中）：至今 1 solve（lm_B1）。GLM（Pass3）未起。

## ■ STOPPED 14:44 — 用户决定「核心结果已足够」
- Sweep 停止（90/156 cells 落盘保留；DS+Q397 全 39 完整 + Q32 12/39 部分 + GLM 未跑）。监控循环结束。
- **理由**：DS/Q397 核心结果已得（union 30/39 vs 24 best-single，14 互补 Goldilocks 候选）；Q32+GLM 慢臂尾巴 ~40h、主要补 floor 数据、且需持续 heal 反复进程死亡，性价比低。
- **交付结果（calibration，1 seed）**：Goldilocks 候选池 = 14 个 DS≠Q397 互补定理（6 DS-only：lm_comm_conj, lm_finset_sup, lm_lt_min_max, lm_median_probe, lm_natdeg_pow, lm_prod；8 Q397-only：lm_deriv1, lm_fact, lm_ineq1, lm_nt_cop_cubic, lm_nt_gcd2, lm_probe1, lm_probeC3, lm_sum_cubes）。
- **未提交的交付物（待 commit/PR）**：
  - `src/bin/lean_market_agent.rs` + `tests/carrier_worktx_terminal_manifest.rs` = carrier WorkTx receipt + terminal-manifest 修复（已实现+测+外审，PR 未开 = 原任务 #5）。
  - `tests/fixtures/lean_theorems_pool.jsonl` = option A 4 个 example→theorem 重命名（备份 /tmp/lean_theorems_pool.jsonl.bak）。
  - `scripts/calibration_sweep.sh`（新）+ `scripts/autoloop/estimate_calibration_cost.sh` = 校准 harness。
  - `handover/evidence/h2_calibration_2026-06-17/` = 90-cell 校准数据。
- **待 architect**：option B（verify() 支持匿名 example，根治 name-gate）；gate-#4 sign-off 冻结 Goldilocks 池 → 跑 confirmatory（真 router policy + ≥12 seeds + paired stats）测实现增益。
