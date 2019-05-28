#[test]
fn stack_parse() {
    use regex::Regex;
    let regex = Regex::new(r"^:([\d,]*):$").unwrap();
    assert!(regex.is_match(":3,2,1:"));
}


#[test]
fn parse() {
    use crate::Session;

    let session = Session::parse(":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3").unwrap();
    assert_eq!(session.name, "0");
    assert_eq!(session.id, 0);
}
