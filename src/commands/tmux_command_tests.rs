#[test]
fn to_string() {
    use crate::commands::tmux_command::Args;
    use crate::TmuxCommand;

    let mut tmux = TmuxCommand::new();
    tmux.cmd("cmd");
    tmux.push_flag("-c");
    tmux.push_flag("-d");
    assert_eq!(tmux.to_string(), "cmd -c -d");
}
