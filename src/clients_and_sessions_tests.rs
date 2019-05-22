
#[test]
fn session_parse() {
    use crate::Session;

    let session = Session::parse("1 1557947146 1557947146 1557947146 $0 0 4").unwrap();
    assert_eq!(session.name, "0");
    assert_eq!(session.id, 0);
}

#[test]
fn sessions_parse() {
    use crate::Sessions;

    let sessions = Sessions::parse("1 1557947146 1557947146 1557947146 $0 b 4\n1 1557947146 1557947146 1557947146 $0 b 4").unwrap();
    assert_eq!(sessions[0].id, 0);
}


#[test]
fn list_sessions() {
    use crate::session::Sessions;
    use crate::tmux_interface::TmuxInterface;
    use crate::LIST_SESSIONS_FORMAT;

    let tmux = TmuxInterface::new(None);
    let sessions_str = tmux.list_sessions(Some(LIST_SESSIONS_FORMAT)).unwrap();
    let sessions = Sessions::parse(&sessions_str).unwrap();
    for session in &sessions {
        if session.id == 0 {
        }
    }
    assert_eq!(sessions[0].id, 0);
}


#[test]
fn has_session() {
    use crate::tmux_interface::TmuxInterface;

    let tmux = TmuxInterface::new(None);
    assert_eq!(tmux.has_session("0").unwrap(), true);
    assert_ne!(tmux.has_session("1").unwrap(), true);
}
