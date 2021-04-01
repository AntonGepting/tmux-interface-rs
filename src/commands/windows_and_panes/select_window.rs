use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Select the window at target-window.
///
/// # Manual
///
/// tmux ^1.8:
/// ```text
/// tmux select-window [-lnpT] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux select-window [-lnp] [-t target-window]
/// (alias: selectw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux select-window [-t target-window]
/// (alias: selectw)
/// ```
#[derive(Debug, Clone)]
pub struct SelectWindow<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SelectWindow<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SELECT_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> SelectWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-l]` - equivalent to last-window
    #[cfg(feature = "tmux_1_5")]
    pub fn last(&mut self) -> &mut Self {
        self.0.push_flag(L_LOWERCASE_KEY);
        self
    }

    /// `[-n]` - equivalent to next-window
    #[cfg(feature = "tmux_1_5")]
    pub fn next(&mut self) -> &mut Self {
        self.0.push_flag(N_LOWERCASE_KEY);
        self
    }

    /// `[-p]` - equivalent to previous-window
    #[cfg(feature = "tmux_1_5")]
    pub fn previous(&mut self) -> &mut Self {
        self.0.push_flag(P_LOWERCASE_KEY);
        self
    }

    /// `[-T]` - if the selected window is already the current window, behave like last-window
    #[cfg(feature = "tmux_1_8")]
    pub fn switch(&mut self) -> &mut Self {
        self.0.push_flag(T_UPPERCASE_KEY);
        self
    }

    /// `[-t target-window]` - target-window
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SelectWindow<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SELECT_WINDOW)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SelectWindow<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SELECT_WINDOW)),
            ..Default::default()
        })
    }
}
