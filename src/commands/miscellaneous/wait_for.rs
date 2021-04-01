use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

// TODO: enum for arg
/// # Manual
///
/// tmux ^1.9:
/// ```text
/// tmux wait-for [-L | -S | -U] channel
/// (alias: wait)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux wait-for -LSU channel
/// (alias: wait)
/// ```
// FIXME: not multiple, only one choice
#[derive(Debug, Clone)]
pub struct WaitFor<'a>(pub TmuxCommand<'a>);

impl<'a> Default for WaitFor<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(WAIT_FOR)),
            ..Default::default()
        })
    }
}

impl<'a> WaitFor<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-L]`
    #[cfg(feature = "tmux_1_8")]
    pub fn locked(&mut self) -> &mut Self {
        self.0.push_flag(L_UPPERCASE_KEY);
        self
    }

    /// `[-S]`
    #[cfg(feature = "tmux_1_8")]
    pub fn woken(&mut self) -> &mut Self {
        self.0.push_flag(S_UPPERCASE_KEY);
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_1_8")]
    pub fn unlocked(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `channel`
    #[cfg(feature = "tmux_1_8")]
    pub fn channel<S: Into<Cow<'a, str>>>(&mut self, channel: S) -> &mut Self {
        self.0.push_param(channel);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for WaitFor<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(WAIT_FOR)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for WaitFor<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(WAIT_FOR)),
            ..Default::default()
        })
    }
}
