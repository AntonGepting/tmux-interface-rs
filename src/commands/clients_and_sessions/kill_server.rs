use crate::commands::constants::*;
use crate::TmuxCommand;

/// Kill the tmux server and clients and destroy all sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux kill-server
/// ```
#[derive(Debug, Default, Clone)]
pub struct KillServer;

impl KillServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(KILL_SERVER);

        cmd
    }
}
