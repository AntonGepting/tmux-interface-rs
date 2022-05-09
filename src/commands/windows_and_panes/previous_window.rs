use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Move to the previous window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// tmux previous-window [-a] [-t target-session]
/// (alias: prev)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux previous-window [-t target-session]
/// (alias: prev)
/// ```
#[derive(Debug, Default, Clone)]
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
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.parent_sighup = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_session<S: Into<Cow<'a, str>>>(&mut self, target_session: S) -> &mut Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(PREVIOUS_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_0_9")]
        if self.parent_sighup {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_session) = &self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session.as_ref());
        }

        cmd
    }
}
