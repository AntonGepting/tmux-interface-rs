use crate::{
    RemainOnExit, SetPaneOption, SetPaneOptionTrait, SetUserOptions, Switch, TmuxCommand,
    TmuxCommands,
};
use std::borrow::Cow;

#[derive(Debug)]
pub struct SetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> SetPaneOptionsTrait<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

    fn new() -> Self {
        Self {
            options: TmuxCommands::new(),
        }
    }

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }

    fn push_cmds(&mut self, options: TmuxCommands<'a>) {
        self.options.push_cmds(options);
    }

    fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}

impl<'a> SetUserOptions<'a> for SetPaneOptions<'a> {
    type Setter = SetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

pub trait SetPaneOptionsTrait<'a> {
    type Setter: SetPaneOptionTrait;

    fn new() -> Self;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn push_cmds(&mut self, options: TmuxCommands<'a>);

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// allow-rename [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn allow_rename(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::allow_rename(switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::alternate_screen(switch));
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
    fn remain_on_exit(mut self, switch: Option<RemainOnExit>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::remain_on_exit(switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::window_active_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style<S: Into<Cow<'a, str>>>(mut self, style: Option<S>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::window_style(style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes(mut self, switch: Option<Switch>) -> Self
    where
        Self: Sized,
    {
        self.push(SetPaneOption::synchronize_panes(switch));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
