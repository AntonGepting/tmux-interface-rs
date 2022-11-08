use crate::{
    GetPaneOption, GetPaneOptionTrait, GetUserOption, GetUserOptions, TmuxCommand, TmuxCommands,
};
use std::fmt;

#[derive(Debug)]
pub struct GetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetPaneOptionsTrait<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn new() -> Self
    where
        Self: Sized,
    {
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

impl<'a> GetUserOptions<'a> for GetPaneOptions<'a> {
    type Getter = GetPaneOption;

    fn push(&mut self, option: TmuxCommand<'a>) {
        self.options.push(option);
    }
}

pub trait GetPaneOptionsTrait<'a> {
    type Getter: GetPaneOptionTrait;

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
    fn allow_rename(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::allow_rename());
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::alternate_screen());
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
    fn remain_on_exit(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::remain_on_exit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::window_active_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::window_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes(mut self) -> Self
    where
        Self: Sized,
    {
        self.push(GetPaneOption::synchronize_panes());
        self
    }

    fn build(self) -> TmuxCommands<'a>;
}
