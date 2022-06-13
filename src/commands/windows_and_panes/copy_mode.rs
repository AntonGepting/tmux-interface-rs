use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Enter copy mode
///
/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux copy-mode [-u] [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CopyMode<'a> {
    /// `[-e]`
    #[cfg(feature = "tmux_2_1")]
    pub bottom_exit: bool,

    /// `[-H]` - hides the position indicator in the top right
    #[cfg(feature = "tmux_3_2")]
    pub hide_position: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub mouse_drag: bool,

    /// `[-q]` - cancels copy mode and any other modes
    #[cfg(feature = "tmux_3_2")]
    pub cancel: bool,

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub page_up: bool,

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub target_window: Option<Cow<'a, str>>,
}

impl<'a> CopyMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_1")]
    pub fn bottom_exit(mut self) -> Self {
        self.bottom_exit = true;
        self
    }

    /// `[-H]` - hides the position indicator in the top right
    #[cfg(feature = "tmux_3_2")]
    pub fn hide_position(mut self) -> Self {
        self.hide_position = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub fn mouse_drag(mut self) -> Self {
        self.mouse_drag = true;
        self
    }

    /// `[-q]` - cancels copy mode and any other modes
    #[cfg(feature = "tmux_3_2")]
    pub fn cancel(mut self) -> Self {
        self.cancel = true;
        self
    }

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub fn page_up(mut self) -> Self {
        self.page_up = true;
        self
    }

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(COPY_MODE);

        // `[-e]`
        #[cfg(feature = "tmux_2_1")]
        if self.bottom_exit {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-H]` - hides the position indicator in the top right
        #[cfg(feature = "tmux_3_2")]
        if self.hide_position {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_2_1")]
        if self.mouse_drag {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-q]` - cancels copy mode and any other modes
        #[cfg(feature = "tmux_3_2")]
        if self.cancel {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-u]`
        #[cfg(feature = "tmux_0_8")]
        if self.page_up {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-s src-pane]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_0")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        cmd
    }
}
