// command sequence, Tmux struct as parent, commands structs (NewSession, NewWindow...) as children
#[test]
fn example1() {
    use tmux_interface::{HasSession, KillSession, NewSession, NewWindow, SplitWindow, Tmux};

    let target_session = "example_1";

    // ```text
    // tmux new -d -s example_1 ; neww ; splitw -v
    // ```
    Tmux::new()
        .add_command(NewSession::new().detached().session_name(target_session))
        .add_command(NewWindow::new())
        .add_command(SplitWindow::new().vertical())
        .output()
        .unwrap();

    // ```text
    // tmux has -t example_1
    // ```
    let status = Tmux::with_command(HasSession::new().target_session(target_session))
        .status()
        .unwrap()
        .success();

    assert!(status);

    // ```text
    // tmux kill-session -t example_1
    // ```
    Tmux::with_command(KillSession::new().target_session(target_session))
        .output()
        .unwrap();
}
