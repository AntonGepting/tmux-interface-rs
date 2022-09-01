#[test]
#[cfg(feature = "tmux_0_8")]
fn activity() {
    use crate::Activity;

    assert_eq!(Activity::On.to_string(), "on");
    assert_eq!(Activity::Off.to_string(), "off");
    #[cfg(feature = "tmux_2_6")]
    assert_eq!(Activity::Both.to_string(), "both");
}
