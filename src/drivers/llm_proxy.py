#!/usr/bin/env python3
"""
LLM Proxy v4 — OpenAI-compatible local HTTP server with token metering.

Phase A atom A7. Adapted from v3's `src/drivers/llm_proxy.py` with one
load-bearing v4 change: per-provider multi-key round-robin so the three
SiliconFlow keys (SILICONFLOW_API_KEY / _SECONDARY / _TERTIARY) split
concurrent traffic across separate rate-limit pools — the V3L-27
N=30 → 401/429 collapse documented in `cases/V3_LESSONS.md` was
single-key. The same pattern extends to other providers if multiple
keys are configured.

Endpoints:
  POST /v1/chat/completions  (OpenAI-compatible, forwards to cloud APIs)
  GET  /health
  GET  /stats               (token counters + per-key request distribution)
  POST /stats/reset         (reset counters — call before each experiment)

Usage:
  SILICONFLOW_API_KEY=sk-xxx \\
  SILICONFLOW_API_KEY_SECONDARY=sk-yyy \\
  SILICONFLOW_API_KEY_TERTIARY=sk-zzz \\
    python3 src/drivers/llm_proxy.py --port 8080

Without --provider, model identity drives routing:
  - "deepseek-*" → deepseek
  - "Qwen/...", "openai/...", anything containing "/" → siliconflow
  - else → dashscope
"""
import os, sys, json, logging, argparse, time, threading, itertools
from http.server import HTTPServer, BaseHTTPRequestHandler
from socketserver import ThreadingMixIn
from openai import OpenAI, RateLimitError, APIStatusError

