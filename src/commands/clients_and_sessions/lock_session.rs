use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Lock all clients attached to `target-session`
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// tmux lock-session [-t target-session]
/// (alias: locks)
/// ```
#[derive(Clone, Default, Debug)]
pub struct LockSession<'a> {
    /// `[-t target-session]`
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> LockSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    pub fn target_session<T: Into<Cow<'a, str>>>(&mut self, target_session: T) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// run command
    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LOCK_SESSION);

        // `[-t target-session]`
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        cmd
    }
}
