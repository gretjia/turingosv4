# Constitution Coverage Gap Audit — Pass 2 (2026-05-07)

## §0 Authority + Method

- **Parent**: `handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md` enumerated 73 clauses → 30 numbered gaps (G-001…G-030) across 16 distinct clause groups.
- **User directive (2026-05-07)**: "every word in constitution is countable... real problem from web... no manipulation".
- **Pass 2 mandate**: per gap, propose a **specific witness-closure approach + candidate source(s)**. NO test code (Pass 3). NO source modifications. NO architect ratification action.
- **Method per gap-type**:
  - **Type 1 (runtime invariant)**: WebSearch + WebFetch for adversarial corpora, multi-agent benchmarks, contamination literature. Reject candidates that look cherry-picked.
  - **Type 2 (substrate property)**: identify specific evidence file path under `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/` or `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/` whose post-state allows offline `derive_from_tape` reconstruction. Web research only when constitution clause references external standard (G-030).
  - **Type 3 (audit/policy/process)**: identify which `handover/audits/*.md` or `handover/tracer_bullets/TB_LOG.tsv` rows the parser would scan, and which property to assert on each row.
  - **Type 4 (architectural/structural)**: identify the grep target + path scope.
  - **Escalation (G-009 + G-020)**: draft a one-paragraph framing for architect / forward-bound TB.
- **Self-check applied**: every gap from Pass 1 §3 covered (G-001..G-030, 30 entries, none merged or skipped). All Type-1 gaps received at least one WebSearch + one WebFetch (or a defensible "no-canonical-corpus-found" escalation per the no-manipulation rule).
- **No-manipulation rule** in operation: candidate problems must be defensible as adversarial-without-cherrypicking. Where no clean candidate exists (e.g., no public Lean false-proof corpus survived inspection), this is reported as **meta-escalation: "Pass 3 must propose construction method that user approves"** rather than synthesizing a candidate.
- **Scope discipline**: Pass 2 is propose-only. The deliverable is the candidate list; the actual selection (and the construction-method approval for meta-escalations) is Pass 3 user input.

---

## §1 Per-gap candidate witnesses

### G-001 — Art. 0.2 §1 derivable-views conservation [Type 2]

- **Clause text**: "任意 cost / time / provenance / market price / wallet state / rejection feedback / search history / boltzmann routing / mr tick，frozen tape 上必有充分信息可推导".
- **Witness closure approach**: Evidence-derivation (offline; no new LLM compute).
- **Candidate source**: `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P08_aime_1983_p1/` — richest case (50 tx, 117 CAS objects spanning AttemptTelemetry/LeanResult/EvidenceCapsule/EvidenceManifest/CompressedRunLog/ProposalPayload/Generic per `cas/.turingos_cas_index.jsonl`). Secondary: P05 (20 attempts, 12 L4.E entries per `chain_invariant_post_fix.json`) + P07 `numbertheory_2pownm1prime_nprime` (50 attempts, capsule emitter).
- **Property to assert** (per named view in clause):
  1. `cost`: derive `Σ cost` from L4 + L4.E + CAS attempt-telemetry payloads; assert `==` whatever side-channel `RunCostAccumulator` reports (if any) for the same run.
  2. `time`: derive `wall_clock_elapsed` from min/max `created_at_logical_t` in CAS index; cross-check against `verdict_post_fix.json.tape_root` advance.
  3. `provenance`: every `proposal_record_cid` in `runtime_repo/agent_audit_trail.jsonl` resolves into `cas/.turingos_cas_index.jsonl`.
  4. `market price`: TB-14 already covers; ensure assertion lives in same harness.
  5. `wallet state`: derive from L4 typed-tx kinds (`task_open=1, escrow_lock=1, terminal_summary=1` per `verdict_post_fix.json.tx_kind_counts`); assert `==` `WalletTool` projection.
  6. `rejection feedback`: 7 rows in `runtime_repo/rejections.jsonl` must each map to an L4.E entry with the same `tx_id` prefix.
  7. `search history`: per-run search cache derivation from CAS lookup events (constitution §0.2 lists `search_cache`).
  8. `boltzmann routing`: parent_state_root distribution in rejections.jsonl — assert mode/median over agent IDs matches expected diversity (subsumes G-014 partial).
  9. `mr tick`: see G-025 (clock+mr-tick).
- **Test shape sketch**: One offline integration test per view; each view-derivation function reads `cas/`, `runtime_repo/`, and `verdict_post_fix.json`; emits an in-memory canonical struct; `assert_eq!` against the side-channel's serialized state. P08 is primary because it has all 7 CAS object types attested.
- **Pass 3 effort estimate**: M (9 sub-checks; substrate is one canonical evidence dir).
- **Open questions**: confirm sub-view #4 (market price) can co-exist with TB-14 test, or if the TB-14 invariant already subsumes it; confirm scope of #7 search_cache (current evaluator path may not maintain a separate cache).

---

### G-002 — Art. 0.2 §2 every parallel-ledger has assert_eq守恒测试 [Type 2]

- **Clause text**: "每个派生视图都必须有 `assert_eq!(view, derive_from_tape(tape))` 守恒测试".
- **Witness closure approach**: Evidence-derivation (offline).
- **Candidate source**: same TB-C0 batch; specifically the "named ledgers" enumerated in constitution.md line 64: `RunCostAccumulator`, `WalletTool`, `search_cache`, `LibrarianTool`, `bus.graveyard`, `FC trace`.
- **Property to assert**: for each enumerated parallel ledger that exists in the codebase, a one-test-per-ledger pair: (a) ledger snapshot at run-end (b) `derive_from_tape(L4 + L4.E + CAS)` reconstruction; assert `==`. Ledgers that have been removed (e.g., `bus.graveyard` per Art. 0.2 closure) are asserted absent (`grep -L`).
- **Test shape sketch**: a single test file with N sub-tests, one per named view, each loading P08 evidence and asserting derivation equality. Reuses G-001 derivation primitives.
- **Pass 3 effort estimate**: M (cumulative with G-001 — share derivation library).
- **Open questions**: which views are still alive in current code (`bus.graveyard` may be deleted per Commit-4 of Art. 0.2 atomic plan).

---

### G-003 — Art. 0.2 §3 PputResult fields all reconstructible from tape [Type 2]

- **Clause text**: "任何不能从 tape 重建的字段都不可进入 PputResult 主指标".
- **Witness closure approach**: Evidence-derivation (offline).
- **Candidate source**: every `<problem>/extracted_pput.json` in TB-C0 batch (9 problems). Example field list from P08: `tx_count`, `halt`, `solved`, `hit_max_tx`, `verified`, `tool_dist.{step, step_partial_ok, step_reject}`, `step_partial_ok` (top-level mirror).
- **Property to assert**: for each PputResult field, declare its tape-derivation source (e.g., `solved` ← `verdict.tape_root.l4_count > 0 ∧ accepted_work_predicate_results_true=Pass`; `tool_dist.step` ← count of CAS AttemptTelemetry where `outcome ∈ {LeanFail, PartialAccepted}`). Assert derived value equals reported field value on all 9 problems.
- **Test shape sketch**: extends existing matrix §M `chain_derived_facts_not_evaluator_stdout` (currently AMBER) by adding per-field provenance assertion. Reads `extracted_pput.json` + `cas/` + `verdict_post_fix.json` for each problem.
- **Pass 3 effort estimate**: M (9 problems × ~7 fields).
- **Open questions**: should `tx_count` (50 on P08) include synthetic admin scaffold (per `feedback_class4_cannot_hide_in_class3` and `OBS_TB18R_INV1_NONLLM_TX_2026-05-07.md`)? CLAUDE.md says canonical invariant uses `evaluator_reported_completed_llm_calls`, not `tx_count`. Pass 3 should respect this clarification.

