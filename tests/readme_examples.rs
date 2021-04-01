#[test]
fn example1() {
    use tmux_interface::TmuxCommand;

    let tmux = TmuxCommand::new();

    tmux.new_session()
        .detached()
        .session_name("example_1")
        .output()
        .unwrap();
    tmux.has_session()
        .target_session("example_1")
        .output()
        .unwrap();
    tmux.kill_session()
        .target_session("example_1")
        .output()
        .unwrap();
}

#[test]
fn example2() {
    use tmux_interface::{HasSession, KillSession, NewSession};

    NewSession::new()
        .detached()
        .session_name("example_2")
        .output()
        .unwrap();
    HasSession::new()
        .target_session("example_2")
        .output()
        .unwrap();
    KillSession::new()
        .target_session("example_2")
        .output()
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
