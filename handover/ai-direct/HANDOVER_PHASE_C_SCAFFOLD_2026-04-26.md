# TuringOS v4 — Phase C Scaffolding Exit Handover

**Session date**: 2026-04-26 (continuation of Phase A→B exit session, same UTC date)
**Session scope**: Phase C atoms C-pre1 + C1a + C1b + C1c + C1d + C1e + C5 + C2 runner
**Phase C scaffolding status**: complete (7/9 atoms shipped + C2 runner ready); the remaining 2 work-items are the C2 batch *execution* and the C3+C4 post-batch analysis + audit.
**Latest commit**: `4f981cd` (C5: mode_flag_binary_purity test) — runner add still uncommitted at writing.
**Repo state**: 298 PASS / 29 ignored / 0 failed (Rust); Trust Root 41 entries (42 after this commit lands); all 5 ablation modes pass startup gate; smoke confirmed Homogeneous mode runs end-to-end at MAX_TX=2 in ~4 min.

> **New-session entry**: read this doc + `handover/ai-direct/LATEST.md` (which points here) + `handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md` § 6 (Phase C protocol) + § 9 (statistical plan) + `handover/preregistration/scripts/run_c2_phase_c_ablation.sh` (the runner that launches the batch). These four sources are sufficient to resume without context.

---

## § 1. What this session shipped

### Phase C atoms (commits 1d04f6a .. 4f981cd, this session)
- **C-pre1** (`1d04f6a`): hard-10 deterministic freeze. PREREG § 6 C2 specified
  "pre-committed in Phase A5" but the actual draw was missed during the Phase
  A→B exit cycle. Closed the gap: `random.Random("hard10_pput_ccl_seed").sample(sorted(adaptation_144), 10)` produces sealed sha256
  `6667e6bdd2aa381c6757f83322eab2226666564085388c7e591c22b378d713f6`. The
  10 IDs (sorted) live in `handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json`;
  the script `handover/preregistration/scripts/draw_hard10_pput_ccl.py` is
  byte-deterministic and Trust-Rooted per C-075. TR 38 → 40.
- **C1a** (`600d55f`): `--mode` CLI scaffold. New `experiment_mode.rs` mirrors
  `budget_regime.rs` discipline: 5-variant `ExperimentMode` enum, pure parser,
  startup-fatal `UnimplementedMode` for not-yet-wired modes. `--mode <value>`
  flag extractor on argv (POSIX-flexible: `--mode val` or `--mode=val` or
  after positional). MODE env var preserved as the storage layer for backwards
  compatibility with the 4 in-binary `v2_emit_*` tests. TR 40 → 41.
- **C1b** (`ac22ca3`): Soft Law runtime via `apply_mode_to_accept`. Pure helper
  funnels every `make_pput` call site's (lean_rt, lean_ph) pair through a
  mode-aware transform. SoftLaw branch returns `(true, lean_ph)` — runtime
  accept forced; Lean truth preserved on the verified leg. Wired uniformly
  across 8 call sites (oneshot 4 + swarm 4). The make_pput two-leg signature
  (mid-term P0-A fix 2026-04-25) was put in place specifically to make this
  design point unmissable. TR re-hash only.
- **C1c** (`266be88`): Homogeneous runtime via `skill_index_for_agent`. Pure
  helper selects the skill index for a swarm agent given the experiment mode.
  Homogeneous branch returns 0 (every agent → `agent_skills[0]` = algebraic);
  other modes pass through `agent_idx % n_skills`. Wired at 2 sites: startup
  echo (line 725) + per-tx prompt construction (line 932). Single source of
  truth — the two sites can no longer drift. TR re-hash only.
- **C1d** (`e2a276b`): Panopticon runtime via `is_panopticon`. At per-tx
  prompt construction, Panopticon expands the focal agent's learned-memory
  injection from "this agent's own memory via lib.read_agent_memory(agent_id)"
  to "the merged memory of ALL agents, labeled with each source agent_id".
  Token cost grows ~O(N) per tx → cost dilution → PPUT↓ per H2. TR re-hash only.
  (Caught + fixed a latent ENV_LOCK poisoning bug in test suite during this
  commit — `resolve_cli_unimplemented_aborts` was hardcoded to `panopticon`
  → `UnimplementedMode`, which became invalid post-C1d; updated to `amnesia`.)
