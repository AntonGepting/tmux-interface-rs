#[test]
fn if_shell() {
    use crate::{IfShell, TargetPane};
    use std::borrow::Cow;

    // Structure for conditional commands executing
    //
    // # Manual
    //
    // tmux ^2.0:
    // ```text
    // if-shell [-bF] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // if-shell [-b] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // if-shell shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // if-shell shell-command command
    // (alias: if)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let if_shell = IfShell::new();
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell.background();
    #[cfg(feature = "tmux_2_0")]
    let if_shell = if_shell.not_execute();
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_6")]
    let if_shell = if_shell.shell_command("2");
    #[cfg(feature = "tmux_0_8")]
    let if_shell = if_shell.command("3");
    //#[cfg(feature = "tmux_1_6")]
    //if_shell.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "if-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "if";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    s.push("-b");
    #[cfg(feature = "tmux_2_0")]
    s.push("-F");
    #[cfg(feature = "tmux_1_8")]
    s.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_0_8")]
    s.push("2");
    #[cfg(feature = "tmux_0_8")]
    s.push("3");
    //#[cfg(feature = "tmux_1_6")]
    //s.push("4");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let if_shell = if_shell.build().to_vec();

    assert_eq!(if_shell, s);
}
