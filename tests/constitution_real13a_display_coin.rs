//! REAL-13C — DisplayCoin cognitive bridge gate.

use turingosv4::runtime::display_coin::{
    format_display_coin, parse_display_coin_to_micro, DISPLAY_COIN_MICRO,
};

#[test]
fn display_coin_parser_uses_fixed_scale_integer_micro_units() {
    assert_eq!(DISPLAY_COIN_MICRO, 1_000_000);
    assert_eq!(parse_display_coin_to_micro("0").unwrap(), 0);
    assert_eq!(parse_display_coin_to_micro("1").unwrap(), 1_000_000);
    assert_eq!(parse_display_coin_to_micro("1.50").unwrap(), 1_500_000);
    assert_eq!(parse_display_coin_to_micro("100.00").unwrap(), 100_000_000);
    assert_eq!(format_display_coin(1_500_000), "1.50 display-coins");
}

#[test]
fn display_coin_parser_rejects_floaty_or_ambiguous_forms() {
    for invalid in [
        "1.234", "1e6", "1E6", "1_000", "-1", "+1", "NaN", "Infinity", "", " 1.00", "1.00 ",
    ] {
        assert!(
            parse_display_coin_to_micro(invalid).is_err(),
            "DisplayCoin parser must reject {invalid:?}"
        );
    }
}

#[test]
fn display_coin_parser_fails_closed_on_overflow() {
    assert!(parse_display_coin_to_micro("9223372036854775808").is_err());
}
