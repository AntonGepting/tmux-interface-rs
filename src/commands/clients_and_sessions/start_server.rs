use crate::commands::constants::*;
use crate::TmuxCommand;

/// Start the tmux server, if not already running, without creating any sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// start-server
/// (alias: start)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StartServer;

impl StartServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build<'a>(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(START_SERVER);

        cmd
    }
}
