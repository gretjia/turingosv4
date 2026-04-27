# System Keypair Security Spec v1

> **Date**: 2026-04-27
> **Purpose**: Gemini v3.2 review Q9 VETO — runtime system keypair (signs `RejectedAttemptSummary` + `TerminalSummaryTx`) lifecycle was unspecified; this doc defines generation, storage, rotation, threat model, and audit gates.
> **Authority**: Constitution Art. 0–0.4 (Tape Canonical) + Art. V.1 (separation of powers).
> **Audit**: Gemini v3.2 cross-review (2026-04-27) marked Plan v3.2 VETO until this exists.

---

## § 1 Why a system keypair exists

D-VETO-6 / Codex CHALLENGE: failure metadata cannot be agent self-reported (wrong trust boundary). The runtime's white-box predicate runner stamps `RejectedAttemptSummary` onto next accepted `WorkTx`, and emits `TerminalSummaryTx` on no-accept runs. These stamps must be **cryptographically bound to the runtime instance**, not forgeable by any agent.

Hence: a runtime-level `system_keypair` distinct from any agent keypair.

---

## § 2 Threat Model

### Adversary capabilities
- A1: malicious agent inside swarm — can submit work_tx / verify_tx / challenge_tx with arbitrary contents but **cannot sign as system**
- A2: external attacker without runtime access — can read public tape; cannot inject tx
- A3: external attacker with read access to runtime host filesystem — can attempt to extract keypair from disk
- A4: external attacker with full runtime host compromise — can read memory, sign arbitrary system tx until detected
- A5: malicious code path inside runtime (e.g., compromised dependency) — can call sign API arbitrarily

### What the keypair MUST defend against
- A1 forging system signatures → **defended by signature verification at append time**
- A2 reading retry metadata for Goodhart attack → **partially defended by L1 visibility filter**; system signature is orthogonal
- A3 extracting key from rest disk → **defended by encrypted-at-rest with user-derived KDF**

### What the keypair CANNOT defend against
- A4 full host compromise → **acknowledged loss**; mitigation = epoch rotation + L4 anchor proves PAST tx still valid
- A5 internal call abuse → **partially mitigated** by sign API only callable from `predicate_runner` + `terminal_summary_emitter` modules; static analysis enforces

### Recovery requirement
If A4/A5 detected: user can rotate to a new system keypair via Art V.3 amendment + signed git tag. Old key remains pinned in L4 history for replay verification but is invalidated for new tx. Future tx use new key.

---

## § 3 Keypair Lifecycle

### 3.1 Generation

**When**: at runtime first boot (post-genesis verification).

**How**:
```rust
// src/bottom_white/ledger/system_keypair.rs (NEW per CO1.7.0b)
pub fn generate_or_load_system_keypair(
    keystore_path: &Path,
    user_kdf_password: &SecretString,
) -> Result<Ed25519Keypair, KeypairError> {
    if keystore_path.exists() {
        return load_existing_keypair(keystore_path, user_kdf_password);
    }

    // First boot: generate ed25519 keypair
    let keypair = Ed25519Keypair::generate_with_secure_entropy()?;

    // Encrypt with user-derived key (Argon2id KDF on user_kdf_password)
    let encrypted = encrypt_at_rest(&keypair, user_kdf_password)?;
    fs::write(keystore_path, encrypted)?;
    set_file_permissions(keystore_path, 0o600)?;

    Ok(keypair)
}
```

**Algorithm**: ed25519 (small, fast, deterministic signatures, side-channel resistant).
**Entropy**: from `getrandom(2)` (Linux) / `SecRandomCopyBytes` (macOS); never from agent input or PRNG seeded by tape.

### 3.2 Storage

**At rest**:
- Path: `~/.turingos/keystore/system_keypair_v{epoch}.enc` (NOT in repo, NOT in any cas/ledger directory)
- Permissions: 0o600 (user read/write only)
- Encryption: ChaCha20-Poly1305 with key derived from user password via Argon2id (32-byte derived key)
- KDF parameters: Argon2id with m=64MB, t=3, p=4 (current OWASP recommendation as of 2026-04)
- Salt: stored alongside encrypted blob; randomly generated on first encryption

**In memory**:
- Loaded once at boot; held in `Arc<Ed25519Keypair>` inside `Runtime` struct
- Memory-locked via `mlock(2)` to prevent swap to disk
- Zeroized on Runtime drop (`zeroize` crate)
- NEVER serialized to ledger/CAS/log
- NEVER passed to agent code or LLM payload

### 3.3 Sign API contract

```rust
// Only callable from these two paths, enforced by `pub(restricted)` + cargo-deny
pub(restricted = predicate_runner, terminal_summary_emitter)
fn sign_system_message(
    keypair: &Ed25519Keypair,
    message: &CanonicalMessage,
) -> SystemSignature {
    keypair.sign(canonical_digest(message))
}
```

`CanonicalMessage` is a typed enum — `RejectedAttemptSummary | TerminalSummaryTx | EpochRotationProof`. No free-form message signing exposed.

### 3.4 Verification (public)

```rust
pub fn verify_system_signature(
    sig: &SystemSignature,
    message: &CanonicalMessage,
    epoch: SystemEpoch,
    pinned_pubkeys: &PinnedSystemPubkeys,
) -> bool {
    let pk = pinned_pubkeys.get(epoch).expect("epoch pubkey missing");
    pk.verify(canonical_digest(message), sig)
}
```

`pinned_pubkeys` is loaded from `genesis_payload.toml` `[system_pubkeys]` section (NEW per this spec). Each epoch has a public key entry; private key is in encrypted keystore.

