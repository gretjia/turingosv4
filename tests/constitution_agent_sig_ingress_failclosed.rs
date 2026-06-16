//! Constitution gate — agent economic-tx signature verification is FAIL-CLOSED
//! at `submit_agent_tx` ingress for ALL 12 signable economic variants.
//!
//! Authority: OBS_AGENT_SIG_REPLAY_GAP closure; user §8 token (Class-4)
//! `APPROVE-AGENT-SIG-INGRESS-FAILCLOSED-ALL-12` (all 12 agent economic
//! variants fail-closed at ingress).
//!
//! ## What this gate locks
//!
//! `src/state/sequencer.rs::submit_agent_tx` routes every signable ECONOMIC
//! agent variant through the single shared `verify_economic_agent_sig` helper,
//! FAIL-CLOSED:
//!   - no agent-pubkey manifest configured → `SubmitError::AgentManifestRequired`
//!     (state 4). Pre-closure an absent manifest silently bypassed the gate
//!     (Class-3/4 money txs admitted unauthenticated) — the documented
//!     OBS_AGENT_SIG_REPLAY_GAP hole.
//!   - manifest set, valid signature → ACCEPTED (state 1).
//!   - manifest set, forged (zero) signature → `AgentSignatureInvalid` (state 2).
//!   - manifest set, impostor signature (signed by a different registered
//!     agent) → `AgentSignatureInvalid` (state 3).
//!
//! ## Mutation sensitivity
//!
//! The gate asserts all 4 states for a representative subset spanning the 5
//! variants NEWLY covered by this closure (Work / Verify / Challenge /
//! TaskOpen / EscrowLock) AND 2 of the 7 already-covered TB-13/Stage-C
//! variants (CompleteSetMint / MarketSeed). Reverting any one variant's
//! ingress arm to `_ => {}` flips state 4 RED (the unconfigured tx is admitted
//! instead of rejected) AND flips states 2+3 RED (a forged/impostor tx is
//! admitted). Reverting the helper's `None → AgentManifestRequired` arm to a
//! silent bypass (manifest-when-set) flips state 4 RED. Every state is a
//! distinct caught mutant.
//!
//! This is a tape-aware ingress gate: each accept-case (state 1) really enters
//! the sequencer queue; each reject-case asserts the tx is refused BEFORE the
//! queue (the receiver never sees it). No fixture forgery.

use std::sync::Arc;

use tempfile::TempDir;

use turingosv4::bottom_white::cas::schema::Cid;
use turingosv4::runtime::adapter::{
    make_real_challengetx_signed_by, make_real_complete_set_mint_signed_by,
    make_real_escrow_lock_signed_by, make_real_market_seed_signed_by,
    make_real_task_open_signed_by, make_real_verifytx_signed_by, make_real_worktx_signed_by,
};
use turingosv4::runtime::agent_keypairs::{AgentKeypairRegistry, AgentPubkeyManifest};
use turingosv4::state::q_state::{AgentId, Hash, QState, TxId};
use turingosv4::state::sequencer::SubmitError;
use turingosv4::state::typed_tx::{AgentSignature, TypedTx};

mod support;
use support::{fresh_harness, Harness};

// A small placeholder proposal CID for WorkTx (ingress does not resolve it;
// signature verification is over the canonical signing payload only).
fn placeholder_proposal_cid() -> Cid {
    Cid::from_content(b"failclosed-gate-proposal")
}

/// Build a fresh registry rooted in a throwaway dir, register the named agents,
/// and return the registry + the manifest derived from it. The registry can
/// then sign on behalf of any registered agent (real signer OR impostor).
fn registry_with(agents: &[&str]) -> (TempDir, AgentKeypairRegistry, Arc<AgentPubkeyManifest>) {
    let dir = TempDir::new().expect("registry dir");
    let mut reg = AgentKeypairRegistry::open(dir.path()).expect("open registry");
    for a in agents {
        reg.get_or_create(&AgentId((*a).into()))
            .expect("get_or_create");
    }
    let manifest = Arc::new(reg.manifest());
    (dir, reg, manifest)
}

