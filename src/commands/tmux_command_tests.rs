#[test]
fn tmux_command_to_string() {
    use crate::TmuxCommand;

    let mut tmux = TmuxCommand::new();
    tmux.name("cmd");
    tmux.push_flag_short('c');
    tmux.push_flag_short('d');
    assert_eq!(tmux.to_string(), "cmd -cd");
    tmux.not_combine_short_flags();
    assert_eq!(tmux.to_string(), "cmd -c -d");
}

#[test]
fn tmux_command_to_vec() {
    use crate::TmuxCommand;

    let mut tmux = TmuxCommand::new();
    tmux.name("cmd");
    tmux.push_flag("--a");
    tmux.push_flag("--b");
    tmux.push_param("param");
    tmux.push_flag("--c");
    assert_eq!(tmux.to_vec(), vec!["cmd", "--a", "--b", "param", "--c"]);
}

// check `process::Command::output()` response if command is empty
// command: ""
// error: `Os { code: 2, kind: NotFound, message: "No such file or directory" }`
//#[test]
//fn empty_command() {
//use std::process::Command;

//let mut cmd = Command::new("");
//let output = cmd.output();
//dbg!(output);
//}

#[test]
fn tmux_multiple_subcommands() {
    #[cfg(feature = "tmux_2_3")]
    use crate::ListCommands;
    use crate::Tmux;

    let tmux = Tmux::new().verbose_logging().version();
    #[cfg(feature = "tmux_2_3")]
    let tmux = tmux.command(ListCommands::new().format("listcommands1"));
    #[cfg(feature = "tmux_2_3")]
    let tmux = tmux.command(ListCommands::new().format("listcommands2"));
    let tmux = tmux.build().to_string();

    #[cfg(not(feature = "cmd_alias"))]
    let cmd = "list-commands";
    #[cfg(feature = "cmd_alias")]
    let cmd = "lscm";

    let s = vec![
        "tmux",
        "-v",
        "-V",
        cmd,
        "-F",
        "listcommands1",
        ";",
        cmd,
        "-F",
        "listcommands2",
    ]
    .join(" ");

    assert_eq!(tmux, s);
}

#[test]
fn tmux_command() {
    use crate::{KillSession, NewSession, Tmux};

    let output = Tmux::with_command(NewSession::new().session_name("asdf").detached())
        .output()
        .unwrap();
    assert!(output.status().success());

    let output = Tmux::with_command(KillSession::new().target_session("asdf"))
        .output()
        .unwrap();
    assert!(output.status().success());
}
