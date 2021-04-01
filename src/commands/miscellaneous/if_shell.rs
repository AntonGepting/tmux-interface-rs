use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Structure for conditional commands executing
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// tmux if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.8:
/// ```text
/// tmux if-shell [-b] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.6:
/// ```text
/// tmux if-shell shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux if-shell shell-command command
/// (alias: if)
/// ```

#[derive(Debug, Clone)]
pub struct IfShell<'a>(pub TmuxCommand<'a>);

impl<'a> Default for IfShell<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(IF_SHELL)),
            ..Default::default()
        })
    }
}

impl<'a> IfShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - run in the background
    #[cfg(feature = "tmux_1_8")]
    pub fn backgroud(&mut self) -> &mut Self {
        self.0.push_flag(B_LOWERCASE_KEY);
        self
    }

    /// `[-F]` not execute but considered success if neither empty nor zero
    #[cfg(feature = "tmux_2_0")]
    pub fn not_execute(&mut self) -> &mut Self {
        self.0.push_flag(F_UPPERCASE_KEY);
        self
    }

    /// `[-t target-pane]` specify the target-pane
    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    /// `[command]` - specify the second command
    #[cfg(feature = "tmux_0_8")]
    pub fn command<S: Into<Cow<'a, str>>>(&mut self, command: S) -> &mut Self {
        self.0.push_param(command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for IfShell<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(IF_SHELL)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for IfShell<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(IF_SHELL)),
            ..Default::default()
        })
    }
}
