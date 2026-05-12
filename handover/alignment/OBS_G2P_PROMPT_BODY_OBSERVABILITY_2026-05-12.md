# OBS — TB-G G2P prompt-body observability gap (2026-05-12)

**Type**: Forward-bound evidence-bundle gap — Codex G2 G2P audit Q1
CHALLENGE closure path.

**Status**: 🟡 **OBS filed** — G2P.1 wire-up is structurally correct
(`src/sdk/prompt.rs:141` + `experiments/minif2f_v4/src/bin/evaluator.rs:~2188`
verified; 14 lib+gate tests GREEN), but production batch evidence does
not capture prompt bodies, so the audit cannot empirically prove the
`=== Pending Peer Reviews ===` block was rendered into the LLM's
prompt during the 9-task batch.

**Authority**: Codex G2 G2P audit Q1 verdict
`handover/audits/CODEX_G2_TB_G_G2P_VERDICT.md` — CHALLENGE, NOT VETO;
audit overall PROCEED 11/12 PASS.

---

## §1 What Codex observed

`rg` over the G2P smoke evidence dir
(`handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/`) found:
- **0 hits** for `Pending Peer Reviews` in `P*_*/evaluator.{stdout,stderr}`
- **0 hits** for `PromptCapsule` in CAS index / audit trail
- per-task `evaluator.stdout` files are 1 line each (the `PPUT_RESULT`
  JSON line; no echoed prompt body)

The source wire is present:
- `src/sdk/prompt.rs:~141` renders `=== Pending Peer Reviews ===` block
  when the passed string is non-empty
- `experiments/minif2f_v4/src/bin/evaluator.rs:~2188` calls
  `pending_peer_reviews::render_pending_peer_reviews(&q, &agent_id, K)`
  and threads result into `build_agent_prompt`'s 9th param

But the prompt body is POSTed to the LLM proxy + immediately moved into
`Message.content` (`evaluator.rs:~2204`); no CAS write of the prompt
bytes occurs in the production swarm path.

## §2 Why this is OBS, not blocker

- Codex's verdict is CHALLENGE, NOT VETO. Audit overall PROCEED 11/12.
- G2P.1 unit tests (8 in `src/sdk/pending_peer_reviews.rs::tests`) +
  constitution gates (6 in `tests/constitution_g2p_pending_peer_reviews.rs`)
  pin the renderer + wire-up at the source-grep / fixture-render layer.
- Architect §8.5 OR-branch ("empty market as valid empirical result")
  is satisfied by the explicit §F.X MECHANISM BOTTLENECK render — the
  prompt-block-reach evidence is "nice-to-have" but not the architect
  §8.2 ship-gate signal.
- This is an observability gap in the **evidence packaging policy**,
  not in the production wire-up.

## §3 Forward closure path

The PromptCapsule schema is already LANDED per
`handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` (Art. III selective
shielding row, prompt persistence; charter §4.3 G-016/G-019/G-021/G-028
ratified). What is missing is the **wire-up at the evaluator swarm path**
to write a PromptCapsule CAS object per LLM call, with:
- `prompt_context_hash` (already computed at `evaluator.rs:~r2_prompt_ctx_hash`)
- `read_set` (the canonical state surfaces the prompt drew from:
  `chain_so_far`, `econ_position`, `pending_peer_reviews`, `market_ticker`,
  `team_board`, `recent_errors`, `recent_search_hits`)
- `policy_version` (e.g. `g2p_v1`)
- `hidden_fields_redacted` (empty for now; future: PnL or autopsy CIDs
  the viewer should NOT see)
- `visible_context_cid` (CAS-anchored verbatim prompt body, Class-4
  encrypted/audit-only or Class-3 if redaction policy permits)
- `system_prompt_template_hash` (sha256 of the build_agent_prompt
  template at the binary's HEAD)
- `agent_view_manifest_cid` (manifest of `read_set` resolved to
  concrete (key, value) pairs at prompt build time)

**Class**: 2-3 (additive write path; no admission-arm mutation; no
typed_tx schema change). G2P module charter §1 "Class peak: 2" envelope
already covers this. Architect §8 packet NOT required.

## §4 Forward closure criteria

This OBS closes when:
- evaluator swarm path writes a `PromptCapsule` CAS object per LLM
  call (or per N calls if rate-limited)
- next persistent batch's evidence dir contains a non-empty
  `prompt_capsules/` directory OR a CAS index entry per-call
- a `tests/constitution_g2p_prompt_body_evidence.rs` gate asserts
  ≥1 PromptCapsule CAS object per accepted WorkTx OR per task

## §5 Why this is `feedback_norm_needs_mechanism`-clean

Mechanism: the forward closure criterion (§4 above) is a concrete gate
that fails if the next batch still has zero PromptCapsule CAS writes.
Adding the constitution test BEFORE the wire-up makes this a "build
the gate first, then close it" pattern per the rule.

## §6 Cross-references

- Codex audit verdict: `handover/audits/CODEX_G2_TB_G_G2P_VERDICT.md`
- Codex audit transcript: `handover/audits/CODEX_G2_TB_G_G2P_AUDIT.log`
- G2P smoke evidence: `handover/evidence/g_phase_g2p_2026-05-12T01-34-37Z/`
- PromptCapsule schema: `src/runtime/prompt_capsule.rs`
- G2P.1 source wire: `src/sdk/prompt.rs` + `src/sdk/pending_peer_reviews.rs`
  + `experiments/minif2f_v4/src/bin/evaluator.rs` (line ~2188)
- Charter §4.3 PromptCapsule landing row:
  `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md`
