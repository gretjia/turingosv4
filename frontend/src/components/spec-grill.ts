// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 7 W6 — spec interview centerpiece.
// <tos-spec-grill> walks a non-developer user through 8 customer-development
// questions one at a time, posts to /api/spec/submit, then hands off to
// <tos-spec-result>. State machine: idle | loading_questions | interviewing
// | submitting | spec_ready | error. XSS hygiene: textContent/createElement
// only. Sets data-block-type="spec_grill" on self.
//
// W8 (Phase 6.3.x): URL ?mode=driven enables driven mode — per-turn POSTs to
// /api/spec/turn with LLM-driven question flow. Falls back to static-mode on
// 2× 5xx, 404 (session lost), or absent URL param. State machine documented
// in handover/directives/2026-05-18_TISR_PHASE6_3_X_GRILL_LLM_DRIVEN_SECTION8_PACKET_R2.md.

import type {
  SpecQuestionsResponse,
  SpecSubmitResponse,
  WsMessage,
  SpecTurnAdvancedEvent,
  SpecGrillCompleteEvent,
  SpecTurnTriageRejectEvent,
} from '../ir.js';
import type { TurnRequest, TurnResponse, GrillState as DrivenGrillState } from '../types/spec.js';

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
  // ── Static-mode state ───────────────────────────────────────────────────
  private _state: GrillState = 'idle';
  private _questions: string[] = [];
  private _answers: string[] = [];
  private _currentIndex = 0;
  private _errorMessage = '';
  private _specResponse: SpecSubmitResponse | null = null;

  private _wsListener: ((e: Event) => void) | null = null;
  /** Bound keydown handler — Cmd/Ctrl+Enter advances. */
  private _keyHandler: ((e: KeyboardEvent) => void) | null = null;

  // ── Driven-mode state (W8 Phase 6.3.x) ──────────────────────────────────
  /** True when URL contains ?mode=driven. Detected once in connectedCallback. */
  driven_mode = false;
  private _drivenState: DrivenGrillState = { kind: 'idle' };
  private _drivenSessionId = '';
  /** Counts of consecutive 5xx responses; 2 triggers static fallback. */
  private _recent5xxCount = 0;
  /** Nudge text shown when triage rejects an answer (clears on next submit). */
  private _drivenNudge = '';

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'spec_grill');

    // W8: detect driven mode from URL params once at mount time.
    const params = new URLSearchParams(window.location.search);
    this.driven_mode = params.get('mode') === 'driven';

    if (this.driven_mode) {
      this._drivenState = { kind: 'idle' };
      this._renderDriven();
    } else {
      this._setState('idle');
      this._render();
    }

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
    if (detail == null) return;

    // W8 driven-mode WS events.
    if (this.driven_mode) {
      if (detail.msg_type === 'SpecTurnAdvanced') {
        const ev = detail as SpecTurnAdvancedEvent;
        if (ev.session_id !== this._drivenSessionId) return;
        // Optimistic update: if we are still awaiting_user_answer, update the
        // question text (usually redundant with POST response).
        if (
          this._drivenState.kind === 'awaiting_user_answer' &&
          ev.turn_index === this._drivenState.turn_index
        ) {
          this._drivenState = {
            kind: 'awaiting_user_answer',
            turn_index: ev.turn_index,
            question: ev.question_text,
          };
          this._renderDriven();
        }
        return;
      }
      if (detail.msg_type === 'SpecGrillComplete') {
        const ev = detail as SpecGrillCompleteEvent;
        if (ev.session_id !== this._drivenSessionId) return;
        this._drivenState = { kind: 'complete', spec_capsule_cid: ev.spec_capsule_cid };
        this._renderDriven();
        return;
      }
      if (detail.msg_type === 'SpecTurnTriageReject') {
        const ev = detail as SpecTurnTriageRejectEvent;
        if (ev.session_id !== this._drivenSessionId) return;
        if (ev.triage_class === 'off_topic') {
          this._drivenNudge = '能换一种说法吗？刚才听不太懂';
        } else {
          // abusive | gibberish
          this._drivenNudge = '您似乎在测试我，可以继续吗？';
        }
        this._renderDriven();
        return;
      }
    }

    // Static-mode: WS arrival corroborates POST; POST stays the source of truth.
    if (detail.msg_type !== 'spec_complete') return;
    if (this._specResponse != null && this._specResponse.session_id === detail.session_id) return;
  }

  private _onKeydown(e: KeyboardEvent): void {
    // Driven mode: Cmd/Ctrl+Enter submits the answer textarea.
    if (this.driven_mode) {
      if (this._drivenState.kind === 'awaiting_user_answer') {
        if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
          e.preventDefault();
          this._drivenSubmitAnswer();
        }
      }
      return;
    }
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

  // ── Driven-mode helpers (W8 Phase 6.3.x) ──────────────────────────────────

  /** Fall back to static-mode and display one-time toast. */
  private _fallbackToStatic(): void {
    this.driven_mode = false;
    this._recent5xxCount = 0;
    this._drivenNudge = '';
    // Show one-time toast.
    this._showDrivenToast('切换至 8 问经典模式');
    // Reset and render static idle state.
    this._setState('idle');
    this._render();
  }

  private _showDrivenToast(message: string): void {
    const toast = document.createElement('div');
    toast.className = 'spec-grill-toast';
    toast.setAttribute('role', 'status');
    toast.setAttribute('aria-live', 'polite');
    toast.textContent = message;
    // Insert toast at top of element briefly, then remove.
    this.insertBefore(toast, this.firstChild);
    setTimeout(() => {
      if (toast.parentNode === this) {
        this.removeChild(toast);
      }
    }, 4000);
  }

  /** POST /api/spec/turn and handle the response state transition. */
  private async _postTurn(userAnswer: string | null): Promise<void> {
    const body: TurnRequest = {
      session_id: this._drivenSessionId,
      user_answer: userAnswer,
      lang: 'zh',
    };
    let resp: Response;
    try {
      resp = await fetch('/api/spec/turn', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
    } catch {
      // Network-level error counts as 5xx for fallback purposes.
      this._recent5xxCount++;
      if (this._recent5xxCount >= 2) {
        this._fallbackToStatic();
      }
      return;
    }

    if (!resp.ok) {
      if (resp.status === 404) {
        // Session not found (server restart per R2 §A14) — immediate fallback.
        this._fallbackToStatic();
        return;
      }
      if (resp.status >= 500) {
        this._recent5xxCount++;
        if (this._recent5xxCount >= 2) {
          this._fallbackToStatic();
          return;
        }
        // One 5xx: show error in nudge area and let user retry.
        this._drivenNudge = `服务器错误 (${resp.status})，请稍后重试。`;
        this._renderDriven();
        return;
      }
      // Other 4xx — surface as nudge.
      this._drivenNudge = `请求错误 (${resp.status})。`;
      this._renderDriven();
      return;
    }

    // Success: reset 5xx counter.
    this._recent5xxCount = 0;
    const data = (await resp.json()) as TurnResponse;

    if (data.terminated && data.spec_capsule_cid !== null) {
      this._drivenState = { kind: 'complete', spec_capsule_cid: data.spec_capsule_cid };
      this._drivenNudge = '';
      this._renderDriven();
      return;
    }

    if (data.done && data.playback !== null) {
      this._drivenState = {
        kind: 'playback_review',
        playback: data.playback,
        session_id: this._drivenSessionId,
      };
      this._drivenNudge = '';
      this._renderDriven();
      return;
    }

    if (data.question_text !== null) {
      this._drivenState = {
        kind: 'awaiting_user_answer',
        turn_index: data.turn_index,
        question: data.question_text,
      };
      this._drivenNudge = '';
      this._renderDriven();
      return;
    }

    // Unexpected shape — treat as non-fatal; stay in current state.
    this._drivenNudge = '响应格式异常，请稍后重试。';
    this._renderDriven();
  }

  /** Handler when user clicks CTA in driven-mode idle state. */
  private _drivenStart(): void {
    this._drivenSessionId = crypto.randomUUID();
    this._drivenState = { kind: 'awaiting_first_turn' };
    this._drivenNudge = '';
    this._renderDriven();
    void this._postTurn(null);
  }

  /** Handler when user submits their answer in driven mode. */
  private _drivenSubmitAnswer(): void {
    if (this._drivenState.kind !== 'awaiting_user_answer') return;
    const ta = this.querySelector(
      'textarea[name="driven-answer"]',
    ) as HTMLTextAreaElement | null;
    if (ta === null) return;
    const answer = ta.value.trim();
    if (answer.length === 0) {
      this._drivenNudge = '请写一点内容再继续。';
      this._renderDriven();
      return;
    }
    if (answer.length > ANSWER_MAX_CHARS) {
      this._drivenNudge = `回答太长了：${answer.length} 字符，最多 ${ANSWER_MAX_CHARS}。`;
      this._renderDriven();
      return;
    }
    // Optimistically enter loading state.
    const turnIdx = this._drivenState.turn_index;
    this._drivenState = {
      kind: 'awaiting_user_answer',
      turn_index: turnIdx,
      question: this._drivenState.question,
    };
    this._drivenNudge = '';
    void this._postTurn(answer);
  }

  /** Handler for playback-review confirm ("没问题"). */
  private _drivenConfirmPlayback(): void {
    if (this._drivenState.kind !== 'playback_review') return;
    this._drivenNudge = '';
    this._drivenState = {
      kind: 'playback_review',
      playback: this._drivenState.playback,
      session_id: this._drivenState.session_id,
    };
    this._renderDriven();
    void this._postTurn('确认');
  }

  /** Handler for playback-review edit request. */
  private _drivenEditPlayback(prevQuestion: string): void {
    this._drivenNudge = '';
    // Revert to awaiting_user_answer with a synthetic "last question" placeholder.
    this._drivenState = {
      kind: 'awaiting_user_answer',
      turn_index: 0, // Will be updated on next POST response.
      question: prevQuestion,
    };
    this._renderDriven();
  }

  // ── Driven-mode render methods ─────────────────────────────────────────────

  private _renderDriven(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    switch (this._drivenState.kind) {
      case 'idle':
        this._renderDrivenIdle();
        break;
      case 'awaiting_first_turn':
        this._renderLoading('正在启动 spec 访谈');
        break;
      case 'awaiting_user_answer':
        this._renderDrivenQuestion(
          this._drivenState.turn_index,
          this._drivenState.question,
        );
        break;
      case 'playback_review':
        this._renderDrivenPlayback(this._drivenState.playback);
        break;
      case 'complete':
        this._renderDrivenComplete(this._drivenState.spec_capsule_cid);
        break;
    }
  }

  private _renderDrivenIdle(): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-idle';

    const eyebrow = document.createElement('p');
    eyebrow.className = 'spec-grill-eyebrow';
    eyebrow.textContent = 'TISR · LLM 驱动访谈';
    wrap.appendChild(eyebrow);

    const lede = document.createElement('p');
    lede.className = 'spec-grill-lede';
    lede.textContent =
      '我会问一系列关于"日常麻烦"的问题，直到我对你的需求有足够的了解。回答后，spec 会自动生成。';
    wrap.appendChild(lede);

    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'spec-grill-cta';
    btn.setAttribute('data-cta', 'start');
    btn.textContent = '开始 spec 访谈 →';
    btn.addEventListener('click', () => this._drivenStart());
    wrap.appendChild(btn);

    this.appendChild(wrap);
  }

  private _renderDrivenQuestion(turnIndex: number, question: string): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-question';

    const progress = document.createElement('div');
    progress.className = 'spec-grill-progress';
    progress.textContent = `问题 ${turnIndex}`;
    wrap.appendChild(progress);

    const q = document.createElement('p');
    q.className = 'spec-grill-question-text';
    q.textContent = question;
    wrap.appendChild(q);

    // Nudge display (triage reject or transient error).
    if (this._drivenNudge.length > 0) {
      const nudge = document.createElement('p');
      nudge.className = 'spec-grill-nudge';
      nudge.setAttribute('role', 'alert');
      nudge.textContent = this._drivenNudge;
      wrap.appendChild(nudge);
    }

    const ta = document.createElement('textarea');
    ta.name = 'driven-answer';
    ta.className = 'spec-grill-input';
    ta.rows = 6;
    ta.placeholder = '在这里写下你的回答…   (⌘/Ctrl+Enter 提交)';
    ta.autocapitalize = 'sentences';
    ta.spellcheck = false;
    requestAnimationFrame(() => ta.focus());
    wrap.appendChild(ta);

    const footer = document.createElement('footer');
    footer.className = 'spec-grill-footer';

    const submit = document.createElement('button');
    submit.type = 'button';
    submit.className = 'spec-grill-advance';
    submit.textContent = '提交回答 →';
    submit.addEventListener('click', () => this._drivenSubmitAnswer());
    footer.appendChild(submit);

    wrap.appendChild(footer);
    this.appendChild(wrap);
  }

  private _renderDrivenPlayback(playback: string): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-playback';

    const heading = document.createElement('p');
    heading.className = 'spec-grill-eyebrow';
    heading.textContent = '访谈回顾';
    wrap.appendChild(heading);

    const pre = document.createElement('pre');
    pre.className = 'spec-grill-playback-text';
    pre.textContent = playback;
    wrap.appendChild(pre);

    const footer = document.createElement('footer');
    footer.className = 'spec-grill-footer';

    const confirmBtn = document.createElement('button');
    confirmBtn.type = 'button';
    confirmBtn.className = 'spec-grill-advance';
    confirmBtn.setAttribute('data-action', 'confirm-playback');
    confirmBtn.textContent = '没问题，生成 spec →';
    confirmBtn.addEventListener('click', () => this._drivenConfirmPlayback());
    footer.appendChild(confirmBtn);

    const editBtn = document.createElement('button');
    editBtn.type = 'button';
    editBtn.className = 'spec-grill-back';
    editBtn.setAttribute('data-action', 'edit-playback');
    editBtn.textContent = '← 修改回答';
    editBtn.addEventListener('click', () => this._drivenEditPlayback(playback));
    footer.appendChild(editBtn);

    wrap.appendChild(footer);
    this.appendChild(wrap);
  }

  private _renderDrivenComplete(capsuleCid: string): void {
    const wrap = document.createElement('section');
    wrap.className = 'spec-grill-complete';

    const msg = document.createElement('p');
    msg.className = 'spec-grill-complete-msg';
    msg.textContent = 'Spec generated. Capsule CID: ';

    const cid = document.createElement('code');
    cid.className = 'spec-grill-cid';
    cid.textContent = capsuleCid;
    msg.appendChild(cid);

    wrap.appendChild(msg);
    this.appendChild(wrap);
  }

  // ── Static-mode render ─────────────────────────────────────────────────────

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
