use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Search for the fnmatch(3) pattern `match-string` in window names,
/// titles, and visible content (but not history)
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux find-window [-rCNTZ] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^2.6:
/// ```text
/// tmux find-window [-CNT] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^1.7:
/// ```text
/// tmux find-window [-CNT] [-F format] [-t target-pane] match-string
/// (alias: findw)
///
/// tmux ^0.8:
/// ```text
/// tmux find-window [-t target-pane] match-string
/// (alias: findw)
/// ```
#[derive(Debug, Clone)]
pub struct FindWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for FindWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(FIND_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> FindWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-r]` - regular expression
    #[cfg(feature = "tmux_3_0")]
    pub fn regex(&mut self) -> &mut Self {
        self.0.push_flag(R_LOWERCASE_KEY);
        self
    }

    /// `[-C]` - match only visible window contents
    #[cfg(feature = "tmux_1_7")]
    pub fn only_visible(&mut self) -> &mut Self {
        self.0.push_flag(C_UPPERCASE_KEY);
        self
    }

    /// `[-N]` - match only the window name
    #[cfg(feature = "tmux_1_7")]
    pub fn only_name(&mut self) -> &mut Self {
        self.0.push_flag(N_UPPERCASE_KEY);
        self
    }

    /// `[-T]` - match only the window title
    #[cfg(feature = "tmux_1_7")]
    pub fn only_title(&mut self) -> &mut Self {
        self.0.push_flag(T_UPPERCASE_KEY);
        self
    }

    /// `[-Z]` - zoom the pane
    #[cfg(feature = "tmux_3_0")]
    pub fn zoom(&mut self) -> &mut Self {
        self.0.push_flag(Z_UPPERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for FindWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(FIND_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for FindWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(FIND_WINDOW)),
            ..Default::default()
        })
    }
}