---

### G-004 — Art. 0.2 §4 Phase D ArchitectAI cost attribution to golden path [Type 2]

- **Clause text**: "Phase D ArchitectAI 必须从 tape 上 attribute per-node cost/provenance 到 golden path".
- **Witness closure approach**: Evidence-derivation (forward-bound; partial probe possible now).
- **Candidate source**: any solved MiniF2F problem with multi-step proof. P03 `mathd_algebra_107` (1-shot omega solve in TB-C0; `verdict.tape_root.l4_count=1`) is a minimal case; richer multi-step golden path is needed for full clause coverage and would require a Phase D run with a multi-step omega.
- **Property to assert**: golden-path Node sequence reconstructible from L4 hash chain (`l4_parent_state_continuity`, already Pass on P08); each Node's cost (CAS AttemptTelemetry payload) attributable to that path.
- **Test shape sketch**: probe-only for now — assert "for the L4 hash chain leading to OmegaAccepted, every Node has a CAS AttemptTelemetry CID and a non-zero declared cost". Full attribution semantics await Phase D charter.
- **Pass 3 effort estimate**: S for probe; **L for full coverage (depends on Phase D charter)**.
- **Open questions**: is Phase D currently on roadmap exit? If not, mark as forward-bound and ship a probe only.

---

### G-005 — Art. 0.2 §6 WAL per-line SHA-256 hash chain unbroken [Type 2]

- **Clause text**: "WAL 必须有 per-line SHA-256 hash chain（无 hash chain → tampering 不可检测）".
- **Witness closure approach**: Evidence-derivation (offline).
- **Candidate source**: `runtime_repo/agent_audit_trail.jsonl` on any TB-C0 problem. Each line carries `prev_hash` + `hash` fields (verified by inspection: P08 line 1 `prev_hash` = all-zeros, line 2 `prev_hash` = line-1 `hash`). P07 `numbertheory_2pownm1prime_nprime` (50 attempts) is the richest WAL case for unbroken-chain verification. Additionally `cas/.turingos_cas_index.jsonl` is a parallel content-addressable WAL.
- **Property to assert**: for each line `i` in `agent_audit_trail.jsonl`, `line_i.prev_hash == sha256_chain(line_{i-1})`. Genesis (line 1): `prev_hash == [0; 32]`.
- **Test shape sketch**: pure offline parser test; loads JSONL, walks lines, verifies hash chain. Distinct from `audit_tape_tamper` (security probe) — this is the *real-load chain-resident* witness Pass 1 flagged as missing.
- **Pass 3 effort estimate**: S.
- **Open questions**: confirm WAL hash-chain semantics: is the chain already SHA-256 (per directive) or some other function of `prev_hash + payload`? Pass 3 needs to inspect `src/wal.rs` to lock the canonical hash function.

---

### G-006 — Art. 0.2 24-violation closure traceability [Type 2]

- **Clause text**: "已知违反点 (2026-04-26 双 auditor 审计; 24 处违反)".
- **Witness closure approach**: Parser-test over committed docs (no new evidence).
- **Candidate source**: `handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md` (24-row violation table) cross-referenced with `handover/tracer_bullets/TB_LOG.tsv` ship-row TB-13..TB-18R closure references.
- **Property to assert**: every of the 24 violations cited in the auditor doc has either (a) a closure row in TB_LOG.tsv with explicit violation-ID reference, OR (b) an explicit `OBS_*.md` filing the violation as forward-bound with timestamp.
- **Test shape sketch**: parser walks the 24-row table; for each row, grep TB_LOG.tsv + handover/alignment/OBS_*.md for the violation ID; assert ≥1 closure citation. No real run needed.
- **Pass 3 effort estimate**: M.
- **Open questions**: are the 24 violations enumerated by stable IDs (V-01..V-24)? If not, Pass 3 should normalize IDs first.

---

### G-007 — Art. 0.2 10-commit atomic plan row-by-row closure [Type 2]

- **Clause text**: 10-commit table rows 1-10 in constitution.md lines 80-94 closing specific violations.
- **Witness closure approach**: Parser-test over committed docs (no new evidence).
- **Candidate source**: constitution.md §0.2 10-commit table itself + `git log --grep="Commit [0-9]+:" --all` against the working repo.
- **Property to assert**: each of the 10 commits has a corresponding repo commit (matched by message tag `Commit N:` or by closing-violation reference V-XX); each closing commit references the violation IDs claimed (V-01..V-24).
- **Test shape sketch**: walk constitution.md 10-row table; for each row, run `git log --grep="<violation-ID>"` and assert ≥1 hit. Alternatively (preferred), parser scans `handover/tracer_bullets/*.md` charters for `Closes V-XX` markers.
- **Pass 3 effort estimate**: M.
- **Open questions**: same as G-006 — violation ID normalization needed first.

---

### G-008 — Art. 0.3 Node hash field semantic slot reservation [Type 2]

- **Clause text**: "Node 字段命名 + bus.append 签名必须为此预留扩展空间".
- **Witness closure approach**: Static-shape test (no real-problem needed; treat as Type-2/4 hybrid).
- **Candidate source**: `src/ledger.rs::Node` struct + `src/bus.rs::append_internal` signature.
- **Property to assert**: (a) `Node` struct has no field that would conflict with future `hash: [u8; 32]` (e.g., no existing `hash` field with different semantics); (b) `bus.append_internal` accepts non-`hash`-bearing nodes today AND its signature is `#[non_exhaustive]` or extension-friendly (compile-test by adding a stub `hash` field in a parallel test mod).
- **Test shape sketch**: a Rust source-grep + a positive compile-test that adds a `hash: [u8;32]` field to `Node` (in a test cfg) and asserts the existing `bus.append` machinery still compiles.
- **Pass 3 effort estimate**: S.
- **Open questions**: per Art. 0.4 caveat, this slot is "Path A" only; if architect chooses Path B (real git substrate), Art. 0.3 self-hash slot is moot. Pass 3 should defer this gap until G-010 (path A/B/C decision) lands, OR ship a "shape-test conditional on path choice" — Pass 3 user input needed.

---

### G-009 — Art. 0.4 Q_t version-control triple — HEAD_t completely unimplemented [ESCALATION]

