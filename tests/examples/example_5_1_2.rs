#[test]
fn example_5_1_2() {
    use tmux_interface::TmuxCommand;

    // new-session -ADEd -s example_5_1_2
    let mut new_session = TmuxCommand::new();
    new_session
        .name("new-session")
        .push_flag_short('A')
        .push_flag_short('D')
        .push_flag_short('E')
        .push_flag_short('d')
        .arg("-s", "example_5_1_2");

    // tmux -2uv new-session -ADEd -s example_5_1_2
    let mut tmux = TmuxCommand::new();
    tmux.name("tmux")
        .push_flag_short('2')
        .push_flag_short('u')
        .push_flag_short('v')
        .push_cmd(new_session)
        .combine_short_flags();

    let output = tmux.to_command().output().unwrap();

    assert!(output.status.success());

    // kill-session -t example_5_1_2
    let mut kill_session = TmuxCommand::new();
    kill_session.name("kill-session").arg("-t", "example_5_1_2");

    // tmux -2uv kill-session -t example_5_1_2
    let mut tmux = TmuxCommand::new();
    tmux.name("tmux")
        .push_flag_short('2')
        .push_flag_short('u')
        .push_flag_short('v')
        .push_cmd(kill_session)
        .combine_short_flags();

    let output = tmux.to_command().output().unwrap();

    assert!(output.status.success());
}
