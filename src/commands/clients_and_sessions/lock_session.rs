use crate::commands::constants::*;
use crate::{TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Lock all clients attached to `target-session`
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// tmux lock-session [-t target-session]
/// (alias: locks)
/// ```
#[derive(Debug)]
pub struct LockSession<'a>(TmuxCommand<'a>);

impl<'a> Default for LockSession<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(LOCK_SESSION)),
            ..Default::default()
        })
    }
}

impl<'a> LockSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// [-t target-session]
    pub fn target_session<T: Into<String>>(&mut self, target_session: T) -> &mut Self {
        self.0.push_option(t_KEY, target_session);
        self
    }

    /// run command
    pub fn output(&self) -> TmuxOutput {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for LockSession<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(LOCK_SESSION)),
            ..Default::default()
        })
    }
}
