// TRACE_MATRIX FC1-N5: read view materialization — text block component
//
// <tos-text-block> custom element. Renders TextBlock IR payload as <p> elements.
// XSS hygiene: uses textContent exclusively — never innerHTML with dynamic strings.

import type { TextBlock } from '../ir.js';

/** Sentinel to avoid double-define on hot-reload. */
const ELEMENT_NAME = 'tos-text-block';

export class TosTextBlock extends HTMLElement {
  private _block: TextBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'text');
    // If payload was set before connection, render now.
    const payloadAttr = this.dataset['payload'];
    if (payloadAttr != null && this._block === null) {
      try {
        this._block = JSON.parse(payloadAttr) as TextBlock;
      } catch {
        // Malformed payload — render nothing.
      }
    }
    this._render();
  }

  /** Update with a new TextBlock payload (for incremental updates from turingos-root). */
  update(block: TextBlock): void {
    this._block = block;
    if (this.isConnected) {
      this._render();
    }
  }

  private _render(): void {
    const block = this._block;
    // Clear children safely (no innerHTML).
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (block === null) {
      return;
    }
    // Split on newlines; render each line as a <p>.
    const lines = block.content.split('\n');
    for (const line of lines) {
      const p = document.createElement('p');
      p.textContent = line;
      this.appendChild(p);
    }
  }
}

/** Register the custom element exactly once. */
export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTextBlock);
  }
}
