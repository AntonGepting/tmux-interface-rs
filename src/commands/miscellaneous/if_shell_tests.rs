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
    // tmux if-shell [-bF] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.8:
    // ```text
    // tmux if-shell [-b] [-t target-pane] shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^1.6:
    // ```text
    // tmux if-shell shell-command command [command]
    // (alias: if)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux if-shell shell-command command
    // (alias: if)
    // ```
    let target_pane = TargetPane::Raw("1").to_string();

    let mut if_shell = IfShell::new();
    #[cfg(feature = "tmux_1_8")]
    if_shell.backgroud();
    #[cfg(feature = "tmux_2_0")]
    if_shell.not_execute();
    #[cfg(feature = "tmux_1_8")]
    if_shell.target_pane(&target_pane);
    #[cfg(feature = "tmux_1_6")]
    if_shell.shell_command("2");
    #[cfg(feature = "tmux_0_8")]
    if_shell.command("3");
    #[cfg(feature = "tmux_0_8")]
    if_shell.command("4");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "if-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "if";

    let mut s = Vec::new();
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
    #[cfg(feature = "tmux_1_6")]
    s.push("4");
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(if_shell.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(if_shell.0.bin_args, None);
    assert_eq!(if_shell.0.cmd, Some(Cow::Borrowed(cmd)));
    assert_eq!(if_shell.0.cmd_args, Some(s));
}
