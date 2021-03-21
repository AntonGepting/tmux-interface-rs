use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Kill the tmux server and clients and destroy all sessions
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux kill-server
/// ```
#[derive(Debug, Clone)]
pub struct KillServer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for KillServer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(KILL_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> KillServer<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> From<TmuxCommand<'a>> for KillServer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(KILL_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for KillServer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(KILL_SERVER)),
            ..Default::default()
        })
    }
}
