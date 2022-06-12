#[test]
fn list_commands() {
    use crate::ListCommands;
    use std::borrow::Cow;

    // List the syntax of all commands supported by tmux
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // tmux list-commands [-F format] [command]
    // (alias: lscm)
    // ```
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
    let mut list_commands = list_commands.format("1");
    #[cfg(feature = "tmux_3_2")]
    let mut list_commands = list_commands.command("2");

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-commands";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lscm";

    let mut s = Vec::new();
    s.push(cmd);
    #[cfg(feature = "tmux_2_3")]
    s.extend_from_slice(&["-F", "1"]);
    #[cfg(feature = "tmux_3_2")]
    s.push("2");
    //#[cfg(feature = "tmux_2_3")]
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    let list_commands = list_commands.build().to_vec();

    assert_eq!(list_commands, s);
}
