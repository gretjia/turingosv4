#!/usr/bin/env python3
"""Gemini ROUND-4 RE-AUDIT — Phase B B7-extra after dual-VETO fixes.
Independent of Codex. Output: GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md"""
import json, sys, time, subprocess, urllib.request, urllib.error
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini ROUND-4 RE-AUDIT — Phase B B7-extra (after VETO fixes)

**Role**: skeptical adversarial reviewer. Independent of Codex. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: this is the FOURTH audit pass.
- **Round 1** YOU VETO on Q2.b/Q2.e. Fixed in `15b87fb`.
- **Round 2** YOU PASS — but Codex VETO caught silent absorption defect. Round-2 fix `1df1f62`.
- **Round 3** YOU CHALLENGE on EXIT=0 + empty PPUT_RESULT non-exhaustive case. Codex CHALLENGE on `problem_file_missing` synthetic emit + boot preflight `|| true` exit-discard. **All addressed in commit `d0d474e`**.
- **Round 4 (this audit)**: scrutinize commit `d0d474e`. Did it resolve all round-3 CHALLENGE findings WITHOUT introducing new defects? **If PASS: 576-run batch authorized to launch overnight per user directive.**

**Round-3 fix summary (commit `d0d474e`)**:
1. Pre-loop adaptation-file existence preflight (Codex P0 #1): iterates ADAPTATION_IDS, checks each `$MINIF2F_DIR/MiniF2F/Test/${pid}.lean` exists, lists missing PIDs (up to 10), aborts with exit 2 if any missing. In-loop missing-file branch downgraded to race-condition guard (exit 6).
2. Boot preflight exit-code assertion (Codex P0 #2): captures PREFLIGHT_EXIT explicitly; asserts exit != 0 AND exit != 124; then content grep for TRUST_ROOT_TAMPERED. Three failure modes get distinct diagnostic messages.
3. EXIT=0 + empty PPUT_RESULT explicit branch (Gemini round-3 CHALLENGE): malformed-run case now exits 5 with dedicated diagnostic.

**Verification done**:
- 187/187 cargo test --workspace PASS + 20 ignored
- Happy-path smoke (--smoke, 40s) all SOLVED tx=1; new preflight steps print OK

**Decision rule**:
- If `d0d474e` resolves all round-3 CHALLENGE findings AND no new P0 AND prior VETO items (Q2.b/Q2.e) remain resolved: **PASS** with HIGH conviction → batch authorized.
- If new P0: **CHALLENGE**.
- If deeper architectural issue: **VETO**.

**Constitutional amendments since last audit** (commit `c061450`, sudo-authorized):
- Art. V.1.1: human sudo applies *only* to constitution.md
- Art. V.1.2: ArchitectAI has commit authority on non-constitution files (no sudo post-Veto-AI PASS)
- Art. V.1.3: JudgeAI renamed to **Veto-AI**; output domain narrowed to {PASS, VETO} only
- FC3 mermaid: judgeAI → vetoAI
- Art. V.3 (NEW): Constitutional Amendment Log

**State of the code (re-audit baseline)**:
- 187/187 cargo test --workspace PASS + 20 deferred-stub `#[ignore]`
- Hard-problem smoke (aime_1983_p2): control SOLVED tx=15 (~2.5 min); treatment UNSOLVED tx=50 with `synthetic_short_circuit=true` ✓
- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)

## Fix summary by previous finding (GEMINI focus)

| Previous finding | Severity | Fix commit + location |
|---|---|---|
| Q2.b: src/main.rs not in manifest | **VETO** | 15b87fb: added with SHA-256 622fee2d96a980d24f9fbaab3d0531c195a0a337fc3ddd2efb60bca90a1cfbf9 |
| Q2.e: Cargo.lock not in manifest | **VETO** | 15b87fb: added with SHA-256 577446e8fe11e91bc8751bf13e5ddca6c5faa64d3309b878768c550d3e6feb98 |
| Q1.a: equivalence claim wrong | CHALLENGE | 15b87fb: rollback_sim.rs header rewritten — narrow equivalence on (problem, seed, solved); explicit non-equivalence list |
| Q1.b: info!→warn! | CHALLENGE | 15b87fb: short-circuit log now `warn!` |
| Q3.a: proxy precision | CHALLENGE | Acknowledged in TRACE_MATRIX_v1 + rollback_sim.rs header — no PREREG modification (frozen) |
| Q3.d: silent skip in compute_p0 | CHALLENGE | 15b87fb: strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144 |
| Q3.e: runner script seeds not in Trust Root | CHALLENGE | Resolved transitively: runner.sh now in manifest |
| Q6.a: exit-2 not propagated | CHALLENGE | 15b87fb: runner has `set -euo pipefail` + invokes compute_p0 + propagates exit code |
| Q7.a: API drift mitigation | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps logged |
| Q7.b: timeout → MEASUREMENT_ERROR sampling bias | **VETO equivalent** | 15b87fb: synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` disambiguator |
| Q8: rollback_sim ✅ → ⚠️ partial | CHALLENGE | 15b87fb: TRACE_MATRIX_v1 § 2 downgraded with annotation |

## Specific re-audit questions

### RQ1 — Q2.b/e fix completeness (the two VETO items)
Trust Root manifest now has 20 entries including `src/main.rs` and `Cargo.lock`. The negative test confirmed: tampering with src/main.rs hash → evaluator panic at boot.

Vulnerabilities:
- (RQ1.a) Is the manifest now COMPLETE for the threat model you previously framed (passive tamper without recompile)? Audit the 20 entries — anything still missing?
- (RQ1.b) `genesis_payload.toml` itself is not self-hashed (chicken-and-egg, documented in TRACE_MATRIX_v1 § 4). Is this acceptable now that the call site (main.rs + evaluator entry) is hashed?
- (RQ1.c) Does evaluator.rs's call to `verify_trust_root` happen BEFORE any other state-modifying action (env_logger init, env::set_var for CLASSIFIER_VERSION)? Walk the order.

### RQ2 — Q7.b fix (timeout sampling bias)
Runner now emits a synthetic UNSOLVED row with `synthetic_timeout_or_crash: true` instead of dropping to MEASUREMENT_ERROR. compute_p0.py strict-completeness ensures every (problem, seed) pair is present in both modes.

Vulnerabilities:
- (RQ2.a) Treating timeout-as-UNSOLVED is the right call (you previously argued for this). Confirm the synthetic row's schema is correct: `progress: 0`, `solved: false`, `verified: false`, `synthetic_timeout_or_crash: true`. Does `compute_p0.py` `solved()` correctly read the new row as UNSOLVED?
- (RQ2.b) Are there OTHER failure modes that still cause data loss? E.g., evaluator emits PPUT_RESULT but the runner's grep fails; runner's python3 enrichment crashes; etc. Trace runner.sh:174-178 (the success path) for any silent skip.

### RQ3 — Q3.d strict-completeness fix
compute_p0.py now fails on missing tags / duplicates / seed mismatch / problem-set mismatch / N≠expected.

Vulnerabilities:
- (RQ3.a) When ANY strict-completeness check fails, compute_p0.py exits 1 (sys.exit with string). The runner propagates exit codes: 0 → freeze; 2 → ABORT; * → "investigate". Is exit 1 (data integrity failure) handled distinctly from exit 2 (ceiling violation)?
- (RQ3.b) The `expected_seed_set` and `expected_n_problems` are runtime parameters with PREREG defaults (31415, 2718, 144). Could a future caller pass wrong values and bypass? Inspect main() args.

### RQ4 — Q1.a equivalence claim
rollback_sim.rs header now says "narrow equivalence on (problem_id, seed, solved) only" with explicit non-equivalence list (cost / wall-clock / WAL / bus predicate / tx_count).

Vulnerabilities:
- (RQ4.a) Is the new header text accurate? Does it cover ALL non-equivalence dimensions (e.g., RNG state for downstream sampling, agent reputation accumulation in WalletTool)?
- (RQ4.b) The header references `compute_p0.py` honors the narrow equivalence by reading only `progress`. Verify compute_p0.py actually does this.

### RQ5 — New surfaces introduced by code-simplifier (commit 438a648)
Three changes: `Sha256::digest` one-shot, `matches!` macro for env-var read, dropped redundant `: PathBuf` annotation.

Vulnerabilities:
- (RQ5.a) `Sha256::digest(&bytes)` — semantic-equivalent to new()/update()/finalize()? sha2 0.10 docs say yes; verify.
- (RQ5.b) The shared `write_single_entry_repo` test helper in boot.rs — does it cover the same code paths as the original duplicated-test version? Or does the consolidation drop a test?

### RQ6 — Constitutional alignment (post-amendment)
The user amended Art. V.1.1/V.1.2/V.1.3 + Art. V.3 amendment log + FC3 node rename judgeAI→vetoAI. The fix activities (commit 15b87fb) all happened under the new model: ArchitectAI commit authority on non-constitution files.

Vulnerabilities:
- (RQ6.a) Did any fix in 15b87fb modify a file that should have required Veto-AI proposal review (i.e., a file with constitutional implications)? Trace the 7 modified files:
  - experiments/minif2f_v4/src/bin/evaluator.rs (added verify_trust_root call)
  - experiments/minif2f_v4/src/rollback_sim.rs (header rewrite)
  - experiments/minif2f_v4/tests/trust_root_immutability.rs (manifest assertion)
  - genesis_payload.toml (manifest expansion)
  - handover/alignment/TRACE_MATRIX_v1_2026-04-25.md (status downgrade + amendment note)
  - handover/preregistration/scripts/compute_p0.py (strict-completeness)
  - handover/preregistration/scripts/run_p0_calibration.sh (set -e + timeout + invocation)
  None of these touched constitution.md. Sound?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED with 576-run batch / FIX-THEN-PROCEED again / REDESIGN

If PASS: cite the specific evidence that Q2.b, Q2.e, Q7.b are now resolved AND that no new P0 defect was introduced.

---

# Appended source files (post-fix state)
"""

def slice_section(text, start_pat, end_pat):
    lines = text.splitlines()
    out, in_section = [], False
    for line in lines:
        if line.startswith(start_pat):
            in_section = True
        elif line.startswith(end_pat) and in_section:
            break
        if in_section:
            out.append(line)
    return "\n".join(out)

rollback_sim = (ROOT / "experiments/minif2f_v4/src/rollback_sim.rs").read_text()
boot_rs = (ROOT / "src/boot.rs").read_text()
main_rs = (ROOT / "src/main.rs").read_text()
genesis = (ROOT / "genesis_payload.toml").read_text()
runner_sh = (ROOT / "handover/preregistration/scripts/run_p0_calibration.sh").read_text()
compute_py = (ROOT / "handover/preregistration/scripts/compute_p0.py").read_text()
trace_v1 = (ROOT / "handover/alignment/TRACE_MATRIX_v1_2026-04-25.md").read_text()
trust_test = (ROOT / "experiments/minif2f_v4/tests/trust_root_immutability.rs").read_text()
constitution = (ROOT / "constitution.md").read_text()
art_v = "\n".join(constitution.splitlines()[533:720])

evaluator_diff = subprocess.run(
    ["git", "-C", str(ROOT), "diff", "fa93943..HEAD", "--", "experiments/minif2f_v4/src/bin/evaluator.rs"],
    capture_output=True, text=True, check=True
).stdout

# Smoke evidence
smoke_dir = ROOT / "experiments/minif2f_v4/logs"
control_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_control.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_control.jsonl")) else "(no control smoke yet)"
treatment_jsonl = sorted(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl"))[-1].read_text() if list(smoke_dir.glob("p0_smoke_hard_*_treatment.jsonl")) else "(no treatment smoke yet)"

full_prompt = (
    brief +
    f"\n## experiments/minif2f_v4/src/rollback_sim.rs (post code-simplifier)\n\n```rust\n{rollback_sim}\n```\n" +
    f"\n## src/boot.rs (post code-simplifier)\n\n```rust\n{boot_rs}\n```\n" +
    f"\n## src/main.rs\n\n```rust\n{main_rs}\n```\n" +
    f"\n## genesis_payload.toml (20 entries)\n\n```toml\n{genesis}\n```\n" +
    f"\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n{runner_sh}\n```\n" +
    f"\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n{compute_py}\n```\n" +
    f"\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff fa93943..HEAD)\n\n```diff\n{evaluator_diff}\n```\n" +
    f"\n## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md (post-fix)\n\n{trace_v1}\n" +
    f"\n---\n\n## constitution.md Art. V (post-amendment)\n\n{art_v}\n" +
    f"\n---\n\n## experiments/minif2f_v4/tests/trust_root_immutability.rs (post-fix)\n\n```rust\n{trust_test}\n```\n" +
    f"\n## smoke evidence — control row (aime_1983_p2)\n\n```json\n{control_jsonl}\n```\n" +
    f"\n## smoke evidence — treatment row (aime_1983_p2)\n\n```json\n{treatment_jsonl}\n```\n" +
    "\n---\n\nNow give your INDEPENDENT re-audit. Cite §/file:line for every finding."
)

print(f"[gemini round4_audit] prompt size: {len(full_prompt):,} chars", file=sys.stderr)

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
    print(f"[gemini round4_audit] HTTP {e.code}: {e.read().decode(errors='replace')[:2000]}", file=sys.stderr)
    sys.exit(1)

elapsed = time.time() - t0
print(f"[gemini round4_audit] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_B7_EXTRA_ROUND4_AUDIT_2026-04-25.md"
header = (f"# Gemini PPUT-CCL Phase B B7-extra ROUND-4 RE-AUDIT (post-VETO-fixes)\n"
          f"**Date**: 2026-04-25\n"
          f"**Predecessor**: GEMINI_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)\n"
          f"**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)\n"
          f"**Test baseline**: 187/187 PASS + 20 ignored\n"
          f"**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true\n"
          f"**Negative test**: tamper Trust Root → evaluator panic at boot ✓\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(full_prompt):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini round4_audit] saved: {out}")
