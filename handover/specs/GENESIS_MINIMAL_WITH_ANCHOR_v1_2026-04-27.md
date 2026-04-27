# Genesis Minimal-With-Anchor Schema v1

> **Date**: 2026-04-27
> **Purpose**: D-VETO-3 (Codex CHALLENGE on hyper-minimal 5-line) revised. Minimal genesis root with content-addressed anchors so `amendment_predicate_id` resolves to actual code, not just a name.
> **Authority**: Constitution Art. 0–0.4 + WP architecture § 5.L0 + § 11.1 boot.
> **Audit**: Codex CO P0.7 T+S review (2026-04-27) demanded "minimal root must still anchor content-addressed code, not merely name it."

---

## § 1 Why Codex VETO'd the 5-line version

Original Claude proposal:
```toml
[constitution_root]
constitution_hash = "..."
creator_signature = "..."
signed_at = "..."
amendment_predicate_id = "..."
schema_version = 1
```

Codex objection:
- `amendment_predicate_id` is a **name**, not a hash, not a root, not proof of resolved code
- "What code that name resolves to" is unanchored at genesis → bootstrap circularity
- WP § 5.L0 lists `human_signature` + `sudo_policy` + `allowed_meta_update_rules` + `physical_or_hardware_attestation`
- WP § 11.1 boot block lists `initial_predicate_registry_root` + `initial_tool_registry_root` + `initial_state_root` + `initial_budget_state` + `on_init_coin_supply` + `boot_time` + `boot_attestation`

Reconciling: `sudo_policy` political prose CAN be replaced by content-addressed predicate (Satoshi-style), BUT the root-of-roots MUST anchor content, not names.

---

## § 2 Revised Schema (8 fields, all content-addressed where applicable)

```toml
[constitution_root]
# Document-level anchors
constitution_hash               = "<sha256 of constitution.md>"
creator_signature               = "<gretjia's PGP/SSH detached-sig of constitution.md>"
signed_at                       = "2026-04-27T..."
schema_version                  = 1

# Content-addressed governance anchors (replace WP "sudo_policy" + "allowed_meta_update_rules")
amendment_predicate_hash        = "<sha256 of the amendment predicate's wasm/rust bytecode>"
initial_predicate_registry_root = "<merkle root of L1 PredicateRegistry at genesis (may be empty tree)>"
initial_tool_registry_root      = "<merkle root of L2 ToolRegistry at genesis (may be empty tree)>"

# Boot attestation (WP § 11.1)
boot_attestation_hash           = "<sha256 of boot manifest including this very file's hash, computed self-referentially via fixed-point procedure>"
```

**8 fields**. Every one anchors content via cryptographic hash (or self-referential boot attestation).

---

## § 3 What each field means + how it's verified at boot

### 3.1 `constitution_hash`

The current `constitution.md` SHA. Boot recomputes; mismatch → `TRUST_ROOT_TAMPERED`. Already in current `genesis_payload.toml`.

### 3.2 `creator_signature`

Detached PGP signature (or SSH-signed git tag fingerprint as fallback per Codex acceptance) covering `constitution.md`. Boot verifies signature against creator's pinned public key. Out-of-band public key registration: pinned in `boot.rs` build-time const **OR** in a separate `creator_keys.toml` file that is itself trust-rooted.

**Recommendation**: pin in `boot.rs` for v4 (hard constant). In v4.1 promote to a separate keyring file once multi-signer governance arrives.

### 3.3 `amendment_predicate_hash`

The hash of the **executable bytecode** (or Rust source for v4) of the predicate that decides "is this proposed Constitution diff valid?". Stored at L3 CAS at hash address `amendment_predicate_hash`.

For v4 (single-signer mode): the amendment predicate is roughly:
```rust
fn amendment_predicate(diff: ConstitutionDiff, signature: PgpSig) -> bool {
    verify_pgp_signature(signature, diff.canonical_form, GRETJIA_PUBKEY)
}
```
Bytecode hash = sha256 of the compiled WASM (or sha256 of the canonical Rust source if WASM compilation deferred).

Future v4.1+ multi-signer: same predicate replaced by:
```rust
fn amendment_predicate_v2(diff: ConstitutionDiff, sigs: Vec<PgpSig>) -> bool {
    let valid = sigs.iter().filter(|s| verify_against_keyring(s, &diff)).count();
    valid >= QUORUM_M
}
```
**Replacing this predicate requires the current predicate to PASS** the replacement diff. Recursive. Bitcoin softfork-equivalent.

### 3.4 `initial_predicate_registry_root`

Merkle root of L1 at genesis time. For v4 genesis with empty registry: this is `EMPTY_TREE_ROOT` (a known constant SHA-256 of empty Merkle tree).

After registry mutations, future `state_root_t` reflects evolving L1 root. Genesis anchor stays immutable.

### 3.5 `initial_tool_registry_root`

Same as 3.4 but for L2.

### 3.6 `boot_attestation_hash`

Self-referential: this hash covers a "boot manifest" that contains `[constitution_hash, creator_signature, signed_at, schema_version, amendment_predicate_hash, initial_predicate_registry_root, initial_tool_registry_root]` plus the existing `[trust_root]` table.

Computation procedure:
```text
1. assemble boot_manifest_bytes = canonical_serialize([all above fields except boot_attestation_hash])
2. boot_attestation_hash = sha256(boot_manifest_bytes)
3. write back to file
4. on boot, recompute and verify
```

This makes `boot_attestation_hash` a **single-byte trust anchor** — if attacker tampers with any field, the boot attestation fails and boot.rs aborts.

---

