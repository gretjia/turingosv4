// TRACE_MATRIX FC1-N5: read view materialization — <turingos-root> host element
// TRACE_MATRIX FC1-N10: write-path integration — task_created WS message handling
//
// Subscribes to "turingos:ir_update" events dispatched by the W2 inline WS
// script and re-renders the matching block-type custom elements for the current
// view. Caches all received IRRoot payloads keyed by view for route changes.
// XSS hygiene: all rendering delegated to sub-components which use textContent.
//
// W4: also handles "task_created" events — on receive, re-fetches /api/tasks
// and re-renders the tasks view within ~200ms (gives backend write settle time).
//
// W4: on /tasks view, renders a <tos-task-open-form> element above the task
// list. Other views (dashboard, agents, audit) do NOT render the form.

import type { IRRoot, WsMessage, Block } from './ir.js';
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
      p.textContent = 'Connecting…';
      this.appendChild(p);
    } else {
      // Re-render from cache if we have a cached view (e.g. reconnect).
      this._renderCurrentView();
    }

    // Subscribe to WS messages (both ir_update and task_created).
    this._boundListener = (e: Event) => this._onWsMessage(e);
    document.addEventListener('turingos:ir_update', this._boundListener);
  }

  disconnectedCallback(): void {
    if (this._boundListener !== null) {
      document.removeEventListener('turingos:ir_update', this._boundListener);
      this._boundListener = null;
    }
  }

  private _onWsMessage(e: Event): void {
    const detail = (e as CustomEvent<WsMessage>).detail;
    if (detail == null) return;

    if (detail.msg_type === 'ir_update') {
      if (detail.ir == null) return;
      // Store in cache for all views.
      this._cache.set(detail.view, detail.ir);

      // Only re-render if this update is for the current view.
      // /audit reuses 'dashboard' rendering (per W3 spec §3).
      const view = currentView();
      const effectiveView: string = view === 'audit' ? 'dashboard' : view;
      if (detail.view === effectiveView) {
        this._renderIr(detail.ir);
      }
    } else if (detail.msg_type === 'task_created') {
      // W4: task_created received → re-fetch /api/tasks and update the tasks view.
      // Small delay (200ms) lets the backend write settle before we read back.
      setTimeout(() => {
        fetch('/api/tasks')
          .then((r) => r.json())
          .then((ir: unknown) => {
            const irRoot = ir as IRRoot;
            this._cache.set('tasks', irRoot);
            // Only re-render if we are currently on the tasks view.
            if (currentView() === 'tasks') {
              this._renderIr(irRoot);
            }
          })
          .catch((err: unknown) => {
            // Non-fatal: log the error but do not break the page.
            console.warn('turingos-root: failed to re-fetch /api/tasks after task_created', err);
          });
      }, 200);
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

    // W4: on /tasks view only, render the task-open form above the task list.
    if (currentView() === 'tasks') {
      const formEl = document.createElement('tos-task-open-form');
      this.appendChild(formEl);
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
