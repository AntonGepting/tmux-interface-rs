use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^2.2:
/// ```text
/// show-hooks [-g] [-t target-session]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowHooks<'a> {
    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub global: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_2_2")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> ShowHooks<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]`
    #[cfg(feature = "tmux_2_2")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-t target-session]`
    #[cfg(feature = "tmux_2_2")]
    pub fn target_session<S: Into<Cow<'a, str>>>(mut self, target_session: S) -> Self {
        self.target_session = Some(target_session.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_HOOKS);

        // `[-g]`
        #[cfg(feature = "tmux_2_2")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-t target-session]`
        #[cfg(feature = "tmux_2_2")]
        if let Some(target_session) = self.target_session {
            cmd.push_option(T_LOWERCASE_KEY, target_session);
        }

        cmd
    }
}