/// Replace the signature of a built+signed TypedTx with the all-zero forgery,
/// preserving every other field (signer included). Mirrors the tb_13 template
/// "Path B" zero-sig forgery.
fn forge_zero_sig(tx: &TypedTx) -> TypedTx {
    let zero = AgentSignature::from_bytes([0u8; 64]);
    match tx.clone() {
        TypedTx::Work(mut t) => {
            t.signature = zero;
            TypedTx::Work(t)
        }
        TypedTx::Verify(mut t) => {
            t.signature = zero;
            TypedTx::Verify(t)
        }
        TypedTx::Challenge(mut t) => {
            t.signature = zero;
            TypedTx::Challenge(t)
        }
        TypedTx::TaskOpen(mut t) => {
            t.signature = zero;
            TypedTx::TaskOpen(t)
        }
        TypedTx::EscrowLock(mut t) => {
            t.signature = zero;
            TypedTx::EscrowLock(t)
        }
        TypedTx::CompleteSetMint(mut t) => {
            t.signature = zero;
            TypedTx::CompleteSetMint(t)
        }
        TypedTx::MarketSeed(mut t) => {
            t.signature = zero;
            TypedTx::MarketSeed(t)
        }
        other => panic!("forge_zero_sig: unsupported variant in this gate: {other:?}"),
    }
}

/// Submit `tx` to a fresh sequencer that has `manifest` pinned and assert the
/// ingress verdict equals `expect`. When `expect == Ok`, also assert the
/// envelope actually reached the queue receiver (the tx was admitted, not
/// silently dropped). When `expect == Err`, assert the receiver is EMPTY
/// (rejected BEFORE the queue — never reached the sequencer).
async fn assert_ingress_with_manifest(
    manifest: &Arc<AgentPubkeyManifest>,
    tx: TypedTx,
    expect: Result<(), SubmitError>,
    case: &str,
) {
    let mut h: Harness = fresh_harness(QState::genesis());
    h.seq
        .set_agent_pubkeys(manifest.clone())
        .expect("set_agent_pubkeys once");
    run_case(&mut h, tx, expect, case).await;
}

/// Submit `tx` to a fresh sequencer with NO manifest pinned (state 4) and
/// assert it is rejected with `AgentManifestRequired` before the queue.
async fn assert_ingress_no_manifest(tx: TypedTx, case: &str) {
    let mut h: Harness = fresh_harness(QState::genesis());
    run_case(&mut h, tx, Err(SubmitError::AgentManifestRequired), case).await;
}

async fn run_case(h: &mut Harness, tx: TypedTx, expect: Result<(), SubmitError>, case: &str) {
    let res = h.seq.submit_agent_tx(tx).await;
    match (expect, res) {
        (Ok(()), Ok(_receipt)) => {
            // Admitted: the envelope must be in the queue (ingress did not drop it).
            assert!(
                h.rx.try_recv().is_ok(),
                "{case}: ingress ACCEPTED but no envelope reached the queue receiver"
            );
        }
        (Ok(()), Err(e)) => {
            panic!("{case}: expected ingress ACCEPT, got reject {e:?}");
        }
        (Err(want), Ok(_)) => {
            panic!("{case}: expected ingress REJECT {want:?}, but tx was ADMITTED to the queue");
        }
        (Err(want), Err(got)) => {
            assert_eq!(
                std::mem::discriminant(&want),
                std::mem::discriminant(&got),
                "{case}: expected reject {want:?}, got {got:?}"
            );
            // Rejected pre-queue: the receiver must be empty.
            assert!(
                h.rx.try_recv().is_err(),
                "{case}: ingress REJECTED but an envelope still reached the queue"
            );
        }
    }
}

/// Run all 4 fail-closed states for one economic variant.
///
/// - `valid`   : correctly signed by the real signer (manifest set) → ACCEPT.
/// - `impostor`: signed by a different registered agent for the same signer
///               field (manifest set) → AgentSignatureInvalid.
/// (`forged` is derived here from `valid` by zeroing its signature.)
async fn assert_four_states(
    manifest: &Arc<AgentPubkeyManifest>,
    valid: TypedTx,
    impostor: TypedTx,
    variant: &str,
) {
    // State 1 — valid signature, manifest set → ACCEPT.
    assert_ingress_with_manifest(manifest, valid.clone(), Ok(()), &format!("{variant}/valid"))
        .await;

    // State 2 — forged (all-zero) signature, manifest set → AgentSignatureInvalid.
    let forged = forge_zero_sig(&valid);
    assert_ingress_with_manifest(
        manifest,
        forged,
        Err(SubmitError::AgentSignatureInvalid),
        &format!("{variant}/forged-zero"),
    )
    .await;

    // State 3 — impostor signature (different registered agent) → AgentSignatureInvalid.
    assert_ingress_with_manifest(
        manifest,
        impostor,
        Err(SubmitError::AgentSignatureInvalid),
        &format!("{variant}/impostor"),
    )
    .await;

    // State 4 — NO manifest configured → AgentManifestRequired (fail-closed).
    assert_ingress_no_manifest(valid, &format!("{variant}/no-manifest")).await;
}

