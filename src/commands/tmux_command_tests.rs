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
fn tmux_subcommands() {
    use crate::TmuxCommand;

    let mut tmux = TmuxCommand::new();
    tmux.name("tmux");
    tmux.push_flag("--a");
    tmux.push_flag("--b");

    //assert_eq!(tmux)
}

#[test]
fn tmux_multiple_subcommands() {
    use crate::{ListCommands, Tmux};

    let tmux = Tmux::new()
        .verbose_logging()
        .version()
        .command(ListCommands::new().format("listcommands1").build())
        .command(ListCommands::new().format("listcommands2").build())
        .build()
        .to_string();

    assert_eq!(
        tmux,
        "tmux -v -V lscm -F listcommands1 ; lscm -F listcommands2"
    );
}

#[test]
fn tmux_command() {
    use crate::{KillSession, NewSession, Tmux};

    let cmd = Tmux::new()
        .command(NewSession::new().session_name("asdf").detached().build())
        .build();
    let output = cmd.output();
    dbg!(output);

    let cmd = Tmux::new()
        .command(KillSession::new().target_session("asdf").build())
        .build();
    let output = cmd.output();

    dbg!(output);

    //assert_eq!(
    //tmux,
    //"tmux -v -V lscm -F listcommands1 ; lscm -F listcommands2"
    //);
}
