#[cfg(feature = "tmux_1_6")]
#[test]
fn get_windows() {
    use tmux_interface::{KillSession, NewSession, NewWindow, Tmux, WindowsCtl};

    const TARGET_SESSION: &str = "test_get_windows";

    Tmux::with_command(NewSession::new().detached().session_name(TARGET_SESSION))
        .output()
        .unwrap();

    Tmux::with_command(NewWindow::new().window_name(TARGET_SESSION))
        .output()
        .unwrap();

    let windows = WindowsCtl::new().get_all().unwrap();
    let mut found = false;
    for window in windows {
        if window.name == Some(TARGET_SESSION.to_string()) {
            found = true;
        }
    }
    assert!(found);

    Tmux::with_command(KillSession::new().target_session(TARGET_SESSION))
        .output()
        .unwrap();
}
