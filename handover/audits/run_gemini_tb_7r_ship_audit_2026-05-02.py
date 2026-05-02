#!/usr/bin/env python3
"""Gemini TB-7R ship audit (audit-point-2 per charter §3 Deliverable G).

Class 3 (auth-crypto-money) — Gemini strategic / architectural angle.
Independent of Codex ship audit (parallel, implementation-paranoid).
Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

Strategic / architectural / constitutional angle: did the implementation
honor every constraint the charter committed to? Specifically, four-clause
acceptance + 7 ship conditions + hard guardrails. Plus: bounding check on
the two architect-acknowledged OBS items folded in.

Strategic-tier model `gemini-3.1-pro-preview` first; if 429 capacity-exhausted,
caller must label this audit `degraded` per memory `feedback_dual_audit`.
"""
import json
import pathlib
import subprocess
import sys
import time
import urllib.error
import urllib.request

REPO = pathlib.Path("/home/zephryj/projects/turingosv4")
ENV_FILE = pathlib.Path("/home/zephryj/projects/turingosv3/.env")
OUT = REPO / "handover/audits/GEMINI_TB_7R_SHIP_AUDIT_2026-05-02.md"

key = None
for line in ENV_FILE.read_text().splitlines():
    if line.startswith("GEMINI_API_KEY="):
        key = line.split("=", 1)[1].strip()
        break
if not key:
    sys.exit("ERROR: no GEMINI_API_KEY in env")

