// auto-generated file
//

// Conditional commands executing
//
// # Manual
//
// tmux >=2.0:
// ```text
// if-shell [-bF] [-t target-pane] shell-command command [command]
// (alias: if)
// ```
//
// tmux >=1.8:
// ```text
// if-shell [-b] [-t target-pane] shell-command command [command]
// (alias: if)
// ```
//
// tmux >=1.6:
// ```text
// if-shell shell-command command [command]
// (alias: if)
// ```
//
// tmux >=1.5:
// ```text
// if-shell shell-command command
// (alias: if)
// ```
#[test]
fn if_shell() {
    use crate::IfShell;
    use std::borrow::Cow;

    let if_shell = IfShell::new();
    // `[-b]`
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell.background();

    // `[-F]`
    #[cfg(feature = "tmux_2_0")]
    let if_shell = if_shell.not_execute();

    // `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    let if_shell = if_shell.target_pane("1");

    // `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    let if_shell = if_shell.shell_command("2");

    // `[command]`
    #[cfg(feature = "tmux_1_5")]
    let if_shell = if_shell.command("3");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "if-shell";
    #[cfg(feature = "cmd_alias")]
    let cmd = "if";

    let mut v = Vec::new();
    v.push(cmd);
    #[cfg(feature = "tmux_1_8")]
    v.push("-b");
    #[cfg(feature = "tmux_2_0")]
    v.push("-F");
    #[cfg(feature = "tmux_1_8")]
    v.extend_from_slice(&["-t", "1"]);
    #[cfg(feature = "tmux_1_5")]
    v.push("2");
    #[cfg(feature = "tmux_1_5")]
    v.push("3");
    let v: Vec<Cow<str>> = v.into_iter().map(|a| a.into()).collect();

    let if_shell = if_shell.build().to_vec();

    assert_eq!(if_shell, v);
}