## § 4 What's REMOVED vs WP § 5.L0 / § 11.1

The following WP-listed fields are **NOT** in this schema, with rationale:

| WP field | Why removed | Equivalent location |
|---|---|---|
| `sudo_policy` (prose) | replaced by `amendment_predicate_hash` (content-addressed) | content-hash anchor |
| `allowed_meta_update_rules` (list) | replaced by `amendment_predicate` recursion (predicate decides what counts as valid amendment) | content-hash anchor |
| `physical_or_hardware_attestation` | v4 is software-only solo; no TPM/Intel SGX | deferred to v4.1+ if hardware attestation arrives |
| `initial_state_root` | empty at genesis = `EMPTY_TREE_ROOT` constant; redundant | implicit |
| `initial_budget_state` | bound at task creation per task, not at genesis | TaskMarket config |
| `on_init_coin_supply` | Inv 4 says NO post-init Coin minting; supply at Inv 4 = empty (founder grant per task) | implicit (zero) |
| `boot_time` | self-referential `boot_attestation_hash` covers it | covered |

What's RETAINED explicitly: anchors that prevent bootstrap circularity per Codex CHALLENGE.

---

## § 5 Migration from current `genesis_payload.toml`

Current file has:
```toml
[pput_accounting_0]
schema_version = 1
... (10 PPUT fields)

[trust_root]
... (49 path → SHA entries)
```

New file structure:
```toml
[constitution_root]      # NEW per this spec
... (8 fields)

[pput_accounting_0]      # KEEP as-is (PPUT-CCL arc spec; not part of Constitution Root)
... (unchanged)

[trust_root]             # KEEP as-is (file-level Trust Root remains essential)
... (49 entries)
```

Both `[constitution_root]` and `[trust_root]` coexist. Boot verifies BOTH:
- `[constitution_root]` is the **abstract** root: governance predicate, registry roots, attestation
- `[trust_root]` is the **concrete** root: file-level SHA-256 of every load-bearing file

`[trust_root]["constitution.md"]` MUST equal `[constitution_root].constitution_hash`.

---

## § 6 boot.rs changes required

Current `boot::verify_trust_root` reads `[trust_root]` table only. Extension:

```rust
pub fn verify_trust_root() -> Result<(), TrustRootError> {
    let manifest = parse_genesis_payload()?;

    // Existing: file-level checks
    verify_trust_root_files(&manifest.trust_root)?;

    // NEW: constitution_root checks
    verify_constitution_root(&manifest.constitution_root)?;

    Ok(())
}

fn verify_constitution_root(cr: &ConstitutionRoot) -> Result<(), TrustRootError> {
    // 1. constitution_hash matches actual constitution.md SHA
    let actual = sha256_file("constitution.md")?;
    if actual != cr.constitution_hash {
        return Err(TrustRootError::ConstitutionHashMismatch);
    }

    // 2. creator_signature verifies
    verify_pgp_or_ssh(&cr.creator_signature, "constitution.md", &PINNED_CREATOR_PUBKEY)?;

    // 3. amendment_predicate_hash exists in L3 CAS
    cas::lookup(&cr.amendment_predicate_hash).ok_or(TrustRootError::AmendmentPredicateMissing)?;

    // 4. initial_*_registry_root match EMPTY_TREE_ROOT for v4 genesis
    if cr.initial_predicate_registry_root != EMPTY_TREE_ROOT {
        return Err(TrustRootError::NonEmptyInitialRegistry);  // v4 genesis must start empty
    }
    if cr.initial_tool_registry_root != EMPTY_TREE_ROOT {
        return Err(TrustRootError::NonEmptyInitialToolRegistry);
    }

    // 5. boot_attestation_hash self-referential check
    let computed = sha256(canonical_serialize_boot_manifest_minus_attestation(&manifest));
    if computed != cr.boot_attestation_hash {
        return Err(TrustRootError::BootAttestationMismatch);
    }

    Ok(())
}
```

New tests:
```
tests/genesis_constitution_root_verify.rs
tests/genesis_amendment_predicate_resolves.rs
tests/genesis_initial_registry_empty.rs
tests/genesis_boot_attestation_self_referential.rs
tests/genesis_creator_signature_verifies.rs
```

---

## § 7 Out-of-scope (deferred)

- Hardware attestation (TPM/SGX) — v4.1 if needed
- Multi-signer creator keyring — v4.1
- WASM compilation of amendment predicate — current spec allows hash of canonical Rust source as fallback; v4.1 promotes to WASM
- `boot_attestation_hash` self-reference circularity protection — currently uses fixed-point convention (compute from all OTHER fields); strengthen with hardware HSM in v4.1 if available

---

## § 8 Honest Acknowledgements

What this spec achieves:
- 8 content-addressed root anchors instead of 5 unanchored names
- No `sudo_policy` political prose at root level (replaced by content hash)
- Recursive amendment via `amendment_predicate` (Bitcoin-softfork-style)
- Closes Codex CHALLENGE on bootstrap circularity

What this spec is honest about:
- `creator_signature` requires pinned public key in `boot.rs`. If attacker controls source, they can swap the pinned key. Mitigation only via reproducible build + checksum verification — out of v4 scope.
- `boot_attestation_hash` self-reference: any tamper that updates ALL fields including the attestation passes self-check. Defense is the chain of trust extending OUT from this file via git tag signatures + multi-source backup. Hardware attestation in v4.1+.
- Empty initial registries means v4 genesis has zero predicates / tools registered. Each subsequent atom (CO P1.5, CO P1.6) populates them via L4 transitions.

— ArchitectAI, 2026-04-27
