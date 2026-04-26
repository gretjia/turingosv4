#!/usr/bin/env python3
"""Phase A atom A7 — per-key SiliconFlow probe.

Invoked by `scripts/smoke_siliconflow.sh`. Reads the three keys from
env (`SILICONFLOW_API_KEY` / `_SECONDARY` / `_TERTIARY`), issues one
tiny chat-completion call per key, and reports OK/FAIL per key WITHOUT
printing any key material. Exits non-zero if any configured key fails.

Cost bound: 3 calls × ~50 tokens. Qwen2.5-7B-Instruct on SiliconFlow
free tier is the cheapest stable option (V3L-27 N=30 collapse caveat
applies only at high concurrency; one call per key is safe).
"""
import os
import sys
import time

try:
    from openai import OpenAI, APIStatusError, RateLimitError
except ImportError:
    print("[A7-smoke] FAIL: openai SDK not installed (pip install openai)")
    sys.exit(2)

KEY_ENVS = [
    ("primary", "SILICONFLOW_API_KEY"),
    ("secondary", "SILICONFLOW_API_KEY_SECONDARY"),
    ("tertiary", "SILICONFLOW_API_KEY_TERTIARY"),
]
BASE_URL = "https://api.siliconflow.cn/v1"
# Qwen2.5-7B-Instruct: smallest stable production model on SF free tier.
# Avoids expensive reasoning models during probe.
PROBE_MODEL = "Qwen/Qwen2.5-7B-Instruct"
PROBE_PROMPT = "Reply with the single word: ack"
PROBE_MAX_TOKENS = 8


def probe_one(label: str, env_name: str, key: str) -> tuple[bool, str]:
    """Return (ok, summary). Never returns the key in `summary`."""
    client = OpenAI(api_key=key, base_url=BASE_URL)
    t0 = time.time()
    try:
        resp = client.chat.completions.create(
            model=PROBE_MODEL,
            messages=[{"role": "user", "content": PROBE_PROMPT}],
            temperature=0.0,
            max_tokens=PROBE_MAX_TOKENS,
            stream=False,
        )
    except RateLimitError as e:
        return False, f"RateLimitError ({type(e).__name__}): {str(e)[:120]}"
    except APIStatusError as e:
        return False, f"APIStatusError {getattr(e, 'status_code', '?')}: {str(e)[:120]}"
    except Exception as e:
        return False, f"Error {type(e).__name__}: {str(e)[:120]}"
    dt_ms = int((time.time() - t0) * 1000)
    msg = resp.choices[0].message
    content = (msg.content or "").strip()
    usage = resp.usage
    pt = getattr(usage, "prompt_tokens", "?") if usage else "?"
    ct = getattr(usage, "completion_tokens", "?") if usage else "?"
    return True, (
        f"{dt_ms}ms; tokens prompt={pt} completion={ct}; "
        f"content[:32]={content[:32]!r}"
    )


def main() -> int:
    print(
        f"[A7-smoke] SiliconFlow probe — model={PROBE_MODEL} "
        f"max_tokens={PROBE_MAX_TOKENS}"
    )
    # A8e11 fix P1 (Codex R10#1): the V3L-27 mitigation requires THREE
    # keys for high-concurrency rate-limit pool splitting (case C-027).
    # Pre-P1 the smoke would PASS if only the primary key was set,
    # silently degrading the gate to a single-key probe and matching
    # exactly the V3L-27 collapse pattern.
    # Post-P1: ALL THREE keys must be set AND respond. Missing OR
    # failing key = FAIL. Explicit env-var opt-out
    # `SILICONFLOW_SMOKE_ALLOW_PARTIAL=1` lets a developer probe a
    # subset (e.g. for credential rotation testing); the bypass is
    # logged loudly and the result is downgraded to a partial-PASS
    # exit code that callers can distinguish.
    any_failed = False
    missing: list[str] = []
    failed: list[str] = []
    succeeded: list[str] = []
    for label, env_name in KEY_ENVS:
        key = os.environ.get(env_name, "").strip()
        if not key:
            missing.append(label)
            print(f"  [{label:9s}] {env_name}: NOT SET")
            continue
        ok, summary = probe_one(label, env_name, key)
        verdict = "OK  " if ok else "FAIL"
        print(f"  [{label:9s}] {env_name}: {verdict} {summary}")
        if ok:
            succeeded.append(label)
        else:
            failed.append(label)
            any_failed = True

    allow_partial = os.environ.get("SILICONFLOW_SMOKE_ALLOW_PARTIAL", "") == "1"

    if missing or any_failed:
        print(
            f"[A7-smoke] result: FAIL — "
            f"missing={missing or '(none)'} failed={failed or '(none)'} "
            f"ok={succeeded or '(none)'}"
        )
        if missing:
            print(
                "[A7-smoke] V3L-27 mitigation requires ALL 3 SiliconFlow "
                "keys configured (primary + secondary + tertiary). "
                "A single-key probe replicates the very collapse pattern "
                "the 3-key pool was meant to mitigate."
            )
        if allow_partial and not failed:
            print(
                "[A7-smoke] SILICONFLOW_SMOKE_ALLOW_PARTIAL=1 — explicit "
                "downgrade accepted. Exit code 3 = partial-PASS (not full)."
            )
            return 3
        return 1
    print("[A7-smoke] result: PASS (3/3 keys configured + responded)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
