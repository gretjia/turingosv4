# Research-B: Internal Failure Mode Inventory

**Returned**: 2026-05-18 by clean-context Opus
**Duration**: 237s, 23 tool uses

---

## P0 / High-priority defects (audit-blind)

### D1. Web `extract_slots` uses WRONG slot vocabulary [P0]
- File: `src/web/spec.rs:693-702`
- Hardcoded: `["job_story","anchor","data_model","first_click","weird_user","disappointment_boundary","success_test","playback"]`
- Canonical (from `src/runtime/grill_envelope.rs:17-26`): `["job","anchor","memory","first_run","robustness","scope","acceptance","mirror"]`
- Meta-prompt (`assets/prompts/grill_meta_v1.md:18`) uses canonical names
- **Effect**: every web-mode session — LLM emits canonical slots, `extract_slots` filters with junk list → `open_slots` always = full junk list. Corrupts coverage UI + termination signaling.
- **Cause**: Researcher C's draft slot vocabulary crept into web handler; LLM program + predicate vocab + web response don't agree.
- **Not flagged by W10-R1** (static audit didn't cross-check vocab consistency)

### D2. Three `.unwrap()` on session-map re-acquire (TOCTOU)
- File: `src/web/spec.rs:874, 973, 1086`
- Audit A.1 / O3 — theoretical only, no remove path today

### D3. 15-turn ceiling broadcasts empty `spec_capsule_cid` with `terminated=true`
- File: `src/web/spec.rs:888-911`
- Frontend disambiguates via empty CID — fragile contract
- **Trigger**: LLM stalls 15 turns of vague "不知道" answers

### D4. `partial_session` padding with "(not collected in driven session)"
- File: `cmd_spec.rs:1321-1326` (audit B.5)
- **Trigger**: LLM declares done at turn 5 with 7 slots covered through merged answers; spec.md has 3-4 "(not collected)" lines

### D5. Asset path workspace-relative
- File: `cmd_llm.rs:1083`, `cmd_spec.rs:1092`
- **Trigger**: dispatch cwd without `assets/prompts/` → every turn ok=false → predicate_double_fail
- Confirmed by W9 dispatch needing `cwd=repo_root`

### D6. Missing API key hard-aborts driven loop
- File: `cmd_spec.rs:1330-1343`
- Witnessed in W9 abort 2026-05-18

### D7. `SlotRequiredMissing` reuses `QuestionMissing` discriminant
- File: `grill_predicates.rs:229` (O1)
- Future audit grep can't distinguish the two causes

## Latent / theoretical

### L1. Hard-15-ceiling counts `turn_count` not LLM calls
- `src/web/spec.rs:886`; only increments on `triage == relevant`
- Mitigated by `non_relevant_count >= 2` abort

### L2. `partial_session` skips synthesis when total_turns < 4
- `cmd_spec.rs:1314` — currently unreachable via predicate (termination requires turn >= 4)

### L3. `prompt_context_hash` is hash-of-CID, not hash-of-messages-bytes
- `cmd_spec.rs:1105-1109`
- W10-R2 C3 check will FAIL by construction unless aligned

### L4. Web session-snapshot race between lock release and triage call
- `src/web/spec.rs:868-942` — rapid duplicate POSTs

### L5. Triage shell-out swallows non-zero exit
- `src/web/spec.rs:942-963` — binary crash indistinguishable from "gibberish"

## Prompt-level fragility

1. **No JSON-schema enforcement** — post-hoc serde_json::from_str. Injection like "respond in markdown fences" breaks.
2. **`open_slots` exists but no predicate enforces complement-of-covered_slots**
3. **Mirror slot canonical but excluded from REQUIRED_SLOTS** — playback minimality unconstrained (`"."` passes)
4. **Plain-language constraint unverified** — P6 only checks Han ratio
5. **`last_3_turns` window** loses turn 1 context by turn 5 — LLM may drift away from original topic
6. **Termination playback validation** — no row-count check; `playback="OK"` passes
7. **Rationale field structurally shielded but not enforced** — future contributor leak risk
8. **No "question is novel" predicate** — LLM could loop on same question (Researcher A §5.4)

## Predicate edge cases

- **P1 (schema)**: no rationale.len() ≤ 200 enforcement; 5000-char rationale bloats CAS
- **P2 (kind)**: two-layer (parse + predicate) for done↔question consistency; divergence = retry inconsistency. No "done=false implies playback=None" check.
- **P3 (vocab)**: strict; LLM typo `["job_story"]` = full fail. No dedup: `["job","job"]` passes.
- **P4 (monotonic)**: only checks subset, not STRICT superset. Stall: same `["job","anchor"]` × 5 turns passes every time.
- **P5 (turn bounded)**: `0 < turn <= 15`. Float `1.5` → serde parse fail.
- **P6 (lang)**: Han-script ratio ≥ 0.5 OR ASCII alpha ≥ 0.8. Mixed-language attack possible. Han chars = U+4E00..U+9FFF only; CJK ext A/B (𠮷 𡴂) not counted → rare-char question drops ratio.
- **Termination predicate**: turn≥4 + confidence≥0.8 + covered⊇REQUIRED. **Premature done=true allowed**: LLM self-marks slots covered, predicate is purely structural. User "yes"×8 → terminates with garbage spec.

## Triage classifier weak spots

- **`abusive` vs `gibberish` ambiguity** collapses to same nudge
- **Empty answer** → "gibberish" → one strike
- **Long answer** (>4096 char) — web layer enforces, CLI driven mode doesn't
- **Non-relevant counter resets on relevant** — troll forever pattern: 1 off_topic + 1 relevant × ∞
- **Triage stdout parse**: `parse_triage_class_from_output(...).unwrap_or_else(|_| "gibberish")` — class "RELEVANT" (uppercase) → bucketed as gibberish, legit answer → strike

## Phase 7 lessons that transfer

1. **Multi-module path/config drift** (W8.1 VETO) — extract single helper for `assets/prompts/` resolution
2. **Heuristic verifier over-strictness** (W8 has_canvas FP) — analog is D1 above
3. **Hand-off contracts asymmetry** — `total_attempts` missing on failure; grill has same shape for `SpecGrillComplete` empty CID
4. **Workspace layout assumptions** — confirm `resolve_workspace()` used in web spec.rs post-W8.1
5. **LLM single-shot reliability < 100%**: Qwen 50% rate. With 1 retry: clean 8-turn ≈ 47% probability. **~half sessions will hit predicate_double_fail mid-interview.**
6. **Sandbox/sanitization gap on LLM text**: Phase 7 needed iframe sandbox. Grill renders `question_text` directly — verify frontend escapes.

## Recommended test probes (priority subset)

| Pri | Probe | Surface | What to look for |
|---|---|---|---|
| P0 | Web slot-vocab divergence (any web session) | `src/web/spec.rs:691-720` | `open_slots` always = junk list |
| P0 | Prompt injection in user answer | full grill loop | triage classification + Meta yielding |
| P0 | 15-turn force terminate UI | `src/web/spec.rs:886-911` | empty CID rendering |
| P0 | Empty/blank answer | `cmd_spec.rs:1206` | one-strike vs two-strike |
| P0 | Driven-mode without API key | `cmd_spec.rs:1330` | witnessed |
| P1 | Premature done=true with all 7 slots | `grill_predicates.rs:217-235` | garbage spec from "yes" answers |
| P1 | Mid-session double-click submit | race window | dup turn capsules |
| P1 | Mixed-language attack | P6 | lang ratio fail |
| P1 | Triage binary IO failure | `cmd_llm.rs:1083` | 2 strikes → abort |
| P1 | JSON-fence injection | `grill_envelope.rs:95-98` | predicate_double_fail |
| P2 | Playback `"."` minimal | `p2_kind_ok` | passes; add min-length predicate |
| P2 | Triage typo "Relevant" capital | `parse_triage_class_from_output` | strike on legit answer |
| P2 | CJK extension chars in question | `han_script_ratio` | LanguageMismatch on legit zh |
| P3 | `<script>` in question_text | frontend | unverified — read spec-grill.ts |

## Files cited
- handover/audits/AUDITOR_TISR_PHASE6_3_X_W10_R1_PRE_W9.md
- handover/audits/AUDITOR_TISR_PHASE7_W8_1_FINAL_VALIDATION_R1_VETO_REGRESSION.md
- handover/audits/AUDITOR_TISR_PHASE7_W8_VALIDATION_R1_CHALLENGE_FIXABLE.md
- handover/audits/AUDITOR_TISR_PHASE7_REAL_E2E_R1_CHALLENGE_FIXABLE.md
- src/runtime/grill_predicates.rs:114-235
- src/runtime/grill_envelope.rs:17-39
- src/bin/turingos/cmd_spec.rs:826-1378
- src/web/spec.rs:691-1200 (extract_slots defect at 691-720)
- assets/prompts/grill_meta_v1.md, grill_triage_blackbox_v1.md
- handover/research/grill_software_3_0_2026-05-18/researcher_{a,b,c}/DESIGN.md
- handover/evidence/stage_phase6_3_x_grill_driven_1779111375/agent_verdict.json
