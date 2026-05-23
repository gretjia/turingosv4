//! TRACE_MATRIX FC1-N14: Polymarket PR1 revision (2026-05-23, Karpathy nice-fix
//! #3) — shared CID hex codec.
//!
//! Decodes a 64-character lowercase hex string into a 32-byte
//! [`Cid`]. Previously duplicated as `cid_from_hex_str` /
//! `hex_nibble` in BOTH `src/bin/turingos/cmd_generate.rs` and
//! `src/web/market_view.rs`. Centralized here so the two call sites
//! share a single canonical decoder.
//!
//! Class 1: additive pure helper. No state, no I/O.
//!
//! Note: `Cid` exposes `hex()` (encoder) but no `from_hex` constructor on
//! the canonical-signing-payload surface (`src/bottom_white/cas/schema.rs`
//! is a §6 restricted surface — not extended here).

use crate::bottom_white::cas::schema::Cid;

/// TRACE_MATRIX FC1-N14: shared 64-char hex → [`Cid`] decoder.
///
/// Returns `Err` when:
/// - input length is not 64
/// - any byte is outside `[0-9a-fA-F]`.
pub fn cid_from_hex_str(s: &str) -> Result<Cid, String> {
    if s.len() != 64 {
        return Err(format!("expected 64 hex chars, got {}", s.len()));
    }
    let mut bytes = [0u8; 32];
    for (i, byte_pair) in s.as_bytes().chunks(2).enumerate() {
        let hi = hex_nibble(byte_pair[0])?;
        let lo = hex_nibble(byte_pair[1])?;
        bytes[i] = (hi << 4) | lo;
    }
    Ok(Cid(bytes))
}

fn hex_nibble(b: u8) -> Result<u8, String> {
    match b {
        b'0'..=b'9' => Ok(b - b'0'),
        b'a'..=b'f' => Ok(10 + b - b'a'),
        b'A'..=b'F' => Ok(10 + b - b'A'),
        _ => Err(format!("non-hex byte 0x{b:02x}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cid_from_hex_str_roundtrip() {
        let original = [0xab; 32];
        let hex_str: String = original.iter().map(|b| format!("{b:02x}")).collect();
        let cid = cid_from_hex_str(&hex_str).expect("decode");
        assert_eq!(cid.0, original);
    }

    #[test]
    fn cid_from_hex_str_rejects_wrong_length() {
        assert!(cid_from_hex_str("ab").is_err());
        assert!(cid_from_hex_str(&"a".repeat(63)).is_err());
        assert!(cid_from_hex_str(&"a".repeat(65)).is_err());
    }

    #[test]
    fn cid_from_hex_str_rejects_non_hex() {
        assert!(cid_from_hex_str(&"g".repeat(64)).is_err());
    }

    #[test]
    fn cid_from_hex_str_accepts_uppercase() {
        let original = [0xAB; 32];
        let hex_str: String = original.iter().map(|b| format!("{b:02X}")).collect();
        let cid = cid_from_hex_str(&hex_str).expect("decode");
        assert_eq!(cid.0, original);
    }
}
