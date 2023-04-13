#[cfg(feature = "tmux_1_6")]
#[test]
fn get_panes() {
    use tmux_interface::{KillSession, NewSession, PanesCtl, SplitWindow, Tmux};

    const TARGET_SESSION: &str = "test_get_panes";
    const COMMAND: &str = "top";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    Tmux::with_command(SplitWindow::new().shell_command(COMMAND))
        .output()
        .unwrap();

    let panes = PanesCtl::new().get_all().unwrap();
    let mut found = false;
    for pane in panes {
        if pane.current_command == Some(COMMAND.to_string()) {
            found = true;
        }
    }
    assert!(found);

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