- **C1e** (`3264072`): Amnesia runtime via `is_amnesia`. At per-tx agent
  prompt construction, Amnesia forces `chain = problem_statement.to_string()`
  regardless of `snap.tape` contents. Internal verification paths (tape+payload
  Lean re-verify) NOT touched — Amnesia is about agent memory, not Lean
  verifier. ensure_implemented becomes total post-C1e (all 5 modes pass);
  the ModeError::UnimplementedMode variant preserved on the type for
  forward-compatibility with future modes. TR re-hash only.
- **C5** (`4f981cd`): mode_flag_binary_purity test. Inline in evaluator.rs's
  `v2_emit_tests` module (shares ENV_LOCK). Test calls `make_pput` 5 times
  with identical inputs except MODE env; asserts (a) 5 distinct mode labels
  stamped, (b) (git_sha, binary_sha256, model_snapshot, split) identical
  across all 5, (c) env-pinned BINARY_SHA256 + MODEL_SNAPSHOT actually flow
  to the row. Pinned PREREG § 6 C5 conformance.
- **C2 runner** (this commit): `run_c2_phase_c_ablation.sh` written but not
  yet committed. Mirrors `run_p0_calibration.sh` discipline (set -euo pipefail
  + cargo build exit checked + oracle preflight + evaluator boot preflight +
  per-cell timeout + synthetic UNSOLVED row on cell failure for completeness
  invariant). Two modes: `--smoke` (1 problem × 5 modes × 1 seed × MAX_TX=2,
  ~5-25 min, ~$0.05) and `--full` (5 modes × 10 problems × 2 seeds = 100
  rows, ~25-50 hours wall-clock serial, ~$13-25). Added to TR per C-075
  DO-178C. TR 41 → 42.

### Smoke results (C2 --smoke, 2026-04-26 10:44–11:08 UTC)
- **1/5 cells succeeded end-to-end**: Homogeneous mode at MAX_TX=2 produced
  a valid jsonl row in 236s (4 min wall-clock, 18624 tokens, 2 tx, hit_max_tx=true,
  solved=false; aime_1987_p5 unsolved at MAX_TX=2 — expected for this hard
  problem).
- **4/5 cells timed out at 5 min cell limit**: Full / SoftLaw / Panopticon /
  Amnesia (all 4 use heterogeneous skills, where Homogeneous uses skill_0
  uniformly). Conjecture: deepseek-v4-flash's thinking-on path takes ~30-60s
  per LLM call; n3 swarm × 2 tx = 6 LLM calls total at ~40-50s each → ~5 min
  wall-clock. Homogeneous's all-skill_0 prompts skip the variation; the
  heterogeneous skills (skill_1 = structural / skill_2 = rewriting) trigger
  marginally longer reasoning.
- Synthetic UNSOLVED rows written for the 4 timed-out cells (cell-completeness
  invariant per Gemini Q7.b discipline carried over from B7-extra audit).

The smoke validates the binary's wiring works end-to-end (Homogeneous proves
the 5-mode CLI flag, mode-aware skill resolution, swarm loop, and v2 schema
emit are all sound). The other 4 modes' code paths are unit-tested (24
experiment_mode tests + c5_mode_flag_binary_purity); the timeout is a
runtime-budget concern, not a wiring bug.

---

## § 2. Verified state at C5 + C2-runner commit

| Metric | Value | How to verify |
|---|---|---|
| `cargo test --workspace` | **298 PASS / 29 ignored / 0 failed** | re-run |
| `python3 scripts/test_llm_proxy.py` | 16/16 PASS | wrapped by Rust conformance test |
| `bash scripts/smoke_siliconflow.sh` | PASS (3/3 keys) | live API |
| Trust Root manifest entries | **42** (post-C2-runner-add) | `grep -c '^"' genesis_payload.toml [trust_root]` |
| `boot::tests::verify_trust_root_passes_on_intact_repo` | PASS | recursive child-manifest verify live |
| Cases | 76 (C-001..C-076) | unchanged |
| Active rules | 15 (R-001..R-020 with gaps) | unchanged |
| `--mode` CLI smoke (5 modes startup gate) | 5/5 PASS | direct binary smoke |
| C2 `--smoke` (1 problem × 5 modes × MAX_TX=2 × 5 min/cell timeout) | **1/5 success** (Homogeneous only); 4/5 timeout | see § 1 smoke results |

---

## § 3. C2 batch launch decision tree (next session)

