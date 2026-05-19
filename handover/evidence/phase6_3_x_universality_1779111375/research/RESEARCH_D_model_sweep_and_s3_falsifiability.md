# Research-D: Model Sweep Design + Software 3.0 Falsifiability Framework

**Returned**: 2026-05-18 by clean-context Opus
**Duration**: 407s, 30 tool uses

---

## Part I — Model-Size Sweep

### SiliconFlow Catalog (May 2026)

TuringOS defaults are mid-generation. Current catalog:

| Model ID | Params | Context | ¥/M In | ¥/M Out | Reasoning surface |
|---|---|---|---|---|---|
| `deepseek-ai/DeepSeek-V4-Flash` | 284B-MoE/13A | 1024K | 1 | 2 | Think modes (extra_body) |
| `deepseek-ai/DeepSeek-V3.2` *(Meta default)* | 671B | 160K | 2 | 3 | non-thinking |
| `deepseek-ai/DeepSeek-V3.1-Terminus` | 671B | 160K | 4 | 12 | think/non-think |
| `deepseek-ai/DeepSeek-R1` | 671B-MoE | 64K | — | — | **always `<think>...</think>`** |
| `Qwen/Qwen3-8B` | 8.2B | 128K | 0.42 | 0.42 | dual; **thinking-on by default** |
| `Qwen/Qwen3-14B` | 14.8B | 128K | 0.49 | 1.96 | dual |
| `Qwen/Qwen3-32B` | 32.8B | 128K | 0.98 | 3.99 | dual |
| `Qwen/Qwen3-Coder-30B-A3B-Instruct` *(Blackbox default)* | 30B-A3B | 256K | low | low | code MoE; thinking-off |
| `Qwen/Qwen3.5-35B-A3B` | 35B/3A | 256K | 1.6 | 12.8 | thinking, multimodal |
| `Qwen/Qwen3.5-122B-A10B` | 122B/10A | 256K | 2 | 16 | thinking-on default |
| `Qwen/Qwen3.6-27B` | 27B | 256K | 1.8 | 14.4 | dense + vision |
| `Qwen/Qwen3.6-35B-A3B` | 35B/3A | 256K | 1.6 | 12.8 | MoE dual |
| `Pro/zai-org/GLM-4.7` | 355B/32A | 198K | 4 | 16 | enhanced thinking |
| `Pro/zai-org/GLM-5` | 744B/40A | 198K | 4 | 22 | long-context agent |
| `Pro/zai-org/GLM-5.1` | 754B | 198K | 6 | 28 | agent-optimized |
| `Pro/moonshotai/Kimi-K2.5` | 1T/32A | 256K | 4 | 21 | dual reasoning |
| `Pro/moonshotai/Kimi-K2.6` | 1T/32A | 256K | 6.5 | 27 | 4000+ tool calls |
| `MiniMaxAI/MiniMax-M2.5` | 229B-MoE | 192K | 2.1 | 8.4 | strong coding/agent |

### CLI surface (verified, not guessed)

- Update config: `turingos llm config --workspace <PATH> --meta-model <ID> --blackbox-model <ID>` (`cmd_llm.rs:41-45, 237-244, 299-302`)
- Show: `turingos llm show --workspace <PATH>` (`cmd_llm.rs:46, 262-277`)
- Complete: `turingos llm complete --workspace <PATH> --role <meta|blackbox> --prompt-file <PATH|-> [--strict-json] [--max-tokens N] [--temperature F] [--capsule-dir ...] [--turn-id ...] [--meta-prompt ...] [--lang zh|en]`
- Triage: `turingos llm triage --workspace <PATH> --user-answer <STRING|-> [--question ...] [--capsule-dir ... --turn-id ...] [--lang zh|en]`

### CRITICAL Parser Sensitivity Findings

