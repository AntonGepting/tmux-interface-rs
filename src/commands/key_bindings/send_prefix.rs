use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^1.6
/// ```text
/// tmux send-prefix [-2] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux send-prefix [-t target-pane]
/// ```
#[derive(Debug, Clone)]
pub struct SendPrefix<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SendPrefix<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SEND_PREFIX)),
            ..Default::default()
        })
    }
}

impl<'a> SendPrefix<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-2]`
    #[cfg(feature = "tmux_1_6")]
    pub fn secondary(&mut self) -> &mut Self {
        self.0.push_flag(_2_KEY);
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SendPrefix<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SEND_KEYS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SendPrefix<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SEND_KEYS)),
            ..Default::default()
        })
    }
}
