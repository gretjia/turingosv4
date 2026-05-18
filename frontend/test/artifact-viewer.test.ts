// TRACE_MATRIX FC1-N5: Phase 7 W6 — artifact-viewer component tests.
//
// File-shape, XSS-hygiene, and the load-bearing sandbox security checks.
// The sandbox guard is critical: `allow-scripts` MUST coexist with the
// iframe being cross-origin (no `allow-same-origin`); together they
// constitute a known XSS bypass per OWASP iframe sandbox guidance.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// Node has no HTMLElement / customElements / document at module load. The
// component module's `class TosArtifactViewer extends HTMLElement` would
// fail if we let it evaluate naked, so shim minimally — none of the
// helpers under test touch any of these surfaces at all, but the class
// definition itself does.
type GlobalShim = {
  HTMLElement?: unknown;
  customElements?: { get: (n: string) => unknown; define: (n: string, c: unknown) => void };
  document?: { createElement: (tag: string) => unknown; createTextNode: (s: string) => unknown };
};
const g = globalThis as unknown as GlobalShim;
if (g.HTMLElement === undefined) {
  g.HTMLElement = class HTMLElementShim {};
}
if (g.customElements === undefined) {
  const reg = new Map<string, unknown>();
  g.customElements = {
    get: (n: string) => reg.get(n),
    define: (n: string, c: unknown) => {
      reg.set(n, c);
    },
  };
}

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');

const VIEWER_FILE = join(componentsDir, 'artifact-viewer.ts');

test('artifact_viewer_registers_custom_element: file exists + exports register()', () => {
  assert.ok(existsSync(VIEWER_FILE), 'artifact-viewer.ts must exist');
  const src = readFileSync(VIEWER_FILE, 'utf8');
  assert.ok(
    src.includes('export function register()'),
    'artifact-viewer.ts must export register()',
  );
  assert.ok(
    src.includes("'tos-artifact-viewer'"),
    'artifact-viewer.ts must define the tos-artifact-viewer custom element',
  );
});

test('artifact-viewer sets data-block-type and respects XSS hygiene', () => {
  const src = readFileSync(VIEWER_FILE, 'utf8');
  assert.ok(
    src.includes('data-block-type') && src.includes('artifact_viewer'),
    "artifact-viewer.ts must set data-block-type='artifact_viewer'",
  );
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `artifact-viewer.ts:${i + 1} must not use .innerHTML (XSS hygiene)`,
    );
  }
});

// ---------------------------------------------------------------------------
// Sandbox helpers — dynamic-import inside each test to avoid top-level await.
// (artifact-viewer.ts imports nothing browser-specific at module scope, so
// it can be required from Node without a fake DOM.)
// ---------------------------------------------------------------------------

async function getSandboxHelpers(): Promise<{
  buildSandboxAttribute: () => string;
  isSafeSandboxValue: (v: string) => boolean;
}> {
  const mod = (await import('../src/components/artifact-viewer.js')) as {
    buildSandboxAttribute: () => string;
    isSafeSandboxValue: (v: string) => boolean;
  };
  return { buildSandboxAttribute: mod.buildSandboxAttribute, isSafeSandboxValue: mod.isSafeSandboxValue };
}

test('artifact_viewer_constructs_iframe_with_sandbox_attribute', async () => {
  const { buildSandboxAttribute } = await getSandboxHelpers();
  const value = buildSandboxAttribute();
  const tokens = value.split(/\s+/).filter((t) => t.length > 0);
  assert.ok(tokens.includes('allow-scripts'), 'sandbox must include allow-scripts');
  // The exact value is "allow-scripts" — no other tokens.
  assert.equal(value, 'allow-scripts');
});

test('artifact_viewer_blocks_dangerous_sandbox_combinations', async () => {
  const { isSafeSandboxValue } = await getSandboxHelpers();
  // The known XSS bypass: allow-scripts + allow-same-origin.
  assert.equal(
    isSafeSandboxValue('allow-scripts allow-same-origin'),
    false,
    'allow-scripts + allow-same-origin must be flagged as unsafe',
  );
  assert.equal(
    isSafeSandboxValue('allow-same-origin allow-scripts'),
    false,
    'token order must not matter',
  );
  // Safe values:
  assert.equal(isSafeSandboxValue('allow-scripts'), true);
  assert.equal(isSafeSandboxValue('allow-scripts allow-forms'), true);
  assert.equal(isSafeSandboxValue(''), true, 'empty sandbox is the strictest (safe)');
});

test('artifact-viewer source actually applies buildSandboxAttribute() to the iframe', () => {
  // Read the source and confirm the sandbox attribute is set from the helper.
  const src = readFileSync(VIEWER_FILE, 'utf8');
  assert.ok(
    src.includes("setAttribute('sandbox', buildSandboxAttribute())"),
    'iframe must apply buildSandboxAttribute() to sandbox attr',
  );
  // The source MUST NOT contain the literal allow-same-origin token paired
  // with allow-scripts anywhere — defensive belt-and-braces.
  const lowered = src.toLowerCase();
  // We allow the test-related guard token reference ('allow-same-origin' in
  // a guard list / docstring), but not in a sandbox attribute assignment.
  const sandboxAssignmentRegex = /setAttribute\(\s*['"]sandbox['"]\s*,\s*['"][^'"]*allow-same-origin[^'"]*['"]/i;
  assert.ok(
    !sandboxAssignmentRegex.test(lowered),
    'artifact-viewer.ts must never directly set sandbox="...allow-same-origin..."',
  );
});

test('artifact-viewer is registered in main.ts', () => {
  const mainSrc = readFileSync(join(srcDir, 'main.ts'), 'utf8');
  assert.ok(mainSrc.includes('artifact-viewer'), 'main.ts must import artifact-viewer');
});
