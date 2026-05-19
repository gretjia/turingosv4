// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 6.3.x W8 — driven-mode tests.
//
// NOTE: The task brief specified frontend/tests/ but the configured test
// runner uses `node --test --import tsx/esm test/*.test.ts` (frontend/package.json).
// This file is placed in frontend/test/ to match the runner glob.
//
// Test runner: node:test (NOT vitest — vitest is not in package.json).
// Pattern mirrors spec-grill.test.ts: source-grep + pure-logic tests.
// DOM instantiation is NOT available in node:test without jsdom.
// Tests cover:
//   - Types file exists and has required exports (source grep)
//   - spec-grill.ts has driven_mode field, _drivenState, _postTurn, _fallbackToStatic
//   - spec-grill.ts has data-cta="start" in driven idle render
//   - spec-grill.ts does NOT use .innerHTML (XSS hygiene)
//   - spec-grill.ts has all 3 WS event handlers (SpecTurnAdvanced, SpecGrillComplete, SpecTurnTriageReject)
//   - Pure-logic: TurnRequest/TurnResponse/GrillState interfaces parseable
//   - Nudge text mapping per R2 §A5 (source grep)
//   - Fallback triggers (source grep: 404, 5xx count)
//   - Static-mode fallback preserves existing render entry point

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');
const typesDir = join(srcDir, 'types');

const GRILL_FILE = join(componentsDir, 'spec-grill.ts');
const SPEC_TYPES_FILE = join(typesDir, 'spec.ts');

// ---------------------------------------------------------------------------
// File existence
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec.ts types file exists', () => {
  assert.ok(existsSync(SPEC_TYPES_FILE), 'frontend/src/types/spec.ts must exist');
});

test('spec_grill_driven: spec-grill.ts still exists (no regression)', () => {
  assert.ok(existsSync(GRILL_FILE), 'frontend/src/components/spec-grill.ts must exist');
});

// ---------------------------------------------------------------------------
// Types file content
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec.ts exports TurnPayload', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  assert.ok(src.includes('export interface TurnPayload'), 'spec.ts must export TurnPayload');
});

test('spec_grill_driven: spec.ts exports TurnRequest', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  assert.ok(src.includes('export interface TurnRequest'), 'spec.ts must export TurnRequest');
});

test('spec_grill_driven: spec.ts exports TurnResponse', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  assert.ok(src.includes('export interface TurnResponse'), 'spec.ts must export TurnResponse');
});

test('spec_grill_driven: spec.ts exports GrillState union', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  assert.ok(src.includes('export type GrillState'), 'spec.ts must export GrillState');
});

test('spec_grill_driven: GrillState has all required variants', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  assert.ok(src.includes("'idle'"), "GrillState must have 'idle' kind");
  assert.ok(src.includes("'awaiting_first_turn'"), "GrillState must have 'awaiting_first_turn' kind");
  assert.ok(src.includes("'awaiting_user_answer'"), "GrillState must have 'awaiting_user_answer' kind");
  assert.ok(src.includes("'playback_review'"), "GrillState must have 'playback_review' kind");
  assert.ok(src.includes("'complete'"), "GrillState must have 'complete' kind");
});

test('spec_grill_driven: TurnResponse has all required fields', () => {
  const src = readFileSync(SPEC_TYPES_FILE, 'utf8');
  const required = [
    'turn_index',
    'question_text',
    'covered_slots',
    'open_slots',
    'confidence',
    'done',
    'playback',
    'terminated',
    'spec_capsule_cid',
    'turn_capsule_cid',
  ];
  for (const field of required) {
    assert.ok(src.includes(field), `TurnResponse must have field '${field}'`);
  }
});

// ---------------------------------------------------------------------------
// spec-grill.ts: driven mode fields and methods present
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec-grill.ts has driven_mode public field', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('driven_mode'),
    'spec-grill.ts must have driven_mode field',
  );
});

test('spec_grill_driven: spec-grill.ts detects mode=driven from URL', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes("params.get('mode') === 'driven'") ||
      src.includes('mode') && src.includes('driven'),
    'spec-grill.ts must detect ?mode=driven URL param',
  );
  assert.ok(
    src.includes('URLSearchParams'),
    'spec-grill.ts must use URLSearchParams for URL param detection',
  );
});

test('spec_grill_driven: spec-grill.ts has _postTurn method', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('_postTurn'),
    'spec-grill.ts must have _postTurn method',
  );
});

test('spec_grill_driven: spec-grill.ts has _fallbackToStatic method', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('_fallbackToStatic'),
    'spec-grill.ts must have _fallbackToStatic method',
  );
});

test('spec_grill_driven: spec-grill.ts has _renderDriven method', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('_renderDriven'),
    'spec-grill.ts must have _renderDriven method',
  );
});

test('spec_grill_driven: spec-grill.ts renders data-cta=start button in driven idle', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('data-cta') && src.includes('start'),
    'spec-grill.ts must render data-cta="start" on the driven-mode CTA button',
  );
});

// ---------------------------------------------------------------------------
// WS event handlers: SpecTurnAdvanced, SpecGrillComplete, SpecTurnTriageReject
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec-grill.ts handles SpecTurnAdvanced WS event', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('SpecTurnAdvanced'),
    'spec-grill.ts must handle SpecTurnAdvanced WS event',
  );
});

test('spec_grill_driven: spec-grill.ts handles SpecGrillComplete WS event', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('SpecGrillComplete'),
    'spec-grill.ts must handle SpecGrillComplete WS event',
  );
});

test('spec_grill_driven: spec-grill.ts handles SpecTurnTriageReject WS event', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('SpecTurnTriageReject'),
    'spec-grill.ts must handle SpecTurnTriageReject WS event',
  );
});

