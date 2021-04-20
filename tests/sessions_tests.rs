#[cfg(feature = "tmux_1_6")]
#[test]
fn get_sessions() {
    use tmux_interface::{Sessions, TargetSession, TmuxCommand};

    const TARGET_SESSION: &str = "test_get_sessions";
    let target_session = TargetSession::Raw(TARGET_SESSION).to_string();

    let tmux = TmuxCommand::new();
    tmux.new_session()
        .detached()
        .session_name(&target_session)
        .output()
        .unwrap();
    let has_session = tmux
        .has_session()
        .target_session(&target_session)
        .output()
        .unwrap();
    assert_eq!(has_session.0.status.success(), true);

    let _sessions = Sessions::get().unwrap();

    tmux.kill_session()
        .target_session(&target_session)
        .output()
        .unwrap();
}
