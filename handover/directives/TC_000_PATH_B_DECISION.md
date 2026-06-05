# TC-000 Path B Constitutional Decision

Status: active directive for the TuringOS-TC operationalization branch.

Risk class: Class 0 for this decision record. Later implementation atoms that
touch ChainTape authority, CAS authority, sequencer admission, typed
transactions, signing payloads, or constitution/flowchart authority inherit the
project risk rules in `AGENTS.md`.

FC mapping:
- FC1: `Q_t -> rtool -> input -> output -> predicates -> wtool -> Q_{t+1}`.
- FC2: boot must load the trust-root bytes and reconstruct the runtime state.
- FC3: ArchitectAI/Veto-AI can evolve trust-root payloads only through bounded
  constitutional review.

## Decision

Decision: Path B.

TuringOS-TC uses the true git substrate described by constitution Art. 0.4:

- `Q_t = <q_t, HEAD_t, tape_t>`.
- `HEAD_t` is represented by explicit git refs.
- `tape_t` is reconstructed from git object content plus CAS evidence.
- `wtool` writes accepted transitions as git-backed append-only commits.
- rejected transitions remain separate from accepted state.

## Rejected Alternatives

Path A: rejected.

Reason: a semantic in-memory or custom-hash substrate cannot close the Art. 0.4
git-style `HEAD_t` requirement for TC public evidence.

Path C: rejected.

Reason: deferring true git semantics would keep TC implementation on a hybrid
substrate while downstream atoms already depend on git reconstruction.

## Locked Ref Topology

The TC head-ref contract is:

```text
accepted_l4   = refs/chaintape/l4
rejected_l4e  = refs/chaintape/l4e
cas_root      = refs/chaintape/cas
tdma_verified = refs/tdma/verified_head
tdma_tail     = refs/tdma/ledger_tail
```

Authority refs must not be updated with ignored errors. A failed authority-ref
movement must either fail the atom or enter an explicitly detected recovery
state that a gate can observe.

## Ship Gates For TC-000

- This file exists and states the Path B decision.
- Path A and Path C are explicitly rejected.
- The ref topology above is present byte-for-byte in tests.
- No implementation code is claimed complete by this document.
- Strong public claim language remains out of scope until the relevant
  replay, parity, and clean-context audit gates exist.