- **Clause text**: "`HEAD_t` 完全未实现 (runtime 0 处 path pointer 概念)".
- **Witness closure approach**: **ESCALATION — architect-level Path A/B/C decision required first; no Pass-3 test can land without that decision**.
- **Architect-decision framing (1 paragraph)**:
  > Constitution §0.4 explicitly states the runtime grep `Repository::|git2::|libgit2|Command git` returns 0 hits and that `HEAD_t` is completely unimplemented. The constitution lays out three paths: **A** (semantic version: keep `Vec<Node>`, add `hash:[u8;32]` + `HEAD_t: NodeId` last-accepted pointer + explicit triple signatures; ~3 weeks), **B** (real git substrate: libgit2/git2-rs, runtime per-cell git repo, Node = git commit, HEAD_t = git HEAD ref; ~6-8 weeks), or **C** (hybrid: A now, B at Phase E gate; ~3 weeks now + 5 weeks later). Phase E gate **forces B** unless human sudo amends the fidelity requirement. Until the architect commits to a path, no test for HEAD_t can be defensibly authored — a Path-A test would specify `NodeId` semantics that Path-B would discard, and vice versa. **Decision needed**: which path? **Implication for Pass 3**: if Path A or C, ship the rtool/wtool triple-signature shape-test + a `HEAD_t: NodeId` invariant test on TB-C0 evidence. If Path B, ship git-substrate-presence smoke + commit-as-Node integration test (substantial scope, deserves its own TB charter).
- **Pass 3 effort estimate**: blocked. **DO NOT proceed to Pass 3 for G-009 without architect Path A/B/C ratification.**
- **Open questions**: this is the load-bearing escalation; user must obtain architect §8 sign-off on path choice before Pass 3 begins.

---

### G-010 — Art. 0.4 Path A/B/C decision must land at next architecture commit [Type 4]

- **Clause text**: "下次架构 commit 必须明文标注采用 A/B/C 中哪条路径; Phase E gate 强制 B".
- **Witness closure approach**: Parser-test over `constitution.md` §V.3 amendment log + git history for the most recent post-Art.0.4 architecture commit.
- **Candidate source**: existing `tests/constitution_art_v3_amendment_log.rs` provides scaffolding; extend to assert that any §V.3 amendment row tagged `Art. 0.4` carries a path declaration (`Path A`, `Path B`, or `Path C`), OR assert that the last `architecture:`-tagged commit message has the declaration.
- **Property to assert**: every §V.3 row touching Art. 0.4 has a `Path: {A|B|C}` field; equivalently, the latest `git log -i --grep='architecture:'` commit subject contains `Path-{A|B|C}`. Until the decision lands, the test correctly **fails-by-design** as a forcing-function gate.
- **Test shape sketch**: regex over §V.3 table cells + git log post-2026-04-26.
- **Pass 3 effort estimate**: S.
- **Open questions**: Pass 3 may need to defer this gap subordinate to G-009 — once the architect declares the path, the §V.3 log will be amended and the test will start passing. Wire the test now even if it RED today.

---

### G-011 — Art. I.1 hard-vs-soft constraint exclusivity [Type 4]

- **Clause text**: "顶层白盒不能依赖语言 (另一个黑盒) 去约束黑盒，而必须把约束转化为机器可执行的硬约束".
- **Witness closure approach**: Static-shape test (predicate enumeration) + selective web confirmation of LLM-as-Judge anti-pattern.
- **Candidate source(s)**:
  - **Codebase**: `src/sdk/predicate.rs::Predicate` enum/registry; `src/bus.rs::forbidden_patterns`. Enumerate every variant; for each, assert there's a pure-code enforcement path (no LLM call, no natural-language prompt).
  - **Web (anti-pattern reference)**: search results below confirm "LLM-as-judge" is a known failure mode:
    - https://arxiv.org/html/2504.14422v1 — discusses verifier-as-reward-function, treats LLM-judge as soft constraint
    - https://lilianweng.github.io/posts/2024-11-28-reward-hacking/ — Goodhart-style failures of LLM-as-judge
- **Property to assert**: for each predicate kind in `Predicate` enum, the enforcement function `fn check(...) -> VerifyVerdict` does NOT call any LLM API; assertion is a structural code-path grep. Optional belt-and-suspenders: a runtime no-LLM-in-predicate-stack assertion using a tracing fixture.
- **Test shape sketch**: enumerate predicate variants; grep enforcement source for `reqwest::|http::|llm_client|chat_completion`; assert empty.
- **Pass 3 effort estimate**: S.
- **Open questions**: scope — does "predicate" include sequencer admission gates? If so, that surface (`src/state/sequencer.rs`) is also in-scope.

---

### G-012 — Art. I.1.1 PCP soundness statistical floor [Type 1, HIGH PRIORITY]

