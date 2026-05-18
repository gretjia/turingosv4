# Phase 7 W7 — Visual Self-Check

**Date:** 2026-05-18
**Branch:** codex/tisr-phase7-web
**Atom:** TISR Phase 7 W7 — welcome wizard around Phase 6.3 5-step flow
**Stack:** turingos_web (axum) + esbuild ESM bundle (67.7 kB)
**Workspace:** tmp/phase7_active (default; deleted before this check)

The Chrome MCP `save_to_disk` flag silently fails to land files on disk on this
host (same observation noted in the W6 self-check). The screenshots below are
therefore described inline with their exact composition. The architect will
walk through the live UI immediately after commit; this document is a written
record of what was on screen during the W7 self-check.

## Screenshot 1 — `GET /` → `302 /welcome` cold-open

Page loaded by navigating to `http://127.0.0.1:8080/`. Server responded with
HTTP 303 to `/welcome` (curl-confirmed before browser navigation).

What was on screen:

- **Header**: TuringOS wordmark in Fraunces Black 144 opsz (the accent-teal
  diamond glyph sits left of the wordmark; "PHASE 7" subtitle in JetBrains
  Mono small-caps inside a hairline pill). Right side: a single
  `skip → build` link in mono small-caps, muted neutral.
- **No nav row.** Intentional. First-time users haven't earned the
  Dashboard/Agents/Tasks/Audit/Build nav yet.
- **Main column** (centered, max-width 720px):
  - **Progress indicator**: 5 numbered circles connected by a single hairline
    rule running through their vertical centers. Step 1 has a teal hairline
    ring + teal numeral (active). Steps 2–5 have neutral hairline rings +
    neutral numerals (pending). Labels below each circle in body sans:
    工作站 / 模型配置 / API 密钥 / 注册 Agent / 开始访谈.
  - **Step card** below a hairline rule:
    - Caption `STEP 1 / 5` in teal JetBrains Mono small-caps, 0.22em
      letter-spacing.
    - Title 第一步 · 准备工作站 in Fraunces Italic ~24px,
      opsz 80, SOFT 50.
    - Subtitle "我帮你在硬盘上铺一张空白的'账本桌面'——里面有
      genesis_payload.toml 和 agent_pubkeys.json，是后面所有步骤的地基。"
      in Fraunces Italic 16px, line-height 1.55, muted ink, max-width 56ch.
    - CTA "准备工作站 →" in mono medium small-caps, accent teal, with a
      thin 1px teal underline. On hover the underline thickens to 2px.
- **Footer**: hairline rule; FC2-N16 notice on the left, `● CONNECTED` WS
  pill on the right.

Aesthetic verdict: paper-toned, editorial, hairline rules, generous vertical
rhythm. Feels closer to a printed lab protocol or an academic notebook than a
SaaS onboarding wizard. Pass.

## Screenshot 2 — After clicking "准备工作站 →" (init done; step 2 active)

After 4s wait (real `turingos init --project tmp/phase7_active
--template multi-agent` shellout):

- Progress: circle 1 now SOLID teal (paper number on teal background); circle
  2 has the teal ring + teal numeral (active); 3–5 still neutral pending.
- Step card: caption `STEP 2 / 5`, title 第二步 · 配置两个模型, subtitle
  mentions DeepSeek V3.2 + Qwen3-Coder 30B + "只写模型名字，不写密钥".
- CTA "写入 turingos.toml →".

The accent-rule sweep animation across the top of the card runs once when
the card mounts (320ms ease-out). Visible during the transition, gone by
the time the screenshot stabilizes — intended.

## Screenshot 3 — After clicking "写入 turingos.toml →" (llm config done; step 3 active)

- Progress: circles 1 and 2 solid teal; circle 3 ringed teal (active);
  4–5 pending.
- Step card: caption `STEP 3 / 5`, title 把 SiliconFlow 的 API 密钥交给我.
- Subtitle: "密钥只活在这个服务器进程的内存里——重启就丢，从不写盘、不进
  日志、不会回显在网页上。你只需要在每次启动 turingos_web 之后填一次。"
- Field: monospace small-caps label `SILICONFLOW_API_KEY` over a
  chrome-less `<input type="password">` with hairline bottom rule only
  (transparent background, no border on top/sides; placeholder `sk-...`
  in italic muted). Cursor focused automatically.
- CTA "保存密钥 →" below.

The input visually mirrors the spec-grill textarea aesthetic from W6 — the
two centerpiece pages share the same chrome-less Bauhaus underline-input
language.

