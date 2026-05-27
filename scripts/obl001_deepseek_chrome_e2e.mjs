#!/usr/bin/env node

import fs from 'node:fs';
import path from 'node:path';
import { spawn } from 'node:child_process';
import { createServer } from 'node:net';
import { createRequire } from 'node:module';

const require = createRequire(import.meta.url);
let playwright;
try {
  playwright = require('playwright');
} catch {
  playwright = require('/tmp/obl001-playwright/node_modules/playwright');
}
const { chromium } = playwright;

// ============================================================================
// 1. Preflight Env & Load Variables
// ============================================================================

function loadEnv() {
  const possiblePaths = [
    path.join(process.cwd(), '.env'),
    '/home/zephryj/projects/turingosv4/.env'
  ];
  for (const envPath of possiblePaths) {
    if (fs.existsSync(envPath)) {
      const content = fs.readFileSync(envPath, 'utf8');
      for (const line of content.split('\n')) {
        const trimmed = line.trim();
        if (trimmed && !trimmed.startsWith('#') && trimmed.includes('=')) {
          const idx = trimmed.indexOf('=');
          const key = trimmed.substring(0, idx).trim();
          let value = trimmed.substring(idx + 1).trim();
          if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
            value = value.substring(1, value.length - 1);
          }
          if (key && !process.env[key]) {
            process.env[key] = value;
          }
        }
      }
    }
  }
}

loadEnv();

if (!process.env.TURINGOS_SILICONFLOW_ENDPOINT && process.env.DEEPSEEK_API_URL) {
  process.env.TURINGOS_SILICONFLOW_ENDPOINT = process.env.DEEPSEEK_API_URL;
}

// Preflight checks
const deepseekApiKey = process.env.DEEPSEEK_API_KEY;
if (!deepseekApiKey) {
  console.error("Preflight env check failed: DEEPSEEK_API_KEY required.");
  process.exit(1);
}

const siliconflowEndpoint = process.env.TURINGOS_SILICONFLOW_ENDPOINT;
if (!siliconflowEndpoint || !siliconflowEndpoint.toLowerCase().includes('deepseek')) {
  console.error("Preflight env check failed: TURINGOS_SILICONFLOW_ENDPOINT must contain 'deepseek'.");
  process.exit(1);
}

const turingosBin = path.resolve('target/debug/turingos');
const turingosWebBin = path.resolve('target/debug/turingos_web');
if (!fs.existsSync(turingosBin) || !fs.existsSync(turingosWebBin)) {
  console.error("Preflight check failed: target/debug/turingos and turingos_web must exist.");
  process.exit(1);
}

function checkPortFree(port) {
  return new Promise((resolve) => {
    const server = createServer();
    server.once('error', () => resolve(false));
    server.once('listening', () => {
      server.close();
      resolve(true);
    });
    server.listen(port, '127.0.0.1');
  });
}

const portFree = await checkPortFree(8080);
if (!portFree) {
  console.error("Preflight check failed: port 8080 must be free.");
  process.exit(1);
}

console.log("[ok] Preflight checks passed.");

// ============================================================================
// 2. Redaction & Evidence Helpers
// ============================================================================

const generateTimeoutMs = Number.parseInt(process.env.OBL001_GENERATE_TIMEOUT_MS || '1920000', 10);

const timestamp = new Date().toISOString().replace(/[-:]/g, '').split('.')[0] + 'Z';
const evidenceRoot = path.resolve(`handover/evidence/obl001_deepseek_chrome_${timestamp}`);
fs.mkdirSync(evidenceRoot, { recursive: true });
console.log(`[info] Evidence root configured at: ${evidenceRoot}`);

const preflightRecord = {
  timestamp,
  endpoint_host: (() => {
    try {
      return new URL(process.env.TURINGOS_SILICONFLOW_ENDPOINT).host;
    } catch {
      return '[unparseable]';
    }
  })(),
  turingos_bin: turingosBin,
  turingos_web_bin: turingosWebBin,
  port_8080_free_at_start: portFree,
  deepseek_api_key_present: Boolean(process.env.DEEPSEEK_API_KEY),
  deepseek_worker_key_present: Boolean(process.env.DEEPSEEK_API_KEY_WORKER),
  timeout_ms: generateTimeoutMs
};
fs.writeFileSync(path.join(evidenceRoot, 'preflight.json'), JSON.stringify(preflightRecord, null, 2));

