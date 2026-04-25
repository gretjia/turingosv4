#!/usr/bin/env python3
"""Gemini mid-term audit — Phase B B2 (cost) + B3 (wall-clock) + B4 (dual PPUT).
Independent of Codex. Output: GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md"""
import os, sys, json, urllib.request, urllib.error, time, subprocess
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Mid-Term Audit — Phase B B2/B3/B4 Implementation

**Role**: skeptical adversarial reviewer. Independent of Codex.

**Mandate**: this is a MID-TERM audit — Phase B has 7 items (B1 done & committed; B2/B3/B4 just landed locally; B5/B6/B7/B7-extra not started). PASS/CHALLENGE/VETO the B2-B4 implementation BEFORE B5+ build on top.

**Why mid-term**: B2 establishes the cost-accounting accumulator. B3 establishes the wall-clock bracket. B4 establishes the dual-PPUT (runtime vs verified) split. ALL THREE are foundations for the Phase C ablation (Soft Law / Panopticon / Amnesia / Homogeneous). If a Goodhart attack surface or honest-accounting bug is baked in here, B5-B7 won't catch it — the conformance tests B5 will write would be written AGAINST this same code, blind to its own holes.

**State of the code**:
- 143/143 cargo test --workspace PASS (was 131 baseline; +3 B2 unit tests, +5 B3 unit tests, +4 B4 unit tests)
- 4 PREREG § 3 anti-Goodhart conformance tests pass by name:
  - `test_failed_branches_counted_in_total_cost` (B2)
  - `test_wall_clock_first_read_to_final_accept` (B3)
  - `test_pput_verified_zero_when_lean_rejects` (B4)
- Three new files: cost_aggregator.rs (~130 lines), wall_clock.rs (~135 lines), post_hoc_verifier.rs (~165 lines)
- One library API change: GenerateResponse exposes `prompt_tokens` (was dropping it)
- Evaluator wired at every LLM call + tool stdout site; OMEGA-accept paths flip last failed proposal to accepted via `flip_last_failed_to_accepted`

**Specific questions you must answer (cite line/file/§ for every finding)**:

## Q1 — C_i full-cost honesty (anti-Goodhart #8)
PREREG § 3 #8 mandates `total_run_token_count` includes ALL proposals (winning + failed). The B2 wiring records:
- Every successful `client.generate(...)` → `record_llm_call(prompt_tokens, completion_tokens)` + `record_proposal(false)` at parse time
- OMEGA-accept paths → `flip_last_failed_to_accepted()` before return
- Tool stdout sites: search hits, OMEGA reject error_detail, step Reject reason, parse-fail label

Vulnerabilities to scrutinize:
- (Q1.a) Are there Ok(response) call sites NOT wired through `record_llm_call`?
- (Q1.b) Are there proposal parse paths that DON'T call `record_proposal`?
- (Q1.c) Tool stdout: bus mutations (append, invest, post) DON'T add to tool_tokens. Is that defensible (the prompt-embedded next-turn state is already in prompt_tokens of the next call), or is it a hidden cost dropped?
- (Q1.d) `flip_last_failed_to_accepted` saturates at 0. Could a wiring bug call it more times than `record_proposal(false)` and silently corrupt the count?
- (Q1.e) Tool-stdout token counter uses chars/4 ceil heuristic. PREREG § 5 doesn't pin this to API tokens — is chars/4 honest enough, or should it be tokenized?

## Q2 — T_i wall-clock seal (anti-Goodhart Soft Law escape)
PREREG § 5 says T_i = first agent prompt construction → final ground-truth Lean accept (or external timeout). The B3 wiring:
- `mark_first_read` is idempotent — only the FIRST tx's prompt-build sticks
- `mark_final_accept` is NOT idempotent — every call updates the close instant; LAST call wins

Vulnerabilities to scrutinize:
- (Q2.a) `mark_final_accept` fires AFTER each Lean call (run_oneshot: after verify_omega; run_swarm complete: after both verify_omega_detailed paths; run_swarm step: after verify_partial). Is this the right place — i.e., is there a Lean call path that bypasses it?
- (Q2.b) The end-of-loop no-OMEGA path also fires `mark_final_accept`. Correct?
- (Q2.c) Soft Law (Phase C) can fake runtime accept without running Lean. The current bracket fires `mark_final_accept` BEFORE the runtime/verified branch. Is the seal preserved when Phase C lands a `--mode soft_law` toggle?
- (Q2.d) `from_instants` is `#[cfg(test)]`. Could a malicious test-time path leak it into runtime?
- (Q2.e) `elapsed_ms` returns `Some(start.elapsed().as_millis())` when only `first_read` is set (no final_accept). Is that the right semantic for the no-OMEGA exit, or could it under-count?

