// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Enter copy mode
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// copy-mode [-deHMqSu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=3.5:
/// ```text
/// copy-mode [-deHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=3.1a:
/// ```text
/// copy-mode [-eHMqu] [-s src-pane] [-t target-pane]
/// ```
///
/// tmux >=2.1:
/// ```text
/// copy-mode [-Meu] [-t target-pane]
/// ```
///
/// tmux >=1.5:
/// ```text
/// copy-mode [-u] [-t target-pane]
/// ```
///
/// tmux >=0.8:
/// ```text
/// copy-mode [-u] [-t target-window]
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CopyMode<'a> {
    /// `[-d]`
    #[cfg(feature = "tmux_3_5")]
    pub scroll_down: bool,

    /// `[-e]`
    #[cfg(feature = "tmux_2_1")]
    pub bottom_exit: bool,

    /// `[-H]`
    #[cfg(feature = "tmux_3_1a")]
    pub hide_position: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub mouse_drag: bool,

    /// `[-q]`
    #[cfg(feature = "tmux_3_1a")]
    pub cancel: bool,

    /// `[-S]`
    #[cfg(feature = "tmux_3_6")]
    pub from_src_pane: bool,

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub page_up: bool,

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub src_pane: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> CopyMode<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-d]`
    #[cfg(feature = "tmux_3_5")]
    pub fn scroll_down(mut self) -> Self {
        self.scroll_down = true;
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_1")]
    pub fn bottom_exit(mut self) -> Self {
        self.bottom_exit = true;
        self
    }

    /// `[-H]`
    #[cfg(feature = "tmux_3_1a")]
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

    /// `[-q]`
    #[cfg(feature = "tmux_3_1a")]
    pub fn cancel(mut self) -> Self {
        self.cancel = true;
        self
    }

    /// `[-S]`
    #[cfg(feature = "tmux_3_6")]
    pub fn from_src_pane(mut self) -> Self {
        self.from_src_pane = true;
        self
    }

    /// `[-u]`
    #[cfg(feature = "tmux_0_8")]
    pub fn page_up(mut self) -> Self {
        self.page_up = true;
        self
    }

    /// `[-s src-pane]`
    #[cfg(feature = "tmux_3_2")]
    pub fn src_pane<S: Into<Cow<'a, str>>>(mut self, src_pane: S) -> Self {
        self.src_pane = Some(src_pane.into());
        self
    }

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(COPY_MODE);

        // `[-d]`
        #[cfg(feature = "tmux_3_5")]
        if self.scroll_down {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-e]`
        #[cfg(feature = "tmux_2_1")]
        if self.bottom_exit {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-H]`
        #[cfg(feature = "tmux_3_1a")]
        if self.hide_position {
            cmd.push_flag(H_UPPERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_2_1")]
        if self.mouse_drag {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-q]`
        #[cfg(feature = "tmux_3_1a")]
        if self.cancel {
            cmd.push_flag(Q_LOWERCASE_KEY);
        }

        // `[-S]`
        #[cfg(feature = "tmux_3_6")]
        if self.from_src_pane {
            cmd.push_flag(S_UPPERCASE_KEY);
        }

        // `[-u]`
        #[cfg(feature = "tmux_0_8")]
        if self.page_up {
            cmd.push_flag(U_LOWERCASE_KEY);
        }

        // `[-s src-pane]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(src_pane) = self.src_pane {
            cmd.push_option(S_LOWERCASE_KEY, src_pane);
        }

        // `[-t target-window]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
