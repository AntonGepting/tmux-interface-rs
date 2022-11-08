#[test]
#[cfg(feature = "tmux_3_2")]
fn extended_keys() {
    use crate::ExtendedKeys;

    assert_eq!(ExtendedKeys::On.to_string(), "on");
    assert_eq!(ExtendedKeys::Off.to_string(), "off");
    #[cfg(feature = "tmux_3_2a")]
    assert_eq!(ExtendedKeys::Always.to_string(), "always");
}
