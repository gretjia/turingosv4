# TuringOS Web — UI Kit

High-fidelity React recreation of the **`turingos_web`** browser surface
shipped from `turingosv4/frontend/src/`. Designed for clickable mocks, not
production — every network call is a mock `setTimeout`, and the IR data
is hard-coded.

## Open

```
ui_kits/web/index.html
```

The kit is a single-page app routed by URL hash:

| Hash | Screen | Source it mirrors |
|---|---|---|
| `#/welcome`   | 5-step onboarding wizard         | `frontend/src/components/welcome.ts` |
| `#/build`     | Spec-grill interview spread      | `frontend/src/components/spec-grill.ts` |
| `#/dashboard` | Tape epoch metrics + event log   | `src/web/render.rs` dashboard view |
| `#/agents`    | Multi-agent ledger cards         | `frontend/src/components/agent-card-block.ts` |
| `#/tasks`     | Task market + open-task form     | `frontend/src/components/task-open-form.ts` |
| `#/audit`     | ChainTape capsule table          | `src/web/render.rs` audit view |

Click-thru flow that proves the full happy path:

1. Land on `#/welcome` — step through **准备工作站 → 配置模型 → API 密钥 → 注册 Agent → 就绪**.
2. Hit `开始 spec 访谈 →` to land on `#/build`.
3. Click `开始 spec 访谈 →` again to enter the interview; answer 7 turns.
4. Watch the loading ellipsis settle into the synthesized `spec.md`.
5. Hit `生成代码 →` to see the artifact-viewer iframe placeholder.
6. Use the top nav to jump to `#/agents`, `#/tasks`, `#/audit`, `#/dashboard` and inspect the IR-driven views.

## File layout

```
ui_kits/web/
├── README.md         ← you are here
├── index.html        ← single-page entry; loads the four JSX files in order
├── styles.css        ← concatenation of design-system.css + base-styles.css
├── Chrome.jsx        ← Wordmark, Header, Nav, Footer, ConnectionPill,
│                       StatusBadge, LoadingPhrase, PageShell, PageTitle, Dl, ShortHash
├── Welcome.jsx       ← WelcomeScreen + 5-step wizard state machine
├── Build.jsx         ← BuildScreen + spec-grill state machine + ArtifactViewer
├── Screens.jsx       ← TextBlock, DashboardPanel, EventLog, TableBlock,
│                       AgentCard, TaskCard +
│                       DashboardScreen / AgentsScreen / TasksScreen / AuditScreen
└── App.jsx           ← hash router, mounts <App> on #root
```

Each `*.jsx` script runs in its own Babel scope. Components destined for
other files are exported via `Object.assign(window, { … })` at the bottom
of each — see the [agent guidelines][1] for why.

[1]: https://github.com/anthropics/anthropic-cookbook

## What this kit does NOT cover

- The Rust backend (`turingos_web` axum server) — there is no fetch / WS layer.
- Spec-template output documents (`assets/templates/spec_template.html`). Those
  are a separate visual register (warmer, with emoji section markers); not part
  of the chrome and out of scope for this kit.
- The `turingos` CLI. The CLI is plain terminal output (boxed Unicode rules,
  status mark-ups in monospace), not a graphical surface — a CLI screenshot
  mock would be a separate kit.

## Pixel-fidelity notes

The visual chrome is **byte-for-byte the same CSS** the production server
inlines (`styles.css` here is the concatenation of `frontend/src/design-system.css`
and `frontend/src/base-styles.css`). The deltas from the production app are:

- Component implementations are React, not Web Components. Same DOM, same
  `data-block-type` attributes, same class names — only the runtime is different.
- The spec-grill turn loop is hard-coded to 7 canonical questions instead
  of being LLM-decided turn-by-turn.
- The artifact viewer's iframe shows a styled placeholder instead of a
  real sandboxed `index.html`.
- No WebSocket connection; the footer connection pill is statically
  `connected`.
