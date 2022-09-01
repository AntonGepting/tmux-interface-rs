#[test]
#[cfg(feature = "tmux_1_0")]
fn status_justify() {
    use crate::StatusJustify;

    assert_eq!(StatusJustify::Left.to_string(), "left");
    assert_eq!(StatusJustify::Centre.to_string(), "centre");
    assert_eq!(StatusJustify::Right.to_string(), "right");
}