## Q3 — Dual-PPUT divergence as H1 detection
PREREG § 3 #7 / round-4 fix R4-2: under Soft Law, `pput_runtime - pput_verified > 0` is the H1 signal. The B4 implementation:
- `compute_progress_runtime(runtime_accepted) = if runtime_accepted {1} else {0}`
- `compute_progress_verified(runtime_accepted, post_hoc_verified) = if runtime_accepted && post_hoc_verified {1} else {0}`
- In `make_pput`: Phase B sets `post_hoc_verified = has_gp` (i.e., runtime IS Lean today)

Vulnerabilities to scrutinize:
- (Q3.a) `compute_progress_verified` uses AND of (runtime, verified). Is this right? PREREG says progress = Lean ground truth. If runtime fails but verified somehow passes (impossible today, but B4 is the contract), should progress=1 or 0?
- (Q3.b) Phase B make_pput hard-codes `post_hoc_verified = has_gp`. Phase C will diverge this — but the wiring is INSIDE make_pput. Is that the right call site, or should it be at the OMEGA-accept return path (caller-side) so each mode (full / soft_law / panopticon) can override independently?
- (Q3.c) `verify_post_hoc(oracle, payload)` is defined but NOT called by current evaluator. Is this dead code, or correctly deferred to Phase C? If deferred, document the contract clearly enough that Phase C can't accidentally skip it.
- (Q3.d) `pput_m_verified = 1e6 × pput_verified` — display unit. Is the multiplier precision-safe (no float drift on common C_i × T_i values)?

## Q4 — Backward compat with legacy jsonl
B1 introduced schema_version dispatch (`RunRecord::from_json`). B2-B4 ADD optional fields to `PputResult`. Legacy Paper-1 jsonl rows lack these fields.

Vulnerabilities to scrutinize:
- (Q4.a) All new fields use `#[serde(skip_serializing_if = "Option::is_none")]`. A pre-B2 binary's emit produces None for new fields; deserializing on the new code path → Ok(...) with None. Round-trip preserved?
- (Q4.b) The 3 new fields total_run_token_count / total_wall_time_ms / pput_verified are now sometimes present, sometimes None. Downstream tooling needs to handle BOTH. Is the field set on PputResult consistent with B1's RunAggregate v2 schema? Where do they MISS-align?
- (Q4.c) Hybrid_v1 condition uses `..r` field-spread to inherit from the inner result. Do the new fields propagate correctly through the spread?

## Q5 — Schema → emit alignment
B1 froze the v2 RunAggregate schema. B2/B3/B4 add fields to LEGACY PputResult, NOT directly to RunAggregate. The plan says B2-B4 wire emit; B4 implies switching to RunAggregate but the implementation kept PputResult.

Vulnerabilities to scrutinize:
- (Q5.a) Is keeping PputResult (legacy) instead of switching to RunAggregate (v2) defensible at the B4 boundary? When does the switch happen?
- (Q5.b) Field name alignment: `pput_runtime`, `pput_verified`, `pput_m_verified`, `verified` match RunAggregate. But `total_run_token_count` matches RunAggregate; `failed_branch_count` matches RunAggregate; `total_wall_time_ms` matches RunAggregate. Anything mis-spelled?
- (Q5.c) RunAggregate has `progress: u8` (Lean ground truth). PputResult does NOT have `progress` — only `verified: Option<bool>`. Is this gap intentional (B5 will fill it) or a bug?

## Q6 — Architectural / structural concerns
- (Q6.a) STEP_B_PROTOCOL: bus.rs / kernel.rs / wallet.rs are restricted (require parallel-branch A/B, not direct edit). The B2-B4 diff touches `src/drivers/llm_http.rs`. Is `drivers/llm_http.rs` restricted? If yes, this is a process violation.
- (Q6.b) Generator ≠ Evaluator (Art. V.1, C-010): code was authored by Claude. This audit is the Evaluator step. But three modules + ~360 lines wiring is a bigger surface than typical. Are there hidden coupling concerns?
- (Q6.c) Trust Root (PREREG § 1.8): does B2-B4 introduce new files that should be Trust-Rooted (cost_aggregator.rs, wall_clock.rs, post_hoc_verifier.rs)? B7 will set this up; flag if these files should be on the Trust Root manifest.

## Q7 — End-to-end stress test
Mentally simulate a Soft Law ablation run on hard-10 problem #1, seed=31415, with B2-B4 in place + B5+ stubbed:
- Agent gets fake-accept signal (Soft Law fakes runtime acceptance)
- B2 records the LLM call cost (correct)
- B3 closes wall-clock bracket (correct)
- B4 hard-codes post_hoc_verified = has_gp = true (WRONG — should detect Soft Law)

