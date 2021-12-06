#[test]
fn tmux_commands() {
    use crate::commands::tmux_bin_command::TmuxBinCommand;
    use crate::commands::tmux_bin_commands::TmuxBinCommands;
    use crate::commands::tmux_commands::TmuxCommands;
    use crate::{NewSession, SessionOptions, ShowOptions, StartServer, TmuxCommand};
    use std::borrow::Cow;

    let mut cmd = TmuxBinCommand::new();
    //cmds.command.

    let mut multiple_commands = TmuxBinCommands::default();
    multiple_commands.tmux.verbose();

    let mut start_server = StartServer::new();
    //let mut start_server = StartServer::new()
    //.to_command()
    //.append_to(&mut multiple_commands.commands);
    multiple_commands.push(start_server);

    let mut show_options = ShowOptions::new();
    show_options.global().option("default-shell");
    multiple_commands.push(show_options);

    dbg!(&multiple_commands);

    let result = multiple_commands.output().unwrap();

    dbg!(&result);

    let session_options = result.to_string().parse::<SessionOptions>();
    dbg!(&session_options);

    //let v = cmds.to_vec();
    //dbg!(&v);
}