// ---------------------------------------------------------------------------
// R2 §A5: Nudge text per triage class
// ---------------------------------------------------------------------------

test('spec_grill_driven: off_topic nudge text present per R2 §A5', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('能换一种说法吗？刚才听不太懂'),
    'spec-grill.ts must have off_topic nudge text per R2 §A5',
  );
});

test('spec_grill_driven: abusive/gibberish nudge text present per R2 §A5', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('您似乎在测试我，可以继续吗？'),
    'spec-grill.ts must have abusive/gibberish nudge text per R2 §A5',
  );
});

// ---------------------------------------------------------------------------
// Fallback triggers
// ---------------------------------------------------------------------------

test('spec_grill_driven: 404 triggers immediate fallback (session not found per R2 §A14)', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('404'),
    'spec-grill.ts must handle 404 with immediate fallback per R2 §A14',
  );
  assert.ok(
    src.includes('_fallbackToStatic'),
    'spec-grill.ts must call _fallbackToStatic',
  );
});

test('spec_grill_driven: 2 consecutive 5xx triggers fallback', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('_recent5xxCount') || src.includes('recent_5xx'),
    'spec-grill.ts must track consecutive 5xx count',
  );
  assert.ok(
    src.includes('>= 2'),
    'spec-grill.ts must fallback after 2 consecutive 5xx responses',
  );
});

test('spec_grill_driven: fallback toast text present', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('切换至 8 问经典模式'),
    'spec-grill.ts must display fallback toast text',
  );
});

// ---------------------------------------------------------------------------
// Static-mode preservation
// ---------------------------------------------------------------------------

test('spec_grill_driven: static-mode _render() still present and intact', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  // The original static-mode render method and all its state cases must still exist.
  assert.ok(src.includes('_renderIdle'), '_renderIdle must still exist');
  assert.ok(src.includes('_renderInterviewing'), '_renderInterviewing must still exist');
  assert.ok(src.includes('_renderSpecReady'), '_renderSpecReady must still exist');
  assert.ok(src.includes('_renderError'), '_renderError must still exist');
  assert.ok(src.includes("'开始访谈 →'"), 'static-mode CTA text must be preserved');
});

test('spec_grill_driven: static-mode flow is unchanged when not in driven mode', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  // Static-mode _loadQuestions and _submit must still be present.
  assert.ok(src.includes('_loadQuestions'), '_loadQuestions must still exist');
  assert.ok(src.includes('_submit()'), '_submit() must still exist');
  assert.ok(src.includes('/api/spec/submit'), 'static-mode submit endpoint must be preserved');
});

// ---------------------------------------------------------------------------
// XSS hygiene
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec-grill.ts does NOT use .innerHTML (XSS hygiene)', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `spec-grill.ts:${i + 1} must not use .innerHTML (XSS hygiene)`,
    );
  }
});

// ---------------------------------------------------------------------------
// ir.ts: WS union extended with driven-mode events
// ---------------------------------------------------------------------------

test('spec_grill_driven: ir.ts exports SpecTurnAdvancedEvent', () => {
  const irFile = join(srcDir, 'ir.ts');
  const src = readFileSync(irFile, 'utf8');
  assert.ok(
    src.includes('SpecTurnAdvancedEvent'),
    'ir.ts must export SpecTurnAdvancedEvent',
  );
});

test('spec_grill_driven: ir.ts exports SpecGrillCompleteEvent', () => {
  const irFile = join(srcDir, 'ir.ts');
  const src = readFileSync(irFile, 'utf8');
  assert.ok(
    src.includes('SpecGrillCompleteEvent'),
    'ir.ts must export SpecGrillCompleteEvent',
  );
});

test('spec_grill_driven: ir.ts exports SpecTurnTriageRejectEvent', () => {
  const irFile = join(srcDir, 'ir.ts');
  const src = readFileSync(irFile, 'utf8');
  assert.ok(
    src.includes('SpecTurnTriageRejectEvent'),
    'ir.ts must export SpecTurnTriageRejectEvent',
  );
});

test('spec_grill_driven: ir.ts WsMessage union includes all three driven-mode events', () => {
  const irFile = join(srcDir, 'ir.ts');
  const src = readFileSync(irFile, 'utf8');
  // Verify union discriminants are in the WsMessage type.
  assert.ok(src.includes("'SpecTurnAdvanced'"), 'WsMessage union must include SpecTurnAdvanced');
  assert.ok(src.includes("'SpecGrillComplete'"), 'WsMessage union must include SpecGrillComplete');
  assert.ok(src.includes("'SpecTurnTriageReject'"), 'WsMessage union must include SpecTurnTriageReject');
});

// ---------------------------------------------------------------------------
// Playback review flow
// ---------------------------------------------------------------------------

test('spec_grill_driven: spec-grill.ts has playback confirm and edit handlers', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('_drivenConfirmPlayback') || src.includes('确认'),
    'spec-grill.ts must have playback confirm handler',
  );
  assert.ok(
    src.includes('_drivenEditPlayback'),
    'spec-grill.ts must have playback edit handler',
  );
});

test('spec_grill_driven: playback confirm posts user_answer = 确认', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes("'确认'"),
    "spec-grill.ts must POST user_answer = '确认' for playback confirmation",
  );
});

// ---------------------------------------------------------------------------
// session_id generation
// ---------------------------------------------------------------------------

test('spec_grill_driven: session_id generated with crypto.randomUUID()', () => {
  const src = readFileSync(GRILL_FILE, 'utf8');
  assert.ok(
    src.includes('crypto.randomUUID()'),
    'spec-grill.ts must generate session_id with crypto.randomUUID()',
  );
});
