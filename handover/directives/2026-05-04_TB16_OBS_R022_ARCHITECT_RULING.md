# Architect ruling — TB-16 OBS_R022 (Global LATEST_MARKOV pointer = Art. 0.2 平行账本)

**Date**: 2026-05-04
**Filed by**: user (architect) into Claude session immediately after `/clear`
**Responds to**: `handover/architect-insights/REQUEST_OBS_R022_GLOBAL_LATEST_MARKOV_2026-05-04.md` (ratification request committed in `4750778`)
**Authoritative OBS**: `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md`
**Status**: RATIFIED — pending Claude execution authorization
**Compression**: lossless (full ruling text archived verbatim in §B); structured index in §A

---

## §A — Structured index (annotation layer)

### A.1 Verdict at a glance

| Question | Architect ruling |
|---|---|
| Q1: Is `LATEST_MARKOV_CAPSULE.txt` parallel ledger or convenience pointer? | **PARALLEL LEDGER** — Art. 0.2 violation; cannot remain canonical |
| Q2.a: Is Markov capsule a FC1/FC2/FC3 flowchart node? | **NO** — derived view / evidence compression |
| Q2.b: Is `previous_capsule_cid: None` correct genesis on fresh isolated chain? | **YES** — constitutional genesis, NOT a bypass |
| Q2.c: Is `audit_tape --markov-pointer <absent>` the genesis API? | **YES** — but rename/reshape CLI to be more explicit (e.g. `--no-inherited-markov` or `--prior-chain-runtime-repo <path>`) |
| Q3: Multi-task continuation rule (a / b / c)? | **Q3.a** — same `runtime_repo` + same CAS; `previous_capsule_cid` inherited in-tape (Art. 0.4 path B) |
| Q4: Phase Z′ rerun required? | **NO** — no FC modification; Art. 0.2 derived-view enforcement only |

### A.2 Adopted remedy

- **Option α (immediate)**: delete + de-canonicalize `LATEST_MARKOV_CAPSULE.txt`.
- **Option β (long-term)**: Art. 0.4 path B chain continuation — multi-task SINGLE-CHAIN runs anchor Markov inheritance in tape itself.
- **Option γ rejected**: explicitly — "用第二个平行账本监督第一个平行账本" is itself a code smell.

### A.3 Concrete α work-items (architect-mandated, verbatim)

1. Delete `handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt`.
2. `generate_markov_capsule` no longer writes `LATEST_MARKOV_CAPSULE.txt` (per-run only).
3. `handover/markov_capsules/MARKOV_TB-*.json` → marked historical artifact only.
4. `audit_tape` / `audit_tape_tamper`: `--markov-pointer` no longer required.
5. New CLI flag `--prior-chain-runtime-repo <path>` for explicit inheritance.
6. Fresh isolated chain default: no inherited Markov (`markov_capsule = None`, Layer G Skipped).
7. New 守恒 tests:
   - `markov_pointer_no_global_parallel_ledger`
   - `audit_tape_genesis_without_markov_pointer`
   - `audit_tape_blocks_unresolvable_present_markov_pointer`
   - `generate_markov_capsule_does_not_write_global_latest`
   - `markov_capsule_historical_artifact_not_reference_input`
   - The strongest assertion: `assert!(!Path::new("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt").exists())`

### A.4 New ship gates added

- **SG-16.7**: No global Markov pointer canonical input.
- **SG-16.8**: Fresh isolated chain has `markov_capsule=None` and Layer G skipped.
- **SG-16.9**: Present-but-unresolvable Markov pointer BLOCKS.
- **SG-16.10**: Multi-task continuation uses same `runtime_repo` + same CAS, OR explicit `--prior-chain-runtime-repo`.

TB-16 cannot ship until the pointer issue is closed.

### A.5 TB-17 amendments

- New required artifact: `MARKOV_INHERITANCE_POLICY.md`.
- New ship gates: SG-17.9 (Markov inheritance policy documented + tested) + SG-17.10 (no global filesystem pointer source-of-truth remains).
- New preconditions PRE-17.1 … PRE-17.4 (all pointer issues closed before TB-17 opens).

### A.6 Subsequent execution order

