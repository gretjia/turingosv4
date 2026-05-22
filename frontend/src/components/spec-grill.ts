// TRACE_MATRIX FC1-N5 + FC1-N10: Phase 5 driven-default spec interview centerpiece.
// <tos-spec-grill> drives the user through an LLM-controlled grill turn-by-turn:
// the Meta AI picks each next question based on prior answers + canonical slot
// coverage (asset grill_meta_v1.md). No fixed question count — the LLM decides
// when the predicates are satisfied. State machine:
//   idle | awaiting_first_turn | awaiting_user_answer | playback_review | complete
// XSS hygiene: textContent/createElement only. Sets data-block-type="spec_grill".
//
// Phase 5 (2026-05-22) removed the static 8-question batch path entirely
// (formerly `?mode=driven` opt-in; `/api/spec/questions` + `/api/spec/submit`).
// True Software 3.0 — prompt-as-program (embedded grill_meta_v1.md is the
// program), every question is an LLM runtime decision.

import type {
  WsMessage,
  SpecTurnAdvancedEvent,
  SpecGrillCompleteEvent,
  SpecTurnTriageRejectEvent,
} from '../ir.js';
import type { TurnRequest, TurnResponse, GrillState as DrivenGrillState } from '../types/spec.js';

const ELEMENT_NAME = 'tos-spec-grill';

/** Mirror of the backend `spec_turn_handler` validate_user_answer rules. */
const ANSWER_MAX_CHARS = 4096;

export class TosSpecGrill extends HTMLElement {
  private _drivenState: DrivenGrillState = { kind: 'idle' };
  private _drivenSessionId = '';
  /** Counts of consecutive 5xx responses; surfaced in nudge for user retry. */
  private _recent5xxCount = 0;
  /** Nudge text shown when triage rejects an answer (clears on next submit). */
  private _drivenNudge = '';

  private _wsListener: ((e: Event) => void) | null = null;
  /** Bound keydown handler — Cmd/Ctrl+Enter submits the answer. */
  private _keyHandler: ((e: KeyboardEvent) => void) | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'spec_grill');
    this._drivenState = { kind: 'idle' };
    this.setAttribute('data-state', 'idle');
    this._renderDriven();

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

  get currentState(): DrivenGrillState['kind'] {
    return this._drivenState.kind;
  }

  // WS arrival corroborates the POST response for /api/spec/turn driven turns.
  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage | null>).detail;
    if (detail == null) return;

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
    }
  }

  private _onKeydown(e: KeyboardEvent): void {
    if (this._drivenState.kind === 'awaiting_user_answer') {
      if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') {
        e.preventDefault();
        this._drivenSubmitAnswer();
      }
    }
  }

  // ── Toast helper ──────────────────────────────────────────────────────────

  private _showDrivenToast(message: string): void {
    const toast = document.createElement('div');
    toast.className = 'spec-grill-toast';
    toast.setAttribute('role', 'status');
    toast.setAttribute('aria-live', 'polite');
    toast.textContent = message;
    this.insertBefore(toast, this.firstChild);
    setTimeout(() => {
      if (toast.parentNode === this) {
        this.removeChild(toast);
      }
    }, 4000);
  }

  // ── Driven loop ───────────────────────────────────────────────────────────

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
      // Network-level error: surface as nudge; user can retry.
      this._recent5xxCount++;
      this._drivenNudge = '网络错误，请稍后重试。';
      this._renderDriven();
      return;
    }

    if (!resp.ok) {
      if (resp.status === 404) {
        // Session not found (server restart). Re-enter idle to restart.
        this._showDrivenToast('会话已失效，请重新开始访谈');
        this._drivenState = { kind: 'idle' };
        this._drivenSessionId = '';
        this._drivenNudge = '';
        this._recent5xxCount = 0;
        this._renderDriven();
        return;
      }
      if (resp.status >= 500) {
        this._recent5xxCount++;
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
      // Bounce-back: triage rejected the last answer. Set nudge from HTTP
      // response so the user knows why Q is repeated — without relying on
      // the WS SpecTurnTriageReject event (which may be absent or delayed).
      if (data.triage_class) {
        this._drivenNudge =
          data.triage_class === 'off_topic'
            ? '能换一种说法吗？刚才听不太懂'
            : '您似乎在测试我，可以继续吗？';
      } else {
        // Normal advance: clear any stale nudge.
        this._drivenNudge = '';
      }
      this._renderDriven();
      return;
    }

    // Unexpected shape — treat as non-fatal; stay in current state with nudge.
    this._drivenNudge = '响应格式异常，请稍后重试。';
    this._renderDriven();
  }

  /** Handler when user clicks CTA in driven-mode idle state. */
  private _drivenStart(): void {
    this._drivenSessionId = crypto.randomUUID();
    this._drivenState = { kind: 'awaiting_first_turn' };
    this._drivenNudge = '';
    this.setAttribute('data-state', 'awaiting_first_turn');
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
    // Optimistically enter loading state (preserve question text for nudge UX).
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

  // ── Driven render ─────────────────────────────────────────────────────────

  private _renderDriven(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    this.setAttribute('data-state', this._drivenState.kind);
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
      '不用想程序怎么做。我会沿着你的回答继续问下去，直到对你想做的工具有足够了解。一两分钟，spec.md 就会自动写出来。';
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

    // Mount spec-result to expose the generate CTA and artifact viewer.
    // tos-spec-result reads session_id → POST /api/generate → tos-artifact-viewer.
    const specResult = document.createElement('tos-spec-result') as HTMLElement & { spec: unknown };
    (specResult as any).spec = {
      session_id: this._drivenSessionId,
      spec_md: '',
      capsule_cid: capsuleCid,
    };
    (this.parentElement ?? document.querySelector('main') ?? document.body)
      .appendChild(specResult);
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
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosSpecGrill);
  }
}
