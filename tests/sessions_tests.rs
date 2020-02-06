#[test]
fn get_sessions() {
    use tmux_interface::response::session::session::SESSION_ALL;
    use tmux_interface::{NewSession, Sessions, TargetSession, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_get_sessions"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let _sessions = Sessions::get(SESSION_ALL).unwrap();
    //assert_eq!(tmux.has_session(Some("test_has_session")).unwrap(), true);
    tmux.kill_session(None, None, Some(&TargetSession::Raw("test_get_sessions")))
        .unwrap();
}
