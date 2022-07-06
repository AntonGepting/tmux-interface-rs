#[test]
#[cfg(feature = "tmux_0_8")]
fn action() {
    use crate::Action;

    assert_eq!(Action::Any.to_string(), "any");
    assert_eq!(Action::None.to_string(), "none");
    assert_eq!(Action::Current.to_string(), "current");
    #[cfg(feature = "tmux_2_1")]
    assert_eq!(Action::Other.to_string(), "other");
}
