# Session #43 boot prompt — TB-G G2P Peer Verification Bridge

Copy-paste this into the next session's first message.

---

TB-G G2P PARALLEL-priority forward. Session #42 closed at `origin/main` HEAD `62378ed` — **G1 module 🟢 LANDED** (G1.2-5..G1.2-8 SHIPPED; Codex R2 audit PROCEED 12/12 PASS; 9-task chain-continuous batch `n_witnessed=4`; matrix §R G1 row AMBER → GREEN).

Read in this order before any action:

1. `CLAUDE.md`
2. `handover/ai-direct/LATEST.md` (session #42 block at top is the live context; sessions #41/#40/#39 below are audit trail)
3. `handover/tracer_bullets/TB_G_GENERATIVE_ARENA_charter_2026-05-11.md` §1 Module G2P (Peer Verification Bridge — architect §8.2 PARALLEL priority; closes user 2026-05-12 病灶3 "0 verify")
4. `handover/directives/2026-05-11_G_PHASE_GENERATIVE_ARENA_DIRECTIVE.md` §0.6 amendment G-2 verbatim "verify_peer=0 比 invest=0 更危险"
5. `handover/evidence/g_phase_g1_2_full_2026-05-11T23-36-38Z/CROSS_PROBLEM_PERSISTENCE_REPORT.md` §5 forward queue + §4 Q6.4/Q6.6 mechanism-bottleneck analysis
6. `handover/alignment/CONSTITUTION_EXECUTION_MATRIX.md` §R G2P row (currently 🔴 RED pending atom landing)

Forward atoms in order (G2P is canonical NEXT per architect §0.6 amendment G-2 + Q6.6 mechanism analysis):

**G2P.1** — Class 2; `=== Pending Peer Reviews ===` prompt block:
- EDIT `src/sdk/prompt.rs` (new section renderer; per-viewer scoped — never broadcast another agent's PnL or peer-review queue per CLAUDE.md §15 selective shielding)
- EDIT `evaluator.rs:2098..` (call site wiring)
- NEW `tests/constitution_g2p_pending_peer_reviews.rs` (SG-G2P.1 + SG-G2P.2): per-viewer source-grep clean + fixture renders pending-review row
- Constitutional anchor: charter §0.6 amendment G-2 + CLAUDE.md §15 shielding rules

**G2P.2** — Class 2; peer-verify-coverage §F.X dashboard + walker:
- EDIT `src/bin/audit_dashboard.rs::render_tb_n3_run_report` (add §F.X with per-agent `peer_verify_count`)
- NEW `tests/constitution_g2p_peer_verify_coverage.rs` (SG-G2P.3 / SG-G2P.4 / SG-G2P.5):
  - walker emits per-agent peer_verify_count
  - §F.X renders coverage %
  - persistent-batch ≥1 non-solver VerifyTx OR explicit bottleneck explanation

**G2P.3** — Class 1; verifier reward / bond return audit:
- EDIT `src/state/sequencer.rs` (existing VerifyTx arm; verify it preserves TB-N1 A4 reputation accumulation + bond return)
- SG-G2P.6: existing TB-N1 A4 gates GREEN OR `OBS_G2P_VERIFY_PEER_REWARD` filed

**G2P real-LLM smoke** — autonomous after G2P.1+G2P.2+G2P.3 ship pure-code:
- Re-run `bash scripts/run_g_phase_batch.sh g_phase_g2p_<TS> full` with G2P prompt block active
- Expected delta vs G1.2-7 R2 evidence: `reputations_t` non-empty (≥1 VerifyTx accepted); `n_witnessed` increases from 4 → 5+ (reputation Witnessed)
- Codex G2 single-auditor full audit per session #42 cadence ("Gemini 总是 all pass — 意义不大")

Halt-and-re-charter Class 4 trigger (same as G1.2 per Option B+ ruling §Q4): if implementation forces touching sequencer admission semantics / TypedTx canonical schema / canonical signing payload / system tx authorization / EconomicState schema / HEAD_t definition / constitution.md text — HALT and draft separate §8 packet. G2P design as Class 2 should NOT hit this; sequencer's VerifyTx arm already exists (TB-N1 A4).

Memory triggers to honor (unchanged from session #42):
- Before any `bash run_*.sh` runner: `/runner-preflight` skill (7-stage)
- Before drafting new TB charter / picking next atom sequence beyond what's named here: `/constitution-landing-check` skill (this session is already inside an active TB-G module sequence — likely not needed)
- After TB ship final OR audit rounds > 3: `/harness-reflect` skill
- On any FC1/FC2/FC3 problem: trace BEFORE designing fix
- Before writing new `feedback_*.md`: ask "what mechanism enforces this?" — build mechanism, not just norm

Mode: continuing TB-G G2P sequence under existing parent §8 sign-off (G1.1 packet §6 "好，确认可以 ship" 2026-05-11 + G-Phase directive autonomous-forward authorization). No new §8 packet required for G2P (Class 2). G3.2 + G4.2 are the next Class-4 §8 packet boundaries (not this session).

Disk hygiene: G2P real-LLM smoke will produce ~11M evidence dir (matches G1.2-7 R2 11M). Pre-launch `df -h /home/zephryj` and `cargo clean` if < 20G free.

Codex zombie hygiene: session #42 saw 10 stale Codex broker/app-server PIDs (1-41 days old) needing cleanup. If user reports "Agent Thread Limit is full" again, run `ps -eo pid,etime,cmd | grep -E "codex.*broker|codex app-server" | grep -v grep` and SIGKILL anything > 1 day old (worktree may have been deleted; broker holds OpenAI session state). DO NOT kill the current Claude session's broker (cwd=turingosv4, < 1 day) or the Zed codex-acp.

Codex dispatch route preference (per `feedback_codex_bash_exec_direct_dispatch`):
- Try `Skill: codex:rescue` first.
- If internal error or ≥2 rejections: fall back to `nohup codex exec --dangerously-bypass-approvals-and-sandbox -C /home/zephryj/projects/turingosv4 < /tmp/prompt.md > /tmp/codex_out.log 2>&1 &` then Monitor for `^VERDICT:` lines.

Carry-forward Open Questions from session #42:
1. CpmmPool was auto-emitted on G1.2-7 R2 9-task batch but no `BuyWithCoinRouter` swap happened. After G2P + G5.1 ship, re-run smoke to confirm pool now gets traded (closes 病灶2 fully).
2. The 13 distinct agents in G1.2-7 R2 persistence include preseed 12 + 1 boltzmann-seeded solver — confirm this matches architect's intended agent registry shape (preseed list vs runtime-induced identities).
3. WalletBackend trait (charter §0.66, forward TB-H Class-4) — §8 packet during G-Phase or after G7 closes? Carrying forward; not blocking G2P.

---
**Session #42 ship summary** (already in LATEST.md):
- 10 commits dbed8bf → 62378ed
- 6 SG-G1.2-5.* gates + 3 lib unit + 5 SG-G1.2-3.4 unify + persistence binding `is_passing`/`n_witnessed` serialized
- Codex G2 R2 PROCEED 12/12 (Q11 closed by `TURINGOS_CHAINTAPE_PRESEED=1` env wiring)
- Persistence binding `n_witnessed=4` over 9 tasks: balances/positions/pnl/model_identity Witnessed; reputation/autopsy Empty
- Constitution gates 336 → 342
- Matrix §R G1: 🟡 → 🟢; G2/G2P/G3/G4/G5/G6/G7 still 🔴
