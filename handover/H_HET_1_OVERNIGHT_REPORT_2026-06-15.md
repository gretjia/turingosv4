# H-HET-1 自主研发夜间报告（2026-06-15）

> 架构师 mandate：auto-research（research→思辨→计划→执行→recursive audit），尽情试参数直到符合预期，**守宪法为底线**。本报告是夜间自主 loop 的诚实汇报。所有结论 DIRECTIONAL（§17 无 PROVEN）。

---

## 0. 一句话

在 det-family 难度带、等 carrier 预算下，**异质性对最强单模型(Q397)零增益**（对抗审计 5-agent 全 NO-VIOLATION 确认）；但底下藏着真信号——**互补覆盖**（不同厂解不同定理），而现 round-robin 载体**无法利用**它。下一个真正测试架构论点的杠杆 = **动态 model-budget 市场**；其所需的 per-node model 字段恰好 = Art-0.2 §8 改动（合规与科学同一件事）。

## 1. 守住的宪法底线（全程）

- §8-gated `ProposalTelemetry` schema **未碰**（blanket mandate ≠ §8）。
- 全量付费实验 **未跑**（sign-off-gated）；只跑了 bounded pilot（<$5）。
- 所有 gate 绿（167 total / 仅 3 已知 pre-existing 红，零新红）。
- 每个付费 run 前 prereg-frozen（Gate H 反 p-hacking）。
- 每 cell replay-recompute 门（economic_state from L4）；45/45 + smoke 全绿。
- 所有 verified 证明 axiom ⊆ {Classical.choice,Quot.sound,propext}，0 native_decide。
- 结论 DIRECTIONAL（K=3 小、sidecar 归属、§17-G4 未满足）。

## 2. 夜间做了什么（auto-research 弧）

1. **清账**：清除递归检查的 VETO（Gate-D 入 manifest + matrix 行；连带修 matrix_drift）；修 Eng-1（gate-runner POSIX 可移植）、Eng-2（`action_source` tape 字段标记 fail-open solve）。
2. **freeze checkpoint**：carrier+gates file-by-file carve → commit `f73163f4`（branch `claude/het-carrier-freeze`，**未 push**，PR/merge 是你的 call）；在 166 modified+855 untracked 的他-session 脏树里只装我的 26 文件。
3. **Goldilocks 带**（零新花费，分析既有 paid 证据）：probe 层 H-HET-1 现象在 det 带 SOUND 现身。
4. **carrier 验证**（首次端到端真跑）：mechanism smoke + 45-cell pilot，全 replay-clean。
5. **recursive 对抗审计**（wf_fd1ba89f，5 agents，全 pin repo）：headline SURVIVES。
6. **clean serial PPUT benchmark**（JOBS=1，架构师要的 canonical 经济度量）。

## 3. 核心实验结果（45-cell pilot，prereg sha 621de565，3 臂×det 带×K3，等预算 NA4·NR3）

| theorem | HET | DSHOMO | Q397HOMO |
|---|---|---|---|
| lm_det_mul | 3/3 | 3/3 | 3/3 |
| lm_det_2x2 | 3/3(Q397) | 0/3 | 3/3 |
| lm_det_zero | 1/3(Q397) | 1/3(DS) | 0/3 |
| lm_det_3x3 | 0/3 | 1/3(DS) | 0/3 |
| lm_geom_eval | 3/3 | 2/3 | 3/3 |
| **SOLVED** | **10** | **7** | **9** |

**诚实读数（对抗审计确认）：**

1. **预算混淆**：probe 层 DS 0/3 的 lm_det_mul，在等 carrier 预算(12 提案)下 DSHOMO **3/3**。所谓 "Goldilocks" 多是**低预算伪影**，非异质效应。
2. **异质性 ≈ 最强单模型**：HET 10 vs Q397HOMO 9——Wilson 95% CI 全重叠（HET[0.42,0.85]、Q397HOMO[0.36,0.80]、DSHOMO[0.25,0.70]），在 K=3 噪声内不可区分。+1 边际是**单 cell 的 Q397-slot 抽样方差**，非去相关。
3. **半数 roster 死重**：HET 的 10 个 omega = Q397 8 + Q32 2；**DS 0/28、GLM 0/19**。让 roster 真正"异质"的两个厂贡献为零。"异质"≈"Q397+噪声"。
4. **但有真互补覆盖（唯一正面信号，跨臂）**：DSHOMO 独解 {lm_det_zero, lm_det_3x3}（Q397HOMO 各 0/3）；Q397HOMO 独解 {lm_det_2x2}（DSHOMO 0/3）。**无单模型全覆盖**——可 round-robin 载体甚至漏了 DS 单独能解的 det_3x3（稀释饿死 DS 的 shots）。