```
TB-16.x.fix    — execute Option α (delete/de-canonicalize)
TB-16.x.2.2    — ChallengeResolve / remaining arena pieces
TB-16.x.2.4    — multi-WorkTx attempt + Boltzmann runtime  ← begin β
TB-16.x.2.6    — combined arena run, single continuing chain ← β fully realized
TB-17          — Real-World Readiness Gate (preconditions + new SGs)
TB-18          — Low-Risk Real-World Pilot Design
TB-19          — Pilot Sandbox
TB-20          — Limited Real-World Beta
```

### A.7 Layer 1 invariant impact assessment (per `/architect-ingest` step 2)

| Invariant | Impact |
|---|---|
| `kernel.rs` 零领域知识 | None (Markov capsule, audit_tape, generate_markov_capsule are not kernel) |
| Append-Only DAG | **POSITIVE** — α REMOVES a parallel ledger and restores tape canonicality |
| Economic conservation | None |

**Verdict**: no Layer 1 violation; ruling is constitutionally restorative.

### A.8 Memory updates required after execution authorization

- Update `feedback_no_workarounds_strict_constitution.md` with TB-16.x.2.1 → architect-ratified outcome.
- Add new feedback memory: "Markov inheritance must be tape-derived, not global filesystem pointer" (Art. 0.2 derived-view rule, with α/β as concrete realizations).
- Update `project_tb_15_shipped.md` and any TB-16 project memory with: SG-16.7..16.10 ship gates added.
- Update TB-17 memory entry (when written) with PRE-17.1..PRE-17.4 + SG-17.9..17.10 + `MARKOV_INHERITANCE_POLICY.md`.

### A.9 Cross-references

- Ratification request: `handover/architect-insights/REQUEST_OBS_R022_GLOBAL_LATEST_MARKOV_2026-05-04.md`
- OBS file: `handover/alignment/OBS_R022_GLOBAL_LATEST_MARKOV_PARALLEL_LEDGER_2026-05-04.md`
- TB-16 umbrella charter: `handover/tracer_bullets/TB-16.x.2_charter_2026-05-04.md`
- TB-16.x.1 fail-closed fix (already shipped): commit `3735484`
- TB-16.x.2.1 strict-alignment patch (already shipped): commit `fab2977`
- TB-16.x.2.1 ratification request commit: `4750778`
- Constitution Art. 0.2 (Tape Canonical): `constitution.md` lines 52–95
- Constitution Art. 0.4 (Q_t version-controlled): `constitution.md` lines 114–152

---

## §B — Lossless verbatim ruling (architect's full text)

> 我的总裁决：
>
> > **TB-16 不能在现有 `LATEST_MARKOV_CAPSULE.txt` 作为全局 canonical pointer 的状态下 ship。**
> > 必须先执行 **Option α：Delete + de-canonicalize**，把全局 pointer 从 canonical 流程中删除；同时把 **Option β：Art. 0.4 path B chain continuation** 作为后续长期正确架构。
> > 也就是说：**α 立即修宪法违规，β 作为后续多任务单链继承的目标形态。**

### §B.1 — TB-16 当前进展审计

#### §B.1.1 已完成或接近完成的部分

按照你给出的上下文，TB-16 的主目标是 Controlled Market Smoke Arena，也就是在受控沙盒中跑通：

```
compute
position
CompleteSet
PriceIndex
Boltzmann mask
Autopsy
Markov capsule
dashboard / audit_tape
```

这与我们之前 TB-16 的要求一致：受控市场演习，不接真实世界，不接公链，不使用真实资金，所有活动必须可从 ChainTape + CAS replay。此前路线里也明确要求 TB-16 证明至少 3 个 Agent、FirstLong、ShortPosition、price update、Boltzmann mask、AutopsyCapsule 等路径，并禁止 public chain、real-money market、external domain、unbounded leverage 等。

但 TB-16 暴露出的最重要问题不是 market arena 本身，而是 Markov capsule inheritance 的事实层问题。

#### §B.1.2 当前 blocking 问题：`LATEST_MARKOV_CAPSULE.txt`

材料里写得非常清楚：

```
handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
```

是一个单行 hex pointer，用于 `audit_tape --markov-pointer` 解析本次 audit 应该验证哪一个 Markov capsule；它当前指向的 capsule 字节只存在于某一次 TB-16 R3 Round 2 `P8_completeset_b` 的 per-problem CAS 中，不在 global/project-root CAS，也不在其他 per-problem CAS 中。

这会造成：

