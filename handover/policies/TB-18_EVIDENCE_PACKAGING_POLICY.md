# TB-18 EvidencePackagingPolicy (architect §2.3 + TB-7R/TB-8/TB-9 precedent)

**Status**: AUTHORITATIVE for TB-18 M-ladder evidence (M0 / M1 / M2 stages).
**Filed**: 2026-05-05.
**Authority**:
  - Architect TB-18 ratification ruling §2.3 (architect-flagged AI-coder blind spot; cites TB-7R Codex VETO precedent on missing `runtime_repo/.git` + `cas/.git/objects`) — `handover/directives/2026-05-05_TB18_CHARTER_RATIFICATION_ARCHITECT_RULING.md`.
  - TB-7R/TB-8/TB-9 codification of `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` per packaged run — `handover/tracer_bullets/TB-8_charter_2026-05-02.md` + `handover/tracer_bullets/TB-9_charter_2026-05-02.md`.
  - TB-18 charter §1.4 SG-18.13 + §8 EvidencePackagingPolicy spec — `handover/tracer_bullets/TB-18_charter_2026-05-05.md`.
  - Memory binding: `feedback_evidence_packaging_policy_required` (NEW from this charter ratification).

---

## §1 Per-stage strategy

| Stage | Volume | Strategy | Sample composition (deterministic; seed below) |
|---|---|---|---|
| **M0** (20 problems × n1; harness audit) | small | **FULL restorable** evidence: every problem committed with `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` + `verdict.json` + `verdict_replay.json` + `tamper_report.json` + `evaluator.{stdout,stderr}` + `audit_tape*.stderr` | n/a (full coverage) |
| **M1** (50–100 problems × n1/n3) | medium | **SAMPLED restorable** + aggregate manifest + `EVIDENCE_INDEX.json` enumerating ALL runs | 10 random + 5 failure-heavy + 5 solved + 5 unsolved + first-of-each-failure-class (≤5 classes) → ~20-30 packaged runs out of 50-100 total |
| **M2** (100+ problems × n5) | large | **SAMPLED restorable** + aggregate manifest + `EVIDENCE_INDEX.json` | 20 random + 10 failure-heavy + 10 solved + 10 unsolved + first-DegradedLLM + first-WallClockCap + first-ErrorHalt → ~50-60 packaged runs out of 500+ total |

## §2 Sample selection determinism

Sample selection MUST be deterministic given `(manifest_id, sample_strategy_version, sample_seed)`.

```text
sample_strategy_version = "tb-18.evidence_packaging.v1"
sample_seed             = 0xC0DE5AAA  // pinned at this policy doc filing time
```

Future re-runs of the same `(manifest_id, version, seed)` triple MUST produce the same packaged sample. This lets a future auditor re-derive the sample (modulo source-code drift) and confirm what was committed matches the policy.

### §2.1 Sample selection algorithm

Given the full run index `runs[1..N]` (sorted lexicographically by run_id), compute:

```python
import hashlib
import random

def sample_for_stage(stage: str, runs: list[dict], manifest_id: str) -> list[dict]:
    """Deterministic packaging sample selection."""
    sample_seed = 0xC0DE5AAA
    sample_strategy_version = "tb-18.evidence_packaging.v1"
    seed_bytes = hashlib.sha256(
        f"{manifest_id}|{sample_strategy_version}|{sample_seed}".encode()
    ).digest()[:8]
    rng = random.Random(int.from_bytes(seed_bytes, 'big'))

    if stage == "M0":
        return runs  # FULL coverage; no sampling

    # Categorize
    solved = [r for r in runs if r["outcome"] == "solved"]
    unsolved = [r for r in runs if r["outcome"] != "solved" and r["outcome"] != "ErrorHalt"]
    failure_heavy = [r for r in runs if r.get("failure_count", 0) >= 5]
    by_class = {}  # outcome -> list
    for r in runs:
        by_class.setdefault(r["outcome"], []).append(r)

    if stage == "M1":
        sample = []
        sample.extend(rng.sample(runs, min(10, len(runs))))
        sample.extend(rng.sample(failure_heavy, min(5, len(failure_heavy))))
        sample.extend(rng.sample(solved, min(5, len(solved))))
        sample.extend(rng.sample(unsolved, min(5, len(unsolved))))
        for cls in by_class:
            if by_class[cls]:
                sample.append(by_class[cls][0])
        return list({r["run_id"]: r for r in sample}.values())  # dedup by run_id

    if stage == "M2":
        sample = []
        sample.extend(rng.sample(runs, min(20, len(runs))))
        sample.extend(rng.sample(failure_heavy, min(10, len(failure_heavy))))
        sample.extend(rng.sample(solved, min(10, len(solved))))
        sample.extend(rng.sample(unsolved, min(10, len(unsolved))))
        firsts = [
            ("DegradedLLM", next((r for r in runs if r["outcome"] == "DegradedLLM"), None)),
            ("WallClockCap", next((r for r in runs if r["outcome"] == "WallClockCap"), None)),
            ("ErrorHalt", next((r for r in runs if r["outcome"] == "ErrorHalt"), None)),
        ]
        for _, r in firsts:
            if r:
                sample.append(r)
        return list({r["run_id"]: r for r in sample}.values())

    raise ValueError(f"unknown stage: {stage}")
```

Sample selection script lives at `handover/tests/scripts/tb_18_sample_packaged_runs.py` (TB-18.H-impl); committed alongside M1/M2 evidence.

## §3 Per-packaged-run unit (TB-7R/TB-8/TB-9 codification)

Each packaged run directory `evidence/<stage>/<run_id>/` MUST contain:

