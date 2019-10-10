#[test]
fn parse() {
    use crate::Session;
    use crate::Sessions;

    let sessions_str = "1557947146::1:1557947146:1::::0:$0:1557947146:0:0:3,2,1:3\n\
                        1557947146::1:1557947146:1::::0:$0:1557947146:0:0:3,2,1:3";
    let sessions = Sessions::from_str(sessions_str, Session::SESSION_ALL).unwrap();
    assert_eq!(sessions[0].id, Some(0));

    let sessions_str = "1557947146::1:1557947146:1::::0:$0:1557947146:0:0:4,3,2,1:4\n\
                        1557947146::0:1557947146:1::::0:$40:1557947146:0:test_has_session:1:1";
    let sessions = Sessions::from_str(sessions_str, Session::SESSION_ALL).unwrap();
    assert_eq!(sessions[1].id, Some(40));
}

#[test]
fn get() {
    use crate::Session;
    use crate::Sessions;

    let _sessions = Sessions::get(Session::SESSION_ALL).unwrap();
}
