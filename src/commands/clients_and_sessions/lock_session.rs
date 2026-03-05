// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type LockS<'a> = LockSession<'a>;

/// Lock all clients attached to `target-session`
/// # Manual
///
/// tmux >=1.5:
/// ```text
/// lock-session [-t target-session]
/// (alias: locks)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LockSession<'a> {
    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> LockSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(LOCK_SESSION);

        // `[-t target-session]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
