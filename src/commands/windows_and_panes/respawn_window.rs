use crate::commands::constants::*;
use crate::TmuxCommand;
use std::borrow::Cow;

/// Reactivate a window in which the command has exited
///
/// # Manual
///
/// tmux ^3.0:
/// ```text
/// tmux respawn-window [-k] [-c start-directory] [-e environment] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
///
/// tmux ^2.6:
/// ```text
/// tmux respawn-window [-k] [-c start-directory] [-t target-window]
/// [shell-command]
/// (alias: respawnw)
///
/// tmux ^1.2:
/// ```text
/// tmux respawn-window [-k] [-t target-window] [shell-command]
/// (alias: respawnw)
/// ```
///
/// tmux ^0.8:
/// ```text
/// tmux respawn-window [-k] [-t target-window] [command]
/// (alias: respawnw)
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RespawnWindow<'a> {
    /// `[-k]` - any existing command is killed
    #[cfg(feature = "tmux_0_8")]
    pub kill: bool,

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<Cow<'a, str>>,

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<Cow<'a, str>>,

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<Cow<'a, str>>,

    /// `[shell-command]` - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<Cow<'a, str>>,
}

impl<'a> RespawnWindow<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    /// `[-k]` - any existing command is killed
    #[cfg(feature = "tmux_0_8")]
    pub fn kill(mut self) -> Self {
        self.kill = true;
        self
    }

    /// `[-c start-directory]` - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory<S: Into<Cow<'a, str>>>(mut self, start_directory: S) -> Self {
        self.start_directory = Some(start_directory.into());
        self
    }

    /// `[-e environment]` - environment
    #[cfg(feature = "tmux_3_0")]
    pub fn environment<S: Into<Cow<'a, str>>>(mut self, environment: S) -> Self {
        self.environment = Some(environment.into());
        self
    }

    /// `[-t target-pane]` - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub fn target_window<S: Into<Cow<'a, str>>>(mut self, target_window: S) -> Self {
        self.target_window = Some(target_window.into());
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

        cmd.name(RESPAWN_WINDOW);

        // `[-k]` - any existing command is killed
        #[cfg(feature = "tmux_0_8")]
        if self.kill {
            cmd.push_flag(K_LOWERCASE_KEY);
        }

        // `[-c start-directory]` - start-directory
        #[cfg(feature = "tmux_2_6")]
        if let Some(start_directory) = self.start_directory {
            cmd.push_option(C_LOWERCASE_KEY, start_directory);
        }

        // `[-e environment]` - environment
        #[cfg(feature = "tmux_3_0")]
        if let Some(environment) = self.environment {
            cmd.push_option(E_LOWERCASE_KEY, environment);
        }

        // `[-t target-pane]` - target-pane
        #[cfg(feature = "tmux_0_8")]
        if let Some(target_window) = self.target_window {
            cmd.push_option(T_LOWERCASE_KEY, target_window);
        }

        // `[shell-command]` - shell-command
        #[cfg(feature = "tmux_1_2")]
        if let Some(shell_command) = self.shell_command {
            cmd.push_param(shell_command);
        }

        cmd
    }
}
