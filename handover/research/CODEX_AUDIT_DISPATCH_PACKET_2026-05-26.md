# Codex Audit Dispatch Packet — AgentOutputEnvelope Research

For: Clean-context Codex audit witness per AGENTS.md §9 + §14
Witness role: SINGLE Codex (Gemini auditor dropped 2026-05-24 per architect
   ratification — see memory `feedback_dual_audit`)
Verdict domain: `{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE,
   SECOND-SOURCE-DRIFT}` — no subjective opinions accepted (style /
   performance / coverage / architecture preference are out-of-scope)
Date prepared: 2026-05-26
Dispatch status: **PREPARED, NOT YET DISPATCHED** — user authorizes dispatch

---

## 1. Task brief (verbatim handover to Codex)

> 你是一个 clean-context Codex 审计 witness。你**没有看过**这个研究
> 的产出，也**没有**与设计者的对话。你将获得 3 件输入：
>
> 1. 一份研究文档（设计 AgentOutputEnvelope JSON schema + 失败子类
>    taxonomy）
> 2. 一个 PoC subcrate（standalone Rust，24 测试）
> 3. evidence + 自审计记录
>
> 你的工作是判断: 这套设计与 PoC 是否构成"研究结论"的有效证据，
> 还是隐含违宪 / 重建失败 / 第二源漂移。
>
> 你的合法输出空间只有 4 个:
>   - `NO-VIOLATION` — 扫了 N 条款无违宪
>   - `VIOLATION-FOUND <constitutional-clause> <file>:<line>`
>   - `RECONSTRUCTION-FAILURE <which-tape-or-cas-path-cannot-be-reconstructed>`
>   - `SECOND-SOURCE-DRIFT <which-derived-view-is-usurping-ground-truth>`
>
> 越界（任何主观品味 / 性能 / 覆盖率 / 架构偏好评论）= 用户或主体 Claude
> 可拒收。

---

## 2. Risk class declaration

- Research itself: Class 0 (docs/research, worktree-only)
- Future Phase A implementation: Class 1 additive
- This audit packet scope: ONLY Class 0 (the research + PoC, NOT future
  Phase A; that audit happens at Phase A charter draft)

---

## 3. Touched FC nodes / invariants

| FC node | Touched? | How |
|---|---|---|
| FC1a-rtool | DESIGN ONLY — adapter envelope check before judge | research §1.1, §3-§5 |
| FC1a-judge_pi | DESIGN ONLY — judge sees parsed payload, not raw body | research §3.3 |
| FC1a-output_edge | DESIGN ONLY — `extract_body` superseded by `validate` | research §1.1 |
| FC1-N41 (AttemptTelemetry CAS object) | DESIGN ONLY — proposed tail-additive `envelope_validation_subclass: Option<...>` field | research §4.2, §9 Phase B |
| FC1-N42 (attempt-to-L4/L4.E routing) | DESIGN ONLY — surject sub-classes to existing `RejectionClass` | research §4.2 table |
| FC1-N43 (chain_derived_run_facts equation) | INVARIANT-CHECK ONLY — proved unchanged in PoC `fc1_invariant.rs` | research §4.4, EVIDENCE §4 row A4 |
| FC2 (boot / replay / predicate activation) | NOT TOUCHED |
| FC3 (meta-architecture) | DESIGN ONLY — FC3-directive envelope schema sketched at research §5.4; explicit disclaimer that this does not unlock OBL-005 |

**Canonical FC1 invariant under audit**:
```
evaluator_reported_completed_llm_calls
=
  tool_dist.step + tool_dist.parse_fail + tool_dist.llm_err
```
PoC asserts this equality holds on a 12-attempt mixed batch with envelope
sub-classes routing through `parse_fail` bucket (`fc1_invariant.rs:62-117`).

---

## 4. Current diff / commit

- Worktree HEAD: `03a84470` (current main HEAD as of 2026-05-26)
- Branch (in worktree): detached HEAD on 03a84470
- `git diff --name-only HEAD`: **empty** (zero modifications to tracked files)
- Untracked additions: 5 doc files under `handover/research/` + PoC
  subcrate at `research/envelope_poc/` (own workspace, isolated)

Reproduce:
```bash
cd /tmp/turingosv4-agent-schema-research
git log --oneline -1     # expect 03a84470
git diff --name-only HEAD   # expect empty
git ls-files --others --exclude-standard | rg '^src/' || echo "no src/ untracked"
```

---

## 5. Relevant files (read these in order)

### 5.1 Research design

