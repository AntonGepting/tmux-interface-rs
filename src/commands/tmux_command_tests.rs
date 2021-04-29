#[test]
fn to_string() {
    use crate::commands::tmux_command::Args;
    use crate::TmuxCommand;

    let mut tmux = TmuxCommand::new();
    tmux.bin("bin");
    tmux.bin_args.push_flag("-a");
    tmux.bin_args.push_flag("-b");
    tmux.cmd("cmd");
    tmux.push_flag("-c");
    tmux.push_flag("-d");
    //assert_eq!(tmux.to_string(), "bin -a -b cmd -c -d");
    dbg!(tmux.to_string());
}