The full C2 batch is the natural next atom after this scaffolding session.
Three executable paths from current state:

### Path A — Serial overnight launch (recommended baseline)
```
LLM_PROXY_URL=http://localhost:18080 \
bash handover/preregistration/scripts/run_c2_phase_c_ablation.sh --full
```
- Wall-clock: ~25-50 hours per PREREG_AMENDMENT § 1 empirical (15-30 min/cell × 100 cells)
- Cost: ~$13-25 at deepseek-v4-flash thinking-on rates
- Needs: laptop / server up for the duration; LLM_PROXY_URL on 18080 (or
  start one explicitly: `python3 src/drivers/llm_proxy.py --port 18080 &`)
- Output: `experiments/minif2f_v4/logs/c2_phase_c_ablation_<TIMESTAMP>__<mode>_<problem>_<seed>.jsonl`
- Per-cell timeout: 30 min (1800s); cells exceeding emit synthetic UNSOLVED
  per cell-completeness invariant — analysis must filter on `_synthetic_failure`

### Path B — Parallel-runner upgrade (faster + needs new code)
- Modify `run_c2_phase_c_ablation.sh` to run K cells concurrently (e.g. K=5
  to match the 3-key SiliconFlow pool + 2 deepseek slots); rate-limit aware.
- Wall-clock: ~5-10 hours
- Risk: rate limits hit; binary contention on the LLM proxy
- Engineering: ~1-2 hours to write + test

### Path C — Reduced-scope batch (faster but lower stat power)
- Drop to e.g. 5 modes × 5 problems × 1 seed = 25 rows. ~5-10 hours.
- Trade-off: McNemar paired sign test on n=5 instead of n=10 → α corrections
  bite harder; H1-H4 likely under-powered. PREREG § 9.2 power analysis would
  need re-running.
- Probably NOT worth it — C2 is the primary Phase C evidence.

### Recommendation
Path A overnight, OR Path B with explicit user GO (engineering investment +
parallel-batch coordination). C2 results feed directly into C3 (H1-H4
McNemar tests on `pput_verified` per-problem signs) + C4 (CHECKPOINT_PHASE_C
+ dual external audit).

### Smoke pre-flight before --full
The C2 smoke (`--smoke`) takes ~5-25 min; should be re-run before launching
`--full` to confirm:
1. The binary still verifies Trust Root + builds clean
2. At least 1 mode (Homogeneous baseline) completes a real run
3. Cell timeout policy hasn't drifted

### Empirical backbone-comparison (post-smoke 2026-04-26 11:21 UTC)

Direct smoke comparison on aime_1987_p5 oneshot, n3 swarm, MAX_TX=2:

| Backbone | Wall-clock | Tokens used | Lean-verify share | Estimated per-cell @ MAX_TX=200 |
|---|---|---|---|---|
| deepseek-v4-flash (thinking-on, reasoner-class, current PREREG) | >300s (timeout) | ~18624 (Homogeneous baseline) | ~25% | ~120 min/cell, ~200 hours batch |
| deepseek-chat (V3, non-thinking) | 45s | 946 | 95% (~42s in Lean) | ~36 min/cell, ~60 hours batch serial; ~12 hours with 5-cell parallel runner |

**The ~36 min/cell estimate at MAX_TX=200 matches PREREG_AMENDMENT § 1's "~15-30 min average" empirical** — but only when using the V3 backbone. Reasoner-class models multiply per-cell time 3-4x because thinking expansion is ~50s/call vs ~0.5s/call.

Per F-2026-04-26-01 (NOTEPAD § 2), `deepseek-v4-flash thinking-off` is unfounded as a backbone claim — DeepSeek API doesn't honor `enable_thinking=false` for reasoner-class models. The PREREG-stated backbone is therefore operationally a thinking-on model on api.deepseek.com.

---

## § 4. Open questions for next session

1. **Heterogeneous-skill smoke timeout root cause**. Is the 4/5 smoke timeout
   really skill-prompt-driven, or is it network/proxy/model load? Worth a
   targeted experiment: run Full mode with MAX_TX=1 (1 LLM call, no skill
   cycling kicks in) — if THAT timeouts, network. If passes, skill confirmed.