1. `handover/research/AGENT_OUTPUT_ENVELOPE_RESEARCH_2026-05-26.md`
   — full design (§1 baseline, §2 principles, §3 schema, §4 taxonomy,
   §5 4-runner integration, §6 structure-vs-truth table, §7 objections,
   §8 Class 4 red lines, §9 phased rollout, §10 OBL-005 dependency)

### 5.2 PoC code

2. `research/envelope_poc/Cargo.toml`         — standalone workspace
3. `research/envelope_poc/src/envelope.rs`   — validator + surrogate enums
4. `research/envelope_poc/tests/decoupling.rs`     — A1, A3
5. `research/envelope_poc/tests/fc1_invariant.rs`  — A4
6. `research/envelope_poc/tests/robustness.rs`     — A5, A6, A7

### 5.3 Source files referenced by surrogate mapping (read-only)

7. `src/runtime/attempt_telemetry.rs:154-186` — `AttemptOutcome` enum
8. `src/runtime/mod.rs:70` — `RejectionClass::ParseFailed=7` comment
9. `src/tdma_runner.rs:275,377` — `LlmResponse` + `extract_body`
10. `src/judges/generate_judge.rs:83-120` — current ad-hoc fence parser

### 5.4 Supporting docs

11. `handover/research/VALIDATION_AND_AUDIT_PLAN_2026-05-26.md`
12. `handover/research/EVIDENCE_2026-05-26.md`
13. `handover/research/SELF_AUDIT_2026-05-26.md`

---

## 6. Evidence paths

PoC test output: `EVIDENCE_2026-05-26.md` §2-§4
Build output: `EVIDENCE_2026-05-26.md` §1
File-level diff scope: `EVIDENCE_2026-05-26.md` §5
Self-audit rollup: `SELF_AUDIT_2026-05-26.md` §1-§9

---

## 7. Exact verification commands + expected output

Run these in order. Each command must produce the expected output.

```bash
# 1. Confirm zero src/ touch
cd /tmp/turingosv4-agent-schema-research
git diff --name-only HEAD
# expected: (empty)

# 2. Confirm no restricted-surface untracked
git ls-files --others --exclude-standard | rg '^src/' || echo "no src/ untracked"
# expected: "no src/ untracked"

# 3. Build PoC
cargo build --manifest-path research/envelope_poc/Cargo.toml
# expected: "Finished `dev` profile [unoptimized + debuginfo] target(s)" exit 0

# 4. Run PoC tests
cargo test --manifest-path research/envelope_poc/Cargo.toml --no-fail-fast
# expected: "test result: ok. 8 passed" (decoupling)
#           "test result: ok. 2 passed" (fc1_invariant)
#           "test result: ok. 14 passed" (robustness)
#           overall exit 0, 24/24 pass

# 5. FC1 invariant single-test (canonical assertion)
cargo test --manifest-path research/envelope_poc/Cargo.toml --test fc1_invariant fc1_lhs_invariant_holds_under_mixed_batch
# expected: "test result: ok. 1 passed"

# 6. Decoupling assertion
cargo test --manifest-path research/envelope_poc/Cargo.toml --test decoupling envelope_pass_does_not_imply_predicate_pass envelope_fail_short_circuits_predicate
# expected: "test result: ok. 2 passed"

# 7. Privacy invariant
cargo test --manifest-path research/envelope_poc/Cargo.toml --test robustness rejection_payload_carries_only_hash_prefix
# expected: "test result: ok. 1 passed"
```

---

## 8. What Codex should specifically check

Targeted findings (audit MUST report on each):

C1. **Surrogate enum mapping byte-stability**: does
    `research/envelope_poc/src/envelope.rs:23-43` accurately mirror
    `src/runtime/attempt_telemetry.rs::AttemptOutcome` (lines 154-186)?
    If `repr(u8)` discriminants drift between PoC and main crate, the
    Phase A migration claim collapses. Verdict if drift exists:
    `SECOND-SOURCE-DRIFT envelope_poc::AttemptOutcomeSurrogate vs runtime::AttemptOutcome`.

C2. **Privacy invariant CR-18R.4 v2 byte-fence**: does
    `EnvelopeRejectionPayload` (envelope.rs:179-203) leak any candidate
    bytes beyond a sha256 prefix? Re-read CR-18R.4 v2 (attempt_telemetry.rs:34-56)
    and confirm. Verdict if leak: `VIOLATION-FOUND CR-18R.4_v2 research/envelope_poc/src/envelope.rs:179`.

C3. **FC1 invariant LHS scope** matches CLAUDE.md §4 canonical text? The
    PoC LHS = `step + parse_fail + llm_err`, NOT `evaluator_reported_tx_count`.
    Verdict if mismatched: `VIOLATION-FOUND CLAUDE.md§4 research/envelope_poc/tests/fc1_invariant.rs:30`.

