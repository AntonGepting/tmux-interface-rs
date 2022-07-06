#[test]
fn switch() {
    use crate::Switch;

    assert_eq!(Switch::On.to_string(), "on");
    assert_eq!(Switch::Off.to_string(), "off");
}
