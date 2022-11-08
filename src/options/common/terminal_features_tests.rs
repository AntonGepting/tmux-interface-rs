#[test]
fn terminal_features_tests() {
    use crate::TerminalFeatures;

    assert_eq!(Switch::On.to_string(), "on");
    assert_eq!(Switch::Off.to_string(), "off");
}
