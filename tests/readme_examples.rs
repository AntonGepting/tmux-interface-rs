#[test]
fn example1() {
    use tmux_interface::{NewSessionBuilder, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name("session_name1")
        .build();
    tmux.new_session(Some(&new_session)).unwrap();

    tmux.kill_session(None, None, Some("session_name1"))
        .unwrap();
}

//#[test]
//fn example3() {
//use tmux_interface::{TargetSession, TmuxInterface};

//let mut tmux = TmuxInterface::new();
//let id = tmux.new_session(None).unwrap();
//tmux.kill_session(None, None, Some(&TargetSession::Id(id)))
//.unwrap();
//}

#[test]
fn example2() {
    use tmux_interface::{
        AttachSession, AttachSessionBuilder, NewSession, NewSessionBuilder, TargetSession,
        TmuxInterface,
    };

    let target_session = TargetSession::Raw("session_name2").to_string();
    let mut tmux = TmuxInterface::new();
    let _new_session = NewSession {
        detached: Some(true),
        session_name: Some("session_name2"),
        ..Default::default()
    };
    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name("session_name2")
        .shell_command("sleep 5 && exit")
        .build();
    tmux.new_session(Some(&new_session)).unwrap();

    let _attach_session = AttachSession {
        target_session: Some(&target_session),
        ..Default::default()
    };
    let attach_session = AttachSessionBuilder::new()
        .target_session(&target_session)
        .build();
    tmux.attach_session(Some(&attach_session)).unwrap();
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
