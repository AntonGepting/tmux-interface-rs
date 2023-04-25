use crate::{GetPaneOption, GetPaneOptionTr, TmuxCommand, TmuxCommandList};

use std::borrow::Cow;
// XXX: add all() method?
pub trait GetPaneOptionsTr<'a> {
    type Getter: GetPaneOptionTr;

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
    fn allow_rename<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::allow_rename(target));
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn alternate_screen<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::alternate_screen(target));
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
    fn remain_on_exit<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::remain_on_exit(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_active_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::window_active_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    fn window_style<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::window_style(target));
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    fn synchronize_panes<S>(mut self, target: Option<S>) -> Self
    where
        S: Into<Cow<'a, str>>,
        Self: Sized,
    {
        self.push(GetPaneOption::synchronize_panes(target));
        self
    }

    fn build(self) -> TmuxCommandList<'a>;
}
