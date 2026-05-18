// TRACE_MATRIX FC1-N5: Phase 7 W6 — spec.md editorial viewer + generate kickoff.
// <tos-spec-result> receives a SpecSubmitResponse via the `spec` property
// setter and renders spec.md via a hand-rolled markdown walker (#/##/###,
// - bullet, 1. ordered, **bold**, *em*, `code`, ``` fences). On the "生成
// 代码" CTA, POSTs /api/generate then mounts <tos-artifact-viewer>. XSS
// hygiene: every node via createElement + textContent — NEVER innerHTML.
// Sets data-block-type="spec_result" on self.

import type { GenerateResponse, SpecSubmitResponse, WsMessage } from '../ir.js';
import { truncateMiddle } from './render-helpers.js';

const ELEMENT_NAME = 'tos-spec-result';

type ResultState = 'idle' | 'generating' | 'generated' | 'error';

export class TosSpecResult extends HTMLElement {
  private _state: ResultState = 'idle';
  private _spec: SpecSubmitResponse | null = null;
  private _errorMessage = '';
  private _generated: GenerateResponse | null = null;

  private _wsListener: ((e: Event) => void) | null = null;

  /** Property setter: caller assigns the SpecSubmitResponse here. */
  set spec(value: SpecSubmitResponse) {
    this._spec = value;
    if (this.isConnected) {
      this._render();
    }
  }
  get spec(): SpecSubmitResponse | null {
    return this._spec;
  }

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'spec_result');
    if (this._spec === null) {
      const raw = this.dataset['payload'];
      if (raw != null) {
        try { this._spec = JSON.parse(raw) as SpecSubmitResponse; } catch { /* */ }
      }
    }
    this._render();
    this._wsListener = (e: Event) => this._onWsMessage(e);
    document.addEventListener('turingos:ir_update', this._wsListener);
  }

  disconnectedCallback(): void {
    if (this._wsListener !== null) {
      document.removeEventListener('turingos:ir_update', this._wsListener);
      this._wsListener = null;
    }
  }

  // WS arrives alongside POST; POST is source of truth (carries spec_md).
  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage | null>).detail;
    if (detail == null || detail.msg_type !== 'generate_complete') return;
    if (this._spec === null || detail.session_id !== this._spec.session_id) return;
  }

  private async _startGenerate(): Promise<void> {
    if (this._spec === null) return;
    this._state = 'generating';
    this.setAttribute('data-state', this._state);
    this._render();
    try {
      const resp = await fetch('/api/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ session_id: this._spec.session_id }),
      });
      if (!resp.ok) {
        let reason = `HTTP ${resp.status}`;
        try {
          const errBody = (await resp.json()) as { reason?: string };
          if (typeof errBody.reason === 'string') reason = errBody.reason;
        } catch { /* */ }
        throw new Error(reason);
      }
      this._generated = (await resp.json()) as GenerateResponse;
      this._state = 'generated';
      this.setAttribute('data-state', this._state);
      this._render();
    } catch (err: unknown) {
      this._errorMessage = err instanceof Error ? err.message : '生成失败，请稍后重试。';
      this._state = 'error';
      this.setAttribute('data-state', this._state);
      this._render();
    }
  }

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }

    if (this._state === 'generated' && this._generated !== null) {
      const viewer = document.createElement('tos-artifact-viewer') as HTMLElement & {
        artifacts?: GenerateResponse;
      };
      viewer.artifacts = this._generated;
      try { viewer.dataset['payload'] = JSON.stringify(this._generated); } catch { /* */ }
      this.appendChild(viewer);
      return;
    }

    if (this._spec === null) {
      const p = document.createElement('p');
      p.className = 'spec-result-empty';
      p.textContent = '(尚无 spec 数据)';
      this.appendChild(p);
      return;
    }

    const article = document.createElement('article');
    article.className = 'spec-result-article';
    renderMarkdownInto(article, this._spec.spec_md);
    this.appendChild(article);

    if (this._spec.capsule_cid != null && this._spec.capsule_cid.length > 0) {
      const cidFooter = document.createElement('p');
      cidFooter.className = 'spec-result-cid';
      const label = document.createElement('span');
      label.className = 'spec-result-cid-label';
      label.textContent = 'CAS capsule ';
      cidFooter.appendChild(label);
      const code = document.createElement('code');
      const cid = this._spec.capsule_cid;
      code.title = cid;
      code.textContent = 'cid:' + truncateMiddle(cid, 8, 8);
      cidFooter.appendChild(code);
      this.appendChild(cidFooter);
    }

    const cta = document.createElement('div');
    cta.className = 'spec-result-cta';
    const btn = document.createElement('button');
    btn.type = 'button';
    btn.className = 'spec-result-generate-btn';
    if (this._state === 'generating') {
      btn.textContent = '正在生成代码…';
      btn.disabled = true;
    } else if (this._state === 'error') {
      btn.textContent = '重试生成代码 →';
    } else {
      btn.textContent = '生成代码 →';
    }
    btn.addEventListener('click', () => {
      void this._startGenerate();
    });
    cta.appendChild(btn);
    if (this._state === 'error') {
      const errLine = document.createElement('p');
      errLine.className = 'spec-result-error';
      errLine.textContent = this._errorMessage;
      cta.appendChild(errLine);
    }
    this.appendChild(cta);
  }
}

