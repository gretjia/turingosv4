// TRACE_MATRIX FC2-N16: Phase 7 W7 — welcome wizard pure-logic helpers.
//
// Extracted into their own module so the Node test runner can import them
// without pulling in HTMLElement (which is undefined in Node). The
// <tos-welcome-v2> Web Component (welcome.ts) imports these.

import type { NextStep, OnboardingStatus } from '../ir.js';

const API_KEY_MIN = 16;
const API_KEY_MAX = 256;

export type WizardState =
  | 'loading_status'
  | 'step_init'
  | 'step_llm_config'
  | 'step_api_key'
  | 'step_agent_deploy'
  | 'step_ready'
  | 'submitting_init'
  | 'submitting_llm_config'
  | 'submitting_api_key'
  | 'submitting_agent_deploy'
  | 'error_init'
  | 'error_llm_config'
  | 'error_api_key'
  | 'error_agent_deploy'
  | 'error_status';

/**
 * Pure validator (mirrored by backend `validate_api_key_shape`). Returns
 * null on pass, else a Chinese error message.
 */
export function validateApiKey(key: string): string | null {
  if (!key.startsWith('sk-')) {
    return 'API 密钥需要以 "sk-" 开头（SiliconFlow / OpenAI 习惯）。';
  }
  if (key.length < API_KEY_MIN) {
    return `密钥太短了：${key.length} 字符，至少 ${API_KEY_MIN}。`;
  }
  if (key.length > API_KEY_MAX) {
    return `密钥太长了：${key.length} 字符，最多 ${API_KEY_MAX}。`;
  }
  for (let i = 0; i < key.length; i++) {
    const c = key.charCodeAt(i);
    if (c < 33 || c > 126) {
      return '密钥只能包含可见 ASCII 字符。';
    }
  }
  return null;
}

/** Map a NextStep value from the backend into the active wizard state. */
export function stateForNextStep(next: NextStep): WizardState {
  switch (next) {
    case 'Init':
      return 'step_init';
    case 'LlmConfig':
      return 'step_llm_config';
    case 'ApiKey':
      return 'step_api_key';
    case 'AgentDeploy':
      return 'step_agent_deploy';
    case 'Spec':
    case 'Generate':
    case 'Done':
      return 'step_ready';
  }
}

/**
 * Defensive view state derived from the full onboarding snapshot. `next_step`
 * is still the backend's compact answer, but the prerequisite booleans are
 * cheap guard rails for old bundles, stale pages, or inconsistent fixtures.
 */
export function stateForOnboardingStatus(
  status: Pick<
    OnboardingStatus,
    'next_step' | 'init_done' | 'llm_config_done' | 'api_key_set' | 'agents_count'
  >,
): WizardState {
  if (!status.init_done) return 'step_init';
  if (!status.llm_config_done) return 'step_llm_config';
  if (!status.api_key_set) return 'step_api_key';
  if (status.agents_count === 0) return 'step_agent_deploy';
  return stateForNextStep(status.next_step);
}

/**
 * Manual progress navigation. Users may go back to an already available step
 * (for example to replace the API key), but clicking a future step clamps to
 * the next unmet prerequisite instead of bypassing it.
 */
export function stateForProgressIndex(
  status: Pick<
    OnboardingStatus,
    'next_step' | 'init_done' | 'llm_config_done' | 'api_key_set' | 'agents_count'
  >,
  index: number,
): WizardState {
  if (index === 0) return 'step_init';
  if (index === 1) {
    return status.init_done ? 'step_llm_config' : stateForOnboardingStatus(status);
  }
  if (index === 2) {
    return status.init_done && status.llm_config_done
      ? 'step_api_key'
      : stateForOnboardingStatus(status);
  }
  if (index === 3) {
    return status.init_done && status.llm_config_done && status.api_key_set
      ? 'step_agent_deploy'
      : stateForOnboardingStatus(status);
  }
  if (index === 4) {
    return status.init_done &&
      status.llm_config_done &&
      status.api_key_set &&
      status.agents_count > 0
      ? 'step_ready'
      : stateForOnboardingStatus(status);
  }
  return stateForOnboardingStatus(status);
}

/** Order index for a NextStep in the progress indicator (0..=4). */
export function stepIndex(next: NextStep): number {
  switch (next) {
    case 'Init':
      return 0;
    case 'LlmConfig':
      return 1;
    case 'ApiKey':
      return 2;
    case 'AgentDeploy':
      return 3;
    case 'Spec':
    case 'Generate':
    case 'Done':
      return 4;
  }
}

/** The 5 visible wizard step keys + Chinese labels for the progress indicator. */
export const WIZARD_STEPS: ReadonlyArray<{
  key: 'init' | 'llm_config' | 'api_key' | 'agent_deploy' | 'ready';
  label: string;
}> = [
  { key: 'init', label: '工作站' },
  { key: 'llm_config', label: '模型配置' },
  { key: 'api_key', label: 'API 密钥' },
  { key: 'agent_deploy', label: '注册 Agent' },
  { key: 'ready', label: '开始访谈' },
];
