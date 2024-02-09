#[cfg(feature = "tmux_1_6")]
#[test]
fn get_panes() {
    use tmux_interface::{KillSession, NewSession, PanesCtl, SplitWindow, Tmux};

    const TARGET_SESSION: &str = "get_panes_test";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    Tmux::with_command(SplitWindow::new()).output().unwrap();

    let panes = PanesCtl::new().get_all().unwrap();
    let mut found = false;
    for _pane in panes {
        // if pane.current_command == Some(COMMAND.to_string()) {
        found = true;
        // }
    }
    assert!(found);

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
