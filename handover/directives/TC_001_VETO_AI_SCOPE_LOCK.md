# TC-001 Veto-AI Scope Lock

Status: active directive for the TuringOS-TC operationalization branch.

Risk class: Class 0 for this scope document. Any later atom touching restricted
surfaces remains governed by `AGENTS.md` risk classification and section-8
ratification rules.

FC mapping:
- FC3: Veto-AI checks whether an ArchitectAI proposal violates the constitution.
- Art. V.1.3: Veto-AI is not a general code reviewer.

## Locked Output Domain

Veto-AI output domain: `{PASS,VETO}`.

`PASS` means the reviewed proposal is not found to violate the constitution
within the reviewed scope. `VETO` means the proposal violates the constitution
or cannot be reconstructed against the required trust-root evidence.

## Explicit Non-Domain

Veto-AI does not review code style.

Veto-AI does not review performance.

Veto-AI does not review coverage.

Veto-AI does not review architecture taste, naming aesthetics, formatting
preference, or speculative future extensibility. Those are separate engineering
audit roles, not constitutional veto authority.

## TC Audit Roles

TuringOS-TC still uses independent audit roles, but they must not be collapsed
into Veto-AI:

- Constitution Auditor: checks documented constitutional clauses and emits the
  repository witness domain.
- Karpathy Architect Auditor: checks simplicity, data shape, and unnecessary
  abstraction.
- Data-Integrity/Reliability/Formal-Methods Auditors: check the relevant
  implementation surface.
- Obligation witness: emits the obligation-ledger verdict.

## Ship Gates For TC-001

- This file exists and locks the Veto-AI output domain.
- The non-domain list remains explicit.
- Any high-risk TC implementation atom names its audit role separately from
  Veto-AI.
- No implementation code is certified by this document alone.
