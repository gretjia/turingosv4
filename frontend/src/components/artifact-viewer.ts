// TRACE_MATRIX FC1-N5: Phase 7 W6 — generated-artifact preview.
// <tos-artifact-viewer> receives a GenerateResponse via the `artifacts`
// property setter and previews an LLM-generated index.html in a
// cross-origin sandboxed iframe (sandbox="allow-scripts", NEVER together
// with allow-same-origin — that combo is a documented XSS bypass).
// Multi-file: monospace file list switches the previewed artifact.
// Sets data-block-type="artifact_viewer" on self.

import type { ArtifactEntry, GenerateResponse } from '../ir.js';

const ELEMENT_NAME = 'tos-artifact-viewer';

// SANDBOX policy: allow-scripts alone. Never with allow-same-origin (XSS bypass).
const SANDBOX_ALLOWED_TOKENS = ['allow-scripts'];

export function buildSandboxAttribute(): string {
  return SANDBOX_ALLOWED_TOKENS.join(' ');
}

export function isSafeSandboxValue(value: string): boolean {
  const tokens = value.split(/\s+/).filter((t) => t.length > 0);
  return !(tokens.includes('allow-scripts') && tokens.includes('allow-same-origin'));
}

function formatBytes(n: number): string {
  if (n < 1024) return `${n} B`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
  return `${(n / (1024 * 1024)).toFixed(1)} MB`;
}

export class TosArtifactViewer extends HTMLElement {
  private _data: GenerateResponse | null = null;
  private _selectedIdx = 0;

  set artifacts(value: GenerateResponse) {
    this._data = value;
    const idx = value.artifacts.findIndex((a) => a.path.toLowerCase().endsWith('.html'));
    this._selectedIdx = idx >= 0 ? idx : 0;
    if (this.isConnected) this._render();
  }
  get artifacts(): GenerateResponse | null { return this._data; }

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'artifact_viewer');
    if (this._data === null) {
      const raw = this.dataset['payload'];
      if (raw != null) {
        try {
          this._data = JSON.parse(raw) as GenerateResponse;
          const idx = this._data.artifacts.findIndex((a) => a.path.toLowerCase().endsWith('.html'));
          this._selectedIdx = idx >= 0 ? idx : 0;
        } catch { /* */ }
      }
    }
    this._render();
  }

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (this._data === null || this._data.artifacts.length === 0) {
      const p = document.createElement('p');
      p.className = 'artifact-viewer-empty';
      p.textContent = '(尚未生成任何文件)';
      this.appendChild(p);
      return;
    }

    const header = document.createElement('header');
    header.className = 'artifact-viewer-header';
    const eyebrow = document.createElement('p');
    eyebrow.className = 'artifact-viewer-eyebrow';
    eyebrow.textContent = '生成产物 · LIVE PREVIEW';
    header.appendChild(eyebrow);
    const title = document.createElement('h2');
    title.className = 'artifact-viewer-title';
    title.textContent = '你的工具，已经写好了。';
    header.appendChild(title);
    // W8: if generation needed more than one attempt, show a monospace
    // caption above the iframe so the user knows retries happened and
    // the artifact passed heuristic verification.
    const attempts = typeof this._data.total_attempts === 'number' ? this._data.total_attempts : 1;
    if (attempts > 1) {
      const retryCaption = document.createElement('p');
      retryCaption.className = 'artifact-viewer-retry-caption';
      retryCaption.textContent = `经过 ${attempts} 次尝试 · 已通过启发式验证`;
      header.appendChild(retryCaption);
    }
    this.appendChild(header);

    const layout = document.createElement('div');
    layout.className = 'artifact-viewer-layout';

    if (this._data.artifacts.length > 1) {
      const list = document.createElement('ul');
      list.className = 'artifact-viewer-filelist';
      this._data.artifacts.forEach((a, idx) => {
        const li = document.createElement('li');
        li.className = idx === this._selectedIdx ? 'is-selected' : '';
        const btn = document.createElement('button');
        btn.type = 'button';
        btn.textContent = a.path;
        btn.title = a.path;
        btn.addEventListener('click', () => {
          this._selectedIdx = idx;
          this._render();
        });
        li.appendChild(btn);
        list.appendChild(li);
      });
      layout.appendChild(list);
    }

    const main = document.createElement('div');
    main.className = 'artifact-viewer-main';

    const current: ArtifactEntry | undefined = this._data.artifacts[this._selectedIdx];
    if (current === undefined) {
      const p = document.createElement('p');
      p.textContent = '(无文件)';
      main.appendChild(p);
      layout.appendChild(main);
      this.appendChild(layout);
      return;
    }

    const isHtml =
      current.path.toLowerCase().endsWith('.html') ||
      current.path.toLowerCase().endsWith('.htm');

    if (isHtml) {
      const frame = document.createElement('iframe');
      frame.className = 'artifact-viewer-iframe';
      // SECURITY: allow-scripts only — never with allow-same-origin (XSS bypass).
      frame.setAttribute('sandbox', buildSandboxAttribute());
      frame.setAttribute(
        'src',
        `/api/artifact/${encodeURIComponent(this._data.session_id)}/${encodeURIComponent(current.path)}`,
      );
      frame.setAttribute('title', `Preview of ${current.path}`);
      frame.setAttribute('loading', 'lazy');
      main.appendChild(frame);
    } else {
      const note = document.createElement('p');
      note.className = 'artifact-viewer-note';
      note.textContent =
        `${current.path} 是 ${current.content_type} — 无法直接预览。请下载查看。`;
      main.appendChild(note);
    }

    const cap = document.createElement('p');
    cap.className = 'artifact-viewer-caption';
    const pathSpan = document.createElement('span');
    pathSpan.className = 'artifact-viewer-caption-path';
    pathSpan.textContent = current.path;
    cap.appendChild(pathSpan);
    cap.appendChild(document.createTextNode('  ·  '));
    const sizeSpan = document.createElement('span');
    sizeSpan.textContent = formatBytes(current.size_bytes);
    cap.appendChild(sizeSpan);
    cap.appendChild(document.createTextNode('  ·  '));
    const ctSpan = document.createElement('span');
    ctSpan.textContent = current.content_type;
    cap.appendChild(ctSpan);
    main.appendChild(cap);

    // Download link
    const dl = document.createElement('a');
    dl.className = 'artifact-viewer-download';
    dl.href = `/api/artifact/${encodeURIComponent(this._data.session_id)}/${encodeURIComponent(current.path)}`;
    dl.setAttribute('download', current.path);
    dl.textContent = `下载 ${current.path} ↓`;
    main.appendChild(dl);

    layout.appendChild(main);
    this.appendChild(layout);
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosArtifactViewer);
  }
}