```
fresh isolated runtime_repo + fresh cas
  -> 读到全局 LATEST_MARKOV_CAPSULE.txt
  -> 指向另一个 per-problem CAS 的 capsule
  -> 当前 CAS 无法 resolve
  -> 若 fail-open，会错误地 Skipped
  -> 若 fail-closed，会 BLOCK
```

TB-16.x.1 把 `.ok()` collapse 改成 fail-closed BLOCK 是正确的，因为它让隐藏问题暴露出来；材料也明确说 pre-fix 的 `.ok()` collapse 曾经把 pointer-CAS mismatch 静默掩盖成 `Skipped`，导致 CR-15.6 "Markov default prevents context poisoning" 被绕过。

所以我对 TB-16 的判断是：

```
Controlled Market Arena 功能可以继续；
但 TB-16 不能在 global Markov pointer 仍被当作 canonical reference 的情况下 ship。
```

### §B.2 — 回答当前必须裁定的问题

#### §B.2.Q1 `LATEST_MARKOV_CAPSULE.txt` 是便利指针，还是平行账本？

**裁决：它当前是平行账本，不能作为 canonical 输入保留。**

原因：

```
1. 它是 filesystem-side global。
2. 它不属于任何单一 tape。
3. 它携带跨 chain 状态。
4. 它指向只存在于某个 per-problem CAS 的 cid。
5. 它没有 derive_from_tape 守恒测试。
6. 它是 last-writer-wins lifecycle。
```

这正是材料中 Art. 0.2 平行账本的定义和判定。

**采用方案：Option α immediate, Option β long-term。**

也就是：

```
现在：
  删除 / de-canonicalize LATEST_MARKOV_CAPSULE.txt。

长期：
  用 Art. 0.4 path B 的 chain continuation，
  让前一个 capsule 在同一个 runtime_repo + CAS 中被后续任务自然继承。
```

我不接受 Option γ，因为它本质上是"用第二个平行账本监督第一个平行账本"，材料本身也把它描述为最弱方案、code smell。

#### §B.2.Q2.a Markov capsule 是不是 FC1 / FC2 / FC3 的 flowchart node？

**裁决：不是。**

Markov capsule 是派生视图 / evidence compression，不是三张 flowchart 的原生节点。材料明确说明，Markov capsule 由 TB-15 引入，其注释称它是 evidence compression，不是 hidden source of truth；每个字段应能从 chain + CAS 派生，因此它落在 Art. 0.2 的"派生视图 + 守恒义务"范畴，而不是 flowchart node。

所以这不需要修改三张 flowchart，也不需要 Phase Z′ 的 flowchart 级重修。

#### §B.2.Q2.b fresh isolated chain 中 `previous_capsule_cid: None` 是否是正确 genesis？

**裁决：是。**

如果新的 `runtime_repo` 和新的 `cas` 没有继承前一条 tape，那么：

```
previous_capsule_cid = None
markov_capsule = None
Layer G assertions = Skipped
```

这是 constitutional genesis，不是绕过。材料明确说，`previous_capsule_cid: Option<Cid>` 的默认值是 `None`，fresh isolated chain 天然没有 prior capsule；因此 `Pointer-absent -> markov_capsule = None` 是 genesis 分支。

但要强调：

```
pointer absent -> None
```

是正确的；

```
pointer present but CAS cannot resolve -> silent None
```

是错误的，必须 fail-closed。

#### §B.2.Q2.c `audit_tape --markov-pointer <absent path>` 是否是 genesis API？

**裁决：是，但要改名/改接口以降低误解。**

材料中的代码逻辑说明：

```rust
if inputs.markov_pointer.exists() {
    Some(read_markov_capsule(...)?)
} else {
    None
}
```

即 pointer absent 意味着 `markov_capsule = None`。

我建议后续把 CLI 语义改得更显式：

```
--no-inherited-markov
```

或：

```
--prior-chain-runtime-repo <path>
```

不要再让用户通过"传一个不存在的 pointer path"来表达 genesis。这个虽然合宪，但不够清晰。

#### §B.2.Q3 未来多任务 continuation 的 Markov inheritance 规则是什么？

**裁决：选 Q3.a。**

也就是：

```
多个 task 进入同一个 continuing runtime_repo + same CAS；
每个新 task 的 accepted txs append 到同一条 ChainTape；
previous_capsule_cid 从同一个 CAS / tape 中读取；
Markov inheritance 结构性编码在 tape_t 中。
```

