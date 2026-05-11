# TB-G G1.2 — Option B+ Architect Ruling (2026-05-11)

> **Status**: ARCHIVED + AUTHORIZED FOR EXECUTION 2026-05-11 (web ultraplan
> session `01TE83pKjhdGZgVztYMJg7tr` approved verbatim "Ultraplan approved in
> browser" with full plan body returned to terminal).
>
> **Class of this directive**: 0 (architectural ruling; affects Class-3
> implementation of TB-G G1.2). Resolves the §11 questions raised in
> `handover/directives/2026-05-11_TB_G_G1_2_ORCHESTRATION_DECISION_PACKET.md`.
>
> **Supersedes**: G1.2 single-row charter line for "G1.2 Batch driver binary"
> in `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1.

---

## §0. Header

- **TB**: TB-G atom **G1.2** (now expanded to G1.2-0..G1.2-8)
- **Charter**: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- **Parent §8**: G1.1 packet §6 SIGNED 2026-05-11 ("好，确认可以 ship")
- **Class**: 3 by default (production wire-up; binary-layer + orchestration).
  HALT and re-charter Class 4 if implementation forces touching sequencer
  admission, TypedTx schema, canonical signing payload, `HEAD_t` definition,
  economic state schema, system tx authorization, or constitution.md text.
- **`origin/main` HEAD at ruling time**: `5f90171` (post G1.1 ship)
- **Phase-id**: P3-G (RSP Economy Generative Arena)
- **FC-trace**: FC1 Runtime Loop (per-problem L4/L4.E externalization preserved
  per subprocess) + FC2 Boot (cross-problem chain continuity; resume primitive
  promoted to first-class boundary)

---

## §1. Architect verdict (verbatim)

The architect chose:

> **Option B+ — Process-distributed, Tape-continuous runtime**
>
> 每个 problem 可以由 subprocess 执行；
> 但所有 subprocess 必须 resume 同一条 ChainTape：
> - same runtime_repo
> - same CAS
> - same agent registry
> - same system pubkeys
> - same batch_id
> - continuous HEAD_t
> - no fresh genesis
> - no memory-only cross-task state
>
> 如果一个 subprocess 自己创建新的 runtime_repo、新 CAS、新 genesis，那就是不合格。
> 如果 subprocess 只是接续同一条 tape，那就是合格。

Disposition of the three options in the decision packet (§1):

- **Option A** (single-process in-memory loop) → dev-only harness, **not ship
  evidence**.
- **Option B** (subprocess with `TURINGOS_CHAINTAPE_RESUME=1`) → **canonical
  with hardening** (Option B+ below).
- **Option C** (hybrid) → allowed **only as wrapper** over Option B+ semantics
  (orchestrator may manage subprocesses, but the fact layer must be Option B's
  tape continuation).

One-sentence framing:

> **进程是否重启不重要，tape 是否连续才重要。**
> 图灵机可以停机再启动，但纸带不能换。

---

## §2. Six architect Q-resolutions

### Q1 — load-bearing orchestration model

**Option B+**.

Canonical:
- subprocess-per-task allowed iff `TURINGOS_CHAINTAPE_RESUME=1` and an explicit
  resume contract passes preflight.

Non-canonical:
- single-process in-memory loop (Option A) is dev-only.

### Q2 — reaffirm §2.8 operative clause

Yes. Operative reading:

> Forbidden = each subprocess creates own chain.
> Not forbidden = subprocess itself.

But subprocess resume **must be explicit and fail-closed**. Setting
`TURINGOS_CHAINTAPE_RESUME=1` alone is **not enough**. The resume contract
must carry:

- `--runtime-repo`
- `--cas`
- `--expected-head-t`
- `--batch-id`
- `--task-id`
- `--agent-registry`

…else it is a pseudo-resume.

### Q3 — SG-G1.7 reword

Authorized. Replace any "one process" language with "one continuous
ChainTape":

> **SG-G1.7**
> A batch of >=3 tasks must execute on one continuous ChainTape:
> - same runtime_repo
> - same CAS
> - exactly one genesis
> - `task_{k+1}.start_head_t == task_k.end_head_t`
> - agent balances / positions / reputation persist across task boundaries.

### Q4 — risk class

**Class 3 by default**.

It touches: runtime orchestration, evaluator hot path, ChainTape resume,
agent state persistence, evidence semantics — all Class 3 surfaces.

STOP and re-charter as **Class 4** if implementation forces touching any of:

- sequencer admission semantics
- TypedTx canonical schema
- canonical signing payload
- system tx authorization
- economic state schema
- `HEAD_t` definition
- constitution.md text

### Q5 — audit cadence

Do **not** per-atom dual audit. Do **not** wait until full batch for first look.

Execute:
1. **3-task mini-smoke + ResumePreflight ship** → Codex micro-audit.
2. **9-task persistent batch ship** → Codex + Gemini full dual audit.

No schema-only audit cycles. Real tape evidence first.

### Q6 — forward §8 implications

Option B+ establishes a forward principle:

> 所有未来跨 run / 跨 task / 跨 agent 的状态，
> 都必须由 ChainTape/CAS 继承，
> 不能由 evaluator 内存继承。

Affects:
- G5 opportunity scheduler (state must be chain-resident)
- Markov inheritance
- agent PnL
- role differentiation
- NodeMarket prices
- Autopsy
- future real-world pilot

For any future state, ask:

```
- Is this memory on tape?
- Can replay reconstruct it?
- Can HEAD_t / CAS / L4 / L4.E rebuild it?
```

If any answer is no → not compliant.

---

## §3. Architect-identified hidden risks (mandatory mitigations)

The packet listed only three options; the architect added **seven** structural
risks that must be addressed by the G1.2 atom list:

### 3.1 `TURINGOS_CHAINTAPE_RESUME=1` is a signal, not a safety protocol

Add **ResumePreflight**. Pre-spawn checks:

- runtime_repo exists
- CAS exists
- agent_registry exists
- system_pubkeys exist
- HEAD_t exists
- `expected_head_t` matches actual current head
- `batch_id` matches
- `task_index` increments by exactly 1
- no new genesis_report created
- no `on_init` after task 0

On any fail: exit nonzero. No fresh-genesis fallback.

### 3.2 ChainTapeLease (single-writer)

Even sequential subprocesses need a lock — concurrent expansion is forward.

```text
ChainTapeLease {
  holder_pid,
  batch_id,
  start_head_t,
  acquired_at,
}
```

Commit-time check: `current_head_t == expected_start_head_t`. Two subprocesses
must never write `refs/transitions/main` concurrently.

### 3.3 BatchContinuationManifest

Without it, "same batch" has no fact-identity. Add:

```text
BatchContinuationManifest {
  batch_id,
  runtime_repo,
  cas_root,
  initial_head_t,
  tasks: Vec<TaskContinuationEntry>,
  agent_registry_cid,
  system_pubkeys_cid,
  model_manifest_cid,
}

TaskContinuationEntry {
  task_id,
  problem_id,
  start_head_t,
  end_head_t,
  subprocess_command_cid,
  run_summary_cid,
  terminal_tx_id,
}
```

Manifest enters CAS. Referenced by terminal/batch summary tx.

### 3.4 G1.1 3-subprocess mini-smoke is precedent, not proof

G1.2 needs its **own** evidence:

- ≥9 tasks
- same runtime_repo
- same CAS
- one genesis
- continuous HEAD_t
- balances / positions / PnL persist

### 3.5 Cross-problem persistence may expose economic deadlock

Possible: agent balance drops, cannot stake / invest / challenge.

This is not a bug — may be economic truth. G1.2 must address:

- minimum survival allowance?
- bankruptcy policy?
- read-only bankrupt agent?
- autopsy trigger?

**Do not** silently reset balances. Reset = fresh-genesis variant.

### 3.6 Opportunity scheduler must be chain-resident

G5.1 scheduler state cannot be in-memory. Observe-only scheduler OK, but
decisions need CAS records:

```text
SchedulerDecisionTrace {
  task_id,
  head_t,
  visible_agents,
  visible_nodes,
  selected_agent,
  selected_action,
  reason_summary_public,
}
```

### 3.7 Market context requires same-chain persistence

Without persistent balance/positions, observed price is meaningless. Until
G1.2 lands, 0 invest cannot be interpreted — it is per-problem-isolation
artifact, not multi-agent market failure.

---

## §4. Authorized atom list (replaces single G1.2 row)

| Atom | Class | Subject |
|------|-------|---------|
| **G1.2-0** | 0 | Charter amendment + this directive archive |
| **G1.2-1** | 2-3 | ResumePreflight library + CLI shim + 11 tests |
| **G1.2-2** | 2 | ChainTapeLease library + 6 tests |
| **G1.2-3** | 2-3 | Extract `swarm_one_problem` + `batch_evaluator` binary + 5 tests |
| **G1.2-4** | 2 | BatchContinuationManifest + 4 tests |
| **G1.2-5** | 2 | Persistent agent state evidence binding test |
| **G1.2-6** | 2 | 3-task mini-smoke + Codex micro-audit |
| **G1.2-7** | 2 | 9-task persistent batch + Codex+Gemini full dual audit |
| **G1.2-8** | 0-1 | Cross-problem persistence report + matrix sync |

Halt conditions (any of):

- fresh `genesis_report.json` appears at `task_index > 0`
- `runtime_repo` or CAS path differs across tasks
- `HEAD_t` discontinuity between consecutive tasks
- agent balance / positions reset silently
- subprocess falls back to legacy non-ChainTape path
- implementation needs Class-4 surface mutation (see §2 Q4)

---

## §5. Final principle (architect §7 verbatim)

> 你现在真正要证明的，不是"subprocess 是否允许"。
> 你要证明的是：
>
> **TuringOS 的智能生命可以死亡、重启、换进程、换模型，
> 但只要它回到同一条 tape，就仍然是同一个世界。**
>
> 这正是图灵机原教旨主义：
>
> - 机器可以停；
> - 人可以换；
> - 纸带不能丢；
> - 读写头必须知道自己在哪；
> - 下一步必须从上一步的符号继续。

This becomes the load-bearing axis for G1.2 evidence and forward G3/G5/G7
state design.

---

## §6. Cross-references

- Decision packet (now resolved): `handover/directives/2026-05-11_TB_G_G1_2_ORCHESTRATION_DECISION_PACKET.md`
- G1.1 §8 packet: `handover/directives/2026-05-11_TB_G_G1_1_§8_PACKET.md`
- G-Phase directive: `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md`
- Charter: `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
- Matrix: `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R
- CLAUDE.md §1-22 (constitutional operating law)
- `feedback_no_batch_class4_signoff` (no batch §8; per-atom only)
- `feedback_constitutional_harness_engineering` (harness first, real run, then audit)
