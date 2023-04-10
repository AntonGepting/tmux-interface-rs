use crate::options::*;
use crate::{GetOptionTr, GetUserOption, TmuxCommand};
use std::borrow::Cow;

// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
pub trait GetPaneOptionTr: GetOptionTr + GetUserOption {
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn allow_rename<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, ALLOW_RENAME)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, ALTERNATE_SCREEN)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// remain-on-exit [on | off | failed]
    /// ```
    ///
    /// tmux ^3.0:
    /// ```text
    /// remain-on-exit [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn remain_on_exit<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, REMAIN_ON_EXIT)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_ACTIVE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, WINDOW_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes<'a, S>(target: Option<S>) -> TmuxCommand<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self::get_ext(target, SYNCHRONIZE_PANES)
    }
}
