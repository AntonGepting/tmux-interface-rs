use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// # Manual
///
/// tmux ^3.2:
/// ```text
/// tmux run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux run-shell [-b] [-t target-pane] shell-command
/// (alias: run)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux run-shell shell-command
/// (alias: run)
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux run-shell command
/// (alias: run)
/// ```
#[derive(Debug, Clone)]
pub struct RunShell<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RunShell<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RUN_SHELL)),
            ..Default::default()
        })
    }
}

impl<'a> RunShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_1_8")]
    pub fn backgroud(&mut self) -> &mut Self {
        self.0.push_flag(B_LOWERCASE_KEY);
        self
    }

    /// `[-C]` - execute tmux command
    #[cfg(feature = "tmux_3_2")]
    pub fn tmux_command(&mut self) -> &mut Self {
        self.0.push_flag(C_UPPERCASE_KEY);
        self
    }

    /// `[-d delay]`
    #[cfg(feature = "tmux_1_8")]
    pub fn delay(&mut self, delay: usize) -> &mut Self {
        self.0.push_option(D_LOWERCASE_KEY, delay.to_string());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `shell-command`
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    /// `command`
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RunShell<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RUN_SHELL)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RunShell<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RUN_SHELL)),
            ..Default::default()
        })
    }
}
