use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

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
#[derive(Debug, Clone)]
pub struct SelectPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SelectPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SELECT_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> SelectPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-D]` - pane below
    #[cfg(feature = "tmux_1_3")]
    pub fn down(&mut self) -> &mut Self {
        self.0.push_flag(D_UPPERCASE_KEY);
        self
    }

    /// `[-d]` - disable input
    #[cfg(feature = "tmux_2_0")]
    pub fn disable(&mut self) -> &mut Self {
        self.0.push_flag(D_LOWERCASE_KEY);
        self
    }

    /// `[-e]` - enable input
    #[cfg(feature = "tmux_2_0")]
    pub fn enable(&mut self) -> &mut Self {
        self.0.push_flag(E_LOWERCASE_KEY);
        self
    }

    /// `[-g]` - show the current pane style
    #[cfg(feature = "tmux_2_1")]
    pub fn show_style(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-L]` - pane left
    #[cfg(feature = "tmux_1_3")]
    pub fn left(&mut self) -> &mut Self {
        self.0.push_flag(L_UPPERCASE_KEY);
        self
    }

    /// `[-l]` - equivalent to last-pane command
    #[cfg(feature = "tmux_1_5")]
    pub fn last(&mut self) -> &mut Self {
        self.0.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-M]` - clear marked pane
    #[cfg(feature = "tmux_2_1")]
    pub fn set_marked(&mut self) -> &mut Self {
        self.0.push_flag(M_UPPERCASE_KEY);
        self
    }

    /// `[-m]` - set marked pane
    #[cfg(feature = "tmux_2_1")]
    pub fn clear_marked(&mut self) -> &mut Self {
        self.0.push_flag(M_LOWERCASE_KEY);
        self
    }

    /// `[-R]` - pane right
    #[cfg(feature = "tmux_1_3")]
    pub fn right(&mut self) -> &mut Self {
        self.0.push_flag(R_UPPERCASE_KEY);
        self
    }

    /// `[-U]` - pane above
    #[cfg(feature = "tmux_1_3")]
    pub fn up(&mut self) -> &mut Self {
        self.0.push_flag(U_UPPERCASE_KEY);
        self
    }

    /// `[-Z]` - keep the window zoomed if it was zoomed
    #[cfg(feature = "tmux_3_1")]
    pub fn keep_zoomed(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-P style]` - set the style for a single pane
    #[cfg(feature = "tmux_2_1")]
    pub fn style<S: Into<Cow<'a, str>>>(&mut self, style: S) -> &mut Self {
        self.0.push_option(P_UPPERCASE_KEY, style);
        self
    }

    /// `[-T title]` - title
    #[cfg(feature = "tmux_2_6")]
    pub fn title<S: Into<Cow<'a, str>>>(&mut self, title: S) -> &mut Self {
        self.0.push_option(T_UPPERCASE_KEY, title);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_0")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SelectPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SELECT_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SelectPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SELECT_PANE)),
            ..Default::default()
        })
    }
}
