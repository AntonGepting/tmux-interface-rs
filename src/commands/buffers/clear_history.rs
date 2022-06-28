use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Remove and free the history for the specified pane.
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// clear-history [-t target-pane]
/// (alias: clearhist)
/// ```
///
/// tmux ^0.9:
/// ```text
/// clear-history [-p pane-index] [-t target-window]
/// (alias: clearhist)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClearHistory<'a> {
    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub pane_index: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> ClearHistory<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub fn pane_index<S: Into<Cow<'a, str>>>(mut self, pane_index: S) -> Self {
        self.pane_index = Some(pane_index.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(CLEAR_HISTORY);

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-p pane-index]`
        #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
        if let Some(pane_index) = self.pane_index {
            cmd.push_option(P_LOWERCASE_KEY, pane_index);
        }

        // `[-t target-pane]`
        #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
