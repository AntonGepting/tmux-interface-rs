use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.0:
/// ```text
/// tmux clock-mode [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux clock-mode [-t target-window]
/// ```
#[derive(Debug, Clone)]
pub struct ClockMode<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ClockMode<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CLOCK_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> ClockMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ClockMode<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CLOCK_MODE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ClockMode<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CLOCK_MODE)),
            ..Default::default()
        })
    }
}