logging.basicConfig(level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')
log = logging.getLogger("llm_proxy")

# Each provider entry: (base_url, [env-var names tried in order]).
# Multiple env names = multi-key round-robin. The PRIMARY name MUST be
# first; any later names are optional fallback / additional pool keys.
PROVIDERS = {
    "dashscope": (
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
        ["DASHSCOPE_API_KEY"],
    ),
    "aliyun": (
        "https://dashscope.aliyuncs.com/compatible-mode/v1",
        ["DASHSCOPE_API_KEY"],
    ),
    "siliconflow": (
        "https://api.siliconflow.cn/v1",
        [
            "SILICONFLOW_API_KEY",
            "SILICONFLOW_API_KEY_SECONDARY",
            "SILICONFLOW_API_KEY_TERTIARY",
        ],
    ),
    "deepseek": (
        "https://api.deepseek.com",
        [
            "DEEPSEEK_API_KEY",
            # Phase C C2 dual-key launch (2026-04-26): user provided a 2nd
            # key to enable K=4 parallel-pool runs without single-key
            # rate-limit margin risk. Mirrors the 3-key SiliconFlow pattern
            # from A7. The proxy round-robins across whichever subset is
            # configured at startup.
            "DEEPSEEK_API_KEY_SECONDARY",
        ],
    ),
    "volcengine": (
        "https://ark.cn-beijing.volces.com/api/v3",
        ["VOLCENGINE_API_KEY"],
    ),
    "nvidia": (
        "https://integrate.api.nvidia.com/v1",
        ["NVIDIA_NIM_API_KEY"],
    ),
}

# Per-(provider, key-index) OpenAI client cache: provider -> list[OpenAI]
clients_by_provider = {}
# Round-robin counter per provider.
_rr_counters = {}
_rr_lock = threading.Lock()
# Per-key request counters for /stats observability.
_per_key_requests = {}  # provider -> list[int]


def _build_clients(provider):
    """Return list of OpenAI clients for `provider`, one per available key.

    Lazy. Caches in `clients_by_provider`. Raises ValueError if NO key
    is set for the provider.
    """
    if provider in clients_by_provider:
        return clients_by_provider[provider]
    base_url, key_envs = PROVIDERS[provider]
    keys = []
    for env_name in key_envs:
        v = os.environ.get(env_name, "").strip()
        if v:
            keys.append((env_name, v))
    if not keys:
        raise ValueError(
            f"No keys set for provider={provider}; tried env vars {key_envs}"
        )
    clients = [OpenAI(api_key=k, base_url=base_url) for (_, k) in keys]
    clients_by_provider[provider] = clients
    _per_key_requests[provider] = [0] * len(clients)
    log.info(
        f"[provider {provider}] resolved {len(clients)} key(s) from envs: "
        f"{[name for (name, _) in keys]}"
    )
    return clients


def get_client_round_robin(provider):
    """Return (client, key_index) using round-robin across configured keys."""
    clients = _build_clients(provider)
    with _rr_lock:
        idx = _rr_counters.get(provider, 0) % len(clients)
        _rr_counters[provider] = idx + 1
        _per_key_requests[provider][idx] += 1
    return clients[idx], idx


# ── Token Metering ──
_stats_lock = threading.Lock()
_stats = {
    "prompt_tokens": 0,
    "completion_tokens": 0,
    "total_tokens": 0,
    "requests": 0,
    "errors": 0,
    "retries_429": 0,
    "estimated_count": 0,
}


def _record_usage(prompt_tokens, completion_tokens):
    with _stats_lock:
        _stats["prompt_tokens"] += prompt_tokens
        _stats["completion_tokens"] += completion_tokens
        _stats["total_tokens"] += prompt_tokens + completion_tokens
        _stats["requests"] += 1


def _record_estimated():
    with _stats_lock:
        _stats["estimated_count"] += 1


def _record_error():
    with _stats_lock:
        _stats["errors"] += 1


def _record_retry():
    with _stats_lock:
        _stats["retries_429"] += 1


def _reset_stats():
    with _stats_lock:
        for k in _stats:
            _stats[k] = 0
        for prov in _per_key_requests:
            _per_key_requests[prov] = [0] * len(_per_key_requests[prov])


def _get_stats():
    with _stats_lock:
        snap = dict(_stats)
        snap["per_key_requests"] = {
            p: list(v) for p, v in _per_key_requests.items()
        }
        return snap


# ── Rate Limiter ──
_rate_lock = threading.Lock()
_rate_semaphore = threading.Semaphore(int(os.environ.get("LLM_PROXY_CONCURRENCY", "5")))
_cooldown_until = 0.0


def _wait_for_cooldown():
    global _cooldown_until
    now = time.time()
    if now < _cooldown_until:
        wait = _cooldown_until - now
        log.info(f"[RATE LIMITER] Cooling down {wait:.1f}s")
        time.sleep(wait)


def _trigger_cooldown(seconds):
    global _cooldown_until
    with _rate_lock:
        new_until = time.time() + seconds
        if new_until > _cooldown_until:
            _cooldown_until = new_until
            log.warning(f"[RATE LIMITER] Global cooldown {seconds}s")


def detect_provider(model):
    """Route by model identifier. v4 prefers explicit `provider:model`
    syntax (e.g. `siliconflow:Qwen/Qwen2.5-7B-Instruct`); falls back
    to model-string heuristics for backward compat with v3 callers.

    Routing matrix (A8e fix F3 + A8e6 fix K2):
      explicit `provider:...`            → that provider (if known)
      slash-separated id (huggingface-style "Org/Model")
                                          → siliconflow (dispatches the
                                            full catalog including Qwen,
                                            openai-compat, Meta, AND
                                            `deepseek-ai/<distill>` SF-
                                            catalog DeepSeek variants —
                                            see K2 note below)
      bare contains "deepseek"           → deepseek (api.deepseek.com)
      bare "qwen3-*" / "qwen-*"          → dashscope (Aliyun Qwen direct)
      else                                → dashscope (default fallback)

    Round-1 audit caught a routing inversion: `Qwen/Qwen2.5-7B-Instruct`
    used to misroute to dashscope because `m.startswith("qwen")` won
    after the slash check. F3 promoted slash-form above bare-qwen.

    Round-6 audit (Codex R6#2) caught a related collision: HuggingFace-
    style `deepseek-ai/DeepSeek-R1-Distill-Qwen-7B` is a SiliconFlow-
    catalog model (the official DeepSeek API only serves the canonical
    `deepseek-chat` / `deepseek-v4-flash` family, not the Distill
    variants). The pre-K2 logic checked `"deepseek" in m` BEFORE the
    slash check, so `deepseek-ai/...` misrouted to api.deepseek.com
    and 404'd. K2 swaps the order: slash-form is now the FIRST routing
    heuristic (after explicit prefix), and bare-deepseek substring
    only fires when there's no slash.
    """
    if ":" in model:
        prefix = model.split(":", 1)[0].lower()
        if prefix in PROVIDERS:
            return prefix
    if "/" in model:
        # Any slash-form (Qwen/..., openai/..., meta-llama/...,
        # deepseek-ai/...) goes to the heterogeneous catalog provider.
        # K2: this branch is FIRST so HuggingFace-style identifiers
        # never silently fall through to a bare-string provider that
        # doesn't actually serve them.
        return "siliconflow"
    m = model.lower()
    if "deepseek" in m:
        # Bare "deepseek-chat", "deepseek-v4-flash" → official endpoint.
        # Slash-form like "deepseek-ai/..." won't reach here per K2.
        return "deepseek"
    if m.startswith("qwen"):
        # Bare qwen3-*, qwen-* without slash = direct DashScope catalog.
        return "dashscope"
    return "dashscope"


def strip_provider_prefix(model):
    """If model is `provider:foo/bar`, return `foo/bar`; else `model`."""
    if ":" in model:
        prefix, rest = model.split(":", 1)
        if prefix.lower() in PROVIDERS:
            return rest
    return model


class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == "/health":
            self._json_response(200, {"status": "ok"})
        elif self.path == "/stats":
            self._json_response(200, _get_stats())
        else:
            self.send_error(404)

    def do_POST(self):
        if self.path == "/stats/reset":
            _reset_stats()
            self._json_response(200, {"status": "reset"})
            log.info("[STATS] Counters reset")
            return

        if "/v1/chat/completions" not in self.path:
            self.send_error(404)
            return

        length = int(self.headers.get("Content-Length", 0))
        body = json.loads(self.rfile.read(length)) if length else {}

        raw_model = body.get("model", "qwen3-8b")
        messages = body.get("messages", [])
        temperature = body.get("temperature", 0.5)
        max_tokens = body.get("max_tokens", 3072)
        # Honor both the OpenAI/Qwen-style `enable_thinking: true` flag AND the
        # TuringOS Rust client's `thinking: {"type": "enabled"}` field, so the
        # tdma loop (which sends the latter) can run reasoning mode.
        _thinking_field = body.get("thinking")
        enable_thinking = bool(body.get("enable_thinking", False)) or (
            isinstance(_thinking_field, dict) and _thinking_field.get("type") == "enabled"
        )

        provider = FORCED_PROVIDER or detect_provider(raw_model)
        model = strip_provider_prefix(raw_model)

        try:
            client, key_idx = get_client_round_robin(provider)

            extra = {}
            model_lower = model.lower()
            is_qwen3 = "qwen3" in model_lower
            is_deepseek_thinking = ("deepseek-v4" in model_lower
                                    or "deepseek-reasoner" in model_lower)
            if enable_thinking:
                # Caller explicitly opts in. Both providers use this same
                # short-circuit; the per-provider field shape diverges only
                # in the disable branch.
                if is_deepseek_thinking:
                    extra["extra_body"] = {"thinking": {"type": "enabled"}}
                else:
                    extra["extra_body"] = {"enable_thinking": True}
            elif is_qwen3:
                # Qwen3 family disable flag (Qwen-API convention).
                extra["extra_body"] = {"enable_thinking": False}
            elif is_deepseek_thinking:
                # DeepSeek-API convention per
                # https://api-docs.deepseek.com/zh-cn/guides/thinking_mode
                # extra_body={"thinking":{"type":"disabled"}}. The earlier
                # fix tried Qwen-style enable_thinking=false which DeepSeek
                # silently ignores — every Phase C smoke cell hit the 5-min
                # cap because reasoning_content kept blowing up.
                extra["extra_body"] = {"thinking": {"type": "disabled"}}

            max_retries = 8
            content = ""
            reasoning = ""
            usage_prompt = 0
            usage_completion = 0
            estimated = False

            _wait_for_cooldown()
            _rate_semaphore.acquire()
            try:
                for attempt in range(max_retries + 1):
                    _wait_for_cooldown()
                    try:
                        if attempt == 0:
                            log.info(
                                f"→ {provider}#k{key_idx}/{model} "
                                f"(temp={temperature}, max_tok={max_tokens})"
                            )
                        else:
                            log.info(
                                f"→ {provider}#k{key_idx}/{model} "
                                f"(retry {attempt}/{max_retries})"
                            )

                        resp = client.chat.completions.create(
                            model=model,
                            messages=messages,
                            temperature=temperature,
                            max_tokens=max_tokens,
                            stream=False,
                            **extra,
                        )

                        msg = resp.choices[0].message
                        content = msg.content or ""
                        reasoning = getattr(msg, "reasoning_content", None) or ""

                        estimated = False
                        if resp.usage and resp.usage.completion_tokens:
                            usage_prompt = resp.usage.prompt_tokens or 0
                            usage_completion = resp.usage.completion_tokens or 0
                        else:
                            estimated = True
                            usage_prompt = sum(
                                len(m.get("content", "")) for m in messages
                            ) // 3
                            usage_completion = (len(content) + len(reasoning)) // 3

                        break  # success

                    except (RateLimitError, APIStatusError) as e:
                        is_429 = isinstance(e, RateLimitError) or (
                            hasattr(e, "status_code") and e.status_code == 429
                        )
                        if is_429 and attempt < max_retries:
                            _record_retry()
                            wait = min(2 ** attempt + 1, 30)
                            _trigger_cooldown(wait)
                            time.sleep(wait)
                        else:
                            raise
            finally:
                _rate_semaphore.release()

            _record_usage(usage_prompt, usage_completion)
            if estimated:
                _record_estimated()

            result = {
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": content,
                            "reasoning_content": reasoning if reasoning else None,
                        },
                        "finish_reason": "stop",
                    }
                ],
                "model": raw_model,
                "usage": {
                    "prompt_tokens": usage_prompt,
                    "completion_tokens": usage_completion,
                    "total_tokens": usage_prompt + usage_completion,
                    "estimated": estimated,
                },
            }

            log.info(
                f"← {provider}#k{key_idx}/{model}: {len(content)}c content, "
                f"{len(reasoning)}c reasoning, "
                f"{usage_prompt}+{usage_completion}={usage_prompt + usage_completion} tokens"
            )

            self._json_response(200, result)

        except Exception as e:
            _record_error()
            log.error(f"Error on {provider}/{raw_model}: {e}")
            self._json_response(500, {"error": {"message": str(e)}})

    def _json_response(self, code, data):
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())

    def log_message(self, format, *args):
        pass


