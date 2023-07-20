use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Prev<'a> = PreviousWindow<'a>;

/// Move to the previous window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// previous-window [-a] [-t target-session]
/// (alias: prev)
/// ```
///
/// tmux ^0.8:
/// ```text
/// previous-window [-t target-session]
/// (alias: prev)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PreviousWindow<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_0_9")]
    pub parent_sighup: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> PreviousWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_0_9")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(PREVIOUS_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_0_9")]
        if self.parent_sighup {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
