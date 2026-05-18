// TRACE_MATRIX FC1-N5: read view materialization — base styles for client use
//
// Phase 7 W4.4 base styles, exported as a const string for client-side use
// (constructed stylesheets, runtime injection). The canonical authoritative
// copy is at `frontend/src/base-styles.css`; this file mirrors a compact
// subset suited for Web Component scoping (the full sheet ships from the
// server inlined into <head> via include_str!).
//
// Sibling: `frontend/src/design-system.ts`.

export const BASE_STYLES: string = `
/* tos-status badge — used by task-card / event-log / status indicator */
.tos-status {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  font-family: var(--font-mono);
  font-size: 0.6875rem;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.16em;
  padding: 3px 0.5rem;
  border-radius: 2px;
  border: 1px solid currentColor;
  color: var(--color-fg-muted);
  background: transparent;
  white-space: nowrap;
}
.tos-status::before {
  content: "";
  width: 6px; height: 6px; border-radius: 50%;
  background: currentColor; display: inline-block;
}
`;
