// auto-generated file
//

use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

pub type PipeP<'a> = PipePane<'a>;

/// Pipe output sent by the program in target-pane to a shell command or vice versa
///
/// # Manual
///
/// tmux >=2.7:
/// ```text
/// pipe-pane [-IOo] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
///
/// tmux >=1.5:
/// ```text
/// pipe-pane [-o] [-t target-pane] [shell-command]
/// (alias: pipep)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct PipePane<'a> {
    /// `[-I]`
    #[cfg(feature = "tmux_2_7")]
    pub stdout: bool,

    /// `[-O]`
    #[cfg(feature = "tmux_2_7")]
    pub stdin: bool,

    /// `[-o]`
    #[cfg(feature = "tmux_1_5")]
    pub open: bool,

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[shell-command]`
    #[cfg(feature = "tmux_1_5")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> PipePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-I]`
    #[cfg(feature = "tmux_2_7")]
    pub fn stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    /// `[-O]`
    #[cfg(feature = "tmux_2_7")]
    pub fn stdin(mut self) -> Self {
        self.stdin = true;
        self
    }

    /// `[-o]`
    #[cfg(feature = "tmux_1_5")]
    pub fn open(mut self) -> Self {
        self.open = true;
        self
    }

    /// `[-t target-pane]`
    #[cfg(feature = "tmux_1_5")]
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

        cmd.name(PIPE_PANE);

        // `[-I]`
        #[cfg(feature = "tmux_2_7")]
        if self.stdout {
            cmd.push_flag(I_UPPERCASE_KEY);
        }

        // `[-O]`
        #[cfg(feature = "tmux_2_7")]
        if self.stdin {
            cmd.push_flag(O_UPPERCASE_KEY);
        }

        // `[-o]`
        #[cfg(feature = "tmux_1_5")]
        if self.open {
            cmd.push_flag(O_LOWERCASE_KEY);
        }

        // `[-t target-pane]`
        #[cfg(feature = "tmux_1_5")]
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
