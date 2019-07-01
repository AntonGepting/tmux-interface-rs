#[test]
fn parse() {
    use crate::Session;

    let session = Session::parse(":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3").unwrap();
    assert_eq!(session.name, Some("0".to_string()));
    assert_eq!(session.id, Some(0));
}
