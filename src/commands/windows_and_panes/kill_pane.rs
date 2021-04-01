use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Destroy the given pane
///
/// # Manual
///
/// tmux ^1.1:
/// ```text
/// tmux kill-pane [-a] [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux kill-pane [-t target-pane]
/// (alias: killp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux kill-pane [-p pane-index] [-t target-window]
/// (alias: killp)
/// ```
#[derive(Debug, Clone)]
pub struct KillPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for KillPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(KILL_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> KillPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_1")]
    pub fn all(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn pane_index<S: Into<Cow<'a, str>>>(&mut self, pane_index: S) -> &mut Self {
        self.0.push_option(p_KEY, pane_index);
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(t_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for KillPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(KILL_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for KillPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(KILL_PANE)),
            ..Default::default()
        })
    }
}