async function safeScreenshot(page, filePath, warnings) {
  try {
    await page.screenshot({ path: filePath });
  } catch (err) {
    const msg = err.message || String(err);
    console.warn(`[warning] Failed to capture screenshot at ${filePath}:`, msg);
    warnings.push(`Failed to capture ${path.basename(filePath)}: ${msg}`);
  }
}

function makeRedactor() {
  const secrets = [];
  for (const [key, value] of Object.entries(process.env)) {
    if (value && (key.includes('KEY') || key.includes('SECRET') || key.includes('TOKEN') || value.startsWith('sk-') || value.startsWith('hf_'))) {
      if (value.length >= 6) {
        secrets.push(value);
      }
    }
  }

  return function redact(str) {
    if (!str) return str;
    let redacted = str;
    for (const secret of secrets) {
      redacted = redacted.split(secret).join('[REDACTED]');
    }
    redacted = redacted.replace(/sk-[a-zA-Z0-9._-]+/g, '[REDACTED]');
    redacted = redacted.replace(/hf_[a-zA-Z0-9_-]+/g, '[REDACTED]');
    return redacted;
  };
}

const redact = makeRedactor();

// ============================================================================
// 3. Personas Config
// ============================================================================

const personas = [
  {
    name: "退休会计师老张",
    brief: "一个想要超大字体记账计算器的退休会计师。他视力不好，要求界面字体大、色彩对比度高，只需要简单的加减乘除和历史记录，不需要任何云端备份。"
  },
  {
    name: "物理老师李老师",
    brief: "一个需要单摆运动模拟器的中学物理老师。他希望能在网页上调节摆长和重力加速度，观察单摆的周期变化，并能一键暂停和重置模拟。"
  },
  {
    name: "主厨阿明",
    brief: "一个寻找厨房单位转换器的主厨。他需要快速把克（grams）转换成盎司（ounces）、摄氏度转换成华氏度，界面要简单，能单手在手机上操作。"
  },
  {
    name: "园艺爱好者小芳",
    brief: "一个需要植物浇水提醒日程表的园艺爱好者。她想输入植物的名称和浇水间隔天数，并在网页上生成一个本周浇水清单，保存在浏览器本地即可。"
  },
  {
    name: "前端工程师小陈",
    brief: "一个需要正则表达式测试小工具的程序员。他需要输入正则表达式和一段文本，在网页上高亮匹配到的内容，并显示匹配的捕获组。"
  },
  {
    name: "棋友老刘",
    brief: "一个需要双人棋类计时器（Chess Clock）的国际象棋爱好者。他需要支持加秒（Fischer delay/increment）和预设时间，界面要有醒目的红黑对比色。"
  },
  {
    name: "西语学生小美",
    brief: "一个需要西班牙语单词记忆卡片（Flashcard）的学生。她需要自己输入单词和翻译，保存在本地，然后能随机抽取卡片进行正反面翻转测试。"
  },
  {
    name: "咖啡发烧友大强",
    brief: "一个咖啡冲煮比例计算器用户。他需要输入咖啡粉克数，自动按比例计算注水总量，并包含一个简单的分段注水倒计时计时器。"
  },
  {
    name: "音乐教师王老师",
    brief: "一个需要节拍器（Metronome）的钢琴老师。他需要自由调节BPM（速度），选择2/4、3/4、4/4拍子，并有闪烁的视觉指示灯和声音提示。"
  },
  {
    name: "宝妈丽萨",
    brief: "一个需要儿童家务清单与积分板（Chore Chart）的妈妈。她希望给两个孩子列出每天的家务，做完可以勾选，并在网页上累加他们的积分以兑换奖励。"
  },
  {
    name: "徒步旅行者老野",
    brief: "一个旅行打包清单生成器用户。他想输入去往的目的地天气（如冷、热、雨）和旅行天数，自动生成一份基础的户外装备打包清单，并可手动添加删除项目。"
  },
  {
    name: "自由撰稿人阿秋",
    brief: "一个极简无干扰写作计时器与字数统计器用户。他希望界面全屏无杂质，设定一个目标字数 and 限时倒计时，并在达成目标时播放粒子效果动画。"
  },
  {
    name: "马拉松跑者大飞",
    brief: "一个跑步配速计算器用户。他需要输入跑步距离和总时间，计算出每公里的平均配速（分/秒），或者输入配速和距离计算预计完赛时间。"
  },
  {
    name: "茶艺爱好者静静",
    brief: "一个泡茶计时器用户。她想预设绿茶、红茶、普洱茶等不同茶类的水温和冲泡时间，点击即可开始倒计时，并能在最后一分钟发出轻柔的提示。"
  },
  {
    name: "桌游玩家阿豪",
    brief: "一个卡牌/桌游记分牌用户。他需要支持多达6个玩家的姓名录入，并在桌面上以卡片形式显示各人的当前分数，能通过加减按钮快速更新分数。"
  },
  {
    name: "猫奴小吴",
    brief: "一个猫咪喂食时间记录器用户。他想记录每次喂猫的时间和猫粮克数，自动在本地保存，并显示距离上一次喂食过去了多少小时。"
  },
  {
    name: "网页设计师阿雅",
    brief: "一个网页对比度检查器（Accessibility Check）用户。她需要输入前景色和背景色的十六进制代码，自动计算出对比度比例，并给出WCAG AA/AAA级标准是否通过的结论。"
  },
  {
    name: "摄影爱好者老麦",
    brief: "一个景深计算器（Depth of Field Calculator）用户。他需要输入相机画幅、镜头焦距、光圈值和对焦距离，自动计算出前景深、后景深以及超焦距。"
  }
];

