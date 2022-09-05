use crate::GetPaneOption;
use crate::TmuxCommands;
use std::fmt;

#[derive(Debug)]
pub struct GetPaneOptions<'a> {
    pub options: TmuxCommands<'a>,
}

impl<'a> GetPaneOptions<'a> {
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
    pub fn allow_rename(mut self) -> Self {
        self.options.push(GetPaneOption::allow_rename());
        self
    }
    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// alternate-screen [on | off]
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn alternate_screen(mut self) -> Self {
        self.options.push(GetPaneOption::alternate_screen());
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
    pub fn remain_on_exit(mut self) -> Self {
        self.options.push(GetPaneOption::remain_on_exit());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-active-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_active_style(mut self) -> Self {
        self.options.push(GetPaneOption::window_active_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.0:
    /// ```text
    /// window-style style
    /// ```
    #[cfg(feature = "tmux_3_0")]
    pub fn window_style(mut self) -> Self {
        self.options.push(GetPaneOption::window_style());
        self
    }

    /// ### Manual
    ///
    /// tmux ^3.2:
    /// ```text
    /// synchronize-panes [on | off]
    /// ```
    #[cfg(feature = "tmux_3_2")]
    pub fn synchronize_panes(mut self) -> Self {
        self.options.push(GetPaneOption::synchronize_panes());
        self
    }

    /// ### Manual
    ///
    /// ```text
    /// user option
    /// ```
    pub fn user_option<T: fmt::Display>(mut self, name: T) -> Self {
        self.options.push(GetPaneOption::user_option(name));
        self
    }

    pub fn build(self) -> TmuxCommands<'a> {
        self.options
    }
}
