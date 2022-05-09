use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SwapPane<'a> {
    /// `[-d]` - instruct tmux not to change the active pane
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-D]` - swap with the next pane
    #[cfg(feature = "tmux_0_8")]
    pub previous_pane: bool,

    /// `[-U]` - swap with the previous pane
    #[cfg(feature = "tmux_0_8")]
    pub next_pane: bool,

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_0")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_0")]
    pub dst_pane: Option<Cow<'a, str>>,
}

impl<'a> SwapPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]` - instruct tmux not to change the active pane
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(&mut self) -> &mut Self {
        self.detached = true;
        self
    }

    /// `[-D]` - swap with the next pane
    #[cfg(feature = "tmux_0_8")]
    pub fn previous_pane(&mut self) -> &mut Self {
        self.previous_pane = true;
        self
    }

    /// `[-U]` - swap with the previous pane
    #[cfg(feature = "tmux_0_8")]
    pub fn next_pane(&mut self) -> &mut Self {
        self.next_pane = true;
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-s src-pane]` - src-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(&mut self, src_pane: S) -> &mut Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t dst-pane]` - dst-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(&mut self, dst_pane: S) -> &mut Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SWAP_PANE);

        // `[-d]` - instruct tmux not to change the active pane
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-D]` - swap with the next pane
        #[cfg(feature = "tmux_0_8")]
        if self.previous_pane {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-U]` - swap with the previous pane
        #[cfg(feature = "tmux_0_8")]
        if self.next_pane {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]` - keep the window zoomed if it was zoomed
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-s src-pane]` - src-pane
        #[cfg(feature = "tmux_1_0")]
        if let Some(src_pane) = &self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane.as_ref());
        }

        // `[-t dst-pane]` - dst-pane
        #[cfg(feature = "tmux_1_0")]
        if let Some(dst_pane) = &self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane.as_ref());
        }

        cmd
    }
}