C4. **Class 4 red-line check**: research §8 lists 6 things not done. Verify
    each:
       (a) no new AttemptOutcome variant → grep envelope.rs and AttemptOutcomeSurrogate
       (b) no new RejectionClass variant → grep envelope.rs and RejectionClassSurrogate
       (c) no new ObjectType variant → grep — PoC doesn't touch cas/schema.rs at all
       (d) no sequencer admission change → PoC is standalone, no sequencer code
       (e) envelope check has no side effects (no recursive distiller, no message-board write)
           → review envelope.rs::validate signature
       (f) envelope check not inside sequencer → it's in the runner adapter
    Verdict if any violation: `VIOLATION-FOUND class4_red_line_<letter> <file>:<line>`.

C5. **Reconstruction property**: replay a body twice through `validate` —
    does it deterministically produce the same `(EnvelopeValidationSubclass,
    path, message)` triple? Verdict if non-deterministic:
    `RECONSTRUCTION-FAILURE envelope_poc::validate`.

C6. **OBL-005 dependency claim correctness**: research §10 claims this
    research does NOT unlock OBL-005. Verify by reading the current
    OBL-005 evidence section in `OBLIGATIONS.md` and confirming the
    blocker is FC3 production paths (MISSING/EXTERNAL_ONLY), not schema
    design. Verdict if research mis-claims: `VIOLATION-FOUND obligation_substitution research/.../RESEARCH_2026-05-26.md:§10`.

---

## 9. Auditor checklist — predicate verification recipe (AGENTS.md §14)

Adapted to Class 0 research (Phase A would re-run full version):

- [ ] Risk class declared = Class 0 ✓
- [ ] `git diff --name-only HEAD` returns empty
- [ ] No file under `src/` modified or untracked-added
- [ ] PoC tests pass: `cargo test --manifest-path research/envelope_poc/Cargo.toml --no-fail-fast` exit 0
- [ ] All 24 PoC tests GREEN
- [ ] No new `Manager`/`Factory`/`Engine`/`Platform`/`Framework` struct in PoC (`rg "struct (.*Manager|.*Factory|.*Engine|.*Platform|.*Framework)" research/envelope_poc/`)
- [ ] No new trait + single impl in PoC (validate is plain fn)
- [ ] No new board-as-truth file outside CAS evidence chain (research is docs only)
- [ ] No new global-latest pointer (validate takes explicit ValidateContext)
- [ ] Verdict ∈ `{NO-VIOLATION, VIOLATION-FOUND, RECONSTRUCTION-FAILURE, SECOND-SOURCE-DRIFT}`
- [ ] No subjective code-style / performance / coverage opinion in output

If all checks pass and §8 C1-C6 each report clean: `NO-VIOLATION`.

---

## 10. Format expected from Codex

A single short report ending with one of:

```
VERDICT: NO-VIOLATION
or
VERDICT: VIOLATION-FOUND <clause> <file>:<line>
or
VERDICT: RECONSTRUCTION-FAILURE <which-path>
or
VERDICT: SECOND-SOURCE-DRIFT <which-derived-view>
```

Body of report (before the VERDICT line) should:
- list which §8 C1-C6 checks were performed
- cite specific files/lines for each
- distinguish production defects from research-scaffold gaps
- NOT include subjective opinion

---

## 11. Dispatch sequence (when user authorizes)

```bash
# Option A: via /codex-rescue with explicit packet path
# (Claude orchestrator runs:)
/codex --packet /tmp/turingosv4-agent-schema-research/handover/research/CODEX_AUDIT_DISPATCH_PACKET_2026-05-26.md

# Option B: manual via codex CLI from worktree
cd /tmp/turingosv4-agent-schema-research
codex run --witness --packet handover/research/CODEX_AUDIT_DISPATCH_PACKET_2026-05-26.md

# Capture audit output → handover/research/CODEX_AUDIT_VERDICT_2026-05-26.md
```

---

## 12. Pre-dispatch user authorization checklist

Before dispatching:

- [ ] User confirms research scope is Class 0 (docs/research only)
- [ ] User confirms audit witness verdict is binding for the future Phase A
      charter authorization (i.e., a `VIOLATION-FOUND` blocks the Phase A
      charter)
- [ ] User confirms single-Codex audit policy (Gemini dropped per 2026-05-24
      ratification)
- [ ] User confirms verdict-domain enforcement (subjective opinions → reject)

---

(结束 — packet ready, awaiting user dispatch authorization)
