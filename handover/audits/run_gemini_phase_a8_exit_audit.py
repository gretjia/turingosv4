#!/usr/bin/env python3
"""Gemini Phase A → B exit audit — covers A0–A7 deliverables. Independent of Codex."""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [
    ROOT / ".env",
    Path("/home/zephryj/projects/turingosv3/.env"),
]

# A8e10 fix O1 (Codex R9#1): A8_AUDIT_ROUND is REQUIRED. Fail fast
# BEFORE the API call so we never spend money on an unattended re-run
# that would silently overwrite a prior transcript. Round suffix
# becomes the file suffix in handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_<round>.md.
round_label = os.environ.get("A8_AUDIT_ROUND")
if not round_label:
    print(
        "[run_gemini_a8_exit] error: A8_AUDIT_ROUND env var is required\n"
        "    usage: A8_AUDIT_ROUND=R<n> python3 handover/audits/run_gemini_phase_a8_exit_audit.py",
        file=sys.stderr,
    )
    sys.exit(2)
_out_path = ROOT / f"handover/audits/GEMINI_PHASE_A8_EXIT_AUDIT_2026-04-26_{round_label}.md"
if _out_path.exists():
    print(
        f"[run_gemini_a8_exit] error: {_out_path} already exists; refusing to overwrite\n"
        f"    (prior audit transcripts are append-only governance artifacts;\n"
        f"    delete the file explicitly if you really intend to re-run round {round_label})",
        file=sys.stderr,
    )
    sys.exit(2)


def load_env() -> dict[str, str]:
    """Read GEMINI_API_KEY (and others) without echoing values."""
    env: dict[str, str] = {}
    for fp in ENV_FILES:
        if not fp.exists():
            continue
        for line in fp.read_text().splitlines():
            if "=" in line and not line.strip().startswith("#"):
                k, v = line.split("=", 1)
                env.setdefault(k.strip(), v.strip().strip('"').strip("'"))
    return env


env = load_env()
if "GEMINI_API_KEY" not in env:
    print("[gemini a8 exit] GEMINI_API_KEY not found in .env files", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini Phase A → B Exit Audit (PPUT-CCL arc)

**Role**: skeptical adversarial reviewer. Independent of Codex. Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: Phase A is pre-flight (days 1–3 of the 30-day arc). 8 atoms (A0a–e + A1–A7) must be auditable as a unit before Phase B (kernel instrumentation + PPUT accounting) is authorized to start. PREREG_PPUT_CCL_2026-04-26.md (round-4 PASS/PASS, frozen) + PREREG_AMENDMENT_p0_defer_2026-04-25.md (Trust Root entry 25) are the contracts.

The packet below is self-contained. The Phase A0 exit audit (CHALLENGE/CHALLENGE → 7 fixes) is the precedent for rigor.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to Phase B / FIX-THEN-PROCEED / REDESIGN

Cite §/file:line for every finding. Be specific about which atom and which line.

---

"""

# ── Packet + history (A8e7 split) ──
packet_path = ROOT / "handover/audits/A8_EXIT_PACKET_2026-04-26.md"
history_path = ROOT / "handover/audits/A8_AUDIT_HISTORY_2026-04-26.md"
brief += packet_path.read_text()
brief += "\n\n---\n\n# Audit history (append-only chronology)\n\n"
brief += history_path.read_text()

# ── Source files appended ──
brief += "\n\n---\n\n# Appended source files\n\n"

source_paths = [
    "handover/preregistration/PREREG_AMENDMENT_p0_defer_2026-04-25.md",
    "experiments/minif2f_v4/src/agent_models.rs",
    "experiments/minif2f_v4/src/budget_regime.rs",
    "experiments/minif2f_v4/src/fc_trace.rs",
    "experiments/minif2f_v4/src/run_id.rs",
    "experiments/minif2f_v4/src/jsonl_schema.rs",
    "experiments/minif2f_v4/src/bin/evaluator.rs",
    "src/drivers/llm_proxy.py",
    "scripts/smoke_siliconflow.sh",
    "scripts/_smoke_siliconflow.py",
    "scripts/test_llm_proxy.py",
    "experiments/minif2f_v4/tests/fc_trace_smoke.rs",
    "experiments/minif2f_v4/tests/trust_root_immutability.rs",
    "experiments/minif2f_v4/tests/llm_proxy_python_conformance.rs",
    "experiments/minif2f_v4/examples/fc_trace_emit_one.rs",
    "handover/alignment/TRACE_MATRIX_v2_2026-04-25.md",
    "genesis_payload.toml",
]

LANG = {".rs": "rust", ".py": "python", ".sh": "bash", ".toml": "toml", ".yaml": "yaml", ".yml": "yaml"}
for rel in source_paths:
    fp = ROOT / rel
    if not fp.exists():
        continue
    lang = LANG.get(fp.suffix, "")
    brief += f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"

brief += "\n---\n\nGive your INDEPENDENT Phase A → B exit audit. Cite §/file:line for every finding.\n"

print(f"[gemini a8 exit] prompt size: {len(brief):,} chars", file=sys.stderr)

# ── Call ──
url = (
    "https://generativelanguage.googleapis.com/v1beta/models/"
    f"gemini-2.5-pro:generateContent?key={env['GEMINI_API_KEY']}"
)
body = json.dumps({
    "contents": [{"parts": [{"text": brief}]}],
    "generationConfig": {"temperature": 0.2, "maxOutputTokens": 16384},
}).encode()

t0 = time.time()
req = urllib.request.Request(
    url, data=body, headers={"Content-Type": "application/json"}, method="POST"
)
try:
    with urllib.request.urlopen(req, timeout=900) as resp:
        data = json.loads(resp.read())
except Exception as e:
    print(f"[gemini a8 exit] error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini a8 exit] API returned in {elapsed:.1f}s", file=sys.stderr)

text = data["candidates"][0]["content"]["parts"][0]["text"]
# A8e10 fix O1: round_label + _out_path were validated at script start
# (before API call) so we don't burn API budget on a misconfigured run.
out = _out_path
header = (
    f"# Gemini Phase A → B Exit Audit (PPUT-CCL arc)\n"
    f"**Round**: {round_label}\n"
    f"**Date**: 2026-04-26\n"
    f"**Reference**: see `handover/audits/A8_AUDIT_HISTORY_2026-04-26.md` "
    f"for full chronology including atom commit chain + per-round "
    f"verdicts/fixes.\n"
    f"**Test baseline**: 265 PASS + 29 ignored + 0 failed (Rust); "
    f"16/16 PASS (Python proxy tests)\n"
    f"**Trust Root**: 35-entry manifest verifies clean\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n\n---\n\n"
)
out.write_text(header + text)
print(f"[gemini a8 exit] saved: {out}")