PROMPT = """You are Gemini DeepThink, the strategic architectural reviewer for TuringOS v4.

This is the **TB-7R ship-gate audit (audit-point-2)** per charter §3 Deliverable G. Class 3 (auth-crypto-money). Your angle is **strategic / architectural / constitutional**. Codex is running the ship audit in parallel with the implementation-paranoid angle.

Per memory `feedback_dual_audit_conflict`: VETO > CHALLENGE > PASS.

## Mandate

TB-7R ("Constitution-Aligned Frame B Repair") is the strict-constitution version of TB-7.7 closure: predicate-pass → L4 accepted, predicate-fail → L4.E rejected evidence; stake/escrow legitimacy ≠ predicate pass. Audit the FULL TB-7R commit range against the four-clause acceptance, the 7 ship conditions, and the hard-guardrail forbidden list.

## Audit target

```text
Predecessor (TB-7 ship):  9e74195
TB-7R range:              9e74195..4470036  (5 commits)
HEAD:                     4470036  TB-7R parent_tx ParentTxState + 6 conformance tests + verdict 2026-05-02
                          013f2ce  TB-7R F (smoke evidence) — 10 runs across single/half/full
                          b517ae5  TB-7R audit-fix — Codex CHALLENGE Claim 7 → orphan TRACE_MATRIX
                          392a516  TB-7R C+D+CP2 — genesis_report.json emission + on-chain TaskOpen/EscrowLock
                          696d10f  TB-7R A+B+E — verdict ingestion + L4 purity audit + ChainTape-mode fail-closed
```

Workspace canonical test count at HEAD: `cargo test --workspace` → **712 / 0 / 150** (+26 net TB-7R tests vs TB-7 ship 686/0/150 baseline).

## The four-clause acceptance criterion (charter §1)

```text
1. For every externalized LLM proposal:
     L4 accepted WorkTx OR L4.E rejected evidence — never both, never neither.
2. For every L4 accepted WorkTx:
     predicate evidence (Lean VerificationResult) exists and resolves from CAS.
3. For every failed proposal:
     in L4.E only; raw diagnostic shielded but auditable.
4. For every dashboard report:
     deletable and regeneratable from ChainTape + CAS alone.
```

## The 7 ship conditions (architect verdict 2026-05-02 §4)

```text
1. All seven dashboard indicators remain green
2. All real externalized proposals are represented in L4 or L4.E
3. Solved runs have chain_oracle_verified=true and a rendered golden path
4. Unsolved runs have no fake accepted nodes
5. Proposal telemetry and proposal payload CIDs resolve
6. Forced parent_tx conformance test passes (6/6 in tests/tb_7r_parent_tx_conformance.rs)
7. README explicitly states that natural parent_tx_edges=0 occurred because
   complete-tool runs solved in one proposal
```

## Two architect-acknowledged OBS items (folded in for awareness)

These are KNOWN at the time of this audit. Treat them as published claims; flag VETO only if you find evidence the OBS bounding is wrong (i.e. the issue is actually a TB-7R ship blocker, not a follow-up).

**OBS-1 (architect-acknowledged):** `handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md`
- Architect verdict 2026-05-02 §6 explicitly frames coverage-denominator hardening as **post-TB-7R**, not a TB-7R blocker.
- The OBS argues: the strict three-node taxonomy makes "every externalized proposal lands in L4 or L4.E" trivially true, because only `submit_typed_tx` is "externalized." But `step` tool's `PartialOk` and `Reject` branches consume LLM responses and bypass `submit_typed_tx`.
- Question for you: is the OBS's framing constitutionally consistent? Is the architect's pass-through (delaying this to TB-7.5/TB-8+) a defensible reading of Frame B closure under the four-clause acceptance? Or does the four-clause acceptance actually require ALL LLM-output paths (not just `submit_typed_tx`-routed) to terminate in L4 or L4.E?

**OBS-2 (architect-silent):** `handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md`
- Claude self-flagged 2026-05-02; architect verdict did NOT explicitly address. Per OBS §3, "this is NOT a TB-7R ship blocker per the verdict's listed ship conditions." Flagged for future architect ruling.
- Concern: `acc.record_tool_stdout(reason)` flows raw Lean error reasons into next-prompt's errors_history. In multi-agent runs, if `acc` is shared, agent-1's raw Lean error could leak into agent-2's prompt — potentially violating Art. III.4 selective-broadcasting / shielding ("失败候选不能污染其他 Agent 上下文").
- Question for you: Art. III.4 vs PPUT-CCL B2 (which explicitly designs error-feedback-loops to flow into next prompt) is a constitutional tension. Is the OBS deferring this correctly, or does the constitution actually bind here in a way that should block ship?

## Round-1 audit questions (8; strategic / architectural)

**Q1. Four-clause acceptance — clause 1 (every externalized proposal in L4 OR L4.E)**: walk the architecture from "LLM emits a proposal" to "row appears in L4 / L4.E / neither." Under the strict three-node taxonomy (where "externalized" = `submit_typed_tx`-routed), is clause 1 trivially or substantively satisfied? Specifically — under the OBS-1 bounding, is there a reading of the constitution under which `step`-tool `PartialOk` rows (which go to kernel.tape shadow_only, not chain) are themselves "externalized proposals" that should land in L4 / L4.E? Tie this to Art. I.1 + Art. III.4 + WP § 5 L3/L4.

**Q2. Four-clause acceptance — clause 2 (predicate evidence resolves from CAS)**: TB-7R adds `VerificationResult` as a CAS object referenced by `ProposalTelemetry.verification_result_cid`. Walk `src/runtime/verification_result.rs` + `src/runtime/proposal_telemetry.rs` + `src/runtime/chain_derived_run_facts.rs`. Is `chain_oracle_verified=true` a sufficient predicate for "L4 accepted is predicate-passed"? Does it close the architectural defect that previously allowed stake/escrow legitimacy to manufacture acceptance? Or is there a residual gap?

**Q3. Four-clause acceptance — clause 3 (failed proposal in L4.E only; raw diagnostic shielded)**: TB-1 P0-3 `raw_diagnostic_cid` serde shield is the structural shielding mechanism. Verify it still holds across TB-7R changes. The L4.E entries in TB-7R smoke must show shielded raw_diagnostic in the public materialized view. Is there any path where a failed-proposal raw diagnostic leaks into a publicly-readable L4.E surface?

**Q4. Four-clause acceptance — clause 4 (dashboard regeneratable from ChainTape + CAS alone)**: TB-7R adds `src/bin/audit_dashboard.rs` (~791 LOC). The architect rule (charter §4 + §8 production claim) is "audit dashboard is a read-only materialized view from ChainTape + CAS — deleting it does not lose authoritative state." Does the dashboard implementation honor this? Specifically: does the dashboard ever WRITE to ChainTape or CAS during rendering? Does it cache state that, if lost, would prevent regeneration? Does it consult any source-of-truth other than the runtime_repo's L4 + L4.E + cas/?

**Q5. Hard-guardrail compliance (charter §4)**: re-walk the 13 forbidden lines. The most subtle ones to check architecturally:
- "no per-tactic decomposition" (verdict A1 deferred to TB-8+) — does anything in TB-7R range introduce per-tactic node-counting or per-tactic chain entries? The `parent_tx` plumbing is proposal-level only — verify.
- "no retroactive ledger rewrite" — search if any TB-7R commit modifies files inside `handover/evidence/tb_7_*_2026-05-01/` directories (i.e. pre-TB-7R evidence). README annotations are fine; ledger-state mutation is forbidden.
- "no fabricated historical genesis_report.json" — verify no genesis_report.json appears in any pre-TB-7R evidence dir as a result of TB-7R changes.
- "no new TypedTx variant" — verify (the charter inherits the WP-canonical reading; VerificationResult is a CAS object, not a TypedTx).

**Q6. parent_tx as conditional invariant (verdict 2026-05-02 §1-§3)**: the architect verdict 2026-05-02 introduced `parent_tx_state` enum with four cases (`SingletonGoldenPathValid` / `NoMultiAttemptObserved` / `MultiAttemptDagValid` / `MissingParentTxViolation`). Walk the dashboard logic that distinguishes these four cases. Is the case-split exhaustive? Could a future LLM run produce a state that doesn't match any of the four? Specifically — is there any state where the chain has multi-attempt branches BUT the dashboard renders `SingletonGoldenPathValid` (i.e. a missed violation)?

**Q7. The constitutional question raised by OBS-2**: Art. III.4 ("失败候选不能污染其他 Agent 上下文 / 顶层白盒对系统信息做 量化、广播、屏蔽") vs PPUT-CCL B2 ("step rejection reason flows into next prompt for agent self-correction"). The architect verdict 2026-05-02 was silent on this. Two readings are plausible: (a) architect implicitly accepts PPUT-CCL B2 as the override; (b) architect didn't notice, and constitutional interpretation actually binds here. Which reading is defensible? Should TB-7R ship with OBS-2 deferred (current path) or should it block ship?

If your reading is (b) AND multi-agent runs are demonstrably leaking — VETO. If (a) is defensible OR if multi-agent runs aren't actually leaking — OBS-2 deferral stands.

**Q8. Production claim defensibility (charter §8)**: re-read the production claim TB-7R commits to:

> "TuringOS Frame B is constitution-aligned: every externalized LLM proposal lands in either L4 accepted (predicate-passed) or L4.E rejected evidence, never both. Predicate pass / fail (Lean VerificationResult) alone determines L4 vs L4.E — stake/escrow do not manufacture acceptance. The DAG is proposal-level (per-tactic deferred to TB-8+). Genesis state for new runs is established via on-chain TaskOpenTx + EscrowLockTx, not memory preseed. Historical evidence is grandfathered, not rewritten. The audit dashboard is a read-only materialized view from ChainTape + CAS — deleting it does not lose authoritative state. NodeMarket, settlement, slash, and per-tactic DAG remain post-TB-7R."

Is this claim defensible at HEAD? Are there any clauses that overstate what the implementation actually delivers? Are there clauses that understate (i.e. the implementation is actually more than the claim)? Suggest revisions if needed.

## Verdict format

Section A: Overall ship verdict (PASS / CHALLENGE / VETO) with conviction (1-5).
Section B: Per-Q1-Q8 disposition (one paragraph each + verdict tag + cite file:line where possible).
Section C: NEW constitutional debt introduced by TB-7R (each entry: file:line + what + why + remediation).
Section D: OBS bounding review — does OBS-1 stand? Does OBS-2 stand? Severity if either is wrong.
Section E: Production claim review — clauses that overstate / understate / need rewording.
Section F: Recommendation — ship-clear / ship-with-OBS-tightening / revise / VETO + rationale.

Be direct. Cite file:line. The materials below include: charter, authorization verdict, parent_tx verdict, two OBS files, smoke evidence README, prior Codex micro-audit (audit-point-1), L4 purity audit, the diff `git diff 9e74195..4470036`, and the new/heavily-changed source files post-state.

---

# XREF materials follow.
"""

