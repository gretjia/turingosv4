#!/usr/bin/env python3
"""Bare-V4 single-shot control arm for the SWE-bench loop probe (NOT TuringOS).

Rebuilds the byte-identical first-round coding-repair prompt the TuringOS loop
arm uses (mirror of SWEBENCH_SYSTEM_PROMPT + make_swebench_user_prompt in
src/bin/turingos/cmd_tdma.rs), sends ONE shot to deepseek-v4-pro through the
local proxy (:8123, thinking-off — matching the loop's hardcoded thinking:None),
extracts the patch, and runs the SAME official swebench harness ONCE in HF
offline mode (matching the fixed SwebenchTestJudge). No CAS, no ChainTape, no
verify-retry: this isolates the single variable — the multi-step loop.

Usage:
  probe_bare_v4_swebench.py <sample.json> <proxy_url> <venv_python> <workdir>
"""
import sys, json, hashlib, subprocess, os, urllib.request

if len(sys.argv) != 5:
    print("usage: probe_bare_v4_swebench.py <sample.json> <proxy_url> <venv_python> <workdir>", file=sys.stderr)
    sys.exit(2)

sample_path, proxy_url, venv_python, workdir = sys.argv[1:5]
MODEL = "deepseek-v4-pro"
DATASET = "princeton-nlp/SWE-bench_Lite"
MODEL_NAME = "turingos-bare"

with open(sample_path) as f:
    s = json.load(f)

# ---- mirror of SWEBENCH_SYSTEM_PROMPT (cmd_tdma.rs:249) ----
SYS = 'You are a software engineer. Output ONLY strict JSON {"patch":"<unified git diff>","rationale":"..."}.'

# ---- mirror of make_swebench_user_prompt (cmd_tdma.rs:256-277) ----
def build_user(sample):
    ht = (sample.get("hints_text") or "").strip()
    hints = f"\n\nMaintainer hints:\n{ht}" if ht else ""
    ftp = sample.get("fail_to_pass") or []
    failing = "\n".join(ftp) if ftp else "(none listed)"
    return (
        f"Repository: {sample['repo']}\n"
        f"Base commit: {sample['base_commit']}\n\n"
        f"Problem statement:\n{sample['problem_statement']}{hints}\n\n"
        f"Target failing tests that your patch must make pass:\n{failing}\n\n"
        "Return a unified git diff patch (standard `git diff` format, file paths "
        "relative to the repository root, beginning with `diff --git`) that resolves "
        "the issue so the failing tests pass. Output ONLY the strict JSON object "
        '{"patch":"...","rationale":"..."} with the diff as the `patch` value. Do not '
        "include or quote any hidden test code, reference solution, or benchmark patch."
    )

USER = build_user(s)

# ---- single-shot model call (thinking-off, like the loop's thinking:None) ----
body = json.dumps({
    "model": MODEL,
    "temperature": 0.7,
    "max_tokens": 16000,  # match the loop; thinking reasoning counts toward completion
    "enable_thinking": True,  # match the loop's meta role (toml thinking="on")
    "messages": [{"role": "system", "content": SYS}, {"role": "user", "content": USER}],
}).encode()
req = urllib.request.Request(f"{proxy_url}/v1/chat/completions", data=body,
                            headers={"Content-Type": "application/json"})
resp = json.load(urllib.request.urlopen(req, timeout=600))
msg = (resp.get("choices") or [{}])[0].get("message", {}) or {}
content = msg.get("content") or ""
usage = resp.get("usage")

# ---- mirror of SwebenchTestJudge::extract_patch ----
def extract_patch(b):
    t = b.strip()
    obj = None
    try:
        obj = json.loads(t)
    except Exception:
        i, j = t.find("{"), t.rfind("}")
        if 0 <= i < j:
            try: obj = json.loads(t[i:j+1])
            except Exception: obj = None
    if isinstance(obj, dict):
        p = obj.get("patch") or obj.get("diff")
        if isinstance(p, str) and p.strip():
            return p.strip()
    # fenced ```diff / ``` block
    lines, collecting, buf = b.splitlines(), False, []
    for ln in lines:
        st = ln.lstrip()
        if not collecting:
            if st.startswith("```"):
                collecting, buf = True, []
        elif st.startswith("```"):
            c = "\n".join(buf).strip()
            if c.startswith("diff --git") or c.startswith("--- "):
                return c
            collecting, buf = False, []
        else:
            buf.append(ln)
    if t.startswith("diff --git") or t.startswith("--- "):
        return t
    return None

patch = extract_patch(content)

out = {
    "arm": "bare_v4_single_shot_no_turingos",
    "model": MODEL,
    "instance_id": s["instance_id"],
    "thinking": "on",
    "system_prompt_sha256": hashlib.sha256(SYS.encode()).hexdigest(),
    "user_prompt_sha256": hashlib.sha256(USER.encode()).hexdigest(),
    "user_prompt_chars": len(USER),
    "patch_present": patch is not None,
    "usage": usage,
    "resolved": None,
    "fail_to_pass_unresolved": None,
    "harness_error": None,
}

if patch is None:
    out["harness_error"] = "model output contained no unified diff"
    print(json.dumps(out, indent=2)); sys.exit(0)

# ---- write predictions + run the official harness ONCE, HF-offline ----
os.makedirs(workdir, exist_ok=True)
preds = os.path.join(workdir, "preds_bare.jsonl")
with open(preds, "w") as f:
    f.write(json.dumps({"instance_id": s["instance_id"],
                        "model_name_or_path": MODEL_NAME,
                        "model_patch": patch}) + "\n")
preds_abs = os.path.abspath(preds)
run_id = f"barev4_{s['instance_id'].replace('/', '_')}"
env = dict(os.environ)
env["HF_HUB_OFFLINE"] = "1"
env["HF_DATASETS_OFFLINE"] = "1"
cmd = [venv_python, "-m", "swebench.harness.run_evaluation",
       "--dataset_name", DATASET, "--predictions_path", preds_abs,
       "--instance_ids", s["instance_id"], "--run_id", run_id,
       "--namespace", "none", "--max_workers", "1", "--cache_level", "instance"]
proc = subprocess.run(cmd, cwd=workdir, env=env, capture_output=True, text=True, timeout=3600)

report_path = os.path.join(workdir, "logs", "run_evaluation", run_id, MODEL_NAME,
                           s["instance_id"], "report.json")
if not os.path.exists(report_path):
    out["harness_error"] = f"no report.json (exit {proc.returncode}); stderr tail: {proc.stderr[-400:]}"
    print(json.dumps(out, indent=2)); sys.exit(0)

with open(report_path) as f:
    report = json.load(f)
inst = report.get(s["instance_id"], {})
out["resolved"] = inst.get("resolved")
out["patch_successfully_applied"] = inst.get("patch_successfully_applied")
ftp = inst.get("tests_status", {}).get("FAIL_TO_PASS", {})
out["fail_to_pass_unresolved"] = ftp.get("failure")
out["fail_to_pass_success"] = ftp.get("success")
print(json.dumps(out, indent=2))
