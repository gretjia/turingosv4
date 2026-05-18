// TRACE_MATRIX FC1-N5: read view materialization — text block component
//
// <tos-text-block> custom element. Renders TextBlock IR payload as <p>
// elements wrapped in an <article>-styled host (light DOM; styled via the
// global [data-block-type="text"] selectors from base-styles.css).
//
// XSS hygiene: uses textContent exclusively — never innerHTML with dynamic
// strings.

import type { TextBlock } from '../ir.js';

const ELEMENT_NAME = 'tos-text-block';

export class TosTextBlock extends HTMLElement {
  private _block: TextBlock | null = null;

  connectedCallback(): void {
    this.setAttribute('data-block-type', 'text');
    this.classList.add('block', 'block-text');
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
    while (this.firstChild) {
      this.removeChild(this.firstChild);
    }
    if (block === null) {
      return;
    }
    const lines = block.content.split('\n');
    for (const line of lines) {
      if (line.length === 0) continue;
      const p = document.createElement('p');
      p.textContent = line;
      this.appendChild(p);
    }
  }
}

export function register(): void {
  if (!customElements.get(ELEMENT_NAME)) {
    customElements.define(ELEMENT_NAME, TosTextBlock);
  }
}