attachments = []

def append_file(label, path, fence="markdown"):
    full = REPO / path
    if not full.exists():
        attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}` [missing]\n")
        return
    attachments.append(f"\n\n---\n\n## XREF: {label} — `{path}`\n\n```{fence}\n{full.read_text()}\n```\n")

# Charter + verdicts.
append_file("TB-7R charter (the ship gate)", "handover/tracer_bullets/TB-7R_charter_2026-05-01.md")
append_file("TB-7R authorization verdict 2026-05-01", "handover/directives/2026-05-01_TB7R_AUTHORIZATION_VERDICT.md")
append_file("TB-7R parent_tx verdict 2026-05-02 (BINDING)", "handover/directives/2026-05-02_TB7R_PARENT_TX_DAG_SMOKE_VERDICT.md")

# Two OBS items folded in for visibility.
append_file("OBS-1: post-TB-7R coverage denominator (architect-acknowledged follow-up)", "handover/alignment/OBS_TB7R_COVERAGE_DENOMINATOR_2026-05-02.md")
append_file("OBS-2: Art. III.4 prompt pollution (architect-silent; flag for future ruling)", "handover/alignment/OBS_TB7R_ART_III_4_PROMPT_POLLUTION_2026-05-02.md")

# Prior audit-point-1 micro-audit + remediation.
append_file("Codex TB-7R micro-audit (audit-point-1)", "handover/audits/CODEX_TB7R_MICRO_AUDIT_2026-05-02.md")
append_file("L4 purity audit (Deliverable A; zero violations)", "handover/audits/L4_PURITY_AUDIT_TB7R_2026-05-02.md")

# Smoke evidence top-level README.
append_file("TB-7R smoke evidence README (Deliverable F)", "handover/evidence/tb_7r_smoke_2026-05-02/README.md")

# TRACE_MATRIX orphan registry (Claim 7 remediation).
append_file("TRACE_MATRIX orphan registry (Claim 7 remediation)", "handover/alignment/OBS_R022_TRACE_MATRIX_TB7R_ORPHANS_2026-05-02.md")

# Three-node taxonomy + L4/L4.E separation (background).
append_file("Three-node taxonomy decision record", "handover/alignment/DECISION_ATTEMPT_STATE_REJECTION_NODES_2026-05-01.md")
append_file("L4 / L4.E ledger separation decision record", "handover/alignment/DECISION_REJECTION_EVIDENCE_LEDGER_2026-04-29.md")

# Constitution (architectural reference — auditor needs this for Art. references).
append_file("constitution.md (Art. I/III/V references — needed for Art. III.4 question)", "constitution.md")

# Heavily-changed source files (post-state).
append_file("src/runtime/genesis_report.rs (NEW; Deliverable C)", "src/runtime/genesis_report.rs", fence="rust")
append_file("src/runtime/proposal_telemetry.rs (extended)", "src/runtime/proposal_telemetry.rs", fence="rust")
append_file("src/runtime/verification_result.rs (NEW; Deliverable A predicate evidence)", "src/runtime/verification_result.rs", fence="rust")
append_file("src/runtime/chain_derived_run_facts.rs (extended; chain_oracle_verified)", "src/runtime/chain_derived_run_facts.rs", fence="rust")
append_file("src/runtime/agent_audit_trail.rs (small change)", "src/runtime/agent_audit_trail.rs", fence="rust")
append_file("src/runtime/mod.rs (delta)", "src/runtime/mod.rs", fence="rust")
append_file("src/bin/audit_dashboard.rs (NEW; clause-4 artifact)", "src/bin/audit_dashboard.rs", fence="rust")
append_file("experiments/minif2f_v4/src/chaintape_mode_gate.rs (NEW; Deliverable B fail-closed)", "experiments/minif2f_v4/src/chaintape_mode_gate.rs", fence="rust")
append_file("tests/tb_7r_parent_tx_conformance.rs (NEW; verdict 2026-05-02 §3 6-pack)", "tests/tb_7r_parent_tx_conformance.rs", fence="rust")

# Diff summary (full diff is too large for prompt; commit log + per-file stats are enough for arch-review).
log = subprocess.run(
    ["git", "-C", str(REPO), "log", "--pretty=format:%h %s%n%n%b%n----", "9e74195..4470036"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: TB-7R commit log (5 commits)\n\n```\n{log}\n```\n")

stat = subprocess.run(
    ["git", "-C", str(REPO), "diff", "--stat", "9e74195..4470036"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: TB-7R diff --stat (188 files)\n\n```\n{stat}\n```\n")

# Truncated diff (first 200k chars) — gives auditor the actual diff context.
diff = subprocess.run(
    ["git", "-C", str(REPO), "diff", "9e74195..4470036"],
    capture_output=True, text=True,
).stdout
attachments.append(f"\n\n---\n\n## XREF: TB-7R diff (truncated to first 200k chars)\n\n```diff\n{diff[:200_000]}\n```\n")

full_prompt = PROMPT + "".join(attachments)
print(f"[gemini tb-7r ship] prompt size: {len(full_prompt)} chars", file=sys.stderr)

# Strategic-tier first; degraded fallback if exhausted.
TIERS = [
    ("gemini-3.1-pro-preview", "strategic"),
    ("gemini-2.5-pro", "strategic-fallback"),
    ("gemini-2.5-flash", "strategic-flash"),
    ("gemini-2.5-flash-lite", "degraded"),
]

result_text = None
chosen_model = None
chosen_label = None
last_error = None

for model, label in TIERS:
    url = f"https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={key}"
    body = json.dumps({
        "contents": [{"parts": [{"text": full_prompt}]}],
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": 65536,
        },
    }).encode()
    req = urllib.request.Request(
        url, data=body,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    print(f"[gemini tb-7r ship] trying {model} ({label})", file=sys.stderr)
    t0 = time.time()
    try:
        with urllib.request.urlopen(req, timeout=900) as resp:
            data = json.loads(resp.read().decode())
    except urllib.error.HTTPError as e:
        body_text = e.read().decode()[:500]
        print(f"  HTTP {e.code}: {body_text}", file=sys.stderr)
        last_error = f"{model}: HTTP {e.code} — {body_text}"
        if e.code in (429, 503):
            continue
        sys.exit(1)
    except Exception as e:
        last_error = f"{model}: {e}"
        print(f"  exception: {e}", file=sys.stderr)
        continue
    elapsed = time.time() - t0
    print(f"  API returned in {elapsed:.1f}s", file=sys.stderr)
    candidates = data.get("candidates", [])
    if not candidates:
        last_error = f"{model}: no candidates — {json.dumps(data)[:500]}"
        continue
    parts = candidates[0].get("content", {}).get("parts", [])
    text = "".join(p.get("text", "") for p in parts)
    if not text:
        last_error = f"{model}: empty text — {json.dumps(data)[:500]}"
        continue
    result_text = text
    chosen_model = model
    chosen_label = label
    chosen_elapsed = elapsed
    break

if not result_text:
    sys.exit(f"All tiers failed. Last error: {last_error}")

OUT.parent.mkdir(parents=True, exist_ok=True)
git_head = subprocess.run(
    ["git", "-C", str(REPO), "rev-parse", "HEAD"],
    capture_output=True, text=True,
).stdout.strip()
header = f"""# Gemini TB-7R Ship Audit (audit-point-2; round 1)

**Date**: 2026-05-02
**Range**: `9e74195..4470036` (5 commits)
**HEAD**: {git_head}
**Workspace test count**: 712 / 0 / 150 (cargo test --workspace canonical)
**Audit class**: Class 3 (auth-crypto-money) — full dual; Codex-impl + Gemini-arch.
**Auditor**: Gemini DeepThink ({chosen_model}; tier label = {chosen_label})
**API latency**: {chosen_elapsed:.1f}s
**Prompt size**: {len(full_prompt)} chars
**Mandate**: TB-7R ship-gate strategic / architectural / constitutional audit (Q1-Q8). Independent of Codex (parallel; implementation-paranoid).

> If `tier label = degraded` above, this audit is DEGRADED-MODE per memory `feedback_dual_audit`. The merged dual verdict MUST display `degraded — Gemini at degraded tier` per TB-5/TB-6/TB-7 supplement precedent.

---

"""
OUT.write_text(header + result_text)
print(f"[gemini tb-7r ship] saved: {OUT}", file=sys.stderr)
print(f"[gemini tb-7r ship] tier used: {chosen_model} ({chosen_label})", file=sys.stderr)
