#!/usr/bin/env python3
"""
TB-C0 FC-witness extractor.

Walks a TB-18R-style problem run directory (containing runtime_repo/, cas/,
chain_invariant.json, verdict.json, run_summary.json, etc.) and emits a per-
FC-node witness manifest. For every FC1 / FC2 / FC3 node enumerated in
`handover/alignment/TRACE_FLOWCHART_MATRIX.md`, asserts whether tape-resident
evidence exists for that node's invariant.

For nodes WITHOUT evidence in the run, classifies the gap into:
  - "unexercised": the run didn't traverse this code path (need a different
    problem). Per `feedback_real_problems_not_designed`, gap remediation is
    to FIND a real existing problem (MiniF2F / Mathlib / Putnam / IMO /
    research-paper / web research), not to synthesize one.
  - "code_bug": the path SHOULD have been exercised but the tape lacks the
    expected witness — likely an implementation bug. Gap is escalated to
    architect / new TB.
  - "structural_only": this node is constitution-document-level (architect
    role, judge role, constitution hash) and witness is a structural
    artifact (directive trail, audit trail), not chain-resident.

Authority: TB-C0 charter §1, §2 FR-C0.12.
Charter: handover/tracer_bullets/TB-C0_charter_2026-05-06.md
Directive: handover/directives/2026-05-06_TBC0_CONSTITUTION_LANDING_RESET_DIRECTIVE.md

Usage:
    python3 scripts/fc_witness_extract.py <run_dir> [--out <manifest.json>]

Where <run_dir> is a problem subdirectory like:
    handover/evidence/tb_18r_phase_3_2026-05-06T14-13-55Z/P05_mathd_algebra_114
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path
from typing import Any


def load_jsonl(path: Path) -> list[dict[str, Any]]:
    """Load a .jsonl file. Returns empty list if missing."""
    if not path.exists():
        return []
    rows: list[dict[str, Any]] = []
    with path.open() as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError:
                pass
    return rows


def load_json(path: Path) -> dict[str, Any] | None:
    if not path.exists():
        return None
    try:
        with path.open() as f:
            return json.load(f)
    except (json.JSONDecodeError, OSError):
        return None


def cas_object_types(cas_index_path: Path) -> dict[str, int]:
    counts: dict[str, int] = {}
    for o in load_jsonl(cas_index_path):
        t = o.get("object_type", "?")
        counts[t] = counts.get(t, 0) + 1
    return counts


def assess_run(run_dir: Path) -> dict[str, Any]:
    """Build a fact bundle from the run dir we can use to map FC nodes."""
    runtime_repo = run_dir / "runtime_repo"
    cas_dir = run_dir / "cas"

    facts: dict[str, Any] = {
        "run_dir": str(run_dir),
        "runtime_repo_exists": runtime_repo.exists(),
        "cas_dir_exists": cas_dir.exists(),
    }

    # Genesis / boot artifacts (FC2)
    facts["genesis_report"] = load_json(runtime_repo / "genesis_report.json")
    facts["pinned_pubkeys"] = load_json(runtime_repo / "pinned_pubkeys.json")
    facts["agent_pubkeys"] = load_json(runtime_repo / "agent_pubkeys.json")
    facts["initial_q_state"] = load_json(runtime_repo / "initial_q_state.json")
    facts["agent_audit_trail"] = load_jsonl(runtime_repo / "agent_audit_trail.jsonl")
    facts["run_summary"] = load_json(runtime_repo / "run_summary.json")

    # L4 = transition_ledger entries — git-stored as objects in .git/objects/
    # but high-level summary lives in run_summary.l4_entries
    facts["l4_entries"] = (
        facts["run_summary"].get("l4_entries", [])
        if facts["run_summary"]
        else []
    )

    # L4.E = rejections.jsonl
    facts["rejections"] = load_jsonl(runtime_repo / "rejections.jsonl")

    # CAS objects
    facts["cas_index"] = load_jsonl(cas_dir / ".turingos_cas_index.jsonl")
    facts["cas_object_types"] = cas_object_types(cas_dir / ".turingos_cas_index.jsonl")

    # Audit-tape verdict + invariant + verdict_kind summary
    facts["audit_tape_verdict"] = load_json(run_dir / "verdict.json")
    facts["chain_invariant"] = load_json(run_dir / "chain_invariant.json")
    facts["verdict_kind_summary"] = load_json(run_dir / "verdict_kind_summary.json")
    facts["extracted_pput"] = load_json(run_dir / "extracted_pput.json")
    facts["architect_inv1_check"] = load_json(run_dir / "architect_inv1_check.json")

    # Markov capsule presence
    cas_types = facts["cas_object_types"]
    facts["has_evidence_capsule"] = cas_types.get("EvidenceCapsule", 0) >= 1
    facts["has_compressed_run_log"] = cas_types.get("CompressedRunLog", 0) >= 1
    facts["attempt_telemetry_count"] = cas_types.get("AttemptTelemetry", 0)
    facts["lean_result_count"] = cas_types.get("LeanResult", 0)
    facts["proposal_payload_count"] = cas_types.get("ProposalPayload", 0)

    return facts


def fc_witness_status(facts: dict[str, Any]) -> dict[str, Any]:
    """For every FC node + invariant, return witness status."""
    nodes: dict[str, dict[str, Any]] = {}

    # ── FC1 — Runtime loop ──────────────────────────────────────────────
    nodes["FC1-N1_q_state_carrier"] = {
        "witness": "initial_q_state.json + final state via run_summary",
        "status": "✅" if facts["initial_q_state"] else "🔴",
        "detail": "initial_q_state present"
        if facts["initial_q_state"]
        else "MISSING initial_q_state.json",
    }
    nodes["FC1-N2_q_t_slice"] = {
        "witness": "Q_t advancement visible via state_root in rejections.jsonl + L4 entries",
        "status": "✅" if facts["rejections"] else "🟡",
        "detail": f"{len(facts['rejections'])} rejection records carry parent_state_root"
        if facts["rejections"]
        else "no rejections (single-shot or empty run)",
    }
    nodes["FC1-N3_HEAD_t_pointer"] = {
        "witness": "audit_tape verdict.head_state_root_hex + head_ledger_root_hex",
        "status": "✅"
        if facts.get("audit_tape_verdict", {})
        and facts["audit_tape_verdict"].get("tape_root", {}).get("head_state_root_hex")
        else "🔴",
        "detail": str(
            facts.get("audit_tape_verdict", {}).get("tape_root", {}).get("head_state_root_hex", "MISSING")
        )[:40],
    }
    l4_count = (
        facts.get("audit_tape_verdict", {}).get("tape_root", {}).get("l4_count", 0)
        if facts.get("audit_tape_verdict")
        else 0
    )
    l4e_count = (
        facts.get("audit_tape_verdict", {}).get("tape_root", {}).get("l4e_count", 0)
        if facts.get("audit_tape_verdict")
        else 0
    )
    nodes["FC1-N4_q1_after_delta"] = {
        "witness": "L4 entries (accepted WorkTx advance Q_t)",
        "status": "✅" if l4_count > 0 else "🟡",
        "detail": f"l4_count={l4_count}",
    }
    nodes["FC1-N5_rtool"] = {
        "witness": "agent_audit_trail.jsonl records (snapshot reads)",
        "status": "✅" if facts["agent_audit_trail"] else "🔴",
        "detail": f"{len(facts['agent_audit_trail'])} agent audit-trail records",
    }
    nodes["FC1-N7_delta_AI_call"] = {
        "witness": "AttemptTelemetry CAS objects (one per LLM call)",
        "status": "✅" if facts["attempt_telemetry_count"] > 0 else "🔴",
        "detail": f"{facts['attempt_telemetry_count']} AttemptTelemetry CAS objects",
    }
    nodes["FC1-N11_predicates"] = {
        "witness": "LeanResult CAS objects (predicate verdict per attempt)",
        "status": "✅" if facts["lean_result_count"] > 0 else "🔴",
        "detail": f"{facts['lean_result_count']} LeanResult CAS objects",
    }
    nodes["FC1-N13_wtool"] = {
        "witness": "L4 + L4.E entries (sequencer-mediated writes)",
        "status": "✅" if (l4_count + l4e_count) > 0 else "🔴",
        "detail": f"l4={l4_count}, l4e={l4e_count}",
    }
    nodes["FC1-N15_reject_branch"] = {
        "witness": "L4.E entries (rejected proposals routed away from L4)",
        "status": "✅" if len(facts["rejections"]) > 0 else "🟡 unexercised",
        "detail": f"{len(facts['rejections'])} rejection records on L4.E",
    }
    # FC1-INV1: every externalized attempt is tape-visible
    chain_inv = facts.get("chain_invariant", {})
    inv1_check = facts.get("architect_inv1_check", {})
    arch_match = inv1_check.get("match") if inv1_check else None
    nodes["FC1-INV1_every_attempt_tape_visible"] = {
        "witness": "AttemptTelemetry count == l4_work + l4e_work + capsule_anchored (extended FC1 hard invariant)",
        "status": "✅" if arch_match is True else "🔴 code_bug" if arch_match is False else "🟡",
        "detail": (
            f"chain_invariant: expected={chain_inv.get('expected_completed_attempts')}, "
            f"l4={chain_inv.get('l4_work_attempt_count')}, "
            f"l4e={chain_inv.get('l4e_work_attempt_count')}, "
            f"AT_count={facts['attempt_telemetry_count']}, "
            f"architect_inv1.match={arch_match} "
            f"(architect_chain_attempt_count={inv1_check.get('chain_attempt_count') if inv1_check else 'N/A'}, "
            f"evaluator_reported_tx_count={inv1_check.get('evaluator_reported_tx_count') if inv1_check else 'N/A'})"
        ),
    }
    # FC1-INV3: count equality (constitutional 3-term formula).
    # **TB-C0 strict-audit Bug 3 fix (2026-05-07)**: the prior implementation
    # `capsule_anchored = AT - l4 - l4e` was TAUTOLOGICAL by construction
    # (yielding `at == at` always). Now the chain_invariant binary computes
    # `capsule_anchored_attempt_count` independently by walking CAS for
    # AttemptTelemetry records with `outcome == AttemptOutcome::PartialAccepted`.
    # See `STRICT_AUDIT_TBC0_TAPE_2026-05-07.md` §1 Finding C.
    expected = chain_inv.get("expected_completed_attempts", 0)
    l4_w = chain_inv.get("l4_work_attempt_count", 0)
    l4e_w = chain_inv.get("l4e_work_attempt_count", 0)
    at_count = facts["attempt_telemetry_count"]
    # Independent capsule_anchored count from chain_invariant.json (Bug 3 fix).
    # If the binary that produced the chain_invariant.json predates Bug 3 fix,
    # the field is absent → fall back to derived `at - l4 - l4e` (with explicit
    # warning in the detail string) AND mark as legacy.
    capsule_anchored_independent = chain_inv.get("capsule_anchored_attempt_count")
    is_independent = capsule_anchored_independent is not None
    if not is_independent:
        capsule_anchored_independent = max(0, at_count - l4_w - l4e_w)
    constitutional_lhs = expected
    constitutional_rhs = (
        l4_w + l4e_w + capsule_anchored_independent
    )
    invariant_holds = constitutional_lhs == constitutional_rhs
    delta_3term = constitutional_rhs - constitutional_lhs
    if invariant_holds:
        status = "✅" if is_independent else "🟡"
    else:
        status = "🔴 code_bug" if is_independent else "🟡"
    nodes["FC1-INV3_count_equality_constitutional"] = {
        "witness": "expected == L4_work + L4E_work + capsule_anchored (independent count)",
        "status": status,
        "detail": (
            f"expected={expected} vs (l4={l4_w} + l4e={l4e_w} + capsule={capsule_anchored_independent}) "
            f"= {constitutional_rhs}; delta_3term={delta_3term:+d}; AT_count={at_count}; "
            f"capsule_source={'chain_invariant.json (independent, Bug 3-fix binary)' if is_independent else 'derived AT-l4-l4e (legacy binary; tautological)'}"
        ),
    }

    # ── FC2 — Boot ──────────────────────────────────────────────────────
    gr = facts["genesis_report"]
    nodes["FC2-N16_InitAI"] = {
        "witness": "genesis_report.json with constitution_hash + run_id + Q_0",
        "status": "✅" if gr else "🔴",
        "detail": f"genesis_report fields={list(gr.keys())[:6]}" if gr else "MISSING",
    }
    nodes["FC2-N18_constitution_ground_truth"] = {
        "witness": "audit_tape verdict.constitution_hash_hex + assertion 'constitution_hash_matches_genesis'",
        "status": "✅"
        if facts.get("audit_tape_verdict", {})
        .get("tape_root", {})
        .get("constitution_hash_hex")
        else "🔴",
        "detail": str(
            facts.get("audit_tape_verdict", {}).get("tape_root", {}).get("constitution_hash_hex", "MISSING")
        )[:32]
        + "...",
    }
    nodes["FC2-N21_Q0_minted"] = {
        "witness": "initial_q_state.json shows pre-seeded EconomicState (Coin balances at boot)",
        "status": "✅" if facts["initial_q_state"] else "🔴",
        "detail": "Q_0 ledger present"
        if facts["initial_q_state"]
        else "no initial_q_state",
    }
    nodes["FC2-N22_HALT"] = {
        "witness": "chain_invariant.terminal_halt_class (RunOutcome variant)",
        "status": "✅" if chain_inv.get("terminal_halt_class") else "🔴",
        "detail": f"halt={chain_inv.get('terminal_halt_class')}",
    }
    assertions = facts.get("audit_tape_verdict", {}).get("assertions", []) if facts.get("audit_tape_verdict") else []
    passed_count = sum(1 for a in assertions if a.get("result") == "Pass")
    failed_count = sum(1 for a in assertions if a.get("result") not in ("Pass", "Skipped"))
    skipped_count = sum(1 for a in assertions if a.get("result") == "Skipped")
    total = len(assertions)
    nodes["FC2-INV1_genesis_replayable"] = {
        "witness": "audit_tape verdict assertions Pass (replay verifies; audit_tape walks L4+L4.E+CAS)",
        "status": "✅"
        if (total > 0 and failed_count == 0)
        else "🔴 code_bug" if failed_count > 0
        else "🟡",
        "detail": (
            f"{passed_count} Pass / {failed_count} Fail / {skipped_count} Skipped "
            f"of {total} audit_tape assertions"
        ),
    }
    nodes["FC2-INV4_taskopen_escrowlock_chain_events"] = {
        "witness": "audit_tape tx_kind_counts shows task_open >= 1 + escrow_lock >= 1",
        "status": "✅"
        if facts.get("audit_tape_verdict", {})
        and facts["audit_tape_verdict"].get("tx_kind_counts", {}).get("task_open", 0)
        >= 1
        else "🟡 unexercised",
        "detail": (
            f"task_open={facts.get('audit_tape_verdict', {}).get('tx_kind_counts', {}).get('task_open', 0)}, "
            f"escrow_lock={facts.get('audit_tape_verdict', {}).get('tx_kind_counts', {}).get('escrow_lock', 0)}"
        ),
    }
    nodes["FC2-INV6_pubkeys_verify"] = {
        "witness": "pinned_pubkeys.json + audit_tape assertion 'system_pubkey_verifies_terminal_summary'",
        "status": "✅" if facts["pinned_pubkeys"] else "🔴",
        "detail": "pinned_pubkeys present"
        if facts["pinned_pubkeys"]
        else "MISSING pinned_pubkeys.json",
    }
    nodes["FC2-INV7_agent_registry_resolves"] = {
        "witness": "agent_pubkeys.json with resolvable agent_id → ed25519 pubkey",
        "status": "✅"
        if facts["agent_pubkeys"]
        and (
            isinstance(facts["agent_pubkeys"], dict)
            or isinstance(facts["agent_pubkeys"], list)
        )
        else "🔴",
        "detail": "agent_pubkeys present"
        if facts["agent_pubkeys"]
        else "MISSING",
    }

    # ── FC3 — Meta ──────────────────────────────────────────────────────
    nodes["FC3-INV1_capsule_derived"] = {
        "witness": "EvidenceCapsule CAS object (derived from L4+CAS)",
        "status": "✅" if facts["has_evidence_capsule"] else "🟡 unexercised",
        "detail": "EvidenceCapsule present in CAS"
        if facts["has_evidence_capsule"]
        else "no EvidenceCapsule (run didn't trigger capsule emission)",
    }
    legacy_pointer = Path("handover/markov_capsules/LATEST_MARKOV_CAPSULE.txt")
    nodes["FC3-INV2_no_global_pointer"] = {
        "witness": "filesystem absence of LATEST_MARKOV_CAPSULE.txt",
        "status": "✅" if not legacy_pointer.exists() else "🔴",
        "detail": "absent (Art. 0.2 honored)"
        if not legacy_pointer.exists()
        else "REGRESSION: file reappeared",
    }
    nodes["FC3-INV3_raw_logs_shielded"] = {
        "witness": "agent_audit_trail.jsonl shows snapshot reads do NOT include raw stderr; structural test fc3_raw_logs_not_in_agent_read_view",
        "status": "🟡 structural_only",
        "detail": "verified via constitution_fc3_meta::fc3_raw_logs_not_in_agent_read_view (source-grep test)",
    }
    nodes["FC3-INV5_deep_history_override"] = {
        "witness": "structural test fc3_deep_history_requires_override + TURINGOS_MARKOV_OVERRIDE env-flag",
        "status": "🟡 structural_only",
        "detail": "verified via constitution_fc3_meta::fc3_deep_history_requires_override",
    }
    nodes["FC3-INV7_architect_propose_only"] = {
        "witness": "handover/directives/*.md trail (architect proposals as committed-doc artifacts)",
        "status": "🟡 structural_only",
        "detail": "verified via constitution_fc3_meta::fc3_architectai_proposal_not_direct_write",
    }
    nodes["FC3-INV8_judge_veto_only"] = {
        "witness": "handover/audits/*.md trail (Codex+Gemini verdict reports, no code commits)",
        "status": "🟡 structural_only",
        "detail": "verified via constitution_fc3_meta::fc3_judgeai_veto_only",
    }

    return nodes


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    p.add_argument("run_dir", type=Path, help="problem run directory")
    p.add_argument(
        "--out",
        type=Path,
        default=None,
        help="manifest output path (default: <run_dir>/fc_witness_manifest.json)",
    )
    args = p.parse_args()

    if not args.run_dir.exists():
        print(f"ERROR: {args.run_dir} does not exist", file=sys.stderr)
        return 2

    facts = assess_run(args.run_dir)
    nodes = fc_witness_status(facts)

    # Tally
    green = sum(1 for v in nodes.values() if v["status"].startswith("✅"))
    amber = sum(1 for v in nodes.values() if v["status"].startswith("🟡"))
    red = sum(1 for v in nodes.values() if v["status"].startswith("🔴"))

    manifest = {
        "schema_version": 1,
        "tb_id": "TB-C0",
        "tool": "scripts/fc_witness_extract.py",
        "run_dir": str(args.run_dir),
        "node_status_counts": {"green": green, "amber": amber, "red": red},
        "fc_nodes": nodes,
        "facts_summary": {
            "attempt_telemetry_count": facts["attempt_telemetry_count"],
            "lean_result_count": facts["lean_result_count"],
            "l4_count": facts.get("audit_tape_verdict", {})
            .get("tape_root", {})
            .get("l4_count", 0),
            "l4e_count": facts.get("audit_tape_verdict", {})
            .get("tape_root", {})
            .get("l4e_count", 0),
            "cas_object_types": facts["cas_object_types"],
            "rejections_jsonl_count": len(facts["rejections"]),
            "agent_audit_trail_count": len(facts["agent_audit_trail"]),
        },
    }

    out_path = args.out or args.run_dir / "fc_witness_manifest.json"
    with out_path.open("w") as f:
        json.dump(manifest, f, indent=2)

    # Print summary to stdout
    print(f"=== FC-witness manifest for {args.run_dir} ===")
    print(f"Wrote: {out_path}")
    print(f"Tally: {green} GREEN, {amber} AMBER, {red} RED")
    print()
    for k, v in nodes.items():
        print(f"  {v['status']}  {k}")
        if v["detail"]:
            print(f"        {v['detail']}")
    return 0 if red == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
