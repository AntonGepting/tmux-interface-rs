use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

use crate::commands::tmux_commands::TmuxCommands;

/// Start the tmux server, if not already running, without creating any sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux start-server
/// (alias: start)
/// ```
#[derive(Debug, Clone)]
pub struct StartServer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for StartServer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(START_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> StartServer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }

    pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
        self.0.append_to(cmds)
    }

    pub fn to_command(self) -> TmuxCommand<'a> {
        self.0
    }
}

impl<'a> From<TmuxCommand<'a>> for StartServer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(START_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for StartServer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(START_SERVER)),
            ..Default::default()
        })
    }
}
