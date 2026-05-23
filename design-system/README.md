# TuringOS Design System

> A paper-toned editorial design system for **TuringOS** — a tape-first
> constitutional operating substrate for LLM / AGI agents.

This kit captures the visual language of the TuringOS Phase 7 web UI and CLI
so design agents can prototype mockups, slides, and one-off interfaces that
look and feel like they came out of the same workshop.

---

## What TuringOS is

TuringOS v4 is **not** a consumer SaaS app. It's a research instrument
shipping in two forms:

| Surface | Binary / route | What it is |
|---|---|---|
| **`turingos` CLI** | `target/debug/turingos` | ~25 subcommands across `init`, `spec`, `generate`, `verify`, `audit`, `replay`, `task`, `agent`, `welcome`, etc. The primary developer entry point. |
| **`turingos_web` browser UI** | `http://127.0.0.1:8080` | A vanilla TypeScript + Web Components frontend wrapping the same `spec → generate → preview` flow. Five top-level routes: `/welcome`, `/build`, `/`, `/tasks`, `/agents`, `/audit`. Server-rendered chrome + client-upgraded `<turingos-*>` custom elements. |

Underneath: a Rust `ChainTape + CAS evidence` substrate where every state
transition is a content-addressed `EvidenceCapsule`. Reports and dashboards
are "materialized views" — never authoritative. The constitution (`constitution.md`)
is in Mandarin Chinese; the production UI runs in `zh-CN` with English
section subtitles in monospace.

The aesthetic brief (from the production codebase, frontend/src/design-system.css):

> Research instrument, NOT consumer SaaS. Closer to a paper-toned editorial
> publication, a UNIX manual page, or an academic notebook than a B2B
> dashboard. The constitution rendered as a UI.

---

## Sources used to build this system

