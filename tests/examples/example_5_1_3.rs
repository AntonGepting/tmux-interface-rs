#[test]
fn example_5_1_3() {
    use tmux_interface::{KillSession, NewSession, Tmux};

    let session_name = "example_5_1_3";

    // tmux -2uv new-session -ADEd -s example_5_1_3
    let tmux = Tmux::with_command(
        NewSession::new()
            .attach()
            .detach_other()
            .not_update_env()
            .detached()
            .session_name(session_name),
    )
    .colours256()
    .force_utf8()
    .verbose_logging();

    let output = tmux.output().unwrap();

    assert!(output.success());

    // tmux -2uv kill-session -t example_5_1_3
    let tmux = Tmux::with_command(KillSession::new().target_session(session_name))
        .colours256()
        .force_utf8()
        .verbose_logging();

    let output = tmux.output().unwrap();

    assert!(output.success());
}
