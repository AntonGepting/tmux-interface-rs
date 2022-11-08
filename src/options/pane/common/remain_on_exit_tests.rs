#[test]
#[cfg(feature = "tmux_3_0")]
fn remain_on_exit() {
    use crate::RemainOnExit;

    assert_eq!(RemainOnExit::On.to_string(), "on");
    assert_eq!(RemainOnExit::Off.to_string(), "off");
    #[cfg(feature = "tmux_3_2")]
    assert_eq!(RemainOnExit::Failed.to_string(), "failed");
}
