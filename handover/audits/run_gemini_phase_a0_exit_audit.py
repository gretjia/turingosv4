#!/usr/bin/env python3
"""Gemini Phase A0 exit audit — harness modernization gate. Independent of Codex."""
import json, sys, time, urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
V3_ENV = Path("/home/zephryj/projects/turingosv3/.env")

env = {}
for line in V3_ENV.read_text().splitlines():
    if "=" in line and not line.strip().startswith("#"):
        k, v = line.split("=", 1)
        env[k.strip()] = v.strip().strip('"').strip("'")

brief = """# Gemini Phase A0 Exit Audit — Harness Modernization

**Role**: skeptical adversarial reviewer. Independent of Codex. Decision rule: PASS → Phase A1-A8 engineering atoms can begin; CHALLENGE → fix in same cycle; VETO → A0 redesign.

**Mandate**: Phase A0 is the harness modernization that PRECEDES Phase A engineering work. Per the auto-research execution plan: "harness 不修复, Phase A 之后的 atoms 也会有 silent drift".

**A0 deliverables (4 commits)**:
- `2e7f75a` A0a: 4 new rules R-014/R-015/R-018/R-019 + judge.sh constitution-special-case + R-016 fc_trace_in_commit hook
- `d8950ee` A0b: tests/fc_alignment_conformance.rs witness battery (17 PASS + 9 ignored stubs)
- `2a65339` A0c: 5 new cases C-071..C-075 (constitution amendment process / Veto-AI scope / ArchitectAI commit authority / FC-first / DO-178C tool qualification)
- `e94e1b9` A0d: TRACE_MATRIX_v2 + Trust Root manifest 20 → 24 entries

**Constitutional context** (Art. V amendment 2026-04-25, commit c061450 — sudo-authorized): Art. V.1.1 sudo only constitution.md / Art. V.1.2 ArchitectAI commit authority on non-constitution / Art. V.1.3 Veto-AI {PASS, VETO} only.

**Verification done by ArchitectAI**:
- 4 smoke tests on judge.sh confirm: R-018 BLOCKS constitution.md edit / R-016 WARNS on git commit without FC-trace / silent pass on FC-traced commit / non-constitution .md still skipped
- cargo test --workspace: 204 passed (was 187, +17 fc_alignment_conformance) + 29 ignored
- boot::tests::verify_trust_root_passes_on_intact_repo: PASS at 24-entry manifest

## Specific questions

### Q1 — Rule completeness
4 new rules cover: Trust Root manifest drift (R-014 warn), TRACE_MATRIX pub-symbol freshness (R-015 warn), constitution.md sudo gate (R-018 BLOCK), model-snapshot canonical labeling (R-019 warn). Plus R-016 fc_trace_in_commit (in judge.sh).

- (Q1.a) Other B7+ surfaces still uncovered? E.g., synthetic_short_circuit field discipline, AGENT_MODELS env var sanity, BUDGET_MODE values?
- (Q1.b) R-014 fires on EVERY *.rs edit. Acceptable noise or scope-narrow?
- (Q1.c) R-018 BLOCK depends on judge.sh special-case BEFORE the *.md skip-list. Path-routing logic (lines 50-67) airtight, or could relative path / symlink bypass?
- (Q1.d) R-016 greps the COMMAND for `FC-trace:`. `git commit -F /tmp/msg` would miss. Limitation worth fixing now?

### Q2 — Conformance battery integrity
17 witnesses + 9 ignored stubs.

- (Q2.a) Are witnesses TRULY witnesses (would fail compile if symbol removed) or just `type_name::<T>()` calls?
- (Q2.b) Ignored stubs use `panic!()` body. Right pattern?
- (Q2.c) Cross-crate symbols (Lean4Oracle in minif2f_v4) need second fc_alignment_conformance.rs in `experiments/minif2f_v4/tests/`. A0b ignored. Acceptable deferral?
- (Q2.d) Binary-only symbols (run_swarm/run_oneshot) ignored with rationale. Should A1-A8 refactor evaluator to expose for testing?

### Q3 — Case-law sediment
5 cases C-071..C-075.

- (Q3.a) TRUE precedents (binding for future) or session-summary docs?
- (Q3.b) C-073 declares ArchitectAI scope "ALL files EXCEPT constitution.md" — overreach vs Art. V.1.2?
- (Q3.c) C-074 overlaps memory feedback_fc_first_problem_handling. Duplication acceptable?
- (Q3.d) Case cross-references form DAG. Circular reasoning hazard?

### Q4 — Trust Root manifest expansion 20 → 24
4 new entries: rules/MANIFEST.sha256 + rules/engine.py + .claude/hooks/judge.sh + tests/fc_alignment_conformance.rs.

- (Q4.a) Should rules/SCHEMA.yaml also be in Trust Root?
- (Q4.b) `.claude/hooks/judge.sh` included; build-check.sh + session-end.sh excluded — gap?
- (Q4.c) Should ALL tests be in Trust Root, not just fc_alignment_conformance?
- (Q4.d) TRACE_MATRIX_v?.md docs themselves NOT in Trust Root (self-reference). Acceptable?

### Q5 — Constitutional alignment (meta-check)
- (Q5.a) Did A0a-d touch constitution.md? Trace each commit.
- (Q5.b) Did each A0 commit message contain `FC-trace:`?
- (Q5.c) Regression test that asserts A0 itself can't be silently rolled back?

---

End with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase A engineering atoms / FIX-THEN-PROCEED / REDESIGN A0
"""

