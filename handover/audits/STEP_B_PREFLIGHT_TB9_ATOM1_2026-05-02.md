# STEP_B Preflight — TB-9 Atom 1: Durable Agent Keystore

**Date**: 2026-05-02
**Atom**: TB-9 Atom 1 (Class 3 — auth-crypto-money; durable keystore primitive)
**Restricted file**: `src/runtime/agent_keypairs.rs` (auth primitive — every WorkTx signature flows through this module's `sign()` method).

## §1 STEP_B classification per `feedback_step_b_protocol`

`agent_keypairs.rs` is the agent-side equivalent of `system_keypair.rs`; under the standard restricted-file list (`bus.rs / kernel.rs / wallet.rs` + auth primitives by extension), it triggers STEP_B. Per the protocol: parallel-branch A/B with side-by-side test on existing precedent.

**Existing precedent**: `src/bottom_white/ledger/system_keypair.rs:417-463` ships a load-or-generate factory (`generate_or_load_system_keypair`) for the system keypair. TB-9 Atom 1 mirrors this pattern for agent keypairs.

## §2 Surface change — additive

```text
NEW   src/runtime/agent_keystore.rs           (encryption primitives; ~300 lines)
EDIT  src/runtime/agent_keypairs.rs           (+constructor; +durable persist hook; ~40 net lines added)
EDIT  src/runtime/mod.rs                      (+pub mod agent_keystore; 1 line)
```

**No deletions or behavior changes** to existing API:
- `AgentKeypairRegistry::open(runtime_repo_path)` — unchanged (TB-7 fail-closed-on-existing semantics retained for tests + non-evaluator call sites).
- `AgentKeypairRegistry::sign(agent_id, digest)` — unchanged signature; behavior extended (also persists to durable keystore if registry was opened in durable mode).
- `AgentKeypair::generate()` / `AgentKeypair::sign_digest()` / `verify_agent_signature()` — unchanged.
- `AgentPubkeyManifest` — unchanged (per-run sidecar continues to be written exactly as TB-7).

**New API**:
- `pub fn AgentKeypair::from_secret_bytes(seed: [u8; 32]) -> Result<Self, AgentKeypairError>` — reconstructs keypair from saved secret.
- `pub fn AgentKeypairRegistry::generate_or_load_durable(runtime_repo_path, durable_keystore_path, password) -> Result<Self, AgentKeypairError>` — load-or-generate; persists to durable keystore on every new keypair.
- `pub mod agent_keystore` with: `default_agent_keystore_path()`, `load_agent_keystore_file(path, password)`, `save_agent_keystore_file(path, password, secrets)`, `AgentKeystoreError`.

## §3 Side-by-side test plan

Test parity with TB-7 fail-closed-on-existing semantics (existing tests U-A1.a..U-A1.f at `src/runtime/agent_keypairs.rs:354-432`):

| TB-7 test | Behavior under TB-9 Atom 1 | Verdict |
|---|---|---|
| U-A1.a generate produces signing keypair | Unchanged (still calls `AgentKeypair::generate`) | ✓ pass |
| U-A1.b registry persists manifest on first use | Unchanged (manifest write path same; durable persist is no-op in non-durable mode) | ✓ pass |
| U-A1.c same agent reuses keypair across signs | Unchanged | ✓ pass |
| U-A1.d manifest round-trip | Unchanged | ✓ pass |
| U-A1.e registry open refuses existing manifest | Unchanged for `::open(...)`. New `generate_or_load_durable` allows reopening (the WHOLE POINT of durability). | ✓ both pass |
| U-A1.f wrong pubkey rejects signature | Unchanged | ✓ pass |

**New tests added in this atom** (post-implementation, U-A2.a..U-A2.f):

| Test | Assertion |
|---|---|
| U-A2.a generate-or-load fresh produces empty registry + creates encrypted file | Path exists; file size > 0; first byte is magic `T` of `TOS4AGTKEY1` |
| U-A2.b second-load decrypts + signatures verify under same pubkey | Run-A signs digest D under agent "n1"; record pubkey1. Drop registry. Run-B: `generate_or_load_durable` same path + password. Sign digest D under "n1"; record pubkey2. Assert pubkey1 == pubkey2; both signatures verify under that pubkey. |
| U-A2.c corrupted keystore rejected | Tamper with one byte mid-file; assert `Err(...)` not silent regenerate. |
| U-A2.d wrong KDF password rejected | Save with password "alpha"; load with "beta"; assert `Err(...)`. |
| U-A2.e env override `TURINGOS_AGENT_KEYSTORE_PATH` honored | Set env; resolve default path; assert override value. |
| U-A2.f file permissions 0600 (Unix) | Stat the keystore; assert mode == 0o600. |

## §4 Side-effect surface

- `~/.turingos/keystore/` directory creation: matches existing `system_keypair.rs` precedent.
- `argon2 / chacha20poly1305 / secrecy` already in `Cargo.toml`; no new deps.
- Format magic `TOS4AGTKEY1` is distinct from system's `TOS4SYSKEY1` — file confusion impossible.

## §5 Recursive failure modes considered

1. **Keystore decryption succeeds but reconstructed pubkey ≠ stored public_key**: `AgentKeypair::from_secret_bytes` recomputes pubkey from seed; if seed is corrupted such that decryption AEAD passes but the value is wrong, signing will produce signatures that don't verify under the (stale) manifest entry. Defense: `from_secret_bytes` returns the recomputed pubkey; the keystore stores ONLY secrets (32 bytes per agent), pubkeys are recomputed at load time. Result: corrupted seed → wrong pubkey → manifest mismatch → next sign produces invalid signature. Caller-side handling: loud failure, no silent corruption.

2. **Concurrent evaluator processes same keystore**: file lock not implemented in MVP. Tests use TempDir + override path, so no real-world concurrency on `~/.turingos/keystore/agent_keystore.enc`. Production guard: post-v1.0 polish (TB-16+).

3. **Password leaked via env**: env var `TURINGOS_AGENT_KEYSTORE_PASSWORD` is read at evaluator boot via `secrecy::SecretString` (zeroize on drop). Not logged. Documented MVP per `feedback_kolmogorov_compression` — production-grade prompt + zeroize on stack is post-v1.0 polish.

4. **Replay can't verify because manifest absent**: per-run `agent_pubkeys.json` continues to be written exactly as TB-7. `verify_chaintape` semantics unchanged. Durable keystore is additive.

## §6 STEP_B verdict

**PROCEED**. Surface is purely additive; existing TB-7 tests retained; new tests cover the durable-mode paths. Ready for Atom 1 dispatch.
