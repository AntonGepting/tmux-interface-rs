#[test]
fn example_5_1_4() {
    use tmux_interface::{kill_session, new_session, tmux};

    let session_name = "example_5_1_4";

    // tmux -2uv new-session -ADEd -s example_5_1_4
    let tmux = tmux!(-2, -u, -v, new_session!(-A, -D, -E, -d, -s session_name));

    let output = tmux.output().unwrap();

    assert!(output.success());

    // tmux -2uv kill-session -t example_5_1_4
    let tmux = tmux!(-2, -u, -v, kill_session!(-t session_name));

    let output = tmux.output().unwrap();

    assert!(output.success());
}
