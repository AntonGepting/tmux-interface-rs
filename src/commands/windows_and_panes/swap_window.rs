use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SwapWindow<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    pub src_window: Option<Cow<'a, str>>,

    /// `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    pub dst_window: Option<Cow<'a, str>>,
}

impl<'a> SwapWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-s src-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn src_window<S: Into<Cow<'a, str>>>(mut self, src_window: S) -> Self {
        self.src_window = Some(src_window.into());
        self
    }

    /// `[-t dst-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn dst_window<S: Into<Cow<'a, str>>>(mut self, dst_window: S) -> Self {
        self.dst_window = Some(dst_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SWAP_WINDOW);

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-s src-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(src_window) = self.src_window {
            cmd.push_option(S_LOWERCASE_KEY, src_window);
        }

        // `[-t dst-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(dst_window) = self.dst_window {
            cmd.push_option(T_LOWERCASE_KEY, dst_window);
        }

        cmd
    }
}
