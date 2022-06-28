use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Move a window to the next layout and rearrange the panes to fit
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// next-layout [-t target-window]
/// (alias: nextl)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct NextLayout<'a> {
    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> NextLayout<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(NEXT_LAYOUT);

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
