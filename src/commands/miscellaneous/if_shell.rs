use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type If<'a> = IfShell<'a>;

// XXX: solution for command [command]
/// Structure for conditional commands executing
///
/// # Manual
///
/// tmux ^2.0:
/// ```text
/// if-shell [-bF] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.8:
/// ```text
/// if-shell [-b] [-t target-pane] shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^1.6:
/// ```text
/// if-shell shell-command command [command]
/// (alias: if)
/// ```
///
/// tmux ^0.8:
/// ```text
/// if-shell shell-command command
/// (alias: if)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct IfShell<'a> {
    /// `[-b]` - run in the background
    #[cfg(feature = "tmux_1_8")]
    pub background: bool,

    /// `[-F]` not execute but considered success if neither empty nor zero
    #[cfg(feature = "tmux_2_0")]
    pub not_execute: bool,

    /// `[-t target-pane]` specify the target-pane
    #[cfg(feature = "tmux_1_8")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub shell_command: Option<Cow<'a, str>>,

    /// `[command]` - specify the second command
    #[cfg(feature = "tmux_0_8")]
    pub command: Option<Cow<'a, str>>,
}

impl<'a> IfShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]` - run in the background
    #[cfg(feature = "tmux_1_8")]
    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    /// `[-F]` not execute but considered success if neither empty nor zero
    #[cfg(feature = "tmux_2_0")]
    pub fn not_execute(mut self) -> Self {
        self.not_execute = true;
        self
    }

    /// `[-t target-pane]` specify the target-pane
    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_0_8")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// `[command]` - specify the second command
    #[cfg(feature = "tmux_0_8")]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(IF_SHELL);

        // `[-b]` - run in the background
        #[cfg(feature = "tmux_1_8")]
        if self.background {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-F]` not execute but considered success if neither empty nor zero
        #[cfg(feature = "tmux_2_0")]
        if self.not_execute {
            cmd.push_flag(F_UPPERCASE_KEY);
        }

        // `[-t target-pane]` specify the target-pane
        #[cfg(feature = "tmux_1_8")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_0_8")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        // `[command]` - specify the second command
        #[cfg(feature = "tmux_0_8")]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
