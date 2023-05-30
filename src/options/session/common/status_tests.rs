#[test]
#[cfg(feature = "tmux_0_8")]
fn status() {
    use crate::Status;

    assert_eq!(Status::On.to_string(), "on");
    assert_eq!(Status::Off.to_string(), "off");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::TwoRows.to_string(), "2");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::ThreeRows.to_string(), "3");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::FourRows.to_string(), "4");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::FiveRows.to_string(), "5");
}
