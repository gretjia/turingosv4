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
  --color-bg: #FAFAF7;
  --color-bg-elev: #F5F4EE;
  --color-fg: #1A1817;
  --color-fg-muted: #5B5651;
  --color-fg-subtle: #8B847C;
  --color-accent: #1F6E6B;
  --color-accent-soft: #C7DDDB;
  --color-hairline: #E5E3DC;
  --color-hairline-strong: #C9C5BC;
  --color-status-open: #1F6E6B;
  --color-status-accepted: #3F6E3F;
  --color-status-rejected: #9C3A2F;
  --color-status-finalized: #A87431;
  --color-status-bankrupt: #3A3633;
  --color-status-expired: #807974;
  --color-status-solved: #3F6E3F;
  --color-status-exhausted: #807974;
  --color-layer-l4: #3F6E3F;
  --color-layer-l4e: #9C3A2F;
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
  --radius-sm: 2px;
  --radius-md: 3px;
  --border-hairline: 1px solid var(--color-hairline);
  color-scheme: light dark;
}

@media (prefers-color-scheme: dark) {
  :root {
    --color-bg: #14110E;
    --color-bg-elev: #1F1B17;
    --color-fg: #E8E4DA;
    --color-fg-muted: #A29B8F;
    --color-fg-subtle: #75706A;
    --color-accent: #5BB3A6;
    --color-accent-soft: #1F3A38;
    --color-hairline: #2A2724;
    --color-hairline-strong: #3D3935;
    --color-status-open: #5BB3A6;
    --color-status-accepted: #7BAE6F;
    --color-status-rejected: #D87262;
    --color-status-finalized: #D9A057;
    --color-status-bankrupt: #756F69;
    --color-status-expired: #807974;
    --color-status-solved: #7BAE6F;
    --color-status-exhausted: #807974;
    --color-layer-l4: #7BAE6F;
    --color-layer-l4e: #D87262;
  }
}
`;
