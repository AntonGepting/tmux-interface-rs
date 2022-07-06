#[test]
#[cfg(feature = "tmux_1_4")]
fn activity() {
    use crate::DetachOnDestroy;

    assert_eq!(DetachOnDestroy::On.to_string(), "on");
    assert_eq!(DetachOnDestroy::Off.to_string(), "off");
    #[cfg(feature = "tmux_3_2")]
    assert_eq!(DetachOnDestroy::NoDetached.to_string(), "no-detached");
}
