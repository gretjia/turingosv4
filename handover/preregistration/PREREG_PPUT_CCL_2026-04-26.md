# PREREG_PPUT_CCL_2026-04-26

experiment_id: PPUT_CCL_FULL_PASS
date_created: 2026-04-26
author: gretjia (with Claude Opus 4.7 collaborator)
committed_commit_sha: PENDING — this file committed before any data-collection run; commit hash recorded by git post-commit hook.

supersedes: Phase 8/9/10 "Paper Preprint Ready" arc. Paper 1 v2.1.1 (commit `c1d7e7c`) reached dual-audit PASS/PASS 2026-04-25; arXiv submission deferred per user directive 2026-04-25 in favor of this longer arc.

cross_references:
- Architect directive: 2026-04-25 FULL PASS upgrade to PPUT-driven evaluation (saved in handover/architect-insights/)
- Cases: C-052 (PPUT as sole metric), C-070 (pre-reg + multiplicity discipline), C-066 (external-agent verification), C-068 (model drift), C-069 (constitutional alignment audit)
- Notepad: handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md F-2026-04-25-02 (this arc's launch entry)

## thesis (frozen, EN + ZH per user)

EN: TuringOS pursues constitution-bound capability compilation measured by Verified PPUT. Black-box agents generate high-throughput proposals, but progress is credited only when a golden path is settled by executable predicates. All failed branches, delays, and context waste remain counted as physical cost. The system therefore improves not by producing more text, but by increasing held-out verified progress per token-time. Capability compilation succeeds only when failure logs are quarantined, distilled into user-space white-box assets, and shown to improve held-out PPUT without increasing false accepts, regressions, or context pollution.

ZH: TuringOS 追求的是以 Verified PPUT 衡量的宪法约束能力编译。黑盒 Agent 可以高吞吐地产生候选提案，但只有当 golden path 被可执行谓词结算时，系统才获得 progress。所有失败分支、时间延迟与上下文浪费都必须计入物理成本。因此，系统能力的提升不在于产生更多文本，而在于提高未见任务上的单位 token-time 可验证进展率。能力编译只有在失败日志被隔离、被提炼为用户态白盒资产，并且这些资产在 held-out 上提升 PPUT 且不增加误判、退化和上下文污染时，才算真正成功。

## 1. formal definitions (frozen at commit time)

### 1.1 Progress

For task `i`:
```
Progress_i = 1   iff   GroundTruth(G_i) = 1   (Lean 4 toolchain verifies the golden-path proof)
Progress_i = 0   otherwise
```

`GroundTruth` is the Lean oracle in `experiments/minif2f_v4/lean4_oracle.rs` enforced via `check_payload` (forbidden patterns: `sorry`, `admit`, `native_decide`, `decide`, `omega` — per C-011 corollary in F-2026-04-20-05). LLM-judge verdicts and runtime "accepted" markers are NOT GroundTruth.

### 1.2 Cost C_i (full physical cost)

```
C_i = sum over all messages m in task i of m.tokens_total
    where messages include:
      - every agent prompt + completion (prompt_tokens + completion_tokens)
      - every tool call's stdout (tool_tokens, hashed and length-summed)
      - every retry and failed branch
      - every architect/auditor meta-loop call attributed to task i
      - every proxy / middleware augmentation
```

C_i is NOT restricted to the golden-path payload. Failed-branch tokens MUST be counted. Goal: a system that wastes 1M tokens to find a 1k-token proof has lower PPUT than one that finds the same proof in 100k tokens.

### 1.3 Time T_i (wall-clock, end-to-end)

```
T_i = end_time - start_time   (seconds)
   start_time = first read of the task statement by any agent in the run
   end_time   = final ground-truth accept (Lean PASS) OR external timeout
```

Lean verification time is included (it is a real cost). Architect/auditor meta-loop time is included.

### 1.4 Verified PPUT

```
VPPUT_i = Progress_i / (C_i × T_i)
```

If `Progress_i = 0`: `VPPUT_i = 0` regardless of how much was spent.

For dashboard readability we report scaled values:
```
PPUT-M_i = 10^6 × VPPUT_i    (per million token-second)
PPUT-B_i = 10^9 × VPPUT_i    (per billion token-second)
```
Raw `VPPUT_i` retained in jsonl for paper / audit. PPUT-M is the dashboard default.

### 1.5 Held-out Verified PPUT (NORTH STAR)

```
H-VPPUT = (sum_{i in heldout_set} Progress_i) / (sum_{i in heldout_set} C_i × T_i)
```

This is the sole optimization signal. All other dashboard metrics are guardrails or diagnostics.

### 1.6 Auxiliary metrics (frozen list)

| Metric | Definition | Role |
|---|---|---|
| FAR | False Accept Rate: runtime accepts that Lean rejects post-hoc / total runtime accepts | Safety guardrail |
| RR  | Regression Rate: tasks SOLVED at baseline that become UNSOLVED after a meta-change | Backwards-compat guardrail |
| ERR | External Recovery Rate: post-rollback retry success rate on tasks that previously hit recoverable failure | Tape / Q^world hygiene |
| CPR | Context Pollution Rate: bytes of rejected-trace text observed in agent prompts / total prompt bytes | Info hygiene |
| IAC | Inter-Agent Correlation: pairwise payload Jaccard mean across agents in the same branch | Diversity (Art. II.2.1) |
| Generalization Gap | (PPUT_adaptation − PPUT_meta_val) on a fixed model snapshot | Adaptation overfit detector |
| AH  | Autonomy Hours: continuous wall-clock without human sudo | Long-run autonomy |
| HIF | Human Intervention Frequency: count of human-overrides per 10K tx | Long-run autonomy |
| WBCG_PPUT | sum over candidate artifacts Δ of `1[ArtifactState(Δ) = Certified]` where `Certified` requires (i) j-PPUT + j-FAR + j-CPR all reject null at family-corrected α per § 5.3 + § 9, (ii) j-RR ≤ p_0 point-check guardrail per § 5.4, (iii) Rollbackable(Δ) = 1 per § 6 E2 (see § 1.7 for ArtifactState; § 7 Gate H for full reachability conditions) | Capability-compilation success |

`N_use ≥ 3` (minimum re-use threshold for an artifact to count as "real").

VTR (Verified Tactic Rate) is **demoted to descriptive** — micro-step gameable. Reported but not gating.

### 1.7 ArtifactState — layered confidence (Patch A from Gemini DeepThink 2026-04-26)

For ANY user-space artifact Δ generated by ArchitectAI (broadcast rule, doc, skill, retrievable tool):

```
ArtifactState(Δ) ∈ {Accepted, Quarantined, Certified, Reverted}

Accepted    : Δ has just passed § 3 metering + § 3.5 + § 3.5.1 AuditorAI
              content meta-predicate battery; eligible for human review (§ 6 D5).
Quarantined : Δ has been Accepted AND user-approved (§ 6 D5) AND has measured
              positive ΔPPUT on meta_validation set with N_use ≥ 3.
              Quarantined artifacts are callable in adaptation / meta_val runs
              and ENTER Phase E heldout sealed eval (§ 6 E2). Their meta_val
              effects are MEASURED but provisional — meta_val ΔPPUT > 0 is a
              FILTER for Phase E entry, not a certification.
Certified   : Δ entered Phase E AND survived (round 4 — three inferential + one
              guardrail + rollback): j-PPUT, j-FAR, j-CPR all reject null at
              family-corrected α (per § 5.3 + § 9), AND j-RR ≤ p_0 point check
              (per § 5.4 — descriptive guardrail, NOT inferential), AND the
              rollback sub-eval confirms Rollbackable=1 (per § 6 E2). Counts
              toward WBCG_PPUT (§ 1.6 + § 7 Gate H).
Reverted    : Δ failed any of the three primary tests, OR violated the j-RR
              point-check guardrail, OR caused regression in post-Phase-E
              audit, OR failed the rollback sub-eval (§ 6 E2). Permanently
              flagged; rollback executed; cannot re-enter promotion within
              this arc (resubmission requires new artifact_id + design change
              per C-035).
```

Critical invariants:
- **Task** Progress stays binary (`Lean accepts ⇒ 1`, else `0`); 4-state machine applies ONLY to artifacts. Mixing the two would corrupt PPUT (Lean gives a perfect predicate; artifact certification is a non-perfect-predicate process — they must not be conflated).
- **WBCG_PPUT counts only `Certified`**. `Quarantined` artifacts contribute to PPUT measurement but not to capability-compilation success. This blocks "degenerate caching" — high-frequency-but-neutral artifacts cannot inflate WBCG.
- **`Reverted` is permanent**. Once an artifact is reverted, it cannot be re-promoted within this arc; resubmission requires a NEW artifact_id with documented design change (per C-035 zero-tolerance).

### 1.8 Trust Root composition (Patch B from Gemini DeepThink 2026-04-26)

The Trust Root is the set of artifacts that **only humans + Boot may write**. ArchitectAI / AuditorAI / any swarm agent attempting to mutate Trust Root entries triggers **runtime SIGKILL-level abort** (not an audit-time warning):

```
Trust Root := {
  src/kernel.rs,                    -- bus / kernel core
  src/wal.rs,                       -- write-ahead log
  src/bus.rs,                       -- predicate registration / capability gates
  experiments/minif2f_v4/src/lean4_oracle.rs,  -- ground-truth predicate
  constitution.md,                  -- normative spec
  cases/C-*.yaml,                   -- common-law precedent (append-only by human)
  genesis_payload.toml [pput_accounting_0],   -- PPUT accounting rules
  handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json,  -- frozen splits
  handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md     -- this file
}
```

Enforcement mechanism (Phase B7 work item):
1. `genesis_payload.toml [trust_root]` lists frozen SHA-256 of each Trust Root file.
2. `Boot` verifies all hashes match at startup; mismatch → abort with `TRUST_ROOT_TAMPERED`.
3. ArchitectAI write tool whitelists `user_space/Δ_*/`; any path outside whitelist returns `EPERM` from the syscall layer (not a soft refusal at the LLM prompt layer).
4. Conformance test `tests/trust_root_immutability.rs`: simulates ArchitectAI attempting to write each Trust Root path → asserts `EPERM` returned.

Heldout sealed hash specifically: `sha256(sorted_newline_joined(heldout_problem_ids))`. This hash is committed to `genesis_payload.toml`; any tampering with the heldout list (adding / removing / re-ordering) detected at Boot.

This is the **software floor** of the eventual hardware Trust Root (Open Claw → FPGA path); software immutability today, hardware immutability later.

## 2. three-split protocol (irreversible)

### 2.1 Source pool

MiniF2F/Test split: 244 problems in `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/*.lean`. MiniF2F/Valid (also 244 problems) is reserved as out-of-distribution heldout for future arcs and NOT part of this arc's three-split.

**Source pool SHA-256 (canonical = sorted IDs newline-joined, no trailing newline)**: `77179cf2598b0df707d78a6663e763121dfe8e73a6538073155f488feab95093`

This fingerprint is committed to `genesis_payload.toml [pput_accounting_0]` at Phase B7. Boot verifies match; tampering = `TRUST_ROOT_TAMPERED` abort.

### 2.2 Split rule

Deterministic, hash-based, 60/20/20:
```python
import hashlib
SEED = "20260426_PPUT_CCL"   # frozen string seed, declared BEFORE any draw
def bucket(pid: str) -> str:
    h = hashlib.sha256((SEED + ":" + pid).encode()).hexdigest()
    n = int(h[:8], 16) % 100
    if n < 60:  return "adaptation"      # nominal ~146, realized 144 — Phase B-D iteration
    if n < 80:  return "meta_validation" # nominal ~49, realized 46    — CCL internal audit (Phase D)
    return "heldout"                     # nominal ~49, realized 54    — North Star (Phase E only)
```

Script committed as `handover/preregistration/scripts/split_pput_ccl.py`. Output committed as `handover/preregistration/PPUT_CCL_SPLITS_2026-04-26.json` with three lists + per-bucket counts + SHA-256 of each list.

**Realized counts (committed 2026-04-26)**: adaptation 144 / meta_validation 46 / heldout 54 (nominal 146 / 49 / 49; all within ±5 tolerance gate).

**Heldout sealed SHA-256**: `51440807c9ecc5c366d1adb640afcc96fcd227d18e4a35c7f85aaec78475086b`

This sealed hash is the heldout-specific Trust Root entry (PREREG § 1.8). Committed to `genesis_payload.toml [pput_accounting_0].heldout_sealed_hash` at Phase B7.

### 2.3 Heldout sealing — operational, not cryptographic (revised round 2 per Codex audit M2)

**Honest framing**: the heldout problem IDs ARE in cleartext in `PPUT_CCL_SPLITS_2026-04-26.json`, and the seed + bucket rule are public. ANY entity that can read repo files can recompute the heldout list. The SHA-256 in § 2.2 / § 1.8 is therefore **NOT a confidentiality seal — it is an integrity check** (detects tampering / re-draws).

Real sealing in this arc is **operational**: enforced by access pattern + conformance testing. **Round 3 closure of Codex R3-3 (sealing leaky under recomputation)**: the round-2 blacklist covered only PREREG and SPLITS files; an agent could still grep the seed string from architect insight files, audit echoes, or tool call args, then run `hashlib` itself. The full operational sealing is now multi-layered:

1. `tests/heldout_operational_sealing.rs` (Phase B7 implementation; **5 layers**):

   **L1 — File-path read isolation (existing, kept)**:
   - Greps `experiments/minif2f_v4/src/`, `src/`, `bin/`, plus any agent loader, for any read of `heldout_*.txt`, splits JSON's heldout key, or substrings matching the heldout list.
   - Only `bin/heldout_evaluator.rs` is whitelisted (Phase E gate runner).

   **L2 — Agent prompt context blacklist (existing, expanded round 3)**:
   - At agent prompt construction, the following file BYTES cannot enter the prompt:
     - `PREREG_PPUT_CCL_2026-04-26.md` (this file)
     - `PPUT_CCL_SPLITS_2026-04-26.json` (the split output)
     - `handover/preregistration/scripts/split_pput_ccl.py` (the split generator)
     - `handover/architect-insights/PPUT_DRIVEN_FULL_PASS_2026-04-25.md` (mentions seed)
     - `handover/architect-insights/GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` (mentions seed)
     - `handover/audits/CODEX_PPUT_CCL_AUDIT*.md` and `handover/audits/GEMINI_PPUT_CCL_AUDIT*.md` (echo seed)
     - `handover/audits/DUAL_AUDIT_PPUT_CCL_VERDICT_2026-04-26.md` (echoes findings)
     - Any file containing the literal seed string `20260426_PPUT_CCL`
   - Any prompt that includes their bytes → `EHELDOUT_CTX_LEAK` abort.

   **L3 — Tool call pre-flight filter (round 4 generalized per Codex round-3 R3 hash-literal critique)**:
   - File-reading tool calls (`cat` / `Read` / `head` / etc.) issued by Phase B-D agents pre-flight-checked: path matches blacklist → `EPERM`.
   - Tool call ARGUMENTS scanned for **substring matches** of seed components, not just the literal full string:
     - Any string containing `20260426_PPUT_CCL`
     - Any string containing both `20260426` AND `PPUT_CCL` (catches concatenation `"20260426" + "_" + "PPUT_CCL"`)
     - Any string containing `PPUT_CCL` paired with a year-month-day pattern (catches `"PPUT_CCL"` reconstruction patterns)
   - Tool call arguments scanned for shell hash invocations:
     - `sha256sum`, `sha1sum`, `sha224sum`, `sha384sum`, `sha512sum`, `md5sum`, `b3sum`, `xxh*sum`, `b2sum`
     - `openssl dgst` and any subcommand
     - `python -c "import hashlib; ..."` (any inline hashlib invocation)
     - `python3 -c "..."` containing `hashlib` or `cryptography.hazmat`
     - `node -e "..."` containing `crypto.createHash`
   - Match on either pattern → `EPERM`. The whitelist `handover/preregistration/scripts/split_pput_ccl.py` runs ONCE at Phase A2; this script's path is never invoked from a Phase B-D agent code path.

   **L4 — Hash function & code-path static analysis gate (round 4 generalized)**:
   - Conformance: any agent-readable code path containing **any** hash invocation AND containing the seed substring (`20260426_PPUT_CCL` or its concatenation pieces) is BLOCKED at static analysis time:
     - **Python**: `hashlib.*`, `cryptography.hazmat.primitives.hashes.*`, `Crypto.Hash.*` (PyCryptodome), manual hex sha-256 implementations (regex pattern: function bodies containing `0x6a09e667`, `0xbb67ae85`, ... — the SHA-256 IV constants — flagged for review)
     - **Rust**: `sha2`, `sha1`, `sha3`, `blake2`, `blake3`, `md5`, `md-5`, `digest`, `ring`, `rustcrypto-hashes`, `openssl::hash`
     - **JavaScript/TypeScript**: `crypto.createHash`, `subtle.digest`, `js-sha256`, `tweetnacl-hash`, `hash.js`
     - **Shell**: any of the L3 shell hash binaries
   - Detection: grep agent-readable code paths for both (hash function name) AND (seed substring); non-empty intersection → BLOCKER at conformance test time, `cargo test` fails.
   - Whitelist: `handover/preregistration/scripts/split_pput_ccl.py` (Phase A2 only); `bin/heldout_evaluator.rs` (Phase E only, by which point sealing is no longer required).

   **L5 — MiniF2F/Test source-pool enumeration block (round 4 generalized per Codex round-3 R3 enumeration concern)**:
   - The 244-problem MiniF2F/Test ID list is blacklisted from Phase B-D agent context: agents cannot enumerate the source pool from which heldout was drawn.
   - Phase B-D agents see only:
     - The adaptation 144 IDs (cleartext, in their training-set role)
     - The meta_val 46 IDs (cleartext, in their internal-audit role)
     - **NOT** the heldout 54 IDs and **NOT** the source pool 244 IDs
   - **Path enumeration block (round 4)**: agent tool calls cannot enumerate the MiniF2F/Test directory:
     - `ls`, `find`, `rg --files`, `tree`, `dir`, `Get-ChildItem`, `glob.glob`, Python `os.listdir` against any path containing `MiniF2F/Test` literal substring → `EPERM`.
     - Path-blacklist applies recursively: `/home/zephryj/projects/turingosv3/experiments/minif2f_data_lean4/MiniF2F/Test/`, any symlink to it, any cached-copy in `target/` or `.lake/`.
     - L_t archive (Phase B onward) excludes any references to MiniF2F/Test paths from its agent-visible projection.
   - Whitelist: `bin/heldout_evaluator.rs` reads the full list at Phase E (sealed eval gate).

2. CI / `cargo test` MUST run this conformance battery pre-commit. Failure = arc commit blocked.

3. Heldout problems NEVER appear in adaptation / meta_val runs. Detected leak (post-Phase E audit) = BLOCKER, arc invalidated, must respin with a new seed string AND a new heldout draw.

**Residual risk** (acknowledged honestly): if a future ArchitectAI artifact ENCODES (via paraphrase / parametric template) bucket-derivation logic such that calling the artifact at heldout time triggers re-bucketing on the heldout's own pid, this defense fails. M6 (§ 3.5.1 lookup-table evasion + parametric templates max-cardinality) tightens artifact content checks. **The deepest residual**: the seed string is committed to git history forever; future archaeologists with full repo access can recompute. Operational sealing is for the 30-day arc lifetime, not eternity. Post-Phase-E audit may reveal the seed publicly; this is the price of reproducibility.

**Integrity vs confidentiality distinction**: SHA-256 of the heldout list is part of Trust Root (§ 1.8); tampering = `TRUST_ROOT_TAMPERED` Boot abort. This protects the "did anyone change which problems are heldout" question, not the "can anyone see which problems are heldout" question. The latter is operational sealing only.

### 2.4 Heldout evaluation cadence

- Phase B (Week 1 instrumentation): heldout NOT touched.
- Phase C (Week 2 ablation): heldout NOT touched. Ablation runs on adaptation set only.
- Phase D (Week 3 shadow CCL): heldout NOT touched. Meta_val used for ArchitectAI candidate scoring.
- Phase E (Week 4 controlled activation): **single sealed evaluation** on heldout. No iteration on heldout result. Deterministic seeds pre-committed (see § 6).

Any second heldout evaluation requires a NEW pre-reg with fresh split (i.e., heldout becomes burned).

## 3. anti-Goodhart guardrails (frozen)

PPUT becomes the sole optimization target, so it becomes a Goodhart attack surface. Conformance battery (`tests/pput_anti_goodhart_conformance.rs`) MUST hold:

```
test_all_model_tokens_counted              — total_tokens accounting includes prompt + completion + tool stdout
test_tool_stdout_hash_logged                — every tool call records SHA-256 of stdout + length
test_no_hidden_unmetered_generation         — no LLM call path bypasses the token meter (grep test on src/)
test_no_problem_id_hardcode                 — no `problem_id ==` constant comparisons in agent code
test_no_metric_file_access_by_agents        — agent code paths cannot read PPUT logs / metric jsonl
test_no_pput_in_agent_prompt                — prompt builders never inject PPUT scalars / dashboard values
test_golden_path_requires_ground_truth      — Progress = 1 iff Lean returns Pass on full proof
test_failed_branches_in_total_cost          — C_i unit test: 5 failed proposals + 1 success → C_i = sum(6)
test_wall_clock_first_read_to_final_accept  — T_i bracket includes Lean verify time
test_heldout_ids_inaccessible               — heldout list not readable from agent code paths
```

10 tests, must all PASS at every Phase gate.

### 3.5 Artifact content meta-predicates (Patch C — AuditorAI battery, Phase D)

Run by AuditorAI on every candidate artifact Δ before it can transition `Accepted → Quarantined`. These guard artifact CONTENT quality (separate axis from § 3 which guards METERING):

```
docs_contain_no_raw_failed_trace(Δ)
  -- No raw L_t entries embedded as text/code in artifact body.
  -- Detection: for each raw_error_hash in L_t, assert hash absent from Δ tokens.
  -- Why: prevents hallucination-repeat — agents reading the artifact would
  --      re-encounter the failed pattern as if it were guidance.

docs_do_not_include_exact_adaptation_solution(Δ)
  -- For each problem p in adaptation set with golden_path_payload G_p,
  --   assert sha256(any 64-token window of Δ) ≠ sha256(any 64-token window of G_p).
  -- Why: prevents data leakage. ArchitectAI distilling adaptation L_t must
  --      compress to abstractions, not memorize answers verbatim.
  -- Detection: rolling 64-token-window hash check; threshold = 0 collisions.

docs_code_blocks_are_parametric_templates(Δ)
  -- For every fenced ``` code block in Δ:
  --   AST-parse the block; assert it has ≥ 1 placeholder identifier
  --   matching pattern `<[A-Z][A-Z_]*>` OR `${...}` OR `{{...}}`.
  -- Why: blocks pure-hardcoded copy-paste artifacts. White-box compilation
  --      must produce abstractions, not constants masquerading as wisdom.

docs_include_scope_and_expiration(Δ)
  -- Δ frontmatter MUST include `scope: <list-of-pattern-or-domain>` and
  --   `expires: <ISO date or "permanent" with justification>`.
  -- Why: lifecycle hygiene; enables future GC of stale artifacts and prevents
  --      eternal-rule-accumulation degenerate state.
```

Failure of ANY predicate ⇒ `Δ.state = Rejected` (not Accepted); ArchitectAI receives the failure category as feedback for the next attempt; the failed candidate is logged but not stored as artifact.

These 4 predicates are deliberately separate from § 3 (anti-Goodhart on metering) because they target a different attack surface: METERING gaming would let the system inflate PPUT numbers; CONTENT gaming would let ArchitectAI launder adaptation answers into doc form. Both surfaces must be defended.

### 3.5.1 Lookup-table evasion predicates (round 2 per Codex audit M6 / P1 → P0)

Codex round 1 noted that the basic 4 predicates above can be bypassed by:
- Using **problem IDs as keys** (e.g., a YAML rule "for `mathd_algebra_171` apply `linarith`")
- Using **theorem names as keys** (e.g., a Mathlib-like dict `Finset.sum_range_succ → ...`)
- Using **63-token chunks** to defeat the fixed-64 rolling-hash window
- Using **paraphrased tactic recipes** that escape exact-match (harder, defended descriptively)
- Using **high-cardinality dict structures** that look parametric but encode lookup

These 4 added predicates close the gap:

```
docs_no_problem_id_keys(Δ)
  -- For each pid in adaptation ∪ meta_validation problem ID lists:
  --   assert pid (as a string) does NOT appear in Δ body or Δ frontmatter.
  --   Exception: pid may appear ONLY in artifact-id provenance fields
  --   (`source_log_hashes`, `derived_from_run_id`) — never in the rule body.
  -- Why: blocks per-problem rule-of-the-day disguised as "white-box rule".

docs_no_theorem_name_keys(Δ)
  -- Parse Δ body if it has YAML / TOML / JSON-style structure.
  -- Reject if any dict key matches the regex `[A-Z][A-Za-z0-9_]*\.[a-z][A-Za-z0-9_]*`
  --   (Mathlib namespace.lemma form) AND the dict has > 1 entry.
  -- Why: blocks "lemma_name → tactic" lookup tables disguised as rules.

docs_rolling_hash_multi_window(Δ)
  -- For each problem p in adaptation ∪ meta_validation with golden_path_payload G_p:
  --   For window_size in [16, 32, 64, 128]:
  --     assert no rolling-window hash collision between Δ tokens and G_p tokens.
  -- Why: defeats off-by-one chunking attacks against the fixed 64-token check.

docs_max_dict_cardinality(Δ)
  -- For every dict-style structure in Δ (YAML/TOML/JSON):
  --   assert dict.keys().count() ≤ 8.
  -- Why: high-cardinality maps are lookup tables, not rules. Real rules
  --      compress N -> 1 (many situations to one decision); they don't
  --      enumerate N -> N.
```

ArchitectAI's response on rejection: failure category fed back; candidate logged but not stored. Three consecutive same-category rejections on the same source-log cluster = ArchitectAI is in degenerate mode for that cluster; user notified, that cluster put on hold.

**Residual evasion** (acknowledged): paraphrased tactic recipes (e.g., rewriting a proof in equivalent Lean syntax) cannot be 100% machine-detected. Defense is layered: the parametric_templates predicate (must contain placeholder syntax) + the human-as-live-meta-predicate (§ 6 D5) reading each candidate. Paraphrased lookup is hardest for ArchitectAI to disguise as "abstract rule" under human review.

## 4. North Star + decision criterion

```
H-VPPUT (heldout) is the sole optimization metric.
WBCG_PPUT (over the arc) is the success metric.
FAR, RR, CPR, IAC, Generalization Gap are guardrails (must not regress).
```

**Final FULL PASS** = WBCG_PPUT > 0 on heldout (Gate H, § 7).

## 5. directional hypotheses (CLEAN REWRITE round 4 — single source of truth)

Round 1-3 patches left inconsistencies between sections on the independent unit and which tests are inferential. **Round 4 rewrites § 5 + § 9 from scratch**. Single source of truth from this point forward; any earlier per-(problem, seed) reference is superseded by per-problem.

### 5.1 Independent units (frozen)

| Phase | Unit | n | Justification |
|---|---|---|---|
| C (hard-10 ablation) | **per-problem** | 10 | hard-10 adaptation problems × 2 random Boltzmann seeds; seeds are noise-reducing replicates (correlated within problem). Per-problem score = paired-binary aggregated over 2 seeds. |
| E (heldout-54) | **per-problem** | 54 | All 54 heldout problems × 3 pre-committed Boltzmann seeds; seeds are replicates. Per-problem score = paired sign of mean VPPUT over 3 seeds. |

`(problem, seed)` is NOT used as an independent unit anywhere in this PREREG. Any prior reference is superseded.

### 5.2 Phase C inferential hypotheses (4 tests)

For each pair (Full, Mode_alt) where Mode_alt ∈ {Soft Law, Panopticon, Amnesia, Homogeneous}:

| ID | Hypothesis (one-sided directional) | Primary endpoint | Direction |
|---|---|---|---|
| H1 | Soft Law has lower verified-PPUT than Full | McNemar paired sign test on n=10 per-problem paired-binary outcomes: `1 if mean(VPPUT_Full)_p > mean(VPPUT_SoftLaw)_p else 0` (mean over 2 seeds) | Full > Soft Law |
| H2 | Panopticon has lower H-VPPUT than Full | Same construction, Full vs Panopticon | Full > Panopticon |
| H3 | Amnesia has lower H-VPPUT than Full | Same construction, Full vs Amnesia | Full > Amnesia |
| H4 | Homogeneous swarm has lower H-VPPUT than Full | Same construction, Full vs Homogeneous | Full > Homogeneous |

**Test mechanics**: for each problem p in hard-10, average VPPUT over 2 seeds per mode → compare per-problem means → produce paired-binary (`1` if Full mean > Alt mean, `0` otherwise) → McNemar one-sided exact binomial on the 10 paired-binary observations.

**Descriptive secondary endpoints** (logged, NOT in inferential family, NO α correction): mode-level CPR, mode-level IAC, mode-level FAR, gap between pput_runtime and pput_verified for Soft Law, ERR for Amnesia, mean prompt length per mode.

### 5.3 Phase E inferential hypotheses (3 per artifact)

For each artifact j ∈ {1, ..., k} (k ≤ k_max = 10) entering Phase E with `ArtifactState = Quarantined`, **three** primary inferential tests run on heldout-54 (NOT four — j-RR is a descriptive guardrail per § 5.4):

| ID family | Hypothesis (one-sided) | Primary endpoint (n=54) | Direction |
|---|---|---|---|
| `j-PPUT` | Artifact j increases held-out VPPUT | Sign test on n=54 per-problem signs of `mean(VPPUT_all_p) − mean(VPPUT_minus_j_p)` (mean over 3 seeds; leave-one-out per § 6 E2) | sign > 0 |
| `j-FAR` | Artifact j does not increase FAR | Non-inferiority sign test on n=54 per-problem signs of `FAR_all_p − FAR_minus_j_p` | sign ≤ 0 |
| `j-CPR` | Artifact j does not increase CPR | Non-inferiority sign test on n=54 per-problem signs of `CPR_all_p − CPR_minus_j_p` | sign ≤ 0 |

`j-RR` is **NOT** in the inferential family — see § 5.4.

### 5.4 j-RR as descriptive guardrail (round 4 fix per Codex round-3 R4-2)

**Why this changed**: Round 3 placed j-RR in the inferential family with H0: `regression_rate ≥ p_0 = 0.10`. Codex round-3 noted this is mathematically unwinnable: best case 0/54 regressions gives `p = 0.9^54 ≈ 0.00343`, but smallest Holm threshold at N_max = 44 was 0.05/44 ≈ 0.00114 → fails to reject even with zero regressions. Gate H unreachable.

**Round 4 fix**: j-RR is reclassified as a **descriptive guardrail** (point estimate check, no α correction, no place in family):

```
j-RR criterion (point check, NOT inferential):
  r_j = |{p in heldout : SOLVED(p, baseline) = 1
                       AND SOLVED(p, all_artifacts_active) = 0
                       AND SOLVED(p, minus_j) = 1}|
  -- Problems where artifact j SPECIFICALLY caused a regression versus
  -- baseline, attribution by leave-one-out: removing j fixes the regression.

  RR_j = r_j / 54        (point estimate, NOT a hypothesis test)

  Pass criterion: RR_j ≤ p_0   where p_0 is calibrated per § 5.5.
```

**Why descriptive is statistically correct here**: j-RR is a constraint, not a discovery hypothesis. We do not need to argue "RR_j is significantly less than p_0"; we only need RR_j ≤ p_0. A point check is precisely the right tool. No multiplicity correction applies because this is not a test.

### 5.5 p_0 calibration protocol

`p_0` (baseline regression rate) — full calibration protocol:

```
Toggle:        evaluator --mode full         (control)
               evaluator --mode full --simulate-rollback-at-tx-50  (treatment;
                 forces a synthetic mid-run rollback at tx 50, mimicking the
                 worst-case stochastic edge case Phase E artifacts could trigger)
Sample:        FULL adaptation-144 problem set × Boltzmann seeds [31415, 2718]
                 = 288 paired runs.
Estimator:     for each problem p, define regression_p = 1 iff control SOLVED
                 (Lean PASS) AND treatment UNSOLVED for the SAME (problem, seed).
               Per-problem regression: max over the 2 seeds (worst case).
               p_0 = sum_p regression_p / 144.
Schedule:      Phase B7 (mandatory; not deferrable to Phase D). Phase D cannot
               start without p_0 frozen.
Freeze point:  end of Phase B7. Value written to
                 `genesis_payload.toml [pput_accounting_0].baseline_regression_rate`
               with SHA-256 of the calibration jsonl in
                 `[pput_accounting_0].baseline_regression_jsonl_sha256`.
Audit trail:   raw calibration jsonl committed to
                 `experiments/minif2f_v4/logs/p0_calibration_2026-04-*.jsonl`
               BEFORE Phase D begins. This file is part of Trust Root (§ 1.8) —
               post-freeze tampering = TRUST_ROOT_TAMPERED Boot abort.
Ceiling:       if measured p_0 > 0.10, ABORT — toggle too aggressive or system
               too brittle. NOT an opportunity for j-RR auto-pass.
Dual audit:    p_0 calibration result in Phase B → C audit packet.
```

This protocol forecloses tuning-surface attacks: every parameter is pre-committed; the calibration runs are observable jsonl; the value enters Trust Root.

### 5.6 Family total (clean restatement)

```
inferential_family_size = 4 (Phase C) + 3·k (Phase E)
N_max = 4 + 3·k_max = 4 + 3·10 = 34   (frozen for ALL Holm decisions; see § 9)
```

j-RR is NOT in the family because it is a point-check guardrail.

### 5.7 H5 — deterministic gate (NOT in inferential family)

```
Gate H: count of artifacts j with ArtifactState(j) = Certified ≥ 1
```

`Certified` requires the conjunction of:
1. **j-PPUT, j-FAR, j-CPR** all reject the null at the family-corrected α level (per § 9; 3 inferential tests),
2. **j-RR ≤ p_0** (point check guardrail, no α correction; per § 5.4),
3. **Rollbackable(j) = 1** (verified by an explicit rollback sub-eval in § 6 E2).

H5 is a deterministic AND of (3 inferential rejections) + (1 point check) + (1 rollback verification). Reaching Gate H = at least 1 artifact passes all five conditions. H5 itself is not a hypothesis test; no separate p-value.

## 6. execution plan (30 wall-clock days, pre-committed)

### Phase A — Pre-flight (days 1-3)
- A1 ✏️ this PREREG drafted
- A2 split + sealed hash committed
- A3 notepad updated to reflect arc pivot
- A4 dual external audit (Codex + Gemini); conservative VETO>CHALLENGE>PASS; iterate to PASS/PASS before A5
- A5 commit gate: PREREG + split + notepad + audit verdicts. **No Phase B work before A5.**

### Phase B — Kernel instrumentation + PPUT accounting (days 4-10)
- B1 JSONL schema v2 (proposal-level + run-level fields per architect § 14)
- B2 `C_i` full-cost aggregator (all agents × branches × failures × tool stdout)
- B3 `T_i` wall-clock = first-read → final-accept incl. Lean verify
- B4 `pput_verified` vs `pput_runtime` dual-field separation
- B5 10-test anti-Goodhart conformance battery (§ 3) PASSING
- B6 PPUT-context-leak audit: grep evaluator.rs / prompt.rs / sdk for any path that injects PPUT scalars into agent prompts; gate them
- B7 Boot freeze: `genesis_payload.toml` adds `pput_accounting_0` block (Progress definition / Cost scope / heldout sealed hash). Frozen — any change triggers Phase Z' 6-stage re-audit per C-069.
- **Gate B**: on hard-10 adaptation subset, any single run can self-consistently report (verified golden path? total_tokens? wall_time? VPPUT?). All 10 anti-Goodhart tests PASS. Heldout never touched.

### Phase C — Ablation smoke tests (days 11-17)
- C1 **5 modes implemented as a single `--mode` CLI flag on one binary** (Patch E from Gemini DeepThink): `evaluator --mode {full, panopticon, amnesia, soft_law, homogeneous}`. Modes change runtime behavior; binary bytes do not change between modes.
- C2 **hard-10 adaptation × 2 random Boltzmann routing seeds; independent unit = per-problem (seeds = noise-reducing replicates), not (problem, seed)** (round 3 fix per Codex round-2 R3-1 — within-problem seed correlation). Hard-10 sample is drawn from the adaptation set (144 problems) using the same hash-based deterministic procedure as the three-split: `random.Random("hard10_pput_ccl_seed").sample(adaptation_set, 10)`. The 10 problem IDs + their fingerprint are pre-committed in Phase A5. Boltzmann seeds = `[31415, 2718]`. Total per-mode runs: 5 modes × 10 problems × 2 seeds = 100 jsonl rows. McNemar per condition pair uses **n = 10 per-problem paired-binary observations** (paired sign of `mean(VPPUT_seed1, VPPUT_seed2)_full > mean(VPPUT_seed1, VPPUT_seed2)_alt`). See § 9.2 + § 9.5 for power analysis.
- C3 verify H1-H4 (§ 5) on PPUT axis
- C4 CHECKPOINT_PHASE_C document + 7-redline check + dual external audit
- C5 **Mode-purity conformance**: `tests/mode_flag_binary_purity.rs` asserts that running all 5 modes on the same problem produces jsonl with **identical `git_sha`, `binary_sha256`, and `model_snapshot` fields** — only the `mode` field differs. Any drift = BLOCKER (rules out "Soft Law happened to use a different binary" confound).
- **Gate C**: H1-H4 each pass at Bonferroni α (§ 9); IF any H1-H4 fails, diagnose root cause, do NOT enlarge N (per feedback_phased_checkpoint). Heldout never touched.

### Phase D — CCL shadow mode (days 18-24)
- D1 ArchitectAI role: shadow only; reads adaptation L_t **(ArchitectAI is the SOLE reader of L_t, see D4)**; produces a distilled summary + candidate artifact Δ in human-readable form; proposes user-space artifacts (skill rules, broadcast rules, retrievable docs, tools).
- D2 AuditorAI role: runs **§ 3.5 artifact content meta-predicates** + structural checks (rollbackable? schema-conformant? non-Trust-Root path?) on candidate Δ. **AuditorAI receives Δ + ArchitectAI distillation summary ONLY; never raw L_t** (Patch D — cognitive isolation, prevents Generator-Evaluator collapse per C-010).
- D3 Per-artifact JSONL row (architect § 14 schema): artifact_id, state ∈ {Accepted | Quarantined | Certified | Reverted} (per § 1.7), source_log_hashes, generated_by, audited_by, meta_predicate_results (§ 3 + § 3.5 results), estimated_pput_gain_prior, actual_pput_gain_meta_val, used_count, rollbackable
- D4 **Cognitive isolation conformance** (Patch D upgrade):
  - `tests/architect_sole_lt_reader.rs`: greps src/ + experiments/ + bin/ for any reader of `L_t / failure_log / rejection_trace` paths; whitelist contains exactly **`bin/architect_distill.rs`**. Any other reader = BLOCKER.
  - `tests/auditor_sees_candidate_only.rs`: AuditorAI's input parameters at call site exclude raw L_t; only `(candidate_artifact, architect_summary)` accepted.
- D5 **Human-as-live-meta-predicate (Patch — confirmed by审查官 2026-04-26)**: Phase D has no automated `Quarantined → Certified` promotion. User (`gretjia`) reviews each `Accepted` candidate Δ in human-readable form (≤1 min per artifact), issues `keep | drop | revise` verdict. Approved Δ enters `Quarantined` for meta_val measurement; user keystroke logs become Phase E's golden Trust Data for training Phase E auto-meta-predicates. Defense-in-depth: prevents Claude-only homogeneous LLM stack from establishing a covert promotion-loop (Art. V + C-010 protection).
  
  **D5 SPOF contingency (round 2 per dual audit M5)**:
  - Per-candidate review SLA: **48 hours wall-clock** from `Accepted` → user verdict.
  - SLA breach: candidate auto-moves to `deferred` queue. Phase D continues processing other candidates; deferred items revisited if user returns within Phase D window.
  - **30-day arc clock does NOT pause for user unavailability**. User unavailability = lost Phase D throughput, accepted operational risk; arc continues on schedule.
  - **Hard abort condition**: if ≥ 5 candidates queue > 48h simultaneously, abort Phase D early and proceed to Phase E with whatever artifacts are already `Quarantined`. This prevents Phase D from blocking the entire arc on user availability.
  - All `keep | drop | revise` verdicts logged with timestamp + (optional) one-line user reasoning to `handover/ccl/human_meta_predicate_log_2026-04-*.jsonl` for Phase E auto-meta-predicate training.
- **Gate D**: at least one candidate artifact reaches `Accepted` AND user-approved AND shows ΔPPUT > 0 on meta_val (i.e. promotes to `Quarantined` and survives meta_val window). NO auto-merge. NO `Certified` state achievable in Phase D — only Phase E heldout audit can mint Certified. Heldout never touched.

### Phase E — Controlled activation + heldout sealed eval (days 25-30)

**Single-touch discipline**: heldout-54 problem set is touched ONCE for an evaluation campaign. Within that single touch, the sealed eval runs `k+2` sub-evaluations on the SAME problem set; this is not multiple independent draws — it is a leave-one-out factorial design within one sample (round 2 per Codex audit M3).

- E1 Auto-loop: ArchitectAI → AuditorAI → meta-predicate gate → user-space write (no kernel writes; rollbackable artifact directory `user_space/Δ_*/`).
- E2 **Sealed heldout evaluation campaign — leave-one-out within sealed eval (revised round 2)**:
  - Pre-committed seeds: `[20260426, 31415, 2718]` (3 Boltzmann routing seeds; same 3 seeds used for every sub-evaluation below for paired comparison).
  - Let `Q = {q_1, ..., q_k}` denote the set of `Quarantined` artifacts entering Phase E from Phase D.
  - **Sub-eval 0 (BASELINE)**: heldout-54 × 3 seeds with NO Phase D artifacts active. Produces `PPUT_baseline`.
  - **Sub-eval ALL**: heldout-54 × 3 seeds with ALL of Q active simultaneously. Produces `PPUT_all`.
  - **Sub-evals LOO (leave-one-out)**: for j ∈ 1..k, run heldout-54 × 3 seeds with `Q \ {q_j}` active (every artifact EXCEPT q_j). Produces `PPUT_minus_j`.
  - Total sub-evaluations: `k + 2`. All on the SAME 54 problems × 3 seeds. The 54 problems are not redrawn between sub-evals — single-touch discipline is preserved.
  - Per-artifact ΔPPUT: `Δ_j = PPUT_all − PPUT_minus_j` (the marginal contribution of q_j given the rest).
  - Same logic for ΔFAR, ΔCPR. RR_j is computed by comparing per-problem solved/unsolved status with q_j active (ALL run) vs without q_j (LOO_j).
- E3 Statistical inference per § 5.3 + § 9. For each j, evaluate the **three** primary inferential tests (j-PPUT, j-FAR, j-CPR) under Holm-Bonferroni at family `4 + 3k`, N_max = 34. j-RR is a separate descriptive guardrail check (point estimate ≤ p_0 per § 5.4), NOT in the inferential family.
- E4 Final dual external audit (Codex + Gemini); conservative VETO > CHALLENGE > PASS.

**No iteration on heldout result**: results compute once, reported once. If round-1 Phase E reveals the auto-loop is broken (e.g., 0 of k artifacts pass any test), the arc is declared a NEGATIVE finding. Re-running Phase E with a fresh draw requires a NEW pre-registration AND a new split (heldout = burned).

**Compute budget for E2**: `54 problems × 3 seeds × (k + 2)`. With k ≤ 10 (a realistic Phase D throughput), that's at most `54 × 3 × 12 = 1944` problem runs. At ~50s per run (chat oneshot tier), ~27 wall-hours. Fits within Phase E's 6-day window with ~5x headroom.

## 7. FINAL PASS gates (Gates A-H, conjunctive — ALL must hold)

| Gate | Condition |
|---|---|
| A | AuditorAI does not have final merge authority — only meta-predicate output; final accept = Lean ground truth (for tasks) + dual external audit (for arc) + human sudo (for artifact promotion in Phase D) |
| B | **Trust Root immutability** (Patch B upgrade + round 2 fallback per Codex audit TR): ArchitectAI writes only to `user_space/Δ_*/`. Primary enforcement: syscall-layer `EPERM` trap. **Fallback enforcement** (if syscall layer not feasible in Rust user-space without privileged sandbox): library-level write gate at the artifact-write API + path whitelist + `panic!()` abort on bypass attempt + post-write `git diff` against Trust Root paths; any non-empty diff = `panic!("TRUST_ROOT_BYPASS")` and process termination. Either primary OR fallback enforcement passing the `tests/trust_root_immutability.rs` conformance suite is sufficient for Gate B; bypass through both layers = arc BLOCKER. The PREREG accepts the fallback as equivalent strength because the conformance test is identical: simulated ArchitectAI write attempts to Trust Root paths must abort the process. |
| C | WBCG_PPUT measured on heldout, not on adaptation or meta_val |
| D | Raw L_t never reaches AuditorAI or general agent context (asserted by D4 cognitive-isolation conformance: `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs`) |
| E | Three splits enforced; heldout touched at most once (E2) |
| F | All failed branches counted in every C_i (asserted by anti-Goodhart test_failed_branches_in_total_cost) |
| G | Progress = 1 only iff GroundTruth(G_i) = 1 (Lean) |
| H | **Heldout WBCG_PPUT > 0 with ArtifactState = Certified gate** (round 4 — j-RR moved out of inferential family per Codex round-3): at least one ArchitectAI-generated user-space artifact Δ has `ArtifactState(Δ) = Certified` per § 1.7 — meaning (i) was Accepted (passed § 3 + § 3.5 AuditorAI battery), (ii) was Quarantined (user-approved + ΔPPUT_meta_val > 0 + N_use ≥ 3 on meta_val), (iii) at heldout sealed eval **three** primary inferential tests reject null at family-corrected α (per § 5.3 + § 9): **j-PPUT** (sign test on n=54), **j-FAR** (non-inferiority sign test), **j-CPR** (non-inferiority sign test), AND (iv) **j-RR ≤ p_0 point check guardrail** (per § 5.4 — descriptive, NOT inferential, NO α correction), AND (v) is rollbackable (artifact directory deletion at the protocol level restores prior heldout PPUT — verified by an explicit rollback sub-eval, see § 6 E2). Quarantined-only artifacts (failed any of the five conditions) do NOT count toward Gate H. |

If any gate A-G fails: arc reported as negative finding, no FULL PASS, write CCL-1 negative paper.

If H fails but A-G hold: arc reported as "infrastructure works, capability compilation not yet demonstrated" — also publishable as negative result. No claim of CCL.

## 8. what would falsify

Each of the following individually FALSIFIES the central thesis as stated:

- F1: Phase C — Soft Law mode has VPPUT comparable to Full (within Bonferroni-corrected CI). Implies PPUT does not penalize fake-progress; metric is broken or mode does not actually violate constitution.
- F2: Phase C — Panopticon and Amnesia modes both at VPPUT parity with Full. Implies CPR / ERR are not load-bearing; constitutional analysis rejected.
- F3: Phase D — zero candidate artifacts pass meta-validation across full week. Implies CCL shadow loop cannot generate value; ArchitectAI is non-functional.
- F4: Phase E — heldout WBCG_PPUT ≤ 0. Implies adaptation gains do not generalize; capability compilation fails.
- F5: Any anti-Goodhart conformance test fails post-Phase B. Implies metric is exploitable; arc must restart with strengthened metering.

## 9. multiplicity (CLEAN REWRITE round 4 — aligned with § 5)

### 9.1 Family construction

```
inferential_family_size = 4 (Phase C H1-H4) + 3·k (Phase E per-artifact j-PPUT/j-FAR/j-CPR)
k_max = 10        (a priori upper bound on artifacts entering Phase E; pre-committed at A5)
N_max = 4 + 3·k_max = 34   (frozen family size for ALL Holm decisions)
```

j-RR is excluded from the family (descriptive guardrail per § 5.4). H5 (Gate H) is excluded (deterministic AND per § 5.7).

**Behavior under realized k**:
- If realized k ≤ k_max: Holm uses N_max = 34 throughout. Unused slots (positions k+1..k_max) forfeit their α budget; we are conservative.
- If realized k > k_max: arc ABORTS at end of Phase D. User may submit a formal addendum + new dual external audit raising k_max, but ALL inferential decisions in the original PREREG remain at N_max = 34. No post-hoc threshold relaxation.

### 9.2 Correction procedure: Holm-Bonferroni stepwise (a priori)

Family-wise α = 0.05.

1. Compute p-values for the 4 Phase C tests (always populated) + 3·k Phase E tests (k determined at end of Phase D).
2. Sort all populated p-values ascending: `p_(1) ≤ p_(2) ≤ ... ≤ p_(M)` where M = 4 + 3k.
3. For i = 1, 2, ..., M:
   - Threshold = `α_family / (N_max - i + 1) = 0.05 / (34 - i + 1)`.
   - If `p_(i) > threshold`: STOP. Reject hypotheses 1..i-1; accept null for i..M.
   - If `p_(i) ≤ threshold`: continue to i+1.

The denominator uses N_max = 34, not realized M. Conservative.

### 9.3 Worked example

| k | M | p_(1) threshold | p_(M) threshold |
|---|---|---|---|
| 0 (no Phase E, Phase C only) | 4 | 0.05/34 ≈ 0.00147 | 0.05/31 ≈ 0.00161 |
| 3 | 13 | 0.05/34 ≈ 0.00147 | 0.05/22 ≈ 0.00227 |
| 10 (k_max) | 34 | 0.05/34 ≈ 0.00147 | 0.05/1 = 0.05 |

### 9.4 Power expectation — Phase C (n=10 paired-binary, McNemar one-sided)

Smallest Holm threshold (worst case) = 0.05/34 ≈ 0.00147.

For sign test on n=10 paired-binary observations, one-sided exact binomial under null p=0.5:

| Discordant successes b | one-sided p-value | Significant at α = 0.00147? |
|---|---|---|
| 8/10 | 0.0547 | NO |
| 9/10 | 0.0107 | NO |
| 10/10 | 0.000977 | **YES** (0.000977 ≤ 0.00147) |

**Phase C requires 10/10 paired wins for any of H1-H4 to reject** at the smallest Holm threshold. This is a strict criterion. Implication:
- Phase C is powered for ablations that produce STRONG, CONSISTENT effects across all 10 hard problems. This is plausible for genuine constitutional violations (Soft Law / Panopticon / Amnesia should affect every adversarial-hard problem, not just some).
- Weaker effects (e.g., Mode_alt loses on 8/10 or 9/10 problems) → Phase C fails to reject → reported as NEGATIVE finding. This is the explicit pre-registered tradeoff (per `feedback_phased_checkpoint`: prefer NEGATIVE finding to N enlargement).

### 9.5 Power expectation — Phase E (n=54 paired sign test)

Smallest Holm threshold = 0.05/34 ≈ 0.00147; mid threshold (e.g. position 7 in k=3 case) ≈ 0.05/28 ≈ 0.00179; latest = 0.05.

For sign test on n=54, one-sided exact binomial under null p=0.5:

| Successes b | normal-approx z | one-sided p-value | Significant at α = 0.00147? |
|---|---|---|---|
| 35/54 | 2.04 | 0.0207 | NO |
| 36/54 | 2.31 | 0.0103 | NO |
| 37/54 | 2.59 | 0.00484 | NO |
| 38/54 | 2.86 | 0.00211 | NO |
| 39/54 | 3.13 | 0.000871 | **YES** |
| 40/54 | 3.40 | 0.000336 | **YES** |

**Phase E per-artifact j-PPUT requires ≥ 39/54 (≈72%) heldout problems improved** at the smallest Holm threshold (k_max = 10 case, where j-PPUT is at the front of the Holm sort).

For artifacts at later Holm positions (more p-values cleared earlier in the sort), thresholds relax up to 0.05; correspondingly fewer heldout improvements suffice. Realistic effect for a genuinely useful artifact (improving 39+ of 54 heldout problems) is achievable.

### 9.6 Pre-registration constraint

Family construction (tests, directions, thresholds, k_max = 10, N_max = 34) is FROZEN at Phase A5 commit gate. No tests added post-hoc. No threshold relaxation post-hoc. No k_max increase without formal addendum + dual external re-audit.

## 10. claim language constraints (revised round 2 per dual audit M7)

Per C-070 + C-052, the following claims are FORBIDDEN unless data supports at threshold:

| Claim | Threshold | Required scope qualifier |
|---|---|---|
| "TuringOS achieves capability compilation" | WBCG_PPUT > 0 on heldout (Gate H, § 7) | "on MiniF2F-Test heldout-54 with deepseek-v4-flash thinking-off + Phase D heterogeneous (v4-flash thinking-on + Gemini 2.5 Pro)" |
| "PPUT detects Soft Law / Panopticon / Amnesia constitutional violations" | H1-H3 pass at family-corrected α (§ 9) | "on deepseek-v4-flash thinking-off; replication on heterogeneous LLM stack deferred to future arc" |
| "Heterogeneous skill-prompt swarm beats homogeneous swarm" | H4 passes at family-corrected α | "on deepseek-v4-flash thinking-off, hard-10 adaptation set" (paper-1-style scope) |
| "Architect AI generates white-box assets" | ≥ 1 artifact passes meta_val (Quarantined) | "in shadow mode; heldout certification deferred to Phase E" |
| "Artifact j contributes ΔPPUT = X" or "Artifact j is responsible for X% of capability gain" (round 3 fix per Codex round-2 M3 caveat) | Phase E LOO measurement only | "marginal contribution given the rest of Q (the other Quarantined artifacts active simultaneously); standalone efficacy of artifact j is NOT measured by this PREREG" |
| "Solve rate" used as headline | FORBIDDEN — must accompany VPPUT-M (C-052) | n/a |
| "emergence" / "swarm intelligence" | FORBIDDEN unless accompanied by causal-mechanism evidence beyond aggregate gain | n/a |
| "first / novel" | requires prior-art search with documented null result | n/a |

### 10.1 Default claim language for negative outcomes (round 2 — partial-outcome cases added)

| Outcome | Allowed claim |
|---|---|
| Phase C all H1-H4 fail | "Three constitutional ablations did not significantly affect VPPUT on hard-10 adaptation (10 per-problem units; 20 seed-level runs per condition pair) within Phase C statistical power (10/10 paired-binary wins required at family-corrected α); either ablations were under-effective or VPPUT signal too noisy at this scale" |
| Phase D zero artifacts reach Quarantined | "ArchitectAI candidate generation passed § 3 metering checks; § 3.5 + human-as-meta-predicate filter rejected all candidates as non-abstract / lookup-table-shaped; CCL pipeline at this configuration cannot generate user-space artifacts that survive content audit" |
| Phase D ≥ 1 Quarantined, Phase E zero Certified | "Phase D produced k Quarantined artifact(s) with positive ΔPPUT on meta_validation; on heldout-54 sealed eval, no artifact passed all five Certified conditions (three inferential tests j-PPUT / j-FAR / j-CPR at family-corrected α + j-RR ≤ p_0 point check + Rollbackable=1); capability-compilation gain on Phase D set did not generalize to heldout under Holm-Bonferroni correction" |
| Phase E ≥ 1 Certified — partial PASS | "Capability compilation demonstrated on `<count>` user-space artifact(s) generated by ArchitectAI from Phase B-C failure logs; certified on MiniF2F-Test heldout-54 under deepseek-v4-flash thinking-off backbone; FULL PASS Gates A-H all hold (per PREREG § 7)" |
| Phase E sealed eval but rollback test fails | "FULL PASS NOT achieved — Gate H sub-criterion (rollbackable) did not hold; artifact effects entangled with adaptation state and could not be cleanly reverted; arc reported as infrastructure-built / capability-not-cleanly-attributed" |

Default claim language for full-arc negative outcome (no Certified):
- "PPUT-driven CCL infrastructure built and metered on heldout-54; capability compilation not demonstrated within 30-day budget on the deepseek-v4-flash backbone"
- "Three constitutional ablations [replicate / fail to replicate] prior structural findings on PPUT axis"

## 11. stopping rules

Per phase:
- Gate failure → diagnose, do NOT enlarge N (feedback_phased_checkpoint)
- Anti-Goodhart conformance fail → STOP, restart phase with strengthened metering
- Heldout-leak detection → BLOCKER, arc invalidated

Wall-clock cap: 30 days from PASS/PASS on Phase A audit. Hard stop. If arc not at Phase E by day 28, accept best partial result.

API budget cap: USD 500 across the arc (Codex + Gemini + DeepSeek). Hard stop on overrun.

## 12. compute environment freeze

Per F-2026-04-22-08 (model drift) + C-068:

### 12.1 Backbone (frozen)

- Primary in-system backbone: **`deepseek-v4-flash`** with **thinking mode OFF** (per project_chat_over_reasoner: TuringOS scaffold IS the externalized CoT; reasoner-style internal CoT is a control, not the default). User directive 2026-04-25.
- DeepSeek API endpoint: https://api-docs.deepseek.com/ (1M context, max output 384K, ¥0.2 input-cache / ¥1 input-miss / ¥2 output per 1M tokens). Note: `deepseek-chat` and `deepseek-reasoner` are legacy aliases for v4-flash thinking-off and thinking-on respectively; both deprecate. We pin `deepseek-v4-flash` explicitly to avoid the alias-deprecation drift mode.
- Snapshot: capture exact v4-flash model version string + API revision at start of each Phase B-E run; record in jsonl `model_snapshot` field.
- Pre-batch smoke probe (per feedback_smoke_before_batch): single problem oneshot, must produce a SOLVED result before launching batch; > 50% deviation from baseline aborts.

### 12.2 Heterogeneous-LLM introduction point (decided 2026-04-26)

User directive 2026-04-25: introduce real heterogeneous LLMs at "appropriate time, decided by Claude". Decision:

| Phase | Configuration | Rationale |
|---|---|---|
| B | `deepseek-v4-flash` only (thinking off) | Instrumentation phase; no semantic confound |
| C | `deepseek-v4-flash` only (thinking off) | Ablation modes (Soft Law / Panopticon / Amnesia / Homogeneous / Full) must vary ONLY the constitutional axis; mixing models would confound mode-effect with model-effect |
| D | **Heterogeneous meta-loop**: ArchitectAI = `deepseek-v4-flash` thinking-ON (generator role); AuditorAI = **Gemini 2.5 Pro** (critic role, different cognitive prior) | Prevents Generator-equals-Evaluator collapse at the CCL meta-loop level (C-010). Reuses existing dual-audit Gemini infrastructure. |
| D-optional | "Real heterogeneous swarm" candidate experiment: 4× `deepseek-v4-flash` + 4× `gemini-2.5-flash` agents in same swarm | Tests whether model-level diversity reduces IAC beyond skill-prompt diversity (Paper 1 E1V2 finding). Pre-committed as candidate, not gating. |
| E | LOCK the Phase D winning configuration; no new heterogeneity at heldout | Heldout result must be deterministic; introducing new model identities at heldout time would invalidate sealed-eval discipline |

**Why not earlier**: Phase C's hypotheses H1-H4 require a pure constitutional-axis comparison. If we mix LLMs in C, a "Soft Law mode loses VPPUT" finding could be re-attributed to "Soft Law mode happens to use weaker LLM in this batch". Heterogeneity is introduced once the mode-axis is sealed.

**Why exactly Phase D**: ArchitectAI (creative generator) and AuditorAI (critical evaluator) are functionally distinct roles. C-010 (generator-equals-evaluator) explicitly warns against same-cognitive-prior in those roles. Different-LLM here is constitutional-spec-aligned, not just diversity-for-diversity.

### 12.3 External-audit backbones (unchanged from Paper 1 arc)

- Audit backbones: **Codex** (latest GPT-5-codex / GPT-5.2 family, whichever current at audit time) + **Gemini 2.5 Pro**
- Conservative merge: VETO > CHALLENGE > PASS (feedback_dual_audit_conflict)

### 12.4 Memory updates from this freeze

- `project_chat_over_reasoner.md`: append "2026-04-25 user directive: pin to `deepseek-v4-flash` thinking-off as new canonical name; old `deepseek-chat` alias deprecates"
- New memory `project_pput_ccl_arc.md`: pointer to this PREREG + architect directive archive

## 13. evidence locations (post-arc)

- All run jsonl: `experiments/minif2f_v4/logs/pput_ccl_*.jsonl`
- Per-phase checkpoint docs: `handover/audits/CHECKPOINT_PHASE_{B,C,D,E}_2026-04-*.md`
- Audit outputs: `handover/audits/{CODEX,GEMINI}_PPUT_CCL_*_AUDIT_*.md`
- Artifact directory: `user_space/Δ_*/`
- Reproducer scripts: `handover/preregistration/scripts/split_pput_ccl.py`, `handover/audits/run_gemini_pput_ccl_audit.py`, `handover/audits/run_codex_pput_ccl_audit.py`

## 14. author signature

Pre-registered AT commit time (before any data-collection run). Any deviation from this pre-reg in subsequent reports MUST be explicitly flagged in the corresponding report's methods section per C-070.

This file is committed FIRST, then split (§ 2.2) is generated, then dual external audit (§ A4) launched. Phase B does NOT begin before Phase A4 returns PASS/PASS. Do not modify after Phase A5 commit gate except via formal addendum (`PREREG_PPUT_CCL_2026-04-26_ADDENDUM_*.md`) which itself requires dual external audit.

### 14.1 Date stamp note (round 2 per Codex audit H2)

The filename and frontmatter date `2026-04-26` reflect the date of the `PPUT_CCL_SPLITS_2026-04-26.json` generation and the `GEMINI_DEEPTHINK_FULL_PASS_2026-04-26.md` archive landing. Per the running session's `currentDate` indicator, drafting began evening of `2026-04-25`; the dual external audit + round-2 revisions were completed on the same multi-hour session, with the date stamp committed forward to `2026-04-26` to match the canonical artifact suite (split JSON, audit verdict). Both refer to the same arc launch event; the one-day forward stamp is a documentation-stability convention, not a backdating issue. The actual arc start = first commit of this PREREG round-2 + Phase A5 commit gate, which is what `committed_commit_sha` captures.

## changelog

| Date | Change | Source |
|---|---|---|
| 2026-04-26 | Initial draft (architect FULL PASS / PPUT-driven). | architect 2026-04-25 |
| 2026-04-26 | § 12 backbone pinned to `deepseek-v4-flash` thinking-off; § 12.2 heterogeneous-LLM timing table (Phase D entry point). | user directive 2026-04-25 + Claude decision 2026-04-26 |
| 2026-04-26 | **Patch A** (Gemini DeepThink): § 1.7 ArtifactState 4-state machine for artifacts (Accepted / Quarantined / Certified / Reverted); § 1.6 WBCG_PPUT row + § 7 Gate H tightened to "Certified-only". Task Progress remains binary (Lean perfect predicate). | Gemini DeepThink 2026-04-26 PPUT-driven FULL PASS confirmation |
| 2026-04-26 | **Patch B**: § 1.8 Trust Root composition (kernel.rs / wal.rs / bus.rs / lean4_oracle.rs / constitution.md / cases / genesis_payload[pput_accounting_0] / splits JSON / this PREREG); § 7 Gate B upgraded to syscall-layer EPERM trap. | same |
| 2026-04-26 | **Patch C**: § 3.5 4 doc/artifact content meta-predicates (no_raw_failed_trace / no_exact_adaptation_solution with hash check / parametric_templates / scope_and_expiration). Run by AuditorAI in Phase D. Distinct attack surface from § 3 anti-Goodhart metering. | same |
| 2026-04-26 | **Patch D**: § 6 Phase D D4 expanded — `architect_sole_lt_reader.rs` + `auditor_sees_candidate_only.rs` cognitive-isolation conformance; AuditorAI sees only candidate Δ + ArchitectAI summary, never raw L_t. | same |
| 2026-04-26 | **Patch E**: § 6 Phase C — 5 ablation modes implemented as single-binary `--mode` CLI flag; `tests/mode_flag_binary_purity.rs` asserts identical git_sha / binary_sha256 / model_snapshot across modes. | same |
| 2026-04-26 | § 6 Phase D D5 added — human (`gretjia`) acts as live meta-predicate during shadow mode; defense-in-depth against Claude-only homogeneous LLM stack covert promotion-loop. Phase D produces NO Certified artifacts; only Phase E sealed-eval can mint Certified. | Gemini DeepThink confirmation 2026-04-26 + user agreement |
| 2026-04-26 (round 2) | **M1**: § 5 restructured — each H1-H4 has ONE primary endpoint (per-(problem,seed) Lean-verified Progress sign or paired VPPUT sign); H5 reclassified as deterministic gate (no α). § 5.2 added — per-artifact heldout family `4·k` tests (j-PPUT / j-FAR / j-RR / j-CPR). § 5.3 family size = `4 + 4k`. § 6 C2 independent unit clarified = (problem, seed). § 9 rewritten with Holm-Bonferroni stepwise procedure + power expectation + family construction frozen at A5. | Codex audit M1 + Gemini audit M1 (round 1 dual-CHALLENGE convergent) |
| 2026-04-26 (round 2) | **M2**: § 2.3 reframed — heldout sealing is **operational** (access-pattern + conformance), not cryptographic; SHA-256 is integrity check (tamper detection), not access control; new context-blacklist: PREREG + SPLITS files cannot enter agent prompt context; new tool-call pre-flight EPERM filter; residual paraphrasing risk acknowledged. | Codex audit M2 (round 1 LEAKAGE CHALLENGE — Gemini missed) |
| 2026-04-26 (round 2) | **M3**: § 6 E2-E3 rewritten — leave-one-out within sealed eval. `k+2` sub-evaluations on the SAME 54 problems × 3 seeds (BASELINE + ALL + k LOO). Per-artifact ΔPPUT = PPUT_all − PPUT_minus_j. Single-touch discipline preserved. Compute budget validated (~27 wall-hours at k=10, 5x headroom). | Codex audit M3 (round 1 GATE-H MEASUREMENT CHALLENGE — Gemini missed) |
| 2026-04-26 (round 2) | **M4**: § 7 Gate H — RR criterion relaxed from brittle `RR = 0` to per-problem one-sided exact binomial test against pre-registered baseline regression rate `p_0` (calibrated in Phase B6/B7 on adaptation set). Reachability calibrated to empirical reality. | Gemini audit M4 (round 1 GATE-H REACHABILITY CHALLENGE — Codex missed) |
| 2026-04-26 (round 2) | **M5**: § 6 D5 — SPOF contingency added. 48h per-candidate review SLA; SLA breach → `deferred` queue; 30-day arc clock does NOT pause for user unavailability; ≥ 5 candidates queued > 48h triggers Phase D early abort to Phase E. User keystroke logs to `human_meta_predicate_log_*.jsonl` (Phase E auto-meta-predicate Trust Data). | Gemini audit M5 + Codex audit P1 (both flagged; consolidated to P0 per conservative rule) |
| 2026-04-26 (round 2) | **M6**: § 3.5.1 — 4 additional content predicates (`docs_no_problem_id_keys`, `docs_no_theorem_name_keys`, `docs_rolling_hash_multi_window` at sizes [16/32/64/128], `docs_max_dict_cardinality ≤ 8`); 3-strikes-on-source-cluster ArchitectAI degenerate-mode notification. Closes lookup-table evasion via 63-token chunks / problem ID / theorem name keys / high-cardinality dicts. | Codex audit M6 (round 1 GOODHART CHALLENGE — P1 escalated to P0) |
| 2026-04-26 (round 2) | **M7**: § 10 rewritten — every claim row gets a "scope qualifier" column requiring `deepseek-v4-flash thinking-off` + Phase D heterogeneous specifier; § 10.1 added — partial-outcome allowed claims for 5 specific scenarios (Phase C all fail / Phase D zero Quarantined / k Quarantined zero Certified / partial PASS / rollback fail). | Codex audit M7 (round 1 CLAIM-LANG + HETEROGENEITY-TIMING scoping) |
| 2026-04-26 (round 2) | **TR**: § 7 Gate B + § 1.8 Trust Root — fallback enforcement spec added (lib-level write gate + path whitelist + post-write `git diff` + `panic!()` on bypass) for the case where Rust user-space cannot reach syscall-level EPERM. Either primary or fallback enforcement passing the conformance suite is sufficient for Gate B. | Codex audit TR (round 1 TRUST-ROOT-ENFORCEMENT CHALLENGE) |
| 2026-04-26 (round 2) | **H1**: heldout-49 → heldout-54 grep-and-replace through PREREG + notepad. Bucket-pseudocode comments updated to "nominal X / realized Y". | Codex audit H1 (round 1 REPRO PASS-with-cleanup) |
| 2026-04-26 (round 2) | **H2**: § 14.1 added — date stamp note explaining 2026-04-25 drafting / 2026-04-26 stamp convention. | Codex audit H2 (round 1 REPRO PASS-with-cleanup) |
| 2026-04-26 (round 3) | **R3-1**: § 9 rewritten with hard-cap k_max = 10 / N_max = 44; Holm thresholds use N_max throughout (Gate C decisions made before k is known become statistically sound); arc aborts if realized k > 10 (no post-hoc relaxation). § 6 C2 + § 9.2 independent unit changed from (problem, seed) to per-problem with seeds as noise-reducing replicates; n = 10 paired-binary observations for hard-10 McNemar. | Codex round-2 audit P0-fam (still-open) |
| 2026-04-26 (round 3) | **R3-2**: § 5.2 p_0 calibration protocol fully specified — toggle (`--simulate-rollback-at-tx-50`), sample (full adaptation-144 × 2 seeds = 288 paired runs), estimator (per-problem worst-case across seeds), schedule (Phase B7 mandatory before Phase D), freeze (genesis_payload.toml + Trust Root SHA-256 of calibration jsonl), ceiling (p_0 > 0.10 = ABORT), audit (dual-audit at Phase B → C transition). Closes round-2 tuning-surface concern. | Codex round-2 audit P0-gate-h (partial) |
| 2026-04-26 (round 3) | **R3-3**: § 2.3 operational sealing rewritten with 5 layers (L1 file-path / L2 prompt-context / L3 tool-call args / L4 hashlib gate / L5 MiniF2F/Test ID blacklist). Seed-string leak vector (architect insight files, audit echoes) closed via L2/L3. Hashlib-with-seed dynamic invocation closed via L3/L4. Source pool enumeration closed via L5. Honest residual risk acknowledged (git history). | Codex round-2 audit P0-leak (partial) |
| 2026-04-26 (round 3) | **R3-4**: § 10 added marginal-contribution claim caveat — per-artifact ΔPPUT from LOO is conditional on Q\{j} being active, NOT a standalone-efficacy measurement. Closes Codex round-2 P0-measure claim caveat. | Codex round-2 audit P0-measure (closed-with-caveat) |
| 2026-04-26 (round 4) | **R4-1**: § 5 + § 9 CLEAN REWRITE — patch-stacking from rounds 1-3 had left inconsistencies (some sections still said `(problem, seed)` n=20, others said per-problem n=10). Round 4 single source of truth: per-problem unit everywhere (n=10 Phase C, n=54 Phase E). Power tables in § 9.4 + § 9.5 corrected (Phase C requires 10/10 paired wins; Phase E ≥ 39/54). | Codex round-3 audit P0-fam (still-open) — internal inconsistency caught |
| 2026-04-26 (round 4) | **R4-2**: § 5.4 j-RR moved out of inferential family (was mathematically unwinnable: 0.9^54 ≈ 0.00343 > 0.05/44 ≈ 0.00114; even zero regressions failed to reject). Now a descriptive guardrail: point check `RR_j ≤ p_0`, no α correction. Family size shrinks from 4+4k to 4+3k; N_max from 44 to 34. § 1.6 + § 1.7 + § 7 Gate H synced to "3 inferential + 1 guardrail + rollback" framing. | Codex round-3 audit P0-gate-h (partial; mathematical impossibility) |
| 2026-04-26 (round 4) | **R4-3**: § 2.3 L3/L4/L5 generalized — L3 detects seed substring + concatenation patterns + broad shell hash invocations (sha256sum/sha1sum/md5sum/openssl dgst/hashlib python -c/etc.); L4 broad hash function blacklist (Python hashlib/cryptography/Crypto, Rust sha2/sha1/blake2/blake3/md5/digest/ring/openssl, JS crypto/SubtleCrypto/js-sha256, all shell binaries) plus SHA-256 IV-constant detection for manual implementations; L5 path enumeration block on MiniF2F/Test directory (ls/find/rg --files/glob.glob/os.listdir all blocked). | Codex round-3 audit P0-leak (partial; literal-only defense) |
| 2026-04-26 (round 4) | **R4 internal-consistency sweep**: changelog rounds 1-3 all referenced family `4+4k`; round 4 changes to `4+3k` are the canonical source. Earlier `(problem, seed)` references in round 1-2 changelog rows are historical context, not current spec. | self-audit |
