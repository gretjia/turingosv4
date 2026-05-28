// TRACE_MATRIX FC1-N5: read view materialization — TuringOS frontend entry point
//
// W3: Register all custom elements and log readiness.
// Replaces W0 placeholder stub.

import { register as registerTextBlock } from './components/text-block.js';
import { register as registerTableBlock } from './components/table-block.js';
import { register as registerAgentCardBlock } from './components/agent-card-block.js';
import { register as registerTaskCardBlock } from './components/task-card-block.js';
import { register as registerEventLogBlock } from './components/event-log-block.js';
import { register as registerDashboardPanelBlock } from './components/dashboard-panel-block.js';
import { register as registerTaskOpenForm } from './components/task-open-form.js';
import { register as registerTuringOSStatus } from './components/turingos-status.js';
import { register as registerSpecGrill } from './components/spec-grill.js';
import { register as registerSpecResult } from './components/spec-result.js';
import { register as registerArtifactViewer } from './components/artifact-viewer.js';
import { register as registerAgentAttemptsPanel } from './components/agent-attempts-panel.js';
import { register as registerAgentPresenceTree } from './components/agent-presence-tree.js';
import { register as registerWelcome } from './components/welcome.js';
import { register as registerTuringOSRoot } from './turingos-root.js';
import { currentView } from './router.js';

// Register all custom elements exactly once.
// Each register() is guarded by a customElements.get() sentinel.
registerTextBlock();
registerTableBlock();
registerAgentCardBlock();
registerTaskCardBlock();
registerEventLogBlock();
registerDashboardPanelBlock();
registerTaskOpenForm();
registerTuringOSStatus();
// W6: spec interview centerpiece + artifact preview
registerSpecGrill();
registerSpecResult();
registerArtifactViewer();
// Polymarket PR1: agent attempts panel (read-only market view)
registerAgentAttemptsPanel();
// Live agent-presence causal tree (committed tape + derived progress cursor)
registerAgentPresenceTree();
// W7: first-time-user onboarding wizard
registerWelcome();
registerTuringOSRoot();

document.addEventListener('DOMContentLoaded', () => {
  console.info('TuringOS frontend ready, view:', currentView());
});
