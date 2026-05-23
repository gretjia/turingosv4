// TuringOS Web UI kit — app router.
//
// Wires up the six screens (welcome, build, dashboard, agents, tasks, audit)
// into a single SPA. State is held in URL hash so refresh preserves position;
// nav clicks rewrite the hash.

const { useState: useStateA, useEffect: useEffectA, Fragment: FragmentA } = React;

const ROUTES = ["welcome", "build", "dashboard", "agents", "tasks", "audit"];

function readHash() {
  const h = window.location.hash.replace(/^#\/?/, "");
  if (ROUTES.includes(h)) return h;
  return "welcome";
}

function App() {
  const [view, setView] = useStateA(readHash());

  useEffectA(() => {
    const onHash = () => setView(readHash());
    window.addEventListener("hashchange", onHash);
    return () => window.removeEventListener("hashchange", onHash);
  }, []);

  const navigate = (next) => {
    if (next === view) return;
    window.location.hash = `#/${next}`;
    setView(next);
  };

  // body data-view drives small per-view style hooks in base-styles.css
  useEffectA(() => {
    document.body.dataset.view = view;
  }, [view]);

  if (view === "welcome")   return <WelcomeScreen   onLaunchBuild={() => navigate("build")} />;
  if (view === "build")     return <BuildScreen     onNavigate={navigate} />;
  if (view === "agents")    return <AgentsScreen    onNavigate={navigate} />;
  if (view === "tasks")     return <TasksScreen     onNavigate={navigate} />;
  if (view === "audit")     return <AuditScreen     onNavigate={navigate} />;
  return <DashboardScreen onNavigate={navigate} />;
}

ReactDOM.createRoot(document.getElementById("root")).render(<App />);
