#[test]
fn window_parse() {
    use crate::Window;

    let window = Window::parse("1557947146 0 0 0 @0 1 asdf 0").unwrap();
    assert_eq!(window.name, "asdf");
    assert_eq!(window.id, 0);
    // FIXME: name with dots
    //let window = Window::from_str("1557947146 0 0 0 @0 1 python3.7 0").unwrap();
    //assert_eq!(window.name, "asdf");
    //assert_eq!(window.id, 0);
}
