use crate::{RemainOnExit, SetPaneOption, SetPaneOptionTr, Switch, TmuxCommand, TmuxCommandList};

use std::borrow::Cow;
pub trait SetPaneOptionsTr<'a> {
    type Setter: SetPaneOptionTr;

    fn new() -> Self;

    fn push(&mut self, option: TmuxCommand<'a>);

    fn push_cmds(&mut self, options: TmuxCommandList<'a>);

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

    fn build(self) -> TmuxCommandList<'a>;
}
