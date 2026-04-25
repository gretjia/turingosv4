#!/usr/bin/env bash
# Codex ROUND-3 RE-AUDIT — Phase B B7-extra after dual-VETO fixes (commit 15b87fb +
# code-simplifier 438a648 + constitution amendment c061450 + audit packet df77b0a).
#
# Independent of Gemini. Output: CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md
set -euo pipefail

ROOT="/home/zephryj/projects/turingosv4"
OUT="${ROOT}/handover/audits/CODEX_B7_EXTRA_ROUND3_AUDIT_2026-04-25.md"
TMP_PROMPT="$(mktemp /tmp/b7_extra_codex_round3_audit.XXXXXX.md)"
trap 'rm -f "$TMP_PROMPT"' EXIT

cat > "$TMP_PROMPT" <<'BRIEF_EOF'
# Codex ROUND-3 RE-AUDIT — Phase B B7-extra (after VETO fixes)

**Role**: skeptical adversarial reviewer. Independent of Gemini. CLAUDE.md "Audit Standard": VETO > CHALLENGE > PASS; conservative reading wins on disagreement.

**Mandate**: this is the THIRD audit pass.
- **Round 1** (CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md): VETO with B1+B2+B3 top blockers. Fixed in commit `15b87fb`.
- **Round 2** (CODEX_B7_EXTRA_REAUDIT_2026-04-25.md): YOU returned VETO again, catching a defect introduced by the round-1 fix: synthetic-UNSOLVED-on-any-non-zero-exit silently absorbed TRUST_ROOT_TAMPERED panics into "valid" calibration data, neutralizing B1. Fixed in commit `1df1f62`.
- **Round 3 (this audit)**: determine whether the round-2 fix resolves your previous VETO AND whether it introduces NEW defects. Specifically scrutinize the new crash-vs-timeout discrimination logic and the boot preflight.

**Round-2 fix summary (commit `1df1f62`, on top of `15b87fb`+`438a648`)**:
1. Crash discrimination in runner loop:
   - exit 124 (timeout) → synthetic UNSOLVED row (only this case emits synthetic data)
   - exit non-124, non-0 (Rust panic 101, segfault 139, OOM 137, etc.) → ABORT BATCH with exit 3 (TRUST_ROOT_TAMPERED detected via stderr grep) or exit 4 (other crash). NO synthetic row emitted; partial calibration is forfeited.
2. Pre-batch evaluator boot preflight: invokes `evaluator /nonexistent.lean`; if stderr contains TRUST_ROOT_TAMPERED, runner exits 2 at preflight stage — zero API spend wasted.
3. Synthetic row schema: `golden_path_token_count: 0` added (was missing per your CHALLENGE 2).
4. TRACE_MATRIX_v1 manifest size doc-drift fixed (16 → 20 with composition breakdown).

**Verification done**:
- 187/187 cargo test --workspace PASS + 20 ignored
- Tamper test: Cargo.lock entry tampered → runner aborts at preflight with TRUST_ROOT_TAMPERED diagnostic (exit 2). Restored. Boot tests re-pass.
- Happy path smoke: --smoke (mathd_algebra_107, 4 runs, 40s) all SOLVED tx=1; jsonl V2 + calibration tags + MODEL_SNAPSHOT all stamped; no abort.

**Decision rule**:
- If the round-2 fix RESOLVES your prior VETO (no silent absorption path) AND introduces no new P0: **PASS** with HIGH conviction → batch authorized.
- If any new P0 defect or the round-2 fix is incomplete: **CHALLENGE** with specific items.
- If a deeper architectural issue surfaces: **VETO** with explicit "not safe to launch 576-run batch".

**Constitutional amendments since last audit** (commit `c061450`, sudo-authorized):
- Art. V.1.1: human sudo applies *only* to constitution.md (not other Trust Root files)
- Art. V.1.2: ArchitectAI has commit authority on non-constitution files (no sudo needed post-Veto-AI PASS)
- Art. V.1.3: JudgeAI renamed to **Veto-AI**; output domain narrowed to {PASS, VETO} only — explicit white-list exclusion of subjective quality judgments
- FC3 mermaid: judgeAI[JudgeAI] → vetoAI[Veto-AI]
- Art. V.3 (NEW): Constitutional Amendment Log