const targetCompletions = Number.parseInt(process.env.OBL001_TARGET_COMPLETIONS || '15', 10);
const maxPersonas = Math.min(
  personas.length,
  Number.parseInt(process.env.OBL001_MAX_PERSONAS || String(personas.length), 10)
);

// ============================================================================
// 4. API Client for Simulated User
// ============================================================================

async function callDeepSeekDirect(question, personaBrief, history) {
  const endpoint = process.env.TURINGOS_SILICONFLOW_ENDPOINT;
  const apiKey = process.env.DEEPSEEK_API_KEY_WORKER || process.env.DEEPSEEK_API_KEY;
  const model = process.env.OBL001_USER_MODEL || 'deepseek-v4-flash';

  const messages = [
    {
      role: 'system',
      content: `你正在模拟一位用户参加一个软件需求访谈。你的角色画像如下：
"${personaBrief}"

请根据画像，自然、简短地回答访谈助手的问题。你的回答代表你想要开发的小工具的功能需求。回答必须符合你的角色，使用中文，且长度控制在1-2句话内。不要出戏，不要作多余解释。`
    },
    ...history,
    {
      role: 'user',
      content: question
    }
  ];

  const res = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${apiKey}`
    },
    body: JSON.stringify({
      model,
      messages: messages,
      temperature: 0.7,
      max_tokens: 200
    })
  });

  if (!res.ok) {
    const errText = await res.text();
    throw new Error(`DeepSeek API request failed: HTTP ${res.status} - ${redact(errText)}`);
  }

  const data = await res.json();
  if (!data.choices || data.choices.length === 0) {
    throw new Error(`DeepSeek API returned empty choices: ${JSON.stringify(data)}`);
  }

  return data.choices[0].message.content.trim();
}

function waitForSpecTurn(page, timeout = 120000) {
  return page.waitForResponse(
    (resp) => resp.url().includes('/api/spec/turn') && resp.request().method() === 'POST',
    { timeout }
  );
}

// ============================================================================
// 5. Port Poller
// ============================================================================

async function waitForServer(url, timeoutMs = 30000) {
  const start = Date.now();
  while (Date.now() - start < timeoutMs) {
    try {
      const res = await fetch(url);
      if (res.status === 200) {
        return true;
      }
    } catch {
      // Retry
    }
    await new Promise((r) => setTimeout(r, 1000));
  }
  throw new Error(`Server at ${url} did not start within ${timeoutMs}ms`);
}

// ============================================================================
// 6. Tape Presence Audit
// ============================================================================

function checkTapePresence(workspaceDir, sessionId) {
  const sessionDir = path.join(workspaceDir, 'sessions', sessionId);
  const specMdPath = path.join(sessionDir, 'spec.md');
  const artifactsDir = path.join(sessionDir, 'artifacts');
  const l4Path = path.join(workspaceDir, 'runtime_repo', '.git', 'refs', 'chaintape', 'l4');
  const rejectionsPath = path.join(workspaceDir, 'runtime_repo', 'rejections.jsonl');
  const casDir = path.join(workspaceDir, 'cas');

  let spec_md = false;
  try {
    spec_md = fs.existsSync(specMdPath) && fs.statSync(specMdPath).size > 0;
  } catch {}

  let artifacts_count = 0;
  try {
    if (fs.existsSync(artifactsDir)) {
      artifacts_count = fs.readdirSync(artifactsDir).length;
    }
  } catch {}

  let chaintape_l4 = false;
  try {
    chaintape_l4 = fs.existsSync(l4Path);
  } catch {}

  let rejections_jsonl = false;
  try {
    rejections_jsonl = fs.existsSync(rejectionsPath);
  } catch {}

  let cas_dir = false;
  try {
    cas_dir = fs.existsSync(casDir) && fs.readdirSync(casDir).length > 0;
  } catch {}

  return {
    spec_md,
    artifacts_count,
    chaintape_l4,
    rejections_jsonl,
    cas_dir
  };
}

// ============================================================================
// 7. Core Execution Loop
// ============================================================================

async function run() {
  const headless = process.env.OBL001_HEADLESS === '1';
  console.log(`[info] Headless mode: ${headless}`);

  const browser = await chromium.launch({
    channel: 'chrome',
    headless: headless,
    args: ['--disable-gpu', '--disable-dev-shm-usage']
  });
  const browserInfo = {
    chrome_version: browser.version(),
    user_agent: ''
  };

  const personaResults = [];
  let completedCount = 0;
  let personaIndex = 0;

  const globalStartTime = Date.now();

  try {
    while (completedCount < targetCompletions && personaIndex < maxPersonas) {
      const persona = personas[personaIndex];
      const personaName = persona.name;
      const personaDir = path.join(evidenceRoot, `persona_${personaIndex}`);
      fs.mkdirSync(personaDir, { recursive: true });

      console.log(`\n============================================================================`);
      console.log(`[Persona ${personaIndex}] Starting run: ${personaName}`);
      console.log(`============================================================================`);

      const workspaceDir = path.join(personaDir, 'workspace');
      fs.mkdirSync(workspaceDir, { recursive: true });

      // Start the local web service backend for this persona
      const serverLogPath = path.join(personaDir, 'server.log');
      const serverLogStream = fs.createWriteStream(serverLogPath);

      const serverEnv = {
        ...process.env,
        TURINGOS_WEB_WORKSPACE: workspaceDir,
        TURINGOS_BACKEND_OVERRIDE: turingosBin
      };

      const serverProc = spawn(turingosWebBin, [], {
        env: serverEnv
      });

      serverProc.stdout.on('data', (data) => {
        serverLogStream.write(redact(data.toString()));
      });
      serverProc.stderr.on('data', (data) => {
        serverLogStream.write(redact(data.toString()));
      });

      let localSuccess = false;
      let errorMsg = '';
      let sessionId = '';
      let specCapsuleCid = '';
      let tape_presence = {};
      const transcript = [];
      const screenshotWarnings = [];
      const generation_progress = [];
      const personaStartTime = Date.now();

      const context = await browser.newContext();
      const page = await context.newPage();
      if (browserInfo.user_agent === '') {
        browserInfo.user_agent = await page.evaluate(() => navigator.userAgent).catch(() => '');
      }

      const consoleLogs = [];
      page.on('console', msg => {
        consoleLogs.push({ type: msg.type(), text: redact(msg.text()) });
      });

      const networkRequests = [];
      page.on('request', request => {
        networkRequests.push({ url: redact(request.url()), method: request.method() });
      });

      const networkResponses = [];
      page.on('response', response => {
        networkResponses.push({
          url: redact(response.url()),
          status: response.status(),
          statusText: response.statusText()
        });
      });

      // Monitor session ID from API calls
      page.on('request', req => {
        if (req.url().includes('/api/spec/turn') && req.method() === 'POST') {
          try {
            const body = JSON.parse(req.postData());
            if (body.session_id) {
              sessionId = body.session_id;
            }
          } catch {}
        }
      });

      try {
        // Wait for server to bind & return status
        await waitForServer('http://127.0.0.1:8080/api/welcome/status', 30000);
        console.log(`[Persona ${personaIndex}] Backend ready. Navigating...`);

        await page.goto('http://127.0.0.1:8080/welcome');

        // Capture starting state
        await safeScreenshot(page, path.join(personaDir, '01_welcome_init.png'), screenshotWarnings);

        // Step 1: Click "准备工作站"
        await page.waitForSelector('button:has-text("准备工作站")');
        await page.click('button:has-text("准备工作站")');

        // Wait for either step_llm_config or step_api_key (skipped if LLM is already configured)
        await page.waitForSelector('tos-welcome[data-state="step_llm_config"], tos-welcome[data-state="step_api_key"]', { timeout: 30000 });
        const currentState = await page.locator('tos-welcome').getAttribute('data-state');

        if (currentState === 'step_llm_config') {
          // Step 2: Click "写入 turingos.toml"
          await page.waitForSelector('button:has-text("写入")');
          await page.click('button:has-text("写入")');
          await page.waitForSelector('tos-welcome[data-state="step_api_key"]', { timeout: 30000 });
        }
        const configPath = path.join(workspaceDir, 'turingos.toml');
        if (fs.existsSync(configPath)) {
          fs.writeFileSync(
            path.join(personaDir, 'redacted_turingos.toml'),
            redact(fs.readFileSync(configPath, 'utf8'))
          );
        }

        // Step 3: API Key Entry
        await page.waitForSelector('#welcome-api-key-input');
        await page.fill('#welcome-api-key-input', deepseekApiKey);
        await page.click('button:has-text("保存密钥")');
        await page.waitForSelector('tos-welcome[data-state="step_agent_deploy"]', { timeout: 30000 });

        // Step 4: Agent Deploy
        await page.waitForSelector('button:has-text("注册 agent_001")');
        await page.click('button:has-text("注册 agent_001")');
        await page.waitForSelector('tos-welcome[data-state="step_ready"]', { timeout: 30000 });

        // Onboarding complete capture
        await safeScreenshot(page, path.join(personaDir, '02_welcome_ready.png'), screenshotWarnings);

        // Step 5: Transition to /build
        await Promise.all([
          page.waitForNavigation({ url: '**/build', timeout: 30000 }),
          page.click('button:has-text("开始")')
        ]);

        console.log(`[Persona ${personaIndex}] Onboarding complete. Started spec interview.`);
        await safeScreenshot(page, path.join(personaDir, '03_spec_start.png'), screenshotWarnings);

        // Start driven interview
        await page.waitForSelector('button[data-cta="start"]');
        await Promise.all([
          waitForSpecTurn(page),
          page.click('button[data-cta="start"]')
        ]);

        let history = [];
        let interviewDone = false;
        const interviewTimeout = Date.now() + 450000; // 7.5 mins

        while (Date.now() < interviewTimeout) {
          await page.waitForTimeout(1500);
          const state = await page.locator('tos-spec-grill').getAttribute('data-state');

          if (state === 'awaiting_user_answer') {
            const qTextLoc = page.locator('.spec-grill-question-text');
            if (await qTextLoc.isVisible()) {
              const questionText = await qTextLoc.textContent();
              console.log(`[Persona ${personaIndex}] Turn Q: ${questionText}`);

              let promptContext = questionText;
              const nudgeLoc = page.locator('.spec-grill-nudge');
              if (await nudgeLoc.isVisible()) {
                const nudgeText = await nudgeLoc.textContent();
                console.log(`[Persona ${personaIndex}] Nudge visible: ${nudgeText}`);
                promptContext += `\n(系统提示: ${nudgeText})`;
              }

              const answer = await callDeepSeekDirect(promptContext, persona.brief, history);
              console.log(`[Persona ${personaIndex}] Simulated User A: ${answer}`);
              transcript.push({
                turn: transcript.length + 1,
                question: questionText,
                answer
              });

              // Save a screenshot in the middle of interview once
              if (history.length === 2) {
                await safeScreenshot(page, path.join(personaDir, '04_spec_interview.png'), screenshotWarnings);
              }

              history.push({ role: 'user', content: questionText });
              history.push({ role: 'assistant', content: answer });

              await page.fill('textarea[name="driven-answer"]', answer);
              await Promise.all([
                waitForSpecTurn(page),
                page.click('button.spec-grill-advance')
              ]);
            }
          } else if (state === 'playback_review') {
            const playbackLoc = page.locator('.spec-grill-playback-text');
            if (await playbackLoc.isVisible()) {
              const playbackText = await playbackLoc.textContent();
              console.log(`[Persona ${personaIndex}] Interview playback received: ${playbackText.substring(0, 100)}...`);

              await safeScreenshot(page, path.join(personaDir, '05_playback_review.png'), screenshotWarnings);
              await Promise.all([
                waitForSpecTurn(page, 180000),
                page.click('button[data-action="confirm-playback"]')
              ]);
            }
          } else if (state === 'complete') {
            // Get capsule CID from spec result if available
            try {
              const cidLoc = page.locator('.spec-result-cid code');
              if (await cidLoc.isVisible()) {
                const rawCid = await cidLoc.textContent();
                specCapsuleCid = rawCid.replace('cid:', '').trim();
              }
            } catch {}
            console.log(`[Persona ${personaIndex}] Spec synthesis finished. CID: ${specCapsuleCid}`);
            interviewDone = true;
            break;
          }
        }

        if (!interviewDone) {
          throw new Error("Spec interview timed out or did not complete correctly.");
        }

        // Code Generation
        await page.waitForSelector('tos-spec-result');
        await safeScreenshot(page, path.join(personaDir, '06_gen_start.png'), screenshotWarnings);

        console.log(`[Persona ${personaIndex}] Launching code generation...`);
        await page.click('.spec-result-generate-btn');

        let genState = 'idle';
        const genTimeout = Date.now() + generateTimeoutMs;
        let lastSeenAttempt = '';
        let lastSeenNote = '';
        while (Date.now() < genTimeout) {
          genState = await page.locator('tos-spec-result').getAttribute('data-state');

          if (genState === 'generating') {
            const counterLoc = page.locator('.spec-result-progress-counter');
            let attemptText = '';
            if (await counterLoc.isVisible()) {
              attemptText = (await counterLoc.textContent()) || '';
            }

            const noteLoc = page.locator('.spec-result-progress-note');
            let noteText = '';
            if (await noteLoc.isVisible()) {
              noteText = (await noteLoc.textContent()) || '';
            }

            if (attemptText !== lastSeenAttempt || noteText !== lastSeenNote) {
              const redactedNote = redact(noteText);
              const progressEntry = {
                timestamp: new Date().toISOString(),
                state: genState,
                attempt: attemptText,
                note: redactedNote
              };
              console.log(`[Persona ${personaIndex}] Gen progress update: attempt=${attemptText}, note=${redactedNote}`);
              generation_progress.push(progressEntry);
              lastSeenAttempt = attemptText;
              lastSeenNote = redactedNote;
            }
          } else if (genState === 'generated' || genState === 'error') {
            const progressEntry = {
              timestamp: new Date().toISOString(),
              state: genState,
              attempt: lastSeenAttempt,
              note: redact(lastSeenNote)
            };
            if (genState === 'error') {
              const errLoc = page.locator('.spec-result-error');
              if (await errLoc.isVisible()) {
                progressEntry.error = redact(await errLoc.textContent());
              }
            }
            generation_progress.push(progressEntry);
            break;
          }
          await page.waitForTimeout(2000);
        }

        if (genState !== 'generated' && genState !== 'error') {
          const redactedNote = redact(lastSeenNote);
          const progressEntry = {
            timestamp: new Date().toISOString(),
            state: 'timeout',
            genState: genState,
            currentGenState: genState,
            timeout_ms: generateTimeoutMs,
            attempt: lastSeenAttempt,
            note: redactedNote,
            last_observed_attempt: lastSeenAttempt,
            last_observed_note: redactedNote,
            lastObservedAttempt: lastSeenAttempt,
            lastObservedNote: redactedNote
          };
          generation_progress.push(progressEntry);
        }

        await safeScreenshot(page, path.join(personaDir, '07_gen_complete.png'), screenshotWarnings);

        if (genState === 'generated') {
          console.log(`[Persona ${personaIndex}] Code generation succeeded!`);
          localSuccess = true;
          completedCount++;
        } else {
          const errorText = await page.locator('.spec-result-error').textContent().catch(() => '');
          throw new Error(`Generation failed with state: ${genState}. Error: ${redact(errorText)}`);
        }
      } catch (err) {
        console.error(`[Persona ${personaIndex}] Error encountered:`, err);
        localSuccess = false;
        errorMsg = redact(err.message || String(err));
      } finally {
        // Collect tape flags
        if (sessionId) {
          tape_presence = checkTapePresence(workspaceDir, sessionId);
        } else {
          tape_presence = {
            spec_md: false,
            artifacts_count: 0,
            chaintape_l4: false,
            rejections_jsonl: false,
            cas_dir: false
          };
        }

        // Write persona manifest
        const manifest = {
          persona,
          success: localSuccess,
          session_id: sessionId,
          spec_capsule_cid: specCapsuleCid,
          duration_ms: Date.now() - personaStartTime,
          error: errorMsg ? redact(errorMsg) : null,
          tape_presence,
          generation_progress,
          screenshots: [
            '01_welcome_init.png',
            '02_welcome_ready.png',
            '03_spec_start.png',
            '04_spec_interview.png',
            '05_playback_review.png',
            '06_gen_start.png',
            '07_gen_complete.png'
          ],
          screenshot_warnings: screenshotWarnings,
          console_logs: consoleLogs,
          network_requests: networkRequests,
          network_responses: networkResponses
        };
        fs.writeFileSync(path.join(personaDir, 'manifest.json'), JSON.stringify(manifest, null, 2));
        fs.writeFileSync(
          path.join(personaDir, 'transcript.json'),
          JSON.stringify({ persona, session_id: sessionId, transcript }, null, 2)
        );
        fs.writeFileSync(
          path.join(personaDir, 'transcript.md'),
          [
            `# Persona ${personaIndex}: ${personaName}`,
            '',
            persona.brief,
            '',
            ...transcript.flatMap((t) => [
              `## Turn ${t.turn}`,
              '',
              `Question: ${redact(t.question)}`,
              '',
              `Answer: ${redact(t.answer)}`,
              ''
            ])
          ].join('\n')
        );

        personaResults.push({
          index: personaIndex,
          name: personaName,
          success: localSuccess,
          session_id: sessionId,
          spec_capsule_cid: specCapsuleCid,
          duration_ms: Date.now() - personaStartTime,
          tape_presence
        });

        // Clean up page & context
        await page.close();
        await context.close();

        // Stop server process cleanly
        serverProc.kill('SIGTERM');
        await new Promise((r) => setTimeout(r, 1000));
        if (serverProc.exitCode === null) {
          serverProc.kill('SIGKILL');
        }
        serverLogStream.end();
      }

      personaIndex++;
    }
  } finally {
    await browser.close();
  }

  const globalDuration = Date.now() - globalStartTime;

  // Write metrics.json
  const metrics = {
    agent: "agy",
    task_id: "OBL001_SCRIPT_MINIMAL",
    workspace: "/tmp/turingosv4-gaia-runner-next",
    ok: completedCount >= targetCompletions,
    status: completedCount >= targetCompletions ? "complete" : "failed",
    summary: `Completed ${completedCount}/${targetCompletions} personas.`,
    timestamp: new Date().toISOString(),
    timeout_ms: generateTimeoutMs,
    target_completed: targetCompletions,
    max_personas: maxPersonas,
    browser: browserInfo,
    global_duration_ms: globalDuration,
    completed_personas_count: completedCount,
    total_personas_attempted: personaIndex,
    personas: personaResults
  };
  fs.writeFileSync(path.join(evidenceRoot, 'metrics.json'), JSON.stringify(metrics, null, 2));

  // Write summary.md
  let summaryMd = `# OBL-001 DeepSeek Chrome E2E Verification Report

- **Date**: ${new Date().toISOString()}
- **Completed**: ${completedCount} / ${targetCompletions}
- **Status**: ${completedCount >= targetCompletions ? 'PASS' : 'FAIL'}
- **Total Duration**: ${(globalDuration / 1000).toFixed(1)} seconds
- **Chrome**: ${browserInfo.chrome_version}
- **User Agent**: ${redact(browserInfo.user_agent)}
- **Command form**: \`NODE_PATH=/tmp/obl001-playwright/node_modules xvfb-run -a node scripts/obl001_deepseek_chrome_e2e.mjs\`
- **OBL-001 status impact**: candidate evidence only; ledger closure requires post-run audit.

## Persona Verification Matrix

| Index | Persona Name | Success | Session ID | Spec Capsule CID | Artifacts | L4 Tape | Rejections | CAS |
|---|---|---|---|---|---|---|---|---|
`;

  for (const r of personaResults) {
    summaryMd += `| ${r.index} | ${r.name} | ${r.success ? '✓' : '✗'} | \`${r.session_id || 'N/A'}\` | \`${r.spec_capsule_cid || 'N/A'}\` | ${r.tape_presence.artifacts_count} | ${r.tape_presence.chaintape_l4 ? '✓' : '✗'} | ${r.tape_presence.rejections_jsonl ? '✓' : '✗'} | ${r.tape_presence.cas_dir ? '✓' : '✗'} |\n`;
  }
  fs.writeFileSync(path.join(evidenceRoot, 'summary.md'), summaryMd);

  // Perform Redaction Audit
  console.log("\n[info] Starting Redaction Audit...");
  const textFiles = findTextFiles(evidenceRoot);
  const secretsToAudit = [];
  for (const [key, value] of Object.entries(process.env)) {
    if (value && (key.includes('KEY') || key.includes('SECRET') || key.includes('TOKEN') || value.startsWith('sk-') || value.startsWith('hf_'))) {
      if (value.length >= 6) {
        secretsToAudit.push(value);
      }
    }
  }

  const findings = [];
  for (const file of textFiles) {
    // Skip auditing redaction_audit.json itself
    if (file.endsWith('redaction_audit.json')) continue;

    const content = fs.readFileSync(file, 'utf8');
    for (const secret of secretsToAudit) {
      if (content.includes(secret)) {
        findings.push({ file: path.relative(evidenceRoot, file), type: 'exact_secret_value' });
      }
    }

    const skMatch = content.match(/sk-[a-zA-Z0-9._-]+/g);
    if (skMatch) {
      // Exclude strings that look like generic placeholders or already redacted tags
      const realLeaks = skMatch.filter(m => m !== 'sk-...' && m !== 'sk-xxxx');
      if (realLeaks.length > 0) {
        findings.push({ file: path.relative(evidenceRoot, file), type: 'regex_sk', matches: realLeaks });
      }
    }

    const hfMatch = content.match(/hf_[a-zA-Z0-9_-]+/g);
    if (hfMatch) {
      const realLeaks = hfMatch.filter(m => m !== 'hf-...');
      if (realLeaks.length > 0) {
        findings.push({ file: path.relative(evidenceRoot, file), type: 'regex_hf', matches: realLeaks });
      }
    }
  }

  const auditResult = {
    audited_files: textFiles.map(f => path.relative(evidenceRoot, f)),
    secrets_found: findings.length > 0,
    findings: findings
  };

  fs.writeFileSync(path.join(evidenceRoot, 'redaction_audit.json'), JSON.stringify(auditResult, null, 2));

  console.log(`[info] Redaction audit complete. Secrets found: ${findings.length > 0}`);
  if (findings.length > 0) {
    console.error("[ERROR] Secrets detected in evidence files:", JSON.stringify(findings, null, 2));
    process.exit(1);
  }

  console.log("[info] Verification complete.");
  const finalJson = {
    agent: "agy",
    task_id: "OBL001_SCRIPT_MINIMAL",
    workspace: "/tmp/turingosv4-gaia-runner-next",
    ok: completedCount >= targetCompletions,
    status: completedCount >= targetCompletions ? "complete" : "failed",
    summary: `Successfully completed ${completedCount} personas.`,
    changed_files: ["scripts/obl001_deepseek_chrome_e2e.mjs"],
    artifacts: []
  };
  console.log("\nFINAL_JSON_OUTPUT_MARKER:" + JSON.stringify(finalJson));
  if (completedCount < targetCompletions) {
    process.exit(1);
  }
}

function findTextFiles(dir) {
  let files = [];
  const list = fs.readdirSync(dir);
  for (const file of list) {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    if (stat.isDirectory()) {
      files = files.concat(findTextFiles(filePath));
    } else if (file.endsWith('.log') || file.endsWith('.json') || file.endsWith('.md') || file.endsWith('.jsonl')) {
      files.push(filePath);
    }
  }
  return files;
}

run().catch((err) => {
  console.error("Unhandled execution error:", err);
  process.exit(1);
});
