#[test]
fn get_panes() {
    use tmux_interface::response::pane::pane::PANE_ALL;
    use tmux_interface::{NewSession, Panes, TargetSession, TargetWindowEx, TmuxInterface};

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some("test_get_panes"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let _panes = Panes::get(&TargetWindowEx::raw("0:1"), PANE_ALL).unwrap();
    //assert_eq!(tmux.has_session(Some("test_has_session")).unwrap(), true);
    tmux.kill_session(None, None, Some(&TargetSession::Raw("test_get_panes")))
        .unwrap();
}
