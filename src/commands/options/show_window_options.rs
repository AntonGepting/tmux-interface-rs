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
/// tmux ^1.8:
/// ```text
/// tmux show-window-options [-gv] [-t target-window] [option]
/// (alias: showw)
/// ```
///
/// tmux ^1.7:
/// ```text
/// tmux show-window-options [-g] [-t target-window] [option]
/// (alias: showw)
/// ```
///
/// tmux ^1.0:
/// ```text
/// tmux show-window-options [-g] [-t target-window]
/// (alias: showw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux show-window-options [-t target-window] option value
/// (alias: showw)
/// ```
#[derive(Debug, Clone)]
pub struct ShowWindowOptions<'a>(pub TmuxCommand<'a>);

impl<'a> Default for ShowWindowOptions<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(SHOW_WINDOW_OPTIONS)),
            ..Default::default()
        })
    }
}

impl<'a> ShowWindowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub fn global(&mut self) -> &mut Self {
        self.0.push_flag(G_LOWERCASE_KEY);
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_1_8")]
    pub fn only_value(&mut self) -> &mut Self {
        self.0.push_flag(V_LOWERCASE_KEY);
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(&mut self, target_window: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_window);
        self
    }

    /// `option`
    #[cfg(feature = "tmux_0_8")]
    pub fn option<S: Into<Cow<'a, str>>>(&mut self, option: S) -> &mut Self {
        self.0.push_param(option);
        self
    }

    /// `value`
    #[cfg(feature = "tmux_0_8")]
    pub fn value<S: Into<Cow<'a, str>>>(&mut self, value: S) -> &mut Self {
        self.0.push_param(value);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for ShowWindowOptions<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(SHOW_WINDOW_OPTIONS)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for ShowWindowOptions<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(SHOW_WINDOW_OPTIONS)),
            ..Default::default()
        })
    }
}
