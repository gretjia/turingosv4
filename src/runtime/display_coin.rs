//! REAL-13C — human-readable DisplayCoin adapter.
//!
//! This module is a prompt/parser bridge only. It never changes the canonical
//! money unit and never uses floating-point parsing.

/// TRACE_MATRIX FC1/ArtIII: display-only fixed-point scale for prompt-facing
/// amounts; canonical money remains MicroCoin integer units.
pub const DISPLAY_COIN_MICRO: i64 = 1_000_000;

/// TRACE_MATRIX FC1/ArtIII: parses prompt-facing DisplayCoin strings into
/// integer MicroCoin units without f64/f32 or scientific notation.
pub fn parse_display_coin_to_micro(input: &str) -> Result<i64, String> {
    if input.is_empty() || input.trim() != input {
        return Err("display coin amount must be non-empty and unpadded".into());
    }
    if input.contains(['e', 'E', '+', '-', '_']) {
        return Err("display coin amount must be plain unsigned decimal string".into());
    }
    let mut parts = input.split('.');
    let whole = parts.next().unwrap_or_default();
    let frac = parts.next();
    if parts.next().is_some() {
        return Err("display coin amount has more than one decimal point".into());
    }
    if whole.is_empty() || !whole.chars().all(|c| c.is_ascii_digit()) {
        return Err("display coin whole part must be digits".into());
    }
    let whole_micro = whole
        .parse::<i64>()
        .map_err(|_| "display coin whole part overflows i64".to_string())?
        .checked_mul(DISPLAY_COIN_MICRO)
        .ok_or_else(|| "display coin micro amount overflow".to_string())?;

    let frac_micro = match frac {
        None => 0,
        Some("") => return Err("display coin fractional part must be digits".into()),
        Some(frac) => {
            if frac.len() > 2 || !frac.chars().all(|c| c.is_ascii_digit()) {
                return Err("display coin fractional part supports at most two digits".into());
            }
            let frac_value = frac
                .parse::<i64>()
                .map_err(|_| "display coin fractional part invalid".to_string())?;
            let scale = match frac.len() {
                1 => 100_000,
                2 => 10_000,
                _ => unreachable!(),
            };
            frac_value
                .checked_mul(scale)
                .ok_or_else(|| "display coin fractional micro overflow".to_string())?
        }
    };
    whole_micro
        .checked_add(frac_micro)
        .ok_or_else(|| "display coin micro amount overflow".to_string())
}

/// TRACE_MATRIX FC1/ArtIII: renders integer MicroCoin amounts for Trader UX
/// without changing the canonical accounting unit.
pub fn format_display_coin(micro: i64) -> String {
    let whole = micro / DISPLAY_COIN_MICRO;
    let cents = (micro % DISPLAY_COIN_MICRO).abs() / 10_000;
    format!("{whole}.{cents:02} display-coins")
}
