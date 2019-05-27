#[test]
fn session_parse() {
    use crate::Session;

    let session = Session::parse("1 1557947146 1557947146 1557947146 $0 0 4").unwrap();
    assert_eq!(session.name, "0");
    assert_eq!(session.id, 0);
}