/// Re-sign a built+signed economic tx so the signature is a VALID signature
/// produced by `impostor` (a different registered agent) over the SAME
/// canonical digest, while leaving the signer field pointing at the real
/// agent. Manifest lookup resolves the real agent's pubkey; verification of
/// the impostor's signature against it fails → AgentSignatureInvalid. Mirrors
/// the tb_13 template "Path C".
fn impostor_resign(reg: &mut AgentKeypairRegistry, tx: &TypedTx, impostor: &str) -> TypedTx {
    let impostor_id = AgentId(impostor.into());
    macro_rules! resign {
        ($t:expr, $ctor:path) => {{
            let mut t = $t.clone();
            let digest = t.to_signing_payload().canonical_digest();
            t.signature = reg.sign(&impostor_id, digest).expect("impostor sign");
            $ctor(t)
        }};
    }
    match tx {
        TypedTx::Work(t) => resign!(t, TypedTx::Work),
        TypedTx::Verify(t) => resign!(t, TypedTx::Verify),
        TypedTx::Challenge(t) => resign!(t, TypedTx::Challenge),
        TypedTx::TaskOpen(t) => resign!(t, TypedTx::TaskOpen),
        TypedTx::EscrowLock(t) => resign!(t, TypedTx::EscrowLock),
        TypedTx::CompleteSetMint(t) => resign!(t, TypedTx::CompleteSetMint),
        TypedTx::MarketSeed(t) => resign!(t, TypedTx::MarketSeed),
        other => panic!("impostor_resign: unsupported variant: {other:?}"),
    }
}

const SIGNER: &str = "alice-failclosed";
const IMPOSTOR: &str = "mallory-failclosed";

// ── FC1 stake-bearing agent variants (NEWLY covered by this closure) ─────────

#[tokio::test]
async fn work_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_worktx_signed_by(
        &mut reg,
        "task-fc1",
        SIGNER,
        parent,
        1_000_000,
        "fc",
        placeholder_proposal_cid(),
        true,
        7,
    )
    .expect("build work");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "Work").await;
}

#[tokio::test]
async fn verify_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_verifytx_signed_by(
        &mut reg,
        parent,
        TxId("target-work".into()),
        SIGNER,
        1_000_000,
        "fc",
        true,
        8,
    )
    .expect("build verify");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "Verify").await;
}

#[tokio::test]
async fn challenge_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_challengetx_signed_by(
        &mut reg,
        parent,
        TxId("target-work".into()),
        SIGNER,
        1_000_000,
        placeholder_proposal_cid(),
        "fc",
        9,
    )
    .expect("build challenge");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "Challenge").await;
}

#[tokio::test]
async fn task_open_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_task_open_signed_by(&mut reg, "task-fc-open", SIGNER, parent, "fc", 10)
        .expect("build task_open");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "TaskOpen").await;
}

#[tokio::test]
async fn escrow_lock_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_escrow_lock_signed_by(
        &mut reg,
        "task-fc-open",
        SIGNER,
        50_000_000,
        parent,
        "fc",
        11,
    )
    .expect("build escrow_lock");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "EscrowLock").await;
}

// ── 2 of the 7 already-covered TB-13 / Stage-C variants (regression lock) ────

#[tokio::test]
async fn complete_set_mint_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_complete_set_mint_signed_by(
        &mut reg,
        parent,
        "task-fc-event",
        SIGNER,
        1_000_000,
        "fc",
        12,
    )
    .expect("build mint");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "CompleteSetMint").await;
}

#[tokio::test]
async fn market_seed_ingress_fail_closed_four_states() {
    let (_dir, mut reg, manifest) = registry_with(&[SIGNER, IMPOSTOR]);
    let parent = Hash::ZERO;
    let valid = make_real_market_seed_signed_by(
        &mut reg,
        parent,
        "task-fc-event",
        SIGNER,
        5_000_000,
        "fc",
        13,
    )
    .expect("build market_seed");
    let impostor = impostor_resign(&mut reg, &valid, IMPOSTOR);
    assert_four_states(&manifest, valid, impostor, "MarketSeed").await;
}
