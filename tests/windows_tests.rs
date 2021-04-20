#[cfg(feature = "tmux_1_6")]
#[test]
fn get_windows() {
    use tmux_interface::{NewSession, TargetSession, TmuxCommand, Windows};

    const TARGET_SESSION: &str = "test_get_windows";
    let target_session = TargetSession::Raw(TARGET_SESSION);
    let target_session_str = target_session.to_string();

    let tmux = TmuxCommand::new();

    //NewSession::new()
    //.detached()
    //.session_name(TARGET_SESSION)
    //.output()
    //.unwrap();
    tmux.new_session()
        .detached()
        .session_name(&target_session_str)
        .output()
        .unwrap();
    let has_session = tmux
        .has_session()
        .target_session(&target_session_str)
        .output()
        .unwrap();
    assert_eq!(has_session.0.status.success(), true);

    let _windows = Windows::get(&target_session).unwrap();

    tmux.kill_session()
        .target_session(&target_session_str)
        .output()
        .unwrap();
}
