#!/usr/bin/env python3
"""TB-14 Atom 6 production wire-swap — Gemini Class 3 ship audit.

Architectural strategic angle (parallel to Codex implementation-paranoid).
Per memory feedback_dual_audit + feedback_dual_audit_conflict.
"""
import json
import os
import sys
import time
import urllib.request
from pathlib import Path

ROOT = Path("/home/zephryj/projects/turingosv4")
ENV_FILES = [ROOT / ".env", Path("/home/zephryj/projects/turingosv3/.env")]
ROUND = os.environ.get("TB14_AUDIT_ROUND", "R1")
OUT = ROOT / f"handover/audits/GEMINI_TB_14_SHIP_AUDIT_2026-05-03_{ROUND}.md"

if OUT.exists():
    print(f"[gemini tb-14] error: {OUT} already exists; refusing to overwrite", file=sys.stderr)
    sys.exit(2)


def load_env():
    env = {}
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
    print("[gemini tb-14] GEMINI_API_KEY not in .env (checked v4 + v3)", file=sys.stderr)
    sys.exit(2)

# ── Brief ──
brief = """# Gemini TB-14 Atom 6 Ship Audit — production wire-swap (Class 3 architectural strategic review)

**Role**: skeptical adversarial reviewer, architectural strategic angle. Independent of Codex (parallel, implementation-paranoid). Conservative merge rule (memory `feedback_dual_audit_conflict`): VETO > CHALLENGE > PASS.

**Mandate**: TB-14 Atom 6 of TURINGOS v4 implements per architect 2026-05-03 ruling §5: production wire-swap from legacy decimal-float CPMM scaffolding to integer-rational `state::compute_price_index` + `state::compute_mask_set` derived views. Closes `OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03`. Class 3 (production code path; STEP_B restricted files src/kernel.rs + src/bus.rs touched).

The architect's TB-14 forbidden list (§5.6) explicitly bans: market trading / price-based settlement / parent deletion / Goodhart leak / **no f64 anywhere in TB-14 code surface** / AMM / DPMM / price-as-oracle language.

The architect's TB-14 halt-trigger list (§5.7 — instant stop, no round-2):
1. price affects predicate result
2. price changes L4/L4.E decision
3. parent node deleted/removed from ChainTape
4. f64 introduced
5. price computed for zero-liquidity node as non-None
6. unresolved challenge masked as safe

All 6 are verified GREEN at HEAD `38412bf` per `cargo test --test tb_14_halt_triggers`.

**Architect §5 + charter §3 Atom 6 mandated audit questions** (these are YOUR primary mandate):

1. **CR-14.1**: Does the price signal influence predicate gates? Trace `src/state/sequencer.rs` body for any reference to `compute_price_index | compute_mask_set | NodeMarketEntry | RationalPrice | BoltzmannMaskPolicy`. (Halt-trigger #1 is the file-level fence.)

2. **CR-14.2**: Does the price signal affect L4 / L4.E classification? Trace `src/state/sequencer.rs` `use` block — must be permanently TB-14-import-free even after Atom 6's bus.rs snapshot wire-swap legitimately imports TB-14 types. (Halt-trigger #2.)

3. **CR-14.3 / SG-14.3**: Are masked parents still in `tape.nodes()`? Mask is read-view, NOT deletion. Trace `compute_mask_set` (`src/state/price_index.rs:364-429`) and verify Tape is never mutated.

4. **CR-14.4 / SG-14.8**: Low-liquidity children cannot mask parent. Trace `src/state/price_index.rs:402-407`.

5. **CR-14.5 / SG-14.7 / halt-trigger #6**: Open challenges block masking. Trace `src/state/price_index.rs:377-382, 410-412`.

6. **CR-14.6 / Goodhart shield**: `NodeMarketEntry` does not expose private predicate content. Trace `src/state/price_index.rs:97-109` (10 fields) + `src/bin/audit_dashboard.rs` §14 render block (lines ~1500-1570).

7. **G-14.11 / charter §5.6**: No f64 in TB-14 module surface. Verify `src/state/price_index.rs` (halt-trigger #4 fence target), `src/sdk/snapshot.rs`, `src/sdk/actor.rs` v2 span (post-Atom-5 deletion of legacy f64 + Atom 6 deletion of legacy tests), `src/bus.rs` (post-F1 fix), `src/bin/audit_dashboard.rs` §14 render block. Note: `experiments/minif2f_v4/src/bin/evaluator.rs::prompt_balance: f64` is the prompt.rs render contract; prompt.rs is NOT a TB-14 module surface (the G-14.11 fence targets price/mask code surface only). Take a position on whether this scope decision is sound.

8. **Art.0.2 replay determinism**: `tests/tb_14_chaintape_smoke.rs:307-348` claims `compute_price_index(live) == compute_price_index(replayed)` byte-equal + idempotent across 5 invocations + empty pre-condition honest. The argument is by composition: pure function over byte-equal-replayed `EconomicState` yields byte-equal output. Is this argument sound?

9. **Charter §5.6 forbidden list (no market trading / settlement / parent deletion / AMM / DPMM / price-as-oracle)**: walk `git diff a9fbdf3..38412bf` and confirm zero introductions.

**Plus architectural strategic questions** (Class 3 review beyond impl-paranoid):

10. **STEP_B Phase 1 deviation**: the commit body declares working directly on main rather than `.claude/worktrees/stepb-tb14-atom6`. Justification: Phase 0 (necessity audit) satisfied by architect ratification (charter §3 IS the ratified spec); Phase 1 (worktree isolation) adds operational coordination overhead with no audit-quality gain for a directly-spec-compliant wire-swap; Phase 3 (dual audit + merge gate) preserved via THIS audit. The internal `auditor` subagent took position "acceptable for this atom but should not become a default". Take YOUR position. Is this a load-bearing review-quality concern that should VETO, or a process-discipline observation that PASSes the wire-swap?

11. **Bus.snapshot() sequencer-optional empty fallback architectural soundness**: when `bus.sequencer == None` (legacy WAL-only smoke tests), the snapshot returns empty `price_index` + `mask_set`. The wire-swap's contract says "consumers (evaluator / dashboard) treat empty as 'no signal yet'". Is this the right semantics? Could a downstream consumer silently mistake "sequencer not wired" for "no positions exist", leading to incorrect prompt rendering or incorrect Boltzmann selection in a chaintape-misconfigured run?

12. **F1 follow-up commit (38412bf) atom-cohesion**: the dead `BusResult::Invested { shares: f64 }` deletion was a separate commit post-44cd480 rather than amended into 44cd480. The G2 single-rehash discipline is satisfied per-commit. Take YOUR position: is this the right hygiene, or should Class 3 atoms always land as a single atomic commit?

13. **Dashboard §14 SG-14.6 enforcement**: the dashboard renders the literal banner "PRICE IS SIGNAL, NOT TRUTH" (architect §5.1 verbatim) + per-node `price_yes` / `price_no` as `numerator/denominator` integer-rational strings (NEVER decimal). The `DashboardReport.price_index` is populated by `price_index_from_exposures` synthesizing an EconomicState from `exposures: Vec<ExposureRecordRow>` and calling canonical `compute_price_index` (no second source-of-truth — architect §5.1; charter §7 auto-resolution A). Is this synthesis approach sound, or does it create a rendering-time vs reality drift risk?

**Internal `auditor` subagent verdict (read-only Class 3 self-review on 44cd480)**: CHALLENGE conviction=high, recommendation=FIX-THEN-PROCEED, with a single CHALLENGE-level finding (F1: dead `BusResult::Invested { shares: f64 }` enum variant in TB-14-touched bus.rs — pre-TB-9 invest-path residual; zero call sites; halt-trigger #4 didn't fence bus.rs). F1 ADDRESSED by 38412bf (4-line deletion). Other findings F2-F5 ACCEPTED (cosmetic / out-of-scope / process-discipline / pending-external).

**Iteration cap**: 72h (Class 3 production wire-up exception per `feedback_iteration_cap_24h`). **Sync mode**: STOP after dual audit; user reviews verdict before SHIP.

End your audit with:
- **VERDICT**: PASS / CHALLENGE / VETO
- **Conviction**: low / medium / high
- **Recommendation**: PROCEED to SHIP / FIX-THEN-PROCEED / REDESIGN

Cite file:line for every finding. Be paranoid about Q1-Q9 (architect's mandated CR-14.x conformance + halt-trigger soundness). Q10-Q13 are strategic — flag concrete review-quality concerns; do not VETO on theoretical-only worries.

---

"""


