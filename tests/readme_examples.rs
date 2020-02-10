#[test]
fn example1() {
    use tmux_interface::{
        AttachSession, AttachSessionBuilder, NewSession, NewSessionBuilder, TargetPane,
        TargetSession, TmuxInterface,
    };

    let target_session = TargetSession::Raw("session_name");
    let mut tmux = TmuxInterface::new();
    let _new_session = NewSession {
        detached: Some(true),
        session_name: Some("session_name"),
        ..Default::default()
    };
    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name("session_name")
        .build();
    tmux.new_session(Some(&new_session)).unwrap();

    let _attach_session = AttachSession {
        target_session: Some(&target_session),
        ..Default::default()
    };
    let attach_session = AttachSessionBuilder::new()
        .target_session(&target_session)
        .build();
    tmux.send_keys::<TargetPane>(None, &vec!["exit", "C-m"])
        .unwrap();
    tmux.attach_session(Some(&attach_session)).unwrap();
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
