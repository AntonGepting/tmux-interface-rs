#[test]
fn example_4() {
    use tmux_interface::{kill_session, new_session, new_window, split_window, tmux};

    let target_session = "example_4";
    let target_window = "example_4_window_1";

    // tmux new-session -d -s example_4 ;
    //              new-window -n example_4_window_1 ;
    //              split-window -t example_4_window_1 ;
    //              kill-session -t example_4
    let tmux = tmux!(
        new_session!(-d, -s target_session),
        new_window!(-n target_window),
        split_window!(-t target_window),
        kill_session!(-t target_session)
    );
    let output = tmux.output().unwrap();

    assert!(output.success())
}
