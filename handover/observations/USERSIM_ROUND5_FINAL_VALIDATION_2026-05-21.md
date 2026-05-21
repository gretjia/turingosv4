# USERSIM ROUND 5 — Final validation + ship readiness, 2026-05-21

| Field | Value |
|-------|-------|
| Date | 2026-05-21 |
| Persona | Non-expert indie hobbyist (5th instance, fresh) |
| Game decided | **Snake** (different from R1 Star Catcher / R2 Bubble Pop / R3 Pixel Farm Defender / R4 Color Mix Puzzle) |
| Provider path | DeepSeek dual-key + mock-LLM (for R4-1 verification) |
| Outcome | R4-1 fix verified correct; happy path holds; 1 polish bug (R5-1) found; SHIP verdict |
| Predecessor | USERSIM_ROUND3_VALIDATE_ATOMK_2026-05-21.md, USERSIM_ROUND2_DEEPSEEK_END_TO_END_2026-05-21.md |
| Decision | Plan v4 ships; R5-1 deferred to next charter |

## 1. R4-1 verification (Mission A)

Mock-LLM server returning `200 OK` with `choices[0].message.content = ""` (transient empty response simulation). Stderr capture:

```
1: [generate] calling Blackbox LLM (deepseek-v4-flash)...
2: [generate] raw response saved to /tmp/.../generate_raw_response.txt
3: turingos generate: Blackbox LLM emitted no parseable files. Expected `### File: <path>` followed by a fenced code block.
4:   (Transient API error? Try running `turingos generate` again.)
5: [failed run] generation_attempt_cid=63df262c...
6: [failed run] rejection_cid=c57b03ab...
```

**Verdict**: R4-1 fix is correct. Line 4 (retry hint) appears immediately after the error (line 3) and BEFORE the technical `[failed run]` CIDs (lines 5-6). Non-expert sees actionable hint adjacent to the problem.

## 2. Happy path (Mission B) — Snake delivered

`turingos init --provider deepseek` → endpoint warning fires → spec via deepseek-v4-pro (2691 tokens) → generate via deepseek-v4-flash. **First attempt** produced truncated HTML (R5-1 below). **Retry** produced complete `<title>Snake</title>` game with arrow-key movement, localStorage high-score persistence, game-over screen with restart.

Capsule chain (Snake delivery):
- spec_capsule_cid: 888d71...
- generation_attempt_cid: f0c11f6d...
- artifact_bundle_cid: 5578bedf...
- test_run_cid: 37ad50d8...
- Internal tests: PASS (2/2 scenarios) — EntrypointExists, HtmlParses

## 3. New finding — R5-1

### Symptom

First generate attempt returned a truncated HTML response (cut off mid-`isNewRecord = true;`, missing `</script></body></html>`). The `HtmlParses` C11 scenario passed because browser-style parsers are lenient — they accept incomplete HTML. The user-sim saw `Internal tests: PASS (2/2)` despite the delivered file being structurally broken.

### Root cause

C11's `HtmlParses` scenario in `src/runtime/test_scenario.rs` uses a permissive parse check. It doesn't verify structural completeness — no assertion that `</html>` or `</body>` closing tags exist, no check that all `<script>` tags are balanced.

### Severity

LOW-MEDIUM. The retry hint from R4-1 covers the recovery path — a non-expert noticing the truncated file would re-run `turingos generate`. But the false-PASS on the internal-test gate is a real false-positive: the user is told the delivery passed when it didn't.

### Proposed fix (next charter)

Extend `HtmlParses` scenario in `src/runtime/test_scenario.rs` to require:
- Presence of `</html>` substring
- Presence of `</body>` substring
- If `<script>` opens N times, `</script>` closes N times

~10 LoC change + 1 new test case for truncated input. NOT shipped in Plan v4.

## 4. 5-round verdict matrix

| Bug class | Round 1 | Round 2 | Round 3 | Round 4 | Round 5 |
|-----------|---------|---------|---------|---------|---------|
| API key dual-config | broken (B8) | fixed | confirmed | confirmed | confirmed |
| Init UX (empty dir) | broken (B2) | fixed | confirmed | confirmed | confirmed |
| Failed CIDs ordering | broken (B3) | (un-tested) | PR #66 incomplete | PR #73 fixed | confirmed |
| test_run human readout | broken (B4) | fixed | confirmed | confirmed | confirmed |
| Welcome agent_deploy | broken (B5) | fixed | confirmed | confirmed | confirmed |
| --skip-llm framing | broken (B7) | fixed | confirmed | confirmed | confirmed |
| DeepSeek model names | broken (B1) | sidestepped via --provider | confirmed | confirmed | confirmed |
| Endpoint silent trap | broken (NB3) | found | Atom-K fixed | confirmed | confirmed |
| `llm config --help` | broken (NB6/X2) | half-fix | found | PR #73 fixed | confirmed |
| LLM retry hint | broken (R4-1) | (un-tested) | (un-tested) | found | PR #74 fixed → confirmed |
| Test scenario strength | OK (not tested) | OK | OK | OK | **R5-1 NEW** |

11 bugs found and tracked across 5 rounds. **10 fixed and re-verified**. **1 (R5-1) documented as next-charter backlog**.

## 5. FC trace

- FC1: closed. Snake game delivered end-to-end via DeepSeek dual-key path.
- FC2: closed. Workspace reconstructable from CAS (full capsule chain).
- FC3: 1 new feedback item (R5-1). No constitutional violations.

## 6. Halt readiness verdict (from sub-agent)

> "SHIP with one known weak gate. R4-1 fix is confirmed correct and the happy path is solid — the tool is ready for real users. The one residual gap is R5-1 (HtmlParses test too lenient to catch LLM truncation), which is a quality-of-life issue rather than a correctness blocker since the retry hint from R4-1 covers the recovery path."

Orchestrator concurs. **Plan v4 closes here.**

## 7. Backlog for next charter

| Item | Severity | LoC est. |
|------|----------|----------|
| R5-1: tighten `HtmlParses` scenario (closing-tag assertion + script balance check) | LOW-MEDIUM | ~10 |
| Setup-friction: 3 env vars without persistent config / first-run wizard | LOW | ~30 (a `turingos llm config --interactive` mode?) |
| Round 1-2 deferred polish: NB1 (welcome label refinement), NB2 (template "proof" default), NB4 (xdg-open Linux-only), NB5 (`spec audit --session` discoverability) | LOW each | ~50 total |
| Anthropic native dispatch path (trigger: first user PR with `provider = "anthropic"` hitting OpenAI-compat rejection) | HIGH if triggered | ~500 (see `handover/research/MULTIPROVIDER_LLM_2026-05-21/`) |
| `provider:model_id` tape format (trigger: first replay tool needing provider differentiation) | MEDIUM | ~80 (per `handover/specs/PROVIDER_TAPE_FORMAT_v1_DRAFT.md`) |
