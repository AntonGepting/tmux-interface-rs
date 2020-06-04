extern crate tmux_interface;
use crate::tmux_interface::TmuxInterface;

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
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    //tmux.tmux = Some("./tests/tmux_mock.sh");
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_has_session"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    assert_eq!(tmux.has_session(Some("test_has_session")).unwrap(), true);
    tmux.kill_session(None, None, Some("test_has_session"))
        .unwrap();
}

//#[test]
//fn kill_server() {
//unimplemented!();
//}

#[test]
fn kill_session() {
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_kill_session"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    tmux.kill_session(None, None, Some("test_kill_session"))
        .unwrap();
}

#[test]
fn callback() {
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_kill_session"),
        ..Default::default()
    };

    tmux.pre_hook = Some(Box::new(|bin, options, subcmd| {
        *bin = "./tests/tmux_test.sh".to_string();
        println!("callback() prehook: {:?} {:?} {:?}", bin, options, subcmd);
        //Err(Error::new("hook"))
        Ok(None)
    }));

    tmux.new_session(Some(&new_session)).unwrap();
    tmux.kill_session(None, None, Some("test_kill_session"))
        .unwrap();
}

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
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_list_sessions"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    tmux.list_sessions(None).unwrap();
    tmux.kill_session(None, None, Some("test_list_session"))
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
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_new_session"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    tmux.kill_session(None, None, Some("test_new_session"))
        .unwrap();
}

//#[test]
//fn refresh_client() {
//unimplemented!();
//}

#[test]
fn rename_session() {
    use crate::tmux_interface::NewSession;

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_rename_session"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    tmux.rename_session(Some("test_rename_session"), "rename_test_session")
        .unwrap();
    assert_eq!(tmux.has_session(Some("rename_test_session")).unwrap(), true);
    tmux.kill_session(None, None, Some("rename_test_session"))
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
    use crate::tmux_interface::{NewSession, SendKeys, TargetPane};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_send_keys"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let target_pane = TargetPane::Raw("test_send_keys:^.0").to_string();
    let send_keys = SendKeys {
        target_pane: Some(&target_pane),
        ..Default::default()
    };
    tmux.send_keys(Some(&send_keys), &vec!["top", "C-m"])
        .unwrap();
    tmux.kill_session(None, None, Some("test_send_keys"))
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