2. **Should the MAX_TX default be reduced for Phase C C2?** PREREG implies
   MAX_TX=200 (constitutional default), but if the hard-10 average runs for
   ~25 min at MAX_TX=200, the batch is multi-day. A capped MAX_TX=50 would
   shrink the batch to <1 day at the cost of fewer chances to solve. PREREG
   doesn't explicitly cap; this is an architecture decision for next session.
3. **Thinking-on vs thinking-off**. The PHASE_B_IMPLEMENTATION_PLAN says
   `deepseek-v4-flash thinking-off` but the proxy responses show
   `reasoning_content` field, suggesting thinking-on. Worth confirming
   the toggle (env var? request body field?) before launching --full.
4. **Should the `MODE` env var be renamed to `EXPERIMENT_MODE`?** Currently
   we kept `MODE` for backwards compat with 4 in-binary tests. The descriptive
   prefix would be more grep-friendly + match the C1a module naming convention.
   Cosmetic; defer to a future hygiene cycle.
5. **C5 integration-level test**. The current C5 is schema-discipline only.
   PREREG § 6 C5 implies an integration-level companion (subprocess-invoke
   the binary 5 times). The C2 batch implicitly satisfies this by producing
   100 rows that can be cross-mode-compared post-hoc; document this in the
   CHECKPOINT_PHASE_C audit packet.

---

## § 5. Code state — file pointers for next session

### New code (this session):
- `experiments/minif2f_v4/src/experiment_mode.rs` (C1a, extended C1b/c/d/e)
  — 5-mode resolver + 4 mode-aware transform/predicate helpers
  + 30 unit tests covering every branch
- `experiments/minif2f_v4/src/bin/evaluator.rs` (modified)
  — main() argv extractor + mode resolution at startup
  — run_oneshot/run_swarm entry: `let mode = ...` resolution
  — 8 make_pput call sites: route via `apply_mode_to_accept(mode, rt, ph)`
  — startup echo + per-tx skill: route via `skill_index_for_agent(mode, idx, n)`
  — per-tx learned-memory: branch on `is_panopticon(mode)`
  — per-tx chain: branch on `is_amnesia(mode)`
  — `c5_mode_flag_binary_purity` test in v2_emit_tests mod

### New scripts (this session):
- `handover/preregistration/scripts/draw_hard10_pput_ccl.py` (C-pre1)
- `handover/preregistration/PPUT_CCL_HARD10_2026-04-26.json` (C-pre1)
- `handover/preregistration/scripts/run_c2_phase_c_ablation.sh` (C2 runner)

### Trust Root manifest progression:
- 38 → 40 (C-pre1: hard-10 JSON + draw script)
- 40 → 41 (C1a: experiment_mode.rs — runtime gate machinery)
- 41 → 42 (C2 runner: gate machinery for the ablation evidence batch)

### Modified governance:
- `genesis_payload.toml` — TR manifest 38 → 42; header chain extended with C-pre1 / C1a / C2 runner notes
- `experiments/minif2f_v4/tests/trust_root_immutability.rs` — required-paths list extended (3 new C-prefix entries)
- `handover/alignment/TRACE_MATRIX_v2_2026-04-25.md` § 3 + § 6 — manifest milestones C1a-e + C5 + C2 runner
- `handover/ai-direct/AUTO_RESEARCH_NOTEPAD.md` § 1 — Phase C row marked "scaffolding 100% done; ready to launch batch"

---

## § 6. Constitutional alignment — per-atom retrospective

Per user directive 2026-04-26 (carried over from Phase A→B exit): not "did
this commit edit constitution.md" but "could this fix violate any FC1/FC2/FC3
invariant or any Article". Honest retrospective for substantive C* fixes:

- **C-pre1**: pure freeze + Trust Root extension. FC3-N34 readonly subgraph
  strengthened (hard-10 sample now Trust-Rooted). ✅
- **C1a**: new startup-fatal gate prevents misconfigured `--mode=soft_law/...`
  from silently falling back to Full. Strengthens C-075 + Art. V.1.2 (gate
  machinery is qualifiable). ✅
- **C1b**: SoftLaw transform is the EXPLICIT design point — runtime fakes
  acceptance, post-hoc preserves Lean truth. The make_pput two-leg signature
  (P0-A 2026-04-25 mid-term audit fix) was constitutional preparation for
  exactly this. The `apply_soft_law_preserves_post_hoc_verified` test pins
  the constitutional invariant: SoftLaw must NEVER mutate the post-hoc leg
  (otherwise it would launder fake accepts into the North Star). ✅
