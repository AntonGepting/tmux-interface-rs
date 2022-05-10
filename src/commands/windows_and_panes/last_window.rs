use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Select the last (previously selected) window
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux last-window [-t target-session]
/// (alias: last)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LastWindow<'a> {
    /// `[-t target-session]`
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> LastWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(LAST_WINDOW);

        // `[-t target-session]`
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        cmd
    }
}