这是长期正确方向，也就是材料里的 Option β。它说在 Art. 0.4 path B 下，chain continuation 意味着每个新任务的 runtime_repo 扩展前一个任务的 git history，不是 separate repos；此时前一个 task 的 Markov capsule 在同一个 CAS 中，不需要全局 pointer。

我不建议 Q3.b 作为主路径。
`import prior CAS bytes into next per-problem CAS` 可以作为离线迁移工具，但不是长期 canonical 模型。

#### §B.2.Q4 是否需要 Phase Z′ rerun / 修改 flowchart？

**裁决：不需要。**

原因：

```
Markov capsule 不是 flowchart node；
本次问题是 Art. 0.2 派生视图 / 平行账本问题；
修复方式是删除 global pointer 或把继承锚定到 tape continuation；
不修改 FC1/FC2/FC3。
```

因此不触发 Phase Z′ 6-stage rerun。

### §B.3 — 立即执行的 TB-16 修复方案

#### §B.3.1 采用 Option α immediate

AI coder 应立即执行：

```
1. 删除 handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt。
2. generate_markov_capsule 不再写 LATEST_MARKOV_CAPSULE.txt。
3. handover/markov_capsules/MARKOV_TB-*.json 标记为 historical artifact only。
4. audit_tape / audit_tape_tamper 不再要求 --markov-pointer。
5. 新增 --prior-chain-runtime-repo <path>，用于显式继承 prior chain。
6. fresh chain 默认 no inherited Markov。
7. 加守恒测试：全局 latest pointer 不存在，audit_tape 不依赖它。
```

材料中的 Option α 已列出这些具体步骤，并且明确它是最简单的合宪路径。

#### §B.3.2 `handover/markov_capsules/` 目录如何处理？

我的裁决：

```
保留目录可以，但里面的文件不能作为 canonical input。
```

更精确：

```
handover/markov_capsules/MARKOV_TB-*.json
  -> historical artifact / human convenience only

runtime audit / replay
  -> must use in-tape capsule or explicit prior-chain-runtime-repo
```

如果 AI coder 能快速迁移，我更建议：

```
per-run Markov JSON 放在 handover/evidence/tb_*/markov/
```

因为 capsule 是 run evidence 的派生物，而不是全局项目状态。

#### §B.3.3 守恒测试必须新增

新增测试建议：

```
markov_pointer_no_global_parallel_ledger
audit_tape_genesis_without_markov_pointer
audit_tape_blocks_unresolvable_present_markov_pointer
generate_markov_capsule_does_not_write_global_latest
markov_capsule_historical_artifact_not_reference_input
```

其中最重要的是：

```
assert!(!Path::new("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt").exists())
```

以及：

```
audit_tape(fresh_repo, fresh_cas, no_markov_pointer) -> Layer G skipped as genesis
audit_tape(pointer_present_unresolvable) -> BLOCK
```

### §B.4 — TB-16 目前是否可以继续做 sub-atom 2.2 / 2.4 / 2.6？

可以，但条件是：

```
先修 α。
```

尤其 sub-atom 2.4 / 2.6 要跑 multi-task SINGLE-CHAIN continuation。那是长期 β 的开始。

执行顺序：

```
TB-16.x.fix:
  delete/de-canonicalize global pointer

TB-16.x.2.2:
  ChallengeResolve / remaining arena pieces

TB-16.x.2.4:
  multi-WorkTx attempt + Boltzmann runtime

TB-16.x.2.6:
  combined arena run in single continuing chain
```

从 2.4 / 2.6 开始，才可以逐步建立 β：

```
single runtime_repo
single CAS
previous_capsule_cid found in same tape
```

不要在 α 未完成时继续用 global pointer 跑 arena audit。

### §B.5 — TB-16 ship gate 修正版

TB-16 要 ship，至少必须满足：

```
SG-16.1 Controlled market smoke produces replayable ChainTape.
SG-16.2 Dashboard shows positions, prices, masks, autopsies.
SG-16.3 No fake accepted nodes.
SG-16.4 Unsolved tasks show failure evidence / bankruptcy anchors.
SG-16.5 All market balances conserved.
SG-16.6 No unresolved evidence gaps.
SG-16.7 No global Markov pointer canonical input.
SG-16.8 Fresh isolated chain has markov_capsule=None and Layer G skipped.
SG-16.9 Present-but-unresolvable Markov pointer blocks.
SG-16.10 Multi-task continuation uses same runtime_repo + same CAS or explicit prior-chain-runtime-repo.
```

