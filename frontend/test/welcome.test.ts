// TRACE_MATRIX FC2-N16: Phase 7 W7 — <tos-welcome> component tests.
//
// Pure-logic + source-grep tests; no DOM. Mirrors the established pattern
// (see spec-grill.test.ts, task-open-form.test.ts):
//   - File exists at expected path and exports register()
//   - Sets data-block-type='welcome'
//   - Does NOT use .innerHTML (XSS hygiene)
//   - validateApiKey rejects malformed inputs / accepts a format-valid one
//   - stateForNextStep / stepIndex map correctly
//   - main.ts wires the component into the registry

import { test } from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  validateApiKey,
  stateForNextStep,
  stateForOnboardingStatus,
  stateForProgressIndex,
  stepIndex,
} from '../src/components/welcome-state.js';

const __dirname = dirname(fileURLToPath(import.meta.url));
const srcDir = join(__dirname, '..', 'src');
const componentsDir = join(srcDir, 'components');
const WELCOME_FILE = join(componentsDir, 'welcome.ts');

test('welcome_registers_custom_element: file exists and exports register()', () => {
  assert.ok(existsSync(WELCOME_FILE), 'welcome.ts must exist');
  const src = readFileSync(WELCOME_FILE, 'utf8');
  assert.ok(
    src.includes('export function register()'),
    'welcome.ts must export register()',
  );
  assert.ok(
    src.includes("'tos-welcome-v2'"),
    'welcome.ts must define the tos-welcome-v2 custom element',
  );
});

test('welcome sets data-block-type and respects XSS hygiene', () => {
  const src = readFileSync(WELCOME_FILE, 'utf8');
  assert.ok(
    src.includes('data-block-type') && src.includes("'welcome'"),
    "welcome.ts must set data-block-type='welcome'",
  );
  // XSS: no .innerHTML
  const lines = src.split('\n');
  for (const [i, line] of lines.entries()) {
    const stripped = line.replace(/\/\/.*$/, '');
    assert.ok(
      !stripped.includes('.innerHTML'),
      `welcome.ts:${i + 1} must not use .innerHTML (XSS hygiene)`,
    );
  }
});

test('welcome is registered in main.ts', () => {
  const mainSrc = readFileSync(join(srcDir, 'main.ts'), 'utf8');
  assert.ok(mainSrc.includes('welcome'), 'main.ts must import welcome');
  assert.ok(
    mainSrc.includes('registerWelcome'),
    'main.ts must register welcome',
  );
});

// ---------------------------------------------------------------------------
// validateApiKey — mirror of the backend validate_api_key_shape rules.
// ---------------------------------------------------------------------------

test('welcome_validates_api_key_format: accepts a format-valid key', () => {
  assert.equal(
    validateApiKey('sk-stub-test-key-for-welcome-XXX'),
    null,
    'a format-valid sk- key should pass',
  );
});

test('welcome_validates_api_key_format: rejects missing sk- prefix', () => {
  const r = validateApiKey('plain-junk-no-prefix-1234567890');
  assert.notEqual(r, null, 'missing sk- prefix must fail');
});

test('welcome_validates_api_key_format: rejects too-short key', () => {
  const r = validateApiKey('sk-tiny');
  assert.notEqual(r, null, 'too-short key must fail');
});

test('welcome_validates_api_key_format: rejects too-long key', () => {
  const tooLong = 'sk-' + 'x'.repeat(300);
  const r = validateApiKey(tooLong);
  assert.notEqual(r, null, 'too-long key must fail');
});

test('welcome_validates_api_key_format: rejects non-printable chars', () => {
  const withCtl = 'sk-abcdefghij\x01klmnop';
  const r = validateApiKey(withCtl);
  assert.notEqual(r, null, 'control chars must fail');
});

test('welcome_validates_api_key_format: rejects empty string', () => {
  const r = validateApiKey('');
  assert.notEqual(r, null, 'empty must fail');
});

// ---------------------------------------------------------------------------
// stateForNextStep — maps backend NextStep to the wizard state machine.
// ---------------------------------------------------------------------------

test('welcome_state_machine_advances: NextStep -> WizardState', () => {
  assert.equal(stateForNextStep('Init'), 'step_init');
  assert.equal(stateForNextStep('LlmConfig'), 'step_llm_config');
  assert.equal(stateForNextStep('ApiKey'), 'step_api_key');
  assert.equal(stateForNextStep('AgentDeploy'), 'step_agent_deploy');
  assert.equal(stateForNextStep('Spec'), 'step_ready');
  assert.equal(stateForNextStep('Generate'), 'step_ready');
  assert.equal(stateForNextStep('Done'), 'step_ready');
});

test('welcome_state_machine_advances: stepIndex monotonic', () => {
  assert.equal(stepIndex('Init'), 0);
  assert.equal(stepIndex('LlmConfig'), 1);
  assert.equal(stepIndex('ApiKey'), 2);
  assert.equal(stepIndex('AgentDeploy'), 3);
  assert.equal(stepIndex('Spec'), 4);
  assert.equal(stepIndex('Generate'), 4);
  assert.equal(stepIndex('Done'), 4);
});

test('welcome_state_machine_guards_ready_when_api_key_missing', () => {
  assert.equal(
    stateForOnboardingStatus({
      init_done: true,
      llm_config_done: true,
      api_key_set: false,
      agents_count: 1,
      next_step: 'Spec',
    }),
    'step_api_key',
  );
});

test('welcome_progress_buttons_do_not_bypass_missing_api_key', () => {
  const status = {
    init_done: true,
    llm_config_done: true,
    api_key_set: false,
    agents_count: 1,
    next_step: 'ApiKey' as const,
  };
  assert.equal(stateForProgressIndex(status, 4), 'step_api_key');
  assert.equal(stateForProgressIndex(status, 2), 'step_api_key');
});

// ---------------------------------------------------------------------------
// Type contract: ir.ts must export OnboardingStatus + NextStep
// ---------------------------------------------------------------------------

test('ir.ts exports OnboardingStatus + NextStep + ApiKeyRequest', () => {
  const irSrc = readFileSync(join(srcDir, 'ir.ts'), 'utf8');
  assert.ok(
    irSrc.includes('OnboardingStatus'),
    'ir.ts must export OnboardingStatus',
  );
  assert.ok(irSrc.includes('NextStep'), 'ir.ts must export NextStep');
  assert.ok(
    irSrc.includes('ApiKeyRequest'),
    'ir.ts must export ApiKeyRequest',
  );
});

// ---------------------------------------------------------------------------
// router.ts must accept 'welcome' view
// ---------------------------------------------------------------------------

test('router.ts recognizes /welcome pathname', () => {
  const src = readFileSync(join(srcDir, 'router.ts'), 'utf8');
  assert.ok(src.includes("'welcome'"), 'router.ts must include welcome view');
  assert.ok(src.includes("'/welcome'"), 'router.ts must match /welcome path');
});
