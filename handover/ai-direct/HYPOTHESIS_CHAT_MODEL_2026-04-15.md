# Hypothesis: TuringOS replaces model internal CoT (chat > reasoner)

**Proposed by**: User (2026-04-15 ~04:30 UTC)  
**Status**: Queued for v3.2 experiment post-M4

## Claim

TuringOS architecture (tape, market, Boltzmann routing, recent_errors broadcast, wallet) is an externalized Chain-of-Thought. Running reasoning models (deepseek-reasoner, GPT-o1, Claude Opus thinking) inside TuringOS is double-reasoning: internal CoT + external scaffold = redundant.

Therefore: **prefer chat models** for TuringOS agents. Cheaper (~4-8×), faster (~5-10×), and the architecture compensates for lower per-call reasoning depth.

## Mapping (TuringOS ↔ CoT)

| Architectural component | CoT analog |
|---|---|
| Tape (linear nodes) | Thought steps |
| Market (node prices) | Beam-search / heuristic eval |
| Recent_errors broadcast | Self-correction |
| Boltzmann routing | Temperature / exploration |
| Wallet (Coin budget) | Token budget |
| Multi-agent | Multi-sampling |

## Testable prediction

**chat + TuringOS will use `append` more than reasoner + TuringOS**.

Reasoning model internally produces full proof → direct `complete`, bypasses tape.  
Chat model cannot produce full proof one-shot → forced to use tape to accumulate partial progress.

## Constitutional framing

- **C-034 "mechanism > prompt"**: model choice is a mechanism. Selecting chat-model forces collective machinery to earn its keep.
- **C-033 emergence requires causal proof**: chat-model test distinguishes "scaffold contributes" from "reasoner carries".
- **C-031 institution > tuning**: model choice is institution-level, not parameter-tweaking.
- **Art. II.2 broadcast price signals + II.2.1 exploration**: these mechanisms are designed to aid *cooperation among limited agents*, not to coordinate already-complete individual reasoners.

## Proposed v3.2 design

- **Sample**: identical (seed=74677, N=50, fingerprint=796ead6c40351ae9) — enables direct paired comparison
- **Conditions**: oneshot, n1, n3 (same three)
- **Model**: `ACTIVE_MODEL=deepseek-chat` (only change vs v3.1)
- **Prompt/schema/timeout**: unchanged
- **Abort gate**: unchanged (10-problem / 3-timeout)

### New metric (v3.2 only)

Append-usage rate: `tape_depth_at_OMEGA / total_solves`. Measures how much each condition relied on tape vs direct-complete.

- If `append_rate(chat) > append_rate(reasoner)` → thesis confirmed (scaffold activates when model is weaker)
- If `append_rate(chat) ≈ 0` → model too weak to use scaffold meaningfully; different failure mode
- If `append_rate(chat) < append_rate(reasoner)` → unexpected; thesis falsified

## Expected wall time + cost

- reasoner v3.1 (ongoing): ~12h, ~$25 API
- chat v3.2 (planned): ~75 min, ~$4 API

**10× efficiency gain at equal sample size.** If SolveRate holds up, thesis is confirmed and TuringOS becomes dramatically cheaper.

## Decision gate

v3.2 executes only if:
1. v3.1 M4 audit = PASS (Codex + Gemini)
2. v3.2 plan passes dual audit
3. No unresolved URGENT_*.md in handover

## Side implications if thesis confirmed

- All future TuringOS benchmarks should use chat models as default
- Reasoner is an upper-bound control, not production choice
- C-031 precedent extends: model-class selection is institutional design

## Side implications if thesis falsified

- TuringOS scaffold is INSUFFICIENT to replace internal CoT
- Architecture engineering should focus on making tape/market actually contribute (append incentives per C-034)
- Current reasoner+TuringOS "+33% PPUT" was likely just k-sample advantage, not architecture value