新增的 SG-16.7–16.10 是必须的，因为它们直接来自本次 OBS_R022。

### §B.6 — 对 TB-17 的更新方案

TB-17 原先是 Real-World Readiness Gate，这仍然正确。但必须增加一个新的准入条件：

> **不得进入真实世界 readiness，直到 Markov inheritance 不再依赖全局 filesystem pointer。**

所以 TB-17 的 preconditions 应更新为：

```
PRE-17.1
TB-16 global Markov pointer issue closed.

PRE-17.2
All run-to-run inheritance is either:
  a) in-tape continuation, or
  b) explicit prior-chain-runtime-repo input.

PRE-17.3
No global latest pointer acts as source of truth.

PRE-17.4
audit_tape can distinguish:
  genesis no prior Markov
  inherited Markov from valid prior chain
  invalid/unresolvable Markov pointer
```

如果这些没完成，TB-17 不能开，因为真实世界任务最怕"隐藏平行账本"。
真实世界 oracle / challenge / settlement 都必须建立在 canonical tape 上，不能依赖 last-writer-wins 的全局辅助文件。

### §B.7 — 与三张 Flowchart 的对齐

#### §B.7.1 Flowchart 1：Runtime Loop

TB-16 不能让 Markov pointer 作为 runtime hidden input 影响 read view 或 audit result。
Runtime 的事实应来自：

```
Q_t
ChainTape
CAS
Predicate
wtool
```

而不是：

```
handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt
```

所以 α 修复直接服务 FC1。

#### §B.7.2 Flowchart 2：Boot

Fresh isolated chain 是新的 Boot / Genesis：

```
previous_capsule_cid = None
markov_capsule = None
Layer G skipped
```

这不是绕过，而是正确 boot 语义。材料也明确说 Markov chain genesis 是 `previous_capsule_cid: None`，fresh isolated chain 没有 prior capsule。

如果要继承历史：

```
必须在 Boot 阶段显式提供 prior-chain-runtime-repo
或使用 continuing ChainTape。
```

#### §B.7.3 Flowchart 3：Meta Architecture

Markov capsule 是 logs archive 的压缩视图，不是 FC3 中的独立 ground truth。材料里也说它是派生视图，不是 flowchart node。

所以：

```
Markov capsule 可作为 InitAI 默认上下文；
但它必须由 tape + CAS 派生；
不能由 global pointer 作为隐性 source of truth。
```

### §B.8 — 后续到上线的方案梳理

#### §B.8.1 当前阶段：完成 TB-16

TB-16 结束前必须做：

```
1. 关闭 global LATEST_MARKOV pointer。
2. 通过 controlled market smoke。
3. 确认 no real funds / no public chain / no external domain。
4. 确认 positions / prices / masks / autopsies 都可 replay。
5. 确认 Markov capsule 只是派生视图，不是 source of truth。
```

#### §B.8.2 TB-17：Real-World Readiness Gate

TB-17 不执行真实世界任务，只做准入报告。

必须输出：

```
REAL_WORLD_READINESS_REPORT.md
DOMAIN_SELECTION_CRITERIA.md
ORACLE_REQUIREMENTS.md
CHALLENGE_COURT_REQUIREMENTS.md
SAFETY_BOUNDARY.md
IRREVERSIBLE_ACTION_POLICY.md
MARKOV_INHERITANCE_POLICY.md
```

新增最后一个：

```
MARKOV_INHERITANCE_POLICY.md
```

因为本次 TB-16 暴露了 run-to-run 继承的关键问题。

TB-17 ship gates 应更新为：

```
SG-17.1 Real-world readiness report passes audit.
SG-17.2 At least 3 candidate domains classified.
SG-17.3 At least 1 low-risk pilot domain approved.
SG-17.4 Oracle design documented.
SG-17.5 ChallengeCourt design documented.
SG-17.6 Human escalation path documented.
SG-17.7 No production real-world task launched yet.
SG-17.8 Irreversible action policy tested against examples.
SG-17.9 Markov inheritance policy documented and tested.
SG-17.10 No global filesystem pointer source-of-truth remains.
```

#### §B.8.3 TB-18：Low-Risk Real-World Pilot Design

TB-18 才开始设计真实世界 pilot，但仍不执行高风险行动。

候选领域应满足：

```
T2-like
low stakes
clear oracle
no irreversible external action
delayed settlement
human review available
```

建议候选：

