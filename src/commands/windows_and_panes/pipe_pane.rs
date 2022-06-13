use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// tmux ^2.7:
/// ```text
/// tmux pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.2:
/// ```text
/// tmux pipe-pane [-o] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux ^1.1:
/// ```text
/// tmux pipe-pane [-o] [-t target-pane] [command]
/// (alias: pipep)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PipePane<'a> {
    /// `[-I]` - stdin is connected
    #[cfg(feature = "tmux_2_7")]
    pub stdout: bool,

    /// `[-O]` - stdout is connected
    #[cfg(feature = "tmux_2_7")]
    pub stdin: bool,

    /// `[-o]` - only open a new pipe if no previous pipe exists
    #[cfg(feature = "tmux_1_1")]
    pub open: bool,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_1")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> PipePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-I]` - stdin is connected
    #[cfg(feature = "tmux_2_7")]
    pub fn stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    /// `[-O]` - stdout is connected
    #[cfg(feature = "tmux_2_7")]
    pub fn stdin(mut self) -> Self {
        self.stdin = true;
        self
    }

    /// `[-o]` - only open a new pipe if no previous pipe exists
    #[cfg(feature = "tmux_1_1")]
    pub fn open(mut self) -> Self {
        self.open = true;
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_1")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(mut self, target_pane: S) -> Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(mut self, shell_command: S) -> Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    pub fn build(self) -> TmuxCommand<'a> {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(PIPE_PANE);

        // `[-I]` - stdin is connected
        #[cfg(feature = "tmux_2_7")]
        if self.stdout {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-O]` - stdout is connected
        #[cfg(feature = "tmux_2_7")]
        if self.stdin {
            cmd.push_flag(O_UPPERCASE_KEY);
        }

        // `[-o]` - only open a new pipe if no previous pipe exists
        #[cfg(feature = "tmux_1_1")]
        if self.open {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_1_1")]
        if let Some(target_pane) = self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane);
        }

        // `[shell-command]` - shell-command
        #[cfg(feature = "tmux_1_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
