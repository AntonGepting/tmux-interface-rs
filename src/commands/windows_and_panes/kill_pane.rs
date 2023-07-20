use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type KillP<'a> = KillPane<'a>;

/// Destroy the given pane
///
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// kill-pane [-a] [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// kill-pane [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// kill-pane [-p pane-index] [-t target-window]
/// (alias: killp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct KillPane<'a> {
    /// `[-a]`
    #[cfg(feature = "tmux_1_1")]
    pub all: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub pane_index: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> KillPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_1")]
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn pane_index<S: Into<Cow<'a, str>>>(mut self, pane_index: S) -> Self {
        self.pane_index = Some(pane_index.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(KILL_PANE);

        // `[-a]`
        #[cfg(feature = "tmux_1_1")]
        if self.all {
            cmd.push_flag(A_LOWERCASE_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-p pane-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
        if let Some(pane_index) = self.pane_index {
            cmd.push_option(P_LOWERCASE_KEY, pane_index);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
