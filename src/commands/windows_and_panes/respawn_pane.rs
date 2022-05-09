use crate::commands::constants::*;
use crate::TmuxCommand;
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
#[derive(Debug, Default, Clone)]
pub struct RespawnPane<'a> {
    /// `[-k]` - any existing command is killed
    #[cfg(feature = "tmux_1_5")]
    pub kill: bool,

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub target_pane: Option<Cow<'a, str>>,

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_2_6")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> RespawnPane<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-k]` - any existing command is killed
    #[cfg(feature = "tmux_1_5")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = true;
        self
    }

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(&mut self, start_directory: S) -> &mut Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub fn environment<S: Into<Cow<'a, str>>>(&mut self, environment: S) -> &mut Self {
        self.environment = Some(environment.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_1_5")]
    pub fn target_pane<S: Into<Cow<'a, str>>>(&mut self, target_pane: S) -> &mut Self {
        self.target_pane = Some(target_pane.into());
        self
    }

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_2_6")]
    pub fn shell_command<S: Into<Cow<'a, str>>>(&mut self, shell_command: S) -> &mut Self {
        self.shell_command = Some(shell_command.into());
        self
    }

    pub fn build(&self) -> TmuxCommand {
        let mut cmd = TmuxCommand::new();

        cmd.cmd(RESPAWN_PANE);

        // `[-k]` - any existing command is killed
        #[cfg(feature = "tmux_1_5")]
        if self.kill {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-c start-directory]` - start-directory
        #[cfg(feature = "tmux_2_6")]
        if let Some(start_directory) = &self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory.as_ref());
        }

        // `[-e environment]` - environment
        #[cfg(feature = "tmux_3_0")]
        if let Some(environment) = &self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment.as_ref());
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_1_5")]
        if let Some(target_pane) = &self.target_pane {
            cmd.push_option(T_LOWERCASE_KEY, target_pane.as_ref());
        }

        // `[shell-command]` - shell-command
        #[cfg(feature = "tmux_2_6")]
        if let Some(shell_command) = &self.shell_command {
            cmd.push_param(shell_command.as_ref());
        }

        cmd
    }
}
