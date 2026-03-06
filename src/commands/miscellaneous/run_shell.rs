// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type Run<'a> = RunShell<'a>;

/// Execute shell-command using `/bin/sh`
///
/// # Manual
///
/// tmux >=3.6:
/// ```text
/// run-shell [-bCE] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux >=3.2:
/// ```text
/// run-shell [-bC] [-d delay] [-t target-pane] [shell-command]
/// (alias: run)
/// ```
///
/// tmux >=1.8:
/// ```text
/// run-shell [-b] [-t target-pane] shell-command
/// (alias: run)
/// ```
///
/// tmux >=1.5:
/// ```text
/// run-shell shell-command
/// (alias: run)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RunShell<'a> {
    /// `[-b]`
    #[cfg(feature = "tmux_1_8")]
    pub background: bool,

    /// `[-C]`
    #[cfg(feature = "tmux_3_2")]
    pub tmux_command: bool,

    /// `[-E]`
    #[cfg(feature = "tmux_3_6")]
    pub redirect_stderr: bool,

    /// `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    pub delay: Option<Cow<'a, str>>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> RunShell<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-b]`
    #[cfg(feature = "tmux_1_8")]
    pub fn background(mut self) -> Self {
        self.background = true;
        self
    }

    /// `[-C]`
    #[cfg(feature = "tmux_3_2")]
    pub fn tmux_command(mut self) -> Self {
        self.tmux_command = true;
        self
    }

    /// `[-E]`
    #[cfg(feature = "tmux_3_6")]
    pub fn redirect_stderr(mut self) -> Self {
        self.redirect_stderr = true;
        self
    }

    /// `[-d delay]`
    #[cfg(feature = "tmux_3_2")]
    pub fn delay<S: Into<Cow<'a, str>>>(mut self, delay: S) -> Self {
        self.delay = Some(delay.into());
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// build command with arguments in right order
    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.name(RUN_SHELL);

        // `[-b]`
        #[cfg(feature = "tmux_1_8")]
        if self.background {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-C]`
        #[cfg(feature = "tmux_3_2")]
        if self.tmux_command {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-E]`
        #[cfg(feature = "tmux_3_6")]
        if self.redirect_stderr {
            cmd.push_flag(E_UPPERCASE_KEY);
        }

        // `[-d delay]`
        #[cfg(feature = "tmux_3_2")]
        if let Some(delay) = self.delay {
            cmd.push_option(D_LOWERCASE_KEY, delay);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[shell-command]`
        #[cfg(feature = "tmux_1_5")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
