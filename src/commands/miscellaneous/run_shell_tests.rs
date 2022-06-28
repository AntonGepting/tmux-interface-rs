#[test]
fn run_shell() {
    use crate::{RunShell, TargetPane};
    use std::borrow::Cow;

    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
    // (alias: run)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // run-shell [-b] [-t target-pane] shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.2:
    // ```text
    // run-shell shell-command
    // (alias: run)
    // ```
    //
    // tmux ^1.1:
    // ```text
    // run-shell command
    // (alias: run)
    // ```
    let target_pane = TargetPane::Raw("2").to_string();

    let run_shell = RunShell::new();
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell.background();
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell.tmux_command();
    #[cfg(feature = "tmux_3_2")]
    let run_shell = run_shell.delay(1);
    #[cfg(feature = "tmux_1_8")]
    let run_shell = run_shell.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_2")]
    let run_shell = run_shell.shell_command("3");
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    let run_shell = run_shell.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "run-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "run";

    let mut s = Vec::new();
    s.push(cmd);
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
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let run_shell = run_shell.build().to_vec();

    assert_eq!(run_shell, s);
}
