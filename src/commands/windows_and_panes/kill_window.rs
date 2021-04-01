use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct KillWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for KillWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(KILL_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> KillWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]`
    #[cfg(feature = "tmux_1_7")]
    pub fn parent_sighup(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for KillWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(KILL_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for KillWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(KILL_WINDOW)),
            ..Default::default()
        })
    }
}
