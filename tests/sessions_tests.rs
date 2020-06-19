#[cfg(feature = "tmux_1_6")]
#[test]
fn get_sessions() {
    use tmux_interface::SESSION_ALL;
    use tmux_interface::{NewSessionBuilder, Sessions, TargetSession, TmuxInterface};

    const TARGET_SESSION: &str = "test_get_sessions";
    let target_session = TargetSession::Raw(TARGET_SESSION).to_string();

    let mut tmux = TmuxInterface::new();
    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name(TARGET_SESSION)
        .build();
    tmux.new_session(Some(&new_session)).unwrap();
    assert_eq!(tmux.has_session(Some(&target_session)).unwrap(), true);

    let _sessions = Sessions::get(SESSION_ALL).unwrap();

    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
