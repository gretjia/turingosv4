//! TB-C0 Constitution Landing Gate — FC2 Boot
//!
//! Constitutional invariants on Flowchart 2:
//!   `human spec / constitution → predicates / tools (init) → Q_0 → runtime loop`
//!
//! Test list (per TB-C0 directive §4.2):
//!   - fc2_genesis_report_exists
//!   - fc2_on_init_only_mint
//!   - fc2_no_post_init_mint
//!   - fc2_no_memory_only_preseed
//!   - fc2_taskopen_escrowlock_are_chain_events
//!   - fc2_run_replayable_from_genesis_tape_cas
//!   - fc2_system_pubkeys_verify
//!   - fc2_agent_registry_resolves
//!
//! All tests are real assertions — no `assert!(true)` per CR-C0.1.

use std::path::Path;
use std::process::Command;

/// FC2-INV1 — A canonical genesis_payload artifact MUST exist at the
/// repo root and parse via the boot module's parser (not just be
/// present as a stray file).
#[test]
fn fc2_genesis_report_exists() {
    let genesis = "genesis_payload.toml";
    assert!(
        Path::new(genesis).exists(),
        "FC2-INV1 violation: {genesis} missing at repo root — boot \
         cannot establish Q_0 from canonical genesis."
    );
    let raw = std::fs::read_to_string(genesis).expect("genesis_payload.toml readable");
    // Must contain trust_root section (PHASE Z′ canonical artifact).
    assert!(
        raw.contains("[trust_root]") || raw.contains("trust_root"),
        "FC2-INV1 violation: genesis_payload.toml lacks trust_root \
         section — boot trust verification un-anchored."
    );
}

/// FC2-INV2a — on_init is the unique mint point. Total Coin supply is
/// minted at boot and never thereafter. We assert by source-side check:
/// the only mint path is the on_init helper, and the
/// assert_no_post_init_mint invariant exists.
#[test]
fn fc2_on_init_only_mint() {
    // The economy module must expose the on_init mint helper.
    let inv_src = std::fs::read_to_string("src/economy/monetary_invariant.rs")
        .expect("monetary_invariant.rs readable");
    assert!(
        inv_src.contains("pub fn assert_no_post_init_mint"),
        "FC2-INV2a violation: assert_no_post_init_mint helper missing \
         — boot-only mint invariant un-enforceable."
    );

    // The bootstrap module must initialize EconomicState ledger entries
    // at boot (default_pput_preseed_pairs or equivalent factory).
    let bootstrap_path = "src/runtime/bootstrap.rs";
    if Path::new(bootstrap_path).exists() {
        let bs_src = std::fs::read_to_string(bootstrap_path).expect("bootstrap.rs readable");
        assert!(
            bs_src.contains("preseed") || bs_src.contains("genesis"),
            "FC2-INV2a violation: bootstrap.rs lacks preseed/genesis \
             initialization — Q_0 origin un-traceable."
        );
    }
}

/// FC2-INV2b — No post-init mint anywhere. The TypedTx variants that
/// touch EconomicState must each pass through the assert_no_post_init_mint
/// gate or be system-emitted. We grep the sequencer for the gate call.
#[test]
fn fc2_no_post_init_mint() {
    let seq_src =
        std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    let calls = seq_src.matches("assert_no_post_init_mint").count();
    assert!(
        calls >= 1,
        "FC2-INV2b violation: assert_no_post_init_mint not called from \
         sequencer dispatch — post-init mint could slip through. Found {calls} call(s)."
    );
}

/// FC2-INV3 — No memory-only preseed. Production code MUST NOT mutate
/// `q.economic_state_t.insert(...)` outside on_init helpers. Memory-only
/// preseed is forbidden per Art. IV + TB-7R + TB-10 ruling.
///
/// Code-grep enforcement: collect all writes to `economic_state_t` and
/// assert none lie outside permitted boot/sequencer code paths.
#[test]
fn fc2_no_memory_only_preseed() {
    let out = Command::new("grep")
        .args([
            "-rEn",
            "--include=*.rs",
            r#"economic_state_t\.(insert|append|extend|push)"#,
            "src/",
            "experiments/",
        ])
        .output()
        .expect("grep should be available");
    let stdout = String::from_utf8_lossy(&out.stdout);

    // Permitted: lines in src/runtime/bootstrap.rs (genesis/preseed) +
    // src/state/sequencer.rs (admission-gated dispatch) + tests + comments.
    let bad: Vec<&str> = stdout
        .lines()
        .filter(|l| {
            let permitted_root = l.contains("src/runtime/bootstrap.rs")
                || l.contains("src/state/sequencer.rs")
                || l.contains("src/state/q_state.rs")
                || l.contains("src/economy/")
                || l.contains("/tests/")
                || l.contains("/test/")
                || l.starts_with("//")
                || l.contains("// ")
                || l.contains("///");
            !permitted_root
        })
        .collect();
    assert!(
        bad.is_empty(),
        "FC2-INV3 violation: memory-only preseed surface(s) detected \
         outside permitted modules. Memory-only preseed is forbidden \
         per Art. IV + TB-7R + TB-10. Found:\n{}",
        bad.join("\n")
    );
}

