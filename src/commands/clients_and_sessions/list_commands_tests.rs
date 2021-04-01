#[test]
fn list_commands() {
    use crate::ListCommands;
    use std::borrow::Cow;

    // List the syntax of all commands supported by tmux
    //
    // # Manual
    //
    // tmux ^2.3:
    // ```text
    // tmux list-commands [-F format]
    // (alias: lscm)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // tmux list-commands
    // (alias: lscm)
    // ```
    let mut list_commands = ListCommands::new();
    #[cfg(feature = "tmux_2_3")]
    list_commands.format("1");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-commands";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lscm";

    #[cfg(feature = "tmux_2_3")]
    let mut s = Vec::new();
    #[cfg(feature = "tmux_2_3")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_2_3")]
    let s = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(list_commands.0.bin, Cow::Borrowed("tmux"));
    assert_eq!(list_commands.0.bin_args, None);
    assert_eq!(list_commands.0.cmd, Some(Cow::Borrowed(cmd)));
    #[cfg(feature = "tmux_2_3")]
    assert_eq!(list_commands.0.cmd_args, Some(s));
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_2_3")))]
    assert_eq!(list_commands.0.cmd_args, None);
}
