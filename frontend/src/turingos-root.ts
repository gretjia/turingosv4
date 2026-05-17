// TRACE_MATRIX FC1-N5: read view materialization — <turingos-root> host element
//
// Subscribes to "turingos:ir_update" events dispatched by the W2 inline WS
// script and re-renders the matching block-type custom elements for the current
// view. Caches all received IRRoot payloads keyed by view for route changes.
// XSS hygiene: all rendering delegated to sub-components which use textContent.

import type { IRRoot, IRUpdateEvent, Block } from './ir.js';
import { currentView } from './router.js';

const ELEMENT_NAME = 'turingos-root';

export class TuringOSRoot extends HTMLElement {
  /** Cache of received IR by view name. */
  private _cache: Map<string, IRRoot> = new Map();

  private _boundListener: ((e: Event) => void) | null = null;

  connectedCallback(): void {
    // Render connecting placeholder until first IR arrives.
    if (this._cache.size === 0) {
      const p = document.createElement('p');
      p.textContent = 'Connecting…'; // "Connecting…"
      this.appendChild(p);
    } else {
      // Re-render from cache if we have a cached view (e.g. reconnect).
      this._renderCurrentView();
    }

    // Subscribe to WS IR updates.
    this._boundListener = (e: Event) => this._onIrUpdate(e);
    document.addEventListener('turingos:ir_update', this._boundListener);
  }

  disconnectedCallback(): void {
    if (this._boundListener !== null) {
      document.removeEventListener('turingos:ir_update', this._boundListener);
      this._boundListener = null;
    }
  }

  private _onIrUpdate(e: Event): void {
    const detail = (e as CustomEvent<IRUpdateEvent>).detail;
    if (detail == null || detail.msg_type !== 'ir_update' || detail.ir == null) {
      return;
    }
    // Store in cache for all views.
    this._cache.set(detail.view, detail.ir);

    // Only re-render if this update is for the current view.
    // /audit reuses 'dashboard' rendering (per W3 spec §3).
    const view = currentView();
    const effectiveView: string = view === 'audit' ? 'dashboard' : view;
    if (detail.view === effectiveView) {
      this._renderIr(detail.ir);
    }
  }

  /** Re-render using whatever is in the cache for the current view. */
  private _renderCurrentView(): void {
    const view = currentView();
    const effectiveView: string = view === 'audit' ? 'dashboard' : view;
    const cached = this._cache.get(effectiveView);
    if (cached != null) {
      this._renderIr(cached);
    }
  }

  private _renderIr(ir: IRRoot): void {
    // Clear self.
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    // Render each block.
    for (const block of ir.blocks) {
      const el = buildBlockElement(block);
      if (el !== null) {
        this.appendChild(el);
      }
    }
  }
}

/** Instantiate the matching custom element for a Block and wire up its payload. */
function buildBlockElement(block: Block): HTMLElement | null {
  let el: HTMLElement;
  switch (block.kind) {
    case 'text':
      el = document.createElement('tos-text-block');
      break;
    case 'table':
      el = document.createElement('tos-table-block');
      break;
    case 'agent_card':
      el = document.createElement('tos-agent-card-block');
      break;
    case 'task_card':
      el = document.createElement('tos-task-card-block');
      break;
    case 'event_log':
      el = document.createElement('tos-event-log-block');
      break;
    case 'dashboard_panel':
      el = document.createElement('tos-dashboard-panel-block');
      break;
    default:
      // Exhaustive: TypeScript never type; at runtime silently skip unknown kinds.
      return null;
  }
  // Pass payload via dataset — components read this in connectedCallback.
  el.dataset['payload'] = JSON.stringify(block);
  return el;
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TuringOSRoot);
  }
}
