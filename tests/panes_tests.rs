#[cfg(feature = "tmux_1_6")]
#[test]
fn get_panes() {
    use tmux_interface::PANE_ALL;
    use tmux_interface::{Panes, TargetSession, TargetWindowExt, TmuxCommand};
    use tmux_interface::{SessionOptionsBuilder, BASE_INDEX};

    const TARGET_SESSION: &str = "test_get_panes";
    const WINDOW_INDEX: usize = 1;

    let target_session = TargetSession::Raw(TARGET_SESSION);
    let target_session_str = target_session.to_string();
    let target_window = TargetWindowExt::id(Some(&target_session), WINDOW_INDEX);

    let tmux = TmuxCommand::new();

    SessionOptionsBuilder::new()
        .base_index(WINDOW_INDEX)
        .build()
        .set(BASE_INDEX)
        .unwrap();

    tmux.new_session()
        .detached()
        .session_name(&target_session_str)
        //.session_name(TARGET_SESSION)
        .output()
        .unwrap();

    let has_session = tmux
        .has_session()
        .target_session(&target_session_str)
        .output()
        .unwrap();
    assert_eq!(has_session.success(), true);

    let _panes = Panes::get(&target_window, PANE_ALL).unwrap();

    tmux.kill_session()
        .target_session(&target_session_str)
        .output()
        .unwrap();
}
