#[test]
fn get_panes() {
    use tmux_interface::response::window::window::WINDOW_ALL;
    use tmux_interface::{NewSession, TargetSession, TmuxInterface, Windows};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_get_windows"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let _windows = Windows::get(&TargetSession::Raw("0"), WINDOW_ALL).unwrap();
    //assert_eq!(tmux.has_session(Some("test_has_session")).unwrap(), true);
    tmux.kill_session(None, None, Some(&TargetSession::Raw("test_get_windows")))
        .unwrap();
}
