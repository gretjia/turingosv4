# Claude Auditor Verdict — CO1.7.0a-f system_keypair

> **Date**: 2026-04-27
> **Subject**: working-tree implementation of system_keypair (Codex implementer); HEAD `c2f94c6` at audit time.
> **Authority**: Hard rule 2 (Tri-Model Orchestration Protocol § 9) — Claude (auditor) reviews atoms Codex implements; never Codex auditing Codex.
> **Scope**: only the keypair atom; Q_t (CO1.2) audit is in `CODEX_CO1_2_QSTATE_AUDIT_2026-04-27.md`.

---

## Q1-Q15 Verdicts

| # | Item | Verdict | Rationale |
|---|------|---------|-----------|
| Q1 | Algorithm correctness | PASS | `ed25519_dalek::SigningKey` (line 15), `Argon2 + Algorithm::Argon2id + Version::V0x13` (line 583), `ChaCha20Poly1305` (line 14), `getrandom::getrandom` (lines 236, 549, 550). All four primitives present and correctly wired. |
| Q2 | C-027 no hardcoded params | PASS | `KdfParams::from_env` reads `TURINGOS_KDF_MEMORY_KIB` / `TURINGOS_KDF_ITER` / `TURINGOS_KDF_LANES` (lines 636-638); defaults `65_536 / 3 / 4` match spec § 3.2 (m=64 MiB, t=3, p=4); keystore path overridable via `TURINGOS_KEYSTORE_PATH` (line 357). All four params env-overridable, zero-value rejected (line 649). |
| Q3 | Zeroize discipline | PASS | `Ed25519Keypair` derives `Zeroize, ZeroizeOnDrop` (line 225) and crucially does NOT derive `Debug`, `Serialize`, or `Clone` — secret cannot be Debug-printed or serialized. Local `seed` / `secret` / `key` / `plaintext` byte arrays explicitly `.zeroize()` after use (lines 243, 274, 292, 398, 559, 560). `secret_key` is `Box<[u8]>` (heap-zeroized on drop). No log/Debug leak path. |
| Q4 | mlock attempt | PASS | `mlock_best_effort` invoked in `generate_with_secure_entropy` (line 244) and `from_plaintext` (line 275); `cfg(unix)` calls `libc::mlock` (line 794); `cfg(not unix)` returns false; failure non-fatal per spec § 3.2. |
| Q5 | Restricted sign API | PASS | Two `pub(crate) mod` scopes (`predicate_runner`, `terminal_summary_emitter`, lines 474, 501); inner `sign_system_message_inner` is private free function (line 535); `CanonicalMessage` typed enum gates all signing (line 188); no public `sign_*` API; no byte-slice free-form signing. Conformance test enforces this textually. |
| Q6 | Verify API public | PASS | `pub fn verify_system_signature` (line 433) + `pub fn verify_epoch_rotation_proof` (line 452) + `pub fn canonical_digest` (line 403) — verification fully public per spec § 3.4. |
| Q7 | Filesystem | PASS | Default path `~/.turingos/keystore/system_keypair_v{epoch}.enc` (lines 361-364); `OpenOptions::create_new(true)` + mode 0o600 set both at open and via `set_permissions` post-write (lines 596-602, 609, 618); 16-byte random salt generated alongside ciphertext via `getrandom` (line 549); salt + nonce + ciphertext + KDF params all in self-describing envelope with magic `TOS4SYSKEY1` + version byte. |
| Q8 | TRACE_MATRIX coverage | PASS | All public symbols carry `/// TRACE_MATRIX FC1-Sig` and/or `FC3-Sig` doc-comments — module header, `SystemEpoch`, `SystemPublicKey`, `SystemSignature`, `RejectedAttemptSummary`, `TerminalSummaryTx`, `EpochRotationProof`, `CanonicalMessage` + each variant, `PinnedSystemPubkeys` + methods, `Ed25519Keypair` + methods, all `KeypairError` variants, `default_system_keystore_path`, `generate_or_load_system_keypair`, `load_existing_keypair`, `verify_system_signature`, `verify_epoch_rotation_proof`, `verify_system_pubkeys`, `canonical_digest`. WP § 16 backlink format honored. |
| Q9 | Conformance tests present | PASS | All five files exist with mandated names (`tests/system_keypair_{generation,load_and_decrypt,sign_only_from_runner,verify_correctness,rotation_proof}.rs`). |
| Q10 | Env-var test lock | PASS | `tests/system_keypair_generation.rs` and `tests/system_keypair_load_and_decrypt.rs` (the only env-mutating tests) both use `static ENV_LOCK: OnceLock<Mutex<()>>` + per-test `_lock = env_lock().lock()` (lines 8-11, 38). Verify and rotation tests do not mutate env. |
| Q11 | No unsafe outside mlock helper | PASS | `grep -n unsafe` returns exactly one hit at line 794 inside `mlock_os_best_effort`, with safety comment justifying it. |
| Q12 | No `println!`/`eprintln!` in non-test code | PASS | `grep` returns no matches in `system_keypair.rs`. |
| Q13 | `.env` not modified, keystore not committed | PASS | `git status -s` shows `.env` unchanged; `~/.turingos/keystore/` lives in user HOME outside the repo. Tests use `tempdir()` for keystore paths — no host-wide artifact written by `cargo test`. |
| Q14 | Test pass rate 244/0 | PASS | `cargo test`: 190 lib + 54 integration = 244 PASS, 0 FAIL, 125 ignored (pre-existing stubs). `cargo check --lib --tests` clean (warnings only, none in keypair files). |
| Q15 | TR refresh accuracy | PASS | All 10 working-tree shas match `genesis_payload.toml [trust_root]` exactly. Spec doc itself TR-pinned (line 233). |

## Holistic verdict: **PASS**

## Must-fix: **none**

## Minor observations (informational, not blocking)
1. `verify_system_pubkeys` (line 464) is a stub with TODO — acceptable per atom scope CO1.7.0a-f (creator PGP verification arrives with B-1 PGP Tag Governance atom). Function correctly no-ops when section absent rather than silently passing on malformed data.
2. `Ed25519Keypair::sign_digest` does an extra `secret.copy_from_slice` into a stack buffer (line 288) before constructing the `SigningKey`. The stack copy is properly zeroized but adds one transient cleartext copy per signature. Acceptable trade-off; could be optimized later by storing a `SigningKey` directly with custom `Drop`.

---

— Claude Auditor (read-only subagent), 2026-04-27 post-Wave-4-B
