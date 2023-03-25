use crate::options::*;
use crate::{SetOption, SetUserOption, TmuxCommand};
use std::borrow::Cow;

pub struct SetPaneOption;

impl SetPaneOptionTrait for SetPaneOption {}

impl SetUserOption for SetPaneOption {}

impl SetOptionExt for SetPaneOption {
    // unset if value = None
    fn set<'a, U: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>, S: Into<Cow<'a, str>>>(
        target: Option<U>,
        name: T,
        value: Option<S>,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().pane().option(name);
        let cmd = match target {
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            Some(target) => cmd.target(target),
            #[cfg(feature = "tmux_3_0")]
            Some(target) => cmd.target_pane(target),
            None => cmd,
        };
        let cmd = match value {
            Some(value) => cmd.value(value),
            None => cmd.unset(),
        };
        cmd.build()
    }

    fn unset<'a, S: Into<Cow<'a, str>>, T: Into<Cow<'a, str>>>(
        target: Option<S>,
        name: T,
    ) -> TmuxCommand<'a> {
        let cmd = SetOption::new().pane().option(name).unset();
        let cmd = match target {
            #[cfg(all(feature = "tmux_1_2", not(feature = "tmux_3_0")))]
            Some(target) => cmd.target(target),
            #[cfg(feature = "tmux_3_0")]
            Some(target) => cmd.target_pane(target),
            None => cmd,
        };
        cmd.build()
    }
}

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait SetPaneOptionTrait: SetOptionExt {
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn allow_rename<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set(target, ALLOW_RENAME, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set(target, ALTERNATE_SCREEN, switch.map(|s| s.to_string()))
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
    fn remain_on_exit<'a, S>(
        target: Option<S>,
        remain_on_exit: Option<RemainOnExit>,
    ) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set(
            target,
            REMAIN_ON_EXIT,
            remain_on_exit.map(|s| s.to_string()),
        )
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set(target, WINDOW_ACTIVE_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style<'a, S, T>(target: Option<S>, style: Option<T>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::set(target, WINDOW_STYLE, style)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes<'a, S>(target: Option<S>, switch: Option<Switch>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::set(target, SYNCHRONIZE_PANES, switch.map(|s| s.to_string()))
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    fn user_option<'a, S, T>(target: Option<S>, name: T, value: Option<String>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>> + std::fmt::Display,
    {
        Self::set(target, format!("{}{}", USER_OPTION_MARKER, name), value)
    }
}
