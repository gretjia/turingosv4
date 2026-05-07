# TuringOS Constitution-First Project Plan

## 0. Current judgement

The project is not ready for more feature TBs.

Current facts:

```
Constitution landing map:
  LANDED            = 28/64 = 44%
  LANDED + PARTIAL  = 41/64 = 64%
  NOT-LANDED        = 14
  BLOCKED-DECISION  = 7
  DEFERRED-FORWARD  = 2
```

Weakest chapter:

- Art. III selective shielding / prompt persistence
- 0% fully landed

Conclusion:

Constitution is not fully landed.
Feature development is frozen until critical gates are green.

***

## 1. Strategic decisions now settled

### G-009 HEAD_t

Decision:

- Path C hybrid

Work:

- C1 immediate `HEAD_t` witness
- C2 libgit2 production refs

### G-012 PCP soundness

Decision:

- Lean tactic-mutation adversarial corpus first
- MiniF2F-v2 misalignment later

### G-016 / G-019 / G-021 / G-028 prompt persistence

Decision:

- Class-3 `PromptCapsule` + L4 anchor
- Class-4 encrypted / audit-only verbatim prompt only if explicitly ratified

***

## 2. Immediate execution plan

### Day 0–1: Real test rerun

Run:

- P38
- P49
- M0 mini-batch

Goal:

```
attempt equality:
evaluator_reported_completed_llm_calls
=
  l4_work_attempt_count
+ l4e_work_attempt_count
+ capsule_anchored_attempt_count
```

If fail:

- stop and fix evaluator / `AttemptTelemetry` / L4.E routing

### Day 1–3: Wave 1 static gates

Close no-dependency shape gaps:

- `no_legacy_authoritative_append`
- `no_global_markov_pointer`
- `no_f64_money_path`
- `system_tx_not_agent_submittable`
- `dashboard_not_source_of_truth`
- `no_memory_only_preseed`
- `no_shadow_canonical_id_mix`

### Day 3–7: Wave 2 parser / manifest gates

Implement parsers and tests:

- `genesis_report`
- `PromptCapsule`
- `AttemptTelemetry`
- `LeanResult`
- `EvidenceCapsule`
- `MarkovEvidenceCapsule`
- `HEAD_t`
- `BenchmarkManifest`
- `EvidencePackagingPolicy`

### Week 2: Wave 3 load-bearing harness

Implement:

- `HEAD_t` C1 witness
- `PromptCapsule` + L4 anchor
- PCP synthetic corpus
- P38 / P49 rerun
- 20-problem diagnostic

### Week 3–4: Diagnostic benchmark

Run:

- 20 real problems
- 50 real problems if 20 passes

No public report.

### Week 5–8: Deep foundation

Implement:

- `HEAD_t` C2 libgit2
- MiniF2F-v2 misalignment corpus
- Art. III deeper prompt persistence / shielding tests

***

## 3. Resume conditions for feature TBs

Do not resume NodeMarket / PriceIndex / Polymarket / benchmark scale-up until:

- FC composite green
- Art. III >= 60% LANDED+PARTIAL and at least one LANDED
- Art. 0 >= 70% LANDED+PARTIAL
- `HEAD_t` C1 green
- PCP synthetic corpus green
- `PromptCapsule` anchored
- P38 / P49 attempt equality green
- `cargo test --workspace` 0 fail
- `scripts/run_constitution_gates.sh` 0 fail
- no unresolved critical BLOCKED-DECISION

***

## 4. MiniF2F scaling policy

Allowed now:

- P38
- P49
- M0 mini-batch
- 20-problem diagnostic

Not allowed now:

- 100+ full M2
- public benchmark report
- H-VPPU claim
- formal benchmark passed claim

Large-scale MiniF2F begins only after constitution critical gates pass.

***

## 5. TB sequence after constitution landing

After critical gates green:

- TB-18R Final
  - tape restoration and attempt equality final ship
- TB-18B
  - formal benchmark scale-up M1 / M2
- TB-19
  - low-risk real-world pilot design, documents only
- TB-20
  - low-risk real-world sandbox pilot, no irreversible action
- TB-21
  - limited beta with delayed settlement and human escalation

NodeMarket / Polymarket features resume only when:

- attempt denominator fixed
- PCP soundness green
- prompt shielding green
- `HEAD_t` green

***

## 6. Project management reset

Stop:

- cosmetic wave progress
- audit-before-evidence
- one-word sign-off
- ship report before audit
- feature TBs while constitution blockers red

Start:

- real-test-first harness
- strategic blockers first
- constitution matrix as primary board
- short diagnostic loops
- hard kill gates

***

## 7. Weekly operating cadence

### Monday

- update Constitution Execution Matrix
- choose 1–2 load-bearing blockers
- run targeted real tests

### Daily

- run constitution gates
- run current diagnostic problem set
- fix only what failed

### End of week

- publish evidence capsule
- update landing map
- decide whether feature freeze continues

***

## 8. Exit condition for Constitution Landing Reset

TuringOS exits the reset when it can answer from ChainTape + CAS alone:

- What did the Agent externalize?
- What did it see?
- What passed predicates?
- What failed predicates?
- What is on L4?
- What is on L4.E?
- What is only CAS evidence?
- What did dashboard derive?
- What moved money?
- What remained shielded?

If any answer requires evaluator stdout as primary source, constitution is not landed.
