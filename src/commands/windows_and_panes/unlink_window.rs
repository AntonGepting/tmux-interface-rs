use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Unlink `target-window`
///
/// # Manual
///
/// tmux ^1.0:
/// ```text
/// tmux unlink-window [-k] [-t target-window]
/// (alias: unlinkw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux unlink-window [-t target-window]
/// (alias: unlinkw)
/// ```
#[derive(Debug, Clone)]
pub struct UnlinkWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for UnlinkWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(UNLINK_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> UnlinkWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-k]`
    #[cfg(feature = "tmux_1_0")]
    pub fn detach_other(&mut self) -> &mut Self {
        self.0.push_flag(K_LOWERCASE_KEY);
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

impl<'a> From<TmuxCommand<'a>> for UnlinkWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(UNLINK_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for UnlinkWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(UNLINK_WINDOW)),
            ..Default::default()
        })
    }
}
