// TRACE_MATRIX FC1-N5: Phase 7 W6 — spec-result component tests.
//
// File-shape + XSS-hygiene checks (no DOM needed) plus a direct exercise of
// the exported `renderMarkdownInto` walker via a minimal fake-DOM target.
// The walker is the load-bearing piece — anything its tests don't cover
// shows up in the §6a visual verifier.

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

// Node has no HTMLElement / customElements at module load — the component
// module's `class TosSpecResult extends HTMLElement` would throw without
// these minimal shims. We additionally provide a fake `document` further
// down so the renderMarkdownInto walker can build trees.
type GlobalShim2 = {
  HTMLElement?: unknown;
  customElements?: { get: (n: string) => unknown; define: (n: string, c: unknown) => void };
  document?: unknown;
};
const gShim = globalThis as unknown as GlobalShim2;
if (gShim.HTMLElement === undefined) {
  gShim.HTMLElement = class HTMLElementShim {};
}
if (gShim.customElements === undefined) {
  const reg = new Map<string, unknown>();
  gShim.customElements = {
    get: (n: string) => reg.get(n),
    define: (n: string, c: unknown) => {
      reg.set(n, c);
    },
  };
}

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');

const RESULT_FILE = join(componentsDir, 'spec-result.ts');

test('spec_result_registers_custom_element: file exists + exports register()', () => {
  assert.ok(existsSync(RESULT_FILE), 'spec-result.ts must exist');
  const src = readFileSync(RESULT_FILE, 'utf8');
  assert.ok(
    src.includes('export function register()'),
    'spec-result.ts must export register()',
  );
  assert.ok(
    src.includes("'tos-spec-result'"),
    'spec-result.ts must define the tos-spec-result custom element',
  );
});

test('spec-result sets data-block-type and respects XSS hygiene', () => {
  const src = readFileSync(RESULT_FILE, 'utf8');
  assert.ok(
    src.includes('data-block-type') && src.includes('spec_result'),
    "spec-result.ts must set data-block-type='spec_result'",
  );
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `spec-result.ts:${i + 1} must not use .innerHTML (XSS hygiene)`,
    );
  }
});

// ---------------------------------------------------------------------------
// Direct test of the markdown walker.
//
// We build a minimal fake-DOM (just enough surface for the walker) so the
// pure logic runs in Node without happy-dom/jsdom. The walker only calls:
//   - document.createElement(tag)
//   - elem.appendChild(child)
//   - text node creation via document.createTextNode
//   - .textContent assignment
// All of these are easy to fake.
// ---------------------------------------------------------------------------

interface FakeNode {
  tag: string;
  children: FakeNode[];
  text: string;
}

function makeFakeDom(): {
  doc: {
    createElement: (tag: string) => FakeNode;
    createTextNode: (text: string) => FakeNode;
  };
  root: FakeNode;
} {
  const doc = {
    createElement: (tag: string): FakeNode => ({
      tag,
      children: [],
      get text() { return ''; },
      set text(_v: string) { /* ignored — textContent setter goes through proxy */ },
    } as unknown as FakeNode),
    createTextNode: (text: string): FakeNode => ({ tag: '#text', children: [], text }),
  };
  return { doc, root: { tag: 'div', children: [], text: '' } };
}

// Wire up the fake DOM globally before importing the module under test.
const fakeDom = makeFakeDom();

// Patch element prototype with appendChild + textContent.
function makeElementProxy(node: FakeNode): FakeNode {
  Object.defineProperty(node, 'appendChild', {
    value: (child: FakeNode) => {
      node.children.push(child);
      return child;
    },
    enumerable: false,
  });
  Object.defineProperty(node, 'textContent', {
    set(value: string) {
      node.text = value;
    },
    get() {
      if (node.tag === '#text') return node.text;
      return node.text + node.children.map((c) => (c as unknown as { textContent: string }).textContent ?? '').join('');
    },
    configurable: true,
  });
  return node;
}

// Globally polyfill `document` for the import.
type GlobalishDocument = {
  createElement: (tag: string) => FakeNode;
  createTextNode: (text: string) => FakeNode;
};

(globalThis as unknown as { document: GlobalishDocument }).document = {
  createElement: (tag: string) => {
    const n: FakeNode = { tag, children: [], text: '' };
    return makeElementProxy(n);
  },
  createTextNode: (text: string) => {
    const n: FakeNode = { tag: '#text', children: [], text };
    return makeElementProxy(n);
  },
};

// Import inside each test via dynamic import to avoid top-level await
// (tsconfig.test.json uses module:ES2020 which forbids it).
async function getRenderer(): Promise<
  (target: HTMLElement, md: string) => void
> {
  const mod = (await import('../src/components/spec-result.js')) as {
    renderMarkdownInto: (target: HTMLElement, md: string) => void;
  };
  return mod.renderMarkdownInto;
}

test('spec_result_renders_markdown_headings', async () => {
  const renderMarkdownInto = await getRenderer();
  const root = (globalThis as unknown as { document: GlobalishDocument }).document.createElement(
    'div',
  );
  const md = '# Title\n\n## Section A\n\nA paragraph here.\n\n### Sub heading';
  renderMarkdownInto(root as unknown as HTMLElement, md);

  const tags = root.children.map((c) => c.tag);
  assert.deepEqual(tags, ['h1', 'h2', 'p', 'h3'], 'must render headings + paragraph in order');

  // h1 must contain text "Title" (via descendant text node)
  const h1 = root.children[0]!;
  const h1Text = (h1 as unknown as { textContent: string }).textContent;
  assert.ok(h1Text.includes('Title'), `h1 must contain 'Title', got ${h1Text}`);
});

test('spec-result markdown walker handles lists', async () => {
  const renderMarkdownInto = await getRenderer();
  const root = (globalThis as unknown as { document: GlobalishDocument }).document.createElement(
    'div',
  );
  const md = '- one\n- two\n- three\n\n1. first\n2. second';
  renderMarkdownInto(root as unknown as HTMLElement, md);
  const tags = root.children.map((c) => c.tag);
  assert.deepEqual(tags, ['ul', 'ol'], 'must render ul then ol');
  assert.equal(root.children[0]!.children.length, 3, 'ul must have 3 items');
  assert.equal(root.children[1]!.children.length, 2, 'ol must have 2 items');
});

test('spec-result markdown walker handles fenced code blocks', async () => {
  const renderMarkdownInto = await getRenderer();
  const root = (globalThis as unknown as { document: GlobalishDocument }).document.createElement(
    'div',
  );
  const md = '```\nconst x = 1;\n```\n\nafter';
  renderMarkdownInto(root as unknown as HTMLElement, md);
  const tags = root.children.map((c) => c.tag);
  assert.deepEqual(tags, ['pre', 'p'], 'must render pre + paragraph');
});

test('spec-result markdown walker treats unknown lines as paragraphs', async () => {
  const renderMarkdownInto = await getRenderer();
  const root = (globalThis as unknown as { document: GlobalishDocument }).document.createElement(
    'div',
  );
  const md = 'Just a normal line of body text.';
  renderMarkdownInto(root as unknown as HTMLElement, md);
  assert.equal(root.children.length, 1, 'plain prose must produce one paragraph');
  assert.equal(root.children[0]!.tag, 'p');
});
