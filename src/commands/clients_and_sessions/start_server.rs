use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

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
}

impl<'a> From<TmuxCommand<'a>> for StartServer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(START_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for StartServer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(START_SERVER)),
            ..Default::default()
        })
    }
}
