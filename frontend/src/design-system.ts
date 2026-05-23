// TRACE_MATRIX FC1-N5: read view materialization — design system tokens
//
// Phase 7 W4.4 design system. Exports CSS variable token set as a const
// string for client-side use (constructed stylesheets, runtime injection,
// future Shadow DOM scopes). The canonical authoritative copy is at
// `frontend/src/design-system.css`; this file mirrors it as a TS literal
// so esbuild can bundle it without disk I/O at runtime.
//
// Sibling: `frontend/src/base-styles.ts` (rendered atom + block styles).
//
// Tokens follow Anthropic generative-UI guidance (no Inter/Roboto/Arial,
// no purple gradients, distinctive editorial+monospace pair, hairline
// borders, restrained accents). Dark mode declared via
// @media (prefers-color-scheme: dark).

export const DESIGN_TOKENS: string = `
:root {
  --color-bg: #F5F0E5;
  --color-bg-elev: #EEE7D6;
  --color-bg-bright: #FBF7EE;
  --color-fg: #1E1813;
  --color-fg-muted: #5E5448;
  --color-fg-subtle: #8E8474;
  --color-accent: #2D7A72;
  --color-accent-bright: #3A958A;
  --color-accent-soft: #D5E5DF;
  --color-accent-glow: #B8D6CE;
  --color-hairline: #E0D9C7;
  --color-hairline-strong: #C3BAA5;
  --color-status-open: #2D7A72;
  --color-status-accepted: #557F4A;
  --color-status-rejected: #A6493D;
  --color-status-finalized: #B1813D;
  --color-status-bankrupt: #463E36;
  --color-status-expired: #8E8474;
  --color-status-solved: #557F4A;
  --color-status-exhausted: #8E8474;
  --color-layer-l4: #557F4A;
  --color-layer-l4e: #A6493D;
  --font-display: "Fraunces", "Iowan Old Style", "Baskerville", Times, serif;
  --font-mono: "JetBrains Mono", "IBM Plex Mono", "SF Mono", Menlo, ui-monospace, monospace;
  --font-body: "IBM Plex Sans", "Söhne", ui-sans-serif, sans-serif;
  --space-1: 0.25rem;
  --space-2: 0.5rem;
  --space-3: 0.75rem;
  --space-4: 1rem;
  --space-5: 1.5rem;
  --space-6: 2rem;
  --space-7: 3rem;
  --space-8: 4rem;
  --space-9: 6rem;
  --radius-sm: 2px;
  --radius-md: 3px;
  --border-hairline: 1px solid var(--color-hairline);
  color-scheme: light dark;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #1B1612;
    --color-bg-elev: #25201A;
    --color-bg-bright: #2C261F;
    --color-fg: #ECE4D2;
    --color-fg-muted: #A89E8B;
    --color-fg-subtle: #7A7066;
    --color-accent: #6BBFB1;
    --color-accent-bright: #82D2C4;
    --color-accent-soft: #1F3C39;
    --color-accent-glow: #2D5651;
    --color-hairline: #2F2A23;
    --color-hairline-strong: #423C32;
    --color-status-open: #6BBFB1;
    --color-status-accepted: #88B97B;
    --color-status-rejected: #DC7E6F;
    --color-status-finalized: #E0AC68;
    --color-status-bankrupt: #756F69;
    --color-status-expired: #807974;
    --color-status-solved: #88B97B;
    --color-status-exhausted: #807974;
    --color-layer-l4: #88B97B;
    --color-layer-l4e: #DC7E6F;
  }
}
`;
