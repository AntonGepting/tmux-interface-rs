use crate::options::*;
use crate::{ShowOptions, TmuxCommand};
use std::borrow::Cow;
use std::fmt;

pub struct GetPaneOption;

pub struct GetGlobalPaneOption(GetPaneOption);

impl std::ops::Deref for GetGlobalPaneOption {
    type Target = GetPaneOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Getter for GetGlobalPaneOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .pane()
            .global()
            .option(name.into())
            .value()
            .build()
    }
}

pub trait Getter {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new().pane().option(name).value().build()
    }
}

impl Getter for GetPaneOption {
    fn get<'a, T: Into<Cow<'a, str>>>(name: T) -> TmuxCommand<'a> {
        ShowOptions::new()
            .pane()
            .option(name.into())
            .value()
            .build()
    }
}
// NOTE: method avoiding names like set_set_clipboard
// NOTE: multiple commands should be avoided in case short form is used (only the value will be returned
// back) bc. not possible to differentiate between multi line array option value and single line
// option value
//
impl GetPaneOption {
    /// ### Manual
    ///
    /// tmux ^3.1:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn allow_rename<'a>() -> TmuxCommand<'a> {
        Self::get(ALLOW_RENAME)
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen<'a>() -> TmuxCommand<'a> {
        Self::get(ALTERNATE_SCREEN)
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
    pub fn remain_on_exit<'a>() -> TmuxCommand<'a> {
        Self::get(REMAIN_ON_EXIT)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style<'a>() -> TmuxCommand<'a> {
        Self::get(WINDOW_ACTIVE_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_style<'a>() -> TmuxCommand<'a> {
        Self::get(WINDOW_STYLE)
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes<'a>() -> TmuxCommand<'a> {
        Self::get(SYNCHRONIZE_PANES)
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<'a, S: fmt::Display>(name: S) -> TmuxCommand<'a> {
        Self::get(format!("{}{}", USER_OPTION_MARKER, name))
    }
}

#[test]
fn get_pane_option_c() {
    let cmd = Tmux::new()
        .command(GetPaneOption::get(BUFFER_LIMIT))
        .output()
        .unwrap();
    let cmd = Tmux::new()
        .command(GetPaneOption::buffer_limit())
        .command(GetPaneOption::set_clipboard())
        .output()
        .unwrap();
    dbg!(&cmd);

    let cmd = Tmux::new()
        .command(GetPaneOption::command_alias())
        .output()
        .unwrap();
    let cmd = TmuxServerOptionOutput::from(cmd).command_alias();
    dbg!(&cmd);

    let cmds = SetPaneOption::command_alias(Some(vec!["asdf".to_string(), "a".to_string()]));
    dbg!(&cmds);
}
