#[test]
#[cfg(feature = "tmux_1_0")]
fn clock_mode_style() {
    use crate::ClockModeStyle;

    assert_eq!(ClockModeStyle::H12.to_string(), "12");
    assert_eq!(ClockModeStyle::H24.to_string(), "24");
}
