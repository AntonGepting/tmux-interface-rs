#[test]
fn example_3() {
    use std::borrow::Cow;
    use tmux_interface::{KillSession, NewSession, NewWindow, SplitWindow, Tmux};

    let target_session = "example_3";
    let target_window = "example_3_window_1";

    // tmux new-session -d -s example_3 ;
    //              new-window -n example_3_window_1 ;
    //              split-window -t example_3_window_1 ;
    //              kill-session -t example_3

    let new_session = NewSession {
        detached: true,
        session_name: Some(Cow::Borrowed(target_session)),
        ..Default::default()
    };

    let new_window = NewWindow {
        window_name: Some(Cow::Borrowed(target_window)),
        ..Default::default()
    };

    let split_window = SplitWindow {
        target_window: Some(Cow::Borrowed(target_window)),
        ..Default::default()
    };

    let kill_session = KillSession {
        target_session: Some(Cow::Borrowed(target_session)),
        ..Default::default()
    };

    let tmux = Tmux {
        verbose_logging: true,
        ..Default::default()
    };

    let tmux = tmux
        .add_command(new_session)
        .add_command(new_window)
        .add_command(split_window)
        .add_command(kill_session);

    let output = tmux.output().unwrap();

    assert!(output.success())
}
