#[test]
fn run_shell() {
    use crate::{RunShell, TargetPane};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
    // (alias: run)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux run-shell [-b] [-t target-pane] shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // tmux run-shell shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.1:
    // ```text
    // tmux run-shell command
    // (alias: run)
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let mut run_shell = RunShell::new();
    #[cfg(feature = "tmux_1_8")]
    run_shell.background();
    #[cfg(feature = "tmux_3_2")]
    run_shell.tmux_command();
    #[cfg(feature = "tmux_3_2")]
    run_shell.delay(1);
    #[cfg(feature = "tmux_1_8")]
    run_shell.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_2")]
    run_shell.shell_command("3");
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    run_shell.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "run-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "run";

    let mut s = Vec::new();
    #[cfg(feature = "tmux_1_8")]
    s.push("-b");
    #[cfg(feature = "tmux_3_2")]
    s.push("-C");
    #[cfg(feature = "tmux_3_2")]
    s.extend_from_slice(&["-d", "1"]);
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-t", "2"]);
    #[cfg(feature = "tmux_1_2")]
    s.push("3");
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    s.push("4");
    let s = s.into_iter().map(|a| a.into()).collect();

    //assert_eq!(run_shell.0.bin, Cow::Borrowed("tmux"));
    //assert_eq!(run_shell.0.bin_args, None);
    assert_eq!(run_shell.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(run_shell.0.args, Some(s));
}
