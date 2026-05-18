// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W6 — spec interview centerpiece.
// <tos-spec-grill> walks a non-developer user through 8 customer-development
// questions one at a time, posts to /api/spec/submit, then hands off to
// <tos-spec-result>. State machine: idle | loading_questions | interviewing
// | submitting | spec_ready | error. XSS hygiene: textContent/createElement
// only. Sets data-block-type="spec_grill" on self.

import type { SpecQuestionsResponse, SpecSubmitResponse, WsMessage } from '../ir.js';

const ELEMENT_NAME = 'tos-spec-grill';

type GrillState =
  | 'idle'
  | 'loading_questions'
  | 'interviewing'
  | 'submitting'
  | 'spec_ready'
  | 'error';

/** Mirror of the backend `validate_answers` rules (src/web/spec.rs). */
const ANSWER_MAX_CHARS = 4096;

/** Number of canonical interview questions (must stay in sync with backend). */
const QUESTION_COUNT = 8;

export class TosSpecGrill extends HTMLElement {
  private _state: GrillState = 'idle';
  private _questions: string[] = [];
  private _answers: string[] = [];
  private _currentIndex = 0;
  private _errorMessage = '';
  private _specResponse: SpecSubmitResponse | null = null;

  private _wsListener: ((e: Event) => void) | null = null;
  /** Bound keydown handler — Cmd/Ctrl+Enter advances. */
  private _keyHandler: ((e: KeyboardEvent) => void) | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'spec_grill');
    this._setState('idle');
    this._render();

    this._wsListener = (e: Event) => this._onWsMessage(e);
    document.addEventListener('turingos:ir_update', this._wsListener);

    this._keyHandler = (e: KeyboardEvent) => this._onKeydown(e);
    this.addEventListener('keydown', this._keyHandler);
  }

  disconnectedCallback(): void {
    if (this._wsListener !== null) {
      document.removeEventListener('turingos:ir_update', this._wsListener);
      this._wsListener = null;
    }
    if (this._keyHandler !== null) {
      this.removeEventListener('keydown', this._keyHandler);
      this._keyHandler = null;
    }
  }

  get currentState(): GrillState {
    return this._state;
  }
  get answers(): readonly string[] {
    return this._answers;
  }
  get currentIndex(): number {
    return this._currentIndex;
  }

  private _setState(next: GrillState): void {
    this._state = next;
    this.setAttribute('data-state', next);
  }

  /** null on pass, else a Chinese error message. */
  validateAnswer(answer: string): string | null {
    if (answer.length === 0) {
      return '请写一点内容再继续。';
    }
    if (answer.length > ANSWER_MAX_CHARS) {
      return `回答太长了：${answer.length} 字符，最多 ${ANSWER_MAX_CHARS}。`;
    }
    return null;
  }

  advanceWithAnswer(answer: string): boolean {
    const trimmed = answer.trim();
    if (this.validateAnswer(trimmed) !== null) return false;
    this._answers[this._currentIndex] = trimmed;
    if (this._currentIndex < this._questions.length - 1) {
      this._currentIndex += 1;
      return true;
    }
    this._currentIndex = this._questions.length;
    return true;
  }


  private async _loadQuestions(): Promise<void> {
    this._setState('loading_questions');
    this._render();
    try {
      const resp = await fetch('/api/spec/questions');
      if (!resp.ok) {
        throw new Error(`HTTP ${resp.status}`);
      }
      const data = (await resp.json()) as SpecQuestionsResponse;
      if (!Array.isArray(data.questions) || data.questions.length !== QUESTION_COUNT) {
        throw new Error(`expected ${QUESTION_COUNT} questions, got ${data.questions?.length}`);
      }
      this._questions = data.questions.slice();
      this._answers = new Array<string>(QUESTION_COUNT).fill('');
      this._currentIndex = 0;
      this._setState('interviewing');
      this._render();
    } catch (err: unknown) {
      this._errorMessage =
        err instanceof Error ? err.message : '加载问题失败，请稍后重试。';
      this._setState('error');
      this._render();
    }
  }

  private async _submit(): Promise<void> {
    this._setState('submitting');
    this._render();
    try {
      const resp = await fetch('/api/spec/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ answers: this._answers }),
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
      const data = (await resp.json()) as SpecSubmitResponse;
      this._specResponse = data;
      this._setState('spec_ready');
      this._render();
    } catch (err: unknown) {
      this._errorMessage =
        err instanceof Error ? err.message : '合成 spec 失败，请稍后重试。';
      this._setState('error');
      this._render();
    }
  }


  // WS arrival corroborates POST; POST stays the source of truth for spec_md.
  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage | null>).detail;
    if (detail == null || detail.msg_type !== 'spec_complete') return;
    if (this._specResponse != null && this._specResponse.session_id === detail.session_id) return;
  }

  private _onKeydown(e: KeyboardEvent): void {
    if (this._state !== 'interviewing') return;
    if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
      e.preventDefault();
      this._submitCurrent();
    }
  }

  private _submitCurrent(): void {
    const ta = this.querySelector('textarea[name="spec-answer"]') as HTMLTextAreaElement | null;
    if (ta === null) return;
    const value = ta.value;
    const errMsg = this.validateAnswer(value.trim());
    if (errMsg !== null) {
      this._showInlineError(errMsg);
      return;
    }
    const wasLast = this._currentIndex === this._questions.length - 1;
    this.advanceWithAnswer(value);
    if (wasLast) {
      void this._submit();
    } else {
      this._render();
    }
  }

  private _showInlineError(message: string): void {
    const err = this.querySelector('small[data-error]') as HTMLElement | null;
    if (err === null) return;
    err.textContent = message;
    err.style.display = '';
  }


  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    switch (this._state) {
      case 'idle':
        this._renderIdle();
        break;
      case 'loading_questions':
        this._renderLoading('正在加载问题');
        break;
      case 'interviewing':
        this._renderInterviewing();
        break;
      case 'submitting':
        this._renderLoading('正在合成 spec');
        break;
      case 'spec_ready':
        this._renderSpecReady();
        break;
      case 'error':
        this._renderError();
        break;
    }
  }

  private _renderIdle(): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-idle';

    const eyebrow = document.createElement('p');
    eyebrow.className = 'spec-grill-eyebrow';
    eyebrow.textContent = 'TISR · 八问访谈';
    wrap.appendChild(eyebrow);

    const lede = document.createElement('p');
    lede.className = 'spec-grill-lede';
    lede.textContent =
      '不用想程序怎么做。我会问八个关于"日常麻烦"的问题，你像聊天那样回答就好。问完之后，spec.md 会自动写出来——那是你工具的设计草稿。再下一步，网页就会被生成。';
    wrap.appendChild(lede);

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'spec-grill-cta';
    btn.textContent = '开始访谈 →';
    btn.addEventListener('click', () => {
      void this._loadQuestions();
    });
    wrap.appendChild(btn);

    this.appendChild(wrap);
  }

  private _renderLoading(label: string): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-loading';
    const phrase = document.createElement('p');
    phrase.className = 'spec-grill-loading-phrase';
    phrase.appendChild(document.createTextNode(label));
    const dots = document.createElement('span');
    dots.className = 'spec-grill-dots';
    dots.setAttribute('aria-hidden', 'true');
    for (let i = 0; i < 3; i++) {
      const dot = document.createElement('span');
      dot.textContent = '·';
      dots.appendChild(dot);
    }
    phrase.appendChild(dots);
    wrap.appendChild(phrase);
    this.appendChild(wrap);
  }

  private _renderInterviewing(): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-question';

    const progress = document.createElement('div');
    progress.className = 'spec-grill-progress';
    progress.textContent = `Q ${this._currentIndex + 1} / ${this._questions.length}`;
    wrap.appendChild(progress);

    const q = document.createElement('p');
    q.className = 'spec-grill-question-text';
    q.textContent = this._questions[this._currentIndex] ?? '';
    wrap.appendChild(q);

    const ta = document.createElement('textarea');
    ta.name = 'spec-answer';
    ta.className = 'spec-grill-input';
    ta.rows = 6;
    ta.value = this._answers[this._currentIndex] ?? '';
    ta.placeholder = '在这里写下你的回答…   (⌘/Ctrl+Enter 进入下一题)';
    ta.autocapitalize = 'sentences';
    ta.spellcheck = false;
    requestAnimationFrame(() => ta.focus());
    wrap.appendChild(ta);

    const err = document.createElement('small');
    err.setAttribute('data-error', '');
    err.className = 'spec-grill-error';
    err.style.display = 'none';
    wrap.appendChild(err);

    const footer = document.createElement('footer');
    footer.className = 'spec-grill-footer';

    if (this._currentIndex > 0) {
      const back = document.createElement('button');
      back.type = 'button';
      back.className = 'spec-grill-back';
      back.textContent = '← 上一题';
      back.addEventListener('click', () => {
        this._answers[this._currentIndex] = ta.value;
        this._currentIndex -= 1;
        this._render();
      });
      footer.appendChild(back);
    }

    const advance = document.createElement('button');
    advance.type = 'button';
    advance.className = 'spec-grill-advance';
    const isLast = this._currentIndex === this._questions.length - 1;
    advance.textContent = isLast ? '完成访谈 →' : '下一题 →';
    advance.addEventListener('click', () => this._submitCurrent());
    footer.appendChild(advance);

    wrap.appendChild(footer);

    this.appendChild(wrap);
  }

  private _renderSpecReady(): void {
    const result = document.createElement('tos-spec-result') as HTMLElement & {
      spec?: SpecSubmitResponse;
    };
    if (this._specResponse !== null) {
      result.spec = this._specResponse;
      try { result.dataset['payload'] = JSON.stringify(this._specResponse); } catch { /* */ }
    }
    this.appendChild(result);
  }

  private _renderError(): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-errstate';

    const phrase = document.createElement('p');
    phrase.className = 'spec-grill-errmsg';
    phrase.textContent = this._errorMessage || '出错了。';
    wrap.appendChild(phrase);

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'spec-grill-cta';
    btn.textContent = '重试';
    btn.addEventListener('click', () => {
      this._errorMessage = '';
      if (this._questions.length === 0) {
        void this._loadQuestions();
      } else {
        // We had questions and were submitting — go back to the last question.
        this._currentIndex = Math.max(0, this._questions.length - 1);
        this._setState('interviewing');
        this._render();
      }
    });
    wrap.appendChild(btn);

    this.appendChild(wrap);
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosSpecGrill);
  }
}