- **Codebase**: `turingosv4/` (attached via the user's local file system)
  — primary source of truth.
- **GitHub repo**: <https://github.com/gretjia/turingosv4> — the same
  codebase on GitHub. Anyone iterating on this design system can browse
  it for additional context (component implementations, server-rendered
  HTML, prompt templates, the full constitution).

Specific files lifted into this kit:

| Imported as | From |
|---|---|
| `reference/design-system.css` | `frontend/src/design-system.css` |
| `reference/base-styles.css`   | `frontend/src/base-styles.css`   |

Components were rebuilt from `frontend/src/components/*.ts` (Web Components
authored in TypeScript) rather than screenshots — pixel-fidelity to the
shipping code.

---

## Index

```
.
├── README.md                  ← you are here
├── SKILL.md                   ← Agent Skill manifest (Claude Code-compatible)
├── colors_and_type.css        ← canonical CSS vars + semantic h1/h2/p/code defaults
├── assets/                    ← wordmark, mark, dark variant
│   ├── wordmark.svg
│   ├── wordmark-dark.svg
│   └── mark.svg
├── reference/                 ← raw lifted source from frontend/
│   ├── design-system.css
│   └── base-styles.css
├── preview/                   ← Design-System-tab cards (atomic specimens)
│                                — includes a `Patterns` group of
│                                  Software-3.0 / AGI-era affordances
│                                  (streaming oracle, reasoning trace,
│                                  agent presence, provenance, prompt-
│                                  as-code, chaintape flow)
└── ui_kits/
    └── web/                   ← turingos_web high-fidelity recreations
        ├── README.md
        ├── index.html         ← interactive click-thru: welcome → build → spec
        ├── Welcome.jsx        ← /welcome 5-step wizard
        ├── Build.jsx          ← /build spec-grill interview
        ├── Chrome.jsx         ← wordmark, header, nav, footer, shared atoms
        ├── Screens.jsx        ← dashboard / agents / tasks / audit views
        └── App.jsx            ← hash router + React mount
```

No slide template was attached, so there is no `slides/`.

---

## Content fundamentals — copy, tone, voice

The product UI runs in **Simplified Chinese (zh-CN)** with English section
subtitles in monospace small-caps. Even agents who don't read Chinese should
preserve this bilingual register; it is the most distinctive content trait.

**Tone.** Technical, precise, philosophical. The codebase frames itself in
the vocabulary of Turing's 1948 paper ("paper, pencil, rubber, strict
discipline"), constitutional law ("Art. 0.1", "§8 ratification"), and
auditable systems (`ChainTape`, `CAS`, `CID`, `EvidenceCapsule`). Writing
copy in this voice means **using these terms, not softening them**.

**Pronoun.** The product talks to the user as `你` ("you") and refers to
itself as `我` ("I") — first-person, conversational, slightly intimate.
Example from `welcome.ts`:

> 我帮你在硬盘上铺一张空白的"账本桌面"——里面有 genesis_payload.toml
> 和 agent_pubkeys.json，是后面所有步骤的地基。
>
> _(I'll lay out a blank "ledger desktop" on your disk for you — with
> genesis_payload.toml and agent_pubkeys.json inside, the foundation
> for every step that follows.)_

**Casing.**
- Body copy: sentence-case in Chinese; mid-sentence English terms stay in
  their canonical case (`spec`, `CAS`, `TuringOS`, `DeepSeek V3.2`).
- Eyebrows / labels: **mono uppercase** with `0.16–0.22em` letter-spacing
  (e.g. `STEP 1 / 5`, `BUILD · SPEC INTERVIEW · PHASE 7 W6`).
- Section English subtitles: mono uppercase (e.g. `Build Now`, `Deeper Insight`).
- Hashes / CIDs / agent ids: lowercase mono, tight letter-spacing.

**Emoji.** Almost never. The base UI uses **zero emoji**. The one exception
is the static `spec_template.html` document, which uses 🎯 🛡️ ✅ 🚫 ⚡ as
section markers — and that file is generated _output_, not chrome. Default
to no emoji.

**Vibe.** Examples lifted from the product:

| Surface | Copy |
|---|---|
| /build hero | `从一段闲聊开始，做出你想要的那个小工具。` |
| /build subline | `build · spec interview · phase 7 w6` |
| Welcome CTA | `开始 spec 访谈 →` |
| Welcome step caption | `STEP 3 / 5` |
| Welcome subtitle | `密钥只活在这个服务器进程的内存里——重启就丢，从不写盘、不进日志、不会回显在网页上。` |
| Footer | `FC3-N31: materialized view — not authoritative over ChainTape/CAS.` |
| Spec section | `🎯 目标` / `<span class="en">Goal</span>` |

Notice the em-dashes (`——` for Chinese, `—` for English), the trailing
arrows on buttons (`→`), and the absence of "Welcome!", "Let's get started!",
or any consumer-grade exuberance.

**Forbidden copywriting moves.**
- Exclamation points (the product has none).
- "Awesome", "Magic", "Effortlessly", "Beautifully", or any marketing-AI slop.
- Calls-to-action like "Get Started" or "Sign Up" — TuringOS users
  `开始 spec 访谈` / `保存密钥`.
- Emoji as primary brand element.

---

## Visual foundations

### Color

- **Paper** (`#FAFAF7`) is the page background. Warm off-white, never
  pure white.
- **Ink** (`#1A1817`) is the body color. Warm near-black, never pure black.
- **Oxidized teal** (`#1F6E6B`) is the one and only accent. It marks links,
  the active nav item, the success state of the wizard, the rotated square
  in the wordmark, and that's _it_. Do not introduce a second accent.
- **Hairlines** are `#E5E3DC` (1px) and `#C9C5BC` (stronger 1px). Cards,
  inputs, tables, and the nav bar are bounded by hairlines — never shadows.
- **Status colors** are typographic + colored badges, never icon-only:
  moss `#3F6E3F` (accepted), brick `#9C3A2F` (rejected), amber `#A87431`
  (finalized), charcoal `#3A3633` (bankrupt), neutral `#807974` (expired).

Dark mode is warm ink-on-paper-reversed (`#14110E` bg, `#E8E4DA` fg, teal
shifts to `#5BB3A6`). Never pure black. Never cool blue.

### Typography

Three families. They never substitute for each other.

- **Fraunces** — display serif. Used italic for `h1` and large headings
  with `font-variation-settings: "opsz" 60-144, "SOFT" 30-50`. Used non-italic
  Black 900 for the wordmark with `-0.02em` letter-spacing.
- **IBM Plex Sans** — body. Weights 300/400/500/700. Long-form prose.
- **JetBrains Mono** — every label, eyebrow, hash, CID, agent id,
  monospace caption. Letter-spacing `0.12–0.22em` for caps; `-0.01em` for
  inline hashes.

Type scale: 11 / 12 / 14 / 16 / 20 / 26 / 34 / 46 px (1.333 ratio).

### Spacing

4px scale: 4, 8, 12, 16, 24, 32, 48, 64 px. Generous vertical rhythm —
the page feels under-filled, like a printed page with margins.

### Backgrounds & textures

**None.** No photography, no gradients, no patterns, no illustrations,
no full-bleed imagery. Pages are paper. Cards sit on raised paper
(`--bg-elev: #F5F4EE`). The only "decoration" in the entire system is:
- the rotated 10px teal square in the wordmark,
- the 1px accent rule that animates across the top of the wizard card on mount,
- the dotted progress connector running through the 5 numbered circles on `/welcome`.

If a design needs imagery, that imagery should be **monochromatic, warm,
and grainy** if introduced at all — but the default is "no image."

### Corners and elevation

- **Radii are restrained.** `--radius-sm: 2px`, `--radius-md: 3px`.
  Maximum. The progress circles are 50% only because they are dots;
  there are no `rounded-xl` pillowy cards.
- **No drop shadows. Anywhere.** Cards are bounded by 1px hairlines.
  Elevation is communicated by `--bg-elev` (`#F5F4EE` — 2 shades warmer
  than the page).
- **No outer glows.** No inset shadows. No skeuomorphism.

### Borders

- Default: `1px solid var(--hairline)` (`#E5E3DC`).
- Stronger / interactive: `1px solid var(--hairline-strong)` (`#C9C5BC`).
- Table headers underlined with `2px solid var(--fg)`.
- Cards: full hairline border + optional **3px accent left-stripe** on
  `agent_card` / `task_card` (the one exception to the no-left-accent-border
  rule, justified because it's used as a status indicator, not decoration).

### Hover & press states

- **Links:** `border-bottom` thickens from 1px → 2px on hover; color
  shifts from `--accent` → `--fg`.
- **Buttons:** background shifts from `--fg` → `--accent` on hover for
  filled buttons; for "ghost" text-only buttons, the bottom underline
  thickens 1px → 2px and color shifts to `--fg`.
- **Card rows:** background shifts from `--bg` → `--bg-elev` on hover.
- **Nav links:** color shifts `--fg-muted` → `--fg`; active item has a
  `2px solid var(--accent)` bottom border.
- **Press state:** there is no separate press state in the production CSS.
  No scale shrink, no color darken on `:active`. Click is instantaneous.

### Motion

- `--motion-fast: 120ms`, `--motion-normal: 180ms`.
- Easing: `cubic-bezier(0.2, 0.7, 0.2, 1)` — quick-out, gentle-in.
- **Page-load:** blocks rise 4px and fade in over 180ms with a 40ms
  staggered delay (`@keyframes tos-rise`).
- **Wizard card mount:** the new card rises 6px / 280ms, and a hairline
  sweep animates across the top edge of the card (0 → 100% width, 320ms).
- **Loading:** typographic ellipsis (`· · ·`) where dots pulse opacity
  20% → 100% at 200ms staggered offsets, set in Fraunces italic.
  **Never a spinner GIF, never a circular loader, never a progress bar.**
- **Connection pulse:** the footer's connection-state dot pulses
  opacity 1 → 0.35 + scale 1 → 0.85 over 1.4s when reconnecting.
- **Reduced motion:** all animation durations zeroed under
  `@media (prefers-reduced-motion: reduce)`.

### Transparency, blur, gradients

- **No backdrop-filter blur.** None in production.
- **No gradients.** None in production. (Specifically forbidden: bluish-purple gradients.)
- **Limited transparency.** Only on the connection-pulse keyframe and
  the selection background (`--accent-soft` is solid, not transparent).

### Layout rules

- Header / nav are `position: sticky; top: 0; z-index: 10` on the nav.
- Main content cap: `--measure-prose: 68ch` for prose, `--measure-wide:
  1200px` for dashboards. The Build interview is centred at `720px max-width`.
- Welcome page is even narrower — `840px max-width`, single-column,
  generous top padding (`--space-7`).
- Footer is full-width, hairline-bordered, all-caps monospace at 11px.

### Cards

- Default card: `1px solid var(--hairline)`, padding `--space-4 --space-5`,
  background `--bg` or `--bg-elev`.
- Agent / task cards add a **3px coloured left stripe** keyed to status
  (teal for agents, amber for open tasks). Header is a baseline-aligned
  row of mono id + mono caps role label, hairline-separated from the dl
  metadata grid below.
- Welcome wizard card: `1px hairline` border, `--bg-elev` background, the
  accent sweep animation on mount, optional `3px solid var(--accent)`
  left stripe on the final "ready" state.
- Dashboard panels: `border-top: 1px solid var(--fg)` (thick top rule,
  no other borders) — a printed table-of-contents register.

### Use of imagery

There is no first-party imagery in the product. If a design needs photos,
the only acceptable register is **warm, grainy, monochromatic, low-contrast**
— think of a black-and-white plate in a 1970s research monograph, not a
hero photo on a SaaS landing page.

---

## Iconography

**TuringOS uses essentially no icons.**

The shipping product (`frontend/src/components/*.ts`) renders zero icon
elements — no icon font, no SVG sprite, no Lucide / Heroicons import.
Every signal that a consumer-grade product would carry with an icon is
carried instead by **typography + color**:

- Status badges: a 6px coloured dot (CSS `border-radius: 50%` on a
  pseudo-element) followed by an uppercase mono caps label. No glyph.
- Connection state: same — 7px dot + uppercase mono label.
- Nav active state: hairline-thick bottom underline in `--accent`. No icon.
- Progress circles in the welcome wizard: numbered dots (1–5), not glyphs.
- The wordmark "logo" is a single rotated 10px teal square. That's the
  entire mark.

**The one ornament** that does appear in the static `spec_template.html`
(an _output document_, not chrome) is a set of Unicode-emoji section
markers: 🎯 (goal), 🛡️ (robustness), ✅ (acceptance), 🚫 (out-of-scope),
⚡ (AI-coder prompt). Treat these as **template-document furniture**, not
brand iconography. Do not introduce them elsewhere.

If a design absolutely requires an icon (e.g. a file-tree row in a
dashboard, an external-link indicator), substitute the closest
**Lucide** stroke icon (`stroke-width: 1.5`, currentColor) from
<https://unpkg.com/lucide-static@latest/icons/> and flag the substitution
to the user — TuringOS itself ships no icon set.

**Emoji.** Forbidden in chrome. Permitted only in template output documents.

**Unicode glyphs used as semantic indicators in chrome:**
- `→` (U+2192) — trailing arrow on primary CTAs (`开始 spec 访谈 →`)
- `·` (U+00B7) — middle dot, used as a separator in metadata strings
  (`build · spec interview · phase 7 w6`)
- `——` (U+2014 doubled) — em-dash break in Chinese prose
- `…` (U+2026) — ellipsis after loading-state phrases

---

## Sub-section authority

- For exact pixel values, see `colors_and_type.css` (CSS vars) and
  `reference/design-system.css` (the production token sheet).
- For component-level patterns (event log, dashboard panel, task card,
  spec grill, welcome wizard), see `reference/base-styles.css` and the
  matching `frontend/src/components/*.ts` in the upstream repo
  <https://github.com/gretjia/turingosv4>.
- For tone, voice, and copy register, see `welcome.ts`, `spec-grill.ts`,
  and `constitution.md` in the upstream repo.

---

## AGI-era patterns (Software 3.0 lens)

TuringOS is a Software 3.0 system at its core — LLM agents drive a
constitutional substrate, every state transition is a content-addressed
capsule, and the primary "programming" surface is conversation. The
visual register stays paper-toned and editorial (it's a deliberate
counter-trend, not a limitation), but the **behaviors** the UI affords
should make the AGI-era nature of the product feel inevitable.

Six pattern cards in the `Patterns` group capture this:

- **Streaming Oracle** — agent output token-streams into the page with
  a live caret at the trailing edge. This is the system's default
  loading affordance. Never a spinner.
- **Reasoning Trace** — every agent decision can be expanded into a
  monospace scratchpad showing the chain-of-thought, with token count
  and latency. Trace summaries are first-class; collapsed by default,
  one-click to expand.
- **Agent Presence** — multiple agents are users too. A live activity
  feed with pulsing dots shows which agents are mid-task, blocked, or
  paused, at what action, with elapsed time.
- **Provenance Chips** — every claim in the UI carries `[source · CID ·
  confidence]`. CAS-rooted facts read `confidence ≥ 0.95`; agent claims
  start lower. The chip itself is the trust signal — no separate
  "verified" badge.
- **Prompt as Code** — prompts are first-class artifacts (with their
  own CIDs and PromptPromotionReceipts). They render as `<pre>` blocks
  with role markers (`@system / @user / @assistant`) and variable
  slots in teal.
- **ChainTape Flow** — for multi-agent activity, a horizontal swimlane
  visualization with animated dashed lines connecting transactions
  across solver / verifier / judge lanes. The animation is slow (3.6 s),
  not decorative.

These are additions on top of — not replacements for — the editorial
chrome. A TuringOS screen is a paper page _with live agents working on
it_.

### Software 3.0 copy moves to prefer

- Describe the agent as a collaborator: `我帮你…` ("I'll help you…"),
  not `system has…`.
- Show, don't tell, the model. Mention `DeepSeek V3.2`, `Qwen3-Coder 30B`,
  the prompt CID, the turn count.
- Surface uncertainty when it exists. `confidence 0.51 · unverified`
  reads more credible than a confident UI.
- Speak in turns, not transactions. "turn 04 / 11" is the AGI-era
  equivalent of "step 4 of 11."


This kit references Fraunces, IBM Plex Sans, and JetBrains Mono from
**Google Fonts** at runtime via `<link rel="stylesheet">` (the exact tag
is in the comment at the top of `colors_and_type.css`). No font files are
bundled. If you need them offline or for a print artifact:

- [Fraunces](https://fonts.google.com/specimen/Fraunces) — variable, ital + opsz + SOFT axes
- [IBM Plex Sans](https://fonts.google.com/specimen/IBM+Plex+Sans)
- [JetBrains Mono](https://fonts.google.com/specimen/JetBrains+Mono)

All three are open source (OFL / Apache-2.0). The production frontend
loads them from Google Fonts; this kit follows suit.
