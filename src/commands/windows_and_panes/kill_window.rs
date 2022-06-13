use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Kill the current window or the window at target-window, removing it from any sessions
/// to which it is linked
///
/// # Manual
///
/// tmux ^1.7:
/// ```text
/// tmux kill-window [-a] [-t target-window]
/// (alias: killw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux kill-window [-a] [-t target-window]
/// (alias: killw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct KillWindow<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_7")]
    pub parent_sighup: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> KillWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_7")]
    pub fn parent_sighup(mut self) -> Self {
        self.parent_sighup = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(KILL_WINDOW);

        // `[-a]`
        #[cfg(feature = "tmux_1_7")]
        if self.parent_sighup {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