**State of the code (re-audit baseline)**:
- 187/187 cargo test --workspace PASS + 20 deferred-stub `#[ignore]`
- Hard-problem smoke (aime_1983_p2): control SOLVED tx=15 (~2.5 min); treatment UNSOLVED tx=50 with `synthetic_short_circuit=true` ✓
- **Negative test**: tampered genesis_payload.toml manifest entry → evaluator panicked at startup with `TRUST_ROOT_TAMPERED` (B1 fix verified end-to-end)
- Trust Root manifest 16 → 20 entries (audit additions: src/main.rs, Cargo.lock, run_p0_calibration.sh, compute_p0.py)

## Fix summary by previous finding

| Previous finding | Severity | Fix commit + location |
|---|---|---|
| B1: evaluator doesn't call verify_trust_root | **VETO** | 15b87fb: evaluator.rs:163-183 — `verify_trust_root` called at top of `async fn main`; panic on Err. Negative-test verified end-to-end. |
| B2: compute_p0 silently computes on incomplete subset | **VETO** | 15b87fb: compute_p0.py strict-completeness — fails loudly on missing tags / duplicates / seed mismatch / N≠144. Denominator pinned to PREREG-frozen 144. |
| B3: p_0 ceiling abort not enforced | **VETO** | 15b87fb: run_p0_calibration.sh end-of-batch invokes compute_p0.py with --out-json; exit 0 → freeze authorized; exit 2 → ABORT message; other → investigate. |
| Q1.b: info!→warn! at short-circuit log | CHALLENGE | 15b87fb: evaluator.rs short-circuit log now `warn!` with cost-asymmetry warning |
| Q1.a: equivalence claim wrong | CHALLENGE | 15b87fb: rollback_sim.rs header rewritten — narrow equivalence on (problem, seed, solved) only; explicit non-equivalence list |
| Q2.b: src/main.rs not in manifest | **VETO (Gemini)** | 15b87fb: added to genesis_payload.toml |
| Q2.e: Cargo.lock not in manifest | **VETO (Gemini)** | 15b87fb: added |
| Q2: scripts not in Trust Root | CHALLENGE | 15b87fb: run_p0_calibration.sh + compute_p0.py both added to manifest |
| Q3 (Codex): solved() reads progress_verified not v2 progress | CHALLENGE | 15b87fb: compute_p0.py reads `progress` field |
| Q4 + Q8: TRACE_MATRIX rollback_sim ✅ → ⚠️ | CHALLENGE | 15b87fb: TRACE_MATRIX_v1 § 2 downgraded to ⚠️ partial with annotation |
| Q7.a: API drift detection | CHALLENGE | 15b87fb: MODEL_SNAPSHOT (active_model@git_sha[:12]) + BUILD_SHA + canary timestamps |
| Q7.b: timeout drops to MEASUREMENT_ERROR | **VETO equivalent (sampling bias)** | 15b87fb: synthetic UNSOLVED row emitted with `synthetic_timeout_or_crash: true` disambiguator |
| Q7.c: no resume mode | CHALLENGE (deferred) | Not addressed in this round — 576 runs are 8h, full restart cost is bounded; resume adds complexity not justified pre-batch |
| Q3.e: runner script seeds not in Trust Root | CHALLENGE | Resolved transitively: runner.sh now in Trust Root (Q2 fix) so seed array is hash-protected |
| Q6.c: yellow-band rule for borderline p_0 | CHALLENGE (open) | Codex previously said "don't post-hoc retune" — accepted; no yellow band defined; passive accept on (0, 0.10] |

## Specific re-audit questions

### RQ1 — B1 fix completeness
`evaluator.rs:163-183` calls `turingosv4::boot::verify_trust_root(&repo_root)` at the top of `async fn main`. The repo_root is resolved from `env!("CARGO_MANIFEST_DIR")` (= `experiments/minif2f_v4`) joined with `..` × 2 + canonicalize.

Vulnerabilities:
- (RQ1.a) Is the canonicalize() correct under all deployment scenarios (e.g., binary copied to a different directory)? PREREG runs the binary from the project tree; deployment-elsewhere is out of scope, agreed?
- (RQ1.b) The verify_trust_root call is BEFORE env_logger::init(). Does that matter for diagnostics? (Panic message goes to stderr regardless.)
- (RQ1.c) Could a future evaluator addition place ANY logic before verify_trust_root that would compromise the order? Defensive structure?
- (RQ1.d) The negative-test (tampered manifest → panic) was verified by the implementer. Is there a corresponding automated test (cargo test) that catches accidental removal of the verify_trust_root call? Do you require one?