# Source files (compact)
def read(p):
    return (ROOT / p).read_text()

source = {
    "rules/active/R-014_trust_root_manifest_drift.yaml": read("rules/active/R-014_trust_root_manifest_drift.yaml"),
    "rules/active/R-015_trace_matrix_pub_symbol.yaml": read("rules/active/R-015_trace_matrix_pub_symbol.yaml"),
    "rules/active/R-018_constitution_amendment_sudo.yaml": read("rules/active/R-018_constitution_amendment_sudo.yaml"),
    "rules/active/R-019_model_snapshot_canonical.yaml": read("rules/active/R-019_model_snapshot_canonical.yaml"),
    ".claude/hooks/judge.sh": read(".claude/hooks/judge.sh"),
    "tests/fc_alignment_conformance.rs": read("tests/fc_alignment_conformance.rs"),
    "handover/alignment/TRACE_MATRIX_v2_2026-04-25.md": read("handover/alignment/TRACE_MATRIX_v2_2026-04-25.md"),
    "genesis_payload.toml": read("genesis_payload.toml"),
}
for cid in ["071", "072", "073", "074", "075"]:
    fp = next(ROOT.glob(f"cases/C-{cid}_*.yaml"))
    source[str(fp.relative_to(ROOT))] = fp.read_text()

appended = brief + "\n\n# Appended source files\n\n"
for path, content in source.items():
    fence = "yaml" if path.endswith(".yaml") else ("rust" if path.endswith(".rs") else ("bash" if path.endswith(".sh") else ("toml" if path.endswith(".toml") else "")))
    appended += f"\n## {path}\n\n```{fence}\n{content}\n```\n"
appended += "\n---\n\nNow give your INDEPENDENT Phase A0 exit audit. Cite §/file:line for every finding."

print(f"[gemini a0 exit] prompt size: {len(appended):,} chars", file=sys.stderr)

url = f"https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
body = json.dumps({
    "contents": [{"parts": [{"text": appended}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()

t0 = time.time()
req = urllib.request.Request(url, data=body, headers={"Content-Type": "application/json"}, method="POST")
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini a0 exit] error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini a0 exit] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
out = ROOT / "handover/audits/GEMINI_PHASE_A0_EXIT_AUDIT_2026-04-25.md"
header = (f"# Gemini Phase A0 Exit Audit (Harness Modernization)\n"
          f"**Date**: 2026-04-25\n"
          f"**Commits**: 2e7f75a / d8950ee / 2a65339 / e94e1b9\n"
          f"**Test baseline**: 204 PASS + 29 ignored\n"
          f"**Trust Root**: 24-entry manifest verifies clean\n"
          f"**Elapsed**: {elapsed:.1f}s\n"
          f"**Prompt size**: {len(appended):,} chars\n\n---\n\n")
out.write_text(header + text)
print(f"[gemini a0 exit] saved: {out}")
