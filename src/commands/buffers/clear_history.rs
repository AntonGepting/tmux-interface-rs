use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Remove and free the history for the specified pane.
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// tmux clear-history [-t target-pane]
/// (alias: clearhist)
/// ```
///
/// tmux ^0.9:
/// ```text
/// tmux clear-history [-p pane-index] [-t target-window]
/// (alias: clearhist)
/// ```
#[derive(Debug, Clone)]
pub struct ClearHistory<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ClearHistory<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(CLEAR_HISTORY)),
            ..Default::default()
        })
    }
}

impl<'a> ClearHistory<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub fn pane_index<S: Into<Cow<'a, str>>>(&mut self, pane_index: S) -> &mut Self {
        self.0.push_option(P_LOWERCASE_KEY, pane_index);
        self
    }

    /// `[-t target-pane]`
    #[cfg(all(feature = "tmux_0_9", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ClearHistory<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(CLEAR_HISTORY)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ClearHistory<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(CLEAR_HISTORY)),
            ..Default::default()
        })
    }
}