class ThreadedHTTPServer(ThreadingMixIn, HTTPServer):
    daemon_threads = True


FORCED_PROVIDER = None


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--port",
        type=int,
        default=int(os.environ.get("LLM_PROXY_PORT", "8080")),
    )
    parser.add_argument(
        "--provider",
        type=str,
        default=None,
        help="Force all requests to this provider (overrides model-based routing)",
    )
    args = parser.parse_args()

    if args.provider:
        if args.provider not in PROVIDERS:
            log.error(
                f"Unknown provider: {args.provider}. Available: {list(PROVIDERS.keys())}"
            )
            sys.exit(1)
        FORCED_PROVIDER = args.provider
        log.info(f"Provider forced to: {args.provider}")

    # Pre-resolve which providers have keys configured (for /health-style
    # diagnostics). Don't fail boot on missing keys — they're only
    # required when the corresponding provider is actually invoked.
    configured = []
    for prov_name, (_, key_envs) in PROVIDERS.items():
        present = [e for e in key_envs if os.environ.get(e, "").strip()]
        if present:
            configured.append(f"{prov_name}({len(present)}k)")

    server = ThreadedHTTPServer(("127.0.0.1", args.port), Handler)
    log.info(f"LLM Proxy v4 listening on 127.0.0.1:{args.port}")
    log.info(f"Providers configured: {', '.join(configured) if configured else '(none)'}")
    log.info("Token metering: enabled (/stats, /stats/reset)")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        log.info("Shutting down")
