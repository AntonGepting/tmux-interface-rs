use crate::error::Error;
use crate::tmux_interface::*;
use std::fmt::Display;
use std::process::Output;

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
#[derive(Default, Debug)]
pub struct RespawnWindow<'a, T: Display> {
    /// [-k] - any existing command is killed
    #[cfg(feature = "tmux_0_8")]
    pub kill: Option<bool>,
    /// [-c start-directory] - start-directory
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<&'a str>,
    /// [-e environment] - environment
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    /// [-t target-pane] - target-pane
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a T>,
    /// [shell-command] - shell-command
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a, T: Display + Default> RespawnWindow<'a, T> {
    pub fn new() -> RespawnWindow<'a, T> {
        Default::default()
    }
}

#[derive(Default, Debug)]
pub struct RespawnWindowBuilder<'a, T: Display> {
    #[cfg(feature = "tmux_0_8")]
    pub kill: Option<bool>,
    #[cfg(feature = "tmux_2_6")]
    pub start_directory: Option<&'a str>,
    #[cfg(feature = "tmux_3_0")]
    pub environment: Option<&'a str>,
    #[cfg(feature = "tmux_0_8")]
    pub target_window: Option<&'a T>,
    #[cfg(feature = "tmux_1_2")]
    pub shell_command: Option<&'a str>,
}

impl<'a, T: Display + Default> RespawnWindowBuilder<'a, T> {
    pub fn new() -> Self {
        Default::default()
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn kill(&mut self) -> &mut Self {
        self.kill = Some(true);
        self
    }

    #[cfg(feature = "tmux_2_6")]
    pub fn start_directory(&mut self, start_directory: &'a str) -> &mut Self {
        self.start_directory = Some(start_directory);
        self
    }

    #[cfg(feature = "tmux_3_0")]
    pub fn environment(&mut self, environment: &'a str) -> &mut Self {
        self.environment = Some(environment);
        self
    }

    #[cfg(feature = "tmux_0_8")]
    pub fn target_window(&mut self, target_window: &'a T) -> &mut Self {
        self.target_window = Some(target_window);
        self
    }

    #[cfg(feature = "tmux_1_2")]
    pub fn shell_command(&mut self, shell_command: &'a str) -> &mut Self {
        self.shell_command = Some(shell_command);
        self
    }

    pub fn build(&self) -> RespawnWindow<'a, T> {
        RespawnWindow {
            #[cfg(feature = "tmux_0_8")]
            kill: self.kill,
            #[cfg(feature = "tmux_2_6")]
            start_directory: self.start_directory,
            #[cfg(feature = "tmux_3_0")]
            environment: self.environment,
            #[cfg(feature = "tmux_0_8")]
            target_window: self.target_window,
            #[cfg(feature = "tmux_1_2")]
            shell_command: self.shell_command,
        }
    }
}

impl<'a> TmuxInterface<'a> {
    const RESPAWN_WINDOW: &'static str = "respawn-window";

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
    pub fn respawn_window<T: Display>(
        &mut self,
        respawn_window: Option<&RespawnWindow<T>>,
    ) -> Result<Output, Error> {
        let mut args: Vec<&str> = Vec::new();
        let s;
        if let Some(respawn_window) = respawn_window {
            #[cfg(feature = "tmux_0_8")]
            {
                if respawn_window.kill.unwrap_or(false) {
                    args.push(k_KEY);
                }
            }
            #[cfg(feature = "tmux_2_6")]
            {
                if let Some(s) = respawn_window.start_directory {
                    args.extend_from_slice(&[c_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_3_0")]
            {
                if let Some(s) = respawn_window.environment {
                    args.extend_from_slice(&[e_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_0_8")]
            {
                if let Some(target_window) = respawn_window.target_window {
                    s = target_window.to_string();
                    args.extend_from_slice(&[t_KEY, &s])
                }
            }
            #[cfg(feature = "tmux_1_2")]
            {
                if let Some(s) = respawn_window.shell_command {
                    args.push(s)
                }
            }
        }
        let output = self.subcommand(TmuxInterface::RESPAWN_WINDOW, &args)?;
        Ok(output)
    }
}
