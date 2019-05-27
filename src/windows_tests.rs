#[test]
fn windows_parse() {
    use crate::Windows;

    let windows = Windows::parse("1557947146 0 0 0 @0 1 asdf 0\n1557947146 0 0 0 @1 2 fdas 0").unwrap();
    assert_eq!(windows[0].id, 0);
}