- **Clause text**: "如果候选解是错误的，谓词不必做到全知全能地识别所有错误，但必须以极高概率拒绝".
- **Witness closure approach**: Web-research + adversarial corpus injection.
- **Candidate source(s)** (web research log §4 #1, #2, #6):
  1. **MiniF2F-v2 misalignment audit**: per https://arxiv.org/html/2511.03108v1 ("miniF2F-Lean Revisited"), >50% of formal statements in miniF2F-v1 were misaligned with their informal counterparts. The **misaligned statements** form a defensible adversarial corpus: each is a "wrong" theorem that an LLM might believe is correct but a sound verifier should not accept as proof of the intended informal claim. **Source URL**: https://arxiv.org/html/2511.03108v1 (open arXiv preprint). License: arXiv standard.
  2. **DafnyBench unsoundness corpus**: per https://arxiv.org/html/2512.10187v2 (MiniF2F-Dafny, POPL 2026), DafnyBench-2024 weak validators allowed `:axiom` attribute / `assume` statements / strengthened-precondition / weakened-postcondition exploits. Each exploit is a known false-but-passes proof. **Source URL**: https://arxiv.org/html/2512.10187v2. License: arXiv standard.
  3. **ProofNet++ "common structural and semantic flaws"**: per https://arxiv.org/abs/2505.24230, ProofNet++ tested its self-correction module against a corpus of >120,000 normalized formal proofs with structural flaws. Codebase release referenced; full corpus URL not in abstract.
  4. **mCoq (mutation testing for Coq)**: per https://users.ece.utexas.edu/~gligoric/papers/CelikETAL19mCoq.pdf, MutantChick generates type-checked mutant proofs in Coq. Direct Lean equivalent appears not to exist — Pass 3 may need to port a small subset.
- **Adversarial framing (defensible-without-cherrypicking)**: pick the misalignment-fix list from miniF2F-v2 (per (1) above). The misalignment was discovered by external auditors, not curated for this test. The corpus is "real adversarial without us choosing it".
- **Test shape sketch**: inject N (≥30) misaligned-but-syntactically-valid theorem-proof pairs into the predicate gate; assert ≥99% rejection rate (Wilson 95% CI lower-bound > 0.95). Alternative: sorry-padded proofs (every miniF2F problem with `sorry` swapped for the goal type).
- **Pass 3 effort estimate**: L (corpus prep + injection harness + statistical assertion).
- **Open questions**:
  1. License + redistribution for miniF2F-v2 misalignment list — confirm with user before downloading.
  2. Statistical floor: the constitution says "极高概率" (extremely high). Is 99% the right Wilson lower-bound, or 99.9%? User input needed.
  3. **Is there value in a curated synthetic corpus too?** If the natural corpora prove insufficient (e.g., too few examples per failure class), Pass 3 may need user approval to **construct a small synthetic adversarial corpus** built systematically by Lean tactic mutation. **Meta-escalation: user approval needed before constructing.**

---

### G-013 — Art. I.2 Report standard executable enforcement [Type 3]

- **Clause text** (composite from CLAUDE.md Report Standard + Art. I.2): "每报必填: ΣPPUT + Mean PPUT (solved) + 95% CI (Wilson)".
- **Witness closure approach**: Parser-test over `handover/tracer_bullets/TB-*_charter_*.md` and `handover/audits/*.md` ship-reports.
- **Candidate source**: every ship-style report in `handover/tracer_bullets/` and `handover/audits/`. Plus current `handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/PHASE_3_CANDIDATE_REPORT_v3.md`.
- **Property to assert**: every report file under those dirs whose filename matches `*_SHIP_*` or `*_CANDIDATE_REPORT_*` or `TB-*_charter_*.md` contains regex matches for: (a) `ΣPPUT`, (b) `Mean PPUT`, (c) `95% CI` AND `Wilson`, (d) `halt_reason_distribution`, (e) `attempt_count_equality_report`, (f) for n≥2 runs additionally `parent_selection_entropy` + `pairwise_payload_diversity_mean`.
- **Test shape sketch**: parser test reading these dirs; per file, assert regex hits; report missing fields with file path + line.
- **Pass 3 effort estimate**: M (need to handle deprecated/superseded files — likely via a `DEPRECATED:` front-matter marker).
- **Open questions**: which TB ship-reports pre-date this rule and should be grandfathered? Suggest cutoff date = 2026-05-06 (TB-C0 SHIPPED FINAL); pre-cutoff files in `handover/audits/` are exempt.

---

### G-014 — Art. I.2 consensus extraction = mode/median (not LLM-judged) [Type 1]

- **Clause text**: "通过计算众数或中位数，机械地剥离极端的'幻觉'偏差".
- **Witness closure approach**: Evidence-derivation on TB-C0 multi-agent batch (n=5 agents).
- **Candidate source**: `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P*/runtime_repo/agent_audit_trail.jsonl` — each problem has 5 agent outputs; consensus extraction (if computed) should be observable. P05 has both 12 L4.E + 8 capsule-anchored attempts spread over 5 agents (per `chain_invariant_post_fix.json`).
- **Property to assert**: where the runtime computes a consensus signal across n≥2 agents (e.g., mode of `solved` flag, median of `tx_count`), the extraction function is `mode` or `median` (pure code), not an LLM call. Verify via source-grep + a runtime trace assertion.
- **Test shape sketch**: enumerate `consensus_*` symbols in `src/`; assert no LLM client imports in their call-graph; on the n=5 TB-C0 batch, replay the consensus output and verify it matches the canonical mode/median computed offline.
- **Pass 3 effort estimate**: M.
- **Open questions**: does the current code path even compute a consensus signal? If not (n=5 was multi-agent admission, not consensus), this gap is a forward-bound gardener-style requirement; mark as such.

---

### G-015 — Art. I.2 utility scoring shape (期望 / 方差) [Type 2]

- **Clause text**: "用严谨的数学公式 (例如求平均、求方差) 计算一份'体检报告'".
- **Witness closure approach**: Extension of G-013 parser test (no new evidence).
- **Candidate source**: same as G-013 — TB ship reports.
- **Property to assert**: in addition to `ΣPPUT` + `Mean PPUT` + `95% CI`, every report carries `σ(PPUT)` (or `var(PPUT)`, or `stddev`) for the solved-problem set.
- **Test shape sketch**: extend G-013 parser regex set with `σ\\(PPUT\\)|stddev|variance|var\\(`.
- **Pass 3 effort estimate**: S.
- **Open questions**: confirm with user that σ(PPUT) is a required field (CLAUDE.md Report Standard does not currently mandate it).

---

### G-016 — Art. II.1 broadcast typical errors — chain-resident no-leak test [Type 1]

- **Clause text**: "顶层白盒绝不能把具体报错日志群发给所有人".
- **Witness closure approach**: Evidence-derivation on chain-resident agent prompts.
- **Candidate source**: `handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/P*/runtime_repo/agent_audit_trail.jsonl` — every agent prompt sent during the run. P07 (50 attempts, all rejection-rich) is the high-load case where if any leakage existed it would manifest. P08 (50 tx, includes capsule-emit path with rich error context) is the second probe.
- **Property to assert**: for each line in `agent_audit_trail.jsonl`, the prompt body contains zero matches for raw-stderr indicators: regex `error: type mismatch|unknown identifier|stack overflow|/tmp/[a-z0-9]+\.lean:[0-9]+:[0-9]+: error|(?i)compiler error|unsolved goals` etc. (Lean stderr signature regex set, mined from real Lean compiler stderr).
- **Test shape sketch**: load all 9 problems' agent_audit_trail.jsonl; per line, regex-scan the prompt; emit any hit with file/line/agent. Zero tolerance.
- **Pass 3 effort estimate**: M (regex curation + 9-problem scan).
- **Open questions**: agent_audit_trail.jsonl in TB-C0 schema appears to be tx-level not prompt-level (per inspection: 2 lines in P08, both tx records, no prompt body field). **This is a critical question for Pass 3: where do agent prompts persist on tape?** If they don't, this gap requires schema work first (Class 4) and is forward-bound. **Meta-escalation: confirm prompt persistence before Pass 3.**

---

### G-017 — Art. II.1 typical-error → globalized rule pipeline [Type 3]

- **Clause text**: "将这类典型错误抽象出来 → 更新全局架构文档 → 再把抽象后的规则广播给所有 Agent".
- **Witness closure approach**: Parser-test over `~/.claude/projects/.../memory/feedback_*.md` history vs. TB ship reports.
- **Candidate source**: every TB-N ship report in `handover/tracer_bullets/` cross-referenced with `~/.claude/projects/-home-zephryj-projects-turingosv4/memory/feedback_*.md` creation timestamps (filesystem mtime fallback if no frontmatter).
- **Property to assert**: every TB ship that documented a "typical error / lesson" must have at least one corresponding `feedback_*.md` written within 7 days of ship date (mechanism per `feedback_norm_needs_mechanism`).
- **Test shape sketch**: parse TB ship reports for "lesson:" / "教训:" / "OBS-" sections; for each lesson, assert ≥1 `feedback_*.md` exists with the same topic keyword; report unmatched lessons.
- **Pass 3 effort estimate**: M.
- **Open questions**: what is the canonical "lesson section" marker in TB ship reports? Pass 3 needs schema; existing reports vary.

---

### G-018 — Art. II.2.1 explore/exploit symmetric stress test [Type 1]

- **Clause text**: "探索-利用 平衡; 过度利用 = 同质化, 过度探索 = 信号失效".
- **Witness closure approach**: Web-research (multi-agent RL bandit baselines) + dual-baseline injection.
- **Candidate source(s)** (web research log §4 #3):
  1. **Boltzmann exploration baselines**: https://arxiv.org/abs/1901.08708 ("Almost Boltzmann Exploration") — provides canonical "all-greedy" (T→0) + "all-uniform" (T→∞) extreme settings. Inject these as control conditions; observe diversity collapse.
  2. **Multi-agent diversity collapse benchmarks**: https://proceedings.neurips.cc/paper/2021/file/dd1970fb03877a235d530476eb727dab-Paper.pdf ("Exploration-Exploitation in Multi-Agent Competition") — provides canonical "diversity collapse" test scenarios.
- **Adversarial framing**: run TB-C0-style n=5 multi-agent batch with `BOLTZMANN_TEMPERATURE=0.01` (over-exploit) and another with `BOLTZMANN_TEMPERATURE=100.0` (over-explore); assert that `axiom_2_payload_diversity` actually fires below floor in the first case and that `parent_selection_entropy` falls below threshold in the second. Real-load symmetric stress test.
- **Test shape sketch**: env-var-driven mini-batch (3 problems, n=5); two runs with extreme temperatures; assert the gates fire correctly.
- **Pass 3 effort estimate**: L (requires real LLM compute — TB-style mini-run; not offline).
- **Open questions**:
  1. Does the codebase expose a `BOLTZMANN_TEMPERATURE` knob? If not, Pass 3 needs to introduce one (Class 3).
  2. User input on LLM budget: 6 problem-runs (3 problems × 2 conditions × n=5) — is this acceptable for harness verification?

---

### G-019 — Art. III.1 in-context learning of bad pattern [Type 1]

- **Clause text**: "一个坏模式一旦污染上下文，就会被后续所有 Agent 当作'正确示例'学习".
- **Witness closure approach**: Evidence-derivation on multi-cycle runs + selective web confirmation.
- **Candidate source(s)**:
  - **Evidence**: P07 `numbertheory_2pownm1prime_nprime` (50 attempts, capsule emitter, multiple rejection cycles). The agent_audit_trail.jsonl across 50 cycles is the substrate.
  - **Web**: https://openreview.net/forum?id=YlpaaYxx4t (CoDeC contamination detection via in-context learning) — provides methodology framing for "in-context contamination" detection.
- **Property to assert**: for each agent prompt P_n at cycle n, no field in P_n contains the verbatim rejected-proof bytes from cycles 1..n-1. (The "rejection feedback" should be abstracted, not raw-injected.)
- **Test shape sketch**: load P07 agent_audit_trail.jsonl across all 50 cycles; for each prompt, byte-substring search against the union of all prior cycles' rejected proposal bytes; zero tolerance.
- **Pass 3 effort estimate**: M.
- **Open questions**: Same as G-016 — confirm prompts persist on tape. If not, Class-4 schema work first.

---

### G-020 — Art. III.1 background gardener Agent (GC of stale code/docs) [ESCALATION]

- **Clause text**: "部署后台'园丁 Agent'，定期扫描并屏蔽偏离黄金原则的陈旧代码与过期文档".
- **Witness closure approach**: **ESCALATION — forward-bound TB charter required; NO Pass-3 test possible until gardener Agent role is implemented.**
- **Forward-bound TB framing (1 paragraph)**:
  > Constitution §III.1 mandates a "gardener Agent" (园丁 Agent) for ongoing GC of stale code and expired docs. No such Agent role exists in current `src/runtime/`. The constitution uses "必须" (mandatory). This is implementation-pending, not test-pending: any test we write today would be a stub asserting an absent feature, which yields negative information. **Pass 3 should NOT attempt to ship a test for G-020.** Instead, surface as a forward TB charter request: TB-Gardener-1 (proposed scope: define Agent role / cadence / scan targets / shielding mechanism per §III.1; tape-resident witness via gardener-sweep capsules emitted to CAS). Decision needed: prioritize TB-Gardener-1 in roadmap (after FREEZE LIFTED 2026-05-07 per LATEST.md).
- **Pass 3 effort estimate**: blocked. **DO NOT proceed to Pass 3 for G-020.**
- **Open questions**: gardener Agent should be on roadmap; user input needed on placement (P3+ vs M1/M2).

---

### G-021 — Art. III.2 progressive disclosure / agent prompt size budget [Type 2]

- **Clause text**: "Agent 按需加载特定文档 → 上下文不被无关信息污染".
- **Witness closure approach**: Evidence-derivation (offline; presumes prompt persistence — see G-016 caveat).
- **Candidate source**: TB-C0 batch agent prompts (if they persist on tape; see G-016 open question). If not on tape, the evaluator's `build_agent_prompt` source path is the structural fallback.
- **Property to assert**: every agent prompt token-count ≤ declared budget (e.g., 8K tokens, configurable per problem class). Use `tiktoken` or equivalent for token counting.
- **Test shape sketch**: load all prompts; count tokens; assert ≤ budget. Report distribution.
- **Pass 3 effort estimate**: M.
- **Open questions**: same prompt-persistence dependency as G-016. Pass 3 must clarify before proceeding.

---

### G-022 — Art. III.3 horizontal context-isolation (independence on real n≥2) [Type 1]

- **Clause text**: "如果所有黑盒共享完全相同的实时上下文和中间状态，那么它们的输出会高度相关".
- **Witness closure approach**: Evidence-derivation (TB-C0 n=5 batch) + selective web framing.
- **Candidate source(s)**:
  - **Evidence**: TB-C0 batch is n=5 multi-agent; each agent's `agent_pubkeys.json` resolves to a distinct ed25519 pubkey. agent_audit_trail.jsonl carries `tx_id` patterns including agent identifier suffixes.
  - **Web**: https://arxiv.org/html/2503.03800v1 (Multi-Agent Systems Powered by LLMs: Swarm Intelligence) — framing for horizontal-isolation-vs-shared-state pattern catalog.
- **Property to assert**: across the 5 agents on the same problem, no two agents share an in-memory mutable state object (verified by source-grep on the runner orchestration: each agent gets its own `RunCostAccumulator`, `WalletTool`, etc. instance). Plus: across agents, output diversity is `pairwise_payload_diversity_mean ≥ 0.25`.
- **Test shape sketch**: combination of source-grep (no shared `static mut` / no shared `Arc<Mutex<...>>` for state-of-truth ledgers) + offline evidence assertion (load 5 agents' outputs from TB-C0 P05 / P07 / P08; compute pairwise edit-distance / token-set Jaccard; assert ≥ 0.25 floor).
- **Pass 3 effort estimate**: M.
- **Open questions**: confirm 0.25 floor is the architect-declared threshold (CLAUDE.md report standard says `< 0.25 = warning`; gate could be the same or stricter).

---

### G-023 — Art. III.4 scoring-formula leakage [Type 2]

- **Clause text**: "黑盒只能通过持续试错来感受错误信息，而不能把度量函数本身作为优化捷径".
- **Witness closure approach**: Static-shape source-grep + offline prompt scan (no new evidence).
- **Candidate source**: `src/runtime/evaluator.rs` (PPUT formula) + `src/economy/reputation.rs` (reputation accumulator) — extract formula constants; cross-grep against `build_agent_prompt` source + (if persistent) every agent prompt in TB-C0 batch.
- **Property to assert**: the numeric coefficients of PPUT formula (e.g., constants 1, 0, weights) do NOT appear in any agent prompt as literal numbers at the same position/role. Plus: no symbol named `compute_pput`, `score_formula`, `weight_*` is referenced in prompt builder.
- **Test shape sketch**: grep + regex; offline.
- **Pass 3 effort estimate**: S.
- **Open questions**: G-023 partially overlaps with G-016 (raw stderr leak); Pass 3 should consolidate into a single "shielding gate" extension if scope similar. Also: web check on Goodhart benchmark (see web research log §4 #6) — useful framing reference https://lilianweng.github.io/posts/2024-11-28-reward-hacking/.

---

### G-024 — Art. IV initialization is one-shot [Type 4]

- **Clause text**: "这一步只发生一次 …… 一旦系统被'拉起来'，它就会在既定规则下自行运行".
- **Witness closure approach**: Static-shape test enumerating mutation surfaces.
- **Candidate source**: `src/runtime/evaluator.rs::run_swarm` + `src/state/sequencer.rs::genesis` + `src/sdk/predicate.rs::PredicateRegistry`.
- **Property to assert**: (a) `PredicateRegistry` exposes no `add` / `remove` / `replace` / `mutate` API after `genesis()` — already partial in `tests/constitution_fc3_meta.rs`; (b) the `bus.append` path is the only post-init mutation surface for `Q_t`; (c) no `static mut` in `src/runtime/` writeable post-genesis; (d) `EconomicState` has no post-init mint path (`fc2_on_init_only_mint` covers this — extend to assert no `fn add_*` / `fn extend_*` / `fn override_*` on the registry).
- **Test shape sketch**: enumeration test over `src/sdk/predicate.rs` + `src/runtime/evaluator.rs` symbols.
- **Pass 3 effort estimate**: S.
- **Open questions**: which "registries" are in-scope? Pass 3 needs an explicit list (Predicate registry, Tool registry, Q_0 schema, Agent registry).

---

### G-025 — Art. IV clock+mr-tick on long-run real problem [Type 1]

- **Clause text**: clock advance → mr-tick (FC2-N20).
- **Witness closure approach**: Real-load run (modest cost) + selective evidence-derivation.
- **Candidate source(s)**:
  - **Evidence (existing)**: P07 `numbertheory_2pownm1prime_nprime` and P08 `aime_1983_p1` already exhausted at `tx_count=50` per `extracted_pput.json`. Confirm `TICK_INTERVAL` setting; if `TICK_INTERVAL ≤ 50`, mr-tick should already be observable in the existing evidence.
  - **Web (forward)**: https://arxiv.org/pdf/2407.11214 (PutnamBench) provides longer-form proofs as substrate if MiniF2F p38/p49 prove insufficient; PutnamBench Lean 4 has 672 problems including very long proofs.
- **Property to assert**: in P07/P08 evidence, locate mr-tick events on tape (CAS `MarketResolve` / sequencer tick events / or a dedicated `MrTickTx`); assert ≥1 such event with a logical timestamp `> TICK_INTERVAL`.
- **Test shape sketch**: parse P07/P08 `verdict_post_fix.json` + CAS index; locate mr-tick markers; assert presence.
- **Pass 3 effort estimate**: M (existing evidence may suffice; if not, modest Putnam mini-run needed).
- **Open questions**: what is the canonical mr-tick event signature on tape? CLAUDE.md doesn't specify. Pass 3 needs to inspect `src/bus.rs::clock` to lock the marker.

---

### G-026 — Art. V.1.1 三段守护 — Veto-AI middle layer executable check [Type 3]

- **Clause text**: "sudo + Veto-AI + Boot manifest 三段守护结构".
- **Witness closure approach**: Parser-test over `handover/audits/*.md` audit verdict files.
- **Candidate source**: `handover/audits/CODEX_*.md` and `handover/audits/GEMINI_*.md` files — verified existence per Pass-2 inspection: 47 Codex files, 30+ Gemini files, plus DUAL_AUDIT_*_VERDICT_*.md series. Confirmed sample format: `CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md` line 4 — `**Aggregate verdict: VETO until further work.**`; per-question table shows `**PASS** | **CHALLENGE** | **VETO**` verdict tokens. v2 line ?: `Verdict ranking applied: VETO > CHALLENGE > PASS.`
- **Property to assert** (per audit file):
  1. The aggregate verdict line matches one of `{PASS, CHALLENGE, VETO}` (output domain narrowness).
  2. Every per-question / per-finding verdict row matches one of `{PASS, CHALLENGE, VETO}`.
  3. No verdict line includes subjective-quality tokens like "code style" / "performance" / "perf" / "readability" / "test coverage" without ALSO citing a constitution clause.
  4. Conservative ranking is honored: if any sub-row is VETO, aggregate is VETO; if any sub-row is CHALLENGE (and none VETO), aggregate is at most CHALLENGE.
- **Test shape sketch**: parser walks `handover/audits/CODEX_*.md` + `handover/audits/GEMINI_*.md` + DUAL_AUDIT files; per file, regex-extract verdict lines; assert (1)-(4).
- **Pass 3 effort estimate**: M.
- **Open questions**:
  1. Pre-2026-05-06 audits: grandfather or apply retroactively? Suggest grandfather pre-TB-C0 SHIPPED (2026-05-07).
  2. Some audit files are dispatch / prompt files (not verdict) — exclude by filename suffix pattern (`_AUDIT_DISPATCH_` etc. → skip).

---

### G-027 — Art. V.1.2 ArchitectAI commit authority — executable enforcement [Type 3]

- **Clause text**: "ArchitectAI 拥有架构升级的 commit 权限 …… 经 Veto-AI 校核未发现违宪后由 ArchitectAI 直接落盘".
- **Witness closure approach**: Parser-test over git history + `handover/directives/*.md` + `handover/audits/*.md`.
- **Candidate source**: every commit in `git log` since 2026-04-25 (Art. V.1.2 amendment date); for each "architecture:"-tagged commit, assert there exists a `handover/audits/*VERDICT*` file with `PASS` (or aggregate-PASS-with-non-VETO-rank) dated ≤ commit date.
- **Property to assert**: every architecture-class commit (matching `architecture:|TB-[0-9]+ ship|FINAL ship`) is preceded by at least one Codex audit AND one Gemini audit AND aggregate verdict ≠ VETO.
- **Test shape sketch**: parser walks git log; per commit, find candidate verdict files by date proximity + TB-tag; assert ≥1 PASS/non-VETO verdict per audit camp (Codex + Gemini).
- **Pass 3 effort estimate**: L (need git2-rs + audit-file index; nontrivial cross-reference).
- **Open questions**:
  1. Definition of "architecture commit" — by file-touch (touches `src/state/typed_tx.rs`, `src/bottom_white/cas/schema.rs`, etc.) or by message tag? Pass 3 user input needed.
  2. Pre-2026-04-25 commits grandfathered? Yes — clause amendment date is the natural cutoff.

---

### G-028 — Art. V.1.3 Veto-AI output domain {PASS, VETO} (no quality/perf judgments) [Type 3]

- **Clause text**: "白名单严格排除: 主观质量评价 / 性能 / 测试覆盖率主观打分".
- **Witness closure approach**: Same audit parser as G-026 — extended assertion.
- **Candidate source**: `handover/audits/*.md` (Codex + Gemini verdicts).
- **Property to assert**: for every non-PASS verdict line (i.e., every CHALLENGE / VETO), the file contains at least one constitution clause citation pattern: regex `Art\\. ?(0|I|II|III|IV|V)(\\.[0-9]+)*|FC[123]-(N|INV)[0-9]+|C-[0-9]{3}` within ≤30 lines of the verdict line. Verdicts with NO clause citation = clause-violation (the audit overstepped its narrow scope).
- **Test shape sketch**: parser shares scaffolding with G-026; per non-PASS verdict, scan ±30 lines for clause-citation regex; report file:line of un-cited verdicts.
- **Pass 3 effort estimate**: M (shares effort with G-026).
- **Open questions**: 30-line proximity is heuristic; user may prefer "in same §" semantic boundary. Pass 3 to confirm.

---

### G-029 — Art. V.1.3 JudgeAI → Veto-AI rename — residual symbol grep [Type 4]

- **Clause text**: "JudgeAI 重命名为 Veto-AI" (2026-04-25 amendment).
- **Witness closure approach**: Static-shape grep test.
- **Candidate source**: `src/`, `experiments/`, and `tests/` — Pass-2 inspection found ONE residual: `/home/zephryj/projects/turingosv4/src/runtime/autopsy_capsule.rs` line ~`/// SG-15.8); routing is ArchitectAI proposal → JudgeAI/VetoAI →`. This is in a doc-comment, transitional during rename; needs cleanup.
- **Property to assert**: zero hits for symbol name `JudgeAI` (case-sensitive, word-boundary) in `src/` or `tests/` source files (excluding case-files `cases/C-072_veto_ai_scope_narrowing.yaml` historical record, and `handover/` audit history). Doc-comment transitional `JudgeAI/VetoAI` allowed only if also annotated `// historical alias`.
- **Test shape sketch**: shell-style grep test in Rust: `find src/ tests/ -name '*.rs' -print0 | xargs -0 grep -nE '\\bJudgeAI\\b'`; assert empty (or only the explicit-historical-alias line).
- **Pass 3 effort estimate**: S.
- **Open questions**: should the test FAIL today (1 hit in autopsy_capsule.rs) and pass after a one-line edit? Suggested: yes — Pass 3 ships test + edit in same atom.

---

### G-030 — Art. V.2 reversibility constraint (Q_{t-1} rollback) [Type 2]

- **Clause text**: "任何状态变更必须具有可逆性 (总是能够回滚到 Q_{t-1})".
- **Witness closure approach**: Evidence-derivation + minor web framing on event-sourcing compensating-transactions pattern.
- **Candidate source(s)**:
  - **Web (framing)**: https://learn.microsoft.com/en-us/azure/architecture/patterns/compensating-transaction — canonical event-sourcing reversibility pattern; https://martinfowler.com/eaaDev/EventSourcing.html — the original Fowler treatment. These ground the test design (compensating events instead of mutation).
  - **Evidence**: TB-C0 batch L4 tape per problem; `verdict.tape_root.head_state_root_hex` records advance.
- **Property to assert**: for every typed-tx kind in `src/state/typed_tx.rs` (count: from inspection — `WorkTx, VerifyTx, ChallengeTx, ReuseTx, TaskOpenTx, EscrowLockTx, CompleteSetMintTx, CompleteSetRedeemTx, MarketSeedTx, FinalizeRewardTx, ChallengeResolveTx, TerminalSummaryTx, TaskExpireTx, TaskBankruptcyTx`), declare its inverse / compensating kind (or mark `irreversible-by-design` with a constitution-citation justification). Build a registry `INVERSE_TX_KIND: HashMap<TxKind, Option<TxKind>>` and assert every kind has a populated entry.
- **Test shape sketch**: enumerate TxKind variants from `src/state/typed_tx.rs`; assert each has either (a) a registered inverse kind whose application restores Q_{t-1}, OR (b) explicit `IRREVERSIBLE_BY_CONSTITUTION` annotation with a clause-citation comment.
- **Pass 3 effort estimate**: M.
- **Open questions**: per architect, is reversibility actually required for **every** kind? `TerminalSummaryTx` may be canonically irreversible (it marks halt). User input needed on the canonical irreversibility list (vs. `WorkTx` rollback).

---

## §2 Aggregate plan

### Counts (internally consistent with §1)

| Category | Count | Gap IDs |
|---|---|---|
| Gaps closable in Pass 3 without further user input (Type 4 static-shape + simple parser) | 4 | G-008, G-024, G-029, G-007 |
| Gaps closable in Pass 3 with minor user clarification on threshold / scope | 11 | G-001, G-002, G-003, G-005, G-006, G-013, G-015, G-022, G-023, G-025, G-030 |
| Gaps closable in Pass 3 but require offline derivation library to be built first | 3 | G-014, G-017, G-027 |
| Gaps requiring real-LLM mini-run before Pass 3 lands | 1 | G-018 (Boltzmann temperature stress; ≤6 problem-runs) |
| Gaps requiring confirmation of agent-prompt persistence schema (Class-4 dependency) | 4 | G-016, G-019, G-021, G-028 (G-028 cross-cuts) |
| Gaps requiring web-corpus license / construction-method approval (meta-escalation) | 1 | G-012 (PCP soundness adversarial corpus) |
| Gaps requiring path A/B/C architect decision FIRST | 2 | G-009, G-010 |
| Gaps deferred to forward TB charter | 2 | G-004 (Phase D), G-020 (gardener Agent) |
| Gaps with executable-test-FAIL today (forcing-function gates) | 2 | G-010 (until path declared), G-029 (until autopsy_capsule.rs edited) |

**Total**: 4 + 11 + 3 + 1 + 4 + 1 + 2 + 2 = 28. Gaps with executable-test-FAIL (2) overlap with G-010 + G-029 already counted above. Internal consistency check: the 30 Pass-1 gaps are all surfaced (G-001..G-030).

Note: G-011 is in "no further user input" once predicate enum is enumerated; it's small. G-026 + G-028 share scaffolding so are counted under "minor user clarification".

### Architect ratification needed

- **G-009**: Path A/B/C decision (BLOCKING for HEAD_t). User must obtain architect §8 sign-off before Pass 3.
- **G-020**: TB-Gardener-1 charter prioritization (forward-bound; Pass 3 cannot land a test).

### Forward-bound TB charters

- **G-004**: Phase D ArchitectAI cost attribution — partial probe possible now; full coverage awaits Phase D charter.
- **G-020**: TB-Gardener-1.

---

## §3 Pass 3 ordering recommendation

Suggested sequence (lowest-risk → highest-risk; aligns with `feedback_real_problems_not_designed` "real first" rule):

1. **Wave 1 — Static-shape (Type 4) tests** (lowest risk, no real-load, no user clarification): G-008, G-024, G-029 first; then G-011 + G-023 (small enumeration scans). Gates the JudgeAI rename (G-029 will FAIL today and forces a one-line edit). Effort: ~1 day.

2. **Wave 2 — Parser tests over committed docs** (Type 3): G-026 + G-028 (shared scaffolding); G-013 + G-015 (shared regex set); G-006 + G-007 (shared 24-violation + 10-commit substrate); G-017 (TB↔feedback cross-ref); G-027 (git history × audit camp). Effort: ~3-5 days.

3. **Wave 3 — Offline evidence-derivation (Type 2)**: G-005 (WAL hash chain — simplest); G-001 + G-002 + G-003 (TB-C0 derivation library; share infrastructure); G-022 (n=5 diversity); G-030 (typed-tx inverse registry). Effort: ~5-7 days. **Defer G-014 + G-021** until G-016 prompt-persistence question resolved.

4. **Wave 4 — Real-load runtime invariants (Type 1)**: G-025 (mr-tick — likely uses existing P07/P08 evidence, no new run); then G-019 (in-context contamination) IF prompt persistence resolved; G-018 (Boltzmann stress, requires LLM mini-run). Effort: ~3-5 days + LLM budget.

5. **Wave 5 — Web-corpus injection (Type 1)**: G-012 PCP soundness — requires user approval of corpus selection / construction method first. Effort: ~5-10 days after approval.

6. **Wave 6 — Blocked / forward-bound**: G-004 (probe only), G-009 + G-010 (architect path decision), G-020 (TB charter). DO NOT land in Pass 3 without prerequisite resolution.

**Rationale for this ordering**:
- Static-shape tests are zero-risk and clean up surface (G-029 in particular).
- Parser tests over committed docs add no LLM compute and immediately formalize audit/report discipline.
- Offline derivation establishes the substrate library that future Type-1 tests can reuse.
- Real-load LAST because it's the most expensive and most likely to surface schema gaps that would derail earlier waves.

---

## §4 Web-research log

Compact log of all WebSearches + WebFetches performed in Pass 2, in chronological order.

| # | Query / URL | What was looked for | Usable candidate produced? | Used in gap(s) |
|---|---|---|---|---|
| 1 | WebSearch: "Lean 4 false proof corpus adversarial benchmark sorry-free verification 2024 2025" | Lean adversarial corpora for PCP soundness | Yes — VeriBench (2025), Clever (May 2025 sorry-free) | G-012 |
| 2 | WebSearch: "ProofNet false proof rejection benchmark formal verification soundness" | Soundness rejection benchmark | Yes — ProofNet++ (2025), 120k normalized formal proofs corpus referenced | G-012 |
| 3 | WebSearch: "multi-agent reinforcement learning exploration exploitation diversity collapse benchmark Boltzmann" | Multi-agent diversity-collapse baselines | Yes — Almost-Boltzmann arXiv:1901.08708; NeurIPS 2021 exploration-exploitation MARL competition | G-018 |
| 4 | WebSearch: "in-context learning prompt contamination bad pattern propagation LLM agent benchmark" | In-context contamination methodology | Yes — CoDeC (OpenReview YlpaaYxx4t); LiveBench (arXiv:2406.19314) | G-019 |
| 5 | WebSearch: "multi-agent independence horizontal context isolation LLM swarm shared state correlation collapse" | Horizontal isolation patterns | Partial — Strands Agents SDK / LangChain swarm patterns; arXiv:2503.03800v1 swarm-intelligence framing | G-022 |
| 6 | WebSearch: "PutnamBench long proof Lean 4 max steps benchmark 2024" | Long-proof corpus for mr-tick | Yes — PutnamBench (arXiv:2407.11214; NeurIPS D&B 2024); 672 Lean 4 problems | G-025 |
| 7 | WebSearch: "Goodhart's law LLM gaming reward hacking scoring formula leakage benchmark" | Scoring-leakage anti-pattern | Yes — Reward Hacking Benchmark RHB (arXiv:2605.02964); Lilian Weng 2024-11-28 reward-hacking treatment | G-023 (framing only) |
| 8 | WebSearch: "event sourcing compensating transactions reversibility audit ledger pattern" | Reversibility pattern reference | Yes — Microsoft Azure Compensating Transaction Pattern; Fowler EventSourcing | G-030 (framing only) |
| 9 | WebSearch: ""miniF2F" adversarial false proof injection "type checking" bypass benchmark 2024" | miniF2F adversarial set | Yes — miniF2F-Lean Revisited (arXiv:2511.03108); MiniF2F-Dafny (arXiv:2512.10187) listing DafnyBench-2024 unsoundness exploits | G-012 |
| 10 | WebSearch: "mutation testing formal proof corruption error injection Lean Coq sound rejection rate" | Mutation testing corpora | Partial — mCoq (Celik et al. 2019); no direct Lean equivalent surfaced | G-012 (motivates synthetic-corpus meta-escalation) |
| 11 | WebFetch: https://arxiv.org/abs/2505.24230 (ProofNet++) | Concrete adversarial dataset URLs / sizes / licenses | NO — abstract-level content does not enumerate the 120k corpus URL/license. Pass 3 must fetch the full paper PDF or codebase release. | G-012 (corpus access needs follow-up) |

**Summary**: 10 WebSearch queries + 1 WebFetch. **9 of 10 searches produced usable candidate signal**; 1 (mutation testing for Lean specifically) returned partial — Coq-only mCoq, no direct Lean port. **Cleanest hit**: miniF2F-v2 misalignment audit (arXiv:2511.03108) directly applicable to G-012 PCP soundness without cherrypicking. **Most uncertain**: G-012's full corpus access (license + size) requires Pass 3 follow-up via the ProofNet++ codebase release or direct PDF fetch.

**Honest gaps in web research**:
- **G-012**: no canonical ready-to-inject Lean adversarial corpus surfaced (miniF2F-v2 misalignment list is the closest, but not packaged as a "false proof injection corpus"). Pass 3 may need to **construct** the injection set from the miniF2F-v2 diff. **Meta-escalation**: user approval needed for the construction method before Pass 3.
- **G-022**: no direct multi-agent LLM-swarm horizontal-isolation benchmark surfaced (most results are Strands/LangChain framework docs, not benchmarks). The TB-C0 n=5 batch is the canonical real-load substrate; web framing is supplementary only.

---

## §5 Cross-references

- Pass 1: `/home/zephryj/projects/turingosv4/handover/audits/CONSTITUTION_COVERAGE_GAP_AUDIT_PASS_1_2026-05-07.md`
- Constitution: `/home/zephryj/projects/turingosv4/constitution.md`
- FC-witness catalog: `/home/zephryj/projects/turingosv4/handover/alignment/FC_WITNESS_CATALOG_2026-05-06.md`
- Constitution execution matrix: `/home/zephryj/projects/turingosv4/handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md`
- TB-C0 multi-agent batch evidence (substrate for Type-2 + Type-1 tests): `/home/zephryj/projects/turingosv4/handover/evidence/tb_c0_multi_agent_2026-05-06T16-30-36Z/`
- TB-18R Phase 3 evidence (secondary substrate): `/home/zephryj/projects/turingosv4/handover/evidence/tb_18r_phase_3_2026-05-07T08-33-05Z/`
- Existing Codex audit verdict format reference (G-026 / G-028 parser target): `/home/zephryj/projects/turingosv4/handover/audits/CODEX_TBC0_STRICT_CONSTITUTIONAL_AUDIT_VERDICT_2026-05-07.md` (and v2..v5 series)
- 24-violation source (G-006): `/home/zephryj/projects/turingosv4/handover/architect-insights/TAPE_CANONICAL_AUDIT_2026-04-26_AUDITOR.md`
- TB ship log (G-006 / G-007 / G-013 / G-027 cross-reference): `/home/zephryj/projects/turingosv4/handover/tracer_bullets/TB_LOG.tsv`
- Memory rules referenced: `feedback_real_problems_not_designed`, `feedback_class4_cannot_hide_in_class3`, `feedback_norm_needs_mechanism`, `feedback_constitutional_harness_engineering`, `feedback_audit_after_evidence`, `feedback_no_retroactive_evidence_rewrite`
- Cases (common-law cross-reference): `/home/zephryj/projects/turingosv4/cases/C-072_veto_ai_scope_narrowing.yaml` (G-026, G-028, G-029); `/home/zephryj/projects/turingosv4/cases/C-073_architect_ai_commit_authority.yaml` (G-027); `/home/zephryj/projects/turingosv4/cases/C-074_fc_first_problem_handling.yaml` (cross-cutting)
