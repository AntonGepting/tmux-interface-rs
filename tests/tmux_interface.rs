extern crate tmux_interface;

//#[test]
//fn attach_session() {
//unimplemented!();
//}

//#[test]
//fn detach_client() {
//unimplemented!();
//}

#[test]
fn has_session() {
    use crate::tmux_interface::TmuxCommand;

    let tmux = TmuxCommand::new();
    //tmux.tmux = Some("./tests/tmux_mock.sh");
    let session_name = "test_has_session";
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();
    let has_session = tmux
        .has_session()
        .target_session(session_name)
        .output()
        .unwrap();
    assert_eq!(has_session.0.status.success(), true);
    tmux.kill_session()
        .target_session(session_name)
        .output()
        .unwrap();
}

//#[test]
//fn kill_server() {
//unimplemented!();
//}

#[test]
fn kill_session() {
    use crate::tmux_interface::TmuxCommand;

    let session_name = "test_kill_session";

    let tmux = TmuxCommand::new();
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();
    tmux.kill_session()
        .target_session(session_name)
        .output()
        .unwrap();
}

// NOTE: comment out, bash scripts moved out from tests directory
//#[test]
//fn callback() {
//use crate::tmux_interface::NewSession;

//let mut tmux = TmuxInterface::new();
//let new_session = NewSession {
//detached: Some(true),
//session_name: Some("test_kill_session"),
//..Default::default()
//};

//tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
//*bin = "./tests/tmux_test.sh".to_string();
//println!("callback() prehook: {:?} {:?} {:?}", bin, options, subcmd);
////Err(Error::new("hook"))
//Ok(None)
//}));

//tmux.new_session(Some(&new_session)).unwrap();
//tmux.kill_session(None, None, Some("test_kill_session"))
//.unwrap();
//}

//#[test]
//fn list_clients() {
//unimplemented!();
//}

//#[test]
//fn list_commands() {
//unimplemented!();
//}

#[test]
fn list_sessions() {
    use crate::tmux_interface::TmuxCommand;

    let session_name = "test_list_sessions";
    let tmux = TmuxCommand::new();
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();
    tmux.list_sessions().output().unwrap();
    tmux.kill_session()
        .target_session(session_name)
        .output()
        .unwrap();
}

//#[test]
//fn lock_client() {
//unimplemented!();
//}

//#[test]
//fn lock_session() {
//unimplemented!();
//}

#[test]
fn new_session() {
    use crate::tmux_interface::TmuxCommand;

    let tmux = TmuxCommand::new();

    let session_name = "test_new_session";
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();
    tmux.kill_session()
        .target_session(session_name)
        .output()
        .unwrap();
}

//#[test]
//fn refresh_client() {
//unimplemented!();
//}

#[test]
fn rename_session() {
    use crate::tmux_interface::TmuxCommand;

    let session_name = "test_rename_session";
    let new_name = "rename_test_session";
    let tmux = TmuxCommand::new();
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();
    tmux.rename_session()
        .target_session(session_name)
        .new_name(new_name)
        .output()
        .unwrap();
    let has_session = tmux
        .has_session()
        .target_session(new_name)
        .output()
        .unwrap();
    assert_eq!(has_session.0.status.success(), true);
    tmux.kill_session()
        .target_session(new_name)
        .output()
        .unwrap();
}

//#[test]
//fn show_messages() {
//unimplemented!();
//}

//#[test]
//fn source_file() {
//unimplemented!();
//}

//#[test]
//fn start_server() {
//unimplemented!();
//}

//#[test]
//fn suspend_client() {
//unimplemented!();
//}

//#[test]
//fn switch_client() {
//unimplemented!();
//}

#[test]
fn send_keys() {
    use crate::tmux_interface::{TargetPane, TmuxCommand};

    let session_name = "test_send_keys";

    let tmux = TmuxCommand::new();
    tmux.new_session()
        .detached()
        .session_name(session_name)
        .output()
        .unwrap();

    let target_pane = TargetPane::Raw("test_send_keys:^.0").to_string();
    #[cfg(feature = "tmux_1_6")]
    tmux.send_keys()
        .target_pane(&target_pane)
        .key("top")
        .key("C-m")
        .output()
        .unwrap();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    tmux.send_keys()
        .target_window(&target_pane)
        .key("top")
        .key("C-m")
        .output()
        .unwrap();
    tmux.kill_session()
        .target_session(session_name)
        .output()
        .unwrap();
}

//#[test]
//fn list_sessions() {
//use crate::session::Sessions;
//use crate::tmux_interface::TmuxInterface;
//use crate::LIST_SESSIONS_FORMAT;

//let tmux = TmuxInterface::new();
//let sessions_str = tmux.list_sessions(Some(LIST_SESSIONS_FORMAT)).unwrap();
//let sessions = Sessions::parse(&sessions_str).unwrap();
//for session in &sessions {
//if session.id == 0 {
//}
//}
//assert_eq!(sessions[0].id, 0);
//}
