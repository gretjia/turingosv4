// TuringOS Web UI kit — /welcome 5-step onboarding wizard.
//
// Faithful recreation of frontend/src/components/welcome.ts. The state
// machine, copy, and visual structure are kept exactly; the network calls
// to /api/welcome/* are stubbed with setTimeout for the mock click-thru.
//
// 5 steps: 准备工作站 → 配置模型 → API 密钥 → 注册 Agent → 就绪

const { useState: useStateW, useEffect: useEffectW, useRef: useRefW } = React;

const WIZARD_STEPS = [
  { key: "Init",        label: "准备工作站" },
  { key: "LlmConfig",   label: "配置模型" },
  { key: "ApiKey",      label: "API 密钥" },
  { key: "AgentDeploy", label: "注册 Agent" },
  { key: "Done",        label: "就绪" },
];

function stepIndex(nextStep) {
  return Math.max(0, WIZARD_STEPS.findIndex((s) => s.key === nextStep));
}

function WelcomeProgress({ currentStep }) {
  const activeIdx = stepIndex(currentStep);
  return (
    <ol className="welcome-progress" aria-label="安装进度">
      {WIZARD_STEPS.map((step, idx) => {
        let phase;
        if (currentStep === "Done") phase = "done";
        else if (idx < activeIdx) phase = "done";
        else if (idx === activeIdx) phase = "active";
        else phase = "pending";
        return (
          <li key={step.key} className="welcome-progress-step" data-phase={phase}>
            <span className="welcome-progress-num">{idx + 1}</span>
            <span className="welcome-progress-label">{step.label}</span>
          </li>
        );
      })}
    </ol>
  );
}

/* ────────────────────────────────────────────────────────────
   Step copy — lifted verbatim from welcome.ts so the brand
   voice is preserved.
   ──────────────────────────────────────────────────────────── */
const STEP_COPY = {
  Init: {
    index: 1,
    title: "第一步 · 准备工作站",
    subtitle:
      "我帮你在硬盘上铺一张空白的「账本桌面」——里面有 genesis_payload.toml 和 agent_pubkeys.json，是后面所有步骤的地基。",
    cta: "准备工作站 →",
    submitting: "正在初始化工作站",
  },
  LlmConfig: {
    index: 2,
    title: "第二步 · 配置两个模型",
    subtitle:
      "我会把两个 LLM 写进 turingos.toml——一个负责「问你问题」（DeepSeek V3.2），一个负责「写代码」（Qwen3-Coder 30B）。只写模型名字，不写密钥。",
    cta: "写入 turingos.toml →",
    submitting: "正在写入模型配置",
  },
  AgentDeploy: {
    index: 4,
    title: "第三步 · 给工作站注册一个 Agent",
    subtitle:
      "注册一个 Solver 角色的 agent_001，告诉系统「以后是这个 agent 在跑工作」。这是 Phase 6.1 的多 agent 体系的最小入口。",
    cta: "注册 agent_001 →",
    submitting: "正在注册 Agent",
  },
};

function WelcomeStepCard({ stepKey, onAdvance }) {
  const copy = STEP_COPY[stepKey];
  const [submitting, setSubmitting] = useStateW(false);

  const click = () => {
    setSubmitting(true);
    setTimeout(() => { setSubmitting(false); onAdvance?.(); }, 850);
  };

  return (
    <div className="welcome-card">
      <p className="welcome-step-caption">{`STEP ${copy.index} / 5`}</p>
      <h2 className="welcome-step-title">{copy.title}</h2>
      <p className="welcome-step-subtitle">{copy.subtitle}</p>
      {submitting
        ? <LoadingPhrase>{copy.submitting}</LoadingPhrase>
        : <button type="button" className="welcome-cta" onClick={click}>{copy.cta}</button>}
    </div>
  );
}

