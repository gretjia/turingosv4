// TRACE_MATRIX FC1-N5: read view materialization — client-side router
//
// Derives the current view from location.pathname. W4 will use onPopState
// for back/forward navigation.

export type View = 'dashboard' | 'agents' | 'tasks' | 'audit' | 'build';

/**
 * Derive the current view from the URL pathname.
 *   /          → 'dashboard'
 *   /agents    → 'agents'
 *   /tasks     → 'tasks'
 *   /audit     → 'audit'
 *   /build     → 'build'   (Phase 7 W6: spec-grill interview centerpiece)
 *   anything else → 'dashboard'
 */
export function currentView(): View {
  const path = location.pathname;
  if (path === '/agents' || path.startsWith('/agents/')) {
    return 'agents';
  }
  if (path === '/tasks' || path.startsWith('/tasks/')) {
    return 'tasks';
  }
  if (path === '/audit' || path.startsWith('/audit/')) {
    return 'audit';
  }
  if (path === '/build' || path.startsWith('/build/')) {
    return 'build';
  }
  // '/' and anything else → dashboard
  return 'dashboard';
}

/**
 * Wire a handler for browser popstate events (back/forward navigation).
 * W4 will attach route-switching logic here.
 */
export function onPopState(handler: () => void): void {
  window.addEventListener('popstate', handler);
}
