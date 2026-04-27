# Constitution Amendment Draft — Art. 0.5

> **🧊 FROZEN 2026-04-27**: User directive: "现在不能修改宪法，因为白皮书还没正式定稿，现阶段不作任何宪法修订"
> Enactment is **PAUSED** until white paper is formally finalized. The text below remains the canonical proposal but is NOT to be applied via cp-workflow at this time.
>
> Trigger to unfreeze: user signs a `v4-whitepaper-finalized-*` tag covering both architecture + economic chapters; THEN Ceremony B in `ENACTMENT_PROCEDURE_2026-04-27.md` may run.
>
> **Status**: DRAFT (FROZEN) awaiting white paper finalization.
>
> **Authority required**: per Art. V.3, constitution amendments require human architect explicit sudo. ArchitectAI cannot self-enact.
>
> **D2 choice**: B — pointer + 6 公理 only (full white paper text stays in `handover/whitepapers/`).
>
> **Insertion point**: after Art. 0.4 (line ~155 of constitution.md), before "## 0.6 …" or before Art. I (whichever exists).
>
> **Drafted**: 2026-04-26 night shift, ArchitectAI auto-research mode.

---

## Proposed Text (to be inserted into constitution.md)

```markdown
## 0.5 白皮书集成与六公理 [Art. 0.5]

> **触发**：用户 2026-04-26 ultrathink 提供完整白皮书 (architecture chapter + economic chapter) 并要求"用最大智慧呈现完整按照宪法落地的 turingos"。本宪法集成白皮书作为本宪法的 elaboration，以 pointer + 公理形式纳入，避免本宪法臃肿。

### 0.5.1 白皮书的宪法地位

下列两份文档与本宪法**同等地位**，构成 TuringOS v4 的基础规范：

- `handover/whitepapers/TURINGOS_WHITEPAPER_v1_2026-04-26.md`（架构章，21 §）
- `handover/whitepapers/TURINGOS_WHITEPAPER_v1_ECONOMIC_2026-04-26.md`（经济章，21 §）

两份文档由人类架构师 (gretjia) 2026-04-26 ultrathink 模式 authored；SHA 已锁入 Trust Root manifest（`genesis_payload.toml`）。任何修改走 Art. V.3 amendment 程序。

本宪法（constitution.md）描述**最小必要规则**；白皮书描述**完整架构体系**。冲突时：

1. 若 conflict 为 spec gap（白皮书无定义）→ 以本宪法 default 为准
2. 若 conflict 为 spec drift（本宪法滞后）→ 以白皮书为权威，本宪法触发 Art. V.3 amendment 同步
3. 若 conflict 为 substantive disagreement → ArchitectAI 必须停止实现，触发 Veto-AI 双外审 + 用户 sudo 仲裁

### 0.5.2 六公理 (Six Axioms)

ArchitectAI 在白皮书全文中提取的**最高优先级公理**，作为本宪法的 hard constraint：

#### 公理 1：反奥利奥三层 (Anti-Oreo)

系统必须显式分为三层：⚪ **顶层白盒**（谓词/信号/预算）/ ⚫ **中间黑盒**（agent 推理）/ ⚪ **底层白盒**（tape/CAS/ledger/沙盒）。每个 `pub` 符号必须可分类到**且仅一层**。Mixed-layer 模块是宪法违规。

**code locus**: `src/{top_white, middle_black, bottom_white}/*.rs` + `src/economy/*` + `src/{state, transition}/*`

**conformance test**: `tests/anti_oreo_layer_audit.rs`

#### 公理 2：Tape Canonical（与 Art. 0.2 一致）

`Q_t` 的所有字段必须可由 ChainTape 重放重建。任何"live cache only"字段都是宪法违规。共识只证记录被接受，不证现实事实为真（Inv 12）。

**code locus**: `src/state/q_state.rs::QState` + `src/bottom_white/tape/chain_tape.rs::replay_from_genesis`

**conformance test**: `tests/q_state_reconstruct.rs`, `tests/tape_canonical_V01..V24.rs`

#### 公理 3：Goodhart 屏蔽

谓词分三种 visibility：`Public` / `Private` / `CommitReveal`。Agent 的 read view 必须按 visibility 过滤。私有谓词不得通过 error msg / log / retry count 泄漏。价格信号广播；完整评分器屏蔽（Inv 10）。

**code locus**: `src/top_white/predicates/visibility.rs` + `src/bottom_white/materializer/agent_view.rs`

**conformance test**: `tests/goodhart_shield.rs`, `tests/economic_invariant_INV10_signal_vs_evaluator.rs`

#### 公理 4：Boolean × Statistical 信号二分

所有信号属于二选一：**布尔**（pass/fail，谓词输出）或**统计**（价格、信誉、稀缺度）。不得创造第三类（"模糊布尔"或"加权统计降为布尔"皆违宪）。

**code locus**: `src/top_white/signals/{boolean.rs, statistical.rs}`

**conformance test**: `tests/signal_dichotomy.rs`

#### 公理 5：Predicate-Gated Transition（与 Inv 6 一致）

未通过谓词的 work_tx 不得改变 world state。状态转移流程严格七步：READ → PROPOSE → PREDICATE GATE → PROVISIONAL → STATE TRANSITION → SIGNAL EMIT → CHALLENGE WINDOW（→ FINALIZE 在挑战窗结束后）。任何 short-circuit 是宪法违规。

**code locus**: `src/transition/mod.rs::step_transition`

**conformance test**: `tests/economic_invariant_INV6_predicate_gated.rs`, `tests/economic_invariant_INV7_provisional_then_final.rs`

#### 公理 6：Escrow-Only Reward（与 Inv 1, 2, 3, 4 一致）

奖金必须来自预锁定 escrow / 合法 treasury，不得事后增发（Inv 3, 4）。Agent 不直接领奖，只提交 claim；Settlement Engine 决定最终发放（Inv 2）。Agent 不因思考获奖，只因被接受的状态转移获奖（Inv 1）。最终奖金公式：

```
reward_i = Finalize(Escrow × Accept × Attribution × Survival × Utility × Constitution)
```

**code locus**: `src/economy/{escrow_vault.rs, settlement_engine.rs}`

**conformance test**: `tests/economic_invariant_INV1..4.rs`, `tests/final_reward_formula.rs`

### 0.5.3 公理与白皮书章节的 mapping

| 公理 | architecture 章节 | economic 章节 |
|---|---|---|
| 1 反奥利奥 | § 3 | （隐含；agent layer 边界） |
| 2 Tape Canonical | § 4, § 5 | § 2 (Q_t amendment), Inv 11, Inv 12 |
| 3 Goodhart 屏蔽 | § 9.4 | Inv 10, § 16 |
| 4 信号二分 | § 7 | § 11 (price index), § 14 (reputation) |
| 5 Predicate-gated | § 6 | Inv 6, Inv 7 |
| 6 Escrow-only | § 10 (Laws of Money), § 12 | Inv 1-4, § 21 final formula |

### 0.5.4 修订协议

- 公理修订：Art. V.3 amendment 程序，要求 ArchitectAI 提案 + Veto-AI 双外审 + 用户 sudo
- 白皮书修订：同上，且白皮书修订必须**同时**触发本宪法 Art. 0.5.3 mapping table 同步
- 公理与白皮书 conflict：见 0.5.1 三条规则

### 0.5.5 与 Art. 0-0.4 关系

- Art. 0（图灵机原教旨）→ 公理 2（Tape Canonical）扩展
- Art. 0.1（四要素映射）→ 公理 1（反奥利奥三层）+ 公理 5（predicate-gated）扩展
- Art. 0.2（Tape Canonical 公理）→ 直接公理 2
- Art. 0.3（区块链化保留）→ 公理 2 实现路径
- Art. 0.4（Q_t version-controlled）→ 公理 2 实现路径选择 (B = real git)

Art. 0.5 是 Art. 0-0.4 的**集成与升华**，不替代任何先前条款。
```

---

## Notes for User Enactment

### Insertion procedure (per R-018 V.3 cp workflow)

```bash
# 1. backup current constitution
cp /home/zephryj/projects/turingosv4/constitution.md /tmp/c_old.md

# 2. open in your editor of choice; insert proposed text after Art. 0.4 closing paragraph
# (the line containing "Phase E gate 强制 B" or similar marking end of Art. 0.4)

# 3. cp back
cp /tmp/c_new.md /home/zephryj/projects/turingosv4/constitution.md

# 4. update Art. V.3 modification log with this entry
# 5. recompute SHA + update genesis_payload.toml trust_root["constitution.md"]
sha256sum /home/zephryj/projects/turingosv4/constitution.md
```

### Decision points before enactment

1. **D2 confirm**: still B (pointer-only)? Or override to A (full white paper text inline)?
2. **公理顺序**: 6 axioms are listed in spec-flow order (architecture → economy). User may prefer Inv-priority order (Inv 1 first).
3. **公理粒度**: ArchitectAI selected 6; user may want to expand to 9 (one per Q_t economic_state_t sub-field) or compress to 4 (Anti-Oreo + Tape + Predicate + Economy).
4. **mapping table**: Art. 0.5.3 lists 6 rows; can be replaced by deeper section-by-section mapping (would grow doc).

### Trust Root impact

After Art. 0.5 lands:
- constitution.md SHA changes
- `genesis_payload.toml` trust_root["constitution.md"] must update
- Entire boot verify_trust_root chain must re-validate
- Any external audit packet referring to old constitution SHA must re-issue

— ArchitectAI, 2026-04-26 night DRAFT