function WelcomeApiKeyCard({ onAdvance }) {
  const [val, setVal] = useStateW("");
  const [submitting, setSubmitting] = useStateW(false);
  const [done, setDone] = useStateW(false);
  const inputRef = useRefW(null);

  useEffectW(() => { inputRef.current?.focus(); }, []);

  const save = () => {
    if (!val.trim().startsWith("sk-")) return;
    setSubmitting(true);
    setTimeout(() => { setSubmitting(false); setDone(true); setTimeout(onAdvance, 350); }, 700);
  };

  return (
    <div className="welcome-card">
      <p className="welcome-step-caption">STEP 3 / 5</p>
      <h2 className="welcome-step-title">把 SiliconFlow 的 API 密钥交给我</h2>
      <p className="welcome-step-subtitle">
        密钥只活在这个服务器进程的内存里——重启就丢，从不写盘、不进日志、不会回显在网页上。
        你只需要在每次启动 turingos_web 之后填一次。
      </p>
      {submitting && <LoadingPhrase>正在保存到内存</LoadingPhrase>}
      {!submitting && done && (
        <p className="welcome-api-set">API 密钥已设置（仅保存在内存中）</p>
      )}
      {!submitting && !done && (
        <Fragment>
          <div className="welcome-api-field">
            <label className="welcome-api-label" htmlFor="welcome-api-key-input">
              SILICONFLOW_API_KEY
            </label>
            <input
              ref={inputRef}
              id="welcome-api-key-input"
              type="password"
              placeholder="sk-..."
              autoComplete="off"
              spellCheck="false"
              className="welcome-api-input"
              value={val}
              onChange={(e) => setVal(e.target.value)}
              onKeyDown={(e) => { if (e.key === "Enter") save(); }}
            />
          </div>
          <button type="button" className="welcome-cta" onClick={save}>
            保存密钥 →
          </button>
        </Fragment>
      )}
    </div>
  );
}

function WelcomeReadyCard({ onLaunch }) {
  return (
    <div className="welcome-card welcome-ready-card">
      <p className="welcome-step-caption">完成 / READY</p>
      <h2 className="welcome-step-title">你的工作站已就绪。</h2>
      <p className="welcome-step-subtitle">
        五步全部完成。点下面开始 spec 访谈——我会问你八个关于「日常麻烦」的问题，然后帮你生成一个小工具。
      </p>
      <button type="button" className="welcome-cta" onClick={onLaunch}>
        开始 spec 访谈 →
      </button>
    </div>
  );
}

/* ────────────────────────────────────────────────────────────
   Welcome screen — full wrap: header (no nav), main content,
   footer. Drives its own state machine.
   ──────────────────────────────────────────────────────────── */
function WelcomeScreen({ onLaunchBuild }) {
  const [nextStep, setNextStep] = useStateW("Init");
  const advance = (target) => () => setNextStep(target);

  let card;
  if (nextStep === "Init")        card = <WelcomeStepCard stepKey="Init"        onAdvance={advance("LlmConfig")} />;
  else if (nextStep === "LlmConfig") card = <WelcomeStepCard stepKey="LlmConfig" onAdvance={advance("ApiKey")} />;
  else if (nextStep === "ApiKey") card = <WelcomeApiKeyCard onAdvance={() => setNextStep("AgentDeploy")} />;
  else if (nextStep === "AgentDeploy") card = <WelcomeStepCard stepKey="AgentDeploy" onAdvance={advance("Done")} />;
  else card = <WelcomeReadyCard onLaunch={onLaunchBuild} />;

  return (
    <div className="tos-page-wrap">
      <header className="tos-header" role="banner">
        <Wordmark href="/welcome" />
        <a href="#" className="tos-welcome-skip" onClick={(e) => { e.preventDefault(); onLaunchBuild?.(); }}>
          skip → build
        </a>
      </header>
      <main className="tos-main tos-main-welcome" id="tos-main" role="main">
        <section data-block-type="welcome">
          <div className="welcome-wrap">
            <WelcomeProgress currentStep={nextStep} />
            {card}
          </div>
        </section>
      </main>
      <Footer connection="connected" />
    </div>
  );
}

Object.assign(window, { WelcomeScreen, WelcomeProgress, WIZARD_STEPS });
