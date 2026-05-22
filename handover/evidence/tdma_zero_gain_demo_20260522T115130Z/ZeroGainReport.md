# TDMA-Bounded-RC1 Atom 11 — Zero-Gain Escalation Demo

Demonstrates the zero_gain fuse (KILL-tdma-8) firing on a realistic stuck-LLM scenario.

**Setup**: judge advanced to Step 5 of Nesbitt's proof; worker produces 4+ consecutive direction-reversal failures (same signature, same predicate).

**Expected behavior**: kernel escalates at attempt ZERO_GAIN_K+1=4 with reason=`ZERO_GAIN`, NOT `MAX_RETRIES` (which would fire at attempt 6+).

## Outcome

- Escalated: **true**
- Escalated at attempt: **4** (expected 4)
- Reason: **ZERO_GAIN** (expected `ZERO_GAIN`)
- Timing correct: **true**
- Reason correct: **true**
- Escalation beat MAX_RETRIES=5: **true** (so it was the zero_gain fuse, NOT the retry counter)

## zero_gain_streak history per attempt

```
attempt 1: zero_gain_streak = 0
attempt 2: zero_gain_streak = 1
attempt 3: zero_gain_streak = 2
attempt 4: zero_gain_streak = 3
```

Expected progression: 0, 1, 2, [escalate before reaching attempt that would log streak ≥ ZERO_GAIN_K=3].

## Prompt size per retry attempt

```
attempt 1: prompt = 384 tokens
attempt 2: prompt = 384 tokens
attempt 3: prompt = 384 tokens
```

## Verdict

**Overall: PASS**
- per_attempt_probes.jsonl sha256: 06fe5cbdf3afde7a629f416848b2b32cb2c957a422a6a71c6b620536b351dc8e
- chaintape.jsonl sha256: 3021e1c3bfbfce3834392fa12b836e2990790b6517cd42919e558ca5188ba3fe
