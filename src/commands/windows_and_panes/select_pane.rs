// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type SelectP<'a> = SelectPane<'a>;

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux >=3.1:
/// ```text
/// select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.6:
/// ```text
/// select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.1:
/// ```text
/// select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=2.0:
/// ```text
/// select-pane [-DdeLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=1.5:
/// ```text
/// select-pane [-DLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux >=0.8:
/// ```text
/// select-pane [-p pane-index] [-t target-window]
/// (alias: selectp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SelectPane<'a> {
    /// `[-D]`
    #[cfg(feature = "tmux_1_5")]
    pub down: bool,

    /// `[-d]`
    #[cfg(feature = "tmux_2_0")]
    pub disable: bool,

    /// `[-e]`
    #[cfg(feature = "tmux_2_0")]
    pub enable: bool,

    /// `[-g]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    pub show_style: bool,

    /// `[-L]`
    #[cfg(feature = "tmux_1_5")]
    pub left: bool,

    /// `[-l]`
    #[cfg(feature = "tmux_1_5")]
    pub last: bool,

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub set_marked: bool,

    /// `[-m]`
    #[cfg(feature = "tmux_2_1")]
    pub clear_marked: bool,

    /// `[-R]`
    #[cfg(feature = "tmux_1_5")]
    pub right: bool,

    /// `[-U]`
    #[cfg(feature = "tmux_1_5")]
    pub up: bool,

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub pane_index: Option<Cow<'a, str>>,

    /// `[-P style]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    pub style: Option<Cow<'a, str>>,

    /// `[-T title]`
    #[cfg(feature = "tmux_2_6")]
    pub title: Option<Cow<'a, str>>,

    /// `[-t target-window]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub target_window: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,
}

impl<'a> SelectPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]`
    #[cfg(feature = "tmux_1_5")]
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-d]`
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(mut self) -> Self {
        self.disable = true;
        self
    }

    /// `[-e]`
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    /// `[-g]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
    pub fn show_style(mut self) -> Self {
        self.show_style = true;
        self
    }

    /// `[-L]`
    #[cfg(feature = "tmux_1_5")]
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-l]`
    #[cfg(feature = "tmux_1_5")]
    pub fn last(mut self) -> Self {
        self.last = true;
        self
    }

    /// `[-M]`
    #[cfg(feature = "tmux_2_1")]
    pub fn set_marked(mut self) -> Self {
        self.set_marked = true;
        self
    }

    /// `[-m]`
    #[cfg(feature = "tmux_2_1")]
    pub fn clear_marked(mut self) -> Self {
        self.clear_marked = true;
        self
    }

    /// `[-R]`
    #[cfg(feature = "tmux_1_5")]
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-U]`
    #[cfg(feature = "tmux_1_5")]
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-Z]`
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(mut self) -> Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-p pane-index]`
    #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
    pub fn pane_index<S: Into<Cow<'a, str>>>(mut self, pane_index: S) -> Self {
        self.pane_index = Some(pane_index.into());
        self
    }

    /// `[-P style]`
    #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
    pub fn style<S: Into<Cow<'a, str>>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// `[-T title]`
    #[cfg(feature = "tmux_2_6")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
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

        cmd.name(SELECT_PANE);

        // `[-D]`
        #[cfg(feature = "tmux_1_5")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-d]`
        #[cfg(feature = "tmux_2_0")]
        if self.disable {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-e]`
        #[cfg(feature = "tmux_2_0")]
        if self.enable {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-g]`
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_1")))]
        if self.show_style {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-L]`
        #[cfg(feature = "tmux_1_5")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-l]`
        #[cfg(feature = "tmux_1_5")]
        if self.last {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-M]`
        #[cfg(feature = "tmux_2_1")]
        if self.set_marked {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-m]`
        #[cfg(feature = "tmux_2_1")]
        if self.clear_marked {
            cmd.push_flag(M_LOWERCASE_KEY);
        }

        // `[-R]`
        #[cfg(feature = "tmux_1_5")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-U]`
        #[cfg(feature = "tmux_1_5")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]`
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-p pane-index]`
        #[cfg(all(feature = "tmux_0_8", not(feature = "tmux_1_5")))]
        if let Some(pane_index) = self.pane_index {
            cmd.push_option(P_LOWERCASE_KEY, pane_index);
        }

        // `[-P style]`
        #[cfg(all(feature = "tmux_2_1", not(feature = "tmux_3_0a")))]
        if let Some(style) = self.style {
            cmd.push_option(P_UPPERCASE_KEY, style);
        }

        // `[-T title]`
        #[cfg(feature = "tmux_2_6")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
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