/** Minimal markdown walker — line-based, conservative, no innerHTML. */
export function renderMarkdownInto(target: HTMLElement, md: string): void {
  const lines = md.replace(/\r\n/g, '\n').split('\n');
  let i = 0;
  while (i < lines.length) {
    const line = lines[i] ?? '';
    if (line.startsWith('```')) {
      const code = document.createElement('pre');
      const inner = document.createElement('code');
      const buf: string[] = [];
      i += 1;
      while (i < lines.length && !(lines[i] ?? '').startsWith('```')) {
        buf.push(lines[i] ?? '');
        i += 1;
      }
      i += 1;
      inner.textContent = buf.join('\n');
      code.appendChild(inner);
      target.appendChild(code);
      continue;
    }
    const heading = /^(#{1,6})\s+(.*)$/.exec(line);
    if (heading !== null) {
      const level = heading[1]!.length;
      const h = document.createElement(`h${Math.min(level, 6)}`);
      renderInlineInto(h, heading[2] ?? '');
      target.appendChild(h);
      i += 1;
      continue;
    }
    if (/^\s*[-*]\s+/.test(line)) {
      const ul = document.createElement('ul');
      while (i < lines.length && /^\s*[-*]\s+/.test(lines[i] ?? '')) {
        const item = (lines[i] ?? '').replace(/^\s*[-*]\s+/, '');
        const li = document.createElement('li');
        renderInlineInto(li, item);
        ul.appendChild(li);
        i += 1;
      }
      target.appendChild(ul);
      continue;
    }
    if (/^\s*\d+\.\s+/.test(line)) {
      const ol = document.createElement('ol');
      while (i < lines.length && /^\s*\d+\.\s+/.test(lines[i] ?? '')) {
        const item = (lines[i] ?? '').replace(/^\s*\d+\.\s+/, '');
        const li = document.createElement('li');
        renderInlineInto(li, item);
        ol.appendChild(li);
        i += 1;
      }
      target.appendChild(ol);
      continue;
    }
    if (line.trim() === '') {
      i += 1;
      continue;
    }
    const paraBuf: string[] = [line];
    i += 1;
    while (
      i < lines.length &&
      (lines[i] ?? '').trim() !== '' &&
      !/^(#{1,6})\s+/.test(lines[i] ?? '') &&
      !(lines[i] ?? '').startsWith('```') &&
      !/^\s*[-*]\s+/.test(lines[i] ?? '') &&
      !/^\s*\d+\.\s+/.test(lines[i] ?? '')
    ) {
      paraBuf.push(lines[i] ?? '');
      i += 1;
    }
    const p = document.createElement('p');
    renderInlineInto(p, paraBuf.join(' '));
    target.appendChild(p);
  }
}

/** Inline **bold**, *em*, `code`. Unknown text goes through as a text node. */
function renderInlineInto(parent: HTMLElement, text: string): void {
  let remaining = text;
  while (remaining.length > 0) {
    const codeIdx = remaining.indexOf('`');
    const boldIdx = remaining.indexOf('**');
    let emIdx = -1;
    for (let k = 0; k < remaining.length; k++) {
      if (remaining[k] === '*' && remaining[k + 1] !== '*' && remaining[k - 1] !== '*') {
        emIdx = k;
        break;
      }
    }
    const candidates: Array<[number, 'code' | 'bold' | 'em']> = [];
    if (codeIdx >= 0) candidates.push([codeIdx, 'code']);
    if (boldIdx >= 0) candidates.push([boldIdx, 'bold']);
    if (emIdx >= 0) candidates.push([emIdx, 'em']);
    if (candidates.length === 0) {
      parent.appendChild(document.createTextNode(remaining));
      return;
    }
    candidates.sort((a, b) => a[0] - b[0]);
    const [pos, kind] = candidates[0]!;
    if (pos > 0) {
      parent.appendChild(document.createTextNode(remaining.slice(0, pos)));
    }
    if (kind === 'code') {
      const end = remaining.indexOf('`', pos + 1);
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const span = document.createElement('code');
      span.textContent = remaining.slice(pos + 1, end);
      parent.appendChild(span);
      remaining = remaining.slice(end + 1);
    } else if (kind === 'bold') {
      const end = remaining.indexOf('**', pos + 2);
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const strong = document.createElement('strong');
      strong.textContent = remaining.slice(pos + 2, end);
      parent.appendChild(strong);
      remaining = remaining.slice(end + 2);
    } else {
      // em
      let end = -1;
      for (let k = pos + 1; k < remaining.length; k++) {
        if (remaining[k] === '*' && remaining[k + 1] !== '*' && remaining[k - 1] !== '*') {
          end = k;
          break;
        }
      }
      if (end < 0) {
        parent.appendChild(document.createTextNode(remaining.slice(pos)));
        return;
      }
      const em = document.createElement('em');
      em.textContent = remaining.slice(pos + 1, end);
      parent.appendChild(em);
      remaining = remaining.slice(end + 1);
    }
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosSpecResult);
  }
}
