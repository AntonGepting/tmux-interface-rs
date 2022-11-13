/// List the syntax of all commands supported by tmux
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// list-commands [-F format] [command]
/// (alias: lscm)
/// ```
///
/// tmux ^2.3:
/// ```text
/// list-commands [-F format]
/// (alias: lscm)
/// ```
///
/// tmux ^0.8:
/// ```text
/// list-commands
/// (alias: lscm)
/// ```
#[macro_export]
macro_rules! list_commands {
    (@cmd ($cmd:expr) -F $format:expr, $($tail:tt)*) => {{
        $crate::list_commands!(@cmd ({
            $cmd.format($format)
        }) $($tail)*)
    }};
    // `[-s target-session]` - specify the session, all clients currently attached
    (@cmd ($cmd:expr) $command:expr, $($tail:tt)*) => {{
        $crate::list_commands!(@cmd ({
            $cmd.command($command)
        }) $($tail)*)
    }};
    //(@cmd ($cmd:expr) -$unknown:tt, $($tail:tt)*) => {{
        //::std::compile_error!("unknown flag, option or parameter");
    //}};
    (@cmd ($cmd:expr)) => {{
        $cmd
    }};
    () => {{
        $crate::ListCommands::new()
    }};
    (($cmd:expr), $($tail:tt)*) => {{
        $crate::list_commands!(@cmd ($cmd) $($tail)*,)
    }};
    ($($tail:tt)*) => {{
        $crate::list_commands!(@cmd ({ $crate::ListCommands::new() }) $($tail)*,)
    }};

}

#[test]
fn list_commands_macro() {
    use crate::ListCommands;
    use std::borrow::Cow;

    // List the syntax of all commands supported by tmux
    //
    // # Manual
    //
    // tmux ^3.2:
    // ```text
    // list-commands [-F format] [command]
    // (alias: lscm)
    // ```
    //
    // tmux ^2.3:
    // ```text
    // list-commands [-F format]
    // (alias: lscm)
    // ```
    //
    // tmux ^0.8:
    // ```text
    // list-commands
    // (alias: lscm)
    // ```
    let mut list_commands = list_commands!();
    #[cfg(feature = "tmux_2_3")]
    let mut list_commands = list_commands!((list_commands), -F "1");
    #[cfg(feature = "tmux_3_2")]
    let mut list_commands = list_commands!((list_commands), "2");

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