### 3.5 Rotation

**Trigger conditions**:
- Suspected compromise (A4/A5)
- Scheduled rotation: every 12 months from epoch start (long enough for v4 timeline)
- Architecture amendment that changes signature algorithm

**Procedure**:
1. User runs `cargo run --bin rotate-system-keypair -- --new-epoch=<N+1>`
2. Tool generates fresh ed25519 keypair
3. Tool emits `EpochRotationProof` signed by **both** old (epoch N) and new (epoch N+1) keys, certifying continuity
4. User PGP-signs the rotation: `git tag -s v4-syskey-rotate-N-to-N+1`
5. `genesis_payload.toml` `[system_pubkeys]` updated to add new pubkey
6. AUDIT_LEDGER row added with: rotation timestamp, old/new fingerprint, rotation tag fingerprint, user signature verification

**After rotation**:
- All NEW tx signed with epoch N+1 key
- Old epoch N key retained in `[system_pubkeys]` for verifying historical L4 entries
- Old encrypted keystore moved to `~/.turingos/keystore/archive/system_keypair_v{N}.enc.archived`

### 3.6 Compromise response

If A4 host compromise detected (e.g., user notices unauthorized tx):
1. **STOP runtime immediately** (kill process, prevent further sign calls)
2. Verify via `git log` whether unauthorized commits exist; if so, trigger Art V.3 amendment
3. Run `cargo run --bin emergency-rotate-system-keypair`
4. Future runtime starts use new key
5. AUDIT_LEDGER + LATEST.md document the event
6. Constitution Art V.3 amendment if compromise window changes any L4 invariants

**Note**: post-compromise, OLD tx signed by compromised key remain VERIFIABLE (their pubkey still pinned). What's lost: confidence that those tx were emitted by legitimate runtime. User can choose to mass-mark them as "post-compromise quarantine" via amendment, but that's a policy decision, not a cryptographic one.

---

## § 4 Conformance Tests (5 new)

```
tests/system_keypair_generation.rs           — first-boot keypair generated; encrypted at rest; correct permissions
tests/system_keypair_load_and_decrypt.rs     — second-boot load with correct password succeeds; wrong password fails
tests/system_keypair_sign_only_from_runner.rs — static check: sign API not exported beyond predicate_runner + terminal_summary_emitter
tests/system_keypair_verify_correctness.rs   — round-trip: sign then verify, with correct epoch pubkey lookup
tests/system_keypair_rotation_proof.rs       — EpochRotationProof signed by BOTH old + new key; verifies continuity
```

---

## § 5 New `[system_pubkeys]` Section in genesis_payload.toml

Extend `GENESIS_MINIMAL_WITH_ANCHOR_v1` schema:

```toml
[constitution_root]
... (8 fields from genesis spec)

[system_pubkeys]
epoch_1 = "<base64 ed25519 pubkey>"
epoch_1_signed_at = "2026-04-27T..."
epoch_1_creator_pgp_sig = "<PGP sig of pubkey covering creator authorization>"
# Future epochs added here on rotation.
```

Boot extension:
```rust
pub fn verify_system_pubkeys(manifest: &GenesisPayload) -> Result<(), TrustRootError> {
    for (epoch, pk_entry) in &manifest.system_pubkeys {
        verify_pgp_signature(&pk_entry.creator_pgp_sig, &pk_entry.pubkey, &PINNED_CREATOR_PUBKEY)?;
    }
    Ok(())
}
```

---

## § 6 Interaction with B-1 PGP Tag Governance

Every system keypair rotation produces a PGP-signed git tag. This tag goes into AUDIT_LEDGER per the existing B-1 governance protocol. Net result: every system signature ever produced has a complete provenance chain:

```
runtime tx → system signature (epoch N)
            ↓
            epoch N pubkey (in genesis_payload.toml [system_pubkeys])
            ↓
            creator_pgp_sig (user authorized this epoch's pubkey)
            ↓
            user PGP key (pinned in boot.rs as PINNED_CREATOR_PUBKEY)
            ↓
            git tag v4-syskey-rotate-{from}-to-{to} (history of all rotations)
```

A complete tamper requires compromising user's PGP key, which is out of scope for runtime defense (user responsibility to protect personal cryptographic identity).

---

## § 7 Out of Scope

- **HSM (Hardware Security Module)**: v4 is software-only; v4.1+ may add HSM backing
- **Multi-party threshold signatures (FROST/MuSig)**: only relevant when multiple human architects join (currently solo); v5
- **Post-quantum signatures**: ed25519 ample for v4 horizon (5-10 years); revisit when post-quantum standards stabilize
- **TPM remote attestation**: cloud-deployment scenario; v5

---

## § 8 Honest Acknowledgements

What this spec achieves:
- Closes Gemini v3.2 Q9 VETO on undefined keypair lifecycle
- Defines threat model honestly (A1-A5)
- Acknowledges A4 host compromise as out-of-scope-for-cryptography
- Provides concrete rotation procedure + emergency response

What this spec is honest about:
- v4 keypair security is **as strong as user's password + filesystem permissions + memory hygiene** — not stronger
- Single-instance, software-only, single-user system; multi-party operations are v5
- A4 = full host compromise is a known unfixable hole; the design limits damage to "future tx forgery from compromise time forward", not retroactive

What this spec adds to v4 atom count:
- CO1.7.0c (system keypair lifecycle implementation): 5 atoms (gen + load + sign API + rotation tool + emergency rotation tool)
- 5 new conformance tests
- New `[system_pubkeys]` section in genesis_payload.toml (extends CO1.0)

— ArchitectAI, 2026-04-27
