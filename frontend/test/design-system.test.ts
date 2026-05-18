// TRACE_MATRIX FC1-N5: Phase 7 W4.4 — design system compliance tests
//
// Asserts the design-system + base-styles bundle exercises the Anthropic
// generative-UI guidance: distinctive typography, no Inter/Roboto/Arial,
// no purple-gradient clichés, semantic tokens declared, dark-mode block
// present.
//
// These are static-string compliance checks, NOT visual regression tests
// (visual self-check is run via Chrome MCP in the §6a verifier; see the
// commit body for the screenshot outcome).

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import { DESIGN_TOKENS } from '../src/design-system.js';
import { BASE_STYLES } from '../src/base-styles.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');

// ---------------------------------------------------------------------------
// Token-presence checks
// ---------------------------------------------------------------------------

test('DESIGN_TOKENS string declares the core color tokens', () => {
  assert.ok(DESIGN_TOKENS.includes('--color-bg'), 'must declare --color-bg');
  assert.ok(DESIGN_TOKENS.includes('--color-fg'), 'must declare --color-fg');
  assert.ok(DESIGN_TOKENS.includes('--color-accent'), 'must declare --color-accent');
});

test('DESIGN_TOKENS string declares the typography tokens', () => {
  assert.ok(DESIGN_TOKENS.includes('--font-display'), 'must declare --font-display');
  assert.ok(DESIGN_TOKENS.includes('--font-mono'), 'must declare --font-mono');
  assert.ok(DESIGN_TOKENS.includes('--font-body'), 'must declare --font-body');
});

test('DESIGN_TOKENS declares status color tokens', () => {
  assert.ok(DESIGN_TOKENS.includes('--color-status-open'));
  assert.ok(DESIGN_TOKENS.includes('--color-status-accepted'));
  assert.ok(DESIGN_TOKENS.includes('--color-status-rejected'));
});

test('DESIGN_TOKENS contains a prefers-color-scheme: dark block', () => {
  assert.ok(
    DESIGN_TOKENS.includes('@media (prefers-color-scheme: dark)'),
    'design system must define a dark-mode @media block',
  );
});

test('DESIGN_TOKENS declares a 4px-based spacing scale', () => {
  assert.ok(DESIGN_TOKENS.includes('--space-1'));
  assert.ok(DESIGN_TOKENS.includes('--space-8'));
});

test('BASE_STYLES string is non-trivial and exports .tos-status', () => {
  assert.ok(BASE_STYLES.length > 50, 'BASE_STYLES must be non-trivial');
  assert.ok(BASE_STYLES.includes('.tos-status'), 'must declare .tos-status badge');
});

// ---------------------------------------------------------------------------
// Anthropic generative-UI compliance checks
// ---------------------------------------------------------------------------

/** Strip CSS /* ... *​/ block comments before checking for forbidden patterns. */
function stripCssComments(s: string): string {
  return s.replace(/\/\*[\s\S]*?\*\//g, '');
}

test('design system does NOT use Inter as a font-family (Anthropic guidance)', () => {
  // Comments may mention Inter (e.g., "no Inter") — only the active rules count.
  const css = stripCssComments(readFileSync(join(srcDir, 'design-system.css'), 'utf8'));
  assert.ok(!/\bInter\b/.test(css), 'design-system.css must not declare Inter in CSS rules');
  assert.ok(!/\bInter\b/.test(DESIGN_TOKENS), 'DESIGN_TOKENS must not declare Inter');
});

test('design system does NOT use Roboto as a font-family (Anthropic guidance)', () => {
  const css = stripCssComments(readFileSync(join(srcDir, 'design-system.css'), 'utf8'));
  assert.ok(!/\bRoboto\b/.test(css), 'design-system.css must not declare Roboto');
  assert.ok(!/\bRoboto\b/.test(DESIGN_TOKENS), 'DESIGN_TOKENS must not declare Roboto');
});

test('design system does NOT use Arial as a font-family (Anthropic guidance)', () => {
  const css = stripCssComments(readFileSync(join(srcDir, 'design-system.css'), 'utf8'));
  assert.ok(!/\bArial\b/.test(css), 'design-system.css must not declare Arial in CSS rules');
  assert.ok(!/\bArial\b/.test(DESIGN_TOKENS), 'DESIGN_TOKENS must not declare Arial');
});

test('design system does NOT use purple gradients (Anthropic cliché check)', () => {
  const css = stripCssComments(readFileSync(join(srcDir, 'design-system.css'), 'utf8'));
  const base = stripCssComments(readFileSync(join(srcDir, 'base-styles.css'), 'utf8'));
  // The specific generic-AI cliché: purple in a linear-gradient.
  const purpleGradient = /linear-gradient\([^)]*purple/i;
  assert.ok(!purpleGradient.test(css), 'design-system.css must not contain purple linear-gradient');
  assert.ok(!purpleGradient.test(base), 'base-styles.css must not contain purple linear-gradient');
  // The word "purple" must not appear as a color token in active CSS rules.
  assert.ok(!/\bpurple\b/i.test(css), 'design-system.css must not contain "purple" in rules');
  assert.ok(!/\bpurple\b/i.test(base), 'base-styles.css must not contain "purple" in rules');
});

test('design system picks editorial+monospace pair (Fraunces + JetBrains Mono)', () => {
  const css = readFileSync(join(srcDir, 'design-system.css'), 'utf8');
  assert.ok(/\bFraunces\b/.test(css), 'design-system must use Fraunces as display face');
  assert.ok(/\bJetBrains Mono\b/.test(css), 'design-system must use JetBrains Mono');
});

// ---------------------------------------------------------------------------
// Render integration: server-side render must reference the new tokens.
// (Sanity check that the CSS made it through include_str! into render.rs.)
// ---------------------------------------------------------------------------

test('canonical CSS files exist on disk for render.rs include_str!', () => {
  assert.ok(existsSync(join(srcDir, 'design-system.css')), 'design-system.css must exist');
  assert.ok(existsSync(join(srcDir, 'base-styles.css')), 'base-styles.css must exist');
});

test('turingos-status component file exists', () => {
  assert.ok(
    existsSync(join(srcDir, 'components', 'turingos-status.ts')),
    'turingos-status.ts component must exist',
  );
});
