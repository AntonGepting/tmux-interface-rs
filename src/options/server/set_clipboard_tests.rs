#[test]
#[cfg(feature = "tmux_1_5")]
fn set_clipboard() {
    use crate::SetClipboard;

    assert_eq!(SetClipboard::On.to_string(), "on");
    assert_eq!(SetClipboard::Off.to_string(), "off");
    #[cfg(feature = "tmux_2_6")]
    assert_eq!(SetClipboard::External.to_string(), "external");
}
