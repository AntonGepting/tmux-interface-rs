use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

// XXX: better result return?
/// Report if the specified session exist
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux has-session [-t target-session]
/// (alias: has)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct HasSession<'a> {
    /// `[-t target-session]`
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> HasSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(HAS_SESSION);

        // `[-t target-session]`
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
