#[test]
fn sessions_parse() {
    use crate::Sessions;

    let sessions = Sessions::parse(":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3\n:1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3").unwrap();
    assert_eq!(sessions[0].id, 0);
}
