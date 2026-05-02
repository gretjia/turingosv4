# Codex TB-7R Ship Audit — Round 2 (post-remediation re-check)
**Date**: 2026-05-02
**Range**: `9e74195..4470036` + working-tree remediation
**HEAD**: 4470036876d06d794036a1818d0331692046a482
**Workspace test count**: 712 / 0 / 150 (unchanged from round 1)
**Audit class**: Class 3 (auth-crypto-money) — round-2 per `feedback_elon_mode_policy` round-cap
**Auditor**: Codex (implementation-paranoid; same as round 1)
**Prompt size**: 36432 chars

---

Reading prompt from stdin...
OpenAI Codex v0.128.0 (research preview)
--------
workdir: /home/zephryj/projects/turingosv4
model: gpt-5.5
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: xhigh
reasoning summaries: none
session id: 019de78b-f79d-7501-891d-3e29b69e2cba
--------
user
# Codex TB-7R Ship Audit — ROUND 2 (post-remediation re-check)

**Role**: same skeptical adversarial implementer-reviewer as round 1.

**Mandate**: round-2 ship audit per `feedback_elon_mode_policy` after determinate-best surgical remediation of round-1 VETO. Confirm whether the four explicit round-1 findings are closed. NOT a re-do of round 1 — focus on the deltas.

You returned **VETO / HIGH conviction** in round 1 (`handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md`). The remediation since round 1 is uncommitted (working tree). HEAD is still `4470036`.

## Round-1 findings + remediation summary

| Round-1 finding | Severity | Remediation |
|---|---|---|
| #1 Evidence packaging: `runtime_repo/.git/` + `cas/.git/objects/` not committed; only sidecar JSON. CasStore::get fails on committed-only state. README claims `.git` present. | VETO basis | Each of 10 evidence dirs now ships `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` (compressed `.git/` stores; ~12KB each, total 892KB across 10 runs). README §0 + §4 + §5.1 updated to reflect new packaging + add reproduction recipe. Round-trip verified: extract tar.gz pair into a fresh dir + `verify_chaintape` produces structurally identical replay_report.json to the committed one. |
| #2 No `replay_report.json` per run | constitutional debt #3 in round 1 | Each evidence dir now ships `replay_report.json` (literal `verify_chaintape` output: all 9 boolean indicators true; `l4_entries` + `l4e_entries` counts; `final_state_root_hex` + `final_ledger_root_hex` + `head_commit_oid_hex`). |
| #3 PartialOk → Complete proof-prefix dependency: accepted L4 WorkTx `proof_artifact_cid` resolves to `tactic` only, but verify_partial uses `tape_chain + tactic`. Replay can't reconstruct full proof. | CHALLENGE (independent of VETO) | OBS-1 (`OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`) §2.1 added with sharper framing per your Q2 quote. §4 hardening recommendation extended with explicit "self-contained proof artifact" requirement (§4.3). Deferred to TB-8+ per architect verdict A1=B′ (per-tactic decomposition is TB-8+ scope). NOT touched in code. |
| #4 OBS-2 premise stale: `acc.record_tool_stdout` only increments token cost via `cost_aggregator.rs:57`; raw Lean text doesn't flow to prompt. | (your Q10 PASS w/ correction) | OBS-2 (`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`) §0 added: explicit "premise empirically unfounded per Codex round-1 Q10" correction. Closed-as-empirically-unfounded. |

## What changed since round 1 (working tree, uncommitted)

```text
M  handover/evidence/tb_7r_smoke_2026-05-02/README.md      — §0 packaging update + §4 artifact list rewrite + §5.1 reproduce recipe
M  handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md  — §2.1 + §4.3 added
M  handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md  — §0 closure-as-stale-premise added
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/runtime_repo.dotgit.tar.gz
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/cas.dotgit.tar.gz
A  handover/evidence/tb_7r_smoke_2026-05-02/<10 runs>/replay_report.json
```

No code changes. `cargo test --workspace` still 712/0/150.

## Round-2 audit questions (4; targeted re-check of round-1 VETO basis)

**RQ1. Packaging completeness**: pick 2 random evidence dirs (e.g. `single_n1_mathd_algebra_171` and `full_5_problems_n1/run_4_aime_1997_p9` — pick 1 SOLVED + 1 UNSOLVED). For each:
- Confirm `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` exist and are non-empty.
- Extract them into a fresh tempdir (along with the sidecar JSONs as in §5.1).
- Run `target/debug/verify_chaintape --repo $WORK/runtime_repo --cas $WORK/cas --out $WORK/r.json`.
- Confirm the resulting `r.json` has all 9 boolean indicators true.
- Compare to the committed `replay_report.json` (modulo `run_id` + `epoch` which are runtime-tagged).

If both runs round-trip cleanly → RQ1 closed. If any indicator flips false or any tar.gz is corrupted → RQ1 fails.

**RQ2. Round-1 finding #1 closure** (clause 4 + ship cond #5): given RQ1 outcome, can a third-party auditor with only this repo + a built `target/debug/verify_chaintape` independently reproduce the chain_oracle_verified claim for any of the 10 runs? This is the strict reading of acceptance clause 4. If yes → finding #1 closed. If no → identify the residual gap.

**RQ3. CID resolution from committed state** (was VETO sub-finding on Q1): pick one accepted L4 Work entry from any solved run. From the extracted tempdir, demonstrate that the WorkTx's `proposal_cid` (read from CAS sidecar `.turingos_cas_index.jsonl`) resolves through `CasStore::get(cid)` to a non-empty payload. Then walk `proposal_telemetry.verification_result_cid` → `VerificationResult.proof_artifact_cid` chain, confirming each CID resolves. (One run is sufficient — you found 1 in-scope L4 Work in the L4 purity audit; that pattern repeats.)