def append_file(rel: str, lang: str = "rust"):
    fp = ROOT / rel
    if not fp.exists():
        return ""
    return f"\n## {rel}\n\n```{lang}\n{fp.read_text()}\n```\n"


# ── Reference docs ──
brief += "# Reference: Charter + Architect §5 + Atom 6 kickoff + Closing OBS\n"
brief += append_file("handover/tracer_bullets/TB-14_charter_2026-05-03.md", "markdown")
brief += append_file("handover/ai-direct/TB-14_ATOM_6_KICKOFF_2026-05-03.md", "markdown")
brief += append_file("handover/directives/2026-05-03_TB13_TO_TB17_POST_TB12_ARCHITECT_RULING.md", "markdown")
brief += append_file("handover/alignment/OBS_TB_12_LEGACY_CPMM_QUARANTINE_2026-05-03.md", "markdown")
brief += append_file("handover/evidence/tb_14_chaintape_smoke_2026-05-03/README.md", "markdown")

# ── TB-14 Atom 6 source files under audit ──
brief += "\n\n---\n\n# TB-14 Atom 6 source code under audit (post-38412bf)\n"
for rel in [
    "src/state/price_index.rs",
    "src/sdk/snapshot.rs",
    "src/sdk/actor.rs",
    "src/kernel.rs",
    "src/bus.rs",
    "src/lib.rs",
    "src/bin/audit_dashboard.rs",
    "src/state/sequencer.rs",
    "src/state/q_state.rs",
    "experiments/minif2f_v4/src/bin/evaluator.rs",
    "tests/tb_14_chaintape_smoke.rs",
    "tests/tb_14_halt_triggers.rs",
    "tests/tb_14_price_index.rs",
    "tests/tb_14_mask_set.rs",
    "tests/tb_13_legacy_cpmm_forward_fence.rs",
    "tests/fc_alignment_conformance.rs",
]:
    lang = "markdown" if rel.endswith(".md") else "rust"
    brief += append_file(rel, lang)

