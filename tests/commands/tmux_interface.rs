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
    use tmux_interface::{HasSession, KillSession, NewSession, Tmux};

    let session_name = "test_has_session";

    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();
    let has_session = Tmux::with_command(HasSession::new().target_session(session_name))
        .output()
        .unwrap();

    assert_eq!(has_session.0.status.success(), true);

    Tmux::with_command(KillSession::new().target_session(session_name))
        .output()
        .unwrap();
}

//#[test]
//fn kill_server() {
//unimplemented!();
//}

#[test]
fn kill_session() {
    use tmux_interface::{KillSession, NewSession, Tmux};
    let session_name = "test_kill_session";

    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();
    Tmux::with_command(KillSession::new().target_session(session_name))
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
    use tmux_interface::{KillSession, ListSessions, NewSession, Tmux};

    let session_name = "test_list_sessions";
    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();
    Tmux::with_command(ListSessions::new()).output().unwrap();
    Tmux::with_command(KillSession::new().target_session(session_name))
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
    use crate::tmux_interface::{KillSession, NewSession, Tmux};

    let session_name = "test_new_session";
    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();
    Tmux::with_command(KillSession::new().target_session(session_name))
        .output()
        .unwrap();
}

//#[test]
//fn refresh_client() {
//unimplemented!();
//}

#[test]
fn rename_session() {
    use tmux_interface::{KillSession, NewSession, RenameSession, Tmux};

    let session_name = "test_rename_session";
    let new_name = "rename_test_session";
    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();
    Tmux::with_command(
        RenameSession::new()
            .target_session(session_name)
            .new_name(new_name),
    )
    .output()
    .unwrap();
    // let has_session = Tmux::with_command(HasSession::new().target_session(new_name))
    // .output()
    // .unwrap();
    //assert_eq!(has_session.0.status.success(), true);
    Tmux::with_command(KillSession::new().target_session(new_name))
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
    //use tmux_interface::commands::tmux_bin_command::TmuxBinCommand;
    use tmux_interface::TargetPane;
    use tmux_interface::{KillSession, NewSession, SendKeys, Tmux};

    let session_name = "test_send_keys";

    //let tmux = TmuxBinCommand::new();
    Tmux::with_command(NewSession::new().detached().session_name(session_name))
        .output()
        .unwrap();

    let target_pane = TargetPane::Raw("test_send_keys:^.0").to_string();
    #[cfg(feature = "tmux_1_6")]
    Tmux::with_command(
        SendKeys::new()
            .target_pane(&target_pane)
            .key("top")
            .key("C-m"),
    )
    .output()
    .unwrap();
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_6")))]
    Tmux::with_command(
        SendKeys::new()
            .target_window(&target_pane)
            .key("top")
            .key("C-m"),
    )
    .output()
    .unwrap();
    Tmux::with_command(KillSession::new().target_session(session_name))
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
