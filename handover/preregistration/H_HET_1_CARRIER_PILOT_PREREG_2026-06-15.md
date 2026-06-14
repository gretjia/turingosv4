# H-HET-1 Carrier Pilot Preregistration (2026-06-15)

**Scope:** DIRECTIONAL PILOT, autonomous under architect 2026-06-15 auto-research
mandate. **NOT** the full claimable experiment (that needs ≥N preregistered
seeds for §17-G4 power, the Art-0.2 §8 close so model attribution rests on the
tape not the sidecar, and explicit architect sign-off). Written + frozen BEFORE
the run (Gate H anti-p-hacking substance).

## Hypothesis
At equal budget, a HETEROGENEOUS cross-lab autonomous proof-market solves
det-family Lean theorems that a HOMOGENEOUS DeepSeek-only market cannot — and
does so beyond what the single strongest model alone (Q397-homo) achieves
(i.e. the lever is heterogeneity/decorrelation, not merely "one strong model in
the roster"). Primary expected observation: HET omega-solves the Goldilocks
targets (DS-homo fails them); HET ≥ Q397-homo on solved-count.

## Theorem Set
Det-family from `tests/fixtures/lean_theorems_pool.jsonl`
(sha256 6a1c888fd8c55e9f5dfe5ae9549187d5bceab336d46eca315bc706c56c32e001):
- Goldilocks (DS fails solo @ probe, cross-lab solves): `lm_det_mul`,
  `lm_det_2x2`, `lm_det_zero`
- DS-solves controls (DS 1/3 @ probe — sanity that DS-homo CAN solve some):
  `lm_det_3x3`, `lm_geom_eval`
All `needs_mathlib=true`, difficulty 15. Verification = real Lean 4 kernel +
`#print axioms` ⊆ {Classical.choice, Quot.sound, propext}.

## Goldilocks Difficulty Band
Selected from already-paid probe evidence (calib sweep 134 recs + K=3 pilot 84
recs, 2026-06-14), analyzed 2026-06-15: per-(theorem,model) verified counts
under the SAME non-thinking @2048 regime. Goldilocks = DS 0/K AND ≥1 cross-lab
model ≥1/K, all Verifieds axiom-clean. lm_det_mul (DS 0/3, Q32 1/3, Q397 3/3),
lm_det_2x2 (DS 0/3, Q397 1/3), lm_det_zero (DS 0/3, GLM 1/3). 12 harder theorems
were all-null (excluded — out of band non-thinking). Carrier smoke 2026-06-15
reproduced lm_det_mul solve via Q397 (omega, axiom-clean), DS=SorryBlocked.

## Baseline / Arms (equal budget NA=4, NR=3 → 12 proposals/cell)
- **HET** — `--models deepseek-ai/DeepSeek-V4-Pro,Qwen/Qwen3-32B,zai-org/GLM-4.5-Air,Qwen/Qwen3.5-397B-A17B`
  (round-robin → 4 agents = 4 distinct vendors).
- **DSHOMO** — `--models deepseek-ai/DeepSeek-V4-Pro` (4 agents all DS; the
  "homogeneous can't-solve" baseline).
- **Q397HOMO** — `--models Qwen/Qwen3.5-397B-A17B` (4 agents all Q397;
  best-single-model control to isolate heterogeneity from model strength).
All `--policy autonomous_market` (Hayekian self-selection solve|short),
non-thinking @ max_tokens 2048 (门2 regime), proxy http://localhost:8123.

## Agent / Model Roster
1. deepseek-ai/DeepSeek-V4-Pro — non-thinking (enable_thinking:false), temp per binary consts
2. Qwen/Qwen3-32B — non-thinking
3. zai-org/GLM-4.5-Air — non-thinking
4. Qwen/Qwen3.5-397B-A17B — non-thinking

## Budget Cap
Pilot scale: 5 theorems × 3 arms × 3 seeds = 45 cells × ≤12 proposals + route +
bear ≈ ≤ ~700 model calls. At non-thinking @2048, ≈ <$5 worst case (smoke cell
= 5721 tokens ≈ <$0.02). No per-vendor wallet cap relied on for the science
(directional). Hard stop if total wall-clock > 3h or any arm errors > 50%.

## Stopping Rule
Per cell: first omega (verified) OR budget exhausted (NA×NR proposals). Run:
all 45 cells complete OR 3h timeout. Resumable (cells with manifest +
replay-clean report are skipped — 断点续做).

## Primary Metric
Per-(theorem, arm): omega-solve rate over 3 seeds (0/3..3/3), real-Lean +
axiom-clean. H-HET-1 directional-confirmed if HET solves ≥1 Goldilocks target
that DSHOMO solves 0/3, AND HET solved-count ≥ Q397HOMO solved-count.

## Secondary Metrics
- which vendor produced each omega (via round-robin sidecar attribution —
  caveat: NOT tape-canonical until Art-0.2 §8 close).
- tokens / wall-clock per solve; parse_fail / action_source mix (self-selection honesty).
- short-action incidence (does any agent self-select short on a live node?).

## Exclusion Rule
A cell is excluded if `verify_chaintape` economic_state_reconstructed != true
(replay-recompute gate), or carrier exit != 0, or proxy_unreachable. Excluded
cells are reported, not silently dropped.

## Commit Hash (Frozen)
Carrier source = f73163f44b3e56282d1c8f82bb65fed808581daa (branch
claude/het-carrier-freeze). NOTE: working tree is dirty (159 other-session
files); binary built 2026-06-15 from committed carrier + dirty lib — acceptable
for a DIRECTIONAL pilot, documented; the full experiment requires a clean build.

## Tape / WAL Hash
bank sha256 6a1c888fd8c55e9f5dfe5ae9549187d5bceab336d46eca315bc706c56c32e001;
seeds {1,2,3} pin RNG (not LLM sampling). Each cell = fresh repo/cas; every
counted cell replay-verified.

---
**Architect sign-off (Gate H, full experiment):** PENDING — this pilot runs
under the standing 2026-06-15 autonomous-research mandate; the full claimable
experiment is separately sign-off-gated.
**Authored:** 2026-06-15 (autonomous)
