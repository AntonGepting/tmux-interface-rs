#[test]
fn tmux_commands() {
    use crate::commands::TmuxCommandList;
    use crate::ShowOptions;

    //let cmd = StartServer::new().build();

    let mut cmds = TmuxCommandList::new();

    //let mut cmd = StartServer::new();
    //cmd.args = Some(vec![Cow::Borrowed("-v")]);
    //cmds.push(cmd.build());

    //let mut cmd = NewSession::new();
    //cmd.session_name("session1").detached();
    //cmds.push(cmd);

    //let mut cmd = NewSession::new();
    //cmd.session_name("session2").detached();
    //cmds.push(cmd);

    let cmd = ShowOptions::new().global().option("default-shell").build();
    dbg!(&cmd);
    cmds.push(cmd);

    dbg!(&cmds);

    //let result = cmds.to_tmux_bin_commands().output().unwrap();

    //dbg!(&result);

    //let session_options = result.to_string().parse::<SessionOptions>();
    //dbg!(&session_options);

    //let v = cmds.to_vec();
    //dbg!(&v);
}

// First variant:
// 1. build commands
// 2. push commands
//
#[test]
fn tmux_commands_push1() {
    use crate::commands::tmux_command_list::TmuxCommandList;
    use crate::{HasSession, KillSession, NewSession, TmuxCommand};

    let tmux_command = TmuxCommand::new();

    let mut cmds = TmuxCommandList::new();
    cmds.push(NewSession::new().detached().session_name("session").build());
    cmds.push(HasSession::new().target_session("session").build());
    cmds.push(KillSession::new().target_session("session").build());
    cmds.push(tmux_command);

    //let cmds = cmds.to_vec();
    dbg!(cmds);
}

// NOTE:
//  error move occurs because value has type `TmuxCommand<'_>`, which does not implement the `Copy`
//  trait
//
// fix: impl from &mut NewSession into TmuxCommand trait
//
// Second variant:
// 1. push in place created
//#[test]
//fn tmux_commands_push2() {
//use crate::commands::tmux_commands::TmuxCommands;
//use crate::{HasSession, KillSession, NewSession, TmuxCommand};

//let mut cmds = TmuxCommands::new();
//cmds.push(NewSession::new().detached().session_name("session").build());
//cmds.push(HasSession::new().target_session("session").build());
//cmds.push(KillSession::new().target_session("session").build());
//cmds.push(TmuxCommand::new());

//dbg!(cmds);
//}

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
//
//
// NOTE: from bin
//// TODO: All in one variants demo
//
//#[test]
//fn tmux_bin_commands() {
//use crate::commands::tmux_bin_commands::TmuxBinCommands;
//use crate::{HasSession, KillSession, ListSessions, NewSession};

//let target_session = "example2";
//let target_session = String::from("example2");

//let mut cmds = TmuxBinCommands::new();
//cmds.push(NewSession::new().detached().session_name(target_session));
//cmds.push(HasSession::new().target_session(target_session));
//cmds.push(KillSession::new().target_session(target_session));
//cmds.output().unwrap();
//}

//#[test]
//fn tmux_bin_commands2() {
//use crate::commands::tmux_bin_commands::TmuxBinCommands;
//use crate::{ListCommands, ShowOptions, StartServer};

////let tmux = TmuxBin::default();
////let mut tmux = TmuxBin::new();
////tmux.bin("tmux");
////tmux.bin = Some("tmux");
////let mut cmds = TmuxBinCommands::from(tmux);

//let mut cmds = TmuxBinCommands::default();
//cmds.tmux.verbose();

//// E0716
////let list_commands = ListCommands::new().format("1");
////cmds.push(list_commands);

//// OK build(self) consuming
////let mut list_commands = ListCommands::new();
////list_commands.format("1");
////cmds.push(list_commands.build());

//// OK build(self) consuming
//let mut list_commands = ListCommands::new();
//list_commands.format("1");
//cmds.push(&list_commands);

//// E0716
////cmds.push(ListCommands::new().format("1"));

//let start_server = StartServer::new();
//cmds.push(start_server.build());

//// return type transfer Self -> &mut self -> Command
//// non consuming builders preferred
//let mut list_commands = ListCommands::new();
//list_commands.format("#{command_list_alias}");
//cmds.push(&list_commands);

//let mut show_options1 = ShowOptions::new();
//show_options1.global().option("default-shell");
//cmds.push(show_options1.build());

//// E0716
//// can be fixed using consumable builder pattern
////cmds.push(ListCommands::new().format("1").build().to_owned());

//let mut show_options2 = ShowOptions::new();
//show_options2.global().option("default-shell");
//cmds.push(show_options2.build());

//dbg!(&cmds);
//let result = cmds.output().unwrap();
//dbg!(&result);
//}

//#[test]
//fn tmux_bin_commands2() {
//let cmds = TmuxBinCommands::default();

//cmds.new_session().detached().session_name("kd").build();

//cmds.has_session().session_name("kd").build();

//cmds.kill_session()
//}
