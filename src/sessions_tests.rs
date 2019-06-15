#[test]
fn parse() {
    use crate::Sessions;

    let sessions_str = ":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3\n:1:1557947146:1557947146:1:1557947146::::0:$0:0:0:3,2,1:3";
    let sessions = Sessions::parse(sessions_str).unwrap();
    assert_eq!(sessions[0].id, 0);

    let sessions_str = ":1:1557947146:1557947146:1:1557947146::::0:$0:0:0:4,3,2,1:4\n:0:1557947146:1557947146:1:::::0:$40:0:test_has_session:1:1";
    let sessions = Sessions::parse(sessions_str).unwrap();
    assert_eq!(sessions[1].id, 40);
}


#[test]
fn new_session() {
    use crate::TmuxInterface;
    //use crate::Sessions;
    use crate::NewSession;

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_has_session"),
        ..Default::default()
    };
    tmux.new_session(&new_session).unwrap();
    //let sessions = Sessions::get().unwrap();
    //assert_eq!(sessions[1].name, "test_has_session");
    //dbg!(sessions);
    tmux.kill_session(Some("test_has_session"), None, None).unwrap();
}
