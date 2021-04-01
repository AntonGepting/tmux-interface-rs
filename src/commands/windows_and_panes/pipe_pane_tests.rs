#[test]
fn pipe_pane() {
    use crate::{PipePane, TargetPane};
    use std::borrow::Cow;

    // Pipe output sent by the program in target-pane to a shell command or vice versa
    //
    // # Manual
    //
    // tmux ^2.7:
    // ```text
    // tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
    // (alias: pipep)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux pipe-pane [-o] [-t target-pane] [shell-command]
    // (alias: pipep)
    // ```
    //
    // tmux ^1.1:
    // ```text
    // tmux pipe-pane [-o] [-t target-pane] [command]
    // (alias: pipep)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();
    let mut pipe_pane = PipePane::new();
    #[cfg(feature = "tmux_2_7")]
    pipe_pane.stdout();
    #[cfg(feature = "tmux_2_7")]
    pipe_pane.stdin();
    #[cfg(feature = "tmux_1_1")]
    pipe_pane.open();
    #[cfg(feature = "tmux_1_1")]
    pipe_pane.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_2")]
    pipe_pane.shell_command("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "pipe-pane";
    #[cfg(feature = "cmd_alias")]
    let cmd = "pipep";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_7")]
    s.push("-I");
    #[cfg(feature = "tmux_2_7")]
    s.push("-O");
    #[cfg(feature = "tmux_1_1")]
    s.push("-o");
    #[cfg(feature = "tmux_1_1")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("2");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(pipe_pane.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(pipe_pane.0.bin_args, None);
    assert_eq!(pipe_pane.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(pipe_pane.0.cmd_args, Some(s));
}
