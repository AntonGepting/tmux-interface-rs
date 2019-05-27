#[test]
fn sessions_parse() {
    use crate::Sessions;

    let sessions = Sessions::parse("1 1557947146 1557947146 1557947146 $0 b 4\n1 1557947146 1557947146 1557947146 $0 b 4").unwrap();
    assert_eq!(sessions[0].id, 0);
}