### RQ2 — B2 fix completeness
`compute_p0.py` now: (i) fails on missing `calibration_problem_id` / `calibration_seed`; (ii) fails on duplicate (problem, seed); (iii) fails on seed set ≠ {31415, 2718}; (iv) fails on control vs treatment problem-set mismatch; (v) fails on row count ≠ expected_n_problems × len(seeds); (vi) denominator = expected_n_problems (144) not observed.

Vulnerabilities:
- (RQ2.a) Synthetic timeout/crash rows carry `synthetic_timeout_or_crash: true` + `progress: 0`. They satisfy strict completeness. Is treating timeout-as-UNSOLVED defensible vs treating timeout-as-data-loss? PREREG § 5.5 doesn't specify, but Gemini's prior CHALLENGE said timeout IS a valid UNSOLVED outcome. Confirm.
- (RQ2.b) Strict-completeness failure modes call `sys.exit("ERROR: ...")`. The runner has `set +e` around compute_p0 invocation specifically (line ~245), captures EXIT, then propagates. Is exit code 1 (sys.exit with string) vs 2 (ceiling) handled distinctly?

### RQ3 — B3 fix completeness
runner: `set +e; python3 compute_p0.py --out-json $P0_JSON; P0_EXIT=$?; set -e; case $P0_EXIT in 0) freeze; 2) ABORT; *) error;`

Vulnerabilities:
- (RQ3.a) When P0_EXIT=2 (ceiling violated), the runner prints "p_0 EXCEEDS ceiling" and exits 2. Does it ALSO ensure the calibration jsonl files are NOT silently absorbed into Trust Root? Manifest update is a separate manual ArchitectAI step — is that documented?
- (RQ3.b) When P0_EXIT=other (e.g., compute_p0 itself crashed), the runner exits with that code. Does this preserve the calibration jsonl for diagnosis (yes — never deleted), and is the operator told NOT to re-run blindly?

### RQ4 — Synthetic timeout row schema
`emit_synthetic_unsolved` at runner.sh:77-115 emits a JSON object with `synthetic_timeout_or_crash: true` + all v2 RunAggregate fields (default values).