1. **`complete --strict-json` does NOT strip `<think>`** — `cmd_llm.rs:746-755` calls parse directly on `chat_result.content`. Any thinking-mode model breaks at P1.
2. **`triage`'s `strip_thinking_wrapper`** (cmd_llm.rs:1183-1185, 1309-1316) only splits at **last** `</think>`. Unclosed leak.
3. **Robust `strip_think_blocks` exists** at `src/sdk/protocol.rs:107` (iterative, handles unclosed) — **NOT wired into `complete`**. Easy reuse.
4. **`ChatRequest`** (`siliconflow_client.rs:70-78`) has NO `extra_body` field — cannot turn off Qwen3 dense thinking from the API.
5. **No `response_format`** field — cannot use OpenAI-compat `json_schema` constrained decoding (which would eliminate ~70% of envelope-parse failures on small models).
6. **180s timeout** at line 170 — fine for non-think, R1 thinking can exceed.
7. **GLM/Kimi-K2 dual modes** emit reasoning in separate `reasoning_content` field; client only reads `choices[0].message.content` — wastes spend, may PASS envelope.

### Meta-role Matrix

| Model | Hypothesis | Risk |
|---|---|---|
| Qwen3-8B | FAIL (parse + think leak) | high |
| Qwen3-32B | partial PASS | medium |
| Qwen3-Coder-30B-A3B | PASS envelope; weak Voss mirror | medium |
| DeepSeek-V3.2 (default) | PASS baseline | low |
| DeepSeek-V3.1-Terminus (think-off) | PASS | low |
| DeepSeek-V3.1-Terminus (think-on) | **FAIL P1** until strip added | high |
| DeepSeek-V4-Flash | PASS; 2x faster, ½ price | low |
| GLM-4.7 | unknown reasoning_content risk | medium |
| Kimi-K2.5 | PASS; wasted spend | low |
| MiniMax-M2.5 | likely PASS | low |

### Blackbox-role Matrix

| Model | Hypothesis |
|---|---|
| Qwen3-8B | partial PASS; 4-class accuracy unknown |
| Qwen3-14B | likely PASS (sweet spot) |
| Qwen3-Coder-30B-A3B (default) | PASS baseline |
| Qwen3.6-27B | PASS; better gibberish detection? |
| DeepSeek-V3.2 | PASS; overkill bracket |
| GLM-4.7 | unknown reasoning_content interference |

### Bound-Finding Pairs (priority)

1. **Meta P1-envelope bound**: Qwen3-32B (barely fail; think leak) vs Qwen3-Coder-30B-A3B (barely pass; clean JSON). If 32B fails, **proves extra_body gap load-bearing**.
2. **Meta semantic bound**: DeepSeek-V4-Flash (cheaper) vs DeepSeek-V3.2. If V4-Flash matches at 50% cost → swap default.
3. **Meta capability ceiling**: Kimi-K2.5 vs MiniMax-M2.5 — does 1T-MoE add anything beyond V3.2?
4. **Blackbox cost-floor**: Qwen3-8B vs Qwen3-14B — smallest model holding triage ≥95%.
5. **Reasoning-trace falsifier**: DeepSeek-R1 or V3.1-Terminus think-on. **Expected to fail with current code.** Useful regression test forever.

### Mechanical Metrics (per session)

- `envelope_parse_rate` = strict-json exit-0 / total
- `slot_coverage_at_term` = |covered ∩ REQUIRED| / 7
- `turn_count_to_term`
- `triage_class_accuracy` on N hand-labeled
- `mean_tokens_per_response` (usage.completion_tokens)
- `mean_latency_ms` (elapsed_ms)
- `p1_retry_rate`
- `vocab_violation_rate` — set-diff against CANONICAL_SLOTS
- `monotonicity_violation_rate`
- `cost_per_session_¥` = Σ(prompt×in_price + completion×out_price)

