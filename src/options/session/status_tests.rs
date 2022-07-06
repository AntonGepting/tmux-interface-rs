#[test]
#[cfg(feature = "tmux_0_8")]
fn status() {
    use crate::Status;

    assert_eq!(Status::On.to_string(), "on");
    assert_eq!(Status::Off.to_string(), "off");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::_2.to_string(), "2");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::_3.to_string(), "3");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::_4.to_string(), "4");
    #[cfg(feature = "tmux_2_9")]
    assert_eq!(Status::_5.to_string(), "5");
}
