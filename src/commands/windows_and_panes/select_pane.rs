use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;
use std::marker::PhantomData;

/// Make pane `target-pane` the active pane in window `target-window`
///
/// # Manual
///
/// tmux ^3.1:
/// ```text
/// tmux select-pane [-DdeLlMmRUZ] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux select-pane [-DdeLlMmRU] [-T title] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.1:
/// ```text
/// tmux select-pane [-DdegLlMmRU] [-P style] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^2.0:
/// ```text
/// tmux select-pane [-DdeLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-pane [-DLlRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.3:
/// ```text
/// tmux select-pane [-DLRU] [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux select-pane [-t target-pane]
/// (alias: selectp)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux select-pane [-p pane-index] [-t target-window]
/// (alias: selectp)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SelectPane<'a> {
    /// `[-D]` - pane below
    #[cfg(feature = "tmux_1_3")]
    pub down: bool,

    /// `[-d]` - disable input
    #[cfg(feature = "tmux_2_0")]
    pub disable: bool,

    /// `[-e]` - enable input
    #[cfg(feature = "tmux_2_0")]
    pub enable: bool,

    /// `[-g]` - show the current pane style
    #[cfg(feature = "tmux_2_1")]
    pub show_style: bool,

    /// `[-L]` - pane left
    #[cfg(feature = "tmux_1_3")]
    pub left: bool,

    /// `[-l]` - equivalent to last-pane command
    #[cfg(feature = "tmux_1_5")]
    pub last: bool,

    /// `[-M]` - clear marked pane
    #[cfg(feature = "tmux_2_1")]
    pub set_marked: bool,

    /// `[-m]` - set marked pane
    #[cfg(feature = "tmux_2_1")]
    pub clear_marked: bool,

    /// `[-R]` - pane right
    #[cfg(feature = "tmux_1_3")]
    pub right: bool,

    /// `[-U]` - pane above
    #[cfg(feature = "tmux_1_3")]
    pub up: bool,

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub keep_zoomed: bool,

    /// `[-P style]` - set the style for a single pane
    #[cfg(feature = "tmux_2_1")]
    pub style: Option<Cow<'a, str>>,

    /// `[-T title]` - title
    #[cfg(feature = "tmux_2_6")]
    pub title: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_0")]
    pub target_pane: Option<Cow<'a, str>>,

    _phantom_data: PhantomData<&'a ()>,
}

impl<'a> SelectPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]` - pane below
    #[cfg(feature = "tmux_1_3")]
    pub fn down(mut self) -> Self {
        self.down = true;
        self
    }

    /// `[-d]` - disable input
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(mut self) -> Self {
        self.disable = true;
        self
    }

    /// `[-e]` - enable input
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    /// `[-g]` - show the current pane style
    #[cfg(feature = "tmux_2_1")]
    pub fn show_style(mut self) -> Self {
        self.show_style = true;
        self
    }

    /// `[-L]` - pane left
    #[cfg(feature = "tmux_1_3")]
    pub fn left(mut self) -> Self {
        self.left = true;
        self
    }

    /// `[-l]` - equivalent to last-pane command
    #[cfg(feature = "tmux_1_5")]
    pub fn last(mut self) -> Self {
        self.last = true;
        self
    }

    /// `[-M]` - clear marked pane
    #[cfg(feature = "tmux_2_1")]
    pub fn set_marked(mut self) -> Self {
        self.set_marked = true;
        self
    }

    /// `[-m]` - set marked pane
    #[cfg(feature = "tmux_2_1")]
    pub fn clear_marked(mut self) -> Self {
        self.clear_marked = true;
        self
    }

    /// `[-R]` - pane right
    #[cfg(feature = "tmux_1_3")]
    pub fn right(mut self) -> Self {
        self.right = true;
        self
    }

    /// `[-U]` - pane above
    #[cfg(feature = "tmux_1_3")]
    pub fn up(mut self) -> Self {
        self.up = true;
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(mut self) -> Self {
        self.keep_zoomed = true;
        self
    }

    /// `[-P style]` - set the style for a single pane
    #[cfg(feature = "tmux_2_1")]
    pub fn style<S: Into<Cow<'a, str>>>(mut self, style: S) -> Self {
        self.style = Some(style.into());
        self
    }

    /// `[-T title]` - title
    #[cfg(feature = "tmux_2_6")]
    pub fn title<S: Into<Cow<'a, str>>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(SELECT_PANE);

        // `[-D]` - pane below
        #[cfg(feature = "tmux_1_3")]
        if self.down {
            cmd.push_flag(D_UPPERCASE_KEY);
        }

        // `[-d]` - disable input
        #[cfg(feature = "tmux_2_0")]
        if self.disable {
            cmd.push_flag(D_LOWERCASE_KEY);
        }

        // `[-e]` - enable input
        #[cfg(feature = "tmux_2_0")]
        if self.enable {
            cmd.push_flag(E_LOWERCASE_KEY);
        }

        // `[-g]` - show the current pane style
        #[cfg(feature = "tmux_2_1")]
        if self.show_style {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-L]` - pane left
        #[cfg(feature = "tmux_1_3")]
        if self.left {
            cmd.push_flag(L_UPPERCASE_KEY);
        }

        // `[-l]` - equivalent to last-pane command
        #[cfg(feature = "tmux_1_5")]
        if self.last {
            cmd.push_flag(L_LOWERCASE_KEY);
        }

        // `[-M]` - clear marked pane
        #[cfg(feature = "tmux_2_1")]
        if self.set_marked {
            cmd.push_flag(M_UPPERCASE_KEY);
        }

        // `[-m]` - set marked pane
        #[cfg(feature = "tmux_2_1")]
        if self.clear_marked {
            cmd.push_flag(M_LOWERCASE_KEY);
        }

        // `[-R]` - pane right
        #[cfg(feature = "tmux_1_3")]
        if self.right {
            cmd.push_flag(R_UPPERCASE_KEY);
        }

        // `[-U]` - pane above
        #[cfg(feature = "tmux_1_3")]
        if self.up {
            cmd.push_flag(U_UPPERCASE_KEY);
        }

        // `[-Z]` - keep the window zoomed if it was zoomed
        #[cfg(feature = "tmux_3_1")]
        if self.keep_zoomed {
            cmd.push_flag(Z_UPPERCASE_KEY);
        }

        // `[-P style]` - set the style for a single pane
        #[cfg(feature = "tmux_2_1")]
        if let Some(style) = self.style {
            cmd.push_option(P_UPPERCASE_KEY, style);
        }

        // `[-T title]` - title
        #[cfg(feature = "tmux_2_6")]
        if let Some(title) = self.title {
            cmd.push_option(T_UPPERCASE_KEY, title);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_1_0")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        cmd
    }
}
