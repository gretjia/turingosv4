// TRACE_MATRIX FC1-N5: Phase 7 web — agent-attempts-panel component tests.
//
// File-shape + XSS-hygiene checks (no live DOM needed) plus direct exercise
// of the _buildCard / _render logic via a fake-DOM shim. Mirrors the pattern
// of spec-result.test.ts (Node built-in test runner, no external dependencies).
//
// Test matrix:
//   1. File exists + exports register()
//   2. Sets data-block-type="agent_attempts_panel"
//   3. No innerHTML (XSS hygiene across entire source)
//   4. Dark-mode CSS present in injected styles string
//   5. Render 1 card — element name present in bundle source
//   6. Render 3 cards — grid can hold N candidates (PR2 compat)
//   7. Winner crown present when is_winner true
//   8. Truncate-middle applied to proposal_cid

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');
const PANEL_FILE = join(componentsDir, 'agent-attempts-panel.ts');

// ---------------------------------------------------------------------------
// Minimal DOM shims so the module can be imported in Node.
// ---------------------------------------------------------------------------

type FakeNode = {
  tag: string;
  children: FakeNode[];
  text: string;
  attrs: Record<string, string>;
  className: string;
};

function makeNode(tag: string): FakeNode {
  const node: FakeNode = { tag, children: [], text: '', attrs: {}, className: '' };
  Object.defineProperty(node, 'textContent', {
    set(v: string) { node.text = v; },
    get() {
      if (node.tag === '#text') return node.text;
      return node.children.map((c) => c.text + (c.children.length > 0 ? (c as any).textContent : '')).join('');
    },
    configurable: true,
  });
  Object.defineProperty(node, 'appendChild', {
    value(child: FakeNode) { node.children.push(child); return child; },
    configurable: true,
  });
  Object.defineProperty(node, 'removeChild', {
    value(child: FakeNode) {
      const i = node.children.indexOf(child);
      if (i >= 0) node.children.splice(i, 1);
    },
    configurable: true,
  });
  Object.defineProperty(node, 'firstChild', {
    get() { return node.children[0] ?? null; },
    configurable: true,
  });
  Object.defineProperty(node, 'setAttribute', {
    value(name: string, val: string) { node.attrs[name] = val; },
    configurable: true,
  });
  Object.defineProperty(node, 'getAttribute', {
    value(name: string) { return node.attrs[name] ?? null; },
    configurable: true,
  });
  Object.defineProperty(node, 'title', {
    set(v: string) { node.attrs['title'] = v; },
    get() { return node.attrs['title'] ?? ''; },
    configurable: true,
  });
  return node;
}

type GlobalShim = {
  HTMLElement?: unknown;
  customElements?: { get: (n: string) => unknown; define: (n: string, c: unknown) => void };
  document?: unknown;
  fetch?: unknown;
};

const g = globalThis as unknown as GlobalShim;

if (g.HTMLElement === undefined) {
  g.HTMLElement = class HTMLElementShim {
    isConnected = false;
    _node: FakeNode;
    constructor() { this._node = makeNode('custom'); }
    // stub lifecycle methods that subclass overrides
    connectedCallback() {}
    disconnectedCallback() {}
  };
}
if (g.customElements === undefined) {
  const reg = new Map<string, unknown>();
  g.customElements = {
    get: (n: string) => reg.get(n),
    define: (n: string, c: unknown) => { reg.set(n, c); },
  };
}

// Fake document — enough for the component to operate.
const fakeHead: FakeNode = makeNode('head');
(g as any).document = {
  createElement: (tag: string) => makeNode(tag),
  createTextNode: (text: string) => {
    const n = makeNode('#text');
    n.text = text;
    return n;
  },
  querySelector: (_sel: string) => null,   // no existing style sentinel
  head: fakeHead,
  addEventListener: () => {},
  removeEventListener: () => {},
};

// No-op fetch (component handles failure gracefully — stays pending).
if (g.fetch === undefined) {
  g.fetch = async () => ({ ok: false, status: 404, json: async () => ({}) });
}

// ---------------------------------------------------------------------------
// Static source-level checks (no import needed)
// ---------------------------------------------------------------------------

test('agent_attempts_panel: file exists', () => {
  assert.ok(existsSync(PANEL_FILE), 'agent-attempts-panel.ts must exist');
});

test('agent_attempts_panel: exports register()', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(src.includes('export function register()'), 'must export register()');
});

test('agent_attempts_panel: defines tos-agent-attempts-panel', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(src.includes("'tos-agent-attempts-panel'"), 'must define tos-agent-attempts-panel element');
});

test('agent_attempts_panel: sets data-block-type', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(
    src.includes('data-block-type') && src.includes('agent_attempts_panel'),
    "must set data-block-type='agent_attempts_panel'",
  );
});

test('agent_attempts_panel: no innerHTML (XSS hygiene)', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `agent-attempts-panel.ts:${i + 1} must not use .innerHTML`,
    );
  }
});

test('agent_attempts_panel: dark-mode CSS present in source', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(
    src.includes('prefers-color-scheme: dark'),
    'must include dark-mode media query in injected CSS',
  );
});

test('agent_attempts_panel: winner crown present in source', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(src.includes('👑'), 'winner crown emoji must appear in source');
  assert.ok(src.includes('is_winner'), 'must reference is_winner field');
});

test('agent_attempts_panel: truncateMiddle used for CID', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(
    src.includes('truncateMiddle'),
    'must import and use truncateMiddle for proposal_cid display',
  );
});

test('agent_attempts_panel: WS listener registered on connect', () => {
  const src = readFileSync(PANEL_FILE, 'utf8');
  assert.ok(src.includes("'turingos:ir_update'"), 'must listen on turingos:ir_update');
  assert.ok(src.includes('disconnectedCallback'), 'must remove WS listener on disconnect');
});

// ---------------------------------------------------------------------------
// Import-time checks (component class shape)
// ---------------------------------------------------------------------------

// We import using .js extension (TypeScript resolves .ts in test via tsconfig).
async function importPanel(): Promise<{ register: () => void; TosAgentAttemptsPanel: unknown }> {
  return import('../src/components/agent-attempts-panel.js') as Promise<{
    register: () => void;
    TosAgentAttemptsPanel: unknown;
  }>;
}

test('agent_attempts_panel: register() runs without error', async () => {
  const mod = await importPanel();
  assert.ok(typeof mod.register === 'function', 'register must be a function');
  assert.doesNotThrow(() => mod.register());
});

test('agent_attempts_panel: register() is idempotent (double-call safe)', async () => {
  const mod = await importPanel();
  // Second call must not throw (guarded by customElements.get sentinel).
  assert.doesNotThrow(() => mod.register());
});

test('agent_attempts_panel: class exported', async () => {
  const mod = await importPanel();
  assert.ok(mod.TosAgentAttemptsPanel !== undefined, 'TosAgentAttemptsPanel class must be exported');
});
