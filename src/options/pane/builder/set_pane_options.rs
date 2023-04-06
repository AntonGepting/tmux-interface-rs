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
    fn allow_rename<S>(mut self, target: Option<S>, switch: Option<Switch>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::allow_rename(target, switch));
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen<S>(mut self, target: Option<S>, switch: Option<Switch>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::alternate_screen(target, switch));
        self
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
    fn remain_on_exit<S>(mut self, target: Option<S>, switch: Option<RemainOnExit>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::remain_on_exit(target, switch));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style<S, T>(mut self, target: Option<T>, style: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::window_active_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style<S, T>(mut self, target: Option<S>, style: Option<T>) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::window_style(target, style));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes<S>(mut self, target: Option<S>, switch: Option<Switch>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(SetPaneOption::synchronize_panes(target, switch));
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
