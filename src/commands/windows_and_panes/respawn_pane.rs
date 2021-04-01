use crate::commands::constants::*;
use crate::{Error, TmuxCommand, TmuxOutput};
use std::borrow::Cow;

/// Reactivate a pane in which the command has exited
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-e environment] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^2.6:
/// ```text
/// tmux respawn-pane [-k] [-c start-directory] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
///
/// tmux ^1.5:
/// ```text
/// tmux respawn-pane [-k] [-t target-pane] [shell-command]
/// (alias: respawnp)
/// ```
#[derive(Debug, Clone)]
pub struct RespawnPane<'a>(pub TmuxCommand<'a>);

impl<'a> Default for RespawnPane<'a> {
    fn default() -> Self {
        Self(TmuxCommand {
            cmd: Some(Cow::Borrowed(RESPAWN_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> RespawnPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-k]` - any existing command is killed
    #[cfg(feature = "tmux_1_5")]
    pub fn kill(&mut self) -> &mut Self {
        self.0.push_flag(K_LOWERCASE_KEY);
        self
    }

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, start_directory: S) -> &mut Self {
        self.0.push_option(C_LOWERCASE_KEY, start_directory);
        self
    }

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub fn environment<S: Into<Cow<'a, str>>>(&mut self, environment: S) -> &mut Self {
        self.0.push_option(E_LOWERCASE_KEY, environment);
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.0.push_option(T_LOWERCASE_KEY, target_pane);
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_2_6")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.0.push_param(shell_command);
        self
    }

    pub fn output(&self) -> Result<TmuxOutput, Error> {
        self.0.output()
    }
}

impl<'a> From<TmuxCommand<'a>> for RespawnPane<'a> {
    fn from(item: TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin,
            cmd: Some(Cow::Borrowed(RESPAWN_PANE)),
            ..Default::default()
        })
    }
}

impl<'a> From<&TmuxCommand<'a>> for RespawnPane<'a> {
    fn from(item: &TmuxCommand<'a>) -> Self {
        Self(TmuxCommand {
            bin: item.bin.clone(),
            cmd: Some(Cow::Borrowed(RESPAWN_PANE)),
            ..Default::default()
        })
    }
}
