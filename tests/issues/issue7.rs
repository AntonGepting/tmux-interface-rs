// issue 7: Running multiple commands in one invocation

// control mode:
// ```text
// start-server; show-options -g
// ```
//
// default mode:
// ```text
// tmux start-server; show-options -g
// ```

// prepare command for execution in control mode
// next step could be .output() in control mode
#[test]
fn issue7_control_mode() {
    use std::borrow::Cow;
    use tmux_interface::{ShowOptions, StartServer, TmuxCommands};

    let show_options = ShowOptions::new();
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options.value();
    #[cfg(feature = "tmux_1_7")]
    let show_options = show_options.option("default-terminal");
    let show_options = show_options.build();

    let cmds = TmuxCommands::new();
    let cmds = cmds.add_command(StartServer::new().build());
    let cmds = cmds.add_command(show_options);

    // dbg!(&cmds.to_string());

    #[cfg(not(feature = "cmd_alias"))]
    let cmd1 = "start-server";
    #[cfg(feature = "cmd_alias")]
    let cmd1 = "start";

    #[cfg(not(feature = "cmd_alias"))]
    let cmd2 = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd2 = "show";

    let mut s = Vec::new();
    s.push(cmd1);
    s.push(";");
    s.push(cmd2);
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_1_7")]
    s.push("default-terminal");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(s, cmds.to_vec())
}

// prepare and execute command in default mode
#[test]
fn issue7_default_mode() {
    use std::borrow::Cow;
    use tmux_interface::{ShowOptions, StartServer, Tmux};

    let show_options = ShowOptions::new();
    #[cfg(feature = "tmux_1_8")]
    let show_options = show_options.value();
    #[cfg(feature = "tmux_1_7")]
    let show_options = show_options.option("default-terminal");
    let show_options = show_options.build();

    let cmds = Tmux::new();
    let cmds = cmds.add_command(StartServer::new().build());
    let cmds = cmds.add_command(show_options);
    let cmds = cmds.build();

    // dbg!(&cmds.to_string());

    #[cfg(not(feature = "cmd_alias"))]
    let cmd1 = "start-server";
    #[cfg(feature = "cmd_alias")]
    let cmd1 = "start";

    #[cfg(not(feature = "cmd_alias"))]
    let cmd2 = "show-options";
    #[cfg(feature = "cmd_alias")]
    let cmd2 = "show";

    let mut s = Vec::new();
    s.push("tmux");
    s.push(cmd1);
    s.push(";");
    s.push(cmd2);
    #[cfg(feature = "tmux_1_8")]
    s.push("-v");
    #[cfg(feature = "tmux_1_7")]
    s.push("default-terminal");
    let s: Vec<Cow<str>> = s.into_iter().map(|a| a.into()).collect();

    assert_eq!(s, cmds.to_vec())
}
