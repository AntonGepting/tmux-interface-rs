use crate::{RemainOnExit, SetPaneOption, Switch, TmuxCommands};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug)]
pub struct SetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetPaneOptions<'a> {
    pub fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn allow_rename(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetPaneOption::allow_rename(switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetPaneOption::alternate_screen(switch));
        self
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
    pub fn remain_on_exit(mut self, switch: Option<RemainOnExit>) -> Self {
        self.options.push(SetPaneOption::remain_on_exit(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options.push(SetPaneOption::window_active_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self {
        self.options.push(SetPaneOption::window_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes(mut self, switch: Option<Switch>) -> Self {
        self.options.push(SetPaneOption::synchronize_panes(switch));
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<S: fmt::Display>(mut self, name: S, value: Option<String>) -> Self {
        self.options.push(SetPaneOption::user_option(name, value));
        self
    }

    pub fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}