brief += "\n---\n\nGive your INDEPENDENT TB-14 Atom 6 ship audit. Be paranoid about Q1-Q9 (the architect's mandated halt-triggers + CR-14.x conformance). Q10-Q13 are strategic — flag review-quality concerns. Cite file:line for every finding.\n"

print(f"[gemini tb-14] prompt size: {len(brief):,} chars", file=sys.stderr)

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
    print(f"[gemini tb-14] API error: {e}", file=sys.stderr)
    sys.exit(1)
elapsed = time.time() - t0
print(f"[gemini tb-14] API returned in {elapsed:.1f}s", file=sys.stderr)

# Extract text
try:
    text = data["candidates"][0]["content"]["parts"][0]["text"]
except (KeyError, IndexError) as e:
    print(f"[gemini tb-14] response shape error: {e}\nfull response: {json.dumps(data, indent=2)[:2000]}", file=sys.stderr)
    sys.exit(1)

header = (
    f"# Gemini TB-14 Atom 6 Ship Audit — production wire-swap (Class 3)\n"
    f"**Round**: {ROUND}\n"
    f"**Date**: 2026-05-03\n"
    f"**HEAD**: 38412bf (Atom 6 main 44cd480 + auditor F1 follow-up 38412bf)\n"
    f"**Test baseline**: cargo test --workspace = 821 PASS / 0 FAILED / 150 ignored\n"
    f"**Halt-triggers**: 6/6 GREEN (architect §5.7)\n"
    f"**ChainTape smoke**: chain-backed PASS (handover/evidence/tb_14_chaintape_smoke_2026-05-03/)\n"
    f"**Internal auditor**: CHALLENGE→F1 addressed by 38412bf; F2-F5 ACCEPTED\n"
    f"**Elapsed**: {elapsed:.1f}s\n"
    f"**Prompt size**: {len(brief):,} chars\n"
    f"**Audit mode**: Class 3 dual (Codex impl-paranoid + Gemini architectural strategic; per charter §4)\n\n---\n\n"
)
OUT.write_text(header + text)
print(f"[gemini tb-14] saved: {OUT}")