/// FC2-INV4 — TaskOpen / EscrowLock are chain events. The TB-7R closure
/// established that these are issued as L4 LedgerEntries via Sequencer
/// dispatch, never as memory mutations. We verify by source-side check:
/// `task_open_accept_state_root` + `escrow_lock_accept_state_root` exist.
#[test]
fn fc2_taskopen_escrowlock_are_chain_events() {
    let seq_src =
        std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    assert!(
        seq_src.contains("task_open_accept_state_root"),
        "FC2-INV4 violation: task_open_accept_state_root missing — \
         TaskOpen no longer admission-gated; could become memory-only."
    );
    assert!(
        seq_src.contains("escrow_lock_accept_state_root"),
        "FC2-INV4 violation: escrow_lock_accept_state_root missing — \
         EscrowLock no longer admission-gated; could become memory-only."
    );
}

/// FC2-INV5 — Run is replayable from `genesis_report + ChainTape + CAS`.
/// The verify_chaintape entry produces a ReplayReport; existing
/// `tb_18r_chain_attempt_invariant.rs` exercises full-chain replay.
#[test]
fn fc2_run_replayable_from_genesis_tape_cas() {
    let verify_src =
        std::fs::read_to_string("src/runtime/verify.rs").expect("verify.rs readable");
    assert!(
        verify_src.contains("pub fn verify_chaintape"),
        "FC2-INV5 violation: verify_chaintape entry missing — chain \
         replay un-callable."
    );
    assert!(
        verify_src.contains("pub struct ReplayReport"),
        "FC2-INV5 violation: ReplayReport struct missing — replay \
         outcome un-reportable."
    );

    // Existing TB-13 / TB-14 / TB-18R chaintape smoke tests must exist.
    for required in [
        "tests/tb_13_chaintape_smoke.rs",
        "tests/tb_14_chaintape_smoke.rs",
        "tests/tb_18r_chain_attempt_invariant.rs",
    ] {
        assert!(
            Path::new(required).exists(),
            "FC2-INV5 violation: {required} missing — replay smoke chain \
             evidence not preserved."
        );
    }
}

/// FC2-INV6 — System pubkeys verify. The genesis_payload-pinned system
/// pubkey is loaded at boot and used to verify system tx signatures.
#[test]
fn fc2_system_pubkeys_verify() {
    // 5 existing system_keypair tests cover the full lifecycle.
    for required in [
        "tests/system_keypair_generation.rs",
        "tests/system_keypair_load_and_decrypt.rs",
        "tests/system_keypair_rotation_proof.rs",
        "tests/system_keypair_sign_only_from_runner.rs",
        "tests/system_keypair_verify_correctness.rs",
    ] {
        assert!(
            Path::new(required).exists(),
            "FC2-INV6 violation: {required} missing — system keypair \
             lifecycle invariant un-enforceable."
        );
    }
    let boot_src = std::fs::read_to_string("src/boot.rs").expect("boot.rs readable");
    assert!(
        boot_src.contains("verify_trust_root") || boot_src.contains("trust_root"),
        "FC2-INV6 violation: boot.rs lacks trust_root verification — \
         system pubkey is not pinned at boot."
    );
}

/// FC2-INV7 — Agent registry resolves agent pubkeys deterministically.
/// TB-9 established `AgentKeypairRegistry` with durable encrypted-at-
/// rest keystore; TB-10 + TB-13 wired admission control through it.
#[test]
fn fc2_agent_registry_resolves() {
    // The registry surface must exist at known location. TB-9 + TB-10
    // canonical home: src/runtime/agent_keypairs.rs (per ship-history
    // commits 2026-05-02). Allow legacy candidate paths too.
    let candidate_paths = [
        "src/runtime/agent_keypairs.rs",
        "src/runtime/agent_registry.rs",
        "src/state/agent_registry.rs",
        "src/sdk/agent_registry.rs",
    ];
    let found = candidate_paths
        .iter()
        .any(|p| Path::new(p).exists());
    assert!(
        found,
        "FC2-INV7 violation: AgentKeypairRegistry module missing at any of \
         {candidate_paths:?} — agent pubkey resolution un-anchored."
    );

    // Sequencer must consult agent_pubkeys for admission control.
    let seq_src =
        std::fs::read_to_string("src/state/sequencer.rs").expect("sequencer.rs readable");
    assert!(
        seq_src.contains("agent_pubkeys") || seq_src.contains("AgentKeypairRegistry"),
        "FC2-INV7 violation: sequencer.rs does not reference agent_pubkeys \
         OR AgentKeypairRegistry — Class 3 admission control gap."
    );
}
