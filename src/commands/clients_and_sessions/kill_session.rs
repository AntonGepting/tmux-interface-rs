use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Destroy the given session
///
/// # Manual
///
/// tmux ^2.2:
/// ```text
/// kill-session [-aC] [-t target-session]
/// ```
///
/// tmux ^1.7:
/// ```text
/// kill-session [-a] [-t target-session]
/// ```
///
/// tmux ^0.8:
/// ```text
/// kill-session [-t target-session]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct KillSession<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_2_2")]
    pub all: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_1_7")]
    pub clear_alerts: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> KillSession<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_2_2")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_1_7")]
    pub fn clear_alerts(mut self) -> Self {
        self.clear_alerts = true;
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

        cmd.name(KILL_SESSION);

        // `[-a]`
        #[cfg(feature = "tmux_2_2")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_1_7")]
        if self.clear_alerts {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
