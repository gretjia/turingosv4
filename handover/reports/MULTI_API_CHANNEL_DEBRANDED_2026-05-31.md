# Multi-API-Channel, De-Branded — architecture note + real-test evidence

**Date:** 2026-05-31
**Directive (architect):** "你需要 strong model 的话还是可以用 deepseek official api 调用 deepseek v4 pro，正好可以
试试 multi-API channel，注意所有的调用必须是 de-branded, de-channel, must be generic, 以后可能会调用很多品牌的 api。"

**Finding:** the architecture the directive asks for **already exists** and is now exercised across two live
channels. This note records the principle, the verified evidence, and the one-row contract for adding future
brands.

---

## 1. The principle

- **De-branded calling layer.** Code that *uses* a model never names a vendor. It sends a model-id string to
  one generic OpenAI-compatible endpoint and gets a completion back. It does not know — and must not encode —
  which company serves that model.
- **De-channel routing, centralized.** Exactly one component knows the channel map: the local gateway
  `src/drivers/llm_proxy.py`. Routing lives in its `PROVIDERS` registry + `detect_provider()` + the explicit
  `provider:model` escape hatch. Nowhere else.
- **Generic / future-proof.** Adding a brand = one `PROVIDERS` row (base_url + key env names). Zero caller
  changes. "以后可能会调用很多品牌的 api" is a config edit, not a refactor.

## 2. Verified reality (real-test evidence, not assertion)

The gateway is a generic OpenAI-compatible HTTP server on `127.0.0.1:8123`. Its `detect_provider(model)` routes:

| model-id shape | channel | base_url |
|---|---|---|
| explicit `provider:model` | that provider | (registry) |
| bare `deepseek-v4-pro`, `deepseek-v4-flash`, `deepseek-*` | **DeepSeek official** | api.deepseek.com |
| slash-form `Org/Model` (`deepseek-ai/DeepSeek-V3.2`, `Qwen/Qwen3-32B`, …) | **SiliconFlow** | api.siliconflow.cn |
| bare `qwen*` | DashScope | dashscope.aliyuncs.com |

Live catalog discovery (2026-05-31): DeepSeek official API serves exactly **`deepseek-v4-flash`** and
**`deepseek-v4-pro`** (deepseek-chat / deepseek-reasoner being deprecated → flash non-thinking / thinking).

Real call through the generic gateway, bare id `deepseek-v4-pro` → official channel:
> prompt: prove `theorem t (n:Nat) : n + 0 = n`, tactic only
> reply: `induction n with | zero => simp | succ n ih => simp [ih]` — a **correct** Lean proof, real token
> metering (`estimated: false`), `reasoning_len=0` (thinking-off default; gateway honors `thinking:{type:enabled}`
> if a caller opts in).

The EMERGE Stage-1 study runs **three strong models across two channels in one job** — `deepseek-v4-pro`
(DeepSeek official) + `deepseek-ai/DeepSeek-V3.2` + `Qwen/Qwen3-32B` (SiliconFlow) — and the harness passes
only the three id strings; the gateway does all routing.

## 3. De-branded audit (the calling layer names zero channels)

`grep -rniE 'siliconflow|dashscope|api\.deepseek|…' src/bin/` over the **study harness**
`src/bin/lean_emergence.rs`: **zero** channel/brand literals. It uses one generic proxy URL
(`http://localhost:8123`) and model-id strings only. ✓

Brand strings that DO appear elsewhere are not routing and do not violate the principle:
- `src/bin/lean_hayek_market.rs` — the `MODEL_RATES` **pricing** table (a per-model price map legitimately
  names models + cites which channel each routes to, for price accuracy; OBL-012). Pricing ≠ routing.
- `*_current_kernel.rs` — a descriptive diagnostic string ("…access is outside the kernel through the local
  LLM proxy"). Documentation, not a hardcoded channel.

## 4. Contract for the next brand (the generic extension point)

To onboard brand X with model `foo-1`:
1. add `"x": ("https://api.x.com/v1", ["X_API_KEY"])` to `PROVIDERS` in `llm_proxy.py`;
2. (optional) add a `detect_provider` heuristic, OR just call it as `x:foo-1` (the always-generic escape hatch);
3. callers change **nothing** — they pass `x:foo-1` (or the bare id) like any other model;
4. if X becomes a cost-headline workhorse, add one `MODEL_RATES` row with X's published price (OBL-012 honesty).

That is the whole surface. The harness stays de-branded; the gateway stays the single channel authority; the
money path stays honest per-model.

## 5. Constitution posture

No FC1/FC2/FC3 touch, no §6 surface. The gateway is outside the kernel (LLM access is a tool boundary, per the
`*_current_kernel.rs` note). De-branded calling + centralized routing is consistent with "agent read views must
be scoped" — the calling agent sees a model id, never a vendor secret or channel detail.
