use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux lock-server
/// (alias: lock)
/// ```
#[derive(Debug, Clone)]
pub struct LockServer<'a>(pub TmuxCommand<'a>);

impl<'a> Default for LockServer<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOCK_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> LockServer<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LockServer<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LOCK_SERVER)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for LockServer<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(LOCK_SERVER)),
            ..Default::default()
        })
    }
}
