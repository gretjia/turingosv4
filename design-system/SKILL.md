---
name: turingos-design
description: Use this skill to generate well-branded interfaces and assets for TuringOS — a tape-first constitutional operating substrate for LLM/AGI agents — either for production or throwaway prototypes/mocks/decks. Contains essential design guidelines, the paper-toned editorial color & type system, fonts, brand assets (wordmark + mark), iconography rules, content/voice fundamentals, and a high-fidelity Web Components UI kit recreating the turingos_web frontend.
user-invocable: true
---

Read the `README.md` file within this skill, and explore the other available files (`colors_and_type.css`, `assets/`, `preview/`, `ui_kits/web/`, `reference/`).

If creating visual artifacts (slides, mocks, throwaway prototypes, etc), copy assets out of `assets/` and `ui_kits/web/` and create static HTML files for the user to view. Always include the Google Fonts `<link>` tag for Fraunces + IBM Plex Sans + JetBrains Mono — the brand is _defined_ by that typographic pair. Import `colors_and_type.css` for the canonical tokens.

If working on production code (the upstream `turingosv4` Rust + vanilla-TS repo), copy assets and read the rules here to become an expert in designing with this brand. Defer to `reference/design-system.css` and `reference/base-styles.css` for exact pixel-level production values.

Key constraints to enforce no matter what:

- **No emoji in chrome.** No purple gradients. No drop shadows. No pillowy radii (max 3px).
- **No icons** unless you import Lucide and flag the substitution — TuringOS ships none.
- **Paper-toned** off-white (`#FAFAF7`) backgrounds, warm near-black (`#1A1817`) ink, oxidized-teal (`#1F6E6B`) as the single accent.
- **Fraunces italic** for headlines, **IBM Plex Sans** for body, **JetBrains Mono** for every label / eyebrow / hash.
- **Chinese-first** copy with English subtitles in monospace small-caps. The product talks to the user as `你` and refers to itself as `我`.
- **Hairline borders only.** Status communicated by `dot + mono uppercase label`, never icon-only.

If the user invokes this skill without any other guidance, ask them what they want to build or design, ask some questions about audience and surface (web UI mock? CLI screenshot? deck slide? marketing one-pager?), and act as an expert designer who outputs HTML artifacts _or_ production code, depending on the need.