```
1. 文档引用核验 / citation verification
2. 开源代码 issue reproduction
3. 数学/Lean/Coq/Isabelle formalization
4. Web benchmark data extraction with deterministic checks
```

不建议一开始做：

```
medical
legal
financial trading
physical robotics
security exploit deployment
autonomous API actuation
```

#### §B.8.4 TB-19：Pilot Sandbox

在 sandbox 跑真实世界低风险任务：

```
no real money payout
no external side effects
oracle logs in CAS
challenge window
human review
```

#### §B.8.5 TB-20：Limited Real-World Beta

小范围、低风险、限额、延迟结算：

```
small bounty
manual sponsor approval
human escalation
no irreversible action
audit dashboard
```

### §B.9 — 给 AI coder 的直接执行指令（architect 原文）

```
Architect ruling for TB-16:

1. Confirm Claude's strict-alignment analysis:
   MarkovEvidenceCapsule is NOT a flowchart node.
   It is a derived view / evidence compression.
   It must be derivable from ChainTape + CAS.

2. Confirm genesis semantics:
   Fresh isolated runtime_repo + fresh CAS => previous_capsule_cid=None.
   markov_capsule=None.
   Layer G Skipped is constitutional in this genesis case.

3. Confirm that present-but-unresolvable pointer must BLOCK.
   Silent .ok() collapse is forbidden.

4. Rule on OBS_R022:
   LATEST_MARKOV_CAPSULE.txt is a parallel ledger under Art. 0.2.
   It cannot remain canonical.

5. Adopt Option α immediately:
   delete/de-canonicalize LATEST_MARKOV_CAPSULE.txt.
   make --markov-pointer optional.
   add --prior-chain-runtime-repo for explicit inheritance.
   stop generate_markov_capsule from writing global latest pointer.
   add tests that global latest pointer does not exist and audit_tape does not require it.

6. Adopt Option β as long-term:
   multi-task continuation uses same runtime_repo + same CAS.
   previous_capsule_cid is inherited in-tape.
   no global pointer.

7. Reject Option γ:
   do not add a second provenance sidecar to police the first parallel ledger.

8. No Phase Z′ required:
   no FC1/FC2/FC3 modification.
   This is Art. 0.2 derived-view enforcement, not flowchart revision.

9. TB-16 cannot ship until the pointer issue is closed.

10. TB-17 readiness gate must add MARKOV_INHERITANCE_POLICY.md and a no-global-pointer source-of-truth ship gate.
```

### §B.10 — 最终判断（architect 原文）

TB-16 的 Controlled Market Smoke Arena 方向是正确的，但它暴露了一个更基础的宪法问题：

```
当系统开始跨 run / 跨 task 使用 Markov memory 时，
任何"全局 latest pointer"都会变成隐藏平行账本。
```

这类问题如果现在不修，到真实世界阶段会非常危险，因为真实世界 oracle、challenge、settlement、human escalation 都会依赖历史证据继承。

所以我的最终判断是：

```
TB-16 当前进展可继续；
但 ship 前必须先清除 LATEST_MARKOV_CAPSULE.txt 的 canonical 地位；
α 立即执行；
β 作为后续 chain continuation 目标；
TB-17 增加 Markov inheritance readiness gate。
```

这一步虽然看起来像"小文件指针问题"，实质上是 TuringOS tape canonical 公理的一次实战检验。当前必须严格处理，不能凑合。

---

## §C — Status

`RATIFIED — pending Claude execution authorization`

Per `/architect-ingest` step 4: Claude has archived + analyzed; **no pointer deletions, no source code edits, no test additions until user explicitly authorizes execution.**

Suggested execution order on authorization:
1. Open new TB charter `TB-16.x.fix` (Class 2 production wire-up; FC-trace: FC1 + FC2 + Art. 0.2; phase_id: P5; roadmap_exit_criteria_addressed: SG-16.7..16.10; kill_criteria_tested: parallel-ledger no-global-pointer守恒).
2. Step-B protocol: `audit_tape.rs` / `audit_tape_tamper.rs` are restricted-binary surfaces; A/B parallel branch.
3. Implement work-items §A.3 in order; ship 5 守恒 tests first (red), then code (green).
4. Smoke-before-batch on a fresh genesis chain to confirm Layer G Skipped + replay.
5. Risk-class audit (Class 2 → hybrid dual: Codex + Gemini).
6. Update memory entries §A.8 after ship.