**RQ4. OBS framing tightness** (round-1 finding #3 + #4):
- Read OBS-1 §2.1 + §4.3. Does the new §2.1.a ("PartialOk → Complete proof-prefix dependency") accurately capture what you found in round-1 Q2? Does §4.3 ("self-contained proof artifact") propose a sufficient post-TB-7R fix?
- Read OBS-2 §0. Is the closure-as-empirically-unfounded language correct given your round-1 Q10 finding? Or does it overstate / understate the conclusion?

## Verdict format (round 2)

Section A: Round-2 verdict (PASS / CHALLENGE-but-clearable / VETO).
- PASS = round-1 VETO is closed; ship may proceed.
- CHALLENGE-but-clearable = remaining items are non-VETO and have clear written remediations; ship may proceed with explicit OBS carry-forward.
- VETO = round-1 VETO not closed (or new VETO surfaced by remediation).

Section B: Per-RQ1-RQ4 disposition (one paragraph each + verdict tag + cite file:line / commit / shell-result).

Section C: Round-1 → round-2 finding closure table (4 rows: #1, #2, #3, #4).

Section D: Recommendation. Specifically:
- If PASS → state explicitly that round-1 VETO is closed.
- If CHALLENGE-but-clearable → list which OBS need final-form text changes before ship.
- If VETO → state what additional remediation is required.

The remediation is uncommitted in working tree. You can run `git status`, `git diff`, and any reproduction commands. Sandbox is `danger-full-access`.



---

# XREF: working tree status (post-remediation)

```
 M h_vppu_history.json
 M handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
 M handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/chain_derived_run_facts.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M handover/evidence/tb_7r_smoke_2026-05-02/README.md
 M rules/enforcement.log
?? handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md
?? handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md
?? handover/audits/run_codex_tb_7r_round_2_audit_2026-05-02.sh
?? handover/audits/run_codex_tb_7r_ship_audit_2026-05-02.sh
?? handover/audits/run_gemini_tb_7r_ship_audit_2026-05-02.py
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.dotgit.tar.gz

```


---

# XREF: README diff (handover/evidence/tb_7r_smoke_2026-05-02/README.md)

```diff
diff --git a/handover/evidence/tb_7r_smoke_2026-05-02/README.md b/handover/evidence/tb_7r_smoke_2026-05-02/README.md
index c5bef62..222a035 100644
--- a/handover/evidence/tb_7r_smoke_2026-05-02/README.md
+++ b/handover/evidence/tb_7r_smoke_2026-05-02/README.md
@@ -27,6 +27,8 @@
 
 **Aggregate**: 8/10 SOLVED + chain_oracle_verified=true; 2/10 UNSOLVED + chain_oracle_verified=false. **All 7 indicators GREEN on every run.** **No fake accepted node** in any UNSOLVED run.
 
+**2026-05-02 packaging update** (post Codex ship-audit round-1 VETO on evidence reproducibility): each run subdirectory now ships with self-contained `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + `replay_report.json`. Total committed evidence size = **892 KB** (10 runs combined; loose .git stores would be 4.8 MB but git auto-ignores nested .git directories — tar.gz keeps the artifacts tracked while compressing 18× smaller than loose objects). Each `replay_report.json` is the literal output of `target/debug/verify_chaintape --repo <run>/runtime_repo --cas <run>/cas --out replay_report.json` (run after extracting the two tar.gz files) — every run shows all 9 boolean indicators true (`ledger_root_verified`, `system_signatures_verified`, `state_reconstructed`, `economic_state_reconstructed`, `cas_payloads_retrievable`, `agent_signatures_verified`, `proposal_telemetry_cas_retrievable`, plus the existing `l4_entries`/`l4e_entries` counts). Acceptance clause 4 ("dashboard regeneratable from ChainTape + CAS alone") is now strictly satisfied from committed evidence; ship condition #5 ("proposal telemetry + payload CIDs resolve") is independently checkable by any auditor via §5.1 below. Round-trip verified: extracting the tar.gz pair into a fresh dir and re-running `verify_chaintape` produces a structurally identical replay_report.json (modulo runtime-tagged `run_id`/`epoch`).
+
 ---
 
 ## §1 Verdict §F acceptance — claim by claim
@@ -178,30 +180,80 @@ The "every failed proposal" clause is satisfied for the failed-proposals that ac
 
 ## §4 Per-run artifacts
 
-Each run subdirectory contains:
+Each run subdirectory contains (committed to git after 2026-05-02 packaging update):
 
 ```text
 runtime_repo/
-  genesis_report.json    — TB-7R Deliverable C
-  initial_q_state.json   — TB-7.7 D7 (preseeded balances + task state)
+  genesis_report.json            — TB-7R Deliverable C
+  initial_q_state.json           — TB-7.7 D7 (preseeded balances + task state)
   agent_audit_trail.jsonl
   agent_pubkeys.json
   pinned_pubkeys.json
-  rejections.jsonl       — L4.E records
+  rejections.jsonl               — L4.E records (raw_diagnostic shielded per Art. III.4)
   synthetic_rejection_label.json
-  (.git refs/transitions/main = L4 chain)
+runtime_repo.dotgit.tar.gz       — compressed `.git/` containing refs/transitions/main + chain commit objects (LedgerEntry blobs); ~12 KB; expand into runtime_repo/ to enable replay
 cas/
-  .git/                  — CAS git store
-  .turingos_cas_index.jsonl  — sidecar index (TB-7.6 atomic-write)
-stdout                   — PPUT_RESULT JSON line
-stderr                   — RUST_LOG=warn output
-dashboard.txt            — `audit_dashboard --repo runtime_repo --cas cas` output
+  .turingos_cas_index.jsonl      — sidecar index (TB-7.6 atomic-write)
+cas.dotgit.tar.gz                — compressed `.git/` containing CAS payload blobs (proposal payloads + ProposalTelemetry + VerificationResult + agent audit trail records); ~12 KB; expand into cas/ to enable CID resolution
+replay_report.json               — literal `verify_chaintape` output (9 boolean indicators true + l4/l4e counts + final_state_root_hex + final_ledger_root_hex + head_commit_oid_hex)
+stdout                           — PPUT_RESULT JSON line
+stderr                           — RUST_LOG=warn output
+dashboard.txt                    — `audit_dashboard --repo runtime_repo --cas cas` output
+```
+
+**Why tar.gz instead of loose `.git/` directories**: git automatically treats nested `.git/` directories as submodule sentinels and refuses to track their contents (would otherwise require submodule registration with external remotes). Compressed-archive packaging keeps the chain stores fully tracked under the parent repo's history while compressing 18× smaller than loose objects (892 KB total for 10 runs vs ~4.8 MB loose).
+
+Per-run replay_report indicator counts:
+```text
+SOLVED runs (8):   l4_entries=3 (TaskOpen + EscrowLock + accepted Work), l4e_entries=3 (synthetic seeds + zero-stake rejections)
+UNSOLVED runs (2): l4_entries=2 (TaskOpen + EscrowLock; no accepted Work — chain_oracle_verified=false), l4e_entries=2
+```
+
+Per-run replay_report indicator counts:
+```text
+SOLVED runs (8):   l4_entries=3 (TaskOpen + EscrowLock + accepted Work), l4e_entries=3 (synthetic seeds + zero-stake rejections)
+UNSOLVED runs (2): l4_entries=2 (TaskOpen + EscrowLock; no accepted Work — chain_oracle_verified=false), l4e_entries=2
 ```
 
 ---
 
 ## §5 Reproduce
 
+### §5.1 Verify committed evidence (no LLM / Lean required)
+
+Any auditor with this repo + a built `target/debug/{verify_chaintape,audit_dashboard}` can independently verify any of the 10 runs from committed evidence alone:
+
+```bash
+RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171  # or any run dir
+
+# Stage a working copy with .git stores expanded:
+WORK=/tmp/audit_$$
+mkdir -p $WORK/runtime_repo $WORK/cas
+cp $RUN/runtime_repo/*.json $RUN/runtime_repo/*.jsonl $WORK/runtime_repo/
+cp $RUN/cas/.turingos_cas_index.jsonl $WORK/cas/
+tar -xzf $RUN/runtime_repo.dotgit.tar.gz -C $WORK/runtime_repo
+tar -xzf $RUN/cas.dotgit.tar.gz -C $WORK/cas
+
+# Re-derive replay_report.json from committed ChainTape + CAS:
+target/debug/verify_chaintape \
+  --repo $WORK/runtime_repo \
+  --cas  $WORK/cas \
+  --out  $WORK/replay_report.audit.json
+
+# Compare to committed (byte-identical modulo run_id + epoch):
+jq -S 'del(.run_id,.epoch)' $RUN/replay_report.json    > /tmp/orig.json
+jq -S 'del(.run_id,.epoch)' $WORK/replay_report.audit.json > /tmp/repro.json
+diff /tmp/orig.json /tmp/repro.json && echo "STRUCTURALLY IDENTICAL"
+
+# Re-derive dashboard from committed ChainTape + CAS:
+target/debug/audit_dashboard --repo $WORK/runtime_repo --cas $WORK/cas > $WORK/dashboard.audit.txt
+diff $RUN/dashboard.txt $WORK/dashboard.audit.txt   # semantic content identical (timestamps may differ)
+```
+
+This satisfies acceptance clause 4: every dashboard report is regeneratable from committed ChainTape + CAS alone.
+
+### §5.2 Generate fresh evidence (LLM + Lean required)
+
 ```bash
 mkdir -p /tmp/tb7r_repro/{runtime_repo,cas}
 TURINGOS_CHAINTAPE_PATH=/tmp/tb7r_repro/runtime_repo \
@@ -213,6 +265,7 @@ MAX_TRANSACTIONS=20 \
 target/debug/evaluator mathd_algebra_171.lean
 
 target/debug/audit_dashboard --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas
+target/debug/verify_chaintape --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas --out /tmp/tb7r_repro/replay_report.json
 ```
 
 ---

```


---

# XREF: OBS-1 diff (OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md)

```diff
diff --git a/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md b/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
index 4b746b0..dce3cfb 100644
--- a/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
+++ b/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
@@ -46,9 +46,47 @@ action that Lean accepts, only the Complete branch fires — and that's
 chained. For harder problems where the LLM emits intermediate `step`
 actions:
 - PartialOk → goes to kernel.tape (shadow_only) but NOT chain
-- Reject → goes to in-memory counter, AND the raw `reason` flows back
-  into the next prompt via `acc.record_tool_stdout(&reason)` (see
-  `OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`)
+- Reject → goes to in-memory counter (token-cost only; see §2.1 update)
+
+### §2.1 Codex round-1 ship-audit refinement (2026-05-02)
+
+Codex round-1 ship audit Q2/Q10 surfaced a sharper framing of the gap and
+corrected one premise:
+
+**(a) PartialOk → Complete proof-prefix dependency** (Codex Q2 CHALLENGE,
+new constitutional debt #2): the issue is not just "PartialOk is unchained
+LLM activity," but specifically that a later `Complete` action verifies
+`tape_chain + tactic` (`evaluator.rs:2132`) while storing **only `tactic`**
+as the WorkTx artifact (`evaluator.rs:2222`). This means an accepted L4
+WorkTx's `proof_artifact_cid` resolves to a `tactic` payload that, on its
+own, is not Lean-verifiable — the reconstructable proof depends on
+prior unchained PartialOk steps in `kernel.tape`.
+
+Replay from L4 + CAS alone CAN reconstruct the QState + EconomicState +
+predicate_results (`replay_report.json` shows this on every TB-7R smoke
+run). What replay CANNOT reconstruct from L4 + CAS alone is the *Lean
+proof object* that justified the predicate-pass — that proof object is
+the tape_chain prefix that lived in unchained kernel.tape during the run.
+
+Under TB-7R's strict three-node taxonomy + verdict A1=B′ (proposal-level
+DAG, per-tactic deferred), this is *internally consistent* because
+"externalized" is defined as `submit_typed_tx`-routed and the WorkTx
+*acceptance signal* is on chain even if the proof *artifact* is partial.
+But under the broader Tape Canonical Axiom (Art. 0.2) + acceptance
+clause 2 ("predicate evidence resolves from CAS"), the proof artifact
+should be self-contained.
+
+**(b) Reject path correction** (Codex Q10 PASS for prompt isolation but
+clarifying premise): `PartialVerdict::Reject` records a bounded rejection
+class label via `bus.record_rejection(agent_id, class.label())` plus
+`acc.record_tool_stdout(&reason)` for token-cost accounting only
+(`cost_aggregator.rs:57`). The raw `reason` does NOT flow into the next
+prompt — `prompt_builder` reads bounded class labels from
+`evaluator.rs:1344` and `bus.rs:576`, NOT raw Lean stderr. This means
+the original OBS-2 (`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`)
+had a stale premise: there is no Art. III.4 prompt-pollution risk; only
+the Reject side of the coverage-denominator question (raw oracle output
+not reaching chain even as L4.E rejection evidence) remains.
 
 ## §3 Why this is post-TB-7R
 
@@ -74,13 +112,23 @@ A future TB (TB-7.5? TB-8?) should:
 2. Re-route `PartialVerdict::Reject` through `submit_typed_tx` with
    `predicate_passes=false`, landing in L4.E with
    `rejection_class = LeanFailed` and `raw_diagnostic_cid` shielded.
-3. Verify the strict invariant: every LLM tool-call action that runs
+3. **Self-contained proof artifact**: when a `Complete` action follows
+   prior PartialOk steps, the accepted WorkTx's `proof_artifact_cid`
+   must resolve to the *full reconstructable proof* (tape_chain + tactic),
+   not just the final tactic. Either store the full prefix on chain via
+   the §4.1 reroute, or store the concatenated `tape_chain + tactic`
+   blob in CAS as the WorkTx artifact and update verify_partial to
+   verify against that committed blob alone. (Codex round-1 §2.1.a)
+4. Verify the strict invariant: every LLM tool-call action that runs
    Lean (or any oracle) must produce exactly one chain entry — either
    L4 accepted or L4.E rejected — never an unchained tool_dist counter
-   bump.
+   bump. AND every accepted L4 WorkTx's proof artifact must be
+   Lean-verifiable from CAS alone (no kernel.tape dependency).
 
 The Sequencer's existing `apply_one` + `predicate_results` machinery
-already supports this; the change is at the evaluator dispatch site.
+already supports the routing change in §4.1+§4.2; the proof-self-containment
+change in §4.3 is at the evaluator dispatch site (or at the Lean proof
+serialization site within the `complete` tool).
 
 ## §5 Conformance criterion (post-implementation)
 

```


---

# XREF: OBS-2 diff (OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md)

```diff
diff --git a/handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md b/handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
index 7b53a45..9d742b0 100644
--- a/handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
+++ b/handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
@@ -2,7 +2,38 @@
 
 **Class**: Observation (constitutional risk; architect verdict-silent)
 **Driver**: Claude self-assessment 2026-05-02 ultrathink turn surfaced this; architect verdict 2026-05-02 did NOT explicitly address.
-**Status**: ACTIVE — flagged for future architect ruling. NOT a TB-7R blocker per current verdict.
+**Status**: **CORRECTED — premise stale per Codex round-1 ship-audit Q10 (2026-05-02).** No Art. III.4 prompt-pollution risk; original concern is empirically unfounded. Retained for traceability + closure path. **Not a TB-7R blocker (and not a future-ruling required item either).**
+
+---
+
+## §0 Correction (2026-05-02 post Codex round-1)
+
+Codex round-1 ship audit Q10 verified the actual code paths and found:
+
+> "I found no concrete cross-agent raw Lean error pollution in full-n5
+> evidence; it contains no prompt/stdout transcript and only two synthetic
+> audit-trail rows. Code also contradicts the OBS premise:
+> `acc.record_tool_stdout` only increments token cost
+> (`cost_aggregator.rs:57`). Prompt errors come from bounded class labels
+> via `evaluator.rs:1344` and `bus.rs:576`, not raw Lean text."
+
+The remainder of this document (§1–§7 below) is preserved as the
+original concern + closure-path narrative, but the central premise —
+that `acc.record_tool_stdout(&reason)` flows raw Lean diagnostics into
+prompt context — is **incorrect**. Actual data flow:
+
+1. `PartialVerdict::Reject(reason)` → `bus.record_rejection(agent_id, class.label())` (bounded label, not raw)
+2. `PartialVerdict::Reject(reason)` → `acc.record_tool_stdout(&reason)` (token-cost accounting via `cost_aggregator.rs:57`; **does NOT write to prompt errors_history**)
+3. Next-iteration prompt errors come from `prompt_builder` reading the bounded class labels at `evaluator.rs:1344` and `bus.rs:576` — never the raw `reason`
+
+Because the raw `reason` is bounded to token-cost accounting, there is no
+agent-to-agent diagnostic leakage path even in multi-agent runs.
+Art. III.4 selective-broadcasting / shielding is honored.
+
+**Closure**: this OBS is closed-as-empirically-unfounded. The
+`coverage_denominator` OBS (companion file) absorbs the *Reject path
+not reaching chain* concern, which is a coverage question, not a
+prompt-pollution question.
 
 ---
 

```


---

# XREF: round-1 audit (your prior verdict)

**A. Overall**

**VETO, HIGH conviction.** The implementation mostly routes the intended production paths correctly, and `cargo test --workspace` really passes **712 / 0 / 150**. But TB-7R ship evidence is not reproducible from committed ChainTape + CAS: the committed smoke dirs omit the git-backed `runtime_repo/.git` ledger and `cas/.git` blob stores while the README claims they are present at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:192) and [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:194). Only `.turingos_cas_index.jsonl` is tracked for each CAS dir. A copied replay of committed `single_n1` regenerated an empty-chain dashboard, because missing git repos are initialized as empty by [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:659) and [transition_ledger.rs](/home/zephryj/projects/turingosv4/src/bottom_white/ledger/transition_ledger.rs:663). That violates acceptance clause 4 and ship condition #5.

**B. Q1-Q10**

**Q1: VETO.** The dashboard snapshots claim 8 solved accepted Work entries and `chain_oracle_verified=true` for those runs, e.g. [dashboard.txt](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/dashboard.txt:58) and [dashboard.txt](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/dashboard.txt:68). The CAS sidecar has relevant metadata, e.g. payload/VR/telemetry entries at [`.turingos_cas_index.jsonl`](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl:8), [line 9](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl:9), and [line 10](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl:10). But `CasStore::get()` requires the git blob object by backend OID at [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:219). Those blobs are not committed, so direct CID resolution cannot be confirmed.

**Q2: CHALLENGE.** `Complete` routes through CAS + `submit_typed_tx` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2222) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2340). `Reject` only records a bounded rejection label plus cost accounting at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2430). However `PartialOk` appends to kernel tape only at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2406), and a later `Complete` verifies `tape_chain + tactic` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2132) while storing only `tactic` as the Work/VR artifact at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2222). No Reject-to-L4 path found, but OBS-1 is under-bounded for multi-step PartialOk→Complete.

**Q3: PASS.** ChainTape gate is called before dispatch at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:327). It fail-closes `oneshot` under `TURINGOS_CHAINTAPE_PATH` at [chaintape_mode_gate.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/chaintape_mode_gate.rs:32). OMEGA full and per-tactic paths are wired through `submit_typed_tx` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1898) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2340). New `b517ae5..4470036` submit sites are test-only.

**Q4: VETO.** The single-run `genesis_report.json` has all nine fields and the constitution hash matches `sha256sum constitution.md`, see [genesis_report.json](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo/genesis_report.json:2) and [genesis_payload.toml](/home/zephryj/projects/turingosv4/genesis_payload.toml:119). But exact `task_open_tx` / `escrow_lock_tx` linkage to real L4 entries cannot be verified from committed evidence because the L4 git chain is absent.

**Q5: VETO.** Production preseed correctly submits TaskOpen/EscrowLock through `submit_typed_tx` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:857) and [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:902). Direct economic inserts found only in sequencer transition apply at [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:734) and [sequencer.rs](/home/zephryj/projects/turingosv4/src/state/sequencer.rs:780). But TB-7R smoke has no committed `replay_report.json`, and committed replay cannot reconstruct the claimed L4 state.

**Q6: PASS with note.** The six parent_tx tests are real synthetic fixtures, not stubs. Test 2 sets `Some(attempt_1_id)` at [tb_7r_parent_tx_conformance.rs](/home/zephryj/projects/turingosv4/tests/tb_7r_parent_tx_conformance.rs:289). Test 3 deliberately sets `None` on attempt 2 at [tb_7r_parent_tx_conformance.rs](/home/zephryj/projects/turingosv4/tests/tb_7r_parent_tx_conformance.rs:356). Detection is chain-derived from L4/L4.E + CAS, not dashboard-only, via [chain_derived_run_facts.rs](/home/zephryj/projects/turingosv4/src/runtime/chain_derived_run_facts.rs:189) and [chain_derived_run_facts.rs](/home/zephryj/projects/turingosv4/src/runtime/chain_derived_run_facts.rs:222). It observably flags; it does not sequencer-reject.

**Q7: VETO.** The README claims all 10 runs green at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:28), and the wording for natural `parent_tx_edges=0` is explicit at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:118). But no `replay_report.json` files exist in TB-7R smoke, and the artifact list omits them at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:181). The dashboard snapshots cannot substitute for committed replayable ChainTape + CAS.

**Q8: PASS.** No diff to `constitution.md`, `src/state/typed_tx.rs`, RootBox, predicate registry, or historical `genesis_report.json` backfills. Existing `FinalizeRewardTx`/slash references predate this range; TB-7R did not add new `TypedTx` variants. Historical 2026-05-01 evidence has no fabricated `genesis_report.json`.

**Q9: PASS.** Re-ran canonical `cargo test --workspace`: **712 passed, 0 failed, 150 ignored**. The TB-7R range adds no `#[ignore]` annotations. The new executable coverage includes the 6 parent_tx conformance tests at [tb_7r_parent_tx_conformance.rs](/home/zephryj/projects/turingosv4/tests/tb_7r_parent_tx_conformance.rs:189), plus new gate/genesis/CAS/VR/telemetry/replay tests.

**Q10: PASS / OBS stands.** I found no concrete cross-agent raw Lean error pollution in full-n5 evidence; it contains no prompt/stdout transcript and only two synthetic audit-trail rows at [agent_audit_trail.jsonl](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo/agent_audit_trail.jsonl:1). Code also contradicts the OBS premise: `acc.record_tool_stdout` only increments token cost at [cost_aggregator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs:57). Prompt errors come from bounded class labels via [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1344) and [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:576), not raw Lean text.

**C. New Constitutional Debt**

1. [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:192): evidence claims committed `.git` ChainTape/CAS stores, but they are absent. Remediate by archiving the git object stores or exporting a replayable ledger/CAS bundle, then regenerate dashboards from the committed bundle.

2. [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2406): `PartialOk` is unchained but can become proof context for a later accepted `Complete`. Remediate by routing PartialOk through ChainTape or by storing/verifying the full prefix artifact in the accepted WorkTx.

3. [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:181): TB-7R smoke lacks per-run `replay_report.json`. Remediate by committing replay reports produced from the committed evidence, not from ephemeral `/tmp` stores.

**D. OBS Review**

OBS-1 bounding does **not** fully stand. Reject remains unchained and does not reach L4, but PartialOk can feed an accepted downstream Complete through kernel tape. Severity: **CHALLENGE** independent of the evidence-packaging VETO.

OBS-2 bounding stands for TB-7R smoke. No cross-agent raw diagnostic leak is demonstrated, and current code stores raw reason only as token-cost accounting, not prompt context. The OBS should be corrected because its `acc.errors_history` premise is stale.

**E. Recommendation**

**VETO / revise.** Do not ship TB-7R on the current artifact set. First produce replayable smoke evidence with committed/restorable ChainTape and CAS stores, add `replay_report.json` per run, and rerun the dashboard from those committed artifacts. Then tighten or explicitly defer the PartialOk→Complete non-self-contained proof path.


---

# XREF: ls -la of one evidence dir (single_n1_mathd_algebra_171)

```
total 52
drwxr-xr-x 4 zephryj zephryj  4096 May  2 07:13 .
drwxr-xr-x 6 zephryj zephryj  4096 May  2 07:14 ..
drwxr-xr-x 2 zephryj zephryj  4096 May  2 07:13 cas
-rw-r--r-- 1 zephryj zephryj 11827 May  2 07:13 cas.dotgit.tar.gz
-rw-r--r-- 1 zephryj zephryj  3885 May  2 06:21 dashboard.txt
-rw-r--r-- 1 zephryj zephryj   792 May  2 07:07 replay_report.json
drwxr-xr-x 2 zephryj zephryj  4096 May  2 07:13 runtime_repo
-rw-r--r-- 1 zephryj zephryj 12485 May  2 07:13 runtime_repo.dotgit.tar.gz

```


# XREF: replay_report.json from single_n1

```json
{
  "l4_entries": 3,
  "l4e_entries": 3,
  "ledger_root_verified": true,
  "system_signatures_verified": true,
  "state_reconstructed": true,
  "economic_state_reconstructed": true,
  "cas_payloads_retrievable": true,
  "agent_signatures_verified": true,
  "proposal_telemetry_cas_retrievable": true,
  "run_id": "tb7r-smoke-1",
  "epoch": 1,
  "detail": {
    "final_state_root_hex": "e5199d29e268c057f197ce8492e02429ab7af0046cda510cc08bf55efc4f7647",
    "final_ledger_root_hex": "68bd2fe6a0b9c394151fa217da3d73087de75d4f894db208ac7376243661fb69",
    "head_commit_oid_hex": "d4d704beb5ab16782425f75194cae07d83842f5c",
    "l4e_last_hash_hex": "d3006f954373cefa3dfc29edb8880b91f787b572d7947e5ee00cb9a149e7973f",
    "replay_failure": null,
    "initial_q_state_loaded_from_disk": true
  }
}
```


---

Now give your INDEPENDENT round-2 audit. Be direct. Cite file:line / shell-result. Verdict: PASS / CHALLENGE-but-clearable / VETO.

codex
I’ll do this as a targeted re-check, not a fresh ship audit: first load the repo instructions, then verify the two replay bundles and one CID chain from extracted committed artifacts.
exec
/bin/bash -lc 'git status --short' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
 M h_vppu_history.json
 M handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md
 M handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/agent_pubkeys.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/chain_derived_run_facts.json
 M handover/evidence/tb_7_chaintape_smoke_2026-05-01/replay_report.json
 M handover/evidence/tb_7r_smoke_2026-05-02/README.md
 M rules/enforcement.log
?? handover/audits/CODEX_TB_7R_SHIP_AUDIT_2026-05-02.md
?? handover/audits/CODEX_TB_7R_SHIP_AUDIT_R2_2026-05-02.md
?? handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md
?? handover/audits/run_codex_tb_7r_round_2_audit_2026-05-02.sh
?? handover/audits/run_codex_tb_7r_ship_audit_2026-05-02.sh
?? handover/audits/run_gemini_tb_7r_ship_audit_2026-05-02.py
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359/runtime_repo.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.dotgit.tar.gz
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json
?? handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.dotgit.tar.gz

exec
/bin/bash -lc "pwd && rg --files -g 'AGENTS.md' -g '"'!target'"'" in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
/home/zephryj/projects/turingosv4

codex
The working tree matches the remediation surface the prompt described, plus an untracked round-2 audit file already present. I didn’t find a repo-local `AGENTS.md` with the first scan, so I’m checking the parent scopes before relying on repository files.
exec
/bin/bash -lc 'for f in /AGENTS.md /home/AGENTS.md /home/zephryj/AGENTS.md /home/zephryj/projects/AGENTS.md /home/zephryj/projects/turingosv4/AGENTS.md; do [ -f "$f" ] && printf '"'%s\\n' \""'$f"; done' in /home/zephryj/projects/turingosv4
 exited 1 in 0ms:
exec
/bin/bash -lc "rg --files -g 'AGENTS.md' /home/zephryj/projects/turingosv4 /home/zephryj/projects /home/zephryj 2>/dev/null" in /home/zephryj/projects/turingosv4
 succeeded in 435ms:
/home/zephryj/projects/turingclaw/AGENTS.md
/home/zephryj/projects/omega/AGENTS.md
/home/zephryj/projects/turingosv2/AGENTS.md
/home/zephryj/projects/omega_kepler_discovery/AGENTS.md
/home/zephryj/projects/turingos/AGENTS.md
/home/zephryj/projects/turingclaw/AGENTS.md
/home/zephryj/projects/turingosv2/AGENTS.md
/home/zephryj/projects/omega/AGENTS.md
/home/zephryj/projects/omega_kepler_discovery/AGENTS.md
/home/zephryj/projects/turingos/AGENTS.md

  → Inspect updated evidence docs and line references
  • Round-trip two evidence bundles with verify_chaintape
  • Resolve one WorkTx CID chain from extracted CAS
  • Assess OBS text and write verdict
exec
/bin/bash -lc "nl -ba handover/evidence/tb_7r_smoke_2026-05-02/README.md | sed -n '25,285p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    25	| Full-B-4 | n1 × `aime_1997_p9` × MAX_TX=20 | UNSOLVED | none | false |
    26	| Full-B-5 | n1 × `mathd_numbertheory_5` × MAX_TX=20 | UNSOLVED | none | false |
    27	
    28	**Aggregate**: 8/10 SOLVED + chain_oracle_verified=true; 2/10 UNSOLVED + chain_oracle_verified=false. **All 7 indicators GREEN on every run.** **No fake accepted node** in any UNSOLVED run.
    29	
    30	**2026-05-02 packaging update** (post Codex ship-audit round-1 VETO on evidence reproducibility): each run subdirectory now ships with self-contained `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + `replay_report.json`. Total committed evidence size = **892 KB** (10 runs combined; loose .git stores would be 4.8 MB but git auto-ignores nested .git directories — tar.gz keeps the artifacts tracked while compressing 18× smaller than loose objects). Each `replay_report.json` is the literal output of `target/debug/verify_chaintape --repo <run>/runtime_repo --cas <run>/cas --out replay_report.json` (run after extracting the two tar.gz files) — every run shows all 9 boolean indicators true (`ledger_root_verified`, `system_signatures_verified`, `state_reconstructed`, `economic_state_reconstructed`, `cas_payloads_retrievable`, `agent_signatures_verified`, `proposal_telemetry_cas_retrievable`, plus the existing `l4_entries`/`l4e_entries` counts). Acceptance clause 4 ("dashboard regeneratable from ChainTape + CAS alone") is now strictly satisfied from committed evidence; ship condition #5 ("proposal telemetry + payload CIDs resolve") is independently checkable by any auditor via §5.1 below. Round-trip verified: extracting the tar.gz pair into a fresh dir and re-running `verify_chaintape` produces a structurally identical replay_report.json (modulo runtime-tagged `run_id`/`epoch`).
    31	
    32	---
    33	
    34	## §1 Verdict §F acceptance — claim by claim
    35	
    36	### Single smoke (1 problem × MAX_TX small)
    37	
    38	```text
    39	Pass criterion: genesis_report.json valid; ≥1 attempt node in ChainTape
    40	                (L4 or L4.E); replay reconstructs.
    41	```
    42	
    43	| Criterion | Status | Citation |
    44	|---|---|---|
    45	| genesis_report.json valid | ✓ | `single_n1_mathd_algebra_171/runtime_repo/genesis_report.json` (9 fields populated, constitution_hash matches genesis_payload.toml) |
    46	| ≥1 attempt node in ChainTape | ✓ | dashboard §2: `L4=3, L4.E=3` (1 real-LLM L4 Work + 3 synthetic-seed L4.E Work / Verify) |
    47	| replay reconstructs | ✓ | dashboard §2: `ledger_root_verified ✓`, `state_reconstructed ✓` |
    48	
    49	**Single smoke: PASS.**
    50	
    51	### Half smoke (3 problems × MAX_TX=20)
    52	
    53	```text
    54	Pass criterion: same as single, plus parent_tx edges visible
    55	                when multiple externalized proposals exist on same branch.
    56	```
    57	
    58	| Run | 7 indicators | chain_oracle_verified | parent_tx edges |
    59	|---|---|---|---|
    60	| Half-1 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |
    61	| Half-2 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |
    62	| Half-3 | ALL 7 GREEN | true ✓ | none (1-attempt solve) |
    63	
    64	The "when multiple externalized proposals exist on same branch" conditional is NOT triggered — every half-smoke problem solved on the first OMEGA-pertactic emission. Per architect verdict A1=B′, this is the correct behavior: 1 compound proposal = 1 Attempt Node, no per-tactic decomposition.
    65	
    66	**Half smoke: PASS** (parent_tx criterion vacuously satisfied; conditional not triggered).
    67	
    68	### Full smoke (5 problems OR CONDITION=n5 × MAX_TX≥20)
    69	
    70	```text
    71	Pass criterion: ≥2 agent_ids; ≥1 parent_tx edge;
    72	                all externalized proposals in ChainTape (L4 or L4.E);
    73	                solved problem has chain_oracle_verified golden proposal;
    74	                unsolved problem has L4.E failures and no fake accepted node.
    75	```
    76	
    77	Two complementary runs were captured (the verdict's `(or CONDITION=n5)` parenthetical):
    78	
    79	#### Full-A (CONDITION=n5 × `mathd_algebra_171` × MAX_TX=20)
    80	
    81	| Criterion | Status | Citation |
    82	|---|---|---|
    83	| ≥2 agent_ids | ✓ | dashboard §4: Agent_0 + tb6-smoke-agent + tb6-smoke-sponsor + tb7-7-sponsor |
    84	| ≥1 parent_tx edge | **✗** | dashboard §6: `(no branch edges)` |
    85	| All externalized proposals in ChainTape | ✓ | proposal_count=2; runtime_externalized=2 |
    86	| Solved problem has chain_oracle_verified golden proposal | ✓ | dashboard §3 + §7 |
    87	| Unsolved-problem clauses | N/A | (this run solved) |
    88	
    89	#### Full-B (5 problems × n1 × MAX_TX=20)
    90	
    91	| Run | 7 indicators | chain_oracle_verified | L4.E real-LLM Work? | parent_tx edges |
    92	|---|---|---|---|---|
    93	| Full-B-1 (mathd_algebra_171, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
    94	| Full-B-2 (mathd_algebra_107, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
    95	| Full-B-3 (mathd_algebra_359, SOLVED) | ALL 7 GREEN | true ✓ | n/a | none |
    96	| Full-B-4 (aime_1997_p9, UNSOLVED) | ALL 7 GREEN | false ✓ | none (LLM gave up before externalizing any proposal) | none |
    97	| Full-B-5 (mathd_numbertheory_5, UNSOLVED) | ALL 7 GREEN | false ✓ | none (same shape as B-4) | none |
    98	
    99	| Criterion | Status |
   100	|---|---|
   101	| ≥2 agent_ids (across all 5 runs) | ✓ |
   102	| ≥1 parent_tx edge | **✗** |
   103	| All externalized proposals in ChainTape | ✓ |
   104	| Solved problems have chain_oracle_verified golden proposal | ✓ (3/3 solved) |
   105	| Unsolved problems have L4.E failures | **partial** — synthetic-seed L4.E entries present, but no real-LLM L4.E Work (LLM gave up entirely without externalizing a failed proposal) |
   106	| Unsolved problems have no fake accepted node | ✓ |
   107	
   108	**Full smoke: PARTIAL PASS** — 4 of 6 sub-criteria pass strictly; 2 sub-criteria are **architecturally not satisfiable under verdict A1=B′ + the `complete` tool** (see §2 below).
   109	
   110	---
   111	
   112	## §2 parent_tx natural absence — architect verdict 2026-05-02
   113	
   114	**Architect ruling**: per `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md`, parent_tx is a **conditional invariant**, not an unconditional smoke requirement:
   115	
   116	> "If a real LLM run solves in a single externalized proposal under B′ complete-tool semantics, then parent_tx_edges = 0 is valid; the golden path is a singleton node; there is no DAG defect."
   117	
   118	**Observation across all 10 smoke runs**:
   119	
   120	`natural parent_tx_edges = 0 occurred because complete-tool runs solved in one externalized proposal`. This is the architect-sanctioned outcome under verdict A1=B′ + the `complete` tool's one-compound-proposal-per-turn semantics:
   121	
   122	1. The `complete` tool produces **one compound proposal per LLM turn** (whole proof in one tool call).
   123	2. If the proposal's Lean verification passes, OMEGA-pertactic emits **one** WorkTx and the run terminates with `chain_oracle_verified=true`.
   124	3. If the LLM cannot produce a working proof in this `complete` action, current behavior is to give up rather than emit a failed proposal — so unsolved runs likewise have 0 externalized proposals.
   125	
   126	Per architect ruling, `do not fabricate parent_tx edges in natural smoke evidence`. The plumbing is proven separately by **deterministic conformance tests** at `tests/tb_7r_parent_tx_conformance.rs`.
   127	
   128	**Dashboard's parent_tx_state field** distinguishes the architect-mandated four cases (extends architect-listed three with a positive multi-attempt state):
   129	
   130	| state | meaning | seen in this smoke? |
   131	|---|---|---|
   132	| `SingletonGoldenPathValid` | 1 L4 Work + chain_oracle_verified=true; B′ singleton solve | ✓ 8 of 10 runs |
   133	| `NoMultiAttemptObserved` | DAG not exercised; conformance test demonstrates plumbing | ✓ 2 of 10 runs (unsolved) |
   134	| `MultiAttemptDagValid` | ≥1 multi-attempt branch with all parent_tx edges present | ✗ (per architect ruling, expected absence) |
   135	| `MissingParentTxViolation` | ≥1 multi-attempt branch with missing parent_tx (REAL VIOLATION) | ✗ |
   136	
   137	**Conformance test results** (separate from natural smoke; deterministic synthetic fixtures):
   138	
   139	```
   140	running 6 tests
   141	test singleton_golden_path_has_zero_edges_and_is_valid          ... ok
   142	test second_attempt_same_branch_has_parent_tx                   ... ok
   143	test missing_parent_on_nonroot_attempt_is_violation             ... ok
   144	test dashboard_renders_singleton_golden_path                    ... ok
   145	test unsolved_runs_have_no_fake_accepted_nodes                  ... ok
   146	test proposal_count_chain_equals_externalized_proposal_count    ... ok
   147	test result: ok. 6 passed; 0 failed
   148	```
   149	
   150	These six tests prove:
   151	- Plumbing for `MultiAttemptDagValid` works on a synthetic 2-attempt fixture (test 2).
   152	- Plumbing detects `MissingParentTxViolation` on a synthetic 2-attempt fixture with attempt_2.parent_tx=None (test 3).
   153	- Singleton solved → `SingletonGoldenPathValid` (test 1).
   154	- Unsolved → `NoMultiAttemptObserved`, no fake accepted node (test 5).
   155	- Dashboard renders singleton golden path with depth=0 [ORACLE] (test 4).
   156	- proposal_count exactly matches externalized count (test 6).
   157	
   158	Per architect ship condition: **"forced parent_tx conformance test passes" — ✓ MET.**
   159	
   160	**Carry-forward to TB-8+**: per-tactic decomposition is deferred (verdict A1=B′). When TB-8 reopens per-tactic, multi-attempt branches will become natural in smoke evidence too. Until then, `parent_tx_state` for natural smoke is expected to be `SingletonGoldenPathValid` or `NoMultiAttemptObserved`.
   161	
   162	---
   163	
   164	## §3 What this evidence proves (vs verdict §11 acceptance)
   165	
   166	```text
   167	For every externalized LLM proposal:                        ✓
   168	  it is represented as either L4 accepted WorkTx or L4.E rejected evidence.
   169	For every L4 accepted WorkTx:                                ✓
   170	  predicate evidence (VerificationResult CAS) exists and resolves.
   171	For every failed proposal:                                   ✓ (partial — see §2)
   172	  it is in L4.E only; raw_diagnostic shielded but auditable.
   173	For every dashboard report:                                  ✓
   174	  it can be deleted and regenerated from ChainTape + CAS alone.
   175	```
   176	
   177	The "every failed proposal" clause is satisfied for the failed-proposals that actually externalized (zero in this smoke; LLM gave up without emitting). The "no fake accepted node" rule is correctly enforced — UNSOLVED runs have `chain_oracle_verified=false` and no L4 Work entry.
   178	
   179	---
   180	
   181	## §4 Per-run artifacts
   182	
   183	Each run subdirectory contains (committed to git after 2026-05-02 packaging update):
   184	
   185	```text
   186	runtime_repo/
   187	  genesis_report.json            — TB-7R Deliverable C
   188	  initial_q_state.json           — TB-7.7 D7 (preseeded balances + task state)
   189	  agent_audit_trail.jsonl
   190	  agent_pubkeys.json
   191	  pinned_pubkeys.json
   192	  rejections.jsonl               — L4.E records (raw_diagnostic shielded per Art. III.4)
   193	  synthetic_rejection_label.json
   194	runtime_repo.dotgit.tar.gz       — compressed `.git/` containing refs/transitions/main + chain commit objects (LedgerEntry blobs); ~12 KB; expand into runtime_repo/ to enable replay
   195	cas/
   196	  .turingos_cas_index.jsonl      — sidecar index (TB-7.6 atomic-write)
   197	cas.dotgit.tar.gz                — compressed `.git/` containing CAS payload blobs (proposal payloads + ProposalTelemetry + VerificationResult + agent audit trail records); ~12 KB; expand into cas/ to enable CID resolution
   198	replay_report.json               — literal `verify_chaintape` output (9 boolean indicators true + l4/l4e counts + final_state_root_hex + final_ledger_root_hex + head_commit_oid_hex)
   199	stdout                           — PPUT_RESULT JSON line
   200	stderr                           — RUST_LOG=warn output
   201	dashboard.txt                    — `audit_dashboard --repo runtime_repo --cas cas` output
   202	```
   203	
   204	**Why tar.gz instead of loose `.git/` directories**: git automatically treats nested `.git/` directories as submodule sentinels and refuses to track their contents (would otherwise require submodule registration with external remotes). Compressed-archive packaging keeps the chain stores fully tracked under the parent repo's history while compressing 18× smaller than loose objects (892 KB total for 10 runs vs ~4.8 MB loose).
   205	
   206	Per-run replay_report indicator counts:
   207	```text
   208	SOLVED runs (8):   l4_entries=3 (TaskOpen + EscrowLock + accepted Work), l4e_entries=3 (synthetic seeds + zero-stake rejections)
   209	UNSOLVED runs (2): l4_entries=2 (TaskOpen + EscrowLock; no accepted Work — chain_oracle_verified=false), l4e_entries=2
   210	```
   211	
   212	Per-run replay_report indicator counts:
   213	```text
   214	SOLVED runs (8):   l4_entries=3 (TaskOpen + EscrowLock + accepted Work), l4e_entries=3 (synthetic seeds + zero-stake rejections)
   215	UNSOLVED runs (2): l4_entries=2 (TaskOpen + EscrowLock; no accepted Work — chain_oracle_verified=false), l4e_entries=2
   216	```
   217	
   218	---
   219	
   220	## §5 Reproduce
   221	
   222	### §5.1 Verify committed evidence (no LLM / Lean required)
   223	
   224	Any auditor with this repo + a built `target/debug/{verify_chaintape,audit_dashboard}` can independently verify any of the 10 runs from committed evidence alone:
   225	
   226	```bash
   227	RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171  # or any run dir
   228	
   229	# Stage a working copy with .git stores expanded:
   230	WORK=/tmp/audit_$$
   231	mkdir -p $WORK/runtime_repo $WORK/cas
   232	cp $RUN/runtime_repo/*.json $RUN/runtime_repo/*.jsonl $WORK/runtime_repo/
   233	cp $RUN/cas/.turingos_cas_index.jsonl $WORK/cas/
   234	tar -xzf $RUN/runtime_repo.dotgit.tar.gz -C $WORK/runtime_repo
   235	tar -xzf $RUN/cas.dotgit.tar.gz -C $WORK/cas
   236	
   237	# Re-derive replay_report.json from committed ChainTape + CAS:
   238	target/debug/verify_chaintape \
   239	  --repo $WORK/runtime_repo \
   240	  --cas  $WORK/cas \
   241	  --out  $WORK/replay_report.audit.json
   242	
   243	# Compare to committed (byte-identical modulo run_id + epoch):
   244	jq -S 'del(.run_id,.epoch)' $RUN/replay_report.json    > /tmp/orig.json
   245	jq -S 'del(.run_id,.epoch)' $WORK/replay_report.audit.json > /tmp/repro.json
   246	diff /tmp/orig.json /tmp/repro.json && echo "STRUCTURALLY IDENTICAL"
   247	
   248	# Re-derive dashboard from committed ChainTape + CAS:
   249	target/debug/audit_dashboard --repo $WORK/runtime_repo --cas $WORK/cas > $WORK/dashboard.audit.txt
   250	diff $RUN/dashboard.txt $WORK/dashboard.audit.txt   # semantic content identical (timestamps may differ)
   251	```
   252	
   253	This satisfies acceptance clause 4: every dashboard report is regeneratable from committed ChainTape + CAS alone.
   254	
   255	### §5.2 Generate fresh evidence (LLM + Lean required)
   256	
   257	```bash
   258	mkdir -p /tmp/tb7r_repro/{runtime_repo,cas}
   259	TURINGOS_CHAINTAPE_PATH=/tmp/tb7r_repro/runtime_repo \
   260	TURINGOS_CAS_PATH=/tmp/tb7r_repro/cas \
   261	TURINGOS_CHAINTAPE_PRESEED=1 \
   262	TURINGOS_RUN_ID=tb7r-repro \
   263	CONDITION=n5 \
   264	MAX_TRANSACTIONS=20 \
   265	target/debug/evaluator mathd_algebra_171.lean
   266	
   267	target/debug/audit_dashboard --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas
   268	target/debug/verify_chaintape --repo /tmp/tb7r_repro/runtime_repo --cas /tmp/tb7r_repro/cas --out /tmp/tb7r_repro/replay_report.json
   269	```
   270	
   271	---
   272	
   273	## §6 Cross-references
   274	
   275	- TB-7R authorization: `handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md`
   276	- TB-7R charter: `handover/tracer_bullets/TB-7R_charter_2026-05-01.md`
   277	- L4 purity audit: `handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md`
   278	- Codex micro-audit: `handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md`
   279	- TRACE_MATRIX orphan registry: `handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md`
   280	- Three-node taxonomy: `handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md`
   281	- Pre-TB-7R smoke (grandfathered): `handover/evidence/tb_7_7_dag_capable_smoke_2026-05-01/`, `tb_7_chaintape_smoke_2026-05-01/`, `tb_7_real_smoke_5_problems_2026-05-01/`

exec
/bin/bash -lc "nl -ba handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md | sed -n '1,150p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# OBS — Coverage Denominator (post-TB-7R hardening) (2026-05-02)
     2	
     3	**Class**: Observation (post-TB-7R audit risk)
     4	**Driver**: Architect verdict 2026-05-02 (`2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md` §6 "Next insight")
     5	**Status**: ACTIVE — post-TB-7R follow-up; NOT a TB-7R blocker.
     6	
     7	---
     8	
     9	## §1 The risk
    10	
    11	Architect 2026-05-02:
    12	
    13	> "ChainTape can only prove what reached it. The next hardening step is
    14	> to ensure every LLM response that becomes an externalized proposal is
    15	> counted before submission and must either land in L4/L4.E or fail-closed.
    16	> Without this, a hidden legacy path can still produce unchained proposals."
    17	
    18	The strict three-node taxonomy used in TB-7R defines an "externalized
    19	proposal" as `bus.submit_typed_tx`. Under that strict definition,
    20	TB-7R smoke passes "every externalized proposal in L4 or L4.E"
    21	trivially — because the only path to externalization IS submit_typed_tx,
    22	and submit_typed_tx routes through Sequencer to L4 or L4.E.
    23	
    24	**The risk is in the IMPLICIT step**: how does an LLM output become an
    25	externalized proposal? If a code path consumes an LLM response, processes
    26	it, and bypasses submit_typed_tx (e.g. into `bus.append` shadow_only,
    27	into `bus.record_rejection` counter, or into a future legacy path), then
    28	that LLM output is "consumed but not chained." The denominator of
    29	"all LLM proposals" is unprotected.
    30	
    31	## §2 Concrete current-state inventory
    32	
    33	In `experiments/minif2f_v4/src/bin/evaluator.rs`, the `step` tool's
    34	PartialVerdict dispatch:
    35	
    36	```text
    37	"step" => match oracle.verify_partial(prefix) {
    38	    PartialVerdict::Complete   → bus.submit_typed_tx → L4 (or L4.E)  [chained]
    39	    PartialVerdict::PartialOk  → bus.append_oracle_accepted          [shadow_only / not chained]
    40	    PartialVerdict::Reject     → bus.record_rejection (counter)      [in-memory / not chained]
    41	}
    42	```
    43	
    44	For the `mathd_*` smoke problems where the LLM emits a one-shot `complete`
    45	action that Lean accepts, only the Complete branch fires — and that's
    46	chained. For harder problems where the LLM emits intermediate `step`
    47	actions:
    48	- PartialOk → goes to kernel.tape (shadow_only) but NOT chain
    49	- Reject → goes to in-memory counter (token-cost only; see §2.1 update)
    50	
    51	### §2.1 Codex round-1 ship-audit refinement (2026-05-02)
    52	
    53	Codex round-1 ship audit Q2/Q10 surfaced a sharper framing of the gap and
    54	corrected one premise:
    55	
    56	**(a) PartialOk → Complete proof-prefix dependency** (Codex Q2 CHALLENGE,
    57	new constitutional debt #2): the issue is not just "PartialOk is unchained
    58	LLM activity," but specifically that a later `Complete` action verifies
    59	`tape_chain + tactic` (`evaluator.rs:2132`) while storing **only `tactic`**
    60	as the WorkTx artifact (`evaluator.rs:2222`). This means an accepted L4
    61	WorkTx's `proof_artifact_cid` resolves to a `tactic` payload that, on its
    62	own, is not Lean-verifiable — the reconstructable proof depends on
    63	prior unchained PartialOk steps in `kernel.tape`.
    64	
    65	Replay from L4 + CAS alone CAN reconstruct the QState + EconomicState +
    66	predicate_results (`replay_report.json` shows this on every TB-7R smoke
    67	run). What replay CANNOT reconstruct from L4 + CAS alone is the *Lean
    68	proof object* that justified the predicate-pass — that proof object is
    69	the tape_chain prefix that lived in unchained kernel.tape during the run.
    70	
    71	Under TB-7R's strict three-node taxonomy + verdict A1=B′ (proposal-level
    72	DAG, per-tactic deferred), this is *internally consistent* because
    73	"externalized" is defined as `submit_typed_tx`-routed and the WorkTx
    74	*acceptance signal* is on chain even if the proof *artifact* is partial.
    75	But under the broader Tape Canonical Axiom (Art. 0.2) + acceptance
    76	clause 2 ("predicate evidence resolves from CAS"), the proof artifact
    77	should be self-contained.
    78	
    79	**(b) Reject path correction** (Codex Q10 PASS for prompt isolation but
    80	clarifying premise): `PartialVerdict::Reject` records a bounded rejection
    81	class label via `bus.record_rejection(agent_id, class.label())` plus
    82	`acc.record_tool_stdout(&reason)` for token-cost accounting only
    83	(`cost_aggregator.rs:57`). The raw `reason` does NOT flow into the next
    84	prompt — `prompt_builder` reads bounded class labels from
    85	`evaluator.rs:1344` and `bus.rs:576`, NOT raw Lean stderr. This means
    86	the original OBS-2 (`OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`)
    87	had a stale premise: there is no Art. III.4 prompt-pollution risk; only
    88	the Reject side of the coverage-denominator question (raw oracle output
    89	not reaching chain even as L4.E rejection evidence) remains.
    90	
    91	## §3 Why this is post-TB-7R
    92	
    93	Architect verdict 2026-05-02 explicitly **frames this as the next
    94	hardening step, not a TB-7R blocker**. Under the strict three-node
    95	interpretation TB-7R adopts, the current state is internally consistent.
    96	
    97	The TB-7R smoke shows the natural consequence: aime_1997_p9 ran 20 step
    98	actions (18 reject + 2 partial-OK), but **0 of those reached chain**
    99	because the chain-routing path is gated on `Complete` outcome only.
   100	This is documented in `handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/stdout`
   101	(see `tool_dist`).
   102	
   103	## §4 Recommended hardening (post-TB-7R)
   104	
   105	A future TB (TB-7.5? TB-8?) should:
   106	
   107	1. Re-route `PartialVerdict::PartialOk` through `submit_typed_tx` with
   108	   `predicate_passes=true` and a distinct acceptance class
   109	   (`lean_partial`), landing intermediate progress in L4 with a
   110	   non-OMEGA-terminating semantics. This puts every `step` action's
   111	   verified-progress claim on chain.
   112	2. Re-route `PartialVerdict::Reject` through `submit_typed_tx` with
   113	   `predicate_passes=false`, landing in L4.E with
   114	   `rejection_class = LeanFailed` and `raw_diagnostic_cid` shielded.
   115	3. **Self-contained proof artifact**: when a `Complete` action follows
   116	   prior PartialOk steps, the accepted WorkTx's `proof_artifact_cid`
   117	   must resolve to the *full reconstructable proof* (tape_chain + tactic),
   118	   not just the final tactic. Either store the full prefix on chain via
   119	   the §4.1 reroute, or store the concatenated `tape_chain + tactic`
   120	   blob in CAS as the WorkTx artifact and update verify_partial to
   121	   verify against that committed blob alone. (Codex round-1 §2.1.a)
   122	4. Verify the strict invariant: every LLM tool-call action that runs
   123	   Lean (or any oracle) must produce exactly one chain entry — either
   124	   L4 accepted or L4.E rejected — never an unchained tool_dist counter
   125	   bump. AND every accepted L4 WorkTx's proof artifact must be
   126	   Lean-verifiable from CAS alone (no kernel.tape dependency).
   127	
   128	The Sequencer's existing `apply_one` + `predicate_results` machinery
   129	already supports the routing change in §4.1+§4.2; the proof-self-containment
   130	change in §4.3 is at the evaluator dispatch site (or at the Lean proof
   131	serialization site within the `complete` tool).
   132	
   133	## §5 Conformance criterion (post-implementation)
   134	
   135	```text
   136	For every run:
   137	  externalized_proposal_count ==
   138	    L4 Work entries + L4.E Work entries
   139	  (no LLM oracle action lands only in tool_dist or only in kernel.tape)
   140	```
   141	
   142	This is stronger than TB-7R's strict three-node interpretation
   143	("every submit_typed_tx call lands in L4 or L4.E") because it closes
   144	the implicit step from "LLM output" to "submit_typed_tx call".
   145	
   146	## §6 Cross-references
   147	
   148	- Verdict: `handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md` §6
   149	- Companion OBS: `handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`
   150	- Smoke evidence (aime run with 20 step actions, 0 on chain):

exec
/bin/bash -lc "nl -ba handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	# OBS — Art. III.4 Prompt Pollution via acc.record_tool_stdout (2026-05-02)
     2	
     3	**Class**: Observation (constitutional risk; architect verdict-silent)
     4	**Driver**: Claude self-assessment 2026-05-02 ultrathink turn surfaced this; architect verdict 2026-05-02 did NOT explicitly address.
     5	**Status**: **CORRECTED — premise stale per Codex round-1 ship-audit Q10 (2026-05-02).** No Art. III.4 prompt-pollution risk; original concern is empirically unfounded. Retained for traceability + closure path. **Not a TB-7R blocker (and not a future-ruling required item either).**
     6	
     7	---
     8	
     9	## §0 Correction (2026-05-02 post Codex round-1)
    10	
    11	Codex round-1 ship audit Q10 verified the actual code paths and found:
    12	
    13	> "I found no concrete cross-agent raw Lean error pollution in full-n5
    14	> evidence; it contains no prompt/stdout transcript and only two synthetic
    15	> audit-trail rows. Code also contradicts the OBS premise:
    16	> `acc.record_tool_stdout` only increments token cost
    17	> (`cost_aggregator.rs:57`). Prompt errors come from bounded class labels
    18	> via `evaluator.rs:1344` and `bus.rs:576`, not raw Lean text."
    19	
    20	The remainder of this document (§1–§7 below) is preserved as the
    21	original concern + closure-path narrative, but the central premise —
    22	that `acc.record_tool_stdout(&reason)` flows raw Lean diagnostics into
    23	prompt context — is **incorrect**. Actual data flow:
    24	
    25	1. `PartialVerdict::Reject(reason)` → `bus.record_rejection(agent_id, class.label())` (bounded label, not raw)
    26	2. `PartialVerdict::Reject(reason)` → `acc.record_tool_stdout(&reason)` (token-cost accounting via `cost_aggregator.rs:57`; **does NOT write to prompt errors_history**)
    27	3. Next-iteration prompt errors come from `prompt_builder` reading the bounded class labels at `evaluator.rs:1344` and `bus.rs:576` — never the raw `reason`
    28	
    29	Because the raw `reason` is bounded to token-cost accounting, there is no
    30	agent-to-agent diagnostic leakage path even in multi-agent runs.
    31	Art. III.4 selective-broadcasting / shielding is honored.
    32	
    33	**Closure**: this OBS is closed-as-empirically-unfounded. The
    34	`coverage_denominator` OBS (companion file) absorbs the *Reject path
    35	not reaching chain* concern, which is a coverage question, not a
    36	prompt-pollution question.
    37	
    38	---
    39	
    40	## §1 The observation
    41	
    42	Constitution Art. III.4 (selective broadcasting / shielding):
    43	
    44	> "失败候选不能污染其他 Agent 上下文。
    45	> 顶层白盒对系统信息做 量化、广播、屏蔽。"
    46	
    47	In `experiments/minif2f_v4/src/bin/evaluator.rs:2430-2438` (`step` tool's
    48	`PartialVerdict::Reject` branch):
    49	
    50	```rust
    51	PartialVerdict::Reject(reason) => {
    52	    let class = classify_lean_error(&reason);
    53	    bus.record_rejection(agent_id, class.label());
    54	    // PPUT-CCL B2: step rejection reason flows into next prompt.
    55	    acc.record_tool_stdout(&reason);
    56	    *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
    57	    let preview = reason.chars().take(200).collect::<String>();
    58	    warn!("[tx {}] step rejected ({}): {}", tx, class.label(), preview);
    59	}
    60	```
    61	
    62	The line `acc.record_tool_stdout(&reason)` flows the **raw Lean error
    63	reason** (full stderr text from Lean's verify_partial call) back to the
    64	next prompt's "errors" section. The classifier label `class.label()`
    65	also flows separately, but the raw `reason` is preserved verbatim.
    66	
    67	**Implication under Art. III.4**: any agent reading the next-iteration
    68	prompt sees the raw Lean error from a prior step's rejection. This means:
    69	
    70	- A single agent can leak its own private failure modes into its own
    71	  next-prompt. **This may be acceptable** (agent learning from its own
    72	  mistakes is the design intent of PPUT-CCL B2).
    73	- **Multi-agent runs (n5, n10, etc.)**: if `acc` is shared across
    74	  agents (or if agents share the kernel.tape that includes prompt
    75	  history), one agent's raw Lean error leaks into another agent's
    76	  prompt. **This violates Art. III.4** — failure pollutes other-agent
    77	  context.
    78	
    79	## §2 The architectural ambiguity
    80	

exec
/bin/bash -lc "test -x target/debug/verify_chaintape && target/debug/verify_chaintape --help | sed -n '1,80p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
verify_chaintape: --help requested
usage: verify_chaintape --repo <runtime_repo_path> --cas <cas_path> [--run-id <id>] [--out <path>]

codex
The updated README now describes the archive packaging and reproduce path at lines 183-253, but I’m treating that as a claim to prove rather than taking it on trust. One minor doc wart is already visible: the per-run replay count block is duplicated at README lines 206-216, but that does not affect replayability.
  ✓ Inspect updated evidence docs and line references
  → Round-trip two evidence bundles with verify_chaintape
  • Resolve one WorkTx CID chain from extracted CAS
  • Assess OBS text and write verdict
exec
/bin/bash -lc 'set -euo pipefail
ROOT=/home/zephryj/projects/turingosv4
RUNS=(
  handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171
  handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9
)
for RUN in "${RUNS[@]}"; do
  WORK=$(mktemp -d /tmp/tb7r_r2_XXXXXX)
  mkdir -p "$WORK/runtime_repo" "$WORK/cas"
  cp "$ROOT/$RUN"/runtime_repo/*.json "$ROOT/$RUN"/runtime_repo/*.jsonl "$WORK/runtime_repo/"
  cp "$ROOT/$RUN"/cas/.turingos_cas_index.jsonl "$WORK/cas/"
  stat -c "RUN=$RUN artifact=%n bytes=%s" "$ROOT/$RUN/runtime_repo.dotgit.tar.gz" "$ROOT/$RUN/cas.dotgit.tar.gz"
  tar -tzf "$ROOT/$RUN/runtime_repo.dotgit.tar.gz" >/dev/null
  tar -tzf "$ROOT/$RUN/cas.dotgit.tar.gz" >/dev/null
  tar -xzf "$ROOT/$RUN/runtime_repo.dotgit.tar.gz" -C "$WORK/runtime_repo"
  tar -xzf "$ROOT/$RUN/cas.dotgit.tar.gz" -C "$WORK/cas"
  test -d "$WORK/runtime_repo/.git"
  test -d "$WORK/cas/.git"
  "$ROOT/target/debug/verify_chaintape" --repo "$WORK/runtime_repo" --cas "$WORK/cas" --out "$WORK/r.json" >/dev/null
  jq -S '"'del(.run_id,.epoch)' \""'$ROOT/$RUN/replay_report.json" > "$WORK/orig.norm.json"
  jq -S '"'del(.run_id,.epoch)' \""'$WORK/r.json" > "$WORK/repro.norm.json"
  cmp -s "$WORK/orig.norm.json" "$WORK/repro.norm.json"
  printf '"'RUN=%s compare=STRUCTURALLY_IDENTICAL\\n' \""'$RUN"
  jq -r '"'\"RUN='\""'$RUN"'"' l4_entries=\\(.l4_entries) l4e_entries=\\(.l4e_entries) booleans=\" + ([paths(scalars) as "'$p | select(getpath($p)|type=="boolean") | "'"\\("'$p|join("."))='"\\(getpath("'$p))"] | join(","))'"' \""'$WORK/r.json"
  jq -r '"'\"RUN='\""'$RUN"'"' final_state_root=\\(.detail.final_state_root_hex) final_ledger_root=\\(.detail.final_ledger_root_hex) head_commit_oid=\\(.detail.head_commit_oid_hex) replay_failure=\\(.detail.replay_failure)\"' \""'$WORK/r.json"
  printf '"'RUN=%s workdir=%s\\n' \""'$RUN" "$WORK"
done' in /home/zephryj/projects/turingosv4
 succeeded in 440ms:
RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171 compare=STRUCTURALLY_IDENTICAL
RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171 l4_entries=3 l4e_entries=3 booleans=ledger_root_verified=true,system_signatures_verified=true,state_reconstructed=true,economic_state_reconstructed=true,cas_payloads_retrievable=true,agent_signatures_verified=true,proposal_telemetry_cas_retrievable=true,detail.initial_q_state_loaded_from_disk=true
RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171 final_state_root=e5199d29e268c057f197ce8492e02429ab7af0046cda510cc08bf55efc4f7647 final_ledger_root=68bd2fe6a0b9c394151fa217da3d73087de75d4f894db208ac7376243661fb69 head_commit_oid=d4d704beb5ab16782425f75194cae07d83842f5c replay_failure=null
RUN=handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171 workdir=/tmp/tb7r_r2_uyTXAx
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 artifact=/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz bytes=11445
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 artifact=/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz bytes=10678
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 compare=STRUCTURALLY_IDENTICAL
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 l4_entries=2 l4e_entries=2 booleans=ledger_root_verified=true,system_signatures_verified=true,state_reconstructed=true,economic_state_reconstructed=true,cas_payloads_retrievable=true,agent_signatures_verified=true,proposal_telemetry_cas_retrievable=true,detail.initial_q_state_loaded_from_disk=true
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 final_state_root=97caed87cc19241835d1637dc68f7e304277c5128f334dd35d0c049450266d02 final_ledger_root=4e8a88526131bc48bb9cc2c98cf2a00df8599b1824f7e1e8f6d0cdf453172ca5 head_commit_oid=57e7f532ff8fab6ab13a221d649554f45cc15709 replay_failure=null
RUN=handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 workdir=/tmp/tb7r_r2_3Dwb6k

codex
Both sampled archives extract cleanly and `verify_chaintape` reproduces normalized reports byte-for-byte. The verifier output exposes eight boolean-valued JSON fields rather than nine; all eight are true, and the top-level seven verifier indicators requested in the README are true for both samples.
  ✓ Inspect updated evidence docs and line references
  ✓ Round-trip two evidence bundles with verify_chaintape
  → Resolve one WorkTx CID chain from extracted CAS
  • Assess OBS text and write verdict
exec
/bin/bash -lc 'git --git-dir=/tmp/tb7r_r2_uyTXAx/runtime_repo/.git --work-tree=/tmp/tb7r_r2_uyTXAx/runtime_repo show-ref' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
d4d704beb5ab16782425f75194cae07d83842f5c refs/transitions/main

exec
/bin/bash -lc 'git --git-dir=/tmp/tb7r_r2_uyTXAx/runtime_repo/.git --work-tree=/tmp/tb7r_r2_uyTXAx/runtime_repo log --oneline --decorate --all --stat' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
d4d704b transition logical_t=3
 entry_canonical | Bin 260 -> 260 bytes
 payload_cid     |   2 +-
 signature       |   2 +-
 3 files changed, 2 insertions(+), 2 deletions(-)
f03a896 transition logical_t=2
 entry_canonical | Bin 260 -> 260 bytes
 payload_cid     |   2 +-
 signature       | Bin 64 -> 64 bytes
 3 files changed, 1 insertion(+), 1 deletion(-)
c767830 transition logical_t=1
 entry_canonical | Bin 0 -> 260 bytes
 payload_cid     |   1 +
 signature       | Bin 0 -> 64 bytes
 3 files changed, 1 insertion(+)

exec
/bin/bash -lc "nl -ba /tmp/tb7r_r2_uyTXAx/cas/.turingos_cas_index.jsonl | sed -n '1,40p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{"cid":[194,67,6,224,159,63,199,152,181,201,3,232,78,9,120,137,51,56,39,77,244,155,191,203,95,142,180,194,142,161,16,167],"backend_oid_hex":"730a689d4bff787c0abdc6c14e73229dc80a0898","object_type":"ProposalPayload","creator":"sequencer-epoch-1","created_at_logical_t":1,"schema_id":"TypedTx.v1","size_bytes":284}
     2	{"cid":[229,60,110,6,205,198,189,34,184,33,53,149,8,158,26,204,232,84,117,117,133,71,66,18,68,88,18,63,183,200,44,6],"backend_oid_hex":"f58607aa7b44ce89b0ae0a90192e050d8ae01371","object_type":"ProposalPayload","creator":"sequencer-epoch-1","created_at_logical_t":2,"schema_id":"TypedTx.v1","size_bytes":258}
     3	{"cid":[253,87,135,183,123,122,228,125,142,177,30,79,16,102,16,156,232,171,168,64,101,116,165,180,205,8,51,77,1,71,94,236],"backend_oid_hex":"0b58ba399885c81a96a6fe891a0e125e57270927","object_type":"Generic","creator":"tb6-atom5-smoke","created_at_logical_t":1,"schema_id":"turingosv4.agent_proposal_record.v1","size_bytes":259}
     4	{"cid":[5,231,129,73,109,178,8,244,27,164,151,13,103,167,11,4,146,55,90,144,184,160,216,224,221,45,114,217,58,3,62,245],"backend_oid_hex":"00032587261c99890051ffba3a49571476e4fed9","object_type":"ProposalPayload","creator":"sequencer.rejection_path.epoch-1","created_at_logical_t":2,"schema_id":"TypedTx.v1","size_bytes":287}
     5	{"cid":[123,201,179,38,84,117,225,88,36,238,237,105,59,240,47,186,124,128,129,27,238,91,210,98,40,122,58,126,214,140,157,1],"backend_oid_hex":"0b2ef68b361bf8458796498fbe4cf896792ce2f0","object_type":"Generic","creator":"tb6-atom5-smoke","created_at_logical_t":1,"schema_id":"turingosv4.agent_proposal_record.v1","size_bytes":281}
     6	{"cid":[94,65,45,211,51,167,215,161,33,245,89,49,150,104,78,140,126,228,40,181,205,2,151,12,116,90,83,17,161,116,74,184],"backend_oid_hex":"6bc19735e4a5667a82f8c985f2dcdf50e9295f9e","object_type":"Generic","creator":"sequencer.rejection_path.epoch-1","created_at_logical_t":2,"schema_id":"TransitionError.display.v1","size_bytes":23}
     7	{"cid":[140,46,2,196,137,30,192,41,157,40,93,12,111,206,196,5,17,124,212,128,70,1,113,50,107,73,255,81,97,45,117,221],"backend_oid_hex":"62f49fa42bf808c951f5a04c178dfc80a0fa9db7","object_type":"ProposalPayload","creator":"sequencer.rejection_path.epoch-1","created_at_logical_t":2,"schema_id":"TypedTx.v1","size_bytes":383}
     8	{"cid":[195,26,11,201,177,23,132,46,61,248,8,31,191,208,20,10,236,38,226,117,46,167,233,40,130,215,111,54,88,101,174,131],"backend_oid_hex":"7a66b55285ce8ca22f6b44656b1e8c4ed60b4f1a","object_type":"ProposalPayload","creator":"tb7-atom3-omega-pertactic-payload","created_at_logical_t":2,"schema_id":"turingosv4.proposal_payload.v1","size_bytes":84}
     9	{"cid":[234,143,35,0,6,25,177,239,234,169,161,108,192,114,56,116,177,9,27,254,46,212,218,212,52,105,200,177,210,94,114,135],"backend_oid_hex":"4c49ac7701ce9cb92667d61c0cbe6f6f888d49cd","object_type":"Generic","creator":"tb7-atom3-omega-pertactic-vr","created_at_logical_t":2,"schema_id":"turingosv4.verification_result.v1","size_bytes":220}
    10	{"cid":[187,173,229,151,206,28,222,196,167,28,241,80,66,221,136,169,83,169,63,249,141,0,150,231,213,113,120,234,163,101,126,81],"backend_oid_hex":"1c2d6cce6c20c7623949a89d3a96f2912f832f9c","object_type":"Generic","creator":"tb7-atom3-omega-pertactic","created_at_logical_t":2,"schema_id":"turingosv4.proposal_telemetry.v1","size_bytes":184}
    11	{"cid":[210,250,47,242,24,216,246,232,215,206,129,76,246,255,65,127,110,221,169,101,185,80,28,209,80,103,32,96,12,255,145,167],"backend_oid_hex":"467d358f52191ec017b87eaf6c55c494b208a298","object_type":"ProposalPayload","creator":"sequencer-epoch-1","created_at_logical_t":3,"schema_id":"TypedTx.v1","size_bytes":361}
    12	{"cid":[63,36,233,114,102,106,100,84,112,12,78,243,80,195,238,168,253,168,11,226,34,131,210,114,151,170,172,202,205,222,119,16],"backend_oid_hex":"427d2d5ccd5dfd57338c27c28613e8adbbb8f391","object_type":"ProposalPayload","creator":"sequencer.rejection_path.epoch-1","created_at_logical_t":3,"schema_id":"TypedTx.v1","size_bytes":249}

exec
/bin/bash -lc "rg \"verify_chaintape\" -n src experiments tests --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
exec
/bin/bash -lc "rg \"struct WorkTx|enum TypedTx|proposal_cid|proposal_telemetry\" -n src experiments tests --glob '"'!target'"'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
tests/tb_7_atom6_chain_backed_smoke.rs:56:use turingosv4::runtime::proposal_telemetry::{
tests/tb_7_atom6_chain_backed_smoke.rs:85:/// 4. (Gate 5) ReplayReport.proposal_telemetry_cas_retrievable = true.
tests/tb_7_atom6_chain_backed_smoke.rs:206:        report.proposal_telemetry_cas_retrievable,
tests/tb_7_atom6_chain_backed_smoke.rs:207:        "Gate 5: every WorkTx.proposal_cid must resolve to CAS ProposalTelemetry — {report:?}"
tests/tb_7_atom6_chain_backed_smoke.rs:235:    // aren't decoded for proposal_telemetry. Asserts shape, not exact count.)
tests/tb_7_atom6_chain_backed_smoke.rs:277:                 4. **Gate 5** (ProposalTelemetry CAS): every WorkTx.proposal_cid resolves to a CAS ProposalTelemetry object.\n\
tests/tb_2_runtime_boundary.rs:48:struct WorkTxFixtureOpts {
tests/tb_2_runtime_boundary.rs:102:        proposal_cid: Default::default(),
tests/tb_4_rsp2_admission_surface.rs:148:        proposal_cid: Default::default(),
experiments/minif2f_v4/src/bin/evaluator.rs:1476:                                // q_snapshot / CAS open / proposal_telemetry write /
experiments/minif2f_v4/src/bin/evaluator.rs:1510:                                    let pt = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:1517:                                        turingosv4::runtime::proposal_telemetry::TokenCounts {
experiments/minif2f_v4/src/bin/evaluator.rs:1533:                                    let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
experiments/minif2f_v4/src/bin/evaluator.rs:1541:                                            error!("[chaintape/atom2] FAIL-CLOSED: proposal_telemetry CAS write failed: {e}");
experiments/minif2f_v4/src/bin/evaluator.rs:1773:                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:1780:                                                turingosv4::runtime::proposal_telemetry::TokenCounts {
experiments/minif2f_v4/src/bin/evaluator.rs:1825:                                            let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
experiments/minif2f_v4/src/bin/evaluator.rs:2217:                                            let pt_partial = match turingosv4::runtime::proposal_telemetry::ProposalTelemetry::build_for_evaluator_append_with_parent(
experiments/minif2f_v4/src/bin/evaluator.rs:2224:                                                turingosv4::runtime::proposal_telemetry::TokenCounts {
experiments/minif2f_v4/src/bin/evaluator.rs:2267:                                            let tel_cid = match turingosv4::runtime::proposal_telemetry::write_to_cas(
src/runtime/adapter.rs:122:        proposal_cid: Default::default(),
src/runtime/adapter.rs:139:/// 1. Takes `proposal_cid` as a real CAS reference (the
src/runtime/adapter.rs:140:///    `ProposalTelemetry` object written by Atom 1.5 `proposal_telemetry`).
src/runtime/adapter.rs:159:    proposal_cid: Cid,
src/runtime/adapter.rs:192:        proposal_cid,
src/runtime/adapter.rs:207:        proposal_cid,
src/runtime/adapter.rs:308:            proposal_cid: work.proposal_cid,
src/runtime/mod.rs:36:pub mod proposal_telemetry;
src/runtime/proposal_telemetry.rs:4://! routed through `bus.submit_typed_tx` carries a `proposal_cid` pointing to
src/runtime/proposal_telemetry.rs:33://!   `WorkTx.proposal_cid` so chain-derived run facts (golden_path_token_count
src/runtime/proposal_telemetry.rs:38://! object that grounds `WorkTx.proposal_cid` per TB-7 §4.5 / Gate 5 / Atom 4
src/runtime/proposal_telemetry.rs:44://!   `turingosv4.proposal_telemetry.v1`
src/runtime/proposal_telemetry.rs:56:const PROPOSAL_TELEMETRY_SCHEMA_ID: &str = "turingosv4.proposal_telemetry.v1";
src/runtime/proposal_telemetry.rs:99:/// to by `WorkTx.proposal_cid`. Schema per ARCHITECT_RULING 2026-05-01 D5 +
src/runtime/proposal_telemetry.rs:309:/// so `WorkTx.proposal_cid` is byte-stable across runs as long as the
src/runtime/proposal_telemetry.rs:331:/// `WorkTx.proposal_cid` references during replay.
src/runtime/proposal_telemetry.rs:343:/// resolve `WorkTx.proposal_cid` references with only the on-disk paths.
src/runtime/proposal_telemetry.rs:397:    /// `WorkTx.proposal_cid` be byte-stable across reruns of the same
src/runtime/run_summary.rs:107:    pub candidate_proposal_cids: Vec<Cid>,
src/runtime/run_summary.rs:180:            candidate_proposal_cids: candidate_cids.into_iter().collect(),
src/runtime/run_summary.rs:242:            candidate_proposal_cids: vec![Cid([1u8; 32]), Cid([2u8; 32])],
src/runtime/run_summary.rs:261:            candidate_proposal_cids: vec![],
src/runtime/chain_derived_run_facts.rs:30://!    proposal_cids contribute 0
src/runtime/chain_derived_run_facts.rs:56:use crate::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
src/runtime/chain_derived_run_facts.rs:168:/// proposal_cid) are excluded — only telemetry-linked real proposals
src/runtime/chain_derived_run_facts.rs:171:struct WorkTxAttempt {
src/runtime/chain_derived_run_facts.rs:340:                if work.proposal_cid.0 != [0u8; 32] {
src/runtime/chain_derived_run_facts.rs:341:                    if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
src/runtime/chain_derived_run_facts.rs:411:    // TypedTx::Work with non-zero proposal_cid that resolves to a CAS
src/runtime/chain_derived_run_facts.rs:431:            if work.proposal_cid.0 != [0u8; 32] {
src/runtime/chain_derived_run_facts.rs:432:                if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
src/runtime/chain_derived_run_facts.rs:525:    use crate::runtime::proposal_telemetry::{write_to_cas, ProposalTelemetry, TokenCounts};
src/runtime/chain_derived_run_facts.rs:583:        // Pre-write a ProposalTelemetry to CAS so proposal_cid is non-zero.
src/runtime/chain_derived_run_facts.rs:670:    /// linked from accepted L4 WorkTx.proposal_cid →
src/runtime/verify.rs:148:    /// `proposal_cid` resolves to a CAS-resident `ProposalTelemetry`
src/runtime/verify.rs:152:    /// `WorkTx.proposal_cid` resolves to a CAS `ProposalTelemetry`
src/runtime/verify.rs:155:    pub proposal_telemetry_cas_retrievable: bool,
src/runtime/verify.rs:189:    /// and `proposal_telemetry_cas_retrievable` (Gate 5) indicators.
src/runtime/verify.rs:197:            && self.proposal_telemetry_cas_retrievable
src/runtime/verify.rs:336:    let (agent_signatures_verified, proposal_telemetry_cas_retrievable) =
src/runtime/verify.rs:348:        proposal_telemetry_cas_retrievable,
src/runtime/verify.rs:365:/// checking that every `WorkTx.proposal_cid` resolves to a CAS-resident
src/runtime/verify.rs:368:/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
src/runtime/verify.rs:378:    use crate::runtime::proposal_telemetry::read_from_cas as read_telemetry;
src/runtime/verify.rs:393:    let mut proposal_telemetry_cas_retrievable = true;
src/runtime/verify.rs:420:                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
src/runtime/verify.rs:421:                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
src/runtime/verify.rs:422:                if work.proposal_cid.0 != [0u8; 32] {
src/runtime/verify.rs:423:                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
src/runtime/verify.rs:424:                        proposal_telemetry_cas_retrievable = false;
src/runtime/verify.rs:456:        proposal_telemetry_cas_retrievable,
src/runtime/verify.rs:539:        // `proposal_telemetry_cas_retrievable` (Gate 5).
src/runtime/verify.rs:549:            proposal_telemetry_cas_retrievable: true,
src/runtime/verify.rs:569:        r.proposal_telemetry_cas_retrievable = false;
src/runtime/agent_audit_trail.rs:4://! `agent_id`, `prompt_context_hash`, `read_set`, `write_set`, `proposal_cid`,
src/runtime/agent_audit_trail.rs:71:/// (`proposal_cid` + `candidate_proof_cid` + `tx_id`) + how the system
src/runtime/agent_audit_trail.rs:100:    pub proposal_cid: Cid,
src/runtime/agent_audit_trail.rs:157:        h.update(self.proposal_cid.0);
src/runtime/agent_audit_trail.rs:486:        proposal_cid: Cid([0u8; 32]),
src/runtime/agent_audit_trail.rs:522:        proposal_cid: Cid([0u8; 32]),
src/runtime/agent_audit_trail.rs:577:            proposal_cid: Cid([3u8; 32]),
src/runtime/agent_audit_trail.rs:603:        assert_eq!(decoded.proposal_cid, r.proposal_cid);
src/economy/ledger.rs:402:            proposal_cid: Cid([0x13; 32]),
src/bin/audit_dashboard.rs:39:use turingosv4::runtime::proposal_telemetry::read_from_cas as read_proposal_telemetry;
src/bin/audit_dashboard.rs:118:    proposal_telemetry_cas_retrievable: bool,
src/bin/audit_dashboard.rs:282:                if work.proposal_cid.0 != [0u8; 32] {
src/bin/audit_dashboard.rs:283:                    if let Ok(tel) = read_proposal_telemetry(&cas, &work.proposal_cid) {
src/bin/audit_dashboard.rs:473:                    if w.proposal_cid.0 != [0u8; 32] {
src/bin/audit_dashboard.rs:474:                        if let Ok(tel) = read_proposal_telemetry(&cas, &w.proposal_cid) {
src/bin/audit_dashboard.rs:540:            proposal_telemetry_cas_retrievable: replay.proposal_telemetry_cas_retrievable,
src/bin/audit_dashboard.rs:608:        "  proposal_telemetry_cas_retrievable [Gate 5]: {}\n",
src/bin/audit_dashboard.rs:609:        if r.indicators.proposal_telemetry_cas_retrievable { "✓" } else { "✗" }
src/bottom_white/ledger/transition_ledger.rs:1193:            proposal_cid: Cid([0; 32]),
src/bottom_white/cas/schema.rs:12:/// `WorkTx.proposal_cid: Cid` references this.
src/bottom_white/cas/mod.rs:9://! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§1.2 (proposal_cid): CAS root
src/state/typed_tx.rs:223:pub struct WorkTx {
src/state/typed_tx.rs:230:    pub proposal_cid: Cid,                            //  7
src/state/typed_tx.rs:532:    pub proposal_cid: Cid,
src/state/typed_tx.rs:703:            proposal_cid: self.proposal_cid,
src/state/typed_tx.rs:836:pub enum TypedTx {
src/state/typed_tx.rs:1325:            proposal_cid: cid(0x13),
tests/tb_6_agent_audit_trail.rs:4://! `agent_id`, `prompt_context_hash`, `read_set`, `write_set`, `proposal_cid`,
tests/tb_6_agent_audit_trail.rs:61:        proposal_cid: turingosv4::bottom_white::cas::schema::Cid([0x11; 32]),
tests/tb_6_agent_audit_trail.rs:115:    assert!(!recovered.proposal_cid.0.iter().all(|&b| b == 0));
tests/tb_6_agent_audit_trail.rs:209:        "proposal_cid",
tests/tb_6_agent_audit_trail.rs:221:    // write_set / proposal_cid / candidate_proof_cid / tx_id /
tests/tb_3_rsp1_formal_surface.rs:155:        proposal_cid: Default::default(),
src/state/sequencer.rs:1706:            proposal_cid: Default::default(),
src/state/sequencer.rs:1974:        // Now construct WorkTx with parent matching the funded state's state_root.
src/state/sequencer.rs:2280:            proposal_cid: Default::default(),
tests/tb_7r_parent_tx_conformance.rs:33:use turingosv4::runtime::proposal_telemetry::{
tests/tb_7_authoritative_routing.rs:64:        proposal_cid: work.proposal_cid,
tests/tb_6_run_summary.rs:100:        !summary.candidate_proposal_cids.is_empty(),
tests/tb_6_run_summary.rs:135:    assert!(summary.candidate_proposal_cids.is_empty());
tests/tb_6_run_summary.rs:148:        candidate_proposal_cids: vec![],
tests/tb_1_acceptance.rs:115:        proposal_cid: Cid([0x13; 32]),

 succeeded in 109ms:
tests/tb_6_verify_chaintape.rs:1://! TB-6 Atom 4 — `verify_chaintape` integration tests.
tests/tb_6_verify_chaintape.rs:6://! disk → `verify_chaintape` reports all 7 architect-mandated boolean
tests/tb_6_verify_chaintape.rs:22:use turingosv4::runtime::verify::{verify_chaintape, VerifyOptions};
tests/tb_6_verify_chaintape.rs:60:        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:93:    // verify_chaintape replay can pick up pre-seeded balances + open task
tests/tb_6_verify_chaintape.rs:109:        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:135:    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:165:        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:184://   verify_chaintape returns Err(Cas) at open time (catches L4 entries
tests/tb_6_verify_chaintape.rs:196:/// `verify_chaintape` to return Err at CAS-open time. (Pre-fix, this
tests/tb_6_verify_chaintape.rs:200:async fn i90d_tampered_cas_index_breaks_verify_chaintape() {
tests/tb_6_verify_chaintape.rs:215:    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:234:        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default());
tests/tb_6_verify_chaintape.rs:237:        "tampered CAS index must break verify_chaintape at CAS-open time; got Ok({result:?})"
tests/tb_6_verify_chaintape.rs:267:    let pre = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())
tests/tb_6_verify_chaintape.rs:285:        verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default());
tests/tb_6_verify_chaintape.rs:288:        "tampered L4.E row must break verify_chaintape at L4.E-open time; got Ok({result:?})"
tests/tb_6_verify_chaintape.rs:294:/// `verify_chaintape` treats it as an empty L4.E writer (legitimate
tests/tb_6_verify_chaintape.rs:321:    let report = verify_chaintape(&cfg.runtime_repo_path, &cfg.cas_path, &VerifyOptions::default())

exec
/bin/bash -lc "target/debug/audit_dashboard --repo /tmp/tb7r_r2_uyTXAx/runtime_repo --cas /tmp/tb7r_r2_uyTXAx/cas | sed -n '1,140p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
=================================================================
 TB-8 Audit Dashboard — run_id=tb7r-smoke-1 epoch=1
=================================================================

§1 Run metadata
---------------
  head_commit_oid: d4d704beb5ab16782425f75194cae07d83842f5c
  final_state_root: e5199d29e268c057f197ce8492e02429ab7af0046cda510cc08bf55efc4f7647
  final_ledger_root: 68bd2fe6a0b9c394151fa217da3d73087de75d4f894db208ac7376243661fb69
  initial_q_state_loaded_from_disk: true

§2 Chain stats + 7 indicators
------------------------------
  L4 entries:  3
  L4.E entries: 3
  ledger_root_verified              : ✓
  system_signatures_verified        : ✓
  state_reconstructed               : ✓
  economic_state_reconstructed      : ✓
  cas_payloads_retrievable          : ✓
  agent_signatures_verified [Gate 4]: ✓
  proposal_telemetry_cas_retrievable [Gate 5]: ✓
  ALL 7 PASS                        : GREEN

§3 ChainDerivedRunFacts (§4.4 bit-exact set)
---------------------------------------------
  solved                  : false
  verified                : false
  tx_count                : 6
  proposal_count          : 2
  golden_path_token_count : 471
  gp_payload (CID hex)    : -
  gp_path                 : -
  tactic_diversity        : 1
  failed_branch_count     : 3
  chain_oracle_verified   : true ✓ (Lean accepted ≥1 proof; oracle-level)
  chain_economic_finalized: false (always false in TB-7; settlement = TB-9 territory)
  tool_dist:
    step_complete: 1

§4 Per-agent activity
---------------------
  agent_id          | pubkey | Work✓ | Work✗ | Verify✓ | Verify✗
  ------------------+--------+-------+-------+---------+--------
  Agent_0           | ✓      | 1     | 0     | 0       | 1
  tb6-smoke-agent   | ✗      | 0     | 1     | 0       | 0
  tb6-smoke-sponsor | ✗      | 0     | 0     | 0       | 0

§5 Proposal flow (chronological by logical_t)
----------------------------------------------
  side  | t   | tx_kind         | agent      | tactic     | branch     | oracle | reject
  ------+-----+-----------------+------------+------------+------------+--------+-------
  L4.E  |   0 | TaskOpen        | tb6-smoke-sponsor | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Work            | tb6-smoke-agent | -          | -          | -      | PolicyViolation
  L4.E  |   0 | Verify          | Agent_0    | -          | -          | -      | PolicyViolation
  L4    |   1 | TaskOpen        | tb7-7-sponsor | -          | -          | -      | -
  L4    |   2 | EscrowLock      | tb7-7-sponsor | -          | -          | -      | -
  L4    |   3 | Work            | Agent_0    | step_complete | Agent_0.b1 | ✓      | -
        payload: calc ⏎   f 1 = 5 * 1 + 4 := by rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by norm_n

§6 Branch lineage (parent_tx → child_tx via ProposalTelemetry.parent_tx)
------------------------------------------------------------------------
  parent_tx_state: SingletonGoldenPathValid (B′ singleton solve — parent_tx=None correct; conformance test demonstrates plumbing)
  edges: (none — see parent_tx_state above for interpretation)

§7 Golden path (root → oracle-verified WorkTx)
------------------------------------------------
  ✓depth=0  [ORACLE] | agent=Agent_0 | tactic=step_complete | tx=worktx-task-n1_mathd_algebra_171_1777694645204-omega-pertactic-1
           payload: calc ⏎   f 1 = 5 * 1 + 4 := by rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by norm_n

§8 Cross-checks
---------------
  audit_trail_rows         : 2
  chain_proposal_count     : 2
  audit_rows == proposal_count: ✓
  audit_trail_chain_valid     : ✓
  (Note: pre-TB-7.6 the agent_audit_trail.jsonl is populated only
   by the synthetic-seed hook; full per-LLM-proposal audit-trail
   wiring is part of TB-7.6 carry-forward action #4 / #5.)


exec
/bin/bash -lc "set -euo pipefail
CAS=/tmp/tb7r_r2_uyTXAx/cas
for OID in 730a689d4bff787c0abdc6c14e73229dc80a0898 f58607aa7b44ce89b0ae0a90192e050d8ae01371 7a66b55285ce8ca22f6b44656b1e8c4ed60b4f1a 4c49ac7701ce9cb92667d61c0cbe6f6f888d49cd 1c2d6cce6c20c7623949a89d3a96f2912f832f9c 467d358f52191ec017b87eaf6c55c494b208a298; do
  printf 'OID=%s type=' \""'$OID"
  git --git-dir="$CAS/.git" cat-file -t "$OID"
  printf '"'size='
  git --git-dir=\""'$CAS/.git" cat-file -s "$OID"
  git --git-dir="$CAS/.git" cat-file -p "$OID" | head -c 500
  printf '"'\\n---\\n'
done" in /home/zephryj/projects/turingosv4
 succeeded in 148ms:
blob
size=184
       Agent_0U�r�	RhA�/����*=�ca2��"ኂ|
F]Y�ɱ�.=���
�&�u.��(��o6Xe��       step_complete      �       6                       
Agent_0.b1 �# ��ꩡl�r8t�	�.���4iȱ�^r�
---
OID=467d358f52191ec017b87eaf6c55c494b208a298 type=blob
size=361
           @worktx-task-n1_mathd_algebra_171_1777694645204-omega-pertactic-1       'task-n1_mathd_algebra_171_1777694645204��il&��/��/�sJ���hq����J����|       Agent_0              k.read              k.write�����ħ�PB݈�S�?�� ���qx�e~Q              acc1                   �_�&s�/MI�W��b�p�BPT@��� Ð��s|�T��_�D;��ݕ��>'��2m��$�˚       
---

exec
/bin/bash -lc "sed -n '200,260p' src/state/typed_tx.rs && sed -n '836,900p' src/state/typed_tx.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
pub enum TxStatus {
    Pending,
    Accepted,
    Rejected(RejectionClass),
    FinalizedReward(MicroCoin),
    FinalizedSlash(SlashEvidenceCid),
}

// ────────────────────────────────────────────────────────────────────────────
// § 5 (cont'd) — Typed tx structs (STATE spec § 1.2-1.6 + § 3.6)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX § 1.2 — agent-submitted work transaction (12-field schema;
/// **D-1 divergence**: field 12 `status: TxStatus` is excluded from canonical
/// wire bytes — TxStatus is runner book-keeping per CO1.1.4-pre1 spec § 5).
///
/// This is the per-tx struct that the CO1.7 sequencer hands to
/// `step_transition` (CO1.7.5 body atom). The `signature` is over
/// `WorkSigningPayload::canonical_digest()` — i.e. the projection produced by
/// `WorkTx::to_signing_payload()` (excludes the signature field itself; per
/// v1.1 P1 the digest pre-image carries the `b"turingosv4.agent_sig.work.v1"`
/// domain prefix).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkTx {
    pub tx_id: TxId,                                  //  1
    pub task_id: TaskId,                              //  2
    pub parent_state_root: Hash,                      //  3
    pub agent_id: AgentId,                            //  4
    pub read_set: BTreeSet<ReadKey>,                  //  5
    pub write_set: BTreeSet<WriteKey>,                //  6
    pub proposal_cid: Cid,                            //  7
    pub predicate_results: PredicateResultsBundle,    //  8 (runner-stamped)
    pub stake: StakeMicroCoin,                        //  9
    pub signature: AgentSignature,                    // 10
    pub timestamp_logical: u64,                       // 11
    // 12: TxStatus — D-1 elision; runtime-only.
}

/// TRACE_MATRIX § 1.3 — verifier verdict transaction.
///
/// **TB-4 (2026-04-30) schema bump**: `parent_state_root: Hash` added as
/// field #2 (per TB-4 charter § 4.1 + directive Q2). Constitutional shape
/// — every accepted-tx variant must carry an explicit parent_state_root
/// for the StaleParent gate. Pre-TB-4 has no production-accepted VerifyTx
/// rows (dispatch arm was `NotYetImplemented`), so the wire bump is harmless.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifyTx {
    pub tx_id: TxId,                       //  1
    pub parent_state_root: Hash,           //  2  (TB-4 NEW)
    pub target_work_tx: TxId,              //  3
    pub verifier_agent: AgentId,           //  4
    pub bond: StakeMicroCoin,              //  5
    pub verdict: VerifyVerdict,            //  6
    pub signature: AgentSignature,         //  7
    pub timestamp_logical: u64,            //  8
}

impl Default for VerifyVerdict {
    fn default() -> Self {
        Self::Confirm
    }
pub enum TypedTx {
    Work(WorkTx),
    Verify(VerifyTx),
    Challenge(ChallengeTx),
    Reuse(ReuseTx),
    FinalizeReward(FinalizeRewardTx),
    TaskExpire(TaskExpireTx),
    TerminalSummary(TerminalSummaryTx),
    TaskOpen(TaskOpenTx),         // TB-3 RSP-1 formal surface
    EscrowLock(EscrowLockTx),     // TB-3 RSP-1 formal surface
    ChallengeResolve(ChallengeResolveTx), // TB-5 RSP-3.0/3.1 system-emitted resolution
}

impl TypedTx {
    /// Project to the [`TxKind`] discriminator stored in `LedgerEntry.tx_kind`.
    pub fn tx_kind(&self) -> crate::bottom_white::ledger::transition_ledger::TxKind {
        use crate::bottom_white::ledger::transition_ledger::TxKind;
        match self {
            Self::Work(_) => TxKind::Work,
            Self::Verify(_) => TxKind::Verify,
            Self::Challenge(_) => TxKind::Challenge,
            Self::Reuse(_) => TxKind::Reuse,
            Self::FinalizeReward(_) => TxKind::FinalizeReward,
            Self::TaskExpire(_) => TxKind::TaskExpire,
            Self::TerminalSummary(_) => TxKind::TerminalSummary,
            Self::TaskOpen(_) => TxKind::TaskOpen,
            Self::EscrowLock(_) => TxKind::EscrowLock,
            Self::ChallengeResolve(_) => TxKind::ChallengeResolve,
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// § 8 HasSubmitter trait (STATE spec § 3.6.5 v1.3)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX STATE spec § 3.6.5 v1.3 — submitter resolution trait used
/// by the implicit-init step in agent-submitted transitions. System-emitted
/// transitions return `None` (no agent to init).
pub trait HasSubmitter {
    fn submitter_id(&self) -> Option<AgentId>;
}

impl HasSubmitter for WorkTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.agent_id.clone())
    }
}

impl HasSubmitter for VerifyTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.verifier_agent.clone())
    }
}

impl HasSubmitter for ChallengeTx {
    fn submitter_id(&self) -> Option<AgentId> {
        Some(self.challenger_agent.clone())
    }
}

impl HasSubmitter for ReuseTx {
    fn submitter_id(&self) -> Option<AgentId> {
        None
    }

exec
/bin/bash -lc "sed -n '80,190p' src/runtime/proposal_telemetry.rs && sed -n '330,370p' src/runtime/proposal_telemetry.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/// TRACE_MATRIX FC1-N14: a single tool-call from the LLM trajectory. Records
/// only the structural shape (tool name + 32-byte argument hash + 32-byte
/// result hash). The full argument / result bytes go into separate CAS objects
/// or stay agent-internal — the audit trail records the call **happened**, not
/// the chain-of-thought that picked it.
///
/// Forbidden contents (TB-6 charter §6 #11 + TB-7 §6 inheritance): raw model
/// deliberation, raw tool transcripts, internal reasoning, raw prompt/completion
/// strings. Hashes only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToolCallRecord {
    pub tool_id: String,
    pub args_hash: Hash,
    pub result_hash: Hash,
}

// ── Telemetry record ────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: the per-WorkTx LLM proposal telemetry object pointed
/// to by `WorkTx.proposal_cid`. Schema per ARCHITECT_RULING 2026-05-01 D5 +
/// TB-7 charter §4.5.
///
/// **Field set (binding; do NOT add fields without architect ratification)**:
/// 1. `agent_id` — must match `WorkTx.agent_id`
/// 2. `prompt_context_hash` — 32-byte sha256; same as `AgentProposalRecord.prompt_context_hash`
/// 3. `proposal_artifact_cid` — CID of the actual proposal payload bytes
///    (proof artifact / candidate tactic body / tool program); separate from
///    this telemetry record's own CID
/// 4. `candidate_tactic` — short identifier for the proposed tactic
///    (e.g. "nlinarith", "ring", "rfl", "induction"); aggregated by
///    `tactic_diversity` in `ChainDerivedRunFacts`
/// 5. `token_counts` — prompt / completion / tool token counts; aggregated
///    by `golden_path_token_count` in `ChainDerivedRunFacts`
/// 6. `tool_calls` — ordered manifest of tool invocations during proposal
///    construction; aggregated by `tool_dist` in `ChainDerivedRunFacts`
/// 7. `branch_id` — short branch label (e.g. "n1.b0", "swarm_a.b3"); used by
///    `failed_branch_count` aggregator
/// 8. `parent_tx` — `TxId` of the parent WorkTx if this proposal was
///    derivative; `None` for root proposals
/// 9. **TB-7.7 D4**: `verification_result_cid` — optional CID to a
///    `VerificationResult` CAS object recording the Lean oracle's
///    verdict (exit code + verified flag + proof artifact hash).
///    `None` for proposals not yet Lean-verified (append-branch
///    intermediate steps); `Some(cid)` for OMEGA-accept proposals
///    where the evaluator has run Lean and recorded the verdict.
///    Replay readers use this to compute `chain_oracle_verified`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalTelemetry {
    pub agent_id: AgentId,
    pub prompt_context_hash: Hash,
    pub proposal_artifact_cid: Cid,
    pub candidate_tactic: String,
    pub token_counts: TokenCounts,
    pub tool_calls: Vec<ToolCallRecord>,
    pub branch_id: String,
    pub parent_tx: Option<TxId>,
    /// TB-7.7 D4: optional CID of the matching `VerificationResult` CAS
    /// object (`runtime::verification_result::VerificationResult`).
    /// Schema-additive; `None` preserves backward compat with pre-TB-7.7
    /// telemetry.
    #[serde(default)]
    pub verification_result_cid: Option<Cid>,
}

impl ProposalTelemetry {
    /// TRACE_MATRIX FC1-N14: convenience constructor for the common case where
    /// the proposal has no parent (root proposal). Used by Atom 2 evaluator
    /// hooks that don't yet track branch lineage.
    pub fn new_root(
        agent_id: AgentId,
        prompt_context_hash: Hash,
        proposal_artifact_cid: Cid,
        candidate_tactic: String,
        token_counts: TokenCounts,
        branch_id: String,
    ) -> Self {
        Self {
            agent_id,
            prompt_context_hash,
            proposal_artifact_cid,
            candidate_tactic,
            token_counts,
            tool_calls: Vec::new(),
            branch_id,
            parent_tx: None,
            verification_result_cid: None,
        }
    }

    /// TRACE_MATRIX FC1-N14: TB-7 Atom 2 — high-level builder for the
    /// evaluator hot path.
    ///
    /// **TB-7.7 fix (2026-05-01)**: this function now ACTUALLY WRITES the
    /// proposal payload bytes to CAS. Pre-TB-7.7 it computed
    /// `proposal_artifact_cid = sha256(payload_bytes)` but never stored
    /// the bytes — meaning a chain reader could verify "a payload with
    /// this hash existed" but could not recover the payload content from
    /// ChainTape + CAS alone (architect ruling 2026-05-01 ultrathink turn
    /// flagged this as the #1 hidden hole in real chaintape).
    ///
    /// Now `proposal_artifact_cid` is the CID returned by
    /// `cas.put(payload_bytes, ObjectType::ProposalPayload, ...)`. The
    /// bytes are durably stored under that CID.
    ///
    /// `parent_tx` is `None` here for backward compat; callers that want
    /// to record branch lineage should use
    /// [`build_for_evaluator_append_with_parent`] instead.
    pub fn build_for_evaluator_append(
        cas_store: &mut CasStore,
        run_id: &str,
        agent_id: &str,
/// `verify_chaintape` extension to retrieve and validate
/// `WorkTx.proposal_cid` references during replay.
pub fn read_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<ProposalTelemetry, ProposalTelemetryError> {
    let bytes = cas.get(cid)?;
    canonical_decode::<ProposalTelemetry>(&bytes)
        .map_err(|e| ProposalTelemetryError::Codec(e.to_string()))
}

/// TRACE_MATRIX FC1-N14: convenience — open a CAS at `cas_path` and read the
/// telemetry record at `cid`. Used by `verify_chaintape` CLI which needs to
/// resolve `WorkTx.proposal_cid` references with only the on-disk paths.
pub fn read_from_cas_path(
    cas_path: &Path,
    cid: &Cid,
) -> Result<ProposalTelemetry, ProposalTelemetryError> {
    let cas = CasStore::open(cas_path)?;
    read_from_cas(&cas, cid)
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use tempfile::TempDir;

    fn fresh_cas() -> (TempDir, CasStore) {
        let dir = TempDir::new().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("open cas");
        (dir, cas)
    }

    fn fresh_record(agent: &str, branch: &str) -> ProposalTelemetry {
        let mut h = Sha256::new();
        h.update(b"telemetry.test.prompt_context");
        h.update(agent.as_bytes());
        let prompt_hash = Hash(h.finalize().into());

exec
/bin/bash -lc "sed -n '1,260p' src/bottom_white/cas/store.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
//! CAS store backed by git2-rs blob layer.
//!
//! Each runtime_repo (per spec § 5.2.2 cell isolation) has its own CasStore.
//! Objects are content-addressed by `Cid` (sha256 of content); git's sha-1
//! OID is recorded but not canonical.
//!
//! **CO1.4-extra (this atom)** adds index persistence: the `Cid → metadata`
//! map is durably persisted to a sidecar JSONL file at
//! `<repo_path>/.turingos_cas_index.jsonl`. On `CasStore::open()` the sidecar
//! is replayed into an in-memory BTreeMap; on `CasStore::put()` (new entries
//! only) one JSONL line is appended + flushed. This closes the Art 0.2
//! tape-canonicality cold-replay gate that CO1.7 spec § 0 + CO1.1.4-pre1
//! v1.1 § 0.1 declared a hard prerequisite for `replay_full_transition`
//! (CO1.7-impl A4).
//!
//! **Design choice (sidecar JSONL)**: chosen over (b) git-tag manifest /
//! (c) bincode index + WAL because (a) is the simplest deterministic
//! append-only artifact, replayable from scratch, easy to audit by reading.
//! Per "压缩即智能" — pick simplest correct shape; upgrade later if profiling
//! shows O(N)-on-restart cost is real.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3 + spec-§5.2.2 (cell isolation): CAS store
//! /// TRACE_MATRIX CO1.7 spec § 0 + CO1.1.4-pre1 § 0.1 cross-atom ordering:
//! /// CAS index persistence — required by `replay_full_transition` cold-restart.

use git2::{ObjectType as Git2ObjectType, Repository};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::schema::{CasObjectMetadata, Cid, ObjectType};

const CAS_INDEX_FILENAME: &str = ".turingos_cas_index.jsonl";

#[derive(Debug)]
pub enum CasError {
    /// git2-rs underlying error.
    Git2(git2::Error),
    /// Cid not found in this CasStore's metadata index.
    CidNotFound(Cid),
    /// Content stored at git OID but Cid metadata absent (corrupted index).
    MetadataMissing(Cid),
    /// Content's sha256 doesn't match the asserted Cid (corruption).
    CidMismatch { expected: Cid, computed: Cid },
    /// I/O error reading or writing the CO1.4-extra sidecar index file.
    IoError(io::Error),
    /// JSON-deserialization error on a sidecar index line. Includes 1-based
    /// line number for diagnostics.
    IndexParse { line: usize, error: String },
}

impl std::fmt::Display for CasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Git2(e) => write!(f, "git2 backend error: {e}"),
            Self::CidNotFound(c) => write!(f, "{c} not found in CAS index"),
            Self::MetadataMissing(c) => write!(f, "{c} metadata missing (index corrupted)"),
            Self::CidMismatch { expected, computed } => write!(
                f,
                "CAS content corruption: expected {expected}, computed {computed}"
            ),
            Self::IoError(e) => write!(f, "cas index I/O error: {e}"),
            Self::IndexParse { line, error } => {
                write!(f, "cas index parse error at line {line}: {error}")
            }
        }
    }
}

impl std::error::Error for CasError {}

impl From<git2::Error> for CasError {
    fn from(e: git2::Error) -> Self {
        Self::Git2(e)
    }
}

impl From<io::Error> for CasError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

fn cas_index_path(repo_path: &Path) -> PathBuf {
    repo_path.join(CAS_INDEX_FILENAME)
}

/// CO1.4-extra: read the sidecar JSONL into an in-memory index.
/// Strict mode — any malformed line aborts the load (per Art 0.2: a
/// corrupted index means the tape is non-canonical; abort + diagnose
/// is more honest than skip-and-warn).
fn load_index_from_sidecar(repo_path: &Path) -> Result<BTreeMap<Cid, CasObjectMetadata>, CasError> {
    let path = cas_index_path(repo_path);
    let mut index = BTreeMap::new();
    if !path.exists() {
        return Ok(index);
    }
    let content = std::fs::read_to_string(&path)?;
    for (i, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let meta: CasObjectMetadata =
            serde_json::from_str(line).map_err(|e| CasError::IndexParse {
                line: i + 1,
                error: e.to_string(),
            })?;
        index.insert(meta.cid, meta);
    }
    Ok(index)
}

/// CO1.4-extra: append a single JSONL line for a newly-created CAS object.
/// Followed by `sync_data` for durability.
///
/// **TB-7.6 fix (2026-05-01)**: write the JSON line + trailing newline
/// in ONE `write_all` call instead of two. POSIX `O_APPEND` guarantees
/// atomicity for individual writes ≤ PIPE_BUF (4096 bytes typical;
/// CasObjectMetadata serializes to ~300-400 bytes). Pre-fix used two
/// separate `write_all` calls (`serialized` then `b"\n"`), which could
/// interleave with another concurrent writer's append, producing
/// corrupted lines like `{...}{...}` (no separator). Discovered during
/// TB-7 real-LLM smoke runs 2 + 5 (mathd_algebra_171 + mathd_numbertheory_5)
/// where evaluator opens multiple CasStore handles concurrently for
/// per-tx writes (Atom 1.5 ProposalTelemetry CAS + Atom 5
/// agent_audit_trail synthetic seed + Atoms 2/3 evaluator hot-path
/// telemetry writes). See
/// `handover/evidence/tb_7_real_smoke_5_problems_2026-05-01/README.md` §3.
fn append_to_sidecar(repo_path: &Path, meta: &CasObjectMetadata) -> Result<(), CasError> {
    let path = cas_index_path(repo_path);
    let serialized = serde_json::to_string(meta).map_err(|e| CasError::IndexParse {
        line: 0,
        error: format!("serialize: {e}"),
    })?;
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    // Atomic single-write append: serialize + newline in one buffer.
    let mut line = serialized.into_bytes();
    line.push(b'\n');
    f.write_all(&line)?;
    f.sync_data()?;
    Ok(())
}

/// Content-addressable store backed by git's blob object database.
#[derive(Debug)]
pub struct CasStore {
    repo_path: PathBuf,
    /// Cid → metadata index. BTreeMap per spec § 2 I-BTREE.
    index: BTreeMap<Cid, CasObjectMetadata>,
}

impl CasStore {
    /// Open or initialize a CAS store at the given runtime_repo path.
    /// Creates the git repo if it doesn't exist. **CO1.4-extra**: replays
    /// the sidecar `.turingos_cas_index.jsonl` (if any) into the in-memory
    /// index, restoring all metadata that was durably appended in prior
    /// sessions.
    pub fn open(repo_path: &Path) -> Result<Self, CasError> {
        let repo_path = repo_path.to_path_buf();
        let _repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path)?,
        };
        let index = load_index_from_sidecar(&repo_path)?;
        Ok(Self { repo_path, index })
    }

    fn open_repo(&self) -> Result<Repository, CasError> {
        Repository::open(&self.repo_path).map_err(CasError::from)
    }

    /// Store content; returns its Cid. Idempotent — same content → same Cid.
    pub fn put(
        &mut self,
        content: &[u8],
        object_type: ObjectType,
        creator: &str,
        created_at_logical_t: u64,
        schema_id: Option<String>,
    ) -> Result<Cid, CasError> {
        let cid = Cid::from_content(content);
        let repo = self.open_repo()?;
        let git_oid = repo.blob(content)?;

        // If already in index, idempotent: just return Cid (content addressing
        // guarantees same content → same Cid → already present)
        if self.index.contains_key(&cid) {
            return Ok(cid);
        }

        let metadata = CasObjectMetadata {
            cid,
            backend_oid_hex: git_oid.to_string(),
            object_type,
            creator: creator.to_string(),
            created_at_logical_t,
            schema_id,
            size_bytes: content.len() as u64,
        };
        // CO1.4-extra: durably append BEFORE inserting into in-memory index
        // (so a crash mid-write leaves the runtime in a consistent state —
        // either the entry is durably recorded AND in-memory, or neither).
        append_to_sidecar(&self.repo_path, &metadata)?;
        self.index.insert(cid, metadata);
        Ok(cid)
    }

    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
        let metadata = self
            .index
            .get(cid)
            .ok_or(CasError::CidNotFound(*cid))?;
        let repo = self.open_repo()?;
        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
            .map_err(CasError::Git2)?;
        let blob = repo.find_blob(git_oid)?;
        let content = blob.content().to_vec();

        // Verify content sha256 matches Cid (defense against corruption).
        let mut h = Sha256::new();
        h.update(&content);
        let computed = Cid(h.finalize().into());
        if &computed != cid {
            return Err(CasError::CidMismatch {
                expected: *cid,
                computed,
            });
        }

        Ok(content)
    }

    /// Get metadata only (no content fetch).
    pub fn metadata(&self, cid: &Cid) -> Option<&CasObjectMetadata> {
        self.index.get(cid)
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Merkle root over all CAS object metadata; deterministic per BTreeMap order.
    pub fn merkle_root(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        for (_cid, meta) in &self.index {
            h.update(meta.canonical_hash());
        }
        h.finalize().into()
    }
}

exec
/bin/bash -lc "sed -n '365,440p' src/runtime/verify.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
/// checking that every `WorkTx.proposal_cid` resolves to a CAS-resident
/// ProposalTelemetry object.
///
/// Returns `(agent_signatures_verified, proposal_telemetry_cas_retrievable)`.
/// Both default to `true` when the manifest doesn't exist or when no
/// Work/Verify entries are present (no signatures to verify ≠ failure).
fn verify_agent_artifacts(
    runtime_repo_path: &Path,
    cas_store: &CasStore,
    entries: &[LedgerEntry],
) -> (bool, bool) {
    use crate::bottom_white::ledger::transition_ledger::canonical_decode;
    use crate::runtime::agent_keypairs::{verify_agent_signature, AgentPubkeyManifest};
    use crate::runtime::proposal_telemetry::read_from_cas as read_telemetry;
    use crate::state::typed_tx::TypedTx;

    let manifest_path = runtime_repo_path.join("agent_pubkeys.json");
    if !manifest_path.exists() {
        // No agent_pubkeys.json (legacy / pre-Atom-1 chain). Both indicators
        // trivially true since there are no agent-side artifacts to fail.
        return (true, true);
    }
    let manifest = match AgentPubkeyManifest::load(&manifest_path) {
        Ok(m) => m,
        Err(_) => return (false, false), // manifest unparseable = both fail
    };

    let mut agent_signatures_verified = true;
    let mut proposal_telemetry_cas_retrievable = true;

    for entry in entries {
        // Get the typed payload from CAS.
        let payload_bytes = match cas_store.get(&entry.tx_payload_cid) {
            Ok(b) => b,
            Err(_) => continue, // cas_payloads_retrievable already covers this
        };
        let typed_tx: TypedTx = match canonical_decode(&payload_bytes) {
            Ok(tx) => tx,
            Err(_) => continue, // payload decode error already covered upstream
        };

        match &typed_tx {
            TypedTx::Work(work) => {
                // Gate 4 — verify WorkTx signature.
                let payload = work.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&work.agent_id);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&work.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }
                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
                if work.proposal_cid.0 != [0u8; 32] {
                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
                        proposal_telemetry_cas_retrievable = false;
                    }
                }
            }
            TypedTx::Verify(verify) => {
                // Gate 4 — verify VerifyTx signature.
                let payload = verify.to_signing_payload();
                let digest = payload.canonical_digest();
                let pubkey_opt = manifest.get(&verify.verifier_agent);
                match pubkey_opt {
                    None => agent_signatures_verified = false,
                    Some(pubkey) => {
                        if verify_agent_signature(&verify.signature, &digest, &pubkey).is_err() {
                            agent_signatures_verified = false;
                        }
                    }
                }

exec
/bin/bash -lc "rg \"fn canonical_decode|canonical_encode|bincode\" -n src | sed -n '1,120p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
src/top_white/predicates/registry.rs:62:        // (Avoiding bincode dep in lib for now; upgrade later if v1.4 conformance test demands.)
src/economy/ledger.rs:41:use crate::bottom_white::ledger::transition_ledger::{canonical_encode, TxKind};
src/economy/ledger.rs:53:/// SHA-256 over the bincode-canonical encoding of the source `TypedTx`,
src/economy/ledger.rs:54:/// re-using the lower-level `canonical_encode` from `transition_ledger`.
src/economy/ledger.rs:64:    /// SHA-256 of `canonical_encode(tx)` — content-address of the payload.
src/economy/ledger.rs:111:    /// `canonical_encode` of the source `TypedTx` failed.
src/economy/ledger.rs:134:            Self::Encode(e) => write!(f, "canonical_encode failed: {}", e),
src/economy/ledger.rs:190:        let bytes = canonical_encode(tx).map_err(|e| LedgerError::Encode(e.to_string()))?;
src/bottom_white/ledger/system_keypair.rs:86:/// arrays > 32). With `bincode` + `fixed_int_encoding` this writes 64 raw bytes —
src/bottom_white/ledger/system_keypair.rs:98:/// (deterministic under bincode `fixed_int_encoding` → 64 raw bytes; no length prefix
src/bottom_white/cas/store.rs:17://! (c) bincode index + WAL because (a) is the simplest deterministic
src/bottom_white/ledger/transition_ledger.rs:133:    /// Canonical SHA-256 digest. Stable wire format (NOT bincode/serde dependent).
src/bottom_white/ledger/transition_ledger.rs:554:// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
src/bottom_white/ledger/transition_ledger.rs:557:/// `bincode::config` used for the canonical `LedgerEntry` wire format.
src/bottom_white/ledger/transition_ledger.rs:562:/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
src/bottom_white/ledger/transition_ledger.rs:565:fn bincode_canonical_config() -> impl bincode::config::Config {
src/bottom_white/ledger/transition_ledger.rs:566:    bincode::config::standard()
src/bottom_white/ledger/transition_ledger.rs:574:pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
src/bottom_white/ledger/transition_ledger.rs:575:    bincode::serde::encode_to_vec(value, bincode_canonical_config())
src/bottom_white/ledger/transition_ledger.rs:579:/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
src/bottom_white/ledger/transition_ledger.rs:581:pub fn canonical_decode<T: serde::de::DeserializeOwned>(
src/bottom_white/ledger/transition_ledger.rs:585:        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
src/bottom_white/ledger/transition_ledger.rs:627:///     - `entry_canonical` = bincode v2 BE + fixed-int encoding of the full
src/bottom_white/ledger/transition_ledger.rs:769:        let canonical = canonical_encode(entry).map_err(|e| {
src/bottom_white/ledger/transition_ledger.rs:770:            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
src/bottom_white/ledger/transition_ledger.rs:1218:        let bytes = canonical_encode(typed_tx).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1444:        let bad_bytes = b"\xff\xff this is not a valid bincode TypedTx";
src/bottom_white/ledger/transition_ledger.rs:1503:    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).
src/bottom_white/ledger/transition_ledger.rs:1507:        let bytes = canonical_encode(&e1).expect("encode");
src/bottom_white/ledger/transition_ledger.rs:1512:        let bytes_again = canonical_encode(&e1).expect("encode again");
src/state/sequencer.rs:33:    append, canonical_encode, LedgerEntry, LedgerEntrySigningPayload, LedgerWriter,
src/state/sequencer.rs:65:/// `canonical_encode` is — and TB-2 wants a single short call site that
src/state/sequencer.rs:70:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:120:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:131:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:154:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:165:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:183:    h.update(canonical_encode(tx).expect("TypedTx is canonical-encodable"));
src/state/sequencer.rs:1401:        let payload_bytes = canonical_encode(tx)
src/state/sequencer.rs:1535:        let payload_bytes = canonical_encode(&tx)
src/state/typed_tx.rs:482:// prefix** before the bincode-canonical body bytes. This implements:
src/state/typed_tx.rs:484://   sig_input = sha256(b"turingosv4.<actor>.<purpose>.v1" || canonical_encode(payload))
src/state/typed_tx.rs:486:// Property: even if two distinct payload TYPES happen to bincode-encode to
src/state/typed_tx.rs:515:    use crate::bottom_white::ledger::transition_ledger::canonical_encode;
src/state/typed_tx.rs:516:    let body = canonical_encode(value).expect("canonical_encode of signing payload");
src/state/typed_tx.rs:1271:    use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/state/typed_tx.rs:1285:        let bytes = canonical_encode(value).expect("encode");
src/state/typed_tx.rs:1432:            let bytes = canonical_encode(&tx).expect("encode");
src/state/typed_tx.rs:1442:        let bytes_a = canonical_encode(&tx).expect("encode a");
src/state/typed_tx.rs:1443:        let bytes_b = canonical_encode(&tx).expect("encode b");
src/state/typed_tx.rs:1455:            let bytes = canonical_encode(&outer).expect("encode");
src/state/typed_tx.rs:1519:    /// (Different field shapes + bincode variant tags → ANY collision is a bincode
src/state/typed_tx.rs:1557:    /// this test locks that bincode honors the iteration order — defensive against
src/state/typed_tx.rs:1573:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1574:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1575:        let bytes_c = canonical_encode(&tx_c).expect("encode c");
src/state/typed_tx.rs:1594:            let bytes = canonical_encode(&tx).expect("encode default");
src/state/typed_tx.rs:1604:    /// even if their bincode bodies COULD overlap, the SHA-256 inputs differ.
src/state/typed_tx.rs:1787:        let bytes_a = canonical_encode(&tx_a).expect("encode a");
src/state/typed_tx.rs:1788:        let bytes_b = canonical_encode(&tx_b).expect("encode b");
src/state/typed_tx.rs:1789:        let bytes_c = canonical_encode(&tx_c).expect("encode c");
src/runtime/verification_result.rs:28:use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/runtime/verification_result.rs:144:        canonical_encode(record).map_err(|e| VerificationResultError::Codec(e.to_string()))?;
src/runtime/proposal_telemetry.rs:42://! - canonical-encoded (bincode v2 BE + fixed-int) for byte-stable CID
src/runtime/proposal_telemetry.rs:53:use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/runtime/proposal_telemetry.rs:318:        canonical_encode(record).map_err(|e| ProposalTelemetryError::Codec(e.to_string()))?;
src/runtime/agent_audit_trail.rs:14://! 1. The full `AgentProposalRecord` is canonical-encoded (bincode) and stored
src/runtime/agent_audit_trail.rs:50:use crate::bottom_white::ledger::transition_ledger::{canonical_decode, canonical_encode};
src/runtime/agent_audit_trail.rs:82:/// Stored canonical-encoded (bincode v2 BE + fixed-int) so the CID is
src/runtime/agent_audit_trail.rs:261:    let bytes = canonical_encode(record).map_err(|e| AgentAuditError::Codec(e.to_string()))?;
src/runtime/agent_audit_trail.rs:595:    fn nine_required_fields_round_trip_through_canonical_encode() {
src/runtime/agent_audit_trail.rs:597:        let bytes = canonical_encode(&r).expect("encode");

exec
/bin/bash -lc "sed -n '1,120p' src/bottom_white/ledger/transition_ledger.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
//! L4 Transition Ledger (CO1.7) — implementation atom.
//!
//! TRACE_MATRIX FC2-Append: canonical envelope appended to L4 once a transition is accepted.
//! TRACE_MATRIX WP § 5.L4: ChainTape Layer 4 spine; one LedgerEntry per accepted transition.
//! TRACE_MATRIX § 1-§ 8 (CO1_7_TRANSITION_LEDGER_v1_2026-04-28 v1.2): schema +
//! append() + replay_chain_integrity() + replay_full_transition() + Git2LedgerWriter.
//!
//! **Status**: CO1.7 spec PASS/PASS (3 rounds) + CO1.7-impl bundle PASS/PASS
//! (3 rounds: A1+A2+A3+A4 + CO1.4-extra). Per-kind transition function bodies
//! deferred to CO1.7.5 (NotYetImplemented stubs in `src/state/sequencer.rs`).
//!
//! v1 → v1.1 changes (smoke for round-2 dual audit):
//! - C1: two-mode replay enum (ChainOnly v1; FullTransition CO1.7.5+); skeleton now
//!   exposes `replay_chain_integrity` only (renamed for honesty).
//! - K1: sequencer dual-counter design — documented in spec § 3; skeleton has no
//!   sequencer code (deferred to CO1.7.5).
//! - K2: `parent_ledger_root: Hash` field added + bound in signing payload (transplant
//!   defense); new test asserts replay rejects parent_ledger_root tamper.
//! - K3: L4/L5 boundary clarified — CO1.7 owns ledger_root + commit-chain head_t;
//!   CO1.8 owns state_root mutation. Skeleton reflects boundary (no state_root mutation).
//! - K5: `TxKind::Slash` DROPPED for v4 (deferred to CO P2.5).
//! - K6: `#[repr(u8)]` + explicit discriminants on TxKind.
//! - K7: +2 conformance tests (parent_ledger_root tamper, digest exclusion).
//! - G1: `extensions: BTreeMap<String, Vec<u8>>` forward-compat field (empty in v1).
//! - C3 / Q8: signing target is `LedgerEntrySigningPayload` (separate struct) ready to
//!   ride a `CanonicalMessage::LedgerEntrySigning(_)` variant when CO1.7.5+ extends
//!   `system_keypair` (Wave 4-B additive extension). Skeleton has the payload struct
//!   + canonical_digest method; the actual CanonicalMessage extension is deferred.
//! - Q9: canonical_digest now lives on LedgerEntrySigningPayload, not LedgerEntry —
//!   structurally enforces "derivatives excluded".
//! - D1: epoch is bound in signing payload (Codex security wins over Gemini orthogonality).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use git2::{ObjectType as Git2ObjectType, Repository, Signature as GitSignature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::bottom_white::cas::schema::Cid;
use crate::bottom_white::ledger::system_keypair::{SystemEpoch, SystemSignature};
use crate::state::q_state::Hash;

// ────────────────────────────────────────────────────────────────────────────
// § 1 LedgerEntry — the stored record (11 fields per v1.1)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append: discriminator for the typed payload behind a CAS Cid.
/// **K6**: `#[repr(u8)]` + explicit discriminants for stable cast in canonical digest.
/// **K5**: NO `Slash` variant — ChallengeCourt slash event deferred to CO P2.5 atom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum TxKind {
    Work            = 0,
    Verify          = 1,
    Challenge       = 2,
    Reuse           = 3,
    FinalizeReward  = 4,
    TaskExpire      = 5,
    TerminalSummary = 6,
    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted task
    /// market registration; metadata-only (no money movement).
    TaskOpen        = 7,
    /// TB-3 RSP-1 formal-tx-surface (charter § 4.1). Sponsor-emitted bounty
    /// funding; the sole RSP-1 path that grows `task_markets_t.total_escrow`.
    EscrowLock      = 8,
    /// TB-5 RSP-3.0/3.1 system-emitted resolution (charter v2 § 4.1 + § 4.5).
    /// System-only: agent ingress rejected pre-queue; emit via
    /// `Sequencer::emit_system_tx`. Released refunds challenger bond + flips
    /// ChallengeCase.status; UpheldDeferred is a marker only (slash is
    /// RSP-3.2 / TB-6 territory).
    ChallengeResolve = 9,
}

/// TRACE_MATRIX FC2-Append + WP § 5.L4: stored LedgerEntry record (11 fields).
///
/// Distinct from `LedgerEntrySigningPayload`: this is the FULL stored record
/// (includes derivatives + signature); the signing payload is the subset that
/// the system keypair attests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntry {
    /// **K1**: assigned ONLY at commit (sequencer dual-counter design); rejected
    /// submissions never get a logical_t.
    pub logical_t: u64,                          //  1
    pub parent_state_root: Hash,                 //  2
    /// **K2 NEW**: parent_ledger_root before fold; bound in signed payload to
    /// prevent transplant attacks.
    pub parent_ledger_root: Hash,                //  3
    pub tx_kind: TxKind,                         //  4
    /// CAS handle (CO1.4) to canonical-serialized payload (DIV-5 5-param put).
    pub tx_payload_cid: Cid,                     //  5
    /// Resulting state_root post-transition (NOT mutated by L4 — accepted as
    /// returned by transition function per K3 boundary).
    pub resulting_state_root: Hash,              //  6
    /// Resulting ledger_root after fold. Derivative; NOT in signed digest.
    pub resulting_ledger_root: Hash,             //  7
    pub timestamp_logical: u64,                  //  8
    /// **D1 / Q10**: epoch bound in signed payload (Codex security wins).
    pub epoch: SystemEpoch,                      //  9
    /// **G1 NEW**: forward-compat extension map. Empty in v1; reserved for v4.x.
    /// Bound in signed payload (G1 cannot bypass signature).
    pub extensions: BTreeMap<String, Vec<u8>>,   // 10
    /// Detached system signature over `LedgerEntrySigningPayload.canonical_digest()`.
    pub system_signature: SystemSignature,       // 11
}

// ────────────────────────────────────────────────────────────────────────────
// § 1.1 LedgerEntrySigningPayload — the signed bytes (NEW per C3 / Q9)
// ────────────────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC2-Append C3: the bytes the system keypair actually signs.
///
/// **Excludes** (Q9 cycle prevention):
/// - `resulting_ledger_root` (derivative; including → cycle)
/// - `system_signature` (its own input)
///
/// **Includes** (9 non-derivative bound fields). Domain-separation prefix is
/// part of the digest to prevent cross-namespace collision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LedgerEntrySigningPayload {

exec
/bin/bash -lc "rg \"struct VerificationResult|verification_result\" -n src/runtime experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1,180p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
experiments/minif2f_v4/src/bin/evaluator.rs:1804:                                            let vr = turingosv4::runtime::verification_result::VerificationResult::from_lean_run(
experiments/minif2f_v4/src/bin/evaluator.rs:1812:                                            let vr_cid = match turingosv4::runtime::verification_result::write_to_cas(
experiments/minif2f_v4/src/bin/evaluator.rs:1824:                                            let pt = pt_partial.with_verification_result(vr_cid);
experiments/minif2f_v4/src/bin/evaluator.rs:2246:                                            let vr = turingosv4::runtime::verification_result::VerificationResult::from_lean_run(
experiments/minif2f_v4/src/bin/evaluator.rs:2254:                                            let vr_cid = match turingosv4::runtime::verification_result::write_to_cas(
experiments/minif2f_v4/src/bin/evaluator.rs:2266:                                            let pt = pt_partial.with_verification_result(vr_cid);
src/runtime/mod.rs:41:/// TRACE_MATRIX FC1-N14: TB-7.7 D4 — VerificationResult CAS object recording Lean oracle verdict (target_work_tx / verifier_agent / lean_exit_code / lean_*_hash / proof_file_hash / proof_artifact_cid / verified). Linked from ProposalTelemetry.verification_result_cid.
src/runtime/mod.rs:42:pub mod verification_result;
src/runtime/proposal_telemetry.rs:119:/// 9. **TB-7.7 D4**: `verification_result_cid` — optional CID to a
src/runtime/proposal_telemetry.rs:137:    /// object (`runtime::verification_result::VerificationResult`).
src/runtime/proposal_telemetry.rs:141:    pub verification_result_cid: Option<Cid>,
src/runtime/proposal_telemetry.rs:165:            verification_result_cid: None,
src/runtime/proposal_telemetry.rs:256:            verification_result_cid: None,
src/runtime/proposal_telemetry.rs:264:    /// called) keeps `verification_result_cid: None`.
src/runtime/proposal_telemetry.rs:265:    pub fn with_verification_result(mut self, cid: Cid) -> Self {
src/runtime/proposal_telemetry.rs:266:        self.verification_result_cid = Some(cid);
src/runtime/proposal_telemetry.rs:420:    /// 1 TB-7.7 D4 additive field (`verification_result_cid`).
src/runtime/proposal_telemetry.rs:422:    /// schema-additive `verification_result_cid: Option<Cid>` (default
src/runtime/proposal_telemetry.rs:426:    fn schema_validity_nine_fields_with_verification_result() {
src/runtime/proposal_telemetry.rs:433:            "ProposalTelemetry must have 9 fields (8 ruling-D5 + 1 TB-7.7 D4 verification_result_cid)"
src/runtime/proposal_telemetry.rs:443:        assert!(obj.contains_key("verification_result_cid"));
src/runtime/verification_result.rs:12://! the matching `ProposalTelemetry.verification_result_cid` field. On
src/runtime/verification_result.rs:31:const VERIFICATION_RESULT_SCHEMA_ID: &str = "turingosv4.verification_result.v1";
src/runtime/verification_result.rs:55:pub struct VerificationResult {
src/runtime/chain_derived_run_facts.rs:57:use crate::runtime::verification_result::read_from_cas as read_verification_result;
src/runtime/chain_derived_run_facts.rs:143:    ///   ProposalTelemetry.verification_result_cid → CAS
src/runtime/chain_derived_run_facts.rs:313:    // TB-7.7 D5: track verification_result_cid per accepted WorkTx so we
src/runtime/chain_derived_run_facts.rs:362:                        // TB-7.7 D5: capture verification_result_cid for
src/runtime/chain_derived_run_facts.rs:365:                            .insert(work.tx_id.clone(), tel.verification_result_cid);
src/runtime/chain_derived_run_facts.rs:459:    // one accepted L4 WorkTx whose ProposalTelemetry.verification_result_cid
src/runtime/chain_derived_run_facts.rs:472:            if let Ok(vr) = read_verification_result(&cas, vr_cid) {
src/runtime/chain_derived_run_facts.rs:671:    /// ProposalTelemetry.verification_result_cid, AND a matching

exec
/bin/bash -lc "sed -n '554,590p' src/bottom_white/ledger/transition_ledger.rs && sed -n '50,120p' src/runtime/verification_result.rs && sed -n '130,160p' src/runtime/verification_result.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
// § 2.5 Canonical serialization (bincode v2; STATE_TRANSITION_SPEC § 2.5)
// ────────────────────────────────────────────────────────────────────────────

/// `bincode::config` used for the canonical `LedgerEntry` wire format.
///
/// **Frozen choices** (per STATE_TRANSITION_SPEC § 2.5):
/// - **Big-endian** byte order (network order; deterministic across platforms).
/// - **Fixed-int encoding** (no varint; fixed-width for byte-stable round-trip).
/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
///   only ever encode `BTreeMap` (sorted by construction) so key order is lex.
/// - **No padding, no implicit alignment.**
fn bincode_canonical_config() -> impl bincode::config::Config {
    bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding()
}

/// Canonical encode any serde-Serialize value to bytes (CO1.7 wire format).
/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
/// needing byte-stable signatures over typed payloads.
pub fn canonical_encode<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalCodecError> {
    bincode::serde::encode_to_vec(value, bincode_canonical_config())
        .map_err(|e| CanonicalCodecError::Encode(e.to_string()))
}

/// Canonical decode the inverse of `canonical_encode`. Returns the value plus
/// the number of bytes consumed (entire input must be consumed for a clean decode).
pub fn canonical_decode<T: serde::de::DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, CanonicalCodecError> {
    let (value, consumed) =
        bincode::serde::decode_from_slice::<T, _>(bytes, bincode_canonical_config())
            .map_err(|e| CanonicalCodecError::Decode(e.to_string()))?;
    if consumed != bytes.len() {
        return Err(CanonicalCodecError::TrailingBytes {
            consumed,
            total: bytes.len(),
///
/// **Forbidden contents**: raw stdout/stderr, raw proof transcripts, any
/// chain-of-thought from the verifier. Hashes only. (Inherits TB-6
/// charter §4.2 "selective shielding" + ProposalTelemetry I91d guard.)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationResult {
    pub target_work_tx: TxId,
    pub verifier_agent: AgentId,
    pub lean_exit_code: i32,
    pub lean_stdout_hash: Hash,
    pub lean_stderr_hash: Hash,
    pub proof_file_hash: Hash,
    pub proof_artifact_cid: Cid,
    pub verified: bool,
}

impl VerificationResult {
    /// TRACE_MATRIX FC1-N14: convenience constructor for OMEGA-accept paths.
    /// Computes the verdict from the exit code (0 → verified, else → not).
    pub fn from_lean_run(
        target_work_tx: TxId,
        verifier_agent: AgentId,
        lean_exit_code: i32,
        proof_artifact_cid: Cid,
        proof_file_path: &str,
        proof_artifact_bytes: &[u8],
    ) -> Self {
        use sha2::{Digest, Sha256};
        let mut h_path = Sha256::new();
        h_path.update(proof_file_path.as_bytes());
        let proof_file_hash = Hash(h_path.finalize().into());
        let mut h_artifact = Sha256::new();
        h_artifact.update(proof_artifact_bytes);
        // (We don't store the artifact bytes here — they live in CAS via
        // proof_artifact_cid. The hash is recorded separately for forensic
        // cross-check; under content-addressing it should match the cid's
        // backend hash but we don't enforce.)
        let _redundant_check_for_audit = Hash(h_artifact.finalize().into());

        let lean_stdout_hash = Hash([0u8; 32]); // populated by caller if available
        let lean_stderr_hash = Hash([0u8; 32]);

        let verified = lean_exit_code == 0;

        Self {
            target_work_tx,
            verifier_agent,
            lean_exit_code,
            lean_stdout_hash,
            lean_stderr_hash,
            proof_file_hash,
            proof_artifact_cid,
            verified,
        }
    }
}

// ── Errors ──────────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: VerificationResult CAS error taxonomy.
#[derive(Debug)]
pub enum VerificationResultError {
    Cas(CasError),
    Codec(String),
}

impl std::fmt::Display for VerificationResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cas(e) => write!(f, "cas error: {e}"),
            Self::Codec(s) => write!(f, "codec error: {s}"),
    }
}

// ── CAS storage ─────────────────────────────────────────────────────────────

/// TRACE_MATRIX FC1-N14: canonical-encode the verification result + CAS put.
/// Idempotent (same record → same CID).
pub fn write_to_cas(
    cas: &mut CasStore,
    record: &VerificationResult,
    creator: &str,
    logical_t: u64,
) -> Result<Cid, VerificationResultError> {
    let bytes =
        canonical_encode(record).map_err(|e| VerificationResultError::Codec(e.to_string()))?;
    let cid = cas.put(
        &bytes,
        ObjectType::Generic,
        creator,
        logical_t,
        Some(VERIFICATION_RESULT_SCHEMA_ID.to_string()),
    )?;
    Ok(cid)
}

/// TRACE_MATRIX FC1-N14: CAS fetch + canonical-decode.
pub fn read_from_cas(
    cas: &CasStore,
    cid: &Cid,
) -> Result<VerificationResult, VerificationResultError> {
    let bytes = cas.get(cid)?;

exec
/bin/bash -lc "sed -n '160,190p' src/runtime/verification_result.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    let bytes = cas.get(cid)?;
    canonical_decode::<VerificationResult>(&bytes)
        .map_err(|e| VerificationResultError::Codec(e.to_string()))
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_cas() -> (TempDir, CasStore) {
        let dir = TempDir::new().expect("tempdir");
        let cas = CasStore::open(dir.path()).expect("open cas");
        (dir, cas)
    }

    fn fresh_record(verified: bool) -> VerificationResult {
        VerificationResult {
            target_work_tx: TxId("worktx-test-1".into()),
            verifier_agent: AgentId("Agent_0".into()),
            lean_exit_code: if verified { 0 } else { 1 },
            lean_stdout_hash: Hash([0u8; 32]),
            lean_stderr_hash: Hash([0u8; 32]),
            proof_file_hash: Hash([0xab; 32]),
            proof_artifact_cid: Cid([0xcd; 32]),
            verified,
        }
    }


exec
/bin/bash -lc "rg \"pub struct Git2Ledger|struct Git2Ledger|read_at|replay_chain|iter|entries\" -n src/bottom_white/ledger/transition_ledger.rs | sed -n '1,200p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
6://! append() + replay_chain_integrity() + replay_full_transition() + Git2LedgerWriter.
14://!   exposes `replay_chain_integrity` only (renamed for honesty).
145:        // Extensions: BTreeMap iterates in lex key order (deterministic);
192:// LedgerWriter trait (K4 reconciled to skeleton signature)
196:/// Production impl is `Git2LedgerWriter` (CO1.7.5+; refs/transitions/main commit chain).
197:/// Test/skeleton impl is `InMemoryLedgerWriter` below.
199:/// **K4**: signature `commit(&mut self) → Hash` (NOT `&self → NodeId`); `iter_from`
201:pub trait LedgerWriter: Send + Sync {
202:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError>;
203:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError>;
212:    /// enforces every `LedgerWriter` implementation declares this method. This
217:    ///   backend has no OID notion (e.g. InMemoryLedgerWriter); no panic risk.
222:pub enum LedgerWriterError {
228:impl std::fmt::Display for LedgerWriterError {
239:impl std::error::Error for LedgerWriterError {}
241:/// In-memory test/skeleton writer; Vec backing strict logical_t enforced at commit.
243:pub struct InMemoryLedgerWriter {
244:    entries: Vec<LedgerEntry>,
247:impl InMemoryLedgerWriter {
253:impl LedgerWriter for InMemoryLedgerWriter {
254:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
255:        let expected = (self.entries.len() as u64) + 1;
257:            return Err(LedgerWriterError::LogicalTGap {
263:        self.entries.push(entry.clone());
267:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
268:        if logical_t == 0 || logical_t > self.entries.len() as u64 {
269:            return Err(LedgerWriterError::NotFound { logical_t });
271:        Ok(self.entries[(logical_t - 1) as usize].clone())
275:        self.entries.len() as u64
409:    entries: &[LedgerEntry],
422:    for (i, entry) in entries.iter().enumerate() {
517:pub fn replay_chain_integrity(
520:    entries: &[LedgerEntry],
525:    for (i, entry) in entries.iter().enumerate() {
562:/// - **`BTreeMap` keys**: bincode iterates the map in serde-supplied order; we
572:/// Used by `Git2LedgerWriter` for commit-message bodies and by future callers
617:// § 5 Git2LedgerWriter — git2-rs commit chain on `refs/transitions/main`
629:///       canonical record — `read_at` decodes it directly).
638:/// **K3 (revised v1.2)**: this writer surfaces `commit_oid` for callers that
639:/// need it (CO1.7.5+ `head_t` wiring), but the `LedgerWriter::commit` trait
641:/// commit OID use [`Git2LedgerWriter::head_commit_oid`] post-commit.
642:pub struct Git2LedgerWriter {
646:    /// Number of entries committed = highest assigned `logical_t` (0 at genesis).
655:impl Git2LedgerWriter {
656:    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
659:    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
664:                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
674:                        LedgerWriterError::BackendCorruption(format!(
684:                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
700:    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
702:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
713:    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
715:            return Err(LedgerWriterError::NotFound { logical_t });
719:        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
723:                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
727:                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
733:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
736:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
740:                LedgerWriterError::BackendCorruption(format!(
746:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
751:impl LedgerWriter for Git2LedgerWriter {
759:    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
762:            return Err(LedgerWriterError::LogicalTGap {
770:            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
775:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
778:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
780:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
783:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
785:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
788:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
790:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
793:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
796:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;
801:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
806:                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
810:        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
821:            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;
828:    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
831:            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
916:    // 3. InMemoryWriter enforces logical_t monotonic
918:    fn in_memory_writer_enforces_logical_t() {
919:        let mut w = InMemoryLedgerWriter::new();
925:        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
930:    fn replay_chain_integrity_clean() {
935:            replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1.clone(), e2.clone(), e3.clone()])
947:        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
959:        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
969:        let err = replay_chain_integrity(Hash::ZERO, Hash::ZERO, &[e1, e2]).unwrap_err();
1070:    // 10–13. Git2LedgerWriter — git2-rs commit chain backend (§ 5)
1075:    fn fresh_git_writer() -> (TempDir, Git2LedgerWriter) {
1077:        let w = Git2LedgerWriter::open(tmp.path()).expect("open");
1083:    fn git2_writer_empty_chain() {
1084:        let (_tmp, w) = fresh_git_writer();
1089:    // 11. Append three entries; len + head_commit_oid advance per commit;
1090:    //     read_at recovers each entry byte-identically (canonical encode/decode round-trip).
1092:    fn git2_writer_append_and_read_back() {
1093:        let (_tmp, mut w) = fresh_git_writer();
1112:        // read_at returns each entry byte-identically.
1113:        assert_eq!(w.read_at(1).expect("read 1"), e1);
1114:        assert_eq!(w.read_at(2).expect("read 2"), e2);
1115:        assert_eq!(w.read_at(3).expect("read 3"), e3);
1120:    fn git2_writer_rejects_logical_t_gap() {
1121:        let (_tmp, mut w) = fresh_git_writer();
1129:        assert!(matches!(err, LedgerWriterError::LogicalTGap { expected: 2, got: 3 }));
1138:    fn git2_writer_reopen_recovers_chain() {
1144:            let mut w = Git2LedgerWriter::open(tmp.path()).expect("open");
1150:        let w2 = Git2LedgerWriter::open(tmp.path()).expect("reopen");
1153:        assert_eq!(w2.read_at(1).expect("read 1"), e1);
1154:        assert_eq!(w2.read_at(2).expect("read 2"), e2);
1157:        let mut w3 = Git2LedgerWriter::open(tmp.path()).expect("reopen 2");
1191:            read_set: [ReadKey("k.r".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
1192:            write_set: [WriteKey("k.w".into())].into_iter().collect::<std::collections::BTreeSet<_>>(),
1497:        // CO1.7.5 plan: submit N tx through Sequencer + collect entries from
1498:        // ledger_writer + replay_full_transition(...) → assert final state_root
1503:    // 14. canonical_encode/decode round-trip for LedgerEntry (foundation of read_at).

exec
/bin/bash -lc "sed -n '655,835p' src/bottom_white/ledger/transition_ledger.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
impl Git2LedgerWriter {
    /// Open or initialize a `Git2LedgerWriter` rooted at `repo_path`.
    /// Creates the underlying git repo if it doesn't exist; resolves the
    /// existing `refs/transitions/main` if present and seeds `head_oid` + `len`.
    pub fn open(repo_path: &Path) -> Result<Self, LedgerWriterError> {
        let repo_path = repo_path.to_path_buf();
        let repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(_) => Repository::init(&repo_path).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("repo init: {e}"))
            })?,
        };

        // Resolve refs/transitions/main if it exists.
        let (head_oid, len) = match repo.find_reference(TRANSITIONS_REF) {
            Ok(reference) => {
                let oid = reference
                    .target()
                    .ok_or_else(|| {
                        LedgerWriterError::BackendCorruption(format!(
                            "{TRANSITIONS_REF} has no direct target"
                        ))
                    })?;
                // Walk parents to count chain length.
                let mut n: u64 = 0;
                let mut cursor = Some(oid);
                while let Some(c) = cursor {
                    n += 1;
                    let commit = repo.find_commit(c).map_err(|e| {
                        LedgerWriterError::BackendCorruption(format!("walk parent: {e}"))
                    })?;
                    cursor = commit.parent(0).ok().map(|p| p.id());
                }
                (Some(oid), n)
            }
            Err(_) => (None, 0),
        };

        Ok(Self {
            repo_path,
            head_oid,
            len,
        })
    }

    fn open_repo(&self) -> Result<Repository, LedgerWriterError> {
        Repository::open(&self.repo_path)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("repo open: {e}")))
    }

    /// Commit OID of the most recent appended entry (None if chain is empty).
    /// CO1.7.5+ `head_t` wiring uses this to surface commit_sha alongside Hash.
    pub fn head_commit_oid(&self) -> Option<git2::Oid> {
        self.head_oid
    }

    /// Read raw canonical-encoded `LedgerEntry` bytes (the `entry_canonical`
    /// tree blob) for the entry at `logical_t`. `logical_t` is 1-indexed.
    fn read_canonical_bytes(&self, logical_t: u64) -> Result<Vec<u8>, LedgerWriterError> {
        if logical_t == 0 || logical_t > self.len {
            return Err(LedgerWriterError::NotFound { logical_t });
        }
        let repo = self.open_repo()?;
        // Walk back (len - logical_t) parents from head.
        let mut cursor = self.head_oid.ok_or(LedgerWriterError::NotFound { logical_t })?;
        let mut steps_back = self.len - logical_t;
        while steps_back > 0 {
            let commit = repo.find_commit(cursor).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("find_commit: {e}"))
            })?;
            cursor = commit
                .parent(0)
                .map_err(|e| LedgerWriterError::BackendCorruption(format!("parent: {e}")))?
                .id();
            steps_back -= 1;
        }
        let commit = repo
            .find_commit(cursor)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_commit: {e}")))?;
        let tree = commit
            .tree()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree: {e}")))?;
        let entry_obj = tree
            .get_name(TREE_BLOB_ENTRY_CANONICAL)
            .ok_or_else(|| {
                LedgerWriterError::BackendCorruption(format!(
                    "missing {TREE_BLOB_ENTRY_CANONICAL} blob at logical_t={logical_t}"
                ))
            })?;
        let blob = repo
            .find_blob(entry_obj.id())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_blob: {e}")))?;
        Ok(blob.content().to_vec())
    }
}

impl LedgerWriter for Git2LedgerWriter {
    /// CO1.7-extra D2: surface 40-char lowercase hex commit OID for sequencer
    /// post-commit head_t wiring. Maps existing `head_commit_oid()` accessor
    /// (returns Option<git2::Oid>) to canonical hex string.
    fn head_commit_oid_hex(&self) -> Option<String> {
        self.head_commit_oid().map(|oid| oid.to_string())
    }

    fn commit(&mut self, entry: &LedgerEntry) -> Result<Hash, LedgerWriterError> {
        let expected = self.len + 1;
        if entry.logical_t != expected {
            return Err(LedgerWriterError::LogicalTGap {
                expected,
                got: entry.logical_t,
            });
        }

        let repo = self.open_repo()?;
        let canonical = canonical_encode(entry).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_encode: {e}"))
        })?;

        let mut tb = repo
            .treebuilder(None)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("treebuilder: {e}")))?;
        let cid_blob = repo
            .blob(&entry.tx_payload_cid.0)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("cid blob: {e}")))?;
        tb.insert(TREE_BLOB_PAYLOAD_CID, cid_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert cid: {e}")))?;
        let sig_blob = repo
            .blob(entry.system_signature.as_bytes())
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("sig blob: {e}")))?;
        tb.insert(TREE_BLOB_SIGNATURE, sig_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert sig: {e}")))?;
        let entry_blob = repo
            .blob(&canonical)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("entry blob: {e}")))?;
        tb.insert(TREE_BLOB_ENTRY_CANONICAL, entry_blob, 0o100644)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree insert entry: {e}")))?;
        let tree_oid = tb
            .write()
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("tree write: {e}")))?;
        let tree = repo
            .find_tree(tree_oid)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("find_tree: {e}")))?;

        // Determinism: time = (logical_t, 0). NO wall clock.
        let time = git2::Time::new(entry.logical_t as i64, 0);
        let author = GitSignature::new("turingosv4 sequencer", "system@turingos", &time)
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("git sig: {e}")))?;
        let committer = author.clone();

        let parents: Vec<git2::Commit<'_>> = match self.head_oid {
            Some(oid) => vec![repo.find_commit(oid).map_err(|e| {
                LedgerWriterError::BackendCorruption(format!("parent commit: {e}"))
            })?],
            None => Vec::new(),
        };
        let parent_refs: Vec<&git2::Commit<'_>> = parents.iter().collect();
        let message = format!("transition logical_t={}\n", entry.logical_t);
        let new_oid = repo
            .commit(
                Some(TRANSITIONS_REF),
                &author,
                &committer,
                &message,
                &tree,
                &parent_refs,
            )
            .map_err(|e| LedgerWriterError::BackendCorruption(format!("commit: {e}")))?;

        self.head_oid = Some(new_oid);
        self.len += 1;
        Ok(entry.resulting_ledger_root)
    }

    fn read_at(&self, logical_t: u64) -> Result<LedgerEntry, LedgerWriterError> {
        let bytes = self.read_canonical_bytes(logical_t)?;
        canonical_decode::<LedgerEntry>(&bytes).map_err(|e| {
            LedgerWriterError::BackendCorruption(format!("canonical_decode at {logical_t}: {e}"))
        })
    }

    fn len(&self) -> u64 {

exec
/bin/bash -lc 'ls target/debug/deps/libturingosv4-*.rlib | tail -n 1' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
target/debug/deps/libturingosv4-dfec76756d97e682.rlib

exec
/bin/bash -lc "sed -n '1,120p' src/bottom_white/cas/schema.rs" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
//! CAS object schema per WP architecture § 5.L3.
//!
//! /// TRACE_MATRIX WP-arch-§5.L3: CAS object schema

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Content-addressed identifier — sha256 of payload bytes.
///
/// Distinct from git's SHA-1 OID (which is an internal storage detail of
/// the git2-rs backend). `Cid` is the v4-canonical identifier; spec § 1.2
/// `WorkTx.proposal_cid: Cid` references this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct Cid(pub [u8; 32]);

impl Cid {
    /// Compute Cid from content bytes.
    pub fn from_content(content: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(content);
        Self(h.finalize().into())
    }

    /// Hex-encoded representation (lowercase; 64 chars).
    pub fn hex(&self) -> String {
        let mut s = String::with_capacity(64);
        for b in self.0 {
            s.push_str(&format!("{:02x}", b));
        }
        s
    }
}

impl std::fmt::Display for Cid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cid:{}", self.hex())
    }
}

/// Type tag for CAS objects (replaces inline string-typed kind).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectType {
    /// Agent's work_tx proposal payload (Lean proof, code patch, etc.).
    ProposalPayload,
    /// Challenger's counterexample for a slashed claim.
    CounterexamplePayload,
    /// Predicate bytecode (Lean tactic, WASM module, Rust source bytes).
    PredicateBytecode,
    /// Tool bytecode.
    ToolBytecode,
    /// Constitution diff (for amendment proposals).
    AmendmentDiff,
    /// Reversibility plan attached to a meta_tx.
    ReversibilityPlan,
    /// Generic / unclassified blob.
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CasObjectMetadata {
    /// Content-addressed identifier (sha256 of content).
    pub cid: Cid,
    /// Backend-specific OID (git sha-1 for git2-rs backend); informational only.
    /// Different backends may have different OID schemes; Cid is canonical.
    pub backend_oid_hex: String,
    pub object_type: ObjectType,
    /// Submitter / author. Use "system" for runtime-emitted objects.
    pub creator: String,
    /// Logical time at insertion (assigned by sequencer; not wall clock).
    pub created_at_logical_t: u64,
    /// Optional schema identifier (JSON Schema URI, type tag, etc.).
    pub schema_id: Option<String>,
    /// Size of content in bytes (informational; not part of canonical hash).
    pub size_bytes: u64,
}

impl CasObjectMetadata {
    /// Canonical hash of metadata for Merkle tree inclusion.
    pub fn canonical_hash(&self) -> [u8; 32] {
        let mut h = Sha256::new();
        h.update(self.cid.0);
        h.update(self.backend_oid_hex.as_bytes());
        h.update(serde_json::to_vec(&self.object_type).expect("object_type serialize"));
        h.update(self.creator.as_bytes());
        h.update(self.created_at_logical_t.to_be_bytes());
        if let Some(s) = &self.schema_id {
            h.update(b"\x01");
            h.update(s.as_bytes());
        } else {
            h.update(b"\x00");
        }
        h.update(self.size_bytes.to_be_bytes());
        h.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_from_empty_content() {
        let cid = Cid::from_content(b"");
        // SHA-256 of empty input = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
        assert_eq!(
            cid.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn cid_deterministic() {
        let cid_a = Cid::from_content(b"hello");
        let cid_b = Cid::from_content(b"hello");
        assert_eq!(cid_a, cid_b);
    }

    #[test]
    fn cid_differs_on_content() {
        let cid_a = Cid::from_content(b"hello");

exec
/bin/bash -lc 'rg "enum CanonicalCodecError|impl std::error::Error for CanonicalCodecError|impl std::fmt::Display for CanonicalCodecError" -n src/bottom_white/ledger/transition_ledger.rs' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
597:pub enum CanonicalCodecError {
603:impl std::fmt::Display for CanonicalCodecError {
614:impl std::error::Error for CanonicalCodecError {}

exec
/bin/bash -lc "sed -n '120,155p' src/runtime/proposal_telemetry.rs && rg \"impl std::error::Error for ProposalTelemetryError|impl std::error::Error for VerificationResultError\" -n src/runtime" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
///    `VerificationResult` CAS object recording the Lean oracle's
///    verdict (exit code + verified flag + proof artifact hash).
///    `None` for proposals not yet Lean-verified (append-branch
///    intermediate steps); `Some(cid)` for OMEGA-accept proposals
///    where the evaluator has run Lean and recorded the verdict.
///    Replay readers use this to compute `chain_oracle_verified`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposalTelemetry {
    pub agent_id: AgentId,
    pub prompt_context_hash: Hash,
    pub proposal_artifact_cid: Cid,
    pub candidate_tactic: String,
    pub token_counts: TokenCounts,
    pub tool_calls: Vec<ToolCallRecord>,
    pub branch_id: String,
    pub parent_tx: Option<TxId>,
    /// TB-7.7 D4: optional CID of the matching `VerificationResult` CAS
    /// object (`runtime::verification_result::VerificationResult`).
    /// Schema-additive; `None` preserves backward compat with pre-TB-7.7
    /// telemetry.
    #[serde(default)]
    pub verification_result_cid: Option<Cid>,
}

impl ProposalTelemetry {
    /// TRACE_MATRIX FC1-N14: convenience constructor for the common case where
    /// the proposal has no parent (root proposal). Used by Atom 2 evaluator
    /// hooks that don't yet track branch lineage.
    pub fn new_root(
        agent_id: AgentId,
        prompt_context_hash: Hash,
        proposal_artifact_cid: Cid,
        candidate_tactic: String,
        token_counts: TokenCounts,
        branch_id: String,
    ) -> Self {
src/runtime/verification_result.rs:125:impl std::error::Error for VerificationResultError {}
src/runtime/proposal_telemetry.rs:291:impl std::error::Error for ProposalTelemetryError {}

codex
For the CID walk I’m using the repository’s own library surface: `Git2LedgerWriter::read_at` to recover the accepted L4 `WorkTx`, then `CasStore::get` and the production `read_from_cas` decoders for `ProposalTelemetry` and `VerificationResult`.
exec
/bin/bash -lc 'set -euo pipefail
RLIB=$(ls target/debug/deps/libturingosv4-*.rlib | tail -n 1)
rustc --edition=2021 -L dependency=target/debug/deps --extern turingosv4="$RLIB" -o /tmp/tb7r_cid_probe <(cat <<'"'RS'
use std::path::Path;

use turingosv4::bottom_white::cas::store::CasStore;
use turingosv4::bottom_white::ledger::transition_ledger::{canonical_decode, Git2LedgerWriter, LedgerWriter};
use turingosv4::runtime::{proposal_telemetry, verification_result};
use turingosv4::state::typed_tx::TypedTx;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() "'!= 3 {
        return Err("usage: tb7r_cid_probe <runtime_repo> <cas>".into());
    }
    let ledger = Git2LedgerWriter::open(Path::new(&args[1]))?;
    let cas = CasStore::open(Path::new(&args[2]))?;
    for logical_t in 1..=ledger.len() {
        let entry = ledger.read_at(logical_t)?;
        let payload_bytes = cas.get(&entry.tx_payload_cid)?;
        let typed_tx: TypedTx = canonical_decode(&payload_bytes)?;
        if let TypedTx::Work(work) = typed_tx {
            if work.proposal_cid.0 == [0u8; 32] {
                continue;
            }
            let telemetry_bytes = cas.get(&work.proposal_cid)?;
            let telemetry = proposal_telemetry::read_from_cas(&cas, &work.proposal_cid)?;
            let vr_cid = telemetry
                .verification_result_cid
                .ok_or("telemetry missing verification_result_cid")?;
            let vr_bytes = cas.get(&vr_cid)?;
            let vr = verification_result::read_from_cas(&cas, &vr_cid)?;
            let proof_bytes = cas.get(&vr.proof_artifact_cid)?;
            println!("logical_t={logical_t}");
            println!("tx_id={}", work.tx_id.0);
            println!("entry_payload_cid={} bytes={}", entry.tx_payload_cid, payload_bytes.len());
            println!("work_proposal_cid={} bytes={}", work.proposal_cid, telemetry_bytes.len());
            println!("telemetry_candidate_tactic={}", telemetry.candidate_tactic);
            println!("telemetry_branch_id={}", telemetry.branch_id);
            println!("telemetry_verification_result_cid={} bytes={}", vr_cid, vr_bytes.len());
            println!("verification_verified={} lean_exit_code={}", vr.verified, vr.lean_exit_code);
            println!("verification_proof_artifact_cid={} bytes={}", vr.proof_artifact_cid, proof_bytes.len());
            println!("proof_preview={}", String::from_utf8_lossy(&proof_bytes).replace('"'\\n', \" ⏎ \"));
            return Ok(());
        }
    }
    Err(\"no accepted WorkTx with non-zero proposal_cid found\".into())
}
RS
)
/tmp/tb7r_cid_probe /tmp/tb7r_r2_uyTXAx/runtime_repo /tmp/tb7r_r2_uyTXAx/cas" in /home/zephryj/projects/turingosv4
 succeeded in 931ms:
logical_t=3
tx_id=worktx-task-n1_mathd_algebra_171_1777694645204-omega-pertactic-1
entry_payload_cid=cid:d2fa2ff218d8f6e8d7ce814cf6ff417f6edda965b9501cd1506720600cff91a7 bytes=361
work_proposal_cid=cid:bbade597ce1cdec4a71cf15042dd88a953a93ff98d0096e7d57178eaa3657e51 bytes=184
telemetry_candidate_tactic=step_complete
telemetry_branch_id=Agent_0.b1
telemetry_verification_result_cid=cid:ea8f23000619b1efeaa9a16cc0723874b1091bfe2ed4dad43469c8b1d25e7287 bytes=220
verification_verified=true lean_exit_code=0
verification_proof_artifact_cid=cid:c31a0bc9b117842e3df8081fbfd0140aec26e2752ea7e92882d76f365865ae83 bytes=84
proof_preview=calc ⏎   f 1 = 5 * 1 + 4 := by rw [h₀] ⏎   _ = 5 + 4 := by ring ⏎   _ = 9 := by norm_num

exec
/bin/bash -lc "nl -ba src/bottom_white/cas/store.rs | sed -n '190,235p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   190	        // If already in index, idempotent: just return Cid (content addressing
   191	        // guarantees same content → same Cid → already present)
   192	        if self.index.contains_key(&cid) {
   193	            return Ok(cid);
   194	        }
   195	
   196	        let metadata = CasObjectMetadata {
   197	            cid,
   198	            backend_oid_hex: git_oid.to_string(),
   199	            object_type,
   200	            creator: creator.to_string(),
   201	            created_at_logical_t,
   202	            schema_id,
   203	            size_bytes: content.len() as u64,
   204	        };
   205	        // CO1.4-extra: durably append BEFORE inserting into in-memory index
   206	        // (so a crash mid-write leaves the runtime in a consistent state —
   207	        // either the entry is durably recorded AND in-memory, or neither).
   208	        append_to_sidecar(&self.repo_path, &metadata)?;
   209	        self.index.insert(cid, metadata);
   210	        Ok(cid)
   211	    }
   212	
   213	    /// Retrieve content by Cid. Verifies content sha256 matches Cid (corruption check).
   214	    pub fn get(&self, cid: &Cid) -> Result<Vec<u8>, CasError> {
   215	        let metadata = self
   216	            .index
   217	            .get(cid)
   218	            .ok_or(CasError::CidNotFound(*cid))?;
   219	        let repo = self.open_repo()?;
   220	        let git_oid = git2::Oid::from_str(&metadata.backend_oid_hex)
   221	            .map_err(CasError::Git2)?;
   222	        let blob = repo.find_blob(git_oid)?;
   223	        let content = blob.content().to_vec();
   224	
   225	        // Verify content sha256 matches Cid (defense against corruption).
   226	        let mut h = Sha256::new();
   227	        h.update(&content);
   228	        let computed = Cid(h.finalize().into());
   229	        if &computed != cid {
   230	            return Err(CasError::CidMismatch {
   231	                expected: *cid,
   232	                computed,
   233	            });
   234	        }
   235	

exec
/bin/bash -lc "nl -ba src/runtime/proposal_telemetry.rs | sed -n '330,352p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   330	/// `verify_chaintape` extension to retrieve and validate
   331	/// `WorkTx.proposal_cid` references during replay.
   332	pub fn read_from_cas(
   333	    cas: &CasStore,
   334	    cid: &Cid,
   335	) -> Result<ProposalTelemetry, ProposalTelemetryError> {
   336	    let bytes = cas.get(cid)?;
   337	    canonical_decode::<ProposalTelemetry>(&bytes)
   338	        .map_err(|e| ProposalTelemetryError::Codec(e.to_string()))
   339	}
   340	
   341	/// TRACE_MATRIX FC1-N14: convenience — open a CAS at `cas_path` and read the
   342	/// telemetry record at `cid`. Used by `verify_chaintape` CLI which needs to
   343	/// resolve `WorkTx.proposal_cid` references with only the on-disk paths.
   344	pub fn read_from_cas_path(
   345	    cas_path: &Path,
   346	    cid: &Cid,
   347	) -> Result<ProposalTelemetry, ProposalTelemetryError> {
   348	    let cas = CasStore::open(cas_path)?;
   349	    read_from_cas(&cas, cid)
   350	}
   351	
   352	// ── Tests ───────────────────────────────────────────────────────────────────

exec
/bin/bash -lc "nl -ba src/runtime/verification_result.rs | sed -n '144,164p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   144	        canonical_encode(record).map_err(|e| VerificationResultError::Codec(e.to_string()))?;
   145	    let cid = cas.put(
   146	        &bytes,
   147	        ObjectType::Generic,
   148	        creator,
   149	        logical_t,
   150	        Some(VERIFICATION_RESULT_SCHEMA_ID.to_string()),
   151	    )?;
   152	    Ok(cid)
   153	}
   154	
   155	/// TRACE_MATRIX FC1-N14: CAS fetch + canonical-decode.
   156	pub fn read_from_cas(
   157	    cas: &CasStore,
   158	    cid: &Cid,
   159	) -> Result<VerificationResult, VerificationResultError> {
   160	    let bytes = cas.get(cid)?;
   161	    canonical_decode::<VerificationResult>(&bytes)
   162	        .map_err(|e| VerificationResultError::Codec(e.to_string()))
   163	}
   164	

exec
/bin/bash -lc "nl -ba src/runtime/verify.rs | sed -n '416,426p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   416	                            agent_signatures_verified = false;
   417	                        }
   418	                    }
   419	                }
   420	                // Gate 5 — verify proposal_cid resolves to a ProposalTelemetry.
   421	                // Skip if proposal_cid is the zero-CID (legacy synthetic seed).
   422	                if work.proposal_cid.0 != [0u8; 32] {
   423	                    if read_telemetry(cas_store, &work.proposal_cid).is_err() {
   424	                        proposal_telemetry_cas_retrievable = false;
   425	                    }
   426	                }

codex
The sampled solved run also has a fully resolvable WorkTx chain: ledger entry payload, `WorkTx.proposal_cid`, telemetry, verification result, and proof artifact all resolved with nonzero payload sizes. I’m now doing a quick all-10 replay sweep because RQ2 asks whether an arbitrary run can be reproduced from the committed bundle.
exec
/bin/bash -lc "set -euo pipefail
ROOT=/home/zephryj/projects/turingosv4
mapfile -t RUNS < <(find handover/evidence/tb_7r_smoke_2026-05-02 -type d -name runtime_repo | sed 's#/runtime_repo"'$##'"' | sort)
for RUN in \""'${RUNS[@]}"; do
  WORK=$(mktemp -d /tmp/tb7r_all_XXXXXX)
  mkdir -p "$WORK/runtime_repo" "$WORK/cas"
  cp "$ROOT/$RUN"/runtime_repo/*.json "$ROOT/$RUN"/runtime_repo/*.jsonl "$WORK/runtime_repo/"
  cp "$ROOT/$RUN"/cas/.turingos_cas_index.jsonl "$WORK/cas/"
  tar -xzf "$ROOT/$RUN/runtime_repo.dotgit.tar.gz" -C "$WORK/runtime_repo"
  tar -xzf "$ROOT/$RUN/cas.dotgit.tar.gz" -C "$WORK/cas"
  "$ROOT/target/debug/verify_chaintape" --repo "$WORK/runtime_repo" --cas "$WORK/cas" --out "$WORK/r.json" >/dev/null
  jq -S '"'del(.run_id,.epoch)' \""'$ROOT/$RUN/replay_report.json" > "$WORK/orig.norm.json"
  jq -S '"'del(.run_id,.epoch)' \""'$WORK/r.json" > "$WORK/repro.norm.json"
  cmp -s "$WORK/orig.norm.json" "$WORK/repro.norm.json"
  BOOL_FALSE=$(jq '"'[paths(scalars) as "'$p | select(getpath($p)|type=="boolean" and getpath($p)==false)] | length'"' \""'$WORK/r.json")
  printf '"'%s replay=ok bool_false=%s l4=%s l4e=%s\\n' \""'$RUN" "$BOOL_FALSE" "$(jq -r .l4_entries "$WORK/r.json")" "$(jq -r .l4e_entries "$WORK/r.json")"
done
printf '"'runs_checked=%s\\n' \""'${#RUNS[@]}"' in /home/zephryj/projects/turingosv4
 exited 5 in 196ms:
jq: error (at /tmp/tb7r_all_wzWc8j/r.json:20): Cannot index boolean with string "ledger_root_verified"

exec
/bin/bash -lc "set -euo pipefail
ROOT=/home/zephryj/projects/turingosv4
mapfile -t RUNS < <(find handover/evidence/tb_7r_smoke_2026-05-02 -type d -name runtime_repo | sed 's#/runtime_repo"'$##'"' | sort)
for RUN in \""'${RUNS[@]}"; do
  WORK=$(mktemp -d /tmp/tb7r_all_XXXXXX)
  mkdir -p "$WORK/runtime_repo" "$WORK/cas"
  cp "$ROOT/$RUN"/runtime_repo/*.json "$ROOT/$RUN"/runtime_repo/*.jsonl "$WORK/runtime_repo/"
  cp "$ROOT/$RUN"/cas/.turingos_cas_index.jsonl "$WORK/cas/"
  tar -xzf "$ROOT/$RUN/runtime_repo.dotgit.tar.gz" -C "$WORK/runtime_repo"
  tar -xzf "$ROOT/$RUN/cas.dotgit.tar.gz" -C "$WORK/cas"
  "$ROOT/target/debug/verify_chaintape" --repo "$WORK/runtime_repo" --cas "$WORK/cas" --out "$WORK/r.json" >/dev/null
  jq -S '"'del(.run_id,.epoch)' \""'$ROOT/$RUN/replay_report.json" > "$WORK/orig.norm.json"
  jq -S '"'del(.run_id,.epoch)' \""'$WORK/r.json" > "$WORK/repro.norm.json"
  cmp -s "$WORK/orig.norm.json" "$WORK/repro.norm.json"
  BOOL_FALSE=$(jq '"'[paths(scalars) as "'$p | select(((getpath($p)|type)=="boolean") and (getpath($p)==false))] | length'"' \""'$WORK/r.json")
  printf '"'%s replay=ok bool_false=%s l4=%s l4e=%s\\n' \""'$RUN" "$BOOL_FALSE" "$(jq -r .l4_entries "$WORK/r.json")" "$(jq -r .l4e_entries "$WORK/r.json")"
done
printf '"'runs_checked=%s\\n' \""'${#RUNS[@]}"' in /home/zephryj/projects/turingosv4
 succeeded in 2855ms:
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_1_mathd_algebra_171 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_2_mathd_algebra_107 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_3_mathd_algebra_359 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9 replay=ok bool_false=0 l4=2 l4e=2
handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_5_mathd_numbertheory_5 replay=ok bool_false=0 l4=2 l4e=2
handover/evidence/tb_7r_smoke_2026-05-02/full_n5_mathd_algebra_171 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_1_mathd_algebra_171 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_2_mathd_algebra_107 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/half_3_problems_n1/run_3_mathd_algebra_359 replay=ok bool_false=0 l4=3 l4e=3
handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171 replay=ok bool_false=0 l4=3 l4e=3
runs_checked=10

exec
/bin/bash -lc "nl -ba handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl | sed -n '8,11p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     8	{"cid":[195,26,11,201,177,23,132,46,61,248,8,31,191,208,20,10,236,38,226,117,46,167,233,40,130,215,111,54,88,101,174,131],"backend_oid_hex":"7a66b55285ce8ca22f6b44656b1e8c4ed60b4f1a","object_type":"ProposalPayload","creator":"tb7-atom3-omega-pertactic-payload","created_at_logical_t":2,"schema_id":"turingosv4.proposal_payload.v1","size_bytes":84}
     9	{"cid":[234,143,35,0,6,25,177,239,234,169,161,108,192,114,56,116,177,9,27,254,46,212,218,212,52,105,200,177,210,94,114,135],"backend_oid_hex":"4c49ac7701ce9cb92667d61c0cbe6f6f888d49cd","object_type":"Generic","creator":"tb7-atom3-omega-pertactic-vr","created_at_logical_t":2,"schema_id":"turingosv4.verification_result.v1","size_bytes":220}
    10	{"cid":[187,173,229,151,206,28,222,196,167,28,241,80,66,221,136,169,83,169,63,249,141,0,150,231,213,113,120,234,163,101,126,81],"backend_oid_hex":"1c2d6cce6c20c7623949a89d3a96f2912f832f9c","object_type":"Generic","creator":"tb7-atom3-omega-pertactic","created_at_logical_t":2,"schema_id":"turingosv4.proposal_telemetry.v1","size_bytes":184}
    11	{"cid":[210,250,47,242,24,216,246,232,215,206,129,76,246,255,65,127,110,221,169,101,185,80,28,209,80,103,32,96,12,255,145,167],"backend_oid_hex":"467d358f52191ec017b87eaf6c55c494b208a298","object_type":"ProposalPayload","creator":"sequencer-epoch-1","created_at_logical_t":3,"schema_id":"TypedTx.v1","size_bytes":361}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json | sed -n '1,24p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "l4_entries": 3,
     3	  "l4e_entries": 3,
     4	  "ledger_root_verified": true,
     5	  "system_signatures_verified": true,
     6	  "state_reconstructed": true,
     7	  "economic_state_reconstructed": true,
     8	  "cas_payloads_retrievable": true,
     9	  "agent_signatures_verified": true,
    10	  "proposal_telemetry_cas_retrievable": true,
    11	  "run_id": "tb7r-smoke-1",
    12	  "epoch": 1,
    13	  "detail": {
    14	    "final_state_root_hex": "e5199d29e268c057f197ce8492e02429ab7af0046cda510cc08bf55efc4f7647",
    15	    "final_ledger_root_hex": "68bd2fe6a0b9c394151fa217da3d73087de75d4f894db208ac7376243661fb69",
    16	    "head_commit_oid_hex": "d4d704beb5ab16782425f75194cae07d83842f5c",
    17	    "l4e_last_hash_hex": "d3006f954373cefa3dfc29edb8880b91f787b572d7947e5ee00cb9a149e7973f",
    18	    "replay_failure": null,
    19	    "initial_q_state_loaded_from_disk": true
    20	  }
    21	}

exec
/bin/bash -lc "nl -ba handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json | sed -n '1,24p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
     1	{
     2	  "l4_entries": 2,
     3	  "l4e_entries": 2,
     4	  "ledger_root_verified": true,
     5	  "system_signatures_verified": true,
     6	  "state_reconstructed": true,
     7	  "economic_state_reconstructed": true,
     8	  "cas_payloads_retrievable": true,
     9	  "agent_signatures_verified": true,
    10	  "proposal_telemetry_cas_retrievable": true,
    11	  "run_id": "tb7r-smoke-full5-4",
    12	  "epoch": 1,
    13	  "detail": {
    14	    "final_state_root_hex": "97caed87cc19241835d1637dc68f7e304277c5128f334dd35d0c049450266d02",
    15	    "final_ledger_root_hex": "4e8a88526131bc48bb9cc2c98cf2a00df8599b1824f7e1e8f6d0cdf453172ca5",
    16	    "head_commit_oid_hex": "57e7f532ff8fab6ab13a221d649554f45cc15709",
    17	    "l4e_last_hash_hex": "d65bea40335e9df8e3bd63fbf962f9ff6a2c44be27d584862e9eaaa70e7d01d0",
    18	    "replay_failure": null,
    19	    "initial_q_state_loaded_from_disk": true
    20	  }
    21	}

exec
/bin/bash -lc 'ls -l handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.dotgit.tar.gz handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.dotgit.tar.gz handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz' in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
-rw-r--r-- 1 zephryj zephryj 10678 May  2 07:13 handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/cas.dotgit.tar.gz
-rw-r--r-- 1 zephryj zephryj 11445 May  2 07:13 handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/runtime_repo.dotgit.tar.gz
-rw-r--r-- 1 zephryj zephryj 11827 May  2 07:13 handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas.dotgit.tar.gz
-rw-r--r-- 1 zephryj zephryj 12485 May  2 07:13 handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/runtime_repo.dotgit.tar.gz

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/cost_aggregator.rs | sed -n '45,65p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
    45	    /// Record one LLM call's API-reported token usage.
    46	    /// Call this AFTER `client.generate(...)` returns Ok — both for winning
    47	    /// proposals and for ones that will be rejected/vetoed/parse-failed
    48	    /// downstream. The call already happened; the tokens already cost.
    49	    pub fn record_llm_call(&mut self, prompt_tokens: u32, completion_tokens: u32) {
    50	        self.prompt_tokens += prompt_tokens as u64;
    51	        self.completion_tokens += completion_tokens as u64;
    52	    }
    53	
    54	    /// Record bytes of agent-observable tool output (search hits, rejection
    55	    /// error message preserved for next-prompt error feedback, etc.).
    56	    /// chars/4 heuristic per plan B2 open Q2 default.
    57	    pub fn record_tool_stdout(&mut self, stdout: &str) {
    58	        let approx = (stdout.chars().count() as u64 + 3) / 4;
    59	        self.tool_tokens += approx;
    60	    }
    61	
    62	    /// Mark one proposal attempt. `accepted = true` for the verified-success
    63	    /// branch (typically called once per run on OMEGA accept); all other
    64	    /// proposals (parse fails, vetoed appends, rejected OMEGAs, step rejects)
    65	    /// pass `false` so they accrete failed_branch_count.

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '1328,1352p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  1328	        // when snap.tape is genuinely empty), so every proposal is
  1329	        // re-derived from scratch every tx — ERR=0 + time/token
  1330	        // inflation are the H detection mechanisms. Internal
  1331	        // verification paths (tape+payload Lean re-verify) below
  1332	        // are NOT gated on is_amnesia; that's verifier mechanics,
  1333	        // not agent memory.
  1334	        let chain = if minif2f_v4::experiment_mode::is_amnesia(mode) || snap.tape.is_empty() {
  1335	            problem_statement.to_string()
  1336	        } else {
  1337	            let nodes: Vec<String> = snap.tape.time_arrow().iter()
  1338	                .filter_map(|id| snap.tape.get(id))
  1339	                .map(|n| format!("[{}] {}: {}", n.id, n.author, n.payload))
  1340	                .collect();
  1341	            format!("{}\n\n=== Proof Chain ===\n{}", problem_statement, nodes.join("\n"))
  1342	        };
  1343	
  1344	        let errors = bus.recent_rejections(agent_id, 3);
  1345	        // Art. II.2.1: per-agent skill specialization + Librarian learned memory.
  1346	        // C1c: route skill index through experiment_mode helper so Homogeneous
  1347	        // mode pins every agent_idx to 0 (Paper-1 era A condition; H4 detection).
  1348	        let skill_idx = minif2f_v4::experiment_mode::skill_index_for_agent(
  1349	            mode, agent_idx, agent_skills.len(),
  1350	        );
  1351	        let base_skill = agent_skills.get(skill_idx).unwrap_or(&"");
  1352	        // C1d Panopticon: in cognitive-isolation-breach mode, the focal

exec
/bin/bash -lc "nl -ba src/bus.rs | sed -n '568,582p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
   568	            RejectionScope::Global => {
   569	                // Flatten all authors' recent; keep most recent `max` across swarm.
   570	                let mut all: Vec<&String> = self.graveyard.values().flatten().collect();
   571	                // Heuristic: assume push-order ~= time-order; take last `max` global entries.
   572	                let start = all.len().saturating_sub(max);
   573	                all.drain(..start);
   574	                all.into_iter().cloned().collect()
   575	            }
   576	            RejectionScope::TopKClasses(k) => {
   577	                // C-022 shield: broadcast abstracted CLASSES with COUNTS, not raw strings.
   578	                // Expects reason strings to already be class labels (see error_abstraction).
   579	                let mut counts: HashMap<String, u32> = HashMap::new();
   580	                for v in self.graveyard.values() {
   581	                    for r in v {
   582	                        *counts.entry(r.clone()).or_insert(0) += 1;

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '2128,2136p;2218,2228p;2400,2436p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2128	                                let mut ph = std::collections::hash_map::DefaultHasher::new();
  2129	                                tactic.hash(&mut ph);
  2130	                                proposal_hashes.insert(ph.finish());
  2131	                                proposal_count += 1;
  2132	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  2133	                                    .filter_map(|id| bus.kernel.tape.get(id))
  2134	                                    .map(|n| n.payload.clone())
  2135	                                    .collect::<Vec<_>>()
  2136	                                    .join("\n");
  2218	                                                &mut cas_store,
  2219	                                                &run_id,
  2220	                                                agent_id,
  2221	                                                proposal_count as u64,
  2222	                                                tactic.as_bytes(),
  2223	                                                "step_complete",
  2224	                                                turingosv4::runtime::proposal_telemetry::TokenCounts {
  2225	                                                    prompt_tokens: response.prompt_tokens as u64,
  2226	                                                    completion_tokens: response.completion_tokens as u64,
  2227	                                                    tool_tokens: 0,
  2228	                                                },
  2400	                                                        false,
  2401	                                                        proposal_hashes.len() as u64,
  2402	                                                        proposal_count,
  2403	                                                        verifier_wait_ms,
  2404	                                                        budget_regime, budget_max_tx_base, &run_id);
  2405	                                    }
  2406	                                    PartialVerdict::PartialOk => {
  2407	                                        let parent = bus.kernel.tape.time_arrow().last().cloned();
  2408	                                        // shadow_only: PartialOk is intermediate progress, not OMEGA
  2409	                                        // accept. The authoritative routing for intermediate
  2410	                                        // progress is the append-branch routing at evaluator.rs
  2411	                                        // line ~1283 (Atom 2). This call writes only to kernel.tape
  2412	                                        // for next-iteration prompt context. Per TB-7 §4.0 option
  2413	                                        // (3) + §6 #31; will be removed when kernel.tape is
  2414	                                        // L4-derived.
  2415	                                        match bus.append_oracle_accepted(
  2416	                                            agent_id, tactic, parent.as_deref(),
  2417	                                        ) {
  2418	                                            Ok(BusResult::Appended { node_id }) => {
  2419	                                                *tool_dist.entry("step_partial_ok".into()).or_insert(0) += 1;
  2420	                                                info!("[tx {}] {} step+{} partial OK (depth={})",
  2421	                                                      tx, agent_id, node_id,
  2422	                                                      bus.kernel.tape.time_arrow().len());
  2423	                                            }
  2424	                                            Ok(BusResult::Vetoed { reason }) => {
  2425	                                                warn!("[tx {}] step partial OK but bus vetoed: {}", tx, reason);
  2426	                                            }
  2427	                                            _ => {}
  2428	                                        }
  2429	                                    }
  2430	                                    PartialVerdict::Reject(reason) => {
  2431	                                        let class = classify_lean_error(&reason);
  2432	                                        bus.record_rejection(agent_id, class.label());
  2433	                                        // PPUT-CCL B2: step rejection reason flows into next prompt.
  2434	                                        acc.record_tool_stdout(&reason);
  2435	                                        *tool_dist.entry("step_reject".into()).or_insert(0) += 1;
  2436	                                        let preview = reason.chars().take(200).collect::<String>();

exec
/bin/bash -lc "nl -ba experiments/minif2f_v4/src/bin/evaluator.rs | sed -n '2132,2154p'" in /home/zephryj/projects/turingosv4
 succeeded in 0ms:
  2132	                                let tape_chain: String = bus.kernel.tape.time_arrow().iter()
  2133	                                    .filter_map(|id| bus.kernel.tape.get(id))
  2134	                                    .map(|n| n.payload.clone())
  2135	                                    .collect::<Vec<_>>()
  2136	                                    .join("\n");
  2137	                                let prefix = if tape_chain.is_empty() {
  2138	                                    tactic.clone()
  2139	                                } else {
  2140	                                    format!("{}\n{}", tape_chain, tactic)
  2141	                                };
  2142	                                let oracle = Lean4Oracle::new(
  2143	                                    problem_statement.to_string(),
  2144	                                    theorem_name.to_string(),
  2145	                                    lean_path.to_string(),
  2146	                                );
  2147	                                // A4: bracket the Lean partial-verify call.
  2148	                                let v_t0 = Instant::now();
  2149	                                let verdict = oracle.verify_partial(&prefix);
  2150	                                let v_partial_elapsed = v_t0.elapsed().as_millis() as u64;
  2151	                                verifier_wait_ms += v_partial_elapsed;
  2152	                                // A8e fix F4 (Codex#3): FC1-N12 emit for the
  2153	                                // step-verify path. Closes the swarm-side gap
  2154	                                // round-1 audit flagged.

  ✓ Inspect updated evidence docs and line references
  ✓ Round-trip two evidence bundles with verify_chaintape
  ✓ Resolve one WorkTx CID chain from extracted CAS
  → Assess OBS text and write verdict
  ✓ Inspect updated evidence docs and line references
  ✓ Round-trip two evidence bundles with verify_chaintape
  ✓ Resolve one WorkTx CID chain from extracted CAS
  ✓ Assess OBS text and write verdict
codex
**Section A: Round-2 Verdict**

**PASS** for the audited working tree. The round-1 VETO basis is closed: the ChainTape/CAS git stores are now restorable from the packaged archives, replay reports reproduce, and CID resolution works from the extracted committed-style evidence.

Important scope note: `HEAD` is still `4470036`; the audited artifacts are uncommitted/untracked. `HEAD` alone still represents the round-1 VETO state. Ship only after adding/committing the remediation files.

**Section B: RQ Dispositions**

**RQ1: PASS.** I sampled one SOLVED run and one UNSOLVED run. For `single_n1_mathd_algebra_171`, archives are non-empty: `runtime_repo.dotgit.tar.gz=12485` bytes, `cas.dotgit.tar.gz=11827` bytes. For `full_5_problems_n1/run_4_aime_1997_p9`, archives are non-empty: `11445` and `10678` bytes. Both extracted cleanly, `verify_chaintape` ran successfully, and normalized `replay_report.json` matched the checked-in report modulo `run_id`/`epoch`: shell result `compare=STRUCTURALLY_IDENTICAL` for both. The sampled reports show all top-level verifier booleans true at [single replay_report.json](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json:4) and [aime replay_report.json](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json:4). Minor wording nit: the JSON has 7 top-level verifier booleans plus `detail.initial_q_state_loaded_from_disk`; not literally 9 boolean fields.

**RQ2: PASS.** I also swept all 10 run dirs, extracting both archives and re-running `verify_chaintape`; shell result: `runs_checked=10`, every run `replay=ok`, `bool_false=0`. The 8 solved runs reproduced `l4=3 l4e=3`; the 2 unsolved runs reproduced `l4=2 l4e=2`. The README’s reproduce recipe is now directionally correct at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:222), and the artifact list now names the archived `.git` stores and replay reports at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:183). Acceptance clause 4 is satisfied from the working tree artifacts.

**RQ3: PASS.** On the extracted solved run, I decoded the L4 chain via `Git2LedgerWriter::read_at`, decoded the accepted `TypedTx::Work`, then resolved through the production CAS path. Shell result: `logical_t=3`, `entry_payload_cid=cid:d2fa...91a7 bytes=361`, `work_proposal_cid=cid:bbad...7e51 bytes=184`, `telemetry_verification_result_cid=cid:ea8f...7287 bytes=220`, `verification_proof_artifact_cid=cid:c31a...ae83 bytes=84`, `verification_verified=true`. The sidecar contains the corresponding proof payload, verification result, telemetry, and Work payload rows at [`.turingos_cas_index.jsonl`](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl:8). `CasStore::get` is the strict sidecar → git blob → sha256 check at [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:213), and the telemetry / verification decoders call it at [proposal_telemetry.rs](/home/zephryj/projects/turingosv4/src/runtime/proposal_telemetry.rs:332) and [verification_result.rs](/home/zephryj/projects/turingosv4/src/runtime/verification_result.rs:155).

**RQ4: PASS.** OBS-1 §2.1.a accurately captures round-1 Q2: `Complete` builds `tape_chain + tactic` before verification at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2132), but the artifact builder is passed only `tactic.as_bytes()` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2218). The new self-contained proof-artifact recommendation at [OBS-1](/home/zephryj/projects/turingosv4/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md:115) is sufficient for TB-8+: either chain the prefix or store the concatenated proof blob and verify from that committed blob alone. OBS-2 §0 is also correct for round-1 Q10: `record_tool_stdout` only increments token cost at [cost_aggregator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs:57), prompt errors read bounded rejection labels at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1344), and `TopKClasses` broadcasts class counts, not raw strings, at [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:576).

**Section C: Closure Table**

| Round-1 finding | Round-2 disposition |
|---|---|
| #1 Evidence packaging missing `.git` stores | **Closed in working tree.** Two sampled runs and all-10 sweep round-trip from archives. Must be committed before ship. |
| #2 Missing per-run `replay_report.json` | **Closed in working tree.** Reports exist and normalized verifier output matches for sampled and all swept runs. |
| #3 PartialOk → Complete proof-prefix dependency | **Closed for TB-7R ship as explicit OBS carry-forward.** Technical gap remains deferred to TB-8+, with sufficient remediation text in OBS-1 §4.3. |
| #4 OBS-2 stale prompt-pollution premise | **Closed.** Correction matches code path; stale premise is properly retired. |

**Section D: Recommendation**

**Ship may proceed after committing the working-tree remediation artifacts.** The round-1 VETO is closed. Clean up the minor README wording when convenient: change “9 boolean indicators” to the actual verifier shape and remove the duplicated replay-count block at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:206).
tokens used
178,813
**Section A: Round-2 Verdict**

**PASS** for the audited working tree. The round-1 VETO basis is closed: the ChainTape/CAS git stores are now restorable from the packaged archives, replay reports reproduce, and CID resolution works from the extracted committed-style evidence.

Important scope note: `HEAD` is still `4470036`; the audited artifacts are uncommitted/untracked. `HEAD` alone still represents the round-1 VETO state. Ship only after adding/committing the remediation files.

**Section B: RQ Dispositions**

**RQ1: PASS.** I sampled one SOLVED run and one UNSOLVED run. For `single_n1_mathd_algebra_171`, archives are non-empty: `runtime_repo.dotgit.tar.gz=12485` bytes, `cas.dotgit.tar.gz=11827` bytes. For `full_5_problems_n1/run_4_aime_1997_p9`, archives are non-empty: `11445` and `10678` bytes. Both extracted cleanly, `verify_chaintape` ran successfully, and normalized `replay_report.json` matched the checked-in report modulo `run_id`/`epoch`: shell result `compare=STRUCTURALLY_IDENTICAL` for both. The sampled reports show all top-level verifier booleans true at [single replay_report.json](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/replay_report.json:4) and [aime replay_report.json](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/full_5_problems_n1/run_4_aime_1997_p9/replay_report.json:4). Minor wording nit: the JSON has 7 top-level verifier booleans plus `detail.initial_q_state_loaded_from_disk`; not literally 9 boolean fields.

**RQ2: PASS.** I also swept all 10 run dirs, extracting both archives and re-running `verify_chaintape`; shell result: `runs_checked=10`, every run `replay=ok`, `bool_false=0`. The 8 solved runs reproduced `l4=3 l4e=3`; the 2 unsolved runs reproduced `l4=2 l4e=2`. The README’s reproduce recipe is now directionally correct at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:222), and the artifact list now names the archived `.git` stores and replay reports at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:183). Acceptance clause 4 is satisfied from the working tree artifacts.

**RQ3: PASS.** On the extracted solved run, I decoded the L4 chain via `Git2LedgerWriter::read_at`, decoded the accepted `TypedTx::Work`, then resolved through the production CAS path. Shell result: `logical_t=3`, `entry_payload_cid=cid:d2fa...91a7 bytes=361`, `work_proposal_cid=cid:bbad...7e51 bytes=184`, `telemetry_verification_result_cid=cid:ea8f...7287 bytes=220`, `verification_proof_artifact_cid=cid:c31a...ae83 bytes=84`, `verification_verified=true`. The sidecar contains the corresponding proof payload, verification result, telemetry, and Work payload rows at [`.turingos_cas_index.jsonl`](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/single_n1_mathd_algebra_171/cas/.turingos_cas_index.jsonl:8). `CasStore::get` is the strict sidecar → git blob → sha256 check at [store.rs](/home/zephryj/projects/turingosv4/src/bottom_white/cas/store.rs:213), and the telemetry / verification decoders call it at [proposal_telemetry.rs](/home/zephryj/projects/turingosv4/src/runtime/proposal_telemetry.rs:332) and [verification_result.rs](/home/zephryj/projects/turingosv4/src/runtime/verification_result.rs:155).

**RQ4: PASS.** OBS-1 §2.1.a accurately captures round-1 Q2: `Complete` builds `tape_chain + tactic` before verification at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2132), but the artifact builder is passed only `tactic.as_bytes()` at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:2218). The new self-contained proof-artifact recommendation at [OBS-1](/home/zephryj/projects/turingosv4/handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md:115) is sufficient for TB-8+: either chain the prefix or store the concatenated proof blob and verify from that committed blob alone. OBS-2 §0 is also correct for round-1 Q10: `record_tool_stdout` only increments token cost at [cost_aggregator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/cost_aggregator.rs:57), prompt errors read bounded rejection labels at [evaluator.rs](/home/zephryj/projects/turingosv4/experiments/minif2f_v4/src/bin/evaluator.rs:1344), and `TopKClasses` broadcasts class counts, not raw strings, at [bus.rs](/home/zephryj/projects/turingosv4/src/bus.rs:576).

**Section C: Closure Table**

| Round-1 finding | Round-2 disposition |
|---|---|
| #1 Evidence packaging missing `.git` stores | **Closed in working tree.** Two sampled runs and all-10 sweep round-trip from archives. Must be committed before ship. |
| #2 Missing per-run `replay_report.json` | **Closed in working tree.** Reports exist and normalized verifier output matches for sampled and all swept runs. |
| #3 PartialOk → Complete proof-prefix dependency | **Closed for TB-7R ship as explicit OBS carry-forward.** Technical gap remains deferred to TB-8+, with sufficient remediation text in OBS-1 §4.3. |
| #4 OBS-2 stale prompt-pollution premise | **Closed.** Correction matches code path; stale premise is properly retired. |

**Section D: Recommendation**

**Ship may proceed after committing the working-tree remediation artifacts.** The round-1 VETO is closed. Clean up the minor README wording when convenient: change “9 boolean indicators” to the actual verifier shape and remove the duplicated replay-count block at [README.md](/home/zephryj/projects/turingosv4/handover/evidence/tb_7r_smoke_2026-05-02/README.md:206).
