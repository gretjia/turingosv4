# Phase 7 §6a E2E driver

Scripts for setting up and tearing down a deterministic TuringOS workspace
so the §6a Chrome-driven verifier can exercise a real write path.

---

## Setup contract

`frontend_e2e_setup_workspace.sh [WORKSPACE_DIR]`

**Default workspace**: `tmp/phase7_workspace` (relative to repo root → absolute).

**What it does** (in order):

1. Verifies repo root (Cargo.toml present).
2. Builds `turingos` CLI if source is newer than binary.
3. Builds `turingos_web` (with `--features web`) if source is newer.
4. Builds `frontend/dist/main.js` via `npm run build` if TypeScript sources are newer.
5. Runs `turingos init --project <DIR> --template multi-agent` — idempotent via `--force`.
6. Runs `turingos agent deploy --workspace <DIR> --id agent_001 --pubkey <64hex> --role Solver`.
7. Prints two `export` lines the verifier can `eval`:

```
TURINGOS_WEB_WORKSPACE=<absolute-path>
TURINGOS_WEB_BIN=<absolute-path-to-turingos_web>
```

**Files produced under `<workspace>/` after setup:**

| Path | Description |
|---|---|
| `genesis_payload.toml` | Multi-agent arena genesis template |
| `agent_pubkeys.json` | Agent registry (contains `agent_001` entry) |
| `runtime_repo/` | Libgit2-backed ChainTape store (created by `init`) |
| `cas/` | Content-addressed store directory (created by `init`) |

**Idempotency**: if the workspace directory already exists, the script checks for
all four scaffold files. If intact and `agent_001` is registered, it exits 0
without touching anything. If `agent_001` is missing it re-deploys. If scaffold
files are missing it exits 1 with a diagnostic.

**Env vars exported:**

| Variable | Value |
|---|---|
| `TURINGOS_WEB_WORKSPACE` | Absolute path to the workspace directory |
| `TURINGOS_WEB_BIN` | Absolute path to `target/debug/turingos_web` |

---

## Teardown contract

`frontend_e2e_teardown_workspace.sh [WORKSPACE_DIR]`

**Default**: same `tmp/phase7_workspace` default.

**Safety guards:**

- Uses `realpath` to canonicalize the path before any deletion.
- Refuses any path **not** under `<repo>/tmp/` or `/tmp/`.  
  Exit 1: `[teardown] REFUSED: '<path>' is not under <repo>/tmp/ or /tmp/`
- Refuses the tmp root itself (`/tmp` or `<repo>/tmp` alone) — must be a subdirectory.
- If the directory does not exist: prints `[teardown] nothing to remove` and exits 0.
- On success: `rm -rf <DIR>` then prints `[teardown] removed <DIR>`.

---

## Verifier orchestration

Recommended call sequence for the §6a autonomous Chrome-driven verifier:

```bash
# 1. Setup
eval "$(bash tests/frontend_e2e_setup_workspace.sh 2>&1 | grep '^TURINGOS_')"
# — or source the exported vars directly:
bash tests/frontend_e2e_setup_workspace.sh
export TURINGOS_WEB_WORKSPACE=/absolute/path/to/workspace
export TURINGOS_WEB_BIN=/absolute/path/to/target/debug/turingos_web

# 2. Start the web server (background)
"${TURINGOS_WEB_BIN}" --workspace "${TURINGOS_WEB_WORKSPACE}" &
WEB_PID=$!

# 3. Drive Chrome via MCP tools
#    navigate to http://localhost:8080 ...
#    verify Page 2 DOM contains "agent_001"
#    trigger form submit → task open → ChainTape advances

# 4. Post-session: audit tape (optional cross-check)
target/debug/turingos report wallet --chaintape "${TURINGOS_WEB_WORKSPACE}"
target/debug/turingos replay        --chaintape "${TURINGOS_WEB_WORKSPACE}"
target/debug/turingos audit dashboard --chaintape "${TURINGOS_WEB_WORKSPACE}"

# 5. Teardown
kill $WEB_PID
bash tests/frontend_e2e_teardown_workspace.sh
```

---

## Manual smoke

To verify the setup machinery works **without driving Chrome**:

```bash
# From repo root:
bash tests/frontend_e2e_setup_workspace.sh
#   → should print TURINGOS_WEB_WORKSPACE and TURINGOS_WEB_BIN

# Verify agent is registered:
target/debug/turingos agent view \
  --workspace tmp/phase7_workspace \
  --id agent_001
#   → should print: id=agent_001, role=Solver

# Re-run setup (idempotency check):
bash tests/frontend_e2e_setup_workspace.sh
#   → should print "[setup] workspace exists … integrity OK" and exit 0

# Run the full meta-test (7 assertions):
bash tests/frontend_e2e_setup_workspace_test.sh

# Safety guard check:
bash tests/frontend_e2e_teardown_workspace.sh /etc
#   → should exit 1 with REFUSED message

# Final teardown:
bash tests/frontend_e2e_teardown_workspace.sh
#   → should print "[teardown] removed …" and exit 0
```
