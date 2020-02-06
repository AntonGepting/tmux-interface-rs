#[test]
fn example1() {
    use tmux_interface::{AttachSession, NewSession, TargetPane, TargetSession, TmuxInterface};

    let target_session = TargetSession::Raw("session_name");
    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("session_name"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let attach_session = AttachSession {
        target_session: Some(&target_session),
        ..Default::default()
    };
    tmux.send_keys::<TargetPane>(None, &vec!["exit", "C-m"])
        .unwrap();
    tmux.attach_session(Some(&attach_session)).unwrap();
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
