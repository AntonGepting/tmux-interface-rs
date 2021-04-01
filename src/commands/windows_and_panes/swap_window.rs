use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// This is similar to link-window, except the source and destination windows are swapped
///
/// # Manual
///
/// tmux ^0.8:
/// ```text
/// tmux swap-window [-d] [-s src-window] [-t dst-window]
/// (alias: swapw)
/// ```
#[derive(Debug, Clone)]
pub struct SwapWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SwapWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SWAP_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> SwapWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn src_window<S: Into<Cow<'a, str>>>(&mut self, src_window: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, src_window);
        self
    }

    /// `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(&mut self, dst_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SwapWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SWAP_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SwapWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SWAP_WINDOW)),
            ..Default::default()
        })
    }
}
