# Wave Runner Agent Brief — Template

**Purpose**: each persona / scenario in Waves 1-5 dispatches one of these. Replace `{{PLACEHOLDERS}}` per-persona.

---

You are a Phase 6.3.x universality test runner. Run ONE driven-grill session as a specific persona, capture full evidence, return verdict. Clean-context (no implementation transcript).

## Identity

- Repo: `/Users/zephryj/work/turingosv4`
- Branch: `codex/tisr-phase6-3-x-grill-driven` (read-only git; do NOT switch/commit/push)
- Backend: `http://127.0.0.1:8080` (already running, orchestrator-started; do NOT start/stop)
- Wave/persona: **{{WAVE}}/{{PERSONA_ID}} — {{PERSONA_NAME}}**
- Evidence dir (you write here): `handover/evidence/phase6_3_x_universality_1779111375/{{WAVE_DIR}}/{{PERSONA_ID}}/`
- API key: pre-exported in backend env; you don't need to handle it

## Persona spec (verbatim from Research-{{A|C}})

{{PERSONA_PARAGRAPH_VERBATIM}}

## Turn-by-turn answer sequence

{{ANSWER_LIST or "Improvise per persona spec; aim to cover 7 required slots if persona is cooperative; otherwise let triage/predicates handle it"}}

## Pipeline (5 steps; same shape for every persona)

1. **Create workspace**:
   ```bash
   WS=/tmp/wave_{{WAVE}}_{{PERSONA_ID}}_$(date +%s)
   mkdir -p "$WS"
   # Copy turingos.toml from repo's tmp/phase7_active/ for default config
   cp /Users/zephryj/work/turingosv4/tmp/phase7_active/turingos.toml "$WS/" 2>/dev/null || \
     printf 'llm.provider = "siliconflow"\nllm.meta.model = "deepseek-ai/DeepSeek-V3.2"\nllm.meta.api_key_env = "SILICONFLOW_API_KEY"\nllm.blackbox.model = "Qwen/Qwen3-Coder-30B-A3B-Instruct"\nllm.blackbox.api_key_env = "SILICONFLOW_API_KEY"\n' > "$WS/turingos.toml"
   echo "$WS" > "$EVIDENCE_DIR/workspace_path.txt"
   ```

2. **Generate session_id + bootstrap**:
   - There is NO separate `/start` endpoint. Sessions are created implicitly by the first POST.
   - Pick a session_id like `wave{{WAVE}}_{{PERSONA_ID}}_$(date +%s)` (must match `^[a-zA-Z0-9_-]{1,128}$`).
   - **Bootstrap POST**: `curl -X POST http://127.0.0.1:8080/api/spec/turn -H 'Content-Type: application/json' -d '{"session_id":"...","user_answer":null,"lang":"zh"}'`
   - This returns Q1 in `question_text`. If bootstrap fails: write verdict FAIL, exit.

3. **Submit answers turn-by-turn**:
   - For each answer in the sequence (or improvised based on Q):
     - POST `http://127.0.0.1:8080/api/spec/turn` with body `{"session_id":"<same id>","user_answer":"<answer text>","lang":"zh"}`
     - Capture full HTTP response (status, body)
     - Append to `session_log.jsonl` as `{"turn":N,"q_text":"...","user_answer":"...","response":{...},"elapsed_ms":...}`
     - Response shape: `{turn_index, question_text, covered_slots, open_slots, confidence, done, playback, terminated, spec_capsule_cid, turn_capsule_cid}`
     - If response `terminated: true` → stop submitting (synthesis happened); record `spec_capsule_cid`
     - If response `done: true` AND `terminated: false` → keep going (playback confirmation pending; submit "没问题" or similar)
     - If turn count > 16 → halt, mark suspect (15-turn ceiling should have fired)
     - If response is 5xx → halt, mark FAIL

4. **CAS audit walk**:
   ```bash
   cd /Users/zephryj/work/turingosv4
   ./target/debug/turingos spec audit --workspace "$WS" --session "$SESSION_ID" > "$EVIDENCE_DIR/cas_walk.txt" 2>&1
   echo "exit=$?" >> "$EVIDENCE_DIR/cas_walk.txt"
   ```
   Verify (best-effort): session-rollup capsule + N turn capsules + final spec capsule present.

5. **Metrics + verdict**:
   - Parse `session_log.jsonl` to compute:
     - `total_turns`
     - `terminated_at_turn`
     - `terminate_reason` (extract from final response: llm_done_predicate_pass / predicate_double_fail / turn_limit_forced / user_input_unparseable)
     - `covered_slots_at_term` (from final response)
     - `triage_class_distribution` (relevant / off_topic / abusive / gibberish counts, if visible in responses)
     - `envelope_parse_success_rate` (turns where backend responded 200 with parsed envelope)
     - `mean_latency_ms` (mean of elapsed_ms)
     - `cost_proxy_completion_tokens` (sum if visible; else N/A)

   Write `verdict.json`:
   ```json
   {
     "wave": "{{WAVE}}",
     "persona_id": "{{PERSONA_ID}}",
     "persona_name": "{{PERSONA_NAME}}",
     "verdict": "PASS" | "PARTIAL" | "FAIL",
     "expected_behavior": "{{EXPECTED}}",
     "suspected_failure": "{{SUSPECTED}}",
     "actual_behavior": "<your prose summary>",
     "metrics": { ... },
     "anomalies": [ ... ],
     "session_id": "...",
     "workspace": "...",
     "evidence_files": [ "session_log.jsonl", "cas_walk.txt", "verdict.json" ],
     "started_at_unix": ...,
     "completed_at_unix": ...
   }
   ```

## Verdict criteria

- **PASS**: session terminated successfully with `covered_slots ⊇ REQUIRED_SLOTS` AND `final_spec_capsule_cid` present AND no 5xx during run AND behavior matches expected
- **PARTIAL**: session ran without crash, but expected behavior not fully met (e.g., adversarial scenario contained but slot coverage uneven)
- **FAIL**: 5xx storm OR session aborted with user_input_unparseable OR final spec absent when expected OR clear regression vs baseline

## Tools

ALLOWED: `Bash` (curl, jq, turingos CLI, mkdir, cp, date, grep, sort, awk), `Read`, `Write` (ONLY into your evidence dir), `Glob`, `Grep`. Read-only git.

FORBIDDEN: `Edit`, `WebSearch`, `WebFetch`, `git commit/push/checkout/reset`, browser MCP (this run is curl-based; orchestrator separately runs sanity-check browser sessions), any backend start/stop.

## Anti-hallucination

- All metrics must come from `session_log.jsonl` you captured; never fabricate numbers
- If a value isn't observable, write `null` or `"unknown"` in verdict.json, not a guess
- If the session crashes mid-run, still write verdict.json with `verdict=FAIL`, `metrics` populated for turns completed, `anomalies` describing the crash

## Time budget

10 min wall clock. Most sessions are 4-10 LLM turns × 5-15s each = 1-3 min for the loop, plus walk + verdict = total ~5 min.

Begin.
