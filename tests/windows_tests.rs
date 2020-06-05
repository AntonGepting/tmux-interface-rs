#[cfg(feature = "tmux_1_6")]
#[test]
fn get_windows() {
    use tmux_interface::response::window::window::WINDOW_ALL;
    use tmux_interface::{NewSessionBuilder, TargetSession, TmuxInterface, Windows};

    const TARGET_SESSION: &str = "test_get_windows";
    let target_session = TargetSession::Raw(TARGET_SESSION);
    let target_session_str = target_session.to_string();

    let mut tmux = TmuxInterface::new();

    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name(TARGET_SESSION)
        .build();
    tmux.new_session(Some(&new_session)).unwrap();
    assert_eq!(tmux.has_session(Some(&target_session_str)).unwrap(), true);

    let _windows = Windows::get(&target_session, WINDOW_ALL).unwrap();

    tmux.kill_session(None, None, Some(&target_session_str))
        .unwrap();
}
