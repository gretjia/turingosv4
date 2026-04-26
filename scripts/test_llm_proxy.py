#!/usr/bin/env python3
"""A8e fix F3 — unit tests for src/drivers/llm_proxy.py routing logic.

Codex#4 (round-1 A8 audit) caught: `Qwen/Qwen2.5-7B-Instruct` misrouted
to DashScope because `m.startswith("qwen")` won after the slash check.
This test file pins the routing matrix as a CI conformance gate.

Also verifies the round-robin mechanic (F2) without invoking any cloud
API: `_build_clients` is monkeypatched to return a list of dummy
sentinels, then `get_client_round_robin` is called multiple times and
the per-key counter distribution is asserted.

Run: `python3 scripts/test_llm_proxy.py` (no pytest required).
"""
import os
import sys
import unittest

# Inject repo root so `from src.drivers import llm_proxy` works.
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, ROOT)

# Import via the file path (the proxy isn't a package; it's a script).
import importlib.util

spec = importlib.util.spec_from_file_location(
    "llm_proxy", os.path.join(ROOT, "src", "drivers", "llm_proxy.py")
)
llm_proxy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(llm_proxy)


class RoutingMatrixTests(unittest.TestCase):
    """detect_provider over the canonical model id matrix."""

    def test_explicit_prefix_wins(self):
        self.assertEqual(llm_proxy.detect_provider("siliconflow:Qwen/Qwen2.5-7B-Instruct"), "siliconflow")
        self.assertEqual(llm_proxy.detect_provider("deepseek:deepseek-v4-flash"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("dashscope:qwen3-8b"), "dashscope")

    def test_unknown_explicit_prefix_falls_through_to_heuristic(self):
        # `nonsense:` is not in PROVIDERS — treat the whole string as
        # a bare model id and route by heuristic. A bare string with
        # a colon and no slash falls to the dashscope default.
        self.assertEqual(llm_proxy.detect_provider("nonsense:foo-bar"), "dashscope")

    def test_deepseek_substring(self):
        self.assertEqual(llm_proxy.detect_provider("deepseek-v4-flash"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("deepseek-chat"), "deepseek")
        self.assertEqual(llm_proxy.detect_provider("DeepSeek-V4-Flash"), "deepseek")

    def test_qwen_huggingface_style_routes_to_siliconflow(self):
        # A8e fix F3 (Codex#4): the round-1 bug was that this
        # misrouted to dashscope because "qwen" prefix won after the
        # slash check. Pinned here so it can never regress.
        self.assertEqual(
            llm_proxy.detect_provider("Qwen/Qwen2.5-7B-Instruct"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("Qwen/Qwen3.5-397B-A17B"),
            "siliconflow",
        )

    def test_other_huggingface_style_routes_to_siliconflow(self):
        self.assertEqual(llm_proxy.detect_provider("openai/gpt-4o"), "siliconflow")
        self.assertEqual(
            llm_proxy.detect_provider("meta-llama/Llama-3.1-70B-Instruct"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("THUDM/glm-4-9b-chat"),
            "siliconflow",
        )

    def test_deepseek_slash_form_routes_to_siliconflow(self):
        # A8e6 fix K2 (Codex R6#2): `deepseek-ai/DeepSeek-R1-Distill-*`
        # is a SiliconFlow-catalog model — the official DeepSeek API
        # at api.deepseek.com only serves bare `deepseek-chat` and
        # `deepseek-v4-flash`, not the Distill variants. Pre-K2 the
        # routing checked "deepseek" substring BEFORE the slash check
        # and misrouted these to api.deepseek.com, which returns 404.
        # Pinned here so the slash-vs-substring precedence never
        # regresses.
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-ai/DeepSeek-R1-Distill-Qwen-7B"),
            "siliconflow",
        )
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-ai/DeepSeek-V2.5"),
            "siliconflow",
        )
        # Sanity: bare deepseek model ids still route to deepseek.
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-chat"),
            "deepseek",
        )
        self.assertEqual(
            llm_proxy.detect_provider("deepseek-v4-flash"),
            "deepseek",
        )

    def test_bare_qwen_routes_to_dashscope(self):
        # Bare model ids without a slash are direct DashScope catalog.
        self.assertEqual(llm_proxy.detect_provider("qwen3-8b"), "dashscope")
        self.assertEqual(llm_proxy.detect_provider("qwen-max"), "dashscope")

    def test_default_fallback_dashscope(self):
        self.assertEqual(llm_proxy.detect_provider("some-unknown-model"), "dashscope")
        self.assertEqual(llm_proxy.detect_provider(""), "dashscope")


class StripProviderPrefixTests(unittest.TestCase):
    def test_strips_known_prefix(self):
        self.assertEqual(
            llm_proxy.strip_provider_prefix("siliconflow:Qwen/Qwen2.5-7B-Instruct"),
            "Qwen/Qwen2.5-7B-Instruct",
        )
        self.assertEqual(
            llm_proxy.strip_provider_prefix("deepseek:deepseek-v4-flash"),
            "deepseek-v4-flash",
        )

    def test_leaves_unknown_prefix_intact(self):
        # `nonsense:foo` is NOT a known provider, so the colon is part
        # of the model identifier and must round-trip unchanged.
        self.assertEqual(
            llm_proxy.strip_provider_prefix("nonsense:foo-bar"),
            "nonsense:foo-bar",
        )

    def test_leaves_bare_model_intact(self):
        self.assertEqual(llm_proxy.strip_provider_prefix("qwen3-8b"), "qwen3-8b")
        self.assertEqual(
            llm_proxy.strip_provider_prefix("Qwen/Qwen2.5-7B-Instruct"),
            "Qwen/Qwen2.5-7B-Instruct",
        )


class RoundRobinTests(unittest.TestCase):
    """A8e fix F2 — round-robin distribution conformance.

    Verifies the V3L-27 single-key collapse mitigation without
    invoking any cloud API: monkeypatch `_build_clients` to return a
    fixed list of dummy strings, then call `get_client_round_robin`
    and assert the per-key counter advances [1,0,0] → [1,1,0] → [1,1,1]
    → [2,1,1] → [2,2,1] → [2,2,2] across 6 calls.
    """

    def setUp(self):
        # Reset the proxy module's process-global state between tests.
        llm_proxy.clients_by_provider.clear()
        llm_proxy._rr_counters.clear()
        llm_proxy._per_key_requests.clear()

    def test_three_key_round_robin_distributes_evenly(self):
        # Inject a 3-element dummy client pool for `siliconflow`.
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0, 0]

        results = []
        for _ in range(6):
            client, idx = llm_proxy.get_client_round_robin("siliconflow")
            results.append((client, idx))

        # Indices must cycle 0, 1, 2, 0, 1, 2.
        self.assertEqual(
            [r[1] for r in results],
            [0, 1, 2, 0, 1, 2],
            "round-robin must visit every key in order before repeating",
        )
        # Each client must equal the corresponding pool element.
        for client, idx in results:
            self.assertEqual(client, f"k{idx}")

        # Final per_key_requests must be [2, 2, 2] — the documented
        # invariant from the A7 commit message and TRACE_MATRIX § 2.
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [2, 2, 2],
            "after 6 calls the 3-key pool must distribute evenly",
        )

    def test_single_key_pool_always_returns_index_zero(self):
        # Single-key provider (e.g. deepseek) — round-robin must
        # degrade gracefully, not throw modulo-by-zero.
        llm_proxy.clients_by_provider["deepseek"] = ["only-key"]
        llm_proxy._per_key_requests["deepseek"] = [0]

        for expected_count in range(1, 5):
            client, idx = llm_proxy.get_client_round_robin("deepseek")
            self.assertEqual(idx, 0)
            self.assertEqual(client, "only-key")
            self.assertEqual(
                llm_proxy._per_key_requests["deepseek"],
                [expected_count],
            )

    def test_two_key_pool_alternates(self):
        # Two-key pool: 4 calls → [2, 2].
        llm_proxy.clients_by_provider["siliconflow"] = ["a", "b"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0]
        for _ in range(4):
            llm_proxy.get_client_round_robin("siliconflow")
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [2, 2],
        )


class StatsAggregationTests(unittest.TestCase):
    def setUp(self):
        llm_proxy._reset_stats()

    def test_get_stats_includes_per_key_distribution(self):
        # Seed a 3-key pool and fire 3 calls.
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [0, 0, 0]
        for _ in range(3):
            llm_proxy.get_client_round_robin("siliconflow")
        stats = llm_proxy._get_stats()
        self.assertEqual(stats["per_key_requests"]["siliconflow"], [1, 1, 1])

    def test_reset_clears_per_key_counters(self):
        llm_proxy.clients_by_provider["siliconflow"] = ["k0", "k1", "k2"]
        llm_proxy._per_key_requests["siliconflow"] = [3, 3, 3]
        llm_proxy._reset_stats()
        self.assertEqual(
            llm_proxy._per_key_requests["siliconflow"],
            [0, 0, 0],
            "_reset_stats must zero per-key counters too",
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
