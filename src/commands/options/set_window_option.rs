use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.0:
/// ```text
/// (removed)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux set-window-option [-aFgoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.9:
/// ```text
/// tmux set-window-option [-agoqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux set-window-option [-agqu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux set-window-option [-agu] [-t target-window] option value
/// (alias: setw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux set-window-option [-gu] [-t target-window] option value
/// (alias: setw)
/// ```
#[derive(Debug, Clone)]
pub struct SetWindowOption<'a>(pub TmuxCommand<'a>);

impl<'a> Default for SetWindowOption<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SET_WINDOW_OPTION)),
            ..Default::default()
        })
    }
}

impl<'a> SetWindowOption<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-a]` -
    #[cfg(feature = "tmux_1_0")]
    pub fn append(&mut self) -> &mut Self {
        self.0.push_flag(A_LOWERCASE_KEY);
        self
    }

    /// `[-F]` -
    #[cfg(feature = "tmux_2_6")]
    pub fn format(&mut self) -> &mut Self {
        self.0.push_flag(F_UPPERCASE_KEY);
        self
    }

    /// `[-g]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-o]` -
    #[cfg(feature = "tmux_1_9")]
    pub fn not_overwrite(&mut self) -> &mut Self {
        self.0.push_flag(O_LOWERCASE_KEY);
        self
    }

    /// `[-q]` -
    #[cfg(feature = "tmux_1_7")]
    pub fn quiet(&mut self) -> &mut Self {
        self.0.push_flag(Q_LOWERCASE_KEY);
        self
    }

    /// `[-u]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn unset(&mut self) -> &mut Self {
        self.0.push_flag(U_LOWERCASE_KEY);
        self
    }

    /// `[-t target-window]` -
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `option`
    pub fn option<S: Into<Cow<'a, str>>>(&mut self, option: S) -> &mut Self {
        self.0.push_param(option);
        self
    }

    /// `value`
    pub fn value<S: Into<Cow<'a, str>>>(&mut self, value: S) -> &mut Self {
        self.0.push_param(value);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for SetWindowOption<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SET_WINDOW_OPTION)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for SetWindowOption<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SET_WINDOW_OPTION)),
            ..Default::default()
        })
    }
}