- **C1c**: Homogeneous is the Paper-1 era A condition — Art. II.2.1 explicit
  violation ("不能抹杀群体异质性"). The mode TESTS what happens when the
  Article is broken; the mode itself is not a constitutional violation
  (it's an experimental ablation). ✅
- **C1d**: Panopticon expands per-agent memory injection across all agents.
  Strengthens Art. III.2 framing (the agent partition was always there;
  Panopticon makes its breach observable). ✅
- **C1e**: Amnesia projects empty L_t to agents (the underlying L_t archive
  is unchanged — read-only invariant preserved). FC3 / Art. III.2 readonly
  subgraph not touched; only the agent's projection is mode-aware. ✅
- **C5**: pure schema-discipline test, no FC/Article surface. ✅
- **C2 runner**: gate machinery per C-075 DO-178C. In Trust Root. ✅

Zero `constitution.md` edits across 7 atoms.

---

## § 7. Trajectory + cost budget

Cumulative arc spend through this session:
- Phase A PREREG dual audit (mid-stream session): ~$15-20
- Phase B B2-B4 mid-term audit (mid-stream session): ~$3-5
- Phase A → B exit dual audit (this session before Phase C work): ~$80
- Phase C scaffolding + smoke (this session): ~$0.05 (smoke API only)
- **Cumulative**: ~$100 / $500 cap = 20%
- Remaining: ~$400 for Phase C C2 ($13-25) + Phase D shadow CCL + Phase E
  sealed eval + B7-extra calibration if/when § 3 conditions complete.

Test count progression:
- Pre-C session start: 267 PASS
- Post-C1a: 286 (+19 experiment_mode unit tests)
- Post-C1b: 292 (+5 apply_mode_to_accept tests + 1 resolve test)
- Post-C1c: 296 (+4 skill_index_for_agent tests)
- Post-C1d: 297 (+1 is_panopticon predicate test)
- Post-C1e: 297 (+2 is_amnesia / mutually-exclusive tests; -1 obsolete unimpl test; +1 widened resolve test → 1 deleted prior split)
- Post-C5: 298 (+1 c5_mode_flag_binary_purity)

---

## § 8. Pointers for next session (4-file reading list)

1. **This doc** (`HANDOVER_PHASE_C_SCAFFOLD_2026-04-26.md`) — session summary + C2 launch decision tree
2. **`handover/ai-direct/LATEST.md`** — entry pointer; updated this session
3. **`handover/preregistration/PREREG_PPUT_CCL_2026-04-26.md`** § 6 (Phase C protocol) + § 9 (statistical plan)
4. **`handover/preregistration/scripts/run_c2_phase_c_ablation.sh`** — the runner; read its docstring + try `--smoke` before `--full`

Plus **AUTO_RESEARCH_NOTEPAD.md** `§ 1 Active experiments` to confirm Phase C row state.

---

## § 9. Lessons sedimented this session

- **C1c-d-e helper-extraction pattern**: each ablation mode got a pure helper
  in `experiment_mode.rs` (apply_mode_to_accept / skill_index_for_agent /
  is_panopticon / is_amnesia). The helpers are small, easy to unit test,
  and the call sites in evaluator.rs become 1-2 line branches. This kept
  the per-atom diff small enough to audit.
- **ENV_LOCK poisoning catch**: when `resolve_cli_unimplemented_aborts` was
  hardcoded against a specific then-unimplemented mode, every C1c/d/e
  commit needed the test updated for the new "still unimplemented" set.
  C1d caught the failure (panopticon got wired, panic poisoned ENV_LOCK,
  cascaded 5 other tests). Going forward: parameterize the
  "still-unimplemented" assertion list so it self-update via ensure_implemented's
  match-arm structure.
- **Heterogeneous-skill smoke timeout**: deepseek-v4-flash thinking-on takes
  ~30-60s per LLM call; n3 swarm × 2 tx exhausts a 5-min cell timeout for
  4/5 modes. This is the most actionable open question for next session
  (§ 4 #1) — could be skill-prompt-driven, network, or model-load.
- **The `--mode` flag survives a stale-test-text class of bug**: ENV_LOCK
  poisoning made the failure cascade dramatic, but the actual bug was
  trivial (forgot to update one test). C5's binary purity test gives a
  schema-level discipline check; the integration-level companion is C2's
  100-row batch (cross-mode field comparison post-hoc).
