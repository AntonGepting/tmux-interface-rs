use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ShowWindowOptions<'a> {
    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub global: bool,

    /// `[-v]`
    #[cfg(feature = "tmux_1_8")]
    pub only_value: bool,

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `option`
    #[cfg(feature = "tmux_0_8")]
    pub option: Option<Cow<'a, str>>,

    /// `value`
    #[cfg(feature = "tmux_0_8")]
    pub value: Option<Cow<'a, str>>,
}

impl<'a> ShowWindowOptions<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-g]`
    #[cfg(feature = "tmux_1_0")]
    pub fn global(mut self) -> Self {
        self.global = true;
        self
    }

    /// `[-v]`
    #[cfg(feature = "tmux_1_8")]
    pub fn only_value(mut self) -> Self {
        self.only_value = true;
        self
    }

    /// `[-t target-window]`
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
        self
    }

    /// `option`
    #[cfg(feature = "tmux_0_8")]
    pub fn option<S: Into<Cow<'a, str>>>(mut self, option: S) -> Self {
        self.option = Some(option.into());
        self
    }

    /// `value`
    #[cfg(feature = "tmux_0_8")]
    pub fn value<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(SHOW_WINDOW_OPTIONS);

        // `[-g]`
        #[cfg(feature = "tmux_1_0")]
        if self.global {
            cmd.push_flag(G_LOWERCASE_KEY);
        }

        // `[-v]`
        #[cfg(feature = "tmux_1_8")]
        if self.only_value {
            cmd.push_flag(V_LOWERCASE_KEY);
        }

        // `[-t target-window]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `option`
        #[cfg(feature = "tmux_0_8")]
        if let Some(option) = self.option {
            cmd.push_param(option);
        }

        // `value`
        #[cfg(feature = "tmux_0_8")]
        if let Some(value) = self.value {
            cmd.push_param(value);
        }

        cmd
    }
}
