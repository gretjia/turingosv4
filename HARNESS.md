# TuringOS Unified Harness

## 0. Purpose

The harness exists to make the constitution executable while giving LLM agents
room to reason.

It is not a checklist stack and not a prompt wall. It is a self-hosting
development cognition system:

```text
Human intent
-> Meta AI / Cortex
-> Module / Molecule / Atom contract
-> Codex / Claude / fast executor
-> turingos_dev evidence entry
-> DevEvidence hash chain + Tape/CAS witnesses
-> clean-context Codex review
-> harness evolution notes
```

Supreme truth remains:

```text
constitution.md
> canonical flowcharts and hashes
> ChainTape + CAS
> executable gates and replay/audit verifiers
> alignment matrices
> handover/directives
> reports/stdout/human-readable dashboards
```

Reports are views. Stdout is supporting evidence. A claim that cannot be
reconstructed from tape/CAS/gates is not a TuringOS validity claim.

## 1. Shared Agent Contract

`AGENTS.md` is the short cross-agent router for Codex, Claude Code, and future
fast executors. `CLAUDE.md` imports `@AGENTS.md` and then adds Claude-specific
operating detail. Large architecture and history stay in `constitution.md`,
`handover/alignment/*`, and `handover/ai-direct/LATEST.md`.

`HARNESS_MANUAL.md` is the operational runbook. Any future agent that needs to
execute a new task should read it after `AGENTS.md` and before opening a
self-hosting dev run.

Default audit path is one clean-context Codex review after implementation
evidence exists. Gemini is not part of the default harness unless a future user
message or directive explicitly asks for it.

Veto-AI is not a code reviewer. Per `constitution.md` Art. V.1.3, Veto-AI only
checks constitutionality and outputs `{PASS, VETO}`. Ordinary engineering
review is done by an independent clean-context reviewer using
`PROCEED | CHALLENGE | VETO`.

## 2. Cortex, Modules, Molecules, Atoms

The Meta AI / Cortex layer translates human intent into an executable contract:

- `module`: long-lived capability line, such as G3 observability, G4 model
  assignment, or PromptCapsule runtime wire-up
- `molecule_or_atom`: execution unit
- `risk_class`: Class 0-4
- `fc_nodes`: touched FC1/FC2/FC3 nodes or invariants
- `allowed_paths`: write surface
- `acceptance_commands`: evidence commands
- `audit_required`: derived from risk and blast radius

Use `Molecule` as the default unit for low/medium-risk related work. Molecules
share context, tests, and review so the system can move without burning tokens
on needless atom ceremony.

Use `Atom` for Class 3/4 and irreversible/high-blast-radius surfaces:
auth, money, CAS integrity, production evidence, audit_tape, constitution,
flowcharts, sequencer admission, typed transaction schema, canonical signing
payloads, RootBox/kernel authority, or trust-root authority.

Risk controls are not delegated to Cortex alone. Cortex may suggest, compress,
or increase caution; static hard-locks set the risk floor.

## 3. Blast-Radius Hard-Lock

Any diff or allowed path touching these surfaces is Class 3/4 candidate until
proved otherwise:

- `src/state/sequencer.rs`
- `src/state/typed_tx.rs`
- `src/kernel.rs`
- `src/bus.rs`
- `src/sdk/tools/wallet.rs`
- `src/bottom_white/cas/schema.rs`
- canonical signing payload/system key surfaces
- ChainTape transition ledger authority surfaces
- `genesis_payload.toml`
- `constitution.md`
- canonical flowchart/hash authority documents

Class 4 requires explicit per-atom section-8 architect/user ratification before
implementation or ship. One-word approvals are not ratification.

## 4. Self-Hosting Shadow Mode

The first development entry is thin:

```bash
turingos_dev open --title <title> --module <module> --risk <0-4> --fc <nodes> --allowed <paths>
turingos_dev record-diff --run <run_id>
turingos_dev record-command --run <run_id> -- <command...>
turingos_dev record-audit --run <run_id> --reviewer clean-context-codex --verdict PROCEED|CHALLENGE|VETO --file <audit.md>
turingos_dev validate --run <run_id>
turingos_dev close --run <run_id>
turingos_dev summarize --run <run_id>
```

`turingos_dev` records development evidence; it does not make TuringOS an
autonomous developer yet and it does not create a second canonical tape. v1 is
a DevEvidence hash-chain sidecar under:

```text
handover/evidence/dev_self_hosting/<run_id>/
  DevTaskManifest.json
  FCWitnessManifest.json
  events.jsonl
  events_hash_chain.json
  artifacts/
  DevAuditVerdict.json
  DevRunSummary.json
```

No global latest pointer is allowed. Use `--run`, `--run-dir`, or explicit
`TURINGOS_DEV_RUN`.

`close` must fail closed if the event hash chain breaks, acceptance evidence is
missing/failing, restricted paths force a higher audit requirement, or required
audit is absent/non-PROCEED.

## 5. Evidence Intelligence

Every evidence-bearing task should answer:

- What human intent produced this work?
- Which module/molecule/atom did it belong to?
- Which risk class and FC nodes were touched?
- What diff was produced?
- Which commands actually ran, with stdout/stderr/exit code?
- Which clean-context review judged it?
- Can the evidence package be replayed or at least integrity-checked?

Development evidence uses an append-only JSONL hash chain. This is not
"Shadow CAS"; it is a sidecar that may later be anchored into real ChainTape/CAS
after G3.2, G4.2, and PromptCapsule runtime wire-up are closed.

Training use is opt-in only. Raw prompts, chain-of-thought, private diagnostics,
raw stderr, and hidden fields must not become default SFT/RLAIF corpus material.
Any corpus export must be redacted, audit-approved, and compatible with
selective shielding.

## 6. Executable Substrate Gates

The intelligent harness still rests on hard gates:

- H1 static constitution gates:
  `bash scripts/run_constitution_gates.sh`
- H2 parser/manifest gates:
  `genesis_report`, `PromptCapsule`, `AttemptTelemetry`, `LeanResult`,
  `EvidenceCapsule`, `MarkovEvidenceCapsule`, `HEAD_t`, `BenchmarkManifest`
- H3 real-run gates:
  P38, P49, M0 mini-batch, attempt-count equality
- H4 replay gates:
  replay from `genesis_report` + ChainTape + CAS + agent registry + pubkeys,
  dashboard regeneration, economic state reconstruction, `HEAD_t`
  reconstruction
- H5 audit gates:
  clean-context Codex review after H1-H4 evidence exists

Persistent constitution test families:

- FC1 runtime:
  `fc1_every_externalized_attempt_is_tape_visible`,
  `fc1_predicate_pass_goes_l4`, `fc1_predicate_fail_goes_l4e`,
  `fc1_no_legacy_authoritative_append`, `fc1_attempt_count_equals_tape_count`,
  `fc1_no_fake_accepted_nodes`, `fc1_dashboard_not_source_of_truth`
- FC2 boot:
  `fc2_genesis_report_exists`, `fc2_run_replayable_from_genesis_tape_cas`,
  `fc2_no_memory_only_preseed`, `fc2_on_init_only_mint`,
  `fc2_taskopen_escrowlock_are_chain_events`, `fc2_system_pubkeys_verify`,
  `fc2_agent_registry_resolves`
- FC3 meta:
  `fc3_capsule_derived_from_tape_cas`, `fc3_no_global_markov_pointer`,
  `fc3_latest_capsule_context_only`, `fc3_deep_history_requires_override`,
  `fc3_raw_logs_not_in_agent_read_view`, `fc3_no_automatic_predicate_mutation`
- Predicate:
  `predicate_result_is_binary`, `predicate_failure_cannot_enter_l4`,
  `predicate_pass_required_for_l4`, `lean_verified_required_for_verified_worktx`,
  `price_never_overrides_predicate`
- Shielding:
  `raw_lean_stderr_not_in_agent_read_view`,
  `private_diagnostic_cid_not_serialized_publicly`,
  `evidence_capsule_raw_logs_audit_only`,
  `prompt_capsule_redacts_hidden_fields`,
  `dashboard_does_not_leak_private_failure_detail`
- Economy:
  `economy_read_is_free`, `economy_write_requires_stake_or_escrow`,
  `economy_no_post_init_mint`, `economy_total_coin_conserved`,
  `economy_complete_set_yes_no_not_coin`, `economy_no_ghost_liquidity`,
  `economy_wallet_read_only_projection`, `economy_no_f64_money_path`,
  `system_tx_not_agent_submittable`
- Tape canonical:
  `no_parallel_ledger_source_of_truth`, `no_shadow_tape_authoritative_parent`,
  `canonical_txid_not_shadow_id`, `dashboard_regenerates_from_tape_cas`,
  `chain_derived_facts_not_evaluator_stdout`,
  `all_externalized_attempts_have_cas_payload`,
  `all_lean_results_have_cas_payload`

## 7. Runner Policy

Before any runner that writes evidence or evaluates real problems, invoke
`/runner-preflight` when available or perform its checklist:

1. clean/understood tree
2. fresh binaries vs source/HEAD
3. evidence immutability
4. risk class
5. FC trace
6. charter/directive completeness
7. audit-round state

Never run a large benchmark before P38/P49 equality, M0, constitution gates,
HEAD_t, PromptCapsule, and PCP synthetic corpus are green for the relevant
surface.

## 8. Kill Gates

Stop immediately on:

- attempt equality mismatch
- fake accepted node
- predicate failure in L4
- Lean reject only in stdout
- dashboard requiring stdout for core facts
- memory-only preseed
- global Markov pointer
- CTF conservation failure
- `f64` money path
- system tx accepted from agent ingress
- shadow/canonical ID confusion
- broken DevEvidence hash chain on a self-hosting run

## 9. Done Definition

A task is done only when the risk-appropriate gates pass, the diff is reviewed
against touched FC nodes, evidence is linked, and high-risk/ship-path work has
clean-context Codex review. Dynamic handover is updated only when dynamic state
actually changes.
