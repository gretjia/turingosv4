# Architect Directive — PPUT-Driven FULL PASS Upgrade

date_received: 2026-04-25 (received in user message after Paper 1 v2.1.1 dual-audit PASS/PASS)
date_archived: 2026-04-26
verdict_from_architect: **FULL PASS** (upgraded to PPUT-driven version)
authorization_status: launched as `PPUT_CCL_FULL_PASS` arc per user directive 2026-04-25; Paper 1 arXiv submission deferred
governing_pre_reg: `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`

## why archived

Per CLAUDE.md "Audit Standard" + C-069 Constitutional Alignment Audit Protocol, architect directives that change the optimization signal (here: `solve rate / WBCG / VTR` → `Held-out Verified PPUT / WBCG_PPUT`) MUST be archived in cleartext for downstream auditors (Codex, Gemini) to read independently when assessing the pre-reg.

This file is the source of truth for what the architect said. The pre-reg (`PREREG_PPUT_CCL_2026-04-26.md`) is a compressed, formalized restatement; if the two ever conflict, this archive is canonical and the pre-reg must be amended via formal addendum + dual external audit.

## directive (verbatim, EN translation where needed)

The user's message of 2026-04-25 carried the architect's reply, in mixed Chinese / English. Key sections preserved verbatim below.

### 1. Verdict

> 给 FULL PASS，并升级为 PPUT 驱动版 FULL PASS。

> 你的补充非常关键。PPUT 应该成为整个 TuringOS 实验体系的北极星指标，因为它同时惩罚三种系统失败：
> 1. 无效搜索：消耗大量 token 但没有 golden path；
> 2. 慢速搜索：能成功但耗时太长；
> 3. 伪进展：产生很多中间状态、文档、工具，但没有通向真正有效结果的 golden path。

### 2. Long-term goal restated

> TuringOS 的长期目标不是单纯提高 solve rate，而是在宪法约束下最大化 Verified PPUT：单位 token、单位时间内产生的可验证 golden-path progress。

### 3. PPUT formal definition

```
Progress_i = 1   iff verified golden path exists for task i
Progress_i = 0   otherwise
PPUT_i = Progress_i / (C_i × T_i)
```

Verified PPUT = ground-truth gated:
```
VPPUT_i = 1[GroundTruth(G_i) = 1] / (C_i × T_i)
```

C_i = ALL token cost across all agents, all branches, all failed proposals, all tool stdout context.
T_i = wall-clock from first-read to final-accept.

### 4. Mapping to constitution

| 宪法条款 | PPUT 对应含义 |
|---|---|
| Art. I.1 | 只有谓词通过才有 progress |
| Art. I.2 | PPUT 是统计信号 |
| Art. II.2 | PPUT 可以作为广播价格信号 |
| Art. III.4 | runtime 自评 PPUT 不能暴露为可操纵目标 |
| Art. IV   | golden path 必须写入版本化 Q^world |
| Art. V    | 新工具是否有价值，看它是否提升 held-out PPUT |

### 5. WBCG redefinition

OLD: ΔVTR_heldout > 0
NEW:
```
WBCG = sum over Δ of 1[ used(Δ) ≥ N
                       ∧ ΔPPUT_heldout > 0
                       ∧ ΔFAR ≤ 0
                       ∧ RR = 0
                       ∧ Rollbackable(Δ) ]
```

### 6. North Star

```
H-VPPUT = (sum_i Progress_i^{heldout}) / (C_heldout × T_heldout)
```

> Held-out Verified Progress Per Unit Token-Time
> 中文：未见任务上的可验证单位 token-time 进展率。
> 这是你的最高指标。

### 7. Dashboard restructure (architect § 4)

| 层级 | 指标 | 作用 |
|---|---|---|
| 北极星 | Held-out Verified PPUT | 衡量系统是否真的更快、更省、更有效地产生 golden path |
| 长期自举 | WBCG_PPUT | 衡量新白盒资产是否提升 held-out PPUT |
| 安全 | FAR | 防止错误状态被当成 progress |
| 回归 | RR | 防止 meta-change 伤害旧能力 |
| 泛化 | Generalization Gap | 防止 adaptation set 过拟合 |
| 恢复 | ERR | 衡量 rollback 后是否能重新找到 golden path |
| 信息卫生 | CPR | 衡量 rejected trace 是否污染上下文 |
| 多样性 | IAC | 衡量 Agent 是否相关性坍塌 |
| 自治 | AH / HIF | 衡量系统能否少依赖人类 sudo |

VTR demoted to auxiliary (architect § 4 final paragraph: "VTR 可以被 micro-step gaming；PPUT 更难被游戏，因为没有 golden path 就是 0").

### 8. CCL-1 hypothesis (architect § 7)

> TuringOS can convert failure logs into reusable white-box assets that increase held-out Verified PPUT without increasing FAR, RR, or context pollution.

> 中文：TuringOS 能否把失败日志编译成新的白盒资产，并在 held-out 任务上提升可验证 PPUT，同时不增加误判率、退化率和上下文污染？

### 9. Three ablation modes restated in PPUT (architect § 8)

- Soft Law: runtime accept fast → Lean post-hoc reject → Progress = 0 → VPPUT drops
- Panopticon: context length up + IAC up + CPR up + tokens up → PPUT down
- Amnesia: rollback fail → time inflation + token inflation → PPUT down

### 10. Anti-Goodhart guardrails (architect § 13)

10 conformance tests (translated/expanded into pre-reg § 3):

