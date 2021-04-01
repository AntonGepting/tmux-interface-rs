use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
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
#[derive(Debug, Clone)]
pub struct PipePane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for PipePane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(PIPE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> PipePane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-I]` - stdin is connected
    #[cfg(feature = "tmux_2_7")]
    pub fn stdout(&mut self) -> &mut Self {
        self.0.push_flag(I_UPPERCASE_KEY);
        self
    }

    /// `[-O]` - stdout is connected
    #[cfg(feature = "tmux_2_7")]
    pub fn stdin(&mut self) -> &mut Self {
        self.0.push_flag(O_UPPERCASE_KEY);
        self
    }

    /// `[-o]` - only open a new pipe if no previous pipe exists
    #[cfg(feature = "tmux_1_1")]
    pub fn open(&mut self) -> &mut Self {
        self.0.push_flag(O_LOWERCASE_KEY);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_1")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for PipePane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(PIPE_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for PipePane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(PIPE_PANE)),
            ..Default::default()
        })
    }
}
