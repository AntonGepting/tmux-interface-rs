use crate::options::*;
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;
use std::fmt;

pub struct SetPaneOption;

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl SetPaneOption {
    fn set<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        match value {
            Some(data) => Self::set_ext(name, Some(data)),
            None => Self::unset(name),
        }
    }

    fn unset<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        SetOption::new().pane().option(name).unset().build()
    }

    // unset if value = None
    fn set_ext<'a, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        //(self.setter)(name.into(), value.map(|s| s.into()))
        let cmd = match value {
            Some(data) => SetOption::new().pane().option(name).value(data),
            None => SetOption::new().pane().option(name),
        };
        cmd.build()
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn allow_rename<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(ALLOW_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(ALTERNATE_SCREEN, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn remain_on_exit<'a>(remain_on_exit: Option<RemainOnExit>) -> TmuxCommand<'a> {
        Self::set(REMAIN_ON_EXIT, remain_on_exit.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(WINDOW_ACTIVE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_style<'a, S: Into<Cow<'a, str>>>(style: Option<S>) -> TmuxCommand<'a> {
        Self::set(WINDOW_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes<'a>(switch: Option<Switch>) -> TmuxCommand<'a> {
        Self::set(SYNCHRONIZE_PANES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<'a, S: fmt::Display>(name: S, value: Option<String>) -> TmuxCommand<'a> {
        Self::set(format!("{}{}", USER_OPTION_MARKER, name), value)
    }
}
