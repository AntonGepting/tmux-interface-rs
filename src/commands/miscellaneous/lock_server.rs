use crate::commands::constants::*;
use crate::TmuxCommand;

/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux lock-server
/// (alias: lock)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LockServer;

impl LockServer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LOCK_SERVER);

        cmd
    }
}