### Anti-recommendations
- Skip pure-vision Qwen3-VL (text grill, wasted spend)
- Skip Z-Image variants (wrong category)
- Skip Qwen3-235B-A22B (256K ctx wasted on bounded grill)
- DeepSeek-R1 think-on only as parser-hole regression test, NOT a production candidate, until A1 ships
- Skip Pro/* SLA tier until non-Pro establishes baseline

---

## Part II — Software 3.0 落地评估框架

### 10 S-Predicates (S3.0 "lands" iff all hold)

| # | Predicate | Test method | Pass criterion |
|---|---|---|---|
| **S1** | Non-engineer (Mrs Chen) produces spec.md that downstream codegen accepts without engineer rewrite | Wave 1 baseline → `turingos generate` → does it compile + run Tetris MVP? | ≥70% spec.md codegen-able without manual edit |
| **S2** | Prompt-only edit fixes observed defect (the program IS the prompt) | Triage wave: induce defect, edit `grill_meta_v1.md` only (no Rust), re-run | Zero Rust LOC delta; bug observably gone |
| **S3** | Meta swaps gracefully across model size/family | Wave 6 sweep, ≥5 Meta models | slot_coverage_at_term ≥ 0.85 on ≥3 of 5 |
| **S4** | Blackbox swaps with stable triage labels | Wave 6 triage sub-sweep, N≥10 labeled | accuracy ≥0.95 on ≥2 of 3; κ ≥0.8 on gibberish/abusive boundary |
| **S5** | Adversarial degrades gracefully (no crash, no false termination) | Wave 4 | Zero panics; zero done=true with required slots un-covered; all routed to abusive/gibberish |
| **S6** | Replay-without-recall holds (S3.0 observationally pure given CAS) | Wave 7: 5 past sessions re-derive view, no LLM re-call | Byte-equal reconstruction |
| **S7** | Envelope contract canonical; no LLM-as-judge | static `rg "chat_complete.*LLM-as-judge"` | Zero hits; all 6 predicates pure Rust |
| **S8** | Capability bound TIGHT not asymptotic | Bound-pair #1 (Part I) | One swap-in <0.5 parse rate; one ≥0.95; bimodal |
| **S9** | LLM agency in choosing next question (closes Researcher A §1 "Software 1.5" critique) | 8× same opener → distinct turn-2 questions | Diversity ≥4/8; no `i==8` fixed loop |
| **S10** | Cost per session bounded + FC1 attempt-equality invariant | Wave 1+6 token counts | cost ±20% of prediction; Researcher B §1.4 invariant holds |

### TuringOS vs Karpathy Fit Analysis

**Alignments**:
- Prompt as program (grill_meta_v1.md is 65-line interviewer source)
- LLM as kernel (Rust handles memory paging, bounded exec, predicate gates)
- Context engineering (structured-history shielding)
- Build for agents (JSON-on-stdout, Karpathy "llms.txt for CLIs")

**Gaps Karpathy hints at, TuringOS misses**:
- Computer-use / agentic tool harness (grill→generate is text-only)
- Jagged intelligence acceptance (Meta is monolithic, no per-turn model routing)
- Prompt self-edit on observed failure

**Architectural divergences**:
- JSON envelope vs free-form NL — **justified** by FC1 + shielding; risk: should move to wire-layer constrained decoding when SiliconFlow exposes
- Predicates + retry + Blackbox — **not a regression**; matches Karpathy "context engineering happens AROUND the LLM"
- Blackbox split is *more* S3.0-faithful than monolithic
- Retry-once with fixed suffix — could be more S3.0-pure with prompt-self-heal

### "差什么" — Production-Readiness Gap Inventory

#### G1: Self-improvement loop
- Current: human reads transcripts → hand-edits prompt
- Missing: `prompt-evolution` skill ingesting test regressions + capsule failures → proposes v2 diff with quantitative claim
- Why: S2 enabled but not automated; Karpathy endgame = prompt as versioned CI artifact

#### G2: Multi-prompt orchestration beyond Meta+Blackbox
- Current: exactly 2 roles
- Missing: synthesis prompt parametrized; adversarial sanity-check role; cooking/canvas/code-review presets
- Why: Karpathy S3.0 artifacts (Cursor/v0/GPT-Engineer) are all multi-prompt

#### G3: Prompt versioning + A/B runtime switch
- Current: hash in PromptCapsule audit only; --meta-prompt flag informational
- Missing: workspace TOML `llm.meta.prompt_path`; canary routing; capsule `prompt_canary_bucket`
- Why: S3.0 production needs safe rollout

#### G4: User-correctable prompt mid-session
- Current: prompt loaded at startup, immutable mid-grill
- Missing: WS side-channel `user_meta_directive`
- Why: native S3.0 systems treat user as co-author

#### G5: Cross-session memory
- Current: each session blank
- Missing: `--memory-from <session-id>` loading prior playback into read_set
- Why: "grill remembers Mrs Chen from last week" is baseline expectation
- Class-3 (touches read_set scoping)

#### G6: Code-side schema vs prompt text drift
- Current: CANONICAL_SLOTS in Rust + repeated as Markdown in prompt
- Missing: build-time grep check, or `{{CANONICAL_SLOTS}}` substitution at load
- Why: Software 2.0 / 3.0 collision point

#### G7: Wire-layer structured-output discipline
- Current: ChatRequest has no `response_format` field
- Missing: passthrough of provider-native `response_format: json_schema`
- Why: ~70% of small-model envelope failures eliminable at wire layer

#### G8: Reasoning-trace handling (THE parser hole)
- Current: complete path doesn't strip <think>; triage handles last-only
- Missing: wire `protocol.rs::strip_think_blocks` into complete; support reasoning_content field
- Why: Wave 6 sweep on any thinking model breaks immediately

### Recommended Next 3-5 Atoms (Phase 6.3.y)

| # | Atom | Risk | LOC | S-predicates closed | Sequencing |
|---|---|---|---|---|---|
| **A1** | strict-json `<think>` strip + reasoning_content hoist | Class 2 | ~50 | S3, S7, S8 | **Ship before Wave 6** |
| **A2** | `turingos llm prompt-eval` regression harness | Class 1 | ~250 | S2, G1 | After Wave 1 baseline fixtures |
| **A3** | `response_format: json_schema` wire passthrough | Class 2 | ~80 | S7, G7 | After A1 |
| **A4** | TOML `llm.meta.prompt_path` + `prompt_canary_bucket` | Class 2 | ~120 | S2, G3 | After A2 |
| **A5** | Arbitrary role lookup `llm.<role_id>.model` | Class 2 | ~80 | G2, debate mode | Gated on insufficient-2-roles signal |

### Campaign Artifacts to Produce

- `campaign_s3_falsification_matrix.md` (S1..S10 rows × wave/evidence-CID/verdict)
- `wave6_model_sweep/<model_id>/session.jsonl`
- `wave6_model_sweep/RESULTS.md`
- `fixtures/grill_qa_corpus.jsonl`
- `fixtures/triage_gold_labels.jsonl` (N≥30)
- `tests/grill_session_replay_byte_identical.rs`
- `tests/fc1_grill_attempt_equality.rs`
- `assets/prompts/grill_meta_v2.md` *only if* A2 shows measurable improvement

## Key file paths
- src/bin/turingos/cmd_llm.rs (CLI + complete/triage + parser hole)
- src/bin/turingos/siliconflow_client.rs (client; no extra_body/response_format)
- src/runtime/grill_envelope.rs (TurnPayload + canonical/required slots)
- src/sdk/protocol.rs:107 (robust strip_think_blocks — not wired into complete)
- src/drivers/llm_proxy.py:294-325 (different driver, has DeepSeek-thinking handling)

## Sources
- siliconflow.cn/models (May 2026 fetch)
- siliconflow.com/models international mirror
- Karpathy "Software Is Changing (Again)" YC AI Startup School June 2025 + latent.space writeup
- docs.siliconflow.cn/en/userguide/capabilities/text-generation
