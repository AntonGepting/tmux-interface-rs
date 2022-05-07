use crate::commands::constants::*;
use crate::TmuxCommand;

/// Start the tmux server, if not already running, without creating any sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux start-server
/// (alias: start)
/// ```
#[derive(Debug, Default, Clone)]
pub struct StartServer;

impl StartServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(START_SERVER);

        cmd
    }

    //pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
    //self.0.append_to(cmds)
    //}

    //pub fn to_command(self) -> TmuxCommand<'a> {
    //self.0
    //}
}
