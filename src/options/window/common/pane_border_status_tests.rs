#[test]
#[cfg(feature = "tmux_2_3")]
fn pane_border_status() {
    use crate::PaneBorderStatus;

    assert_eq!(PaneBorderStatus::Off.to_string(), "off");
    assert_eq!(PaneBorderStatus::Top.to_string(), "top");
    assert_eq!(PaneBorderStatus::Bottom.to_string(), "bottom");
}
