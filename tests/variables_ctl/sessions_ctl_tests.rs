#[cfg(feature = "tmux_1_6")]
#[test]
fn get_sessions() {
    use tmux_interface::{KillSession, NewSession, SessionsCtl, Tmux};

    const TARGET_SESSION: &str = "get_sessions_test";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    let sessions = SessionsCtl::new().get_all().unwrap();
    let mut found = false;
    for session in sessions {
        if session.name == Some(TARGET_SESSION.to_string()) {
            found = true;
        }
    }
    assert!(found);

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
