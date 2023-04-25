#[test]
fn example_2() {
    use tmux_interface::{KillSession, NewSession, NewWindow, SplitWindow, Tmux};

    let target_session = "example_2";
    let target_window = "example_2_window_1";

    // tmux new-session -d -s example_2 ;
    //              new-window -n example_2_window_1 ;
    //              split-window -t example_2_window_1 ;
    //              kill-session -t example_2
    let output = Tmux::new()
        .add_command(NewSession::new().detached().session_name(target_session))
        .add_command(NewWindow::new().window_name(target_window))
        .add_command(SplitWindow::new().target_window(target_window))
        .add_command(KillSession::new().target_session(target_session))
        .output()
        .unwrap();

    assert!(output.success())
}
