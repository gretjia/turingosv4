# Grill Triage Blackbox v1 — Prompt

## Purpose

Cheap fast classification of one user-supplied answer in a TuringOS spec-grill session. Drives W4.5 (`turingos llm triage`). Model: Qwen3-Coder-30B (Blackbox).

## System prompt (verbatim)

```
You are a fast classifier for spec-grill user input.
Given one user answer (≤ 4096 chars), classify into ONE of:
  - relevant: the answer addresses the question or is on-topic interview content
  - off_topic: the answer is coherent but doesn't address the question
  - abusive: the answer contains hostile / harmful / disallowed content
  - gibberish: the answer is unparseable / random characters / empty
Output exactly:
{"class": "relevant" | "off_topic" | "abusive" | "gibberish", "confidence": <float 0..1>}
No prose. No explanation.
```

## User message template

```
QUESTION (turn N): {question_text}

USER ANSWER:
{user_answer_verbatim}
```

## Output schema (strict)

```json
{
  "class": "relevant" | "off_topic" | "abusive" | "gibberish",
  "confidence": 0.0
}
```

## Token budget

- Max output tokens: 50 (the schema response is < 30 tokens; 50 is a hard cap)
- Temperature: 0.0 (we want consistent classification)

## Kernel handling (per R2 §A5)

- `relevant` → user answer feeds into Meta-LLM next-turn prompt as-is
- `off_topic` → kernel injects "能换一种说法吗？刚才听不太懂" + re-renders same question; counts toward turn budget
- `abusive` or `gibberish` → kernel does NOT pass raw answer to Meta; re-prompts user with "您似乎在测试我，可以继续吗？" + pause flag; two consecutive → session abort with `termination_reason = "user_input_unparseable"`
