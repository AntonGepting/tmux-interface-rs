#[test]
fn tmux_commands() {
    use crate::commands::tmux_bin_commands::TmuxBinCommands;
    use crate::commands::tmux_commands::TmuxCommands;
    use crate::{NewSession, SessionOptions, ShowOptions, StartServer, TmuxCommand};
    use std::borrow::Cow;

    let mut cmd = TmuxCommand::new();
    let mut cmd = cmd.start_server();

    let mut cmds = TmuxCommands::new();

    let mut cmd = StartServer::new();
    cmd.0.args = Some(vec![Cow::Borrowed("-v")]);
    cmds.push(cmd.0);

    //let mut cmd = NewSession::new();
    //cmd.session_name("session1").detached();
    //cmds.push(cmd);

    //let mut cmd = NewSession::new();
    //cmd.session_name("session2").detached();
    //cmds.push(cmd);

    let mut cmd = ShowOptions::new();
    cmd.global().option("default-shell");
    dbg!(&cmd);
    cmds.push(cmd.0);

    dbg!(&cmds);

    //let result = cmds.to_tmux_bin_commands().output().unwrap();

    //dbg!(&result);

    //let session_options = result.to_string().parse::<SessionOptions>();
    //dbg!(&session_options);

    //let v = cmds.to_vec();
    //dbg!(&v);
}

// build commands, push commands
#[test]
fn tmux_commands_push1() {
    use crate::commands::tmux_commands::TmuxCommands;
    use crate::{HasSession, KillSession, NewSession, TmuxCommand};

    let mut new_session = NewSession::new();
    new_session.detached().session_name("session");
    let mut has_session = HasSession::new();
    has_session.target_session("session");
    let mut kill_session = KillSession::new();
    kill_session.target_session("session");

    let mut cmds = TmuxCommands::new();
    cmds.push(new_session.0);
    cmds.push(has_session.0);
    cmds.push(kill_session.0);

    dbg!(cmds);
}

// NOTE:
//  error move occurs because value has type `TmuxCommand<'_>`, which does not implement the `Copy`
//  trait
//
// fix: impl from &mut NewSession into TmuxCommand trait
//
#[test]
fn tmux_commands_push2() {
    use crate::commands::tmux_commands::TmuxCommands;
    use crate::{HasSession, KillSession, NewSession, TmuxCommand};

    let mut cmds = TmuxCommands::new();
    cmds.push(NewSession::new().detached().session_name("session"));
    cmds.push(HasSession::new().target_session("session"));
    cmds.push(KillSession::new().target_session("session"));

    dbg!(cmds);
}

// nice would be?
//#[test]
//fn tmux_commands_push2() {
//use crate::commands::tmux_commands::TmuxCommands;
//use crate::{HasSession, KillSession, NewSession, TmuxCommand};

//let mut cmds = TmuxCommands::new();
//cmds.new_session().detached().session_name("session");
//cmds.has_session().target_session("session");
//cmds.kill_session().target_session("session");

//dbg!(cmds);
//}
