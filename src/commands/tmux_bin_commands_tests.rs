// TODO: All in one variants demo
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

#[test]
fn tmux_bin_commands2() {
    use crate::commands::tmux_bin_commands::TmuxBinCommands;
    use crate::{ListCommands, ShowOptions, StartServer};

    //let tmux = TmuxBin::default();
    //let mut tmux = TmuxBin::new();
    //tmux.bin("tmux");
    //tmux.bin = Some("tmux");
    //let mut cmds = TmuxBinCommands::from(tmux);

    let mut cmds = TmuxBinCommands::default();
    cmds.tmux.verbose();

    // E0716
    //let list_commands = ListCommands::new().format("1");
    //cmds.push(list_commands);

    // OK build(self) consuming
    //let mut list_commands = ListCommands::new();
    //list_commands.format("1");
    //cmds.push(list_commands.build());

    // OK build(self) consuming
    let mut list_commands = ListCommands::new();
    list_commands.format("1");
    cmds.push(&list_commands);

    // E0716
    //cmds.push(ListCommands::new().format("1"));

    let start_server = StartServer::new();
    cmds.push(start_server.build());

    // return type transfer Self -> &mut self -> Command
    // non consuming builders preferred
    let mut list_commands = ListCommands::new();
    list_commands.format("#{command_list_alias}");
    cmds.push(&list_commands);

    let mut show_options1 = ShowOptions::new();
    show_options1.global().option("default-shell");
    cmds.push(show_options1.build());

    // E0716
    // can be fixed using consumable builder pattern
    //cmds.push(ListCommands::new().format("1").build().to_owned());

    let mut show_options2 = ShowOptions::new();
    show_options2.global().option("default-shell");
    cmds.push(show_options2.build());

    dbg!(&cmds);
    let result = cmds.output().unwrap();
    dbg!(&result);
}

//#[test]
//fn tmux_bin_commands2() {
//let cmds = TmuxBinCommands::default();

//cmds.new_session().detached().session_name("kd").build();

//cmds.has_session().session_name("kd").build();

//cmds.kill_session()
//}
