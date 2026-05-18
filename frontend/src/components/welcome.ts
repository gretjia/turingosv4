// TRACE_MATRIX FC2-N16: Phase 7 W7 — onboarding wizard centerpiece.
// <tos-welcome> guides a non-developer through the Phase 6.3 5-step flow
// inside the browser: workspace init, LLM config, API-key entry,
// agent deploy, then a final "ready to interview" panel that hands off to
// /build. State machine:
//   loading_status → step_init | step_llm_config | step_api_key | step_agent_deploy | step_ready
// Each step has submitting_<step> + error_<step> sub-states. The API key
// step uses <input type="password"> and posts to /api/welcome/api-key;
// the value is NEVER echoed back from the server or rendered in the DOM
// after first set. XSS hygiene: textContent / createElement only; no
// dynamic innerHTML. Sets data-block-type="welcome" on self.

import type { OnboardingStatus } from '../ir.js';
import {
  validateApiKey,
  stateForNextStep,
  stepIndex,
  WIZARD_STEPS,
  type WizardState,
} from './welcome-state.js';

const ELEMENT_NAME = 'tos-welcome';

// Re-export so callers / tests that prefer the component path keep working.
export { validateApiKey, stateForNextStep, stepIndex };

export class TosWelcome extends HTMLElement {
  private _state: WizardState = 'loading_status';
  private _status: OnboardingStatus | null = null;
  private _errorMessage = '';

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'welcome');
    this._setState('loading_status');
    this._render();
    void this._loadStatus();
  }

  // No event listeners outside this component; nothing to remove.
  disconnectedCallback(): void {}

  get currentState(): WizardState {
    return this._state;
  }
  get currentStatus(): OnboardingStatus | null {
    return this._status;
  }

  private _setState(next: WizardState): void {
    this._state = next;
    this.setAttribute('data-state', next);
    if (this._status !== null) {
      this.setAttribute('data-active-step', this._status.next_step);
    }
  }

  private async _loadStatus(): Promise<void> {
    try {
      const resp = await fetch('/api/welcome/status');
      if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
      const data = (await resp.json()) as OnboardingStatus;
      this._status = data;
      this._setState(stateForNextStep(data.next_step));
      this._render();
    } catch (err: unknown) {
      this._errorMessage = err instanceof Error ? err.message : '加载状态失败。';
      this._setState('error_status');
      this._render();
    }
  }

  private async _postStep(
    endpoint: string,
    submittingState: WizardState,
    errorState: WizardState,
    body?: unknown,
  ): Promise<void> {
    this._setState(submittingState);
    this._render();
    try {
      const resp = await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: body !== undefined ? JSON.stringify(body) : '{}',
      });
      if (!resp.ok) {
        let reason = `HTTP ${resp.status}`;
        try {
          const errBody = (await resp.json()) as { reason?: string };
          if (typeof errBody.reason === 'string') reason = errBody.reason;
        } catch {
          // ignore
        }
        throw new Error(reason);
      }
      const data = (await resp.json()) as OnboardingStatus;
      this._status = data;
      this._setState(stateForNextStep(data.next_step));
      this._render();
    } catch (err: unknown) {
      this._errorMessage = err instanceof Error ? err.message : '请求失败。';
      this._setState(errorState);
      this._render();
    }
  }

  // ---------- per-step handlers ---------------------------------------------

  private _doInit(): Promise<void> {
    return this._postStep('/api/welcome/init', 'submitting_init', 'error_init');
  }
  private _doLlmConfig(): Promise<void> {
    return this._postStep(
      '/api/welcome/llm-config',
      'submitting_llm_config',
      'error_llm_config',
    );
  }
  private _doAgentDeploy(): Promise<void> {
    return this._postStep(
      '/api/welcome/agent-deploy',
      'submitting_agent_deploy',
      'error_agent_deploy',
    );
  }
  private async _doSetApiKey(key: string): Promise<void> {
    const errMsg = validateApiKey(key);
    if (errMsg !== null) {
      this._errorMessage = errMsg;
      this._setState('error_api_key');
      this._render();
      return;
    }
    await this._postStep(
      '/api/welcome/api-key',
      'submitting_api_key',
      'error_api_key',
      { api_key: key },
    );
  }

  // ---------- rendering -----------------------------------------------------

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }

    const wrap = document.createElement('section');
    wrap.className = 'welcome-wrap';

    // Always render the progress indicator (even in loading) so the layout
    // feels stable.
    wrap.appendChild(this._renderProgress());

    if (this._state === 'loading_status') {
      wrap.appendChild(this._renderLoading('加载中'));
    } else if (this._state === 'error_status') {
      wrap.appendChild(this._renderStatusError());
    } else {
      wrap.appendChild(this._renderCard());
    }

    this.appendChild(wrap);
  }

  private _renderProgress(): HTMLElement {
    const nav = document.createElement('ol');
    nav.className = 'welcome-progress';
    nav.setAttribute('aria-label', '安装进度');

    const status = this._status;
    const activeIdx = status !== null ? stepIndex(status.next_step) : 0;

    WIZARD_STEPS.forEach((step, idx) => {
      const li = document.createElement('li');
      li.className = 'welcome-progress-step';
      // Done vs active vs pending — drives the three CSS visual states.
      let phase: 'done' | 'active' | 'pending';
      if (status !== null && status.next_step === 'Done') {
        phase = 'done';
      } else if (idx < activeIdx) {
        phase = 'done';
      } else if (idx === activeIdx) {
        phase = 'active';
      } else {
        phase = 'pending';
      }
      li.setAttribute('data-phase', phase);

      const circle = document.createElement('span');
      circle.className = 'welcome-progress-num';
      circle.textContent = String(idx + 1);
      li.appendChild(circle);

      const label = document.createElement('span');
      label.className = 'welcome-progress-label';
      label.textContent = step.label;
      li.appendChild(label);

      nav.appendChild(li);
    });

    return nav;
  }

  private _renderLoading(label: string): HTMLElement {
    const wrap = document.createElement('div');
    wrap.className = 'welcome-loading';
    const phrase = document.createElement('p');
    phrase.className = 'welcome-loading-phrase';
    phrase.textContent = label;
    const dots = document.createElement('span');
    dots.className = 'welcome-dots';
    dots.setAttribute('aria-hidden', 'true');
    for (let i = 0; i < 3; i++) {
      const dot = document.createElement('span');
      dot.textContent = '·';
      dots.appendChild(dot);
    }
    phrase.appendChild(dots);
    wrap.appendChild(phrase);
    return wrap;
  }

  private _renderStatusError(): HTMLElement {
    const wrap = document.createElement('div');
    wrap.className = 'welcome-card welcome-error-card';
    const caption = document.createElement('p');
    caption.className = 'welcome-step-caption';
    caption.textContent = '加载状态失败';
    wrap.appendChild(caption);
    const msg = document.createElement('p');
    msg.className = 'welcome-error-msg';
    msg.textContent = this._errorMessage || '无法读取 /api/welcome/status。';
    wrap.appendChild(msg);
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'welcome-cta';
    btn.textContent = '重试';
    btn.addEventListener('click', () => {
      this._errorMessage = '';
      void this._loadStatus();
    });
    wrap.appendChild(btn);
    return wrap;
  }

  /** Render the active card for whichever step is current. */
  private _renderCard(): HTMLElement {
    if (this._state === 'step_init' || this._state === 'submitting_init' || this._state === 'error_init') {
      return this._renderStepCard({
        index: 1,
        title: '第一步 · 准备工作站',
        subtitle:
          '我帮你在硬盘上铺一张空白的"账本桌面"——里面有 genesis_payload.toml 和 agent_pubkeys.json，是后面所有步骤的地基。',
        ctaLabel: '准备工作站 →',
        submitting: this._state === 'submitting_init',
        submittingLabel: '正在初始化工作站',
        showError: this._state === 'error_init',
        onClick: () => void this._doInit(),
        retryLabel: '重试 init',
      });
    }
    if (
      this._state === 'step_llm_config' ||
      this._state === 'submitting_llm_config' ||
      this._state === 'error_llm_config'
    ) {
      return this._renderStepCard({
        index: 2,
        title: '第二步 · 配置两个模型',
        subtitle:
          '我会把两个 LLM 写进 turingos.toml——一个负责"问你问题"（DeepSeek V3.2），一个负责"写代码"（Qwen3-Coder 30B）。只写模型名字，不写密钥。',
        ctaLabel: '写入 turingos.toml →',
        submitting: this._state === 'submitting_llm_config',
        submittingLabel: '正在写入模型配置',
        showError: this._state === 'error_llm_config',
        onClick: () => void this._doLlmConfig(),
        retryLabel: '重试 llm config',
      });
    }
    if (
      this._state === 'step_api_key' ||
      this._state === 'submitting_api_key' ||
      this._state === 'error_api_key'
    ) {
      return this._renderApiKeyCard();
    }
    if (
      this._state === 'step_agent_deploy' ||
      this._state === 'submitting_agent_deploy' ||
      this._state === 'error_agent_deploy'
    ) {
      return this._renderStepCard({
        index: 4,
        title: '第三步 · 给工作站注册一个 Agent',
        subtitle:
          '注册一个 Solver 角色的 agent_001，告诉系统"以后是这个 agent 在跑工作"。这是 Phase 6.1 的多 agent 体系的最小入口。',
        ctaLabel: '注册 agent_001 →',
        submitting: this._state === 'submitting_agent_deploy',
        submittingLabel: '正在注册 Agent',
        showError: this._state === 'error_agent_deploy',
        onClick: () => void this._doAgentDeploy(),
        retryLabel: '重试 agent deploy',
      });
    }
    // step_ready
    return this._renderReadyCard();
  }

  private _renderStepCard(opts: {
    index: number;
    title: string;
    subtitle: string;
    ctaLabel: string;
    submitting: boolean;
    submittingLabel: string;
    showError: boolean;
    onClick: () => void;
    retryLabel: string;
  }): HTMLElement {
    const card = document.createElement('div');
    card.className = 'welcome-card';

    const caption = document.createElement('p');
    caption.className = 'welcome-step-caption';
    caption.textContent = `STEP ${opts.index} / 5`;
    card.appendChild(caption);

    const h2 = document.createElement('h2');
    h2.className = 'welcome-step-title';
    h2.textContent = opts.title;
    card.appendChild(h2);

    const sub = document.createElement('p');
    sub.className = 'welcome-step-subtitle';
    sub.textContent = opts.subtitle;
    card.appendChild(sub);

    if (opts.submitting) {
      card.appendChild(this._renderLoading(opts.submittingLabel));
      return card;
    }

    if (opts.showError) {
      const errBlock = document.createElement('div');
      errBlock.className = 'welcome-error-block';
      const msg = document.createElement('p');
      msg.className = 'welcome-error-msg';
      msg.textContent = this._errorMessage || '出错了。';
      errBlock.appendChild(msg);

      const retry = document.createElement('button');
      retry.type = 'button';
      retry.className = 'welcome-cta';
      retry.textContent = opts.retryLabel;
      retry.addEventListener('click', () => {
        this._errorMessage = '';
        opts.onClick();
      });
      errBlock.appendChild(retry);
      card.appendChild(errBlock);
      return card;
    }

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'welcome-cta';
    btn.textContent = opts.ctaLabel;
    btn.addEventListener('click', () => opts.onClick());
    card.appendChild(btn);

    return card;
  }

  private _renderApiKeyCard(): HTMLElement {
    const card = document.createElement('div');
    card.className = 'welcome-card';

    const caption = document.createElement('p');
    caption.className = 'welcome-step-caption';
    caption.textContent = 'STEP 3 / 5';
    card.appendChild(caption);

    const h2 = document.createElement('h2');
    h2.className = 'welcome-step-title';
    h2.textContent = '把 SiliconFlow 的 API 密钥交给我';
    card.appendChild(h2);

    const sub = document.createElement('p');
    sub.className = 'welcome-step-subtitle';
    sub.textContent =
      '密钥只活在这个服务器进程的内存里——重启就丢，从不写盘、不进日志、不会回显在网页上。你只需要在每次启动 turingos_web 之后填一次。';
    card.appendChild(sub);

    if (this._state === 'submitting_api_key') {
      card.appendChild(this._renderLoading('正在保存到内存'));
      return card;
    }

    const alreadySet =
      this._status !== null && this._status.api_key_set && this._state !== 'error_api_key';

    if (alreadySet) {
      const setLine = document.createElement('p');
      setLine.className = 'welcome-api-set';
      setLine.textContent = 'API 密钥已设置（仅保存在内存中）';
      card.appendChild(setLine);

      const replace = document.createElement('button');
      replace.type = 'button';
      replace.className = 'welcome-cta-soft';
      replace.textContent = '替换密钥';
      replace.addEventListener('click', () => {
        // Manually clear the local flag so we re-render the input.
        if (this._status !== null) {
          this._status = { ...this._status, api_key_set: false };
        }
        this._setState('step_api_key');
        this._render();
      });
      card.appendChild(replace);

      // Auto-advance: if api-key is set and the next step on the backend is
      // still ApiKey (shouldn't happen) we stay; otherwise the wizard already
      // moved on via setState.
      return card;
    }

    const field = document.createElement('div');
    field.className = 'welcome-api-field';

    const label = document.createElement('label');
    label.className = 'welcome-api-label';
    label.setAttribute('for', 'welcome-api-key-input');
    label.textContent = 'SILICONFLOW_API_KEY';
    field.appendChild(label);

    const input = document.createElement('input');
    input.type = 'password';
    input.id = 'welcome-api-key-input';
    input.name = 'api_key';
    input.placeholder = 'sk-...';
    input.autocomplete = 'off';
    input.spellcheck = false;
    input.className = 'welcome-api-input';
    field.appendChild(input);

    card.appendChild(field);

    if (this._state === 'error_api_key' && this._errorMessage) {
      const err = document.createElement('p');
      err.className = 'welcome-error-msg';
      err.textContent = this._errorMessage;
      card.appendChild(err);
    }

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'welcome-cta';
    btn.textContent = '保存密钥 →';
    btn.addEventListener('click', () => {
      const value = input.value.trim();
      void this._doSetApiKey(value);
    });
    card.appendChild(btn);

    requestAnimationFrame(() => input.focus());
    return card;
  }

  private _renderReadyCard(): HTMLElement {
    const card = document.createElement('div');
    card.className = 'welcome-card welcome-ready-card';

    const caption = document.createElement('p');
    caption.className = 'welcome-step-caption';
    caption.textContent = '完成 / READY';
    card.appendChild(caption);

    const h2 = document.createElement('h2');
    h2.className = 'welcome-step-title';
    h2.textContent = '你的工作站已就绪。';
    card.appendChild(h2);

    const sub = document.createElement('p');
    sub.className = 'welcome-step-subtitle';
    sub.textContent =
      '五步全部完成。点下面开始 spec 访谈——我会问你八个关于"日常麻烦"的问题，然后帮你生成一个小工具。';
    card.appendChild(sub);

    const cta = document.createElement('button');
    cta.type = 'button';
    cta.className = 'welcome-cta';
    cta.textContent = '开始 spec 访谈 →';
    cta.addEventListener('click', () => {
      // Hard navigate to /build. `turingos-root` has no popstate listener,
      // so a soft pushState would leave the welcome view rendered while the
      // URL bar flipped. Use a single assign with no prior pushState — that
      // also avoids the Safari/Chrome same-URL-no-op trap that surfaces
      // when pushState changes location.pathname first.
      window.location.assign('/build');
    });
    card.appendChild(cta);

    return card;
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosWelcome);
  }
}
