use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Swap two panes
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
/// (alias: swapp)
/// ```
#[derive(Debug, Clone)]
pub struct SwapPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SwapPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SWAP_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> SwapPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - instruct tmux not to change the active pane
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-D]` - swap with the next pane
    #[cfg(feature = "tmux_0_8")]
    pub fn previous_pane(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - swap with the previous pane
    #[cfg(feature = "tmux_0_8")]
    pub fn next_pane(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.0.push_option(S_LOWERCASE_KEY, src_pane);
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, dst_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SwapPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SWAP_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SwapPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SWAP_PANE)),
            ..Default::default()
        })
    }
}
