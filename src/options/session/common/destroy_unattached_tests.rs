#[test]
#[cfg(feature = "tmux_1_5")]
fn action() {
    use crate::DestroyUnattached;

    assert_eq!(DestroyUnattached::Off.to_string(), "off");
    assert_eq!(DestroyUnattached::On.to_string(), "on");
    #[cfg(feature = "tmux_3_4")]
    assert_eq!(DestroyUnattached::KeepLast.to_string(), "keep-last");
    #[cfg(feature = "tmux_3_4")]
    assert_eq!(DestroyUnattached::KeepGroup.to_string(), "keep-group");
}
