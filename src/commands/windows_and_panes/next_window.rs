use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Next<'a> = NextWindow<'a>;

/// Move to the next window in the session
///
/// # Manual
///
/// tmux ^0.9:
/// ```text
/// next-window [-a] [-t target-session]
/// (alias: next)
/// ```
///
/// tmux ^0.8:
/// ```text
/// next-window [-t target-session]
/// (alias: next)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NextWindow<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_0_9")]
    pub attach: bool,

    /// `[-t target-session]`
    #[cfg(feature = "tmux_0_8")]
    pub target_session: Option<Cow<'a, str>>,
}

impl<'a> NextWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_0_9")]
    pub fn attach(mut self) -> Self {
        self.attach = true;
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

        cmd.name(NEXT_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_0_9")]
        if self.attach {
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
