#[test]
#[cfg(feature = "tmux_1_7")]
fn status_position() {
    use crate::StatusPosition;

    assert_eq!(StatusPosition::Top.to_string(), "top");
    assert_eq!(StatusPosition::Bottom.to_string(), "bottom");
}