```text
runtime_repo.dotgit.tar.gz       # FULL .git/objects + branches; tar -czf .git
cas.dotgit.tar.gz                # FULL .git/objects + branches
runtime_repo/                    # working tree files (non-.git): agent_pubkeys.json, genesis_report.json, etc.
cas/                             # working tree files (non-.git): .turingos_cas_index.jsonl, etc.
verdict.json                     # audit_tape verdict from runtime
verdict_replay.json              # audit_tape verdict from replay (byte-equal to verdict.json)
tamper_report.json               # audit_tape_tamper detected count + per-variant detail
evaluator.stdout                 # PPUT_RESULT JSON line(s) (omitted if empty)
evaluator.stderr                 # tracing logs + warnings (omitted if empty)
audit_tape.stderr                # audit_tape stderr (verdict messages)
audit_tape_tamper.stderr         # audit_tape_tamper stderr
proofs/<theorem>_<timestamp>_<hash>.lean   # winning proof file (if solved); else absent
h_vppu_history.json              # per-problem rolling H-VPPUT history (atom A; if condition=n1)
```

Plus directory-level:

```text
M0_RUN_MANIFEST.json             # per-stage manifest + git_head + preflight Mathlib verify
M0_BATCH_SUMMARY.json            # aggregate counts (proceed/block/error/replay/tamper/solved/exhausted)
.gitignore                       # see §3.1 nested .git policy
```

### §3.1 Nested .git handling

The `runtime_repo/` and `cas/` working trees historically contain `.git/` dirs (chain history). Two failure modes if not handled:
1. Git submodule semantics on add (committing a working tree with .git auto-creates a submodule entry).
2. Loss of `.git/objects` blobs if working tree committed without tarball backup → TB-7R Codex VETO precedent.

Resolution (per TB-7R/TB-8/TB-9): tar each `.git` dir into `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz`; either delete the `.git` dirs from the working tree (preferred) OR add a `.gitignore` excluding them (alternative if working tree must remain re-runnable in-place).

## §4 Local-only evidence pointer (uncommitted runs)

For runs NOT in the packaged sample, an `EVIDENCE_INDEX.json` enumerates them:

```json
{
  "stage": "M2",
  "manifest_id": "<sha256 of TB-18_BENCHMARK_MANIFEST.json>",
  "sample_strategy_version": "tb-18.evidence_packaging.v1",
  "sample_seed": "0xC0DE5AAA",
  "total_runs": 525,
  "packaged_runs_count": 53,
  "uncommitted_runs": [
    {
      "run_id": "n5_mathd_algebra_500_1777999999999",
      "problem_id": "mathd_algebra_500",
      "outcome": "MaxTxExhausted",
      "wall_clock_seconds": 480,
      "local_evidence_path": "/home/zephryj/projects/turingosv4/local_evidence/tb_18_m2/n5_mathd_algebra_500_1777999999999/",
      "host_id": "turingosv4-host",
      "kept_until_utc": "2026-08-05T00:00:00Z",
      "tamper_proof_local": "sha256(local_evidence_path/runtime_repo.dotgit.tar.gz)=<hash>"
    }
  ]
}
```

This satisfies architect §2.3 verbatim: "如果你打算不 commit all CAS for 100+ problems，需要明确：where full evidence lives; how auditor can reproduce; what subset is committed; how subset was selected."

## §5 Replay integrity check (per packaged run)

For every committed `runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz` pair:

```bash
# Extract
mkdir -p /tmp/replay/{rt,cas}
tar -xzf <run>/runtime_repo.dotgit.tar.gz -C /tmp/replay/rt
tar -xzf <run>/cas.dotgit.tar.gz -C /tmp/replay/cas

# Working tree (already committed); join with .git from tarballs
cp -r <run>/runtime_repo/* /tmp/replay/rt/  # if working tree files separate
cp -r <run>/cas/*          /tmp/replay/cas/

# 1. fsck
git -C /tmp/replay/rt fsck --strict || fail
git -C /tmp/replay/cas fsck --strict || fail

# 2. audit_tape from extracted state
target/release/audit_tape \
  --runtime-repo /tmp/replay/rt \
  --cas-dir /tmp/replay/cas \
  --agent-pubkeys /tmp/replay/rt/agent_pubkeys.json \
  --pinned-pubkeys /tmp/replay/rt/pinned_pubkeys.json \
  --genesis genesis_payload.toml \
  --constitution constitution.md \
  --alignment-dir handover/alignment \
  --out /tmp/replay/verdict.json
[ "$(jq -r .verdict /tmp/replay/verdict.json)" = "PROCEED" ] || fail

# 3. byte-equal vs committed verdict
diff <run>/verdict.json /tmp/replay/verdict.json && echo "REPLAY OK"
```

Closes TB-7R Codex VETO precedent (missing `.git/objects` blobs would fail step 1 fsck).

## §6 Verification script

`handover/tests/scripts/tb_18_replay_packaged_evidence.sh` (TB-18.H-impl follow-up; not yet committed): iterates every packaged run, runs the §5 integrity check, exits non-zero on any failure. Run as part of TB-18 ship gate SG-18.13.

## §7 Cross-references

- TB-18 charter §1.4 SG-18.13 + §8 EvidencePackagingPolicy spec
- Architect TB-18 ratification ruling §2.3 (architect-flagged AI-coder blind spot)
- TB-7R / TB-8 / TB-9 charter precedent (`runtime_repo.dotgit.tar.gz` + `cas.dotgit.tar.gz`)
- TB-18 BenchmarkManifest: `handover/manifests/TB-18_BENCHMARK_MANIFEST.json`
- Memory: `feedback_evidence_packaging_policy_required` (binding for any future scaled-benchmark TB)

---

**End of policy.** TB-18 M-ladder atom H sub-stages MUST adhere to this packaging policy at SHIP gate SG-18.13.
