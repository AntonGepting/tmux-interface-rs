use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.0:
/// ```text
/// clock-mode [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// clock-mode [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ClockMode<'a> {
    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> ClockMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
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

        cmd.name(CLOCK_MODE);

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