Vulnerabilities:
- (RQ4.a) The synthetic row schema fields match v2 RunAggregate. Is there a field MISSING that downstream tooling would reasonably expect (e.g., `verified` field — present, set false; `binary_sha256` — empty string; etc.)?
- (RQ4.b) The synthetic row's `synthetic_short_circuit` is implicitly absent (None). Any conflict with `synthetic_timeout_or_crash`? Both can't be true simultaneously by construction (timeout means evaluator didn't return at all).

### RQ5 — New surfaces introduced by fix
- (RQ5.a) `MODEL_SNAPSHOT` env now drift-detection-stamped at runner level. Evaluator was already reading `MODEL_SNAPSHOT` env before B7-extra fix (jsonl_schema.rs:112). The runner's `${ACTIVE_MODEL}@${GIT_SHA:0:12}${GIT_DIRTY}` format works for downstream pairing? `-dirty` suffix when working tree has unstaged changes.
- (RQ5.b) `BUILD_SHA` env: same. Is `git rev-parse HEAD` the right baseline (vs e.g., `git describe --always`)?
- (RQ5.c) Trust Root manifest now 20 entries. Is this growing unbounded? PREREG § 1.8 is meant to be relatively stable.

### RQ6 — Did the fix introduce code-simplifier regressions?
A code-simplifier pass (commit `438a648`) ran AFTER the audit fix commit (`15b87fb`) but BEFORE this re-audit. Modifications: `Sha256::digest` one-shot in boot.rs, `matches!` macro in rollback_sim.rs, dropped redundant `: PathBuf` annotation in evaluator.rs.

Vulnerabilities:
- (RQ6.a) `Sha256::digest(&bytes)` — semantic-equivalent to `Sha256::new() / update(&bytes) / finalize()`? Does it handle large inputs (genesis_payload.toml: ~3KB; constitution.md: ~25KB; cases/MANIFEST.sha256: ~5KB) correctly without intermediate state issues?
- (RQ6.b) `hex_lower` now uses `std::fmt::Write` + `write!(out, "{b:02x}").unwrap()`. The `.unwrap()` on a `String` write — write to a String never errs by spec (infallible). Sound.
- (RQ6.c) `rollback_simulation_enabled()` reads env via `matches!(... .as_deref(), Ok("1"))`. Earlier was `... .ok().as_deref() == Some("1")`. Equivalent?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED with 576-run batch / FIX-THEN-PROCEED again / REDESIGN

If PASS: cite the specific evidence that the three previous top blockers (B1+B2+B3) are now resolved, AND that no new P0 defect was introduced by the fix.

---

# Appended source files (post-fix state)

BRIEF_EOF

# Source files (post-fix)
printf '\n## experiments/minif2f_v4/src/rollback_sim.rs (post code-simplifier)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/experiments/minif2f_v4/src/rollback_sim.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## src/boot.rs (post code-simplifier)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/boot.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## src/main.rs\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/src/main.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## genesis_payload.toml (20 entries)\n\n```toml\n' >> "$TMP_PROMPT"
cat "${ROOT}/genesis_payload.toml" >> "$TMP_PROMPT"

printf '\n```\n\n## handover/preregistration/scripts/run_p0_calibration.sh (audit-fixed)\n\n```bash\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/scripts/run_p0_calibration.sh" >> "$TMP_PROMPT"

printf '\n```\n\n## handover/preregistration/scripts/compute_p0.py (audit-fixed)\n\n```python\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/preregistration/scripts/compute_p0.py" >> "$TMP_PROMPT"

printf '\n```\n\n## evaluator.rs B7-extra wiring + verify_trust_root call (post-fix diff)\n\n```diff\n' >> "$TMP_PROMPT"
git -C "${ROOT}" diff fa93943..HEAD -- experiments/minif2f_v4/src/bin/evaluator.rs >> "$TMP_PROMPT"

printf '\n```\n\n## handover/alignment/TRACE_MATRIX_v1_2026-04-25.md (post-fix)\n\n' >> "$TMP_PROMPT"
cat "${ROOT}/handover/alignment/TRACE_MATRIX_v1_2026-04-25.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## constitution.md Art. V (post-amendment)\n\n' >> "$TMP_PROMPT"
sed -n '534,720p' "${ROOT}/constitution.md" >> "$TMP_PROMPT"

printf '\n\n---\n\n## experiments/minif2f_v4/tests/trust_root_immutability.rs (post-fix)\n\n```rust\n' >> "$TMP_PROMPT"
cat "${ROOT}/experiments/minif2f_v4/tests/trust_root_immutability.rs" >> "$TMP_PROMPT"

printf '\n```\n\n## smoke evidence — control row (aime_1983_p2)\n\n```json\n' >> "$TMP_PROMPT"
ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_control.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"

printf '\n```\n\n## smoke evidence — treatment row (aime_1983_p2)\n\n```json\n' >> "$TMP_PROMPT"
ls -t "${ROOT}/experiments/minif2f_v4/logs/p0_smoke_hard_"*"_treatment.jsonl" 2>/dev/null | head -1 | xargs cat >> "$TMP_PROMPT"

printf '\n```\n\n---\n\nNow give your INDEPENDENT re-audit. Cite §/file:line for every finding.\n' >> "$TMP_PROMPT"

prompt_size=$(wc -c < "$TMP_PROMPT")
echo "[codex round3_audit] prompt size: ${prompt_size} chars" >&2

t0=$(date +%s)
{
  printf '# Codex PPUT-CCL Phase B B7-extra ROUND-3 RE-AUDIT (post-VETO-fixes)\n'
  printf '**Date**: 2026-04-25\n'
  printf '**Predecessor**: CODEX_B7_EXTRA_PREBATCH_AUDIT_2026-04-25.md (verdict VETO)\n'
  printf '**Fixes commits**: 15b87fb (Phase A+B+C) + 438a648 (code-simplifier) + c061450 (constitution) + df77b0a (audit packet record)\n'
  printf '**Test baseline**: 187/187 PASS + 20 ignored\n'
  printf '**Smoke**: control SOLVED tx=15, treatment UNSOLVED tx=50 with synthetic_short_circuit=true\n'
  printf '**Negative test**: tamper Trust Root → evaluator panic at boot ✓\n'
  printf '**Prompt size**: %s chars\n\n---\n\n' "$prompt_size"
  codex exec --skip-git-repo-check < "$TMP_PROMPT" 2>&1
} > "$OUT"
t1=$(date +%s)
elapsed=$((t1 - t0))
echo "[codex round3_audit] API returned in ${elapsed}s" >&2
echo "[codex round3_audit] saved: $OUT" >&2