Question: does the architecture FORCE Phase C Soft Law to wire the post-hoc verify call BEFORE setting has_gp=true? Or could a future implementer inadvertently set has_gp=true without running post-hoc, missing the divergence?

This is the question that matters: is the current B4 design Phase-C-safe by construction, or only Phase-C-safe by future discipline?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- If CHALLENGE: top 3 P0 issues with line numbers
- Conviction
- Specific recommendations for B5/B6/B7 design (do B2-B4 leave any debt)?

You MUST be willing to PASS if the implementation is sound.

---

# PHASE_B_IMPLEMENTATION_PLAN.md (B2/B3/B4 sections)

"""

# Read PHASE_B_IMPLEMENTATION_PLAN B2/B3/B4 sections (lines 99-175)
plan_text = (ROOT / "handover/preregistration/PHASE_B_IMPLEMENTATION_PLAN.md").read_text()
plan_lines = plan_text.splitlines()
plan_b2_b4 = "\n".join(plan_lines[98:175])

# Read PREREG § 3 + § 5 sections via awk-like splitting
prereg_text = (ROOT / "handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md").read_text()
def slice_section(text, start_marker, end_marker):
    lines = text.splitlines()
    out, capturing = [], False
    for line in lines:
        if line.startswith(start_marker):
            capturing = True
        elif capturing and line.startswith(end_marker):
            break
        if capturing:
            out.append(line)
    return "\n".join(out)

prereg_s3 = slice_section(prereg_text, "## 3. anti-Goodhart", "## 4.")
prereg_s5 = slice_section(prereg_text, "## 5. directional hypotheses", "## 6.")

# New files (verbatim)
ca = (ROOT / "experiments/minif2f_v4/src/cost_aggregator.rs").read_text()
wc = (ROOT / "experiments/minif2f_v4/src/wall_clock.rs").read_text()
phv = (ROOT / "experiments/minif2f_v4/src/post_hoc_verifier.rs").read_text()

# Diff (pre-captured to /tmp/b2_b4_evaluator_diff.patch)
diff_path = Path("/tmp/b2_b4_evaluator_diff.patch")
if not diff_path.exists():
    # Re-capture
    res = subprocess.run(
        ["git", "diff", "--no-color", "--",
         "experiments/minif2f_v4/src/bin/evaluator.rs",
         "experiments/minif2f_v4/src/lib.rs",
         "src/drivers/llm_http.rs"],
        cwd=ROOT, capture_output=True, text=True, check=True)
    diff_text = res.stdout
else:
    diff_text = diff_path.read_text()

full_prompt = (
    brief + plan_b2_b4 +
    "\n\n---\n\n# PREREG § 3 (anti-Goodhart conformance)\n\n" + prereg_s3 +
    "\n\n---\n\n# PREREG § 5 (directional hypotheses + definitions)\n\n" + prereg_s5 +
    "\n\n---\n\n# experiments/minif2f_v4/src/cost_aggregator.rs (B2, NEW)\n\n```rust\n" + ca +
    "\n```\n\n---\n\n# experiments/minif2f_v4/src/wall_clock.rs (B3, NEW)\n\n```rust\n" + wc +
    "\n```\n\n---\n\n# experiments/minif2f_v4/src/post_hoc_verifier.rs (B4, NEW)\n\n```rust\n" + phv +
    "\n```\n\n---\n\n# Diff: evaluator.rs + lib.rs + drivers/llm_http.rs (B2/B3/B4 wiring)\n\n```diff\n" + diff_text +
    "\n```\n\n---\n\nNow give your INDEPENDENT mid-term audit. Cite §/line for every finding."
)

print(f"[gemini b2-b4] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": full_prompt}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()
headers = {"Content-Type": "application/json"}

t0 = time.time()
req = urllib.request.Request(url, data=body, headers=headers, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except urllib.error.HTTPError as e:
    print(f"[gemini b2-b4] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini b2-b4] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PPUT_CCL_B2_B4_AUDIT_2026-04-25.md"
header = (f"# Gemini PPUT-CCL Phase B B2/B3/B4 Mid-Term Audit\n"
          f"**Date**: 2026-04-25\n"
          f"**Targets**: cost_aggregator.rs + wall_clock.rs + post_hoc_verifier.rs + evaluator.rs wiring\n"
          f"**Test baseline**: 143/143 PASS (was 131; +12 from B2-B4 unit tests)\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini b2-b4] saved: {out}")