## 4. PPUT 经济度量（架构师指定的 token-economics 关键指标）

PPUT = golden_path_tokens / wall_clock_s（到 OMEGA 的获胜路径 token 吞吐；未解=0）。§17 报告标准 = ΣPPUT + Mean-PPUT(solved) + Wilson CI。

**pilot（JOBS=4 并发，wall_clock 受污染——故 serial 重测）：**

| arm | solve(Wilson95) | ΣPPUT | Mean-PPUT(solved) | gp_tokens(solved) | tot_tok/solve |
|---|---|---|---|---|---|
| HET | 10/15 [.42,.85] | 78.0 | 7.80 | 508 | 15298 |
| DSHOMO | 7/15 [.25,.70] | 63.6 | 9.09 | 415 | — |
| Q397HOMO | 9/15 [.36,.80] | **164.9** | **18.32** | **324** | 11039 |

**Q397HOMO 经济压制**：ΣPPUT ~2×、Mean-PPUT ~2.3×、gp_tokens 最省（324 vs HET 508，**并发无关也压制**）。即异质性不止"不更好"，在 token 经济上**被最强单模型支配**——HET 获胜路径更长 + 烧预算养死重模型。

**clean serial PPUT（JOBS=1，无并发污染）：** `<待 serial run 回填>`
（gp_tokens 并发无关已定向；serial 给 canonical 时间分量。预期 Q397HOMO 优势更强，因其 wall 最短。）

## 5. 结论 + 下一个杠杆

**对架构师论点的诚实裁决：** "异质廉价提议者 + 市场 → 涌现超越单模型" 在本 pilot **未被证实**。原因不是论点错，而是**现载体只路由 node、不路由 model-budget**：固定 round-robin 把预算均摊到 4 厂，故无法把"互补覆盖"（唯一真信号）变现，反而被最强单模型在经济上支配。

**下一个杠杆（审计与我独立收敛）= 动态 model-budget 市场**：
- 用 priced/bandit 载体替换固定 round-robin——按"哪个模型在 verify / 价格信号 / 历史失败(Art.II Librarian)"把提案预算**重分配**给正在赢的模型（Q397 攻 det_2x2、DS 攻 {det_zero,det_3x3}）。
- prereg 成功判据 = **het-dynamic 在等或更低 token 预算下，UNION 覆盖胜过最强单模型**（解出任何单模型预算内解不出的定理）。
- 功效：K 从 3 提到 **≥12 seeds + within-seed Wilcoxon 配对**；Goldilocks 池预选 {某模型 0/3 且另一模型 ≥1/3}（让预算 BIND、去相关能表达）。
- **加 per-node {model,vendor} 字段**退役 round-robin sidecar 推断 = **正是 Art-0.2 §8 的 `model_id` 改动**（合规与实验仪器同一件事，见 ART_0_2_FULL_CLOSE_DESIGN）。

**这是关键洞见**：动态市场不是"再调参数"，是兑现架构核心论点（哈耶克式资本流向赢家）的必要机制——现载体根本没实现"市场分配最重要的资源(模型选择)"。

## 6. 待你裁决（晨起）

1. **§8 裁决**：批准 `ProposalTelemetry` binding schema 加性扩展（v1→v2 + legacy decode + `model_id: Option<String>`）。**双重价值**：闭合 Art-0.2 tape-canonical + 给下个实验干净归属。设计见 `handover/audits/ART_0_2_FULL_CLOSE_DESIGN_2026-06-15.md`。
2. **动态 model-budget 市场设计**：是否批准我起草 TB charter（这是 carrier 核心机制改 = Class 2-3，应过你的设计评审，故未自主实现）。
3. **freeze branch `claude/het-carrier-freeze`（`f73163f4`）push/merge**：你的 call。
4. （平行）全量 H-HET-1 实验仍 BLOCKED 至：§8 闭合 + 动态市场建成 + K≥12 prereg + sign-off。

## 7. 证据指针

- pilot：`handover/evidence/het_carrier_pilot_2026-06-15/`（45 cells + replay）+ prereg `handover/preregistration/H_HET_1_CARRIER_PILOT_PREREG_2026-06-15.md`(.sha256)
- serial PPUT：`handover/evidence/het_carrier_pput_serial_2026-06-15/`
- smoke：`handover/evidence/het_carrier_smoke_2026-06-15/`
- 审计：workflow wf_fd1ba89f（5 agents NO-VIOLATION）
- 分析脚本：`scripts/analyze_het_carrier_pilot.py`、`scripts/het_carrier_pilot.sh`
- Goldilocks 源数据分析：calib sweep + K=3 probe pilot（2026-06-14，复用既有）