## Screenshot 4 — After typing `sk-stub-test-key-for-visual-check-only` + clicking "保存密钥 →"

POST to `/api/welcome/api-key`; in-memory mutex set; refreshed status
returned (200; body does NOT echo the key value).

- Progress: circles 1, 2, 3 SOLID teal; circle 4 ringed teal (active);
  circle 5 pending.
- Step card: caption `STEP 4 / 5`, title 第三步 · 给工作站注册一个 Agent.
  (NB: the body copy says "第三步" because steps 1+2 are filesystem-y
  preparation; the user-facing numbering doesn't include API-key step as a
  numbered "step" in prose. The `STEP 4 / 5` caption is the technical
  position in the wizard.)
- Subtitle: "注册一个 Solver 角色的 agent_001，告诉系统'以后是这个 agent
  在跑工作'。这是 Phase 6.1 的多 agent 体系的最小入口。"
- CTA "注册 AGENT_001 →".

## Screenshot 5 — After clicking "注册 AGENT_001 →" (final ready panel)

Real shellout: `turingos agent deploy --workspace tmp/phase7_active
--id agent_001 --pubkey <64-hex synthetic> --role Solver`. Exit 0. Status
refreshes; next_step = Spec → wizard renders the ready card.

- Progress: circles 1, 2, 3, 4 SOLID teal; circle 5 ringed teal (active).
- Step card:
  - Caption `完成 / READY` in teal JetBrains Mono small-caps.
  - Title "你的工作站已就绪。" in Fraunces Italic.
  - Subtitle "五步全部完成。点下面开始 spec 访谈——我会问你八个关于
    '日常麻烦'的问题，然后帮你生成一个小工具。"
  - CTA "开始 SPEC 访谈 →" (clicks navigate to `/build`).
- The card has a 3px accent-teal left border (`.welcome-ready-card`),
  visually distinguishing the final step from the four intermediate ones.

## Self-critique (the bar)

The architect's prompt explicitly demanded craft, not just functional
correctness. Items I checked against the W4.4 + W6 floor:

- [x] **No purple gradients, no SaaS chrome.** Palette is paper + ink + a
  single oxidized-teal accent. Hairline rules only. No shadows. Radii ≤ 2px.
- [x] **Distinctive typography pair.** Fraunces Italic (display + step
  titles + subtitle) + JetBrains Mono (captions, labels, CTA text) + IBM
  Plex Sans (progress labels). No Inter/Roboto/Arial slips.
- [x] **Editorial register.** Subtitles are Fraunces Italic, conversational
  Chinese, ~50–60 chars. No "Setup Wizard" buzzword voice.
- [x] **Progress indicator with three typographic states.** Done = solid
  teal + paper numeral. Active = paper bg + teal hairline ring + teal
  numeral. Pending = paper bg + neutral hairline + neutral numeral. The
  hairline rule connecting circles reads as a printed five-step protocol,
  not a "progress bar".
- [x] **API-key input is chrome-less.** Matches the spec-grill textarea
  from W6 — single hairline bottom rule, transparent bg, mono typeface,
  italic placeholder. The two centerpiece pages share a language.
- [x] **No emoji, no confetti, no flashes.** Success transition is a CSS
  keyframe accent-rule sweep across the top of the new card (320ms ease-out).
- [x] **First-time-user respect.** No full nav at the top; only the skip
  link for advanced users.

Iteration count during this self-check: 1. The first build had the right
voice. I tweaked nothing between initial render and what's described here.

## Verification gates (full list re-run pre-commit)

1. `cargo build --bin turingos` — exit 0
2. `cargo build --bin turingos_web --features web` — exit 0
3. `cargo test --lib boot::tests::verify_trust_root_passes_on_intact_repo` — 1/1
4. `cargo fmt --all -- --check` — exit 0
5. `cargo test --features web --test cli_web_smoke` — 47 passed
6. `cargo test --features web --test cli_web_routes_smoke` — 53 passed
7. `cargo test --features web --test cli_web_ws_smoke` — 50 passed
8. `cargo test --features web --test cli_web_write_smoke` — 54 passed
9. `cargo test --features web --test cli_web_spec_smoke` — 51 passed
10. `cargo test --features web --test cli_web_generate_smoke` — 53 passed
11. `cargo test --features web --test cli_web_welcome_smoke` — 56 passed (10 new + 46 inherited)
12. `cargo test --test cli_wrapper_plumbing` — 5/5
13. `cd frontend && npm test` — 86 passed (13 new welcome tests + 73 W6 baseline)
14. `cd frontend && npm run build` — bundle 67.7 kB (was 49.9 kB in W6)