```
test_all_model_tokens_counted
test_tool_stdout_hash_logged
test_no_hidden_unmetered_generation
test_no_problem_id_hardcode
test_no_metric_file_access_by_agents
test_golden_path_requires_ground_truth_acceptance
test_failed_branches_count_toward_total_cost
test_wall_clock_measured_from_first_read_to_final_accept
test_heldout_ids_inaccessible
```

(Pre-reg adds an 11th: `test_no_pput_in_agent_prompt` — explicit gate on prompt builders.)

### 11. JSONL schema additions (architect § 14)

Per-proposal:
```
run_id, problem_id, split (adaptation|meta_validation|heldout),
agent_id, role, context_hash, branch_id, proposal_hash,
predicate_result, ground_truth_result, lean_error_category, raw_error_hash,
accepted, rollback_to,
prompt_tokens, completion_tokens, tool_tokens, total_tokens,
wall_time_ms, start_time, end_time,
ast_depth, peer_agents_in_branch, tool_stdout_hash,
is_on_golden_path, golden_path_id
```

Per-run aggregate:
```
run_id, problem_id, solved, verified,
golden_path_token_count, total_run_token_count, total_wall_time_ms,
progress (0|1), pput, failed_branch_count, rollback_count,
far, err, iac, cpr
```

### 12. PPUT scaling for dashboard

```
PPUT-M = 10^6 × VPPUT     (per million token-second)
PPUT-B = 10^9 × VPPUT     (per billion token-second)
```

Raw VPPUT retained in jsonl for paper / audit. PPUT-M is the dashboard default.

### 13. 30-day phased plan (architect § 16)

- Week 1 (Phase B): Kernel instrumentation + PPUT accounting
- Week 2 (Phase C): Phase 1 ablation smoke tests with PPUT
- Week 3 (Phase D): CCL Shadow Mode with PPUT attribution
- Week 4 (Phase E): CCL Controlled Activation with held-out PPUT

(Pre-reg prepends Phase A pre-flight: this archive + PREREG draft + split + dual external audit, days 1-3.)

### 14. FINAL PASS gates (architect § 17, copied verbatim)

| Gate | 条件 |
|---|---|
| A | AuditorAI 无最终裁决权 |
| B | ArchitectAI 只能写 user-space |
| C | WBCG 必须基于 held-out ΔPPUT |
| D | L_t raw logs 不进入普通 Agent context |
| E | 三 split 严格执行 |
| F | 所有失败分支计入 token/time cost |
| G | PPUT 只能由 verified golden path 产生 |
| H | PPUT metric 对普通 Agent 屏蔽，防 Goodhart |

(Pre-reg restates Gate H as the WBCG_PPUT > 0 success criterion + folds the architect's "PPUT-metric-shielding" into the anti-Goodhart conformance battery, since shielding is enforcement-side and success is measurement-side. Both must hold for FULL PASS.)

### 15. Final thesis (architect § 18, frozen verbatim in pre-reg § thesis)

EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.

ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。

## reception decisions (Claude, 2026-04-26)

1. Skip Paper 1 v2.1.1 → arXiv submission as standalone publication this cycle. User authorized 2026-04-25.
2. Treat this directive as the canonical research charter for the PPUT-CCL arc; pre-reg is its formalization.
3. The architect's directive does NOT modify constitution.md or the existing flowcharts; it specifies a measurement regime + research arc. Per C-069, no Phase Z' triggered.
4. The directive's "Boot must initialize PPUT_accounting_0" is treated as a Phase B7 work item, not a constitutional change.
5. **Compute backbone pinned to `deepseek-v4-flash` thinking-off** per user directive 2026-04-25 (delivered after this directive). 1M context, deprecates legacy `deepseek-chat` alias. Pin model id explicitly to avoid alias-deprecation drift mode (C-068 lesson).
6. **Heterogeneous-LLM timing decided 2026-04-26**: enter at Phase D meta-loop (ArchitectAI = v4-flash thinking-on, AuditorAI = Gemini 2.5 Pro) — constitutionally motivated by C-010 Generator≠Evaluator. Phases B+C stay single-model to keep the ablation axes clean. Pre-reg § 12.2 has the full table.
7. **Gemini DeepThink dual-chamber FULL PASS absorbed 2026-04-26** (per C-023): independent reviewer issued PPUT-DRIVEN FULL PASS approving 5 ontological patches (ArtifactState 4-state machine, Trust Root immutability, doc/artifact content meta-predicates, ArchitectAI sole-L_t-reader cognitive isolation, single-binary `--mode` flag). All 5 absorbed into PREREG `## changelog`. Verbatim archive: `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md`. Two FULL PASS authorizations now load-bearing on this arc (measure-theoretic v1 + ontological v2); PREREG fuses both.
8. **Phase D human-as-live-meta-predicate confirmed 2026-04-26** (Gemini DeepThink mandated option (a)): user `gretjia` reviews each `Accepted` candidate Δ, issues keep/drop/revise verdict before promotion to `Quarantined`. Defense-in-depth against Claude-only homogeneous LLM stack covert promotion-loop (C-010 protection). User keystroke logs become Trust Data for Phase E auto-meta-predicate training. Estimated load: ≤ 10 min/day.

## downstream artifacts created from this directive

- `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` (formalized pre-reg)
- (planned) `handover/preregistration/scripts/split_pput_ccl.py`
- (planned) `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json`
- (planned) `handover/audits/run_codex_pput_ccl_audit.py` + `run_gemini_pput_ccl_audit.py`
