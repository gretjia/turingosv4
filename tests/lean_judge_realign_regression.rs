//! 门3 binding regression (OBL-018): a PURE, toolchain-free lock on `lean_judge::realign`
//! so the de-alignment fix cannot silently regress. Unlike the real-Lean end-to-end tests
//! (which skip when the pinned toolchain is absent), these assertions run everywhere,
//! including CI under `cargo test --workspace`.

use turingosv4::judges::lean_judge::realign;

#[test]
fn realign_flushes_flat_first_line_shallow() {
    // IP1: flush-first tactic, indented sibling (common prefix "" — dedent can't recover).
    assert_eq!(realign("simp\n  ring"), "simp\nring");
    // IP2-shaped: the inline `:= by` slice " tac\n  tac" (common prefix " ").
    assert_eq!(realign(" simp\n  ring"), "simp\nring");
    // uniform indent → col 0.
    assert_eq!(realign("  simp\n  ring"), "simp\nring");
    // already col 0 → unchanged (no regression on the common path).
    assert_eq!(realign("simp\nnorm_num"), "simp\nnorm_num");
    // tabs expanded then flushed.
    assert_eq!(realign("\tsimp\n\tring"), "simp\nring");
    // CRLF tolerated.
    assert_eq!(realign("simp\r\n  ring"), "simp\nring");
    // single line trims.
    assert_eq!(realign("  rfl  "), "rfl");
}

#[test]
fn realign_preserves_genuine_nesting() {
    // a `have … := by` opener → defer to conservative dedent (strip shared prefix only).
    assert_eq!(
        realign("  have h : True := by\n    trivial\n  exact h"),
        "have h : True := by\n  trivial\nexact h"
    );
    // focus-dot block preserved (· starts a child block).
    assert_eq!(
        realign("  constructor\n  · exact hp\n  · exact hq"),
        "constructor\n· exact hp\n· exact hq"
    );
}
