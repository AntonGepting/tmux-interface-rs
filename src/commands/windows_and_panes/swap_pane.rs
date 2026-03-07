// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SwapP<'a> = SwapPane<'a>;

/// Swap two panes
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// swap-pane [-dDUZ] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// swap-pane [-dDU] [-s src-pane] [-t dst-pane]
/// (alias: swapp)
/// ```
///
/// tmux >=0.8:
/// ```text
/// swap-pane [-dDU] [-p src-index] [-t target-window] [-q dst-index]
/// (alias: swapp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SwapPane<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub detached: bool,

    /// `[-D]`
    #[cfg(feature = "tmux_0_8")]
    pub previous_pane: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_0_8")]
    pub next_pane: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-p src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub src_index: Option<Cow<'a, str>>,

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-q dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub dst_index: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t dst-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub dst_pane: Option<Cow<'a, str>>,
}

impl<'a> SwapPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_0_8")]
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// `[-D]`
    #[cfg(feature = "tmux_0_8")]
    pub fn previous_pane(mut self) -> Self {
        self.previous_pane = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_0_8")]
    pub fn next_pane(mut self) -> Self {
        self.next_pane = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(mut self) -> Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-p src-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn src_index<S: Into<Cow<'a, str>>>(mut self, src_index: S) -> Self {
        self.src_index = Some(src_index.into());
        self
    }

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-q dst-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn dst_index<S: Into<Cow<'a, str>>>(mut self, dst_index: S) -> Self {
        self.dst_index = Some(dst_index.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t dst-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn dst_pane<S: Into<Cow<'a, str>>>(mut self, dst_pane: S) -> Self {
        self.dst_pane = Some(dst_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SWAP_PANE);

        // `[-d]`
        #[cfg(feature = "tmux_0_8")]
        if self.detached {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-D]`
        #[cfg(feature = "tmux_0_8")]
        if self.previous_pane {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_0_8")]
        if self.next_pane {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-p src-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(src_index) = self.src_index {
            cmd.push_option(P_LOWERCASE_KEY, src_index);
        }

        // `[-s src-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-q dst-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(dst_index) = self.dst_index {
            cmd.push_option(Q_LOWERCASE_KEY, dst_index);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t dst-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(dst_pane) = self.dst_pane {
            cmd.push_option(T_LOWERCASE_KEY, dst_pane);
        }

        cmd
    }
}
