use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RunShell<'a> {
    /// `[-b]`
    #[cfg(feature = "tmux_1_8")]
    pub background: bool,

    /// `[-C]` - execute tmux command
    #[cfg(feature = "tmux_3_2")]
    pub tmux_command: bool,

    /// `[-d delay]`
    #[cfg(feature = "tmux_1_8")]
    pub delay: Option<usize>,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `shell-command`
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<Cow<'a, str>>,

    /// `command`
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    pub command: Option<Cow<'a, str>>,
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

    /// `[-C]` - execute tmux command
    #[cfg(feature = "tmux_3_2")]
    pub fn tmux_command(mut self) -> Self {
        self.tmux_command = true;
        self
    }

    /// `[-d delay]`
    #[cfg(feature = "tmux_1_8")]
    pub fn delay(mut self, delay: usize) -> Self {
        self.delay = Some(delay);
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_8")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `shell-command`
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    /// `command`
    #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
    pub fn command<S: Into<Cow<'a, str>>>(mut self, command: S) -> Self {
        self.command = Some(command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(RUN_SHELL);

        // `[-b]`
        #[cfg(feature = "tmux_1_8")]
        if self.background {
            cmd.push_flag(B_LOWERCASE_KEY);
        }

        // `[-C]` - execute tmux command
        #[cfg(feature = "tmux_3_2")]
        if self.tmux_command {
            cmd.push_flag(C_UPPERCASE_KEY);
        }

        // `[-d delay]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(delay) = self.delay {
            cmd.push_option(D_LOWERCASE_KEY, delay.to_string());
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_8")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `shell-command`
        #[cfg(feature = "tmux_1_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        // `command`
        #[cfg(all(feature = "tmux_1_1", not(feature = "tmux_1_2")))]
        if let Some(command) = self.command {
            cmd.push_param(command);
        }

        cmd
    }
}
