#[test]
fn get_panes() {
    use tmux_interface::response::pane::pane::PANE_ALL;
    use tmux_interface::{NewSessionBuilder, Panes, TargetSession, TargetWindowExt, TmuxInterface};
    use tmux_interface::{SessionOptionsBuilder, BASE_INDEX};

    const TARGET_SESSION: &str = "test_get_panes";
    const WINDOW_INDEX: usize = 1;

    let target_session = TargetSession::Raw(TARGET_SESSION);
    let target_window = TargetWindowExt::id(Some(&target_session), WINDOW_INDEX);

    let mut tmux = TmuxInterface::new();

    SessionOptionsBuilder::new()
        .base_index(WINDOW_INDEX)
        .build()
        .set(BASE_INDEX)
        .unwrap();

    let new_session = NewSessionBuilder::new()
        .detached()
        .session_name(TARGET_SESSION)
        .build();
    tmux.new_session(Some(&new_session)).unwrap();
    assert_eq!(tmux.has_session(Some(&target_session)).unwrap(), true);

    let _panes = Panes::get(&target_window, PANE_ALL).unwrap();

    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
