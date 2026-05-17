// TRACE_MATRIX FC1-N5: read view materialization — WS connection state pill
//
// <turingos-status> custom element. Renders a small pill in the footer
// indicating the current WebSocket connection state. Listens for
// `turingos:ws_state` CustomEvents dispatched by the inline WS bootstrap
// script in render.rs.
//
// States: connecting | connected | reconnecting | disconnected.
// Styling lives in design-system.css (.tos-conn-pill / .tos-conn-dot).
// XSS hygiene: textContent only; no innerHTML.

const ELEMENT_NAME = 'turingos-status';

type ConnState = 'connecting' | 'connected' | 'reconnecting' | 'disconnected';

const STATE_LABEL: Record<ConnState, string> = {
  connecting: 'connecting',
  connected: 'connected',
  reconnecting: 'reconnecting',
  disconnected: 'offline',
};

declare global {
  interface Window {
    __turingos_ws_state?: ConnState;
  }
}

export class TuringOSStatus extends HTMLElement {
  private _pill: HTMLSpanElement | null = null;
  private _label: HTMLSpanElement | null = null;
  private _bound: ((e: Event) => void) | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'connection_status');
    this._render();
    // Reflect any state already established before this element mounted.
    const initial = (window.__turingos_ws_state as ConnState | undefined) ?? 'connecting';
    this._apply(initial);

    this._bound = (e: Event) => {
      const detail = (e as CustomEvent<{ state: ConnState }>).detail;
      if (detail && typeof detail.state === 'string') {
        this._apply(detail.state);
      }
    };
    document.addEventListener('turingos:ws_state', this._bound);
  }

  disconnectedCallback(): void {
    if (this._bound !== null) {
      document.removeEventListener('turingos:ws_state', this._bound);
      this._bound = null;
    }
  }

  private _render(): void {
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    const pill = document.createElement('span');
    pill.className = 'tos-conn-pill';
    pill.setAttribute('role', 'status');
    pill.setAttribute('aria-live', 'polite');

    const dot = document.createElement('span');
    dot.className = 'tos-conn-dot';
    dot.setAttribute('aria-hidden', 'true');
    pill.appendChild(dot);

    const label = document.createElement('span');
    label.className = 'tos-conn-label';
    label.textContent = STATE_LABEL.connecting;
    pill.appendChild(label);

    this.appendChild(pill);
    this._pill = pill;
    this._label = label;
  }

  private _apply(state: ConnState): void {
    if (this._pill === null || this._label === null) return;
    this._pill.dataset['state'] = state;
    this._label.textContent = STATE_LABEL[state] ?? state;
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TuringOSStatus);
  }
}
